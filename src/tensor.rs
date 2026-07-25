//! The public tensor handle: an interned graph node with operators.
//!
//! A [`Tensor`] is a thin, cloneable wrapper over [`NodeRef`] — the same
//! interned, immutable graph the whole compiler works on. Building one is
//! building the graph; nothing executes until the graph is compiled.
//!
//! Dimensions are addressed positionally (`usize`/`isize`) or by NAME:
//! every op taking `impl Dimension` accepts the axis's display name, so
//! model code reads as `hidden.sum("hidden")` rather than index arithmetic.
//! Names identify a dimension only within one tensor's shape (the
//! convention: distinct spaces get distinct names); the graph underneath
//! stays positional.

use std::ops::{Add, Deref, Div, Mul, Neg, Sub};

use crate::ir::{self, Axis, AxisSelector, Dimension, Dtype, MapOp, Monoid, NodeRef};

/// A value in the computation graph. Cloning is O(1) (an `Arc` bump);
/// structurally identical expressions are the SAME tensor.
#[derive(Clone)]
pub struct Tensor {
    node: NodeRef,
}

impl Tensor {
    /// A named graph input with a declared storage dtype.
    pub fn input(name: impl Into<String>, shape: impl AsRef<[Axis]>, dtype: Dtype) -> Tensor {
        ir::input(name, shape, dtype).into()
    }

    /// A scalar constant.
    pub fn scalar(value: f64) -> Tensor {
        ir::konst(value).into()
    }

    /// The positional index along a fresh axis: `[0, 1, …, extent-1]`.
    pub fn iota(axis: Axis) -> Tensor {
        ir::iota(axis).into()
    }

    /// The underlying graph node.
    pub fn node(&self) -> &NodeRef {
        &self.node
    }

    /// Ordered output shape.
    pub fn shape(&self) -> Vec<Axis> {
        self.node.shape()
    }

    /// The extent of one dimension, by position or name.
    pub fn extent(&self, dim: impl Dimension) -> usize {
        let shape = self.shape();
        shape[dim.resolve(&shape, "extent")].extent()
    }

    // ── elementwise ──────────────────────────────────────────────────────────

    fn unary(&self, op: MapOp) -> Tensor {
        ir::map(op, vec![self.node.clone()]).into()
    }

    fn binary(&self, op: MapOp, rhs: impl Into<Tensor>) -> Tensor {
        ir::map(op, vec![self.node.clone(), rhs.into().node]).into()
    }

    pub fn exp(&self) -> Tensor {
        self.unary(MapOp::Exp)
    }
    pub fn log(&self) -> Tensor {
        self.unary(MapOp::Log)
    }
    pub fn sqrt(&self) -> Tensor {
        self.unary(MapOp::Sqrt)
    }
    pub fn tanh(&self) -> Tensor {
        self.unary(MapOp::Tanh)
    }
    pub fn sin(&self) -> Tensor {
        self.unary(MapOp::Sin)
    }
    pub fn cos(&self) -> Tensor {
        self.unary(MapOp::Cos)
    }
    pub fn recip(&self) -> Tensor {
        self.unary(MapOp::Recip)
    }
    pub fn silu(&self) -> Tensor {
        ir::silu(self.node.clone()).into()
    }

    /// Declare a storage boundary: this value, rounded exactly as a
    /// materialization at `dtype` would round it. Because the rounding is
    /// part of the VALUE, the numerics here are schedule-independent — the
    /// compiler may cut anywhere, store wider, or fuse straight through,
    /// and the reloaded bits never change. Gradients pass straight through.
    pub fn stored(&self, dtype: Dtype) -> Tensor {
        self.unary(MapOp::RoundTo(dtype))
    }

    /// Elementwise maximum/minimum (the reductions are [`Tensor::max`]/
    /// [`Tensor::min`]).
    pub fn maximum(&self, rhs: impl Into<Tensor>) -> Tensor {
        self.binary(MapOp::Max, rhs)
    }
    pub fn minimum(&self, rhs: impl Into<Tensor>) -> Tensor {
        self.binary(MapOp::Min, rhs)
    }

    /// `1.0` where `self < rhs`, else `0.0`.
    pub fn lt(&self, rhs: impl Into<Tensor>) -> Tensor {
        self.binary(MapOp::Lt, rhs)
    }

    /// `self` as a mask: where it is non-zero take `then`, else `otherwise`.
    pub fn select(&self, then: impl Into<Tensor>, otherwise: impl Into<Tensor>) -> Tensor {
        ir::map(
            MapOp::Where,
            vec![self.node.clone(), then.into().node, otherwise.into().node],
        )
        .into()
    }

    /// Ones with this tensor's shape (and its dimension occurrences).
    pub fn ones_like(&self) -> Tensor {
        ir::ones_like(self.node.clone()).into()
    }

    // ── reductions and scans ─────────────────────────────────────────────────

    fn reduce(&self, dim: impl Dimension, op: Monoid) -> Tensor {
        ir::reduce(self.node.clone(), dim, op).into()
    }

    pub fn sum(&self, dim: impl Dimension) -> Tensor {
        self.reduce(dim, Monoid::Add)
    }
    pub fn prod(&self, dim: impl Dimension) -> Tensor {
        self.reduce(dim, Monoid::Mul)
    }
    pub fn max(&self, dim: impl Dimension) -> Tensor {
        self.reduce(dim, Monoid::Max)
    }
    pub fn min(&self, dim: impl Dimension) -> Tensor {
        self.reduce(dim, Monoid::Min)
    }
    pub fn logsumexp(&self, dim: impl Dimension) -> Tensor {
        self.reduce(dim, Monoid::LogSumExp)
    }
    pub fn cumsum(&self, dim: impl Dimension) -> Tensor {
        ir::scan(self.node.clone(), dim, Monoid::Add).into()
    }

    pub fn mean(&self, dim: impl Dimension) -> Tensor {
        let shape = self.shape();
        let dim = dim.resolve(&shape, "mean");
        let extent = shape[dim].extent() as f64;
        self.sum(dim) * (1.0 / extent)
    }

    pub fn softmax(&self, dim: impl Dimension) -> Tensor {
        ir::softmax(self.node.clone(), dim).into()
    }

    pub fn argmax(&self, dim: impl Dimension) -> Tensor {
        ir::argmax(self.node.clone(), dim).into()
    }

    // ── contraction and gathering ────────────────────────────────────────────

    /// Batched matrix product over the two trailing dimensions.
    pub fn matmul(&self, rhs: impl Into<Tensor>) -> Tensor {
        ir::matmul(self.node.clone(), rhs.into().node).into()
    }

    /// Rows of `self` selected by `ids` along `dim`.
    pub fn gather(&self, ids: impl Into<Tensor>, dim: impl Dimension) -> Tensor {
        ir::gather(self.node.clone(), ids.into().node, dim).into()
    }

    /// Embedding lookup: `self` is the table, `ids` index `dim`.
    pub fn embedding(&self, ids: impl Into<Tensor>, dim: impl Dimension) -> Tensor {
        ir::embedding(self.node.clone(), ids.into().node, dim).into()
    }

    /// The coordinate along `dim`, broadcast over this tensor's shape.
    pub fn coordinate(&self, dim: impl Dimension) -> Tensor {
        ir::coordinate(self.node.clone(), dim).into()
    }

    // ── structure ────────────────────────────────────────────────────────────

    pub fn transpose(&self, dim0: impl Dimension, dim1: impl Dimension) -> Tensor {
        ir::transpose(self.node.clone(), dim0, dim1).into()
    }

    /// Split one dimension into `outer × inner`.
    pub fn split(&self, dim: impl Dimension, outer: Axis, inner: Axis) -> Tensor {
        ir::split(self.node.clone(), dim, outer, inner).into()
    }

    /// Flatten a contiguous group of dimensions into `to`.
    pub fn flatten(&self, group: impl ir::Dimensions, to: Axis) -> Tensor {
        ir::flatten(self.node.clone(), group, to).into()
    }

    pub fn unsqueeze(&self, dim: impl Dimension) -> Tensor {
        ir::unsqueeze(self.node.clone(), dim).into()
    }
    pub fn squeeze(&self, dim: impl Dimension) -> Tensor {
        ir::squeeze(self.node.clone(), dim).into()
    }

    /// Give one dimension a new display axis (extent must match).
    pub fn rename(&self, dim: impl Dimension, to: Axis) -> Tensor {
        ir::rename(self.node.clone(), dim, to).into()
    }

    /// A contiguous sub-range of `dim`, starting at `start`, with `to`'s
    /// extent.
    pub fn slice(&self, dim: impl Dimension, to: Axis, start: usize) -> Tensor {
        ir::slice(self.node.clone(), dim, to, start).into()
    }

    /// Zero-pad `dim` up to `to`'s extent, with `low` zeros in front.
    pub fn pad(&self, dim: impl Dimension, to: Axis, low: usize) -> Tensor {
        ir::pad(self.node.clone(), dim, to, low).into()
    }

    /// Reverse one dimension.
    pub fn flip(&self, dim: impl Dimension) -> Tensor {
        ir::flip(self.node.clone(), dim).into()
    }

    /// The resolved occurrence of one dimension — for compiler-facing APIs
    /// (deriving a kernel along an axis).
    pub fn axis(&self, dim: impl AxisSelector) -> ir::AxisRef {
        dim.resolve_axis(&self.node, "axis")
            .expect("axis: selector did not resolve")
    }

    // ── autodiff ─────────────────────────────────────────────────────────────

    /// d(self)/d(each target), one entry per target — `None` when this loss
    /// does not depend on it. `self` must be a scalar. Targets may be any
    /// tensor, weights or interior values alike. `stop` tensors are gradient
    /// boundaries: the gradient reaches them but never flows through them to
    /// their inputs (stop-gradient as a property of the query, not the graph).
    pub fn gradient(&self, targets: &[&Tensor], stop: &[&Tensor]) -> Vec<Option<Tensor>> {
        let targets: Vec<NodeRef> = targets.iter().map(|t| t.node.clone()).collect();
        let stop: Vec<NodeRef> = stop.iter().map(|t| t.node.clone()).collect();
        crate::grad::grad_nodes(&self.node, &targets, &stop)
            .into_iter()
            .map(|gradient| gradient.map(Tensor::from))
            .collect()
    }
}

impl Deref for Tensor {
    type Target = NodeRef;
    fn deref(&self) -> &NodeRef {
        &self.node
    }
}

impl From<NodeRef> for Tensor {
    fn from(node: NodeRef) -> Tensor {
        Tensor { node }
    }
}

impl From<&NodeRef> for Tensor {
    fn from(node: &NodeRef) -> Tensor {
        Tensor { node: node.clone() }
    }
}

impl From<Tensor> for NodeRef {
    fn from(tensor: Tensor) -> NodeRef {
        tensor.node
    }
}

impl From<&Tensor> for Tensor {
    fn from(tensor: &Tensor) -> Tensor {
        tensor.clone()
    }
}

impl From<f64> for Tensor {
    fn from(value: f64) -> Tensor {
        Tensor::scalar(value)
    }
}

/// The four arithmetic operators, for every owned/borrowed/scalar pairing.
macro_rules! arithmetic {
    ($trait:ident, $method:ident, $op:expr) => {
        impl<R: Into<Tensor>> $trait<R> for Tensor {
            type Output = Tensor;
            fn $method(self, rhs: R) -> Tensor {
                self.binary($op, rhs)
            }
        }
        impl<R: Into<Tensor>> $trait<R> for &Tensor {
            type Output = Tensor;
            fn $method(self, rhs: R) -> Tensor {
                self.binary($op, rhs)
            }
        }
        impl $trait<Tensor> for f64 {
            type Output = Tensor;
            fn $method(self, rhs: Tensor) -> Tensor {
                Tensor::scalar(self).binary($op, rhs)
            }
        }
        impl $trait<&Tensor> for f64 {
            type Output = Tensor;
            fn $method(self, rhs: &Tensor) -> Tensor {
                Tensor::scalar(self).binary($op, rhs)
            }
        }
    };
}

arithmetic!(Add, add, MapOp::Add);
arithmetic!(Sub, sub, MapOp::Sub);
arithmetic!(Mul, mul, MapOp::Mul);
arithmetic!(Div, div, MapOp::Div);

impl Neg for Tensor {
    type Output = Tensor;
    fn neg(self) -> Tensor {
        self.unary(MapOp::Neg)
    }
}

impl Neg for &Tensor {
    type Output = Tensor;
    fn neg(self) -> Tensor {
        self.unary(MapOp::Neg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::axis;

    #[test]
    fn named_dimensions_resolve_and_operators_build_the_same_graph() {
        let (s, d) = (axis("s", 3), axis("d", 4));
        let x = Tensor::input("x", [s, d], Dtype::F32);

        // name and position resolve to the same reduction — and interning
        // makes them literally the same node
        assert!(std::sync::Arc::ptr_eq(x.sum("d").node(), x.sum(1usize).node()));

        // operator sugar builds the same graph as the ir functions
        let sugared = (&x * &x + 1.0).sqrt();
        let spelled: Tensor = ir::map(
            MapOp::Sqrt,
            vec![ir::map(
                MapOp::Add,
                vec![
                    ir::map(MapOp::Mul, vec![x.node().clone(), x.node().clone()]),
                    ir::konst(1.0),
                ],
            )],
        )
        .into();
        assert!(std::sync::Arc::ptr_eq(sugared.node(), spelled.node()));
    }

    #[test]
    #[should_panic(expected = "no axis named")]
    fn unknown_axis_name_panics_with_the_shape() {
        let x = Tensor::input("x", [axis("s", 3)], Dtype::F32);
        let _ = x.sum("hidden");
    }
}
