# 2026-08-01 — Canonical carriers are syntactic transition monoids

**Status:** mathematical resolution and compiler design consequence.

This note answers the carrier question left open by
`2026-08-01-symbolic-carriers-and-hardware-realization.md`:

> What is the general object from which lawful bounded streaming carriers
> should be derived, without adding a permanent `SlotKind` for every new
> coupling?

The answer is classical but the compiler consequence is unusually direct:

> A lawful carrier for a stream function is a monoid recognizing that
> function. Its canonical minimal form is the function's **syntactic
> monoid**, equivalently the **transition monoid** of its minimal one-way
> state machine.

`ExpShifted`, `AtExtremum`, affine recurrences, ordered selection, and plain
reductions are coordinate presentations of generated transition monoids.
They are not distinct semantic species.

The construction also corrects three claims in the current theory:

1. Suffix-future equivalence gives minimal one-way state, not by itself a
   mergeable carrier.
2. Prefix/suffix Hankel rank gives minimal **linear automaton state** in the
   linear category, not the number of carrier fields in general.
3. Running the same rank test right-to-left is not an independent merge
   certificate: the reversed matrix is a transpose and has the same rank.

The right merge object is two-sided context equivalence and the transition
algebra it generates.

---

## 1. Exact problem statement

Let `X` be the type of one streamed element and `X*` the free monoid of finite
streams under concatenation, with empty stream `ε`. Let

```text
h : X* → Y
```

be the observable computed by one candidate graph cut along one axis.

A semantic carrier is data

```text
(M, e, ⊗, lift, project)
```

such that:

```text
(M, e, ⊗) is a monoid
lift* : X* → M is the unique monoid homomorphism extending lift
h      = project ∘ lift*
```

Equivalently:

```text
lift*(ε)      = e
lift*(uv)     = lift*(u) ⊗ lift*(v)
h(w)          = project(lift*(w))
```

This definition contains every execution right Sanic needs:

- left folds use `⊗` one element at a time;
- tiles summarize independently;
- tree reduction uses associativity;
- split reductions serialize and merge elements of `M`;
- commutativity, when separately certified, permits out-of-order merges.

The carrier question is therefore an algebraic recognition problem:

> Through which smallest representable monoid does `h` factor?

---

## 2. One-way state and merge state are different quotients

### 2.1 Right-future equivalence

Define

```text
u ≡R v    iff    ∀z ∈ X*: h(uz) = h(vz).
```

This is the usual Myhill–Nerode-style future equivalence. The quotient

```text
Qh = X* / ≡R
```

is the canonical minimal state set for a deterministic left-to-right machine.
Appending a word acts on it:

```text
Tw([u]) = [uw].
```

`≡R` is a **right congruence**: `u ≡R v` implies `uw ≡R vw`. It need not be a
left congruence. Consequently `[u]·[v]=[uv]` need not be well-defined on
`Qh`.

This is the exact gap in the current statement that a determining suffix
sketch is “equivalently” a carrier. It is a one-way state candidate. More is
needed before independently produced chunk summaries can merge.

### 2.2 A minimal counterexample

Let `h(w)` return the second symbol of `w`, or a fixed sentinel if `w` has
fewer than two symbols. Over two distinct symbols `a` and `b`:

```text
[a] ≡R [b]
```

because after either one-symbol prefix, every suffix supplies the same second
symbol and the current sentinel is the same. But prepending a symbol `x`
separates them:

```text
h(xa) = a
h(xb) = b.
```

Thus `a ≡R b` but `xa ≢R xb`. Multiplication on right-future classes is not
defined.

The function still has a finite merge carrier: a chunk can summarize its
length category and first two symbols. The point is not non-streamability; it
is that the one-way quotient is the wrong algebraic object for merging.

### 2.3 Two-sided syntactic equivalence

Define the **syntactic congruence** of `h`:

```text
u ≡h v    iff    ∀x,z ∈ X*: h(xuz) = h(xvz).
```

Unlike `≡R`, this relation is stable under both left and right concatenation.
Therefore

```text
Syn(h) = X* / ≡h
```

is a monoid under `[u]·[v]=[uv]`.

This is the canonical semantic carrier.

---

## 3. Minimal-carrier theorem

### Theorem 1 — canonical carrier

For every function `h : X* → Y`, `Syn(h)` is a lawful carrier for `h` and is
a quotient of every other reachable lawful carrier recognizing `h`.

### Proof

The quotient map

```text
ηh : X* → Syn(h),    ηh(w) = [w]
```

is a monoid homomorphism because `≡h` is a two-sided congruence. Define

```text
projecth([w]) = h(w).
```

This is well-defined by taking empty left and right contexts in the definition
of `≡h`. Hence `h = projecth ∘ ηh`.

Now let

```text
η : X* → M
h = project ∘ η
```

be any other reachable carrier. If `η(u)=η(v)`, then for every `x,z`:

```text
η(xuz) = η(x)η(u)η(z) = η(x)η(v)η(z) = η(xvz),
```

so `h(xuz)=h(xvz)` and therefore `u ≡h v`. Thus

```text
kernel(η) ⊆ ≡h.
```

The map `η(w) ↦ [w]` is consequently a well-defined surjective monoid
homomorphism from the reachable part of `M` onto `Syn(h)`. Every recognizing
carrier contains at least the distinctions of `Syn(h)`. ∎

This is the exact sense in which the syntactic monoid is minimal. A concrete
machine presentation can contain redundant coordinates, a numerical gauge,
or unreachable states; all of those quotient onto the semantic carrier.

---

## 4. Transition-monoid construction

The canonical carrier can be constructed from the minimal one-way state,
without pretending that the state itself has a merge.

Every word `w` induces a state transformation

```text
Tw : Qh → Qh
Tw([u]) = [uw].
```

For words `u,v`, in stream order:

```text
Tuv = Tv ∘ Tu.
```

Use program-order composition

```text
Tu ⊗ Tv := Tv ∘ Tu.
```

Then the generated transformations form a monoid with identity `Tε`.

### Theorem 2 — syntactic monoid equals transition monoid

The image

```text
Th = {Tw | w ∈ X*} ⊆ End(Qh)
```

is isomorphic to `Syn(h)`.

### Proof

The map `w ↦ Tw` is a monoid homomorphism under program-order composition. Its
kernel is:

```text
Tu = Tv
iff ∀x: [xu] = [xv]
iff ∀x,z: h(xuz) = h(xvz)
iff u ≡h v.
```

The first isomorphism theorem gives `X*/≡h ≅ Th`. ∎

The executable carrier is now immediate:

```text
identity       = identity transformation
lift(x)        = Tx
combine(A, B)  = B ∘ A
project(A)     = output(A(initial_state))
```

Associativity is inherited from function composition. It is not a separate
guess or a property test that synthesis might accidentally miss.

### Compiler interpretation

The most general chunk summary is not “the accumulator state after starting
from the identity.” It is:

> the transformation this chunk performs on any reachable incoming state.

For ordinary reductions, the transformation family is parameterized by the
same data as the state, so the distinction disappears. For more general
recurrences it is load-bearing.

---

## 5. Why one scalar of sequential state can need a larger carrier

Consider the affine recurrence

```text
q' = a·q + b.
```

A sequential evaluator carries one scalar `q`. A chunk, however, denotes an
affine transformation

```text
T(q) = A·q + B,
```

represented by two scalars `(A,B)`. If the left chunk is `(A₁,B₁)` and the
right chunk `(A₂,B₂)`, then

```text
(A₁,B₁) ⊗ (A₂,B₂)
    = (A₂A₁, A₂B₁ + B₂).
```

This is associative because it is affine-function composition. The state
dimension is one; the merge-carrier dimension is two.

The distinction also gives a clean boundary for sequential recurrences. For

```text
q' = q² + x,
```

one step is a quadratic transformation, two steps generically have degree 4,
and `n` steps degree `2ⁿ`. The one-way state still has one scalar, but no
fixed-degree polynomial family of state transformations is compositionally
closed. This proves failure of every bounded-degree polynomial carrier
template. It does not rule out pathological encodings; representation-relative
limits must always say which map theory they quantify over.

---

## 6. “Small” is relative to a representation category

The abstract syntactic monoid always exists as a quotient set. That does not
mean it has a useful finite program representation.

Without restrictions, “`k` real parameters” is not a sound invariant:
arbitrary discontinuous encodings can pack tuples of reals into one real.
Conversely, requiring smooth, semialgebraic, affine, polynomial, or
compiler-expressible maps produces different notions of dimension.

Sanic therefore needs to state a category of admissible presentations. A
practical choice is not one grand mathematical category but a stratified
grammar:

```text
finite products of primitive scalar monoids
products and quotients
affine / matrix transformation monoids
semilattice-indexed payload monoids
bounded-degree polynomial or rational transformation families
piecewise combinations with explicit guards
```

For each stratum, “small” means a bounded schema expressible in that grammar,
and minimality is relative to that stratum plus a target cost.

The universal semantic object remains `Syn(h)`. The grammar controls which
finite presentations of it the compiler can discover and execute.

---

## 7. The exact role of Hankel rank

For a scalar function over a field,

```text
H[p,s] = h(ps)
```

is the classical Hankel matrix. Finite rank `r` is equivalent to a
`r`-dimensional linear weighted-automaton realization:

```text
h(x₁…xₙ) = α A(x₁)…A(xₙ) ω.
```

This is a theorem about the category of linear representations. Its rank is
the dimension of minimal linear **one-way state**. It is not, in general, the
number of scalar fields in a nonlinear monoid presentation.

### 7.1 Constructing a linear merge carrier

Once the minimal transition matrices `A(x)` are known, close them under matrix
multiplication and linear span:

```text
Ah = span { I, A(x), A(x₁)A(x₂), … } ⊆ End(K^r).
```

`Ah` is the syntactic associative algebra—the linear transition monoid. If its
dimension is `a`, choose a basis `B₁…Bₐ` and structure constants

```text
BᵢBⱼ = Σₖ μᵏᵢⱼ Bₖ.
```

A chunk is represented by coordinates `c ∈ Kᵃ`; `combine` is the bilinear
program supplied by `μ`; `project` applies the represented matrix between `α`
and `ω`.

For a reachable minimal linear automaton:

```text
r ≤ a ≤ r².
```

The upper bound is `Ah ⊆ End(K^r)`. The lower bound follows because the orbit
map from the transition algebra to the reachable state space is surjective.

Thus ordinary Hankel rank is a lower bound on linear merge-algebra size, not
its exact size.

### 7.2 A two-sided context matrix

The linear merge object is measured directly by

```text
C[u,(x,z)] = h(xuz).
```

Rows are middle chunks; columns are two-sided contexts. Under a minimal
observable linear realization:

```text
C[u,(x,z)] = α A(x) A(u) A(z) ω.
```

The context functionals separate elements of the syntactic algebra, so the
rank of the full context matrix is `dim Ah`. Its centered/affine rank measures
the affine hull of reachable transformations, which can remove fixed
homogeneous coordinates such as the constant `1` in an additive translation.

This is the pool-free numerical instrument that corresponds to merge-carrier
size. The current `H[p,s]` instrument corresponds to one-way state size.

### 7.3 Why reversing the current test adds no theorem

Switching from `h(ps)` to `h(sp)` transposes the complete Hankel matrix.
Therefore

```text
rank(H) = rank(Hᵀ)
```

identically. Matching sampled ranks in both directions is a useful harness
symmetry check, but it cannot independently certify merge existence. The
transition-algebra construction or a two-sided congruence check is the merge
certificate.

### 7.4 Constructive extraction in the linear stratum

Given a complete finite Hankel block:

1. Factor `H = P S` at rank `r`.
2. For each input symbol or symbolic input feature, form
   `Hₓ[p,s] = h(p x s)`.
3. Solve `A(x) = P⁺ Hₓ S⁺`.
4. Close `{I,A(x)}` under multiplication and linear independence.
5. Compute basis structure constants.
6. Emit `identity`, `lift`, `combine`, and `project` from those constants.
7. Promote the sampled result only after symbolic or property-based
   verification against the graph.

This is an actual synthesis algorithm, not only a dimension estimate.

---

## 8. Indexed payload monoids as a generic presentation

The indexed construction in `examples/indexed_carrier.rs` is one important
finite presentation of a transition monoid. It can be stated without a
special carrier kind.

Let:

- `(J,∨,⊥)` be a join semilattice;
- `F : J → Mon` be a functor from the order category of `J` to monoids;
- for `i≤j`, write `Fᵢⱼ : F(i) → F(j)` for its transport homomorphism.

Define the total state

```text
ΣF = { (j,p) | j∈J and p∈F(j) }
```

and multiplication

```text
(i,p) ⊗ (j,q)
    = (k, Fᵢₖ(p) ·ₖ Fⱼₖ(q))
where k = i∨j.
```

The identity is `(⊥,e⊥)`.

### Theorem 3 — indexed total monoid

`ΣF` is a monoid. It is commutative when every fiber monoid is commutative.

### Proof

For three keys let `m=i∨j∨k`. Expanding either parenthesization and using that
transport preserves multiplication and composes gives the same payload:

```text
Fᵢₘ(p) ·ₘ Fⱼₘ(q) ·ₘ Fₖₘ(r).
```

Fiber associativity completes the proof. Functoriality and preservation of
identity give the total identity. If fiber multiplication is commutative,
swapping the two inputs changes neither the joined key nor the payload. ∎

### 8.1 Stable exponential statistics

```text
J             = extended scores under max
F(m)          = additive sufficient-statistic vectors
Fₘₙ(p)        = exp(m−n)·p
```

Functoriality is the telescoping law

```text
exp(m−n) exp(n−k) = exp(m−k).
```

This is the construction formerly exposed as the `ExpShifted` slot kind. It is
now stored as an `IndexedPayload` component whose transport is the executable
combine program.

### 8.2 Extremal payload filtering

```text
J             = keys under max (or min)
F(k)          = the payload tie monoid
Fₖⱼ(p)        = p when k=j, else payload identity
```

The losing transport is the constant homomorphism to identity. The resulting
total monoid is the construction formerly exposed as `AtExtremum`; it now uses
the same `IndexedPayload` provenance with a different executable transport.

Neither construction needs a permanent enum variant. Each is ordinary data:
a key algebra, a fiber algebra, transport programs, and functoriality
certificates.

---

## 9. Semantic carrier versus machine representation

Let `M=Syn(h)` be the semantic carrier. A machine representation may use a
different monoid `R` with a decoding homomorphism

```text
decode : R → M
```

satisfying

```text
decode(eR)       = eM
decode(a ⊗R b)   = decode(a) ⊗M decode(b).
```

The visible result is `projectM ∘ decode`.

This is the correct home for:

- stable max rebasing;
- alternate gauges;
- redundant physical fields;
- packed or distributed state;
- serialized partial carriers;
- target-specific refinement.

Literal equality in `R` is not required when decoded equality is the stated
semantic contract. However, approximate floating-point agreement is not an
equivalence relation and must not be called a quotient. Numerical contracts
must instead be one of:

- exact refinement over real semantics plus an explicit error bound;
- equality after an exact decode;
- a fixed deterministic evaluation tree;
- bitwise machine equality.

A relative tolerance remains a testing metric, not an algebraic congruence.

The running maximum `m` in stable softmax illustrates the split. The semantic
carrier can be unshifted sufficient statistics `(ℓ,o)` over exact reals. The
machine carrier `(m,ℓ̂,ô)` is a stable representation with a decode. `m` is a
numerical gauge field, not necessarily a semantic-state dimension.

---

## 10. Constructive discovery for Sanic

The following loop is general enough to replace a growing `SlotKind`
vocabulary while remaining executable.

### Step 1 — choose the observable cut

Form `h : X*→Y` for a convex candidate cut. Apply only certified semantic
rewrites such as linear-consumer deferral, invariant hoisting, and structural
coordinate normalization. Streamability belongs to this chosen observable.

### Step 2 — derive or infer one-way residual state

Compute a finite presentation of the residual family

```text
Ru(z) = h(uz).
```

Possible engines, from strongest certificate to weakest candidate:

- symbolic derivative closure modulo verified normalization;
- finite-state minimization on finite domains;
- Hankel factorization in the linear stratum;
- template synthesis from the existing fold grammar;
- collision/rank probing as a candidate generator.

The result is a state schema `Q`, an initial state, singleton transition, and
output map. At this point only sequential bounded-state evaluation is known.

### Step 3 — monoidify by transition closure

Represent the transformations generated on `Q` by singleton inputs and close
that representation under composition. Search presentations in increasing
strata:

```text
self-translation / primitive monoid
product
affine or matrix action
indexed payload transport
bounded polynomial/rational action
guarded piecewise action
```

Composition is `combine`. The closure proof is the associativity certificate.
Quotient transformations that no two-sided context distinguishes.

### Step 4 — choose a numerical representation

Construct `R → M` for stable arithmetic. Prove the decode laws separately
from floating-point error and deterministic-order claims.

### Step 5 — expose realization rights

The semantic certificate carries the construction evidence for:

```text
associativity reason (primitive/indexed theorem or transition composition)
merge-order right (commutative construction or program order)
identity and empty-chunk behavior
serialization reason (explicit scalar coordinates)
decoded-equality contract
```

The hardware search consumes these rights but does not modify them.

### Step 6 — test the right negative claim

A candidate sketch is merge-sufficient only when its equality is stable under
two-sided contexts, or when a concrete composition law has been constructed.
Upgrade probes from

```text
h(uy) versus h(vy)
```

to sampled

```text
h(xuy) versus h(xvy),
```

and separately test that candidate-summary equality is a congruence. Growing
two-sided fooling families lower-bound the merge carrier directly.

---

## 11. Concrete source architecture

The existing executable `Carrier` is close to the right normal form:

```text
identity
into
combine
project
```

Those programs should remain the backend-facing semantic data. What should
change is their provenance and metadata.

```text
CarrierLaw {
    state_schema
    identity
    lift
    combine
    project
    certificate
}

LawCertificate {
    associativity: PrimitiveAndIndexedMonoids | TransitionComposition
    merge_order: CommutativeConstruction | ProgramOrder
    serialization: ScalarCoordinates
}

construction :=
    primitive monoid
  | product
  | indexed total of key/fiber/transport programs
  | generated transformation closure
  | representation refinement through decode
```

These are universal algebraic constructors, not workload-named cases.

Implemented consequences:

- `SlotKind` disappears as semantic vocabulary.
- `combine` remains the actual executable truth.
- primitive matching reads `combine`, schema, and certificates.
- `spans` moves into `LogicalStateSchema`; it is not an algebraic law.
- `aliases` belongs to coordinate correspondence metadata.
- commutativity and order rights are explicit certificates, not inferred from
  an exhaustive match over current variants.
- prefix-mask elimination becomes a proved optimization over `lift/combine`
  plus range facts, not recognition of one slot arrangement.
- `transition::TransitionFamily` closes matrix directions under multiplication,
  computes structure constants, and emits executable scalar carrier programs.
- the affine recurrence is generated as the two-coordinate `(A,B)` carrier;
  its certificate permits reassociation but preserves program order.
- the two-sided context-rank regression now checks that measured action rank
  against the generated carrier dimension and its executable fold.

The migration touched every former `SlotKind` consumer:

- `derive::assemble`;
- partition's in-body coupling decision;
- `plan::mergeable_out_of_order`;
- Metal's prefix-mask optimization.

The enum is now absent. A structural regression test prevents planner,
partitioner, or Metal from matching even the universal construction-provenance
enum: those layers consume only `LawCertificate`, `LogicalStateSchema`, and
queries that verify provenance against the executable programs.

---

## 12. Relationship to Γ and hardware realization

Γ is already the right architectural pattern for a different question:

```text
expression value        = what is true
Γ(name)                  = how a materialized program name is represented
```

The analogous carrier boundary is:

```text
syntactic carrier law    = what is true
machine refinement       = how semantic state is represented
distributed realization = where fragments live and how operations schedule
```

Γ should continue to own storage widths of materialized buffer names. It
should not absorb carrier layouts, actor ownership, pipeline lifetimes, or
register fragments. Those are decisions about a realization candidate, not
about a program name's boundary representation.

The complete stack is:

```text
h : X* → Y
    ↓ two-sided semantic quotient
Syn(h)
    ↓ finite compiler presentation
Carrier { programs, LogicalStateSchema, LawCertificate }
    ↓ decode-preserving numerical refinement and physical layout
MachineStateSchema
    ↓ primitive cover, distribution, placement, schedule
DistributedRealization
    ↓ liveness and target limits
ResourceCertificate
```

---

## 13. What is solved and what remains partial

### Solved

- The canonical semantic carrier is `Syn(h)`.
- Merge is composition in the transition monoid.
- Minimality is two-sided contextual minimality.
- One-way state and merge state are formally separated.
- The indexed max/payload cases share one functorial construction.
- Linear synthesis has a concrete extraction algorithm: Hankel realization,
  transition-algebra closure, structure constants.
- Finite-dimensional transition closure and structure-constant emission are
  implemented; the graph-to-singleton-family extractor remains a separate
  symbolic-front-end problem.
- Stable machine states are refinements through a decode homomorphism.

### Necessarily partial

- A useful finite presentation of `Syn(h)` may not exist in Sanic's scalar
  expression grammar.
- Equality and associativity of arbitrary real expressions with transcendental
  functions are not generally decidable merely because a slot budget is
  fixed.
- Specification reframing remains a rewrite search over candidate cuts and
  observables.
- Numerical profitability and physical realization remain target searches.

The correct completeness statement is therefore relative:

> For a declared presentation grammar and resource bound, either construct a
> verified presentation of the syntactic transition monoid, produce a
> two-sided contextual lower bound excluding that grammar/bound, or return an
> explicit unknown.

That is strong enough to make every success a certificate and every decline
an honest, scoped claim.

---

## 14. Immediate implementation sequence

1. **Done:** add a transition-carrier example demonstrating affine recurrence state
   versus chunk-transform state and unbounded polynomial closure.
2. **Done:** correct completeness documentation: suffix probing is a one-way-state
   detector; reversed Hankel rank is not a merge certificate.
3. **Done:** add a sampled two-sided context-matrix oracle alongside the current Hankel
   test.
4. **Done:** refactor the indexed prototype around `F : J→Mon`, including an
   example whose bottom and live fibers use different payload representations.
5. **Done:** introduce construction provenance and law certificates beside the current
   executable carrier, without changing emission yet.
6. **Done:** replace `SlotKind` consumers one at a time with certificate/schema queries.
7. **Done:** remove `SlotKind` and pin its absence with a structural test.
8. **Done:** implement finite-dimensional transition-family closure, executable
   generated carrier programs, and the affine `(A,B)` bridge from the
   two-sided context oracle.

---

## 15. Primary references

- J. Gibbons, D. Lester, and R. Bird, “The Third Homomorphism Theorem,”
  *Journal of Functional Programming* 6(4), 1996.
  <https://doi.org/10.1017/S0956796800001908>
- J. Adámek, S. Milius, and H. Urbat, “A Categorical Approach to Syntactic
  Monoids,” 2018. <https://arxiv.org/abs/1804.03011>
- B. Balle, P. Panangaden, and D. Precup, “A Canonical Form for Weighted
  Automata and Applications to Approximate Minimization,” 2015.
  <https://arxiv.org/abs/1501.06841>
- D. Arrivault, D. Benielli, F. Denis, and R. Eyraud, “Sp2Learn: A Toolbox for
  the Spectral Learning of Weighted Automata,” 2016.
  <https://proceedings.mlr.press/v57/arrivault16.html>
