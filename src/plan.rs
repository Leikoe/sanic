//! Planning: pick the axis to stream, the axes to grid, and a block that fits.
//!
//! Nothing here knows what computation it is scheduling. Every decision is
//! read off information the other layers already expose: the carrier's slot
//! spans say which axes are candidates for each block role; the IR says which
//! inputs carry the streamed axis; the cost model ranks the candidates.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::cost::{DeviceProfile, Kernel, feasible, kernel_time};
use crate::derive::{Carrier, Expr, SlotKind};
use crate::ir::{
    self, AxisRef, AxisSelector, Dtype, Node as NodeKind, NodeRef as Node, input_axes,
    input_dtypes, leaf_names,
};

/// Elements streamed per step along the folded axis.
const TILE_N: usize = 64;

/// A planned fused fold: the streamed axis, its carrier, and the cost that
/// ranked it against other fold axes and against cutting. The physical GPU
/// schedule is chosen downstream by [`fold_sched`]; this spec carries what
/// the partitioner and the emitters actually consume.
#[derive(Debug, Clone)]
pub struct KernelSpec {
    /// The axis this kernel folds over.
    pub streaming_axis: AxisRef,
    pub carrier: Carrier,
    pub input_names: Vec<&'static str>,
    pub output_name: String,
    pub cost: f64,
}

/// Choose the cheapest feasible block structure for streaming `node` over one
/// axis. `None` if nothing fits the device.
pub fn plan_axis(
    node: &Node,
    streaming_axis: impl AxisSelector,
    carrier: &Carrier,
    dev: &DeviceProfile,
) -> Option<KernelSpec> {
    plan_axis_with_groups(
        node,
        streaming_axis,
        carrier,
        dev,
        &mut GroupCache::default(),
    )
}

/// Flatten-group membership across a retained DAG, collected once per node:
/// member axis → (grouped axis, member's share of the group). A GQA key
/// cache's `kv_heads` is a member of the group that forms `query_heads`, so a
/// block covering a tile of the group touches only the proportional share of
/// the member. The cache lives for one compile, like the structure cache.
#[derive(Default)]
pub struct GroupCache {
    members: HashMap<AxisRef, (AxisRef, f64)>,
    visited: HashSet<*const NodeKind>,
    keepalive: Vec<Node>,
}

impl GroupCache {
    fn collect(&mut self, node: &Node) {
        let mut stack = vec![node.clone()];
        while let Some(current) = stack.pop() {
            if !self.visited.insert(Rc::as_ptr(&current)) {
                continue;
            }
            for (group, grouped) in ir::view_groups(&current) {
                for &member in &group {
                    let share = member.extent() as f64 / grouped.extent() as f64;
                    self.members.insert(member, (grouped, share));
                }
            }
            stack.extend(ir::children(&current));
            self.keepalive.push(current);
        }
    }
}

/// [`plan_axis`] with a compile-lifetime [`GroupCache`], so repeated planning
/// over one retained DAG walks each node once.
pub fn plan_axis_with_groups(
    node: &Node,
    streaming_axis: impl AxisSelector,
    carrier: &Carrier,
    dev: &DeviceProfile,
    groups: &mut GroupCache,
) -> Option<KernelSpec> {
    let streaming_axis = streaming_axis
        .resolve_axis(node, "plan_axis")
        .expect("planning axis is absent from the selected node");
    let b_bytes = dev.dtype_bytes;
    // A streamed slab never exceeds the axis itself (a 7-token KV cache is
    // not a 64-element slab).
    let tn = (TILE_N as f64).min(streaming_axis.extent() as f64);

    // ── axis roles, read off the carrier and the IR ──────────────────────────
    let out_axes = ir::axis_refs(node);
    let out_set: HashSet<AxisRef> = out_axes.iter().copied().collect();

    // Each slot spans some free axes per output point. Axes shared by every
    // slot are row/batch/col-tile candidates; axes on only some slots are
    // per-slot columns (attention: m, ℓ span {sq}; o spans {sq, e} → shared
    // sq, column e).
    let span_union: Vec<AxisRef> = carrier
        .spans
        .iter()
        .flat_map(|span| span.iter().copied())
        .fold(Vec::new(), |mut axes, axis| {
            if !axes.contains(&axis) {
                axes.push(axis);
            }
            axes
        });
    let span_intersection: HashSet<AxisRef> = {
        let mut inter: HashSet<AxisRef> = span_union.iter().copied().collect();
        for s in &carrier.spans {
            let s_set: HashSet<AxisRef> = s.iter().copied().collect();
            inter.retain(|ax| s_set.contains(ax));
        }
        inter
    };
    let col_axes: Vec<AxisRef> = span_union
        .iter()
        .filter(|ax| !span_intersection.contains(ax))
        .copied()
        .collect();
    let span_schedulable: Vec<AxisRef> = {
        // A deferred divisor keeps its partial-span columns unscheduled — not
        // because tiling them is illegal (it only duplicates the normalizer
        // slot per block), but because the EMITTER recomputes everything per
        // output point and the cost model does not yet price that
        // duplication: measured with it scheduled, the vocab-head RMSNorm
        // fused at 128k outputs runs 15× worse than planned and the fused
        // projections re-add the whole residual history per streamed element
        // (llama decode 28.6 → 141 ms/tok). Lift this exclusion when leaf
        // cutting prices recomputation (Prop 4.1) so the roofline can reject
        // those fusions itself. Other partial-span slots stay schedulable.
        if carrier.rules.contains(&"defer-div") {
            span_union
                .iter()
                .filter(|axis| span_intersection.contains(axis))
                .copied()
                .collect()
        } else {
            span_union.clone()
        }
    };

    // ── classify inputs ──────────────────────────────────────────────────────
    // Dedup by name: a DAG can reach the same tensor along several paths
    // (softmax forks its scores), but it is physically one tensor. Consts and
    // iotas never appear here — they cost nothing to read.
    let inputs: Vec<(&'static str, Vec<AxisRef>)> = {
        let mut seen = HashSet::new();
        input_axes(node)
            .into_iter()
            .map(|(name, axes)| {
                let mut canonical = Vec::new();
                for axis in axes {
                    let axis = carrier.aliases.get(&axis).copied().unwrap_or(axis);
                    if !canonical.contains(&axis) {
                        canonical.push(axis);
                    }
                }
                (name, canonical)
            })
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

    groups.collect(node);
    let group_member = &groups.members;

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
    let ext = |ax: AxisRef| ax.extent() as f64;
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
    let mut by_extent: Vec<AxisRef> = span_schedulable.clone();
    by_extent.sort_by(|&a, &b| ext(b).total_cmp(&ext(a)));

    let mut best_cost: Option<f64> = None;

    let row_options: Vec<Option<AxisRef>> =
        by_extent.iter().map(|&a| Some(a)).chain([None]).collect();
    for &row in &row_options {
        let col_options: Vec<Option<AxisRef>> = [None]
            .into_iter()
            .chain(
                by_extent
                    .iter()
                    .filter(|&&a| Some(a) != row)
                    .map(|&a| Some(a)),
            )
            .collect();
        for &col in &col_options {
            let rest: Vec<AxisRef> = span_schedulable
                .iter()
                .copied()
                .filter(|&a| Some(a) != row && Some(a) != col)
                .collect();
            // descending bit patterns: all-batch first
            for bits in (0..(1u32 << rest.len().min(8))).rev() {
                let batch: Vec<AxisRef> = rest
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
                let per_block = |axes: &[AxisRef], tm: f64, tc: f64| -> f64 {
                    let scheduled = |ax: AxisRef| -> Option<f64> {
                        if ax == streaming_axis {
                            Some(tn)
                        } else if Some(ax) == row {
                            Some(tm)
                        } else if Some(ax) == col {
                            Some(tc)
                        } else if batch.contains(&ax) {
                            Some(1.0)
                        } else {
                            None
                        }
                    };
                    axes.iter()
                        .map(|&ax| {
                            if let Some(tile) = scheduled(ax) {
                                tile
                            } else if let Some(&(grouped, share)) = group_member.get(&ax) {
                                // a block covering a tile of the grouped axis
                                // touches only that tile's share of the member
                                match scheduled(grouped) {
                                    Some(tile) => (tile * share).clamp(1.0, ext(ax)),
                                    None => ext(ax),
                                }
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
                        // "Carries the axis" includes carrying a group member
                        // coupled to it: each block of the grouped axis reads
                        // its own slice of the member, not the whole tensor
                        // again.
                        let carries = |axes: &[AxisRef], scheduled: AxisRef| -> bool {
                            axes.iter().any(|&ax| {
                                ax == scheduled
                                    || group_member
                                        .get(&ax)
                                        .is_some_and(|&(grouped, _)| grouped == scheduled)
                            })
                        };
                        let mut hbm = output_vol * b_bytes;
                        for (nm, axes) in &inputs {
                            let vol: f64 = axes.iter().map(|&ax| ext(ax)).product();
                            let mut factor = 1.0;
                            if let Some(r) = row
                                && !carries(axes, r)
                            {
                                factor *= row_blocks;
                            }
                            if let Some(c) = col
                                && !carries(axes, c)
                            {
                                factor *= col_blocks;
                            }
                            for &b in &batch {
                                if !carries(axes, b) {
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
                        if best_cost.is_none_or(|b| cost < b) {
                            best_cost = Some(cost);
                        }
                    }
                }
            }
        }
    }

    Some(KernelSpec {
        streaming_axis,
        carrier: carrier.clone(),
        input_names: leaf_names(node),
        output_name: "Out".to_string(),
        cost: best_cost?,
    })
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
    pub lane_axis: Option<AxisRef>,
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
/// `Monoid` is commutative; ExpShifted rescaling and extremal-key payload
/// selection are symmetric, so all three split freely.
pub fn mergeable_out_of_order(carrier: &Carrier) -> bool {
    carrier.kinds.iter().all(|k| {
        matches!(
            k,
            SlotKind::Plain(_) | SlotKind::ExpShifted { .. } | SlotKind::AtExtremum { .. }
        )
    })
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
        Expr::Exp(a)
        | Expr::Log(a)
        | Expr::Sqrt(a)
        | Expr::Tanh(a)
        | Expr::Sin(a)
        | Expr::Cos(a) => 1.0 + expr_ops(a),
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
        NodeKind::Iota { .. } | NodeKind::Coordinate { .. } => vol(node),
        NodeKind::Input { shape, .. } => (1.0 + shape.len() as f64) * vol(node),
        NodeKind::Map { inputs, .. } => inputs.iter().map(count_issue_ops).sum::<f64>() + vol(node),
        NodeKind::Reduce { src, .. } | NodeKind::Scan { src, .. } => {
            count_issue_ops(src) + vol(src)
        }
        NodeKind::Gather { src, index, .. } => {
            count_issue_ops(src) + count_issue_ops(index) + 2.0 * vol(node)
        }
        NodeKind::View { src, dims } => {
            let split_ops: f64 = dims
                .iter()
                .map(|dim| {
                    if dim.sources.len() > 1 {
                        2.0 * dim.sources.len() as f64
                    } else {
                        0.0
                    }
                })
                .sum();
            count_issue_ops(src) + split_ops * vol(node)
        }
        NodeKind::Reindex {
            src, map, padded, ..
        } => {
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
        NodeKind::Reduce { src, dim, .. } => {
            ir::source_axis(src, *dim).extent() % SIMD == 0 || has_simd_reduce(src)
        }
        NodeKind::Map { inputs, .. } => inputs.iter().any(has_simd_reduce),
        NodeKind::Gather { src, index, .. } => has_simd_reduce(src) || has_simd_reduce(index),
        NodeKind::View { src, .. } | NodeKind::Reindex { src, .. } | NodeKind::Scan { src, .. } => {
            has_simd_reduce(src)
        }
        NodeKind::Input { .. }
        | NodeKind::Const { .. }
        | NodeKind::Iota { .. }
        | NodeKind::Coordinate { .. } => false,
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
    streaming_axis: AxisRef,
    carrier: &Carrier,
    dev: &DeviceProfile,
) -> FoldSched {
    if !mergeable_out_of_order(carrier) || carrier.project.len() != 1 {
        return FoldSched::scalar();
    }
    let ext = |ax: AxisRef| ax.extent() as f64;
    let s_ext = ext(streaming_axis);
    let out_axes = ir::axis_refs(fold_node);
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
    let leaf_stats: Vec<(f64, Vec<AxisRef>, bool)> = carrier
        .leaves
        .iter()
        .map(|l| {
            let axes = ir::axis_refs(l)
                .into_iter()
                .map(|axis| carrier.aliases.get(&axis).copied().unwrap_or(axis))
                .collect::<Vec<_>>();
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
fn fold_sched_candidates(
    fold_node: &Node,
    streaming_axis: AxisRef,
    carrier: &Carrier,
) -> Vec<FoldSched> {
    let mut cands = vec![FoldSched::scalar()];
    if !mergeable_out_of_order(carrier) || carrier.project.len() != 1 {
        return cands;
    }
    let ext = |ax: AxisRef| ax.extent() as f64;
    let s_ext = ext(streaming_axis) as usize;
    let out_axes = ir::axis_refs(fold_node);
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
fn has_contraction(node: &Node, streaming: AxisRef, out_set: &HashSet<AxisRef>) -> bool {
    match node.as_ref() {
        NodeKind::Reduce { src, dim, .. } => {
            let axis = ir::source_axis(src, *dim);
            (axis != streaming && !out_set.contains(&axis))
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
        NodeKind::Input { .. }
        | NodeKind::Const { .. }
        | NodeKind::Iota { .. }
        | NodeKind::Coordinate { .. } => false,
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
    oa: &mut HashMap<*const NodeKind, Vec<AxisRef>>,
    fc: &mut HashMap<*const NodeKind, f64>,
) -> f64 {
    let key = std::rc::Rc::as_ptr(node);
    if let Some(f) = fc.get(&key) {
        return *f;
    }
    let vol = |n: &Node, oa: &mut HashMap<*const NodeKind, Vec<AxisRef>>| -> f64 {
        let axes = oa
            .entry(std::rc::Rc::as_ptr(n))
            .or_insert_with(|| ir::axis_refs(n))
            .clone();
        axes.iter().map(|ax| ax.extent() as f64).product()
    };
    let f = match node.as_ref() {
        NodeKind::Input { .. }
        | NodeKind::Const { .. }
        | NodeKind::Iota { .. }
        | NodeKind::Coordinate { .. } => 0.0,
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
    use crate::ir::*;
    use crate::nn::scaled_dot_product_attention;

    #[test]
    fn attention_plans_a_feasible_batched_kernel() {
        let (b, h, sq, k, d, e) = (
            axis("b", 1),
            axis("h", 8),
            axis("sq", 1024),
            axis("k", 1024),
            axis("d", 64),
            axis("e", 64),
        );
        let key = input("K", [b, h, k, d], Dtype::F32);
        let key_axis = axis_refs(&key)[2];
        let attn = scaled_dot_product_attention(
            input("Q", [b, h, sq, d], Dtype::F32),
            key,
            input("V", [b, h, k, e], Dtype::F32),
            None,
            0.0,
            false,
            Some(1.0),
            false,
        );
        let c = derive(&attn, key_axis).unwrap();
        let dev = DeviceProfile::toy();
        let spec = plan_axis(&attn, key_axis, &c, &dev).unwrap();

        assert_eq!(spec.streaming_axis, key_axis);
        assert!(spec.cost > 0.0);
    }

    #[test]
    fn a_scalar_output_fold_still_plans() {
        // Reduce(X[n], n, Add) → scalar output; no row axis.
        let n = axis("n", 4096);
        let x = input("X", [n], Dtype::F32);
        let stream = axis_refs(&x)[0];
        let s = reduce(x, 0usize, Monoid::Add);
        let c = derive(&s, stream).unwrap();
        let dev = DeviceProfile::toy();
        let spec = plan_axis(&s, stream, &c, &dev).unwrap();

        assert!(spec.cost > 0.0, "a scalar-output fold still plans");
    }

    #[test]
    fn a_partial_span_axis_can_still_be_tiled() {
        let (stream, singleton, output) = (
            axis("stream", 2048),
            axis("singleton", 1),
            axis("output", 2048),
        );
        let x = input("X", [stream, singleton], Dtype::F32);
        let stream_axis = axis_refs(&x)[0];
        let dot = matmul(
            transpose(x, 0usize, 1usize),
            transpose(input("W", [output, stream], Dtype::BF16), 0usize, 1usize),
        );
        let mut carrier = derive(&dot, stream_axis).unwrap();
        let invariant = carrier.slots;
        carrier.slots += 1;
        carrier.into.push(Expr::Const(0.0));
        carrier.combine.push(Expr::A(invariant));
        carrier.identity.push(0.0);
        carrier.spans.push(Vec::new());
        carrier.kinds.push(SlotKind::Plain(Monoid::Add));

        plan_axis(&dot, stream_axis, &carrier, &DeviceProfile::m1_pro())
            .expect("the output axis is a legal tile even if one slot is invariant");
    }

    // Declared storage dtypes change the bandwidth bill: the same
    // memory-bound GEMV prices strictly cheaper with int8 weights, and
    // cheaper again with int4 — the quantization win, visible to the ranker.
    #[test]
    fn quantized_weights_price_less_bandwidth() {
        let (s, d, f) = (axis("s", 4), axis("d", 4096), axis("f", 4096));
        let dev = DeviceProfile::toy();

        let cost_of = |w: NodeRef| {
            let x = input("X", [s, d], Dtype::F32);
            let stream = axis_refs(&x)[1];
            let g = matmul(x, transpose(w, 0usize, 1usize));
            let c = derive(&g, stream).unwrap();
            plan_axis(&g, stream, &c, &dev).unwrap().cost
        };
        let full = cost_of(input("W", [f, d], Dtype::F32));
        let int8 = cost_of(input("W8", [f, d], Dtype::I8));
        let int4 = cost_of(input("W4", [f, d], Dtype::I4));
        assert!(
            int8 < full,
            "int8 weights must price below f32: {int8} vs {full}"
        );
        assert!(int4 < int8, "int4 must price below int8: {int4} vs {int8}");
    }
}
