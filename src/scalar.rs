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
        Dtype::U32 => {
            panic!("round_to: U32 stores saturate; they are not a rounding of ℝ")
        }
        // The panic that named M16 — "needs a scale, not a plain rounding"
        // — retires: Q8 HAS one, and its round trip is a genuine rounding
        // of ℝ̄ onto the grid `{q · scale : −127 ≤ q ≤ 127}`. Idempotent
        // like every declared rounding: a grid point rounds to itself.
        Dtype::Q8 { scale_bits } => {
            let scale = f32::from_bits(scale_bits) as f64;
            let q = (v / scale).round().clamp(-127.0, 127.0);
            q * scale
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
    /// Unsigned 32-bit integers — the index format. Stores SATURATE: a
    /// `+∞` fold identity lands on `u32::MAX` by definition rather than by
    /// accident, which is what lets an argmax buffer be an integer at all.
    U32,
    /// Affine-quantized 8-bit: a representation OF ℝ̄, not a rounding of
    /// one — the value `q · scale` for a signed byte `q`, stores clamped to
    /// ±127. The first parameterised format: the scale is part of the TYPE
    /// (carried as its f32 bits so the enum stays `Copy + Eq + Hash`),
    /// exactly MLIR's `!quant.uniform<i8:f32, scale>` observation — storage
    /// type and expressed type in one.
    Q8 {
        scale_bits: u32,
    },
    I8,
    I4,
}

impl Dtype {
    /// Sign, exponent and significand bits. The single description every
    /// other property is computed from, so widths and ranges cannot drift
    /// apart. Integer formats have no exponent; their significand is the
    /// magnitude.
    ///
    /// This wants to be a struct rather than a match the moment a format is
    /// *parameterised* — a quantized dtype carries a scale and a zero point,
    /// which are values, not variants. Until then the enum is worth keeping:
    /// `buf_ty`, `out_ty`, `store_expr`, `buffer_load` and `round_to` all
    /// match exhaustively and panic on formats a target cannot handle, and
    /// that exhaustiveness is what makes those refusals loud.
    const fn layout(self) -> (u32, u32, u32) {
        match self {
            Dtype::F64 => (1, 11, 52),
            Dtype::F32 => (1, 8, 23),
            Dtype::F16 => (1, 5, 10),
            Dtype::BF16 => (1, 8, 7),
            Dtype::U32 => (0, 0, 32),
            Dtype::Q8 { .. } => (1, 0, 7),
            Dtype::I8 => (1, 0, 7),
            Dtype::I4 => (1, 0, 3),
        }
    }

    /// An IEEE-style floating format.
    pub const fn is_float(self) -> bool {
        self.layout().1 > 0
    }

    /// An unsigned integer format: no sign bit, no negative values — and
    /// stores saturate rather than wrap.
    pub const fn is_unsigned(self) -> bool {
        self.layout().0 == 0
    }

    /// Does an out-of-range store land on the nearest representable value
    /// by definition? True of the unsigned integer formats, whose emitted
    /// store clamps — `+∞ → MAX` is the contract, not an accident.
    pub const fn saturates(self) -> bool {
        self.is_unsigned()
    }

    /// Can this format represent ±∞?
    ///
    /// Not a synonym for [`Dtype::is_float`], though the two coincide today:
    /// the `fp8` finite-only formats are floats without infinities. Every
    /// order monoid's identity is an infinity, so this decides whether a
    /// fold's output can be stored in a given format at all.
    pub const fn has_infinities(self) -> bool {
        self.is_float()
    }

    /// Integers this format holds exactly. A property of the format,
    /// identical on every device — bf16 is exact to 256 on any silicon.
    ///
    /// A float is exact up to `2^(significand+1)`, where the implicit
    /// leading bit buys the extra power. A signed integer stops one short
    /// of its magnitude range.
    pub const fn exact_integers_to(self) -> i64 {
        // A scaled format holds integers exactly only where its grid lands
        // on them; claiming any is claiming the scale, so it claims none.
        if matches!(self, Dtype::Q8 { .. }) {
            return 0;
        }
        let (_, exponent, significand) = self.layout();
        if exponent > 0 {
            1i64 << (significand + 1)
        } else {
            (1i64 << significand) - 1
        }
    }

    /// Quantized 8-bit with this scale. The scale rides the type.
    pub fn q8(scale: f32) -> Dtype {
        Dtype::Q8 {
            scale_bits: scale.to_bits(),
        }
    }

    /// The quantization scale, for the formats that have one.
    pub fn scale(self) -> Option<f32> {
        match self {
            Dtype::Q8 { scale_bits } => Some(f32::from_bits(scale_bits)),
            _ => None,
        }
    }

    /// Does an out-of-range store clamp to the nearest representable value
    /// by definition? The unsigned index format and the quantized formats:
    /// `+∞ → MAX` there is contract, and for the SIGNED quantized grid
    /// `−∞ → −127·scale` has a home too.
    pub const fn clamps_signed(self) -> bool {
        matches!(self, Dtype::Q8 { .. })
    }

    /// Width of one element. The honest unit: sub-byte formats have no
    /// fractional-byte size, and newer ones (fp4, fp6) are not even a power
    /// of two, so bits is the only unit that stays exact.
    pub const fn nbits(self) -> u32 {
        let (sign, exponent, significand) = self.layout();
        sign + exponent + significand
    }

    /// Does an element occupy less than a byte, so that storage packs
    /// several per byte and an element has no address of its own?
    pub const fn is_subbyte(self) -> bool {
        self.nbits() < 8
    }

    /// Bytes an array of `elements` occupies, packing included. This is the
    /// allocation and bounds-check unit; [`Dtype::bytes_per_element`] is not,
    /// because a width of 0.5 truncates to zero the moment anyone writes
    /// `as usize`.
    pub const fn nbytes(self, elements: usize) -> usize {
        (elements * self.nbits() as usize).div_ceil(8)
    }

    /// Per-element width as a *pricing weight*, for bandwidth arithmetic
    /// already in floating point. Never a size — see [`Dtype::nbytes`].
    pub fn bytes_per_element(self) -> f64 {
        f64::from(self.nbits()) / 8.0
    }
}
