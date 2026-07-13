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
use crate::interp::Extents;
use crate::ir::{Axis, BinOp, Dtype, MapOp, Monoid, Node, NodeKind, output_axes};

// ── fresh names ──────────────────────────────────────────────────────────────

pub struct Gen {
    pub n: usize,
    /// Storage dtypes of the inputs in scope (from [`crate::ir::input_dtypes`]):
    /// a name found here loads through [`Lang::buffer_load`] with its declared
    /// width — packed int4 nibbles, halfs — instead of the default float read.
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
    /// fused epilogue reads the fold's own result: the projection lands in
    /// a local, the epilogue's `input(out)` resolves to it, and the kernel
    /// writes the final value in one pass.
    pub local_inputs: HashMap<String, String>,
}

/// See [`Gen::lane_body`].
#[derive(Clone, Copy)]
pub struct LaneBody {
    /// The lane-distributed output axis, if any: a reduce whose subtree
    /// reads it varies per lane and must stay serial.
    pub avoid_axis: Option<Axis>,
    pub simd_width: usize,
}

impl Gen {
    pub fn new() -> Self {
        Gen {
            n: 0,
            dtypes: HashMap::new(),
            lane_body: None,
            local_inputs: HashMap::new(),
        }
    }
    pub fn fresh(&mut self, tag: &str) -> String {
        self.n += 1;
        format!("{tag}{}", self.n)
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
pub fn buffers(node: &Node) -> Vec<(&'static str, Vec<Axis>)> {
    fn go(n: &Node, out: &mut Vec<(&'static str, Vec<Axis>)>) {
        match n.as_ref() {
            NodeKind::Input { name, axes, .. } => {
                if !out.iter().any(|(nm, _)| nm == name) {
                    out.push((name, axes.clone()));
                }
            }
            NodeKind::Const { .. } | NodeKind::Iota { .. } => {}
            NodeKind::Map { inputs, .. } => inputs.iter().for_each(|i| go(i, out)),
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
pub fn offset(axes: &[Axis], coord: &HashMap<Axis, String>, ext: &Extents) -> String {
    if axes.is_empty() {
        return "0".into();
    }
    let mut terms = Vec::new();
    for (k, a) in axes.iter().enumerate() {
        let stride: usize = axes[k + 1..].iter().map(|x| ext[x]).product();
        let iv = &coord[a];
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
    /// Read element `off` of input buffer `name` stored at `dtype` width,
    /// as a float expression. `None` is the target's native float buffer.
    /// The Metal target reads halfs and unpacks int4 nibbles here; a target
    /// without typed storage must decline loudly rather than mis-read.
    fn buffer_load(&self, name: &str, off: &str, dtype: Option<Dtype>) -> String {
        match dtype {
            None => format!("{}[{off}]", san(name)),
            Some(d) => panic!("this backend has no {d:?} storage support"),
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
    coord: &HashMap<Axis, String>,
    ext: &Extents,
    g: &mut Gen,
    out: &mut Vec<String>,
) -> String {
    match node.as_ref() {
        NodeKind::Const { v } => lang.lit(*v),
        NodeKind::Iota { axis } => lang.iota_val(&coord[axis]),
        NodeKind::Input { name, axes, .. } => {
            if let Some(v) = g.local_inputs.get(*name) {
                return v.clone();
            }
            let dt = g.dtypes.get(name).copied();
            lang.buffer_load(name, &offset(axes, coord, ext), dt)
        }
        NodeKind::Map { op, inputs } => {
            let a: Vec<String> = inputs
                .iter()
                .map(|i| value(lang, i, coord, ext, g, out))
                .collect();
            lang.map_op(*op, &a)
        }
        NodeKind::Reduce { src, axis, op } => {
            let m = match op {
                BinOp::Monoid(m) => *m,
                other => panic!("codegen: reduce with {other:?} is not a monoid"),
            };
            let acc = g.fresh("acc");
            out.push(lang.scalar_decl(&acc, &lang.lit(m.identity())));
            let lv = g.fresh("r");
            let mut coord2 = coord.clone();
            coord2.insert(*axis, lv.clone());
            let mut body = Vec::new();
            let ev = value(lang, src, &coord2, ext, g, &mut body);
            // Lane-split the contraction when the scheduled emitter asked
            // for it and it is sound here: every monoid is commutative, so
            // a strided lane partition + shuffle merge equals the serial
            // fold — provided the subtree is lane-uniform (it must not read
            // the lane-distributed axis).
            let lane_split = g.lane_body.and_then(|lb| {
                let lane = lang.lane_var()?;
                let uniform = lb.avoid_axis.is_none_or(|a| !output_axes(src).contains(&a));
                let merge = lang.simd_lane_merge(&acc, m, lb.simd_width)?;
                (ext[axis] % lb.simd_width == 0 && uniform).then_some((lane, lb.simd_width, merge))
            });
            match lane_split {
                Some((lane, w, merge)) => {
                    out.push(format!(
                        "for (uint {lv} = {lane}; {lv} < {}; {lv} += {w}) {{",
                        ext[axis]
                    ));
                    out.extend(body);
                    out.push(assign(&acc, &lang.monoid(m, &acc, &ev)));
                    out.push("}".into());
                    out.extend(merge);
                }
                None => {
                    out.push(lang.for_open(&lv, ext[axis]));
                    out.extend(body);
                    out.push(assign(&acc, &lang.monoid(m, &acc, &ev)));
                    out.push("}".into());
                }
            }
            acc
        }
        NodeKind::Gather { src, index, axis } => {
            let ie = value(lang, index, coord, ext, g, out);
            let gi = g.fresh("gi");
            out.push(lang.round_index(&gi, &ie));
            let mut coord2 = coord.clone();
            coord2.insert(*axis, gi);
            value(lang, src, &coord2, ext, g, out)
        }
        NodeKind::View { src, groups } => {
            let mut coord2 = coord.clone();
            for (members, to) in groups {
                if members.len() == 1 {
                    // rename: the source axis takes the output axis's index.
                    coord2.insert(members[0], coord[to].clone());
                } else {
                    // flatten: split the merged index (first member most
                    // significant, so the last runs fastest).
                    let rem = g.fresh("rem");
                    out.push(lang.index_decl(&rem, &coord[to], true));
                    for m in members.iter().rev() {
                        let iv = g.fresh("m");
                        out.push(lang.index_decl(&iv, &format!("{rem} % {}", ext[m]), false));
                        out.push(format!("{rem} /= {};", ext[m]));
                        coord2.insert(*m, iv);
                    }
                }
            }
            value(lang, src, &coord2, ext, g, out)
        }
        // Affine reindexing: compute each mapped source axis's signed index;
        // padded reads clamp the index (so any setup statements below stay in
        // bounds) and select 0.0 when the raw index was out of range. The
        // codegen twin of the interpreter's `Reindex` arm.
        NodeKind::Reindex { src, map, padded } => {
            let mut coord2 = coord.clone();
            let mut guards: Vec<String> = Vec::new();
            for (m, terms, off) in map {
                let mut parts: Vec<String> = terms
                    .iter()
                    .map(|(coef, a)| {
                        let iv = lang.to_signed(&coord[a]);
                        if *coef == 1 {
                            iv
                        } else {
                            format!("{coef}*{iv}")
                        }
                    })
                    .collect();
                if *off != 0 {
                    parts.push(format!("({off})"));
                }
                let val = if parts.is_empty() {
                    "0".to_string()
                } else {
                    parts.join(" + ")
                };
                let ri = g.fresh("ri");
                out.push(lang.signed_index_decl(&ri, &val));
                let n = ext[m];
                let ci = g.fresh("ci");
                if *padded {
                    guards.push(format!("{ri} >= 0 && {ri} < {n}"));
                    out.push(lang.clamped_index_decl(&ci, &ri, n));
                } else {
                    out.push(lang.index_from_signed(&ci, &ri));
                }
                coord2.insert(*m, ci);
            }
            let v = value(lang, src, &coord2, ext, g, out);
            if guards.is_empty() {
                v
            } else {
                lang.select_bool(&guards.join(" && "), &v, &lang.lit(0.0))
            }
        }

        NodeKind::Scan { .. } => panic!("codegen: Scan is not implemented (sequential recurrence)"),
    }
}

/// Render a derived carrier [`Expr`]: `Item(i)` → `x[i]`, `A(i)`/`F(i)` →
/// `acc[i]`, `B(i)` → `el[i]` (identical in both targets); operators route
/// through the language's [`Lang::map_op`].
pub fn carrier_expr<L: Lang>(lang: &L, e: &Expr) -> String {
    carrier_expr_map(lang, e, &|i| format!("x[{i}]"), &|i| format!("acc[{i}]"), &|i| {
        format!("el[{i}]")
    })
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
    grid: &[Axis],
    ext: &Extents,
    g: &mut Gen,
    out: &mut Vec<String>,
) -> HashMap<Axis, String> {
    thread_grid_decode_from(lang, "gid", grid, ext, g, out)
}

/// [`thread_grid_decode`] against an arbitrary index variable — a split-
/// reduction partial kernel decodes the grid from `gid / blocks`.
pub fn thread_grid_decode_from<L: Lang>(
    _lang: &L,
    gid_var: &str,
    grid: &[Axis],
    ext: &Extents,
    g: &mut Gen,
    out: &mut Vec<String>,
) -> HashMap<Axis, String> {
    let mut coord = HashMap::new();
    for (k, &a) in grid.iter().enumerate() {
        let stride: usize = grid[k + 1..].iter().map(|x| ext[x]).product();
        let iv = format!("i_{}", g.fresh("g"));
        if stride == 1 {
            out.push(format!("uint {iv} = {gid_var} % {};", ext[&a]));
        } else {
            out.push(format!("uint {iv} = ({gid_var} / {stride}) % {};", ext[&a]));
        }
        coord.insert(a, iv);
    }
    coord
}

/// The output grid (free axes) and its flattened size for a kernel node.
pub fn grid_of(node: &Node, ext: &Extents) -> (Vec<Axis>, usize) {
    let grid = output_axes(node);
    let size = grid.iter().map(|a| ext[a]).product::<usize>().max(1);
    (grid, size)
}
