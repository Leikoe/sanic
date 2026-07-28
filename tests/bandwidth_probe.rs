//! What can this machine ACTUALLY stream? A hand-written PROBE, no model in
//! sight — just the simplest kernel that reads memory and cannot have the read
//! optimized away.
//!
//! Context: llama decode is memory-bound by 25× (2.47 GFLOP against 2.47 GB
//! per token, 1.0 FLOP/byte against a machine balance of 25), so the only
//! roofline that binds is bandwidth. The M1 Pro datasheet says 200 GB/s —
//! LPDDR5-6400 on a 256-bit bus, 204.8 GB/s theoretical.
//!
//! **THE ANSWER, in two parts.**
//!
//! **1. The memory system reaches and exceeds its rated number — with two
//! clients.** Driving the GPU and the CPU at once:
//!
//! ```text
//!   GPU alone                    184.4 GB/s
//!   GPU while the CPU streams    120.4
//!   CPU concurrently              90.1
//!   COMBINED                     210.5 GB/s   > the 200 GB/s figure
//! ```
//!
//! The CPU side is 6 threads over 384 MB slabs each — 2.3 GB live, far past
//! the 24 MB system cache, so it is DRAM traffic and not a cache replay. The
//! fabric is therefore NOT the thing stopping a GPU kernel at 185.
//!
//! **2. One GPU client tops out at ~185 GB/s, and no shape beat it.** That is
//! 90% of the 204.8 GB/s LPDDR5-6400 x 256-bit theoretical, 92% of Apple's
//! 200. Everything tried, and none of it moved the number: per-lane widths
//! 4/8/16 B; 1/2/4/8 independent streams per lane; threadgroups of
//! 64/256/1024; lane counts 8k-4M; shared vs GPU-private storage (identical,
//! so coherency is not the cost); populated vs untouched pages (barely
//! differs, so not faulting); read vs read+write; 1/2/4/8 concurrent
//! independent kernels (MORE streams is slightly WORSE); and the fixed launch
//! cost, measured at 3.9 us against a 733 us kernel — 0.5%, not the gap.
//!
//! So for a GPU-only workload, ~185 GB/s is the honest ceiling on this
//! machine, and the 200 GB/s figure is fabric capacity shared across clients
//! rather than anything one engine can take.
//!
//! **The one shape that DOES matter is traversal locality**, and it is worth
//! 48%:
//!
//! The decisive measurement. Same 2 GB, same 65k lanes, same bytes read; the
//! only difference is how many pages are live at once:
//!
//! ```text
//!   one grid-stride pass over 2 GB        121.5 GB/s
//!   4 windows of 512 MB                   178.2
//!   16 windows of 128 MB                  179.8
//!   32 windows of 64 MB                   179.7
//! ```
//!
//! **+48% for free.** A grid-stride over the whole buffer puts 65k lanes on
//! 65k different pages simultaneously; walking the same bytes in windows keeps
//! the live page set to one window. Nothing about the memory changed.
//!
//! This corrects an earlier reading of the size sweep below (185 at 256 MB
//! falling to 120 at 2 GB) as "bandwidth degrades with working set". It does
//! not. That curve is a traversal artifact: those runs strided across the
//! whole buffer, so the footprint and the live page set grew together. Filling
//! every page first (`fill_all`) barely moves it, so it was never unbacked
//! pages either. Bandwidth as a function of TRANSFER size behaves the ordinary
//! way — 64 MB 181.6, 128 MB 188.5, the rising limb of amortized overhead.
//!
//! Why it matters here: a llama step streams 2.47 GB per token, and reads it
//! as ~150 separate weight tensors of 2-33 MB each — which is windowed
//! traversal by construction. That is why the step measures 158 GB/s rather
//! than the 121 a naive flat pass over the same volume gets. Against the ~180
//! a windowed stream of that size sustains, 158 is 88%, and THAT is the honest
//! headroom number for this workload — not 73% of a 200 GB/s spec.
//!
//! Stated honestly, 185 is a LOWER BOUND on what this machine can stream, not
//! a proven ceiling — it is the best of a search, and a search only covers the
//! shapes someone thought of. Tried and did NOT beat it: per-lane widths
//! 4/8/16 B, 1/2/4/8 independent streams per lane, threadgroups of 64/256/1024,
//! lane counts from 4k to 4M, coalesced against per-lane contiguous, read
//! against read+write, and SHARED against GPU-PRIVATE storage (identical, so
//! CPU coherency is not the cost). Untried, and where a higher number would
//! most plausibly hide: several source buffers streamed concurrently, and
//! anything that enlarges page-translation reach — which the curve above says
//! is the live lever.
//!
//! The lane/width sweep, reproducible to ~1% across runs:
//!
//! ```text
//!   pure read, best of a lane/width sweep      185.8 GB/s   93% of spec
//!   copy (read+write, counting both)           181.0 GB/s   90% of spec
//!   per-lane contiguous runs (no coalescing)   136.1 GB/s   74% of the 184
//! ```
//!
//! Three things follow, and they retire a recurring question:
//!
//! 1. The ~8% under spec looks like the bus, not us. Reads and copies land in
//!    the same place, so it is not a direction-mix or a kernel-shape effect.
//!    Quote 184 as the achievable roofline — and if someone finds a shape that
//!    beats it, that is a real result and this probe is where it should land.
//! 2. **The lm_head fold already runs AT that ceiling** (184 GB/s measured in
//!    a real step). It is not near-optimal, it is optimal, and no codegen work
//!    on it can pay. `cost.rs`'s `hbm_bandwidth` of 2.0e11 is therefore ~9%
//!    optimistic as a ranking input.
//! 3. Access pattern is worth 26%. Lanes striding together (each taking 4–16 B
//!    of a shared run) reach 184; each lane walking its OWN contiguous run
//!    tops out at 136 no matter how many lanes there are. Coalescing across
//!    the simdgroup is the whole difference.
//!
//! The peak needs a specific shape — ~65k lanes of scalar 4-byte reads. Wider
//! per-lane vectors do not help and often hurt, which is the same verdict
//! `wide_loads_probe.rs` reached from the model side.
//!
//! The sweep is over the two quantities Little's law says decide it — how many
//! lanes are streaming and how many bytes each keeps in flight — plus the
//! access pattern.

#![cfg(target_os = "macos")]

use sanic::ir::Dtype;
use sanic::metal::{Dispatch, MetalDevice};

/// Big enough that no cache holds it. The M1 Pro's system level cache is
/// 24 MB; this is 20× that, so every byte comes from DRAM.
const BYTES: usize = 512 << 20;

const MSL: &str = r#"
#include <metal_stdlib>
using namespace metal;

// Grid-stride reads: consecutive lanes touch consecutive addresses, so each
// simdgroup's loads coalesce into full cache lines. The accumulator is XOR so
// the compiler cannot fold the loop, and it is stored, so nothing is dead.

kernel void stream_u32(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = ELEMS;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

kernel void stream_u32x2(
    device const uint2* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = ELEMS / 2;
    uint2 acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc.x ^ acc.y;
}

kernel void stream_u32x4(
    device const uint4* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = ELEMS / 4;
    uint4 acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc.x ^ acc.y ^ acc.z ^ acc.w;
}

// Two independent streams per lane: doubles bytes in flight without doubling
// lanes, which is the other half of Little's law.
kernel void stream_u32x4_dual(
    device const uint4* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = ELEMS / 4;
    uint4 a = 0, b = 0;
    uint mid = n / 2;
    for (uint i = gid; i < mid; i += lanes) { a ^= src[i]; b ^= src[i + mid]; }
    uint4 acc = a ^ b;
    out[gid] = acc.x ^ acc.y ^ acc.z ^ acc.w;
}

// Deeper per-lane streams: Little's law says outstanding bytes = lanes x
// bytes-in-flight-per-lane, and the sweep above only ever tested 1 or 2
// independent loads per lane. If the ceiling is a lanes*depth product rather
// than a hard bus limit, more depth at fewer lanes should find it.
kernel void stream_deep4(
    device const uint4* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = ELEMS / 4, q = n / 4;
    uint4 a = 0, b = 0, c = 0, d = 0;
    for (uint i = gid; i < q; i += lanes) {
        a ^= src[i]; b ^= src[i + q]; c ^= src[i + 2*q]; d ^= src[i + 3*q];
    }
    uint4 acc = (a ^ b) ^ (c ^ d);
    out[gid] = acc.x ^ acc.y ^ acc.z ^ acc.w;
}

kernel void stream_deep8(
    device const uint4* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = ELEMS / 4, q = n / 8;
    uint4 acc = 0;
    for (uint i = gid; i < q; i += lanes) {
        uint4 t = 0;
        for (uint j = 0; j < 8u; j++) t ^= src[i + j*q];
        acc ^= t;
    }
    out[gid] = acc.x ^ acc.y ^ acc.z ^ acc.w;
}

// Threadgroup size was never varied above: the dispatch takes
// min(pipeline max, grid), which pinned it at 1024. These cap it explicitly.
[[max_total_threads_per_threadgroup(256)]]
kernel void stream_tg256(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = ELEMS;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

[[max_total_threads_per_threadgroup(64)]]
kernel void stream_tg64(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = ELEMS;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

// Element count as a buffer argument, so one pipeline serves any size.
kernel void stream_u32_n(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = BIGELEMS;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}


kernel void stream_sz64(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 16777216u;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

kernel void stream_sz128(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 33554432u;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

kernel void stream_sz256(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 67108864u;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

kernel void stream_sz512(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 134217728u;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

kernel void stream_sz1024(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 268435456u;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

kernel void stream_sz2048(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 536870912u;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

// Make every page real. A private buffer is allocated but never written, so
// reads of it are not reads of memory — they are whatever the system does with
// an untouched mapping. Any streaming number taken before this is a fiction,
// and the fiction gets worse with size, which is exactly the wrong direction.
kernel void fill_all(
    device uint4* dst [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = FILLELEMS;
    for (uint i = gid; i < n; i += lanes) dst[i] = uint4(i, i ^ 0x5a5au, i + 1u, ~i);
    if (gid == 0) out[0] = 1;
}


kernel void win_u32(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 33554432u;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

kernel void win_u32x2(
    device const uint2* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 16777216u;
    uint2 acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc.x ^ acc.y;
}

kernel void win_u32x4(
    device const uint4* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 8388608u;
    uint4 acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc.x ^ acc.y ^ acc.z ^ acc.w;
}

// A dispatch that reads nothing: whatever GPUStartTime->GPUEndTime charges
// for an empty command buffer is fixed cost sitting inside every number above.
kernel void nothing(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]]
) {
    if (gid == 0) out[0] = src[0];
}

// The same 128 MB window read REPEATS times inside ONE dispatch, so the fixed
// cost is paid once over 16x the bytes instead of once over 0.7 ms.
kernel void win_repeat(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = WINELEMS;
    uint acc = 0;
    for (uint r = 0; r < REPEATS; r++)
        for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}


kernel void ml_512k(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 131072u;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

kernel void ml_1024k(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 262144u;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

kernel void ml_2048k(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 524288u;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

kernel void ml_4096k(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 1048576u;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

kernel void ml_8192k(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 2097152u;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

kernel void ml_16384k(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 4194304u;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

kernel void ml_33792k(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 8650752u;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

kernel void ml_65536k(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = 16777216u;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

// Dependent pointer chase: each load's ADDRESS comes from the previous load,
// so nothing can overlap and the timing is pure latency. Stride is 8 KB so
// every hop lands on a different page and defeats both prefetch and row reuse.
kernel void chase(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]]
) {
    uint idx = gid;
    for (uint i = 0; i < HOPS; i++) idx = src[idx];
    out[gid] = idx;
}

// One threadgroup occupies one core, so below the core count this measures
// how bandwidth scales with cores directly.
kernel void per_core(
    device const uint* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = WINELEMS;
    uint acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc;
}

// Buffer path only, texture path only, and BOTH at once over the same bytes.
// If the two paths have separate miss queues, "both" beats either.
kernel void path_buffer(
    device const uint4* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = WINELEMS / 4;
    uint4 acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= src[i];
    out[gid] = acc.x ^ acc.y ^ acc.z ^ acc.w;
}

kernel void path_texture(
    device const uint4* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    texture_buffer<uint, access::read> tex [[texture(0)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = WINELEMS / 4;
    uint4 acc = 0;
    for (uint i = gid; i < n; i += lanes) acc ^= tex.read(i);
    out[gid] = acc.x ^ acc.y ^ acc.z ^ acc.w;
}

// Half the elements through each path, issued together so both queues fill.
kernel void path_both(
    device const uint4* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    texture_buffer<uint, access::read> tex [[texture(0)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = WINELEMS / 4;
    uint mid = n / 2;
    uint4 a = 0, b = 0;
    for (uint i = gid; i < mid; i += lanes) {
        a ^= src[i];
        b ^= tex.read(i + mid);
    }
    uint4 acc = a ^ b;
    out[gid] = acc.x ^ acc.y ^ acc.z ^ acc.w;
}

// Read AND write: a memory spec quotes the BUS, and a copy exercises both
// directions. If 200 GB/s is a combined figure, this is where it shows.
kernel void copy_u32x4(
    device const uint4* src [[buffer(0)]],
    device uint4* dst [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = ELEMS / 4;
    for (uint i = gid; i < n; i += lanes) dst[i] = src[i];
}

// The shape a matvec fold actually has: every lane walks a CONTIGUOUS run of
// its own instead of striding with its neighbours. Same bytes, no coalescing
// between lanes.
kernel void stream_u32x4_blocked(
    device const uint4* src [[buffer(0)]],
    device uint* out [[buffer(1)]],
    uint gid [[thread_position_in_grid]],
    uint lanes [[threads_per_grid]]
) {
    uint n = ELEMS / 4;
    uint per = n / lanes;
    uint base = gid * per;
    uint4 acc = 0;
    for (uint i = 0; i < per; i++) acc ^= src[base + i];
    out[gid] = acc.x ^ acc.y ^ acc.z ^ acc.w;
}
"#;

#[test]
fn what_this_machine_can_actually_stream() {
    let Some(device) = MetalDevice::open() else {
        println!("SKIP: no Metal device");
        return;
    };
    let elems = BYTES / 4;
    let source = MSL
        .replace("HOPS", "2000u")
        .replace("WINELEMS", &format!("{}u", (128usize << 20) / 4))
        .replace("REPEATS", "16u")
        .replace("FILLELEMS", &format!("{}u", (2048usize << 20) / 16))
        .replace("BIGELEMS", &format!("{}u", BYTES * 4 / 4))
        .replace("ELEMS", &format!("{elems}u"));
    let pipes = device.compile(&source);

    let src = device.alloc_elems(elems, Dtype::F32);
    // Same bytes, GPU-private: no CPU-visible mapping, so the fabric owes it
    // no coherency. If shared storage is costing bandwidth, it shows here.
    let src_private = device.alloc_private_bytes(BYTES);

    let bandwidth_of = |kernel: &str, lanes: usize, buffer: &sanic::metal::MetalBuf| -> f64 {
        let out = device.alloc_elems(lanes, Dtype::F32);
        let dispatches = vec![Dispatch {
            pipe: pipes.get(kernel),
            inputs: vec![buffer.clone()],
            output: out,
            grid: lanes,
            argbuf: None,
        }];
        let graph = device.capture(&dispatches);
        // first replays fault the pages in; the floor is the steady state
        let seconds = (0..12)
            .filter_map(|_| device.run_graph_timed(&graph).ok())
            .fold(f64::INFINITY, f64::min);
        BYTES as f64 / seconds / 1e9
    };
    let bandwidth = |kernel: &str, lanes: usize| bandwidth_of(kernel, lanes, &src);

    println!("\n  reading {} MB per pass, min of 12\n", BYTES >> 20);
    println!(
        "  {:<26} {:>9} {:>9} {:>9} {:>9}",
        "lanes", "u32", "u32x2", "u32x4", "u4 dual"
    );
    let mut best = 0.0f64;
    for shift in [12, 14, 16, 18, 20, 21, 22] {
        let lanes = 1usize << shift;
        let row: Vec<f64> = ["stream_u32", "stream_u32x2", "stream_u32x4", "stream_u32x4_dual"]
            .iter()
            .map(|k| bandwidth(k, lanes))
            .collect();
        for &v in &row {
            best = best.max(v);
        }
        println!(
            "  {:<26} {:>9.1} {:>9.1} {:>9.1} {:>9.1}",
            format!("{lanes} (2^{shift})"),
            row[0],
            row[1],
            row[2],
            row[3]
        );
    }

    println!("\n  per-lane contiguous runs (the shape a matvec fold has):");
    for shift in [16, 18, 20, 22] {
        let lanes = 1usize << shift;
        println!(
            "  {:<26} {:>9.1}",
            format!("{lanes} (2^{shift})"),
            bandwidth("stream_u32x4_blocked", lanes)
        );
    }

    println!("\n  widening the search: stream depth, and threadgroup size");
    println!(
        "  {:<26} {:>9} {:>9} {:>9} {:>9}",
        "lanes", "deep4", "deep8", "tg=256", "tg=64"
    );
    for shift in [14, 16, 18, 20, 22] {
        let lanes = 1usize << shift;
        let row: Vec<f64> = ["stream_deep4", "stream_deep8", "stream_tg256", "stream_tg64"]
            .iter()
            .map(|k| bandwidth(k, lanes))
            .collect();
        for &v in &row {
            best = best.max(v);
        }
        println!(
            "  {:<26} {:>9.1} {:>9.1} {:>9.1} {:>9.1}",
            format!("{lanes} (2^{shift})"),
            row[0],
            row[1],
            row[2],
            row[3]
        );
    }

    println!("\n  SHARED vs PRIVATE storage — is coherency costing us bandwidth?");
    println!("  {:<26} {:>11} {:>11}", "lanes / kernel", "shared", "private");
    for (k, shift) in [
        ("stream_u32", 16),
        ("stream_u32", 20),
        ("stream_u32x4", 20),
        ("stream_u32x4", 22),
    ] {
        let lanes = 1usize << shift;
        let sh = bandwidth_of(k, lanes, &src);
        let pv = bandwidth_of(k, lanes, &src_private);
        best = best.max(pv);
        println!("  {:<26} {:>11.1} {:>11.1}", format!("{k} 2^{shift}"), sh, pv);
    }

    // Two things the sweep above never controlled: a fixed launch cost folded
    // into a 2.8 ms kernel, and DVFS — this GPU idles near 534 MHz and ramps
    // to ~1 GHz only under sustained load. Both bias a short benchmark DOWN.
    println!("\n  bigger buffer + sustained load (amortize launch, force the clock up)");
    {
        let big_bytes = BYTES * 4;
        let big = device.alloc_private_bytes(big_bytes);
        let lanes = 1usize << 16;
        let out = device.alloc_elems(lanes, Dtype::F32);
        let dispatches = vec![Dispatch {
            pipe: pipes.get("stream_u32_n"),
            inputs: vec![big.clone()],
            output: out,
            grid: lanes,
            argbuf: None,
        }];
        let graph = device.capture(&dispatches);
        // burn in hard first so the clock is at its ceiling, THEN measure
        for _ in 0..40 {
            let _ = device.run_graph_timed(&graph);
        }
        let seconds = (0..40)
            .filter_map(|_| device.run_graph_timed(&graph).ok())
            .fold(f64::INFINITY, f64::min);
        let gbs = big_bytes as f64 / seconds / 1e9;
        best = best.max(gbs);
        println!(
            "  {:>5} MB, 80 passes         {:>9.1} GB/s   ({:.2} ms/pass)",
            big_bytes >> 20,
            gbs,
            seconds * 1e3
        );
    }
    if let Some(watch) = device.clock()
        && let Some(clock) = watch.read()
    {
        println!("  GPU clock state right after the sustained run: {clock:?}");
    }

    // Where does size start to hurt? Our model streams 2.47 GB per token, so
    // if bandwidth falls off with working-set size the roofline that matters
    // is the one at OUR size, not at a convenient one.
    println!("\n  bandwidth vs working-set size (private, 2^16 lanes, sustained)");
    // DESCENDING, and each buffer is dropped before the next is taken: an
    // ascending sweep that leaks its buffers reaches the 2 GB row holding 4 GB
    // live, and would blame size for what is really memory pressure.
    for mb in [2048usize, 1024, 512, 256, 128, 64] {
        let bytes = mb << 20;
        let buf = device.alloc_private_bytes(bytes);
        let lanes = 1usize << 16;
        let out = device.alloc_elems(lanes, Dtype::F32);
        let kernel = format!("stream_sz{mb}");
        let dispatches = vec![Dispatch {
            pipe: pipes.get(&kernel),
            inputs: vec![buf.clone()],
            output: out,
            grid: lanes,
            argbuf: None,
        }];
        // make the pages real first, or the read is measuring nothing
        let fill = vec![Dispatch {
            pipe: pipes.get("fill_all"),
            inputs: vec![buf.clone()],
            output: device.alloc_elems(4, Dtype::F32),
            grid: bytes / 16,
            argbuf: None,
        }];
        device.run(&fill);
        let graph = device.capture(&dispatches);
        for _ in 0..10 {
            let _ = device.run_graph_timed(&graph);
        }
        let seconds = (0..20)
            .filter_map(|_| device.run_graph_timed(&graph).ok())
            .fold(f64::INFINITY, f64::min);
        println!("  {:>5} MB   {:>9.1} GB/s", mb, bytes as f64 / seconds / 1e9);
        drop(graph);
        drop(buf);
    }

    // The decisive test for the falling limb. Same 2 GB of bytes either way;
    // the only difference is how many pages are in flight AT ONCE. A
    // grid-stride over the whole buffer has 65k lanes each landing on its own
    // page — 65k pages live. Walking the same bytes in windows keeps the live
    // page set to one window. If bandwidth comes back, the limit is
    // translation reach and not the DRAM.
    println!("\n  2 GB traversed whole vs in windows (same bytes, same lanes)");
    {
        let total = 2048usize << 20;
        let buf = device.alloc_private_bytes(total);
        let out = device.alloc_elems(1 << 16, Dtype::F32);
        device.run(&[Dispatch {
            pipe: pipes.get("fill_all"),
            inputs: vec![buf.clone()],
            output: device.alloc_elems(4, Dtype::F32),
            grid: total / 16,
            argbuf: None,
        }]);
        for window_mb in [2048usize, 512, 128, 64] {
            let window = window_mb << 20;
            let passes: Vec<Dispatch> = (0..total / window)
                .map(|w| Dispatch {
                    pipe: pipes.get(&format!("stream_sz{window_mb}")),
                    inputs: vec![buf.slice(w * window)],
                    output: out.clone(),
                    grid: 1 << 16,
                    argbuf: None,
                })
                .collect();
            let graph = device.capture(&passes);
            for _ in 0..5 {
                let _ = device.run_graph_timed(&graph);
            }
            let seconds = (0..15)
                .filter_map(|_| device.run_graph_timed(&graph).ok())
                .fold(f64::INFINITY, f64::min);
            println!(
                "  window {:>5} MB x{:<3}   {:>9.1} GB/s",
                window_mb,
                total / window,
                total as f64 / seconds / 1e9
            );
        }
    }

    // Joint sweep. Windowing and per-lane width were only ever varied one at
    // a time — the window test used scalar reads, the width test strided the
    // whole buffer. This crosses them, over a 2 GB total in 128 MB windows.
    println!("\n  JOINT: 2 GB in 128 MB windows x width x lanes");
    println!("  {:<12} {:>9} {:>9} {:>9}", "lanes", "u32", "u32x2", "u32x4");
    {
        let total = 2048usize << 20;
        let window = 128usize << 20;
        let buf = device.alloc_private_bytes(total);
        device.run(&[Dispatch {
            pipe: pipes.get("fill_all"),
            inputs: vec![buf.clone()],
            output: device.alloc_elems(4, Dtype::F32),
            grid: total / 16,
            argbuf: None,
        }]);
        for shift in [13usize, 14, 15, 16, 17, 18] {
            let lanes = 1usize << shift;
            let out = device.alloc_elems(lanes, Dtype::F32);
            let row: Vec<f64> = ["win_u32", "win_u32x2", "win_u32x4"]
                .iter()
                .map(|k| {
                    let passes: Vec<Dispatch> = (0..total / window)
                        .map(|w| Dispatch {
                            pipe: pipes.get(k),
                            inputs: vec![buf.slice(w * window)],
                            output: out.clone(),
                            grid: lanes,
                            argbuf: None,
                        })
                        .collect();
                    let graph = device.capture(&passes);
                    for _ in 0..4 {
                        let _ = device.run_graph_timed(&graph);
                    }
                    let secs = (0..15)
                        .filter_map(|_| device.run_graph_timed(&graph).ok())
                        .fold(f64::INFINITY, f64::min);
                    total as f64 / secs / 1e9
                })
                .collect();
            for &v in &row {
                best = best.max(v);
            }
            println!(
                "  {:<12} {:>9.1} {:>9.1} {:>9.1}",
                format!("2^{shift}"),
                row[0],
                row[1],
                row[2]
            );
        }
    }

    println!("\n  how much of every number above is fixed launch cost?");
    {
        let win = 128usize << 20;
        let buf = device.alloc_private_bytes(win);
        device.run(&[Dispatch {
            pipe: pipes.get("fill_all"),
            inputs: vec![buf.clone()],
            output: device.alloc_elems(4, Dtype::F32),
            grid: win / 16,
            argbuf: None,
        }]);
        let lanes = 1usize << 16;
        let out = device.alloc_elems(lanes, Dtype::F32);
        let time = |kernel: &str, grid: usize| -> f64 {
            let g = device.capture(&[Dispatch {
                pipe: pipes.get(kernel),
                inputs: vec![buf.clone()],
                output: out.clone(),
                grid,
                argbuf: None,
            }]);
            for _ in 0..8 {
                let _ = device.run_graph_timed(&g);
            }
            (0..30)
                .filter_map(|_| device.run_graph_timed(&g).ok())
                .fold(f64::INFINITY, f64::min)
        };
        let empty = time("nothing", 32);
        let once = time("stream_sz128", lanes);
        let rep = time("win_repeat", lanes);
        let bytes = win as f64;
        println!(
            "  empty dispatch                {:>9.1} us   <- fixed cost",
            empty * 1e6
        );
        println!(
            "  128 MB once                   {:>9.1} us   {:>6.1} GB/s   (minus fixed: {:.1})",
            once * 1e6,
            bytes / once / 1e9,
            bytes / (once - empty) / 1e9
        );
        println!(
            "  128 MB x16 in one dispatch    {:>9.1} us   {:>6.1} GB/s   <- fixed cost amortised 16x",
            rep * 1e6,
            16.0 * bytes / rep / 1e9
        );
        best = best.max(16.0 * bytes / rep / 1e9);
    }

    // Can ONE kernel saturate the fabric? Independent dispatches over separate
    // buffers, no barrier between them, so they overlap. If N concurrent
    // streams beat one, a single client was never the ceiling.
    println!("\n  concurrent independent streams (128 MB each, no barriers)");
    {
        let win = 128usize << 20;
        let lanes = 1usize << 16;
        let bufs: Vec<_> = (0..8).map(|_| device.alloc_private_bytes(win)).collect();
        for b in &bufs {
            device.run(&[Dispatch {
                pipe: pipes.get("fill_all"),
                inputs: vec![b.clone()],
                output: device.alloc_elems(4, Dtype::F32),
                grid: win / 16,
                argbuf: None,
            }]);
        }
        for n in [1usize, 2, 4, 8] {
            let passes: Vec<Dispatch> = (0..n)
                .map(|i| Dispatch {
                    pipe: pipes.get("stream_sz128"),
                    inputs: vec![bufs[i].clone()],
                    output: device.alloc_elems(lanes, Dtype::F32),
                    grid: lanes,
                    argbuf: None,
                })
                .collect();
            let graph = device.capture(&passes);
            for _ in 0..6 {
                let _ = device.run_graph_timed(&graph);
            }
            let secs = (0..20)
                .filter_map(|_| device.run_graph_timed(&graph).ok())
                .fold(f64::INFINITY, f64::min);
            let gbs = (n * win) as f64 / secs / 1e9;
            best = best.max(gbs);
            println!("  {n} concurrent      {:>9.1} GB/s", gbs);
        }
    }

    // Last question: is 185 the MEMORY's limit or the GPU CLIENT's? If the
    // 200 GB/s figure is fabric capacity shared by CPU, GPU and ANE, then one
    // client saturating at 185 is expected, and CPU traffic running alongside
    // should ADD rather than steal.
    println!("\n  is the ceiling the memory, or one client? (CPU streams alongside the GPU)");
    {
        use std::sync::Arc as StdArc;
        use std::sync::atomic::{AtomicBool, Ordering};
        let win = 128usize << 20;
        let buf = device.alloc_private_bytes(win);
        device.run(&[Dispatch {
            pipe: pipes.get("fill_all"),
            inputs: vec![buf.clone()],
            output: device.alloc_elems(4, Dtype::F32),
            grid: win / 16,
            argbuf: None,
        }]);
        let lanes = 1usize << 16;
        let out = device.alloc_elems(lanes, Dtype::F32);
        let graph = device.capture(&[Dispatch {
            pipe: pipes.get("stream_sz128"),
            inputs: vec![buf.clone()],
            output: out,
            grid: lanes,
            argbuf: None,
        }]);
        let gpu_only = {
            for _ in 0..6 {
                let _ = device.run_graph_timed(&graph);
            }
            let s = (0..20)
                .filter_map(|_| device.run_graph_timed(&graph).ok())
                .fold(f64::INFINITY, f64::min);
            win as f64 / s / 1e9
        };

        // CPU side: a few threads summing their own large slabs.
        let stop = StdArc::new(AtomicBool::new(false));
        let counter = StdArc::new(std::sync::atomic::AtomicU64::new(0));
        let threads: Vec<_> = (0..6)
            .map(|_| {
                let stop = stop.clone();
                let counter = counter.clone();
                std::thread::spawn(move || {
                    // 384 MB per thread, far past any cache, so this is DRAM
                    // traffic and not an L2/SLC replay.
                    let slab: Vec<u64> = vec![1; (384usize << 20) / 8];
                    while !stop.load(Ordering::Relaxed) {
                        let mut acc = 0u64;
                        for &v in &slab {
                            acc ^= v;
                        }
                        std::hint::black_box(acc);
                        counter.fetch_add(slab.len() as u64 * 8, Ordering::Relaxed);
                    }
                })
            })
            .collect();
        std::thread::sleep(std::time::Duration::from_millis(300));
        let cpu_start = counter.load(Ordering::Relaxed);
        let wall = std::time::Instant::now();
        let gpu_with_cpu = {
            let s = (0..20)
                .filter_map(|_| device.run_graph_timed(&graph).ok())
                .fold(f64::INFINITY, f64::min);
            win as f64 / s / 1e9
        };
        let elapsed = wall.elapsed().as_secs_f64();
        let cpu_bytes = counter.load(Ordering::Relaxed) - cpu_start;
        stop.store(true, Ordering::Relaxed);
        for t in threads {
            let _ = t.join();
        }
        let cpu_gbs = cpu_bytes as f64 / elapsed / 1e9;
        println!("  GPU alone                    {gpu_only:>9.1} GB/s");
        println!("  GPU while CPU streams        {gpu_with_cpu:>9.1} GB/s");
        println!("  CPU concurrently             {cpu_gbs:>9.1} GB/s");
        println!("  COMBINED                     {:>9.1} GB/s", gpu_with_cpu + cpu_gbs);
    }

    // The regime an ML kernel actually runs in. Two things have to be right:
    // the read must be ML-SIZED (a llama weight is 2-33 MB, every number above
    // is >=64 MB), and it must be COLD. Re-reading one small buffer measures
    // the 24 MB system cache — 16 MB reads back at 332 GB/s that way, above
    // anything the DRAM can do. So each size gets enough DISTINCT buffers to
    // total ~256 MB, read once each, exactly as a decode step reads its
    // weights once per token.
    println!("\n  ML-SIZED COLD reads: effective GB/s vs bytes per kernel");
    println!(
        "  {:>7} {:>7} {:>9} {:>9} {:>9}  what reads this",
        "MB", "count", "us each", "GB/s", "% of 188"
    );
    for kb in [512usize, 1024, 2048, 4096, 8192, 16384, 33792, 65536] {
        let bytes = kb * 1024;
        let count = ((256usize << 20) / bytes).max(4);
        let bufs: Vec<_> = (0..count).map(|_| device.alloc_private_bytes(bytes)).collect();
        for b in &bufs {
            device.run(&[Dispatch {
                pipe: pipes.get("fill_all"),
                inputs: vec![b.clone()],
                output: device.alloc_elems(4, Dtype::F32),
                grid: bytes / 16,
                argbuf: None,
            }]);
        }
        let lanes = (1usize << 16).min(bytes / 4);
        let out = device.alloc_elems(lanes, Dtype::F32);
        let passes: Vec<Dispatch> = bufs
            .iter()
            .map(|b| Dispatch {
                pipe: pipes.get(&format!("ml_{kb}k")),
                inputs: vec![b.clone()],
                output: out.clone(),
                grid: lanes,
                argbuf: None,
            })
            .collect();
        let g = device.capture(&passes);
        for _ in 0..3 {
            let _ = device.run_graph_timed(&g);
        }
        let secs = (0..12)
            .filter_map(|_| device.run_graph_timed(&g).ok())
            .fold(f64::INFINITY, f64::min);
        let total = (bytes * count) as f64;
        let gbs = total / secs / 1e9;
        let mb = kb as f64 / 1024.0;
        let what = if mb < 1.0 {
            ""
        } else if mb < 3.0 {
            "k/v proj"
        } else if mb < 9.0 {
            "q proj, o proj"
        } else if mb < 34.0 {
            "gate/up, down"
        } else {
            ""
        };
        println!(
            "  {:>7.1} {:>7} {:>9.1} {:>9.1} {:>8.0}%  {}",
            mb,
            count,
            secs * 1e6 / count as f64,
            gbs,
            gbs / 188.0 * 100.0,
            what
        );
    }

    // Why can we not read faster? Bandwidth = outstanding bytes / latency.
    // Measure the latency, then read off how much has to be in flight.
    println!("\n  DRAM latency, and what it demands");
    {
        let bytes = 256usize << 20;
        let elems = bytes / 4;
        let stride = 8192usize / 4; // 8 KB apart: new page every hop
        let mut chain: Vec<u32> = vec![0; elems];
        let mut at = 0usize;
        for _ in 0..elems {
            let next = (at + stride) % elems;
            chain[at] = next as u32;
            at = next;
        }
        let buf = device.from_bytes(unsafe { std::slice::from_raw_parts(chain.as_ptr() as *const u8, bytes) });
        let out = device.alloc_elems(64, Dtype::F32);
        let g = device.capture(&[Dispatch {
            pipe: pipes.get("chase"),
            inputs: vec![buf.clone()],
            output: out,
            grid: 1,
            argbuf: None,
        }]);
        for _ in 0..5 {
            let _ = device.run_graph_timed(&g);
        }
        let secs = (0..20)
            .filter_map(|_| device.run_graph_timed(&g).ok())
            .fold(f64::INFINITY, f64::min);
        let latency_ns = (secs - 3.9e-6).max(0.0) / 2000.0 * 1e9;
        println!("  dependent load latency        {latency_ns:>8.0} ns");
        for bw in [179.7f64, 188.0, 204.8] {
            let outstanding = bw * 1e9 * latency_ns * 1e-9;
            println!(
                "  to sustain {bw:>5.1} GB/s          {:>8.0} KB in flight = {:>6.0} lines of 128 B",
                outstanding / 1024.0,
                outstanding / 128.0
            );
        }
        println!("  the machine has 14 cores, so that is per-core:");
        for bw in [179.7f64, 204.8] {
            let outstanding = bw * 1e9 * latency_ns * 1e-9;
            println!(
                "  {bw:>5.1} GB/s -> {:>5.0} outstanding 128 B lines per core",
                outstanding / 128.0 / 14.0
            );
        }
    }

    // Multiple GPU CLIENTS, not multiple dispatches. Independent command
    // queues are separate submission contexts; dispatches inside one command
    // buffer (tested above, no gain) are not the same question.
    println!("\n  independent command QUEUES streaming at once");
    {
        let win = 128usize << 20;
        let buf = device.alloc_private_bytes(win);
        device.run(&[Dispatch {
            pipe: pipes.get("fill_all"),
            inputs: vec![buf.clone()],
            output: device.alloc_elems(4, Dtype::F32),
            grid: win / 16,
            argbuf: None,
        }]);
        let lanes = 1usize << 16;
        let g = device.capture(&[Dispatch {
            pipe: pipes.get("stream_sz128"),
            inputs: vec![buf.clone()],
            output: device.alloc_elems(lanes, Dtype::F32),
            grid: lanes,
            argbuf: None,
        }]);
        for queues in [1usize, 2, 4, 8] {
            for _ in 0..4 {
                let _ = device.replay_on_parallel_queues(&g, queues);
            }
            let secs = (0..12)
                .map(|_| device.replay_on_parallel_queues(&g, queues))
                .fold(f64::INFINITY, f64::min);
            let total = (win * queues) as f64;
            let gbs = total / secs / 1e9;
            best = best.max(gbs);
            println!(
                "  {queues} queue(s)   {:>8.1} GB/s combined   ({:>6.1} each)",
                gbs,
                gbs / queues as f64
            );
        }
    }

    println!("\n  copy: read + write, counting BOTH directions");
    let mut best_copy = 0.0f64;
    for shift in [16, 18, 20, 22] {
        let lanes = 1usize << shift;
        let dst = device.alloc_elems(elems, Dtype::F32);
        let dispatches = vec![Dispatch {
            pipe: pipes.get("copy_u32x4"),
            inputs: vec![src.clone()],
            output: dst,
            grid: lanes,
            argbuf: None,
        }];
        let graph = device.capture(&dispatches);
        let seconds = (0..12)
            .filter_map(|_| device.run_graph_timed(&graph).ok())
            .fold(f64::INFINITY, f64::min);
        let combined = 2.0 * BYTES as f64 / seconds / 1e9;
        best_copy = best_copy.max(combined);
        println!(
            "  {:<26} {:>9.1} GB/s combined",
            format!("{lanes} (2^{shift})"),
            combined
        );
    }

    println!(
        "\n  BEST SUSTAINED READ:  {best:.1} GB/s ({:.0}% of the 200 GB/s spec)",
        best / 2.0
    );
    println!(
        "  BEST COPY (r+w):      {best_copy:.1} GB/s ({:.0}% of spec)",
        best_copy / 2.0
    );
}

/// Does bandwidth scale with CORES? If the ceiling is a per-core limit on
/// outstanding cache-line requests — 219 ns latency and ~22 lines per core is
/// what 180 GB/s needs — then below one threadgroup per core this is linear,
/// and it flattens exactly when every core is busy. If instead it is flat from
/// the start, or keeps climbing past 14, the per-core model is wrong.
#[test]
fn bandwidth_scales_with_cores() {
    let Some(device) = MetalDevice::open() else {
        println!("SKIP: no Metal device");
        return;
    };
    let win = 128usize << 20;
    let source = MSL
        .replace("HOPS", "2000u")
        .replace("WINELEMS", &format!("{}u", win / 4))
        .replace("REPEATS", "16u")
        .replace("FILLELEMS", &format!("{}u", win / 16))
        .replace("BIGELEMS", &format!("{}u", win / 4))
        .replace("ELEMS", &format!("{}u", win / 4));
    let pipes = device.compile(&source);
    let buf = device.alloc_private_bytes(win);
    device.run(&[Dispatch {
        pipe: pipes.get("fill_all"),
        inputs: vec![buf.clone()],
        output: device.alloc_elems(4, Dtype::F32),
        grid: win / 16,
        argbuf: None,
    }]);

    println!("\n  threadgroups (1024 threads each) on a 14-core GPU\n");
    println!("  {:>4}  {:>10}  {:>12}", "tgs", "GB/s", "per group");
    let mut previous = 0.0f64;
    for tgs in [1usize, 2, 4, 7, 14, 28, 64] {
        let grid = tgs * 1024;
        let out = device.alloc_elems(grid, Dtype::F32);
        let g = device.capture(&[Dispatch {
            pipe: pipes.get("per_core"),
            inputs: vec![buf.clone()],
            output: out,
            grid,
            argbuf: None,
        }]);
        for _ in 0..6 {
            let _ = device.run_graph_timed(&g);
        }
        let secs = (0..20)
            .filter_map(|_| device.run_graph_timed(&g).ok())
            .fold(f64::INFINITY, f64::min);
        let gbs = win as f64 / secs / 1e9;
        let marker = if tgs > 1 && gbs < previous * 1.15 {
            "  <- flattening"
        } else {
            ""
        };
        println!("  {tgs:>4}  {gbs:>10.1}  {:>12.1}{marker}", gbs / tgs as f64);
        previous = gbs;
    }
}

/// Do the buffer and texture paths have SEPARATE miss queues? A buffer-only
/// kernel tops out at ~23 outstanding lines per core. Apple GPUs serve
/// textures through their own cache, so a kernel driving both at once could
/// hold more in flight than either alone — the one route past the ceiling that
/// nothing else in this file has ruled out.
#[test]
fn buffer_and_texture_paths_together() {
    let Some(device) = MetalDevice::open() else {
        println!("SKIP: no Metal device");
        return;
    };
    let win = 128usize << 20;
    let source = MSL
        .replace("HOPS", "2000u")
        .replace("WINELEMS", &format!("{}u", win / 4))
        .replace("REPEATS", "16u")
        .replace("FILLELEMS", &format!("{}u", win / 16))
        .replace("BIGELEMS", &format!("{}u", win / 4))
        .replace("ELEMS", &format!("{}u", win / 4));
    let pipes = device.compile(&source);
    let buf = device.alloc_bytes(win);
    device.run(&[Dispatch {
        pipe: pipes.get("fill_all"),
        inputs: vec![buf.clone()],
        output: device.alloc_elems(4, Dtype::F32),
        grid: win / 16,
        argbuf: None,
    }]);
    let tex = device.texture_view(&buf, win / 16);
    let lanes = 1usize << 16;
    let out = device.alloc_elems(lanes, Dtype::F32);

    println!("\n  same 128 MB, by path\n");
    for (name, kernel, with_tex) in [
        ("buffer only", "path_buffer", false),
        ("texture only", "path_texture", true),
        ("BOTH at once", "path_both", true),
    ] {
        let pipe = pipes.get(kernel);
        let t = device.time_with_texture(
            &pipe,
            std::slice::from_ref(&buf),
            if with_tex { Some(&tex) } else { None },
            &out,
            lanes,
            25,
        );
        println!("  {name:<14} {:>8.1} GB/s", win as f64 / t / 1e9);
    }
}
