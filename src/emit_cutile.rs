//! Emit CuTile Python from a planned kernel.
//!
//! Scope, stated plainly: this emitter covers the row-tiled family with an
//! inner contraction — attention-shaped kernels (one row input, one streamed
//! key-like input, optional streamed value inputs), with either no batch axes
//! or exactly two (batch, head). Everything inside that family is driven by
//! the carrier (`identity`, `kinds`, `combine`, `project`, `spans`); nothing
//! is pattern-matched against a known computation. Anything outside the
//! family gets `None`, not a guess.
//!
//! The mapping from carrier to CuTile:
//!   `identity`   → `ct.full(shape, value)` accumulator init
//!   `kinds[i]`   → intra-tile reduction for slot i (ct.max / ct.sum / ct.mma)
//!   `combine[i]` → inter-tile accumulator update
//!   `project`    → final read-out before the store
//!
//! Honesty note: the emitted source is exercised by string tests only — it
//! has not yet been run on a GPU.

use std::collections::HashMap;

use crate::derive::{Carrier, Expr, SlotKind};
use crate::ir::{Axis, Monoid};
use crate::plan::{KernelSpec, Plan};

// ── Expr → Python ────────────────────────────────────────────────────────────

fn lit_py(v: f64) -> String {
    if v == f64::NEG_INFINITY {
        return "-np.inf".into();
    }
    if v == f64::INFINITY {
        return "np.inf".into();
    }
    if v.fract() == 0.0 && v.abs() < 1e15 {
        return format!("{v:.1}");
    }
    format!("{v}")
}

fn py_prec(e: &Expr) -> u8 {
    match e {
        Expr::Add(..) | Expr::Sub(..) => 1,
        Expr::Mul(..) | Expr::Div(..) => 2,
        _ => 3,
    }
}

/// Render a carrier `Expr` as Python. `A(i)` / `F(i)` → `acc_i`,
/// `B(i)` → `batch_i`. `subst` maps a hoisted `max(acc_i, batch_i)` to its
/// CSE variable.
fn emit_py(e: &Expr, parent_prec: u8, subst: &HashMap<String, String>) -> String {
    if let Expr::Max(a, b) = e
        && let (Expr::A(i), Expr::B(j)) = (a.as_ref(), b.as_ref())
        && i == j
        && let Some(alias) = subst.get(&format!("max_a{i}_b{j}"))
    {
        return alias.clone();
    }

    let p = py_prec(e);
    let r = |a: &Expr, prec| emit_py(a, prec, subst);
    let s = match e {
        Expr::Const(v) => lit_py(*v),
        Expr::Item(i) => format!("item_{i}"),
        Expr::A(i) | Expr::F(i) => format!("acc_{i}"),
        Expr::B(i) => format!("batch_{i}"),
        Expr::Add(a, b) => format!("{} + {}", r(a, p), r(b, p)),
        Expr::Sub(a, b) => format!("{} - {}", r(a, p), r(b, p + 1)),
        Expr::Mul(a, b) => format!("{} * {}", r(a, p), r(b, p)),
        Expr::Div(a, b) => format!("{} / {}", r(a, p), r(b, p + 1)),
        Expr::Max(a, b) => format!("max({}, {})", r(a, 0), r(b, 0)),
        Expr::Min(a, b) => format!("min({}, {})", r(a, 0), r(b, 0)),
        Expr::Exp(a) => format!("ct.exp({})", r(a, 0)),
        Expr::Log(a) => format!("ct.log({})", r(a, 0)),
        Expr::Sqrt(a) => format!("ct.sqrt({})", r(a, 0)),
        Expr::Sin(a) => format!("ct.sin({})", r(a, 0)),
        Expr::Cos(a) => format!("ct.cos({})", r(a, 0)),
        Expr::Lt(a, b) => format!("({} < {})", r(a, 0), r(b, 0)),
        Expr::Where(c, a, b) => format!("ct.where({}, {}, {})", r(c, 0), r(a, 0), r(b, 0)),
    };
    if p < parent_prec { format!("({s})") } else { s }
}

// ── slot shapes ──────────────────────────────────────────────────────────────

fn slot_shape(
    slot: usize,
    c: &Carrier,
    row_axis: Option<Axis>,
    col_axis: Option<Axis>,
) -> &'static str {
    let span = &c.spans[slot];
    let has_row = row_axis.is_some_and(|r| span.contains(&r));
    let has_col = col_axis.is_some_and(|cx| span.contains(&cx));
    match (has_row, has_col) {
        (true, true) => "(TILE_M, TILE_E)",
        (true, false) => "(TILE_M, 1)",
        _ => "(1, 1)",
    }
}

// ── intra-tile reduction ─────────────────────────────────────────────────────

/// One `batch_i = …` line per slot, chosen by `SlotKind`. `scores` is the
/// intra-tile primary (the inner-contraction result). Returns `None` for slot
/// kinds this emitter does not cover.
fn intra_tile_batch(c: &Carrier, col_axis: Option<Axis>, row_inp: &str) -> Option<String> {
    let mut lines: Vec<String> = Vec::new();
    // One exp-weight tile per running max, shared by every slot that rides it.
    let mut exp_vars: HashMap<usize, String> = HashMap::new();

    for i in 0..c.slots {
        let has_col = col_axis.is_some_and(|cx| c.spans[i].contains(&cx));

        match c.kinds[i] {
            SlotKind::AffineStep => return None,
            SlotKind::ArgIdx { .. } => return None, // index-carrying: not in this family
            SlotKind::KBestVal { .. } | SlotKind::KBestIdx { .. } => return None,

            SlotKind::Plain(m) => {
                let op = match m {
                    Monoid::Max => "ct.max",
                    Monoid::Min => "ct.min",
                    Monoid::Mul => "ct.prod",
                    Monoid::Add | Monoid::LogSumExp => "ct.sum",
                };
                let src = item_var(&c.into[i]);
                lines.push(format!(
                    "        batch_{i} = {op}({src}, axis=-1, keepdims=True)"
                ));
            }

            SlotKind::ExpShifted { max_slot } => {
                let w_var = exp_vars
                    .entry(max_slot)
                    .or_insert_with(|| {
                        let v = format!("_w{max_slot}");
                        lines.push(format!("        {v} = ct.exp(scores - batch_{max_slot})"));
                        v
                    })
                    .clone();

                if has_col {
                    // Matrix slot: weighted sum of a streamed value via MMA.
                    let val_var = match &c.into[i] {
                        Expr::Item(j) if *j > 0 => format!("streaming_{}", j - 1),
                        _ => "streaming_0".to_string(),
                    };
                    lines.push(format!(
                        "        batch_{i} = ct.mma({w_var}.astype({row_inp}.dtype), {val_var}, \
                         ct.full((TILE_M, TILE_E), 0.0, dtype=np.float32))"
                    ));
                } else {
                    lines.push(format!(
                        "        batch_{i} = ct.sum({w_var}, axis=-1, keepdims=True)"
                    ));
                }
            }
        }
    }

    Some(lines.join("\n"))
}

/// Python variable for the per-element tile a slot lifts from.
/// `Item(0)` / `Const` → the scores tile; `Item(j>0)` → streamed value j−1.
fn item_var(into_expr: &Expr) -> String {
    match into_expr {
        Expr::Item(j) if *j > 0 => format!("streaming_{}", j - 1),
        _ => "scores".to_string(),
    }
}

// ── inter-tile combine ───────────────────────────────────────────────────────

/// The `new_i = …` lines from the carrier's `combine`, with the running-max
/// merge hoisted once and reused by every slot that references it.
fn emit_combine(c: &Carrier) -> String {
    let mut subst: HashMap<String, String> = HashMap::new();
    let mut lines: Vec<String> = Vec::new();

    for i in 0..c.slots {
        if matches!(&c.combine[i], Expr::Max(a, b)
            if matches!(a.as_ref(), Expr::A(j) if *j == i)
            && matches!(b.as_ref(), Expr::B(j) if *j == i))
        {
            let alias = format!("_m{i}");
            lines.push(format!("        {alias} = max(acc_{i}, batch_{i})"));
            subst.insert(format!("max_a{i}_b{i}"), alias.clone());
            lines.push(format!("        new_{i} = {alias}"));
        } else {
            let expr = emit_py(&c.combine[i], 0, &subst);
            lines.push(format!("        new_{i} = {expr}"));
        }
    }

    let lhs: Vec<String> = (0..c.slots).map(|i| format!("acc_{i}")).collect();
    let rhs: Vec<String> = (0..c.slots).map(|i| format!("new_{i}")).collect();
    lines.push(format!("        {} = {}", lhs.join(", "), rhs.join(", ")));
    lines.join("\n")
}

// ── kernel emitter ───────────────────────────────────────────────────────────

/// Emit one `@ct.kernel` from a planned spec, or `None` when the spec is
/// outside the covered family (no inner contraction, no row tile, affine
/// slots, or a batch layout other than none / (batch, head)).
pub fn cutile_kernel(spec: &KernelSpec) -> Option<String> {
    let c = &spec.carrier;
    if spec.contract_axes.is_empty() || spec.row_axis.is_none() {
        return None;
    }
    let has_batch = match spec.batch_axes.len() {
        0 => false,
        2 => true,
        _ => return None, // only flat and (batch, head) layouts are covered
    };

    let name = &spec.name;
    let stream = &spec.streaming_axis;
    let col_axis = spec.col_axes.first().copied();
    let row_inp = spec.input_names.first().copied().unwrap_or("Q");
    let key_inp = spec.input_names.get(1).copied().unwrap_or("K");
    let val_inps: Vec<&'static str> = spec.input_names.get(2..).unwrap_or(&[]).to_vec();

    // ── grid decode ──────────────────────────────────────────────────────────
    let grid_setup = if has_batch {
        "    bid_m = ct.bid(0)\n    batch_idx = ct.bid(1) // H\n    head_idx = ct.bid(1) % H\n"
            .to_string()
    } else {
        "    bid_m = ct.bid(0)\n".to_string()
    };

    // ── index tuples and tile shapes ─────────────────────────────────────────
    let (row_idx, key_idx, out_idx) = if has_batch {
        (
            "(batch_idx, head_idx, bid_m, 0)".to_string(),
            "(batch_idx, head_idx, 0, j)".to_string(),
            "(batch_idx, head_idx, bid_m, 0)".to_string(),
        )
    } else {
        (
            "(bid_m, 0)".to_string(),
            "(0, j)".to_string(),
            "(bid_m, 0)".to_string(),
        )
    };
    let val_idx = if has_batch {
        "(batch_idx, head_idx, j, 0)".to_string()
    } else {
        "(j, 0)".to_string()
    };

    let (row_shape, key_shape, key_order, out_shape, k_seq_dim) = if has_batch {
        (
            "(1, 1, TILE_M, TILE_D)",
            "(1, 1, TILE_D, TILE_N)",
            "(0, 1, 3, 2)",
            "(1, 1, TILE_M, TILE_E)",
            2usize,
        )
    } else {
        (
            "(TILE_M, TILE_D)",
            "(TILE_D, TILE_N)",
            "(1, 0)",
            "(TILE_M, TILE_E)",
            0usize,
        )
    };
    let val_shape = if has_batch {
        "(1, 1, TILE_N, TILE_E)"
    } else {
        "(TILE_N, TILE_E)"
    };

    // ── accumulator init ─────────────────────────────────────────────────────
    let mut acc_init = String::new();
    for i in 0..c.slots {
        let shape = slot_shape(i, c, spec.row_axis, col_axis);
        let val = lit_py(c.identity[i]);
        acc_init.push_str(&format!(
            "    acc_{i} = ct.full({shape}, {val}, dtype=np.float32)\n"
        ));
    }

    // ── row input, loaded once per block ─────────────────────────────────────
    let row_load = format!(
        "    {row_inp}_t = ct.load({row_inp}, index={row_idx}, shape={row_shape})\
         .reshape((TILE_M, TILE_D))\n"
    );

    // ── the streaming loop ───────────────────────────────────────────────────
    let mut loop_body = String::new();
    loop_body.push_str(&format!(
        "        {key_inp}_t = ct.load({key_inp}, index={key_idx}, shape={key_shape},\n"
    ));
    loop_body.push_str(&format!(
        "                      order={key_order}, latency=2).reshape((TILE_D, TILE_N))\n"
    ));
    loop_body.push_str(&format!(
        "        scores = ct.mma({row_inp}_t, {key_inp}_t, ct.full((TILE_M, TILE_N), 0.0, dtype=np.float32))\n"
    ));
    for (vi, vname) in val_inps.iter().enumerate() {
        loop_body.push_str(&format!(
            "        streaming_{vi} = ct.load({vname}, index={val_idx}, \
             shape={val_shape}, latency=4).reshape((TILE_N, TILE_E))\n"
        ));
    }
    loop_body.push_str(&intra_tile_batch(c, col_axis, row_inp)?);
    loop_body.push('\n');
    loop_body.push_str(&emit_combine(c));
    loop_body.push('\n');

    let project = emit_py(&c.project[0], 0, &HashMap::new());

    // ── assemble ─────────────────────────────────────────────────────────────
    let inputs_sig = spec.input_names.join(", ");
    let h_param = if has_batch {
        "H: ConstInt,\n              "
    } else {
        ""
    };
    let out_name = &spec.output_name;

    let src = format!(
        "\
import cuda.tile as ct
import numpy as np

ConstInt = ct.Constant[int]


@ct.kernel(occupancy=2)
def {name}({inputs_sig}, {out_name},
              {h_param}TILE_D: ConstInt,
              TILE_M: ConstInt,
              TILE_N: ConstInt,
              TILE_E: ConstInt):
    \"\"\"Generated from the derived carrier — streams over `{stream}`.\"\"\"
{grid_setup}
{acc_init}
{row_load}
    # stream over `{stream}`
    for j in range(ct.cdiv({key_inp}.shape[{k_seq_dim}], TILE_N)):
{loop_body}
    # project (from carrier.project) and store
    result = {project}
    ct.store({out_name}, index={out_idx},
             tile=result.reshape({out_shape}).astype({out_name}.dtype))
"
    );

    Some(src)
}

// ── module emitter ───────────────────────────────────────────────────────────

/// Emit a whole plan as one Python module: every kernel that the emitter
/// covers, then a `run(...)` launcher that passes each kernel its real
/// arguments (tile constants baked in, `H` and tile extents read off the
/// tensors). Kernels outside the covered family become a TODO comment.
pub fn emit_module(plan: &Plan) -> String {
    let emitted: Vec<(&KernelSpec, Option<String>)> = plan
        .kernels
        .iter()
        .map(|spec| (spec, cutile_kernel(spec)))
        .collect();

    let mut parts: Vec<String> = Vec::new();
    for (spec, src) in &emitted {
        match src {
            Some(src) => parts.push(src.clone()),
            None => parts.push(format!(
                "# TODO: no emitter for kernel '{}' (streaming over '{}')\n",
                spec.name, spec.streaming_axis
            )),
        }
    }
    parts.push(emit_run(&emitted));
    parts.join("\n")
}

fn emit_run(kernels: &[(&KernelSpec, Option<String>)]) -> String {
    let Some((last, _)) = kernels.last() else {
        return "def run(): pass\n".to_string();
    };

    let mut seen = std::collections::HashSet::new();
    let unique_inputs: Vec<&str> = kernels
        .iter()
        .flat_map(|(s, _)| s.input_names.iter().copied())
        .filter(|n| seen.insert(*n))
        .collect();
    let sig = unique_inputs.join(", ");
    let out = &last.output_name;

    let mut body = String::new();
    for (spec, src) in kernels {
        if src.is_none() {
            body += &format!("    # TODO: kernel '{}' has no emitter\n", spec.name);
            continue;
        }
        let row = spec.input_names.first().copied().unwrap_or("Q");
        let has_batch = spec.batch_axes.len() == 2;

        let grid = if has_batch {
            format!(
                "(ct.cdiv({row}.shape[-2], {m}), {row}.shape[0] * {row}.shape[1], 1)",
                m = spec.tile_m
            )
        } else {
            format!("(ct.cdiv({row}.shape[-2], {m}), 1, 1)", m = spec.tile_m)
        };

        // Arguments mirror the kernel signature exactly:
        // inputs…, Out, [H,] TILE_D, TILE_M, TILE_N, TILE_E.
        let h_arg = if has_batch {
            format!("{row}.shape[1], ")
        } else {
            String::new()
        };
        body += &format!(
            "    # '{}' — streams over '{}'\n",
            spec.name, spec.streaming_axis
        );
        body += &format!(
            "    ct.launch(torch.cuda.current_stream(), {grid}, {name}, \
             ({inputs}, {out}, {h_arg}{row}.shape[-1], {m}, {n}, {out}.shape[-1]))\n",
            name = spec.name,
            inputs = spec.input_names.join(", "),
            out = spec.output_name,
            m = spec.tile_m,
            n = spec.tile_n,
        );
    }

    format!("def run({sig}, {out}):\n    import torch\n    import cuda.tile as ct\n{body}")
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cost::Device;
    use crate::derive::derive;
    use crate::ir::*;
    use crate::plan::plan;

    struct Ax {
        sq: Axis,
        k: Axis,
        d: Axis,
        e: Axis,
    }
    fn ax() -> Ax {
        Ax {
            sq: axis("sq"),
            k: axis("k"),
            d: axis("d"),
            e: axis("e"),
        }
    }

    fn spec_for(node: &Node, a: &Ax, batch_axes: Vec<Axis>) -> KernelSpec {
        KernelSpec {
            name: "fused_attn".to_string(),
            streaming_axis: a.k,
            contract_axes: vec![a.d],
            row_axis: Some(a.sq),
            col_axes: vec![a.e],
            batch_axes,
            carrier: derive(node, a.k).unwrap(),
            tile_m: 128,
            tile_n: 64,
            col_tile_axis: None,
            tile_c: 1,
            input_names: vec!["Q", "K", "V"],
            output_name: "Out".to_string(),
            cost: 0.0,
            roofline: dummy_roofline(),
        }
    }

    fn dummy_roofline() -> crate::cost::Kernel {
        crate::cost::Kernel {
            name: "test".to_string(),
            flops: 0.0,
            hbm_bytes: 0.0,
            sram_per_block: 0.0,
            regs_per_block: 0.0,
            parallel_blocks: 1.0,
            lanes_per_block: 1.0,
        }
    }

    #[test]
    fn attention_carrier_emits_kernel() {
        let a = ax();
        let (b, h) = (axis("b"), axis("h"));
        let attn = attention(
            input("Q", &[b, h, a.sq, a.d]),
            input("K", &[b, h, a.k, a.d]),
            input("V", &[b, h, a.k, a.e]),
            a.d,
            a.k,
        );
        let src = cutile_kernel(&spec_for(&attn, &a, vec![b, h])).expect("attention must emit");

        assert!(src.contains("@ct.kernel"));
        assert!(src.contains("def fused_attn("));
        assert!(src.contains("streams over `k`"));

        // two MMAs: the QK contraction and the weighted-value sum
        assert!(src.matches("ct.mma(").count() >= 2, "expected ≥2 ct.mma");
        assert!(src.contains("ct.max("), "missing row-max");
        assert!(src.contains("ct.sum("), "missing softmax-sum");
        assert!(src.contains("ct.exp("), "missing exp");
        assert!(src.contains("-np.inf"), "missing -inf identity");
        assert!(src.contains("acc_2 / acc_1"), "missing projection");

        // the value dim is TILE_E, not conflated with the contract dim
        assert!(src.contains("TILE_E: ConstInt"));
        assert!(src.contains("(TILE_N, TILE_E)"));

        // no phantom parameters: the IR had no scale, so the kernel has none
        assert!(!src.contains("qk_scale"));

        // CSE: the running-max update is hoisted exactly once
        assert_eq!(src.matches("_m0 = max(acc_0, batch_0)").count(), 1);
    }

    #[test]
    fn single_head_omits_batch_params() {
        let a = ax();
        let attn = attention(
            input("Q", &[a.sq, a.d]),
            input("K", &[a.k, a.d]),
            input("V", &[a.k, a.e]),
            a.d,
            a.k,
        );
        let src = cutile_kernel(&spec_for(&attn, &a, vec![])).expect("single-head must emit");
        assert!(!src.contains("H: ConstInt"), "H param should be absent");
        assert!(!src.contains("batch_idx"), "batch decode should be absent");
    }

    #[test]
    fn affine_scan_returns_none() {
        let (t, d, b) = (axis("t"), axis("d"), axis("b"));
        let ssm = ssm_scan(input("AB", &[t]), t);
        let spec = KernelSpec {
            name: "ssm".to_string(),
            streaming_axis: t,
            contract_axes: vec![d],
            row_axis: Some(b),
            col_axes: vec![],
            batch_axes: vec![],
            carrier: derive(&ssm, t).unwrap(),
            tile_m: 1,
            tile_n: 64,
            col_tile_axis: None,
            tile_c: 1,
            input_names: vec!["AB"],
            output_name: "Out".to_string(),
            cost: 0.0,
            roofline: dummy_roofline(),
        };
        assert!(cutile_kernel(&spec).is_none(), "affine slots must decline");
    }

    #[test]
    fn sum_without_contraction_returns_none() {
        let n = axis("n");
        let x = input("X", &[n]);
        let s = reduce(x, n, BinOp::Monoid(Monoid::Add));
        let spec = KernelSpec {
            name: "sum_kernel".to_string(),
            streaming_axis: n,
            contract_axes: vec![],
            row_axis: None,
            col_axes: vec![],
            batch_axes: vec![],
            carrier: derive(&s, n).unwrap(),
            tile_m: 1,
            tile_n: 64,
            col_tile_axis: None,
            tile_c: 1,
            input_names: vec!["X"],
            output_name: "Out".to_string(),
            cost: 0.0,
            roofline: dummy_roofline(),
        };
        assert!(cutile_kernel(&spec).is_none());
    }

    #[test]
    fn emitted_module_has_a_working_launcher() {
        let a = ax();
        let attn = attention(
            input("Q", &[a.sq, a.d]),
            input("K", &[a.k, a.d]),
            input("V", &[a.k, a.e]),
            a.d,
            a.k,
        );
        let dev = Device::toy();
        let extents: HashMap<Axis, f64> = [(a.sq, 1024.0), (a.k, 1024.0), (a.d, 64.0), (a.e, 64.0)]
            .iter()
            .copied()
            .collect();

        let plan = plan(&attn, &dev, &extents).unwrap();
        let py = emit_module(&plan);

        assert!(py.contains("@ct.kernel"), "missing kernel");
        assert!(py.contains("ct.mma("), "missing matmul");
        assert!(py.contains("acc_2 / acc_1"), "missing projection");
        assert!(py.contains("def run(Q, K, V, Out):"), "missing launcher");
        // the launcher passes real arguments, mirroring the signature:
        // grid from the row input, then Out, TILE_D, TILE_M, TILE_N, TILE_E
        assert!(py.contains("ct.cdiv(Q.shape[-2],"), "grid from row input");
        assert!(
            py.contains("Q.shape[-1],") && py.contains("Out.shape[-1]))"),
            "tile extents read off the tensors"
        );
        // no references to bare axis names as if they were Python variables
        assert!(!py.contains("b.shape") && !py.contains("h.shape"));
    }

    #[cfg(test)]
    mod print_kernel {
        // `cargo test -- --ignored print_generated` to eyeball the output.
        #[test]
        #[ignore]
        fn print_generated() {
            use super::super::*;
            use super::{Ax, ax, dummy_roofline};
            use crate::derive::derive;
            use crate::ir::*;
            let a = ax();
            let (b, h) = (axis("b"), axis("h"));
            let attn = attention(
                input("Q", &[b, h, a.sq, a.d]),
                input("K", &[b, h, a.k, a.d]),
                input("V", &[b, h, a.k, a.e]),
                a.d,
                a.k,
            );
            let Ax { sq, k, d, e } = a;
            let spec = KernelSpec {
                name: "flash_attention".to_string(),
                streaming_axis: k,
                contract_axes: vec![d],
                row_axis: Some(sq),
                col_axes: vec![e],
                batch_axes: vec![b, h],
                carrier: derive(&attn, k).unwrap(),
                tile_m: 128,
                tile_n: 64,
                col_tile_axis: None,
                tile_c: 1,
                input_names: vec!["Q", "K", "V"],
                output_name: "Out".to_string(),
                cost: 0.0,
                roofline: dummy_roofline(),
            };
            println!("{}", cutile_kernel(&spec).unwrap());
        }
    }
}
