//! Partition an IR node into a `ScheduleGraph` — an ordered list of
//! `KernelSpec`s — and emit the resulting CuTile Python module.
//!
//! The partition algorithm: run `analyze_all` to find every MONOIDAL axis
//! that has a carrier, call `best_kernel_for_axis` for each, pick the axis
//! whose kernel has the lowest cost. For the MVP the graph is always a single
//! kernel; recursive splitting (for multi-kernel pipelines) is left as a
//! natural extension.

use std::collections::HashMap;

use crate::auto_schedule::{best_kernel_for_axis, KernelSpec};
use crate::codegen_cutile::{cutile_kernel, KernelCodegenParams};
use crate::engine::analyze_all;
use crate::engine_ir::Node;
use crate::schedule::Device;
use crate::stage1::Parallelism;

/// A DAG of kernels in topological (execution) order.
/// For the current MVP this is always a single kernel.
/// Edges are materialized HBM tensors: `output_name` of kernel[i] appears
/// in `input_names` of kernel[j > i].
pub struct ScheduleGraph {
    pub kernels: Vec<KernelSpec>,
    pub total_cost: f64,
}

impl ScheduleGraph {
    /// Emit a complete Python source file: all `@ct.kernel` definitions
    /// followed by a `def run(...)` launcher function.
    pub fn emit_python(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        for spec in &self.kernels {
            if let Some(src) = emit_kernel(spec) {
                parts.push(src);
            } else {
                parts.push(format!(
                    "# TODO: no emitter for kernel '{}' (streaming over '{}')\n",
                    spec.name, spec.streaming_axis
                ));
            }
        }
        parts.push(emit_run(&self.kernels));
        parts.join("\n")
    }
}

/// Partition `node` into the cheapest feasible single-kernel `ScheduleGraph`.
/// Tries every MONOIDAL axis that has a derived carrier, picks the one with
/// the lowest roofline cost.
pub fn partition(
    node: &Node,
    dev: &Device,
    extents: &HashMap<String, f64>,
) -> Option<ScheduleGraph> {
    let report = analyze_all(node);

    let mut best: Option<KernelSpec> = None;
    for axis_report in &report.axes {
        if axis_report.structure.level != Parallelism::Monoidal {
            continue;
        }
        let Some(ref carrier) = axis_report.carrier else { continue };
        let Some(spec) = best_kernel_for_axis(node, &axis_report.axis, carrier, dev, extents)
        else {
            continue;
        };
        let cheaper = best.as_ref().map_or(true, |b| spec.cost < b.cost);
        if cheaper {
            best = Some(spec);
        }
    }

    let spec = best?;
    let cost = spec.cost;
    Some(ScheduleGraph {
        kernels: vec![spec],
        total_cost: cost,
    })
}

// ── kernel emission (tries known patterns in order) ────────────────────────

fn emit_kernel(spec: &KernelSpec) -> Option<String> {
    let params = KernelCodegenParams {
        name: spec.name.clone(),
        stream_axis: spec.streaming_axis.clone(),
        contract_axis: spec.contract_axes.first().cloned().unwrap_or_default(),
        row_axis: spec.row_axis.clone(),
        col_axis: spec.col_axes.first().cloned().unwrap_or_default(),
        batch_axes: spec.batch_axes.clone(),
        input_names: spec.input_names.clone(),
        output_name: spec.output_name.clone(),
    };
    cutile_kernel(&spec.carrier, &params)
}

fn emit_run(kernels: &[KernelSpec]) -> String {
    if kernels.is_empty() {
        return "def run(): pass\n".to_string();
    }
    let mut seen = std::collections::HashSet::new();
    let unique_inputs: Vec<&str> = kernels
        .iter()
        .flat_map(|s| s.input_names.iter().copied())
        .filter(|n| seen.insert(*n))
        .collect();

    let sig = unique_inputs.join(", ");
    let mut body = String::new();
    for spec in kernels {
        body += &format!(
            "    # launch '{}' — streams over '{}'\n",
            spec.name, spec.streaming_axis
        );
        // Grid: (ceil(sq / tile_m), batch_volume, 1)
        let grid = if spec.row_axis.is_empty() {
            "(1, 1, 1)".to_string()
        } else {
            let batch = if spec.batch_axes.is_empty() {
                "1".to_string()
            } else {
                spec.batch_axes
                    .iter()
                    .map(|ax| format!("{ax}.shape[0]"))
                    .collect::<Vec<_>>()
                    .join(" * ")
            };
            format!(
                "(ct.cdiv({}.shape[-2], {tile_m}), {batch}, 1)",
                spec.input_names.first().copied().unwrap_or("Q"),
                tile_m = spec.tile_m
            )
        };
        body += &format!(
            "    ct.launch(torch.cuda.current_stream(), {grid}, {name}, ({inputs}, {out}))\n",
            name = spec.name,
            inputs = spec.input_names.join(", "),
            out = spec.output_name,
        );
    }
    let last = kernels.last().unwrap();
    format!(
        "def run({sig}, {out}):\n    import torch\n    import cuda.tile as ct\n{body}",
        out = last.output_name,
    )
}

// ── tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine_ir::*;

    fn ext(pairs: &[(&str, f64)]) -> HashMap<String, f64> {
        pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    #[test]
    fn attention_partitions_to_single_kernel() {
        let attn = attention(
            input("Q", &["sq", "d"]),
            input("K", &["k", "d"]),
            input("V", &["k", "e"]),
            "d", "k",
        );
        let dev = Device::toy();
        let extents = ext(&[("sq", 1024.0), ("k", 1024.0), ("d", 64.0), ("e", 64.0)]);

        let graph = partition(&attn, &dev, &extents).expect("must find a feasible kernel");
        assert_eq!(graph.kernels.len(), 1);
        assert_eq!(graph.kernels[0].streaming_axis, "k");
        assert!(graph.total_cost > 0.0);
    }

    #[test]
    fn attention_emits_valid_python() {
        let attn = attention(
            input("Q", &["sq", "d"]),
            input("K", &["k", "d"]),
            input("V", &["k", "e"]),
            "d", "k",
        );
        let dev = Device::toy();
        let extents = ext(&[("sq", 1024.0), ("k", 1024.0), ("d", 64.0), ("e", 64.0)]);

        let graph = partition(&attn, &dev, &extents).unwrap();
        let py = graph.emit_python();

        assert!(py.contains("@ct.kernel"), "missing kernel decorator");
        assert!(py.contains("ct.mma("), "missing matmul call");
        assert!(py.contains("acc_2 / acc_1"), "missing projection");
        assert!(py.contains("def run("), "missing launcher");
    }

    #[test]
    fn sum_partitions_to_single_kernel() {
        let x = input("X", &["n"]);
        let s = reduce(x, "n", crate::op::BinOp::Monoid(crate::op::Monoid::Add));
        let dev = Device::toy();
        let extents = ext(&[("n", 65536.0)]);

        // sum has a carrier; partition should find it even though codegen
        // can't emit CuTile for it yet (returns placeholder comment).
        let graph = partition(&s, &dev, &extents);
        // A carrier exists for sum, so partition should succeed.
        assert!(graph.is_some(), "sum should have a feasible kernel");
        assert_eq!(graph.unwrap().kernels.len(), 1);
    }
}
