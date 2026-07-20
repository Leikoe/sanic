# The attic

Examples parked while the API is churning. Cargo does not build this
directory, so nothing here costs anything to keep — and nothing here is
promised to compile. `examples/llama3.rs` is the ONE example kept up to date
against the current frontend API; everything else waits here and gets revived
in a batch when the surface settles.

Parked at the axes-as-plain-data boundary (2026-07-17): `Axis` became
`{ name, extent }` with structural identity, the `Extents` side-tables left
every pipeline signature, and `interp::Tensor` became `interp::Value`. The
migration recipe for reviving a file is mechanical and lives in `todo.md`
(the "Axes are plain data" entry): give every `axis("x")` its extent, delete
the extents maps, drop the extents argument from `eval` / `partition` /
`plan` / `grad` / `execute` / `Session::new` / `Value::from_fn`, and read
`.name` / `.extent` off the axis.

Since parked, the hand-rolled loaders also left (2026-07-19):
`src/safetensors.rs` (`StFile`, `load`, `RawTensor`, `bf16_roundtrip`),
its tiny JSON parser, and the `src/bpe.rs` GPT-2 encoder are all gone in
favor of the `safetensors` and `tokenizers` crates (dev-dependencies),
and the page-aligned zero-copy open is example-local now —
`examples/llama3_2.rs` shows the pattern.

What lives here and why it's worth reviving:

* `gpt2.rs` — the capstone: real OpenAI weights on the GPU matching
  HuggingFace, KV-cache decode at ~8 ms/tok, zero-copy weight binding.
* `trinity.rs` — 5.5B int4 MoE on a 16 GB laptop: GQA as axis structure,
  top-8 routing as folds, packed-nibble GEMMs, measured-tuning flags.
* `mnist.rs` / `shakespeare.rs` — training end to end on the GPU: `grad`,
  fused SGD epilogues, buffer-swap commits (these two were mid-migration
  when parked and may be closest to compiling).
* `derive.rs`, `llm.rs`, `mha.rs`, `mla.rs`, `walkthrough.rs`, `kernels.rs`,
  `execute.rs`, `probe.rs` — the guided tours (structure maps, attention, the
  13-kernel block, and hand-scheduled prototype probes).
