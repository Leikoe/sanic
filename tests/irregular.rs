//! M7 — the irregular tail: top-k and scatter, as compositions of the
//! existing basis (no new node kinds), verified against hand references and
//! through the partitioned pipeline.
//!
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

use std::collections::HashMap;

use sanic::cost::DeviceProfile;
use sanic::interp::{Env, Value, eval};
use sanic::kernel_ir::*;
use sanic::partition::{partition, partition_many};

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

// ── top-k: values and indices, largest first ─────────────────────────────────

#[test]
fn topk_matches_a_hand_sort() {
    let n = axis("n", 12);
    let mut rng = Lcg(0x70B5);
    let x = rand_tensor(&[n], &mut rng); // continuous → no ties
    let env: Env = [("X", x.clone())].into_iter().collect();

    let k = 3;
    let pairs = topk(input("X", &[n], Dtype::F32), n, k);

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
    let pairs = topk(input("X", &[n], Dtype::F32), n, k);
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
    let all_indices = topk_all(scores.clone(), n, k, rk, true);
    let all_values = topk_all(scores, n, k, rk, false);

    let expected_indices = eval(&all_indices, &env);
    let expected_values = eval(&all_values, &env);
    let mut order: Vec<(f64, usize)> = x.data.iter().copied().zip(0..).collect();
    order.sort_by(|a, b| b.0.total_cmp(&a.0));
    for (r, &(expected_value, expected_index)) in order.iter().take(k).enumerate() {
        let coord: HashMap<Axis, usize> = [(rk, r)].into_iter().collect();
        assert_eq!(
            expected_indices.at(&coord),
            expected_index as f64,
            "index at rank {r}"
        );
        assert_eq!(
            expected_values.at(&coord),
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
        let coord: HashMap<Axis, usize> = [(rk, r)].into_iter().collect();
        assert_eq!(values.at(&coord), expected_value, "scheduled value {r}");
        assert_eq!(
            indices.at(&coord),
            expected_index as f64,
            "scheduled index {r}"
        );
    }
}

// Planted exact ties: the shared-slot fold keeps first-max-wins across the
// WHOLE selection, exactly like the per-rank folds it replaces.
#[test]
fn topk_all_ties_match_per_rank_folds() {
    let k = 3usize;
    let (n, rk) = (axis("n", 8), axis("rk", k));
    // ties everywhere: [2, 5, 5, 1, 5, 2, 0, 5] — ranks 0..3 are all the 5s,
    // first-seen order 1, 2, 4
    let data = [2.0, 5.0, 5.0, 1.0, 5.0, 2.0, 0.0, 5.0];
    let x = Value::from_fn(&[n], |c| data[c[&n]]);
    let env: Env = [("X", x)].into_iter().collect();

    let all = topk_all(input("X", &[n], Dtype::F32), n, k, rk, true);
    let got = eval(&all, &env);
    let pairs = topk(input("X", &[n], Dtype::F32), n, k);
    for (r, (_, i)) in pairs.iter().enumerate() {
        let coord: HashMap<Axis, usize> = [(rk, r)].into_iter().collect();
        assert_eq!(got.at(&coord), eval(i, &env).data[0], "rank {r} under ties");
    }
    let coord0: HashMap<Axis, usize> = [(rk, 0)].into_iter().collect();
    let coord1: HashMap<Axis, usize> = [(rk, 1)].into_iter().collect();
    let coord2: HashMap<Axis, usize> = [(rk, 2)].into_iter().collect();
    assert_eq!(got.at(&coord0), 1.0);
    assert_eq!(got.at(&coord1), 2.0);
    assert_eq!(got.at(&coord2), 4.0);
}

// Batched top-1 (the MoE router shape): argmax per row.
#[test]
fn batched_top1_routes_rows() {
    let (b, e) = (axis("b", 5), axis("e", 8));
    let mut rng = Lcg(0x40E);
    let gates = rand_tensor(&[b, e], &mut rng);
    let env: Env = [("G", gates.clone())].into_iter().collect();

    let pairs = topk(input("G", &[b, e], Dtype::F32), e, 1);
    let idx = eval(&pairs[0].1, &env);
    for bi in 0..5 {
        let mut best = 0usize;
        for ei in 1..8 {
            let c = |e_i| {
                let m: HashMap<Axis, usize> = [(b, bi), (e, e_i)].into_iter().collect();
                gates.at(&m)
            };
            if c(ei) > c(best) {
                best = ei;
            }
        }
        let got = idx.at(&[(b, bi)].into_iter().collect());
        assert_eq!(got, best as f64, "row {bi}");
    }
}

#[test]
#[should_panic(expected = "topk requires k >= 1")]
fn topk_rejects_an_empty_selection() {
    let n = axis("n", 4);
    let _ = topk(input("X", &[n], Dtype::F32), n, 0);
}

#[test]
#[should_panic(expected = "topk_all rank axis extent must equal k")]
fn topk_all_requires_one_rank_axis_position_per_result() {
    let n = axis("n", 4);
    let ranks = axis("ranks", 3);
    let _ = topk_all(input("X", &[n], Dtype::F32), n, 2, ranks, true);
}

// ── scatter-add: the inverse of gather, collisions summed ────────────────────

#[test]
fn scatter_add_matches_hand_with_collisions() {
    let (i, j, d) = (axis("i", 7), axis("j", 4), axis("d", 3));
    let mut rng = Lcg(0x5CA7);
    let src = rand_tensor(&[i, d], &mut rng);
    // indices with deliberate collisions and a hole (nothing maps to 2)
    let idx_vals = [0usize, 1, 1, 3, 0, 1, 3];
    let idx = Value::from_fn(&[i], |c| idx_vals[c[&i]] as f64);
    let env: Env = [("S", src.clone()), ("idx", idx)].into_iter().collect();

    let sc = scatter_add(
        input("S", &[i, d], Dtype::F32),
        input("idx", &[i], Dtype::F32),
        i,
        j,
    );
    let got = eval(&sc, &env);

    let hand = Value::from_fn(&[j, d], |c| {
        idx_vals
            .iter()
            .enumerate()
            .filter(|&(_, &jj)| jj == c[&j])
            .map(|(ii, _)| {
                let m: HashMap<Axis, usize> = [(i, ii), (d, c[&d])].into_iter().collect();
                src.at(&m)
            })
            .sum()
    });
    let hand_p = hand.permuted_to(&got.axes);
    assert_eq!(got.shape, hand_p.shape);
    for (a, b) in got.data.iter().zip(&hand_p.data) {
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
    let ids = Value::from_fn(&[s], |c| perm[c[&s]] as f64);
    let env: Env = [("T", table.clone()), ("ids", ids)].into_iter().collect();

    let gathered = gather(
        input("T", &[v, d], Dtype::F32),
        input("ids", &[s], Dtype::F32),
        v,
    ); // [d, s]
    let back = scatter_add(gathered, input("ids", &[s], Dtype::F32), s, v2); // [d, v2]
    let got = eval(&back, &env);
    for vi in 0..5 {
        for di in 0..3 {
            let g: HashMap<Axis, usize> = [(v2, vi), (d, di)].into_iter().collect();
            let t: HashMap<Axis, usize> = [(v, vi), (d, di)].into_iter().collect();
            assert!((got.at(&g) - table.at(&t)).abs() < 1e-12);
        }
    }
}
