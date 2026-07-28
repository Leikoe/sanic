# Apple GPU stack — measured facts

Everything here was measured on this machine or read out of Apple's own
binaries. No vendor claims are repeated without a measurement beside them.

**Machine under test.** Apple M1 Pro, 14-core GPU, 16 GB unified, macOS.
`system_profiler SPDisplaysDataType`. Apple7 family.

| file | contents |
|---|---|
| [`hardware.md`](hardware.md) | what the machine reports about itself, and what it hides |
| [`bandwidth.md`](bandwidth.md) | what the memory system delivers, and to whom |
| [`execution.md`](execution.md) | command buffers, encoders, barriers, hazards |
| [`counters.md`](counters.md) | what is observable from code, and what is not |
| [`msl.md`](msl.md) | Metal Shading Language and compiler behaviour |

Reproductions live in `tests/` at the repo root:
`bandwidth_probe.rs`, `wide_loads_probe.rs`, `block_rows_probe.rs`,
`residual_stream_probe.rs`.

## `metal/` — Apple's reference, mirrored

[`metal/`](metal/) is a markdown mirror of Apple's Metal documentation, so the
reference is greppable offline instead of a browser tab and a network round trip
per lookup. [`tools/appledoc_md.py`](tools/appledoc_md.py) regenerates it.

**It is documentation, not measurement.** The rule above is unchanged: nothing
from `metal/` belongs in the five files listed above without a number measured
on this machine beside it. The mirror is kept in its own subtree precisely so
the boundary stays visible.

## Conventions

- **Measured** — a number this repo produced, with the probe named.
- **Read** — extracted from an Apple binary or plist.
- **Inferred** — a conclusion from measurements, labelled as such.

## Mined — what came of the vendor docs

- [Debugging with interactive command-line tools](https://developer.apple.com/documentation/xcode/debugging-with-interactive-command-line-tools)
  and [Investigating GPU issues with AI agents](https://developer.apple.com/documentation/xcode/investigating-gpu-issues-with-ai-agents)
  — both describe `gpudebug`, a text REPL over `.gputrace` files built for
  scripted and agent-driven use. **It does not exist on this machine** at macOS
  26.5.2 / Xcode 26.6; measured absence and what it would buy us are in
  [`counters.md`](counters.md#gpudebug-documented-and-not-on-this-machine).
  Revisit when a seed ships it — we already emit the traces.
