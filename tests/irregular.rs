//! M7 — the irregular tail: argmax, top-k and scatter, as compositions of the
//! existing basis (no new node kinds), verified against hand references and
//! through the partitioned pipeline.
//!
//! * `argmax` — min payload among indices tied at the maximum key. The generic
//!   extremal-key/payload law derives its two-slot carrier from composition.
//! * `topk` — repeated max/argmax and one-hot masking, built by the frontend
//!   from ordinary graph operations. Carrier discovery for the whole
//!   composition remains compiler work rather than Top-k semantics.
//! * `scatter_add` — the inverse of gather as a one-hot contraction,
//!   add-combining collisions. Order-free (deterministic in parallel), and
//!   exactly the backward of `gather`, which autodiff (M8) leans on.
//!
//! Full sort is *declined* deliberately: a data-movement network, not a
//! fold — outside the algebra this compiler trusts, and unneeded for
//! inference.

use sanic::cost::DeviceProfile;
use sanic::derive::{SlotKind, derive};
use sanic::interp::{Env, Value, eval};
use sanic::ir::*;
use sanic::partition::{Stage, partition, partition_many};

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

// ── argmax: a generic extremal-key/payload product fold ─────────────────────

#[test]
fn argmax_is_one_generic_product_fold() {
    let n = axis("n", 8);
    let data = [2.0, 5.0, 5.0, 1.0, 5.0, 2.0, 0.0, 5.0];
    let values = Value::from_shape_fn(&[8], |coordinate| data[coordinate[0]]);
    let env: Env = [("X", values)].into_iter().collect();
    let x = input("X", &[n], Dtype::F32);
    let stream = axis_refs(&x)[0];
    let node = argmax(x, 0usize);

    let carrier = derive(&node, stream).expect("argmax composition should derive");
    assert_eq!(
        carrier.slots, 2,
        "Acc = (maximum value, minimum tied index)"
    );
    assert!(carrier.rules.contains(&"extremum-filter"));
    assert!(matches!(
        carrier.kinds.as_slice(),
        [
            SlotKind::Plain(Monoid::Max),
            SlotKind::AtExtremum {
                key_slot: 0,
                key: Monoid::Max,
                ties: Monoid::Min,
            }
        ]
    ));

    let schedule = partition(&node, &DeviceProfile::toy());
    assert_eq!(
        schedule.stages.len(),
        1,
        "argmax composition should be one product fold:\n{}",
        schedule.render()
    );
    assert!(matches!(&schedule.stages[0], Stage::Fused { spec, .. } if spec.carrier.slots == 2));
    assert_eq!(schedule.execute(&env).data, vec![1.0]);
}

// ── top-k: values and indices, largest first ─────────────────────────────────

#[test]
fn topk_matches_a_hand_sort() {
    let n = axis("n", 12);
    let mut rng = Lcg(0x70B5);
    let x = rand_tensor(&[n], &mut rng); // continuous → no ties
    let env: Env = [("X", x.clone())].into_iter().collect();

    let k = 3;
    let pairs = topk(input("X", &[n], Dtype::F32), 0usize, k);

    // hand reference: sort (value, index) descending by value
    let mut order: Vec<(f64, usize)> = x.data.iter().copied().zip(0..).collect();
    order.sort_by(|a, b| b.0.total_cmp(&a.0));

    for (round, (v, i)) in pairs.iter().enumerate() {
        let got_v = eval(v, &env).data[0];
        let got_i = eval(i, &env).data[0];
        assert_eq!(got_v, order[round].0, "value of round {round}");
        assert_eq!(got_i, order[round].1 as f64, "index of round {round}");
    }
}

// The whole top-k chain as ONE multi-output schedule: later rounds read the
// earlier rounds' materialized max/argmax through the entangler cuts, and the
// executed schedule reproduces the reference.
#[test]
fn topk_partitions_and_executes() {
    let n = axis("n", 16);
    let mut rng = Lcg(0x70B52);
    let x = rand_tensor(&[n], &mut rng);
    let env: Env = [("X", x.clone())].into_iter().collect();

    let k = 3;
    let pairs = topk(input("X", &[n], Dtype::F32), 0usize, k);
    let names: Vec<(NodeRef, &'static str)> = pairs
        .iter()
        .enumerate()
        .flat_map(|(r, (v, i))| {
            let vn: &'static str = Box::leak(format!("v{r}").into_boxed_str());
            let in_: &'static str = Box::leak(format!("i{r}").into_boxed_str());
            [(v.clone(), vn), (i.clone(), in_)]
        })
        .collect();
    let sched = partition_many(&names, &DeviceProfile::toy());

    let mut run_env = env.clone();
    sched.execute_env(&mut run_env);

    let mut order: Vec<(f64, usize)> = x.data.iter().copied().zip(0..).collect();
    order.sort_by(|a, b| b.0.total_cmp(&a.0));
    for (r, &(expected_value, expected_index)) in order.iter().take(k).enumerate() {
        let v = run_env.get(format!("v{r}").as_str()).unwrap().data[0];
        let i = run_env.get(format!("i{r}").as_str()).unwrap().data[0];
        assert_eq!(v, expected_value, "scheduled value of round {r}");
        assert_eq!(i, expected_index as f64, "scheduled index of round {r}");
    }
}

// All ranks assembled over a rank axis. This checks the frontend composition;
// it deliberately makes no kernel-count promise while carrier inference for
// bounded ordered selection remains open compiler work.
#[test]
fn topk_all_composition_matches_and_schedules() {
    let k = 4usize;
    let (n, rk) = (axis("n", 16), axis("rk", k));
    let mut rng = Lcg(0xA11);
    let x = rand_tensor(&[n], &mut rng);
    let env: Env = [("X", x.clone())].into_iter().collect();

    let scores = input("X", &[n], Dtype::F32);
    let all_indices = topk_all(scores.clone(), 0usize, k, rk, true);
    let all_values = topk_all(scores, 0usize, k, rk, false);

    let expected_indices = eval(&all_indices, &env);
    let expected_values = eval(&all_values, &env);
    let mut order: Vec<(f64, usize)> = x.data.iter().copied().zip(0..).collect();
    order.sort_by(|a, b| b.0.total_cmp(&a.0));
    for (r, &(expected_value, expected_index)) in order.iter().take(k).enumerate() {
        assert_eq!(
            expected_indices.at_index(&[r]),
            expected_index as f64,
            "index at rank {r}"
        );
        assert_eq!(
            expected_values.at_index(&[r]),
            expected_value,
            "value at rank {r}"
        );
    }

    // The generic graph still runs through the ordinary partitioned pipeline.
    let sched = partition_many(
        &[(all_values, "values"), (all_indices, "indices")],
        &DeviceProfile::toy(),
    );
    let mut run_env = env.clone();
    sched.execute_env(&mut run_env);
    let values = &run_env["values"];
    let indices = &run_env["indices"];
    for (r, &(expected_value, expected_index)) in order.iter().take(k).enumerate() {
        assert_eq!(values.at_index(&[r]), expected_value, "scheduled value {r}");
        assert_eq!(
            indices.at_index(&[r]),
            expected_index as f64,
            "scheduled index {r}"
        );
    }
}

// Planted exact ties: the all-ranks frontend composition preserves the same
// first-max-wins behavior as its constituent per-rank folds.
#[test]
fn topk_all_ties_match_per_rank_folds() {
    let k = 3usize;
    let (n, rk) = (axis("n", 8), axis("rk", k));
    // ties everywhere: [2, 5, 5, 1, 5, 2, 0, 5] — ranks 0..3 are all the 5s,
    // first-seen order 1, 2, 4
    let data = [2.0, 5.0, 5.0, 1.0, 5.0, 2.0, 0.0, 5.0];
    let x = Value::from_shape_fn(&[8], |coordinate| data[coordinate[0]]);
    let env: Env = [("X", x)].into_iter().collect();

    let all = topk_all(input("X", &[n], Dtype::F32), 0usize, k, rk, true);
    let got = eval(&all, &env);
    let pairs = topk(input("X", &[n], Dtype::F32), 0usize, k);
    for (r, (_, i)) in pairs.iter().enumerate() {
        assert_eq!(
            got.at_index(&[r]),
            eval(i, &env).data[0],
            "rank {r} under ties"
        );
    }
    assert_eq!(got.at_index(&[0]), 1.0);
    assert_eq!(got.at_index(&[1]), 2.0);
    assert_eq!(got.at_index(&[2]), 4.0);
}

// Batched top-1 (the MoE router shape): argmax per row.
#[test]
fn batched_top1_routes_rows() {
    let (b, e) = (axis("b", 5), axis("e", 8));
    let mut rng = Lcg(0x40E);
    let gates = rand_tensor(&[b, e], &mut rng);
    let env: Env = [("G", gates.clone())].into_iter().collect();

    let pairs = topk(input("G", &[b, e], Dtype::F32), 1usize, 1);
    let idx = eval(&pairs[0].1, &env);
    for bi in 0..5 {
        let mut best = 0usize;
        for ei in 1..8 {
            let c = |e_i| gates.at_index(&[bi, e_i]);
            if c(ei) > c(best) {
                best = ei;
            }
        }
        let got = idx.at_index(&[bi]);
        assert_eq!(got, best as f64, "row {bi}");
    }
}

#[test]
#[should_panic(expected = "topk requires k >= 1")]
fn topk_rejects_an_empty_selection() {
    let n = axis("n", 4);
    let _ = topk(input("X", &[n], Dtype::F32), 0usize, 0);
}

#[test]
#[should_panic(expected = "topk_all rank axis extent must equal k")]
fn topk_all_requires_one_rank_axis_position_per_result() {
    let n = axis("n", 4);
    let ranks = axis("ranks", 3);
    let _ = topk_all(input("X", &[n], Dtype::F32), 0usize, 2, ranks, true);
}

// ── scatter-add: the inverse of gather, collisions summed ────────────────────

#[test]
fn scatter_add_matches_hand_with_collisions() {
    let (i, j, d) = (axis("i", 7), axis("j", 4), axis("d", 3));
    let mut rng = Lcg(0x5CA7);
    let src = rand_tensor(&[i, d], &mut rng);
    // indices with deliberate collisions and a hole (nothing maps to 2)
    let idx_vals = [0usize, 1, 1, 3, 0, 1, 3];
    let idx = Value::from_shape_fn(&[7], |coordinate| idx_vals[coordinate[0]] as f64);
    let env: Env = [("S", src.clone()), ("idx", idx)].into_iter().collect();

    let sc = scatter_add(
        input("S", &[i, d], Dtype::F32),
        input("idx", &[i], Dtype::F32),
        0usize,
        j,
    );
    let got = eval(&sc, &env);

    let hand = Value::from_shape_fn(&[4, 3], |coordinate| {
        idx_vals
            .iter()
            .enumerate()
            .filter(|&(_, &jj)| jj == coordinate[0])
            .map(|(ii, _)| src.at_index(&[ii, coordinate[1]]))
            .sum()
    });
    assert_eq!(got.shape, hand.shape);
    for (a, b) in got.data.iter().zip(&hand.data) {
        assert!((a - b).abs() < 1e-12, "{a} vs {b}");
    }

    // and through the pipeline: one fused kernel (a one-hot contraction)
    let sched = partition(&sc, &DeviceProfile::toy());
    assert_eq!(
        sched.stages.len(),
        1,
        "scatter-add is one fold:\n{}",
        sched.render()
    );
    let executed = sched.execute(&env);
    let exec_p = executed.permuted_to(&got.axes);
    for (a, b) in got.data.iter().zip(&exec_p.data) {
        assert!((a - b).abs() < 1e-12);
    }
}

// scatter_add(gather(x)) with a permutation index is the identity — the
// adjointness that makes it gather's backward.
#[test]
fn scatter_add_inverts_a_permutation_gather() {
    let (v, s, v2, d) = (axis("v", 5), axis("s", 5), axis("v2", 5), axis("d", 3));
    let mut rng = Lcg(0x1D);
    let table = rand_tensor(&[v, d], &mut rng);
    let perm = [3usize, 0, 4, 1, 2];
    let ids = Value::from_shape_fn(&[5], |coordinate| perm[coordinate[0]] as f64);
    let env: Env = [("T", table.clone()), ("ids", ids)].into_iter().collect();

    let gathered = gather(
        input("T", &[v, d], Dtype::F32),
        input("ids", &[s], Dtype::F32),
        0usize,
    ); // [s, d]
    let back = scatter_add(gathered, input("ids", &[s], Dtype::F32), 0usize, v2); // [v2, d]
    let got = eval(&back, &env);
    for vi in 0..5 {
        for di in 0..3 {
            assert!((got.at_index(&[vi, di]) - table.at_index(&[vi, di])).abs() < 1e-12);
        }
    }
}
