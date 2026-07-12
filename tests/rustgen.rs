//! The code-generating backend, verified the honest way: emit a schedule as
//! Rust, **compile it with `rustc`, run the binary**, and compare to the
//! interpreter oracle. Passing means the generated machine code — not an
//! interpretation of it — computes the derived kernels correctly.

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use sanic::cost::Device;
use sanic::interp::{Env, Extents, Tensor, eval};
use sanic::ir::*;
use sanic::partition::partition;
use sanic::rustgen::{Program, emit_schedule};

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
fn rand_tensor(axes: &[Axis], ext: &Extents, rng: &mut Lcg) -> Tensor {
    Tensor::from_fn(axes, ext, |_| rng.f())
}
fn as_f64(ext: &Extents) -> HashMap<Axis, f64> {
    ext.iter().map(|(&a, &n)| (a, n as f64)).collect()
}
fn add_r() -> BinOp {
    BinOp::Monoid(Monoid::Add)
}

fn bake(data: &[f64]) -> String {
    data.iter()
        .map(|v| format!("{v:?}f64"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Append a self-checking `main` (baked inputs + interpreter-computed
/// expected), compile with `rustc`, run, and assert the binary reports a match.
fn compile_and_verify(label: &str, program: &Program, env: &Env, reference: &Tensor) {
    // input buffers, each in its declared-axis layout
    let mut main = String::from("\nfn main() {\n");
    let mut call_args = Vec::new();
    for (name, axes) in &program.inputs {
        let t = env
            .get(name)
            .unwrap_or_else(|| panic!("missing input {name}"))
            .permuted_to(axes);
        main.push_str(&format!(
            "    let b_{}: Vec<f64> = vec![{}];\n",
            sanitize(name),
            bake(&t.data)
        ));
        call_args.push(format!("&b_{}[..]", sanitize(name)));
    }
    let expected = reference.permuted_to(&program.output_axes);
    main.push_str(&format!(
        "    let expected: Vec<f64> = vec![{}];\n",
        bake(&expected.data)
    ));
    main.push_str(&format!("    let got = run({});\n", call_args.join(", ")));
    main.push_str(
        "    assert_eq!(got.len(), expected.len(), \"length mismatch\");\n\
         \x20   let mut maxe = 0.0f64;\n\
         \x20   for (a, b) in got.iter().zip(&expected) { maxe = maxe.max((a - b).abs()); }\n\
         \x20   if maxe < 1e-9 { println!(\"OK {maxe:e}\"); } else { eprintln!(\"MISMATCH {maxe:e}\"); std::process::exit(1); }\n\
         }\n",
    );

    let dir: PathBuf = std::env::var("CLAUDE_JOB_DIR")
        .map(|d| PathBuf::from(d).join("tmp"))
        .unwrap_or_else(|_| std::env::temp_dir());
    std::fs::create_dir_all(&dir).unwrap();
    let src = dir.join(format!("sanic_gen_{label}.rs"));
    let bin = dir.join(format!("sanic_gen_{label}"));
    std::fs::write(&src, format!("{}{}", program.source, main)).unwrap();

    let out = Command::new("rustc")
        .args(["-O", "--edition", "2021", "-o"])
        .arg(&bin)
        .arg(&src)
        .output()
        .expect("failed to invoke rustc");
    assert!(
        out.status.success(),
        "generated program failed to COMPILE:\n{}\n--- source at {} ---",
        String::from_utf8_lossy(&out.stderr),
        src.display()
    );

    let run = Command::new(&bin)
        .output()
        .expect("failed to run generated binary");
    assert!(
        run.status.success(),
        "generated program RAN but diverged:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr),
    );
    assert!(
        String::from_utf8_lossy(&run.stdout).contains("OK"),
        "unexpected output: {}",
        String::from_utf8_lossy(&run.stdout)
    );
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

// ── the derived FlashAttention kernel, compiled and run ──────────────────────
#[test]
fn flash_attention_compiles_and_matches() {
    let (sq, k, d, e) = (axis("sq"), axis("k"), axis("d"), axis("e"));
    let ext: Extents = [(sq, 5), (k, 7), (d, 4), (e, 6)].into_iter().collect();
    let mut rng = Lcg(0xF1A54);
    let env: Env = [
        ("Q", rand_tensor(&[sq, d], &ext, &mut rng)),
        ("K", rand_tensor(&[k, d], &ext, &mut rng)),
        ("V", rand_tensor(&[k, e], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let attn = attention(
        input("Q", &[sq, d]),
        input("K", &[k, d]),
        input("V", &[k, e]),
        d,
        k,
    );
    let sched = partition(&attn, &Device::toy(), &as_f64(&ext));
    let program = emit_schedule(&sched, &ext);
    let reference = eval(&attn, &env, &ext);
    compile_and_verify("flash", &program, &env, &reference);
}

// ── quantized matmul: dequant-fused GEMM, compiled and run ───────────────────
#[test]
fn quantized_matmul_compiles_and_matches() {
    let (s, dm, o) = (axis("s"), axis("dm"), axis("o"));
    let ext: Extents = [(s, 4), (dm, 16), (o, 8)].into_iter().collect();
    let mut rng = Lcg(0x9114A7);
    let qw = Tensor::from_fn(&[o, dm], &ext, |_| (rng.f() * 8.0).round());
    let env: Env = [
        ("X", rand_tensor(&[s, dm], &ext, &mut rng)),
        ("qW", qw),
        (
            "scale",
            Tensor::from_fn(&[o], &ext, |_| 0.05 * (rng.f() + 1.5)),
        ),
    ]
    .into_iter()
    .collect();

    let dw = map(
        MapOp::Mul,
        vec![input("qW", &[o, dm]), input("scale", &[o])],
    );
    let y = matmul(input("X", &[s, dm]), dw, dm);
    let sched = partition(&y, &Device::toy(), &as_f64(&ext));
    let program = emit_schedule(&sched, &ext);
    let reference = eval(&y, &env, &ext);
    compile_and_verify("quant", &program, &env, &reference);
}

// ── conv2d (window + flatten + matmul), compiled and run ─────────────────────
// The movement vocabulary through the compiled backend: the windowed reads
// become signed index arithmetic in the generated Rust, the padded reads a
// clamp + select. One implicit-GEMM kernel, no im2col buffer anywhere.
#[test]
fn conv2d_compiles_and_matches() {
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
        (ci, 2),
        (h0, 7),
        (w0, 8),
        (oh, 5),
        (ow, 6),
        (kh, 3),
        (kw, 3),
        (r, 18),
        (co, 4),
    ]
    .into_iter()
    .collect();
    let mut rng = Lcg(0x2DC0);
    let env: Env = [
        ("X", rand_tensor(&[ci, h0, w0], &ext, &mut rng)),
        ("W", rand_tensor(&[co, ci, kh, kw], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let xw = reindex(
        input("X", &[ci, h0, w0]),
        vec![
            (h0, vec![(1, oh), (1, kh)], 0),
            (w0, vec![(1, ow), (1, kw)], 0),
        ],
        false,
    );
    let xf = flatten(xw, &[ci, kh, kw], r);
    let wf = flatten(input("W", &[co, ci, kh, kw]), &[ci, kh, kw], r);
    let conv = matmul(xf, wf, r);

    let sched = partition(&conv, &Device::toy(), &as_f64(&ext));
    let program = emit_schedule(&sched, &ext);
    let reference = eval(&conv, &env, &ext);
    compile_and_verify("conv2d", &program, &env, &reference);
}

// ── sliding-window attention (padded windowed K/V), compiled and run ─────────
#[test]
fn sliding_window_attention_compiles_and_matches() {
    let (s, t, j, d, e) = (axis("s"), axis("t"), axis("j"), axis("d"), axis("e"));
    let (ns, w) = (9usize, 4usize);
    let ext: Extents = [(s, ns), (t, ns), (j, w), (d, 5), (e, 4)]
        .into_iter()
        .collect();
    let mut rng = Lcg(0x51DE);
    let env: Env = [
        ("Q", rand_tensor(&[s, d], &ext, &mut rng)),
        ("K", rand_tensor(&[t, d], &ext, &mut rng)),
        ("V", rand_tensor(&[t, e], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let off = -((w - 1) as i64);
    let kw = reindex(
        input("K", &[t, d]),
        vec![(t, vec![(1, s), (1, j)], off)],
        true,
    );
    let vw = reindex(
        input("V", &[t, e]),
        vec![(t, vec![(1, s), (1, j)], off)],
        true,
    );
    let scores = matmul(input("Q", &[s, d]), kw, d);
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

    let sched = partition(&attn, &Device::toy(), &as_f64(&ext));
    let program = emit_schedule(&sched, &ext);
    let reference = eval(&attn, &env, &ext);
    compile_and_verify("swa", &program, &env, &reference);
}

// ── the full transformer block, compiled and run ─────────────────────────────
#[test]
fn transformer_block_compiles_and_matches() {
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
    let mut rng = Lcg(0x7A11);
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

    let rmsnorm = |x: Node, g: Node, ax: Axis| {
        let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), ax, add_r());
        let mean = map(MapOp::Mul, vec![ss, konst(1.0 / n)]);
        let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
        map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom])
    };

    let x = input("X", &[s, dm]);
    let xn = rmsnorm(x.clone(), input("g1", &[dm]), dm);
    let xn_kv = rename(xn.clone(), s, t);
    let q = matmul(xn, input("Wq", &[h, dk, dm]), dm);
    let k = matmul(xn_kv.clone(), input("Wk", &[h, dk, dm]), dm);
    let vv = matmul(xn_kv, input("Wv", &[h, dv, dm]), dm);
    let scores = matmul(q, k, dk);
    let scaled = map(
        MapOp::Mul,
        vec![scores, konst(1.0 / (ext[&dk] as f64).sqrt())],
    );
    let masked = map(MapOp::Add, vec![scaled, causal_mask(s, t)]);
    let attn = matmul(softmax(masked, t), vv, t);
    let flat = flatten(attn, &[h, dv], dmv);
    let o = matmul(flat, input("Wo", &[dmv, dm]), dmv);
    let res1 = map(MapOp::Add, vec![o, x]);
    let hn = rmsnorm(res1.clone(), input("g2", &[dm]), dm);
    let gate = matmul(hn.clone(), input("Wg", &[f, dm]), dm);
    let up = matmul(hn, input("Wu", &[f, dm]), dm);
    let act = map(MapOp::Mul, vec![silu(gate), up]);
    let mlp = reduce(
        map(MapOp::Mul, vec![act, input("Wd", &[f, dm])]),
        f,
        add_r(),
    );
    let yb = map(MapOp::Add, vec![mlp, res1]);
    let logits = matmul(yb, input("W_lm", &[v, dm]), dm);

    let sched = partition(&logits, &Device::toy(), &as_f64(&ext));
    let program = emit_schedule(&sched, &ext);
    let reference = eval(&logits, &env, &ext);
    compile_and_verify("block", &program, &env, &reference);
}

// ── a BACKWARD pass, compiled and run: training code is just another graph ───
#[test]
fn attention_gradient_compiles_and_matches() {
    let (s, t, dk, dv) = (axis("s"), axis("t"), axis("dk"), axis("dv"));
    let ext: Extents = [(s, 4), (t, 4), (dk, 3), (dv, 3)].into_iter().collect();
    let mut rng = Lcg(0x6ADC);
    let env: Env = [
        ("Q", rand_tensor(&[s, dk], &ext, &mut rng)),
        ("K", rand_tensor(&[t, dk], &ext, &mut rng)),
        ("V", rand_tensor(&[t, dv], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let scores = matmul(input("Q", &[s, dk]), input("K", &[t, dk]), dk);
    let masked = map(MapOp::Add, vec![scores, causal_mask(s, t)]);
    let out = matmul(softmax(masked, t), input("V", &[t, dv]), t);
    let sq = map(MapOp::Mul, vec![out.clone(), out]);
    let loss = reduce(reduce(sq, s, add_r()), dv, add_r());

    let grads = sanic::grad::grad(&loss, &["Q"], &ext);
    let g = &grads["Q"];
    let sched = partition(g, &Device::toy(), &as_f64(&ext));
    let program = emit_schedule(&sched, &ext);
    let reference = eval(g, &env, &ext);
    compile_and_verify("dq", &program, &env, &reference);
}
