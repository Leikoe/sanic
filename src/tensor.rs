//! The explicit graph-building frontend.
//!
//! [`TensorExpr`] is a symbolic tensor-valued expression. [`Tensor`] is the
//! concrete data bound when a finished [`Graph`] runs. A [`GraphBuilder`]
//! assigns input IDs during construction; [`GraphBuilder::finish`] freezes
//! the output expressions into a reusable graph.
//!
//! One deep difference from tinygrad: axes are named, not positional. Two
//! operands align where they mention the same axis and broadcast everywhere
//! else, so there is no positional broadcasting, no reshape-to-align, no
//! `keepdim` — `x + b` with `x` over `[s, dm]` and `b` over `[dm]` just
//! works, and reducing an axis simply removes it from the shape.
//!
//! ```
//! use sanic::{GraphBuilder, axis};
//!
//! let (s, t, d) = (axis("s", 128), axis("t", 128), axis("d", 64));
//! let mut graph = GraphBuilder::new();
//! let q = graph.input(&[s, d]);
//! let k = graph.input(&[t, d]);
//! let v = graph.input(&[t, d]);
//!
//! // Naive attention, the textbook three-step form. `derive` reconstructs
//! // the FlashAttention streaming accumulator from this graph.
//! let out = q.matmul(&k, d).softmax(t).matmul(&v, t);
//! let graph = graph.finish([out]);
//! assert_eq!(graph.input_count(), 3);
//! ```

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::derive::Carrier;
use crate::interp::{Env, Value};
use crate::ir::{self, Axis, BinOp, Dtype, MapOp, Monoid, Node, axis, leaf_names, output_axes};

/// Concrete tensor data bound to a graph input at execution time.
pub type Tensor = Value;

/// Dense, per-graph input index. It is intentionally not a global name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InputId(usize);

impl InputId {
    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug)]
struct InputSpec {
    id: InputId,
    name: &'static str,
    axes: Vec<Axis>,
    dtype: Option<Dtype>,
}

static NEXT_GRAPH: AtomicUsize = AtomicUsize::new(0);

/// Mutable construction context. It owns the dense input order for one graph.
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

    /// Add a bindable f64-semantic input. Inputs are bound in this call order.
    pub fn input(&mut self, axes: &[Axis]) -> TensorExpr {
        self.input_impl(axes, None)
    }

    /// Add an input with an explicit storage dtype.
    pub fn input_dt(&mut self, axes: &[Axis], dtype: Dtype) -> TensorExpr {
        self.input_impl(axes, Some(dtype))
    }

    fn input_impl(&mut self, axes: &[Axis], dtype: Option<Dtype>) -> TensorExpr {
        let id = InputId(self.inputs.len());
        let name = Box::leak(format!("__sanic_g{}_input{}", self.graph_id, id.0).into_boxed_str());
        self.inputs.push(InputSpec {
            id,
            name,
            axes: axes.to_vec(),
            dtype,
        });
        TensorExpr::input_node(name, axes, dtype, id)
    }

    /// A scalar expression in the graph being built.
    pub fn scalar(&self, value: f64) -> TensorExpr {
        TensorExpr::scalar(value)
    }

    pub fn iota(&self, axis: Axis) -> TensorExpr {
        TensorExpr::iota(axis)
    }

    pub fn causal_mask(&self, query: Axis, key: Axis) -> TensorExpr {
        TensorExpr::causal_mask(query, key)
    }

    /// Freeze the reachable expressions as the graph's ordered outputs.
    pub fn finish(self, outputs: impl IntoIterator<Item = TensorExpr>) -> Graph {
        let outputs: Vec<TensorExpr> = outputs.into_iter().collect();
        assert!(!outputs.is_empty(), "a graph needs at least one output");
        for output in &outputs {
            for name in leaf_names(&output.node) {
                assert!(
                    self.inputs.iter().any(|input| input.name == name),
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

    /// Run the finished graph with buffers in [`InputId`] order.
    pub fn run(&self, inputs: impl AsRef<[Tensor]>) -> Vec<Tensor> {
        let inputs = inputs.as_ref();
        assert_eq!(
            inputs.len(),
            self.inputs.len(),
            "graph expects {} input buffers, received {}",
            self.inputs.len(),
            inputs.len()
        );
        let mut env = Env::new();
        for (spec, tensor) in self.inputs.iter().zip(inputs) {
            let expected: Vec<usize> = spec.axes.iter().map(|axis| axis.extent).collect();
            assert_eq!(
                tensor.shape, expected,
                "input {} has shape {:?}; graph expects {:?}",
                spec.id.0, tensor.shape, expected
            );
            let _ = spec.dtype;
            env.insert(spec.name, tensor.clone());
        }
        self.outputs
            .iter()
            .map(|output| crate::interp::eval(output, &env))
            .collect()
    }

    pub fn output_shapes(&self) -> Vec<Vec<Axis>> {
        self.outputs.iter().map(output_axes).collect()
    }
}

/// A symbolic tensor-valued expression. It has no concrete data or storage.
#[derive(Clone, Debug)]
pub struct TensorExpr {
    node: Node,
    input_id: Option<InputId>,
}

fn wrap(node: Node) -> TensorExpr {
    TensorExpr {
        node,
        input_id: None,
    }
}

impl TensorExpr {
    // ── constructors ─────────────────────────────────────────────────────────

    fn input_node(
        name: &'static str,
        axes: &[Axis],
        dtype: Option<Dtype>,
        input_id: InputId,
    ) -> TensorExpr {
        TensorExpr {
            node: match dtype {
                Some(dtype) => ir::input_dt(name, axes, dtype),
                None => ir::input(name, axes),
            },
            input_id: Some(input_id),
        }
    }

    /// A literal scalar.
    fn scalar(v: f64) -> TensorExpr {
        wrap(ir::konst(v))
    }

    /// The index along an axis, as a value (0, 1, 2, …) — free to compute.
    fn iota(a: Axis) -> TensorExpr {
        wrap(ir::iota(a))
    }

    /// A causal mask computed from indices (0 where key ≤ query, −LARGE
    /// after) — costs no memory traffic.
    fn causal_mask(query: Axis, key: Axis) -> TensorExpr {
        wrap(ir::causal_mask(query, key))
    }

    /// `1.0` where `iota(a) == v`, else `0.0` — a computed one-hot.
    pub fn one_hot(a: Axis, v: &TensorExpr) -> TensorExpr {
        wrap(ir::one_hot(a, v.node.clone()))
    }

    pub fn input_id(&self) -> Option<InputId> {
        self.input_id
    }

    // ── shape ────────────────────────────────────────────────────────────────

    /// The output axes, in the graph's axis order — each carrying its extent.
    /// A scalar has none.
    pub fn shape(&self) -> Vec<Axis> {
        output_axes(&self.node)
    }

    fn expect_axis(&self, a: Axis, op: &str) {
        assert!(
            output_axes(&self.node).contains(&a),
            "{op}: axis `{a}` is not an output axis of this tensor (shape [{}])",
            self.shape()
                .iter()
                .map(|a| a.name)
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    // ── elementwise ──────────────────────────────────────────────────────────

    fn unary(&self, op: MapOp) -> TensorExpr {
        wrap(ir::map(op, vec![self.node.clone()]))
    }

    fn binary(&self, op: MapOp, other: &TensorExpr) -> TensorExpr {
        wrap(ir::map(op, vec![self.node.clone(), other.node.clone()]))
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
        wrap(ir::map(
            MapOp::Where,
            vec![
                self.node.clone(),
                if_true.node.clone(),
                if_false.node.clone(),
            ],
        ))
    }

    /// `silu(x) = x · sigmoid(x)` — a composition, so it fuses into whatever
    /// consumes it.
    pub fn silu(&self) -> TensorExpr {
        wrap(ir::silu(self.node.clone()))
    }

    // ── reductions ───────────────────────────────────────────────────────────

    /// Fold `a` with `op`; the result no longer carries `a`.
    pub fn reduce(&self, a: Axis, op: BinOp) -> TensorExpr {
        self.expect_axis(a, "reduce");
        wrap(ir::reduce(self.node.clone(), a, op))
    }

    pub fn sum(&self, a: Axis) -> TensorExpr {
        self.reduce(a, BinOp::Monoid(Monoid::Add))
    }
    pub fn prod(&self, a: Axis) -> TensorExpr {
        self.reduce(a, BinOp::Monoid(Monoid::Mul))
    }
    pub fn max(&self, a: Axis) -> TensorExpr {
        self.reduce(a, BinOp::Monoid(Monoid::Max))
    }
    pub fn min(&self, a: Axis) -> TensorExpr {
        self.reduce(a, BinOp::Monoid(Monoid::Min))
    }
    pub fn logsumexp(&self, a: Axis) -> TensorExpr {
        self.reduce(a, BinOp::Monoid(Monoid::LogSumExp))
    }
    /// The index of the first maximum along `a`, as a value — one
    /// index-carrying fold, matching every framework's tie convention.
    pub fn argmax(&self, a: Axis) -> TensorExpr {
        self.reduce(a, BinOp::ArgMax)
    }

    /// All k ranks of the top-k selection along `a` as ONE tensor over a
    /// fresh rank axis (returned) — and, downstream, one kernel. `idx` picks
    /// positions instead of values.
    pub fn topk_all(
        &self,
        a: Axis,
        k: usize,
        rank_name: &'static str,
        idx: bool,
    ) -> (TensorExpr, Axis) {
        self.expect_axis(a, "topk_all");
        let rank = axis(rank_name, k);
        (wrap(ir::topk_all(self.node.clone(), a, k, rank, idx)), rank)
    }

    /// Prefix recurrence over `a` (the axis is kept); foldable iff `op` is
    /// associative.
    pub fn scan(&self, a: Axis, op: BinOp) -> TensorExpr {
        self.expect_axis(a, "scan");
        wrap(ir::scan(self.node.clone(), a, op))
    }

    // ── contractions and friends ─────────────────────────────────────────────

    /// Contract `contract` between `self` and `other`:
    /// `Reduce(Map(×, a, b), contract, Add)`. Every other shared axis is
    /// batched; every unshared one broadcasts — matmul, matvec, and batched
    /// attention contractions are all this one call.
    pub fn matmul(&self, other: &TensorExpr, contract: Axis) -> TensorExpr {
        self.expect_axis(contract, "matmul (left operand)");
        other.expect_axis(contract, "matmul (right operand)");
        wrap(ir::matmul(self.node.clone(), other.node.clone(), contract))
    }

    /// Softmax over `a` as the textbook dataflow — max, shift, exp, sum,
    /// divide — from which `derive` reconstructs the online form.
    pub fn softmax(&self, a: Axis) -> TensorExpr {
        self.expect_axis(a, "softmax");
        wrap(ir::softmax(self.node.clone(), a))
    }

    /// `softmax(self·kᵀ over d)·v`, normalized over `keys` — naive attention,
    /// the graph FlashAttention is derived from.
    pub fn attention(&self, k: &TensorExpr, v: &TensorExpr, d: Axis, keys: Axis) -> TensorExpr {
        wrap(ir::attention(
            self.node.clone(),
            k.node.clone(),
            v.node.clone(),
            d,
            keys,
        ))
    }

    /// `self[index[…]]` — data-dependent access along `a` (embedding lookup,
    /// expert selection).
    pub fn gather(&self, index: &TensorExpr, a: Axis) -> TensorExpr {
        self.expect_axis(a, "gather");
        wrap(ir::gather(self.node.clone(), index.node.clone(), a))
    }

    /// Scatter-add along `from` into buckets over `to` (gather's adjoint;
    /// collisions add).
    pub fn scatter_add(&self, index: &TensorExpr, from: Axis, to: Axis) -> TensorExpr {
        self.expect_axis(from, "scatter_add");
        wrap(ir::scatter_add(
            self.node.clone(),
            index.node.clone(),
            from,
            to,
        ))
    }

    // ── movement ─────────────────────────────────────────────────────────────

    /// The same values under a different axis — how two tensors come to share
    /// (or stop sharing) an index space. Extents must match.
    pub fn rename(&self, from: Axis, to: Axis) -> TensorExpr {
        self.expect_axis(from, "rename");
        assert_eq!(
            from.extent, to.extent,
            "rename: `{from}` has extent {} but `{to}` has {}",
            from.extent, to.extent
        );
        wrap(ir::rename(self.node.clone(), from, to))
    }

    /// Merge a group of axes into one fresh axis (returned; extent = the
    /// product), row-major in the group's order. No data moves.
    pub fn flatten(&self, group: &[Axis], name: &'static str) -> (TensorExpr, Axis) {
        for a in group {
            self.expect_axis(*a, "flatten");
        }
        let to = axis(name, group.iter().map(|a| a.extent).product());
        (wrap(ir::flatten(self.node.clone(), group, to)), to)
    }

    /// Split one axis into (outer, inner) fresh axes (returned) — the inverse
    /// of [`TensorExpr::flatten`]; `inner_extent` must divide `from.extent`.
    pub fn split(
        &self,
        from: Axis,
        outer_name: &'static str,
        inner_name: &'static str,
        inner_extent: usize,
    ) -> (TensorExpr, Axis, Axis) {
        self.expect_axis(from, "split");
        assert_eq!(
            from.extent % inner_extent,
            0,
            "split: {inner_extent} does not divide `{from}` (extent {})",
            from.extent
        );
        let outer = axis(outer_name, from.extent / inner_extent);
        let inner = axis(inner_name, inner_extent);
        (
            wrap(ir::split(self.node.clone(), from, outer, inner)),
            outer,
            inner,
        )
    }

    /// A contiguous slice `[start, start + len)` along `from`, as a fresh
    /// axis (returned).
    pub fn slice(
        &self,
        from: Axis,
        start: usize,
        len: usize,
        name: &'static str,
    ) -> (TensorExpr, Axis) {
        self.expect_axis(from, "slice");
        assert!(
            start + len <= from.extent,
            "slice: [{start}, {}) exceeds `{from}` (extent {})",
            start + len,
            from.extent
        );
        let to = axis(name, len);
        (wrap(ir::slice(self.node.clone(), from, to, start)), to)
    }

    /// Zero-pad along `from` by `lo` below and `hi` above, as a fresh axis
    /// (returned; extent = lo + from + hi).
    pub fn pad(&self, from: Axis, lo: usize, hi: usize, name: &'static str) -> (TensorExpr, Axis) {
        self.expect_axis(from, "pad");
        let to = axis(name, lo + from.extent + hi);
        (wrap(ir::pad(self.node.clone(), from, to, lo)), to)
    }

    /// Sliding windows along `from`: output `(out, k)` reads
    /// `from[out·stride + k·dilation]`. `out` and `k` are caller-supplied
    /// because `out` may deliberately be an *existing* axis to share an index
    /// space (sliding-window attention rides the query axis). Convolution and
    /// pooling are this followed by a contraction / max over `k`; compose
    /// with [`TensorExpr::pad`] for SAME-style windows.
    pub fn window(
        &self,
        from: Axis,
        out: Axis,
        k: Axis,
        stride: usize,
        dilation: usize,
    ) -> TensorExpr {
        self.expect_axis(from, "window");
        let last_read = (out.extent - 1) * stride + (k.extent - 1) * dilation;
        assert!(
            last_read < from.extent,
            "window: last read index {last_read} exceeds `{from}` (extent {}) — pad first?",
            from.extent
        );
        wrap(ir::window(
            self.node.clone(),
            from,
            out,
            k,
            stride,
            dilation,
        ))
    }

    // ── the pipeline ─────────────────────────────────────────────────────────

    /// Derive the streaming accumulator for folding `a` — None where no legal
    /// fold exists.
    pub fn derive(&self, a: Axis) -> Option<Carrier> {
        crate::derive::derive(&self.node, a)
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
