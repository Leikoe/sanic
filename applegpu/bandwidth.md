# Memory bandwidth — M1 Pro

Theoretical: LPDDR5-6400 on a 256-bit bus = **204.8 GB/s**. Apple quotes
200 GB/s. System level cache 24 MB.

All numbers: `tests/bandwidth_probe.rs`, min of ≥15 timed passes after warmup,
reproducible to ~1% across runs.

## The rated number is not a per-client number

| | GB/s |
|---|---|
| GPU alone | 184.4 |
| GPU while 6 CPU threads stream | 120.4 |
| CPU concurrently | 90.1 |
| **combined** | **210.5** |

CPU side is 6 threads × 384 MB slabs (2.3 GB live, 96× the SLC), so it is DRAM
traffic, not cache replay.

**One GPU client saturates at 185–188 GB/s = 90–92% of theoretical**, and no
kernel shape tried moves it (table below). The fabric
delivers ≥210 GB/s when two engines drive it. A GPU-only workload cannot reach
the rated figure; nothing about the kernel changes that.

## Traversal locality is worth 48%

Same 2 GB, same 65k lanes, same bytes read. Only the live page set differs.

| traversal | GB/s |
|---|---|
| one grid-stride pass over 2 GB | 121.5 |
| 4 windows × 512 MB | 178.2 |
| 16 windows × 128 MB | 179.8 |
| 32 windows × 64 MB | 179.7 |

A grid-stride over a whole buffer puts every lane on its own page — 65k pages
live at once. Windowing bounds the live set. **Inferred:** address-translation
reach, not DRAM, is what falls over.

Corollary: a size sweep that strides the whole buffer conflates footprint with
live page set and reads as "bandwidth degrades with size". It does not.

## Transfer size

Ordinary rising limb, measured with windowed traversal so translation reach is
not the variable:

| working set | 64 MB | 128 | 256 | 512 |
|---|---|---|---|---|
| GB/s | 181.6 | **188.5** | 185.7 | 180.7 |

188.5 GB/s is the highest single number observed on this machine.

## Saturation shape

Peak needs ~65k lanes (2^16) of **scalar 4-byte** reads, which is 64
threadgroups of 1024.

| dimension | tried | effect |
|---|---|---|
| per-lane width | 4, 8, 16 B | none, or worse |
| independent streams per lane | 1, 2, 4, 8 | deeper is worse (170 at depth 4) |
| threadgroup size | 64, 256, 1024 | none (180.2 at 256, 175.7 at 64) |
| lane count | 8k – 4M | optimum ~65k; 4k gives 54 |
| threadgroups vs core count | 14, 28, 56, 64, 112, 224, 448 | **not core-aligned**: 64 → 187.9 beats 56 → 177.8 and 112 → 185.2 |
| storage mode | `Shared` vs `Private` | identical — coherency is not a cost |
| pages populated first | yes/no | ~5%; not faulting |
| direction | read vs read+write copy | same (185 vs 181 combined) |
| concurrent kernels | 1, 2, 4, 8 | 1 is best; more is worse |

Fixed cost of a dispatch: **3.9 µs** (empty kernel, GPUStartTime→GPUEndTime),
0.5% of a 733 µs kernel. Not a significant term at these sizes.

Powers of two beat exact multiples of the 14 cores, so threadgroup dispatch is
not plain round-robin over cores.

## Why it stops there: ~23 outstanding lines per core

Measured, not inferred. Bandwidth scales **linearly with cores** and flattens
exactly at the core count:

| threadgroups of 1024 | 1 | 2 | 4 | 7 | 14 | 28 | 64 |
|---|---|---|---|---|---|---|---|
| GB/s | 13.5 | 25.1 | 54.3 | 91.8 | 163.5 | 164.1 | 188.1 |
| per threadgroup | 13.5 | 12.6 | 13.6 | 13.1 | 11.7 | — | — |

One core sustains **~13.1–13.5 GB/s**, and 14 × 13.1 = 183 GB/s, which is the
ceiling every other experiment kept hitting.

A dependent pointer chase (8 KB stride, new page every hop, so no prefetch and
no row reuse) measures **219 ns** of load-to-use latency. Bandwidth is
outstanding bytes over latency, so one core holds:

```
13.5 GB/s × 219 ns = 2,956 B = 23 cache lines of 128 B
```

**That is the cap: about 23 outstanding line requests per core.** Reaching the
204.8 GB/s theoretical would need 14.63 GB/s per core — **25 lines**. The whole
shortfall is two more misses in flight per core.

This explains every flat row in the table above at once. Lanes, per-lane width,
stream depth, threadgroup size and concurrent kernels all fail for the same
reason: none of them buys miss slots. It also explains the two things that DID
move: a second engine (the CPU) brings its own request path, and traversal
locality matters because a translation miss occupies a slot.

Past 14 threadgroups there is a second, smaller rise (164 → 188 at 64), which
is latency hiding *within* a core rather than more cores.

### The texture path is the same queue

Apple GPUs serve textures through their own cache, so a kernel driving buffer
and texture together would beat either if the two had separate miss slots. Same
128 MB, `texture_buffer` view over the same bytes, no copy:

| path | buffer only | texture only | both at once |
|---|---|---|---|
| GB/s | 169.8 | 169.2 | **170.2** |

The texture path reaches the same ceiling alone and adds 0.2% when combined.
One queue, shared by both.

### Open: occupancy

GPU counters put **Compute Occupancy at 63% median, 77% max** while the read
path is at 96%. If occupancy could be raised, more resident threads would hold
more requests in flight. Whether that is reachable — or whether the request
queue saturates regardless of how many threads want it — is not settled, and it
is the one number in the counter table that has not been explained. The
counters can answer it directly rather than by elimination.

## More GPU queues do not help — they hurt

Independent `MTLCommandQueue`s, each with its own command buffer, committed at
once and timed over the GPU span (first `GPUStartTime` to last `GPUEndTime`):

| queues | 1 | 2 | 4 | 8 |
|---|---|---|---|---|
| GB/s combined | 159.2 | 74.0 | 58.9 | 53.1 |

Separate submission contexts feed the same 14 cores, so there are no extra miss
slots to win — and the contexts contend. Multiple GPU clients is not a lever;
multiple ENGINES is (see the CPU result above).

## Consequence for inference

A llama-3.2-1B bf16 decode step must read 1,235,814,400 weights = 2,471.6 MB
per token. Arithmetic intensity 1.0 FLOP/byte against a machine balance of
25 — memory-bound by 25×, compute never binds.

| | ms/step |
|---|---|
| 2,471.6 MB at 179.7 GB/s — GPU-only floor | 13.75 |
| + the 4.2 µs per dispatch at current weight sizes | 14.13 |
| measured, GPU replay | 15.60 |
| measured, wall | 16.90 |

So the step decomposes as:

- **13.75 ms irreducible** — the weights, at the best rate cold streaming
  reaches. 72.7 tok/s is the ceiling for this model on this GPU.
- **0.37 ms (2.4%)** per-dispatch cost at current tensor sizes. Merging q/k/v
  into one 12 MB read and gate+up into one 67 MB read recovers **0.20 ms** of
  it; the rest is unavoidable while the model has this many distinct weights.
- **1.47 ms (9.5%)** everything else — the small ops that move almost no bytes
  (norms, rope, attention, silu) and the stalls between dependent phases.

Measured 15.60 against a 13.75 floor is **88%**. The largest remaining item is
not bandwidth and not kernel quality: it is the 1.47 ms of work that streams
nothing.
