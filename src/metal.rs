//! The Metal runtime — sanic's device layer for Apple GPUs.
//!
//! Shaped after tinygrad's runtime split (`ops_metal`: device / compiler /
//! program / allocator), sized to what a compiler-correctness project needs:
//!
//! * [`MetalDevice`] — the device and its command queue; opens `None` on a
//!   machine without a GPU so callers can skip cleanly.
//! * **Compiler** — [`MetalDevice::compile`] turns generated MSL into named
//!   pipelines; [`MetalDevice::compile_chunked`] splits multi-thousand-kernel
//!   programs into chunks first (Metal's front-end time grows superlinearly
//!   with source size — a 9k-kernel model compiles in seconds chunked,
//!   minutes whole).
//! * **Allocator** — [`MetalBuf`]s in shared (unified) memory, with typed
//!   upload paths: f32/f64 widening writes, raw bytes for packed int4 and
//!   f16 storage.
//! * **Program** — a [`Dispatch`] is one kernel launch (pipeline, buffers in
//!   `[[buffer(i)]]` order, thread count); [`MetalDevice::run`] encodes a
//!   dispatch list into one command buffer (an encoder per dispatch, so
//!   Metal's hazard tracking serializes on buffer dependencies) and waits.
//!
//! [`program_dispatches`] resolves a whole emitted [`MetalProgram`] against a
//! name→buffer map — rebuilt per step when a runtime swaps buffers (the
//! KV-cache commit), since dispatches bind by name at build time.

use std::collections::HashMap;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSString;
use objc2_metal::{
    MTLBuffer, MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLComputeCommandEncoder,
    MTLComputePipelineState, MTLCreateSystemDefaultDevice, MTLDevice, MTLLibrary,
    MTLResourceOptions, MTLSize,
};

use crate::emit_metal::MetalProgram;

/// A pipeline for one compiled kernel.
pub type Pipeline = Retained<ProtocolObject<dyn MTLComputePipelineState>>;

/// A device buffer in shared (unified) memory. Cloning retains the same
/// underlying allocation — a name→buffer map can swap entries in O(1), which
/// is exactly how a session commits a KV-cache update on device.
#[derive(Clone)]
pub struct MetalBuf(Retained<ProtocolObject<dyn MTLBuffer>>);

impl MetalBuf {
    pub fn byte_len(&self) -> usize {
        self.0.length()
    }
}

/// Compiled kernels, indexed by entry-point name.
pub struct Pipelines {
    map: HashMap<String, Pipeline>,
}

impl Pipelines {
    pub fn get(&self, name: &str) -> Pipeline {
        self.map
            .get(name)
            .unwrap_or_else(|| panic!("no compiled kernel named `{name}`"))
            .clone()
    }
}

/// One kernel launch: the pipeline, its input buffers in `[[buffer(0..)]]`
/// order, the output buffer, and the flat thread-grid size.
pub struct Dispatch {
    pub pipe: Pipeline,
    pub inputs: Vec<MetalBuf>,
    pub output: MetalBuf,
    pub grid: usize,
}

pub struct MetalDevice {
    dev: Retained<ProtocolObject<dyn MTLDevice>>,
    queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
}

impl MetalDevice {
    /// `None` when the machine has no Metal device (CI without a GPU).
    pub fn open() -> Option<MetalDevice> {
        let dev = MTLCreateSystemDefaultDevice()?;
        let queue = dev.newCommandQueue().expect("command queue");
        Some(MetalDevice { dev, queue })
    }

    // ── compiler ─────────────────────────────────────────────────────────────

    /// Compile one MSL source; index every kernel it defines.
    pub fn compile(&self, msl: &str) -> Pipelines {
        let lib = self
            .dev
            .newLibraryWithSource_options_error(&NSString::from_str(msl), None)
            .unwrap_or_else(|e| panic!("generated MSL failed to compile: {e}"));
        let mut map = HashMap::new();
        for name in kernel_names(msl) {
            let f = lib
                .newFunctionWithName(&NSString::from_str(&name))
                .unwrap_or_else(|| panic!("kernel `{name}` missing after compile"));
            let p = self
                .dev
                .newComputePipelineStateWithFunction_error(&f)
                .unwrap_or_else(|e| panic!("pipeline `{name}`: {e}"));
            map.insert(name, p);
        }
        Pipelines { map }
    }

    /// Compile a large program in chunks of `chunk` kernels, each prefixed
    /// with `header` (the shared prelude every kernel needs). Progress goes
    /// to stderr when `progress` is set.
    pub fn compile_chunked(
        &self,
        msl: &str,
        header: &str,
        chunk: usize,
        progress: bool,
    ) -> Pipelines {
        let body = msl.strip_prefix(header).unwrap_or(msl);
        let kernels: Vec<&str> = body.split("kernel void ").skip(1).collect();
        let mut map = HashMap::new();
        let t0 = std::time::Instant::now();
        for (ci, group) in kernels.chunks(chunk).enumerate() {
            let mut src = String::from(header);
            for k in group {
                src.push_str("kernel void ");
                src.push_str(k);
            }
            let lib = self
                .dev
                .newLibraryWithSource_options_error(&NSString::from_str(&src), None)
                .unwrap_or_else(|e| panic!("MSL chunk {ci} failed to compile: {e}"));
            for k in group {
                let name = k.split('(').next().unwrap().trim().to_string();
                let f = lib
                    .newFunctionWithName(&NSString::from_str(&name))
                    .unwrap_or_else(|| panic!("kernel `{name}` missing"));
                let p = self
                    .dev
                    .newComputePipelineStateWithFunction_error(&f)
                    .unwrap_or_else(|e| panic!("pipeline `{name}`: {e}"));
                map.insert(name, p);
            }
            if progress && ci % 8 == 0 {
                eprint!(
                    "\r  compiling MSL: {}/{} kernels ({:.0}s)",
                    (ci * chunk + group.len()).min(kernels.len()),
                    kernels.len(),
                    t0.elapsed().as_secs_f32()
                );
            }
        }
        if progress {
            eprintln!(
                "\r  compiled {} kernels in {:.1}s        ",
                kernels.len(),
                t0.elapsed().as_secs_f32()
            );
        }
        Pipelines { map }
    }

    // ── allocator ────────────────────────────────────────────────────────────

    /// A zeroed buffer of `count` f32 elements.
    pub fn alloc_f32(&self, count: usize) -> MetalBuf {
        self.alloc_bytes(count.max(1) * 4)
    }

    /// A zeroed buffer of raw bytes.
    pub fn alloc_bytes(&self, bytes: usize) -> MetalBuf {
        let buf = self
            .dev
            .newBufferWithLength_options(bytes.max(4), MTLResourceOptions::StorageModeShared)
            .expect("buffer allocation");
        unsafe { std::ptr::write_bytes(buf.contents().as_ptr() as *mut u8, 0, bytes.max(4)) };
        MetalBuf(buf)
    }

    pub fn from_f32(&self, data: &[f32]) -> MetalBuf {
        let buf = self.alloc_bytes(data.len() * 4);
        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                buf.0.contents().as_ptr() as *mut f32,
                data.len(),
            )
        };
        buf
    }

    /// Host f64 → device f32 (the compute width of this backend).
    pub fn from_f64(&self, data: &[f64]) -> MetalBuf {
        let buf = self.alloc_f32(data.len());
        self.write_f64(&buf, data);
        buf
    }

    /// Raw bytes — packed int4 nibbles, f16 halves: typed storage uploads.
    pub fn from_bytes(&self, data: &[u8]) -> MetalBuf {
        let buf = self.alloc_bytes(data.len());
        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                buf.0.contents().as_ptr() as *mut u8,
                data.len(),
            )
        };
        buf
    }

    /// Overwrite a buffer's leading elements (f64 host values → f32 device).
    pub fn write_f64(&self, buf: &MetalBuf, data: &[f64]) {
        let ptr = buf.0.contents().as_ptr() as *mut f32;
        for (i, &v) in data.iter().enumerate() {
            unsafe { *ptr.add(i) = v as f32 };
        }
    }

    pub fn read_f32(&self, buf: &MetalBuf, count: usize) -> Vec<f32> {
        let ptr = buf.0.contents().as_ptr() as *const f32;
        (0..count).map(|i| unsafe { *ptr.add(i) }).collect()
    }

    // ── execution ────────────────────────────────────────────────────────────

    /// Encode every dispatch into ONE command buffer — an encoder per
    /// dispatch, so Metal serializes on buffer hazards — run it, and wait.
    pub fn run(&self, dispatches: &[Dispatch]) {
        let cb = self.queue.commandBuffer().expect("command buffer");
        for d in dispatches {
            let enc = cb.computeCommandEncoder().expect("compute encoder");
            enc.setComputePipelineState(&d.pipe);
            for (i, b) in d.inputs.iter().enumerate() {
                unsafe { enc.setBuffer_offset_atIndex(Some(&b.0), 0, i) };
            }
            unsafe { enc.setBuffer_offset_atIndex(Some(&d.output.0), 0, d.inputs.len()) };
            let tg = d.pipe.maxTotalThreadsPerThreadgroup().min(d.grid);
            enc.dispatchThreads_threadsPerThreadgroup(
                MTLSize {
                    width: d.grid,
                    height: 1,
                    depth: 1,
                },
                MTLSize {
                    width: tg,
                    height: 1,
                    depth: 1,
                },
            );
            enc.endEncoding();
        }
        cb.commit();
        cb.waitUntilCompleted();
    }
}

/// Entry-point names defined in an MSL source.
fn kernel_names(msl: &str) -> Vec<String> {
    msl.split("kernel void ")
        .skip(1)
        .map(|k| k.split('(').next().unwrap().trim().to_string())
        .collect()
}

/// Resolve a whole emitted schedule against a name→buffer map. Rebuild after
/// swapping entries (a cache commit): dispatches capture buffers, not names.
pub fn program_dispatches(
    program: &MetalProgram,
    bufs: &HashMap<String, MetalBuf>,
    pipes: &Pipelines,
) -> Vec<Dispatch> {
    program
        .stages
        .iter()
        .map(|st| Dispatch {
            pipe: pipes.get(&st.kernel),
            inputs: st.inputs.iter().map(|n| bufs[n].clone()).collect(),
            output: bufs[&st.output].clone(),
            grid: st.grid_size,
        })
        .collect()
}
