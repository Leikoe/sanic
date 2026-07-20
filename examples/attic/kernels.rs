//! A gallery of derived FlashAttention kernels, emitted as real code.
//!
//! Every kernel below is constructed by `derive` from a naive dataflow graph
//! — max, exp, sum, divide, two matmuls — and transcribed by the emitters.
//! No flash-shaped template exists anywhere; the variants differ only in the
//! graph handed in.
//!
//!     cargo run --example kernels

use std::collections::HashMap;

use sanic::cost::DeviceProfile;
use sanic::derive;
use sanic::emit_rust::{rust_kernel, tiled_kernel};
use sanic::ir::*;
use sanic::plan::plan;

fn banner(title: &str) {
    println!("\n{}", "═".repeat(74));
    println!("  {title}");
    println!("{}\n", "═".repeat(74));
}

fn main() {
    // ── 1. vanilla single-head flash, as scalar Rust ─────────────────────────
    banner("1. softmax(QKᵀ)·V  →  the (m, ℓ, o) fold, scalar Rust");
    let (sq, k, d, e) = (axis("sq"), axis("k"), axis("d"), axis("e"));
    let attn = attention(
        input("Q", &[sq, d], Dtype::F32),
        input("K", &[k, d], Dtype::F32),
        input("V", &[k, e], Dtype::F32),
        d,
        k,
    );
    let flash = derive(&attn, k).unwrap();
    println!("derived carrier:\n{}\n", flash.render());
    println!(
        "{}",
        rust_kernel(&flash, "flash_attention", "k", &["sq", "e"])
    );

    // ── 2. causally-masked + scaled flash: the mask is index arithmetic ─────
    banner("2. softmax(QKᵀ·c + causal)·V — computed mask fused into the lift");
    let (s2, t2, dk2, dv2) = (axis("s"), axis("t"), axis("dk"), axis("dv"));
    let scores = matmul(input("Q", &[s2, dk2], Dtype::F32), input("K", &[t2, dk2], Dtype::F32), dk2);
    let scaled = map(MapOp::Mul, vec![scores, konst(0.125)]);
    let masked = map(MapOp::Add, vec![scaled, causal_mask(s2, t2)]);
    let causal = matmul(softmax(masked, t2), input("V", &[t2, dv2], Dtype::F32), t2);
    let cf = derive(&causal, t2).unwrap();
    println!("derived carrier (x0=score, x1=query pos, x2=key pos, x3=value):");
    println!("{}\n", cf.render());
    println!(
        "{}",
        rust_kernel(&cf, "causal_flash_attention", "t", &["s", "dv"])
    );

    // ── 3. the planner-tiled variant ─────────────────────────────────────────
    banner("3. the same fold, blocked by the tile the planner priced");
    let dev = DeviceProfile::toy();
    let extents: HashMap<Axis, f64> = [(sq, 2048.0), (k, 4096.0), (d, 64.0), (e, 64.0)]
        .into_iter()
        .collect();
    let spec = &plan(&attn, &dev, &extents).unwrap().kernels[0];
    println!(
        "planner: row {}×{}, {} scalars resident\n",
        spec.row_axis.unwrap(),
        spec.tile_m,
        spec.tile_m * flash.slots
    );
    println!(
        "{}",
        tiled_kernel(&flash, "flash_attention_tiled", "k", spec.tile_m)
    );

    // ── 4. multi-value attention: two value tensors, one softmax ────────────
    banner("4. two value tensors sharing one softmax — (m, ℓ, o₁, o₂)");
    let k4 = axis("k");
    let sc4 = input("S", &[k4], Dtype::F32);
    let w4 = softmax(sc4, k4);
    let o1 = reduce(
        map(MapOp::Mul, vec![w4.clone(), input("V1", &[k4], Dtype::F32)]),
        k4,
        BinOp::Monoid(Monoid::Add),
    );
    let o2 = reduce(
        map(MapOp::Mul, vec![w4, input("V2", &[k4], Dtype::F32)]),
        k4,
        BinOp::Monoid(Monoid::Add),
    );
    let total = map(MapOp::Add, vec![o1, o2]);
    let mv = derive(&total, k4).unwrap();
    println!("{}", rust_kernel(&mv, "multi_value_attention", "k", &[]));
}
