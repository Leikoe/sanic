//! M7 — the irregular tail: top-k and scatter, as compositions of the
//! existing basis (no new node kinds), verified against hand references and
//! through the partitioned pipeline.
//!
//! * `topk` — k rounds of (max, mask-the-winner). Small k is all sampling
//!   and MoE routing ever need, and each round is an ordinary fold.
//! * `scatter_add` — the inverse of gather as a one-hot contraction,
//!   add-combining collisions. Order-free (deterministic in parallel), and
//!   exactly the backward of `gather`, which autodiff (M8) leans on.
//!
//! Full sort is *declined* deliberately: a data-movement network, not a
//! fold — outside the algebra this compiler trusts, and unneeded for
//! inference.

use std::collections::HashMap;

use sanic::cost::Device;
use sanic::interp::{Env, Extents, Tensor, eval};
use sanic::ir::*;
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
fn rand_tensor(axes: &[Axis], ext: &Extents, rng: &mut Lcg) -> Tensor {
    Tensor::from_fn(axes, ext, |_| rng.f())
}
fn as_f64(ext: &Extents) -> HashMap<Axis, f64> {
    ext.iter().map(|(&a, &n)| (a, n as f64)).collect()
}

// ── top-k: values and indices, largest first ─────────────────────────────────

#[test]
fn topk_matches_a_hand_sort() {
    let n = axis("n");
    let ext: Extents = [(n, 12)].into_iter().collect();
    let mut rng = Lcg(0x70B5);
    let x = rand_tensor(&[n], &ext, &mut rng); // continuous → no ties
    let env: Env = [("X", x.clone())].into_iter().collect();

    let k = 3;
    let pairs = topk(input("X", &[n]), n, k);

    // hand reference: sort (value, index) descending by value
    let mut order: Vec<(f64, usize)> = x.data.iter().copied().zip(0..).collect();
    order.sort_by(|a, b| b.0.total_cmp(&a.0));

    for (round, (v, i)) in pairs.iter().enumerate() {
        let got_v = eval(v, &env, &ext).data[0];
        let got_i = eval(i, &env, &ext).data[0];
        assert_eq!(got_v, order[round].0, "value of round {round}");
        assert_eq!(got_i, order[round].1 as f64, "index of round {round}");
    }
}

// The whole top-k chain as ONE multi-output schedule: later rounds read the
// earlier rounds' materialized max/argmax through the entangler cuts, and the
// executed schedule reproduces the reference.
#[test]
fn topk_partitions_and_executes() {
    let n = axis("n");
    let ext: Extents = [(n, 16)].into_iter().collect();
    let mut rng = Lcg(0x70B52);
    let x = rand_tensor(&[n], &ext, &mut rng);
    let env: Env = [("X", x.clone())].into_iter().collect();

    let k = 3;
    let pairs = topk(input("X", &[n]), n, k);
    let names: Vec<(Node, &'static str)> = pairs
        .iter()
        .enumerate()
        .flat_map(|(r, (v, i))| {
            let vn: &'static str = Box::leak(format!("v{r}").into_boxed_str());
            let in_: &'static str = Box::leak(format!("i{r}").into_boxed_str());
            [(v.clone(), vn), (i.clone(), in_)]
        })
        .collect();
    let sched = partition_many(&names, &Device::toy(), &as_f64(&ext));

    let mut run_env = env.clone();
    sched.execute_env(&mut run_env, &ext);

    let mut order: Vec<(f64, usize)> = x.data.iter().copied().zip(0..).collect();
    order.sort_by(|a, b| b.0.total_cmp(&a.0));
    for r in 0..k {
        let v = run_env.get(format!("v{r}").as_str()).unwrap().data[0];
        let i = run_env.get(format!("i{r}").as_str()).unwrap().data[0];
        assert_eq!(v, order[r].0, "scheduled value of round {r}");
        assert_eq!(i, order[r].1 as f64, "scheduled index of round {r}");
    }
}

// ALL ranks of one selection as ONE fold: the rank axis is grid, the k-best
// list is derived ONCE (slots deduped across the rank queries), and the
// projection reads the rank one-hot at project time — the composition
// `topk_all` builds and the derive rule `project-leaf` makes streamable.
#[test]
fn topk_all_is_one_fold_with_shared_slots() {
    let (n, rk) = (axis("n"), axis("rk"));
    let k = 4usize;
    let ext: Extents = [(n, 16), (rk, k)].into_iter().collect();
    let mut rng = Lcg(0xA11);
    let x = rand_tensor(&[n], &ext, &mut rng);
    let env: Env = [("X", x.clone())].into_iter().collect();

    let all = topk_all(input("X", &[n]), n, k, rk, true);

    // one carrier: k value slots + k index slots, not k separate lists
    let c = sanic::derive::derive(&all, n).expect("topk_all derives over the scored axis");
    assert_eq!(c.slots, 2 * k, "k-best slots shared across rank queries");
    assert!(c.project_reads_leaves(), "rank one-hot read at project time");
    assert!(c.rules.contains(&"project-leaf"), "rules: {:?}", c.rules);

    // the carrier equals the naive semantics, per rank
    let got = sanic::interp::run_carrier(&all, n, &c, &env, &ext);
    let want = eval(&all, &env, &ext);
    let mut order: Vec<(f64, usize)> = x.data.iter().copied().zip(0..).collect();
    order.sort_by(|a, b| b.0.total_cmp(&a.0));
    for r in 0..k {
        let coord: HashMap<Axis, usize> = [(rk, r)].into_iter().collect();
        assert_eq!(got.at(&coord), want.at(&coord), "rank {r}: carrier == eval");
        assert_eq!(got.at(&coord), order[r].1 as f64, "rank {r}: == hand sort");
    }

    // and the whole thing schedules as ONE kernel
    let sched = partition(&all, &Device::toy(), &as_f64(&ext));
    assert_eq!(sched.stages.len(), 1, "one fold for all ranks");
    let mut run_env = env.clone();
    sched.execute_env(&mut run_env, &ext);
    let out = run_env.get(sched.outputs[0].as_str()).unwrap();
    for r in 0..k {
        let coord: HashMap<Axis, usize> = [(rk, r)].into_iter().collect();
        assert_eq!(out.at(&coord), order[r].1 as f64, "scheduled rank {r}");
    }
}

// Planted exact ties: the shared-slot fold keeps first-max-wins across the
// WHOLE selection, exactly like the per-rank folds it replaces.
#[test]
fn topk_all_ties_match_per_rank_folds() {
    let (n, rk) = (axis("n"), axis("rk"));
    let k = 3usize;
    let ext: Extents = [(n, 8), (rk, k)].into_iter().collect();
    // ties everywhere: [2, 5, 5, 1, 5, 2, 0, 5] — ranks 0..3 are all the 5s,
    // first-seen order 1, 2, 4
    let data = [2.0, 5.0, 5.0, 1.0, 5.0, 2.0, 0.0, 5.0];
    let x = Tensor::from_fn(&[n], &ext, |c| data[c[&n]]);
    let env: Env = [("X", x)].into_iter().collect();

    let all = topk_all(input("X", &[n]), n, k, rk, true);
    let got = eval(&all, &env, &ext);
    let pairs = topk(input("X", &[n]), n, k);
    for (r, (_, i)) in pairs.iter().enumerate() {
        let coord: HashMap<Axis, usize> = [(rk, r)].into_iter().collect();
        assert_eq!(
            got.at(&coord),
            eval(i, &env, &ext).data[0],
            "rank {r} under ties"
        );
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
    let (b, e) = (axis("b"), axis("e"));
    let ext: Extents = [(b, 5), (e, 8)].into_iter().collect();
    let mut rng = Lcg(0x40E);
    let gates = rand_tensor(&[b, e], &ext, &mut rng);
    let env: Env = [("G", gates.clone())].into_iter().collect();

    let pairs = topk(input("G", &[b, e]), e, 1);
    let idx = eval(&pairs[0].1, &env, &ext);
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

// ── scatter-add: the inverse of gather, collisions summed ────────────────────

#[test]
fn scatter_add_matches_hand_with_collisions() {
    let (i, j, d) = (axis("i"), axis("j"), axis("d"));
    let ext: Extents = [(i, 7), (j, 4), (d, 3)].into_iter().collect();
    let mut rng = Lcg(0x5CA7);
    let src = rand_tensor(&[i, d], &ext, &mut rng);
    // indices with deliberate collisions and a hole (nothing maps to 2)
    let idx_vals = [0usize, 1, 1, 3, 0, 1, 3];
    let idx = Tensor::from_fn(&[i], &ext, |c| idx_vals[c[&i]] as f64);
    let env: Env = [("S", src.clone()), ("idx", idx)].into_iter().collect();

    let sc = scatter_add(input("S", &[i, d]), input("idx", &[i]), i, j);
    let got = eval(&sc, &env, &ext);

    let hand = Tensor::from_fn(&[j, d], &ext, |c| {
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
    let sched = partition(&sc, &Device::toy(), &as_f64(&ext));
    assert_eq!(sched.stages.len(), 1, "scatter-add is one fold:\n{}", sched.render());
    let executed = sched.execute(&env, &ext);
    let exec_p = executed.permuted_to(&got.axes);
    for (a, b) in got.data.iter().zip(&exec_p.data) {
        assert!((a - b).abs() < 1e-12);
    }
}

// scatter_add(gather(x)) with a permutation index is the identity — the
// adjointness that makes it gather's backward.
#[test]
fn scatter_add_inverts_a_permutation_gather() {
    let (v, s, v2, d) = (axis("v"), axis("s"), axis("v2"), axis("d"));
    let ext: Extents = [(v, 5), (s, 5), (v2, 5), (d, 3)].into_iter().collect();
    let mut rng = Lcg(0x1D);
    let table = rand_tensor(&[v, d], &ext, &mut rng);
    let perm = [3usize, 0, 4, 1, 2];
    let ids = Tensor::from_fn(&[s], &ext, |c| perm[c[&s]] as f64);
    let env: Env = [("T", table.clone()), ("ids", ids)].into_iter().collect();

    let gathered = gather(input("T", &[v, d]), input("ids", &[s]), v); // [d, s]
    let back = scatter_add(gathered, input("ids", &[s]), s, v2); // [d, v2]
    let got = eval(&back, &env, &ext);
    for vi in 0..5 {
        for di in 0..3 {
            let g: HashMap<Axis, usize> = [(v2, vi), (d, di)].into_iter().collect();
            let t: HashMap<Axis, usize> = [(v, vi), (d, di)].into_iter().collect();
            assert!((got.at(&g) - table.at(&t)).abs() < 1e-12);
        }
    }
}
