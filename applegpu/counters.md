# Performance counters — what is reachable

## From code: one counter

Queried directly on this M1 Pro:

```
sampling points: AtStageBoundary only
counter sets:    timestamp, containing one counter, GPUTimestamp
```

No stage-utilization, no statistics, no memory or occupancy counters. Not gated
by environment — `METAL_CAPTURE_ENABLED`, `MTL_CAPTURE_ENABLED`,
`METAL_DEVICE_WRAPPER_TYPE`, `MTL_COUNTERS_ENABLED` all leave the set at one.

So a stage-boundary GPU timestamp is the entire programmatic budget. That is
enough for per-kernel time, and nothing else.

## The private API exists and is entitlement-gated

Present on the real classes (`AGXG13XDevice → IOGPUMetalDevice → _MTLDevice`):

```
_MTLDevice supportsGPUStatistics            -> 1
_MTLCommandQueue requestCounters:
_MTLCommandQueue addPerfSampleHandler:
_MTLCommandBuffer runPerfCounterCallbackWithBlock:
_MTLDevice resolveCounters:withRange:
```

Every one returns nil/0 unentitled. Disassembling `AGXGPURawCounterImpl::init`
and reproducing it standalone locates the wall exactly:

```
IOConnectCallStructMethod(gpu, selector 261, 72B, 72B) -> kIOReturnNotPrivileged
```

Neighbouring selectors 0x100–0x104 are ungated; only 261 is privilege-checked.
**Not SIP** — disabled here, still refuses. AMFI enforcing
`com.apple.private.gputools.client`, which `gputoolsserviced` carries and a
self-signed binary cannot. Forging a set via `MTLCounterSetInternal` and the
MTL4 counter path are both dead too.

## What does work: xctrace, out of process — and it is parseable

Instruments injects nothing into the target; collection runs entirely in the
entitled daemon. Drive its CLI — no root, no Developer Mode, no Xcode UI:

```
xcrun xctrace record --instrument "Metal GPU Counters" \
    --output run.trace --target-stdout - --launch -- <binary>
xcrun xctrace export --input run.trace \
    --xpath '/trace-toc/run[@number="1"]/data/table[@schema="gpu-counter-info"]'
xcrun xctrace export --input run.trace \
    --xpath '/trace-toc/run[@number="1"]/data/table[@schema="gpu-counter-value"]'
```

`--instrument "Metal GPU Counters"` is the part that matters. `--template
"Metal System Trace"` records with `Counter Set: (null)` and yields nothing.

That gives all **31 WWDC20 counters**: ALU / Buffer Read / Buffer Write /
Texture Cache / Threadgroup / GPU Last Level Cache / MMU limiters and
utilizations, Compute Occupancy, GPU Read and Write Bandwidth, MMU TLB Miss
Rate. `gpu-counter-info` maps counter-id → name; `gpu-counter-value` is
(timestamp, counter-id, value).

**Parsing note.** The exported XML uses an id/ref compression scheme: a cell
either carries `id="N"` and its text, or `ref="N"` reusing an earlier cell's
text. A parser must keep an id→text table and resolve refs, or every repeated
value reads as empty. Columns are positional, in schema order. Export is ~135 MB
for a 0.7 s trace, so use `iterparse` and keep workloads sub-second.

Counters are GPU-WIDE (keyed by accelerator, not pid) — quiesce the machine.
The sample interval is 10 ms, so absolute bandwidth figures are averaged over
windows that include idle time and read LOW; use the limiters relatively.

## Measured: what actually limits a streaming read

Top 200 windows by read bandwidth, during a pure-read benchmark:

| counter | median | max |
|---|---|---|
| **Buffer Read Limiter** | 57.9 | **96.2** |
| **MMU Limiter** | 54.4 | **83.9** |
| GPU Last Level Cache Limiter | 43.4 | 52.7 |
| Compute Occupancy | 63.4 | 76.7 |
| ALU Limiter | 6.3 | 9.9 |
| MMU TLB Miss Rate | 1.4 | 19.0 |

The load-request path saturates first (Buffer Read Limiter to 96%), with
address translation close behind (MMU Limiter to 84%). ALU at 6% confirms
compute is irrelevant. This is the direct-measurement version of what
`bandwidth.md` infers from timing: the ceiling is the request path, and
translation is a real co-limiter — which is why windowing a large read is worth
48%.

## What does work: a trace document

`MTLCaptureManager` with destination `GPUTraceDocument`, under
`METAL_CAPTURE_ENABLED=1`, writes a `.gputrace` that opens in Xcode with all
150+ counters per dispatch — limiters (ALU, buffer read/write, last-level
cache), bytes read from main memory, occupancy.

Out of process, in an entitled daemon. That is the supported route to anything
past a timestamp. `SANIC_GPUTRACE=<path>` in `src/metal.rs`.

### `gpudebug`: documented, and NOT on this machine

Apple documents a command-line reader for exactly these traces — a text REPL
with a `performance` subtree, `info`, and `fetch`, explicitly pitched at
scripted and agent-driven use. It would be the terminal route to the counters
that currently need the Xcode GUI.

It does not exist here. **Measured 2026-07-28**, macOS 26.5.2 (25F84),
Xcode 26.6 (17F113) — newer than the macOS 26.2 in Apple's own example:

```
which gpudebug                  -> not found
xcrun -f gpudebug               -> unable to find utility "gpudebug"
man -w gpudebug                 -> No manual entry
find /Applications/Xcode.app -name 'gpudebug*'   -> nothing
find /Library/Developer -name 'gpudebug*'        -> nothing
```

So it ships in some SDK or seed this machine does not have, and nothing in
`applegpu/` may lean on it yet. Everything below about what it *would* show is
Apple's documentation, unverified here:

- root has four children: `commands`, `performance`, `api_calls`, `resources`
- `gpudebug -t t.gputrace -c list`, then `gpudebug -s <id> -c ...` reuses the
  loaded session; `--oneshot` pays the load cost every call
- every documented example is RENDER work — draws, color attachments, vertex
  layouts. Whether a compute-only trace exposes anything useful under
  `performance` is unknown, and is the first thing to check.

We already emit the input: `SANIC_GPUTRACE=<path>` writes one. When `gpudebug`
appears, the experiment is one command against a trace we can produce today.

## Instruments / xctrace

Counters are **GPU-wide** (keyed by accelerator, not pid) — quiesce the machine
before trusting a trace. Export is ~250 MB of XML per second, so keep traced
workloads sub-second.

Counter-set selection lives in the `.tracetemplate`, an NSKeyedArchiver plist
with plain integer knobs: `counterprofile` (3 = 31 limiters, 4 = 16
utilization+bandwidth; other values empty).

## Static tables

Derived counter definitions for this exact GPU, if absolute byte counts are
ever needed by a cost model:

```
/System/Library/Extensions/AGXMetalG13X.bundle/Contents/Resources/
  AGXMetalStatisticsExternalA14X-counters.plist   276 derived counters,
                                                  incl. Bytes Read From Main
                                                  Memory, L2 Bytes Read/Written
  AGXMetalPerfCountersExternal.plist              mux registers
  *-derived.js                                    formulas
```

`Bytes Read From Main Memory` is not exposed by xctrace.
