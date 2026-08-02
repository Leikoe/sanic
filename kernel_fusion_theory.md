# A Theory of Kernel Fusion

Five claims, then their development. Everything below exists to state and prove these.

1. **Legality is convexity.** A fusion plan is valid iff its blocks are convex in the dataflow DAG — a structural fact, independent of hardware and cost.
2. **Value is traffic.** Total work is plan-invariant; fusion's entire economic content is the intermediate traffic it de-materializes, bounded by the ratio of intermediates to boundary data, and worth nothing once compute-bound.
3. **Reduction boundaries fall by algebra — two laws, two freedoms.** Associativity governs how a fixed reduction executes; the right Myhill–Nerode quotient measures sequential state, while the finer syntactic congruence measures merge state: *collapsible* (closed form → no pass at all) ⊊ *foldable* (bounded sketch → single-pass streaming) ⊊ *monoidal* (associative merge → tiling and tree parallelism). Distributivity governs which reduction is computed — factorization changes work itself, and its surviving scalar fragment under a blocking nonlinearity is normalizer deferral, the algebraic content of FlashAttention. Impossibility is proved by fooling sets on the right quotient or by growth of the syntactic transition monoid.
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

Foldability composes structurally: maps ride inside `into` (before a reduction) or `out` (after); independent reductions over the same axis take the product carrier; coupled reductions take a *strengthened* product. A max coupled with Σexp(·−max) gives the online-softmax carrier (m, s), associative because exponential rescalings telescope. Filtering a payload monoid to elements tied at a key's max/min gives (extremal key, tied payload); Argmax is the max-key/min-index instance.

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
- **Negative claims** (a decline) are *detected* by collision probing — search for a sketch under which colliding prefixes have identical futures on sampled suffixes; a surviving sketch names candidate **sequential-state** slots — and *proved* by exhibiting a growing fooling family (Theorem 5.3). To promote a survivor to a merge carrier, also construct its chunk action (or combine) and validate closure, identity, and associativity.

A sound-by-evaluation, complete-by-probing pair of oracles makes the fusability frontier itself a regression-tested object.

**Practice (probe instantiation).** The sketch space must be named for the probe to be implementable. A workable stratification, by state budget k = 1, 2, …: sketches are tuples of fold-computable slots drawn from { Σφ(x), max φ(x), min φ(x), count[φ(x) > 0] } with φ ranging over a small feature dictionary — identity, coordinates, exp, log|·|, low powers, and pairwise products of already-admitted slots — searched smallest-k first so a surviving sketch is a *minimal* candidate sequential state in that dictionary. Collision pairs are generated adversarially, not only randomly: seed prefixes agreeing on the sketch but differing maximally elsewhere (mismatched length, permuted order, outlier magnitudes), since random collisions concentrate on easy agreement. Exact semantics use exact comparison. Under floating-point execution, error tolerances are validation bounds rather than an equivalence relation: approximate equality is generally not transitive and need not be a congruence. Every probe verdict (survivor, kill pair, budget exhaustion) is emitted as a replayable artifact. Budget exhaustion with no survivor is the trigger to switch from probing to fooling-set construction (Theorem 5.3); a survivor triggers transition-action extraction, not an immediate MONOIDAL verdict.

### 5.5 Tier zero, and rewrite closure

**Definition (collapsible).** h is *collapsible* along an axis if it equals a closed-form function of the axis bounds and O(1) axis-independent quantities — zero state, zero passes; the loop is deleted, not fused. Sums of expressions piecewise-polynomial in the index (indicator ranges, iota polynomials via Faulhaber) are collapsible, as is the one-hot gather Σ_r [i = r]·e(r) → e(i). Collapsible ⊊ foldable, completing the hierarchy of Theorem 5.1 downward.

Rewriting also relativizes the whole predicate. Semantics-preserving identities (x·0 → 0, gate folding, one-hot collapse) transform G itself — deleting work, deleting edges, and moving nodes between structure classes — so Law 3.1's invariance holds only per rewrite-equivalence class. Define three nested predicates:

> **F_syn** (the rules fire on G as written) ⊆ **F_rw** (some semantically equal G′ is in F_syn) ⊆ **F_sem** (an admissibly represented bounded monoid factorization exists).

The completeness probe of §5.4 samples a necessary sequential-state condition for membership in F_sem; a survivor in a cut declined by F_syn is an alarm, not yet a proof of F_sem ∖ F_syn. Transition-action extraction can confirm the miss. A confirmed miss admits two remediations — extend the carrier vocabulary, or add the rewrite that maps the pattern into it — and the irreducible residue F_sem ∖ F_rw is the specification-reframing frontier of Problem 2.

Two provisos that any implementation of the rewrite layer must respect. *Soundness is relative to a semantics*: x·0 = 0 fails under NaN and poison values, and IEEE addition is not associative, so every declared law — including the MONOIDAL tag itself — is a law of a chosen quotient semantics, and rule ordering (poison propagation before ring identities) is part of soundness. *Normalization is optimization*: the index algebra with floordiv/mod, closed under the recombination x%c + (x//c)·c = x, is what makes the tiling reindexing r ↦ (r//s, r%s) invertible, and its rewrites must be cost-guarded (accept a merge only if divmod complexity does not grow) — the rewriter is a search, not a confluent normal form.

### 5.6 The second law: distributivity

Over a semiring (⊕, ⊗), two laws grant two different freedoms:

- **⊕-associativity grants execution freedom** — how a *fixed* reduction runs (stream, tile, tree, split). It is the entire content of §5.1 and never changes W.
- **⊗-over-⊕ distributivity grants algorithmic freedom** — *which* reduction is computed. It reorganizes the term graph and changes W itself, so it lives in the rewrite layer F_rw, beyond the reach of any fusion plan.

Distributivity acts at two scales. **Hoisting** (its scalar fragment, one factor through one fold): a reduction-invariant factor moves out of `into` and into `out` — Σ(c ⊗ xᵢ) = c ⊗ Σxᵢ. Theorem 5.2 is exactly this with c the deferred normalizer, and the tier-0 collapse rules of §5.5 decompose as hoisting followed by closed-form counting. **Factorization** (nested sums): Σᵢⱼ aᵢ ⊗ bⱼ = (Σaᵢ) ⊗ (Σbⱼ) turns n² work into 2n — the generalized distributive law, whose instances include FFT, Viterbi in the (max,+) semiring, and einsum reassociation. Choosing the cost-optimal reassociation is NP-hard: the W-level twin of Theorem 8.1's Q-level hardness.

**Attention, resolved by this dichotomy.** Full distributivity is linear attention: (QKᵀ)V ⇝ Q(KᵀV), reducing O(n²d) work to O(nd²). Softmax's nonlinearity blocks the reassociation; the fragments of distributivity that survive it are exactly two scalars — the normalizer 1/ℓ through the linear V-sum (Theorem 5.2) and the shift e^m through Σexp (factorization via the exp homomorphism from (+) to (×)) — and these two fragments are precisely the (m, ℓ, o) carrier. FlashAttention is the maximal fragment of distributivity surviving softmax; fusion, licensed by associativity, is what executes when factorization is blocked.

### 5.7 Sequential state and the canonical merge carrier

The suffix equivalence of §5.1 is a **right congruence**. Its quotient Q_h is the canonical minimal deterministic one-way state: a word w acts by `[p] ↦ [p·w]`. It is not generally a monoid quotient, because `p ≡_h q` need not imply `x·p ≡_h x·q`. For example, if h returns the second symbol (or a sentinel on inputs shorter than two), the one-letter prefixes `a` and `b` have identical futures, yet prepending `x` distinguishes `xa` from `xb`. Thus `[u] ⊗ [v] = [u·v]` is not well-defined on Q_h.

The canonical merge object uses the finer **syntactic congruence**

> u ≈_h v iff h(x·u·z) = h(x·v·z) for every left context x and right context z.

**Theorem 5.4 (Syntactic carrier).** Syn(h) = List(X)/≈_h, with `[u] ⊗ [v] = [u·v]`, is a monoid recognizing h. It is minimal among recognizing monoids: if η : List(X) → M is any monoid homomorphism and h = project ∘ η, then η(u) = η(v) implies u ≈_h v, so the reachable part of M surjects onto Syn(h).

Equivalently, each chunk w induces the state transformation T_w : Q_h → Q_h given by T_w([p]) = [p·w]. With program-order composition `T_u ⊗ T_v = T_v ∘ T_u`, the transformations generated by chunks form the **transition monoid**, isomorphic to Syn(h). This is the general constructive answer: a chunk is represented by its action on sequential states. The gap between Q_h and its transition monoid is exactly the gap between one-pass foldability and associative merging. An affine recurrence `q ← a·q+b` makes it visible: sequential state is one scalar q, while a chunk needs the two-scalar action `(A,B)`, combined as `(A₁,B₁) ⊗ (A₂,B₂) = (A₂A₁, A₂B₁+B₂)`.

"k real parameters" is not an invariant without naming an admissible representation category: arbitrary real encodings can hide unbounded discrete information. Carrier-size claims must therefore be relative to a grammar (finite sets, affine maps, bounded-degree polynomials, tuples of known monoids, generated linear algebras, and so on), plus its exactness contract.

**Linear specialization.** The classical Hankel matrix H[p,s] = h(p·s) has rank r iff h has an r-dimensional minimal **linear sequential realization**. It does not in general measure the number of scalar fields in an arbitrary nonlinear carrier. From a rank factorization, solve the symbol transition matrices A_x, then close `{I,A_x}` under multiplication and linear span. The resulting syntactic transition algebra A_h ⊆ End(K^r) is the linear merge carrier; if its dimension is a, then r ≤ a ≤ r² for a reachable minimal realization. A basis and its structure constants give executable `identity`, `into`, `combine`, and `project`.

The merge-aware sampling object is the two-sided context matrix

> C[u,(x,z)] = h(x·u·z).

In the linear category its rank measures the dimension of the generated transition algebra (and centering estimates the affine hull of reachable transformations). By contrast, reversing the ordinary Hankel construction gives its transpose when the same word families are used, hence the same rank; it is a harness symmetry check, not an independent merge certificate.

**Practice (discovery loop).** First search suffix futures for a small sequential state or a fooling witness. Then synthesize each chunk's action on that state in the chosen representation grammar and close the actions under composition. Validate `identity`, `into`, `combine`, and `project`, and only then emit MONOIDAL. Finite sampled ranks are lower bounds; plateaus are hypotheses to be certified symbolically or by a derived carrier. A full linear rank remains ambiguous between a true wall, a nonlinear carrier, and a missing semantics-preserving rewrite, so it routes back into the rewrite search of §5.5 rather than terminating analysis. The executable construction and its limits are developed in `2026-08-01-canonical-carriers-and-transition-monoids.md`.

## 6. Structure over a graph

Streamability is per-(operator, axis), not per-operator: an axis contracted in one node is free in another. The executable IR classifies each pair into
> FREE (no dependence) < MONOIDAL (associative fold; LINEAR flag when Thm 5.2 applies) < OPAQUE (data-dependent access).
Maps pass structure through, scalar-monoid reductions and scans introduce MONOIDAL dependence, and gathers poison to OPAQUE. A non-associative recurrence would require a fourth, SEQUENTIAL class, but it is not admitted until the IR carries an executable step body rather than a scheduling label. The per-axis map separates the two distinct fusion mechanisms — same-axis carrier merge (attention) and cross-axis producer-consumer tiling (fused MLP) — which block-level analysis conflates, and isolates the residue (routing, sampling, decode loops) that no fold covers.

**The decision loop (sliding the cut).** Streamability of an operator in isolation is the wrong query — softmax alone is unstreamable while softmax∘linear is — so the unit of analysis is the *cut*: a convex subgraph from the axis's elements to a candidate materialization frontier. For each reduction axis, enumerate cuts outward from the reducing node toward each downstream materialization point, and for each cut form the composite h and test it: (1) syntactic tier match (§5.5 collapse rules, then the fold vocabulary); (2) on failure, fire annotation-triggered rewrites and retest; (3) on continued full linear rank within budget, run the collision probe (§5.4); (4) for a surviving sequential sketch, extract and close chunk transformations; (5) record the verdict — certified carrier, fooling family, representation-relative closure failure, or surviving hypothesis — as an artifact attached to the cut. Widening the cut past a consumer that is a function of the frontier alone cannot grow the right-quotient state requirement; a consumer that also absorbs sibling branches of the same stream bounds the widened state only by the branch product, and it can grow (Appendix B.3 measures 1 → 2 through a linear Add). Cuts are still explored in increasing width — early exit after constructing a small carrier is safe because any found carrier is valid — but the exit is a heuristic, not an optimality guarantee. The trigger table, mechanical because every trigger is an annotation already present in the graph: a *linear consumer on the reduced axis* (any matmul or semimodule map) fires normalizer deferral (Theorem 5.2); a *shift- or scale-invariant normalizer* fires the exp/log homomorphism transport (§5.6); a *reduction-invariant factor* fires hoisting; a *piecewise-polynomial index dependence* fires tier-0 collapse (§5.5); a *one-hot or gate pattern* fires the corresponding deletion rewrite. Attention resolves at step (2) on the first widened cut; median exhausts (3) and exits with a fooling family; a genuinely novel pattern exits with a named surviving state or transition-family hypothesis — the specification of the missing rewrite or carrier, queued for the vocabulary.

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
2. **Carrier synthesis.** With a finite semantic domain and finite carrier, or a finite enumerable carrier grammar with a decidable equality theory, bounded synthesis is decidable by search. Unrestricted synthesis over real-valued expressions is not made decidable merely by fixing the number of fields: equivalence and associativity inherit the expression theory's hard or undecidable cases. The separate *specification reframing* problem is choosing to verify softmax∘linear rather than softmax. Characterize the useful carrier grammars and the reframings mechanically discoverable from linearity annotations alone.
3. **Cover optimization.** Approximation algorithms for joint fusion + rematerialization (convex covers with capacity); the partition case inherits bin packing's landscape, the cover case is open.
4. **Memory hierarchies.** With ℓ levels, plans become ℓ-nested partitions; do Laws 3.2–3.5 relativize per level, and does per-level convexity remain the exact legality condition?

---

## Appendix A. The linear-state rank test, executed

An empirical run of §5.7's linear-state test (finite Hankel matrices H[p,s] = h(p·s) over random prefixes/suffixes, numerical rank by SVD at threshold 1e-8; elements are (score, v) pairs with head dimension d = 4). Results, and what they force us to sharpen in the main text.

**A.1 The measurement.**

| h | coordinates | measured rank |
|---|---|---|
| Σ scores | raw | 2 (= 1 state + 1 affine offset) |
| softmax(s) @ V, per row | raw output | full (37/40) |
| softmax(s) @ V, per row | normalizer deferred: output (ℓ, o) = (Σeˢ, Σeˢv) | 6 (= 5 state + 1 affine offset) |
| median | raw | full (40/40) |
| median | exp features | full (40/40) |

**A.2 Findings.**

1. **The raw test correctly refuses a small linear realization of attention's projected output.** The final division makes the futures a curved (nonlinear) family, so the linear Hankel rank is full. This does not say attention is unstreamable; it says that its scalar projected output is not a finite-dimensional linear state coordinate. Section 5.2 supplies a nonlinear projection from a small additive carrier.

2. **One rewrite exposes linear coordinates.** Applying Theorem 5.2 (defer the division into `out`) and the exp homomorphism of §5.6, the measured centered rank drops to 1+d: the scalar denominator ℓ and the d-dimensional unnormalized accumulator o. The running max m contributes nothing to the exact-real rank: it is numerical stabilization, not semantic state. The exact-real carrier is (ℓ, o); (m, ℓ, o) is its stable floating-point refinement.

3. **The rank is category-relative.** Measured literally on the projected output, attention's linear rank is full; the small rank is a property of the deferred coordinates. A full-rank result is therefore ambiguous between a true wall (median, settled separately by a fooling family), a nonlinear finite carrier, and a removable coordinate obstruction such as division. The loop is *rewrite-candidate → linear rank → extract transitions → close under composition*, not *rank → carrier*.

4. **Constructive extraction (linear coordinates).** Where the rank is small, factor H ≈ U·W; the state of prefix p is its row of U; symbol transitions are solved from one-element extensions p ↦ p·x; and `out` is the empty-suffix observation. To obtain a merge carrier, close the symbol transitions under multiplication and linear span, choose a basis of that algebra, and solve its structure constants. Property-testing per §5.4 checks the extracted operations against samples; an exact symbolic derivation or declared numerical contract supplies the certificate.

**A.3 Consequence for Problem 2.** The experiment locates the specification-reframing frontier precisely: no procedure inside the raw coordinate vocabulary discovers rank 6 — the reframing (defer, exponentiate) had to be supplied, and was then verified in one SVD. Mechanically discoverable reframings from linearity annotations alone (the consumer's semimodule structure licensing Theorem 5.2) would have found this one: the annotation "V-contraction is linear" mechanically suggests deferral. Whether all practically occurring reframings are so annotated is exactly Problem 2.

## Appendix B. The decision loop, tested

Adversarial verification of §6's load-bearing claims against the implementation (`tests/theory_check.rs`), run on the KV-cache decode graph whose schedule motivated them. One claim verified, one located as a vocabulary gap, one corrected.

**B.1 The widened cut resolves masked decode attention.** The full cone — scores, an additive mask computed in-graph from `iota` and a runtime position scalar, softmax, ·V — derives the 3-slot `(m, ℓ, o)` carrier in one query and its single kernel matches the interpreter at 5e-8. §6's "attention resolves at step (2) on the first widened cut" is true as stated for this form, mask included; a partitioner that only ever offers the narrow cut leaves a 5-kernel schedule on the table without any decline being recorded — the cut, not the carrier, is the missing analysis.

**B.2 The GQA head repeat was an F_syn miss — closed by an identity law, and the blocker moved to cost.** The same cone with grouped-query broadcast (keys/values repeated across query heads through an affine reindex and a flatten) declined at the normalizer sum (`still-per-element`). The function is semantically streamable — the repeat acts on a free axis — and the miss turned out to be an identity inconsistency, not a missing rule: a reindex minted a fresh axis occurrence even for dimensions it passes through untouched, while the view arm already preserved 1:1 identities. Restoring one general law — *a reindex acts only on the dimensions it transforms; an untransformed dimension keeps its identity* — makes the full GQA+masked cone derive the `(m, ℓ, o)` carrier with no operation-specific case (`tests/laws.rs::coupled_carrier_composes_through_free_axis_repeat`). The residual blocker is now §3's, not §5's: on the 32-head/64-dim decode graph the planner prices the fused kernel as having no feasible block structure and carves the normalizer division back out, splitting the cone. The compact-extent instance of the same graph fuses to one kernel. Streamability is settled; the open question is the tiling search.

**B.3 Erratum: widening is not monotone.** §6 formerly claimed "widening the cut past a linear consumer can only shrink the Nerode dimension." That holds only when the consumer is a function of the frontier alone (post-composition: futures compose, dimension cannot rise). A linear consumer that merges a sibling branch of the same stream — which convexity forces into the widened cut — yields the branches' product carrier, and the dimension can grow: Σxᵢ measures dimension 1, and widening past `Add(·, Σ i·xᵢ)` measures 2. Early exit at a small-rank frontier remains safe (any found carrier is valid) but is a heuristic. Measurement note: the counterexample is invisible to a futures matrix built from fixed-length prefixes — the shared length term is absorbed by column centering, so position-carrying state costs no rank. Rank harnesses must mix prefix lengths within one matrix; §5.7's "grow prefix length until the spectrum plateaus" is necessary but not sufficient when the growth is applied uniformly.
