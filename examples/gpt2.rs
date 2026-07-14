//! GPT-2 (124M), real OpenAI weights, end to end — the capstone.
//!
//! ```text
//! cargo run --release --example gpt2               # generate, streaming tokens as they land
//! cargo run --release --example gpt2 -- --n 32     # cap the number of generated tokens
//! cargo run --release --example gpt2 -- --bf16     # also run with bf16-round-tripped weights
//! cargo run --release --example gpt2 -- --oracle   # also run the f64 interpreter oracle (slow)
//! ```
//!
//! What happens: the official `model.safetensors` is loaded by the crate's
//! dependency-free reader (BF16/F16/F32 all decode), the 12-layer network is
//! built as a plain dataflow graph in the IR — LayerNorm from basis ops,
//! learned positions as `gather(wpe, iota(s))`, the fused qkv weight split
//! host-side, GELU as a tanh composition, weight-tied logits — partitioned
//! by `partition_many` into one multi-kernel schedule, lowered to MSL, and
//! **dispatched on the Apple GPU**. Greedy tokens stream to stdout as each
//! dispatch completes (byte-level BPE decoded, holding back incomplete UTF-8
//! sequences), generation stops at `<|endoftext|>` or when the fixed causal
//! window (`T_GEN`) fills, and the first step's logits are compared against a
//! HuggingFace `transformers` reference (`weights/reference.json`).
//!
//! Weights not checked in; fetch once with:
//! ```text
//! mkdir -p weights && cd weights
//! curl -sLO https://huggingface.co/openai-community/gpt2/resolve/main/model.safetensors
//! curl -sLO https://huggingface.co/openai-community/gpt2/resolve/main/vocab.json
//! ```

use std::collections::HashMap;

use sanic::cost::Device;
#[cfg(target_os = "macos")]
use sanic::metal::{MetalBuf, MetalDevice, MetalGraph, program_dispatches};
use sanic::interp::{Env, Extents, Tensor};
use sanic::ir::*;
use sanic::partition::{Schedule, partition_many};
use sanic::safetensors::{Json, RawTensor, bf16_roundtrip, load, parse_json};

const N_LAYER: usize = 12;
const N_HEAD: usize = 12;
const D: usize = 768;
const DK: usize = 64;
const F: usize = 3072;
const VOCAB: usize = 50257;
const T_GEN: usize = 256; // the fixed causal window: prompt + generation live inside it
const EOS: usize = 50256; // <|endoftext|>
const EPS: f64 = 1e-5;

fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

// ── weights → our conventions ────────────────────────────────────────────────
// HF's Conv1D stores W as [in, out] with y = x·W + b. Our GEMMs contract the
// model axis of a weight laid out [out…, dm], so every projection transposes
// host-side; the fused qkv weight additionally splits into three heads-major
// tensors [h, dk, dm].

fn transpose2(w: &[f64], rows: usize, cols: usize) -> Vec<f64> {
    let mut out = vec![0.0; w.len()];
    for r in 0..rows {
        for c in 0..cols {
            out[c * rows + r] = w[r * cols + c];
        }
    }
    out
}

struct Weights {
    env: Env,
}

fn load_weights(path: &str, bf16: bool) -> Weights {
    let mut raw = load(std::path::Path::new(path)).expect("load safetensors");
    if bf16 {
        for t in raw.values_mut() {
            for v in t.data.iter_mut() {
                *v = bf16_roundtrip(*v);
            }
        }
    }
    let mut take = |name: &str| -> RawTensor {
        raw.remove(name)
            .or_else(|| raw.remove(&format!("transformer.{name}")))
            .unwrap_or_else(|| panic!("missing tensor `{name}`"))
    };

    // stash raw data; axes are attached later, where the graph mints them
    let mut store: HashMap<&'static str, (Vec<usize>, Vec<f64>)> = HashMap::new();
    let wte = take("wte.weight");
    store.insert("wte", (wte.shape.clone(), wte.data));
    let wpe = take("wpe.weight");
    store.insert("wpe", (wpe.shape.clone(), wpe.data));
    for l in 0..N_LAYER {
        let p = |s: &str| format!("h.{l}.{s}");
        for (short, hf) in [
            ("g1", "ln_1.weight"),
            ("c1", "ln_1.bias"),
            ("g2", "ln_2.weight"),
            ("c2", "ln_2.bias"),
        ] {
            let t = take(&p(hf));
            store.insert(leak(format!("{short}_{l}")), (t.shape.clone(), t.data));
        }
        // fused qkv: W [768, 2304] (in, out) → three [h, dk, dm]
        let w = take(&p("attn.c_attn.weight"));
        let b = take(&p("attn.c_attn.bias"));
        for (i, nm) in ["wq", "wk", "wv"].iter().enumerate() {
            let mut wt = vec![0.0; D * D];
            for m in 0..D {
                for o in 0..D {
                    // out index o of block i lives at column i*D + o
                    wt[o * D + m] = w.data[m * 3 * D + i * D + o];
                }
            }
            store.insert(leak(format!("{nm}_{l}")), (vec![N_HEAD, DK, D], wt));
            let bt = b.data[i * D..(i + 1) * D].to_vec();
            store.insert(leak(format!("b{}_{l}", &nm[1..2])), (vec![N_HEAD, DK], bt));
        }
        let w = take(&p("attn.c_proj.weight")); // [dmv(in), dm(out)] → [dm, dmv]
        store.insert(
            leak(format!("wo_{l}")),
            (vec![D, D], transpose2(&w.data, D, D)),
        );
        let b = take(&p("attn.c_proj.bias"));
        store.insert(leak(format!("bo_{l}")), (vec![D], b.data));
        let w = take(&p("mlp.c_fc.weight")); // [dm, f] → [f, dm]
        store.insert(
            leak(format!("wf_{l}")),
            (vec![F, D], transpose2(&w.data, D, F)),
        );
        let b = take(&p("mlp.c_fc.bias"));
        store.insert(leak(format!("bf_{l}")), (vec![F], b.data));
        let w = take(&p("mlp.c_proj.weight")); // [f, dm] → [dm, f]
        store.insert(
            leak(format!("wp_{l}")),
            (vec![D, F], transpose2(&w.data, F, D)),
        );
        let b = take(&p("mlp.c_proj.bias"));
        store.insert(leak(format!("bp_{l}")), (vec![D], b.data));
    }
    let t = take("ln_f.weight");
    store.insert("gf", (t.shape.clone(), t.data));
    let t = take("ln_f.bias");
    store.insert("cf", (t.shape.clone(), t.data));

    Weights {
        env: store
            .into_iter()
            .map(|(k, (_shape, data))| {
                (
                    k,
                    Tensor {
                        axes: Vec::new(), // filled by `bind_axes`
                        shape: Vec::new(),
                        data,
                    },
                )
            })
            .collect(),
    }
}

/// Attach graph axes to a loaded blob (shapes come from the axes' extents).
fn bind_axes(env: &mut Env, name: &str, axes: &[Axis], ext: &Extents) {
    let t = env
        .get_mut(name)
        .unwrap_or_else(|| panic!("no blob {name}"));
    let shape: Vec<usize> = axes.iter().map(|a| ext[a]).collect();
    assert_eq!(
        shape.iter().product::<usize>(),
        t.data.len(),
        "{name}: axes don't match data length"
    );
    t.axes = axes.to_vec();
    t.shape = shape;
}

// ── the graph ────────────────────────────────────────────────────────────────

fn layer_norm(x: Node, g: &'static str, b: &'static str, dm: Axis) -> Node {
    let inv_n = konst(1.0 / D as f64);
    let mean = map(
        MapOp::Mul,
        vec![
            reduce(x.clone(), dm, BinOp::Monoid(Monoid::Add)),
            inv_n.clone(),
        ],
    );
    let xc = map(MapOp::Sub, vec![x, mean]);
    let var = map(
        MapOp::Mul,
        vec![
            reduce(
                map(MapOp::Mul, vec![xc.clone(), xc.clone()]),
                dm,
                BinOp::Monoid(Monoid::Add),
            ),
            inv_n,
        ],
    );
    let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![var, konst(EPS)])]);
    map(
        MapOp::Add,
        vec![
            map(
                MapOp::Mul,
                vec![map(MapOp::Div, vec![xc, denom]), input(g, &[dm])],
            ),
            input(b, &[dm]),
        ],
    )
}

fn gelu_new(h: Node) -> Node {
    let h3 = map(
        MapOp::Mul,
        vec![h.clone(), map(MapOp::Mul, vec![h.clone(), h.clone()])],
    );
    let inner = map(
        MapOp::Mul,
        vec![
            konst(0.7978845608028654), // sqrt(2/π)
            map(
                MapOp::Add,
                vec![h.clone(), map(MapOp::Mul, vec![konst(0.044715), h3])],
            ),
        ],
    );
    map(
        MapOp::Mul,
        vec![
            map(MapOp::Mul, vec![konst(0.5), h]),
            map(MapOp::Add, vec![konst(1.0), map(MapOp::Tanh, vec![inner])]),
        ],
    )
}

struct Gpt2 {
    sched: Schedule,
    ext: Extents,
    env: Env,
    logits_axes: (Axis, Axis), // (s, vocab)
}

/// The whole-window prefill graph: all `t_len` positions at once. Used by
/// `--oracle` to certify the decode path against the f64 interpreter.
fn build_prefill(weights: Weights, label: &str, t_len: usize) -> Gpt2 {
    let mut env = weights.env;
    let (s, dm, vv, pp) = (axis("s"), axis("dm"), axis("v"), axis("p"));
    let mut ext_us: Extents = [(s, t_len), (dm, D), (vv, VOCAB), (pp, 1024)]
        .into_iter()
        .collect();

    bind_axes(&mut env, "wte", &[vv, dm], &ext_us);
    bind_axes(&mut env, "wpe", &[pp, dm], &ext_us);
    bind_axes(&mut env, "gf", &[dm], &ext_us);
    bind_axes(&mut env, "cf", &[dm], &ext_us);

    // embedding: token row + learned position row (positions are data-free —
    // the index IS iota over the sequence axis)
    let tok = gather(input("wte", &[vv, dm]), input("ids", &[s]), vv); // [dm, s]
    let pos = gather(input("wpe", &[pp, dm]), iota(s), pp); // [dm, s]
    let x0 = map(MapOp::Add, vec![tok, pos]);

    let mut roots: Vec<(Node, &'static str)> = vec![(x0.clone(), "xb0")];
    let mut prev_axes = output_axes(&x0);
    for l in 0..N_LAYER {
        let (t, h, dk, dv, dmv, f) = (
            axis("t"),
            axis("h"),
            axis("dk"),
            axis("dv"),
            axis("dmv"),
            axis("f"),
        );
        for (a, n) in [
            (t, t_len),
            (h, N_HEAD),
            (dk, DK),
            (dv, DK),
            (dmv, D),
            (f, F),
        ] {
            ext_us.insert(a, n);
        }
        let nm = |p: &str| leak(format!("{p}_{l}"));
        bind_axes(&mut env, nm("g1"), &[dm], &ext_us);
        bind_axes(&mut env, nm("c1"), &[dm], &ext_us);
        bind_axes(&mut env, nm("g2"), &[dm], &ext_us);
        bind_axes(&mut env, nm("c2"), &[dm], &ext_us);
        bind_axes(&mut env, nm("wq"), &[h, dk, dm], &ext_us);
        bind_axes(&mut env, nm("wk"), &[h, dk, dm], &ext_us);
        bind_axes(&mut env, nm("wv"), &[h, dv, dm], &ext_us);
        bind_axes(&mut env, nm("bq"), &[h, dk], &ext_us);
        bind_axes(&mut env, nm("bk"), &[h, dk], &ext_us);
        bind_axes(&mut env, nm("bv"), &[h, dv], &ext_us);
        bind_axes(&mut env, nm("wo"), &[dm, dmv], &ext_us);
        bind_axes(&mut env, nm("bo"), &[dm], &ext_us);
        bind_axes(&mut env, nm("wf"), &[f, dm], &ext_us);
        bind_axes(&mut env, nm("bf"), &[f], &ext_us);
        bind_axes(&mut env, nm("wp"), &[dm, f], &ext_us);
        bind_axes(&mut env, nm("bp"), &[dm], &ext_us);

        let x = input(leak(format!("xb{l}")), &prev_axes);

        // ── attention ──
        let xn = layer_norm(x.clone(), nm("g1"), nm("c1"), dm);
        let xnt = rename(xn.clone(), s, t);
        let q = map(
            MapOp::Add,
            vec![
                matmul(xn.clone(), input(nm("wq"), &[h, dk, dm]), dm),
                input(nm("bq"), &[h, dk]),
            ],
        );
        let k = map(
            MapOp::Add,
            vec![
                matmul(xnt.clone(), input(nm("wk"), &[h, dk, dm]), dm),
                input(nm("bk"), &[h, dk]),
            ],
        );
        let v = map(
            MapOp::Add,
            vec![
                matmul(xnt, input(nm("wv"), &[h, dv, dm]), dm),
                input(nm("bv"), &[h, dv]),
            ],
        );
        let scores = map(
            MapOp::Mul,
            vec![matmul(q, k, dk), konst(1.0 / (DK as f64).sqrt())],
        );
        let masked = map(MapOp::Add, vec![scores, causal_mask(s, t)]);
        let o = matmul(softmax(masked, t), v, t); // [.., h, dv]
        let flat = flatten(o, &[h, dv], dmv);
        let proj = map(
            MapOp::Add,
            vec![
                matmul(flat, input(nm("wo"), &[dm, dmv]), dmv),
                input(nm("bo"), &[dm]),
            ],
        );
        let x1 = map(MapOp::Add, vec![x, proj]);

        // ── mlp ──
        let xn2 = layer_norm(x1.clone(), nm("g2"), nm("c2"), dm);
        let hpre = map(
            MapOp::Add,
            vec![
                matmul(xn2, input(nm("wf"), &[f, dm]), dm),
                input(nm("bf"), &[f]),
            ],
        );
        let act = gelu_new(hpre);
        let m = map(
            MapOp::Add,
            vec![
                matmul(act, input(nm("wp"), &[dm, f]), f),
                input(nm("bp"), &[dm]),
            ],
        );
        let x2 = map(MapOp::Add, vec![x1, m]);

        prev_axes = output_axes(&x2);
        roots.push((x2, leak(format!("xb{}", l + 1))));
    }

    // final norm + weight-tied logits
    let xf = layer_norm(
        input(leak(format!("xb{N_LAYER}")), &prev_axes),
        "gf",
        "cf",
        dm,
    );
    let logits = matmul(xf, input("wte", &[vv, dm]), dm); // [.., v]
    roots.push((logits, "logits"));

    let ext_f: HashMap<Axis, f64> = ext_us.iter().map(|(&a, &n)| (a, n as f64)).collect();
    let t0 = std::time::Instant::now();
    let sched = partition_many(&roots, &Device::toy(), &ext_f);
    println!(
        "[{label}] partitioned: {} kernels in {:.2}s",
        sched.stages.len(),
        t0.elapsed().as_secs_f32()
    );

    Gpt2 {
        sched,
        ext: ext_us,
        env,
        logits_axes: (s, vv),
    }
}

struct DecodeModel {
    sched: Schedule,
    ext: Extents,
    env: Env,
}

/// The KV-cache decode step: ONE token in (`id`, `pos` as data), the updated
/// caches and the next-token logits out — O(T) work per token instead of the
/// prefill's O(T²) re-run. Per layer, the cache row write is the pure-basis
/// `where(one_hot(t, pos), new, cache)`, attention reads the *updated* cache
/// with positions beyond `pos` masked, and the runtime commits each update by
/// buffer swap after the step — the M6 mechanism, on the real model.
fn build_decode(weights: Weights, label: &str) -> DecodeModel {
    let mut env = weights.env;
    let (dm, vv, pp) = (axis("dm"), axis("v"), axis("p"));
    let mut ext_us: Extents = [(dm, D), (vv, VOCAB), (pp, 1024)].into_iter().collect();
    bind_axes(&mut env, "wte", &[vv, dm], &ext_us);
    bind_axes(&mut env, "wpe", &[pp, dm], &ext_us);
    bind_axes(&mut env, "gf", &[dm], &ext_us);
    bind_axes(&mut env, "cf", &[dm], &ext_us);

    let pos = input("pos", &[]);
    let tok = gather(input("wte", &[vv, dm]), input("id", &[]), vv); // [dm]
    let prow = gather(input("wpe", &[pp, dm]), pos.clone(), pp); // [dm]
    let x0 = map(MapOp::Add, vec![tok, prow]);

    let mut roots: Vec<(Node, &'static str)> = vec![(x0.clone(), "xd0")];
    let mut prev_axes = output_axes(&x0);
    for l in 0..N_LAYER {
        let (t, h, dk, dv, dmv, f) = (
            axis("t"),
            axis("h"),
            axis("dk"),
            axis("dv"),
            axis("dmv"),
            axis("f"),
        );
        for (a, n) in [
            (t, T_GEN),
            (h, N_HEAD),
            (dk, DK),
            (dv, DK),
            (dmv, D),
            (f, F),
        ] {
            ext_us.insert(a, n);
        }
        let nm = |p: &str| leak(format!("{p}_{l}"));
        for w in ["g1", "c1", "g2", "c2", "bo", "bp"] {
            bind_axes(&mut env, nm(w), &[dm], &ext_us);
        }
        bind_axes(&mut env, nm("wq"), &[h, dk, dm], &ext_us);
        bind_axes(&mut env, nm("wk"), &[h, dk, dm], &ext_us);
        bind_axes(&mut env, nm("wv"), &[h, dv, dm], &ext_us);
        bind_axes(&mut env, nm("bq"), &[h, dk], &ext_us);
        bind_axes(&mut env, nm("bk"), &[h, dk], &ext_us);
        bind_axes(&mut env, nm("bv"), &[h, dv], &ext_us);
        bind_axes(&mut env, nm("wo"), &[dm, dmv], &ext_us);
        bind_axes(&mut env, nm("wf"), &[f, dm], &ext_us);
        bind_axes(&mut env, nm("bf"), &[f], &ext_us);
        bind_axes(&mut env, nm("wp"), &[dm, f], &ext_us);

        let x = input(leak(format!("xd{l}")), &prev_axes);

        // ── attention over the updated cache ──
        let xn = layer_norm(x.clone(), nm("g1"), nm("c1"), dm);
        let q = map(
            MapOp::Add,
            vec![
                matmul(xn.clone(), input(nm("wq"), &[h, dk, dm]), dm),
                input(nm("bq"), &[h, dk]),
            ],
        ); // [h, dk]
        let nk = map(
            MapOp::Add,
            vec![
                matmul(xn.clone(), input(nm("wk"), &[h, dk, dm]), dm),
                input(nm("bk"), &[h, dk]),
            ],
        );
        let nv = map(
            MapOp::Add,
            vec![
                matmul(xn, input(nm("wv"), &[h, dv, dm]), dm),
                input(nm("bv"), &[h, dv]),
            ],
        );
        let ck = map(
            MapOp::Where,
            vec![
                one_hot(t, pos.clone()),
                nk,
                input(nm("cache_k"), &[t, h, dk]),
            ],
        ); // [t, h, dk]
        let cv = map(
            MapOp::Where,
            vec![
                one_hot(t, pos.clone()),
                nv,
                input(nm("cache_v"), &[t, h, dv]),
            ],
        );
        roots.push((ck.clone(), leak(format!("ckN_{l}"))));
        roots.push((cv.clone(), leak(format!("cvN_{l}"))));

        let scores = map(
            MapOp::Mul,
            vec![matmul(q, ck, dk), konst(1.0 / (DK as f64).sqrt())],
        ); // [h, t]
        let future = map(MapOp::Lt, vec![pos.clone(), iota(t)]);
        let masked = map(
            MapOp::Add,
            vec![
                scores,
                map(MapOp::Where, vec![future, konst(-1e30), konst(0.0)]),
            ],
        );
        let o = matmul(softmax(masked, t), cv, t); // [h, dv]
        let flat = flatten(o, &[h, dv], dmv);
        let proj = map(
            MapOp::Add,
            vec![
                matmul(flat, input(nm("wo"), &[dm, dmv]), dmv),
                input(nm("bo"), &[dm]),
            ],
        );
        let x1 = map(MapOp::Add, vec![x, proj]);

        // ── mlp ──
        let xn2 = layer_norm(x1.clone(), nm("g2"), nm("c2"), dm);
        let hpre = map(
            MapOp::Add,
            vec![
                matmul(xn2, input(nm("wf"), &[f, dm]), dm),
                input(nm("bf"), &[f]),
            ],
        );
        let act = gelu_new(hpre);
        let m = map(
            MapOp::Add,
            vec![
                matmul(act, input(nm("wp"), &[dm, f]), f),
                input(nm("bp"), &[dm]),
            ],
        );
        let x2 = map(MapOp::Add, vec![x1, m]);

        prev_axes = output_axes(&x2);
        roots.push((x2, leak(format!("xd{}", l + 1))));
    }

    let xf = layer_norm(
        input(leak(format!("xd{N_LAYER}")), &prev_axes),
        "gf",
        "cf",
        dm,
    );
    let logits = matmul(xf, input("wte", &[vv, dm]), dm); // [v]
    roots.push((logits, "logits"));

    let ext_f: HashMap<Axis, f64> = ext_us.iter().map(|(&a, &n)| (a, n as f64)).collect();
    let t0 = std::time::Instant::now();
    let sched = partition_many(&roots, &Device::toy(), &ext_f);
    println!(
        "[{label}] decode step partitioned: {} kernels in {:.2}s (cache window {T_GEN})",
        sched.stages.len(),
        t0.elapsed().as_secs_f32()
    );

    DecodeModel {
        sched,
        ext: ext_us,
        env,
    }
}

// ── BPE decode (vocab.json + the GPT-2 byte↔unicode table) ───────────────────

/// Token id → the raw BYTES it stands for. Byte-level BPE means a token can
/// end mid-UTF-8-sequence, so streaming must buffer bytes, not strings.
fn load_vocab(path: &str) -> HashMap<usize, Vec<u8>> {
    let src = std::fs::read_to_string(path).expect("read vocab.json");
    let Json::Obj(kvs) = parse_json(&src).expect("parse vocab.json") else {
        panic!("vocab.json is not an object")
    };
    // GPT-2's byte-level BPE prints bytes through a unicode remap; invert it.
    let mut inv: HashMap<u32, u8> = HashMap::new();
    let mut n = 0u32;
    for b in 0u32..256 {
        let printable =
            (33..=126).contains(&b) || (161..=172).contains(&b) || (174..=255).contains(&b);
        if printable {
            inv.insert(b, b as u8);
        } else {
            inv.insert(256 + n, b as u8);
            n += 1;
        }
    }
    kvs.into_iter()
        .map(|(tok, id)| {
            let bytes: Vec<u8> = tok.chars().map(|c| inv[&(c as u32)]).collect();
            (id.as_num().unwrap() as usize, bytes)
        })
        .collect()
}

/// Streams decoded tokens to stdout as they arrive, printing only complete
/// UTF-8 characters and holding partial sequences back until the next token
/// finishes them.
struct StreamPrinter {
    pending: Vec<u8>,
}

impl StreamPrinter {
    fn new() -> Self {
        StreamPrinter { pending: Vec::new() }
    }

    fn push(&mut self, bytes: &[u8]) {
        use std::io::Write;
        self.pending.extend_from_slice(bytes);
        // print the longest valid UTF-8 prefix, keep the tail for later
        let valid = match std::str::from_utf8(&self.pending) {
            Ok(s) => s.len(),
            Err(e) => e.valid_up_to(),
        };
        if valid > 0 {
            let out = std::io::stdout();
            let mut lock = out.lock();
            lock.write_all(&self.pending[..valid]).ok();
            lock.flush().ok();
            self.pending.drain(..valid);
        }
    }

    fn finish(&mut self) {
        use std::io::Write;
        if !self.pending.is_empty() {
            // a dangling partial sequence at the very end — replacement char
            print!("\u{FFFD}");
            self.pending.clear();
        }
        println!();
        std::io::stdout().flush().ok();
    }
}

// ── the GPU host (macOS) ─────────────────────────────────────────────────────


// ── main ─────────────────────────────────────────────────────────────────────

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("the gpt2 example dispatches on Metal — run it on macOS");
}

#[cfg(target_os = "macos")]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let oracle = args.iter().any(|a| a == "--oracle");
    let with_bf16 = args.iter().any(|a| a == "--bf16");
    let max_new: usize = args
        .iter()
        .position(|a| a == "--n")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(usize::MAX);
    let wdir = "weights";
    let st_path = format!("{wdir}/model.safetensors");
    if !std::path::Path::new(&st_path).exists() {
        eprintln!("weights not found — fetch them first (see the module doc)");
        return;
    }

    // the HF reference, if generated (weights/reference.py)
    let reference = std::fs::read_to_string(format!("{wdir}/reference.json"))
        .ok()
        .map(|s| parse_json(&s).expect("parse reference.json"));
    // --prompt "text": encode with the in-tree byte-level BPE (the reference
    // comparison only applies to the default prompt, so it is skipped)
    let custom_prompt = args
        .iter()
        .position(|a| a == "--prompt")
        .and_then(|i| args.get(i + 1).cloned());
    let reference = if custom_prompt.is_some() { None } else { reference };
    let prompt_ids: Vec<usize> = match &custom_prompt {
        Some(text) => {
            let bpe = sanic::bpe::Bpe::from_gpt2(
                &format!("{wdir}/vocab.json"),
                &format!("{wdir}/merges.txt"),
            )
            .expect("load BPE (vocab.json + merges.txt)");
            bpe.encode(text).into_iter().map(|i| i as usize).collect()
        }
        None => reference
            .as_ref()
            .and_then(|r| r.get("ids"))
            .and_then(Json::as_arr)
            .map(|a| a.iter().map(|v| v.as_num().unwrap() as usize).collect())
            .unwrap_or_else(|| vec![15496, 11, 314, 1101, 257, 3303, 2746, 11]),
    };
    let vocab = load_vocab(&format!("{wdir}/vocab.json"));

    let mut variants: Vec<(&str, bool)> = vec![("f32", false)];
    if with_bf16 {
        variants.push(("bf16", true));
    }
    for (label, bf16) in variants {
        let t0 = std::time::Instant::now();
        let w = load_weights(&st_path, bf16);
        println!(
            "[{label}] loaded 124M params in {:.2}s",
            t0.elapsed().as_secs_f32()
        );
        let model = build_decode(w, label);

        let t0 = std::time::Instant::now();
        let program =
            sanic::emit_metal::emit_schedule_metal_on(&Device::m1_pro(), &model.sched, &model.ext);
        let Some(g) = MetalDevice::open() else {
            eprintln!("no Metal device — skipping");
            return;
        };
        let pipes = g.compile(&program.msl);
        println!(
            "[{label}] MSL compiled ({} kernels) in {:.2}s",
            program.stages.len(),
            t0.elapsed().as_secs_f32()
        );

        // ZERO COPY: unified memory means the checkpoint's bytes can BE the
        // device buffer. The file wraps whole (page-aligned, leaked for the
        // model's lifetime) and any tensor stored f32 and used UNTRANSPOSED
        // binds at its file offset — wte (154 MB, doubling as the tied
        // logits head) and wpe. The rest are transposed or split host-side
        // and still copy; bf16 mode round-trips values, so it never
        // zero-copies.
        let zc = if bf16 {
            None
        } else {
            sanic::safetensors::StFile::open_zero_copy(std::path::Path::new(&st_path))
                .ok()
                .and_then(|(st, region)| Some((st, g.from_bytes_nocopy(region)?)))
        };

        // buffers: weights uploaded once; `id`/`pos` rewritten per step;
        // caches allocated zeroed and PERSISTED across steps by swap-commit
        let mut bufs: HashMap<String, MetalBuf> = HashMap::new();
        let mut zc_bytes = 0usize;
        for (name, axes) in &program.inputs {
            let size: usize = axes.iter().map(|a| model.ext[a]).product::<usize>().max(1);
            if *name == "id" || *name == "pos" || name.starts_with("cache_") {
                bufs.insert(name.to_string(), g.alloc_f32(size));
                continue;
            }
            let src = match *name {
                "wte" => Some("wte.weight"),
                "wpe" => Some("wpe.weight"),
                _ => None,
            };
            if let (Some((st, fb)), Some(src)) = (&zc, src)
                && st.meta(src).0 == "F32"
                && st.file_range(src).0 % 4 == 0
            {
                let (a, b) = st.file_range(src);
                zc_bytes += b - a;
                bufs.insert(name.to_string(), fb.slice(a));
                continue;
            }
            let t = model.env[name].permuted_to(axes);
            let b = g.alloc_f32(t.data.len());
            g.write_f64(&b, &t.data);
            bufs.insert(name.to_string(), b);
        }
        if zc_bytes > 0 {
            println!(
                "[{label}] {:.0} MB bound zero-copy from the checkpoint (no upload)",
                zc_bytes as f64 / 1e6
            );
        }
        for (n, size) in &program.buffers {
            bufs.insert(n.clone(), g.alloc_f32(*size));
        }

        // one decode step: feed (id, pos), dispatch, commit the cache updates
        // by swapping buffers (the Session discipline, on device), return the
        // next-token logits
        let graphs: std::cell::RefCell<[Option<MetalGraph>; 2]> = Default::default();
        let step = |bufs: &mut HashMap<String, MetalBuf>, id: usize, pos: usize| {
            g.write_f64(&bufs["id"], &[id as f64]);
            g.write_f64(&bufs["pos"], &[pos as f64]);
            // graph replay: swap commits flip bindings with period two, so
            // one captured graph per step parity covers every step — encode
            // twice, then it's one executeCommandsInBuffer per token
            let mut gr = graphs.borrow_mut();
            let slot = &mut gr[pos % 2];
            if slot.is_none() {
                *slot = Some(g.capture(&program_dispatches(&program, bufs, &pipes)));
            }
            g.run_graph(slot.as_ref().unwrap());
            for l in 0..N_LAYER {
                for (upd, cache) in [
                    (format!("ckN_{l}"), format!("cache_k_{l}")),
                    (format!("cvN_{l}"), format!("cache_v_{l}")),
                ] {
                    let new = bufs[&upd].clone();
                    let old = bufs.insert(cache, new).unwrap();
                    bufs.insert(upd, old);
                }
            }
        };

        // ── debug: one step, then scan every stage output for non-finites ──
        if args.iter().any(|a| a == "--debug") {
            step(&mut bufs, prompt_ids[0], 0);
            for st in &program.stages {
                let out = g.read_f32(&bufs[&st.output], st.grid_size);
                let bad = out.iter().filter(|v| !v.is_finite()).count();
                let mx = out.iter().cloned().fold(0.0f32, |a, b| a.max(b.abs()));
                if bad > 0 {
                    println!(
                        "[debug] FIRST NON-FINITE at kernel `{}` → `{}` ({bad}/{} bad, max|v|={mx:e})",
                        st.kernel, st.output, st.grid_size
                    );
                    break;
                } else {
                    println!(
                        "[debug] `{}` → `{}` ok (max|v|={mx:.3e})",
                        st.kernel, st.output
                    );
                }
            }
            return;
        }

        // ── prompt feed, then STREAMING greedy generation ──
        let mut stream = StreamPrinter::new();
        for &id in &prompt_ids {
            stream.push(&vocab[&id]);
        }
        let t0 = std::time::Instant::now();
        for (p, &tid) in prompt_ids.iter().enumerate() {
            step(&mut bufs, tid, p);
        }
        let prompt_logits = g.read_f32(&bufs["logits"], VOCAB);
        let prompt_s = t0.elapsed().as_secs_f32();

        let mut ids: Vec<usize> = prompt_ids.clone();
        let mut cur = prompt_logits.clone();
        let t0 = std::time::Instant::now();
        loop {
            let next = (0..VOCAB)
                .max_by(|&a, &b| cur[a].total_cmp(&cur[b]))
                .unwrap();
            ids.push(next);
            if next == EOS {
                stream.push(b"<|endoftext|>");
                break;
            }
            stream.push(&vocab[&next]);
            if ids.len() >= T_GEN || ids.len() - prompt_ids.len() >= max_new {
                break;
            }
            step(&mut bufs, next, ids.len() - 1);
            cur = g.read_f32(&bufs["logits"], VOCAB);
        }
        stream.finish();
        let new_tokens = ids.len() - prompt_ids.len();
        println!(
            "[{label}] prompt {} tok in {prompt_s:.2}s; generated {new_tokens} tok in {:.2}s \
             ({:.0} ms/token, {:.1} tok/s)",
            prompt_ids.len(),
            t0.elapsed().as_secs_f32(),
            t0.elapsed().as_secs_f32() * 1000.0 / new_tokens.max(1) as f32,
            new_tokens as f32 / t0.elapsed().as_secs_f32().max(1e-9)
        );

        // ── the logit match against HuggingFace (at the prompt's last position) ──
        if let Some(r) = &reference {
            let ref_last: Vec<f64> = r
                .get("logits_last")
                .and_then(Json::as_arr)
                .unwrap()
                .iter()
                .map(|v| v.as_num().unwrap())
                .collect();
            let mut max_abs = 0.0f64;
            for (tok, &rv) in ref_last.iter().enumerate() {
                max_abs = max_abs.max((prompt_logits[tok] as f64 - rv).abs());
            }
            let our_argmax = (0..VOCAB)
                .max_by(|&a, &b| prompt_logits[a].total_cmp(&prompt_logits[b]))
                .unwrap();
            let hf_argmax = ref_last
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.total_cmp(b.1))
                .unwrap()
                .0;
            let ref_greedy: Vec<usize> = r
                .get("greedy")
                .and_then(Json::as_arr)
                .unwrap()
                .iter()
                .map(|v| v.as_num().unwrap() as usize)
                .collect();
            println!(
                "[{label}] vs HF: max |Δlogit| over the full last row = {max_abs:.4}; \
                 argmax {our_argmax} vs HF {hf_argmax} — {}",
                if our_argmax == hf_argmax { "MATCH" } else { "MISMATCH" }
            );
            let n = ids.len().min(ref_greedy.len());
            println!(
                "[{label}] greedy tokens vs HF: {}/{} match{}",
                ids[..n]
                    .iter()
                    .zip(&ref_greedy[..n])
                    .filter(|(a, b)| a == b)
                    .count(),
                n,
                if ids[..n] == ref_greedy[..n] {
                    " — SEQUENCE MATCH"
                } else {
                    ""
                }
            );
        }

        // ── the f64 oracle: PREFILL through the interpreter vs DECODE on the
        // GPU — the incremental-decode-equals-prefill theorem, on GPT-2 ──
        if oracle && !bf16 {
            println!(
                "[oracle] prefill@{} through the f64 interpreter (the naive semantics — minutes)…",
                prompt_ids.len()
            );
            let t0 = std::time::Instant::now();
            let pre = build_prefill(load_weights(&st_path, false), "oracle", prompt_ids.len());
            let mut env = pre.env.clone();
            env.insert(
                "ids",
                Tensor {
                    axes: vec![pre.logits_axes.0],
                    shape: vec![prompt_ids.len()],
                    data: prompt_ids.iter().map(|&i| i as f64).collect(),
                },
            );
            pre.sched.execute_env(&mut env, &pre.ext);
            let interp_logits = env["logits"].clone();
            let mut worst = 0.0f64;
            let last = prompt_ids.len() - 1;
            for tok in 0..VOCAB {
                let c: HashMap<Axis, usize> =
                    [(pre.logits_axes.0, last), (pre.logits_axes.1, tok)]
                        .into_iter()
                        .collect();
                worst = worst.max((prompt_logits[tok] as f64 - interp_logits.at(&c)).abs());
            }
            println!(
                "[oracle] done in {:.1}s; GPU decode-with-KV-cache vs f64 interpreter prefill: \
                 max |Δ| = {worst:.4}",
                t0.elapsed().as_secs_f32()
            );
        }
        println!();
    }
}
