//! Planning: pick the axis to stream, the axes to grid, and a block that fits.
//!
//! Nothing here knows what computation it is scheduling. Every decision is
//! read off information the other layers already expose: the carrier's slot
//! spans say which axes are candidates for each block role; the IR says which
//! inputs carry the streamed axis; the cost model ranks the candidates.

use std::collections::{HashMap, HashSet};

use crate::analyze::{Parallelism, analyze_all};
use crate::cost::{DeviceProfile, Kernel, feasible, kernel_time, schedule_time};
use crate::derive::{Carrier, Expr, SlotKind};
use crate::kernel_ir::{
    Axis, Dtype, Node as NodeKind, NodeRef as Node, all_axes, input_axes, input_dtypes, leaf_names,
};

/// Elements streamed per step along the folded axis.
pub const TILE_N: usize = 64;

/// A fully-resolved kernel: everything an emitter needs.
#[derive(Debug, Clone)]
pub struct KernelSpec {
    pub name: String,
    /// The axis this kernel folds over.
    pub streaming_axis: Axis,
    /// Axes contracted internally (in the inputs, but neither streamed nor in
    /// the output).
    pub contract_axes: Vec<Axis>,
    /// The axis tiled across SRAM rows per block. `None` means the output is
    /// scalar-ish — the row tile is always 1.
    pub row_axis: Option<Axis>,
    /// Axes spanned only by some accumulator slots (e.g. the value head dim).
    pub col_axes: Vec<Axis>,
    /// Axes handled as grid-level parallelism, not tiled in SRAM.
    pub batch_axes: Vec<Axis>,
    pub carrier: Carrier,
    pub tile_m: usize,
    pub tile_n: usize,
    /// A second tiled block dimension, when the planner chose one (the 2D
    /// GEMM block). `None` / 1 when the block is one-dimensional.
    pub col_tile_axis: Option<Axis>,
    pub tile_c: usize,
    pub input_names: Vec<&'static str>,
    pub output_name: String,
    pub cost: f64,
    /// The winning roofline instance behind `cost` — what the block structure
    /// actually costs in flops/traffic/SRAM/parallelism. Downstream searches
    /// (the split-reduction factor) reprice variations of THIS kernel instead
    /// of inventing a second, incomparable model.
    pub roofline: Kernel,
}

/// A schedule: kernels in execution order, with a total cost. Currently the
/// planner always produces a single fused kernel; splitting into pipelines is
/// a natural extension, not a done one.
pub struct Plan {
    pub kernels: Vec<KernelSpec>,
    pub total_cost: f64,
}

/// Plan the whole graph: try every axis that folds, keep the cheapest kernel.
pub fn plan(node: &Node, dev: &DeviceProfile) -> Option<Plan> {
    let report = analyze_all(node);

    let mut best: Option<KernelSpec> = None;
    for axis_report in &report.axes {
        if axis_report.structure.level != Parallelism::Monoidal {
            continue;
        }
        let Some(ref carrier) = axis_report.carrier else {
            continue;
        };
        let Some(spec) = plan_axis(node, axis_report.axis, carrier, dev) else {
            continue;
        };
        if best.as_ref().is_none_or(|b| spec.cost < b.cost) {
            best = Some(spec);
        }
    }

    let spec = best?;
    let cost = spec.cost;
    Some(Plan {
        kernels: vec![spec],
        total_cost: cost,
    })
}

/// Choose the cheapest feasible block structure for streaming `node` over one
/// axis. `None` if nothing fits the device.
pub fn plan_axis(
    node: &Node,
    streaming_axis: Axis,
    carrier: &Carrier,
    dev: &DeviceProfile,
) -> Option<KernelSpec> {
    let b_bytes = dev.dtype_bytes;
    let tn = TILE_N as f64;

    // ── axis roles, read off the carrier and the IR ──────────────────────────
    let out_axes = node.shape();
    let out_set: HashSet<Axis> = out_axes.iter().copied().collect();

    // Contract axes: in the graph, but neither streamed nor in the output.
    let contract_axes: Vec<Axis> = all_axes(node)
        .into_iter()
        .filter(|&ax| ax != streaming_axis && !out_set.contains(&ax))
        .collect();

    // Each slot spans some free axes per output point. Axes shared by every
    // slot are row/batch/col-tile candidates; axes on only some slots are
    // per-slot columns (attention: m, ℓ span {sq}; o spans {sq, e} → shared
    // sq, column e).
    let span_union: HashSet<Axis> = carrier
        .spans
        .iter()
        .flat_map(|s| s.iter().copied())
        .collect();
    let span_intersection: HashSet<Axis> = {
        let mut inter: HashSet<Axis> = span_union.clone();
        for s in &carrier.spans {
            let s_set: HashSet<Axis> = s.iter().copied().collect();
            inter.retain(|ax| s_set.contains(ax));
        }
        inter
    };
    let col_axes: Vec<Axis> = {
        let mut v: Vec<Axis> = span_union
            .iter()
            .filter(|ax| !span_intersection.contains(ax))
            .copied()
            .collect();
        v.sort();
        v
    };
    let span_schedulable: Vec<Axis> = {
        // A deferred divisor is intentionally kept whole: tiling an axis
        // absent from its normalizer slot would recompute that normalization
        // once per tile (the giant-vocabulary RMSNorm head). Let partitioning
        // cut the divisor instead. Other partial-span slots, including the
        // structural slot introduced by a flattened attention projection,
        // are cheap and legal to duplicate across tiles.
        let source = if carrier.rules.contains(&"defer-div") {
            &span_intersection
        } else {
            &span_union
        };
        let mut v: Vec<Axis> = source.iter().copied().collect();
        v.sort();
        v
    };

    // ── classify inputs ──────────────────────────────────────────────────────
    // Dedup by name: a DAG can reach the same tensor along several paths
    // (softmax forks its scores), but it is physically one tensor. Consts and
    // iotas never appear here — they cost nothing to read.
    let inputs: Vec<(&'static str, Vec<Axis>)> = {
        let mut seen = HashSet::new();
        input_axes(node)
            .into_iter()
            .filter(|(n, _)| seen.insert(*n))
            .collect()
    };
    // Per-input storage bytes. Every input declares its representation, so
    // int8/int4 weights earn their bandwidth win without a device-dependent
    // fallback.
    let declared: HashMap<&'static str, f64> = input_dtypes(node)
        .into_iter()
        .map(|(n, d)| (n, d.bytes()))
        .collect();
    let in_bytes = |name: &'static str| {
        declared
            .get(name)
            .copied()
            .expect("every input must declare a storage dtype")
    };

    // An inner contraction (matmul inside the fold) keeps a scores-like
    // intermediate tile resident (tile_m × tile_c × TILE_N).
    let has_inner_contraction = has_contraction(node, streaming_axis, &out_set);

    let output_vol: f64 = out_axes.iter().map(|ax| ax.extent() as f64).product();
    let total_flops = count_flops(node);

    // ── structure choice: no heuristics — price every assignment ────────────
    // Each shared-span axis plays one of four roles, and each has a real
    // consequence the model can price:
    //   row      — tiled in SRAM (tile_m); inputs lacking it are re-read
    //              once per row block (the FlashAttention K/V trade);
    //   col      — a second tiled dimension (tile_c); with `row` this is the
    //              classic 2D GEMM block, trading the two re-read factors;
    //   batch    — a grid dimension; inputs lacking it are re-read per
    //              instance (a mask per head);
    //   resident — held whole in the block; costs SRAM instead of traffic.
    // The space is tiny (a handful of axes × ~10 tile sizes each), so
    // enumerate everything and let the roofline rank. Iteration order breaks
    // cost ties toward simple structures: large row axes first, no col tile
    // first, most batching first.
    let ext = |ax: Axis| ax.extent() as f64;
    let pows = |limit: f64| {
        let mut v = Vec::new();
        let mut t = 1.0f64;
        while t <= limit {
            v.push(t);
            t *= 2.0;
        }
        v
    };
    // An axis need not span every accumulator slot to be tiled. Flash
    // attention's value dimension, and a positional reorder's zero-valued
    // ordering slot, both span only a subset. Tiling such an axis merely
    // recomputes the invariant slots in each tile; the issue and traffic
    // models already price that duplication. Requiring the intersection here
    // made otherwise ordinary projections impossible when one invariant slot
    // accompanied their accumulator.
    let mut by_extent: Vec<Axis> = span_schedulable.clone();
    by_extent.sort_by(|&a, &b| ext(b).total_cmp(&ext(a)));

    struct Best {
        row: Option<Axis>,
        col: Option<Axis>,
        batch: Vec<Axis>,
        tile_m: usize,
        tile_c: usize,
        cost: f64,
        kernel: Kernel,
    }
    let mut best: Option<Best> = None;

    let row_options: Vec<Option<Axis>> = by_extent.iter().map(|&a| Some(a)).chain([None]).collect();
    for &row in &row_options {
        let col_options: Vec<Option<Axis>> = [None]
            .into_iter()
            .chain(
                by_extent
                    .iter()
                    .filter(|&&a| Some(a) != row)
                    .map(|&a| Some(a)),
            )
            .collect();
        for &col in &col_options {
            let rest: Vec<Axis> = span_schedulable
                .iter()
                .copied()
                .filter(|&a| Some(a) != row && Some(a) != col)
                .collect();
            // descending bit patterns: all-batch first
            for bits in (0..(1u32 << rest.len().min(8))).rev() {
                let batch: Vec<Axis> = rest
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| bits >> i & 1 == 1)
                    .map(|(_, a)| *a)
                    .collect();
                // (everything in `rest` not chosen as batch stays resident)
                let batch_volume: f64 = batch.iter().map(|&ax| ext(ax)).product();

                // What one block holds of a tensor spanning `axes`, given the
                // tile sizes: the streamed axis contributes a TILE_N slab, the
                // row/col axes their tiles, batch axes nothing, and resident
                // axes their full extent.
                let per_block = |axes: &[Axis], tm: f64, tc: f64| -> f64 {
                    axes.iter()
                        .map(|&ax| {
                            if ax == streaming_axis {
                                tn
                            } else if Some(ax) == row {
                                tm
                            } else if Some(ax) == col {
                                tc
                            } else if batch.contains(&ax) {
                                1.0
                            } else {
                                ext(ax)
                            }
                        })
                        .product()
                };

                let tms = pows(row.map_or(1.0, ext));
                let tcs = pows(col.map_or(1.0, ext));
                for &tm in &tms {
                    for &tc in &tcs {
                        // SRAM: input slabs (at their storage width) + the
                        // accumulator + the inner-contraction intermediate.
                        let mut sram = 0.0f64;
                        for (nm, axes) in &inputs {
                            sram += per_block(axes, tm, tc) * in_bytes(nm);
                        }
                        sram += carrier.acc_scalars(|ax| {
                            if Some(ax) == row {
                                tm
                            } else if Some(ax) == col {
                                tc
                            } else if batch.contains(&ax) {
                                1.0
                            } else {
                                ext(ax)
                            }
                        }) * b_bytes;
                        if has_inner_contraction {
                            sram += tm * tc * tn * b_bytes;
                        }

                        // HBM: every input is re-read once per grid instance
                        // it does not carry — row blocks, col blocks, batch —
                        // each moving its own storage width.
                        let row_blocks = row.map_or(1.0, |r| (ext(r) / tm).ceil());
                        let col_blocks = col.map_or(1.0, |c| (ext(c) / tc).ceil());
                        let mut hbm = output_vol * b_bytes;
                        for (nm, axes) in &inputs {
                            let vol: f64 = axes.iter().map(|&ax| ext(ax)).product();
                            let mut factor = 1.0;
                            if let Some(r) = row
                                && !axes.contains(&r)
                            {
                                factor *= row_blocks;
                            }
                            if let Some(c) = col
                                && !axes.contains(&c)
                            {
                                factor *= col_blocks;
                            }
                            for &b in &batch {
                                if !axes.contains(&b) {
                                    factor *= ext(b);
                                }
                            }
                            hbm += vol * factor * in_bytes(nm);
                        }

                        // one lane per output point of the block: the tile
                        // area times the col span held resident in it
                        let resident_cols: f64 = col_axes
                            .iter()
                            .filter(|&&ax| {
                                Some(ax) != row && Some(ax) != col && !batch.contains(&ax)
                            })
                            .map(|&ax| ext(ax))
                            .product();
                        let k = Kernel {
                            name: format!("fused(tile={tm}x{tc})"),
                            flops: total_flops,
                            hbm_bytes: hbm,
                            sram_per_block: sram,
                            // Registers are not modeled separately; reuse SRAM.
                            regs_per_block: sram,
                            parallel_blocks: batch_volume * row_blocks * col_blocks,
                            lanes_per_block: tm * tc * resident_cols,
                        };
                        if !feasible(dev, &k) {
                            continue;
                        }
                        let cost = kernel_time(dev, &k);
                        if best.as_ref().is_none_or(|b| cost < b.cost) {
                            best = Some(Best {
                                row,
                                col,
                                batch: batch.clone(),
                                tile_m: tm as usize,
                                tile_c: tc as usize,
                                cost,
                                kernel: k,
                            });
                        }
                    }
                }
            }
        }
    }

    let Best {
        row: row_axis,
        col: col_tile_axis,
        batch: batch_axes,
        tile_m,
        tile_c,
        cost,
        kernel,
    } = best?;

    Some(KernelSpec {
        name: "fused".to_string(),
        streaming_axis,
        contract_axes,
        row_axis,
        col_axes,
        batch_axes,
        carrier: carrier.clone(),
        tile_m,
        tile_n: TILE_N,
        col_tile_axis,
        tile_c,
        input_names: leaf_names(node),
        output_name: "Out".to_string(),
        cost,
        roofline: kernel,
    })
}

/// Should this fold run as a TWO-STAGE split reduction (GROUP), and in how
/// many blocks? Price every factor with the same roofline that ranks tiles:
/// stage 1 folds `blocks` chunks in parallel and writes `grid·blocks·slots`
/// raw partials; stage 2 reads them back, merges, projects. Splitting wins
/// exactly when the one-pass kernel cannot occupy the device — a matvec's
/// grid of 1 or a giant softmax denominator leaves it latency-bound, and
/// re-associating the fold (legal by the monoid law) buys `blocks`-way
/// parallelism for the price of one small round trip. Returns the winning
/// factor, or `None` when one pass is already cheapest.
pub fn split_factor(
    node: &Node,
    streaming_axis: Axis,
    carrier: &Carrier,
    dev: &DeviceProfile,
) -> Option<usize> {
    let spec = plan_axis(node, streaming_axis, carrier, dev)?;
    let single = spec.cost;
    let bytes = dev.dtype_bytes;
    let grid_vol: f64 = node.shape().iter().map(|ax| ax.extent() as f64).product();
    let slots = carrier.slots as f64;
    let n = streaming_axis.extent() as f64;

    let mut best: Option<(usize, f64)> = None;
    let mut b = 2usize;
    while (b as f64) <= n && b <= 4096 {
        let bf = b as f64;
        let partials = grid_vol * bf * slots;
        // Stage 1 is THE SAME kernel the single-pass plan chose — same tiles,
        // same traffic, same working set — with two differences the split
        // buys/pays for: `b`-way more block parallelism, and the partials
        // written out instead of projected in registers.
        let mut stage1 = spec.roofline.clone();
        stage1.name = format!("partial(×{b})");
        stage1.parallel_blocks *= bf;
        stage1.hbm_bytes += partials * bytes;
        // Stage 2 reads the partials back and merges them per output point.
        let stage2 = Kernel {
            name: "combine".to_string(),
            flops: partials,
            hbm_bytes: (partials + grid_vol) * bytes,
            sram_per_block: slots * bytes,
            regs_per_block: slots * bytes,
            parallel_blocks: grid_vol,
            lanes_per_block: 1.0,
        };
        if let Some(t) = schedule_time(dev, &[stage1, stage2])
            && best.as_ref().is_none_or(|(_, bt)| t < *bt)
        {
            best = Some((b, t));
        }
        b *= 2;
    }
    best.filter(|(_, t)| *t < single).map(|(b, _)| b)
}

// ── intra-kernel fold schedules ──────────────────────────────────────────────

/// How a fused fold maps onto the GPU's execution hierarchy. This is the
/// intra-kernel form of the GROUP split: the SAME re-association law
/// (`run_carrier_split`), with the partials merged through simd shuffles or
/// threadgroup memory instead of a buffer round trip. Nothing here is
/// computation-specific — the descriptor names axes and split factors, and
/// the emitter renders the carrier's own `combine` as the merge at each
/// level. The all-trivial value is today's one-thread-per-output kernel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FoldSched {
    /// An output axis distributed across the simd lanes of every simdgroup
    /// (extent must be a multiple of the simd width). Slots whose span
    /// lacks this axis are computed once per simdgroup instead of once per
    /// output point — the generic form of "the attention score does not
    /// depend on the value head dim", read off `Carrier::spans`, and the
    /// license for lane-splitting in-body contractions.
    pub lane_axis: Option<Axis>,
    /// Split the streamed axis across this many simdgroups per threadgroup
    /// (1 = no split); partial carriers merge in threadgroup memory.
    pub sgs: usize,
    /// Split the streamed axis across the simd lanes as well (exclusive
    /// with `lane_axis`); partials merge over simd shuffles.
    pub lane_stream: bool,
    /// With `lane_stream`, each lane folds CONTIGUOUS runs of this many
    /// streamed elements (1 = element-strided). Legal for any mergeable
    /// carrier — chunking only re-associates — and chosen when a packed
    /// (int4) leaf makes contiguity pay: consecutive nibbles share bytes
    /// and cache lines, and the unrolled run's index arithmetic constant-
    /// folds instead of re-deriving per element.
    pub chunk: usize,
}

/// The GPU simd width the scheduled emitter targets (Apple GPUs).
pub const SIMD: usize = 32;

impl FoldSched {
    pub fn scalar() -> Self {
        FoldSched {
            lane_axis: None,
            sgs: 1,
            lane_stream: false,
            chunk: 1,
        }
    }
    pub fn is_scalar(&self) -> bool {
        *self == Self::scalar()
    }
    /// Threads per threadgroup this schedule needs.
    pub fn tg_threads(&self) -> usize {
        if self.is_scalar() { 1 } else { SIMD * self.sgs }
    }
}

/// Can this carrier's partial states be merged out of stream order? Every
/// `Monoid` is commutative and the ExpShifted rescale merge is symmetric,
/// so both split freely. ArgIdx ties break by FIRST position and the k-best
/// combine is a singleton insert, not a two-list merge — an interleaved
/// lane/simdgroup partition would change their meaning, so they decline
/// (the same rule `emit_split_metal` enforces); AffineStep is sequential.
pub fn mergeable_out_of_order(carrier: &Carrier) -> bool {
    carrier
        .kinds
        .iter()
        .all(|k| matches!(k, SlotKind::Plain(_) | SlotKind::ExpShifted { .. }))
}

/// Operation count of a carrier expression (for pricing slot updates).
fn expr_ops(e: &Expr) -> f64 {
    match e {
        Expr::Const(_) | Expr::Item(_) | Expr::A(_) | Expr::B(_) | Expr::F(_) => 0.0,
        Expr::Add(a, b)
        | Expr::Sub(a, b)
        | Expr::Mul(a, b)
        | Expr::Div(a, b)
        | Expr::Max(a, b)
        | Expr::Min(a, b)
        | Expr::Lt(a, b) => 1.0 + expr_ops(a) + expr_ops(b),
        Expr::Exp(a) | Expr::Log(a) | Expr::Sqrt(a) | Expr::Sin(a) | Expr::Cos(a) => {
            1.0 + expr_ops(a)
        }
        Expr::Where(c, a, b) => 1.0 + expr_ops(c) + expr_ops(a) + expr_ops(b),
    }
}

/// Per-element ISSUE cost of evaluating a node the way the scalar emitter
/// emits it — loads, index arithmetic (a flatten is a div/mod chain per
/// member, a gather a rounded index), and arithmetic all cost slots, not
/// just the `count_flops` op. Schedule choice hinges on recompute, and
/// recompute is paid in issue slots; pricing it at one flop per element is
/// what made one-thread-per-output look cheap.
fn count_issue_ops(node: &Node) -> f64 {
    let vol = |n: &Node| -> f64 { n.shape().iter().map(|ax| ax.extent() as f64).product() };
    match node.as_ref() {
        NodeKind::Const { .. } => 0.0,
        NodeKind::Iota { .. } => vol(node),
        NodeKind::Input { axes, .. } => (1.0 + axes.len() as f64) * vol(node),
        NodeKind::Map { inputs, .. } => {
            inputs.iter().map(|i| count_issue_ops(i)).sum::<f64>() + vol(node)
        }
        NodeKind::Reduce { src, .. } | NodeKind::Scan { src, .. } => {
            count_issue_ops(src) + vol(src)
        }
        NodeKind::Gather { src, index, .. } => {
            count_issue_ops(src) + count_issue_ops(index) + 2.0 * vol(node)
        }
        NodeKind::View { src, groups } => {
            let split_ops: f64 = groups
                .iter()
                .map(|(m, _)| {
                    if m.len() > 1 {
                        2.0 * m.len() as f64
                    } else {
                        0.0
                    }
                })
                .sum();
            count_issue_ops(src) + split_ops * vol(node)
        }
        NodeKind::Reindex { src, map, padded } => {
            let per = map.len() as f64 * 2.0 + if *padded { 2.0 } else { 0.0 };
            count_issue_ops(src) + per * vol(node)
        }
    }
}

/// Does a subtree contain a lane-splittable contraction (a `Reduce` whose
/// extent is a simd multiple)? Such a leaf's work divides across the lanes
/// instead of being issued redundantly by each.
fn has_simd_reduce(node: &Node) -> bool {
    match node.as_ref() {
        NodeKind::Reduce { src, axis, .. } => axis.extent() % SIMD == 0 || has_simd_reduce(src),
        NodeKind::Map { inputs, .. } => inputs.iter().any(|i| has_simd_reduce(i)),
        NodeKind::Gather { src, index, .. } => has_simd_reduce(src) || has_simd_reduce(index),
        NodeKind::View { src, .. } | NodeKind::Reindex { src, .. } | NodeKind::Scan { src, .. } => {
            has_simd_reduce(src)
        }
        NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => false,
    }
}

/// Pick the fold schedule for one fused kernel by pricing every candidate
/// with the same roofline that ranks tiles and split factors. The decisive
/// term is EXECUTION flops — what each schedule actually issues, recompute
/// included: one thread per output point re-issues every span-invariant
/// leaf per point (the measured disease), a lane-distributed axis issues it
/// once per simdgroup, and a lane-split contraction divides it across the
/// lanes. Traffic is schedule-invariant; parallelism and merge scratch are
/// per-candidate. Scalar wins ties, so a fold that already fills the
/// machine (a 200k-row head) keeps today's kernel.
pub fn fold_sched(
    fold_node: &Node,
    streaming_axis: Axis,
    carrier: &Carrier,
    dev: &DeviceProfile,
) -> FoldSched {
    if !mergeable_out_of_order(carrier)
        || carrier.project.len() != 1
        || carrier.project_reads_leaves()
    {
        return FoldSched::scalar();
    }
    let ext = |ax: Axis| ax.extent() as f64;
    let s_ext = ext(streaming_axis);
    let out_axes = fold_node.shape();
    let out_vol: f64 = out_axes.iter().map(|&a| ext(a)).product::<f64>().max(1.0);
    let b_bytes = 4.0f64; // the Metal backend computes in f32

    // Traffic — identical across schedules; only needed so memory-bound
    // kernels rank by memory occupancy rather than flop noise.
    let declared: HashMap<&'static str, f64> = input_dtypes(fold_node)
        .into_iter()
        .map(|(n, d)| (n, d.bytes()))
        .collect();
    let mut seen = HashSet::new();
    let hbm: f64 = input_axes(fold_node)
        .into_iter()
        .filter(|(n, _)| seen.insert(*n))
        .map(|(n, axes)| {
            let vol: f64 = axes.iter().map(|&a| ext(a)).product();
            vol * declared
                .get(n)
                .copied()
                .expect("every input must declare a storage dtype")
        })
        .sum::<f64>()
        + out_vol * b_bytes;

    // Per-leaf stats: issue cost of one evaluation, and which axes it reads.
    let leaf_stats: Vec<(f64, Vec<Axis>, bool)> = carrier
        .leaves
        .iter()
        .map(|l| {
            let axes = l.shape();
            let vol: f64 = axes.iter().map(|&a| ext(a)).product::<f64>().max(1.0);
            let per_eval = count_issue_ops(l) / vol;
            (per_eval, axes, has_simd_reduce(l))
        })
        .collect();
    let slot_ops: Vec<f64> = (0..carrier.slots)
        .map(|j| expr_ops(&carrier.into[j]) + expr_ops(&carrier.combine[j]))
        .collect();

    // Execution flops + the candidate kernel, for one schedule.
    let simd = SIMD as f64;
    let price = |sched: &FoldSched| -> Option<f64> {
        let f = (if sched.lane_stream { simd } else { 1.0 }) * sched.sgs as f64;
        if f > s_ext {
            return None; // an empty split unit would merge identity (the −∞ edge)
        }
        let (groups, lane_vol) = match sched.lane_axis {
            Some(a) => (out_vol / ext(a), ext(a)),
            None => (out_vol, 1.0),
        };
        let mut flops = 0.0;
        for (per_eval, axes, splits) in &leaf_stats {
            let issues = match sched.lane_axis {
                Some(a) if !axes.contains(&a) => {
                    // span-invariant: once per simdgroup; a simd-wide
                    // contraction divides across the lanes, anything else
                    // is issued by all of them.
                    groups * if *splits { 1.0 } else { simd }
                }
                _ => out_vol,
            };
            flops += issues * per_eval * s_ext;
        }
        for (j, ops) in slot_ops.iter().enumerate() {
            let sliced = sched
                .lane_axis
                .is_some_and(|a| carrier.spans[j].contains(&a));
            let issues = if sched.lane_axis.is_some() && !sliced {
                groups * simd
            } else {
                out_vol
            };
            flops += issues * ops * s_ext;
        }
        // merges: a shuffle butterfly per lane split, threadgroup rounds
        // per simdgroup split
        let merge_ops: f64 = slot_ops.iter().sum();
        if sched.lane_stream {
            flops += (simd.log2()) * merge_ops * groups * sched.sgs as f64;
        }
        if sched.sgs > 1 {
            flops += (sched.sgs as f64).log2() * merge_ops * groups * lane_vol;
        }
        // scratch: the threadgroup partial arrays
        let sliced_scratch: f64 = (0..carrier.slots)
            .map(|j| {
                if sched
                    .lane_axis
                    .is_some_and(|a| carrier.spans[j].contains(&a))
                {
                    lane_vol
                } else {
                    1.0
                }
            })
            .sum();
        let sram = if sched.sgs > 1 {
            sched.sgs as f64 * sliced_scratch * b_bytes
        } else {
            carrier.slots as f64 * b_bytes
        };
        let k = Kernel {
            name: "sched".into(),
            flops,
            hbm_bytes: hbm,
            sram_per_block: sram,
            regs_per_block: (sched.tg_threads() * carrier.slots) as f64 * b_bytes,
            parallel_blocks: if sched.is_scalar() { out_vol } else { groups },
            lanes_per_block: sched.tg_threads() as f64,
        };
        feasible(dev, &k).then(|| kernel_time(dev, &k))
    };

    let cands = fold_sched_candidates(fold_node, streaming_axis, carrier);
    let mut best = FoldSched::scalar();
    let mut best_t = f64::INFINITY;
    for c in cands {
        if let Some(t) = price(&c)
            && t < best_t
        {
            best = c;
            best_t = t;
        }
    }
    // A refinement of the winner, not a candidate: when the fold reads a
    // PACKED input, each lane folds contiguous 8-element runs instead of
    // striding — consecutive nibbles share bytes and cache lines, and the
    // unrolled run's index chains constant-fold. Pure re-association
    // (legal for exactly the carriers the lane split already requires);
    // the roofline cannot see load contiguity, so this is a stated rule,
    // not a priced choice.
    if best.lane_stream
        && best.lane_axis.is_none()
        && input_dtypes(fold_node)
            .iter()
            .any(|(_, d)| matches!(d, Dtype::I4))
    {
        let f_split = SIMD * best.sgs;
        if (s_ext as usize) % (f_split * 8) == 0 {
            best.chunk = 8;
        }
    }
    best
}

/// Every LEGAL cooperative schedule for a fold — the candidate set the
/// analytical chooser prices, exposed so a measured tuner can time the same
/// set on the real device and overrule the model (`--tune`). Order-sensitive
/// carriers get only the scalar entry, exactly as the chooser treats them.
pub fn fold_sched_candidates(
    fold_node: &Node,
    streaming_axis: Axis,
    carrier: &Carrier,
) -> Vec<FoldSched> {
    let mut cands = vec![FoldSched::scalar()];
    if !mergeable_out_of_order(carrier)
        || carrier.project.len() != 1
        || carrier.project_reads_leaves()
    {
        return cands;
    }
    let ext = |ax: Axis| ax.extent() as f64;
    let s_ext = ext(streaming_axis) as usize;
    let out_axes = fold_node.shape();
    let packed = input_dtypes(fold_node)
        .iter()
        .any(|(_, d)| matches!(d, Dtype::I4));
    for sgs in [1usize, 2, 4, 8, 16, 32] {
        cands.push(FoldSched {
            lane_axis: None,
            sgs,
            lane_stream: true,
            chunk: 1,
        });
        // the chunked twin prices identically (the roofline cannot see load
        // contiguity), so it never changes the analytic choice; it exists
        // for the measured tuner
        if packed && s_ext % (SIMD * sgs * 8) == 0 {
            cands.push(FoldSched {
                lane_axis: None,
                sgs,
                lane_stream: true,
                chunk: 8,
            });
        }
        for &a in &out_axes {
            if ext(a) as usize % SIMD == 0 && ext(a) >= SIMD as f64 {
                cands.push(FoldSched {
                    lane_axis: Some(a),
                    sgs,
                    lane_stream: false,
                    chunk: 1,
                });
            }
        }
    }
    cands
}

// ── helpers ──────────────────────────────────────────────────────────────────

/// True when the IR reduces an axis that is neither streamed nor in the output
/// — an inner contraction (the `d` of a matvec), whose intermediate must sit
/// in SRAM.
fn has_contraction(node: &Node, streaming: Axis, out_set: &HashSet<Axis>) -> bool {
    match node.as_ref() {
        NodeKind::Reduce { axis, src, .. } => {
            (*axis != streaming && !out_set.contains(axis))
                || has_contraction(src, streaming, out_set)
        }
        NodeKind::Map { inputs, .. } => inputs
            .iter()
            .any(|i| has_contraction(i, streaming, out_set)),
        NodeKind::Scan { src, .. } => has_contraction(src, streaming, out_set),
        NodeKind::Gather { src, index, .. } => {
            has_contraction(src, streaming, out_set) || has_contraction(index, streaming, out_set)
        }
        NodeKind::View { src, .. } | NodeKind::Reindex { src, .. } => {
            has_contraction(src, streaming, out_set)
        }
        NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => false,
    }
}

/// Rough flop count: one op per Map output element, one per Reduce/Scan source
/// element. Good enough to rank on the roofline. Memoized by pointer — a DAG
/// otherwise re-counts shared subtrees per path, the dominant cost profiling
/// found; the `output_axes` cache is threaded through so per-node volumes are
/// computed once for the whole walk.
fn count_flops(node: &Node) -> f64 {
    let mut oa = HashMap::new();
    let mut fc = HashMap::new();
    count_flops_memo(node, &mut oa, &mut fc)
}

fn count_flops_memo(
    node: &Node,
    oa: &mut HashMap<*const NodeKind, Vec<Axis>>,
    fc: &mut HashMap<*const NodeKind, f64>,
) -> f64 {
    let key = std::rc::Rc::as_ptr(node);
    if let Some(f) = fc.get(&key) {
        return *f;
    }
    let vol = |n: &Node, oa: &mut HashMap<*const NodeKind, Vec<Axis>>| -> f64 {
        let axes = oa
            .entry(std::rc::Rc::as_ptr(n))
            .or_insert_with(|| n.shape())
            .clone();
        axes.iter().map(|ax| ax.extent() as f64).product()
    };
    let f = match node.as_ref() {
        NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => 0.0,
        NodeKind::Map { inputs, .. } => {
            let child: f64 = inputs.iter().map(|i| count_flops_memo(i, oa, fc)).sum();
            child + vol(node, oa)
        }
        NodeKind::Reduce { src, .. } | NodeKind::Scan { src, .. } => {
            count_flops_memo(src, oa, fc) + vol(src, oa)
        }
        NodeKind::Gather { src, index, .. } => {
            count_flops_memo(src, oa, fc) + count_flops_memo(index, oa, fc)
        }
        // Reindexing costs nothing.
        NodeKind::View { src, .. } | NodeKind::Reindex { src, .. } => count_flops_memo(src, oa, fc),
    };
    fc.insert(key, f);
    f
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cost::DeviceProfile;
    use crate::derive::{Expr, SlotKind, derive};
    use crate::kernel_ir::*;

    #[test]
    fn attention_selects_row_axis_sq() {
        let (b, h, sq, k, d, e) = (
            axis("b", 1),
            axis("h", 8),
            axis("sq", 1024),
            axis("k", 1024),
            axis("d", 64),
            axis("e", 64),
        );
        let attn = attention(
            input("Q", &[b, h, sq, d], Dtype::F32),
            input("K", &[b, h, k, d], Dtype::F32),
            input("V", &[b, h, k, e], Dtype::F32),
            d,
            k,
        );
        let c = derive(&attn, k).unwrap();
        let dev = DeviceProfile::toy();
        let spec = plan_axis(&attn, k, &c, &dev).unwrap();

        assert_eq!(
            spec.row_axis,
            Some(sq),
            "row axis must be sq, not a batch dim"
        );
        assert_eq!(spec.col_axes, vec![e], "col axis is the value head dim");
        assert!(spec.batch_axes.contains(&b));
        assert!(spec.batch_axes.contains(&h));
        assert!(spec.tile_m > 1, "a non-trivial tile should be chosen");
        assert!(spec.cost > 0.0);
    }

    #[test]
    fn sum_schedules_with_tile_1() {
        // Reduce(X[n], n, Add) → scalar output; no row axis.
        let n = axis("n", 4096);
        let x = input("X", &[n], Dtype::F32);
        let s = reduce(x, n, BinOp::Monoid(Monoid::Add));
        let c = derive(&s, n).unwrap();
        let dev = DeviceProfile::toy();
        let spec = plan_axis(&s, n, &c, &dev).unwrap();

        assert_eq!(spec.tile_m, 1, "scalar output → row tile must be 1");
        assert_eq!(spec.row_axis, None);
        assert!(spec.cost > 0.0);
    }

    #[test]
    fn a_partial_span_axis_can_still_be_tiled() {
        let (stream, singleton, output) = (
            axis("stream", 2048),
            axis("singleton", 1),
            axis("output", 2048),
        );
        let dot = matmul(
            input("X", &[stream, singleton], Dtype::F32),
            input("W", &[output, stream], Dtype::BF16),
            stream,
        );
        let mut carrier = derive(&dot, stream).unwrap();
        let invariant = carrier.slots;
        carrier.slots += 1;
        carrier.into.push(Expr::Const(0.0));
        carrier.combine.push(Expr::A(invariant));
        carrier.identity.push(0.0);
        carrier.spans.push(Vec::new());
        carrier.kinds.push(SlotKind::Plain(Monoid::Add));

        let spec = plan_axis(&dot, stream, &carrier, &DeviceProfile::m1_pro())
            .expect("the output axis is a legal tile even if one slot is invariant");
        assert_eq!(spec.row_axis, Some(output));
    }

    #[test]
    fn attention_plans_to_single_kernel() {
        let (sq, k, d, e) = (
            axis("sq", 1024),
            axis("k", 1024),
            axis("d", 64),
            axis("e", 64),
        );
        let attn = attention(
            input("Q", &[sq, d], Dtype::F32),
            input("K", &[k, d], Dtype::F32),
            input("V", &[k, e], Dtype::F32),
            d,
            k,
        );
        let dev = DeviceProfile::toy();

        let plan = plan(&attn, &dev).expect("must find a feasible kernel");
        assert_eq!(plan.kernels.len(), 1);
        assert_eq!(plan.kernels[0].streaming_axis, k);
        assert!(plan.total_cost > 0.0);
    }

    #[test]
    fn sum_plans_to_single_kernel() {
        let n = axis("n", 65536);
        let x = input("X", &[n], Dtype::F32);
        let s = reduce(x, n, BinOp::Monoid(Monoid::Add));
        let dev = DeviceProfile::toy();

        // Sum has a carrier, so planning succeeds even for a fold with no
        // row tile and no inner contraction.
        let plan = plan(&s, &dev);
        assert!(plan.is_some(), "sum should have a feasible kernel");
        assert_eq!(plan.unwrap().kernels.len(), 1);
    }

    // Declared storage dtypes change the bandwidth bill: the same
    // memory-bound GEMV prices strictly cheaper with int8 weights, and
    // cheaper again with int4 — the quantization win, visible to the ranker.
    #[test]
    fn quantized_weights_price_less_bandwidth() {
        let (s, d, f) = (axis("s", 4), axis("d", 4096), axis("f", 4096));
        let dev = DeviceProfile::toy();

        let cost_of = |w: NodeRef| {
            let g = matmul(input("X", &[s, d], Dtype::F32), w, d);
            let c = derive(&g, d).unwrap();
            plan_axis(&g, d, &c, &dev).unwrap().cost
        };
        let full = cost_of(input("W", &[f, d], Dtype::F32));
        let int8 = cost_of(input("W8", &[f, d], Dtype::I8));
        let int4 = cost_of(input("W4", &[f, d], Dtype::I4));
        assert!(
            int8 < full,
            "int8 weights must price below f32: {int8} vs {full}"
        );
        assert!(int4 < int8, "int4 must price below int8: {int4} vs {int8}");
    }
}
