# Framework Review

Reviewed against the vendored Tinygrad source and specification on July 18,
2026; repository-contract findings refreshed after the July 20 core audit, and
re-verified against the tree on 2026-08-01 (findings 1, 4 and 5 closed, the
Llama paragraph in 4 marked stale).

## Verdict

Sanic is a compelling compiler research prototype, but it is not yet a mature framework.

Its strongest idea is genuinely differentiated: deriving online carriers from ordinary tensor expressions, especially across dependent reductions. Tinygrad does not generally solve that problem—its normal softmax remains multiple kernels, while several deeper fusion cases are still skipped in its tests.

However, Sanic currently has important gaps between:

- the algebra it claims;
- the schedule it plans;
- the code it emits; and
- the functionality its documentation presents.

| Area | Current state |
|---|---|
| Core research idea | Strong and distinctive |
| High-level IR | Promising, but contracts are inconsistent |
| Fusion derivation | Sanic's clearest competitive advantage |
| Scheduling | Prototype; partially disconnected from code generation |
| Lowering and memory | Substantially behind Tinygrad |
| Frontend and runtime | Useful fixture, not yet framework-quality |
| Correctness evidence | Good focused and full-suite coverage; CI enforces it (no GPU on the runners) |
| Documentation | Theory docs are honest and cite real code; the README overstated what the tree builds (fixed 2026-08-01) |

## Most serious problems

### 1. The planner does not describe the kernel actually emitted ~~(open)~~ — RESOLVED

*Resolved by deletion, not by wiring.* `KernelSpec` no longer claims to be a
resolved physical plan: it carries five fields — streaming axis, carrier, input
names, output name, cost — and its own docstring now states that the physical
schedule is chosen downstream by `fold_sched`. The tile sizes, row/column axes,
batch roles, and SRAM/register estimates that nothing read were removed rather
than plumbed through, so the planner can no longer score choices that never
reach generated code. The measured tuner (`SANIC_TUNE=1`) is the sanctioned way
the device overrules the cost model, and it is on the ordinary execution path.

The original finding, for the record: `KernelSpec` contained tile sizes, row and
column axes, batch roles, and resource estimates that the Metal emitter ignored
while independently choosing a `FoldSched` — so the cost model could score tile
choices that never reached code, and reported SRAM/register estimates did not
necessarily describe the emitted kernel.

Relevant code:

- [`src/plan.rs`](src/plan.rs)
- [`src/emit_metal.rs`](src/emit_metal.rs)
- [`src/partition.rs`](src/partition.rs)

### 2. The numerical contract must remain explicit

The IR now admits only scalar monoids with complete interpreter and backend
semantics. Their numerical contracts still differ:

- Floating-point addition and multiplication are not exactly associative.
- Maximum and minimum behavior depends on NaNs and signed zero.

The implementation needs separate concepts for:

- algebraically associative operations over ideal values;
- numerically reorderable operations within a stated tolerance;
- executable operations supported by a backend.

At present, the word "monoid" hides these distinctions.

Relevant code:

- [`src/ir.rs`](src/ir.rs)
- [`src/derive.rs`](src/derive.rs)
- [`src/interp.rs`](src/interp.rs)
- [`tests/laws.rs`](tests/laws.rs)

### 3. The lowering stack is too implicit

Tinygrad's specification covers tensor operations, symbolic ranges, loads, stores, barriers, dependencies, WMMA operations, programs, and memory planning. Sanic jumps from semantic partitions toward handwritten Metal rendering.

That leaves no authoritative representation for:

- loop and thread ranges;
- memory address spaces;
- synchronization;
- buffer lifetime and reuse;
- copies and device dependencies;
- tensor-core operations; or
- general effects.

This missing layer helps explain the planner/emitter split: there is no intermediate object capable of carrying the complete physical decision.

Relevant Tinygrad references:

- [`references/tinygrad/spec/tinyspec.tex`](references/tinygrad/spec/tinyspec.tex)
- [`references/tinygrad/tinygrad/uop/spec.py`](references/tinygrad/tinygrad/uop/spec.py)
- [`references/tinygrad/tinygrad/schedule/memory.py`](references/tinygrad/tinygrad/schedule/memory.py)

### 4. The frontend leaks implementation details into semantics

Several choices will become painful as models get larger:

- Input names are leaked into `'static` storage.
- Compilation hash-conses the immutable `Rc` DAG and analyses use scoped
  pointer-keyed caches, but shape metadata is still recomputed across passes.
  Any longer-lived cache needs explicit graph ownership, not stable node IDs.
- Dynamic dimensions are narrowly supported; many shape paths require static extents.
- Most failures panic instead of returning structured diagnostics.

~~The current Llama example only constructs a graph. It does not load weights, compile, or run inference. On the review machine, cached graph construction took roughly 2.7 seconds in release and 24 seconds in debug. That strongly suggests repeated whole-DAG analysis.~~

*Stale as of 2026-08-01.* `examples/llama3_2.rs` now loads a real Meta
checkpoint, binds 146 tensors (2.47 GB) zero-copy, compiles 344 kernels, and
runs greedy inference at 62 tok/s bf16. Graph construction is **0.01 s**, not
2.7 s — the repeated whole-DAG analysis this paragraph inferred was real and
was fixed. Compilation is now the slow step at ~5.2 s, which is a different
problem from the one described here. The frontend concerns above it (leaked
`'static` input names, narrow dynamic-dimension support, panics instead of
structured diagnostics) still stand.

Relevant code:

- [`src/ir.rs`](src/ir.rs)
- [`examples/llama3_2.rs`](examples/llama3_2.rs)

### 5. The repository contract is not enforced continuously ~~(open)~~ — RESOLVED

`.github/workflows/ci.yml` (added 2026-07-20) now enforces the contract on every
push: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and
`cargo test --all-targets` on both the host and the Linux target. A
`.githooks` pre-commit hook runs the same checks at commit time. As of
2026-08-01 the suite is **273 tests across 30 suites**, all passing.

Two caveats the config cannot fix, both worth knowing before trusting a green
check: CI's macOS runners have **no GPU**, so the Metal path is never exercised
there — only local runs cover it. And `cargo test --release` alone silently
skips the `#[cfg(test)]` modules inside `examples/`; `--all-targets` is what
picks them up, which is why CI specifies it.

The original finding, for the record: as of July 20 the suite passed but no CI
configuration continuously enforced it.

## What Sanic does better than Tinygrad

The most valuable difference is semantic rather than ergonomic:

- Sanic represents axes by identity instead of relying primarily on positional dimensions.
- Its dependence and carrier analysis can reason about reductions structurally.
- The online softmax and attention result is derived from algebraic rules rather than matching a workload named "FlashAttention."
- The pure graph plus commit-after-success session model is a good foundation for stateful decoding.
- The closed high-level IR is easier to reason about than Tinygrad's large multiphase `UOp` dialect.

There are structural patterns in Sanic—contraction recognition, extremal-key
payloads, and masked maxima—so "no patterns" is too strong. The fair claim is
that Sanic uses reusable algebraic patterns rather than workload-specific
kernel templates.

Tinygrad's own test suite documents the current fusion boundary:

- [`references/tinygrad/test/backend/test_softmax_fusion.py`](references/tinygrad/test/backend/test_softmax_fusion.py)

## What to learn from Tinygrad

Copy its phase discipline, not its accumulated complexity.

The most useful ideas are:

1. Stable graph nodes with cached derived properties, like Tinygrad's hash-consed UOps.
2. Executable verification at every phase boundary.
3. A typed command/effect plan for ranges, loads, stores, barriers, dependencies, and hardware operations.
4. Semantic dtype, device, address-space, and symbolic-index information.
5. Lifetime-based buffer reuse.
6. Measured tuning of the exact schedule that will be emitted.
7. A large, continuously running backend test matrix.

Sanic should not copy Tinygrad's enormous single union dialect, pervasive heuristic matching, or global Python state. Keep one immutable positional tensor IR; attach typed analysis and scheduling data to it, and introduce a command/effect plan only where backend side effects actually begin.

## Recommended architecture

```text
semantic tensor graph
        |
        v
structural and numerical contract verification
        |
        v
carrier derivation and fusion partitioning
        |
        v
executable schedule/range plan
        |
        v
typed command/effect plan
        |
        v
memory planning and target rendering
```

The schedule plan must be the sole source of tiling and placement decisions consumed by every backend.

## Priority order

1. ~~Add CI that continuously runs the current full-suite and clippy contract.~~ Resolved: `.github/workflows/ci.yml`, plus a pre-commit hook running the same checks.
2. ~~Fix the top-k/tree-fold correctness hole and stop classifying unsupported affine values as executable monoids.~~ Resolved: both shortcuts were removed; Argmax fusion was recovered generically.
3. State the floating-point reordering contract explicitly.
4. ~~Unify `KernelSpec` and `FoldSched` into one schedule representation that Metal actually consumes.~~ Resolved by deletion: the unread physical fields were removed, leaving `fold_sched` the single owner of the physical schedule.
5. Keep node-relative occurrence metadata pass-local, with explicit ownership and scoped caches; do not add persistent node IDs.
6. Separate semantic axes, layout order, and hardware iteration ranges.
7. Add typed scalar, index, and product values plus a proper command/effect plan.
8. Add memory planning, and only then broaden backend coverage.

Sanic does not need a formal LaTeX specification at this stage. It needs small executable phase contracts and tests that make invalid states impossible to pass downstream.

## Bottom line

Sanic has a defensible research moat, but today that moat surrounds one excellent compiler idea rather than a complete framework. The best next move is to make the scheduler/code-generation contract honest and executable before adding more operators or models.
