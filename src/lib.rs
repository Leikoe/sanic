//! An algebraic engine for deriving streaming kernels.
//!
//! Implements the buildable core of `streamability_engine.md`:
//!   * §3   the IR (primitive basis + derived operators)            → `engine_ir`
//!   * §3.2 operator algebra metadata                               → `op`
//!   * §4   Stage 1: structure classification                       → `stage1`
//!   * §5   Stage 2A: carrier derivation by composition (R1–R5)     → `carrier`
//!
//! Stage 2B (SMT carrier synthesis, §7) is the research frontier and is left
//! unimplemented by design.

pub mod carrier;
pub mod engine_ir;
pub mod op;
pub mod stage1;

// Legacy sketch modules kept for reference.
pub mod experimental;
pub mod hir;
pub mod mir;

#[cfg(test)]
mod acceptance {
    //! §10 — Acceptance tests. The engine is correct on the core if, with no
    //! hand-written accumulators, it produces these classifications and carriers.
    //! Reproducing the FlashAttention `(m, ℓ, o)` accumulator from the
    //! composition rules — not a stored template — is the primary criterion.

    use crate::carrier::{self, Carrier};
    use crate::engine_ir::*;
    use crate::op::{BinOp, Monoid};
    use crate::stage1::{Parallelism, streamable, structure};

    fn add_r() -> BinOp {
        BinOp::Monoid(Monoid::Add)
    }
    fn max_r() -> BinOp {
        BinOp::Monoid(Monoid::Max)
    }
    fn lse_r() -> BinOp {
        BinOp::Monoid(Monoid::LogSumExp)
    }

    // ── a tiny deterministic PRNG so tests need no external crates ───────────
    struct Lcg(u64);
    impl Lcg {
        fn new(seed: u64) -> Self {
            Lcg(seed.max(1))
        }
        fn next_f64(&mut self) -> f64 {
            // xorshift64*
            let mut x = self.0;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            self.0 = x;
            let u = x.wrapping_mul(0x2545F4914F6CDD1D);
            ((u >> 11) as f64 / (1u64 << 53) as f64) * 6.0 - 3.0 // ~[-3, 3]
        }
    }

    fn approx(a: f64, b: f64) {
        let tol = 1e-9 * (1.0 + a.abs().max(b.abs()));
        assert!((a - b).abs() <= tol, "approx failed: {a} vs {b}");
    }

    /// A derived carrier must agree with itself under a tree split
    /// (associativity) and with the reference semantics
    /// (`project ∘ fold = reference`) — the validation of checklist item 3.
    fn check(car: &Carrier, items: &[Vec<f64>], reference: &[f64]) {
        let folded = car.fold(items);
        let tree = car.tree_fold(items);
        assert_eq!(folded.len(), reference.len());
        for i in 0..folded.len() {
            approx(folded[i], reference[i]);
            approx(tree[i], reference[i]); // tree == sequential ⇒ associative
        }
    }

    // ── §10: matmul tags i,j FREE and k MONOIDAL+LINEAR ──────────────────────
    #[test]
    fn matmul_axis_tags() {
        let a = input("A", &["i", "k"]);
        let b = input("B", &["k", "j"]);
        let mm = matmul(a, b, "k");

        assert_eq!(structure(&mm, "i").level, Parallelism::Free);
        assert_eq!(structure(&mm, "j").level, Parallelism::Free);

        let k = structure(&mm, "k");
        assert_eq!(k.level, Parallelism::Monoidal);
        assert!(k.linear, "contraction is a linear (+) reduction");
        assert!(streamable(&mm, "k"));
    }

    // ── the dot-product carrier reproduces a contraction (R1 + R2) ───────────
    #[test]
    fn dot_product_carrier() {
        let a = input("A", &["k"]);
        let b = input("B", &["k"]);
        let mm = matmul(a, b, "k");

        let car = carrier::derive(&mm, "k").expect("matmul k is derivable");
        assert_eq!(car.slots, 1);

        let mut rng = Lcg::new(1);
        let items: Vec<Vec<f64>> = (0..17)
            .map(|_| vec![rng.next_f64(), rng.next_f64()])
            .collect();
        let reference: f64 = items.iter().map(|p| p[0] * p[1]).sum();
        check(&car, &items, &[reference]);
    }

    // ── §10: attention s_q FREE, k MONOIDAL, derives Acc=(m,s,o), proj=o/s ───
    #[test]
    fn attention_axis_tags_and_carrier() {
        let q = input("Q", &["sq", "d"]);
        let k = input("K", &["k", "d"]);
        let v = input("V", &["k", "e"]);
        let attn = attention(q, k, v, "d", "k");

        assert_eq!(structure(&attn, "sq").level, Parallelism::Free);
        assert_eq!(structure(&attn, "k").level, Parallelism::Monoidal);
        assert!(streamable(&attn, "k"));

        // The headline: derive the (m, ℓ, o) accumulator from the rules, by the
        // generic compositional fold — no FlashAttention-shaped template.
        let car = carrier::derive(&attn, "k").expect("attention k is derivable");
        assert_eq!(car.slots, 3, "Acc = (m, ℓ, o)");
        // R4 = the online-softmax coupling, R5 = deferring the ÷s normalizer.
        assert_eq!(car.rules, vec!["R1", "R3", "R4", "R5"]);

        // Run it on random (score, value) pairs and compare to softmax·V.
        let mut rng = Lcg::new(42);
        let items: Vec<Vec<f64>> = (0..23)
            .map(|_| vec![rng.next_f64(), rng.next_f64()])
            .collect();

        let max = items.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
        let denom: f64 = items.iter().map(|p| (p[0] - max).exp()).sum();
        let numer: f64 = items.iter().map(|p| (p[0] - max).exp() * p[1]).sum();
        check(&car, &items, &[numer / denom]);
    }

    // ── §10: derive (sum,count) for mean from its IR ─────────────────────────
    #[test]
    fn mean_carrier() {
        // mean = (Σ x) / (Σ 1)
        let x = input("X", &["a"]);
        let sum = reduce(x.clone(), "a", add_r());
        let count = reduce(map(ONE, vec![x.clone()]), "a", add_r());
        let mean = map(DIV, vec![sum, count]);

        let car = carrier::derive(&mean, "a").expect("mean is derivable");
        assert_eq!(car.slots, 2, "Acc = (sum, count)");
        assert!(car.rules.contains(&"R3")); // product carrier

        let mut rng = Lcg::new(7);
        let xs: Vec<f64> = (0..31).map(|_| rng.next_f64()).collect();
        let items: Vec<Vec<f64>> = xs.iter().map(|&x| vec![x]).collect();
        let reference = xs.iter().sum::<f64>() / xs.len() as f64;
        check(&car, &items, &[reference]);
    }

    // ── §10: derive a (sumx2, sumx, count) variance carrier from its IR ──────
    #[test]
    fn variance_carrier() {
        // var = E[x²] − E[x]²  =  Σx²/n − (Σx/n)²
        let x = input("X", &["a"]);
        let sumx2 = reduce(map(MUL, vec![x.clone(), x.clone()]), "a", add_r());
        let sumx = reduce(x.clone(), "a", add_r());
        let count = reduce(map(ONE, vec![x.clone()]), "a", add_r());
        let ex2 = map(DIV, vec![sumx2, count.clone()]);
        let ex = map(DIV, vec![sumx, count]);
        let var = map(SUB_F, vec![ex2, map(MUL, vec![ex.clone(), ex])]);

        let car = carrier::derive(&var, "a").expect("variance is derivable");
        assert_eq!(car.slots, 3, "Acc = (Σx², Σx, count)");

        let mut rng = Lcg::new(99);
        let xs: Vec<f64> = (0..40).map(|_| rng.next_f64()).collect();
        let items: Vec<Vec<f64>> = xs.iter().map(|&x| vec![x]).collect();
        let n = xs.len() as f64;
        let mu = xs.iter().sum::<f64>() / n;
        let reference = xs.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / n;
        check(&car, &items, &[reference]);
    }

    // ── §10: derive (max,Σexp) for logsumexp from its IR ─────────────────────
    #[test]
    fn logsumexp_carrier() {
        // lse(x) = log(Σ exp(x − m)) + m,   m = max x
        let x = input("X", &["a"]);
        let m = reduce(x.clone(), "a", max_r());
        let e = map(EXP_SUB, vec![x.clone(), m.clone()]);
        let s = reduce(e, "a", add_r());
        let lse = map(ADD_F, vec![map(LOG, vec![s]), m]);

        let car = carrier::derive(&lse, "a").expect("logsumexp is derivable");
        assert_eq!(car.slots, 2, "Acc = (max, Σexp)");
        assert!(car.rules.contains(&"R4")); // the max/exp coupling

        let mut rng = Lcg::new(123);
        let xs: Vec<f64> = (0..29).map(|_| rng.next_f64()).collect();
        let items: Vec<Vec<f64>> = xs.iter().map(|&x| vec![x]).collect();
        let reference = xs.iter().map(|x| x.exp()).sum::<f64>().ln();
        check(&car, &items, &[reference]);
    }

    // ── the SAME fold handles attention over *several* value tensors: two
    // o-slots ride the one softmax. A template for FlashAttention could not. ──
    #[test]
    fn multi_value_attention_generalizes() {
        // Σ softmax(scores)·V1  and  Σ softmax(scores)·V2, summed — exercises the
        // generic coupling with two deferred linear reductions sharing one (m,s).
        let scores = input("S", &["k"]);
        let v1 = input("V1", &["k"]);
        let v2 = input("V2", &["k"]);
        let w = softmax(scores, "k");
        let o1 = reduce(map(MUL, vec![w.clone(), v1]), "k", add_r());
        let o2 = reduce(map(MUL, vec![w, v2]), "k", add_r());
        let total = map(ADD_F, vec![o1, o2]);

        let car = carrier::derive(&total, "k").expect("multi-value attention derivable");
        assert_eq!(car.slots, 4, "Acc = (m, s, o1, o2) — one shared softmax");

        let mut rng = Lcg::new(2027);
        let items: Vec<Vec<f64>> = (0..15)
            .map(|_| vec![rng.next_f64(), rng.next_f64(), rng.next_f64()])
            .collect();
        let mx = items.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
        let denom: f64 = items.iter().map(|p| (p[0] - mx).exp()).sum();
        let n1: f64 = items.iter().map(|p| (p[0] - mx).exp() * p[1]).sum();
        let n2: f64 = items.iter().map(|p| (p[0] - mx).exp() * p[2]).sum();
        check(&car, &items, &[n1 / denom + n2 / denom]);
    }

    // ── §10: tanh-RNN time axis SEQUENTIAL, no accumulator ───────────────────
    #[test]
    fn tanh_rnn_is_sequential() {
        let x = input("X", &["t", "h"]);
        let rnn = tanh_rnn(x, "t");
        assert_eq!(structure(&rnn, "t").level, Parallelism::Sequential);
        assert!(!streamable(&rnn, "t"));
        assert!(
            carrier::derive(&rnn, "t").is_none(),
            "refuses to emit an accumulator for a non-associative recurrence"
        );
    }

    // ── §10: linear/SSM scan time axis MONOIDAL, affine-map carrier ──────────
    #[test]
    fn ssm_scan_is_monoidal() {
        let params = input("AB", &["t"]); // each step carries its (A_t, b_t)
        let ssm = ssm_scan(params, "t");
        assert_eq!(structure(&ssm, "t").level, Parallelism::Monoidal);
        assert!(streamable(&ssm, "t"));

        let car = carrier::derive(&ssm, "t").expect("affine scan is derivable");
        assert_eq!(car.slots, 2, "carrier is the affine map (A, b)");

        // reference: iterate h_t = A_t·h_{t-1} + b_t from h_0 = 0
        let mut rng = Lcg::new(555);
        let steps: Vec<(f64, f64)> = (0..19)
            .map(|_| (rng.next_f64() * 0.3, rng.next_f64()))
            .collect();
        let mut h = 0.0;
        for &(a, b) in &steps {
            h = a * h + b;
        }
        let items: Vec<Vec<f64>> = steps.iter().map(|&(a, b)| vec![a, b]).collect();
        check(&car, &items, &[h]);
    }

    // ── §10: embedding / gather axis OPAQUE ──────────────────────────────────
    #[test]
    fn embedding_is_opaque() {
        let table = input("E", &["vocab", "d"]);
        let ids = input("ids", &["seq"]);
        let emb = embedding(table, ids, "vocab");
        assert_eq!(structure(&emb, "vocab").level, Parallelism::Opaque);
        assert!(!streamable(&emb, "vocab"));
        assert!(carrier::derive(&emb, "vocab").is_none());
    }

    // ── §4.3: per-(node,axis) — the middle axis of a double-GEMM is reduced in
    // one sub-expression and FREE in another. Distinguishes the two fusion kinds.
    #[test]
    fn per_node_axis_double_gemm() {
        // (X·Y)·Z. The middle axis `m` is a FREE output index of GEMM-1 but the
        // contraction of GEMM-2 — the same axis, two structures, distinguished
        // only because we track per-(node, axis) rather than collapsing to the
        // output. This is what separates the two fusion kinds of §8.
        let x = input("X", &["i", "a"]);
        let y = input("Y", &["a", "m"]);
        let g1 = matmul(x, y, "a"); // contracts a → output [i, m]
        let z = input("Z", &["m", "j"]);
        let g2 = matmul(g1.clone(), z, "m"); // contracts m → output [i, j]

        // `m` is a free output index of GEMM-1 ...
        assert_eq!(structure(&g1, "m").level, Parallelism::Free);
        // ... but the contraction of GEMM-2, at that node.
        assert_eq!(structure(&g2, "m").level, Parallelism::Monoidal);
    }

    // associativity must hold for *every* split point, exercised at the
    // accumulator level via the public `fold_acc` / `merge` / `project`.
    #[test]
    fn flash_attention_associative_all_splits() {
        let q = input("Q", &["sq", "d"]);
        let k = input("K", &["k", "d"]);
        let v = input("V", &["k", "e"]);
        let attn = attention(q, k, v, "d", "k");
        let car = carrier::derive(&attn, "k").unwrap();
        let mut rng = Lcg::new(2024);
        let items: Vec<Vec<f64>> = (0..12)
            .map(|_| vec![rng.next_f64(), rng.next_f64()])
            .collect();
        let whole = car.fold(&items);
        for split in 1..items.len() {
            let l = car.fold_acc(&items[..split]);
            let r = car.fold_acc(&items[split..]);
            let merged = car.project(&car.merge(&l, &r));
            for i in 0..whole.len() {
                approx(whole[i], merged[i]);
            }
        }
    }

    // ── battle-test: the CTC forward DP, which fuses every axis kind at once ──
    //
    //   α_t[s] = logsumexp(α_{t-1}[s], α_{t-1}[s-1], α_{t-1}[s-2]) + logp_t[ℓ(s)]
    //   loss   = −logsumexp_s α_T[s]
    //
    // Checked against the published acceptance oracle, axis by axis. The single
    // most important pass/fail: tag `t` SEQUENTIAL while tagging BOTH logsumexps
    // MONOIDAL-with-a-derived-(m,s)-carrier — stream *inside* a timestep, but
    // serialize *across* time. An engine that conflates "reduction-shaped" with
    // "foldable" would illegally parallelize the time axis.
    #[test]
    fn ctc_forward_battle_test() {
        use crate::carrier::Expr;

        // The (max, Σexp) carrier must be DERIVED, not stored: 2 slots, the
        // rescaling combine (telescoping exp — not a naive Add), and the
        // log-space projection `log(s) + m`.
        fn has_exp(e: &Expr) -> bool {
            match e {
                Expr::Exp(_) => true,
                Expr::Add(a, b) | Expr::Mul(a, b) | Expr::Sub(a, b) | Expr::Div(a, b)
                | Expr::Max(a, b) | Expr::Min(a, b) => has_exp(a) || has_exp(b),
                Expr::Log(a) => has_exp(a),
                _ => false,
            }
        }
        fn is_logsumexp_carrier(c: &Carrier) -> bool {
            c.slots == 2
                && c.identity[0] == f64::NEG_INFINITY
                && c.identity[1] == 0.0
                && matches!(&c.combine[0], Expr::Max(..)) // m = max
                && has_exp(&c.combine[1]) // s telescopes via exp(m_old − m_new)
                && matches!(&c.project[0],   // project = log(s) + m
                    Expr::Add(lg, m) if matches!(**lg, Expr::Log(_)) && matches!(**m, Expr::F(0)))
        }

        // axes: b=batch, t=time, s=state, pred=predecessor (2–3 states), v=vocab
        let logp = input("logp", &["b", "t", "v"]);
        let labels = input("labels", &["s"]);
        let emit = gather(logp, labels, "v"); // logp_t[ℓ(s)] — index vocab by label

        let prev = input("alpha_prev", &["b", "pred", "s"]); // α_{t-1} at predecessors
        let trans = reduce(prev, "pred", lse_r()); // logsumexp over predecessors
        let step = map(ADD_F, vec![trans.clone(), emit.clone()]); // + emission
        let alpha = scan(step.clone(), "t", BinOp::NonAssoc("ctc_forward")); // recurrence

        let alpha_t = input("alpha_T", &["b", "s"]);
        let loss = reduce(alpha_t, "s", lse_r()); // final logsumexp over states

        // ── oracle row: b (batch) → FREE → grid ──────────────────────────────
        assert_eq!(structure(&alpha, "b").level, Parallelism::Free);

        // ── oracle row: s within a timestep → FREE → parallel across states ──
        assert_eq!(structure(&step, "s").level, Parallelism::Free);

        // ── oracle row: pred → MONOIDAL, derive (max, Σexp), fuse, no intermediate
        assert_eq!(structure(&trans, "pred").level, Parallelism::Monoidal);
        assert!(structure(&trans, "pred").linear, "log-semiring additive");
        let tc = carrier::derive(&trans, "pred").expect("transition logsumexp derivable");
        assert!(is_logsumexp_carrier(&tc), "derived (m, s) carrier, project=log(s)+m");
        assert!(tc.rules.contains(&"R4"));
        {
            let mut rng = Lcg::new(314);
            let xs: Vec<f64> = (0..6).map(|_| rng.next_f64()).collect(); // 2–3+ predecessors
            let items: Vec<Vec<f64>> = xs.iter().map(|&x| vec![x]).collect();
            let reference = xs.iter().map(|x| x.exp()).sum::<f64>().ln();
            check(&tc, &items, &[reference]);
        }

        // ── oracle row: v (vocab, via label gather) → OPAQUE → not foldable ──
        assert_eq!(structure(&emit, "v").level, Parallelism::Opaque);
        assert!(!streamable(&emit, "v"));
        assert!(carrier::derive(&emit, "v").is_none());

        // ── oracle row: t (time) → SEQUENTIAL → must REFUSE to fold ──────────
        assert_eq!(structure(&alpha, "t").level, Parallelism::Sequential);
        assert!(!streamable(&alpha, "t"));
        assert!(
            carrier::derive(&alpha, "t").is_none(),
            "must refuse to emit a fold for the non-associative time recurrence"
        );

        // ── oracle row: final s reduction → MONOIDAL, (max, Σexp), log(s)+m ──
        assert_eq!(structure(&loss, "s").level, Parallelism::Monoidal);
        let lc = carrier::derive(&loss, "s").expect("final logsumexp derivable");
        assert!(is_logsumexp_carrier(&lc));
        assert!(lc.rules.contains(&"R4"));

        // ── the headline guarantee: in ONE model, t is serial while BOTH
        //    logsumexps fold; and the two carriers are derived *uniformly* (the
        //    same (m,s) accumulator), proving there is no stored template — they
        //    live on different axes, one buried inside the scan body. ──────────
        assert!(!streamable(&alpha, "t") && streamable(&trans, "pred") && streamable(&loss, "s"));
        assert_eq!(format!("{:?}", tc.combine), format!("{:?}", lc.combine));
        assert_eq!(format!("{:?}", tc.into), format!("{:?}", lc.into));
        assert_eq!(format!("{:?}", tc.project), format!("{:?}", lc.project));
    }

    // ── the harder composite: a soft-attention readout over a streamed log-space
    //    DP. ONE graph that forces R4, R5, OPAQUE, the SEQUENTIAL atom, AND the
    //    same-axis vs cross-axis fusion split:
    //
    //      score[b,k]   = Reduce(Map(×, Q[b], K[b,k]), d, ADD)   # LINEAR on d
    //      weight       = Softmax(score, k)                      # (m,s) via R4
    //      value[b,k,d] = LogMatMul(Wv, H[b,k]; contract=h)      # logsumexp-matmul on h
    //      out[b,d]     = Reduce(Map(×, weight, value), k, ADD)  # LINEAR consumer on k → R5
    //
    //    The softmax (m,s) and the out-reduction share axis k → same-axis MERGE
    //    into one (m,s,o) fold. The values are a reduction over a DIFFERENT axis
    //    h → cross-axis PRODUCER/CONSUMER: from the k-pass, `value` is a per-k
    //    leaf, and its h-carrier is derived separately. K is gather-indexed
    //    (OPAQUE), and the whole readout is one step of a serial recurrence
    //    (SEQUENTIAL).
    #[test]
    fn soft_attention_over_logspace_dp() {
        use crate::carrier::Expr;

        // gather-indexed score: K is selected from a table by a runtime label.
        let q = input("Q", &["b", "d"]);
        let ktable = input("Ktable", &["idx", "k", "d"]);
        let labels = input("labels", &["b"]);
        let kgath = gather(ktable, labels, "idx"); // K[b,k,d] via runtime index → OPAQUE on idx
        let score = reduce(map(MUL, vec![q, kgath.clone()]), "d", add_r()); // LINEAR contraction on d
        let weight = softmax(score.clone(), "k"); // (m, s) via R4

        // VALUES are themselves a streamed reduction: a logsumexp-matmul over h.
        let wv = input("Wv", &["d", "h"]);
        let hmat = input("H", &["b", "t", "k", "h"]);
        let value = reduce(map(ADD_F, vec![wv, hmat]), "h", lse_r()); // LogMatMul contract=h

        let out = reduce(map(MUL, vec![weight, value.clone()]), "k", add_r()); // LINEAR consumer on k
        let recur = scan(out.clone(), "t", BinOp::NonAssoc("recurrent_readout")); // serial in time

        // ── d: the score contraction is MONOIDAL and LINEAR ──────────────────
        let sd = structure(&score, "d");
        assert_eq!(sd.level, Parallelism::Monoidal);
        assert!(sd.linear);

        // ── same-axis MERGE on k: derive the (m, s, o) carrier, R4 + R5. The
        //    values are atomic per-k inputs, so exactly 3 slots — h is NOT
        //    swallowed into the k-fold. ─────────────────────────────────────────
        assert_eq!(structure(&out, "k").level, Parallelism::Monoidal);
        let oc = carrier::derive(&out, "k").expect("k readout derivable");
        assert_eq!(oc.slots, 3, "Acc = (m, s, o) — softmax merged, h not fused in");
        assert!(oc.rules.contains(&"R4"), "online-softmax coupling");
        assert!(oc.rules.contains(&"R5"), "deferred normalizer (linear consumer)");
        assert_eq!(oc.identity, vec![f64::NEG_INFINITY, 0.0, 0.0]);
        {
            let mut rng = Lcg::new(7001);
            let items: Vec<Vec<f64>> = (0..20) // (score, value) per key
                .map(|_| vec![rng.next_f64(), rng.next_f64()])
                .collect();
            let mx = items.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
            let den: f64 = items.iter().map(|p| (p[0] - mx).exp()).sum();
            let num: f64 = items.iter().map(|p| (p[0] - mx).exp() * p[1]).sum();
            check(&oc, &items, &[num / den]);
        }

        // ── cross-axis PRODUCER on h: value's own (max, Σexp) carrier, with the
        //    log-space product (Wv + H) folded into `into` (R2) and R4 coupling. ─
        assert_eq!(structure(&value, "h").level, Parallelism::Monoidal);
        let vc = carrier::derive(&value, "h").expect("logsumexp-matmul derivable");
        assert_eq!(vc.slots, 2, "(max, Σexp)");
        assert!(vc.rules.contains(&"R2"), "the additive pre-map folds into `into`");
        assert!(vc.rules.contains(&"R4"));
        // project = log(s) + m
        assert!(matches!(&vc.project[0],
            Expr::Add(lg, m) if matches!(**lg, Expr::Log(_)) && matches!(**m, Expr::F(0))));
        {
            let mut rng = Lcg::new(7002);
            let items: Vec<Vec<f64>> = (0..14) // (Wv_h, H_h) per contracted h
                .map(|_| vec![rng.next_f64(), rng.next_f64()])
                .collect();
            let reference = items.iter().map(|p| (p[0] + p[1]).exp()).sum::<f64>().ln();
            check(&vc, &items, &[reference]);
        }

        // ── THE double-fusion distinction, surfaced per-(node, axis): at `out`,
        //    BOTH k and h are foldable, but they are different fusion kinds.
        //    `value` is FREE along k (→ cross-axis tiling, treated as a leaf in
        //    the k-fold), while h is still MONOIDAL at the `out` node (the
        //    producer reduction lives one level down). ─────────────────────────
        assert_eq!(structure(&value, "k").level, Parallelism::Free);
        assert_eq!(structure(&out, "h").level, Parallelism::Monoidal);
        assert_eq!(structure(&out, "k").level, Parallelism::Monoidal);

        // ── OPAQUE: the gather-indexed score input is runtime-determined ──────
        assert_eq!(structure(&kgath, "idx").level, Parallelism::Opaque);
        assert!(carrier::derive(&kgath, "idx").is_none());

        // ── SEQUENTIAL atom: the recurrent readout serializes across time ────
        assert_eq!(structure(&recur, "t").level, Parallelism::Sequential);
        assert!(carrier::derive(&recur, "t").is_none());

        // ── batch stays FREE through the whole composite ─────────────────────
        assert_eq!(structure(&recur, "b").level, Parallelism::Free);
    }
}
