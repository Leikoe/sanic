# What the machine says about itself

Read off this M1 Pro, with the command that produced each line. Nothing here is
a vendor claim.

## Memory and cache

```
system_profiler SPMemoryDataType     LPDDR5, Hynix, 16 GB
sysctl hw.cachelinesize              128
sysctl hw.memsize                    17179869184
sysctl hw.perflevel0.l2cachesize     12582912   (6 performance cores)
sysctl hw.perflevel1.l2cachesize      4194304   (2 efficiency cores)
```

System level cache is **~24 MB**, inferred rather than read: a repeated read of
a 16 MB buffer returns 332 GB/s — above anything the DRAM can do — while 33 MB
and up return DRAM rates. No property exposes the size.

## GPU

```
system_profiler SPDisplaysDataType   Apple M1 Pro, 14 cores
```

Apple7 / G13 family. SIMD width 32. Threadgroup memory 32 KB.

## DVFS ladders, from the device tree

`ioreg -lw0` exposes `voltage-states<N>` blobs under the power manager. Each is
a sequence of `(u32 Hz, u32 mV)` pairs. The format is not documented; it was
established by decoding `voltage-states14` and finding it reproduces the GPU's
known ladder exactly, then applying the same decoder to the rest.

| table | states (MHz) | what |
|---|---|---|
| `voltage-states9`, `14` | 0, 388.8, 486, 648, 777.6, 972, **1296** | GPU |
| `voltage-states8` | 300, 540, 780, 1020, 1260, **1500** | memory controller |
| `voltage-states31` | 400, 720, 900 | unidentified |
| `voltage-states1`, `5`, `13` | encoded differently | CPU clusters |

Voltages run 550–875 mV for the memory controller and 400–878 mV for the GPU.

The **1500 MHz memory controller ceiling** is not published anywhere I have
found. Under load the GPU pins its own top state (see `execution.md`).

## DRAM controller properties

`ioreg` carries a handful of `dcs-*` (DRAM Controller Subsystem) properties:

```
dram-capacity        = 8
dcs-bw-threshold     = 16 u32s: 2413035, 0, 0, 0, 4822138, 0, 2347499, 3, …
dcs-bwr-threshold    = 2064384, 5072486, 7176192, 9155379
dcs-tvm, dcs-metadata
```

**Units unknown.** They are plausibly bandwidth thresholds for controller DVFS,
but nothing here establishes the scale and no interpretation is offered. The
four-entry shape of `dcs-bwr-threshold` and the four groups in
`dcs-bw-threshold` are suggestive of four steps or four channels, and that is
as far as the evidence goes.

## What is NOT exposed

**Bus width and channel count.** Nothing found in `ioreg`, `sysctl`, or
`system_profiler` reports either.

This matters for every percentage in these documents: the **204.8 GB/s
theoretical** figure (LPDDR5-6400 × 256-bit) is derived from Apple's marketing
number, not read from this machine. The measured quantities that do not depend
on it — 179.7 GB/s cold-streaming asymptote, 210.5 GB/s across two engines,
~13.3 GB/s per core, 219 ns latency — are the safer ones to reason from.
