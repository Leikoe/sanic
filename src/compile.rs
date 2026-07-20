//! Root-based compilation and execution.
//!
//! The public graph is positional. Compilation lowers it into the existing
//! algebraic kernel IR using axes minted inside this one compilation, then
//! verifies, derives, partitions, and prepares a backend executable.

use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use crate::cost;
use crate::interp::{Env, Value};
use crate::ir::{Axis, Dtype, Extent, Node, NodeRef};
use crate::kernel_ir as kir;
use crate::partition::{Schedule, partition_many};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileError {
    EmptyOutputs,
    DynamicShapesNotYetSupported,
    InvalidInput(String),
    InvalidGraph(String),
    Backend(String),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompileError::EmptyOutputs => f.write_str("cannot compile an empty output set"),
            CompileError::DynamicShapesNotYetSupported => {
                f.write_str("compiling dynamic shapes is not supported yet")
            }
            CompileError::InvalidInput(reason) => write!(f, "invalid input declaration: {reason}"),
            CompileError::InvalidGraph(reason) => write!(f, "invalid graph: {reason}"),
            CompileError::Backend(reason) => write!(f, "backend compilation failed: {reason}"),
        }
    }
}

impl Error for CompileError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunError {
    MissingInput(String),
    UnknownInput(String),
    DuplicateInput(String),
    Shape {
        name: String,
        expected: Vec<usize>,
        actual: Vec<usize>,
    },
    Dtype {
        name: String,
        expected: Dtype,
        actual: Dtype,
    },
    Feedback(String),
    Backend(String),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunError::MissingInput(name) => write!(f, "input `{name}` was not bound"),
            RunError::UnknownInput(name) => write!(f, "program has no input named `{name}`"),
            RunError::DuplicateInput(name) => write!(f, "input `{name}` was bound more than once"),
            RunError::Shape {
                name,
                expected,
                actual,
            } => write!(
                f,
                "input `{name}` has shape {actual:?}; expected {expected:?}"
            ),
            RunError::Dtype {
                name,
                expected,
                actual,
            } => write!(
                f,
                "input `{name}` has dtype {actual:?}; expected {expected:?}"
            ),
            RunError::Feedback(reason) => write!(f, "feedback wiring: {reason}"),
            RunError::Backend(reason) => write!(f, "backend execution failed: {reason}"),
        }
    }
}

impl Error for RunError {}

/// A backend-specific shaped buffer.
pub trait Buffer {
    fn shape(&self) -> &[usize];
    fn dtype(&self) -> Dtype;
}

/// A compile-and-run backend. Only CPU and Metal implement this in 1.0.
pub trait Backend: Clone + private::Sealed + 'static {
    type Buffer: Buffer;
    type Executable;

    fn profile(&self) -> cost::DeviceProfile;
    fn prepare(
        &self,
        schedule: &Schedule,
        output_shapes: &[Vec<usize>],
    ) -> Result<Self::Executable, CompileError>;
    fn execute(
        &self,
        executable: &Self::Executable,
        schedule: &Schedule,
        inputs: &[InputSpec],
        bindings: &[&Self::Buffer],
        output_shapes: &[Vec<usize>],
    ) -> Result<Vec<Self::Buffer>, RunError>;
}

mod private {
    pub trait Sealed {}
}

#[doc(hidden)]
#[derive(Clone)]
pub struct InputSpec {
    name: String,
    lowered_name: &'static str,
    shape: Vec<Axis>,
    axes: Vec<kir::Axis>,
    dtype: Dtype,
}

impl InputSpec {
    fn concrete_shape(&self) -> Vec<usize> {
        self.shape.iter().copied().map(Axis::extent).collect()
    }
}

/// A compiled multi-output program. Output buffers are always returned in the
/// same order as the roots passed to [`Compile::compile`].
pub struct Program<B: Backend> {
    backend: B,
    schedule: Schedule,
    executable: B::Executable,
    inputs: Vec<InputSpec>,
    output_shapes: Vec<Vec<usize>>,
}

impl<B: Backend> Program<B> {
    pub fn input_names(&self) -> impl ExactSizeIterator<Item = &str> {
        self.inputs.iter().map(|input| input.name.as_str())
    }

    pub fn output_shapes(&self) -> &[Vec<usize>] {
        &self.output_shapes
    }

    pub fn kernel_count(&self) -> usize {
        self.schedule.kernel_count()
    }

    /// Execute with named buffers and return outputs in compilation-root order.
    ///
    /// Binding errors are programming errors in the direct API. Use
    /// [`Program::try_run`] when they need to be handled explicitly.
    pub fn run<'a, T>(&self, bindings: impl IntoIterator<Item = (&'a str, T)>) -> Vec<B::Buffer>
    where
        T: Borrow<B::Buffer>,
    {
        let bindings = bindings.into_iter().collect::<Vec<_>>();
        self.try_run(
            bindings
                .iter()
                .map(|(name, buffer)| (*name, <T as Borrow<B::Buffer>>::borrow(buffer))),
        )
        .unwrap_or_else(|error| panic!("program execution failed: {error}"))
    }

    pub fn try_run<'a>(
        &self,
        bindings: impl IntoIterator<Item = (&'a str, &'a B::Buffer)>,
    ) -> Result<Vec<B::Buffer>, RunError> {
        let ordered = self.ordered_bindings(bindings)?;
        self.backend.execute(
            &self.executable,
            &self.schedule,
            &self.inputs,
            &ordered,
            &self.output_shapes,
        )
    }

    /// Validate a named binding set and order it by the program's inputs —
    /// the checks behind [`Program::try_run`] and the Metal capture path.
    fn ordered_bindings<'a>(
        &self,
        bindings: impl IntoIterator<Item = (&'a str, &'a B::Buffer)>,
    ) -> Result<Vec<&'a B::Buffer>, RunError> {
        let mut by_name = HashMap::<&str, &B::Buffer>::new();
        for (name, buffer) in bindings {
            if !self.inputs.iter().any(|input| input.name == name) {
                return Err(RunError::UnknownInput(name.to_string()));
            }
            if by_name.insert(name, buffer).is_some() {
                return Err(RunError::DuplicateInput(name.to_string()));
            }
        }

        let mut ordered = Vec::with_capacity(self.inputs.len());
        for input in &self.inputs {
            let buffer = *by_name
                .get(input.name.as_str())
                .ok_or_else(|| RunError::MissingInput(input.name.clone()))?;
            let expected = input.concrete_shape();
            if buffer.shape() != expected {
                return Err(RunError::Shape {
                    name: input.name.clone(),
                    expected,
                    actual: buffer.shape().to_vec(),
                });
            }
            if buffer.dtype() != input.dtype {
                return Err(RunError::Dtype {
                    name: input.name.clone(),
                    expected: input.dtype,
                    actual: buffer.dtype(),
                });
            }
            ordered.push(buffer);
        }
        Ok(ordered)
    }
}

/// Something that can provide an ordered set of output roots.
pub trait Roots {
    fn roots(&self) -> Vec<NodeRef>;
}

impl Roots for NodeRef {
    fn roots(&self) -> Vec<NodeRef> {
        vec![self.clone()]
    }
}

impl Roots for &NodeRef {
    fn roots(&self) -> Vec<NodeRef> {
        vec![(*self).clone()]
    }
}

impl Roots for Vec<NodeRef> {
    fn roots(&self) -> Vec<NodeRef> {
        self.clone()
    }
}

impl Roots for &[NodeRef] {
    fn roots(&self) -> Vec<NodeRef> {
        self.to_vec()
    }
}

impl<const N: usize> Roots for [NodeRef; N] {
    fn roots(&self) -> Vec<NodeRef> {
        self.to_vec()
    }
}

macro_rules! tuple_roots {
    ($(($($name:ident),+)),+ $(,)?) => {
        $(
            impl<$($name: RootItem),+> Roots for ($($name,)+) {
                #[allow(non_snake_case)]
                fn roots(&self) -> Vec<NodeRef> {
                    let ($($name,)+) = self;
                    vec![$($name.root(),)+]
                }
            }
        )+
    };
}

pub trait RootItem {
    fn root(&self) -> NodeRef;
}

impl RootItem for NodeRef {
    fn root(&self) -> NodeRef {
        self.clone()
    }
}

impl RootItem for &NodeRef {
    fn root(&self) -> NodeRef {
        (*self).clone()
    }
}

tuple_roots!(
    (A, B),
    (A, B, C),
    (A, B, C, D),
    (A, B, C, D, E),
    (A, B, C, D, E, F),
    (A, B, C, D, E, F, G),
    (A, B, C, D, E, F, G, H),
);

/// Extension trait for compiling one or more roots.
pub trait Compile: Roots {
    fn compile<B: Backend>(&self, backend: &B) -> Result<Program<B>, CompileError> {
        compile_roots(self.roots(), backend)
    }
}

impl<T: Roots> Compile for T {}

fn compile_roots<B: Backend>(roots: Vec<NodeRef>, backend: &B) -> Result<Program<B>, CompileError> {
    if roots.is_empty() {
        return Err(CompileError::EmptyOutputs);
    }
    if contains_dynamic(&roots) {
        return Err(CompileError::DynamicShapesNotYetSupported);
    }

    // Canonicalize BEFORE lowering: lowering mints fresh scoped axes per
    // node, so two structurally identical public subtrees (a RoPE frequency
    // table built once for the query path and once for the key path) become
    // axis-distinct — and unmergeable — the moment they lower. Merged here,
    // they lower ONCE through the pointer-keyed memo.
    let roots = crate::ir::canonicalize_many(&roots);

    let mut lowerer = Lowerer::default();
    let lowered = roots
        .iter()
        .map(|root| lowerer.lower(root))
        .collect::<Result<Vec<_>, _>>()?;
    let lowered_roots = lowered
        .into_iter()
        .map(|lowered| ensure_order(lowered.node, &lowered.axes))
        .collect::<Vec<_>>();
    let output_shapes = roots
        .iter()
        .map(|root| root.shape().into_iter().map(Axis::extent).collect())
        .collect::<Vec<Vec<usize>>>();

    crate::verify::verify_many(&lowered_roots)
        .map_err(|error| CompileError::InvalidGraph(error.to_string()))?;

    let output_names = (0..roots.len())
        .map(|index| leak(format!("Out{index}")))
        .collect::<Vec<_>>();
    let named_roots = lowered_roots
        .iter()
        .cloned()
        .zip(output_names)
        .collect::<Vec<_>>();
    let schedule = partition_many(&named_roots, &backend.profile());
    let executable = backend.prepare(&schedule, &output_shapes)?;

    Ok(Program {
        backend: backend.clone(),
        schedule,
        executable,
        inputs: lowerer.inputs,
        output_shapes,
    })
}

fn leak(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

fn contains_dynamic(roots: &[NodeRef]) -> bool {
    fn visit(node: &NodeRef, seen: &mut HashSet<*const Node>) -> bool {
        if !seen.insert(Rc::as_ptr(node)) {
            return false;
        }
        if node
            .shape()
            .iter()
            .any(|axis| axis.extent == Extent::Dynamic)
        {
            return true;
        }
        match node.as_ref() {
            Node::Input { .. } | Node::Const { .. } | Node::Iota { .. } => false,
            Node::Coordinate { src, .. } => visit(src, seen),
            Node::Map { inputs, .. } => inputs.iter().any(|input| visit(input, seen)),
            Node::Reduce { src, .. }
            | Node::Scan { src, .. }
            | Node::View { src, .. }
            | Node::Reindex { src, .. } => visit(src, seen),
            Node::Gather { src, index, .. } => visit(src, seen) || visit(index, seen),
        }
    }
    let mut seen = HashSet::new();
    roots.iter().any(|root| visit(root, &mut seen))
}

#[derive(Default)]
struct Lowerer {
    next_axis: usize,
    memo: HashMap<*const Node, Lowered>,
    inputs: Vec<InputSpec>,
}

#[derive(Clone)]
struct Lowered {
    node: kir::NodeRef,
    /// Kernel axes corresponding to the public node's positional dimensions.
    /// `None` is a source-free singleton inserted only for broadcasting.
    axes: Vec<Option<kir::Axis>>,
}

impl Lowerer {
    fn fresh_axis(&mut self, axis: Axis) -> kir::Axis {
        let id = self.next_axis;
        self.next_axis += 1;
        kir::scoped_axis(id, axis.name, axis.extent)
    }

    fn lower(&mut self, node: &NodeRef) -> Result<Lowered, CompileError> {
        let pointer = Rc::as_ptr(node);
        if let Some(lowered) = self.memo.get(&pointer) {
            return Ok(lowered.clone());
        }

        let lowered = match node.as_ref() {
            Node::Input { name, shape, dtype } => {
                if name.is_empty() {
                    return Err(CompileError::InvalidInput(
                        "input names cannot be empty".into(),
                    ));
                }
                if let Some(previous) = self.inputs.iter().find(|input| input.name == *name) {
                    let previous_extents: Vec<Extent> =
                        previous.shape.iter().map(|axis| axis.extent).collect();
                    let extents: Vec<Extent> = shape.iter().map(|axis| axis.extent).collect();
                    if previous_extents != extents || previous.dtype != *dtype {
                        return Err(CompileError::InvalidInput(format!(
                            "`{name}` was declared incompatibly"
                        )));
                    }
                }
                let axes = shape
                    .iter()
                    .copied()
                    .map(|axis| self.fresh_axis(axis))
                    .collect::<Vec<_>>();
                let lowered_name = self
                    .inputs
                    .iter()
                    .find(|input| input.name == *name)
                    .map(|input| input.lowered_name)
                    .unwrap_or_else(|| leak(name.clone()));
                if !self.inputs.iter().any(|input| input.name == *name) {
                    self.inputs.push(InputSpec {
                        name: name.clone(),
                        lowered_name,
                        shape: shape.clone(),
                        axes: axes.clone(),
                        dtype: *dtype,
                    });
                }
                Lowered {
                    node: kir::input(lowered_name, &axes, *dtype),
                    axes: axes.into_iter().map(Some).collect(),
                }
            }
            Node::Const { v } => Lowered {
                node: kir::konst(*v),
                axes: Vec::new(),
            },
            Node::Iota { axis } => {
                let axis = self.fresh_axis(*axis);
                Lowered {
                    node: kir::iota(axis),
                    axes: vec![Some(axis)],
                }
            }
            Node::Coordinate { src, dim } => {
                let lowered = self.lower(src)?;
                let node = lowered.axes[*dim]
                    .map(kir::iota)
                    .unwrap_or_else(|| kir::konst(0.0));
                Lowered {
                    node,
                    axes: lowered.axes,
                }
            }
            Node::Map { op, inputs } => {
                let output_shape = node.shape();
                let lowered_inputs = inputs
                    .iter()
                    .map(|input| self.lower(input))
                    .collect::<Result<Vec<_>, _>>()?;
                let input_shapes = inputs.iter().map(|input| input.shape()).collect::<Vec<_>>();
                let rank = output_shape.len();
                let output_axes = (0..rank)
                    .map(|output_dim| {
                        let mut fallback = None;
                        for (shape, lowered) in input_shapes.iter().zip(&lowered_inputs) {
                            let Some(source_dim) =
                                output_dim.checked_sub(rank.saturating_sub(shape.len()))
                            else {
                                continue;
                            };
                            let candidate = lowered.axes[source_dim];
                            fallback = fallback.or(candidate);
                            if shape[source_dim].extent != Extent::Static(1) {
                                return candidate;
                            }
                        }
                        fallback
                    })
                    .collect::<Vec<_>>();
                let aligned = lowered_inputs
                    .into_iter()
                    .zip(&input_shapes)
                    .map(|(lowered, shape)| align(lowered, shape, &output_shape, &output_axes))
                    .collect();
                let mapped = kir::map(*op, aligned);
                Lowered {
                    node: ensure_order(mapped, &output_axes),
                    axes: output_axes,
                }
            }
            Node::Reduce { src, dim, op } => {
                let mut lowered = self.lower(src)?;
                if let Some(axis) = lowered.axes.remove(*dim) {
                    lowered.node = kir::reduce(lowered.node, axis, *op);
                }
                lowered
            }
            Node::Scan { src, dim, op } => {
                let mut lowered = self.lower(src)?;
                if let Some(axis) = lowered.axes[*dim] {
                    lowered.node = kir::scan(lowered.node, axis, *op);
                }
                lowered
            }
            Node::Gather { src, index, dim } => {
                let mut lowered_src = self.lower(src)?;
                let lowered_index = self.lower(index)?;
                let axis = lowered_src.axes.remove(*dim).ok_or_else(|| {
                    CompileError::InvalidGraph(
                        "cannot gather a source-free singleton dimension".into(),
                    )
                })?;
                let mut axes = lowered_src.axes;
                axes.splice(*dim..*dim, lowered_index.axes);
                let gathered = kir::gather(lowered_src.node, lowered_index.node, axis);
                Lowered {
                    node: ensure_order(gathered, &axes),
                    axes,
                }
            }
            Node::View { src, dims } => {
                let lowered = self.lower(src)?;
                let mut groups = Vec::new();
                let mut output_axes = Vec::with_capacity(dims.len());
                for dim in dims {
                    let members = dim
                        .sources
                        .iter()
                        .filter_map(|&source| lowered.axes[source])
                        .collect::<Vec<_>>();
                    let output = match members.as_slice() {
                        [] => None,
                        [axis] => Some(*axis),
                        _ => {
                            let output = self.fresh_axis(dim.axis);
                            groups.push((members, output));
                            Some(output)
                        }
                    };
                    output_axes.push(output);
                }
                let viewed = if groups.is_empty() {
                    lowered.node
                } else {
                    kir::view(lowered.node, groups)
                };
                Lowered {
                    node: viewed,
                    axes: output_axes,
                }
            }
            Node::Reindex {
                src,
                shape,
                map,
                padded,
            } => {
                let lowered = self.lower(src)?;
                let output_axes = shape
                    .iter()
                    .copied()
                    .map(|axis| self.fresh_axis(axis))
                    .collect::<Vec<_>>();
                let map = map
                    .iter()
                    .filter_map(|(source, terms, offset)| {
                        lowered.axes[*source].map(|source_axis| {
                            (
                                source_axis,
                                terms
                                    .iter()
                                    .map(|(coefficient, output)| {
                                        (*coefficient, output_axes[*output])
                                    })
                                    .collect(),
                                *offset,
                            )
                        })
                    })
                    .collect();
                let reindexed = kir::reindex(lowered.node, map, *padded);
                let axes = output_axes.into_iter().map(Some).collect::<Vec<_>>();
                Lowered {
                    node: ensure_order(reindexed, &axes),
                    axes,
                }
            }
        };

        self.memo.insert(pointer, lowered.clone());
        Ok(lowered)
    }
}

fn align(
    lowered: Lowered,
    source: &[Axis],
    target: &[Axis],
    target_axes: &[Option<kir::Axis>],
) -> kir::NodeRef {
    let mut node = lowered.node;
    let offset = target.len() - source.len();
    for (source_dim, source_axis) in source.iter().enumerate() {
        let Some(lowered_axis) = lowered.axes[source_dim] else {
            continue;
        };
        let target_dim = offset + source_dim;
        let target_axis = target[target_dim];
        match (
            source_axis.extent,
            target_axis.extent,
            target_axes[target_dim],
        ) {
            (Extent::Static(1), right, _) if right != Extent::Static(1) => {
                node = kir::reindex(node, vec![(lowered_axis, Vec::new(), 0)], false);
            }
            (left, right, Some(target_axis)) if left == right => {
                if lowered_axis != target_axis {
                    node = kir::rename(node, lowered_axis, target_axis);
                }
            }
            _ => unreachable!("public map shape validation accepted incompatible dimensions"),
        }
    }
    node
}

fn ensure_order(node: kir::NodeRef, axes: &[Option<kir::Axis>]) -> kir::NodeRef {
    let axes = axes.iter().flatten().copied().collect::<Vec<_>>();
    if node.shape() == axes {
        return node;
    }
    let mut zero = kir::konst(0.0);
    for axis in axes {
        zero = kir::map(
            kir::MapOp::Add,
            vec![
                zero,
                kir::map(kir::MapOp::Mul, vec![kir::iota(axis), kir::konst(0.0)]),
            ],
        );
    }
    kir::map(kir::MapOp::Add, vec![zero, node])
}

// ── CPU backend ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default)]
pub struct CpuDevice;

impl CpuDevice {
    pub const fn new() -> Self {
        CpuDevice
    }

    pub fn buffer(
        &self,
        shape: impl Into<Vec<usize>>,
        dtype: Dtype,
        data: impl Into<Vec<f64>>,
    ) -> Result<CpuBuffer, RunError> {
        let shape = shape.into();
        let data = data.into();
        let expected = shape.iter().product::<usize>().max(1);
        if data.len() != expected {
            return Err(RunError::Backend(format!(
                "shape {shape:?} requires {expected} values, received {}",
                data.len()
            )));
        }
        Ok(CpuBuffer { shape, dtype, data })
    }
}

impl private::Sealed for CpuDevice {}

#[derive(Clone, Debug)]
pub struct CpuBuffer {
    shape: Vec<usize>,
    dtype: Dtype,
    data: Vec<f64>,
}

impl CpuBuffer {
    pub fn data(&self) -> &[f64] {
        &self.data
    }

    pub fn into_data(self) -> Vec<f64> {
        self.data
    }
}

impl Buffer for CpuBuffer {
    fn shape(&self) -> &[usize] {
        &self.shape
    }

    fn dtype(&self) -> Dtype {
        self.dtype
    }
}

impl Backend for CpuDevice {
    type Buffer = CpuBuffer;
    type Executable = ();

    fn profile(&self) -> cost::DeviceProfile {
        cost::DeviceProfile::toy()
    }

    fn prepare(
        &self,
        _schedule: &Schedule,
        _output_shapes: &[Vec<usize>],
    ) -> Result<Self::Executable, CompileError> {
        Ok(())
    }

    fn execute(
        &self,
        _executable: &Self::Executable,
        schedule: &Schedule,
        inputs: &[InputSpec],
        bindings: &[&Self::Buffer],
        output_shapes: &[Vec<usize>],
    ) -> Result<Vec<Self::Buffer>, RunError> {
        let mut env = Env::new();
        for (input, buffer) in inputs.iter().zip(bindings) {
            env.insert(
                input.lowered_name,
                Value {
                    axes: input.axes.clone(),
                    shape: buffer.shape.clone(),
                    data: buffer.data.clone(),
                },
            );
        }
        schedule.execute_env(&mut env);
        schedule
            .outputs
            .iter()
            .zip(output_shapes)
            .map(|(name, shape)| {
                let value = env.remove(name.as_str()).ok_or_else(|| {
                    RunError::Backend(format!("schedule did not produce `{name}`"))
                })?;
                Ok(CpuBuffer {
                    shape: shape.clone(),
                    dtype: Dtype::F64,
                    data: value.data,
                })
            })
            .collect()
    }
}

// ── Metal backend ───────────────────────────────────────────────────────────

#[cfg(target_os = "macos")]
mod metal_backend {
    use super::*;
    use crate::emit_metal::{MetalProgram, emit_schedule_metal_on};
    use crate::metal::{
        Dispatch, MetalBuf, MetalDevice, MetalGraph, Pipelines, program_dispatches,
    };

    pub struct MetalExecutable {
        program: MetalProgram,
        pipelines: Pipelines,
    }

    #[derive(Clone)]
    pub struct MetalBuffer {
        raw: MetalBuf,
        shape: Vec<usize>,
        dtype: Dtype,
    }

    impl Buffer for MetalBuffer {
        fn shape(&self) -> &[usize] {
            &self.shape
        }

        fn dtype(&self) -> Dtype {
            self.dtype
        }
    }

    impl MetalBuffer {
        pub fn raw(&self) -> &MetalBuf {
            &self.raw
        }
    }

    impl private::Sealed for MetalDevice {}

    impl Backend for MetalDevice {
        type Buffer = MetalBuffer;
        type Executable = MetalExecutable;

        fn profile(&self) -> cost::DeviceProfile {
            cost::DeviceProfile::m1_pro()
        }

        fn prepare(
            &self,
            schedule: &Schedule,
            _output_shapes: &[Vec<usize>],
        ) -> Result<Self::Executable, CompileError> {
            if schedule
                .stages
                .iter()
                .any(|stage| matches!(stage, crate::partition::Stage::Infeasible { .. }))
            {
                return Err(CompileError::Backend(format!(
                    "Metal schedule contains an infeasible stage:\n{}",
                    schedule.render()
                )));
            }
            let program = emit_schedule_metal_on(&self.profile(), schedule);
            let pipelines = MetalDevice::compile(self, &program.msl);
            Ok(MetalExecutable { program, pipelines })
        }

        fn execute(
            &self,
            executable: &Self::Executable,
            schedule: &Schedule,
            inputs: &[InputSpec],
            bindings: &[&Self::Buffer],
            output_shapes: &[Vec<usize>],
        ) -> Result<Vec<Self::Buffer>, RunError> {
            let mut buffers = HashMap::<String, MetalBuf>::new();
            for (input, buffer) in inputs.iter().zip(bindings) {
                buffers.insert(input.lowered_name.to_string(), buffer.raw.clone());
            }
            for (name, size) in &executable.program.buffers {
                buffers.insert(name.clone(), self.alloc_f32(*size));
            }
            let dispatches =
                program_dispatches(&executable.program, &buffers, &executable.pipelines);
            if crate::debug_level() >= 2 {
                run_debug(self, &executable.program, schedule, &dispatches);
            } else {
                self.run(&dispatches);
            }
            schedule
                .outputs
                .iter()
                .zip(output_shapes)
                .map(|(name, shape)| {
                    let raw = buffers.get(name).cloned().ok_or_else(|| {
                        RunError::Backend(format!("Metal schedule did not produce `{name}`"))
                    })?;
                    Ok(MetalBuffer {
                        raw,
                        shape: shape.clone(),
                        dtype: Dtype::F32,
                    })
                })
                .collect()
        }
    }

    /// A program frozen over one binding set and captured as replayable
    /// Metal graphs — the production decode path. [`Program::capture`]
    /// encodes every dispatch ONCE into an indirect command buffer
    /// ([`crate::metal::MetalGraph`]); [`MetalReplay::step`] then replays
    /// the whole schedule as one encoder and one `executeCommandsInBuffer`
    /// instead of re-allocating buffers and re-encoding N kernels per step.
    ///
    /// `feedback` wires an output to an input of the NEXT step (a KV cache
    /// flowing through a decode loop). Bindings are frozen at capture, so a
    /// fed-back pair ping-pongs between two buffers: two graphs are captured
    /// with the pair's roles swapped and steps alternate between them.
    /// Everything else — weights, intermediates, CPU-written inputs like a
    /// token id — is the same buffer in both parities.
    pub struct MetalReplay<'p> {
        program: &'p Program<MetalDevice>,
        /// One captured graph, or two when feedback swaps bindings.
        graphs: Vec<MetalGraph>,
        /// Per-parity dispatch lists — the `SANIC_DEBUG=2` fallback.
        dispatches: Vec<Vec<Dispatch>>,
        /// Per-parity outputs in compilation-root order.
        outputs: Vec<Vec<MetalBuffer>>,
        parity: usize,
    }

    impl Program<MetalDevice> {
        /// Freeze this program over `bindings` and capture it for replay —
        /// see [`MetalReplay`]. Binding validation matches
        /// [`Program::try_run`]; every `feedback` pair `(output index,
        /// input name)` must name a real output and a real input of equal
        /// shape (outputs are f32, so the input must be too).
        pub fn capture<'a>(
            &self,
            bindings: impl IntoIterator<Item = (&'a str, &'a MetalBuffer)>,
            feedback: &[(usize, &str)],
        ) -> Result<MetalReplay<'_>, RunError> {
            use std::collections::HashSet;

            let ordered = self.ordered_bindings(bindings)?;
            // (output buffer name, the fed input's lowered name)
            let mut swaps = Vec::new();
            let mut fed_outputs = HashSet::new();
            let mut fed_inputs = HashSet::new();
            for &(output, input_name) in feedback {
                let output_name = self.schedule.outputs.get(output).ok_or_else(|| {
                    RunError::Feedback(format!(
                        "output index {output} is out of range ({} outputs)",
                        self.schedule.outputs.len()
                    ))
                })?;
                let input = self
                    .inputs
                    .iter()
                    .find(|input| input.name == input_name)
                    .ok_or_else(|| {
                        RunError::Feedback(format!("program has no input named `{input_name}`"))
                    })?;
                if self.output_shapes[output] != input.concrete_shape() {
                    return Err(RunError::Feedback(format!(
                        "output {output} has shape {:?}; input `{input_name}` expects {:?}",
                        self.output_shapes[output],
                        input.concrete_shape()
                    )));
                }
                if input.dtype != Dtype::F32 {
                    return Err(RunError::Feedback(format!(
                        "input `{input_name}` is {:?}; outputs are F32",
                        input.dtype
                    )));
                }
                if !fed_outputs.insert(output) {
                    return Err(RunError::Feedback(format!(
                        "output {output} is fed back more than once"
                    )));
                }
                if !fed_inputs.insert(input_name) {
                    return Err(RunError::Feedback(format!(
                        "input `{input_name}` is fed more than once"
                    )));
                }
                swaps.push((output_name.clone(), input.lowered_name));
            }

            let device = &self.backend;
            let executable = &self.executable;
            let mut base = HashMap::<String, MetalBuf>::new();
            for (input, buffer) in self.inputs.iter().zip(&ordered) {
                base.insert(input.lowered_name.to_string(), buffer.raw.clone());
            }
            for (name, size) in &executable.program.buffers {
                base.insert(name.clone(), device.alloc_f32(*size));
            }

            let parities = if feedback.is_empty() { 1 } else { 2 };
            let mut graphs = Vec::with_capacity(parities);
            let mut dispatches = Vec::with_capacity(parities);
            let mut outputs = Vec::with_capacity(parities);
            for parity in 0..parities {
                let mut buffers = base.clone();
                if parity == 1 {
                    for (output_name, lowered_name) in &swaps {
                        let fed_input = base[*lowered_name].clone();
                        let first_output = base[output_name.as_str()].clone();
                        buffers.insert(output_name.clone(), fed_input);
                        buffers.insert(lowered_name.to_string(), first_output);
                    }
                    // a bindless stage's address table is filled at dispatch
                    // build; this parity binds different buffers, so it needs
                    // its own table
                    for stage in &executable.program.stages {
                        if let Some(argbuf) = &stage.argbuf {
                            let size = executable
                                .program
                                .buffers
                                .iter()
                                .find(|(name, _)| name == argbuf)
                                .map(|(_, size)| *size)
                                .unwrap_or(stage.inputs.len() * 2);
                            buffers.insert(argbuf.clone(), device.alloc_f32(size));
                        }
                    }
                }
                let dispatch_list =
                    program_dispatches(&executable.program, &buffers, &executable.pipelines);
                graphs.push(device.capture(&dispatch_list));
                dispatches.push(dispatch_list);
                outputs.push(
                    self.schedule
                        .outputs
                        .iter()
                        .zip(&self.output_shapes)
                        .map(|(name, shape)| {
                            let raw = buffers.get(name).cloned().ok_or_else(|| {
                                RunError::Backend(format!(
                                    "Metal schedule did not produce `{name}`"
                                ))
                            })?;
                            Ok(MetalBuffer {
                                raw,
                                shape: shape.clone(),
                                dtype: Dtype::F32,
                            })
                        })
                        .collect::<Result<Vec<_>, RunError>>()?,
                );
            }

            Ok(MetalReplay {
                program: self,
                graphs,
                dispatches,
                outputs,
                parity: 0,
            })
        }
    }

    impl MetalReplay<'_> {
        /// Replay one step and return its outputs in compilation-root order.
        /// Fed-back outputs are already wired as the NEXT step's inputs;
        /// write CPU-driven inputs (a token id) into their bound buffers
        /// before calling. Under `SANIC_DEBUG=2` the step runs through the
        /// per-dispatch timed path and prints the launch dump instead of
        /// replaying the captured graph.
        ///
        /// Errs on any command buffer error — the step's writes are
        /// untrustworthy and a decode loop must not continue on them.
        pub fn step(&mut self) -> Result<&[MetalBuffer], RunError> {
            self.step_timed().map(|(outputs, _)| outputs)
        }

        /// [`Self::step`], returning the replayed command buffer's GPU
        /// residency in seconds (see [`crate::metal::MetalDevice::run_timed`]).
        /// Under `SANIC_DEBUG=2` the returned time is the per-dispatch sum —
        /// a debug number with a sync floor, as the dump's footer says.
        pub fn step_timed(&mut self) -> Result<(&[MetalBuffer], f64), RunError> {
            let parity = self.advance();
            let seconds = if crate::debug_level() >= 2 {
                run_debug(
                    &self.program.backend,
                    &self.program.executable.program,
                    &self.program.schedule,
                    &self.dispatches[parity],
                )
            } else {
                self.program
                    .backend
                    .run_graph_timed(&self.graphs[parity])
                    .map_err(RunError::Backend)?
            };
            Ok((&self.outputs[parity], seconds))
        }

        fn advance(&mut self) -> usize {
            let parity = self.parity;
            self.parity = (parity + 1) % self.graphs.len();
            parity
        }
    }

    /// The `SANIC_DEBUG=2` runtime dump. One line per launch, printed after
    /// the step so shares are exact:
    ///
    /// ```text
    /// *** metal  407 Out32        fold   grid   128256   7934us █████████▏ 16.6%  plan ×1.28  ~ 525.9MB bw  33%
    /// ```
    ///
    /// The index and OUTPUT name match the `SANIC_DEBUG=1` schedule dump, so
    /// the two dumps cross-reference. The bar is scaled to the slowest
    /// launch; `plan ×r` is measured over the cost the planner CHOSE this
    /// schedule by (fused stages), so the dump audits the cost model — the
    /// footer's `plan Σ` is its aggregate calibration on this machine. `bw`
    /// is the fraction of the device's memory bandwidth actually achieved;
    /// bytes are logical buffer sizes (an upper bound — a gather reading one
    /// row of a huge table overcounts, so implausible ratios print `--`).
    ///
    /// Times come from one command buffer per dispatch — accurate per
    /// kernel, but the submits add overhead: the SUM is a debug number, and
    /// `MetalDevice::run_timed` measures the production step.
    fn run_debug(
        device: &MetalDevice,
        program: &MetalProgram,
        schedule: &Schedule,
        dispatches: &[Dispatch],
    ) -> f64 {
        use crate::partition::Stage;
        // Logical bytes per buffer name. An allocation's `byte_len` would
        // overcount: a zero-copy checkpoint tensor is a SLICE of the whole
        // weights file.
        let mut logical_bytes = HashMap::<&str, f64>::new();
        for (name, elements) in &program.buffers {
            logical_bytes.insert(name, *elements as f64 * 4.0);
        }
        for (name, axes) in &program.inputs {
            let elements: usize = axes.iter().map(|a| a.extent()).product();
            let width = program.dtypes.get(*name).map_or(4.0, |dtype| dtype.bytes());
            logical_bytes.insert(name, elements as f64 * width);
        }
        // Kind and planned cost, by the output name each dispatch writes.
        let mut stage_info = HashMap::<&str, (&'static str, Option<f64>)>::new();
        for stage in &schedule.stages {
            let (kind, planned) = match stage {
                Stage::Fused { spec, .. } => ("fold", Some(spec.cost)),
                Stage::Elementwise { .. } => ("map", None),
                Stage::Gather { .. } => ("gather", None),
                Stage::Sequential { .. } => ("seq", None),
                Stage::Infeasible { .. } => ("infeasible", None),
            };
            stage_info.insert(crate::partition::stage_output(stage), (kind, planned));
        }

        let times = device.run_each_timed(dispatches);
        let total = times.iter().sum::<f64>().max(1e-12);
        let slowest = times.iter().copied().fold(0.0, f64::max).max(1e-12);
        let (mut fused_measured, mut fused_planned) = (0.0f64, 0.0f64);
        for (index, ((stage, dispatch), &seconds)) in program
            .stages
            .iter()
            .zip(dispatches)
            .zip(&times)
            .enumerate()
        {
            let bytes = stage
                .inputs
                .iter()
                .chain(std::iter::once(&stage.output))
                .map(|name| logical_bytes.get(name.as_str()).copied().unwrap_or(0.0))
                .sum::<f64>();
            let (kind, planned) = stage_info
                .get(stage.output.as_str())
                .copied()
                .unwrap_or(("?", None));
            let micros = seconds * 1e6;
            let time = if micros >= 1000.0 {
                format!("\x1b[33m{micros:7.0}us\x1b[0m") // slow launch: yellow
            } else {
                format!("{micros:7.0}us")
            };
            let plan = match planned {
                Some(p) if p > 0.0 => {
                    fused_measured += seconds;
                    fused_planned += p;
                    format!("plan ×{:<5.2}", seconds / p)
                }
                _ => " ".repeat(11),
            };
            let peak_fraction = bytes / seconds.max(1e-9) / device.profile().hbm_bandwidth;
            let bandwidth = if peak_fraction > 5.0 {
                "bw  --".to_string()
            } else {
                format!("bw {:3.0}%", peak_fraction * 100.0)
            };
            eprintln!(
                "*** metal {index:4} {:<12} {kind:<6} grid {:>8} {time} {} {:4.1}%  {plan} ~{:>6.1}MB {bandwidth}",
                stage.output,
                dispatch.grid,
                crate::debug_bar(seconds / slowest, 10),
                100.0 * seconds / total,
                bytes / 1e6,
            );
        }

        let mut ranked: Vec<(&str, f64)> = program
            .stages
            .iter()
            .map(|s| s.output.as_str())
            .zip(times.iter().copied())
            .collect();
        ranked.sort_by(|a, b| b.1.total_cmp(&a.1));
        let top = ranked
            .iter()
            .take(4)
            .map(|(name, t)| format!("{name} {:.1}%", 100.0 * t / total))
            .collect::<Vec<_>>()
            .join("  ");
        let rest = 100.0 * (total - ranked.iter().take(4).map(|(_, t)| t).sum::<f64>()) / total;
        let calibration = if fused_planned > 0.0 {
            format!("plan Σ ×{:.2}", fused_measured / fused_planned)
        } else {
            String::new()
        };
        eprintln!(
            "*** metal step: {} launches {:.2}ms GPU (per-launch sync; production time = run_timed)",
            times.len(),
            total * 1e3,
        );
        eprintln!("*** metal top:  {top}  rest {rest:.0}%  |  {calibration}");
        times.iter().sum()
    }

    impl MetalDevice {
        /// Wrap an existing Metal allocation in a shaped public buffer.
        ///
        /// This is the zero-copy checkpoint path: callers can wrap one
        /// page-aligned file allocation with [`MetalDevice::from_bytes_nocopy`],
        /// slice it at a safetensors offset, then retain the checkpoint's BF16
        /// storage without a host decode or GPU upload.
        pub fn tensor_from_raw(
            &self,
            raw: MetalBuf,
            shape: impl Into<Vec<usize>>,
            dtype: Dtype,
        ) -> Result<MetalBuffer, RunError> {
            let shape = shape.into();
            let elements = shape.iter().product::<usize>().max(1);
            let required_bytes = match dtype {
                Dtype::I4 => elements.div_ceil(2),
                _ => elements * dtype.bytes() as usize,
            };
            if raw.byte_len() < required_bytes {
                return Err(RunError::Backend(format!(
                    "shape {shape:?} with dtype {dtype:?} requires {required_bytes} bytes, \
                     but the Metal allocation has {}",
                    raw.byte_len()
                )));
            }
            Ok(MetalBuffer { raw, shape, dtype })
        }

        pub fn tensor_from_f64(
            &self,
            shape: impl Into<Vec<usize>>,
            dtype: Dtype,
            data: &[f64],
        ) -> Result<MetalBuffer, RunError> {
            let shape = shape.into();
            let expected = shape.iter().product::<usize>().max(1);
            if data.len() != expected {
                return Err(RunError::Backend(format!(
                    "shape {shape:?} requires {expected} values, received {}",
                    data.len()
                )));
            }
            // Existing upload helpers widen/narrow according to generated
            // kernel load type. F32 is the general public upload path today.
            if dtype != Dtype::F32 {
                return Err(RunError::Backend(format!(
                    "Metal tensor upload for {dtype:?} is not exposed yet"
                )));
            }
            Ok(MetalBuffer {
                raw: self.from_f64(data),
                shape,
                dtype,
            })
        }

        pub fn read_tensor_f32(&self, buffer: &MetalBuffer) -> Vec<f32> {
            self.read_f32(&buffer.raw, buffer.shape.iter().product::<usize>().max(1))
        }
    }

    pub use MetalBuffer as PublicMetalBuffer;
}

#[cfg(target_os = "macos")]
pub use metal_backend::{MetalReplay, PublicMetalBuffer as MetalBuffer};

#[cfg(test)]
mod positional_lowering_tests {
    use super::*;
    use crate::ir::{axis, input};
    use crate::nn::functional::scaled_dot_product_attention;

    #[test]
    fn direct_attention_lowers_to_one_kernel() {
        let sequence = axis("sequence", 2);
        let features = axis("features", 2);
        let q = input("q", [sequence, features], Dtype::F32);
        let k = input("k", [sequence, features], Dtype::F32);
        let v = input("v", [sequence, features], Dtype::F32);
        let output = scaled_dot_product_attention(q, k, v, None, 0.0, false, None, false);
        let program = output.compile(&CpuDevice::new()).unwrap();
        assert_eq!(program.kernel_count(), 1, "{}", program.schedule.render());
    }

    #[test]
    fn causal_attention_lowers_to_one_kernel() {
        let query_sequence = axis("query_sequence", 2);
        let key_sequence = axis("key_sequence", 3);
        let features = axis("features", 2);
        let output = scaled_dot_product_attention(
            input("q", [query_sequence, features], Dtype::F32),
            input("k", [key_sequence, features], Dtype::F32),
            input("v", [key_sequence, features], Dtype::F32),
            None,
            0.0,
            true,
            Some(0.5),
            false,
        );
        let program = output.compile(&CpuDevice::new()).unwrap();
        assert_eq!(program.kernel_count(), 1, "{}", program.schedule.render());
    }
}
