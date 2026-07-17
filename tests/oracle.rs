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
use sanic::interp::{Env, Extents, Value, eval, run_carrier};
use sanic::ir::*;

/// A computed RoPE rotation matrix `R[pos, p, j, i]` (extent 2 on i, j): the
/// 2×2 rotation by θ = pos·freq_p, freq_p = exp(p·c) — synthesized from indices
/// with `cos`/`sin`/`exp`, no rotation tensor in memory. `R·q` (contracting i)
/// rotates each (q[p,0], q[p,1]) pair, which is exactly RoPE.
fn rope_rotation(pos: Axis, p: Axis, j: Axis, i: Axis, c: f64) -> Node {
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

fn rand_tensor(axes: &[Axis], ext: &Extents, rng: &mut Lcg) -> Value {
    Value::from_fn(axes, ext, |_| rng.f())
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
fn assert_kernel_matches_reference(node: &Node, axis: Axis, env: &Env, ext: &Extents) {
    let reference = eval(node, env, ext);
    let carrier = derive(node, axis).expect("axis is derivable");
    let via_kernel = run_carrier(node, axis, &carrier, env, ext);
    assert_close(&via_kernel, &reference);
}

// ── multi-head attention: contraction-in-body under a 4-axis grid ────────────
// b, h, sq, e are all free (grid); d contracted inside the QKᵀ leaf; k folded.
// This is the derived FlashAttention kernel with batch and head as grid dims —
// the shape a real transformer runs.
#[test]
fn multihead_flash_matches_naive() {
    let (b, h, sq, k, d, e) = (
        axis("b"),
        axis("h"),
        axis("sq"),
        axis("k"),
        axis("d"),
        axis("e"),
    );
    let ext: Extents = [(b, 2), (h, 3), (sq, 5), (k, 7), (d, 4), (e, 6)]
        .into_iter()
        .collect();
    let mut rng = Lcg(0xA11CE);
    let env: Env = [
        ("Q", rand_tensor(&[b, h, sq, d], &ext, &mut rng)),
        ("K", rand_tensor(&[b, h, k, d], &ext, &mut rng)),
        ("V", rand_tensor(&[b, h, k, e], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let mha = attention(
        input("Q", &[b, h, sq, d]),
        input("K", &[b, h, k, d]),
        input("V", &[b, h, k, e]),
        d,
        k,
    );
    assert_kernel_matches_reference(&mha, k, &env, &ext);
}

// ── RMSNorm-fused projection: the `defer-div` derivation on real tensors ──────
// q = Σ_dm (x·g·W) / √(Σ_dm x²/n + ε): the projection's contraction and the
// norm's own reduction ride the SAME folded axis, with 1/√· deferred to
// project. This is llm.rs kernel [0] (`fold dm`, 2 slots, defer-div).
#[test]
fn rmsnorm_fused_projection_matches_naive() {
    let (s, dm, f) = (axis("s"), axis("dm"), axis("f"));
    let n = 16.0;
    let ext: Extents = [(s, 4), (dm, 16), (f, 5)].into_iter().collect();
    let mut rng = Lcg(R_MS);
    let env: Env = [
        ("X", rand_tensor(&[s, dm], &ext, &mut rng)),
        ("g", rand_tensor(&[dm], &ext, &mut rng)),
        ("W", rand_tensor(&[f, dm], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let x = input("X", &[s, dm]);
    let g = input("g", &[dm]);
    let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), dm, add_r());
    let mean = map(MapOp::Mul, vec![ss, konst(1.0 / n)]);
    let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
    let xn = map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom]);
    let q = matmul(xn, input("W", &[f, dm]), dm);

    assert_kernel_matches_reference(&q, dm, &env, &ext);
}

// ── logsumexp: the max/Σexp pair projected by log(s)+m ───────────────────────
#[test]
fn logsumexp_matches_naive() {
    let (r, k) = (axis("r"), axis("k"));
    let ext: Extents = [(r, 4), (k, 9)].into_iter().collect();
    let mut rng = Lcg(0x15E);
    let env: Env = [("X", rand_tensor(&[r, k], &ext, &mut rng))]
        .into_iter()
        .collect();
    let lse = reduce(input("X", &[r, k]), k, BinOp::Monoid(Monoid::LogSumExp));
    assert_kernel_matches_reference(&lse, k, &env, &ext);
}

// ── quantized matmul: dequant fuses into the GEMM lift, automatically ────────
// y = Σ_dm x[s,dm] · (qW[o,dm] · scale[o]) — int-quantized weights dequantized
// per output channel. The dequant is elementwise, so the deriver folds it into
// the contraction's lift: ONE fused kernel, no materialized dequantized weight.
// This is the heart of quantized inference, generated with no new op.
#[test]
fn quantized_matmul_dequant_fuses() {
    let (s, dm, o) = (axis("s"), axis("dm"), axis("o"));
    let ext: Extents = [(s, 4), (dm, 12), (o, 6)].into_iter().collect();
    let mut rng = Lcg(0x9114A7);
    // integer-valued quantized weights (int4-ish range), per-channel scale
    let qw = Value::from_fn(&[o, dm], &ext, |_| (rng.f() * 8.0).round());
    let env: Env = [
        ("X", rand_tensor(&[s, dm], &ext, &mut rng)),
        ("qW", qw),
        (
            "scale",
            Value::from_fn(&[o], &ext, |_| 0.05 * (rng.f() + 1.5)),
        ),
    ]
    .into_iter()
    .collect();

    let dw = map(
        MapOp::Mul,
        vec![input("qW", &[o, dm]), input("scale", &[o])],
    );
    let y = matmul(input("X", &[s, dm]), dw, dm);

    // one fused kernel — the dequant rode the lift (or deferred), not a separate pass
    let carrier = derive(&y, dm).expect("quantized matmul derives");
    assert!(carrier.rules.contains(&"fold"));
    assert_kernel_matches_reference(&y, dm, &env, &ext);
}

// ── plain contraction (matmul) across a 2-axis grid ──────────────────────────
#[test]
fn matmul_kernel_matches_naive() {
    let (i, j, k) = (axis("i"), axis("j"), axis("k"));
    let ext: Extents = [(i, 5), (j, 6), (k, 8)].into_iter().collect();
    let mut rng = Lcg(0x2468);
    let env: Env = [
        ("A", rand_tensor(&[i, k], &ext, &mut rng)),
        ("B", rand_tensor(&[k, j], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();
    let mm = matmul(input("A", &[i, k]), input("B", &[k, j]), k);
    assert_kernel_matches_reference(&mm, k, &env, &ext);
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
    let (r, v) = (axis("r"), axis("v"));
    let ext: Extents = [(r, 5), (v, 20)].into_iter().collect();
    let mut rng = Lcg(0xA6A5);
    let logits = rand_tensor(&[r, v], &ext, &mut rng);
    let env: Env = [("L", logits.clone())].into_iter().collect();

    let am = argmax(input("L", &[r, v]), v);
    let got = eval(&am, &env, &ext);

    let want = Value::from_fn(&[r], &ext, |c| {
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
    let (t, d) = (axis("t"), axis("d"));
    let ext: Extents = [(t, 8), (d, 4)].into_iter().collect();
    let mut rng = Lcg(0x1CACE);
    let cache = rand_tensor(&[t, d], &ext, &mut rng);
    let new_k = rand_tensor(&[d], &ext, &mut rng);
    let pos = 5usize;
    let env: Env = [
        ("cache", cache.clone()),
        ("new_k", new_k.clone()),
        ("pos", Value::scalar(pos as f64)),
    ]
    .into_iter()
    .collect();

    let it = iota(t);
    let pos_in = input("pos", &[]);
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
        vec![is_pos, input("new_k", &[d]), input("cache", &[t, d])],
    );
    let got = eval(&updated, &env, &ext);

    let want = Value::from_fn(&[t, d], &ext, |c| {
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
    let (s, p, j, i) = (axis("s"), axis("p"), axis("j"), axis("i"));
    let ext: Extents = [(s, 5), (p, 4), (j, 2), (i, 2)].into_iter().collect();
    let c = -0.5;
    let mut rng = Lcg(0x60E5);
    let q = rand_tensor(&[s, p, i], &ext, &mut rng);
    let env: Env = [("Q", q.clone())].into_iter().collect();

    // rotated_Q[s,p,j] = Σ_i R[s,p,j,i]·Q[s,p,i]
    let r = rope_rotation(s, p, j, i, c);
    let qr = matmul(input("Q", &[s, p, i]), r, i);
    let got = eval(&qr, &env, &ext);

    // hand reference: rotate each pair by θ = s·exp(p·c)
    let want = Value::from_fn(&[s, p, j], &ext, |cd| {
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
        axis("s"),
        axis("t"),
        axis("p"),
        axis("i"),
        axis("j"),
        axis("dk"),
        axis("e"),
    );
    let ext: Extents = [(s, 5), (t, 5), (p, 4), (i, 2), (j, 2), (dk, 8), (e, 6)]
        .into_iter()
        .collect();
    let c = -0.4;
    let mut rng = Lcg(0x60EA77);
    let env: Env = [
        ("Q", rand_tensor(&[s, p, i], &ext, &mut rng)),
        ("K", rand_tensor(&[t, p, i], &ext, &mut rng)),
        ("V", rand_tensor(&[t, e], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let rq = rope_rotation(s, p, j, i, c);
    let rk = rope_rotation(t, p, j, i, c);
    let qr = matmul(input("Q", &[s, p, i]), rq, i); // [s, p, j]
    let kr = matmul(input("K", &[t, p, i]), rk, i); // [t, p, j]
    let scores = matmul(flatten(qr, &[p, j], dk), flatten(kr, &[p, j], dk), dk); // [s, t]
    let attn = matmul(softmax(scores, t), input("V", &[t, e]), t);

    let carrier = derive(&attn, t).expect("RoPE flash derives");
    assert!(
        carrier.rules.contains(&"rescale"),
        "online-softmax coupling present"
    );
    assert_kernel_matches_reference(&attn, t, &env, &ext);
}

// ── a computed cosine relative-position bias fused into attention ─────────────
// bias(s,t) = cos((s − t)·ω), synthesized from indices (no bias tensor, no
// traffic) and fused into the flash lift — the new `Cos` basis op flowing
// through derive → interpreter on real tensors. A relative-position bias of
// the ALiBi/rotary family, expressed as a plain composition.
#[test]
fn cosine_relative_bias_attention_matches_naive() {
    let (s, t, dk, dv) = (axis("s"), axis("t"), axis("dk"), axis("dv"));
    let ext: Extents = [(s, 6), (t, 6), (dk, 8), (dv, 5)].into_iter().collect();
    let mut rng = Lcg(0xC0517E);
    let env: Env = [
        ("Q", rand_tensor(&[s, dk], &ext, &mut rng)),
        ("K", rand_tensor(&[t, dk], &ext, &mut rng)),
        ("V", rand_tensor(&[t, dv], &ext, &mut rng)),
    ]
    .into_iter()
    .collect();

    let scores = matmul(input("Q", &[s, dk]), input("K", &[t, dk]), dk);
    let rel = map(MapOp::Sub, vec![iota(s), iota(t)]);
    let bias = map(MapOp::Cos, vec![map(MapOp::Mul, vec![rel, konst(0.1)])]);
    let biased = map(MapOp::Add, vec![scores, bias]);
    let attn = matmul(softmax(biased, t), input("V", &[t, dv]), t);

    assert_kernel_matches_reference(&attn, t, &env, &ext);
}

// a distinct nonzero seed constant, spelled to read as "RMS"
#[allow(non_upper_case_globals)]
const R_MS: u64 = 0x524D53;
