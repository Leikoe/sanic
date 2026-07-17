//! The compiler-correctness theorem, as a runnable check.
//!
//! `partition` splits a whole graph into kernels; `Schedule::execute` runs
//! them — fused stages stream their derived carriers, elementwise/gather
//! stages evaluate their spliced sub-graphs, intermediates flow between them
//! by name. This file asserts the composition equals the naive reference:
//!
//!     partition(g).execute(inputs) == eval(g, inputs)
//!
//! i.e. the derivation, the fusion cuts, and the whole-graph dataflow all
//! preserve the semantics — on a real multi-head transformer block, not a
//! single kernel in isolation.

use sanic::cost::Device;
use sanic::interp::{Env, Value, eval};
use sanic::ir::*;
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

fn rand_tensor(axes: &[Axis], rng: &mut Lcg) -> Value {
    Value::from_fn(axes, |_| rng.f())
}

fn assert_close(x: &Value, y: &Value) {
    let y = y.permuted_to(&x.axes);
    assert_eq!(x.shape, y.shape, "shape: {:?} vs {:?}", x.axes, y.axes);
    let mut worst = 0.0f64;
    for (a, b) in x.data.iter().zip(&y.data) {
        worst = worst.max((a - b).abs() / (1.0 + a.abs().max(b.abs())));
    }
    assert!(worst < 1e-9, "max relative error {worst:e}");
}

fn add_r() -> BinOp {
    BinOp::Monoid(Monoid::Add)
}

fn rmsnorm(x: Node, g: Node, n: f64, ax: Axis) -> Node {
    let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), ax, add_r());
    let mean = map(MapOp::Mul, vec![ss, konst(1.0 / n)]);
    let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
    map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom])
}

// ── a focused case: flash attention + residual epilogue + output GEMM ────────
#[test]
fn attention_block_schedule_executes_to_reference() {
    let (s, t, dm, dk, dv) = (
        axis("s", 4),
        axis("t", 4),
        axis("dm", 6),
        axis("dk", 5),
        axis("dv", 6),
    );
    let mut rng = Lcg(0xBEEF);
    let env: Env = [
        ("X", rand_tensor(&[s, dm], &mut rng)),
        ("Wq", rand_tensor(&[dk, dm], &mut rng)),
        ("Wk", rand_tensor(&[dk, dm], &mut rng)),
        ("Wv", rand_tensor(&[dv, dm], &mut rng)),
        ("Wo", rand_tensor(&[dm, dv], &mut rng)),
    ]
    .into_iter()
    .collect();

    let x = input("X", &[s, dm]);
    let xk = rename(x.clone(), s, t);
    let q = matmul(x.clone(), input("Wq", &[dk, dm]), dm); // [s, dk]
    let k = matmul(xk.clone(), input("Wk", &[dk, dm]), dm); // [t, dk]
    let v = matmul(xk, input("Wv", &[dv, dm]), dm); // [t, dv]
    let attn = attention(q, k, v, dk, t); // [s, dv]
    let o = matmul(attn, input("Wo", &[dm, dv]), dv); // [s, dm]
    let y = map(MapOp::Add, vec![o, x]); // residual

    let sched = partition(&y, &Device::toy());
    let executed = sched.execute(&env);
    let reference = eval(&y, &env);
    assert_close(&executed, &reference);
}

// ── the full transformer block + logits head (llm.rs, small extents) ─────────
// Every stage type at once: RMSNorm-fused projections, the multi-head flash
// core, a head flatten, residual epilogues, SwiGLU-fused down GEMM, and the
// logits GEMM. Executing the whole schedule must reproduce the naive graph.
#[test]
fn full_transformer_block_schedule_executes_to_reference() {
    let (v, s, t, dm, h, dk, dv, dmv, f) = (
        axis("v", 16),
        axis("s", 4),
        axis("t", 4),
        axis("dm", 8),
        axis("h", 2),
        axis("dk", 4),
        axis("dv", 4),
        axis("dmv", 8), // h · dv
        axis("f", 12),
    );
    let n = dm.extent as f64;

    let mut rng = Lcg(0x5A5A_1234);
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

    let x = input("X", &[s, dm]);
    let xn = rmsnorm(x.clone(), input("g1", &[dm]), n, dm);
    let xn_kv = rename(xn.clone(), s, t);

    let q = matmul(xn, input("Wq", &[h, dk, dm]), dm); // [s, h, dk]
    let k = matmul(xn_kv.clone(), input("Wk", &[h, dk, dm]), dm); // [t, h, dk]
    let vv = matmul(xn_kv, input("Wv", &[h, dv, dm]), dm); // [t, h, dv]

    let scores = matmul(q, k, dk); // [s, h, t]
    let scaled = map(
        MapOp::Mul,
        vec![scores, konst(1.0 / (dk.extent as f64).sqrt())],
    );
    let masked = map(MapOp::Add, vec![scaled, causal_mask(s, t)]);
    let attn = matmul(softmax(masked, t), vv, t); // [s, h, dv]

    let flat = flatten(attn, &[h, dv], dmv); // [s, dmv]
    let o = matmul(flat, input("Wo", &[dmv, dm]), dmv); // [s, dm]
    let res1 = map(MapOp::Add, vec![o, x]);

    let hn = rmsnorm(res1.clone(), input("g2", &[dm]), n, dm);
    let gate = matmul(hn.clone(), input("Wg", &[f, dm]), dm); // [s, f]
    let up = matmul(hn, input("Wu", &[f, dm]), dm); // [s, f]
    let act = map(MapOp::Mul, vec![silu(gate), up]);
    let mlp = reduce(
        map(MapOp::Mul, vec![act, input("Wd", &[f, dm])]),
        f,
        add_r(),
    );
    let yb = map(MapOp::Add, vec![mlp, res1]);

    let logits = matmul(yb, input("W_lm", &[v, dm]), dm); // [s, v]

    let sched = partition(&logits, &Device::toy());
    assert!(
        sched.kernel_count() >= 10,
        "expected a multi-kernel schedule"
    );
    let executed = sched.execute(&env);
    let reference = eval(&logits, &env);
    assert_close(&executed, &reference);
}
