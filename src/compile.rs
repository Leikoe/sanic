//! Root-based compilation and execution.
//!
//! The graph stays positional throughout compilation. Analysis resolves each
//! dimension occurrence lazily as `(node pointer, dimension index)` metadata;
//! there is no second graph representation or graph-rewriting lowering step.

use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::cost;
use crate::interp::{Env, Value};
use crate::ir::{self, Axis, AxisRef, Dtype, Extent, Node, NodeRef};
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
            CompileError::DynamicShapesNotYetSupported => f.write_str("compiling dynamic shapes is not supported yet"),
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
            RunError::Shape { name, expected, actual } => {
                write!(f, "input `{name}` has shape {actual:?}; expected {expected:?}")
            }
            RunError::Dtype { name, expected, actual } => {
                write!(f, "input `{name}` has dtype {actual:?}; expected {expected:?}")
            }
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
    fn prepare(&self, schedule: &Schedule, output_shapes: &[Vec<usize>]) -> Result<Self::Executable, CompileError>;
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
    axes: Vec<AxisRef>,
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
    /// Roots that write the buffer of an input instead of one of their own,
    /// as (root index, that input's name). See [`compile_roots_in_place`].
    /// Only the replay path acts on it, and only Metal has one.
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    in_place: Vec<(usize, &'static str)>,
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

impl RootItem for crate::tensor::Tensor {
    fn root(&self) -> NodeRef {
        self.node().clone()
    }
}

impl RootItem for &crate::tensor::Tensor {
    fn root(&self) -> NodeRef {
        self.node().clone()
    }
}

impl Roots for crate::tensor::Tensor {
    fn roots(&self) -> Vec<NodeRef> {
        vec![self.node().clone()]
    }
}

impl Roots for &crate::tensor::Tensor {
    fn roots(&self) -> Vec<NodeRef> {
        vec![self.node().clone()]
    }
}

impl Roots for Vec<crate::tensor::Tensor> {
    fn roots(&self) -> Vec<NodeRef> {
        self.iter().map(|tensor| tensor.node().clone()).collect()
    }
}

impl Roots for &[crate::tensor::Tensor] {
    fn roots(&self) -> Vec<NodeRef> {
        self.iter().map(|tensor| tensor.node().clone()).collect()
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
        compile_roots_in_place(self.roots(), Vec::new(), Vec::new(), backend)
    }
}

impl<T: Roots> Compile for T {}

/// Compile `roots`, where the roots named in `in_place` write the buffer of
/// an existing input rather than one of their own — `(root index, input
/// name)`.
///
/// The caller owns that claim: it must know the root's value agrees with the
/// input everywhere the root does not store, and that nothing in the step
/// still wants the input's OLD contents. In exchange the emitted kernel skips
/// the part it would have copied, and no second buffer is allocated or
/// swapped between steps.
pub(crate) fn compile_roots_in_place<B: Backend>(
    roots: Vec<NodeRef>,
    in_place: Vec<(usize, &'static str)>,
    output_dtypes: Vec<Option<Dtype>>,
    backend: &B,
) -> Result<Program<B>, CompileError> {
    if roots.is_empty() {
        return Err(CompileError::EmptyOutputs);
    }
    if contains_dynamic(&roots) {
        return Err(CompileError::DynamicShapesNotYetSupported);
    }

    // Constructors intern: structurally identical subtrees are already one
    // immutable DAG node, across roots and across the whole process.
    let inputs = collect_inputs(&roots)?;
    let output_shapes = roots
        .iter()
        .map(|root| root.shape().into_iter().map(Axis::extent).collect())
        .collect::<Vec<Vec<usize>>>();

    crate::verify::verify_many(&roots).map_err(|error| CompileError::InvalidGraph(error.to_string()))?;

    let output_names = (0..roots.len())
        .map(|index| leak(format!("Out{index}")))
        .collect::<Vec<_>>();
    let named_roots = roots.iter().cloned().zip(output_names).collect::<Vec<_>>();
    let mut schedule = partition_many(&named_roots, &backend.profile());
    if !output_dtypes.is_empty() {
        assert_eq!(
            output_dtypes.len(),
            schedule.outputs.len(),
            "one storage width per root"
        );
        schedule.output_dtypes = output_dtypes;
    }
    schedule.agrees_in_place = in_place
        .iter()
        .map(|&(root, _)| schedule.outputs[root].clone())
        .collect();
    let executable = backend.prepare(&schedule, &output_shapes)?;

    Ok(Program {
        backend: backend.clone(),
        schedule,
        executable,
        inputs,
        output_shapes,
        in_place,
    })
}

fn leak(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

fn contains_dynamic(roots: &[NodeRef]) -> bool {
    fn visit(node: &NodeRef, seen: &mut HashSet<*const Node>) -> bool {
        if !seen.insert(Arc::as_ptr(node)) {
            return false;
        }
        if node.shape().iter().any(|axis| axis.extent == Extent::Dynamic) {
            return true;
        }
        match node.as_ref() {
            Node::Input { .. } | Node::Const { .. } | Node::Iota { .. } => false,
            Node::Coordinate { src, .. } => visit(src, seen),
            Node::Map { inputs, .. } => inputs.iter().any(|input| visit(input, seen)),
            Node::Reduce { src, .. } | Node::Scan { src, .. } | Node::View { src, .. } | Node::Reindex { src, .. } => {
                visit(src, seen)
            }
            Node::Gather { src, index, .. } => visit(src, seen) || visit(index, seen),
        }
    }
    let mut seen = HashSet::new();
    roots.iter().any(|root| visit(root, &mut seen))
}

fn collect_inputs(roots: &[NodeRef]) -> Result<Vec<InputSpec>, CompileError> {
    fn visit(node: &NodeRef, seen: &mut HashSet<*const Node>, inputs: &mut Vec<InputSpec>) -> Result<(), CompileError> {
        if !seen.insert(Arc::as_ptr(node)) {
            return Ok(());
        }
        match node.as_ref() {
            Node::Input { name, shape, dtype } => {
                if name.is_empty() {
                    return Err(CompileError::InvalidInput("input names cannot be empty".into()));
                }
                if let Some(previous) = inputs.iter().find(|input| input.name == *name) {
                    let previous_extents = previous.shape.iter().map(|axis| axis.extent).collect::<Vec<_>>();
                    let extents = shape.iter().map(|axis| axis.extent).collect::<Vec<_>>();
                    if previous_extents != extents || previous.dtype != *dtype {
                        return Err(CompileError::InvalidInput(format!(
                            "`{name}` was declared incompatibly"
                        )));
                    }
                } else {
                    inputs.push(InputSpec {
                        name: (*name).to_string(),
                        lowered_name: name,
                        shape: shape.clone(),
                        axes: ir::axis_refs(node),
                        dtype: *dtype,
                    });
                }
            }
            Node::Const { .. } | Node::Iota { .. } => {}
            Node::Coordinate { src, .. }
            | Node::Reduce { src, .. }
            | Node::Scan { src, .. }
            | Node::View { src, .. }
            | Node::Reindex { src, .. } => visit(src, seen, inputs)?,
            Node::Map { inputs: children, .. } => {
                for child in children {
                    visit(child, seen, inputs)?;
                }
            }
            Node::Gather { src, index, .. } => {
                visit(src, seen, inputs)?;
                visit(index, seen, inputs)?;
            }
        }
        Ok(())
    }

    let mut inputs = Vec::new();
    let mut seen = HashSet::new();
    for root in roots {
        visit(root, &mut seen, &mut inputs)?;
    }
    Ok(inputs)
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

    fn prepare(&self, _schedule: &Schedule, _output_shapes: &[Vec<usize>]) -> Result<Self::Executable, CompileError> {
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
                    keepalive: Vec::new(),
                },
            );
        }
        schedule.execute_env(&mut env);
        schedule
            .outputs
            .iter()
            .zip(output_shapes)
            .map(|(name, shape)| {
                let value = env
                    .remove(name.as_str())
                    .ok_or_else(|| RunError::Backend(format!("schedule did not produce `{name}`")))?;
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
    use crate::emit_metal::{MetalProgram, emit_schedule_metal_on, emit_schedule_metal_tuned};
    use crate::metal::{Dispatch, MetalBuf, MetalDevice, MetalGraph, Pipelines, program_dispatches};
    use crate::plan::FoldSched;

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
            cost::DeviceProfile::m1_pro().with_storage(self.storage())
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
            let program = if std::env::var_os("SANIC_TUNE").is_some() {
                emit_schedule_metal_tuned(&self.profile(), schedule, &tune_fold_scheds(self, schedule))
            } else {
                emit_schedule_metal_on(&self.profile(), schedule)
            };
            // `SANIC_MSL=<path>` dumps the whole generated source — the raw
            // artifact behind every kernel name the runtime dumps print.
            if let Some(path) = std::env::var_os("SANIC_MSL") {
                std::fs::write(&path, &program.msl)
                    .unwrap_or_else(|error| panic!("SANIC_MSL: cannot write {path:?}: {error}"));
            }
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
            // Kernels for these roots store only the part they change, on the
            // promise that the rest is already in the buffer they write. Only
            // `capture` binds that buffer; a fresh allocation here would leave
            // everything the kernel skipped unwritten.
            if !schedule.agrees_in_place.is_empty() {
                return Err(RunError::Backend(format!(
                    "outputs {:?} are compiled to overwrite an input's buffer; run this \
                     program through capture(), which binds the two as one",
                    schedule.agrees_in_place
                )));
            }
            let mut buffers = HashMap::<String, MetalBuf>::new();
            for (input, buffer) in inputs.iter().zip(bindings) {
                buffers.insert(input.lowered_name.to_string(), buffer.raw.clone());
            }
            for (name, size) in &executable.program.buffers {
                buffers.insert(name.clone(), alloc_scratch(self, &executable.program, name, *size));
            }
            let dispatches = program_dispatches(&executable.program, &buffers, &executable.pipelines);
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
                    let raw = buffers
                        .get(name)
                        .cloned()
                        .ok_or_else(|| RunError::Backend(format!("Metal schedule did not produce `{name}`")))?;
                    Ok(MetalBuffer {
                        raw,
                        shape: shape.clone(),
                        dtype: executable
                            .program
                            .dtypes
                            .get(name.as_str())
                            .copied()
                            .unwrap_or(executable.program.storage),
                    })
                })
                .collect()
        }
    }

    /// A program frozen over one binding set for repeated Metal execution.
    /// Small schedules use an indirect command buffer; large schedules use
    /// ordered encoders in one command buffer. Both paths retain allocations
    /// and bindings across steps.
    ///
    /// `feedback` wires an output to an input of the NEXT step (a KV cache
    /// flowing through a decode loop). Bindings are frozen at capture, so a
    /// fed-back pair ping-pongs between two buffers: two graphs are captured
    /// with the pair's roles swapped and steps alternate between them.
    /// Everything else — weights, intermediates, CPU-written inputs like a
    /// token id — is the same buffer in both parities.
    pub struct MetalReplay<'p> {
        program: &'p Program<MetalDevice>,
        /// One frozen graph, or two when feedback swaps bindings.
        graphs: Vec<MetalGraph>,
        /// Whether each parity's graph was already dumped (`SANIC_DEBUG=3`).
        dumped: Vec<bool>,
        /// Per-parity dispatch lists — the `SANIC_DEBUG>=3` breakdown and
        /// the `SANIC_DEBUG=4` solo-timed path.
        dispatches: Vec<Vec<Dispatch>>,
        /// Per-parity outputs in compilation-root order.
        outputs: Vec<Vec<MetalBuffer>>,
        parity: usize,
    }

    impl Program<MetalDevice> {
        /// Freeze this program over `bindings` for replay —
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
            // the same pair, where the two names already denote one buffer
            let mut aliases = Vec::new();
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
                    .ok_or_else(|| RunError::Feedback(format!("program has no input named `{input_name}`")))?;
                if self.output_shapes[output] != input.concrete_shape() {
                    return Err(RunError::Feedback(format!(
                        "output {output} has shape {:?}; input `{input_name}` expects {:?}",
                        self.output_shapes[output],
                        input.concrete_shape()
                    )));
                }
                if input.dtype != self.executable.program.storage {
                    return Err(RunError::Feedback(format!(
                        "input `{input_name}` is {:?}; outputs are stored {:?}",
                        input.dtype, self.executable.program.storage
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
                // A root compiled in place already writes its predecessor's
                // buffer, so the pair is one buffer, not two that trade roles.
                if self.in_place.iter().any(|&(root, _)| root == output) {
                    aliases.push((output_name.clone(), input.lowered_name));
                } else {
                    swaps.push((output_name.clone(), input.lowered_name));
                }
            }

            let device = &self.backend;
            let executable = &self.executable;
            let mut base = HashMap::<String, MetalBuf>::new();
            for (input, buffer) in self.inputs.iter().zip(&ordered) {
                base.insert(input.lowered_name.to_string(), buffer.raw.clone());
            }
            for (name, size) in &executable.program.buffers {
                if aliases.iter().any(|(output_name, _)| output_name == name) {
                    continue;
                }
                base.insert(name.clone(), alloc_scratch(device, &executable.program, name, *size));
            }
            for (output_name, lowered_name) in &aliases {
                let buffer = base[*lowered_name].clone();
                base.insert(output_name.clone(), buffer);
            }

            let parities = if swaps.is_empty() { 1 } else { 2 };
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
                let dispatch_list = program_dispatches(&executable.program, &buffers, &executable.pipelines);
                graphs.push(device.capture(&dispatch_list));
                dispatches.push(dispatch_list);
                outputs.push(
                    self.schedule
                        .outputs
                        .iter()
                        .zip(&self.output_shapes)
                        .map(|(name, shape)| {
                            let raw = buffers
                                .get(name)
                                .cloned()
                                .ok_or_else(|| RunError::Backend(format!("Metal schedule did not produce `{name}`")))?;
                            Ok(MetalBuffer {
                                raw,
                                shape: shape.clone(),
                                dtype: executable
                                    .program
                                    .dtypes
                                    .get(name.as_str())
                                    .copied()
                                    .unwrap_or(executable.program.storage),
                            })
                        })
                        .collect::<Result<Vec<_>, RunError>>()?,
                );
            }

            Ok(MetalReplay {
                program: self,
                graphs,
                dumped: vec![false; parities],
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
        /// before calling. `SANIC_DEBUG=2` prints one line per replayed
        /// graph (its real time); `3` additionally dumps each graph's
        /// contents once; `4` abandons the frozen graph and times every
        /// kernel in-graph via encoder-boundary GPU timestamps.
        ///
        /// Errs on any command buffer error — the step's writes are
        /// untrustworthy and a decode loop must not continue on them.
        pub fn step(&mut self) -> Result<&[MetalBuffer], RunError> {
            self.step_timed().map(|(outputs, _)| outputs)
        }

        /// [`Self::step`], returning the replayed command buffer's GPU
        /// residency in seconds (see
        /// [`crate::metal::MetalDevice::run_graph_timed`]). Under
        /// `SANIC_DEBUG=4` the step runs with in-graph timestamp sampling
        /// and the returned time is that command buffer's residency — still
        /// a real step time (or, on a device without counter sampling, the
        /// sync-floored per-dispatch sum the dump's footer flags).
        pub fn step_timed(&mut self) -> Result<(&[MetalBuffer], f64), RunError> {
            let parity = self.advance();
            if crate::debug_level() >= 4 {
                let seconds = run_debug(
                    &self.program.backend,
                    &self.program.executable.program,
                    &self.program.schedule,
                    &self.dispatches[parity],
                );
                return Ok((&self.outputs[parity], seconds));
            }
            let watch = (crate::debug_level() >= 2)
                .then(|| self.program.backend.clock())
                .flatten();
            let seconds = self
                .program
                .backend
                .run_graph_timed(&self.graphs[parity])
                .map_err(RunError::Backend)?;
            if crate::debug_level() >= 2 {
                if crate::debug_level() >= 3 && !std::mem::replace(&mut self.dumped[parity], true) {
                    dump_graph(
                        &self.program.executable.program,
                        &self.program.schedule,
                        &self.dispatches[parity],
                        parity,
                    );
                }
                print_step_line(
                    &self.program.backend,
                    &self.program.executable.program,
                    &self.program.schedule,
                    seconds,
                    watch.and_then(|watch| watch.read()),
                );
            }
            Ok((&self.outputs[parity], seconds))
        }

        fn advance(&mut self) -> usize {
            let parity = self.parity;
            self.parity = (parity + 1) % self.graphs.len();
            parity
        }

        /// The outputs of the last completed step.
        pub fn last_outputs(&self) -> &[MetalBuffer] {
            let last = (self.parity + self.graphs.len() - 1) % self.graphs.len();
            &self.outputs[last]
        }
    }

    /// Allocate one scratch buffer from `program.buffers`. Tensor
    /// intermediates are sized at the boundary storage width; an ARGUMENT
    /// BUFFER is a table of 64-bit GPU addresses counted in f32 elements —
    /// its byte size never narrows with the tensor storage.
    fn alloc_scratch(
        device: &MetalDevice,
        program: &MetalProgram,
        name: &str,
        elements: usize,
    ) -> crate::metal::MetalBuf {
        let is_argbuf = program.stages.iter().any(|stage| stage.argbuf.as_deref() == Some(name));
        if is_argbuf {
            device.alloc_f32(elements)
        } else {
            // an output that pinned its own width is registered in `dtypes`
            let width = program.dtypes.get(name).copied().unwrap_or(program.storage);
            device.alloc_elems(elements, width)
        }
    }

    /// Logical bytes per buffer name. An allocation's `byte_len` would
    /// overcount: a zero-copy checkpoint tensor is a SLICE of the whole
    /// weights file.
    fn logical_byte_table(program: &MetalProgram) -> HashMap<&str, f64> {
        let mut logical_bytes = HashMap::<&str, f64>::new();
        for (name, elements) in &program.buffers {
            logical_bytes.insert(name, *elements as f64 * program.storage.bytes_per_element());
        }
        for (name, axes) in &program.inputs {
            let elements: usize = axes.iter().map(|a| a.extent()).product();
            let width = program.dtypes.get(*name).map_or(4.0, |dtype| dtype.bytes_per_element());
            logical_bytes.insert(name, elements as f64 * width);
        }
        logical_bytes
    }

    /// Planned cost by the output name each dispatch writes — the fused-stage
    /// costs the planner CHOSE this schedule by; other stage kinds are unpriced.
    fn stage_plans(schedule: &Schedule) -> HashMap<&str, f64> {
        use crate::partition::Stage;
        let mut planned_costs = HashMap::new();
        for stage in &schedule.stages {
            if let Stage::Fused { spec, .. } = stage {
                planned_costs.insert(crate::partition::stage_output(stage), spec.cost);
            }
        }
        planned_costs
    }

    /// Logical DRAM traffic of one dispatch: its stage's inputs plus output —
    /// except gather stages, whose table traffic is known statically and is
    /// NOT the table's size (see [`gather_stage_bytes`]).
    fn stage_bytes(
        stage: &crate::emit_metal::MetalStageInfo,
        logical_bytes: &HashMap<&str, f64>,
        gather_bytes: &HashMap<&str, f64>,
    ) -> f64 {
        if let Some(&bytes) = gather_bytes.get(stage.output.as_str()) {
            return bytes;
        }
        stage
            .inputs
            .iter()
            .chain(std::iter::once(&stage.output))
            .map(|name| logical_bytes.get(name.as_str()).copied().unwrap_or(0.0))
            .sum()
    }

    /// Bytes a gather stage moves, known statically: each output element
    /// loads exactly ONE table element, so the table's traffic is the OUTPUT
    /// volume times its element width — an embedding lookup touches
    /// `sequence×hidden` elements, never the whole table that
    /// [`logical_byte_table`] would charge it. The index and the written
    /// output keep their logical sizes.
    fn gather_stage_bytes<'sched>(
        schedule: &'sched Schedule,
        program: &MetalProgram,
        logical_bytes: &HashMap<&str, f64>,
    ) -> HashMap<&'sched str, f64> {
        use crate::partition::Stage;
        let element_width = |name: &str| {
            program
                .dtypes
                .get(name)
                .map_or(program.storage.bytes_per_element(), |dtype| dtype.bytes_per_element())
        };
        let mut traffic = HashMap::new();
        for stage in &schedule.stages {
            if let Stage::Gather {
                inputs, output, exec, ..
            } = stage
            {
                let table_reads = ir::volume(exec) as f64 * element_width(inputs[0]);
                let rest: f64 = inputs[1..]
                    .iter()
                    .chain(std::iter::once(&output.as_str()))
                    .filter_map(|name| logical_bytes.get(*name))
                    .sum();
                traffic.insert(output.as_str(), table_reads + rest);
            }
        }
        traffic
    }

    /// The `SANIC_DEBUG=2` runtime line — one per replayed graph, tinygrad
    /// style: the REAL command buffer time (this IS the decode number, unlike
    /// the `=4` dump's sync-floored sum), the whole step against the plan's
    /// fold total, and the step's aggregate DRAM position. Unplanned stages
    /// and inter-dispatch bubbles land in the ratio on purpose: it is the
    /// end-to-end honesty number, not per-kernel calibration.
    fn print_step_line(
        device: &MetalDevice,
        program: &MetalProgram,
        schedule: &Schedule,
        seconds: f64,
        clock: Option<crate::metal::Clock>,
    ) {
        let logical_bytes = logical_byte_table(program);
        let gather_bytes = gather_stage_bytes(schedule, program, &logical_bytes);
        let plans = stage_plans(schedule);
        let mut planned = 0.0f64;
        let mut bytes = 0.0f64;
        for stage in &program.stages {
            bytes += stage_bytes(stage, &logical_bytes, &gather_bytes);
            if let Some(cost) = plans.get(stage.output.as_str()) {
                planned += cost;
            }
        }
        let calibration = if planned > 0.0 {
            format!("vs plan Σ ×{:.2}", seconds / planned)
        } else {
            String::new()
        };
        let peak_fraction = bytes / seconds.max(1e-9) / device.profile().hbm_bandwidth;
        // The clock this step actually ran at: a line measured below the
        // GPU's top DVFS state is not comparable to one measured at it, and
        // saying so beats silently publishing a throttled number.
        let clock = match clock {
            Some(clock) if !clock.at_peak() => format!("  \x1b[33m{clock}\x1b[0m"),
            _ => String::new(),
        };
        eprintln!(
            "*** metal batched {} {:6.2}ms GPU  {calibration}  ~{:.0}MB bw {:3.0}%{clock}",
            program.stages.len(),
            seconds * 1e3,
            bytes / 1e6,
            peak_fraction * 100.0,
        );
    }

    /// The frozen graph's contents, once per parity at `SANIC_DEBUG=3`:
    /// execution order, each kernel's planned cost and share of the fold
    /// plan. Plan-side only — measured per-kernel time is `SANIC_DEBUG=4`,
    /// which samples GPU timestamps at encoder boundaries inside the one
    /// command buffer (re-timing each kernel solo where the device can't
    /// sample counters).
    fn dump_graph(program: &MetalProgram, schedule: &Schedule, dispatches: &[Dispatch], parity: usize) {
        let logical_bytes = logical_byte_table(program);
        let gather_bytes = gather_stage_bytes(schedule, program, &logical_bytes);
        let plans = stage_plans(schedule);
        let plan_total = program
            .stages
            .iter()
            .filter_map(|stage| plans.get(stage.output.as_str()))
            .sum::<f64>()
            .max(1e-12);
        eprintln!(
            "*** metal graph parity {parity}: {} kernels, plan Σ {:.2}ms",
            dispatches.len(),
            plan_total * 1e3,
        );
        for (index, (stage, dispatch)) in program.stages.iter().zip(dispatches).enumerate() {
            let plan = match plans.get(stage.output.as_str()) {
                Some(&cost) if cost > 0.0 => {
                    format!("plan {:7.0}us {:4.1}%", cost * 1e6, 100.0 * cost / plan_total)
                }
                _ => format!("plan {:>7}        ", "--"),
            };
            let block = dispatch.threadgroup_threads();
            eprintln!(
                "***   {index:4} {:<12} grid=({:>6},1,1) block=({:>4},1,1) {plan}  ~{:>6.1}MB  {}",
                stage.output,
                dispatch.grid.div_ceil(block),
                block,
                stage_bytes(stage, &logical_bytes, &gather_bytes) / 1e6,
                stage.kernel,
            );
        }
    }

    /// The per-kernel runtime dump (`SANIC_DEBUG=2` one-shot, `=4` replay).
    /// One line per launch, printed after the step so shares are exact:
    ///
    /// ```text
    /// *** metal  407 Out32        grid=(   126,1,1) block=(1024,1,1)   7934us 16.6%  plan ×1.28  ~ 525.9MB bw  33%  fold_batch4_vocab128256_over_hidden2048
    /// ```
    ///
    /// The index and OUTPUT name match the `SANIC_DEBUG=1` schedule dump, so
    /// the two dumps cross-reference. `plan ×r` is measured over the cost
    /// the planner CHOSE this schedule by (fused stages), so the dump audits
    /// the cost model — the footer's `plan Σ` is its aggregate calibration
    /// on this machine. `bw`
    /// is the fraction of the device's memory bandwidth actually achieved;
    /// bytes are logical buffer sizes (an upper bound), except gathers,
    /// whose table traffic is statically the output volume (see
    /// [`gather_stage_bytes`]); a still-implausible ratio prints `--`.
    ///
    /// Times are GPU timestamps sampled at each encoder's boundaries inside
    /// ONE command buffer (`MetalDevice::run_kernel_timed`). Production runs
    /// ONE concurrent encoder instead — stage-boundary sampling is what the
    /// per-kernel encoders pay for — so the profiled wall carries the
    /// boundary cost production doesn't. Where the device can't sample
    /// counters, the fallback is one command buffer per dispatch: accurate
    /// per kernel, but the submits add a sync floor, so that SUM is a debug
    /// number and the footer says so.
    fn run_debug(device: &MetalDevice, program: &MetalProgram, schedule: &Schedule, dispatches: &[Dispatch]) -> f64 {
        let logical_bytes = logical_byte_table(program);
        let gather_bytes = gather_stage_bytes(schedule, program, &logical_bytes);
        let stage_info = stage_plans(schedule);

        let (times, wall_seconds) = match device.run_kernel_timed(dispatches) {
            Some((wall, times)) => (times, Some(wall)),
            None => (device.run_each_timed(dispatches), None),
        };
        if std::env::var_os("SANIC_NANSCAN").is_some() {
            for (stage, dispatch) in program.stages.iter().zip(dispatches) {
                let width = program.storage.bytes_per_element();
                let count = (dispatch.output.byte_len() as f64 / width) as usize;
                let data = device.read_as_f32(&dispatch.output, count, program.storage);
                if let Some(at) = data.iter().position(|v| !v.is_finite()) {
                    eprintln!(
                        "*** nanscan: `{}` is the first stage with a non-finite output \
                         ({} at element {at} of {count})",
                        stage.output, data[at],
                    );
                    break;
                }
            }
        }
        let total = times.iter().sum::<f64>().max(1e-12);
        let (mut fused_measured, mut fused_planned) = (0.0f64, 0.0f64);
        for (index, ((stage, dispatch), &seconds)) in program.stages.iter().zip(dispatches).zip(&times).enumerate() {
            let bytes = stage_bytes(stage, &logical_bytes, &gather_bytes);
            let micros = seconds * 1e6;
            let time = if micros >= 1000.0 {
                format!("\x1b[33m{micros:7.0}us\x1b[0m") // slow launch: yellow
            } else {
                format!("{micros:7.0}us")
            };
            let plan = match stage_info.get(stage.output.as_str()).copied() {
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
            let block = dispatch.threadgroup_threads();
            eprintln!(
                "*** metal {index:4} {:<12} grid=({:>6},1,1) block=({:>4},1,1) {time} {:4.1}%  {plan} ~{:>6.1}MB {bandwidth}  {}",
                stage.output,
                dispatch.grid.div_ceil(block),
                block,
                100.0 * seconds / total,
                bytes / 1e6,
                stage.kernel,
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
        match wall_seconds {
            Some(wall) => eprintln!(
                "*** metal step: {} launches, {:.2}ms GPU wall, Σ kernels {:.2}ms (in-graph timestamps)",
                times.len(),
                wall * 1e3,
                total * 1e3,
            ),
            None => eprintln!(
                "*** metal step: {} launches {:.2}ms GPU (per-launch sync; production time = run_timed)",
                times.len(),
                total * 1e3,
            ),
        }
        eprintln!("*** metal top:  {top}  rest {rest:.0}%  |  {calibration}");
        wall_seconds.unwrap_or_else(|| times.iter().sum())
    }

    /// `SANIC_TUNE=1` — measured schedule tuning. Every plausibly
    /// competitive fold schedule is timed IN CONTEXT: substituted into the
    /// full step over zeroed scratch bindings and judged by the
    /// production-replay wall time, and the fastest overrules the analytic
    /// chooser. Solo microbenchmarks mislead here — candidates re-reading
    /// warm scratch reward extra parallelism that a DRAM-bound step
    /// punishes (measured: solo winners made the real step 12% slower) —
    /// so the step itself is the instrument.
    ///
    /// Isomorphic stages form one FAMILY (keyed on the scalar probe's
    /// canonical source): all of a family's stages swap to a candidate
    /// together and one wall time judges them. Candidates priced beyond 2×
    /// the family's analytic best are not worth a replay; stages past the
    /// direct-bind cap keep the analytic choice; and the tuned program is
    /// re-measured against the analytic one at the end — a tune that does
    /// not win ships nothing.
    ///
    /// Every verdict is a DIFFERENCE of adjacent measurements: each family
    /// re-measures its own baseline next to its rivals, because the GPU's
    /// clock drifts over a tuning run and a stale baseline hands every
    /// candidate a free half-millisecond. Overruling needs a 1% margin;
    /// shipping needs the combined program to beat a fresh baseline.
    ///
    /// Costs compile time (llama-3.2-1B: 5.2s → 13.3s) to buy ~6% (bf16) /
    /// ~14% (f32) per step, so it is opt-in rather than the default.
    fn tune_fold_scheds(device: &MetalDevice, schedule: &Schedule) -> HashMap<String, FoldSched> {
        use crate::emit_metal::{METAL_MAX_BUFFERS, MSL_HEADER, canonical_source, emit_fused_metal_sched_with};
        use crate::partition::Stage;
        use crate::plan::{fold_sched, priced_fold_sched_candidates};

        /// Fastest of a few replays — the min sheds clock ramp and paging.
        const ROUNDS: usize = 3;

        // The analytic program over zeroed scratch is the instrument: the
        // real dispatch list, barrier schedule, and DRAM pressure.
        let program = emit_schedule_metal_on(&device.profile(), schedule);
        let pipelines = device.compile(&program.msl);
        let mut bindings: HashMap<String, MetalBuf> = HashMap::new();
        for (name, axes) in &program.inputs {
            let elements: usize = axes.iter().map(|axis| axis.extent()).product();
            let dtype = program.dtypes.get(*name).copied().unwrap_or(program.storage);
            bindings.insert(name.to_string(), device.alloc_elems(elements.max(1), dtype));
        }
        for (name, elements) in &program.buffers {
            bindings.insert(name.clone(), alloc_scratch(device, &program, name, *elements));
        }
        let base = program_dispatches(&program, &bindings, &pipelines);
        // Each replay is bracketed by a clock watch and DISCARDED if the GPU
        // ran it below its top DVFS state — a throttled sample is ~25% slow
        // for reasons the schedule had nothing to do with, and would hand a
        // candidate a win or a loss it did not earn. Falls back to the
        // unfiltered minimum if the GPU never reached peak at all, so a warm
        // machine degrades to the previous behaviour rather than tuning
        // nothing.
        let step_wall = |dispatches: &[Dispatch]| -> f64 {
            let graph = device.capture(dispatches);
            let (mut at_peak, mut any) = (f64::INFINITY, f64::INFINITY);
            for _ in 0..ROUNDS {
                let watch = device.clock();
                let Ok(seconds) = device.run_graph_timed(&graph) else {
                    continue;
                };
                any = any.min(seconds);
                if watch.and_then(|watch| watch.read()).is_none_or(|clock| clock.at_peak()) {
                    at_peak = at_peak.min(seconds);
                }
            }
            if at_peak.is_finite() { at_peak } else { any }
        };
        // Warm the machine before ANY verdict: the first replays carry clock
        // ramp and paging, and a cold baseline would flatter every later
        // candidate (measured: an unwarmed baseline "lost" to noise-level
        // rivals in every family).
        let _ = step_wall(&base);
        let analytic_wall = step_wall(&base);

        // Group the fused stages into isomorphism families —
        // `schedule.stages`, `program.stages`, and `base` are index-parallel.
        let mut families: Vec<(String, Vec<usize>)> = Vec::new();
        for (index, stage) in schedule.stages.iter().enumerate() {
            let Stage::Fused {
                spec,
                fold_node,
                epilogue_node,
                epi_fold_read,
                ..
            } = stage
            else {
                continue;
            };
            let epi = epilogue_node.as_ref().map(|node| (node, *epi_fold_read));
            let probe = emit_fused_metal_sched_with(
                "tune_probe",
                &spec.carrier,
                spec.streaming_axis,
                fold_node,
                FoldSched::scalar(),
                epi,
                device.storage(),
            );
            if probe.inputs.len() + 1 > METAL_MAX_BUFFERS {
                continue; // bindless stage: keep the analytic choice
            }
            let family = canonical_source(&probe);
            match families.iter_mut().find(|(key, _)| *key == family) {
                Some((_, stage_indices)) => stage_indices.push(index),
                None => families.push((family, vec![index])),
            }
        }

        // A family whose whole planned time is a sliver of the step cannot
        // win more than measurement noise — don't spend replays on it (the
        // norm folds are ~33 single-stage families of ~25µs each).
        let family_price = |stage_indices: &[usize]| -> f64 {
            let Stage::Fused { spec, fold_node, .. } = &schedule.stages[stage_indices[0]] else {
                unreachable!("families hold fused stages");
            };
            let best = priced_fold_sched_candidates(fold_node, spec.streaming_axis, &spec.carrier, &device.profile())
                .into_iter()
                .filter_map(|(_, priced)| priced.map(|price| price.time))
                .fold(f64::INFINITY, f64::min);
            if best.is_finite() {
                best * stage_indices.len() as f64
            } else {
                0.0
            }
        };
        let planned_total: f64 = families
            .iter()
            .map(|(_, stage_indices)| family_price(stage_indices))
            .sum::<f64>()
            .max(1e-12);

        let mut winners: HashMap<String, FoldSched> = HashMap::new();
        for (_, stage_indices) in &families {
            if family_price(stage_indices) < 0.02 * planned_total {
                continue;
            }
            let Stage::Fused {
                spec,
                fold_node,
                epilogue_node,
                epi_fold_read,
                ..
            } = &schedule.stages[stage_indices[0]]
            else {
                unreachable!("families hold fused stages");
            };
            let analytic = fold_sched(fold_node, spec.streaming_axis, &spec.carrier, &device.profile());
            let priced = priced_fold_sched_candidates(fold_node, spec.streaming_axis, &spec.carrier, &device.profile());
            let best_price = priced
                .iter()
                .filter_map(|(_, priced)| priced.map(|price| price.time))
                .fold(f64::INFINITY, f64::min);
            let candidates: Vec<FoldSched> = priced
                .into_iter()
                .filter_map(|(candidate, priced)| priced.map(|price| (candidate, price.time)))
                .filter(|&(candidate, t)| candidate != analytic && t <= 2.0 * best_price)
                .map(|(candidate, _)| candidate)
                .collect();
            if candidates.is_empty() {
                continue;
            }
            let epi = epilogue_node.as_ref().map(|node| (node, *epi_fold_read));
            let mut msl = String::from(MSL_HEADER);
            let mut emitted = Vec::new();
            for (index, &candidate) in candidates.iter().enumerate() {
                let kernel = emit_fused_metal_sched_with(
                    &format!("cand_{index}"),
                    &spec.carrier,
                    spec.streaming_axis,
                    fold_node,
                    candidate,
                    epi,
                    device.storage(),
                );
                msl.push_str(&kernel.msl.replace(MSL_HEADER, ""));
                msl.push('\n');
                emitted.push(kernel);
            }
            let candidate_pipes = device.compile(&msl);

            // The baseline must be measured ADJACENT to its rivals: the GPU's
            // clock drifts across a whole tuning run, so a stale baseline
            // loses to noise (measured: every family "won" ~0.5ms against
            // the start-of-run figure, and the combined program shipped
            // nothing of it).
            let family_baseline = step_wall(&base);
            let (mut winner, mut winner_wall) = (analytic, family_baseline);
            for (kernel, &candidate) in emitted.iter().zip(&candidates) {
                let mut dispatches = base.clone();
                for &stage in stage_indices {
                    dispatches[stage].pipe = candidate_pipes.get(&kernel.name);
                    dispatches[stage].grid = kernel.grid_size;
                }
                let wall = step_wall(&dispatches);
                if wall < winner_wall {
                    winner = candidate;
                    winner_wall = wall;
                }
            }
            // Overrule only past the noise floor — the analytic choice wins
            // ties, exactly as scalar wins ties in the chooser.
            let wins = winner != analytic && winner_wall < family_baseline * 0.99;
            if crate::debug_level() >= 1 {
                eprintln!(
                    "*** tune {}×{}: {} rivals vs analytic {analytic:?} at {:.2}ms{}",
                    spec.output_name,
                    stage_indices.len(),
                    candidates.len(),
                    family_baseline * 1e3,
                    if wins {
                        format!(" → {winner:?} {:.2}ms", winner_wall * 1e3)
                    } else {
                        String::from(" — none beats it past noise")
                    },
                );
            }
            if wins {
                for &stage in stage_indices {
                    let Stage::Fused { spec, .. } = &schedule.stages[stage] else {
                        unreachable!("families hold fused stages");
                    };
                    winners.insert(spec.output_name.clone(), winner);
                }
            }
        }

        // The combined verdict must beat the analytic program on the same
        // instrument, against a FRESH baseline (the machine is warmest now)
        // — a tune that does not win ships nothing.
        if !winners.is_empty() {
            let tuned_program = emit_schedule_metal_tuned(&device.profile(), schedule, &winners);
            let tuned_pipes = device.compile(&tuned_program.msl);
            let tuned_wall = step_wall(&program_dispatches(&tuned_program, &bindings, &tuned_pipes));
            let analytic_fresh = step_wall(&base).min(analytic_wall);
            let ships = tuned_wall < analytic_fresh * 0.995;
            if crate::debug_level() >= 1 {
                eprintln!(
                    "*** tune verdict: analytic {:.2}ms, tuned {:.2}ms — {} ships",
                    analytic_fresh * 1e3,
                    tuned_wall * 1e3,
                    if ships { "tuned" } else { "analytic" },
                );
            }
            if !ships {
                winners.clear();
            }
        }
        winners
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
            let required_bytes = dtype.nbytes(elements);
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
            self.read_as_f32(&buffer.raw, buffer.shape.iter().product::<usize>().max(1), buffer.dtype)
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
