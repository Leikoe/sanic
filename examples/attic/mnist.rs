//! beautiful mnist — a small MLP, trained end to end on the Apple GPU.
//!
//! The one idea worth the file: **training is just another graph.** The
//! forward network, the softmax cross-entropy loss, and — via [`grad`] — the
//! *entire* backward pass are one dataflow graph in the same IR that expresses
//! GPT-2 inference. The SGD update `w ← w − lr·∇w` is BUILT INTO that graph
//! (a fused epilogue of each gradient's final fold), so ONE partitioned
//! schedule computes the loss and the new weights together. It is lowered to
//! MSL and dispatched on the GPU; between steps the weights commit by buffer
//! swap — the same commit-after-execute discipline the KV cache uses.
//!
//! No layer objects, no optimizer, no autograd tape: `mlp` and `cross_entropy`
//! build plain IR, `grad` transposes it, `partition_many` cuts it into
//! kernels. That is the whole trainer.
//!
//! ```text
//! cargo run --release --example mnist
//! cargo run --release --example mnist -- --steps 4000 --lr 0.2 --hidden 128
//! ```
//!
//! Data (not checked in) — fetch once:
//! ```text
//! mkdir -p data && cd data
//! base=https://ossci-datasets.s3.amazonaws.com/mnist
//! for f in train-images-idx3-ubyte train-labels-idx1-ubyte \
//!          t10k-images-idx3-ubyte t10k-labels-idx1-ubyte; do
//!   curl -sLO $base/$f.gz && gunzip -f $f.gz
//! done
//! ```

use sanic::cost::Device;
use sanic::grad::grad;
use sanic::interp::Value;
use sanic::ir::*;
use sanic::partition::partition_many;

const DIN: usize = 784; // 28×28 pixels
const CLS: usize = 10; // digits 0–9

fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

// ── the network + loss, as one dataflow graph ────────────────────────────────

/// A dense layer `Wx + b`: `matmul` contracts `cin`, the bias broadcasts over
/// whatever axes remain (here the batch).
fn dense(x: Node, w: &'static str, b: &'static str, out: Axis, cin: Axis) -> Node {
    map(
        MapOp::Add,
        vec![matmul(x, input(w, &[out, cin]), cin), input(b, &[out])],
    )
}

/// 784 → hidden → 10, ReLU (`max(·, 0)`) on the hidden layer. The whole model
/// is two matmuls and a max — no layer types, just axes flowing through.
fn mlp(b: Axis, din: Axis, hid: Axis, cls: Axis) -> Node {
    let h = map(
        MapOp::Max,
        vec![dense(input("X", &[b, din]), "W1", "b1", hid, din), konst(0.0)],
    );
    dense(h, "W2", "b2", cls, hid)
}

/// Softmax cross-entropy, averaged over the batch, as `logsumexp(z) − z[y]`.
///
/// Written as a *composition*, not a primitive: `logsumexp(z) = m + log(Σ
/// exp(z − m))` with `m = max z` — the numerically stable form, built from the
/// scalar basis. There is no softmax and no `logsumexp` op in the graph. `grad`
/// differentiates straight through it; the max-shift's gradient cancels, and
/// what falls out is exactly `softmax − onehot(y)`. `derive` then fuses the max
/// and the Σexp into ONE streaming `(m, ℓ)` carrier — the very accumulator
/// FlashAttention's softmax reduces to, differing only in the projection. The
/// label enters as a computed one-hot (`iota == y`), so no gather, and no
/// gradient flows to the targets.
fn cross_entropy(logits: Node, b: Axis, c: Axis, batch: usize) -> Node {
    let m = reduce(logits.clone(), c, BinOp::Monoid(Monoid::Max)); // [b]
    let shifted = map(MapOp::Exp, vec![map(MapOp::Sub, vec![logits.clone(), m.clone()])]);
    let lse = map(
        MapOp::Add,
        vec![m, map(MapOp::Log, vec![reduce(shifted, c, BinOp::Monoid(Monoid::Add))])],
    ); // [b]
    let picked = reduce(
        map(MapOp::Mul, vec![logits, one_hot(c, input("y", &[b]))]),
        c,
        BinOp::Monoid(Monoid::Add),
    ); // [b] — the logit of the correct class
    let per_example = map(MapOp::Sub, vec![lse, picked]);
    map(
        MapOp::Mul,
        vec![
            reduce(per_example, b, BinOp::Monoid(Monoid::Add)),
            konst(1.0 / batch as f64),
        ],
    )
}

// ── MNIST, from the IDX ubyte files ──────────────────────────────────────────

/// Read an IDX image file into `(count, pixels)` — pixels flat, row-major,
/// scaled to [0, 1].
fn read_images(path: &str) -> (usize, Vec<f64>) {
    let raw = std::fs::read(path).unwrap_or_else(|_| panic!("read {path}"));
    let n = u32::from_be_bytes(raw[4..8].try_into().unwrap()) as usize;
    let pixels = raw[16..].iter().map(|&b| b as f64 / 255.0).collect();
    (n, pixels)
}

/// Read an IDX label file into class indices.
fn read_labels(path: &str) -> Vec<u8> {
    let raw = std::fs::read(path).unwrap_or_else(|_| panic!("read {path}"));
    raw[8..].to_vec()
}

// ── a tiny PRNG for batch sampling and weight init ───────────────────────────

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
    fn below(&mut self, n: usize) -> usize {
        (self.next_u64() >> 11) as usize % n
    }
    /// A uniform sample in [−1, 1].
    fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
    }
}

/// Glorot-uniform for a weight, zeros for a bias — sized from the layer's
/// fan-in/out, keyed by name.
fn init_param(name: &str, axes: &[Axis], hid: usize, rng: &mut Rng) -> Value {
    let fan = match name {
        "W1" => Some((DIN, hid)),
        "W2" => Some((hid, CLS)),
        _ => None, // biases
    };
    match fan {
        Some((fin, fout)) => {
            let lim = (6.0 / (fin + fout) as f64).sqrt();
            Value::from_fn(axes, |_| rng.unit() * lim)
        }
        None => Value::from_fn(axes, |_| 0.0),
    }
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("the mnist example dispatches on Metal — run it on macOS");
}

// ── the GPU host: buffers, a step, an accuracy pass ───────────────────────────

#[cfg(target_os = "macos")]
use sanic::metal::{MetalBuf, MetalDevice, Pipelines, program_dispatches};
#[cfg(target_os = "macos")]
use sanic::emit_metal::MetalProgram;
#[cfg(target_os = "macos")]
use std::collections::HashMap;

/// Copy `rows` of the dataset into the X/y device buffers.
#[cfg(target_os = "macos")]
fn upload_batch(
    dev: &MetalDevice,
    bufs: &HashMap<String, MetalBuf>,
    imgs: &[f64],
    labels: &[u8],
    rows: &[usize],
) {
    let mut xb = vec![0f64; rows.len() * DIN];
    let mut yb = vec![0f64; rows.len()];
    for (i, &r) in rows.iter().enumerate() {
        xb[i * DIN..(i + 1) * DIN].copy_from_slice(&imgs[r * DIN..(r + 1) * DIN]);
        yb[i] = labels[r] as f64;
    }
    dev.write_f64(&bufs["X"], &xb);
    dev.write_f64(&bufs["y"], &yb);
}

/// Fraction of the test set the current weights classify correctly (%), by
/// running the graph over each test batch and taking the argmax logit.
#[cfg(target_os = "macos")]
fn accuracy(
    dev: &MetalDevice,
    program: &MetalProgram,
    pipes: &Pipelines,
    bufs: &HashMap<String, MetalBuf>,
    imgs: &[f64],
    labels: &[u8],
    batch: usize,
) -> f64 {
    let (mut correct, n) = (0usize, labels.len() / batch * batch);
    for chunk in 0..labels.len() / batch {
        let rows: Vec<usize> = (0..batch).map(|i| chunk * batch + i).collect();
        upload_batch(dev, bufs, imgs, labels, &rows);
        dev.run(&program_dispatches(program, bufs, &pipes));
        let out = dev.read_f32(&bufs["logits"], batch * CLS);
        for i in 0..batch {
            let row = &out[i * CLS..(i + 1) * CLS];
            let pred = (0..CLS).max_by(|&a, &c| row[a].total_cmp(&row[c])).unwrap();
            correct += (pred == labels[chunk * batch + i] as usize) as usize;
        }
    }
    100.0 * correct as f64 / n as f64
}

#[cfg(target_os = "macos")]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg = |flag: &str, def: f64| -> f64 {
        args.iter()
            .position(|a| a == flag)
            .and_then(|i| args.get(i + 1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(def)
    };
    let batch = arg("--batch", 100.0) as usize;
    let hid = arg("--hidden", 128.0) as usize;
    let lr = arg("--lr", 0.2);
    let steps = arg("--steps", 3000.0) as usize;

    if !std::path::Path::new("data/train-images-idx3-ubyte").exists() {
        eprintln!("MNIST not found in data/ — fetch it first (see the module doc)");
        return;
    }
    let (ntrain, train_x) = read_images("data/train-images-idx3-ubyte");
    let train_y = read_labels("data/train-labels-idx1-ubyte");
    let (_, test_x) = read_images("data/t10k-images-idx3-ubyte");
    let test_y = read_labels("data/t10k-labels-idx1-ubyte");
    println!("MNIST: {ntrain} train, {} test — {DIN} → {hid} → {CLS}", test_y.len());

    // ── build the training graph: loss + logits + the fused SGD updates ──
    let (b, din, hidx, cls) = (
        axis("b", batch),
        axis("din", DIN),
        axis("hid", hid),
        axis("cls", CLS),
    );

    let logits = mlp(b, din, hidx, cls);
    let loss = cross_entropy(logits.clone(), b, cls, batch);

    let params: [(&'static str, Vec<Axis>); 4] = [
        ("W1", vec![hidx, din]),
        ("b1", vec![hidx]),
        ("W2", vec![cls, hidx]),
        ("b2", vec![cls]),
    ];
    let grads = grad(&loss, &["W1", "b1", "W2", "b2"]);
    // one schedule, many outputs: the logits (for accuracy), the loss, and each
    // post-step weight `w − lr·∇w` — the update rides its gradient's kernel.
    // Logits first so it is materialized before the cross-entropy's logsumexp,
    // which then READS it instead of recomputing the second GEMM in-body.
    let mut roots: Vec<(Node, &'static str)> = vec![(logits, "logits"), (loss.clone(), "loss")];
    for (name, axes) in &params {
        let update = map(
            MapOp::Sub,
            vec![
                input(name, axes),
                map(MapOp::Mul, vec![konst(lr), grads[name].clone()]),
            ],
        );
        roots.push((update, leak(format!("{name}_next"))));
    }

    // Algebraically simplify the whole training step before scheduling. The
    // gradient's stabilizing max-shift cancels to `softmax` (no winner-mask),
    // and cross-root CSE lets the backward reuse the forward's logsumexp carrier
    // via `exp(z − lse)` instead of recomputing it — so the composed
    // cross-entropy's backward derives to the same kernels a hand-written
    // `softmax − onehot` rule would. See `sanic::simplify`.
    let nodes = sanic::simplify::simplify_many(&roots.iter().map(|(n, _)| n.clone()).collect::<Vec<_>>());
    let roots: Vec<(Node, &'static str)> =
        nodes.into_iter().zip(&roots).map(|(n, (_, name))| (n, *name)).collect();

    let t0 = std::time::Instant::now();
    let sched = partition_many(&roots, &Device::toy());
    let program = sanic::emit_metal::emit_schedule_metal_on(&Device::m1_pro(), &sched);
    println!(
        "training step: {} kernels, partitioned + lowered to MSL in {:.2}s",
        program.stages.len(),
        t0.elapsed().as_secs_f32()
    );

    let Some(dev) = MetalDevice::open() else {
        eprintln!("no Metal device — skipping");
        return;
    };
    let pipes = dev.compile(&program.msl);

    // ── buffers: weights uploaded once, X/y rewritten per step, the rest scratch ──
    let mut rng = Rng(0x9E3779B97F4A7C15);
    let mut bufs: HashMap<String, MetalBuf> = HashMap::new();
    for (name, axes) in &program.inputs {
        let size = axes.iter().map(|a| a.extent).product::<usize>().max(1);
        if *name == "X" || *name == "y" {
            let expect: Vec<Axis> = if *name == "X" { vec![b, din] } else { vec![b] };
            assert_eq!(axes, &expect, "`{name}` uploaded assuming declared axis order");
            bufs.insert(name.to_string(), dev.alloc_f32(size));
        } else {
            let init = init_param(name, axes, hid, &mut rng);
            bufs.insert(name.to_string(), dev.from_f64(&init.data));
        }
    }
    for (name, size) in &program.buffers {
        bufs.insert(name.clone(), dev.alloc_f32(*size));
    }

    // ── the training loop: minibatch SGD, entirely on the GPU ──
    println!("training {steps} steps (batch {batch}, lr {lr})…");
    let t0 = std::time::Instant::now();
    for step in 0..steps {
        let rows: Vec<usize> = (0..batch).map(|_| rng.below(ntrain)).collect();
        upload_batch(&dev, &bufs, &train_x, &train_y, &rows);
        dev.run(&program_dispatches(&program, &bufs, &pipes));

        // commit-after-execute: the new weights swap in for the next step
        for (name, _) in &params {
            let next = format!("{name}_next");
            let new = bufs[&next].clone();
            let old = bufs.insert(name.to_string(), new).unwrap();
            bufs.insert(next, old);
        }

        if step % 500 == 0 || step == steps - 1 {
            let loss = dev.read_f32(&bufs["loss"], 1)[0];
            let acc = accuracy(&dev, &program, &pipes, &bufs, &test_x, &test_y, batch);
            println!(
                "  step {step:>5}  loss {loss:.4}  test acc {acc:.2}%  ({:.0} steps/s)",
                (step + 1) as f32 / t0.elapsed().as_secs_f32()
            );
        }
    }
    println!(
        "done in {:.1}s — final test accuracy {:.2}%",
        t0.elapsed().as_secs_f32(),
        accuracy(&dev, &program, &pipes, &bufs, &test_x, &test_y, batch)
    );
}
