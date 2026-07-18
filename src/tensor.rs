//! The explicit graph-building frontend.
//!
//! [`TensorExpr`] is a symbolic tensor-valued expression. [`Tensor`] is the
//! concrete data bound when a finished [`Graph`] runs. A [`GraphBuilder`]
//! assigns each input a binding name; [`GraphBuilder::finish`] freezes the
//! output expressions into a reusable graph.
//!
//! Dimensions are positional, following Torch conventions. Elementwise
//! operations broadcast trailing dimensions, reductions take a dimension
//! index, and matrix multiplication contracts `(m, k) @ (k, n)`. Dimension
//! names are optional diagnostic labels and never determine semantics.
//!
//! ```
//! use sanic::{Dtype, Extent, GraphBuilder};
//!
//! let mut graph = GraphBuilder::new();
//! let q = graph.input("q", [Extent::Dynamic, Extent::Static(64)], Dtype::F32);
//! let k = graph.input("k", [Extent::Static(64), Extent::Dynamic], Dtype::F32);
//! let v = graph.input("v", [Extent::Dynamic, Extent::Static(64)], Dtype::F32);
//!
//! // Naive attention, the textbook three-step form. `derive` reconstructs
//! // the FlashAttention streaming accumulator from this graph.
//! let out = q.matmul(&k).softmax(-1).matmul(&v);
//! let graph = graph.finish([out]);
//! assert_eq!(graph.input_count(), 3);
//! ```

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::derive::Carrier;
use crate::interp::{Env, Value};
use crate::ir::{
    self, Axis, BinOp, Dtype, Extent, MapOp, Monoid, Node, NodeKind, axis, leaf_names, output_axes,
};

/// Concrete tensor data bound to a graph input at execution time.
pub type Tensor = Value;

/// Concrete buffers keyed by their graph input names.
pub type Bindings = HashMap<String, Tensor>;

/// The ordered axes describing a tensor expression.
#[derive(Clone, Debug)]
pub struct Shape(Vec<Axis>);

impl Shape {
    pub fn new(axes: impl IntoIterator<Item = Axis>) -> Self {
        Shape(axes.into_iter().collect())
    }

    pub fn axes(&self) -> &[Axis] {
        &self.0
    }

    pub fn extents(&self) -> impl ExactSizeIterator<Item = Extent> + '_ {
        self.0.iter().map(|dimension| dimension.extent)
    }

    pub fn rank(&self) -> usize {
        self.0.len()
    }

    pub fn element_count(&self) -> Option<usize> {
        self.0.iter().try_fold(1usize, |count, axis| {
            axis.extent
                .static_value()
                .and_then(|extent| count.checked_mul(extent))
        })
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.extents().eq(other.extents())
    }
}

impl Eq for Shape {}

impl Hash for Shape {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rank().hash(state);
        for extent in self.extents() {
            extent.hash(state);
        }
    }
}

fn unnamed_axis(extent: impl Into<Extent>) -> Axis {
    axis("", extent)
}

impl<const N: usize> From<[Axis; N]> for Shape {
    fn from(axes: [Axis; N]) -> Self {
        Shape(axes.into())
    }
}

impl<const N: usize> From<[usize; N]> for Shape {
    fn from(extents: [usize; N]) -> Self {
        Shape(extents.into_iter().map(unnamed_axis).collect())
    }
}

impl<const N: usize> From<[Extent; N]> for Shape {
    fn from(extents: [Extent; N]) -> Self {
        Shape(extents.into_iter().map(unnamed_axis).collect())
    }
}

impl<const N: usize> From<&[Axis; N]> for Shape {
    fn from(axes: &[Axis; N]) -> Self {
        Shape(axes.to_vec())
    }
}

impl From<&[Axis]> for Shape {
    fn from(axes: &[Axis]) -> Self {
        Shape(axes.to_vec())
    }
}

impl From<Vec<Axis>> for Shape {
    fn from(axes: Vec<Axis>) -> Self {
        Shape(axes)
    }
}

impl From<Vec<usize>> for Shape {
    fn from(extents: Vec<usize>) -> Self {
        Shape(extents.into_iter().map(unnamed_axis).collect())
    }
}

impl From<Vec<Extent>> for Shape {
    fn from(extents: Vec<Extent>) -> Self {
        Shape(extents.into_iter().map(unnamed_axis).collect())
    }
}

impl AsRef<[Axis]> for Shape {
    fn as_ref(&self) -> &[Axis] {
        self.axes()
    }
}

#[derive(Clone, Debug)]
struct InputSpec {
    binding_name: String,
    leaf_name: &'static str,
    shape: Shape,
}

static NEXT_GRAPH: AtomicUsize = AtomicUsize::new(0);

/// Mutable construction context. It owns the named inputs for one graph.
pub struct GraphBuilder {
    graph_id: usize,
    inputs: Vec<InputSpec>,
}

/// An immutable graph with ordered inputs and declared output expressions.
pub struct Graph {
    inputs: Vec<InputSpec>,
    outputs: Vec<Node>,
}

impl GraphBuilder {
    pub fn new() -> GraphBuilder {
        GraphBuilder {
            graph_id: NEXT_GRAPH.fetch_add(1, Ordering::Relaxed),
            inputs: Vec::new(),
        }
    }

    /// Add a named input with its storage dtype.
    pub fn input(
        &mut self,
        name: impl Into<String>,
        shape: impl Into<Shape>,
        dtype: Dtype,
    ) -> TensorExpr {
        let binding_name = name.into();
        let shape = shape.into();
        assert!(
            !binding_name.is_empty(),
            "input binding names cannot be empty"
        );
        assert!(
            !self
                .inputs
                .iter()
                .any(|input| input.binding_name == binding_name),
            "input `{binding_name}` was declared more than once"
        );
        let input_index = self.inputs.len();
        let leaf_name =
            Box::leak(format!("__sanic_g{}_input{input_index}", self.graph_id).into_boxed_str());
        // Every input owns its positional dimensions. Reusing a diagnostic
        // label (or even the same shape descriptor) never aliases dimensions
        // across tensors.
        let shape = Shape::new(
            shape
                .axes()
                .iter()
                .map(|dimension| axis(dimension.name.as_str(), dimension.extent)),
        );
        self.inputs.push(InputSpec {
            binding_name,
            leaf_name,
            shape: shape.clone(),
        });
        TensorExpr::input_node(leaf_name, shape.axes(), dtype)
    }

    /// Freeze the reachable expressions as the graph's ordered outputs.
    pub fn finish(self, outputs: impl IntoIterator<Item = TensorExpr>) -> Graph {
        let outputs: Vec<TensorExpr> = outputs.into_iter().collect();
        assert!(!outputs.is_empty(), "a graph needs at least one output");
        for output in &outputs {
            for name in leaf_names(&output.node) {
                assert!(
                    self.inputs.iter().any(|input| input.leaf_name == name),
                    "output reads `{name}`, an input from a different GraphBuilder"
                );
            }
        }
        Graph {
            inputs: self.inputs,
            outputs: outputs.into_iter().map(|output| output.node).collect(),
        }
    }
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    pub fn input_count(&self) -> usize {
        self.inputs.len()
    }

    pub fn output_count(&self) -> usize {
        self.outputs.len()
    }

    /// Run the finished graph with buffers keyed by their declared names.
    pub fn run(&self, inputs: &Bindings) -> Vec<Tensor> {
        for name in inputs.keys() {
            assert!(
                self.inputs.iter().any(|input| input.binding_name == *name),
                "graph has no input named `{name}`"
            );
        }

        let mut resolved = HashMap::new();
        for spec in &self.inputs {
            let tensor = inputs
                .get(&spec.binding_name)
                .unwrap_or_else(|| panic!("graph input `{}` was not bound", spec.binding_name));
            assert_eq!(
                tensor.shape.len(),
                spec.shape.rank(),
                "input `{}` has rank {}; graph expects rank {}",
                spec.binding_name,
                tensor.shape.len(),
                spec.shape.rank()
            );
            let elements = tensor.shape.iter().product::<usize>().max(1);
            assert_eq!(
                tensor.data.len(),
                elements,
                "input `{}` shape {:?} requires {elements} values, received {}",
                spec.binding_name,
                tensor.shape,
                tensor.data.len()
            );
            for (&axis, &actual) in spec.shape.axes().iter().zip(&tensor.shape) {
                match axis.extent {
                    Extent::Static(expected) => assert_eq!(
                        actual, expected,
                        "input `{}` axis `{}` has extent {actual}; graph expects {expected}",
                        spec.binding_name, axis.name
                    ),
                    Extent::Dynamic => {
                        let previous = resolved.entry(axis).or_insert(actual);
                        assert_eq!(
                            *previous, actual,
                            "dynamic axis `{}` resolved to both {} and {actual}",
                            axis.name, *previous
                        );
                    }
                }
            }
        }

        let mut env = Env::new();
        for spec in &self.inputs {
            let tensor = &inputs[&spec.binding_name];
            env.insert(
                spec.leaf_name,
                Tensor {
                    axes: spec
                        .shape
                        .axes()
                        .iter()
                        .map(|&axis| concretize_axis(axis, &resolved))
                        .collect(),
                    shape: tensor.shape.clone(),
                    data: tensor.data.clone(),
                },
            );
        }

        let mut memo = HashMap::new();
        self.outputs
            .iter()
            .map(|output| {
                let output = concretize_node(output, &resolved, &mut memo);
                crate::interp::eval(&output, &env)
            })
            .collect()
    }

    pub fn output_shapes(&self) -> Vec<Shape> {
        self.outputs
            .iter()
            .map(|output| Shape(output_axes(output)))
            .collect()
    }
}

fn concretize_axis(axis: Axis, resolved: &HashMap<Axis, usize>) -> Axis {
    match axis.extent {
        Extent::Static(_) => axis,
        Extent::Dynamic => {
            axis.with_extent(Extent::Static(*resolved.get(&axis).unwrap_or_else(|| {
                panic!(
                    "dynamic axis `{}` cannot be inferred from any graph input",
                    axis.name
                )
            })))
        }
    }
}

fn concretize_node(
    node: &Node,
    resolved: &HashMap<Axis, usize>,
    memo: &mut HashMap<*const NodeKind, Node>,
) -> Node {
    let pointer = Rc::as_ptr(node);
    if let Some(node) = memo.get(&pointer) {
        return node.clone();
    }
    let concrete_axis = |axis| concretize_axis(axis, resolved);
    let concrete = match node.as_ref() {
        NodeKind::Input { name, axes, dtype } => Rc::new(NodeKind::Input {
            name,
            axes: axes.iter().copied().map(concrete_axis).collect(),
            dtype: *dtype,
        }),
        NodeKind::Const { v } => Rc::new(NodeKind::Const { v: *v }),
        NodeKind::Iota { axis } => Rc::new(NodeKind::Iota {
            axis: concrete_axis(*axis),
        }),
        NodeKind::Map { op, inputs } => Rc::new(NodeKind::Map {
            op: *op,
            inputs: inputs
                .iter()
                .map(|input| concretize_node(input, resolved, memo))
                .collect(),
        }),
        NodeKind::Reduce { src, axis, op } => Rc::new(NodeKind::Reduce {
            src: concretize_node(src, resolved, memo),
            axis: concrete_axis(*axis),
            op: *op,
        }),
        NodeKind::Scan { src, axis, op } => Rc::new(NodeKind::Scan {
            src: concretize_node(src, resolved, memo),
            axis: concrete_axis(*axis),
            op: *op,
        }),
        NodeKind::Gather { src, index, axis } => Rc::new(NodeKind::Gather {
            src: concretize_node(src, resolved, memo),
            index: concretize_node(index, resolved, memo),
            axis: concrete_axis(*axis),
        }),
        NodeKind::View { src, groups } => Rc::new(NodeKind::View {
            src: concretize_node(src, resolved, memo),
            groups: groups
                .iter()
                .map(|(members, output)| {
                    let members: Vec<Axis> = members.iter().copied().map(concrete_axis).collect();
                    let output = if output.is_dynamic() && !resolved.contains_key(output) {
                        output.with_extent(Extent::Static(
                            members.iter().map(|member| member.extent()).product(),
                        ))
                    } else {
                        concrete_axis(*output)
                    };
                    (members, output)
                })
                .collect(),
        }),
        NodeKind::Reindex { src, map, padded } => Rc::new(NodeKind::Reindex {
            src: concretize_node(src, resolved, memo),
            map: map
                .iter()
                .map(|(source, terms, offset)| {
                    (
                        concrete_axis(*source),
                        terms
                            .iter()
                            .map(|(coefficient, axis)| (*coefficient, concrete_axis(*axis)))
                            .collect(),
                        *offset,
                    )
                })
                .collect(),
            padded: *padded,
        }),
    };
    memo.insert(pointer, concrete.clone());
    concrete
}

/// A symbolic tensor-valued expression. It has no concrete data or storage.
#[derive(Clone, Debug)]
pub struct TensorExpr {
    node: Node,
}

impl TensorExpr {
    // ── constructors ─────────────────────────────────────────────────────────

    fn input_node(name: &'static str, axes: &[Axis], dtype: Dtype) -> TensorExpr {
        TensorExpr {
            node: ir::input_dt(name, axes, dtype),
        }
    }

    /// A literal scalar expression.
    pub fn scalar(v: f64) -> TensorExpr {
        TensorExpr { node: ir::konst(v) }
    }

    /// `0, 1, 2, …, length - 1`.
    pub fn arange(length: usize) -> TensorExpr {
        TensorExpr {
            node: ir::iota(axis("arange", length)),
        }
    }

    /// A `(query_length, key_length)` causal mask.
    pub fn causal_mask(query_length: usize, key_length: usize) -> TensorExpr {
        let query = axis("query", query_length);
        let key = axis("key", key_length);
        TensorExpr {
            node: ir::map(
                MapOp::Where,
                vec![
                    ir::map(MapOp::Lt, vec![ir::iota(query), ir::iota(key)]),
                    ir::konst(-1e30),
                    ir::konst(0.0),
                ],
            ),
        }
    }

    /// Append a class dimension and encode integer values as one-hot rows.
    pub fn one_hot(&self, classes: usize) -> TensorExpr {
        let class = axis("class", classes);
        let iota = ir::iota(class);
        TensorExpr {
            node: ir::map(
                MapOp::Sub,
                vec![
                    ir::map(
                        MapOp::Sub,
                        vec![
                            ir::konst(1.0),
                            ir::map(MapOp::Lt, vec![iota.clone(), self.node.clone()]),
                        ],
                    ),
                    ir::map(MapOp::Lt, vec![self.node.clone(), iota]),
                ],
            ),
        }
    }

    // ── shape ────────────────────────────────────────────────────────────────

    /// The output axes, in the graph's axis order — each carrying its extent.
    /// A scalar has none.
    pub fn shape(&self) -> Shape {
        Shape(output_axes(&self.node))
    }

    fn axis(&self, dimension: isize, op: &str) -> Axis {
        let axes = output_axes(&self.node);
        let rank = axes.len() as isize;
        let index = if dimension < 0 {
            rank + dimension
        } else {
            dimension
        };
        assert!(
            (0..rank).contains(&index),
            "{op}: dimension {dimension} is out of range for a rank-{} tensor",
            axes.len()
        );
        axes[index as usize]
    }

    fn static_extent(axis: Axis, op: &str) -> usize {
        axis.extent.static_value().unwrap_or_else(|| {
            panic!("{op}: dynamic dimensions are resolved only when the graph runs")
        })
    }

    /// Rebind `node` positionally to `target`, applying size-one broadcasting.
    fn broadcast_node(node: Node, target: &[Axis], op: &str) -> Node {
        let source = output_axes(&node);
        assert!(
            source.len() <= target.len(),
            "{op}: cannot broadcast rank {} to rank {}",
            source.len(),
            target.len()
        );
        let offset = target.len() - source.len();
        let mut node = node;
        for (index, &from) in source.iter().enumerate() {
            let to = target[offset + index];
            if from == to {
                continue;
            }
            match (from.extent, to.extent) {
                (Extent::Static(1), _) => {
                    node = ir::reduce(node, from, BinOp::Monoid(Monoid::Add));
                }
                (left, right) if left == right => {
                    node = ir::rename(node, from, to);
                }
                _ => panic!(
                    "{op}: dimensions with extents {:?} and {:?} cannot broadcast",
                    from.extent, to.extent
                ),
            }
        }

        if output_axes(&node) == target {
            return node;
        }

        Self::with_axis_order(node, target)
    }

    fn with_axis_order(node: Node, target: &[Axis]) -> Node {
        if output_axes(&node) == target {
            return node;
        }
        // Putting a zero expression first establishes the exact Torch output
        // order without making diagnostic labels semantic.
        let mut zero = ir::konst(0.0);
        for &axis in target {
            zero = ir::map(
                MapOp::Add,
                vec![
                    zero,
                    ir::map(MapOp::Mul, vec![ir::iota(axis), ir::konst(0.0)]),
                ],
            );
        }
        ir::map(MapOp::Add, vec![zero, node])
    }

    fn broadcast_axes(left: &[Axis], right: &[Axis], op: &str) -> Vec<Axis> {
        let rank = left.len().max(right.len());
        let mut output = Vec::with_capacity(rank);
        for output_index in 0..rank {
            let left_axis = output_index
                .checked_sub(rank - left.len())
                .map(|index| left[index]);
            let right_axis = output_index
                .checked_sub(rank - right.len())
                .map(|index| right[index]);
            let axis = match (left_axis, right_axis) {
                (Some(left), Some(right)) => match (left.extent, right.extent) {
                    (Extent::Static(1), _) => right,
                    (_, Extent::Static(1)) => left,
                    (left_extent, right_extent) if left_extent == right_extent => left,
                    _ => panic!(
                        "{op}: dimensions with extents {:?} and {:?} cannot broadcast",
                        left.extent, right.extent
                    ),
                },
                (Some(axis), None) | (None, Some(axis)) => axis,
                (None, None) => unreachable!(),
            };
            output.push(axis);
        }
        output
    }

    // ── elementwise ──────────────────────────────────────────────────────────

    fn unary(&self, op: MapOp) -> TensorExpr {
        TensorExpr {
            node: ir::map(op, vec![self.node.clone()]),
        }
    }

    fn binary(&self, op: MapOp, other: &TensorExpr) -> TensorExpr {
        let left_axes = output_axes(&self.node);
        let right_axes = output_axes(&other.node);
        let axes = Self::broadcast_axes(&left_axes, &right_axes, "elementwise operation");
        TensorExpr {
            node: ir::map(
                op,
                vec![
                    Self::broadcast_node(self.node.clone(), &axes, "elementwise operation"),
                    Self::broadcast_node(other.node.clone(), &axes, "elementwise operation"),
                ],
            ),
        }
    }

    pub fn exp(&self) -> TensorExpr {
        self.unary(MapOp::Exp)
    }
    pub fn log(&self) -> TensorExpr {
        self.unary(MapOp::Log)
    }
    pub fn sqrt(&self) -> TensorExpr {
        self.unary(MapOp::Sqrt)
    }
    pub fn tanh(&self) -> TensorExpr {
        self.unary(MapOp::Tanh)
    }
    pub fn sin(&self) -> TensorExpr {
        self.unary(MapOp::Sin)
    }
    pub fn cos(&self) -> TensorExpr {
        self.unary(MapOp::Cos)
    }
    pub fn recip(&self) -> TensorExpr {
        self.unary(MapOp::Recip)
    }

    /// `sigmoid(x) = 1 / (1 + exp(-x))`.
    pub fn sigmoid(&self) -> TensorExpr {
        TensorExpr {
            node: ir::map(
                MapOp::Recip,
                vec![ir::map(
                    MapOp::Add,
                    vec![
                        ir::konst(1.0),
                        ir::map(
                            MapOp::Exp,
                            vec![ir::map(MapOp::Neg, vec![self.node.clone()])],
                        ),
                    ],
                )],
            ),
        }
    }

    /// Elementwise max (the reduction over an axis is [`TensorExpr::max`]).
    pub fn maximum(&self, other: &TensorExpr) -> TensorExpr {
        self.binary(MapOp::Max, other)
    }
    /// Elementwise min (the reduction over an axis is [`TensorExpr::min`]).
    pub fn minimum(&self, other: &TensorExpr) -> TensorExpr {
        self.binary(MapOp::Min, other)
    }
    /// `self < other` as 1.0 / 0.0 — with [`TensorExpr::select`], how masks are
    /// *computed* instead of loaded.
    pub fn lt(&self, other: &TensorExpr) -> TensorExpr {
        self.binary(MapOp::Lt, other)
    }
    /// `self != 0 ? if_true : if_false`, elementwise; `self` is the condition.
    pub fn select(&self, if_true: &TensorExpr, if_false: &TensorExpr) -> TensorExpr {
        let condition_axes = output_axes(&self.node);
        let true_axes = output_axes(&if_true.node);
        let false_axes = output_axes(&if_false.node);
        let axes = Self::broadcast_axes(
            &Self::broadcast_axes(&condition_axes, &true_axes, "where"),
            &false_axes,
            "where",
        );
        TensorExpr {
            node: ir::map(
                MapOp::Where,
                vec![
                    Self::broadcast_node(self.node.clone(), &axes, "where"),
                    Self::broadcast_node(if_true.node.clone(), &axes, "where"),
                    Self::broadcast_node(if_false.node.clone(), &axes, "where"),
                ],
            ),
        }
    }

    /// `silu(x) = x · sigmoid(x)` — a composition, so it fuses into whatever
    /// consumes it.
    pub fn silu(&self) -> TensorExpr {
        self * self.sigmoid()
    }

    /// Associative affine recurrence over `time`.
    pub fn ssm_scan(&self, dimension: isize) -> TensorExpr {
        let time = self.axis(dimension, "ssm_scan");
        TensorExpr {
            node: ir::scan(self.node.clone(), time, BinOp::AffineCompose),
        }
    }

    /// Sequential `tanh` recurrence over `time`.
    pub fn tanh_rnn(&self, dimension: isize) -> TensorExpr {
        let time = self.axis(dimension, "tanh_rnn");
        TensorExpr {
            node: ir::scan(
                ir::map(MapOp::Tanh, vec![self.node.clone()]),
                time,
                BinOp::NonAssoc("tanh_recur"),
            ),
        }
    }

    // ── reductions ───────────────────────────────────────────────────────────

    /// Fold one dimension; the result no longer carries it.
    pub fn reduce(&self, dimension: isize, op: BinOp) -> TensorExpr {
        let axis = self.axis(dimension, "reduce");
        TensorExpr {
            node: ir::reduce(self.node.clone(), axis, op),
        }
    }

    pub fn sum(&self, dimension: isize) -> TensorExpr {
        self.reduce(dimension, BinOp::Monoid(Monoid::Add))
    }
    pub fn prod(&self, dimension: isize) -> TensorExpr {
        self.reduce(dimension, BinOp::Monoid(Monoid::Mul))
    }
    pub fn max(&self, dimension: isize) -> TensorExpr {
        self.reduce(dimension, BinOp::Monoid(Monoid::Max))
    }
    pub fn min(&self, dimension: isize) -> TensorExpr {
        self.reduce(dimension, BinOp::Monoid(Monoid::Min))
    }
    pub fn logsumexp(&self, dimension: isize) -> TensorExpr {
        self.reduce(dimension, BinOp::Monoid(Monoid::LogSumExp))
    }
    /// The index of the first maximum along `a`, as a value — one
    /// index-carrying fold, matching every framework's tie convention.
    pub fn argmax(&self, dimension: isize) -> TensorExpr {
        self.reduce(dimension, BinOp::ArgMax)
    }

    /// All k ranks of the top-k selection along `a` as ONE tensor over a
    /// fresh rank axis (returned) — and, downstream, one kernel. `idx` picks
    /// positions instead of values.
    pub fn topk_all(
        &self,
        dimension: isize,
        k: usize,
        rank_name: &'static str,
        idx: bool,
    ) -> TensorExpr {
        let reduction_axis = self.axis(dimension, "topk_all");
        let rank = axis(rank_name, k);
        assert!(k > 0, "topk_all requires k >= 1");
        let mut output = None;
        for index in 0..k {
            let ranked = ir::reduce(
                self.node.clone(),
                reduction_axis,
                BinOp::TopK {
                    k: k as u8,
                    rank: index as u8,
                    idx,
                },
            );
            let rank_iota = ir::iota(rank);
            let selector = ir::map(
                MapOp::Sub,
                vec![
                    ir::map(
                        MapOp::Sub,
                        vec![
                            ir::konst(1.0),
                            ir::map(MapOp::Lt, vec![rank_iota.clone(), ir::konst(index as f64)]),
                        ],
                    ),
                    ir::map(MapOp::Lt, vec![ir::konst(index as f64), rank_iota]),
                ],
            );
            let term = ir::map(MapOp::Mul, vec![selector, ranked]);
            output = Some(match output {
                Some(output) => ir::map(MapOp::Add, vec![output, term]),
                None => term,
            });
        }
        TensorExpr {
            node: output.unwrap(),
        }
    }

    /// The top `k` values and their first-max-wins indices along `axis`.
    pub fn topk(&self, dimension: isize, k: usize) -> Vec<(TensorExpr, TensorExpr)> {
        let axis = self.axis(dimension, "topk");
        (0..k)
            .map(|rank| {
                let value = ir::reduce(
                    self.node.clone(),
                    axis,
                    BinOp::TopK {
                        k: k as u8,
                        rank: rank as u8,
                        idx: false,
                    },
                );
                let index = ir::reduce(
                    self.node.clone(),
                    axis,
                    BinOp::TopK {
                        k: k as u8,
                        rank: rank as u8,
                        idx: true,
                    },
                );
                (TensorExpr { node: value }, TensorExpr { node: index })
            })
            .collect()
    }

    /// Prefix recurrence over `a` (the axis is kept); foldable iff `op` is
    /// associative.
    pub fn scan(&self, dimension: isize, op: BinOp) -> TensorExpr {
        let axis = self.axis(dimension, "scan");
        TensorExpr {
            node: ir::scan(self.node.clone(), axis, op),
        }
    }

    // ── contractions and friends ─────────────────────────────────────────────

    /// Torch-style matrix multiplication: `(m, k) @ (k, n)`.
    ///
    /// Leading dimensions use trailing-dimension broadcasting.
    pub fn matmul(&self, other: &TensorExpr) -> TensorExpr {
        let left = output_axes(&self.node);
        let right = output_axes(&other.node);
        assert!(
            left.len() >= 2 && right.len() >= 2,
            "matmul requires tensors with rank >= 2; received ranks {} and {}",
            left.len(),
            right.len()
        );
        let left_k = left[left.len() - 1];
        let right_k = right[right.len() - 2];
        assert_eq!(
            left_k.extent, right_k.extent,
            "matmul: left k dimension {:?} does not match right k dimension {:?}",
            left_k.extent, right_k.extent
        );

        let batch = Self::broadcast_axes(
            &left[..left.len() - 2],
            &right[..right.len() - 2],
            "matmul batch dimensions",
        );
        let mut left_target = batch.clone();
        left_target.extend([left[left.len() - 2], left_k]);
        let mut right_target = batch;
        // `n` is an output dimension of this matmul, not the semantic axis
        // carried by the right operand. In particular, `x @ x.T` must produce
        // `(m, n)`, even though both dimensions originate from the same input
        // position.
        let right_n = right[right.len() - 1];
        let output_n = axis(right_n.name.as_str(), right_n.extent);
        right_target.extend([left_k, output_n]);
        let left = Self::broadcast_node(self.node.clone(), &left_target, "matmul");
        let right = Self::broadcast_node(other.node.clone(), &right_target, "matmul");
        TensorExpr {
            node: ir::reduce(
                ir::map(MapOp::Mul, vec![left, right]),
                left_k,
                BinOp::Monoid(Monoid::Add),
            ),
        }
    }

    /// Softmax over `a` as the textbook dataflow — max, shift, exp, sum,
    /// divide — from which `derive` reconstructs the online form.
    pub fn softmax(&self, dimension: isize) -> TensorExpr {
        let axis = self.axis(dimension, "softmax");
        let maximum = ir::reduce(self.node.clone(), axis, BinOp::Monoid(Monoid::Max));
        let exponent = ir::map(
            MapOp::Exp,
            vec![ir::map(MapOp::Sub, vec![self.node.clone(), maximum])],
        );
        let normalizer = ir::reduce(exponent.clone(), axis, BinOp::Monoid(Monoid::Add));
        TensorExpr {
            node: ir::map(MapOp::Div, vec![exponent, normalizer]),
        }
    }

    /// `softmax(self·kᵀ over d)·v`, normalized over `keys` — naive attention,
    /// the graph FlashAttention is derived from.
    pub fn attention(&self, k: &TensorExpr, v: &TensorExpr) -> TensorExpr {
        self.matmul(k).softmax(-1).matmul(v)
    }

    /// `self[index[…]]` — data-dependent access along `a` (embedding lookup,
    /// expert selection).
    pub fn gather(&self, index: &TensorExpr, dimension: isize) -> TensorExpr {
        let axis = self.axis(dimension, "gather");
        TensorExpr {
            node: ir::gather(self.node.clone(), index.node.clone(), axis),
        }
    }

    /// Look up `ids` in a table along its vocabulary axis.
    pub fn embedding(&self, ids: &TensorExpr) -> TensorExpr {
        let vocabulary = self.axis(0, "embedding");
        let gathered = ir::gather(self.node.clone(), ids.node.clone(), vocabulary);
        let mut axes = output_axes(&ids.node);
        axes.extend(output_axes(&self.node).into_iter().skip(1));
        TensorExpr {
            node: Self::with_axis_order(gathered, &axes),
        }
    }

    /// Scatter-add along `from` into buckets over `to` (gather's adjoint;
    /// collisions add).
    pub fn scatter_add(
        &self,
        index: &TensorExpr,
        dimension: isize,
        output_size: usize,
    ) -> TensorExpr {
        let from = self.axis(dimension, "scatter_add");
        let one_hot = index.one_hot(output_size);
        TensorExpr {
            node: ir::reduce(
                ir::map(MapOp::Mul, vec![one_hot.node, self.node.clone()]),
                from,
                BinOp::Monoid(Monoid::Add),
            ),
        }
    }

    // ── movement ─────────────────────────────────────────────────────────────

    /// Reorder dimensions. `dimensions` must be a permutation of the rank.
    pub fn permute(&self, dimensions: &[isize]) -> TensorExpr {
        let axes = output_axes(&self.node);
        assert_eq!(
            dimensions.len(),
            axes.len(),
            "permute: expected {} dimensions, received {}",
            axes.len(),
            dimensions.len()
        );
        let target: Vec<Axis> = dimensions
            .iter()
            .map(|&dimension| self.axis(dimension, "permute"))
            .collect();
        let mut unique = target.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(
            unique.len(),
            axes.len(),
            "permute: dimensions must not repeat"
        );
        TensorExpr {
            node: Self::with_axis_order(self.node.clone(), &target),
        }
    }

    pub fn transpose(&self, first: isize, second: isize) -> TensorExpr {
        let rank = output_axes(&self.node).len();
        let mut dimensions: Vec<isize> = (0..rank as isize).collect();
        let normalize = |dimension: isize| {
            if dimension < 0 {
                (rank as isize + dimension) as usize
            } else {
                dimension as usize
            }
        };
        let first = normalize(first);
        let second = normalize(second);
        assert!(
            first < rank && second < rank,
            "transpose: dimension out of range for rank {rank}"
        );
        dimensions.swap(first, second);
        self.permute(&dimensions)
    }

    /// Insert a size-one dimension.
    pub fn unsqueeze(&self, dimension: isize) -> TensorExpr {
        let mut axes = output_axes(&self.node);
        let rank = axes.len() as isize;
        let index = if dimension < 0 {
            rank + dimension + 1
        } else {
            dimension
        };
        assert!(
            (0..=rank).contains(&index),
            "unsqueeze: dimension {dimension} is out of range for rank {rank}"
        );
        axes.insert(index as usize, axis("singleton", 1));
        TensorExpr {
            node: Self::with_axis_order(self.node.clone(), &axes),
        }
    }

    /// Reverse the values along `dimension`, without moving storage. This is an
    /// affine reindex, useful for pairwise transforms such as RoPE.
    pub fn flip(&self, dimension: isize) -> TensorExpr {
        let axis = self.axis(dimension, "flip");
        TensorExpr {
            node: ir::reindex(
                self.node.clone(),
                vec![(axis, vec![(-1, axis)], axis.extent() as i64 - 1)],
                false,
            ),
        }
    }

    /// Flatten the inclusive dimension range in row-major order.
    pub fn flatten(&self, start_dimension: isize, end_dimension: isize) -> TensorExpr {
        let axes = output_axes(&self.node);
        let start = self.axis(start_dimension, "flatten");
        let end = self.axis(end_dimension, "flatten");
        let start = axes.iter().position(|axis| *axis == start).unwrap();
        let end = axes.iter().position(|axis| *axis == end).unwrap();
        assert!(
            start <= end,
            "flatten: start dimension follows end dimension"
        );
        let group = &axes[start..=end];
        let extent: usize = group
            .iter()
            .map(|axis| Self::static_extent(*axis, "flatten"))
            .product();
        let output = axis("flattened", extent);
        TensorExpr {
            node: ir::flatten(self.node.clone(), group, output),
        }
    }

    /// Split one axis into (outer, inner) fresh axes (returned) — the inverse
    /// of [`TensorExpr::flatten`]; `inner_extent` must divide `from.extent()`.
    pub fn split(&self, dimension: isize, inner_extent: usize) -> TensorExpr {
        let from = self.axis(dimension, "split");
        assert_eq!(
            from.extent() % inner_extent,
            0,
            "split: {inner_extent} does not divide `{from}` (extent {})",
            from.extent()
        );
        let outer = axis("outer", from.extent() / inner_extent);
        let inner = axis("inner", inner_extent);
        TensorExpr {
            node: ir::split(self.node.clone(), from, outer, inner),
        }
    }

    /// A contiguous slice `[start, start + len)` along `from`, as a fresh
    /// axis (returned).
    pub fn slice(&self, dimension: isize, start: usize, len: usize) -> TensorExpr {
        let from = self.axis(dimension, "slice");
        assert!(
            start + len <= from.extent(),
            "slice: [{start}, {}) exceeds `{from}` (extent {})",
            start + len,
            from.extent()
        );
        let to = axis("slice", len);
        TensorExpr {
            node: ir::slice(self.node.clone(), from, to, start),
        }
    }

    /// Zero-pad along `from` by `lo` below and `hi` above, as a fresh axis
    /// (returned; extent = lo + from + hi).
    pub fn pad(&self, dimension: isize, lo: usize, hi: usize) -> TensorExpr {
        let from = self.axis(dimension, "pad");
        let to = axis("padded", lo + from.extent() + hi);
        TensorExpr {
            node: ir::pad(self.node.clone(), from, to, lo),
        }
    }

    /// Sliding windows along one dimension. The result replaces it with
    /// `(output_size, kernel_size)` and reads
    /// `source[output·stride + kernel·dilation]`.
    pub fn window(
        &self,
        dimension: isize,
        output_size: usize,
        kernel_size: usize,
        stride: usize,
        dilation: usize,
    ) -> TensorExpr {
        let from = self.axis(dimension, "window");
        let out = axis("window", output_size);
        let k = axis("kernel", kernel_size);
        let last_read = (out.extent() - 1) * stride + (k.extent() - 1) * dilation;
        assert!(
            last_read < from.extent(),
            "window: last read index {last_read} exceeds `{from}` (extent {}) — pad first?",
            from.extent()
        );
        TensorExpr {
            node: ir::window(self.node.clone(), from, out, k, stride, dilation),
        }
    }

    // ── the pipeline ─────────────────────────────────────────────────────────

    /// Derive the streaming accumulator for folding `a` — None where no legal
    /// fold exists.
    pub fn derive(&self, dimension: isize) -> Option<Carrier> {
        crate::derive::derive(&self.node, self.axis(dimension, "derive"))
    }
}

// ── operators ────────────────────────────────────────────────────────────────

// The four arithmetic operators over every (owned | borrowed) × (TensorExpr | f64)
// pairing — mechanical, so a macro writes the 32 impls; each is one `binary`.
macro_rules! arith {
    ($Op:ident, $method:ident, $mapop:expr) => {
        impl std::ops::$Op<&TensorExpr> for &TensorExpr {
            type Output = TensorExpr;
            fn $method(self, rhs: &TensorExpr) -> TensorExpr {
                self.binary($mapop, rhs)
            }
        }
        impl std::ops::$Op<TensorExpr> for &TensorExpr {
            type Output = TensorExpr;
            fn $method(self, rhs: TensorExpr) -> TensorExpr {
                self.binary($mapop, &rhs)
            }
        }
        impl std::ops::$Op<&TensorExpr> for TensorExpr {
            type Output = TensorExpr;
            fn $method(self, rhs: &TensorExpr) -> TensorExpr {
                self.binary($mapop, rhs)
            }
        }
        impl std::ops::$Op<TensorExpr> for TensorExpr {
            type Output = TensorExpr;
            fn $method(self, rhs: TensorExpr) -> TensorExpr {
                self.binary($mapop, &rhs)
            }
        }
        impl std::ops::$Op<f64> for &TensorExpr {
            type Output = TensorExpr;
            fn $method(self, rhs: f64) -> TensorExpr {
                self.binary($mapop, &TensorExpr::scalar(rhs))
            }
        }
        impl std::ops::$Op<f64> for TensorExpr {
            type Output = TensorExpr;
            fn $method(self, rhs: f64) -> TensorExpr {
                self.binary($mapop, &TensorExpr::scalar(rhs))
            }
        }
        impl std::ops::$Op<&TensorExpr> for f64 {
            type Output = TensorExpr;
            fn $method(self, rhs: &TensorExpr) -> TensorExpr {
                TensorExpr::scalar(self).binary($mapop, rhs)
            }
        }
        impl std::ops::$Op<TensorExpr> for f64 {
            type Output = TensorExpr;
            fn $method(self, rhs: TensorExpr) -> TensorExpr {
                TensorExpr::scalar(self).binary($mapop, &rhs)
            }
        }
    };
}

arith!(Add, add, MapOp::Add);
arith!(Sub, sub, MapOp::Sub);
arith!(Mul, mul, MapOp::Mul);
arith!(Div, div, MapOp::Div);

impl std::ops::Neg for &TensorExpr {
    type Output = TensorExpr;
    fn neg(self) -> TensorExpr {
        self.unary(MapOp::Neg)
    }
}
impl std::ops::Neg for TensorExpr {
    type Output = TensorExpr;
    fn neg(self) -> TensorExpr {
        self.unary(MapOp::Neg)
    }
}
