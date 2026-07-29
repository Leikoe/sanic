//! What a value *is*, as opposed to how it is stored.
//!
//! `Dtype` answers "how many bytes, and in what layout". It cannot answer
//! "is this an index" — every value in this IR computes as a float, so a
//! storage width says nothing about whether the value is integral. That
//! second question is what decides whether narrowing a boundary is sound:
//! bf16 represents integers exactly only to 256, so an index above it is
//! silently corrupted by a storage policy that is perfectly fine for scores.
//!
//! This module answers it. [`NumberSystem`] is a chain ordered by inclusion,
//! [`Bounds`] carries the range of an exact value, and [`infer`] derives both
//! from the graph. Nothing here knows about a device: the width a value can
//! be stored at is a consequence, computed elsewhere, and `infer` is a pure
//! function of the expression.
//!
//! ## Extended systems
//!
//! Every system is *extended*: ±∞ is adjoined. That is forced rather than
//! chosen — `Monoid::Min`'s identity is `+∞` and `Max`'s is `−∞`, so every
//! order fold introduces a sentinel, and `argmax`'s index accumulator starts
//! at `+∞`. Every float format represents ±∞ exactly, so extension costs
//! nothing at the storage end.
//!
//! It follows that `Const(±∞)` joins as the *bottom* system, not as a real:
//! it is an identity element belonging to every system. Classifying it from
//! its value instead ("not an integer, therefore ℝ") makes `argmax`'s `Where`
//! join to ℝ and loses the exactness the whole analysis exists to find.
//!
//! NaN is outside every system. `∞ − ∞` is NaN, so the arithmetic rows below
//! are closed only on the finite part; a NaN means something upstream was
//! wrong, which is the position `SANIC_NANSCAN` already takes.

use std::collections::HashMap;

use crate::ir::{Node, NodeRef};
use crate::scalar::{Dtype, MapOp, Monoid};

/// A number system, ordered by inclusion: 𝔹 ⊂ ℕ ⊂ ℤ ⊂ ℝ.
///
/// The variant order *is* the inclusion law, so `join` is `max` — a chain,
/// not merely a lattice, which is why no lattice machinery appears here.
/// `Bool` sits at the bottom because the IR already encodes predicates as
/// 0.0/1.0.
///
/// ℂ is deliberately absent. Nothing in this compiler can produce or store a
/// complex value, and the link ℝ ⊂ ℂ is false once ±∞ is adjoined — ±∞ has no
/// image in ℂ — so `max` would not be a join at the top. RoPE's complex
/// rotation stays what it is today: real pairs over an extent-2 axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NumberSystem {
    Bool,
    Natural,
    Integer,
    Real,
}

impl NumberSystem {
    /// The smallest system containing both. `max` on a chain.
    pub fn join(self, other: Self) -> Self {
        self.max(other)
    }

    /// Are this system's values integers? Exactness is what licenses storing
    /// a value in a representation that rounds — or rather, forbids it.
    pub fn is_exact(self) -> bool {
        self <= NumberSystem::Integer
    }
}

/// The range of a value's **finite** part, plus whether ±∞ can occur.
///
/// Those are two separate facts and the analysis needs both. An `argmax`
/// output's finite values are indices in `[0, extent−1]` while its range
/// formally admits the `+∞` that `Monoid::Min`'s identity injects — and
/// collapsing them into one interval loses the upper bound, which is the
/// bound that decides whether bf16 can hold it. So `[0, 128255] ∪ {∞}` is
/// represented as such, not as `[0, ∞]`.
///
/// `None` on an endpoint means unbounded on that side, which is also where
/// overflow saturates. The empty finite set — a value that is *only* ±∞ —
/// is `lo > hi`, so `union` stays plain `min`/`max`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bounds {
    pub lo: Option<i64>,
    pub hi: Option<i64>,
    /// Can this value be ±∞? An integer representation cannot hold one, so
    /// this is a precondition on every integer store.
    pub infinite: bool,
}

impl Bounds {
    pub const UNBOUNDED: Bounds = Bounds {
        lo: None,
        hi: None,
        infinite: true,
    };

    /// No finite values at all: an infinity and nothing else.
    pub const INFINITE_ONLY: Bounds = Bounds {
        lo: Some(i64::MAX),
        hi: Some(i64::MIN),
        infinite: true,
    };

    pub fn point(value: i64) -> Bounds {
        Bounds::range(value, value)
    }

    pub fn range(lo: i64, hi: i64) -> Bounds {
        Bounds {
            lo: Some(lo),
            hi: Some(hi),
            infinite: false,
        }
    }

    /// The smallest claim containing both.
    pub fn union(self, other: Bounds) -> Bounds {
        Bounds {
            lo: lower(self.lo, other.lo, i64::min),
            hi: lower(self.hi, other.hi, i64::max),
            infinite: self.infinite || other.infinite,
        }
    }

    /// Does every finite value fit within `limit` in magnitude? Written
    /// without `abs`, because `i64::MIN.abs()` panics in debug and *wraps*
    /// in release, which would certify the widest possible range as exact.
    pub fn within(self, limit: i64) -> bool {
        self.lo.is_some_and(|lo| lo >= -limit) && self.hi.is_some_and(|hi| hi <= limit)
    }

    pub fn is_finite(self) -> bool {
        !self.infinite
    }
}

/// Combine two endpoints where `None` is unbounded and absorbs.
fn lower(a: Option<i64>, b: Option<i64>, pick: fn(i64, i64) -> i64) -> Option<i64> {
    match (a, b) {
        (Some(a), Some(b)) => Some(pick(a, b)),
        _ => None,
    }
}

/// Interval arithmetic over the four corners — the usual rule, and the only
/// one correct when a range straddles zero. Overflow saturates to unbounded,
/// which forces the value out of the exact systems rather than wrapping into
/// a small range that would look storable. `coordinate * extent + coordinate`
/// overflows `i64` in four multiplies, so this is not defensive.
fn corners(a: Bounds, b: Bounds, combine: fn(i64, i64) -> Option<i64>) -> Bounds {
    let (Some(a_lo), Some(a_hi), Some(b_lo), Some(b_hi)) = (a.lo, a.hi, b.lo, b.hi) else {
        return Bounds {
            lo: None,
            hi: None,
            infinite: a.infinite || b.infinite,
        };
    };
    let mut lo = None;
    let mut hi = None;
    let mut overflowed = false;
    for (x, y) in [(a_lo, b_lo), (a_lo, b_hi), (a_hi, b_lo), (a_hi, b_hi)] {
        match combine(x, y) {
            Some(corner) => {
                lo = Some(lo.map_or(corner, |seen: i64| seen.min(corner)));
                hi = Some(hi.map_or(corner, |seen: i64| seen.max(corner)));
            }
            None => overflowed = true,
        }
    }
    Bounds {
        lo: if overflowed { None } else { lo },
        hi: if overflowed { None } else { hi },
        infinite: a.infinite || b.infinite,
    }
}

fn negated(bounds: Bounds) -> Bounds {
    Bounds {
        lo: bounds.hi.and_then(i64::checked_neg),
        hi: bounds.lo.and_then(i64::checked_neg),
        infinite: bounds.infinite,
    }
}

/// What the analysis knows about one value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Inferred {
    pub system: NumberSystem,
    /// Meaningful only when `system.is_exact()`; [`Bounds::UNBOUNDED`]
    /// otherwise.
    pub bounds: Bounds,
}

impl Inferred {
    fn exact(system: NumberSystem, bounds: Bounds) -> Inferred {
        Inferred { system, bounds }
    }

    fn real() -> Inferred {
        Inferred {
            system: NumberSystem::Real,
            bounds: Bounds::UNBOUNDED,
        }
    }

    /// Raise to at least `floor`, keeping the bounds. Leaving the exact
    /// systems discards them, since they describe integers.
    fn at_least(self, floor: NumberSystem) -> Inferred {
        let system = self.system.join(floor);
        if system.is_exact() {
            Inferred::exact(system, self.bounds)
        } else {
            Inferred {
                system,
                bounds: Bounds::UNBOUNDED,
            }
        }
    }
}

/// May a value with this analysis be stored as `dtype`?
///
/// Exact systems must be represented exactly and within range. Reals may
/// round, and how far is a policy question this module does not answer. In
/// either case a value that can be infinite needs a representation that
/// holds ±∞, which rules out the integer formats.
pub fn may_store(value: Inferred, dtype: Dtype) -> bool {
    if !value.bounds.is_finite() && !dtype.has_infinities() {
        return false;
    }
    if !value.system.is_exact() {
        return true;
    }
    value.bounds.within(dtype.exact_integers_to())
}

/// Infer every node's system and bounds, memoised by address the way shapes
/// are. The result is transient: its only consumer is the choice of a
/// storage width, and nothing carries it past that.
pub fn infer(root: &NodeRef) -> HashMap<*const Node, Inferred> {
    let mut known = HashMap::new();
    infer_node(root, &mut known);
    known
}

/// [`infer`] when only the root's answer is wanted.
pub fn infer_root(root: &NodeRef) -> Inferred {
    let mut known = HashMap::new();
    infer_node(root, &mut known)
}

fn infer_node(node: &NodeRef, known: &mut HashMap<*const Node, Inferred>) -> Inferred {
    let key = std::ptr::from_ref(node.as_ref());
    if let Some(&already) = known.get(&key) {
        return already;
    }
    let inferred = derive_node(node, known);
    known.insert(key, inferred);
    inferred
}

fn derive_node(node: &NodeRef, known: &mut HashMap<*const Node, Inferred>) -> Inferred {
    match node.as_ref() {
        // An input's system is whatever its declared storage can tell us,
        // which for a float format is only "a real". A caller that knows
        // better — `tokens` is an index stored as f32 — has no way to say so
        // yet; that is the open question this analysis waits on.
        Node::Input { dtype, .. } => match dtype {
            Dtype::I8 | Dtype::I4 => {
                Inferred::exact(NumberSystem::Integer, Bounds::UNBOUNDED)
            }
            _ => Inferred::real(),
        },

        Node::Const { v } => constant(*v),

        Node::Iota { axis } => coordinate_of(axis.extent()),

        Node::Coordinate { src, dim } => {
            let extent = src.shape()[*dim].extent();
            coordinate_of(extent)
        }

        Node::Map { op, inputs } => {
            let arguments: Vec<Inferred> = inputs.iter().map(|input| infer_node(input, known)).collect();
            map(*op, &arguments)
        }

        Node::Reduce { src, dim, op } | Node::Scan { src, dim, op } => {
            let operand = infer_node(src, known);
            let extent = src.shape()[*dim].extent();
            fold(*op, operand, extent)
        }

        // The result is drawn from the table, so it inhabits the table's
        // system. The index's own system is a precondition, not a component.
        Node::Gather { src, index, .. } => {
            infer_node(index, known);
            infer_node(src, known)
        }

        Node::View { src, .. } => infer_node(src, known),

        // A padded reindex fills out-of-range coordinates with 0.0, so the
        // value range gains it.
        Node::Reindex { src, padded, .. } => {
            let source = infer_node(src, known);
            if *padded && source.system.is_exact() {
                Inferred::exact(source.system.join(NumberSystem::Natural), source.bounds.union(Bounds::point(0)))
            } else {
                source
            }
        }
    }
}

/// A coordinate along an axis: the only exact producer in the IR.
fn coordinate_of(extent: usize) -> Inferred {
    Inferred::exact(
        NumberSystem::Natural,
        Bounds {
            lo: Some(0),
            hi: i64::try_from(extent).ok().map(|n| n - 1),
            infinite: false,
        },
    )
}

/// ±∞ is an identity element, not a real: it belongs to every extended
/// system, so it joins as the bottom one. A finite integer takes the
/// smallest system containing it; anything else is a real.
fn constant(v: f64) -> Inferred {
    if v.is_infinite() {
        return Inferred::exact(NumberSystem::Bool, Bounds::INFINITE_ONLY);
    }
    if !v.is_finite() || v.fract() != 0.0 {
        return Inferred::real();
    }
    // Every f64 past 2^53 is "an integer"; treating those as exact would
    // claim a precision the value never had.
    if v.abs() > (1i64 << 53) as f64 {
        return Inferred::real();
    }
    let exact = v as i64;
    let system = match exact {
        0 | 1 => NumberSystem::Bool,
        positive if positive > 0 => NumberSystem::Natural,
        _ => NumberSystem::Integer,
    };
    Inferred::exact(system, Bounds::point(exact))
}

fn map(op: MapOp, arguments: &[Inferred]) -> Inferred {
    let join_all = || {
        arguments
            .iter()
            .copied()
            .reduce(|a, b| Inferred {
                system: a.system.join(b.system),
                bounds: a.bounds.union(b.bounds),
            })
            .unwrap_or_else(Inferred::real)
    };
    match op {
        // 𝔹 is not closed under addition — 1 + 1 = 2 — so the floor is ℕ.
        MapOp::Add => {
            let system = arguments.iter().fold(NumberSystem::Natural, |a, b| a.join(b.system));
            let bounds = corners(arguments[0].bounds, arguments[1].bounds, i64::checked_add);
            Inferred { system, bounds }.at_least(system)
        }
        MapOp::Sub => {
            let system = arguments
                .iter()
                .fold(NumberSystem::Integer, |a, b| a.join(b.system));
            let bounds = corners(
                arguments[0].bounds,
                negated(arguments[1].bounds),
                i64::checked_add,
            );
            Inferred { system, bounds }.at_least(system)
        }
        MapOp::Neg => Inferred {
            system: arguments[0].system,
            bounds: negated(arguments[0].bounds),
        }
        .at_least(NumberSystem::Integer),
        MapOp::Mul => {
            let system = arguments.iter().fold(NumberSystem::Bool, |a, b| a.join(b.system));
            let bounds = corners(arguments[0].bounds, arguments[1].bounds, i64::checked_mul);
            Inferred { system, bounds }.at_least(system)
        }
        // Order maps are closed in every system and keep the operands' range.
        MapOp::Max | MapOp::Min => join_all(),
        // A predicate, whatever its operands were.
        MapOp::Lt => Inferred::exact(NumberSystem::Bool, Bounds::range(0, 1)),
        // The condition is tested `!= 0`, not `== 1`, and masks built from
        // arithmetic (`1 - lt`) are ℤ-valued while holding only {0,1}. So it
        // is checked for nothing and contributes nothing: the result is the
        // join of the two branches alone.
        MapOp::Where => Inferred {
            system: arguments[1].system.join(arguments[2].system),
            bounds: arguments[1].bounds.union(arguments[2].bounds),
        },
        // Leaving the exact systems: no inverse in ℤ, and the transcendentals
        // are real-valued outright. `RoundTo` is a declared storage rounding,
        // which is a statement about reals.
        MapOp::Div
        | MapOp::Recip
        | MapOp::Exp
        | MapOp::Log
        | MapOp::Sqrt
        | MapOp::Tanh
        | MapOp::Sin
        | MapOp::Cos
        | MapOp::RoundTo(_) => Inferred::real(),
    }
}

/// A fold injects its identity and repeats its operation `extent` times, so
/// both the system floor and the range differ from the map of the same name.
fn fold(op: Monoid, operand: Inferred, extent: usize) -> Inferred {
    let count = i64::try_from(extent).unwrap_or(i64::MAX);
    match op {
        // Summing predicates counts them, which leaves 𝔹 immediately.
        Monoid::Add => Inferred {
            system: operand.system,
            bounds: corners(operand.bounds, Bounds::point(count), i64::checked_mul),
        }
        .at_least(NumberSystem::Natural),
        // `lo^n`..`hi^n` saturates fast, which is the honest answer.
        Monoid::Mul => {
            let mut bounds = Bounds::point(1);
            for _ in 0..extent.min(64) {
                bounds = corners(bounds, operand.bounds, i64::checked_mul);
            }
            if extent > 64 {
                bounds = Bounds::UNBOUNDED;
            }
            Inferred {
                system: operand.system,
                bounds,
            }
            .at_least(operand.system)
        }
        // A fold over an order monoid returns one of its elements — or its
        // identity, if the axis is empty. So: the operand's finite range,
        // and the sentinel this monoid injects.
        Monoid::Max | Monoid::Min => Inferred {
            system: operand.system,
            bounds: operand.bounds.union(Bounds::INFINITE_ONLY),
        },
        Monoid::LogSumExp => Inferred::real(),
    }
}
