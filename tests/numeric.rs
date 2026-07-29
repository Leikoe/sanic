//! The number-system analysis, held to the interpreter.
//!
//! The contract under test: for a node inferred into an exact system, every
//! finite value the interpreter produces is an integer inside the inferred
//! bounds. That is the half of the analysis which licenses narrowing a
//! boundary, so it is the half that must not be merely plausible.
//!
//! Integrality is checked separately from containment. A range check passes
//! for 200.5 in `[0, 200]`, and 200.5 stored to bf16 is wrong — so containment
//! alone would leave the licensing claim untested.

use std::collections::HashMap;

use sanic::interp::{Env, Value, eval};
use sanic::ir::*;
use sanic::numeric::{Bounds, Inferred, NumberSystem, infer_root, may_store};

// A tiny deterministic PRNG, matching the convention in `tests/laws.rs`.
struct Lcg(u64);
impl Lcg {
    fn new(seed: u64) -> Self {
        Lcg(seed.max(1))
    }
    fn next_f64(&mut self) -> f64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        let u = x.wrapping_mul(0x2545F4914F6CDD1D);
        ((u >> 11) as f64 / (1u64 << 53) as f64) * 6.0 - 3.0
    }
}

/// Every finite value must sit inside the claim; every value of an exact
/// system must additionally be an integer.
fn assert_honest(node: &NodeRef, env: &Env, what: &str) {
    let claim = infer_root(node);
    for &value in &eval(node, env).data {
        if !value.is_finite() {
            assert!(
                !claim.bounds.is_finite(),
                "{what}: produced {value} but the bounds claim finiteness"
            );
            continue;
        }
        if claim.system.is_exact() {
            assert_eq!(
                value.fract(),
                0.0,
                "{what}: {value} is not an integer, but the system is {:?}",
                claim.system
            );
        }
        if let Some(lo) = claim.bounds.lo {
            assert!(value >= lo as f64, "{what}: {value} is below the claimed {lo}");
        }
        if let Some(hi) = claim.bounds.hi {
            assert!(value <= hi as f64, "{what}: {value} is above the claimed {hi}");
        }
    }
}

fn random(name: &'static str, extent: usize, seed: u64) -> (Env, Axis) {
    let dimension = axis(name, extent);
    let mut rng = Lcg::new(seed);
    let data = Value::from_shape_fn(&[extent], |_| rng.next_f64());
    let env: Env = [(name, data)].into_iter().collect();
    (env, dimension)
}

#[test]
fn variant_order_is_the_inclusion_law() {
    // The chain lives in the declaration order and is checked by nothing else.
    assert!(NumberSystem::Bool < NumberSystem::Natural);
    assert!(NumberSystem::Natural < NumberSystem::Integer);
    assert!(NumberSystem::Integer < NumberSystem::Real);
    assert_eq!(NumberSystem::Bool.join(NumberSystem::Integer), NumberSystem::Integer);
    assert!(NumberSystem::Integer.is_exact());
    assert!(!NumberSystem::Real.is_exact());
}

#[test]
fn coordinates_are_natural_and_bounded_by_the_extent() {
    let vocab = axis("vocab", 128_256);
    let claim = infer_root(&iota(vocab));
    assert_eq!(claim.system, NumberSystem::Natural);
    assert_eq!(claim.bounds, Bounds::range(0, 128_255));
    assert_honest(&iota(vocab), &HashMap::new(), "iota");
}

#[test]
fn comparison_is_a_predicate_whatever_its_operands() {
    let (env, x) = random("X", 16, 7);
    let scores = input("X", [x], Dtype::F32);
    let mask = map(MapOp::Lt, vec![scores.clone(), konst(0.0)]);
    let claim = infer_root(&mask);
    assert_eq!(claim.system, NumberSystem::Bool);
    assert_eq!(claim.bounds, Bounds::range(0, 1));
    assert_honest(&mask, &env, "lt");
}

#[test]
fn summing_predicates_leaves_bool() {
    // 1 + 1 = 2, so `Add` cannot floor at 𝔹. The mask-summing idiom is live
    // in `grad.rs`, which folds a 0/1 winner mask to count ties.
    let (env, x) = random("X", 16, 11);
    let scores = input("X", [x], Dtype::F32);
    let mask = map(MapOp::Lt, vec![scores.clone(), konst(0.0)]);
    let two = map(MapOp::Add, vec![mask.clone(), mask.clone()]);
    let claim = infer_root(&two);
    assert_eq!(claim.system, NumberSystem::Natural);
    assert_eq!(claim.bounds, Bounds::range(0, 2));
    assert_honest(&two, &env, "lt + lt");
}

#[test]
fn subtraction_leaves_the_naturals() {
    let n = axis("n", 8);
    let difference = map(MapOp::Sub, vec![iota(n), iota(n)]);
    let claim = infer_root(&difference);
    assert_eq!(claim.system, NumberSystem::Integer);
    assert_eq!(claim.bounds, Bounds::range(-7, 7));
    assert_honest(&difference, &HashMap::new(), "iota - iota");
}

#[test]
fn division_leaves_the_exact_systems() {
    let n = axis("n", 8);
    let ratio = map(MapOp::Div, vec![iota(n), konst(3.0)]);
    assert_eq!(infer_root(&ratio).system, NumberSystem::Real);
}

#[test]
fn an_infinite_constant_joins_as_the_bottom_system() {
    // ±∞ is an identity element, not a real: it belongs to every extended
    // system. Classifying it from its value would send argmax's `Where` to ℝ
    // and lose the exactness this analysis exists to find.
    let claim = infer_root(&konst(f64::INFINITY));
    assert_eq!(claim.system, NumberSystem::Bool);
    assert!(!claim.bounds.is_finite());
}

#[test]
fn the_argmax_sentinel_does_not_destroy_exactness() {
    // The shape `first_index_of_maximum` builds: a coordinate guarded by a
    // comparison, with +∞ on the rejected branch.
    let vocab = axis("vocab", 128_256);
    let scores = input("X", [vocab], Dtype::F32);
    let maximum = reduce(scores.clone(), 0usize, Monoid::Max);
    let guarded = map(
        MapOp::Where,
        vec![
            map(MapOp::Lt, vec![scores.clone(), unsqueeze(maximum, 0usize)]),
            konst(f64::INFINITY),
            coordinate(scores.clone(), 0usize),
        ],
    );
    let claim = infer_root(&guarded);
    assert_eq!(
        claim.system,
        NumberSystem::Natural,
        "the sentinel must not drag the index into the reals"
    );
    assert_eq!(claim.bounds.lo, Some(0));
    assert_eq!(
        claim.bounds.hi,
        Some(128_255),
        "the sentinel must not erase the upper bound"
    );
}

#[test]
fn argmax_is_an_index_that_bf16_cannot_hold() {
    // The motivating defect, as a property of the analysis rather than of a
    // GPU: a vocabulary index needs an exact representation, and bf16 is
    // exact only to 256.
    let vocab = axis("vocab", 128_256);
    let scores = input("X", [vocab], Dtype::F32);
    let index = argmax(scores, 0usize);
    let claim = infer_root(&index);

    assert!(claim.system.is_exact(), "an index is not a real");
    assert!(
        !may_store(claim, Dtype::BF16),
        "bf16 holds integers only to 256; this is the defect"
    );
    assert!(!may_store(claim, Dtype::F16), "f16 holds integers only to 2048");
    assert!(
        may_store(claim, Dtype::F32),
        "f32 holds this index exactly, and ±∞ besides"
    );
}

#[test]
fn a_small_index_fits_a_narrow_float() {
    // Narrowing is not forbidden for exact values, only unsound ones — a
    // 200-element axis indexes fine in bf16.
    let small = axis("small", 200);
    let scores = input("X", [small], Dtype::F32);
    let claim = infer_root(&argmax(scores, 0usize));
    assert!(may_store(claim, Dtype::BF16));
}

#[test]
fn integer_formats_are_refused_when_a_value_can_be_infinite() {
    // An order fold injects its identity, so the result's range carries ±∞ —
    // and no integer representation holds it.
    let vocab = axis("vocab", 1024);
    let scores = input("X", [vocab], Dtype::F32);
    let claim = infer_root(&argmax(scores, 0usize));
    assert!(!claim.bounds.is_finite());
    assert!(!may_store(claim, Dtype::I8));
    assert!(may_store(claim, Dtype::F32));
}

#[test]
fn a_sum_fold_grows_the_range_by_the_extent() {
    let n = axis("n", 8);
    let total = reduce(iota(n), 0usize, Monoid::Add);
    let claim = infer_root(&total);
    assert_eq!(claim.system, NumberSystem::Natural);
    // Conservative: 8 × [0, 7]. The true maximum is 28.
    assert_eq!(claim.bounds, Bounds::range(0, 56));
    assert_honest(&total, &HashMap::new(), "sum of iota");
}

#[test]
fn bounds_do_not_wrap_on_overflow() {
    // `i64::MIN.abs()` panics in debug and wraps in release, which would
    // certify the widest possible range as exact. The check is written
    // without negation for exactly this reason.
    let widest = Bounds {
        lo: Some(i64::MIN),
        hi: Some(0),
        infinite: false,
    };
    assert!(!widest.within(1 << 24));
    let exact = Inferred {
        system: NumberSystem::Natural,
        bounds: widest,
    };
    assert!(!may_store(exact, Dtype::F32));
}

#[test]
fn exact_integer_ranges_match_the_formats() {
    assert_eq!(Dtype::BF16.exact_integers_to(), 256);
    assert_eq!(Dtype::F16.exact_integers_to(), 2048);
    assert_eq!(Dtype::F32.exact_integers_to(), 1 << 24);
    assert_eq!(Dtype::F64.exact_integers_to(), 1 << 53);
    // A signed integer stops one short of its magnitude range: i8 reaches
    // 127, not 128. Deriving every property from one layout is what keeps
    // this from drifting away from `bytes()`.
    assert_eq!(Dtype::I8.exact_integers_to(), 127);
    assert_eq!(Dtype::I4.exact_integers_to(), 7);

    assert_eq!(Dtype::F64.nbits(), 64);
    assert_eq!(Dtype::BF16.nbits(), 16);
    assert_eq!(Dtype::I4.nbits(), 4);
    assert!(Dtype::BF16.is_float() && Dtype::BF16.has_infinities());
    assert!(!Dtype::I8.is_float() && !Dtype::I8.has_infinities());
    assert!(Dtype::I4.is_subbyte() && !Dtype::I8.is_subbyte());
}

#[test]
fn sub_byte_sizes_round_up_rather_than_truncate() {
    // Three int4 values need two bytes. A per-element width of 0.5 gives
    // 1.5, and `as usize` makes that 1 — which is why sizes are computed in
    // bits and `bytes()` is documented as a pricing weight, not a size.
    assert_eq!(Dtype::I4.nbytes(3), 2);
    assert_eq!(Dtype::I4.nbytes(4), 2);
    assert_eq!(Dtype::I4.nbytes(5), 3);
    assert_eq!(Dtype::I4.nbytes(0), 0);
    assert_eq!(Dtype::I8.nbytes(3), 3);
    assert_eq!(Dtype::F32.nbytes(3), 12);
    assert_eq!(Dtype::BF16.nbytes(3), 6);
}

#[test]
fn a_float_input_is_a_real() {
    let n = axis("n", 4);
    assert_eq!(infer_root(&input("W", [n], Dtype::BF16)).system, NumberSystem::Real);
}

// ── the refusal: M11's second half, on the compiled program ─────────────────
//
// The done-condition asserts on the SCHEDULE, not on a computed value —
// CI's macOS runners have no GPU, and the defect is a property of the
// compilation either way.

#[test]
fn the_law_mints_the_width_the_policy_cannot_supply() {
    use sanic::cost::DeviceProfile;
    use sanic::partition::partition;

    let vocab = axis("vocab", 128_256);
    let scores = input("X", [vocab], Dtype::F32);
    let index = argmax(scores, 0usize);

    // Partitioned FOR a bf16 boundary policy: the argmax output is an exact
    // boundary the policy cannot carry, so the law mints f32 for that one
    // buffer — no refusal, no manual pin, and every other buffer keeps the
    // policy. This is the milestone: the defect that opened the arc is now
    // closed by construction rather than refused.
    let narrow = DeviceProfile::toy().with_storage(Dtype::BF16);
    let schedule = partition(&index, &narrow);

    assert!(!schedule.exact_boundaries.is_empty(), "the fact is recorded");
    assert!(schedule.unstorable(Dtype::BF16).is_empty(), "minted, not refused");
    let out = schedule.outputs[0].clone();
    assert_eq!(schedule.minted_dtypes.get(&out).copied(), Some(Dtype::F32));
    assert_eq!(schedule.width_of(&out, Dtype::BF16), Dtype::F32);

    // A pin outranks the mint — the caller is the Caller row — but only a
    // LAWFUL pin: width_of resolves pin first, and unstorable() judges it.
    let mut pinned = schedule;
    pinned.output_dtypes = vec![Some(Dtype::F16)];
    assert_eq!(pinned.width_of(&out, Dtype::BF16), Dtype::F16);
    assert!(!pinned.unstorable(Dtype::BF16).is_empty(), "128255 does not fit f16");
}

#[test]
fn an_unmintable_exact_boundary_is_still_refused() {
    use sanic::cost::DeviceProfile;
    use sanic::partition::partition;

    // A product fold over coordinates saturates its bounds — no writable
    // dtype can promise to carry it, so the law has nothing to mint and the
    // program is refused rather than guessed at. "Decline, don't guess"
    // survives minting.
    let n = axis("n", 64);
    let product = reduce(iota(n), 0usize, Monoid::Mul);

    let schedule = partition(&product, &DeviceProfile::toy().with_storage(Dtype::BF16));
    assert!(!schedule.exact_boundaries.is_empty());
    assert!(schedule.minted_dtypes.is_empty(), "nothing lawful to mint");
    let refused = schedule.unstorable(Dtype::BF16);
    assert!(!refused.is_empty(), "an unboundable exact value has no lawful width");
    assert!(schedule.outputs.contains(&refused[0].0));
}

#[test]
fn a_pinned_output_is_judged_against_its_pin() {
    use sanic::cost::DeviceProfile;
    use sanic::partition::partition;

    // The reconciliation with `output_at` (#21): the caller pinning an
    // argmax output to f32 is the LAWFUL choice, and the refusal must
    // respect it even under a bf16 boundary policy — pins are the Caller
    // row of the who-chooses table, the policy is only the default.
    let vocab = axis("vocab", 128_256);
    let scores = input("X", [vocab], Dtype::F32);
    let index = argmax(scores, 0usize);

    let mut schedule = partition(&index, &DeviceProfile::toy());
    schedule.output_dtypes = vec![Some(Dtype::F32)];
    assert!(
        schedule.unstorable(Dtype::BF16).is_empty(),
        "a pin wider than the policy satisfies the law"
    );

    // And the law still catches a pin that is itself too narrow.
    schedule.output_dtypes = vec![Some(Dtype::F16)];
    assert!(
        !schedule.unstorable(Dtype::F32).is_empty(),
        "an unlawful pin is caught even under a lawful default"
    );
}

#[test]
fn real_valued_boundaries_are_untouched_by_the_refusal() {
    use sanic::cost::DeviceProfile;
    use sanic::partition::partition;

    // llama's shape: logits are reals, and the 16% lives on bf16 staying
    // permitted for them. Reals are not even recorded as exact boundaries —
    // every writable float carries them, so they can never fail the law.
    let vocab = axis("vocab", 128_256);
    let hidden = axis("hidden", 64);
    let x = input("X", [hidden], Dtype::F32);
    let w = input("W", [vocab, hidden], Dtype::BF16);
    let logits = reduce(map(MapOp::Mul, vec![unsqueeze(x, 0usize), w]), 1usize, Monoid::Add);

    let schedule = partition(&logits, &DeviceProfile::toy());
    assert!(schedule.exact_boundaries.is_empty());
    assert!(schedule.unstorable(Dtype::BF16).is_empty());
}
