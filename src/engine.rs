//! The front door: the **structure map**.
//!
//! Both design docs name a single artifact as the engine's output — per (node,
//! axis), the structure tag, and for the foldable axes the derived accumulator.
//! `analyze` produces exactly that in one call: run Stage 1 to classify each
//! axis, and Stage 2A to derive a carrier wherever the axis folds.

use crate::carrier::{self, Carrier};
use crate::engine_ir::{self, Node};
use crate::stage1::{self, Parallelism, Structure};

/// The verdict for one axis of a graph.
pub struct AxisReport {
    pub axis: String,
    pub structure: Structure,
    /// The streaming accumulator — present iff the axis is MONOIDAL (foldable).
    /// FREE axes are grid dimensions (nothing to fold); OPAQUE/SEQUENTIAL axes
    /// have no one-pass form.
    pub carrier: Option<Carrier>,
}

/// The structure map for a graph over a chosen set of axes.
pub struct Report {
    pub axes: Vec<AxisReport>,
}

/// Classify each axis and derive its accumulator where it folds.
pub fn analyze(node: &Node, axes: &[&str]) -> Report {
    let axes = axes
        .iter()
        .map(|a| {
            let structure = stage1::structure(node, a);
            let carrier = match structure.level {
                Parallelism::Monoidal => carrier::derive(node, a),
                _ => None,
            };
            AxisReport {
                axis: a.to_string(),
                structure,
                carrier,
            }
        })
        .collect();
    Report { axes }
}

/// Classify *every* axis the graph touches — the zero-config front door. The
/// engine discovers the axes itself, so the caller needn't name them.
pub fn analyze_all(node: &Node) -> Report {
    analyze(node, &engine_ir::all_axes(node))
}

impl Report {
    /// Render the whole map: one line per axis with its tag and what it licenses,
    /// followed by the derived accumulator for the foldable ones.
    pub fn render(&self) -> String {
        let mut out = String::from("structure map\n");
        for a in &self.axes {
            let tag = match a.structure.level {
                Parallelism::Free => "FREE",
                Parallelism::Monoidal if a.structure.linear => "MONOIDAL (linear)",
                Parallelism::Monoidal => "MONOIDAL",
                Parallelism::Opaque => "OPAQUE",
                Parallelism::Sequential => "SEQUENTIAL",
            };
            let action = match a.structure.level {
                Parallelism::Free => "grid (DOALL)",
                // MONOIDAL with no carrier here folds in a sub-expression (e.g. a
                // contraction reduced one level down) — a cross-axis producer.
                Parallelism::Monoidal if a.carrier.is_some() => "fold",
                Parallelism::Monoidal => "fold (in a sub-expression)",
                Parallelism::Opaque => "runtime gather",
                Parallelism::Sequential => "serial — no fold",
            };
            out += &format!("  {:<4} {:<18} → {}\n", a.axis, tag, action);
            if let Some(c) = &a.carrier {
                for line in c.render().lines() {
                    out += &format!("         {line}\n");
                }
            }
        }
        out
    }
}
