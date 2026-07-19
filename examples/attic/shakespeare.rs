//! nanoGPT on tinyshakespeare — a small char-level transformer trained from
//! scratch, end to end on the Apple GPU. The transformer sibling of
//! `beautiful_mnist`: the forward net, the next-token cross-entropy and — via
//! `grad` — the whole backward pass are ONE dataflow graph; the SGD update
//! `w ← w − lr·∇w` rides each gradient's fold as an epilogue, so one
//! partitioned schedule computes the loss and every new weight together. It
//! lowers to MSL and dispatches on Metal, updating each weight IN PLACE —
//! `partition` orders every read of a weight before its update, so the new
//! value overwrites the old with no shadow buffer (half the weight VRAM).
//!
//! ```text
//! cargo run --release --example shakespeare
//! cargo run --release --example shakespeare -- --steps 4000 --lr 0.05 --batch 32
//! ```
//!
//! Data (not checked in) — fetch once:
//! ```text
//! mkdir -p data && curl -sL -o data/shakespeare.txt \
//!   https://raw.githubusercontent.com/karpathy/char-rnn/master/data/tinyshakespeare/input.txt
//! ```

use sanic::cost::Device;
use sanic::grad::grad;
use sanic::interp::Value;
use sanic::ir::*;
use sanic::partition::partition_many;

const D: usize = 128; // embedding / single-head attention dim
const F: usize = 512; // MLP hidden
const S: usize = 64; // context length
const EPS: f64 = 1e-5;

fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

// ── graph pieces (RMSNorm and GELU from the scalar basis) ────────────────────

/// RMSNorm: `x / sqrt(mean(x²) + ε) · g` — no mean subtraction (what LLaMA and
/// modern GPTs use). `grad`'s finite-difference suite covers this norm.
fn rms_norm(x: Node, g: &'static str, dm: Axis) -> Node {
    let inv_n = konst(1.0 / D as f64);
    let ms = map(MapOp::Mul, vec![reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), dm, add_r()), inv_n]);
    let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![ms, konst(EPS)])]);
    map(MapOp::Mul, vec![map(MapOp::Div, vec![x, denom]), input(g, &[dm], Dtype::F32)])
}

/// GELU (tanh approximation), the activation nanoGPT uses.
fn gelu(h: Node) -> Node {
    let h3 = map(MapOp::Mul, vec![h.clone(), map(MapOp::Mul, vec![h.clone(), h.clone()])]);
    let inner = map(
        MapOp::Mul,
        vec![konst(0.7978845608028654), map(MapOp::Add, vec![h.clone(), map(MapOp::Mul, vec![konst(0.044715), h3])])],
    );
    map(
        MapOp::Mul,
        vec![map(MapOp::Mul, vec![konst(0.5), h]), map(MapOp::Add, vec![konst(1.0), map(MapOp::Tanh, vec![inner])])],
    )
}

fn add_r() -> BinOp {
    BinOp::Monoid(Monoid::Add)
}

/// The trainable weights of the model, in the order they are built: `(name, axes)`.
fn weight_list(dm: Axis, vv: Axis, pp: Axis, per_layer: &[[Axis; 4]]) -> Vec<(&'static str, Vec<Axis>)> {
    let mut w = vec![("wte", vec![vv, dm]), ("wpe", vec![pp, dm]), ("gf", vec![dm])];
    for (l, [dk, dv, _t, f]) in per_layer.iter().enumerate() {
        let nm = |p: &str| leak(format!("{p}_{l}"));
        w.push((nm("g1"), vec![dm]));
        w.push((nm("Wq"), vec![*dk, dm]));
        w.push((nm("Wk"), vec![*dk, dm]));
        w.push((nm("Wv"), vec![*dv, dm]));
        w.push((nm("Wo"), vec![dm, *dv]));
        w.push((nm("g2"), vec![dm]));
        w.push((nm("Wf"), vec![*f, dm]));
        w.push((nm("Wp"), vec![dm, *f]));
    }
    w
}

// ── data: char-level tokenizer over tinyshakespeare ──────────────────────────

struct Vocab {
    itos: Vec<char>,
    stoi: std::collections::HashMap<char, usize>,
}

fn load_data() -> (Vec<usize>, Vocab) {
    let text = std::fs::read_to_string("data/shakespeare.txt").expect("read data/shakespeare.txt");
    let mut chars: Vec<char> = text.chars().collect::<std::collections::BTreeSet<_>>().into_iter().collect();
    chars.sort_unstable();
    let stoi: std::collections::HashMap<char, usize> = chars.iter().enumerate().map(|(i, &c)| (c, i)).collect();
    let ids = text.chars().map(|c| stoi[&c]).collect();
    (ids, Vocab { itos: chars, stoi })
}

// ── a tiny PRNG for sampling + init ──────────────────────────────────────────

struct Rng(u64);
impl Rng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next_u64() >> 11) as usize % n
    }
    /// Approximately-normal sample (sum of uniforms), for weight init.
    fn normal(&mut self, std: f64) -> f64 {
        ((0..6).map(|_| self.unit()).sum::<f64>() - 3.0) * std / 1.732_050_8
    }
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("the shakespeare example dispatches on Metal — run it on macOS");
}

#[cfg(target_os = "macos")]
fn main() {
    use sanic::metal::{MetalBuf, MetalDevice, program_dispatches};
    use std::collections::HashMap;

    let args: Vec<String> = std::env::args().collect();
    let argf = |flag: &str, def: f64| -> f64 {
        args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).and_then(|v| v.parse().ok()).unwrap_or(def)
    };
    let lr = argf("--lr", 0.05);
    let steps = argf("--steps", 4000.0) as usize;
    let n_layer = argf("--layers", 2.0) as usize; // ≥3 exercises the bindless path
    let batch = argf("--batch", 32.0) as usize; // seqs/step — amortizes weight loads, lifts MFU

    if !std::path::Path::new("data/shakespeare.txt").exists() {
        eprintln!("tinyshakespeare not found — fetch it first (see the module doc)");
        return;
    }
    let (data, vocab) = load_data();
    let vsize = vocab.itos.len();
    println!("tinyshakespeare: {} chars, vocab {vsize} — {n_layer}L {D}d {S}ctx ×{batch}b", data.len());

    // ── axes ──
    let (s, dm, vv, pp) = (axis("s", S), axis("dm", D), axis("v", vsize), axis("p", S));
    // batch is one more parallel axis threaded through every activation; weights
    // carry no `b`, so they stay shared and their gradients sum over the batch.
    let bb = axis("b", batch);
    let bs = axis("bs", batch * S); // batch·seq flattened — the embedding gather wants a 1-axis index
    let per_layer: Vec<[Axis; 4]> =
        (0..n_layer).map(|_| [axis("dk", D), axis("dv", D), axis("t", S), axis("f", F)]).collect();

    // ── forward graph: embed → blocks → final norm → tied logits ──
    // ids is a flat [b·s] index (single-axis, as the gather's scatter-add
    // backward requires); split the gathered rows back to [b, s].
    let tok_flat = gather(input("wte", &[vv, dm], Dtype::F32), input("ids", &[bs], Dtype::F32), vv); // [dm, bs]
    let tok = split(tok_flat, bs, bb, s); // [dm, b, s]
    let pos = gather(input("wpe", &[pp, dm], Dtype::F32), iota(s), pp); // [dm, s] — shared across the batch
    let mut x = map(MapOp::Add, vec![tok, pos]);
    for (l, [dk, dv, t, f]) in per_layer.iter().enumerate() {
        let (dk, dv, t, f) = (*dk, *dv, *t, *f);
        let nm = |p: &str| leak(format!("{p}_{l}"));
        // attention
        let xn = rms_norm(x.clone(), nm("g1"), dm);
        let xnt = rename(xn.clone(), s, t);
        let q = matmul(xn, input(nm("Wq"), &[dk, dm], Dtype::F32), dm); // [dk, s]
        let k = matmul(xnt.clone(), input(nm("Wk"), &[dk, dm], Dtype::F32), dm); // [dk, t]
        let v = matmul(xnt, input(nm("Wv"), &[dv, dm], Dtype::F32), dm); // [dv, t]
        let scores = map(MapOp::Mul, vec![matmul(q, k, dk), konst(1.0 / (D as f64).sqrt())]);
        let masked = map(MapOp::Add, vec![scores, causal_mask(s, t)]);
        let o = matmul(softmax(masked, t), v, t); // [dv, s]
        let proj = matmul(o, input(nm("Wo"), &[dm, dv], Dtype::F32), dv); // [dm, s]
        let x1 = map(MapOp::Add, vec![x, proj]);
        // mlp
        let xn2 = rms_norm(x1.clone(), nm("g2"), dm);
        let hpre = matmul(xn2, input(nm("Wf"), &[f, dm], Dtype::F32), dm); // [f, s]
        let m = matmul(gelu(hpre), input(nm("Wp"), &[dm, f], Dtype::F32), f); // [dm, s]
        x = map(MapOp::Add, vec![x1, m]);
    }
    let xf = rms_norm(x, "gf", dm);
    let logits = matmul(xf, input("wte", &[vv, dm], Dtype::F32), dm); // [s, v]  (weight-tied)

    // ── next-token cross-entropy, mean over positions (composed logsumexp) ──
    let mmax = reduce(logits.clone(), vv, BinOp::Monoid(Monoid::Max));
    let sh = map(MapOp::Exp, vec![map(MapOp::Sub, vec![logits.clone(), mmax.clone()])]);
    let lse = map(MapOp::Add, vec![mmax, map(MapOp::Log, vec![reduce(sh, vv, add_r())])]); // [s]
    let picked = reduce(map(MapOp::Mul, vec![logits.clone(), one_hot(vv, input("tgt", &[bb, s], Dtype::F32))]), vv, add_r());
    // mean over every token in the batch: fold positions AND batch, scale by
    // 1/(B·S). Keeping it a mean (not a sum) holds the gradient scale — and thus
    // `lr` — invariant to batch size, and the scalar keeps `grad` happy.
    let per_tok = map(MapOp::Sub, vec![lse, picked]); // [b, s]
    let summed = reduce(reduce(per_tok, s, add_r()), bb, add_r()); // scalar
    let loss = map(MapOp::Mul, vec![summed, konst(1.0 / (S * batch) as f64)]);

    // ── one schedule: logits, loss, and every fused SGD update ──
    let params = weight_list(dm, vv, pp, &per_layer);
    let names: Vec<&'static str> = params.iter().map(|(n, _)| *n).collect();
    let grads = grad(&loss, &names);
    let mut roots: Vec<(Node, &'static str)> = vec![(logits.clone(), "logits"), (loss.clone(), "loss")];
    for (name, axes) in &params {
        // the update is named after the weight itself — it writes `w` in place
        // (partition orders it after every reader), so no shadow buffer.
        let upd = map(MapOp::Sub, vec![input(name, axes, Dtype::F32), map(MapOp::Mul, vec![konst(lr), grads[name].clone()])]);
        roots.push((upd, name));
    }
    // training-time algebraic simplification: the winner-mask cancels and the
    // softmax backward reuses the forward's log-sum-exp (composed-LSE parity).
    let raw: Vec<Node> = roots.iter().map(|(n, _)| n.clone()).collect();
    let roots: Vec<(Node, &'static str)> =
        sanic::simplify::simplify_many(&raw).into_iter().zip(&roots).map(|(n, (_, nm))| (n, *nm)).collect();

    let sched = partition_many(&roots, &Device::toy());
    let program = sanic::emit_metal::emit_schedule_metal_on(&Device::m1_pro(), &sched);
    let bindless = program.stages.iter().filter(|s| s.argbuf.is_some()).count();
    println!("training step: {} kernels ({bindless} bindless)", program.stages.len());

    let Some(dev) = MetalDevice::open() else {
        eprintln!("no Metal device — skipping");
        return;
    };
    let pipes = dev.compile(&program.msl);

    // ── buffers: weights init + uploaded, ids/tgt per step, rest scratch ──
    let mut rng = Rng(0xA5A5_1234_5678);
    let mut bufs: HashMap<String, MetalBuf> = HashMap::new();
    for (name, axes) in &program.inputs {
        let size = axes.iter().map(|a| a.extent).product::<usize>().max(1);
        if *name == "ids" || *name == "tgt" {
            bufs.insert(name.to_string(), dev.alloc_f32(size));
            continue;
        }
        // weight init: N(0, 0.02) for the matmuls, 1 for the norm gains.
        let init = if name.starts_with('g') {
            Value::from_fn(axes, |_| 1.0)
        } else {
            Value::from_fn(axes, |_| rng.normal(0.02))
        };
        bufs.insert(name.to_string(), dev.from_f64(&init.data));
    }
    for (name, size) in &program.buffers {
        bufs.insert(name.clone(), dev.alloc_f32(*size));
    }

    // One compute encoder per dispatch, all in one command buffer per step —
    // NOT the captured-graph replay (`capture`/`run_graph`) the decode examples
    // use. That path replays a frozen indirect command buffer, and on Apple7/8 a
    // reused ICB whose bound weights are mutated IN PLACE across replays
    // accumulates stale GPU state and corrupts (tinygrad avoids this by updating
    // weights functionally, not in place). The per-dispatch encoders re-bind
    // every step, so in-place training is correct — and batching, not the graph,
    // is what lifts MFU here.
    let run_window = |bufs: &HashMap<String, MetalBuf>, ids: &[f64], tgt: &[f64]| {
        dev.write_f64(&bufs["ids"], ids);
        dev.write_f64(&bufs["tgt"], tgt);
        dev.run(&program_dispatches(&program, bufs, &pipes));
    };

    // ── sampling: temperature-softmax autoregressive generation from a seed ──
    // logits are [b, s, v] row-major; we drive batch lane 0 and read its block,
    // where position p's row is logits[p*vsize .. ] (lane 0 starts at offset 0).
    let mut srng = Rng(0x5EED_C0DE);
    let temp = 0.8f32;
    let mut sample = |bufs: &HashMap<String, MetalBuf>, seed: &str, n: usize| -> String {
        let mut ctx: Vec<usize> = seed.chars().map(|c| vocab.stoi[&c]).collect();
        let mut out = String::from(seed);
        for _ in 0..n {
            // feed the window as batch lane 0; the other B−1 lanes idle at zero.
            let mut win = vec![0f64; batch * S];
            let start = ctx.len().saturating_sub(S);
            let last = ctx.len() - start; // number of real tokens in the window
            for (i, &id) in ctx[start..].iter().enumerate() {
                win[i] = id as f64;
            }
            run_window(bufs, &win, &vec![0f64; batch * S]);
            let lg = dev.read_f32(&bufs["logits"], S * vsize); // lane 0's [s, v] block
            let row = &lg[(last - 1) * vsize..last * vsize];
            let mx = row.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let ps: Vec<f32> = row.iter().map(|z| ((z - mx) / temp).exp()).collect();
            let mut r = srng.unit() as f32 * ps.iter().sum::<f32>();
            let mut next = vsize - 1;
            for (i, &p) in ps.iter().enumerate() {
                r -= p;
                if r <= 0.0 {
                    next = i;
                    break;
                }
            }
            ctx.push(next);
            out.push(vocab.itos[next]);
        }
        out
    };

    // ── model FLOPs / step, for MFU (PaLM formula): 6N counts every weight
    // matmul forward+backward, 12·L·H·Q·T the attention score/value matmuls
    // (H=1 head, Q=D head dim, T=S). Per token that is 6N + 12·L·D·S; one step
    // is B seqs of S tokens, so B·S tokens.
    let n_params: usize = params.iter().map(|(_, ax)| ax.iter().map(|a| a.extent).product::<usize>()).sum();
    let flops_per_step = ((6 * n_params + 12 * n_layer * D * S) * S * batch) as f64;
    let peak_flops = Device::m1_pro().peak_flops;

    // ── training loop: SGD on next-token prediction ──
    println!("training {steps} steps (lr {lr}) — {n_params} params, {:.0} MFLOP/step…", flops_per_step / 1e6);
    let t0 = std::time::Instant::now();
    let mut ids = vec![0f64; batch * S];
    let mut tgt = vec![0f64; batch * S];
    for step in 0..steps {
        // B independent windows, laid out [b, s] row-major (b outer, s inner).
        for b in 0..batch {
            let i = rng.below(data.len() - S - 1);
            for j in 0..S {
                ids[b * S + j] = data[i + j] as f64;
                tgt[b * S + j] = data[i + j + 1] as f64;
            }
        }
        run_window(&bufs, &ids, &tgt); // updates every weight in place
        if step < 8 || step % 250 == 0 || step == steps - 1 {
            let loss = dev.read_f32(&bufs["loss"], 1)[0];
            let sps = (step + 1) as f64 / t0.elapsed().as_secs_f64();
            let mfu = 100.0 * flops_per_step * sps / peak_flops;
            println!("  step {step:>5}  loss {loss:.3}  ({sps:.0} steps/s, {mfu:.2}% MFU)");
        }
    }

    println!("\n── sample ──");
    println!("{}", sample(&bufs, "\n", 400));
}
