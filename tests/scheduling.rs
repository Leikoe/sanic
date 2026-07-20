//! The FlashAttention fuse-vs-cut decision, built as a *client* of the
//! generic cost model. Attention-specific knowledge — the flops, the traffic,
//! the cut decomposition — lives here, not in the library. The library
//! contributes the device model, feasibility, the roofline, and the two
//! searches (`best_tile`, `cheapest`). Because legality is already settled by
//! the derivation, the cost model only ranks — it can pick a slow plan, never
//! a wrong one.

use sanic::cost::*;
use sanic::kernel_ir::*;

const KT: f64 = 64.0; // K/V block streamed per step

/// The engine-supplied accumulator size: derive the attention carrier and
/// read off its per-query scalar count (m, ℓ, plus o over the value dim).
fn acc_per_lane(d: f64) -> f64 {
    let (sq, k, dd, e) = (axis("sq", 1), axis("k", 1), axis("d", 1), axis("e", 1));
    let attn = attention(
        input("Q", &[sq, dd], Dtype::F32),
        input("K", &[k, dd], Dtype::F32),
        input("V", &[k, e], Dtype::F32),
        dd,
        k,
    );
    sanic::derive::derive(&attn, k)
        .unwrap()
        .acc_scalars(|ax| if ax == e { d } else { 1.0 })
}

/// The fused flash kernel and its query tile: the inner search over
/// power-of-two tiles. Larger tiles amortize the per-block K/V re-reads
/// against SRAM / occupancy pressure.
fn fused(dev: &DeviceProfile, sq: f64, k: f64, d: f64) -> Option<(usize, Kernel)> {
    let b = dev.dtype_bytes;
    let constant = 2.0 * KT * d * b; // K/V tiles
    let per_tile = (d + acc_per_lane(d) + KT) * b; // Q tile + |Acc| + scores block

    let mut cands = Vec::new();
    let mut t = 1.0;
    while t <= sq {
        let blocks = (sq / t).ceil();
        cands.push((
            t as usize,
            Kernel {
                flops: 4.0 * sq * k * d,
                hbm_bytes: (2.0 * sq * d + blocks * 2.0 * k * d) * b, // Q,O once; K,V per block
                sram_per_block: constant + per_tile * t,
                regs_per_block: per_tile * t,
                parallel_blocks: blocks,
                lanes_per_block: t * d, // t query rows × d value outputs
            },
        ));
        t *= 2.0;
    }
    best_tile(dev, cands)
}

/// The cut: materialize the sq×k scores, then softmax·V. Two dense matmuls.
fn cut(dev: &DeviceProfile, sq: f64, k: f64, d: f64) -> Vec<Kernel> {
    let b = dev.dtype_bytes;
    let mm_sram = (64.0 * 32.0 + 32.0 * 64.0 + 64.0 * 64.0) * b;
    vec![
        Kernel {
            flops: 2.0 * sq * k * d,
            hbm_bytes: (sq * d + k * d + sq * k) * b,
            sram_per_block: mm_sram,
            regs_per_block: mm_sram,
            parallel_blocks: (sq / 64.0).ceil() * (k / 64.0).ceil(),
            lanes_per_block: 64.0 * 64.0, // one lane per output point of the tile
        },
        Kernel {
            flops: 2.0 * sq * k * d + sq * k,
            hbm_bytes: (sq * k + k * d + sq * d) * b,
            sram_per_block: mm_sram,
            regs_per_block: mm_sram,
            parallel_blocks: (sq / 64.0).ceil() * (d / 64.0).ceil(),
            lanes_per_block: 64.0 * 64.0,
        },
    ]
}

/// Does the scheduler fuse? The outer search over the two candidate plans.
fn fuses(dev: &DeviceProfile, sq: f64, k: f64, d: f64) -> bool {
    let cut_plan = cut(dev, sq, k, d);
    match fused(dev, sq, k, d) {
        Some((_, fk)) => {
            let fused_plan = std::slice::from_ref(&fk);
            cheapest(dev, &[fused_plan, &cut_plan]) == Some((0, kernel_time(dev, &fk)))
        }
        None => false, // infeasible to fuse → forced to cut
    }
}

// reproduce flash attention: at a typical head dim, fusing wins.
#[test]
fn reproduces_flash_at_typical_shape() {
    let dev = DeviceProfile::toy();
    assert!(fuses(&dev, 2048.0, 4096.0, 64.0));
}

// the real test: cut when fusion doesn't pay (large d collapses occupancy).
#[test]
fn cuts_when_fusion_does_not_pay() {
    let dev = DeviceProfile::toy();
    assert!(fuses(&dev, 2048.0, 2048.0, 32.0));
    assert!(!fuses(&dev, 2048.0, 2048.0, 512.0));
}

// the crossover is monotone in d: fuse below it, cut above, no flip-flopping.
#[test]
fn fuse_cut_crossover_is_monotone() {
    let dev = DeviceProfile::toy();
    let ds = [16.0, 32.0, 64.0, 128.0, 256.0, 512.0, 1024.0, 2048.0];
    let decisions: Vec<bool> = ds.iter().map(|&d| fuses(&dev, 2048.0, 2048.0, d)).collect();
    let first_cut = decisions.iter().position(|f| !f).unwrap();
    assert!(first_cut > 0, "must fuse at the smallest d");
    assert!(
        decisions[first_cut..].iter().all(|f| !f),
        "no flip back to fuse"
    );
}

// feasibility forces a correct fallback: huge d can't fuse at any tile → cut.
#[test]
fn infeasible_fusion_forces_cut() {
    let dev = DeviceProfile::toy();
    assert!(fused(&dev, 2048.0, 2048.0, 2048.0).is_none());
    assert!(!fuses(&dev, 2048.0, 2048.0, 2048.0));
}

// query-tiling amortizes K/V reads, so the inner search picks a real tile.
#[test]
fn fusion_chooses_a_real_query_tile() {
    let dev = DeviceProfile::toy();
    let (tile, _) = fused(&dev, 2048.0, 4096.0, 64.0).unwrap();
    assert!(tile > 1, "tiling amortizes K/V reads → tile > 1");
}

// the economic case: the cut round-trips the sq×k scores through HBM —
// exactly the traffic fusion avoids by never materializing them.
#[test]
fn cut_pays_for_materializing_the_scores() {
    let dev = DeviceProfile::toy();
    let (sq, k, d) = (2048.0, 4096.0, 64.0);
    let cut_total: f64 = cut(&dev, sq, k, d).iter().map(|c| c.hbm_bytes).sum();
    let essential = (2.0 * sq * d + 2.0 * k * d) * dev.dtype_bytes;
    let scores_roundtrip = 2.0 * sq * k * dev.dtype_bytes;
    assert!((cut_total - essential - scores_roundtrip).abs() < 1.0);
}
