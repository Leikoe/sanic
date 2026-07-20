//! M9 — split reductions (GROUP), the two-stage fold.
//!
//! Re-associating a fold into per-chunk partials + a combine stage is legal
//! by the monoid law the carrier already certifies — these tests make that
//! numeric: `run_carrier_split(blocks) == eval` for awkward block counts
//! (remainders, more blocks than elements), for the coupled online-softmax
//! carrier where the merge does real work (rescaling by `exp(m − M)`), not
//! just for plain sums.
//!
//! The same re-association is the data-parallel story: each device folds its
//! shard, the allreduce is stage 2's merge — and it is the law behind the
//! intra-kernel simdgroup merges `plan::fold_sched` schedules.

use sanic::derive::derive;
use sanic::interp::{Env, Value, eval, run_carrier_split};
use sanic::kernel_ir::*;

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

/// The one tolerance policy (`verify::rel_tolerance`) at this file's chain
/// length: the toys fold ≤ 64 summed terms along their deepest path.
const CHAIN_TERMS: usize = 64;

fn assert_close(x: &Value, y: &Value) {
    let y = y.permuted_to(&x.axes);
    assert_eq!(x.shape, y.shape);
    for (a, b) in x.data.iter().zip(&y.data) {
        let tol =
            sanic::verify::rel_tolerance(Dtype::F64, CHAIN_TERMS) * (1.0 + a.abs().max(b.abs()));
        assert!((a - b).abs() <= tol, "{a} vs {b}");
    }
}

// ── the oracle: split == one-pass == eval, even for coupled carriers ─────────
#[test]
fn split_flash_equals_eval_for_any_block_count() {
    let (s, k, d, e) = (axis("s", 4), axis("k", 9), axis("d", 5), axis("e", 4));
    let mut rng = Lcg(0x6408);
    let env: Env = [
        ("Q", rand_tensor(&[s, d], &mut rng)),
        ("K", rand_tensor(&[k, d], &mut rng)),
        ("V", rand_tensor(&[k, e], &mut rng)),
    ]
    .into_iter()
    .collect();

    let attn = attention(
        input("Q", &[s, d], Dtype::F32),
        input("K", &[k, d], Dtype::F32),
        input("V", &[k, e], Dtype::F32),
        d,
        k,
    );
    let reference = eval(&attn, &env);
    let carrier = derive(&attn, k).unwrap();

    // 1 = the plain kernel; 2/3/7 leave uneven remainders; 9 = one element
    // per chunk, so stage 2's merge does ALL the softmax coupling work.
    for blocks in [1usize, 2, 3, 7, 9] {
        let got = run_carrier_split(&attn, k, &carrier, blocks, &env);
        assert_close(&got, &reference);
    }
}

// The merge is doing real algebra here: an all-masked leading chunk carries
// m = −∞ partials that the rescale must absorb without NaNs.
#[test]
fn split_causal_flash_handles_identity_partials() {
    let (s, t, dk, dv) = (axis("s", 6), axis("t", 6), axis("dk", 4), axis("dv", 3));
    let mut rng = Lcg(0x640C);
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
    let attn = matmul(softmax(masked, t), input("V", &[t, dv], Dtype::F32), t);

    let reference = eval(&attn, &env);
    let carrier = derive(&attn, t).unwrap();
    for blocks in [2usize, 3, 6] {
        let got = run_carrier_split(&attn, t, &carrier, blocks, &env);
        assert_close(&got, &reference);
    }
}
