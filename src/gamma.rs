//! Γ — everything decided about the names of one program, sealed.
//!
//! The design's one-line thesis (`number_systems_and_representations.md`):
//! *the expression is what's true; the context is what's decided.* This is
//! the context. A term (`ir::Node`) carries no storage; every width lives
//! here, entered by exactly three deciders and read through exactly one
//! resolution:
//!
//! ```text
//! width_of(name) = pin  →  mint  →  declaration  →  policy default
//!                (Caller,  Law,     Caller,          Policy)
//! ```
//!
//! The fields are private on purpose. This type began as a *model* in
//! `tests/contract.rs` — a spec written before the implementation existed —
//! and the implementation first shipped as public maps on `Schedule`, whose
//! invariants ("one entry per name, minted once, never overwritten") held by
//! convention: any caller could clobber a mint or replace a pin. Promoting
//! the model into source is rule 4 of the contract file completing: the
//! clauses that were promises are now privacy.
//!
//! What deliberately did NOT cross from the model: the proof-token API
//! ("a width cannot be asked for before it is assigned") — the design has a
//! legitimate default row, so an undeclared name resolving to the policy is
//! truth, not a fallback bug — and the invariant-lifetime brand, which is
//! more machinery than one `Bindings` per compilation needs.

use std::collections::HashMap;

use crate::ir::Dtype;
use crate::numeric::{self, Inferred};

/// Γ for one compilation. Constructed from the caller's declarations,
/// grown append-only by the law during partition, pinned once by the
/// program's outputs after it.
#[derive(Debug, Default, Clone)]
pub struct Bindings {
    /// Caller row, inputs: name → the storage its bound buffer uses.
    declared: HashMap<String, Dtype>,
    /// Law row: produced buffers whose exact value needs more than the
    /// policy supplies, at the narrowest dtype that carries it.
    minted: HashMap<String, Dtype>,
    /// Caller row, outputs: pins, keyed by name once validated.
    pins: HashMap<String, Dtype>,
    /// The analysis' facts: every exact-valued boundary, with its claim.
    /// Facts, not verdicts — [`Bindings::unstorable`] applies the law at
    /// the effective widths.
    exact: Vec<(String, Inferred)>,
}

impl Bindings {
    /// Γ seeded with the caller's input declarations.
    pub(crate) fn with_declared(declared: HashMap<String, Dtype>) -> Bindings {
        Bindings {
            declared,
            ..Bindings::default()
        }
    }

    /// The law, at the moment a buffer is named (partition-only). Records
    /// the claim as a fact when it is exact, and mints a width when the
    /// policy cannot carry it. Append-only: a name is observed once, when
    /// it is born — observing it again is a defect in the *observer*.
    pub(crate) fn observe(&mut self, name: &str, claim: Inferred, policy: Dtype) {
        if !claim.system.is_exact() {
            return;
        }
        assert!(
            !self.exact.iter().any(|(existing, _)| existing == name),
            "`{name}` observed twice: Γ is append-only, and a buffer is born once"
        );
        self.exact.push((name.to_string(), claim));
        if let Some(width) = numeric::store_dtype(claim, policy)
            && width != policy
        {
            self.minted.insert(name.to_string(), width);
        }
    }

    /// The caller's output pins, once. Re-pinning is refused: the second
    /// decision for a name is a defect in the decider, never a replacement.
    pub(crate) fn pin(&mut self, outputs: &[String], pins: Vec<Option<Dtype>>) {
        assert_eq!(pins.len(), outputs.len(), "one storage width per root");
        for (name, pin) in outputs.iter().zip(pins) {
            if let Some(dtype) = pin {
                assert!(!self.pins.contains_key(name), "`{name}` pinned twice: Γ is append-only");
                self.pins.insert(name.clone(), dtype);
            }
        }
    }

    /// A buffer's identity moved (partition's epilogue absorption renames
    /// a fold's output): the fact and any mint travel with the name, or
    /// writer and reader would disagree about the very width the mint
    /// exists to protect.
    pub(crate) fn rename(&mut self, from: &str, to: &str) {
        if let Some(width) = self.minted.remove(from) {
            self.minted.insert(to.to_string(), width);
        }
        for (name, _) in self.exact.iter_mut() {
            if name == from {
                *name = to.to_string();
            }
        }
    }

    /// The one resolution of a buffer's storage width — every consumer of
    /// the answer (emission, allocation, readback, pricing, the refusal)
    /// asks here, which is what makes their disagreement inexpressible.
    pub fn width_of(&self, name: &str, default: Dtype) -> Dtype {
        self.pins
            .get(name)
            .or_else(|| self.minted.get(name))
            .or_else(|| self.declared.get(name))
            .copied()
            .unwrap_or(default)
    }

    /// The law's choice for `name`, if it made one.
    pub fn minted(&self, name: &str) -> Option<Dtype> {
        self.minted.get(name).copied()
    }

    /// The caller's declaration for `name`, if any.
    pub fn declared(&self, name: &str) -> Option<Dtype> {
        self.declared.get(name).copied()
    }

    /// A mint's or declaration's width, for pricing — `None` means the name
    /// takes a default the caller of this method owns.
    pub(crate) fn width_hint(&self, name: &str) -> Option<Dtype> {
        self.minted.get(name).or_else(|| self.declared.get(name)).copied()
    }

    /// The declarations, cloned — emission's resolved-Γ base.
    pub fn snapshot_declared(&self) -> HashMap<String, Dtype> {
        self.declared.clone()
    }

    /// Every exact-valued boundary and its claim.
    pub fn exact_boundaries(&self) -> &[(String, Inferred)] {
        &self.exact
    }

    /// The law, applied at each boundary's EFFECTIVE width. Fires for
    /// exactly two things: an exact value no writable dtype carries
    /// (saturated bounds), and a pin narrower than the law allows. Empty
    /// means the program is storable as configured.
    pub fn unstorable(&self, default: Dtype) -> Vec<(String, Inferred, Dtype)> {
        self.exact
            .iter()
            .filter_map(|(name, claim)| {
                let width = self.width_of(name, default);
                (!numeric::may_store(*claim, width)).then(|| (name.clone(), *claim, width))
            })
            .collect()
    }
}
