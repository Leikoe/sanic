# A Theory of Kernel Fusion

Five claims, then their development. Everything below exists to state and prove these.

1. **Legality is convexity.** A fusion plan is valid iff its blocks are convex in the dataflow DAG — a structural fact, independent of hardware and cost.
2. **Value is traffic.** Total work is plan-invariant; fusion's entire economic content is the intermediate traffic it de-materializes, bounded by the ratio of intermediates to boundary data, and worth nothing once compute-bound.
3. **Reduction boundaries fall by algebra — two laws, two freedoms.** Associativity governs how a fixed reduction executes; the Myhill–Nerode condition on the suffix congruence stratifies it: *collapsible* (closed form → no pass at all) ⊊ *foldable* (bounded sketch → single-pass streaming) ⊊ *monoidal* (associative merge → tiling and tree parallelism). Distributivity governs which reduction is computed — factorization changes work itself, and its surviving scalar fragment under a blocking nonlinearity is normalizer deferral, the algebraic content of FlashAttention. Impossibility is proved by fooling sets on the same congruence.
4. **Hardness comes only from capacity and sharing.** Without them, more fusion is always free; with them, optimal fusion is NP-hard, while chains stay polynomial.
5. **Fusion is restricted pebbling.** Partition-structured schedules are a strict subclass of all I/O-optimal executions; the residual gap (the *price of fusion*) is the territory of megakernels.

---

## 1. Model

**Machine** M = (π, β, S, λ): flop rate, slow↔fast memory bandwidth, fast-memory capacity in words, per-kernel launch overhead. **Machine balance** ρ = π/β.

**Program**: a DAG G = (V, E). Vertex v is an operator with work w(v); edge e carries a tensor of size |T_e| words. Distinguished input and output tensors. For a region B: W(B) = Σ w(v), and its **arithmetic intensity** I(B) = W(B)/Q(B) with Q(B) defined in §3.

## 2. Legality

**Definition (plan).** A fusion plan is a partition P = {B₁,…,B_k} of V; each block compiles to one kernel; tensors on intra-block edges are never materialized in slow memory.

**Definition (convex).** B is convex if every path between two vertices of B lies in B.

**Theorem 2.1.** P admits a valid execution order iff the quotient G/P is acyclic iff every block is convex.

*Proof.* A non-convex block yields a path leaving and re-entering it, hence a quotient cycle: mutual data dependence between kernels. Conversely an acyclic quotient is topologically orderable, and each block internally so. ∎

Legality is exact and cost-free to decide; everything downstream searches only among correct plans.

## 3. Cost, and the four laws

Let In(B), Out(B) be the distinct tensors crossing into and out of B. The **boundary traffic** is Q₀(B) = Σ|In| + Σ|Out|; the **true traffic** Q(B) is the minimum I/O of B's sub-DAG in the red–blue pebble game with S fast pebbles. Always Q ≥ Q₀, with equality when B streams within capacity. Under a roofline with perfect overlap:

> Time(B) = max( W(B)/π, Q(B)/β ),  Cost(P) = Σ_B Time(B) + |P|·λ.

**Law 3.1 (Invariance).** W is identical for all plans. Fusion trades exactly three currencies: traffic, launches, and (under covers, §4) duplicated work.

**Law 3.2 (Pointwise chains).** On a chain of pointwise operators over N elements, the fully fused plan is optimal.
*Proof.* Fused, the chain streams with O(1) per-element state, so Q = 2N — the input and output volumes, a lower bound for every plan. Work is invariant and |P| = 1 minimizes launches: every cost term is simultaneously minimal. ∎
Hence greedy maximal pointwise fusion is *exact*, not heuristic.

**Law 3.3 (Saturation).** If two adjacent blocks each have I ≥ ρ, merging them improves Cost by at most λ. Traffic hidden under compute is already free.

**Law 3.4 (Threshold).** Merging memory-bound B₁ → B₂ with intermediate volume t is profitable iff
> 2t/β + λ > ΔQ_spill/β + ΔW_recompute/π.
When the merge is capacity-safe and duplication-free the right side is zero: fusion is then unconditionally profitable. All difficulty in fusion originates in capacity and sharing.

**Law 3.5 (Leverage).** For a region with boundary volume A and intermediate volume t, fusion's speedup on the traffic term is at most (A + 2t)/A. Fusion pays where intermediates are fat and boundaries thin — attention scores, normalizations — and cannot pay where the boundary dominates.

## 4. Recomputation: covers

**Definition.** A *cover plan* relaxes the partition to a family of convex sets covering V with acyclic quotient; a vertex in m blocks is recomputed m times, and a tensor is materialized only when some consumer's block excludes its producer.

**Proposition 4.1 (Rematerialization inequality).** Replicating producer u (output volume |T|) into m blocks instead of materializing beats materialization iff
> (m−1)·w(u)/π + (m−1)·(u's own input traffic)/β < (m+1)·|T|/β.
Recomputation wins precisely for producers that are cheap relative to their output — pointwise ops nearly always, contractions nearly never. Gradient checkpointing is the special case where the blocks are the forward and backward passes.

Fusion theory over covers is the joint fusion + rematerialization problem.

## 5. Streamability

The frontier of fusion is the reduction boundary: when can a consumer of a reduced quantity be fused into the producing pass? Fix an axis and view the computation as h : List(X) → Y on the sequence of elements along it.

### 5.1 The congruence

**Definition (suffix congruence).** p ≡_h q iff h(p·s) = h(q·s) for every suffix s.

**Definition (foldable).** h is *k-foldable* if there is a state space Σ of k words, ι ∈ Σ, step : Σ × X → Σ, out : Σ → Y with h = out ∘ foldl(step, ι). Equivalently: a fold-computable sketch σ with k-word state refines ≡_h and determines h.

**Definition (monoidal).** h is *k-monoidal* if additionally the block map H : List(X) → Σ satisfies H(xs ++ ys) = H(xs) ⊗ H(ys) for an associative ⊗ on Σ (with identity H([])).

**Theorem 5.1 (Two tiers).** k-monoidal ⇒ k-foldable, and the inclusion is strict: there exist O(1)-foldable functions whose every associative-merge carrier requires state growing with the block length.
*Proof sketch.* Monoidal gives step(s, x) = s ⊗ into(x). For strictness: a left fold's block acts on Σ as a transfer function Σ → Σ; merging blocks requires representing that function, and folds exist whose transfer functions have description size Ω(n) (e.g. steps whose induced maps are piecewise-linear with unboundedly many breakpoints), while the sequential state stays O(1). ∎

The two tiers are two execution rights: **foldable** licenses single-pass streaming (the long axis never materializes); **monoidal** additionally licenses tiling, tree reduction, and split-reduction (blocks combine in any order). A carrier can hold the first right without the second — one-pass k-best insertion versus the two-list merge is the practical instance.

### 5.2 Composition and deferral

Foldability composes structurally: maps ride inside `into` (before a reduction) or `out` (after); independent reductions over the same axis take the product carrier; coupled reductions take a *strengthened* product whose merge rescales (max coupled with Σexp(·−max) gives the online-softmax carrier (m, s), associative because exponential rescalings telescope).

The load-bearing rule is:

**Theorem 5.2 (Normalizer deferral).** Let r(xs) produce per-element weights f(xs_i)/N(xs) with a global normalizer N, and let the consumer L be linear (a semimodule homomorphism): L(c·w, v) = c·L(w, v). Then L ∘ r is foldable with carrier = (carrier of N, unnormalized accumulator of L), and the division by N moves into `out`, applied once.
*Proof.* Linearity factors the scalar out of the reduction: Σ (f_i/N)·v_i = (Σ f_i·v_i)/N; the numerator is an ordinary coupled fold. ∎

*Instance.* softmax alone is not foldable to its (size-n) output; softmax followed by the linear V-contraction is, with carrier (m, ℓ, o) and out = o/ℓ. This single application of Theorem 5.2 to the coupled (m, s) carrier *is* the legality of FlashAttention; its profitability is Law 3.5 (the n×n score matrix is the fat intermediate, the boundary Θ(nd)).

### 5.3 Impossibility

**Theorem 5.3 (Fooling sets).** Any streaming realization of h must use state distinguishing every pair of ≡_h-inequivalent prefixes. Hence a family of pairwise-inequivalent prefixes of size N(n) forces state ≥ log₂ N(n) bits.

**Corollary (median).** Two prefixes with different value multisets can be separated by a suffix placing the differing element at the middle rank; thus ≡_median is multiset equality, the state must determine the entire multiset, and no o(n)-state fold computes a running median.

The same argument disposes of thresholds against final aggregates (e.g. counting elements above half the eventual max): each element's contribution depends on the final normalizer in a way no bounded correction survives. "Composed of monoidal pieces" does not imply "monoidal composite."

### 5.4 Verification duality

The two directions of §5.1 are independently checkable, which turns a fusion system's *declines* into claims rather than silent conservatism:

- **Positive certificates** (a carrier) are validated by evaluation: property-test associativity, identity, and out ∘ fold ≡ reference on random data and random splits.
- **Negative claims** (a decline) are *detected* by collision probing — search for a sketch under which colliding prefixes have identical futures on sampled suffixes; a surviving sketch names candidate carrier slots constructively — and *proved* by exhibiting a growing fooling family (Theorem 5.3).

A sound-by-evaluation, complete-by-probing pair of oracles makes the fusability frontier itself a regression-tested object.

**Practice (probe instantiation).** The sketch space must be named for the probe to be implementable. A workable stratification, by state budget k = 1, 2, …: sketches are tuples of fold-computable slots drawn from { Σφ(x), max φ(x), min φ(x), count[φ(x) > 0] } with φ ranging over a small feature dictionary — identity, coordinates, exp, log|·|, low powers, and pairwise products of already-admitted slots — searched smallest-k first so a surviving sketch is a *minimal* candidate carrier. Collision pairs are generated adversarially, not only randomly: seed prefixes agreeing on the sketch but differing maximally elsewhere (mismatched length, permuted order, outlier magnitudes), since random collisions concentrate on easy agreement. Futures are compared at a tolerance tied to the semantics quotient of §5.5 — exact for integer/semiring carriers, relative-error bands under IEEE — and every probe verdict (survivor, kill pair, budget exhaustion) is emitted as a replayable artifact. Budget exhaustion with no survivor is the trigger to switch from probing to fooling-set construction (Theorem 5.3).

### 5.5 Tier zero, and rewrite closure

**Definition (collapsible).** h is *collapsible* along an axis if it equals a closed-form function of the axis bounds and O(1) axis-independent quantities — zero state, zero passes; the loop is deleted, not fused. Sums of expressions piecewise-polynomial in the index (indicator ranges, iota polynomials via Faulhaber) are collapsible, as is the one-hot gather Σ_r [i = r]·e(r) → e(i). Collapsible ⊊ foldable, completing the hierarchy of Theorem 5.1 downward.

Rewriting also relativizes the whole predicate. Semantics-preserving identities (x·0 → 0, gate folding, one-hot collapse) transform G itself — deleting work, deleting edges, and moving nodes between structure classes — so Law 3.1's invariance holds only per rewrite-equivalence class. Define three nested predicates:

> **F_syn** (the rules fire on G as written) ⊆ **F_rw** (some semantically equal G′ is in F_syn) ⊆ **F_sem** (the Nerode condition holds).

The completeness probe of §5.4 measures F_sem ∖ F_syn; each detected miss admits exactly two remediations — extend the carrier vocabulary, or add the rewrite that maps the pattern into it — and the irreducible residue F_sem ∖ F_rw is the specification-reframing frontier of Problem 2.

Two provisos that any implementation of the rewrite layer must respect. *Soundness is relative to a semantics*: x·0 = 0 fails under NaN and poison values, and IEEE addition is not associative, so every declared law — including the MONOIDAL tag itself — is a law of a chosen quotient semantics, and rule ordering (poison propagation before ring identities) is part of soundness. *Normalization is optimization*: the index algebra with floordiv/mod, closed under the recombination x%c + (x//c)·c = x, is what makes the tiling reindexing r ↦ (r//s, r%s) invertible, and its rewrites must be cost-guarded (accept a merge only if divmod complexity does not grow) — the rewriter is a search, not a confluent normal form.

### 5.6 The second law: distributivity

Over a semiring (⊕, ⊗), two laws grant two different freedoms:

- **⊕-associativity grants execution freedom** — how a *fixed* reduction runs (stream, tile, tree, split). It is the entire content of §5.1 and never changes W.
- **⊗-over-⊕ distributivity grants algorithmic freedom** — *which* reduction is computed. It reorganizes the term graph and changes W itself, so it lives in the rewrite layer F_rw, beyond the reach of any fusion plan.

Distributivity acts at two scales. **Hoisting** (its scalar fragment, one factor through one fold): a reduction-invariant factor moves out of `into` and into `out` — Σ(c ⊗ xᵢ) = c ⊗ Σxᵢ. Theorem 5.2 is exactly this with c the deferred normalizer, and the tier-0 collapse rules of §5.5 decompose as hoisting followed by closed-form counting. **Factorization** (nested sums): Σᵢⱼ aᵢ ⊗ bⱼ = (Σaᵢ) ⊗ (Σbⱼ) turns n² work into 2n — the generalized distributive law, whose instances include FFT, Viterbi in the (max,+) semiring, and einsum reassociation. Choosing the cost-optimal reassociation is NP-hard: the W-level twin of Theorem 8.1's Q-level hardness.

**Attention, resolved by this dichotomy.** Full distributivity is linear attention: (QKᵀ)V ⇝ Q(KᵀV), reducing O(n²d) work to O(nd²). Softmax's nonlinearity blocks the reassociation; the fragments of distributivity that survive it are exactly two scalars — the normalizer 1/ℓ through the linear V-sum (Theorem 5.2) and the shift e^m through Σexp (factorization via the exp homomorphism from (+) to (×)) — and these two fragments are precisely the (m, ℓ, o) carrier. FlashAttention is the maximal fragment of distributivity surviving softmax; fusion, licensed by associativity, is what executes when factorization is blocked.

### 5.7 Streamability without the carrier

The Nerode quotient of §5.1 is itself the canonical minimal carrier — classes are states, prefix extension induces the step, and every realization factors through it — so carrier *existence* is a property of a semantic invariant, decidable in principle without proposing any shape. Define the **future** of a prefix as F_p : s ↦ h(p·s).

**Theorem 5.4 (Dimension criterion).** h is k-streamable iff the family {F_p} lies in a k-parameter family; the dimension of the futures space is the minimal carrier size. In linear settings this is realization theory: a k-state realization exists iff the Hankel matrix H[p,s] = h(p·s) has rank k. (Sum: dimension 1. Attention output: dimension 3 — the (m, ℓ, o) carrier appears as a measured rank, not a design. Median: infinite.)

Three consequences. *Refutation is finitely witnessed*: k+1 prefixes with independent futures — a rank witness, the linear-algebraic form of a fooling set — certify state > k with no carrier language at all; certifying streamability instead needs a global dimension bound, provable symbolically as an order-k annihilating recurrence of the futures under prefix extension (a holonomicity condition, detectable by telescoping-style algorithms without solving it — the same technology that decides tier-0 collapse). *The monoidal tier is also carrier-free*: by the third homomorphism theorem, h left-streamable and right-streamable implies an associative list homomorphism exists — run the dimension test in both directions and the merge's existence is certified without constructing ⊗. *Synthesis becomes stratified*: a measured dimension k tells a carrier synthesizer exactly which stratum to search, replacing exponential shape search with rank estimation followed by extraction. Dimension first, shape second — and the shape only when the kernel is actually wanted.

**Practice (running the rank test).** Finite sampling makes the measured rank a certified *lower* bound and only presumptive upper bound: grow prefix length and sample count until the singular spectrum plateaus — a plateau is evidence of streamability at that dimension, monotone growth is a refutation trend to be converted into an explicit fooling family. Center the matrix (or discard one unit of rank) before reading the state size: additive carriers produce H[p,s] = state(p) + g(s), an affine family whose matrix rank exceeds the parameter dimension by one. Set the threshold on the *relative* spectrum σᵢ/σ₁ and require a visible gap, not a fixed epsilon; under IEEE semantics a soft tail is expected and its scale should match the property-test tolerance, so that "rank" and "passes fold-vs-reference" are calibrated against the same quotient. Run the test in both directions in the same harness — the right-to-left rank is the merge-existence certificate (third homomorphism theorem) and costs one transpose. A full-rank verdict is ambiguous between a true wall and a missing rewrite (F_sem ∖ F_syn); it must therefore route back into the rewrite search of §5.5 with the verdict attached, never terminate the analysis by itself. The loop is *rewrite-candidate → rank → extract*, not *rank → extract*.

## 6. Structure over a graph

Streamability is per-(operator, axis), not per-operator: an axis contracted in one node is free in another. Classify each pair into
> FREE (no dependence) < MONOIDAL (associative fold; LINEAR flag when Thm 5.2 applies) < OPAQUE (data-dependent access) < SEQUENTIAL (non-associative recurrence),
propagated by structural recursion (maps pass structure through; a reduction is MONOIDAL iff its operator is; gathers poison to OPAQUE; non-associative scans to SEQUENTIAL). The per-axis map separates the two distinct fusion mechanisms — same-axis carrier merge (attention) and cross-axis producer-consumer tiling (fused MLP) — which block-level analysis conflates, and isolates the residue (routing, sampling, decode loops) that no fold covers.

**The decision loop (sliding the cut).** Streamability of an operator in isolation is the wrong query — softmax alone is unstreamable while softmax∘linear is — so the unit of analysis is the *cut*: a convex subgraph from the axis's elements to a candidate materialization frontier. For each reduction axis, enumerate cuts outward from the reducing node toward each downstream materialization point, and for each cut form the composite h and test it: (1) syntactic tier match (§5.5 collapse rules, then the fold vocabulary); (2) on failure, fire annotation-triggered rewrites and retest; (3) on continued full rank within budget, run the collision probe (§5.4); (4) record the verdict — carrier, fooling family, or surviving sketch — as an artifact attached to the cut. Widening the cut past a linear consumer can only shrink the Nerode dimension, so cuts are explored in increasing width with early exit at the first small-rank frontier. The trigger table, mechanical because every trigger is an annotation already present in the graph: a *linear consumer on the reduced axis* (any matmul or semimodule map) fires normalizer deferral (Theorem 5.2); a *shift- or scale-invariant normalizer* fires the exp/log homomorphism transport (§5.6); a *reduction-invariant factor* fires hoisting; a *piecewise-polynomial index dependence* fires tier-0 collapse (§5.5); a *one-hot or gate pattern* fires the corresponding deletion rewrite. Attention resolves at step (2) on the first widened cut; median exhausts (3) and exits with a fooling family; a genuinely novel pattern exits with a named surviving sketch — the specification of the missing rewrite, queued for the vocabulary.

## 7. Contractions: the limit of fusion

Contractions carry intrinsic I/O lower bounds: an n×n matmul moves Ω(n³/√S) words under any schedule.

**Proposition 7.1.** Fusing (A·B)·C saves the Θ(n²) intermediate against per-matmul traffic Θ(n³/√S): a relative gain Θ(√S/n) → 0, and by Law 3.3 the merge of two compute-bound GEMMs is worth only λ. Large contractions are fusion-inert; fuse epilogues into them, not them into each other.

**Proposition 7.2 (Thin-dimension exception).** If a contracted dimension d = O(√S), the intermediate (n² for attention) exceeds the tile boundary (Θ(nd)), inverting Law 3.5's ratio. Attention with head dimension 64–128 sits exactly here: Prop 7.2 supplies profitability, Theorem 5.2 supplies legality — their intersection is FlashAttention.

**Slogan.** Fusion lives in the gap between an operator's intermediates and its I/O lower bound; contractions close the gap, pointwise operators leave it wide open.

## 8. Complexity

**Theorem 8.1.** Cost-optimal legal fusion is NP-hard, already for independent pointwise operators.
*Proof.* With footprints s₁…s_n, capacity S, λ > 0: traffic is plan-invariant, so Cost = const + |P|·λ, and minimizing block count under Σs_i ≤ S per block is Bin Packing. ∎

**Theorem 8.2.** On chains, convex blocks are intervals, and cost(i) = min_{j<i} [cost(j) + Time(j+1..i)] solves optimal fusion in O(n²); trees and series-parallel DAGs admit analogous DP.

The implied architecture: fuse pointwise regions greedily (exact by Law 3.2); fuse across reductions where §5 licenses; fuse epilogues into contractions per §7; solve the small residual boundary problem by DP or ILP. Hardness (8.1) is why the last step is on the *residual*, not the whole graph.

**Guarantees for greedy.** A universal constant factor is impossible, for two structural reasons. *Fan-out:* a cheap producer feeding m capacity-separated consumers puts every partition — greedy or optimal — a factor Ω(m) above the cover that rematerializes it (Prop 4.1), so bounds must either stay within the partition class or admit replication as a greedy move. *Interference:* feasible contraction sets are not subset-closed ({x-w, w-z, y-z} is legal on x→w→z, x→y→z while its subset {x-w, w-z} creates the quotient cycle through y), so matroid and submodular arguments are unavailable. What survives: (i) max(a,b) ≤ a+b ≤ 2max(a,b) reduces roofline cost to the additive surrogate at factor 2, so surrogate bounds transfer; (ii) when neither capacity nor sharing binds, every safe merge is profitable (Law 3.4) and any maximal greedy is exact; (iii) in the capacity regime, decreasing-footprint greedy is First-Fit-Decreasing and inherits its 11/9 bound on the launch term — tight, since the instance is bin packing; (iv) greedy by maximum marginal saving realizes ≥ Sav*/(1+Δ), Δ the interference degree, by sequentially charging each blocked optimal merge to the at-least-as-good merge that blocked it — a savings bound that yields only additive cost guarantees, since cost is the complement objective. The strongest statement is parameterized exactness rather than approximation: DP over a tree decomposition of the residual quotient is exact in polynomial time for bounded treewidth (pseudo-polynomial in S; FPTAS by footprint rounding), and real model graphs have small residual width — the NP-hardness of 8.1 is confined to dense interference that they do not exhibit.

## 9. The price of fusion

The globally optimal execution of G is its optimal red–blue pebbling, which need not decompose into convex blocks each run to completion. Define
> PoF(𝒢) = sup_{G∈𝒢} (best cover-plan cost) / (optimal pebbling cost).

PoF = 1 on pointwise chains (Law 3.2). In general, schedules that interleave and revisit blocks — persistent megakernels, cross-kernel software pipelining — lie outside every partition- or cover-structured plan; PoF measures exactly what that restriction costs, and megakernels are the attempt to escape it.

## 10. Open problems

1. **PoF bounds.** Is the price of fusion O(1) on bounded-degree DAGs, or can it grow with |V|?
2. **Carrier synthesis.** With a state budget k, carrier existence is decidable (search combine/into/out modulo one associativity constraint); the undecidable remainder is *specification reframing* — choosing to verify softmax∘linear rather than softmax, which no rewriting inside a fixed vocabulary performs. Characterize the reframings mechanically discoverable from linearity annotations alone.
3. **Cover optimization.** Approximation algorithms for joint fusion + rematerialization (convex covers with capacity); the partition case inherits bin packing's landscape, the cover case is open.
4. **Memory hierarchies.** With ℓ levels, plans become ℓ-nested partitions; do Laws 3.2–3.5 relativize per level, and does per-level convexity remain the exact legality condition?

---

## Appendix A. The dimension criterion, executed

An empirical run of §5.7's rank test (finite Hankel matrices H[p,s] = h(p·s) over random prefixes/suffixes, numerical rank by SVD at threshold 1e-8; elements are (score, v) pairs with head dimension d = 4). Results, and what they force us to sharpen in the main text.

**A.1 The measurement.**

| h | coordinates | measured rank |
|---|---|---|
| Σ scores | raw | 2 (= 1 state + 1 affine offset) |
| softmax(s) @ V, per row | raw output | full (37/40) |
| softmax(s) @ V, per row | normalizer deferred: output (ℓ, o) = (Σeˢ, Σeˢv) | 6 (= 5 state + 1 affine offset) |
| median | raw | full (40/40) |
| median | exp features | full (40/40) |

**A.2 Findings.**

1. **The raw test correctly refuses attention.** The final division makes the futures a curved (nonlinear) family; the linear Hankel rank is full. This is not a failure of the criterion — it is the criterion reporting, truthfully, that *softmax as written* is not streamable (§5.2's instance). The linear rank test decides streamability of the function in the coordinates it is handed.

2. **One rewrite collapses it.** Applying Theorem 5.2 (defer the division into `out`) and the exp homomorphism of §5.6, the measured rank drops to 1 + (1 + d): the scalar denominator ℓ and the d-dimensional unnormalized accumulator o. FlashAttention's carrier appears as a measured number — but only *after* the rewrite. The running max m contributes nothing to the rank: it is numerical stabilization, not semantic state. The semantic carrier is (ℓ, o); (m, ℓ, o) is its floating-point implementation.

3. **Erratum to §5.7's parenthetical.** "Attention output: dimension 3 — the (m, ℓ, o) carrier appears as a measured rank, not a design" compresses two steps into one. Measured literally on the raw output the rank is full; the small rank is a property of the *deferred* form, i.e. of the rewrite-equivalence class (F_rw), not of the graph as written (F_syn). The correct statement: the dimension criterion is exact per coordinate system; the Nerode dimension of §5.1 is the infimum over semantics-preserving reparameterizations, and the linear test certifies only the coordinates it is run in. A full-rank result is therefore ambiguous between a true wall (median: full rank under every feature map tried, consistent with the Theorem 5.3 fooling set, which settles it) and a removable one (the division). Disambiguation is exactly the rewrite search of §5.5 — with the rank test as its cheap, per-candidate oracle. This sharpens the stratified-synthesis claim: the loop is *rewrite-candidate → rank → extract*, not *rank → extract*.

4. **Constructive extraction (linear coordinates).** Where the rank is small the factorization is the kernel, by standard realization theory (Ho–Kalman / subspace identification): factor H ≈ U·W; the state of prefix p is its row of U; `step` is solved from one-element extensions p ↦ p·x (least squares in the linear case); `out` is the empty-suffix column of W. Property-testing per §5.4 then promotes the extracted (step, out) from fits-the-samples to trusted. For nonlinear settings the annihilating-recurrence route of §5.7 remains the certification path; the SVD pipeline is its linear-coordinates special case, made practical.

**A.3 Consequence for Problem 2.** The experiment locates the specification-reframing frontier precisely: no procedure inside the raw coordinate vocabulary discovers rank 6 — the reframing (defer, exponentiate) had to be supplied, and was then verified in one SVD. Mechanically discoverable reframings from linearity annotations alone (the consumer's semimodule structure licensing Theorem 5.2) would have found this one: the annotation "V-contraction is linear" mechanically suggests deferral. Whether all practically occurring reframings are so annotated is exactly Problem 2.
