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

/// `SANIC_DEBUG` level, parsed once — the shape of tinygrad's `DEBUG`.
/// `1` dumps the compiled schedule (see [`partition`]); `2` additionally
/// times every kernel at runtime and prints one line per launch.
pub(crate) fn debug_level() -> u32 {
    static LEVEL: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    *LEVEL.get_or_init(|| {
        std::env::var("SANIC_DEBUG")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(0)
    })
}

/// A `width`-character bar filled to `fraction` of its length,
/// eighth-block resolution — the hotspot column of the `SANIC_DEBUG=2`
/// runtime dumps.
pub(crate) fn debug_bar(fraction: f64, width: usize) -> String {
    const PARTIAL: [&str; 7] = ["▏", "▎", "▍", "▌", "▋", "▊", "▉"];
    let eighths = (fraction.clamp(0.0, 1.0) * (width * 8) as f64).round() as usize;
    let mut bar = "█".repeat(eighths / 8);
    if eighths % 8 > 0 {
        bar.push_str(PARTIAL[eighths % 8 - 1]);
    }
    let filled = eighths.div_ceil(8);
    bar.push_str(&" ".repeat(width - filled));
    bar
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
#[doc(hidden)]
pub mod simplify;
#[doc(hidden)]
pub mod verify;

#[cfg(target_os = "macos")]
pub use compile::{MetalBuffer, MetalReplay};
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
