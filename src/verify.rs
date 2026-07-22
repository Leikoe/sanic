//! Structural verification for the positional tensor IR.
//!
//! Constructors reject malformed graphs locally, but [`Node`] remains public
//! so tests and tooling can build raw nodes. This bottom-up pass validates the
//! invariants analysis, scheduling, and code generation rely on without
//! assigning graph-global meaning to axis descriptors.

use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::ir::{AffineIndex, Axis, Dtype, Extent, Node as NodeKind, NodeRef as Node, ViewDim};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyError {
    pub node_index: usize,
    pub op: &'static str,
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

/// Relative drift permitted when only accumulation order changes.
pub fn rel_tolerance(dtype: Dtype, terms: usize) -> f64 {
    let eps = match dtype {
        Dtype::F64 => f64::EPSILON,
        Dtype::F32 => f32::EPSILON as f64,
        Dtype::F16 => 9.77e-4,
        Dtype::BF16 => 7.82e-3,
        Dtype::I8 | Dtype::I4 => 0.0,
    };
    64.0 * eps * terms.max(1) as f64
}

pub fn verify(node: &Node) -> Result<(), VerifyError> {
    verify_many(std::slice::from_ref(node))
}

pub fn verify_many(roots: &[Node]) -> Result<(), VerifyError> {
    let mut verifier = Verifier::default();
    for root in roots {
        verifier.visit(root)?;
    }
    Ok(())
}

#[track_caller]
pub fn assert_valid(node: &Node) {
    if let Err(error) = verify(node) {
        panic!("{error}");
    }
}

#[track_caller]
pub fn assert_valid_many(roots: &[Node]) {
    if let Err(error) = verify_many(roots) {
        panic!("{error}");
    }
}

#[derive(Clone)]
struct InputDecl {
    node_index: usize,
    shape: Vec<Axis>,
    dtype: Dtype,
}

#[derive(Default)]
struct Verifier {
    shapes: HashMap<*const NodeKind, Rc<Vec<Axis>>>,
    inputs: HashMap<&'static str, InputDecl>,
    next_node: usize,
}

impl Verifier {
    fn visit(&mut self, node: &Node) -> Result<Rc<Vec<Axis>>, VerifyError> {
        let pointer = Rc::as_ptr(node);
        if let Some(shape) = self.shapes.get(&pointer) {
            return Ok(shape.clone());
        }

        let child_shapes = match node.as_ref() {
            NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => Vec::new(),
            NodeKind::Coordinate { src, .. }
            | NodeKind::Reduce { src, .. }
            | NodeKind::Scan { src, .. }
            | NodeKind::View { src, .. }
            | NodeKind::Reindex { src, .. } => vec![self.visit(src)?],
            NodeKind::Map { inputs, .. } => inputs
                .iter()
                .map(|input| self.visit(input))
                .collect::<Result<Vec<_>, _>>()?,
            NodeKind::Gather { src, index, .. } => vec![self.visit(src)?, self.visit(index)?],
        };

        let node_index = self.next_node;
        self.next_node += 1;
        let op = op_name(node);
        let fail = |reason: String| VerifyError {
            node_index,
            op,
            reason,
        };

        let shape = match node.as_ref() {
            NodeKind::Input { name, shape, dtype } => {
                if name.is_empty() {
                    return Err(fail("input name cannot be empty".into()));
                }
                if let Some(previous) = self.inputs.get(name) {
                    let compatible = previous.shape.len() == shape.len()
                        && previous
                            .shape
                            .iter()
                            .zip(shape)
                            .all(|(left, right)| left.extent == right.extent);
                    if !compatible {
                        return Err(fail(format!(
                            "input `{name}` conflicts with node {}: positional extents differ",
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
                            shape: shape.clone(),
                            dtype: *dtype,
                        },
                    );
                }
                shape.clone()
            }
            NodeKind::Const { .. } => Vec::new(),
            NodeKind::Iota { axis } => vec![*axis],
            NodeKind::Coordinate { dim, .. } => {
                check_dim(*dim, &child_shapes[0], "coordinate").map_err(&fail)?;
                (*child_shapes[0]).clone()
            }
            NodeKind::Map { op: map_op, inputs } => {
                if inputs.len() != map_op.arity() {
                    return Err(fail(format!(
                        "{} expects {} inputs, received {}",
                        map_op.name(),
                        map_op.arity(),
                        inputs.len()
                    )));
                }
                broadcast_shape(&child_shapes).map_err(&fail)?
            }
            NodeKind::Reduce { dim, .. } => {
                check_dim(*dim, &child_shapes[0], "reduce").map_err(&fail)?;
                let mut shape = (*child_shapes[0]).clone();
                shape.remove(*dim);
                shape
            }
            NodeKind::Scan { dim, .. } => {
                check_dim(*dim, &child_shapes[0], "scan").map_err(&fail)?;
                (*child_shapes[0]).clone()
            }
            NodeKind::Gather { dim, .. } => {
                check_dim(*dim, &child_shapes[0], "gather").map_err(&fail)?;
                let mut shape = Vec::new();
                shape.extend_from_slice(&child_shapes[0][..*dim]);
                shape.extend_from_slice(&child_shapes[1]);
                shape.extend_from_slice(&child_shapes[0][*dim + 1..]);
                shape
            }
            NodeKind::View { dims, .. } => {
                check_view(&child_shapes[0], dims).map_err(&fail)?;
                dims.iter().map(|dim| dim.axis).collect()
            }
            NodeKind::Reindex {
                shape, map, padded, ..
            } => {
                check_reindex(&child_shapes[0], shape, map, *padded).map_err(&fail)?;
                shape.clone()
            }
        };

        check_shape(&shape).map_err(&fail)?;
        let shape = Rc::new(shape);
        self.shapes.insert(pointer, shape.clone());
        Ok(shape)
    }
}

fn check_dim(dim: usize, shape: &[Axis], op: &str) -> Result<(), String> {
    if dim >= shape.len() {
        Err(format!(
            "{op} dimension {dim} is out of range for rank {}",
            shape.len()
        ))
    } else {
        Ok(())
    }
}

fn broadcast_shape(shapes: &[Rc<Vec<Axis>>]) -> Result<Vec<Axis>, String> {
    let rank = shapes.iter().map(|shape| shape.len()).max().unwrap_or(0);
    let mut output = Vec::with_capacity(rank);
    for output_dim in 0..rank {
        let mut selected: Option<Axis> = None;
        for shape in shapes {
            let Some(input_dim) = output_dim.checked_sub(rank - shape.len()) else {
                continue;
            };
            let candidate = shape[input_dim];
            selected = Some(match selected {
                None => candidate,
                Some(current) => match (current.extent, candidate.extent) {
                    (Extent::Static(1), _) => candidate,
                    (_, Extent::Static(1)) => current,
                    (left, right) if left == right => current,
                    _ => {
                        return Err(format!(
                            "map dimensions with extents {:?} and {:?} cannot broadcast",
                            current.extent, candidate.extent
                        ));
                    }
                },
            });
        }
        output.push(selected.ok_or_else(|| "map output dimension has no source".to_string())?);
    }
    Ok(output)
}

fn check_view(source: &[Axis], dims: &[ViewDim]) -> Result<(), String> {
    let mut consumed = vec![false; source.len()];
    for dim in dims {
        if dim.sources.is_empty() && dim.axis.extent != Extent::Static(1) {
            return Err("a source-free view dimension must have extent one".into());
        }
        let mut product = Some(1usize);
        for &source_dim in &dim.sources {
            check_dim(source_dim, source, "view source")?;
            if std::mem::replace(&mut consumed[source_dim], true) {
                return Err(format!(
                    "view source dimension {source_dim} is consumed more than once"
                ));
            }
            product = match (product, source[source_dim].extent) {
                (Some(acc), Extent::Static(extent)) => acc.checked_mul(extent),
                _ => None,
            };
        }
        if let (Some(expected), Extent::Static(actual)) = (product, dim.axis.extent)
            && expected != actual
        {
            return Err(format!(
                "view output extent {actual} does not match grouped source extent {expected}"
            ));
        }
    }
    if let Some(missing) = consumed.iter().position(|consumed| !consumed) {
        return Err(format!("view source dimension {missing} is not consumed"));
    }
    Ok(())
}

fn check_reindex(
    source: &[Axis],
    output: &[Axis],
    map: &[AffineIndex],
    padded: bool,
) -> Result<(), String> {
    if map.len() != source.len() {
        return Err(format!(
            "reindex maps {} source dimensions; expected {}",
            map.len(),
            source.len()
        ));
    }
    let mut seen = vec![false; source.len()];
    for (source_dim, terms, offset) in map {
        check_dim(*source_dim, source, "reindex source")?;
        if std::mem::replace(&mut seen[*source_dim], true) {
            return Err(format!(
                "reindex source dimension {source_dim} is mapped more than once"
            ));
        }
        for &(_, output_dim) in terms {
            check_dim(output_dim, output, "reindex output")?;
        }
        if !padded
            && let Some((minimum, maximum)) = affine_bounds(terms, *offset, output)?
            && let Extent::Static(extent) = source[*source_dim].extent
            && (minimum < 0 || maximum >= extent as i128)
        {
            return Err(format!(
                "reindex of source dimension {source_dim} reaches [{minimum}, {maximum}], outside [0, {extent}) without padding"
            ));
        }
    }
    Ok(())
}

fn affine_bounds(
    terms: &[(i64, usize)],
    offset: i64,
    output: &[Axis],
) -> Result<Option<(i128, i128)>, String> {
    let (mut minimum, mut maximum) = (offset as i128, offset as i128);
    let mut coefficients = HashMap::<usize, i128>::new();
    for &(coefficient, output_dim) in terms {
        let combined = coefficients.entry(output_dim).or_default();
        *combined = combined
            .checked_add(coefficient as i128)
            .ok_or_else(|| "affine coefficient overflows i128".to_string())?;
    }
    for (output_dim, coefficient) in coefficients {
        let Extent::Static(extent) = output[output_dim].extent else {
            return Ok(None);
        };
        let delta = coefficient
            .checked_mul((extent - 1) as i128)
            .ok_or_else(|| "affine index range overflows i128".to_string())?;
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

fn check_shape(shape: &[Axis]) -> Result<(), String> {
    let mut volume = 1usize;
    for (dim, axis) in shape.iter().enumerate() {
        match axis.extent {
            Extent::Static(0) => {
                return Err(format!("dimension {dim} has zero extent"));
            }
            Extent::Static(extent) => {
                volume = volume.checked_mul(extent).ok_or_else(|| {
                    format!("static output volume overflows usize at dimension {dim}")
                })?;
            }
            Extent::Dynamic => {}
        }
    }
    Ok(())
}

fn op_name(node: &Node) -> &'static str {
    match node.as_ref() {
        NodeKind::Input { .. } => "Input",
        NodeKind::Const { .. } => "Const",
        NodeKind::Iota { .. } => "Iota",
        NodeKind::Coordinate { .. } => "Coordinate",
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
    use crate::ir::{MapOp, axis, input, konst};

    #[test]
    fn rejects_bad_map_arity() {
        let invalid = Rc::new(NodeKind::Map {
            op: MapOp::Add,
            inputs: vec![konst(1.0)],
        });
        assert!(
            verify(&invalid)
                .unwrap_err()
                .reason
                .contains("expects 2 inputs")
        );
    }

    #[test]
    fn axis_descriptors_are_not_dimension_identities() {
        let n = axis("n", 4);
        assert_eq!(verify(&input("X", [n, n], Dtype::F32)), Ok(()));
    }

    #[test]
    fn rejects_out_of_range_positional_dimension() {
        let n = axis("n", 4);
        let invalid = Rc::new(NodeKind::Reduce {
            src: input("X", [n], Dtype::F32),
            dim: 1,
            op: crate::ir::Monoid::Add,
        });
        assert!(
            verify(&invalid)
                .unwrap_err()
                .reason
                .contains("out of range")
        );
    }
}
