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
//!   dispatch list as ONE concurrent compute encoder in one command buffer,
//!   a buffer-scope memory barrier at each statically computed dependency
//!   frontier ([`barrier_schedule`]), and waits. Metal does no hazard
//!   tracking inside a concurrent encoder — the schedule is load-bearing,
//!   and every oracle test runs through it.
//!   [`MetalDevice::run_kernel_timed`] instead gives every kernel its own
//!   encoder and samples a GPU timestamp at each boundary — per-kernel time,
//!   at the price of per-kernel encoder boundaries.
//!
//! [`program_dispatches`] resolves a whole emitted [`MetalProgram`] against a
//! name→buffer map — rebuilt per step when a runtime swaps buffers (the
//! KV-cache commit), since dispatches bind by name at build time.
//!
//! * **Graphs** — [`MetalDevice::capture`] freezes a dispatch list, its
//!   bindings, and its barrier schedule; a replay re-encodes them cheaply
//!   each step. Swap commits keep one frozen graph per parity.

use std::collections::{HashMap, HashSet};

use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2_foundation::{NSRange, NSString, NSURL};
use objc2_metal::{
    MTLBarrierScope, MTLBuffer, MTLCaptureDescriptor, MTLCaptureDestination, MTLCaptureManager, MTLCommandBuffer,
    MTLCommandEncoder, MTLCommandQueue, MTLCommonCounterSetTimestamp, MTLComputeCommandEncoder,
    MTLComputePassDescriptor, MTLComputePipelineDescriptor, MTLComputePipelineState, MTLCounterSampleBuffer,
    MTLCounterSampleBufferDescriptor, MTLCounterSamplingPoint, MTLCounterSet, MTLCreateSystemDefaultDevice, MTLDevice,
    MTLDispatchType, MTLFunction, MTLLibrary, MTLPipelineOption, MTLPixelFormat, MTLResource, MTLResourceOptions,
    MTLResourceUsage, MTLSize, MTLStorageMode, MTLTexture, MTLTextureDescriptor, MTLTextureType,
};

use crate::emit_metal::MetalProgram;
use crate::scalar::Dtype;

/// A pipeline for one compiled kernel.
pub type Pipeline = Retained<ProtocolObject<dyn MTLComputePipelineState>>;

/// IEEE 754 half → f32: sign through, exponent rebiased, subnormals scaled.
fn half_to_f32(h: u16) -> f32 {
    let sign = ((h >> 15) & 1) as u32;
    let exp = ((h >> 10) & 0x1F) as u32;
    let frac = (h & 0x3FF) as u32;
    let bits = match (exp, frac) {
        (0, 0) => sign << 31,
        (0, f) => return if sign == 1 { -1.0 } else { 1.0 } * f as f32 * 2f32.powi(-24),
        (0x1F, f) => (sign << 31) | 0x7F80_0000 | (f << 13),
        (e, f) => (sign << 31) | ((e + 112) << 23) | (f << 13),
    };
    f32::from_bits(bits)
}

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

/// A texture VIEW over bytes that already live in a buffer — no copy.
#[derive(Clone)]
pub struct MetalTexture(Retained<ProtocolObject<dyn MTLTexture>>);

impl Dispatch {
    /// Threads per threadgroup — CUDA's block size: the pipeline's occupancy
    /// cap, clamped to the grid. The one launch-shape rule, shared by
    /// [`encode`] and the debug dumps.
    pub fn threadgroup_threads(&self) -> usize {
        self.pipe.maxTotalThreadsPerThreadgroup().min(self.grid)
    }
}

#[derive(Clone)]
pub struct MetalDevice {
    dev: Retained<ProtocolObject<dyn MTLDevice>>,
    queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    /// Boundary storage the compiled schedule writes at kernel boundaries.
    storage: Dtype,
}

impl MetalDevice {
    /// `None` when the machine has no Metal device (CI without a GPU).
    pub fn open() -> Option<MetalDevice> {
        let dev = MTLCreateSystemDefaultDevice()?;
        let queue = dev.newCommandQueue().expect("command queue");
        Some(MetalDevice {
            dev,
            queue,
            storage: Dtype::F32,
        })
    }

    /// The same device compiling schedules that store intermediates and
    /// outputs at `storage` precision (kernels still accumulate f32).
    pub fn with_storage(mut self, storage: Dtype) -> Self {
        self.storage = storage;
        self
    }

    pub fn storage(&self) -> Dtype {
        self.storage
    }

    // ── compiler ─────────────────────────────────────────────────────────────

    /// Compile one MSL source; index every kernel it defines.
    ///
    /// Math mode is RELAXED, not the default FAST: fast math lets the shader
    /// compiler assume no ±inf ever occurs, which deletes additive `-INFINITY`
    /// masks (softmax masking) at compile time. Relaxed keeps the value
    /// optimizations but preserves inf/nan semantics.
    pub fn compile(&self, msl: &str) -> Pipelines {
        let options = objc2_metal::MTLCompileOptions::new();
        options.setMathMode(objc2_metal::MTLMathMode::Relaxed);
        let lib = self
            .dev
            .newLibraryWithSource_options_error(&NSString::from_str(msl), Some(&options))
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

    fn pipeline(&self, f: &ProtocolObject<dyn MTLFunction>, name: &str) -> Pipeline {
        let desc = MTLComputePipelineDescriptor::new();
        desc.setComputeFunction(Some(f));
        self.dev
            .newComputePipelineStateWithDescriptor_options_reflection_error(&desc, MTLPipelineOption::empty(), None)
            .unwrap_or_else(|e| panic!("pipeline `{name}`: {e}"))
    }

    // ── allocator ────────────────────────────────────────────────────────────

    /// A zeroed buffer of `count` f32 elements.
    pub fn alloc_f32(&self, count: usize) -> MetalBuf {
        self.alloc_bytes(count.max(1) * 4)
    }

    /// A zeroed buffer of `count` elements at a storage dtype's width.
    pub fn alloc_elems(&self, count: usize, dtype: Dtype) -> MetalBuf {
        self.alloc_bytes(dtype.nbytes(count.max(1)))
    }

    /// Read `count` elements stored as `dtype`, widened to f32. The bf16
    /// widen mirrors the kernels' load: high 16 bits of the f32 pattern.
    pub fn read_as_f32(&self, buf: &MetalBuf, count: usize, dtype: Dtype) -> Vec<f32> {
        match dtype {
            Dtype::F32 => self.read_f32(buf, count),
            Dtype::BF16 => {
                let ptr = buf.contents() as *const u16;
                (0..count)
                    .map(|i| f32::from_bits((unsafe { *ptr.add(i) } as u32) << 16))
                    .collect()
            }
            Dtype::F16 => {
                let ptr = buf.contents() as *const u16;
                (0..count).map(|i| half_to_f32(unsafe { *ptr.add(i) })).collect()
            }
            // Exact to 2^24 in this widening — and the law admits nothing an
            // f32 register could not have held exactly anyway.
            Dtype::U32 => {
                let ptr = buf.contents() as *const u32;
                (0..count).map(|i| unsafe { *ptr.add(i) } as f32).collect()
            }
            other => panic!("{other:?} is not a boundary storage dtype"),
        }
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

    /// Replay one frozen graph on `queues` INDEPENDENT command queues at once,
    /// returning the wall seconds for all of them to finish.
    ///
    /// Submissions to different queues are separate submission contexts and may
    /// execute concurrently. Dispatches inside one command buffer are not the
    /// same question: this asks whether the device's bandwidth ceiling belongs
    /// to a queue or to the hardware (`tests/bandwidth_probe.rs`). Every queue
    /// replays the same read-only graph, so there is no hazard between them.
    pub fn replay_on_parallel_queues(&self, g: &MetalGraph, queues: usize) -> f64 {
        let extra: Vec<_> = (0..queues)
            .map(|_| self.dev.newCommandQueue().expect("command queue"))
            .collect();
        let buffers: Vec<_> = extra
            .iter()
            .map(|q| {
                let cb = q.commandBuffer().expect("command buffer");
                encode_graph(&cb, &g.dispatches, &g.barriers);
                cb.commit();
                cb
            })
            .collect();
        for cb in &buffers {
            cb.waitUntilCompleted();
        }
        // GPU span, not wall: wall would be dominated by encoding every graph
        // on the host before any of them starts, which is not what is being
        // asked. First start to last end covers whatever overlap the GPU gave.
        let starts = buffers
            .iter()
            .map(|cb| -> f64 { unsafe { msg_send![&**cb, GPUStartTime] } });
        let ends = buffers
            .iter()
            .map(|cb| -> f64 { unsafe { msg_send![&**cb, GPUEndTime] } });
        let first = starts.fold(f64::INFINITY, f64::min);
        let last = ends.fold(f64::NEG_INFINITY, f64::max);
        last - first
    }

    /// A `texture_buffer` view over `buf`'s first `elements` RGBA32Uint texels
    /// (16 B each). Same bytes, different path into the core.
    pub fn texture_view(&self, buf: &MetalBuf, elements: usize) -> MetalTexture {
        let desc = MTLTextureDescriptor::new();
        desc.setTextureType(MTLTextureType::TypeTextureBuffer);
        desc.setPixelFormat(MTLPixelFormat::RGBA32Uint);
        unsafe {
            desc.setWidth(elements);
            desc.setHeight(1);
        }
        desc.setStorageMode(buf.0.storageMode());
        desc.setUsage(objc2_metal::MTLTextureUsage::ShaderRead);
        let tex = buf
            .0
            .newTextureWithDescriptor_offset_bytesPerRow(&desc, buf.1, elements * 16)
            .expect("texture view over buffer");
        MetalTexture(tex)
    }

    /// Time one dispatch that binds `tex` at `[[texture(0)]]` alongside its
    /// buffers, returning the fastest GPU span over `reps` replays.
    ///
    /// Apple GPUs serve textures through their own cache and request path. If
    /// that path has miss slots of its own, a kernel reading through BOTH can
    /// hold more outstanding lines per core than the ~23 a buffer-only kernel
    /// tops out at — which is the one way past the ceiling that the rest of
    /// `tests/bandwidth_probe.rs` has not ruled out.
    pub fn time_with_texture(
        &self,
        pipe: &Pipeline,
        inputs: &[MetalBuf],
        texture: Option<&MetalTexture>,
        output: &MetalBuf,
        grid: usize,
        reps: usize,
    ) -> f64 {
        let mut best = f64::INFINITY;
        for _ in 0..reps {
            let cb = self.queue.commandBuffer().expect("command buffer");
            let enc = cb
                .computeCommandEncoderWithDispatchType(MTLDispatchType::Concurrent)
                .expect("encoder");
            enc.setComputePipelineState(pipe);
            for (i, b) in inputs.iter().enumerate() {
                unsafe { enc.setBuffer_offset_atIndex(Some(&b.0), b.1, i) };
            }
            unsafe { enc.setBuffer_offset_atIndex(Some(&output.0), output.1, inputs.len()) };
            if let Some(t) = texture {
                unsafe { enc.setTexture_atIndex(Some(&t.0), 0) };
            }
            let tg = pipe.maxTotalThreadsPerThreadgroup().min(grid);
            enc.dispatchThreads_threadsPerThreadgroup(
                MTLSize {
                    width: grid,
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
            cb.commit();
            cb.waitUntilCompleted();
            best = best.min(gpu_seconds(&cb));
        }
        best
    }

    /// A GPU-PRIVATE buffer of raw bytes: no CPU-visible mapping, so the
    /// fabric owes it no coherency. Nothing can read it from the host, which
    /// is why the rest of the allocator is `Shared` — but a streaming
    /// benchmark can ask whether coherency is costing bandwidth
    /// (`tests/bandwidth_probe.rs`).
    pub fn alloc_private_bytes(&self, bytes: usize) -> MetalBuf {
        let buf = self
            .dev
            .newBufferWithLength_options(bytes.max(4), MTLResourceOptions::StorageModePrivate)
            .expect("private buffer allocation");
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
            self.dev.newBufferWithBytesNoCopy_length_options_deallocator(
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

    /// Run every dispatch now: one command buffer, ONE concurrent encoder,
    /// barriers where [`barrier_schedule`] found dependency frontiers — the
    /// same execution law as a frozen graph replay — and wait.
    pub fn run(&self, dispatches: &[Dispatch]) {
        let cb = self.queue.commandBuffer().expect("command buffer");
        encode_graph(&cb, dispatches, &barrier_schedule(dispatches));
        cb.commit();
        cb.waitUntilCompleted();
    }

    /// [`Self::run`] with a GPU timestamp sampled at every encoder boundary:
    /// per-kernel times measured INSIDE the one command buffer — the
    /// production regime, no per-launch sync floor. Returns the whole
    /// buffer's GPU residency and each kernel's seconds; `None` when the
    /// device cannot sample timestamps at stage boundaries (callers fall
    /// back to [`Self::run_each_timed`]).
    pub fn run_kernel_timed(&self, dispatches: &[Dispatch]) -> Option<(f64, Vec<f64>)> {
        let timer = self.encoder_timestamps(dispatches.len())?;
        let cb = self.queue.commandBuffer().expect("command buffer");
        for (kernel, dispatch) in dispatches.iter().enumerate() {
            encode(&cb, dispatch, Some(&timer.pass_descriptor(kernel)));
        }
        cb.commit();
        cb.waitUntilCompleted();
        let seconds = gpu_seconds(&cb);
        Some((seconds, timer.kernel_seconds(seconds)?))
    }

    /// A timestamp sample buffer with room to bracket `kernels` dispatches.
    /// `None` where the device can't sample at stage boundaries or has no
    /// timestamp counter set — both are device capabilities, present on
    /// Apple Silicon.
    fn encoder_timestamps(&self, kernels: usize) -> Option<EncoderTimestamps> {
        if kernels == 0
            || !self
                .dev
                .supportsCounterSampling(MTLCounterSamplingPoint::AtStageBoundary)
        {
            return None;
        }
        let sets = self.dev.counterSets()?;
        let timestamp_name = unsafe { MTLCommonCounterSetTimestamp };
        let timestamp_set = sets.iter().find(|set| &*set.name() == timestamp_name)?;
        let descriptor = MTLCounterSampleBufferDescriptor::new();
        descriptor.setCounterSet(Some(&timestamp_set));
        descriptor.setStorageMode(MTLStorageMode::Shared);
        unsafe { descriptor.setSampleCount(2 * kernels) };
        let samples = self.dev.newCounterSampleBufferWithDescriptor_error(&descriptor).ok()?;
        Some(EncoderTimestamps { samples, kernels })
    }

    /// One command buffer PER dispatch: each kernel's own GPU residency, in
    /// order (tinygrad's `DEBUG=2` also synchronizes per kernel to time it).
    /// The fallback behind [`Self::run_kernel_timed`] — the per-dispatch
    /// submit adds a sync floor, so the SUM is not a decode-speed number;
    /// [`Self::run_graph_timed`] is. Individual kernel times are accurate:
    /// GPU start→end, no CPU cost.
    pub fn run_each_timed(&self, dispatches: &[Dispatch]) -> Vec<f64> {
        dispatches
            .iter()
            .map(|d| {
                let cb = self.queue.commandBuffer().expect("command buffer");
                encode(&cb, d, None);
                cb.commit();
                cb.waitUntilCompleted();
                gpu_seconds(&cb)
            })
            .collect()
    }
}

/// GPU timestamps sampled at encoder boundaries DURING one command buffer.
/// Apple GPUs sample counters only at stage boundaries — but [`encode`]
/// gives every kernel its own encoder, so encoder boundaries ARE kernel
/// boundaries: samples 2i and 2i+1 bracket dispatch i.
struct EncoderTimestamps {
    samples: Retained<ProtocolObject<dyn MTLCounterSampleBuffer>>,
    kernels: usize,
}

impl EncoderTimestamps {
    /// A compute-pass descriptor directing kernel `kernel`'s encoder to
    /// record its start/end GPU timestamps.
    fn pass_descriptor(&self, kernel: usize) -> Retained<MTLComputePassDescriptor> {
        let pass = MTLComputePassDescriptor::new();
        let attachment = unsafe { pass.sampleBufferAttachments().objectAtIndexedSubscript(0) };
        attachment.setSampleBuffer(Some(&self.samples));
        unsafe { attachment.setStartOfEncoderSampleIndex(2 * kernel) };
        unsafe { attachment.setEndOfEncoderSampleIndex(2 * kernel + 1) };
        pass
    }

    /// Per-kernel seconds. The GPU tick rate is undocumented, so the sampled
    /// tick span is scaled onto the command buffer's own GPU residency
    /// (`cb_seconds`) — self-calibrating, no CPU/GPU clock correlation.
    /// `None` if any sample resolved to the error value (`u64::MAX`,
    /// `MTLCounterErrorValue`).
    fn kernel_seconds(&self, cb_seconds: f64) -> Option<Vec<f64>> {
        let range = NSRange {
            location: 0,
            length: 2 * self.kernels,
        };
        let resolved = unsafe { self.samples.resolveCounterRange(range) }?.to_vec();
        let ticks: Vec<u64> = resolved
            .chunks_exact(8)
            .map(|bytes| u64::from_ne_bytes(bytes.try_into().expect("8-byte chunk")))
            .collect();
        if ticks.len() != 2 * self.kernels || ticks.contains(&u64::MAX) {
            return None;
        }
        let first = *ticks.iter().min()?;
        let span = ticks.iter().max()? - first;
        if span == 0 {
            return None;
        }
        let seconds_per_tick = cb_seconds / span as f64;
        ticks
            .chunks_exact(2)
            .map(|pair| pair[1].checked_sub(pair[0]).map(|dt| dt as f64 * seconds_per_tick))
            .collect()
    }
}

/// Encode one kernel launch in its OWN encoder — the PROFILING regime.
/// [`MetalDevice::run_kernel_timed`] samples GPU timestamps at encoder
/// boundaries (the only sampling points Apple GPUs expose), so per-kernel
/// timing needs per-kernel encoders; `pass` carries its timestamp
/// attachment. Production execution is [`encode_graph`]'s single
/// concurrent encoder instead.
fn encode(cb: &ProtocolObject<dyn MTLCommandBuffer>, d: &Dispatch, pass: Option<&MTLComputePassDescriptor>) {
    let enc = match pass {
        Some(pass) => cb.computeCommandEncoderWithDescriptor(pass),
        None => cb.computeCommandEncoder(),
    }
    .expect("compute encoder");
    encode_dispatch(&enc, d);
    enc.endEncoding();
}

/// Encode a dispatch list into ONE concurrent compute encoder: binds and
/// launches back-to-back, a buffer-scope memory barrier exactly where
/// [`barrier_schedule`] found a dependency frontier. Independent neighbors
/// overlap on the GPU; dependent ones pay a cheap in-encoder barrier, never
/// an encoder boundary (an encoder per kernel idled the GPU ~10µs per
/// launch — ~15% of a llama step). Inside a concurrent encoder Metal does
/// NO hazard tracking between dispatches: the barrier schedule is
/// load-bearing for correctness, and the whole oracle suite runs through
/// this path to keep it honest.
fn encode_graph(cb: &ProtocolObject<dyn MTLCommandBuffer>, dispatches: &[Dispatch], barriers: &[bool]) {
    let enc = cb
        .computeCommandEncoderWithDispatchType(MTLDispatchType::Concurrent)
        .expect("compute encoder");
    for (dispatch, &barrier) in dispatches.iter().zip(barriers) {
        if barrier {
            enc.memoryBarrierWithScope(MTLBarrierScope::Buffers);
        }
        encode_dispatch(&enc, dispatch);
    }
    enc.endEncoding();
}

/// Bind and launch one kernel on an open compute encoder — the binding
/// rules shared by [`encode`] and [`encode_graph`].
fn encode_dispatch(enc: &ProtocolObject<dyn MTLComputeCommandEncoder>, d: &Dispatch) {
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
    let tg = d.threadgroup_threads();
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
}

/// A committed command buffer's GPU residency in seconds.
fn gpu_seconds(cb: &ProtocolObject<dyn MTLCommandBuffer>) -> f64 {
    // CFTimeInterval (f64 seconds); raw messages sidestep feature gates
    let t0: f64 = unsafe { msg_send![cb, GPUStartTime] };
    let t1: f64 = unsafe { msg_send![cb, GPUEndTime] };
    t1 - t0
}

/// A dispatch sequence with frozen bindings and a capture-time barrier
/// schedule, replayed each step as one concurrent encoder ([`encode_graph`]).
///
/// Buffer BINDINGS are frozen at capture. A session's swap commits flip
/// bindings with period two, so a decode loop keeps one graph per step
/// parity and replays the matching one.
///
/// History note: this replaced an `MTLIndirectCommandBuffer` path — see
/// `vs_tinygrad.md` § "The Metal ICB residency lesson" for why ICBs are not
/// a foundation on Apple7 (two distinct failure modes; only pipeline
/// residency has a clean fix).
pub struct MetalGraph {
    dispatches: Vec<Dispatch>,
    /// `barriers[i]` true — a memory barrier goes before dispatch `i`.
    barriers: Vec<bool>,
    /// The NEXT step's command buffer, encoded while the previous step's was
    /// still on the GPU. See [`MetalDevice::run_graph_timed`].
    pending: std::cell::RefCell<Option<Retained<ProtocolObject<dyn MTLCommandBuffer>>>>,
}

impl MetalDevice {
    /// Freeze a dispatch list into a replayable graph: the bindings as
    /// given, the barrier schedule computed once. Sanic knows the whole
    /// dependency structure statically, so unlike MLX's dynamic hazard
    /// tracking the schedule is derived at capture, not rediscovered per
    /// step.
    pub fn capture(&self, dispatches: &[Dispatch]) -> MetalGraph {
        let barriers = barrier_schedule(dispatches);
        if crate::debug_level() >= 3 {
            eprintln!(
                "*** metal capture: {} dispatches, {} barriers ({} concurrent phases)",
                dispatches.len(),
                barriers.iter().filter(|b| **b).count(),
                barriers.iter().filter(|b| **b).count() + 1,
            );
        }
        MetalGraph {
            barriers,
            dispatches: dispatches.to_vec(),
            pending: std::cell::RefCell::new(None),
        }
    }

    /// Replay a frozen graph and wait, returning GPU residency in seconds
    /// (`GPUEndTime − GPUStartTime`: kernel time plus inter-dispatch
    /// bubbles, free of CPU encode/submit cost).
    ///
    /// Any command buffer error — a GPU fault of our own, or "Discarded
    /// (victim of GPU error/recovery)" when something ELSE faults the GPU
    /// mid-flight — is an `Err`: the step's writes are untrustworthy and a
    /// decode loop must not continue on them.
    /// One command buffer with this graph's whole dispatch list encoded.
    fn encoded(&self, g: &MetalGraph) -> Retained<ProtocolObject<dyn MTLCommandBuffer>> {
        let cb = self.queue.commandBuffer().expect("command buffer");
        // Name the step for Instruments/Xcode GPU captures.
        cb.setLabel(Some(&NSString::from_str(&format!(
            "sanic batched {}",
            g.dispatches.len()
        ))));
        encode_graph(&cb, &g.dispatches, &g.barriers);
        cb
    }

    pub fn run_graph_timed(&self, g: &MetalGraph) -> Result<f64, String> {
        let capture = self.capture_trace();
        let cb = g.pending.borrow_mut().take().unwrap_or_else(|| self.encoded(g));
        cb.commit();
        // Encode the NEXT step while the GPU works on this one. Sound because
        // the dispatch list and its bindings never change: a step's inputs
        // reach its kernels through buffer CONTENTS, so nothing here can
        // depend on the result being waited for. Measured, this is 0.73 ms of
        // encoding per step for llama's 342 dispatches, and it was sitting on
        // the critical path.
        *g.pending.borrow_mut() = Some(self.encoded(g));
        cb.waitUntilCompleted();
        if let Some(manager) = capture {
            manager.stopCapture();
        }
        match cb.error() {
            Some(error) => Err(format!("graph replay failed: {error}")),
            None => Ok(gpu_seconds(&cb)),
        }
    }

    /// `SANIC_GPUTRACE=<path>` — capture the FIRST graph replay into a
    /// `.gputrace` document, once per process.
    ///
    /// This exists because the counters worth having are not reachable from
    /// code. Apple exposes 150+ performance counters — the limiters (ALU,
    /// buffer read/write, last-level cache), bytes read from main memory,
    /// occupancy — but only through Xcode's Metal Debugger and Instruments.
    /// What `MTLCounterSampleBuffer` offers this machine is exactly one
    /// counter set, `timestamp`, at stage boundaries, which is what
    /// [`Self::run_kernel_timed`] already spends. A trace file is therefore
    /// the only way to see WHY a kernel is slow rather than how slow it is:
    /// open the result in Xcode, where every dispatch carries the kernel
    /// name this compiler generated and every command buffer the step label.
    ///
    /// Requires `METAL_CAPTURE_ENABLED=1` in the environment.
    fn capture_trace(&self) -> Option<Retained<MTLCaptureManager>> {
        static TAKEN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        let path = std::env::var_os("SANIC_GPUTRACE")?;
        if TAKEN.swap(true, std::sync::atomic::Ordering::Relaxed) {
            return None;
        }
        let manager = unsafe { MTLCaptureManager::sharedCaptureManager() };
        if !manager.supportsDestination(MTLCaptureDestination::GPUTraceDocument) {
            eprintln!("*** SANIC_GPUTRACE: this process cannot write a trace (set METAL_CAPTURE_ENABLED=1)");
            return None;
        }
        let descriptor = MTLCaptureDescriptor::new();
        // SAFETY: ProtocolObject is a transparent wrapper over AnyObject
        let queue: &AnyObject = unsafe { &*(Retained::as_ptr(&self.queue) as *const AnyObject) };
        unsafe { descriptor.setCaptureObject(Some(queue)) };
        descriptor.setDestination(MTLCaptureDestination::GPUTraceDocument);
        descriptor.setOutputURL(Some(&NSURL::fileURLWithPath(&NSString::from_str(
            &path.to_string_lossy(),
        ))));
        match manager.startCaptureWithDescriptor_error(&descriptor) {
            Ok(()) => {
                eprintln!("*** SANIC_GPUTRACE: capturing one replay to {}", path.to_string_lossy());
                Some(manager)
            }
            Err(error) => {
                eprintln!("*** SANIC_GPUTRACE: capture refused: {error}");
                None
            }
        }
    }
}

/// Where memory barriers go when a dispatch list runs in ONE concurrent
/// encoder: before every dispatch that conflicts with an access since the
/// last barrier. Reading or overwriting an earlier write is RAW/WAW;
/// overwriting an earlier read is WAR. An encoder barrier is a phase
/// boundary — everything encoded after it waits on everything before — so
/// both access histories reset where one is placed. Allocation identity is
/// deliberately conservative: two slices of one `MTLBuffer` synchronize
/// even when their byte ranges do not overlap (the zero-copy weights file
/// is one allocation, but weights are only ever read).
fn barrier_schedule(dispatches: &[Dispatch]) -> Vec<bool> {
    let allocation = |buffer: &MetalBuf| Retained::as_ptr(&buffer.0) as usize;
    let mut earlier_reads: HashSet<usize> = HashSet::new();
    let mut earlier_writes: HashSet<usize> = HashSet::new();
    dispatches
        .iter()
        .map(|d| {
            let reads: Vec<usize> = d.inputs.iter().chain(d.argbuf.as_ref()).map(allocation).collect();
            let write = allocation(&d.output);
            barrier_before(&reads, write, &mut earlier_reads, &mut earlier_writes)
        })
        .collect()
}

/// One step of [`barrier_schedule`]: record a dispatch's accesses and report
/// whether a barrier must precede it. Placing one resets both histories —
/// the phase boundary fences every earlier access at once.
fn barrier_before(
    reads: &[usize],
    write: usize,
    earlier_reads: &mut HashSet<usize>,
    earlier_writes: &mut HashSet<usize>,
) -> bool {
    let hazard = reads.iter().any(|read| earlier_writes.contains(read))
        || earlier_writes.contains(&write)
        || earlier_reads.contains(&write);
    if hazard {
        earlier_reads.clear();
        earlier_writes.clear();
    }
    earlier_reads.extend(reads.iter().copied());
    earlier_writes.insert(write);
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
    use super::barrier_before;
    use std::collections::HashSet;

    #[test]
    fn barriers_sit_on_dependency_frontiers_only() {
        let mut reads = HashSet::new();
        let mut writes = HashSet::new();

        // producer writes 1; the first consumer barriers; the barrier is a
        // PHASE boundary, so the second consumer of the same producer is
        // already fenced and rides free.
        assert!(!barrier_before(&[], 1, &mut reads, &mut writes));
        assert!(barrier_before(&[1], 2, &mut reads, &mut writes));
        assert!(
            !barrier_before(&[1], 3, &mut reads, &mut writes),
            "the phase boundary already fenced the producer's write"
        );
        // ...but a consumer of a post-barrier write must barrier again
        assert!(barrier_before(&[2], 6, &mut reads, &mut writes));

        let mut reads = HashSet::new();
        let mut writes = HashSet::new();
        assert!(!barrier_before(&[4], 5, &mut reads, &mut writes));
        assert!(
            barrier_before(&[], 4, &mut reads, &mut writes),
            "an in-place writer must wait for earlier readers"
        );
    }
}

// ── what the GPU's clock actually did ────────────────────────────────────────

/// The GPU's DVFS behaviour over a measured window.
///
/// Apple GPUs do not run at one speed: they sit in discrete (frequency,
/// voltage) P-states and drop down when idle, warm, or sharing the package
/// power budget. A run that lands a state below peak is ~25% slower for
/// reasons that have nothing to do with the code — which is exactly the
/// drift that made this crate's tuner re-measure a baseline beside every
/// candidate. This is how a benchmark can tell instead of guess.
#[derive(Debug, Clone, Copy)]
pub struct Clock {
    /// Fraction of the window the GPU was not in the OFF state.
    pub busy: f64,
    /// Mean MHz *while busy* — the speed the work in this window ran at.
    pub mhz: f64,
    /// The device's top DVFS state.
    pub peak_mhz: f64,
}

impl Clock {
    /// Did this window run at full speed? A measurement taken when this is
    /// false is not comparable to one taken when it is true.
    ///
    /// The state can be MEASURED but not pinned — Instruments' own
    /// minimum/medium/maximum settings all leave an M1 Pro at its top state
    /// — so the only honest response to `false` is to discard the sample.
    pub fn at_peak(&self) -> bool {
        self.busy > 0.0 && self.mhz >= 0.98 * self.peak_mhz
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0} MHz, {:.0}% busy", self.mhz, 100.0 * self.busy)
    }
}

/// An open DVFS measurement. Residency is a DELTA between two IOReport
/// samples, so this measures a WINDOW, never an instant: open it before the
/// work and [`ClockWatch::read`] after.
///
/// Device-wide, not process-wide — the channel reports the accelerator, so
/// another process's load lands in the number. That is worth knowing rather
/// than hiding: it is also how "something else was using the GPU during my
/// benchmark" becomes visible.
pub struct ClockWatch {
    subscription: clock_report::Subscription,
    opened: clock_report::Sample,
    peak_mhz: f64,
}

impl ClockWatch {
    /// Busy fraction and mean clock since this watch was opened.
    pub fn read(&self) -> Option<Clock> {
        let now = self.subscription.sample()?;
        let (busy, mhz) = self.subscription.residency(&self.opened, &now)?;
        Some(Clock {
            busy,
            mhz,
            peak_mhz: self.peak_mhz,
        })
    }
}

impl MetalDevice {
    /// Open a [`ClockWatch`] over this GPU's DVFS states. `None` where the
    /// IOReport channels are unavailable.
    ///
    /// ```no_run
    /// # let device = sanic::MetalDevice::open().unwrap();
    /// # let graph = unimplemented!();
    /// let watch = device.clock();
    /// let seconds = device.run_graph_timed(&graph)?;
    /// // a number measured below peak is not comparable to one measured at it
    /// let trustworthy = watch.as_ref().and_then(|w| w.read()).is_none_or(|c| c.at_peak());
    /// # Ok::<(), String>(())
    /// ```
    pub fn clock(&self) -> Option<ClockWatch> {
        let subscription = clock_report::Subscription::open()?;
        let opened = subscription.sample()?;
        Some(ClockWatch {
            subscription,
            opened,
            peak_mhz: clock_report::peak_mhz()?,
        })
    }
}

/// IOReport, the unprivileged path to the GPU's DVFS residency.
///
/// The counters worth having (ALU/buffer/cache limiters, occupancy,
/// bandwidth) are NOT reachable in-process: the private AGX raw-counter
/// session refuses without an Apple-only entitlement, and Instruments
/// collects them out-of-process in an entitled daemon (`SANIC_GPUTRACE`
/// captures a trace for it instead). DVFS residency is the exception — it
/// is a plain IOReport channel any process may subscribe to.
mod clock_report {
    use std::ffi::{CStr, CString, c_char, c_void};

    type Ref = *const c_void;

    #[link(name = "CoreFoundation", kind = "framework")]
    unsafe extern "C" {
        static kCFAllocatorDefault: Ref;
        fn CFStringCreateWithCString(allocator: Ref, text: *const c_char, encoding: u32) -> Ref;
        fn CFStringGetCString(text: Ref, buffer: *mut c_char, len: isize, encoding: u32) -> bool;
        fn CFDictionaryGetValue(dictionary: Ref, key: Ref) -> Ref;
        fn CFArrayGetCount(array: Ref) -> isize;
        fn CFArrayGetValueAtIndex(array: Ref, index: isize) -> Ref;
        fn CFDataGetLength(data: Ref) -> isize;
        fn CFDataGetBytePtr(data: Ref) -> *const u8;
        fn CFRelease(object: Ref);
    }

    #[link(name = "IOKit", kind = "framework")]
    unsafe extern "C" {
        fn IORegistryEntryFromPath(port: u32, path: *const c_char) -> u32;
        fn IORegistryEntryCreateCFProperty(entry: u32, key: Ref, allocator: Ref, options: u32) -> Ref;
        fn IOObjectRelease(object: u32) -> i32;
    }

    #[link(name = "IOReport")]
    unsafe extern "C" {
        fn IOReportCopyChannelsInGroup(group: Ref, subgroup: Ref, a: u64, b: u64, c: u64) -> Ref;
        fn IOReportCreateSubscription(a: *mut c_void, channels: Ref, out: *mut Ref, id: u64, x: Ref) -> Ref;
        fn IOReportCreateSamples(subscription: Ref, channels: Ref, x: Ref) -> Ref;
        fn IOReportCreateSamplesDelta(first: Ref, second: Ref, x: Ref) -> Ref;
        fn IOReportChannelGetChannelName(channel: Ref) -> Ref;
        fn IOReportStateGetCount(channel: Ref) -> i32;
        fn IOReportStateGetNameForIndex(channel: Ref, index: i32) -> Ref;
        fn IOReportStateGetResidency(channel: Ref, index: i32) -> i64;
    }

    const UTF8: u32 = 0x0800_0100;
    /// The GPU's DVFS residency channel, in ticks of a 24 MHz counter.
    const GPU_STATE_CHANNEL: &str = "GPUPH";

    /// A CoreFoundation object released on drop.
    struct Owned(Ref);

    impl Drop for Owned {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { CFRelease(self.0) };
            }
        }
    }

    fn string(text: &str) -> Owned {
        let c = CString::new(text).expect("no interior nul");
        Owned(unsafe { CFStringCreateWithCString(kCFAllocatorDefault, c.as_ptr(), UTF8) })
    }

    fn read_string(text: Ref) -> String {
        let mut buffer = [0 as c_char; 128];
        unsafe {
            if text.is_null() || !CFStringGetCString(text, buffer.as_mut_ptr(), 128, UTF8) {
                return String::new();
            }
            CStr::from_ptr(buffer.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// The GPU's DVFS frequency table, state index → MHz, from the device
    /// tree rather than the accelerator: `voltage-states9` is a run of
    /// little-endian `(hertz: u32, millivolts: u32)` pairs whose order lines
    /// up with the residency channel's states (index 0 = OFF = 0 Hz).
    fn dvfs_table() -> Option<Vec<f64>> {
        let path = CString::new("IODeviceTree:/arm-io/pmgr").expect("no interior nul");
        let entry = unsafe { IORegistryEntryFromPath(0, path.as_ptr()) };
        if entry == 0 {
            return None;
        }
        let key = string("voltage-states9");
        let data = Owned(unsafe { IORegistryEntryCreateCFProperty(entry, key.0, kCFAllocatorDefault, 0) });
        unsafe { IOObjectRelease(entry) };
        if data.0.is_null() {
            return None;
        }
        let bytes =
            unsafe { std::slice::from_raw_parts(CFDataGetBytePtr(data.0), CFDataGetLength(data.0).max(0) as usize) };
        Some(
            bytes
                .chunks_exact(8)
                .map(|pair| u32::from_le_bytes(pair[..4].try_into().expect("4 bytes")) as f64 / 1e6)
                .collect(),
        )
    }

    /// The device's top DVFS state, in MHz.
    pub fn peak_mhz() -> Option<f64> {
        dvfs_table()?
            .into_iter()
            .fold(None, |top: Option<f64>, mhz| Some(top.map_or(mhz, |t| t.max(mhz))))
    }

    pub struct Sample(Owned);

    pub struct Subscription {
        subscription: Owned,
        channels: Owned,
        mhz: Vec<f64>,
    }

    impl Subscription {
        pub fn open() -> Option<Subscription> {
            let (group, subgroup) = (string("GPU Stats"), string("GPU Performance States"));
            let channels = Owned(unsafe { IOReportCopyChannelsInGroup(group.0, subgroup.0, 0, 0, 0) });
            if channels.0.is_null() {
                return None;
            }
            let mut subscribed: Ref = std::ptr::null();
            let subscription = Owned(unsafe {
                IOReportCreateSubscription(std::ptr::null_mut(), channels.0, &mut subscribed, 0, std::ptr::null())
            });
            if subscription.0.is_null() {
                return None;
            }
            Some(Subscription {
                subscription,
                channels: Owned(subscribed),
                mhz: dvfs_table()?,
            })
        }

        pub fn sample(&self) -> Option<Sample> {
            let sample =
                Owned(unsafe { IOReportCreateSamples(self.subscription.0, self.channels.0, std::ptr::null()) });
            (!sample.0.is_null()).then_some(Sample(sample))
        }

        /// Busy fraction and mean MHz while busy, between two samples.
        pub fn residency(&self, from: &Sample, to: &Sample) -> Option<(f64, f64)> {
            let delta = Owned(unsafe { IOReportCreateSamplesDelta(from.0.0, to.0.0, std::ptr::null()) });
            if delta.0.is_null() {
                return None;
            }
            let key = string("IOReportChannels");
            let channels = unsafe { CFDictionaryGetValue(delta.0, key.0) };
            if channels.is_null() {
                return None;
            }
            for index in 0..unsafe { CFArrayGetCount(channels) } {
                let channel = unsafe { CFArrayGetValueAtIndex(channels, index) };
                if read_string(unsafe { IOReportChannelGetChannelName(channel) }) != GPU_STATE_CHANNEL {
                    continue;
                }
                let states = unsafe { IOReportStateGetCount(channel) };
                let ticks: Vec<i64> = (0..states)
                    .map(|state| unsafe { IOReportStateGetResidency(channel, state) })
                    .collect();
                let total: i64 = ticks.iter().sum();
                if total <= 0 {
                    return None;
                }
                let (mut busy, mut weighted_mhz) = (0.0, 0.0);
                for (state, &residency) in ticks.iter().enumerate() {
                    if residency == 0 {
                        continue;
                    }
                    let share = residency as f64 / total as f64;
                    let off = read_string(unsafe { IOReportStateGetNameForIndex(channel, state as i32) }) == "OFF";
                    if !off {
                        busy += share;
                        weighted_mhz += share * self.mhz.get(state).copied().unwrap_or(0.0);
                    }
                }
                return Some((busy, if busy > 0.0 { weighted_mhz / busy } else { 0.0 }));
            }
            None
        }
    }
}
