//! Does a block-rows projection with its normalizer fused beat the three
//! kernels sanic emits today? This is a hand-written PROBE — MSL by hand, no
//! derivation — answering one question before any compiler work is built on
//! the answer (todo.md item 4).
//!
//! Today, per RMSNorm + projection, sanic dispatches three kernels with two
//! synchronization phases between them:
//!   1. reduce   Σx² over hidden → one scalar, in ONE threadgroup
//!   2. apply    xn = x·w/‖x‖ over hidden
//!   3. project  out[j] = Σ_h W[j][h]·xn[h], one threadgroup per output row
//!
//! The proposed shape is one kernel: each threadgroup owns a BLOCK of output
//! rows, stages the activation row in threadgroup memory, computes Σx² once
//! during that staging, and projects its block. The normalizer is then
//! recomputed once per threadgroup instead of once per output point — the
//! amortization that was supposed to make the fusion affordable.
//!
//! **It loses, ~1.6×, and this test exists to keep saying so.** Blocking
//! rows is the one thing a batch-1 decode cannot afford: it trades away
//! THREADS. Per-point runs 2048 threadgroups (524k threads, 8 elements
//! each); block-rows runs 256 (65k threads, 64 elements each). Sustained
//! bandwidth is outstanding bytes over latency, so 8× fewer threads is 8×
//! fewer loads in flight — and the 8 KB of staged activation caps how many
//! blocks stay resident per core on top of that. The projection is
//! DRAM-bound, so parallelism is the whole game and amortizing arithmetic
//! buys nothing.
//!
//! The lesson generalizes: on this machine a fusion that reduces thread
//! count is a regression even when it removes a kernel and a barrier.
//!
//! Both shapes are checked against each other before either is timed.

#![cfg(target_os = "macos")]

use sanic::metal::{Dispatch, MetalDevice};

const HIDDEN: usize = 2048; // llama-3.2-1B
const OUTPUTS: usize = 2048; // a q_proj / o_proj
const THREADS: usize = 256; // 8 simdgroups
const ROWS_PER_BLOCK: usize = 8; // one output row per simdgroup

const MSL: &str = r#"
#include <metal_stdlib>
using namespace metal;

inline float bf(ushort v) { return as_type<float>((uint)v << 16u); }
inline ushort to_bf(float f) {
    uint b = as_type<uint>(f);
    return (ushort)((b + 0x7fffu + ((b >> 16) & 1u)) >> 16);
}

// ── the three kernels sanic emits today ──────────────────────────────────────
[[max_total_threads_per_threadgroup(256)]]
kernel void rms_reduce(
    device const ushort* x [[buffer(0)]],
    device float* denom [[buffer(1)]],
    uint tid [[thread_position_in_threadgroup]],
    uint lane [[thread_index_in_simdgroup]],
    uint sgid [[simdgroup_index_in_threadgroup]]
) {
    threadgroup float partial[8];
    float acc = 0.0f;
    for (uint h = tid; h < 2048u; h += 256u) { float v = bf(x[h]); acc += v * v; }
    acc = simd_sum(acc);
    if (lane == 0) partial[sgid] = acc;
    threadgroup_barrier(mem_flags::mem_threadgroup);
    if (tid == 0) {
        float s = 0.0f;
        for (uint i = 0; i < 8u; i++) s += partial[i];
        denom[0] = sqrt(s / 2048.0f + 1e-5f);
    }
}

kernel void rms_apply(
    device const ushort* x [[buffer(0)]],
    device const ushort* w [[buffer(1)]],
    device const float* denom [[buffer(2)]],
    device ushort* xn [[buffer(3)]],
    uint gid [[thread_position_in_grid]]
) {
    if (gid >= 2048u) return;
    xn[gid] = to_bf(bf(x[gid]) * bf(w[gid]) / denom[0]);
}

// MLX's shape (mlx/backend/metal/kernels/rms_norm.metal): ONE kernel, one
// threadgroup per row — accumulate the sum of squares, reduce it across the
// threadgroup, then a second pass writes the whole row.
[[max_total_threads_per_threadgroup(256)]]
kernel void rms_fused(
    device const ushort* x [[buffer(0)]],
    device const ushort* w [[buffer(1)]],
    device ushort* xn [[buffer(2)]],
    uint tid [[thread_position_in_threadgroup]],
    uint lane [[thread_index_in_simdgroup]],
    uint sgid [[simdgroup_index_in_threadgroup]]
) {
    threadgroup float partial[8];
    float acc = 0.0f;
    for (uint h = tid; h < 2048u; h += 256u) { float v = bf(x[h]); acc += v * v; }
    acc = simd_sum(acc);
    if (lane == 0) partial[sgid] = acc;
    threadgroup_barrier(mem_flags::mem_threadgroup);
    float s = 0.0f;
    for (uint i = 0; i < 8u; i++) s += partial[i];
    float denom = sqrt(s / 2048.0f + 1e-5f);
    for (uint h = tid; h < 2048u; h += 256u)
        xn[h] = to_bf(bf(x[h]) * bf(w[h]) / denom);
}

// one threadgroup per output row; the 256 threads split the contraction —
// exactly the shape `sgs=8, lane_stream` produces today
[[max_total_threads_per_threadgroup(256)]]
kernel void project_per_point(
    device const ushort* weights [[buffer(0)]],
    device const ushort* xn [[buffer(1)]],
    device float* out [[buffer(2)]],
    uint gid [[thread_position_in_grid]],
    uint lane [[thread_index_in_simdgroup]],
    uint sgid [[simdgroup_index_in_threadgroup]]
) {
    threadgroup float partial[8];
    uint row = gid / 256u;
    float acc = 0.0f;
    for (uint h = sgid * 32u + lane; h < 2048u; h += 256u)
        acc += bf(weights[row * 2048u + h]) * bf(xn[h]);
    acc = simd_sum(acc);
    if (lane == 0) partial[sgid] = acc;
    threadgroup_barrier(mem_flags::mem_threadgroup);
    if (sgid == 0 && lane == 0) {
        float s = 0.0f;
        for (uint i = 0; i < 8u; i++) s += partial[i];
        out[row] = s;
    }
}

// ── the proposed shape: one kernel, a block of rows per threadgroup ──────────
[[max_total_threads_per_threadgroup(256)]]
kernel void project_block_rows(
    device const ushort* weights [[buffer(0)]],
    device const ushort* x [[buffer(1)]],
    device const ushort* w [[buffer(2)]],
    device float* out [[buffer(3)]],
    uint gid [[thread_position_in_grid]],
    uint lane [[thread_index_in_simdgroup]],
    uint sgid [[simdgroup_index_in_threadgroup]]
) {
    threadgroup float staged[2048];
    threadgroup float partial[8];
    uint tid = sgid * 32u + lane;
    uint block = gid / 256u;

    // pass one: stage the activation row AND fold it, in the same sweep
    float acc = 0.0f;
    for (uint h = tid; h < 2048u; h += 256u) {
        float v = bf(x[h]);
        staged[h] = v;
        acc += v * v;
    }
    acc = simd_sum(acc);
    if (lane == 0) partial[sgid] = acc;
    threadgroup_barrier(mem_flags::mem_threadgroup);
    float s = 0.0f;
    for (uint i = 0; i < 8u; i++) s += partial[i];
    float scale = 1.0f / sqrt(s / 2048.0f + 1e-5f);
    for (uint h = tid; h < 2048u; h += 256u) staged[h] = staged[h] * bf(w[h]) * scale;
    threadgroup_barrier(mem_flags::mem_threadgroup);

    // pass two: this simdgroup owns one row of the block
    uint row = block * 8u + sgid;
    float a = 0.0f;
    for (uint h = lane; h < 2048u; h += 32u) a += bf(weights[row * 2048u + h]) * staged[h];
    a = simd_sum(a);
    if (lane == 0) out[row] = a;
}
"#;

fn bf16(value: f64) -> u16 {
    let bits = (value as f32).to_bits();
    ((bits + 0x7fff + ((bits >> 16) & 1)) >> 16) as u16
}

#[test]
fn block_rows_trades_away_the_threads_dram_bandwidth_needs() {
    let Some(device) = MetalDevice::open() else {
        println!("SKIP: no Metal device");
        return;
    };
    let pipes = device.compile(MSL);

    // deterministic inputs; values small enough that bf16 rounding is stable
    let activation: Vec<u16> = (0..HIDDEN).map(|i| bf16(((i % 17) as f64 - 8.0) / 32.0)).collect();
    let norm_weight: Vec<u16> = (0..HIDDEN).map(|i| bf16(1.0 + (i % 5) as f64 / 64.0)).collect();
    let weights: Vec<u16> = (0..OUTPUTS * HIDDEN)
        .map(|i| bf16(((i % 31) as f64 - 15.0) / 256.0))
        .collect();

    let x = device.from_bytes(bytemuck_cast(&activation));
    let w = device.from_bytes(bytemuck_cast(&norm_weight));
    let weight_buffer = device.from_bytes(bytemuck_cast(&weights));
    let denom = device.alloc_f32(1);
    let staged = device.alloc_elems(HIDDEN, sanic::ir::Dtype::BF16);
    let out_three = device.alloc_f32(OUTPUTS);
    let out_fused = device.alloc_f32(OUTPUTS);

    let three = vec![
        Dispatch {
            pipe: pipes.get("rms_reduce"),
            inputs: vec![x.clone()],
            output: denom.clone(),
            grid: THREADS,
            argbuf: None,
        },
        Dispatch {
            pipe: pipes.get("rms_apply"),
            inputs: vec![x.clone(), w.clone(), denom.clone()],
            output: staged.clone(),
            grid: HIDDEN,
            argbuf: None,
        },
        Dispatch {
            pipe: pipes.get("project_per_point"),
            inputs: vec![weight_buffer.clone(), staged.clone()],
            output: out_three.clone(),
            grid: OUTPUTS * THREADS,
            argbuf: None,
        },
    ];
    let fused = vec![Dispatch {
        pipe: pipes.get("project_block_rows"),
        inputs: vec![weight_buffer.clone(), x.clone(), w.clone()],
        output: out_fused.clone(),
        grid: OUTPUTS / ROWS_PER_BLOCK * THREADS,
        argbuf: None,
    }];

    // same answer first, then the timing means something
    device.run(&three);
    device.run(&fused);
    let (a, b) = (
        device.read_f32(&out_three, OUTPUTS),
        device.read_f32(&out_fused, OUTPUTS),
    );
    let worst = a
        .iter()
        .zip(&b)
        .map(|(l, r)| (*l as f64 - *r as f64).abs() / (1.0 + l.abs() as f64))
        .fold(0.0, f64::max);
    assert!(worst < 2e-3, "the two shapes disagree: {worst:e}");

    // fastest of several replays, both through the production path
    let time = |dispatches: &[Dispatch]| -> f64 {
        let graph = device.capture(dispatches);
        (0..20)
            .filter_map(|_| device.run_graph_timed(&graph).ok())
            .fold(f64::INFINITY, f64::min)
    };
    let (mut three_seconds, mut fused_seconds) = (f64::INFINITY, f64::INFINITY);
    for _ in 0..3 {
        three_seconds = three_seconds.min(time(&three));
        fused_seconds = fused_seconds.min(time(&fused));
    }
    println!(
        "three kernels {:7.1}us   block-rows fused {:7.1}us   {:.2}x",
        three_seconds * 1e6,
        fused_seconds * 1e6,
        three_seconds / fused_seconds,
    );
    // The finding, pinned: fusing by blocking rows LOSES. If a future
    // schedule, dtype or machine flips this, that is a real result and this
    // test should be the thing that reports it.
    assert!(
        fused_seconds > three_seconds,
        "block-rows now WINS ({:.1}us vs {:.1}us) — revisit todo.md item 4",
        fused_seconds * 1e6,
        three_seconds * 1e6,
    );
}

fn bytemuck_cast(values: &[u16]) -> &[u8] {
    // SAFETY: u16 has no padding and any bit pattern is a valid u8 pair
    unsafe { std::slice::from_raw_parts(values.as_ptr() as *const u8, std::mem::size_of_val(values)) }
}

/// Does MLX's fused RMSNorm — one kernel, two passes over the row — beat the
/// reduce-then-apply pair sanic emits? This isolates the NORMALIZER, with no
/// projection involved, so the answer is about the shape itself.
#[test]
fn fused_rms_against_reduce_then_apply() {
    let Some(device) = MetalDevice::open() else {
        println!("SKIP: no Metal device");
        return;
    };
    let pipes = device.compile(MSL);
    let activation: Vec<u16> = (0..HIDDEN).map(|i| bf16(((i % 17) as f64 - 8.0) / 32.0)).collect();
    let norm_weight: Vec<u16> = (0..HIDDEN).map(|i| bf16(1.0 + (i % 5) as f64 / 64.0)).collect();
    let x = device.from_bytes(bytemuck_cast(&activation));
    let w = device.from_bytes(bytemuck_cast(&norm_weight));
    let denom = device.alloc_f32(1);
    let out_pair = device.alloc_elems(HIDDEN, sanic::ir::Dtype::BF16);
    let out_fused = device.alloc_elems(HIDDEN, sanic::ir::Dtype::BF16);

    let pair = vec![
        Dispatch {
            pipe: pipes.get("rms_reduce"),
            inputs: vec![x.clone()],
            output: denom.clone(),
            grid: THREADS,
            argbuf: None,
        },
        Dispatch {
            pipe: pipes.get("rms_apply"),
            inputs: vec![x.clone(), w.clone(), denom.clone()],
            output: out_pair.clone(),
            grid: HIDDEN,
            argbuf: None,
        },
    ];
    let fused = vec![Dispatch {
        pipe: pipes.get("rms_fused"),
        inputs: vec![x.clone(), w.clone()],
        output: out_fused.clone(),
        grid: THREADS,
        argbuf: None,
    }];

    device.run(&pair);
    device.run(&fused);
    let (a, b) = (
        device.read_as_f32(&out_pair, HIDDEN, sanic::ir::Dtype::BF16),
        device.read_as_f32(&out_fused, HIDDEN, sanic::ir::Dtype::BF16),
    );
    let worst = a
        .iter()
        .zip(&b)
        .map(|(l, r)| (*l as f64 - *r as f64).abs() / (1.0 + l.abs() as f64))
        .fold(0.0, f64::max);
    assert!(worst < 2e-3, "the two shapes disagree: {worst:e}");

    let time = |dispatches: &[Dispatch]| -> f64 {
        let graph = device.capture(dispatches);
        (0..50)
            .filter_map(|_| device.run_graph_timed(&graph).ok())
            .fold(f64::INFINITY, f64::min)
    };
    let (mut pair_seconds, mut fused_seconds) = (f64::INFINITY, f64::INFINITY);
    for _ in 0..3 {
        pair_seconds = pair_seconds.min(time(&pair));
        fused_seconds = fused_seconds.min(time(&fused));
    }
    println!(
        "reduce+apply {:7.2}us   fused (MLX shape) {:7.2}us   {:.2}x",
        pair_seconds * 1e6,
        fused_seconds * 1e6,
        pair_seconds / fused_seconds,
    );
}
