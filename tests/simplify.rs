//! The algebraic simplifier, as a training-time graph pass: value-preserving,
//! and it derives a composed logsumexp's BACKWARD to the same kernels a
//! hand-written `softmax − onehot` rule would — no LSE-specific gradient rule,
//! no stop-gradient. The winner-mask of the stabilizing max-shift cancels, and
//! cross-root CSE lets the backward reuse the forward's logsumexp carrier.

use sanic::cost::Device;
use sanic::grad::grad;
use sanic::interp::{Env, Value, eval};
use sanic::ir::*;
use sanic::partition::partition_many;
use sanic::simplify::simplify_many;

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

/// Softmax cross-entropy over `[b, c]`, `logsumexp − logit[y]`. `primitive`
/// uses the `LogSumExp` monoid; otherwise the composition `m + log(Σ exp(x−m))`.
fn cross_entropy(logits: &Node, b: Axis, c: Axis, primitive: bool) -> Node {
    let lse = if primitive {
        reduce(logits.clone(), c, BinOp::Monoid(Monoid::LogSumExp))
    } else {
        let m = reduce(logits.clone(), c, BinOp::Monoid(Monoid::Max));
        let sh = map(MapOp::Exp, vec![map(MapOp::Sub, vec![logits.clone(), m.clone()])]);
        map(
            MapOp::Add,
            vec![m, map(MapOp::Log, vec![reduce(sh, c, BinOp::Monoid(Monoid::Add))])],
        )
    };
    let picked = reduce(
        map(MapOp::Mul, vec![logits.clone(), one_hot(c, input("y", &[b]))]),
        c,
        BinOp::Monoid(Monoid::Add),
    );
    reduce(map(MapOp::Sub, vec![lse, picked]), b, BinOp::Monoid(Monoid::Add))
}

/// The simplifier is a value-preserving identity: the simplified gradient
/// computes the same tensor as the raw one on random data (the max-shift
/// really does cancel, `exp(z − lse)` really is `softmax`).
#[test]
fn simplify_preserves_the_gradient() {
    let (b, c) = (axis("b", 8), axis("c", 5));
    let mut rng = Lcg(0x5117);
    let env: Env = [
        ("Z", rand_tensor(&[b, c], &mut rng)),
        ("y", Value::from_fn(&[b], |crd| (crd[&b] % 5) as f64)),
    ]
    .into_iter()
    .collect();

    let loss = cross_entropy(&input("Z", &[b, c]), b, c, false);
    let dz = grad(&loss, &["Z"])["Z"].clone();
    let simplified = simplify_many(&[loss, dz.clone()]).pop().unwrap();

    let raw = eval(&dz, &env);
    let simp = eval(&simplified, &env).permuted_to(&raw.axes);
    assert_eq!(raw.shape, simp.shape);
    for (a, b) in raw.data.iter().zip(&simp.data) {
        assert!(
            (a - b).abs() <= 1e-9 * (1.0 + a.abs()),
            "simplify changed the gradient value: {a} vs {b}"
        );
    }
}

/// The composed logsumexp's backward, simplified, partitions to the SAME number
/// of kernels as the hand-written `LogSumExp` primitive's — the winner-mask
/// cancels and the softmax reuses the forward carrier, so no LSE gradient rule
/// is needed to match it.
#[test]
fn composed_logsumexp_backward_matches_the_primitive() {
    let (b, c) = (axis("b", 100), axis("c", 10));
    let kernels = |primitive: bool| {
        let loss = cross_entropy(&input("Z", &[b, c]), b, c, primitive);
        let dz = grad(&loss, &["Z"])["Z"].clone();
        let roots = simplify_many(&[loss, dz]);
        partition_many(
            &[(roots[0].clone(), "loss"), (roots[1].clone(), "dZ")],
            &Device::toy(),
        )
        .stages
        .len()
    };
    assert_eq!(
        kernels(false),
        kernels(true),
        "composed backward must derive to the primitive's kernel count"
    );
}
