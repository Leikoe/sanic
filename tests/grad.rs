//! M8 — reverse-mode autodiff, held to finite differences.
//!
//! Every rule in [`sanic::grad`] is checked the only way that counts: the
//! gradient *graph*, evaluated by the oracle, must match central finite
//! differences of the forward loss, element by element. The suite covers the
//! chain rule through matmul, softmax cross-entropy, RMSNorm, masked
//! attention, stride-1 convolution (window transpose + split), and an
//! embedding gather (scatter-add backward). Then the payoff claim: a
//! gradient graph goes through `partition` → `execute` → compiled Rust
//! **exactly like a forward graph** — the backward pass is just another
//! dataflow program.

use std::collections::HashMap;

use sanic::cost::Device;
use sanic::grad::grad;
use sanic::interp::{Env, Extents, Tensor, eval};
use sanic::ir::*;
use sanic::partition::partition;

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
fn rand_tensor(axes: &[Axis], ext: &Extents, rng: &mut Lcg) -> Tensor {
    Tensor::from_fn(axes, ext, |_| rng.f())
}
fn as_f64(ext: &Extents) -> HashMap<Axis, f64> {
    ext.iter().map(|(&a, &n)| (a, n as f64)).collect()
}
fn add_r() -> BinOp {
    BinOp::Monoid(Monoid::Add)
}

/// Central finite differences of the scalar `loss` w.r.t. every element of
/// input `name`.
fn numeric_grad(loss: &Node, env: &Env, ext: &Extents, name: &'static str) -> Tensor {
    let base = env.get(name).unwrap().clone();
    let mut out = base.clone();
    for i in 0..base.data.len() {
        let h = 1e-5 * (1.0 + base.data[i].abs());
        let mut plus = env.clone();
        let mut minus = env.clone();
        let mut tp = base.clone();
        tp.data[i] += h;
        plus.insert(name, tp);
        let mut tm = base.clone();
        tm.data[i] -= h;
        minus.insert(name, tm);
        let lp = eval(loss, &plus, ext).data[0];
        let lm = eval(loss, &minus, ext).data[0];
        out.data[i] = (lp - lm) / (2.0 * h);
    }
    out
}

/// Assert the gradient graph for each name matches finite differences.
fn check_grads(loss: &Node, env: &Env, ext: &Extents, names: &[&'static str]) {
    let grads = grad(loss, names, ext);
    for &name in names {
        let g = grads
            .get(name)
            .unwrap_or_else(|| panic!("no gradient produced for `{name}`"));
        let analytic = eval(g, env, ext).permuted_to(&env.get(name).unwrap().axes);
        let numeric = numeric_grad(loss, env, ext, name);
        assert_eq!(analytic.shape, numeric.shape, "shape of d/d{name}");
        for (i, (a, n)) in analytic.data.iter().zip(&numeric.data).enumerate() {
            let tol = 1e-5 * (1.0 + a.abs().max(n.abs()));
            assert!(
                (a - n).abs() <= tol,
                "d(loss)/d({name})[{i}]: analytic {a} vs numeric {n}"
            );
        }
    }
}

// ── matmul chain: d/dX, d/dW of a squared projection ─────────────────────────
#[test]
fn matmul_squared_loss() {
    let (s, d, f) = (axis("s"), axis("d"), axis("f"));
    let ext: Extents = [(s, 3), (d, 4), (f, 5)].into_iter().collect();
    let mut rng = Lcg(0x6AD1);
    let env: Env = [
        ("X", rand_tensor(&[s, d], &ext, &mut rng)),
        ("W", rand_tensor(&[f, d], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let y = matmul(input("X", &[s, d]), input("W", &[f, d]), d); // [s, f]
    let sq = map(MapOp::Mul, vec![y.clone(), y]);
    let loss = reduce(reduce(sq, s, add_r()), f, add_r());
    check_grads(&loss, &env, &ext, &["X", "W"]);
}

// ── softmax cross-entropy: the classifier loss, end to end ───────────────────
#[test]
fn softmax_cross_entropy() {
    let (s, d, v) = (axis("s"), axis("d"), axis("v"));
    let ext: Extents = [(s, 3), (d, 4), (v, 6)].into_iter().collect();
    let mut rng = Lcg(0xCE11);
    // a one-hot-ish (soft) target — any distribution works for the math
    let target = Tensor::from_fn(&[s, v], &ext, |_| {
        let r = rng.f().abs() + 0.05;
        r / 3.0
    });
    let env: Env = [
        ("X", rand_tensor(&[s, d], &ext, &mut rng)),
        ("W", rand_tensor(&[v, d], &ext, &mut rng)),
        ("T", target),
    ]
    .into_iter()
    .collect();

    let logits = matmul(input("X", &[s, d]), input("W", &[v, d]), d); // [s, v]
    let p = softmax(logits, v);
    let ll = map(MapOp::Mul, vec![input("T", &[s, v]), map(MapOp::Log, vec![p])]);
    let loss = map(
        MapOp::Neg,
        vec![reduce(reduce(ll, v, add_r()), s, add_r())],
    );
    check_grads(&loss, &env, &ext, &["X", "W"]);
}

// ── RMSNorm: sqrt/div plus a broadcast-backward for the gain ─────────────────
#[test]
fn rmsnorm_gain_and_input() {
    let (s, d) = (axis("s"), axis("d"));
    let ext: Extents = [(s, 3), (d, 5)].into_iter().collect();
    let mut rng = Lcg(0x4A15);
    let env: Env = [
        ("X", rand_tensor(&[s, d], &ext, &mut rng)),
        ("G", rand_tensor(&[d], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let x = input("X", &[s, d]);
    let g = input("G", &[d]);
    let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), d, add_r());
    let mean = map(MapOp::Mul, vec![ss, konst(1.0 / 5.0)]);
    let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
    let y = map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom]);
    let sq = map(MapOp::Mul, vec![y.clone(), y]);
    let loss = reduce(reduce(sq, s, add_r()), d, add_r());
    check_grads(&loss, &env, &ext, &["X", "G"]);
}

// ── causally masked attention: the flash backward, as algebra ────────────────
#[test]
fn masked_attention_qkv() {
    let (s, t, dk, dv) = (axis("s"), axis("t"), axis("dk"), axis("dv"));
    let ext: Extents = [(s, 3), (t, 3), (dk, 3), (dv, 2)].into_iter().collect();
    let mut rng = Lcg(0xA77E);
    let env: Env = [
        ("Q", rand_tensor(&[s, dk], &ext, &mut rng)),
        ("K", rand_tensor(&[t, dk], &ext, &mut rng)),
        ("V", rand_tensor(&[t, dv], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let scores = matmul(input("Q", &[s, dk]), input("K", &[t, dk]), dk);
    let scaled = map(MapOp::Mul, vec![scores, konst(0.5)]);
    let masked = map(MapOp::Add, vec![scaled, causal_mask(s, t)]);
    let out = matmul(softmax(masked, t), input("V", &[t, dv]), t); // [s, dv]
    let sq = map(MapOp::Mul, vec![out.clone(), out]);
    let loss = reduce(reduce(sq, s, add_r()), dv, add_r());
    check_grads(&loss, &env, &ext, &["Q", "K", "V"]);
}

// ── stride-1 conv1d: window transpose (overlap-add) + split ⟵ flatten ────────
#[test]
fn conv1d_input_and_filter() {
    let (ci, w0, o, kk, r, co) = (
        axis("ci"),
        axis("w0"),
        axis("o"),
        axis("k"),
        axis("r"),
        axis("co"),
    );
    let ext: Extents = [(ci, 2), (w0, 7), (o, 5), (kk, 3), (r, 6), (co, 3)]
        .into_iter()
        .collect();
    let mut rng = Lcg(0xC04D6);
    let env: Env = [
        ("X", rand_tensor(&[ci, w0], &ext, &mut rng)),
        ("W", rand_tensor(&[co, ci, kk], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let xw = window(input("X", &[ci, w0]), w0, o, kk, 1, 1);
    let xf = flatten(xw, &[ci, kk], r);
    let wf = flatten(input("W", &[co, ci, kk]), &[ci, kk], r);
    let conv = matmul(xf, wf, r); // [o, co]
    let sq = map(MapOp::Mul, vec![conv.clone(), conv]);
    let loss = reduce(reduce(sq, o, add_r()), co, add_r());
    check_grads(&loss, &env, &ext, &["X", "W"]);
}

// ── embedding gather: the scatter-add backward ───────────────────────────────
#[test]
fn embedding_table_gradient() {
    let (v, d, s) = (axis("v"), axis("d"), axis("s"));
    let ext: Extents = [(v, 6), (d, 3), (s, 4)].into_iter().collect();
    let mut rng = Lcg(0xE4B);
    // repeated ids → colliding scatter contributions must add
    let ids = Tensor::from_fn(&[s], &ext, |c| [2.0, 5.0, 2.0, 0.0][c[&s]]);
    let env: Env = [
        ("E", rand_tensor(&[v, d], &ext, &mut rng)),
        ("ids", ids),
        ("Y", rand_tensor(&[d, s], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let emb = embedding(input("E", &[v, d]), input("ids", &[s]), v); // [d, s]
    let err = map(MapOp::Sub, vec![emb, input("Y", &[d, s])]);
    let sq = map(MapOp::Mul, vec![err.clone(), err]);
    let loss = reduce(reduce(sq, d, add_r()), s, add_r());
    check_grads(&loss, &env, &ext, &["E"]);
}

// ── an aliased read (rename view): both index spaces contribute ──────────────
#[test]
fn shared_input_through_a_rename() {
    let (s, t, d) = (axis("s"), axis("t"), axis("d"));
    let ext: Extents = [(s, 3), (t, 3), (d, 4)].into_iter().collect();
    let mut rng = Lcg(0x11A5);
    let env: Env = [
        ("X", rand_tensor(&[s, d], &ext, &mut rng)),
        ("A", rand_tensor(&[s, t], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    // X read at query positions AND (through a rename) at key positions —
    // the gradient must sum both paths. The final reduce over the absent `d`
    // axis is deliberate: it folds extent(d) copies (forward ×n), and the
    // gradient must carry the same factor.
    let x = input("X", &[s, d]);
    let xt = rename(x.clone(), s, t);
    let y = matmul(map(MapOp::Mul, vec![input("A", &[s, t]), xt]), x, d); // uses both
    let loss = reduce(reduce(reduce(y, s, add_r()), t, add_r()), d, add_r());
    let loss = map(MapOp::Mul, vec![loss.clone(), loss]);
    check_grads(&loss, &env, &ext, &["X"]);
}

// ── the payoff: a gradient graph is just another graph ───────────────────────
// dLoss/dV of masked attention, PARTITIONED and EXECUTED like any forward
// computation — same partitioner, same executor, same oracle equality.
#[test]
fn gradient_schedules_like_any_graph() {
    let (s, t, dk, dv) = (axis("s"), axis("t"), axis("dk"), axis("dv"));
    let ext: Extents = [(s, 4), (t, 4), (dk, 3), (dv, 3)].into_iter().collect();
    let mut rng = Lcg(0x6AD5);
    let env: Env = [
        ("Q", rand_tensor(&[s, dk], &ext, &mut rng)),
        ("K", rand_tensor(&[t, dk], &ext, &mut rng)),
        ("V", rand_tensor(&[t, dv], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let scores = matmul(input("Q", &[s, dk]), input("K", &[t, dk]), dk);
    let masked = map(MapOp::Add, vec![scores, causal_mask(s, t)]);
    let out = matmul(softmax(masked, t), input("V", &[t, dv]), t);
    let sq = map(MapOp::Mul, vec![out.clone(), out]);
    let loss = reduce(reduce(sq, s, add_r()), dv, add_r());

    let grads = grad(&loss, &["V", "Q"], &ext);
    for name in ["V", "Q"] {
        let g = &grads[name];
        let reference = eval(g, &env, &ext);
        let sched = partition(g, &Device::toy(), &as_f64(&ext));
        assert!(
            sched.stages.len() >= 1,
            "gradient of {name} must partition:\n{}",
            sched.render()
        );
        let executed = sched.execute(&env, &ext).permuted_to(&reference.axes);
        assert_eq!(executed.shape, reference.shape);
        for (a, b) in executed.data.iter().zip(&reference.data) {
            let tol = 1e-9 * (1.0 + a.abs().max(b.abs()));
            assert!((a - b).abs() <= tol, "d/d{name}: scheduled {a} vs eval {b}");
        }
    }
}

// ── optimizer fusion + the session: a real training loop ─────────────────────
// The whole training-step mechanism, end to end: the SGD update
// `w − lr·∇loss` is BUILT INTO the gradient's graph (the subtraction fuses as
// an epilogue of the gradient's final fold), the step schedule runs against
// session state, and the commit swaps the new weights in — the same
// Store/After discipline the KV cache uses. Loss must actually fall.
#[test]
fn sgd_training_loop_converges() {
    let (s, d) = (axis("s"), axis("d"));
    let ext: Extents = [(s, 8), (d, 4)].into_iter().collect();
    let mut rng = Lcg(0x54D6);
    let xs = rand_tensor(&[s, d], &ext, &mut rng);
    // targets from a hidden true weight vector — learnable exactly
    let w_true = rand_tensor(&[d], &ext, &mut rng);
    let targets = Tensor::from_fn(&[s], &ext, |c| {
        (0..4)
            .map(|di| {
                let xc: HashMap<Axis, usize> = [(s, c[&s]), (d, di)].into_iter().collect();
                let wc: HashMap<Axis, usize> = [(d, di)].into_iter().collect();
                xs.at(&xc) * w_true.at(&wc)
            })
            .sum()
    });

    // loss(w) = Σ_s (X·w − t)²
    let pred = matmul(input("X", &[s, d]), input("Wt", &[d]), d); // [s]
    let err = map(MapOp::Sub, vec![pred, input("T", &[s])]);
    let loss_node = reduce(map(MapOp::Mul, vec![err.clone(), err]), s, add_r());

    let grads = grad(&loss_node, &["Wt"], &ext);
    let step = map(
        MapOp::Sub,
        vec![
            input("Wt", &[d]),
            map(MapOp::Mul, vec![konst(0.05), grads["Wt"].clone()]),
        ],
    );

    // one schedule computes the loss AND the updated weights
    let sched = sanic::partition::partition_many(
        &[(loss_node.clone(), "loss"), (step, "w_next")],
        &Device::toy(),
        &as_f64(&ext),
    );

    let mut sess = sanic::runtime::Session::new(ext.clone());
    sess.bind("X", xs);
    sess.bind("T", targets);
    sess.bind("Wt", rand_tensor(&[d], &ext, &mut rng)); // random init

    let mut first = f64::NAN;
    let mut last = f64::NAN;
    for it in 0..100 {
        sess.step(&sched, &[("w_next", "Wt")]);
        let l = sess.get("loss").data[0];
        if it == 0 {
            first = l;
        }
        last = l;
    }
    assert!(
        last < first * 1e-9,
        "SGD did not converge: first loss {first:e}, last {last:e}"
    );
}
