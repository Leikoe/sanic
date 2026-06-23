//! §5 — Stage 2A: carrier derivation by composition.
//!
//! The heart of the engine: turn a MONOIDAL/LINEAR axis into a *concrete
//! accumulator*. The accumulator is **derived, not searched, and not matched**:
//! `derive` is a single bottom-up structural fold over the IR. It never asks
//! "is this FlashAttention?" — it asks, of each sub-expression, "what does
//! streaming *this* over the axis contribute to the accumulator?", and composes
//! the answers with the rules R1–R5. The same fold therefore produces the carrier
//! for `sum`, `mean`, `variance`, `logsumexp`, a plain contraction, FlashAttention
//! — and anything else built from the same operators (e.g. attention over several
//! value tensors, or any normalized reduction consumed linearly).
//!
//! A derived carrier is represented as *data*: a vector of scalar slots plus three
//! symbolic programs (`into`, `combine`, `project`). Because it is data, the
//! FlashAttention `(m, ℓ, o)` accumulator is genuinely *constructed* by the fold —
//! nowhere written as a literal — then executed by the interpreter and
//! property-tested against a reference. That is the primary criterion of §10.

use std::collections::BTreeSet;
use std::rc::Rc;

use crate::engine_ir::{Node, NodeKind};
use crate::op::{BinOp, Monoid};
use crate::stage1::{Parallelism, structure};

// ── symbolic expressions over carrier slots ─────────────────────────────────

/// A pure scalar expression. `Item(i)` reads field `i` of the element being
/// lifted (used only in `into`); `A(i)`/`B(i)` read field `i` of the two
/// accumulators being combined (used only in `combine`); `F(i)` reads field `i`
/// of the final accumulator (used only in `project`).
#[derive(Debug, Clone)]
pub enum Expr {
    Const(f64),
    Item(usize),
    A(usize),
    B(usize),
    F(usize),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Max(Box<Expr>, Box<Expr>),
    Min(Box<Expr>, Box<Expr>),
    Exp(Box<Expr>),
    Log(Box<Expr>),
}

// terse constructors
fn cst(v: f64) -> Expr {
    Expr::Const(v)
}
fn sub(a: Expr, b: Expr) -> Expr {
    Expr::Sub(Box::new(a), Box::new(b))
}
fn emax(a: Expr, b: Expr) -> Expr {
    Expr::Max(Box::new(a), Box::new(b))
}
fn emin(a: Expr, b: Expr) -> Expr {
    Expr::Min(Box::new(a), Box::new(b))
}
fn exp(a: Expr) -> Expr {
    Expr::Exp(Box::new(a))
}
fn log(a: Expr) -> Expr {
    Expr::Log(Box::new(a))
}

// simplifying constructors — fold the `Const(0/1)` units so derived carriers
// stay readable (e.g. `o`'s `into` is `Item(1)`, not `1·Item(1)`).
fn is1(e: &Expr) -> bool {
    matches!(e, Expr::Const(c) if *c == 1.0)
}
fn is0(e: &Expr) -> bool {
    matches!(e, Expr::Const(c) if *c == 0.0)
}
fn padd(a: Expr, b: Expr) -> Expr {
    if is0(&a) {
        b
    } else if is0(&b) {
        a
    } else {
        Expr::Add(Box::new(a), Box::new(b))
    }
}
fn pmul(a: Expr, b: Expr) -> Expr {
    if is0(&a) || is0(&b) {
        return cst(0.0);
    }
    // Push multiplication through division so a deferred normalizer stays a
    // single fraction: (n/d)·c becomes (n·c)/d, i.e. (1/ℓ)·o renders as o/ℓ.
    match (a, b) {
        (Expr::Div(n, d), b) => pdiv(pmul(*n, b), *d),
        (a, Expr::Div(n, d)) => pdiv(pmul(a, *n), *d),
        (a, b) if is1(&a) => b,
        (a, b) if is1(&b) => a,
        (a, b) => Expr::Mul(Box::new(a), Box::new(b)),
    }
}
fn pdiv(a: Expr, b: Expr) -> Expr {
    if is1(&b) {
        a
    } else {
        Expr::Div(Box::new(a), Box::new(b))
    }
}

struct Env<'a> {
    item: Option<&'a [f64]>,
    a: Option<&'a [f64]>,
    b: Option<&'a [f64]>,
    f: Option<&'a [f64]>,
}

fn eval(e: &Expr, env: &Env) -> f64 {
    match e {
        Expr::Const(v) => *v,
        Expr::Item(i) => env.item.expect("Item outside into")[*i],
        Expr::A(i) => env.a.expect("A outside combine")[*i],
        Expr::B(i) => env.b.expect("B outside combine")[*i],
        Expr::F(i) => env.f.expect("F outside project")[*i],
        Expr::Add(a, b) => eval(a, env) + eval(b, env),
        Expr::Sub(a, b) => eval(a, env) - eval(b, env),
        Expr::Mul(a, b) => eval(a, env) * eval(b, env),
        Expr::Div(a, b) => eval(a, env) / eval(b, env),
        Expr::Max(a, b) => eval(a, env).max(eval(b, env)),
        Expr::Min(a, b) => eval(a, env).min(eval(b, env)),
        Expr::Exp(a) => eval(a, env).exp(),
        Expr::Log(a) => eval(a, env).ln(),
    }
}

// ── the derived carrier ─────────────────────────────────────────────────────

/// A concrete, executable instance of the §5.1 `Streamable` object.
/// `into : Item -> Acc`, `combine : (Acc, Acc) -> Acc` (associative `⊗`),
/// `identity : Acc`, `project : Acc -> Out`. `Acc = Out` exactly when no
/// strengthening was needed; otherwise `project` discards the surplus state.
#[derive(Debug, Clone)]
pub struct Carrier {
    pub slots: usize,
    pub into: Vec<Expr>,
    pub combine: Vec<Expr>,
    pub identity: Vec<f64>,
    pub project: Vec<Expr>,
    /// The rules that fired while building this carrier (deduped, for §10).
    pub rules: Vec<&'static str>,
}

impl Carrier {
    /// Fold a list of elements left-to-right, O(1) state (the streaming path).
    pub fn fold(&self, items: &[Vec<f64>]) -> Vec<f64> {
        let mut acc = self.identity.clone();
        for it in items {
            let el = self.lift(it);
            acc = self.merge(&acc, &el);
        }
        self.project(&acc)
    }

    /// Fold by recursive bisection, combining sub-results in a tree (the
    /// parallelism path). Equality with `fold` for all split points is exactly
    /// the associativity certificate of §1.
    pub fn tree_fold(&self, items: &[Vec<f64>]) -> Vec<f64> {
        self.project(&self.tree_acc(items))
    }

    fn tree_acc(&self, items: &[Vec<f64>]) -> Vec<f64> {
        match items.len() {
            0 => self.identity.clone(),
            1 => self.lift(&items[0]),
            n => {
                let mid = n / 2;
                let l = self.tree_acc(&items[..mid]);
                let r = self.tree_acc(&items[mid..]);
                self.merge(&l, &r)
            }
        }
    }

    /// Sequential fold into the raw accumulator, *without* projecting.
    pub fn fold_acc(&self, items: &[Vec<f64>]) -> Vec<f64> {
        let mut acc = self.identity.clone();
        for it in items {
            let el = self.lift(it);
            acc = self.merge(&acc, &el);
        }
        acc
    }

    /// `into`: lift one element into the carrier.
    pub fn lift(&self, item: &[f64]) -> Vec<f64> {
        let env = Env {
            item: Some(item),
            a: None,
            b: None,
            f: None,
        };
        self.into.iter().map(|e| eval(e, &env)).collect()
    }

    /// `combine`: the associative `⊗`.
    pub fn merge(&self, a: &[f64], b: &[f64]) -> Vec<f64> {
        let env = Env {
            item: None,
            a: Some(a),
            b: Some(b),
            f: None,
        };
        self.combine.iter().map(|e| eval(e, &env)).collect()
    }

    /// `project`: applied once, at the end.
    pub fn project(&self, acc: &[f64]) -> Vec<f64> {
        let env = Env {
            item: None,
            a: None,
            b: None,
            f: Some(acc),
        };
        self.project.iter().map(|e| eval(e, &env)).collect()
    }

    /// Render the derived carrier as readable math — so the *result* of a
    /// derivation can be inspected, not just trusted. (`xᵢ` = element field,
    /// `aᵢ`/`bᵢ` = the two accumulators, `sᵢ` = a state slot.)
    pub fn render(&self) -> String {
        let row = |v: &[Expr]| -> String {
            v.iter()
                .enumerate()
                .map(|(i, e)| format!("s{i} = {}", render_expr(e, 0)))
                .collect::<Vec<_>>()
                .join(";  ")
        };
        format!(
            "carrier ({} slots) [{}]\n  into:    {}\n  combine: {}\n  project: {}",
            self.slots,
            self.rules.join(", "),
            row(&self.into),
            row(&self.combine),
            self.project
                .iter()
                .map(|e| render_expr(e, 0))
                .collect::<Vec<_>>()
                .join(";  "),
        )
    }
}

// ── readable rendering of carrier expressions ────────────────────────────────

fn precedence(e: &Expr) -> u8 {
    match e {
        Expr::Add(..) | Expr::Sub(..) => 2,
        Expr::Mul(..) | Expr::Div(..) => 3,
        _ => 4, // atoms and function calls (max/min/exp/log) bind tightest
    }
}

/// Infix rendering with minimal parentheses. `parent` is the precedence of the
/// enclosing operator; we parenthesize only when this node binds more loosely.
fn render_expr(e: &Expr, parent: u8) -> String {
    let p = precedence(e);
    let num = |v: f64| {
        if v == f64::NEG_INFINITY {
            "-∞".to_string()
        } else if v == v.trunc() && v.abs() < 1e15 {
            format!("{}", v as i64)
        } else {
            format!("{v}")
        }
    };
    let s = match e {
        Expr::Const(v) => num(*v),
        Expr::Item(i) => format!("x{i}"),
        Expr::A(i) => format!("a{i}"),
        Expr::B(i) => format!("b{i}"),
        Expr::F(i) => format!("s{i}"),
        // left child at this precedence, right child one tighter so that the
        // non-associative `-` / `/` parenthesize their right operand correctly.
        Expr::Add(a, b) => format!("{} + {}", render_expr(a, p), render_expr(b, p)),
        Expr::Sub(a, b) => format!("{} - {}", render_expr(a, p), render_expr(b, p + 1)),
        Expr::Mul(a, b) => format!("{}·{}", render_expr(a, p), render_expr(b, p)),
        Expr::Div(a, b) => format!("{} / {}", render_expr(a, p), render_expr(b, p + 1)),
        Expr::Max(a, b) => format!("max({}, {})", render_expr(a, 0), render_expr(b, 0)),
        Expr::Min(a, b) => format!("min({}, {})", render_expr(a, 0), render_expr(b, 0)),
        Expr::Exp(a) => format!("exp({})", render_expr(a, 0)),
        Expr::Log(a) => format!("log({})", render_expr(a, 0)),
    };
    if p < parent { format!("({s})") } else { s }
}

// ── the compositional deriver ────────────────────────────────────────────────

/// One accumulator slot under construction.
struct Slot {
    kind: SlotKind,
    into: Expr, // per-element contribution, an Expr over `Item`
}

#[derive(Clone, Copy)]
enum SlotKind {
    /// Combine by a monoid directly: `A ⊕ B`.
    Plain(Monoid),
    /// The exp-domain ADD slot of the online-softmax monoid (R4): this slot is
    /// accumulated as `Σ exp(score − running_max)·raw`, where `max` is slot
    /// `max_slot`. On merge it telescopes: rescale by `exp(m − M_new)`.
    ExpShifted { max_slot: usize },
}

/// The result of streaming a sub-expression over the axis.
#[derive(Clone)]
enum S {
    /// Still indexed by the axis — a *per-element* value. `raw` is the
    /// multiplicand (over `Item`); `shift = Some(m)` means it carries an implicit
    /// `exp(score_m − running_max)` factor coupled to max slot `m` (R4); `post`
    /// is a factor over the *final* accumulator that a downstream additive
    /// reduction may pull out of the sum by distributivity (R5).
    Pe {
        raw: Expr,
        shift: Option<usize>,
        post: Expr,
    },
    /// Collapsed over the axis — a reduced value, an Expr over the final slots.
    Coll(Expr),
}

struct Ctx {
    slots: Vec<Slot>,
    /// Maps a FREE-along-axis leaf node to its `Item` field index.
    leaves: Vec<*const NodeKind>,
    /// Memoizes `go` per node so shared sub-expressions (the IR is a DAG) map to
    /// the same slots instead of registering duplicates.
    memo: Vec<(*const NodeKind, S)>,
    rules: BTreeSet<&'static str>,
}

impl Ctx {
    fn leaf(&mut self, node: &Node) -> usize {
        let ptr = Rc::as_ptr(node);
        if let Some(i) = self.leaves.iter().position(|p| *p == ptr) {
            i
        } else {
            self.leaves.push(ptr);
            self.leaves.len() - 1
        }
    }

    fn push_slot(&mut self, kind: SlotKind, into: Expr) -> usize {
        // R2: a `Map` was fused into the producer reduction iff the contribution
        // is a compound expression rather than a bare field/constant.
        if !matches!(into, Expr::Item(_) | Expr::Const(_)) {
            self.rules.insert("R2");
        }
        self.rules.insert("R1");
        self.slots.push(Slot { kind, into });
        if self.slots.len() > 1 {
            self.rules.insert("R3"); // a product carrier
        }
        self.slots.len() - 1
    }
}

/// Derive the streaming carrier for folding `node` over `axis`, by a single
/// bottom-up fold. Returns `None` when the axis is not foldable (SEQUENTIAL /
/// OPAQUE — §6) or the expression leaves the supported operator vocabulary (the
/// Stage 2B frontier, §7).
pub fn derive(node: &Node, axis: &str) -> Option<Carrier> {
    // An affine / SSM scan folds under a known monoid (affine-map composition),
    // not under R1–R5. It is the one extra "known carrier" the library ships.
    if let NodeKind::Scan {
        axis: sc,
        op: BinOp::AffineCompose,
        ..
    } = node.as_ref()
        && *sc == axis
    {
        return Some(affine_scan_carrier());
    }

    let mut ctx = Ctx {
        slots: Vec::new(),
        leaves: Vec::new(),
        memo: Vec::new(),
        rules: BTreeSet::new(),
    };
    let s = go(node, axis, &mut ctx)?;

    // The target must collapse the axis to a scalar answer.
    let project = match s {
        S::Coll(e) => vec![e],
        // A still-per-element result (e.g. softmax *weights*) has no one-pass
        // scalar projection; the caller should target the reduction instead.
        S::Pe { .. } => return None,
    };

    let (into, combine, identity) = assemble(&ctx.slots);
    Some(Carrier {
        slots: ctx.slots.len(),
        into,
        combine,
        identity,
        project,
        rules: ctx.rules.into_iter().collect(),
    })
}

/// Stream `node` over `axis`, registering any reductions-over-axis as slots.
/// Memoized per node, so DAG-shared sub-expressions register their slots once.
fn go(node: &Node, axis: &str, ctx: &mut Ctx) -> Option<S> {
    let ptr = Rc::as_ptr(node);
    if let Some((_, s)) = ctx.memo.iter().find(|(p, _)| *p == ptr) {
        return Some(s.clone());
    }
    let s = go_uncached(node, axis, ctx)?;
    ctx.memo.push((ptr, s.clone()));
    Some(s)
}

fn go_uncached(node: &Node, axis: &str, ctx: &mut Ctx) -> Option<S> {
    // A maximal sub-expression that is FREE along the axis is, from the axis's
    // point of view, a per-element constant: treat it as a leaf element input.
    // (Maps are always decomposed so their elementwise work folds into `into` —
    // that is rule R2.)
    if !matches!(node.as_ref(), NodeKind::Map { .. })
        && structure(node, axis).level == Parallelism::Free
    {
        return Some(S::Pe {
            raw: Expr::Item(ctx.leaf(node)),
            shift: None,
            post: cst(1.0),
        });
    }

    match node.as_ref() {
        NodeKind::Map { f, inputs } => map(f.name, inputs, axis, ctx),

        NodeKind::Reduce {
            src,
            axis: red,
            op,
        } if *red == axis => reduce(src, *op, axis, ctx),

        // A reduction over a *different* axis collapses something orthogonal; if
        // it is not FREE along our axis we leave the supported fragment.
        _ => None,
    }
}

/// Reduce `src` over `axis` with monoid `op`, allocating slot(s).
fn reduce(src: &Node, op: BinOp, axis: &str, ctx: &mut Ctx) -> Option<S> {
    let BinOp::Monoid(m) = op else {
        return None; // non-associative / affine handled elsewhere
    };
    let s = go(src, axis, ctx)?;

    match m {
        Monoid::Add => {
            let S::Pe { raw, shift, post } = s else {
                return None; // cannot re-reduce an already-collapsed axis
            };
            let kind = match shift {
                Some(max_slot) => {
                    ctx.rules.insert("R4"); // coupled (rescaling) reduction
                    SlotKind::ExpShifted { max_slot }
                }
                None => SlotKind::Plain(Monoid::Add),
            };
            let slot = ctx.push_slot(kind, raw);
            if !is1(&post) {
                // R5: a normalizer (a value collapsed over the same axis) factored
                // out of this linear reduction and is applied once, in `project`.
                ctx.rules.insert("R5");
            }
            Some(S::Coll(pmul(post, Expr::F(slot))))
        }

        Monoid::LogSumExp => {
            // log-space reduction = the same (max, Σexp) coupling, projected by
            // `log(s) + m`. Built from the rules, not stored.
            let S::Pe {
                raw,
                shift: None,
                post,
            } = s
            else {
                return None;
            };
            if !is1(&post) {
                return None;
            }
            let max_slot = ctx.push_slot(SlotKind::Plain(Monoid::Max), raw);
            ctx.rules.insert("R4");
            let sum_slot = ctx.push_slot(SlotKind::ExpShifted { max_slot }, cst(1.0));
            Some(S::Coll(padd(log(Expr::F(sum_slot)), Expr::F(max_slot))))
        }

        // max / min / product: plain monoid slots; no coupling or deferral.
        Monoid::Max | Monoid::Min | Monoid::Mul => {
            let S::Pe {
                raw,
                shift: None,
                post,
            } = s
            else {
                return None;
            };
            if !is1(&post) {
                return None;
            }
            let slot = ctx.push_slot(SlotKind::Plain(m), raw);
            Some(S::Coll(Expr::F(slot)))
        }
    }
}

/// Combine the streamed inputs of an elementwise map.
fn map(name: &str, inputs: &[Node], axis: &str, ctx: &mut Ctx) -> Option<S> {
    match (name, inputs.len()) {
        // x ↦ 1 — materializes a count (mean's denominator).
        ("one", 1) => Some(S::Pe {
            raw: cst(1.0),
            shift: None,
            post: cst(1.0),
        }),

        // exp(x − m), where `m` is a MAX reduction over the same axis. This is
        // the coupling site (R4): the result rides the exp domain of `m`. The
        // element's own score is its local max, so the unit contribution is
        // `exp(score − score) = 1`.
        ("exp_sub", 2) => {
            go(&inputs[0], axis, ctx)?; // the score x — validate/register the leaf
            let m = go(&inputs[1], axis, ctx)?;
            let max_slot = coupled_max_slot(&m, ctx)?;
            Some(S::Pe {
                raw: cst(1.0),
                shift: Some(max_slot),
                post: cst(1.0),
            })
        }

        ("log", 1) => match go(&inputs[0], axis, ctx)? {
            S::Coll(e) => Some(S::Coll(log(e))),
            S::Pe { .. } => None,
        },

        ("mul", 2) => binop(&inputs[0], &inputs[1], axis, ctx, Bin::Mul),
        ("div", 2) => binop(&inputs[0], &inputs[1], axis, ctx, Bin::Div),
        ("add", 2) => binop(&inputs[0], &inputs[1], axis, ctx, Bin::Add),
        ("sub", 2) => binop(&inputs[0], &inputs[1], axis, ctx, Bin::Sub),
        _ => None,
    }
}

#[derive(Clone, Copy)]
enum Bin {
    Mul,
    Div,
    Add,
    Sub,
}

fn binop(x: &Node, y: &Node, axis: &str, ctx: &mut Ctx, op: Bin) -> Option<S> {
    let a = go(x, axis, ctx)?;
    let b = go(y, axis, ctx)?;
    match (op, a, b) {
        // Both collapsed → a plain scalar combination of reduced values.
        (Bin::Mul, S::Coll(p), S::Coll(q)) => Some(S::Coll(pmul(p, q))),
        (Bin::Div, S::Coll(p), S::Coll(q)) => Some(S::Coll(pdiv(p, q))),
        (Bin::Add, S::Coll(p), S::Coll(q)) => Some(S::Coll(padd(p, q))),
        (Bin::Sub, S::Coll(p), S::Coll(q)) => Some(S::Coll(sub(p, q))),

        // Per-element × / ÷ a value collapsed over the *same* axis. By
        // distributivity (semiring linearity) the collapsed factor is constant
        // w.r.t. the axis, so it is *deferred*: pushed into `post` and applied
        // once after the downstream reduction. This is the mechanism of R5.
        (Bin::Mul, S::Pe { raw, shift, post }, S::Coll(q))
        | (Bin::Mul, S::Coll(q), S::Pe { raw, shift, post }) => Some(S::Pe {
            raw,
            shift,
            post: pmul(post, q),
        }),
        (Bin::Div, S::Pe { raw, shift, post }, S::Coll(q)) => Some(S::Pe {
            raw,
            shift,
            post: pdiv(post, q),
        }),

        // Two per-element values: combine pointwise. Shifts must agree (you
        // cannot mix two different exp domains in one element).
        (Bin::Mul, S::Pe { raw: r1, shift: s1, post: p1 }, S::Pe { raw: r2, shift: s2, post: p2 }) => {
            Some(S::Pe {
                raw: pmul(r1, r2),
                shift: merge_shift(s1, s2)?,
                post: pmul(p1, p2),
            })
        }
        (Bin::Add, S::Pe { raw: r1, shift: None, post: p1 }, S::Pe { raw: r2, shift: None, post: p2 })
            if is1(&p1) && is1(&p2) =>
        {
            Some(S::Pe { raw: padd(r1, r2), shift: None, post: cst(1.0) })
        }
        (Bin::Sub, S::Pe { raw: r1, shift: None, post: p1 }, S::Pe { raw: r2, shift: None, post: p2 })
            if is1(&p1) && is1(&p2) =>
        {
            Some(S::Pe { raw: sub(r1, r2), shift: None, post: cst(1.0) })
        }
        _ => None,
    }
}

fn merge_shift(a: Option<usize>, b: Option<usize>) -> Option<Option<usize>> {
    match (a, b) {
        (None, x) | (x, None) => Some(x),
        (Some(i), Some(j)) if i == j => Some(Some(i)),
        _ => None, // two distinct exp domains — unsupported
    }
}

/// If `m` is exactly a MAX reduction over the axis (a single collapsed slot that
/// is `Plain(Max)`), return that slot index — the coupling target for an
/// `exp_sub` shift.
fn coupled_max_slot(m: &S, ctx: &Ctx) -> Option<usize> {
    let S::Coll(Expr::F(i)) = m else { return None };
    match ctx.slots.get(*i)?.kind {
        SlotKind::Plain(Monoid::Max) => Some(*i),
        _ => None,
    }
}

/// Turn the registered slots into `(into, combine, identity)`.
fn assemble(slots: &[Slot]) -> (Vec<Expr>, Vec<Expr>, Vec<f64>) {
    let into = slots.iter().map(|s| s.into.clone()).collect();
    let combine = slots
        .iter()
        .enumerate()
        .map(|(i, s)| match s.kind {
            SlotKind::Plain(Monoid::Add) | SlotKind::Plain(Monoid::LogSumExp) => {
                Expr::Add(Box::new(Expr::A(i)), Box::new(Expr::B(i)))
            }
            SlotKind::Plain(Monoid::Mul) => Expr::Mul(Box::new(Expr::A(i)), Box::new(Expr::B(i))),
            SlotKind::Plain(Monoid::Max) => emax(Expr::A(i), Expr::B(i)),
            SlotKind::Plain(Monoid::Min) => emin(Expr::A(i), Expr::B(i)),
            SlotKind::ExpShifted { max_slot: mx } => {
                // s' = sₐ·exp(mₐ − M) + s_b·exp(m_b − M),  M = max(mₐ, m_b)
                let big = emax(Expr::A(mx), Expr::B(mx));
                let ra = exp(sub(Expr::A(mx), big.clone()));
                let rb = exp(sub(Expr::B(mx), big));
                padd(pmul(Expr::A(i), ra), pmul(Expr::B(i), rb))
            }
        })
        .collect();
    let identity = slots
        .iter()
        .map(|s| match s.kind {
            SlotKind::Plain(m) => m.identity().unwrap(),
            SlotKind::ExpShifted { .. } => 0.0,
        })
        .collect();
    (into, combine, identity)
}

/// The SSM / linear-attention carrier: the affine map `(A, b)` under composition
/// (§5.4). `combine(L, R) = R ∘ L`, `identity = (1, 0)`, `project` applies the
/// map to `h₀ = 0` (returns `b`).
fn affine_scan_carrier() -> Carrier {
    Carrier {
        slots: 2,
        into: vec![Expr::Item(0), Expr::Item(1)], // (A_t, b_t)
        // (A', b') for R∘L with L first:  x ↦ A_R(A_L x + b_L) + b_R
        combine: vec![
            Expr::Mul(Box::new(Expr::B(0)), Box::new(Expr::A(0))),
            padd(
                Expr::Mul(Box::new(Expr::B(0)), Box::new(Expr::A(1))),
                Expr::B(1),
            ),
        ],
        identity: vec![1.0, 0.0], // identity affine map
        project: vec![Expr::F(1)],
        rules: vec!["affine-compose"],
    }
}
