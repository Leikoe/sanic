//! A dense reference interpreter for the IR — the correctness oracle.
//!
//! Everything else in this crate is *clever*: `derive` strengthens a fold into
//! a streaming accumulator, `partition` cuts a graph into kernels, the
//! emitters transcribe carriers to code. This module is deliberately *dumb*.
//! It evaluates a [`Node`] the naive way — materialize every intermediate,
//! reduce with a plain loop, no fusion, no rescaling, no deferral — and
//! returns a dense [`Value`]. Because it is the definition of what the IR
//! *means*, it is the ground truth every derived kernel must reproduce.
//!
//! Two entry points:
//!
//! * [`eval`] — the semantics of the graph. `eval(attention) ` computes
//!   `softmax(QKᵀ)·V` by literally building the scores, softmaxing, and
//!   contracting. Slow and obvious on purpose.
//! * [`run_carrier`] — drives a *derived* carrier the way a real kernel would:
//!   grid over the free axes, stream the folded axis, lift/combine/project
//!   each element from real tensor data. Where [`eval`] says what the answer
//!   is, this says what the generated kernel computes.
//!
//! Holding `run_carrier == eval` on random tensors is the end-to-end
//! correctness certificate: not "the carrier folds consistently with itself"
//! (which [`crate::derive`]'s law tests already show), but "the derived
//! kernel computes the real math on real data." That is the guarantee a
//! compiler owes its users, and the foundation everything downstream
//! (dtypes, GPU backends, autodiff) is validated against.
//!
//! Scope: the elementwise · reduce · scan · gather · view · iota core — i.e.
//! the whole attention / transformer vocabulary.

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use crate::derive::Carrier;
use crate::ir::{
    self, AxisRef, AxisSelector, Extent, MapOp, Monoid, Node as NodeKind, NodeRef as Node,
};

/// Input tensors by leaf name.
pub type Env = HashMap<&'static str, Value>;

// ── dense tensors ────────────────────────────────────────────────────────────

/// A dense, row-major tensor tagged with the axis each dimension ranges over —
/// the interpreter's value domain: what a graph *means* on concrete data.
/// (The public graph is made of [`crate::ir::NodeRef`] values; this is data.)
/// Resolved axis occurrences are opaque semantic identities; their labels are diagnostic.
/// The position in `axes` is the storage order. Two values with the same
/// internal axes in different orders denote the same tensor value (see
/// [`Value::permuted_to`]).
#[derive(Debug, Clone)]
pub struct Value {
    pub axes: Vec<AxisRef>,
    pub shape: Vec<usize>,
    pub data: Vec<f64>,
    pub(crate) keepalive: Vec<Node>,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.axes == other.axes && self.shape == other.shape && self.data == other.data
    }
}

impl Value {
    /// A 0-dimensional tensor (a scalar) — no axes.
    pub fn scalar(v: f64) -> Value {
        Value {
            axes: Vec::new(),
            shape: Vec::new(),
            data: vec![v],
            keepalive: Vec::new(),
        }
    }

    /// A concrete tensor from runtime shape and row-major data. Dynamic axis
    /// extents are supplied by `shape`; static extents are validated.
    pub fn from_data(axes: &[AxisRef], shape: Vec<usize>, data: Vec<f64>) -> Value {
        assert_eq!(
            axes.len(),
            shape.len(),
            "tensor rank {} does not match {} axes",
            shape.len(),
            axes.len()
        );
        for (&axis, &actual) in axes.iter().zip(&shape) {
            if let Extent::Static(expected) = axis.extent {
                assert_eq!(
                    actual, expected,
                    "axis `{}` has extent {actual}; expected {expected}",
                    axis.name
                );
            }
        }
        let elements = shape.iter().product::<usize>().max(1);
        assert_eq!(
            data.len(),
            elements,
            "shape {shape:?} requires {elements} values, received {}",
            data.len()
        );
        Value {
            axes: axes.to_vec(),
            shape,
            data,
            keepalive: Vec::new(),
        }
    }

    /// Construct row-major data by positional coordinates. Input nodes rebind
    /// these detached value dimensions to their own occurrences when read.
    pub fn from_shape_fn(shape: &[usize], mut f: impl FnMut(&[usize]) -> f64) -> Value {
        let descriptors = shape
            .iter()
            .map(|&extent| ir::axis("value", extent))
            .collect::<Vec<_>>();
        let holder = ir::input("__detached_value", &descriptors, crate::ir::Dtype::F64);
        let axes = ir::axis_refs(&holder);
        let total = shape.iter().product::<usize>().max(1);
        let mut data = Vec::with_capacity(total);
        let mut coordinate = vec![0usize; shape.len()];
        for _ in 0..total {
            data.push(f(&coordinate));
            for dim in (0..coordinate.len()).rev() {
                coordinate[dim] += 1;
                if coordinate[dim] < shape[dim] {
                    break;
                }
                coordinate[dim] = 0;
            }
        }
        Value {
            axes,
            shape: shape.to_vec(),
            data,
            keepalive: vec![holder],
        }
    }

    /// Read one element by positional coordinate.
    pub fn at_index(&self, coordinate: &[usize]) -> f64 {
        assert_eq!(coordinate.len(), self.shape.len(), "coordinate rank");
        let offset = coordinate
            .iter()
            .zip(self.strides())
            .map(|(&index, stride)| index * stride)
            .sum::<usize>();
        self.data[offset]
    }

    /// Construct an interpreter-internal value over resolved occurrences.
    fn from_ref_fn(axes: &[AxisRef], mut f: impl FnMut(&HashMap<AxisRef, usize>) -> f64) -> Value {
        let shape: Vec<usize> = axes.iter().map(|a| a.extent()).collect();
        let total: usize = shape.iter().product();
        let mut data = Vec::with_capacity(total);
        let mut coord = Coord::new(axes, &shape);
        for _ in 0..total {
            data.push(f(coord.map()));
            coord.step();
        }
        Value {
            axes: axes.to_vec(),
            shape,
            data,
            keepalive: Vec::new(),
        }
    }

    /// Row-major strides for `self.axes`.
    fn strides(&self) -> Vec<usize> {
        let mut s = vec![1usize; self.shape.len()];
        for i in (0..self.shape.len().saturating_sub(1)).rev() {
            s[i] = s[i + 1] * self.shape[i + 1];
        }
        s
    }

    /// Read the element at an assignment that covers (at least) `self.axes`.
    /// Axes of `self` absent from `assign` are a programming error — the
    /// caller must supply a full coordinate for this tensor's axes.
    pub fn at(&self, assign: &HashMap<AxisRef, usize>) -> f64 {
        let strides = self.strides();
        let mut off = 0;
        for (k, &a) in self.axes.iter().enumerate() {
            let i = assign
                .get(&a)
                .copied()
                .unwrap_or_else(|| panic!("coordinate missing axis {a} for tensor read"));
            debug_assert!(i < self.shape[k], "index {i} out of bounds for axis {a}");
            off += strides[k] * i;
        }
        self.data[off]
    }

    /// The same tensor with its dimensions reordered to `target` (a permutation
    /// of `self.axes`). Used to bring a user-supplied input into the axis order
    /// a node declares.
    pub fn permuted_to(&self, target: &[AxisRef]) -> Value {
        debug_assert_eq!(self.axes.len(), target.len(), "permute arity");
        let mut used = vec![false; self.axes.len()];
        let target = target
            .iter()
            .map(|key| {
                let position = self
                    .axes
                    .iter()
                    .enumerate()
                    .find(|(position, axis)| !used[*position] && *key == **axis)
                    .map(|(position, _)| position)
                    .expect("permutation target is not an axis occurrence of the value");
                used[position] = true;
                self.axes[position]
            })
            .collect::<Vec<_>>();
        if self.axes == target {
            return self.clone();
        }
        Value::from_ref_fn(&target, |c| self.at(c))
    }
}

// ── coordinate iteration ─────────────────────────────────────────────────────

/// A mixed-radix counter over a set of axes: enumerates every coordinate of a
/// shape in row-major order, exposing the current index of each axis.
struct Coord<'a> {
    axes: &'a [AxisRef],
    shape: &'a [usize],
    idx: Vec<usize>,
    map: HashMap<AxisRef, usize>,
}

impl<'a> Coord<'a> {
    fn new(axes: &'a [AxisRef], shape: &'a [usize]) -> Self {
        let map = axes.iter().map(|&a| (a, 0usize)).collect();
        Coord {
            axes,
            shape,
            idx: vec![0; axes.len()],
            map,
        }
    }

    fn map(&self) -> &HashMap<AxisRef, usize> {
        &self.map
    }

    /// Advance to the next coordinate (row-major: last axis fastest).
    fn step(&mut self) {
        for k in (0..self.axes.len()).rev() {
            self.idx[k] += 1;
            if self.idx[k] < self.shape[k] {
                self.map.insert(self.axes[k], self.idx[k]);
                return;
            }
            self.idx[k] = 0;
            self.map.insert(self.axes[k], 0);
        }
        // wrapped around fully — leaves all-zero, which callers don't observe
        // because they step exactly `product(shape)` times.
    }
}

// ── the evaluator ────────────────────────────────────────────────────────────

/// Evaluate `node` to a dense tensor under the given inputs.
///
/// This is the reference semantics of the IR. Shared sub-graphs are memoized
/// (by node identity), so a DAG with residuals and a reused normalizer costs
/// what its distinct nodes cost, not its unfolded tree.
pub fn eval(node: &Node, env: &Env) -> Value {
    crate::verify::assert_valid(node);
    let mut cache: HashMap<*const NodeKind, Rc<Value>> = HashMap::new();
    let mut value = (*eval_rc(node, env, &mut cache)).clone();
    value.keepalive.push(node.clone());
    value
}

fn eval_rc(node: &Node, env: &Env, cache: &mut HashMap<*const NodeKind, Rc<Value>>) -> Rc<Value> {
    let ptr = Arc::as_ptr(node);
    if let Some(t) = cache.get(&ptr) {
        return t.clone();
    }
    let t = Rc::new(eval_node(node, env, cache));
    cache.insert(ptr, t.clone());
    t
}

fn eval_node(node: &Node, env: &Env, cache: &mut HashMap<*const NodeKind, Rc<Value>>) -> Value {
    match node.as_ref() {
        NodeKind::Input { name, shape, .. } => {
            let t = env
                .get(name)
                .unwrap_or_else(|| panic!("no input tensor provided for `{name}`"));
            let axes = ir::axis_refs(node);
            let want: Vec<usize> = shape.iter().map(|axis| axis.extent()).collect();
            assert_eq!(
                t.shape, want,
                "input `{name}`: buffer shape {:?} does not match {want:?}",
                t.shape
            );
            Value {
                axes,
                shape: t.shape.clone(),
                data: t.data.clone(),
                keepalive: t.keepalive.clone(),
            }
        }

        NodeKind::Const { v } => Value::scalar(*v),

        NodeKind::Iota { .. } => {
            let axis = ir::axis_refs(node)[0];
            Value::from_ref_fn(&[axis], |c| c[&axis] as f64)
        }

        NodeKind::Coordinate { src, dim } => {
            let axes = ir::axis_refs(node);
            let source_axis = ir::axis_refs(src)[*dim];
            Value::from_ref_fn(&axes, |c| c[&source_axis] as f64)
        }

        NodeKind::Map { op, inputs } => {
            let ins: Vec<Rc<Value>> = inputs.iter().map(|n| eval_rc(n, env, cache)).collect();
            let oaxes = ir::axis_refs(node);
            let output_shape = node.shape();
            Value::from_ref_fn(&oaxes, |c| {
                let args: Vec<f64> = inputs
                    .iter()
                    .zip(&ins)
                    .map(|(input, tensor)| {
                        let input_shape = input.shape();
                        let lead = output_shape.len() - input_shape.len();
                        let mut coordinate = HashMap::new();
                        for (input_dim, axis) in tensor.axes.iter().copied().enumerate() {
                            let output_dim = lead + input_dim;
                            let index = if input_shape[input_dim].extent == Extent::Static(1) {
                                0
                            } else {
                                c[&oaxes[output_dim]]
                            };
                            coordinate.insert(axis, index);
                        }
                        tensor.at(&coordinate)
                    })
                    .collect();
                apply_map(*op, &args)
            })
        }

        NodeKind::Reduce { src, dim, op } => {
            let s = eval_rc(src, env, cache);
            let oaxes = ir::axis_refs(node);
            let axis = ir::source_axis(src, *dim);
            let n = s.shape[*dim];
            Value::from_ref_fn(&oaxes, |c| {
                let mut coord = c.clone();
                let mut acc = op.identity();
                for i in 0..n {
                    coord.insert(axis, i);
                    acc = monoid_combine(*op, acc, s.at(&coord));
                }
                acc
            })
        }

        NodeKind::Scan { src, dim, op } => eval_scan(src, *dim, *op, env, cache),

        NodeKind::Gather { src, index, dim } => {
            let s = eval_rc(src, env, cache);
            let idx = eval_rc(index, env, cache);
            let oaxes = ir::axis_refs(node);
            let bound = s.shape[*dim];
            Value::from_ref_fn(&oaxes, |c| {
                let mut index_coord = HashMap::new();
                for (index_dim, axis) in idx.axes.iter().copied().enumerate() {
                    index_coord.insert(axis, c[&oaxes[*dim + index_dim]]);
                }
                let i = idx.at(&index_coord).round() as usize;
                assert!(
                    i < bound,
                    "gather index {i} out of bounds for dimension {dim}"
                );

                let mut source_coord = HashMap::new();
                for (source_dim, axis) in s.axes.iter().copied().enumerate() {
                    let value = if source_dim < *dim {
                        c[&oaxes[source_dim]]
                    } else if source_dim == *dim {
                        i
                    } else {
                        c[&oaxes[source_dim - 1 + idx.axes.len()]]
                    };
                    source_coord.insert(axis, value);
                }
                s.at(&source_coord)
            })
        }

        NodeKind::View { src, dims } => {
            let s = eval_rc(src, env, cache);
            eval_view(&s, dims, node)
        }

        // Affine reindexing: each output coordinate computes the (signed)
        // source index of every mapped axis; out-of-range is 0.0 when padded,
        // an error otherwise. Pure index arithmetic — the definition the
        // emitters must reproduce.
        NodeKind::Reindex {
            src, map, padded, ..
        } => {
            let s = eval_rc(src, env, cache);
            let oaxes = ir::axis_refs(node);
            Value::from_ref_fn(&oaxes, |c| {
                let mut coord = HashMap::new();
                for (source_dim, terms, off) in map {
                    let mut idx: i64 = *off;
                    for (coef, output_dim) in terms {
                        idx += coef * c[&oaxes[*output_dim]] as i64;
                    }
                    let n = s.shape[*source_dim] as i64;
                    if idx < 0 || idx >= n {
                        assert!(
                            padded,
                            "reindex: index {idx} out of bounds for dimension {source_dim} (extent {n}) \
                             and the node is not padded"
                        );
                        return 0.0;
                    }
                    coord.insert(s.axes[*source_dim], idx as usize);
                }
                s.at(&coord)
            })
        }
    }
}

/// An inclusive prefix fold. The output shares the source's shape.
fn eval_scan(
    src: &Node,
    dim: usize,
    op: Monoid,
    env: &Env,
    cache: &mut HashMap<*const NodeKind, Rc<Value>>,
) -> Value {
    let s = eval_rc(src, env, cache);
    let axis = ir::source_axis(src, dim);
    Value::from_ref_fn(&s.axes.clone(), |c| {
        // prefix fold up to and including this coordinate's position on `axis`
        let upto = c[&axis];
        let mut coord = c.clone();
        let mut acc = op.identity();
        for i in 0..=upto {
            coord.insert(axis, i);
            acc = monoid_combine(op, acc, s.at(&coord));
        }
        acc
    })
}

/// Evaluate a `View`: relabel (rename) or merge (flatten) index spaces with no
/// computation. Each source coordinate maps to exactly one output coordinate,
/// so this is a pure data reshuffle.
fn eval_view(s: &Value, dims: &[ir::ViewDim], node: &Node) -> Value {
    let output_axes = ir::axis_refs(node);
    Value::from_ref_fn(&output_axes, |output| {
        let mut source = HashMap::new();
        for (output_dim, view_dim) in dims.iter().enumerate() {
            let mut flat = output[&output_axes[output_dim]];
            for &source_dim in view_dim.sources.iter().rev() {
                let extent = s.shape[source_dim];
                source.insert(s.axes[source_dim], flat % extent);
                flat /= extent;
            }
            debug_assert_eq!(flat, 0, "view coordinate exceeds flattened extent");
        }
        s.at(&source)
    })
}

// ── scalar semantics ─────────────────────────────────────────────────────────

/// The scalar meaning of each [`MapOp`], matching `ir::MapOp` exactly.
fn apply_map(op: MapOp, a: &[f64]) -> f64 {
    match op {
        MapOp::Add => a[0] + a[1],
        MapOp::Sub => a[0] - a[1],
        MapOp::Mul => a[0] * a[1],
        MapOp::Div => a[0] / a[1],
        MapOp::Max => a[0].max(a[1]),
        MapOp::Min => a[0].min(a[1]),
        MapOp::Lt => (a[0] < a[1]) as u8 as f64,
        MapOp::Neg => -a[0],
        MapOp::Recip => 1.0 / a[0],
        MapOp::Exp => a[0].exp(),
        MapOp::Log => a[0].ln(),
        MapOp::Sqrt => a[0].sqrt(),
        MapOp::Tanh => a[0].tanh(),
        MapOp::Sin => a[0].sin(),
        MapOp::Cos => a[0].cos(),
        MapOp::Where => {
            if a[0] != 0.0 {
                a[1]
            } else {
                a[2]
            }
        }
    }
}

fn monoid_combine(m: Monoid, a: f64, b: f64) -> f64 {
    match m {
        Monoid::Add => a + b,
        Monoid::Mul => a * b,
        Monoid::Max => a.max(b),
        Monoid::Min => a.min(b),
        // numerically-stable log(exp(a) + exp(b)); identity is −∞.
        Monoid::LogSumExp => {
            if a == f64::NEG_INFINITY {
                b
            } else if b == f64::NEG_INFINITY {
                a
            } else {
                let m = a.max(b);
                m + ((a - m).exp() + (b - m).exp()).ln()
            }
        }
    }
}

// ── driving a derived kernel on real tensors ─────────────────────────────────

/// Run a derived `carrier` over real input tensors, exactly as a generated
/// kernel would: for each point of the output grid (the free axes), stream the
/// folded `axis`, lifting each element from the carrier's leaves and combining.
///
/// The carrier's leaves are graph nodes free along `axis` (a raw input, an
/// index, or a contraction like FlashAttention's QKᵀ). Evaluating them with
/// [`eval`] and folding with the carrier's own `lift`/`merge`/`project` is the
/// same arithmetic the emitted kernel performs — so agreement with
/// `eval(node)` certifies the kernel computes the real math.
///
/// Requires a scalar-projecting carrier (`project.len() == 1`), which covers
/// attention, matmul, softmax-weighted sums, RMS/variance, and logsumexp.
pub fn run_carrier(node: &Node, axis: impl AxisSelector, carrier: &Carrier, env: &Env) -> Value {
    run_carrier_split(node, axis, carrier, 1, env)
}

/// Run a derived carrier as a TWO-STAGE split reduction (the GROUP schedule):
/// partition the streamed axis into `blocks` contiguous chunks, fold each
/// chunk into a raw accumulator (stage 1 — the parallel partials), then merge
/// the partials with the carrier's own associative `combine` and project
/// (stage 2). Equal to the one-pass fold *by the monoid law* — this function
/// is the oracle a split-reduction backend is checked against, and with
/// `blocks = 1` it is exactly [`run_carrier`].
///
/// The same re-association is what data parallelism is: each "device" folds
/// its shard of the axis, and the allreduce is stage 2's merge.
///
/// `blocks` may not exceed the axis extent: an empty chunk's partial is the
/// carrier identity, and merging an identity-valued *accumulator* puts the
/// online-softmax rescale at `exp(−∞ − −∞)` — the same −∞ edge
/// causal masking documents. Keeping every chunk non-empty keeps
/// the whole computation in the finite domain, exactly as real split-K flash
/// kernels do.
pub fn run_carrier_split(
    node: &Node,
    axis: impl AxisSelector,
    carrier: &Carrier,
    blocks: usize,
    env: &Env,
) -> Value {
    let axis = axis
        .resolve_axis(node, "run_carrier")
        .expect("carrier axis is absent from the selected node");
    assert_eq!(
        carrier.project.len(),
        1,
        "run_carrier supports scalar-projecting carriers; got {} outputs",
        carrier.project.len()
    );

    let grid = ir::axis_refs(node); // the free axes — the kernel's parallel grid
    let n = axis.extent();
    assert!(
        blocks >= 1 && blocks <= n,
        "blocks must be in [1, extent({axis}) = {n}]; an empty chunk's identity \
         partial would hit the −∞ rescale edge"
    );

    // Evaluate every leaf once, over its full shape (including `axis` where the
    // leaf depends on it), through one shared cache.
    let mut cache: HashMap<*const NodeKind, Rc<Value>> = HashMap::new();
    let leaves: Vec<Rc<Value>> = carrier
        .leaves
        .iter()
        .map(|l| {
            let t = eval_rc(l, env, &mut cache);
            for &a in &t.axes {
                if a.extent == Extent::Static(1) {
                    continue;
                }
                let a = carrier.aliases.get(&a).copied().unwrap_or(a);
                assert!(
                    a == axis || grid.contains(&a),
                    "leaf exposes axis {a} outside grid ∪ {{stream}} — \
                     not a valid per-element input"
                );
            }
            t
        })
        .collect();

    let mut value = Value::from_ref_fn(&grid, |gc| {
        let mut coord = gc.clone();
        // stage 1: one raw accumulator per chunk (identity-valued when the
        // chunk is empty — blocks may exceed n and the law still holds)
        let mut partials: Vec<Vec<f64>> = Vec::with_capacity(blocks);
        for b in 0..blocks {
            let (lo, hi) = (b * n / blocks, (b + 1) * n / blocks);
            let mut acc = carrier.identity.clone();
            for i in lo..hi {
                coord.insert(axis, i);
                let item: Vec<f64> = leaves
                    .iter()
                    .map(|tensor| {
                        let local = tensor
                            .axes
                            .iter()
                            .copied()
                            .map(|local_axis| {
                                let value = if local_axis.extent == Extent::Static(1) {
                                    0
                                } else {
                                    let canonical = carrier
                                        .aliases
                                        .get(&local_axis)
                                        .copied()
                                        .unwrap_or(local_axis);
                                    coord[&canonical]
                                };
                                (local_axis, value)
                            })
                            .collect::<HashMap<_, _>>();
                        tensor.at(&local)
                    })
                    .collect();
                let el = carrier.lift(&item);
                acc = carrier.merge(&acc, &el);
            }
            partials.push(acc);
        }
        // stage 2: merge the partials, project once. Start from the first
        // partial; this also avoids asking unusual carriers to merge an
        // identity-valued partial before any real data.
        let mut acc = partials[0].clone();
        for p in &partials[1..] {
            acc = carrier.merge(&acc, p);
        }
        carrier.project(&acc)[0]
    });
    value.keepalive.push(node.clone());
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derive::derive;
    use crate::ir::*;
    use crate::nn::scaled_dot_product_attention;

    struct Lcg(u64);
    impl Lcg {
        fn f(&mut self) -> f64 {
            let mut x = self.0;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            self.0 = x;
            ((x.wrapping_mul(0x2545F4914F6CDD1D) >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0
        }
    }

    fn rand_tensor(name_axes: &[Axis], rng: &mut Lcg) -> Value {
        let shape = name_axes
            .iter()
            .map(|axis| axis.extent())
            .collect::<Vec<_>>();
        Value::from_shape_fn(&shape, |_| rng.f())
    }

    fn approx(a: f64, b: f64) {
        let tol = 1e-9 * (1.0 + a.abs().max(b.abs()));
        assert!((a - b).abs() <= tol, "{a} vs {b}");
    }

    fn assert_tensors_eq(x: &Value, y: &Value) {
        let y = y.permuted_to(&x.axes);
        assert_eq!(x.shape, y.shape, "shape");
        for (a, b) in x.data.iter().zip(&y.data) {
            approx(*a, *b);
        }
    }

    // eval computes an honest matmul.
    #[test]
    fn eval_matmul_matches_naive() {
        let (i, j, k) = (axis("i", 3), axis("j", 4), axis("k", 5));
        let mut rng = Lcg(1);
        let a = rand_tensor(&[i, k], &mut rng);
        let b = rand_tensor(&[k, j], &mut rng);
        let env: Env = [("A", a.clone()), ("B", b.clone())].into_iter().collect();

        let mm = matmul(
            input("A", [i, k], Dtype::F32),
            input("B", [k, j], Dtype::F32),
        );
        let got = eval(&mm, &env);

        let want = Value::from_shape_fn(&[i.extent(), j.extent()], |c| {
            (0..5)
                .map(|kk| a.data[c[0] * 5 + kk] * b.data[kk * 4 + c[1]])
                .sum()
        });
        assert_eq!(got.shape, want.shape);
        for (actual, expected) in got.data.iter().zip(&want.data) {
            approx(*actual, *expected);
        }
    }

    // softmax rows are non-negative and sum to one.
    #[test]
    fn eval_softmax_normalizes() {
        let (r, k) = (axis("r", 4), axis("k", 7));
        let mut rng = Lcg(9);
        let x = rand_tensor(&[r, k], &mut rng);
        let env: Env = [("X", x)].into_iter().collect();
        let sm = eval(&softmax(input("X", [r, k], Dtype::F32), 1usize), &env);
        for r_i in 0..4 {
            let mut s = 0.0;
            for k_i in 0..7 {
                let v = sm.data[r_i * 7 + k_i];
                assert!(v >= 0.0);
                s += v;
            }
            approx(s, 1.0);
        }
    }

    // THE headline: the derived FlashAttention carrier, driven on real Q/K/V,
    // computes exactly what the naive attention graph evaluates to.
    #[test]
    fn derived_flash_kernel_equals_naive_attention() {
        let (sq, k, d, e) = (axis("sq", 6), axis("k", 9), axis("d", 8), axis("e", 5));
        let mut rng = Lcg(0xF1A5);
        let q = rand_tensor(&[sq, d], &mut rng);
        let kk = rand_tensor(&[k, d], &mut rng);
        let v = rand_tensor(&[k, e], &mut rng);
        let env: Env = [("Q", q), ("K", kk), ("V", v)].into_iter().collect();

        let key = input("K", [k, d], Dtype::F32);
        let key_axis = axis_refs(&key)[0];
        let attn = scaled_dot_product_attention(
            input("Q", [sq, d], Dtype::F32),
            key,
            input("V", [k, e], Dtype::F32),
            None,
            0.0,
            false,
            Some(1.0),
            false,
        );
        let reference = eval(&attn, &env);

        let carrier = derive(&attn, key_axis).unwrap();
        let via_kernel = run_carrier(&attn, key_axis, &carrier, &env);

        assert_tensors_eq(&via_kernel, &reference);
    }

    // The same, with the computed causal mask and score scaling fused into the
    // lift — the kernel must match a naive masked-softmax reference.
    #[test]
    fn derived_causal_flash_equals_naive() {
        let (s, t, dk, dv) = (axis("s", 7), axis("t", 7), axis("dk", 8), axis("dv", 6));
        let mut rng = Lcg(0xC0FFEE);
        let q = rand_tensor(&[s, dk], &mut rng);
        let kk = rand_tensor(&[t, dk], &mut rng);
        let v = rand_tensor(&[t, dv], &mut rng);
        let env: Env = [("Q", q), ("K", kk), ("V", v)].into_iter().collect();

        let key = input("K", [t, dk], Dtype::F32);
        let key_axis = axis_refs(&key)[0];
        let attn = scaled_dot_product_attention(
            input("Q", [s, dk], Dtype::F32),
            key,
            input("V", [t, dv], Dtype::F32),
            None,
            0.0,
            true,
            Some(0.125),
            false,
        );

        let reference = eval(&attn, &env);
        let carrier = derive(&attn, key_axis).unwrap();
        let via_kernel = run_carrier(&attn, key_axis, &carrier, &env);
        assert_tensors_eq(&via_kernel, &reference);
    }

    // Embedding lookup: a gather along the vocabulary axis reads the right rows.
    #[test]
    fn eval_embedding_gather() {
        let (v, dm, s) = (axis("v", 10), axis("dm", 4), axis("s", 3));
        let mut rng = Lcg(7);
        let table = rand_tensor(&[v, dm], &mut rng);
        let ids = Value::from_shape_fn(&[s.extent()], |c| [2.0, 7.0, 0.0][c[0]]);
        let env: Env = [("E", table.clone()), ("ids", ids)].into_iter().collect();

        let emb = embedding(
            input("E", [v, dm], Dtype::F32),
            input("ids", [s], Dtype::F32),
            0usize,
        );
        let got = eval(&emb, &env);
        // Gather replaces the selected source dimension with the index shape.
        for (row, &id) in [2usize, 7, 0].iter().enumerate() {
            for dd in 0..4 {
                approx(got.data[row * 4 + dd], table.data[id * 4 + dd]);
            }
        }
    }

    // A flatten view merges two axes row-major (first member most significant).
    #[test]
    fn eval_view_flatten_is_row_major() {
        let (h, dv, dmv) = (axis("h", 2), axis("dv", 3), axis("dmv", 6));
        let mut rng = Lcg(3);
        let x = rand_tensor(&[h, dv], &mut rng);
        let env: Env = [("X", x.clone())].into_iter().collect();
        let flat = eval(
            &flatten(input("X", [h, dv], Dtype::F32), &[0, 1][..], dmv),
            &env,
        );
        assert_eq!(flat.shape, vec![6]);
        for hh in 0..2 {
            for dd in 0..3 {
                approx(flat.data[hh * 3 + dd], x.data[hh * 3 + dd]);
            }
        }
    }
}
