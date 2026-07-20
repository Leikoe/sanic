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

use sanic::cost::DeviceProfile;
use sanic::grad::grad;
use sanic::interp::{Env, Value, eval};
use sanic::kernel_ir::*;
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
fn rand_tensor(axes: &[Axis], rng: &mut Lcg) -> Value {
    Value::from_fn(axes, |_| rng.f())
}
fn add_r() -> BinOp {
    BinOp::Monoid(Monoid::Add)
}

/// Central finite differences of the scalar `loss` w.r.t. every element of
/// input `name`.
fn numeric_grad(loss: &NodeRef, env: &Env, name: &'static str) -> Value {
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
        let lp = eval(loss, &plus).data[0];
        let lm = eval(loss, &minus).data[0];
        out.data[i] = (lp - lm) / (2.0 * h);
    }
    out
}

/// Assert the gradient graph for each name matches finite differences.
fn check_grads(loss: &NodeRef, env: &Env, names: &[&'static str]) {
    let grads = grad(loss, names);
    for &name in names {
        let g = grads
            .get(name)
            .unwrap_or_else(|| panic!("no gradient produced for `{name}`"));
        let analytic = eval(g, env).permuted_to(&env.get(name).unwrap().axes);
        let numeric = numeric_grad(loss, env, name);
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
    let (s, d, f) = (axis("s", 3), axis("d", 4), axis("f", 5));
    let mut rng = Lcg(0x6AD1);
    let env: Env = [
        ("X", rand_tensor(&[s, d], &mut rng)),
        ("W", rand_tensor(&[f, d], &mut rng)),
    ]
    .into_iter()
    .collect();

    let y = matmul(
        input("X", &[s, d], Dtype::F32),
        input("W", &[f, d], Dtype::F32),
        d,
    ); // [s, f]
    let sq = map(MapOp::Mul, vec![y.clone(), y]);
    let loss = reduce(reduce(sq, s, add_r()), f, add_r());
    check_grads(&loss, &env, &["X", "W"]);
}

// ── softmax cross-entropy: the classifier loss, end to end ───────────────────
#[test]
fn softmax_cross_entropy() {
    let (s, d, v) = (axis("s", 3), axis("d", 4), axis("v", 6));
    let mut rng = Lcg(0xCE11);
    // a one-hot-ish (soft) target — any distribution works for the math
    let target = Value::from_fn(&[s, v], |_| {
        let r = rng.f().abs() + 0.05;
        r / 3.0
    });
    let env: Env = [
        ("X", rand_tensor(&[s, d], &mut rng)),
        ("W", rand_tensor(&[v, d], &mut rng)),
        ("T", target),
    ]
    .into_iter()
    .collect();

    let logits = matmul(
        input("X", &[s, d], Dtype::F32),
        input("W", &[v, d], Dtype::F32),
        d,
    ); // [s, v]
    let p = softmax(logits, v);
    let ll = map(
        MapOp::Mul,
        vec![input("T", &[s, v], Dtype::F32), map(MapOp::Log, vec![p])],
    );
    let loss = map(MapOp::Neg, vec![reduce(reduce(ll, v, add_r()), s, add_r())]);
    check_grads(&loss, &env, &["X", "W"]);
}

// ── RMSNorm: sqrt/div plus a broadcast-backward for the gain ─────────────────
#[test]
fn rmsnorm_gain_and_input() {
    let (s, d) = (axis("s", 3), axis("d", 5));
    let mut rng = Lcg(0x4A15);
    let env: Env = [
        ("X", rand_tensor(&[s, d], &mut rng)),
        ("G", rand_tensor(&[d], &mut rng)),
    ]
    .into_iter()
    .collect();

    let x = input("X", &[s, d], Dtype::F32);
    let g = input("G", &[d], Dtype::F32);
    let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), d, add_r());
    let mean = map(MapOp::Mul, vec![ss, konst(1.0 / 5.0)]);
    let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
    let y = map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom]);
    let sq = map(MapOp::Mul, vec![y.clone(), y]);
    let loss = reduce(reduce(sq, s, add_r()), d, add_r());
    check_grads(&loss, &env, &["X", "G"]);
}

// ── causally masked attention: the flash backward, as algebra ────────────────
#[test]
fn masked_attention_qkv() {
    let (s, t, dk, dv) = (axis("s", 3), axis("t", 3), axis("dk", 3), axis("dv", 2));
    let mut rng = Lcg(0xA77E);
    let env: Env = [
        ("Q", rand_tensor(&[s, dk], &mut rng)),
        ("K", rand_tensor(&[t, dk], &mut rng)),
        ("V", rand_tensor(&[t, dv], &mut rng)),
    ]
    .into_iter()
    .collect();

    let scores = matmul(
        input("Q", &[s, dk], Dtype::F32),
        input("K", &[t, dk], Dtype::F32),
        dk,
    );
    let scaled = map(MapOp::Mul, vec![scores, konst(0.5)]);
    let masked = map(MapOp::Add, vec![scaled, causal_mask(s, t)]);
    let out = matmul(softmax(masked, t), input("V", &[t, dv], Dtype::F32), t); // [s, dv]
    let sq = map(MapOp::Mul, vec![out.clone(), out]);
    let loss = reduce(reduce(sq, s, add_r()), dv, add_r());
    check_grads(&loss, &env, &["Q", "K", "V"]);
}

// ── stride-1 conv1d: window transpose (overlap-add) + split ⟵ flatten ────────
#[test]
fn conv1d_input_and_filter() {
    let (ci, w0, o, kk, r, co) = (
        axis("ci", 2),
        axis("w0", 7),
        axis("o", 5),
        axis("k", 3),
        axis("r", 6),
        axis("co", 3),
    );
    let mut rng = Lcg(0xC04D6);
    let env: Env = [
        ("X", rand_tensor(&[ci, w0], &mut rng)),
        ("W", rand_tensor(&[co, ci, kk], &mut rng)),
    ]
    .into_iter()
    .collect();

    let xw = window(input("X", &[ci, w0], Dtype::F32), w0, o, kk, 1, 1);
    let xf = flatten(xw, &[ci, kk], r);
    let wf = flatten(input("W", &[co, ci, kk], Dtype::F32), &[ci, kk], r);
    let conv = matmul(xf, wf, r); // [o, co]
    let sq = map(MapOp::Mul, vec![conv.clone(), conv]);
    let loss = reduce(reduce(sq, o, add_r()), co, add_r());
    check_grads(&loss, &env, &["X", "W"]);
}

// ── embedding gather: the scatter-add backward ───────────────────────────────
#[test]
fn embedding_table_gradient() {
    let (v, d, s) = (axis("v", 6), axis("d", 3), axis("s", 4));
    let mut rng = Lcg(0xE4B);
    // repeated ids → colliding scatter contributions must add
    let ids = Value::from_fn(&[s], |c| [2.0, 5.0, 2.0, 0.0][c[&s]]);
    let env: Env = [
        ("E", rand_tensor(&[v, d], &mut rng)),
        ("ids", ids),
        ("Y", rand_tensor(&[d, s], &mut rng)),
    ]
    .into_iter()
    .collect();

    let emb = embedding(
        input("E", &[v, d], Dtype::F32),
        input("ids", &[s], Dtype::F32),
        v,
    ); // [d, s]
    let err = map(MapOp::Sub, vec![emb, input("Y", &[d, s], Dtype::F32)]);
    let sq = map(MapOp::Mul, vec![err.clone(), err]);
    let loss = reduce(reduce(sq, d, add_r()), s, add_r());
    check_grads(&loss, &env, &["E"]);
}

// ── an aliased read (rename view): both index spaces contribute ──────────────
#[test]
fn shared_input_through_a_rename() {
    let (s, t, d) = (axis("s", 3), axis("t", 3), axis("d", 4));
    let mut rng = Lcg(0x11A5);
    let env: Env = [
        ("X", rand_tensor(&[s, d], &mut rng)),
        ("A", rand_tensor(&[s, t], &mut rng)),
    ]
    .into_iter()
    .collect();

    // X read at query positions AND (through a rename) at key positions —
    // the gradient must sum both paths. The final reduce over the absent `d`
    // axis is deliberate: it folds extent(d) copies (forward ×n), and the
    // gradient must carry the same factor.
    let x = input("X", &[s, d], Dtype::F32);
    let xt = rename(x.clone(), s, t);
    let y = matmul(
        map(MapOp::Mul, vec![input("A", &[s, t], Dtype::F32), xt]),
        x,
        d,
    ); // uses both
    let loss = reduce(reduce(reduce(y, s, add_r()), t, add_r()), d, add_r());
    let loss = map(MapOp::Mul, vec![loss.clone(), loss]);
    check_grads(&loss, &env, &["X"]);
}

// ── the payoff: a gradient graph is just another graph ───────────────────────
// dLoss/dV of masked attention, PARTITIONED and EXECUTED like any forward
// computation — same partitioner, same executor, same oracle equality.
#[test]
fn gradient_schedules_like_any_graph() {
    let (s, t, dk, dv) = (axis("s", 4), axis("t", 4), axis("dk", 3), axis("dv", 3));
    let mut rng = Lcg(0x6AD5);
    let env: Env = [
        ("Q", rand_tensor(&[s, dk], &mut rng)),
        ("K", rand_tensor(&[t, dk], &mut rng)),
        ("V", rand_tensor(&[t, dv], &mut rng)),
    ]
    .into_iter()
    .collect();

    let scores = matmul(
        input("Q", &[s, dk], Dtype::F32),
        input("K", &[t, dk], Dtype::F32),
        dk,
    );
    let masked = map(MapOp::Add, vec![scores, causal_mask(s, t)]);
    let out = matmul(softmax(masked, t), input("V", &[t, dv], Dtype::F32), t);
    let sq = map(MapOp::Mul, vec![out.clone(), out]);
    let loss = reduce(reduce(sq, s, add_r()), dv, add_r());

    let grads = grad(&loss, &["V", "Q"]);
    for name in ["V", "Q"] {
        let g = &grads[name];
        let reference = eval(g, &env);
        let sched = partition(g, &DeviceProfile::toy());
        assert!(
            !sched.stages.is_empty(),
            "gradient of {name} must partition:\n{}",
            sched.render()
        );
        let executed = sched.execute(&env).permuted_to(&reference.axes);
        assert_eq!(executed.shape, reference.shape);
        for (a, b) in executed.data.iter().zip(&reference.data) {
            let tol = sanic::verify::rel_tolerance(Dtype::F64, 64) * (1.0 + a.abs().max(b.abs()));
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
    let (s, d) = (axis("s", 8), axis("d", 4));
    let mut rng = Lcg(0x54D6);
    let xs = rand_tensor(&[s, d], &mut rng);
    // targets from a hidden true weight vector — learnable exactly
    let w_true = rand_tensor(&[d], &mut rng);
    let targets = Value::from_fn(&[s], |c| {
        (0..4)
            .map(|di| {
                let xc: HashMap<Axis, usize> = [(s, c[&s]), (d, di)].into_iter().collect();
                let wc: HashMap<Axis, usize> = [(d, di)].into_iter().collect();
                xs.at(&xc) * w_true.at(&wc)
            })
            .sum()
    });

    // loss(w) = Σ_s (X·w − t)²
    let pred = matmul(
        input("X", &[s, d], Dtype::F32),
        input("Wt", &[d], Dtype::F32),
        d,
    ); // [s]
    let err = map(MapOp::Sub, vec![pred, input("T", &[s], Dtype::F32)]);
    let loss_node = reduce(map(MapOp::Mul, vec![err.clone(), err]), s, add_r());

    let grads = grad(&loss_node, &["Wt"]);
    let step = map(
        MapOp::Sub,
        vec![
            input("Wt", &[d], Dtype::F32),
            map(MapOp::Mul, vec![konst(0.05), grads["Wt"].clone()]),
        ],
    );

    // one schedule computes the loss AND the updated weights
    let sched = sanic::partition::partition_many(
        &[(loss_node.clone(), "loss"), (step, "w_next")],
        &DeviceProfile::toy(),
    );

    let mut sess = sanic::runtime::Session::new();
    sess.bind("X", xs);
    sess.bind("T", targets);
    sess.bind("Wt", rand_tensor(&[d], &mut rng)); // random init

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

// ── cumsum backward: the reversed prefix sum, held to finite differences ─────
#[test]
fn cumsum_backward_is_the_reversed_cumsum() {
    let (s, t) = (axis("s", 3), axis("t", 7));
    let mut rng = Lcg(0xC5C5);
    let env: Env = [
        ("X", rand_tensor(&[s, t], &mut rng)),
        ("W", rand_tensor(&[s, t], &mut rng)),
    ]
    .into_iter()
    .collect();

    // loss = Σ (W ⊙ cumsum_t(X))² — the scan inside a nonlinear consumer
    let cs = scan(
        input("X", &[s, t], Dtype::F32),
        t,
        BinOp::Monoid(Monoid::Add),
    );
    let wx = map(MapOp::Mul, vec![cs, input("W", &[s, t], Dtype::F32)]);
    let sq = map(MapOp::Mul, vec![wx.clone(), wx]);
    let loss = reduce(
        reduce(sq, t, BinOp::Monoid(Monoid::Add)),
        s,
        BinOp::Monoid(Monoid::Add),
    );
    check_grads(&loss, &env, &["X", "W"]);
}

// ── strided AND dilated conv backward: the dense one-hot scatter ──────────────
// No affine inverse exists (the transpose needs a modular division); the
// rule scatters through a one-hot contraction, held to finite differences.
#[test]
fn strided_dilated_conv_backward() {
    // stride 2, dilation 2: input width 2·(o−1) + 2·(k−1) + 1 ≤ 11
    let (w0, o, kk) = (axis("w0", 11), axis("o", 4), axis("k", 3));
    let mut rng = Lcg(0x5D5D);
    let env: Env = [
        ("X", rand_tensor(&[w0], &mut rng)),
        ("W", rand_tensor(&[kk], &mut rng)),
    ]
    .into_iter()
    .collect();

    let xw = window(input("X", &[w0], Dtype::F32), w0, o, kk, 2, 2); // [o, k]
    let conv = reduce(
        map(MapOp::Mul, vec![xw, input("W", &[kk], Dtype::F32)]),
        kk,
        add_r(),
    ); // [o]
    let sq = map(MapOp::Mul, vec![conv.clone(), conv]);
    let loss = reduce(sq, o, add_r());
    check_grads(&loss, &env, &["X", "W"]);
}
