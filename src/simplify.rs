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
//! bare `max` fold falls out as dead code. The result is exactly
//! `softmax − onehot`, with no LSE-specific gradient rule.
//!
//! Every rewrite is a value-preserving identity (off measure-zero
//! singularities, where the gradient is undefined anyway), so the
//! finite-difference tests in `tests/grad.rs` remain the correctness check.

use std::collections::HashMap;
use std::rc::Rc;

use crate::ir::{
    Axis, BinOp, MapOp, Monoid, Node, NodeKind, gather, konst, map, output_axes, reduce, reindex,
    scan, view,
};

/// Simplify `node` to a fixpoint: bottom-up rewriting with CSE, repeated until
/// a pass changes nothing.
pub fn simplify(node: &Node) -> Node {
    let mut cur = node.clone();
    for _ in 0..32 {
        let mut cse: HashMap<String, Node> = HashMap::new();
        let mut memo: HashMap<*const NodeKind, Node> = HashMap::new();
        let mut changed = false;
        cur = pass(&cur, &mut cse, &mut memo, &mut changed);
        if !changed {
            break;
        }
    }
    cur
}

/// One bottom-up pass: canonicalize children, apply a local rewrite, hash-cons.
fn pass(
    node: &Node,
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
        NodeKind::Map { op, inputs } => {
            let ins: Vec<Node> = inputs.iter().map(|i| pass(i, cse, memo, changed)).collect();
            let rebuilt = match map_rule(*op, &ins) {
                Some(r) => {
                    *changed = true;
                    r
                }
                None if same(&ins, inputs) => node.clone(),
                None => map(*op, ins),
            };
            hashcons(rebuilt, cse)
        }
        NodeKind::Reduce { src, axis, op } => {
            let s = pass(src, cse, memo, changed);
            let rebuilt = match reduce_rule(&s, *axis, *op) {
                Some(r) => {
                    *changed = true;
                    r
                }
                None if Rc::ptr_eq(&s, src) => node.clone(),
                None => reduce(s, *axis, *op),
            };
            hashcons(rebuilt, cse)
        }
        NodeKind::Scan { src, axis, op } => {
            let s = pass(src, cse, memo, changed);
            let rebuilt = if Rc::ptr_eq(&s, src) { node.clone() } else { scan(s, *axis, *op) };
            hashcons(rebuilt, cse)
        }
        NodeKind::Gather { src, index, axis } => {
            let s = pass(src, cse, memo, changed);
            let i = pass(index, cse, memo, changed);
            let rebuilt = if Rc::ptr_eq(&s, src) && Rc::ptr_eq(&i, index) {
                node.clone()
            } else {
                gather(s, i, *axis)
            };
            hashcons(rebuilt, cse)
        }
        NodeKind::View { src, groups } => {
            let s = pass(src, cse, memo, changed);
            let rebuilt = if Rc::ptr_eq(&s, src) { node.clone() } else { view(s, groups.clone()) };
            hashcons(rebuilt, cse)
        }
        NodeKind::Reindex { src, map: m, padded } => {
            let s = pass(src, cse, memo, changed);
            let rebuilt = if Rc::ptr_eq(&s, src) {
                node.clone()
            } else {
                reindex(s, m.clone(), *padded)
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
/// test for the rewrites below.
fn hashcons(node: Node, cse: &mut HashMap<String, Node>) -> Node {
    cse.entry(key(&node)).or_insert(node).clone()
}

fn key(n: &Node) -> String {
    let p = |c: &Node| Rc::as_ptr(c) as usize;
    match n.as_ref() {
        NodeKind::Const { v } => format!("C{}", v.to_bits()),
        NodeKind::Input { name, axes, dtype } => format!("I{name}{axes:?}{dtype:?}"),
        NodeKind::Iota { axis } => format!("O{axis:?}"),
        NodeKind::Map { op, inputs } => {
            format!("M{op:?}{:?}", inputs.iter().map(p).collect::<Vec<_>>())
        }
        NodeKind::Reduce { src, axis, op } => format!("R{op:?}{axis:?}.{}", p(src)),
        NodeKind::Scan { src, axis, op } => format!("S{op:?}{axis:?}.{}", p(src)),
        NodeKind::Gather { src, index, axis } => format!("G{axis:?}.{}.{}", p(src), p(index)),
        NodeKind::View { src, groups } => format!("V{groups:?}.{}", p(src)),
        NodeKind::Reindex { src, map, padded } => format!("X{map:?}{padded}.{}", p(src)),
    }
}

// ── local rewrites (children already canonical, so `≡` is `Rc::ptr_eq`) ───────

/// Are two operand lists pointer-identical (nothing rebuilt beneath)?
fn same(a: &[Node], b: &[Node]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| Rc::ptr_eq(x, y))
}

fn cst(n: &Node) -> Option<f64> {
    match n.as_ref() {
        NodeKind::Const { v } => Some(*v),
        _ => None,
    }
}

fn as_map<'a>(n: &'a Node, want: MapOp) -> Option<&'a [Node]> {
    match n.as_ref() {
        NodeKind::Map { op, inputs } if *op == want => Some(inputs),
        _ => None,
    }
}

/// One rewrite of an elementwise op over canonical operands, or `None`.
fn map_rule(op: MapOp, ins: &[Node]) -> Option<Node> {
    let eq = |a: &Node, b: &Node| Rc::ptr_eq(a, b);
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
            if let Some(d) = as_map(a, MapOp::Div) {
                if eq(&d[1], b) {
                    return Some(d[0].clone());
                }
            }
            if let Some(d) = as_map(b, MapOp::Div) {
                if eq(&d[1], a) {
                    return Some(d[0].clone());
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
fn reduce_rule(s: &Node, axis: Axis, op: BinOp) -> Option<Node> {
    if op != BinOp::Monoid(Monoid::Add) {
        return None;
    }
    if cst(s) == Some(0.0) {
        return Some(konst(0.0)); // Σ 0 = 0 for any extent
    }
    // Σ(k · x) → k · Σx when k is invariant along the axis (defer-scale)
    if let Some(m) = as_map(s, MapOp::Mul) {
        let (l, r) = (&m[0], &m[1]);
        if !output_axes(l).contains(&axis) {
            return Some(map(MapOp::Mul, vec![l.clone(), reduce(r.clone(), axis, op)]));
        }
        if !output_axes(r).contains(&axis) {
            return Some(map(MapOp::Mul, vec![r.clone(), reduce(l.clone(), axis, op)]));
        }
    }
    // Σ(−x) → −Σx
    if let Some(n) = as_map(s, MapOp::Neg) {
        return Some(map(MapOp::Neg, vec![reduce(n[0].clone(), axis, op)]));
    }
    None
}
