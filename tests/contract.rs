//! The Γ contract — witnesses against the REAL types.
//!
//! # The habit, and how this file kept its own rule 4
//!
//! A contract is a set of types whose *signatures* are the promises. This
//! file began as a MODEL: `Gamma`, `Bound`, `Binding` and `store_dtype`
//! written here first, as a spec, before any of it existed in `src/` — and
//! its rules said what must happen next: *"src/ converges to the contract,
//! milestone by milestone… this file shrinks to re-exports and witnesses."*
//!
//! It did, in stages. `store_dtype` moved into `numeric` when M12a's minting
//! landed. `Policy` became real in `cost` at M14. `Var` was fulfilled by
//! M12b — `ir::Node::Input` took exactly its shape. And finally the model
//! `Gamma` itself crossed: `src/gamma.rs::Bindings` is the sealed context,
//! its append-only and law-only clauses enforced by privacy where this file
//! could only enforce them in a toy. The toy is deleted. What remains is
//! what rule 4 promised: witnesses.
//!
//! What deliberately did not cross, recorded so nobody re-derives it: the
//! proof-token API (the design has a legitimate policy-default row, so an
//! unresolved name is an answer, not an error) and the invariant-lifetime
//! brand (more machinery than one `Bindings` per compilation needs).

use sanic::cost::{DeviceSpecs, Policy};
use sanic::ir::*;
use sanic::numeric::{Bounds, Inferred, NumberSystem, infer_root, may_store, store_dtype};
use sanic::partition::partition;

/// A free variable of the calculus: a name and a shape. **There is no dtype
/// field, and that absence is the contract** — representation is a fact
/// *about* a name, decided in Γ, never a part of the term.
///
/// FULFILLED by M12b: `ir::Node::Input` took exactly this shape,
/// `shallow_key` lost its dtype component (CSE identity is pure structure),
/// and the structure ledger holds `ir.rs` at zero storage mentions. This
/// mirror stays as the record of what was promised before it was true.
#[allow(dead_code)]
pub struct Var {
    pub name: &'static str,
    pub shape: Vec<Axis>,
}

// ── witnesses: the law as a choice ──────────────────────────────────────────

/// `store_dtype` returns the narrowest sufficient width, never a jump to
/// f32 — and a lawful preference stands.
#[test]
fn the_law_chooses_the_narrowest_sufficient_width() {
    let small = Inferred {
        system: NumberSystem::Natural,
        bounds: Bounds::range(0, 1000),
    };
    assert_eq!(store_dtype(small, Dtype::BF16), Some(Dtype::F16));

    let tiny = Inferred {
        system: NumberSystem::Natural,
        bounds: Bounds::range(0, 200),
    };
    assert_eq!(
        store_dtype(tiny, Dtype::BF16),
        Some(Dtype::BF16),
        "lawful preferences stand"
    );
}

/// The ℝ̄ half is unpoliced: an approximate value takes the preference
/// unchanged. This is the clause that protects the measured bf16 win.
#[test]
fn an_undeclared_real_takes_the_policy_preference() {
    let real = Inferred {
        system: NumberSystem::Real,
        bounds: Bounds::UNBOUNDED,
    };
    assert_eq!(store_dtype(real, Dtype::BF16), Some(Dtype::BF16));
}

// ── witnesses: Γ is sealed ──────────────────────────────────────────────────

/// The whole arc in one flow, on the real types: the analysis claims, the
/// law mints u32 for the claim the policy cannot carry, a caller's pin
/// outranks the mint, and the resolution chain reports each stage.
#[test]
fn the_width_chain_resolves_pin_over_mint_over_policy() {
    let vocab = axis("vocab", 128_256);
    let index = argmax(input("X", [vocab]), 0usize);

    let narrow = DeviceSpecs::toy().under(Policy { boundary: Dtype::BF16 });
    let mut schedule = partition(&index, &narrow);
    let out = schedule.outputs[0].clone();

    assert_eq!(schedule.bindings().minted(&out), Some(Dtype::U32), "the law's row");
    assert_eq!(schedule.width_of(&out, Dtype::BF16), Dtype::U32);

    schedule.pin_outputs(vec![Some(Dtype::F32)]);
    assert_eq!(
        schedule.width_of(&out, Dtype::BF16),
        Dtype::F32,
        "the caller's row outranks"
    );
    assert!(schedule.unstorable(Dtype::BF16).is_empty());
}

/// Γ is append-only: re-pinning a name is refused loudly. Before the
/// promotion this was a public field any caller could reassign — the toy
/// enforced what the source did not. Now the source refuses.
#[test]
#[should_panic(expected = "pinned twice")]
fn a_pin_is_decided_exactly_once() {
    let vocab = axis("vocab", 1024);
    let index = argmax(input("X", [vocab]), 0usize);
    let mut schedule = partition(&index, &DeviceSpecs::toy());
    schedule.pin_outputs(vec![Some(Dtype::F32)]);
    schedule.pin_outputs(vec![Some(Dtype::F16)]);
}

/// One storage width per root — the arity is validated where the pins
/// enter, not where they are read.
#[test]
#[should_panic(expected = "one storage width per root")]
fn a_pin_vector_must_match_the_roots() {
    let n = axis("n", 8);
    let mut schedule = partition(&reduce(input("X", [n]), 0usize, Monoid::Add), &DeviceSpecs::toy());
    schedule.pin_outputs(vec![Some(Dtype::F32), Some(Dtype::F32)]);
}

/// A declaration is append-only from the very first entry: redeclaring an
/// input at a different width is refused at the declaration site. (The
/// promotion found this hole — `Graph::input` silently last-wins-overwrote
/// before Γ was sealed.)
#[test]
#[should_panic(expected = "declared twice at different widths")]
fn an_input_declares_one_width() {
    let mut graph = sanic::Graph::new();
    let d = axis("d", 4);
    graph.input("w", [d], Dtype::BF16);
    graph.input("w", [d], Dtype::F16);
}

/// Redeclaring at the SAME width is the same declaration — a shared weight
/// mentioned twice is one binding, exactly as two mentions of a variable
/// are one node.
#[test]
fn redeclaring_the_same_width_is_the_same_declaration() {
    let mut graph = sanic::Graph::new();
    let d = axis("d", 4);
    let first = graph.input("w", [d], Dtype::BF16);
    let second = graph.input("w", [d], Dtype::BF16);
    assert!(std::sync::Arc::ptr_eq(first.node(), second.node()));
}

/// The residual obligations no type carries, held elsewhere: integrality is
/// tested against the interpreter (`tests/numeric.rs`, `fract() == 0`), and
/// `may_store` never trusts an annotation it can check — a bogus exactness
/// claim with no bounds behind it fails closed.
#[test]
fn an_unbounded_exactness_claim_fails_closed() {
    let bogus = Inferred {
        system: NumberSystem::Natural,
        bounds: Bounds::UNBOUNDED,
    };
    assert!(!may_store(bogus, Dtype::F32));
    let real_argmax = infer_root(&argmax(input("X", [axis("v", 100)]), 0usize));
    assert!(may_store(real_argmax, Dtype::F32));
}
