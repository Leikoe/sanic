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
//!   nothing — they double-count, a measure-zero event on continuous data)
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

use std::collections::HashMap;
use std::rc::Rc;

use crate::ir::{
    Axis, BinOp, MapOp, Monoid, Node, NodeKind, iota, konst, map, output_axes, reduce, reindex,
    scatter_add, view,
};

/// d(loss)/d(each named input), as graphs. `loss` must be a scalar (no free
/// axes). Inputs that the loss does not depend on are absent from the result.
pub fn grad(loss: &Node, wrt: &[&'static str]) -> HashMap<&'static str, Node> {
    assert!(
        output_axes(loss).is_empty(),
        "grad: the loss must be a scalar (reduce it first); got free axes {:?}",
        output_axes(loss)
    );

    // reverse topological order (postorder of the DAG, reversed)
    let mut order: Vec<Node> = Vec::new();
    let mut seen: Vec<*const NodeKind> = Vec::new();
    postorder(loss, &mut seen, &mut order);

    // cotangent per node, accumulated across consumers
    let mut adj: HashMap<*const NodeKind, Node> = HashMap::new();
    adj.insert(Rc::as_ptr(loss), konst(1.0));

    // gradient per input NAME: distinct `Input` nodes with the same name are
    // the same buffer read under (possibly) different axis labels, so their
    // adjoints sum after relabeling to the first-seen declaration.
    let mut by_name: HashMap<&'static str, (Vec<Axis>, Node)> = HashMap::new();

    for node in order.iter().rev() {
        let Some(g) = adj.get(&Rc::as_ptr(node)).cloned() else {
            continue; // the loss does not depend on this node
        };
        match node.as_ref() {
            NodeKind::Input { name, axes, .. } => {
                if !wrt.contains(name) {
                    continue;
                }
                let contrib = reduce_to(g, axes);
                match by_name.get_mut(name) {
                    None => {
                        by_name.insert(name, (axes.clone(), contrib));
                    }
                    Some((canon, acc)) => {
                        // relabel this declaration's axes onto the canonical
                        // ones, positionally — the same rebinding the
                        // interpreter applies to aliased reads.
                        assert_eq!(
                            canon.len(),
                            axes.len(),
                            "grad: input `{name}` declared with different ranks"
                        );
                        let relabeled = if canon == axes {
                            contrib
                        } else {
                            let groups: Vec<(Vec<Axis>, Axis)> = axes
                                .iter()
                                .zip(canon.iter())
                                .filter(|(a, c)| a != c)
                                .map(|(a, c)| (vec![*a], *c))
                                .collect();
                            view(contrib, groups)
                        };
                        *acc = map(MapOp::Add, vec![acc.clone(), relabeled]);
                    }
                }
            }
            NodeKind::Const { .. } | NodeKind::Iota { .. } => {}

            NodeKind::Map { op, inputs } => {
                for (contrib, child) in map_backward(*op, inputs, &g) {
                    let target = output_axes(child);
                    add_adj(&mut adj, child, reduce_to(contrib, &target));
                }
            }

            NodeKind::Reduce { src, axis, op } => {
                let mut contrib = match op {
                    BinOp::Monoid(Monoid::Add) => g.clone(), // broadcast back along `axis`
                    BinOp::Monoid(Monoid::Max) | BinOp::Monoid(Monoid::Min) => {
                        // winner mask: 1 where src equals the reduced result
                        let hit = winner_mask(src, node);
                        map(MapOp::Mul, vec![g.clone(), hit])
                    }
                    BinOp::Monoid(Monoid::LogSumExp) => {
                        // ∂LSE/∂src = exp(src − LSE) — the softmax Jacobian row
                        let sm = map(
                            MapOp::Exp,
                            vec![map(MapOp::Sub, vec![src.clone(), node.clone()])],
                        );
                        map(MapOp::Mul, vec![g.clone(), sm])
                    }
                    BinOp::Monoid(Monoid::Mul) => panic!(
                        "grad: Reduce(Mul) has no gradient here — result/src is \
                         undefined at zeros; rewrite the product or extend the rule"
                    ),
                    other => panic!("grad: reduce with {other:?} is not differentiable"),
                };
                // Reducing an axis the source does not carry folds `n` copies
                // of the same value: forward = n·src for Add (and shifts LSE
                // by ln n), so the additive contributions scale by n. Max/Min
                // of identical copies stay the identity.
                if !output_axes(src).contains(axis)
                    && matches!(
                        op,
                        BinOp::Monoid(Monoid::Add) | BinOp::Monoid(Monoid::LogSumExp)
                    )
                {
                    contrib = map(MapOp::Mul, vec![contrib, konst(axis.extent as f64)]);
                }
                add_adj(&mut adj, src, contrib);
            }

            NodeKind::Gather { src, index, axis } => {
                // adjoint of an indexed read: an add-combined indexed write
                let idx_axes = output_axes(index);
                assert_eq!(
                    idx_axes.len(),
                    1,
                    "grad: gather backward needs a single-axis index (flatten first)"
                );
                let scattered = scatter_add(g.clone(), index.clone(), idx_axes[0], *axis);
                add_adj(&mut adj, src, scattered);
                // integer indices carry no gradient
            }

            NodeKind::View { src, groups } => {
                // the inverse reindexing of the cotangent
                let mut inv = g.clone();
                for (members, to) in groups {
                    inv = match members.len() {
                        1 => view(inv, vec![(vec![*to], members[0])]), // rename back
                        2 => {
                            // flatten ⟵ split (row-major: first member most
                            // significant, inner extent = the second member's)
                            crate::ir::split(inv, *to, members[0], members[1])
                        }
                        n => panic!("grad: view backward for {n}-member groups not implemented"),
                    };
                }
                add_adj(&mut adj, src, inv);
            }

            NodeKind::Reindex { src, map: rmap, .. } => {
                let mut inv = g.clone();
                for (m, terms, off) in rmap {
                    inv = transpose_axis_map(inv, *m, terms, *off);
                }
                add_adj(&mut adj, src, inv);
            }

            // A prefix SUM's adjoint is the reversed prefix sum of the
            // cotangent: y_j = Σ_{i≤j} x_i ⇒ ∂L/∂x_i = Σ_{j≥i} g_j —
            // reverse, cumsum, reverse, all existing IR (reversal is a
            // self-referential Reindex). Max/Min scans (winner masks per
            // prefix) and the affine scan's adjoint recurrence stay
            // declined, stated in todo.md.
            NodeKind::Scan {
                src,
                axis,
                op: BinOp::Monoid(Monoid::Add),
            } => {
                let rev = |x: Node| {
                    let flip = (axis.extent - 1) as i64;
                    crate::ir::reindex(x, vec![(*axis, vec![(-1, *axis)], flip)], false)
                };
                let contrib = rev(crate::ir::scan(
                    rev(g.clone()),
                    *axis,
                    BinOp::Monoid(Monoid::Add),
                ));
                add_adj(&mut adj, src, contrib);
            }
            NodeKind::Scan { op, .. } => panic!(
                "grad: {op:?} scan backward is not implemented (Add-scan has \
                 the reversed-cumsum rule; Max/Min need per-prefix winner \
                 masks and the affine scan an adjoint recurrence — see todo.md)"
            ),
        }
    }

    by_name.into_iter().map(|(n, (_, g))| (n, g)).collect()
}

/// The transpose of one affine axis map: the cotangent `g` (over the map's
/// term axes) pushed back to the source axis `m`.
fn transpose_axis_map(g: Node, m: Axis, terms: &[(i64, Axis)], off: i64) -> Node {
    match terms {
        // constant index: the whole cotangent lands on position `off`
        [] => map(
            MapOp::Mul,
            vec![g, crate::ir::one_hot(m, konst(off as f64))],
        ),
        // slice / pad: a = m − off, out-of-range contributes nothing
        [(1, a)] => reindex(g, vec![(*a, vec![(1, m)], -off)], true),
        // split: the exact inverse is a flatten view (row-major, checked)
        [(c1, a1), (1, a2)] if off == 0 && a2.extent as i64 == *c1 => {
            view(g, vec![(vec![*a1, *a2], m)])
        }
        // stride-1 window: overlap-add — read the cotangent at the mirrored
        // offset (a1 = m − d·a2 − off) and sum over the window axis
        [(1, a1), (d, a2)] => reduce(
            reindex(g, vec![(*a1, vec![(1, m), (-d, *a2)], -off)], true),
            *a2,
            BinOp::Monoid(Monoid::Add),
        ),
        // dilation-1 window with stride: mirrored, summing the other axis
        [(st, a1), (1, a2)] if off == 0 => reduce(
            reindex(g, vec![(*a2, vec![(1, m), (-st, *a1)], -off)], true),
            *a1,
            BinOp::Monoid(Monoid::Add),
        ),
        // strided AND dilated (or offset) window: the inverse needs a
        // modular division, which no affine reindex expresses — so scatter
        // DENSELY through a one-hot contraction instead, exactly like
        // `scatter_add` (gather's adjoint):
        //   grad[m] = Σ_{a1,a2} g[a1,a2] · [m = c1·a1 + c2·a2 + off]
        // O(|g|·|m|) as a graph; the partitioner folds it like any
        // contraction, and out-of-range forward positions match no m, so a
        // padded read's dropped cotangent falls out for free.
        [(c1, a1), (c2, a2)] => {
            let target = map(
                MapOp::Add,
                vec![
                    map(
                        MapOp::Add,
                        vec![
                            map(MapOp::Mul, vec![konst(*c1 as f64), iota(*a1)]),
                            map(MapOp::Mul, vec![konst(*c2 as f64), iota(*a2)]),
                        ],
                    ),
                    konst(off as f64),
                ],
            );
            let sel = crate::ir::one_hot(m, target);
            reduce(
                reduce(
                    map(MapOp::Mul, vec![g, sel]),
                    *a2,
                    BinOp::Monoid(Monoid::Add),
                ),
                *a1,
                BinOp::Monoid(Monoid::Add),
            )
        }
        _ => panic!(
            "grad: no transpose for reindex terms {terms:?} (more than two \
             driving axes; decompose the index map first)"
        ),
    }
}

/// `1` where `src` equals the reduced `result` (its running max/min), else
/// `0` — the subgradient mask. Exact for a unique winner; ties double-count.
fn winner_mask(src: &Node, result: &Node) -> Node {
    // [src == r] as (1 − [src < r]) · (1 − [r < src]) — max needs only the
    // first factor, min only the second, but the product is correct for both.
    let a = map(
        MapOp::Sub,
        vec![
            konst(1.0),
            map(MapOp::Lt, vec![src.clone(), result.clone()]),
        ],
    );
    let b = map(
        MapOp::Sub,
        vec![
            konst(1.0),
            map(MapOp::Lt, vec![result.clone(), src.clone()]),
        ],
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
        MapOp::Sub => vec![
            (g.clone(), &inputs[0]),
            (m(MapOp::Neg, vec![g.clone()]), &inputs[1]),
        ],
        MapOp::Mul => vec![
            (
                m(MapOp::Mul, vec![g.clone(), inputs[1].clone()]),
                &inputs[0],
            ),
            (
                m(MapOp::Mul, vec![g.clone(), inputs[0].clone()]),
                &inputs[1],
            ),
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
            let a_wins = m(
                MapOp::Sub,
                vec![konst(1.0), m(MapOp::Lt, vec![a.clone(), b.clone()])],
            );
            let (wa, wb) = if op == MapOp::Max {
                (a_wins.clone(), m(MapOp::Lt, vec![a.clone(), b.clone()]))
            } else {
                (
                    m(MapOp::Lt, vec![a.clone(), b.clone()]),
                    m(
                        MapOp::Sub,
                        vec![konst(1.0), m(MapOp::Lt, vec![a.clone(), b.clone()])],
                    ),
                )
            };
            vec![
                (m(MapOp::Mul, vec![g.clone(), wa]), a),
                (m(MapOp::Mul, vec![g.clone(), wb]), b),
            ]
        }
        MapOp::Lt => vec![], // piecewise constant
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
            m(
                MapOp::Mul,
                vec![g.clone(), m(MapOp::Exp, vec![inputs[0].clone()])],
            ),
            &inputs[0],
        )],
        MapOp::Log => vec![(
            m(MapOp::Div, vec![g.clone(), inputs[0].clone()]),
            &inputs[0],
        )],
        MapOp::Sqrt => {
            let x = &inputs[0];
            let d = m(
                MapOp::Div,
                vec![
                    g.clone(),
                    m(
                        MapOp::Mul,
                        vec![konst(2.0), m(MapOp::Sqrt, vec![x.clone()])],
                    ),
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
                    m(
                        MapOp::Sub,
                        vec![konst(1.0), m(MapOp::Mul, vec![t.clone(), t])],
                    ),
                ],
            );
            vec![(d, &inputs[0])]
        }
        MapOp::Sin => vec![(
            m(
                MapOp::Mul,
                vec![g.clone(), m(MapOp::Cos, vec![inputs[0].clone()])],
            ),
            &inputs[0],
        )],
        MapOp::Cos => vec![(
            m(
                MapOp::Neg,
                vec![m(
                    MapOp::Mul,
                    vec![g.clone(), m(MapOp::Sin, vec![inputs[0].clone()])],
                )],
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

/// Sum a cotangent down to `target` axes — the broadcast-backward. Axes the
/// contribution carries that the operand does not are reduced away with Add.
fn reduce_to(n: Node, target: &[Axis]) -> Node {
    let mut out = n;
    for a in output_axes(&out) {
        if !target.contains(&a) {
            out = reduce(out, a, BinOp::Monoid(Monoid::Add));
        }
    }
    out
}

fn add_adj(adj: &mut HashMap<*const NodeKind, Node>, child: &Node, contrib: Node) {
    match adj.get(&Rc::as_ptr(child)) {
        None => {
            adj.insert(Rc::as_ptr(child), contrib);
        }
        Some(prev) => {
            adj.insert(
                Rc::as_ptr(child),
                map(MapOp::Add, vec![prev.clone(), contrib]),
            );
        }
    }
}

fn postorder(node: &Node, seen: &mut Vec<*const NodeKind>, out: &mut Vec<Node>) {
    let ptr = Rc::as_ptr(node);
    if seen.contains(&ptr) {
        return;
    }
    seen.push(ptr);
    match node.as_ref() {
        NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => {}
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
