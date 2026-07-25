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

use sanic::cost::DeviceProfile;
use sanic::grad::grad;
use sanic::interp::{Env, Value, eval};
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
fn rand_tensor(axes: &[Axis], rng: &mut Lcg) -> Value {
    let shape = axes.iter().map(|axis| axis.extent()).collect::<Vec<_>>();
    Value::from_shape_fn(&shape, |_| rng.f())
}
fn add_r() -> Monoid {
    Monoid::Add
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
        let analytic = eval(g, env);
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
        input("X", [s, d], Dtype::F32),
        transpose(input("W", [f, d], Dtype::F32), 0usize, 1usize),
    ); // [s, f]
    let sq = map(MapOp::Mul, vec![y.clone(), y]);
    let loss = reduce(reduce(sq, 1usize, add_r()), 0usize, add_r());
    check_grads(&loss, &env, &["X", "W"]);
}

// ── softmax cross-entropy: the classifier loss, end to end ───────────────────
#[test]
fn softmax_cross_entropy() {
    let (s, d, v) = (axis("s", 3), axis("d", 4), axis("v", 6));
    let mut rng = Lcg(0xCE11);
    // a one-hot-ish (soft) target — any distribution works for the math
    let target = Value::from_shape_fn(&[s.extent(), v.extent()], |_| {
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
        input("X", [s, d], Dtype::F32),
        transpose(input("W", [v, d], Dtype::F32), 0usize, 1usize),
    ); // [s, v]
    let p = softmax(logits, 1usize);
    let ll = map(
        MapOp::Mul,
        vec![input("T", [s, v], Dtype::F32), map(MapOp::Log, vec![p])],
    );
    let loss = map(MapOp::Neg, vec![reduce(reduce(ll, 1usize, add_r()), 0usize, add_r())]);
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

    let x = input("X", [s, d], Dtype::F32);
    let g = input("G", [d], Dtype::F32);
    let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), 1usize, add_r());
    let mean = map(MapOp::Mul, vec![ss, konst(1.0 / 5.0)]);
    let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
    let y = map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), unsqueeze(denom, 1usize)]);
    let sq = map(MapOp::Mul, vec![y.clone(), y]);
    let loss = reduce(reduce(sq, 1usize, add_r()), 0usize, add_r());
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
        input("Q", [s, dk], Dtype::F32),
        transpose(input("K", [t, dk], Dtype::F32), 0usize, 1usize),
    );
    let scaled = map(MapOp::Mul, vec![scores, konst(0.5)]);
    let masked = map(
        MapOp::Add,
        vec![scaled.clone(), causal_mask_like(scaled, 0usize, 1usize)],
    );
    let out = matmul(softmax(masked, 1usize), input("V", [t, dv], Dtype::F32)); // [s, dv]
    let sq = map(MapOp::Mul, vec![out.clone(), out]);
    let loss = reduce(reduce(sq, 1usize, add_r()), 0usize, add_r());
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

    let xw = window(input("X", [ci, w0], Dtype::F32), 1usize, o, kk, 1, 1);
    let xf = transpose(flatten(xw, &[0usize, 2usize][..], r), 0usize, 1usize);
    let wf = transpose(
        flatten(input("W", [co, ci, kk], Dtype::F32), &[1usize, 2usize][..], r),
        0usize,
        1usize,
    );
    let conv = matmul(xf, wf); // [o, co]
    let sq = map(MapOp::Mul, vec![conv.clone(), conv]);
    let loss = reduce(reduce(sq, 1usize, add_r()), 0usize, add_r());
    check_grads(&loss, &env, &["X", "W"]);
}

// ── embedding gather: the scatter-add backward ───────────────────────────────
#[test]
fn embedding_table_gradient() {
    let (v, d, s) = (axis("v", 6), axis("d", 3), axis("s", 4));
    let mut rng = Lcg(0xE4B);
    // repeated ids → colliding scatter contributions must add
    let ids = Value::from_shape_fn(&[s.extent()], |c| [2.0, 5.0, 2.0, 0.0][c[0]]);
    let env: Env = [
        ("E", rand_tensor(&[v, d], &mut rng)),
        ("ids", ids),
        ("Y", rand_tensor(&[s, d], &mut rng)),
    ]
    .into_iter()
    .collect();

    let emb = embedding(input("E", [v, d], Dtype::F32), input("ids", [s], Dtype::F32), 0usize); // [s, d]
    let err = map(MapOp::Sub, vec![emb, input("Y", [s, d], Dtype::F32)]);
    let sq = map(MapOp::Mul, vec![err.clone(), err]);
    let loss = reduce(reduce(sq, 1usize, add_r()), 0usize, add_r());
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

    // X is read at query positions and, through a rename, at key positions.
    // Explicit singleton insertion defines the shared [s, t, d] iteration
    // space; the gradient must sum both paths back into X.
    let x = input("X", [s, d], Dtype::F32);
    let xt = rename(x.clone(), 0usize, t);
    let y = map(
        MapOp::Mul,
        vec![
            map(
                MapOp::Mul,
                vec![unsqueeze(input("A", [s, t], Dtype::F32), 2usize), unsqueeze(xt, 0usize)],
            ),
            unsqueeze(x, 1usize),
        ],
    );
    let loss = reduce(reduce(reduce(y, 2usize, add_r()), 1usize, add_r()), 0usize, add_r());
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
        input("Q", [s, dk], Dtype::F32),
        transpose(input("K", [t, dk], Dtype::F32), 0usize, 1usize),
    );
    let masked = map(
        MapOp::Add,
        vec![scores.clone(), causal_mask_like(scores, 0usize, 1usize)],
    );
    let out = matmul(softmax(masked, 1usize), input("V", [t, dv], Dtype::F32));
    let sq = map(MapOp::Mul, vec![out.clone(), out]);
    let loss = reduce(reduce(sq, 1usize, add_r()), 0usize, add_r());

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
        let executed = sched.execute(&env);
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
    let targets = Value::from_shape_fn(&[s.extent()], |c| {
        (0..4).map(|di| xs.at_index(&[c[0], di]) * w_true.at_index(&[di])).sum()
    });

    // loss(w) = Σ_s (X·w − t)²
    let pred = reduce(
        map(
            MapOp::Mul,
            vec![input("X", [s, d], Dtype::F32), input("Wt", [d], Dtype::F32)],
        ),
        1usize,
        add_r(),
    ); // [s]
    let err = map(MapOp::Sub, vec![pred, input("T", [s], Dtype::F32)]);
    let loss_node = reduce(map(MapOp::Mul, vec![err.clone(), err]), 0usize, add_r());

    let grads = grad(&loss_node, &["Wt"]);
    let step = map(
        MapOp::Sub,
        vec![
            input("Wt", [d], Dtype::F32),
            map(MapOp::Mul, vec![konst(0.05), grads["Wt"].clone()]),
        ],
    );

    // one schedule computes the loss AND the updated weights
    let sched =
        sanic::partition::partition_many(&[(loss_node.clone(), "loss"), (step, "w_next")], &DeviceProfile::toy());

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
    let cs = scan(input("X", [s, t], Dtype::F32), 1usize, Monoid::Add);
    let wx = map(MapOp::Mul, vec![cs, input("W", [s, t], Dtype::F32)]);
    let sq = map(MapOp::Mul, vec![wx.clone(), wx]);
    let loss = reduce(reduce(sq, 1usize, Monoid::Add), 0usize, Monoid::Add);
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
    let env: Env = [("X", rand_tensor(&[w0], &mut rng)), ("W", rand_tensor(&[kk], &mut rng))]
        .into_iter()
        .collect();

    let xw = window(input("X", [w0], Dtype::F32), 0usize, o, kk, 2, 2); // [o, k]
    let conv = reduce(map(MapOp::Mul, vec![xw, input("W", [kk], Dtype::F32)]), 1usize, add_r()); // [o]
    let sq = map(MapOp::Mul, vec![conv.clone(), conv]);
    let loss = reduce(sq, 0usize, add_r());
    check_grads(&loss, &env, &["X", "W"]);
}

// Gradient targets are NODES, not just input names: an interior value (the
// normalizer of a softmax-like cone) gets its accumulated adjoint, and the
// same call returns the leaf gradient — matching the name-keyed API.
#[test]
fn tensor_targets_reach_interior_nodes() {
    use sanic::Tensor;
    let d = axis("d", 6);
    let x = Tensor::input("x", [d], Dtype::F32);
    let interior = (&x * &x).sum(0usize); // s = Σx²
    let loss = interior.log();

    let grads = loss.gradient(&[&interior, &x], &[]);
    let mut rng = Lcg(0x9d5);
    let env: Env = [("x", rand_tensor(&[d], &mut rng))].into_iter().collect();

    // dL/ds = 1/s
    let s = eval(interior.node(), &env).data[0];
    let ds = eval(grads[0].as_ref().unwrap().node(), &env).data[0];
    assert!((ds - 1.0 / s).abs() < 1e-12);

    // dL/dx matches the name-keyed grad
    let by_name = grad(loss.node(), &["x"]);
    let via_names = eval(&by_name["x"], &env);
    let via_nodes = eval(grads[1].as_ref().unwrap().node(), &env);
    assert_eq!(via_nodes.data, via_names.data);
}

// A stop tensor is a gradient boundary: the gradient reaches it, but its
// inputs see nothing through it. loss = x·detach(x) must give d/dx = detach(x),
// not 2x.
#[test]
fn stop_gradients_block_flow_without_blocking_targets() {
    use sanic::Tensor;
    let d = axis("d", 4);
    let x = Tensor::input("x", [d], Dtype::F32);
    let weight = (&x * &x).sum(0usize); // pretend-weighting we want frozen
    let loss = (&x * (&x * 0.0 + weight.clone())).sum(0usize);

    let mut rng = Lcg(0xde7ac);
    let env: Env = [("x", rand_tensor(&[d], &mut rng))].into_iter().collect();

    // stopped: d/dx = the frozen factor alone (weight, broadcast), plus the
    // adjoint still REACHES the stopped node as a target
    let grads = loss.gradient(&[&x, &weight], &[&weight]);
    let got = eval(grads[0].as_ref().unwrap().node(), &env);
    let w = eval(weight.node(), &env).data[0];
    for v in &got.data {
        assert!((v - w).abs() < 1e-9, "stopped gradient must be the frozen factor");
    }
    let at_stop = eval(grads[1].as_ref().unwrap().node(), &env).data[0];
    let x_sum: f64 = env["x"].data.iter().sum();
    assert!(
        (at_stop - x_sum).abs() < 1e-9,
        "the boundary itself still gets its adjoint"
    );
}

// Ties split the mass: max over [t, t] must give each element g/2, and the
// shares must sum to g exactly.
#[test]
fn max_reduce_ties_share_the_gradient_mass() {
    let d = axis("d", 4);
    let x = input("x", [d], Dtype::F32);
    let loss = reduce(x.clone(), 0usize, Monoid::Max);
    let grads = grad(&loss, &["x"]);

    let env: Env = [("x", Value::from_shape_fn(&[4], |i| if i[0] < 2 { 7.0 } else { 1.0 }))]
        .into_iter()
        .collect();
    let g = eval(&grads["x"], &env);
    assert_eq!(g.data, vec![0.5, 0.5, 0.0, 0.0]);
}

// Measurement harness, not a pin (run with --ignored --nocapture): the
// stage census of a llama-shaped decoder block's backward pass. This gates
// the reindex fiber-law work: if the dense affine transposes already fuse,
// that rewrite is optimization without a measurement.
#[test]
#[ignore]
fn backward_schedule_census() {
    use sanic::Tensor;
    use sanic::nn::ops::{attention, rms_norm, rope, rope_inv_freq, update_cache};
    use sanic::partition::{Stage, partition_many};

    let (hidden, heads, kv_heads, hd, ctx, ff) = (
        axis("hidden", 64),
        axis("heads", 4),
        axis("kv_heads", 2),
        axis("hd", 16),
        axis("ctx", 8),
        axis("ff", 128),
    );
    let seq = axis("sequence", 1);
    let position = Tensor::input("position", [], Dtype::F32);
    let x = Tensor::input("x", [seq, hidden], Dtype::F32);
    let w = |n: &str, o: Axis, i: Axis| Tensor::input(n, [o, i], Dtype::F32);

    let attn_in = rms_norm(&x, &Tensor::input("g1", [hidden], Dtype::F32), 1e-5);
    let project = |t: &Tensor, w: &Tensor, h: Axis| {
        t.matmul(w.transpose(0usize, 1usize))
            .split(-1isize, h, hd)
            .transpose(0usize, 1usize)
    };
    let q = project(&attn_in, &w("wq", axis("q_proj", 64), hidden), heads);
    let q = rope(&q, &position, seq, hd, rope_inv_freq(10_000.0));
    let k = project(&attn_in, &w("wk", axis("kv_proj", 32), hidden), kv_heads);
    let k = rope(&k, &position, seq, hd, rope_inv_freq(10_000.0));
    let v = project(&attn_in, &w("wv", axis("kv_proj", 32), hidden), kv_heads);
    let cache_shape = [kv_heads, ctx, hd];
    let kc = update_cache(&Tensor::input("ck", cache_shape, Dtype::F32), &k, &position);
    let vc = update_cache(&Tensor::input("cv", cache_shape, Dtype::F32), &v, &position);
    let mask = Tensor::iota(ctx).lt(&position + 1.0).select(0.0, f64::NEG_INFINITY);
    let attended = attention(&q, &kc, &vc, Some(&mask), None, true)
        .transpose(0usize, 1usize)
        .flatten(&[1usize, 2usize][..], hidden);
    let attended = attended.matmul(w("wo", hidden, hidden).transpose(0usize, 1usize));
    let res = &x + attended;
    let mlp_in = rms_norm(&res, &Tensor::input("g2", [hidden], Dtype::F32), 1e-5);
    let gate = mlp_in.matmul(w("wg", ff, hidden).transpose(0usize, 1usize));
    let up = mlp_in.matmul(w("wu", ff, hidden).transpose(0usize, 1usize));
    let out = &res + (gate.silu() * up).matmul(w("wd", hidden, ff).transpose(0usize, 1usize));
    let loss = (&out * &out).sum(1usize).sum(0usize);

    let weights = ["wq", "wk", "wv", "wo", "wg", "wu", "wd", "g1", "g2"];
    let grads = grad(loss.node(), &weights);
    let grad_names = ["d_wq", "d_wk", "d_wv", "d_wo", "d_wg", "d_wu", "d_wd", "d_g1", "d_g2"];
    let roots: Vec<(NodeRef, &'static str)> = weights
        .iter()
        .zip(grad_names)
        .filter_map(|(name, out)| grads.get(name).map(|g| (g.clone(), out)))
        .collect();
    let schedule = partition_many(&roots, &DeviceProfile::m1_pro());

    let mut census: std::collections::HashMap<&str, usize> = Default::default();
    for stage in &schedule.stages {
        let kind = match stage {
            Stage::Fused { .. } => "fold",
            Stage::Elementwise { .. } => "map",
            Stage::Gather { .. } => "gather",
            Stage::Fallback { .. } => "FALLBACK",
            Stage::Infeasible { .. } => "INFEASIBLE",
        };
        *census.entry(kind).or_default() += 1;
    }
    eprintln!("backward census ({} stages): {census:?}", schedule.stages.len());
    eprintln!("{}", schedule.decline_census());
}

// stored() boundaries are transparent to gradients: the estimator is
// straight-through, so d/dx of sum(2 · stored_bf16(x)) is exactly 2.
#[test]
fn stored_boundaries_pass_gradients_straight_through() {
    use sanic::Tensor;
    let d = axis("d", 4);
    let x = Tensor::input("x", [d], Dtype::F32);
    let loss = (x.stored(Dtype::BF16) * 2.0).sum(0usize);
    let grads = loss.gradient(&[&x], &[]);

    let mut rng = Lcg(0x570);
    let env: Env = [("x", rand_tensor(&[d], &mut rng))].into_iter().collect();
    let g = eval(grads[0].as_ref().unwrap().node(), &env);
    assert_eq!(g.data, vec![2.0; 4]);
}
