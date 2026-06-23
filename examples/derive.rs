//! Derive streaming accumulators straight from the dataflow graph and print the
//! result as readable math. Nothing here is hand-written — every formula below
//! is constructed by the composition rules in `carrier::derive`.
//!
//!     cargo run --example derive

use sanic::carrier;
use sanic::engine::analyze_all;
use sanic::engine_ir::*;
use sanic::op::{BinOp, Monoid};

fn add() -> BinOp {
    BinOp::Monoid(Monoid::Add)
}
fn max() -> BinOp {
    BinOp::Monoid(Monoid::Max)
}
fn show(title: &str, node: &Node, axis: &str) {
    println!("── {title} (fold over `{axis}`) ──");
    match carrier::derive(node, axis) {
        Some(c) => println!("{}\n", c.render()),
        None => println!("not foldable on this axis\n"),
    }
}

fn main() {
    // mean = (Σx) / (Σ1)  →  strengthened to (sum, count)
    let x = input("X", &["a"]);
    let mean = map(DIV, vec![
        reduce(x.clone(), "a", add()),
        reduce(map(ONE, vec![x.clone()]), "a", add()),
    ]);
    show("mean", &mean, "a");

    // logsumexp = log(Σ exp(x − m)) + m  →  the (max, Σexp) carrier
    let m = reduce(x.clone(), "a", max());
    let e = map(EXP_SUB, vec![x.clone(), m.clone()]);
    let lse = map(ADD_F, vec![map(LOG, vec![reduce(e, "a", add())]), m]);
    show("logsumexp", &lse, "a");

    // attention = softmax(QKᵀ)·V — the whole structure map in one call: the
    // query axis is a grid, the key axis folds into the FlashAttention
    // (m, ℓ, o) accumulator (online-softmax coupling R4 + deferred normalizer R5).
    let q = input("Q", &["sq", "d"]);
    let k = input("K", &["k", "d"]);
    let v = input("V", &["k", "e"]);
    let attn = attention(q, k, v, "d", "k");
    println!("── attention: softmax(QKᵀ)·V (every axis, auto-discovered) ──");
    print!("{}", analyze_all(&attn).render());
    println!();

    // the time axis of a tanh-RNN: a non-associative recurrence → refused.
    let rnn = tanh_rnn(input("H", &["t", "h"]), "t");
    show("tanh-RNN time axis", &rnn, "t");
}
