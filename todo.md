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
  normalizers (defer-div), fused elementwise, and multi-slot tuples.
  Associative carrier families are checked with
  `tree_fold == fold == reference`.
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
  rotation matmul fused into flash from positional pair dimensions).
- **`emit_rust`** — the derived kernel as compilable Rust (scalar + tiled), for
  a single carrier (superseded by `rustgen` for whole schedules).

- **Axes carry extents; labels are diagnostic** *(2026-07-17)* — the extent
  is written once at the mint
  (`axis("s", 512)`) — so every shape is derivable from any graph and the
  `Extents` side-tables are GONE from the whole pipeline. `eval(node, env)`,
  `partition(node, dev)`, `plan(node, dev)`, `grad(loss, wrt)`,
  `Schedule::execute(env)`, `Session::new()`, `Value::from_shape_fn(shape, f)`,
  `volume(node)` — every extents/f64-map parameter deleted crate-wide (the
  dual usize/f64 map wart with it). Axis labels do not participate in
  equality, hashing, broadcasting, or shape resolution. `Extent::Dynamic` considered and
  DEFERRED, stated: no consumer exists (interp/plan/emitters all need
  concrete sizes; runtime dynamism is data-dependent bounds inside fixed
  shapes — the honest-window pattern); the enum is a mechanical widening
  when the first dynamic-shape workload lands.

- **Explicit graph frontend (`src/tensor.rs`)** *(new, 2026-07-17)* — the
  graph-building surface separates symbolic `TensorExpr` from concrete
  `Tensor` data. `GraphBuilder::input` allocates dense `InputId`s; ordinary
  Rust functions compose expressions; `finish(outputs)` freezes a reusable
  `Graph`; `Graph::run([tensors])` binds concrete buffers by input order.
  Dimensions are positional: operators use integer indices, elementwise
  operations follow Torch trailing-dimension broadcasting, and `matmul`
  assumes `(m, k) @ (k, n)`. Internal axes still carry extents, but labels
  are diagnostics only. The
  frontend delegates graph semantics to the IR; `tests/tensor.rs` covers
  reusable execution, dense IDs, multiple outputs, and foreign-graph
  rejection. `examples/llama3.rs` is the current frontend fixture.

- **One-example policy while the API churns** *(2026-07-17)* — migrating
  every example on every surface change is too costly, so `llama3.rs` is the
  ONE example kept current. It is the graph-builder fixture: a compact Llama
  3 decoder graph used to iterate on modeling ergonomics. Everything
  else — gpt2, trinity, mnist, shakespeare, and the guided tours — is parked
  UNMIGRATED in `examples/attic/` (cargo doesn't build it; see its README for
  the revival recipe). The TESTS are the API-correctness net and stay fully
  green — the capstone claims those examples established (HF-matching logits,
  tok/s ladders) are history in this file, not things re-verified per commit.
  Revive the attic in one batch when the surface settles.

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
- **Storage dtypes, priced AND real** *(new)* — `input(name, axes,
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
- **Graph execution** *(new)* — `MetalDevice::capture` freezes a dispatch
  list into an `MTLIndirectCommandBuffer` (nonuniform grids via indirect
  `dispatchThreads`; hazard-aware barriers, so independent stages still
  overlap); `run_graph` replays it with ONE encoder and one
  `executeCommandsInBuffer` per step — the graph submission tinygrad/MLX
  use. Swap commits flip bindings with period two, so the decode loops
  keep one graph per step parity: after two captured steps, a Trinity
  token is two Metal calls instead of 1,856 encoder round-trips. Trinity
  236 → 200 ms/tok, GPT-2 numerics/latency unchanged, replay-stability
  GPU test (`graph_replay_matches_oracle`). Wired into the Program path
  (2026-07-19): `Program::capture` freezes a binding set into a
  `MetalReplay` — declared feedback pairs (a cache output becomes the
  next step's input) ping-pong between two eagerly captured parities;
  `SANIC_DEBUG=2` steps fall back to the per-dispatch timed dump.
  Bindless stages now capture correctly (address table at 0, per-parity
  tables) and binding offsets are guarded to u32 — an ICB wire limit
  tinygrad also rejects; a >4 GB zero-copy checkpoint slice must stay on
  direct dispatch. Every replay is error-checked and fails fast: any
  command buffer error — our own fault or an "innocent victim" discard
  from a system-wide GPU recovery (observed live) — surfaces as an `Err`
  through `run_graph`/`MetalReplay::step`; the step's writes are
  untrustworthy and a decode loop must not continue on them.
  Llama 3.2 1B decode 20.5 → 25.8 tok/s wall at 38 ms/tok GPU
  replay residency (now printed by the example), prefill 2.6×, greedy
  tokens unchanged (`captured_replay_*` GPU tests).
- **Cooperative fold schedules (M10)** *(new)* — any derived fold can
  split its streamed axis across lanes/simdgroups and lane-distribute one
  output axis, with every merge rendered from the carrier's own `combine`
  (`simd_shuffle_xor` butterflies, threadgroup rounds) — the GROUP law
  intra-kernel. Priced by the roofline on a measured `Device::m1_pro`
  profile with issue-op leaf costs; order-sensitive carriers decline.
  Trinity 211.7 → 26.0 ms/step, GPT-2 29 → 8 ms/tok, numerics pinned, four
  dedicated GPU tests (`coop_*` in `tests/metal.rs`). Details at M10 and
  `vs_mlx.md`.

**Exists but narrow / unproven:**

- **`Scan`** — narrowed: scalar-monoid prefix scans evaluate and EMIT (each point folds its own
  prefix — Rust and GPU, oracle-checked, `monoidal_prefix_scans_run_on_gpu`)
  and Add-scans DIFFERENTIATE (cumsum ⟵ reversed cumsum, held to finite
  differences). Product-valued and non-associative recurrences are absent
  until the IR can represent their inputs and step semantics honestly.
  Max/Min scan backward still needs per-prefix contribution masks.

**Absent (the honest remainder):** two-pass row-resident kernels
(softmax-as-OUTPUT: the derivation's Pe forms already carry the recipe —
slots = the same-axis normalizer folds, the per-element expression = the
raw with the running max's per-element contribution substituted by the
FINAL slot value; that substitution is the missing piece, and no current
workload materializes a softmax — greedy decode never does, and sampling
wants top-k plus a k-length host softmax — so the convention waits for a
consumer rather than being guessed), dynamic shapes, multi-device
execution (the allreduce *math* is `run_carrier_split`'s merge; a device
runtime is not built). Cut placement, `Scan` backward (Add), and
strided-AND-dilated transposes CLOSED 2026-07-13 (see milestone notes);
autotuning landed as measured device profiles + the `--bench`/`--proto`
loop (see M10/vs_mlx.md) with on-line per-kernel re-choice still open.

## Runtime timing arc — what the in-graph timer says to do (2026-07-25)

Ground truth: `SANIC_DEBUG=4` now times every kernel INSIDE the replayed
command buffer (stage-boundary GPU timestamp counters,
`MetalDevice::run_kernel_timed`), so per-kernel numbers are
production-regime for the first time. llama-3.2-1B f32 decode, M1 Pro:
**23.7ms wall, Σ kernels 19.4ms** — the difference is inter-encoder
bubbles no prior mode could see. Four work items fall out, in value order:

1. **Close the wall−Σ gap — LANDED 2026-07-25.** Measured: f32
   22.3→18.9ms/step (40.9→49.6 tok/s); bf16 23.5→15.6ms/step
   (59.7 tok/s) with `plan Σ ×1.04` — the encoder bubbles WERE most of
   the cost-model miscalibration. The wall now sits below the old
   Σ-kernels (19.4ms): independent stages really overlap. Implementation:
   `encode_graph` + `barrier_schedule` in metal.rs; both old paths
   (encoder-per-dispatch and the 64-capped ICB) deleted, `run` and replay
   share the one law, the in-file `barrier_before` unit test pins the
   phase-boundary semantics, and the whole oracle suite runs through the
   concurrent encoder. Original analysis kept below for the record.

   A 263-kernel step was 263 compute encoders in one command buffer so
   Metal's hazard tracking ordered them; each encoder boundary idled the
   GPU ~10–16µs. The neighbors' answers (both in `references/`):
   - **MLX**: ONE `DispatchTypeConcurrent` encoder spans many dispatches
     (committed every 20–50 ops); a wrapper tracks prev/next input/output
     buffer sets and emits `memoryBarrier(BarrierScopeBuffers)` before a
     dispatch only when a tracked hazard exists
     (`mlx/backend/metal/device.cpp:352-416,576`). Independent kernels
     overlap; dependent ones pay a cheap in-encoder barrier, never an
     encoder teardown.
   - **tinygrad**: the whole jitted graph is ONE concurrent-dispatch ICB
     under ONE encoder, with `setBarrier()` on EVERY command — fully
     serial, no hazard analysis (`tinygrad/runtime/graph/metal.py`). The
     Apple7/8 ICB crash — the very reason sanic caps
     `MAX_INDIRECT_COMMANDS` at 64 — is worked around there by forcing
     every pipeline resident via a zero-size dispatch before
     `executeCommandsInBuffer` (`FIX_METAL_ICB`, lines 76–84; unneeded on
     Apple9+). Its graph profiling *fakes* per-kernel times by evenly
     spacing GPUStart→End — the counter-sample timer is strictly ahead.
   History that rules the ICB route out as the foundation: sanic already
   found the first-class fix for tinygrad's crash (pipelines conform to
   `MTLAllocation` → one residency set covers buffers AND pipelines,
   `fb1b54f`; written up in `vs_tinygrad.md` § "The Metal ICB residency
   lesson") — and >64-command concurrent ICBs were STILL unstable on
   Apple7 after it (`4d200c1` → the 64 cap). The instability postdates
   the fix; ICBs are not the proper path on this hardware.

   **The proper design (decided 2026-07-25): one concurrent compute
   encoder per frozen graph, direct dispatches, barriers computed at
   capture time.** Sanic knows the whole dependency structure statically —
   MLX must track hazards dynamically because it is eager; sanic can
   compute the barrier schedule once at capture from the same
   per-allocation read/write conflict analysis the ICB path already has
   (`icb_command_needs_barrier`/`icb_resource_usages`), then replay as:
   one `MTLDispatchTypeConcurrent` encoder, `dispatchThreads` per kernel,
   `memoryBarrierWithScope(Buffers)` exactly at dependency frontiers.
   Independent stages overlap; dependent ones pay an in-encoder barrier,
   never an encoder teardown. This subsumes BOTH existing paths — delete
   the ICB machinery (u32-offset trap, 64 cap, `executeCommandsInBuffer`
   msg_send) and the encoder-per-dispatch Direct path, leaving one
   execution law for every graph size; `MetalDevice::run` takes the same
   path so the whole oracle suite gates the barrier analysis. The
   analysis becomes load-bearing for correctness (no driver hazard
   tracking inside a concurrent encoder) — slices aliasing one allocation
   must conflict, which the per-allocation usage combining already
   handles. `=4` keeps encoder-per-kernel (stage boundaries are the only
   Apple sampling points); `=2` wall adjudicates the win (~3ms ceiling).

2. **The launch-floor folds — mostly EATEN by item 1 (re-measured
   2026-07-25).** In the concurrent-encoder regime the tiny norm folds
   profile at ~700µs/step (≈25µs × 33, and the profiled regime overstates
   them — production hides part behind overlap). The two-pass fusion
   feature is no longer justified by decode perf alone; keep it parked
   for training/prefill. What the post-overlap profile DOES say
   (bf16, Σ 16.4ms profiled): the four matmul-fold families are ~95% of
   kernel time — MLP up+gate 6.3ms at 85% bw, MLP down 3.5ms at **77%**,
   logits 2.9ms at 90%, QKV/attn ~2.3ms. The remaining MLX gap is fold
   DRAM rate: P1 chunk+wide-loads and the measured tuner are the attack.

   Discovery while checking why 16 "isomorphic" layers emit 33 distinct
   norm folds (`SANIC_MSL` dump, kernel names made it visible): the
   RESIDUAL STREAM IS NEVER MATERIALIZED — layer N's norm fold re-sums
   all N+2 prior residual contributions in-register (growing arity, so
   layers genuinely aren't isomorphic and dedup is correct). At batch-1
   decode re-reading k×4KB beats a materialize round-trip — the right
   call. (WRONG, measured in item 7a: the re-reads cost 0.588 ms/step at
   16 layers — norms are 0.951 ms where arity 1 would be 0.363.) At long-context prefill the re-reads grow ~quadratically with
   depth (2k tokens: ~1.2GB extra over 16 layers) — a cost-model cut
   decision to revisit when prefill matters.

3. **The measured tuner — LANDED 2026-07-25 as `SANIC_TUNE=1`.**
   Measured: bf16 15.8→14.9ms/step (60.2→63.5 tok/s), f32
   18.7→16.1ms (51.2→59.4 tok/s); costs compile time (5.2→13.3s), so
   opt-in. `priced_fold_sched_candidates` is now the one rule with two
   readers — the analytic chooser ranks by the price, the tuner times
   the feasible entries — and `emit_schedule_metal_tuned` takes the
   verdicts.

   Two lessons worth keeping, both found by being wrong first:
   - **Solo timing is a liar.** Timing candidates as isolated dispatches
     (the obvious design, and what `run_kernel_timed` invites) picked
     winners that made the real step ~12% SLOWER: a candidate re-reading
     warm scratch rewards parallelism that a DRAM-bound step punishes.
     The instrument has to be the step — substitute the candidate into
     the full dispatch list over zeroed bindings and replay it.
   - **Every verdict must be a difference of ADJACENT measurements.** The
     GPU clock drifts across a tuning run; against a start-of-run
     baseline every family "won" ~0.5ms and the combined program shipped
     none of it. Each family re-measures its own baseline beside its
     rivals, overruling needs a 1% margin, and the final program must
     beat a fresh baseline or nothing ships.

   Still open: the analytic chooser's `chunk` refinement is applied
   post-hoc (`best_fold_sched`), so `analytic` is not always in the
   candidate list the tuner enumerates; a persistent verdict cache
   (keyed on the canonical source) would let a tuned build pay the 8s
   once.

4. **THE REMAINING PROBLEM IS SYNCHRONIZATION, NOT KERNELS (measured
   2026-07-25).** Decompose a bf16 llama decode step (15.55 ms GPU):

   | | ms | |
   |---|---|---|
   | traffic floor at 184 GB/s | 13.48 | 2480 MB, and the traffic is IRREDUCIBLE — every weight read exactly once, 0.8% overhead |
   | measured with `barriers=none` | 13.87 | **97% of practical peak** |
   | measured, real barrier schedule | 15.55 | |
   | **= synchronization** | **1.68** | **11% of the step, 212 barriers × 7.9 µs** |

   `barriers=none` is a deliberately WRONG run (races), used only as a
   floor. Its verdict: the generated kernels are essentially optimal —
   184 GB/s is the same ceiling MLX's best kernel reaches. **P2
   rows-per-thread and any further wide-load work are therefore dead
   ends; do not spend time there.** The whole remaining gap is phase
   count, and the only lever on phase count is FUSION.
   (SUPERSEDED by item 7: the 97% is an AGGREGATE measured with barriers
   off, where neighbours fill each other's ramps. Per class, in
   isolation, the MLP folds run at 83-90% of the 184 GB/s our own
   lm_head demonstrates with the same codegen. Wide loads are alive;
   only P2, which REDUCES thread count, is dead.)

   The graph is 263 dispatches in 213 serial phases (`SANIC_DEBUG=3`
   prints this). 134 of the 263 are STARVED — ≤8 of the M1 Pro's 16
   cores — while being only 8.8% of kernel time. The worst offenders,
   and the fusion each one wants:

   | starved family | n | grid | fusion |
   |---|---|---|---|
   | `fold_..._over_hidden2048` (RMS reduce) | 32 | **1 threadgroup** | two-pass row-resident (below) |
   | `map_singleton1_hidden2048` (RMS apply) | 32 | 2 | same |
   | `map_kv_heads8_cache_...` (cache write) | 32 | 5 | into rope |
   | `map_singleton1_intermediate8192` (SwiGLU) | 16 | 8 | into down-proj as a leaf |

   **The two-pass row-resident kernel is the single biggest item (64 of
   263 dispatches, ~0.8 ms).** It is already named as the honest
   remainder at line 196 with its recipe; what parked it — "no current
   workload materializes a softmax, so the convention waits for a
   consumer" — has expired: RMSNorm is that consumer, 32× per step.

   Precisely what blocks it today: `emit_cone` (partition.rs) requires
   an epilogue host of EXACTLY the cone's shape
   (`p.shape() == node.shape()`), but the RMS denominator is `[1,1]`
   while its cone is `[1,2048]`, so the apply cannot ride the reduce.
   The interpreter (`eval` broadcasts) and rustgen (separate pointwise
   pass) already handle a broadcast epilogue; only the Metal emitter
   needs the second pass — after the projection lands in a register,
   loop the threadgroup's threads over the streamed axis writing the
   epilogue.

   NOT the answer, verified by experiment: deferring the divisor into
   the consumer (`SANIC_DIVCUT=never`) is **10× slower** (170 ms) — the
   normalizer is then recomputed per OUTPUT POINT. The cost model is
   right to cut; the missing option is the third one, computing the
   normalizer once per THREADGROUP.

   **First attempt at the two-pass, and why it failed (2026-07-25).**
   Built and measured, then reverted: relax `epilogue_rides` to admit a
   host the cone broadcasts over one reduced axis (right-aligned), and
   give both Metal emitters a pass two — project into threadgroup
   memory, barrier, stripe the row across the threadgroup's threads. It
   FUSED as intended (263→230 dispatches, 212→179 barriers, llama text
   unchanged) and was still **slower: 17.4 vs 15.0 ms/tok.**

   The reason is geometric and is the thing to fix before trying again:
   a fused kernel inherits the FOLD's launch geometry. The RMS reduce
   dispatches `n_tgs = fold output volume = 1` threadgroup, so pass two
   writes 2048 elements with 256 threads where the separate map kernel
   had 2048 threads across 2 threadgroups. The fusion buys one barrier
   and pays 8× parallelism on the apply — a bad trade at these shapes.
   It also NaN'd `attention_backward_runs_on_gpu` (dq), so the
   coordinate mapping is not yet right for gradient folds.

   **Second attempt, and the law it found (`tests/block_rows_probe.rs`).**
   The reasoned fix for the geometry problem was: give a threadgroup a
   BLOCK of output rows, stage the activation row in threadgroup memory,
   fold `Σx²` once during that staging — so the normalizer amortizes over
   the block instead of being recomputed per output point. Hand-written
   MSL, both shapes checked equal, then timed:

   > three kernels 132 µs · block-rows fused **218 µs** · **0.61×**

   Blocking rows is the one thing batch-1 decode cannot afford, because
   it trades away THREADS. Per-point runs 2048 threadgroups — 524k
   threads, 8 elements each. Block-rows runs 256 — 65k threads, 64 each.
   Sustained bandwidth is outstanding bytes over latency, so 8× fewer
   threads is 8× fewer loads in flight, and 8 KB of staged activation
   caps residency per core on top of that. The projection is DRAM-bound;
   parallelism is the whole game and amortized arithmetic buys nothing.

   **The law, which now governs this whole item: on this machine a
   fusion that reduces thread count is a regression, even when it
   removes a kernel and a barrier.** That kills block-rows (and P2
   rows-per-thread with it, for a second and better reason than "the
   kernels are already at peak"). It also explains attempt one: making
   the apply ride the reduce inherited a ONE-threadgroup geometry.

   **Third direction, closed by arithmetic (no probe needed).** Fusing
   SIDEWAYS — merging independent, equally parallel kernels (q|k|v into
   one dispatch, gate|up into one) — keeps every thread, so neither law
   above touches it. But it saves nothing, and the barrier count already
   proves it: 263 dispatches with 212 barriers means **51 dispatches
   ride an earlier barrier for free**, and the layer structure predicts
   exactly 3 per layer × 16 = 48 free (k and v riding q's barrier, up
   riding gate's) plus the prologue. The match is exact. Those kernels
   are ALREADY concurrent inside their phase; merging them removes zero
   barriers and only ~0.1 ms of CPU encode, which is hidden behind the
   GPU anyway.

   **So all three fusion directions are closed**, and the 212 barriers
   are true data dependencies of a serial transformer at batch 1, each
   costing ~7.9 µs of pipeline drain. Fusion cannot remove them:
   - consumer rides producer → inherits a 1-threadgroup geometry
   - producer absorbed into consumer → trades away threads (the law)
   - independent kernels merged → already concurrent

   **What MLX does, and the correction it forces (2026-07-25).** MLX's
   `rms_norm.metal` IS the two-pass row-resident kernel — accumulate
   Σx², threadgroup-reduce, second pass writes the row; one kernel where
   sanic emits two. So the shape is right and attempt one was a bad
   implementation of a good idea. But timed head to head
   (`fused_rms_against_reduce_then_apply`):

   > reduce+apply 8.00 µs · fused, MLX's shape 7.71 µs · **1.04×**

   **0.29 µs per norm — 9 µs per step, 0.06%.** The whole normalizer
   fusion is worth nothing measurable, and that retires item 4's ~0.8 ms
   estimate for good.

   It also corrects the "7.9 µs per barrier" figure above, which was
   `(15.55 − 13.87)/212` — an average that assumed the entire gap was
   barrier overhead. A barrier between two dependent kernels actually
   costs ~0.3 µs. What `barriers=none` really buys is letting the SEVEN
   BIG DRAM-BOUND MATMULS of a layer run concurrently, filling each
   other's ramp. **The 1.68 ms is the cost of a serial dependency chain,
   not of synchronization** — and no fusion of small kernels recovers
   it, which is exactly what all three closed directions were failing to
   find.

   A persistent/megakernel would not recover it either: keeping
   threadgroups resident removes drain-and-refill, but layer N+1's
   matmul still cannot start before layer N's output exists. The
   serialization is in the model, not the runtime.

   **Conclusion: ~15.5 ms is the floor for batch-1 decode of this model
   on this machine with this decomposition, and MLX sits at the same
   floor for the same reason** (same serial chain, same seven GEMVs, its
   own kernels at the same 184 GB/s ceiling). Parity is not a
   coincidence; both implementations are bandwidth-bound on identical
   traffic with identical dependencies. Going faster requires breaking
   the serialization, not the kernels: speculative decoding (verify k
   tokens per pass), batching (>1 sequence makes every GEMV a GEMM), or
   quantization (fewer bytes, the only lever on the floor itself).
   Further single-step kernel work on this workload is not worth doing.
   (SUPERSEDED by item 7 — and note the goal is the ROOFLINE, not the
   fastest number, so all three levers named just above are out of
   scope: each moves the floor instead of reaching it. There is 3.12 ms
   between the measured step and the traffic floor, itemized per class
   in item 7.)

5. **What the GPU will actually tell us (asked and answered 2026-07-25).**
   Apple exposes 150+ counters — performance LIMITERS (ALU, buffer
   read/write, last-level cache, tile memory), bytes read from main
   memory, occupancy — and the limiter is the thing worth having: it
   says *why* a kernel is slow, where a timestamp only says how slow
   (WWDC20 "Optimize Metal apps and games with GPU counters").

   Queried directly, this M1 Pro exposes to CODE:

   > sampling points: `AtStageBoundary` only
   > counter sets: **`timestamp`, containing one counter, `GPUTimestamp`**

   No stage-utilization, no statistics, no memory or occupancy counters —
   and NOT gated by environment (tried `METAL_CAPTURE_ENABLED`,
   `MTL_CAPTURE_ENABLED`, `METAL_DEVICE_WRAPPER_TYPE`,
   `MTL_COUNTERS_ENABLED`; the set is one, always). So
   `MetalDevice::run_kernel_timed` is already spending the entire
   programmatic budget: there is no richer counter to add to sanic.

   `SANIC_GPUTRACE=<path>` (with `METAL_CAPTURE_ENABLED=1`) is the way
   through: it captures one graph replay to a `.gputrace` document that
   opens in Xcode with all 150+ counters, per dispatch, each carrying
   the descriptive kernel name this compiler generates and each command
   buffer its step label. That is how to check the claims item 4 makes
   by inference — real DRAM bytes vs our logical estimate, real
   occupancy, and which limiter each fold is actually on.

   **Reversed properly, and the conclusion above is HALF WRONG.** The
   counters are absent from *our process*, not from the machine.

   *In-process: closed, and exactly where.* The private API is all
   there — `_MTLDevice supportsGPUStatistics` → 1,
   `_MTLCommandQueue requestCounters:`, `addPerfSampleHandler:`,
   `_MTLCommandBuffer runPerfCounterCallbackWithBlock:`,
   `newSample`, `_MTLDevice resolveCounters:withRange:` — on real
   classes `AGXG13XDevice → IOGPUMetalDevice → _MTLDevice`. Every one
   returns nil/0 unentitled. Disassembling `AGXGPURawCounterImpl::init`
   and reproducing it standalone finds the wall precisely:
   `IOConnectCallStructMethod(gpu, selector 261, 72B, 72B)` →
   `kIOReturnNotPrivileged`. Neighbours 0x100–0x104 are ungated; only
   261 is privilege-checked. **Not SIP** (disabled here, still refuses)
   — AMFI enforcing a `com.apple.private.gputools.client` entitlement
   that `gputoolsserviced` carries and a self-signed binary cannot.
   Also dead: forging a set via `MTLCounterSetInternal`, MTL4 counter
   heaps (timestamp-only), `enableConsistentPerfState:`,
   `GRCCopyAllCounterSourceGroupWithError`.

   *Out-of-process: OPEN, and it works.* Instruments injects NOTHING
   into the target (verified by diffing `_dyld_image_count` and the
   full class list under four templates — byte-identical), so there was
   never anything to trace; collection runs entirely in the entitled
   daemon. Drive its CLI instead — no root, no Developer Mode:

   ```
   xcrun xctrace record --instrument "Metal GPU Counters" \
       --output run.trace --target-stdout - --launch -- <binary>
   xcrun xctrace export --input run.trace \
       --xpath '/trace-toc/run[@number="1"]/data/table[@schema="gpu-counter-value"]'
   ```

   That yields all 31 WWDC20 counters at 20 µs resolution — **ALU
   Limiter, Buffer Read/Write Limiter, GPU Last Level Cache Limiter,
   Compute Occupancy, GPU Read/Write Bandwidth** — and combined with
   `--template "Metal System Trace"` also gives encoder boundaries and
   a per-shader timeline, which is how a counter gets attributed to one
   of our named kernels. Validated on a two-phase FMA/stream workload:
   ALU Limiter 94% in the compute phase, Buffer Write Limiter 72% in
   the stream phase. Counters are GPU-WIDE (keyed by accelerator, not
   pid), so quiesce the machine. Export is ~250 MB of XML per second of
   trace — keep workloads sub-second.

   Counter-set selection lives in the `.tracetemplate`, an
   NSKeyedArchiver plist with plain int knobs: `counterprofile` (3 =
   31 limiters, 4 = 16 utilization+bandwidth; others empty) and
   `gpuperformancestate`. **Negative result: the performance state is
   recorded faithfully but does NOT move the clock on this M1 Pro** —
   Min/Med/Max measured 1241.8/1254.1/1291.7 MHz, all pinned at P6. The
   clock cannot be pinned, only measured.

   *`IOReport`: open, unprivileged, ~1 ms — and this is the one to wire
   in.* Group `"GPU Stats"`, subgroup `"GPU Performance States"`,
   channel `GPUPH`, residency per DVFS state at 24 MHz ticks; the
   index→MHz table is the device tree's `IODeviceTree:/arm-io/pmgr`
   property `voltage-states9` (here `0, 388.8, 486.0, 648.0, 777.6,
   972.0, 1296.0`). Verified: idle sits 55% OFF / 45% P1, a Metal load
   sits 100% P6 at 1296 MHz. `ticks/24e6` recovers the wall window to
   ~2%, so a bad sampling window is self-evident. **This is the sanity
   gate every benchmark in this file should have had** — the clock
   drift that forced the tuner's per-family adjacent baselines is now
   directly measurable.

   Static tables, if the cost model ever wants absolute bytes:
   `/System/Library/Extensions/AGXMetalG13X.bundle/Contents/Resources/`
   `AGXMetalStatisticsExternalA14X-counters.plist` holds 276 derived
   counters for this exact GPU — including `Bytes Read From Main
   Memory` and `L2 Bytes Read/Written`, which xctrace does not expose —
   with the mux registers in `AGXMetalPerfCountersExternal.plist` and
   the formulas in the `-derived.js` sibling.

6. **Dump leftovers.** Collapse sub-0.1% rows (a llama table is 263 rows,
   ~170 of them launch floor); print `--` for `plan ×`/`bw` where the
   launch floor dominates the measurement; number step footers so the
   cold first step (clock ramp + weight page-in) is identifiable.

7. **THE ROOFLINE, PER CLASS — and why item 4's "kernels are essentially
   optimal" is the wrong reading (2026-07-25).** The goal here is the
   ROOFLINE: 100% of bandwidth on the traffic we actually do. Not the
   fastest possible number — quantization, speculative decoding and
   batching all move the floor instead of reaching it, and are out of
   scope for this item by construction.

   Item 4 measured `barriers=none` at 13.87 ms against a 13.48 ms traffic
   floor and concluded 97% of practical peak, hence "the generated kernels
   are essentially optimal" and "any further wide-load work is a dead end;
   do not spend time there". **That conclusion does not survive a per-class
   look.** `SANIC_DEBUG=4` times each kernel in ISOLATION (encoder per
   kernel, no neighbour to overlap with), and isolated rates are not
   uniform — llama-3.2-1B bf16, warm run, Σ 16.57 ms:

   | class | MB | ms | GB/s | at 184 | slack |
   |---|---|---|---|---|---|
   | mlp gate/up | 1075.2 | 6.46 | 166 | 5.84 | 0.62 |
   | mlp down | 537.6 | 3.52 | **153** | 2.92 | 0.60 |
   | lm_head | 525.6 | 2.86 | **184** | 2.86 | 0.00 |
   | q proj | 134.4 | 0.85 | 158 | 0.73 | 0.12 |
   | o proj | 134.4 | 0.89 | 151 | 0.73 | 0.16 |
   | k/v proj | 67.2 | 0.56 | **120** | 0.37 | 0.19 |
   | small ops (norms, rope, sdpa, silu, cache) | ~5 | 1.43 | — | 0.00 | 1.43 |
   | **total** | **2474** | **16.57** | | **13.45** | **3.12** |

   The lm_head row is the control: the SAME one-thread-per-output fold
   reaches 184 GB/s when the grid is 128k wide, and 153–166 when it is
   8k. So 184 is demonstrably reachable by our own codegen, and the big
   MLP folds run at 83–90% of it. **Weight-class slack alone is 1.69 ms**
   — against the 1.68 ms item 4 attributes to the serial dependency
   chain. Those are not two problems; they are one problem seen from two
   sides. A fold that cannot saturate DRAM alone either gets its ramp
   filled by a concurrent neighbour (`barriers=none` → aggregate 97%) or
   runs below peak in isolation (real schedule → 83–90%). Item 4 closed
   BOTH doors: the chain is irreducible AND the kernels are optimal. At
   most one of those can be true, and the per-class table says it is the
   first. **The open door is raising the isolated per-kernel rate**, and
   the lever is bytes in flight (Little's law, already in `cost.rs`), so
   P1 chunk+wide-loads is alive; only P2 rows-per-thread, which REDUCES
   thread count, is dead. Do not cite item 4 to skip this.

   Work items, in measured value order:

   a. **Materialize the residual stream — ~0.4 ms/step, measured cause.**
      Norms cost 0.951 ms; held at arity 1 they would cost 0.363. The
      0.588 ms difference is the re-sum: with the residual never
      materialized, each norm re-adds every prior layer, reduce climbing
      6 → 31 µs and apply 5 → 12 µs across the 16 layers, the fold's
      `Add` count growing by exactly two per layer (1, 3, 5, …, 65).
      Cost of the fix is ~32 small adds (~0.16 ms by analogy with our
      other 2048-wide maps), or zero if the add lands in the producing
      matmul's epilogue. NOTE this contradicts item 2's standing line
      that "at batch-1 decode re-reading k×4KB beats a materialize
      round-trip — the right call": that was reasoned, not timed.
      It does NOT contradict `fused_rms_against_reduce_then_apply`
      (8.00 vs 7.71 µs, 1.04×) — that probe is correct and runs at arity
      ONE, which is exactly the regime the re-sum leaves.

   b. **Wide/vectorized loads on the MLP folds — ~1.2 ms of the 1.69.**
      gate/up 166 and down 153 against lm_head's demonstrated 184, same
      codegen, 15× fewer output lanes. More bytes in flight per lane is
      the lever.

   c. **Merge q/k/v into one projection — ~0.19 ms.** k/v at 120 GB/s is
      not a codegen fault: a 2 MB read cannot keep enough bytes in
      flight. One [3072, 2048] stream fixes it by construction. Neither
      sanic nor mlx-lm does this today.

   d. **Fuse the small ops into the streams they ride — up to 1.43 ms.**
      Norms, rope, cache writes, sdpa and silu touch ~5 MB total, i.e.
      0.03 ms of DRAM, and cost 1.43 ms purely as separate dispatches.
      sanic already HAS the mechanism: the deferred-division tuple fold
      it emits at layer 0's k/v computes Σx² alongside the projection dot
      in one pass and divides at the end — norm∘matmul in one kernel,
      exactly. It is applied at layer 0 and nowhere else, and (a) is what
      unblocks applying it everywhere. This is the megakernel direction,
      approached one fusion at a time rather than by hand-writing it.

   e. **Pipeline submission — ~1 ms, host side, orthogonal to all of the
      above.** See `vs_mlx.md` § "The two schedules, side by side": MLX
      chops a step into ~31 command buffers and commits while the CPU
      keeps encoding; sanic encodes 263 dispatches, commits once, waits.
      Moving MLX's own caps prices the effect at 17.70 ms/tok (~124
      cbufs) → 16.38 (~4) → 17.11 (1), and with chopping off MLX runs
      17.5 ms/tok, exactly sanic's. ~4 command buffers is the optimum.

   Cross-check that this is not sanic-specific: mlx-lm, hand-written for
   this chip, reconstructs to 16.42 ms on the same traffic — 75% and 76%
   of the 200 GB/s spec roofline respectively. Kernel for kernel the two
   are within 2%, and sanic's derived folds BEAT MLX's gemv on every
   attention projection and on lm_head (`vs_mlx.md` § "The kernels, class
   by class"). Being at parity with MLX is therefore not evidence of
   being at the roofline; both are a quarter short of it, for the same
   reason, and that is the gap this item exists to close.

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
3. *The M3 follow-ups all closed since:* the emitters unified behind the
   `Lang`-parameterized `codegen` core; benchmarking exists as the trinity
   `--bench`/`--proto` GPU-timestamp harnesses; and threadgroup-memory /
   simd scheduling landed as M10's cooperative fold schedules, driven from
   the carrier's own structure.

### M4 — Basis + dtypes (unblocks quantized inference & RoPE) · [done]
Additive to the IR, no algebra changes:
- **Transcendentals `Sin`/`Cos`** — [done]. Threaded through every layer
  (`ir`, `derive` incl. `Expr`, `codegen`, `emit_rust`) — ~2 lines each,
  confirming the closed basis stays cheap to extend and total.
- **RoPE** — [done, without new IR]. Explicit positional pair/half dimensions
  make the split part of the graph structure rather than a special operation:
  apply a **computed 2×2 rotation matmul** (memory-free, from
  `iota`/`cos`/`sin`/`exp`). Because the rotations are free along the key axis,
  RoPE'd attention derives to **one fused flash kernel** — verified vs a
  hand-written RoPE reference, vs the interpreter, and **on the GPU**
  (`tests/oracle.rs`, `tests/metal.rs`). This is the payoff of axis identity:
  what others do with reshapes/concats and a separate kernel, sanic fuses.
- **Quantized dequant→matmul** — [done]. Int-quantized weights × per-channel
  scale, dequantized *inside* the GEMM lift automatically (elementwise fuses
  into the contraction). One fused kernel, no materialized dequant weight,
  no new op — verified interp / compiled Rust / GPU.
- **Storage dtype → cost** — [done]. `input(…, Dtype::I8/I4)` declares a
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

### M7 — Irregular frontend compositions · [partial, sort declined]
- **argmax** — a frontend composition of max, comparison/where, `iota`, and
  min, with first-max-wins ties. The core has no Argmax operation or
  Argmax-specific carrier slot. The generic `extremum-filter` law derives its
  (maximum, minimum tied index) product carrier, so the composition is one
  kernel without teaching the compiler about Argmax.
- **top-k** — a frontend composition of repeated max/argmax and one-hot
  masking, with first-max-wins ties. The core has no Top-k operation or
  K-best carrier. Deriving the bounded ordered-pair carrier from this graph is
  an explicit completeness-ledger gap; until then the ordinary partitioned
  composition is correct but uses multiple folds.
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
- **Cost-driven cut on plan failure** — [done 2026-07-13]. A derived fold
  that is legal but UNPLANNABLE (the deferred normalizer of an RMSNorm
  fused into a 200k-vocab head prices a per-slot column as SRAM-resident)
  no longer emits `Infeasible`: `emit_fold` cuts the smallest normalizer
  application site in the body (`Div`, or `Mul`-of-`Recip` — the two
  spellings of ÷) and re-emits; each retry removes one site, and any
  feasible schedule strictly beats an infeasible stage. Trinity's manual
  `xfinal` root is GONE — the partitioner places that cut itself, same
  numerics (suite-pinned in `unplannable_norm_head_cuts_the_normalizer`).
- **Auto-invoking the two-dispatch split** — assessed and deferred, with
  the reason on record: M10's cooperative schedules apply the SAME
  re-association law intra-kernel and cover every occupancy-starved fold
  both real models have (the 4-lane matvec that motivated the split now
  lane-splits inside one kernel). The split pair's remaining niche is
  order-INSENSITIVE folds too large for one threadgroup's worth of lanes —
  no current workload has one; wire `plan::split_factor` into `partition`
  when one appears.
- **Still open:** per-axis partial realize, and multi-device execution —
  the allreduce math IS `run_carrier_split`'s stage-2 merge (each device
  folds its shard), but no device runtime exists. M10 put the same
  re-association law INSIDE kernels; the multi-device tier sits above it.

### M10 — Cooperative fold schedules · [done] · retires *kernel-quality risk*
The biggest single latency win in the project's history, and it is ONE
general mechanism, not a kernel library (`plan::FoldSched` →
`plan::fold_sched` → `emit_fused_metal_sched`): any derived fold may split
its streamed axis across simd lanes and/or simdgroups and may distribute
one output axis across the lanes — slots whose span lacks that axis are
computed once per simdgroup (the generic form of "the attention score does
not depend on the value head dim", read off `Carrier::spans`), slots that
span it vectorize per lane, and in-body contractions lane-split
(`Gen::lane_body`). EVERY merge — lane butterfly over `simd_shuffle_xor`,
threadgroup-memory rounds across simdgroups — renders the carrier's own
`combine`: the M9 re-association law made intra-kernel, so the coupled
online-softmax carrier merges by the same rescale algebra as a plain sum.
The schedule is CHOSEN, per fold, by the existing roofline over a measured
device profile (`Device::m1_pro`) with two honesty fixes the measurements
forced: leaves priced in ISSUE ops (`count_issue_ops` — loads, div/mod
index chains, gather arithmetic, not one flop per element; underpricing
recompute is what made one-thread-per-output look fine), and hardware
constants grounded in this machine's own kernels. Every current carrier merge
is symmetric and may use the split schedules. MLX's sdpa-vector and qmv shapes fall out as priced instances; so
does the *non*-change (the 200k-row lm_head stays scalar — it was already
at bandwidth). **Measured: Trinity 211.7 → 26.0 ms/step GPU (196 → 26
ms/tok wall, 38.1 tok/s), GPT-2 29 → 8 ms/tok (128 tok/s); numerics pinned
(argmax MATCH at the same Δlogit, 24/24 SEQUENCE MATCH); 137 tests, four
new `coop_*` GPU tests pinning each emitter path against the oracle.**
Emitted-vs-hand-proto headroom that remains: 1.1× on f32 matvecs (at
ceiling), 2.6× on int4 folds (vectorized `uint32` nibble loads, row
batching per simdgroup), 1.8× on flash at full window (`float4` loads) —
plus the honest-window early exit. `vs_mlx.md` has the full ledger.

## Critical path to "a real LLM on a GPU" — **REACHED**

**GPT-2 (124M), real OpenAI weights, matches HuggingFace.**
`cargo run --release --example gpt2`: the official `model.safetensors` loads
through a dependency-free reader (BF16/F16/F32 all decoded; since replaced
by the `safetensors` crate), the 12-layer network is built as plain IR — LayerNorm from basis
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
  roots, commits as on-device buffer swaps — **8 ms/token (128 tok/s) with
  tokens streamed to stdout as each dispatch lands** (byte-level BPE decoded,
  partial UTF-8 held back); this path started at ~1080 ms/token as a
  full-window re-prefill, went to 30 with the decode graph, and to 8 with
  M10's cooperative folds.

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
- **MoE routing uses the M7 frontend composition**: sigmoid scores +
  expert-bias top-8, weights re-gathered from the raw scores, normalized,
  `route_scale`d — and the expert weights are fetched by
  `gather` **over the expert axis of the packed int4 tensors**:
  data-dependent weight selection through a typed load, still one fused
  fold per projection.
- QK-RMSNorm, sigmoid-gated attention, RoPE on sliding layers only (NoPE
  every 4th), μP embed scaling — all plain basis compositions.
- The former **1,856 kernels per decode step / 38 tok/s** measurement used
  the removed Top-k semantic shortcut and is historical. The compositional
  tree must be remeasured after generic bounded-selection inference lands.
  QK-norms fold their flattened
  head pair in one kernel, and rotate-half RoPE is a pure `Reindex`
  (src `j2 = 1 − j2`) — no fold at all.
- **Same machine, same models — the measured ladder** (batch-1 KV decode,
  M1 Pro 16 GB):

  Latencies are historical measurements from before the 2026-07-20 Top-k
  cleanup (the ladder climbed 2026-07-13; see
  `vs_mlx.md` for the per-rung autopsy). The "before" figures are the
  cooperative-fold baseline this arc started from.

  | GPT-2 124M | kernels/step | latency |
  |---|---|---|
  | MLX | **~164** (494 primitives − 330 views; sdpa fused, GELU mx.compile'd) | **5.3 ms/tok** (190 tok/s) |
  | sanic | 221 derived kernels | **~8 ms/tok (median 9, best 7)** |
  | tinygrad (their examples/gpt2.py) | 250 kernels + 60 copies (jit replay census; 310 unjitted) | 98 ms/tok jitted (f32, no BEAM/HALF) |
  | torch eager MPS | 1,250 aten ops | 5.9 ms/tok (169 tok/s) |

  | Trinity 5.5B | kernels/step | latency |
  |---|---|---|
  | nanoinfer megakernel (int4/fp8) | **1 dispatch** (hand-written) | ~15 ms/tok (67.5 tok/s) |
  | **sanic int4** | **1,478 dispatches, ≈30 unique — fewest of any framework** | **~22 ms/tok (18 with `--tune`)**; 26 before |
  | mlx-lm 8-bit (upstream afmoe) | ~2,733 (4,137 primitives − 1,404 views: QuantizedMatmul×503, RMSNorm×337, GatherQMM×162) | 16.1 ms/tok (62 tok/s) |
  | tinygrad (afmoe port, f16 dequant) | 3,493 kernels in 7 jit graphs (3,438 scheduler kernels; **72,134 without a realize per layer**) | — (count-only: shrunk dims, no W4A16 path) |
  | torch eager | 93,228 aten ops | 1,180 ms/tok CPU — bf16 exceeds MPS on 16 GB |

  tinygrad methodology (weights/tinygrad_*.py, tinygrad master in a uv
  venv): the afmoe architecture ported op-for-op from nanoinfer's
  modeling_afmoe.py (56 layers, GQA 8/2, QK-norms, gated attention, dual
  sandwich norms, sigmoid router + bias top-8, shared expert, muP);
  dequantized-f16 semantics because tinygrad has no compressed-tensors
  W4A16 path (favors tinygrad — no unpack ops); dimensions shrunk 8× to
  fit, count verified size-independent (3,438 vs 3,441 at 4×); TinyJit is
  a replay cache + dispatch batcher, NOT the compiler — the scheduler and
  codegen run on every realize, and the captured graphs replay exactly
  the scheduler's kernels (3,493 ≈ 3,438 + input handling), so jitting
  changes wall time, not count. Their count drivers, measured: router
  top-8 = a 37-kernel bitonic-sort cascade (`Tensor.topk` has no k-best
  fold), attention ≈ 15 kernels/layer (no online-softmax fusion — the
  dependent-reduce cut of `vs_tinygrad.md`, now a number), norms 2 each;
  and the 72k figure is what happens to a purely-lazy 56-layer chain
  (each router sort re-walks the unrealized prefix) — the scheduling
  fragility an algebraic fusion criterion exists to remove.

  sanic now dispatches ~32% FEWER kernels than MLX on Trinity (1,856 vs
  ~2,733) — every one derived, none from a primitive library — and MLX is
  still ~13× faster: kernel count is settled as NOT the latency story.
  **The gap is now measured per kernel class** (`vs_mlx.md`, the `--bench`
  GPU-timestamp profile): the 211 ms replayed step is 55% the derived
  flash fold itself (one thread per output point recomputes the QKᵀ dot
  once per rv lane — a 128× in-thread redundancy — and streams the full
  T_MAX window), 25% the int4 MoE folds (per-nibble unpack, per-element
  scale loads, 2.3k threads), 14% the f32 attention projections (26 GB/s
  at grid 1024 — the same scalar fold hits 172 GB/s on the 200k-thread
  lm_head, so it's launch shape, not codegen). The earlier "their qmv is
  89% of bandwidth" story was the wrong suspect: at decode shapes MLX's
  small matvecs are launch-bound too (43 GB/s measured); they win on
  schedule and bytes.

  All four GPT-2 rows emit the same greedy text. `--proto` bounded the fix
  with hand-scheduled variants of sanic's own kernels, oracle-checked on
  the real weights (flash 2,362 → 18.5 µs at the SAME 256 window — parity
  with MLX's hand-written sdpa_vector, in f32; q-proj 185 → 26 µs; MoE
  gate 384 → 14 µs), and M10 then shipped it as general codegen: the
  measured step is now **26.0 ms replayed / 26 ms/tok wall**, with the
  automatic kernels within 1.1× (f32 matvecs), 2.6× (int4 folds), and
  1.8× (flash, full window) of those protos. The remaining 26 → 15.7
  ladder is itemized in `vs_mlx.md` §"The blueprint, implemented".
- The kernel-count postmortem drove three partitioner improvements, all
  oracle-guarded: fold leaves keep CHEAP per-element arithmetic in-body
  (dequantization, masks, gathers — packed int4 never spills; 342M
  materialized elements/step became 1.4k) while transcendental chains and
  in-body contraction operands still materialize (inline, a GELU or a
  RoPE'd query recomputes per stream step); and `entanglers` now descends
  views/reindexes/gathers with the AXIS TRANSLATED at each boundary
  (below a flatten the entanglement lives on the members), placing retry
  cuts as deep as the algebra allows. The count ladder then continued on
  theory, not tuning: an Argmax shortcut replaced the old
  `Σ i·[x == max]` spelling, which is tie-unsound to fuse. That shortcut and
  a later Top-k shortcut reduced routing kernels but were removed on
  2026-07-20 because they encoded frontend operations in the core. Argmax's
  one-fold behavior was recovered through the generic extremal-key/payload
  law; sharing stopped being a fusion barrier where recompute is cheap (a
  residual add per consumer is nothing next to a launch + round trip);
  transcendentals inline when their
  subtree is stream-INVARIANT (a norm's rsqrt hoists out of the loop — so
  normalized activations fuse into every consumer GEMM with no norm map
  stage at all); and gathers joined elementwise cones as in-body indexed
  loads (all 433 gather stages vanished). 9,443 → 4,143 → 3,947 → 3,515 →
  3,083 → **1,856**, numerics pinned at every step. Unrolled-expert
  baseline for reference: 9,443 kernels, 1.3 GB/step of gather spill,
  122 s partition (now 10 s).
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

**Zero-copy weights (2026-07-13, unified memory):** the library now binds
host memory as device buffers with NO upload — `MetalBuf` carries a byte
offset (several tensors alias one `MTLBuffer`), `MetalDevice::
from_bytes_nocopy` wraps any page-aligned region (`newBufferWithBytesNoCopy`
— tinygrad's external-ptr move), and `StFile::open_zero_copy` reads a
checkpoint once into a page-aligned leaked region, LEAD-PADDING it so a
header whose length isn't a multiple of 4 (GPT-2's!) still lands every
tensor at a bindable offset. GPT-2 binds wte+wpe — 158 MB including the
tied logits head — straight from the checkpoint, numerics bit-identical
(24/24), GPU test proves pointer identity (a host write is visible to the
GPU with no re-upload).

Trinity, measured honestly, does NOT benefit — and the mmap path to make
it benefit was BUILT, measured, and reverted (2026-07-13) because it lost
on this checkpoint. Two independent blockers, each proven on the metal:
(1) its 128 experts are interleaved in STRING order (e0, e1, e10, e100…),
so the stacked per-projection tensors the gathers read are not contiguous
file ranges and MUST be assembled host-side; the experts are 72% of the
file, so zero-copying the other 28% (the bf16 attn/head/embed weights,
which DID bind — 1.23 GB, numerics pinned) can't beat the simple path.
(2) a lazy mmap made the dominant cost — reading the experts — SLOWER
(scattered page faults: 14.8 s vs 5.9 s), and a sequential prefault to
fix that just faults the whole file resident, defeating the memory point.
The `Dtype::BF16` half of it SHIPPED (the weights are now checkpoint-
native and bindable); the mmap primitives did not (no caller wins). The
real unlock stands unchanged: a one-time REPACK to a device-image file
(tensors stacked and aligned exactly as the graph reads them, header
padded to 4) that every later run binds whole — bf16 is the prerequisite,
now in place; the repack is the remaining piece.

GPT-2's binding is single-copy-plus-offset, not a true mmap: its header
length isn't a multiple of 4, so every tensor sits at an unbindable
offset; `open_zero_copy` reads the file once into a lead-PADDED
page-aligned region (that region IS the device buffer via
`newBufferWithBytesNoCopy`, so the disk read lands straight in GPU
memory — no host→device copy) and binds wte+wpe at their realigned
offsets. A true mmap can't pad, so it can't serve GPT-2's misaligned
checkpoint; the padded single-copy is the honest best here.

**Ring-buffer caches — designed precisely, deferred (2026-07-13):** a
sliding window beyond T_MAX needs no new runtime machinery, because
softmax is PERMUTATION-INVARIANT over the key set — slots may live at
`pos mod W` in any order as long as the mask knows each slot's true age.
The pieces: the write index `pos mod W` arrives as a second per-step data
input (host-side mod, zero new ops); the mask reconstructs each slot's
absolute position `p(t) = pos − ((pos − t) mod W)` in-kernel, which needs
ONE basis addition (`Floor` — a 2-line-per-layer extension by the M4
discipline) since `t` varies per element; RoPE is unaffected (keys cache
pre-rotated at their own positions). Cost: the honest-window early exit
declines on a wrapped interval (streams the full W). Deferred because
neither capstone exercises ctx > its window and the fixed-shape graph
capture is unchanged either way; build it with the first long-context
model.

**Big-infra status (2026-07-13), each a decision, not silence:**
*Dynamic shapes* — the decode graphs stay SHAPE-static by design (that is
what makes ICB capture and 2-call tokens possible); dynamism enters as
DATA-dependent bounds inside fixed shapes — the honest window is the
pattern (pos read at runtime, loop bound clamped) — and grows case by
case, never by making extents symbolic everywhere. *Multi-device* — the
allreduce math is proven (`run_carrier_split`'s stage-2 merge); this
machine has one GPU, so a runtime cannot be built honestly here; declined
until hardware exists. *Per-axis partial realize* — largely subsumed:
leaf cuts with axis translation + cone-top lifting + the plan-failure
retry place materializations per-subtree already; what remains is
realizing a PREFIX of one axis, which no current workload wants.

Still open beyond the capstones: the repack-to-device-image file for
full zero-copy, and the rest of the
ladder, each
measured in `vs_mlx.md`: autotuning, multi-device, flash float4 loads,
and the rest of the packed-fold proto gap (explicit 32-bit packed-word
loads, output-row batching per simdgroup, the −8·Σx zero-point hoist).
CLIMBED: kernel dedup (2026-07-13) — canonicalized sources (entry name +
positional buffer identifiers masked) let 1,478 dispatches share ~30
entry points; MSL compile 10.3 s → 0.2 s; closes MLX's specialization
column. CLIMBED: measured tuning (2026-07-13) — `metal::tune_schedules`
times every legal FoldSched per canonical class on the model's real
buffers, verifies each against the scalar base BEFORE it may win, and a
full-step logits gate accepts or discards the tuned program; `trinity
--tune`: 450 stages overruled, 21 → **18 ms/token (56 tok/s)**, numerics
exact. CLIMBED: cone fusion via wider epilogues (2026-07-13) — a cone
rides the LAST of its producers (multi-producer epilogues: the SwiGLU
and attention-gate cones ride their up/gate folds), and epilogues render
INSIDE the fold kernel (`Gen::local_inputs` resolves the fold's own
output to a register), one dispatch per fold+epilogue. Trinity 1,856 →
**1,478 dispatches/step**, GPT-2 233 → 221; numerics bit-identical.
Partition speed itself (9.6 s at 1.5k kernels) remains open — the
walkers re-derive whole prefixes per emit; global memoization of
`structure`/`derive` is the design, deferred for churn risk. CLIMBED:
honest-window early exit (2026-07-13) — a prefix-masked rescale fold
stops at the mask edge (`pos` read at runtime, graph stays capturable);
provably bit-identical (masked tail = exact f32 no-op; coop bound
clamped to the split width so no lane merges identity — the −∞ edge);
prefill causal flash gets per-row windows from the same detector; flash
class 2.01 → 0.51 ms, GPU-tested at four positions vs the full oracle.
CLOSED: tokenizer encoder (2026-07-13) — `src/bpe.rs` (since deleted in
favor of the `tokenizers` crate), GPT-2 byte-level
BPE in the dependency-free house style (the pre-tokenizer regex
hand-rolled as its ordered-alternative matcher), held to HuggingFace
tokenizations generated on this repo's own vocab files (`tests/bpe.rs`,
unicode/emoji/whitespace gauntlet); `gpt2 --prompt "..."` now takes raw
text. Trinity's tokenizer DECLINED, stated: its pre-tokenizer is a
sequence of lookahead regexes this crate cannot honestly reproduce
without a regex engine — a wrong split silently corrupts prompts, so
trinity keeps pre-tokenized ids. CLIMBED:
chunked lane streams (2026-07-13) — `FoldSched.chunk` folds contiguous
8-element runs per lane when a packed leaf makes contiguity pay; MoE
gate/up 2.2×, down 4.4 → 3.75 ms, GPU-bit-checked
(`coop_chunked_w4_matvec`), numerics pinned (same Δ, SEQUENCE MATCH).
HISTORICAL: one-fold-per-layer top-k (2026-07-13) — the former
operation-specific selection machinery produced 432 rank kernels → 54 and
Trinity 1,968 → 1,590 kernels, 20.3 → **19.4 ms/step (~22 ms/token
wall)**. This is retained as a measurement record, not a claim about the
current generic frontend composition.
REMOVED (2026-07-20): the one-fold Argmax and Top-k results above depended on
semantic shortcuts and operation-specific carrier machinery in the core.
Argmax and Top-k are now frontend compositions of generic maps and folds.
Generic extremal-key/payload inference recovers Argmax's product carrier;
the completeness ledger keeps bounded ordered-selection inference for Top-k
open. The old Top-k kernel-count and latency figures remain historical
measurements, not claims about the current tree.
CLIMBED: f16 attention weights (2026-07-13) — the five projections upload bf16→f16
through the existing typed-load path, −413 MB, 23.3 → **20.3 ms/step
(~21 ms/token)**, and the 0.010 near-tie flips the other way: Trinity now
greedy-matches the HF reference token-for-token (SEQUENCE MATCH). CLIMBED (2026-07-13): MoE-down leaf placement —
`leaf_cuts` translates the streamed axis at every structural boundary
(flatten/split/gather, exactly as `entanglers` does) and lifts the cut to
the top of an offending elementwise cone when that materializes no more
elements (cutting the exp alone would trade one transcendental for three
in-body loads — measured slower); the sigmoid-gated attention output was
the same disease and fixed by the same rule. A cut node whose derivable
axis lives only beneath a materialization no longer re-folds it
(`best_fold` vetoes via a memoized, sharing-preserving splice probe).
Trinity 26.0 → **23.3 ms/step replayed, ~23 ms/token wall (43 tok/s)**,
MoE-down class 7.8 → 4.3 ms, numerics bit-identical (same per-position
Δlogit, argmax MATCH, same text, same 0.010 near-tie flip), GPT-2
untouched (8 ms/tok, 24/24). Kernels 1,856 → 1,968 (+112 tiny activation
cones, fully hidden by graph replay). When the sibling projections share
a contraction axis the lifted cone instead derives as ONE fold (both dot
products in one carrier, exp at project) — pinned by test.

## Capstone III — training on the GPU (`beautiful_mnist`, and the road to nanoGPT)

The inference capstones prove the forward direction; this proves training end
to end on the metal. **Training is just another graph**: `grad` transposes the
forward net into ordinary IR, and the SGD update `w − lr·∇w` rides each
gradient's fold as an epilogue, so ONE `partition_many` schedule computes the
loss and the new weights — lowered to Metal, weights committed by buffer swap
between steps (the M6 discipline, on optimizer state instead of a KV cache).

- **`beautiful_mnist`** (`cargo run --release --example mnist`) — a 784→128→10
  MLP, ReLU, softmax cross-entropy, plain minibatch SGD, trained from scratch on
  the GPU: **~97% test accuracy in ~4 s, 13 kernels/step** (forward + backward +
  all four weight updates in one schedule). The whole trainer is `mlp` +
  `cross_entropy` as plain IR, then `grad` / `simplify_many` / `partition_many` /
  `emit_metal` — the same pipeline GPT-2 inference uses.

- **The composed logsumexp now derives — forward AND backward — to the same
  kernels the hardcoded `Monoid::LogSumExp` would**, so softmax cross-entropy
  needs no gradient rule for logsumexp at all. Three pieces, each
  value-preserving and suite-pinned:
  - `derive`: a free-along-axis map wrapping only PLAIN reductions folds WHOLE
    (`m + log(Σexp)` → one carrier), guarded against contractions so flash QKᵀ
    and SwiGLU GEMMs still decompose. Composed CE forward 4→3 kernels.
  - `simplify` (`src/simplify.rs`, a CLIENT-SIDE training pass, not on the
    inference partition path — its factoring/regrouping perturb tuned inference
    fusion, measured as Trinity +82 kernels): two phases to a fixpoint with CSE.
    Phase 1 cancels the stabilizing max-shift's winner-mask — its cotangent is
    algebraically ZERO (`+g` from `+m`, `−g` from the shift), recognized via
    `Σ(k·x)=k·Σx`, `(a/b)·b=a`, and CSE of the gradient's `Σexp` with the forward
    `s`. Phase 2 reconstructs `exp(z−lse)` (the log-sum-exp identity) and, run
    over forward+backward TOGETHER (`simplify_many`, one CSE table), reuses the
    forward carrier — the backward drops to the primitive's kernel count. NOT a
    stop-gradient: the cancellation is derived.
  - `partition`: `emit_fold` no longer recomputes an online-softmax score
    contraction in-body when it is ALREADY materialized (a logits GEMM demanded
    as an output, then re-folded by the loss's logsumexp) — it reads the live
    buffer. Flash scores are never materialized elsewhere, so attention is
    untouched (plain_attention one kernel, gpt2 221 + SEQUENCE MATCH, trinity
    1478). mnist 14→13.

- **The backward-graph walkers were exponential — CLOSED** (the "walker
  memoization" hardening flagged twice above). A transformer training step's
  partition HUNG for minutes; a sampler pinned two pure axis walks
  (`ir::collect_axes` via `all_axes`, `derive::other_axis_folds`) re-expanding
  DAG-shared subtrees along every path — backward graphs share heavily, so it
  blew up (66 s to partition ONE attention weight's gradient). Memoizing both
  (per-node visited-set / cache; identical results, 152 tests green) took a
  1-block GPT training step from hanging to **0.07 s** total (grad + partition +
  simplify + emit). Unblocks transformer training. (The broader `structure` /
  `derive` global memoization for Trinity's forward partition speed stays open.)

- **`shakespeare`** (`cargo run --release --example shakespeare`) — a char-level
  nanoGPT (1 layer, 128 dim, single-head, block 64, char vocab) trained from
  scratch on the GPU: embeddings, causal self-attention, MLP(GELU), tied LM
  head, next-token cross-entropy, `grad` over every weight, fused SGD — ONE
  73-kernel schedule computes the loss and every new weight. **loss 4.15 → 2.5**,
  ~330 steps/s, weights committed by buffer swap, sampled with temperature.

  - **The core bug it surfaced — output buffers were sized by the DISPATCH GRID,
    not the tensor shape** (`emit_metal`). A cooperative/packed fold dispatches
    fewer threads than it writes elements (each thread projects a lane strip), so
    `grid_size` undercounts the allocation: `dWf` is `[f=512, dm=128]=65536` but
    dispatches 4096 threads (16 elements each), so the buffer was allocated 16×
    too small — the kernel wrote out of bounds, corrupting memory (GPU gradients
    exploded to ~3e37 while the interpreter gave correct ~1e-3). Only training
    exercises packed-fold outputs, so inference never tripped it. Fixed: size
    every stage output by `grid_of(node).1` (the product of its `output_axes`
    extents — the SHAPE), never `grid_size`, which stays for dispatch alone.
    Any node has a valid shape; allocation must key off it.

- **[next] — lift the Metal 31-buffer limit for ≥2 layers.** A 2+ layer training
  step builds a gradient-accumulation cone wider than Metal's 31-buffer bind
  limit (3L peaks at 46 inputs on one kernel); the from-scratch demo runs at
  1 layer to stay under it. Needs cone-splitting in emit/partition (spill the
  accumulation into a reduction tree of ≤30-input partials). Fine-tuning the
  124M checkpoint (full 12-layer backward + optimizer state) sits behind this.

## The completeness oracle (`tests/completeness.rs`)

The Argmax and Top-k product-state fusions were found by counting kernels
against MLX. Argmax is now derived generically; bounded Top-k remains open.
That was a process failure, named precisely: soundness always had an oracle
(everything derived runs against `eval`), completeness never did — nothing
ever checked that a DECLINE was correct. Now it is a checkable claim:

- **The criterion is semantic and universal**: h streams in one pass iff a
  constant-size sketch of the prefix determines h on every extension —
  a list homomorphism into a small carrier (Myhill–Nerode for folds). It
  never mentions the deriver.
- **The probe searches for the sketch** by collision testing over a
  quantized alphabet (the tupling method): σ-colliding prefixes whose
  futures agree on every suffix ⇒ a constructive carrier candidate whose
  components NAME the slots; a separating witness ⇒ the decline is
  justified relative to the pool, with the counterexample printed.
- **The ledger** classifies a syllabus + a random-program sweep:
  DERIVED (probe agrees, dimension audited against slot count),
  GRAPH-FORM (graph declines, carrier exists, covered by a named op —
  the two historical misses are pinned here and are found mechanically),
  JUSTIFIED (median, count-above-half-max — witnesses printed), and any
  unexplained decline-with-carrier is a RED TEST.
- **It already paid for itself**: the first run of the random sweep flagged
  thirteen declined-but-fusable programs. Ten became four new derivation
  rules — `invariant` (Σ over an unvarying axis = n·value, the count is a
  slot), `lattice` (`min_i max(z_i, c) = max(min z, c)`), `defer-add`
  (offsets commute with order reductions), `defer-scale` (extremum of c·z
  carries BOTH extrema, sign-dispatched at project) — each held to
  `run_carrier == eval` in `tests/laws.rs`. Three are pinned with
  explanations (closed-form iota sums; one alphabet-limited artifact).
- **Stated limits**: the pool, alphabet, and collision budget bound what
  the probe sees (they are printed in the report); a pass is evidence plus
  a candidate, not a proof — the derivation itself, oracle-checked, is the
  proof. The pins are open work items, not exemptions.

"What proves we won't miss more?" — nothing proves it absolutely (the
general question is undecidable); what exists now is a standing tripwire
with the right failure mode: a missed fusion is a failing test naming its
carrier, not a benchmark surprise.

## Principles (don't regress these)

- **Every generated artifact is checked against the interpreter.** New emitter,
  new op, new backend — all verified numerically before it's "done." The oracle
  is the thing that lets us move fast without breaking correctness.
- **The algebra stays honest.** `Monoid`/linearity laws are load-bearing; a
  false law is a wrong kernel. Keep the enums tiny and the laws few.
- **Decline, don't guess.** Where a computation leaves the supported fragment,
  return `None`/refuse (as `derive` already does) rather than emit something
  unverified. Coverage grows by adding provable cases.
- **Keep tinygrad's substrate, replace its criterion** (`vs_tinygrad.md`): index
  arithmetic, per-axis realize, measured tuning — with derivation where they cut.
- **Derive the kernel, then derive the schedule** (`vs_mlx.md`): launch
  geometry is chosen by pricing carrier structure — axis spans, combine
  laws, issue costs — never by matching operation shapes. A hand-written
  kernel is a measurement target, not a template.
