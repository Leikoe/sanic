# Shapes and Indexing

*2026-07-29. The Γ treatment applied to movement: name the one object the
crate already computes six times, and make disagreement inexpressible.
Companion to `number_systems_and_representations.md` (values) and
`vs_tinygrad.md` (the two ways neighbours got this wrong). All receipts are
file:line on today's `main`.*

The one-line thesis, in the house form: **a movement node denotes a linear
correspondence between two index lattices; the node stores the one direction
in which that correspondence is affine, and every structural question — up,
down, across, backward — is a query against it.** Today each question has
its own hand-written answerer, and two of them disagree about the law.

## 1. The defect

### 1a. Six transports of one correspondence

"How does an index move across this node" is answered in six places, each a
partial projection of the same per-node map, none of them named:

| # | mechanism | direction | component tracked |
|---|---|---|---|
| 1 | `ir::axis_refs_rc` (ir.rs:378) | up | identity dims only |
| 2 | `derive::axis_aliases` (derive.rs:767, 181 lines) | down | identity dims, plus the broadcast-back pattern (`alias_collapsed`) |
| 3 | `partition::stream_provenance` + `stream_below_{view,reindex,gather}` (partition.rs:1835, 1927) | down | support — every driving dim |
| 4 | `partition::relocate_axis` (partition.rs:1645) | across a rebuild | positional zip of two graphs |
| 5 | `simplify::preserve_shape` (simplify.rs:259) | across a rewrite | occurrence match, then position, then **extent** |
| 6 | `grad::{lift_to_shape, broadcast_to, reduce_to}` (grad.rs:252, 507, 553) | frame alignment | trailing-broadcast convention |

Mechanisms 4 and 5 state contradictory laws. `emit_fold`'s comment on
relocation (partition.rs:1334-1338): *"positional relocation, not descriptor
matching: equal names/extents on unrelated dimensions remain unrelated."*
`preserve_shape`'s third fallback (simplify.rs:317-321) matches dimensions
**by extent** — the forbidden thing — and backs it with an `.expect`. Nothing
reconciles them because nothing owns the question.

Each answerer also re-implements the walk itself: the six carry five
near-identical match skeletons over the nine node kinds. A new node kind
today means six edits; a forgotten one means a silent wrong answer in
whichever pass forgot.

### 1b. Two nodes, one object, and the asymmetry it causes

`View` and `Reindex` are one mathematical object stored in its two
functional directions:

- `Reindex{map}` stores the **read** direction: each *source* coordinate is
  affine in *output* coordinates — `y_i = Σ c·x_j + b` (ir.rs:54-56).
- `View{dims}` stores the **write** direction: each *output* coordinate is a
  mixed-radix recomposition of its source group — `x_j = Σ y_{s_k}·stride_k`,
  strides the suffix products — affine from *source* to *output*
  (ir.rs:61-69).

Each stores the direction in which its map **is** affine. The other
direction exists pointwise (View is bijective by construction — every source
dimension consumed exactly once, ir.rs:708-711) but needs div/mod, which the
semantic graph rightly refuses.

The pipeline knows all of this and says none of it:

- `simplify::cancel_view_reindex` (simplify.rs:201) is Read∘Write
  composition, hand-restricted to the permutation case.
- `grad::invert_view` (grad.rs:293) is the transpose of a Write map —
  computed exactly, because bijectivity is structural.
- `grad::transpose_reindex` (grad.rs:349) is the transpose of a Read map —
  computed as a **dense one-hot contraction** over the product of both
  lattices, `O(|out|·|src|)` work, by its own comment *"intentionally dense
  but correct… later fusion can recover an efficient kernel."* Correctness
  by brute force, for maps whose inverses are one movement op away: a flip's
  transpose is a flip, a split's is a merge, a slice's is a zero-pad, a
  broadcast's is a sum. `scatter_add` (grad.rs:265) takes the same dense
  detour for `Gather`.

### 1c. Validity is a bool

`padded: bool` means "out-of-range reads produce 0" — and every consumer
rediscovers what that implies:

- interp clamps and substitutes 0.0 (interp.rs:402-420);
- codegen clamps the index and discards the load (codegen.rs:380-406);
- simplify **refuses padded maps wholesale** (cancel_view_reindex bails at
  simplify.rs:207) because it cannot see which region is valid;
- the emitter's honest window re-derives an index interval from mask
  arithmetic (emit_metal.rs:566);
- verify rechecks dynamic affine bounds after inputs bind (verify.rs:337).

Five sites doing interval reasoning on affine index forms — while
`numeric.rs` already owns exactly that algebra for values (`Bounds`,
`corners`, numeric.rs:83, 166).

### 1d. `Extent::Dynamic` is a marker that panics

`extent()` panics on Dynamic (ir.rs:35, 145); `contains_dynamic` refuses
compilation (compile.rs:459). There is no arithmetic: "cache length ≤ 1030"
is unsayable, a dynamic axis cannot be split, priced, or allocated. Decode
today sidesteps this with prefix masks — the honest window — which is the
*right* answer for a growing valid region, but bind-late extents (compile
once, bind the length at run) remain unexpressible.

## 2. What already holds

Load-bearing decisions this design keeps, stated so nobody re-litigates
them:

1. **Positional dimensions, descriptors as diagnostics.** `Axis{name,
   extent}` is plain data; equality of names means nothing across spaces
   (the convention: distinct spaces need distinct names). The frontend
   rewrite (`df77288`, `61228ad`) made this true end-to-end and killed
   `with_axis_order` — axis order is no longer smuggled through `0·Iota`
   arithmetic. `vs_tinygrad.md` design-debt 4 is **paid**.
2. **Occurrence identity is computed, never stored.** `AxisRef =
   (owner: *const Node, dim)` (ir.rs:127-139), minted by structural
   boundaries, transported by identity dims (ir.rs:447-462 for Reindex's
   identity rule). No IDs in the graph; interning makes structural equality
   pointer equality.
3. **The movement contracts are tight.** View: a partition of source
   dimensions — bijective by construction. Reindex: every source dimension
   mapped exactly once, affine. These invariants are asserted at
   construction (ir.rs:678-794), not hoped for.
4. **Masks are index arithmetic**, not mask tensors — the causal mask is
   `Iota`/`Lt`/`Where` fused into the flash lift, zero traffic.
5. **The semantic graph is pure**, and layout is not in it. Storage order
   exists only at emission (`codegen::offset`) and in Γ-side program facts.
   The implicit law, stated so it stays deliberate: *a materialized
   buffer's layout is its producer's dimension order at the cut, and the
   partitioner may insert a `View` before a cut to choose it.* That degree
   of freedom is real and measured — defect A (V stored
   sequence-innermost, #17) bought milliseconds with it — and it is one
   tinygrad cannot state at all (buffers there are row-major in
   stage-range order, always; `vs_tinygrad.md`, indexing/layout path §4).
   No new machinery; just the sentence.

6. **Movement never forces a buffer.** Torch's `.contiguous()` cliff —
   copy whenever a composite stops being one strided view of one buffer
   (`t().reshape(-1)`, `expand(...).reshape(...)`) — does not exist here:
   movement is graph structure, buffers appear only where the partitioner
   cuts, and emission composes the chain into index arithmetic inside the
   consuming kernel (llama already fuses RoPE reindexes and KV views
   through the flash fold this way). Materialization is a *pricing*
   decision (`inline_pays`, the shared-gather barrier, a layout choice at
   a cut), never a representational failure — and the closed affine
   vocabulary keeps this, because a composite one node cannot hold simply
   stays two nodes: the failure mode is untidiness, not traffic.

The neighbours' two failures frame the target (`vs_tinygrad.md`, *shapes*
section): strides-and-masks bookkeeping in the semantic graph (tinygrad's
ShapeTracker, deleted after years) and movement dissolved into div/mod that
a 400-line simplifier must re-factorize (their replacement). Keep structure
stated; keep arithmetic at the boundary.

## 3. The calculus

The mechanism half-exists. `ir::Resolver` already centralizes `axes`,
`map_input_axis`, `source_axis`, `view_groups`, and `resolved_reindex` —
the six mechanisms of §1a already lean on it for their per-node knowledge;
what they duplicate is the walk skeletons and the per-kind dispatch around
it. The design is therefore to **finish Resolver into the one answerer**,
not to stand a second mechanism beside it. `Correspondence` names the
concept; the implementation is Resolver growing the queries it is missing.

**The contract compiles: `tests/shapes_contract.rs`** — the gamma
precedent (its header: "began as a model in tests/contract.rs"), model
types and the laws as passing tests. Law 1 is privacy (`mint` is the one
constructor), Law 3 is the field list (`AffineDim` has no div, no mod, no
data term — a gather is not a value of the type), Law 5 is
`cap()` total / `at(bindings)` partial (the panic is absent from the
vocabulary), and §5's classification is a theorem there: the symbolic
verdict from coefficients alone equals the enumerated preimage structure
of the actual relation, over the whole zoo — permute, flip, pad, slice,
strided slice, split, merge, broadcast, overlapping window. Transpose as
data reinterpretation is a passing test (`merge.transposed() == split`),
not a claim. Promotion moves the laws into `ir::Resolver`/`grad` and
deletes the file, exactly as Γ's model was promoted:

```text
Correspondence = { direction: Read | Write,
                   dims: per stored-side dimension,
                          terms: Vec<(coefficient, other-side dim)>,
                          offset,
                   validity: per source dim, an interval fact   (§ M-F) }
```

- `correspondence(node)` — the accessor. `Reindex` yields Read (its `map`,
  verbatim). `View` yields Write (groups with suffix-product strides).
  `Gather` yields Read with one **data term** — a coordinate read from a
  tensor. The data term is what removes Gather from the linear algebra;
  that is its definition, not a defect. Every other node kind yields the
  identity correspondence on its pass-through dims (Map's right-aligned
  broadcast, Reduce's deletion, Scan's preservation — the rules
  `axis_refs_rc` already implements).

Three derived queries answer everything §1a's six mechanisms compute:

- `identity(dim)` — the coefficient-1, offset-0, sole-reader component.
  What mechanisms 1 and 2 track.
- `support(dim)` — every other-side dim carrying any term. What mechanism
  3 tracks.
- `transpose()` — swap the direction tag. **Free.** A Write map's
  transpose is Read-shaped data read backward, and vice versa; no div/mod
  is introduced because nothing is recomputed — the factorization is
  stated in the node and stays stated.

## 4. The laws

**Law 1 — one answerer.** Every structural question about a node — up,
down, across, backward — is a query against `correspondence(node)`. A pass
may fold the queries over its own walk; it may not re-derive the per-node
map. This is Γ's move (`gamma.rs:107-117`) applied to indexing: when
emission, pricing, gradient, and partition all ask the same object,
their disagreement is inexpressible. `preserve_shape`'s extent-matching
arm stops being a bug to catch and becomes a sentence that cannot be
written.

**Law 2 — closure.** The vocabulary is closed under transpose (total) and
under composition where directions align (Read∘Read and Write∘Write, by
substitution). Mixed-direction composition is partial — defined exactly
when the composite is affine in one direction — and `cancel_view_reindex`
is its one caller today; it stays exactly that size (§7). Consequence: an
adjoint stays inside the vocabulary whenever the map is injective (§5).
This is the half of tinygrad worth stealing (six one-line movement
gradients, `mixin/gradient.py:69-76`) without the substrate.

The trade, stated against their substitution engine. For permute, flip,
shrink, expand, tinygrad's `apply_movement_op` and this vocabulary are the
same object — a tuple swap is a tuple swap, `(s-1)-r` is
`AffineDim{[(-1,0)], e-1}`. The divergence is two-fold. First, their
engine speaks only the read direction (ranges flow consumer→producer;
PERMUTE needs `argsort` for exactly this reason), so RESHAPE always mints
div/mod: a split's cancels back to affine after simplifier work, a
merge's is permanent. Storing the affine *direction* per node (`View` =
write, `Reindex` = read) is one bit that replaces that simplifier.
Second, their open expression language makes composition total — a
movement chain collapses to one expression, free — but every query
becomes a canonicalization question, which is how their kernel boundaries
ended up downstream of simplifier completeness. The closed `AffineDim`
takes the opposite side: queries are field reads, composition is partial,
and a composite that leaves the affine language just stays two nodes.
The bet is that sanic's movement chains are shallow (simplify cancels,
interning dedups); if a workload ever shows deep chains, a composed-map
cache on Resolver is the pressure valve — still inside the closed
language. *Measured 2026-07-30, llama decode graph at ctx 1030: 2,070
nodes, 760 movement (36.7%), maximal chain depth **4** (375/225/96/64
nodes at depths 1→4). The bet holds; no cache.*

**Law 3 — structure stays stated.** Factorizations live in nodes: a split's
radices, a view's groups. Div/mod exists in exactly one place — emitted
index expressions (`codegen::value`/`offset`) — as *representation*,
downstream of all reasoning. The test this law applies to any
representation choice: **how much downstream machinery must re-derive what
construction once knew?** tinygrad fails it three times (reshape merging,
gather recovery, validity pruning — `vs_tinygrad.md`, indexing/layout
path §3); sanic's remaining sins are the same species, smaller —
`preserve_shape` guesses a frame the rewrite knew, `relocate_axis`
re-derives a correspondence the rebuild knew. Minimize reconstruction, not
vocabulary.

**Law 3b — the bottom layer is data, not text.** Law 3's "one place" for
div/mod is emitted index expressions — and today that place is `String`
(`codegen::offset`, the `Lang` trait): a low-level index IR made of text,
invisible to `numeric::Bounds`, untestable except through whole kernels.
tinygrad is right about this layer: indices should end as typed
expression data. The corrected shape is three levels, each earning its
place — semantic `Correspondence` fields (decisions read these; field
reads, no simplifier), a small typed `IndexExpr` at lowering (composed
maps; `Div`/`Mod` legal here, after every decision; interval-checkable
against buffer bounds by the same algebra `numeric.rs` owns), and `Lang`
as dumb rendering. What stays refused is hoisting expressions into the
semantic level, where decisions would inherit the simplifier's
completeness. *Driver: no measured index-emission defect yet — build when
one fires, or when address CSE stops being the Metal compiler's job.*

**Law 4 — validity is an interval fact.** `padded: bool` generalizes to
per-source-dim membership `0 ≤ f_i(x) < n_i`, evaluated by the same
interval algebra `numeric.rs` applies to values. One evaluator; interp,
codegen, simplify, verify, and the honest window consume verdicts instead
of re-deriving them. A padded map whose validity proves total cancels like
any other — simplify's wholesale refusal (§1c) dissolves.

**Law 5 — one integer form for extents and coefficients.** v1 is
deliberately small: `Extent::{Static(n), Sym{id, cap}}` — a symbol with a
static cap. Enough to compile once and bind the KV length at run, price and
allocate at the cap, and keep every existing law checkable. No products, no
quotients, no general inequalities until a kernel forces one (§7).

## 5. Adjoints from the correspondence

The gradient of a movement is the transpose of its correspondence, and the
transpose's *shape* dictates the code — one classification replacing five
hand helpers:

| map class | adjoint | today |
|---|---|---|
| Write (View) — bijective by construction | transpose, exact | `invert_view`, already exact — becomes `transpose()` |
| Read, per-dim unimodular (permute, flip, split-image) | movement by the inverse map | dense one-hot |
| Read, injective not surjective (slice, stride) | inverse movement + zero-fill (a padded map — Law 4's object) | dense one-hot |
| Read, non-injective dims (broadcast: empty terms; overlapping windows: shared support) | reduce-add over the fiber, then movement | `reduce_to` / dense one-hot |
| Gather (data term) | scatter-add along the data | dense one-hot composite |

The dense contraction **remains** — as the total fallback for the genuinely
non-injective cases (overlapping windows are conv's correlation adjoint;
they are not going to become a bijection) and as the differential-test
oracle for every classified case. The classification narrows the dense
corner to where it is the mathematics, instead of the default.

## 6. What it deletes

Priced against `vs_tinygrad.md`'s accounting (autodiff 4.4× the density
baseline; ~390 lines of standing shape tax):

| site | today | after |
|---|---|---|
| `stream_below_{view,reindex,gather}` (partition.rs:1927-1962) | 35 | queries on `support` |
| `axis_aliases` walk arms (derive.rs:767) | ~100 of 181 | a fold over `identity` |
| `stream_provenance` walk arms (partition.rs:1835) | ~50 of 80 | the same fold, `support` mode |
| `preserve_shape` heuristics (simplify.rs:259) | ~40 | rewrites return their frame (M-D) |
| `grad` helpers: `invert_view`, `transpose_reindex`, `scatter_add`, `broadcast_to`, `reduce_to` | ~205 | one classified adjoint, ~80 |
| duplicated walk skeletons | 5 copies | callers fold; the map lives once |

Net estimate: **250–350 lines out of src**, the dense-adjoint work blowup
gone for every injective map, and — the actual point — one law where there
were six answerers, two of them contradictory.

Estimate honesty: the walk *skeletons* are the duplication; the per-pass
semantics inside each walk (derive's broadcast-back aliasing, partition's
done-stopping) are not, and they stay where they are. If an arm resists
the fold, that is evidence about the design, not a reason to force it —
the abstraction is on trial here, not the arm.

## 7. Not doing

- **Not merging `View` and `Reindex` into one node.** The stored direction
  is a real fact about each node; merging trades two honest constructors
  for a direction flag every consumer must branch on. Merge the questions
  (`correspondence`), not the nodes.
- **No general composition algebra.** Composition has exactly one caller
  (`cancel_view_reindex`). An algebra amortized over one use site is the
  same mistake as importing `PatternMatcher` for five traversals — the
  argument this repo already used against tinygrad's engine applies to its
  own designs first.
- **No layout in the semantic graph.** Strides, storage order, and widths
  remain program facts (Γ-side, emission-side). ShapeTracker is the
  cautionary tale.
- **No general symbolic integers.** No div/mod terms, no inequality
  language, no Presburger. `Sym{id, cap}` and nothing else until a kernel
  a user needs is blocked on more.
- **No polyhedral dependence analysis.** The carrier algebra is the
  legality story; the correspondence is bookkeeping for it, not a second
  prover.
- **No named-axis identity.** Names stay diagnostics; occurrences stay the
  identity. The capture problem stays solved.
- **Gather stays Gather.** The data term is the definition of
  data-dependent access, not a hole to patch.

## 8. Staging

Each step lands green (`cargo test --all-targets`, 227) and alone. Each
carries its *driver* — the thing that bleeds without it — because a step
without one is speculation, however elegant. Nothing here bleeds today;
M-C bleeds first.

- **M-A — finish Resolver.** *[LANDED 2026-07-30 — the `support` half:
  `Resolver::support_below` replaces `stream_below_{view,reindex,gather}`,
  and their three duplicated dispatch arms in `leaf_cuts`, `hot_volume`,
  and `stream_provenance` collapse to one query. Gate held: llama MSL
  byte-identical, all correctness targets green. `identity` already lives
  as `axis_refs_rc`'s pass-through rule; `transpose` waits for its first
  consumer (M-C) — a query with no caller is dead surface, not progress.]*
- **M-B — the folds.** *[TRANSITIONAL 2026-07-30 — `axis_aliases` now
  runs twice: the hand-written arms (authoritative) beside
  `axis_aliases_via_transport`, a fold over `Resolver::frame_below` (the
  down identity transport, `FrameSlot::{Parent, Consumed, Broken}`), with
  `debug_assert_eq!` on every derivation the suite compiles — the
  assertion held across the whole suite on the first run. The swap
  (delete the arms, keep the fold, drop the double compute) is its own
  reviewed commit. What stayed in derive, correctly: the consumed-dim
  alias choice and `alias_collapsed`'s broadcast-back — semantics, not
  transport. `stream_provenance` already thinned to `support_below`
  queries at M-A; folding its walk skeleton further is churn, not
  simplification — left alone deliberately.]*
- **M-C — the adjoint.** Classified transpose in `grad`; dense corner only
  for non-injective and data cases. Gate: `tests/grad.rs` + randomized
  movement chains, classified vs dense vs interp, bit-agreement in f64;
  grad graph node counts recorded before/after. *Driver: the gradient of a
  `split` — a pure relabeling — is a dense equality-mask contraction
  TODAY, reachable from the public surface (`grad.rs` Reindex arm is
  unconditional); the training arc (synthesis P4) makes it hot.*
- **M-D — shape-changing rewrites state their frames.** Scoped to the few
  `simplify` rules that change rank or order — they return their frame
  instead of `preserve_shape` guessing it. NOT a crate-wide interface
  change; if that is what it turns into, stop and reconsider. Gate:
  simplify + laws suites.
- **M-E — `Sym{id, cap}`** (independent of A-D). Verify's recheck-at-bind
  extends; `compile_for` accepts a symbolic length; allocation at cap
  (precedent: tinygrad allocates at `vmax` and shrinks). Gate: decode
  context sweep matches static-extent compiles token-for-token. *Driver:
  none measured yet — honest-window masks carry decode. Build when
  bind-late compilation is demanded by a real caller, not before.*
- **M-F — validity intervals** (after M-E; v2). `padded: bool` becomes the
  interval fact; simplify cancels padded maps it can prove total; the
  honest window reads the same fact it currently re-derives. *Driver: no
  kernel is known lost to the padded refusal — measure that before
  building this.*

## 9. Open questions

1. **Scan's kept axis.** Scan preserves its dimension's occurrence
   (ir.rs:424) though the output is a different function of that axis than
   the input. Harmless for today's carriers (prefix state is the point);
   revisit if a pass ever needs "same loop variable" to imply "same
   values".
2. **Overlapping-window adjoints.** Conv's correlation adjoint is
   non-injective; the dense form or a future scatter node are the options.
   Decide when training work (synthesis P4) makes it measurable.
3. **`Coordinate` under the calculus.** `Coordinate{src, dim}` is
   `Iota∘broadcast` in correspondence terms; whether to express it that way
   or keep the node is a churn question, not a design one.
4. **Frames at multi-root boundaries.** `graph.rs` value-reads across
   roots use the same alias discipline; M-B should confirm one frame type
   serves both, not grow a second.
5. **Per-dimension materialization.** tinygrad realizes a DAG-shared node
   only along the axes where its consumers' index expressions differ
   (`realize_map[x] = axis list`; the partial stage lands in LOCAL memory —
   `vs_tinygrad.md`, *indexing/layout path* §2). Sanic's partitioner cuts
   shared nodes whole. In correspondence terms the question is well-posed:
   two consumers whose maps agree on a dim's `identity` component need no
   materialization along it. Worth pricing once the Correspondence type
   exists (after M-A); it is the one idea from their path that is not
   already here in some form.
