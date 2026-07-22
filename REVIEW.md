# Framework Review

Reviewed against the vendored Tinygrad source and specification on July 18,
2026; repository-contract findings refreshed after the July 20 core audit.

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
| Correctness evidence | Good focused and full-suite coverage; no CI enforcement |
| Documentation | Overstates current capabilities |

## Most serious problems

### 1. The planner does not describe the kernel actually emitted

`KernelSpec` claims to be a fully resolved physical plan and contains tile sizes, row and column axes, batch roles, and resource estimates. The Metal emitter largely ignores those fields and independently chooses a `FoldSched`.

Consequences:

- The cost model can score tile choices that never reach generated code.
- Reported SRAM and register estimates do not necessarily describe the emitted kernel.
- Autotuning and split scheduling exist in pieces but are not wired into the ordinary execution path.

This is the highest-priority architectural issue. A schedule should be executable, not advisory.

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

The current Llama example only constructs a graph. It does not load weights, compile, or run inference. On the review machine, cached graph construction took roughly 2.7 seconds in release and 24 seconds in debug. That strongly suggests repeated whole-DAG analysis.

Relevant code:

- [`src/ir.rs`](src/ir.rs)
- [`examples/llama3_2.rs`](examples/llama3_2.rs)

### 5. The repository contract is not enforced continuously

The stale removed-API call sites have been fixed. On July 20, `cargo test`
passed 157 tests plus the doctest (one diagnostic-only test ignored), and
`cargo clippy --all-targets -- -D warnings` passed. The remaining process gap
is that no CI configuration continuously enforces that contract.

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

1. Add CI that continuously runs the current full-suite and clippy contract.
2. ~~Fix the top-k/tree-fold correctness hole and stop classifying unsupported affine values as executable monoids.~~ Resolved: both shortcuts were removed; Argmax fusion was recovered generically.
3. State the floating-point reordering contract explicitly.
4. Unify `KernelSpec` and `FoldSched` into one schedule representation that Metal actually consumes.
5. Keep node-relative occurrence metadata pass-local, with explicit ownership and scoped caches; do not add persistent node IDs.
6. Separate semantic axes, layout order, and hardware iteration ranges.
7. Add typed scalar, index, and product values plus a proper command/effect plan.
8. Add memory planning, and only then broaden backend coverage.

Sanic does not need a formal LaTeX specification at this stage. It needs small executable phase contracts and tests that make invalid states impossible to pass downstream.

## Bottom line

Sanic has a defensible research moat, but today that moat surrounds one excellent compiler idea rather than a complete framework. The best next move is to make the scheduler/code-generation contract honest and executable before adding more operators or models.
