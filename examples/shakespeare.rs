//! nanoGPT on tinyshakespeare — a small char-level transformer trained from
//! scratch, end to end on the Apple GPU. The transformer sibling of
//! `beautiful_mnist`: the forward net, the next-token cross-entropy and — via
//! `grad` — the whole backward pass are ONE dataflow graph; the SGD update
//! `w ← w − lr·∇w` rides each gradient's fold as an epilogue, so one
//! partitioned schedule computes the loss and every new weight together. It
//! lowers to MSL, dispatches on Metal, and commits weights by buffer swap
//! between steps (the KV-cache discipline, on optimizer state).
//!
//! ```text
//! cargo run --release --example shakespeare
//! cargo run --release --example shakespeare -- --steps 4000 --lr 0.05
//! ```
//!
//! Data (not checked in) — fetch once:
//! ```text
//! mkdir -p data && curl -sL -o data/shakespeare.txt \
//!   https://raw.githubusercontent.com/karpathy/char-rnn/master/data/tinyshakespeare/input.txt
//! ```

use sanic::cost::Device;
use sanic::grad::grad;
use sanic::interp::{Extents, Tensor};
use sanic::ir::*;
use sanic::partition::partition_many;

const N_LAYER: usize = 1;
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
    map(MapOp::Mul, vec![map(MapOp::Div, vec![x, denom]), input(g, &[dm])])
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

    if !std::path::Path::new("data/shakespeare.txt").exists() {
        eprintln!("tinyshakespeare not found — fetch it first (see the module doc)");
        return;
    }
    let (data, vocab) = load_data();
    let vsize = vocab.itos.len();
    println!("tinyshakespeare: {} chars, vocab {vsize} — {N_LAYER}L {D}d {S}ctx", data.len());

    // ── axes ──
    let (s, dm, vv, pp) = (axis("s"), axis("dm"), axis("v"), axis("p"));
    let mut ext: Extents = [(s, S), (dm, D), (vv, vsize), (pp, S)].into_iter().collect();
    let per_layer: Vec<[Axis; 4]> = (0..N_LAYER).map(|_| [axis("dk"), axis("dv"), axis("t"), axis("f")]).collect();
    for [dk, dv, t, f] in &per_layer {
        ext.insert(*dk, D);
        ext.insert(*dv, D);
        ext.insert(*t, S);
        ext.insert(*f, F);
    }

    // ── forward graph: embed → blocks → final norm → tied logits ──
    let tok = gather(input("wte", &[vv, dm]), input("ids", &[s]), vv); // [dm, s]
    let pos = gather(input("wpe", &[pp, dm]), iota(s), pp); // [dm, s]
    let mut x = map(MapOp::Add, vec![tok, pos]);
    for (l, [dk, dv, t, f]) in per_layer.iter().enumerate() {
        let (dk, dv, t, f) = (*dk, *dv, *t, *f);
        let nm = |p: &str| leak(format!("{p}_{l}"));
        // attention
        let xn = rms_norm(x.clone(), nm("g1"), dm);
        let xnt = rename(xn.clone(), s, t);
        let q = matmul(xn, input(nm("Wq"), &[dk, dm]), dm); // [dk, s]
        let k = matmul(xnt.clone(), input(nm("Wk"), &[dk, dm]), dm); // [dk, t]
        let v = matmul(xnt, input(nm("Wv"), &[dv, dm]), dm); // [dv, t]
        let scores = map(MapOp::Mul, vec![matmul(q, k, dk), konst(1.0 / (D as f64).sqrt())]);
        let masked = map(MapOp::Add, vec![scores, causal_mask(s, t)]);
        let o = matmul(softmax(masked, t), v, t); // [dv, s]
        let proj = matmul(o, input(nm("Wo"), &[dm, dv]), dv); // [dm, s]
        let x1 = map(MapOp::Add, vec![x, proj]);
        // mlp
        let xn2 = rms_norm(x1.clone(), nm("g2"), dm);
        let hpre = matmul(xn2, input(nm("Wf"), &[f, dm]), dm); // [f, s]
        let m = matmul(gelu(hpre), input(nm("Wp"), &[dm, f]), f); // [dm, s]
        x = map(MapOp::Add, vec![x1, m]);
    }
    let xf = rms_norm(x, "gf", dm);
    let logits = matmul(xf, input("wte", &[vv, dm]), dm); // [s, v]  (weight-tied)

    // ── next-token cross-entropy, mean over positions (composed logsumexp) ──
    let mmax = reduce(logits.clone(), vv, BinOp::Monoid(Monoid::Max));
    let sh = map(MapOp::Exp, vec![map(MapOp::Sub, vec![logits.clone(), mmax.clone()])]);
    let lse = map(MapOp::Add, vec![mmax, map(MapOp::Log, vec![reduce(sh, vv, add_r())])]); // [s]
    let picked = reduce(map(MapOp::Mul, vec![logits.clone(), one_hot(vv, input("tgt", &[s]))]), vv, add_r());
    let loss = map(MapOp::Mul, vec![reduce(map(MapOp::Sub, vec![lse, picked]), s, add_r()), konst(1.0 / S as f64)]);

    // ── one schedule: logits, loss, and every fused SGD update ──
    let params = weight_list(dm, vv, pp, &per_layer);
    let names: Vec<&'static str> = params.iter().map(|(n, _)| *n).collect();
    let grads = grad(&loss, &names, &ext);
    let mut roots: Vec<(Node, &'static str)> = vec![(logits.clone(), "logits"), (loss.clone(), "loss")];
    for (name, axes) in &params {
        let upd = map(MapOp::Sub, vec![input(name, axes), map(MapOp::Mul, vec![konst(lr), grads[name].clone()])]);
        roots.push((upd, leak(format!("{name}_next"))));
    }
    // training-time algebraic simplification: the winner-mask cancels and the
    // softmax backward reuses the forward's log-sum-exp (composed-LSE parity).
    let raw: Vec<Node> = roots.iter().map(|(n, _)| n.clone()).collect();
    let roots: Vec<(Node, &'static str)> =
        sanic::simplify::simplify_many(&raw).into_iter().zip(&roots).map(|(n, (_, nm))| (n, *nm)).collect();

    let ext_f: HashMap<Axis, f64> = ext.iter().map(|(&a, &n)| (a, n as f64)).collect();
    let sched = partition_many(&roots, &Device::toy(), &ext_f);
    let program = sanic::emit_metal::emit_schedule_metal_on(&Device::m1_pro(), &sched, &ext);
    println!("training step: {} kernels", program.stages.len());

    let Some(dev) = MetalDevice::open() else {
        eprintln!("no Metal device — skipping");
        return;
    };
    let pipes = dev.compile(&program.msl);

    // ── buffers: weights init + uploaded, ids/tgt per step, rest scratch ──
    let mut rng = Rng(0xA5A5_1234_5678);
    let mut bufs: HashMap<String, MetalBuf> = HashMap::new();
    for (name, axes) in &program.inputs {
        let size = axes.iter().map(|a| ext[a]).product::<usize>().max(1);
        if *name == "ids" || *name == "tgt" {
            bufs.insert(name.to_string(), dev.alloc_f32(size));
            continue;
        }
        // weight init: N(0, 0.02) for the matmuls, 1 for the norm gains.
        let init = if name.starts_with('g') {
            Tensor::from_fn(axes, &ext, |_| 1.0)
        } else {
            Tensor::from_fn(axes, &ext, |_| rng.normal(0.02))
        };
        bufs.insert(name.to_string(), dev.from_f64(&init.data));
    }
    for (name, size) in &program.buffers {
        bufs.insert(name.clone(), dev.alloc_f32(*size));
    }

    let run_window = |bufs: &HashMap<String, MetalBuf>, ids: &[f64], tgt: &[f64]| {
        dev.write_f64(&bufs["ids"], ids);
        dev.write_f64(&bufs["tgt"], tgt);
        dev.run(&program_dispatches(&program, bufs, &pipes));
    };
    // swap each weight with the fused `w_next` the schedule just wrote.
    let commit = |bufs: &mut HashMap<String, MetalBuf>| {
        for (name, _) in &params {
            let next = format!("{name}_next");
            let new = bufs[&next].clone();
            let old = bufs.insert(name.to_string(), new).unwrap();
            bufs.insert(next, old);
        }
    };

    // ── sampling: temperature-softmax autoregressive generation from a seed ──
    // logits are [s, v] row-major, so position p's row is logits[p*vsize .. ].
    let mut srng = Rng(0x5EED_C0DE);
    let temp = 0.8f32;
    let mut sample = |bufs: &HashMap<String, MetalBuf>, seed: &str, n: usize| -> String {
        let mut ctx: Vec<usize> = seed.chars().map(|c| vocab.stoi[&c]).collect();
        let mut out = String::from(seed);
        for _ in 0..n {
            let mut win = vec![0f64; S];
            let start = ctx.len().saturating_sub(S);
            let last = ctx.len() - start; // number of real tokens in the window
            for (i, &id) in ctx[start..].iter().enumerate() {
                win[i] = id as f64;
            }
            run_window(bufs, &win, &vec![0f64; S]);
            let lg = dev.read_f32(&bufs["logits"], S * vsize);
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

    // ── training loop: SGD on next-token prediction ──
    println!("training {steps} steps (lr {lr})…");
    let t0 = std::time::Instant::now();
    let mut ids = vec![0f64; S];
    let mut tgt = vec![0f64; S];
    for step in 0..steps {
        let i = rng.below(data.len() - S - 1);
        for j in 0..S {
            ids[j] = data[i + j] as f64;
            tgt[j] = data[i + j + 1] as f64;
        }
        run_window(&bufs, &ids, &tgt);
        commit(&mut bufs);
        if step < 8 || step % 250 == 0 || step == steps - 1 {
            let loss = dev.read_f32(&bufs["loss"], 1)[0];
            println!("  step {step:>5}  loss {loss:.3}  ({:.0} steps/s)", (step + 1) as f32 / t0.elapsed().as_secs_f32());
        }
    }

    println!("\n── sample ──");
    println!("{}", sample(&bufs, "\n", 400));
}
