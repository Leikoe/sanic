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
(combine blocks in any order). Everything else in this repo is that one fact
applied by structural recursion.

## The layers

```
ir  →  analyze  →  derive  →  cost / plan / partition  →  interp (oracle)
                                     ↓                        ↓
                       grad (autodiff)   runtime (sessions)   rustgen / emit_metal
```

- **`ir`** — a CLOSED compute basis: `Input · Const · Iota · Map · Reduce ·
  Scan · Gather`, plus two *structural* operators: `View` (rename an axis,
  or flatten a group into one) and `Reindex` (affine index maps — slice,
  zero-pad, reshape-split, sliding windows, reversal — with no computation
  and no copy in either). `Map` applies a small closed enum of scalar
  primitives (`add mul max lt where exp log sqrt …`), so the deriver is
  total over the vocabulary — there is no open set of named ops to
  special-case. Matmul, softmax, attention, silu, causal masks, one-hot
  position writes, scatter-add, convolution and SSM scans are *compositions*
  of the basis; argmax and top-k are single `Reduce` ops over tuple monoids
  (an index-carrying max, and sorted k-lists under merge). Axes are variables, not strings: an `Axis`
  is a fresh integer identity with a printing label.
- **`analyze`** — one recursive pass tags every `(node, axis)` as `FREE`
  (grid), `MONOIDAL` (foldable), `OPAQUE` (data-dependent gather), or
  `SEQUENTIAL` (non-associative recurrence). `analyze_all` packages the
  verdicts into the structure map.
- **`derive`** — one bottom-up fold turns a foldable axis into a concrete
  accumulator. A fixed set of rules fires, and each carrier records which
  did: `fold`, `fused-map`, `tuple`, `rescale` (the online-softmax
  coupling — a slot rides a running max), `defer-div` (the normalizer is
  applied once, at the end), `k-best` (argmax / top-k as index-carrying
  tuple monoids), `invariant` / `lattice` / `defer-add` / `defer-scale`
  (the distributive laws — Σ over an unvarying axis is n·value, order
  reductions commute with max/min/offset/scale couplings). Coupling and
  deferral are detected by *data dependence* — the online-softmax coupling
  is discovered from the plain composition `Exp(x − max)`, not matched
  against a fused special form — so the same code derives `sum`, `mean`,
  `variance`, `logsumexp`, FlashAttention, and RMSNorm-fused GEMMs alike.
  Soundness AND completeness are both oracle-tested: every carrier runs
  against the interpreter, and every DECLINE faces a semantic
  carrier-existence probe (`tests/completeness.rs`) — a declined program
  whose carrier the probe can exhibit is a failing test, so missed fusions
  surface as red CI, not benchmark surprises.
- **`cost` / `plan`** — the profitability half. `cost` knows only `Device`s
  and `Kernel`s: feasibility, a roofline, and two searches. `plan` assigns
  every output axis a role — row tile, column tile, grid batch, or
  SRAM-resident — by *enumerating and pricing every assignment* (the spaces
  are tiny), not by heuristics. Row×column blocks recover the classic 2D
  GEMM I/O trade; row-only blocks with a streamed re-read recover the
  FlashAttention trade. Legality is already settled, so the cost model only
  *ranks* — it can pick a slow plan, never a wrong one.
- **`partition`** — split a *whole graph* into kernels. One rule does the
  work: **the derive frontier is the fusion boundary**. Everything one
  `derive` call swallows is, by construction, one legal fused kernel; the
  carrier's leaves are exactly where the derivation stopped composing, so
  that is exactly where the cuts go. Each cut subgraph is partitioned
  recursively; shared producers (residuals) are materialized once; a lone
  elementwise consumer of matching shape rides its producer as an epilogue.
  No pattern library, no fusion heuristics over op names.
- **`interp`** — the dense reference interpreter: the correctness oracle.
  `eval` is the definition of what a graph means; `run_carrier` drives a
  derived accumulator on real tensors; `run_carrier_split` re-associates it
  into a two-stage split reduction (partials + combine — the GROUP schedule,
  equal by the monoid law). Everything downstream is checked against it.
- **`grad`** — reverse-mode autodiff over the closed basis. Backward graphs
  are ordinary IR (gather's adjoint is `scatter_add`, flatten's is `split`,
  a stride-1 window's is the mirrored window), so a training step goes
  through the same derive/partition/execute/compile pipeline as inference.
  Every rule is held to central finite differences.
- **`runtime`** — stateful execution: a `Session` owns persistent buffers
  (weights, KV caches, optimizer state); a step runs a multi-output schedule
  (`partition_many`) and then *commits* chosen outputs over existing buffers
  — write-after-read discipline by construction, no `Store` node, no
  aliasing proofs. Incremental KV-cache decode is proven equal to full
  prefill through this path, and an SGD loop converges through it.
- **`codegen` / `rustgen` / `emit_metal`** — one shared node→code recursion
  behind a `Lang` trait; the backends are thin. `rustgen` lowers a whole
  schedule to Rust (verified by compiling with `rustc` and running the
  binary); `emit_metal` lowers it to MSL and the tests **dispatch it on the
  Apple GPU** through `objc2-metal` (in-process — no external toolchain),
  including the decode loop with persistent device buffers, a backward pass,
  and a two-kernel split reduction.
- **`emit_rust`** — the derived kernel as compilable Rust: scalar and tiled
  (single-carrier; superseded by `rustgen` for whole schedules).

```rust
let (sq, k, d, e) = (axis("sq"), axis("k"), axis("d"), axis("e"));
let attn = attention(q, kk, v, d, k);  // softmax(QKᵀ)·V
let map  = analyze_all(&attn);         // classify every axis + derive
// k → MONOIDAL with the derived (m, ℓ, o) accumulator; sq, e → grid
```

## Why "derived, not matched" matters

The carrier is *data* — slots plus symbolic `into` / `combine` / `project`
programs — so every derivation is executed and property-tested:
`tree_fold == fold == reference` on random data, which is associativity and
correctness in one assertion. Because the deriver is generic, it passes
graphs no template anticipates:

- **multi-value attention** — two value tensors share one softmax →
  `(m, ℓ, o₁, o₂)`.
- **CTC forward** — the full log-space DP: derives `(max, Σexp)` for the
  predecessor and loss logsumexps while correctly tagging the time recurrence
  `SEQUENTIAL` (refusing to fold it) and the label gather `OPAQUE`.
- **soft attention over a streamed log-space DP** — one graph that forces the
  rescale coupling, the deferred divide, `OPAQUE`, `SEQUENTIAL`, *and* the
  same-axis-merge vs cross-axis-tiling distinction at once.
- **MLA (DeepSeek-style latent attention)** — standard and absorbed forms
  both derive the exact FlashAttention carrier (`cargo run --example mla`).

CTC's logsumexp and FlashAttention's softmax come out as the *same*
derivation with different projections (`log(s)+m` vs `o/s`) — both built from
one exp-shifted-sum slot.

## Axes are variables (the scoping semantics)

Axes ARE variables — literally: an `Axis` is a fresh integer id minted by
`axis("label")` (the string is only for printing), so Rust's own lexical
scoping becomes the axis scoping. Pass the same `Axis` value where two
tensors share an index space; mint a new one where they don't. Reusing a
live variable is an explicit act, not a string-collision accident. On top
of that identity, the analysis treats binding the way an interpreter does:

- **`Reduce` is a binder.** Inside the reduction the axis is live; the
  result no longer carries it.
- **`View` re-binds.** `rename(x, s, t)` is alpha-conversion — the same
  buffer under a fresh position variable, so the key/value side of attention
  can read the *same* normalized tensor the query side computed, without a
  copy and without deriving the norm twice. `flatten(x, [h, dv], dmv)` is
  the structural isomorphism that merges index spaces — heads fold into the
  model dimension before an output projection, for free.
- **Classification is capture-aware at view boundaries.** A grouped output
  axis inherits the *join* of its members' structures; a consumed source
  name is out of scope above the view — asking about it is asking about a
  variable that no longer exists.

What this buys concretely: one norm shared across Q/K/V views, multi-head
attention with a legal head-flatten, and a way to *shield* downstream
consumers from upstream axis reuse by renaming at the rebirth point. The
remaining frontier is *forced* reuse: a residual chain genuinely requires
the same axis end-to-end, so a norm's bound `dm` and the residual's free
`dm` share one variable and the analysis conservatively entangles them (the
partitioner then cuts more than strictly necessary — visible in the LLM
schedule, where the second norm does not fuse into the gate/up GEMMs the
way the first fuses into Q). Scope inference over the graph — proving the
bound occurrence can be alpha-renamed apart from the free one — is the
natural next step, along with `split` (the inverse of flatten) and
layout/transpose as further structural operators.

## A real workload: an LLM forward pass

`cargo run --example llm` builds a full *multi-head* transformer block plus
the logits head — embedding gather, RMSNorm, QKV projections, scaled
attention with a **computed** causal mask (`iota` + compare + `where` — no
mask tensor exists), head flatten, output projection with residual, SwiGLU
MLP with residual — and partitions it into kernels:

```
schedule — 13 kernels
  [ 0] t7   = fold `dm`(X, g1, Wq)  [2 slots: defer-div, fold, fused-map, tuple]
              ← the Q projection reads RAW X: the RMSNorm is fused into the
                GEMM (Σx² rides as a second slot, ÷√(mean+ε) deferred)
  [ 1] t11  = fold `dm`(X)                    Σx² for the K/V-side norm
  [ 2] t10  = map div·mul·sqrt·add(X, g1, t11)
  [ 3] t8   = fold `dm`(t10, Wk)   row t×64  grid {h}
  [ 4] t12  = fold `dm`(t10, Wv)   row t×64  grid {h}
  [ 5] t6   = fold `t`(t7, t8, t12)
              [3 slots: defer-div, fold, fused-map, rescale, tuple]
              row s×32  grid {h}
              ← FlashAttention: QKᵀ in-body, the scale (a literal) AND the
                causal mask (index arithmetic) fused into the lift — the
                kernel's inputs are just the three projections
  [ 6] t3   = fold `dmv`(t6, Wo)  row s×64 col dm×128  + epilogue add(X)
  [ 7] t13  = fold `dm`(t3)                   Σx² for the second norm
  [ 8] t2   = map div·mul·sqrt·add(...)
  [ 9] t1   = fold `dm`(t2, Wg)   row f×64  col s×128    gate GEMM
  [10] t14  = fold `dm`(t2, Wu)   row f×64  col s×128    up GEMM
  [11] t0   = fold `f`(t1, t14, Wd)  row s×64 col dm×256
              ← down GEMM, silu·gate·up fused into the lift (silu is a
                composition of basis ops, not a special form)
  [12] Out  = fold `dm`(t0, t3, W_lm)  row v×128 col s×128
              ← logits GEMM, second residual fused into the lift
```

Two of these go *beyond* the fusion structure mainstream compilers produce:
the norm-fused Q projection (stage 0 — torch.compile emits three kernels for
that) and the zero-traffic computed causal mask (stage 5). Every cut is a
point where the algebra stopped composing, and every block shape is a priced
choice. (The K/V-side norm is still materialized: the residual chain reuses
`dm`, which entangles the second norm's consumers — the known axis-reuse
frontier.)

## What runs today, end to end

**GPT-2 (124M), real OpenAI weights, on the GPU, matching HuggingFace:**
`cargo run --release --example gpt2 --prompt "…"` loads the official
safetensors (dependency-free reader; bf16/f16/f32), builds the 12-layer
network as plain IR, partitions the KV-cache decode step into ~220 kernels
in **0.04 s**, and dispatches on Metal — the wte/wpe tables (158 MB) bound
**zero-copy** straight from the checkpoint on unified memory, no upload. It
**streams greedy tokens at ~125 tok/s (~8 ms/tok)** through the in-tree
byte-level BPE encoder (cache commits are on-device buffer swaps — the
Session discipline). Logits match a `transformers` reference to **max
|Δlogit| = 1e-4 with 24/24 greedy tokens identical** (bf16-round-tripped
weights: ≤ 0.54 drift, same tokens).

**Trinity-Nano (5.5B AFMoE), int4-packed, on a 16 GB laptop:**
`cargo run --release --example trinity` compiles a 56-layer, 128-expert MoE
with grouped-query attention into **1,478 dispatches per decode step, ~30
unique kernels** (~46% fewer than mlx-lm dispatches for the same model, with
zero primitives) — GQA as shared axis variables, top-8 sigmoid routing as
**one fold over all ranks** (the k-best tuple monoid, rank-indexed
projection), and the MoE proper as a router plus **grouped gate/up/down folds
over a 9-slot axis** (8 routed experts + the shared expert), one
vector-indexed `gather` selecting every slot's weights **directly from packed
int4 device buffers** (typed storage: nibbles unpack inside the GEMM folds,
per-group scales fused as axis structure; the 3.8 GB checkpoint never
dequantizes). Per-position logits sit at bf16-reference noise, greedy output
matches the HF reference token-for-token, streaming *"The capital of France
is Paris."* at **~45 tok/s (~22 ms/tok, or 18 with `--tune`)** — a measured
ladder from 26 ms, itemized against MLX's hand-written kernels in
`vs_mlx.md`.

Every capability below is verified numerically against the interpreter, and
where it says GPU, the kernels were dispatched on an Apple GPU and matched:

- **Inference:** flash attention (plain / causal / cosine-bias / **RoPE** /
  **sliding-window**), RMSNorm-fused GEMMs, SwiGLU-fused GEMMs, quantized
  (dequant-fused) matmul, embedding gather, argmax greedy sampling, the full
  multi-head transformer block — all GPU-verified.
- **Convolution** = `window + flatten + matmul`: one implicit-GEMM kernel,
  no im2col buffer, SAME padding included — interp → compiled Rust → GPU.
- **Autoregressive decode:** a persistent KV cache updated in place
  (commit-after-execute), position fed as data — T incremental steps equal
  one causal prefill on the interpreter, in compiled Rust, and on the GPU.
- **Training:** reverse-mode gradients as ordinary graphs (finite-difference
  checked), attention backward compiled and GPU-dispatched, an SGD loop that
  converges through the same session runtime.
- **Split reductions (GROUP):** occupancy-starved folds re-associate into
  partial + combine kernels, priced by the same roofline that picks tiles,
  and run as two kernels on the GPU.

## Where this falls short of a real ML compiler

Each is a concrete, reproducible gap:

1. **Axis scoping is explicit, not inferred.** `View`/`Reindex` give rename,
   flatten, split, slice, pad and windows — but the *author* must place them
   and pick fresh names. Reusing a live axis name still entangles the
   analysis and forces extra cuts. Automatic alpha-renaming and
   layout/transpose inference remain open.
2. **No row-resident two-pass kernels.** A per-element output normalized by a
   same-axis fold — RMSNorm, softmax *as an output* — legally needs the row
   in SRAM and two passes over it. The engine's kernel model only has
   one-pass folds, so these split into fold + elementwise pairs (stages 0–1
   above). Real compilers fuse them.
3. **Cost-blind cuts.** The partitioner cuts where fusion is *illegal*; it
   never asks whether an extra legal cut would be *cheaper*. The
   split-reduction factor is priced (`plan::split_factor`) but `partition`
   does not yet invoke it automatically, and epilogue fusion only fires for
   a single same-shape consumer.
4. **Elementwise, gather and sequential stages are not costed**, registers
   are not modeled separately from SRAM, tiles are power-of-two only, and the
   device is a toy roofline — ranking-grade, not measurement-grade. (Block
   structure is at least priced, not guessed: row/col/batch/resident roles
   are enumerated per kernel, storage dtypes bill their true bytes, and
   memory-level parallelism is modeled separately from occupancy.)
5. **No autotuning, no real byte storage for int8/int4** (the *pricing* and
   the dequant fusion exist; buffers are still f64/f32), **no memory
   planning** (liveness, buffer reuse), **no dynamic shapes, no multi-device
   execution** (the allreduce math exists as the split-reduction merge; a
   device runtime does not). Monoidal prefix scans emit and `Add`-scans
   differentiate (cumsum ⟵ reversed cumsum); `Max`/`Min` and affine-scan
   backward are declined.

What's genuinely novel here versus the mainstream: XLA/Inductor-class
compilers fuse by op-category pattern rules and treat online-softmax-style
rewrites as special cases; here both the *kernel body* (the accumulator) and
the *kernel boundaries* (the cuts) fall out of one algebraic analysis. The
closest published relatives are the monoid/scan literature and
fusion-by-rewriting systems (Futhark, tinygrad's scheduler); the
derive-the-carrier-then-cut-at-its-leaves formulation is the part worth
pushing further.

## Fuse or cut?

The fuse-vs-cut decision for attention is a *client* of the library, in
`tests/scheduling.rs`: it prices the fused kernel (stream `k`, keep
`(m, ℓ, o)` in SRAM — the size read off the derived carrier) against the cut
two-matmul plan (materialize the scores), and asks `cheapest` which wins. It
fuses at small head dims, cuts when fusion stops paying (SRAM pressure
collapses occupancy past the materialization it avoids), and falls back to
cut when fusion is infeasible.

## Run it

```
cargo run --example derive   # print structure maps + derived carriers
cargo run --example mha      # naive multi-head attention → FlashAttention
cargo run --example mla      # DeepSeek MLA, standard & absorbed → same kernel
cargo run --example llm      # a transformer block, split into 13 kernels
cargo run --example execute  # run a partitioned schedule on real tensors
cargo run --example kernels  # a gallery of derived flash kernels (Rust)
cargo run --release --example mnist  # train an MLP on MNIST end to end, on the GPU
cargo test                   # 123 tests (incl. rustc-compiled and GPU-dispatched)
```

`cargo run --example mha` builds naive multi-head attention as an AST,
derives FlashAttention from it with no MHA-specific code (batch and head are
just extra free axes), and emits the kernel as Rust source:

```rust
/// Fused streaming kernel — grid over {b, h, sq, e}, stream over `k`.
pub fn flash_attention(elements: impl IntoIterator<Item = [f64; 2]>) -> f64 {
    let mut acc = [f64::NEG_INFINITY, 0.0f64, 0.0f64];
    for x in elements {
        let el = [x[0], 1.0f64, x[1]];
        acc = [
            acc[0].max(el[0]),
            acc[1] * (acc[0] - acc[0].max(el[0])).exp() + el[1] * (el[0] - acc[0].max(el[0])).exp(),
            acc[2] * (acc[0] - acc[0].max(el[0])).exp() + el[2] * (el[0] - acc[0].max(el[0])).exp(),
        ];
    }
    acc[2] / acc[1]
}
```

The planner then sizes it — the carrier's exact accumulator size feeds the
SRAM constraint, the tile sweep keeps the cheapest tile that fits — and the
emitter blocks the kernel by that tile. For the GPU, `emit_metal` lowers the
same schedule to MSL and the tests dispatch it on the Apple GPU.

The structure map for attention, straight from `cargo run --example derive`:

```
structure map
  k    MONOIDAL           → fold
         carrier (3 slots) [defer-div, fold, rescale, tuple]
           into:    s0 = x0;  s1 = 1;  s2 = x1
           combine: s0 = max(a0, b0)
                    s1 = a1·exp(a0 - max(a0, b0)) + b1·exp(b0 - max(a0, b0))
                    s2 = a2·exp(a0 - max(a0, b0)) + b2·exp(b0 - max(a0, b0))
           project: s2 / s1
  d    MONOIDAL           → fold (in a sub-expression)
  sq   FREE               → grid (DOALL)
  e    FREE               → grid (DOALL)
```

## Relatives

`vs_tinygrad.md` is a code-level comparison against tinygrad (whose new
rangeify scheduler independently converged on the same substrate — kernels
as shared loop ranges, movement as index arithmetic — and whose fusion
criterion stops exactly where this engine's derivation keeps going).

## History

`streamability_engine.md` and `scheduler_engine.md` are the original design
documents (they use older module names and rule numbering); the code above is
the current shape.
