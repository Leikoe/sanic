# Core ideas

These ideas define Sanic independently of its current code, IR, API, scheduler,
or backend.

## 1. The third homomorphism theorem

### The theoretical idea

The third homomorphism theorem says, informally:

> If a function on lists can be computed both from left to right and from right
> to left, then it is a list homomorphism.

If

```text
h = foldl(left_step, left_identity)
h = foldr(right_step, right_identity)
```

then, under the theorem's conditions, there exists an associative operation
`combine` on its results such that

```text
h(xs ++ ys) = combine(h(xs), h(ys))
```

Sanic uses this shape more generally when the desired visible result is not
itself homomorphic: it searches for a strengthened homomorphic carrier `s` and
a projection:

```text
h(xs) = project(s(xs))
s(xs ++ ys) = combine(s(xs), s(ys))
```

The important implication is that parallelism is a semantic property of the
computation. A list homomorphism is independent of a particular traversal
order or partition:

```text
                 xs
              /      \
             x₁      x₂
           /   \    /   \
          …     …  …     …
           \   /    \   /
           s₁          s₂
              combine
                 |
              project
```

The input may be divided anywhere. Each part is summarized independently, and
the summaries may be joined in any order-preserving parenthesization permitted
by associativity.

This gives a path from a sequential description to a divide-and-conquer
program:

```text
left-to-right computation
              +
right-to-left computation
              ↓
homomorphic carrier exists
              ↓
parallel tree reduction exists
```

The theorem is an existence result. By itself it does not guarantee:

- that the carrier is small enough to be useful;
- that the carrier or its `combine` is easy to discover;
- that unordered merging is legal when `combine` is not commutative;
- that floating-point reassociation is numerically acceptable; or
- that a particular hardware schedule is profitable.

Those are the constructive and engineering problems Sanic takes on.

### What it means for GPU programming

The central consequence for ML workloads is **independent ownership of output
values or tiles**.

An ML reduction usually has output axes and a reduction axis:

```text
output[i, j] = reduce over k of contribution(i, j, k)
```

This structure belongs to each **subcomputation and axis**, not to an operator
name in isolation:

- A **free** axis has no cross-element dependence and becomes part of the
  output grid.
- A **monoidal** axis carries an associative dependence and can be streamed or
  tree-reduced.
- An **opaque** axis has data-dependent access whose structure is not known
  statically.
- A **sequential** axis carries a non-associative recurrence and cannot be
  reassociated.

The executable core currently represents only free, monoidal, and opaque
axes. Sequential remains a model-level category until a recurrence node can
carry an executable step body; a name-only scheduling label is not semantics.

The same semantic axis may be free at one subcomputation and monoidal at a
consumer. For example, the middle axis of `(X·Y)·Z` is an output axis of the
first matrix multiplication and the reduction axis of the second. Parallelism
must therefore be discovered per `(subcomputation, axis)`, rather than assigned
once to an operation or tensor.

Give each output value or tile to one GPU thread or threadgroup. That owner
walks the whole reduction axis, updating only its bounded carrier:

```text
threadgroup owns output tile T

state[T] = identity
for each input tile K along the reduction axis:
    state[T] = combine(state[T], lift(T, K))
output[T] = project(state[T])
```

Different owners are parallel over the output grid. Each owner completes its
output locally, so threadgroups do not exchange partial outputs or synchronize
with one another during the computation:

```text
output tile 0  ← block 0 folds the entire reduction
output tile 1  ← block 1 folds the entire reduction
output tile 2  ← block 2 folds the entire reduction
       ...

no block-to-block reduction traffic
```

The homomorphic carrier is what makes the local fold sufficient. Its size does
not grow with the reduction axis, so it can remain in registers or threadgroup
memory. Only the final output tile needs to be written to HBM.

This is the core idea of FlashAttention. A threadgroup owns a query/output tile,
streams successive key and value tiles, and maintains the online-softmax
carrier `(m, ℓ, o)` on-chip. It never materializes the complete `QKᵀ` score
matrix or softmax probability matrix in HBM:

```text
naive attention
    QKᵀ → HBM → softmax → HBM → ·V → output

homomorphic attention
    stream K/V through on-chip (m, ℓ, o) → output
```

The important parallelism is therefore two-level:

1. **Across outputs:** independent threadgroups own different output tiles.
   This is communication-free grid parallelism.
2. **Inside one owner:** lanes may cooperate on the tile and merge carrier
   partials through SIMD operations or threadgroup memory.

Associativity permits arbitrary order-preserving tiling and merging inside the
owner without changing the mathematical result. It also permits splitting one
reduction across multiple threadgroups when the output grid is too small, but
that is an optional scheduling tradeoff: it deliberately reintroduces a small
partial-carrier write and combine pass. It is not required by the algebra, and
it is not the path that removes FlashAttention's HBM intermediates.

The central compiler problem is therefore not “which known GPU kernel matches
this graph?” It is:

> Can we construct a bounded homomorphic carrier so that one GPU owner can
> complete each output tile locally, without materializing the reduction's
> intermediate values in HBM?

## 2. Completeness through future equivalence

### The problem

Soundness asks:

> When the deriver produces a carrier, does it compute the original function?

That can be checked by executing the carrier against a reference evaluator.
But soundness alone permits a useless deriver that declines every difficult
program.

Completeness asks the opposite question:

> When the deriver declines a program, was there nevertheless a small carrier
> it failed to find?

This is the idea behind [`tests/completeness.rs`](tests/completeness.rs).

### Prefixes are equivalent when no future can distinguish them

For a function `h`, associate each prefix `x` with its behavior under every
possible suffix:

```text
future(x) = y ↦ h(x ++ y)
```

Two prefixes are equivalent when every continuation produces the same answer:

```text
x ≡ₕ x'    iff    for every y, h(x ++ y) = h(x' ++ y)
```

This is a Myhill–Nerode-style view of streaming state. A correct state summary
must retain exactly the distinctions between prefixes that some future can
observe.

A sketch `σ` is sufficient when

```text
σ(x) = σ(x')    implies    x ≡ₕ x'
```

In words: if two prefixes have the same state, no suffix may later reveal which
prefix was seen.

This explains accumulator strengthening. If `h(x)` alone merges prefixes that a
future can distinguish, the visible result is not enough state. Add another
component to the sketch until those prefixes are separated:

```text
visible result
      ↓ insufficient
(visible result, auxiliary fold₁)
      ↓ perhaps still insufficient
(visible result, auxiliary fold₁, auxiliary fold₂, …)
      ↓
future-determining carrier candidate
```

This is the classical tupling idea viewed as a search for sufficient streaming
state.

### What the completeness probe does

The test turns that semantic idea into a bounded experiment:

1. Generate many prefixes and suffixes over a small quantized alphabet.
2. Build candidate sketches `σ = (h, auxiliary folds...)` from tuples of known
   bounded-state folds such as length, sum, maximum, argmax, or second maximum.
3. Group distinct prefixes whose sketches collide.
4. Append the sampled suffixes to both prefixes.
5. If one suffix produces different results, it is a concrete separating
   witness: the sketch forgot necessary state.
6. If many sketch collisions survive every sampled suffix, the tuple is a
   constructive carrier candidate. Its components suggest the slots the
   deriver is missing.

This changes a derivation decline from an unquestioned answer into a checkable
claim. A declined program with a strong bounded-state candidate is a possible
fusion miss and must be explained, supported through another representation,
or used to extend the algebra.

The random-program sweep applies the same pressure beyond a hand-written
syllabus. It is intended to discover missing derivation rules before a model
benchmark happens to expose them.

### What the probe proves—and what it does not

The probe is a detector, not a proof of completeness.

A separating suffix proves that one candidate sketch is insufficient. It does
not prove that no other bounded carrier exists.

A sketch that survives sampled suffixes is evidence for a carrier, not yet a
certificate. The next steps are still required:

1. Define how one element is lifted into the state.
2. Construct a general combine for two states.
3. Check identity and associativity.
4. Prove or test equivalence with the original function.
5. State the numerical and ordering contract.

The probe is deliberately bounded by its alphabet, candidate pool, tuple size,
and sample budget. Those bounds should remain visible.

### What it means for GPU programming

The future-equivalence view tells us what must fit in the per-chunk state.

- If a small composable sketch determines every future, one GPU owner can use
  it as register- or threadgroup-resident state while completing an output
  tile locally.
- If increasingly many distinctions are required as the input grows, an exact
  constant-state fused reduction is unlikely; the computation must materialize
  more information, use multiple passes, or accept a restricted domain.
- The number and shape of sketch components also expose the physical price of
  the carrier: register pressure, merge work, and threadgroup storage.

Thus completeness is not merely about finding more fusions. It connects the
semantic question “what information can the future observe?” to the GPU
question “what state must every parallel worker carry?”

## 3. Streamability belongs to the composite

### A non-streamable intermediate may have a streamable consumer

Whether a computation streams depends on which result is actually demanded.
It is not, in general, a permanent property of each intermediate operator.

Consider softmax:

```text
m = maxᵢ xᵢ
ℓ = Σᵢ exp(xᵢ - m)
wᵢ = exp(xᵢ - m) / ℓ
```

Producing the complete vector of weights `w` requires information from the
whole row. A bounded-state worker cannot emit the early weights before it
knows `ℓ`; without storing the row or revisiting it, softmax as an observable
vector is not a one-pass bounded-state result.

Attention asks for a different observable:

```text
o = Σᵢ wᵢvᵢ
```

Because the consuming sum is linear, the global normalizer factors out:

```text
o = (Σᵢ exp(xᵢ - m)vᵢ) / ℓ
```

The numerator may remain unnormalized until the final projection. This changes
the required state from an entire vector into the bounded carrier `(m, ℓ, o)`.
For two independently computed chunks:

```text
M  = max(m₁, m₂)
ℓ  = ℓ₁ exp(m₁ - M) + ℓ₂ exp(m₂ - M)
o  = o₁ exp(m₁ - M) + o₂ exp(m₂ - M)

combine((m₁, ℓ₁, o₁), (m₂, ℓ₂, o₂)) = (M, ℓ, o)
project(m, ℓ, o) = o / ℓ
```

Two distinct algebraic facts are at work:

1. **Associativity** makes the strengthened carrier streamable and
   parallelizable.
2. **Distributivity through a linear consumer** moves the normalizer out of
   the reduction and makes that carrier possible for the composite result.

The second fact is not a consequence of the third homomorphism theorem. The
theorem licenses reassociation once the homomorphic carrier is known;
distributivity changes the specification boundary at which a useful carrier
can be found.

### What it means for compilers and GPUs

The unit of analysis must be the demanded composite expression, not a list of
named operators:

```text
softmax weights demanded
    → retain or revisit row-sized information

softmax consumed only by a linear reduction
    → defer normalization into project
    → carry (m, ℓ, o)
    → no score or probability matrix in HBM
```

This principle extends beyond attention. A normalization consumed by a linear
projection may be carried as an unnormalized numerator plus the normalization
statistics. Several linear consumers may share the same statistics by adding
more numerator slots to the product carrier.

The compiler question is therefore:

> What is the outermost observable result through which the required state can
> still compose associatively?

Stopping at an intermediate tensor too early can manufacture a materialization
boundary that the mathematics does not require.

## How the three ideas fit together

The third homomorphism theorem provides the existence bridge:

```text
left fold + right fold
          ↓
homomorphic divide-and-conquer form exists
```

Future equivalence provides a way to search for and audit the missing state:

```text
derivation declined
          ↓
search for a bounded sketch that determines every future
          ↓
carrier candidate or separating witness
```

Contextual composition determines the right result boundary:

```text
non-streamable intermediate
          +
consumer with a compatible law
          ↓
streamable composite with strengthened state
```

Together the three ideas define the enduring Sanic loop:

```text
ordinary sequential computation
          ↓
choose the complete observable expression
          ↓
discover sufficient state
          ↓
construct and validate an associative combine
          ↓
challenge both successful derivations and declines
          ↓
give each output tile an owner and keep its carrier on-chip
```

The carrier should be a constructive, executable witness, not merely a
“fusable” label. It contains:

```text
lift      one streamed item into state
combine   merge two independently computed states
identity  represent the empty input
project   recover the demanded result
```

Successful composition of this witness certifies a legal streaming kernel.
Where its construction stops, some value must be supplied from outside.
Therefore the **derivation frontier is the fusion boundary**: everything
swallowed by one carrier may remain within one streaming kernel, while its
leaves are the principled places at which materialization may occur.

The implementation may change completely. This loop is the core.

## Consequences and engineering disciplines

The following principles matter, but they are consequences of the core ideas
or disciplines for preserving them. They should not be confused with the
theorems themselves.

1. **Prefer a small, law-carrying basis to a catalogue of workload names.**
   Compound operations such as attention, normalization, convolution, and
   activation functions should be expressible as compositions whenever
   possible. The compiler can then apply a small collection of laws by
   structural recursion and generalize to composites that no kernel template
   anticipated.

2. **Represent movement as indexing until a real boundary requires storage.**
   Renames, slices, padding, windows, flattening, and similar views describe
   where values are read; they do not intrinsically compute or copy a tensor.
   Preserving them as index transformations lets convolution become
   `window + flatten + matmul` without an `im2col` buffer, and lets windowed
   reads participate in the same carrier derivation as dense reads.

3. **Keep semantic dependence separate from physical scheduling.** Semantic
   axes say which values vary together and which reductions are legal.
   Thread grids, tiles, lanes, SIMD groups, split reductions, and memory
   placement are later physical choices. A schedule may choose among
   reassociations licensed by the carrier; it must not silently invent a new
   algebraic permission.

4. **Separate legality from profitability.** Carrier construction and
   verification decide what transformations preserve the computation. A
   device model then ranks only legal choices by HBM traffic, register and
   SRAM pressure, occupancy, recomputation, and launch cost. An inaccurate
   cost model may select a slow program; it must not be able to select an
   incorrect one.

5. **Keep the mathematical graph pure and introduce effects at an explicit
   boundary.** Mutation, persistent caches, buffer reuse, and device commands
   should not complicate the proof of a carrier. Stateful execution can run a
   pure computation first and commit its resulting buffers afterward, making
   read-before-write ordering structural rather than implicit.

6. **Make derived programs pass through the same pipeline.** Gradients,
   simplifications, decompositions, and other transformations should produce
   ordinary semantic graphs when possible. They then inherit the same
   analysis, carrier derivation, partitioning, scheduling, and correctness
   checks as forward computations instead of creating parallel compiler
   stacks.

7. **Validate every semantic boundary with an independent executable oracle.**
   Associativity only shows that a carrier agrees with its own
   parenthesizations; it does not show that it computes the intended tensor
   expression. The useful ladder is:

   ```text
   carrier tree fold = carrier sequential fold
   carrier execution = naive graph evaluation
   partitioned schedule = naive graph evaluation
   generated target program = naive graph evaluation
   ```

   Each equality catches a different class of error and should remain
   independently testable.

8. **State declines and numerical contracts explicitly.** Data-dependent
   access, non-associative recurrences, order-sensitive tie behavior, and
   floating-point reassociation do not all obey the same monoid contract.
   Unsupported cases should remain visible declines. Supported cases should
   say whether they require order preservation, commutativity, exact equality,
   or an accepted numerical tolerance.

## Reference

- Jeremy Gibbons, “The Third Homomorphism Theorem,” *Journal of Functional
  Programming* 6(4), 1996:
  [paper](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/8DB1295DC8EB3C4942FB635C439B016E/S0956796800001908a.pdf/functional-pearls-the-third-homomorphism-theorem.pdf)
