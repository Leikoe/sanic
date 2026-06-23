# Scheduling layer for the algebraic kernel engine

## Implementation report

This document specifies the **scheduler** that sits on top of the algebraic
streamability engine (see the companion report, *An algebraic engine for
deriving streaming kernels*). The algebraic engine has already solved
**legality**: it classifies every axis of the forward graph and, for every
streamable region, derives a correct-by-construction accumulator
`(Acc, ⊗, e, project)`. The scheduler solves **profitability**: given that
constrained, already-correct skeleton, it chooses how to partition the graph
into kernels and how to parameterize each one for a target device.

Codegen is **out of scope**. The scheduler's output is a fully-parameterized
schedule (fusion partition + per-kernel tile sizes, loop assignment,
stream-vs-tree choices, and memory placement) that a downstream backend lowers
to device code.

The defining property, which shapes the entire design: **every choice the
scheduler can make is legal.** It cannot emit an incorrect program, only a slow
one. The search therefore optimizes; it never validates. This removes the
hardest part of a classical scheduler (per-transformation dependence checking)
and reduces the problem from "search the space of correct programs" to "search
the space of parameterizations of a fixed correct skeleton."

---

## 1. Inputs and outputs

### 1.1 Input: the constrained skeleton

From the algebraic engine, per region of the graph:

- **Structure map**: for each (node, axis), a tag in
  `{FREE, MONOIDAL, LINEAR, OPAQUE, SEQUENTIAL}`. `LINEAR` is a `MONOIDAL`
  refinement (the fold is also a semimodule homomorphism). Tracked
  per-(node, axis), not collapsed to outputs — an axis may be contracted in one
  node and free in another.
- **Accumulators**: for each streamable axis, the derived
  `(Acc, ⊗, e, project)`. Critically, `Acc` carries a **known size** (scalar
  count / bytes per lane), which is a direct term in the SRAM feasibility
  constraint.
- **Fusable groups**: the maximal sets of nodes the engine has proven may be
  fused. These bound the partition search.

### 1.2 Output: a schedule

A schedule assigns, for the whole graph, values to the five decision-variable
families of §3. It is feasible (satisfies all hardware constraints) and ranked
best under the cost model. Concretely, per kernel: its node set (the fusion
partition), grid axes, the streamed axis, stream-vs-tree per monoidal axis, tile
sizes per axis, and buffer placement for any materialized intermediates and
OPAQUE/SEQUENTIAL state.

### 1.3 Target description

The scheduler is parameterized by a device model: SRAM/shared-memory capacity,
register file size per thread/block, HBM bandwidth, peak FLOP/s per relevant
dtype, launch overhead, and the occupancy model (threads/warps in flight as a
function of register and SRAM use). All hardware-specific numbers live here;
the rest of the scheduler is device-agnostic.

---

## 2. Principle: the algebra shrinks the problem

Three properties bought by the algebraic layer define the scheduler's shape and
must be exploited rather than re-derived:

1. **No legality check in the loop.** Candidates are correct by construction.
   The inner cost evaluation never runs dependence analysis.

2. **`|Acc|` is a known input to feasibility.** The central constraint — does a
   fused kernel's working set fit in SRAM? — is a closed-form function of tile
   sizes and the (already known) accumulator size, solvable analytically for
   tile bounds *before* search.

3. **Fusion groups are pre-identified.** The expensive "what *can* fuse" question
   is answered; the scheduler decides only "what *should* fuse," a far smaller
   space.

---

## 3. Decision variables

Exactly five families. The structure map dictates which apply where.

| # | Variable | Domain | Governed by |
|---|----------|--------|-------------|
| 1 | **Fusion boundaries** | a partition of each fusable group into kernels | highest leverage; combinatorial core |
| 2 | **Loop assignment** | which FREE axes are grid; which MONOIDAL axis is the streamed inner loop | per kernel |
| 3 | **Stream-vs-tree** | per MONOIDAL axis: sequential O(1) fold, or parallel tree-reduction | axis length + critical path |
| 4 | **Tile sizes** | block size per axis; register-vs-SRAM residency of `Acc` | hard-bounded by capacity |
| 5 | **Memory placement** | buffers at fusion cuts; KV-cache/recurrence state; gather index layout | OPAQUE + SEQUENTIAL regions |

Notes per family:

- **(1) Fusion boundaries.** The engine gives the *maximal* legal group; the
  scheduler chooses a partition of it. Fusing everything legal is frequently
  wrong — it blows the SRAM budget, collapses occupancy, or serializes work that
  wanted to overlap. Decision per fusable edge: fuse or cut. A cut materializes
  the intermediate (HBM traffic + a buffer); a fuse keeps it in SRAM.

- **(2) Loop assignment.** FREE axes are interchangeable as grid dimensions;
  the scheduler maps them to grid/block indices. The streamed axis is the
  MONOIDAL axis whose materialization you are avoiding.

- **(3) Stream-vs-tree.** Associativity licenses both. Heuristic: stream the
  long axis you are avoiding materializing (e.g. attention keys `k`);
  tree-reduce the short critical-path axis (e.g. a contraction `d`). The
  scheduler should treat this as a real decision variable, not a fixed rule,
  because the crossover depends on axis extent and occupancy.

- **(4) Tile sizes.** The continuous-ish core. Bounded hard by the SRAM and
  register constraints of §4; ranked by the cost model of §5. The accumulator
  size enters the SRAM constraint directly.

- **(5) Memory placement.** The structure map flags *where* materialization is
  forced (fusion cuts, OPAQUE gathers, SEQUENTIAL recurrence state). The
  scheduler sizes and places these buffers. For OPAQUE, use the gather index
  refinement (permutation / sorted / arbitrary) to choose coalesced vs.
  scattered handling.

---

## 4. Feasibility: hard constraints

A schedule is feasible iff every kernel fits the device. Encode these as
functions of the decision variables; they prune most of the search space
immediately and are checked before any cost evaluation.

- **SRAM / shared memory.** For each kernel:
  `Σ(live buffers) + tile_size_product × |Acc| × lanes ≤ SRAM_capacity`.
  The `|Acc|` term is supplied by the algebraic engine. Solve this for the
  maximum feasible tile along each axis analytically to seed the tile search.
- **Registers.** Per-thread register use ≤ register file; couples to tile size
  and `Acc` residency choices.
- **Occupancy floor.** Optionally require a minimum number of resident
  warps/blocks (register and SRAM use determine occupancy); treat as a soft
  constraint or a cost-model term rather than hard, since the optimum sometimes
  trades occupancy for tile size.
- **Divisibility / padding.** Tile sizes must divide axis extents or the kernel
  must handle remainder/padding; record which.

Feasibility is a constraint-satisfaction problem. Implement it as a fast
predicate `feasible(schedule) -> bool` and an analytic
`max_tile(kernel, axis) -> int` derived from the SRAM constraint.

---

## 5. Profitability: the cost model

A per-kernel analytic model, summed over the graph. **Ranking accuracy, not
absolute accuracy, is the bar** — the model only has to order candidate
schedules correctly. Minimum terms:

- **HBM traffic (dominant).** Bytes read from and written to global memory as a
  function of fusion boundaries and tile sizes. A fused kernel reads its inputs
  once and writes only its final output; a cut materializes the intermediate
  (extra write + read). This term is *why fusion pays* and must be modeled
  explicitly — it is the entire economic case for flash-style fusion.
- **Arithmetic intensity / roofline.** FLOPs ÷ bytes against device peak FLOP/s
  and bandwidth; tile sizes trade compute reuse against memory traffic.
- **Occupancy / latency hiding.** Resident warps as a function of register and
  SRAM use; penalizes tiles so large they starve parallelism.
- **Stream-vs-tree.** Streaming gives O(1) state but serial depth along the
  axis; tree gives log depth but more state and threads. Model the
  latency/parallelism tradeoff per monoidal axis.
- **Launch overhead.** A fixed per-kernel cost; penalizes over-cutting and
  balances the fusion partition.

Start from a roofline model and calibrate (see §7). Roofline typically ranks
correctly even when absolute predictions are off, which suffices for search.

---

## 6. Search

Given feasibility (§4) and the cost model (§5), search the decision variables.
Structure it as nested search: outer over fusion partition, inner over the
parameterization of each resulting kernel.

### 6.1 Inner: parameterize a fixed kernel

Given a fixed fusion group treated as one kernel, choose tile sizes, loop
assignment, and stream-vs-tree.

- Seed tile bounds from `max_tile` (§4), analytically.
- For simple kernels (single matmul, single reduction) the optimum is often
  closed-form or a tiny enumeration.
- For the rest, enumerate a bounded candidate set (powers of two within the
  feasible range) and rank by the cost model — the standard autotuning-style
  inner loop (cf. TVM Ansor, XLA tiling, Triton autotune). This machinery can be
  lifted wholesale because the candidates are already legal.

Output: the best feasible parameterization and its cost, per candidate kernel.

### 6.2 Outer: fusion partition

Choose the partition of each fusable group into kernels. This is the
combinatorial core, but bounded because the fusable groups are pre-identified
and modest in size.

- **Dynamic programming over the graph** when the cost model is local
  (fusion decisions exhibit optimal substructure): partition cost decomposes
  over cuts. This is the recommended default.
- **ILP** for exact solutions on small/critical subgraphs.
- **Greedy / beam search** for large graphs where DP is too expensive: start
  fully fused, cut the edge with the best marginal improvement until no cut
  helps (or vice versa).

Each candidate partition is costed by invoking the inner search (§6.1) on each
resulting kernel and summing, plus launch overhead.

### 6.3 OPAQUE / SEQUENTIAL memory planning

A parallel track, driven by the structure map's flags rather than the
fuse/tile search:

- buffer allocation and lifetime for intermediates at fusion cuts;
- KV-cache and recurrence-state layout for SEQUENTIAL regions (the
  autoregressive loop, scans);
- gather/scatter index layout for OPAQUE regions, using the index-structure
  refinement to choose coalesced vs. scattered access paths.

---

## 7. Build order

1. **Cost model, validated standalone.** Build the roofline + HBM-traffic +
   launch-overhead model first. Calibrate on a handful of known kernels (plain
   matmul, flash attention, fused MLP) until it *ranks* them correctly against
   measured runtimes. Everything downstream trusts this; do not proceed until
   ranking is reliable.
2. **Feasibility layer.** Encode SRAM/register/occupancy constraints as
   functions of (tile sizes, `|Acc|`, live-buffer count). Implement
   `feasible(...)` and analytic `max_tile(...)`.
3. **Inner tiling search.** Per fixed fusion group: analytic seed + bounded
   enumeration, ranked by the cost model.
4. **Outer fusion-partition search.** DP over the graph (ILP for small exact
   cases, greedy/beam for large), costing each candidate via the inner search.
5. **OPAQUE/SEQUENTIAL memory planning.** Buffer allocation at cuts, KV-cache and
   gather layout, as a parallel track.

The order is dependency-driven: the partition search (4) needs the inner search
(3) to cost candidates; both need feasibility (2) and the cost model (1).

---

## 8. Acceptance tests

The scheduler is correct when, **without any hand-written schedules**, it:

- **Reproduces flash attention** on the attention subgraph: fuse the
  `softmax ∘ reduction`, stream `k`, tree-reduce `d`, and pick a key/query
  tiling that fits SRAM — derived from the cost model and search, not a template.
- **Makes the right cut/fuse call where fusion does not pay.** For small `k`
  (where the fused kernel's overhead exceeds the materialization it avoids), it
  should choose to *cut* and materialize. Getting this boundary right — fuse
  when it pays, cut when it doesn't — is the real test; reproducing flash
  attention alone is not sufficient.
- **Reproduces fused-MLP / fused-MoE-expert** scheduling: up-proj → in-register
  activation → down-proj, hidden tensor never materialized, with a tiling that
  fits the wider intermediate.
- **Plans the autoregressive/decode and KV-cache regions** as SEQUENTIAL +
  OPAQUE with correct buffer placement, rather than attempting to fuse across
  the decode loop.
- **Scales to a full forward graph**: produces a feasible, fully-parameterized
  schedule for an entire model in acceptable compiler time, with the partition
  search bounded by the pre-identified fusable groups.

---

## 9. The soft underbelly: cost-model error

Legality is exact (proven by the algebraic engine); profitability is
*predicted*, and prediction error is where schedulers go wrong. Two mitigations,
both recommended:

- **Calibrate aggressively.** Spend real effort fitting the cost model to
  measured runtimes on the target device across representative shapes; revisit
  calibration whenever the device or dtype changes.
- **Measured tiebreak on hot kernels.** For the kernels that dominate runtime,
  have the search narrow to the top-k candidate schedules and *measure* them on
  device, picking the empirical winner. The cost model narrows to a handful; a
  cheap measurement pass breaks the tie.

The guarantee to hold onto: because legality is structural, the scheduler can
never select a *wrong* kernel — only a suboptimal one. The cost model is
deciding *which correct schedule is fastest*, so a measured tiebreak among legal
candidates is safe, cheap insurance and never a correctness risk.
