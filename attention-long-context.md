# Issue: attention does not scale with context length

A llama-3.2-1B bf16 decode step costs 15.4 ms of GPU time at 32 generated
tokens and 20.2 ms at 1024. Weight streaming — the mlp and projection classes,
which are the bulk of the step — is flat across that range. **All 4.8 ms of the
regression is attention over the KV cache**, and none of it is fundamental:
attention's whole working set is 33.75 MB, which is 0.19 ms at this machine's
streaming rate.

Three independent defects, in descending order of cost. They can be fixed
separately and verified separately.

Self-contained; nothing below assumes context from the session that found it.

**Status (2026-07-28).** **A and B are fixed**, both by layout rather than by
the transforms proposed below, and both worth less than predicted — see
[What a fix is worth](#what-a-fix-is-worth) before trusting the number for C.
Measured warm at ctx 1030: **19.18 → 17.08 ms/tok**, of which B is 0.24 and A is
1.84. C is untouched.

The order mattered. Fixing B is what made A's fix free, and this document said A
should not be fixed that way — see the transpose trap, which was correct when
written and is now wrong.

**The gap at 1024 is now 4.9%, down from 14.6% — and none of what is left is on
the GPU.** sanic's GPU replay is 17.0 ms against mlx-lm's 18.39 ms of wall
clock; the deficit is 2.6 ms of host overhead. See
[the re-measurement](#re-measured-after-a-and-b-2026-07-28-one-warm-session).
That, not C, is where the next millisecond is.

## The measurement

sanic vs mlx-lm, both bf16 Llama-3.2-1B, batch 1, prefill excluded, greedy.
ABBA-counterbalanced (`sanic, mlx, mlx, sanic` per length) because this machine
penalises whichever variant runs first. Wall-clock ms/tok, mean of the two
slots:

| generated tokens | sanic | mlx-lm | |
|---|---|---|---|
| 32 | **16.45** | 17.25 | sanic ahead 4.9% |
| 128 | 18.05 | **17.46** | mlx ahead 3.3% |
| 512 | 20.30 | **18.09** | mlx ahead 12.2% |
| 1024 | 22.65 | **19.76** | mlx ahead 14.6% |

Run-to-run spread within a pair is 1–3% (worst, sanic at 512: 5.8%), so the
1024 gap is well outside noise. The crossover is between 32 and 128 tokens.

The shape is the finding, not the endpoint. Going 32 → 1024 **mlx-lm loses
2.51 ms/tok; sanic loses 6.20** — sanic degrades ~2.5× faster. Of sanic's 6.20,
4.8 is GPU replay and the remaining ~1.4 is host-side overhead outside replay.

### Re-measured after A and B (2026-07-28, one warm session)

| generated tokens | sanic | mlx-lm | |
|---|---|---|---|
| 32 | **15.78** | 16.49 | sanic ahead 4.3% |
| 128 | **16.80** | 17.48 | sanic ahead 3.9% |
| 512 | **18.39** | 18.42 | tie |
| 1024 | 19.29 | **18.39** | mlx ahead 4.9% |

1024 is n=8 per engine; the others are one ABBA block each. **The gap at 1024
went from 14.6% to 4.9%, and the crossover moved from between 32–128 out to
between 512–1024.** Slope 32 → 1024: sanic 3.43, mlx 1.90 — sanic still
degrades faster, but 1.8× rather than 2.5×.

Both columns moved, so do not read these against the table above: that one
predates the warm-up discipline (see Traps). Only the within-session comparison
means anything.

### The remaining gap is not on the GPU

sanic's GPU replay at 1024 is **17.0 ms, on every one of the eight runs.** That
is 1.4 ms *faster than mlx-lm's entire wall time*. Everything sanic is behind by
is host-side. Measured, one instrumented run at ctx 1030:

| per token | ms |
|---|---|
| GPU replay | 17.0 |
| **`step()` wall − GPU** | **1.908** |
| CPU argmax over 128,256 logits | 0.569 |
| read + widen the logits buffer | 0.040 |
| everything else (tokenizer, buffer writes) | 0.062 |

That sums to the wall clock, so nothing is hiding. **Host overhead is now the
largest single item left — 2.6 ms/tok, against ~0.4–0.7 for defect C and ≤0.4
for B's grid.** Two things to look at, in order:

- **The 1.9 ms around the replay**, now split by measurement: **0.73 ms** is the
  CPU encoding 342 dispatches (2.1 µs each), and **~1.0 ms** is commit → GPU
  start → completion notification. The encode is pure waste: the dispatch list
  and its bindings are IDENTICAL every step — the token and position travel
  through buffer *contents*, not the command stream — so nothing about the
  encoding depends on what the model produced. It can be built once, or built
  for step N+1 while step N runs. Neither is done today; `run_graph_timed`
  encodes, commits and blocks, in that order.
- ~~**The 0.569 ms argmax.**~~ **Done** (`04eb8a9`) — sampling runs on the GPU.

**Both measured, together, once the machine was inside its window**: a balanced
ABBA at ctx 1030 against the pre-sampling build, whose two baseline runs agreed
to 0.17 ms — so the sweep sat in the plateau rather than the drift.

| | ms/tok |
|---|---|
| before GPU sampling and pipelined encoding | **19.27** |
| after both | **18.32** |
| | **−0.95** |

Predicted 1.30 (0.569 argmax + 0.73 encode); collected 0.95, about 73% — the two
savings partially overlap, which is the same pattern as every other estimate in
this document coming in high. The baseline agrees with the 19.29 measured for
the mlx-lm comparison, so these are the same regime and directly comparable:
**sanic 18.32 against mlx-lm's 18.39 at 1024 — level, from 14.6% behind.**

A second cycle corroborated the direction (18.06 against 19.63/19.56) but its
last run jumped to 23.77, which is throttle onset, and is discarded under the
rule two bullets down: an upward-drifting run is heat, not signal.

### Why argmax does not fuse into the lm_head, and why that is not a one-line fix

Worth writing down, because the obvious fix is a trap. `argmax` reads the logits
twice — `ΣMax/vocab`, then `ΣMin/vocab` over `where(x < max, +∞, coord)` — so the
vocabulary is materialized between two folds. Nothing DECLINED over `vocab`; the
folds simply never couple.

They never couple because coupling is discovered in exactly one place, and it is
narrower than it looks. It is not keyed on `exp` — it is keyed on SUBTRACTION:

```rust
(Bin::Sub, S::Pe { .. }, S::Coll(Expr::F(i)))
    if matches!(ctx.slots[i].kind, SlotKind::Plain(Monoid::Max)) => S::PeOff { .. }
```

`x − max` is the online-softmax shift, and `exp` is its only consumer;
everything else declines. Argmax couples through a COMPARISON (`x < max`), so
the rule cannot see it.

**Do not "fix" this by matching `Where(Lt(x, max), +∞, coord)` reduced by `Min`.**
That is a pattern match on argmax's spelling — an operation-specific rule of
exactly the kind this repo refuses, one layer below where they are usually
caught.

What is defensible is a FAMILY: one algebraic identity per scalar primitive that
can ride a running max, each stating how a partial accumulation repairs itself
when the max advances.

| consumer of a max-coupled value | repair when the max advances |
|---|---|
| `exp` | multiply by `exp(m_old − m_new)` — exp's homomorphism |
| `lt` | discard the accumulated index and take the new one |

That is a second instance of the same principle, not a generalisation of it, and
it should be argued for on those terms rather than smuggled in. Note also that
`(max, index)` needs LESS machinery than softmax: it is already an associative
pair monoid — larger value wins, smaller index breaks ties — with no repair step
at all. Softmax's pair merge contains `exp` only because its formula does.

Fully general fusion of dependent reductions would mean synthesizing the repair
operator, which is a different and much larger problem than adding a rule.

Fix both and sanic is ~17 ms against mlx's 18.4 at 1024 — ahead at every length
measured, which the GPU work already earns today.

Reproduce:

```
./target/release/examples/llama3_2 "The capital of France is" -n 1024 --bf16
.venv/bin/python weights/mlx_llama_bench.py 1024
```

## Where the 4.8 ms goes

One decode step under `SANIC_DEBUG=4`, per kernel class, in µs. This regime
gives every kernel its own encoder and inflates the total (21.1 ms reported vs
15.4 ms in production, a consistent ×0.735 across both lengths), so read the
**deltas** — this table says where the time is, and it is right about that.

It is NOT right about what removing a class is worth. ×0.735 is the ratio of
*totals*; applied to a *delta* it over-predicted B by 3× (see
[What a fix is worth](#what-a-fix-is-worth)). Use this table to aim, and the
wall clock to score.

| class | ctx 38 | ctx 1030 | Δ |
|---|---|---|---|
| attention output fold (over V) | 254 | 4952 | **+4698** |
| attention scores fold (q·k) | 190 | 2105 | **+1915** |
| rope + cache write | 346 | 1403 | **+1057** |
| all weight-streaming classes | 18 800 | 17 415 | −1385 |
| **total** | **21 054** | **27 498** | **+6444** |

The weight classes do not move with context; their drift is clock and noise.

Cache geometry throughout: `[kv_heads=8, cache_sequence=N, head_dim=64]`, bf16,
contiguous in `head_dim`. One `(head, position)` row is 64 × 2 = **128 bytes,
exactly one cache line**. Per layer, K is `8 × 1030 × 64 × 2` = 1.055 MB, and V
the same; K+V across 16 layers is 33.75 MB.

### A. The output fold read V with a 128-byte stride — FIXED, worth 1.84 ms

```
fold_query_heads32_singleton1_head_dim64_over_cache_sequence1030
grid=(2048,1,1) block=(32,1,1)   bw=1%
```

`out[qh, d] = Σ_s P[qh, s] · V[kv_head(qh), s, d]`. The grid is 32 query heads
× 64 head-dim, so a threadgroup owns one `(qh, d)` and folds over `s`. Stepping
`s` at fixed `d` advances the address by `head_dim × 2` = 128 bytes: **every
iteration lands on a different cache line and uses 2 of its 128 bytes.**

Measured 309 µs per layer against a 1.055 MB working set — **3.4 GB/s, 53× off
the 179.7 GB/s roofline**. That ratio is close to the 64 elements per line you
would waste with no cross-threadgroup line reuse at all.

The 64 threadgroups sharing a `qh` differ only in `d` and could in principle
reuse each line 64×. Measurement says they largely do not. Whether that is a
residency/scheduling artefact or something else is **not established** — the
stride is the structural cause, the reuse behaviour is worth confirming before
choosing between the fixes below.

**FIXED in `3e52313`, worth 1.84 ms — by layout, not by either fix proposed
below.** The stride is not inherent to the fold; it is where V sits. K is
contracted over `head_dim` by the scores fold and wants `head_dim` innermost;
V is contracted over `cache_sequence` by the output fold and wants
`cache_sequence` innermost. They are different tensors and may have different
layouts. V is now stored `[kv_heads, head_dim, cache_sequence]`, and the load
`V[kv*65920 + s*64 + d]` becomes `V[kv*65920 + d*1030 + s]` — contiguous in the
folded axis, so a 32-lane load touches one line instead of 32.

The transpose **fuses into the fold** — same 342 kernels, nothing materialized —
because a view only changes the offsets codegen computes. The whole change is
the model's cache declaration plus `update_cache` taking the sequence dimension
instead of assuming dim 1.

| | base | V transposed | Δ |
|---|---|---|---|
| output fold class, `SANIC_DEBUG=4`, ctx 1030 | 4454 µs | 801 µs | **5.6×** |
| scores fold class (K untouched) | 1920 µs | 1891 µs | flat |
| cache write, both tensors | 750 µs | 823 µs | +73 µs |
| **production wall, ctx 1030** (ABBA×4, n=8) | **18.91** | **17.08** | **−1.84 ms** |

Pooled sd 0.07, ranges disjoint (18.8–19.0 vs 17.0–17.2).

**Why this was the only lever.** sanic does not generate tiled algorithms. The
emitter is one thread per output point with the contracted axis folded in
registers; `FoldSched` schedules the *reduction* (across simdgroups, across
lanes, in chunks) and threadgroup memory is used only to merge partial
reduction results — never to stage operands for reuse. `plan.rs` prices tiles,
but no emitter reads a tile field. So there is no blocking transform to reach
for: with no tiling and thread count effectively pinned by the 0.61× trap
below, the only thing left to change is where the data sits.

### B. The whole cache was rewritten every step — FIXED, worth 0.24 ms

```
map_kv_heads8_cache_sequence1030_head_dim64
grid=(515,1,1) block=(1024,1,1)   bw=9%
```

515 × 1024 = 527,360 threads = `8 × 1030 × 64` — the entire cache, mapped every
step, to change **one** position. `update_cache` is functional: it produces a
new full cache tensor, and the emitted map covered the full extent rather than
the single live slot.

32 dispatches (16 layers × k and v) × 2.11 MB read+write = **67.5 MB per step**
to update 64 values per layer. The useful write is 32 × 1024 B = 32 KB. That is
a factor of ~2100.

**Fixed in `bc37a49`**, as a general law and not a cache-specific case:
`where(c, x, s)` differs from `s` only where `c` is nonzero, so when a state's
successor has that shape and `s` is the state's own input, the successor IS the
old state with a part replaced — and it can be written into `s`'s own buffer,
because the points it does not store already hold the right values. The kernel
evaluates `c`, retires the threads that fail it, and never loads `s` at all;
`capture` binds the pair as one buffer instead of two that ping-pong. The one
precondition — that nobody still wants the old contents — is settled by
counting: the step reads the state's values exactly once, in the fallback arm
the law erases. (A `Coordinate`'s source does not count, being a function of
shape, not contents.)

The law is stated in `Graph::overwritable_states` (`src/graph.rs`), carried to
emission by `compile_roots_in_place` (`src/compile.rs`) through
`Schedule::agrees_in_place`, and acted on in `emit_pointwise_metal_on`
(`src/emit_metal.rs`). `a_row_write_state_keeps_what_earlier_steps_wrote`
(`tests/graph.rs`) is the guard: nothing copies earlier rows forward any more,
so if the alias were wrong it would read back zeros.

What it was worth, measured both ways:

| regime | base | in place | Δ |
|---|---|---|---|
| per-kernel class, `SANIC_DEBUG=4`, ctx 1030 | 2402 µs | 1630 µs | −772 µs |
| **production wall, ctx 1030** (ABBA×4, n=8 each) | **19.18** | **18.94** | **−0.24 ms** |
| production wall, ctx 38 | 15.28 | 15.30 | none, as predicted |

Pooled sd 0.06 at ctx 1030, ranges non-overlapping (base 19.1–19.3, in place
18.9–19.0), same sign in all four cycles. Nothing else in the per-kernel
breakdown moved by more than 1%.

**What remains of B.** Only the traffic is gone; the grid is untouched. It still
covers the full extent — 515 threadgroups, 527,360 threads, to write 512 values.
Closing that needs the condition turned into a *domain restriction*: `c` pins
`cache_sequence` to one runtime index, so the kernel could iterate a single
point at a runtime offset while keeping the full-extent stride for the store.
That is a real analysis — affine bounds on a coordinate, plus a way to relate a
nested `Coordinate`'s axis occurrence back to a grid axis, which the emitter
cannot do by `AxisRef` equality because each `Map` level remaps them (the
rendered loop-variable name is the identity that actually survives). Worth
somewhere under ~0.4 ms by the ratio below — do A first.

### C. The scores fold is latency-bound — +1.9 ms

```
fold_query_heads32_singleton1_cache_sequence1030_over_head_dim64
grid=(1030,1,1) block=(64,1,1)   bw=3%
```

Reads are contiguous here — one threadgroup per cache position, 64 threads over
`head_dim`, one 128-byte line each. The problem is volume in flight: each
threadgroup touches 128 bytes and then reduces. 132 µs per layer for 1.055 MB
is **8.0 GB/s, 22× off roofline**.

Sustained bandwidth on this machine is outstanding bytes over latency (see
`applegpu/bandwidth.md`: ~23 outstanding 128 B lines per core, 219 ns
load-to-use). 1030 threadgroups × 128 B is not enough to fill that.

## What a fix is worth

The original estimates scaled the `SANIC_DEBUG=4` deltas to production by
×0.735:

| fix | predicted at 1024 | measured | debug Δ → wall Δ |
|---|---|---|---|
| A | ~3.45 ms | **1.84 ms** | −3653 µs → −1.84 ms (×0.50) |
| B | ~0.78 ms | **0.24 ms** | −772 µs → −0.24 ms (×0.31) |
| C | ~1.41 ms | — | — |

**Both tested predictions were high, by 1.9× and 3.2×.** The debug→production
ratio is not the ×0.735 assumed here; it came out ×0.50 for A and ×0.31 for B,
so it is not even a constant — it depends on how well the class overlaps other
work.

Why: in production every kernel shares one concurrent encoder, so these
dispatches overlap other work. Deleting their traffic does not delete their
time. `SANIC_DEBUG=4` gives each kernel its own encoder, which is exactly what
makes a per-kernel delta look like a step delta when it is not.

So **treat C's 1.41 ms as an upper bound** and expect ~0.4–0.7 ms. With A and B
done, C plus B's remaining grid is all that is left of the three, and together
they are worth about 1 ms against the 2.08 already collected.

The roofline argument is unchanged and is the real target: attention's working
set is 33.75 MB, which is 0.19 ms of streaming. Nothing about the *ceiling*
moved — only the estimate of how much of it each fix collects.

## Traps

Each of these has already cost real time in this repo; none are hypothetical.

- **Do not fix a strided fold by giving threadgroups more rows.** The obvious
  "one threadgroup handles several `d`, amortise the loads" transform reduces
  thread count, and `applegpu/msl.md` records that exact shape measuring
  **0.61×** on a matvec — 8× fewer threads is 8× fewer loads in flight. Still
  true, and it is why A was fixed by layout: with thread count pinned and no
  tiling in the compiler, *where the data sits* is the only lever left. (This
  entry used to propose staging a V tile in threadgroup memory, flash-style.
  That would have meant building a tiling capability sanic does not have — see
  "Why this was the only lever" under A.)
- ~~**Transposing the cache moves the problem, it does not solve it.**~~
  **This trap was true when written and is now wrong — and how it stopped being
  true is the lesson.** It said transposing makes A's fold contiguous but B's
  write strided. That was a fair trade only while the write mapped the whole
  extent. Once B was fixed the write became 512 threads, and V's scatter over 64
  lines costs 391 µs against K's contiguous 432 — no penalty at all. **Fixing B
  is what made A's fix free.** Transpose K and V independently; each wants the
  axis its own fold contracts stored innermost. The trap's real advice — "decide
  with a measurement, not on paper" — is what caught this.
- **Measurement regimes do not transfer.** `SANIC_DEBUG=4` understates
  production by ~27% here. A standalone attention probe over one small cache
  measures the SLC, not DRAM. Trust wall-clock A/B on the real example. Now
  demonstrated, not just asserted: a per-kernel delta of −772 µs bought −0.24 ms
  of step. **Never quote a `SANIC_DEBUG=4` delta as a result.**
- **Always-baseline-first A/B is biased.** Whichever variant runs first loses on
  this machine; an ordered A/B once produced a spurious +3.4% that vanished
  under ABBA. Report pooled sd and position-matched pairs.
- **There is a measurement WINDOW, and both edges bite.** From cold the machine
  is slow and gets faster: an 8-run ABBA at 1024 with 75 s gaps drifted
  **28.0 → 19.1 ms/tok monotonically**, the same binary, fastest at the end — a
  "cooled" sweep measures the warm-up, not the change. After ~10 minutes of
  continuous load it plateaus, back-to-back runs hold ±0.1 ms, and a 0.24 ms
  effect is resolvable. **After some HOURS of sustained GPU load it throttles the
  other way**: a later sweep on a quiet machine ran **18.65 → 41.78** across ten
  back-to-back runs, degrading monotonically. So: warm up, measure inside the
  plateau, and treat a sweep that drifts UPWARD as heat, not signal. If a run
  takes 2× the first run of the day, stop and let it cool.
- **Check `ps` properly before blaming the machine.** A `cargo test` child is
  named after its test file — `completeness-<hash>`, `bandwidth_probe-<hash>` —
  so `ps | grep cargo` finds NOTHING while a GPU probe saturates the device.
  `completeness` runs 601 s and `bandwidth_probe` 273 s, both on the GPU. This
  cost an hour of "the machine is contended by something outside this session",
  written into a commit message before the grep was checked. Sort by CPU
  (`ps -Ao pid,%cpu,comm | sort -k2 -rn`) rather than grepping for names you
  expect.
- **Absolute numbers here are not reproducible across sessions.** The
  `SANIC_DEBUG=4` step total was 27.5 ms the day this was written and 43.8 ms
  when re-measured, on the same machine and commit. Only matched pairs taken
  within one session mean anything; the tables above are shape, not ground truth.
- **Check the f32 path.** It is the default and no test covers it; a change can
  pass every test and still leave only `--bf16` working. Diff the text against a
  baseline binary — do not just check that it runs and looks like English.
- **`cargo test --release` is not what CI runs.** CI runs `cargo test
  --all-targets`, which additionally builds the `#[cfg(test)]` modules inside
  `examples/` — where the llama decode graph's shapes are pinned. A layout
  change turns those assertions red while a full green `--release` run says
  nothing, because it never compiles them.

## How to verify a fix

1. **The acceptance test is flatness.** sanic's ms/tok at 1024 within a few
   percent of its ms/tok at 32. A fix that improves the 1024 number but leaves
   the *slope* intact has not solved this. Slope over 32 → 1024, from matched
   ABBA sets in one warm session:

   | | ctx 38 | ctx 1030 | slope |
   |---|---|---|---|
   | before | 15.28 | 19.18 | +3.90 |
   | B fixed | 15.20 | 18.91 | +3.71 |
   | **A + B fixed** | **15.20** | **17.08** | **+1.88** |

   Halved, and neither fix costs anything at short context. Still a slope: C and
   B's remaining grid own the rest.
2. Per-defect, from `SANIC_DEBUG=4` at `-n 1024` — use these to confirm the
   *mechanism* changed, then go to the wall clock for what it is worth:
   - A: ~~output-fold `bw=1%` rises~~ **done** — the fold reads V contiguously
     in the folded axis; class 4454 → 801 µs.
   - B: ~~the cache-write kernel stops reading the cache~~ **done** — it now
     evaluates the condition and retires. The grid criterion is NOT met: it
     should be O(1) in context, and is still 515 threadgroups at 1030.
   - C: scores-fold `bw=3%` rises.
3. **`cargo test --all-targets`** — what CI runs, and not the same thing as
   `cargo test --release`. `--all-targets` builds the `#[cfg(test)]` modules
   inside `examples/`, which plain `cargo test` never compiles. This cost a red
   CI run: `llama3_2.rs` pins the cache root shapes, the V transpose changed one
   of them, and a green `cargo test --release` said nothing about it because the
   assertion was never built. Run both — release for speed on the GPU probes,
   `--all-targets` before pushing.
4. Both dtypes, text byte-identical.
   `./target/release/examples/llama3_2 "The capital of France is" -n 32 --bf16`
   should print `…is Paris. It is the most visited city in the world. It is
   also the most expensive city in the world.`, and the f32 default must still
   compile and run and print `…the country's capital and largest city`.

   **The bf16 text changed when sampling moved to the GPU, and the reason is
   worth keeping.** `ir::argmax` returns the FIRST maximum, the CPU scan it
   replaced used `Iterator::max_by`, which returns the LAST. bf16 carries 8
   mantissa bits, so exact ties are common — at step 6 of this prompt, tokens
   12263 (` visited`) and 95551 (` populous`) both score exactly 21.500000, and
   the two rules part company there. f32 is unchanged because ties are rare at
   24 bits, which is what shows the divergence is tie-breaking and not a bug.
   First-index is the convention numpy, torch and mlx all follow.
5. Re-run the ABBA sweep against mlx-lm at 32/128/512/1024 and compare slopes,
   not just the 1024 endpoint.

## Numbers to build on

Measured on the M1 Pro this repo targets; see `applegpu/bandwidth.md`.

- DRAM: **179.7 GB/s** asymptotic cold-stream, **219 ns** load-to-use latency
- Per-dispatch fixed cost: **4.2 µs**
- Size law `t(S) = S/179.7 GB/s + 4.2 µs`, good to ~1% on real llama classes
- Cache line **128 B** — one `(head, position)` row of this KV cache exactly
- System level cache ~24 MB (inferred); cache bandwidth ≥332 GB/s
- KV cache at 1030 context: 1.055 MB per tensor per layer, **33.75 MB** total
