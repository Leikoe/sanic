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
//! Scope: the elementwise · reduce · gather · view · iota core — i.e. the
//! whole attention / transformer vocabulary. Monoidal `Scan` (prefix folds)
//! is evaluated too; the affine-compose and non-associative recurrences are
//! rejected with a clear message rather than guessed at, so the oracle never
//! lies about a computation it does not actually implement.

use std::collections::HashMap;
use std::rc::Rc;

use crate::derive::Carrier;
use crate::kernel_ir::{Axis, BinOp, MapOp, Monoid, Node as NodeKind, NodeRef as Node};

/// Input tensors by leaf name.
pub type Env = HashMap<&'static str, Value>;

// ── dense tensors ────────────────────────────────────────────────────────────

/// A dense, row-major tensor tagged with the axis each dimension ranges over —
/// the interpreter's value domain: what a graph *means* on concrete data.
/// (The public graph is made of [`crate::ir::NodeRef`] values; this is data.)
/// Lowered axes are opaque semantic identities; their labels are diagnostic.
/// The position in `axes` is the storage order. Two values with the same
/// internal axes in different orders denote the same lowered object (see
/// [`Value::permuted_to`]).
#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub axes: Vec<Axis>,
    pub shape: Vec<usize>,
    pub data: Vec<f64>,
}

impl Value {
    /// A 0-dimensional tensor (a scalar) — no axes.
    pub fn scalar(v: f64) -> Value {
        Value {
            axes: Vec::new(),
            shape: Vec::new(),
            data: vec![v],
        }
    }

    /// A row-major tensor described only by its positional shape.
    pub fn from_shape(shape: impl AsRef<[usize]>, data: Vec<f64>) -> Value {
        let shape = shape.as_ref().to_vec();
        let axes: Vec<Axis> = shape
            .iter()
            .map(|&extent| crate::kernel_ir::axis("", extent))
            .collect();
        Value::from_data(&axes, shape, data)
    }

    /// A concrete tensor from runtime shape and row-major data. Dynamic axis
    /// extents are supplied by `shape`; static extents are validated.
    pub fn from_data(axes: &[Axis], shape: Vec<usize>, data: Vec<f64>) -> Value {
        assert_eq!(
            axes.len(),
            shape.len(),
            "tensor rank {} does not match {} axes",
            shape.len(),
            axes.len()
        );
        for (&axis, &actual) in axes.iter().zip(&shape) {
            if let crate::kernel_ir::Extent::Static(expected) = axis.extent {
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
        }
    }

    /// A tensor over the given axes (shape read off their extents), filled by
    /// a closure over the per-axis coordinate (a map from each axis to its
    /// index).
    pub fn from_fn(axes: &[Axis], mut f: impl FnMut(&HashMap<Axis, usize>) -> f64) -> Value {
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
    pub fn at(&self, assign: &HashMap<Axis, usize>) -> f64 {
        let strides = self.strides();
        let mut off = 0;
        for (k, &a) in self.axes.iter().enumerate() {
            let i = *assign
                .get(&a)
                .unwrap_or_else(|| panic!("coordinate missing axis {a} for tensor read"));
            debug_assert!(i < self.shape[k], "index {i} out of bounds for axis {a}");
            off += strides[k] * i;
        }
        self.data[off]
    }

    /// The same tensor with its dimensions reordered to `target` (a permutation
    /// of `self.axes`). Used to bring a user-supplied input into the axis order
    /// a node declares.
    pub fn permuted_to(&self, target: &[Axis]) -> Value {
        debug_assert_eq!(self.axes.len(), target.len(), "permute arity");
        if self.axes == target {
            return self.clone();
        }
        Value::from_fn(target, |c| self.at(c))
    }
}

// ── coordinate iteration ─────────────────────────────────────────────────────

/// A mixed-radix counter over a set of axes: enumerates every coordinate of a
/// shape in row-major order, exposing the current index of each axis.
struct Coord<'a> {
    axes: &'a [Axis],
    shape: &'a [usize],
    idx: Vec<usize>,
    map: HashMap<Axis, usize>,
}

impl<'a> Coord<'a> {
    fn new(axes: &'a [Axis], shape: &'a [usize]) -> Self {
        let map = axes.iter().map(|&a| (a, 0usize)).collect();
        Coord {
            axes,
            shape,
            idx: vec![0; axes.len()],
            map,
        }
    }

    fn map(&self) -> &HashMap<Axis, usize> {
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
    (*eval_rc(node, env, &mut cache)).clone()
}

fn eval_rc(node: &Node, env: &Env, cache: &mut HashMap<*const NodeKind, Rc<Value>>) -> Rc<Value> {
    let ptr = Rc::as_ptr(node);
    if let Some(t) = cache.get(&ptr) {
        return t.clone();
    }
    let t = Rc::new(eval_node(node, env, cache));
    cache.insert(ptr, t.clone());
    t
}

fn eval_node(node: &Node, env: &Env, cache: &mut HashMap<*const NodeKind, Rc<Value>>) -> Value {
    match node.as_ref() {
        NodeKind::Input { name, axes, .. } => {
            let t = env
                .get(name)
                .unwrap_or_else(|| panic!("no input tensor provided for `{name}`"));
            if sorted(&t.axes) == sorted(axes) {
                // same labels, maybe reordered — bring into declared order.
                t.permuted_to(axes)
            } else {
                // Different labels over the same storage: the "one buffer, two
                // index spaces" aliasing (a normalized tensor read at both
                // query and key positions). Rebind the buffer to the declared
                // axes *positionally* — renames preserve dimension order, so
                // this is a pure relabel, valid exactly when the extents line
                // up position-for-position.
                let want: Vec<usize> = axes.iter().map(|a| a.extent()).collect();
                assert_eq!(
                    t.shape, want,
                    "input `{name}`: buffer shape {:?} cannot be rebound to axes {axes:?}",
                    t.shape
                );
                Value {
                    axes: axes.clone(),
                    shape: t.shape.clone(),
                    data: t.data.clone(),
                }
            }
        }

        NodeKind::Const { v } => Value::scalar(*v),

        NodeKind::Iota { axis } => Value::from_fn(&[*axis], |c| c[axis] as f64),

        NodeKind::Map { op, inputs } => {
            let ins: Vec<Rc<Value>> = inputs.iter().map(|n| eval_rc(n, env, cache)).collect();
            let oaxes = node.shape();
            Value::from_fn(&oaxes, |c| {
                let args: Vec<f64> = ins.iter().map(|t| t.at(c)).collect();
                apply_map(*op, &args)
            })
        }

        NodeKind::Reduce { src, axis, op } => {
            let s = eval_rc(src, env, cache);
            let oaxes: Vec<Axis> = s.axes.iter().copied().filter(|a| a != axis).collect();
            let n = axis.extent();
            // k-best: stable descending sort = first-max-wins ranks
            if let BinOp::TopK { k: _, rank, idx } = op {
                let (rank, idx) = (*rank as usize, *idx);
                return Value::from_fn(&oaxes, |c| {
                    let mut coord = c.clone();
                    let mut items: Vec<(f64, usize)> = (0..n)
                        .map(|i| {
                            coord.insert(*axis, i);
                            (s.at(&coord), i)
                        })
                        .collect();
                    items
                        .sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
                    match items.get(rank) {
                        Some(&(v, i)) => {
                            if idx {
                                i as f64
                            } else {
                                v
                            }
                        }
                        None => {
                            if idx {
                                0.0
                            } else {
                                f64::NEG_INFINITY
                            }
                        }
                    }
                });
            }
            // the index-carrying maximum: value = position of the FIRST max
            if let BinOp::ArgMax = op {
                return Value::from_fn(&oaxes, |c| {
                    let mut coord = c.clone();
                    let (mut m, mut mi) = (f64::NEG_INFINITY, 0usize);
                    for i in 0..n {
                        coord.insert(*axis, i);
                        let v = s.at(&coord);
                        if v > m {
                            m = v;
                            mi = i;
                        }
                    }
                    mi as f64
                });
            }
            Value::from_fn(&oaxes, |c| {
                let mut coord = c.clone();
                let mut acc = binop_identity(*op);
                for i in 0..n {
                    coord.insert(*axis, i);
                    acc = binop_combine(*op, acc, s.at(&coord));
                }
                acc
            })
        }

        NodeKind::Scan { src, axis, op } => eval_scan(src, *axis, *op, env, cache),

        NodeKind::Gather { src, index, axis } => {
            let s = eval_rc(src, env, cache);
            let idx = eval_rc(index, env, cache);
            let oaxes = node.shape();
            let bound = axis.extent();
            Value::from_fn(&oaxes, |c| {
                let i = idx.at(c).round() as usize;
                assert!(i < bound, "gather index {i} out of bounds for axis {axis}");
                let mut coord = c.clone();
                coord.insert(*axis, i);
                s.at(&coord)
            })
        }

        NodeKind::View { src, groups } => {
            let s = eval_rc(src, env, cache);
            eval_view(&s, groups, node)
        }

        // Affine reindexing: each output coordinate computes the (signed)
        // source index of every mapped axis; out-of-range is 0.0 when padded,
        // an error otherwise. Pure index arithmetic — the definition the
        // emitters must reproduce.
        NodeKind::Reindex { src, map, padded } => {
            let s = eval_rc(src, env, cache);
            let oaxes = node.shape();
            Value::from_fn(&oaxes, |c| {
                let mut coord = c.clone();
                for (m, terms, off) in map {
                    let mut idx: i64 = *off;
                    for (coef, a) in terms {
                        idx += coef * c[a] as i64;
                    }
                    let n = src_extent(&s, *m) as i64;
                    if idx < 0 || idx >= n {
                        assert!(
                            padded,
                            "reindex: index {idx} out of bounds for axis {m} (extent {n}) \
                             and the node is not padded"
                        );
                        return 0.0;
                    }
                    coord.insert(*m, idx as usize);
                }
                s.at(&coord)
            })
        }
    }
}

/// A prefix scan. Monoidal steps fold as a running accumulator (output shares
/// the source's shape). The affine and non-associative recurrences are not
/// implemented here on purpose: the oracle must not fabricate semantics it
/// cannot state exactly, so it refuses rather than guesses.
fn eval_scan(
    src: &Node,
    axis: Axis,
    op: BinOp,
    env: &Env,
    cache: &mut HashMap<*const NodeKind, Rc<Value>>,
) -> Value {
    let BinOp::Monoid(m) = op else {
        panic!(
            "interp: {op:?} scan is not implemented in the reference oracle \
             (only monoidal prefix scans are); extend before relying on it"
        );
    };
    let s = eval_rc(src, env, cache);
    let n = axis.extent();
    Value::from_fn(&s.axes.clone(), |c| {
        // prefix fold up to and including this coordinate's position on `axis`
        let upto = c[&axis];
        let mut coord = c.clone();
        let mut acc = m.identity();
        for i in 0..=upto {
            coord.insert(axis, i);
            acc = monoid_combine(m, acc, s.at(&coord));
        }
        let _ = n;
        acc
    })
}

/// Evaluate a `View`: relabel (rename) or merge (flatten) index spaces with no
/// computation. Each source coordinate maps to exactly one output coordinate,
/// so this is a pure data reshuffle.
fn eval_view(s: &Value, groups: &[(Vec<Axis>, Axis)], node: &Node) -> Value {
    let oaxes = node.shape();
    // Extent of each output axis: a plain pass-through keeps its extent; a
    // flattened group's extent is the product of its members'.
    let ext_of = |a: Axis| -> usize {
        if let Some((members, _)) = groups.iter().find(|(_, to)| *to == a) {
            members.iter().map(|m| src_extent(s, *m)).product()
        } else {
            src_extent(s, a)
        }
    };
    let out_shape: Vec<usize> = oaxes.iter().map(|&a| ext_of(a)).collect();
    // Cross-check against each output axis's declared extent: a flatten whose
    // target axis was minted with the wrong product dies here, named.
    for (i, &a) in oaxes.iter().enumerate() {
        assert_eq!(
            a.extent(),
            out_shape[i],
            "view output extent mismatch on {a}"
        );
    }

    let mut out = Value {
        axes: oaxes.clone(),
        shape: out_shape.clone(),
        data: vec![0.0; out_shape.iter().product()],
    };
    let out_strides = out.strides();

    // Walk every source coordinate, scatter its value to the mapped output cell.
    let total: usize = s.shape.iter().product();
    let mut coord = Coord::new(&s.axes, &s.shape);
    for _ in 0..total {
        let a = coord.map();
        let mut off = 0usize;
        for (k, &oax) in oaxes.iter().enumerate() {
            let coordinate = if let Some((members, _)) = groups.iter().find(|(_, to)| *to == oax) {
                // row-major merge: first member most significant.
                let mut c = 0usize;
                for &m in members {
                    c = c * src_extent(s, m) + a[&m];
                }
                c
            } else {
                a[&oax]
            };
            off += out_strides[k] * coordinate;
        }
        out.data[off] = s.at(a);
        coord.step();
    }
    out
}

/// Extent of `axis` as seen at the source of a view: prefer the source
/// tensor's own shape, fall back to the axis's declared extent.
fn src_extent(s: &Value, axis: Axis) -> usize {
    s.axes
        .iter()
        .position(|a| *a == axis)
        .map(|k| s.shape[k])
        .unwrap_or_else(|| axis.extent())
}

fn sorted(axes: &[Axis]) -> Vec<Axis> {
    let mut v = axes.to_vec();
    v.sort();
    v
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

fn binop_identity(op: BinOp) -> f64 {
    match op {
        BinOp::Monoid(m) => m.identity(),
        other => panic!("interp: reduce with {other:?} is not a monoid"),
    }
}

fn binop_combine(op: BinOp, a: f64, b: f64) -> f64 {
    match op {
        BinOp::Monoid(m) => monoid_combine(m, a, b),
        other => panic!("interp: reduce with {other:?} is not a monoid"),
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
pub fn run_carrier(node: &Node, axis: Axis, carrier: &Carrier, env: &Env) -> Value {
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
/// [`crate::kernel_ir::causal_mask`] documents. Keeping every chunk non-empty keeps
/// the whole computation in the finite domain, exactly as real split-K flash
/// kernels do.
pub fn run_carrier_split(
    node: &Node,
    axis: Axis,
    carrier: &Carrier,
    blocks: usize,
    env: &Env,
) -> Value {
    assert_eq!(
        carrier.project.len(),
        1,
        "run_carrier supports scalar-projecting carriers; got {} outputs",
        carrier.project.len()
    );

    let grid = node.shape(); // the free axes — the kernel's parallel grid
    let n = axis.extent();
    assert!(
        blocks == 1
            || !carrier.kinds.iter().any(|k| matches!(
                k,
                crate::derive::SlotKind::KBestVal { .. } | crate::derive::SlotKind::KBestIdx { .. }
            )),
        "split reduction: a k-best carrier's combine is the singleton insert, \
         not a two-list merge — partials cannot be merged"
    );
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
                assert!(
                    a == axis || grid.contains(&a),
                    "leaf exposes axis {a} outside grid ∪ {{stream}} — \
                     not a valid per-element input"
                );
            }
            t
        })
        .collect();

    Value::from_fn(&grid, |gc| {
        let mut coord = gc.clone();
        // stage 1: one raw accumulator per chunk (identity-valued when the
        // chunk is empty — blocks may exceed n and the law still holds)
        let mut partials: Vec<Vec<f64>> = Vec::with_capacity(blocks);
        for b in 0..blocks {
            let (lo, hi) = (b * n / blocks, (b + 1) * n / blocks);
            let mut acc = carrier.identity.clone();
            for i in lo..hi {
                coord.insert(axis, i);
                let item: Vec<f64> = leaves.iter().map(|t| t.at(&coord)).collect();
                let el = carrier.lift(&item);
                acc = carrier.merge(&acc, &el);
            }
            partials.push(acc);
        }
        // stage 2: merge the partials, project once. Start from the first
        // partial, not the identity: for a monoid the two are equal, and a
        // k-best carrier (whose combine is the singleton insert, legal only
        // at blocks = 1) must not see its full list on the B side of a merge.
        let mut acc = partials[0].clone();
        for p in &partials[1..] {
            acc = carrier.merge(&acc, p);
        }
        // A projection may read leaves that are constant along the streamed
        // axis (a grid-axis one-hot picking this output's rank). A
        // stream-varying leaf is meaningless here: poison it so misuse is
        // loud, not silently the last element.
        let proj_items: Vec<f64> = leaves
            .iter()
            .map(|t| {
                if t.axes.contains(&axis) {
                    f64::NAN
                } else {
                    t.at(gc)
                }
            })
            .collect();
        carrier.project_with(&acc, &proj_items)[0]
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derive::derive;
    use crate::kernel_ir::*;

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
        Value::from_fn(name_axes, |_| rng.f())
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
            input("A", &[i, k], Dtype::F32),
            input("B", &[k, j], Dtype::F32),
            k,
        );
        let got = eval(&mm, &env);

        let want = Value::from_fn(&[i, j], |c| {
            (0..5)
                .map(|kk| a.data[c[&i] * 5 + kk] * b.data[kk * 4 + c[&j]])
                .sum()
        });
        assert_tensors_eq(&got, &want);
    }

    // softmax rows are non-negative and sum to one.
    #[test]
    fn eval_softmax_normalizes() {
        let (r, k) = (axis("r", 4), axis("k", 7));
        let mut rng = Lcg(9);
        let x = rand_tensor(&[r, k], &mut rng);
        let env: Env = [("X", x)].into_iter().collect();
        let sm = eval(&softmax(input("X", &[r, k], Dtype::F32), k), &env);
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

        let attn = attention(
            input("Q", &[sq, d], Dtype::F32),
            input("K", &[k, d], Dtype::F32),
            input("V", &[k, e], Dtype::F32),
            d,
            k,
        );
        let reference = eval(&attn, &env);

        let carrier = derive(&attn, k).unwrap();
        let via_kernel = run_carrier(&attn, k, &carrier, &env);

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

        let scores = matmul(
            input("Q", &[s, dk], Dtype::F32),
            input("K", &[t, dk], Dtype::F32),
            dk,
        );
        let scaled = map(MapOp::Mul, vec![scores, konst(0.125)]);
        let masked = map(MapOp::Add, vec![scaled, causal_mask(s, t)]);
        let attn = matmul(softmax(masked, t), input("V", &[t, dv], Dtype::F32), t);

        let reference = eval(&attn, &env);
        let carrier = derive(&attn, t).unwrap();
        let via_kernel = run_carrier(&attn, t, &carrier, &env);
        assert_tensors_eq(&via_kernel, &reference);
    }

    // Embedding lookup: a gather along the vocabulary axis reads the right rows.
    #[test]
    fn eval_embedding_gather() {
        let (v, dm, s) = (axis("v", 10), axis("dm", 4), axis("s", 3));
        let mut rng = Lcg(7);
        let table = rand_tensor(&[v, dm], &mut rng);
        let ids = Value::from_fn(&[s], |c| [2.0, 7.0, 0.0][c[&s]]);
        let env: Env = [("E", table.clone()), ("ids", ids)].into_iter().collect();

        let emb = embedding(
            input("E", &[v, dm], Dtype::F32),
            input("ids", &[s], Dtype::F32),
            v,
        );
        let got = eval(&emb, &env);
        // read by coordinate, not raw offset — gather's output axis order is
        // (kept ∪ index) = [dm, s], and `at` is layout-agnostic.
        for (row, &id) in [2usize, 7, 0].iter().enumerate() {
            for dd in 0..4 {
                let got_c = HashMap::from([(s, row), (dm, dd)]);
                let tab_c = HashMap::from([(v, id), (dm, dd)]);
                approx(got.at(&got_c), table.at(&tab_c));
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
            &flatten(input("X", &[h, dv], Dtype::F32), &[h, dv], dmv),
            &env,
        );
        assert_eq!(flat.axes, vec![dmv]);
        for hh in 0..2 {
            for dd in 0..3 {
                approx(flat.data[hh * 3 + dd], x.data[hh * 3 + dd]);
            }
        }
    }
}
