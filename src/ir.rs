//! Public positional tensor IR.
//!
//! A dimension is identified only by its position in a node's ordered shape.
//! [`Axis`] carries display metadata and an extent; it is never a graph-wide
//! variable or an equality key. Compiler passes lower these nodes into their
//! own scoped loop representation.

use std::fmt;
use std::rc::Rc;

pub use crate::kernel_ir::{BinOp, Dtype, Extent, MapOp, Monoid};

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

    pub const fn is_dynamic(self) -> bool {
        matches!(self.extent, Extent::Dynamic)
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
        name: String,
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
        op: BinOp,
    },
    Scan {
        src: NodeRef,
        dim: usize,
        op: BinOp,
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
    Rc::new(Node::Input {
        name: name.into(),
        shape: shape.as_ref().to_vec(),
        dtype,
    })
}

pub fn konst(v: f64) -> NodeRef {
    Rc::new(Node::Const { v })
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

pub fn reduce(src: NodeRef, dim: impl Dimension, op: BinOp) -> NodeRef {
    let dim = dim.resolve(&src.shape(), "reduce");
    Rc::new(Node::Reduce { src, dim, op })
}

pub fn scan(src: NodeRef, dim: impl Dimension, op: BinOp) -> NodeRef {
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
                "view: output extent {actual} does not match source product {expected}"
            );
        }
    }
    assert!(
        consumed.into_iter().all(|value| value),
        "view: every source dimension must be consumed exactly once"
    );
    Rc::new(Node::View { src, dims })
}

pub trait IntoViewDims {
    fn into_view_dims(self, source: &[Axis]) -> Vec<ViewDim>;
}

impl IntoViewDims for Vec<(Vec<usize>, Axis)> {
    fn into_view_dims(self, _source: &[Axis]) -> Vec<ViewDim> {
        self.into_iter()
            .map(|(sources, axis)| ViewDim { sources, axis })
            .collect()
    }
}

/// Build a positional view. Groups are listed in output order.
pub fn view(src: NodeRef, dims: impl IntoViewDims) -> NodeRef {
    let shape = src.shape();
    positional_view(src, dims.into_view_dims(&shape))
}

pub fn rename(src: NodeRef, from: impl Dimension, to: Axis) -> NodeRef {
    let source = src.shape();
    let from = from.resolve(&source, "rename");
    let dims = source
        .iter()
        .enumerate()
        .map(|(index, &axis)| ViewDim {
            sources: vec![index],
            axis: if index == from { to } else { axis },
        })
        .collect();
    positional_view(src, dims)
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

pub fn slice(src: NodeRef, from: impl Dimension, to: Axis, start: usize) -> NodeRef {
    let source = src.shape();
    let from = from.resolve(&source, "slice");
    let mut shape = source.clone();
    shape[from] = to;
    let map = source
        .iter()
        .enumerate()
        .map(|(dim, _)| {
            (
                dim,
                vec![(1, dim)],
                if dim == from { start as i64 } else { 0 },
            )
        })
        .collect();
    positional_reindex(src, shape, map, false)
}

pub fn pad(src: NodeRef, from: impl Dimension, to: Axis, lo: usize) -> NodeRef {
    let source = src.shape();
    let from = from.resolve(&source, "pad");
    let mut shape = source.clone();
    shape[from] = to;
    let map = source
        .iter()
        .enumerate()
        .map(|(dim, _)| {
            (
                dim,
                vec![(1, dim)],
                if dim == from { -(lo as i64) } else { 0 },
            )
        })
        .collect();
    positional_reindex(src, shape, map, true)
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

pub fn window(
    src: NodeRef,
    from: impl Dimension,
    out: Axis,
    kernel: Axis,
    stride: usize,
    dilation: usize,
) -> NodeRef {
    let source = src.shape();
    let from = from.resolve(&source, "window");
    let mut shape = source.clone();
    shape.splice(from..=from, [out, kernel]);
    let map = source
        .iter()
        .enumerate()
        .map(|(source_dim, _)| {
            if source_dim == from {
                (
                    source_dim,
                    vec![(stride as i64, from), (dilation as i64, from + 1)],
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

pub fn volume(node: &NodeRef) -> usize {
    node.shape()
        .into_iter()
        .map(Axis::extent)
        .product::<usize>()
        .max(1)
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
    reduce(product, contract, BinOp::Monoid(Monoid::Add))
}

fn unsqueeze_at(src: NodeRef, dim: usize) -> NodeRef {
    let source = src.shape();
    assert!(dim <= source.len(), "unsqueeze dimension out of range");
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

pub fn softmax(x: NodeRef, dim: impl Dimension) -> NodeRef {
    let dim = dim.resolve(&x.shape(), "softmax");
    let maximum = reduce(x.clone(), dim, BinOp::Monoid(Monoid::Max));
    let maximum = unsqueeze_at(maximum, dim);
    let exponent = map(MapOp::Exp, vec![map(MapOp::Sub, vec![x, maximum])]);
    let normalizer = reduce(exponent.clone(), dim, BinOp::Monoid(Monoid::Add));
    map(MapOp::Div, vec![exponent, unsqueeze_at(normalizer, dim)])
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

pub fn causal_mask(query: Axis, key: Axis) -> NodeRef {
    let query_position = unsqueeze_at(iota(query), 1);
    let key_position = unsqueeze_at(iota(key), 0);
    map(
        MapOp::Where,
        vec![
            map(MapOp::Lt, vec![query_position, key_position]),
            konst(-1e30),
            konst(0.0),
        ],
    )
}

pub(crate) fn causal_mask_like(
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
