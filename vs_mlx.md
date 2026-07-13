# sanic vs MLX: the same model, kernel by kernel

Where `vs_tinygrad.md` compares fusion *criteria*, this compares *generated
kernels* — sanic's emitted Metal against MLX's hand-written kernels, on the
same workload (Trinity-Nano 5.5B afmoe, batch-1 KV decode), the same M1 Pro
16 GB, measured the same way. MLX is 0.31.2 (its Metal kernel sources ship
in the wheel under `mlx/include/mlx/backend/metal/kernels/`, cited below);
the mlx-lm run is the upstream afmoe port, 8-bit affine group-64. sanic is
the `feat/interp-oracle` tree, W4A16 (int4 experts, f32 attention weights,
f16 lm_head).

Method: sanic per-kernel times are command-buffer **GPU timestamps** over 16
back-to-back repeats (`--bench`; no per-dispatch sync floor), classified by
which weight each stage reads; the full-step number is one
`executeCommandsInBuffer` replay. MLX per-op times are lazy batches of 100+
independent applications at exact decode shapes
(`weights/mlx_trinity_bench.py`) — their natural submission mode. Repeats
keep small weights SLC-hot on both sides, so per-kernel figures are floors;
each side's *full-step* ground truth anchors the totals, and both
reconstruct from the parts (sanic: Σ 236.5 ms vs 211.7 ms replayed, the gap
is replay overlap; MLX: Σ over census counts ≈ 16.4 ms vs 15.7 measured).

## Scoreboard (one decode step)

|  | sanic | mlx-lm 8-bit |
|---|---|---|
| kernels dispatched | **1,856** (all derived) | ~2,733 (primitive library) |
| step GPU time | 211.7 ms replayed (246.5 encoder-per-dispatch) | **15.7 ms** (63.6 tok/s measured) |
| active bytes/step | ~1.58 GB (f32 attn + f16 head + int4 experts) | ~0.87 GB (8-bit everything) |
| bandwidth floor | ~8 ms | ~4.5 ms |

sanic dispatches 32% fewer kernels and takes 13.4× longer: kernel count and
kernel quality are fully decoupled, and this document is the autopsy of the
quality gap. It ends measured, not speculative: three hand-scheduled
rewrites of sanic's own kernels (same math, same buffers, oracle-checked)
recover 123 ms of the 196 ms gap, and the per-class ledger accounts for the
rest.

## Where the 13× lives (sanic per-class GPU times)

From `--bench` (weights/trinity_bench.log): 211.7 ms replayed splits as

| class | ms/step | share | eff. GB/s | MLX same work |
|---|---|---|---|---|
| attention core (56 derived flash folds, T=256 window) | **130.5** | 55% | 0.2 | 0.9 ms (56 × 15.4 µs sdpa @ T=256; 0.5 ms at real T≈16) |
| MoE grouped int4 folds (gate/up/down × 54) | **52.2** | 22% | 3–6 | 3.3 ms (54 × 61 µs for the whole gather_qmm chain) |
| attention projections q/k/v/gate/o (f32) | **32.0** | 14% | 25–26 | 5.3 ms (8-bit qmv) |
| dense MLP (2 layers, int4) | 7.0 | 3% | 0.6–5 | 0.24 ms |
| topk rank folds (432 tiny kernels) | 4.6 | 2% | — | 1.1 ms (54 × 20.7 µs argpartition) |
| other elementwise (641 kernels) | 4.8 | 2% | — | ~3 ms (its own tail: 337 RMSNorm + ~500 maps × 3–4 µs) |
| router / combine / cache / epilogues | 3.1 | 1% | — | ~1 ms |
| lm_head f16 matvec [1024→200192] | 2.4 | 1% | **172** | 1.3 ms (8-bit, 164 GB/s) |

The lm_head row is the control experiment: the same scalar one-thread-per-
output fold that manages 0.2–26 GB/s everywhere else hits 172 GB/s — nearly
the machine ceiling — when the grid is 200k threads. **The emitted code is
not too slow; the launch shapes are.** Three specific diseases, visible by
putting the sources side by side:

**1. The flash fold recomputes the score once per output lane.** sanic's
derived kernel (`k_b_t*_fold`) assigns one thread per output point
`(hk, qg, rv)` and streams all 256 cache rows; each step's QKᵀ dot
(`for r6 in 0..128`) does not depend on `rv` — so all 128 rv-threads of a
head compute the same 128-wide dot, a 128× compute redundancy, with
div/mod flatten-index chains *per element* and 4 `exp` calls per step. At
1024 threads total the GPU is also idle. MLX's `sdpa_vector.h` is the same
online-softmax carrier — running `(m, ℓ, o)` with rescale factors,
literally the algebra `derive` reconstructs — but *scheduled*: one
threadgroup per (batch × q-head), 32 lanes split the head dim and share the
score via `simd_sum` (computed **once** per key), 32 simdgroups split the
key axis, and the 32 partial carriers merge through threadgroup memory
(`sdpa_vector.h:151-175`) — which is exactly sanic's `run_carrier_split`
stage-2 merge, proven legal in `tests/group.rs`, fused into the kernel.
And it reads only the N live rows, where sanic's fixed T_MAX=256 window
streams a masked tail that folds the monoid identity — dead work the
algebra already knows is dead.

**2. The quantized folds unpack one nibble at a time and load the scale
per element.** sanic's MoE fold (grid 2304) does, per multiply-add: a byte
load + shift/mask, a float→int conversion of the *loop-invariant* expert
index, a div/mod chain, and a fresh scale load. MLX's `qmv_fast_impl`
(`quantized.h:750-815`; the gather variant at `:1900`) makes each simdgroup
produce **4 output rows**, lanes pull 8 packed values as one `uint32`, the
activation vector loads once into registers and is *reused across the 4
rows*, the group scale loads once per block, and a `simd_sum` closes each
row. ~2 instructions per MAC instead of ~12, at 4–8× the occupancy.

**3. The f32 projections pay 2× bytes and 4× occupancy.** q/gate/o at grid
1024 run at 26 GB/s; the checkpoint stores these weights in bf16, so f32
upload is a self-inflicted doubling (825 MB/step of the 1.58 GB total).
MLX runs them 8-bit. (Their small-matvec kernels are not magic — measured
alone, their 1 MB qmv is 43.6 GB/s hot, launch-bound like everything at
these shapes; they win on bytes and never being catastrophically
underoccupied.)

## The measured blueprint (`--proto`)

The fix is not "write kernels like MLX" — it is one codegen change:
**cooperative fold scheduling**. Assign a fold to a simdgroup (or
threadgroup) instead of a thread: lane-split the in-body contraction and
close it with `simd_sum`; split the stream axis across simdgroups and merge
partial carriers with the monoid merge the split-reduction rung already
proved; vectorize loads; hoist stream-invariant scalars. Nothing in it is
new algebra — the carrier, the merge legality, and the axis roles
(`plan`'s row/col/batch) all exist. To bound the win before touching the
emitter, `--proto` hand-schedules the three dominant classes against the
emitted kernels **on the same buffers, checked against their outputs**
(weights/trinity_proto.log):

| kernel (real weights, GPU timestamps) | emitted | proto | speedup | max rel err |
|---|---|---|---|---|
| flash fold, window 16 | 2,366 µs | **7.7 µs** | 308× | 0 |
| flash fold, window 256 (identical work) | 2,362 µs | **18.5 µs** | 128× | 0 |
| q proj + fused RMSNorm (f32 4.2 MB) | 185 µs | **26.4 µs** | 7.0× | 6e-7 |
| MoE gate, 9 × int4 [1024→256] | 384 µs | **14.3 µs** | 27× | 9e-8 |

The flash proto at the full 256 window — **18.5 µs in f32** — matches MLX's
sdpa at the same window (15.4 µs in f16). Parity with the hand-written
kernel, from the derived carrier, in one afternoon of scheduling. The
carrier didn't change; the launch did.

Substituting the proto into all 56 flash stages of the real step (same
output buffers, downstream untouched) replays at **88.9 ms** — from
211.3 — confirming the per-class ledger adds linearly. Applying the same
arithmetic to the remaining classes:

| step | ms/step |
|---|---|
| today (replayed) | 211 |
| + cooperative flash, honest window (measured substitution) | **89** |
| + cooperative MoE folds (162 × ~15 µs vs 52.1 ms) | ~39 |
| + cooperative projections (f32: 280 folds at ~26/8 µs vs 32.2 ms) | ~13 |
| + dense/router coop, tail as-is | **~11–13** |

That lands at MLX's 15.7 with margin, *before* dtype parity. The remaining
known rungs, each with its measured price: f16 attention weights (−413 MB
≈ −2 ms of floor, and the checkpoint is bf16 anyway), one fold per layer
for all 8 top-k ranks instead of 432 single-rank kernels (the k-best
monoid already carries every rank's state; −3–4 ms), int8 lm_head
(−1.1 ms), elementwise-cone fusion for the 641-map tail. Sub-10 ms/token
— *above* mlx-lm — is the honest target, because sanic starts with 32%
fewer dispatches and a graph replay MLX doesn't have (they re-encode
every step; it's why their 1 MB qmv reads 25 µs at the Python boundary).

## Bytes ledger (roofline, per decode step)

| | sanic today | sanic @ f16 attn | mlx-lm 8-bit |
|---|---|---|---|
| attention weights | 825 MB (f32) | 413 | 218 |
| active experts + dense | 218 MB (int4 + f32 scales) | 218 | 426 |
| lm_head | 411 MB (f16) | 411 | 218 |
| KV + router + tail | ~125 MB | ~98 | ~10 |
| **total → floor @ ~190 GB/s** | **1.58 GB → 8.3 ms** | 1.14 GB → 6.0 ms | 0.87 GB → 4.6 ms |

MLX runs 3.4× above its floor (launch-bound small matvecs, B=1); sanic
today runs 25× above its own, and the blueprint's endpoint (~11–13 ms on
today's dtypes) is ~1.5× above floor — better than MLX's ratio, on fatter
weights. Int4's active-byte win over 8-bit shows in the experts row: sanic
touches half their expert bytes per token.

## What MLX has that this comparison surfaced (honest column)

Function-constant specialization of one kernel source per shape family vs
sanic's 1,856 distinct entry points (their compile is amortized across all
shapes; sanic's chunked front-end pays 15 s cold). `mx.compile` elementwise
fusion on demand. A 2-pass sdpa for long contexts (`sdpa_vector_2pass_1`)
— the GROUP split applied to attention, which sanic has proven
(`run_carrier_split` on coupled carriers) but not emitted as one fused
pair. Quantized KV cache. Dynamic shapes end to end — sanic's fixed
T_MAX window and period-2 graph capture are decode-loop rigidities MLX
simply doesn't have (their cost: re-encoding ~2,733 kernels per token on
the CPU).

And the inversion worth stating: MLX's `sdpa_vector`, `qmv_fast`,
`rms_norm` are the hand-written primitive library sanic exists to make
unnecessary. The comparison says the *derivation* side is settled — the
derived carrier IS their kernel's accumulator, at parity when scheduled
equally — and the entire remaining gap is the launch geometry. The
research claim sharpens: **derive the kernel, then derive the schedule** —
simdgroup roles from the same axis classification that already picks
tiles, the carrier merge from the same monoid law that already justifies
splits.

## The blueprint, implemented (2026-07-13)

The schedule change shipped as ONE general mechanism, not per-shape
templates (`plan::FoldSched` / `plan::fold_sched` / `emit_fused_metal_sched`):
any derived fold may split its streamed axis across lanes and/or simdgroups
and distribute one output axis across lanes — and every merge is the
carrier's own `combine`, rendered over `simd_shuffle_xor` butterflies or
threadgroup-memory rounds. That is `run_carrier_split`'s re-association law
made intra-kernel; in-body contractions lane-split through the same rule
(`Gen::lane_body`). The chooser prices every candidate with the existing
roofline on a measured device profile (`Device::m1_pro`) plus one honesty
fix the investigation forced: leaves are priced in ISSUE ops (loads, div/mod
index chains, gathers — what the emitter actually emits), not one flop per
element, which is exactly what had made one-thread-per-output look cheap.
Order-sensitive carriers (first-max-wins ArgIdx, k-best's singleton insert,
AffineStep) decline to scalar — the same rule the GROUP split already
enforced, now tested on planted ties. No kernel names any operation: the
sdpa-vector shape, the qmv shape, and the lm_head *non*-change all fall out
of pricing.

Measured, same machine, same models, numerics pinned (Trinity argmax MATCH
at the same Δlogit; GPT-2 24/24 SEQUENCE MATCH at 1e-4; 137 tests):

| | before | after | MLX |
|---|---|---|---|
| Trinity 5.5B step (GPU, replayed) | 211.7 ms | **26.0 ms** | 15.7 ms |
| Trinity decode wall | 196 ms/tok (5.1 tok/s) | **26 ms/tok (38.1 tok/s)** | 63.6 tok/s |
| GPT-2 124M decode wall | 29 ms/tok | **8 ms/tok (128 tok/s)** | 5.3 ms/tok |

Per class (weights/trinity_bench_coop.log): flash 130.8 → **1.9 ms** (the
lane-axis schedule: score once per key per simdgroup, in-body butterfly,
sliced o); projections 32.2 → **5.3 ms at 130–146 GB/s** — within 1.1× of
the hand proto, i.e. at the f32 ceiling; MoE 52.1 → **12.3 ms**; dense
40× faster; router/norm folds all cooperative; lm_head correctly kept
scalar by the chooser (it was already at bandwidth).

What separates 26.0 from 15.7, now itemized by measurement: the MoE down
fold (7.8 ms — the partitioner leaves SwiGLU in-body, recomputed per output
row: a CUT-placement decision, not a schedule one), the 432 top-k rank
kernels (4.9 ms — the k-best fold computes every rank's state but emits one
rank per kernel; a multi-output stage collapses 8→1 per layer), the proto
gap on quantized folds (2.6×: vectorized uint32 nibble loads, x-reuse
across rows per simdgroup) and flash (1.8× full-window: float4 loads; plus
the honest-window early exit), f32 attention weights (−2.6 ms at f16), and
the 641-kernel elementwise tail. Summed, the identified rungs reach
~10–12 ms/tok — past mlx-lm, on an int4 model it can't hold at 8-bit.

## Reproduce

```
cargo run --release --example trinity -- --bench   # per-class GPU profile
cargo run --release --example trinity -- --proto   # scheduled variants vs emitted, oracle-checked
.venv/bin/python weights/mlx_trinity_bench.py      # MLX per-op (nanoinfer venv)
python -m mlx_lm generate --model Trinity-Nano-MLX-8bit --prompt "..." --max-tokens 48
```

Logs: `weights/trinity_bench.log` (scalar era), `weights/trinity_bench_coop.log`
(cooperative), `weights/trinity_proto.log` (protos vs scalar),
`weights/trinity_proto2.log` (protos vs cooperative — the remaining
headroom), `weights/mlx_trinity_bench.log`, `weights/mlx_count.log` (census).
