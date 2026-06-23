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

```rust
let attn = attention(q, k, v, "d", "k");        // softmax(QKᵀ)·V
let acc  = carrier::derive(&attn, "k").unwrap(); // fold over the key axis
assert_eq!(acc.slots, 3);                        // (m, ℓ, o) — derived, not written
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
cheaper feasible one. Because legality is already proven, the cost model only
*ranks* — it can pick a slow plan, never a wrong one. It fuses at small head
dims, **cuts when fusion stops paying** (SRAM pressure collapses occupancy past
the materialization it avoids), and falls back to cut when fusion is infeasible.

## Run it

```
cargo run --example derive   # print derived carriers as readable math
cargo test                   # 22 tests
```

`cargo run --example derive` shows the engine reconstructing FlashAttention from
the graph — no formula is written by hand:

```
── FlashAttention (fold over `k`) ──
carrier (3 slots) [R1, R3, R4, R5]
  into:    s0 = x0;  s1 = 1;  s2 = x1
  combine: s0 = max(a0, b0)
           s1 = a1·exp(a0 - max(a0, b0)) + b1·exp(b0 - max(a0, b0))
           s2 = a2·exp(a0 - max(a0, b0)) + b2·exp(b0 - max(a0, b0))
  project: s2 / s1
```

The tests cover the acceptance oracle, the battle-tests above, and the
scheduler's fuse-vs-cut crossover.

## Design docs

- `streamability_engine.md` — the legality engine (stages 1 & 2).
- `scheduler_engine.md` — the profitability layer.
