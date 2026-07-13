# TODO sanic

A living plan for turning the algebraic-derivation prototype into a real ML
compiler + runtime. Companion to `vs_tinygrad.md` (which argues *why* the
algebraic fusion criterion is the differentiator) — this doc is *how* we ship
it. Written against the tree at the `feat/interp-oracle` branch point.

## The thesis, and what defends it

sanic derives streaming kernels from algebra instead of hand-writing them:
given naive `softmax(QKᵀ)·V`, `derive` reconstructs the FlashAttention
`(m, ℓ, o)` online accumulator with no template. That is the one thing tinygrad
(and everyone else) does *not* do — their fusion is syntactic and cuts at
dependent reductions, so softmax is 3 kernels and flash is hand-written
(`vs_tinygrad.md`). The moat is `derive` + `analyze`. Everything else on this
page is substrate we need so the moat is usable on real workloads.

## Where we are (ground truth, verified by running it)

**Works, end to end, correct:**

- **`derive`** — the core. Reconstructs online-softmax (rescale), deferred
  normalizers (defer-div), fused elementwise, multi-slot tuples, affine/SSM
  scans. 25 law tests: `tree_fold == fold == reference`.
- **`analyze` / `plan` / `cost` / `partition`** — classify axes, pick tiles by
  analytical roofline, split a whole graph at the derive frontier. A full
  transformer block + logits head lowers to 13 kernels with the attention core
  surviving as **one** fused flash kernel and RMSNorm folding into the Q GEMM.
- **`interp`** *(new)* — a dense reference interpreter: the correctness oracle.
  `eval` gives the naive semantics; `run_carrier` drives a derived carrier on
  real tensors. `run_carrier == eval` proves the kernel computes the real math.
- **`Schedule::execute`** *(new)* — runs a whole partitioned schedule on real
  tensors (fused stages stream their carriers; elementwise/gather stages eval
  their spliced sub-graphs; rename/flatten aliasing handled). Proven equal to
  `eval` of the original graph on a full multi-head block
  (`tests/schedule_exec.rs`). **This is the compiler-correctness theorem made
  numeric.**
- **`rustgen`** *(new)* — a real code-generating backend. One recursive
  node→code emitter (`value`, the codegen twin of `interp::eval`) plus the
  carrier layered on top emits a whole schedule as Rust: one fn per kernel + a
  `run` driver. Verified by **compiling with `rustc` and running the binary**
  against the interpreter — the derived flash kernel and the full 13-kernel
  transformer block both compile and match (`tests/rustgen.rs`).
- **`emit_metal`** *(new)* — the derived kernel as Metal (MSL): one GPU thread
  per output point, `(m,ℓ,o)` in registers, QKᵀ in-thread. **Runs on the Apple
  GPU** and matches the interpreter to f32 tolerance. Verified on an M1 Pro:
  flash, causal flash, cosine-bias flash, quantized matmul, **RoPE flash**, and
  the **whole 14-kernel block** (`tests/metal.rs`).
- **`codegen`** *(new)* — the shared node→code core (`value`, `carrier_expr`,
  `offset`, `buffers`) behind a `Lang` trait. `rustgen` and `emit_metal` are
  now thin per-target impls, so a new op or fix is a one-place change and the
  two backends cannot drift. The refactor was behaviour-preserving (byte-
  identical GPU output).
- **Kernel families proven end-to-end (interp → compiled Rust → GPU):** flash
  attention (+ causal, + cosine relative-position bias), RMSNorm-fused GEMM,
  SwiGLU-fused down-GEMM, embedding gather, the whole transformer block,
  **quantized matmul** (int-weight dequant fused into the GEMM lift, no
  materialized dequantized weight), and **RoPE'd flash attention** (computed
  rotation matmul fused into flash — no movement ops, via named-axis pairing).
- **`emit_rust`** — the derived kernel as compilable Rust (scalar + tiled), for
  a single carrier (superseded by `rustgen` for whole schedules).

- **Movement ops (`Reindex`)** *(new)* — slice / zero-pad / reshape-split /
  windows as ONE affine-reindex operator beside `View`. Convolution is
  `window + flatten + matmul` — **one implicit-GEMM kernel, no im2col
  buffer** — pooling is `window + reduce(Max)`, and **sliding-window
  attention derives to one flash kernel** streaming the window axis (O(s·w)).
  All verified interp → compiled Rust → GPU (`tests/movement.rs`).
- **KV-cache decode runtime** *(new)* — `partition_many` (multi-output
  schedules, shared producers cut once), `runtime::Session` (persistent
  buffers; commit-after-execute = the write-after-read discipline, checked
  against `Schedule::reads()`). **T incremental decode steps == one causal
  prefill**, proven on the interpreter, on compiled Rust with caches living
  across a real host loop, and **on the GPU with persistent MTLBuffers and
  buffer-swap commits** (`tests/decode.rs`, `tests/metal.rs`).
- **top-k / scatter-add** *(new)* — compositions, not new nodes: top-k = k
  rounds of (max, mask-the-winner); scatter-add = a one-hot contraction (=
  gather's adjoint). Full sort *declined*: a data-movement network, not a
  fold (`tests/irregular.rs`).
- **Autodiff (`grad`)** *(new)* — reverse-mode over the closed basis;
  backward graphs are ordinary IR that derive/partition/execute/compile like
  any forward graph. Every rule held to central finite differences (matmul,
  softmax-CE, RMSNorm, masked attention, stride-1 conv, embedding
  scatter-add, aliased renames); attention backward **compiled via rustc AND
  dispatched on the GPU**; an SGD training loop (update fused into the
  gradient schedule, weights committed through the Session) converges
  (`tests/grad.rs`).
- **Split reductions / GROUP** *(new)* — `run_carrier_split` re-associates a
  fold into per-chunk partials + a combine stage (legal by the monoid law;
  proven equal for coupled online-softmax carriers), `emit_split_metal` runs
  the two-kernel form **on the GPU**, and `plan::split_factor` prices the
  factor off the winning kernel's own roofline — an occupancy-starved matvec
  splits, a 1024² flash does not (`tests/group.rs`). The cost model now
  distinguishes latency-hiding occupancy (compute) from memory-level
  parallelism (bandwidth: `lanes_per_block`, `Device::mem_lanes`).
- **Storage dtypes, priced AND real** *(new)* — `input_dt(name, axes,
  Dtype::I4)` declares a buffer's storage width; the planner bills each
  input's true bytes (int4 < int8 < f16 for the same GEMV), and the Metal
  backend now reads **typed device buffers**: `half` widens on load, packed
  int4 nibbles (compressed-tensors layout) unpack inside the GEMM fold with
  per-group scales fused as axis structure — the weight never exists
  dequantized (`tests/metal.rs::w4_grouped_matvec_runs_on_gpu`, bit-checked
  against the f64 oracle). The interpreter's semantics stay f64: an I4 input
  *means* its integer values; packing is a storage encoding the backend
  understands.
- **A device runtime in the library** *(new)* — `src/metal.rs`, shaped after
  tinygrad's runtime split (device / compiler / program / allocator):
  `MetalDevice` (open, chunked MSL compilation for multi-thousand-kernel
  programs, typed uploads, one-command-buffer dispatch), `MetalBuf` (O(1)
  clone = the swap-commit primitive), `Pipelines`, `Dispatch`,
  `program_dispatches`. Tests and examples all run through it; objc2-metal
  is a macOS-gated real dependency.

**Exists but narrow / unproven:**

- **`emit_cutile`** — CuTile Python for the attention-shaped family only;
  string-tested, **never run on a GPU** (one `#[ignore]`d test). Superseded by
  `emit_metal` as the GPU path on this hardware.
- **SSM/`Scan`** — classified and derivable (affine carrier), but not emitted
  and not in the interpreter beyond monoidal prefix scans.

**Absent (the honest remainder):** real int8/int4 *byte storage* (buffers of
bytes + bit-unpacking — the pricing is done, the buffer model is not),
two-pass row-resident kernels (softmax-as-output), cost-driven *cut*
placement (the split-reduction factor is priced, but `partition` does not
yet auto-invoke it or weigh extra legal cuts), autotuning/measurement,
dynamic shapes, multi-device execution (the allreduce *math* is
`run_carrier_split`'s merge; a device runtime is not built), `Scan` backward,
strided-AND-dilated window transposes.

## What "done" looks like

Two honest end states, pick the driving one:

1. **Inference on a real open-weights LLM, on a real GPU, matching HF logits.**
   The narrowest credible "this is a compiler," and the best demo.
2. **Broad kernel coverage** — the compiler auto-generates the full modern-ML
   kernel zoo (conv, attention variants, MoE, quantized matmul, …) that today
   are hand-written even in tinygrad's `extra/`.

They share ~80% of the substrate. The sequence below is ordered for (1) with
(2)'s building blocks pulled forward where they're cheap.

## Milestones

Legend: **[done]**, **[next]**, effort ~ S/M/L, and the risk each retires.

### M1 — Reference interpreter (oracle) · [done] · retires *correctness risk*
`src/interp.rs`. Nothing downstream can be trusted without a ground-truth
evaluator; now we have one, and every kernel is checked against it.

### M2 — Executable whole-schedule runtime · [done] · retires *"is it a compiler" risk*
`Schedule::execute`. Partition → run → equals reference on a real block. The
prototype now takes a model graph and produces correct numbers.

### M3 — Real device backend (CPU→GPU), executed · [done] · retires *execution risk*
The credibility step: **a kernel sanic emits runs on real hardware.**
1. **`rustgen`** — whole schedule → compiled Rust + `run` driver, verified by
   `rustc`-compile-and-run against the interpreter (flash + full block). [done]
2. **`emit_metal`** — derived kernels → MSL, **launched on the Apple GPU**,
   matched to the interpreter: plain flash, causal flash, cosine-bias flash,
   and — via `emit_schedule_metal` + a multi-kernel Swift host — the **whole
   14-kernel transformer block, every kernel dispatched on the GPU** with
   device-buffer intermediates and in-place epilogues (`tests/metal.rs`). [done]
3. *Still open under M3 (follow-ups, not blockers):* no tiling / threadgroup-
   memory use (drive it from `plan`'s row/col/batch roles); no benchmarking vs a
   naive baseline; `rustgen` and `emit_metal` duplicate the node→code emitter and
   should be unified behind one `Lang`-parameterized core.

### M4 — Basis + dtypes (unblocks quantized inference & RoPE) · [done]
Additive to the IR, no algebra changes:
- **Transcendentals `Sin`/`Cos`** — [done]. Threaded through every layer
  (`ir`, `derive` incl. `Expr`, `codegen`, `emit_cutile`, `emit_rust`) — ~2
  lines each, confirming the closed basis stays cheap to extend and total.
- **RoPE** — [done, without new IR]. Named axes make the pair/half split a
  matter of *axis structure*, not a reshape: express the head dim as a pair
  axis and apply a **computed 2×2 rotation matmul** (memory-free, from
  `iota`/`cos`/`sin`/`exp`). Because the rotations are free along the key axis,
  RoPE'd attention derives to **one fused flash kernel** — verified vs a
  hand-written RoPE reference, vs the interpreter, and **on the GPU**
  (`tests/oracle.rs`, `tests/metal.rs`). This is the payoff of axis identity:
  what others do with reshapes/concats and a separate kernel, sanic fuses.
- **Quantized dequant→matmul** — [done]. Int-quantized weights × per-channel
  scale, dequantized *inside* the GEMM lift automatically (elementwise fuses
  into the contraction). One fused kernel, no materialized dequant weight,
  no new op — verified interp / compiled Rust / GPU.
- **Storage dtype → cost** — [done]. `input_dt(…, Dtype::I8/I4)` declares a
  buffer's storage width; the planner prices each input's true bytes, so
  int-quantized weights earn their bandwidth win in the ranking
  (int4 < int8 < f16 on a memory-bound GEMV, `plan::tests`).
- **Declined, on purpose:** `Pow` (a composition — `exp(y·log x)`; the basis
  stays tiny) and `Cast`/`Bitcast` (meaningless while the oracle computes in
  f64; they arrive with byte storage). **Still open:** real int8/int4 *byte
  storage* (buffers of bytes + bit-unpacking) — the pricing and the fusion
  are proven; the buffer model is the remaining piece.

### M5 — Movement-op vocabulary (conv/pooling/windows) · [done]
One new structural operator, `Reindex`: each mapped source axis reads
`Σ coef·i(out_axis) + offset` (signed), with optional zero-padding —
slice, pad, reshape-split, windows and reversal in one node, exactly the
"lowered to index arithmetic" plan. Constructors: `slice`/`pad`/`split`/
`window`. Proven (`tests/movement.rs`, `tests/rustgen.rs`, `tests/metal.rs`):
**conv1d/conv2d = window + flatten + matmul → ONE implicit-GEMM kernel** (no
im2col buffer, padded SAME conv included), **maxpool = window + reduce**, and
**sliding-window attention → one flash kernel over the window axis** — all
verified interp → compiled Rust → GPU.

### M6 — Mutable state / KV-cache (real serving) · [done]
The IR stays pure; state lives at the runtime boundary. `partition_many`
emits multi-output schedules with shared producers cut once (cache updates +
logits reuse the same projections); `runtime::Session` owns persistent
buffers and **commits outputs after the whole step executes** — the
`AFTER(STORE(...))` write-after-read discipline made structural (a commit is
a buffer swap; `Schedule::reads()` rejects output names that would clobber a
read). The update kernel itself is pure basis (`where(t == pos, new, cache)`
via `one_hot`). **Proven: T incremental decode steps equal one causal
prefill** — interpreter, compiled Rust (caches as `Vec`s across a real host
loop, `run` returning the (ck, cv, logits) triple), and GPU (persistent
MTLBuffers, buffer-swap commits, 7 kernels/step × 6 steps). The same
mechanism runs optimizer state (see M8's SGD loop).

### M7 — Irregular ops that aren't fold+elementwise · [done, sort declined]
Decided per op, decomposition over new node kinds in every case:
- **argmax** — `Σ i·[x == max]`, promoted to `ir::argmax` (GPU-exact).
- **top-k** — k rounds of (max, mask-the-winner's-position); values and
  indices, batched routing (MoE top-1) included; partitions into one
  multi-output schedule with the rounds chained through materialized cuts.
- **scatter-add** — `ir::scatter_add`, a one-hot contraction: the inverse of
  gather with order-free collision handling — exactly gather's adjoint,
  which M8 leans on. Dense O(n·m) as a graph; atomics are a backend concern.
- **sort — declined.** A data-movement network, not a fold; nothing in an
  inference pipeline needs one (top-k covers sampling/MoE). Stated, not
  guessed at.

### M8 — Autodiff (training) · [done]
`src/grad.rs`: reverse-mode over the closed basis — one rule per op, backward
graphs in the same IR, so they derive/partition/execute/compile like any
forward graph. The structural transposes are the movement vocabulary pointing
at itself: broadcast ⟵ reduce, `Reduce(Add)` ⟵ broadcast, Max/Min ⟵ computed
winner masks, LSE ⟵ softmax Jacobian, gather ⟵ scatter-add, rename ⟵ rename,
flatten ⟵ split, slice ⟵ pad, stride-1 window ⟵ mirrored-window overlap-add.
Declines stated: `Scan` backward, `Reduce(Mul)` at zeros, strided-AND-dilated
window transposes. **Every rule held to central finite differences**; the
attention backward is partitioned, compiled via rustc, AND dispatched on the
GPU; **an SGD training loop** (the update `w − lr·∇` fused into the gradient
schedule as an epilogue, weights committed through the Session) **converges
to 1e-9 of its start loss** (`tests/grad.rs`).

### M9 — Scheduling precision · [GROUP done; realize/multi-device open]
- **Two-stage split reductions (GROUP)** — [done]. `run_carrier_split`
  re-associates any derived fold into per-chunk partials + a combine stage —
  legal by the monoid law, proven numerically for the coupled online-softmax
  carrier (the merge does the rescaling work; blocks ≤ extent keeps the −∞
  rescale edge out, same policy as `causal_mask`). `emit_split_metal` emits
  the partial/combine kernel pair (**GPU-verified** at k=4096).
  `plan::split_factor` prices the factor by repricing the *winning kernel's
  own roofline* with B× parallelism + the partials round trip — no second
  model. The cost model learned the distinction that makes this decidable:
  compute needs resident blocks (latency hiding), bandwidth needs total
  lanes in flight (`lanes_per_block`, `Device::mem_lanes`) — so the 4-lane
  matvec splits and the 1024² flash does not (`tests/group.rs`).
- **Still open:** per-axis partial realize, cost-driven placement of *cuts*
  (the priced split is not yet auto-invoked by `partition`), and
  multi-device execution — the allreduce math IS `run_carrier_split`'s
  stage-2 merge (each device folds its shard), but no device runtime exists.

## Critical path to "a real LLM on a GPU" — **REACHED**

**GPT-2 (124M), real OpenAI weights, matches HuggingFace.**
`cargo run --release --example gpt2`: the official `model.safetensors` loads
through a dependency-free reader (`src/safetensors.rs`, BF16/F16/F32 all
decode), the 12-layer network is built as plain IR — LayerNorm from basis
ops, learned positions as `gather(wpe, iota(s))`, the fused qkv weight split
host-side, GELU as a tanh composition, weight-tied logits — `partition_many`
splits it into **223 kernels in 0.18 s**, and the whole schedule dispatches
on the Apple GPU. Against a `transformers` reference
(`weights/reference.py`):

- **max |Δlogit| = 1e-4 over the full 50257-logit row** (f32 rounding), and
- **24/24 greedy tokens identical** — `"Hello, I'm a language model, not a
  programming language. I'm a language model. I'm a language model"`,
- the **bf16** round-trip of every weight (what loading a bf16 checkpoint
  means numerically) shifts logits by ≤ 0.54 and still matches all 24 greedy
  tokens — the storage-dtype story with real weights attached,
- the f64 interpreter oracle (`--oracle`) agrees with the GPU to
  **max |Δ| = 2e-4** on the same row — three independent executions
  (HF/PyTorch, sanic-GPU/f32, sanic-interp/f64) of one graph, all agreeing,
- **generation runs through the M6 KV-cache decode path**: one token per
  step (`id`/`pos` as data), 24 per-layer cache-row writes as extra schedule
  roots, commits as on-device buffer swaps — **~30 ms/token (33 tok/s) with
  tokens streamed to stdout as each dispatch lands** (byte-level BPE decoded,
  partial UTF-8 held back), 36× the full-window re-prefill this replaced.

The hunt also fixed a real backend bug the whole test suite had missed:
Metal's fast-math `tanh` goes through `exp(2x)` and returns NaN for
|x| ≳ 44 — and GPT-2's MLP activations genuinely exceed that. The emitter
now uses `precise::tanh` (regression-tested on the GPU).

## Capstone II — **Trinity-Nano (AFMoE, 5.5B), a real MoE LLM, int4-packed**

`cargo run --release --example trinity` (weights from `../nanoinfer`): a
56-layer, 128-expert MoE with grouped-query attention — the architecture
stress test, and on a 16 GB machine only possible because the 3.8 GB
compressed-tensors checkpoint **stays packed on device end to end**:

- **GQA is pure axis structure**: q as `[hk, qg, …]`, k/v as `[hk, …]` —
  the shared kv head is a shared axis variable, no repeat_kv tensor.
- **MoE routing is the M7 composition**: sigmoid scores + expert-bias top-8
  (each round a named schedule root), weights re-gathered from the raw
  scores, normalized, `route_scale`d — and the expert weights are fetched by
  `gather` **over the expert axis of the packed int4 tensors**:
  data-dependent weight selection through a typed load, still one fused
  fold per projection.
- QK-RMSNorm, sigmoid-gated attention, RoPE on sliding layers only (NoPE
  every 4th), μP embed scaling — all plain basis compositions.
- **3,947 kernels per decode step** (attention ~27, top-8 routing ~27, the
  MoE itself ~10 — a router fold plus GROUPED gate/up/down folds over a
  9-slot axis (8 routed + the shared expert as stacked index 128), one
  vector-indexed gather selecting every expert's packed weights at once);
  chunked MSL compile ~15 s cold / <1 s cached; 4.7 GB resident;
  **~4 tok/s streaming at ~250 ms/token**. QK-norms fold their flattened
  head pair in one kernel, and rotate-half RoPE is a pure `Reindex`
  (src `j2 = 1 − j2`) — no fold at all.
- **Same machine, same models — the measured ladder** (batch-1 KV decode,
  M1 Pro 16 GB):

  | GPT-2 124M | kernels/step | latency |
  |---|---|---|
  | MLX | **~164** (494 primitives − 330 views; sdpa fused, GELU mx.compile'd) | **5.3 ms/tok** (190 tok/s) |
  | torch eager MPS | 1,250 aten ops | 5.9 ms/tok (169 tok/s) |
  | sanic | 271 derived kernels | 29 ms/tok (35 tok/s) |

  | Trinity 5.5B | kernels/step | latency |
  |---|---|---|
  | nanoinfer megakernel (int4/fp8) | **1 dispatch** | ~15 ms/tok (67.5 tok/s) |
  | mlx-lm 8-bit (upstream afmoe) | **~2,733** (4,137 primitives − 1,404 views: QuantizedMatmul×503, RMSNorm×337, GatherQMM×162) | 16.1 ms/tok (62 tok/s) |
  | sanic int4 | 3,947 derived kernels | 252 ms/tok (4 tok/s) |
  | torch eager | 93,228 aten ops | 1,180 ms/tok CPU — bf16 exceeds MPS on 16 GB |

  MLX launches ~2.7k kernels per Trinity token — the same order as sanic —
  and is 15× faster: kernel count is settled as NOT the story. The gap is
  per-kernel quality (simdgroup-cooperative qmv at ~89% of bandwidth
  ceiling, fused sdpa/rms_norm) plus async pipelining of the step.

  All four GPT-2 rows emit the same greedy text. Lessons: sanic already
  dispatches FEWER kernels than eager torch (the per-expert Python loop
  probes 128 experts × 54 layers); the ~15× gap to MLX / the megakernel is
  **per-kernel quality** — their decode matvec is a simdgroup-cooperative,
  vectorized kernel at ~89% of the bandwidth ceiling (their own
  measurement), ours is one thread per output with scalar loads. That is
  the tiling/threadgroup-memory rung of the ladder, not kernel count.
- The kernel-count postmortem drove three partitioner improvements, all
  oracle-guarded: fold leaves keep CHEAP per-element arithmetic in-body
  (dequantization, masks, gathers — packed int4 never spills; 342M
  materialized elements/step became 1.4k) while transcendental chains and
  in-body contraction operands still materialize (inline, a GELU or a
  RoPE'd query recomputes per stream step); and `entanglers` now descends
  views/reindexes/gathers with the AXIS TRANSLATED at each boundary
  (below a flatten the entanglement lives on the members), placing retry
  cuts as deep as the algebra allows. Unrolled-expert baseline for
  reference: 9,443 kernels, 1.3 GB/step of gather spill, 122 s partition
  (now 10 s).
- **Validated against the HF reference**: per-position prompt logit error
  is FLAT (0.23–0.57, bf16-reference noise, not positional drift), the
  prompt-end argmax MATCHES, and the first greedy divergence is a 0.010
  top-2 near-tie — the flip class nanoinfer's own harness tolerates. Output:
  *"The capital of France is Paris.\nParis is the capital of France."*

Found along the way, stated honestly: the deferred-normalizer fusion into a
200k-vocab head is legal but unplannable (a per-slot "column" priced as
SRAM-resident — another cost-driven-cuts instance; the example cuts it
manually), and deeply *nested* routing expressions blow up the un-memoized
graph walkers (bounded by making rounds schedule roots; walker memoization
is a good future hardening).

Still open beyond the capstones: GQA-style long-context ring buffers for
sliding windows, a tokenizer *encoder* (prompts are pre-tokenized ids),
partition speed at 10k-kernel scale (~2 min), and the performance ladder
(two-pass row kernels, cost-driven cuts, tiling/threadgroup memory, kernel
dedup across isomorphic layers, autotuning, multi-device).

## Principles (don't regress these)

- **Every generated artifact is checked against the interpreter.** New emitter,
  new op, new backend — all verified numerically before it's "done." The oracle
  is the thing that lets us move fast without breaking correctness.
- **The algebra stays honest.** `Monoid`/linearity laws are load-bearing; a
  false law is a wrong kernel. Keep the enums tiny and the laws few.
- **Decline, don't guess.** Where a computation leaves the supported fragment,
  return `None`/refuse (as `derive` and `emit_cutile` already do) rather than
  emit something unverified. Coverage grows by adding provable cases.
- **Keep tinygrad's substrate, replace its criterion** (`vs_tinygrad.md`): index
  arithmetic, per-axis realize, measured tuning — with derivation where they cut.
```
