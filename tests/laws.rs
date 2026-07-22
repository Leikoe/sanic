//! Acceptance tests. The engine is correct on the core if, with no
//! hand-written accumulators, it produces these classifications and carriers.
//! Reconstructing the FlashAttention `(m, ℓ, o)` accumulator from the
//! composition rules — not a stored template — is the primary criterion.
//!
//! Every associative carrier family is held to
//! `tree_fold == fold == reference` on random data: associativity and
//! correctness in one assertion.

use sanic::analyze::{Parallelism, analyze, analyze_all, streamable, structure};
use sanic::derive::{Carrier, Expr, derive};
use sanic::ir::*;
use sanic::nn::scaled_dot_product_attention;

fn attention(query: NodeRef, key: NodeRef, value: NodeRef) -> NodeRef {
    scaled_dot_product_attention(query, key, value, None, 0.0, false, Some(1.0), false)
}

fn add_r() -> Monoid {
    Monoid::Add
}
fn max_r() -> Monoid {
    Monoid::Max
}
fn lse_r() -> Monoid {
    Monoid::LogSumExp
}

// ── a tiny deterministic PRNG so tests need no external crates ──────────────
struct Lcg(u64);
impl Lcg {
    fn new(seed: u64) -> Self {
        Lcg(seed.max(1))
    }
    fn next_f64(&mut self) -> f64 {
        // xorshift64*
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        let u = x.wrapping_mul(0x2545F4914F6CDD1D);
        ((u >> 11) as f64 / (1u64 << 53) as f64) * 6.0 - 3.0 // ~[-3, 3]
    }
}

/// Every comparison reads the one tolerance policy (`verify::rel_tolerance`);
/// `terms` is the reduction length the compared values accumulate over.
fn approx_within(a: f64, b: f64, terms: usize) {
    let tol = sanic::verify::rel_tolerance(Dtype::F64, terms) * (1.0 + a.abs().max(b.abs()));
    assert!(
        (a - b).abs() <= tol,
        "approx failed: {a} vs {b} (n = {terms})"
    );
}

/// A derived carrier must agree with itself under a tree split (associativity)
/// and with the reference semantics (`project ∘ fold = reference`).
fn check(car: &Carrier, items: &[Vec<f64>], reference: &[f64]) {
    let folded = car.fold(items);
    let tree = car.tree_fold(items);
    assert_eq!(folded.len(), reference.len());
    for i in 0..folded.len() {
        approx_within(folded[i], reference[i], items.len());
        approx_within(tree[i], reference[i], items.len()); // tree == sequential ⇒ associative
    }
}

// ── matmul tags i,j FREE and k MONOIDAL + linear ─────────────────────────────
#[test]
fn matmul_axis_tags() {
    let (i, j, k) = (axis("i", 5), axis("j", 6), axis("k", 8));
    let a = input("A", [i, k], Dtype::F32);
    let b = input("B", [k, j], Dtype::F32);
    let [i_axis, k_axis] = axis_refs(&a).try_into().unwrap();
    let j_axis = axis_refs(&b)[1];
    let mm = matmul(a, b);

    assert_eq!(structure(&mm, i_axis).level, Parallelism::Free);
    assert_eq!(structure(&mm, j_axis).level, Parallelism::Free);

    let sk = structure(&mm, k_axis);
    assert_eq!(sk.level, Parallelism::Monoidal);
    assert!(sk.linear, "contraction is a linear (+) reduction");
    assert!(streamable(&mm, k_axis));
}

// ── the dot-product carrier reproduces a contraction ─────────────────────────
#[test]
fn dot_product_carrier() {
    let k = axis("k", 17);
    let a = input("A", [k], Dtype::F32);
    let b = input("B", [k], Dtype::F32);
    let stream = axis_refs(&a)[0];
    let mm = reduce(map(MapOp::Mul, vec![a, b]), 0usize, Monoid::Add);

    let car = derive(&mm, stream).expect("dot-product dimension is derivable");
    assert_eq!(car.slots, 1);

    let mut rng = Lcg::new(1);
    let items: Vec<Vec<f64>> = (0..17)
        .map(|_| vec![rng.next_f64(), rng.next_f64()])
        .collect();
    let reference: f64 = items.iter().map(|p| p[0] * p[1]).sum();
    check(&car, &items, &[reference]);
}

// The derived FlashAttention carrier, rendered as math — the headline claim
// made legible and locked. The online-softmax rescaling combine and the
// deferred normalizer, constructed by composition over the closed basis
// (softmax is Exp(x − max), not a fused special form).
#[test]
fn renders_derived_flash_attention() {
    let (sq, k, d, e) = (axis("sq", 8), axis("k", 16), axis("d", 64), axis("e", 64));
    let q = input("Q", [sq, d], Dtype::F32);
    let kk = input("K", [k, d], Dtype::F32);
    let v = input("V", [k, e], Dtype::F32);
    let stream = axis_refs(&kk)[0];
    let car = derive(&attention(q, kk, v), stream).unwrap();
    let r = car.render();
    assert!(r.contains("into:    s0 = x0;  s1 = 1;  s2 = x1"));
    // each side's rescale factor is guarded: a −∞ max (the identity of an
    // all-masked partial) contributes weight 0 instead of exp(−∞ − −∞) = NaN
    assert!(r.contains(
        "s1 = a1·where(-∞ < a0, exp(a0 - max(a0, b0)), 0) + b1·where(-∞ < b0, exp(b0 - max(a0, b0)), 0)"
    ));
    assert!(r.contains(
        "s2 = a2·where(-∞ < a0, exp(a0 - max(a0, b0)), 0) + b2·where(-∞ < b0, exp(b0 - max(a0, b0)), 0)"
    ));
    assert!(r.contains("project: s2 / s1"));
}

// The structure map (the engine's named output): one call classifies every
// axis and attaches the derived accumulator to the foldable ones.
#[test]
fn structure_map_for_attention() {
    let (sq, k, d, e) = (axis("sq", 8), axis("k", 16), axis("d", 64), axis("e", 64));
    let q = input("Q", [sq, d], Dtype::F32);
    let kk = input("K", [k, d], Dtype::F32);
    let v = input("V", [k, e], Dtype::F32);
    let query_axis = axis_refs(&q)[0];
    let key_axis = axis_refs(&kk)[0];
    let report = analyze(&attention(q, kk, v), &[query_axis, key_axis]);

    let rs = &report.axes[0];
    assert_eq!(rs.structure.level, Parallelism::Free);
    assert!(rs.carrier.is_none(), "a grid axis has nothing to fold");

    let rk = &report.axes[1];
    assert_eq!(rk.structure.level, Parallelism::Monoidal);
    assert_eq!(
        rk.carrier.as_ref().unwrap().slots,
        3,
        "(m, ℓ, o) attached to k"
    );
}

// Zero-config: the engine discovers every axis itself and classifies it. For
// attention that is the fusion axis k (folds, carrier), the contraction d
// (folds one level down — no carrier here), and the grid axes sq, e.
#[test]
fn structure_map_auto_discovers_axes() {
    let (sq, k, d, e) = (axis("sq", 8), axis("k", 16), axis("d", 64), axis("e", 64));
    let q = input("Q", [sq, d], Dtype::F32);
    let kk = input("K", [k, d], Dtype::F32);
    let v = input("V", [k, e], Dtype::F32);
    let query_axis = axis_refs(&q)[0];
    let contract_axis = axis_refs(&q)[1];
    let key_axis = axis_refs(&kk)[0];
    let value_axis = axis_refs(&v)[1];
    let attn = attention(q, kk, v);

    let discovered = all_axis_refs(&attn);
    assert!(
        [query_axis, contract_axis, key_axis, value_axis]
            .iter()
            .all(|axis| discovered.contains(axis))
    );

    let report = analyze_all(&attn);
    let by = |axis: AxisRef| {
        report
            .axes
            .iter()
            .find(|report| report.axis == axis)
            .unwrap()
    };
    assert_eq!(by(key_axis).carrier.as_ref().unwrap().slots, 3); // the fusion axis
    assert_eq!(by(contract_axis).structure.level, Parallelism::Monoidal);
    assert!(
        by(contract_axis).carrier.is_none(),
        "the contraction folds deeper, not here"
    );
    assert_eq!(by(query_axis).structure.level, Parallelism::Free);
    assert_eq!(by(value_axis).structure.level, Parallelism::Free);

    let r = report.render();
    assert!(r.contains("project: s2 / s1"));
    assert!(r.contains("fold (in a sub-expression)"));
}

// The carrier knows its own accumulator size: each slot's free-axis span is
// derived from the graph's shapes, so |Acc| is exact (not a magic constant).
// For attention: m, ℓ are per-query {sq}; o additionally spans the value
// feature {sq, e}. Per query that is m + ℓ + o[d] = 2 + d scalars.
#[test]
fn carrier_knows_its_accumulator_size() {
    use std::collections::BTreeSet;

    let (sq, k, d, e) = (axis("sq", 8), axis("k", 16), axis("d", 64), axis("e", 64));
    let q = input("Q", [sq, d], Dtype::F32);
    let kk = input("K", [k, d], Dtype::F32);
    let v = input("V", [k, e], Dtype::F32);
    let stream = axis_refs(&kk)[0];
    let value_axis = axis_refs(&v)[1];
    let attn = attention(q, kk, v);

    // output-shape inference: attention is indexed by query and value-feature.
    let out: BTreeSet<Axis> = attn.shape().into_iter().collect();
    assert_eq!(out, [sq, e].into_iter().collect());

    let car = derive(&attn, stream).unwrap();
    let span = |i: usize| {
        car.spans[i]
            .iter()
            .map(|axis| (axis.name, axis.extent))
            .collect::<BTreeSet<_>>()
    };
    assert_eq!(
        span(0),
        [(sq.name, sq.extent)].into_iter().collect(),
        "m is per-query"
    );
    assert_eq!(
        span(1),
        [(sq.name, sq.extent)].into_iter().collect(),
        "ℓ is per-query"
    );
    assert_eq!(
        span(2),
        [(sq.name, sq.extent), (e.name, e.extent)]
            .into_iter()
            .collect(),
        "o spans query × value-feature"
    );

    // exact |Acc| per query (sq → 1) with value-feature extent e = 64.
    let acc = car.acc_scalars(|axis| if axis == value_axis { 64.0 } else { 1.0 });
    assert_eq!(acc, 2.0 + 64.0);
}

// Naive multi-head attention derives into the *same* FlashAttention kernel as
// single-head — fully generically. Batch and head are just extra free axes;
// the combine / into / project are byte-identical, only the accumulator's
// spans grow to carry them.
#[test]
fn multi_head_attention_derives_identically_to_single_head() {
    let (b, h, sq, k, d, e) = (
        axis("b", 2),
        axis("h", 3),
        axis("sq", 8),
        axis("k", 16),
        axis("d", 64),
        axis("e", 64),
    );
    let mq = input("Q", [b, h, sq, d], Dtype::F32);
    let mk = input("K", [b, h, k, d], Dtype::F32);
    let mv = input("V", [b, h, k, e], Dtype::F32);
    let [batch_axis, head_axis, _, _] = axis_refs(&mq).try_into().unwrap();
    let multi_stream = axis_refs(&mk)[2];
    let value_axis = axis_refs(&mv)[3];
    let mha = attention(mq, mk, mv);
    let sk = input("K", [k, d], Dtype::F32);
    let single_stream = axis_refs(&sk)[0];
    let sha = attention(
        input("Q", [sq, d], Dtype::F32),
        sk,
        input("V", [k, e], Dtype::F32),
    );
    let cm = derive(&mha, multi_stream).unwrap();
    let cs = derive(&sha, single_stream).unwrap();

    // the derived kernel is the same — no MHA special-casing
    assert_eq!(format!("{:?}", cm.into), format!("{:?}", cs.into));
    assert_eq!(format!("{:?}", cm.combine), format!("{:?}", cs.combine));
    assert_eq!(format!("{:?}", cm.project), format!("{:?}", cs.project));
    assert_eq!(cm.slots, 3);

    // only the spans differ: MHA's output slot carries the batch & head axes
    let o_span: std::collections::HashSet<_> = cm.spans[2].iter().copied().collect();
    assert!(
        [batch_axis, head_axis, value_axis]
            .iter()
            .all(|axis| o_span.contains(axis))
    );
}

// ── attention: sq FREE, k MONOIDAL, derives Acc = (m, ℓ, o), proj = o/ℓ ──────
#[test]
fn attention_axis_tags_and_carrier() {
    let (sq, k, d, e) = (axis("sq", 8), axis("k", 23), axis("d", 64), axis("e", 64));
    let q = input("Q", [sq, d], Dtype::F32);
    let kk = input("K", [k, d], Dtype::F32);
    let v = input("V", [k, e], Dtype::F32);
    let query_axis = axis_refs(&q)[0];
    let key_axis = axis_refs(&kk)[0];
    let attn = attention(q, kk, v);

    assert_eq!(structure(&attn, query_axis).level, Parallelism::Free);
    assert_eq!(structure(&attn, key_axis).level, Parallelism::Monoidal);
    assert!(streamable(&attn, key_axis));

    // The headline: derive the (m, ℓ, o) accumulator from the rules, by the
    // generic compositional fold — no FlashAttention-shaped template.
    let car = derive(&attn, key_axis).expect("attention key dimension is derivable");
    assert_eq!(car.slots, 3, "Acc = (m, ℓ, o)");
    // `rescale` = the online-softmax coupling; `defer-div` = the ÷ℓ
    // normalizer applied once at the end.
    assert_eq!(car.rules, vec!["defer-div", "fold", "rescale", "tuple"]);

    // Run it on random (score, value) pairs and compare to softmax·V.
    let mut rng = Lcg::new(42);
    let items: Vec<Vec<f64>> = (0..23)
        .map(|_| vec![rng.next_f64(), rng.next_f64()])
        .collect();

    let max = items.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
    let denom: f64 = items.iter().map(|p| (p[0] - max).exp()).sum();
    let numer: f64 = items.iter().map(|p| (p[0] - max).exp() * p[1]).sum();
    check(&car, &items, &[numer / denom]);
}

// ── derive (sum, count) for mean from its IR ─────────────────────────────────
#[test]
fn mean_carrier() {
    // mean = (Σ x) / (Σ 1) — the count is a fold over a literal 1.
    let a = axis("a", 31);
    let x = input("X", [a], Dtype::F32);
    let stream = axis_refs(&x)[0];
    let sum = reduce(x.clone(), 0usize, add_r());
    let count = reduce(ones_like(x.clone()), 0usize, add_r());
    let mean = map(MapOp::Div, vec![sum, count]);

    let car = derive(&mean, stream).expect("mean is derivable");
    assert_eq!(car.slots, 2, "Acc = (sum, count)");
    assert!(car.rules.contains(&"tuple")); // more than one slot needed

    let mut rng = Lcg::new(7);
    let xs: Vec<f64> = (0..31).map(|_| rng.next_f64()).collect();
    let items: Vec<Vec<f64>> = xs.iter().map(|&x| vec![x]).collect();
    let reference = xs.iter().sum::<f64>() / xs.len() as f64;
    check(&car, &items, &[reference]);
}

// ── derive a (Σx², Σx, count) variance carrier from its IR ───────────────────
#[test]
fn variance_carrier() {
    // var = E[x²] − E[x]²  =  Σx²/n − (Σx/n)²
    let a = axis("a", 40);
    let x = input("X", [a], Dtype::F32);
    let stream = axis_refs(&x)[0];
    let sumx2 = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), 0usize, add_r());
    let sumx = reduce(x.clone(), 0usize, add_r());
    let count = reduce(ones_like(x.clone()), 0usize, add_r());
    let ex2 = map(MapOp::Div, vec![sumx2, count.clone()]);
    let ex = map(MapOp::Div, vec![sumx, count]);
    let var = map(MapOp::Sub, vec![ex2, map(MapOp::Mul, vec![ex.clone(), ex])]);

    let car = derive(&var, stream).expect("variance is derivable");
    assert_eq!(car.slots, 3, "Acc = (Σx², Σx, count)");

    let mut rng = Lcg::new(99);
    let xs: Vec<f64> = (0..40).map(|_| rng.next_f64()).collect();
    let items: Vec<Vec<f64>> = xs.iter().map(|&x| vec![x]).collect();
    let n = xs.len() as f64;
    let mu = xs.iter().sum::<f64>() / n;
    let reference = xs.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / n;
    check(&car, &items, &[reference]);
}

// ── derive (max, Σexp) for logsumexp from its IR — built from Exp∘Sub ────────
#[test]
fn logsumexp_carrier() {
    // lse(x) = log(Σ exp(x − m)) + m,   m = max x
    let a = axis("a", 29);
    let x = input("X", [a], Dtype::F32);
    let stream = axis_refs(&x)[0];
    let m = reduce(x.clone(), 0usize, max_r());
    let e = map(
        MapOp::Exp,
        vec![map(MapOp::Sub, vec![x.clone(), m.clone()])],
    );
    let s = reduce(e, 0usize, add_r());
    let lse = map(MapOp::Add, vec![map(MapOp::Log, vec![s]), m]);

    let car = derive(&lse, stream).expect("logsumexp is derivable");
    assert_eq!(car.slots, 2, "Acc = (max, Σexp)");
    assert!(car.rules.contains(&"rescale")); // the max/exp coupling

    let mut rng = Lcg::new(123);
    let xs: Vec<f64> = (0..29).map(|_| rng.next_f64()).collect();
    let items: Vec<Vec<f64>> = xs.iter().map(|&x| vec![x]).collect();
    let reference = xs.iter().map(|x| x.exp()).sum::<f64>().ln();
    check(&car, &items, &[reference]);
}

// ── payloads filtered to an extremal key form a generic product monoid ──────
#[test]
fn payloads_at_an_extremal_key_derive_generically() {
    let n = axis("n", 7);
    let key = input("K", [n], Dtype::F32);
    let payload = input("P", [n], Dtype::F32);
    let stream = axis_refs(&key)[0];
    let maximum = reduce(key.clone(), 0usize, max_r());
    let at_maximum = map(
        MapOp::Where,
        vec![
            map(MapOp::Lt, vec![key, maximum]),
            konst(Monoid::Add.identity()),
            payload,
        ],
    );
    let sum_at_maximum = reduce(at_maximum, 0usize, add_r());

    let carrier = derive(&sum_at_maximum, stream).expect("extremal payloads are derivable");
    assert_eq!(carrier.slots, 2, "Acc = (maximum key, tied payload sum)");
    assert!(carrier.rules.contains(&"extremum-filter"));

    let keys = [-1.0, 4.0, 2.0, 4.0, -1.0, 4.0, 3.0];
    let payloads = [10.0, 2.0, 20.0, 3.0, 30.0, 5.0, 40.0];
    let items: Vec<Vec<f64>> = keys
        .into_iter()
        .zip(payloads)
        .map(|(key, payload)| vec![key, payload])
        .collect();
    check(&carrier, &items, &[10.0]);

    let key = input("K", [n], Dtype::F32);
    let payload = input("P", [n], Dtype::F32);
    let stream = axis_refs(&key)[0];
    let minimum = reduce(key.clone(), 0usize, Monoid::Min);
    let at_minimum = map(
        MapOp::Where,
        vec![
            map(MapOp::Lt, vec![minimum, key]),
            konst(Monoid::Max.identity()),
            payload,
        ],
    );
    let max_at_minimum = reduce(at_minimum, 0usize, max_r());
    let carrier = derive(&max_at_minimum, stream).expect("minimum-key payloads are derivable");
    assert_eq!(carrier.slots, 2, "Acc = (minimum key, tied payload max)");
    assert!(carrier.rules.contains(&"extremum-filter"));
    check(&carrier, &items, &[30.0]);
}

// ── magnitude tiers: the rescale coupling exercised for its numerical purpose ─
// Everything above operates near ±3, where a naive unshifted Σexp never
// overflows — the online-softmax coupling was verified for its algebra but
// never for the regime it exists for. These tiers center the scores at ±50,
// ±88 (the f32 exp overflow boundary — the GPU oracle's tier), ±300, and
// ±710 (past f64 exp overflow at ~709.8), plant the true maximum first and
// last in the stream, and hold the carrier — sequential AND tree-split — to
// a stable two-pass reference at exactly the magnitudes where the naive
// fold visibly does not survive.

/// Score streams centered at `center`, with the true maximum planted at
/// `max_pos`: the adversarial cases for a rescaling merge (a late max forces
/// every accumulated slot to rescale; an early max forces the tail to ride
/// far below it).
fn planted_scores(center: f64, n: usize, max_pos: usize, seed: u64) -> Vec<f64> {
    let mut rng = Lcg::new(seed);
    let mut scores: Vec<f64> = (0..n).map(|_| center + rng.next_f64()).collect();
    scores[max_pos] = center + 5.0;
    scores
}

#[test]
fn attention_rescale_survives_overflow_magnitudes() {
    let (sq, k, d, e) = (axis("sq", 8), axis("k", 16), axis("d", 64), axis("e", 64));
    let q = input("Q", [sq, d], Dtype::F32);
    let kk = input("K", [k, d], Dtype::F32);
    let v = input("V", [k, e], Dtype::F32);
    let stream = axis_refs(&kk)[0];
    let car = derive(&attention(q, kk, v), stream).unwrap();

    let n = 23;
    for &center in &[50.0, 88.0, 300.0, 710.0, -710.0] {
        for max_pos in [0, n - 1] {
            let scores = planted_scores(center, n, max_pos, 42);
            let mut rng = Lcg::new(7);
            let items: Vec<Vec<f64>> = scores.iter().map(|&s| vec![s, rng.next_f64()]).collect();

            // The stable two-pass reference: shift by the true max.
            let max = scores.iter().fold(f64::NEG_INFINITY, |a, &s| a.max(s));
            let denom: f64 = scores.iter().map(|s| (s - max).exp()).sum();
            let numer: f64 = items.iter().map(|p| (p[0] - max).exp() * p[1]).sum();
            check(&car, &items, &[numer / denom]);

            // The tier has teeth: past f64 exp overflow the NAIVE unshifted
            // fold is not even finite — the carrier's agreement above is not
            // an agreement between two computations that both happen to work.
            if center >= 710.0 {
                let naive: f64 = items.iter().map(|p| p[0].exp() * p[1]).sum::<f64>()
                    / scores.iter().map(|s| s.exp()).sum::<f64>();
                assert!(!naive.is_finite(), "naive fold must overflow at ±{center}");
            }
        }
    }
}

#[test]
fn logsumexp_rescale_survives_overflow_magnitudes() {
    let a = axis("a", 23);
    let x = input("X", [a], Dtype::F32);
    let stream = axis_refs(&x)[0];
    let m = reduce(x.clone(), 0usize, max_r());
    let e = map(
        MapOp::Exp,
        vec![map(MapOp::Sub, vec![x.clone(), m.clone()])],
    );
    let s = reduce(e, 0usize, add_r());
    let lse = map(MapOp::Add, vec![map(MapOp::Log, vec![s]), m]);
    let car = derive(&lse, stream).unwrap();

    let n = 23;
    for &center in &[50.0, 88.0, 300.0, 710.0, -710.0] {
        for max_pos in [0, n - 1] {
            let scores = planted_scores(center, n, max_pos, 11);
            let items: Vec<Vec<f64>> = scores.iter().map(|&s| vec![s]).collect();

            let max = scores.iter().fold(f64::NEG_INFINITY, |a, &s| a.max(s));
            let reference = max + scores.iter().map(|s| (s - max).exp()).sum::<f64>().ln();
            check(&car, &items, &[reference]);

            if center >= 710.0 {
                let naive = scores.iter().map(|s| s.exp()).sum::<f64>().ln();
                assert!(
                    naive.is_infinite(),
                    "naive logsumexp must overflow at ±{center}"
                );
            }
        }
    }
}

// ── real-workload attention: scale and mask fuse into the lift ───────────────
// softmax(scores·scale + mask)·V — the production form. The compound score
// expression fuses into the per-element lift (`fused-map`), and the carrier is
// still the same 3-slot FlashAttention accumulator.
#[test]
fn masked_scaled_attention_derives() {
    let (s, k, e) = (axis("s", 8), axis("k", 21), axis("e", 64));
    let scores = input("S", [s, k], Dtype::F32);
    let stream = axis_refs(&scores)[1];
    let scale = input("scale", [], Dtype::F32);
    let mask = input("M", [s, k], Dtype::F32);
    let v = input("V", [k, e], Dtype::F32);

    let sc = map(MapOp::Add, vec![map(MapOp::Mul, vec![scores, scale]), mask]);
    let out = matmul(softmax(sc, 1usize), v);

    let car = derive(&out, stream).expect("masked scaled attention derivable");
    assert_eq!(car.slots, 3, "still (m, ℓ, o)");
    assert!(
        car.rules.contains(&"fused-map"),
        "scale+mask fused into the lift"
    );
    assert!(car.rules.contains(&"rescale"));
    assert!(car.rules.contains(&"defer-div"));

    // element = (score, scale, mask, value); reference = softmax(s·c + m)·v
    let mut rng = Lcg::new(31337);
    let c = 0.125; // 1/√64
    let items: Vec<Vec<f64>> = (0..21)
        .map(|_| vec![rng.next_f64(), c, rng.next_f64(), rng.next_f64()])
        .collect();
    let z = |p: &[f64]| p[0] * p[1] + p[2];
    let mx = items.iter().map(|p| z(p)).fold(f64::NEG_INFINITY, f64::max);
    let den: f64 = items.iter().map(|p| (z(p) - mx).exp()).sum();
    let num: f64 = items.iter().map(|p| (z(p) - mx).exp() * p[3]).sum();
    check(&car, &items, &[num / den]);
}

// ── a COMPUTED causal mask: iota + compare + where, fused into the lift ──────
// No mask tensor exists; the mask is index arithmetic, and the carrier is
// still the 3-slot flash accumulator.
#[test]
fn computed_causal_mask_derives() {
    let (s, t, e) = (axis("s", 8), axis("t", 12), axis("e", 64));
    let scores = input("S", [s, t], Dtype::F32);
    let stream = axis_refs(&scores)[1];
    let v = input("V", [t, e], Dtype::F32);
    let masked = map(
        MapOp::Add,
        vec![scores.clone(), causal_mask_like(scores, 0usize, 1usize)],
    );
    let out = matmul(softmax(masked, 1usize), v);

    let car = derive(&out, stream).expect("causally masked attention derivable");
    assert_eq!(car.slots, 3);
    assert!(car.rules.contains(&"fused-map"));

    // element = (score, query index, key index, value); the reference masks
    // out keys strictly after the query position.
    let mut rng = Lcg::new(777);
    let qi = 4.0; // fixed query position for this stream
    let items: Vec<Vec<f64>> = (0..12)
        .enumerate()
        .map(|(ti, _)| vec![rng.next_f64(), qi, ti as f64, rng.next_f64()])
        .collect();
    let z = |p: &[f64]| if p[1] < p[2] { p[0] - 1e30 } else { p[0] };
    let mx = items.iter().map(|p| z(p)).fold(f64::NEG_INFINITY, f64::max);
    let den: f64 = items.iter().map(|p| (z(p) - mx).exp()).sum();
    let num: f64 = items.iter().map(|p| (z(p) - mx).exp() * p[3]).sum();
    check(&car, &items, &[num / den]);
}

// ── every unary MapOp in the closed basis can ride a reduction lift ─────────
#[test]
fn tanh_fuses_into_a_reduction() {
    let n = axis("n", 17);
    let x = input("X", [n], Dtype::F32);
    let stream = axis_refs(&x)[0];
    let sum = reduce(map(MapOp::Tanh, vec![x]), 0usize, Monoid::Add);
    let carrier = derive(&sum, stream).expect("tanh should ride the additive lift");
    assert_eq!(carrier.slots, 1);
    assert!(carrier.rules.contains(&"fused-map"));

    let mut rng = Lcg::new(0x7A4);
    let values: Vec<f64> = (0..17).map(|_| rng.next_f64()).collect();
    let items: Vec<Vec<f64>> = values.iter().map(|&value| vec![value]).collect();
    check(
        &carrier,
        &items,
        &[values.iter().map(|value| value.tanh()).sum()],
    );
}

// ── real-workload MLP: silu (a composition!) fuses into the down-proj lift ──
// Σ_f silu(gate_f)·up_f·w_f — an activation-fused GEMM, derived not matched.
#[test]
fn silu_fuses_into_a_contraction() {
    let f = axis("f", 27);
    let gate = input("G", [f], Dtype::F32);
    let stream = axis_refs(&gate)[0];
    let up = input("U", [f], Dtype::F32);
    let w = input("Wd", [f], Dtype::F32);
    let act = map(MapOp::Mul, vec![silu(gate), up]);
    let down = reduce(map(MapOp::Mul, vec![act, w]), 0usize, add_r());

    let car = derive(&down, stream).expect("silu contraction derivable");
    assert_eq!(
        car.slots, 1,
        "one running sum — the activation is in the lift"
    );
    assert!(car.rules.contains(&"fused-map"));

    let mut rng = Lcg::new(808);
    let items: Vec<Vec<f64>> = (0..27)
        .map(|_| vec![rng.next_f64(), rng.next_f64(), rng.next_f64()])
        .collect();
    let silu_f = |x: f64| x / (1.0 + (-x).exp());
    let reference: f64 = items.iter().map(|p| silu_f(p[0]) * p[1] * p[2]).sum();
    check(&car, &items, &[reference]);
}

// ── RMSNorm fuses into a consuming projection: the whole norm becomes a
// second slot plus a deferred normalizer — Σ(x·g·w) / √(Σx²/n + ε), one pass.
#[test]
fn rmsnorm_fused_projection_carrier() {
    let d = axis("d", 16);
    let x = input("X", [d], Dtype::F32);
    let stream = axis_refs(&x)[0];
    let g = input("G", [d], Dtype::F32);
    let w = input("W", [d], Dtype::F32);
    let n = 16.0;
    let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), 0usize, add_r());
    let mean = map(MapOp::Mul, vec![ss, konst(1.0 / n)]);
    let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
    let norm = map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom]);
    let proj = reduce(map(MapOp::Mul, vec![norm, w]), 0usize, Monoid::Add);

    let car = derive(&proj, stream).expect("norm-fused projection derivable");
    assert_eq!(car.slots, 2, "Acc = (Σ x·g·w, Σx²)");
    assert!(car.rules.contains(&"defer-div"));

    let mut rng = Lcg::new(1234);
    let items: Vec<Vec<f64>> = (0..16)
        .map(|_| vec![rng.next_f64(), rng.next_f64(), rng.next_f64()])
        .collect();
    let dot: f64 = items.iter().map(|p| p[0] * p[1] * p[2]).sum();
    let ssq: f64 = items.iter().map(|p| p[0] * p[0]).sum();
    let reference = dot / (ssq / n + 1e-5).sqrt();
    check(&car, &items, &[reference]);
}

// ── views: axes are variables, reduction binds them, a view re-binds ─────────
#[test]
fn view_scoping_rules() {
    // rename: the same values under a new position variable.
    let (s, t, dm, f, sf) = (
        axis("s", 4),
        axis("t", 4),
        axis("dm", 8),
        axis("f", 6),
        axis("sf", 24),
    );
    let x = input("X", [s, dm], Dtype::F32);
    let [source_s, _] = axis_refs(&x).try_into().unwrap();
    let xt = rename(x.clone(), 0usize, t);
    let renamed_t = axis_refs(&xt)[0];
    assert_eq!(xt.shape(), vec![t, dm]);
    assert_eq!(structure(&xt, renamed_t).level, Parallelism::Free);
    assert_eq!(
        structure(&xt, source_s).level,
        Parallelism::Free,
        "the consumed name is out of scope above the view"
    );

    // a grouped output joins its members' structures; the members go out of
    // scope — asking about them above the view is asking about variables
    // that no longer exist.
    let weight = input("W", [f, dm], Dtype::F32);
    let output_f = axis_refs(&weight)[0];
    let mm = matmul(x, transpose(weight, 0usize, 1usize)); // [s, f]
    let grouped = flatten(mm, &[0, 1][..], sf);
    let grouped_axis = axis_refs(&grouped)[0];
    assert_eq!(grouped.shape(), vec![sf]);
    assert_eq!(structure(&grouped, grouped_axis).level, Parallelism::Free);
    assert_eq!(structure(&grouped, source_s).level, Parallelism::Free);
    assert_eq!(structure(&grouped, output_f).level, Parallelism::Free);
}

// ── a fold whose leaf is a flattened view — the MHA output projection ────────
#[test]
fn fold_through_a_flattened_view() {
    // out = Σ_dmv flat[dmv]·w[dmv], where flat reindexes a *computed* [h, dv]
    let (h, dv, dmv) = (axis("h", 4), axis("dv", 6), axis("dmv", 24));
    let a = input("A", [h, dv], Dtype::F32);
    let b = input("B", [h, dv], Dtype::F32);
    let prod = map(MapOp::Mul, vec![a, b]); // computed, not a raw input
    let flat = flatten(prod, &[0, 1][..], dmv);
    let stream = axis_refs(&flat)[0];
    let w = input("W", [dmv], Dtype::F32);
    let out = reduce(map(MapOp::Mul, vec![flat, w]), 0usize, add_r());

    let car = derive(&out, stream).expect("folds over the flattened axis");
    assert_eq!(car.slots, 1, "a plain contraction against the view");

    // element = (flattened value, weight) per flattened index
    let mut rng = Lcg::new(4242);
    let items: Vec<Vec<f64>> = (0..24)
        .map(|_| vec![rng.next_f64(), rng.next_f64()])
        .collect();
    let reference: f64 = items.iter().map(|p| p[0] * p[1]).sum();
    check(&car, &items, &[reference]);
}

// ── embedding / gather axis OPAQUE ───────────────────────────────────────────
#[test]
fn embedding_is_opaque() {
    let (vocab, d, seq) = (axis("vocab", 32), axis("d", 16), axis("seq", 8));
    let table = input("E", [vocab, d], Dtype::F32);
    let selected = axis_refs(&table)[0];
    let ids = input("ids", [seq], Dtype::F32);
    let emb = embedding(table, ids, 0usize);
    assert_eq!(structure(&emb, selected).level, Parallelism::Opaque);
    assert!(!streamable(&emb, selected));
    assert!(derive(&emb, selected).is_err());
}

// ── Scan has complete scalar-monoid semantics and an honest fallback stage ─
#[test]
fn scalar_prefix_scan_executes_through_the_fallback() {
    use sanic::cost::DeviceProfile;
    use sanic::interp::{Env, Value, eval};
    use sanic::partition::{Stage, partition};

    let n = axis("n", 5);
    let values = Value::from_shape_fn(&[5], |coordinate| [2.0, -1.0, 3.0, 0.5, 4.0][coordinate[0]]);
    let env: Env = [("X", values)].into_iter().collect();
    let prefix = scan(input("X", [n], Dtype::F32), 0usize, Monoid::Add);

    assert_eq!(eval(&prefix, &env).data, vec![2.0, 1.0, 4.0, 4.5, 8.5]);
    let schedule = partition(&prefix, &DeviceProfile::toy());
    assert!(matches!(
        schedule.stages.as_slice(),
        [Stage::Fallback { .. }]
    ));
    assert_eq!(schedule.execute(&env).data, vec![2.0, 1.0, 4.0, 4.5, 8.5]);
}

// ── per-(node, axis): the middle axis of a double-GEMM is reduced in one
// sub-expression and FREE in another — the two fusion kinds, distinguished. ──
#[test]
fn per_node_axis_double_gemm() {
    // (X·Y)·Z. The middle axis `m` is a FREE output index of GEMM-1 but the
    // contraction of GEMM-2 — the same axis, two structures, distinguished
    // only because the analysis is per (node, axis) rather than collapsed
    // onto the output.
    let (i, a, m, j) = (axis("i", 4), axis("a", 5), axis("m", 6), axis("j", 7));
    let x = input("X", [i, a], Dtype::F32);
    let y = input("Y", [a, m], Dtype::F32);
    let middle = axis_refs(&y)[1];
    let g1 = matmul(x, y); // contracts a → output [i, m]
    let z = input("Z", [m, j], Dtype::F32);
    let g2 = matmul(g1.clone(), z); // contracts m → output [i, j]

    // `m` is a free output index of GEMM-1 ...
    assert_eq!(structure(&g1, middle).level, Parallelism::Free);
    // ... but the contraction of GEMM-2, at that node.
    assert_eq!(structure(&g2, middle).level, Parallelism::Monoidal);
}

// associativity must hold for *every* split point, exercised at the
// accumulator level via the public `fold_acc` / `merge` / `project`.
#[test]
fn flash_attention_associative_all_splits() {
    let (sq, k, d, e) = (axis("sq", 8), axis("k", 12), axis("d", 64), axis("e", 64));
    let q = input("Q", [sq, d], Dtype::F32);
    let kk = input("K", [k, d], Dtype::F32);
    let v = input("V", [k, e], Dtype::F32);
    let stream = axis_refs(&kk)[0];
    let attn = attention(q, kk, v);
    let car = derive(&attn, stream).unwrap();
    let mut rng = Lcg::new(2024);
    let items: Vec<Vec<f64>> = (0..12)
        .map(|_| vec![rng.next_f64(), rng.next_f64()])
        .collect();
    let whole = car.fold(&items);
    for split in 1..items.len() {
        let l = car.fold_acc(&items[..split]);
        let r = car.fold_acc(&items[split..]);
        let merged = car.project(&car.merge(&l, &r));
        for i in 0..whole.len() {
            approx_within(whole[i], merged[i], items.len());
        }
    }
}

// ── the SAME fold handles attention over *several* value tensors: two o-slots
// ride the one softmax. A FlashAttention template could not. ─────────────────
#[test]
fn multi_value_attention_generalizes() {
    // Σ softmax(scores)·V1 and Σ softmax(scores)·V2, summed — the coupling
    // with two deferred linear reductions sharing one (m, s).
    let k = axis("k", 15);
    let scores = input("S", [k], Dtype::F32);
    let stream = axis_refs(&scores)[0];
    let v1 = input("V1", [k], Dtype::F32);
    let v2 = input("V2", [k], Dtype::F32);
    let w = softmax(scores, 0usize);
    let o1 = reduce(map(MapOp::Mul, vec![w.clone(), v1]), 0usize, add_r());
    let o2 = reduce(map(MapOp::Mul, vec![w, v2]), 0usize, add_r());
    let total = map(MapOp::Add, vec![o1, o2]);

    let car = derive(&total, stream).expect("multi-value attention derivable");
    assert_eq!(car.slots, 4, "Acc = (m, s, o1, o2) — one shared softmax");

    let mut rng = Lcg::new(2027);
    let items: Vec<Vec<f64>> = (0..15)
        .map(|_| vec![rng.next_f64(), rng.next_f64(), rng.next_f64()])
        .collect();
    let mx = items.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
    let denom: f64 = items.iter().map(|p| (p[0] - mx).exp()).sum();
    let n1: f64 = items.iter().map(|p| (p[0] - mx).exp() * p[1]).sum();
    let n2: f64 = items.iter().map(|p| (p[0] - mx).exp() * p[2]).sum();
    check(&car, &items, &[n1 / denom + n2 / denom]);
}

// ── CTC's inner transition and final loss use the same derived carrier ───────
//
//   α_t[s] = logsumexp(α_{t-1}[s], α_{t-1}[s-1], α_{t-1}[s-2]) + logp_t[ℓ(s)]
//   loss   = −logsumexp_s α_T[s]
//
#[test]
fn ctc_logsumexp_carriers_match() {
    // The (max, Σexp) carrier must be DERIVED, not stored: 2 slots, the
    // rescaling combine (telescoping exp — not a naive Add), and the
    // log-space projection `log(s) + m`.
    fn has_exp(e: &Expr) -> bool {
        match e {
            Expr::Exp(_) => true,
            Expr::Add(a, b)
            | Expr::Mul(a, b)
            | Expr::Sub(a, b)
            | Expr::Div(a, b)
            | Expr::Max(a, b)
            | Expr::Min(a, b)
            | Expr::Lt(a, b) => has_exp(a) || has_exp(b),
            Expr::Log(a) | Expr::Sqrt(a) => has_exp(a),
            Expr::Where(c, a, b) => has_exp(c) || has_exp(a) || has_exp(b),
            _ => false,
        }
    }
    fn is_logsumexp_carrier(c: &Carrier) -> bool {
        c.slots == 2
            && c.identity[0] == f64::NEG_INFINITY
            && c.identity[1] == 0.0
            && matches!(&c.combine[0], Expr::Max(..)) // m = max
            && has_exp(&c.combine[1]) // s telescopes via exp(m_old − m_new)
            && matches!(&c.project[0],   // project = log(s) + m
                Expr::Add(lg, m) if matches!(**lg, Expr::Log(_)) && matches!(**m, Expr::F(0)))
    }

    // axes: b=batch, t=time, s=state, pred=predecessor (2–3 states), v=vocab
    let (b, t, s, pred, v) = (
        axis("b", 2),
        axis("t", 8),
        axis("s", 7),
        axis("pred", 6),
        axis("v", 32),
    );
    let logp = input("logp", [b, t, v], Dtype::F32);
    let batch_axis = axis_refs(&logp)[0];
    let vocab_axis = axis_refs(&logp)[2];
    let labels = input("labels", [s], Dtype::F32);
    let emit = gather(logp, labels, 2usize); // logp_t[ℓ(s)] — index vocab by label

    let prev = input("alpha_prev", [b, pred, s], Dtype::F32); // α_{t-1} at predecessors
    let pred_axis = axis_refs(&prev)[1];
    let state_axis = axis_refs(&prev)[2];
    let trans = reduce(prev, 1usize, lse_r()); // logsumexp over predecessors
    let step = map(
        MapOp::Add,
        vec![unsqueeze(trans.clone(), 1usize), emit.clone()],
    ); // + emission
    let alpha_t = input("alpha_T", [b, s], Dtype::F32);
    let final_state_axis = axis_refs(&alpha_t)[1];
    let loss = reduce(alpha_t, 1usize, lse_r()); // final logsumexp over states

    // ── b (batch) → FREE → grid ──────────────────────────────────────────────
    assert_eq!(structure(&step, batch_axis).level, Parallelism::Free);

    // ── s within a timestep → FREE → parallel across states ─────────────────
    assert_eq!(structure(&step, state_axis).level, Parallelism::Free);

    // ── pred → MONOIDAL, derive (max, Σexp), fuse, no intermediate ──────────
    assert_eq!(structure(&trans, pred_axis).level, Parallelism::Monoidal);
    assert!(structure(&trans, pred_axis).linear, "log-semiring additive");
    let tc = derive(&trans, pred_axis).expect("transition logsumexp derivable");
    assert!(
        is_logsumexp_carrier(&tc),
        "derived (m, s) carrier, project = log(s) + m"
    );
    assert!(tc.rules.contains(&"rescale"));
    {
        let mut rng = Lcg::new(314);
        let xs: Vec<f64> = (0..6).map(|_| rng.next_f64()).collect(); // 2–3+ predecessors
        let items: Vec<Vec<f64>> = xs.iter().map(|&x| vec![x]).collect();
        let reference = xs.iter().map(|x| x.exp()).sum::<f64>().ln();
        check(&tc, &items, &[reference]);
    }

    // ── v (vocab, via label gather) → OPAQUE → not foldable ─────────────────
    assert_eq!(structure(&emit, vocab_axis).level, Parallelism::Opaque);
    assert!(!streamable(&emit, vocab_axis));
    assert!(derive(&emit, vocab_axis).is_err());

    // ── final s reduction → MONOIDAL, (max, Σexp), log(s)+m ─────────────────
    assert_eq!(
        structure(&loss, final_state_axis).level,
        Parallelism::Monoidal
    );
    let lc = derive(&loss, final_state_axis).expect("final logsumexp derivable");
    assert!(is_logsumexp_carrier(&lc));
    assert!(lc.rules.contains(&"rescale"));

    // Both logsumexps derive uniformly to the same (m, s) accumulator; there
    // is no stored CTC or loss template.
    assert!(streamable(&trans, pred_axis) && streamable(&loss, final_state_axis));
    assert_eq!(format!("{:?}", tc.combine), format!("{:?}", lc.combine));
    assert_eq!(format!("{:?}", tc.into), format!("{:?}", lc.into));
    assert_eq!(format!("{:?}", tc.project), format!("{:?}", lc.project));
}

// ── the harder composite: a soft-attention readout over a streamed log-space
//    DP. ONE graph that forces the rescaling coupling, the deferred
//    normalizer, OPAQUE, and the same-axis-merge vs cross-axis-tiling
//    distinction at once.
#[test]
fn soft_attention_over_logspace_dp() {
    // gather-indexed score: K is selected from a table by a runtime label.
    let (b, d, idx, k, h, t) = (
        axis("b", 2),
        axis("d", 8),
        axis("idx", 4),
        axis("k", 20),
        axis("h", 14),
        axis("t", 6),
    );
    let q = input("Q", [b, d], Dtype::F32);
    let batch_axis = axis_refs(&q)[0];
    let contract_axis = axis_refs(&q)[1];
    let ktable = input("Ktable", [idx, k, d], Dtype::F32);
    let index_axis = axis_refs(&ktable)[0];
    let key_axis = axis_refs(&ktable)[1];
    let labels = input("labels", [b], Dtype::F32);
    let kgath = gather(ktable, labels, 0usize); // K[b,k,d] via runtime index → OPAQUE on idx
    let score = reduce(
        map(MapOp::Mul, vec![unsqueeze(q, 1usize), kgath.clone()]),
        2usize,
        add_r(),
    ); // linear contraction on d
    let weight = softmax(score.clone(), 1usize); // (m, s) coupling

    // VALUES are themselves a streamed reduction: a logsumexp-matmul over h.
    let wv = input("Wv", [k, h], Dtype::F32);
    let value_key_axis = axis_refs(&wv)[0];
    let hidden_axis = axis_refs(&wv)[1];
    let hmat = input("H", [b, t, k, h], Dtype::F32);
    let value = reduce(map(MapOp::Add, vec![wv, hmat]), 3usize, lse_r()); // LogMatMul contract=h

    let out = reduce(
        map(MapOp::Mul, vec![unsqueeze(weight, 1usize), value.clone()]),
        2usize,
        add_r(),
    ); // linear consumer on k
    // ── d: the score contraction is MONOIDAL and linear ─────────────────────
    let sd = structure(&score, contract_axis);
    assert_eq!(sd.level, Parallelism::Monoidal);
    assert!(sd.linear);

    // ── same-axis MERGE on k: derive the (m, s, o) carrier — coupling plus
    //    deferred normalizer. The values are atomic per-k inputs, so exactly
    //    3 slots — h is NOT swallowed into the k-fold. ─────────────────────────
    assert_eq!(structure(&out, key_axis).level, Parallelism::Monoidal);
    let oc = derive(&out, key_axis).expect("k readout derivable");
    assert_eq!(
        oc.slots, 3,
        "Acc = (m, s, o) — softmax merged, h not fused in"
    );
    assert!(oc.rules.contains(&"rescale"), "online-softmax coupling");
    assert!(oc.rules.contains(&"defer-div"), "deferred normalizer");
    assert_eq!(oc.identity, vec![f64::NEG_INFINITY, 0.0, 0.0]);
    {
        let mut rng = Lcg::new(7001);
        let items: Vec<Vec<f64>> = (0..20) // (score, value) per key
            .map(|_| vec![rng.next_f64(), rng.next_f64()])
            .collect();
        let mx = items.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
        let den: f64 = items.iter().map(|p| (p[0] - mx).exp()).sum();
        let num: f64 = items.iter().map(|p| (p[0] - mx).exp() * p[1]).sum();
        check(&oc, &items, &[num / den]);
    }

    // ── cross-axis PRODUCER on h: value's own (max, Σexp) carrier, with the
    //    log-space product (Wv + H) fused into `into`, plus the coupling. ─────
    assert_eq!(structure(&value, hidden_axis).level, Parallelism::Monoidal);
    let vc = derive(&value, hidden_axis).expect("logsumexp-matmul derivable");
    assert_eq!(vc.slots, 2, "(max, Σexp)");
    assert!(
        vc.rules.contains(&"fused-map"),
        "the additive pre-map fused into the lift"
    );
    assert!(vc.rules.contains(&"rescale"));
    // project = log(s) + m
    assert!(matches!(&vc.project[0],
        Expr::Add(lg, m) if matches!(**lg, Expr::Log(_)) && matches!(**m, Expr::F(0))));
    {
        let mut rng = Lcg::new(7002);
        let items: Vec<Vec<f64>> = (0..14) // (Wv_h, H_h) per contracted h
            .map(|_| vec![rng.next_f64(), rng.next_f64()])
            .collect();
        let reference = items.iter().map(|p| (p[0] + p[1]).exp()).sum::<f64>().ln();
        check(&vc, &items, &[reference]);
    }

    // ── THE double-fusion distinction, surfaced per (node, axis): at `out`,
    //    BOTH k and h are foldable, but they are different fusion kinds.
    //    `value` is FREE along k (→ cross-axis tiling, a leaf in the k-fold),
    //    while h is still MONOIDAL at the `out` node (the producer reduction
    //    lives one level down). ─────────────────────────────────────────────
    assert_eq!(structure(&value, value_key_axis).level, Parallelism::Free);
    assert_eq!(structure(&out, hidden_axis).level, Parallelism::Monoidal);
    assert_eq!(structure(&out, key_axis).level, Parallelism::Monoidal);

    // ── OPAQUE: the gather-indexed score input is runtime-determined ────────
    assert_eq!(structure(&kgath, index_axis).level, Parallelism::Opaque);
    assert!(derive(&kgath, index_axis).is_err());

    // ── batch stays FREE through the whole composite ─────────────────────────
    assert_eq!(structure(&out, batch_axis).level, Parallelism::Free);
}

/// The distributive laws the completeness probe forced into the algebra
/// (invariant reduction, lattice coupling, additive deferral, signed
/// defer-scale), each held to
/// `run_carrier == eval` on random data — including sign flips, which are
/// exactly where a wrong defer-scale would lie.
#[test]
fn probe_discovered_laws_are_sound() {
    use sanic::interp::{Env, Value, eval, run_carrier};

    fn coll(x: &NodeRef, op: Monoid) -> NodeRef {
        reduce(x.clone(), 0usize, op)
    }
    fn fold_broadcast(x: &NodeRef, value: NodeRef, op: Monoid) -> NodeRef {
        reduce(
            map(MapOp::Mul, vec![ones_like(x.clone()), value]),
            0usize,
            op,
        )
    }
    fn mul(a: NodeRef, b: NodeRef) -> NodeRef {
        map(MapOp::Mul, vec![a, b])
    }
    fn add(a: NodeRef, b: NodeRef) -> NodeRef {
        map(MapOp::Add, vec![a, b])
    }
    fn subn(a: NodeRef, b: NodeRef) -> NodeRef {
        map(MapOp::Sub, vec![a, b])
    }
    fn mx(a: NodeRef, b: NodeRef) -> NodeRef {
        map(MapOp::Max, vec![a, b])
    }
    fn mn(a: NodeRef, b: NodeRef) -> NodeRef {
        map(MapOp::Min, vec![a, b])
    }

    // Each program is a builder: the axis carries its extent, so every trial
    // length below mints its own `n` and rebuilds the graph around it.
    type Program = fn(NodeRef) -> NodeRef;
    let programs: Vec<(&str, Program)> = vec![
        // invariant reductions: Σ/max/min/lse over a same-axis collapsed value
        ("sum_of_sum", |x| {
            fold_broadcast(&x, coll(&x, add_r()), add_r())
        }),
        ("max_of_sum", |x| {
            fold_broadcast(&x, coll(&x, add_r()), max_r())
        }),
        ("lse_of_max", |x| {
            fold_broadcast(&x, coll(&x, max_r()), lse_r())
        }),
        ("sum_of_scaled_max", |x| {
            fold_broadcast(&x, mul(coll(&x, max_r()), konst(-1.5)), add_r())
        }),
        // lattice coupling: reduce_m(max/min(pe, coll)) for m ∈ {Max, Min}
        ("min_of_max_pe_coll", |x| {
            let coordinate = coordinate(x.clone(), 0usize);
            reduce(
                mx(subn(coordinate, x.clone()), coll(&x, add_r())),
                0usize,
                Monoid::Min,
            )
        }),
        ("max_of_max_coll_pe", |x| {
            reduce(mx(coll(&x, add_r()), x), 0usize, max_r())
        }),
        ("max_of_min_pe_coll", |x| {
            reduce(mn(x.clone(), coll(&x, add_r())), 0usize, max_r())
        }),
        ("min_of_min_coll_pe", |x| {
            reduce(mn(coll(&x, max_r()), x), 0usize, Monoid::Min)
        }),
        // additive deferral: pe ± coll under Max/Min/Add
        ("max_of_pe_plus_coll", |x| {
            reduce(
                add(coordinate(x.clone(), 0usize), coll(&x, max_r())),
                0usize,
                max_r(),
            )
        }),
        ("min_of_coll_minus_pe", |x| {
            reduce(subn(coll(&x, max_r()), x), 0usize, Monoid::Min)
        }),
        ("sum_of_pe_plus_coll", |x| {
            reduce(add(x.clone(), coll(&x, max_r())), 0usize, add_r())
        }),
        // signed defer-scale: extremum of coll·pe dispatches on the sign
        ("min_of_scaled_pe", |x| {
            reduce(
                mul(add(x.clone(), konst(0.5)), coll(&x, max_r())),
                0usize,
                Monoid::Min,
            )
        }),
        ("max_of_scaled_iota", |x| {
            reduce(
                mul(
                    mul(coordinate(x.clone(), 0usize), coll(&x, max_r())),
                    coll(&x, add_r()),
                ),
                0usize,
                max_r(),
            )
        }),
    ];

    let mut rng = Lcg::new(0xD15C0);
    for (name, build) in programs {
        for trial in 0..6 {
            let len = 3 + trial;
            let n = axis("n", len);
            let x = input("X", [n], Dtype::F32);
            let stream = axis_refs(&x)[0];
            let g = build(x);
            let c = derive(&g, stream).unwrap_or_else(|d| panic!("{name}: must derive, got {d}"));
            // mixed signs, planted ties: the adversarial cases for these laws
            let vals: Vec<f64> = (0..len)
                .map(|i| {
                    let v = (rng.next_f64() * 4.0 - 2.0) * 100.0;
                    (v.round() / 4.0) * if i % 3 == 2 { -1.0 } else { 1.0 }
                })
                .collect();
            let env: Env = [(
                "X",
                Value::from_shape_fn(&[len], |coordinate| vals[coordinate[0]]),
            )]
            .into_iter()
            .collect();
            let want = eval(&g, &env).data[0];
            let got = run_carrier(&g, stream, &c, &env).data[0];
            assert!(
                (want - got).abs() <= 1e-9 * (1.0 + want.abs()),
                "{name} (len {len}): eval={want} run_carrier={got}\nvals={vals:?}"
            );
        }
    }
}

// ── free-axis reindexing commutes with folding ───────────────────────────────
// A reindex acts only on the dimensions it transforms; a dimension it passes
// through 1:1 keeps its identity, so a coupled carrier composes through a
// broadcast along a FREE axis exactly as it does without one. The GQA head
// repeat is one instance; nothing here names it.
#[test]
fn coupled_carrier_composes_through_free_axis_repeat() {
    let (g, r, s) = (axis("g", 2), axis("r", 3), axis("s", 8));
    let grouped = axis("grouped", 6);

    let softmax_denominator = |x: NodeRef| {
        let rank = x.shape().len();
        let maximum = unsqueeze(reduce(x.clone(), rank - 1, Monoid::Max), rank - 1);
        reduce(
            map(MapOp::Exp, vec![map(MapOp::Sub, vec![x, maximum])]),
            rank - 1,
            Monoid::Add,
        )
    };
    let repeat_middle = |src: NodeRef| {
        let source = src.shape();
        let mut expanded = source.clone();
        expanded.insert(1, r);
        let map = source
            .iter()
            .enumerate()
            .map(|(dim, _)| (dim, vec![(1, if dim == 0 { 0 } else { dim + 1 })], 0))
            .collect();
        positional_reindex(src, expanded, map, false)
    };

    let base = input("x", [g, s], Dtype::F32);
    let stream = source_axis(&base, 1);
    for (label, node) in [
        ("no repeat", softmax_denominator(base.clone())),
        ("repeat", softmax_denominator(repeat_middle(base.clone()))),
        (
            "repeat + flatten",
            softmax_denominator(flatten(
                repeat_middle(base.clone()),
                &[0usize, 1usize][..],
                grouped,
            )),
        ),
    ] {
        let carrier = derive(&node, stream)
            .unwrap_or_else(|decline| panic!("{label} must derive, got: {decline}"));
        assert_eq!(carrier.kinds.len(), 2, "{label}: the (max, Σexp) tuple");
    }
}
