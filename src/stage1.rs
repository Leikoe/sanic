//! §4 — Stage 1: structure classification.
//!
//! Abstract interpretation over a finite lattice. Each (node, axis) pair is
//! assigned a structure describing how the computation depends on that axis,
//! ordered from most to least parallel. The pass is total and terminating
//! (structural recursion over a DAG).

use crate::engine_ir::NodeKind;

/// The parallelism rung (§4.1), ordered FREE < MONOIDAL < OPAQUE < SEQUENTIAL.
/// `Ord` follows that order, so `join` is `max` (the *least* parallel wins).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Parallelism {
    /// No dependence along the axis → grid / DOALL.
    Free,
    /// Associative `⊕` with identity → stream or tree-reduce, O(1) state.
    Monoidal,
    /// Data-dependent access (gather) → runtime-determined.
    Opaque,
    /// Dependence with non-associative `⊕` → strict serial, depth = N.
    Sequential,
}

/// The structure tag (§4.1). Modelled as `(parallelism, is_linear)` rather than
/// a single rung, because `LINEAR` is a *tag* on a `MONOIDAL` classification —
/// not a distinct rung — and forcing a total order would be false.
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

    /// Join of two input structures (§4.1): take the max parallelism level (the
    /// least-parallel rung) and AND the linearity flags.
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

/// Classify how `node` depends on `axis` (§4.2). Computed by structural
/// recursion; the per-(node, axis) result is what distinguishes the two fusion
/// kinds of §5 (e.g. the middle axis of a double-GEMM is reduced in one
/// sub-expression and free in another).
pub fn structure(node: &NodeKind, axis: &str) -> Structure {
    match node {
        // Raw data has no dependence on any axis.
        NodeKind::Input { .. } => Structure::FREE,

        // Elementwise: passes the joined input structure through unchanged.
        // Linearity survives only if the map itself preserves it.
        NodeKind::Map { f, inputs } => {
            let up = join_all(inputs.iter().map(|n| structure(n, axis)));
            Structure::at(up.level, up.linear && f.preserves_linear)
        }

        NodeKind::Reduce { src, axis: red, op } => {
            let up = structure(src, axis);
            if *red != axis {
                // Reducing a *different* axis is per-`axis` independent.
                up
            } else if matches!(up.level, Parallelism::Sequential | Parallelism::Opaque) {
                // Poisoned upstream: propagate it.
                up
            } else if op.is_monoid() {
                // Linear iff the reduction op is semiring-additive AND the
                // summand is linear in the reduced quantity.
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
    }
}

/// §4.3 — the streamable verdict: an axis folds in one pass iff it is FREE or
/// MONOIDAL. (LINEAR is a MONOIDAL refinement, so it is included.)
pub fn streamable(node: &NodeKind, axis: &str) -> bool {
    matches!(
        structure(node, axis).level,
        Parallelism::Free | Parallelism::Monoidal
    )
}
