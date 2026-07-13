//! A Metal (MSL) backend: derived kernels as code that runs on an Apple GPU.
//!
//! The node→code recursion and carrier rendering are shared with the Rust
//! backend in [`crate::codegen`]; this file is the Metal [`Lang`] impl plus the
//! GPU-shaped wrapper — **one thread per output grid point**, each thread
//! streaming the folded axis in registers. FlashAttention's QKᵀ becomes an
//! in-thread loop; the `(m, ℓ, o)` accumulator lives in registers; only Q/K/V
//! and the output touch memory.
//!
//! [`emit_fused_metal`] emits one fused kernel; [`emit_schedule_metal`] emits a
//! whole schedule (every kernel + dispatch bookkeeping) for a multi-kernel host
//! to run. Emits `float` (Apple GPUs have no f64), so verification uses an f32
//! tolerance. `tests/metal.rs` compiles the MSL via a generated Swift host,
//! dispatches on the GPU, and checks against [`crate::interp`].

use std::collections::HashMap;

use crate::codegen::{
    Gen, Lang, LaneBody, buffers, carrier_expr, carrier_expr_map, grid_of, offset, san,
    thread_grid_decode, thread_grid_decode_from, value,
};
use crate::derive::{Carrier, Expr};
use crate::interp::Extents;
use crate::ir::{Axis, Dtype, MapOp, Monoid, Node, input_dtypes, output_axes};
use crate::partition::{Schedule, Stage};
use crate::plan::{FoldSched, SIMD, fold_sched, mergeable_out_of_order};

// ── the Metal target ─────────────────────────────────────────────────────────

struct MetalLang;
const METAL: MetalLang = MetalLang;

impl Lang for MetalLang {
    fn lit(&self, v: f64) -> String {
        if v == f64::NEG_INFINITY {
            "(-INFINITY)".into()
        } else if v == f64::INFINITY {
            "INFINITY".into()
        } else {
            format!("{v:?}f")
        }
    }
    fn iota_val(&self, ivar: &str) -> String {
        format!("((float){ivar})")
    }
    fn scalar_decl(&self, name: &str, init: &str) -> String {
        format!("float {name} = {init};")
    }
    fn for_open(&self, var: &str, count: usize) -> String {
        format!("for (uint {var} = 0; {var} < {count}; {var}++) {{")
    }
    fn round_index(&self, name: &str, val: &str) -> String {
        format!("uint {name} = (uint)({val} + 0.5f);")
    }
    fn index_decl(&self, name: &str, val: &str, _mutable: bool) -> String {
        format!("uint {name} = {val};")
    }
    fn signed_index_decl(&self, name: &str, val: &str) -> String {
        format!("long {name} = {val};")
    }
    fn to_signed(&self, expr: &str) -> String {
        format!("((long)({expr}))")
    }
    fn index_from_signed(&self, name: &str, val: &str) -> String {
        format!("uint {name} = (uint)({val});")
    }
    fn clamped_index_decl(&self, name: &str, val: &str, n: usize) -> String {
        format!(
            "uint {name} = (uint)min(max({val}, (long)0), (long){});",
            n - 1
        )
    }
    fn select_bool(&self, cond: &str, a: &str, b: &str) -> String {
        format!("(({cond}) ? ({a}) : ({b}))")
    }
    fn map_op(&self, op: MapOp, a: &[String]) -> String {
        match op {
            MapOp::Add => format!("({} + {})", a[0], a[1]),
            MapOp::Sub => format!("({} - {})", a[0], a[1]),
            MapOp::Mul => format!("({} * {})", a[0], a[1]),
            MapOp::Div => format!("({} / {})", a[0], a[1]),
            MapOp::Max => format!("max({}, {})", a[0], a[1]),
            MapOp::Min => format!("min({}, {})", a[0], a[1]),
            MapOp::Lt => format!("(({}) < ({}) ? 1.0f : 0.0f)", a[0], a[1]),
            MapOp::Neg => format!("(-({}))", a[0]),
            MapOp::Recip => format!("(1.0f / ({}))", a[0]),
            MapOp::Exp => format!("exp({})", a[0]),
            MapOp::Log => format!("log({})", a[0]),
            MapOp::Sqrt => format!("sqrt({})", a[0]),
            // fast-math tanh goes through exp(2x) and returns NaN for |x| ≳ 44;
            // GPT-2's MLP activations really do exceed that. precise:: is IEEE.
            MapOp::Tanh => format!("precise::tanh({})", a[0]),
            MapOp::Sin => format!("sin({})", a[0]),
            MapOp::Cos => format!("cos({})", a[0]),
            MapOp::Where => format!("(({}) != 0.0f ? ({}) : ({}))", a[0], a[1], a[2]),
        }
    }
    fn monoid(&self, m: Monoid, acc: &str, ev: &str) -> String {
        match m {
            Monoid::Add => format!("{acc} + {ev}"),
            Monoid::Mul => format!("{acc} * {ev}"),
            Monoid::Max => format!("max({acc}, {ev})"),
            Monoid::Min => format!("min({acc}, {ev})"),
            Monoid::LogSumExp => format!(
                "({acc} == -INFINITY) ? ({ev}) : (({ev}) == -INFINITY ? {acc} : \
                 (max({acc}, {ev}) + log(exp({acc} - max({acc}, {ev})) + exp(({ev}) - max({acc}, {ev})))))"
            ),
        }
    }
    /// Typed storage reads — the byte-level half of quantization. A `half`
    /// buffer widens on load; packed int4 unpacks two's-complementless
    /// (compressed-tensors stores UNSIGNED q+8 nibbles, low nibble = even
    /// element); int8 is plain signed bytes.
    fn buffer_load(&self, name: &str, off: &str, dtype: Option<Dtype>) -> String {
        let b = san(name);
        match dtype {
            None | Some(Dtype::F32) => format!("{b}[{off}]"),
            Some(Dtype::F16) => format!("((float){b}[{off}])"),
            Some(Dtype::I8) => format!("((float){b}[{off}])"),
            Some(Dtype::I4) => format!("w4({b}, {off})"),
            Some(Dtype::F64) => panic!("Apple GPUs have no f64 buffers"),
        }
    }
    /// Present only in scheduled (cooperative) kernels, which is the only
    /// place [`Gen::lane_body`] is ever set.
    fn lane_var(&self) -> Option<String> {
        Some("lane".into())
    }
    /// A shuffle butterfly: sound for any commutative monoid, which every
    /// [`Monoid`] is.
    fn simd_lane_merge(&self, acc: &str, m: Monoid, width: usize) -> Option<Vec<String>> {
        let mut out = Vec::new();
        let mut off = width / 2;
        while off > 0 {
            let shuf = format!("simd_shuffle_xor({acc}, {off}u)");
            out.push(format!("{acc} = {};", self.monoid(m, acc, &shuf)));
            off /= 2;
        }
        Some(out)
    }
}

/// The `[[buffer(i)]]` pointer type for a storage dtype.
fn buf_ty(dtype: Option<Dtype>) -> &'static str {
    match dtype {
        None | Some(Dtype::F32) => "device const float*",
        Some(Dtype::F16) => "device const half*",
        Some(Dtype::I8) => "device const char*",
        Some(Dtype::I4) => "device const uchar*",
        Some(Dtype::F64) => panic!("Apple GPUs have no f64 buffers"),
    }
}

/// The shared MSL prelude: the int4 nibble decode used by `buffer_load`.
const MSL_HEADER: &str = "#include <metal_stdlib>\nusing namespace metal;\n\n\
inline float w4(device const uchar* p, uint i) {\n\
    return (float)(int)((p[i >> 1] >> ((i & 1u) << 2)) & 0xFu) - 8.0f;\n\
}\n\n";

// ── kernels ──────────────────────────────────────────────────────────────────

/// An emitted Metal kernel: MSL source, entry name, the input buffers it binds
/// (in `[[buffer(0..)]]` order), their declared storage dtypes (absent =
/// float), and the output grid.
pub struct MetalKernel {
    pub msl: String,
    pub name: String,
    pub inputs: Vec<(&'static str, Vec<Axis>)>,
    pub dtypes: HashMap<&'static str, Dtype>,
    pub grid: Vec<Axis>,
    pub grid_size: usize,
}

fn signature(
    bufs: &[(&'static str, Vec<Axis>)],
    dtypes: &HashMap<&'static str, Dtype>,
) -> String {
    let mut params: Vec<String> = bufs
        .iter()
        .enumerate()
        .map(|(i, (n, _))| {
            format!(
                "{} {} [[buffer({i})]]",
                buf_ty(dtypes.get(n).copied()),
                san(n)
            )
        })
        .collect();
    params.push(format!("device float* outb [[buffer({})]]", bufs.len()));
    params.push("uint gid [[thread_position_in_grid]]".to_string());
    params.join(",\n    ")
}

fn wrap(name: &str, sig: String, body: Vec<String>) -> String {
    format!(
        "kernel void {name}(\n    {sig}\n) {{\n{}\n}}\n",
        body.iter()
            .map(|l| format!("    {l}"))
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

/// [`wrap`] for a cooperative kernel: pins the threadgroup size (the host
/// derives it from `maxTotalThreadsPerThreadgroup`) and binds the simdgroup
/// and lane indices the schedule addresses.
fn wrap_sched(name: &str, bufs_sig: String, body: Vec<String>, tg_threads: usize) -> String {
    let sig = format!(
        "{bufs_sig},\n    uint sgid [[simdgroup_index_in_threadgroup]],\n    \
         uint lane [[thread_index_in_simdgroup]]"
    );
    format!(
        "[[max_total_threads_per_threadgroup({tg_threads})]]\n{}",
        wrap(name, sig, body)
    )
}

/// Which items and accumulator slots a carrier expression reads.
fn expr_refs(e: &Expr, items: &mut std::collections::HashSet<usize>, slots: &mut std::collections::HashSet<usize>) {
    match e {
        Expr::Const(_) => {}
        Expr::Item(i) => {
            items.insert(*i);
        }
        Expr::A(k) | Expr::F(k) | Expr::B(k) => {
            slots.insert(*k);
        }
        Expr::Add(a, b)
        | Expr::Sub(a, b)
        | Expr::Mul(a, b)
        | Expr::Div(a, b)
        | Expr::Max(a, b)
        | Expr::Min(a, b)
        | Expr::Lt(a, b) => {
            expr_refs(a, items, slots);
            expr_refs(b, items, slots);
        }
        Expr::Exp(a) | Expr::Log(a) | Expr::Sqrt(a) | Expr::Sin(a) | Expr::Cos(a) => {
            expr_refs(a, items, slots)
        }
        Expr::Where(c, a, b) => {
            expr_refs(c, items, slots);
            expr_refs(a, items, slots);
            expr_refs(b, items, slots);
        }
    }
}

/// Emit an MSL kernel for one fused, scalar-projecting carrier: one thread per
/// output grid point, streaming `stream` in registers.
pub fn emit_fused_metal(
    name: &str,
    carrier: &Carrier,
    stream: Axis,
    fold_node: &Node,
    ext: &Extents,
) -> MetalKernel {
    assert_eq!(
        carrier.project.len(),
        1,
        "metal kernel needs a scalar projection"
    );
    let (grid, grid_size) = grid_of(fold_node, ext);
    let bufs = buffers(fold_node);
    let dtypes: HashMap<&'static str, Dtype> = input_dtypes(fold_node).into_iter().collect();

    let mut g = Gen::new();
    g.dtypes = dtypes.clone();
    let mut body: Vec<String> = vec![format!("if (gid >= {grid_size}) return;")];
    let coord = thread_grid_decode(&METAL, &grid, ext, &mut g, &mut body);

    let slots = carrier.slots;
    let ident = carrier
        .identity
        .iter()
        .map(|v| METAL.lit(*v))
        .collect::<Vec<_>>()
        .join(", ");
    body.push(format!("float acc[{slots}] = {{ {ident} }};"));

    let sv = g.fresh("s");
    let mut cs = coord.clone();
    cs.insert(stream, sv.clone());
    let mut sbody = Vec::new();
    let items: Vec<String> = carrier
        .leaves
        .iter()
        .map(|l| value(&METAL, l, &cs, ext, &mut g, &mut sbody))
        .collect();
    body.push(format!(
        "for (uint {sv} = 0; {sv} < {}; {sv}++) {{",
        ext[&stream]
    ));
    body.extend(sbody.into_iter().map(|s| format!("    {s}")));
    body.push(format!(
        "    float x[{}] = {{ {} }};",
        items.len().max(1),
        if items.is_empty() {
            "0.0f".into()
        } else {
            items.join(", ")
        }
    ));
    body.push(format!(
        "    float el[{slots}] = {{ {} }};",
        carrier
            .into
            .iter()
            .map(|e| carrier_expr(&METAL, e))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    // compute all new accumulator values before overwriting (combine reads old acc)
    body.push(format!(
        "    float na[{slots}] = {{ {} }};",
        carrier
            .combine
            .iter()
            .map(|e| carrier_expr(&METAL, e))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    body.push(format!(
        "    for (uint _j = 0; _j < {slots}; _j++) acc[_j] = na[_j];"
    ));
    body.push("}".into());
    body.push(format!(
        "outb[{}] = {};",
        offset(&grid, &coord, ext),
        carrier_expr(&METAL, &carrier.project[0])
    ));

    MetalKernel {
        msl: format!("{MSL_HEADER}{}", wrap(name, signature(&bufs, &dtypes), body)),
        name: name.to_string(),
        inputs: bufs,
        dtypes,
        grid,
        grid_size,
    }
}

/// [`emit_fused_metal`] under a cooperative [`FoldSched`]: the streamed axis
/// splits across simdgroups and/or lanes, an optional lane-distributed
/// output axis vectorizes the slots that span it (slots that don't are
/// computed once per simdgroup — the span-asymmetry dedup), in-body
/// contractions lane-split through [`Gen::lane_body`], and every merge is
/// the carrier's own `combine`, rendered over simd shuffles or threadgroup
/// memory — the same re-association `run_carrier_split` certifies, one
/// level down from the GROUP split. Any precondition failure falls back to
/// the scalar kernel: a schedule can be slow, never wrong.
pub fn emit_fused_metal_sched(
    name: &str,
    carrier: &Carrier,
    stream: Axis,
    fold_node: &Node,
    ext: &Extents,
    sched: FoldSched,
) -> MetalKernel {
    use std::collections::HashSet;
    let scalar = || emit_fused_metal(name, carrier, stream, fold_node, ext);
    if sched.is_scalar() || !mergeable_out_of_order(carrier) || carrier.project.len() != 1 {
        return scalar();
    }
    let s_ext = ext[&stream];
    let f_split = if sched.lane_stream { SIMD * sched.sgs } else { sched.sgs };
    if f_split > s_ext || (sched.lane_stream && sched.lane_axis.is_some()) {
        return scalar();
    }
    let (grid, _) = grid_of(fold_node, ext);
    if let Some(a) = sched.lane_axis
        && (!grid.contains(&a) || ext[&a] % SIMD != 0)
    {
        return scalar();
    }

    let slots = carrier.slots;
    let sliced_slot: Vec<bool> = (0..slots)
        .map(|j| sched.lane_axis.is_some_and(|a| carrier.spans[j].contains(&a)))
        .collect();
    let sliced_leaf: Vec<bool> = carrier
        .leaves
        .iter()
        .map(|l| sched.lane_axis.is_some_and(|a| output_axes(l).contains(&a)))
        .collect();
    // A slot the schedule holds once per simdgroup must not read a
    // lane-sliced item or slot; spans should guarantee it — verify anyway.
    for j in 0..slots {
        if sliced_slot[j] {
            continue;
        }
        let (mut items, mut srefs) = (HashSet::new(), HashSet::new());
        expr_refs(&carrier.into[j], &mut items, &mut srefs);
        expr_refs(&carrier.combine[j], &mut items, &mut srefs);
        if items.iter().any(|&i| sliced_leaf[i]) || srefs.iter().any(|&k| sliced_slot[k]) {
            return scalar();
        }
    }

    let bufs = buffers(fold_node);
    let dtypes: HashMap<&'static str, Dtype> = input_dtypes(fold_node).into_iter().collect();
    let mut g = Gen::new();
    g.dtypes = dtypes.clone();

    let tgt = sched.tg_threads();
    let sgs = sched.sgs;
    let e_a = sched.lane_axis.map(|a| ext[&a]).unwrap_or(1);
    let v_cnt = e_a / SIMD; // 0 only when lane_axis is None (e_a = 1)
    let tg_grid: Vec<Axis> = grid
        .iter()
        .copied()
        .filter(|ax| Some(*ax) != sched.lane_axis)
        .collect();
    let n_tgs: usize = tg_grid.iter().map(|a| ext[a]).product::<usize>().max(1);

    let mut body: Vec<String> = vec![format!("uint tg_ = gid / {tgt}u;")];
    let coord = thread_grid_decode_from(&METAL, "tg_", &tg_grid, ext, &mut g, &mut body);
    if sgs > 1 {
        // threadgroup partial arrays, declared at kernel scope
        body.push(format!("threadgroup float tgu[{}];", slots * sgs));
        for j in 0..slots {
            if sliced_slot[j] {
                body.push(format!("threadgroup float tgs_{j}[{}];", sgs * e_a));
            }
        }
    }

    let ident: Vec<String> = carrier.identity.iter().map(|v| METAL.lit(*v)).collect();
    body.push(format!("float accu[{slots}] = {{ {} }};", ident.join(", ")));
    for j in 0..slots {
        if sliced_slot[j] {
            body.push(format!(
                "float accs_{j}[{v_cnt}]; for (uint v_ = 0; v_ < {v_cnt}u; v_++) accs_{j}[v_] = {};",
                ident[j]
            ));
        }
    }

    // ── the stream loop, strided over the split units ────────────────────────
    let unit = if sched.lane_stream {
        format!("(sgid * {SIMD}u + lane)")
    } else {
        "sgid".to_string()
    };
    body.push(format!(
        "for (uint s_ = {unit}; s_ < {s_ext}u; s_ += {f_split}u) {{"
    ));
    let mut cs = coord.clone();
    cs.insert(stream, "s_".into());
    if sched.lane_axis.is_some() {
        g.lane_body = Some(LaneBody {
            avoid_axis: sched.lane_axis,
            simd_width: SIMD,
        });
    }
    let mut inner: Vec<String> = Vec::new();
    for (i, l) in carrier.leaves.iter().enumerate() {
        if sliced_leaf[i] {
            continue;
        }
        let mut stmts = Vec::new();
        let v = value(&METAL, l, &cs, ext, &mut g, &mut stmts);
        inner.extend(stmts);
        inner.push(format!("float xu_{i} = {v};"));
    }
    // renderers: `ctx_sliced` decides whether lane-sliced names are in scope
    let item_at = |i: usize, ctx_sliced: bool| -> String {
        if sliced_leaf[i] {
            debug_assert!(ctx_sliced);
            format!("xs_{i}")
        } else {
            format!("xu_{i}")
        }
    };
    let a_at = |k: usize| -> String {
        if sliced_slot[k] {
            format!("accs_{k}[v_]")
        } else {
            format!("accu[{k}]")
        }
    };
    let b_el = |k: usize| -> String {
        if sliced_slot[k] {
            format!("els_{k}")
        } else {
            format!("elu_{k}")
        }
    };
    for j in 0..slots {
        if sliced_slot[j] {
            continue;
        }
        inner.push(format!(
            "float elu_{j} = {};",
            carrier_expr_map(&METAL, &carrier.into[j], &|i| item_at(i, false), &a_at, &b_el)
        ));
    }
    for j in 0..slots {
        if sliced_slot[j] {
            continue;
        }
        inner.push(format!(
            "float nau_{j} = {};",
            carrier_expr_map(&METAL, &carrier.combine[j], &|i| item_at(i, false), &a_at, &b_el)
        ));
    }
    if sliced_slot.iter().any(|&s| s) || sliced_leaf.iter().any(|&s| s) {
        inner.push(format!("for (uint v_ = 0; v_ < {v_cnt}u; v_++) {{"));
        inner.push(format!("    uint la_ = lane + v_ * {SIMD}u;"));
        let mut cv = cs.clone();
        cv.insert(sched.lane_axis.unwrap(), "la_".into());
        let mut vstmts: Vec<String> = Vec::new();
        for (i, l) in carrier.leaves.iter().enumerate() {
            if !sliced_leaf[i] {
                continue;
            }
            let mut stmts = Vec::new();
            let v = value(&METAL, l, &cv, ext, &mut g, &mut stmts);
            vstmts.extend(stmts);
            vstmts.push(format!("float xs_{i} = {v};"));
        }
        for j in 0..slots {
            if !sliced_slot[j] {
                continue;
            }
            vstmts.push(format!(
                "float els_{j} = {};",
                carrier_expr_map(&METAL, &carrier.into[j], &|i| item_at(i, true), &a_at, &b_el)
            ));
        }
        for j in 0..slots {
            if !sliced_slot[j] {
                continue;
            }
            vstmts.push(format!(
                "float nas_{j} = {};",
                carrier_expr_map(&METAL, &carrier.combine[j], &|i| item_at(i, true), &a_at, &b_el)
            ));
        }
        for j in 0..slots {
            if sliced_slot[j] {
                vstmts.push(format!("accs_{j}[v_] = nas_{j};"));
            }
        }
        inner.extend(vstmts.into_iter().map(|s| format!("    {s}")));
        inner.push("}".into());
    }
    for j in 0..slots {
        if !sliced_slot[j] {
            inner.push(format!("accu[{j}] = nau_{j};"));
        }
    }
    body.extend(inner.into_iter().map(|s| format!("    {s}")));
    body.push("}".into());
    g.lane_body = None;

    // ── merges: the carrier's combine at each level ──────────────────────────
    let no_item = |_i: usize| "(0.0f)".to_string(); // combine is item-free (split stage 2 relies on it too)
    if sched.lane_stream {
        body.push(format!("for (uint off_ = {}; off_ > 0; off_ >>= 1) {{", SIMD / 2));
        body.push(format!("    float elb[{slots}];"));
        body.push(format!(
            "    for (uint j_ = 0; j_ < {slots}u; j_++) elb[j_] = simd_shuffle_xor(accu[j_], off_);"
        ));
        for j in 0..slots {
            body.push(format!(
                "    float nab_{j} = {};",
                carrier_expr_map(&METAL, &carrier.combine[j], &no_item, &|k| format!("accu[{k}]"), &|k| {
                    format!("elb[{k}]")
                })
            ));
        }
        for j in 0..slots {
            body.push(format!("    accu[{j}] = nab_{j};"));
        }
        body.push("}".into());
    }
    if sgs > 1 {
        body.push("if (lane == 0) {".into());
        for j in 0..slots {
            if !sliced_slot[j] {
                body.push(format!("    tgu[{j} * {sgs}u + sgid] = accu[{j}];"));
            }
        }
        body.push("}".into());
        for j in 0..slots {
            if sliced_slot[j] {
                body.push(format!(
                    "for (uint v_ = 0; v_ < {v_cnt}u; v_++) tgs_{j}[sgid * {e_a}u + lane + v_ * {SIMD}u] = accs_{j}[v_];"
                ));
            }
        }
        body.push("threadgroup_barrier(mem_flags::mem_threadgroup);".into());
        body.push(format!("for (uint off_ = {}; off_ > 0; off_ >>= 1) {{", sgs / 2));
        body.push("    if (sgid < off_) {".into());
        for j in 0..slots {
            if !sliced_slot[j] {
                body.push(format!(
                    "        float au_{j} = tgu[{j} * {sgs}u + sgid]; float bu_{j} = tgu[{j} * {sgs}u + sgid + off_];"
                ));
            }
        }
        let a_tg = |k: usize| -> String {
            if sliced_slot[k] { format!("as_{k}") } else { format!("au_{k}") }
        };
        let b_tg = |k: usize| -> String {
            if sliced_slot[k] { format!("bs_{k}") } else { format!("bu_{k}") }
        };
        if sliced_slot.iter().any(|&s| s) {
            body.push(format!("        for (uint v_ = 0; v_ < {v_cnt}u; v_++) {{"));
            body.push(format!("            uint la_ = lane + v_ * {SIMD}u;"));
            for j in 0..slots {
                if sliced_slot[j] {
                    body.push(format!(
                        "            float as_{j} = tgs_{j}[sgid * {e_a}u + la_]; float bs_{j} = tgs_{j}[(sgid + off_) * {e_a}u + la_];"
                    ));
                }
            }
            for j in 0..slots {
                if sliced_slot[j] {
                    body.push(format!(
                        "            float nms_{j} = {};",
                        carrier_expr_map(&METAL, &carrier.combine[j], &no_item, &a_tg, &b_tg)
                    ));
                }
            }
            for j in 0..slots {
                if sliced_slot[j] {
                    body.push(format!("            tgs_{j}[sgid * {e_a}u + la_] = nms_{j};"));
                }
            }
            body.push("        }".into());
        }
        body.push("        if (lane == 0) {".into());
        for j in 0..slots {
            if !sliced_slot[j] {
                body.push(format!(
                    "            float nmu_{j} = {};",
                    carrier_expr_map(&METAL, &carrier.combine[j], &no_item, &a_tg, &b_tg)
                ));
            }
        }
        for j in 0..slots {
            if !sliced_slot[j] {
                body.push(format!("            tgu[{j} * {sgs}u + sgid] = nmu_{j};"));
            }
        }
        body.push("        }".into());
        body.push("    }".into());
        body.push("    threadgroup_barrier(mem_flags::mem_threadgroup);".into());
        body.push("}".into());
    }

    // ── project + write ──────────────────────────────────────────────────────
    let proj_a = |k: usize| -> String {
        match (sgs > 1, sliced_slot[k]) {
            (true, true) => format!("tgs_{k}[la_]"),
            (true, false) => format!("tgu[{k} * {sgs}u]"),
            (false, true) => format!("accs_{k}[v_]"),
            (false, false) => format!("accu[{k}]"),
        }
    };
    let proj = carrier_expr_map(&METAL, &carrier.project[0], &no_item, &proj_a, &b_el);
    if sched.lane_axis.is_some() {
        body.push("if (sgid == 0) {".into());
        body.push(format!("    for (uint v_ = 0; v_ < {v_cnt}u; v_++) {{"));
        body.push(format!("        uint la_ = lane + v_ * {SIMD}u;"));
        let mut wc = coord.clone();
        wc.insert(sched.lane_axis.unwrap(), "la_".into());
        body.push(format!("        outb[{}] = {proj};", offset(&grid, &wc, ext)));
        body.push("    }".into());
        body.push("}".into());
    } else {
        body.push("if (sgid == 0 && lane == 0) {".into());
        body.push(format!("    outb[{}] = {proj};", offset(&grid, &coord, ext)));
        body.push("}".into());
    }

    MetalKernel {
        msl: format!(
            "{MSL_HEADER}{}",
            wrap_sched(name, signature(&bufs, &dtypes), body, tgt)
        ),
        name: name.to_string(),
        inputs: bufs,
        dtypes,
        grid,
        grid_size: n_tgs * tgt,
    }
}

/// A fold as a TWO-STAGE split reduction on the GPU — the GROUP schedule.
///
/// The *partial* kernel runs `blocks` threads per output point; thread
/// `(g, b)` folds the b-th contiguous chunk of the streamed axis into a RAW
/// accumulator (no projection) and writes it to a `[grid, block, slot]`
/// buffer. The *combine* kernel runs one thread per output point, merges its
/// `blocks` partials with the carrier's own associative `combine`, and
/// projects once. Equal to the one-pass kernel by the monoid law — which is
/// exactly what `interp::run_carrier_split` certifies numerically.
///
/// This is what makes an occupancy-starved fold (a matvec's grid of 1, a
/// giant softmax denominator) fill the device: stage 1 has `grid·blocks`
/// parallelism. [`crate::plan::split_factor`] prices when it wins.
pub fn emit_split_metal(
    name: &str,
    carrier: &Carrier,
    stream: Axis,
    fold_node: &Node,
    ext: &Extents,
    blocks: usize,
) -> (MetalKernel, MetalKernel) {
    assert!(
        !carrier.kinds.iter().any(|k| matches!(
            k,
            crate::derive::SlotKind::KBestVal { .. } | crate::derive::SlotKind::KBestIdx { .. }
        )),
        "split reduction: a k-best carrier's combine is the singleton insert, \
         not a two-list merge — partials cannot be merged"
    );
    assert_eq!(
        carrier.project.len(),
        1,
        "metal split kernel needs a scalar projection"
    );
    assert!(
        blocks >= 1 && blocks <= ext[&stream],
        "blocks must not exceed the streamed extent (empty chunks would merge \
         identity partials — the −∞ rescale edge)"
    );
    let (grid, grid_size) = grid_of(fold_node, ext);
    let bufs = buffers(fold_node);
    let dtypes: HashMap<&'static str, Dtype> = input_dtypes(fold_node).into_iter().collect();
    let slots = carrier.slots;
    let n = ext[&stream];
    let ident = carrier
        .identity
        .iter()
        .map(|v| METAL.lit(*v))
        .collect::<Vec<_>>()
        .join(", ");

    // ── stage 1: partials ────────────────────────────────────────────────────
    let total = grid_size * blocks;
    let mut g = Gen::new();
    g.dtypes = dtypes.clone();
    let mut body: Vec<String> = vec![
        format!("if (gid >= {total}) return;"),
        format!("uint blk = gid % {blocks};"),
        format!("uint gpt = gid / {blocks};"),
    ];
    let coord = thread_grid_decode_from(&METAL, "gpt", &grid, ext, &mut g, &mut body);
    body.push(format!("float acc[{slots}] = {{ {ident} }};"));
    let sv = g.fresh("s");
    let mut cs = coord.clone();
    cs.insert(stream, sv.clone());
    let mut sbody = Vec::new();
    let items: Vec<String> = carrier
        .leaves
        .iter()
        .map(|l| value(&METAL, l, &cs, ext, &mut g, &mut sbody))
        .collect();
    body.push(format!("uint lo = (blk * {n}u) / {blocks}u;"));
    body.push(format!("uint hi = ((blk + 1) * {n}u) / {blocks}u;"));
    body.push(format!("for (uint {sv} = lo; {sv} < hi; {sv}++) {{"));
    body.extend(sbody.into_iter().map(|s| format!("    {s}")));
    body.push(format!(
        "    float x[{}] = {{ {} }};",
        items.len().max(1),
        if items.is_empty() {
            "0.0f".into()
        } else {
            items.join(", ")
        }
    ));
    body.push(format!(
        "    float el[{slots}] = {{ {} }};",
        carrier
            .into
            .iter()
            .map(|e| carrier_expr(&METAL, e))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    body.push(format!(
        "    float na[{slots}] = {{ {} }};",
        carrier
            .combine
            .iter()
            .map(|e| carrier_expr(&METAL, e))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    body.push(format!(
        "    for (uint _j = 0; _j < {slots}; _j++) acc[_j] = na[_j];"
    ));
    body.push("}".into());
    body.push(format!(
        "for (uint _j = 0; _j < {slots}; _j++) outb[gid*{slots} + _j] = acc[_j];"
    ));
    let partial = MetalKernel {
        msl: format!(
            "{MSL_HEADER}{}",
            wrap(&format!("{name}_partial"), signature(&bufs, &dtypes), body)
        ),
        name: format!("{name}_partial"),
        inputs: bufs,
        dtypes,
        grid: grid.clone(),
        grid_size: total,
    };

    // ── stage 2: combine + project ───────────────────────────────────────────
    let mut body: Vec<String> = vec![format!("if (gid >= {grid_size}) return;")];
    body.push(format!("float acc[{slots}] = {{ {ident} }};"));
    body.push(format!("for (uint b = 0; b < {blocks}; b++) {{"));
    body.push(format!("    float el[{slots}];"));
    body.push(format!(
        "    for (uint _j = 0; _j < {slots}; _j++) el[_j] = b_partials[gid*{} + b*{slots} + _j];",
        blocks * slots
    ));
    body.push(format!(
        "    float na[{slots}] = {{ {} }};",
        carrier
            .combine
            .iter()
            .map(|e| carrier_expr(&METAL, e))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    body.push(format!(
        "    for (uint _j = 0; _j < {slots}; _j++) acc[_j] = na[_j];"
    ));
    body.push("}".into());
    body.push(format!(
        "outb[gid] = {};",
        carrier_expr(&METAL, &carrier.project[0])
    ));
    let combine_inputs: Vec<(&'static str, Vec<Axis>)> = vec![("partials", Vec::new())];
    let combine = MetalKernel {
        msl: format!(
            "{MSL_HEADER}{}",
            wrap(
                &format!("{name}_combine"),
                signature(&combine_inputs, &HashMap::new()),
                body
            )
        ),
        name: format!("{name}_combine"),
        inputs: combine_inputs,
        dtypes: HashMap::new(),
        grid,
        grid_size,
    };

    (partial, combine)
}

/// A straight-line (elementwise / gather / reduce) MSL kernel: one thread per
/// output point, writing [`value`] of the spliced graph. No carrier.
pub fn emit_pointwise_metal(name: &str, exec: &Node, ext: &Extents) -> MetalKernel {
    let (grid, grid_size) = grid_of(exec, ext);
    let bufs = buffers(exec);
    let dtypes: HashMap<&'static str, Dtype> = input_dtypes(exec).into_iter().collect();

    let mut g = Gen::new();
    g.dtypes = dtypes.clone();
    let mut body: Vec<String> = vec![format!("if (gid >= {grid_size}) return;")];
    let coord = thread_grid_decode(&METAL, &grid, ext, &mut g, &mut body);
    let mut vbody = Vec::new();
    let v = value(&METAL, exec, &coord, ext, &mut g, &mut vbody);
    body.extend(vbody);
    body.push(format!("outb[{}] = {v};", offset(&grid, &coord, ext)));

    MetalKernel {
        msl: format!("{MSL_HEADER}{}", wrap(name, signature(&bufs, &dtypes), body)),
        name: name.to_string(),
        inputs: bufs,
        dtypes,
        grid,
        grid_size,
    }
}

// ── whole schedule on the GPU ────────────────────────────────────────────────

/// One dispatch in a whole-schedule GPU program.
pub struct MetalStageInfo {
    pub kernel: String,
    /// Buffer names bound to `[[buffer(0..)]]`, in order.
    pub inputs: Vec<String>,
    /// Buffer name written by this dispatch.
    pub output: String,
    pub grid_size: usize,
}

/// A whole schedule lowered to Metal: all kernels in one MSL source, plus the
/// dispatch order and buffer bookkeeping a host needs. An elementwise epilogue
/// runs *in place* on its fold's output buffer (safe: it is per-element).
pub struct MetalProgram {
    pub msl: String,
    pub stages: Vec<MetalStageInfo>,
    pub inputs: Vec<(&'static str, Vec<Axis>)>,
    /// Storage dtype per input that declared one (absent = float32).
    pub dtypes: HashMap<String, Dtype>,
    /// Intermediate/output buffers to allocate: name → element count.
    pub buffers: Vec<(String, usize)>,
    pub output_name: String,
    pub output_axes: Vec<Axis>,
}

/// Lower a whole schedule to a Metal program (the GPU analog of
/// [`crate::rustgen::emit_schedule`]).
pub fn emit_schedule_metal(sched: &Schedule, ext: &Extents) -> MetalProgram {
    emit_schedule_metal_on(&crate::cost::Device::toy(), sched, ext)
}

/// [`emit_schedule_metal`] with fold schedules priced against a specific
/// device — the Metal examples pass [`crate::cost::Device::m1_pro`], the
/// machine the kernels actually run on.
pub fn emit_schedule_metal_on(dev: &crate::cost::Device, sched: &Schedule, ext: &Extents) -> MetalProgram {
    let ext_f: HashMap<Axis, f64> = ext.iter().map(|(&a, &n)| (a, n as f64)).collect();
    let mut msl = String::from(MSL_HEADER);
    let mut all_dtypes: HashMap<String, Dtype> = HashMap::new();
    let mut stages: Vec<MetalStageInfo> = Vec::new();
    let mut inputs: Vec<(&'static str, Vec<Axis>)> = Vec::new();
    let mut bufsizes: Vec<(String, usize)> = Vec::new();
    let mut produced: Vec<String> = Vec::new();

    let note_inputs =
        |node: &Node, produced: &[String], inputs: &mut Vec<(&'static str, Vec<Axis>)>| {
            for (n, axes) in buffers(node) {
                if !produced.iter().any(|p| p == n) && !inputs.iter().any(|(m, _)| *m == n) {
                    inputs.push((n, axes));
                }
            }
        };
    let note_buffer = |name: &str, size: usize, bufsizes: &mut Vec<(String, usize)>| {
        if !bufsizes.iter().any(|(n, _)| n == name) {
            bufsizes.push((name.to_string(), size));
        }
    };
    // strip the per-kernel MSL header; the program has one shared header.
    let strip = |k: &MetalKernel| k.msl.replace(MSL_HEADER, "");

    let mut last_out = String::new();
    let mut last_axes: Vec<Axis> = Vec::new();

    for stage in &sched.stages {
        match stage {
            Stage::Fused {
                spec,
                fold_node,
                epilogue_node,
                ..
            } => {
                note_inputs(fold_node, &produced, &mut inputs);
                let out = spec.output_name.clone();
                let kname = format!("k_{}_fold", san(&out));
                let sched = fold_sched(fold_node, spec.streaming_axis, &spec.carrier, dev, &ext_f);
                let k = emit_fused_metal_sched(
                    &kname,
                    &spec.carrier,
                    spec.streaming_axis,
                    fold_node,
                    ext,
                    sched,
                );
                for (n, d) in &k.dtypes {
                    all_dtypes.insert(n.to_string(), *d);
                }
                msl.push_str(&strip(&k));
                msl.push('\n');
                note_buffer(&out, k.grid_size, &mut bufsizes);
                stages.push(MetalStageInfo {
                    kernel: kname,
                    inputs: k.inputs.iter().map(|(n, _)| n.to_string()).collect(),
                    output: out.clone(),
                    grid_size: k.grid_size,
                });
                produced.push(out.clone());
                if let Some(epi) = epilogue_node {
                    note_inputs(epi, &produced, &mut inputs);
                    let ename = format!("k_{}_epi", san(&out));
                    let k = emit_pointwise_metal(&ename, epi, ext);
                    for (n, d) in &k.dtypes {
                        all_dtypes.insert(n.to_string(), *d);
                    }
                    msl.push_str(&strip(&k));
                    msl.push('\n');
                    stages.push(MetalStageInfo {
                        kernel: ename,
                        inputs: k.inputs.iter().map(|(n, _)| n.to_string()).collect(),
                        output: out.clone(), // in place on the fold buffer
                        grid_size: k.grid_size,
                    });
                }
                last_out = out;
                last_axes = output_axes(epilogue_node.as_ref().unwrap_or(fold_node));
            }
            Stage::Elementwise { output, exec, .. }
            | Stage::Gather { output, exec, .. }
            | Stage::Sequential { output, exec, .. } => {
                note_inputs(exec, &produced, &mut inputs);
                let kname = format!("k_{}", san(output));
                let k = emit_pointwise_metal(&kname, exec, ext);
                for (n, d) in &k.dtypes {
                    all_dtypes.insert(n.to_string(), *d);
                }
                msl.push_str(&strip(&k));
                msl.push('\n');
                note_buffer(output, k.grid_size, &mut bufsizes);
                stages.push(MetalStageInfo {
                    kernel: kname,
                    inputs: k.inputs.iter().map(|(n, _)| n.to_string()).collect(),
                    output: output.clone(),
                    grid_size: k.grid_size,
                });
                produced.push(output.clone());
                last_out = output.clone();
                last_axes = output_axes(exec);
            }
            Stage::Infeasible { output, .. } => {
                panic!("metal: cannot emit an infeasible stage producing `{output}`")
            }
        }
    }

    MetalProgram {
        msl,
        stages,
        inputs,
        dtypes: all_dtypes,
        buffers: bufsizes,
        output_name: last_out,
        output_axes: last_axes,
    }
}
