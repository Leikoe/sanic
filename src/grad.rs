//! Reverse-mode autodiff over the closed basis — backward graphs in the
//! same IR.
//!
//! This is the payoff of keeping the operator set small: one differentiation
//! rule per basis op and per structural operator, and the backward pass of
//! *anything* — attention, RMSNorm, convolution, an embedding lookup — falls
//! out as an ordinary dataflow graph. No tape, no interpreter hooks: [`grad`]
//! maps a scalar loss to gradient *graphs*, and those graphs then go through
//! `derive` / `partition` / `Schedule::execute` / the emitters exactly like
//! any forward computation. `tests/grad.rs` holds every rule to central
//! finite differences, and runs gradient schedules through the same pipeline
//! the forward passes use.
//!
//! The structural transposes are the interesting part, and each is the
//! movement vocabulary pointing back at itself:
//!
//! * broadcast (a `Map` over a smaller operand) ⟵ reduce the extra axes
//! * `Reduce(Add)` ⟵ broadcast (implicit in an axis-set IR)
//! * `Reduce(Max/Min)` ⟵ a computed winner mask (subgradient; ties split
//!   the mass evenly, conserving the cotangent)
//! * `Reduce(LogSumExp)` ⟵ `exp(src − result)` — the softmax Jacobian
//! * `Gather` ⟵ [`scatter_add`] — the adjoint of an indexed read is an
//!   add-combined indexed write
//! * `View` rename ⟵ rename back; flatten ⟵ [`split`]
//! * `Reindex` slice/pad ⟵ each other; [`split`] ⟵ flatten; a stride-1
//!   `window` ⟵ the mirrored window of the cotangent (overlap-add)
//!
//! Declines, stated rather than guessed at: `Scan` recurrences,
//! `Reduce(Mul)` (undefined at zeros), data-dependent index gradients, and
//! reindex patterns whose transpose is not affine (strided *and* dilated
//! windows). Each panics with a message naming the gap.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::ir::{
    self, AffineIndex, Axis, Extent, MapOp, Monoid, Node as NodeKind, NodeRef as Node, ViewDim, konst, map,
    positional_reindex, positional_view, reduce, scan, split,
};

/// d(loss)/d(each named input), as graphs. `loss` must be a scalar (no free
/// axes). Inputs that the loss does not depend on are absent from the result.
///
/// A name-keyed convenience over [`grad_nodes`]: distinct `Input` nodes with
/// the same name are the same buffer read under (possibly) different axis
/// labels, so their adjoints sum after relabeling to the first-seen
/// declaration.
pub fn grad(loss: &Node, wrt: &[&'static str]) -> HashMap<&'static str, Node> {
    let mut order: Vec<Node> = Vec::new();
    let mut seen: Vec<*const NodeKind> = Vec::new();
    postorder(loss, &mut seen, &mut order);
    let targets: Vec<Node> = order
        .iter()
        .filter(|node| matches!(node.as_ref(), NodeKind::Input { name, .. } if wrt.contains(name)))
        .cloned()
        .collect();

    let mut by_name: HashMap<&'static str, (Vec<Axis>, Node)> = HashMap::new();
    for (target, gradient) in targets.iter().zip(grad_nodes(loss, &targets, &[])) {
        let Some(gradient) = gradient else { continue };
        let NodeKind::Input { name, shape, .. } = target.as_ref() else {
            unreachable!("targets are input nodes")
        };
        let contrib = reduce_to(gradient, shape);
        match by_name.get_mut(name) {
            None => {
                by_name.insert(name, (shape.clone(), contrib));
            }
            Some((canon, acc)) => {
                assert_eq!(
                    canon.len(),
                    shape.len(),
                    "grad: input `{name}` declared with different ranks"
                );
                *acc = map(MapOp::Add, vec![acc.clone(), contrib]);
            }
        }
    }
    by_name.into_iter().map(|(n, (_, g))| (n, g)).collect()
}

/// d(loss)/d(each target NODE), one entry per target — `None` when the loss
/// does not depend on it. Targets may be any node, interior or leaf.
///
/// `stop` nodes are gradient BOUNDARIES: the gradient flows *to* a stopped
/// node (it can itself be a target) but never through it to its inputs.
/// Stop-gradient is a property of the QUESTION, not of the graph — an IR
/// marker would make two structurally identical subtrees, one detached,
/// refuse to intern to one node.
pub fn grad_nodes(loss: &Node, targets: &[Node], stop: &[Node]) -> Vec<Option<Node>> {
    crate::verify::assert_valid(loss);
    assert!(
        loss.shape().is_empty(),
        "grad: the loss must be a scalar (reduce it first); got free axes {:?}",
        loss.shape()
    );

    // reverse topological order (postorder of the DAG, reversed)
    let mut order: Vec<Node> = Vec::new();
    let mut seen: Vec<*const NodeKind> = Vec::new();
    postorder(loss, &mut seen, &mut order);

    // Only nodes from which a target is reachable need adjoints. Pruning is
    // lossless: any consumer of an on-path node reaches the same target, so
    // it is on-path itself and no contribution is dropped.
    let target_ptrs: HashSet<*const NodeKind> = targets.iter().map(Arc::as_ptr).collect();
    let stop_ptrs: HashSet<*const NodeKind> = stop.iter().map(Arc::as_ptr).collect();
    let mut on_path: HashSet<*const NodeKind> = HashSet::new();
    for node in &order {
        // postorder: children are decided before their consumers
        let reaches = target_ptrs.contains(&Arc::as_ptr(node))
            || ir::children(node).iter().any(|c| on_path.contains(&Arc::as_ptr(c)));
        if reaches {
            on_path.insert(Arc::as_ptr(node));
        }
    }

    // cotangent per node, accumulated across consumers
    let mut adj: HashMap<*const NodeKind, Node> = HashMap::new();
    adj.insert(Arc::as_ptr(loss), konst(1.0));

    for node in order.iter().rev() {
        if !on_path.contains(&Arc::as_ptr(node)) {
            continue; // no target below this node
        }
        let Some(g) = adj.get(&Arc::as_ptr(node)).cloned() else {
            continue; // the loss does not depend on this node
        };
        if stop_ptrs.contains(&Arc::as_ptr(node)) {
            continue; // a gradient boundary: keep the adjoint, propagate nothing
        }
        // A map can consume a broadcastable cotangent directly. Keeping the
        // singleton form produced by a reduction avoids manufacturing an
        // affine expansion that hides ordinary algebra from simplification.
        // Structural rules need exact coordinates, so all other nodes still
        // receive an exact-shape cotangent.
        let g = if matches!(node.as_ref(), NodeKind::Map { .. }) {
            g
        } else {
            broadcast_to(g, &node.shape())
        };
        match node.as_ref() {
            NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } | NodeKind::Coordinate { .. } => {}

            NodeKind::Map { op, inputs } => {
                for (contrib, child) in map_backward(*op, inputs, &g) {
                    let target = child.shape();
                    add_adj(&mut adj, child, reduce_to(contrib, &target));
                }
            }

            NodeKind::Reduce { src, dim, op } => {
                let expanded_g = ir::unsqueeze(g.clone(), *dim);
                let expanded_result = ir::unsqueeze(node.clone(), *dim);
                let contrib = match op {
                    Monoid::Add => expanded_g,
                    Monoid::Max | Monoid::Min => {
                        // winner mask, with the mass split evenly across
                        // ties: the subgradient must conserve g, and the
                        // winner count is ≥ 1 wherever the result is
                        // attained, so the division is total.
                        let hit = winner_mask(src, &expanded_result);
                        let count = ir::unsqueeze(reduce(hit.clone(), *dim, Monoid::Add), *dim);
                        let share = map(MapOp::Div, vec![hit, count]);
                        map(MapOp::Mul, vec![expanded_g, share])
                    }
                    Monoid::LogSumExp => {
                        // ∂LSE/∂src = exp(src − LSE) — the softmax Jacobian row
                        let sm = map(MapOp::Exp, vec![map(MapOp::Sub, vec![src.clone(), expanded_result])]);
                        map(MapOp::Mul, vec![expanded_g, sm])
                    }
                    Monoid::Mul => panic!(
                        "grad: Reduce(Mul) has no gradient here — result/src is \
                         undefined at zeros; rewrite the product or extend the rule"
                    ),
                };
                add_adj(&mut adj, src, contrib);
            }

            NodeKind::Gather { src, index, dim } => {
                let scattered = scatter_add(g.clone(), index.clone(), *dim, &src.shape());
                add_adj(&mut adj, src, scattered);
                // integer indices carry no gradient
            }

            NodeKind::View { src, dims } => {
                add_adj(&mut adj, src, invert_view(g, &src.shape(), dims));
            }

            NodeKind::Reindex { src, map: rmap, .. } => {
                add_adj(&mut adj, src, transpose_reindex(g, &src.shape(), &node.shape(), rmap));
            }

            // A prefix SUM's adjoint is the reversed prefix sum of the
            // cotangent: y_j = Σ_{i≤j} x_i ⇒ ∂L/∂x_i = Σ_{j≥i} g_j —
            // reverse, cumsum, reverse, all existing IR (reversal is a
            // self-referential Reindex). Other monoid scans need per-prefix
            // contribution masks and stay declined, stated in todo.md.
            NodeKind::Scan {
                src,
                dim,
                op: Monoid::Add,
            } => {
                let reverse = |x: Node| {
                    let shape = x.shape();
                    let flip = (shape[*dim].extent() - 1) as i64;
                    let reindex = shape
                        .iter()
                        .enumerate()
                        .map(|(source_dim, _)| {
                            if source_dim == *dim {
                                (source_dim, vec![(-1, source_dim)], flip)
                            } else {
                                (source_dim, vec![(1, source_dim)], 0)
                            }
                        })
                        .collect();
                    positional_reindex(x, shape, reindex, false)
                };
                let contrib = reverse(scan(reverse(g.clone()), *dim, Monoid::Add));
                add_adj(&mut adj, src, contrib);
            }
            NodeKind::Scan { op, .. } => panic!(
                "grad: {op:?} scan backward is not implemented (Add-scan has \
                 the reversed-cumsum rule; other monoids need per-prefix \
                 contribution masks — see todo.md)"
            ),
        }
    }

    targets
        .iter()
        .map(|target| {
            adj.get(&Arc::as_ptr(target))
                .map(|g| broadcast_to(g.clone(), &target.shape()))
        })
        .collect()
}

fn equal(left: Node, right: Node) -> Node {
    let left_lt_right = map(MapOp::Lt, vec![left.clone(), right.clone()]);
    let right_lt_left = map(MapOp::Lt, vec![right, left]);
    map(
        MapOp::Mul,
        vec![
            map(MapOp::Sub, vec![konst(1.0), left_lt_right]),
            map(MapOp::Sub, vec![konst(1.0), right_lt_left]),
        ],
    )
}

fn lift_to_shape(src: Node, shape: Vec<Axis>, output_dims: &[usize]) -> Node {
    assert_eq!(src.shape().len(), output_dims.len());
    let map = output_dims
        .iter()
        .enumerate()
        .map(|(source_dim, &output_dim)| (source_dim, vec![(1, output_dim)], 0))
        .collect();
    positional_reindex(src, shape, map, false)
}

/// The gather adjoint as a generic one-hot contraction. The temporary
/// iteration space is `[source-prefix, index, gathered-axis, source-suffix]`;
/// summing its index dimensions leaves exactly the source shape.
fn scatter_add(g: Node, index: Node, dim: usize, source_shape: &[Axis]) -> Node {
    let index_rank = index.shape().len();
    let mut iteration = g.shape();
    iteration.insert(dim + index_rank, source_shape[dim]);

    let g_dims = (0..g.shape().len())
        .map(|source_dim| {
            if source_dim < dim + index_rank {
                source_dim
            } else {
                source_dim + 1
            }
        })
        .collect::<Vec<_>>();
    let lifted_g = lift_to_shape(g, iteration.clone(), &g_dims);
    let index_dims = (dim..dim + index_rank).collect::<Vec<_>>();
    let lifted_index = lift_to_shape(index, iteration, &index_dims);
    let target = ir::coordinate(lifted_g.clone(), dim + index_rank);
    let mut out = map(MapOp::Mul, vec![lifted_g, equal(target, lifted_index)]);
    for index_dim in (dim..dim + index_rank).rev() {
        out = reduce(out, index_dim, Monoid::Add);
    }
    out
}

/// Invert a storage-preserving positional view. Singleton insertions reduce
/// away, flattened groups split back into their members, and the final view
/// restores the source dimension order.
fn invert_view(mut g: Node, source_shape: &[Axis], dims: &[ViewDim]) -> Node {
    let mut labels = Vec::<usize>::new();
    let mut current_dim = 0usize;
    for dim in dims {
        match dim.sources.as_slice() {
            [] => {
                g = ir::squeeze(g, current_dim);
            }
            [source_dim] => {
                labels.push(*source_dim);
                current_dim += 1;
            }
            members => {
                for member_index in 0..members.len() - 1 {
                    let outer = source_shape[members[member_index]];
                    let rest = &members[member_index + 1..];
                    let inner = if rest.len() == 1 {
                        source_shape[rest[0]]
                    } else {
                        Axis::new(
                            "view_grad_tail",
                            Extent::Static(
                                rest.iter()
                                    .map(|&source_dim| source_shape[source_dim].extent())
                                    .product(),
                            ),
                        )
                    };
                    g = split(g, current_dim + member_index, outer, inner);
                }
                labels.extend_from_slice(members);
                current_dim += members.len();
            }
        }
    }

    let reordered = source_shape
        .iter()
        .enumerate()
        .map(|(source_dim, &axis)| ViewDim {
            sources: vec![
                labels
                    .iter()
                    .position(|&label| label == source_dim)
                    .expect("view inverse lost a source dimension"),
            ],
            axis,
        })
        .collect();
    positional_view(g, reordered)
}

/// Generic transpose of an affine reindex. It contracts the cotangent over
/// every output coordinate against equality masks for all source dimensions.
/// This is intentionally dense but correct for slices, pads, windows, splits,
/// and arbitrary affine maps; later fusion can recover an efficient kernel.
fn transpose_reindex(g: Node, source_shape: &[Axis], output_shape: &[Axis], reindex: &[AffineIndex]) -> Node {
    let output_rank = output_shape.len();
    let mut iteration = output_shape.to_vec();
    iteration.extend_from_slice(source_shape);
    let g_dims = (0..output_rank).collect::<Vec<_>>();
    let lifted = lift_to_shape(g, iteration, &g_dims);
    let mut out = lifted.clone();

    for (source_dim, terms, offset) in reindex {
        let mut target = konst(*offset as f64);
        for (coefficient, output_dim) in terms {
            target = map(
                MapOp::Add,
                vec![
                    target,
                    map(
                        MapOp::Mul,
                        vec![konst(*coefficient as f64), ir::coordinate(lifted.clone(), *output_dim)],
                    ),
                ],
            );
        }
        let source_coordinate = ir::coordinate(lifted.clone(), output_rank + *source_dim);
        out = map(MapOp::Mul, vec![out, equal(source_coordinate, target)]);
    }
    for dim in (0..output_rank).rev() {
        out = reduce(out, dim, Monoid::Add);
    }
    out
}

/// `1` where `src` equals the reduced `result` (its running max/min), else
/// `0` — the subgradient mask. The caller divides by the winner count, so
/// ties share the mass instead of double-counting it.
fn winner_mask(src: &Node, result: &Node) -> Node {
    // [src == r] as (1 − [src < r]) · (1 − [r < src]) — max needs only the
    // first factor, min only the second, but the product is correct for both.
    let a = map(
        MapOp::Sub,
        vec![konst(1.0), map(MapOp::Lt, vec![src.clone(), result.clone()])],
    );
    let b = map(
        MapOp::Sub,
        vec![konst(1.0), map(MapOp::Lt, vec![result.clone(), src.clone()])],
    );
    map(MapOp::Mul, vec![a, b])
}

/// The per-operand cotangent contributions of one elementwise op:
/// `(contribution, operand)` pairs. Operands with zero contribution (an
/// `Lt`'s, a `Where`'s condition) are simply absent.
fn map_backward<'a>(op: MapOp, inputs: &'a [Node], g: &Node) -> Vec<(Node, &'a Node)> {
    let m = |op, v| map(op, v);
    match op {
        MapOp::Add => vec![(g.clone(), &inputs[0]), (g.clone(), &inputs[1])],
        MapOp::Sub => vec![(g.clone(), &inputs[0]), (m(MapOp::Neg, vec![g.clone()]), &inputs[1])],
        MapOp::Mul => vec![
            (m(MapOp::Mul, vec![g.clone(), inputs[1].clone()]), &inputs[0]),
            (m(MapOp::Mul, vec![g.clone(), inputs[0].clone()]), &inputs[1]),
        ],
        MapOp::Div => {
            let (a, b) = (&inputs[0], &inputs[1]);
            let da = m(MapOp::Div, vec![g.clone(), b.clone()]);
            let db = m(
                MapOp::Neg,
                vec![m(
                    MapOp::Div,
                    vec![
                        m(MapOp::Mul, vec![g.clone(), a.clone()]),
                        m(MapOp::Mul, vec![b.clone(), b.clone()]),
                    ],
                )],
            );
            vec![(da, a), (db, b)]
        }
        MapOp::Max | MapOp::Min => {
            let (a, b) = (&inputs[0], &inputs[1]);
            // subgradient: the winner takes g; ties go to `a` for Max's
            // first operand convention (1 − [a<b] vs [a<b])
            let a_wins = m(MapOp::Sub, vec![konst(1.0), m(MapOp::Lt, vec![a.clone(), b.clone()])]);
            let (wa, wb) = if op == MapOp::Max {
                (a_wins.clone(), m(MapOp::Lt, vec![a.clone(), b.clone()]))
            } else {
                (
                    m(MapOp::Lt, vec![a.clone(), b.clone()]),
                    m(MapOp::Sub, vec![konst(1.0), m(MapOp::Lt, vec![a.clone(), b.clone()])]),
                )
            };
            vec![
                (m(MapOp::Mul, vec![g.clone(), wa]), a),
                (m(MapOp::Mul, vec![g.clone(), wb]), b),
            ]
        }
        MapOp::Lt => vec![], // piecewise constant
        // straight-through: rounding is the identity almost everywhere, and
        // training through a declared storage boundary wants the estimator
        MapOp::RoundTo(_) => vec![(g.clone(), &inputs[0])],
        MapOp::Neg => vec![(m(MapOp::Neg, vec![g.clone()]), &inputs[0])],
        MapOp::Recip => {
            let x = &inputs[0];
            let d = m(
                MapOp::Neg,
                vec![m(
                    MapOp::Div,
                    vec![g.clone(), m(MapOp::Mul, vec![x.clone(), x.clone()])],
                )],
            );
            vec![(d, x)]
        }
        MapOp::Exp => vec![(
            // reuse the forward node when the caller shares it; rebuilding is
            // equal by value
            m(MapOp::Mul, vec![g.clone(), m(MapOp::Exp, vec![inputs[0].clone()])]),
            &inputs[0],
        )],
        MapOp::Log => vec![(m(MapOp::Div, vec![g.clone(), inputs[0].clone()]), &inputs[0])],
        MapOp::Sqrt => {
            let x = &inputs[0];
            let d = m(
                MapOp::Div,
                vec![
                    g.clone(),
                    m(MapOp::Mul, vec![konst(2.0), m(MapOp::Sqrt, vec![x.clone()])]),
                ],
            );
            vec![(d, x)]
        }
        MapOp::Tanh => {
            let t = m(MapOp::Tanh, vec![inputs[0].clone()]);
            let d = m(
                MapOp::Mul,
                vec![
                    g.clone(),
                    m(MapOp::Sub, vec![konst(1.0), m(MapOp::Mul, vec![t.clone(), t])]),
                ],
            );
            vec![(d, &inputs[0])]
        }
        MapOp::Sin => vec![(
            m(MapOp::Mul, vec![g.clone(), m(MapOp::Cos, vec![inputs[0].clone()])]),
            &inputs[0],
        )],
        MapOp::Cos => vec![(
            m(
                MapOp::Neg,
                vec![m(MapOp::Mul, vec![g.clone(), m(MapOp::Sin, vec![inputs[0].clone()])])],
            ),
            &inputs[0],
        )],
        MapOp::Where => {
            let c = &inputs[0];
            let da = m(MapOp::Where, vec![c.clone(), g.clone(), konst(0.0)]);
            let db = m(MapOp::Where, vec![c.clone(), konst(0.0), g.clone()]);
            vec![(da, &inputs[1]), (db, &inputs[2])] // the condition gets none
        }
    }
}

fn broadcast_to(n: Node, target: &[Axis]) -> Node {
    let source = n.shape();
    assert!(source.len() <= target.len(), "broadcast rank cannot shrink");
    if source == target {
        return n;
    }

    let leading = target.len() - source.len();
    if leading == 0 && source.iter().zip(target).all(|(from, to)| from.extent == to.extent) {
        let dims = target
            .iter()
            .enumerate()
            .map(|(dim, &axis)| ViewDim {
                sources: vec![dim],
                axis,
            })
            .collect();
        return positional_view(n, dims);
    }

    let map = source
        .iter()
        .enumerate()
        .map(|(source_dim, from)| {
            let output_dim = leading + source_dim;
            let to = target[output_dim];
            let terms = if from.extent == to.extent {
                vec![(1, output_dim)]
            } else {
                assert_eq!(
                    from.extent,
                    Extent::Static(1),
                    "cannot broadcast extent {:?} to {:?}",
                    from.extent,
                    to.extent
                );
                Vec::new()
            };
            (source_dim, terms, 0)
        })
        .collect();
    positional_reindex(n, target.to_vec(), map, false)
}

/// Sum a cotangent down to a trailing-broadcast operand shape, then restore
/// reduced singleton dimensions in their original positions.
fn reduce_to(n: Node, target: &[Axis]) -> Node {
    let source = n.shape();
    assert!(source.len() >= target.len(), "gradient rank cannot grow");
    let leading = source.len() - target.len();
    let mut reduced = (0..leading).collect::<Vec<_>>();
    for (target_dim, target_axis) in target.iter().enumerate() {
        let source_dim = leading + target_dim;
        if target_axis.extent == Extent::Static(1) && source[source_dim].extent != Extent::Static(1) {
            reduced.push(source_dim);
        }
    }

    let mut out = n;
    for &dim in reduced.iter().rev() {
        out = reduce(out, dim, Monoid::Add);
    }
    let survivors = (0..source.len())
        .filter(|dim| !reduced.contains(dim))
        .collect::<Vec<_>>();
    let dims = target
        .iter()
        .enumerate()
        .map(|(target_dim, &axis)| {
            let original = leading + target_dim;
            let survivor = survivors.iter().position(|&dim| dim == original);
            ViewDim {
                sources: survivor.into_iter().collect(),
                // A surviving singleton cannot be relabeled as a larger
                // dimension by a storage-preserving view. Keep it singleton
                // here; the explicit reindex below performs the broadcast.
                axis: survivor
                    .filter(|_| source[original].extent != axis.extent)
                    .map(|_| source[original])
                    .unwrap_or(axis),
            }
        })
        .collect();
    out = positional_view(out, dims);
    broadcast_to(out, target)
}

fn add_adj(adj: &mut HashMap<*const NodeKind, Node>, child: &Node, contrib: Node) {
    match adj.get(&Arc::as_ptr(child)) {
        None => {
            adj.insert(Arc::as_ptr(child), contrib);
        }
        Some(prev) => {
            adj.insert(Arc::as_ptr(child), map(MapOp::Add, vec![prev.clone(), contrib]));
        }
    }
}

fn postorder(node: &Node, seen: &mut Vec<*const NodeKind>, out: &mut Vec<Node>) {
    let ptr = Arc::as_ptr(node);
    if seen.contains(&ptr) {
        return;
    }
    seen.push(ptr);
    match node.as_ref() {
        NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => {}
        NodeKind::Coordinate { src, .. } => postorder(src, seen, out),
        NodeKind::Map { inputs, .. } => {
            for i in inputs {
                postorder(i, seen, out);
            }
        }
        NodeKind::Reduce { src, .. }
        | NodeKind::Scan { src, .. }
        | NodeKind::View { src, .. }
        | NodeKind::Reindex { src, .. } => postorder(src, seen, out),
        NodeKind::Gather { src, index, .. } => {
            postorder(src, seen, out);
            postorder(index, seen, out);
        }
    }
    out.push(node.clone());
}
