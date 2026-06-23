# fusion-engine

An algebraic engine that **derives** streaming kernels instead of hand-writing
them. Its headline result: given `softmax(QKᵀ)·V`, it reconstructs the
FlashAttention online accumulator `(m, ℓ, o)` — the running max, the rescaled
denominator, and the unnormalized output — *from composition rules, not a stored
template*.

## The one idea

A computation can be streamed in one pass and parallelised in a tree along an
axis **iff, viewed as a function of the elements along that axis, it is a monoid
homomorphism**:

```
H(xs ++ ys) = H(xs) ⊗ H(ys)
```

Associativity of `⊗` is the whole game: it is simultaneously the streaming
certificate (fold left-to-right, O(1) state) and the parallelism certificate
(combine blocks in any order). Everything else follows by structural recursion.

## The pipeline

```
        op            engine_ir            stage1                  carrier
   ┌──────────┐    ┌────────────┐    ┌───────────────┐   ┌─────────────────────┐
   │ operator │ →  │ 5-operator │ →  │ classify each  │ → │ derive the streaming │
   │  algebra │    │    IR      │    │ (node, axis)   │   │   accumulator        │
   └──────────┘    └────────────┘    └───────────────┘   └─────────────────────┘
```

- **`op`** — every operator carries its laws (associative? identity? linear?).
- **`engine_ir`** — five primitives: `Input · Map · Reduce · Scan · Gather`.
  `MatMul`, `Softmax`, `Attention`, SSM scans are *derived* compositions, so the
  analysis needs no special cases.
- **`stage1`** — abstract interpretation over a tiny lattice tags each
  `(node, axis)` as `FREE` (grid), `MONOIDAL` (foldable), `OPAQUE` (data-dependent
  gather), or `SEQUENTIAL` (non-associative recurrence).
- **`carrier`** — a single bottom-up fold turns a foldable axis into a concrete
  accumulator via five composition rules (R1–R5). It is **not a matcher**:
  coupling (the online-softmax rescaling, R4) and normalizer deferral (R5) are
  detected by *data dependence*, so the same code derives `sum`, `mean`,
  `variance`, `logsumexp`, and FlashAttention alike.
- **`engine::analyze`** — the front door: one call returns the **structure map**,
  the artifact both design docs name — every axis classified, with the derived
  accumulator attached to the foldable ones. `analyze_all` discovers the axes for
  you.
- **`codegen`** — emit the fused streaming kernel as Rust source: the grid/stream
  loop nest from the structure map, the body from the derived carrier, blocked by
  the query-tile the scheduler chose (`tile × |Acc|` resident).

```rust
let attn = attention(q, k, v, "d", "k");  // softmax(QKᵀ)·V
let map  = analyze_all(&attn);             // classify every axis + derive, zero-config
// k → MONOIDAL with the derived (m, ℓ, o) accumulator; sq, e → grid; d → contraction
```

## Why "derived, not matched" matters

The carrier is *data* (slots + symbolic `into` / `combine` / `project`), so each
derivation is executed and property-tested: `tree_fold == fold == reference`
(associativity *and* correctness) on random data. Because the deriver is generic,
it passes graphs no template anticipates:

- **multi-value attention** — two value tensors share one softmax → `(m, ℓ, o₁, o₂)`.
- **CTC forward** — the full log-space DP: derives `(max, Σexp)` for the
  predecessor and loss `logsumexp`s while correctly tagging the time recurrence
  `SEQUENTIAL` (refusing to fold it) and the label gather `OPAQUE`.
- **soft-attention over a streamed log-space DP** — one graph that forces R4, R5,
  `OPAQUE`, `SEQUENTIAL`, *and* the same-axis-merge vs cross-axis-tiling
  distinction at once.

CTC's `logsumexp` and FlashAttention's softmax come out as the *same* derivation
with different projections (`log(s)+m` vs `o/s`) — both built from one
exp-shifted-sum slot.

## Legality, then profitability

The algebra above decides **legality** (what *can* fuse, and the exact
accumulator). `schedule` adds the other half — **profitability**: against a
device model it costs the fused flash kernel (streams `k`, keeps `(m, ℓ, o)` in
SRAM) versus the cut two-matmul plan (materializes the scores), and picks the
cheaper feasible one. The accumulator size in the SRAM constraint is read off the
**derived carrier** (`Carrier::acc_scalars`, using per-slot axis spans), not a
magic constant — exactly the `|Acc|` handoff the design doc prescribes. Because
legality is already proven, the cost model only *ranks* — it can pick a slow
plan, never a wrong one. It fuses at small head dims, **cuts when fusion stops
paying** (SRAM pressure collapses occupancy past the materialization it avoids),
and falls back to cut when fusion is infeasible.

## Run it

```
cargo run --example derive   # print the structure map + derived carriers
cargo run --example mha       # naive multi-head attention → FlashAttention kernel
cargo test                   # 27 tests
```

`cargo run --example mha` builds *naive* multi-head attention as an AST, derives
FlashAttention from it with no MHA-specific code (batch and head are just extra
free axes), and **emits the fused kernel as Rust source** — the grid/stream loop
nest from the structure map, the body from the derived carrier:

```rust
/// Fused streaming kernel — grid over {b, h, sq, e}, stream over `k`.
pub fn flash_attention(elements: impl IntoIterator<Item = [f64; 2]>) -> f64 {
    let mut acc = [f64::NEG_INFINITY, 0.0f64, 0.0f64];
    for x in elements {
        let el = [x[0], 1.0f64, x[1]];
        acc = [
            acc[0].max(el[0]),
            acc[1] * (acc[0] - acc[0].max(el[0])).exp() + el[1] * (el[0] - acc[0].max(el[0])).exp(),
            acc[2] * (acc[0] - acc[0].max(el[0])).exp() + el[2] * (el[0] - acc[0].max(el[0])).exp(),
        ];
    }
    acc[2] / acc[1]
}
```

The scheduler then sizes it: it costs fuse-vs-cut, and when it fuses it picks the
query-tile that fits SRAM (here `tile = 64`, amortizing the K/V reads). `codegen`
emits the **blocked** kernel for that tile — `tile × |Acc|` accumulators resident
across the key stream. (Still a CPU scalar kernel; GPU/backend lowering is
downstream.)

The example shows the engine classifying attention and reconstructing
FlashAttention from the graph — no formula is written by hand:

```
structure map
  k    MONOIDAL           → fold
         carrier (3 slots) [R1, R3, R4, R5]
           into:    s0 = x0;  s1 = 1;  s2 = x1
           combine: s0 = max(a0, b0)
                    s1 = a1·exp(a0 - max(a0, b0)) + b1·exp(b0 - max(a0, b0))
                    s2 = a2·exp(a0 - max(a0, b0)) + b2·exp(b0 - max(a0, b0))
           project: s2 / s1
  d    MONOIDAL           → fold (in a sub-expression)
  sq   FREE               → grid (DOALL)
  e    FREE               → grid (DOALL)
```

The tests cover the acceptance oracle, the battle-tests above, and the
scheduler's fuse-vs-cut crossover.

## Design docs

- `streamability_engine.md` — the legality engine (stages 1 & 2).
- `scheduler_engine.md` — the profitability layer.
