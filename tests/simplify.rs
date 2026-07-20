//! The algebraic simplifier, as a training-time graph pass: value-preserving,
//! and it derives a composed logsumexp's BACKWARD to the same kernels a
//! hand-written `softmax − onehot` rule would — no LSE-specific gradient rule,
//! no stop-gradient. The winner-mask of the stabilizing max-shift cancels, and
//! cross-root CSE lets the backward reuse the forward's logsumexp carrier.

use sanic::cost::DeviceProfile;
use sanic::grad::grad;
use sanic::interp::{Env, Value, eval};
use sanic::kernel_ir::*;
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
fn cross_entropy(logits: &NodeRef, b: Axis, c: Axis, primitive: bool) -> NodeRef {
    let lse = if primitive {
        reduce(logits.clone(), c, BinOp::Monoid(Monoid::LogSumExp))
    } else {
        let m = reduce(logits.clone(), c, BinOp::Monoid(Monoid::Max));
        let sh = map(
            MapOp::Exp,
            vec![map(MapOp::Sub, vec![logits.clone(), m.clone()])],
        );
        map(
            MapOp::Add,
            vec![
                m,
                map(MapOp::Log, vec![reduce(sh, c, BinOp::Monoid(Monoid::Add))]),
            ],
        )
    };
    let picked = reduce(
        map(
            MapOp::Mul,
            vec![logits.clone(), one_hot(c, input("y", &[b], Dtype::F32))],
        ),
        c,
        BinOp::Monoid(Monoid::Add),
    );
    reduce(
        map(MapOp::Sub, vec![lse, picked]),
        b,
        BinOp::Monoid(Monoid::Add),
    )
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

    let loss = cross_entropy(&input("Z", &[b, c], Dtype::F32), b, c, false);
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
        let loss = cross_entropy(&input("Z", &[b, c], Dtype::F32), b, c, primitive);
        let dz = grad(&loss, &["Z"])["Z"].clone();
        let roots = simplify_many(&[loss, dz]);
        partition_many(
            &[(roots[0].clone(), "loss"), (roots[1].clone(), "dZ")],
            &DeviceProfile::toy(),
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

/// Two separately constructed, structurally identical subgraphs become ONE
/// shared node: after canonicalization, structural equality is pointer
/// equality — and the merge preserves the computed value.
#[test]
fn canonicalize_merges_structural_duplicates() {
    let (b, c) = (axis("b", 4), axis("c", 6));
    let x = input("x", &[b, c], Dtype::F32);
    let row_energy = |x: &NodeRef| {
        reduce(
            map(MapOp::Mul, vec![x.clone(), x.clone()]),
            c,
            BinOp::Monoid(Monoid::Add),
        )
    };
    // Built twice on purpose: equal structure, distinct nodes.
    let once = row_energy(&x);
    let again = row_energy(&x);
    assert!(!std::rc::Rc::ptr_eq(&once, &again));

    let y = map(MapOp::Add, vec![once, again]); // 2·Σx² per row
    let canonical = canonicalize_many(std::slice::from_ref(&y)).pop().unwrap();
    let Node::Map { inputs, .. } = canonical.as_ref() else {
        panic!("expected the root Add to survive canonicalization");
    };
    assert!(
        std::rc::Rc::ptr_eq(&inputs[0], &inputs[1]),
        "the duplicated reduction must canonicalize to one shared node"
    );

    let mut rng = Lcg(0xCA11);
    let env: Env = [("x", rand_tensor(&[b, c], &mut rng))]
        .into_iter()
        .collect();
    assert_eq!(eval(&canonical, &env).data, eval(&y, &env).data);
}

/// Partition computes a structurally duplicated subgraph ONCE: building the
/// same graph with the subtree shared vs rebuilt yields the same number of
/// kernels, and the schedule still executes to the naive reference.
#[test]
fn partition_computes_a_structural_duplicate_once() {
    let (b, c) = (axis("b", 4), axis("c", 6));
    let x = || input("x", &[b, c], Dtype::F32);
    let row_energy = |x: NodeRef| {
        reduce(
            map(MapOp::Mul, vec![x.clone(), x]),
            c,
            BinOp::Monoid(Monoid::Add),
        )
    };
    // One fold consumed by two different parents — written once with the
    // node shared, once with the fold rebuilt from scratch.
    let combine = |e1: NodeRef, e2: NodeRef| {
        map(
            MapOp::Add,
            vec![map(MapOp::Exp, vec![e1]), map(MapOp::Sqrt, vec![e2])],
        )
    };
    let shared_fold = row_energy(x());
    let shared = combine(shared_fold.clone(), shared_fold);
    let duplicated = combine(row_energy(x()), row_energy(x()));

    let toy = DeviceProfile::toy();
    let shared_schedule = partition_many(&[(shared.clone(), "Out")], &toy);
    let duplicated_schedule = partition_many(&[(duplicated.clone(), "Out")], &toy);
    assert_eq!(
        duplicated_schedule.stages.len(),
        shared_schedule.stages.len(),
        "a structural duplicate must partition like an explicitly shared node"
    );

    let mut rng = Lcg(0xD00D);
    let env: Env = [("x", rand_tensor(&[b, c], &mut rng))]
        .into_iter()
        .collect();
    let executed = duplicated_schedule.execute(&env);
    let reference = eval(&duplicated, &env).permuted_to(&executed.axes);
    assert_eq!(executed.shape, reference.shape);
    for (got, want) in executed.data.iter().zip(&reference.data) {
        assert!(
            (got - want).abs() <= 1e-9 * (1.0 + want.abs()),
            "schedule diverged from the reference: {got} vs {want}"
        );
    }
}
