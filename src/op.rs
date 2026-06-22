//! §3.2 — Operator algebra metadata.
//!
//! Each binary operator carries its *laws*, not just a symbol. The whole engine
//! trusts these laws (the soundness obligation of §3.2): mis-declaring
//! associativity yields incorrect fusions, so a real deployment would ship
//! machine-checked proofs of each. Here they are declared and exercised by the
//! property tests in `carrier.rs`.

/// The monoidal reduction operators the dense affine core is built from.
/// `+`, `×`, `max`, `min`, `logsumexp` are all monoids (associative + identity).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Monoid {
    Add,
    Mul,
    Max,
    Min,
    LogSumExp,
}

impl Monoid {
    pub fn symbol(self) -> &'static str {
        match self {
            Monoid::Add => "+",
            Monoid::Mul => "×",
            Monoid::Max => "max",
            Monoid::Min => "min",
            Monoid::LogSumExp => "logsumexp",
        }
    }

    /// Every monoid here is associative — that is the whole point of the class.
    pub fn associative(self) -> bool {
        true
    }

    pub fn commutative(self) -> bool {
        // all of these happen to be commutative; tree-reduce needs no order care
        true
    }

    /// `None` ⇒ no neutral element.
    pub fn identity(self) -> Option<f64> {
        Some(match self {
            Monoid::Add => 0.0,
            Monoid::Mul => 1.0,
            Monoid::Max | Monoid::LogSumExp => f64::NEG_INFINITY,
            Monoid::Min => f64::INFINITY,
        })
    }

    pub fn is_monoid(self) -> bool {
        self.associative() && self.identity().is_some()
    }

    /// The semiring-*additive* operator (the `⊕` a global scalar distributes
    /// over). This is the linearity certificate of §1.2: `Σ (c·pᵢ)·vᵢ = c·Σ ...`.
    /// `Add` is additive in the usual `(+, ×)` semiring; `LogSumExp` is the
    /// additive op of the log-space (max-plus-ish) semiring.
    pub fn is_additive(self) -> bool {
        matches!(self, Monoid::Add | Monoid::LogSumExp)
    }
}

/// The binary operator carried by `Reduce`/`Scan`. It may be a monoid, an
/// associative-but-not-built-in composition (affine-map composition, the SSM
/// carrier of §5.4), or a genuine magma (a non-associative recurrence step,
/// §6 — the `tanh`-RNN).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Monoid(Monoid),
    /// Composition of affine maps `h ↦ A·h + b`. Associative, identity = id map.
    AffineCompose,
    /// A non-associative step such as `tanh(W·h + x)`. A magma: no `combine`.
    NonAssoc(&'static str),
}

impl BinOp {
    pub fn associative(self) -> bool {
        match self {
            BinOp::Monoid(m) => m.associative(),
            BinOp::AffineCompose => true,
            BinOp::NonAssoc(_) => false,
        }
    }

    pub fn has_identity(self) -> bool {
        match self {
            BinOp::Monoid(m) => m.identity().is_some(),
            BinOp::AffineCompose => true, // identity affine map (A=1, b=0)
            BinOp::NonAssoc(_) => false,
        }
    }

    pub fn is_monoid(self) -> bool {
        self.associative() && self.has_identity()
    }

    pub fn is_additive(self) -> bool {
        matches!(self, BinOp::Monoid(m) if m.is_additive())
    }
}
