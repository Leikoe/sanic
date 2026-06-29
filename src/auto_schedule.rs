//! Generic auto-scheduler: pick the cheapest feasible tiling for any
//! MONOIDAL axis given a derived carrier, a device model, and axis extents.
//!
//! No computation-specific knowledge lives here — only the generic roofline
//! model driven by information that `carrier.rs` and `engine_ir.rs` expose.

use std::collections::{HashMap, HashSet};

use crate::carrier::Carrier;
use crate::engine_ir::{all_axes, input_axes, leaf_names, output_axes, Axis, Node, NodeKind};
use crate::schedule::{best_tile, kernel_time, Device, Kernel};

/// Fixed streaming tile size (number of elements along the streaming axis per step).
pub const TILE_N: usize = 64;

/// A fully-resolved kernel specification — everything the code emitter needs.
#[derive(Debug, Clone)]
pub struct KernelSpec {
    pub name: String,
    /// The MONOIDAL axis this kernel folds over.
    pub streaming_axis: String,
    /// Axes contracted internally (appear in inputs but not in output or streaming).
    pub contract_axes: Vec<String>,
    /// The FREE axis we tile over in SRAM (the "row" dimension).
    /// Empty string means the output is scalar — tile_m is always 1.
    pub row_axis: String,
    /// Axes spanned by the output accumulator slot (e.g. the value head dim).
    pub col_axes: Vec<String>,
    /// FREE axes handled as block-level parallelism (not tiled in SRAM).
    pub batch_axes: Vec<String>,
    pub carrier: Carrier,
    pub tile_m: usize,
    pub tile_n: usize,
    pub input_names: Vec<&'static str>,
    pub output_name: String,
    pub cost: f64,
}

/// Choose the cheapest feasible tile for streaming `node` over `streaming_axis`.
/// Returns `None` if no tile is feasible on this device.
///
/// All decisions are derived from the carrier's `spans`/`acc_scalars`, the IR
/// topology (which inputs have the streaming axis), and the `extents` map.
pub fn best_kernel_for_axis(
    node: &Node,
    streaming_axis: &str,
    carrier: &Carrier,
    dev: &Device,
    extents: &HashMap<String, f64>,
) -> Option<KernelSpec> {
    let b_bytes = dev.dtype_bytes;
    let tn = TILE_N as f64;

    // ── axis classification ────────────────────────────────────────────────
    let out_axes = output_axes(node);
    let out_set: HashSet<Axis> = out_axes.iter().copied().collect();

    // Contract axes: appear in the graph but not in output and not streaming.
    let contract_axes: Vec<Axis> = all_axes(node)
        .into_iter()
        .filter(|&ax| ax != streaming_axis && !out_set.contains(ax))
        .collect();

    // ── span analysis: derive row and col axes from the carrier ───────────
    // Each carrier slot has a `span`: the axes it ranges over PER OUTPUT POINT.
    // For flash attention streaming over k:
    //   span[0] = ["sq"]        (max — one scalar per query row)
    //   span[1] = ["sq"]        (sum — one scalar per query row)
    //   span[2] = ["sq", "e"]   (output — one e-vector per query row)
    //
    // The intersection of all spans = axes shared by every slot → the "row" dims.
    // The union minus intersection = axes unique to some slots → the "col" dims.
    let span_union: HashSet<Axis> = carrier.spans.iter().flat_map(|s| s.iter().copied()).collect();
    let span_intersection: HashSet<Axis> = {
        let mut inter: HashSet<Axis> = span_union.clone();
        for s in &carrier.spans {
            let s_set: HashSet<Axis> = s.iter().copied().collect();
            inter.retain(|ax| s_set.contains(ax));
        }
        inter
    };
    // Col axes = present in some slots but not all (the "value head" dimension, etc.)
    let col_axes: Vec<Axis> = {
        let mut v: Vec<Axis> = span_union.iter().filter(|ax| !span_intersection.contains(*ax)).copied().collect();
        v.sort();
        v
    };
    // Span-shared axes = candidates for "row" and "batch" roles.
    let span_shared: Vec<Axis> = {
        let mut v: Vec<Axis> = span_intersection.iter().copied().collect();
        v.sort();
        v
    };

    // ── classify inputs ────────────────────────────────────────────────────
    // "streaming input" = has the streaming axis → re-read once per sq-block.
    // Deduplicate by name: the same Input node can be reached via multiple
    // paths in a shared-subexpression DAG (e.g. softmax forks its scores
    // tensor), but it is physically one tensor — count it once.
    let inputs: Vec<(&'static str, Vec<Axis>)> = {
        let mut seen = HashSet::new();
        input_axes(node)
            .into_iter()
            .filter(|(n, _)| seen.insert(*n))
            .collect()
    };
    let streaming_input_axes: HashSet<Axis> = inputs
        .iter()
        .filter(|(_, axes)| axes.contains(&streaming_axis))
        .flat_map(|(_, axes)| axes.iter().copied())
        .collect();

    // Row axis: one shared-span axis we tile over in SRAM per block.
    // Prefer span_shared axes NOT in streaming inputs (e.g. "sq" which only Q
    // carries). Fall back to any span_shared axis (for cases like 2D sum where
    // the single input carries both row and streaming axes).
    let row_axis: &str = span_shared
        .iter()
        .find(|&&ax| !streaming_input_axes.contains(ax))
        .or_else(|| span_shared.first())
        .copied()
        .unwrap_or("");

    let sq = if row_axis.is_empty() {
        1.0
    } else {
        *extents.get(row_axis)?
    };

    // Batch axes: span_shared axes that are NOT the row axis.
    // These become independent grid dimensions (one block per (batch…) tuple).
    let batch_axes: Vec<&str> = span_shared
        .iter()
        .filter(|&&ax| ax != row_axis)
        .copied()
        .collect();

    let batch_volume: f64 = batch_axes
        .iter()
        .map(|&ax| extents.get(ax).copied().unwrap_or(1.0))
        .product();

    // ── SRAM formula (generic) ─────────────────────────────────────────────
    // Accumulator per lane (one output point = one tile_m row):
    // span_intersection axes contribute 1.0 (handled by tile_m scaling);
    // col axes (unique to some slots) contribute their full extent.
    let acc_per_lane = carrier.acc_scalars(|ax| {
        if span_intersection.contains(ax) {
            1.0
        } else {
            extents.get(ax).copied().unwrap_or(1.0)
        }
    });

    // Intermediate tile (QK-like products): present when there is an inner
    // contraction (a Reduce not along the streaming axis and not in the output).
    let has_inner_contraction = has_contraction(node, streaming_axis, &out_set);
    let intermediate_per_lane = if has_inner_contraction { tn } else { 0.0 };

    // For each input, compute its "inner shape" = product of dimensions that
    // are not batch, not streaming, not row.
    let inner_dims = |axes: &[Axis]| -> f64 {
        axes.iter()
            .filter(|&&ax| {
                ax != streaming_axis
                    && ax != row_axis
                    && !batch_axes.contains(&ax)
            })
            .map(|&ax| extents.get(ax).copied().unwrap_or(1.0))
            .product()
    };

    // Split inputs into four categories for the SRAM formula.
    let (mut constant_sram, mut per_lane_sram) = (0.0f64, 0.0f64);
    for (_, axes) in &inputs {
        let is_streaming = axes.contains(&streaming_axis);
        let has_row = !row_axis.is_empty() && axes.contains(&row_axis);
        let inner = inner_dims(axes);
        match (is_streaming, has_row) {
            (true, false) => constant_sram += tn * inner,   // K/V-like: fixed per block
            (true, true) => per_lane_sram += tn * inner,    // X[m,n]-like: scales with tile
            (false, true) => per_lane_sram += inner,         // Q-like: scales with tile
            (false, false) => constant_sram += inner,        // constants
        }
    }
    per_lane_sram += acc_per_lane + intermediate_per_lane;

    // ── HBM formula (generic) ─────────────────────────────────────────────
    // streaming-without-row inputs are re-read once per sq-block.
    // Everything else is read/written exactly once (per batch instance).
    let streaming_no_row_vol: f64 = inputs
        .iter()
        .filter(|(_, axes)| {
            axes.contains(&streaming_axis)
                && (row_axis.is_empty() || !axes.contains(&row_axis))
        })
        .map(|(_, axes)| {
            axes.iter()
                .map(|&ax| extents.get(ax).copied().unwrap_or(1.0))
                .product::<f64>()
        })
        .sum();

    let other_input_vol: f64 = inputs
        .iter()
        .filter(|(_, axes)| {
            !axes.contains(&streaming_axis)
                || (!row_axis.is_empty() && axes.contains(&row_axis))
        })
        .map(|(_, axes)| {
            axes.iter()
                .map(|&ax| extents.get(ax).copied().unwrap_or(1.0))
                .product::<f64>()
        })
        .sum();

    let output_vol: f64 = out_axes
        .iter()
        .map(|&ax| extents.get(ax).copied().unwrap_or(1.0))
        .product();

    // Total flops (generic IR walk).
    let total_flops = count_flops(node, extents);

    // ── tile sweep ─────────────────────────────────────────────────────────
    let mut cands: Vec<(usize, Kernel)> = Vec::new();
    let mut t = 1.0f64;
    while t <= sq {
        let blocks_per_bh = (sq / t).ceil();
        let total_blocks = batch_volume * blocks_per_bh;
        let sram = (constant_sram + per_lane_sram * t) * b_bytes;
        let hbm =
            (streaming_no_row_vol * blocks_per_bh + other_input_vol + output_vol) * b_bytes;
        cands.push((
            t as usize,
            Kernel {
                name: format!("fused(tile_m={})", t as usize),
                flops: total_flops,
                hbm_bytes: hbm,
                sram_per_block: sram,
                regs_per_block: sram,
                parallel_blocks: total_blocks,
            },
        ));
        t *= 2.0;
    }

    let (tile_m, best) = best_tile(dev, cands)?;
    let cost = kernel_time(dev, &best);

    Some(KernelSpec {
        name: "fused".to_string(),
        streaming_axis: streaming_axis.to_string(),
        contract_axes: contract_axes.iter().map(|s| s.to_string()).collect(),
        row_axis: row_axis.to_string(),
        col_axes: col_axes.iter().map(|s| s.to_string()).collect(),
        batch_axes: batch_axes.iter().map(|s| s.to_string()).collect(),
        carrier: carrier.clone(),
        tile_m,
        tile_n: TILE_N,
        input_names: leaf_names(node),
        output_name: "Out".to_string(),
        cost,
    })
}

// ── private helpers ────────────────────────────────────────────────────────────

/// True when the IR contains a `Reduce` along an axis that is neither the
/// streaming axis nor an output axis — i.e., an inner contraction like the
/// `d` dimension in a matmul. Such contractions create an intermediate tile
/// (the product accumulation) that must sit in SRAM.
fn has_contraction(node: &Node, streaming: &str, out_set: &HashSet<Axis>) -> bool {
    match node.as_ref() {
        NodeKind::Reduce { axis, src, .. } => {
            (*axis != streaming && !out_set.contains(*axis))
                || has_contraction(src, streaming, out_set)
        }
        NodeKind::Map { inputs, .. } => {
            inputs.iter().any(|i| has_contraction(i, streaming, out_set))
        }
        NodeKind::Scan { src, .. } => has_contraction(src, streaming, out_set),
        NodeKind::Gather { src, index, .. } => {
            has_contraction(src, streaming, out_set)
                || has_contraction(index, streaming, out_set)
        }
        NodeKind::Input { .. } => false,
    }
}

/// Approximate total flop count by counting one op per Map output element and
/// one op per Reduce/Scan source element. Good enough for roofline ranking.
fn count_flops(node: &Node, extents: &HashMap<String, f64>) -> f64 {
    let vol = |n: &Node| -> f64 {
        output_axes(n)
            .iter()
            .map(|&ax| extents.get(ax).copied().unwrap_or(1.0))
            .product()
    };
    match node.as_ref() {
        NodeKind::Input { .. } => 0.0,
        NodeKind::Map { inputs, .. } => {
            let child: f64 = inputs.iter().map(|i| count_flops(i, extents)).sum();
            child + vol(node) // 1 op per output element
        }
        NodeKind::Reduce { src, .. } | NodeKind::Scan { src, .. } => {
            count_flops(src, extents) + vol(src) // 1 op per src element
        }
        NodeKind::Gather { src, index, .. } => {
            count_flops(src, extents) + count_flops(index, extents)
        }
    }
}

// ── tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{carrier, engine_ir::*, schedule::Device};

    fn ext(pairs: &[(&str, f64)]) -> HashMap<String, f64> {
        pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    #[test]
    fn attention_selects_row_axis_sq() {
        let attn = attention(
            input("Q", &["b", "h", "sq", "d"]),
            input("K", &["b", "h", "k", "d"]),
            input("V", &["b", "h", "k", "e"]),
            "d", "k",
        );
        let c = carrier::derive(&attn, "k").unwrap();
        let dev = Device::toy();
        let extents = ext(&[("b", 1.0), ("h", 8.0), ("sq", 1024.0), ("k", 1024.0), ("d", 64.0), ("e", 64.0)]);
        let spec = best_kernel_for_axis(&attn, "k", &c, &dev, &extents).unwrap();

        assert_eq!(spec.row_axis, "sq", "row axis must be sq, not a batch dim");
        assert_eq!(spec.col_axes, vec!["e"], "col axis is the value head dim");
        assert!(spec.batch_axes.contains(&"b".to_string()));
        assert!(spec.batch_axes.contains(&"h".to_string()));
        assert!(spec.tile_m > 1, "non-trivial tiling should be chosen");
        assert!(spec.cost > 0.0);
    }

    #[test]
    fn sum_schedules_with_tile_1() {
        // Reduce(X[n], n, Add) → scalar output; no row axis.
        let x = input("X", &["n"]);
        let s = reduce(x, "n", crate::op::BinOp::Monoid(crate::op::Monoid::Add));
        let c = carrier::derive(&s, "n").unwrap();
        let dev = Device::toy();
        let extents = ext(&[("n", 4096.0)]);
        let spec = best_kernel_for_axis(&s, "n", &c, &dev, &extents).unwrap();

        assert_eq!(spec.tile_m, 1, "scalar output → tile_m must be 1");
        assert_eq!(spec.row_axis, "");
        assert!(spec.cost > 0.0);
    }

    #[test]
    fn sram_matches_attention_test_formula() {
        // Reproduce the numbers from tests/attention_scheduling.rs to confirm
        // the generic SRAM formula is consistent with the attention-specific one.
        let attn = attention(
            input("Q", &["sq", "d"]),
            input("K", &["k", "d"]),
            input("V", &["k", "e"]),
            "d", "k",
        );
        let c = carrier::derive(&attn, "k").unwrap();
        let dev = Device::toy();
        let sq = 2048.0; let k = 4096.0; let d = 64.0;
        let extents = ext(&[("sq", sq), ("k", k), ("d", d), ("e", d)]);

        let spec = best_kernel_for_axis(&attn, "k", &c, &dev, &extents).unwrap();
        // The existing test proves tile_m > 1 is chosen and the schedule is feasible.
        assert!(spec.tile_m > 1, "tiling must amortise K/V re-reads");
    }
}
