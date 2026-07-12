//! The IR: a closed compute basis plus one structural operator, and the
//! algebraic laws each operator carries.
//!
//! Matmul, softmax, attention, silu, SSM scans and everything else are
//! compositions of the basis (constructors at the bottom of the file). That
//! is the whole trick: an analysis that handles the basis handles every
//! composite, with no special cases and no pattern library. The elementwise
//! vocabulary is a *closed* enum of scalar primitives, so the deriver can be
//! total over it — there is no open set of named ops to special-case.
//!
//! Axes are variables, not strings: an [`Axis`] is a fresh integer identity
//! with a label used only for printing. `Reduce` binds its axis; `View`
//! re-binds names (rename) or merges index spaces (flatten) without moving
//! data. Because axes are host-language values, Rust's own lexical scoping
//! is the axis scoping — pass the same `Axis` where two tensors share an
//! index space, mint a fresh one where they don't.

use std::fmt;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};

pub type Node = Rc<NodeKind>;

// ── axes ─────────────────────────────────────────────────────────────────────

/// An axis variable. Identity is the id; the label is only for printing —
/// two calls to `axis("s")` make two *different* variables.
#[derive(Clone, Copy, Debug)]
pub struct Axis {
    id: u32,
    label: &'static str,
}

static NEXT_AXIS: AtomicU32 = AtomicU32::new(0);

/// Mint a fresh axis variable with a printing label.
pub fn axis(label: &'static str) -> Axis {
    Axis {
        id: NEXT_AXIS.fetch_add(1, Ordering::Relaxed),
        label,
    }
}

impl Axis {
    pub fn label(self) -> &'static str {
        self.label
    }
}

impl PartialEq for Axis {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Axis {}
impl std::hash::Hash for Axis {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl PartialOrd for Axis {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Axis {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

// ── operator laws ────────────────────────────────────────────────────────────

/// The associative reductions. Every variant is a monoid — associative, with
/// the identity below. These laws are the only thing the engine trusts:
/// declare a false one and the derived kernels are wrong. So the enum is tiny.
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

    /// True for the "plus" of its semiring — the op a constant factor
    /// distributes over (`Σ c·xᵢ = c·Σ xᵢ`). This is what lets a normalizer
    /// be pulled out of a reduction and applied once at the end.
    pub fn is_additive(self) -> bool {
        matches!(self, Monoid::Add | Monoid::LogSumExp)
    }
}

/// The operator a `Reduce` or `Scan` folds with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Monoid(Monoid),
    /// Composition of affine maps `h ↦ A·h + b`. Associative with an identity,
    /// so an SSM / linear recurrence streams even though it is not a scalar
    /// monoid.
    AffineCompose,
    /// A non-associative step, e.g. `tanh(W·h + x)`. No legal fold exists; an
    /// axis governed by one of these is strictly serial.
    NonAssoc(&'static str),
}

impl BinOp {
    /// Associative with an identity — i.e. foldable.
    pub fn is_monoid(self) -> bool {
        !matches!(self, BinOp::NonAssoc(_))
    }

    pub fn is_additive(self) -> bool {
        matches!(self, BinOp::Monoid(m) if m.is_additive())
    }
}

// ── the closed elementwise basis ─────────────────────────────────────────────

/// The scalar primitives. This set is CLOSED: every elementwise computation
/// in a graph is a composition of these, so the deriver and the emitters can
/// be total over it. (tinygrad proves the same idea at scale: a dozen ALU
/// ops, everything else derived.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapOp {
    // binary
    Add,
    Sub,
    Mul,
    Div,
    Max,
    Min,
    /// `a < b`, producing 1.0 / 0.0 — with `Where`, this is how masks are
    /// *computed* instead of loaded.
    Lt,
    // unary
    Neg,
    Recip,
    Exp,
    Log,
    Sqrt,
    Tanh,
    Sin,
    Cos,
    // ternary
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

    /// Does this op preserve linearity in its arguments? (Scaling and
    /// addition do; `exp` and friends do not.)
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

// ── storage dtypes ───────────────────────────────────────────────────────────

/// The STORAGE width of an input buffer — what a load actually moves over the
/// bus, which is what the cost model must price (int4 weights read 8× fewer
/// bytes than f32). Semantically the oracle still computes in f64: dtype is a
/// statement about bytes, not about values, until real byte storage lands in
/// the buffer model. Quantized *arithmetic* is already expressible without
/// it (dequant fuses into the GEMM lift); this prices the bandwidth win.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dtype {
    F64,
    F32,
    F16,
    I8,
    I4,
}

impl Dtype {
    /// Bytes per element (f64 so a half-byte int4 is representable).
    pub fn bytes(self) -> f64 {
        match self {
            Dtype::F64 => 8.0,
            Dtype::F32 => 4.0,
            Dtype::F16 => 2.0,
            Dtype::I8 => 1.0,
            Dtype::I4 => 0.5,
        }
    }
}

// ── nodes ────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum NodeKind {
    /// Raw data. Depends on no axis. `dtype` is the buffer's declared storage
    /// width — pricing information for the planner, not a change of
    /// semantics; `None` prices at the device's compute dtype.
    Input {
        name: &'static str,
        axes: Vec<Axis>,
        dtype: Option<Dtype>,
    },
    /// A literal scalar. No axes, no storage.
    Const { v: f64 },
    /// The index along an axis, as a value (0, 1, 2, …). Free to compute —
    /// this is what makes aranges and causal masks cost no memory traffic.
    Iota { axis: Axis },
    /// Elementwise / broadcast application of a basis op.
    Map { op: MapOp, inputs: Vec<Node> },
    /// Folds `axis` with `op`; the result no longer carries `axis`.
    Reduce { src: Node, axis: Axis, op: BinOp },
    /// Prefix recurrence over `axis`; foldable iff `op` is associative.
    Scan { src: Node, axis: Axis, op: BinOp },
    /// `src[index[...]]` — data-dependent access along `axis`.
    Gather { src: Node, index: Node, axis: Axis },
    /// Reindexing — the one structural operator: no computation, no copy,
    /// just new names for the same values. Each group maps source axes to
    /// one fresh output axis: `[s] → t` is a rename, `[h, dv] → dm` is a
    /// flatten. A consumed source axis goes out of scope above the view; a
    /// grouped output inherits the joined structure of its members.
    View {
        src: Node,
        groups: Vec<(Vec<Axis>, Axis)>,
    },
    /// Affine reindexing — the movement vocabulary past rename/flatten. Each
    /// mapped source axis reads the index `Σ coef·i(out_axis) + offset`
    /// (signed arithmetic); with `padded`, an out-of-range read is 0.0
    /// instead of an error. Like `View`, no computation and no copy — only
    /// index arithmetic. Slice, zero-pad, reshape-split, sliding windows and
    /// reversal are all this one operator (constructors below). A mapped
    /// source axis goes out of scope above the node; the term axes are the
    /// (possibly shared) output axes that drive it.
    Reindex {
        src: Node,
        /// (source axis, terms over output axes, constant offset).
        map: Vec<(Axis, Vec<(i64, Axis)>, i64)>,
        padded: bool,
    },
}

// ── constructors ─────────────────────────────────────────────────────────────

pub fn input(name: &'static str, axes: &[Axis]) -> Node {
    Rc::new(NodeKind::Input {
        name,
        axes: axes.to_vec(),
        dtype: None,
    })
}

/// An input with an explicit storage dtype — how quantized weights tell the
/// planner they cost 1 (or 0.5) bytes of bandwidth per element.
pub fn input_dt(name: &'static str, axes: &[Axis], dtype: Dtype) -> Node {
    Rc::new(NodeKind::Input {
        name,
        axes: axes.to_vec(),
        dtype: Some(dtype),
    })
}

/// Every input that DECLARES a storage dtype, by name (first declaration
/// wins). Inputs without one price at the device's compute dtype.
pub fn input_dtypes(node: &Node) -> Vec<(&'static str, Dtype)> {
    fn go(n: &Node, out: &mut Vec<(&'static str, Dtype)>) {
        match n.as_ref() {
            NodeKind::Input {
                name,
                dtype: Some(d),
                ..
            } => {
                if !out.iter().any(|(m, _)| m == name) {
                    out.push((name, *d));
                }
            }
            NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => {}
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

pub fn konst(v: f64) -> Node {
    Rc::new(NodeKind::Const { v })
}

pub fn iota(axis: Axis) -> Node {
    Rc::new(NodeKind::Iota { axis })
}

pub fn map(op: MapOp, inputs: Vec<Node>) -> Node {
    debug_assert_eq!(op.arity(), inputs.len(), "{op:?} arity");
    Rc::new(NodeKind::Map { op, inputs })
}

pub fn reduce(src: Node, axis: Axis, op: BinOp) -> Node {
    Rc::new(NodeKind::Reduce { src, axis, op })
}

pub fn scan(src: Node, axis: Axis, op: BinOp) -> Node {
    Rc::new(NodeKind::Scan { src, axis, op })
}

pub fn gather(src: Node, index: Node, axis: Axis) -> Node {
    Rc::new(NodeKind::Gather { src, index, axis })
}

pub fn view(src: Node, groups: Vec<(Vec<Axis>, Axis)>) -> Node {
    Rc::new(NodeKind::View { src, groups })
}

/// The same values under a different axis variable — `X[s,·]` seen as `X[t,·]`.
pub fn rename(src: Node, from: Axis, to: Axis) -> Node {
    view(src, vec![(vec![from], to)])
}

/// Merge a group of axes into one (`[h, dv] → dm`); extent(to) is the
/// product of the members' extents.
pub fn flatten(src: Node, group: &[Axis], to: Axis) -> Node {
    view(src, vec![(group.to_vec(), to)])
}

pub fn reindex(src: Node, map: Vec<(Axis, Vec<(i64, Axis)>, i64)>, padded: bool) -> Node {
    Rc::new(NodeKind::Reindex { src, map, padded })
}

/// A contiguous slice along one axis: `out[i] = src[i + start]`. `to` is a
/// fresh, shorter axis (its declared extent is the slice length).
pub fn slice(src: Node, from: Axis, to: Axis, start: usize) -> Node {
    reindex(src, vec![(from, vec![(1, to)], start as i64)], false)
}

/// Zero-pad along one axis: `out[i] = src[i − lo]`, 0.0 outside. `to` is a
/// fresh axis whose declared extent is `lo + extent(from) + hi`.
pub fn pad(src: Node, from: Axis, to: Axis, lo: usize) -> Node {
    reindex(src, vec![(from, vec![(1, to)], -(lo as i64))], true)
}

/// Split one axis into `(outer, inner)` — the inverse of [`flatten`]:
/// `out[o, i] = src[o·inner_extent + i]`. `inner_extent` must equal the
/// declared extent of `inner`.
pub fn split(src: Node, from: Axis, outer: Axis, inner: Axis, inner_extent: usize) -> Node {
    reindex(
        src,
        vec![(from, vec![(inner_extent as i64, outer), (1, inner)], 0)],
        false,
    )
}

/// Sliding windows along `from`: output `(out, k)` reads
/// `src[out·stride + k·dilation]` — convolution and pooling are a `window`
/// followed by a [`reduce`] over `k`. `out` may be an *existing* axis to
/// share an index space (sliding-window attention rides the query axis).
/// Compose with [`pad`] for SAME-style windows that hang off the ends.
pub fn window(src: Node, from: Axis, out: Axis, k: Axis, stride: usize, dilation: usize) -> Node {
    reindex(
        src,
        vec![(from, vec![(stride as i64, out), (dilation as i64, k)], 0)],
        false,
    )
}

// ── shape and axis queries ───────────────────────────────────────────────────

/// The axes of a node's output — its shape, inferred. A reduce drops its
/// axis; a gather swaps the indexed axis for the index's axes; a view
/// remaps. This is what tells the deriver which free axes an accumulator
/// slot spans.
pub fn output_axes(node: &Node) -> Vec<Axis> {
    let union = |mut acc: Vec<Axis>, more: Vec<Axis>| {
        for a in more {
            if !acc.contains(&a) {
                acc.push(a);
            }
        }
        acc
    };
    match node.as_ref() {
        NodeKind::Input { axes, .. } => axes.clone(),
        NodeKind::Const { .. } => Vec::new(),
        NodeKind::Iota { axis } => vec![*axis],
        NodeKind::Map { inputs, .. } => inputs
            .iter()
            .fold(Vec::new(), |acc, i| union(acc, output_axes(i))),
        NodeKind::Reduce { src, axis, .. } => {
            output_axes(src).into_iter().filter(|a| a != axis).collect()
        }
        NodeKind::Scan { src, .. } => output_axes(src),
        NodeKind::Gather { src, index, axis } => {
            let kept = output_axes(src).into_iter().filter(|a| a != axis).collect();
            union(kept, output_axes(index))
        }
        NodeKind::View { src, groups } => {
            let mut out: Vec<Axis> = Vec::new();
            for a in output_axes(src) {
                if let Some((_, to)) = groups.iter().find(|(members, _)| members.contains(&a)) {
                    if !out.contains(to) {
                        out.push(*to);
                    }
                } else if !out.contains(&a) {
                    out.push(a);
                }
            }
            out
        }
        // A mapped source axis is replaced, in place, by the axes its index
        // is computed from; unmapped axes pass through.
        NodeKind::Reindex { src, map, .. } => {
            let mut out: Vec<Axis> = Vec::new();
            for a in output_axes(src) {
                if let Some((_, terms, _)) = map.iter().find(|(m, _, _)| *m == a) {
                    for (_, t) in terms {
                        if !out.contains(t) {
                            out.push(*t);
                        }
                    }
                } else if !out.contains(&a) {
                    out.push(a);
                }
            }
            out
        }
    }
}

/// Every `Input` leaf and its axes, in first-seen order. May repeat a name if
/// the same tensor is reached along several paths — callers dedup as needed.
/// `Const` and `Iota` carry no storage and are not reported.
pub fn input_axes(node: &Node) -> Vec<(&'static str, Vec<Axis>)> {
    let mut out = Vec::new();
    collect_input_axes(node, &mut out);
    out
}

fn collect_input_axes(node: &Node, out: &mut Vec<(&'static str, Vec<Axis>)>) {
    match node.as_ref() {
        NodeKind::Input { name, axes, .. } => out.push((name, axes.clone())),
        NodeKind::Const { .. } | NodeKind::Iota { .. } => {}
        NodeKind::Map { inputs, .. } => {
            for i in inputs {
                collect_input_axes(i, out);
            }
        }
        NodeKind::Reduce { src, .. } | NodeKind::Scan { src, .. } => {
            collect_input_axes(src, out);
        }
        NodeKind::Gather { src, index, .. } => {
            collect_input_axes(src, out);
            collect_input_axes(index, out);
        }
        // A view relabels: report the inputs below under the viewed names, so
        // shape-based modeling (SRAM, traffic) sees what the consumer reads.
        NodeKind::View { src, groups } => {
            let before = out.len();
            collect_input_axes(src, out);
            for (_, axes) in out[before..].iter_mut() {
                let mut mapped: Vec<Axis> = Vec::new();
                for a in axes.iter() {
                    match groups.iter().find(|(members, _)| members.contains(a)) {
                        Some((_, to)) => {
                            if !mapped.contains(to) {
                                mapped.push(*to);
                            }
                        }
                        None => {
                            if !mapped.contains(a) {
                                mapped.push(*a);
                            }
                        }
                    }
                }
                *axes = mapped;
            }
        }
        // Same for an affine reindex: a mapped axis is reported as the axes
        // driving it (a windowed input reads as out×k — the naive traffic,
        // which is what the consumer-side model should price).
        NodeKind::Reindex { src, map, .. } => {
            let before = out.len();
            collect_input_axes(src, out);
            for (_, axes) in out[before..].iter_mut() {
                let mut mapped: Vec<Axis> = Vec::new();
                for a in axes.iter() {
                    match map.iter().find(|(m, _, _)| m == a) {
                        Some((_, terms, _)) => {
                            for (_, t) in terms {
                                if !mapped.contains(t) {
                                    mapped.push(*t);
                                }
                            }
                        }
                        None => {
                            if !mapped.contains(a) {
                                mapped.push(*a);
                            }
                        }
                    }
                }
                *axes = mapped;
            }
        }
    }
}

/// Every `Input` leaf's name, deduped, in first-seen order.
pub fn leaf_names(node: &Node) -> Vec<&'static str> {
    let mut out: Vec<&'static str> = Vec::new();
    for (name, _) in input_axes(node) {
        if !out.contains(&name) {
            out.push(name);
        }
    }
    out
}

/// Every axis the graph touches, in first-seen order — the set the structure
/// map classifies.
pub fn all_axes(node: &Node) -> Vec<Axis> {
    let mut out = Vec::new();
    collect_axes(node, &mut out);
    out
}

fn collect_axes(node: &Node, out: &mut Vec<Axis>) {
    let push = |a: Axis, out: &mut Vec<Axis>| {
        if !out.contains(&a) {
            out.push(a);
        }
    };
    match node.as_ref() {
        NodeKind::Input { axes, .. } => {
            for &a in axes {
                push(a, out);
            }
        }
        NodeKind::Const { .. } => {}
        NodeKind::Iota { axis } => push(*axis, out),
        NodeKind::Map { inputs, .. } => {
            for i in inputs {
                collect_axes(i, out);
            }
        }
        NodeKind::Reduce { src, axis, .. } | NodeKind::Scan { src, axis, .. } => {
            push(*axis, out);
            collect_axes(src, out);
        }
        NodeKind::Gather { src, index, axis } => {
            push(*axis, out);
            collect_axes(src, out);
            collect_axes(index, out);
        }
        NodeKind::View { src, groups } => {
            for (_, to) in groups {
                push(*to, out);
            }
            collect_axes(src, out);
        }
        NodeKind::Reindex { src, map, .. } => {
            for (_, terms, _) in map {
                for (_, t) in terms {
                    push(*t, out);
                }
            }
            collect_axes(src, out);
        }
    }
}

// ── derived operators — compositions, not primitives ─────────────────────────

/// `matmul(A, B; contract=k) = Reduce(Map(×, A, B), k, Add)`.
pub fn matmul(a: Node, b: Node, contract: Axis) -> Node {
    reduce(
        map(MapOp::Mul, vec![a, b]),
        contract,
        BinOp::Monoid(Monoid::Add),
    )
}

/// Softmax as the textbook dataflow — max, shift, exp, sum, divide — built
/// from the basis; no fused special form:
/// ```text
/// m = Reduce(x, k, Max); e = Exp(x − m); s = Reduce(e, k, Add); e / s
/// ```
pub fn softmax(x: Node, k: Axis) -> Node {
    let m = reduce(x.clone(), k, BinOp::Monoid(Monoid::Max));
    let e = map(MapOp::Exp, vec![map(MapOp::Sub, vec![x, m])]);
    let s = reduce(e.clone(), k, BinOp::Monoid(Monoid::Add));
    map(MapOp::Div, vec![e, s])
}

/// `attention(Q,K,V) = matmul(softmax(matmul(Q,K; d), k), V; k)`.
pub fn attention(q: Node, k_in: Node, v: Node, d: Axis, k: Axis) -> Node {
    let scores = matmul(q, k_in, d);
    let weights = softmax(scores, k);
    matmul(weights, v, k)
}

/// `silu(x) = x · sigmoid(x) = x / (1 + exp(−x))` — a composition, so it
/// fuses into the lift of a downstream reduction with no special case.
pub fn silu(x: Node) -> Node {
    let sig = map(
        MapOp::Recip,
        vec![map(
            MapOp::Add,
            vec![
                konst(1.0),
                map(MapOp::Exp, vec![map(MapOp::Neg, vec![x.clone()])]),
            ],
        )],
    );
    map(MapOp::Mul, vec![x, sig])
}

/// A causal mask, computed from indices instead of loaded from memory:
/// `key ≤ query ? 0 : −LARGE`. Costs no traffic — the comparison fuses into
/// whatever lift consumes it. The mask is a large *finite* negative, not −∞:
/// an all-masked block would otherwise feed `exp(−∞ − −∞) = NaN` into the
/// online-softmax rescale — the same edge real flash kernels guard against.
pub fn causal_mask(query: Axis, key: Axis) -> Node {
    map(
        MapOp::Where,
        vec![
            map(MapOp::Lt, vec![iota(query), iota(key)]), // key index > query index
            konst(-1e30),
            konst(0.0),
        ],
    )
}

/// A linear / SSM recurrence `h_t = A_t·h_{t−1} + b_t`: a `Scan` whose step is
/// affine-map composition — associative, so it streams.
pub fn ssm_scan(params: Node, t: Axis) -> Node {
    scan(params, t, BinOp::AffineCompose)
}

/// A `tanh`-RNN `h_t = tanh(W·h_{t−1} + x_t)`: a non-associative step, so its
/// time axis is serial. The engine must refuse to fold it.
pub fn tanh_rnn(x: Node, t: Axis) -> Node {
    scan(map(MapOp::Tanh, vec![x]), t, BinOp::NonAssoc("tanh_recur"))
}

/// Embedding lookup: a gather along the vocabulary axis.
pub fn embedding(table: Node, ids: Node, axis: Axis) -> Node {
    gather(table, ids, axis)
}

/// `1.0` where `iota(axis) == v` (an integer-valued node), else `0.0` — a
/// computed one-hot: equality from two `Lt`s, no Eq op needed. The KV-cache
/// row write, scatter's collision test and topk's mask are all this.
pub fn one_hot(axis: Axis, v: Node) -> Node {
    let it = iota(axis);
    map(
        MapOp::Sub,
        vec![
            map(
                MapOp::Sub,
                vec![konst(1.0), map(MapOp::Lt, vec![it.clone(), v.clone()])],
            ),
            map(MapOp::Lt, vec![v, it]),
        ],
    )
}

/// The index of the maximum along `axis`, as a value: `Σ i·[x == max]`.
/// Exact for a unique maximum; ties sum their indices (undefined) — random
/// continuous data never ties, and a caller that must break ties can add an
/// index-scaled epsilon.
pub fn argmax(x: Node, axis: Axis) -> Node {
    let m = reduce(x.clone(), axis, BinOp::Monoid(Monoid::Max));
    let hit = map(
        MapOp::Sub,
        vec![konst(1.0), map(MapOp::Lt, vec![x, m])],
    ); // [x == max], since nothing exceeds the max
    reduce(
        map(MapOp::Mul, vec![iota(axis), hit]),
        axis,
        BinOp::Monoid(Monoid::Add),
    )
}

/// Top-k along `axis` by repeated (max, mask-the-winner): k `(value, index)`
/// pairs, largest first. Each round masks exactly the previous winner's
/// position (a computed [`one_hot`] of its argmax), so equal values elsewhere
/// survive; a tie *within* one round is undefined, as for [`argmax`]. This is
/// a decomposition, not a new operator — sampling and MoE routing need small
/// k, and each round is an ordinary fold the deriver already streams. A full
/// sort stays out of the supported fragment on purpose: it is a
/// data-movement network, not a fold, and nothing in an inference pipeline
/// needs one.
pub fn topk(x: Node, axis: Axis, k: usize) -> Vec<(Node, Node)> {
    let mut cur = x;
    let mut out = Vec::new();
    for _ in 0..k {
        let v = reduce(cur.clone(), axis, BinOp::Monoid(Monoid::Max));
        let i = argmax(cur.clone(), axis);
        out.push((v, i.clone()));
        cur = map(
            MapOp::Where,
            vec![one_hot(axis, i), konst(f64::NEG_INFINITY), cur],
        );
    }
    out
}

/// Scatter-add — the inverse of [`gather`], add-combining collisions:
/// `out[to,·] = Σ_from (index[from] == to) · src[from,·]`. Dense as a graph
/// (O(n·m) — a one-hot contraction); a device backend may later emit atomics,
/// but the *semantics* need no new node. Add-combine is chosen because it is
/// order-free (deterministic under any parallel schedule) and is exactly
/// gather's backward, so embedding gradients fall out of this constructor.
pub fn scatter_add(src: Node, index: Node, from: Axis, to: Axis) -> Node {
    reduce(
        map(MapOp::Mul, vec![one_hot(to, index), src]),
        from,
        BinOp::Monoid(Monoid::Add),
    )
}
