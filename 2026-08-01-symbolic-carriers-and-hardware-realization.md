# 2026-08-01 — Symbolic carriers and hardware realization

**Status:** research log and design direction, not an implementation plan.

This note records the full line of inquiry from the movement/indexing audit,
through Metal resource binding and the carrier prototype, to the comparison
with CuTe, CUTLASS, and ThunderKittens. It separates facts observed in the
current code and references from proposed architecture.

The conclusion in one sentence:

> Sanic's distinctive layer is the derivation of a lawful semantic carrier;
> the missing layer is a search for a target-specific, distributed,
> asynchronously scheduled realization of that carrier, with layout,
> placement, liveness, and resource use made explicit.

---

## 1. Current baseline

Sanic already has several unusually strong pieces:

- A pure tensor DAG rather than an instruction graph contaminated with
  stores, barriers, and backend operations.
- Per-subcomputation axis analysis rather than one permanent role assigned to
  a tensor dimension.
- Constructive carrier derivation: `lift`, associative `combine`, and
  `project` are executable symbolic programs, not a tag saying that a known
  kernel should be used.
- Dense evaluation, executable carriers, generated Rust, and Metal as
  independent correctness layers.
- A partitioner and analytical planner that already reason about fusion,
  recomputation, traffic, and accumulator spans.

The current carrier representation in `src/derive.rs` is:

```text
Carrier {
    slots
    leaves
    into[]
    combine[]
    identity[]
    project[]
    spans[]
    rules[]
    kinds[]
}
```

This is real executable data and is soundly useful. The concern is not that
it fails to work. The concern is that `slots` and `SlotKind` have become the
semantic vocabulary through which new constructions must pass:

```text
Plain(Monoid)
ExpShifted { max_slot }
AtExtremum { key_slot, key, ties }
```

`ExpShifted` and `AtExtremum` look like general laws only after their common
structure has been exposed. As permanent variants they risk the same growth
pattern as a compiler that adds a named iterator or kernel class for every
new access pattern.

The goal is therefore not to throw away the current carrier. It is to find a
more general source from which its product fields and merge program can be
derived.

---

## 2. Movement and indexing: Sanic versus current tinygrad

Audited tinygrad revision: `f315df29a0c4` (2026-07-17).

### 2.1 Sanic's current movement model

Sanic retains movement as structured semantic graph nodes:

- `View` stores the direction in which regrouping/permutation is affine. It
  is bijective by construction and stores a write-direction factorization.
- `Reindex` stores affine read coordinates, covering such operations as
  slicing, padding, flips, striding, splitting, and windowed reads.
- `Gather` is the separate data-dependent case. Its coordinate contains a
  tensor read and therefore is not an affine correspondence.

Movement does not force materialization. Chains are composed into address
arithmetic at the consuming kernel unless the partitioner independently
decides to cut. This avoids the `.contiguous()` cliff in systems where an
otherwise valid composition ceases to fit one strided-view representation.

The remaining defect is duplicated structural reasoning. Axis identity,
support, relocation, simplifier preservation, streaming provenance, and
gradient transposition all answer variants of "how does an index move across
this node?" through separate walks. `shapes_and_indexing.md` proposes naming
the shared object `Correspondence` and making those operations queries over
one resolver.

### 2.2 What tinygrad does

Tinygrad retains six familiar movement UOps at the tensor level:

```text
RESHAPE  PERMUTE  EXPAND  PAD  SHRINK  FLIP
```

They have compact, closed gradient rules. During range lowering,
`apply_movement_op` substitutes the scheduled ranges and movement dissolves
into symbolic integer expressions and validity conditions. Reshape introduces
div/mod where necessary.

This has two important advantages:

1. Movement gradients are closed in the same small vocabulary.
2. The symbolic integer language is shared by shapes, ranges, indices,
   validity, and min/max reasoning.

It also loses structure early. Factorizations stated by reshape become
integer expressions that later simplifiers must rediscover. Scheduling ranges
retain loop identity, but cannot directly answer a semantic question about a
particular tensor-axis occurrence.

### 2.3 The lesson

The useful synthesis is neither tinygrad's old ShapeTracker bookkeeping nor
immediately dissolving every structural fact into div/mod:

```text
semantic Correspondence
    structured, closed, cheap queries
        ↓
typed IndexExpr
    Div/Mod/validity/bounds legal here
        ↓
backend rendering
```

Keep structure stated while semantic decisions are made. Lower to one shared,
typed symbolic integer language before code generation. Do not introduce more
movement constructors unless a genuinely new correspondence class exists.

### 2.4 What a cone means in the current partitioner

An elementwise **cone** is the fusible dependency region rooted at a value.
The current `partition::cone` walks downward through:

- the root map;
- cheap maps that are legal to recompute;
- `View` and `Reindex`, which remain address arithmetic;
- `Gather`, which remains an indexed load;
- constants and coordinates, which need no buffer.

It stops at the **frontier**: reductions, already materialized results,
shared expensive maps, and other nodes that must be supplied as inputs.

So "cone" is not a new IR object or an algebraic carrier. It is a partitioning
query that finds one elementwise kernel body and its buffer frontier.

See `shapes_and_indexing.md`, `vs_tinygrad.md`, and
`src/partition.rs::cone` for the detailed audit.

---

## 3. Metal buffers, pointer tables, and argument buffers

### 3.1 The official limit

Apple's 2026 Metal Feature Set Tables state:

- The ordinary buffer argument table has **31 entries per graphics or kernel
  function** for every listed Apple GPU family.
- The threadgroup-memory argument table also has 31 entries.
- The number of buffers accessible through an **argument buffer** is a
  separate, family-dependent limit:
  - Apple1/Apple2: 31
  - Apple3/Apple4: 96
  - Apple5 and later: no limit in the table

The direct kernel signature therefore has slots `[[buffer(0)]]` through
`[[buffer(30)]]`. If one slot is the output, at most 30 direct input buffers
fit.

Sources:

- `references/metal/Metal-Feature-Set-Tables.pdf`, pages 7–8.
- `references/metal/Metal-Shading-Language-Specification.pdf`, section 2.13.

An important correction is that **Tier 2 capability alone should not be used
as a synonym for universally unlimited argument-buffer resources**. The
official family table is the authoritative capacity input.

### 3.2 Why `void *[N]` cannot simply be copied into the kernel ABI

An array of pointers does not evade the resource model merely by being
written as `void *[N]`:

- A C-style array parameter is not an inline by-value aggregate in the usual
  C ABI sense; it denotes storage reached through a pointer.
- Metal buffer pointers are shader resources with residency and address-space
  semantics, not ordinary portable scalar values.
- An inline struct containing integer addresses would still need a supported
  way to turn those integers into device pointers. Argument-buffer pointer
  members are the supported representation for this job.

The workable form is an argument-buffer struct:

```metal
struct Args {
    device const float *a;
    device const half  *b;
    // ...
};

kernel void k(constant Args &args [[buffer(0)]],
              device float *out [[buffer(1)]]) { ... }
```

The host places GPU virtual addresses into an `MTLBuffer`, binds that table at
one ordinary buffer index, makes the referenced resources resident, and the
shader loads pointer members from the table.

Sanic already implements this in `emit_metal::make_bindless` and
`metal::program_dispatches` for kernels exceeding the direct cap.

### 3.3 The indirection and its downsides

Yes, the argument buffer adds an indirection relative to a directly bound
buffer:

```text
direct bind:    resource-table entry → data
argument buffer: resource-table entry → address table → data
```

The practical costs are:

- Pointer-table loads in the shader. They are small and normally uniform, but
  they are not free.
- Host work to populate or update the address table.
- Explicit resource residency declarations such as `useResource` for
  indirectly referenced buffers.
- Scratch storage and lifetime management for the table.
- Family-dependent capacity and feature checks.
- More complicated reflection, validation, and debugging.
- Potential loss of optimization information compared with a small fixed
  directly bound signature.

The benefit is avoiding an artificial kernel split and its intermediate
traffic. Whether the indirection wins is therefore a cost comparison, not a
universal rule. Wide gradient or fusion cones are likely candidates; ordinary
narrow kernels should keep direct binding.

---

## 4. From special carrier slots to an indexed algebra

### 4.1 Join semilattice

A join semilattice is a partially ordered set in which every pair has a least
upper bound, written `a ∨ b`. Its join is associative, commutative, and
idempotent:

```text
a ∨ (b ∨ c) = (a ∨ b) ∨ c
a ∨ b       = b ∨ a
a ∨ a       = a
```

The elements need not be totally ordered. `max` over numbers is merely the
simplest example. Adding a bottom element `⊥` gives an identity:

```text
⊥ ∨ a = a
```

### 4.2 Semilattice-indexed payload monoids

Suppose a state contains:

- a key `k` in a join semilattice;
- a payload living in a coordinate system or **fiber** indexed by `k`;
- a transport operation that moves payloads from one key's fiber to a later
  common fiber;
- a monoid operation that combines payloads once they share a fiber.

For states `(k, x)` and `(k', y)`:

```text
j = k ∨ k'
(k, x) ⊗ (k', y)
    = (j, transport(k → j, x) + transport(k' → j, y))
```

The required transport laws are:

1. Transport to the same key is identity.
2. Transports compose.
3. Transport is a homomorphism of the payload monoid.

These laws make the total indexed state a monoid. The current prototype uses
one Rust payload type for every fiber, but the mathematical construction can
allow genuinely different fiber representations.

### 4.3 `ExpShifted` is one transport

For stable exponential statistics:

```text
key       = running maximum m
payload   = additive sufficient statistics measured relative to m
join      = max
transport = x ↦ exp(m_old - m_new) * x
```

The FlashAttention state can store the payload vector:

```text
[sum exp(score-m), sum exp(score-m) * value_0, ...]
```

`ExpShifted` is therefore not a primitive carrier kind. It is exponential
transport over a max-indexed additive monoid.

### 4.4 Extremal payload filtering is the same construction

For argmax with a tie-breaking payload:

```text
key       = candidate value
payload   = index, combined by min over ties
join      = max
transport = keep payload if its key survives, otherwise payload identity
```

The existing `AtExtremum` rule is thus another transport instance, not a
separate species of carrier.

### 4.5 Prototype

`examples/indexed_carrier.rs` implements:

- `Monoid`;
- `JoinSemilattice`;
- `Transport`;
- the generic `IndexedMonoid` construction;
- exponentially rebased weighted statistics;
- extremal filtering with earliest-index tie breaking;
- identity, commutativity, associativity, and decode/refinement checks.

Run it with:

```sh
cargo run --example indexed_carrier
```

It demonstrates stable finite output for scores around 10,000, where direct
exponentiation overflows, and verifies that sequential and tree folds agree
within the prototype's floating-point tolerance.

### 4.6 Representation refinement is more general than literal state equality

Let `R` be a machine representation and `S` the semantic algebra. The useful
certificate is:

```text
decode(identity_R) = identity_S
decode(a ⊗R b)     = decode(a) ⊗S decode(b)
```

Two machine states may use different numerical references yet decode to the
same semantic value. Associativity may therefore hold modulo decoded
equivalence rather than literal bitwise state equality.

ThunderKittens' Blackwell attention kernel provides a concrete example: it
does not always rebase to the exact newest maximum. A sufficiently small
maximum change may retain the old reference to avoid rescaling the persistent
output. That is a chosen numerical gauge, not the exact semilattice join in
the physical representation.

This also means algebraic laws and floating-point guarantees must be recorded
separately. Exact-real refinement, bounded error, deterministic tree order,
and bitwise equality are different contracts.

### 4.7 What remains unsolved by the indexed construction

The indexed monoid is a promising general rule, not a complete carrier
deriver. Open cases include:

- discovering the key/fiber decomposition from arbitrary symbolic programs;
- products of several dependent indexed structures;
- noncommutative carriers and order-sensitive merges;
- transports whose legality needs symbolic side conditions;
- numerical policies where the physical gauge differs from the semantic key;
- deciding which representation is profitable on a target.

---

## 5. Scalars, tensors, logical domains, and physical footprints

A scalar can be represented as a rank-zero tensor, or embedded into a
higher-rank ambient domain as a tensor whose strides are all zero. A matrix is
a rank-two tensor over two logical dimensions. `Tensor<1,1,...>` has the same
element count as a scalar but carries structural axes that a rank-zero value
does not.

This distinction matters for carrier fields.

Within the ambient FlashAttention output domain `(q, v)`:

```text
m: logical shape (Bq, Bv), layout stride (1, 0)
l: logical shape (Bq, Bv), layout stride (1, 0)
o: logical shape (Bq, Bv), ordinary storage layout
```

`m` and `l` are defined for every output coordinate `(q,v)` but all `v`
coordinates alias. Their physical footprint is `Bq`, not `Bq*Bv`.

This is better than describing them solely as `Tensor<Bq,1>` because it
preserves their relationship to the ambient logical operation while still
stating their compact storage. It also shows why Range identity cannot be
encoded only by an extent: coalescing or deleting a size-one mode must not
erase which semantic range a value came from.

The proposed field model is:

```text
CarrierField {
    dtype
    logical domain over named range factors
    layout: logical coordinates → storage coordinates/offsets
}
```

Shape describes where the field is logically meaningful. Layout describes
aliasing, broadcasting, permutation, swizzling, and footprint.

---

## 6. Block sizes are a factorization of named loop ranges

Let the block environment be:

```text
β : RangeId → block extent
```

Applying `β` does not merely replace a symbolic number. It factors each
scheduled range into hierarchical coordinates:

```text
range coordinate = (inner tile coordinate, outer/rest coordinate)
```

For FlashAttention, a candidate environment may include `Bq`, `Bk`, and
`Bv` or the equivalent named factors. It induces:

```text
persistent m/l     inner_q
persistent o       inner_q × inner_v
score temporary    inner_q × inner_k
Q staging          inner_q × contraction factors
K/V staging        inner_k × contraction/value factors
outer streamed k   rest_k
```

The logical tile sizes come from the block environment, but the physical
fragments do not follow from those shapes alone. Selecting a matrix or
reduction primitive further partitions and possibly replicates the logical
tile among lanes, SIMD groups, warps, warp groups, or other target agents.

The block environment is therefore the bridge between symbolic ranges and
logical tiled state, not a complete local-memory plan.

---

## 7. Why "does the carrier fit in local memory?" is the wrong final query

There is no single generic local memory:

- per-thread registers or private storage;
- SIMD/warp-distributed registers;
- threadgroup/shared memory;
- architecture-specific accumulator spaces such as Blackwell tensor memory;
- caches, which affect cost but are not normally explicit allocations;
- device memory used for partial states or split reductions.

A carrier may be fragmented across several of these at once and owned by
different cooperating actors. Pipeline stages replicate some fragments.
Temporaries may dominate the persistent carrier. Mutually exclusive
lifetimes may permit aliasing.

The feasibility question is:

> Does a lawful logical carrier admit a target-specific, distributed,
> asynchronously scheduled realization whose peak live fragments fit every
> relevant memory space and ownership budget?

Carrier state volume alone is not enough. The calculation must include:

- primitive-induced register fragments;
- replication across agents;
- staged input tiles;
- score and conversion temporaries;
- barrier/semaphore storage;
- epilogue redistribution;
- role-specific register allocations;
- legal lifetime aliases;
- occupancy consequences;
- optional workspace for split-range schedules.

---

## 8. CuTe audit

Audited NVlabs/CuTe revision: `1a4a2cd49ec5`.

This repository is the 2026 pure-Python PyCuTe reference algebra. It is not a
complete CUDA kernel system; the C++ hardware-facing CuTe implementation lives
inside CUTLASS.

### 8.1 Transferable ideas

1. **Layout is coordinate algebra.** A layout is a hierarchical shape plus a
   congruent hierarchical stride, mapping logical coordinates to offsets or
   coordinates.
2. **Tensor is accessor composed with layout.** The accessor supplies storage
   or generated values; the layout supplies indexing.
3. **Hierarchical shapes are factorizations.** `logical_divide` separates tile
   coordinates from the grid/rest coordinates.
4. **Zero-stride modes express broadcast/nullspace.** Logical domain and
   physical footprint can differ.
5. **Thread-value layouts are ordinary layouts.** A map from
   `(participant_id, local_value_id)` to logical tile coordinates describes
   distribution without inventing a new concept.
6. **Composition, inverse, complement, and nullspace are reusable primitive
   matching tools.** A backend can ask whether layouts compose legally rather
   than pattern-matching a list of named arrangements.
7. **Swizzles can participate in the same algebra.** They need not be opaque
   backend strings.

Useful references:

- `references/cute/docs/03_layout.md`
- `references/cute/docs/04_layout_algebra.md`
- `references/cute/docs/05_tensor.md`
- `references/cute/docs/06_swizzle.md`

### 8.2 Cautions

- CuTe's modes are largely positional; Sanic must retain named RangeIds and
  semantic provenance.
- Size-one/coalesced modes cannot be the sole carrier of axis identity.
- Some symbolic operations reject or trust conditions based on what is
  statically provable. Sanic should retain explicit guards such as
  `Divides`, `Injective`, and `Compatible`; `Unknown` is not `False`.
- Structural equality, equality as a flat integral function, and equality on
  natural hierarchical coordinates are not the same theorem.
- The C++ type-level style is not the lesson. Ordinary inspectable,
  hash-consed IR data is a better fit for Sanic.

---

## 9. CUTLASS audit

Audited CUTLASS revision: `f94ec46f4f63` (CUTLASS 4.6.1 development tree).

### 9.1 The useful hierarchy

CUTLASS 3 organizes realization approximately as:

```text
operation atom
    → tiled MMA/copy
    → collective mainloop or epilogue
    → kernel and outer work schedule
```

An MMA atom contains more than an instruction name. Its traits describe
logical operand and result types, logical shape, participating agents, and
thread-value layouts. A tiled operation replicates and composes atoms over a
larger tile.

This motivates a Sanic target contract:

```text
PrimitiveAtom {
    semantic operation
    operand/result tensor patterns
    dtypes
    participant domain
    thread/value distributions
    required memory spaces
    layout and alignment constraints
    async completion and synchronization effects
    temporary resources
    cost model
}
```

### 9.2 Logical carrier versus physical fragment

CUTLASS FlashAttention begins with logical block shapes, but its persistent
output accumulator is allocated through the selected tiled MMA's accumulator
fragment. Even row-max and row-sum state and their cross-lane reductions are
derived from the accumulator's physical thread-value layout.

Therefore Sanic needs two distinct objects:

```text
CarrierStateSchema[β]                 logical
CarrierRealization[primitive, agents] physical
```

### 9.3 Liveness and pipeline lessons

CUTLASS contains all three important memory patterns:

- Sequential phases use `max(mainloop, epilogue)` storage when lifetimes do
  not overlap.
- Ping-pong producer/consumer kernels hold phase storage simultaneously.
- FMHA aliases mutually exclusive K and V shared-memory tiles with a union.

Pipeline stage count is computed from tile bytes plus barrier storage after
subtracting other shared-memory carveouts. It cannot be chosen independently
of tile size, primitive, epilogue, scheduler, and carrier placement.

The pipeline lifecycle is explicit:

```text
producer_acquire
producer_commit
consumer_wait
consumer_release
```

These are scheduled effects and dependencies, not properties of the semantic
carrier.

### 9.4 Lessons from CUTLASS's mistakes

CUTLASS documents several mistakes worth avoiding:

- CUTLASS 2.x mirrored a threadblock/warp/thread hardware hierarchy. Hopper
  warp-group instructions did not fit it cleanly.
- Bespoke iterators and named mainloop types proliferated combinatorially.
- Tag dispatch created extension points but did not create generic synthesis;
  architecture/algorithm specializations remained handwritten.
- `StageCountAuto` is a local convenience heuristic, not a global optimizer.
- Template metaprogramming imposed learning and compilation costs large enough
  to motivate the Python CuTe DSL.
- FMHA remains a handwritten realization. CUTLASS does not derive its online
  carrier algebra.

This is strong evidence against turning every new algebraic pattern into a
Rust enum variant or trait specialization.

Useful references:

- `references/cutlass/media/docs/cpp/cutlass_3x_design.md`
- `references/cutlass/media/docs/cpp/gemm_api_3x.md`
- `references/cutlass/media/docs/cpp/pipeline.md`
- `references/cutlass/media/docs/cpp/efficient_gemm.md`
- `references/cutlass/media/docs/cpp/cute/0t_mma_atom.md`
- `references/cutlass/examples/88_hopper_fmha/`

---

## 10. ThunderKittens audit

Audited ThunderKittens revision: `1c3920d99340`.

ThunderKittens answers the physical realization problem much better than the
carrier derivation problem.

### 10.1 Its useful vocabulary

ThunderKittens represents placement in distinct typed families:

- register tiles distributed over a warp;
- register vectors with deliberate distributions and replication;
- shared-memory tiles with hardware swizzles;
- global-memory views;
- Blackwell tensor-memory tiles;
- operation scopes such as thread, warp, warp group, block, and grid.

The transferable description of a physical value is:

```text
logical shape
dtype
memory space
layout
distribution and replication
collective ownership scope
```

Its operations also demonstrate that primitive legality includes source and
destination spaces, tile shapes, dtypes, layouts, collective scope, and an
async synchronization protocol.

### 10.2 Hopper FlashAttention

The Hopper kernels show direct coupling between shape and resources:

- query/output and key/value tile sizes are explicit constants;
- changing head dimension changes key/value block size and pipeline depth;
- producer and consumer roles receive different register budgets;
- the persistent `(m,l,o)` carrier lives in distributed registers;
- Q shared storage is later reused for output after Q dies.

For example, the optimized kernel changes from four pipeline stages to two
when `D` doubles. This is evidence that `β`, pipeline depth, and memory use
must be selected jointly.

### 10.3 Blackwell FlashAttention

The B300 kernel is even more revealing:

- a two-CTA cluster shards K and V differently;
- the logical output carrier resides partly in tensor memory;
- row maximum and row sum live in distributed registers;
- shared vectors hand rescale factors and statistics between specialized
  roles;
- softmax, MMA, and correction actors cooperate through semaphores;
- the output correction may be performed by a different actor from the one
  that selected the reference maximum.

Thus the realization of a carrier merge may be an asymmetric, in-place
microprotocol rather than one coordinate-wise expression:

```text
step(current_state, streamed_block)
    = primitive DAG over several actors and memory spaces
```

### 10.4 What not to copy

- Fixed 2D tile and vector categories.
- The positional `B,D,R,C` global view.
- NVIDIA-specific 32-thread warps, four-warp warp groups, WGMMA, TMA,
  `setmaxnreg`, TMEM, TCGEN05, and CTA clusters.
- Manual semaphore phase conventions.
- Hand-tuned resource constants as if they were derivation theory.
- Reinterpret casts and inline assembly as the normal compiler IR.

These belong in target capabilities and backend escape hatches.

Useful references:

- `references/thunderkittens/README.md`
- `references/thunderkittens/include/types/`
- `references/thunderkittens/include/ops/`
- `references/thunderkittens/kernels/attention/mha_h100/`
- `references/thunderkittens/kernels/attention/mha_h100_lcf/`
- `references/thunderkittens/kernels/attention/bf16_b300_mha_noncausal/`

---

## 11. Proposed architecture boundary

The combined structure suggested by the three reference systems is:

```text
CarrierLaw
    semantic state, lift, merge, project, decode
    associativity/commutativity/order/numerical certificate

NamedTiledFrame
    RangeId → hierarchical inner-tile/outer-rest factors

LogicalStateSchema
    product of CarrierFields
    each field = dtype + logical domain + layout

PrimitiveCatalog
    target operation/copy contracts and costs

DistributedRealization
    primitive cover
    agent/value layouts
    physical fragments and ownership

PipelineSchedule
    actors, async operations, dependencies, stages, synchronization

PlacementAndLiveness
    memory spaces, lifetimes, aliases, peak live resources, traffic

WorkSchedule
    grid assignment, persistent scheduling, split-range reduction
    constrained by the CarrierLaw's algebraic rights

ResourceCertificate
    registers by actor
    shared/threadgroup intervals
    target-local spaces
    barriers and pipeline stages
    occupancy estimate

Candidate
    one complete, legal, compilable and benchmarkable plan
```

The main boundaries are load-bearing:

- Carrier derivation remains target-independent.
- Logical state shape is derived from the carrier and block environment.
- Physical representation depends on primitive selection and participant
  distribution.
- Pipeline and synchronization are effects after the math is settled.
- Outer work scheduling may consume associativity, commutativity,
  determinism, and serializability rights from the carrier certificate.
- Occupancy is finally a property of the compiled realization, not just an
  algebraic estimate.

---

## 12. Candidate search

For each useful block environment `β`:

1. Derive the semantic `CarrierLaw` and logical state schema.
2. Factor named ranges into tile and rest coordinates.
3. Match carrier operations and tensor layouts against target primitives.
4. Enumerate compatible distributions, placements, and actor roles.
5. Enumerate a small set of pipeline depths and schedules.
6. Compute structured liveness and legal aliasing.
7. Reject any candidate exceeding a memory-space, register, synchronization,
   or launch constraint.
8. Rank survivors by traffic, occupancy, issue cost, synchronization, and
   estimated latency.
9. Retain a Pareto frontier rather than one greedy `Auto` decision.
10. Compile and benchmark the survivors; use empirical results to select and
    improve the cost model.

Analytical resource estimates are pruning tools. CUTLASS's runtime occupancy
queries and autotuning confirm that the actual compiled kernel remains the
final authority.

---

## 13. Design rules to carry forward

1. Do not encode every discovered carrier trick as a permanent kind.
2. Do not confuse a product-field decomposition with the semantic algebra
   that produced it.
3. Do not infer storage footprint from logical shape alone.
4. Keep named semantic ranges separate from scheduled loop identities.
5. Treat size-one and zero-stride modes as layout facts, not axis identity.
6. Keep target participant hierarchies out of semantic IR.
7. Store primitive capabilities as inspectable data and patterns, not forests
   of host-language specializations.
8. Carry symbolic legality guards instead of collapsing uncertainty to a
   boolean.
9. Model async operations through typed dependency/effect tokens.
10. Compute resource fit from scheduled liveness, including temporaries,
    pipeline replicas, and aliases.
11. A carrier projection may require redistribution and an epilogue; it need
    not execute in the accumulator's current layout.
12. A split reduction needs algebraic permission and a physical serialized
    partial-state representation.
13. "Fits" is legality, not profitability.
14. Keep an escape hatch for target-specific kernels without making those
    kernels the semantic vocabulary.
15. Keep the IR and diagnostics ordinary and inspectable; do not make the Rust
    type system perform the search.

---

## 14. Open questions

### Carrier theory

- How should indexed-monoid/key-fiber structures be discovered rather than
  manually recognized?
- What is the minimal algebra that covers products, dependent transports,
  and numerical gauges without becoming category-theory-shaped ceremony?
- Which equality does each certificate promise: literal state equality,
  decoded equality, real-number equivalence, bounded error, or fixed-tree
  reproducibility?
- Can a carrier representation change gauge dynamically while preserving a
  simple proof object?

### Layout and tiling

- What normalization of hierarchical layouts preserves named-coordinate
  semantics?
- How should symbolic divisibility and predication be represented?
- Should `Correspondence` and carrier `Layout` share a common coordinate-map
  core, or are their proof obligations sufficiently different to keep them
  separate?

### Hardware realization

- What exact Metal operations should become initial `PrimitiveAtom`s?
- How should Apple SIMD-group matrix operations, reductions, shuffles,
  threadgroup memory, cooperative tensors, async copies, and barriers be
  described through one spatial contract?
- How accurately can register use and occupancy be predicted before compiling
  Metal source?
- What target feedback should be cached and generalized across shapes?

### Search

- How wide must the candidate frontier be to avoid greedy stage/tile choices?
- Which decisions can be separated without losing strong candidates?
- Where should empirical benchmarking enter without making compilation
  prohibitively expensive?

---

## 15. Concrete next prototype

Extend `examples/indexed_carrier.rs`, or add a neighboring runnable example,
with the smallest end-to-end realization witness:

1. Named hierarchical ranges and a block environment.
2. Carrier fields with logical domains and zero-stride layouts.
3. A tiny target-independent `PrimitiveAtom` contract.
4. One mock Metal-like target with scalar/SIMD reduction and matrix fragments.
5. Distribution maps from `(agent, local value)` to logical coordinates.
6. Explicit memory placement and lifetimes.
7. A two-stage producer/consumer pipeline.
8. Peak-live resource calculation and a `ResourceCertificate`.
9. At least two legal candidates showing a real tradeoff.

The point of that prototype is not to generate a fast kernel yet. It is to
test whether the proposed layer boundaries can express FlashAttention's
logical carrier and a fragmented realization without reintroducing
`ExpShifted`, NVIDIA names, or fixed 2D slots into the generic theory.

Before promoting the design into production code, audit Apple's current Metal
primitive and resource contracts using the same questions applied to CUTLASS
and ThunderKittens.

---

## 16. Reference ledger

Repository snapshots used today:

| Reference | Revision | Role in the audit |
|---|---:|---|
| tinygrad | `f315df29a0c4` | movement/index lowering, symbolic ranges, scheduling comparison |
| NVlabs/CuTe | `1a4a2cd49ec5` | executable hierarchical layout algebra |
| NVIDIA/CUTLASS | `f94ec46f4f63` | atom/tiled-op/collective/kernel realization hierarchy |
| ThunderKittens | `1c3920d99340` | explicit physical tile vocabulary and high-performance attention protocols |

Local source documents:

- `core_ideas.md`
- `shapes_and_indexing.md`
- `vs_tinygrad.md`
- `number_systems_and_representations.md`
- `src/derive.rs`
- `src/plan.rs`
- `src/partition.rs`
- `src/emit_metal.rs`
- `src/metal.rs`
- `examples/indexed_carrier.rs`
- `references/metal/Metal-Feature-Set-Tables.pdf`
- `references/metal/Metal-Shading-Language-Specification.pdf`

---

## 17. The same range algebra can extend across GPUs and nodes

If ranges, their hierarchical factorization, and participant domains remain
generic, multi-GPU and multi-node execution should not require a new semantic
IR. It becomes another level of realization and scheduling:

```text
logical range
    → node factor
    → device factor
    → threadgroup factor
    → SIMD/lane factor
    → local sequential factor
```

Those target-specific names should not be built into `Range` itself. The
generic structure is:

```text
RangeId
    → hierarchical factors

factor
    → participant domain

logical coordinate
    → (participant, memory space, local offset)
```

This makes distributed sharding the same kind of operation as block and lane
tiling, while keeping factorization, participant assignment, and placement as
separate decisions.

### 17.1 Axis algebra determines communication

The semantic classification of the split range tells the scheduler what
distributed execution means:

- Splitting a **free axis** assigns independent outputs to different devices.
  No carrier communication is required.
- Splitting a **monoidal axis** makes each device produce a partial carrier.
  Those carriers must be exchanged and merged before projection.
- An associative but noncommutative carrier requires an order-preserving
  reduction tree.
- A commutative carrier permits arbitrary trees, rings, all-reduce,
  reduce-scatter, or other topology-driven organizations.
- An opaque or sequential axis cannot be distributed through carrier merging
  unless more structure is discovered.

FlashAttention gives both important cases:

```text
split q across devices
    → independent query/output tiles
    → no carrier merge between devices

split k across devices
    → one partial (m, l, o) per device
    → stable carrier merge across devices
    → project only after the distributed merge
```

The second case is exactly where constructive carrier derivation becomes more
than an on-chip fusion tool: it provides the partial-state representation and
algebraic permission needed by a distributed reduction.

### 17.2 Distributed layout

The layout concept can be generalized so its codomain is not merely a flat
offset:

```text
logical coordinate
    → (owner, owner's local coordinate)
    → (memory space, local offset)
```

A lane-distributed register fragment, a threadgroup tile, a GPU shard, and a
node shard then share one conceptual vocabulary at different levels of the
participant hierarchy. Replication, multicast, and sharding become properties
of this distributed coordinate map.

### 17.3 Communication primitives

Network and inter-device operations can use the same primitive-contract
boundary as local matrix, reduction, and copy operations:

```text
CommunicationAtom {
    operation: send | broadcast | reduce | all_reduce | reduce_scatter
    source and destination participant domains
    accepted carrier representation
    ordering guarantees
    topology and alignment restrictions
    async completion and synchronization effects
    temporary/workspace requirements
    bandwidth and latency model
}
```

This avoids baking NCCL, one vendor's link topology, or a particular collective
algorithm into the semantic carrier. They are target realization choices.

### 17.4 Additional distributed certificates

A distributed candidate needs more than the local resource certificate:

- whether the carrier may be reassociated and/or reordered;
- whether partial carrier state can be serialized;
- its serialized representation and byte size;
- ownership and replication of inputs and outputs;
- bytes sent over each topology link;
- communication/computation overlap and async dependencies;
- per-device memory pressure and peak live network buffers;
- load balance and tail effects;
- deterministic reduction guarantees;
- device heterogeneity constraints;
- eventually, failure and retry semantics for node-scale execution.

The important qualification is that good distributed performance remains a
difficult search problem. The architectural win is narrower and stronger:

> Multi-GPU and multi-node execution can become additional participant,
> placement, communication, and scheduling levels rather than a redesign of
> Sanic's mathematical compiler.
