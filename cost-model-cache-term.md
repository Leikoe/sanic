# Issue: the cost model has no cache term

`src/cost.rs` prices every byte at DRAM rates. Re-reading a 4 KB activation
2048 times is charged as 8 MB of DRAM traffic when it is one DRAM read and 2047
cache hits. Every fusion decision that recomputes a small hot operand is
therefore mispriced toward materializing it.

Self-contained; nothing below assumes context from the session that found it.

## The evidence that the model is wrong

**k/v projections fuse their input RMS norm and it measurably pays.** The
`defer-div+tuple` carrier computes Σx² alongside the projection dot, so each of
512 output threadgroups re-reads the whole 2048-element activation — 512 × 4 KB
= 2 MB of re-reads on top of a 2 MB weight. If those re-reads were DRAM traffic
the class would run at half rate.

It does not. Measured, that class sustains **120 GB/s on 2 MB**, and the
independently-fitted size law `t(S) = S/179.7 GB/s + 4.2 µs` predicts
**119.8 GB/s** for a 2 MB read *counting the weight alone*. The re-reads cost
approximately nothing, because 4 KB stays in cache.

**The model says the opposite.** `inline_pays` (`src/partition.rs`, Prop 4.1)
prices that same cone's recompute through `cost::sustainable_bandwidth`, a DRAM
rate, and returns false — materialize. Algebra and pricing disagree, and the
measurement sides with algebra.

Reproduce the disagreement:

```
cargo test --release --test wide_loads_probe -- --nocapture   # the size law
SANIC_DEBUG=2 cargo run --release --example llama3_2 -- "hi" -n 1 --bf16
```

In the dump, per layer: `k_proj` and `v_proj` carry `defer-div+fold+fused-map+tuple`
and read the residual plus the norm weight directly. `q_proj` reads a
materialized `tNN` produced by a separate reduce/apply pair — two kernels per
layer, 32 across the model, whose only consumer is that one projection.

## Why it matters

A llama-3.2-1B bf16 decode step is 15.6 ms of GPU time against a 13.75 ms floor
of irreducible weight streaming. Of the 1.85 ms gap, ~1.47 ms is work that
moves almost no bytes — norms, rope, silu, cache writes — much of which exists
because the model believes recomputing is expensive. That is the largest
remaining item in the step and it is downstream of this pricing.

## What a fix has to get right

The two cases must come out differently, and today nothing distinguishes them:

| case | consumers of the norm | correct answer |
|---|---|---|
| input layernorm → q | **one** (q only) | fuse; materializing costs 2 kernels to serve 1 consumer |
| post-attention norm → gate, up | **two** | materialize once; fusing computes it twice |

If a change makes both fuse, or both materialize, it has not solved the
problem. That asymmetry is the acceptance test.

## Numbers to build on

All measured on the M1 Pro this repo targets; see `applegpu/bandwidth.md`.

- DRAM: **179.7 GB/s** asymptotic cold-stream, **219 ns** load-to-use latency
- Per-dispatch fixed cost: **4.2 µs**
- Size law: `t(S) = S/179.7 GB/s + 4.2 µs`, which predicts the per-class rates
  of a real llama step to within ~1%
- System level cache: **~24 MB** (inferred — no property exposes it)
- **Cache bandwidth: ≥332 GB/s.** A 16 MB buffer re-read returns 332 GB/s,
  which is above anything the DRAM can do; that number is a lower bound on the
  cache rate and the closest thing to a measured constant for the new term
- Cache line 128 B

## Shape of a fix

The missing quantity is whether a byte comes from DRAM or from cache. A
first-order model: traffic to an operand that is **small relative to the cache
and read repeatedly within one kernel** prices at cache bandwidth, not
`hbm_bandwidth`.

`rebuild_cost` in `src/partition.rs` already walks exactly the cone whose bytes
are in question and knows each node's volume, so tagging those bytes hot/cold
is local to it. `cost::sustainable_bandwidth` is the DRAM rate; a sibling for
cache would keep the two rates named and separate.

Open design questions, in the order they bite:

1. **What counts as hot?** Volume × dtype under some fraction of 24 MB is the
   crude version. The honest version is reuse distance, which the partitioner
   does not currently track.
2. **Reuse is per streamed element.** The cone is re-evaluated once per output
   point, so an operand is hot iff it stays resident across the whole fold, not
   merely iff it is small.
3. **Does the roofline still hold?** `kernel_time` and the tuner also divide by
   `hbm_bandwidth`. If the cache rate enters only `inline_pays`, the model
   becomes internally inconsistent — worth deciding deliberately rather than by
   omission.

## How to verify a fix

1. `inline_pays` starts returning true for cones over small hot operands.
   Instrument it and look at the distribution, currently **93 false / 128 true**
   on a llama step.
2. The 32 q-norm kernels disappear; gate/up's shared norm **stays**
   materialized. This is the acceptance test above.
3. `cargo test --release` — 225 pass, 0 fail.
4. Both dtypes, text byte-identical:
   `./target/release/examples/llama3_2 "The capital of France is" -n 32 --bf16`
   should still print `…is Paris. It is the most populous city in France and
   the center of the Paris`, and the f32 default must still compile and run.
5. Perf: **counterbalanced ABBA** against the base commit. Expect ~0.18 ms/step
   if only the q norms fold in; a change to a cut criterion touches every
   schedule decision, so diff the per-class table rather than the total.

## Traps this repo has already fallen into

Each cost real time; none are hypothetical.

- **Measurement regimes do not transfer.** `SANIC_DEBUG=4` gives every kernel
  its own encoder and understates production by ~10%. A standalone probe that
  reuses one small weight measures the SLC — a 16 MB re-read reports 332 GB/s.
  Neither predicts a real step. Trust wall-clock A/B and back-to-back probes
  over distinct cold buffers.
- **Always-baseline-first A/B is biased.** On this machine whichever variant
  runs first loses. An alternating-but-ordered A/B produced a spurious +3.4%
  that vanished under ABBA. Report the pooled sd and how many
  position-matched pairs the winner won.
- **Do not generalize from filtered instrumentation.** The 93/128 split above
  was first reported as "93 out of 93 false" because the `eprintln!` was gated
  on `volume >= 2048`.
- **Check the f32 path.** It is the default and no test covers it; a change can
  pass all 225 tests and still leave `--bf16`-only working.
