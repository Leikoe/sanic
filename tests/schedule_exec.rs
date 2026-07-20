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

use sanic::cost::DeviceProfile;
use sanic::interp::{Env, Value, eval};
use sanic::kernel_ir::*;
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
    // the one tolerance policy at this file's deepest fold chain
    assert!(
        worst < sanic::verify::rel_tolerance(Dtype::F64, 64),
        "max relative error {worst:e}"
    );
}

fn add_r() -> BinOp {
    BinOp::Monoid(Monoid::Add)
}

fn rmsnorm(x: NodeRef, g: NodeRef, n: f64, ax: Axis) -> NodeRef {
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

    let x = input("X", &[s, dm], Dtype::F32);
    let xk = rename(x.clone(), s, t);
    let q = matmul(x.clone(), input("Wq", &[dk, dm], Dtype::F32), dm); // [s, dk]
    let k = matmul(xk.clone(), input("Wk", &[dk, dm], Dtype::F32), dm); // [t, dk]
    let v = matmul(xk, input("Wv", &[dv, dm], Dtype::F32), dm); // [t, dv]
    let attn = attention(q, k, v, dk, t); // [s, dv]
    let o = matmul(attn, input("Wo", &[dm, dv], Dtype::F32), dv); // [s, dm]
    let y = map(MapOp::Add, vec![o, x]); // residual

    let sched = partition(&y, &DeviceProfile::toy());
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
    let n = dm.extent() as f64;

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

    let x = input("X", &[s, dm], Dtype::F32);
    let xn = rmsnorm(x.clone(), input("g1", &[dm], Dtype::F32), n, dm);
    let xn_kv = rename(xn.clone(), s, t);

    let q = matmul(xn, input("Wq", &[h, dk, dm], Dtype::F32), dm); // [s, h, dk]
    let k = matmul(xn_kv.clone(), input("Wk", &[h, dk, dm], Dtype::F32), dm); // [t, h, dk]
    let vv = matmul(xn_kv, input("Wv", &[h, dv, dm], Dtype::F32), dm); // [t, h, dv]

    let scores = matmul(q, k, dk); // [s, h, t]
    let scaled = map(
        MapOp::Mul,
        vec![scores, konst(1.0 / (dk.extent() as f64).sqrt())],
    );
    let masked = map(MapOp::Add, vec![scaled, causal_mask(s, t)]);
    let attn = matmul(softmax(masked, t), vv, t); // [s, h, dv]

    let flat = flatten(attn, &[h, dv], dmv); // [s, dmv]
    let o = matmul(flat, input("Wo", &[dmv, dm], Dtype::F32), dmv); // [s, dm]
    let res1 = map(MapOp::Add, vec![o, x]);

    let hn = rmsnorm(res1.clone(), input("g2", &[dm], Dtype::F32), n, dm);
    let gate = matmul(hn.clone(), input("Wg", &[f, dm], Dtype::F32), dm); // [s, f]
    let up = matmul(hn, input("Wu", &[f, dm], Dtype::F32), dm); // [s, f]
    let act = map(MapOp::Mul, vec![silu(gate), up]);
    let mlp = reduce(
        map(MapOp::Mul, vec![act, input("Wd", &[f, dm], Dtype::F32)]),
        f,
        add_r(),
    );
    let yb = map(MapOp::Add, vec![mlp, res1]);

    let logits = matmul(yb, input("W_lm", &[v, dm], Dtype::F32), dm); // [s, v]

    let sched = partition(&logits, &DeviceProfile::toy());
    assert!(
        sched.kernel_count() >= 10,
        "expected a multi-kernel schedule"
    );
    let executed = sched.execute(&env);
    let reference = eval(&logits, &env);
    assert_close(&executed, &reference);

    // The decline census (CRITIQUE.md §2.2): the declines THIS workload hit,
    // not the syllabus's. Printed for inspection; the assertions pin that
    // every bucket is in the deriver's stable rule vocabulary, so a new kind
    // of decline showing up on a transformer is a visible event, not noise.
    let census = sched.decline_census();
    println!("{census}");
    let mut buckets = std::collections::BTreeMap::new();
    for decline in &sched.declines {
        *buckets.entry(decline.rule).or_insert(0) += 1;
    }
    assert_eq!(
        buckets,
        [
            ("binop-of-coupled", 1),
            ("not-streamed", 32),
            ("still-per-element", 2),
        ]
        .into_iter()
        .collect(),
        "a transformer decline bucket changed; inspect the census above"
    );
}
