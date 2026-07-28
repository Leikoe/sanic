# Metal Shading Language and compiler

## Fast math deletes infinity masks

The default math mode lets the shader compiler assume no ±inf occurs, which
deletes additive `-INFINITY` masks at compile time — a causal-attention mask
silently becomes a no-op.

`MTLMathMode::Relaxed` keeps the value and the rest of the relaxation. Use it;
do not use the default. Symptom before the fix: llama diverging from a
reference only where masking mattered.

## Reserved names

`half` is the fp16 type. A variable named `half` fails to parse with a
misleading `expected '(' for function-style cast`.

## Launch geometry

- SIMD width 32.
- `[[max_total_threads_per_threadgroup(N)]]` caps the threadgroup; without it
  a pipeline reports 1024 and `dispatchThreads` uses `min(cap, grid)`.
- Buffer bind offsets must be 4-byte aligned (`MTLBuffer`).
- Direct buffer binds cap at 31. Past that, an argument buffer holding
  `gpuAddress()` values — Tier 2, so a `device T*` member is just the address.

## Load width does not matter; grid width does

Replacing eight scalar 2-byte loads with one 16-byte `uint4` on a bf16 matvec:
**1.01×** on a 2048→8192 projection, **1.00×** on 8192→2048.
(`tests/wide_loads_probe.rs`.)

The same codegen at different grid widths, back-to-back over cold weights:

| shape | threadgroups | GB/s |
|---|---|---|
| 2048→8192 | 8192 | 183.6 |
| 8192→2048 | 2048 | 167.6 |

Identical code, 9% apart. Bytes in flight is a lanes×width product and the lane
count is the term that moves.

## A fusion that reduces thread count is a regression

Blocking output rows so a threadgroup owns several — amortizing a normalizer
across them — measures **0.61×**. Per-point runs 2048 threadgroups (524k
threads); block-rows runs 256 (65k). Sustained bandwidth is outstanding bytes
over latency, so 8× fewer threads is 8× fewer loads in flight, and the staged
activation caps residency on top. (`tests/block_rows_probe.rs`.)

This holds even when the fusion removes a kernel *and* a barrier.

## Measurement regimes do not transfer

Three ways to measure the same kernel, three different numbers:

| regime | what it reports |
|---|---|
| `SANIC_DEBUG=4`, encoder per kernel | each kernel's isolated span, ramp included — understates production ~10% |
| standalone probe, one op repeated | SLC-hot if the weight is small: 303 GB/s on a 8 MB matrix, above DRAM peak |
| in-graph, production schedule | the number that predicts wall time |

A per-op benchmark that reuses one weight measures cache. Allocate one distinct
weight per dispatch to reproduce a step's byte ledger.

## A/B ordering

On this machine, whichever variant runs first in a pair loses. Alternating is
not enough — an always-baseline-first A/B produced a spurious +3.4%. Use ABBA
or randomize, and report the pooled sd.
