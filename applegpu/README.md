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

## Conventions

- **Measured** — a number this repo produced, with the probe named.
- **Read** — extracted from an Apple binary or plist.
- **Inferred** — a conclusion from measurements, labelled as such.

## Unmined — vendor documentation worth reading

Listed so it is not forgotten, NOT cited. Nothing here has been read against
this machine yet, so nothing from it belongs in the files above until a
measurement stands beside it.

- [Debugging with interactive command-line tools](https://developer.apple.com/documentation/xcode/debugging-with-interactive-command-line-tools)
- [Investigating GPU issues with AI agents](https://developer.apple.com/documentation/xcode/investigating-gpu-issues-with-ai-agents)
