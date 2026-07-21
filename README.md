# sanic

An engine that **derives** streaming kernels instead of hand-writing them.
Given naive `softmax(QKᵀ)·V` as a dataflow graph, it reconstructs the
FlashAttention online accumulator `(m, ℓ, o)` — running max, rescaled
denominator, unnormalized output — from composition rules. There is no stored
template; the formula is built, executed, and property-tested every run.

## The one idea

A computation can be streamed in one pass **and** parallelized in a tree along
an axis exactly when it is an associative fold along that axis:

```
H(xs ++ ys) = H(xs) ⊗ H(ys)
```

Associativity is the whole game — it is simultaneously the streaming
certificate (fold left-to-right, O(1) state) and the parallelism certificate
(combine blocks in any order). That one fact, applied by structural recursion,
is what lets the *same* code derive `sum`, `softmax`, FlashAttention,
RMSNorm-fused GEMMs and argmax. Top-k is expressed by the frontend as repeated
max/argmax plus one-hot masking; deriving its bounded ordered-selection carrier
from that composition remains an open compiler problem rather than a special
Top-k operation in the core IR.

## Why it's fast on the GPU

The two certificates are also the two things that make a kernel fast — they land
straight on the memory hierarchy:

- **Streaming ⟹ nothing spills to VRAM.** The fold keeps O(1) state
  (FlashAttention's running `(m, ℓ, o)`), so the intermediate it would otherwise
  build — the full `s×k` score matrix — is never written out and read back. It
  stays in registers / threadgroup memory inside a *single* kernel launch. The
  payoff is skipping the round-trip to off-chip HBM, not a faster interconnect.
- **Parallelism ⟹ each output reduces on its own.** Because blocks combine in
  any order, one core (a Metal threadgroup, a CUDA SM) owns an output tile and
  runs its whole reduction to completion — no partial sums traded with other
  cores. That is exactly why FlashAttention needs no separate cross-block
  softmax-combine pass.

So a derived fold streams on-chip and its streamed-axis reduction is core-local
*by construction*. Splitting one reduction across cores (split-K /
Flash-Decoding) stays available — it re-introduces a small combine on purpose —
but that is a scheduling choice the planner makes, not something the algebra
forces.

## What runs today, end to end

**GPT-2 (124M), real OpenAI weights, on the GPU, matching HuggingFace.**
`cargo run --release --example gpt2 --prompt "…"` loads the official
safetensors, builds the 12-layer network as plain IR, partitions the KV-cache
decode step into ~220 kernels in **0.04 s**, and streams greedy tokens on Metal
at **~125 tok/s** — the weight tables bind **zero-copy** from the checkpoint on
unified memory, no upload. Logits match a `transformers` reference to
**max |Δlogit| = 1e-4, with 24/24 greedy tokens identical**.

**Trinity-Nano (5.5B AFMoE), int4-packed, on a 16 GB laptop.**
`cargo run --release --example trinity` compiles a 56-layer, 128-expert MoE
with grouped-query attention into **1,478 dispatches / ~30 unique kernels** per
step (~46% fewer than mlx-lm, with zero primitives) and streams *"The capital
of France is Paris."* at **~45 tok/s** — nibbles unpack inside the GEMM folds
and the 3.8 GB checkpoint never dequantizes. Greedy output matches the HF
reference token-for-token. The measured climb from 26 ms is in `vs_mlx.md`.

Everything else is verified numerically against a reference interpreter (and
dispatched on an Apple GPU where it says GPU): flash attention (causal / RoPE /
sliding-window), quantized matmul, convolution as `window + flatten + matmul`,
autoregressive decode with an in-place KV cache, reverse-mode training with an
SGD loop that converges, and split reductions.

## Run it

```
cargo run --example direct_attention
cargo test
```

The current frontend constructs immutable nodes directly and compiles one or
more output roots; no explicit graph builder is required. Larger historical
model fixtures are parked in `examples/attic/` while they migrate to this
surface. The engine still derives the streaming attention carrier from the
naive graph:

```
structure map
  k    MONOIDAL           → fold
         carrier (3 slots) [defer-div, fold, rescale, tuple]
           into:    s0 = x0;  s1 = 1;  s2 = x1
           combine: s0 = max(a0, b0)
                    s1 = a1·exp(a0 - max(a0, b0)) + b1·exp(b0 - max(a0, b0))
                    s2 = a2·exp(a0 - max(a0, b0)) + b2·exp(b0 - max(a0, b0))
           project: s2 / s1
  sq   FREE               → grid (DOALL)
  e    FREE               → grid (DOALL)
```

## Going deeper

- **`streamability_engine.md` · `scheduler_engine.md`** — the design docs: how
  every axis is classified, how carriers are derived, and how a whole graph is
  cut into kernels (the derive frontier *is* the fusion boundary).
- **`vs_mlx.md`** — the Trinity performance ladder, itemized against MLX's
  hand-written kernels.
- **`vs_tinygrad.md`** — a code-level comparison against tinygrad's rangeify
  scheduler.
- **`todo.md`** — the honest gaps and the plan to close them (inferred axis
  scoping, row-resident two-pass kernels, cost-aware cuts, real byte storage,
  memory planning).
