//! Derive streaming kernels from algebra, not templates.
//!
//! One fact does all the work: a computation can be streamed in one pass and
//! parallelized in a tree along an axis **iff it is an associative fold along
//! that axis**. Everything here is that fact applied by structural recursion:
//!
//! * [`ir`] — five compute node kinds plus two structural operators: `View`
//!   (rename / flatten) and `Reindex` (slice / pad / split / windows — affine
//!   index maps). Internal axes carry their extents and diagnostic labels;
//!   labels are never identity, so every shape is derivable from any graph
//!   and nothing takes a shape map.
//!   Matmul, softmax, attention, convolution, argmax, top-k and scatter-add
//!   are compositions
//! * [`tensor`] — the explicit graph-building frontend: symbolic
//!   [`TensorExpr`] values become a reusable [`Graph`] through
//!   [`GraphBuilder`], while [`Tensor`] is concrete bound data
//! * [`analyze`] — classify every (node, axis): FREE / MONOIDAL / OPAQUE /
//!   SEQUENTIAL, and build the structure map
//! * [`verify`] — check arity, axis scope, input declarations and movement
//!   bounds before a graph reaches analysis or code generation
//! * [`derive`] — turn each foldable axis into a concrete accumulator
//! * [`cost`] — device model, feasibility, roofline; ranks, never validates
//!   (legality is already settled)
//! * [`plan`] — pick the streamed axis, a tile that fits SRAM, and whether a
//!   fold should run as a two-stage split reduction
//! * [`partition`] — split a whole graph (or several roots at once) into
//!   kernels: the derive frontier is the fusion boundary, so cuts land
//!   exactly where the algebra stops
//! * [`grad`] — reverse-mode autodiff over the closed basis; backward graphs
//!   are ordinary IR and go through the same pipeline as any forward graph
//! * [`runtime`] — stateful sessions: persistent buffers, multi-output steps,
//!   commit-after-execute updates (the KV-cache decode loop, optimizer state)
//! * [`interp`] — a dense reference interpreter: the correctness oracle. It
//!   evaluates the naive graph *and* drives a derived carrier on real
//!   tensors, so `run_carrier == eval` certifies the generated kernel
//!   computes the real math, not just that it folds consistently
//! * [`codegen`] / [`rustgen`] / [`emit_metal`] — one shared node→code
//!   recursion behind a `Lang` trait; whole schedules lower to compilable
//!   Rust (verified by `rustc`-compile-and-run) and to Metal (dispatched on
//!   the Apple GPU by the tests)
//! * [`emit_rust`] — a single derived kernel as compilable Rust
//!
//! The headline: given naive `softmax(QKᵀ)·V` as a dataflow graph, `derive`
//! reconstructs the FlashAttention `(m, ℓ, o)` running accumulator from
//! composition rules alone. The property tests in `tests/laws.rs` hold each
//! associative carrier family to `tree_fold == fold == reference` on random
//! data — the associativity and correctness certificates in one assertion.
//! K-best's current singleton-insert carrier is reference-checked as a
//! sequential fold and explicitly excluded from tree/split execution until it
//! grows a true two-list merge.

pub mod analyze;
pub mod bpe;
pub mod codegen;
pub mod cost;
pub mod derive;
pub mod emit_metal;
pub mod emit_rust;
pub mod grad;
pub mod interp;
pub mod ir;
#[cfg(target_os = "macos")]
pub mod metal;
pub mod partition;
pub mod plan;
pub mod runtime;
pub mod rustgen;
pub mod safetensors;
pub mod simplify;
pub mod tensor;
pub mod verify;

pub use analyze::{
    AxisReport, Parallelism, Report, Structure, analyze, analyze_all, streamable, structure,
};
pub use derive::{Carrier, Expr, SlotKind, derive};
pub use interp::{Env, Value, eval, run_carrier};
pub use ir::*;
pub use tensor::{Bindings, Graph, GraphBuilder, Shape, Tensor, TensorExpr};
