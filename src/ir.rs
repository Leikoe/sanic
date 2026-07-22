//! Public positional tensor IR.
//!
//! A dimension is identified only by its position in a node's ordered shape.
//! [`Axis`] carries display metadata and an extent; it is never a graph-wide
//! variable or an equality key. Compiler passes annotate these same nodes
//! with ephemeral `(node, dimension)` loop metadata; there is no second graph
//! IR.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub use crate::scalar::{Dtype, Extent, MapOp, Monoid};

/// A shared reference to an immutable graph node.
pub type NodeRef = Rc<Node>;

/// Metadata for one position in a node's shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Axis {
    pub name: &'static str,
    pub extent: Extent,
}

impl Axis {
    pub const fn new(name: &'static str, extent: Extent) -> Self {
        Axis { name, extent }
    }

    /// The concrete extent required by the current interpreter and compiler.
    pub fn extent(self) -> usize {
        match self.extent {
            Extent::Static(value) => value,
            Extent::Dynamic => panic!("axis `{}` has a dynamic extent", self.name),
        }
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

/// A compact axis descriptor.
pub fn axis(name: &'static str, extent: impl Into<Extent>) -> Axis {
    Axis {
        name,
        extent: extent.into(),
    }
}

/// One source-dimension affine index:
/// `(source dimension, (coefficient, output dimension) terms, offset)`.
pub type AffineIndex = (usize, Vec<(i64, usize)>, i64);

/// One output dimension of a storage-preserving view.
///
/// `sources` are positions in the source shape. A singleton insertion has no
/// source positions and must have extent one.
#[derive(Clone, Debug)]
pub struct ViewDim {
    pub sources: Vec<usize>,
    pub axis: Axis,
}

#[derive(Debug)]
pub enum Node {
    Input {
        name: &'static str,
        shape: Vec<Axis>,
        dtype: Dtype,
    },
    Const {
        v: f64,
    },
    Iota {
        axis: Axis,
    },
    /// Coordinate along one positional dimension of `src`, broadcast over
    /// the rest of `src`'s shape.
    Coordinate {
        src: NodeRef,
        dim: usize,
    },
    Map {
        op: MapOp,
        inputs: Vec<NodeRef>,
    },
    Reduce {
        src: NodeRef,
        dim: usize,
        op: Monoid,
    },
    Scan {
        src: NodeRef,
        dim: usize,
        op: Monoid,
    },
    Gather {
        src: NodeRef,
        index: NodeRef,
        dim: usize,
    },
    View {
        src: NodeRef,
        dims: Vec<ViewDim>,
    },
    Reindex {
        src: NodeRef,
        shape: Vec<Axis>,
        map: Vec<AffineIndex>,
        padded: bool,
    },
}

/// One logical tensor dimension, identified by an output position of a
/// particular immutable node. It is compiler metadata, never stored in a
/// graph node.
///
/// Elementwise maps and descriptor-preserving views reuse an input occurrence
/// where possible. A relabel, flatten, or affine reindex introduces an
/// occurrence owned by that structural node. Equality is pointer identity
/// plus position; `name` and `extent` are diagnostics only.
#[derive(Clone, Copy)]
pub struct AxisRef {
    owner: *const Node,
    pub dim: usize,
    pub name: &'static str,
    pub extent: Extent,
}

impl AxisRef {
    pub fn extent(self) -> usize {
        match self.extent {
            Extent::Static(value) => value,
            Extent::Dynamic => panic!("axis `{}` has a dynamic extent", self.name),
        }
    }
}

impl PartialEq for AxisRef {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner && self.dim == other.dim
    }
}

impl Eq for AxisRef {}

impl Hash for AxisRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.owner.hash(state);
        self.dim.hash(state);
    }
}

impl fmt::Display for AxisRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

impl fmt::Debug for AxisRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AxisRef")
            .field("node", &(self.owner as usize))
            .field("dim", &self.dim)
            .field("name", &self.name)
            .field("extent", &self.extent)
            .finish()
    }
}

/// A dimension selector accepted at compiler-analysis boundaries. Public code
/// normally passes `usize`/`isize`; compiler passes may pass an already
/// resolved [`AxisRef`] when they need to follow one occurrence through the
/// graph. A selector is always relative to the specific node supplied at the
/// call site; display descriptors never participate in resolution.
pub trait AxisSelector: Copy {
    fn resolve_axis(self, node: &NodeRef, op: &str) -> Option<AxisRef>;
}

impl AxisSelector for AxisRef {
    fn resolve_axis(self, _node: &NodeRef, _op: &str) -> Option<AxisRef> {
        Some(self)
    }
}

impl AxisSelector for usize {
    fn resolve_axis(self, node: &NodeRef, op: &str) -> Option<AxisRef> {
        Some(axis_refs(node)[self.resolve(&node.shape(), op)])
    }
}

impl AxisSelector for isize {
    fn resolve_axis(self, node: &NodeRef, op: &str) -> Option<AxisRef> {
        Some(axis_refs(node)[self.resolve(&node.shape(), op)])
    }
}

impl Node {
    /// Ordered output shape. Positions, not `Axis` values, identify dimensions.
    pub fn shape(&self) -> Vec<Axis> {
        shape_memo(self, &mut std::collections::HashMap::new())
    }
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
    if let Some(shape) = cache.get(&key) {
        return shape.clone();
    }
    let shape = match node {
        Node::Input { shape, .. } => shape.clone(),
        Node::Const { .. } => Vec::new(),
        Node::Iota { axis } => vec![*axis],
        Node::Coordinate { src, dim } => {
            let shape = shape_memo(src, cache);
            assert_dim(&shape, *dim, "coordinate");
            shape
        }
        Node::Map { inputs, .. } => broadcast_shapes(
            &inputs
                .iter()
                .map(|input| shape_memo(input, cache))
                .collect::<Vec<_>>(),
            "map",
        ),
        Node::Reduce { src, dim, .. } => {
            let mut shape = shape_memo(src, cache);
            assert_dim(&shape, *dim, "reduce");
            shape.remove(*dim);
            shape
        }
        Node::Scan { src, dim, .. } => {
            let shape = shape_memo(src, cache);
            assert_dim(&shape, *dim, "scan");
            shape
        }
        Node::Gather { src, index, dim } => {
            let source = shape_memo(src, cache);
            let index = shape_memo(index, cache);
            assert_dim(&source, *dim, "gather");
            let mut output = Vec::with_capacity(source.len() - 1 + index.len());
            output.extend_from_slice(&source[..*dim]);
            output.extend(index);
            output.extend_from_slice(&source[*dim + 1..]);
            output
        }
        Node::View { dims, .. } => dims.iter().map(|dim| dim.axis).collect(),
        Node::Reindex { shape, .. } => shape.clone(),
    };
    let shape = Rc::new(shape);
    cache.insert(key, shape.clone());
    shape
}

/// Resolved logical dimensions of `node`, in output-shape order.
///
/// This is the compiler's axis-resolution pass in lazy form. It annotates the
/// existing positional DAG; it does not rebuild it into another IR.
pub fn axis_refs(node: &NodeRef) -> Vec<AxisRef> {
    Resolver::default().axes(node)
}

/// Cached positional resolution for compiler passes over one retained DAG.
/// This is pass metadata only; nodes remain the sole IR.
#[derive(Default)]
pub(crate) struct Resolver {
    shapes: std::collections::HashMap<*const Node, Rc<Vec<Axis>>>,
    axes: std::collections::HashMap<*const Node, Rc<Vec<AxisRef>>>,
}

impl Resolver {
    pub(crate) fn axes(&mut self, node: &NodeRef) -> Vec<AxisRef> {
        (*axis_refs_rc(node, &mut self.axes, &mut self.shapes)).clone()
    }

    pub(crate) fn source_axis(&mut self, src: &NodeRef, dim: usize) -> AxisRef {
        self.axes(src)[dim]
    }

    pub(crate) fn map_input_axis(
        &mut self,
        map_node: &NodeRef,
        input: &NodeRef,
        axis: AxisRef,
    ) -> AxisRef {
        let output_axes = self.axes(map_node);
        let Some(output_dim) = output_axes.iter().position(|candidate| *candidate == axis) else {
            return axis;
        };
        let output_shape = shape_memo(map_node, &mut self.shapes);
        let input_shape = shape_memo(input, &mut self.shapes);
        let Some(input_dim) = output_dim.checked_sub(output_shape.len() - input_shape.len()) else {
            return axis;
        };
        if input_shape[input_dim].extent == Extent::Static(1)
            && output_shape[output_dim].extent != Extent::Static(1)
        {
            axis
        } else {
            self.axes(input)[input_dim]
        }
    }

    pub(crate) fn view_groups(&mut self, node: &NodeRef) -> Vec<(Vec<AxisRef>, AxisRef)> {
        let Node::View { src, dims } = node.as_ref() else {
            return Vec::new();
        };
        let source = self.axes(src);
        let output = self.axes(node);
        dims.iter()
            .enumerate()
            .filter(|(output_dim, dim)| {
                !dim.sources.is_empty()
                    && !matches!(dim.sources.as_slice(), [source_dim]
                        if source[*source_dim] == output[*output_dim])
            })
            .map(|(output_dim, dim)| {
                (
                    dim.sources
                        .iter()
                        .map(|&source_dim| source[source_dim])
                        .collect(),
                    output[output_dim],
                )
            })
            .collect()
    }

    pub(crate) fn resolved_reindex(
        &mut self,
        node: &NodeRef,
    ) -> Vec<(AxisRef, Vec<(i64, AxisRef)>, i64)> {
        let Node::Reindex { src, map, .. } = node.as_ref() else {
            return Vec::new();
        };
        let source = self.axes(src);
        let output = self.axes(node);
        map.iter()
            .map(|(source_dim, terms, offset)| {
                (
                    source[*source_dim],
                    terms
                        .iter()
                        .map(|(coefficient, output_dim)| (*coefficient, output[*output_dim]))
                        .collect(),
                    *offset,
                )
            })
            .collect()
    }
}

fn own_axis(node: &NodeRef, dim: usize, descriptor: Axis) -> AxisRef {
    AxisRef {
        owner: Rc::as_ptr(node),
        dim,
        name: descriptor.name,
        extent: descriptor.extent,
    }
}

fn axis_refs_rc(
    node: &NodeRef,
    cache: &mut std::collections::HashMap<*const Node, Rc<Vec<AxisRef>>>,
    shape_cache: &mut std::collections::HashMap<*const Node, Rc<Vec<Axis>>>,
) -> Rc<Vec<AxisRef>> {
    let key = Rc::as_ptr(node);
    if let Some(axes) = cache.get(&key) {
        return axes.clone();
    }

    let descriptors = shape_memo(node, shape_cache);
    let axes = match node.as_ref() {
        Node::Input { shape, .. } => shape
            .iter()
            .copied()
            .enumerate()
            .map(|(dim, axis)| own_axis(node, dim, axis))
            .collect(),
        Node::Const { .. } => Vec::new(),
        Node::Iota { axis } => vec![own_axis(node, 0, *axis)],
        Node::Coordinate { src, .. } => (*axis_refs_rc(src, cache, shape_cache)).clone(),
        Node::Map { inputs, .. } => {
            let rank = descriptors.len();
            (0..rank)
                .map(|output_dim| {
                    let mut fallback = None;
                    for input in inputs {
                        let input_shape = shape_memo(input, shape_cache);
                        let Some(input_dim) =
                            output_dim.checked_sub(rank.saturating_sub(input_shape.len()))
                        else {
                            continue;
                        };
                        let candidate = axis_refs_rc(input, cache, shape_cache)[input_dim];
                        fallback = fallback.or(Some(candidate));
                        if input_shape[input_dim].extent != Extent::Static(1) {
                            return candidate;
                        }
                    }
                    fallback.unwrap_or_else(|| own_axis(node, output_dim, descriptors[output_dim]))
                })
                .collect()
        }
        Node::Reduce { src, dim, .. } => {
            let mut axes = (*axis_refs_rc(src, cache, shape_cache)).clone();
            axes.remove(*dim);
            axes
        }
        Node::Scan { src, .. } => (*axis_refs_rc(src, cache, shape_cache)).clone(),
        Node::Gather { src, index, dim } => {
            let mut axes = (*axis_refs_rc(src, cache, shape_cache)).clone();
            axes.splice(
                *dim..=*dim,
                axis_refs_rc(index, cache, shape_cache).iter().copied(),
            );
            axes
        }
        Node::View { src, dims } => {
            let source_axes = axis_refs_rc(src, cache, shape_cache);
            let source_shape = shape_memo(src, shape_cache);
            dims.iter()
                .enumerate()
                .map(|(output_dim, view_dim)| match view_dim.sources.as_slice() {
                    [source_dim] if view_dim.axis == source_shape[*source_dim] => {
                        source_axes[*source_dim]
                    }
                    _ => own_axis(node, output_dim, view_dim.axis),
                })
                .collect()
        }
        Node::Reindex { shape, .. } => shape
            .iter()
            .copied()
            .enumerate()
            .map(|(dim, axis)| own_axis(node, dim, axis))
            .collect(),
    };
    let axes = Rc::new(axes);
    cache.insert(key, axes.clone());
    axes
}

/// Axis consumed by a reduction, scan, or gather.
pub fn source_axis(src: &NodeRef, dim: usize) -> AxisRef {
    Resolver::default().source_axis(src, dim)
}

/// Translate one map-output axis to the corresponding occurrence in `input`.
/// An axis consumed inside an input is not an output dimension of the map; in
/// that case it passes through unchanged so recursive analysis can find it.
pub fn map_input_axis(map_node: &NodeRef, input: &NodeRef, axis: AxisRef) -> AxisRef {
    Resolver::default().map_input_axis(map_node, input, axis)
}

/// Flatten/view relations in resolved axis coordinates.
pub fn view_groups(node: &NodeRef) -> Vec<(Vec<AxisRef>, AxisRef)> {
    Resolver::default().view_groups(node)
}

/// Positional affine reindexing expressed in resolved axis coordinates.
pub fn resolved_reindex(node: &NodeRef) -> Vec<(AxisRef, Vec<(i64, AxisRef)>, i64)> {
    Resolver::default().resolved_reindex(node)
}

/// Every logical dimension occurrence reachable from `node`, in first-seen
/// order. Ordinary elementwise/pass-through axes deduplicate naturally;
/// flatten and reindex boundaries introduce new occurrences.
pub fn all_axis_refs(node: &NodeRef) -> Vec<AxisRef> {
    fn walk(
        node: &NodeRef,
        out: &mut Vec<AxisRef>,
        seen: &mut std::collections::HashSet<*const Node>,
    ) {
        if !seen.insert(Rc::as_ptr(node)) {
            return;
        }
        for axis in axis_refs(node) {
            if !out.contains(&axis) {
                out.push(axis);
            }
        }
        match node.as_ref() {
            Node::Input { .. } | Node::Const { .. } | Node::Iota { .. } => {}
            Node::Coordinate { src, .. }
            | Node::Reduce { src, .. }
            | Node::Scan { src, .. }
            | Node::View { src, .. }
            | Node::Reindex { src, .. } => walk(src, out, seen),
            Node::Map { inputs, .. } => {
                for input in inputs {
                    walk(input, out, seen);
                }
            }
            Node::Gather { src, index, .. } => {
                walk(src, out, seen);
                walk(index, out, seen);
            }
        }
    }
    let mut axes = Vec::new();
    walk(node, &mut axes, &mut std::collections::HashSet::new());
    axes
}

fn assert_dim(shape: &[Axis], dim: usize, op: &str) {
    assert!(
        dim < shape.len(),
        "{op}: dimension {dim} is out of range for rank {}",
        shape.len()
    );
}

fn broadcast_shapes(shapes: &[Vec<Axis>], op: &str) -> Vec<Axis> {
    let rank = shapes.iter().map(Vec::len).max().unwrap_or(0);
    let mut output = Vec::with_capacity(rank);
    for output_dim in 0..rank {
        let mut selected: Option<Axis> = None;
        for shape in shapes {
            let Some(input_dim) = output_dim.checked_sub(rank - shape.len()) else {
                continue;
            };
            let candidate = shape[input_dim];
            selected = Some(match selected {
                None => candidate,
                Some(current) => match (current.extent, candidate.extent) {
                    (Extent::Static(1), _) => candidate,
                    (_, Extent::Static(1)) => current,
                    (left, right) if left == right => current,
                    _ => panic!(
                        "{op}: dimensions with extents {:?} and {:?} cannot broadcast",
                        current.extent, candidate.extent
                    ),
                },
            });
        }
        output.push(selected.expect("broadcast output dimension has no source"));
    }
    output
}

/// Conversion for positional operator arguments.
///
/// `usize` addresses a dimension from the front and `isize` additionally
/// supports negative indices from the back.
pub trait Dimension {
    fn resolve(self, shape: &[Axis], op: &str) -> usize;
}

impl Dimension for usize {
    fn resolve(self, shape: &[Axis], op: &str) -> usize {
        assert_dim(shape, self, op);
        self
    }
}

impl Dimension for isize {
    fn resolve(self, shape: &[Axis], op: &str) -> usize {
        let rank = shape.len() as isize;
        let dim = if self < 0 { rank + self } else { self };
        assert!(
            (0..rank).contains(&dim),
            "{op}: dimension {self} is out of range for rank {rank}"
        );
        dim as usize
    }
}

pub fn input(name: impl Into<String>, shape: impl AsRef<[Axis]>, dtype: Dtype) -> NodeRef {
    let name = Box::leak(name.into().into_boxed_str());
    Rc::new(Node::Input {
        name,
        shape: shape.as_ref().to_vec(),
        dtype,
    })
}

pub fn konst(v: f64) -> NodeRef {
    Rc::new(Node::Const { v })
}

/// A tensor of ones with the same positional shape as `source`, expressed
/// from scalar maps so it preserves the source dimension occurrences.
pub fn ones_like(source: NodeRef) -> NodeRef {
    map(
        MapOp::Add,
        vec![map(MapOp::Mul, vec![source, konst(0.0)]), konst(1.0)],
    )
}

pub fn iota(axis: Axis) -> NodeRef {
    Rc::new(Node::Iota { axis })
}

/// The positional index along `dim`, broadcast over the shape of `src`.
pub fn coordinate(src: NodeRef, dim: impl Dimension) -> NodeRef {
    let dim = dim.resolve(&src.shape(), "coordinate");
    Rc::new(Node::Coordinate { src, dim })
}

pub fn map(op: MapOp, inputs: Vec<NodeRef>) -> NodeRef {
    assert_eq!(op.arity(), inputs.len(), "{op:?} arity");
    // Validate broadcasting at construction so malformed graphs fail locally.
    let _ = broadcast_shapes(
        &inputs.iter().map(|input| input.shape()).collect::<Vec<_>>(),
        op.name(),
    );
    Rc::new(Node::Map { op, inputs })
}

pub fn reduce(src: NodeRef, dim: impl Dimension, op: Monoid) -> NodeRef {
    let dim = dim.resolve(&src.shape(), "reduce");
    Rc::new(Node::Reduce { src, dim, op })
}

pub fn scan(src: NodeRef, dim: impl Dimension, op: Monoid) -> NodeRef {
    let dim = dim.resolve(&src.shape(), "scan");
    Rc::new(Node::Scan { src, dim, op })
}

pub fn gather(src: NodeRef, index: NodeRef, dim: impl Dimension) -> NodeRef {
    let dim = dim.resolve(&src.shape(), "gather");
    Rc::new(Node::Gather { src, index, dim })
}

pub fn positional_view(src: NodeRef, dims: Vec<ViewDim>) -> NodeRef {
    let source = src.shape();
    let mut consumed = vec![false; source.len()];
    for dim in &dims {
        if dim.sources.is_empty() {
            assert_eq!(
                dim.axis.extent,
                Extent::Static(1),
                "view: a source-free dimension must be a static singleton"
            );
        }
        let mut product = Some(1usize);
        for &source_dim in &dim.sources {
            assert_dim(&source, source_dim, "view");
            assert!(
                !std::mem::replace(&mut consumed[source_dim], true),
                "view: source dimension {source_dim} is consumed more than once"
            );
            product = match (product, source[source_dim].extent) {
                (Some(acc), Extent::Static(value)) => acc.checked_mul(value),
                _ => None,
            };
        }
        if let (Some(expected), Extent::Static(actual)) = (product, dim.axis.extent) {
            assert_eq!(
                actual, expected,
                "view: output extent {actual} does not match source product {expected}; source={source:?}, dim={dim:?}"
            );
        }
    }
    assert!(
        consumed.into_iter().all(|value| value),
        "view: every source dimension must be consumed exactly once"
    );
    Rc::new(Node::View { src, dims })
}

/// Swap two dimensions without copying storage.
pub fn transpose(src: NodeRef, dim0: impl Dimension, dim1: impl Dimension) -> NodeRef {
    let source = src.shape();
    let dim0 = dim0.resolve(&source, "transpose");
    let dim1 = dim1.resolve(&source, "transpose");
    let dims = (0..source.len())
        .map(|output_dim| {
            let source_dim = if output_dim == dim0 {
                dim1
            } else if output_dim == dim1 {
                dim0
            } else {
                output_dim
            };
            ViewDim {
                sources: vec![source_dim],
                axis: source[source_dim],
            }
        })
        .collect();
    positional_view(src, dims)
}

pub trait Dimensions {
    fn resolve_all(self, shape: &[Axis], op: &str) -> Vec<usize>;
}

impl Dimensions for &[usize] {
    fn resolve_all(self, shape: &[Axis], op: &str) -> Vec<usize> {
        self.iter().map(|&dim| dim.resolve(shape, op)).collect()
    }
}

pub fn flatten<G: Dimensions>(src: NodeRef, group: G, to: Axis) -> NodeRef {
    let source = src.shape();
    let group = group.resolve_all(&source, "flatten");
    assert!(!group.is_empty(), "flatten: group cannot be empty");
    let first = *group.iter().min().unwrap();
    let mut dims = Vec::new();
    for (index, &axis) in source.iter().enumerate() {
        if index == first {
            dims.push(ViewDim {
                sources: group.clone(),
                axis: to,
            });
        } else if !group.contains(&index) {
            dims.push(ViewDim {
                sources: vec![index],
                axis,
            });
        }
    }
    positional_view(src, dims)
}

pub fn positional_reindex(
    src: NodeRef,
    shape: Vec<Axis>,
    map: Vec<AffineIndex>,
    padded: bool,
) -> NodeRef {
    let source = src.shape();
    assert_eq!(
        map.len(),
        source.len(),
        "reindex: every source dimension must be mapped exactly once"
    );
    let mut seen = vec![false; source.len()];
    for (source_dim, terms, _) in &map {
        assert_dim(&source, *source_dim, "reindex source");
        assert!(
            !std::mem::replace(&mut seen[*source_dim], true),
            "reindex: source dimension {source_dim} is mapped more than once"
        );
        for &(_, output_dim) in terms {
            assert_dim(&shape, output_dim, "reindex output");
        }
    }
    Rc::new(Node::Reindex {
        src,
        shape,
        map,
        padded,
    })
}

pub fn split(src: NodeRef, from: impl Dimension, outer: Axis, inner: Axis) -> NodeRef {
    let source = src.shape();
    let from = from.resolve(&source, "split");
    let mut shape = source.clone();
    shape.splice(from..=from, [outer, inner]);
    let map = source
        .iter()
        .enumerate()
        .map(|(source_dim, _)| {
            if source_dim == from {
                (
                    source_dim,
                    vec![(inner.extent() as i64, from), (1, from + 1)],
                    0,
                )
            } else {
                let output_dim = if source_dim < from {
                    source_dim
                } else {
                    source_dim + 1
                };
                (source_dim, vec![(1, output_dim)], 0)
            }
        })
        .collect();
    positional_reindex(src, shape, map, false)
}

/// Rebuild `roots` with maximal sharing: separately constructed but
/// structurally identical subtrees collapse into ONE node. One table spans
/// all roots, and a subtree that is already canonical keeps its original
/// `Rc`.
pub fn canonicalize_many(roots: &[NodeRef]) -> Vec<NodeRef> {
    let mut canonicalizer = Canonicalizer::default();
    roots.iter().map(|root| canonicalizer.tree(root)).collect()
}

/// Canonical table retained by passes that rebuild nodes after entry.
#[derive(Default)]
pub struct Canonicalizer {
    canonical: std::collections::HashMap<String, NodeRef>,
    memo: std::collections::HashMap<*const Node, NodeRef>,
}

impl Canonicalizer {
    pub fn tree(&mut self, node: &NodeRef) -> NodeRef {
        canonicalize_node(node, &mut self.canonical, &mut self.memo)
    }

    /// Canonicalize one node whose children are already canonical.
    pub fn shallow(&mut self, node: NodeRef) -> NodeRef {
        self.canonical
            .entry(shallow_key(&node))
            .or_insert(node)
            .clone()
    }
}

fn canonicalize_node(
    node: &NodeRef,
    canonical: &mut std::collections::HashMap<String, NodeRef>,
    memo: &mut std::collections::HashMap<*const Node, NodeRef>,
) -> NodeRef {
    if let Some(n) = memo.get(&Rc::as_ptr(node)) {
        return n.clone();
    }
    // Children first; keep the ORIGINAL `Rc` when nothing beneath changed.
    let rebuilt = match node.as_ref() {
        Node::Input { .. } | Node::Const { .. } | Node::Iota { .. } => node.clone(),
        Node::Map { op, inputs } => {
            let ins: Vec<NodeRef> = inputs
                .iter()
                .map(|i| canonicalize_node(i, canonical, memo))
                .collect();
            if ins.iter().zip(inputs).all(|(a, b)| Rc::ptr_eq(a, b)) {
                node.clone()
            } else {
                Rc::new(Node::Map {
                    op: *op,
                    inputs: ins,
                })
            }
        }
        Node::Coordinate { src, dim } => {
            let s = canonicalize_node(src, canonical, memo);
            if Rc::ptr_eq(&s, src) {
                node.clone()
            } else {
                Rc::new(Node::Coordinate { src: s, dim: *dim })
            }
        }
        Node::Reduce { src, dim, op } => {
            let s = canonicalize_node(src, canonical, memo);
            if Rc::ptr_eq(&s, src) {
                node.clone()
            } else {
                Rc::new(Node::Reduce {
                    src: s,
                    dim: *dim,
                    op: *op,
                })
            }
        }
        Node::Scan { src, dim, op } => {
            let s = canonicalize_node(src, canonical, memo);
            if Rc::ptr_eq(&s, src) {
                node.clone()
            } else {
                Rc::new(Node::Scan {
                    src: s,
                    dim: *dim,
                    op: *op,
                })
            }
        }
        Node::Gather { src, index, dim } => {
            let s = canonicalize_node(src, canonical, memo);
            let i = canonicalize_node(index, canonical, memo);
            if Rc::ptr_eq(&s, src) && Rc::ptr_eq(&i, index) {
                node.clone()
            } else {
                Rc::new(Node::Gather {
                    src: s,
                    index: i,
                    dim: *dim,
                })
            }
        }
        Node::View { src, dims } => {
            let s = canonicalize_node(src, canonical, memo);
            if Rc::ptr_eq(&s, src) {
                node.clone()
            } else {
                Rc::new(Node::View {
                    src: s,
                    dims: dims.clone(),
                })
            }
        }
        Node::Reindex {
            src,
            shape,
            map,
            padded,
        } => {
            let s = canonicalize_node(src, canonical, memo);
            if Rc::ptr_eq(&s, src) {
                node.clone()
            } else {
                Rc::new(Node::Reindex {
                    src: s,
                    shape: shape.clone(),
                    map: map.clone(),
                    padded: *padded,
                })
            }
        }
    };
    let out = canonical
        .entry(shallow_key(&rebuilt))
        .or_insert(rebuilt)
        .clone();
    memo.insert(Rc::as_ptr(node), out.clone());
    out
}

/// One node's structure with children identified by POINTER — valid as an
/// identity key exactly when the children are already canonical.
pub(crate) fn shallow_key(n: &NodeRef) -> String {
    let p = |c: &NodeRef| Rc::as_ptr(c) as usize;
    match n.as_ref() {
        Node::Input { name, shape, dtype } => format!("I{name}{shape:?}{dtype:?}"),
        Node::Const { v } => format!("C{}", v.to_bits()),
        Node::Iota { axis } => format!("O{axis:?}"),
        Node::Coordinate { src, dim } => format!("D{dim}.{}", p(src)),
        Node::Map { op, inputs } => {
            format!("M{op:?}{:?}", inputs.iter().map(p).collect::<Vec<_>>())
        }
        Node::Reduce { src, dim, op } => format!("R{op:?}{dim}.{}", p(src)),
        Node::Scan { src, dim, op } => format!("S{op:?}{dim}.{}", p(src)),
        Node::Gather { src, index, dim } => format!("G{dim}.{}.{}", p(src), p(index)),
        Node::View { src, dims } => format!("V{dims:?}.{}", p(src)),
        Node::Reindex {
            src,
            shape,
            map,
            padded,
        } => format!("X{shape:?}{map:?}{padded}.{}", p(src)),
    }
}

/// Number of elements in a materialized output (`1` for a scalar).
pub fn volume(node: &NodeRef) -> usize {
    node.shape()
        .iter()
        .map(|axis| axis.extent())
        .product::<usize>()
        .max(1)
}

/// Input declarations and their resolved storage dimensions.
pub fn input_axes(node: &NodeRef) -> Vec<(&'static str, Vec<AxisRef>)> {
    fn walk(
        node: &NodeRef,
        output: &mut Vec<(&'static str, Vec<AxisRef>)>,
        seen: &mut std::collections::HashSet<*const Node>,
    ) {
        if !seen.insert(Rc::as_ptr(node)) {
            return;
        }
        match node.as_ref() {
            Node::Input { name, .. } => output.push((*name, axis_refs(node))),
            Node::Const { .. } | Node::Iota { .. } => {}
            Node::Coordinate { src, .. }
            | Node::Reduce { src, .. }
            | Node::Scan { src, .. }
            | Node::View { src, .. }
            | Node::Reindex { src, .. } => walk(src, output, seen),
            Node::Map { inputs, .. } => {
                for input in inputs {
                    walk(input, output, seen);
                }
            }
            Node::Gather { src, index, .. } => {
                walk(src, output, seen);
                walk(index, output, seen);
            }
        }
    }
    let mut output = Vec::new();
    walk(node, &mut output, &mut std::collections::HashSet::new());
    output
}

pub fn input_dtypes(node: &NodeRef) -> Vec<(&'static str, Dtype)> {
    fn walk(
        node: &NodeRef,
        output: &mut Vec<(&'static str, Dtype)>,
        seen: &mut std::collections::HashSet<*const Node>,
    ) {
        if !seen.insert(Rc::as_ptr(node)) {
            return;
        }
        match node.as_ref() {
            Node::Input { name, dtype, .. } => {
                if !output.iter().any(|(existing, _)| existing == name) {
                    output.push((*name, *dtype));
                }
            }
            Node::Const { .. } | Node::Iota { .. } => {}
            Node::Coordinate { src, .. }
            | Node::Reduce { src, .. }
            | Node::Scan { src, .. }
            | Node::View { src, .. }
            | Node::Reindex { src, .. } => walk(src, output, seen),
            Node::Map { inputs, .. } => {
                for input in inputs {
                    walk(input, output, seen);
                }
            }
            Node::Gather { src, index, .. } => {
                walk(src, output, seen);
                walk(index, output, seen);
            }
        }
    }

    let mut output = Vec::new();
    walk(node, &mut output, &mut std::collections::HashSet::new());
    output
}

/// Input names in first-seen order.
pub fn leaf_names(node: &NodeRef) -> Vec<&'static str> {
    let mut names = Vec::new();
    for (name, _) in input_axes(node) {
        if !names.contains(&name) {
            names.push(name);
        }
    }
    names
}

// ── derived compositions ────────────────────────────────────────────────────

/// Batched matrix multiplication, contracting the left last dimension with
/// the right penultimate dimension: `[..., m, k] @ [..., k, n]`.
pub fn matmul(a: NodeRef, b: NodeRef) -> NodeRef {
    let left_shape = a.shape();
    let right_shape = b.shape();

    // Build a common positional iteration space by inserting singleton
    // dimensions: left [..., m, k, 1], right [..., 1, k, n].
    assert!(
        left_shape.len() >= 2 && right_shape.len() >= 2,
        "matmul requires rank >= 2"
    );
    assert_eq!(
        left_shape.last().unwrap().extent,
        right_shape[right_shape.len() - 2].extent,
        "matmul contraction extents must match"
    );
    let singleton = axis("singleton", 1);
    let left = positional_view(
        a,
        left_shape
            .iter()
            .enumerate()
            .map(|(dim, &axis)| ViewDim {
                sources: vec![dim],
                axis,
            })
            .chain([ViewDim {
                sources: Vec::new(),
                axis: singleton,
            }])
            .collect(),
    );
    let right_insert = right_shape.len() - 2;
    let right = positional_view(
        b,
        right_shape
            .iter()
            .enumerate()
            .flat_map(|(dim, &axis)| {
                let leading = (dim == right_insert).then_some(ViewDim {
                    sources: Vec::new(),
                    axis: singleton,
                });
                leading.into_iter().chain([ViewDim {
                    sources: vec![dim],
                    axis,
                }])
            })
            .collect(),
    );
    let product = map(MapOp::Mul, vec![left, right]);
    let contract = product.shape().len() - 2;
    reduce(product, contract, Monoid::Add)
}

pub fn unsqueeze(src: NodeRef, dim: impl Dimension) -> NodeRef {
    let source = src.shape();
    let rank = source.len() as isize + 1;
    // Resolve against the output rank so `-1` appends.
    let synthetic = vec![axis("unsqueeze", 1); rank as usize];
    let dim = dim.resolve(&synthetic, "unsqueeze");
    let singleton = axis("singleton", 1);
    let mut dims = Vec::with_capacity(source.len() + 1);
    for output_dim in 0..=source.len() {
        if output_dim == dim {
            dims.push(ViewDim {
                sources: Vec::new(),
                axis: singleton,
            });
        } else {
            let source_dim = if output_dim < dim {
                output_dim
            } else {
                output_dim - 1
            };
            dims.push(ViewDim {
                sources: vec![source_dim],
                axis: source[source_dim],
            });
        }
    }
    positional_view(src, dims)
}

/// Remove a static singleton dimension without aggregation.
pub fn squeeze(src: NodeRef, dim: impl Dimension) -> NodeRef {
    let source = src.shape();
    let dim = dim.resolve(&source, "squeeze");
    assert_eq!(
        source[dim].extent,
        Extent::Static(1),
        "squeeze requires a static singleton dimension"
    );
    let mut shape = source.clone();
    shape.remove(dim);
    let map = source
        .iter()
        .enumerate()
        .map(|(source_dim, _)| {
            let terms = if source_dim == dim {
                Vec::new()
            } else {
                let output_dim = source_dim - usize::from(source_dim > dim);
                vec![(1, output_dim)]
            };
            (source_dim, terms, 0)
        })
        .collect();
    positional_reindex(src, shape, map, false)
}

pub fn softmax(x: NodeRef, dim: impl Dimension) -> NodeRef {
    let dim = dim.resolve(&x.shape(), "softmax");
    let maximum = reduce(x.clone(), dim, Monoid::Max);
    let maximum = unsqueeze(maximum, dim);
    let exponent = map(MapOp::Exp, vec![map(MapOp::Sub, vec![x, maximum])]);
    let normalizer = reduce(exponent.clone(), dim, Monoid::Add);
    map(MapOp::Div, vec![exponent, unsqueeze(normalizer, dim)])
}

/// Change one dimension's display descriptor without changing storage.
pub fn rename(src: NodeRef, dim: impl Dimension, to: Axis) -> NodeRef {
    let source = src.shape();
    let dim = dim.resolve(&source, "rename");
    positional_view(
        src,
        source
            .iter()
            .enumerate()
            .map(|(source_dim, &axis)| ViewDim {
                sources: vec![source_dim],
                axis: if source_dim == dim { to } else { axis },
            })
            .collect(),
    )
}

/// A contiguous slice along one dimension.
pub fn slice(src: NodeRef, dim: impl Dimension, to: Axis, start: usize) -> NodeRef {
    let source = src.shape();
    let dim = dim.resolve(&source, "slice");
    let mut shape = source.clone();
    shape[dim] = to;
    let map = source
        .iter()
        .enumerate()
        .map(|(source_dim, _)| {
            (
                source_dim,
                vec![(1, source_dim)],
                if source_dim == dim { start as i64 } else { 0 },
            )
        })
        .collect();
    positional_reindex(src, shape, map, false)
}

/// Zero-pad one dimension; `to` declares the resulting extent.
pub fn pad(src: NodeRef, dim: impl Dimension, to: Axis, low: usize) -> NodeRef {
    let source = src.shape();
    let dim = dim.resolve(&source, "pad");
    let mut shape = source.clone();
    shape[dim] = to;
    let map = source
        .iter()
        .enumerate()
        .map(|(source_dim, _)| {
            (
                source_dim,
                vec![(1, source_dim)],
                if source_dim == dim { -(low as i64) } else { 0 },
            )
        })
        .collect();
    positional_reindex(src, shape, map, true)
}

/// Sliding windows along one dimension. The selected source dimension is
/// replaced by `(out, kernel)`.
pub fn window(
    src: NodeRef,
    dim: impl Dimension,
    out: Axis,
    kernel: Axis,
    stride: usize,
    dilation: usize,
) -> NodeRef {
    let source = src.shape();
    let dim = dim.resolve(&source, "window");
    let mut shape = source.clone();
    shape.splice(dim..=dim, [out, kernel]);
    let map = source
        .iter()
        .enumerate()
        .map(|(source_dim, _)| {
            if source_dim == dim {
                (
                    source_dim,
                    vec![(stride as i64, dim), (dilation as i64, dim + 1)],
                    0,
                )
            } else {
                let output_dim = source_dim + usize::from(source_dim > dim);
                (source_dim, vec![(1, output_dim)], 0)
            }
        })
        .collect();
    positional_reindex(src, shape, map, false)
}

/// Embedding lookup is a gather along the table's vocabulary dimension.
pub fn embedding(table: NodeRef, ids: NodeRef, dim: impl Dimension) -> NodeRef {
    gather(table, ids, dim)
}

/// Add-combine rows according to a one-dimensional index tensor. This is the
/// positional adjoint of gathering along `dim`, expressed from reindexing,
/// coordinates, maps, and one reduction.
pub fn scatter_add(
    src: NodeRef,
    index: NodeRef,
    dim: impl Dimension,
    output_axis: Axis,
) -> NodeRef {
    let source = src.shape();
    let dim = dim.resolve(&source, "scatter_add");
    let index_shape = index.shape();
    assert_eq!(index_shape.len(), 1, "scatter_add index must be rank one");
    assert_eq!(
        index_shape[0].extent, source[dim].extent,
        "scatter_add index extent must match the selected source dimension"
    );

    let mut iteration = source.clone();
    iteration.insert(dim + 1, output_axis);
    let source_map = source
        .iter()
        .enumerate()
        .map(|(source_dim, _)| {
            let output_dim = source_dim + usize::from(source_dim > dim);
            (source_dim, vec![(1, output_dim)], 0)
        })
        .collect();
    let lifted_source = positional_reindex(src, iteration.clone(), source_map, false);
    let lifted_index = positional_reindex(index, iteration, vec![(0, vec![(1, dim)], 0)], false);
    let selected = one_hot_like(lifted_source.clone(), dim + 1, lifted_index);
    reduce(
        map(MapOp::Mul, vec![selected, lifted_source]),
        dim,
        Monoid::Add,
    )
}

/// `1` where the coordinate of `template` along `dim` equals `value`.
pub fn one_hot_like(template: NodeRef, dim: impl Dimension, value: NodeRef) -> NodeRef {
    let dim = dim.resolve(&template.shape(), "one_hot");
    let coordinate = coordinate(template, dim);
    map(
        MapOp::Sub,
        vec![
            map(
                MapOp::Sub,
                vec![
                    konst(1.0),
                    map(MapOp::Lt, vec![coordinate.clone(), value.clone()]),
                ],
            ),
            map(MapOp::Lt, vec![value, coordinate]),
        ],
    )
}

fn first_index_of_maximum(x: NodeRef, maximum: NodeRef, dim: usize) -> NodeRef {
    let maximum = unsqueeze(maximum, dim);
    let candidate = map(
        MapOp::Where,
        vec![
            map(MapOp::Lt, vec![x.clone(), maximum]),
            konst(f64::INFINITY),
            coordinate(x, dim),
        ],
    );
    reduce(candidate, dim, Monoid::Min)
}

/// Index of the first maximum along a positional dimension.
pub fn argmax(x: NodeRef, dim: impl Dimension) -> NodeRef {
    let dim = dim.resolve(&x.shape(), "argmax");
    let maximum = reduce(x.clone(), dim, Monoid::Max);
    first_index_of_maximum(x, maximum, dim)
}

/// Top-k as frontend composition: repeated generic max reductions, equality
/// masks, and index reductions. The core has no top-k node or matcher.
pub fn topk(x: NodeRef, dim: impl Dimension, k: usize) -> Vec<(NodeRef, NodeRef)> {
    let dim = dim.resolve(&x.shape(), "topk");
    let extent = x.shape()[dim].extent();
    assert!(k > 0, "topk requires k >= 1");
    assert!(k <= extent, "topk requires k <= dimension extent {extent}");

    let mut remaining = x;
    let mut selected = Vec::with_capacity(k);
    for _ in 0..k {
        let value = reduce(remaining.clone(), dim, Monoid::Max);
        let index = first_index_of_maximum(remaining.clone(), value.clone(), dim);
        let selected_position = one_hot_like(remaining.clone(), dim, unsqueeze(index.clone(), dim));
        remaining = map(
            MapOp::Where,
            vec![selected_position, konst(f64::NEG_INFINITY), remaining],
        );
        selected.push((value, index));
    }
    selected
}

/// Stack every selected rank in a new trailing dimension.
pub fn topk_all(
    x: NodeRef,
    dim: impl Dimension,
    k: usize,
    rank: Axis,
    return_indices: bool,
) -> NodeRef {
    assert_eq!(rank.extent(), k, "topk_all rank axis extent must equal k");
    let mut sum = None;
    for (rank_index, (value, index)) in topk(x, dim, k).into_iter().enumerate() {
        let selected = if return_indices { index } else { value };
        let source = selected.shape();
        let mut shape = source.clone();
        shape.push(rank);
        let selected = positional_reindex(
            selected,
            shape,
            source
                .iter()
                .enumerate()
                .map(|(source_dim, _)| (source_dim, vec![(1, source_dim)], 0))
                .collect(),
            false,
        );
        let rank_coordinate = coordinate(selected.clone(), -1isize);
        let at_rank = map(
            MapOp::Sub,
            vec![
                map(
                    MapOp::Sub,
                    vec![
                        konst(1.0),
                        map(
                            MapOp::Lt,
                            vec![rank_coordinate.clone(), konst(rank_index as f64)],
                        ),
                    ],
                ),
                map(MapOp::Lt, vec![konst(rank_index as f64), rank_coordinate]),
            ],
        );
        let term = map(MapOp::Mul, vec![at_rank, selected]);
        sum = Some(match sum {
            None => term,
            Some(previous) => map(MapOp::Add, vec![previous, term]),
        });
    }
    sum.expect("topk requires k >= 1")
}

pub fn silu(x: NodeRef) -> NodeRef {
    let sigmoid = map(
        MapOp::Recip,
        vec![map(
            MapOp::Add,
            vec![
                konst(1.0),
                map(MapOp::Exp, vec![map(MapOp::Neg, vec![x.clone()])]),
            ],
        )],
    );
    map(MapOp::Mul, vec![x, sigmoid])
}

pub fn causal_mask_like(
    scores: NodeRef,
    query_dim: impl Dimension,
    key_dim: impl Dimension,
) -> NodeRef {
    let shape = scores.shape();
    let query_dim = query_dim.resolve(&shape, "causal_mask query");
    let key_dim = key_dim.resolve(&shape, "causal_mask key");
    map(
        MapOp::Where,
        vec![
            map(
                MapOp::Lt,
                vec![
                    coordinate(scores.clone(), query_dim),
                    coordinate(scores, key_dim),
                ],
            ),
            konst(-1e30),
            konst(0.0),
        ],
    )
}
