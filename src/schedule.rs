//! Scheduling layer — `scheduler_engine.md`, build-order steps 1–2.
//!
//! The algebraic engine (`stage1` + `carrier`) solved **legality**: every axis
//! is classified and every foldable axis has a correct-by-construction
//! accumulator of *known size* `|Acc|`. This module is the **profitability**
//! core: a device model, a hard-constraint feasibility layer, an analytic cost
//! model, and the two generic searches (inner over a tile, outer over a fusion
//! partition).
//!
//! It is *workload-agnostic*: it knows only `Device`s and `Kernel`s. How a given
//! computation decomposes into candidate kernels (their flops, traffic, working
//! set) is the caller's business — see `tests/attention_scheduling.rs` for the
//! FlashAttention fuse-vs-cut worked example built on top of these primitives.
//!
//! The defining property (§ intro): every schedule the caller hands in is already
//! legal, so the cost model only has to *order* candidates, never validate them.

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
    /// Total global-memory traffic (reads + writes). Materializing an
    /// intermediate (a fusion cut) shows up here as extra write+read.
    pub hbm_bytes: f64,
    /// Working set that must stay resident per block — includes the `|Acc|` term
    /// the algebraic engine supplies. This is the SRAM constraint.
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

// ── §6: the two generic searches ─────────────────────────────────────────────

/// §6.1 inner search — pick the cheapest feasible kernel from a family of
/// candidates (typically one per tile size). Returns the candidate and its cost.
pub fn best_tile<T>(dev: &Device, candidates: impl IntoIterator<Item = (T, Kernel)>) -> Option<(T, Kernel)> {
    candidates
        .into_iter()
        .filter(|(_, k)| feasible(dev, k))
        .min_by(|a, b| kernel_time(dev, &a.1).total_cmp(&kernel_time(dev, &b.1)))
}

/// §6.2 outer search — pick the cheapest feasible schedule (fusion partition)
/// among candidate plans. Returns its index and total cost; `None` if none fit.
pub fn cheapest(dev: &Device, plans: &[&[Kernel]]) -> Option<(usize, f64)> {
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
            name: "k".into(),
            flops,
            hbm_bytes: hbm,
            sram_per_block: sram,
            regs_per_block: sram,
            parallel_blocks: blocks,
        }
    }

    // The analytic max_tile (§4) matches a brute-force feasible-tile search.
    #[test]
    fn analytic_max_tile_matches_search() {
        let dev = Device::toy();
        let (constant, per_tile) = (40_000.0, 900.0);
        let analytic = max_tile(&dev, constant, per_tile);
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
        let small = occupancy(&dev, &kernel(1e9, 1e6, 8_000.0, 1e6));
        let big = occupancy(&dev, &kernel(1e9, 1e6, 80_000.0, 1e6));
        assert!(small > 0.0 && small <= 1.0);
        assert!(big <= small, "more SRAM per block ⇒ no higher occupancy");
    }

    // schedule_time sums feasible kernels and rejects an infeasible one.
    #[test]
    fn schedule_time_rejects_infeasible() {
        let dev = Device::toy();
        let ok = kernel(1e9, 1e6, 16_000.0, 1e4);
        let too_big = kernel(1e9, 1e6, dev.sram_bytes + 1.0, 1e4);
        assert!(schedule_time(&dev, &[ok.clone(), ok.clone()]).is_some());
        assert!(schedule_time(&dev, &[ok, too_big]).is_none());
    }

    // best_tile picks the minimum-time feasible candidate, skipping infeasible.
    #[test]
    fn best_tile_picks_cheapest_feasible() {
        let dev = Device::toy();
        // tile 4 is cheapest but infeasible; among feasible, more parallelism wins
        let cands = vec![
            (1usize, kernel(1e10, 1e6, 16_000.0, 1.0)),
            (2usize, kernel(1e10, 1e6, 16_000.0, 64.0)),
            (4usize, kernel(1e10, 1e6, dev.sram_bytes + 1.0, 1e6)),
        ];
        let (t, _) = best_tile(&dev, cands).unwrap();
        assert_eq!(t, 2);
    }

    // cheapest picks the lower-cost feasible plan and ignores infeasible ones.
    #[test]
    fn cheapest_plan_wins() {
        let dev = Device::toy();
        let cheap = vec![kernel(1e9, 1e6, 16_000.0, 1e4)];
        let dear = vec![kernel(1e12, 1e6, 16_000.0, 1e4)];
        let dead = vec![kernel(1e6, 1e6, dev.sram_bytes + 1.0, 1e4)];
        let (i, _) = cheapest(&dev, &[&dear, &cheap, &dead]).unwrap();
        assert_eq!(i, 1);
    }
}
