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

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::analyze::{Parallelism, structure};
use crate::cost::DeviceProfile;
use crate::derive::{Carrier, SlotKind, derive, items_of};
use crate::interp::{Env, Value, eval, run_carrier};
use crate::kernel_ir::{
    Axis, BinOp, Canonicalizer, Dtype, MapOp, Monoid, Node as NodeKind, NodeRef as Node, all_axes,
    input, leaf_names,
};
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
        /// materialized intermediates (`input("tN", …, dtype)`) or free
        /// sources. This is what the interpreter drives to *execute* the
        /// derived kernel.
        fold_node: Node,
        /// When an epilogue was fused on, the elementwise node that turns the
        /// fold's output (read as `input(epi_fold_read, …, dtype)`) plus the
        /// epilogue inputs into the final result. `None` when the fold output
        /// is final.
        epilogue_node: Option<Node>,
        /// The name the epilogue reads the fold's OWN output under. Normally the
        /// output name, but for an in-place update (`w = w − lr·∇w`, output
        /// named `w`) the epilogue also reads the weight `w`, so the fold output
        /// gets the fold's distinct temp name instead — otherwise the two reads
        /// alias and `w` is replaced by `∇w`.
        epi_fold_read: &'static str,
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
    /// An axis derives here but the cut graph declines or no block structure
    /// fits the device — a real finding, reported with its reason instead of
    /// guessed around.
    Infeasible {
        axis: Axis,
        output: String,
        why: String,
    },
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
pub fn partition(node: &Node, dev: &DeviceProfile) -> Schedule {
    partition_many(&[(node.clone(), "Out")], dev)
}

/// Split several roots into ONE schedule with shared producers cut once — a
/// decode step's cache updates and its logits reuse the same projections
/// instead of recomputing them per output. Roots are emitted in order, and a
/// root reachable from a *later* root is reused through its materialization
/// (so put producers before consumers). Each root lands under its given name.
pub fn partition_many(roots: &[(Node, &'static str)], dev: &DeviceProfile) -> Schedule {
    // Canonicalize FIRST: everything below reads sharing through
    // pointer-keyed maps (`parents`, `done`), so two separately built but
    // structurally identical subgraphs must already be one node when the
    // counting starts — otherwise the same value is derived, and possibly
    // computed, once per copy. The partitioner KEEPS the canonical table:
    // every node it constructs while emitting (spliced reads, rebuilt cut
    // graphs) goes back through it, so a rebuilt structural twin resolves to
    // the original node and an already-cut value is read, not re-emitted.
    let mut canon = Canonicalizer::default();
    let graph_roots: Vec<Node> = roots.iter().map(|(root, _)| canon.tree(root)).collect();
    crate::verify::assert_valid_many(&graph_roots);
    let roots: Vec<(Node, &'static str)> = graph_roots
        .into_iter()
        .zip(roots.iter().map(|(_, name)| *name))
        .collect();
    let mut parents = HashMap::new();
    for (r, _) in &roots {
        count_parents(r, &mut parents);
    }
    let mut p = Partitioner {
        dev,
        stages: Vec::new(),
        fresh: 0,
        done: HashMap::new(),
        keepalive: Vec::new(),
        parents,
        canon,
    };
    let mut outputs = Vec::new();
    for (r, name) in &roots {
        let landed = p.emit(r, name);
        // a later root (or leaf) reaching this one reuses the buffer
        p.done.insert(Rc::as_ptr(r), landed);
        outputs.push(landed.to_string());
    }
    // In-place updates: a root named after a graph input (`w = w − lr·∇w`)
    // writes that weight's own buffer. Order every reader of the weight before
    // its writer so the new value never overwrites the old mid-step — no shadow
    // buffer, half the weight/optimizer VRAM. A no-op unless a name aliases.
    let graph_inputs: HashSet<String> = roots
        .iter()
        .flat_map(|(r, _)| leaf_names(r))
        .map(|s| s.to_string())
        .collect();
    let stages = order_in_place(p.stages, &graph_inputs);
    let sched = Schedule { stages, outputs };
    // SANIC_DEBUG >= 1: dump the schedule and each kernel's fusion, like
    // tinygrad's DEBUG — the compilation made inspectable.
    if crate::debug_level() >= 1 {
        sched.debug_dump();
    }
    sched
}

pub(crate) fn stage_output(s: &Stage) -> &str {
    match s {
        Stage::Fused { spec, .. } => &spec.output_name,
        Stage::Elementwise { output, .. }
        | Stage::Gather { output, .. }
        | Stage::Sequential { output, .. }
        | Stage::Infeasible { output, .. } => output,
    }
}

/// The buffer names a Fused stage reads: its fold leaves plus any epilogue
/// leaves, deduped, with the epilogue's read of the fold's OWN output
/// (`epi_fold_read`) dropped — that is ordinary epilogue fusion, not an
/// external read. The fold's output NAME is kept, so a caller that needs to spot
/// a genuine in-place self-read (`w = f(w, …)`) still sees it.
fn fused_leaf_reads(
    fold_node: &Node,
    epilogue_node: &Option<Node>,
    epi_fold_read: &'static str,
) -> Vec<&'static str> {
    let mut reads = leaf_names(fold_node);
    if let Some(epi) = epilogue_node {
        for n in leaf_names(epi) {
            if !reads.contains(&n) {
                reads.push(n);
            }
        }
    }
    reads.retain(|n| *n != epi_fold_read);
    reads
}

/// Every buffer name a single stage reads (its fold/cone leaves, minus its own
/// fused-output read).
fn stage_reads(s: &Stage) -> Vec<&'static str> {
    match s {
        Stage::Fused {
            spec,
            fold_node,
            epilogue_node,
            epi_fold_read,
            ..
        } => {
            let mut reads = fused_leaf_reads(fold_node, epilogue_node, epi_fold_read);
            reads.retain(|n| *n != spec.output_name.as_str()); // its own output is not an external read
            reads
        }
        Stage::Elementwise { inputs, .. }
        | Stage::Gather { inputs, .. }
        | Stage::Sequential { inputs, .. } => inputs.clone(),
        Stage::Infeasible { .. } => Vec::new(),
    }
}

/// Every name a stage's cone reads, keeping a GENUINE read of its own output
/// name (the signature of an in-place update `w = f(w, …)`, which [`stage_reads`]
/// strips) but dropping the epilogue's read of the fold's OWN output under
/// `epi_fold_read` — that is ordinary epilogue fusion, not a self-read. This is
/// what separates a real in-place update (reads the weight under a name ≠
/// `epi_fold_read`) from a materialized cut (`xd{l+1} = block(xd{l})`), whose
/// only read of its output name IS the fold-output read.
fn stage_reads_self(s: &Stage) -> Vec<&'static str> {
    match s {
        Stage::Fused {
            fold_node,
            epilogue_node,
            epi_fold_read,
            ..
        } => fused_leaf_reads(fold_node, epilogue_node, epi_fold_read),
        Stage::Elementwise { inputs, .. }
        | Stage::Gather { inputs, .. }
        | Stage::Sequential { inputs, .. } => inputs.clone(),
        Stage::Infeasible { .. } => Vec::new(),
    }
}

/// Stable topological reorder enforcing the in-place write-after-read rule:
/// when a stage's output names a `graph_input` (a weight updated in place),
/// every stage reading that buffer is ordered before the writer. Data
/// dependencies (producer before consumer) are honored throughout, and an
/// in-place writer is NOT treated as the producer of that name — readers take
/// the pre-update value from the external input, not the writer's output.
/// Panics on a write-after-read cycle (mutually-recursive in-place updates).
fn order_in_place(stages: Vec<Stage>, graph_inputs: &HashSet<String>) -> Vec<Stage> {
    let n = stages.len();
    let out: Vec<String> = stages.iter().map(|s| stage_output(s).to_string()).collect();
    let reads: Vec<Vec<&'static str>> = stages.iter().map(stage_reads).collect();
    // A stage is an in-place update only if its output names a graph input AND
    // its cone reads that same name (`w = f(w, …)`). A materialized cut whose
    // output shares a graph-input name but does NOT read it (`xd{l+1} =
    // block(xd{l})`, the decode residual stream) is a genuine producer — dropping
    // its producer→reader edge would let a block read the stale cut buffer.
    let inplace: Vec<bool> = (0..n)
        .map(|i| {
            graph_inputs.contains(&out[i])
                && stage_reads_self(&stages[i]).iter().any(|r| *r == out[i])
        })
        .collect();
    // producers of genuine intermediates only (an in-place writer's readers
    // read the external weight, so it must NOT create a producer→reader edge)
    let mut producer: HashMap<String, usize> = HashMap::new();
    for i in 0..n {
        if !inplace[i] {
            producer.insert(out[i].clone(), i);
        }
    }
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut indeg = vec![0usize; n];
    let edge = |a: usize, b: usize, adj: &mut Vec<Vec<usize>>, indeg: &mut Vec<usize>| {
        if a != b && !adj[a].contains(&b) {
            adj[a].push(b);
            indeg[b] += 1;
        }
    };
    for i in 0..n {
        for r in &reads[i] {
            if let Some(&j) = producer.get(*r) {
                edge(j, i, &mut adj, &mut indeg);
            }
        }
    }
    for i in 0..n {
        if !inplace[i] {
            continue;
        }
        let o = out[i].as_str();
        for k in 0..n {
            if reads[k].iter().any(|r| *r == o) {
                edge(k, i, &mut adj, &mut indeg);
            }
        }
    }
    // stable Kahn: smallest original index first, so an already-topological
    // input (no in-place aliasing) comes back byte-for-byte unchanged.
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut heap: BinaryHeap<Reverse<usize>> =
        (0..n).filter(|&i| indeg[i] == 0).map(Reverse).collect();
    let mut order = Vec::with_capacity(n);
    while let Some(Reverse(i)) = heap.pop() {
        order.push(i);
        for j in adj[i].clone() {
            indeg[j] -= 1;
            if indeg[j] == 0 {
                heap.push(Reverse(j));
            }
        }
    }
    assert_eq!(
        order.len(),
        n,
        "in-place update created a write-after-read cycle: two weights' updates each read \
         the other, so neither can run last — materialize one gradient to a temp to break it"
    );
    let mut slots: Vec<Option<Stage>> = stages.into_iter().map(Some).collect();
    order
        .into_iter()
        .map(|i| slots[i].take().unwrap())
        .collect()
}

struct Partitioner<'a> {
    dev: &'a DeviceProfile,
    stages: Vec<Stage>,
    fresh: usize,
    /// Nodes already materialized, by pointer → the name they live under.
    /// A DAG-shared producer (a residual, say) is cut once, not per consumer.
    done: HashMap<*const NodeKind, &'static str>,
    /// Keeps every `done`-keyed node alive for the whole partition. `done` is
    /// keyed by raw pointer, and partition rebuilds transient graphs
    /// (`replace_many` in a Reduce/backward cut); once such a node drops, the
    /// allocator can hand its address to a NEW node, and the stale `done` entry
    /// would then answer for the wrong node (a materialized read under totally
    /// unrelated axes). Holding an `Rc` pins the address, so no reuse.
    keepalive: Vec<Node>,
    /// How many consumers each node has in the original graph. A node with
    /// more than one is a fusion barrier for elementwise cones: computing it
    /// inside one consumer would recompute or corrupt it for the others.
    parents: HashMap<*const NodeKind, usize>,
    /// The canonical table the roots were canonicalized with at entry. Every
    /// node the partitioner constructs afterwards (spliced reads, rebuilt
    /// cut graphs) goes back through it, so a rebuilt structural twin IS the
    /// original node and `done` keeps deduplicating across consumers.
    canon: Canonicalizer,
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
            NodeKind::Iota { axis } => return leak(&format!("iota({})", axis.name)),
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
                self.keepalive.push(node.clone());
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
                        // splice, not input(name, output_axes, dtype): a cut below a
                        // view must be read under its stored axes
                        (c.clone(), self.splice(c, false))
                    })
                    .collect();
                let spliced_src = replace_many(src, &subs, &mut HashMap::new(), &mut self.canon);
                let rebuilt = self
                    .canon
                    .shallow(crate::kernel_ir::reduce(spliced_src, *axis, *op));
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
        self.keepalive.push(node.clone());
        name
    }

    /// Is this node consumed by more than one parent in the original graph?
    fn shared(&self, node: &Node) -> bool {
        self.parents.get(&Rc::as_ptr(node)).copied().unwrap_or(1) > 1
    }

    /// Build an executable version of `node`: the same computation, but every
    /// already-materialized producer beneath it replaced by a read of its
    /// buffer (`input(name, axes, dtype)`). Views stay transparent — a rename of a
    /// materialized tensor becomes a rename of the *read*, which is exactly
    /// the "one buffer, two index spaces" aliasing (a normalized tensor read
    /// at query and key positions). The root op is always kept: it is what
    /// this stage computes, not a boundary. The interpreter runs the result.
    fn executable(&mut self, node: &Node) -> Node {
        self.splice(node, true)
    }

    fn splice(&mut self, node: &Node, is_root: bool) -> Node {
        self.splice_memo(node, is_root, &mut HashMap::new())
    }

    /// Memoized by pointer, and a subtree with nothing spliced beneath
    /// returns the ORIGINAL `Rc`: DAG sharing survives the rebuild (the
    /// deriver dedups leaves by pointer, and `done` is keyed by pointer, so
    /// a node another consumer will cut must keep its identity), and deep
    /// shared chains splice in linear time.
    fn splice_memo(
        &mut self,
        node: &Node,
        is_root: bool,
        memo: &mut HashMap<*const NodeKind, Node>,
    ) -> Node {
        if !is_root {
            if is_free_source(node) {
                return node.clone(); // read a raw input / const / index directly
            }
            if let Some(m) = memo.get(&Rc::as_ptr(node)) {
                return m.clone();
            }
            if let NodeKind::View { src, groups } = node.as_ref() {
                let s = self.splice_memo(src, false, memo);
                let out = if Rc::ptr_eq(&s, src) {
                    node.clone()
                } else {
                    self.canon.shallow(crate::kernel_ir::view(s, groups.clone()))
                };
                memo.insert(Rc::as_ptr(node), out.clone());
                return out;
            }
            if let NodeKind::Reindex { src, map, padded } = node.as_ref() {
                let s = self.splice_memo(src, false, memo);
                let out = if Rc::ptr_eq(&s, src) {
                    node.clone()
                } else {
                    self.canon
                        .shallow(crate::kernel_ir::reindex(s, map.clone(), *padded))
                };
                memo.insert(Rc::as_ptr(node), out.clone());
                return out;
            }
            if let Some(&name) = self.done.get(&Rc::as_ptr(node)) {
                // a materialized buffer read
                let read = self.canon.shallow(input(name, &node.shape(), Dtype::F32));
                memo.insert(Rc::as_ptr(node), read.clone());
                return read;
            }
        }
        let out = match node.as_ref() {
            NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => node.clone(),
            NodeKind::Map { op, inputs } => {
                let new: Vec<Node> = inputs
                    .iter()
                    .map(|i| self.splice_memo(i, false, memo))
                    .collect();
                if new.iter().zip(inputs).all(|(a, b)| Rc::ptr_eq(a, b)) {
                    node.clone()
                } else {
                    self.canon.shallow(crate::kernel_ir::map(*op, new))
                }
            }
            NodeKind::Reduce { src, axis, op } => {
                let s = self.splice_memo(src, false, memo);
                if Rc::ptr_eq(&s, src) {
                    node.clone()
                } else {
                    self.canon.shallow(crate::kernel_ir::reduce(s, *axis, *op))
                }
            }
            NodeKind::Scan { src, axis, op } => {
                let s = self.splice_memo(src, false, memo);
                if Rc::ptr_eq(&s, src) {
                    node.clone()
                } else {
                    self.canon.shallow(crate::kernel_ir::scan(s, *axis, *op))
                }
            }
            NodeKind::Gather { src, index, axis } => {
                let s = self.splice_memo(src, false, memo);
                let i = self.splice_memo(index, false, memo);
                if Rc::ptr_eq(&s, src) && Rc::ptr_eq(&i, index) {
                    node.clone()
                } else {
                    self.canon.shallow(crate::kernel_ir::gather(s, i, *axis))
                }
            }
            NodeKind::View { src, groups } => {
                let s = self.splice_memo(src, false, memo);
                if Rc::ptr_eq(&s, src) {
                    node.clone()
                } else {
                    self.canon.shallow(crate::kernel_ir::view(s, groups.clone()))
                }
            }
            NodeKind::Reindex { src, map, padded } => {
                let s = self.splice_memo(src, false, memo);
                if Rc::ptr_eq(&s, src) {
                    node.clone()
                } else {
                    self.canon
                        .shallow(crate::kernel_ir::reindex(s, map.clone(), *padded))
                }
            }
        };
        if !is_root {
            memo.insert(Rc::as_ptr(node), out.clone());
        }
        out
    }

    /// The subtrees of a fold LEAF (streamed over `axes`) that must be
    /// materialized: anything carrying a fold of its own (a producer GEMM),
    /// stream-varying transcendental chains, and anything already
    /// materialized (read its buffer). Everything else — elementwise
    /// arithmetic, views, reindexes, gathers, stream-INVARIANT
    /// transcendentals — is per-element work the kernel computes in-body.
    /// The [`Partitioner::entanglers`] idea, applied to leaves: cut as deep
    /// as possible, keep the arithmetic.
    ///
    /// `axes` is the streamed axis IN THE CURRENT FRAME: exactly as in
    /// `entanglers`, the axis translates at every structural boundary (below
    /// a flatten the stream lives on the group members; below a split, on
    /// the mapped source axis; below a gather whose index varies with the
    /// stream, on the gathered axis). Without the translation everything
    /// under a flattened fold looks stream-invariant, and a SwiGLU's exp
    /// stays in-body of the down projection — recomputed once per output
    /// row instead of evaluated once per element.
    fn leaf_cuts(&self, node: &Node, axes: &[Axis], out: &mut Vec<Node>) {
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
        // (a dequant multiply, a mask, a residual add) win inline — even
        // DAG-SHARED ones: recomputing an add per consumer is nothing next
        // to a kernel launch plus a memory round trip, so sharing is a
        // reason to cut only when the work is real. A transcendental map
        // inlines exactly when its subtree does not vary along the streamed
        // axis: hoisted out of the stream loop it costs one evaluation per
        // grid point, the same as a buffer read (a norm's rsqrt of a row
        // scalar); varying along the stream it is recomputed every step (a
        // GELU inside a GEMM) and materializes instead.
        //
        // And when something below WILL be cut, cut at the TOP of the
        // enclosing elementwise cone rather than around the offender — as
        // long as that doesn't materialize more elements. Cutting a SwiGLU's
        // exp alone leaves gate·recip(1+·)·up in-body: the fold then loads
        // gate, up AND the exp per streamed element, which costs more than
        // the exp did. Cutting the whole activation cone leaves ONE load.
        // The launch is already paid; the volume bound keeps the lift from
        // ever writing a broadcast product.
        match node.as_ref() {
            NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => {}
            NodeKind::Map { inputs, op }
                if cheap_op(*op) || {
                    let na = all_axes(node);
                    !axes.iter().any(|a| na.contains(a))
                } =>
            {
                if let Some(hot) = self.hot_volume(node, axes)
                    && self.volume(node) <= hot
                {
                    push(node, out);
                    return;
                }
                for i in inputs {
                    self.leaf_cuts(i, axes, out);
                }
            }
            NodeKind::Gather {
                src,
                index,
                axis: g,
            } => {
                self.leaf_cuts(src, &stream_below_gather(axes, index, *g), out);
                self.leaf_cuts(index, axes, out);
            }
            NodeKind::View { src, groups } => {
                self.leaf_cuts(src, &stream_below_view(axes, groups), out)
            }
            NodeKind::Reindex { src, map, .. } => {
                self.leaf_cuts(src, &stream_below_reindex(axes, map), out)
            }
            _ => push(node, out),
        }
    }

    /// Elements this node materializes to (the product of its output axes'
    /// extents).
    fn volume(&self, node: &Node) -> f64 {
        node.shape().iter().map(|a| a.extent() as f64).product()
    }

    /// The largest volume among stream-varying transcendental maps in the
    /// in-body cone below `node` — the elements `leaf_cuts` is about to
    /// materialize anyway. `None` when nothing below forces a cut. Same
    /// boundaries and axis translation as [`Partitioner::leaf_cuts`].
    fn hot_volume(&self, node: &Node, axes: &[Axis]) -> Option<f64> {
        if self.done.contains_key(&Rc::as_ptr(node)) {
            return None;
        }
        {
            // An invariant subtree cannot host a stream-varying map.
            let na = all_axes(node);
            if !axes.iter().any(|a| na.contains(a)) {
                return None;
            }
        }
        let max = |a: Option<f64>, b: Option<f64>| match (a, b) {
            (Some(x), Some(y)) => Some(x.max(y)),
            (x, None) | (None, x) => x,
        };
        match node.as_ref() {
            NodeKind::Map { inputs, op } => {
                let mut hot = if cheap_op(*op) {
                    None
                } else {
                    Some(self.volume(node))
                };
                for i in inputs {
                    hot = max(hot, self.hot_volume(i, axes));
                }
                hot
            }
            NodeKind::Gather {
                src,
                index,
                axis: g,
            } => max(
                self.hot_volume(src, &stream_below_gather(axes, index, *g)),
                self.hot_volume(index, axes),
            ),
            NodeKind::View { src, groups } => {
                self.hot_volume(src, &stream_below_view(axes, groups))
            }
            NodeKind::Reindex { src, map, .. } => {
                self.hot_volume(src, &stream_below_reindex(axes, map))
            }
            // Fold-bearing subtrees are pushed whole by `leaf_cuts` — their
            // interior is not this cut's concern. Free sources carry no work.
            _ => None,
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
            NodeKind::Gather {
                src,
                index,
                axis: g,
            } if private && *g != axis => {
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

    /// The cheapest axis that derives at this node, if any. Axes that live
    /// only BENEATH an already-materialized producer are vetoed: a cut exp
    /// over a done GEMM is an elementwise map of that buffer, not a second
    /// GEMM with the exp at project. (The veto is the splice's only role
    /// here — derivation, pricing and emission all stay on the original
    /// node, so every surviving choice is unchanged.)
    fn best_fold(&mut self, node: &Node) -> Option<(Axis, Carrier)> {
        let live = all_axes(&self.splice(node, true));
        let mut best: Option<(Axis, Carrier, f64)> = None;
        for axis in all_axes(node) {
            if !live.contains(&axis) {
                continue; // collapsed inside a done producer — read it instead
            }
            if structure(node, axis).level != Parallelism::Monoidal {
                continue;
            }
            let Ok(c) = derive(node, axis) else {
                continue;
            };
            // Rank by planned cost on the uncut graph; an unplannable axis
            // ranks last but is still a legal fold.
            let cost = plan_axis(node, axis, &c, self.dev).map_or(f64::INFINITY, |s| s.cost);
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
            p.leaf_cuts(node, &[axis], &mut cuts);
            for c in cuts {
                p.cut(&c);
                // splice, not `input(name, output_axes, dtype)`: a view/reindex above
                // the cut must keep its reshape so the buffer is read under
                // the axes it was stored with.
                subs.push((c.clone(), p.splice(&c, false)));
            }
        };
        for (idx, leaf) in carrier.leaves.iter().enumerate() {
            // Fuse the online-softmax score contraction in-body (FlashAttention's
            // QKᵀ) — UNLESS it is already materialized. Fusing recomputes the
            // contraction on every streamed step; that pays for itself only by
            // never writing the scores. When the very same contraction is
            // already a live buffer (a logits GEMM demanded as an output, then
            // re-folded by a cross-entropy's logsumexp), reading it is strictly
            // cheaper — recomputing it is the cost-blind cut. Read it instead.
            if in_body.contains(&idx)
                && is_matmul(leaf)
                && !self.done.contains_key(&Rc::as_ptr(leaf))
            {
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
        let cut_graph = replace_many(node, &subs, &mut HashMap::new(), &mut self.canon);

        // Re-derive and plan on the graph the kernel will actually see.
        let c2 = match derive(&cut_graph, axis) {
            Ok(c) => c,
            Err(why) => {
                self.stages.push(Stage::Infeasible {
                    axis,
                    output: out.to_string(),
                    why: why.to_string(),
                });
                return leak(out);
            }
        };
        match plan_axis(&cut_graph, axis, &c2, self.dev) {
            Some(mut spec) => {
                spec.output_name = out.to_string();
                self.stages.push(Stage::Fused {
                    spec: Box::new(spec),
                    epilogue: Vec::new(),
                    epilogue_inputs: Vec::new(),
                    fold_node: cut_graph.clone(),
                    epilogue_node: None,
                    epi_fold_read: leak(out),
                });
            }
            // Legal but UNPLANNABLE: a deferred coupling can price a
            // per-slot column as SRAM-resident (an RMSNorm's deferred
            // normalizer fused into a 200k-vocab head). Cost-driven cut
            // placement, the measured instance: cut the smallest DIV in the
            // body — the normalizer's application site — so the norm
            // becomes its own small stages and the fold re-derives plain.
            // Each retry removes one Div, so the recursion terminates; any
            // feasible schedule strictly beats an Infeasible stage.
            None => {
                if let Some(div) = smallest_div(&cut_graph) {
                    self.cut(&div);
                    let spliced = self.splice(&div, false);
                    let rebuilt = replace_many(
                        &cut_graph,
                        &[(div, spliced)],
                        &mut HashMap::new(),
                        &mut self.canon,
                    );
                    return self.emit(&rebuilt, out);
                }
                self.stages.push(Stage::Infeasible {
                    axis,
                    output: out.to_string(),
                    why: "no block structure fits the device".to_string(),
                });
            }
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

        // Epilogue fusion: the cone rides the LAST of its producers — any
        // earlier ones are materialized first and read as buffers (a
        // SwiGLU cone reads both the gate and the up GEMM; it rides the up
        // fold and loads the gate). The host must be unshared, not yet
        // materialized, and of the cone's own shape.
        let host = complex
            .iter()
            .enumerate()
            .rev()
            .find(|(_, p)| {
                !self.shared(p)
                    && !self.done.contains_key(&Rc::as_ptr(p))
                    && p.shape() == node.shape()
            })
            .map(|(i, _)| i);
        if let Some(hi) = host {
            let producer = complex[hi].clone();
            // materialize the other producers first (execution order)
            let mut subs: Vec<(Node, Node)> = Vec::new();
            let mut extra: Vec<&'static str> = Vec::new();
            for (i, p) in complex.iter().enumerate() {
                if i != hi {
                    let name = self.cut(p);
                    extra.push(name);
                    let read = self.canon.shallow(input(name, &p.shape(), Dtype::F32));
                    subs.push((p.clone(), read));
                }
            }
            let t = self.fresh();
            let before = self.stages.len();
            let landed = self.emit(&producer, t);
            if self.stages.len() > before
                && let Some(Stage::Fused {
                    spec,
                    epilogue,
                    epilogue_inputs,
                    epilogue_node,
                    epi_fold_read,
                    ..
                }) = self.stages.last_mut()
                && spec.output_name == landed
                && epilogue.is_empty()
            {
                // The fold now writes the final output name; the epilogue reads
                // that buffer, the other producers' materializations, and its
                // extra plain inputs, producing the final result in place.
                // In-place update: if the cone ALREADY reads a leaf named `out`
                // (the weight `w` in `w − lr·∇w`), the fold's own output must be
                // read under a distinct name (its temp `landed`) — otherwise the
                // weight read and the fold-output read alias and `w` becomes `∇w`.
                let leaked_out = leak(out);
                let sentinel = if leaf_names(node).iter().any(|n| *n == leaked_out) {
                    landed
                } else {
                    leaked_out
                };
                subs.push((
                    producer.clone(),
                    input(sentinel, &producer.shape(), Dtype::F32),
                ));
                let epi = replace_many(node, &subs, &mut HashMap::new(), &mut self.canon);
                spec.output_name = out.to_string();
                *epilogue = ops;
                let mut all_inputs = plain_inputs;
                all_inputs.extend(extra);
                *epilogue_inputs = all_inputs;
                *epilogue_node = Some(epi);
                *epi_fold_read = sentinel;
                return leak(out);
            }
            // The producer didn't land as a fused kernel — keep the map stage,
            // reading the producers' materialized buffers.
            self.done.insert(Rc::as_ptr(&producer), landed);
            self.keepalive.push(producer.clone());
            let exec = self.executable(node);
            let mut inputs = vec![landed];
            inputs.extend(extra);
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
        let live = !self.done.contains_key(&Rc::as_ptr(node));
        match node.as_ref() {
            // A shared map joins the cone when its op is cheap — each
            // consumer recomputes a few ALU ops instead of forcing a
            // materialized stage. Shared transcendentals stay barriers.
            NodeKind::Map { op, inputs }
                if top || (live && (!self.shared(node) || cheap_op(*op))) =>
            {
                if !ops.contains(&op.name()) {
                    ops.push(op.name());
                }
                for i in inputs {
                    self.cone(i, ops, frontier, false);
                }
            }
            // An indexed load is per-element work, not a producer: keep it
            // in-body and take its source and index as the cone's inputs.
            NodeKind::Gather { src, index, .. } if live => {
                if !ops.contains(&"gather") {
                    ops.push("gather");
                }
                self.cone(src, ops, frontier, false);
                self.cone(index, ops, frontier, false);
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

/// Per-element ops cheap enough to recompute rather than materialize — a
/// launch plus a memory round trip always loses to a handful of ALU ops.
/// The transcendentals are the exception; where they may still inline
/// (stream-invariant subtrees) the caller checks axes, not the op.
fn cheap_op(op: MapOp) -> bool {
    !matches!(
        op,
        MapOp::Exp | MapOp::Log | MapOp::Sqrt | MapOp::Tanh | MapOp::Sin | MapOp::Cos
    )
}

/// Translate a streamed axis set DOWN through one structural boundary — the
/// rule [`Partitioner::leaf_cuts`] and [`Partitioner::hot_volume`] share as they
/// descend. Below a flatten the stream lives on the group members; below a
/// split/window, on the mapped source axes its terms drive; a gather whose
/// index varies with the stream spreads it onto the gathered axis. Anything the
/// boundary doesn't touch passes through. ([`Partitioner::entanglers`] and
/// [`crate::analyze::structure`] translate a single axis the same way, but also
/// stop at an axis *consumed* below the boundary, so they don't reuse this.)
/// Without the translation everything under a flattened fold looks
/// stream-invariant, and a SwiGLU's exp stays in-body of the down projection —
/// recomputed once per output row instead of once per element.
fn stream_below_view(axes: &[Axis], groups: &[(Vec<Axis>, Axis)]) -> Vec<Axis> {
    let mut below = Vec::new();
    for &a in axes {
        match groups.iter().find(|(_, to)| *to == a) {
            Some((members, _)) => below.extend(members.iter().copied()),
            None => below.push(a),
        }
    }
    below
}

fn stream_below_reindex(axes: &[Axis], map: &[crate::kernel_ir::AffineIndex]) -> Vec<Axis> {
    let mut below = Vec::new();
    for &a in axes {
        let mut driving = map
            .iter()
            .filter(|(_, terms, _)| terms.iter().any(|(_, t)| *t == a))
            .map(|(m, _, _)| *m)
            .peekable();
        if driving.peek().is_none() {
            below.push(a);
        } else {
            below.extend(driving);
        }
    }
    below
}

fn stream_below_gather(axes: &[Axis], index: &Node, gathered: Axis) -> Vec<Axis> {
    let index_axes = all_axes(index);
    let mut below = axes.to_vec();
    if axes.iter().any(|a| index_axes.contains(a)) && !below.contains(&gathered) {
        below.push(gathered);
    }
    below
}

/// The smallest-volume normalizer APPLICATION site in the graph — a `Div`,
/// or a `Mul` applying a `Recip` (the two spellings of ÷). This is the cut
/// that removes a deferred coupling from an unplannable fold (see the retry
/// in `emit_fold`).
fn smallest_div(node: &Node) -> Option<Node> {
    let mut best: Option<(f64, Node)> = None;
    fn is_site(node: &Node) -> bool {
        match node.as_ref() {
            NodeKind::Map { op: MapOp::Div, .. } => true,
            NodeKind::Map {
                op: MapOp::Mul,
                inputs,
            } => inputs.iter().any(|i| {
                matches!(
                    i.as_ref(),
                    NodeKind::Map {
                        op: MapOp::Recip,
                        ..
                    }
                )
            }),
            _ => false,
        }
    }
    fn walk(node: &Node, best: &mut Option<(f64, Node)>) {
        match node.as_ref() {
            NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => {}
            NodeKind::Map { inputs, .. } => {
                if is_site(node) {
                    let vol: f64 = node.shape().iter().map(|a| a.extent() as f64).product();
                    if best.as_ref().is_none_or(|(b, _)| vol < *b) {
                        *best = Some((vol, node.clone()));
                    }
                }
                for i in inputs {
                    walk(i, best);
                }
            }
            NodeKind::Reduce { src, .. }
            | NodeKind::Scan { src, .. }
            | NodeKind::View { src, .. }
            | NodeKind::Reindex { src, .. } => walk(src, best),
            NodeKind::Gather { src, index, .. } => {
                walk(src, best);
                walk(index, best);
            }
        }
    }
    walk(node, &mut best);
    best.map(|(_, n)| n)
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
/// Rebuild `node` with `subs` substituted. Every rebuilt node goes back
/// through `canon`, so a spine whose children came out unchanged collapses
/// onto the ORIGINAL node — and two consumers substituting the same reads
/// rebuild the SAME spine, which is what lets `done` deduplicate their cuts.
fn replace_many(
    node: &Node,
    subs: &[(Node, Node)],
    memo: &mut HashMap<*const NodeKind, Node>,
    canon: &mut Canonicalizer,
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
        NodeKind::Map { op, inputs } => crate::kernel_ir::map(
            *op,
            inputs
                .iter()
                .map(|i| replace_many(i, subs, memo, canon))
                .collect(),
        ),
        NodeKind::Reduce { src, axis, op } => {
            crate::kernel_ir::reduce(replace_many(src, subs, memo, canon), *axis, *op)
        }
        NodeKind::Scan { src, axis, op } => {
            crate::kernel_ir::scan(replace_many(src, subs, memo, canon), *axis, *op)
        }
        NodeKind::Gather { src, index, axis } => crate::kernel_ir::gather(
            replace_many(src, subs, memo, canon),
            replace_many(index, subs, memo, canon),
            *axis,
        ),
        NodeKind::View { src, groups } => {
            crate::kernel_ir::view(replace_many(src, subs, memo, canon), groups.clone())
        }
        NodeKind::Reindex { src, map, padded } => {
            crate::kernel_ir::reindex(replace_many(src, subs, memo, canon), map.clone(), *padded)
        }
    };
    let rebuilt = canon.shallow(rebuilt);
    memo.insert(key, rebuilt.clone());
    rebuilt
}

// ── rendering ────────────────────────────────────────────────────────────────

impl Schedule {
    pub fn kernel_count(&self) -> usize {
        self.stages.len()
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
    /// Execution runs at the axes' real shapes.
    pub fn execute(&self, inputs: &Env) -> Value {
        let mut env: Env = inputs.clone();
        self.execute_env(&mut env);
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
    pub fn execute_env(&self, env: &mut Env) {
        // SANIC_DEBUG >= 2: collect per-stage wall times and print them after
        // the run, so each line can carry its share of the whole step (the
        // Metal backend prints GPU times the same way in `crate::compile`).
        let mut timings: Vec<(&'static str, &'static str, usize, f64)> = Vec::new();
        for stage in &self.stages {
            let started = (crate::debug_level() >= 2).then(std::time::Instant::now);
            let (name, tensor): (&'static str, Value) = match stage {
                Stage::Fused {
                    spec,
                    fold_node,
                    epilogue_node,
                    epi_fold_read,
                    ..
                } => {
                    let name = leak(&spec.output_name);
                    let folded = run_carrier(fold_node, spec.streaming_axis, &spec.carrier, env);
                    let result = match epilogue_node {
                        None => folded,
                        Some(epi) => {
                            // expose the fold output under its read name so the
                            // epilogue can read it — distinct from `name` for an
                            // in-place update, so the weight `name` stays intact
                            env.insert(leak(epi_fold_read), folded);
                            eval(epi, env)
                        }
                    };
                    (name, result)
                }
                Stage::Elementwise { output, exec, .. }
                | Stage::Gather { output, exec, .. }
                | Stage::Sequential { output, exec, .. } => (leak(output), eval(exec, env)),
                Stage::Infeasible { output, .. } => {
                    panic!("cannot execute an infeasible stage producing `{output}`")
                }
            };
            if let Some(started) = started {
                let kind = match stage {
                    Stage::Fused { .. } => "fold",
                    Stage::Elementwise { .. } => "map",
                    Stage::Gather { .. } => "gather",
                    Stage::Sequential { .. } => "seq",
                    Stage::Infeasible { .. } => unreachable!(),
                };
                timings.push((name, kind, tensor.data.len(), started.elapsed().as_secs_f64()));
            }
            env.insert(name, tensor);
        }
        if !timings.is_empty() {
            let total = timings.iter().map(|(_, _, _, s)| s).sum::<f64>().max(1e-12);
            let slowest = timings
                .iter()
                .map(|(_, _, _, s)| *s)
                .fold(0.0, f64::max)
                .max(1e-12);
            for (index, (name, kind, elements, seconds)) in timings.iter().enumerate() {
                eprintln!(
                    "*** interp {index:4} {name:<12} {kind:<6} {:7.0}us {} {:4.1}%  {elements:>8} elems",
                    seconds * 1e6,
                    crate::debug_bar(seconds / slowest, 10),
                    100.0 * seconds / total,
                );
            }
            eprintln!(
                "*** interp step: {} stages {:.2}ms",
                timings.len(),
                total * 1e3,
            );
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
                    format!(
                        "{:<4} = fold `{}`({})  [{} slots: {}]{}",
                        spec.output_name,
                        spec.streaming_axis.name,
                        spec.input_names.join(", "),
                        spec.carrier.slots,
                        spec.carrier.rules.join(", "),
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
                    axis.name,
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
                    axis.name,
                    inputs.join(", ")
                ),
                Stage::Infeasible { axis, output, why } => format!(
                    "{output:<4} = fold `{}` — INFEASIBLE: {why}",
                    axis.name
                ),
            };
            out += &format!("  [{i:>2}] {line}\n");
        }
        out
    }

    /// The `SANIC_DEBUG` dump: one line per kernel — its output, kind (fold over
    /// which axis + the derivation moves that fired, map, gather, scan), the
    /// buffers it reads, and the scalar ops it FUSES (the composition collapsed
    /// into that single kernel). This is the fusion boundary made legible: what
    /// the deriver folded into each streaming pass.
    pub fn debug_dump(&self) {
        let (mut nf, mut nm, mut ng, mut ns) = (0, 0, 0, 0);
        for s in &self.stages {
            match s {
                Stage::Fused { .. } => nf += 1,
                Stage::Elementwise { .. } => nm += 1,
                Stage::Gather { .. } => ng += 1,
                Stage::Sequential { .. } => ns += 1,
                Stage::Infeasible { .. } => {}
            }
        }
        eprintln!(
            "[sanic] schedule — {} kernels ({nf} fold, {nm} map, {ng} gather, {ns} scan)",
            self.stages.len()
        );
        for (i, st) in self.stages.iter().enumerate() {
            match st {
                Stage::Fused {
                    spec,
                    fold_node,
                    epilogue,
                    epilogue_inputs,
                    ..
                } => {
                    let mut ops = Vec::new();
                    collect_ops(fold_node, &mut ops);
                    let epi = if epilogue.is_empty() {
                        String::new()
                    } else {
                        format!(
                            "  ▸then {}({})",
                            epilogue.join("·"),
                            epilogue_inputs.join(", ")
                        )
                    };
                    eprintln!(
                        "  [{i:>3}] {:<12} = fold `{}` [{}]  ⇐  {}{}",
                        spec.output_name,
                        spec.streaming_axis.name,
                        spec.carrier.rules.join("+"),
                        op_bag(&ops),
                        epi
                    );
                    eprintln!("        reads {}", spec.input_names.join(", "));
                }
                Stage::Elementwise {
                    output,
                    exec,
                    inputs,
                    ..
                } => {
                    let mut ops = Vec::new();
                    collect_ops(exec, &mut ops);
                    eprintln!("  [{i:>3}] {output:<12} = map  ⇐  {}", op_bag(&ops));
                    eprintln!("        reads {}", inputs.join(", "));
                }
                Stage::Gather {
                    output,
                    axis,
                    inputs,
                    ..
                } => {
                    eprintln!(
                        "  [{i:>3}] {output:<12} = gather `{}`   reads {}",
                        axis.name,
                        inputs.join(", ")
                    );
                }
                Stage::Sequential {
                    output,
                    op,
                    axis,
                    inputs,
                    ..
                } => {
                    eprintln!(
                        "  [{i:>3}] {output:<12} = scan `{op}` over `{}`   reads {}",
                        axis.name,
                        inputs.join(", ")
                    );
                }
                Stage::Infeasible { output, axis, why } => {
                    eprintln!(
                        "  [{i:>3}] {output:<12} = fold `{}` — INFEASIBLE: {why}",
                        axis.name
                    );
                }
            }
        }
    }
}

/// Every scalar op fused into a stage's body, in tree pre-order — walking the
/// cut graph and stopping at its materialized-buffer leaves (`input`), which
/// are the fusion boundary, not ops.
fn collect_ops(node: &Node, out: &mut Vec<String>) {
    match node.as_ref() {
        NodeKind::Input { .. } | NodeKind::Const { .. } | NodeKind::Iota { .. } => {}
        NodeKind::Map { op, inputs } => {
            out.push(format!("{op:?}"));
            inputs.iter().for_each(|i| collect_ops(i, out));
        }
        NodeKind::Reduce { op, axis, src } => {
            out.push(format!("Σ{}/{}", monoid_name(op), axis.name));
            collect_ops(src, out);
        }
        NodeKind::Scan { op, axis, src } => {
            out.push(format!("scan{}/{}", monoid_name(op), axis.name));
            collect_ops(src, out);
        }
        NodeKind::Gather { axis, src, index } => {
            out.push(format!("gather/{}", axis.name));
            collect_ops(src, out);
            collect_ops(index, out);
        }
        NodeKind::View { src, .. } | NodeKind::Reindex { src, .. } => collect_ops(src, out),
    }
}

/// A reduce/scan's combiner as a short label (`Add`, `Max`, `LogSumExp`).
fn monoid_name(op: &BinOp) -> String {
    match op {
        BinOp::Monoid(m) => format!("{m:?}"),
        other => format!("{other:?}"),
    }
}

/// Collapse a repeated op list to counted, first-seen order: `Mul×3, Add, Exp`.
fn op_bag(ops: &[String]) -> String {
    let mut bag: Vec<(String, usize)> = Vec::new();
    for o in ops {
        if let Some(e) = bag.iter_mut().find(|(k, _)| k == o) {
            e.1 += 1;
        } else {
            bag.push((o.clone(), 1));
        }
    }
    bag.iter()
        .map(|(k, n)| {
            if *n > 1 {
                format!("{k}×{n}")
            } else {
                k.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel_ir::*;

    fn add_r() -> BinOp {
        BinOp::Monoid(Monoid::Add)
    }

    // Plain attention over raw tensors: nothing to cut → exactly one kernel.
    #[test]
    fn plain_attention_is_one_kernel() {
        let (s, k, d, e) = (
            axis("s", 1024),
            axis("k", 1024),
            axis("d", 64),
            axis("e", 64),
        );
        let attn = attention(
            input("Q", &[s, d], Dtype::F32),
            input("K", &[k, d], Dtype::F32),
            input("V", &[k, e], Dtype::F32),
            d,
            k,
        );
        let sched = partition(&attn, &DeviceProfile::toy());
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
        let (s, k, dm, dq, dv) = (
            axis("s", 1024),
            axis("k", 1024),
            axis("dm", 512),
            axis("dq", 64),
            axis("dv", 64),
        );
        let x_q = input("Xq", &[s, dm], Dtype::F32);
        let x_kv = input("Xkv", &[k, dm], Dtype::F32);
        let q = matmul(x_q, input("Wq", &[dq, dm], Dtype::F32), dm); // [s, dq]
        let kk = matmul(x_kv.clone(), input("Wk", &[dq, dm], Dtype::F32), dm); // [k, dq]
        let v = matmul(x_kv, input("Wv", &[dv, dm], Dtype::F32), dm); // [k, dv]

        let scores = matmul(q, kk, dq);
        let out = matmul(softmax(scores, k), v, k);

        let sched = partition(&out, &DeviceProfile::toy());

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
        let (s, d) = (axis("s", 1024), axis("d", 1024));
        let x = input("X", &[s, d], Dtype::F32);
        let g = input("G", &[d], Dtype::F32);
        let inv_d = input("inv_d", &[], Dtype::F32);
        let eps = input("eps", &[], Dtype::F32);
        let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), d, add_r());
        let mean = map(MapOp::Mul, vec![ss, inv_d]);
        let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, eps])]);
        let norm = map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom]);

        let sched = partition(&norm, &DeviceProfile::toy());
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
        let (s, d, f) = (axis("s", 1024), axis("d", 1024), axis("f", 512));
        let x = input("X", &[s, d], Dtype::F32);
        let g = input("G", &[d], Dtype::F32);
        let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), d, add_r());
        let mean = map(MapOp::Mul, vec![ss, konst(1.0 / 1024.0)]);
        let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
        let norm = map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom]);
        let proj = matmul(norm, input("W", &[f, d], Dtype::F32), d);

        let sched = partition(&proj, &DeviceProfile::toy());
        assert_eq!(sched.stages.len(), 1, "norm + GEMM = one kernel");
        let Stage::Fused { spec, .. } = &sched.stages[0] else {
            panic!()
        };
        assert_eq!(spec.carrier.slots, 2, "dot product + Σx²");
        assert!(spec.carrier.rules.contains(&"defer-div"));
    }

    // The SAME norm-into-GEMM fusion at a 200k-vocab head is legal but
    // UNPLANNABLE (the deferred normalizer prices a per-slot column as
    // SRAM-resident). The partitioner must not emit Infeasible: it cuts the
    // normalizer's Div, the norm becomes its own stages, and the head
    // re-derives as a plain GEMV — the cut Trinity used to place by hand.
    #[test]
    fn unplannable_norm_head_cuts_the_normalizer() {
        let (s, d, v) = (axis("s", 1), axis("d", 1024), axis("v", 200192));
        let x = input("X", &[s, d], Dtype::F32);
        let g = input("G", &[d], Dtype::F32);
        let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), d, add_r());
        let mean = map(MapOp::Mul, vec![ss, konst(1.0 / 1024.0)]);
        let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
        let norm = map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom]);
        let head = matmul(norm, input("W", &[v, d], Dtype::F32), d);

        let sched = partition(&head, &DeviceProfile::toy());
        assert!(
            !sched
                .stages
                .iter()
                .any(|st| matches!(st, Stage::Infeasible { .. })),
            "the retry must find a feasible schedule, not report Infeasible"
        );
        let Stage::Fused { spec, .. } = sched.stages.last().unwrap() else {
            panic!("head lands as a fold")
        };
        assert_eq!(spec.streaming_axis, d);
        assert_eq!(
            spec.carrier.slots, 1,
            "plain GEMV after the cut, not a deferred-normalizer coupling"
        );
        assert!(
            sched.stages.len() >= 3,
            "norm fold + norm map + head: {} stages",
            sched.stages.len()
        );
    }

    // A residual add rides its producer GEMM as an epilogue — no extra kernel.
    #[test]
    fn residual_add_fuses_as_epilogue() {
        let (s, f, dm) = (axis("s", 1024), axis("f", 4096), axis("dm", 1024));
        let x = input("X", &[s, dm], Dtype::F32);
        let h = input("H", &[s, f], Dtype::F32);
        let w = input("W", &[f, dm], Dtype::F32);
        let proj = matmul(h, w, f); // [s, dm]
        let y = map(MapOp::Add, vec![proj, x]); // residual

        let sched = partition(&y, &DeviceProfile::toy());
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
        let (s, dm, f) = (axis("s", 1024), axis("dm", 1024), axis("f", 4096));
        let x = input("Xn", &[s, dm], Dtype::F32);
        let gate = matmul(x.clone(), input("Wg", &[f, dm], Dtype::F32), dm); // [s, f]
        let up = matmul(x, input("Wu", &[f, dm], Dtype::F32), dm); // [s, f]
        let act = map(MapOp::Mul, vec![silu(gate), up]);
        let down = reduce(
            map(MapOp::Mul, vec![act, input("Wd", &[f, dm], Dtype::F32)]),
            f,
            add_r(),
        );

        let sched = partition(&down, &DeviceProfile::toy());
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

    // A composed logsumexp — `m + log(Σ exp(x − m))` — folds as ONE carrier,
    // not a bare `max` spilled beside a `Σexp` that recomputes it. A free map
    // wrapping only PLAIN reductions is kept whole and re-derives as a single
    // (max, Σexp) rescale carrier; the streamed max never materializes twice.
    // (The dual is above: a free map wrapping a CONTRACTION — silu·gate·up,
    // scale·QKᵀ+mask — still decomposes so the matmul stays in-body / cut.)
    #[test]
    fn composed_logsumexp_folds_as_one_carrier() {
        let (b, c) = (axis("b", 128), axis("c", 32));
        let z = input("Z", &[b, c], Dtype::F32);
        let m = reduce(z.clone(), c, BinOp::Monoid(Monoid::Max));
        let sumexp = reduce(
            map(
                MapOp::Exp,
                vec![map(MapOp::Sub, vec![z.clone(), m.clone()])],
            ),
            c,
            add_r(),
        );
        let lse = map(MapOp::Add, vec![m, map(MapOp::Log, vec![sumexp])]); // [b]
        let loss = reduce(lse, b, add_r()); // scalar

        let sched = partition(&loss, &DeviceProfile::toy());
        assert_eq!(
            sched.stages.len(),
            2,
            "the (max, Σexp) carrier + the outer sum — no third bare-max stage"
        );
        let Stage::Fused { spec, .. } = &sched.stages[0] else {
            panic!("first stage is the fused logsumexp carrier")
        };
        assert_eq!(spec.streaming_axis, c);
        assert_eq!(spec.carrier.slots, 2, "one (max, Σexp) rescale carrier");
    }

    // The FLATTENED variant of the down projection (the W4-matvec shape: the
    // contraction split into (group, lane) so a per-group scale is pure axis
    // structure, then flattened back to one streamed fold). The flatten blocks
    // the lift-fusion path, so the whole product is one LEAF — and the leaf
    // cut must translate the streamed axis through the flatten/split to see
    // that the silu's exp VARIES along the stream. Left in-body it would be
    // recomputed once per output row (the Trinity MoE-down regression). And
    // the cut lands at the TOP of the activation cone, not around the exp:
    // the fold reads ONE materialized activation per streamed element
    // instead of gate + up + exp (three loads cost more than the exp did).
    #[test]
    fn swiglu_leaf_of_a_flattened_fold_materializes_the_cone() {
        let (dm, f, gi, ri, fl) = (
            axis("dm", 1024),
            axis("f", 4096),
            axis("gi", 128),
            axis("ri", 32),
            axis("fl", 4096),
        );
        let gate = input("G", &[f], Dtype::F32);
        let up = input("U", &[f], Dtype::F32);
        let act = map(MapOp::Mul, vec![silu(gate), up]);
        let xs = split(act, f, gi, ri);
        let prod = map(
            MapOp::Mul,
            vec![
                map(MapOp::Mul, vec![input("Wd", &[dm, gi, ri], Dtype::F32), xs]),
                input("Sc", &[dm, gi], Dtype::F32),
            ],
        );
        let down = reduce(flatten(prod, &[gi, ri], fl), fl, add_r());

        let sched = partition(&down, &DeviceProfile::toy());
        // The whole silu·up cone is one elementwise stage; the fold reads it.
        assert_eq!(sched.stages.len(), 2, "activation cone + down fold");
        let Stage::Elementwise { ops, .. } = &sched.stages[0] else {
            panic!("the activation cone is its own elementwise stage")
        };
        assert!(ops.contains(&"exp"), "the cone holds the silu: {ops:?}");
        assert!(ops.contains(&"mul"), "…and the gating multiply: {ops:?}");
        let Stage::Fused {
            spec, fold_node, ..
        } = sched.stages.last().unwrap()
        else {
            panic!("last stage is the fused down fold")
        };
        assert_eq!(spec.streaming_axis, fl);
        fn has_exp(n: &NodeRef) -> bool {
            match n.as_ref() {
                NodeKind::Map { op, inputs } => *op == MapOp::Exp || inputs.iter().any(has_exp),
                NodeKind::Reduce { src, .. } | NodeKind::Scan { src, .. } => has_exp(src),
                NodeKind::View { src, .. } | NodeKind::Reindex { src, .. } => has_exp(src),
                NodeKind::Gather { src, index, .. } => has_exp(src) || has_exp(index),
                _ => false,
            }
        }
        assert!(
            !has_exp(fold_node),
            "no exp recomputed in-body of the flattened fold"
        );
    }

    // When the sibling projections share their contraction axis, the whole
    // activation — both GEMMs, the silu, the gating multiply — derives as ONE
    // fold (the product monoid over both dot products, exp at project): the
    // cone lift hands `emit` the full cone and the algebra takes it whole.
    #[test]
    fn swiglu_siblings_on_one_axis_derive_as_one_fold() {
        let (s, dm, f, gi, ri, fl) = (
            axis("s", 1024),
            axis("dm", 1024),
            axis("f", 4096),
            axis("gi", 128),
            axis("ri", 32),
            axis("fl", 4096),
        );
        let x = input("Xn", &[s, dm], Dtype::F32);
        let gate = matmul(x.clone(), input("Wg", &[f, dm], Dtype::F32), dm); // [s, f]
        let up = matmul(x, input("Wu", &[f, dm], Dtype::F32), dm); // [s, f]
        let act = map(MapOp::Mul, vec![silu(gate), up]);
        let xs = split(act, f, gi, ri);
        let prod = map(
            MapOp::Mul,
            vec![
                map(MapOp::Mul, vec![input("Wd", &[dm, gi, ri], Dtype::F32), xs]),
                input("Sc", &[dm, gi], Dtype::F32),
            ],
        );
        let down = reduce(flatten(prod, &[gi, ri], fl), fl, add_r());

        let sched = partition(&down, &DeviceProfile::toy());
        assert_eq!(sched.stages.len(), 2, "gate+up+silu fold, then down fold");
        let Stage::Fused { spec, .. } = &sched.stages[0] else {
            panic!("the activation derives as one fold")
        };
        assert_eq!(spec.streaming_axis, dm);
        assert!(spec.carrier.slots >= 2, "both dot products in one carrier");
    }

    // An embedding lookup is its own OPAQUE gather stage.
    #[test]
    fn embedding_is_a_gather_stage() {
        let (v, dm, s) = (axis("v", 32000), axis("dm", 1024), axis("s", 1024));
        let emb = embedding(
            input("E", &[v, dm], Dtype::F32),
            input("ids", &[s], Dtype::F32),
            v,
        );
        let sched = partition(&emb, &DeviceProfile::toy());
        assert_eq!(sched.stages.len(), 1);
        assert!(matches!(&sched.stages[0], Stage::Gather { axis, .. } if *axis == v));
    }

    // A rename view shares one materialization: the key/value side of
    // attention reads the SAME normalized tensor the query side computed —
    // one norm in the schedule, zero copies.
    #[test]
    fn a_view_shares_one_norm_across_q_and_kv() {
        let (s, t, dm, dq, dv) = (
            axis("s", 1024),
            axis("t", 1024),
            axis("dm", 512),
            axis("dq", 64),
            axis("dv", 64),
        );
        let x = input("X", &[s, dm], Dtype::F32);
        let g = input("g", &[dm], Dtype::F32);
        let inv = input("inv_dm", &[], Dtype::F32);
        let eps = input("eps", &[], Dtype::F32);
        let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), dm, add_r());
        let mean = map(MapOp::Mul, vec![ss, inv]);
        let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, eps])]);
        let xn = map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom]);
        let xn_t = rename(xn.clone(), s, t); // the key/value view

        let q = matmul(xn, input("Wq", &[dq, dm], Dtype::F32), dm); // [s, dq]
        let k = matmul(xn_t.clone(), input("Wk", &[dq, dm], Dtype::F32), dm); // [t, dq]
        let v = matmul(xn_t, input("Wv", &[dv, dm], Dtype::F32), dm); // [t, dv]
        let attn = matmul(softmax(matmul(q, k, dq), t), v, t);

        let sched = partition(&attn, &DeviceProfile::toy());

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
            axis("h", 8),
            axis("s", 1024),
            axis("t", 1024),
            axis("dk", 64),
            axis("dv", 64),
            axis("dmv", 512),
            axis("dm", 512),
        );
        let attn = attention(
            input("Q", &[h, s, dk], Dtype::F32),
            input("K", &[h, t, dk], Dtype::F32),
            input("V", &[h, t, dv], Dtype::F32),
            dk,
            t,
        );
        let flat = flatten(attn, &[h, dv], dmv); // [s, dmv]
        let o = matmul(flat, input("Wo", &[dmv, dm], Dtype::F32), dmv); // [s, dm]

        let sched = partition(&o, &DeviceProfile::toy());

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
        let (s, t, dk, dv) = (
            axis("s", 1024),
            axis("t", 1024),
            axis("dk", 64),
            axis("dv", 64),
        );
        let scores = matmul(
            input("Q", &[s, dk], Dtype::F32),
            input("K", &[t, dk], Dtype::F32),
            dk,
        );
        let scaled = map(MapOp::Mul, vec![scores, konst(0.125)]);
        let masked = map(MapOp::Add, vec![scaled, causal_mask(s, t)]);
        let out = matmul(softmax(masked, t), input("V", &[t, dv], Dtype::F32), t);

        let sched = partition(&out, &DeviceProfile::toy());
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
