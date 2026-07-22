//! Low-level Metal coverage for the positional IR. Public backend coverage
//! also lives in `tests/compile.rs`; these tests exercise emitted carriers,
//! cooperative schedules, dtypes, replay, and the device runtime directly.
//!
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

use sanic::cost::DeviceProfile;
use sanic::derive::{Carrier, derive};
use sanic::emit_metal::{
    MetalKernel, MetalProgram, emit_fused_metal_sched_with, emit_fused_metal_with,
    emit_schedule_metal_on,
};
use sanic::interp::{Env, Value, eval};
use sanic::ir::*;
use sanic::metal::{Dispatch, MetalBuf, MetalDevice, program_dispatches};
use sanic::partition::partition;

fn emit_fused_metal(name: &str, carrier: &Carrier, stream: AxisRef, node: &NodeRef) -> MetalKernel {
    emit_fused_metal_with(name, carrier, stream, node, None)
}

fn emit_fused_metal_sched(
    name: &str,
    carrier: &Carrier,
    stream: AxisRef,
    node: &NodeRef,
    schedule: sanic::plan::FoldSched,
) -> MetalKernel {
    emit_fused_metal_sched_with(name, carrier, stream, node, schedule, None)
}

fn emit_schedule_metal(schedule: &sanic::partition::Schedule) -> MetalProgram {
    emit_schedule_metal_on(&DeviceProfile::toy(), schedule)
}

fn attention(query: NodeRef, key: NodeRef, value: NodeRef) -> NodeRef {
    let scores = matmul(query, transpose(key, -2isize, -1isize));
    matmul(softmax(scores, -1isize), value)
}

fn linear_vector(vector: NodeRef, weight: NodeRef) -> NodeRef {
    reduce(map(MapOp::Mul, vec![weight, vector]), 1usize, Monoid::Add)
}

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
fn rand_tensor(axes: &[Axis], rng: &mut Lcg) -> Value {
    Value::from_shape_fn(
        &axes.iter().map(|axis| axis.extent()).collect::<Vec<_>>(),
        |_| rng.f(),
    )
}

fn max_rel_err(got: &[f32], expected: &[f64]) -> f64 {
    assert_eq!(got.len(), expected.len(), "output length");
    got.iter()
        .zip(expected)
        .map(|(g, e)| (*g as f64 - e).abs() / (1.0 + e.abs()))
        .fold(0.0, |worst, e| std::cmp::max_by(worst, e, f64::total_cmp))
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
        .map(|(n, _)| dev.from_f64(&env[n].data))
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
    let maxrel = max_rel_err(&got, &reference.data);
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
    for (n, _) in &program.inputs {
        bufs.insert(n.to_string(), dev.from_f64(&env[n].data));
    }
    for (n, size) in &program.buffers {
        bufs.insert(n.clone(), dev.alloc_f32(*size));
    }
    dev.run(&program_dispatches(program, &bufs, &pipes));
    let output = &program
        .stages
        .last()
        .expect("Metal program has no stages")
        .output;
    let got = dev.read_f32(&bufs[output], reference.data.len());
    let maxrel = max_rel_err(&got, &reference.data);
    assert!(maxrel < 3e-3, "GPU MISMATCH {maxrel:e} ({label})");
    Some(format!("GPU OK {maxrel:e}"))
}

// ── the derived FlashAttention kernel, executed on the GPU ───────────────────
#[test]
fn flash_attention_runs_on_gpu() {
    let (sq, k, d, e) = (axis("sq", 16), axis("k", 24), axis("d", 8), axis("e", 8));
    let mut rng = Lcg(0x6D5A);
    let env: Env = [
        ("Q", rand_tensor(&[sq, d], &mut rng)),
        ("K", rand_tensor(&[k, d], &mut rng)),
        ("V", rand_tensor(&[k, e], &mut rng)),
    ]
    .into_iter()
    .collect();

    let key = input("K", [k, d], Dtype::F32);
    let stream = source_axis(&key, 0);
    let attn = attention(
        input("Q", [sq, d], Dtype::F32),
        key,
        input("V", [k, e], Dtype::F32),
    );
    let carrier = derive(&attn, stream).unwrap();
    let kernel = emit_fused_metal("flash", &carrier, stream, &attn);
    let reference = eval(&attn, &env);

    let Some(out) = run_on_gpu("flash", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("flash on GPU: {}", out.trim());
}

#[test]
fn masked_gqa_decode_attention_matches_oracle_on_gpu() {
    let (query_heads, kv_heads, cache, features) = (
        axis("query_heads", 32),
        axis("kv_heads", 8),
        axis("cache", 7),
        axis("features", 64),
    );
    let query_sequence = axis("query_sequence", 1);
    let mut rng = Lcg(0x6A7A);
    let mut key = rand_tensor(&[kv_heads, cache, features], &mut rng);
    let mut value = rand_tensor(&[kv_heads, cache, features], &mut rng);
    for head in 0..kv_heads.extent() {
        for position in 1..cache.extent() {
            for feature in 0..features.extent() {
                let offset = (head * cache.extent() + position) * features.extent() + feature;
                key.data[offset] = 0.0;
                value.data[offset] = 0.0;
            }
        }
    }
    let env: Env = [
        (
            "Q",
            rand_tensor(&[query_heads, query_sequence, features], &mut rng),
        ),
        ("K", key),
        ("V", value),
        (
            "mask",
            Value::from_shape_fn(&[cache.extent()], |position| {
                if position[0] == 0 {
                    0.0
                } else {
                    f64::NEG_INFINITY
                }
            }),
        ),
    ]
    .into_iter()
    .collect();
    let attention = sanic::nn::functional::scaled_dot_product_attention(
        input("Q", [query_heads, query_sequence, features], Dtype::F32),
        input("K", [kv_heads, cache, features], Dtype::F32),
        input("V", [kv_heads, cache, features], Dtype::F32),
        Some(input("mask", [cache], Dtype::F32)),
        0.0,
        false,
        None,
        true,
    );
    let reference = eval(&attention, &env);
    let schedule = partition(&attention, &DeviceProfile::m1_pro());
    let program = emit_schedule_metal_on(&DeviceProfile::m1_pro(), &schedule);

    let Some(result) = run_schedule_on_gpu("masked-gqa-decode", &program, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("masked GQA decode attention: {}", result.trim());
}

#[test]
fn rmsnorm_of_a_shared_residual_matches_oracle_on_gpu() {
    let (token, hidden) = (axis("token", 1), axis("hidden", 2048));
    let mut rng = Lcg(0xA11CE);
    let env: Env = [
        ("a", rand_tensor(&[token, hidden], &mut rng)),
        ("b", rand_tensor(&[token, hidden], &mut rng)),
        ("c", rand_tensor(&[token, hidden], &mut rng)),
        ("gain", rand_tensor(&[hidden], &mut rng)),
    ]
    .into_iter()
    .collect();
    let residual = map(
        MapOp::Add,
        vec![
            map(
                MapOp::Add,
                vec![
                    input("a", [token, hidden], Dtype::F32),
                    input("b", [token, hidden], Dtype::F32),
                ],
            ),
            input("c", [token, hidden], Dtype::F32),
        ],
    );
    let mean_square = map(
        MapOp::Mul,
        vec![
            reduce(
                map(MapOp::Mul, vec![residual.clone(), residual.clone()]),
                1usize,
                Monoid::Add,
            ),
            konst(1.0 / hidden.extent() as f64),
        ],
    );
    let norm = map(
        MapOp::Div,
        vec![
            map(
                MapOp::Mul,
                vec![residual, input("gain", [hidden], Dtype::F32)],
            ),
            unsqueeze(
                map(
                    MapOp::Sqrt,
                    vec![map(MapOp::Add, vec![mean_square, konst(1e-5)])],
                ),
                1usize,
            ),
        ],
    );
    let reference = eval(&norm, &env);
    let schedule = partition(&norm, &DeviceProfile::m1_pro());
    let program = emit_schedule_metal_on(&DeviceProfile::m1_pro(), &schedule);

    let Some(result) = run_schedule_on_gpu("residual-rmsnorm", &program, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("shared-residual RMSNorm: {}", result.trim());
}

#[test]
fn residual_rmsnorm_fused_projection_matches_oracle_on_gpu() {
    let (token, hidden, output) = (axis("token", 1), axis("hidden", 2048), axis("output", 64));
    let mut rng = Lcg(0xF053D);
    let env: Env = [
        ("a", rand_tensor(&[token, hidden], &mut rng)),
        ("b", rand_tensor(&[token, hidden], &mut rng)),
        ("c", rand_tensor(&[token, hidden], &mut rng)),
        ("gain", rand_tensor(&[hidden], &mut rng)),
        ("weight", rand_tensor(&[output, hidden], &mut rng)),
    ]
    .into_iter()
    .collect();
    let residual = map(
        MapOp::Add,
        vec![
            map(
                MapOp::Add,
                vec![
                    input("a", [token, hidden], Dtype::F32),
                    input("b", [token, hidden], Dtype::F32),
                ],
            ),
            input("c", [token, hidden], Dtype::F32),
        ],
    );
    let mean_square = map(
        MapOp::Mul,
        vec![
            reduce(
                map(MapOp::Mul, vec![residual.clone(), residual.clone()]),
                1usize,
                Monoid::Add,
            ),
            konst(1.0 / hidden.extent() as f64),
        ],
    );
    let norm = map(
        MapOp::Div,
        vec![
            map(
                MapOp::Mul,
                vec![residual, input("gain", [hidden], Dtype::F32)],
            ),
            unsqueeze(
                map(
                    MapOp::Sqrt,
                    vec![map(MapOp::Add, vec![mean_square, konst(1e-5)])],
                ),
                1usize,
            ),
        ],
    );
    let projection = matmul(
        norm,
        transpose(
            input("weight", [output, hidden], Dtype::F32),
            0usize,
            1usize,
        ),
    );
    let reference = eval(&projection, &env);
    let schedule = partition(&projection, &DeviceProfile::m1_pro());
    let program = emit_schedule_metal_on(&DeviceProfile::m1_pro(), &schedule);

    let Some(result) =
        run_schedule_on_gpu("residual-rmsnorm-projection", &program, &env, &reference)
    else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("residual RMSNorm projection: {}", result.trim());
}

// ── causal-masked + scaled flash (computed mask), on the GPU ─────────────────
#[test]
fn causal_flash_runs_on_gpu() {
    let (s, t, dk, dv) = (axis("s", 16), axis("t", 16), axis("dk", 8), axis("dv", 8));
    let mut rng = Lcg(0xCA05A1);
    let env: Env = [
        ("Q", rand_tensor(&[s, dk], &mut rng)),
        ("K", rand_tensor(&[t, dk], &mut rng)),
        ("V", rand_tensor(&[t, dv], &mut rng)),
    ]
    .into_iter()
    .collect();

    let key = input("K", [t, dk], Dtype::F32);
    let stream = source_axis(&key, 0);
    let scores = matmul(
        input("Q", [s, dk], Dtype::F32),
        transpose(key, 0usize, 1usize),
    );
    let scaled = map(MapOp::Mul, vec![scores, konst(0.125)]);
    let masked = map(
        MapOp::Add,
        vec![scaled.clone(), causal_mask_like(scaled, 0usize, 1usize)],
    );
    let attn = matmul(softmax(masked, 1usize), input("V", [t, dv], Dtype::F32));

    let carrier = derive(&attn, stream).unwrap();
    let kernel = emit_fused_metal("causal_flash", &carrier, stream, &attn);
    let reference = eval(&attn, &env);

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
    let (s, t, dk, dv) = (axis("s", 16), axis("t", 16), axis("dk", 8), axis("dv", 8));
    let mut rng = Lcg(0xB1A5C0);
    let env: Env = [
        ("Q", rand_tensor(&[s, dk], &mut rng)),
        ("K", rand_tensor(&[t, dk], &mut rng)),
        ("V", rand_tensor(&[t, dv], &mut rng)),
    ]
    .into_iter()
    .collect();

    let key = input("K", [t, dk], Dtype::F32);
    let stream = source_axis(&key, 0);
    let scores = matmul(
        input("Q", [s, dk], Dtype::F32),
        transpose(key, 0usize, 1usize),
    );
    let rel = map(
        MapOp::Sub,
        vec![
            coordinate(scores.clone(), 0usize),
            coordinate(scores.clone(), 1usize),
        ],
    );
    let bias = map(MapOp::Cos, vec![map(MapOp::Mul, vec![rel, konst(0.1)])]);
    let biased = map(MapOp::Add, vec![scores, bias]);
    let attn = matmul(softmax(biased, 1usize), input("V", [t, dv], Dtype::F32));

    let carrier = derive(&attn, stream).unwrap();
    let kernel = emit_fused_metal("cos_bias_flash", &carrier, stream, &attn);
    let reference = eval(&attn, &env);

    let Some(out) = run_on_gpu("cosbias", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("cosine-bias flash on GPU: {}", out.trim());
}

/// A computed RoPE rotation matrix `R[pos, p, i, j]` (extent 2 on i, j): the
/// 2×2 rotation by θ = pos·freq_p, freq_p = exp(p·c) — synthesized from
/// indices, no rotation tensor in memory.
fn rope_rotation(pos: Axis, p: Axis, j: Axis, i: Axis, c: f64) -> NodeRef {
    let template = positional_reindex(konst(0.0), vec![pos, p, i, j], Vec::new(), false);
    let pos_coord = coordinate(template.clone(), 0usize);
    let pair_coord = coordinate(template.clone(), 1usize);
    let i_coord = coordinate(template.clone(), 2usize);
    let j_coord = coordinate(template, 3usize);
    let freq = map(
        MapOp::Exp,
        vec![map(MapOp::Mul, vec![pair_coord, konst(c)])],
    );
    let theta = map(MapOp::Mul, vec![pos_coord, freq]);
    let lt_ij = map(MapOp::Lt, vec![i_coord.clone(), j_coord.clone()]);
    let lt_ji = map(MapOp::Lt, vec![j_coord, i_coord]);
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
        axis("s", 12),
        axis("t", 12),
        axis("p", 4),
        axis("i", 2),
        axis("j", 2),
        axis("dk", 8),
        axis("e", 8),
    );
    let c = -0.4;
    let mut rng = Lcg(0x60EA77);
    let env: Env = [
        ("Q", rand_tensor(&[s, p, i], &mut rng)),
        ("K", rand_tensor(&[t, p, i], &mut rng)),
        ("V", rand_tensor(&[t, e], &mut rng)),
    ]
    .into_iter()
    .collect();

    let qr = squeeze(
        matmul(
            unsqueeze(input("Q", [s, p, i], Dtype::F32), 2usize),
            rope_rotation(s, p, j, i, c),
        ),
        2usize,
    );
    let kr = squeeze(
        matmul(
            unsqueeze(input("K", [t, p, i], Dtype::F32), 2usize),
            rope_rotation(t, p, j, i, c),
        ),
        2usize,
    );
    let stream = source_axis(&kr, 0);
    let q_flat = flatten(qr, &[1usize, 2usize][..], dk);
    let k_flat = flatten(kr, &[1usize, 2usize][..], dk);
    let scores = matmul(q_flat, transpose(k_flat, 0usize, 1usize));
    let attn = matmul(softmax(scores, 1usize), input("V", [t, e], Dtype::F32));

    let carrier = derive(&attn, stream).unwrap();
    let kernel = emit_fused_metal("rope_flash", &carrier, stream, &attn);
    let reference = eval(&attn, &env);
    let Some(out) = run_on_gpu("rope", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("RoPE flash on GPU: {}", out.trim());
}

// ── quantized matmul (dequant-fused) on the GPU ──────────────────────────────
#[test]
fn quantized_matmul_runs_on_gpu() {
    let (s, dm, o) = (axis("s", 8), axis("dm", 32), axis("o", 16));
    let mut rng = Lcg(0x9114A7);
    let qw = Value::from_shape_fn(&[o.extent(), dm.extent()], |_| (rng.f() * 8.0).round());
    let env: Env = [
        ("X", rand_tensor(&[s, dm], &mut rng)),
        ("qW", qw),
        (
            "scale",
            Value::from_shape_fn(&[o.extent()], |_| 0.05 * (rng.f() + 1.5)),
        ),
    ]
    .into_iter()
    .collect();

    let dw = map(
        MapOp::Mul,
        vec![
            input("qW", [o, dm], Dtype::F32),
            unsqueeze(input("scale", [o], Dtype::F32), 1usize),
        ],
    );
    let x = input("X", [s, dm], Dtype::F32);
    let stream = source_axis(&x, 1);
    let y = matmul(x, transpose(dw, 0usize, 1usize));
    let carrier = derive(&y, stream).unwrap();
    let kernel = emit_fused_metal("quant_matmul", &carrier, stream, &y);
    let reference = eval(&y, &env);
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
    let (s, dm, v) = (axis("s", 6), axis("dm", 16), axis("v", 48));
    let mut rng = Lcg(0x6EED);
    let env: Env = [
        ("Y", rand_tensor(&[s, dm], &mut rng)),
        ("W_lm", rand_tensor(&[v, dm], &mut rng)),
    ]
    .into_iter()
    .collect();

    let logits = matmul(
        input("Y", [s, dm], Dtype::F32),
        transpose(input("W_lm", [v, dm], Dtype::F32), 0usize, 1usize),
    ); // [s, v]
    let token = argmax(logits, 1usize); // [s]

    let sched = partition(&token, &DeviceProfile::toy());
    let program = emit_schedule_metal(&sched);
    let reference = eval(&token, &env);
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
        axis("v", 16),
        axis("s", 4),
        axis("t", 4),
        axis("dm", 8),
        axis("h", 2),
        axis("dk", 4),
        axis("dvh", 4),
        axis("dmv", 8),
        axis("f", 10),
    );
    let n = dm.extent() as f64;
    let mut rng = Lcg(0xDEC0DE);
    let env: Env = [
        (
            "ids",
            Value::from_shape_fn(&[s.extent()], |c| [1.0, 7.0, 3.0, 12.0][c[0]]),
        ),
        ("E", rand_tensor(&[vv, dm], &mut rng)),
        ("g1", rand_tensor(&[dm], &mut rng)),
        ("g2", rand_tensor(&[dm], &mut rng)),
        ("Wq", rand_tensor(&[h, dk, dm], &mut rng)),
        ("Wk", rand_tensor(&[h, dk, dm], &mut rng)),
        ("Wv", rand_tensor(&[h, dvh, dm], &mut rng)),
        ("Wo", rand_tensor(&[dmv, dm], &mut rng)),
        ("Wg", rand_tensor(&[f, dm], &mut rng)),
        ("Wu", rand_tensor(&[f, dm], &mut rng)),
        ("Wd", rand_tensor(&[f, dm], &mut rng)),
        ("W_lm", rand_tensor(&[vv, dm], &mut rng)),
    ]
    .into_iter()
    .collect();

    let rms = |x: NodeRef, g: NodeRef| {
        let dim = x.shape().len() - 1;
        let ss = reduce(
            map(MapOp::Mul, vec![x.clone(), x.clone()]),
            dim,
            Monoid::Add,
        );
        let mean = map(MapOp::Mul, vec![ss, konst(1.0 / n)]);
        let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
        map(
            MapOp::Div,
            vec![map(MapOp::Mul, vec![x, g]), unsqueeze(denom, dim)],
        )
    };
    let x = embedding(
        input("E", [vv, dm], Dtype::F32),
        input("ids", [s], Dtype::F32),
        0usize,
    ); // [s, dm]
    let xn = rms(x.clone(), input("g1", [dm], Dtype::F32));
    let xn_kv = rename(xn.clone(), 0usize, t);
    let q = matmul(
        xn,
        transpose(input("Wq", [h, dk, dm], Dtype::F32), 1usize, 2usize),
    );
    let k = matmul(
        xn_kv.clone(),
        transpose(input("Wk", [h, dk, dm], Dtype::F32), 1usize, 2usize),
    );
    let vvv = matmul(
        xn_kv,
        transpose(input("Wv", [h, dvh, dm], Dtype::F32), 1usize, 2usize),
    );
    let scores = matmul(q, transpose(k, 1usize, 2usize));
    let scaled = map(
        MapOp::Mul,
        vec![scores, konst(1.0 / (dk.extent() as f64).sqrt())],
    );
    let masked = map(
        MapOp::Add,
        vec![scaled.clone(), causal_mask_like(scaled, 1usize, 2usize)],
    );
    let attn = matmul(softmax(masked, 2usize), vvv);
    let flat = flatten(transpose(attn, 0usize, 1usize), &[1usize, 2usize][..], dmv);
    let o = matmul(flat, input("Wo", [dmv, dm], Dtype::F32));
    let res1 = map(MapOp::Add, vec![o, x]);
    let hn = rms(res1.clone(), input("g2", [dm], Dtype::F32));
    let gate = matmul(
        hn.clone(),
        transpose(input("Wg", [f, dm], Dtype::F32), 0usize, 1usize),
    );
    let up = matmul(
        hn,
        transpose(input("Wu", [f, dm], Dtype::F32), 0usize, 1usize),
    );
    let act = map(MapOp::Mul, vec![silu(gate), up]);
    let mlp = matmul(act, input("Wd", [f, dm], Dtype::F32));
    let yb = map(MapOp::Add, vec![mlp, res1]);
    let logits = matmul(
        yb,
        transpose(input("W_lm", [vv, dm], Dtype::F32), 0usize, 1usize),
    ); // [s, v]
    let token = argmax(logits, 1usize); // [s] — next-token per position

    let sched = partition(&token, &DeviceProfile::toy());
    let program = emit_schedule_metal(&sched);
    let reference = eval(&token, &env);
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
        axis("v", 12),
        axis("s", 4),
        axis("t", 4),
        axis("dm", 8),
        axis("h", 2),
        axis("dk", 4),
        axis("dv", 4),
        axis("dmv", 8),
        axis("f", 10),
    );
    let n = dm.extent() as f64;
    let mut rng = Lcg(0xB10C6);
    let env: Env = [
        ("X", rand_tensor(&[s, dm], &mut rng)),
        ("g1", rand_tensor(&[dm], &mut rng)),
        ("g2", rand_tensor(&[dm], &mut rng)),
        ("Wq", rand_tensor(&[h, dk, dm], &mut rng)),
        ("Wk", rand_tensor(&[h, dk, dm], &mut rng)),
        ("Wv", rand_tensor(&[h, dv, dm], &mut rng)),
        ("Wo", rand_tensor(&[dmv, dm], &mut rng)),
        ("Wg", rand_tensor(&[f, dm], &mut rng)),
        ("Wu", rand_tensor(&[f, dm], &mut rng)),
        ("Wd", rand_tensor(&[f, dm], &mut rng)),
        ("W_lm", rand_tensor(&[v, dm], &mut rng)),
    ]
    .into_iter()
    .collect();

    let rms = |x: NodeRef, g: NodeRef| {
        let dim = x.shape().len() - 1;
        let ss = reduce(
            map(MapOp::Mul, vec![x.clone(), x.clone()]),
            dim,
            Monoid::Add,
        );
        let mean = map(MapOp::Mul, vec![ss, konst(1.0 / n)]);
        let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
        map(
            MapOp::Div,
            vec![map(MapOp::Mul, vec![x, g]), unsqueeze(denom, dim)],
        )
    };
    let x = input("X", [s, dm], Dtype::F32);
    let xn = rms(x.clone(), input("g1", [dm], Dtype::F32));
    let xn_kv = rename(xn.clone(), 0usize, t);
    let q = matmul(
        xn,
        transpose(input("Wq", [h, dk, dm], Dtype::F32), 1usize, 2usize),
    );
    let k = matmul(
        xn_kv.clone(),
        transpose(input("Wk", [h, dk, dm], Dtype::F32), 1usize, 2usize),
    );
    let vv = matmul(
        xn_kv,
        transpose(input("Wv", [h, dv, dm], Dtype::F32), 1usize, 2usize),
    );
    let scores = matmul(q, transpose(k, 1usize, 2usize));
    let scaled = map(
        MapOp::Mul,
        vec![scores, konst(1.0 / (dk.extent() as f64).sqrt())],
    );
    let masked = map(
        MapOp::Add,
        vec![scaled.clone(), causal_mask_like(scaled, 1usize, 2usize)],
    );
    let attn = matmul(softmax(masked, 2usize), vv);
    let flat = flatten(transpose(attn, 0usize, 1usize), &[1usize, 2usize][..], dmv);
    let o = matmul(flat, input("Wo", [dmv, dm], Dtype::F32));
    let res1 = map(MapOp::Add, vec![o, x]);
    let hn = rms(res1.clone(), input("g2", [dm], Dtype::F32));
    let gate = matmul(
        hn.clone(),
        transpose(input("Wg", [f, dm], Dtype::F32), 0usize, 1usize),
    );
    let up = matmul(
        hn,
        transpose(input("Wu", [f, dm], Dtype::F32), 0usize, 1usize),
    );
    let act = map(MapOp::Mul, vec![silu(gate), up]);
    let mlp = matmul(act, input("Wd", [f, dm], Dtype::F32));
    let yb = map(MapOp::Add, vec![mlp, res1]);
    let logits = matmul(
        yb,
        transpose(input("W_lm", [v, dm], Dtype::F32), 0usize, 1usize),
    );

    let sched = partition(&logits, &DeviceProfile::toy());
    let program = emit_schedule_metal(&sched);
    let reference = eval(&logits, &env);
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
        axis("ci", 3),
        axis("h0", 10),
        axis("w0", 12),
        axis("oh", 8),
        axis("ow", 10),
        axis("kh", 3),
        axis("kw", 3),
        axis("r", 27),
        axis("co", 8),
    );
    let mut rng = Lcg(0x6042D);
    let env: Env = [
        ("X", rand_tensor(&[ci, h0, w0], &mut rng)),
        ("W", rand_tensor(&[co, ci, kh, kw], &mut rng)),
    ]
    .into_iter()
    .collect();

    let xw = window(
        window(input("X", [ci, h0, w0], Dtype::F32), 1usize, oh, kh, 1, 1),
        3usize,
        ow,
        kw,
        1,
        1,
    );
    let xw = positional_view(
        xw,
        vec![
            ViewDim {
                sources: vec![1],
                axis: oh,
            },
            ViewDim {
                sources: vec![3],
                axis: ow,
            },
            ViewDim {
                sources: vec![0],
                axis: ci,
            },
            ViewDim {
                sources: vec![2],
                axis: kh,
            },
            ViewDim {
                sources: vec![4],
                axis: kw,
            },
        ],
    );
    let xf = flatten(xw, &[2usize, 3usize, 4usize][..], r);
    let wf = flatten(
        input("W", [co, ci, kh, kw], Dtype::F32),
        &[1usize, 2usize, 3usize][..],
        r,
    );
    let stream = source_axis(&xf, 2);
    let conv = matmul(xf, transpose(wf, 0usize, 1usize));

    let carrier = derive(&conv, stream).unwrap();
    let kernel = emit_fused_metal("conv2d", &carrier, stream, &conv);
    let reference = eval(&conv, &env);

    let Some(out) = run_on_gpu("conv2d", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("conv2d on GPU: {}", out.trim());
}

// ── sliding-window flash attention (padded windowed K/V), on the GPU ─────────
#[test]
fn sliding_window_flash_runs_on_gpu() {
    let (ns, w) = (24usize, 6usize);
    let (s, t, j, d, e) = (
        axis("s", ns),
        axis("t", ns),
        axis("j", w),
        axis("d", 8),
        axis("e", 8),
    );
    let mut rng = Lcg(0x51DE60);
    let env: Env = [
        ("Q", rand_tensor(&[s, d], &mut rng)),
        ("K", rand_tensor(&[t, d], &mut rng)),
        ("V", rand_tensor(&[t, e], &mut rng)),
    ]
    .into_iter()
    .collect();

    let off = -((w - 1) as i64);
    let kw = positional_reindex(
        input("K", [t, d], Dtype::F32),
        vec![s, j, d],
        vec![(0, vec![(1, 0), (1, 1)], off), (1, vec![(1, 2)], 0)],
        true,
    );
    let stream = source_axis(&kw, 1);
    let vw = positional_reindex(
        input("V", [t, e], Dtype::F32),
        vec![s, j, e],
        vec![(0, vec![(1, 0), (1, 1)], off), (1, vec![(1, 2)], 0)],
        true,
    );
    let scores = reduce(
        map(
            MapOp::Mul,
            vec![unsqueeze(input("Q", [s, d], Dtype::F32), 1usize), kw],
        ),
        2usize,
        Monoid::Add,
    );
    let invalid = map(
        MapOp::Lt,
        vec![
            map(
                MapOp::Add,
                vec![
                    coordinate(scores.clone(), 0usize),
                    coordinate(scores.clone(), 1usize),
                ],
            ),
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
    let attn = reduce(
        map(
            MapOp::Mul,
            vec![unsqueeze(softmax(masked, 1usize), 2usize), vw],
        ),
        1usize,
        Monoid::Add,
    );

    let carrier = derive(&attn, stream).unwrap();
    let kernel = emit_fused_metal("swa_flash", &carrier, stream, &attn);
    let reference = eval(&attn, &env);

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
    let steps = 6usize;
    let (t, s, t2, dm, dk, dv, v) = (
        axis("t", steps),
        axis("s", steps),
        axis("t2", steps),
        axis("dm", 8),
        axis("dk", 6),
        axis("dv", 6),
        axis("v", 12),
    );
    let mut rng = Lcg(0xDEC0DE60);
    let wq = rand_tensor(&[dk, dm], &mut rng);
    let wk = rand_tensor(&[dk, dm], &mut rng);
    let wv = rand_tensor(&[dv, dm], &mut rng);
    let wl = rand_tensor(&[v, dv], &mut rng);
    let xs = rand_tensor(&[s, dm], &mut rng);
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
    let x = input("x", [dm], Dtype::F32);
    let pos = input("pos", [], Dtype::F32);
    let new_k = linear_vector(x.clone(), input("Wk", [dk, dm], Dtype::F32));
    let new_v = linear_vector(x.clone(), input("Wv", [dv, dm], Dtype::F32));
    let q = linear_vector(x, input("Wq", [dk, dm], Dtype::F32));
    let cache_k = input("cache_k", [t, dk], Dtype::F32);
    let ck = map(
        MapOp::Where,
        vec![
            one_hot_like(cache_k.clone(), 0usize, pos.clone()),
            new_k,
            cache_k,
        ],
    );
    let cache_v = input("cache_v", [t, dv], Dtype::F32);
    let cv = map(
        MapOp::Where,
        vec![
            one_hot_like(cache_v.clone(), 0usize, pos.clone()),
            new_v,
            cache_v,
        ],
    );
    let scale = konst(1.0 / (dk.extent() as f64).sqrt());
    let scores = map(
        MapOp::Mul,
        vec![
            reduce(map(MapOp::Mul, vec![ck.clone(), q]), 1usize, Monoid::Add),
            scale,
        ],
    );
    let future = map(MapOp::Lt, vec![pos, iota(t)]);
    let masked = map(
        MapOp::Add,
        vec![
            scores,
            map(MapOp::Where, vec![future, konst(-1e30), konst(0.0)]),
        ],
    );
    let att = softmax(masked, 0usize);
    let out = reduce(
        map(MapOp::Mul, vec![unsqueeze(att, 1usize), cv.clone()]),
        0usize,
        Monoid::Add,
    );
    let logits = linear_vector(out, input("Wl", [v, dv], Dtype::F32));
    let sched = sanic::partition::partition_many(
        &[(ck, "ck_new"), (cv, "cv_new"), (logits, "logits")],
        &DeviceProfile::toy(),
    );
    let program = emit_schedule_metal(&sched);

    // the reference: full causal prefill by the oracle
    let xq = input("X", [s, dm], Dtype::F32);
    let xt = rename(xq.clone(), 0usize, t2);
    let qa = matmul(
        xq,
        transpose(input("Wq", [dk, dm], Dtype::F32), 0usize, 1usize),
    );
    let ka = matmul(
        xt.clone(),
        transpose(input("Wk", [dk, dm], Dtype::F32), 0usize, 1usize),
    );
    let va = matmul(
        xt,
        transpose(input("Wv", [dv, dm], Dtype::F32), 0usize, 1usize),
    );
    let sc = map(
        MapOp::Mul,
        vec![
            matmul(qa, transpose(ka, 0usize, 1usize)),
            konst(1.0 / (dk.extent() as f64).sqrt()),
        ],
    );
    let ma = map(
        MapOp::Add,
        vec![sc.clone(), causal_mask_like(sc, 0usize, 1usize)],
    );
    let oa = matmul(softmax(ma, 1usize), va);
    let logits_ref = eval(
        &matmul(
            oa,
            transpose(input("Wl", [v, dv], Dtype::F32), 0usize, 1usize),
        ),
        &env,
    );
    let expected: Vec<f64> = (0..steps)
        .flat_map(|p| {
            (0..v.extent())
                .map(|vi| logits_ref.at_index(&[p, vi]))
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
        bufs.insert(name.to_string(), dev.from_f64(&tensor.data));
    }
    bufs.insert("cache_k".into(), dev.alloc_f32(steps * dk.extent()));
    bufs.insert("cache_v".into(), dev.alloc_f32(steps * dv.extent()));
    bufs.insert("x".into(), dev.alloc_f32(dm.extent()));
    bufs.insert("pos".into(), dev.alloc_f32(1));
    for (n, size) in &program.buffers {
        bufs.insert(n.clone(), dev.alloc_f32(*size));
    }

    let mut got: Vec<f32> = Vec::new();
    for p in 0..steps {
        let row: Vec<f64> = (0..dm.extent()).map(|di| xs.at_index(&[p, di])).collect();
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
        got.extend(dev.read_f32(&bufs["logits"], v.extent()));
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
    let (s, t, dk, dv) = (axis("s", 8), axis("t", 8), axis("dk", 6), axis("dv", 6));
    let mut rng = Lcg(0x6ADB);
    let env: Env = [
        ("Q", rand_tensor(&[s, dk], &mut rng)),
        ("K", rand_tensor(&[t, dk], &mut rng)),
        ("V", rand_tensor(&[t, dv], &mut rng)),
    ]
    .into_iter()
    .collect();

    let scores = matmul(
        input("Q", [s, dk], Dtype::F32),
        transpose(input("K", [t, dk], Dtype::F32), 0usize, 1usize),
    );
    let masked = map(
        MapOp::Add,
        vec![scores.clone(), causal_mask_like(scores, 0usize, 1usize)],
    );
    let out = matmul(softmax(masked, 1usize), input("V", [t, dv], Dtype::F32));
    let sq = map(MapOp::Mul, vec![out.clone(), out]);
    let loss = reduce(reduce(sq, 0usize, Monoid::Add), 0usize, Monoid::Add);

    let grads = sanic::grad::grad(&loss, &["Q"]);
    let g = &grads["Q"];
    let sched = partition(g, &DeviceProfile::toy());
    let program = emit_schedule_metal(&sched);
    let reference = eval(g, &env);

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
// cooperatively: 16 simdgroups fold chunks per output point
// into raw (m, ℓ, o) states; stage 2 merges them with the carrier's own
// rescaling combine and projects. Matches the one-pass oracle — the monoid
// law, executed on hardware.
#[test]
fn split_reduction_runs_on_gpu() {
    let (s, k, d, e) = (axis("s", 4), axis("k", 4096), axis("d", 8), axis("e", 8));
    let mut rng = Lcg(0x5B117);
    let env: Env = [
        ("Q", rand_tensor(&[s, d], &mut rng)),
        ("K", rand_tensor(&[k, d], &mut rng)),
        ("V", rand_tensor(&[k, e], &mut rng)),
    ]
    .into_iter()
    .collect();

    let key = input("K", [k, d], Dtype::F32);
    let stream = source_axis(&key, 0);
    let attn = attention(
        input("Q", [s, d], Dtype::F32),
        key,
        input("V", [k, e], Dtype::F32),
    );
    let carrier = derive(&attn, stream).unwrap();
    let kernel = emit_fused_metal_sched(
        "swk",
        &carrier,
        stream,
        &attn,
        sanic::plan::FoldSched {
            lane_axis: None,
            sgs: 16,
            lane_stream: false,
            chunk: 1,
        },
    );
    let reference = eval(&attn, &env);
    let Some(out) = run_coop_on_gpu("swk", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("split reduction over k=4096 on GPU: {}", out.trim());
}

// ── regression: fast-math tanh NaN'd on large inputs ─────────────────────────
// GPT-2's MLP activations exceed |x| ≈ 44, where Metal's fast tanh (via
// exp(2x)) returns inf/inf = NaN. The emitter must use the precise variant.
#[test]
fn tanh_survives_large_arguments_on_gpu() {
    let n = axis("n", 6);
    let x = Value::from_shape_fn(&[n.extent()], |c| {
        [-2000.0, -50.0, -1.0, 1.0, 50.0, 2000.0][c[0]]
    });
    let env: Env = [("X", x.clone())].into_iter().collect();

    let t = map(MapOp::Tanh, vec![input("X", [n], Dtype::F32)]);
    let kernel = sanic::emit_metal::emit_pointwise_metal("tanh_big", &t);
    let reference = eval(&t, &env);
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
    let (o, k) = (axis("o", 40), axis("k", 96));
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
            Value::from_shape_fn(&[o.extent(), k.extent()], |c| wvals[c[0] * 96 + c[1]]),
        ),
        ("x", rand_tensor(&[k], &mut rng)),
    ]
    .into_iter()
    .collect();

    let weight = input("W", [o, k], Dtype::BF16);
    let stream = source_axis(&weight, 1);
    let y = reduce(
        map(MapOp::Mul, vec![weight, input("x", [k], Dtype::F32)]),
        1usize,
        Monoid::Add,
    );
    let carrier = derive(&y, stream).unwrap();
    let kernel = emit_fused_metal("bf16mv", &carrier, stream, &y);
    assert!(
        kernel.msl.contains("device const ushort* b_W"),
        "bf16 buffer typed"
    );
    assert!(kernel.msl.contains("<< 16u"), "bf16 widen emitted");
    let reference = eval(&y, &env);

    let Some(dev) = MetalDevice::open() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let pipes = dev.compile(&kernel.msl);
    let pipe = pipes.get(&kernel.name);
    let inputs: Vec<MetalBuf> = kernel
        .inputs
        .iter()
        .map(|(n, _)| match *n {
            "W" => dev.from_bytes(&wbytes),
            _ => dev.from_f64(&env[n].data),
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
    let maxrel = max_rel_err(&got, &reference.data);
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
    let (n_out, n_g, n_r) = (16usize, 4usize, 128usize);
    let n_in = n_g * n_r;
    let (o, gq, r, c) = (
        axis("o", n_out),
        axis("g", n_g),
        axis("r", n_r),
        axis("c", n_in),
    );
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
            Value::from_shape_fn(&[o.extent(), gq.extent(), r.extent()], |cd| {
                q[cd[0] * n_in + cd[1] * n_r + cd[2]] as f64
            }),
        ),
        (
            "S",
            Value::from_shape_fn(&[o.extent(), gq.extent()], |cd| scales[cd[0] * n_g + cd[1]]),
        ),
        ("x", rand_tensor(&[gq, r], &mut rng)),
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
                    input("Wq", [o, gq, r], Dtype::I4),
                    input("x", [gq, r], Dtype::F32),
                ],
            ),
            unsqueeze(input("S", [o, gq], Dtype::F16), 2usize),
        ],
    );
    let flattened = flatten(prod, &[1usize, 2usize][..], c);
    let stream = source_axis(&flattened, 1);
    let y = reduce(flattened, 1usize, Monoid::Add);

    let carrier = derive(&y, stream).unwrap();
    let kernel = emit_fused_metal("w4mv", &carrier, stream, &y);
    assert!(kernel.msl.contains("device const uchar* b_Wq"));
    assert!(kernel.msl.contains("device const half* b_S"));
    let reference = eval(&y, &env);

    let Some(dev) = MetalDevice::open() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let pipes = dev.compile(&kernel.msl);
    let pipe = pipes.get(&kernel.name);
    let inputs: Vec<MetalBuf> = kernel
        .inputs
        .iter()
        .map(|(n, _)| match *n {
            "Wq" => dev.from_bytes(&packed),
            "S" => dev.from_bytes(&scale_f16),
            _ => dev.from_f64(&env[n].data),
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
    let maxrel = max_rel_err(&got, &reference.data);
    assert!(maxrel < 2e-3, "W4 matvec MISMATCH {maxrel:e}");
    eprintln!("W4A16 grouped matvec on GPU (packed int4 + f16 scales): GPU OK {maxrel:e}");
}

/// The generic per-rank Top-k compositions run through the ordinary
/// partitioner and Metal schedule, including planted exact ties where
/// first-max-wins is the contract.
#[test]
fn topk_compositions_run_on_gpu() {
    let n = axis("n", 129);
    let mut rng = Lcg(0xC0BE5);
    let mut vals: Vec<f64> = (0..129).map(|_| rng.f()).collect();
    // exact ties: a duplicated maximum and an interior duplicate
    vals[97] = vals[13]; // tie at some mid rank
    let m = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    vals[110] = m; // duplicate of the max, later position
    let env: Env = [("X", Value::from_shape_fn(&[n.extent()], |c| vals[c[0]]))]
        .into_iter()
        .collect();

    let x = input("X", [n], Dtype::F32);
    for (r, (v, i)) in topk(x, 0usize, 8).into_iter().enumerate() {
        for (tag, node) in [("val", v), ("idx", i)] {
            let name = format!("top8_{tag}_{r}");
            let schedule = partition(&node, &DeviceProfile::toy());
            let program = emit_schedule_metal(&schedule);
            let reference = eval(&node, &env);
            let Some(out) = run_schedule_on_gpu(&name, &program, &env, &reference) else {
                eprintln!("skipping: no Metal device");
                return;
            };
            eprintln!("{name} on GPU: {}", out.trim());
        }
    }
}

/// The generic all-ranks frontend composition runs through the ordinary
/// partitioner and Metal schedule, including planted exact ties.
#[test]
fn topk_all_composition_runs_on_gpu() {
    let (n, rk) = (axis("n", 129), axis("rk", 8));
    let mut rng = Lcg(0xC0BE5);
    let mut vals: Vec<f64> = (0..129).map(|_| rng.f()).collect();
    vals[97] = vals[13];
    let m = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    vals[110] = m;
    let env: Env = [("X", Value::from_shape_fn(&[n.extent()], |c| vals[c[0]]))]
        .into_iter()
        .collect();

    let x = input("X", [n], Dtype::F32);
    let all = topk_all(x, 0usize, 8, rk, true);
    let schedule = partition(&all, &DeviceProfile::toy());
    let program = emit_schedule_metal(&schedule);
    let reference = eval(&all, &env);
    let Some(out) = run_schedule_on_gpu("top8_all", &program, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("top8_all composition on GPU: {}", out.trim());
}

/// Graph execution: the same schedule captured into an indirect command
/// buffer and REPLAYED — twice, to prove the capture is stable — must match
/// the oracle. The schedule is a dependent chain (norm folds feeding GEMMs
/// feeding a flash fold), and the grid sizes are non-round, so this
/// exercises the hazard barriers and the nonuniform indirect dispatch.
#[test]
fn graph_replay_matches_oracle() {
    use sanic::metal::MetalGraph;
    let (s, dm, v) = (axis("s", 7), axis("dm", 19), axis("v", 53));
    let mut rng = Lcg(0x6EA9);
    let env: Env = [
        ("X", rand_tensor(&[s, dm], &mut rng)),
        ("G", rand_tensor(&[dm], &mut rng)),
        ("W", rand_tensor(&[v, dm], &mut rng)),
    ]
    .into_iter()
    .collect();

    // rmsnorm(X)·Wᵀ then a softmax-weighted reduction back over v — a chain
    // with real read-after-write hazards at every stage
    let x = input("X", [s, dm], Dtype::F32);
    let ms = reduce(
        map(MapOp::Mul, vec![x.clone(), x.clone()]),
        1usize,
        Monoid::Add,
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
        vec![
            map(MapOp::Mul, vec![x, input("G", [dm], Dtype::F32)]),
            unsqueeze(inv, 1usize),
        ],
    );
    let logits = matmul(
        xn,
        transpose(input("W", [v, dm], Dtype::F32), 0usize, 1usize),
    ); // [s, v]
    let out = reduce(logits, 1usize, Monoid::LogSumExp); // [s]

    let sched = partition(&out, &DeviceProfile::toy());
    let program = emit_schedule_metal(&sched);
    let reference = eval(&out, &env);

    let Some(dev) = MetalDevice::open() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let pipes = dev.compile(&program.msl);
    let mut bufs: HashMap<String, MetalBuf> = HashMap::new();
    for (n, _) in &program.inputs {
        bufs.insert(n.to_string(), dev.from_f64(&env[n].data));
    }
    for (n, size) in &program.buffers {
        bufs.insert(n.clone(), dev.alloc_f32(*size));
    }
    let graph: MetalGraph = dev.capture(&program_dispatches(&program, &bufs, &pipes));
    let output = &program
        .stages
        .last()
        .expect("Metal program has no stages")
        .output;
    for replay in 0..2 {
        dev.run_graph_timed(&graph).unwrap();
        let got = dev.read_f32(&bufs[output], reference.data.len());
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

#[test]
fn bindless_graph_replay_declares_indirect_resources() {
    use sanic::metal::MetalGraph;

    let elements = axis("elements", 257);
    let mut env = Env::new();
    let mut sum = konst(0.0);
    for input_index in 0..40 {
        let name: &'static str = Box::leak(format!("wide_input_{input_index}").into_boxed_str());
        env.insert(
            name,
            Value::from_shape_fn(&[elements.extent()], |index| {
                input_index as f64 * 0.01 + index[0] as f64 * 0.001
            }),
        );
        sum = map(MapOp::Add, vec![sum, input(name, [elements], Dtype::F32)]);
    }

    let schedule = partition(&sum, &DeviceProfile::toy());
    let program = emit_schedule_metal(&schedule);
    assert!(
        program.stages.iter().any(|stage| stage.argbuf.is_some()),
        "the regression must exercise an argument-buffer dispatch"
    );
    let reference = eval(&sum, &env);

    let Some(device) = MetalDevice::open() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let pipelines = device.compile(&program.msl);
    let mut buffers = HashMap::new();
    for (name, _) in &program.inputs {
        buffers.insert(name.to_string(), device.from_f64(&env[name].data));
    }
    for (name, size) in &program.buffers {
        buffers.insert(name.clone(), device.alloc_f32(*size));
    }
    let graph: MetalGraph = device.capture(&program_dispatches(&program, &buffers, &pipelines));
    let output = &program.stages.last().unwrap().output;
    for replay in 0..2 {
        device.run_graph_timed(&graph).unwrap();
        let got = device.read_f32(&buffers[output], reference.data.len());
        let maxrel = max_rel_err(&got, &reference.data);
        assert!(
            maxrel < 3e-3,
            "bindless graph mismatch on replay {replay}: {maxrel:e}"
        );
    }
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
) -> Option<String> {
    let dev = MetalDevice::open()?;
    let pipes = dev.compile(&kernel.msl);
    let pipe = pipes.get(&kernel.name);
    let inputs: Vec<MetalBuf> = kernel
        .inputs
        .iter()
        .map(|(n, _)| dev.from_f64(&env[n].data))
        .collect();
    // a cooperative kernel's grid_size is threads (TGs × TG width), not
    // output elements — read back the output volume
    let out_n = reference.data.len();
    let output = dev.alloc_f32(kernel.grid_size.max(out_n));
    dev.run(&[Dispatch {
        pipe,
        inputs,
        output: output.clone(),
        grid: kernel.grid_size,
        argbuf: None,
    }]);
    let got = dev.read_f32(&output, out_n);
    let maxrel = max_rel_err(&got, &reference.data);
    assert!(maxrel < 2e-3, "GPU MISMATCH {maxrel:e} ({label})");
    Some(format!("GPU OK {maxrel:e}"))
}

/// The coupled (m, ℓ, o) carrier with the stream split over lanes AND
/// simdgroups: the ExpShifted rescale merge runs through the shuffle
/// butterfly and then threadgroup rounds.
#[test]
fn coop_lane_stream_flash_matches_oracle() {
    use sanic::plan::FoldSched;
    let (sq, k, d, e) = (axis("sq", 4), axis("k", 256), axis("d", 8), axis("e", 8));
    let mut rng = Lcg(0xC007);
    let env: Env = [
        ("Q", rand_tensor(&[sq, d], &mut rng)),
        ("K", rand_tensor(&[k, d], &mut rng)),
        ("V", rand_tensor(&[k, e], &mut rng)),
    ]
    .into_iter()
    .collect();
    let key = input("K", [k, d], Dtype::F32);
    let stream = source_axis(&key, 0);
    let attn = attention(
        input("Q", [sq, d], Dtype::F32),
        key,
        input("V", [k, e], Dtype::F32),
    );
    let carrier = derive(&attn, stream).unwrap();
    let sched = FoldSched {
        lane_axis: None,
        sgs: 4,
        lane_stream: true, // split factor 128 ≤ 256
        chunk: 1,
    };
    let kernel = emit_fused_metal_sched("coop_ls_flash", &carrier, stream, &attn, sched);
    assert!(
        kernel.msl.contains("simd_shuffle_xor"),
        "lane merge emitted"
    );
    assert!(
        kernel.msl.contains("threadgroup_barrier"),
        "sg merge emitted"
    );
    let reference = eval(&attn, &env);
    let Some(out) = run_coop_on_gpu("coop_ls_flash", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!("lane-stream flash (sgs=4) on GPU: {}", out.trim());
}

/// A prefix-masked flash fold whose visible prefix is far shorter than the
/// split width: whole lanes fold only `-inf`-masked elements, so their
/// partials stay at the carrier identity (m = −∞, ℓ = o = 0) and reach the
/// rescale merge. `exp(−∞ − −∞)` is NaN unless the merge forces a −∞ side's
/// weight to zero — decode attention at any real context length hits this on
/// every prefill step.
#[test]
fn masked_flash_with_short_prefix_survives_lane_split() {
    use sanic::plan::FoldSched;
    let (sq, k, d, e) = (axis("sq", 4), axis("k", 256), axis("d", 8), axis("e", 8));
    let visible = 2usize;
    let mut rng = Lcg(0x3A5C);
    let env: Env = [
        ("Q", rand_tensor(&[sq, d], &mut rng)),
        ("K", rand_tensor(&[k, d], &mut rng)),
        ("V", rand_tensor(&[k, e], &mut rng)),
        (
            "mask",
            Value::from_shape_fn(&[k.extent()], |position| {
                if position[0] < visible {
                    0.0
                } else {
                    f64::NEG_INFINITY
                }
            }),
        ),
    ]
    .into_iter()
    .collect();
    let key = input("K", [k, d], Dtype::F32);
    let stream = source_axis(&key, 0);
    let scores = map(
        MapOp::Add,
        vec![
            matmul(
                input("Q", [sq, d], Dtype::F32),
                transpose(key, 0usize, 1usize),
            ),
            input("mask", [k], Dtype::F32),
        ],
    );
    let attn = matmul(softmax(scores, -1isize), input("V", [k, e], Dtype::F32));
    let carrier = derive(&attn, stream).unwrap();
    let sched = FoldSched {
        lane_axis: None,
        sgs: 4,
        lane_stream: true,
        chunk: 1,
    };
    let kernel = emit_fused_metal_sched("masked_ls_flash", &carrier, stream, &attn, sched);
    let reference = eval(&attn, &env);
    let Some(out) = run_coop_on_gpu("masked_ls_flash", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!(
        "short-prefix masked lane-stream flash on GPU: {}",
        out.trim()
    );
}

/// The value head dim distributed across lanes: the o slot vectorizes per
/// lane, m/ℓ stay once-per-simdgroup, the QKᵀ contraction lane-splits with
/// a monoid butterfly (score computed once per key per simdgroup), and the
/// sliced partials merge through threadgroup memory.
#[test]
fn coop_lane_axis_flash_matches_oracle() {
    use sanic::plan::FoldSched;
    let (sq, k, d, e) = (axis("sq", 5), axis("k", 48), axis("d", 32), axis("e", 32));
    let mut rng = Lcg(0x1A4E);
    let env: Env = [
        ("Q", rand_tensor(&[sq, d], &mut rng)),
        ("K", rand_tensor(&[k, d], &mut rng)),
        ("V", rand_tensor(&[k, e], &mut rng)),
    ]
    .into_iter()
    .collect();
    let key = input("K", [k, d], Dtype::F32);
    let stream = source_axis(&key, 0);
    let attn = attention(
        input("Q", [sq, d], Dtype::F32),
        key,
        input("V", [k, e], Dtype::F32),
    );
    let carrier = derive(&attn, stream).unwrap();
    let sched = FoldSched {
        lane_axis: Some(source_axis(&attn, 1)),
        sgs: 8,
        lane_stream: false,
        chunk: 1,
    };
    let kernel = emit_fused_metal_sched("coop_la_flash", &carrier, stream, &attn, sched);
    assert!(kernel.msl.contains("accs_"), "sliced slot emitted");
    assert!(
        kernel.msl.contains("simd_shuffle_xor"),
        "in-body contraction lane-split"
    );
    let reference = eval(&attn, &env);
    let Some(out) = run_coop_on_gpu("coop_la_flash", &kernel, &env, &reference) else {
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
    use sanic::plan::FoldSched;
    let (o, dm) = (axis("o", 64), axis("dm", 128));
    let mut rng = Lcg(0x2517);
    let env: Env = [
        ("x", rand_tensor(&[dm], &mut rng)),
        ("ln", rand_tensor(&[dm], &mut rng)),
        ("w", rand_tensor(&[o, dm], &mut rng)),
    ]
    .into_iter()
    .collect();
    let x = input("x", [dm], Dtype::F32);
    let stream = source_axis(&x, 0);
    let dot = reduce(
        map(
            MapOp::Mul,
            vec![
                map(MapOp::Mul, vec![x.clone(), input("ln", [dm], Dtype::F32)]),
                input("w", [o, dm], Dtype::F32),
            ],
        ),
        1usize,
        Monoid::Add,
    );
    let ms = map(
        MapOp::Mul,
        vec![
            reduce(map(MapOp::Mul, vec![x.clone(), x]), 0usize, Monoid::Add),
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
    let carrier = derive(&y, stream).unwrap();
    let reference = eval(&y, &env);
    for (label, sched) in [
        (
            "rows→lanes",
            FoldSched {
                lane_axis: Some(source_axis(&y, 0)),
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
        let kernel = emit_fused_metal_sched("coop_mv", &carrier, stream, &y, sched);
        let Some(out) = run_coop_on_gpu(label, &kernel, &env, &reference) else {
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
    use sanic::plan::FoldSched;
    let (n_out, n_g, n_r) = (8usize, 8usize, 128usize);
    let n_in = n_g * n_r; // 1024 = 32 lanes × 8-chunks × 4
    let (o, gq, r, c) = (
        axis("o", n_out),
        axis("g", n_g),
        axis("r", n_r),
        axis("c", n_in),
    );
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
            Value::from_shape_fn(&[o.extent(), gq.extent(), r.extent()], |cd| {
                q[cd[0] * n_in + cd[1] * n_r + cd[2]] as f64
            }),
        ),
        (
            "S",
            Value::from_shape_fn(&[o.extent(), gq.extent()], |cd| scales[cd[0] * n_g + cd[1]]),
        ),
        ("x", rand_tensor(&[gq, r], &mut rng)),
    ]
    .into_iter()
    .collect();

    let prod = map(
        MapOp::Mul,
        vec![
            map(
                MapOp::Mul,
                vec![
                    input("Wq", [o, gq, r], Dtype::I4),
                    input("x", [gq, r], Dtype::F32),
                ],
            ),
            unsqueeze(input("S", [o, gq], Dtype::F32), 2usize),
        ],
    );
    let flattened = flatten(prod, &[1usize, 2usize][..], c);
    let stream = source_axis(&flattened, 1);
    let y = reduce(flattened, 1usize, Monoid::Add);
    let carrier = derive(&y, stream).unwrap();
    let sched = FoldSched {
        lane_axis: None,
        sgs: 1,
        lane_stream: true,
        chunk: 8,
    };
    let kernel = emit_fused_metal_sched("coop_w4c", &carrier, stream, &y, sched);
    assert!(
        kernel.msl.contains("c_ * 8u"),
        "chunked stream loop emitted"
    );
    assert!(
        kernel.msl.contains("simd_shuffle_xor"),
        "lane merge emitted"
    );
    let reference = eval(&y, &env);

    let Some(dev) = MetalDevice::open() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let pipes = dev.compile(&kernel.msl);
    let pipe = pipes.get(&kernel.name);
    let inputs: Vec<MetalBuf> = kernel
        .inputs
        .iter()
        .map(|(n, _)| match *n {
            "Wq" => dev.from_bytes(&packed),
            _ => dev.from_f64(&env[n].data),
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
    let maxrel = max_rel_err(&got, &reference.data);
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
    use sanic::plan::FoldSched;
    let (t, d, e) = (axis("t", 256), axis("d", 8), axis("e", 8));
    let mut rng = Lcg(0x90E57);
    let base_env: Env = [
        ("q", rand_tensor(&[d], &mut rng)),
        ("K", rand_tensor(&[t, d], &mut rng)),
        ("V", rand_tensor(&[t, e], &mut rng)),
    ]
    .into_iter()
    .collect();

    // decode attention: scores + where(pos < iota(t), -1e30, 0), softmax·V
    let key = input("K", [t, d], Dtype::F32);
    let stream = source_axis(&key, 0);
    let scores = reduce(
        map(MapOp::Mul, vec![key, input("q", [d], Dtype::F32)]),
        1usize,
        Monoid::Add,
    );
    let position = coordinate(scores.clone(), 0usize);
    let future = map(MapOp::Lt, vec![input("pos", [], Dtype::F32), position]);
    let masked = map(
        MapOp::Add,
        vec![
            scores,
            map(MapOp::Where, vec![future, konst(-1e30), konst(0.0)]),
        ],
    );
    let value = input("V", [t, e], Dtype::F32);
    let attn = reduce(
        map(
            MapOp::Mul,
            vec![unsqueeze(softmax(masked, 0usize), 1usize), value],
        ),
        0usize,
        Monoid::Add,
    );
    let carrier = derive(&attn, stream).unwrap();

    for pos in [0usize, 3, 40, 255] {
        let mut env = base_env.clone();
        env.insert("pos", Value::scalar(pos as f64));
        let reference = eval(&attn, &env);

        let kernel = emit_fused_metal("hw_flash", &carrier, stream, &attn);
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
        let kc = emit_fused_metal_sched("hw_flash_coop", &carrier, stream, &attn, sched);
        assert!(kc.msl.contains("max((uint)("), "lane-clamped bound emitted");
        let Some(out) = run_coop_on_gpu("hw_flash_coop", &kc, &env, &reference) else {
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
    use sanic::plan::FoldSched;
    let (b, n) = (axis("b", 4), axis("n", 64));
    let mut t = rand_tensor(&[b, n], &mut Lcg(0x715));
    // plant an exact tie in every row: positions 7 and 33 share the max
    for row in 0..4 {
        t.data[row * 64 + 7] = 9.0;
        t.data[row * 64 + 33] = 9.0;
    }
    let env: Env = [("x", t)].into_iter().collect();
    let x = input("x", [b, n], Dtype::F32);
    let stream = source_axis(&x, 1);
    let am = argmax(x, 1usize);
    let carrier = derive(&am, stream).unwrap();
    let sched = FoldSched {
        lane_axis: None,
        sgs: 4,
        lane_stream: true,
        chunk: 1,
    };
    let kernel = emit_fused_metal_sched("coop_am", &carrier, stream, &am, sched);
    assert!(
        !kernel.msl.contains("simd_shuffle_xor"),
        "order-sensitive carrier must fall back to the scalar schedule"
    );
    let reference = eval(&am, &env);
    let Some(out) = run_coop_on_gpu("argmax-decline", &kernel, &env, &reference) else {
        eprintln!("skipping: no Metal device");
        return;
    };
    eprintln!(
        "argmax declined to scalar, ties first-max-wins: {}",
        out.trim()
    );
}

/// A monoidal prefix scan (cumsum) EMITS now: each output point folds its
/// own prefix through the scalar fallback path the Metal emitter supports.
/// Max-scan too (running maximum), against the oracle.
#[test]
fn monoidal_prefix_scans_run_on_gpu() {
    let (s, t) = (axis("s", 3), axis("t", 17));
    let mut rng = Lcg(0x5CA9);
    let env: Env = [("X", rand_tensor(&[s, t], &mut rng))]
        .into_iter()
        .collect();
    for (label, m) in [("cumsum", Monoid::Add), ("cummax", Monoid::Max)] {
        let node = scan(input("X", [s, t], Dtype::F32), 1usize, m);
        let kernel = sanic::emit_metal::emit_pointwise_metal(label, &node);
        let reference = eval(&node, &env);
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
    for (i, &value) in got.iter().enumerate() {
        assert_eq!(value, (i as f32) * 1.5, "offset-bound reads");
    }
    // the zero-copy property itself: mutate through the HOST pointer, the
    // device sees it without any re-upload
    unsafe { (base as *mut f32).write(100.0) };
    let got = run(&dev);
    assert_eq!(got[0], 100.0, "host write visible to the GPU: same memory");
    eprintln!("zero-copy wrap + offset binding on GPU: OK");
}
