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
is what lets the *same* code derive `sum`, `softmax`, FlashAttention and
RMSNorm-fused GEMMs. Argmax and Top-k are frontend compositions of ordinary
reductions, comparisons, indices and masking. The compiler derives Argmax's
(extremal key, tied payload) product carrier generically; bounded ordered
selection for Top-k remains open compiler work rather than a named operation
in the core IR.

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

**Llama 3.2 1B, real Meta weights, on the GPU, at mlx-lm's speed.**

```
cargo run --release --example llama3_2 -- "The capital of France is" -n 32 --bf16
```

Compiles one KV-cache decode step into **344 kernels** and binds **146 weight
tensors (2.47 GB) zero-copy** — no upload, no dequantization. On an M1 Pro:
**~52 tok/s** bf16, **~46 tok/s** f32 default. Absolute ms/step drifts 15.6→19
with heat and power, so the durable claim is the head-to-head, measured in one
session against mlx-lm: **parity** (`vs_mlx.md`). Measure warm, on AC, ABBA —
`cost-model-cache-term.md` § Traps.

Everything else is verified numerically against a reference interpreter (and
dispatched on an Apple GPU where it says GPU): flash attention (causal / RoPE /
sliding-window), quantized matmul, convolution as `window + flatten + matmul`,
autoregressive decode with an in-place KV cache, reverse-mode training with an
SGD loop that converges, and split reductions.

GPT-2 124M and Trinity-Nano (5.5B int4 MoE) ran here once; their examples were
deleted 2026-08-01 (`4a642f3`) after rotting unbuilt in `examples/attic/` since
2026-07-17. Results stand in `vs_mlx.md`; nothing in this tree reproduces them.

## Run it

```
cargo run --example direct_attention
cargo test --all-targets          # --all-targets: plain `cargo test` skips the examples' test modules
```

Once per clone, to run CI's checks at commit time. The linux target matters:
Metal is `cfg`'d out there, so macOS-only code becomes dead-code errors a
host-only clippy cannot see.

```
git config core.hooksPath .githooks
rustup target add x86_64-unknown-linux-gnu
```

The current frontend constructs immutable nodes directly and compiles one or
more output roots; no explicit graph builder is required. The engine derives
the streaming attention carrier from the naive graph:

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
