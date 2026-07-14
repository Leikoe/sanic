//! Derivation: turn a foldable axis into a concrete streaming accumulator.
//!
//! This is the heart of the engine, and it is one bottom-up fold over the IR.
//! It never asks "is this FlashAttention?" — it asks, of each sub-expression,
//! "what does streaming *this* contribute to the accumulator?", and composes
//! the answers. Because the elementwise vocabulary is a closed basis
//! ([`MapOp`]), the dispatch below is total over it: there are no named
//! special forms. The online-softmax coupling is *discovered* from plain
//! composition — `Exp(x − m)` where `m` is the running max of the same
//! stream — not matched against a fused `exp_sub` op.
//!
//! A fixed set of things can happen during a derivation, and the carrier
//! records which ones did (its `rules`), in plain words:
//!
//! * `fold` — a reduction along the axis became an accumulator slot.
//! * `fused-map` — elementwise work was folded into the per-element lift, so
//!   no intermediate is ever materialized.
//! * `tuple` — the answer needed more than one slot (sum *and* count, max
//!   *and* Σexp, …).
//! * `rescale` — a slot rides the exp-domain of a running max and must be
//!   rescaled by `exp(m_old − m_new)` on every merge. This is the
//!   online-softmax coupling, detected by data dependence.
//! * `defer-div` — a normalizer (a value folded over the same axis) was
//!   pulled out of a linear reduction by distributivity and is applied once,
//!   at the end.
//! * `k-best` — an index-carrying selection ([`BinOp::ArgMax`] /
//!   [`BinOp::TopK`]): sorted (value, index) lists of length ≤ k form a
//!   tuple monoid, so argmax and every top-k rank are single folds.
//! * `invariant` — a reduction over an axis its operand does not vary along:
//!   `Σ = n·value` (the count is itself a slot), `max/min/lse = value (+ln n)`.
//! * `lattice` — `reduce_m(max/min(z, c))` for m ∈ {Max, Min} and `c`
//!   collapsed over the same axis: order homomorphisms commute, so the
//!   coupling collapses after the fold (`min_i max(z_i, c) = max(min z, c)`).
//! * `defer-add` — `z ± c` with `c` collapsed: offsets commute with order
//!   reductions and leave a sum through a count slot (`Σ(z+c) = Σz + n·c`).
//! * `defer-scale` — an extremum of `c·z`: the sign of `c` decides which
//!   extremum survives, so BOTH are carried and project dispatches on sign.
//!
//! Soundness and completeness are guarded separately. Soundness: every
//! carrier is executable data, run against the interpreter (`tests/laws.rs`
//! holds each rule to `run_carrier == eval`, ties and sign flips included).
//! Completeness: "fusable" has a semantic definition independent of this
//! file — h streams iff some constant-size sketch of the prefix determines
//! every extension (a list homomorphism into a small carrier, tested
//! Myhill–Nerode-style by collision probing) — and `tests/completeness.rs`
//! holds DECLINES to it: a declined program whose carrier the probe can
//! exhibit is a red test, not a benchmark surprise waiting to happen. The
//! `invariant`/`lattice`/`defer-add`/`defer-scale` rules above were found
//! exactly that way.
//!
//! A derived carrier is *data* — slots plus three symbolic programs (`into`,
//! `combine`, `project`) — so it can be executed by the interpreter below,
//! property-tested against a reference, and transcribed to real code by the
//! emitters. The FlashAttention `(m, ℓ, o)` accumulator is never written down
//! anywhere in this repository; it is constructed by this fold.

use std::collections::BTreeSet;
use std::rc::Rc;

use crate::analyze::{Parallelism, structure};
use crate::ir::{Axis, BinOp, MapOp, Monoid, Node, NodeKind, output_axes};

// ── symbolic expressions over carrier slots ──────────────────────────────────

/// A pure scalar expression. `Item(i)` reads field `i` of the element being
/// lifted (used only in `into`); `A(i)`/`B(i)` read field `i` of the two
/// accumulators being combined (used only in `combine`); `F(i)` reads field
/// `i` of the final accumulator (used only in `project`). The variants
/// mirror the closed [`MapOp`] basis.
#[derive(Debug, Clone, PartialEq)]
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
    /// `a < b` as 1.0 / 0.0.
    Lt(Box<Expr>, Box<Expr>),
    Exp(Box<Expr>),
    Log(Box<Expr>),
    Sqrt(Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    /// `cond != 0 ? a : b`.
    Where(Box<Expr>, Box<Expr>, Box<Expr>),
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
fn elt(a: Expr, b: Expr) -> Expr {
    Expr::Lt(Box::new(a), Box::new(b))
}
fn exp(a: Expr) -> Expr {
    Expr::Exp(Box::new(a))
}
fn log(a: Expr) -> Expr {
    Expr::Log(Box::new(a))
}
fn esqrt(a: Expr) -> Expr {
    Expr::Sqrt(Box::new(a))
}
fn esin(a: Expr) -> Expr {
    Expr::Sin(Box::new(a))
}
fn ecos(a: Expr) -> Expr {
    Expr::Cos(Box::new(a))
}
fn ewhere(c: Expr, a: Expr, b: Expr) -> Expr {
    Expr::Where(Box::new(c), Box::new(a), Box::new(b))
}

// simplifying constructors — fold the 0/1 units away so derived carriers stay
// readable (e.g. `o`'s lift is `x1`, not `1·x1`).
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
    // Push multiplication through division so a deferred normalizer stays one
    // fraction: (n/d)·c becomes (n·c)/d, i.e. (1/ℓ)·o renders as o/ℓ.
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
/// `exp(a − b)` with the peepholes that keep derived carriers tidy:
/// `x − x → 0` and `exp(0) → 1`.
fn pexp_sub(a: Expr, b: Expr) -> Expr {
    if a == b {
        return cst(1.0);
    }
    exp(sub(a, b))
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
        Expr::Lt(a, b) => {
            if eval(a, env) < eval(b, env) {
                1.0
            } else {
                0.0
            }
        }
        Expr::Exp(a) => eval(a, env).exp(),
        Expr::Log(a) => eval(a, env).ln(),
        Expr::Sqrt(a) => eval(a, env).sqrt(),
        Expr::Sin(a) => eval(a, env).sin(),
        Expr::Cos(a) => eval(a, env).cos(),
        Expr::Where(c, a, b) => {
            if eval(c, env) != 0.0 {
                eval(a, env)
            } else {
                eval(b, env)
            }
        }
    }
}

// ── the derived carrier ──────────────────────────────────────────────────────

/// A concrete, executable streaming accumulator:
/// `into` lifts one element in, `combine` is the associative merge, `project`
/// turns the final state into the answer. When extra state was needed to make
/// the fold associative, `project` is what discards it again.
#[derive(Debug, Clone)]
pub struct Carrier {
    pub slots: usize,
    /// The fold's per-element inputs, in `Item` index order: the maximal
    /// sub-expressions that are FREE along the streamed axis. This is the
    /// fusion boundary — a leaf that is not a raw `Input` must either be
    /// computed in-body or materialized by a producer kernel (`partition`
    /// decides which).
    pub leaves: Vec<Node>,
    pub into: Vec<Expr>,
    pub combine: Vec<Expr>,
    pub identity: Vec<f64>,
    pub project: Vec<Expr>,
    /// The free axes each slot spans (streamed axis excluded). The exact
    /// accumulator size for a tile is `Σ_slots Π extents[span]` — `acc_scalars`.
    pub spans: Vec<Vec<Axis>>,
    /// Which of the five derivation moves fired, in plain words (see the
    /// module doc): `fold`, `fused-map`, `tuple`, `rescale`, `defer-div`.
    pub rules: Vec<&'static str>,
    /// What kind of reduction each slot is. The emitters read this to pick the
    /// intra-tile operation without pattern-matching the computation.
    pub kinds: Vec<SlotKind>,
}

impl Carrier {
    /// Fold left-to-right, O(1) state — the streaming path.
    pub fn fold(&self, items: &[Vec<f64>]) -> Vec<f64> {
        self.project(&self.fold_acc(items))
    }

    /// Fold by recursive bisection — the parallel path. Agreeing with `fold`
    /// at every split point is the associativity certificate.
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

    /// Sequential fold into the raw accumulator, without projecting.
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

    /// `combine`: the associative merge.
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

    /// [`Carrier::project`] with per-grid-point leaf values available: a
    /// projection may read leaves that are CONSTANT along the streamed axis
    /// (a grid-axis one-hot selecting which slot this output wants — the
    /// rank-indexed k-best projection). Runners that know the grid point
    /// use this; stream-varying leaves are meaningless here and the caller
    /// must not supply them (their slots are NaN-poisoned by
    /// [`crate::interp::run_carrier`]).
    pub fn project_with(&self, acc: &[f64], items: &[f64]) -> Vec<f64> {
        let env = Env {
            item: Some(items),
            a: None,
            b: None,
            f: Some(acc),
        };
        self.project.iter().map(|e| eval(e, &env)).collect()
    }

    /// Does the projection read any leaf? (If so, only runners that supply
    /// per-grid-point leaf values — and emitters that render leaf loads in
    /// project scope — can drive this carrier; split/cooperative schedules
    /// decline.)
    pub fn project_reads_leaves(&self) -> bool {
        self.project.iter().any(|e| !items_of(e).is_empty())
    }

    /// Exact accumulator size (scalar count) for given axis extents — the
    /// number the scheduler feeds into its SRAM constraint. A slot spanning no
    /// free axes is one scalar; a slot spanning `{sq, e}` is
    /// `extent(sq)·extent(e)`.
    pub fn acc_scalars(&self, extent: impl Fn(Axis) -> f64) -> f64 {
        self.spans
            .iter()
            .map(|span| span.iter().map(|&a| extent(a)).product::<f64>())
            .sum()
    }

    /// Render the carrier as readable math, so a derivation can be inspected,
    /// not just trusted. (`xᵢ` = element field, `aᵢ`/`bᵢ` = the two
    /// accumulators, `sᵢ` = a state slot.)
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

// ── readable rendering ───────────────────────────────────────────────────────

fn precedence(e: &Expr) -> u8 {
    match e {
        Expr::Lt(..) => 1,
        Expr::Add(..) | Expr::Sub(..) => 2,
        Expr::Mul(..) | Expr::Div(..) => 3,
        _ => 4, // atoms and function calls bind tightest
    }
}

/// Infix rendering with minimal parentheses. `parent` is the precedence of the
/// enclosing operator; parenthesize only when this node binds more loosely.
fn render_expr(e: &Expr, parent: u8) -> String {
    let p = precedence(e);
    let num = |v: f64| {
        if v == f64::NEG_INFINITY {
            "-∞".to_string()
        } else if v == v.trunc() && v.abs() < 1e15 {
            format!("{}", v as i64)
        } else {
            format!("{v:e}")
        }
    };
    let s = match e {
        Expr::Const(v) => num(*v),
        Expr::Item(i) => format!("x{i}"),
        Expr::A(i) => format!("a{i}"),
        Expr::B(i) => format!("b{i}"),
        Expr::F(i) => format!("s{i}"),
        // left child at this precedence, right child one tighter so the
        // non-associative `-` / `/` parenthesize their right operand.
        Expr::Add(a, b) => format!("{} + {}", render_expr(a, p), render_expr(b, p)),
        Expr::Sub(a, b) => format!("{} - {}", render_expr(a, p), render_expr(b, p + 1)),
        Expr::Mul(a, b) => format!("{}·{}", render_expr(a, p), render_expr(b, p)),
        Expr::Div(a, b) => format!("{} / {}", render_expr(a, p), render_expr(b, p + 1)),
        Expr::Lt(a, b) => format!("{} < {}", render_expr(a, p + 1), render_expr(b, p + 1)),
        Expr::Max(a, b) => format!("max({}, {})", render_expr(a, 0), render_expr(b, 0)),
        Expr::Min(a, b) => format!("min({}, {})", render_expr(a, 0), render_expr(b, 0)),
        Expr::Exp(a) => format!("exp({})", render_expr(a, 0)),
        Expr::Log(a) => format!("log({})", render_expr(a, 0)),
        Expr::Sqrt(a) => format!("sqrt({})", render_expr(a, 0)),
        Expr::Sin(a) => format!("sin({})", render_expr(a, 0)),
        Expr::Cos(a) => format!("cos({})", render_expr(a, 0)),
        Expr::Where(c, a, b) => format!(
            "where({}, {}, {})",
            render_expr(c, 0),
            render_expr(a, 0),
            render_expr(b, 0)
        ),
    };
    if p < parent { format!("({s})") } else { s }
}

// ── the deriver ──────────────────────────────────────────────────────────────

/// One accumulator slot under construction.
struct Slot {
    kind: SlotKind,
    into: Expr,      // per-element contribution, over `Item`
    span: Vec<Axis>, // free axes this slot ranges over (streamed axis excluded)
}

#[derive(Debug, Clone, Copy)]
pub enum SlotKind {
    /// Combine by a monoid directly: `A ⊕ B`.
    Plain(Monoid),
    /// The exp-domain sum of an online softmax: accumulated as
    /// `Σ exp(score − running_max)·raw`, where the running max is slot
    /// `max_slot`. On merge it telescopes — rescale by `exp(m − M_new)`.
    ExpShifted { max_slot: usize },
    /// Affine-map composition (the SSM carrier). Not a scalar monoid slot;
    /// emitters must handle it separately or decline.
    AffineStep,
    /// The index half of an index-carrying maximum: merged as
    /// `a₀ < b₀ ? b_i : a_i` against max slot `max_slot` — first max wins.
    ArgIdx { max_slot: usize },
    /// Value slot `rank` of a k-best selection (descending, first-max-wins);
    /// `base` is the rank-0 slot and ranks are contiguous from it. The
    /// combine is the SINGLETON insert `merge(A, [b])` — exact for
    /// element-at-a-time streaming, NOT a two-list merge, so split
    /// reductions must decline (guarded in `run_carrier_split` /
    /// `emit_split_metal`).
    KBestVal { base: usize, rank: usize },
    /// Index slot `rank` of a k-best selection; `vbase`/`ibase` are the
    /// rank-0 value/index slots. Same singleton-insert caveat as
    /// [`SlotKind::KBestVal`].
    KBestIdx { vbase: usize, ibase: usize, rank: usize },
}

/// What streaming a sub-expression over the axis produced so far.
#[derive(Clone)]
enum S {
    /// Still indexed by the axis — a per-element value. `raw` is the
    /// contribution (over `Item`); `shift = Some(m)` means it carries an
    /// implicit `exp(score − running_max)` factor tied to max slot `m`; `post`
    /// is a factor over the *final* accumulator that a downstream additive
    /// reduction may pull out of the sum by distributivity.
    Pe {
        raw: Expr,
        shift: Option<usize>,
        post: Expr,
    },
    /// A per-element value minus the collapsed running max over the same
    /// axis (`x − m`) — the intermediate state of an online-softmax shift.
    /// Only `Exp` may consume it; that consumption is where the coupling is
    /// discovered.
    PeOff { raw: Expr, max_slot: usize },
    /// A per-element value max/min-ed with a value collapsed over the SAME
    /// axis (`max(z, c)` / `min(z, c)`): the lattice-coupling intermediate.
    /// Only a Max/Min reduction may consume it — lattice distributivity
    /// (`min_i max(z_i, c) = max(min_i z_i, c)`) is where it collapses.
    /// Found by the completeness probe.
    PeExt { raw: Expr, coll: Expr, is_max: bool },
    /// A per-element value plus a value collapsed over the same axis
    /// (`z + c`): the additive-coupling intermediate. Max/Min pull the
    /// offset out unchanged; Add pulls out `n·c` through a count slot.
    /// Found by the completeness probe.
    PeAdd { raw: Expr, off: Expr },
    /// Collapsed over the axis — a reduced value, over the final slots.
    Coll(Expr),
}

/// A collapsed expression, if this value is (or can be promoted to) one:
/// genuine `Coll`s, and per-element values that read no element fields — a
/// literal constant is the same on every element, so it is a valid collapsed
/// value too. This is what lets `mean = Σx² · (1/n)` stay in the collapsed
/// algebra when `1/n` is a `Const`.
fn as_coll(s: &S) -> Option<Expr> {
    match s {
        S::Coll(e) => Some(e.clone()),
        S::Pe {
            raw,
            shift: None,
            post,
        } if is1(post) && items_of(raw).is_empty() => Some(raw.clone()),
        _ => None,
    }
}

/// A plain per-element expression (no shift, no deferred factor), if any.
fn plain_pe(s: &S) -> Option<Expr> {
    match s {
        S::Pe {
            raw,
            shift: None,
            post,
        } if is1(post) => Some(raw.clone()),
        _ => None,
    }
}

struct Ctx {
    slots: Vec<Slot>,
    /// Each leaf (a sub-expression free along the axis) and its free axes.
    /// Leaves are the fold's per-element inputs — and therefore the fusion
    /// boundary: anything here that is not a raw `Input` must either be
    /// computed in-body or materialized by another kernel.
    leaves: Vec<(Node, Vec<Axis>)>,
    /// Memoizes `go` per node so DAG-shared sub-expressions map to the same
    /// slots instead of registering duplicates.
    memo: Vec<(*const NodeKind, S)>,
    rules: BTreeSet<&'static str>,
    /// Memoizes `other_axis_folds` per node — the free-map-vs-contraction check.
    other_folds: std::collections::HashMap<*const NodeKind, (bool, bool)>,
}

impl Ctx {
    fn leaf(&mut self, node: &Node, free: Vec<Axis>) -> usize {
        if let Some(i) = self.leaves.iter().position(|(n, _)| Rc::ptr_eq(n, node)) {
            i
        } else {
            self.leaves.push((node.clone(), free));
            self.leaves.len() - 1
        }
    }

    fn push_slot(&mut self, kind: SlotKind, into: Expr) -> usize {
        // A compound contribution means elementwise work was fused into the
        // lift instead of materializing an intermediate.
        if !matches!(into, Expr::Item(_) | Expr::Const(_)) {
            self.rules.insert("fused-map");
        }
        self.rules.insert("fold");
        // A slot spans the free axes of every leaf it reads, plus — for an
        // exp-shifted slot — the axes of the max slot it rides.
        let mut span: Vec<Axis> = Vec::new();
        for i in items_of(&into) {
            for &a in &self.leaves[i].1 {
                if !span.contains(&a) {
                    span.push(a);
                }
            }
        }
        if let SlotKind::ExpShifted { max_slot } = kind {
            for &a in &self.slots[max_slot].span {
                if !span.contains(&a) {
                    span.push(a);
                }
            }
        }
        self.slots.push(Slot { kind, into, span });
        if self.slots.len() > 1 {
            self.rules.insert("tuple");
        }
        self.slots.len() - 1
    }
}

/// Every `Item` in `e` references a leaf that never touches `axis`: the
/// expression is constant along the fold and may evaluate at project time.
fn invariant_along(e: &Expr, axis: Axis, ctx: &Ctx) -> bool {
    items_of(e)
        .iter()
        .all(|&i| !crate::ir::output_axes(&ctx.leaves[i].0).contains(&axis))
}

/// The slot indices (`F`) an expression reads.
fn slots_of(e: &Expr) -> Vec<usize> {
    fn walk(e: &Expr, out: &mut Vec<usize>) {
        match e {
            Expr::F(i) => out.push(*i),
            Expr::Add(a, b)
            | Expr::Sub(a, b)
            | Expr::Mul(a, b)
            | Expr::Div(a, b)
            | Expr::Max(a, b)
            | Expr::Min(a, b)
            | Expr::Lt(a, b) => {
                walk(a, out);
                walk(b, out);
            }
            Expr::Exp(a) | Expr::Log(a) | Expr::Sqrt(a) | Expr::Sin(a) | Expr::Cos(a) => {
                walk(a, out)
            }
            Expr::Where(c, a, b) => {
                walk(c, out);
                walk(a, out);
                walk(b, out);
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    walk(e, &mut out);
    out
}

/// The `Item` field indices an expression reads.
pub(crate) fn items_of(e: &Expr) -> Vec<usize> {
    let mut out = Vec::new();
    fn walk(e: &Expr, out: &mut Vec<usize>) {
        match e {
            Expr::Item(i) => out.push(*i),
            Expr::Add(a, b)
            | Expr::Sub(a, b)
            | Expr::Mul(a, b)
            | Expr::Div(a, b)
            | Expr::Max(a, b)
            | Expr::Min(a, b)
            | Expr::Lt(a, b) => {
                walk(a, out);
                walk(b, out);
            }
            Expr::Exp(a) | Expr::Log(a) | Expr::Sqrt(a) | Expr::Sin(a) | Expr::Cos(a) => {
                walk(a, out)
            }
            Expr::Where(c, a, b) => {
                walk(c, out);
                walk(a, out);
                walk(b, out);
            }
            _ => {}
        }
    }
    walk(e, &mut out);
    out
}

/// Derive the streaming carrier for folding `node` over `axis`. `None` when
/// the axis is not foldable (serial or data-dependent) or the expression
/// leaves the supported fragment.
pub fn derive(node: &Node, axis: Axis) -> Option<Carrier> {
    // An affine / SSM scan folds under a known monoid (affine-map
    // composition), not under the compositional rules. It is the one extra
    // carrier the library ships.
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
        other_folds: std::collections::HashMap::new(),
    };
    let s = go(node, axis, &mut ctx)?;

    // The target must collapse the axis to a scalar answer. A still-per-element
    // result (e.g. softmax *weights*) has no one-pass scalar projection; the
    // caller should target the reduction that consumes it instead.
    let project = match s {
        S::Coll(e) => vec![e],
        S::Pe { .. } | S::PeOff { .. } | S::PeExt { .. } | S::PeAdd { .. } => return None,
    };

    let (into, combine, identity) = assemble(&ctx.slots);
    let spans = ctx.slots.iter().map(|s| s.span.clone()).collect();
    let kinds = ctx.slots.iter().map(|s| s.kind).collect();
    let leaves = ctx.leaves.iter().map(|(n, _)| n.clone()).collect();
    Some(Carrier {
        slots: ctx.slots.len(),
        leaves,
        into,
        combine,
        identity,
        project,
        spans,
        rules: ctx.rules.into_iter().collect(),
        kinds,
    })
}

/// Classify the folds over axes OTHER than `axis` inside a free-along-`axis`
/// sub-expression: `(has_plain_reduction, has_contraction)`. A logsumexp's
/// `max`/`Σexp` are plain single-tensor reductions; an attention score or a
/// GEMM is a two-tensor contraction the emitters compute in-body (or cut as a
/// separate GEMM). A free map worth keeping WHOLE wraps only plain
/// reductions — wrapping a contraction, it must stay decomposed so the matmul
/// machinery still sees it.
fn other_axis_folds(
    node: &Node,
    axis: Axis,
    cache: &mut std::collections::HashMap<*const NodeKind, (bool, bool)>,
) -> (bool, bool) {
    // Memoized per node (the streamed axis is fixed for a whole derivation):
    // a DAG-shared subtree is classified once. Unmemoized, this re-walks shared
    // subtrees and is exponential on backward graphs.
    let ptr = Rc::as_ptr(node);
    if let Some(&r) = cache.get(&ptr) {
        return r;
    }
    let is_contraction = matches!(node.as_ref(),
        NodeKind::Reduce { src, op: BinOp::Monoid(Monoid::Add), axis: a }
            if *a != axis
            && matches!(src.as_ref(),
                NodeKind::Map { op: MapOp::Mul, inputs } if inputs.len() == 2));
    let result = match node.as_ref() {
        NodeKind::Reduce { src, axis: a, .. } | NodeKind::Scan { src, axis: a, .. } => {
            let (plain, contr) = other_axis_folds(src, axis, cache);
            match (*a != axis, is_contraction) {
                (true, true) => (plain, true),
                (true, false) => (true, contr),
                (false, _) => (plain, contr),
            }
        }
        NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => (false, false),
        NodeKind::Map { inputs, .. } => {
            let (mut p, mut c) = (false, false);
            for i in inputs {
                let (p2, c2) = other_axis_folds(i, axis, cache);
                p |= p2;
                c |= c2;
            }
            (p, c)
        }
        NodeKind::Gather { src, index, .. } => {
            let (p, c) = other_axis_folds(src, axis, cache);
            let (p2, c2) = other_axis_folds(index, axis, cache);
            (p || p2, c || c2)
        }
        NodeKind::View { src, .. } | NodeKind::Reindex { src, .. } => {
            other_axis_folds(src, axis, cache)
        }
    };
    cache.insert(ptr, result);
    result
}

/// Stream `node` over `axis`, registering reductions-over-axis as slots.
/// Memoized per node, so DAG-shared sub-expressions register once.
fn go(node: &Node, axis: Axis, ctx: &mut Ctx) -> Option<S> {
    let ptr = Rc::as_ptr(node);
    if let Some((_, s)) = ctx.memo.iter().find(|(p, _)| *p == ptr) {
        return Some(s.clone());
    }
    let s = go_uncached(node, axis, ctx)?;
    ctx.memo.push((ptr, s.clone()));
    Some(s)
}

fn go_uncached(node: &Node, axis: Axis, ctx: &mut Ctx) -> Option<S> {
    // A literal lifts to a constant expression, never a leaf.
    if let NodeKind::Const { v } = node.as_ref() {
        return Some(S::Pe {
            raw: cst(*v),
            shift: None,
            post: cst(1.0),
        });
    }

    // A maximal sub-expression that is FREE along the axis is, from the
    // axis's point of view, a per-element constant: treat it as a leaf input
    // (raw tensors, iotas, values reduced over other axes, views…). Maps are
    // decomposed first so their elementwise work fuses into the lift — EXCEPT a
    // free map that wraps a fold over another axis. Decomposing a free map buys
    // no algebra (the defer/invariant rules act on the STREAMED axis, which a
    // free map does not touch); it only splits a linear combination of nested
    // folds — logsumexp's `m + log(Σ exp(x − m))` — into a separate leaf per
    // fold, so each becomes its own kernel and the shared inner reduction (the
    // running max) is duplicated. Kept whole, the combination cuts once and
    // re-derives as a single carrier downstream.
    let is_free = structure(node, axis).level == Parallelism::Free;
    let is_map = matches!(node.as_ref(), NodeKind::Map { .. });
    // Keep a free map whole only when it wraps plain reductions and no
    // contraction — logsumexp's `m + log(Σexp)`, yes; `scale·QKᵀ + mask` or
    // `silu(gate)·up`, no (those decompose so the matmul stays in-body / cut).
    let keep_map_whole = is_map && {
        let (plain, contraction) = other_axis_folds(node, axis, &mut ctx.other_folds);
        plain && !contraction
    };
    if is_free && (!is_map || keep_map_whole) {
        // The leaf's free axes are its output shape minus the streamed axis.
        let free = output_axes(node)
            .into_iter()
            .filter(|a| *a != axis)
            .collect();
        return Some(S::Pe {
            raw: Expr::Item(ctx.leaf(node, free)),
            shift: None,
            post: cst(1.0),
        });
    }

    match node.as_ref() {
        NodeKind::Map { op, inputs } => {
            // Roll back on failure: a half-decomposed map must not leave
            // orphan slots or leaves behind.
            let save = (
                ctx.slots.len(),
                ctx.leaves.len(),
                ctx.memo.len(),
                ctx.rules.clone(),
            );
            if let Some(s) = map_op(*op, inputs, axis, ctx) {
                return Some(s);
            }
            // An elementwise composition the fold can't stream through that
            // is nonetheless FREE along the axis is still a legal per-element
            // input: keep the whole map as a leaf instead of failing.
            if structure(node, axis).level == Parallelism::Free {
                ctx.slots.truncate(save.0);
                ctx.leaves.truncate(save.1);
                ctx.memo.truncate(save.2);
                ctx.rules = save.3;
                let free = output_axes(node)
                    .into_iter()
                    .filter(|a| *a != axis)
                    .collect();
                return Some(S::Pe {
                    raw: Expr::Item(ctx.leaf(node, free)),
                    shift: None,
                    post: cst(1.0),
                });
            }
            None
        }

        NodeKind::Reduce { src, axis: red, op } if *red == axis => reduce_op(src, *op, axis, ctx),

        // A reduction over a different axis collapses something orthogonal;
        // if it is not FREE along our axis we are outside the supported set.
        _ => None,
    }
}

/// Reduce `src` over `axis` with monoid `op`, allocating slot(s).
fn reduce_op(src: &Node, op: BinOp, axis: Axis, ctx: &mut Ctx) -> Option<S> {
    // Index-carrying maximum: two slots — the running max of the streamed
    // values, and the index (an iota leaf) that first achieved it.
    if let BinOp::ArgMax = op {
        let s = go(src, axis, ctx)?;
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
        let iota_leaf = ctx.leaf(&crate::ir::iota(axis), Vec::new());
        let idx_slot = ctx.push_slot(
            SlotKind::ArgIdx { max_slot },
            Expr::Item(iota_leaf),
        );
        // The index ranges over whatever grid rows the max does — its `into`
        // (an iota) says nothing about that, so inherit the span.
        ctx.slots[idx_slot].span = ctx.slots[max_slot].span.clone();
        return Some(S::Coll(Expr::F(idx_slot)));
    }
    // k-best selection: one carrier holds the whole sorted (value, index)
    // k-list; each `rank` projects one slot. Ranks > 0 contribute the
    // identity per element (a singleton list is rank 0 only).
    if let BinOp::TopK { k, rank, idx } = op {
        let s = go(src, axis, ctx)?;
        let raw = plain_pe(&s)?;
        let k = k as usize;
        // One selection, many queries: if this derivation already carries
        // the k-list over the same streamed values, REUSE its slots. Eight
        // rank reduces of one score vector become one carrier with eight
        // projections, not eight list copies.
        let existing = ctx.slots.iter().position(|sl| {
            matches!(sl.kind, SlotKind::KBestVal { base, rank: 0 } if {
                ctx.slots[base..].iter().take(k).filter(|s2|
                    matches!(s2.kind, SlotKind::KBestVal { base: b2, .. } if b2 == base)
                ).count() == k
            }) && sl.into == raw
        });
        let vbase = existing.unwrap_or_else(|| {
            let vbase = ctx.slots.len();
            for r in 0..k {
                let into = if r == 0 {
                    raw.clone()
                } else {
                    cst(f64::NEG_INFINITY)
                };
                ctx.push_slot(SlotKind::KBestVal { base: vbase, rank: r }, into);
            }
            vbase
        });
        // A value-only query needs no index half; an index query needs both
        // (reusing the value half if a prior query built it).
        let ibase = if idx {
            let found = ctx.slots.iter().position(|sl| {
                matches!(sl.kind, SlotKind::KBestIdx { vbase: v, rank: 0, .. } if v == vbase)
            });
            found.unwrap_or_else(|| {
                let ibase = ctx.slots.len();
                let iota_leaf = ctx.leaf(&crate::ir::iota(axis), Vec::new());
                for r in 0..k {
                    let into = if r == 0 {
                        Expr::Item(iota_leaf)
                    } else {
                        cst(0.0)
                    };
                    ctx.push_slot(SlotKind::KBestIdx { vbase, ibase, rank: r }, into);
                }
                ibase
            })
        } else {
            usize::MAX
        };
        // Every slot of the list ranges over the same grid rows as rank 0.
        let span0 = ctx.slots[vbase].span.clone();
        for r in 0..k {
            ctx.slots[vbase + r].span = span0.clone();
            if idx {
                ctx.slots[ibase + r].span = span0.clone();
            }
        }
        ctx.rules.insert("k-best");
        let slot = if idx { ibase } else { vbase } + rank as usize;
        return Some(S::Coll(Expr::F(slot)));
    }
    let BinOp::Monoid(m) = op else {
        return None; // non-associative / affine handled elsewhere
    };
    let s = go(src, axis, ctx)?;

    match m {
        Monoid::Add => {
            // Σ over an axis the value does not vary along = n·value: the
            // invariant distributes out of the sum. The count is itself a
            // slot (into = 1) — the fold measures the extent, the carrier
            // never needs to know it. Found by the completeness probe.
            if let Some(e) = as_coll(&s) {
                ctx.rules.insert("invariant");
                let cnt = ctx.push_slot(SlotKind::Plain(Monoid::Add), cst(1.0));
                return Some(S::Coll(pmul(e, Expr::F(cnt))));
            }
            // Σ(z + c) = Σz + n·c — the offset leaves through a count slot.
            if let S::PeAdd { raw, off } = s {
                ctx.rules.insert("defer-add");
                let slot = ctx.push_slot(SlotKind::Plain(Monoid::Add), raw);
                let cnt = ctx.push_slot(SlotKind::Plain(Monoid::Add), cst(1.0));
                return Some(S::Coll(padd(
                    Expr::F(slot),
                    pmul(off, Expr::F(cnt)),
                )));
            }
            let S::Pe { raw, shift, post } = s else {
                return None; // a shifted/deferred form we cannot re-reduce
            };
            let kind = match shift {
                Some(max_slot) => {
                    ctx.rules.insert("rescale"); // rides a running max
                    SlotKind::ExpShifted { max_slot }
                }
                None => SlotKind::Plain(Monoid::Add),
            };
            let slot = ctx.push_slot(kind, raw);
            if !is1(&post) {
                // A normalizer factored out of this linear reduction; it is
                // applied once, in `project`.
                ctx.rules.insert("defer-div");
            }
            Some(S::Coll(pmul(post, Expr::F(slot))))
        }

        Monoid::LogSumExp => {
            // LSE over an invariant = value + ln n (n from a count slot).
            if let Some(e) = as_coll(&s) {
                ctx.rules.insert("invariant");
                let cnt = ctx.push_slot(SlotKind::Plain(Monoid::Add), cst(1.0));
                return Some(S::Coll(padd(e, log(Expr::F(cnt)))));
            }
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
            ctx.rules.insert("rescale");
            let sum_slot = ctx.push_slot(SlotKind::ExpShifted { max_slot }, cst(1.0));
            Some(S::Coll(padd(log(Expr::F(sum_slot)), Expr::F(max_slot))))
        }

        // max / min / product: plain monoid slots; nothing rides, nothing defers.
        Monoid::Max | Monoid::Min | Monoid::Mul => {
            // max/min over n ≥ 1 copies of an invariant is the invariant
            // (extents are ≥ 1 everywhere in this system). A product would
            // be value^n — no closed slot form; it stays declined.
            if !matches!(m, Monoid::Mul)
                && let Some(e) = as_coll(&s)
            {
                ctx.rules.insert("invariant");
                return Some(S::Coll(e));
            }
            // Lattice distributivity: reduce_m(max/min(z, c)) = max/min
            // applied AFTER reduce_m(z) — for every m, j ∈ {Max, Min}.
            if !matches!(m, Monoid::Mul)
                && let S::PeExt { raw, coll, is_max } = &s
            {
                ctx.rules.insert("lattice");
                let slot = ctx.push_slot(SlotKind::Plain(m), raw.clone());
                return Some(S::Coll(if *is_max {
                    emax(Expr::F(slot), coll.clone())
                } else {
                    emin(Expr::F(slot), coll.clone())
                }));
            }
            // max/min(z + c) = max/min(z) + c — offsets commute with order.
            if !matches!(m, Monoid::Mul)
                && let S::PeAdd { raw, off } = &s
            {
                ctx.rules.insert("defer-add");
                let slot = ctx.push_slot(SlotKind::Plain(m), raw.clone());
                return Some(S::Coll(padd(Expr::F(slot), off.clone())));
            }
            let S::Pe {
                raw,
                shift: None,
                post,
            } = s
            else {
                return None;
            };
            if is1(&post) {
                let slot = ctx.push_slot(SlotKind::Plain(m), raw);
                return Some(S::Coll(Expr::F(slot)));
            }
            // Deferred scale under an order reduction: the sign of the
            // factor decides which extremum survives — max(c·z) is c·max(z)
            // for c ≥ 0 but c·min(z) for c < 0 — so carry BOTH extrema and
            // dispatch on the sign at project time. (Mul keeps declining:
            // the factor would need an n-th power.)
            if matches!(m, Monoid::Mul) {
                return None;
            }
            ctx.rules.insert("defer-scale");
            let mx = ctx.push_slot(SlotKind::Plain(Monoid::Max), raw.clone());
            let mn = ctx.push_slot(SlotKind::Plain(Monoid::Min), raw);
            let (pos, neg) = match m {
                Monoid::Max => (mx, mn),
                Monoid::Min => (mn, mx),
                _ => unreachable!(),
            };
            Some(S::Coll(ewhere(
                elt(cst(0.0), post.clone()),
                pmul(post.clone(), Expr::F(pos)),
                pmul(post, Expr::F(neg)),
            )))
        }
    }
}

/// Combine the streamed inputs of an elementwise map. Total over the closed
/// basis (an op the fold genuinely can't stream through returns None and the
/// caller falls back to a whole-map leaf when legal).
fn map_op(op: MapOp, inputs: &[Node], axis: Axis, ctx: &mut Ctx) -> Option<S> {
    match op {
        MapOp::Add => binop(Bin::Add, &inputs[0], &inputs[1], axis, ctx),
        MapOp::Sub => binop(Bin::Sub, &inputs[0], &inputs[1], axis, ctx),
        MapOp::Mul => binop(Bin::Mul, &inputs[0], &inputs[1], axis, ctx),
        MapOp::Div => binop(Bin::Div, &inputs[0], &inputs[1], axis, ctx),
        MapOp::Max => binop(Bin::Max, &inputs[0], &inputs[1], axis, ctx),
        MapOp::Min => binop(Bin::Min, &inputs[0], &inputs[1], axis, ctx),
        MapOp::Lt => binop(Bin::Lt, &inputs[0], &inputs[1], axis, ctx),

        MapOp::Neg => unary(&inputs[0], axis, ctx, |e| sub(cst(0.0), e)),
        MapOp::Recip => unary(&inputs[0], axis, ctx, |e| pdiv(cst(1.0), e)),
        MapOp::Log => unary(&inputs[0], axis, ctx, log),
        MapOp::Sqrt => unary(&inputs[0], axis, ctx, esqrt),
        MapOp::Sin => unary(&inputs[0], axis, ctx, esin),
        MapOp::Cos => unary(&inputs[0], axis, ctx, ecos),

        // exp is where the online-softmax coupling is discovered: exp of
        // `x − m` (a per-element value minus its own running max) rides the
        // exp domain of that max slot. The element's own score is its local
        // max, so when the shifted value IS the max's contribution the unit
        // lift is exp(x − x) = 1.
        MapOp::Exp => {
            let s = go(&inputs[0], axis, ctx)?;
            if let Some(e) = as_coll(&s) {
                return Some(S::Coll(exp(e)));
            }
            match s {
                S::PeOff { raw, max_slot } => {
                    let m_into = ctx.slots[max_slot].into.clone();
                    Some(S::Pe {
                        raw: pexp_sub(raw, m_into),
                        shift: Some(max_slot),
                        post: cst(1.0),
                    })
                }
                S::Pe {
                    raw,
                    shift: None,
                    post,
                } if is1(&post) => Some(S::Pe {
                    raw: exp(raw),
                    shift: None,
                    post: cst(1.0),
                }),
                _ => None,
            }
        }

        // The fold has no closed form for tanh; the whole-map-leaf fallback
        // in `go_uncached` covers the free-along-axis case.
        MapOp::Tanh => None,

        MapOp::Where => {
            let c = go(&inputs[0], axis, ctx)?;
            let a = go(&inputs[1], axis, ctx)?;
            let b = go(&inputs[2], axis, ctx)?;
            if let (Some(c), Some(a), Some(b)) = (as_coll(&c), as_coll(&a), as_coll(&b)) {
                return Some(S::Coll(ewhere(c, a, b)));
            }
            let (c, a, b) = (plain_pe(&c)?, plain_pe(&a)?, plain_pe(&b)?);
            Some(S::Pe {
                raw: ewhere(c, a, b),
                shift: None,
                post: cst(1.0),
            })
        }
    }
}

/// A unary op that applies the same expression transform in both the
/// per-element and collapsed worlds.
fn unary(x: &Node, axis: Axis, ctx: &mut Ctx, f: impl Fn(Expr) -> Expr) -> Option<S> {
    let s = go(x, axis, ctx)?;
    if let Some(e) = as_coll(&s) {
        return Some(S::Coll(f(e)));
    }
    let raw = plain_pe(&s)?;
    Some(S::Pe {
        raw: f(raw),
        shift: None,
        post: cst(1.0),
    })
}

#[derive(Clone, Copy)]
enum Bin {
    Add,
    Sub,
    Mul,
    Div,
    Max,
    Min,
    Lt,
}

fn binop(op: Bin, x: &Node, y: &Node, axis: Axis, ctx: &mut Ctx) -> Option<S> {
    let a = go(x, axis, ctx)?;
    let b = go(y, axis, ctx)?;

    // Both collapsed (or promotable constants) → a scalar combination of
    // reduced values.
    if let (Some(p), Some(q)) = (as_coll(&a), as_coll(&b)) {
        return Some(S::Coll(match op {
            Bin::Add => padd(p, q),
            Bin::Sub => sub(p, q),
            Bin::Mul => pmul(p, q),
            Bin::Div => pdiv(p, q),
            Bin::Max => emax(p, q),
            Bin::Min => emin(p, q),
            Bin::Lt => elt(p, q),
        }));
    }

    // A collapsed value TIMES a per-element factor whose leaves never touch
    // the axis: the factor is constant along the fold — data in the role of
    // a promoted literal (a grid-axis one-hot) — so the product is collapsed
    // too, the factor evaluated at PROJECT time, where the streamed axis is
    // already gone. This is what lets the eight rank queries of one k-best
    // selection become a single fold whose projection is rank-indexed by
    // the grid coordinate (`ir::topk_all`).
    //
    // Deliberately NARROW on two counts. Mul only: an additive version
    // would swallow residual/bias epilogues into the projection. And only
    // when the collapsed side reads ORDER-SENSITIVE slots (k-best, argmax):
    // a leaf-reading projection declines the cooperative schedule, so the
    // absorption is free exactly when the carrier already declines it —
    // an attention gate over a rescale carrier must stay an epilogue, or
    // the flash fold falls back to one thread per output.
    if matches!(op, Bin::Mul) {
        let order_sensitive = |e: &Expr| {
            slots_of(e).iter().any(|&i| {
                matches!(
                    ctx.slots[i].kind,
                    SlotKind::KBestVal { .. } | SlotKind::KBestIdx { .. } | SlotKind::ArgIdx { .. }
                )
            })
        };
        let inv = |s: &S| plain_pe(s).filter(|raw| invariant_along(raw, axis, ctx));
        let pq = match (&a, &b) {
            (S::Coll(p), s) if order_sensitive(p) => inv(s).map(|q| (p.clone(), q)),
            (s, S::Coll(p)) if order_sensitive(p) => inv(s).map(|q| (p.clone(), q)),
            _ => None,
        };
        if let Some((p, q)) = pq {
            ctx.rules.insert("project-leaf");
            return Some(S::Coll(pmul(p, q)));
        }
    }

    match (op, a, b) {
        // Per-element minus the collapsed running max over the same axis:
        // the online-softmax shift intermediate, consumed by Exp.
        (
            Bin::Sub,
            S::Pe {
                raw,
                shift: None,
                post,
            },
            S::Coll(Expr::F(i)),
        ) if is1(&post) && matches!(ctx.slots[i].kind, SlotKind::Plain(Monoid::Max)) => {
            Some(S::PeOff { raw, max_slot: i })
        }

        // Per-element × / ÷ a value GENUINELY collapsed over the same axis
        // (a slot expression — a promoted constant must NOT take this path:
        // deferring a constant scale past a max/exp coupling would change
        // the math). By distributivity the collapsed factor is constant
        // along the axis, so it is deferred: pushed into `post` and applied
        // once after the downstream reduction. This is where `defer-div`
        // comes from.
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

        // Per-element max/min/± against a value GENUINELY collapsed over
        // the same axis (a slot expression — as with the deferred factor, a
        // promoted constant must NOT take these paths: constants combine
        // into the per-element value directly and stay composable). The
        // coupling collapses later, at the reduction that consumes it.
        (Bin::Max, a, b) | (Bin::Min, a, b)
            if matches!(op, Bin::Max | Bin::Min)
                && (plain_pe(&a).is_some() && matches!(b, S::Coll(_))
                    || matches!(a, S::Coll(_)) && plain_pe(&b).is_some()) =>
        {
            let (raw, coll) = match (plain_pe(&a), &b) {
                (Some(r), S::Coll(c)) => (r, c.clone()),
                _ => {
                    let S::Coll(c) = a else { unreachable!() };
                    (plain_pe(&b).unwrap(), c)
                }
            };
            Some(S::PeExt {
                raw,
                coll,
                is_max: matches!(op, Bin::Max),
            })
        }
        (Bin::Add, a, b)
            if plain_pe(&a).is_some() && matches!(b, S::Coll(_))
                || matches!(a, S::Coll(_)) && plain_pe(&b).is_some() =>
        {
            let (raw, off) = match (plain_pe(&a), &b) {
                (Some(r), S::Coll(c)) => (r, c.clone()),
                _ => {
                    let S::Coll(c) = a else { unreachable!() };
                    (plain_pe(&b).unwrap(), c)
                }
            };
            Some(S::PeAdd { raw, off })
        }
        (Bin::Sub, a, b) if plain_pe(&a).is_some() && matches!(b, S::Coll(_)) => {
            let S::Coll(c) = b else { unreachable!() };
            Some(S::PeAdd {
                raw: plain_pe(&a).unwrap(),
                off: sub(cst(0.0), c),
            })
        }
        (Bin::Sub, a, b) if matches!(a, S::Coll(_)) && plain_pe(&b).is_some() => {
            let S::Coll(c) = a else { unreachable!() };
            Some(S::PeAdd {
                raw: sub(cst(0.0), plain_pe(&b).unwrap()),
                off: c,
            })
        }

        // Two per-element values. Multiplication merges exp-shift domains and
        // deferred factors; the other ops require plain values.
        (
            Bin::Mul,
            S::Pe {
                raw: r1,
                shift: s1,
                post: p1,
            },
            S::Pe {
                raw: r2,
                shift: s2,
                post: p2,
            },
        ) => Some(S::Pe {
            raw: pmul(r1, r2),
            shift: merge_shift(s1, s2)?,
            post: pmul(p1, p2),
        }),
        (op, a, b) => {
            let (r1, r2) = (plain_pe(&a)?, plain_pe(&b)?);
            let raw = match op {
                Bin::Add => padd(r1, r2),
                Bin::Sub => sub(r1, r2),
                Bin::Div => pdiv(r1, r2),
                Bin::Max => emax(r1, r2),
                Bin::Min => emin(r1, r2),
                Bin::Lt => elt(r1, r2),
                Bin::Mul => unreachable!("handled above"),
            };
            Some(S::Pe {
                raw,
                shift: None,
                post: cst(1.0),
            })
        }
    }
}

fn merge_shift(a: Option<usize>, b: Option<usize>) -> Option<Option<usize>> {
    match (a, b) {
        (None, x) | (x, None) => Some(x),
        (Some(i), Some(j)) if i == j => Some(Some(i)),
        _ => None, // two distinct exp domains — unsupported
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
            SlotKind::ArgIdx { max_slot: mx } => {
                // first max wins: switch to B only on a STRICT improvement
                ewhere(elt(Expr::A(mx), Expr::B(mx)), Expr::B(i), Expr::A(i))
            }
            // Insert the incoming element (B's rank-0 pair) into the sorted
            // list: strict `<` everywhere, so an equal LATER element never
            // displaces an earlier one — first-max-wins across all ranks.
            SlotKind::KBestVal { base, rank } => {
                let b = Expr::B(base);
                if rank == 0 {
                    ewhere(elt(Expr::A(base), b.clone()), b, Expr::A(base))
                } else {
                    // Displaced at rank r: the new value is the old rank r−1
                    // (shifted down) if the element sits above it, else the
                    // element itself lands exactly here.
                    ewhere(
                        elt(Expr::A(base + rank), b.clone()),
                        ewhere(
                            elt(Expr::A(base + rank - 1), b.clone()),
                            Expr::A(base + rank - 1),
                            b,
                        ),
                        Expr::A(base + rank),
                    )
                }
            }
            SlotKind::KBestIdx { vbase, ibase, rank } => {
                let bv = Expr::B(vbase);
                let bi = Expr::B(ibase);
                if rank == 0 {
                    ewhere(elt(Expr::A(vbase), bv), bi, Expr::A(ibase))
                } else {
                    ewhere(
                        elt(Expr::A(vbase + rank), bv.clone()),
                        ewhere(elt(Expr::A(vbase + rank - 1), bv), Expr::A(ibase + rank - 1), bi),
                        Expr::A(ibase + rank),
                    )
                }
            }
            SlotKind::AffineStep => unreachable!("AffineStep slots are built directly"),
        })
        .collect();
    let identity = slots
        .iter()
        .map(|s| match s.kind {
            SlotKind::Plain(m) => m.identity(),
            SlotKind::ExpShifted { .. } => 0.0,
            SlotKind::ArgIdx { .. } => 0.0,
            SlotKind::KBestVal { .. } => f64::NEG_INFINITY,
            SlotKind::KBestIdx { .. } => 0.0,
            SlotKind::AffineStep => unreachable!("AffineStep slots are built directly"),
        })
        .collect();
    (into, combine, identity)
}

/// The SSM / linear-recurrence carrier: the affine map `(A, b)` under
/// composition. `combine(L, R) = R ∘ L`, `identity` = the identity map,
/// `project` applies the composite to `h₀ = 0` (returns `b`).
fn affine_scan_carrier() -> Carrier {
    Carrier {
        slots: 2,
        leaves: Vec::new(), // the special-cased carrier has no derived leaves
        into: vec![Expr::Item(0), Expr::Item(1)], // (A_t, b_t)
        // (A', b') for R∘L with L first:  x ↦ A_R(A_L x + b_L) + b_R
        combine: vec![
            Expr::Mul(Box::new(Expr::B(0)), Box::new(Expr::A(0))),
            padd(
                Expr::Mul(Box::new(Expr::B(0)), Box::new(Expr::A(1))),
                Expr::B(1),
            ),
        ],
        identity: vec![1.0, 0.0],
        project: vec![Expr::F(1)],
        spans: vec![vec![], vec![]], // scalar affine state, no free axes
        rules: vec!["affine"],
        kinds: vec![SlotKind::AffineStep, SlotKind::AffineStep],
    }
}
