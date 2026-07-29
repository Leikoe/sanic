//! Direct, positional tensor graphs compiled into algebraically derived
//! streaming kernels.
//!
//! Construct immutable [`Node`](ir::Node) values directly, compile one root or
//! an ordered tuple/vector of roots with [`Compile::compile`], then bind
//! backend buffers by input name with [`Program::run`]. Dimension identity is
//! local to each node's ordered shape: an operator such as `reduce(x, 1, op)`
//! always addresses shape index `1`.
//!
//! ```
//! use sanic::{Compile, CpuDevice, Dtype, Monoid, axis, input, reduce};
//!
//! let d = axis("d", 2);
//! let x = input("x", [d, d], Dtype::F32);
//! let rows = reduce(x, 1usize, Monoid::Add);
//!
//! let cpu = CpuDevice::new();
//! let program = rows.compile(&cpu)?;
//! let x = cpu.buffer([2, 2], Dtype::F32, [1.0, 2.0, 3.0, 4.0])?;
//! let outputs = program.run([("x", x)]);
//! assert_eq!(outputs[0].data(), &[3.0, 7.0]);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

/// `SANIC_DEBUG` level, parsed once — the shape of tinygrad's `DEBUG`.
/// `1` dumps the compiled schedule (see [`partition`]); `2` prints one line
/// per replayed graph with its REAL command-buffer time; `3` additionally
/// dumps each frozen graph's contents once (plan-side) and traces
/// compile-time cut decisions; `4` times every kernel INSIDE the step's one
/// command buffer via encoder-boundary GPU timestamps (solo re-timing,
/// sync-floored in sum, where the device can't sample counters).
///
/// The other switches, all read where they act: `SANIC_TUNE=1` measures
/// fold schedules on the real device instead of trusting the cost model
/// (`tune_fold_scheds`), `SANIC_MSL=<path>` writes the generated Metal
/// source, and `SANIC_NANSCAN=1` names the first stage with a non-finite
/// output.
pub(crate) fn debug_level() -> u32 {
    static LEVEL: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    *LEVEL.get_or_init(|| {
        std::env::var("SANIC_DEBUG")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(0)
    })
}

#[doc(hidden)]
pub mod analyze;
#[doc(hidden)]
pub mod codegen;
pub mod compile;
pub mod cost;
#[doc(hidden)]
pub mod derive;
#[doc(hidden)]
pub mod emit_metal;
#[doc(hidden)]
pub mod grad;
#[doc(hidden)]
pub mod graph;
#[doc(hidden)]
pub mod interp;
pub mod ir;
#[doc(hidden)]
#[cfg(target_os = "macos")]
pub mod metal;
pub mod nn;
pub mod numeric;
#[doc(hidden)]
pub mod partition;
#[doc(hidden)]
pub mod plan;
#[doc(hidden)]
pub mod runtime;
#[doc(hidden)]
pub mod rustgen;
mod scalar;
pub mod simplify;
pub mod tensor;
#[doc(hidden)]
pub mod verify;

pub use compile::{Backend, Buffer, Compile, CompileError, CpuBuffer, CpuDevice, Program, RootItem, Roots, RunError};
#[cfg(target_os = "macos")]
pub use compile::{MetalBuffer, MetalReplay};
pub use graph::Graph;
pub use ir::*;
#[cfg(target_os = "macos")]
pub use metal::MetalDevice;
pub use tensor::Tensor;
