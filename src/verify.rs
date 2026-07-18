//! Structural verification for the IR.
//!
//! tinygrad verifies every UOp against a declarative spec before scheduling
//! and again after lowering. Sanic has a much smaller IR, so its equivalent is a
//! single bottom-up pass: infer each node's axes while checking the invariants
//! later passes rely on. This keeps malformed graphs from becoming mysterious
//! interpreter bounds errors or compiler failures.
//!
//! Verification is deliberately separate from construction. The public
//! [`NodeKind`] enum can represent an invalid graph long enough for [`verify`]
//! to return a useful [`VerifyError`]. Public pipeline boundaries call
//! [`assert_valid`] or [`assert_valid_many`] before doing work.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

use crate::ir::{AffineIndex, Axis, AxisName, BinOp, Dtype, Extent, Node, NodeKind};

/// The first node that violates the current IR well-formedness rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyError {
    /// Postorder index of the node, matching the order in which verification
    /// visits a DAG.
    pub node_index: usize,
    /// IR node kind.
    pub op: &'static str,
    /// The violated invariant.
    pub reason: String,
}

impl fmt::Display for VerifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IR verification failed at node {} ({}) — {}",
            self.node_index, self.op, self.reason
        )
    }
}

impl std::error::Error for VerifyError {}

/// Verify one IR root.
pub fn verify(node: &Node) -> Result<(), VerifyError> {
    verify_many(std::slice::from_ref(node))
}

/// Verify several roots as one graph, including consistency between shared
/// input-buffer declarations reached from different roots.
pub fn verify_many(roots: &[Node]) -> Result<(), VerifyError> {
    let mut verifier = Verifier::default();
    for root in roots {
        verifier.visit(root)?;
    }
    Ok(())
}

/// Panic with a diagnostic [`VerifyError`] when one IR root is malformed.
#[track_caller]
pub fn assert_valid(node: &Node) {
    if let Err(error) = verify(node) {
        panic!("{error}");
    }
}

/// Panic with a diagnostic [`VerifyError`] when any of several IR roots is
/// malformed.
#[track_caller]
pub fn assert_valid_many(roots: &[Node]) {
    if let Err(error) = verify_many(roots) {
        panic!("{error}");
    }
}

#[derive(Clone)]
struct InputDecl {
    node_index: usize,
    axes: Vec<Axis>,
    dtype: Option<Dtype>,
}

#[derive(Default)]
struct Verifier {
    shapes: HashMap<*const NodeKind, Rc<Vec<Axis>>>,
    inputs: HashMap<&'static str, InputDecl>,
    axes: HashMap<Axis, (AxisName, Extent)>,
    next_node: usize,
}

impl Verifier {
    fn visit(&mut self, node: &Node) -> Result<Rc<Vec<Axis>>, VerifyError> {
        let pointer = Rc::as_ptr(node);
        if let Some(shape) = self.shapes.get(&pointer) {
            return Ok(shape.clone());
        }

        // Children come first, so `node_index` is a stable topological
        // (postorder) position like tinygrad's type_verify diagnostics.
        let shape = match node.as_ref() {
            NodeKind::Input { axes, .. } => axes.clone(),
            NodeKind::Const { .. } => Vec::new(),
            NodeKind::Iota { axis } => vec![*axis],
            NodeKind::Map { inputs, .. } => {
                let mut shape = Vec::new();
                for input in inputs {
                    let input_shape = self.visit(input)?;
                    union_axes(&mut shape, &input_shape);
                }
                shape
            }
            NodeKind::Reduce { src, axis, .. } => self
                .visit(src)?
                .iter()
                .copied()
                .filter(|candidate| candidate != axis)
                .collect(),
            NodeKind::Scan { src, .. } => (*self.visit(src)?).clone(),
            NodeKind::Gather { src, index, axis } => {
                let source_shape = self.visit(src)?;
                let index_shape = self.visit(index)?;
                let mut shape: Vec<Axis> = source_shape
                    .iter()
                    .copied()
                    .filter(|candidate| candidate != axis)
                    .collect();
                union_axes(&mut shape, &index_shape);
                shape
            }
            NodeKind::View { src, groups } => {
                let source_shape = self.visit(src)?;
                let mut shape = Vec::new();
                for source_axis in source_shape.iter().copied() {
                    if let Some((_, target)) = groups
                        .iter()
                        .find(|(members, _)| members.contains(&source_axis))
                    {
                        push_axis(&mut shape, *target);
                    } else {
                        push_axis(&mut shape, source_axis);
                    }
                }
                shape
            }
            NodeKind::Reindex { src, map, .. } => {
                let source_shape = self.visit(src)?;
                let mut shape = Vec::new();
                for source_axis in source_shape.iter().copied() {
                    if let Some((_, terms, _)) =
                        map.iter().find(|(mapped, _, _)| *mapped == source_axis)
                    {
                        for (_, output_axis) in terms {
                            push_axis(&mut shape, *output_axis);
                        }
                    } else {
                        push_axis(&mut shape, source_axis);
                    }
                }
                shape
            }
        };

        let node_index = self.next_node;
        self.next_node += 1;
        let op = op_name(node);
        self.check_node(node, node_index, op)?;
        self.check_shape(&shape, node_index, op)?;

        let shape = Rc::new(shape);
        self.shapes.insert(pointer, shape.clone());
        Ok(shape)
    }

    fn check_node(
        &mut self,
        node: &Node,
        node_index: usize,
        op: &'static str,
    ) -> Result<(), VerifyError> {
        let fail = |reason: String| VerifyError {
            node_index,
            op,
            reason,
        };

        match node.as_ref() {
            NodeKind::Input {
                name, axes, dtype, ..
            } => {
                for &axis in axes {
                    self.check_axis(axis, node_index, op)?;
                }
                if let Some(previous) = self.inputs.get(name) {
                    let same_axes = sorted_axes(&previous.axes) == sorted_axes(axes);
                    let same_positional_shape = previous.axes.len() == axes.len()
                        && previous
                            .axes
                            .iter()
                            .zip(axes)
                            .all(|(left, right)| left.extent == right.extent);
                    if !same_axes && !same_positional_shape {
                        return Err(fail(format!(
                            "input `{name}` conflicts with node {}: declarations must use \
                             the same axes or the same positional extents",
                            previous.node_index
                        )));
                    }
                    if previous.dtype != *dtype {
                        return Err(fail(format!(
                            "input `{name}` conflicts with node {}: storage dtype {:?} != {:?}",
                            previous.node_index, previous.dtype, dtype
                        )));
                    }
                } else {
                    self.inputs.insert(
                        name,
                        InputDecl {
                            node_index,
                            axes: axes.clone(),
                            dtype: *dtype,
                        },
                    );
                }
            }
            NodeKind::Const { .. } => {}
            NodeKind::Iota { axis } => self.check_axis(*axis, node_index, op)?,
            NodeKind::Map { op: map_op, inputs } => {
                if inputs.len() != map_op.arity() {
                    return Err(fail(format!(
                        "{} expects {} inputs, received {}",
                        map_op.name(),
                        map_op.arity(),
                        inputs.len()
                    )));
                }
            }
            NodeKind::Reduce {
                axis,
                op: reduction,
                ..
            } => {
                self.check_axis(*axis, node_index, op)?;
                // Unlike Scan and Gather, Reduce may bind a fresh axis.
                // `Reduce(Const(1), n, Add)` is sanic's count primitive, and
                // folding an invariant over `n` deliberately repeats it
                // extent(n) times.
                if let BinOp::TopK { k, rank, .. } = reduction {
                    if *k == 0 {
                        return Err(fail("TopK requires k >= 1".into()));
                    }
                    if rank >= k {
                        return Err(fail(format!("TopK rank {rank} is outside k={k}")));
                    }
                }
            }
            NodeKind::Scan { src, axis, .. } => {
                self.check_axis(*axis, node_index, op)?;
                let source_shape = self.shape(src);
                if !source_shape.contains(axis) {
                    return Err(fail(format!(
                        "scan axis {axis:?} is not present in source axes {source_shape:?}"
                    )));
                }
            }
            NodeKind::Gather { src, axis, .. } => {
                self.check_axis(*axis, node_index, op)?;
                let source_shape = self.shape(src);
                if !source_shape.contains(axis) {
                    return Err(fail(format!(
                        "gather axis {axis:?} is not present in source axes {source_shape:?}"
                    )));
                }
            }
            NodeKind::View { src, groups } => {
                self.check_view(src, groups, node_index, op)?;
            }
            NodeKind::Reindex { src, map, padded } => {
                self.check_reindex(src, map, *padded, node_index, op)?;
            }
        }
        Ok(())
    }

    fn check_view(
        &mut self,
        src: &Node,
        groups: &[(Vec<Axis>, Axis)],
        node_index: usize,
        op: &'static str,
    ) -> Result<(), VerifyError> {
        let fail = |reason: String| VerifyError {
            node_index,
            op,
            reason,
        };
        let source_shape = self.shape(src);
        let mut consumed = HashSet::new();
        let mut targets = HashSet::new();

        for (members, target) in groups {
            self.check_axis(*target, node_index, op)?;
            if members.is_empty() {
                return Err(fail(format!(
                    "view target {target:?} has an empty source group"
                )));
            }
            if !targets.insert(*target) {
                return Err(fail(format!(
                    "view target {target:?} is produced by more than one group"
                )));
            }
            if source_shape.contains(target)
                && !(members.len() == 1 && members.first() == Some(target))
            {
                return Err(fail(format!(
                    "view target {target:?} aliases an existing source axis; \
                     targets must be fresh"
                )));
            }

            let mut product = Some(1usize);
            for &member in members {
                self.check_axis(member, node_index, op)?;
                if !source_shape.contains(&member) {
                    return Err(fail(format!(
                        "view source axis {member:?} is not present in source axes {source_shape:?}"
                    )));
                }
                if !consumed.insert(member) {
                    return Err(fail(format!(
                        "view source axis {member:?} is consumed more than once"
                    )));
                }
                product = match (product, member.extent) {
                    (Some(acc), Extent::Static(extent)) => {
                        Some(acc.checked_mul(extent).ok_or_else(|| {
                            fail(format!(
                                "grouped extent overflows usize at source axis {member:?}"
                            ))
                        })?)
                    }
                    _ => None,
                };
            }
            if let (Some(expected), Extent::Static(actual)) = (product, target.extent)
                && expected != actual
            {
                return Err(fail(format!(
                    "view target {target:?} has extent {actual}; grouped source extent is {expected}"
                )));
            }
        }
        Ok(())
    }

    fn check_reindex(
        &mut self,
        src: &Node,
        map: &[AffineIndex],
        padded: bool,
        node_index: usize,
        op: &'static str,
    ) -> Result<(), VerifyError> {
        let fail = |reason: String| VerifyError {
            node_index,
            op,
            reason,
        };
        let source_shape = self.shape(src);
        let mut mapped_axes = HashSet::new();

        for (source_axis, terms, offset) in map {
            self.check_axis(*source_axis, node_index, op)?;
            if !source_shape.contains(source_axis) {
                return Err(fail(format!(
                    "reindex source axis {source_axis:?} is not present in source axes \
                     {source_shape:?}"
                )));
            }
            if !mapped_axes.insert(*source_axis) {
                return Err(fail(format!(
                    "reindex source axis {source_axis:?} is mapped more than once"
                )));
            }
            for &(_, output_axis) in terms {
                self.check_axis(output_axis, node_index, op)?;
            }

            if !padded {
                let bounds = affine_bounds(terms, *offset).map_err(&fail)?;
                if let (Extent::Static(source_extent), Some((minimum, maximum))) =
                    (source_axis.extent, bounds)
                {
                    if minimum < 0 || maximum >= source_extent as i128 {
                        return Err(fail(format!(
                            "reindex of {source_axis:?} reaches [{minimum}, {maximum}], outside \
                             [0, {source_extent}) without padding"
                        )));
                    }
                }
            }
        }
        Ok(())
    }

    fn check_shape(
        &mut self,
        shape: &[Axis],
        node_index: usize,
        op: &'static str,
    ) -> Result<(), VerifyError> {
        let mut seen = HashSet::new();
        let mut volume = Some(1usize);
        for &axis in shape {
            self.check_axis(axis, node_index, op)?;
            if !seen.insert(axis) {
                return Err(VerifyError {
                    node_index,
                    op,
                    reason: format!("output axis {axis:?} occurs more than once"),
                });
            }
            volume = match (volume, axis.extent) {
                (Some(acc), Extent::Static(extent)) => {
                    Some(acc.checked_mul(extent).ok_or_else(|| VerifyError {
                        node_index,
                        op,
                        reason: format!("static output volume overflows usize at axis {axis:?}"),
                    })?)
                }
                _ => None,
            };
        }
        Ok(())
    }

    fn check_axis(
        &mut self,
        axis: Axis,
        node_index: usize,
        op: &'static str,
    ) -> Result<(), VerifyError> {
        if axis.extent == Extent::Static(0) {
            return Err(VerifyError {
                node_index,
                op,
                reason: format!(
                    "axis {axis:?} has zero extent, which the dense IR cannot represent"
                ),
            });
        }
        if let Some(&(name, extent)) = self.axes.get(&axis) {
            if name != axis.name || extent != axis.extent {
                return Err(VerifyError {
                    node_index,
                    op,
                    reason: format!(
                        "axis identity {axis:?} has inconsistent metadata: \
                         ({name:?}, {extent:?}) != ({:?}, {:?})",
                        axis.name, axis.extent
                    ),
                });
            }
        } else {
            self.axes.insert(axis, (axis.name, axis.extent));
        }
        Ok(())
    }

    fn shape(&self, node: &Node) -> Rc<Vec<Axis>> {
        self.shapes[&Rc::as_ptr(node)].clone()
    }
}

fn affine_bounds(terms: &[(i64, Axis)], offset: i64) -> Result<Option<(i128, i128)>, String> {
    let (mut minimum, mut maximum) = (offset as i128, offset as i128);
    // Equal output axes are correlated, so combine their coefficients before
    // taking a box bound (`i - i` is exactly zero, not `[-n, n]`).
    let mut coefficients: HashMap<Axis, i128> = HashMap::new();
    for &(coefficient, axis) in terms {
        let combined = coefficients.entry(axis).or_default();
        *combined = combined
            .checked_add(coefficient as i128)
            .ok_or_else(|| format!("affine coefficient overflows i128 at axis {axis:?}"))?;
    }
    for (axis, coefficient) in coefficients {
        let Extent::Static(extent) = axis.extent else {
            return Ok(None);
        };
        let delta = coefficient
            .checked_mul((extent - 1) as i128)
            .ok_or_else(|| format!("affine index range overflows i128 at axis {axis:?}"))?;
        if delta < 0 {
            minimum = minimum
                .checked_add(delta)
                .ok_or_else(|| "affine index minimum overflows i128".to_string())?;
        } else {
            maximum = maximum
                .checked_add(delta)
                .ok_or_else(|| "affine index maximum overflows i128".to_string())?;
        }
    }
    Ok(Some((minimum, maximum)))
}

fn push_axis(axes: &mut Vec<Axis>, axis: Axis) {
    if !axes.contains(&axis) {
        axes.push(axis);
    }
}

fn union_axes(axes: &mut Vec<Axis>, more: &[Axis]) {
    for &axis in more {
        push_axis(axes, axis);
    }
}

fn sorted_axes(axes: &[Axis]) -> Vec<Axis> {
    let mut sorted = axes.to_vec();
    sorted.sort();
    sorted
}

fn op_name(node: &Node) -> &'static str {
    match node.as_ref() {
        NodeKind::Input { .. } => "Input",
        NodeKind::Const { .. } => "Const",
        NodeKind::Iota { .. } => "Iota",
        NodeKind::Map { .. } => "Map",
        NodeKind::Reduce { .. } => "Reduce",
        NodeKind::Scan { .. } => "Scan",
        NodeKind::Gather { .. } => "Gather",
        NodeKind::View { .. } => "View",
        NodeKind::Reindex { .. } => "Reindex",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{
        MapOp, Monoid, axis, input, input_dt, konst, map, reduce, reindex, rename, view,
    };

    fn add() -> BinOp {
        BinOp::Monoid(Monoid::Add)
    }

    #[test]
    fn accepts_a_valid_composed_graph() {
        let (q, k, d, e) = (axis("q", 3), axis("k", 5), axis("d", 4), axis("e", 2));
        let graph = crate::ir::attention(
            input("Q", &[q, d]),
            input("K", &[k, d]),
            input("V", &[k, e]),
            d,
            k,
        );
        assert_eq!(verify(&graph), Ok(()));
    }

    #[test]
    fn checks_map_arity() {
        let invalid = Rc::new(NodeKind::Map {
            op: MapOp::Add,
            inputs: vec![konst(1.0)],
        });
        let error = verify(&invalid).unwrap_err();
        assert_eq!(error.op, "Map");
        assert!(error.reason.contains("expects 2 inputs"));
    }

    #[test]
    fn checks_axis_scope_and_topk_arguments() {
        let n = axis("n", 4);
        let missing = axis("missing", 4);
        let count = reduce(konst(1.0), missing, add());
        assert_eq!(verify(&count), Ok(()));

        let invalid = crate::ir::scan(input("X", &[n]), missing, add());
        assert!(verify(&invalid).unwrap_err().reason.contains("not present"));

        let invalid = reduce(
            input("X", &[n]),
            n,
            BinOp::TopK {
                k: 2,
                rank: 2,
                idx: false,
            },
        );
        assert!(verify(&invalid).unwrap_err().reason.contains("outside k=2"));
    }

    #[test]
    fn checks_input_axes_and_storage_declarations() {
        let n = axis("n", 4);
        let duplicate_axis = input("X", &[n, n]);
        assert!(
            verify(&duplicate_axis)
                .unwrap_err()
                .reason
                .contains("occurs more than once")
        );

        let m = axis("m", 4);
        let compatible_alias = map(MapOp::Add, vec![input("X", &[n]), input("X", &[m])]);
        assert_eq!(verify(&compatible_alias), Ok(()));

        let conflicting_dtype = map(
            MapOp::Add,
            vec![
                input_dt("X", &[n], Dtype::F16),
                input_dt("X", &[m], Dtype::F32),
            ],
        );
        assert!(
            verify(&conflicting_dtype)
                .unwrap_err()
                .reason
                .contains("storage dtype")
        );
    }

    #[test]
    fn checks_view_groups_and_extents() {
        let (a, b, flat) = (axis("a", 2), axis("b", 3), axis("flat", 5));
        let wrong_extent = view(input("X", &[a, b]), vec![(vec![a, b], flat)]);
        assert!(
            verify(&wrong_extent)
                .unwrap_err()
                .reason
                .contains("grouped source extent is 6")
        );

        let target_collision = rename(input("X", &[a, b]), a, b);
        assert!(
            verify(&target_collision)
                .unwrap_err()
                .reason
                .contains("targets must be fresh")
        );
    }

    #[test]
    fn proves_unpadded_affine_indices_are_in_bounds() {
        let (source, output) = (axis("source", 5), axis("output", 4));
        let invalid = reindex(
            input("X", &[source]),
            vec![(source, vec![(1, output)], 2)],
            false,
        );
        assert!(
            verify(&invalid)
                .unwrap_err()
                .reason
                .contains("outside [0, 5)")
        );

        let padded = reindex(
            input("X", &[source]),
            vec![(source, vec![(1, output)], 2)],
            true,
        );
        assert_eq!(verify(&padded), Ok(()));

        let reversed = reindex(
            input("X", &[source]),
            vec![(source, vec![(-1, source)], 4)],
            false,
        );
        assert_eq!(verify(&reversed), Ok(()));

        let singleton = axis("singleton", 1);
        let correlated = reindex(
            input("Y", &[singleton]),
            vec![(singleton, vec![(1, output), (-1, output)], 0)],
            false,
        );
        assert_eq!(verify(&correlated), Ok(()));
    }

    #[test]
    fn errors_include_the_topological_node_position() {
        let invalid = Rc::new(NodeKind::Map {
            op: MapOp::Add,
            inputs: vec![konst(1.0)],
        });
        let error = verify(&invalid).unwrap_err();
        assert_eq!(error.node_index, 1);
        assert!(error.to_string().contains("node 1 (Map)"));
    }
}
