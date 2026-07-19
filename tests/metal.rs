//! The GPU backend, verified on real hardware: emit the derived kernel as
//! Metal, compile the MSL **in-process** (no external toolchain), dispatch it
//! on the GPU through `objc2-metal`, and compare the result to the
//! interpreter oracle.
//!
//! Passing means a kernel sanic *derived* from naive `softmax(QKᵀ)·V` — with no
//! template anywhere — executed on an Apple GPU and matched the reference. That
//! is the credibility line for "a next-gen ML compiler": it runs on the metal.
//!
//! macOS-only, and skips cleanly if the machine has no Metal device (CI
//! without a GPU, etc.). The host is plain Rust: `MTLDevice` compiles the
//! generated MSL at runtime, buffers are shared-memory `MTLBuffer`s, and one
//! command buffer per step encodes every kernel in order (Metal serializes on
//! buffer hazards).

#![cfg(target_os = "macos")]

use std::collections::HashMap;

use sanic::cost::Device;
use sanic::derive::derive;
use sanic::emit_metal::{MetalKernel, MetalProgram, emit_fused_metal, emit_schedule_metal};
use sanic::interp::{Env, Extents, Value, eval};
use sanic::ir::*;
use sanic::metal::{Dispatch, MetalBuf, MetalDevice, program_dispatches};
use sanic::partition::partition;

struct Lcg(u64);
impl Lcg {
    fn f(&mut self) -> f64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        ((x.wrapping_mul(0x2545F4914F6CDD1D) >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0
    }
}
fn rand_tensor(axes: &[Axis], ext: &Extents, rng: &mut Lcg) -> Value {
    Value::from_fn(axes, ext, |_| rng.f())
}
fn as_f64(ext: &Extents) -> HashMap<Axis, f64> {
    ext.iter().map(|(&a, &n)| (a, n as f64)).collect()
}

fn max_rel_err(got: &[f32], expected: &[f64]) -> f64 {
    assert_eq!(got.len(), expected.len(), "output length");
    got.iter()
        .zip(expected)
        .map(|(g, e)| (*g as f64 - e).abs() / (1.0 + e.abs()))
        .fold(0.0, f64::max)
}

/// Run one fused kernel on the GPU and check it against the oracle.
/// `None` = clean skip (no Metal device).
fn run_on_gpu(label: &str, kernel: &MetalKernel, env: &Env, reference: &Value) -> Option<String> {
    let dev = MetalDevice::open()?;
    let pipes = dev.compile(&kernel.msl);
    let pipe = pipes.get(&kernel.name);
    let inputs: Vec<MetalBuf> = kernel
        .inputs
        .iter()
        .map(|(n, axes)| dev.from_f64(&env[n].permuted_to(axes).data))
        .collect();
    let output = dev.alloc_f32(kernel.grid_size);
    dev.run(&[Dispatch {
        pipe,
        inputs,
        output: output.clone(),
        grid: kernel.grid_size,
        argbuf: None,
    }]);
    let got = dev.read_f32(&output, kernel.grid_size);
    let expected = reference.permuted_to(&kernel.grid);
    let maxrel = max_rel_err(&got, &expected.data);
    assert!(maxrel < 2e-3, "GPU MISMATCH {maxrel:e} ({label})");
    Some(format!("GPU OK {maxrel:e}"))
}

/// Run a whole multi-kernel schedule on the GPU, checking the final output
/// against the oracle. `None` = clean skip (no Metal device).
fn run_schedule_on_gpu(
    label: &str,
    program: &MetalProgram,
    env: &Env,
    reference: &Value,
) -> Option<String> {
    let dev = MetalDevice::open()?;
    let pipes = dev.compile(&program.msl);
    let mut bufs: HashMap<String, MetalBuf> = HashMap::new();
    for (n, axes) in &program.inputs {
        bufs.insert(n.to_string(), dev.from_f64(&env[n].permuted_to(axes).data));
    }
    for (n, size) in &program.buffers {
        bufs.insert(n.clone(), dev.alloc_f32(*size));
    }
    dev.run(&program_dispatches(program, &bufs, &pipes));
    let expected = reference.permuted_to(&program.output_axes);
    let got = dev.read_f32(&bufs[&program.output_name], expected.data.len());
    let maxrel = max_rel_err(&got, &expected.data);
    assert!(maxrel < 3e-3, "GPU MISMATCH {maxrel:e} ({label})");
    Some(format!("GPU OK {maxrel:e}"))
}

// ── the derived FlashAttention kernel, executed on the GPU ───────────────────
#[test]
fn flash_attention_runs_on_gpu() {
    let (sq, k, d, e) = (axis("sq"), axis("k"), axis("d"), axis("e"));
    let ext: Extents = [(sq, 16), (k, 24), (d, 8), (e, 8)].into_iter().collect();
    let mut rng = Lcg(0x6D5A);
    let env: Env = [
        ("Q", rand_tensor(&[sq, d], &ext, &mut rng)),
        ("K", rand_tensor(&[k, d], &ext, &mut rng)),
        ("V", rand_tensor(&[k, e], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let attn = attention(
        input("Q", &[sq, d], Dtype::F32),
        input("K", &[k, d], Dtype::F32),
        input("V", &[k, e], Dtype::F32),
        d,
        k,
    );
    let carrier = derive(&attn, k).unwrap();
    let kernel = emit_fused_metal("flash", &carrier, k, &attn, &ext);
    let reference = eval(&attn, &env, &ext);

    let Some(out) = run_on_gpu("flash", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("flash on GPU: {}", out.trim());
}

// ── causal-masked + scaled flash (computed mask), on the GPU ─────────────────
#[test]
fn causal_flash_runs_on_gpu() {
    let (s, t, dk, dv) = (axis("s"), axis("t"), axis("dk"), axis("dv"));
    let ext: Extents = [(s, 16), (t, 16), (dk, 8), (dv, 8)].into_iter().collect();
    let mut rng = Lcg(0xCA05A1);
    let env: Env = [
        ("Q", rand_tensor(&[s, dk], &ext, &mut rng)),
        ("K", rand_tensor(&[t, dk], &ext, &mut rng)),
        ("V", rand_tensor(&[t, dv], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let scores = matmul(
        input("Q", &[s, dk], Dtype::F32),
        input("K", &[t, dk], Dtype::F32),
        dk,
    );
    let scaled = map(MapOp::Mul, vec![scores, konst(0.125)]);
    let masked = map(MapOp::Add, vec![scaled, causal_mask(s, t)]);
    let attn = matmul(softmax(masked, t), input("V", &[t, dv], Dtype::F32), t);

    let carrier = derive(&attn, t).unwrap();
    let kernel = emit_fused_metal("causal_flash", &carrier, t, &attn, &ext);
    let reference = eval(&attn, &env, &ext);

    let Some(out) = run_on_gpu("causal", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("causal flash on GPU: {}", out.trim());
}

// ── flash with a computed cosine relative-position bias, on the GPU ──────────
// Exercises the `Cos` basis op through the whole GPU path: bias(s,t) =
// cos((s−t)·ω) synthesized in-thread and fused into the flash lift.
#[test]
fn cosine_bias_flash_runs_on_gpu() {
    let (s, t, dk, dv) = (axis("s"), axis("t"), axis("dk"), axis("dv"));
    let ext: Extents = [(s, 16), (t, 16), (dk, 8), (dv, 8)].into_iter().collect();
    let mut rng = Lcg(0xB1A5C0);
    let env: Env = [
        ("Q", rand_tensor(&[s, dk], &ext, &mut rng)),
        ("K", rand_tensor(&[t, dk], &ext, &mut rng)),
        ("V", rand_tensor(&[t, dv], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let scores = matmul(
        input("Q", &[s, dk], Dtype::F32),
        input("K", &[t, dk], Dtype::F32),
        dk,
    );
    let rel = map(MapOp::Sub, vec![iota(s), iota(t)]);
    let bias = map(MapOp::Cos, vec![map(MapOp::Mul, vec![rel, konst(0.1)])]);
    let biased = map(MapOp::Add, vec![scores, bias]);
    let attn = matmul(softmax(biased, t), input("V", &[t, dv], Dtype::F32), t);

    let carrier = derive(&attn, t).unwrap();
    let kernel = emit_fused_metal("cos_bias_flash", &carrier, t, &attn, &ext);
    let reference = eval(&attn, &env, &ext);

    let Some(out) = run_on_gpu("cosbias", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("cosine-bias flash on GPU: {}", out.trim());
}

/// A computed RoPE rotation matrix `R[pos, p, j, i]` (extent 2 on i, j): the
/// 2×2 rotation by θ = pos·freq_p, freq_p = exp(p·c) — synthesized from
/// indices, no rotation tensor in memory.
fn rope_rotation(pos: Axis, p: Axis, j: Axis, i: Axis, c: f64) -> NodeRef {
    let freq = map(MapOp::Exp, vec![map(MapOp::Mul, vec![iota(p), konst(c)])]);
    let theta = map(MapOp::Mul, vec![iota(pos), freq]);
    let lt_ij = map(MapOp::Lt, vec![iota(i), iota(j)]);
    let lt_ji = map(MapOp::Lt, vec![iota(j), iota(i)]);
    let eq = map(
        MapOp::Sub,
        vec![
            map(MapOp::Sub, vec![konst(1.0), lt_ij.clone()]),
            lt_ji.clone(),
        ],
    );
    let offdiag = map(MapOp::Sub, vec![lt_ij, lt_ji]);
    map(
        MapOp::Add,
        vec![
            map(MapOp::Mul, vec![eq, map(MapOp::Cos, vec![theta.clone()])]),
            map(MapOp::Mul, vec![offdiag, map(MapOp::Sin, vec![theta])]),
        ],
    )
}

// ── RoPE'd flash attention, as ONE fused kernel, on the GPU ──────────────────
#[test]
fn rope_flash_runs_on_gpu() {
    let (s, t, p, i, j, dk, e) = (
        axis("s"),
        axis("t"),
        axis("p"),
        axis("i"),
        axis("j"),
        axis("dk"),
        axis("e"),
    );
    let ext: Extents = [(s, 12), (t, 12), (p, 4), (i, 2), (j, 2), (dk, 8), (e, 8)]
        .into_iter()
        .collect();
    let c = -0.4;
    let mut rng = Lcg(0x60EA77);
    let env: Env = [
        ("Q", rand_tensor(&[s, p, i], &ext, &mut rng)),
        ("K", rand_tensor(&[t, p, i], &ext, &mut rng)),
        ("V", rand_tensor(&[t, e], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let qr = matmul(
        input("Q", &[s, p, i], Dtype::F32),
        rope_rotation(s, p, j, i, c),
        i,
    );
    let kr = matmul(
        input("K", &[t, p, i], Dtype::F32),
        rope_rotation(t, p, j, i, c),
        i,
    );
    let scores = matmul(flatten(qr, &[p, j], dk), flatten(kr, &[p, j], dk), dk);
    let attn = matmul(softmax(scores, t), input("V", &[t, e], Dtype::F32), t);

    let carrier = derive(&attn, t).unwrap();
    let kernel = emit_fused_metal("rope_flash", &carrier, t, &attn, &ext);
    let reference = eval(&attn, &env, &ext);
    let Some(out) = run_on_gpu("rope", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("RoPE flash on GPU: {}", out.trim());
}

// ── quantized matmul (dequant-fused) on the GPU ──────────────────────────────
#[test]
fn quantized_matmul_runs_on_gpu() {
    let (s, dm, o) = (axis("s"), axis("dm"), axis("o"));
    let ext: Extents = [(s, 8), (dm, 32), (o, 16)].into_iter().collect();
    let mut rng = Lcg(0x9114A7);
    let qw = Value::from_fn(&[o, dm], &ext, |_| (rng.f() * 8.0).round());
    let env: Env = [
        ("X", rand_tensor(&[s, dm], &ext, &mut rng)),
        ("qW", qw),
        (
            "scale",
            Value::from_fn(&[o], &ext, |_| 0.05 * (rng.f() + 1.5)),
        ),
    ]
    .into_iter()
    .collect();

    let dw = map(
        MapOp::Mul,
        vec![
            input("qW", &[o, dm], Dtype::F32),
            input("scale", &[o], Dtype::F32),
        ],
    );
    let y = matmul(input("X", &[s, dm], Dtype::F32), dw, dm);
    let carrier = derive(&y, dm).unwrap();
    let kernel = emit_fused_metal("quant_matmul", &carrier, dm, &y, &ext);
    let reference = eval(&y, &env, &ext);
    let Some(out) = run_on_gpu("quant", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("quantized matmul on GPU: {}", out.trim());
}

// ── logits head + greedy sampling (argmax), on the GPU ───────────────────────
// y → logits = y·Wᵀ → next-token = argmax_v. The output end of a decode step,
// run as a multi-kernel schedule (GEMM, then max, then the argmax index sum),
// entirely on the GPU.
#[test]
fn greedy_sampling_runs_on_gpu() {
    let (s, dm, v) = (axis("s"), axis("dm"), axis("v"));
    let ext: Extents = [(s, 6), (dm, 16), (v, 48)].into_iter().collect();
    let mut rng = Lcg(0x6EED);
    let env: Env = [
        ("Y", rand_tensor(&[s, dm], &ext, &mut rng)),
        ("W_lm", rand_tensor(&[v, dm], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let logits = matmul(
        input("Y", &[s, dm], Dtype::F32),
        input("W_lm", &[v, dm], Dtype::F32),
        dm,
    ); // [s, v]
    let token = argmax(logits, v); // [s]

    let sched = partition(&token, &Device::toy(), &as_f64(&ext));
    let program = emit_schedule_metal(&sched, &ext);
    let reference = eval(&token, &env, &ext);
    let Some(out) = run_schedule_on_gpu("greedy", &program, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!(
        "greedy sampling on GPU ({} kernels): {}",
        program.stages.len(),
        out.trim()
    );
}

// ── CAPSTONE: a complete greedy-decode step, token IDs → next token, on GPU ──
// Embedding gather → RMSNorm-fused QKV → multi-head causal flash → residual →
// RMSNorm → SwiGLU → residual → logits GEMM → argmax. Every kernel family sanic
// generates, assembled into one graph, partitioned, and dispatched end-to-end
// on the Apple GPU — matched to the interpreter.
#[test]
fn greedy_decode_step_runs_on_gpu() {
    let (vv, s, t, dm, h, dk, dvh, dmv, f) = (
        axis("v"),
        axis("s"),
        axis("t"),
        axis("dm"),
        axis("h"),
        axis("dk"),
        axis("dvh"),
        axis("dmv"),
        axis("f"),
    );
    let ext: Extents = [
        (vv, 16),
        (s, 4),
        (t, 4),
        (dm, 8),
        (h, 2),
        (dk, 4),
        (dvh, 4),
        (dmv, 8),
        (f, 10),
    ]
    .into_iter()
    .collect();
    let n = ext[&dm] as f64;
    let mut rng = Lcg(0xDEC0DE);
    let env: Env = [
        (
            "ids",
            Value::from_fn(&[s], &ext, |c| [1.0, 7.0, 3.0, 12.0][c[&s]]),
        ),
        ("E", rand_tensor(&[vv, dm], &ext, &mut rng)),
        ("g1", rand_tensor(&[dm], &ext, &mut rng)),
        ("g2", rand_tensor(&[dm], &ext, &mut rng)),
        ("Wq", rand_tensor(&[h, dk, dm], &ext, &mut rng)),
        ("Wk", rand_tensor(&[h, dk, dm], &ext, &mut rng)),
        ("Wv", rand_tensor(&[h, dvh, dm], &ext, &mut rng)),
        ("Wo", rand_tensor(&[dmv, dm], &ext, &mut rng)),
        ("Wg", rand_tensor(&[f, dm], &ext, &mut rng)),
        ("Wu", rand_tensor(&[f, dm], &ext, &mut rng)),
        ("Wd", rand_tensor(&[f, dm], &ext, &mut rng)),
        ("W_lm", rand_tensor(&[vv, dm], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let rms = |x: NodeRef, g: NodeRef, ax: Axis| {
        let ss = reduce(
            map(MapOp::Mul, vec![x.clone(), x.clone()]),
            ax,
            BinOp::Monoid(Monoid::Add),
        );
        let mean = map(MapOp::Mul, vec![ss, konst(1.0 / n)]);
        let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
        map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom])
    };
    let x = embedding(
        input("E", &[vv, dm], Dtype::F32),
        input("ids", &[s], Dtype::F32),
        vv,
    ); // [s, dm]
    let xn = rms(x.clone(), input("g1", &[dm], Dtype::F32), dm);
    let xn_kv = rename(xn.clone(), s, t);
    let q = matmul(xn, input("Wq", &[h, dk, dm], Dtype::F32), dm);
    let k = matmul(xn_kv.clone(), input("Wk", &[h, dk, dm], Dtype::F32), dm);
    let vvv = matmul(xn_kv, input("Wv", &[h, dvh, dm], Dtype::F32), dm);
    let scores = matmul(q, k, dk);
    let scaled = map(
        MapOp::Mul,
        vec![scores, konst(1.0 / (ext[&dk] as f64).sqrt())],
    );
    let masked = map(MapOp::Add, vec![scaled, causal_mask(s, t)]);
    let attn = matmul(softmax(masked, t), vvv, t);
    let flat = flatten(attn, &[h, dvh], dmv);
    let o = matmul(flat, input("Wo", &[dmv, dm], Dtype::F32), dmv);
    let res1 = map(MapOp::Add, vec![o, x]);
    let hn = rms(res1.clone(), input("g2", &[dm], Dtype::F32), dm);
    let gate = matmul(hn.clone(), input("Wg", &[f, dm], Dtype::F32), dm);
    let up = matmul(hn, input("Wu", &[f, dm], Dtype::F32), dm);
    let act = map(MapOp::Mul, vec![silu(gate), up]);
    let mlp = reduce(
        map(MapOp::Mul, vec![act, input("Wd", &[f, dm], Dtype::F32)]),
        f,
        BinOp::Monoid(Monoid::Add),
    );
    let yb = map(MapOp::Add, vec![mlp, res1]);
    let logits = matmul(yb, input("W_lm", &[vv, dm], Dtype::F32), dm); // [s, v]
    let token = argmax(logits, vv); // [s] — next-token per position

    let sched = partition(&token, &Device::toy(), &as_f64(&ext));
    let program = emit_schedule_metal(&sched, &ext);
    let reference = eval(&token, &env, &ext);
    let Some(out) = run_schedule_on_gpu("decode", &program, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!(
        "greedy DECODE STEP on GPU ({} kernels, IDs→next-token): {}",
        program.stages.len(),
        out.trim()
    );
}

#[test]
fn full_transformer_block_runs_on_gpu() {
    let (v, s, t, dm, h, dk, dv, dmv, f) = (
        axis("v"),
        axis("s"),
        axis("t"),
        axis("dm"),
        axis("h"),
        axis("dk"),
        axis("dv"),
        axis("dmv"),
        axis("f"),
    );
    let ext: Extents = [
        (v, 12),
        (s, 4),
        (t, 4),
        (dm, 8),
        (h, 2),
        (dk, 4),
        (dv, 4),
        (dmv, 8),
        (f, 10),
    ]
    .into_iter()
    .collect();
    let n = ext[&dm] as f64;
    let mut rng = Lcg(0xB10C6);
    let env: Env = [
        ("X", rand_tensor(&[s, dm], &ext, &mut rng)),
        ("g1", rand_tensor(&[dm], &ext, &mut rng)),
        ("g2", rand_tensor(&[dm], &ext, &mut rng)),
        ("Wq", rand_tensor(&[h, dk, dm], &ext, &mut rng)),
        ("Wk", rand_tensor(&[h, dk, dm], &ext, &mut rng)),
        ("Wv", rand_tensor(&[h, dv, dm], &ext, &mut rng)),
        ("Wo", rand_tensor(&[dmv, dm], &ext, &mut rng)),
        ("Wg", rand_tensor(&[f, dm], &ext, &mut rng)),
        ("Wu", rand_tensor(&[f, dm], &ext, &mut rng)),
        ("Wd", rand_tensor(&[f, dm], &ext, &mut rng)),
        ("W_lm", rand_tensor(&[v, dm], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let rms = |x: NodeRef, g: NodeRef, ax: Axis| {
        let ss = reduce(
            map(MapOp::Mul, vec![x.clone(), x.clone()]),
            ax,
            BinOp::Monoid(Monoid::Add),
        );
        let mean = map(MapOp::Mul, vec![ss, konst(1.0 / n)]);
        let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
        map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom])
    };
    let x = input("X", &[s, dm], Dtype::F32);
    let xn = rms(x.clone(), input("g1", &[dm], Dtype::F32), dm);
    let xn_kv = rename(xn.clone(), s, t);
    let q = matmul(xn, input("Wq", &[h, dk, dm], Dtype::F32), dm);
    let k = matmul(xn_kv.clone(), input("Wk", &[h, dk, dm], Dtype::F32), dm);
    let vv = matmul(xn_kv, input("Wv", &[h, dv, dm], Dtype::F32), dm);
    let scores = matmul(q, k, dk);
    let scaled = map(
        MapOp::Mul,
        vec![scores, konst(1.0 / (ext[&dk] as f64).sqrt())],
    );
    let masked = map(MapOp::Add, vec![scaled, causal_mask(s, t)]);
    let attn = matmul(softmax(masked, t), vv, t);
    let flat = flatten(attn, &[h, dv], dmv);
    let o = matmul(flat, input("Wo", &[dmv, dm], Dtype::F32), dmv);
    let res1 = map(MapOp::Add, vec![o, x]);
    let hn = rms(res1.clone(), input("g2", &[dm], Dtype::F32), dm);
    let gate = matmul(hn.clone(), input("Wg", &[f, dm], Dtype::F32), dm);
    let up = matmul(hn, input("Wu", &[f, dm], Dtype::F32), dm);
    let act = map(MapOp::Mul, vec![silu(gate), up]);
    let mlp = reduce(
        map(MapOp::Mul, vec![act, input("Wd", &[f, dm], Dtype::F32)]),
        f,
        BinOp::Monoid(Monoid::Add),
    );
    let yb = map(MapOp::Add, vec![mlp, res1]);
    let logits = matmul(yb, input("W_lm", &[v, dm], Dtype::F32), dm);

    let sched = partition(&logits, &Device::toy(), &as_f64(&ext));
    let program = emit_schedule_metal(&sched, &ext);
    let reference = eval(&logits, &env, &ext);
    let Some(out) = run_schedule_on_gpu("block", &program, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!(
        "full transformer block on GPU ({} kernels): {}",
        program.stages.len(),
        out.trim()
    );
}

// ── conv2d (window + flatten + matmul), on the GPU ───────────────────────────
// The movement vocabulary on real hardware: windowed reads lower to signed
// index arithmetic in MSL. One implicit-GEMM kernel, no im2col buffer.
#[test]
fn conv2d_runs_on_gpu() {
    let (ci, h0, w0, oh, ow, kh, kw, r, co) = (
        axis("ci"),
        axis("h0"),
        axis("w0"),
        axis("oh"),
        axis("ow"),
        axis("kh"),
        axis("kw"),
        axis("r"),
        axis("co"),
    );
    let ext: Extents = [
        (ci, 3),
        (h0, 10),
        (w0, 12),
        (oh, 8),
        (ow, 10),
        (kh, 3),
        (kw, 3),
        (r, 27),
        (co, 8),
    ]
    .into_iter()
    .collect();
    let mut rng = Lcg(0x6042D);
    let env: Env = [
        ("X", rand_tensor(&[ci, h0, w0], &ext, &mut rng)),
        ("W", rand_tensor(&[co, ci, kh, kw], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let xw = reindex(
        input("X", &[ci, h0, w0], Dtype::F32),
        vec![
            (h0, vec![(1, oh), (1, kh)], 0),
            (w0, vec![(1, ow), (1, kw)], 0),
        ],
        false,
    );
    let xf = flatten(xw, &[ci, kh, kw], r);
    let wf = flatten(input("W", &[co, ci, kh, kw], Dtype::F32), &[ci, kh, kw], r);
    let conv = matmul(xf, wf, r);

    let carrier = derive(&conv, r).unwrap();
    let kernel = emit_fused_metal("conv2d", &carrier, r, &conv, &ext);
    let reference = eval(&conv, &env, &ext);

    let Some(out) = run_on_gpu("conv2d", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("conv2d on GPU: {}", out.trim());
}

// ── sliding-window flash attention (padded windowed K/V), on the GPU ─────────
#[test]
fn sliding_window_flash_runs_on_gpu() {
    let (s, t, j, d, e) = (axis("s"), axis("t"), axis("j"), axis("d"), axis("e"));
    let (ns, w) = (24usize, 6usize);
    let ext: Extents = [(s, ns), (t, ns), (j, w), (d, 8), (e, 8)]
        .into_iter()
        .collect();
    let mut rng = Lcg(0x51DE60);
    let env: Env = [
        ("Q", rand_tensor(&[s, d], &ext, &mut rng)),
        ("K", rand_tensor(&[t, d], &ext, &mut rng)),
        ("V", rand_tensor(&[t, e], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let off = -((w - 1) as i64);
    let kw = reindex(
        input("K", &[t, d], Dtype::F32),
        vec![(t, vec![(1, s), (1, j)], off)],
        true,
    );
    let vw = reindex(
        input("V", &[t, e], Dtype::F32),
        vec![(t, vec![(1, s), (1, j)], off)],
        true,
    );
    let scores = matmul(input("Q", &[s, d], Dtype::F32), kw, d);
    let invalid = map(
        MapOp::Lt,
        vec![
            map(MapOp::Add, vec![iota(s), iota(j)]),
            konst((w - 1) as f64),
        ],
    );
    let masked = map(
        MapOp::Add,
        vec![
            scores,
            map(MapOp::Where, vec![invalid, konst(-1e30), konst(0.0)]),
        ],
    );
    let attn = matmul(softmax(masked, j), vw, j);

    let carrier = derive(&attn, j).unwrap();
    let kernel = emit_fused_metal("swa_flash", &carrier, j, &attn, &ext);
    let reference = eval(&attn, &env, &ext);

    let Some(out) = run_on_gpu("swa", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("sliding-window flash on GPU: {}", out.trim());
}

// ── M6 on the GPU: the decode loop with persistent device caches ─────────────
// The runtime discipline on real hardware: cache_k / cache_v are MTLBuffers
// allocated ONCE; each step writes x/pos, dispatches every kernel of the
// three-output decode schedule, then COMMITS by swapping the cache buffer
// with the update buffer in the name→buffer map (commit-after-execute, no
// copy). T incremental steps on the GPU must equal the interpreter's full
// prefill.
#[test]
fn decode_loop_runs_on_gpu() {
    let (t, s, t2, dm, dk, dv, v) = (
        axis("t"),
        axis("s"),
        axis("t2"),
        axis("dm"),
        axis("dk"),
        axis("dv"),
        axis("v"),
    );
    let steps = 6usize;
    let ext: Extents = [
        (t, steps),
        (s, steps),
        (t2, steps),
        (dm, 8),
        (dk, 6),
        (dv, 6),
        (v, 12),
    ]
    .into_iter()
    .collect();
    let mut rng = Lcg(0xDEC0DE60);
    let wq = rand_tensor(&[dk, dm], &ext, &mut rng);
    let wk = rand_tensor(&[dk, dm], &ext, &mut rng);
    let wv = rand_tensor(&[dv, dm], &ext, &mut rng);
    let wl = rand_tensor(&[v, dv], &ext, &mut rng);
    let xs = rand_tensor(&[s, dm], &ext, &mut rng);
    let env: Env = [
        ("Wq", wq.clone()),
        ("Wk", wk.clone()),
        ("Wv", wv.clone()),
        ("Wl", wl.clone()),
        ("X", xs.clone()),
    ]
    .into_iter()
    .collect();

    // the decode-step schedule: cache updates + logits, three outputs
    let x = input("x", &[dm], Dtype::F32);
    let pos = input("pos", &[], Dtype::F32);
    let new_k = matmul(x.clone(), input("Wk", &[dk, dm], Dtype::F32), dm);
    let new_v = matmul(x.clone(), input("Wv", &[dv, dm], Dtype::F32), dm);
    let q = matmul(x, input("Wq", &[dk, dm], Dtype::F32), dm);
    let ck = map(
        MapOp::Where,
        vec![
            one_hot(t, pos.clone()),
            new_k,
            input("cache_k", &[t, dk], Dtype::F32),
        ],
    );
    let cv = map(
        MapOp::Where,
        vec![
            one_hot(t, pos.clone()),
            new_v,
            input("cache_v", &[t, dv], Dtype::F32),
        ],
    );
    let scale = konst(1.0 / (ext[&dk] as f64).sqrt());
    let scores = map(MapOp::Mul, vec![matmul(q, ck.clone(), dk), scale]);
    let future = map(MapOp::Lt, vec![pos, iota(t)]);
    let masked = map(
        MapOp::Add,
        vec![
            scores,
            map(MapOp::Where, vec![future, konst(-1e30), konst(0.0)]),
        ],
    );
    let att = softmax(masked, t);
    let out = matmul(att, cv.clone(), t);
    let logits = matmul(out, input("Wl", &[v, dv], Dtype::F32), dv);
    let sched = sanic::partition::partition_many(
        &[(ck, "ck_new"), (cv, "cv_new"), (logits, "logits")],
        &Device::toy(),
        &as_f64(&ext),
    );
    let program = emit_schedule_metal(&sched, &ext);

    // the reference: full causal prefill by the oracle
    let xq = input("X", &[s, dm], Dtype::F32);
    let xt = rename(xq.clone(), s, t2);
    let qa = matmul(xq, input("Wq", &[dk, dm], Dtype::F32), dm);
    let ka = matmul(xt.clone(), input("Wk", &[dk, dm], Dtype::F32), dm);
    let va = matmul(xt, input("Wv", &[dv, dm], Dtype::F32), dm);
    let sc = map(
        MapOp::Mul,
        vec![matmul(qa, ka, dk), konst(1.0 / (ext[&dk] as f64).sqrt())],
    );
    let ma = map(MapOp::Add, vec![sc, causal_mask(s, t2)]);
    let oa = matmul(softmax(ma, t2), va, t2);
    let logits_ref = eval(
        &matmul(oa, input("Wl", &[v, dv], Dtype::F32), dv),
        &env,
        &ext,
    );
    let expected: Vec<f64> = (0..steps)
        .flat_map(|p| {
            (0..ext[&v])
                .map(|vi| logits_ref.at(&HashMap::from([(s, p), (v, vi)])))
                .collect::<Vec<f64>>()
        })
        .collect();

    // the native host: persistent caches, a per-step loop, commit-as-swap
    let Some(dev) = MetalDevice::open() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let pipes = dev.compile(&program.msl);
    let mut bufs: HashMap<String, MetalBuf> = HashMap::new();
    for (name, tensor) in [("Wq", &wq), ("Wk", &wk), ("Wv", &wv), ("Wl", &wl)] {
        let axes = program
            .inputs
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, a)| a.clone())
            .unwrap_or_else(|| tensor.axes.clone());
        bufs.insert(
            name.to_string(),
            dev.from_f64(&tensor.permuted_to(&axes).data),
        );
    }
    bufs.insert("cache_k".into(), dev.alloc_f32(steps * ext[&dk]));
    bufs.insert("cache_v".into(), dev.alloc_f32(steps * ext[&dv]));
    bufs.insert("x".into(), dev.alloc_f32(ext[&dm]));
    bufs.insert("pos".into(), dev.alloc_f32(1));
    for (n, size) in &program.buffers {
        bufs.insert(n.clone(), dev.alloc_f32(*size));
    }

    let mut got: Vec<f32> = Vec::new();
    for p in 0..steps {
        let row: Vec<f64> = (0..ext[&dm])
            .map(|di| xs.at(&HashMap::from([(s, p), (dm, di)])))
            .collect();
        dev.write_f64(&bufs["x"], &row);
        dev.write_f64(&bufs["pos"], &[p as f64]);
        // dispatches re-resolve names each step, so the swapped caches bind
        dev.run(&program_dispatches(&program, &bufs, &pipes));
        // commit-after-execute: swap the cache buffers with the updates
        let ckb = bufs["ck_new"].clone();
        let old_k = bufs.insert("cache_k".into(), ckb).unwrap();
        bufs.insert("ck_new".into(), old_k);
        let cvb = bufs["cv_new"].clone();
        let old_v = bufs.insert("cache_v".into(), cvb).unwrap();
        bufs.insert("cv_new".into(), old_v);
        got.extend(dev.read_f32(&bufs["logits"], ext[&v]));
    }
    let maxrel = max_rel_err(&got, &expected);
    assert!(maxrel < 3e-3, "GPU decode loop MISMATCH {maxrel:e}");
    eprintln!(
        "GPU decode loop ({} kernels/step × {steps} steps, persistent caches): GPU OK {maxrel:e}",
        program.stages.len(),
    );
}

// ── M8 on the GPU: a BACKWARD pass, dispatched on the metal ──────────────────
// dLoss/dQ of causally-masked attention: the gradient graph partitions into
// kernels like any forward graph, and the whole multi-kernel backward
// schedule runs on the GPU and matches the oracle.
#[test]
fn attention_backward_runs_on_gpu() {
    let (s, t, dk, dv) = (axis("s"), axis("t"), axis("dk"), axis("dv"));
    let ext: Extents = [(s, 8), (t, 8), (dk, 6), (dv, 6)].into_iter().collect();
    let mut rng = Lcg(0x6ADB);
    let env: Env = [
        ("Q", rand_tensor(&[s, dk], &ext, &mut rng)),
        ("K", rand_tensor(&[t, dk], &ext, &mut rng)),
        ("V", rand_tensor(&[t, dv], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let scores = matmul(
        input("Q", &[s, dk], Dtype::F32),
        input("K", &[t, dk], Dtype::F32),
        dk,
    );
    let masked = map(MapOp::Add, vec![scores, causal_mask(s, t)]);
    let out = matmul(softmax(masked, t), input("V", &[t, dv], Dtype::F32), t);
    let sq = map(MapOp::Mul, vec![out.clone(), out]);
    let loss = reduce(
        reduce(sq, s, BinOp::Monoid(Monoid::Add)),
        dv,
        BinOp::Monoid(Monoid::Add),
    );

    let grads = sanic::grad::grad(&loss, &["Q"], &ext);
    let g = &grads["Q"];
    let sched = partition(g, &Device::toy(), &as_f64(&ext));
    let program = emit_schedule_metal(&sched, &ext);
    let reference = eval(g, &env, &ext);

    let Some(msg) = run_schedule_on_gpu("dq", &program, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!(
        "attention dLoss/dQ on GPU ({} kernels): {}",
        program.stages.len(),
        msg.trim()
    );
}

// ── M9 on the GPU: a split reduction (GROUP), two kernels ────────────────────
// A tall softmax-weighted reduction (grid of 4×8 over a 4096-long axis) run
// as partial + combine kernels: stage 1 folds 64 chunks per output point
// into raw (m, ℓ, o) states; stage 2 merges them with the carrier's own
// rescaling combine and projects. Matches the one-pass oracle — the monoid
// law, executed on hardware.
#[test]
fn split_reduction_runs_on_gpu() {
    let (s, k, d, e) = (axis("s"), axis("k"), axis("d"), axis("e"));
    let ext: Extents = [(s, 4), (k, 4096), (d, 8), (e, 8)].into_iter().collect();
    let mut rng = Lcg(0x5B117);
    let env: Env = [
        ("Q", rand_tensor(&[s, d], &ext, &mut rng)),
        ("K", rand_tensor(&[k, d], &ext, &mut rng)),
        ("V", rand_tensor(&[k, e], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let attn = attention(
        input("Q", &[s, d], Dtype::F32),
        input("K", &[k, d], Dtype::F32),
        input("V", &[k, e], Dtype::F32),
        d,
        k,
    );
    let carrier = derive(&attn, k).unwrap();
    let blocks = 64usize;
    let (partial, combine) =
        sanic::emit_metal::emit_split_metal("swk", &carrier, k, &attn, &ext, blocks);
    let reference = eval(&attn, &env, &ext);

    // one library holding both kernels (strip the second prelude wholesale)
    let msl = format!(
        "{}\n{}",
        partial.msl,
        &combine.msl[combine.msl.find("kernel void").unwrap()..]
    );
    let Some(dev) = MetalDevice::open() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let pipes = dev.compile(&msl);
    let inputs: Vec<MetalBuf> = partial
        .inputs
        .iter()
        .map(|(n, axes)| dev.from_f64(&env[n].permuted_to(axes).data))
        .collect();
    let partials_buf = dev.alloc_f32(partial.grid_size * carrier.slots);
    let out_buf = dev.alloc_f32(combine.grid_size);
    dev.run(&[
        Dispatch {
            pipe: pipes.get(&partial.name),
            inputs,
            output: partials_buf.clone(),
            grid: partial.grid_size,
            argbuf: None,
        },
        Dispatch {
            pipe: pipes.get(&combine.name),
            inputs: vec![partials_buf],
            output: out_buf.clone(),
            grid: combine.grid_size,
            argbuf: None,
        },
    ]);
    let got = dev.read_f32(&out_buf, combine.grid_size);
    let expected = reference.permuted_to(&combine.grid);
    let maxrel = max_rel_err(&got, &expected.data);
    assert!(maxrel < 3e-3, "GPU split reduction MISMATCH {maxrel:e}");
    eprintln!("split reduction on GPU ({blocks} partials/point over k=4096): GPU OK {maxrel:e}");
}

// ── regression: fast-math tanh NaN'd on large inputs ─────────────────────────
// GPT-2's MLP activations exceed |x| ≈ 44, where Metal's fast tanh (via
// exp(2x)) returns inf/inf = NaN. The emitter must use the precise variant.
#[test]
fn tanh_survives_large_arguments_on_gpu() {
    let n = axis("n");
    let ext: Extents = [(n, 6)].into_iter().collect();
    let x = Value::from_fn(&[n], &ext, |c| {
        [-2000.0, -50.0, -1.0, 1.0, 50.0, 2000.0][c[&n]]
    });
    let env: Env = [("X", x.clone())].into_iter().collect();

    let t = map(MapOp::Tanh, vec![input("X", &[n], Dtype::F32)]);
    let kernel = sanic::emit_metal::emit_pointwise_metal("tanh_big", &t, &ext);
    let reference = eval(&t, &env, &ext);
    let Some(out) = run_on_gpu("tanh", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("large-argument tanh on GPU: {}", out.trim());
}

// ── typed storage: bf16 weights loaded straight from their checkpoint bytes ──
// A bf16 buffer widens `bits << 16` in the kernel — exact over the whole f32
// range, and byte-identical to the checkpoint (so it is what a zero-copy
// weight binds). The oracle sees the widened f64 values; equality proves the
// widen is bit-exact.
#[test]
fn bf16_matvec_runs_on_gpu() {
    let (o, k) = (axis("o"), axis("k"));
    let ext: Extents = [(o, 40), (k, 96)].into_iter().collect();
    let mut rng = Lcg(0xBF16);

    // random f32 weights, TRUNCATED to bf16 (drop the low 16 mantissa bits) —
    // exactly what a bf16 checkpoint stores; the oracle uses the same values.
    let bf16_of = |x: f32| -> (u16, f64) {
        let hi = (x.to_bits() >> 16) as u16; // round-toward-zero truncation
        let widened = f32::from_bits((hi as u32) << 16);
        (hi, widened as f64)
    };
    let mut wbytes = Vec::with_capacity(40 * 96 * 2);
    let wvals: Vec<f64> = (0..40 * 96)
        .map(|_| {
            let (bits, v) = bf16_of(rng.f() as f32 * 4.0);
            wbytes.extend_from_slice(&bits.to_le_bytes());
            v
        })
        .collect();

    let env: Env = [
        (
            "W",
            Value::from_fn(&[o, k], &ext, |c| wvals[c[&o] * 96 + c[&k]]),
        ),
        ("x", rand_tensor(&[k], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let y = reduce(
        map(
            MapOp::Mul,
            vec![
                input("W", &[o, k], Dtype::BF16),
                input("x", &[k], Dtype::F32),
            ],
        ),
        k,
        BinOp::Monoid(Monoid::Add),
    );
    let carrier = derive(&y, k).unwrap();
    let kernel = emit_fused_metal("bf16mv", &carrier, k, &y, &ext);
    assert!(
        kernel.msl.contains("device const ushort* b_W"),
        "bf16 buffer typed"
    );
    assert!(kernel.msl.contains("<< 16u"), "bf16 widen emitted");
    let reference = eval(&y, &env, &ext);

    let Some(dev) = MetalDevice::open() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let pipes = dev.compile(&kernel.msl);
    let pipe = pipes.get(&kernel.name);
    let inputs: Vec<MetalBuf> = kernel
        .inputs
        .iter()
        .map(|(n, axes)| match *n {
            "W" => dev.from_bytes(&wbytes),
            _ => dev.from_f64(&env[n].permuted_to(axes).data),
        })
        .collect();
    let out = dev.alloc_f32(kernel.grid_size);
    dev.run(&[Dispatch {
        pipe,
        inputs,
        output: out.clone(),
        grid: kernel.grid_size,
        argbuf: None,
    }]);
    let got = dev.read_f32(&out, kernel.grid_size);
    let expected = reference.permuted_to(&kernel.grid);
    let maxrel = max_rel_err(&got, &expected.data);
    assert!(maxrel < 1e-6, "bf16 matvec MISMATCH {maxrel:e}");
    eprintln!("bf16 matvec on GPU (widened from checkpoint bytes): GPU OK {maxrel:e}");
}

// ── typed storage: a W4A16 grouped-quantized matvec, packed bytes on device ──
// The byte-storage milestone made real: the weight buffer holds PACKED int4
// nibbles (compressed-tensors layout: unsigned q+8, low nibble = even
// element), the per-group scales are an f16 buffer, and the kernel unpacks
// and dequantizes inside the GEMM fold. The oracle sees the same integers as
// f64 — equality proves the byte path bit-exact.
#[test]
fn w4_grouped_matvec_runs_on_gpu() {
    let (o, gq, r, c) = (axis("o"), axis("g"), axis("r"), axis("c"));
    let (n_out, n_g, n_r) = (16usize, 4usize, 128usize);
    let n_in = n_g * n_r;
    let ext: Extents = [(o, n_out), (gq, n_g), (r, n_r), (c, n_in)]
        .into_iter()
        .collect();
    let mut rng = Lcg(0x144A16);

    // integer nibbles q ∈ [-8, 7], f16-exact scales, random activations
    let q: Vec<i8> = (0..n_out * n_in)
        .map(|_| ((rng.f() * 8.0).floor().clamp(-8.0, 7.0)) as i8)
        .collect();
    let mut packed = vec![0u8; n_out * n_in / 2];
    for (i, &v) in q.iter().enumerate() {
        let nib = (v + 8) as u8;
        packed[i / 2] |= nib << ((i & 1) * 4);
    }
    let scales: Vec<f64> = (0..n_out * n_g)
        .map(|_| (1.0 + (rng.f().abs() * 15.0).round()) / 64.0)
        .collect();
    let mut scale_f16 = Vec::with_capacity(scales.len() * 2);
    for &s in &scales {
        // f64 → f16 bits (values chosen exactly representable)
        let f = s as f32;
        let bits = f.to_bits();
        let h = (((bits >> 16) & 0x8000)
            | ((((bits >> 23) & 0xFF) as i32 - 127 + 15) as u32) << 10
            | ((bits >> 13) & 0x3FF)) as u16;
        scale_f16.extend_from_slice(&h.to_le_bytes());
    }

    let env: Env = [
        (
            "Wq",
            Value::from_fn(&[o, gq, r], &ext, |cd| {
                q[cd[&o] * n_in + cd[&gq] * n_r + cd[&r]] as f64
            }),
        ),
        (
            "S",
            Value::from_fn(&[o, gq], &ext, |cd| scales[cd[&o] * n_g + cd[&gq]]),
        ),
        ("x", rand_tensor(&[gq, r], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    // y[o] = Σ_c (q · x · scale), the contraction flattened to one axis
    let prod = map(
        MapOp::Mul,
        vec![
            map(
                MapOp::Mul,
                vec![
                    input("Wq", &[o, gq, r], Dtype::I4),
                    input("x", &[gq, r], Dtype::F32),
                ],
            ),
            input("S", &[o, gq], Dtype::F16),
        ],
    );
    let y = reduce(flatten(prod, &[gq, r], c), c, BinOp::Monoid(Monoid::Add));

    let carrier = derive(&y, c).unwrap();
    let kernel = emit_fused_metal("w4mv", &carrier, c, &y, &ext);
    assert!(kernel.msl.contains("device const uchar* b_Wq"));
    assert!(kernel.msl.contains("device const half* b_S"));
    let reference = eval(&y, &env, &ext);

    let Some(dev) = MetalDevice::open() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let pipes = dev.compile(&kernel.msl);
    let pipe = pipes.get(&kernel.name);
    let inputs: Vec<MetalBuf> = kernel
        .inputs
        .iter()
        .map(|(n, axes)| match *n {
            "Wq" => dev.from_bytes(&packed),
            "S" => dev.from_bytes(&scale_f16),
            _ => dev.from_f64(&env[n].permuted_to(axes).data),
        })
        .collect();
    let out = dev.alloc_f32(kernel.grid_size);
    dev.run(&[Dispatch {
        pipe,
        inputs,
        output: out.clone(),
        grid: kernel.grid_size,
        argbuf: None,
    }]);
    let got = dev.read_f32(&out, kernel.grid_size);
    let expected = reference.permuted_to(&kernel.grid);
    let maxrel = max_rel_err(&got, &expected.data);
    assert!(maxrel < 2e-3, "W4 matvec MISMATCH {maxrel:e}");
    eprintln!("W4A16 grouped matvec on GPU (packed int4 + f16 scales): GPU OK {maxrel:e}");
}

/// The k-best streaming carrier on real hardware: every rank's value AND index
/// of a top-8 selection — including planted exact ties, where first-max-wins
/// is the contract — derived as single folds and dispatched on the GPU. This
/// exercises singleton insertion, not the still-missing two-list merge.
#[test]
fn topk_kbest_folds_run_on_gpu() {
    let n = axis("n");
    let ext: Extents = [(n, 129)].into_iter().collect();
    let mut rng = Lcg(0xC0BE5);
    let mut vals: Vec<f64> = (0..129).map(|_| rng.f()).collect();
    // exact ties: a duplicated maximum and an interior duplicate
    vals[97] = vals[13]; // tie at some mid rank
    let m = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    vals[110] = m; // duplicate of the max, later position
    let env: Env = [("X", Value::from_fn(&[n], &ext, |c| vals[c[&n]]))]
        .into_iter()
        .collect();

    for (r, (v, i)) in topk(input("X", &[n], Dtype::F32), n, 8)
        .into_iter()
        .enumerate()
    {
        for (tag, node) in [("val", v), ("idx", i)] {
            let carrier = derive(&node, n).expect("k-best derives");
            let name = format!("top8_{tag}_{r}");
            let kernel = emit_fused_metal(&name, &carrier, n, &node, &ext);
            let reference = eval(&node, &env, &ext);
            let Some(out) = run_on_gpu(&name, &kernel, &env, &reference) else {
                eprintln!("skipping: no Metal device");
                return;
            };
            eprintln!("{name} on GPU: {}", out.trim());
        }
    }
}

/// ALL ranks of the selection as ONE kernel: the rank axis is the grid, the
/// k-best slots are shared across rank queries, and the projection reads the
/// rank one-hot at PROJECT scope — leaf loads outside the stream loop, on
/// real hardware, against the oracle, with the same planted exact ties.
#[test]
fn topk_all_single_fold_runs_on_gpu() {
    let (n, rk) = (axis("n"), axis("rk"));
    let ext: Extents = [(n, 129), (rk, 8)].into_iter().collect();
    let mut rng = Lcg(0xC0BE5);
    let mut vals: Vec<f64> = (0..129).map(|_| rng.f()).collect();
    vals[97] = vals[13];
    let m = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    vals[110] = m;
    let env: Env = [("X", Value::from_fn(&[n], &ext, |c| vals[c[&n]]))]
        .into_iter()
        .collect();

    let all = topk_all(input("X", &[n], Dtype::F32), n, 8, rk, true);
    let carrier = derive(&all, n).expect("topk_all derives");
    assert_eq!(carrier.slots, 16, "one shared k-best list");
    let kernel = emit_fused_metal("top8_all", &carrier, n, &all, &ext);
    let reference = eval(&all, &env, &ext);
    let Some(out) = run_on_gpu("top8_all", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!(
        "top8_all (one fold, rank-indexed projection) on GPU: {}",
        out.trim()
    );
}

/// Graph execution: the same schedule captured into an indirect command
/// buffer and REPLAYED — twice, to prove the capture is stable — must match
/// the oracle. The schedule is a dependent chain (norm folds feeding GEMMs
/// feeding a flash fold), and the grid sizes are non-round, so this
/// exercises the hazard barriers and the nonuniform indirect dispatch.
#[test]
fn graph_replay_matches_oracle() {
    use sanic::metal::MetalGraph;
    let (s, dm, v) = (axis("s"), axis("dm"), axis("v"));
    let ext: Extents = [(s, 7), (dm, 19), (v, 53)].into_iter().collect();
    let mut rng = Lcg(0x6EA9);
    let env: Env = [
        ("X", rand_tensor(&[s, dm], &ext, &mut rng)),
        ("G", rand_tensor(&[dm], &ext, &mut rng)),
        ("W", rand_tensor(&[v, dm], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    // rmsnorm(X)·Wᵀ then a softmax-weighted reduction back over v — a chain
    // with real read-after-write hazards at every stage
    let x = input("X", &[s, dm], Dtype::F32);
    let ms = reduce(
        map(MapOp::Mul, vec![x.clone(), x.clone()]),
        dm,
        BinOp::Monoid(Monoid::Add),
    );
    let inv = map(
        MapOp::Recip,
        vec![map(
            MapOp::Sqrt,
            vec![map(MapOp::Add, vec![ms, konst(1e-5)])],
        )],
    );
    let xn = map(
        MapOp::Mul,
        vec![map(MapOp::Mul, vec![x, input("G", &[dm], Dtype::F32)]), inv],
    );
    let logits = matmul(xn, input("W", &[v, dm], Dtype::F32), dm); // [s, v]
    let out = reduce(logits, v, BinOp::Monoid(Monoid::LogSumExp)); // [s]

    let sched = partition(&out, &Device::toy(), &as_f64(&ext));
    let program = emit_schedule_metal(&sched, &ext);
    let reference = eval(&out, &env, &ext).permuted_to(&program.output_axes);

    let Some(dev) = MetalDevice::open() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let pipes = dev.compile(&program.msl);
    let mut bufs: HashMap<String, MetalBuf> = HashMap::new();
    for (n, axes) in &program.inputs {
        bufs.insert(n.to_string(), dev.from_f64(&env[n].permuted_to(axes).data));
    }
    for (n, size) in &program.buffers {
        bufs.insert(n.clone(), dev.alloc_f32(*size));
    }
    let graph: MetalGraph = dev.capture(&program_dispatches(&program, &bufs, &pipes));
    for replay in 0..2 {
        dev.run_graph(&graph);
        let got = dev.read_f32(&bufs[&program.output_name], reference.data.len());
        let maxrel = max_rel_err(&got, &reference.data);
        assert!(
            maxrel < 3e-3,
            "GRAPH MISMATCH on replay {replay}: {maxrel:e}"
        );
    }
    eprintln!(
        "graph replay on GPU: {} kernels in one indirect command buffer, two replays match",
        program.stages.len()
    );
}

// ── cooperative fold schedules, each emitter path against the oracle ────────
//
// A FoldSched is the GROUP re-association law rendered intra-kernel: lanes
// or simdgroups fold disjoint slices of the streamed axis and the partials
// merge with the carrier's OWN combine — over simd shuffles at lane level,
// through threadgroup memory across simdgroups. These tests pin every path:
// the coupled online-softmax carrier through both merge kinds, the
// lane-distributed output axis (sliced slots + the in-body contraction
// lane-split), and the order-sensitive decline (first-max-wins argmax must
// NOT split out of stream order).

fn run_coop_on_gpu(
    label: &str,
    kernel: &MetalKernel,
    env: &Env,
    reference: &Value,
    ext: &Extents,
) -> Option<String> {
    let dev = MetalDevice::open()?;
    let pipes = dev.compile(&kernel.msl);
    let pipe = pipes.get(&kernel.name);
    let inputs: Vec<MetalBuf> = kernel
        .inputs
        .iter()
        .map(|(n, axes)| dev.from_f64(&env[n].permuted_to(axes).data))
        .collect();
    // a cooperative kernel's grid_size is threads (TGs × TG width), not
    // output elements — read back the output volume
    let out_n: usize = kernel.grid.iter().map(|a| ext[a]).product::<usize>().max(1);
    let output = dev.alloc_f32(kernel.grid_size.max(out_n));
    dev.run(&[Dispatch {
        pipe,
        inputs,
        output: output.clone(),
        grid: kernel.grid_size,
        argbuf: None,
    }]);
    let got = dev.read_f32(&output, out_n);
    let expected = reference.permuted_to(&kernel.grid);
    let maxrel = max_rel_err(&got, &expected.data);
    assert!(maxrel < 2e-3, "GPU MISMATCH {maxrel:e} ({label})");
    Some(format!("GPU OK {maxrel:e}"))
}

/// The coupled (m, ℓ, o) carrier with the stream split over lanes AND
/// simdgroups: the ExpShifted rescale merge runs through the shuffle
/// butterfly and then threadgroup rounds.
#[test]
fn coop_lane_stream_flash_matches_oracle() {
    use sanic::emit_metal::emit_fused_metal_sched;
    use sanic::plan::FoldSched;
    let (sq, k, d, e) = (axis("sq"), axis("k"), axis("d"), axis("e"));
    let ext: Extents = [(sq, 4), (k, 256), (d, 8), (e, 8)].into_iter().collect();
    let mut rng = Lcg(0xC007);
    let env: Env = [
        ("Q", rand_tensor(&[sq, d], &ext, &mut rng)),
        ("K", rand_tensor(&[k, d], &ext, &mut rng)),
        ("V", rand_tensor(&[k, e], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();
    let attn = attention(
        input("Q", &[sq, d], Dtype::F32),
        input("K", &[k, d], Dtype::F32),
        input("V", &[k, e], Dtype::F32),
        d,
        k,
    );
    let carrier = derive(&attn, k).unwrap();
    let sched = FoldSched {
        lane_axis: None,
        sgs: 4,
        lane_stream: true, // split factor 128 ≤ 256
        chunk: 1,
    };
    let kernel = emit_fused_metal_sched("coop_ls_flash", &carrier, k, &attn, &ext, sched);
    assert!(
        kernel.msl.contains("simd_shuffle_xor"),
        "lane merge emitted"
    );
    assert!(
        kernel.msl.contains("threadgroup_barrier"),
        "sg merge emitted"
    );
    let reference = eval(&attn, &env, &ext);
    let Some(out) = run_coop_on_gpu("coop_ls_flash", &kernel, &env, &reference, &ext) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("lane-stream flash (sgs=4) on GPU: {}", out.trim());
}

/// The value head dim distributed across lanes: the o slot vectorizes per
/// lane, m/ℓ stay once-per-simdgroup, the QKᵀ contraction lane-splits with
/// a monoid butterfly (score computed once per key per simdgroup), and the
/// sliced partials merge through threadgroup memory.
#[test]
fn coop_lane_axis_flash_matches_oracle() {
    use sanic::emit_metal::emit_fused_metal_sched;
    use sanic::plan::FoldSched;
    let (sq, k, d, e) = (axis("sq"), axis("k"), axis("d"), axis("e"));
    let ext: Extents = [(sq, 5), (k, 48), (d, 32), (e, 32)].into_iter().collect();
    let mut rng = Lcg(0x1A4E);
    let env: Env = [
        ("Q", rand_tensor(&[sq, d], &ext, &mut rng)),
        ("K", rand_tensor(&[k, d], &ext, &mut rng)),
        ("V", rand_tensor(&[k, e], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();
    let attn = attention(
        input("Q", &[sq, d], Dtype::F32),
        input("K", &[k, d], Dtype::F32),
        input("V", &[k, e], Dtype::F32),
        d,
        k,
    );
    let carrier = derive(&attn, k).unwrap();
    let sched = FoldSched {
        lane_axis: Some(e),
        sgs: 8,
        lane_stream: false,
        chunk: 1,
    };
    let kernel = emit_fused_metal_sched("coop_la_flash", &carrier, k, &attn, &ext, sched);
    assert!(kernel.msl.contains("accs_"), "sliced slot emitted");
    assert!(
        kernel.msl.contains("simd_shuffle_xor"),
        "in-body contraction lane-split"
    );
    let reference = eval(&attn, &env, &ext);
    let Some(out) = run_coop_on_gpu("coop_la_flash", &kernel, &env, &reference, &ext) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("lane-axis flash (e→lanes, sgs=8) on GPU: {}", out.trim());
}

/// A two-slot Plain(Add) carrier (RMSNorm fused into a matvec, trinity's
/// projection shape): lane-distributed output rows with the norm slot
/// uniform — the mixed sliced/uniform bookkeeping without a threadgroup
/// merge — plus the lane-stream form.
#[test]
fn coop_norm_fused_matvec_matches_oracle() {
    use sanic::emit_metal::emit_fused_metal_sched;
    use sanic::plan::FoldSched;
    let (o, dm) = (axis("o"), axis("dm"));
    let ext: Extents = [(o, 64), (dm, 128)].into_iter().collect();
    let mut rng = Lcg(0x2517);
    let env: Env = [
        ("x", rand_tensor(&[dm], &ext, &mut rng)),
        ("ln", rand_tensor(&[dm], &ext, &mut rng)),
        ("w", rand_tensor(&[o, dm], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();
    let x = input("x", &[dm], Dtype::F32);
    let dot = reduce(
        map(
            MapOp::Mul,
            vec![
                map(MapOp::Mul, vec![x.clone(), input("ln", &[dm], Dtype::F32)]),
                input("w", &[o, dm], Dtype::F32),
            ],
        ),
        dm,
        BinOp::Monoid(Monoid::Add),
    );
    let ms = map(
        MapOp::Mul,
        vec![
            reduce(
                map(MapOp::Mul, vec![x.clone(), x]),
                dm,
                BinOp::Monoid(Monoid::Add),
            ),
            konst(1.0 / 128.0),
        ],
    );
    let y = map(
        MapOp::Mul,
        vec![
            dot,
            map(
                MapOp::Recip,
                vec![map(
                    MapOp::Sqrt,
                    vec![map(MapOp::Add, vec![ms, konst(1e-5)])],
                )],
            ),
        ],
    );
    let carrier = derive(&y, dm).unwrap();
    let reference = eval(&y, &env, &ext);
    for (label, sched) in [
        (
            "rows→lanes",
            FoldSched {
                lane_axis: Some(o),
                sgs: 1,
                lane_stream: false,
                chunk: 1,
            },
        ),
        (
            "stream→lanes×sgs",
            FoldSched {
                lane_axis: None,
                sgs: 2,
                lane_stream: true,
                chunk: 1,
            },
        ),
    ] {
        let kernel = emit_fused_metal_sched("coop_mv", &carrier, dm, &y, &ext, sched);
        let Some(out) = run_coop_on_gpu(label, &kernel, &env, &reference, &ext) else {
            eprintln!("skipping: no Metal device");
            return;
        };
        eprintln!("norm-fused matvec [{label}] on GPU: {}", out.trim());
    }
}

/// The packed-leaf CHUNKED lane stream: each lane folds contiguous runs of
/// 8 elements, so the int4 nibble loads share bytes and the unrolled index
/// chains constant-fold — the trinity W4 fold shape, bit-checked against
/// the f64 oracle (integer nibbles + f16-exact scales, like the scalar W4
/// test).
#[test]
fn coop_chunked_w4_matvec_matches_oracle() {
    use sanic::emit_metal::emit_fused_metal_sched;
    use sanic::plan::FoldSched;
    let (o, gq, r, c) = (axis("o"), axis("g"), axis("r"), axis("c"));
    let (n_out, n_g, n_r) = (8usize, 8usize, 128usize);
    let n_in = n_g * n_r; // 1024 = 32 lanes × 8-chunks × 4
    let ext: Extents = [(o, n_out), (gq, n_g), (r, n_r), (c, n_in)]
        .into_iter()
        .collect();
    let mut rng = Lcg(0xC4C4);

    let q: Vec<i8> = (0..n_out * n_in)
        .map(|_| ((rng.f() * 8.0).floor().clamp(-8.0, 7.0)) as i8)
        .collect();
    let mut packed = vec![0u8; n_out * n_in / 2];
    for (i, &v) in q.iter().enumerate() {
        let nib = (v + 8) as u8;
        packed[i / 2] |= nib << ((i & 1) * 4);
    }
    let scales: Vec<f64> = (0..n_out * n_g)
        .map(|_| (1.0 + (rng.f().abs() * 15.0).round()) / 64.0)
        .collect();

    let env: Env = [
        (
            "Wq",
            Value::from_fn(&[o, gq, r], &ext, |cd| {
                q[cd[&o] * n_in + cd[&gq] * n_r + cd[&r]] as f64
            }),
        ),
        (
            "S",
            Value::from_fn(&[o, gq], &ext, |cd| scales[cd[&o] * n_g + cd[&gq]]),
        ),
        ("x", rand_tensor(&[gq, r], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let prod = map(
        MapOp::Mul,
        vec![
            map(
                MapOp::Mul,
                vec![
                    input("Wq", &[o, gq, r], Dtype::I4),
                    input("x", &[gq, r], Dtype::F32),
                ],
            ),
            input("S", &[o, gq], Dtype::F32),
        ],
    );
    let y = reduce(flatten(prod, &[gq, r], c), c, BinOp::Monoid(Monoid::Add));
    let carrier = derive(&y, c).unwrap();
    let sched = FoldSched {
        lane_axis: None,
        sgs: 1,
        lane_stream: true,
        chunk: 8,
    };
    let kernel = emit_fused_metal_sched("coop_w4c", &carrier, c, &y, &ext, sched);
    assert!(
        kernel.msl.contains("c_ * 8u"),
        "chunked stream loop emitted"
    );
    assert!(
        kernel.msl.contains("simd_shuffle_xor"),
        "lane merge emitted"
    );
    let reference = eval(&y, &env, &ext);

    let Some(dev) = MetalDevice::open() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let pipes = dev.compile(&kernel.msl);
    let pipe = pipes.get(&kernel.name);
    let inputs: Vec<MetalBuf> = kernel
        .inputs
        .iter()
        .map(|(n, axes)| match *n {
            "Wq" => dev.from_bytes(&packed),
            _ => dev.from_f64(&env[n].permuted_to(axes).data),
        })
        .collect();
    let out = dev.alloc_f32(n_out);
    dev.run(&[Dispatch {
        pipe,
        inputs,
        output: out.clone(),
        grid: kernel.grid_size,
        argbuf: None,
    }]);
    let got = dev.read_f32(&out, n_out);
    let expected = reference.permuted_to(&kernel.grid);
    let maxrel = max_rel_err(&got, &expected.data);
    assert!(maxrel < 2e-3, "chunked W4 matvec MISMATCH {maxrel:e}");
    eprintln!("chunked (8-contiguous per lane) W4 matvec on GPU: GPU OK {maxrel:e}");
}

/// The honest window: a decode-shaped causal flash (pos as DATA) stops its
/// stream loop at the mask edge. The masked tail is an exact f32 no-op —
/// K never wins the max, exp(K − m) underflows to 0.0f — so early exit is
/// bit-preserving, checked here at several positions on both the scalar
/// and the cooperative (lane-clamped) kernels against the FULL-window
/// oracle.
#[test]
fn honest_window_flash_matches_full_oracle() {
    use sanic::emit_metal::emit_fused_metal_sched;
    use sanic::plan::FoldSched;
    let (t, d, e) = (axis("t"), axis("d"), axis("e"));
    let ext: Extents = [(t, 256), (d, 8), (e, 8)].into_iter().collect();
    let mut rng = Lcg(0x90E57);
    let base_env: Env = [
        ("q", rand_tensor(&[d], &ext, &mut rng)),
        ("K", rand_tensor(&[t, d], &ext, &mut rng)),
        ("V", rand_tensor(&[t, e], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    // decode attention: scores + where(pos < iota(t), -1e30, 0), softmax·V
    let scores = matmul(
        input("q", &[d], Dtype::F32),
        input("K", &[t, d], Dtype::F32),
        d,
    );
    let future = map(MapOp::Lt, vec![input("pos", &[], Dtype::F32), iota(t)]);
    let masked = map(
        MapOp::Add,
        vec![
            scores,
            map(MapOp::Where, vec![future, konst(-1e30), konst(0.0)]),
        ],
    );
    let attn = matmul(softmax(masked, t), input("V", &[t, e], Dtype::F32), t);
    let carrier = derive(&attn, t).unwrap();

    for pos in [0usize, 3, 40, 255] {
        let mut env = base_env.clone();
        env.insert("pos", Value::from_fn(&[], &ext, |_| pos as f64));
        let reference = eval(&attn, &env, &ext);

        let kernel = emit_fused_metal("hw_flash", &carrier, t, &attn, &ext);
        assert!(kernel.msl.contains("+ 0.5f) + 1u)"), "window bound emitted");
        let Some(out) = run_on_gpu("hw_flash", &kernel, &env, &reference) else {
            eprintln!("skipping: no Metal device");
            return;
        };
        eprintln!("honest-window scalar flash @pos={pos}: {}", out.trim());

        let sched = FoldSched {
            lane_axis: None,
            sgs: 2,
            lane_stream: true,
            chunk: 1,
        };
        let kc = emit_fused_metal_sched("hw_flash_coop", &carrier, t, &attn, &ext, sched);
        assert!(kc.msl.contains("max((uint)("), "lane-clamped bound emitted");
        let Some(out) = run_coop_on_gpu("hw_flash_coop", &kc, &env, &reference, &ext) else {
            return;
        };
        eprintln!("honest-window coop flash @pos={pos}: {}", out.trim());
    }
}

/// First-max-wins argmax is order-SENSITIVE: an interleaved lane partition
/// would change which duplicate wins, so the schedule must decline to the
/// scalar kernel — and still be correct on planted exact ties.
#[test]
fn coop_declines_order_sensitive_argmax() {
    use sanic::emit_metal::emit_fused_metal_sched;
    use sanic::plan::FoldSched;
    let (b, n) = (axis("b"), axis("n"));
    let ext: Extents = [(b, 4), (n, 64)].into_iter().collect();
    let mut t = rand_tensor(&[b, n], &ext, &mut Lcg(0x715));
    // plant an exact tie in every row: positions 7 and 33 share the max
    for row in 0..4 {
        t.data[row * 64 + 7] = 9.0;
        t.data[row * 64 + 33] = 9.0;
    }
    let env: Env = [("x", t)].into_iter().collect();
    let am = argmax(input("x", &[b, n], Dtype::F32), n);
    let carrier = derive(&am, n).unwrap();
    let sched = FoldSched {
        lane_axis: None,
        sgs: 4,
        lane_stream: true,
        chunk: 1,
    };
    let kernel = emit_fused_metal_sched("coop_am", &carrier, n, &am, &ext, sched);
    assert!(
        !kernel.msl.contains("simd_shuffle_xor"),
        "order-sensitive carrier must fall back to the scalar schedule"
    );
    let reference = eval(&am, &env, &ext);
    let Some(out) = run_coop_on_gpu("argmax-decline", &kernel, &env, &reference, &ext) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!(
        "argmax declined to scalar, ties first-max-wins: {}",
        out.trim()
    );
}

/// A monoidal prefix scan (cumsum) EMITS now: each output point folds its
/// own prefix — a Sequential stage the metal path previously refused.
/// Max-scan too (running maximum), against the oracle.
#[test]
fn monoidal_prefix_scans_run_on_gpu() {
    let (s, t) = (axis("s"), axis("t"));
    let ext: Extents = [(s, 3), (t, 17)].into_iter().collect();
    let mut rng = Lcg(0x5CA9);
    let env: Env = [("X", rand_tensor(&[s, t], &ext, &mut rng))]
        .into_iter()
        .collect();
    for (label, m) in [("cumsum", Monoid::Add), ("cummax", Monoid::Max)] {
        let node = scan(input("X", &[s, t], Dtype::F32), t, BinOp::Monoid(m));
        let kernel = sanic::emit_metal::emit_pointwise_metal(label, &node, &ext);
        let reference = eval(&node, &env, &ext);
        let Some(out) = run_on_gpu(label, &kernel, &env, &reference) else {
            eprintln!("skipping: no Metal device");
            return;
        };
        eprintln!("{label} prefix scan on GPU: {}", out.trim());
    }
}

/// Unified memory means weights need no upload: a page-aligned host region
/// wraps as a device buffer (`newBufferWithBytesNoCopy`) and tensors bind
/// at byte offsets into it. The kernel must read the SAME memory — proven
/// by writing through the host pointer AFTER wrapping and seeing the GPU
/// observe it.
#[test]
fn zero_copy_wrap_binds_tensors_at_offsets() {
    use sanic::metal::MetalDevice;
    let Some(dev) = MetalDevice::open() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    const PAGE: usize = 16384;
    let layout = std::alloc::Layout::from_size_align(PAGE, PAGE).unwrap();
    let region: &'static mut [u8] =
        unsafe { std::slice::from_raw_parts_mut(std::alloc::alloc_zeroed(layout), PAGE) };
    // two "tensors" at offsets 0 and 4096 (4-byte aligned, mid-page)
    let n = 64usize;
    for i in 0..n {
        let b = ((i as f32) * 0.5).to_le_bytes();
        region[i * 4..i * 4 + 4].copy_from_slice(&b);
        region[4096 + i * 4..4096 + i * 4 + 4].copy_from_slice(&(i as f32).to_le_bytes());
    }
    let base = region.as_mut_ptr();
    let whole = dev
        .from_bytes_nocopy(unsafe { std::slice::from_raw_parts(base, PAGE) })
        .expect("page-aligned wrap");
    let a = whole.slice(0);
    let b = whole.slice(4096);

    // y[i] = A[i] + B[i], a trivial kernel over the two offset views
    let msl = r#"#include <metal_stdlib>
using namespace metal;
kernel void zc_add(
    device const float* A [[buffer(0)]],
    device const float* B [[buffer(1)]],
    device float* outb [[buffer(2)]],
    uint gid [[thread_position_in_grid]]
) { if (gid < 64) outb[gid] = A[gid] + B[gid]; }
"#;
    let pipes = dev.compile(msl);
    let out = dev.alloc_f32(n);
    let run = |dev: &MetalDevice| {
        dev.run(&[Dispatch {
            pipe: pipes.get("zc_add"),
            inputs: vec![a.clone(), b.clone()],
            output: out.clone(),
            grid: n,
            argbuf: None,
        }]);
        dev.read_f32(&out, n)
    };
    let got = run(&dev);
    for i in 0..n {
        assert_eq!(got[i], (i as f32) * 1.5, "offset-bound reads");
    }
    // the zero-copy property itself: mutate through the HOST pointer, the
    // device sees it without any re-upload
    unsafe { (base as *mut f32).write(100.0) };
    let got = run(&dev);
    assert_eq!(got[0], 100.0, "host write visible to the GPU: same memory");
    eprintln!("zero-copy wrap + offset binding on GPU: OK");
}
