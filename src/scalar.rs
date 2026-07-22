//! Scalar and storage vocabulary shared by the one tensor IR and its compiler.

/// The cardinality of one tensor dimension.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Extent {
    Static(usize),
    Dynamic,
}

impl From<usize> for Extent {
    fn from(value: usize) -> Self {
        Extent::Static(value)
    }
}

/// Associative scalar reductions with an identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Monoid {
    Add,
    Mul,
    Max,
    Min,
    LogSumExp,
}

impl Monoid {
    pub fn identity(self) -> f64 {
        match self {
            Monoid::Add => 0.0,
            Monoid::Mul => 1.0,
            Monoid::Max | Monoid::LogSumExp => f64::NEG_INFINITY,
            Monoid::Min => f64::INFINITY,
        }
    }

    /// True for the "plus" of its semiring — the operation a constant factor
    /// distributes over.
    pub fn is_additive(self) -> bool {
        matches!(self, Monoid::Add | Monoid::LogSumExp)
    }
}

/// The closed scalar primitive basis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapOp {
    Add,
    Sub,
    Mul,
    Div,
    Max,
    Min,
    /// `a < b`, producing 1.0 or 0.0.
    Lt,
    Neg,
    Recip,
    Exp,
    Log,
    Sqrt,
    Tanh,
    Sin,
    Cos,
    /// `cond != 0 ? a : b`.
    Where,
}

impl MapOp {
    pub fn arity(self) -> usize {
        match self {
            MapOp::Neg
            | MapOp::Recip
            | MapOp::Exp
            | MapOp::Log
            | MapOp::Sqrt
            | MapOp::Tanh
            | MapOp::Sin
            | MapOp::Cos => 1,
            MapOp::Where => 3,
            _ => 2,
        }
    }

    /// Does this operation preserve linearity in its arguments?
    pub fn preserves_linear(self) -> bool {
        matches!(
            self,
            MapOp::Add | MapOp::Sub | MapOp::Mul | MapOp::Div | MapOp::Neg
        )
    }

    pub fn name(self) -> &'static str {
        match self {
            MapOp::Add => "add",
            MapOp::Sub => "sub",
            MapOp::Mul => "mul",
            MapOp::Div => "div",
            MapOp::Max => "max",
            MapOp::Min => "min",
            MapOp::Lt => "lt",
            MapOp::Neg => "neg",
            MapOp::Recip => "recip",
            MapOp::Exp => "exp",
            MapOp::Log => "log",
            MapOp::Sqrt => "sqrt",
            MapOp::Tanh => "tanh",
            MapOp::Sin => "sin",
            MapOp::Cos => "cos",
            MapOp::Where => "where",
        }
    }
}

/// Storage width of an input buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dtype {
    F64,
    F32,
    F16,
    BF16,
    I8,
    I4,
}

impl Dtype {
    pub fn bytes(self) -> f64 {
        match self {
            Dtype::F64 => 8.0,
            Dtype::F32 => 4.0,
            Dtype::F16 | Dtype::BF16 => 2.0,
            Dtype::I8 => 1.0,
            Dtype::I4 => 0.5,
        }
    }
}
