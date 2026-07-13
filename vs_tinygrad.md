# sanic vs tinygrad

A code-level comparison against tinygrad at `afeb5c708` (2026-07-11, vendored
at `references/tinygrad`). All tinygrad citations are file:line in that tree.
The short version: the two projects **independently converged on the same
scheduling substrate** (kernels = shared loop ranges, movement = index
arithmetic, cut where structure breaks), and **diverge on exactly one thing**
— tinygrad's fusion criterion is *syntactic* (do loop indices flow through?)
while sanic's is *algebraic* (does the accumulator compose associatively?).
That one difference is why sanic derives FlashAttention from naive
`softmax(QKᵀ)·V` and tinygrad ships it hand-written.

## Where the designs converged (independently)

**tinygrad deleted the ShapeTracker.** The strides+mask view machinery that
defined tinygrad for years is gone — zero hits in the tree. Movement ops
(`RESHAPE/PERMUTE/EXPAND/PAD/SHRINK/FLIP`) are now first-class graph nodes
(`tinygrad/uop/__init__.py:98-123`) that the "rangeify" pass lowers into
index arithmetic over explicit loop variables (`RANGE` UOps): SHRINK is an
index offset, PERMUTE reorders index vars, PAD attaches a validity mask,
RESHAPE is div/mod re-decomposition simplified symbolically
(`tinygrad/schedule/indexing.py:131-148`, `:115-128`). After scheduling,
movement ops no longer exist — only index expressions.

That is the same move as sanic's `View`: reindexing as a structural, zero-
cost operator, resolved at planning time. Sanic's `rename`/`flatten` are a
2-op subset of tinygrad's 6 (no pad/shrink/flip, no masks, no symbolic
extents).

**The same fusion substrate.** In tinygrad, *two ops share a kernel iff they
share loop ranges*: a single-consumer producer inherits its consumer's
ranges and fuses (`indexing.py:194-196`); boundaries appear where ranges
can't flow. In sanic, *everything one `derive` call swallows is one kernel*,
and cuts land at the carrier's leaves. Both replaced op-category fusion
rules with a structural criterion. Neither has a pattern library.

**The same multi-consumer logic — theirs is finer.** When a DAG-shared node
is indexed identically by all consumers, tinygrad recomputes/fuses it
(validity masks OR'd); where consumers' index expressions differ, it
materializes **only those axes** (`indexing.py:197-221`). Sanic's partitioner
materializes a shared node whole (`partition.rs`, the `shared` barrier).
Same rule, but tinygrad's per-axis partial realize is strictly more precise
— worth stealing.

**The same "view of the same buffer at two loop variables".** Sanic's
`rename(xn, s → t)` — attention reading one normalized tensor at query and
key positions — is exactly tinygrad's one-buffer-many-INDEX mechanism, and
both engines resolve it the same way: materialize once, read at two index
expressions.

## The divergence: algebraic vs syntactic fusion

tinygrad's kernel cut between dependent reductions is unconditional at
default settings: a REDUCE closes its ranges, and any producer that feeds a
*second* reduce over ended ranges is forced to materialize
(`ending_ranges`, `indexing.py:224-233`). Consequences, from their own code
and tests:

- **softmax is 3 kernels** — max, exp/sum, div — stated verbatim in
  `test/backend/test_softmax_fusion.py:161-163` ("it becomes 3 kernels").
  The single-kernel softmax in that file is *hand-rewritten* by the test
  author, and `test_auto_softmax` is `@unittest.skip("needs RANGEIFY>1")`.
- **No online softmax exists anywhere.** Even the aspirational
  `RANGEIFY>1`/`PCONTIG=2` fusions preserve the two full passes over the
  axis — they fuse the existing ops, they never *rewrite the math* into a
  running-max rescaling accumulator. FlashAttention ships as opaque
  hand-written kernels in `extra/` (`extra/gemm/amd_flash_attention.py`,
  `extra/thunder/*/fa.py`) injected via `Tensor.custom_kernel`.

This is precisely the boundary sanic's deriver crosses. Where tinygrad sees
"reduce feeding elementwise feeding reduce → cut", sanic asks whether the
accumulator can be *strengthened*: the max/exp/sum dependence becomes the
`rescale` coupling, the trailing divide becomes `defer-div`, and the whole
thing folds in one pass — legality proven by `tree_fold == fold ==
reference`, not asserted. tinygrad's cut point and sanic's `entanglers` cut
are the same *fallback*; sanic just tries the algebra first.

Fair statement of the trade: tinygrad's criterion is cheap, general over
arbitrary index arithmetic, and never needs operator laws. Sanic's needs the
laws (`Monoid`, linearity) declared per op — and pays back with derived
streaming kernels no syntactic fusion can reach.

## Axis identity: positional vs named-and-scoped

tinygrad has **no axis names**: REDUCE's arg is a *count* of leading axes,
RANGE ids are counters tagged with a hardware role (`AxisType`:
GLOBAL/LOCAL/LOOP/REDUCE/UPCAST/…, `uop/ops.py:17-20`), and einops-style
names in `rearrange`/`einsum` are surface sugar compiled straight to
permute/reshape (`mixin/movement.py:347-390`). Positional axes sidestep the
capture problem entirely — and make a question like "is this axis foldable
at this node" unaskable, which is fine for tinygrad because it never asks
it.

Sanic bets the other way: named axes carry semantic identity across the
graph, which is what the per-(node, axis) classification and the carrier's
slot spans are built on — and which is why sanic needed scoping semantics
(`Reduce` binds, `View` re-binds). The two decisions are coupled: **the
algebra needs axis identity; syntactic fusion doesn't.** Any "derive-first"
compiler will have to make sanic's choice.

## Per-kernel optimization: measured beam vs analytical enumeration

The vocabularies map almost one-to-one onto sanic's block roles
(`codegen/opt/__init__.py`, `postrange.py:125-216`):

| tinygrad OptOp | sanic role |
|---|---|
| `LOCAL` (workgroup tile) + residual `GLOBAL` | row tile + grid batch |
| `UPCAST` (register tile on output axes) | col tile / resident |
| `UNROLL` (register tile on reduce axis) | `TILE_N` stream step |
| `GROUP`/`GROUPTOP` (two-stage split reduction) | **missing in sanic** |
| `TC` (tensor cores), `PADTO`, `SWAP`, `THREAD` | not modeled |

How each side *chooses*: tinygrad has **no cost model at all** — grep
confirms no roofline anywhere. Decisions are either
`hand_coded_optimizations` (a pile of literal thresholds: 2048, 1024, %4,
≤7… `opt/heuristic.py:9-198`) or BEAM search that compiles and **times every
candidate on real hardware**, keeping the top-k by measured wall clock
(`opt/search.py:114-187`; the analytical `Estimates` exists but is used only
as a 1000×-compute sanity filter and for GFLOPS display). Kernel
*boundaries* are never searched — BEAM optimizes within a kernel; the
`remove_bufferize` boundary heuristics (`>3 buffers`, `feeds-a-reduce`,
`rangeify.py:215-282`) are fixed.

Sanic sits at the opposite pole: exhaustive enumeration of a small
structured space, ranked by an analytical roofline, no measurement.
Measurement beats a toy model on accuracy; the model beats measurement on
search cost and works without hardware. Notably, **neither system does
cost-driven kernel-boundary placement** — both partition structurally, then
optimize within kernels. That slot is open.

## What tinygrad has that sanic doesn't (the honest column)

209k lines of package Python vs sanic's ~4k of Rust, and most of the
difference is real capability: autodiff, dtypes/quantization, symbolic
shapes (`Variable` extents flowing through index math), masks/padding,
multi-device (`schedule/multi.py`), JIT, memory planning, a dozen working
backends, and correctness proven by running LLaMA. Two specific mechanisms
worth stealing beyond the per-axis realize:

1. **Computed index tensors are free.** tinygrad collapses a REDUCE with no
   loads into closed-form index arithmetic (`codegen/simplify.py:143-146`),
   so aranges and causal masks are synthesized in-register. *(Stolen: sanic
   now has `Iota`/`Lt`/`Where` in its closed basis, and the LLM example's
   causal mask is index arithmetic fused into the flash lift — zero mask
   traffic.)*
2. **`GROUP`-style split reductions.** Sanic's planner has no two-stage
   reduction role, which is the structure that makes large sums/softmaxes
   fast (and is exactly what tinygrad's matvec heuristic reaches for).
   Still open.

Also adopted after this comparison: tinygrad's closed-op-basis discipline
(sanic's `MapOp` replaced an open set of named ops, dissolving the `exp_sub`
and `silu` special forms into compositions) and integer axis identities.

## Measured (2026-07-12)

The claim above is now a number, on the same M1 Pro, batch-1 KV decode:

| model | tinygrad | sanic |
|---|---|---|
| GPT-2 124M | 250 kernels + 60 copies/token (their examples/gpt2.py, jit census; 310 unjitted) | **233** |
| Trinity-Nano 5.5B afmoe | 3,493/token (faithful port, f16 dequant semantics, realize-per-layer; **72,134** left purely lazy) | **1,856** |

Where the 1.9× on Trinity comes from, per component: their `Tensor.topk`
lowers to a 37-kernel bitonic-sort cascade per router (sanic: one k-best
fold per rank — the tuple monoid tinygrad's syntactic criterion cannot
see); attention is ~15 kernels/layer against one derived flash fold (the
dependent-reduce cut, measured); norms are un-fusable two-pass pairs where
sanic's stream-invariant rule dissolves the second pass into every
consumer GEMM. The purely-lazy 72k blowup — each sort stage re-walking the
unrealized 56-layer prefix — is the sharpest form of the argument: fusion
by graph-shape heuristics needs hand-placed realize points to not
collapse, where a criterion that *knows* what a kernel is places its own
boundaries.

## Positioning

tinygrad's own trajectory is the strongest external validation of sanic's
direction: it abandoned view bookkeeping for loop-variable index arithmetic
and structural fusion — the substrate sanic's partitioner assumes. On that
shared substrate, the frontier tinygrad explicitly does not cross (its
dependent-reduce cut, its hand-written flash kernels, its skipped
`test_auto_softmax`) is the frontier sanic's carrier derivation is built to
cross: **put an algebraic criterion where the syntactic one gives up, and
the online-softmax family of kernels stops being hand-written.** The
research program, stated against this comparison: keep tinygrad's substrate
(index arithmetic, per-axis realize, measured tuning), replace its fusion
*criterion* with derivation, and keep axis identity scoped so the algebra
stays askable.
