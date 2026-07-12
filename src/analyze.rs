//! Classification: how does the computation depend on each axis?
//!
//! One recursive pass assigns every (node, axis) a rung on a four-value
//! ladder, ordered from most to least parallel. The verdict is simple:
//! FREE and MONOIDAL axes stream and parallelize; OPAQUE and SEQUENTIAL do
//! not. `analyze_all` packages the verdicts for a whole graph — every axis
//! classified, with the derived accumulator attached wherever one exists.

use crate::derive::{self, Carrier};
use crate::ir::{self, Axis, Node, NodeKind};

/// How parallel an axis is, best to worst. `Ord` follows that order, so the
/// join of two structures is `max` — the least parallel input wins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Parallelism {
    /// No dependence along the axis → a grid dimension.
    Free,
    /// Associative fold with identity → stream in one pass, or tree-reduce.
    Monoidal,
    /// Data-dependent access (gather) → decided at runtime.
    Opaque,
    /// Non-associative dependence → strictly serial.
    Sequential,
}

/// The classification of one (node, axis). `linear` is a refinement of
/// MONOIDAL: the fold is the semiring "plus" and the folded quantity enters
/// it linearly, so constant factors can be pulled out of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Structure {
    pub level: Parallelism,
    pub linear: bool,
}

impl Structure {
    pub const FREE: Structure = Structure {
        level: Parallelism::Free,
        linear: true,
    };

    fn at(level: Parallelism, linear: bool) -> Structure {
        Structure { level, linear }
    }

    /// Combine two input classifications: worst parallelism wins, and
    /// linearity survives only if both sides have it.
    fn join(self, other: Structure) -> Structure {
        Structure {
            level: self.level.max(other.level),
            linear: self.linear && other.linear,
        }
    }
}

fn join_all(it: impl Iterator<Item = Structure>) -> Structure {
    it.fold(Structure::FREE, Structure::join)
}

/// Classify how `node` depends on `axis`. Plain structural recursion. Because
/// the answer is per (node, axis), the middle axis of `(X·Y)·Z` can be free at
/// the first matmul and contracted at the second — which is exactly the fact
/// that distinguishes the two kinds of fusion.
pub fn structure(node: &NodeKind, axis: Axis) -> Structure {
    match node {
        // Raw data, literals and index values depend on no axis (an Iota
        // varies *with* its axis, but elementwise — no cross-element
        // dependence, which is what FREE means).
        NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => Structure::FREE,

        // Elementwise: pass the joined input structure through. Linearity
        // survives only if the op itself preserves it.
        NodeKind::Map { op, inputs } => {
            let up = join_all(inputs.iter().map(|n| structure(n, axis)));
            Structure::at(up.level, up.linear && op.preserves_linear())
        }

        NodeKind::Reduce { src, axis: red, op } => {
            let up = structure(src, axis);
            if *red != axis {
                // Reducing a different axis says nothing about this one.
                up
            } else if matches!(up.level, Parallelism::Sequential | Parallelism::Opaque) {
                // Poisoned upstream: propagate.
                up
            } else if op.is_monoid() {
                Structure::at(Parallelism::Monoidal, op.is_additive() && up.linear)
            } else {
                Structure::at(Parallelism::Sequential, false)
            }
        }

        NodeKind::Scan { src, axis: sc, op } => {
            if *sc != axis {
                structure(src, axis)
            } else if op.is_monoid() {
                let up = structure(src, axis);
                Structure::at(Parallelism::Monoidal, op.is_additive() && up.linear)
            } else {
                Structure::at(Parallelism::Sequential, false)
            }
        }

        NodeKind::Gather { src, axis: g, .. } => {
            let up = structure(src, axis);
            if *g == axis {
                up.join(Structure::at(Parallelism::Opaque, false))
            } else {
                up
            }
        }

        // The scoping operator. A grouped output axis inherits the join of
        // its members' structures; a source name a group consumed is out of
        // scope above the view (asking about it is asking about a variable
        // that no longer exists — FREE, like any absent axis); everything
        // else passes through.
        NodeKind::View { src, groups } => {
            if let Some((members, _)) = groups.iter().find(|(_, to)| *to == axis) {
                return join_all(members.iter().map(|m| structure(src, *m)));
            }
            if groups.iter().any(|(members, _)| members.contains(&axis)) {
                return Structure::FREE;
            }
            structure(src, axis)
        }

        // Affine reindexing scopes like a view: an output axis inherits the
        // join of every source axis whose index it drives (a windowed read is
        // still elementwise — shifting indices adds no cross-element
        // dependence, and a padded 0.0 is a constant); a mapped source axis
        // is out of scope above the node; the rest passes through.
        NodeKind::Reindex { src, map, .. } => {
            let driving: Vec<Axis> = map
                .iter()
                .filter(|(_, terms, _)| terms.iter().any(|(_, a)| *a == axis))
                .map(|(m, _, _)| *m)
                .collect();
            if !driving.is_empty() {
                return join_all(driving.into_iter().map(|m| structure(src, m)));
            }
            if map.iter().any(|(m, _, _)| *m == axis) {
                return Structure::FREE;
            }
            structure(src, axis)
        }
    }
}

/// Can this axis be folded in one pass? Yes iff FREE or MONOIDAL.
pub fn streamable(node: &NodeKind, axis: Axis) -> bool {
    matches!(
        structure(node, axis).level,
        Parallelism::Free | Parallelism::Monoidal
    )
}

// ── the structure map ────────────────────────────────────────────────────────

/// The verdict for one axis of a graph.
pub struct AxisReport {
    pub axis: Axis,
    pub structure: Structure,
    /// The streaming accumulator — present iff the axis folds *at this node*.
    /// FREE axes are grid dimensions; OPAQUE / SEQUENTIAL have no one-pass
    /// form; a MONOIDAL axis without a carrier folds in a sub-expression.
    pub carrier: Option<Carrier>,
}

/// The structure map: every axis classified, accumulators attached.
pub struct Report {
    pub axes: Vec<AxisReport>,
}

/// Classify the given axes and derive an accumulator wherever one folds.
pub fn analyze(node: &Node, axes: &[Axis]) -> Report {
    let axes = axes
        .iter()
        .map(|&a| {
            let structure = structure(node, a);
            let carrier = match structure.level {
                Parallelism::Monoidal => derive::derive(node, a),
                _ => None,
            };
            AxisReport {
                axis: a,
                structure,
                carrier,
            }
        })
        .collect();
    Report { axes }
}

/// Same, but the engine discovers the axes itself — the zero-config front door.
pub fn analyze_all(node: &Node) -> Report {
    analyze(node, &ir::all_axes(node))
}

impl Report {
    /// One line per axis — the tag and what it licenses — followed by the
    /// derived accumulator for the foldable ones.
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
                Parallelism::Monoidal if a.carrier.is_some() => "fold",
                Parallelism::Monoidal => "fold (in a sub-expression)",
                Parallelism::Opaque => "runtime gather",
                Parallelism::Sequential => "serial — no fold",
            };
            out += &format!("  {:<4} {:<18} → {}\n", a.axis.label(), tag, action);
            if let Some(c) = &a.carrier {
                for line in c.render().lines() {
                    out += &format!("         {line}\n");
                }
            }
        }
        out
    }
}
