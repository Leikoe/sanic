//! Multi-head Latent Attention (MLA) — as used in DeepSeek-v2/v3.
//!
//! MLA replaces the full per-head KV cache with a shared compressed latent
//! `c_kv[k, dc]`.  At inference, K and V are recovered on the fly:
//!
//!     K[k, dh] = c_kv[k, :] · W_uk^T           key up-projection
//!     V[k, de] = c_kv[k, :] · W_uv^T           value up-projection
//!     scores    = softmax(Q · K^T / √dh, axis=k)
//!     output    = scores · V
//!
//! The *absorbed* form eliminates the per-key key-projection by folding W_uk
//! into Q once per query block:
//!
//!     Q_abs     = Q · W_uk^T                    absorbed once, not per key
//!     scores    = Q_abs · c_kv^T                contract over dc (not dh)
//!     output    = softmax(scores, k) · V
//!
//! From the engine's perspective both are the same streaming problem: the
//! fusion axis is `k`, the carrier is (m, ℓ, o) — identical to standard
//! single-head attention. The structure map makes this structural equivalence
//! explicit: standard MLA shows two inner contractions (dc, dh); the absorbed
//! form collapses them into one (dc only).
//!
//!     cargo run --example mla

use sanic::carrier;
use sanic::codegen::rust_kernel;
use sanic::engine::analyze_all;
use sanic::engine_ir::*;
use sanic::stage1::Parallelism;

fn main() {
    // ── standard MLA ──────────────────────────────────────────────────────────
    println!("=== Standard MLA (explicit K/V up-projections) ===\n");

    let c_kv = input("c_kv", &["k", "dc"]);  // compressed KV latent
    let w_uk = input("W_uk", &["dh", "dc"]); // key up-projection
    let w_uv = input("W_uv", &["de", "dc"]); // value up-projection
    let q    = input("Q",    &["sq", "dh"]); // query (already in head space)

    let k_proj = matmul(c_kv.clone(), w_uk, "dc"); // [k, dh]
    let v_proj = matmul(c_kv.clone(), w_uv, "dc"); // [k, de]
    let scores  = matmul(q,           k_proj, "dh"); // [sq, k]
    let weights = softmax(scores, "k");
    let out_std = matmul(weights, v_proj, "k");      // [sq, de]

    print!("{}", analyze_all(&out_std).render());

    // The engine derives the same (m, ℓ, o) carrier despite the graph being
    // built through two up-projections. The carrier only sees the streaming
    // axis k; everything else is just a per-k sub-expression treated as a leaf.
    let car_std = carrier::derive(&out_std, "k").expect("k is foldable");
    println!("derived carrier for k:\n{}\n", car_std.render());

    // ── absorbed MLA ──────────────────────────────────────────────────────────
    println!("=== Absorbed MLA (W_uk folded into Q) ===\n");

    // Absorb the key projection into Q once per query block, not per key step.
    // This trades one per-key W_uk load for a single Q·W_uk^T matmul upfront,
    // and contracts over dc < dh — reducing per-key flops.
    let q_abs = input("Q_abs", &["sq", "dc"]); // = Q · W_uk^T, pre-computed
    let c_kv2 = input("c_kv",  &["k", "dc"]);  // same compressed latent
    let w_uv2 = input("W_uv",  &["de", "dc"]); // same value projection

    let scores2  = matmul(q_abs, c_kv2.clone(), "dc"); // [sq, k]  — contracts dc, not dh
    let weights2 = softmax(scores2, "k");
    let v_proj2  = matmul(c_kv2, w_uv2, "dc");          // [k, de]
    let out_abs  = matmul(weights2, v_proj2, "k");       // [sq, de]

    print!("{}", analyze_all(&out_abs).render());

    // Absorbed form has one fewer inner-contraction axis (no dh).
    // The carrier is still (m, ℓ, o) — identical to the standard form.
    let car_abs = carrier::derive(&out_abs, "k").expect("k is foldable");
    println!("derived carrier for k:\n{}\n", car_abs.render());

    // ── three-way comparison ──────────────────────────────────────────────────
    println!("=== Carrier comparison: MLA (std) vs MLA (absorbed) vs SHA ===\n");

    let sha_car = {
        let q = input("Q", &["sq", "d"]);
        let k = input("K", &["k", "d"]);
        let v = input("V", &["k", "e"]);
        carrier::derive(&attention(q, k, v, "d", "k"), "k").unwrap()
    };

    for (name, car) in [("SHA", &sha_car), ("MLA std", &car_std), ("MLA abs", &car_abs)] {
        println!(
            "  {name:<10}  {} slots  rules {:?}",
            car.slots, car.rules
        );
    }

    // combine and project are byte-identical across all three.
    assert_eq!(format!("{:?}", sha_car.combine), format!("{:?}", car_std.combine));
    assert_eq!(format!("{:?}", sha_car.combine), format!("{:?}", car_abs.combine));
    assert_eq!(format!("{:?}", sha_car.project), format!("{:?}", car_std.project));
    assert_eq!(format!("{:?}", sha_car.project), format!("{:?}", car_abs.project));
    println!("\n  combine and project are identical across all three — same streaming kernel ✓");

    // ── numeric verification ──────────────────────────────────────────────────
    println!("\n=== Numeric check (absorbed MLA) ===\n");

    let mut s = 0x9e3779b97f4a7c15u64;
    let mut rnd = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 11) as f64 / (1u64 << 53) as f64 * 4.0 - 2.0
    };

    // Each streaming element: (score, value) — same interface as standard attention.
    let keys: Vec<Vec<f64>> = (0..48).map(|_| vec![rnd(), rnd()]).collect();
    let derived = car_abs.fold(&keys)[0];
    let mx  = keys.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
    let den: f64 = keys.iter().map(|p| (p[0] - mx).exp()).sum();
    let num: f64 = keys.iter().map(|p| (p[0] - mx).exp() * p[1]).sum();
    println!(
        "  derived fold = {derived:.9}   naive softmax·V = {:.9}   ✓",
        num / den
    );

    // ── kernel emission ───────────────────────────────────────────────────────
    println!("\n=== Emitted streaming kernel (absorbed MLA, same as FlashAttention) ===\n");

    let report = analyze_all(&out_abs);
    let grid: Vec<&str> = report
        .axes
        .iter()
        .filter(|a| a.structure.level == Parallelism::Free)
        .map(|a| a.axis.as_str())
        .collect();
    println!("{}", rust_kernel(&car_abs, "mla_flash", "k", &grid));
}
