# Command submission and hazards

## Encoders

An encoder boundary idles the GPU **~10–16 µs**. A 263-dispatch step encoded as
263 compute encoders spends ~15% of the step in those gaps.

One `MTLDispatchTypeConcurrent` encoder per graph, with
`memoryBarrierWithScope(Buffers)` at dependency frontiers, removes them.
Measured on llama bf16: 23.5 → 15.6 ms/step.

**Metal does no hazard tracking between dispatches inside a concurrent
encoder.** The barrier schedule is load-bearing for correctness, not
performance.

A barrier costs **~0.3 µs**. Forcing one before every dispatch (263 instead of
212) costs ~0.15 ms/step total.

## Barrier placement

Both this compiler and MLX arrive at the same law independently:

> accumulate reads and writes since the last barrier; barrier on RAW/WAW/WAR
> against **allocation** identity; reset both histories at the barrier.

`barrier_schedule`/`barrier_before` in `src/metal.rs`;
`CommandEncoder::maybeInsertBarrier` in `mlx/backend/metal/device.cpp`. The
difference is when it runs — at capture from a static graph here, per dispatch
in MLX because it is eager.

## Command buffers

Buffers committed to one queue execute in **commit order**. A chunk boundary is
therefore itself a barrier, and splitting a dispatch list across buffers
preserves a capture-time barrier schedule as long as the flags are sliced in
lockstep. Verified byte-identical output at 1, 2, 3, 7, 139 and 278 buffers per
step (278 = every dispatch in its own buffer, so every dependency edge crosses
a boundary).

Chunking a replay across buffers to overlap host encode with GPU execution has
**no measurable effect** here: +0.8% against a noise sd of 1.05 over
counterbalanced runs. An earlier +3.4% was order bias.

MLX commits every 40 ops **or** 40 mega-*elements* of distinct inputs
(`data_size()` is in items, not bytes) — `device.cpp:604-624`. At 1.24 G
elements of weights per llama step the byte cap dominates: ~31 buffers/step.

## Indirect command buffers

Concurrent-dispatch ICBs are unstable above ~64 commands on Apple7. Two
distinct failure modes; only one has a clean fix (pipelines conform to
`MTLAllocation`, so one residency set covers buffers and pipelines). The
instability postdates that fix. ICBs are not a foundation on this hardware.

tinygrad works around the same crash by forcing every pipeline resident with a
zero-size dispatch before `executeCommandsInBuffer` (`FIX_METAL_ICB`).

## DVFS

DVFS states for this M1 Pro, from the device tree
(`IODeviceTree:/arm-io/pmgr`, property `voltage-states9`):

```
OFF, 388.8, 486.0, 648.0, 777.6, 972.0, 1296.0 MHz
```

Verified residency: idle sits 55% OFF / 45% P1; a Metal load sits **100% P6 at
1296 MHz**. Under load the clock is pinned at the top state, and a benchmark's
mean frequency reads below that only while it is still ramping — first replays
of a llama step measure 18.5 ms against 15.9 warm.

**The performance state cannot be pinned, only measured.** Setting
`gpuperformancestate` in a `.tracetemplate` is recorded faithfully and does not
move the clock: min/med/max 1241.8 / 1254.1 / 1291.7 MHz, all P6.

Reading it is cheap and unprivileged: `IOReport`, group `"GPU Stats"`, subgroup
`"GPU Performance States"`, channel `GPUPH` — residency per state at 24 MHz
ticks, ~1 ms to sample. `ticks/24e6` recovers the wall window to ~2%, so a bad
sampling window is self-evident. `device.clock()` in `src/metal.rs`.

Every benchmark should gate on this. Clock drift is what forced per-family
adjacent baselines in the tuner.
