//! The completeness oracle: DECLINES become checkable claims.
//!
//! Soundness has always had an oracle — everything `derive` produces is run
//! against `eval`. This file is the missing direction. The argmax and top-k
//! fusions were not found by the theory; they were found by counting kernels
//! against another framework. That could happen because "fusable" was only
//! ever checked one way: nothing ever asked whether a *decline* was correct.
//!
//! "Fusable" has a semantic definition that does not mention the deriver:
//!
//! > h streams in one kernel iff it factors through constant-size state —
//! > there exist `into`, an associative `⊕` and `project` with
//! > `h = project ∘ fold(⊕) ∘ map(into)` (a list homomorphism into a small
//! > carrier). Equivalently, Myhill–Nerode-style: some bounded sketch
//! > `σ(xs)` determines `h(xs ++ ys)` for every suffix `ys`.
//!
//! The right-hand form is testable WITHOUT knowing the carrier. Streams are
//! drawn from a small quantized alphabet, so exact sketch collisions occur
//! by pigeonhole; for each declined program the probe searches for a
//! determining sketch σ among tuples of known bounded-state folds (the
//! classic *tupling* method), by collision testing:
//!
//! * σ-colliding prefix pairs whose h-futures agree on every sampled suffix,
//!   across enough pairs → σ is a *carrier candidate*: if `derive` declines
//!   this function, that is a FUSION MISS — the ledger fails unless the
//!   entry is pinned with the op or rewrite that covers it.
//! * every candidate σ separated by a concrete `(p, q, suffix)` witness →
//!   the decline is *justified relative to the pool*, witness printed.
//!
//! The probe is a detector, not a prover: a pass is strong evidence plus a
//! constructive candidate (σ's components name the slots a carrier needs);
//! the derivation itself — oracle-checked against `eval` — remains the
//! proof. The pool, the alphabet, and the collision budget bound what the
//! probe can see, and those bounds are printed rather than hidden.
//!
//! Both historical misses are pinned below in their original graph form:
//! the probe finds their carriers mechanically — σ = (h, max, len) for
//! argmax, σ = (h, max, argmax, max2, len) for the second-rank index —
//! which is exactly the alarm that would have fired long before a kernel
//! count against MLX did. Run with `--nocapture` to read the ledger.

use std::collections::HashMap;

use sanic::derive::derive;
use sanic::interp::{Env, Value, eval};
use sanic::kernel_ir::*;

// ── streams over a quantized alphabet ────────────────────────────────────────

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    /// One of nine values in [-2, 2] — coarse on purpose: exact collisions
    /// and exact ties must be common, or the probe can conclude nothing.
    fn q(&mut self) -> f64 {
        ((self.next() >> 33) % 9) as f64 * 0.5 - 2.0
    }
}

fn prefixes(rng: &mut Lcg, count: usize) -> Vec<Vec<f64>> {
    (0..count)
        .map(|_| {
            // long enough that a handful of moments cannot pin the whole
            // multiset — the false-carrier artifact of tiny streams
            let len = 5 + (rng.next() >> 40) as usize % 5; // 5..=9
            (0..len).map(|_| rng.q()).collect()
        })
        .collect()
}

fn suffixes(rng: &mut Lcg, count: usize) -> Vec<Vec<f64>> {
    let mut out = vec![Vec::new()]; // the empty suffix pins h(p) itself
    for _ in 0..count {
        let len = 1 + (rng.next() >> 40) as usize % 3; // 1..=3
        out.push((0..len).map(|_| rng.q()).collect());
    }
    out
}

// ── evaluating a program on a concrete stream ────────────────────────────────

/// A syllabus program: a scalar function of one streamed input, given as an
/// IR graph builder so `derive` can be asked about the exact same object.
type Build = fn(NodeRef, Axis) -> NodeRef;

fn run_h(build: Build, xs: &[f64]) -> f64 {
    let n = axis("n", xs.len());
    let g = build(input("X", &[n], Dtype::F32), n);
    let x = xs.to_vec();
    let env: Env = [("X", Value::from_fn(&[n], |c| x[c[&n]]))]
        .into_iter()
        .collect();
    eval(&g, &env).data[0]
}

// ── the sketch pool: known bounded-state stream functions ────────────────────

/// Everything here is itself computable by a small fold — that is what makes
/// a passing σ constructive: its components name the slots a carrier needs.
const POOL: &[(&str, fn(&[f64]) -> f64)] = &[
    ("len", |xs| xs.len() as f64),
    ("sum", |xs| xs.iter().sum()),
    ("sumsq", |xs| xs.iter().map(|v| v * v).sum()),
    ("max", |xs| {
        xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }),
    ("min", |xs| xs.iter().cloned().fold(f64::INFINITY, f64::min)),
    ("argmax", |xs| {
        let mut best = (f64::NEG_INFINITY, 0usize);
        for (i, &v) in xs.iter().enumerate() {
            if v > best.0 {
                best = (v, i);
            }
        }
        best.1 as f64
    }),
    ("max2", |xs| {
        let (mut a, mut b) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
        for &v in xs {
            if v > a {
                b = a;
                a = v;
            } else if v > b {
                b = v;
            }
        }
        b
    }),
    ("dot_iota", |xs| {
        xs.iter().enumerate().map(|(i, &v)| i as f64 * v).sum()
    }),
    ("last", |xs| *xs.last().unwrap()),
];

/// σ tuples: h plus up to this many pool components.
const EXTRA: usize = 4;
/// Distinct-prefix collision pairs required before a σ may pass.
const MIN_PAIRS: usize = 30;

// ── the probe ────────────────────────────────────────────────────────────────

enum Verdict {
    /// A determining sketch was found: dimension bound and component names.
    Carrier(usize, Vec<&'static str>),
    /// Every conclusive candidate was separated; one witness kept.
    Separated {
        p: Vec<f64>,
        q: Vec<f64>,
        suffix: Vec<f64>,
        sigma: Vec<&'static str>,
    },
}

fn subsets_upto(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut out: Vec<Vec<usize>> = vec![vec![]];
    for _ in 0..k {
        let mut next = Vec::new();
        for s in &out {
            let start = s.last().map_or(0, |&v| v + 1);
            for i in start..n {
                let mut t = s.clone();
                t.push(i);
                next.push(t);
            }
        }
        out.extend(next.clone());
        out = {
            let mut seen: Vec<Vec<usize>> = Vec::new();
            for s in out {
                if !seen.contains(&s) {
                    seen.push(s);
                }
            }
            seen
        };
    }
    out.sort_by_key(|s| s.len());
    out
}

/// Search for a sketch σ = (h, pool components…) such that σ-colliding
/// prefixes have identical h-futures on every sampled suffix.
fn probe(build: Build) -> Verdict {
    let mut rng = Lcg(0x5EED5);
    let pres = prefixes(&mut rng, 2000);
    let sufs = suffixes(&mut rng, 23);

    // h-futures per prefix, computed once: futures[p][s] = h(pres[p] ++ sufs[s])
    let futures: Vec<Vec<f64>> = pres
        .iter()
        .map(|p| {
            sufs.iter()
                .map(|s| {
                    let mut xs = p.clone();
                    xs.extend_from_slice(s);
                    run_h(build, &xs)
                })
                .collect()
        })
        .collect();

    let mut last_witness: Option<(usize, usize, usize, Vec<&'static str>)> = None;
    'sigma: for cand in subsets_upto(POOL.len(), EXTRA) {
        let names: Vec<&'static str> = cand.iter().map(|&i| POOL[i].0).collect();
        // bucket prefixes by the exact bits of (h(p), σ components)
        let mut buckets: HashMap<Vec<u64>, Vec<usize>> = HashMap::new();
        for (i, p) in pres.iter().enumerate() {
            let mut key = vec![futures[i][0].to_bits()];
            key.extend(cand.iter().map(|&c| POOL[c].1(p).to_bits()));
            buckets.entry(key).or_default().push(i);
        }
        let mut pairs = 0usize;
        for members in buckets.values() {
            for w in members.windows(2) {
                let (a, b) = (w[0], w[1]);
                if pres[a] == pres[b] {
                    continue; // identical streams prove nothing
                }
                pairs += 1;
                for s in 0..sufs.len() {
                    let (fa, fb) = (futures[a][s], futures[b][s]);
                    if (fa - fb).abs() > 1e-9 * (1.0 + fa.abs().max(fb.abs())) {
                        last_witness = Some((a, b, s, names.clone()));
                        continue 'sigma; // separated — next candidate
                    }
                }
            }
        }
        if pairs >= MIN_PAIRS {
            let mut sigma = vec!["h"];
            sigma.extend(names);
            return Verdict::Carrier(1 + cand.len(), sigma);
        }
        // too few conclusive collisions: this σ proves nothing either way
    }
    let (a, b, s, sigma) = last_witness
        .expect("no candidate sketch ever produced a conclusive collision — alphabet too fine?");
    Verdict::Separated {
        p: pres[a].clone(),
        q: pres[b].clone(),
        suffix: sufs[s].clone(),
        sigma,
    }
}

// ── the syllabus ─────────────────────────────────────────────────────────────

#[derive(PartialEq, Clone, Copy, Debug)]
enum Expect {
    /// `derive` succeeds AND the probe confirms a small carrier.
    Derived,
    /// `derive` declines this GRAPH; the probe finds the carrier; the algebra
    /// covers the function through the op named here. Pinned history.
    CoveredElsewhere(&'static str),
    /// `derive` declines and the probe must produce a separating witness.
    JustifiedDecline,
}

fn indicator_lt(a: NodeRef, b: NodeRef) -> NodeRef {
    map(MapOp::Lt, vec![a, b])
}

fn syllabus() -> Vec<(&'static str, Build, Expect)> {
    use Expect::*;
    vec![
        (
            "sum",
            (|x, n| reduce(x, n, BinOp::Monoid(Monoid::Add))) as Build,
            Derived,
        ),
        (
            "max",
            |x, n| reduce(x, n, BinOp::Monoid(Monoid::Max)),
            Derived,
        ),
        (
            "sumsq",
            |x, n| {
                reduce(
                    map(MapOp::Mul, vec![x.clone(), x]),
                    n,
                    BinOp::Monoid(Monoid::Add),
                )
            },
            Derived,
        ),
        (
            "lse",
            |x, n| reduce(x, n, BinOp::Monoid(Monoid::LogSumExp)),
            Derived,
        ),
        (
            "dot_iota",
            |x, n| {
                reduce(
                    map(MapOp::Mul, vec![iota(n), x]),
                    n,
                    BinOp::Monoid(Monoid::Add),
                )
            },
            Derived,
        ),
        ("argmax_op", |x, n| argmax(x, n), Derived),
        (
            "top2_val_op",
            |x, n| {
                reduce(
                    x,
                    n,
                    BinOp::TopK {
                        k: 2,
                        rank: 1,
                        idx: false,
                    },
                )
            },
            Derived,
        ),
        (
            "top2_idx_op",
            |x, n| {
                reduce(
                    x,
                    n,
                    BinOp::TopK {
                        k: 2,
                        rank: 1,
                        idx: true,
                    },
                )
            },
            Derived,
        ),
        // ── the two historical misses, pinned in their original graph form ──
        (
            "argmax_graph",
            |x, n| {
                // Σ i·[x == max x] — how argmax was spelled before the op
                // existed: two kernels, and unsound to fuse (ties sum).
                let m = reduce(x.clone(), n, BinOp::Monoid(Monoid::Max));
                let ge = map(
                    MapOp::Sub,
                    vec![konst(1.0), indicator_lt(x.clone(), m)], // x ≥ max ⇔ ¬(x < max)
                );
                reduce(
                    map(MapOp::Mul, vec![iota(n), ge]),
                    n,
                    BinOp::Monoid(Monoid::Add),
                )
            },
            CoveredElsewhere("BinOp::ArgMax — the (max, idx) tuple monoid"),
        ),
        (
            "top2_idx_graph",
            |x, n| {
                // mask-the-winner round two: argmax over x with the first
                // winner's position forced to −∞
                let first = argmax(x.clone(), n);
                let masked = map(
                    MapOp::Where,
                    vec![one_hot(n, first), konst(f64::NEG_INFINITY), x],
                );
                argmax(masked, n)
            },
            CoveredElsewhere("BinOp::TopK — sequential k-best insertion (full monoid merge TODO)"),
        ),
        // ── genuinely outside constant-state streaming ───────────────────────
        ("median", median_graph, JustifiedDecline),
        (
            "count_above_half_max",
            |x, n| {
                // Σ [x_i > max/2]: the threshold is only known at the END —
                // any future element can move it, so the count needs the
                // whole histogram, not constant state.
                let m = reduce(x.clone(), n, BinOp::Monoid(Monoid::Max));
                let half = map(MapOp::Mul, vec![m, konst(0.5)]);
                reduce(indicator_lt(half, x), n, BinOp::Monoid(Monoid::Add))
            },
            JustifiedDecline,
        ),
    ]
}

/// Lower median, tie-safe: the element with at most ⌊(len−1)/2⌋ strictly
/// smaller and at most ⌈(len−1)/2⌉ strictly greater values. Rank counting
/// needs the whole stream per element — the shape of a true non-fold.
fn median_graph(x: NodeRef, n: Axis) -> NodeRef {
    let j = axis("mj", n.extent());
    let xj = view(x.clone(), vec![(vec![n], j)]);
    let ones = map(MapOp::Mul, vec![xj.clone(), konst(0.0)]);
    let len = reduce(
        map(MapOp::Add, vec![ones, konst(1.0)]),
        j,
        BinOp::Monoid(Monoid::Add),
    );
    let n_less = reduce(
        indicator_lt(xj.clone(), x.clone()),
        j,
        BinOp::Monoid(Monoid::Add),
    );
    let n_greater = reduce(indicator_lt(x.clone(), xj), j, BinOp::Monoid(Monoid::Add));
    // integer ranks vs half-integer thresholds: rank ≤ t ⇔ rank < t + 0.5
    let m_lo = map(
        MapOp::Mul,
        vec![map(MapOp::Sub, vec![len.clone(), konst(1.0)]), konst(0.5)],
    );
    let hi_bound = map(
        MapOp::Sub,
        vec![len, map(MapOp::Add, vec![m_lo.clone(), konst(0.5)])],
    );
    let ok_lo = indicator_lt(n_less, map(MapOp::Add, vec![m_lo, konst(0.5)]));
    let ok_hi = indicator_lt(n_greater, map(MapOp::Add, vec![hi_bound, konst(0.5)]));
    let hit = map(MapOp::Mul, vec![ok_lo, ok_hi]);
    let picked = reduce(
        map(MapOp::Mul, vec![x, hit.clone()]),
        n,
        BinOp::Monoid(Monoid::Add),
    );
    let count = reduce(hit, n, BinOp::Monoid(Monoid::Add));
    map(MapOp::Div, vec![picked, count])
}

// ── the ledger ───────────────────────────────────────────────────────────────

#[test]
fn every_decline_is_justified_or_pinned() {
    let n = axis("n", 8);
    let mut report = String::from(
        "\nLEDGER — semantic carrier probe vs the deriver\n\
         (pool: 9 bounded-state folds; sketches h + ≤4 slots; alphabet 9 × [-2,2];\n\
          2000 prefixes × 24 suffixes; a σ passes only on ≥30 distinct collisions)\n\n",
    );
    let mut failures = Vec::new();

    for (name, build, expect) in syllabus() {
        // One pass over the RAW stream: the root fold derives AND no leaf
        // hides a producer fold. (`derive` alone answers a weaker question —
        // "does the root stream over its leaves" — median's rank reduces,
        // say, are legal leaves but each is its own kernel.)
        let derived = derive(&build(input("X", &[n], Dtype::F32), n), n)
            .filter(|c| c.leaves.iter().all(|l| !contains_fold(l)));
        let verdict = probe(build);
        let line = match (&derived, &verdict, expect) {
            (Some(c), Verdict::Carrier(dim, sigma), Expect::Derived) => format!(
                "  DERIVED     {name:22} {} slot(s); probe agrees: dim ≤ {dim}, σ = ({})\n",
                c.slots,
                sigma.join(", ")
            ),
            (None, Verdict::Carrier(dim, sigma), Expect::CoveredElsewhere(op)) => format!(
                "  GRAPH-FORM  {name:22} graph declines; carrier exists (dim ≤ {dim}, σ = ({})) — covered by {op}\n",
                sigma.join(", ")
            ),
            (
                None,
                Verdict::Separated {
                    p,
                    q,
                    suffix,
                    sigma,
                },
                Expect::JustifiedDecline,
            ) => format!(
                "  JUSTIFIED   {name:22} every sketch separated — e.g. σ = ({}) \
                 collides on {p:?} / {q:?}, split by suffix {suffix:?}\n",
                sigma.join(", ")
            ),
            (d, v, e) => {
                let got = match (d.is_some(), v) {
                    (true, Verdict::Carrier(dim, s)) => {
                        format!("derives; probe dim ≤ {dim} via ({})", s.join(", "))
                    }
                    (true, Verdict::Separated { .. }) => {
                        "derives, but the probe separated every sketch (pool too weak)".into()
                    }
                    (false, Verdict::Carrier(dim, s)) => format!(
                        "DECLINED, but σ = ({}) is a dim-≤{dim} carrier — FUSION MISS",
                        s.join(", ")
                    ),
                    (false, Verdict::Separated { .. }) => "declined; probe separated".into(),
                };
                failures.push(format!("{name}: expected {e:?}; got: {got}"));
                format!("  !! MISMATCH {name:22} {got}\n")
            }
        };
        report.push_str(&line);
    }

    println!("{report}");
    assert!(
        failures.is_empty(),
        "the fusability frontier moved — each entry is either a fusion the \
         algebra misses or a stale pin:\n{}",
        failures.join("\n")
    );
}

/// Random small programs the generator has never seen: any DECLINE the probe
/// can refute must be pinned here with an explanation, or the test fails.
/// This is the standing tripwire that replaces "we'll notice in a benchmark".
#[test]
fn random_declines_survive_the_probe() {
    let n = axis("n", 8);
    // Discovered-by-this-harness gaps, pinned deliberately (seed → note).
    // An empty map is the goal state; entries here are open, EXPLAINED work.
    // This sweep's first run flagged THIRTEEN seeds; ten became the
    // invariant / lattice / defer-add / defer-scale rules in `derive`.
    let pinned: HashMap<u64, &str> = [
        (
            7u64,
            "alphabet-limited evidence: max_i min(sumsq+i, x_i) — the min is              dominated on this alphabet, hiding the per-element dependence;              no carrier claimed",
        ),
        (
            13,
            "closed-form iota sums: Σ max(c1−i, c2) is piecewise-affine in i              (triangular numbers) — a symbolic-summation rewrite family,              deliberately outside the carrier algebra for now",
        ),
        (
            23,
            "closed-form iota sums, same family as seed 13: Σ (max(i, c1) − c2)",
        ),
    ]
    .into_iter()
    .collect();

    let mut new_misses = Vec::new();
    let mut ledger = String::from("\nRANDOM SWEEP — declined programs vs the probe\n\n");
    for seed in 0..24u64 {
        SEED.with(|s| s.set(seed));
        let build: Build = |x, n| random_program(x, n, SEED.with(|s| s.get()));
        let g = build(input("X", &[n], Dtype::F32), n);
        let one_pass = derive(&g, n).is_some_and(|c| c.leaves.iter().all(|l| !contains_fold(l)));
        if one_pass {
            continue; // sound by the existing oracle; nothing to check
        }
        match probe(build) {
            Verdict::Carrier(dim, sigma) => {
                let line = format!(
                    "  seed {seed:2}: declined, σ = ({}) is a dim-≤{dim} carrier — {}\n",
                    sigma.join(", "),
                    pinned.get(&seed).copied().unwrap_or("UNPINNED MISS")
                );
                ledger.push_str(&line);
                if !pinned.contains_key(&seed) {
                    new_misses.push(line);
                }
            }
            Verdict::Separated { .. } => {
                ledger.push_str(&format!("  seed {seed:2}: declined, justified\n"));
            }
        }
    }
    println!("{ledger}");
    assert!(
        new_misses.is_empty(),
        "random sweep found declined-but-fusable programs — extend the algebra \
         or pin them with an explanation:\n{}",
        new_misses.join("")
    );
}

thread_local! {
    static SEED: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Small expression over (x, iota, consts, one nested same-axis fold),
/// reduced at the top — the shape real workloads decline in.
fn random_program(x: NodeRef, n: Axis, seed: u64) -> NodeRef {
    let mut s = Lcg(seed.wrapping_mul(0x9E3779B97F4A7C15) | 1);
    let leaf = |s: &mut Lcg, x: &NodeRef| -> NodeRef {
        match (s.next() >> 30) % 4 {
            0 => x.clone(),
            1 => iota(n),
            2 => konst(((s.next() >> 32) % 5) as f64 * 0.5 - 1.0),
            _ => match (s.next() >> 31) % 3 {
                0 => reduce(x.clone(), n, BinOp::Monoid(Monoid::Add)),
                1 => reduce(x.clone(), n, BinOp::Monoid(Monoid::Max)),
                _ => reduce(
                    map(MapOp::Mul, vec![x.clone(), x.clone()]),
                    n,
                    BinOp::Monoid(Monoid::Add),
                ),
            },
        }
    };
    let op = |s: &mut Lcg| match (s.next() >> 29) % 5 {
        0 => MapOp::Add,
        1 => MapOp::Sub,
        2 => MapOp::Mul,
        3 => MapOp::Max,
        _ => MapOp::Min,
    };
    let a = leaf(&mut s, &x);
    let b = leaf(&mut s, &x);
    let c = leaf(&mut s, &x);
    let o1 = op(&mut s);
    let o2 = op(&mut s);
    let e = map(o2, vec![map(o1, vec![a, b]), c]);
    let top = match (s.next() >> 28) % 3 {
        0 => BinOp::Monoid(Monoid::Add),
        1 => BinOp::Monoid(Monoid::Max),
        _ => BinOp::Monoid(Monoid::Min),
    };
    reduce(e, n, top)
}

fn contains_fold(node: &NodeRef) -> bool {
    match node.as_ref() {
        Node::Reduce { .. } | Node::Scan { .. } => true,
        Node::Input { .. } | Node::Const { .. } | Node::Iota { .. } => false,
        Node::Map { inputs, .. } => inputs.iter().any(contains_fold),
        Node::View { src, .. } | Node::Reindex { src, .. } => contains_fold(src),
        Node::Gather { src, index, .. } => contains_fold(src) || contains_fold(index),
    }
}

/// Inspection helper, not a gate: prints the flagged programs.
#[test]
#[ignore]
fn inspect_flagged_seeds() {
    let n = axis("n", 8);
    for seed in [7u64, 13, 23] {
        SEED.with(|s| s.set(seed));
        let build: Build = |x, n| random_program(x, n, SEED.with(|s| s.get()));
        let g = build(input("X", &[n], Dtype::F32), n);
        println!("seed {seed}: derives={} {:?}\n", derive(&g, n).is_some(), g);
    }
}
