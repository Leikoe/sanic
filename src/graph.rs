//! A stepped computation with persistent state — the public execution API.
//!
//! A [`Graph`] collects what a model actually is at runtime: ordinary
//! outputs (logits), plus STATE — tensors that persist across steps, where
//! each step reads the previous value and declares its successor (a KV
//! cache row-write, a weight update). The graph stays pure; statefulness is
//! entirely in the declaration `update(state, successor)`.
//!
//! [`Graph::compile_for`] lowers the whole step for one target and returns
//! the best program that target admits — on Metal the dispatch sequence is
//! frozen into replayable graphs (indirect command buffers when the size
//! allows, ordered encoders otherwise), with state feedback wired at
//! compile time: state successors are scheduled as roots, and each state's
//! output buffer becomes the NEXT step's input, ping-ponged internally.
//! No capture/replay staircase in sight; [`Machine::step`] just runs.

use crate::ir::{Axis, Dtype};
use crate::tensor::Tensor;

/// One persistent tensor: its per-step input and, once declared, the value
/// that replaces it for the next step.
struct State {
    name: &'static str,
    input: Tensor,
    successor: Option<Tensor>,
}

/// A stepped computation under construction: outputs plus state
/// declarations. Build tensors freely; register what survives the step.
#[derive(Default)]
pub struct Graph {
    states: Vec<State>,
    outputs: Vec<(&'static str, Tensor)>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph::default()
    }

    /// Declare a persistent tensor and get this step's view of it. `dtype`
    /// is its storage at the step boundary — for a narrowed-storage target
    /// it must match the target's boundary policy, which
    /// [`Graph::compile_for`] checks.
    pub fn state(&mut self, name: impl Into<String>, shape: impl AsRef<[Axis]>, dtype: Dtype) -> Tensor {
        let input = Tensor::input(name, shape, dtype);
        let crate::ir::Node::Input { name, .. } = input.node().as_ref() else {
            unreachable!("Tensor::input builds an Input node")
        };
        self.states.push(State {
            name,
            input: input.clone(),
            successor: None,
        });
        input
    }

    /// Declare `successor` as the value `state` carries into the next step.
    pub fn update(&mut self, state: &Tensor, successor: Tensor) {
        let slot = self
            .states
            .iter_mut()
            .find(|s| std::sync::Arc::ptr_eq(s.input.node(), state.node()))
            .expect("update: the tensor is not a declared state of this graph");
        assert!(
            slot.successor.is_none(),
            "update: state `{}` already has a successor",
            slot.name
        );
        assert_eq!(
            state.shape(),
            successor.shape(),
            "update: successor shape must match state `{}`",
            slot.name
        );
        slot.successor = Some(successor);
    }

    /// Declare an ordinary named output of the step.
    pub fn output(&mut self, name: &'static str, value: Tensor) {
        self.outputs.push((name, value));
    }

    /// Every root of the step, state successors first (producers before
    /// consumers, so a later root reaching a successor reuses its buffer).
    pub fn roots(&self) -> Vec<crate::ir::NodeRef> {
        let mut roots = Vec::with_capacity(self.states.len() + self.outputs.len());
        for state in &self.states {
            let successor = state
                .successor
                .as_ref()
                .unwrap_or_else(|| panic!("state `{}` has no successor", state.name));
            roots.push(successor.node().clone());
        }
        for (_, value) in &self.outputs {
            roots.push(value.node().clone());
        }
        roots
    }
}

#[cfg(target_os = "macos")]
mod metal_target {
    use super::*;
    use crate::compile::{Compile, CompileError, MetalBuffer, MetalReplay, Program, RunError};
    use crate::metal::MetalDevice;

    /// A [`Graph`] compiled for one Metal device: the lowered program plus
    /// the state wiring [`StepProgram::instantiate`] needs.
    pub struct StepProgram {
        program: Program<MetalDevice>,
        /// (root index, state input name, concrete shape) per state.
        feedback: Vec<(usize, &'static str, Vec<usize>)>,
        /// Output name → root index.
        output_roots: Vec<(&'static str, usize)>,
    }

    impl Graph {
        /// Lower the whole step for `device` and return the best program the
        /// target admits. State feedback is wired here, not by the caller.
        pub fn compile_for(&self, device: &MetalDevice) -> Result<StepProgram, CompileError> {
            for state in &self.states {
                let declared = crate::ir::input_dtypes(state.input.node())
                    .first()
                    .map(|(_, dtype)| *dtype);
                if declared != Some(device.storage()) {
                    return Err(CompileError::Backend(format!(
                        "state `{}` is declared {:?} but the target stores step \
                         boundaries as {:?}",
                        state.name,
                        declared,
                        device.storage()
                    )));
                }
            }
            let program = self.roots().compile(device)?;
            let feedback = self
                .states
                .iter()
                .enumerate()
                .map(|(index, state)| {
                    let shape: Vec<usize> = state.input.shape().iter().map(|axis| axis.extent()).collect();
                    (index, state.name, shape)
                })
                .collect();
            let output_roots = self
                .outputs
                .iter()
                .enumerate()
                .map(|(index, (name, _))| (*name, self.states.len() + index))
                .collect();
            Ok(StepProgram {
                program,
                feedback,
                output_roots,
            })
        }
    }

    impl StepProgram {
        pub fn kernel_count(&self) -> usize {
            self.program.kernel_count()
        }

        pub fn input_names(&self) -> impl Iterator<Item = &str> {
            self.program.input_names()
        }

        /// Bind the non-state inputs (weights, host-written scalars) and get
        /// a runnable machine. State buffers are allocated here, zeroed;
        /// each state's output feeds the next step's input.
        pub fn instantiate<'a>(
            &self,
            device: &MetalDevice,
            bindings: impl IntoIterator<Item = (&'a str, &'a MetalBuffer)>,
        ) -> Result<Machine<'_>, RunError> {
            let state_buffers: Vec<(&'static str, MetalBuffer)> = self
                .feedback
                .iter()
                .map(|(_, name, shape)| {
                    let elements = shape.iter().product::<usize>().max(1);
                    let raw = device.alloc_elems(elements, device.storage());
                    (
                        *name,
                        device
                            .tensor_from_raw(raw, shape.clone(), device.storage())
                            .expect("state buffer"),
                    )
                })
                .collect();
            let user: Vec<(&str, &MetalBuffer)> = bindings.into_iter().collect();
            let all: Vec<(&str, &MetalBuffer)> = user
                .iter()
                .copied()
                .chain(state_buffers.iter().map(|(n, b)| (*n as &str, b)))
                .collect();
            let pairs: Vec<(usize, &str)> = self.feedback.iter().map(|(index, name, _)| (*index, *name)).collect();
            let replay = self.program.capture(all, &pairs)?;
            Ok(Machine {
                replay,
                output_roots: &self.output_roots,
            })
        }
    }

    /// A live instance: bound buffers, frozen dispatch graphs, advancing
    /// state. One [`Machine::step`] runs the whole step on the GPU.
    pub struct Machine<'p> {
        replay: MetalReplay<'p>,
        output_roots: &'p [(&'static str, usize)],
    }

    impl Machine<'_> {
        /// Run one step; state advances internally. Returns the step's GPU
        /// time in seconds.
        pub fn step(&mut self) -> Result<f64, RunError> {
            self.replay.step_timed().map(|(_, seconds)| seconds)
        }

        /// A named output's buffer as of the LAST completed step.
        pub fn output(&mut self, name: &str) -> MetalBuffer {
            let &(_, index) = self
                .output_roots
                .iter()
                .find(|(n, _)| *n == name)
                .unwrap_or_else(|| panic!("no output named `{name}`"));
            self.replay.last_outputs()[index].clone()
        }
    }
}

#[cfg(target_os = "macos")]
pub use metal_target::{Machine, StepProgram};
