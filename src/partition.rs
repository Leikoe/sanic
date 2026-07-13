//! Partitioning: split a whole graph into kernels, fusing as much as the
//! algebra allows.
//!
//! The rule that makes this simple: **the derive frontier is the fusion
//! boundary**. Everything one `derive` call swallows is, by construction, one
//! legal streaming kernel — the fold, its couplings, its deferred
//! normalizers, and every elementwise map along the way. The carrier's
//! `leaves` are exactly where the derivation stopped composing, so that is
//! exactly where a kernel boundary can go. No pattern library, no fusion
//! heuristics over op names: cut where the algebra stops, then let the cost
//! model rank what remains.
//!
//! Per node, in order:
//!
//! 1. **Fold** — some axis derives here: this node is one streaming kernel.
//!    Each carrier leaf is either a free source (a raw input, a literal, an
//!    index), the score contraction of an online-softmax coupling (computed
//!    in-body, like FlashAttention's QKᵀ), or a producer to cut: it becomes a
//!    named intermediate and is partitioned recursively.
//! 2. **Elementwise** — no axis folds and the node is a map: collect the
//!    maximal elementwise cone into one map kernel. If the cone has exactly
//!    one non-input producer of the same shape, don't even do that — attach
//!    the cone to the producer kernel as an epilogue (a residual add rides
//!    its GEMM for free).
//! 3. **Gather / Sequential** — indexed loads and non-associative scans get
//!    their own stage; their operands are partitioned recursively.
//!
//! Stages come out in execution order (producers first).

use std::collections::HashMap;
use std::rc::Rc;

use crate::analyze::{Parallelism, structure};
use crate::cost::Device;
use crate::derive::{Carrier, SlotKind, derive, items_of};
use crate::interp::{Env, Extents, Tensor, eval, run_carrier};
use crate::ir::{Axis, BinOp, MapOp, Monoid, Node, NodeKind, all_axes, input, leaf_names, output_axes};
use crate::plan::{KernelSpec, plan_axis};

/// One kernel in the schedule.
pub enum Stage {
    /// A derived streaming fold, planned for the device. `epilogue` names
    /// elementwise ops fused onto the output (with `epilogue_inputs` as the
    /// extra tensors they read).
    Fused {
        spec: Box<KernelSpec>,
        epilogue: Vec<&'static str>,
        epilogue_inputs: Vec<&'static str>,
        /// The cut fold graph this kernel streams — its leaves are reads of
        /// materialized intermediates (`input("tN", …)`) or free sources. This
        /// is what the interpreter drives to *execute* the derived kernel.
        fold_node: Node,
        /// When an epilogue was fused on, the elementwise node that turns the
        /// fold's output (read as `input(output_name, …)`) plus the epilogue
        /// inputs into the final result. `None` when the fold output is final.
        epilogue_node: Option<Node>,
    },
    /// A maximal elementwise cone — one pass over the output grid, no fold.
    Elementwise {
        ops: Vec<&'static str>,
        inputs: Vec<&'static str>,
        output: String,
        /// The cone as an executable graph, with cut producers spliced to
        /// reads of their buffers.
        exec: Node,
    },
    /// A data-dependent indexed load (embedding lookup et al.).
    Gather {
        axis: Axis,
        inputs: Vec<&'static str>,
        output: String,
        exec: Node,
    },
    /// A non-associative recurrence: strictly serial along `axis`.
    Sequential {
        op: &'static str,
        axis: Axis,
        inputs: Vec<&'static str>,
        output: String,
        exec: Node,
    },
    /// An axis derives here but no block structure fits the device — a real
    /// finding, reported instead of guessed around.
    Infeasible { axis: Axis, output: String },
}

/// A whole-graph schedule: kernels in execution order.
pub struct Schedule {
    pub stages: Vec<Stage>,
    /// The buffer name each partition root landed under, in root order.
    /// [`Schedule::execute`] returns the last; a stateful runtime
    /// ([`crate::runtime`]) reads them all.
    pub outputs: Vec<String>,
}

/// Split `node` into a schedule of kernels for `dev`.
pub fn partition(node: &Node, dev: &Device, extents: &HashMap<Axis, f64>) -> Schedule {
    partition_many(&[(node.clone(), "Out")], dev, extents)
}

/// Split several roots into ONE schedule with shared producers cut once — a
/// decode step's cache updates and its logits reuse the same projections
/// instead of recomputing them per output. Roots are emitted in order, and a
/// root reachable from a *later* root is reused through its materialization
/// (so put producers before consumers). Each root lands under its given name.
pub fn partition_many(
    roots: &[(Node, &'static str)],
    dev: &Device,
    extents: &HashMap<Axis, f64>,
) -> Schedule {
    let mut parents = HashMap::new();
    for (r, _) in roots {
        count_parents(r, &mut parents);
    }
    let mut p = Partitioner {
        dev,
        extents,
        stages: Vec::new(),
        fresh: 0,
        done: HashMap::new(),
        parents,
    };
    let mut outputs = Vec::new();
    for (r, name) in roots {
        let landed = p.emit(r, name);
        // a later root (or leaf) reaching this one reuses the buffer
        p.done.insert(Rc::as_ptr(r), landed);
        outputs.push(landed.to_string());
    }
    Schedule {
        stages: p.stages,
        outputs,
    }
}

struct Partitioner<'a> {
    dev: &'a Device,
    extents: &'a HashMap<Axis, f64>,
    stages: Vec<Stage>,
    fresh: usize,
    /// Nodes already materialized, by pointer → the name they live under.
    /// A DAG-shared producer (a residual, say) is cut once, not per consumer.
    done: HashMap<*const NodeKind, &'static str>,
    /// How many consumers each node has in the original graph. A node with
    /// more than one is a fusion barrier for elementwise cones: computing it
    /// inside one consumer would recompute or corrupt it for the others.
    parents: HashMap<*const NodeKind, usize>,
}

/// Count graph edges into each node (a DAG walk; every edge counts).
fn count_parents(node: &Node, out: &mut HashMap<*const NodeKind, usize>) {
    let visit = |child: &Node, out: &mut HashMap<*const NodeKind, usize>| {
        let n = out.entry(Rc::as_ptr(child)).or_insert(0);
        *n += 1;
        if *n == 1 {
            count_parents(child, out); // recurse only on first visit
        }
    };
    match node.as_ref() {
        NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => {}
        NodeKind::Map { inputs, .. } => {
            for i in inputs {
                visit(i, out);
            }
        }
        NodeKind::Reduce { src, .. }
        | NodeKind::Scan { src, .. }
        | NodeKind::View { src, .. }
        | NodeKind::Reindex { src, .. } => visit(src, out),
        NodeKind::Gather { src, index, .. } => {
            visit(src, out);
            visit(index, out);
        }
    }
}

impl Partitioner<'_> {
    /// A name for a materialized intermediate. Leaked: `Input` names are
    /// `&'static str`, and a compiler session leaks a few bytes per cut.
    fn fresh(&mut self) -> &'static str {
        let name = format!("t{}", self.fresh);
        self.fresh += 1;
        Box::leak(name.into_boxed_str())
    }

    /// Emit the stages that produce `node` under the name `out`.
    /// The name a consumer should read is returned (`out`, or the source's
    /// own name if no stage was needed).
    fn emit(&mut self, node: &Node, out: &str) -> &'static str {
        match node.as_ref() {
            NodeKind::Input { name, .. } => return name, // already materialized
            NodeKind::Const { v } => return leak(&format!("{v}")),
            NodeKind::Iota { axis } => return leak(&format!("iota({})", axis.label())),
            _ => {}
        }

        // 1) Fold: some axis derives at this node → one streaming kernel.
        if let Some((axis, carrier)) = self.best_fold(node) {
            return self.emit_fold(node, axis, &carrier, out);
        }

        match node.as_ref() {
            // 2) Elementwise cone.
            NodeKind::Map { .. } => self.emit_cone(node, out),

            // A view or affine reindex is a relabeling, not a computation: no
            // stage, no copy. Consumers read the source's materialization
            // under the new indexing.
            NodeKind::View { src, .. } | NodeKind::Reindex { src, .. } => {
                let name = self.cut(src);
                self.done.insert(Rc::as_ptr(node), name);
                name
            }

            // 3) Gather: an indexed load.
            NodeKind::Gather { src, index, axis } => {
                let s = self.cut(src);
                let i = self.cut(index);
                let exec = self.executable(node);
                self.stages.push(Stage::Gather {
                    axis: *axis,
                    inputs: vec![s, i],
                    output: out.to_string(),
                    exec,
                });
                leak(out)
            }

            // 3) Sequential: a non-associative recurrence.
            NodeKind::Scan { src, axis, op } => {
                let name = match op {
                    BinOp::NonAssoc(n) => n,
                    _ => "scan",
                };
                let s = self.cut(src);
                let exec = self.executable(node);
                self.stages.push(Stage::Sequential {
                    op: name,
                    axis: *axis,
                    inputs: vec![s],
                    output: out.to_string(),
                    exec,
                });
                leak(out)
            }

            // A reduce that didn't derive: something in its source entangles
            // the axis in a way no fold can stream through (a norm's own
            // same-axis reduction, a reused contraction axis, …). Don't
            // materialize the source wholesale — that would write out the
            // pre-contraction tensor. Cut exactly the sub-expressions that
            // entangle the axis, and retry: the remainder usually folds.
            NodeKind::Reduce { src, axis, op } => {
                let mut cuts: Vec<Node> = Vec::new();
                self.entanglers(src, *axis, &mut cuts);
                if cuts.is_empty() {
                    // Nothing to free — the fold itself is unsupported.
                    let s = self.cut(src);
                    let exec = self.executable(node);
                    self.stages.push(Stage::Sequential {
                        op: "reduce (no legal fold)",
                        axis: *axis,
                        inputs: vec![s],
                        output: out.to_string(),
                        exec,
                    });
                    return leak(out);
                }
                let subs: Vec<(Node, Node)> = cuts
                    .iter()
                    .map(|c| {
                        self.cut(c);
                        // splice, not input(name, output_axes): a cut below a
                        // view must be read under its stored axes
                        (c.clone(), self.splice(c, false))
                    })
                    .collect();
                let rebuilt =
                    crate::ir::reduce(replace_many(src, &subs, &mut HashMap::new()), *axis, *op);
                self.emit(&rebuilt, out)
            }

            NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => {
                unreachable!("handled above")
            }
        }
    }

    /// Materialize `node` as its own kernel(s); return the intermediate name.
    /// A node cut once stays cut — later consumers reuse the same tensor.
    fn cut(&mut self, node: &Node) -> &'static str {
        if is_free_source(node) {
            return self.emit(node, "");
        }
        if let Some(name) = self.done.get(&Rc::as_ptr(node)) {
            return name;
        }
        let t = self.fresh();
        // A view aliases its source, so the name things actually landed
        // under is emit's return value, not necessarily `t`.
        let name = self.emit(node, t);
        self.done.insert(Rc::as_ptr(node), name);
        name
    }

    /// Is this node consumed by more than one parent in the original graph?
    fn shared(&self, node: &Node) -> bool {
        self.parents.get(&Rc::as_ptr(node)).copied().unwrap_or(1) > 1
    }

    /// Build an executable version of `node`: the same computation, but every
    /// already-materialized producer beneath it replaced by a read of its
    /// buffer (`input(name, axes)`). Views stay transparent — a rename of a
    /// materialized tensor becomes a rename of the *read*, which is exactly
    /// the "one buffer, two index spaces" aliasing (a normalized tensor read
    /// at query and key positions). The root op is always kept: it is what
    /// this stage computes, not a boundary. The interpreter runs the result.
    fn executable(&self, node: &Node) -> Node {
        self.splice(node, true)
    }

    fn splice(&self, node: &Node, is_root: bool) -> Node {
        if !is_root {
            if is_free_source(node) {
                return node.clone(); // read a raw input / const / index directly
            }
            if let NodeKind::View { src, groups } = node.as_ref() {
                return crate::ir::view(self.splice(src, false), groups.clone());
            }
            if let NodeKind::Reindex { src, map, padded } = node.as_ref() {
                return crate::ir::reindex(self.splice(src, false), map.clone(), *padded);
            }
            if let Some(name) = self.done.get(&Rc::as_ptr(node)) {
                return input(name, &output_axes(node)); // a materialized buffer read
            }
        }
        match node.as_ref() {
            NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => node.clone(),
            NodeKind::Map { op, inputs } => {
                crate::ir::map(*op, inputs.iter().map(|i| self.splice(i, false)).collect())
            }
            NodeKind::Reduce { src, axis, op } => {
                crate::ir::reduce(self.splice(src, false), *axis, *op)
            }
            NodeKind::Scan { src, axis, op } => {
                crate::ir::scan(self.splice(src, false), *axis, *op)
            }
            NodeKind::Gather { src, index, axis } => {
                crate::ir::gather(self.splice(src, false), self.splice(index, false), *axis)
            }
            NodeKind::View { src, groups } => {
                crate::ir::view(self.splice(src, false), groups.clone())
            }
            NodeKind::Reindex { src, map, padded } => {
                crate::ir::reindex(self.splice(src, false), map.clone(), *padded)
            }
        }
    }

    /// The subtrees of a fold LEAF that must be materialized: anything
    /// carrying a fold of its own (a producer GEMM), anything DAG-shared
    /// (materialize once, not per consumer), and anything already
    /// materialized (read its buffer). Everything else — elementwise
    /// arithmetic, views, reindexes, gathers — is per-element work the
    /// kernel computes in-body. The [`Partitioner::entanglers`] idea,
    /// applied to leaves: cut as deep as possible, keep the arithmetic.
    fn leaf_cuts(&self, node: &Node, out: &mut Vec<Node>) {
        let push = |node: &Node, out: &mut Vec<Node>| {
            if !out.iter().any(|n| Rc::ptr_eq(n, node)) {
                out.push(node.clone());
            }
        };
        if self.done.contains_key(&Rc::as_ptr(node)) {
            push(node, out); // splices to its buffer read
            return;
        }
        // In-body leaf arithmetic runs once per (grid × stream) point, where
        // a materialized leaf is computed once per its own volume. Cheap ops
        // (a dequant multiply, a mask) win inline; transcendental chains (a
        // GELU, a rotary table) lose — recomputed per stream step they cost
        // more than the spill they save.
        let cheap = |op: MapOp| {
            !matches!(
                op,
                MapOp::Exp | MapOp::Log | MapOp::Sqrt | MapOp::Tanh | MapOp::Sin | MapOp::Cos
            )
        };
        match node.as_ref() {
            NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => {}
            NodeKind::Map { op, inputs } if !self.shared(node) && cheap(*op) => {
                for i in inputs {
                    self.leaf_cuts(i, out);
                }
            }
            NodeKind::Gather { src, index, .. } if !self.shared(node) => {
                self.leaf_cuts(src, out);
                self.leaf_cuts(index, out);
            }
            NodeKind::View { src, .. } | NodeKind::Reindex { src, .. } => {
                self.leaf_cuts(src, out)
            }
            _ => push(node, out),
        }
    }

    /// The sub-expressions of `node` that entangle `axis` — not FREE along
    /// it, so no fold can stream through them (a norm's own same-axis
    /// reduction, an upstream projection over a reused contraction axis).
    /// Descends through private, unmaterialized maps AND through the
    /// structural operators (views, reindexes, gathers — pure index
    /// arithmetic that must stay in the kernel) to place the cut as deep as
    /// possible; anything shared, already materialized, or fold-bearing is
    /// cut whole so other consumers can reuse it.
    fn entanglers(&self, node: &Node, axis: Axis, out: &mut Vec<Node>) {
        if structure(node, axis).level == Parallelism::Free {
            return;
        }
        let private = !self.shared(node) && !self.done.contains_key(&Rc::as_ptr(node));
        match node.as_ref() {
            NodeKind::Map { inputs, .. } if private => {
                for i in inputs {
                    self.entanglers(i, axis, out);
                }
                return;
            }
            // The structural operators alias, so descending costs nothing —
            // but the AXIS TRANSLATES at the boundary, exactly as it does in
            // `structure`: below a flatten the entanglement lives on the
            // group members; below a split/window, on the mapped source
            // axis. Asking about the outer axis below the boundary would
            // find nothing and miss the cut.
            NodeKind::View { src, groups } => {
                if let Some((members, _)) = groups.iter().find(|(_, to)| *to == axis) {
                    for m in members {
                        self.entanglers(src, *m, out);
                    }
                } else if !groups.iter().any(|(members, _)| members.contains(&axis)) {
                    self.entanglers(src, axis, out);
                } // else: consumed below the view — nothing entangles above
                return;
            }
            NodeKind::Reindex { src, map: rmap, .. } => {
                let driving: Vec<Axis> = rmap
                    .iter()
                    .filter(|(_, terms, _)| terms.iter().any(|(_, a)| *a == axis))
                    .map(|(m, _, _)| *m)
                    .collect();
                if !driving.is_empty() {
                    for m in driving {
                        self.entanglers(src, m, out);
                    }
                } else if !rmap.iter().any(|(m, _, _)| *m == axis) {
                    self.entanglers(src, axis, out);
                } // else: consumed below the reindex
                return;
            }
            NodeKind::Gather { src, index, axis: g } if private && *g != axis => {
                self.entanglers(src, axis, out);
                self.entanglers(index, axis, out);
                return;
            }
            _ => {}
        }
        if !out.iter().any(|n| Rc::ptr_eq(n, node)) {
            out.push(node.clone());
        }
    }

    /// The cheapest axis that derives at this node, if any.
    fn best_fold(&self, node: &Node) -> Option<(Axis, Carrier)> {
        let mut best: Option<(Axis, Carrier, f64)> = None;
        for axis in all_axes(node) {
            if structure(node, axis).level != Parallelism::Monoidal {
                continue;
            }
            let Some(c) = derive(node, axis) else {
                continue;
            };
            // Rank by planned cost on the uncut graph; an unplannable axis
            // ranks last but is still a legal fold.
            let cost =
                plan_axis(node, axis, &c, self.dev, self.extents).map_or(f64::INFINITY, |s| s.cost);
            if best.as_ref().is_none_or(|(_, _, b)| cost < *b) {
                best = Some((axis, c, cost));
            }
        }
        best.map(|(a, c, _)| (a, c))
    }

    /// One streaming kernel at `node` over `axis`: cut the carrier leaves the
    /// kernel cannot compute in-body, re-plan on the cut graph, push a stage.
    fn emit_fold(&mut self, node: &Node, axis: Axis, carrier: &Carrier, out: &str) -> &'static str {
        // The score contraction of an online-softmax coupling is computed
        // in-body (FlashAttention's QKᵀ): the leaves the coupled max reads.
        let in_body: Vec<usize> = carrier
            .kinds
            .iter()
            .enumerate()
            .filter(|(i, k)| {
                matches!(k, SlotKind::Plain(Monoid::Max))
                    && carrier
                        .kinds
                        .iter()
                        .any(|k2| matches!(k2, SlotKind::ExpShifted { max_slot } if max_slot == i))
            })
            .flat_map(|(i, _)| items_of(&carrier.into[i]))
            .collect();

        // Collect every cut first, then substitute in ONE rebuild pass — the
        // targets are pointers into the original graph, and any rebuild
        // invalidates them for a second pass.
        let mut subs: Vec<(Node, Node)> = Vec::new();
        let cut_into = |p: &mut Self, node: &Node, subs: &mut Vec<(Node, Node)>| {
            let mut cuts = Vec::new();
            p.leaf_cuts(node, &mut cuts);
            for c in cuts {
                p.cut(&c);
                // splice, not `input(name, output_axes)`: a view/reindex above
                // the cut must keep its reshape so the buffer is read under
                // the axes it was stored with.
                subs.push((c.clone(), p.splice(&c, false)));
            }
        };
        for (idx, leaf) in carrier.leaves.iter().enumerate() {
            if in_body.contains(&idx) && is_matmul(leaf) {
                // Fuse the contraction in-body; cut its operands WHOLE. An
                // in-body operand is re-read on every step of the streamed
                // axis, so arithmetic left inline would be recomputed per
                // step (a RoPE'd query's transcendentals × the whole key
                // axis); materialized once, it is read like any tile.
                let NodeKind::Reduce { src, .. } = leaf.as_ref() else {
                    unreachable!()
                };
                let NodeKind::Map { inputs, .. } = src.as_ref() else {
                    unreachable!()
                };
                for operand in inputs.clone() {
                    if !is_free_source(&operand) {
                        self.cut(&operand);
                        subs.push((operand.clone(), self.splice(&operand, false)));
                    }
                }
            } else {
                // Cut exactly the fold-bearing / shared / already-materialized
                // subtrees of the leaf; the per-element arithmetic around them
                // (dequantization, gathers of expert weights, splits) stays
                // in-body instead of spilling to memory.
                cut_into(self, leaf, &mut subs);
            }
        }
        let cut_graph = replace_many(node, &subs, &mut HashMap::new());

        // Re-derive and plan on the graph the kernel will actually see.
        let Some(c2) = derive(&cut_graph, axis) else {
            self.stages.push(Stage::Infeasible {
                axis,
                output: out.to_string(),
            });
            return leak(out);
        };
        match plan_axis(&cut_graph, axis, &c2, self.dev, self.extents) {
            Some(mut spec) => {
                spec.output_name = out.to_string();
                self.stages.push(Stage::Fused {
                    spec: Box::new(spec),
                    epilogue: Vec::new(),
                    epilogue_inputs: Vec::new(),
                    fold_node: cut_graph.clone(),
                    epilogue_node: None,
                });
            }
            None => self.stages.push(Stage::Infeasible {
                axis,
                output: out.to_string(),
            }),
        }
        leak(out)
    }

    /// A maximal elementwise cone — bounded below by non-map nodes and by
    /// DAG-shared nodes (computing a shared value inside one consumer would
    /// duplicate it for the others). If the cone has exactly one unshared
    /// producer of the same shape, don't even emit a map kernel: attach the
    /// cone to that producer as an epilogue.
    fn emit_cone(&mut self, node: &Node, out: &str) -> &'static str {
        let mut ops = Vec::new();
        let mut frontier: Vec<Node> = Vec::new();
        self.cone(node, &mut ops, &mut frontier, true);

        let complex: Vec<Node> = frontier
            .iter()
            .filter(|n| !is_free_source(n))
            .cloned()
            .collect();
        let plain_inputs: Vec<&'static str> = frontier
            .iter()
            .filter_map(|n| match n.as_ref() {
                NodeKind::Input { name, .. } => Some(*name),
                _ => None,
            })
            .collect();

        // Epilogue fusion: exactly one producer, unshared, not yet
        // materialized, same output shape → ride its kernel for free.
        if let [producer] = complex.as_slice()
            && !self.shared(producer)
            && !self.done.contains_key(&Rc::as_ptr(producer))
            && output_axes(producer) == output_axes(node)
        {
            let t = self.fresh();
            let before = self.stages.len();
            let landed = self.emit(producer, t);
            if self.stages.len() > before
                && let Some(Stage::Fused {
                    spec,
                    epilogue,
                    epilogue_inputs,
                    epilogue_node,
                    ..
                }) = self.stages.last_mut()
                && spec.output_name == landed
                && epilogue.is_empty()
            {
                // The fold now writes the final output name; the epilogue reads
                // that same buffer (`input(out)`) and its extra plain inputs,
                // producing the final result in place.
                let epi = replace_many(
                    node,
                    &[(producer.clone(), input(leak(out), &output_axes(producer)))],
                    &mut HashMap::new(),
                );
                spec.output_name = out.to_string();
                *epilogue = ops;
                *epilogue_inputs = plain_inputs;
                *epilogue_node = Some(epi);
                return leak(out);
            }
            // The producer didn't land as a fused kernel — keep the map stage,
            // reading the producer's materialized buffer.
            self.done.insert(Rc::as_ptr(producer), landed);
            let exec = self.executable(node);
            let mut inputs = vec![landed];
            inputs.extend(plain_inputs);
            self.stages.push(Stage::Elementwise {
                ops,
                inputs,
                output: out.to_string(),
                exec,
            });
            return leak(out);
        }

        let mut inputs: Vec<&'static str> = Vec::new();
        for n in &frontier {
            inputs.push(self.cut(n));
        }
        let exec = self.executable(node);
        self.stages.push(Stage::Elementwise {
            ops,
            inputs,
            output: out.to_string(),
            exec,
        });
        leak(out)
    }

    /// Collect the elementwise cone rooted at `node`: op names, and the
    /// frontier of non-map, shared, or already-materialized nodes beneath it
    /// (deduped by pointer). A materialized node — an earlier root of a
    /// multi-output partition, say — is a frontier read, not something to
    /// recompute inline.
    fn cone(&self, node: &Node, ops: &mut Vec<&'static str>, frontier: &mut Vec<Node>, top: bool) {
        match node.as_ref() {
            NodeKind::Map { op, inputs }
                if top || (!self.shared(node) && !self.done.contains_key(&Rc::as_ptr(node))) =>
            {
                if !ops.contains(&op.name()) {
                    ops.push(op.name());
                }
                for i in inputs {
                    self.cone(i, ops, frontier, false);
                }
            }
            // Literals and iotas are ambient — not inputs, not producers.
            NodeKind::Const { .. } | NodeKind::Iota { .. } => {}
            _ => {
                if !frontier.iter().any(|n| Rc::ptr_eq(n, node)) {
                    frontier.push(node.clone());
                }
            }
        }
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn leak(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

/// Free to read in any kernel: a raw input (possibly behind views or affine
/// reindexes — both are just index arithmetic on the load), a literal, or an
/// index value — never something to materialize.
fn is_free_source(node: &Node) -> bool {
    match node.as_ref() {
        NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => true,
        NodeKind::View { src, .. } | NodeKind::Reindex { src, .. } => is_free_source(src),
        _ => false,
    }
}

/// `Reduce(Map(×, _, _), _, Add)` — a contraction the emitters can compute
/// in-body as an MMA.
fn is_matmul(node: &Node) -> bool {
    matches!(node.as_ref(),
        NodeKind::Reduce { src, op: BinOp::Monoid(Monoid::Add), .. }
            if matches!(src.as_ref(),
                NodeKind::Map { op, inputs } if *op == MapOp::Mul && inputs.len() == 2))
}

/// Rebuild `node` with every `(target, replacement)` applied, in one pass.
/// Memoized by pointer so a DAG-shared sub-expression is rebuilt once and
/// stays shared — the deriver dedups leaves by pointer, so losing sharing
/// would split slots.
fn replace_many(
    node: &Node,
    subs: &[(Node, Node)],
    memo: &mut HashMap<*const NodeKind, Node>,
) -> Node {
    if let Some((_, with)) = subs.iter().find(|(t, _)| Rc::ptr_eq(t, node)) {
        return with.clone();
    }
    let key = Rc::as_ptr(node);
    if let Some(done) = memo.get(&key) {
        return done.clone();
    }
    let rebuilt = match node.as_ref() {
        NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => node.clone(),
        NodeKind::Map { op, inputs } => crate::ir::map(
            *op,
            inputs.iter().map(|i| replace_many(i, subs, memo)).collect(),
        ),
        NodeKind::Reduce { src, axis, op } => {
            crate::ir::reduce(replace_many(src, subs, memo), *axis, *op)
        }
        NodeKind::Scan { src, axis, op } => {
            crate::ir::scan(replace_many(src, subs, memo), *axis, *op)
        }
        NodeKind::Gather { src, index, axis } => crate::ir::gather(
            replace_many(src, subs, memo),
            replace_many(index, subs, memo),
            *axis,
        ),
        NodeKind::View { src, groups } => {
            crate::ir::view(replace_many(src, subs, memo), groups.clone())
        }
        NodeKind::Reindex { src, map, padded } => {
            crate::ir::reindex(replace_many(src, subs, memo), map.clone(), *padded)
        }
    };
    memo.insert(key, rebuilt.clone());
    rebuilt
}

// ── rendering ────────────────────────────────────────────────────────────────

impl Schedule {
    pub fn kernel_count(&self) -> usize {
        self.stages.len()
    }

    /// Sum of the planned costs of the fused stages. Elementwise / gather /
    /// sequential stages are not costed (they are bandwidth-bound copies at
    /// worst; a real model would price them too).
    pub fn fused_cost(&self) -> f64 {
        self.stages
            .iter()
            .map(|s| match s {
                Stage::Fused { spec, .. } => spec.cost,
                _ => 0.0,
            })
            .sum()
    }

    /// Execute the whole schedule on real input tensors; return the final
    /// output — the compiler's end-to-end result.
    ///
    /// Each stage runs the way its kind dictates: a fused stage *streams its
    /// derived carrier* ([`run_carrier`]) then applies any fused epilogue; an
    /// elementwise / gather / monoidal-scan stage evaluates its spliced
    /// sub-graph ([`eval`]) over the intermediates produced so far.
    /// Intermediates live in a growing environment keyed by the same names
    /// [`Schedule::render`] prints, so a stage reads exactly the buffers its
    /// predecessors wrote.
    ///
    /// The guarantee: running the partitioned schedule agrees with
    /// [`eval`] of the *original* graph. That is the compiler-correctness
    /// theorem — derivation, cuts, and dataflow all preserve the naive
    /// semantics — reduced to a numeric equality a test can check.
    ///
    /// `extents` are concrete integer sizes (distinct from the `f64` extents
    /// the planner prices with); execution runs at real shapes.
    pub fn execute(&self, inputs: &Env, extents: &Extents) -> Tensor {
        let mut env: Env = inputs.clone();
        self.execute_env(&mut env, extents);
        let name = self
            .outputs
            .last()
            .expect("cannot execute a schedule with no outputs");
        env.remove(name.as_str())
            .expect("schedule produced no tensor under its output name")
    }

    /// Every buffer name the schedule reads without producing — its true
    /// inputs. The stateful runtime uses this for its write-after-read check:
    /// a schedule output must never share a name with a buffer the schedule
    /// reads, or a later stage could observe the new value as the old one.
    pub fn reads(&self) -> Vec<&'static str> {
        let mut produced: Vec<String> = Vec::new();
        let mut out: Vec<&'static str> = Vec::new();
        let note = |node: &Node, produced: &[String], out: &mut Vec<&'static str>| {
            for n in leaf_names(node) {
                if !produced.iter().any(|p| p == n) && !out.contains(&n) {
                    out.push(n);
                }
            }
        };
        for stage in &self.stages {
            match stage {
                Stage::Fused {
                    spec,
                    fold_node,
                    epilogue_node,
                    ..
                } => {
                    note(fold_node, &produced, &mut out);
                    produced.push(spec.output_name.clone());
                    if let Some(epi) = epilogue_node {
                        note(epi, &produced, &mut out);
                    }
                }
                Stage::Elementwise { output, exec, .. }
                | Stage::Gather { output, exec, .. }
                | Stage::Sequential { output, exec, .. } => {
                    note(exec, &produced, &mut out);
                    produced.push(output.clone());
                }
                Stage::Infeasible { .. } => {}
            }
        }
        out
    }

    /// Run every stage into `env` — inputs, intermediates and all outputs end
    /// up as named buffers. This is the runtime building block: a stateful
    /// session ([`crate::runtime::Session`]) executes into its persistent
    /// environment and then commits outputs over existing buffers.
    pub fn execute_env(&self, env: &mut Env, extents: &Extents) {
        for stage in &self.stages {
            let (name, tensor): (&'static str, Tensor) = match stage {
                Stage::Fused {
                    spec,
                    fold_node,
                    epilogue_node,
                    ..
                } => {
                    let name = leak(&spec.output_name);
                    let folded =
                        run_carrier(fold_node, spec.streaming_axis, &spec.carrier, env, extents);
                    let result = match epilogue_node {
                        None => folded,
                        Some(epi) => {
                            // expose the fold output so the epilogue can read it
                            env.insert(name, folded);
                            eval(epi, env, extents)
                        }
                    };
                    (name, result)
                }
                Stage::Elementwise { output, exec, .. }
                | Stage::Gather { output, exec, .. }
                | Stage::Sequential { output, exec, .. } => (leak(output), eval(exec, env, extents)),
                Stage::Infeasible { output, .. } => {
                    panic!("cannot execute an infeasible stage producing `{output}`")
                }
            };
            env.insert(name, tensor);
        }
    }

    /// One line per kernel, in execution order.
    pub fn render(&self) -> String {
        let mut out = format!("schedule — {} kernels\n", self.stages.len());
        for (i, s) in self.stages.iter().enumerate() {
            let line = match s {
                Stage::Fused {
                    spec,
                    epilogue,
                    epilogue_inputs,
                    ..
                } => {
                    let epi = if epilogue.is_empty() {
                        String::new()
                    } else {
                        format!(
                            "  + epilogue {}({})",
                            epilogue.join("·"),
                            epilogue_inputs.join(", ")
                        )
                    };
                    let mut block = String::new();
                    if let Some(r) = spec.row_axis {
                        block += &format!("row {}\u{d7}{}", r.label(), spec.tile_m);
                    }
                    if let Some(c) = spec.col_tile_axis {
                        block += &format!(" col {}\u{d7}{}", c.label(), spec.tile_c);
                    }
                    if !spec.batch_axes.is_empty() {
                        let labels: Vec<&str> = spec.batch_axes.iter().map(|a| a.label()).collect();
                        block += &format!(" grid {{{}}}", labels.join(","));
                    }
                    if block.is_empty() {
                        block = "scalar".to_string();
                    }
                    format!(
                        "{:<4} = fold `{}`({})  [{} slots: {}]  {}{}",
                        spec.output_name,
                        spec.streaming_axis.label(),
                        spec.input_names.join(", "),
                        spec.carrier.slots,
                        spec.carrier.rules.join(", "),
                        block.trim_start(),
                        epi,
                    )
                }
                Stage::Elementwise {
                    ops,
                    inputs,
                    output,
                    ..
                } => format!(
                    "{:<4} = map {}({})",
                    output,
                    ops.join("·"),
                    inputs.join(", ")
                ),
                Stage::Gather {
                    axis,
                    inputs,
                    output,
                    ..
                } => format!(
                    "{:<4} = gather over `{}`({})   [OPAQUE — indexed load]",
                    output,
                    axis.label(),
                    inputs.join(", ")
                ),
                Stage::Sequential {
                    op,
                    axis,
                    inputs,
                    output,
                    ..
                } => format!(
                    "{:<4} = scan `{}` over `{}`({})   [SEQUENTIAL — serial]",
                    output,
                    op,
                    axis.label(),
                    inputs.join(", ")
                ),
                Stage::Infeasible { axis, output } => format!(
                    "{output:<4} = fold `{}` — DERIVES BUT NO BLOCK FITS THE DEVICE",
                    axis.label()
                ),
            };
            out += &format!("  [{i:>2}] {line}\n");
        }
        out
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::*;

    fn ext(pairs: &[(Axis, f64)]) -> HashMap<Axis, f64> {
        pairs.iter().copied().collect()
    }

    fn add_r() -> BinOp {
        BinOp::Monoid(Monoid::Add)
    }

    // Plain attention over raw tensors: nothing to cut → exactly one kernel.
    #[test]
    fn plain_attention_is_one_kernel() {
        let (s, k, d, e) = (axis("s"), axis("k"), axis("d"), axis("e"));
        let attn = attention(
            input("Q", &[s, d]),
            input("K", &[k, d]),
            input("V", &[k, e]),
            d,
            k,
        );
        let sched = partition(
            &attn,
            &Device::toy(),
            &ext(&[(s, 1024.0), (k, 1024.0), (d, 64.0), (e, 64.0)]),
        );
        assert_eq!(sched.stages.len(), 1);
        let Stage::Fused { spec, .. } = &sched.stages[0] else {
            panic!("expected a fused stage")
        };
        assert_eq!(spec.streaming_axis, k);
        assert_eq!(spec.carrier.slots, 3);
    }

    // Attention over *projections*: the QKV GEMMs are cut into producer
    // kernels; the flash fold streams over their outputs; the score
    // contraction stays in-body.
    #[test]
    fn projected_attention_cuts_the_gemms() {
        let (s, k, dm, dq, dv) = (axis("s"), axis("k"), axis("dm"), axis("dq"), axis("dv"));
        let x_q = input("Xq", &[s, dm]);
        let x_kv = input("Xkv", &[k, dm]);
        let q = matmul(x_q, input("Wq", &[dq, dm]), dm); // [s, dq]
        let kk = matmul(x_kv.clone(), input("Wk", &[dq, dm]), dm); // [k, dq]
        let v = matmul(x_kv, input("Wv", &[dv, dm]), dm); // [k, dv]

        let scores = matmul(q, kk, dq);
        let out = matmul(softmax(scores, k), v, k);

        let sched = partition(
            &out,
            &Device::toy(),
            &ext(&[
                (s, 1024.0),
                (k, 1024.0),
                (dm, 512.0),
                (dq, 64.0),
                (dv, 64.0),
            ]),
        );

        // 3 GEMM producers + 1 flash kernel, producers first.
        assert_eq!(sched.stages.len(), 4);
        for st in &sched.stages[..3] {
            let Stage::Fused { spec, .. } = st else {
                panic!("producers are fused folds")
            };
            assert_eq!(spec.streaming_axis, dm);
            assert_eq!(spec.carrier.slots, 1);
        }
        let Stage::Fused { spec, .. } = &sched.stages[3] else {
            panic!()
        };
        assert_eq!(spec.streaming_axis, k);
        assert_eq!(spec.carrier.slots, 3, "flash fold survives the cuts");
        // its inputs are the materialized intermediates
        assert!(
            spec.input_names
                .iter()
                .filter(|n| n.starts_with('t'))
                .count()
                >= 3
        );
    }

    // RMSNorm with *runtime* scalar inputs splits into a sum-of-squares fold
    // plus an elementwise kernel — the per-element output has no one-pass
    // fold. (With literal constants instead, the norm fuses into a consuming
    // GEMM — see `rmsnorm_fuses_into_a_projection_gemm`.)
    #[test]
    fn rmsnorm_splits_into_fold_plus_map() {
        let (s, d) = (axis("s"), axis("d"));
        let x = input("X", &[s, d]);
        let g = input("G", &[d]);
        let inv_d = input("inv_d", &[]);
        let eps = input("eps", &[]);
        let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), d, add_r());
        let mean = map(MapOp::Mul, vec![ss, inv_d]);
        let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, eps])]);
        let norm = map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom]);

        let sched = partition(&norm, &Device::toy(), &ext(&[(s, 1024.0), (d, 1024.0)]));
        assert_eq!(sched.stages.len(), 2);
        assert!(matches!(&sched.stages[0], Stage::Fused { spec, .. }
            if spec.streaming_axis == d && spec.carrier.slots == 1));
        assert!(matches!(&sched.stages[1], Stage::Elementwise { ops, .. }
            if ops.contains(&"sqrt") && ops.contains(&"div")));
    }

    // With literal constants (ε, 1/n), the whole norm fuses INTO the
    // projection GEMM: one kernel, two slots (the dot product and Σx²), the
    // normalizer deferred to the projection — an RMSNorm-fused GEMM, derived.
    #[test]
    fn rmsnorm_fuses_into_a_projection_gemm() {
        let (s, d, f) = (axis("s"), axis("d"), axis("f"));
        let x = input("X", &[s, d]);
        let g = input("G", &[d]);
        let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), d, add_r());
        let mean = map(MapOp::Mul, vec![ss, konst(1.0 / 1024.0)]);
        let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
        let norm = map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom]);
        let proj = matmul(norm, input("W", &[f, d]), d);

        let sched = partition(
            &proj,
            &Device::toy(),
            &ext(&[(s, 1024.0), (d, 1024.0), (f, 512.0)]),
        );
        assert_eq!(sched.stages.len(), 1, "norm + GEMM = one kernel");
        let Stage::Fused { spec, .. } = &sched.stages[0] else {
            panic!()
        };
        assert_eq!(spec.carrier.slots, 2, "dot product + Σx²");
        assert!(spec.carrier.rules.contains(&"defer-div"));
    }

    // A residual add rides its producer GEMM as an epilogue — no extra kernel.
    #[test]
    fn residual_add_fuses_as_epilogue() {
        let (s, f, dm) = (axis("s"), axis("f"), axis("dm"));
        let x = input("X", &[s, dm]);
        let h = input("H", &[s, f]);
        let w = input("W", &[f, dm]);
        let proj = matmul(h, w, f); // [s, dm]
        let y = map(MapOp::Add, vec![proj, x]); // residual

        let sched = partition(
            &y,
            &Device::toy(),
            &ext(&[(s, 1024.0), (f, 4096.0), (dm, 1024.0)]),
        );
        assert_eq!(sched.stages.len(), 1, "the add must not be its own kernel");
        let Stage::Fused {
            spec,
            epilogue,
            epilogue_inputs,
            ..
        } = &sched.stages[0]
        else {
            panic!()
        };
        assert_eq!(spec.streaming_axis, f);
        assert_eq!(epilogue, &vec!["add"]);
        assert_eq!(epilogue_inputs, &vec!["X"]);
    }

    // SwiGLU down-projection: gate and up GEMMs are cut, but the silu and the
    // gating multiply fuse into the down GEMM's lift (activation-fused GEMM).
    // `silu` is a composition of basis ops, not a special form.
    #[test]
    fn silu_fuses_into_the_down_gemm() {
        let (s, dm, f) = (axis("s"), axis("dm"), axis("f"));
        let x = input("Xn", &[s, dm]);
        let gate = matmul(x.clone(), input("Wg", &[f, dm]), dm); // [s, f]
        let up = matmul(x, input("Wu", &[f, dm]), dm); // [s, f]
        let act = map(MapOp::Mul, vec![silu(gate), up]);
        let down = reduce(
            map(MapOp::Mul, vec![act, input("Wd", &[f, dm])]),
            f,
            add_r(),
        );

        let sched = partition(
            &down,
            &Device::toy(),
            &ext(&[(s, 1024.0), (dm, 1024.0), (f, 4096.0)]),
        );
        assert_eq!(sched.stages.len(), 3, "gate GEMM, up GEMM, fused down GEMM");
        let Stage::Fused { spec, .. } = &sched.stages[2] else {
            panic!()
        };
        assert_eq!(spec.streaming_axis, f);
        assert!(
            spec.carrier.rules.contains(&"fused-map"),
            "silu·gate·up fused into the lift: {:?}",
            spec.carrier.rules
        );
    }

    // An embedding lookup is its own OPAQUE gather stage.
    #[test]
    fn embedding_is_a_gather_stage() {
        let (v, dm, s) = (axis("v"), axis("dm"), axis("s"));
        let emb = embedding(input("E", &[v, dm]), input("ids", &[s]), v);
        let sched = partition(
            &emb,
            &Device::toy(),
            &ext(&[(v, 32000.0), (dm, 1024.0), (s, 1024.0)]),
        );
        assert_eq!(sched.stages.len(), 1);
        assert!(matches!(&sched.stages[0], Stage::Gather { axis, .. } if *axis == v));
    }

    // A rename view shares one materialization: the key/value side of
    // attention reads the SAME normalized tensor the query side computed —
    // one norm in the schedule, zero copies.
    #[test]
    fn a_view_shares_one_norm_across_q_and_kv() {
        let (s, t, dm, dq, dv) = (axis("s"), axis("t"), axis("dm"), axis("dq"), axis("dv"));
        let x = input("X", &[s, dm]);
        let g = input("g", &[dm]);
        let inv = input("inv_dm", &[]);
        let eps = input("eps", &[]);
        let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), dm, add_r());
        let mean = map(MapOp::Mul, vec![ss, inv]);
        let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, eps])]);
        let xn = map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom]);
        let xn_t = rename(xn.clone(), s, t); // the key/value view

        let q = matmul(xn, input("Wq", &[dq, dm]), dm); // [s, dq]
        let k = matmul(xn_t.clone(), input("Wk", &[dq, dm]), dm); // [t, dq]
        let v = matmul(xn_t, input("Wv", &[dv, dm]), dm); // [t, dv]
        let attn = matmul(softmax(matmul(q, k, dq), t), v, t);

        let sched = partition(
            &attn,
            &Device::toy(),
            &ext(&[
                (s, 1024.0),
                (t, 1024.0),
                (dm, 512.0),
                (dq, 64.0),
                (dv, 64.0),
            ]),
        );

        // Σx² fold + norm map + Q/K/V GEMMs + flash — the norm appears ONCE.
        assert_eq!(sched.stages.len(), 6);
        let norm_maps = sched
            .stages
            .iter()
            .filter(|s| matches!(s, Stage::Elementwise { .. }))
            .count();
        assert_eq!(norm_maps, 1, "one norm, shared through the view");
    }

    // A flatten view makes the multi-head output projection expressible:
    // attention leaves [s, h, dv]; the projection streams the flattened
    // (h, dv) axis. Two kernels, no copy in between.
    #[test]
    fn flatten_enables_the_multihead_output_projection() {
        let (h, s, t, dk, dv, dmv, dm) = (
            axis("h"),
            axis("s"),
            axis("t"),
            axis("dk"),
            axis("dv"),
            axis("dmv"),
            axis("dm"),
        );
        let attn = attention(
            input("Q", &[h, s, dk]),
            input("K", &[h, t, dk]),
            input("V", &[h, t, dv]),
            dk,
            t,
        );
        let flat = flatten(attn, &[h, dv], dmv); // [s, dmv]
        let o = matmul(flat, input("Wo", &[dmv, dm]), dmv); // [s, dm]

        let sched = partition(
            &o,
            &Device::toy(),
            &ext(&[
                (h, 8.0),
                (s, 1024.0),
                (t, 1024.0),
                (dk, 64.0),
                (dv, 64.0),
                (dmv, 512.0),
                (dm, 512.0),
            ]),
        );

        assert_eq!(sched.stages.len(), 2, "flash kernel + projection GEMM");
        let Stage::Fused { spec, .. } = &sched.stages[0] else {
            panic!()
        };
        assert_eq!(spec.streaming_axis, t);
        assert_eq!(spec.carrier.slots, 3, "the multi-head flash fold");
        let Stage::Fused { spec, .. } = &sched.stages[1] else {
            panic!()
        };
        assert_eq!(spec.streaming_axis, dmv, "streams the flattened axis");
    }

    // A COMPUTED causal mask (iota + compare + where) fuses into the flash
    // lift: one kernel, no mask tensor, no mask traffic.
    #[test]
    fn computed_causal_mask_fuses_into_flash() {
        let (s, t, dk, dv) = (axis("s"), axis("t"), axis("dk"), axis("dv"));
        let scores = matmul(input("Q", &[s, dk]), input("K", &[t, dk]), dk);
        let scaled = map(MapOp::Mul, vec![scores, konst(0.125)]);
        let masked = map(MapOp::Add, vec![scaled, causal_mask(s, t)]);
        let out = matmul(softmax(masked, t), input("V", &[t, dv]), t);

        let sched = partition(
            &out,
            &Device::toy(),
            &ext(&[(s, 1024.0), (t, 1024.0), (dk, 64.0), (dv, 64.0)]),
        );
        assert_eq!(sched.stages.len(), 1, "mask and scale ride the lift");
        let Stage::Fused { spec, .. } = &sched.stages[0] else {
            panic!()
        };
        assert_eq!(spec.carrier.slots, 3);
        assert!(spec.carrier.rules.contains(&"fused-map"));
        assert_eq!(
            spec.input_names,
            vec!["Q", "K", "V"],
            "no mask tensor is read"
        );
    }
}
