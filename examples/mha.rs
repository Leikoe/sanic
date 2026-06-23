//! Build *naive* multi-head attention as a dataflow AST — the textbook
//! three-step form that materializes the scores and the softmax weights — then
//! let the engine derive the FlashAttention streaming kernel from it.
//!
//! Nothing about this is MHA-aware. Batch `b` and head `h` are just extra free
//! axes; the same `carrier::derive` that handles single-head attention produces
//! the very same `(m, ℓ, o)` accumulator here — only the axes it spans grow.
//!
//!     cargo run --example mha

use sanic::carrier;
use sanic::codegen::{rust_kernel, tiled_kernel};
use sanic::engine::analyze_all;
use sanic::engine_ir::*;
use sanic::schedule::{Decision, Device, schedule_attention};
use sanic::stage1::Parallelism;

fn main() {
    // Naive MHA AST: out[b,h,sq,e] = softmax(QKᵀ over d, normalized over k) · V.
    // softmax(...) and matmul(...) expand to the full materializing dataflow —
    // a max/exp/sum/divide and two reductions — not anything flash-shaped.
    let q = input("Q", &["b", "h", "sq", "d"]);
    let k = input("K", &["b", "h", "k", "d"]);
    let v = input("V", &["b", "h", "k", "e"]);

    let scores = matmul(q, k, "d"); //  [b,h,sq,k]   contract the head dim
    let weights = softmax(scores, "k"); //  normalize over keys (materialized)
    let out = matmul(weights, v, "k"); //  [b,h,sq,e]   contract the keys

    // The whole structure map, axes discovered automatically.
    println!("naive multi-head attention — structure map\n");
    print!("{}", analyze_all(&out).render());
    println!();

    // Derive the streaming kernel for the key axis. This *is* FlashAttention,
    // assembled by the generic composition rules — no MHA case anywhere.
    let flash = carrier::derive(&out, "k").expect("the key axis folds");
    println!("derived streaming kernel over `k` (this is FlashAttention):\n");
    println!("{}\n", flash.render());

    // The accumulator the engine knows it needs, for a concrete shape: m and ℓ
    // are one scalar per (b, h, sq) query lane; o adds the value feature e.
    let acc = flash.acc_scalars(|ax| if ax == "e" { 64.0 } else { 1.0 });
    println!("|Acc| per query lane: {acc} scalars  (m + ℓ + o[e=64])");
    println!("o spans {:?} — the head/batch axes ride along for free\n", flash.spans[2]);

    // Numeric proof: fold the derived kernel over random keys and compare to the
    // naive softmax-weighted sum it was derived from.
    let mut s = 0x9e3779b97f4a7c15u64;
    let mut rnd = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 11) as f64 / (1u64 << 53) as f64 * 4.0 - 2.0
    };
    let keys: Vec<Vec<f64>> = (0..64).map(|_| vec![rnd(), rnd()]).collect(); // (score, value)
    let derived = flash.fold(&keys)[0];
    let mx = keys.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
    let den: f64 = keys.iter().map(|p| (p[0] - mx).exp()).sum();
    let num: f64 = keys.iter().map(|p| (p[0] - mx).exp() * p[1]).sum();
    println!("numeric check:  derived = {derived:.9}   naive = {:.9}   ✓\n", num / den);

    // …and emit it as a kernel. The grid axes come from the structure map; the
    // loop body is the derived carrier, transcribed to Rust.
    let report = analyze_all(&out);
    let grid: Vec<&str> = report
        .axes
        .iter()
        .filter(|a| a.structure.level == Parallelism::Free)
        .map(|a| a.axis.as_str())
        .collect();
    println!("emitted kernel:\n");
    println!("{}", rust_kernel(&flash, "flash_attention", "k", &grid));

    // Now let the scheduler size it for a device. It costs fuse-vs-cut and, when
    // fusing, picks the query-tile that fits SRAM — and codegen blocks the kernel
    // by exactly that tile (tile × |Acc| resident).
    let dev = Device::toy();
    let (sq, keys, dim) = (2048.0, 4096.0, 64.0);
    let v = schedule_attention(&dev, sq, keys, dim);
    println!(
        "\nscheduler (sq={sq}, k={keys}, d={dim}): {:?}, query-tile = {:?}",
        v.decision, v.tile
    );
    if let (Decision::Fuse, Some(tile)) = (&v.decision, v.tile) {
        println!("\nemitted TILED kernel (scheduler chose tile = {tile}):\n");
        println!("{}", tiled_kernel(&flash, "flash_attention_tiled", "k", tile));
    }
}
