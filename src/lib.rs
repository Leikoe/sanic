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
//! use sanic::{BinOp, Compile, CpuDevice, Dtype, Monoid, axis, input, reduce};
//!
//! let d = axis("d", 2);
//! let x = input("x", [d, d], Dtype::F32);
//! let rows = reduce(x, 1usize, BinOp::Monoid(Monoid::Add));
//!
//! let cpu = CpuDevice::new();
//! let program = rows.compile(&cpu)?;
//! let x = cpu.buffer([2, 2], Dtype::F32, [1.0, 2.0, 3.0, 4.0])?;
//! let outputs = program.run([("x", x)]);
//! assert_eq!(outputs[0].data(), &[3.0, 7.0]);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#[doc(hidden)]
pub mod analyze;
pub mod bpe;
#[doc(hidden)]
pub mod codegen;
pub mod compile;
pub mod cost;
#[doc(hidden)]
pub mod derive;
#[doc(hidden)]
pub mod emit_metal;
#[doc(hidden)]
pub mod emit_rust;
#[doc(hidden)]
pub mod grad;
#[doc(hidden)]
pub mod interp;
pub mod ir;
#[doc(hidden)]
pub mod kernel_ir;
#[cfg(target_os = "macos")]
pub mod metal;
pub mod nn;
#[doc(hidden)]
pub mod partition;
#[doc(hidden)]
pub mod plan;
#[doc(hidden)]
pub mod runtime;
#[doc(hidden)]
pub mod rustgen;
pub mod safetensors;
#[doc(hidden)]
pub mod simplify;
#[doc(hidden)]
pub mod verify;

#[cfg(target_os = "macos")]
pub use compile::MetalBuffer;
pub use compile::{
    Backend, Buffer, Compile, CompileError, CpuBuffer, CpuDevice, Program, RootItem, Roots,
    RunError,
};
pub use ir::*;
#[cfg(target_os = "macos")]
pub use metal::MetalDevice;

// Transitional compiler-engine exports used by the low-level law suite. New
// graph construction goes through `ir` and `Compile`.
#[doc(hidden)]
pub use analyze::{
    AxisReport, Parallelism, Report, Structure, analyze, analyze_all, streamable, structure,
};
#[doc(hidden)]
pub use derive::{Carrier, Expr, SlotKind, derive};
#[doc(hidden)]
pub use interp::{Env, Value, eval, run_carrier};
