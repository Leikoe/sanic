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
    /// `(name, value, storage override)`. `None` stores at the target's
    /// boundary width, which is what almost everything wants.
    outputs: Vec<(&'static str, Tensor, Option<Dtype>)>,
    /// Γ's Caller entries: input name → the storage its bound buffer uses.
    /// A term is a free variable; how the buffer that binds it is stored is
    /// declared HERE, at the program layer, by whoever owns the buffer.
    declared: std::collections::HashMap<String, Dtype>,
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
        let input = self.input(name, shape, dtype);
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

    /// Declare an input and the storage its buffer will be bound at — Γ's
    /// Caller row. The term itself carries only name and shape; a bf16
    /// checkpoint weight is bf16 because THIS says so, not because the
    /// mathematics knows.
    pub fn input(&mut self, name: impl Into<String>, shape: impl AsRef<[Axis]>, dtype: Dtype) -> Tensor {
        let input = Tensor::input(name, shape);
        let crate::ir::Node::Input { name, .. } = input.node().as_ref() else {
            unreachable!("Tensor::input builds an Input node")
        };
        self.declared.insert((*name).to_string(), dtype);
        input
    }

    /// The declared storage of every input registered on this graph.
    pub fn declarations(&self) -> &std::collections::HashMap<String, Dtype> {
        &self.declared
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
        self.outputs.push((name, value, None));
    }

    /// An output stored at an explicit width rather than the target's boundary
    /// policy.
    ///
    /// Narrowing is a numeric choice, and it is the right one for tensors of
    /// values: a bf16 activation loses nothing that matters. It is the WRONG
    /// one for a tensor of indices, which are exact or they are wrong — bf16
    /// represents integers exactly only to 256, so a token id past that comes
    /// back as a different token. Same reason an argument buffer of addresses
    /// may never narrow.
    pub fn output_at(&mut self, name: &'static str, value: Tensor, dtype: Dtype) {
        self.outputs.push((name, value, Some(dtype)));
    }

    /// The states whose successor may simply overwrite them, as `(root index,
    /// state name)`.
    ///
    /// `where(c, x, s)` differs from `s` only where `c` is nonzero, so such a
    /// successor is the old state with a part replaced: writing it into `s`'s
    /// own buffer leaves precisely the points it does not store, which already
    /// hold the right values. The one thing that can go wrong is somebody else
    /// still wanting the old contents, and the count settles it — the whole
    /// step reads `s`'s values exactly once, in the fallback arm this erases.
    ///
    /// The law is target-independent; only [`Graph::compile_for`] acts on it
    /// today, and it exists only where there is a device to compile for.
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    fn overwritable_states(&self, roots: &[crate::ir::NodeRef]) -> Vec<(usize, &'static str)> {
        use crate::ir::{MapOp, Node};
        self.states
            .iter()
            .enumerate()
            .filter(|(_, state)| {
                let Some(successor) = &state.successor else {
                    return false;
                };
                let Node::Map {
                    op: MapOp::Where,
                    inputs,
                } = successor.node().as_ref()
                else {
                    return false;
                };
                std::sync::Arc::ptr_eq(&inputs[2], state.input.node()) && value_reads(roots, state.input.node()) == 1
            })
            .map(|(index, state)| (index, state.name))
            .collect()
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
        for (_, value, _) in &self.outputs {
            roots.push(value.node().clone());
        }
        roots
    }
}

/// How many places across `roots` read `target`'s VALUES.
///
/// Every graph edge counts but one: a [`crate::ir::Node::Coordinate`]'s source.
/// A coordinate is an index along one of its operand's dimensions, so it is a
/// function of that operand's SHAPE and never of its contents — the operand is
/// not loaded, and neither is anything beneath it.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
fn value_reads(roots: &[crate::ir::NodeRef], target: &crate::ir::NodeRef) -> usize {
    use crate::ir::{Node, NodeRef, children};
    use std::sync::Arc;

    fn walk(node: &NodeRef, target: &NodeRef, seen: &mut std::collections::HashSet<*const Node>, reads: &mut usize) {
        if matches!(node.as_ref(), Node::Coordinate { .. }) || !seen.insert(Arc::as_ptr(node)) {
            return;
        }
        for child in children(node) {
            if Arc::ptr_eq(&child, target) {
                *reads += 1;
            }
            walk(&child, target, seen, reads);
        }
    }

    let mut seen = std::collections::HashSet::new();
    let mut reads = 0;
    for root in roots {
        walk(root, target, &mut seen, &mut reads);
    }
    reads
}

#[cfg(target_os = "macos")]
mod metal_target {
    use super::*;
    use crate::compile::{CompileError, MetalBuffer, MetalReplay, Program, RunError};
    use crate::metal::MetalDevice;

    /// A [`Graph`] compiled for one Metal device: the lowered program plus
    /// the state wiring [`StepProgram::instantiate`] needs.
    pub struct StepProgram {
        program: Program<MetalDevice>,
        /// (root index, state input name, concrete shape, declared storage)
        /// per state — states allocate at their own declaration.
        feedback: Vec<(usize, &'static str, Vec<usize>, Dtype)>,
        /// Output name → root index.
        output_roots: Vec<(&'static str, usize)>,
    }

    impl Graph {
        /// Lower the whole step for `device` and return the best program the
        /// target admits. State feedback is wired here, not by the caller.
        pub fn compile_for(
            &self,
            device: &MetalDevice,
            policy: crate::cost::Policy,
        ) -> Result<StepProgram, CompileError> {
            for state in &self.states {
                let declared = self.declared.get(state.name).copied();
                if declared != Some(policy.boundary) {
                    return Err(CompileError::Backend(format!(
                        "state `{}` is declared {:?} but the compilation's policy \
                         stores step boundaries as {:?}",
                        state.name, declared, policy.boundary
                    )));
                }
            }
            let roots = self.roots();
            let overwritable = self.overwritable_states(&roots);
            // states store at the boundary policy; only declared outputs may pin
            let mut output_dtypes = vec![None; self.states.len()];
            output_dtypes.extend(self.outputs.iter().map(|(_, _, dtype)| *dtype));
            let program = crate::compile::compile_roots_in_place(
                roots,
                overwritable,
                output_dtypes,
                self.declared.clone(),
                policy,
                device,
            )?;
            let feedback = self
                .states
                .iter()
                .enumerate()
                .map(|(index, state)| {
                    let shape: Vec<usize> = state.input.shape().iter().map(|axis| axis.extent()).collect();
                    let dtype = self.declared[state.name];
                    (index, state.name, shape, dtype)
                })
                .collect();
            let output_roots = self
                .outputs
                .iter()
                .enumerate()
                .map(|(index, (name, _, _))| (*name, self.states.len() + index))
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
                .map(|(_, name, shape, dtype)| {
                    let elements = shape.iter().product::<usize>().max(1);
                    let raw = device.alloc_elems(elements, *dtype);
                    (
                        *name,
                        device
                            .tensor_from_raw(raw, shape.clone(), *dtype)
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
            let pairs: Vec<(usize, &str)> = self
                .feedback
                .iter()
                .map(|(index, name, _, _)| (*index, *name))
                .collect();
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
