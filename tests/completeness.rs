//! The completeness oracle: DECLINES become checkable claims.
//!
//! Soundness has always had an oracle — everything `derive` produces is run
//! against `eval`. This file is the missing direction. Product-state
//! selection fusions were first found by counting kernels against another
//! framework. That could happen because "fusable" was only ever checked one
//! way: nothing ever asked whether a *decline* was correct.
//!
//! "Fusable" has a semantic definition that does not mention the deriver:
//!
//! > h admits arbitrary chunking and tree reduction iff it factors through a
//! > constant-size recognizing monoid: there exist `into`, an associative
//! > `⊕`, and `project` with
//! > `h = project ∘ fold(⊕) ∘ map(into)`.
//!
//! A necessary (but not equivalent) one-way condition is Myhill–Nerode-style:
//! some bounded sketch `σ(xs)` determines `h(xs ++ ys)` for every suffix
//! `ys`. The suffix quotient is minimal sequential state; a merge carrier must
//! additionally represent chunk actions on that state, or equivalently refine
//! the two-sided syntactic congruence.
//!
//! The right-hand form is testable WITHOUT knowing the carrier. Streams are
//! drawn from a small quantized alphabet, so exact sketch collisions occur
//! by pigeonhole; for each declined program the probe searches for a
//! determining sketch σ among tuples of known bounded-state folds (the
//! classic *tupling* method), by collision testing:
//!
//! * σ-colliding prefix pairs whose h-futures agree on every sampled suffix,
//!   across enough pairs → σ is a *sequential-state candidate*. If `derive`
//!   declines, this is an alarm: synthesize a closed chunk action/combine
//!   before calling it a fusion miss.
//! * every candidate σ separated by a concrete `(p, q, suffix)` witness →
//!   the decline is *justified relative to the pool*, witness printed.
//!
//! The probe is a detector, not a prover: a pass is strong evidence plus a
//! constructive sequential-state candidate. A derivation still has to produce
//! a closed associative combine and pass the oracle checks against `eval`.
//! The pool, alphabet, and collision budget bound what the probe can see, and
//! those bounds are printed rather than hidden.
//!
//! Argmax's generic extremal-key/payload carrier is derived from composition.
//! Bounded ordered selection remains an explicit compiler gap after removing
//! its semantic IR shortcut. The probe finds a strong sequential-state
//! candidate; the independently known ordered-list merge confirms the carrier.
//! That is exactly the alarm that should fire before a kernel-count comparison
//! does. Run with `--nocapture` to read the ledger.
//!
//! The probe's admitted blind spot is its pool. The SECOND oracle at the
//! bottom of this file — the Hankel rank test of the theory doc's §5.7 —
//! needs no sketch pool. It estimates minimal *linear sequential-state*
//! dimension from H[p,s] = h(p·s); it does not by itself measure a general
//! nonlinear carrier or certify associative merging.

use std::collections::HashMap;

use sanic::derive::{AssociativityEvidence, Expr, MergeOrderEvidence, derive};
use sanic::interp::{Env, Value, eval};
use sanic::ir::*;
use sanic::transition::affine_transition_carrier;

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
    /// Continuous in [-2, 2] — the rank test wants generic streams, where the
    /// probe wanted collisions.
    fn c(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64 * 4.0 - 2.0
    }
}

fn prefixes(rng: &mut Lcg, count: usize) -> Vec<Vec<f64>> {
    let mut out: Vec<Vec<f64>> = (0..count)
        .map(|_| {
            // long enough that a handful of moments cannot pin the whole
            // multiset — the false-carrier artifact of tiny streams
            let len = 5 + (rng.next() >> 40) as usize % 5; // 5..=9
            (0..len).map(|_| rng.q()).collect()
        })
        .collect();
    // Adversarial generators random draws under-produce (§3.4): planted
    // ties, permutations of the same multiset (identical to every symmetric
    // sketch — only order-sensitive functions tell them apart), and extreme
    // magnitudes planted early and late. These concentrate collisions where
    // wrong carriers are most likely to hide.
    for i in 0..count / 4 {
        let mut p = out[(rng.next() as usize) % out.len()].clone();
        let len = p.len();
        let a = (rng.next() as usize) % len;
        let b = (rng.next() as usize) % len;
        match i % 4 {
            0 => p[a] = p[b],                               // planted tie
            1 => p.rotate_left(1 + a % (len - 1)),          // same multiset, other order
            2 => p[0] = if i % 8 < 4 { 2.0 } else { -2.0 }, // early extreme
            _ => *p.last_mut().unwrap() = 2.0,              // late extreme
        }
        out.push(p);
    }
    out
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
type Build = fn(NodeRef, usize) -> NodeRef;
type Sketch = fn(&[f64]) -> f64;

fn run_h(build: Build, xs: &[f64]) -> f64 {
    let n = axis("n", xs.len());
    let g = build(input("X", [n]), 0);
    let x = xs.to_vec();
    let env: Env = [("X", Value::from_shape_fn(&[xs.len()], |coordinate| x[coordinate[0]]))]
        .into_iter()
        .collect();
    eval(&g, &env).data[0]
}

// ── the sketch pool: known bounded-state stream functions ────────────────────

/// Everything here is itself computable by a small fold — that is what makes
/// a passing σ constructive as sequential state. A merge may need a larger
/// representation of the chunk action on these components.
const POOL: &[(&str, Sketch)] = &[
    ("len", |xs| xs.len() as f64),
    ("sum", |xs| xs.iter().sum()),
    ("sumsq", |xs| xs.iter().map(|v| v * v).sum()),
    ("max", |xs| xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max)),
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
    ("dot_iota", |xs| xs.iter().enumerate().map(|(i, &v)| i as f64 * v).sum()),
    ("last", |xs| *xs.last().unwrap()),
];

/// σ tuples: h plus up to this many pool components.
const EXTRA: usize = 4;
/// Distinct-prefix collision pairs required before a σ may pass.
const MIN_PAIRS: usize = 30;

// ── the probe ────────────────────────────────────────────────────────────────

enum Verdict {
    /// A determining sequential sketch was found. A survivor is not a merge
    /// carrier until a closed chunk action/combine is constructed. The count
    /// is merely this sketch's component count (argmax's passing σ has three
    /// components while its known product carrier has two slots).
    State(usize, Vec<&'static str>),
    /// Every conclusive candidate was separated; one witness kept, shrunk
    /// to the shortest (p, q, suffix) that still collides and splits.
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
    probe_with(build, 0x5EED5, 2000, 23)
}

/// The probe under an explicit seed and budget: one seed is a single sample
/// of a probabilistic argument (§3.4), so justified declines are re-checked
/// under independent seeds at the same budget — a weaker budget
/// under-separates and manufactures false carrier candidates.
fn probe_with(build: Build, seed: u64, n_pres: usize, n_sufs: usize) -> Verdict {
    let mut rng = Lcg(seed);
    let pres = prefixes(&mut rng, n_pres);
    let sufs = suffixes(&mut rng, n_sufs);

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
                for (s, (&fa, &fb)) in futures[a].iter().zip(&futures[b]).enumerate() {
                    // Separation at the semantics-quotient tolerance (theory
                    // §5.4): the one policy, at the probe's stream length.
                    let tol = sanic::verify::rel_tolerance(Dtype::F64, 12) * (1.0 + fa.abs().max(fb.abs()));
                    if (fa - fb).abs() > tol {
                        last_witness = Some((a, b, s, names.clone()));
                        continue 'sigma; // separated — next candidate
                    }
                }
            }
        }
        if pairs >= MIN_PAIRS {
            let mut sigma = vec!["h"];
            sigma.extend(names);
            return Verdict::State(1 + cand.len(), sigma);
        }
        // too few conclusive collisions: this σ proves nothing either way
    }
    let (a, b, s, sigma) =
        last_witness.expect("no candidate sketch ever produced a conclusive collision — alphabet too fine?");
    let (p, q, suffix) = shrink_witness(build, &sigma, pres[a].clone(), pres[b].clone(), sufs[s].clone());
    Verdict::Separated { p, q, suffix, sigma }
}

/// §3.3: QuickCheck-style shrinking — the shortest (p, q, suffix) that still
/// σ-collides (h and every component, bit-exact) and still splits. What
/// `last_witness` keeps is an arbitrary artifact of search order; the
/// minimal witness is a readable proof sketch.
fn shrink_witness(
    build: Build,
    comp_names: &[&'static str],
    mut p: Vec<f64>,
    mut q: Vec<f64>,
    mut suffix: Vec<f64>,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let comps: Vec<fn(&[f64]) -> f64> = comp_names
        .iter()
        .map(|n| POOL.iter().find(|(pn, _)| pn == n).unwrap().1)
        .collect();
    let witnesses = |p: &[f64], q: &[f64], s: &[f64]| -> bool {
        if p.is_empty() || q.is_empty() || p == q {
            return false;
        }
        if run_h(build, p).to_bits() != run_h(build, q).to_bits() {
            return false;
        }
        if comps.iter().any(|c| c(p).to_bits() != c(q).to_bits()) {
            return false;
        }
        let cat = |xs: &[f64]| {
            let mut v = xs.to_vec();
            v.extend_from_slice(s);
            v
        };
        let (fa, fb) = (run_h(build, &cat(p)), run_h(build, &cat(q)));
        (fa - fb).abs() > sanic::verify::rel_tolerance(Dtype::F64, 12) * (1.0 + fa.abs().max(fb.abs()))
    };
    loop {
        let mut shrunk = false;
        for target in 0..3usize {
            let len = [p.len(), q.len(), suffix.len()][target];
            for i in 0..len {
                let (mut tp, mut tq, mut ts) = (p.clone(), q.clone(), suffix.clone());
                [&mut tp, &mut tq, &mut ts][target].remove(i);
                if witnesses(&tp, &tq, &ts) {
                    (p, q, suffix) = (tp, tq, ts);
                    shrunk = true;
                    break;
                }
            }
        }
        if !shrunk {
            return (p, q, suffix);
        }
    }
}

// ── the syllabus ─────────────────────────────────────────────────────────────

#[derive(PartialEq, Clone, Copy, Debug)]
enum Expect {
    /// `derive` supplies the carrier; the probe independently finds bounded
    /// sequential state.
    Derived,
    /// `derive` declines, the probe finds sequential state, and an independent
    /// construction named in the reason supplies the merge carrier. This is
    /// accepted only as compiler debt, never hidden behind a semantic IR op.
    KnownFusionGap(&'static str),
    /// `derive` declines and the probe must produce a separating witness.
    JustifiedDecline,
}

fn indicator_lt(a: NodeRef, b: NodeRef) -> NodeRef {
    map(MapOp::Lt, vec![a, b])
}

fn syllabus() -> Vec<(&'static str, Build, Expect)> {
    use Expect::*;
    vec![
        ("sum", (|x, n| reduce(x, n, Monoid::Add)) as Build, Derived),
        ("max", |x, n| reduce(x, n, Monoid::Max), Derived),
        (
            "sumsq",
            |x, n| reduce(map(MapOp::Mul, vec![x.clone(), x]), n, Monoid::Add),
            Derived,
        ),
        ("lse", |x, n| reduce(x, n, Monoid::LogSumExp), Derived),
        (
            "dot_iota",
            |x, n| {
                let coordinate = coordinate(x.clone(), n);
                reduce(map(MapOp::Mul, vec![coordinate, x]), n, Monoid::Add)
            },
            Derived,
        ),
        ("argmax_frontend", |x, n| argmax(x, n), Derived),
        (
            "top2_idx_frontend",
            |x, n| topk(x, n, 2)[1].1.clone(),
            KnownFusionGap("derive the bounded ordered-pair carrier from composition"),
        ),
        // ── genuinely outside constant-state streaming ───────────────────────
        ("median", median_graph, JustifiedDecline),
        (
            "count_above_half_max",
            |x, n| {
                // Σ [x_i > max/2]: the threshold is only known at the END —
                // any future element can move it, so the count needs the
                // whole histogram, not constant state.
                let m = reduce(x.clone(), n, Monoid::Max);
                let half = map(MapOp::Mul, vec![m, konst(0.5)]);
                reduce(indicator_lt(half, x), n, Monoid::Add)
            },
            JustifiedDecline,
        ),
    ]
}

/// Lower median, tie-safe: the element with at most ⌊(len−1)/2⌋ strictly
/// smaller and at most ⌈(len−1)/2⌉ strictly greater values. Rank counting
/// needs the whole stream per element — the shape of a true non-fold.
fn median_graph(x: NodeRef, n: usize) -> NodeRef {
    // The comparison traverses the same data under two independent logical
    // occurrences. A clone is still the same occurrence; the explicit view
    // creates the second one without relying on its display descriptor.
    let comparison = rename(x.clone(), n, axis("comparison", x.shape()[n].extent()));
    let xi = unsqueeze(x.clone(), 1usize);
    let xj = unsqueeze(comparison, 0usize);
    let ones = map(MapOp::Mul, vec![xj.clone(), konst(0.0)]);
    let len = reduce(map(MapOp::Add, vec![ones, konst(1.0)]), 1usize, Monoid::Add);
    let n_less = reduce(indicator_lt(xj.clone(), xi.clone()), 1usize, Monoid::Add);
    let n_greater = reduce(indicator_lt(xi, xj), 1usize, Monoid::Add);
    // integer ranks vs half-integer thresholds: rank ≤ t ⇔ rank < t + 0.5
    let m_lo = map(
        MapOp::Mul,
        vec![map(MapOp::Sub, vec![len.clone(), konst(1.0)]), konst(0.5)],
    );
    let hi_bound = map(MapOp::Sub, vec![len, map(MapOp::Add, vec![m_lo.clone(), konst(0.5)])]);
    let ok_lo = indicator_lt(n_less, map(MapOp::Add, vec![m_lo, konst(0.5)]));
    let ok_hi = indicator_lt(n_greater, map(MapOp::Add, vec![hi_bound, konst(0.5)]));
    let hit = map(MapOp::Mul, vec![ok_lo, ok_hi]);
    let picked = reduce(map(MapOp::Mul, vec![x, hit.clone()]), n, Monoid::Add);
    let count = reduce(hit, n, Monoid::Add);
    map(MapOp::Div, vec![picked, count])
}

// ── the ledger ───────────────────────────────────────────────────────────────

/// §3.4: a surviving state candidate is based on one stream sample — seed
/// luck can starve the collisions that would have killed a wrong sketch. It
/// therefore has to survive independent draws at the same budget. (The dual
/// is NOT checked across seeds: a separation is a concrete counterexample.)
fn confirm_state_across_seeds(name: &str, build: Build, failures: &mut Vec<String>) {
    for seed in [0xACE1u64, 0xBEEF] {
        if let Verdict::Separated { p, q, suffix, sigma } = probe_with(build, seed, 2000, 23) {
            failures.push(format!(
                "{name}: probe found sequential state under the primary seed, but seed {seed:#x} \
                 separated every sketch — e.g. σ = ({}) on {p:?} / {q:?} via {suffix:?}",
                sigma.join(", ")
            ));
        }
    }
}

#[test]
fn every_decline_is_justified_or_pinned() {
    let n = axis("n", 8);
    let mut report = String::from(
        "\nLEDGER — sequential-state probe vs the carrier deriver\n\
         (pool: 9 bounded-state folds; sketches h + ≤4 slots; alphabet 9 × [-2,2];\n\
          2000 random + 500 adversarial prefixes × 24 suffixes; a σ passes only on\n\
          ≥30 distinct collisions; state candidates must SURVIVE two more seeds —\n\
          a separation is a self-certifying witness, a survival is only evidence)\n\n",
    );
    let mut failures = Vec::new();

    for (name, build, expect) in syllabus() {
        // One pass over the RAW stream: the root fold derives AND no leaf
        // hides a producer fold. (`derive` alone answers a weaker question —
        // "does the root stream over its leaves" — median's rank reduces,
        // say, are legal leaves but each is its own kernel.)
        let x = input("X", [n]);
        let stream = axis_refs(&x)[0];
        let derived = derive(&build(x, 0), stream)
            .ok()
            .filter(|c| c.leaves.iter().all(|l| !contains_fold(l)));
        let verdict = probe(build);
        let line = match (&derived, &verdict, expect) {
            (Some(c), Verdict::State(comps, sigma), Expect::Derived) => {
                confirm_state_across_seeds(name, build, &mut failures);
                format!(
                    "  DERIVED     {name:22} {} slot(s); probe agrees (3 seeds): σ = ({}), {comps} sketch slot(s)\n",
                    c.slot_count(),
                    sigma.join(", ")
                )
            }
            (None, Verdict::State(comps, sigma), Expect::KnownFusionGap(reason)) => {
                confirm_state_across_seeds(name, build, &mut failures);
                format!(
                    "  OPEN GAP    {name:22} graph declines; state candidate survives 3 seeds: σ = ({}), ≤ {comps} slots; known merge: {reason}\n",
                    sigma.join(", ")
                )
            }
            (None, Verdict::Separated { p, q, suffix, sigma }, Expect::JustifiedDecline) => format!(
                "  JUSTIFIED   {name:22} every sketch separated — minimal witness: \
                 σ = ({}) collides on {p:?} / {q:?}, split by {suffix:?}\n",
                sigma.join(", ")
            ),
            (d, v, e) => {
                let got = match (d.is_some(), v) {
                    (true, Verdict::State(comps, s)) => {
                        format!("derives; probe agrees via ({}), ≤ {comps} slots", s.join(", "))
                    }
                    (true, Verdict::Separated { .. }) => {
                        "derives, but the probe separated every sketch (pool too weak)".into()
                    }
                    (false, Verdict::State(comps, s)) => format!(
                        "DECLINED, but σ = ({}) is a ≤{comps}-slot state candidate — NEEDS TRANSITION CLOSURE",
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
        let x = input("X", [n]);
        let stream = axis_refs(&x)[0];
        let g = build(x, 0);
        let one_pass = derive(&g, stream).is_ok_and(|c| c.leaves.iter().all(|l| !contains_fold(l)));
        if one_pass {
            continue; // sound by the existing oracle; nothing to check
        }
        match probe(build) {
            Verdict::State(comps, sigma) => {
                let line = format!(
                    "  seed {seed:2}: declined, σ = ({}) is a ≤{comps}-slot state candidate — {}\n",
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
fn random_program(x: NodeRef, n: usize, seed: u64) -> NodeRef {
    let mut s = Lcg(seed.wrapping_mul(0x9E3779B97F4A7C15) | 1);
    let leaf = |s: &mut Lcg, x: &NodeRef| -> NodeRef {
        match (s.next() >> 30) % 4 {
            0 => x.clone(),
            1 => coordinate(x.clone(), n),
            2 => konst(((s.next() >> 32) % 5) as f64 * 0.5 - 1.0),
            _ => match (s.next() >> 31) % 3 {
                0 => reduce(x.clone(), n, Monoid::Add),
                1 => reduce(x.clone(), n, Monoid::Max),
                _ => reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), n, Monoid::Add),
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
    // All three random leaves may happen to be scalar nested folds/constants.
    // Only that case needs an explicit iteration space for the top reduction.
    let e = if e.shape().is_empty() {
        map(MapOp::Mul, vec![ones_like(x), e])
    } else {
        e
    };
    let top = match (s.next() >> 28) % 3 {
        0 => Monoid::Add,
        1 => Monoid::Max,
        _ => Monoid::Min,
    };
    reduce(e, 0usize, top)
}

fn contains_fold(node: &NodeRef) -> bool {
    match node.as_ref() {
        Node::Reduce { .. } | Node::Scan { .. } => true,
        Node::Input { .. } | Node::Const { .. } | Node::Iota { .. } => false,
        Node::Coordinate { src, .. } => contains_fold(src),
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
        let x = input("X", [n]);
        let stream = axis_refs(&x)[0];
        let g = build(x, 0);
        println!("seed {seed}: derives={} {:?}\n", derive(&g, stream).is_ok(), g);
    }
}

// ── witnesses escalated into proofs: growing fooling families (§3.2) ─────────
//
// A `Separated` verdict is one fooling pair; the theory's standard (Theorem
// 5.3) is a growing fooling FAMILY: k pairwise-separated prefixes force
// state ≥ log₂ k bits. The sharp version restricts the family to prefixes
// that all share the SAME answer h — every separation then proves state
// beyond the output itself, which is what "not streamable" means. For a
// function whose answer determines its futures (sum, max, lse) such a
// family cannot exceed one member; for a true wall it grows with prefix
// length, without bound. A decline then ships with a theorem, not a survey
// result.

/// A maximal-by-greedy set of length-`plen` prefixes agreeing bit-exactly on
/// h yet pairwise separated by some sampled suffix (at the policy tolerance).
/// Suffix length grows WITH the prefix: a suffix of b elements can only move
/// a rank statistic b places, so fixed-length suffixes see a constant-width
/// window around the middle and the family would plateau spuriously.
fn fooling_family(build: Build, plen: usize, seed: u64) -> Vec<Vec<f64>> {
    let mut rng = Lcg(seed);
    let pres: Vec<Vec<f64>> = (0..800).map(|_| (0..plen).map(|_| rng.q()).collect()).collect();
    let mut sufs: Vec<Vec<f64>> = vec![Vec::new()];
    for _ in 0..25 {
        let len = 1 + (rng.next() >> 40) as usize % plen;
        sufs.push((0..len).map(|_| rng.q()).collect());
    }
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
    let separated = |a: usize, b: usize| {
        futures[a].iter().zip(&futures[b]).any(|(&fa, &fb)| {
            (fa - fb).abs() > sanic::verify::rel_tolerance(Dtype::F64, 12) * (1.0 + fa.abs().max(fb.abs()))
        })
    };
    // bucket by the answer, grow the family greedily inside each h-class
    let mut buckets: HashMap<u64, Vec<usize>> = HashMap::new();
    for (i, answers) in futures.iter().enumerate() {
        buckets.entry(answers[0].to_bits()).or_default().push(i);
    }
    let mut best: Vec<usize> = Vec::new();
    for members in buckets.values() {
        let mut family: Vec<usize> = Vec::new();
        for &i in members {
            if family.iter().all(|&j| separated(i, j)) {
                family.push(i);
            }
        }
        if family.len() > best.len() {
            best = family;
        }
    }
    best.into_iter().map(|i| pres[i].clone()).collect()
}

#[test]
fn fooling_families_grow_only_at_true_walls() {
    let same_answer_family = |build: Build, plen: usize| fooling_family(build, plen, 0xFA71).len();

    // The walls: the same-answer family grows with prefix length — the
    // refutation is a loop, and each k prints as state ≥ log₂ k bits.
    let cahm: Build = |x, n| {
        let m = reduce(x.clone(), n, Monoid::Max);
        let half = map(MapOp::Mul, vec![m, konst(0.5)]);
        reduce(indicator_lt(half, x), n, Monoid::Add)
    };
    // The two walls grow at different rates — median's classes are multisets
    // (polynomial in length), count-above-half-max's are counts against a
    // coarse threshold (linear, and slow on a 9-symbol alphabet) — so each
    // asserts its own growth floor over the measured range.
    for (name, build, floor) in [
        ("median", median_graph as Build, 16usize),
        ("count_above_half_max", cahm, 4),
    ] {
        let sizes: Vec<usize> = [4usize, 6, 8].iter().map(|&l| same_answer_family(build, l)).collect();
        println!(
            "{name}: same-answer fooling family {sizes:?} at lengths [4, 6, 8] \
             → state ≥ {:.1} bits and growing",
            (sizes[2] as f64).log2()
        );
        assert!(
            sizes[2] > sizes[0],
            "{name}: the family must grow with prefix length: {sizes:?}"
        );
        assert!(
            sizes[2] >= floor,
            "{name}: at least {floor} mutually-separated same-answer prefixes at length 8: {sizes:?}"
        );
    }

    // The controls: an answer that determines its futures admits NO
    // same-answer separation — the family machinery finds exactly one
    // member, at every length. A false separation here would be the policy
    // tolerance misfiring.
    let sum: Build = |x, n| reduce(x, n, Monoid::Add);
    let mx: Build = |x, n| reduce(x, n, Monoid::Max);
    let lse: Build = |x, n| reduce(x, n, Monoid::LogSumExp);
    for (name, build) in [("sum", sum), ("max", mx), ("lse", lse)] {
        for plen in [5usize, 9] {
            let k = same_answer_family(build, plen);
            assert_eq!(k, 1, "{name} at length {plen}: family of {k} — the answer IS the state");
        }
    }
}

// ── the second oracle: the Hankel rank test (pool-free) ──────────────────────
//
// The collision probe sees only through its sketch pool, and says so. The
// linear realization test (kernel_fusion_theory.md §5.7) needs no pool: the
// numerical rank of H[p,s] = h(p·s) measures minimal linear sequential-state
// dimension in the chosen coordinates. It is not a general carrier dimension.
// Practice notes, all load-bearing:
//
// * columns are CENTERED — an additive carrier produces the affine family
//   H[p,s] = state(p)·w(s) + g(s), whose raw rank exceeds the parameter
//   dimension by one;
// * verdicts read the RELATIVE spectrum σᵢ/σ₁ and require a visible gap;
// * the test is exact PER COORDINATE SYSTEM: a full-rank verdict is
//   ambiguous between a true wall (median) and a missing rewrite (raw
//   softmax·V — the trailing division curves the futures). Appendix A of the
//   theory doc: the loop is rewrite-candidate → rank → extract, never
//   rank → extract. Both halves are pinned as regressions below;
// * running right-to-left is a useful harness symmetry check, but with complete
//   word families it is merely the transpose construction and gives no
//   independent merge certificate; a merge-aware oracle needs two-sided
//   contexts C[u,(x,z)] = h(x·u·z), or an extracted transition algebra;
// * rank GROWING with prefix length is the refutation trend of a true wall.

/// H rows = prefixes, columns = suffix × output-coordinate (multi-output
/// functions stack column blocks). `reversed` streams right-to-left,
/// h(suffix · prefix), as a symmetry check rather than a merge proof.
fn futures_matrix(
    builds: &[Build],
    plen: usize,
    rows: usize,
    n_sufs: usize,
    reversed: bool,
    seed: u64,
) -> Vec<Vec<f64>> {
    let mut rng = Lcg(seed);
    let pres: Vec<Vec<f64>> = (0..rows).map(|_| (0..plen).map(|_| rng.c()).collect()).collect();
    let sufs: Vec<Vec<f64>> = (0..n_sufs).map(|_| (0..3).map(|_| rng.c()).collect()).collect();
    pres.iter()
        .map(|p| {
            let mut row = Vec::new();
            for s in &sufs {
                let xs: Vec<f64> = if reversed {
                    s.iter().chain(p.iter()).copied().collect()
                } else {
                    p.iter().chain(s.iter()).copied().collect()
                };
                for b in builds {
                    row.push(run_h(*b, &xs));
                }
            }
            row
        })
        .collect()
}

/// Singular values by one-sided Jacobi — adequate for these small matrices.
fn singular_values(mut a: Vec<Vec<f64>>) -> Vec<f64> {
    // orthogonalize columns; needs rows ≥ cols, so transpose if wider
    if a[0].len() > a.len() {
        a = (0..a[0].len()).map(|j| a.iter().map(|r| r[j]).collect()).collect();
    }
    let n = a[0].len();
    for _ in 0..60 {
        let mut off = 0.0f64;
        for p in 0..n {
            for q in p + 1..n {
                let (mut app, mut aqq, mut apq) = (0.0, 0.0, 0.0);
                for row in &a {
                    app += row[p] * row[p];
                    aqq += row[q] * row[q];
                    apq += row[p] * row[q];
                }
                let scale = (app * aqq).sqrt().max(1e-300);
                off = off.max(apq.abs() / scale);
                if apq.abs() <= 1e-15 * scale {
                    continue;
                }
                let zeta = (aqq - app) / (2.0 * apq);
                let t = zeta.signum() / (zeta.abs() + (1.0 + zeta * zeta).sqrt());
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = c * t;
                for row in a.iter_mut() {
                    let (rp, rq) = (row[p], row[q]);
                    row[p] = c * rp - s * rq;
                    row[q] = s * rp + c * rq;
                }
            }
        }
        if off < 1e-13 {
            break;
        }
    }
    let mut sv: Vec<f64> = (0..n)
        .map(|j| a.iter().map(|r| r[j] * r[j]).sum::<f64>().sqrt())
        .collect();
    sv.sort_by(|x, y| y.partial_cmp(x).unwrap());
    sv
}

/// Center each column (the affine-rank unit), then the relative spectrum
/// σᵢ/σ₁, descending.
fn relative_spectrum(mut h: Vec<Vec<f64>>) -> Vec<f64> {
    let rows = h.len() as f64;
    for j in 0..h[0].len() {
        let mean = h.iter().map(|r| r[j]).sum::<f64>() / rows;
        for r in h.iter_mut() {
            r[j] -= mean;
        }
    }
    let mut sv = singular_values(h);
    let top = sv[0].max(1e-300);
    for v in sv.iter_mut() {
        *v /= top;
    }
    sv
}

/// Generic continuous words for the pool-free linear-state/context oracles.
fn continuous_words(rng: &mut Lcg, count: usize, length: usize) -> Vec<Vec<f64>> {
    (0..count).map(|_| (0..length).map(|_| rng.c()).collect()).collect()
}

/// A scalar recurrence with one sequential coordinate. Every chunk acts on
/// an incoming state by an affine map, which generically needs two parameters.
fn affine_recurrence(xs: &[f64]) -> f64 {
    xs.iter().fold(0.0, |q, &x| {
        let scale = 1.1 + 0.2 * x;
        let bias = x - 0.3 * x * x;
        scale * q + bias
    })
}

/// Numerical rank at relative threshold `tol`. A cut must show a decade of
/// gap; a spectrum that never crosses `tol` is full rank (no tail at all).
fn rank_with_gap(spectrum: &[f64], tol: f64) -> Option<usize> {
    let r = spectrum.iter().take_while(|&&v| v > tol).count();
    if r == spectrum.len() {
        return Some(r);
    }
    if r == 0 {
        return Some(0);
    }
    (spectrum[r - 1] / spectrum[r].max(1e-300) > 10.0).then_some(r)
}

#[test]
fn rank_oracle_measures_small_linear_states() {
    // Σx — one linear state coordinate; centering removes the affine unit.
    let sum: Build = |x, n| reduce(x, n, Monoid::Add);
    let spec = relative_spectrum(futures_matrix(&[sum], 6, 40, 12, false, 0xA11CE));
    assert_eq!(rank_with_gap(&spec, 1e-8), Some(1), "Σ spectrum: {spec:?}");

    // Σx², same shape, different lift — still one measured slot.
    let sumsq: Build = |x, n| reduce(map(MapOp::Mul, vec![x.clone(), x]), n, Monoid::Add);
    let spec = relative_spectrum(futures_matrix(&[sumsq], 6, 40, 12, false, 0xB0B));
    assert_eq!(rank_with_gap(&spec, 1e-8), Some(1), "Σx² spectrum: {spec:?}");
}

#[test]
fn two_sided_context_rank_separates_state_from_chunk_action() {
    let mut rng = Lcg(0x00C0_7E57);
    let prefixes = continuous_words(&mut rng, 40, 4);
    let suffixes = continuous_words(&mut rng, 16, 3);

    // Ordinary suffix futures depend on a prefix only through its current q.
    // After column centering, the linear one-way-state rank is therefore one.
    let hankel: Vec<Vec<f64>> = prefixes
        .iter()
        .map(|prefix| {
            suffixes
                .iter()
                .map(|suffix| {
                    let word: Vec<_> = prefix.iter().chain(suffix).copied().collect();
                    affine_recurrence(&word)
                })
                .collect()
        })
        .collect();
    let state_spec = relative_spectrum(hankel);
    assert_eq!(
        rank_with_gap(&state_spec, 1e-8),
        Some(1),
        "affine recurrence one-way spectrum: {state_spec:?}"
    );

    // Put each candidate chunk between independently varied left and right
    // contexts. Its effect depends on both affine parameters (A,B), so the
    // centered two-sided context matrix has rank two.
    let middles = continuous_words(&mut rng, 40, 4);
    let lefts = continuous_words(&mut rng, 16, 2);
    let rights = continuous_words(&mut rng, 16, 3);
    let contexts: Vec<_> = lefts.into_iter().zip(rights).collect();
    let context_matrix: Vec<Vec<f64>> = middles
        .iter()
        .map(|middle| {
            contexts
                .iter()
                .map(|(left, right)| {
                    let word: Vec<_> = left.iter().chain(middle).chain(right).copied().collect();
                    affine_recurrence(&word)
                })
                .collect()
        })
        .collect();
    let action_spec = relative_spectrum(context_matrix);
    assert_eq!(
        rank_with_gap(&action_spec, 1e-8),
        Some(2),
        "affine recurrence two-sided spectrum: {action_spec:?}"
    );

    // The context oracle is now constructive in the affine stratum: close
    // the discovered singleton transformation family under composition and
    // require its executable carrier dimension to match the measured rank.
    let item = Expr::Item(0);
    let scale = Expr::Add(
        Box::new(Expr::Const(1.1)),
        Box::new(Expr::Mul(Box::new(Expr::Const(0.2)), Box::new(item.clone()))),
    );
    let bias = Expr::Sub(
        Box::new(item.clone()),
        Box::new(Expr::Mul(
            Box::new(Expr::Const(0.3)),
            Box::new(Expr::Mul(Box::new(item.clone()), Box::new(item))),
        )),
    );
    let generated = affine_transition_carrier(scale, bias, 0.0).unwrap();
    assert_eq!(generated.slot_count(), rank_with_gap(&action_spec, 1e-8).unwrap());
    assert_eq!(
        generated.laws.associativity,
        AssociativityEvidence::TransitionComposition
    );
    assert_eq!(generated.laws.merge_order, MergeOrderEvidence::ProgramOrder);
    assert!(!generated.mergeable_out_of_order());
    for word in prefixes.iter().chain(&middles).chain(&suffixes) {
        let items: Vec<_> = word.iter().map(|value| vec![*value]).collect();
        let expected = affine_recurrence(word);
        assert!((generated.fold(&items)[0] - expected).abs() <= 1e-10);
        assert!((generated.tree_fold(&items)[0] - expected).abs() <= 1e-10);
    }
}

#[test]
fn rank_oracle_reports_attention_ambiguous_raw_and_small_deferred() {
    // softmax·v in RAW coordinates (value channel v = x², keeping the
    // one-input Build shape): the trailing division makes the futures a
    // CURVED family. A smooth curved family's spectrum decays steadily —
    // fast, but with no decade gap anywhere and no numerically-zero tail —
    // which is precisely the AMBIGUOUS verdict of theory §5.7: it cannot
    // distinguish a true wall from a missing rewrite, so it must route into
    // the rewrite search rather than terminate the analysis.
    let raw: Build = |x, n| {
        let e = map(MapOp::Exp, vec![x.clone()]);
        let v = map(MapOp::Mul, vec![x.clone(), x]);
        let num = reduce(map(MapOp::Mul, vec![e.clone(), v]), n, Monoid::Add);
        let den = reduce(e, n, Monoid::Add);
        map(MapOp::Div, vec![num, den])
    };
    let spec = relative_spectrum(futures_matrix(&[raw], 6, 40, 12, false, 0xF1A5));
    assert_eq!(
        rank_with_gap(&spec, 1e-8),
        None,
        "raw softmax·v must have NO clean gap (curved family): {spec:?}"
    );
    assert!(
        *spec.last().unwrap() > 1e-12,
        "raw softmax·v must have no numerically-zero tail: {spec:?}"
    );

    // One rewrite collapses it (Theorem 5.2 + the exp homomorphism): in the
    // DEFERRED coordinates (ℓ, o) = (Σeˣ, Σeˣ·v) the measured linear-state
    // rank is exactly 2. The running max contributes nothing here: it is
    // numerical stabilization, not exact-real semantic state (Appendix A).
    let ell: Build = |x, n| reduce(map(MapOp::Exp, vec![x]), n, Monoid::Add);
    let o: Build = |x, n| {
        let e = map(MapOp::Exp, vec![x.clone()]);
        let v = map(MapOp::Mul, vec![x.clone(), x]);
        reduce(map(MapOp::Mul, vec![e, v]), n, Monoid::Add)
    };
    let spec = relative_spectrum(futures_matrix(&[ell, o], 6, 40, 12, false, 0xF1A5));
    assert_eq!(
        rank_with_gap(&spec, 1e-8),
        Some(2),
        "deferred (ℓ, o) spectrum: {spec:?}"
    );

    // Right-to-left, same coordinates: the rank agrees as a symmetry
    // regression. This does not independently certify a merge; the explicit
    // additive combine for (ℓ, o) supplies that certificate.
    let spec = relative_spectrum(futures_matrix(&[ell, o], 6, 40, 12, true, 0xF1A5));
    assert_eq!(
        rank_with_gap(&spec, 1e-8),
        Some(2),
        "reversed (ℓ, o) spectrum: {spec:?}"
    );
}

#[test]
fn rank_oracle_corroborates_the_median_wall() {
    // Median: the measured rank tracks the SAMPLE SIZE — every enlargement
    // of the futures matrix finds new independent directions, the spectrum
    // never plateaus. That is the refutation trend of a true wall, read with
    // no sketch vocabulary at all; a finite-dimensional linear realization's
    // rank plateaus no matter how many samples are thrown at it (the deferred
    // (ℓ, o) coordinates above: 2 at every size). Rank growth alone would not
    // exclude an arbitrary nonlinear carrier. The collision probe's separating
    // witness and the fooling family settle the claim; this corroborates it
    // from a second, independent instrument.
    let effective_rank = |n_sufs: usize| {
        let spec = relative_spectrum(futures_matrix(&[median_graph], 8, n_sufs + 8, n_sufs, false, 0x3D1A));
        spec.iter().take_while(|&&v| v > 1e-8).count()
    };
    for n_sufs in [8usize, 16, 24] {
        let r = effective_rank(n_sufs);
        assert!(
            r >= n_sufs - 1,
            "median rank must track the sample size, never plateau: {r} of {n_sufs}"
        );
    }
}
