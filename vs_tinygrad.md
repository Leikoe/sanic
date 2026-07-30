# sanic vs tinygrad

A code-level comparison against tinygrad at `f315df29a` (2026-07-17, vendored
at `references/tinygrad`). All tinygrad citations are file:line in that tree.
The short version: the two projects **independently converged on the same
scheduling substrate** (kernels = shared loop ranges, movement = index
arithmetic, cut where structure breaks), and **diverge on exactly one thing**
— tinygrad's fusion criterion is *syntactic* (do loop indices flow through?)
while sanic's is *algebraic* (does the accumulator compose associatively?).
That one difference is why sanic derives FlashAttention from naive
`softmax(QKᵀ)·V` and tinygrad ships it hand-written.

## Where the designs converged (independently)

**tinygrad deleted the ShapeTracker from the production package.** The
strides+mask view machinery that defined tinygrad for years is gone (a few
tests and stale comments still name it). Movement ops
(`RESHAPE/PERMUTE/EXPAND/PAD/SHRINK/FLIP`) are now first-class graph nodes
(`tinygrad/uop/__init__.py:98-123`) that the "rangeify" pass lowers into
index arithmetic over explicit loop variables (`RANGE` UOps): SHRINK is an
index offset, PERMUTE reorders index vars, PAD attaches a validity mask,
RESHAPE is div/mod re-decomposition simplified symbolically
(`tinygrad/schedule/indexing.py:147-165`, `:132-145`). After scheduling,
movement ops no longer exist — only index expressions.

That is the same move as sanic's `View` and affine `Reindex`: reindexing as
structural index arithmetic instead of a copy. Sanic already expresses
rename/flatten, split, slice, zero-pad, flip, and sliding windows with those
two nodes. Tinygrad's version is more general because its indices, extents,
and validity masks share one symbolic UOp language. Sanic also still encodes
pure axis-order changes by adding zero-valued `Iota` arithmetic
(`TensorExpr::with_axis_order`), which is a frontend workaround rather than a
sound long-term representation of layout.

**The same fusion substrate.** In tinygrad, *two ops share a kernel iff they
share loop ranges*: a single-consumer producer inherits its consumer's
ranges and fuses (`indexing.py:211-213`); boundaries appear where ranges
can't flow. In sanic, *everything one `derive` call swallows is one kernel*,
and cuts land at the carrier's leaves. Both replaced op-category fusion
rules with a structural criterion. Sanic still has reusable algebraic patterns;
it avoids workload-named kernel templates.

**The same multi-consumer logic — theirs is finer.** When a DAG-shared node
is indexed identically by all consumers, tinygrad recomputes/fuses it
(validity masks OR'd); where consumers' index expressions differ, it
materializes **only those axes** (`indexing.py:214-238`). Sanic's partitioner
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
(`ending_ranges`, `indexing.py:240-250`). Consequences, from their own code
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

## Axis identity: positional vs occurrence-scoped

tinygrad has **no axis names**: REDUCE's arg is a *count* of leading axes,
RANGE ids are counters tagged with a hardware role (`AxisType`:
GLOBAL/LOCAL/LOOP/REDUCE/UPCAST/…, `uop/ops.py:17-20`), and einops-style
names in `rearrange`/`einsum` are surface sugar compiled straight to
permute/reshape/reduce (`mixin/movement.py:365-405`,
`mixin/op.py:417-455`). Positional axes sidestep the capture problem
entirely — and make a question like "is this axis foldable at this node"
unaskable, which is fine for tinygrad because it never asks it.

Sanic's graph is positional too. During analysis it resolves each logical
dimension to an ephemeral occurrence identified by `(node, dimension)` and
propagates that occurrence through shape-preserving structure. Carrier spans
and per-dimension classification use those occurrences; axis labels remain
diagnostic. The algebra needs dimension provenance, but it does not need names
or identifiers stored in graph nodes.

## The indexing/layout path, end to end (read 2026-07-29)

The complete mechanism, so design arguments cite the machine and not the
folklore. Six stages.

**0 — frontend.** Six movement UOps — `RESHAPE/PERMUTE/EXPAND/PAD/SHRINK/
FLIP` (`uop/__init__.py`, `GroupOp.Movement`) — tensor-graph-only. Broadcast
is *leading*-aligned: EXPAND prepends dims, REDUCE consumes N leading axes
(`uop/ops.py:399-421`). There is **no data-dependent movement op**: `gather`
is arithmetic — `index._one_hot_along_dim(n).where(x, 0).sum(-1)`
(`mixin/op.py:1029`). `mop_cleanup` (`uop/movement.py`) merges adjacent
RESHAPEs and composes PERMUTEs — closure under composition, as rewrites.

**1 — shape.** `UOp._shape` is a derived recursive property
(`uop/ops.py:301`); movement args validate there; extents are `sint`
(int | UOp) with `vmin/vmax`, so symbolic shapes ride the same expressions.

**2 — rangeify** (`schedule/indexing.py:169-280`), where shapes die and
loops are born. Reverse toposort; a realized op mints fresh `RANGE`s per
output dim; a single-consumer op *inherits its consumer's ranges* — that
inheritance IS fusion; a multi-consumer op compares per-dim symbolic index
expressions (`all_same`): equal → pass through with OR'd valids, unequal →
new range and a **partial realize on exactly those axes** (`realize_map[x] =
axis list`; the stage over a range subset lands in LOCAL memory).
`apply_movement_op` (`indexing.py:150-167`) is the semantic definition of
movement, as substitution on the range tuple: SHRINK `r+off`; PERMUTE
argsort; FLIP `(s-1)-r`; EXPAND drops leading ranges; PAD
`(r-off).valid(off ≤ r < s+off)` — **validity attaches to the index
expression itself**; RESHAPE flattens the out-ranges into one linear form
and div/mods it by the in-shape, then hands it to the simplifier with the
in-tree confession: *"this simplify is doing a lot of heavy lifting. this is
the replacement for the reshape view merging code."* `ending_ranges` is the
dependent-reduce cut (§divergence).

**3 — recovery by rewrite.** What stage 0 erased, simplify re-derives:
`reduce_collapse` turns a load-free REDUCE into closed form (arange);
`pm_load_collapse` recovers the one-hot gather as an indirect load —
`(idx≠r).where(0,e).reduce(+,r) → e[r := idx.valid(0≤idx<n)]`
(`codegen/simplify.py:126-152`); div/mod recombination re-merges reshape
chains (`uop/divandmod.py`, `symbolic.py:40-66`). The systemic consequence:
**kernel boundaries are downstream of simplifier canonicalization** — the
multi-consumer rule is structural equality of symbolic expressions, so two
equal indices the simplifier fails to canonicalize identically insert a
spurious partial realize.

**4 — buffers and layout** (`schedule/rangeify.py`). A realized value
becomes `STAGE(value, *closed_ranges)`; `flatten_bufferize` collapses the
ranges by *applying RESHAPE to the range tuple*: **every buffer is 1-D,
contiguous, row-major in stage-range order — strides do not exist as a
concept anywhere in the system.** Layout is therefore not a degree of
freedom; the only place a layout choice could live is the order of ranges at
STAGE, and nothing exercises it. (Sanic's attention defect A — V stored
sequence-innermost, #17, measured — is a decision this representation cannot
state.) Symbolic shapes allocate at `vmax` and SHRINK to the bound value
(`debuf`) — precedent for `shapes_and_indexing.md` M-E's allocate-at-cap.
Kernel splitting renumbers ranges from 0 per kernel so isomorphic kernels
dedup (`renumber_range`) — the same canonicalization as sanic's
`canonical_source`. METAL's 31-buffer limit is met by inserting bufferize
splits (`limit_bufs`, `DEVICE_MAX_BUFS`) where sanic went bindless.

**5 — scheduled loops** (`codegen/opt/postrange.py`, `codegen/gpudims.py`).
Every loop is a `RANGE` tagged with an `AxisType` (GLOBAL, WARP, LOCAL,
LOOP, GROUP_REDUCE, REDUCE, UPCAST, UNROLL, THREAD); an `OptOp` splits a
range into fresh ranges with new tags (`shift_to`); `add_gpudims` groups and
splits GLOBAL/LOCAL-tagged ranges to fit the device's 3-D launch limits and
substitutes `SPECIAL`s; stores missing a local dim are gated
`.valid(lidx==0)`. Semantic identity died at rangeify; scheduling works on
loop identities alone — the clean form of design-debt 5's separation, at the
cost that no scheduling question can ever refer back to a tensor dimension.

## What `tinyspec` teaches us about our IR

Tinygrad's formal spec is not merely its executable verifier. `spec/tinyspec.tex`
defines one UOp dialect from tensor graphs through command buffers: a UOp is
`(op, src, arg, tag)` with derived `dtype`, `shape`, `device`, `addrspace`, and
`min_max` (`spec/tinyspec.tex:41-43`, `:248-288`). The same vocabulary includes
pure movement and arithmetic, functions and tuples, loads/stores and ordering,
loop ranges, barriers, WMMA, and compiled programs (`:67-245`). Its last table
names the whole lowering pipeline from callification and rangeification through
register/memory planning and rendering (`:440-453`).

That is a more important idea than any particular UOp: **one representation
carries facts forward, while each compiler phase admits only a checked subset.**
The executable `type_verify` makes that second half real with `spec_tensor` and
`spec_program` (`tinygrad/uop/spec.py:35-44`, `:134-223`).

Sanic should learn from that without copying the single-dialect boundary
literally:

| concern | tinygrad | sanic today | design verdict |
|---|---|---|---|
| semantic unit | one UOp union across phases | nine pure `NodeKind`s, then `Carrier`, `Stage`, and `KernelSpec` | Sanic's phase separation is clearer; shared scalar/index sublanguages should stop being duplicated |
| axes | positional shapes; schedule `RANGE`s get `AxisType` | positional shapes; analysis derives ephemeral `(node, dimension)` occurrences | Sanic needs provenance for per-dimension algebra, not names stored in the graph; split loops still need schedule identities |
| reductions | `ADD/MAX/MUL` over leading positional axes | positional scalar monoids plus structurally derived coupled carriers | Sanic's core admits only operations with complete scalar semantics; product recurrences wait for product values |
| movement | explicit tensor movement ops lowered to symbolic range indices and validity masks | `View` plus affine `Reindex`, with `Gather` for data-dependent access | The convergence is good; Sanic still needs a general symbolic integer/validity language, not more movement constructors |
| effects | `LOAD/STORE/AFTER/GROUP/SINK` in the dialect | tensor DAG stays pure; ordered stages and runtime commits carry effects | Purity is a strength worth keeping; a typed lower command IR should replace implicit vector order and string-named buffers |
| derived facts | dtype, shape, device, address space, value interval, sharding axis | output axes, structure, carrier spans, storage-byte dtype, roofline | Sanic's algebraic facts are unique; dtype, symbolic extent, device and bounds facts are far too weak |
| reusable graphs | `FUNCTION/PARAM/TUPLE/GETTUPLE/CALL` | whole graphs have multiple roots; individual nodes are single-valued | Product values or multi-result reducers are needed before adding more tuple-like special reductions |

### Sanic's genuine strengths

**The algebra is semantic, executable IR.** Tinygrad records that a handful of
scalar ops are associative, but its formal language has no representation for
an accumulator with `lift/combine/project`, no slot spans, and no proof target
connecting a tree fold to the original tensor program. For the laws it fully
implements, Sanic's carrier is a real compiler object with an identity,
symbolic merge, projection, and leaves. That is why online softmax and weighted
online softmax can be *derived* from ordinary graphs instead of introduced as
opaque calls.

**Dimension occurrences make the algebra local and compositional.** The
analysis pass can classify the nth output dimension of a particular node as
free, reduced, or rebound without placing IDs in the semantic graph. Tinygrad's
positional ranges are excellent scheduling objects; Sanic additionally
reconstructs provenance because carrier derivation asks this semantic question.

**The semantic graph is pure.** A KV-cache update remains a pure expression,
and the runtime commits it only after the schedule finishes. This keeps alias
and ordering arguments out of carrier derivation. Tinygrad's one dialect gains
rewrite uniformity by including effects; Sanic gains a smaller proof surface by
introducing them only after the math is settled.

**There is an independent oracle.** Dense `eval`, executable carriers,
interpreted schedules, generated Rust, and Metal can be compared layer by
layer. Tinygrad has vastly broader tests and backends, but Sanic's explicit
`run_carrier == eval` boundary is unusually strong for the transformation it
is attempting.

### The design debt exposed by the comparison

1. **One closed basis is currently restated too many times.** `MapOp`,
   carrier `Expr`, dense evaluation, reverse mode, Rust rendering, and Metal
   rendering each encode scalar semantics. They can drift: the audit found
   `Tanh` missing from carrier `Expr`; that gap is now closed and regression
   tested, but the duplication remains. The right lesson from UOps is to share
   a typed scalar expression language and make phase capability explicit, not
   to keep six exhaustive matches synchronized by convention.
2. **Product values remain the next type-system boundary.** Reductions and
   scans now admit only scalar `Monoid`s with complete dense semantics; the old
   Argmax, Top-k, affine-compose, and non-associative semantic shortcuts are
   gone. The compiler derives Argmax's extremal-key/payload product while
   bounded Top-k and genuine product-valued recurrences wait for either product
   values or a closed reducer interface whose input, state, result, identity,
   combine, and reference semantics agree.
3. **Dynamic shape is a marker, not symbolic shape.** A dynamic extent can be
   copied from a bound input, but it cannot express products, quotients,
   inequalities, or validity. Tinygrad uses the same symbolic integer UOps for
   shapes, ranges, indices, and min/max bounds. Sanic should build one small
   integer/index language and reuse it for extents, `View`, `Reindex`, planner
   constraints, and bounds proofs.
4. **Axis order is currently smuggled through arithmetic.** *[PAID —
   `df77288`/`61228ad` made dimensions positional; `with_axis_order` and its
   `0 * Iota(axis)` terms are gone. Order is explicit `ViewDim` structure
   now (`ir::transpose`, ir.rs:716). The follow-on design lives in
   `shapes_and_indexing.md`.]*
5. **Semantic axes and scheduled loops need distinct identities.** Today a
   `KernelSpec` assigns roles directly to semantic axes. Split reductions,
   vector lanes, workgroup axes, padding, and tensor-core fragments create new
   loops that are not new tensor dimensions. Tinygrad's `AxisType` table makes
   this separation explicit. Sanic should preserve semantic-axis provenance
   while lowering to independently splittable loop ranges.
6. **The pure/effect boundary needs a typed command/effect plan.** Keeping `Store` out of
   `NodeKind` is correct, but `Vec<Stage>` execution order plus string buffer
   names will not support memory reuse, barriers, async copies, atomics, or
   multi-device dependencies cleanly. Add a typed command/buffer graph after
   partitioning; it is an execution plan, not a second tensor IR. Do not
   contaminate the algebraic tensor IR to get it.
7. **Dtype must become semantics before serious codegen.** Sanic's `Dtype`
   currently prices storage bytes while the oracle computes `f64`. That was a
   useful bootstrap, but casts, accumulation dtype, integer ops, overflow,
   vector types, and quantized loads need derived type rules like tinygrad's.

The resulting architecture is deliberately not tinygrad's single union:

```text
pure tensor DAG (semantic axes, dtype, symbolic extents)
        ↓ algebraic analysis
carrier certificate (lift / associative combine / project)
        ↓ partition + range lowering
typed command DAG (buffers, loop ranges, effects, dependencies)
        ↓ target selection
instructions / source / binary
```

The layers should share scalar and symbolic-index expressions and carry
provenance between them. They should not share every operation or admit every
node in every phase.

## Executable verification, not a frozen language spec

Sanic's `src/verify.rs` is an internal graph-boundary verifier, not a formal
language document and not a stability promise. A bottom-up DAG walk rejects
wrong `Map` arity, inconsistent declarations of one input buffer, duplicate or
zero-sized axes, ill-scoped `View` groups, wrong
flatten extents, and statically out-of-bounds unpadded affine reindexes.
Dynamic affine bounds are rechecked after graph inputs are concretized.

That borrows Tinygrad's useful discipline—malformed IR should fail at a named
boundary—from `type_verify`, without pretending Sanic's early IR is finished.
It checks structural well-formedness, not algebraic truth: carrier identity,
associativity, and reference equivalence remain separate obligations. The
verifier should change freely with the IR and stay smaller than the semantics
it protects.

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

## Line accounting, on tinygrad's own metric (2026-07-29)

tinygrad is the one project in this field that treats size as a hard
constraint: `sz.py` counts lines bearing code tokens — blanks, comments and
docstrings excluded, `runtime/autogen` and `viz/assets` skipped — and CI
fails the build over budget (`MAX_LINE_COUNT=25000` in
`.github/workflows/test.yml:220`). They sit at 22,935, 92% spent. That
makes it the fairest
available yardstick, so sanic is measured the same way: `src/` with
`#[cfg(test)]` modules stripped (tinygrad's `test/` is outside the package),
comment-only and blank lines dropped.

| | lines | tokens | tok/line |
|---|---|---|---|
| sanic `src/` | **12,563** | 113,231 | 9.0 |
| tinygrad package | **22,935** | 420,968 | 18.4 |
| tinygrad *core* (their split: all but `nn`/`llm`/`renderer`/`runtime`/`viz`) | 9,048 | 169,118 | 18.7 |
| sanic *core* (frontend + ir + analysis + derive + scheduling) | **7,673** | 69,473 | 9.1 |

Two facts, neither of them the one the raw counts suggest. **Style accounts
for half the line gap** — 9.0 tokens per line against 18.4, because rustfmt
does not write `if x: return y`. And **sanic's compiler core is smaller than
tinygrad's**: 41% of it by token. Rust spends more tokens per unit of meaning
than Python (`&`, `::`, `<>`, declared types, `Some`/`None`), so that reading
is generous to tinygrad, not to sanic.

Per subsystem, by token, the ratios are lopsided in both directions:

| subsystem | tinygrad | sanic | ratio |
|---|---|---|---|
| runtime / device drivers | 128,641 | 25,064 | 0.19× |
| renderer / codegen | 67,250 | 18,694 | 0.28× |
| IR core (`uop/` vs `ir`+`simplify`+`scalar`+`gamma`) | 45,586 | 17,630 | 0.39× |
| **scheduling** | 16,753 | 23,043 | **1.38×** |

The first two are breadth sanic does not claim — their own kernel-mode AMD/NV
drivers, five rendered ISAs. The scheduling row is the one that matters: it is
where sanic deliberately spends more, and `remove_bufferize` says why.
tinygrad's own comment above its kernel-boundary decision reads `# *** here is
where we compute the cost ***`, and what follows is `len(accessed_buffers) >
3`, `buffer_in_reduce`, `out_in_ratio < 10` (`schedule/rangeify.py:214-282`).
Three literal thresholds against sanic's Prop 4.1 inequality. Those extra
lines are the research, not a defect.

The IR row prices debts this document already names (§*design debt* 3 and
4). Three like-for-like pairs, both sides counted code-only:

| concern | tinygrad | sanic | ratio |
|---|---|---|---|
| algebraic simplification | `uop/symbolic.py` 313 | `simplify.rs` 657 | 2.1× |
| IR well-formedness | `uop/spec.py` 152 | `verify.rs` 323 | 2.1× |
| **autodiff** | `mixin/gradient.py` 114 | `grad.rs` 499 | **4.4×** |

2.1× is precisely the density ratio the totals give (18.7 tokens per line
against 9.1). Simplification and verification land on it exactly — at
parity once style is removed. **Autodiff is double the baseline**, and the
excess over it is ~259 lines.

Shape bookkeeping is 205 of them: `invert_view` 56, `broadcast_to` 46,
`reduce_to` 41, `transpose_reindex` 34, `scatter_add` 28 — 79% of the
anomaly. tinygrad writes none of it, and the reason is worth stating
precisely, because it is **not** the symbolic index language. Autodiff runs
at the tensor level, where movement ops are still first-class nodes, and
every movement gradient is a one-line rule sending the op to its inverse
*inside the same vocabulary*: RESHAPE↔RESHAPE, PAD↔SHRINK,
PERMUTE↔PERMUTE(argsort), FLIP↔FLIP, EXPAND→REDUCE
(`mixin/gradient.py:69-76`). Closure under inversion is a property of the
op set, not of the lowering. Sanic's `invert_view` is 56 hand-written lines
because `View` + `with_axis_order` is not closed: the `0·Iota` order trick
(debt 4) has no inverse image in the vocabulary, so inversion is a function
instead of a rule. `simplify.rs` carries 181 lines of the same per-dimension
bookkeeping (`preserve_shape` 79, `drop_dimension` 53,
`depends_on_dimension` 49) and still holds parity, so its rewrite content
is the leaner of the two.

**~390 lines is the standing cost of a movement vocabulary that does not
invert**, charged again to every pass that has to run structure backward.

The other half of tinygrad's shape story — dissolving movement into
loop-variable div/mod arithmetic — is *not* validated by this comparison,
and their own tree says so. The index language is where they pay, not where
they win: all of `uop/divandmod.py` (106 lines) plus `_quotient_base` /
`fold_add_divmod_recombine` (`uop/symbolic.py:40-66`) exist to re-discover,
by congruence arithmetic, factorization structure the program *stated* as a
reshape and the encoding erased — the exact mirror of sanic smuggling axis
order through `0·Iota`. Same sin, different operator: structure encoded as
arithmetic, then a simplifier hired to see through it. The representation
is also their second attempt (ShapeTracker deleted after years), still
cannot express the fusions they want (`test_auto_softmax` skipped,
`RANGEIFY>1` gated), and makes "does this axis span that subtree"
unaskable — part of why their kernel boundaries fall back to
`len(accessed_buffers) > 3`. Neither project has found the representation
in which movement inverts by construction *and* structure stays visible to
the algebra. Take the closure property; skip the substrate.

One mechanism explains most of the rest of their density: `UPat` +
`PatternMatcher` + `graph_rewrite` (~400 lines, `uop/ops.py:1237-1490`,
`:1666`) carries **731 rules across 145 matchers** — renderers, scheduler,
codegen, the verifier, autodiff, even a device backend. Rewrites are data.
Sanic hand-writes five traversals instead (`simplify::pass`,
`grad::postorder`, `derive::go_uncached`, `partition::{leaf_cuts,
shared_body_cuts, cone}`, `ir::replace_many`). Not a defect to fix by
copying: their engine amortizes over 145 matchers, sanic has five, and a
match arm is debuggable where `graph_rewrite` is not. Understand the leverage;
do not import it on faith.

## What tinygrad has that sanic doesn't (the honest column)

32,699 lines of hand-written package Python against sanic's 17,988 of Rust —
1.8×, not the 50× the repositories suggest, because **202,952 of tinygrad's
235,651 lines are machine-generated ctypes bindings** under
`runtime/autogen`. Most of the real difference is real capability: autodiff,
dtypes/quantization, symbolic shapes (`Variable` extents flowing through index
math), masks/padding, multi-device (`schedule/multi.py`), JIT, memory
planning, a dozen working backends, and correctness proven by running LLaMA.
Two specific mechanisms worth stealing beyond the per-axis realize:

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

## The Metal ICB residency lesson (written down so it is not re-learned)

tinygrad's `FIX_METAL_ICB` (`runtime/graph/metal.py:76-84`) works around a
GPU fault on M1/M2 ICB replay — "the pipelines likely need to be added to
the used resources … but I haven't figured out how" — by binding every
pipeline and issuing a **zero-size dispatch** before `executeCommandsInBuffer`
so the driver considers them used. Sanic found the first-class fix
(`fb1b54f`, 2026-07-15): **an ICB does not make its referenced allocations
resident at replay, and on Apple7/8 a non-resident *pipeline state* faults
the GPU after enough replays. `MTLBuffer` and `MTLComputePipelineState`
both conform to `MTLAllocation`, so one `MTLResidencySet` covers buffers
AND pipelines** — no dummy-dispatch trick, no per-replay `useResource`
sweep, and the set strong-references every allocation so they outlive the
graph. (Unneeded on Apple9+/M3, matching tinygrad's `needs_icb_fix` gate.)

Separately and NOT fixed by residency: >64-command *concurrent* ICBs still
proved unstable on Apple7 even with buffers+pipelines resident
(`4d200c1`, 2026-07-21, hence `MAX_INDIRECT_COMMANDS = 64`). The two
failure modes are distinct; only the first has a known clean fix.

## Measured (2026-07-12)

The claim above is now a number, on the same M1 Pro, batch-1 KV decode:

| model | tinygrad | sanic |
|---|---|---|
| GPT-2 124M | 250 kernels + 60 copies/token (their examples/gpt2.py, jit census; 310 unjitted) | **233** |
| Trinity-Nano 5.5B afmoe | 3,493/token (faithful port, f16 dequant semantics, realize-per-layer; **72,134** left purely lazy) | **1,856** |

Where the 1.9× on Trinity comes from, per component: their `Tensor.topk`
lowers to a 37-kernel bitonic-sort cascade per router (sanic: one k-best
streaming insertion per rank; it still needs a true associative two-list
merge before it licenses split/tree execution); attention is ~15
kernels/layer against one derived flash fold (the
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
