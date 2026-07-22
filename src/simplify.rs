//! Algebraic simplification of a graph — the ring/lattice identities and the
//! one distributivity law the engine already trusts, applied as rewrites to a
//! fixpoint, with common-subexpression elimination.
//!
//! Its reason to exist is the backward pass. [`crate::grad`] transposes a
//! graph faithfully but naively: differentiating logsumexp's numerically
//! stable `m + log(Σ exp(x − m))` emits a winner-mask term for the `max` that
//! is multiplied by a cotangent which is algebraically ZERO. The `max` appears
//! twice — added back (`+ m`) and inside the shift (`x − m`) — so its two
//! contributions are `+g` and `−g`. This is not a stop-gradient: the
//! cancellation is *derivable*, and the fact that makes the `−g` appear is
//! that the gradient's `Σ exp(x − m)` IS the forward's denominator `s`.
//!
//! So the rewrites needed are exactly:
//!
//! * **factor an invariant out of an `Add`-reduce** — `Σ(k·x) = k·Σx` when
//!   `k` does not carry the axis (the same defer-scale `derive` uses), and
//!   `Σ(−x) = −Σx`;
//! * **CSE** — hash-consing makes the gradient's `Σ exp(x − m)` share the
//!   forward `s`'s node, so `(g/s)·Σexp` becomes `(g/s)·s`;
//! * **ring identities** — `(a/b)·b → a`, `x − x → 0`, `k·0 → 0`, `x + 0 → x`.
//!
//! Then `m`'s cotangent is `0`, the winner-mask is `winner·0 = 0`, and the
//! bare `max` fold falls out as dead code — the result is exactly
//! `softmax − onehot`. A second phase then adds the log-sum-exp regrouping
//! `(g/s)·exp(z−m) → g·exp(z − lse)`; run over the forward AND backward
//! together ([`simplify_many`], one shared CSE table), it makes the backward's
//! softmax the SAME node as the forward's materialized logsumexp, so the
//! schedule reuses that carrier instead of recomputing the `(max, Σexp)` fold.
//! The composed cross-entropy's forward+backward then derives to the same
//! kernel count as the hand-written `LogSumExp` monoid — no gradient rule for
//! it anywhere.
//!
//! This is a *client-side* pass, not part of [`crate::partition`]: it earns its
//! keep on gradients (and other naive transposes), and the rewrites that help
//! there — factoring a scale out of a reduction, regrouping a division into an
//! exponent — can perturb the carefully-tuned fusion of an inference graph, so
//! a training loop applies it to its step and inference never pays for it.
//!
//! Every rewrite is a value-preserving identity (off measure-zero
//! singularities, where the gradient is undefined anyway), so the
//! finite-difference tests in `tests/grad.rs` and `tests/simplify.rs` remain
//! the correctness check.

use std::collections::HashMap;
use std::rc::Rc;

use crate::ir::{
    MapOp, Monoid, Node as NodeKind, NodeRef as Node, coordinate, gather, konst, map,
    positional_reindex, positional_view, reduce, scan,
};

/// Simplify several roots TOGETHER, in two phases, each to a fixpoint. One
/// CSE table is shared per pass, so a subtree computed in one root and
/// recomputed in another (a forward value and its reappearance in the
/// backward) becomes a single node.
///
/// Phase 1 is cancellation only — the ring identities, the defer-scale factor
/// and CSE — which collapses a stabilizing max-shift's winner-mask cotangent to
/// zero. Phase 2 additionally enables the log-sum-exp *reconstruction*
/// (`(g/s)·exp(x−m) → g·exp(x−lse)`), which lets a softmax gradient reuse the
/// forward's materialized logsumexp instead of recomputing the `(max, Σexp)`
/// fold. The order matters: the softmax subterm is CSE-shared with the winner
/// mask's cotangent path, so reconstructing it *before* that path has cancelled
/// would rewrite a live reduction into a spurious `Σ softmax`. Cancelling
/// first leaves only the true softmax for phase 2 to reshape.
pub fn simplify_many(roots: &[Node]) -> Vec<Node> {
    let mut cur: Vec<Node> = roots.to_vec();
    for reconstruct in [false, true] {
        for _ in 0..32 {
            let mut cse: HashMap<String, Node> = HashMap::new();
            let mut memo: HashMap<*const NodeKind, Node> = HashMap::new();
            let mut changed = false;
            cur = cur
                .iter()
                .map(|r| pass(r, reconstruct, &mut cse, &mut memo, &mut changed))
                .collect();
            if !changed {
                break;
            }
        }
    }
    cur
}

/// One bottom-up pass: canonicalize children, apply a local rewrite, hash-cons.
fn pass(
    node: &Node,
    reconstruct: bool,
    cse: &mut HashMap<String, Node>,
    memo: &mut HashMap<*const NodeKind, Node>,
    changed: &mut bool,
) -> Node {
    let ptr = Rc::as_ptr(node);
    if let Some(n) = memo.get(&ptr) {
        return n.clone();
    }
    // Rebuild a node only when a child changed or a rule fires; otherwise keep
    // the ORIGINAL `Rc`. This preserves the identity `grad` shares with the
    // forward graph (the gradient reads forward nodes by pointer), so a
    // backward that recomputes the forward's `s` stays the *same* node and the
    // schedule materializes it once — and it keeps CSE keys (built from child
    // pointers) stable, which is what unifies the gradient's `Σ exp(x − m)`
    // with that forward `s` in the first place.
    let out = match node.as_ref() {
        NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => {
            hashcons(node.clone(), cse)
        }
        NodeKind::Coordinate { src, dim } => {
            let source = pass(src, reconstruct, cse, memo, changed);
            hashcons(
                if Rc::ptr_eq(&source, src) {
                    node.clone()
                } else {
                    coordinate(source, *dim)
                },
                cse,
            )
        }
        NodeKind::Map { op, inputs } => {
            let output_shape = node.shape();
            let output_axes = crate::ir::axis_refs(node);
            let ins: Vec<Node> = inputs
                .iter()
                .map(|i| pass(i, reconstruct, cse, memo, changed))
                .collect();
            let rebuilt = match map_rule(*op, &ins, reconstruct) {
                Some(r) => {
                    *changed = true;
                    preserve_shape(r, &output_shape, &output_axes)
                }
                None if same(&ins, inputs) => node.clone(),
                None => map(*op, ins),
            };
            hashcons(rebuilt, cse)
        }
        NodeKind::Reduce { src, dim, op } => {
            let output_shape = node.shape();
            let output_axes = crate::ir::axis_refs(node);
            let s = pass(src, reconstruct, cse, memo, changed);
            let rebuilt = match reduce_rule(&s, *dim, *op) {
                Some(r) => {
                    *changed = true;
                    preserve_shape(r, &output_shape, &output_axes)
                }
                None if Rc::ptr_eq(&s, src) => node.clone(),
                None => reduce(s, *dim, *op),
            };
            hashcons(rebuilt, cse)
        }
        NodeKind::Scan { src, dim, op } => {
            let s = pass(src, reconstruct, cse, memo, changed);
            let rebuilt = if Rc::ptr_eq(&s, src) {
                node.clone()
            } else {
                scan(s, *dim, *op)
            };
            hashcons(rebuilt, cse)
        }
        NodeKind::Gather { src, index, dim } => {
            let s = pass(src, reconstruct, cse, memo, changed);
            let i = pass(index, reconstruct, cse, memo, changed);
            let rebuilt = if Rc::ptr_eq(&s, src) && Rc::ptr_eq(&i, index) {
                node.clone()
            } else {
                gather(s, i, *dim)
            };
            hashcons(rebuilt, cse)
        }
        NodeKind::View { src, dims } => {
            let s = pass(src, reconstruct, cse, memo, changed);
            let identity = dims.len() == s.shape().len()
                && dims.iter().enumerate().all(|(output_dim, dim)| {
                    dim.sources == [output_dim] && dim.axis == s.shape()[output_dim]
                });
            let rebuilt = if identity {
                *changed = true;
                s
            } else if Rc::ptr_eq(&s, src) {
                node.clone()
            } else {
                positional_view(s, dims.clone())
            };
            hashcons(rebuilt, cse)
        }
        NodeKind::Reindex {
            src,
            shape,
            map: m,
            padded,
        } => {
            let s = pass(src, reconstruct, cse, memo, changed);
            let rebuilt = if let Some(unwrapped) = cancel_view_reindex(&s, shape, m, *padded) {
                *changed = true;
                unwrapped
            } else if Rc::ptr_eq(&s, src) {
                node.clone()
            } else {
                positional_reindex(s, shape.clone(), m.clone(), *padded)
            };
            hashcons(rebuilt, cse)
        }
    };
    memo.insert(ptr, out.clone());
    out
}

/// Return the canonical node for this shallow structure — two structurally
/// identical nodes (identical op and identical, already-canonical children)
/// collapse to one, which is what turns structural equality into a pointer
/// test for the rewrites below. The key is [`crate::ir::shallow_key`], shared
/// with the pipeline-entry [`crate::ir::canonicalize_many`].
fn hashcons(node: Node, cse: &mut HashMap<String, Node>) -> Node {
    cse.entry(crate::ir::shallow_key(&node))
        .or_insert(node)
        .clone()
}

// ── local rewrites (children already canonical, so `≡` is `Rc::ptr_eq`) ───────

/// Are two operand lists pointer-identical (nothing rebuilt beneath)?
fn same(a: &[Node], b: &[Node]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| Rc::ptr_eq(x, y))
}

/// Cancel a reindex that merely removes singleton dimensions inserted by a
/// storage view (and/or undoes a permutation of one-to-one view dimensions).
/// This is the positional `squeeze(unsqueeze(x)) = x` identity. Keeping it
/// structural is important: autodiff routinely creates the pair while
/// restoring exact cotangent ranks, and an opaque pair prevents ordinary CSE
/// and ring cancellation above it.
fn cancel_view_reindex(
    node: &Node,
    output_shape: &[crate::ir::Axis],
    reindex: &[crate::ir::AffineIndex],
    padded: bool,
) -> Option<Node> {
    if padded {
        return None;
    }
    let NodeKind::View { src, dims } = node.as_ref() else {
        return None;
    };
    if reindex.len() != dims.len() || dims.iter().any(|dim| dim.sources.len() > 1) {
        return None;
    }

    let mut output_sources = vec![None; output_shape.len()];
    for (view_dim, terms, offset) in reindex {
        if *offset != 0 {
            return None;
        }
        match dims[*view_dim].sources.as_slice() {
            [] if terms.is_empty() => {}
            [source_dim] => {
                let [(coefficient, output_dim)] = terms.as_slice() else {
                    return None;
                };
                if *coefficient != 1
                    || output_sources.get(*output_dim)?.is_some()
                    || src.shape()[*source_dim].extent != output_shape[*output_dim].extent
                {
                    return None;
                }
                output_sources[*output_dim] = Some(*source_dim);
            }
            _ => return None,
        }
    }
    let output_sources = output_sources.into_iter().collect::<Option<Vec<_>>>()?;
    let view_dims = output_sources
        .iter()
        .zip(output_shape)
        .map(|(&source_dim, &axis)| crate::ir::ViewDim {
            sources: vec![source_dim],
            axis,
        })
        .collect::<Vec<_>>();
    let identity = view_dims.iter().enumerate().all(|(output_dim, dim)| {
        dim.sources == [output_dim] && dim.axis == src.shape()[output_dim]
    });
    Some(if identity {
        src.clone()
    } else {
        positional_view(src.clone(), view_dims)
    })
}

fn preserve_shape(
    mut node: Node,
    target: &[crate::ir::Axis],
    target_axes: &[crate::ir::AxisRef],
) -> Node {
    let mut source = node.shape();
    if source == target {
        return node;
    }
    while source.len() > target.len() {
        let source_axes = crate::ir::axis_refs(&node);
        let dim = source
            .iter()
            .enumerate()
            .position(|(dim, axis)| {
                axis.extent == crate::ir::Extent::Static(1)
                    && !target_axes.contains(&source_axes[dim])
            })
            .or_else(|| {
                source
                    .iter()
                    .position(|axis| axis.extent == crate::ir::Extent::Static(1))
            })
            .expect("shape-changing simplification left a non-singleton dimension");
        node = reduce(node, dim, Monoid::Add);
        source.remove(dim);
    }
    assert!(
        source.len() <= target.len(),
        "simplification cannot increase a replacement's rank"
    );
    let source_axes = crate::ir::axis_refs(&node);
    if source.len() == target.len() {
        let permutation = target_axes
            .iter()
            .map(|target_axis| source_axes.iter().position(|axis| axis == target_axis))
            .collect::<Option<Vec<_>>>();
        if let Some(permutation) = permutation {
            return positional_view(
                node,
                permutation
                    .into_iter()
                    .zip(target.iter().copied())
                    .map(|(source_dim, axis)| crate::ir::ViewDim {
                        sources: vec![source_dim],
                        axis,
                    })
                    .collect(),
            );
        }
    }
    let mut used = vec![false; target.len()];
    let map = source
        .iter()
        .enumerate()
        .map(|(source_dim, source_axis)| {
            let exact = target_axes
                .iter()
                .enumerate()
                .find(|(dim, axis)| !used[*dim] && **axis == source_axes[source_dim])
                .map(|(dim, _)| dim);
            let trailing = target.len() - source.len() + source_dim;
            let positional = (!used[trailing] && target[trailing].extent == source_axis.extent)
                .then_some(trailing);
            let same_extent = target
                .iter()
                .enumerate()
                .find(|(dim, axis)| !used[*dim] && axis.extent == source_axis.extent)
                .map(|(dim, _)| dim);
            let terms = if source_axis.extent == crate::ir::Extent::Static(1) {
                Vec::new()
            } else {
                let target_dim = exact
                    .or(positional)
                    .or(same_extent)
                    .expect("shape-preserving rewrite lost a non-singleton dimension");
                used[target_dim] = true;
                vec![(1, target_dim)]
            };
            (source_dim, terms, 0)
        })
        .collect();
    positional_reindex(node, target.to_vec(), map, false)
}

fn cst(n: &Node) -> Option<f64> {
    match n.as_ref() {
        NodeKind::Const { v } => Some(*v),
        NodeKind::View { src, .. } => cst(src),
        NodeKind::Reindex { src, padded, .. } => cst(src).filter(|value| !padded || *value == 0.0),
        _ => None,
    }
}

fn as_map(n: &Node, want: MapOp) -> Option<&[Node]> {
    match n.as_ref() {
        NodeKind::Map { op, inputs } if *op == want => Some(inputs),
        _ => None,
    }
}

/// Pull a pointwise map through a common storage view. This gives alignment
/// one canonical location, so independently constructed `unsqueeze` shells
/// do not hide otherwise identical elementwise expressions from CSE.
fn factor_common_view(op: MapOp, inputs: &[Node]) -> Option<Node> {
    let (first_src, first_dims) = match inputs.first()?.as_ref() {
        NodeKind::View { src, dims } => (src, dims),
        _ => return None,
    };
    let mut sources = vec![first_src.clone()];
    for input in &inputs[1..] {
        let NodeKind::View { src, dims } = input.as_ref() else {
            return None;
        };
        let same_dims = dims.len() == first_dims.len()
            && dims
                .iter()
                .zip(first_dims)
                .all(|(left, right)| left.sources == right.sources && left.axis == right.axis);
        if !same_dims {
            return None;
        }
        sources.push(src.clone());
    }
    let inner = map(op, sources);
    // The view's source positions describe the common inner output too.
    (inner.shape().len() == first_src.shape().len())
        .then(|| positional_view(inner, first_dims.clone()))
}

/// Recognize a division either directly or under one shared positional
/// alignment transform, distributing that transform to the numerator and
/// denominator. This is algebra over generic pointwise structure, not a
/// logsumexp pattern.
fn division_parts(node: &Node) -> Option<(Node, Node)> {
    if let Some(inputs) = as_map(node, MapOp::Div) {
        return Some((inputs[0].clone(), inputs[1].clone()));
    }
    match node.as_ref() {
        NodeKind::View { src, dims } => {
            let inputs = as_map(src, MapOp::Div)?;
            (inputs.iter().all(|input| input.shape() == src.shape())).then(|| {
                (
                    positional_view(inputs[0].clone(), dims.clone()),
                    positional_view(inputs[1].clone(), dims.clone()),
                )
            })
        }
        NodeKind::Reindex {
            src,
            shape,
            map: reindex,
            padded: false,
        } => {
            let inputs = as_map(src, MapOp::Div)?;
            (inputs.iter().all(|input| input.shape() == src.shape())).then(|| {
                (
                    positional_reindex(inputs[0].clone(), shape.clone(), reindex.clone(), false),
                    positional_reindex(inputs[1].clone(), shape.clone(), reindex.clone(), false),
                )
            })
        }
        _ => None,
    }
}

/// One rewrite of an elementwise op over canonical operands, or `None`. With
/// `reconstruct`, the log-sum-exp regrouping rewrites are also enabled (phase
/// 2 — see [`simplify`]).
fn map_rule(op: MapOp, ins: &[Node], reconstruct: bool) -> Option<Node> {
    let eq = |a: &Node, b: &Node| Rc::ptr_eq(a, b);
    if let Some(factored) = factor_common_view(op, ins) {
        return Some(factored);
    }
    match op {
        MapOp::Add => {
            let (a, b) = (&ins[0], &ins[1]);
            if cst(a) == Some(0.0) {
                return Some(b.clone());
            }
            if cst(b) == Some(0.0) {
                return Some(a.clone());
            }
            // x + (−x) → 0, either order
            if let Some(nb) = as_map(b, MapOp::Neg) {
                if eq(a, &nb[0]) {
                    return Some(konst(0.0));
                }
            }
            if let Some(na) = as_map(a, MapOp::Neg) {
                if eq(&na[0], b) {
                    return Some(konst(0.0));
                }
            }
            match (cst(a), cst(b)) {
                (Some(x), Some(y)) => Some(konst(x + y)),
                _ => None,
            }
        }
        MapOp::Sub => {
            let (a, b) = (&ins[0], &ins[1]);
            if eq(a, b) {
                return Some(konst(0.0));
            }
            if cst(b) == Some(0.0) {
                return Some(a.clone());
            }
            if cst(a) == Some(0.0) {
                return Some(map(MapOp::Neg, vec![b.clone()]));
            }
            // (x − y) − z → x − (y + z): regroups the subtracted terms, so a
            // shift `(z − m) − log s` collapses to `z − (m + log s)` and the
            // inner `m + log s` shares the forward logsumexp by CSE.
            if reconstruct {
                if let Some(inner) = as_map(a, MapOp::Sub) {
                    return Some(map(
                        MapOp::Sub,
                        vec![
                            inner[0].clone(),
                            map(MapOp::Add, vec![inner[1].clone(), b.clone()]),
                        ],
                    ));
                }
            }
            match (cst(a), cst(b)) {
                (Some(x), Some(y)) => Some(konst(x - y)),
                _ => None,
            }
        }
        MapOp::Mul => {
            let (a, b) = (&ins[0], &ins[1]);
            if cst(a) == Some(0.0) || cst(b) == Some(0.0) {
                return Some(konst(0.0));
            }
            if cst(a) == Some(1.0) {
                return Some(b.clone());
            }
            if cst(b) == Some(1.0) {
                return Some(a.clone());
            }
            // (p / q) · q → p, either order
            if let Some((numerator, denominator)) = division_parts(a) {
                if eq(&denominator, b) {
                    return Some(numerator);
                }
            }
            if let Some((numerator, denominator)) = division_parts(b) {
                if eq(&denominator, a) {
                    return Some(numerator);
                }
            }
            // (p / q) · exp(c) → p · exp(c − log q): fold the division into the
            // exponent (log-sum-exp). A softmax gradient `(g/s)·exp(z − m)`
            // becomes `g·exp(z − (m + log s))` = `g·exp(z − lse)`, which reuses
            // the forward logsumexp instead of recomputing the (max, Σexp) fold.
            if reconstruct {
                let fold_into_exp = |numerator: Node, denominator: Node, e: &[Node]| {
                    map(
                        MapOp::Mul,
                        vec![
                            numerator,
                            map(
                                MapOp::Exp,
                                vec![map(
                                    MapOp::Sub,
                                    vec![e[0].clone(), map(MapOp::Log, vec![denominator])],
                                )],
                            ),
                        ],
                    )
                };
                if let (Some((numerator, denominator)), Some(e)) =
                    (division_parts(a), as_map(b, MapOp::Exp))
                {
                    return Some(fold_into_exp(numerator, denominator, e));
                }
                if let (Some(e), Some((numerator, denominator))) =
                    (as_map(a, MapOp::Exp), division_parts(b))
                {
                    return Some(fold_into_exp(numerator, denominator, e));
                }
            }
            match (cst(a), cst(b)) {
                (Some(x), Some(y)) => Some(konst(x * y)),
                _ => None,
            }
        }
        MapOp::Div => {
            let (a, b) = (&ins[0], &ins[1]);
            if cst(a) == Some(0.0) {
                return Some(konst(0.0));
            }
            if cst(b) == Some(1.0) {
                return Some(a.clone());
            }
            None
        }
        MapOp::Neg => {
            let a = &ins[0];
            if let Some(x) = cst(a) {
                return Some(konst(-x));
            }
            // −(−x) → x
            as_map(a, MapOp::Neg).map(|n| n[0].clone())
        }
        _ => None,
    }
}

/// One rewrite of a reduction over a canonical source, or `None`. Only the
/// `Add`-reduce distributes (it is the additive semiring); the invariant it
/// pulls out is data that does not carry the reduced axis.
fn reduce_rule(s: &Node, dim: usize, op: Monoid) -> Option<Node> {
    if op != Monoid::Add {
        return None;
    }
    if cst(s) == Some(0.0) {
        return Some(konst(0.0)); // Σ 0 = 0 for any extent
    }
    // Σ(k · x) → k · Σx when k is invariant along the axis (defer-scale).
    // The whole binary product TREE is searched, not just the two direct
    // operands: hoisting is closed under association and operand order, so a
    // scale buried as `q·(k·c)` still leaves and the variant factors rebuild
    // as a clean contraction — the canonical form `derive`'s syntactic
    // matcher contracts for (see `other_axis_folds`).
    if as_map(s, MapOp::Mul).is_some() {
        let (mut inv, mut var) = (Vec::new(), Vec::new());
        split_product(s, Some(dim), &mut inv, &mut var);
        if !inv.is_empty() && !var.is_empty() {
            let product = |factors: Vec<Node>| {
                factors
                    .into_iter()
                    .reduce(|a, b| map(MapOp::Mul, vec![a, b]))
                    .unwrap()
            };
            let variant = product(var);
            let variant_dim = variant.shape().len() - (s.shape().len() - dim);
            return Some(map(
                MapOp::Mul,
                vec![product(inv), reduce(variant, variant_dim, op)],
            ));
        }
    }
    // Σ(−x) → −Σx
    if let Some(n) = as_map(s, MapOp::Neg) {
        return Some(map(MapOp::Neg, vec![reduce(n[0].clone(), dim, op)]));
    }
    None
}

/// Split a binary `Mul` tree into its atomic factors, sorted by whether they
/// carry `axis` (variant) or not (invariant). In-order traversal, so the
/// rebuilt products keep the source's operand order.
fn split_product(n: &Node, dim: Option<usize>, inv: &mut Vec<Node>, var: &mut Vec<Node>) {
    // Alignment views commute with an elementwise product. Looking through
    // them keeps association/operand order irrelevant to factor hoisting
    // without teaching the scheduler about frontend alignment artifacts.
    if let NodeKind::View { src, dims } = n.as_ref()
        && let Some(inputs) = as_map(src, MapOp::Mul)
        && let Some(output_dim) = dim
        && let [source_dim] = dims[output_dim].sources.as_slice()
    {
        let source_shape = src.shape();
        for input in inputs {
            let input_shape = input.shape();
            let lead = source_shape.len() - input_shape.len();
            let input_dim = source_dim.checked_sub(lead);
            let varies = input_dim.is_some_and(|dim| depends_on_dimension(input, dim));
            if !varies {
                push_invariant(input.clone(), input_dim, inv);
                continue;
            }

            let aligned = if input_shape == source_shape {
                input.clone()
            } else {
                let map = input_shape
                    .iter()
                    .enumerate()
                    .map(|(input_dim, axis)| {
                        let output_dim = lead + input_dim;
                        let terms = if axis.extent == crate::ir::Extent::Static(1)
                            && source_shape[output_dim].extent != crate::ir::Extent::Static(1)
                        {
                            Vec::new()
                        } else {
                            vec![(1, output_dim)]
                        };
                        (input_dim, terms, 0)
                    })
                    .collect();
                positional_reindex(input.clone(), source_shape.clone(), map, false)
            };
            let viewed = positional_view(aligned, dims.clone());
            split_product(&viewed, dim, inv, var);
        }
        return;
    }

    if let Some(m) = as_map(n, MapOp::Mul) {
        for input in m {
            let input_dim = dim.and_then(|output_dim| {
                output_dim.checked_sub(n.shape().len().saturating_sub(input.shape().len()))
            });
            if input_dim.is_some_and(|dim| depends_on_dimension(input, dim)) {
                split_product(input, input_dim, inv, var);
            } else {
                push_invariant(input.clone(), input_dim, inv);
            }
        }
    } else if dim.is_some() {
        var.push(n.clone());
    } else {
        // A scalar expanded through singleton-only structural views is still
        // the scalar. Keeping the shell here would give the hoisted factor a
        // fake rank and force a separate broadcast kernel.
        let mut factor = n.clone();
        loop {
            let source = match factor.as_ref() {
                NodeKind::View { src, .. } | NodeKind::Reindex { src, .. }
                    if src.shape().is_empty()
                        && factor
                            .shape()
                            .iter()
                            .all(|axis| axis.extent == crate::ir::Extent::Static(1)) =>
                {
                    src.clone()
                }
                _ => break,
            };
            factor = source;
        }
        inv.push(factor);
    }
}

/// Whether changing one output coordinate can change this node's value.
/// This follows positional dataflow, including affine reindexes whose output
/// shape may contain dimensions that no source coordinate actually uses.
fn depends_on_dimension(node: &Node, dim: usize) -> bool {
    let shape = node.shape();
    if shape[dim].extent == crate::ir::Extent::Static(1) {
        return false;
    }
    match node.as_ref() {
        NodeKind::Input { .. } | NodeKind::Iota { .. } => true,
        NodeKind::Const { .. } => false,
        NodeKind::Coordinate {
            dim: coordinate_dim,
            ..
        } => dim == *coordinate_dim,
        NodeKind::Map { inputs, .. } => inputs.iter().any(|input| {
            dim.checked_sub(shape.len() - input.shape().len())
                .is_some_and(|input_dim| depends_on_dimension(input, input_dim))
        }),
        NodeKind::Reduce {
            src,
            dim: reduced_dim,
            ..
        } => {
            let source_dim = dim + usize::from(dim >= *reduced_dim);
            depends_on_dimension(src, source_dim)
        }
        NodeKind::Scan {
            src, dim: scan_dim, ..
        } => dim == *scan_dim || depends_on_dimension(src, dim),
        NodeKind::Gather {
            src,
            index,
            dim: gather_dim,
        } => {
            let index_rank = index.shape().len();
            if dim < *gather_dim {
                depends_on_dimension(src, dim)
            } else if dim < *gather_dim + index_rank {
                true
            } else {
                depends_on_dimension(src, dim - index_rank + 1)
            }
        }
        NodeKind::View { src, dims } => dims[dim]
            .sources
            .iter()
            .any(|&source_dim| depends_on_dimension(src, source_dim)),
        NodeKind::Reindex {
            src, map, padded, ..
        } => map.iter().any(|(source_dim, terms, _)| {
            let coordinate_depends = terms
                .iter()
                .any(|(coefficient, output_dim)| *coefficient != 0 && *output_dim == dim);
            coordinate_depends && (*padded || depends_on_dimension(src, *source_dim))
        }),
    }
}

fn push_invariant(mut node: Node, dim: Option<usize>, inv: &mut Vec<Node>) {
    if let Some(dim) = dim {
        node = drop_dimension(node, dim);
    }
    // A scalar expanded through singleton-only structural views is still the
    // scalar. Keeping the shell here would give the hoisted factor a fake
    // rank and force a separate broadcast kernel.
    loop {
        let source = match node.as_ref() {
            NodeKind::View { src, .. } | NodeKind::Reindex { src, .. }
                if src.shape().is_empty()
                    && node
                        .shape()
                        .iter()
                        .all(|axis| axis.extent == crate::ir::Extent::Static(1)) =>
            {
                src.clone()
            }
            _ => break,
        };
        node = source;
    }
    inv.push(node);
}

/// Select coordinate zero along a dimension already proven invariant. When
/// the dimension is unused by an affine reindex (the common broadcast case),
/// remove it directly so a following view/reindex identity can cancel.
fn drop_dimension(node: Node, dim: usize) -> Node {
    if let NodeKind::View { src, dims } = node.as_ref()
        && dims[dim].sources.is_empty()
    {
        let mut output = dims.clone();
        output.remove(dim);
        return positional_view(src.clone(), output);
    }
    if let NodeKind::Reindex {
        src,
        shape,
        map,
        padded,
    } = node.as_ref()
        && map
            .iter()
            .all(|(_, terms, _)| terms.iter().all(|(_, output_dim)| *output_dim != dim))
    {
        let mut output = shape.clone();
        output.remove(dim);
        let map = map
            .iter()
            .map(|(source_dim, terms, offset)| {
                (
                    *source_dim,
                    terms
                        .iter()
                        .map(|(coefficient, output_dim)| {
                            (*coefficient, output_dim - usize::from(*output_dim > dim))
                        })
                        .collect(),
                    *offset,
                )
            })
            .collect();
        return positional_reindex(src.clone(), output, map, *padded);
    }

    let source = node.shape();
    let mut output = source.clone();
    output.remove(dim);
    let map = source
        .iter()
        .enumerate()
        .map(|(source_dim, _)| {
            let terms = if source_dim == dim {
                Vec::new()
            } else {
                vec![(1, source_dim - usize::from(source_dim > dim))]
            };
            (source_dim, terms, 0)
        })
        .collect();
    positional_reindex(node, output, map, false)
}
