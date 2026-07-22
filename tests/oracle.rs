//! End-to-end correctness against the reference interpreter.
//!
//! `laws.rs` proves each derived carrier folds consistently with itself
//! (`tree_fold == fold`) and matches a *hand-written scalar reference over a
//! manually-built element stream*. This file proves the stronger, more
//! production-relevant property: the derived kernel, driven on **real input
//! tensors** through its true grid/stream structure, computes exactly what the
//! **naive graph** evaluates to.
//!
//! `run_carrier == eval` — the kernel computes the real math, not just a
//! self-consistent fold. Each test targets a distinct derivation feature
//! (in-body contraction, multi-axis grid, deferred normalizer, log-space
//! projection) so the sweep is a regression net for the whole `derive` layer.

use sanic::derive::derive;
use sanic::interp::{Env, Value, eval, run_carrier};
use sanic::ir::*;
use sanic::nn::scaled_dot_product_attention;

/// A computed RoPE rotation matrix `R[pos, p, j, i]` (extent 2 on i, j): the
/// 2×2 rotation by θ = pos·freq_p, freq_p = exp(p·c) — synthesized from indices
/// with `cos`/`sin`/`exp`, no rotation tensor in memory. `R·q` (contracting i)
/// rotates each (q[p,0], q[p,1]) pair, which is exactly RoPE.
fn rope_rotation(pos: Axis, p: Axis, j: Axis, i: Axis, c: f64) -> NodeRef {
    let freq = map(MapOp::Exp, vec![map(MapOp::Mul, vec![iota(p), konst(c)])]);
    let theta = map(
        MapOp::Mul,
        vec![unsqueeze(iota(pos), 1usize), unsqueeze(freq, 0usize)],
    ); // [pos, p]
    let i_coord = unsqueeze(iota(i), 0usize);
    let j_coord = unsqueeze(iota(j), 1usize);
    let lt_ij = map(MapOp::Lt, vec![i_coord.clone(), j_coord.clone()]);
    let lt_ji = map(MapOp::Lt, vec![j_coord, i_coord]);
    let eq = map(
        MapOp::Sub,
        vec![
            map(MapOp::Sub, vec![konst(1.0), lt_ij.clone()]),
            lt_ji.clone(),
        ],
    );
    let offdiag = map(MapOp::Sub, vec![lt_ij, lt_ji]); // (i<j) − (j<i)
    let theta = unsqueeze(unsqueeze(theta, 2usize), 3usize);
    let eq = unsqueeze(unsqueeze(eq, 0usize), 0usize);
    let offdiag = unsqueeze(unsqueeze(offdiag, 0usize), 0usize);
    map(
        MapOp::Add,
        vec![
            map(MapOp::Mul, vec![eq, map(MapOp::Cos, vec![theta.clone()])]),
            map(MapOp::Mul, vec![offdiag, map(MapOp::Sin, vec![theta])]),
        ],
    )
}

// ── a tiny deterministic PRNG (no external crates) ───────────────────────────
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

/// Assert two tensors are equal up to floating tolerance (the one policy in
/// `verify::rel_tolerance`; `terms` = the folded extent), regardless of axis
/// storage order.
fn assert_close(x: &Value, y: &Value, terms: usize) {
    assert_eq!(
        x.shape, y.shape,
        "shape mismatch: {:?} vs {:?}",
        x.axes, y.axes
    );
    for (i, (a, b)) in x.data.iter().zip(&y.data).enumerate() {
        let tol = sanic::verify::rel_tolerance(Dtype::F64, terms) * (1.0 + a.abs().max(b.abs()));
        assert!((a - b).abs() <= tol, "cell {i}: {a} vs {b}");
    }
}

/// Derive `node` over `axis`, run it on `env`, and require it to reproduce the
/// naive evaluation of the same graph.
fn assert_kernel_matches_reference(node: &NodeRef, axis: AxisRef, env: &Env) {
    let reference = eval(node, env);
    let carrier = derive(node, axis).expect("axis is derivable");
    let via_kernel = run_carrier(node, axis, &carrier, env);
    assert_close(&via_kernel, &reference, axis.extent());
}

// ── multi-head attention: contraction-in-body under a 4-axis grid ────────────
// b, h, sq, e are all free (grid); d contracted inside the QKᵀ leaf; k folded.
// This is the derived FlashAttention kernel with batch and head as grid dims —
// the shape a real transformer runs.
#[test]
fn multihead_flash_matches_naive() {
    let (b, h, sq, k, d, e) = (
        axis("b", 2),
        axis("h", 3),
        axis("sq", 5),
        axis("k", 7),
        axis("d", 4),
        axis("e", 6),
    );
    let mut rng = Lcg(0xA11CE);
    let env: Env = [
        ("Q", rand_tensor(&[b, h, sq, d], &mut rng)),
        ("K", rand_tensor(&[b, h, k, d], &mut rng)),
        ("V", rand_tensor(&[b, h, k, e], &mut rng)),
    ]
    .into_iter()
    .collect();

    let key = input("K", [b, h, k, d], Dtype::F32);
    let key_axis = axis_refs(&key)[2];
    let mha = scaled_dot_product_attention(
        input("Q", [b, h, sq, d], Dtype::F32),
        key,
        input("V", [b, h, k, e], Dtype::F32),
        None,
        0.0,
        false,
        Some(1.0),
        false,
    );
    assert_kernel_matches_reference(&mha, key_axis, &env);
}

// ── RMSNorm-fused projection: the `defer-div` derivation on real tensors ──────
// q = Σ_dm (x·g·W) / √(Σ_dm x²/n + ε): the projection's contraction and the
// norm's own reduction ride the SAME folded axis, with 1/√· deferred to
// project. This is llm.rs kernel [0] (`fold dm`, 2 slots, defer-div).
#[test]
fn rmsnorm_fused_projection_matches_naive() {
    let (s, dm, f) = (axis("s", 4), axis("dm", 16), axis("f", 5));
    let n = 16.0;
    let mut rng = Lcg(R_MS);
    let env: Env = [
        ("X", rand_tensor(&[s, dm], &mut rng)),
        ("g", rand_tensor(&[dm], &mut rng)),
        ("W", rand_tensor(&[f, dm], &mut rng)),
    ]
    .into_iter()
    .collect();

    let x = input("X", [s, dm], Dtype::F32);
    let stream = axis_refs(&x)[1];
    let g = input("g", [dm], Dtype::F32);
    let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), 1usize, add_r());
    let mean = map(MapOp::Mul, vec![ss, konst(1.0 / n)]);
    let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
    let xn = map(
        MapOp::Div,
        vec![map(MapOp::Mul, vec![x, g]), unsqueeze(denom, 1usize)],
    );
    let q = matmul(
        xn,
        transpose(input("W", [f, dm], Dtype::F32), 0usize, 1usize),
    );

    assert_kernel_matches_reference(&q, stream, &env);
}

#[test]
fn standalone_broadcast_denominator_keeps_the_stream_axis() {
    let (sequence, singleton, hidden) = (
        axis("sequence", 1),
        axis("singleton", 1),
        axis("hidden", 16),
    );
    let mut rng = Lcg(0x51A6_1E70);
    let env: Env = [
        ("a", rand_tensor(&[sequence, hidden], &mut rng)),
        ("b", rand_tensor(&[singleton, hidden], &mut rng)),
        ("c", rand_tensor(&[singleton, hidden], &mut rng)),
    ]
    .into_iter()
    .collect();
    let residual = map(
        MapOp::Add,
        vec![
            map(
                MapOp::Add,
                vec![
                    input("a", [sequence, hidden], Dtype::F32),
                    input("b", [singleton, hidden], Dtype::F32),
                ],
            ),
            input("c", [singleton, hidden], Dtype::F32),
        ],
    );
    let stream = axis_refs(&residual)[1];
    let sum_square = reduce(
        map(MapOp::Mul, vec![residual.clone(), residual]),
        1usize,
        Monoid::Add,
    );
    let denominator = unsqueeze(
        map(
            MapOp::Sqrt,
            vec![map(
                MapOp::Add,
                vec![
                    map(MapOp::Mul, vec![sum_square, konst(1.0 / 16.0)]),
                    konst(1e-5),
                ],
            )],
        ),
        1usize,
    );

    assert_kernel_matches_reference(&denominator, stream, &env);
}

// ── logsumexp: the max/Σexp pair projected by log(s)+m ───────────────────────
#[test]
fn logsumexp_matches_naive() {
    let (r, k) = (axis("r", 4), axis("k", 9));
    let mut rng = Lcg(0x15E);
    let env: Env = [("X", rand_tensor(&[r, k], &mut rng))]
        .into_iter()
        .collect();
    let x = input("X", [r, k], Dtype::F32);
    let stream = axis_refs(&x)[1];
    let lse = reduce(x, 1usize, Monoid::LogSumExp);
    assert_kernel_matches_reference(&lse, stream, &env);
}

// ── quantized matmul: dequant fuses into the GEMM lift, automatically ────────
// y = Σ_dm x[s,dm] · (qW[o,dm] · scale[o]) — int-quantized weights dequantized
// per output channel. The dequant is elementwise, so the deriver folds it into
// the contraction's lift: ONE fused kernel, no materialized dequantized weight.
// This is the heart of quantized inference, generated with no new op.
#[test]
fn quantized_matmul_dequant_fuses() {
    let (s, dm, o) = (axis("s", 4), axis("dm", 12), axis("o", 6));
    let mut rng = Lcg(0x9114A7);
    // integer-valued quantized weights (int4-ish range), per-channel scale
    let qw = Value::from_shape_fn(&[o.extent(), dm.extent()], |_| (rng.f() * 8.0).round());
    let env: Env = [
        ("X", rand_tensor(&[s, dm], &mut rng)),
        ("qW", qw),
        (
            "scale",
            Value::from_shape_fn(&[o.extent()], |_| 0.05 * (rng.f() + 1.5)),
        ),
    ]
    .into_iter()
    .collect();

    let dw = map(
        MapOp::Mul,
        vec![
            input("qW", [o, dm], Dtype::F32),
            unsqueeze(input("scale", [o], Dtype::F32), 1usize),
        ],
    );
    let x = input("X", [s, dm], Dtype::F32);
    let stream = axis_refs(&x)[1];
    let y = matmul(x, transpose(dw, 0usize, 1usize));

    // one fused kernel — the dequant rode the lift (or deferred), not a separate pass
    let carrier = derive(&y, stream).expect("quantized matmul derives");
    assert!(carrier.rules.contains(&"fold"));
    assert_kernel_matches_reference(&y, stream, &env);
}

// ── plain contraction (matmul) across a 2-axis grid ──────────────────────────
#[test]
fn matmul_kernel_matches_naive() {
    let (i, j, k) = (axis("i", 5), axis("j", 6), axis("k", 8));
    let mut rng = Lcg(0x2468);
    let env: Env = [
        ("A", rand_tensor(&[i, k], &mut rng)),
        ("B", rand_tensor(&[k, j], &mut rng)),
    ]
    .into_iter()
    .collect();
    let a = input("A", [i, k], Dtype::F32);
    let stream = axis_refs(&a)[1];
    let mm = matmul(a, input("B", [k, j], Dtype::F32));
    assert_kernel_matches_reference(&mm, stream, &env);
}

fn add_r() -> Monoid {
    Monoid::Add
}

// ── argmax (greedy sampling) is a generic frontend composition ────────────────
// `ir::argmax` takes max, replaces smaller values with +∞ and maxima with
// their `iota` index, then takes min. This selects the first maximum on ties.
#[test]
fn argmax_matches_hand() {
    let (r, v) = (axis("r", 5), axis("v", 20));
    let mut rng = Lcg(0xA6A5);
    let mut logits = rand_tensor(&[r, v], &mut rng);
    for ri in 0..5 {
        logits.data[ri * 20 + 3] = 9.0;
        logits.data[ri * 20 + 11] = 9.0;
    }
    let env: Env = [("L", logits.clone())].into_iter().collect();

    let am = argmax(input("L", [r, v], Dtype::F32), 1usize);
    let got = eval(&am, &env);

    let want = Value::from_shape_fn(&[r.extent()], |c| {
        let ri = c[0];
        let (mut best, mut arg) = (f64::NEG_INFINITY, 0);
        for vi in 0..20 {
            let x = logits.at_index(&[ri, vi]);
            if x > best {
                best = x;
                arg = vi;
            }
        }
        arg as f64
    });
    assert_close(&got, &want, v.extent());
}

// ── KV-cache row write, expressed with the existing basis ────────────────────
// updated[t,d] = where(t == pos, new_k[d], cache[t,d]) — the cache-update kernel
// of a decode step, with no new op. (Persisting the buffer across steps and
// writing in place is runtime orchestration; the *kernel* generates today.)
#[test]
fn kv_cache_write_matches_hand() {
    let (t, d) = (axis("t", 8), axis("d", 4));
    let mut rng = Lcg(0x1CACE);
    let cache = rand_tensor(&[t, d], &mut rng);
    let new_k = rand_tensor(&[d], &mut rng);
    let pos = 5usize;
    let env: Env = [
        ("cache", cache.clone()),
        ("new_k", new_k.clone()),
        ("pos", Value::scalar(pos as f64)),
    ]
    .into_iter()
    .collect();

    let cache_node = input("cache", [t, d], Dtype::F32);
    let pos_in = input("pos", [], Dtype::F32);
    let is_pos = one_hot_like(cache_node.clone(), 0usize, pos_in);
    let updated = map(
        MapOp::Where,
        vec![is_pos, input("new_k", [d], Dtype::F32), cache_node],
    );
    let got = eval(&updated, &env);

    let want = Value::from_shape_fn(&[t.extent(), d.extent()], |c| {
        if c[0] == pos {
            new_k.at_index(&[c[1]])
        } else {
            cache.at_index(c)
        }
    });
    assert_close(&got, &want, 1);
}

// ── RoPE rotation is really a rotation (graph vs hand-computed reference) ─────
#[test]
fn rope_rotation_is_correct() {
    let (s, p, j, i) = (axis("s", 5), axis("p", 4), axis("j", 2), axis("i", 2));
    let c = -0.5;
    let mut rng = Lcg(0x60E5);
    let q = rand_tensor(&[s, p, i], &mut rng);
    let env: Env = [("Q", q.clone())].into_iter().collect();

    // rotated_Q[s,p,j] = Σ_i R[s,p,j,i]·Q[s,p,i]
    let r = rope_rotation(s, p, j, i, c);
    let qr = reduce(
        map(
            MapOp::Mul,
            vec![unsqueeze(input("Q", [s, p, i], Dtype::F32), 2usize), r],
        ),
        3usize,
        add_r(),
    );
    let got = eval(&qr, &env);

    // hand reference: rotate each pair by θ = s·exp(p·c)
    let want = Value::from_shape_fn(&[s.extent(), p.extent(), j.extent()], |cd| {
        let (si, pi, ji) = (cd[0], cd[1], cd[2]);
        let theta = si as f64 * (pi as f64 * c).exp();
        let q0 = q.at_index(&[si, pi, 0]);
        let q1 = q.at_index(&[si, pi, 1]);
        if ji == 0 {
            theta.cos() * q0 - theta.sin() * q1
        } else {
            theta.sin() * q0 + theta.cos() * q1
        }
    });
    assert_close(&got, &want, 1);
}

// ── RoPE'd attention derives to ONE fused flash kernel, computed correctly ────
// Q/K carry an explicit pair axis; a computed rotation matmul applies RoPE; the
// rotated head dim (p,j) is flattened and contracted for the score. Because the
// rotations are free along the key axis, the whole thing folds into a single
// FlashAttention carrier — RoPE fused into flash, no movement ops, no template.
#[test]
fn rope_flash_attention_matches_naive() {
    let (s, t, p, i, j, dk, e) = (
        axis("s", 5),
        axis("t", 5),
        axis("p", 4),
        axis("i", 2),
        axis("j", 2),
        axis("dk", 8),
        axis("e", 6),
    );
    let c = -0.4;
    let mut rng = Lcg(0x60EA77);
    let env: Env = [
        ("Q", rand_tensor(&[s, p, i], &mut rng)),
        ("K", rand_tensor(&[t, p, i], &mut rng)),
        ("V", rand_tensor(&[t, e], &mut rng)),
    ]
    .into_iter()
    .collect();

    let q = input("Q", [s, p, i], Dtype::F32);
    let k_input = input("K", [t, p, i], Dtype::F32);
    let key_axis = axis_refs(&k_input)[0];
    let rq = rope_rotation(s, p, j, i, c);
    let rk = rope_rotation(t, p, j, i, c);
    let qr = reduce(
        map(MapOp::Mul, vec![unsqueeze(q, 2usize), rq]),
        3usize,
        add_r(),
    ); // [s, p, j]
    let kr = reduce(
        map(MapOp::Mul, vec![unsqueeze(k_input, 2usize), rk]),
        3usize,
        add_r(),
    ); // [t, p, j]
    let qr = flatten(qr, &[1usize, 2usize][..], dk);
    let kr = flatten(kr, &[1usize, 2usize][..], dk);
    let scores = matmul(qr, transpose(kr, 0usize, 1usize)); // [s, t]
    let attn = matmul(softmax(scores, 1usize), input("V", [t, e], Dtype::F32));

    let carrier = derive(&attn, key_axis).expect("RoPE flash derives");
    assert!(
        carrier.rules.contains(&"rescale"),
        "online-softmax coupling present"
    );
    assert_kernel_matches_reference(&attn, key_axis, &env);
}

// ── a computed cosine relative-position bias fused into attention ─────────────
// bias(s,t) = cos((s − t)·ω), synthesized from indices (no bias tensor, no
// traffic) and fused into the flash lift — the new `Cos` basis op flowing
// through derive → interpreter on real tensors. A relative-position bias of
// the ALiBi/rotary family, expressed as a plain composition.
#[test]
fn cosine_relative_bias_attention_matches_naive() {
    let (s, t, dk, dv) = (axis("s", 6), axis("t", 6), axis("dk", 8), axis("dv", 5));
    let mut rng = Lcg(0xC0517E);
    let env: Env = [
        ("Q", rand_tensor(&[s, dk], &mut rng)),
        ("K", rand_tensor(&[t, dk], &mut rng)),
        ("V", rand_tensor(&[t, dv], &mut rng)),
    ]
    .into_iter()
    .collect();

    let key = input("K", [t, dk], Dtype::F32);
    let key_axis = axis_refs(&key)[0];
    let scores = matmul(
        input("Q", [s, dk], Dtype::F32),
        transpose(key, 0usize, 1usize),
    );
    let rel = map(
        MapOp::Sub,
        vec![
            coordinate(scores.clone(), 0usize),
            coordinate(scores.clone(), 1usize),
        ],
    );
    let bias = map(MapOp::Cos, vec![map(MapOp::Mul, vec![rel, konst(0.1)])]);
    let biased = map(MapOp::Add, vec![scores, bias]);
    let attn = matmul(softmax(biased, 1usize), input("V", [t, dv], Dtype::F32));

    assert_kernel_matches_reference(&attn, key_axis, &env);
}

// a distinct nonzero seed constant, spelled to read as "RMS"
#[allow(non_upper_case_globals)]
const R_MS: u64 = 0x524D53;
