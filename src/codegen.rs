//! Shared code-generation core: one node→code emitter, one carrier renderer,
//! parameterized by a target language.
//!
//! The Rust ([`crate::rustgen`]) and Metal ([`crate::emit_metal`]) backends are
//! the *same* recursion over the IR — the codegen twin of [`crate::interp`] —
//! differing only in leaf syntax (`.exp()` vs `exp()`, `let mut` vs `float`,
//! Rust `for` vs C `for`). That difference is captured by the [`Lang`] trait;
//! everything structural — [`value`], [`carrier_expr`], [`offset`],
//! [`buffers`] — lives here once, so a new op or a fix lands in a single
//! place and both backends stay in lock-step. Per-backend files keep only the
//! genuinely different parts: the kernel wrapper (nested grid loops vs one
//! GPU thread) and the host/runtime glue.

use std::collections::HashMap;

use crate::derive::Expr;
use crate::ir::{self, AxisRef, Dtype, MapOp, Monoid, Node as NodeKind, NodeRef as Node};

// ── fresh names ──────────────────────────────────────────────────────────────

pub struct Gen {
    pub n: usize,
    /// Storage dtypes of the inputs in scope.
    /// Each load uses its declared representation: packed int4 nibbles,
    /// halfs, or a native float.
    pub dtypes: HashMap<&'static str, Dtype>,
    /// When set (by a cooperative GPU emitter), an in-body `Reduce` whose
    /// subtree avoids `avoid_axis` and whose extent is a multiple of the
    /// simd width is emitted lane-split: each lane folds a strided slice,
    /// then the partials merge with the monoid over simd shuffles — the
    /// same re-association the GROUP split proves, one level down. Only
    /// meaningful for a target whose [`Lang::simd_lane_merge`] is `Some`,
    /// and only sound when all lanes execute the reduce convergently (the
    /// scheduled fold emitter guarantees it; pointwise kernels never set
    /// this).
    pub lane_body: Option<LaneBody>,
    /// Input names whose value is already IN A REGISTER at the current
    /// render site — read the variable instead of the buffer. This is how a
    /// fused epilogue reads the fold's own result: the projection lands in a
    /// local, the epilogue's input leaf resolves to it, and the kernel writes
    /// the final value in one pass.
    pub local_inputs: HashMap<String, String>,
    /// Node-local occurrences that denote the same kernel loop coordinate.
    /// Filled by carrier emitters; pointwise structural recursion normally
    /// leaves it empty because it translates coordinates directly.
    pub axis_aliases: HashMap<AxisRef, AxisRef>,
}

/// See [`Gen::lane_body`].
#[derive(Clone, Copy)]
pub struct LaneBody {
    /// The lane-distributed output axis, if any: a reduce whose subtree
    /// reads it varies per lane and must stay serial.
    pub avoid_axis: Option<AxisRef>,
    pub simd_width: usize,
}

impl Gen {
    pub fn new() -> Self {
        Gen {
            n: 0,
            dtypes: HashMap::new(),
            lane_body: None,
            local_inputs: HashMap::new(),
            axis_aliases: HashMap::new(),
        }
    }
    pub fn fresh(&mut self, tag: &str) -> String {
        self.n += 1;
        format!("{tag}{}", self.n)
    }

    fn coordinate<'a>(&self, coord: &'a HashMap<AxisRef, String>, axis: AxisRef) -> &'a String {
        static ZERO: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| "0".to_string());
        if axis.extent == ir::Extent::Static(1) {
            return &ZERO;
        }
        if let Some(value) = coord.get(&axis) {
            return value;
        }
        let target = self.axis_aliases.get(&axis).copied().unwrap_or(axis);
        coord
            .iter()
            .find_map(|(candidate, value)| {
                (self
                    .axis_aliases
                    .get(candidate)
                    .copied()
                    .unwrap_or(*candidate)
                    == target)
                    .then_some(value)
            })
            .unwrap_or_else(|| {
                panic!(
                    "missing coordinate for {axis:?}; available={:?}",
                    coord.keys().collect::<Vec<_>>()
                )
            })
    }

    fn buffer_offset(&self, axes: &[AxisRef], coord: &HashMap<AxisRef, String>) -> String {
        if axes.is_empty() {
            return "0".into();
        }
        let terms = axes
            .iter()
            .enumerate()
            .filter(|(_, axis)| axis.extent() != 1)
            .map(|(index, &axis)| {
                let stride: usize = axes[index + 1..].iter().map(|axis| axis.extent()).product();
                let value = self.coordinate(coord, axis);
                if stride == 1 {
                    value.clone()
                } else {
                    format!("{value}*{stride}")
                }
            })
            .collect::<Vec<_>>();
        if terms.is_empty() {
            "0".into()
        } else {
            terms.join(" + ")
        }
    }
}
impl Default for Gen {
    fn default() -> Self {
        Self::new()
    }
}

/// A buffer name made Rust/C-identifier-safe and prefixed so it can never be a
/// keyword or start with a digit.
pub fn san(name: &str) -> String {
    let body: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    format!("b_{body}")
}

/// Every `Input` leaf and its *declared* axes (its buffer's storage layout),
/// deduped by name in first-seen order. The declared axes are what `offset`
/// strides over; `View` handling in [`value`] remaps coordinates to read the
/// buffer under them.
pub fn buffers(node: &Node) -> Vec<(&'static str, Vec<AxisRef>)> {
    fn go(n: &Node, out: &mut Vec<(&'static str, Vec<AxisRef>)>) {
        match n.as_ref() {
            NodeKind::Input { name, .. } => {
                if !out.iter().any(|(nm, _)| nm == name) {
                    out.push((name, ir::axis_refs(n)));
                }
            }
            NodeKind::Const { .. } | NodeKind::Iota { .. } => {}
            NodeKind::Map { inputs, .. } => inputs.iter().for_each(|i| go(i, out)),
            NodeKind::Coordinate { src, .. } => go(src, out),
            NodeKind::Reduce { src, .. }
            | NodeKind::Scan { src, .. }
            | NodeKind::View { src, .. }
            | NodeKind::Reindex { src, .. } => go(src, out),
            NodeKind::Gather { src, index, .. } => {
                go(src, out);
                go(index, out);
            }
        }
    }
    let mut out = Vec::new();
    go(node, &mut out);
    out
}

/// The row-major flat offset of `axes` at the loop-variable coordinate, with
/// concrete extents baked in as literals. Language-agnostic (integer index
/// arithmetic is the same in Rust and MSL).
pub fn offset(axes: &[AxisRef], coord: &HashMap<AxisRef, String>) -> String {
    if axes.is_empty() {
        return "0".into();
    }
    let mut terms = Vec::new();
    for (k, a) in axes.iter().enumerate() {
        let stride: usize = axes[k + 1..].iter().map(|x| x.extent()).product();
        let iv = coord.get(a).unwrap_or_else(|| {
            panic!(
                "missing coordinate for {a:?}; buffer axes={axes:?}; available={:?}",
                coord.keys().collect::<Vec<_>>()
            )
        });
        if stride == 1 {
            terms.push(iv.clone());
        } else {
            terms.push(format!("{iv}*{stride}"));
        }
    }
    terms.join(" + ")
}

// ── the target-language seam ─────────────────────────────────────────────────

/// The syntactic primitives that differ between code-gen targets. Everything
/// structural is written once against this trait.
pub trait Lang {
    /// A floating literal (handling ±∞).
    fn lit(&self, v: f64) -> String;
    /// Turn a `usize` loop variable into a float value (for `Iota`).
    fn iota_val(&self, ivar: &str) -> String;
    /// Declare a mutable scalar accumulator: `let mut n = init;` / `float n = init;`.
    fn scalar_decl(&self, name: &str, init: &str) -> String;
    /// Open a `0..count` loop over `var`; the close is always `}`.
    fn for_open(&self, var: &str, count: usize) -> String;
    /// Open a loop `var = 0..=bound` where `bound` is a runtime index
    /// EXPRESSION (a prefix scan folds up to its own output position).
    fn for_open_upto(&self, var: &str, bound: &str) -> String;
    /// Declare an integer index from a rounded float value (for `Gather`).
    fn round_index(&self, name: &str, val: &str) -> String;
    /// Declare an integer index (mutable iff it will be updated, as flatten's
    /// running remainder is).
    fn index_decl(&self, name: &str, val: &str, mutable: bool) -> String;
    /// Declare a SIGNED index — movement-op arithmetic (`Reindex`) can go
    /// negative before its bounds check.
    fn signed_index_decl(&self, name: &str, val: &str) -> String;
    /// An unsigned loop variable as a signed value.
    fn to_signed(&self, expr: &str) -> String;
    /// Declare an unsigned index from a signed value known to be in range.
    fn index_from_signed(&self, name: &str, val: &str) -> String;
    /// Declare an unsigned index from a signed value, clamped to `[0, n)` so
    /// a padded read stays in bounds (the loaded value is discarded by the
    /// select when the raw index was out of range).
    fn clamped_index_decl(&self, name: &str, val: &str, n: usize) -> String;
    /// `cond ? a : b` over a BOOLEAN condition string (not a float).
    fn select_bool(&self, cond: &str, a: &str, b: &str) -> String;
    /// Apply an elementwise [`MapOp`] to already-rendered arguments.
    fn map_op(&self, op: MapOp, a: &[String]) -> String;
    /// The scalar monoid combine `acc ⊕ ev`.
    fn monoid(&self, m: Monoid, acc: &str, ev: &str) -> String;
    /// Read element `off` of input buffer `name` stored at `dtype` width as a
    /// float expression. The Metal target reads halfs and unpacks int4
    /// nibbles here; a target without typed storage must decline loudly
    /// rather than mis-read.
    fn buffer_load(&self, name: &str, off: &str, dtype: Dtype) -> String {
        match dtype {
            Dtype::F64 | Dtype::F32 => format!("{}[{off}]", san(name)),
            d => panic!("this backend has no {d:?} storage support"),
        }
    }
    /// The current lane index expression, for targets with a simd width
    /// (`None` = no lane parallelism; in-body reduces stay serial).
    fn lane_var(&self) -> Option<String> {
        None
    }
    /// Merge `acc` across the simd lanes with a COMMUTATIVE monoid (a
    /// butterfly of shuffles). `None` on targets without simd lanes.
    fn simd_lane_merge(&self, _acc: &str, _m: Monoid, _width: usize) -> Option<Vec<String>> {
        None
    }
}

/// `name = val;` — identical across targets.
pub fn assign(name: &str, val: &str) -> String {
    format!("{name} = {val};")
}

// ── the shared recursion ─────────────────────────────────────────────────────

/// Emit code for the scalar value of `node` at `coord` (a map from each
/// in-scope axis to the loop variable holding its index). Statements that need
/// their own scope — a reduction loop, a gather index — are pushed to `out` in
/// order; the return is a pure expression reading whatever `out` set up. This
/// is [`crate::interp`]'s `eval_node`, targeting code.
pub fn value<L: Lang>(
    lang: &L,
    node: &Node,
    coord: &HashMap<AxisRef, String>,
    g: &mut Gen,
    out: &mut Vec<String>,
) -> String {
    match node.as_ref() {
        NodeKind::Const { v } => lang.lit(*v),
        NodeKind::Iota { .. } => lang.iota_val(g.coordinate(coord, ir::axis_refs(node)[0])),
        NodeKind::Coordinate { src, dim } => {
            lang.iota_val(g.coordinate(coord, ir::axis_refs(src)[*dim]))
        }
        NodeKind::Input { name, dtype, .. } => {
            if let Some(v) = g.local_inputs.get(*name) {
                return v.clone();
            }
            lang.buffer_load(name, &g.buffer_offset(&ir::axis_refs(node), coord), *dtype)
        }
        NodeKind::Map { op, inputs } => {
            let a: Vec<String> = inputs
                .iter()
                .map(|input| {
                    let mut input_coord = coord.clone();
                    for output_axis in ir::axis_refs(node) {
                        let input_axis = ir::map_input_axis(node, input, output_axis);
                        if input_axis != output_axis {
                            input_coord
                                .insert(input_axis, g.coordinate(coord, output_axis).clone());
                        }
                    }
                    value(lang, input, &input_coord, g, out)
                })
                .collect();
            lang.map_op(*op, &a)
        }
        NodeKind::Reduce { src, dim, op } => {
            let axis = ir::source_axis(src, *dim);
            let m = *op;
            let acc = g.fresh("acc");
            out.push(lang.scalar_decl(&acc, &lang.lit(m.identity())));
            let lv = g.fresh("r");
            let mut coord2 = coord.clone();
            coord2.insert(axis, lv.clone());
            let mut body = Vec::new();
            let ev = value(lang, src, &coord2, g, &mut body);
            // Lane-split the contraction when the scheduled emitter asked
            // for it and it is sound here: every monoid is commutative, so
            // a strided lane partition + shuffle merge equals the serial
            // fold — provided the subtree is lane-uniform (it must not read
            // the lane-distributed axis).
            let lane_split = g.lane_body.and_then(|lb| {
                let lane = lang.lane_var()?;
                let uniform = lb
                    .avoid_axis
                    .is_none_or(|a| !ir::axis_refs(src).contains(&a));
                let merge = lang.simd_lane_merge(&acc, m, lb.simd_width)?;
                (axis.extent() % lb.simd_width == 0 && uniform).then_some((
                    lane,
                    lb.simd_width,
                    merge,
                ))
            });
            match lane_split {
                Some((lane, w, merge)) => {
                    out.push(format!(
                        "for (uint {lv} = {lane}; {lv} < {}; {lv} += {w}) {{",
                        axis.extent()
                    ));
                    out.extend(body);
                    out.push(assign(&acc, &lang.monoid(m, &acc, &ev)));
                    out.push("}".into());
                    out.extend(merge);
                }
                None => {
                    out.push(lang.for_open(&lv, axis.extent()));
                    out.extend(body);
                    out.push(assign(&acc, &lang.monoid(m, &acc, &ev)));
                    out.push("}".into());
                }
            }
            acc
        }
        NodeKind::Gather { src, index, dim } => {
            let axis = ir::source_axis(src, *dim);
            let ie = value(lang, index, coord, g, out);
            let gi = g.fresh("gi");
            out.push(lang.round_index(&gi, &ie));
            let mut coord2 = coord.clone();
            coord2.insert(axis, gi);
            value(lang, src, &coord2, g, out)
        }
        NodeKind::View { src, .. } => {
            let mut coord2 = coord.clone();
            for (members, to) in ir::view_groups(node) {
                // Flatten: split the merged index (first member most
                // significant, so the last runs fastest).
                let rem = g.fresh("rem");
                out.push(lang.index_decl(&rem, g.coordinate(coord, to), true));
                for member in members.iter().rev() {
                    let iv = g.fresh("m");
                    out.push(lang.index_decl(&iv, &format!("{rem} % {}", member.extent()), false));
                    out.push(format!("{rem} /= {};", member.extent()));
                    coord2.insert(*member, iv);
                }
            }
            value(lang, src, &coord2, g, out)
        }
        // Affine reindexing: compute each mapped source axis's signed index;
        // padded reads clamp the index (so any setup statements below stay in
        // bounds) and select 0.0 when the raw index was out of range. The
        // codegen twin of the interpreter's `Reindex` arm.
        NodeKind::Reindex { src, padded, .. } => {
            let mut coord2 = coord.clone();
            let mut guards: Vec<String> = Vec::new();
            for (mapped, terms, off) in ir::resolved_reindex(node) {
                let mut parts: Vec<String> = terms
                    .iter()
                    .map(|(coef, a)| {
                        let iv = lang.to_signed(g.coordinate(coord, *a));
                        if *coef == 1 {
                            iv
                        } else {
                            format!("{coef}*{iv}")
                        }
                    })
                    .collect();
                if off != 0 {
                    parts.push(format!("({off})"));
                }
                let val = if parts.is_empty() {
                    "0".to_string()
                } else {
                    parts.join(" + ")
                };
                let ri = g.fresh("ri");
                out.push(lang.signed_index_decl(&ri, &val));
                let n = mapped.extent();
                let ci = g.fresh("ci");
                if *padded {
                    guards.push(format!("{ri} >= 0 && {ri} < {n}"));
                    out.push(lang.clamped_index_decl(&ci, &ri, n));
                } else {
                    out.push(lang.index_from_signed(&ci, &ri));
                }
                coord2.insert(mapped, ci);
            }
            let v = value(lang, src, &coord2, g, out);
            if guards.is_empty() {
                v
            } else {
                lang.select_bool(&guards.join(" && "), &v, &lang.lit(0.0))
            }
        }

        // A prefix scan: each output point folds its own prefix —
        // parallel across points, serial within one (O(n²) work; these are
        // small stages, and correctness comes first — a cost-driven
        // work-efficient scan is a schedule refinement, not new semantics).
        NodeKind::Scan { src, dim, op } => {
            let axis = ir::source_axis(src, *dim);
            let m = *op;
            let acc = g.fresh("acc");
            out.push(lang.scalar_decl(&acc, &lang.lit(m.identity())));
            let lv = g.fresh("r");
            let mut coord2 = coord.clone();
            coord2.insert(axis, lv.clone());
            let mut body = Vec::new();
            let ev = value(lang, src, &coord2, g, &mut body);
            out.push(lang.for_open_upto(&lv, g.coordinate(coord, axis)));
            out.extend(body);
            out.push(assign(&acc, &lang.monoid(m, &acc, &ev)));
            out.push("}".into());
            acc
        }
    }
}

/// Render a derived carrier [`Expr`]: `Item(i)` → `x[i]`, `A(i)`/`F(i)` →
/// `acc[i]`, `B(i)` → `el[i]` (identical in both targets); operators route
/// through the language's [`Lang::map_op`].
pub fn carrier_expr<L: Lang>(lang: &L, e: &Expr) -> String {
    carrier_expr_map(
        lang,
        e,
        &|i| format!("x[{i}]"),
        &|i| format!("acc[{i}]"),
        &|i| format!("el[{i}]"),
    )
}

/// [`carrier_expr`] with the leaf renderers supplied by the caller — a
/// cooperative emitter routes `A(i)`/`B(i)` to per-lane arrays, threadgroup
/// partials, or shuffled registers; the expression structure never changes.
pub fn carrier_expr_map<L: Lang>(
    lang: &L,
    e: &Expr,
    item: &dyn Fn(usize) -> String,
    a_slot: &dyn Fn(usize) -> String,
    b_slot: &dyn Fn(usize) -> String,
) -> String {
    let go = |e: &Expr| carrier_expr_map(lang, e, item, a_slot, b_slot);
    let bin = |op: MapOp, a: &Expr, b: &Expr| lang.map_op(op, &[go(a), go(b)]);
    let un = |op: MapOp, a: &Expr| lang.map_op(op, &[go(a)]);
    match e {
        Expr::Const(v) => lang.lit(*v),
        Expr::Item(i) => item(*i),
        Expr::A(i) | Expr::F(i) => a_slot(*i),
        Expr::B(i) => b_slot(*i),
        Expr::Add(a, b) => bin(MapOp::Add, a, b),
        Expr::Sub(a, b) => bin(MapOp::Sub, a, b),
        Expr::Mul(a, b) => bin(MapOp::Mul, a, b),
        Expr::Div(a, b) => bin(MapOp::Div, a, b),
        Expr::Max(a, b) => bin(MapOp::Max, a, b),
        Expr::Min(a, b) => bin(MapOp::Min, a, b),
        Expr::Lt(a, b) => bin(MapOp::Lt, a, b),
        Expr::Exp(a) => un(MapOp::Exp, a),
        Expr::Log(a) => un(MapOp::Log, a),
        Expr::Sqrt(a) => un(MapOp::Sqrt, a),
        Expr::Tanh(a) => un(MapOp::Tanh, a),
        Expr::Sin(a) => un(MapOp::Sin, a),
        Expr::Cos(a) => un(MapOp::Cos, a),
        Expr::Where(c, a, b) => lang.map_op(MapOp::Where, &[go(c), go(a), go(b)]),
    }
}

/// Grid decode shared by kernels that run one thread per output point (the
/// GPU shape): produce `let i_ax = (gid / stride) % extent;` for each grid
/// axis and return the coordinate map.
pub fn thread_grid_decode<L: Lang>(
    lang: &L,
    grid: &[AxisRef],
    g: &mut Gen,
    out: &mut Vec<String>,
) -> HashMap<AxisRef, String> {
    thread_grid_decode_from(lang, "gid", grid, g, out)
}

/// [`thread_grid_decode`] against an arbitrary index variable — a split-
/// reduction partial kernel decodes the grid from `gid / blocks`.
pub fn thread_grid_decode_from<L: Lang>(
    _lang: &L,
    gid_var: &str,
    grid: &[AxisRef],
    g: &mut Gen,
    out: &mut Vec<String>,
) -> HashMap<AxisRef, String> {
    let mut coord = HashMap::new();
    for (k, &a) in grid.iter().enumerate() {
        let stride: usize = grid[k + 1..].iter().map(|x| x.extent()).product();
        let iv = format!("i_{}", g.fresh("g"));
        if stride == 1 {
            out.push(format!("uint {iv} = {gid_var} % {};", a.extent()));
        } else {
            out.push(format!(
                "uint {iv} = ({gid_var} / {stride}) % {};",
                a.extent()
            ));
        }
        coord.insert(a, iv);
    }
    coord
}

/// The output grid (free axes) and its flattened size for a kernel node.
pub fn grid_of(node: &Node) -> (Vec<AxisRef>, usize) {
    (
        ir::axis_refs(node),
        node.shape()
            .iter()
            .map(|axis| axis.extent())
            .product::<usize>()
            .max(1),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{axis, axis_refs, input};

    #[test]
    fn singleton_storage_axes_need_no_loop_coordinate() {
        let node = input("x", &[axis("singleton", 1), axis("hidden", 8)], Dtype::F32);
        let axes = axis_refs(&node);
        let coord = HashMap::from([(axes[1], "h".to_string())]);
        assert_eq!(Gen::new().coordinate(&HashMap::new(), axes[0]), "0");
        assert_eq!(Gen::new().buffer_offset(&axes, &coord), "h");
        assert_eq!(Gen::new().buffer_offset(&axes[..1], &HashMap::new()), "0");
    }
}
