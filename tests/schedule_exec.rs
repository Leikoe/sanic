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
use sanic::ir::*;
use sanic::nn::scaled_dot_product_attention;
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
    let shape = axes.iter().map(|axis| axis.extent()).collect::<Vec<_>>();
    Value::from_shape_fn(&shape, |_| rng.f())
}

fn assert_close(x: &Value, y: &Value) {
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

fn add_r() -> Monoid {
    Monoid::Add
}

fn rmsnorm(x: NodeRef, g: NodeRef, n: f64) -> NodeRef {
    let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), 1usize, add_r());
    let mean = map(MapOp::Mul, vec![ss, konst(1.0 / n)]);
    let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
    map(
        MapOp::Div,
        vec![map(MapOp::Mul, vec![x, g]), unsqueeze(denom, 1usize)],
    )
}

fn head_projection(x: NodeRef, weight: NodeRef) -> NodeRef {
    let x = unsqueeze(unsqueeze(x, 0usize), 2usize);
    let weight = unsqueeze(weight, 1usize);
    reduce(map(MapOp::Mul, vec![x, weight]), 3usize, add_r())
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

    let x = input("X", [s, dm], Dtype::F32);
    let xk = rename(x.clone(), 0usize, t);
    let q = matmul(
        x.clone(),
        transpose(input("Wq", [dk, dm], Dtype::F32), 0usize, 1usize),
    ); // [s, dk]
    let k = matmul(
        xk.clone(),
        transpose(input("Wk", [dk, dm], Dtype::F32), 0usize, 1usize),
    ); // [t, dk]
    let v = matmul(
        xk,
        transpose(input("Wv", [dv, dm], Dtype::F32), 0usize, 1usize),
    ); // [t, dv]
    let attn = scaled_dot_product_attention(q, k, v, None, 0.0, false, Some(1.0), false);
    let o = matmul(
        attn,
        transpose(input("Wo", [dm, dv], Dtype::F32), 0usize, 1usize),
    ); // [s, dm]
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

    let x = input("X", [s, dm], Dtype::F32);
    let xn = rmsnorm(x.clone(), input("g1", [dm], Dtype::F32), n);
    let xn_kv = rename(xn.clone(), 0usize, t);

    let q = head_projection(xn, input("Wq", [h, dk, dm], Dtype::F32)); // [h, s, dk]
    let k = head_projection(xn_kv.clone(), input("Wk", [h, dk, dm], Dtype::F32)); // [h, t, dk]
    let vv = head_projection(xn_kv, input("Wv", [h, dv, dm], Dtype::F32)); // [h, t, dv]

    let attn = scaled_dot_product_attention(
        q,
        k,
        vv,
        None,
        0.0,
        true,
        Some(1.0 / (dk.extent() as f64).sqrt()),
        false,
    ); // [h, s, dv]

    let flat = flatten(transpose(attn, 0usize, 1usize), &[1usize, 2usize][..], dmv); // [s, dmv]
    let o = matmul(flat, input("Wo", [dmv, dm], Dtype::F32)); // [s, dm]
    let res1 = map(MapOp::Add, vec![o, x]);

    let hn = rmsnorm(res1.clone(), input("g2", [dm], Dtype::F32), n);
    let gate = matmul(
        hn.clone(),
        transpose(input("Wg", [f, dm], Dtype::F32), 0usize, 1usize),
    ); // [s, f]
    let up = matmul(
        hn,
        transpose(input("Wu", [f, dm], Dtype::F32), 0usize, 1usize),
    ); // [s, f]
    let act = map(MapOp::Mul, vec![silu(gate), up]);
    let mlp = matmul(act, input("Wd", [f, dm], Dtype::F32));
    let yb = map(MapOp::Add, vec![mlp, res1]);

    let logits = matmul(
        yb,
        transpose(input("W_lm", [v, dm], Dtype::F32), 0usize, 1usize),
    ); // [s, v]

    let sched = partition(&logits, &DeviceProfile::toy());
    assert!(
        sched.kernel_count() >= 10,
        "expected a multi-kernel schedule"
    );
    let executed = sched.execute(&env);
    let reference = eval(&logits, &env);
    assert_close(&executed, &reference);

    // The decline census (CRITIQUE.md §2.2): the declines THIS workload hit,
    // not the syllabus's. A reduction hidden behind the nearest reduction
    // frontier is a producer for a later stage, not a candidate for this one;
    // this transformer therefore reaches no genuine derivation declines.
    let census = sched.decline_census();
    println!("{census}");
    let mut buckets = std::collections::BTreeMap::new();
    for decline in &sched.declines {
        *buckets.entry(decline.rule).or_insert(0) += 1;
    }
    assert!(
        buckets.is_empty(),
        "a transformer decline bucket appeared; inspect the census above"
    );
}
