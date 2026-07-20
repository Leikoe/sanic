//! Internal lowered IR: a closed compute basis plus one structural operator, and the
//! algebraic laws each operator carries.
//!
//! Matmul, softmax, attention, silu, SSM scans and everything else are
//! compositions of the basis (constructors at the bottom of the file). That
//! is the whole trick: an analysis that handles the basis handles every
//! composite, with no special cases and no pattern library. The elementwise
//! vocabulary is a *closed* enum of scalar primitives, so the deriver can be
//! total over it — there is no open set of named ops to special-case.
//!
//! An [`Axis`] is an opaque internal index space with a size and an optional
//! diagnostic label. Labels never participate in identity. The public tensor
//! frontend resolves positional shape indices into these internal axes before
//! lowering. Because an axis carries its extent, every IR shape is derivable
//! from the graph and no side tables exist.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// A shared reference to a graph node.
pub type NodeRef = Rc<Node>;

/// One affine source-axis index:
/// `(source axis, (coefficient, output axis) terms, offset)`.
pub type AffineIndex = (Axis, Vec<(i64, Axis)>, i64);

// ── axes ─────────────────────────────────────────────────────────────────────

/// A stable, human-readable name for an axis.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AxisName(&'static str);

impl AxisName {
    pub const fn new(name: &'static str) -> Self {
        AxisName(name)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for AxisName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl fmt::Debug for AxisName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// The cardinality of an axis.
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

/// An internal index space used by the lowered IR.
///
/// `name` is diagnostic metadata only. It deliberately does not participate
/// in equality, hashing, or ordering.
#[derive(Clone, Copy)]
pub struct Axis {
    key: AxisKey,
    pub name: AxisName,
    pub extent: Extent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum AxisKey {
    /// Compatibility for the internal kernel-IR tests and constructors.
    Named(AxisName, Extent),
    /// A dimension occurrence minted inside one positional graph lowering.
    Scoped(usize),
}

/// Mint an internal axis with a diagnostic label.
pub fn axis(name: &'static str, extent: impl Into<Extent>) -> Axis {
    let extent = extent.into();
    Axis {
        key: AxisKey::Named(AxisName::new(name), extent),
        name: AxisName::new(name),
        extent,
    }
}

/// Mint a compiler-scoped axis. The caller owns the namespace and must use a
/// distinct `id` for every logical dimension in the lowered graph.
pub(crate) const fn scoped_axis(id: usize, name: &'static str, extent: Extent) -> Axis {
    Axis {
        key: AxisKey::Scoped(id),
        name: AxisName::new(name),
        extent,
    }
}

impl Axis {
    /// The concrete extent required by core interpretation and compilation.
    pub fn extent(self) -> usize {
        match self.extent {
            Extent::Static(value) => value,
            Extent::Dynamic => panic!("axis `{}` has a dynamic extent", self.name),
        }
    }

    pub const fn is_dynamic(self) -> bool {
        matches!(self.extent, Extent::Dynamic)
    }
}

impl PartialEq for Axis {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Axis {}

impl Hash for Axis {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl PartialOrd for Axis {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Axis {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl fmt::Debug for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Axis")
            .field("key", &self.key)
            .field("name", &self.name)
            .field("extent", &self.extent)
            .finish()
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
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
    /// Index-carrying maximum: the reduce's VALUE is the position of the
    /// first maximum along the axis. Associative over `(max, idx)` pairs
    /// with first-max-wins tie-breaking — a tuple monoid, not a scalar one,
    /// so it ships its own two-slot carrier (like `AffineCompose`). This is
    /// what makes argmax / top-k selection ONE fold instead of a
    /// max-then-indicator-sum pair (which also differs on ties).
    ArgMax,
    /// One projection of a k-best selection: the `rank`-th largest value
    /// (`idx: false`) or its position (`idx: true`), first-max-wins on ties.
    /// Semantically, sorted lists of length ≤ k form a tuple monoid under
    /// merge-take-k. The current carrier implements only its singleton-insert
    /// streaming path: each rank is one kernel over the raw scores, but
    /// tree/split execution must decline until combine merges two full lists.
    TopK {
        k: u8,
        rank: u8,
        idx: bool,
    },
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
    /// bfloat16 — the top 16 bits of an f32, so a load widens by
    /// `bits << 16` (exact, full f32 range). It is the dtype most
    /// checkpoints ship in, and it is BYTE-IDENTICAL to those bytes, so a
    /// bf16 input can bind zero-copy straight from the file (unlike f16,
    /// which needs a host conversion pass).
    BF16,
    I8,
    I4,
}

impl Dtype {
    /// Bytes per element (f64 so a half-byte int4 is representable).
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

// ── nodes ────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum Node {
    /// Raw data. Depends on no axis. `dtype` is the buffer's declared storage
    /// width — pricing information for the planner, not a change of
    /// semantics.
    Input {
        name: &'static str,
        axes: Vec<Axis>,
        dtype: Dtype,
    },
    /// A literal scalar. No axes, no storage.
    Const { v: f64 },
    /// The index along an axis, as a value (0, 1, 2, …). Free to compute —
    /// this is what makes aranges and causal masks cost no memory traffic.
    Iota { axis: Axis },
    /// Elementwise / broadcast application of a basis op.
    Map { op: MapOp, inputs: Vec<NodeRef> },
    /// Folds `axis` with `op`; the result no longer carries `axis`.
    Reduce { src: NodeRef, axis: Axis, op: BinOp },
    /// Prefix recurrence over `axis`; foldable iff `op` is associative.
    Scan { src: NodeRef, axis: Axis, op: BinOp },
    /// `src[index[...]]` — data-dependent access along `axis`.
    Gather {
        src: NodeRef,
        index: NodeRef,
        axis: Axis,
    },
    /// Reindexing — the one structural operator: no computation, no copy,
    /// just new names for the same values. Each group maps source axes to
    /// one fresh output axis: `[s] → t` is a rename, `[h, dv] → dm` is a
    /// flatten. A consumed source axis goes out of scope above the view; a
    /// grouped output inherits the joined structure of its members.
    View {
        src: NodeRef,
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
        src: NodeRef,
        /// (source axis, terms over output axes, constant offset).
        map: Vec<AffineIndex>,
        padded: bool,
    },
}

// ── constructors ─────────────────────────────────────────────────────────────

/// An input with its storage dtype. The dtype is required because storage
/// width is part of an input declaration, not a target-dependent default.
pub fn input(name: &'static str, axes: &[Axis], dtype: Dtype) -> NodeRef {
    Rc::new(Node::Input {
        name,
        axes: axes.to_vec(),
        dtype,
    })
}

/// Every input's storage dtype, by name (first declaration wins).
pub fn input_dtypes(node: &NodeRef) -> Vec<(&'static str, Dtype)> {
    fn go(
        n: &NodeRef,
        out: &mut Vec<(&'static str, Dtype)>,
        seen: &mut std::collections::HashSet<*const Node>,
    ) {
        // Walk each DAG node once — exponential on shared backward graphs
        // otherwise (dtypes dedup by name, so first-reach is equivalent).
        if !seen.insert(std::rc::Rc::as_ptr(n)) {
            return;
        }
        match n.as_ref() {
            Node::Input { name, dtype, .. } => {
                if !out.iter().any(|(m, _)| m == name) {
                    out.push((name, *dtype));
                }
            }
            Node::Const { .. } | Node::Iota { .. } => {}
            Node::Map { inputs, .. } => inputs.iter().for_each(|i| go(i, out, seen)),
            Node::Reduce { src, .. }
            | Node::Scan { src, .. }
            | Node::View { src, .. }
            | Node::Reindex { src, .. } => go(src, out, seen),
            Node::Gather { src, index, .. } => {
                go(src, out, seen);
                go(index, out, seen);
            }
        }
    }
    let mut out = Vec::new();
    go(node, &mut out, &mut std::collections::HashSet::new());
    out
}

pub fn konst(v: f64) -> NodeRef {
    Rc::new(Node::Const { v })
}

pub fn iota(axis: Axis) -> NodeRef {
    Rc::new(Node::Iota { axis })
}

pub fn map(op: MapOp, inputs: Vec<NodeRef>) -> NodeRef {
    debug_assert_eq!(op.arity(), inputs.len(), "{op:?} arity");
    Rc::new(Node::Map { op, inputs })
}

pub fn reduce(src: NodeRef, axis: Axis, op: BinOp) -> NodeRef {
    Rc::new(Node::Reduce { src, axis, op })
}

pub fn scan(src: NodeRef, axis: Axis, op: BinOp) -> NodeRef {
    Rc::new(Node::Scan { src, axis, op })
}

pub fn gather(src: NodeRef, index: NodeRef, axis: Axis) -> NodeRef {
    Rc::new(Node::Gather { src, index, axis })
}

pub fn view(src: NodeRef, groups: Vec<(Vec<Axis>, Axis)>) -> NodeRef {
    Rc::new(Node::View { src, groups })
}

/// The same values under a different axis variable — `X[s,·]` seen as `X[t,·]`.
pub fn rename(src: NodeRef, from: Axis, to: Axis) -> NodeRef {
    view(src, vec![(vec![from], to)])
}

/// Merge a group of axes into one (`[h, dv] → dm`); extent(to) is the
/// product of the members' extents.
pub fn flatten(src: NodeRef, group: &[Axis], to: Axis) -> NodeRef {
    view(src, vec![(group.to_vec(), to)])
}

pub fn reindex(src: NodeRef, map: Vec<AffineIndex>, padded: bool) -> NodeRef {
    Rc::new(Node::Reindex { src, map, padded })
}

/// A contiguous slice along one axis: `out[i] = src[i + start]`. `to` is a
/// fresh, shorter axis (its declared extent is the slice length).
pub fn slice(src: NodeRef, from: Axis, to: Axis, start: usize) -> NodeRef {
    reindex(src, vec![(from, vec![(1, to)], start as i64)], false)
}

/// Zero-pad along one axis: `out[i] = src[i − lo]`, 0.0 outside. `to` is a
/// fresh axis whose declared extent is `lo + extent(from) + hi`.
pub fn pad(src: NodeRef, from: Axis, to: Axis, lo: usize) -> NodeRef {
    reindex(src, vec![(from, vec![(1, to)], -(lo as i64))], true)
}

/// Split one axis into `(outer, inner)` — the inverse of [`flatten`]:
/// `out[o, i] = src[o·extent(inner) + i]`.
pub fn split(src: NodeRef, from: Axis, outer: Axis, inner: Axis) -> NodeRef {
    reindex(
        src,
        vec![(from, vec![(inner.extent() as i64, outer), (1, inner)], 0)],
        false,
    )
}

/// Sliding windows along `from`: output `(out, k)` reads
/// `src[out·stride + k·dilation]` — convolution and pooling are a `window`
/// followed by a [`reduce`] over `k`. `out` may be an *existing* axis to
/// share an index space (sliding-window attention rides the query axis).
/// Compose with [`pad`] for SAME-style windows that hang off the ends.
pub fn window(
    src: NodeRef,
    from: Axis,
    out: Axis,
    k: Axis,
    stride: usize,
    dilation: usize,
) -> NodeRef {
    reindex(
        src,
        vec![(from, vec![(stride as i64, out), (dilation as i64, k)], 0)],
        false,
    )
}

// ── shape and axis queries ───────────────────────────────────────────────────

impl Node {
    /// The axes of this node's output, in shape order.
    ///
    /// A reduce drops its axis, a gather replaces the indexed axis with the
    /// index shape, and movement nodes remap the source shape.
    pub fn shape(&self) -> Vec<Axis> {
        shape_memo(self, &mut std::collections::HashMap::new())
    }
}

/// The number of elements a node's output holds — the product of its shape's
/// extents (`1` for a scalar). This is the size a materialized buffer for the
/// node needs; a stage's dispatch grid may be smaller (a packed fold writes
/// several elements per thread), so allocation keys off THIS, never the grid.
pub fn volume(node: &NodeRef) -> usize {
    node.shape()
        .iter()
        .map(|a| a.extent())
        .product::<usize>()
        .max(1)
}

fn shape_memo(
    node: &Node,
    cache: &mut std::collections::HashMap<*const Node, Rc<Vec<Axis>>>,
) -> Vec<Axis> {
    (*shape_rc(node, cache)).clone()
}

fn shape_rc(
    node: &Node,
    cache: &mut std::collections::HashMap<*const Node, Rc<Vec<Axis>>>,
) -> Rc<Vec<Axis>> {
    let key = std::ptr::from_ref(node);
    if let Some(v) = cache.get(&key) {
        return v.clone();
    }
    let union = |mut acc: Vec<Axis>, more: &[Axis]| {
        for &a in more {
            if !acc.contains(&a) {
                acc.push(a);
            }
        }
        acc
    };
    let out: Vec<Axis> = match node {
        Node::Input { axes, .. } => axes.clone(),
        Node::Const { .. } => Vec::new(),
        Node::Iota { axis } => vec![*axis],
        Node::Map { inputs, .. } => inputs.iter().fold(Vec::new(), |acc, i| {
            let ia = shape_rc(i, cache);
            union(acc, &ia)
        }),
        Node::Reduce { src, axis, .. } => shape_rc(src, cache)
            .iter()
            .copied()
            .filter(|a| a != axis)
            .collect(),
        Node::Scan { src, .. } => (*shape_rc(src, cache)).clone(),
        Node::Gather { src, index, axis } => {
            let kept: Vec<Axis> = shape_rc(src, cache)
                .iter()
                .copied()
                .filter(|a| a != axis)
                .collect();
            let ia = shape_rc(index, cache);
            union(kept, &ia)
        }
        Node::View { src, groups } => {
            let mut out: Vec<Axis> = Vec::new();
            for a in shape_rc(src, cache).iter().copied() {
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
        Node::Reindex { src, map, .. } => {
            let mut out: Vec<Axis> = Vec::new();
            for a in shape_rc(src, cache).iter().copied() {
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
    };
    let out = Rc::new(out);
    cache.insert(key, out.clone());
    out
}

/// One node's structure with children identified by POINTER. When the
/// children are already canonical, two nodes get the same key exactly when
/// they are structurally identical — so keying is O(children), never
/// O(subtree). This is what [`canonicalize_many`] and the simplifier's CSE
/// both dedup by.
pub(crate) fn shallow_key(n: &NodeRef) -> String {
    let p = |c: &NodeRef| Rc::as_ptr(c) as usize;
    match n.as_ref() {
        Node::Const { v } => format!("C{}", v.to_bits()),
        Node::Input { name, axes, dtype } => format!("I{name}{axes:?}{dtype:?}"),
        Node::Iota { axis } => format!("O{axis:?}"),
        Node::Map { op, inputs } => {
            format!("M{op:?}{:?}", inputs.iter().map(p).collect::<Vec<_>>())
        }
        Node::Reduce { src, axis, op } => format!("R{op:?}{axis:?}.{}", p(src)),
        Node::Scan { src, axis, op } => format!("S{op:?}{axis:?}.{}", p(src)),
        Node::Gather { src, index, axis } => format!("G{axis:?}.{}.{}", p(src), p(index)),
        Node::View { src, groups } => format!("V{groups:?}.{}", p(src)),
        Node::Reindex { src, map, padded } => format!("X{map:?}{padded}.{}", p(src)),
    }
}

/// Rebuild `roots` with maximal sharing: separately constructed but
/// structurally identical subgraphs collapse into ONE node with several
/// consumers. One canonical table spans all roots, so equal subtrees shared
/// *between* roots (a forward value recomputed in the backward) merge too.
/// After this pass structural equality IS pointer equality — the invariant
/// the pointer-keyed caches in [`crate::derive`] and [`crate::partition`]
/// read sharing through. A subtree that is already canonical keeps its
/// original `Rc`, so identities held elsewhere stay valid.
pub fn canonicalize_many(roots: &[NodeRef]) -> Vec<NodeRef> {
    let mut canon = Canonicalizer::default();
    roots.iter().map(|root| canon.tree(root)).collect()
}

/// The canonical table behind [`canonicalize_many`], usable past the entry
/// pass: the partitioner keeps the table it canonicalized its roots with and
/// routes every node it constructs afterwards through [`Canonicalizer::shallow`],
/// so a rebuilt structural twin resolves to the FIRST-SEEN node (the
/// original) and pointer-keyed maps keep deduplicating.
#[derive(Default)]
pub struct Canonicalizer {
    canonical: std::collections::HashMap<String, NodeRef>,
    memo: std::collections::HashMap<*const Node, NodeRef>,
}

impl Canonicalizer {
    /// Canonicalize a whole subtree, bottom-up. Keeps the ORIGINAL `Rc` for
    /// any part that was already canonical, so identities held elsewhere
    /// stay valid.
    pub fn tree(&mut self, node: &NodeRef) -> NodeRef {
        if let Some(n) = self.memo.get(&Rc::as_ptr(node)) {
            return n.clone();
        }
        let rebuilt = match node.as_ref() {
            Node::Input { .. } | Node::Const { .. } | Node::Iota { .. } => node.clone(),
            Node::Map { op, inputs } => {
                let ins: Vec<NodeRef> = inputs.iter().map(|i| self.tree(i)).collect();
                if ins.iter().zip(inputs).all(|(a, b)| Rc::ptr_eq(a, b)) {
                    node.clone()
                } else {
                    map(*op, ins)
                }
            }
            Node::Reduce { src, axis, op } => {
                let s = self.tree(src);
                if Rc::ptr_eq(&s, src) { node.clone() } else { reduce(s, *axis, *op) }
            }
            Node::Scan { src, axis, op } => {
                let s = self.tree(src);
                if Rc::ptr_eq(&s, src) { node.clone() } else { scan(s, *axis, *op) }
            }
            Node::Gather { src, index, axis } => {
                let s = self.tree(src);
                let i = self.tree(index);
                if Rc::ptr_eq(&s, src) && Rc::ptr_eq(&i, index) {
                    node.clone()
                } else {
                    gather(s, i, *axis)
                }
            }
            Node::View { src, groups } => {
                let s = self.tree(src);
                if Rc::ptr_eq(&s, src) { node.clone() } else { view(s, groups.clone()) }
            }
            Node::Reindex { src, map, padded } => {
                let s = self.tree(src);
                if Rc::ptr_eq(&s, src) {
                    node.clone()
                } else {
                    reindex(s, map.clone(), *padded)
                }
            }
        };
        let out = self.shallow(rebuilt);
        self.memo.insert(Rc::as_ptr(node), out.clone());
        out
    }

    /// Canonical node for `node`, whose children must already be canonical:
    /// the first node seen with this structure wins, later twins collapse
    /// onto it.
    pub fn shallow(&mut self, node: NodeRef) -> NodeRef {
        self.canonical
            .entry(shallow_key(&node))
            .or_insert(node)
            .clone()
    }
}

/// Every `Input` leaf and its axes, in first-seen order. May repeat a name if
/// the same tensor is reached along several paths — callers dedup as needed.
/// `Const` and `Iota` carry no storage and are not reported.
pub fn input_axes(node: &NodeRef) -> Vec<(&'static str, Vec<Axis>)> {
    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::new();
    collect_input_axes(node, &mut out, &mut seen);
    out
}

fn collect_input_axes(
    node: &NodeRef,
    out: &mut Vec<(&'static str, Vec<Axis>)>,
    seen: &mut std::collections::HashSet<*const Node>,
) {
    // Walk each DAG node once. Without this the per-PATH result is exponential
    // in a backward graph's sharing (both time AND output size — a diamond
    // doubles it per level). Consumers dedup by name and read only byte volume
    // (`plan`), which is invariant under the rename/flatten relabeling below, so
    // keeping the first-reached occurrence is equivalent.
    if !seen.insert(Rc::as_ptr(node)) {
        return;
    }
    match node.as_ref() {
        Node::Input { name, axes, .. } => out.push((name, axes.clone())),
        Node::Const { .. } | Node::Iota { .. } => {}
        Node::Map { inputs, .. } => {
            for i in inputs {
                collect_input_axes(i, out, seen);
            }
        }
        Node::Reduce { src, .. } | Node::Scan { src, .. } => {
            collect_input_axes(src, out, seen);
        }
        // A gather reads the selected source slices, not the whole indexed
        // axis. Report source inputs under the gather's consumer shape so the
        // planner prices `[sequence, hidden]` embedding rows rather than a
        // resident `[vocab, hidden]` table.
        Node::Gather { src, index, axis } => {
            let before = out.len();
            collect_input_axes(src, out, seen);
            let index_axes = index.shape();
            for (_, axes) in out[before..].iter_mut() {
                let mut mapped = Vec::new();
                for &source_axis in axes.iter() {
                    if source_axis == *axis {
                        for &index_axis in &index_axes {
                            if !mapped.contains(&index_axis) {
                                mapped.push(index_axis);
                            }
                        }
                    } else if !mapped.contains(&source_axis) {
                        mapped.push(source_axis);
                    }
                }
                *axes = mapped;
            }
            collect_input_axes(index, out, seen);
        }
        // A view relabels: report the inputs below under the viewed names, so
        // shape-based modeling (SRAM, traffic) sees what the consumer reads.
        Node::View { src, groups } => {
            let before = out.len();
            collect_input_axes(src, out, seen);
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
        Node::Reindex { src, map, .. } => {
            let before = out.len();
            collect_input_axes(src, out, seen);
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
pub fn leaf_names(node: &NodeRef) -> Vec<&'static str> {
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
pub fn all_axes(node: &NodeRef) -> Vec<Axis> {
    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::new();
    collect_axes(node, &mut out, &mut seen);
    out
}

fn collect_axes(
    node: &NodeRef,
    out: &mut Vec<Axis>,
    seen: &mut std::collections::HashSet<*const Node>,
) {
    // A DAG-shared node reached along several paths is walked ONCE: its axes are
    // already in `out`. Without this, a backward graph's heavy sharing makes the
    // walk exponential in the sharing depth (measured: a 1-block transformer's
    // attention gradient took 66s to partition, almost all of it here).
    if !seen.insert(Rc::as_ptr(node)) {
        return;
    }
    let push = |a: Axis, out: &mut Vec<Axis>| {
        if !out.contains(&a) {
            out.push(a);
        }
    };
    match node.as_ref() {
        Node::Input { axes, .. } => {
            for &a in axes {
                push(a, out);
            }
        }
        Node::Const { .. } => {}
        Node::Iota { axis } => push(*axis, out),
        Node::Map { inputs, .. } => {
            for i in inputs {
                collect_axes(i, out, seen);
            }
        }
        Node::Reduce { src, axis, .. } | Node::Scan { src, axis, .. } => {
            push(*axis, out);
            collect_axes(src, out, seen);
        }
        Node::Gather { src, index, axis } => {
            push(*axis, out);
            collect_axes(src, out, seen);
            collect_axes(index, out, seen);
        }
        Node::View { src, groups } => {
            for (_, to) in groups {
                push(*to, out);
            }
            collect_axes(src, out, seen);
        }
        Node::Reindex { src, map, .. } => {
            for (_, terms, _) in map {
                for (_, t) in terms {
                    push(*t, out);
                }
            }
            collect_axes(src, out, seen);
        }
    }
}

// ── derived operators — compositions, not primitives ─────────────────────────

/// `matmul(A, B; contract=k) = Reduce(Map(×, A, B), k, Add)`.
pub fn matmul(a: NodeRef, b: NodeRef, contract: Axis) -> NodeRef {
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
pub fn softmax(x: NodeRef, k: Axis) -> NodeRef {
    let m = reduce(x.clone(), k, BinOp::Monoid(Monoid::Max));
    let e = map(MapOp::Exp, vec![map(MapOp::Sub, vec![x, m])]);
    let s = reduce(e.clone(), k, BinOp::Monoid(Monoid::Add));
    map(MapOp::Div, vec![e, s])
}

/// `attention(Q,K,V) = matmul(softmax(matmul(Q,K; d), k), V; k)`.
pub fn attention(q: NodeRef, k_in: NodeRef, v: NodeRef, d: Axis, k: Axis) -> NodeRef {
    let scores = matmul(q, k_in, d);
    let weights = softmax(scores, k);
    matmul(weights, v, k)
}

/// `silu(x) = x · sigmoid(x) = x / (1 + exp(−x))` — a composition, so it
/// fuses into the lift of a downstream reduction with no special case.
pub fn silu(x: NodeRef) -> NodeRef {
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
pub fn causal_mask(query: Axis, key: Axis) -> NodeRef {
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
pub fn ssm_scan(params: NodeRef, t: Axis) -> NodeRef {
    scan(params, t, BinOp::AffineCompose)
}

/// A `tanh`-RNN `h_t = tanh(W·h_{t−1} + x_t)`: a non-associative step, so its
/// time axis is serial. The engine must refuse to fold it.
pub fn tanh_rnn(x: NodeRef, t: Axis) -> NodeRef {
    scan(map(MapOp::Tanh, vec![x]), t, BinOp::NonAssoc("tanh_recur"))
}

/// Embedding lookup: a gather along the vocabulary axis.
pub fn embedding(table: NodeRef, ids: NodeRef, axis: Axis) -> NodeRef {
    gather(table, ids, axis)
}

/// `1.0` where `iota(axis) == v` (an integer-valued node), else `0.0` — a
/// computed one-hot: equality from two `Lt`s, no Eq op needed. The KV-cache
/// row write, scatter's collision test and topk's mask are all this.
pub fn one_hot(axis: Axis, v: NodeRef) -> NodeRef {
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

/// The index of the FIRST maximum along `axis`, as a value — one
/// index-carrying fold ([`BinOp::ArgMax`]): the `(max, idx)` tuple monoid
/// with first-max-wins ties, matching every framework's argmax convention.
pub fn argmax(x: NodeRef, axis: Axis) -> NodeRef {
    reduce(x, axis, BinOp::ArgMax)
}

/// Top-k along `axis`: k `(value, index)` pairs, largest first. Each projection
/// is one [`BinOp::TopK`] streaming fold over the raw scores, with no
/// mask-the-winner chain between ranks and first-max-wins ties across the
/// whole selection. The current carrier inserts one item at a time and is not
/// valid for tree/split execution until it implements the semantic
/// merge-take-k operation over two full lists. A full sort stays out of the
/// supported fragment on purpose: it is a data-movement network, not a fold,
/// and nothing in an inference pipeline needs one.
pub fn topk(x: NodeRef, axis: Axis, k: usize) -> Vec<(NodeRef, NodeRef)> {
    let q = |rank: usize, idx: bool| {
        reduce(
            x.clone(),
            axis,
            BinOp::TopK {
                k: k as u8,
                rank: rank as u8,
                idx,
            },
        )
    };
    (0..k).map(|r| (q(r, false), q(r, true))).collect()
}

/// ALL k ranks of the top-k selection as ONE tensor over a fresh rank axis
/// `rk` — and, downstream, one KERNEL. Spelled as Σ_r onehot(rk = r)·rank_r:
/// the per-rank reduces share one streamed source, so the deriver dedups
/// their k-best lists into a single set of slots, and the rank one-hots
/// never touch the streamed axis, so they evaluate at PROJECT time — each
/// grid point of `rk` selects its slot from the shared list. Eight rank
/// kernels per MoE layer become one.
pub fn topk_all(x: NodeRef, axis: Axis, k: usize, rk: Axis, idx: bool) -> NodeRef {
    let mut sum: Option<NodeRef> = None;
    for r in 0..k {
        let q = reduce(
            x.clone(),
            axis,
            BinOp::TopK {
                k: k as u8,
                rank: r as u8,
                idx,
            },
        );
        let term = map(MapOp::Mul, vec![one_hot(rk, konst(r as f64)), q]);
        sum = Some(match sum {
            None => term,
            Some(s) => map(MapOp::Add, vec![s, term]),
        });
    }
    sum.expect("k >= 1")
}

/// Scatter-add — the inverse of [`gather`], add-combining collisions:
/// `out[to,·] = Σ_from (index[from] == to) · src[from,·]`. Dense as a graph
/// (O(n·m) — a one-hot contraction); a device backend may later emit atomics,
/// but the *semantics* need no new node. Add-combine is chosen because it is
/// order-free (deterministic under any parallel schedule) and is exactly
/// gather's backward, so embedding gradients fall out of this constructor.
pub fn scatter_add(src: NodeRef, index: NodeRef, from: Axis, to: Axis) -> NodeRef {
    reduce(
        map(MapOp::Mul, vec![one_hot(to, index), src]),
        from,
        BinOp::Monoid(Monoid::Add),
    )
}
