//! Emit CuTile Python kernels from derived carriers.
//!
//! Generic tile-level code generation. Every structural decision is driven by
//! the carrier (`identity`, `kinds`, `combine`, `project`, `spans`) and the
//! `KernelCodegenParams` axis layout. No computation-specific patterns are
//! checked here; the same emitter covers any carrier the engine produces via
//! R1–R5 with an inner contraction and a SRAM row-tile.
//!
//! The mapping from carrier to CuTile:
//!   `identity`      → `ct.full(shape, value)` accumulator initialisation
//!   `kinds[i]`      → intra-tile batch reduction for slot i (ct.max / ct.sum / ct.mma)
//!   `combine[i]`    → inter-tile accumulator update, Expr → Python
//!   `project`       → final read-out before store

use std::collections::HashMap;

use crate::carrier::{Carrier, Expr, SlotKind};
use crate::op::Monoid;

// ── layout parameters ─────────────────────────────────────────────────────────

/// Axis layout and tensor names for a single CuTile kernel.
pub struct KernelCodegenParams {
    /// Python function name for the emitted `@ct.kernel`.
    pub name: String,
    /// The MONOIDAL axis being streamed (folded).
    pub stream_axis: String,
    /// Inner contraction axis (e.g. head-dim for a dot-product). Empty if none.
    pub contract_axis: String,
    /// The FREE axis tiled across SRAM rows per block.
    pub row_axis: String,
    /// The FREE axis tiled across SRAM columns per block (value head dim). Empty if none.
    pub col_axis: String,
    /// Outer batch axes packed into the Y grid dimension.
    pub batch_axes: Vec<String>,
    /// Input tensor names in order: [row_input, key_input, value_inputs…].
    pub input_names: Vec<&'static str>,
    /// Output tensor name.
    pub output_name: String,
}

// ── Expr → Python ─────────────────────────────────────────────────────────────

fn lit_py(v: f64) -> String {
    if v == f64::NEG_INFINITY { return "-np.inf".into(); }
    if v == f64::INFINITY     { return  "np.inf".into(); }
    if v.fract() == 0.0 && v.abs() < 1e15 { return format!("{:.1}", v); }
    format!("{v}")
}

fn py_prec(e: &Expr) -> u8 {
    match e {
        Expr::Add(..) | Expr::Sub(..) => 1,
        Expr::Mul(..) | Expr::Div(..) => 2,
        _ => 3,
    }
}

/// Render a carrier `Expr` as a CuTile Python expression.
///
/// `A(i)` / `F(i)` → `acc_{i}`, `B(i)` → `batch_{i}`.
/// `subst` maps `"max_a{i}_b{i}"` → a pre-computed CSE variable name, applied
/// whenever a `Max(A(i), B(i))` sub-expression is encountered.
fn emit_py(e: &Expr, parent_prec: u8, subst: &HashMap<String, String>) -> String {
    // Generic CSE: Max(A(i), B(i)) → lookup alias in subst
    if let Expr::Max(a, b) = e {
        if let (Expr::A(i), Expr::B(j)) = (a.as_ref(), b.as_ref()) {
            if i == j {
                let key = format!("max_a{i}_b{j}");
                if let Some(alias) = subst.get(&key) {
                    return alias.clone();
                }
            }
        }
    }

    let p = py_prec(e);
    let r = |a: &Expr, prec| emit_py(a, prec, subst);
    let s = match e {
        Expr::Const(v)           => lit_py(*v),
        Expr::Item(i)            => format!("item_{i}"),
        Expr::A(i) | Expr::F(i) => format!("acc_{i}"),
        Expr::B(i)               => format!("batch_{i}"),
        Expr::Add(a, b)          => format!("{} + {}",    r(a, p),     r(b, p)),
        Expr::Sub(a, b)          => format!("{} - {}",    r(a, p),     r(b, p + 1)),
        Expr::Mul(a, b)          => format!("{} * {}",    r(a, p),     r(b, p)),
        Expr::Div(a, b)          => format!("{} / {}",    r(a, p),     r(b, p + 1)),
        Expr::Max(a, b)          => format!("max({}, {})", r(a, 0),    r(b, 0)),
        Expr::Min(a, b)          => format!("min({}, {})", r(a, 0),    r(b, 0)),
        Expr::Exp(a)             => format!("ct.exp({})",  r(a, 0)),
        Expr::Log(a)             => format!("ct.log({})",  r(a, 0)),
    };
    if p < parent_prec { format!("({s})") } else { s }
}

// ── slot shape ────────────────────────────────────────────────────────────────

fn slot_shape(slot: usize, c: &Carrier, row_axis: &str, col_axis: &str) -> &'static str {
    let span = &c.spans[slot];
    let has_row = span.iter().any(|ax| *ax == row_axis);
    let has_col = !col_axis.is_empty() && span.iter().any(|ax| *ax == col_axis);
    match (has_row, has_col) {
        (true,  true)  => "(TILE_M, TILE_D)",
        (true,  false) => "(TILE_M, 1)",
        _              => "(1, 1)",
    }
}

// ── intra-tile batch reduction ─────────────────────────────────────────────────

/// Generate `batch_i = …` lines for each slot by inspecting `SlotKind`.
///
/// `primary` is the variable holding the intra-tile primary tile (scores after
/// scaling, if an inner contraction exists).  `row_inp` is the name of the row
/// input tensor (used for `.dtype` casts in MMA calls).
fn intra_tile_batch(c: &Carrier, primary: &str, col_axis: &str, row_inp: &str) -> Option<String> {
    let mut lines: Vec<String> = Vec::new();
    // Cache exp-weight variable per max_slot index.
    let mut exp_vars: HashMap<usize, String> = HashMap::new();

    for i in 0..c.slots {
        let has_col = !col_axis.is_empty() && c.spans[i].iter().any(|ax| *ax == col_axis);

        match c.kinds[i] {
            SlotKind::AffineStep => return None,

            SlotKind::Plain(Monoid::Max) => {
                let src = item_var(&c.into[i], primary);
                lines.push(format!("        batch_{i} = ct.max({src}, axis=-1, keepdims=True)"));
            }

            SlotKind::Plain(Monoid::Add | Monoid::LogSumExp) => {
                let src = item_var(&c.into[i], primary);
                lines.push(format!("        batch_{i} = ct.sum({src}, axis=-1, keepdims=True)"));
            }

            SlotKind::Plain(Monoid::Mul) => {
                let src = item_var(&c.into[i], primary);
                lines.push(format!("        batch_{i} = ct.prod({src}, axis=-1, keepdims=True)"));
            }

            SlotKind::Plain(Monoid::Min) => {
                let src = item_var(&c.into[i], primary);
                lines.push(format!("        batch_{i} = ct.min({src}, axis=-1, keepdims=True)"));
            }

            SlotKind::ExpShifted { max_slot } => {
                let w_var = exp_vars.entry(max_slot).or_insert_with(|| {
                    let v = format!("_w{max_slot}");
                    lines.push(format!("        {v} = ct.exp({primary} - batch_{max_slot})"));
                    v
                }).clone();

                if has_col {
                    // Matrix output: weighted sum via MMA.
                    // into[i] is Item(j) where j > 0 → streaming value input j-1.
                    let val_var = match &c.into[i] {
                        Expr::Item(j) if *j > 0 => format!("streaming_{}", j - 1),
                        _ => "streaming_0".to_string(),
                    };
                    lines.push(format!(
                        "        batch_{i} = ct.mma({w_var}.astype({row_inp}.dtype), {val_var}, \
                         ct.full((TILE_M, TILE_D), 0.0, dtype=np.float32))"
                    ));
                } else {
                    lines.push(format!("        batch_{i} = ct.sum({w_var}, axis=-1, keepdims=True)"));
                }
            }
        }
    }

    Some(lines.join("\n"))
}

/// Python variable name for the per-element tile referenced by `into[slot]`.
/// Item(0) → `primary` (the scores / contraction result).
/// Item(j>0) → `streaming_{j-1}` (the j-th streaming value input).
/// Const(_) → `primary` (weight is constant; the slot doesn't use a separate tile).
fn item_var(into_expr: &Expr, primary: &str) -> String {
    match into_expr {
        Expr::Item(0) | Expr::Const(_) => primary.to_string(),
        Expr::Item(j) => format!("streaming_{}", j - 1),
        _ => primary.to_string(), // compound into exprs treated as primary
    }
}

// ── inter-tile combine ────────────────────────────────────────────────────────

/// Emit the `new_i = …` lines and the parallel assignment from the carrier's
/// `combine` expressions, with CSE for `max(acc_i, batch_i)` sub-expressions.
fn emit_combine(c: &Carrier) -> String {
    let mut subst: HashMap<String, String> = HashMap::new();
    let mut lines: Vec<String> = Vec::new();

    for i in 0..c.slots {
        // If combine[i] IS exactly Max(A(i), B(i)), hoist it and add to subst
        // so downstream slots that reference it use the cached alias.
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

// ── kernel emitter ────────────────────────────────────────────────────────────

/// Emit a complete `@ct.kernel` Python source string from a derived carrier.
///
/// Returns `None` when the carrier contains slot kinds the CuTile emitter does
/// not yet support (e.g. `AffineStep`) or when the kernel has no inner
/// contraction or no SRAM row tiling (pure scalar reductions).
pub fn cutile_kernel(c: &Carrier, params: &KernelCodegenParams) -> Option<String> {
    let has_contract = !params.contract_axis.is_empty();
    let has_batch    = params.batch_axes.len() >= 2;

    // Require a row-tiled, contraction-carrying kernel for now.
    if !has_contract || params.row_axis.is_empty() {
        return None;
    }

    let name     = &params.name;
    let stream   = &params.stream_axis;
    let row_inp  = params.input_names.first().copied().unwrap_or("Q");
    let key_inp  = params.input_names.get(1).copied().unwrap_or("K");
    let val_inps: Vec<&'static str> = params.input_names.get(2..).unwrap_or(&[]).to_vec();

    // ── grid / batch setup ───────────────────────────────────────────────────
    let grid_setup = if has_batch {
        "    bid_m = ct.bid(0)\n    batch_idx = ct.bid(1) // H\n    head_idx = ct.bid(1) % H\n"
            .to_string()
    } else {
        "    bid_m = ct.bid(0)\n".to_string()
    };

    // ── index tuples ─────────────────────────────────────────────────────────
    let (row_idx, key_idx, out_idx) = if has_batch {
        (
            "(batch_idx, head_idx, bid_m, 0)".to_string(),
            "(batch_idx, head_idx, 0, j)".to_string(),
            "(batch_idx, head_idx, bid_m, 0)".to_string(),
        )
    } else {
        ("(bid_m, 0)".to_string(), "(0, j)".to_string(), "(bid_m, 0)".to_string())
    };
    let val_idx = if has_batch {
        "(batch_idx, head_idx, j, 0)".to_string()
    } else {
        "(j, 0)".to_string()
    };

    // ── tile shapes ──────────────────────────────────────────────────────────
    let (row_shape, key_shape, key_order, out_shape, k_seq_dim) = if has_batch {
        ("(1, 1, TILE_M, TILE_D)", "(1, 1, TILE_D, TILE_N)", "(0, 1, 3, 2)", "(1, 1, TILE_M, TILE_D)", 2usize)
    } else {
        ("(TILE_M, TILE_D)", "(TILE_D, TILE_N)", "(1, 0)", "(TILE_M, TILE_D)", 0usize)
    };
    let val_shape = if has_batch { "(1, 1, TILE_N, TILE_D)" } else { "(TILE_N, TILE_D)" };

    // ── accumulator init ─────────────────────────────────────────────────────
    let mut acc_init = String::new();
    for i in 0..c.slots {
        let shape = slot_shape(i, c, &params.row_axis, &params.col_axis);
        let val   = lit_py(c.identity[i]);
        acc_init.push_str(&format!(
            "    acc_{i} = ct.full({shape}, {val}, dtype=np.float32)\n"
        ));
    }

    // ── row input load (once per block) ─────────────────────────────────────
    let row_load = format!(
        "    {row_inp}_t = ct.load({row_inp}, index={row_idx}, shape={row_shape})\
         .reshape((TILE_M, TILE_D))\n"
    );

    // ── inner loop body ──────────────────────────────────────────────────────
    let mut loop_body = String::new();

    // Load key tile and compute inner contraction → scores
    loop_body.push_str(&format!(
        "        {key_inp}_t = ct.load({key_inp}, index={key_idx}, shape={key_shape},\n"
    ));
    loop_body.push_str(&format!(
        "                      order={key_order}, latency=2).reshape((TILE_D, TILE_N))\n"
    ));
    loop_body.push_str(&format!(
        "        scores = ct.mma({row_inp}_t, {key_inp}_t, ct.full((TILE_M, TILE_N), 0.0, dtype=np.float32))\n"
    ));
    loop_body.push_str("        primary = scores * qk_scale\n");

    // Load streaming value inputs
    for (vi, vname) in val_inps.iter().enumerate() {
        loop_body.push_str(&format!(
            "        streaming_{vi} = ct.load({vname}, index={val_idx}, \
             shape={val_shape}, latency=4).reshape((TILE_N, TILE_D))\n"
        ));
    }

    // Intra-tile batch (generic from SlotKind)
    let batch = intra_tile_batch(c, "primary", &params.col_axis, row_inp)?;
    loop_body.push_str(&batch);
    loop_body.push('\n');

    // Inter-tile combine (generic from carrier.combine with CSE)
    loop_body.push_str(&emit_combine(c));
    loop_body.push('\n');

    // ── project ──────────────────────────────────────────────────────────────
    let project = emit_py(&c.project[0], 0, &HashMap::new());

    // ── function signature ────────────────────────────────────────────────────
    let inputs_sig = params.input_names.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ");
    let h_param = if has_batch { "H: ConstInt,\n              " } else { "" };
    let out_name = &params.output_name;

    let src = format!(
"\
import cuda.tile as ct
import numpy as np
from cuda.tile import RoundingMode as RMd

ConstInt = ct.Constant[int]


@ct.kernel(occupancy=2)
def {name}({inputs_sig}, {out_name},
              qk_scale: float,
              {h_param}TILE_D: ConstInt,
              TILE_M: ConstInt,
              TILE_N: ConstInt):
    \"\"\"Sanic-generated kernel — streams over `{stream}`, grid over free axes.\"\"\"
{grid_setup}
{acc_init}
{row_load}
    # Stream over `{stream}`
    for j in range(ct.cdiv({key_inp}.shape[{k_seq_dim}], TILE_N)):
{loop_body}
    # Project (from carrier.project) and store
    result = {project}
    ct.store({out_name}, index={out_idx},
             tile=result.reshape({out_shape}).astype({out_name}.dtype))
"
    );

    Some(src)
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{carrier, engine_ir::*};

    fn attn_params(batch_axes: Vec<String>) -> KernelCodegenParams {
        KernelCodegenParams {
            name: "fused_attn".to_string(),
            stream_axis:   "k".to_string(),
            contract_axis: "d".to_string(),
            row_axis:      "sq".to_string(),
            col_axis:      "e".to_string(),
            batch_axes,
            input_names:   vec!["Q", "K", "V"],
            output_name:   "Out".to_string(),
        }
    }

    #[test]
    fn attention_carrier_emits_kernel() {
        let attn = attention(
            input("Q", &["b", "h", "sq", "d"]),
            input("K", &["b", "h", "k", "d"]),
            input("V", &["b", "h", "k", "e"]),
            "d", "k",
        );
        let c = carrier::derive(&attn, "k").unwrap();
        let src = cutile_kernel(&c, &attn_params(vec!["b".into(), "h".into()]))
            .expect("attention carrier must emit a kernel");

        assert!(src.contains("@ct.kernel"));
        assert!(src.contains("def fused_attn("));
        assert!(src.contains("streams over `k`"));

        // Two MMA calls: QK contraction and PV weighted sum
        assert!(src.matches("ct.mma(").count() >= 2, "expected ≥2 ct.mma calls");
        assert!(src.contains("ct.max("),  "missing row-max");
        assert!(src.contains("ct.sum("),  "missing softmax-sum");
        assert!(src.contains("ct.exp("),  "missing exp");
        assert!(src.contains("-np.inf"),  "missing -inf identity");
        assert!(src.contains("acc_2 / acc_1"), "missing project");

        // CSE: the running-max update is hoisted exactly once
        assert_eq!(src.matches("_m0 = max(acc_0, batch_0)").count(), 1);
    }

    #[test]
    fn single_head_omits_batch_params() {
        let attn = attention(
            input("Q", &["sq", "d"]),
            input("K", &["k", "d"]),
            input("V", &["k", "e"]),
            "d", "k",
        );
        let c = carrier::derive(&attn, "k").unwrap();
        let src = cutile_kernel(&c, &attn_params(vec![]))
            .expect("single-head must emit");

        assert!(!src.contains("H: ConstInt"), "H param should be absent");
        assert!(!src.contains("batch_idx"),   "batch_idx should be absent");
    }

    #[test]
    fn affine_scan_carrier_returns_none() {
        use crate::engine_ir::ssm_scan;
        let params_node = input("AB", &["t"]);
        let ssm = ssm_scan(params_node, "t");
        let c = carrier::derive(&ssm, "t").unwrap();
        let params = KernelCodegenParams {
            name: "ssm".to_string(),
            stream_axis:   "t".to_string(),
            contract_axis: "d".to_string(),
            row_axis:      "b".to_string(),
            col_axis:      String::new(),
            batch_axes:    vec![],
            input_names:   vec!["AB"],
            output_name:   "Out".to_string(),
        };
        assert!(
            cutile_kernel(&c, &params).is_none(),
            "AffineStep slots must not emit CuTile"
        );
    }

    #[test]
    fn sum_without_contraction_returns_none() {
        let x = input("X", &["n"]);
        let s = reduce(x, "n", crate::op::BinOp::Monoid(crate::op::Monoid::Add));
        let c = carrier::derive(&s, "n").unwrap();
        let params = KernelCodegenParams {
            name: "sum_kernel".to_string(),
            stream_axis:   "n".to_string(),
            contract_axis: String::new(), // no inner contraction
            row_axis:      String::new(),
            col_axis:      String::new(),
            batch_axes:    vec![],
            input_names:   vec!["X"],
            output_name:   "Out".to_string(),
        };
        assert!(cutile_kernel(&c, &params).is_none());
    }
}

#[cfg(test)]
mod print_test {
    #[test]
    #[ignore]
    fn print_generated_kernel() {
        use super::*;
        use crate::{carrier, engine_ir::*};
        let attn = attention(
            input("Q", &["b", "h", "sq", "d"]),
            input("K", &["b", "h", "k", "d"]),
            input("V", &["b", "h", "k", "e"]),
            "d", "k",
        );
        let c = carrier::derive(&attn, "k").unwrap();
        let params = KernelCodegenParams {
            name: "flash_attention".to_string(),
            stream_axis:   "k".to_string(),
            contract_axis: "d".to_string(),
            row_axis:      "sq".to_string(),
            col_axis:      "e".to_string(),
            batch_axes:    vec!["b".to_string(), "h".to_string()],
            input_names:   vec!["Q", "K", "V"],
            output_name:   "Out".to_string(),
        };
        println!("{}", cutile_kernel(&c, &params).unwrap());
    }
}
