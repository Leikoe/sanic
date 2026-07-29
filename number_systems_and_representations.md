# Number Systems and Representations

**Status:** design, plus the first implementation. The analysis is landed —
`src/numeric.rs`, held by `tests/numeric.rs`, 235 tests green — and wired to
nothing yet. The defect in §1 is still latent on `main`.

`Dtype` is two things wearing one name: the *number system* a value inhabits
(𝔹, ℕ̄, ℤ̄, ℝ̄) and the *representation* a buffer stores it in (f32, bf16,
i8-with-a-scale). This document separates them, states the law relating them,
and locates each in the architecture — which turned out to be the real
question, and the answer is one sentence:

> **The compiler is a first-order calculus with three judgments over one
> shared context; fusion is choosing the let-structure of a term.**

---

## 1. The defect

Appending `.argmax("vocab")` to the llama decode graph is exact under f32 and
silently wrong under `--bf16`. The max value is right every time; only the
index is corrupted, and every error is exactly bf16 rounding of the index:

| host | fused | bf16 spacing there |
|---|---|---|
| 11, 13, 220 | 11, 13, 220 | 1 — exact |
| 279, 315 | 280, 316 | 2 |
| 1102 | 1104 | 8 |
| 12366, 14924 | 12352, 14912 | 64 |

`device.storage()` is a single dtype for every produced buffer, so the index
output is allocated BF16, and bf16 represents integers exactly only to 256.

The buffer at fault is the `Reduce`, not the `Coordinate`: argmax lowers to
`reduce_Min(Where(Lt(x, max(x)), Const(+∞), Coordinate))`, and the coordinate
is computed in-body inside one fused kernel — it is never a buffer. What gets
stored is the fold's output, so its system must survive the `Where` and the
sentinel. That path shaped most of §4 and §6.

`main` never trips this, because llama outputs logits, which are values. Any
graph that puts `argmax`/`topk` at a boundary under `--bf16` does.

## 2. The design: one calculus, one context

The compiler's best component already had the architecture:

```rust
pub type Env = HashMap<&'static str, Value>;     // interp.rs:39
pub fn eval(node: &Node, env: &Env) -> Value
```

The interpreter never smuggles values into nodes: expressions are pure, and
everything known *about* a free variable arrives in a context at the call.
Every defect this design chased came from the two places that violate that —
`Input` carrying a `Dtype`, and the partitioner minting buffer reads as
expression nodes with `dev.storage` baked in.

So:

```rust
// The calculus — eternal, interned, structural identity IS mathematical
// identity. `Input` is a free variable; its Dtype field is evicted.
Node::Input { name, shape }

// The context — everything ever DECIDED about a name, one entry per name.
Γ_value  : name → Value      // evaluation  (exists today: interp's Env)
Γ_system : name → Inferred   // analysis    (src/numeric.rs)
Γ_dtype  : name → Dtype      // emission    (owned by Schedule, minted at partition)
```

and three judgments of one shape:

```text
Γ_value  ⊢ e ⇓ v         eval
Γ_system ⊢ e : (S, B)     infer
Γ_dtype  ⊢ e → kernel     emit
```

**There is no second expression IR.** Stage bodies stay `ir::Node` — that is a
feature, not a leak: it is why the interpreter can evaluate every stage and the
oracle discipline works. The program layer already exists (`Schedule`); it just
owns Γ instead of scattering decisions into `Input` nodes. `ir.rs`'s doctrine
— *"there is no second graph IR"* — survives intact; an interim draft of this
document proposed reversing it, and the Γ formulation showed that was never
necessary.

**Why this is a theorem, not a taste.** Emission's own code proves that
representation facts occur only at *names*: `MetalProgram.dtypes` is keyed by
buffer name, `buffer_load` fires only at leaves, `out_ty` only at a stage's
output, and registers are uniformly f32 in between. A per-node dtype is never
consulted anywhere, so a per-node dtype field is unrepresentable information
waiting to lie. With one Γ entry per name, writer and reader *cannot* disagree
about a buffer's width — the ten-site agreement problem (§9's table, three
independent reviews) stops being expressible rather than getting fixed.

`RoundTo` stays in the calculus, deliberately: "x, rounded as bf16 rounds" is a
genuine function ℝ → ℝ — math, not storage. The rounding is explicit in the
term where it is semantic and in Γ where it is operational, and its
idempotence (rounding twice is rounding once) is what keeps the declaration's
effect independent of where the compiler cuts. Note the narrower claim: it does
*not* make numerics schedule-independent — reassociation already broke that
(§11).

### The graveyard

Five shapes for the annotation died before this one, each to a concrete
counterexample. Recorded so nobody re-derives them:

1. `Graph<NumberSystem> → Graph<Dtype>` (phase parameter) — no such phases
   exist: fused interiors never get a dtype at all.
2. `Assigned { inferred, dtype }` (accumulation) — emit consumes only the
   dtype; the system has no reader past assignment.
3. `Concrete(Dtype) | Abstract(NumberSystem)` (sum) — killed by
   `Input(f32) + Input(f32)`: computing the result needs each operand's
   *system*, and the sum forgot it.
4. `Numeric { system, storage: Option<Dtype> }` (product with optional half) —
   the `Option` modelled "not every node becomes a buffer", which is a fact
   about layers, not about nodes.
5. `Node<A>` (annotation as type parameter) — `AxisRef` holds `*const Node`,
   so genericity is viral through 194 mentions in 12 files plus 94
   pointer-identity sites where semantics depend on `Arc::ptr_eq`.

Every failure was the same failure: co-locating a decided fact with an eternal
term. The survivor separates them, following the code's own precedent —
`shape()` is derived and pointer-memoised in a resolver, never stored on
`Node`.

## 3. The calculus, named

What the IR is: a **first-order algebraic calculus with index binders**.
`Reduce` over an axis is `Σ_{i<n} e(i)` — a binding form over a statically
bounded index set — and `AxisRef { owner: *const Node, dim }` is a
bound-variable reference identified by a pointer to its binder: the
locally-nameless representation, reinvented independently. There are no
arrows, no closures, no function binders, and that absence is load-bearing:
it is why every term denotes, why inference is total, and why the
Myhill–Nerode probe can decide streamability.

The dictionary is uncomfortably complete:

| here | its standard name |
|---|---|
| `Input { name, shape }` | free variable |
| Γ_value / Γ_system / Γ_dtype | context; typing/evaluation judgments |
| materializing a buffer | let-binding |
| a `Schedule` | A-normal form — a chosen let-structure |
| fusion decisions | let-floating / deforestation |
| partition's splicing | substitution |
| "distinct spaces need distinct names" | the Barendregt convention |
| interning | hash-consing |
| the semantics quotient (`derive.rs`) | observational equivalence up to rounding |
| `derive.rs` itself | Bird–Meertens program calculation |

The last row is the tell: `core_ideas.md` opens with the third homomorphism
theorem, which *is* the Bird–Meertens school. The repo was already doing
constructive algorithmics; Γ makes the substrate match the method.

Three theorems become cheap tests:

1. **Substitution lemma** — splicing preserves the judgment. The law
   `partition` is held to when it cuts and records Γ_system entries.
2. **Weakening** — an unused Γ entry changes nothing.
3. **Subject reduction, with the chain as subtyping** — *simplification may
   sharpen a system down the chain, never widen it up.* `x − x → 0` may turn
   ℤ̄ into 𝔹; nothing may turn ℕ̄ into ℝ̄. One property test over `simplify`
   against `infer`.

What to refuse: everything else. Function types would inherit
capture-avoiding substitution and normalization machinery and end the
decidability `derive` rests on. The naming above earns its place only where a
lemma becomes a test; as prestige it is exactly what CLAUDE.md's epigraph
warns about.

## 4. `NumberSystem` — implemented

```rust
pub enum NumberSystem { Bool, Natural, Integer, Real }   // derive(PartialOrd, Ord)
```

Ordered by inclusion, and a **chain**: 𝔹 ⊂ ℕ̄ ⊂ ℤ̄ ⊂ ℝ̄. `join` is `max`;
`is_exact` is `<= Integer`. No lattice machinery, no priority field, and none
of the incoherence ordering *representations* produces (Appendix A). 𝔹 sits at
the bottom because the IR already encodes predicates as 0.0/1.0. Variant order
*is* the inclusion law and is pinned by a test, since nothing else checks it
and inserting ℚ later must not silently rewrite every comparison.

**ℂ is deliberately absent.** Nothing in this compiler can produce or store a
complex value, and the link ℝ̄ ⊂ ℂ is false — ±∞ has no image in ℂ — so `max`
would not be a join at the top. RoPE stays what it is: real pairs over an
extent-2 axis.

**The systems are extended: ±∞ adjoined throughout.** Forced, not chosen —
`Monoid::Min`'s identity is `+∞`, `Max`'s is `−∞`, so every order fold injects
a sentinel and argmax's index accumulator starts at `+∞`. Two consequences,
both learned the hard way:

- **`Const(±∞)` joins as the bottom system.** It is an identity element
  belonging to every extended system. Classifying it from its value ("not an
  integer, therefore ℝ̄") sends argmax's `Where` to ℝ̄ and the design fails to
  catch its own motivating bug. A finite sentinel is not an escape:
  `extremum_filtered_payload` requires `v == ties.identity()`
  (`derive.rs:1412`), so anything finite unfuses argmax.
- **NaN is outside every system.** `∞ − ∞`, `0 × ∞`, `∞/∞` are NaN, so the
  arithmetic rows are closed only on the finite part. A NaN means something
  upstream was wrong — the position `SANIC_NANSCAN` already takes. (Live
  instance, independent of this design: `ones_like` is `Add(Mul(x, 0), 1)`,
  which returns NaN on any tensor containing −∞ — and llama builds such
  tensors via `visible.select(0.0, NEG_INFINITY)`.)

## 5. Signatures — implemented

Every operation declares what it accepts and what it produces; inference
checks and takes the closure. The join of the inputs is a floor, not the
answer — how far above it the result sits is the op's own business. The rows
that were corrected during review, with their counterexamples:

| rule | why |
|---|---|
| `Add` floors at **ℕ̄**, not 𝔹 | 1 + 1 = 2; `grad.rs:163` sums a 0/1 mask to count ties |
| `Where` = join of **branches only** | the condition is tested `!= 0`, and `one_hot_like` = `Sub(Sub(1,Lt),Lt)` is ℤ̄-valued while holding {0,1} — pinning the condition to 𝔹 breaks `topk` |
| `Sub`, `Neg` floor at ℤ̄; `Div`, `Recip` at ℝ̄ | no inverse in ℕ; none in ℤ |
| `Lt` produces 𝔹 `[0,1]` | verified across all three backends |
| folds are **not** maps | a fold injects its identity and grows bounds n-fold: `Reduce{Add}` over `[0,h]` is `[0, n·h]`; `Reduce{Mul}` saturates; `Max`/`Min` add the sentinel |
| `Gather` produces the **source's** system | the index is a precondition, not a component |
| `Reindex{padded}` unions `{0}` | the pad value enters the range |
| `RoundTo` accepts anything, produces ℝ̄ | rounding an exact value *demotes* it — honest, and Stage 2 needs `stored()` as its only declaration surface |
| `Coordinate`, `Iota` produce ℕ̄ bounded by the extent | the only exact producers |

A signature table is not an operation-specific rule: it is part of what each
op *is*, like `MapOp::arity()`, and no rule anywhere names argmax,
coordinate, or topk. The motivating defect is caught by composing three rows.

## 6. `Bounds` — implemented, with the union lesson

```rust
pub struct Bounds {
    pub lo: Option<i64>,      // range of the FINITE part; None = unbounded
    pub hi: Option<i64>,
    pub infinite: bool,       // separately: can ±∞ occur?
}
```

The finite range and the possibility of ±∞ are **two separate facts**, and the
first implementation proved it by failing: an endpoint representation
(`NegInf | Finite(i64) | PosInf`) collapsed `[0, 128255] ∪ {+∞}` into
`[0, +∞]`, destroying exactly the bound that decides whether bf16 can hold an
index. Three tests failed; the doc's contract was right and the type could not
express it. `tests/numeric.rs` pins the fix: *"the sentinel must not erase the
upper bound."*

Mechanics, each with a reason: interval arithmetic is `checked_*` and
saturates to unbounded (a hand-flattened `coordinate * extent + coordinate`
overflows i64 in four multiplies; wrapping would certify a small range);
`within()` is written without `abs` (`i64::MIN.abs()` panics in debug and
*wraps* in release — a divergence this repo has been bitten by before); the
empty finite set is `lo > hi`, so `union` stays plain min/max.

Bounds feed the law and nothing after it. There is no bounds consumer past
assignment — our index arithmetic is generated in the emitter as `uint`, not
modelled in the IR, so tinygrad's emit-time width selection has no analogue
here.

## 7. The law, and who chooses

> A representation may be chosen for a value iff it faithfully carries that
> value's number system — **exactly**, for 𝔹/ℕ̄/ℤ̄ within range; **up to a
> declared rounding**, for ℝ̄ — and it must represent ±∞ if the value can be
> infinite.

Implemented as `may_store(Inferred, Dtype) -> bool`, becoming
`store_dtype(...) -> Result<Dtype, Decline>` (narrowest sufficient dtype —
`Exact{ℕ̄,[0,1000]}` under a bf16 default takes f16, exact to 2048 at the same
two bytes) when the refusal wires in. Two load-bearing details:

- **`Approximate → Ok` is deliberate.** The ℝ̄ half is unpoliced until §11;
  anything else vetoes every real buffer in llama to f32 and destroys the
  measured 16%.
- The ±∞ clause is why Stage 1 widens to **f32** and not to an integer
  format: an argmax buffer's *provable* bounds include the fold identity, and
  proving it finite ("the max is attained") needs a rule that knows
  `Lt(x, max(x))` refers to the folded max — an operation-specific rule,
  forbidden. f32 holds ±∞ exactly; u32 (M15) needs a saturating store.

Who chooses, per value class:

| value | decided by |
|---|---|
| inputs — weights, tokens | **the caller**, per name, as a Γ entry |
| produced, exact system | **the law** — bounds decide; a narrower request is a compile error |
| produced, ℝ̄, declared | **the caller** — `stored(d)`, at the price of a fusion cut |
| produced, ℝ̄, undeclared | **the default policy** — today's `with_storage`, demoted from fiat to default |

`with_storage` was never wrong to *exist* — partition must price every
candidate cut at some width for a buffer that does not exist yet, and nothing
else could answer. It is wrong in the **value** it supplies. Γ_dtype minted at
partition, informed by Γ_system and policy, is the correct answer to the same
question, which is why buffers are created at the right width rather than
repaired afterwards (§9).

Format properties live on `Dtype`, derived from one `layout()` table of
(exponent, significand) bits so they cannot drift: `nbits`, `nbytes(elements)`
(bit-computed, `div_ceil(8)` — the hand-written I4 `div_ceil(2)` special case
is deleted), `is_subbyte`, `is_float`, `has_infinities` (not a synonym: fp8
formats are floats without infinities), `exact_integers_to` (bf16 → 256,
f16 → 2048, f32 → 2²⁴ — and deriving it caught I8's off-by-one, 127 not 128,
on day one). `bytes_per_element()` is a pricing weight and documented as
never-a-size, because 0.5 truncating under `as usize` is how the special case
came to exist.

## 8. The contract

**On a value.** For `(S, B)`: every runtime value lies in `S`; every *finite*
value is an integer in `B` when `S` is exact; ±∞ may occur iff `infinite`;
NaN may not (it is a defect upstream).

**Soundness is asymmetric, and it decides how inference must be written.**
Claiming ℝ̄ where the value is really ℕ̄ is *sound but unprotective* — that is
the no-annotation case, i.e. the bug. Claiming ℕ̄ where the value is really ℝ̄
is *unsound*. Over-approximation up the chain is free and buys nothing, so all
protection comes from the table being sharp. **Integrality has no safe
direction at all** — 200.5 in claimed `[0,200]` passes every range check and
still rounds wrong — so the tests check `fract() == 0.0` against the
interpreter, not containment alone.

**On an operation.** Precondition: argument systems satisfy the row (compile
error otherwise). Postcondition: for finite arguments, the result lies in the
declared system — discharged by property tests drawing from the argument
systems, ±∞ included deliberately (that is how the `Sin`/`Cos`/`LogSumExp`
closure errors were found).

**On Γ.** One entry per name; writer and reader read the same entry. A state
(`Graph::state`) is where the layers meet: a program buffer with a declared
representation fed by an expression with an inferred system, so the law is
exactly the interface condition — `graph.rs:127`'s check becomes "can this
dtype carry that expression's system", not "does it equal the global".

## 9. Staging

### M11 — the analysis, and refuse · [analysis LANDED; refusal open]

Landed: `src/numeric.rs` (system, bounds, inference, `may_store`),
`tests/numeric.rs` (16 tests, held to the interpreter), the `Dtype` layout
work. 235 tests green, clippy clean, zero callers outside tests.

Remaining: thread Γ_system across stage cuts, and **refuse** — if a produced
buffer's value is exact and its bounds exceed what the boundary dtype holds
exactly, fail the compile naming the buffer. No dtype changes anywhere, so
none of the ten agreement sites move; refusal follows *"decline, don't
guess"* and turns silent corruption into a named error. Also: the subject
reduction test (§3) — simplify never widens a system.

*Done when:* `.argmax("vocab")` under `--bf16` fails to compile with a named
buffer; f32 still compiles; llama byte-identical; the 16% untouched. The
regression test asserts on the compiled program, not a computed value — CI's
macOS runners have no GPU.

### M12 — evict `Dtype` from `Input`; Γ on `Schedule` · [todo]

The whole of the former "two graphs" milestone, shrunk to its true size:
`Node::Input` loses its dtype (and `shallow_key` its dtype component — CSE
identity becomes pure structure); `Schedule` gains the bindings table;
`partition`/`plan`/`emit_metal` read Γ instead of `input_dtypes(node)` and
`dev.storage`. Buffers are minted at the right width at the point their name
is created — producer, consumer, allocation, readback and the cost model all
read the same entry, so cuts are priced at the width they will actually have.
Declarations move to the program layer, where `Graph::state` already lives:
`graph.input(name, shape, dtype)` — and `tokens` declares
`(F32, ℕ̄ bounded by vocab)`, which closes the old open question 1.

*Done when:* every backend agrees with `eval` as today; no dtype appears in
any expression node; kernel counts and llama timings hold; the writer/reader
agreement assert exists and passes.

### M13 — *absorbed into M12.* Minting-correct replaces widening; there is no
separate pass.

### M14 — declarations replace the global · [todo]

Retire `with_storage` for per-value `stored(d)`. Motivation is **device
portability**, not hygiene: today the interpreter rounds only at explicit
`RoundTo` while Metal under `--bf16` rounds at every boundary — the same graph
answers differently per backend and nothing in it says so. `stored()` is part
of the value, reproduced bit-for-bit everywhere. Cost is kernel count, not
bandwidth: `RoundTo` is an unconditional fusion decline, so each declaration
is a cut. Verify the 16% with ABBA. Rename `DeviceProfile → DeviceSpecs` in
the same change — "profile" collides with profiling, and once the policy
leaves, the struct is finally the hardware numbers its doc promises (plus the
capabilities it is missing: compute precision, accumulator precision,
writable dtypes).

### M15 — integer buffers; u32 indices · [todo]

Writable integer dtypes (`out_ty`/`store_expr` panic today; `write_f64`
hardcodes `*mut f32`). Indices get **one width, u32**, never
lowest-provable — an index buffer is one element per row, so width selection
saves nothing and each extra writable dtype is another disagreement surface.
Saturating store for the sentinel: `+∞ → u32::MAX`.

### M16 — quantized stores · [todo] · the trigger the arc predicts

i8-with-a-scale is a representation *of ℝ̄*, not a rounding of one —
`round_to` panics on exactly this today. It is also the point `Dtype` must
become parameterised (a scale is a value, not a variant), which is when the
enum→struct question reopens; MLIR's `!quant.uniform<i8:f32, s:z>` is the
prior art.

## 10. Testing

Landed: signature soundness against the interpreter with ±∞ drawn
deliberately; integrality (`fract() == 0.0`), not containment alone; the
sentinel-preserves-the-upper-bound pin; overflow non-wrap; the format table.
To add with their milestones: subject reduction (M11); the compiled-program
regression, no GPU needed (M11); writer/reader agreement and
no-buffer-without-Γ-entry (M12) — the analogue of tinygrad's `SPEC` check,
enforced by construction instead of env flag. Always `cargo test
--all-targets`; `--release` alone skips the example test modules.

## 11. Open questions

1. **Do carrier slots need the analysis?** Carriers invent accumulator slots
   with no IR node (argmax's payload slot, counting slots, defer-scale's
   Max/Min pair), and the ℕ̄-valued ones are where exactness matters. Γ cannot
   see inside a kernel body. Safe while only boundaries are decided (M11/M12);
   possibly not at M16.
2. **Do backward graphs need the veto?** `grad`'s `winner_mask` is
   `[src == r]` from two `Lt`s; under an already-rounded representation,
   distinct values collapse and the mask picks multiple winners — §1
   reappearing in gradients.
3. **The quotient asymmetry, standing.** A declaration is demanded for a
   ~1-ulp boundary rounding while reassociation — unbounded in the worst
   case, tuner-chosen — is free. Inherited from the `derive.rs` quotient,
   admitted here, resolved only by §12's deferred policy.

## 12. Deferred: ℝ̄ precision policy

The law rejects unsound representations; it cannot *choose* among f32/bf16/f8
for a real. Exact systems are decided forward by bounds; ℝ̄ is decided
*backward* by an error budget — output tolerance ÷ downstream conditioning —
which is data-dependent. Sound static analysis is off by ~two orders of
magnitude (worst-case aligned roundings: ≈35% for a 2048-wide bf16 dot
product; realized, independent roundings: ≈0.4% — and the machine matches MLX
token-for-token), activation ranges need a calibration input, and the real
acceptance criterion is discrete and end-to-end ("does the argmax flip").
So `Policy` is irreducible for ℝ̄: best understood as a tolerance plus a
calibration distribution, defaults from **measurement** — the `SANIC_TUNE`
pattern — and explicit declines where unjustifiable. This explains, rather
than excuses, two things in the tree: `RoundTo` is a declaration because
there is nothing to infer it from, and `--bf16` is the policy input with
exactly one knob.

## 13. Not doing

- Function types, closures, normalization machinery — the calculus stays
  first-order; that is what keeps `derive` decidable (§3).
- Integer *arithmetic*. Extent-bounded integer data needs only
  compare/min/max/small-add, exact in f32 below 2²⁴; index math is emitted,
  not modelled. Forking `apply_map`/`Monoid::identity`/`simplify` would end
  the single scalar semantics `tests/laws.rs` and `tests/completeness.rs`
  rest on.
- An order over representations (Appendix A), or an operation-specific
  "indices do not narrow" rule — §7 is general; a rule naming a frontend op
  is the red flag.
- A second expression IR (§2 — the doctrine holds).

---

## Appendix A — what the neighbours do

**tinygrad** needs no system axis because its dtype reaches the ALU
(`mul.lo` vs `mul`; `DType.min/max` from bitsize) — representation determines
arithmetic determines mathematics. We cannot copy that: our IR computes
uniformly in floats, which is what `derive.rs` stands on, so storage-integral
≠ value-integral here. They still need a second axis and it is `AddrSpace`
(`ALU` = our "never materialised"), and their `weakint`/`weakfloat`/`index`
(bitsize 800, five "cannot store" guards, a spec check that they never reach
codegen) are Γ-shaped facts forced into a dtype enum. Their one global
precision knob, `sum_acc_dtype`, can only **widen** — the safe direction; ours
narrowed. Ordering representations produces documented incoherence
(`least_upper_dtype(int64, uint64) = float16`); number systems chain by
inclusion, which is why ours order and theirs cannot.

**XLA / MLIR** split exactly where §2 splits — HloValue/HloBuffer, tensor →
memref bufferization — and both fuse on *typed* values: the structural
transforms are device-parameterised, as ours are (`DeviceProfile` is "the
device the scheduler is parameterized by"). MLIR's `index` type and
tinygrad's are the same idea (platform-width, lowered late) and have no
analogue here because our index math is generated, not represented. MLIR's
`!quant.uniform<i8:f32>` is the one prior art for a two-axis *type* — storage
and expressed type together — and it appears exactly at quantization, which
is M16's trigger, independently confirmed.

## Appendix B — measurements (M1 Pro, Llama-3.2-1B, V=128,256, warm)

- `--bf16` vs f32 storage, ABBA, zero variance: **19.0 / 16.0 / 16.0 / 19.0
  ms/tok** — 16%, which M11/M12 must not disturb and M14 must re-verify.
- The 16% is **unexplained**: analytic traffic is unchanged (2488 vs 2480 MB);
  the LM head reads the same 525.9 MB at 81% of bandwidth (f32) vs 91% (bf16),
  and the gap repeats graph-wide (Σ kernels 19.43 → 16.46 ms). Per-kernel
  store efficiency, not volume. The most under-attended item in this arc.
- Sampling today: full-logit readback 15–31 µs + host argmax 204 µs ≈ 1.4% of
  a 16.9 ms step; the LM head itself is 2884 µs at 91% of bandwidth. Fusing
  greedy argmax on-device is worth ~1.3% and removes the per-token sync; the
  SonicSampler-class win here is not the sampling pipeline, it is never
  materialising 513 KB of logits for a decode step that wants one token.
