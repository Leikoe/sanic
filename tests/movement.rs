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

use sanic::cost::DeviceProfile;
use sanic::interp::{Env, Value, eval};
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
fn rand_tensor(axes: &[Axis], rng: &mut Lcg) -> Value {
    Value::from_shape_fn(
        &axes.iter().map(|axis| axis.extent()).collect::<Vec<_>>(),
        |_| rng.f(),
    )
}

fn assert_close(x: &Value, y: &Value) {
    assert_eq!(x.shape, y.shape, "shape: {:?} vs {:?}", x.axes, y.axes);
    let mut worst = 0.0f64;
    for (a, b) in x.data.iter().zip(&y.data) {
        worst = worst.max((a - b).abs() / (1.0 + a.abs().max(b.abs())));
    }
    // the one tolerance policy at this file's deepest fold chain
    assert!(
        worst < sanic::verify::rel_tolerance(Dtype::F64, 64),
        "max relative error {worst:e}"
    );
}

// ── the four primitive movements, against hand references ────────────────────

#[test]
fn slice_reads_the_shifted_range() {
    let (n, m) = (axis("n", 10), axis("m", 4));
    let mut rng = Lcg(11);
    let x = rand_tensor(&[n], &mut rng);
    let env: Env = [("X", x.clone())].into_iter().collect();

    let sl = slice(input("X", &[n], Dtype::F32), 0usize, m, 3);
    let got = eval(&sl, &env);
    assert_eq!(got.shape, vec![4]);
    for i in 0..4 {
        assert_eq!(got.data[i], x.data[i + 3]);
    }
}

#[test]
fn pad_reads_zero_outside() {
    let (n, p) = (axis("n", 5), axis("p", 9)); // lo=2, hi=2
    let mut rng = Lcg(12);
    let x = rand_tensor(&[n], &mut rng);
    let env: Env = [("X", x.clone())].into_iter().collect();

    let pd = pad(input("X", &[n], Dtype::F32), 0usize, p, 2);
    let got = eval(&pd, &env);
    for i in 0..9usize {
        let want = if (2..7).contains(&i) {
            x.data[i - 2]
        } else {
            0.0
        };
        assert_eq!(got.data[i], want, "position {i}");
    }
}

#[test]
fn split_is_the_inverse_of_flatten() {
    let (h, w, f) = (axis("h", 3), axis("w", 4), axis("f", 12));
    let (h2, w2) = (axis("h2", 3), axis("w2", 4));
    let mut rng = Lcg(13);
    let x = rand_tensor(&[h, w], &mut rng);
    let env: Env = [("X", x.clone())].into_iter().collect();

    // flatten [h,w] → f, then split f → (h2, w2): the identity, relabeled.
    let round = split(
        flatten(input("X", &[h, w], Dtype::F32), &[0, 1][..], f),
        0usize,
        h2,
        w2,
    );
    let got = eval(&round, &env);
    assert_eq!(got.shape, vec![3, 4]);
    assert_eq!(got.data, x.data, "split ∘ flatten must be the identity");
}

#[test]
fn window_matches_the_hand_window() {
    let (n, o, k) = (axis("n", 8), axis("o", 3), axis("k", 3)); // stride 2
    let mut rng = Lcg(14);
    let x = rand_tensor(&[n], &mut rng);
    let env: Env = [("X", x.clone())].into_iter().collect();

    let wd = window(input("X", &[n], Dtype::F32), 0usize, o, k, 2, 1);
    let got = eval(&wd, &env);
    for oi in 0..3 {
        for ki in 0..3 {
            assert_eq!(got.at_index(&[oi, ki]), x.data[oi * 2 + ki]);
        }
    }
}

// ── conv1d: window + flatten + matmul = ONE implicit-GEMM kernel ─────────────

#[test]
fn conv1d_is_one_derived_kernel_and_matches_hand() {
    let (ci, w0, o, kk, r, co) = (
        axis("ci", 3),
        axis("w0", 12),
        axis("o", 10),
        axis("k", 3),
        axis("r", 9),
        axis("co", 4),
    );
    let mut rng = Lcg(0xC04D);
    let x = rand_tensor(&[ci, w0], &mut rng);
    let w = rand_tensor(&[co, ci, kk], &mut rng);
    let env: Env = [("X", x.clone()), ("W", w.clone())].into_iter().collect();

    let xw = window(input("X", &[ci, w0], Dtype::F32), 1usize, o, kk, 1, 1); // [ci, o, k]
    let xf = flatten(xw, &[0, 2][..], r); // [r, o]
    let stream = axis_refs(&xf)[0];
    let wf = flatten(input("W", &[co, ci, kk], Dtype::F32), &[1, 2][..], r); // [co, r]
    let conv = matmul(transpose(xf, 0usize, 1usize), transpose(wf, 0usize, 1usize)); // [o, co]

    // the naive semantics equal a hand-written convolution
    let got = eval(&conv, &env);
    let hand = Value::from_shape_fn(&[10, 4], |coordinate| {
        let mut acc = 0.0;
        for c_i in 0..3 {
            for k_i in 0..3 {
                acc += x.at_index(&[c_i, coordinate[0] + k_i])
                    * w.at_index(&[coordinate[1], c_i, k_i]);
            }
        }
        acc
    });
    assert_close(&got, &hand);

    // it partitions to ONE fused kernel (implicit GEMM over the flattened
    // reduction axis), and the schedule reproduces the reference
    let sched = partition(&conv, &DeviceProfile::toy());
    assert_eq!(
        sched.stages.len(),
        1,
        "conv must be one kernel:\n{}",
        sched.render()
    );
    let Stage::Fused { spec, .. } = &sched.stages[0] else {
        panic!("expected a fused stage")
    };
    assert_eq!(
        spec.streaming_axis, stream,
        "streams the flattened (ci,k) axis"
    );
    let executed = sched.execute(&env);
    assert_close(&executed, &hand);
}

// ── conv2d: two windowed axes in one Reindex, still one kernel ───────────────

#[test]
fn conv2d_is_one_derived_kernel_and_matches_hand() {
    let (ci, h0, w0, oh, ow, kh, kw, r, co) = (
        axis("ci", 2),
        axis("h0", 7),
        axis("w0", 8),
        axis("oh", 5),
        axis("ow", 6),
        axis("kh", 3),
        axis("kw", 3),
        axis("r", 18), // ci·kh·kw
        axis("co", 4),
    );
    let mut rng = Lcg(0xC0442D);
    let x = rand_tensor(&[ci, h0, w0], &mut rng);
    let w = rand_tensor(&[co, ci, kh, kw], &mut rng);
    let env: Env = [("X", x.clone()), ("W", w.clone())].into_iter().collect();

    // both spatial axes window in a single Reindex node
    let xw = positional_reindex(
        input("X", &[ci, h0, w0], Dtype::F32),
        vec![ci, oh, kh, ow, kw],
        vec![
            (0, vec![(1, 0)], 0),
            (1, vec![(1, 1), (1, 2)], 0),
            (2, vec![(1, 3), (1, 4)], 0),
        ],
        false,
    ); // [ci, oh, kh, ow, kw]
    let xf = flatten(xw, &[0, 2, 4][..], r); // [r, oh, ow]
    let xf = transpose(transpose(xf, 0usize, 1usize), 1usize, 2usize); // [oh, ow, r]
    let wf = flatten(input("W", &[co, ci, kh, kw], Dtype::F32), &[1, 2, 3][..], r); // [co, r]
    let conv = matmul(xf, transpose(wf, 0usize, 1usize)); // [oh, ow, co]

    let got = eval(&conv, &env);
    let hand = Value::from_shape_fn(&[5, 6, 4], |coordinate| {
        let mut acc = 0.0;
        for c_i in 0..2 {
            for kh_i in 0..3 {
                for kw_i in 0..3 {
                    acc += x.at_index(&[c_i, coordinate[0] + kh_i, coordinate[1] + kw_i])
                        * w.at_index(&[coordinate[2], c_i, kh_i, kw_i]);
                }
            }
        }
        acc
    });
    assert_close(&got, &hand);

    let sched = partition(&conv, &DeviceProfile::toy());
    assert_eq!(
        sched.stages.len(),
        1,
        "conv2d must be one kernel:\n{}",
        sched.render()
    );
    let executed = sched.execute(&env);
    assert_close(&executed, &hand);
}

// ── SAME-padded conv: pad ∘ window compose ───────────────────────────────────

#[test]
fn padded_conv1d_matches_hand() {
    // SAME: pad by 1 both sides, window k=3 stride 1 → output extent = input's
    let (ci, w0, p0, o, kk, r, co) = (
        axis("ci", 2),
        axis("w0", 6),
        axis("p0", 8),
        axis("o", 6),
        axis("k", 3),
        axis("r", 6),
        axis("co", 3),
    );
    let mut rng = Lcg(0xADC0FFEE);
    let x = rand_tensor(&[ci, w0], &mut rng);
    let w = rand_tensor(&[co, ci, kk], &mut rng);
    let env: Env = [("X", x.clone()), ("W", w.clone())].into_iter().collect();

    let xp = pad(input("X", &[ci, w0], Dtype::F32), 1usize, p0, 1); // [ci, p0]
    let xw = window(xp, 1usize, o, kk, 1, 1); // [ci, o, k]
    let xf = flatten(xw, &[0, 2][..], r);
    let wf = flatten(input("W", &[co, ci, kk], Dtype::F32), &[1, 2][..], r);
    let conv = matmul(transpose(xf, 0usize, 1usize), transpose(wf, 0usize, 1usize)); // [o, co]

    let got = eval(&conv, &env);
    let hand = Value::from_shape_fn(&[6, 3], |coordinate| {
        let mut acc = 0.0;
        for c_i in 0..2 {
            for k_i in 0..3 {
                let pos = coordinate[0] as i64 + k_i as i64 - 1;
                if !(0..6).contains(&pos) {
                    continue; // zero-padded
                }
                acc += x.at_index(&[c_i, pos as usize]) * w.at_index(&[coordinate[1], c_i, k_i]);
            }
        }
        acc
    });
    assert_close(&got, &hand);

    let sched = partition(&conv, &DeviceProfile::toy());
    assert_eq!(sched.stages.len(), 1, "padded conv is still one kernel");
    let executed = sched.execute(&env);
    assert_close(&executed, &hand);
}

// ── pooling: window + reduce(Max), one kernel ────────────────────────────────

#[test]
fn maxpool_is_one_kernel_and_matches_hand() {
    let (c, w0, o, kk) = (axis("c", 3), axis("w0", 8), axis("o", 4), axis("k", 2));
    let mut rng = Lcg(0x9001);
    let x = rand_tensor(&[c, w0], &mut rng);
    let env: Env = [("X", x.clone())].into_iter().collect();

    let xw = window(input("X", &[c, w0], Dtype::F32), 1usize, o, kk, 2, 1); // [c, o, k]
    let pool = reduce(xw, 2usize, Monoid::Max); // [c, o]

    let got = eval(&pool, &env);
    let hand = Value::from_shape_fn(&[3, 4], |coordinate| {
        let base = coordinate[1] * 2;
        x.at_index(&[coordinate[0], base])
            .max(x.at_index(&[coordinate[0], base + 1]))
    });
    assert_close(&got, &hand);

    let sched = partition(&pool, &DeviceProfile::toy());
    assert_eq!(sched.stages.len(), 1, "pooling is one kernel");
    let executed = sched.execute(&env);
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
    let (ns, w) = (10usize, 4usize);
    let (s, t, j, d, e) = (
        axis("s", ns),
        axis("t", ns),
        axis("j", w),
        axis("d", 5),
        axis("e", 6),
    );
    let mut rng = Lcg(0x51D3);
    let q = rand_tensor(&[s, d], &mut rng);
    let k = rand_tensor(&[t, d], &mut rng);
    let v = rand_tensor(&[t, e], &mut rng);
    let env: Env = [("Q", q.clone()), ("K", k.clone()), ("V", v.clone())]
        .into_iter()
        .collect();

    // key position read at (s, j): t = s + j − (w−1); j ranges over the window
    let off = -((w - 1) as i64);
    let kw = positional_reindex(
        input("K", &[t, d], Dtype::F32),
        vec![s, j, d],
        vec![(0, vec![(1, 0), (1, 1)], off), (1, vec![(1, 2)], 0)],
        true,
    ); // [s, j, d]
    let stream = axis_refs(&kw)[1];
    let vw = positional_reindex(
        input("V", &[t, e], Dtype::F32),
        vec![s, j, e],
        vec![(0, vec![(1, 0), (1, 1)], off), (1, vec![(1, 2)], 0)],
        true,
    ); // [s, j, e]

    let scores = reduce(
        map(
            MapOp::Mul,
            vec![unsqueeze(input("Q", &[s, d], Dtype::F32), 1usize), kw],
        ),
        2usize,
        Monoid::Add,
    ); // [s, j]
    // mask out positions before the sequence start: s + j < w−1
    let invalid = map(
        MapOp::Lt,
        vec![
            map(
                MapOp::Add,
                vec![
                    coordinate(scores.clone(), 0usize),
                    coordinate(scores.clone(), 1usize),
                ],
            ),
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
    let attn = reduce(
        map(
            MapOp::Mul,
            vec![unsqueeze(softmax(masked, 1usize), 2usize), vw],
        ),
        1usize,
        Monoid::Add,
    ); // [s, e]

    // hand reference: masked softmax over the valid window positions
    let hand = Value::from_shape_fn(&[ns, 6], |coordinate| {
        let si = coordinate[0];
        let lo = si as i64 - (w as i64 - 1);
        let positions: Vec<usize> = (lo.max(0)..=si as i64).map(|p| p as usize).collect();
        let dots: Vec<f64> = positions
            .iter()
            .map(|&p| {
                (0..5)
                    .map(|di| q.at_index(&[si, di]) * k.at_index(&[p, di]))
                    .sum()
            })
            .collect();
        let m = dots.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exps: Vec<f64> = dots.iter().map(|x| (x - m).exp()).collect();
        let z: f64 = exps.iter().sum();
        positions
            .iter()
            .zip(&exps)
            .map(|(&p, &w_i)| (w_i / z) * v.at_index(&[p, coordinate[1]]))
            .sum()
    });

    let got = eval(&attn, &env);
    assert_close(&got, &hand);

    // one fused flash kernel streaming the WINDOW axis
    let sched = partition(&attn, &DeviceProfile::toy());
    assert_eq!(
        sched.stages.len(),
        1,
        "sliding-window attention must be one kernel:\n{}",
        sched.render()
    );
    let Stage::Fused { spec, .. } = &sched.stages[0] else {
        panic!("expected a fused stage")
    };
    assert_eq!(
        spec.streaming_axis, stream,
        "streams the window axis, not the sequence"
    );
    assert_eq!(
        spec.carrier.slots, 3,
        "the online-softmax (m, ℓ, o) carrier"
    );
    let executed = sched.execute(&env);
    assert_close(&executed, &hand);
}
