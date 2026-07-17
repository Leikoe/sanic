//! From a naive scaled-dot-product-attention graph to a FlashAttention
//! **Metal** kernel — every compiler stage, with its inputs and outputs made
//! visible.
//!
//! The whole point of the project in one file. We take `q`, `k`, `v` — exactly
//! the arguments `torch.nn.functional.scaled_dot_product_attention` takes (with
//! `enable_gqa=True`) — and write attention as a naive dataflow graph:
//!
//!     q : (batch, q_heads,  seq_len,    head_dim)      more query heads
//!     k : (batch, kv_heads, kv_seq_len, head_dim)      fewer, SHARED
//!     v : (batch, kv_heads, kv_seq_len, head_dim)      fewer, SHARED
//!     attn = softmax(qkᵀ · (1/√head_dim)  over kv_seq_len)·v
//!
//! Grouped-query attention (GQA — Llama, Mistral…): more query heads than
//! key/value heads. That is nothing but a reshape — `q_heads` SPLITS into
//! `(kv_heads, group)`, so a group of query heads shares one kv-head. k and v
//! are indexed by `kv_heads` alone, so they BROADCAST across `group` with no
//! head-repeat and no special kernel. (MHA is group=1; MQA is kv_heads=1 — same
//! graph, other extents.) `batch`, the head axes and `seq_len` all fall out as
//! FREE grid axes; the same `derive` that handles `sum` reconstructs the online-
//! softmax `(m, ℓ, o)` accumulator FlashAttention writes by hand. Max, exp,
//! sub, divide, two matmuls, a scale, one split — no flash-shaped template.
//!
//!     cargo run --example walkthrough
//!
//! The pipeline, in order:
//!   ir → analyze → derive → plan (+cost) → partition → emit(Metal) → verify

use std::collections::HashMap;

use sanic::cost::{Device, feasible, kernel_time};
use sanic::derive;
use sanic::emit_metal::emit_schedule_metal;
use sanic::interp::{Env, Extents, Value, eval};
use sanic::ir::*;
use sanic::partition::{Stage, partition_many};
use sanic::plan::plan_axis;
use sanic::{analyze_all, streamable, structure};

/// A tiny deterministic RNG so the numeric check needs no dependency.
struct Lcg(u64);
impl Lcg {
    fn f(&mut self) -> f64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        ((x.wrapping_mul(0x2545F4914F6CDD1D) >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0
    }
}

/// Print a labelled stage banner: what goes in, what comes out.
fn step(n: u32, name: &str, input: &str, output: &str) {
    println!("\n{}", "━".repeat(74));
    println!("STEP {n} · {name}");
    println!("   in  → {input}");
    println!("   out → {output}");
    println!("{}", "─".repeat(74));
}

/// Axis labels of a node's output shape, for printing.
fn shape(node: &Node) -> Vec<&'static str> {
    output_axes(node).iter().map(|a| a.label()).collect()
}

fn main() {
    // ── STEP 0: the naive graph ──────────────────────────────────────────────
    // Scaled-dot-product attention on given q, k, v — the SDPA arguments. q has
    // `q_heads`; k and v have the fewer `kv_heads`. We SPLIT q_heads into
    // (kv_heads, group) so each group of query heads shares one kv-head.
    let batch = axis("batch");
    let seq_len = axis("seq_len"); // query positions
    let kv_seq_len = axis("kv_seq_len"); // key/value positions (KV-cache length)
    let q_heads = axis("q_heads");
    let kv_heads = axis("kv_heads");
    let group = axis("group"); // q_heads = kv_heads × group  (the GQA ratio)
    let head_dim = axis("head_dim");
    let group_size = 4; // query heads per kv-head (the GQA ratio)
    let head_dim_size = 128;

    let q = input("q", &[batch, q_heads, seq_len, head_dim]);
    let k = input("k", &[batch, kv_heads, kv_seq_len, head_dim]);
    let v = input("v", &[batch, kv_heads, kv_seq_len, head_dim]);
    // GQA reshape: q_heads → (kv_heads, group). Now each group shares a kv-head.
    let qg = split(q.clone(), q_heads, kv_heads, group, group_size);
    // Scaled dot-product attention, spelled out from the basis:
    //   scores = (qkᵀ) · (1/√head_dim)  →  softmax over kv_seq_len  →  ·v
    // The 1/√head_dim scale is just a Mul. It FUSES into the flash lift — no
    // scale tensor, no extra pass (a causal mask would ride along the same way).
    let scale = konst(1.0 / (head_dim_size as f64).sqrt());
    let scores = matmul(qg, k.clone(), head_dim); // contract head_dim
    let scaled = map(MapOp::Mul, vec![scores, scale]);
    let attn = matmul(softmax(scaled, kv_seq_len), v.clone(), kv_seq_len);
    // (batch, kv_heads, group, seq_len, head_dim) — the (kv_heads, group) pair is q_heads

    // A realistic attention shape — Mistral-7B / Llama-3-8B: 32 query heads over
    // 8 kv-heads (GQA group=4), head_dim 128, a 2k-token context. The structure
    // map, plan, cost and emitted kernel below are all for THIS shape.
    let real: Extents = [
        (batch, 1),
        (seq_len, 2048),
        (kv_seq_len, 2048),
        (q_heads, 32),
        (kv_heads, 8),
        (group, group_size),
        (head_dim, head_dim_size),
    ]
    .into_iter()
    .collect();
    let real_price: HashMap<Axis, f64> = real.iter().map(|(&a, &n)| (a, n as f64)).collect();
    let dev = Device::toy();

    step(
        0,
        "ir — build the naive dataflow graph",
        "nothing (constructed in code)",
        "a Node DAG over the closed basis",
    );
    println!("   inputs : {:?}", leaf_names(&attn));
    println!("   q      : {:?}", shape(&q));
    println!("   k      : {:?}", shape(&k));
    println!("   v      : {:?}", shape(&v));
    println!("   attn   : {:?}   (the (kv_heads, group) pair is q_heads)", shape(&attn));

    // ── STEP 1: analyze — classify every (node, axis) ────────────────────────
    // The four-value ladder decides what streams: FREE and MONOIDAL axes fold
    // and parallelize; OPAQUE / SEQUENTIAL do not. `batch`, the head axes and
    // `seq_len` all land FREE — grid dimensions. `group` being FREE and absent
    // from k/v is exactly GQA's KV sharing.
    step(
        1,
        "analyze — classify how the graph depends on each axis",
        "the graph",
        "a structure map (FREE / MONOIDAL / OPAQUE / SEQUENTIAL per axis)",
    );
    print!("{}", analyze_all(&attn).render());
    println!(
        "\n   → `kv_seq_len` is {:?} and streamable? {}   → a one-pass fold",
        structure(&attn, kv_seq_len).level,
        streamable(&attn, kv_seq_len),
    );
    println!("   → batch, q/kv heads, group and seq_len are FREE → the grid the fold runs across");

    // ── STEP 2: derive — reconstruct the streaming accumulator ───────────────
    // The headline: from plain composition rules, `derive` finds the
    // FlashAttention (m, ℓ, o) carrier — a running max, a running normalizer,
    // and a running weighted output, with the deferred divide. No template.
    step(
        2,
        "derive — turn the foldable axis `kv_seq_len` into a concrete accumulator",
        "the graph + the MONOIDAL axis `kv_seq_len`",
        "a Carrier: the (m, ℓ, o) online-softmax accumulator",
    );
    let flash = derive(&attn, kv_seq_len).expect("`kv_seq_len` folds into a carrier");
    println!("{}", flash.render());
    println!(
        "   slots = {}   moves fired = [{}]",
        flash.slots,
        flash.rules.join(", "),
    );

    // ── STEP 3: plan (+ cost) — pick a block that fits the device ─────────────
    // Legality is settled; the cost model only *ranks*. The planner reads axis
    // roles off the carrier's slot spans and the IR, then prices a tile that
    // fits SRAM. The FREE axes become grid / tiled blocks; kv_seq_len is streamed.
    step(
        3,
        "plan + cost — choose streamed axis, tile, and block structure",
        "graph + axis + carrier + Device + extents",
        "a KernelSpec (block dims, resident state, priced cost)",
    );
    let spec =
        plan_axis(&attn, kv_seq_len, &flash, &dev, &real_price).expect("a block fits the device");
    println!(
        "   streamed axis    : {}   (folded away, O(1) state)",
        spec.streaming_axis,
    );
    println!(
        "   row tile         : {}×{}",
        spec.row_axis.map(|a| a.to_string()).unwrap_or_else(|| "scalar".into()),
        spec.tile_m,
    );
    if let Some(c) = spec.col_tile_axis {
        println!("   col tile         : {}×{}", c, spec.tile_c);
    }
    println!(
        "   grid axes        : {:?}   (independent blocks — DOALL, no cross-block comms)",
        spec.batch_axes.iter().map(|a| a.label()).collect::<Vec<_>>(),
    );
    println!(
        "   resident scalars : {} = {} rows × {} slots  (the flash state, in SRAM)",
        spec.tile_m * spec.carrier.slots,
        spec.tile_m,
        spec.carrier.slots,
    );
    println!(
        "   roofline         : {:.2e} flop · {:.2e} HBM B · {:.0} B SRAM/block · {:.0} blocks",
        spec.roofline.flops,
        spec.roofline.hbm_bytes,
        spec.roofline.sram_per_block,
        spec.roofline.parallel_blocks,
    );
    println!(
        "   time @ toy dev   : {:.3e} s   (feasible: {})",
        kernel_time(&dev, &spec.roofline),
        feasible(&dev, &spec.roofline),
    );

    // ── STEP 4: partition — split the whole graph into a schedule ────────────
    // The derive frontier IS the fusion boundary. q, k, v are the inputs (the
    // SDPA arguments), so there is nothing upstream to cut: the whole attention
    // — the qkᵀ score, the online softmax, the ·v — fuses into ONE flash kernel.
    // (Project q/k/v from an `x` first and partition would cut those GEMMs into
    // their own producer kernels; this flash fold would stay identical.)
    step(
        4,
        "partition — whole graph → kernels, cutting only where the algebra stops",
        "the graph + Device + extents",
        "a Schedule: Stages in execution order",
    );
    let sched = partition_many(&[(attn.clone(), "attn")], &dev, &real_price);
    print!("{}", sched.render());
    println!("\n   stage-by-stage (reads → writes):");
    for (i, st) in sched.stages.iter().enumerate() {
        match st {
            Stage::Fused { spec, epilogue, .. } => println!(
                "     [{i}] FUSED   fold `{}`  reads [{}] → {}{}",
                spec.streaming_axis,
                spec.input_names.join(", "),
                spec.output_name,
                if epilogue.is_empty() {
                    String::new()
                } else {
                    format!("  +epilogue {}", epilogue.join("·"))
                },
            ),
            Stage::Elementwise { ops, inputs, output, .. } => println!(
                "     [{i}] MAP     {}  reads [{}] → {}",
                ops.join("·"),
                inputs.join(", "),
                output,
            ),
            Stage::Gather { axis, inputs, output, .. } => println!(
                "     [{i}] GATHER  `{}`  reads [{}] → {}",
                axis,
                inputs.join(", "),
                output,
            ),
            Stage::Sequential { op, axis, inputs, output, .. } => println!(
                "     [{i}] SCAN    `{op}`/`{}`  reads [{}] → {}",
                axis,
                inputs.join(", "),
                output,
            ),
            Stage::Infeasible { axis, output } => {
                println!("     [{i}] INFEASIBLE `{}` → {}", axis, output)
            }
        }
    }
    println!(
        "   schedule inputs  : {:?}\n   schedule outputs : {:?}",
        sched.reads(),
        sched.outputs,
    );

    // ── STEP 5: emit — lower the schedule to Metal Shading Language ───────────
    // The schedule the interpreter runs, transcribed to MSL: a real batched
    // grouped-query FlashAttention kernel reading q, k, v. Watch the k/v reads
    // omit `group`: that index simply isn't there, so every query head in a
    // group reads the same k/v — GQA sharing, with nothing added.
    step(
        5,
        "emit(Metal) — lower the schedule to a Metal kernel",
        "the Schedule + extents",
        "Metal Shading Language: the FlashAttention kernel",
    );
    let program = emit_schedule_metal(&sched, &real);
    println!("   Metal program: {} kernel\n", program.stages.len());
    // one kernel in the schedule → program.msl IS the flash kernel, lowered the
    // way the plan chose (we slice off the small typedef preamble).
    let body = program
        .msl
        .find("kernel void")
        .map(|i| &program.msl[i..])
        .unwrap_or(program.msl.as_str());
    println!("{body}");

    // ── STEP 6: verify — the emitted kernel computes the real math ───────────
    // Run the schedule through the reference interpreter on real tensors and
    // compare against a dense eval of the ORIGINAL naive graph. Equality is the
    // compiler-correctness theorem, reduced to a number: derivation, planning,
    // cuts and lowering all preserve the textbook semantics.
    step(
        6,
        "verify — executed schedule == naive graph, to machine precision",
        "the Schedule + real q, k, v tensors",
        "max |flash − naive| ≈ 0",
    );
    // The dense reference interpreter materializes the whole (…, head_dim,
    // kv_seq) product, so it is far too slow at the shape above. Correctness is
    // a property of the DERIVATION, not the size, so we check the very same
    // schedule at a small shape — an 8-token context and 2 kv-heads. head_dim
    // stays 128, so the baked 1/√128 scale is exact; the tiny run certifies the
    // real one.
    let verify: Extents = [
        (batch, 1),
        (seq_len, 8),
        (kv_seq_len, 8),
        (q_heads, 8),
        (kv_heads, 2),
        (group, group_size),
        (head_dim, head_dim_size),
    ]
    .into_iter()
    .collect();
    let mut rng = Lcg(0x5EED_1234_ABCD);
    let mut r = |axes: &[Axis]| Value::from_fn(axes, &verify, |_| rng.f());
    let env: Env = [
        ("q", r(&[batch, q_heads, seq_len, head_dim])),
        ("k", r(&[batch, kv_heads, kv_seq_len, head_dim])),
        ("v", r(&[batch, kv_heads, kv_seq_len, head_dim])),
    ]
    .into_iter()
    .collect();

    let out = sched.execute(&env, &verify);
    let reference = eval(&attn, &env, &verify).permuted_to(&out.axes);
    let max_abs = out
        .data
        .iter()
        .zip(&reference.data)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f64, f64::max);

    println!("   max |schedule − naive graph| = {max_abs:e}");
    assert!(max_abs < 1e-9, "the derived flash kernel diverged from the naive reference");
    println!("   ✓ the derived grouped-query FlashAttention kernel reproduces softmax(qkᵀ)·v exactly");
}
