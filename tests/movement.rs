//! M5 — the movement-op vocabulary (`Reindex`: slice / pad / split / window),
//! verified against hand references and the whole pipeline.
//!
//! The payoff cases: **convolution is `window + flatten + matmul`** — it
//! derives to ONE implicit-GEMM kernel with no im2col materialization and no
//! conv special case anywhere in the compiler — and **sliding-window
//! attention** derives to one flash kernel whose K/V reads are windowed by
//! index arithmetic. Pooling is `window + reduce(Max)`. Every case is checked
//! two ways: `eval` (the naive semantics) against a hand-written reference,
//! and `partition(...).execute` (the compiled schedule) against `eval`.

use std::collections::HashMap;

use sanic::cost::Device;
use sanic::interp::{Env, Extents, Tensor, eval};
use sanic::ir::*;
use sanic::partition::{Stage, partition};

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

fn assert_close(x: &Tensor, y: &Tensor) {
    let y = y.permuted_to(&x.axes);
    assert_eq!(x.shape, y.shape, "shape: {:?} vs {:?}", x.axes, y.axes);
    let mut worst = 0.0f64;
    for (a, b) in x.data.iter().zip(&y.data) {
        worst = worst.max((a - b).abs() / (1.0 + a.abs().max(b.abs())));
    }
    assert!(worst < 1e-9, "max relative error {worst:e}");
}

// ── the four primitive movements, against hand references ────────────────────

#[test]
fn slice_reads_the_shifted_range() {
    let (n, m) = (axis("n"), axis("m"));
    let ext: Extents = [(n, 10), (m, 4)].into_iter().collect();
    let mut rng = Lcg(11);
    let x = rand_tensor(&[n], &ext, &mut rng);
    let env: Env = [("X", x.clone())].into_iter().collect();

    let sl = slice(input("X", &[n]), n, m, 3);
    let got = eval(&sl, &env, &ext);
    assert_eq!(got.axes, vec![m]);
    for i in 0..4 {
        assert_eq!(got.data[i], x.data[i + 3]);
    }
}

#[test]
fn pad_reads_zero_outside() {
    let (n, p) = (axis("n"), axis("p"));
    let ext: Extents = [(n, 5), (p, 9)].into_iter().collect(); // lo=2, hi=2
    let mut rng = Lcg(12);
    let x = rand_tensor(&[n], &ext, &mut rng);
    let env: Env = [("X", x.clone())].into_iter().collect();

    let pd = pad(input("X", &[n]), n, p, 2);
    let got = eval(&pd, &env, &ext);
    for i in 0..9usize {
        let want = if (2..7).contains(&i) { x.data[i - 2] } else { 0.0 };
        assert_eq!(got.data[i], want, "position {i}");
    }
}

#[test]
fn split_is_the_inverse_of_flatten() {
    let (h, w, f) = (axis("h"), axis("w"), axis("f"));
    let (h2, w2) = (axis("h2"), axis("w2"));
    let ext: Extents = [(h, 3), (w, 4), (f, 12), (h2, 3), (w2, 4)]
        .into_iter()
        .collect();
    let mut rng = Lcg(13);
    let x = rand_tensor(&[h, w], &ext, &mut rng);
    let env: Env = [("X", x.clone())].into_iter().collect();

    // flatten [h,w] → f, then split f → (h2, w2): the identity, relabeled.
    let round = split(flatten(input("X", &[h, w]), &[h, w], f), f, h2, w2, 4);
    let got = eval(&round, &env, &ext);
    assert_eq!(got.axes, vec![h2, w2]);
    assert_eq!(got.data, x.data, "split ∘ flatten must be the identity");
}

#[test]
fn window_matches_the_hand_window() {
    let (n, o, k) = (axis("n"), axis("o"), axis("k"));
    let ext: Extents = [(n, 8), (o, 3), (k, 3)].into_iter().collect(); // stride 2
    let mut rng = Lcg(14);
    let x = rand_tensor(&[n], &ext, &mut rng);
    let env: Env = [("X", x.clone())].into_iter().collect();

    let wd = window(input("X", &[n]), n, o, k, 2, 1);
    let got = eval(&wd, &env, &ext);
    for oi in 0..3 {
        for ki in 0..3 {
            let c: HashMap<Axis, usize> = [(o, oi), (k, ki)].into_iter().collect();
            assert_eq!(got.at(&c), x.data[oi * 2 + ki]);
        }
    }
}

// ── conv1d: window + flatten + matmul = ONE implicit-GEMM kernel ─────────────

#[test]
fn conv1d_is_one_derived_kernel_and_matches_hand() {
    let (ci, w0, o, kk, r, co) = (
        axis("ci"),
        axis("w0"),
        axis("o"),
        axis("k"),
        axis("r"),
        axis("co"),
    );
    let ext: Extents = [(ci, 3), (w0, 12), (o, 10), (kk, 3), (r, 9), (co, 4)]
        .into_iter()
        .collect();
    let mut rng = Lcg(0xC04D);
    let x = rand_tensor(&[ci, w0], &ext, &mut rng);
    let w = rand_tensor(&[co, ci, kk], &ext, &mut rng);
    let env: Env = [("X", x.clone()), ("W", w.clone())].into_iter().collect();

    let xw = window(input("X", &[ci, w0]), w0, o, kk, 1, 1); // [ci, o, k]
    let xf = flatten(xw, &[ci, kk], r); // [r, o]
    let wf = flatten(input("W", &[co, ci, kk]), &[ci, kk], r); // [co, r]
    let conv = matmul(xf, wf, r); // [o, co]

    // the naive semantics equal a hand-written convolution
    let got = eval(&conv, &env, &ext);
    let hand = Tensor::from_fn(&[o, co], &ext, |c| {
        let mut acc = 0.0;
        for c_i in 0..3 {
            for k_i in 0..3 {
                let xc: HashMap<Axis, usize> = [(ci, c_i), (w0, c[&o] + k_i)].into_iter().collect();
                let wc: HashMap<Axis, usize> =
                    [(co, c[&co]), (ci, c_i), (kk, k_i)].into_iter().collect();
                acc += x.at(&xc) * w.at(&wc);
            }
        }
        acc
    });
    assert_close(&got, &hand);

    // it partitions to ONE fused kernel (implicit GEMM over the flattened
    // reduction axis), and the schedule reproduces the reference
    let sched = partition(&conv, &Device::toy(), &as_f64(&ext));
    assert_eq!(sched.stages.len(), 1, "conv must be one kernel:\n{}", sched.render());
    let Stage::Fused { spec, .. } = &sched.stages[0] else {
        panic!("expected a fused stage")
    };
    assert_eq!(spec.streaming_axis, r, "streams the flattened (ci,k) axis");
    let executed = sched.execute(&env, &ext);
    assert_close(&executed, &hand);
}

// ── conv2d: two windowed axes in one Reindex, still one kernel ───────────────

#[test]
fn conv2d_is_one_derived_kernel_and_matches_hand() {
    let (ci, h0, w0, oh, ow, kh, kw, r, co) = (
        axis("ci"),
        axis("h0"),
        axis("w0"),
        axis("oh"),
        axis("ow"),
        axis("kh"),
        axis("kw"),
        axis("r"),
        axis("co"),
    );
    let ext: Extents = [
        (ci, 2),
        (h0, 7),
        (w0, 8),
        (oh, 5),
        (ow, 6),
        (kh, 3),
        (kw, 3),
        (r, 18), // ci·kh·kw
        (co, 4),
    ]
    .into_iter()
    .collect();
    let mut rng = Lcg(0xC0442D);
    let x = rand_tensor(&[ci, h0, w0], &ext, &mut rng);
    let w = rand_tensor(&[co, ci, kh, kw], &ext, &mut rng);
    let env: Env = [("X", x.clone()), ("W", w.clone())].into_iter().collect();

    // both spatial axes window in a single Reindex node
    let xw = reindex(
        input("X", &[ci, h0, w0]),
        vec![
            (h0, vec![(1, oh), (1, kh)], 0),
            (w0, vec![(1, ow), (1, kw)], 0),
        ],
        false,
    ); // [ci, oh, kh, ow, kw]
    let xf = flatten(xw, &[ci, kh, kw], r); // [r, oh, ow]
    let wf = flatten(input("W", &[co, ci, kh, kw]), &[ci, kh, kw], r); // [co, r]
    let conv = matmul(xf, wf, r); // [oh, ow, co]

    let got = eval(&conv, &env, &ext);
    let hand = Tensor::from_fn(&[oh, ow, co], &ext, |c| {
        let mut acc = 0.0;
        for c_i in 0..2 {
            for kh_i in 0..3 {
                for kw_i in 0..3 {
                    let xc: HashMap<Axis, usize> =
                        [(ci, c_i), (h0, c[&oh] + kh_i), (w0, c[&ow] + kw_i)]
                            .into_iter()
                            .collect();
                    let wc: HashMap<Axis, usize> =
                        [(co, c[&co]), (ci, c_i), (kh, kh_i), (kw, kw_i)]
                            .into_iter()
                            .collect();
                    acc += x.at(&xc) * w.at(&wc);
                }
            }
        }
        acc
    });
    assert_close(&got, &hand);

    let sched = partition(&conv, &Device::toy(), &as_f64(&ext));
    assert_eq!(sched.stages.len(), 1, "conv2d must be one kernel:\n{}", sched.render());
    let executed = sched.execute(&env, &ext);
    assert_close(&executed, &hand);
}

// ── SAME-padded conv: pad ∘ window compose ───────────────────────────────────

#[test]
fn padded_conv1d_matches_hand() {
    let (ci, w0, p0, o, kk, r, co) = (
        axis("ci"),
        axis("w0"),
        axis("p0"),
        axis("o"),
        axis("k"),
        axis("r"),
        axis("co"),
    );
    // SAME: pad by 1 both sides, window k=3 stride 1 → output extent = input's
    let ext: Extents = [(ci, 2), (w0, 6), (p0, 8), (o, 6), (kk, 3), (r, 6), (co, 3)]
        .into_iter()
        .collect();
    let mut rng = Lcg(0xADC0FFEE);
    let x = rand_tensor(&[ci, w0], &ext, &mut rng);
    let w = rand_tensor(&[co, ci, kk], &ext, &mut rng);
    let env: Env = [("X", x.clone()), ("W", w.clone())].into_iter().collect();

    let xp = pad(input("X", &[ci, w0]), w0, p0, 1); // [ci, p0]
    let xw = window(xp, p0, o, kk, 1, 1); // [ci, o, k]
    let xf = flatten(xw, &[ci, kk], r);
    let wf = flatten(input("W", &[co, ci, kk]), &[ci, kk], r);
    let conv = matmul(xf, wf, r); // [o, co]

    let got = eval(&conv, &env, &ext);
    let hand = Tensor::from_fn(&[o, co], &ext, |c| {
        let mut acc = 0.0;
        for c_i in 0..2 {
            for k_i in 0..3 {
                let pos = c[&o] as i64 + k_i as i64 - 1;
                if pos < 0 || pos >= 6 {
                    continue; // zero-padded
                }
                let xc: HashMap<Axis, usize> =
                    [(ci, c_i), (w0, pos as usize)].into_iter().collect();
                let wc: HashMap<Axis, usize> =
                    [(co, c[&co]), (ci, c_i), (kk, k_i)].into_iter().collect();
                acc += x.at(&xc) * w.at(&wc);
            }
        }
        acc
    });
    assert_close(&got, &hand);

    let sched = partition(&conv, &Device::toy(), &as_f64(&ext));
    assert_eq!(sched.stages.len(), 1, "padded conv is still one kernel");
    let executed = sched.execute(&env, &ext);
    assert_close(&executed, &hand);
}

// ── pooling: window + reduce(Max), one kernel ────────────────────────────────

#[test]
fn maxpool_is_one_kernel_and_matches_hand() {
    let (c, w0, o, kk) = (axis("c"), axis("w0"), axis("o"), axis("k"));
    let ext: Extents = [(c, 3), (w0, 8), (o, 4), (kk, 2)].into_iter().collect();
    let mut rng = Lcg(0x9001);
    let x = rand_tensor(&[c, w0], &ext, &mut rng);
    let env: Env = [("X", x.clone())].into_iter().collect();

    let xw = window(input("X", &[c, w0]), w0, o, kk, 2, 1); // [c, o, k]
    let pool = reduce(xw, kk, BinOp::Monoid(Monoid::Max)); // [c, o]

    let got = eval(&pool, &env, &ext);
    let hand = Tensor::from_fn(&[c, o], &ext, |cd| {
        let base = cd[&o] * 2;
        let a: HashMap<Axis, usize> = [(c, cd[&c]), (w0, base)].into_iter().collect();
        let b: HashMap<Axis, usize> = [(c, cd[&c]), (w0, base + 1)].into_iter().collect();
        x.at(&a).max(x.at(&b))
    });
    assert_close(&got, &hand);

    let sched = partition(&pool, &Device::toy(), &as_f64(&ext));
    assert_eq!(sched.stages.len(), 1, "pooling is one kernel");
    let executed = sched.execute(&env, &ext);
    assert_close(&executed, &hand);
}

// ── sliding-window attention: windowed K/V reads, one flash kernel ───────────
//
// Each query s attends to the last `w` keys (positions s−w+1 … s). The K/V
// reads are windowed by index arithmetic (`Reindex`, padded), the validity
// mask is computed from iotas, and the whole thing derives to ONE flash
// kernel streaming the window axis — O(s·w) work instead of O(s²), with the
// same online-softmax accumulator.
#[test]
fn sliding_window_attention_is_one_flash_kernel() {
    let (s, t, j, d, e) = (axis("s"), axis("t"), axis("j"), axis("d"), axis("e"));
    let (ns, w) = (10usize, 4usize);
    let ext: Extents = [(s, ns), (t, ns), (j, w), (d, 5), (e, 6)]
        .into_iter()
        .collect();
    let mut rng = Lcg(0x51D3);
    let q = rand_tensor(&[s, d], &ext, &mut rng);
    let k = rand_tensor(&[t, d], &ext, &mut rng);
    let v = rand_tensor(&[t, e], &ext, &mut rng);
    let env: Env = [("Q", q.clone()), ("K", k.clone()), ("V", v.clone())]
        .into_iter()
        .collect();

    // key position read at (s, j): t = s + j − (w−1); j ranges over the window
    let off = -((w - 1) as i64);
    let kw = reindex(
        input("K", &[t, d]),
        vec![(t, vec![(1, s), (1, j)], off)],
        true,
    ); // [s, j, d]
    let vw = reindex(
        input("V", &[t, e]),
        vec![(t, vec![(1, s), (1, j)], off)],
        true,
    ); // [s, j, e]

    let scores = matmul(input("Q", &[s, d]), kw, d); // [s, j]
    // mask out positions before the sequence start: s + j < w−1
    let invalid = map(
        MapOp::Lt,
        vec![
            map(MapOp::Add, vec![iota(s), iota(j)]),
            konst((w - 1) as f64),
        ],
    );
    let masked = map(
        MapOp::Add,
        vec![
            scores,
            map(MapOp::Where, vec![invalid, konst(-1e30), konst(0.0)]),
        ],
    );
    let attn = matmul(softmax(masked, j), vw, j); // [s, e]

    // hand reference: masked softmax over the valid window positions
    let hand = Tensor::from_fn(&[s, e], &ext, |c| {
        let si = c[&s];
        let lo = si as i64 - (w as i64 - 1);
        let positions: Vec<usize> = (lo.max(0)..=si as i64).map(|p| p as usize).collect();
        let dots: Vec<f64> = positions
            .iter()
            .map(|&p| {
                (0..5)
                    .map(|di| {
                        let qc: HashMap<Axis, usize> = [(s, si), (d, di)].into_iter().collect();
                        let kc: HashMap<Axis, usize> = [(t, p), (d, di)].into_iter().collect();
                        q.at(&qc) * k.at(&kc)
                    })
                    .sum()
            })
            .collect();
        let m = dots.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exps: Vec<f64> = dots.iter().map(|x| (x - m).exp()).collect();
        let z: f64 = exps.iter().sum();
        positions
            .iter()
            .zip(&exps)
            .map(|(&p, &w_i)| {
                let vc: HashMap<Axis, usize> = [(t, p), (e, c[&e])].into_iter().collect();
                (w_i / z) * v.at(&vc)
            })
            .sum()
    });

    let got = eval(&attn, &env, &ext);
    assert_close(&got, &hand);

    // one fused flash kernel streaming the WINDOW axis
    let sched = partition(&attn, &Device::toy(), &as_f64(&ext));
    assert_eq!(
        sched.stages.len(),
        1,
        "sliding-window attention must be one kernel:\n{}",
        sched.render()
    );
    let Stage::Fused { spec, .. } = &sched.stages[0] else {
        panic!("expected a fused stage")
    };
    assert_eq!(spec.streaming_axis, j, "streams the window axis, not the sequence");
    assert_eq!(spec.carrier.slots, 3, "the online-softmax (m, ℓ, o) carrier");
    let executed = sched.execute(&env, &ext);
    assert_close(&executed, &hand);
}
