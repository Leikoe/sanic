//! Scheduling layer — `scheduler_engine.md`, build-order steps 1–2.
//!
//! The algebraic engine (`stage1` + `carrier`) solved **legality**: every axis
//! is classified and every foldable axis has a correct-by-construction
//! accumulator of *known size* `|Acc|`. This module solves the first half of
//! **profitability**: a synthetic device model, a hard-constraint **feasibility**
//! layer, and an analytic **cost model** that *ranks* candidate schedules.
//!
//! The defining property (§ intro): every schedule here is already legal, so the
//! cost model only has to *order* candidates, never validate them. The search
//! (steps 3–5: tile/partition/memory) is layered on top later; this slice is the
//! foundation everything else trusts, plus just enough inner tile selection to
//! make the headline acceptance call — *fuse when it pays, cut when it doesn't*.
//!
//! The accumulator size `|Acc|` in the SRAM constraint is **not** hard-coded: it
//! comes from the carrier the algebraic engine derives for the streamed axis, via
//! `Carrier::acc_scalars` — exactly the handoff the design doc prescribes.

use crate::carrier;
use crate::engine_ir::{attention, input};

/// §1.3 — the device the scheduler is parameterized by. All hardware-specific
/// numbers live here; the rest of the module is device-agnostic.
#[derive(Debug, Clone, Copy)]
pub struct Device {
    pub sram_bytes: f64,      // shared memory / SRAM available per resident block
    pub regfile_bytes: f64,   // register file per block
    pub hbm_bandwidth: f64,   // bytes / second
    pub peak_flops: f64,      // flop / second
    pub launch_overhead: f64, // seconds per kernel launch
    pub dtype_bytes: f64,
    /// Resident blocks needed to hide latency / saturate the machine. Fewer than
    /// this and the kernel runs at reduced effective throughput (occupancy < 1).
    pub min_blocks: f64,
}

impl Device {
    /// A plausible accelerator-class toy device (≈ A100 fp16, order-of-magnitude).
    pub fn toy() -> Self {
        Device {
            sram_bytes: 163_840.0, // 160 KiB
            regfile_bytes: 256_000.0,
            hbm_bandwidth: 2.0e12, // 2 TB/s
            peak_flops: 3.0e14,    // 300 TFLOP/s
            launch_overhead: 3.0e-6,
            dtype_bytes: 2.0,
            min_blocks: 8.0,
        }
    }
}

/// A fully-parameterized kernel, ready to cost. A schedule is a sequence of
/// these (the fusion partition). Everything is expressed as a function of the
/// decision variables already baked in (tile sizes, fusion boundary), so the
/// cost model is a pure readout.
#[derive(Debug, Clone)]
pub struct Kernel {
    pub name: String,
    pub flops: f64,
    /// Total global-memory traffic (reads + writes). A cut materializes its
    /// intermediate here — the extra write+read is the economic case for fusion.
    pub hbm_bytes: f64,
    /// Working set that must stay resident per block — *includes the `|Acc|`
    /// term* supplied by the algebraic engine. This is the SRAM constraint.
    pub sram_per_block: f64,
    pub regs_per_block: f64,
    /// Count of independent blocks of work (the available parallelism).
    pub parallel_blocks: f64,
}

// ── §4: feasibility (hard constraints) ───────────────────────────────────────

/// A kernel is feasible iff its per-block working set fits the device.
pub fn feasible(dev: &Device, k: &Kernel) -> bool {
    k.sram_per_block <= dev.sram_bytes && k.regs_per_block <= dev.regfile_bytes
}

/// Analytic max tile along an axis: given a working set `constant + per_tile · t`,
/// the largest `t` that still fits SRAM. Seeds the tile search without probing.
pub fn max_tile(dev: &Device, constant: f64, per_tile: f64) -> i64 {
    if per_tile <= 0.0 {
        return if constant <= dev.sram_bytes { i64::MAX } else { 0 };
    }
    ((dev.sram_bytes - constant) / per_tile).floor() as i64
}

// ── §5: the cost model (profitability) ───────────────────────────────────────

/// Effective occupancy ∈ (0, 1]: how much of the machine this kernel can keep
/// busy. Bounded by how many blocks fit in SRAM / registers, and by how much
/// independent work exists; normalized against the latency-hiding floor.
pub fn occupancy(dev: &Device, k: &Kernel) -> f64 {
    let by_sram = (dev.sram_bytes / k.sram_per_block).floor();
    let by_reg = (dev.regfile_bytes / k.regs_per_block).floor();
    let resident = by_sram.min(by_reg).min(k.parallel_blocks).max(1.0);
    (resident / dev.min_blocks).clamp(1.0 / 64.0, 1.0)
}

/// Per-kernel time: the roofline (compute vs. bandwidth, whichever bounds) plus
/// a fixed launch cost. Compute is divided by occupancy — an under-occupied
/// kernel cannot reach peak. Ranking accuracy is the bar, not absolute accuracy.
pub fn kernel_time(dev: &Device, k: &Kernel) -> f64 {
    let compute = k.flops / (dev.peak_flops * occupancy(dev, k));
    let memory = k.hbm_bytes / dev.hbm_bandwidth;
    compute.max(memory) + dev.launch_overhead
}

/// Total time for a schedule (a partition into kernels). `None` if any kernel is
/// infeasible — an infeasible schedule has no cost, it simply cannot run.
pub fn schedule_time(dev: &Device, kernels: &[Kernel]) -> Option<f64> {
    if kernels.iter().all(|k| feasible(dev, k)) {
        Some(kernels.iter().map(|k| kernel_time(dev, k)).sum())
    } else {
        None
    }
}

// ── attention scenario: the fuse-vs-cut decision (§8 acceptance) ─────────────
//
// Problem: `sq` queries attend over `k` keys with head dim `d`. The algebraic
// engine derives the flash accumulator `(m, ℓ, o[d])` for the streamed `k` axis,
// and `Carrier::acc_scalars` reports its per-query size — the `|Acc|` term below
// is read off that carrier, not written here.
//
//   Fused (flash): one kernel, streams k, never materializes the sq×k scores.
//                  But Q-tile + Acc + K/V tile must co-reside in SRAM, so large d
//                  shrinks the feasible tile → occupancy collapses.
//   Cut:           two matmuls with the sq×k scores materialized to HBM. Each is
//                  a dense, high-occupancy matmul, paying extra traffic + a launch.
//
// Small d → fuse (avoids scores, plenty of occupancy). Large d → the fused
// kernel's SRAM/occupancy penalty exceeds the materialization it avoids → cut.

const KT: f64 = 64.0; // K/V block streamed per step

/// Best feasible fused-attention kernel and its query-tile, choosing the sq-tile
/// that minimizes time (the inner tile search of §6.1, a power-of-two scan).
pub fn fused_attention(dev: &Device, sq: f64, k: f64, d: f64) -> Option<(Kernel, usize)> {
    let b = dev.dtype_bytes;

    // |Acc| comes from the engine, not a constant: derive the carrier for the
    // streamed key axis and ask it how many scalars it holds per query. For
    // FlashAttention that is m, ℓ (per query) + o (one per value-feature `e`),
    // i.e. 2 + d — but the number is read off the derived `(m, ℓ, o)` carrier.
    let attn = attention(
        input("Q", &["sq", "d"]),
        input("K", &["k", "d"]),
        input("V", &["k", "e"]),
        "d",
        "k",
    );
    let acc_per_query = carrier::derive(&attn, "k")?.acc_scalars(|ax| match ax {
        "e" => d, // the value-feature axis has extent d here
        _ => 1.0, // m, ℓ are one scalar per query
    });

    // working set per query-tile of size t:
    //   Q tile (t·d) + Acc (t·acc_per_query) + K,V tiles (2·KT·d) + scores (t·KT)
    let constant = 2.0 * KT * d * b;
    let per_tile = (d + acc_per_query + KT) * b;

    let mut best: Option<(Kernel, usize)> = None;
    let mut t = 1.0;
    while t <= sq {
        let sram = constant + per_tile * t;
        if sram <= dev.sram_bytes {
            // K and V are re-read once per query block, so a bigger tile amortizes
            // them — this is *why* you tile queries. Q in / O out are read once.
            let blocks = (sq / t).ceil();
            let kern = Kernel {
                name: format!("flash(sq_tile={})", t as u64),
                flops: 4.0 * sq * k * d, // QKᵀ + AV
                hbm_bytes: (2.0 * sq * d + blocks * 2.0 * k * d) * b, // Q,O once; K,V per block
                sram_per_block: sram,
                regs_per_block: per_tile * t,
                parallel_blocks: blocks,
            };
            let better = match &best {
                None => true,
                Some((bk, _)) => kernel_time(dev, &kern) < kernel_time(dev, bk),
            };
            if better {
                best = Some((kern, t as usize));
            }
        }
        t *= 2.0;
    }
    best
}

/// The two-kernel cut: materialize scores, then softmax+AV. Standard dense
/// matmuls with modest, near-constant SRAM → high occupancy.
pub fn cut_attention(dev: &Device, sq: f64, k: f64, d: f64) -> [Kernel; 2] {
    let b = dev.dtype_bytes;
    // a 64×64 matmul tile with 32-deep inner accumulation (operands + C tile)
    let mm_sram = (64.0 * 32.0 + 32.0 * 64.0 + 64.0 * 64.0) * b;

    let scores = Kernel {
        name: "scores = QKᵀ".into(),
        flops: 2.0 * sq * k * d,
        hbm_bytes: (sq * d + k * d + sq * k) * b, // read Q,K; write S
        sram_per_block: mm_sram,
        regs_per_block: mm_sram,
        parallel_blocks: (sq / 64.0).ceil() * (k / 64.0).ceil(),
    };
    let av = Kernel {
        name: "out = softmax(S)·V".into(),
        flops: 2.0 * sq * k * d + sq * k, // matmul + cheap softmax
        hbm_bytes: (sq * k + k * d + sq * d) * b, // read S,V; write O
        sram_per_block: mm_sram,
        regs_per_block: mm_sram,
        parallel_blocks: (sq / 64.0).ceil() * (d / 64.0).ceil(),
    };
    [scores, av]
}

/// The scheduler's fuse-vs-cut verdict for an attention problem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    Fuse,
    Cut,
}

#[derive(Debug, Clone)]
pub struct Verdict {
    pub decision: Decision,
    pub fused_time: Option<f64>,
    pub cut_time: f64,
    /// The query-tile the fused plan would use (its resident lanes) — what
    /// codegen blocks the kernel by. `None` when fusion is infeasible.
    pub tile: Option<usize>,
}

/// Choose the cheaper feasible plan. If the fused kernel is infeasible at this
/// shape, the engine is *forced* to cut — and that is still a correct program.
pub fn schedule_attention(dev: &Device, sq: f64, k: f64, d: f64) -> Verdict {
    let fused = fused_attention(dev, sq, k, d);
    let fused_time = fused.as_ref().map(|(kern, _)| kernel_time(dev, kern));
    let cut_time = schedule_time(dev, &cut_attention(dev, sq, k, d)).unwrap();
    let decision = match fused_time {
        Some(ft) if ft <= cut_time => Decision::Fuse,
        _ => Decision::Cut,
    };
    Verdict {
        decision,
        fused_time,
        cut_time,
        tile: fused.map(|(_, t)| t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // §8: reproduce flash attention — at a typical head dim the scheduler fuses,
    // streaming k and keeping the (m, ℓ, o) accumulator in SRAM, derived from the
    // cost model rather than a stored template.
    #[test]
    fn reproduces_flash_at_typical_shape() {
        let dev = Device::toy();
        let v = schedule_attention(&dev, 2048.0, 4096.0, 64.0);
        assert_eq!(v.decision, Decision::Fuse);
        assert!(v.fused_time.unwrap() < v.cut_time);
    }

    // §8: the real test — make the right cut/fuse call where fusion does NOT pay.
    // As d grows, the fused kernel's SRAM pressure collapses occupancy until the
    // occupancy penalty exceeds the scores materialization it avoids → cut.
    #[test]
    fn cuts_when_fusion_does_not_pay() {
        let dev = Device::toy();
        assert_eq!(schedule_attention(&dev, 2048.0, 2048.0, 32.0).decision, Decision::Fuse);
        assert_eq!(schedule_attention(&dev, 2048.0, 2048.0, 512.0).decision, Decision::Cut);
    }

    // The crossover is monotone: there is a single d below which we fuse and above
    // which we cut (no flip-flopping) — a sane, rankable cost surface.
    #[test]
    fn fuse_cut_crossover_is_monotone() {
        let dev = Device::toy();
        let ds = [16.0, 32.0, 64.0, 128.0, 256.0, 512.0, 1024.0, 2048.0];
        let decisions: Vec<_> = ds
            .iter()
            .map(|&d| schedule_attention(&dev, 2048.0, 2048.0, d).decision)
            .collect();
        let first_cut = decisions.iter().position(|x| *x == Decision::Cut).unwrap();
        assert!(first_cut > 0, "must fuse at the smallest d");
        // once it cuts, it never goes back to fusing
        assert!(decisions[first_cut..].iter().all(|x| *x == Decision::Cut));
    }

    // Feasibility forces a correct fallback: at a huge head dim the fused kernel
    // cannot fit SRAM even at tile=1, so the engine is *forced* to cut — and that
    // is still a legal program (the whole point: it can only be slow, not wrong).
    #[test]
    fn infeasible_fusion_forces_cut() {
        let dev = Device::toy();
        assert!(fused_attention(&dev, 2048.0, 2048.0, 2048.0).is_none());
        let v = schedule_attention(&dev, 2048.0, 2048.0, 2048.0);
        assert_eq!(v.decision, Decision::Cut);
        assert!(v.fused_time.is_none());
        assert!(v.cut_time.is_finite());
    }

    // The economic case for fusion: the cut pays to round-trip the sq×k scores
    // through HBM — exactly the traffic fusion avoids by never materializing them.
    #[test]
    fn cut_pays_for_materializing_the_scores() {
        let dev = Device::toy();
        let (sq, k, d) = (2048.0, 4096.0, 64.0);
        let cut_total: f64 = cut_attention(&dev, sq, k, d).iter().map(|c| c.hbm_bytes).sum();
        let essential = (2.0 * sq * d + 2.0 * k * d) * dev.dtype_bytes; // inputs once + output once
        let scores_roundtrip = 2.0 * sq * k * dev.dtype_bytes;
        assert!((cut_total - essential - scores_roundtrip).abs() < 1.0);
    }

    // Query-tiling exists to amortize K/V reads, so the inner search picks a real
    // tile > 1 — the number codegen then blocks the kernel by.
    #[test]
    fn fusion_chooses_a_real_query_tile() {
        let dev = Device::toy();
        let v = schedule_attention(&dev, 2048.0, 4096.0, 64.0);
        assert_eq!(v.decision, Decision::Fuse);
        assert!(v.tile.unwrap() > 1, "tiling amortizes K/V reads → tile > 1");
    }

    // The analytic max_tile (§4) matches a brute-force feasible-tile search — the
    // seed the inner search relies on is exact.
    #[test]
    fn analytic_max_tile_matches_search() {
        let dev = Device::toy();
        let (constant, per_tile) = (40_000.0, 900.0);
        let analytic = max_tile(&dev, constant, per_tile);
        // brute force the largest t with constant + per_tile·t ≤ sram
        let mut brute = 0i64;
        let mut t = 1i64;
        while constant + per_tile * t as f64 <= dev.sram_bytes {
            brute = t;
            t += 1;
        }
        assert_eq!(analytic, brute);
    }

    // occupancy is bounded and a strictly worse working set never raises it.
    #[test]
    fn occupancy_is_bounded_and_monotone() {
        let dev = Device::toy();
        let mk = |sram: f64| Kernel {
            name: "k".into(),
            flops: 1e9,
            hbm_bytes: 1e6,
            sram_per_block: sram,
            regs_per_block: sram,
            parallel_blocks: 1e6,
        };
        let small = occupancy(&dev, &mk(8_000.0));
        let big = occupancy(&dev, &mk(80_000.0));
        assert!(small > 0.0 && small <= 1.0);
        assert!(big <= small, "more SRAM per block ⇒ no higher occupancy");
    }
}
