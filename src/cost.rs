//! Cost: is a kernel feasible on a device, and how long does it take?
//!
//! The derivation layer settles *legality* — what can fuse, and the exact
//! accumulator. This module is the other half, *profitability*, and it knows
//! nothing about any particular computation: only `DeviceProfile`s and `Kernel`s.
//! Because every candidate handed in is already legal, the cost model only
//! has to rank — it can pick a slow plan, never a wrong one. Ranking
//! accuracy is the bar, not absolute accuracy.

/// The device the scheduler is parameterized by. All hardware numbers live
/// here; everything else is device-agnostic.
#[derive(Debug, Clone, Copy)]
pub struct DeviceProfile {
    pub sram_bytes: f64,      // shared memory / SRAM per resident block
    pub regfile_bytes: f64,   // register file per block
    pub hbm_bandwidth: f64,   // bytes / second
    pub peak_flops: f64,      // flop / second
    pub launch_overhead: f64, // seconds per kernel launch
    pub dtype_bytes: f64,
    /// Resident blocks needed to hide latency. Fewer than this and the kernel
    /// runs below peak (occupancy < 1).
    pub min_blocks: f64,
    /// Total parallel lanes (blocks × lanes-per-block) needed to keep enough
    /// loads in flight to saturate HBM. A 4-lane matvec cannot reach peak
    /// bandwidth no matter how memory-bound it is; three fat flash blocks
    /// can. This is what makes split reductions (GROUP) priceable.
    pub mem_lanes: f64,
}

impl DeviceProfile {
    /// A plausible accelerator-class toy (≈ A100 fp16, order of magnitude).
    pub fn toy() -> Self {
        DeviceProfile {
            sram_bytes: 163_840.0, // 160 KiB
            regfile_bytes: 256_000.0,
            hbm_bandwidth: 2.0e12, // 2 TB/s
            peak_flops: 3.0e14,    // 300 TFLOP/s
            launch_overhead: 3.0e-6,
            dtype_bytes: 2.0,
            min_blocks: 8.0,
            mem_lanes: 2048.0,
        }
    }

    /// The Apple M1 Pro this repo measures on (16-core GPU, f32 compute).
    /// Two constants matter for schedule choice and are grounded in this
    /// machine's own measured kernels (weights/trinity_bench.log): the f32
    /// flop peak is ~60× below `toy`'s, so recompute-heavy folds really are
    /// compute-bound here; and `mem_lanes` reflects that a 200k-lane scalar
    /// fold reaches the DRAM ceiling while a 2.3k-lane one reaches ~2% of
    /// it — saturation needs tens of thousands of scalar load streams.
    pub fn m1_pro() -> Self {
        DeviceProfile {
            sram_bytes: 32_768.0, // threadgroup memory
            regfile_bytes: 256_000.0,
            hbm_bandwidth: 2.0e11, // 200 GB/s unified
            peak_flops: 5.0e12,    // 16 cores × 128 lanes × FMA × ~1.3 GHz
            launch_overhead: 2.0e-6,
            dtype_bytes: 4.0,
            min_blocks: 32.0, // ~2 resident threadgroups per core to hide latency
            mem_lanes: 32_768.0,
        }
    }
}

/// A fully-parameterized kernel, ready to cost. A schedule is a sequence of
/// these. All decision variables (tile sizes, fusion boundary) are already
/// baked in, so costing is a pure readout.
#[derive(Debug, Clone)]
pub struct Kernel {
    pub flops: f64,
    /// Total global-memory traffic, reads + writes. Materializing an
    /// intermediate (a fusion cut) shows up here as an extra write + read.
    pub hbm_bytes: f64,
    /// Working set resident per block — includes the accumulator size the
    /// derivation supplies. This is the SRAM constraint.
    pub sram_per_block: f64,
    pub regs_per_block: f64,
    /// Independent blocks of work — the available parallelism.
    pub parallel_blocks: f64,
    /// Parallel lanes within one block (a tile's output points). Together
    /// with `parallel_blocks` this is the kernel's total memory-level
    /// parallelism — what decides whether HBM can be saturated.
    pub lanes_per_block: f64,
}

// ── feasibility ──────────────────────────────────────────────────────────────

/// A kernel is feasible iff its per-block working set fits the device.
pub fn feasible(dev: &DeviceProfile, k: &Kernel) -> bool {
    k.sram_per_block <= dev.sram_bytes && k.regs_per_block <= dev.regfile_bytes
}

// ── the cost model ───────────────────────────────────────────────────────────

/// Effective occupancy ∈ (0, 1]: how much of the machine the kernel keeps
/// busy. Bounded by how many blocks fit in SRAM / registers and by how much
/// independent work exists, normalized against the latency-hiding floor.
pub fn occupancy(dev: &DeviceProfile, k: &Kernel) -> f64 {
    let by_sram = (dev.sram_bytes / k.sram_per_block).floor();
    let by_reg = (dev.regfile_bytes / k.regs_per_block).floor();
    let resident = by_sram.min(by_reg).min(k.parallel_blocks).max(1.0);
    (resident / dev.min_blocks).clamp(1.0 / 64.0, 1.0)
}

/// How much of peak HBM bandwidth the kernel's memory-level parallelism can
/// sustain: total lanes against the device's saturation point. A fold with a
/// tiny output grid (a matvec, a huge softmax denominator) cannot keep
/// enough loads in flight — which is exactly why re-associating it into a
/// split reduction pays even when it is memory-bound.
pub fn mem_occupancy(dev: &DeviceProfile, k: &Kernel) -> f64 {
    let lanes = (k.parallel_blocks * k.lanes_per_block).max(1.0);
    (lanes / dev.mem_lanes).clamp(1.0 / 64.0, 1.0)
}

/// Per-kernel time: the roofline (compute vs. bandwidth, whichever binds)
/// plus a fixed launch cost. Compute is divided by block occupancy (latency
/// hiding needs resident warps); bandwidth by memory occupancy (saturation
/// needs loads in flight).
pub fn kernel_time(dev: &DeviceProfile, k: &Kernel) -> f64 {
    let compute = k.flops / (dev.peak_flops * occupancy(dev, k));
    let memory = k.hbm_bytes / (dev.hbm_bandwidth * mem_occupancy(dev, k));
    compute.max(memory) + dev.launch_overhead
}

/// Total time for a schedule. `None` if any kernel is infeasible — an
/// infeasible schedule has no cost, it simply cannot run.
pub fn schedule_time(dev: &DeviceProfile, kernels: &[Kernel]) -> Option<f64> {
    if kernels.iter().all(|k| feasible(dev, k)) {
        Some(kernels.iter().map(|k| kernel_time(dev, k)).sum())
    } else {
        None
    }
}

// ── the two searches ─────────────────────────────────────────────────────────

/// Inner search: the cheapest feasible kernel from a family of candidates
/// (typically one per tile size).
pub fn best_tile<T>(
    dev: &DeviceProfile,
    candidates: impl IntoIterator<Item = (T, Kernel)>,
) -> Option<(T, Kernel)> {
    candidates
        .into_iter()
        .filter(|(_, k)| feasible(dev, k))
        .min_by(|a, b| kernel_time(dev, &a.1).total_cmp(&kernel_time(dev, &b.1)))
}

/// Outer search: the cheapest feasible schedule (fusion partition) among
/// candidate plans. Returns its index and total cost; `None` if none fit.
pub fn cheapest(dev: &DeviceProfile, plans: &[&[Kernel]]) -> Option<(usize, f64)> {
    plans
        .iter()
        .enumerate()
        .filter_map(|(i, p)| schedule_time(dev, p).map(|t| (i, t)))
        .min_by(|a, b| a.1.total_cmp(&b.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kernel(flops: f64, hbm: f64, sram: f64, blocks: f64) -> Kernel {
        Kernel {
            flops,
            hbm_bytes: hbm,
            sram_per_block: sram,
            regs_per_block: sram,
            parallel_blocks: blocks,
            lanes_per_block: 4096.0, // memory-parallel unless a test says otherwise
        }
    }

    // occupancy is bounded, and a worse working set never raises it.
    #[test]
    fn occupancy_is_bounded_and_monotone() {
        let dev = DeviceProfile::toy();
        let small = occupancy(&dev, &kernel(1e9, 1e6, 8_000.0, 1e6));
        let big = occupancy(&dev, &kernel(1e9, 1e6, 80_000.0, 1e6));
        assert!(small > 0.0 && small <= 1.0);
        assert!(big <= small, "more SRAM per block ⇒ no higher occupancy");
    }

    // schedule_time sums feasible kernels and rejects an infeasible one.
    #[test]
    fn schedule_time_rejects_infeasible() {
        let dev = DeviceProfile::toy();
        let ok = kernel(1e9, 1e6, 16_000.0, 1e4);
        let too_big = kernel(1e9, 1e6, dev.sram_bytes + 1.0, 1e4);
        assert!(schedule_time(&dev, &[ok.clone(), ok.clone()]).is_some());
        assert!(schedule_time(&dev, &[ok, too_big]).is_none());
    }

    // best_tile picks the minimum-time feasible candidate.
    #[test]
    fn best_tile_picks_cheapest_feasible() {
        let dev = DeviceProfile::toy();
        // tile 4 is infeasible; among the feasible, more parallelism wins
        let cands = vec![
            (1usize, kernel(1e10, 1e6, 16_000.0, 1.0)),
            (2usize, kernel(1e10, 1e6, 16_000.0, 64.0)),
            (4usize, kernel(1e10, 1e6, dev.sram_bytes + 1.0, 1e6)),
        ];
        let (t, _) = best_tile(&dev, cands).unwrap();
        assert_eq!(t, 2);
    }

    // cheapest picks the lower-cost feasible plan and skips infeasible ones.
    #[test]
    fn cheapest_plan_wins() {
        let dev = DeviceProfile::toy();
        let cheap = vec![kernel(1e9, 1e6, 16_000.0, 1e4)];
        let dear = vec![kernel(1e12, 1e6, 16_000.0, 1e4)];
        let dead = vec![kernel(1e6, 1e6, dev.sram_bytes + 1.0, 1e4)];
        let (i, _) = cheapest(&dev, &[&dear, &cheap, &dead]).unwrap();
        assert_eq!(i, 1);
    }
}
