//! The Metal runtime — sanic's device layer for Apple GPUs.
//!
//! Shaped after tinygrad's runtime split (`ops_metal`: device / compiler /
//! program / allocator), sized to what a compiler-correctness project needs:
//!
//! * [`MetalDevice`] — the device and its command queue; opens `None` on a
//!   machine without a GPU so callers can skip cleanly.
//! * **Compiler** — [`MetalDevice::compile`] turns generated MSL into named
//!   pipelines.
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
//! * **Graphs** — [`MetalDevice::capture`] freezes a dispatch list and its
//!   bindings. Small lists replay through an `MTLIndirectCommandBuffer`; large
//!   lists use ordered encoders in one command buffer because large concurrent
//!   ICBs are unstable on Apple7. Swap commits keep one frozen graph per parity.

use std::collections::{HashMap, HashSet};

use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::{NSRange, NSString};
use objc2_metal::{
    MTLBuffer, MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLComputeCommandEncoder,
    MTLComputePipelineDescriptor, MTLComputePipelineState, MTLCreateSystemDefaultDevice, MTLDevice,
    MTLFunction, MTLIndirectCommandBuffer, MTLIndirectCommandBufferDescriptor,
    MTLIndirectCommandType, MTLIndirectComputeCommand, MTLLibrary, MTLPipelineOption,
    MTLResidencySet, MTLResidencySetDescriptor, MTLResource, MTLResourceOptions, MTLResourceUsage,
    MTLSize,
};

use crate::emit_metal::MetalProgram;

/// A pipeline for one compiled kernel.
pub type Pipeline = Retained<ProtocolObject<dyn MTLComputePipelineState>>;

/// A device buffer in shared (unified) memory, with a byte OFFSET into the
/// underlying allocation: several logical buffers can alias one `MTLBuffer`
/// (a zero-copy weight file wrapped whole, tensors bound at their file
/// offsets). Cloning retains the same underlying allocation — a name→buffer
/// map can swap entries in O(1), which is exactly how a session commits a
/// KV-cache update on device.
#[derive(Clone)]
pub struct MetalBuf(Retained<ProtocolObject<dyn MTLBuffer>>, usize);

impl MetalBuf {
    pub fn byte_len(&self) -> usize {
        self.0.length() - self.1
    }
    /// This buffer, re-based `off` bytes further in. Apple GPUs require
    /// device-buffer bind offsets in multiples of 4; misaligned tensors must
    /// copy instead.
    pub fn slice(&self, off: usize) -> MetalBuf {
        assert!(
            (self.1 + off) % 4 == 0,
            "buffer bind offsets must be 4-byte aligned (got {})",
            self.1 + off
        );
        MetalBuf(self.0.clone(), self.1 + off)
    }
    fn contents(&self) -> *mut u8 {
        unsafe { (self.0.contents().as_ptr() as *mut u8).add(self.1) }
    }
    /// This handle's GPU virtual address (Metal 3+, Apple Silicon), including
    /// its byte offset. On Tier 2 hardware an argument buffer stores exactly
    /// this per `device T*` member, so a bindless kernel binds one address
    /// table in place of dozens of direct buffers (past Metal's 31-bind cap).
    pub fn gpu_address(&self) -> u64 {
        self.0.gpuAddress() + self.1 as u64
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
///
/// A *bindless* dispatch (`argbuf: Some`) instead binds one argument buffer —
/// a table of the inputs' GPU addresses — at `[[buffer(0)]]` and the output at
/// `[[buffer(1)]]`; `inputs` then names the resources to make resident
/// (`useResource`) rather than buffers to bind directly. This is how a kernel
/// reading more than ~30 buffers (a wide gradient-accumulation cone) fits
/// under Metal's 31-argument cap.
#[derive(Clone)]
pub struct Dispatch {
    pub pipe: Pipeline,
    pub inputs: Vec<MetalBuf>,
    pub output: MetalBuf,
    pub grid: usize,
    pub argbuf: Option<MetalBuf>,
}

#[derive(Clone)]
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
        MetalBuf(buf, 0)
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
        unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), buf.contents(), data.len()) };
        buf
    }

    /// ZERO-COPY: wrap caller-owned memory as a device buffer — no upload,
    /// no second residency; on unified memory the pointer IS the device
    /// address. The region must be page-aligned with a page-multiple length
    /// (`None` otherwise — copy instead), and must outlive every use: pass
    /// leaked / 'static memory. Bind individual tensors inside it with
    /// [`MetalBuf::slice`].
    pub fn from_bytes_nocopy(&self, data: &'static [u8]) -> Option<MetalBuf> {
        let page = 16384usize; // Apple silicon page size
        if data.as_ptr() as usize % page != 0 || data.len() % page != 0 || data.is_empty() {
            return None;
        }
        let ptr = std::ptr::NonNull::new(data.as_ptr() as *mut std::ffi::c_void)?;
        let buf = unsafe {
            self.dev
                .newBufferWithBytesNoCopy_length_options_deallocator(
                    ptr,
                    data.len(),
                    MTLResourceOptions::StorageModeShared,
                    None, // caller-owned ('static): no deallocator
                )
        }?;
        Some(MetalBuf(buf, 0))
    }

    /// Overwrite a buffer's leading elements (f64 host values → f32 device).
    pub fn write_f64(&self, buf: &MetalBuf, data: &[f64]) {
        let ptr = buf.contents() as *mut f32;
        for (i, &v) in data.iter().enumerate() {
            unsafe { *ptr.add(i) = v as f32 };
        }
    }

    pub fn read_f32(&self, buf: &MetalBuf, count: usize) -> Vec<f32> {
        let ptr = buf.contents() as *const f32;
        (0..count).map(|i| unsafe { *ptr.add(i) }).collect()
    }

    // ── execution ────────────────────────────────────────────────────────────

    /// Encode every dispatch into ONE command buffer — an encoder per
    /// dispatch, so Metal serializes on buffer hazards while still
    /// overlapping independent stages — run it, and wait.
    pub fn run(&self, dispatches: &[Dispatch]) {
        let cb = self.queue.commandBuffer().expect("command buffer");
        for d in dispatches {
            encode(&cb, d);
        }
        cb.commit();
        cb.waitUntilCompleted();
    }

    /// One command buffer PER dispatch: each kernel's own GPU residency, in
    /// order. This is the `SANIC_DEBUG=2` path (tinygrad's `DEBUG=2` also
    /// synchronizes per kernel to time it) — the per-dispatch submit adds a
    /// sync floor, so the SUM is not a decode-speed number;
    /// [`Self::run_graph_timed`] is. Individual kernel times are accurate:
    /// GPU start→end, no CPU cost.
    pub fn run_each_timed(&self, dispatches: &[Dispatch]) -> Vec<f64> {
        dispatches
            .iter()
            .map(|d| {
                let cb = self.queue.commandBuffer().expect("command buffer");
                encode(&cb, d);
                cb.commit();
                cb.waitUntilCompleted();
                gpu_seconds(&cb)
            })
            .collect()
    }
}

/// Encode one kernel launch onto a command buffer — the one encode path
/// behind [`MetalDevice::run`] and [`MetalDevice::run_each_timed`].
fn encode(cb: &ProtocolObject<dyn MTLCommandBuffer>, d: &Dispatch) {
    let enc = cb.computeCommandEncoder().expect("compute encoder");
    enc.setComputePipelineState(&d.pipe);
    if let Some(ab) = &d.argbuf {
        // bindless: one address table at 0, output at 1, inputs resident
        unsafe { enc.setBuffer_offset_atIndex(Some(&ab.0), ab.1, 0) };
        unsafe { enc.setBuffer_offset_atIndex(Some(&d.output.0), d.output.1, 1) };
        for b in &d.inputs {
            let res: &ProtocolObject<dyn MTLResource> = ProtocolObject::from_ref(&*b.0);
            enc.useResource_usage(res, MTLResourceUsage::Read);
        }
    } else {
        for (i, b) in d.inputs.iter().enumerate() {
            unsafe { enc.setBuffer_offset_atIndex(Some(&b.0), b.1, i) };
        }
        unsafe { enc.setBuffer_offset_atIndex(Some(&d.output.0), d.output.1, d.inputs.len()) };
    }
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

/// A committed command buffer's GPU residency in seconds.
fn gpu_seconds(cb: &ProtocolObject<dyn MTLCommandBuffer>) -> f64 {
    // CFTimeInterval (f64 seconds); raw messages sidestep feature gates
    let t0: f64 = unsafe { msg_send![cb, GPUStartTime] };
    let t1: f64 = unsafe { msg_send![cb, GPUEndTime] };
    t1 - t0
}

/// A dispatch sequence with frozen bindings. Small sequences use an indirect
/// command buffer; large sequences retain their dispatches and replay them as
/// ordered encoders in one command buffer.
///
/// Buffer BINDINGS are frozen at capture. A session's swap commits flip
/// bindings with period two, so a decode loop keeps one graph per step
/// parity and replays the matching one.
///
/// Hazards: commands in an ICB run concurrently; a barrier is set on each
/// command that conflicts with an earlier command. A barrier belongs to that
/// command — it is not a phase boundary inherited by later commands — so the
/// complete access history stays live for the whole ICB.
const MAX_INDIRECT_COMMANDS: usize = 64;

pub struct MetalGraph {
    execution: MetalGraphExecution,
}

enum MetalGraphExecution {
    /// Large concurrent ICBs have proven unstable on Apple7. Frozen direct
    /// dispatches keep the same bindings and use ordered compute encoders in
    /// one command buffer, letting Metal's normal hazard tracking apply.
    Direct(Vec<Dispatch>),
    Indirect {
        command_buffer: MetalIndirectCommandBuffer,
        /// Explicit residency for the command buffer's resources and pipelines.
        residency: Retained<ProtocolObject<dyn MTLResidencySet>>,
    },
}

struct MetalIndirectCommandBuffer {
    icb: Retained<ProtocolObject<dyn MTLIndirectCommandBuffer>>,
    /// Every resource referenced by an indirect command, with read/write
    /// usage combined per underlying allocation. The executing encoder must
    /// declare these; a residency set alone does not track their hazards.
    resources: Vec<(MetalBuf, MTLResourceUsage)>,
    len: usize,
}

impl MetalDevice {
    /// Freeze a dispatch list into a replayable graph.
    ///
    /// On the indirect path, panics if a binding offset exceeds `u32::MAX`:
    /// ICB offsets are u32 on the wire, so accepting one would silently
    /// corrupt the binding. Large schedules already take the direct path.
    pub fn capture(&self, dispatches: &[Dispatch]) -> MetalGraph {
        if dispatches.len() > MAX_INDIRECT_COMMANDS {
            return MetalGraph {
                execution: MetalGraphExecution::Direct(dispatches.to_vec()),
            };
        }
        for d in dispatches {
            for b in d
                .inputs
                .iter()
                .chain(std::iter::once(&d.output))
                .chain(d.argbuf.as_ref())
            {
                assert!(
                    b.1 <= u32::MAX as usize,
                    "buffer offset {} exceeds u32 — indirect command buffers cannot bind it",
                    b.1
                );
            }
        }
        let command_buffer = self.capture_indirect(dispatches);

        // An ICB does not make its referenced allocations resident at replay,
        // and on Apple7/8 (M1/M2) that includes pipeline states. One set covers
        // all chunks and strong-references every allocation for the graph's
        // lifetime.
        let residency = self
            .dev
            .newResidencySetWithDescriptor_error(&MTLResidencySetDescriptor::new())
            .expect("residency set");
        for (buffer, _) in icb_resource_usages(dispatches) {
            residency.addAllocation(ProtocolObject::from_ref(&*buffer.0));
        }
        let mut seen_pipelines = HashSet::new();
        for dispatch in dispatches {
            if seen_pipelines.insert(Retained::as_ptr(&dispatch.pipe) as usize) {
                residency.addAllocation(ProtocolObject::from_ref(&*dispatch.pipe));
            }
        }
        residency.commit();
        residency.requestResidency();

        MetalGraph {
            execution: MetalGraphExecution::Indirect {
                command_buffer,
                residency,
            },
        }
    }

    fn capture_indirect(&self, dispatches: &[Dispatch]) -> MetalIndirectCommandBuffer {
        let desc = MTLIndirectCommandBufferDescriptor::new();
        desc.setCommandTypes(MTLIndirectCommandType::ConcurrentDispatchThreads);
        desc.setInheritPipelineState(false);
        desc.setInheritBuffers(false);
        let max_bufs = dispatches
            .iter()
            .map(|d| {
                if d.argbuf.is_some() {
                    2
                } else {
                    d.inputs.len() + 1
                }
            })
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

        // Every allocation touched/written by an earlier command. ICB
        // barriers belong to one command, not to all commands after it, so
        // neither history can be cleared after placing a barrier.
        let mut accessed: HashSet<usize> = HashSet::new();
        let mut written: HashSet<usize> = HashSet::new();
        for (i, d) in dispatches.iter().enumerate() {
            let cmd = unsafe { icb.indirectComputeCommandAtIndex(i) };
            cmd.setComputePipelineState(&d.pipe);
            // a bindless dispatch binds its address table at 0 and the output
            // at 1; the inputs are reached through the table, so they need
            // residency, not binding slots (mirrors `encode`)
            if let Some(ab) = &d.argbuf {
                unsafe { cmd.setKernelBuffer_offset_atIndex(&ab.0, ab.1, 0) };
                unsafe { cmd.setKernelBuffer_offset_atIndex(&d.output.0, d.output.1, 1) };
            } else {
                for (bi, b) in d.inputs.iter().enumerate() {
                    unsafe { cmd.setKernelBuffer_offset_atIndex(&b.0, b.1, bi) };
                }
                unsafe {
                    cmd.setKernelBuffer_offset_atIndex(&d.output.0, d.output.1, d.inputs.len())
                };
            }
            let input_allocations = d
                .inputs
                .iter()
                .map(|buffer| Retained::as_ptr(&buffer.0) as usize)
                .collect::<Vec<_>>();
            let output_allocation = Retained::as_ptr(&d.output.0) as usize;
            let hazard = icb_command_needs_barrier(
                &input_allocations,
                output_allocation,
                &mut accessed,
                &mut written,
            );
            if hazard {
                cmd.setBarrier();
            }
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
        let resources = icb_resource_usages(dispatches);

        MetalIndirectCommandBuffer {
            icb,
            resources,
            len: dispatches.len(),
        }
    }

    /// Replay a frozen graph and wait, returning GPU residency in seconds
    /// (`GPUEndTime − GPUStartTime`: kernel time plus inter-dispatch
    /// bubbles, free of CPU encode/submit cost).
    pub fn run_graph_timed(&self, g: &MetalGraph) -> Result<f64, String> {
        self.replay_checked(g).map(|cb| gpu_seconds(&cb))
    }

    /// One replay, error-checked. Any command buffer error — a GPU fault of
    /// our own, or "Discarded (victim of GPU error/recovery)" when something
    /// ELSE faults the GPU mid-flight — is an `Err`: the step's writes are
    /// untrustworthy and a decode loop must not continue on them.
    fn replay_checked(
        &self,
        g: &MetalGraph,
    ) -> Result<Retained<ProtocolObject<dyn MTLCommandBuffer>>, String> {
        let cb = self.queue.commandBuffer().expect("command buffer");
        match &g.execution {
            MetalGraphExecution::Direct(dispatches) => {
                for dispatch in dispatches {
                    encode(&cb, dispatch);
                }
            }
            MetalGraphExecution::Indirect {
                command_buffer,
                residency,
            } => {
                cb.useResidencySet(residency);
                let enc = cb.computeCommandEncoder().expect("compute encoder");
                for (buffer, usage) in &command_buffer.resources {
                    let resource: &ProtocolObject<dyn MTLResource> =
                        ProtocolObject::from_ref(&*buffer.0);
                    enc.useResource_usage(resource, *usage);
                }
                // objc2-metal 0.3 has no binding for the compute encoder's
                // `executeCommandsInBuffer:withRange:` (macOS 11+); raw message.
                let range = NSRange {
                    location: 0,
                    length: command_buffer.len,
                };
                let _: () = unsafe {
                    msg_send![&*enc, executeCommandsInBuffer: &*command_buffer.icb, withRange: range]
                };
                enc.endEncoding();
            }
        }
        cb.commit();
        cb.waitUntilCompleted();
        match cb.error() {
            Some(error) => Err(format!("graph replay failed: {error}")),
            None => Ok(cb),
        }
    }
}

fn icb_resource_usages(dispatches: &[Dispatch]) -> Vec<(MetalBuf, MTLResourceUsage)> {
    fn record(
        resources: &mut Vec<(MetalBuf, MTLResourceUsage)>,
        indices: &mut HashMap<usize, usize>,
        buffer: &MetalBuf,
        usage: MTLResourceUsage,
    ) {
        let allocation = Retained::as_ptr(&buffer.0) as usize;
        if let Some(&index) = indices.get(&allocation) {
            resources[index].1 |= usage;
        } else {
            indices.insert(allocation, resources.len());
            resources.push((buffer.clone(), usage));
        }
    }

    let mut resources = Vec::new();
    let mut indices = HashMap::new();
    for dispatch in dispatches {
        for input in &dispatch.inputs {
            record(&mut resources, &mut indices, input, MTLResourceUsage::Read);
        }
        if let Some(argument_buffer) = &dispatch.argbuf {
            record(
                &mut resources,
                &mut indices,
                argument_buffer,
                MTLResourceUsage::Read,
            );
        }
        record(
            &mut resources,
            &mut indices,
            &dispatch.output,
            MTLResourceUsage::Write,
        );
    }
    resources
}

/// Record one concurrent ICB command and report whether it conflicts with an
/// earlier command. Reading or overwriting an earlier write is RAW/WAW;
/// overwriting anything previously accessed is WAR. Allocation identity is
/// deliberately conservative: two slices of one `MTLBuffer` synchronize even
/// when their byte ranges do not overlap.
fn icb_command_needs_barrier(
    inputs: &[usize],
    output: usize,
    accessed: &mut HashSet<usize>,
    written: &mut HashSet<usize>,
) -> bool {
    let hazard = inputs
        .iter()
        .chain(std::iter::once(&output))
        .any(|allocation| written.contains(allocation))
        || accessed.contains(&output);
    accessed.extend(inputs.iter().copied());
    accessed.insert(output);
    written.insert(output);
    hazard
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
        .map(|st| {
            let inputs: Vec<MetalBuf> = st.inputs.iter().map(|n| bufs[n].clone()).collect();
            // a bindless stage fills its argument buffer with the inputs' GPU
            // addresses (rebuilt each call, so a post-commit buffer swap is
            // reflected) and binds that table instead of the inputs directly.
            let argbuf = st.argbuf.as_ref().map(|name| {
                let ab = bufs[name].clone();
                let ptr = ab.contents() as *mut u64;
                for (i, b) in inputs.iter().enumerate() {
                    unsafe { *ptr.add(i) = b.gpu_address() };
                }
                ab
            });
            Dispatch {
                pipe: pipes.get(&st.kernel),
                inputs,
                output: bufs[&st.output].clone(),
                grid: st.grid_size,
                argbuf,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::icb_command_needs_barrier;
    use std::collections::HashSet;

    #[test]
    fn icb_barriers_cover_fanout_and_write_after_read() {
        let mut accessed = HashSet::new();
        let mut written = HashSet::new();

        assert!(!icb_command_needs_barrier(
            &[],
            1,
            &mut accessed,
            &mut written
        ));
        assert!(icb_command_needs_barrier(
            &[1],
            2,
            &mut accessed,
            &mut written
        ));
        assert!(
            icb_command_needs_barrier(&[1], 3, &mut accessed, &mut written),
            "a barrier on the first consumer does not fence the second consumer"
        );

        let mut accessed = HashSet::new();
        let mut written = HashSet::new();
        assert!(!icb_command_needs_barrier(
            &[4],
            5,
            &mut accessed,
            &mut written
        ));
        assert!(
            icb_command_needs_barrier(&[], 4, &mut accessed, &mut written),
            "an in-place writer must wait for earlier readers"
        );
    }
}
