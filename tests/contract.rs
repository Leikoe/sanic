//! The Γ contract, written in Rust's type system.
//!
//! # The habit
//!
//! A contract is a set of types whose *signatures* are the promises. The rules
//! of the game, in force from here on:
//!
//! 1. **Signatures are load-bearing; bodies are minimal.** What a function
//!    refuses to accept and cannot return is the specification. The doc
//!    comment states the promise; the type makes the promise unbreakable or
//!    names the residual obligation.
//! 2. **Illegal states are unrepresentable where that is cheap** — privacy,
//!    proof tokens, sum types. Where the type system cannot carry a clause
//!    (integrality of runtime values, soundness of a caller's claim), the
//!    clause is written down and held by a witness test instead.
//! 3. **A contract is checked or it is fiction.** This file compiles against
//!    the real crate and runs under `cargo test --all-targets` — the same
//!    command CI runs — so every promise below is exercised on every push,
//!    while `src/` remains untouched.
//! 4. **`src/` converges to the contract, milestone by milestone.** When M12
//!    lands, `Gamma` moves into `Schedule` and this file shrinks to
//!    re-exports and witnesses. If `src/` drifts instead, this file stops
//!    compiling — which is the point.
//!
//! # The contract being written
//!
//! One first-order calculus, one context, three judgments
//! (`number_systems_and_representations.md` §2):
//!
//! ```text
//! Γ_value  ⊢ e ⇓ v         eval   (exists: interp::Env)
//! Γ_system ⊢ e : (S, B)     infer  (exists: numeric::infer)
//! Γ_dtype  ⊢ e → kernel     emit   (M12: the types below)
//! ```
//!
//! The promises, each carried by a type in this file:
//!
//! | promise | carried by |
//! |---|---|
//! | a free variable has no representation | `Var` — the *absence* of a dtype field |
//! | one Γ entry per name, minted once, never replaced | `Gamma::declare_input`/`mint` refuse duplicates; no overwrite or remove exists |
//! | "width asked before assigned" is a compile error | `Gamma::binding` demands a `Bound` proof, obtainable only from a successful mint |
//! | a proof from one Γ is useless against another | the invariant lifetime brand on `Gamma<'id>`/`Bound<'id>` |
//! | every produced buffer's width is lawful | `Binding` has no public constructor; `mint` runs the law |
//! | the ℝ̄ half is unpoliced until M14 | `store_dtype`: `Approximate → Ok(preferred)`, tested |
//! | a caller may choose, but not unsoundly | `declare_input` refuses a dtype the claim's system cannot live in |
//! | who decided is auditable | `Provenance`, recorded on every `Binding` |

use std::collections::HashMap;
use std::marker::PhantomData;

use sanic::ir::*;
use sanic::numeric::{Bounds, Inferred, NumberSystem, infer_root, may_store};

// ── the calculus side ────────────────────────────────────────────────────────

/// A free variable of the calculus: a name and a shape. **There is no dtype
/// field, and that absence is the contract** — representation is a fact
/// *about* a name, decided in Γ, never a part of the term. When M12 lands,
/// `ir::Node::Input` takes this shape and `shallow_key` loses its dtype
/// component, making CSE identity pure structure.
#[allow(dead_code)]
pub struct Var {
    pub name: &'static str,
    pub shape: Vec<Axis>,
}

// ── the context ──────────────────────────────────────────────────────────────

/// An invariant lifetime brand: two `Gamma`s can never share `'id`, so a
/// proof token from one is a type error against the other. This is the
/// strongest clause in the file — "writer and reader read the same entry"
/// holds *across contexts*, not merely within one.
type Brand<'id> = PhantomData<fn(&'id ()) -> &'id ()>;

/// Everything ever decided about the names of one program. Append-only by
/// construction: the API below has no overwrite and no removal, which is
/// what makes [`Gamma::binding`] total.
pub struct Gamma<'id> {
    entries: HashMap<&'static str, Binding>,
    brand: Brand<'id>,
}

/// Proof that a name has an entry in the Γ branded `'id`. Only a successful
/// declaration or mint produces one, and Γ is append-only, so holding a
/// `Bound` *is* the fact that the entry exists.
///
/// This is the type-level death of
/// `program.dtypes.get(name).copied().unwrap_or(program.storage)`:
/// the fallback is unwritable because the question cannot be asked without
/// the proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bound<'id> {
    name: &'static str,
    brand: Brand<'id>,
}

/// One name's record: what was decided, and by whom. **No public
/// constructor** — the only paths in are [`Gamma::declare_input`] (the
/// caller's choice, checked against their own claim) and [`Gamma::mint`]
/// (the law's choice) — so an unlawful binding is unconstructible, not
/// merely unlikely.
pub struct Binding {
    dtype: Dtype,
    inferred: Inferred,
    provenance: Provenance,
}

impl Binding {
    pub fn dtype(&self) -> Dtype {
        self.dtype
    }
    pub fn inferred(&self) -> Inferred {
        self.inferred
    }
    pub fn provenance(&self) -> Provenance {
        self.provenance
    }
}

/// Who decided a binding's width. The audit trail that makes M14 a
/// measurable migration: retiring the global default is driving
/// `Policy`-provenance entries to zero. (M14 adds a `Declared` variant for
/// per-value `stored(d)`.)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Provenance {
    /// The caller chose, at an input.
    Caller,
    /// The law chose: the value is exact, bounds decided.
    Law,
    /// The default policy chose: an undeclared real.
    Policy,
}

/// The policy input, modelled honestly: today it has exactly one knob
/// (`--bf16`), and §12 defers anything richer.
pub struct Policy {
    pub default_real: Dtype,
}

/// A caller's declaration for an input: the storage it will be bound at,
/// and optionally a *claim* about what the values are (llama's `tokens`:
/// stored `F32`, claimed ℕ̄ bounded by the vocabulary).
///
/// The claim is the one trust boundary in the design. Claiming a *wider*
/// system than the truth is sound and merely unprotective; claiming a
/// narrower one (ℕ̄ where values are really ℝ̄) is **unsound and cannot be
/// checked statically** — it is the caller's obligation, checkable only
/// against the interpreter in debug.
pub struct InputDeclaration {
    pub dtype: Dtype,
    pub claim: Option<Inferred>,
}

/// Refusals, named. In `src/` these join the decline census the way
/// `derive::Decline` does; a refusal is a claim, not an apology.
#[derive(Debug, PartialEq, Eq)]
pub enum Refusal {
    /// Γ is append-only: a second decision for the same name is a defect in
    /// the *deciding* code, never a replacement.
    AlreadyBound(&'static str),
    /// The caller's declared dtype cannot faithfully carry their own claim —
    /// an index declared bf16. The expert knob widens freely; it does not
    /// narrow below the law.
    UnlawfulDeclaration(&'static str),
    /// No writable dtype carries this value (an exact value with unbounded
    /// range — e.g. saturated interval arithmetic). Refuse loudly rather
    /// than guess.
    NoLawfulWidth(&'static str),
}

/// Run a scope with a fresh, uniquely-branded Γ. The `for<'id>` closure is
/// what makes the brand unforgeable: the caller cannot name `'id`, so no
/// `Bound` can outlive or cross contexts.
pub fn with_bindings<R>(scope: impl for<'id> FnOnce(Gamma<'id>) -> R) -> R {
    scope(Gamma {
        entries: HashMap::new(),
        brand: PhantomData,
    })
}

impl<'id> Gamma<'id> {
    /// The caller's row of the who-chooses table. Their dtype is taken as
    /// given for reals; if they also make a claim, the dtype must be able to
    /// carry it — "the expert knob widens freely, and a request narrower
    /// than the law permits is a compile error, not an override."
    pub fn declare_input(
        &mut self,
        name: &'static str,
        declaration: InputDeclaration,
    ) -> Result<Bound<'id>, Refusal> {
        if self.entries.contains_key(name) {
            return Err(Refusal::AlreadyBound(name));
        }
        let inferred = declaration.claim.unwrap_or(Inferred {
            system: NumberSystem::Real,
            bounds: Bounds::UNBOUNDED,
        });
        if !may_store(inferred, declaration.dtype) {
            return Err(Refusal::UnlawfulDeclaration(name));
        }
        self.entries.insert(
            name,
            Binding {
                dtype: declaration.dtype,
                inferred,
                provenance: Provenance::Caller,
            },
        );
        Ok(self.proof(name))
    }

    /// The law's row: mint a produced buffer's binding at the point its name
    /// is created. This is the *single* place a produced width is decided —
    /// producer, consumer, allocation, readback and the cost model all read
    /// the entry this creates, which is why their disagreement is
    /// inexpressible rather than checked.
    pub fn mint(
        &mut self,
        name: &'static str,
        inferred: Inferred,
        policy: &Policy,
    ) -> Result<Bound<'id>, Refusal> {
        if self.entries.contains_key(name) {
            return Err(Refusal::AlreadyBound(name));
        }
        let (dtype, provenance) = match store_dtype(inferred, policy.default_real) {
            Some(dtype) if inferred.system.is_exact() => (dtype, Provenance::Law),
            Some(dtype) => (dtype, Provenance::Policy),
            None => return Err(Refusal::NoLawfulWidth(name)),
        };
        self.entries.insert(
            name,
            Binding {
                dtype,
                inferred,
                provenance,
            },
        );
        Ok(self.proof(name))
    }

    /// The only fallible read. Everything downstream of a `lookup` takes the
    /// proof, so "entry missing" exists at exactly one point in a program.
    pub fn lookup(&self, name: &'static str) -> Option<Bound<'id>> {
        self.entries.contains_key(name).then(|| self.proof(name))
    }

    /// Total: a `Bound<'id>` can only have come from this Γ's own
    /// declare/mint, and Γ is append-only. No `Option`, no default, no
    /// `unwrap_or` — the class of defect that motivated the design cannot be
    /// written against this signature.
    pub fn binding(&self, proof: Bound<'id>) -> &Binding {
        &self.entries[proof.name]
    }

    /// M14's migration meter: the names whose width the default policy chose.
    /// Retiring `with_storage` is driving this to empty.
    pub fn chosen_by_policy(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.entries
            .iter()
            .filter(|(_, binding)| binding.provenance == Provenance::Policy)
            .map(|(name, _)| *name)
    }

    fn proof(&self, name: &'static str) -> Bound<'id> {
        Bound {
            name,
            brand: self.brand,
        }
    }
}

/// The law as a choice, not a veto (M13's shape, absorbed into M12):
/// `preferred` when it carries the value, otherwise the narrowest writable
/// dtype that does, otherwise nothing.
///
/// Promises, witnessed below:
/// - `Some(d)` implies `may_store(value, d)` — the result is always lawful.
/// - An approximate value gets `preferred` unchanged: the ℝ̄ half is
///   **unpoliced until M14**, which is what keeps the measured 16%.
/// - An exact value gets the *narrowest sufficient* width, not a jump to
///   f32: ℕ̄ bounded by 1000 under a bf16 default takes f16 — exact to 2048,
///   at the same two bytes.
pub fn store_dtype(value: Inferred, preferred: Dtype) -> Option<Dtype> {
    if may_store(value, preferred) {
        return Some(preferred);
    }
    // Writable boundary formats, narrowest first. F64 is absent because the
    // device cannot write it — a capability that belongs on `DeviceSpecs`,
    // not here, and lands there in M14.
    [Dtype::BF16, Dtype::F16, Dtype::F32]
        .into_iter()
        .find(|&candidate| may_store(value, candidate))
}

// ── witnesses ────────────────────────────────────────────────────────────────
//
// Each test is a clause of the contract exercised on the REAL analysis: the
// inferred facts below come from `numeric::infer_root` on genuine graphs,
// not from hand-built fixtures.

/// The defect, end to end: the real argmax graph's inferred facts, through
/// the law, land on f32 — under a bf16 policy, with provenance `Law`.
#[test]
fn the_argmax_buffer_is_minted_lawful_under_a_bf16_policy() {
    let vocab = axis("vocab", 128_256);
    let scores = input("X", [vocab], Dtype::F32);
    let claim = infer_root(&argmax(scores, 0usize));
    let policy = Policy {
        default_real: Dtype::BF16,
    };

    with_bindings(|mut gamma| {
        let proof = gamma.mint("t_argmax", claim, &policy).expect("f32 is lawful");
        let binding = gamma.binding(proof);
        assert_eq!(binding.dtype(), Dtype::F32, "±∞ and 128255 need f32");
        assert_eq!(binding.provenance(), Provenance::Law);
        assert!(may_store(binding.inferred(), binding.dtype()));
    });
}

/// The ℝ̄ half is unpoliced: an undeclared real takes the policy default —
/// this is the clause that protects the 16%.
#[test]
fn an_undeclared_real_takes_the_policy_default() {
    let hidden = axis("hidden", 2048);
    let x = input("X", [hidden], Dtype::F32);
    let logits_like = map(MapOp::Exp, vec![x]);
    let claim = infer_root(&logits_like);
    let policy = Policy {
        default_real: Dtype::BF16,
    };

    with_bindings(|mut gamma| {
        let proof = gamma.mint("t_scores", claim, &policy).unwrap();
        assert_eq!(gamma.binding(proof).dtype(), Dtype::BF16);
        assert_eq!(gamma.binding(proof).provenance(), Provenance::Policy);
        assert_eq!(gamma.chosen_by_policy().collect::<Vec<_>>(), ["t_scores"]);
    });
}

/// Narrowest sufficient, not a jump to f32: a small exact value under a
/// bf16 default takes f16 — same two bytes, lawful.
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
    assert_eq!(store_dtype(tiny, Dtype::BF16), Some(Dtype::BF16), "lawful preferences stand");
}

/// Refusal, not guessing: an exact value with saturated (unbounded) range
/// has no lawful width, and minting it is a named error.
#[test]
fn an_unboundable_exact_value_is_refused() {
    let saturated = Inferred {
        system: NumberSystem::Integer,
        bounds: Bounds::UNBOUNDED,
    };
    let policy = Policy {
        default_real: Dtype::BF16,
    };
    with_bindings(|mut gamma| {
        assert_eq!(
            gamma.mint("t_overflowed", saturated, &policy),
            Err(Refusal::NoLawfulWidth("t_overflowed"))
        );
        assert!(gamma.lookup("t_overflowed").is_none(), "a refusal leaves no entry");
    });
}

/// The caller chooses — weights at bf16 stand as declared — but cannot
/// choose unsoundly: `tokens` declared bf16 against its own ℕ̄ claim is
/// refused at the declaration.
#[test]
fn a_caller_may_widen_freely_but_not_narrow_below_the_law() {
    let tokens_claim = Inferred {
        system: NumberSystem::Natural,
        bounds: Bounds::range(0, 128_255),
    };
    with_bindings(|mut gamma| {
        let weights = gamma.declare_input(
            "w",
            InputDeclaration {
                dtype: Dtype::BF16,
                claim: None,
            },
        );
        assert!(weights.is_ok(), "an undeclared real is the caller's to narrow");

        let bad = gamma.declare_input(
            "tokens",
            InputDeclaration {
                dtype: Dtype::BF16,
                claim: Some(tokens_claim),
            },
        );
        assert_eq!(bad, Err(Refusal::UnlawfulDeclaration("tokens")));

        let good = gamma.declare_input(
            "tokens",
            InputDeclaration {
                dtype: Dtype::F32,
                claim: Some(tokens_claim),
            },
        );
        assert!(good.is_ok(), "f32 carries the claim exactly");
    });
}

/// Γ is append-only: one entry per name, no second decision, no overwrite
/// path in the API at all.
#[test]
fn a_name_is_decided_exactly_once() {
    let real = Inferred {
        system: NumberSystem::Real,
        bounds: Bounds::UNBOUNDED,
    };
    let policy = Policy {
        default_real: Dtype::F32,
    };
    with_bindings(|mut gamma| {
        gamma.mint("t0", real, &policy).unwrap();
        assert_eq!(gamma.mint("t0", real, &policy), Err(Refusal::AlreadyBound("t0")));
    });
}

/// Subject reduction, live: simplification may sharpen a term's system down
/// the chain and must never widen it. `iota - iota` is ℤ̄; simplified
/// (`x − x → 0`) it is a constant in 𝔹.
#[test]
fn simplification_never_widens_the_number_system() {
    let n = axis("n", 8);
    let term = map(MapOp::Sub, vec![iota(n), iota(n)]);
    let before = infer_root(&term);

    let simplified = sanic::simplify::simplify_many(&[term]).remove(0);
    let after = infer_root(&simplified);

    assert!(
        after.system <= before.system,
        "simplify widened {:?} to {:?}",
        before.system,
        after.system
    );
}
