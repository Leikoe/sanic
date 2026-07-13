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
//!
//! * **Graphs** — [`MetalDevice::capture`] freezes a dispatch list into an
//!   `MTLIndirectCommandBuffer`; [`MetalDevice::run_graph`] replays it with
//!   one encoder and one execute call per step. Swap commits flip bindings
//!   with period two, so decode loops keep one graph per step parity.

use std::collections::{HashMap, HashSet};

use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::{NSRange, NSString};
use objc2_metal::{
    MTLBuffer, MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLComputeCommandEncoder,
    MTLComputePipelineDescriptor, MTLComputePipelineState, MTLCreateSystemDefaultDevice,
    MTLDevice, MTLFunction, MTLIndirectCommandBuffer, MTLIndirectCommandBufferDescriptor,
    MTLIndirectCommandType, MTLIndirectComputeCommand, MTLLibrary, MTLPipelineOption,
    MTLResource, MTLResourceOptions, MTLResourceUsage, MTLSize,
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
#[derive(Clone)]
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
            map.insert(name.clone(), self.pipeline(&f, &name));
        }
        Pipelines { map }
    }

    /// Every pipeline opts into indirect command buffers, so any dispatch
    /// list can be captured into a [`MetalGraph`]; free for direct dispatch.
    fn pipeline(&self, f: &ProtocolObject<dyn MTLFunction>, name: &str) -> Pipeline {
        let desc = MTLComputePipelineDescriptor::new();
        desc.setComputeFunction(Some(f));
        desc.setSupportIndirectCommandBuffers(true);
        self.dev
            .newComputePipelineStateWithDescriptor_options_reflection_error(
                &desc,
                MTLPipelineOption::empty(),
                None,
            )
            .unwrap_or_else(|e| panic!("pipeline `{name}`: {e}"))
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
        // A kernel-level attribute (`[[max_total_threads_per_threadgroup(n)]]`
        // on its own line) precedes `kernel void`, so the split leaves it at
        // the TAIL of the previous kernel's segment — reattach it to the
        // kernel it belongs to or a chunk boundary would orphan it.
        let raw: Vec<&str> = body.split("kernel void ").skip(1).collect();
        let mut kernels: Vec<(String, &str)> = Vec::new(); // (attr prefix, body)
        let mut pending = String::new();
        for k in raw {
            let attr = std::mem::take(&mut pending);
            let (piece, tail) = match k.rfind("\n[[") {
                Some(p) if k[p + 1..].trim_end().ends_with("]]") => {
                    (&k[..p + 1], k[p + 1..].trim_end().to_string())
                }
                _ => (k, String::new()),
            };
            pending = tail;
            kernels.push((attr, piece));
        }
        let mut map = HashMap::new();
        let t0 = std::time::Instant::now();
        for (ci, group) in kernels.chunks(chunk).enumerate() {
            let mut src = String::from(header);
            for (attr, k) in group {
                if !attr.is_empty() {
                    src.push_str(attr);
                    src.push('\n');
                }
                src.push_str("kernel void ");
                src.push_str(k);
            }
            let lib = self
                .dev
                .newLibraryWithSource_options_error(&NSString::from_str(&src), None)
                .unwrap_or_else(|e| panic!("MSL chunk {ci} failed to compile: {e}"));
            for (_, k) in group {
                let name = k.split('(').next().unwrap().trim().to_string();
                let f = lib
                    .newFunctionWithName(&NSString::from_str(&name))
                    .unwrap_or_else(|| panic!("kernel `{name}` missing"));
                map.insert(name.clone(), self.pipeline(&f, &name));
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
    /// dispatch, so Metal serializes on buffer hazards while still
    /// overlapping independent stages — run it, and wait.
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

    /// [`Self::run`], returning the command buffer's GPU residency in
    /// seconds (`GPUEndTime − GPUStartTime`) — kernel time plus any
    /// inter-dispatch bubbles, free of CPU encode/submit cost. This is the
    /// number per-dispatch wall clocks can't give: no sync floor.
    pub fn run_timed(&self, dispatches: &[Dispatch]) -> f64 {
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
        // CFTimeInterval (f64 seconds); raw messages sidestep feature gates
        let t0: f64 = unsafe { msg_send![&*cb, GPUStartTime] };
        let t1: f64 = unsafe { msg_send![&*cb, GPUEndTime] };
        t1 - t0
    }
}

/// A captured dispatch sequence in an `MTLIndirectCommandBuffer`: encode
/// once, replay per step with ONE encoder and one `executeCommandsInBuffer`
/// — the graph execution tinygrad and MLX use, without re-encoding a
/// thousand encoders per token on the CPU.
///
/// Buffer BINDINGS are frozen at capture. A session's swap commits flip
/// bindings with period two, so a decode loop keeps one graph per step
/// parity and replays the matching one.
///
/// Hazards: commands in an ICB run concurrently; a barrier is set on each
/// command that touches a buffer written since the last barrier, so
/// independent stages still overlap while dependent ones order correctly.
pub struct MetalGraph {
    icb: Retained<ProtocolObject<dyn MTLIndirectCommandBuffer>>,
    /// Every distinct buffer the commands touch — declared resident at
    /// replay (`useResource`) and retained so the ICB never dangles.
    resources: Vec<MetalBuf>,
    len: usize,
}

impl MetalDevice {
    /// Freeze a dispatch list into a replayable graph.
    pub fn capture(&self, dispatches: &[Dispatch]) -> MetalGraph {
        let desc = MTLIndirectCommandBufferDescriptor::new();
        desc.setCommandTypes(MTLIndirectCommandType::ConcurrentDispatchThreads);
        desc.setInheritPipelineState(false);
        desc.setInheritBuffers(false);
        let max_bufs = dispatches
            .iter()
            .map(|d| d.inputs.len() + 1)
            .max()
            .unwrap_or(1);
        desc.setMaxKernelBufferBindCount(max_bufs);
        let icb = unsafe {
            self.dev
                .newIndirectCommandBufferWithDescriptor_maxCommandCount_options(
                    &desc,
                    dispatches.len(),
                    MTLResourceOptions::StorageModeShared,
                )
        }
        .expect("indirect command buffer");

        let mut resources: Vec<MetalBuf> = Vec::new();
        let mut seen: HashSet<usize> = HashSet::new();
        // buffers written since the last barrier: touching one forces a
        // barrier on the toucher (which fences everything before it)
        let mut written: HashSet<usize> = HashSet::new();
        for (i, d) in dispatches.iter().enumerate() {
            let cmd = unsafe { icb.indirectComputeCommandAtIndex(i) };
            cmd.setComputePipelineState(&d.pipe);
            for (bi, b) in d.inputs.iter().enumerate() {
                unsafe { cmd.setKernelBuffer_offset_atIndex(&b.0, 0, bi) };
                if seen.insert(Retained::as_ptr(&b.0) as usize) {
                    resources.push(b.clone());
                }
            }
            unsafe { cmd.setKernelBuffer_offset_atIndex(&d.output.0, 0, d.inputs.len()) };
            if seen.insert(Retained::as_ptr(&d.output.0) as usize) {
                resources.push(d.output.clone());
            }
            let hazard = d
                .inputs
                .iter()
                .chain(std::iter::once(&d.output))
                .any(|b| written.contains(&(Retained::as_ptr(&b.0) as usize)));
            if hazard {
                cmd.setBarrier();
                written.clear();
            }
            written.insert(Retained::as_ptr(&d.output.0) as usize);
            let tg = d.pipe.maxTotalThreadsPerThreadgroup().min(d.grid);
            cmd.concurrentDispatchThreads_threadsPerThreadgroup(
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
        }
        MetalGraph {
            icb,
            resources,
            len: dispatches.len(),
        }
    }

    /// Replay a captured graph and wait: one command buffer, one encoder,
    /// one execute call.
    pub fn run_graph(&self, g: &MetalGraph) {
        let cb = self.queue.commandBuffer().expect("command buffer");
        let enc = cb.computeCommandEncoder().expect("compute encoder");
        for b in &g.resources {
            let res: &ProtocolObject<dyn MTLResource> = ProtocolObject::from_ref(&*b.0);
            enc.useResource_usage(res, MTLResourceUsage::Read | MTLResourceUsage::Write);
        }
        // objc2-metal 0.3 has no binding for the compute encoder's
        // `executeCommandsInBuffer:withRange:` (macOS 11+); raw message.
        let range = NSRange {
            location: 0,
            length: g.len,
        };
        let _: () = unsafe { msg_send![&*enc, executeCommandsInBuffer: &*g.icb, withRange: range] };
        enc.endEncoding();
        cb.commit();
        cb.waitUntilCompleted();
    }

    /// [`Self::run_graph`], returning GPU residency in seconds (see
    /// [`Self::run_timed`]).
    pub fn run_graph_timed(&self, g: &MetalGraph) -> f64 {
        let cb = self.queue.commandBuffer().expect("command buffer");
        let enc = cb.computeCommandEncoder().expect("compute encoder");
        for b in &g.resources {
            let res: &ProtocolObject<dyn MTLResource> = ProtocolObject::from_ref(&*b.0);
            enc.useResource_usage(res, MTLResourceUsage::Read | MTLResourceUsage::Write);
        }
        let range = NSRange {
            location: 0,
            length: g.len,
        };
        let _: () = unsafe { msg_send![&*enc, executeCommandsInBuffer: &*g.icb, withRange: range] };
        enc.endEncoding();
        cb.commit();
        cb.waitUntilCompleted();
        if let Some(err) = cb.error() {
            eprintln!("graph replay FAILED: {err}");
        }
        let t0: f64 = unsafe { msg_send![&*cb, GPUStartTime] };
        let t1: f64 = unsafe { msg_send![&*cb, GPUEndTime] };
        t1 - t0
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
