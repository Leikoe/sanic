//! Classification: how does the computation depend on each axis?
//!
//! One recursive pass assigns every (node, axis) a rung on a three-value
//! ladder, ordered from most to least parallel. FREE and MONOIDAL axes stream
//! and parallelize; OPAQUE axes require runtime indexing. `analyze_all`
//! packages the verdicts for a whole graph — every axis classified, with the
//! derived accumulator attached wherever one exists.

use crate::derive::{self, Carrier};
use crate::ir::{self, AxisRef, AxisSelector, Node as NodeKind, NodeRef as Node};

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
///
/// Memoized by pointer WITHIN one call: the IR is a DAG (a 56-layer chain
/// shares its whole prefix), so an unmemoized walk re-derives shared
/// subtrees once per path to them — the dominant cost of partitioning a
/// deep model. A fresh cache per call keeps it pure and stale-pointer-free.
pub fn structure(node: &Node, axis: impl AxisSelector) -> Structure {
    let Some(axis) = axis.resolve_axis(node, "structure") else {
        return Structure::FREE;
    };
    StructureCache::default().classify(node, axis)
}

/// Reusable per-graph classification cache for compiler passes that query
/// many `(node, dimension)` pairs from the same retained DAG.
#[derive(Default)]
pub(crate) struct StructureCache {
    entries: std::collections::HashMap<(*const NodeKind, AxisRef), Structure>,
    resolver: ir::Resolver,
}

impl StructureCache {
    pub(crate) fn classify(&mut self, node: &Node, axis: AxisRef) -> Structure {
        structure_memo(node, axis, self)
    }

    /// The positional resolver this cache memoizes with, for passes that
    /// resolve many occurrences over the same retained DAG.
    pub(crate) fn resolver(&mut self) -> &mut ir::Resolver {
        &mut self.resolver
    }
}

fn structure_memo(node: &Node, axis: AxisRef, cache: &mut StructureCache) -> Structure {
    let key = (std::rc::Rc::as_ptr(node), axis);
    if let Some(s) = cache.entries.get(&key) {
        return *s;
    }
    let s = structure_uncached(node, axis, cache);
    cache.entries.insert(key, s);
    s
}

fn structure_uncached(node: &Node, axis: AxisRef, cache: &mut StructureCache) -> Structure {
    match node.as_ref() {
        // Raw data, literals and index values depend on no axis (an Iota
        // varies *with* its axis, but elementwise — no cross-element
        // dependence, which is what FREE means).
        NodeKind::Input { .. }
        | NodeKind::Const { .. }
        | NodeKind::Iota { .. }
        | NodeKind::Coordinate { .. } => Structure::FREE,

        // Elementwise: pass the joined input structure through. Linearity
        // survives only if the op itself preserves it.
        NodeKind::Map { op, inputs } => {
            let up = join_all(inputs.iter().map(|input| {
                let input_axis = cache.resolver.map_input_axis(node, input, axis);
                structure_memo(input, input_axis, cache)
            }));
            Structure::at(up.level, up.linear && op.preserves_linear())
        }

        NodeKind::Reduce { src, dim, op } => {
            let red = cache.resolver.source_axis(src, *dim);
            let up = structure_memo(src, axis, cache);
            if red != axis {
                // Reducing a different axis says nothing about this one.
                up
            } else if matches!(up.level, Parallelism::Opaque) {
                // Poisoned upstream: propagate.
                up
            } else {
                Structure::at(Parallelism::Monoidal, op.is_additive() && up.linear)
            }
        }

        NodeKind::Scan { src, dim, op } => {
            let scanned = cache.resolver.source_axis(src, *dim);
            if scanned != axis {
                structure_memo(src, axis, cache)
            } else {
                let up = structure_memo(src, axis, cache);
                Structure::at(Parallelism::Monoidal, op.is_additive() && up.linear)
            }
        }

        NodeKind::Gather { src, dim, .. } => {
            let gathered = cache.resolver.source_axis(src, *dim);
            let up = structure_memo(src, axis, cache);
            if gathered == axis {
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
        NodeKind::View { src, .. } => {
            let groups = cache.resolver.view_groups(node);
            if let Some((members, _)) = groups.iter().find(|(_, to)| *to == axis) {
                return join_all(members.iter().map(|m| structure_memo(src, *m, cache)));
            }
            if groups.iter().any(|(members, _)| members.contains(&axis)) {
                return Structure::FREE;
            }
            structure_memo(src, axis, cache)
        }

        // Affine reindexing scopes like a view: an output axis inherits the
        // join of every source axis whose index it drives (a windowed read is
        // still elementwise — shifting indices adds no cross-element
        // dependence, and a padded 0.0 is a constant); a mapped source axis
        // is out of scope above the node; the rest passes through.
        NodeKind::Reindex { src, .. } => {
            let map = cache.resolver.resolved_reindex(node);
            let driving: Vec<AxisRef> = map
                .iter()
                .filter(|(_, terms, _)| terms.iter().any(|(_, a)| *a == axis))
                .map(|(m, _, _)| *m)
                .collect();
            if !driving.is_empty() {
                return join_all(driving.into_iter().map(|m| structure_memo(src, m, cache)));
            }
            if map.iter().any(|(m, _, _)| *m == axis) {
                return Structure::FREE;
            }
            structure_memo(src, axis, cache)
        }
    }
}

/// Can this axis be folded in one pass? Yes iff FREE or MONOIDAL.
pub fn streamable(node: &Node, axis: impl AxisSelector) -> bool {
    matches!(
        structure(node, axis).level,
        Parallelism::Free | Parallelism::Monoidal
    )
}

// ── the structure map ────────────────────────────────────────────────────────

/// The verdict for one axis of a graph.
pub struct AxisReport {
    pub axis: AxisRef,
    pub structure: Structure,
    /// The streaming accumulator — present iff the axis folds *at this node*.
    /// FREE axes are grid dimensions; OPAQUE axes have no one-pass form; a
    /// MONOIDAL axis without a carrier folds in a sub-expression.
    pub carrier: Option<Carrier>,
    /// Why a MONOIDAL axis nonetheless has no carrier at this node — the
    /// deriver's claim, kept so the report can say it and a census can
    /// bucket it.
    pub decline: Option<derive::Decline>,
}

/// The structure map: every axis classified, accumulators attached.
pub struct Report {
    pub axes: Vec<AxisReport>,
}

/// Classify the given axes and derive an accumulator wherever one folds.
pub fn analyze<A: AxisSelector>(node: &Node, axes: &[A]) -> Report {
    let mut structures = StructureCache::default();
    let axes = axes
        .iter()
        .map(|&selector| {
            let a = selector
                .resolve_axis(node, "analyze")
                .expect("analyze axis is absent from the selected node");
            let structure = structures.classify(node, a);
            let (carrier, decline) = match structure.level {
                Parallelism::Monoidal => {
                    match derive::derive_with_structure_cache(node, a, &mut structures) {
                        Ok(c) => (Some(c), None),
                        Err(d) => (None, Some(d)),
                    }
                }
                _ => (None, None),
            };
            AxisReport {
                axis: a,
                structure,
                carrier,
                decline,
            }
        })
        .collect();
    Report { axes }
}

/// Same, but the engine discovers the axes itself — the zero-config front door.
pub fn analyze_all(node: &Node) -> Report {
    analyze(node, &ir::all_axis_refs(node))
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
            };
            let action = match a.structure.level {
                Parallelism::Free => "grid (DOALL)",
                Parallelism::Monoidal if a.carrier.is_some() => "fold",
                Parallelism::Monoidal => "fold (in a sub-expression)",
                Parallelism::Opaque => "runtime gather",
            };
            out += &format!("  {:<4} {:<18} → {}\n", a.axis.name, tag, action);
            if let Some(c) = &a.carrier {
                for line in c.render().lines() {
                    out += &format!("         {line}\n");
                }
            }
            if let Some(d) = &a.decline {
                out += &format!("         {d}\n");
            }
        }
        out
    }
}
