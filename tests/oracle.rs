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

use std::collections::HashMap;

use sanic::derive::derive;
use sanic::interp::{Env, Value, eval, run_carrier};
use sanic::kernel_ir::*;

/// A computed RoPE rotation matrix `R[pos, p, j, i]` (extent 2 on i, j): the
/// 2×2 rotation by θ = pos·freq_p, freq_p = exp(p·c) — synthesized from indices
/// with `cos`/`sin`/`exp`, no rotation tensor in memory. `R·q` (contracting i)
/// rotates each (q[p,0], q[p,1]) pair, which is exactly RoPE.
fn rope_rotation(pos: Axis, p: Axis, j: Axis, i: Axis, c: f64) -> NodeRef {
    let freq = map(MapOp::Exp, vec![map(MapOp::Mul, vec![iota(p), konst(c)])]);
    let theta = map(MapOp::Mul, vec![iota(pos), freq]); // [pos, p]
    let lt_ij = map(MapOp::Lt, vec![iota(i), iota(j)]);
    let lt_ji = map(MapOp::Lt, vec![iota(j), iota(i)]);
    let eq = map(
        MapOp::Sub,
        vec![
            map(MapOp::Sub, vec![konst(1.0), lt_ij.clone()]),
            lt_ji.clone(),
        ],
    );
    let offdiag = map(MapOp::Sub, vec![lt_ij, lt_ji]); // (i<j) − (j<i)
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
    Value::from_fn(axes, |_| rng.f())
}

/// Assert two tensors are equal up to floating tolerance, regardless of axis
/// storage order.
fn assert_close(x: &Value, y: &Value) {
    let y = y.permuted_to(&x.axes);
    assert_eq!(
        x.shape, y.shape,
        "shape mismatch: {:?} vs {:?}",
        x.axes, y.axes
    );
    for (i, (a, b)) in x.data.iter().zip(&y.data).enumerate() {
        let tol = 1e-9 * (1.0 + a.abs().max(b.abs()));
        assert!((a - b).abs() <= tol, "cell {i}: {a} vs {b}");
    }
}

/// Derive `node` over `axis`, run it on `env`, and require it to reproduce the
/// naive evaluation of the same graph.
fn assert_kernel_matches_reference(node: &NodeRef, axis: Axis, env: &Env) {
    let reference = eval(node, env);
    let carrier = derive(node, axis).expect("axis is derivable");
    let via_kernel = run_carrier(node, axis, &carrier, env);
    assert_close(&via_kernel, &reference);
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

    let mha = attention(
        input("Q", &[b, h, sq, d], Dtype::F32),
        input("K", &[b, h, k, d], Dtype::F32),
        input("V", &[b, h, k, e], Dtype::F32),
        d,
        k,
    );
    assert_kernel_matches_reference(&mha, k, &env);
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

    let x = input("X", &[s, dm], Dtype::F32);
    let g = input("g", &[dm], Dtype::F32);
    let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), dm, add_r());
    let mean = map(MapOp::Mul, vec![ss, konst(1.0 / n)]);
    let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
    let xn = map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom]);
    let q = matmul(xn, input("W", &[f, dm], Dtype::F32), dm);

    assert_kernel_matches_reference(&q, dm, &env);
}

// ── logsumexp: the max/Σexp pair projected by log(s)+m ───────────────────────
#[test]
fn logsumexp_matches_naive() {
    let (r, k) = (axis("r", 4), axis("k", 9));
    let mut rng = Lcg(0x15E);
    let env: Env = [("X", rand_tensor(&[r, k], &mut rng))]
        .into_iter()
        .collect();
    let lse = reduce(
        input("X", &[r, k], Dtype::F32),
        k,
        BinOp::Monoid(Monoid::LogSumExp),
    );
    assert_kernel_matches_reference(&lse, k, &env);
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
    let qw = Value::from_fn(&[o, dm], |_| (rng.f() * 8.0).round());
    let env: Env = [
        ("X", rand_tensor(&[s, dm], &mut rng)),
        ("qW", qw),
        ("scale", Value::from_fn(&[o], |_| 0.05 * (rng.f() + 1.5))),
    ]
    .into_iter()
    .collect();

    let dw = map(
        MapOp::Mul,
        vec![
            input("qW", &[o, dm], Dtype::F32),
            input("scale", &[o], Dtype::F32),
        ],
    );
    let y = matmul(input("X", &[s, dm], Dtype::F32), dw, dm);

    // one fused kernel — the dequant rode the lift (or deferred), not a separate pass
    let carrier = derive(&y, dm).expect("quantized matmul derives");
    assert!(carrier.rules.contains(&"fold"));
    assert_kernel_matches_reference(&y, dm, &env);
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
    let mm = matmul(
        input("A", &[i, k], Dtype::F32),
        input("B", &[k, j], Dtype::F32),
        k,
    );
    assert_kernel_matches_reference(&mm, k, &env);
}

fn add_r() -> BinOp {
    BinOp::Monoid(Monoid::Add)
}

// ── argmax (greedy sampling) computes the right index, no new IR ──────────────
// `ir::argmax` is `Σ_k k·[x[k] == max_k x]` with the equality indicator from
// `1 − (x < max)`; the partitioner splits it into two kernels (the max, then
// the index sum reading it).
#[test]
fn argmax_matches_hand() {
    let (r, v) = (axis("r", 5), axis("v", 20));
    let mut rng = Lcg(0xA6A5);
    let logits = rand_tensor(&[r, v], &mut rng);
    let env: Env = [("L", logits.clone())].into_iter().collect();

    let am = argmax(input("L", &[r, v], Dtype::F32), v);
    let got = eval(&am, &env);

    let want = Value::from_fn(&[r], |c| {
        let ri = c[&r];
        let (mut best, mut arg) = (f64::NEG_INFINITY, 0);
        for vi in 0..20 {
            let x = logits.at(&HashMap::from([(r, ri), (v, vi)]));
            if x > best {
                best = x;
                arg = vi;
            }
        }
        arg as f64
    });
    assert_close(&got, &want);
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

    let it = iota(t);
    let pos_in = input("pos", &[], Dtype::F32);
    let is_pos = map(
        MapOp::Sub,
        vec![
            map(
                MapOp::Sub,
                vec![konst(1.0), map(MapOp::Lt, vec![it.clone(), pos_in.clone()])],
            ),
            map(MapOp::Lt, vec![pos_in, it]),
        ],
    ); // 1 where t == pos, else 0
    let updated = map(
        MapOp::Where,
        vec![
            is_pos,
            input("new_k", &[d], Dtype::F32),
            input("cache", &[t, d], Dtype::F32),
        ],
    );
    let got = eval(&updated, &env);

    let want = Value::from_fn(&[t, d], |c| {
        if c[&t] == pos {
            new_k.at(&HashMap::from([(d, c[&d])]))
        } else {
            cache.at(c)
        }
    });
    assert_close(&got, &want);
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
    let qr = matmul(input("Q", &[s, p, i], Dtype::F32), r, i);
    let got = eval(&qr, &env);

    // hand reference: rotate each pair by θ = s·exp(p·c)
    let want = Value::from_fn(&[s, p, j], |cd| {
        let (si, pi, ji) = (cd[&s], cd[&p], cd[&j]);
        let theta = si as f64 * (pi as f64 * c).exp();
        let q0 = q.at(&HashMap::from([(s, si), (p, pi), (i, 0)]));
        let q1 = q.at(&HashMap::from([(s, si), (p, pi), (i, 1)]));
        if ji == 0 {
            theta.cos() * q0 - theta.sin() * q1
        } else {
            theta.sin() * q0 + theta.cos() * q1
        }
    });
    assert_close(&got, &want);
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

    let rq = rope_rotation(s, p, j, i, c);
    let rk = rope_rotation(t, p, j, i, c);
    let qr = matmul(input("Q", &[s, p, i], Dtype::F32), rq, i); // [s, p, j]
    let kr = matmul(input("K", &[t, p, i], Dtype::F32), rk, i); // [t, p, j]
    let scores = matmul(flatten(qr, &[p, j], dk), flatten(kr, &[p, j], dk), dk); // [s, t]
    let attn = matmul(softmax(scores, t), input("V", &[t, e], Dtype::F32), t);

    let carrier = derive(&attn, t).expect("RoPE flash derives");
    assert!(
        carrier.rules.contains(&"rescale"),
        "online-softmax coupling present"
    );
    assert_kernel_matches_reference(&attn, t, &env);
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

    let scores = matmul(
        input("Q", &[s, dk], Dtype::F32),
        input("K", &[t, dk], Dtype::F32),
        dk,
    );
    let rel = map(MapOp::Sub, vec![iota(s), iota(t)]);
    let bias = map(MapOp::Cos, vec![map(MapOp::Mul, vec![rel, konst(0.1)])]);
    let biased = map(MapOp::Add, vec![scores, bias]);
    let attn = matmul(softmax(biased, t), input("V", &[t, dv], Dtype::F32), t);

    assert_kernel_matches_reference(&attn, t, &env);
}

// a distinct nonzero seed constant, spelled to read as "RMS"
#[allow(non_upper_case_globals)]
const R_MS: u64 = 0x524D53;
