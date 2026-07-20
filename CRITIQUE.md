# CRITIQUE.md

An external review of sanic at the current tree. Basis: full read of `derive.rs`,
`partition.rs` (partial), `verify.rs`, `cost.rs`, `simplify.rs`, `tests/completeness.rs`,
the four theory documents; built and tested on Linux (stable 1.97): **148/148 tests
pass across 17 suites**, `direct_attention` derives and runs, clippy reports style
lints only. Everything below is ranked by expected payoff. Items marked **[S]** are
small (< a day); **[M]** medium; **[L]** large or open-ended.

The one-line verdict first: the moat (`derive` + the completeness oracle) is real,
correctly built, and honestly documented. The debts are concentrated in three
places — numerical semantics is used but never pinned, declines are silent, and
the completeness story has no pool-free instrument. None require architectural
change.

---

## 1. Correctness and semantics

### 1.1 Pin the floating-point quotient that `derive`'s rewrites assume **[S]**

`pmul` pushes multiplication through `Div` — `(n/d)·c → (n·c)/d` — and `pexp_sub`
folds `x − x → 0`, `exp(0) → 1`. Sound over ℝ; observable under `inf`/`NaN`
(`d = 0`, overflowing `n·c`). The streamability doc's own proviso says every law
is a law *of a chosen quotient semantics*. `derive.rs` should state its quotient
in the module doc (reals-with-rounding, no `inf`/`NaN` guarantees? IEEE-with-
documented-exceptions?) and `laws.rs` should test at the boundary of whichever
one is claimed. Right now the claim is implicit and the tests can't distinguish
the two.

### 1.2 The oracle never exercises the regime the engine exists for **[S]**

The probe alphabet is 9 values in [−2, 2]; `laws.rs` operates in the same tame
range. Naive `Σ exp` with scores ~±2 never overflows — so `rescale`, the
online-softmax coupling that *is* the FlashAttention derivation, is verified for
its algebra but never for its numerical purpose. Add magnitude tiers to the law
tests: scores at ±50, ±700 (f64), ±88 (the f32 path), planted maxima early vs.
late in the stream, and assert the derived carrier agrees with a
stable reference where the naive fold visibly does not. That test failing on a
future refactor is the alarm you actually want.

### 1.3 One tolerance policy, not per-example thresholds **[S]**

GPT-2 asserts `max |Δlogit| = 1e-4` as a bespoke number. Derive-vs-eval,
f64-interp-vs-f32-Metal, and probe future-comparison each have their own ad-hoc
epsilon (`1e-9` relative in the probe). Define the tolerance policy once —
per dtype, per reduction length (error grows with n; a length-aware bound is one
multiply) — and have every oracle read it. When a tolerance is a policy, a
tightening regression is meaningful; when it's a literal, it's a vibe.

### 1.4 `is_contraction` is syntactic in exactly the way the project criticizes **[S]**

`other_axis_folds` matches literally `Reduce{Add, Map{Mul, 2 inputs}}`. A matmul
spelled `Σ x·y·mask` (3 inputs), or with an interposed cast, misclassifies and
flips `keep_map_whole`, silently changing the partition shape. Either (a) close
the pattern under commutativity/associativity of the surrounding `Mul` tree, or
(b) declare canonical form a contract that `simplify` establishes and add a test:
a de-canonicalized GEMM, simplified, partitions identically to the canonical one.
Option (b) is more in keeping with the architecture — but then `simplify` must
actually run before `partition` on all public paths, which today it does not
(it is documented as client-side).

### 1.5 Convexity is guaranteed by construction — assert it anyway **[S]**

Cutting at the derive frontier can only produce convex blocks with an acyclic
quotient, so `partition` never checks. That is an architectural accident holding
up a theorem. One debug assertion over emitted stages (topological order exists;
no stage reads a name written by a later stage) converts the invariant from
"true today" to "checked forever" — and will catch the first future feature
(cover plans, epilogue motion across stages) that breaks it.

---

## 2. The decline path

### 2.1 `derive` returns `Option<Carrier>`: declines carry no reason **[M]**

This is the largest philosophical inconsistency in the codebase. The project's
best idea is "a decline is a claim" — yet the deriver's own declines are `None`.
Make it `Result<Carrier, Decline>` where `Decline` records the node, the axis,
the `S`-state reached, and the first composition rule that had no case. Payoffs
compound: `partition` can attach the reason to the cut it makes; the
completeness ledger can bucket declines by cause; a user asking "why is my block
3 kernels" gets an answer; and the `Decline` type is the natural carrier for the
probe verdict (§3.2). Do this before `partition.rs` grows further — it is the
refactor that pays for the next one.

### 2.2 Completeness is a syllabus; it should be a census **[M]**

`tests/completeness.rs` probes ~14 curated scalar toys. The decline population
that matters is the one `partition` actually produces on real graphs — every cut
in the GPT-2 or Trinity schedule is an unexamined decline. Pipe real partition
boundaries (the cut node, streamed axis, and spliced sub-graph) into the probe
and emit a per-model decline ledger. This turns "our declines on the syllabus
are justified" into "our declines on this workload are justified," and it is the
only version of the claim a user cares about. Requires §2.1's `Decline` to know
*what* was declined.

---

## 3. The completeness probe itself

The idea is the best thing in the repository — completeness as a falsifiable,
regression-tested property, already validated by having found four rules
(`invariant`, `lattice`, `defer-add`, `defer-scale`). The critiques are aperture,
not concept:

### 3.1 Add the pool-free second oracle: the Hankel rank test **[S]**

The probe sees only through its sketch pool and says so. The dimension
criterion needs no pool: sample prefixes × suffixes, fill H[p,s] = h(p·s)
via `interp::eval`, measure numerical rank. Low rank with *no* surviving sketch
flags a miss outside the pool's expressiveness — precisely the probe's admitted
blind spot; rank growing with prefix length corroborates a decline with no
vocabulary at all. ~50 lines against existing machinery. Practical notes:
center the matrix (additive carriers cost +1 affine rank); threshold the
*relative* spectrum and require a gap; grow prefix length until plateau or
monotone growth; run right-to-left too — bidirectional low rank certifies an
associative merge exists (third homomorphism theorem) before anyone writes it,
which is exactly the certificate the top-k combine (§5.2) is missing.

### 3.2 Escalate witnesses into proofs **[S]**

A `Separated` verdict is one fooling pair; the theory's standard is a growing
fooling *family* giving a `state ≥ log₂ N` bound. For median and
`count_above_half_max` the family is a loop: emit k mutually-separated prefixes
at increasing k and print the state lower bound. A decline then ships with a
theorem, not a survey result.

### 3.3 Shrink witnesses **[S]**

`last_witness` keeps whichever separation happened last — an arbitrary artifact.
QuickCheck-style shrinking (shortest p, q, suffix that still separates) makes
the printed witness a readable proof sketch. For median the minimal witness is
genuinely pedagogical; today's is noise.

### 3.4 Multi-seed and adversarial streams **[S]**

One LCG seed makes the probe reproducible but also a single sample of a
probabilistic argument asserted as a hard test. Run 3–5 seeds; add adversarial
generators random draws under-produce: planted ties, permutations of the same
multiset, early/late outlier magnitudes. Also: `Verdict::Carrier(1 + cand.len())`
reports sketch component count, which upper-bounds but does not equal carrier
dimension (argmax reports 3 for a 2-slot carrier) — rename the field or let the
rank test (§3.1) report the real number.

---

## 4. Engineering debt

### 4.1 `Ctx::memo` is the bug its neighbor already fixed **[S]**

`memo: Vec<(*const NodeKind, S)>` with linear `find`, in the same file where
`other_folds` got a `HashMap` and a comment explaining that the unmemoized walk
is exponential on backward graphs. Backward graphs are exactly where `memo`'s
O(n²) will bite too. Make it a `HashMap`.

### 4.2 Raw-pointer memo keys are ABA-fragile **[S]**

`Rc::as_ptr` keys are correct only while no node reachable during a derivation
is dropped and its address reused mid-derivation. True today by call structure;
undocumented. Hold the `Rc` in the key (you already clone into `leaves`) or
write the invariant where the next refactorer must read it.

### 4.3 `Box::leak` for temp names is unbounded in a `Session` **[S]**

Four sites leak per compilation. Fine for a CLI; a long-lived `Session`
recompiling shapes leaks without bound. Intern (`Rc<str>` in the IR, or a
session-owned arena). At minimum, document it on `Session`.

### 4.4 `partition.rs` is accreting **[M]**

2248 lines, 2× the next-largest file, owning cut selection + epilogue
attachment + aliasing + recursive descent. The module doc's rule ("cut where
the algebra stops") is simple; the code now has the cases. The §2.1 `Decline`
refactor is the natural moment to split cut-*selection* (pure: graph → cuts +
reasons) from stage-*assembly* (effectful: cuts → stages, naming, aliasing).

### 4.5 Housekeeping **[S]**

No CI. With a suite this good, an Actions matrix (Linux check+test, macOS
+Metal, clippy, fmt) is disproportionately high-value — the macOS leg is the
only thing standing between "tests pass" and "the GPU path still works."
~25 mechanical clippy lints (indexed loops → iterators). Dead code in
`examples/` from the attic migration. `Cargo.toml` lacks `repository`/
`keywords` if crates.io is ever the plan.

---

## 5. Gaps against the theory (scoped, not sized)

Deliberate scope cuts, listed so they're chosen rather than drifted into:

- **Tier-0 collapse.** No collapsible tier: Faulhaber/iota sums and general
  one-hot gather deletion (delete the loop, don't fuse it) exist only as the
  specific backward-pass identities in `simplify`. Index-heavy graphs leave
  loop deletions on the table.
- **Factorization (W-changing rewrites).** `simplify` has hoisting — the scalar
  fragment of distributivity — but no reassociation: no linear-attention
  `(QKᵀ)V ⇝ Q(KᵀV)`, no einsum reordering. This is the known frontier beyond
  any fusion plan; fine to defer, worth naming in the README's limitations.
- **Residual boundary optimization.** Partitioning is structural/greedy with a
  roofline ranker; there is no small exact solver (DP on chains/trees, FFD on
  the capacity term) over the residual, and no exactness claim even where the
  theory grants one (pointwise chains). Cheap partial win: assert on a test
  that maximal pointwise fusion is what the partitioner in fact produces.
- **Cover plans.** Rematerialization is heuristic (free sources recompute;
  contractions cut). The per-producer inequality (recompute iff
  cost-of-recompute < materialization traffic) is one comparison against
  numbers `cost.rs` already has.

---

## 6. What not to change

Named so this reads as a review, not a demolition: the carrier-as-data design
(one symbolic artifact executed, tested, and transcribed); discovery of the
online-softmax coupling from composition rather than pattern-matching; the
foldable/monoidal distinction made executable and *guarded* at both the
interpreter and emitter for top-k; `verify` separate from construction;
`Schedule::execute == eval` as the compiler-correctness theorem made numeric;
module documentation that states caveats where they bind. The probe's
quantized-alphabet and `MIN_PAIRS` designs are subtle and correct. Keep all of
it.

---

## Suggested order

1. §2.1 `Decline` type (unlocks 2.2, 4.4)
2. §3.1 rank oracle + §1.2 magnitude tiers (the two missing instruments)
3. §1.1 + §1.3 semantics/tolerance policy (turns claims into contracts)
4. §4.1–4.3 mechanical debt, §4.5 CI
5. §2.2 decline census on GPT-2
6. §3.2–3.4 probe upgrades
7. §5 items as roadmap decisions, not code
