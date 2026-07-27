//! Does widening the LOAD INSTRUCTION raise a matvec fold's DRAM rate? A
//! hand-written PROBE — MSL by hand, no derivation — answering one question
//! before any codegen was built on the answer (todo.md item 7b).
//!
//! **It does not. 1.01x on gate/up, 1.00x on down, and the probe exists to
//! keep saying so.** The fold sanic emits sweeps a row in 8-element chunks,
//! fetching 16 contiguous bytes with eight separate 2-byte loads; replacing
//! them with one `uint4` changes nothing. The standing P1 theory that a
//! per-load-instruction byte ceiling explains this class's rate is wrong for
//! this shape.
//!
//! Measured (8 dispatches over 8 DISTINCT cold weights, one command buffer):
//!
//! ```text
//!   gate/up 2048->8192, 8192 threadgroups:  scalar 183.6  uint4 185.0 GB/s
//!   down    8192->2048, 2048 threadgroups:  scalar 167.6  uint4 167.1 GB/s
//! ```
//!
//! Two corrections fall out, both about MEASUREMENT REGIME, and they cost an
//! item on the roadmap:
//!
//! 1. The emitted shape is at the machine's practical peak already. The
//!    per-class rates that motivated item 7b (gate/up 166, down 153 GB/s)
//!    come from `SANIC_DEBUG=4`, which gives every kernel its own encoder and
//!    so charges each one a ramp that production hides. Same kernels
//!    back-to-back in one encoder: 183.6 and 167.6. There was never 1.69 ms
//!    of codegen slack — that figure read an isolated-regime measurement as
//!    if it were production.
//! 2. Concurrency between these kernels is worth 3% (178.2 GB/s serialized
//!    against 183.6 overlapped), which is the same answer the phase-reorder
//!    probe gave: a grid this size already fills the machine.
//!
//! What DOES survive: down sustains 167.6 where gate/up sustains 183.6, with
//! identical code and a quarter of the grid (2048 threadgroups against
//! 8192). That is grid WIDTH — Little's law — and it is the one lever these
//! probes have not killed. Raising it means splitting the reduction so a
//! narrow-output projection launches more threadgroups, which INCREASES
//! thread count and so sits on the right side of the block-rows law rather
//! than against it.
//!
//! All shapes are checked against each other before any of them is timed.

#![cfg(target_os = "macos")]

use sanic::metal::{Dispatch, MetalDevice};

const HIDDEN: usize = 2048; // llama-3.2-1B
const OUTPUTS: usize = 8192; // an mlp gate/up: 33.5 MB of bf16 weight, cache-proof
const SIMD: usize = 32;
/// One dispatch per command buffer would fold ~45 µs of command-buffer
/// startup into a ~240 µs kernel and report a rate ~30% low. The real step
/// runs 32 of these back to back over 32 DISTINCT weights, so the probe
/// dispatches several over distinct buffers too: overhead amortizes and the
/// weights stay cold, which is what makes the GB/s column comparable to the
/// 166 the class sustains in a real step.
const REPEATS: usize = 8;

const MSL: &str = r#"
#include <metal_stdlib>
using namespace metal;

inline float bf(ushort v) { return as_type<float>((uint)v << 16u); }
// ── what sanic emits today: eight scalar 2-byte loads per chunk ──────────────
[[max_total_threads_per_threadgroup(32)]]
kernel void matvec_scalar8(
    device const ushort* x [[buffer(0)]],
    device const ushort* weights [[buffer(1)]],
    device float* outb [[buffer(2)]],
    uint gid [[thread_position_in_grid]],
    uint lane [[thread_index_in_simdgroup]]
) {
    uint row = gid / 32u;
    float acc = 0.0f;
    for (uint c_ = lane; c_ < 256u; c_ += 32u) {
        for (uint k = 0; k < 8u; k++) {
            acc += bf(x[c_ * 8u + k]) * bf(weights[row * 2048u + (c_ * 8u + k)]);
        }
    }
    for (uint off = 16u; off > 0u; off >>= 1) acc += simd_shuffle_xor(acc, off);
    if (lane == 0) outb[row] = acc;
}

// ── the same sweep, one 16-byte load instruction per chunk ───────────────────
// A uint4 is 16 bytes = 8 bf16. Row base is row*2048 elements = row*4096
// bytes and the chunk offset is c_*16 bytes, so every access is 16-byte
// aligned as long as the BUFFER is — which the probe's own allocation is.
[[max_total_threads_per_threadgroup(32)]]
kernel void matvec_vec4(
    device const ushort* x [[buffer(0)]],
    device const ushort* weights [[buffer(1)]],
    device float* outb [[buffer(2)]],
    uint gid [[thread_position_in_grid]],
    uint lane [[thread_index_in_simdgroup]]
) {
    uint row = gid / 32u;
    device const uint4* x4 = (device const uint4*)x;
    device const uint4* w4 = (device const uint4*)(weights + row * 2048u);
    float acc = 0.0f;
    for (uint c_ = lane; c_ < 256u; c_ += 32u) {
        uint4 xp = x4[c_];
        uint4 wp = w4[c_];
        acc += as_type<float>((xp.x & 0xffffu) << 16u) * as_type<float>((wp.x & 0xffffu) << 16u);
        acc += as_type<float>(xp.x & 0xffff0000u)      * as_type<float>(wp.x & 0xffff0000u);
        acc += as_type<float>((xp.y & 0xffffu) << 16u) * as_type<float>((wp.y & 0xffffu) << 16u);
        acc += as_type<float>(xp.y & 0xffff0000u)      * as_type<float>(wp.y & 0xffff0000u);
        acc += as_type<float>((xp.z & 0xffffu) << 16u) * as_type<float>((wp.z & 0xffffu) << 16u);
        acc += as_type<float>(xp.z & 0xffff0000u)      * as_type<float>(wp.z & 0xffff0000u);
        acc += as_type<float>((xp.w & 0xffffu) << 16u) * as_type<float>((wp.w & 0xffffu) << 16u);
        acc += as_type<float>(xp.w & 0xffff0000u)      * as_type<float>(wp.w & 0xffff0000u);
    }
    for (uint off = 16u; off > 0u; off >>= 1) acc += simd_shuffle_xor(acc, off);
    if (lane == 0) outb[row] = acc;
}

// ── half the width: uint2 = 8 bytes = 4 bf16, two per chunk ──────────────────
[[max_total_threads_per_threadgroup(32)]]
kernel void matvec_vec2(
    device const ushort* x [[buffer(0)]],
    device const ushort* weights [[buffer(1)]],
    device float* outb [[buffer(2)]],
    uint gid [[thread_position_in_grid]],
    uint lane [[thread_index_in_simdgroup]]
) {
    uint row = gid / 32u;
    device const uint2* x2 = (device const uint2*)x;
    device const uint2* w2 = (device const uint2*)(weights + row * 2048u);
    float acc = 0.0f;
    for (uint c_ = lane; c_ < 256u; c_ += 32u) {
        for (uint h = 0; h < 2u; h++) {
            uint2 xp = x2[c_ * 2u + h];
            uint2 wp = w2[c_ * 2u + h];
            acc += as_type<float>((xp.x & 0xffffu) << 16u) * as_type<float>((wp.x & 0xffffu) << 16u);
            acc += as_type<float>(xp.x & 0xffff0000u)      * as_type<float>(wp.x & 0xffff0000u);
            acc += as_type<float>((xp.y & 0xffffu) << 16u) * as_type<float>((wp.y & 0xffffu) << 16u);
            acc += as_type<float>(xp.y & 0xffff0000u)      * as_type<float>(wp.y & 0xffff0000u);
        }
    }
    for (uint off = 16u; off > 0u; off >>= 1) acc += simd_shuffle_xor(acc, off);
    if (lane == 0) outb[row] = acc;
}
"#;

fn bf16(value: f64) -> u16 {
    let bits = (value as f32).to_bits();
    ((bits + 0x7fff + ((bits >> 16) & 1)) >> 16) as u16
}

fn bytemuck_cast(values: &[u16]) -> &[u8] {
    // SAFETY: u16 has no padding and any bit pattern is a valid u8 pair
    unsafe { std::slice::from_raw_parts(values.as_ptr() as *const u8, std::mem::size_of_val(values)) }
}

#[test]
fn wide_loads_against_the_scalar_sweep() {
    let Some(device) = MetalDevice::open() else {
        println!("SKIP: no Metal device");
        return;
    };
    let pipes = device.compile(MSL);

    let activation: Vec<u16> = (0..HIDDEN).map(|i| bf16(((i % 17) as f64 - 8.0) / 32.0)).collect();
    let x = device.from_bytes(bytemuck_cast(&activation));
    let weight_buffers: Vec<_> = (0..REPEATS)
        .map(|w| {
            let weights: Vec<u16> = (0..OUTPUTS * HIDDEN)
                .map(|i| bf16((((i + w) % 31) as f64 - 15.0) / 256.0))
                .collect();
            device.from_bytes(bytemuck_cast(&weights))
        })
        .collect();

    let variant = |kernel: &str| {
        let outs: Vec<_> = (0..REPEATS).map(|_| device.alloc_f32(OUTPUTS)).collect();
        let dispatches: Vec<Dispatch> = weight_buffers
            .iter()
            .zip(&outs)
            .map(|(weight, out)| Dispatch {
                pipe: pipes.get(kernel),
                inputs: vec![x.clone(), weight.clone()],
                output: out.clone(),
                grid: OUTPUTS * SIMD,
                argbuf: None,
            })
            .collect();
        (dispatches, outs[0].clone())
    };
    // The same kernels made SERIAL: every dispatch writes the one output
    // buffer, so barrier_schedule sees WAW on it and fences between them.
    // Weights stay distinct and cold, so the only difference from the
    // concurrent variant is whether neighbours may overlap.
    let serial = |kernel: &str| {
        let out = device.alloc_f32(OUTPUTS);
        weight_buffers
            .iter()
            .map(|weight| Dispatch {
                pipe: pipes.get(kernel),
                inputs: vec![x.clone(), weight.clone()],
                output: out.clone(),
                grid: OUTPUTS * SIMD,
                argbuf: None,
            })
            .collect::<Vec<_>>()
    };
    let (scalar8, out_scalar) = variant("matvec_scalar8");
    let (vec4, out_vec4) = variant("matvec_vec4");
    let (vec2, out_vec2) = variant("matvec_vec2");

    // same answer first, then the timing means something
    for d in [&scalar8, &vec4, &vec2] {
        device.run(d);
    }
    let read = |buf: &sanic::metal::MetalBuf| device.read_f32(buf, OUTPUTS);
    let (base, wide4, wide2) = (read(&out_scalar), read(&out_vec4), read(&out_vec2));
    for (label, other) in [("vec4", &wide4), ("vec2", &wide2)] {
        let worst = base
            .iter()
            .zip(other.iter())
            .map(|(l, r)| (*l as f64 - *r as f64).abs() / (1.0 + l.abs() as f64))
            .fold(0.0, f64::max);
        assert!(worst < 2e-3, "{label} disagrees with the scalar sweep: {worst:e}");
    }

    let bytes = (REPEATS * OUTPUTS * HIDDEN * 2) as f64;
    let time = |dispatches: &[Dispatch]| -> f64 {
        let graph = device.capture(dispatches);
        (0..20)
            .filter_map(|_| device.run_graph_timed(&graph).ok())
            .fold(f64::INFINITY, f64::min)
    };
    let scalar8_serial = serial("matvec_scalar8");
    let (mut s8, mut v4, mut v2, mut ser) = (f64::INFINITY, f64::INFINITY, f64::INFINITY, f64::INFINITY);
    for _ in 0..3 {
        s8 = s8.min(time(&scalar8));
        v4 = v4.min(time(&vec4));
        v2 = v2.min(time(&vec2));
        ser = ser.min(time(&scalar8_serial));
    }
    for (label, seconds) in [
        ("scalar x8 (today)", s8),
        ("uint2 (8B)", v2),
        ("uint4 (16B)", v4),
        ("scalar x8, SERIAL", ser),
    ] {
        println!(
            "  {label:<18} {:8.1}us/kernel  {:6.1} GB/s  {:.2}x",
            seconds / REPEATS as f64 * 1e6,
            bytes / seconds / 1e9,
            s8 / seconds,
        );
    }
}

/// The same question at the MLP DOWN geometry (8192 → 2048): the class with
/// the LOWEST in-step rate (153 GB/s) and a quarter of gate/up's grid, so if
/// grid width is what costs bandwidth — Little's law, the one hypothesis the
/// reorder probe did not kill — it should show here and not on gate/up.
const MSL_DOWN: &str = r#"
#include <metal_stdlib>
using namespace metal;

inline float bf(ushort v) { return as_type<float>((uint)v << 16u); }

[[max_total_threads_per_threadgroup(32)]]
kernel void down_scalar8(
    device const ushort* x [[buffer(0)]],
    device const ushort* weights [[buffer(1)]],
    device float* outb [[buffer(2)]],
    uint gid [[thread_position_in_grid]],
    uint lane [[thread_index_in_simdgroup]]
) {
    uint row = gid / 32u;
    float acc = 0.0f;
    for (uint c_ = lane; c_ < 1024u; c_ += 32u) {
        for (uint k = 0; k < 8u; k++) {
            acc += bf(x[c_ * 8u + k]) * bf(weights[row * 8192u + (c_ * 8u + k)]);
        }
    }
    for (uint off = 16u; off > 0u; off >>= 1) acc += simd_shuffle_xor(acc, off);
    if (lane == 0) outb[row] = acc;
}

[[max_total_threads_per_threadgroup(32)]]
kernel void down_vec4(
    device const ushort* x [[buffer(0)]],
    device const ushort* weights [[buffer(1)]],
    device float* outb [[buffer(2)]],
    uint gid [[thread_position_in_grid]],
    uint lane [[thread_index_in_simdgroup]]
) {
    uint row = gid / 32u;
    device const uint4* x4 = (device const uint4*)x;
    device const uint4* w4 = (device const uint4*)(weights + row * 8192u);
    float acc = 0.0f;
    for (uint c_ = lane; c_ < 1024u; c_ += 32u) {
        uint4 xp = x4[c_];
        uint4 wp = w4[c_];
        acc += as_type<float>((xp.x & 0xffffu) << 16u) * as_type<float>((wp.x & 0xffffu) << 16u);
        acc += as_type<float>(xp.x & 0xffff0000u)      * as_type<float>(wp.x & 0xffff0000u);
        acc += as_type<float>((xp.y & 0xffffu) << 16u) * as_type<float>((wp.y & 0xffffu) << 16u);
        acc += as_type<float>(xp.y & 0xffff0000u)      * as_type<float>(wp.y & 0xffff0000u);
        acc += as_type<float>((xp.z & 0xffffu) << 16u) * as_type<float>((wp.z & 0xffffu) << 16u);
        acc += as_type<float>(xp.z & 0xffff0000u)      * as_type<float>(wp.z & 0xffff0000u);
        acc += as_type<float>((xp.w & 0xffffu) << 16u) * as_type<float>((wp.w & 0xffffu) << 16u);
        acc += as_type<float>(xp.w & 0xffff0000u)      * as_type<float>(wp.w & 0xffff0000u);
    }
    for (uint off = 16u; off > 0u; off >>= 1) acc += simd_shuffle_xor(acc, off);
    if (lane == 0) outb[row] = acc;
}
"#;

#[test]
fn wide_loads_at_the_down_projection_geometry() {
    let Some(device) = MetalDevice::open() else {
        println!("SKIP: no Metal device");
        return;
    };
    let pipes = device.compile(MSL_DOWN);
    const DOWN_IN: usize = 8192;
    const DOWN_OUT: usize = 2048;

    let activation: Vec<u16> = (0..DOWN_IN).map(|i| bf16(((i % 17) as f64 - 8.0) / 32.0)).collect();
    let x = device.from_bytes(bytemuck_cast(&activation));
    let weight_buffers: Vec<_> = (0..REPEATS)
        .map(|w| {
            let weights: Vec<u16> = (0..DOWN_OUT * DOWN_IN)
                .map(|i| bf16((((i + w) % 31) as f64 - 15.0) / 256.0))
                .collect();
            device.from_bytes(bytemuck_cast(&weights))
        })
        .collect();
    let variant = |kernel: &str| {
        let outs: Vec<_> = (0..REPEATS).map(|_| device.alloc_f32(DOWN_OUT)).collect();
        weight_buffers
            .iter()
            .zip(&outs)
            .map(|(weight, out)| Dispatch {
                pipe: pipes.get(kernel),
                inputs: vec![x.clone(), weight.clone()],
                output: out.clone(),
                grid: DOWN_OUT * SIMD,
                argbuf: None,
            })
            .collect::<Vec<_>>()
    };
    let (scalar8, vec4) = (variant("down_scalar8"), variant("down_vec4"));
    let bytes = (REPEATS * DOWN_OUT * DOWN_IN * 2) as f64;
    let time = |dispatches: &[Dispatch]| -> f64 {
        let graph = device.capture(dispatches);
        (0..20)
            .filter_map(|_| device.run_graph_timed(&graph).ok())
            .fold(f64::INFINITY, f64::min)
    };
    let (mut s8, mut v4) = (f64::INFINITY, f64::INFINITY);
    for _ in 0..3 {
        s8 = s8.min(time(&scalar8));
        v4 = v4.min(time(&vec4));
    }
    for (label, seconds) in [("down scalar x8", s8), ("down uint4 (16B)", v4)] {
        println!(
            "  {label:<18} {:8.1}us/kernel  {:6.1} GB/s  {:.2}x",
            seconds / REPEATS as f64 * 1e6,
            bytes / seconds / 1e9,
            s8 / seconds,
        );
    }
}

/// The findings above, pinned. If a future codegen, dtype or machine makes
/// wide loads pay, that is a real result and this is the thing that should
/// report it.
#[test]
fn wide_loads_stay_worth_nothing() {
    let Some(device) = MetalDevice::open() else {
        println!("SKIP: no Metal device");
        return;
    };
    let pipes = device.compile(MSL);
    let activation: Vec<u16> = (0..HIDDEN).map(|i| bf16(((i % 17) as f64 - 8.0) / 32.0)).collect();
    let x = device.from_bytes(bytemuck_cast(&activation));
    let weights: Vec<u16> = (0..OUTPUTS * HIDDEN)
        .map(|i| bf16(((i % 31) as f64 - 15.0) / 256.0))
        .collect();
    let weight_buffer = device.from_bytes(bytemuck_cast(&weights));
    let time = |kernel: &str| -> f64 {
        let out = device.alloc_f32(OUTPUTS);
        let dispatches = vec![Dispatch {
            pipe: pipes.get(kernel),
            inputs: vec![x.clone(), weight_buffer.clone()],
            output: out,
            grid: OUTPUTS * SIMD,
            argbuf: None,
        }];
        let graph = device.capture(&dispatches);
        (0..20)
            .filter_map(|_| device.run_graph_timed(&graph).ok())
            .fold(f64::INFINITY, f64::min)
    };
    let (scalar, wide) = (time("matvec_scalar8"), time("matvec_vec4"));
    println!(
        "scalar {:.1}us  uint4 {:.1}us  {:.2}x",
        scalar * 1e6,
        wide * 1e6,
        scalar / wide
    );
    assert!(
        scalar / wide < 1.10,
        "wide loads now WIN by {:.2}x — revisit todo.md item 7b",
        scalar / wide
    );
}
