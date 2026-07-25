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
    /// Round to a storage precision and widen back — the value a boundary
    /// stored at `Dtype` would reload. Declaring it in the graph makes the
    /// numerics schedule-INdependent: storing an already-rounded value at
    /// the same width is exact, and rounding twice is rounding once.
    RoundTo(Dtype),
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
            | MapOp::Cos
            | MapOp::RoundTo(_) => 1,
            MapOp::Where => 3,
            _ => 2,
        }
    }

    /// Does this operation preserve linearity in its arguments?
    pub fn preserves_linear(self) -> bool {
        matches!(self, MapOp::Add | MapOp::Sub | MapOp::Mul | MapOp::Div | MapOp::Neg)
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
            MapOp::RoundTo(_) => "round_to",
        }
    }
}

/// The value `v` reloads as after a round trip through `dtype` storage —
/// the CPU twin of the GPU store path, bit-for-bit (`tests/metal.rs` holds
/// the two together). bf16 rounds to nearest even on the high half of the
/// f32 pattern, NaN kept quiet; f16 via native conversion.
pub fn round_to(dtype: Dtype, v: f64) -> f64 {
    match dtype {
        Dtype::F64 => v,
        Dtype::F32 => v as f32 as f64,
        Dtype::F16 => {
            // f64 → f32 → f16 with the same double rounding the GPU does
            half_round(v as f32) as f64
        }
        Dtype::BF16 => {
            let b = (v as f32).to_bits();
            if (v as f32).is_nan() {
                return f32::from_bits(((b >> 16) | 0x0040) << 16) as f64;
            }
            let r = (b.wrapping_add(0x7FFF).wrapping_add((b >> 16) & 1)) >> 16;
            f32::from_bits(r << 16) as f64
        }
        Dtype::I8 | Dtype::I4 => {
            panic!("round_to: {dtype:?} needs a scale, not a plain rounding")
        }
    }
}

/// f32 → IEEE half → f32, round to nearest even, overflow to infinity.
fn half_round(v: f32) -> f32 {
    if v.is_nan() {
        return v;
    }
    let bits = v.to_bits();
    let sign = bits >> 31;
    let magnitude = f32::from_bits(bits & 0x7FFF_FFFF);
    let rounded = if magnitude.to_bits() >= 0x4780_0000 {
        // above f16 max (65504 rounds up past it): ±inf
        f32::INFINITY
    } else if magnitude.to_bits() < 0x3880_0000 {
        // subnormal in f16: round magnitude/2^-24 to nearest even integer ULPs
        let ulps = magnitude / f32::from_bits(0x3380_0000); // 2^-24
        let n = ulps.round_ties_even();
        n * f32::from_bits(0x3380_0000)
    } else {
        // normal: keep 11 significand bits, round to nearest even
        let b = magnitude.to_bits();
        let r = (b.wrapping_add(0xFFF).wrapping_add((b >> 13) & 1)) & !0x1FFF;
        f32::from_bits(r)
    };
    if sign == 1 { -rounded } else { rounded }
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
