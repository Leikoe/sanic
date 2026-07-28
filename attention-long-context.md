# Issue: attention does not scale with context length

A llama-3.2-1B bf16 decode step costs 15.4 ms of GPU time at 32 generated
tokens and 20.2 ms at 1024. Weight streaming — the mlp and projection classes,
which are the bulk of the step — is flat across that range. **All 4.8 ms of the
regression is attention over the KV cache**, and none of it is fundamental:
attention's whole working set is 33.75 MB, which is 0.19 ms at this machine's
streaming rate.

Three independent defects, in descending order of cost. They can be fixed
separately and verified separately.

Self-contained; nothing below assumes context from the session that found it.

## The measurement

sanic vs mlx-lm, both bf16 Llama-3.2-1B, batch 1, prefill excluded, greedy.
ABBA-counterbalanced (`sanic, mlx, mlx, sanic` per length) because this machine
penalises whichever variant runs first. Wall-clock ms/tok, mean of the two
slots:

| generated tokens | sanic | mlx-lm | |
|---|---|---|---|
| 32 | **16.45** | 17.25 | sanic ahead 4.9% |
| 128 | 18.05 | **17.46** | mlx ahead 3.3% |
| 512 | 20.30 | **18.09** | mlx ahead 12.2% |
| 1024 | 22.65 | **19.76** | mlx ahead 14.6% |

Run-to-run spread within a pair is 1–3% (worst, sanic at 512: 5.8%), so the
1024 gap is well outside noise. The crossover is between 32 and 128 tokens.

The shape is the finding, not the endpoint. Going 32 → 1024 **mlx-lm loses
2.51 ms/tok; sanic loses 6.20** — sanic degrades ~2.5× faster. Of sanic's 6.20,
4.8 is GPU replay and the remaining ~1.4 is host-side overhead outside replay.

Reproduce:

```
./target/release/examples/llama3_2 "The capital of France is" -n 1024 --bf16
.venv/bin/python weights/mlx_llama_bench.py 1024
```

## Where the 4.8 ms goes

One decode step under `SANIC_DEBUG=4`, per kernel class, in µs. This regime
gives every kernel its own encoder and inflates the total (21.1 ms reported vs
15.4 ms in production, a consistent ×0.735 across both lengths), so read the
**deltas**, and scale by 0.735 for production terms.

| class | ctx 38 | ctx 1030 | Δ |
|---|---|---|---|
| attention output fold (over V) | 254 | 4952 | **+4698** |
| attention scores fold (q·k) | 190 | 2105 | **+1915** |
| rope + cache write | 346 | 1403 | **+1057** |
| all weight-streaming classes | 18 800 | 17 415 | −1385 |
| **total** | **21 054** | **27 498** | **+6444** |

The weight classes do not move with context; their drift is clock and noise.

Cache geometry throughout: `[kv_heads=8, cache_sequence=N, head_dim=64]`, bf16,
contiguous in `head_dim`. One `(head, position)` row is 64 × 2 = **128 bytes,
exactly one cache line**. Per layer, K is `8 × 1030 × 64 × 2` = 1.055 MB, and V
the same; K+V across 16 layers is 33.75 MB.

### A. The output fold reads V with a 128-byte stride — +4.7 ms

```
fold_query_heads32_singleton1_head_dim64_over_cache_sequence1030
grid=(2048,1,1) block=(32,1,1)   bw=1%
```

`out[qh, d] = Σ_s P[qh, s] · V[kv_head(qh), s, d]`. The grid is 32 query heads
× 64 head-dim, so a threadgroup owns one `(qh, d)` and folds over `s`. Stepping
`s` at fixed `d` advances the address by `head_dim × 2` = 128 bytes: **every
iteration lands on a different cache line and uses 2 of its 128 bytes.**

Measured 309 µs per layer against a 1.055 MB working set — **3.4 GB/s, 53× off
the 179.7 GB/s roofline**. That ratio is close to the 64 elements per line you
would waste with no cross-threadgroup line reuse at all.

The 64 threadgroups sharing a `qh` differ only in `d` and could in principle
reuse each line 64×. Measurement says they largely do not. Whether that is a
residency/scheduling artefact or something else is **not established** — the
stride is the structural cause, the reuse behaviour is worth confirming before
choosing between the fixes below.

### B. The whole cache is rewritten every step — +1.06 ms

```
map_kv_heads8_cache_sequence1030_head_dim64
grid=(515,1,1) block=(1024,1,1)   bw=9%
```

515 × 1024 = 527,360 threads = `8 × 1030 × 64` — the entire cache, mapped every
step, to change **one** position. `update_cache` is functional: it produces a
new full cache tensor, and the emitted map covers the full extent rather than
the single live slot.

32 dispatches (16 layers × k and v) × 2.11 MB read+write = **67.5 MB per step**
to update 64 values per layer. The useful write is 32 × 1024 B = 32 KB. That is
a factor of ~2100.

This one is nearly mechanical: the write is a scatter of one position, and
nothing downstream needs a fresh tensor identity that an in-place update
cannot provide.

### C. The scores fold is latency-bound — +1.9 ms

```
fold_query_heads32_singleton1_cache_sequence1030_over_head_dim64
grid=(1030,1,1) block=(64,1,1)   bw=3%
```

Reads are contiguous here — one threadgroup per cache position, 64 threads over
`head_dim`, one 128-byte line each. The problem is volume in flight: each
threadgroup touches 128 bytes and then reduces. 132 µs per layer for 1.055 MB
is **8.0 GB/s, 22× off roofline**.

Sustained bandwidth on this machine is outstanding bytes over latency (see
`applegpu/bandwidth.md`: ~23 outstanding 128 B lines per core, 219 ns
load-to-use). 1030 threadgroups × 128 B is not enough to fill that.

## What a fix is worth

Scaled to production (×0.735):

| fix | saves at 1024 |
|---|---|
| A | ~3.45 ms |
| C | ~1.41 ms |
| B | ~0.78 ms |

Fixing **A and B alone** puts sanic at ~18.4 ms/tok wall at 1024, ahead of
mlx-lm's 19.76 rather than 15% behind.

All three at roofline makes attention 0.19 ms of streaming plus launch
overhead, and the decode step **essentially context-independent to 1024** —
about 15.0 ms of GPU time at any length in that range, versus 15.4 at 32 tokens
and 20.2 at 1024 today.

## Traps

Each of these has already cost real time in this repo; none are hypothetical.

- **Do not fix A by giving threadgroups more rows.** The obvious "one
  threadgroup handles several `d`, amortise the loads" transform reduces thread
  count, and `applegpu/msl.md` records that exact shape measuring **0.61×** on a
  matvec — 8× fewer threads is 8× fewer loads in flight. The fix has to make
  loads contiguous *while keeping the thread count*: stage a V tile in
  threadgroup memory with contiguous `head_dim` loads and fold across `s` in
  registers, flash-style.
- **Transposing the cache to `[kv_heads, head_dim, cache_sequence]` moves the
  problem, it does not solve it.** That makes A's fold contiguous and makes B's
  write strided. Decide with a measurement, not on paper.
- **Measurement regimes do not transfer.** `SANIC_DEBUG=4` understates
  production by ~27% here. A standalone attention probe over one small cache
  measures the SLC, not DRAM. Trust wall-clock A/B on the real example.
- **Always-baseline-first A/B is biased.** Whichever variant runs first loses on
  this machine; an ordered A/B once produced a spurious +3.4% that vanished
  under ABBA. Report pooled sd and position-matched pairs.
- **Check the f32 path.** It is the default and no test covers it; a change can
  pass all 225 tests and still leave only `--bf16` working.

## How to verify a fix

1. **The acceptance test is flatness.** sanic's ms/tok at 1024 within a few
   percent of its ms/tok at 32. Today: 22.65 vs 16.45. A fix that improves the
   1024 number but leaves the *slope* intact has not solved this.
2. Per-defect, from `SANIC_DEBUG=4` at `-n 1024`:
   - A: output-fold `bw=1%` rises; per-layer time falls from ~310 µs.
   - B: the cache-write grid stops covering the full extent — it should be O(1)
     in context, not 515 threadgroups at 1030.
   - C: scores-fold `bw=3%` rises.
3. `cargo test --release` — 225 pass, 0 fail.
4. Both dtypes, text byte-identical.
   `./target/release/examples/llama3_2 "The capital of France is" -n 32 --bf16`
   should still print `…is Paris. It is the most populous city in France and
   the center of the Paris`, and the f32 default must still compile and run.
5. Re-run the ABBA sweep against mlx-lm at 32/128/512/1024 and compare slopes,
   not just the 1024 endpoint.

## Numbers to build on

Measured on the M1 Pro this repo targets; see `applegpu/bandwidth.md`.

- DRAM: **179.7 GB/s** asymptotic cold-stream, **219 ns** load-to-use latency
- Per-dispatch fixed cost: **4.2 µs**
- Size law `t(S) = S/179.7 GB/s + 4.2 µs`, good to ~1% on real llama classes
- Cache line **128 B** — one `(head, position)` row of this KV cache exactly
- System level cache ~24 MB (inferred); cache bandwidth ≥332 GB/s
- KV cache at 1030 context: 1.055 MB per tensor per layer, **33.75 MB** total
