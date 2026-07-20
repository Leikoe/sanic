//! Derive streaming accumulators straight from the dataflow graph and print
//! the result as readable math. Nothing here is hand-written — every formula
//! below is constructed by the composition rules in `sanic::derive`, from the
//! closed op basis (softmax is `Exp(x − max)`, not a fused special form).
//!
//!     cargo run --example derive

use sanic::ir::*;
use sanic::{analyze_all, derive};

fn add() -> BinOp {
    BinOp::Monoid(Monoid::Add)
}
fn max() -> BinOp {
    BinOp::Monoid(Monoid::Max)
}
fn show(title: &str, node: &Node, ax: Axis) {
    println!("── {title} (fold over `{ax}`) ──");
    match derive(node, ax) {
        Some(c) => println!("{}\n", c.render()),
        None => println!("not foldable on this axis\n"),
    }
}

fn main() {
    // mean = (Σx) / (Σ1)  →  strengthened to (sum, count)
    let a = axis("a");
    let x = input("X", &[a], Dtype::F32);
    let mean = map(
        MapOp::Div,
        vec![reduce(x.clone(), a, add()), reduce(konst(1.0), a, add())],
    );
    show("mean", &mean, a);

    // logsumexp = log(Σ exp(x − m)) + m  →  the (max, Σexp) carrier. The
    // exp/max coupling is discovered from plain composition.
    let m = reduce(x.clone(), a, max());
    let e = map(
        MapOp::Exp,
        vec![map(MapOp::Sub, vec![x.clone(), m.clone()])],
    );
    let lse = map(
        MapOp::Add,
        vec![map(MapOp::Log, vec![reduce(e, a, add())]), m],
    );
    show("logsumexp", &lse, a);

    // attention = softmax(QKᵀ)·V — the whole structure map in one call: the
    // query axis is a grid, the key axis folds into the FlashAttention
    // (m, ℓ, o) accumulator (the online-softmax rescale plus the deferred
    // divide by the normalizer).
    let (sq, k, d, e_ax) = (axis("sq"), axis("k"), axis("d"), axis("e"));
    let q = input("Q", &[sq, d], Dtype::F32);
    let kk = input("K", &[k, d], Dtype::F32);
    let v = input("V", &[k, e_ax], Dtype::F32);
    let attn = attention(q, kk, v, d, k);
    println!("── attention: softmax(QKᵀ)·V (every axis, auto-discovered) ──");
    print!("{}", analyze_all(&attn).render());
    println!();

    // the time axis of a tanh-RNN: a non-associative recurrence → refused.
    let (t, h) = (axis("t"), axis("h"));
    let rnn = tanh_rnn(input("H", &[t, h], Dtype::F32), t);
    show("tanh-RNN time axis", &rnn, t);
}
