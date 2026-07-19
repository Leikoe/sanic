//! A code-generating backend that lowers a whole [`Schedule`] to compilable,
//! runnable Rust over flat tensor buffers.
//!
//! The node→code recursion, carrier rendering, and helpers live in
//! [`crate::codegen`]; this file is the Rust [`Lang`] impl plus the
//! Rust-specific wrapper — nested grid loops writing a `Vec<f64>`, and a `run`
//! driver that threads intermediates by name (the codegen analog of
//! [`Schedule::execute`]).
//!
//! Verified the honest way: `tests/rustgen.rs` emits a program, compiles it
//! with `rustc`, runs the binary, and compares to the interpreter oracle — the
//! derived flash kernel and the full transformer block both compile and match.

use std::collections::HashMap;

use crate::codegen::{Gen, Lang, buffers, carrier_expr, carrier_expr_map, offset, san, value};
use crate::derive::Carrier;
use crate::kernel_ir::{Axis, MapOp, Monoid, NodeRef as Node};
use crate::partition::{Schedule, Stage};

// ── the Rust target ──────────────────────────────────────────────────────────

struct RustLang;
const RUST: RustLang = RustLang;

impl Lang for RustLang {
    fn lit(&self, v: f64) -> String {
        if v == f64::NEG_INFINITY {
            "f64::NEG_INFINITY".into()
        } else if v == f64::INFINITY {
            "f64::INFINITY".into()
        } else {
            format!("{v:?}f64")
        }
    }
    fn iota_val(&self, ivar: &str) -> String {
        format!("({ivar} as f64)")
    }
    fn scalar_decl(&self, name: &str, init: &str) -> String {
        format!("let mut {name} = {init};")
    }
    fn for_open(&self, var: &str, count: usize) -> String {
        format!("for {var} in 0..{count} {{")
    }
    fn for_open_upto(&self, var: &str, bound: &str) -> String {
        format!("for {var} in 0..=({bound}) {{")
    }
    fn round_index(&self, name: &str, val: &str) -> String {
        format!("let {name} = ({val}).round() as usize;")
    }
    fn index_decl(&self, name: &str, val: &str, mutable: bool) -> String {
        if mutable {
            format!("let mut {name} = {val};")
        } else {
            format!("let {name} = {val};")
        }
    }
    fn signed_index_decl(&self, name: &str, val: &str) -> String {
        format!("let {name}: i64 = {val};")
    }
    fn to_signed(&self, expr: &str) -> String {
        format!("({expr} as i64)")
    }
    fn index_from_signed(&self, name: &str, val: &str) -> String {
        format!("let {name} = {val} as usize;")
    }
    fn clamped_index_decl(&self, name: &str, val: &str, n: usize) -> String {
        format!("let {name} = {val}.max(0).min({}) as usize;", n - 1)
    }
    fn select_bool(&self, cond: &str, a: &str, b: &str) -> String {
        format!("(if {cond} {{ {a} }} else {{ {b} }})")
    }
    fn map_op(&self, op: MapOp, a: &[String]) -> String {
        match op {
            MapOp::Add => format!("({} + {})", a[0], a[1]),
            MapOp::Sub => format!("({} - {})", a[0], a[1]),
            MapOp::Mul => format!("({} * {})", a[0], a[1]),
            MapOp::Div => format!("({} / {})", a[0], a[1]),
            MapOp::Max => format!("({}).max({})", a[0], a[1]),
            MapOp::Min => format!("({}).min({})", a[0], a[1]),
            MapOp::Lt => format!("(((({}) < ({})) as u8) as f64)", a[0], a[1]),
            MapOp::Neg => format!("(-({}))", a[0]),
            MapOp::Recip => format!("(1.0f64 / ({}))", a[0]),
            MapOp::Exp => format!("({}).exp()", a[0]),
            MapOp::Log => format!("({}).ln()", a[0]),
            MapOp::Sqrt => format!("({}).sqrt()", a[0]),
            MapOp::Tanh => format!("({}).tanh()", a[0]),
            MapOp::Sin => format!("({}).sin()", a[0]),
            MapOp::Cos => format!("({}).cos()", a[0]),
            MapOp::Where => format!("(if ({}) != 0.0 {{ {} }} else {{ {} }})", a[0], a[1], a[2]),
        }
    }
    fn monoid(&self, m: Monoid, acc: &str, ev: &str) -> String {
        match m {
            Monoid::Add => format!("{acc} + {ev}"),
            Monoid::Mul => format!("{acc} * {ev}"),
            Monoid::Max => format!("{acc}.max({ev})"),
            Monoid::Min => format!("{acc}.min({ev})"),
            Monoid::LogSumExp => format!(
                "{{ let a = {acc}; let b = {ev}; \
                 if a == f64::NEG_INFINITY {{ b }} else if b == f64::NEG_INFINITY {{ a }} \
                 else {{ let mm = a.max(b); mm + ((a - mm).exp() + (b - mm).exp()).ln() }} }}"
            ),
        }
    }
}

// ── kernel functions ─────────────────────────────────────────────────────────

fn params_and_args(node: &Node) -> (String, Vec<&'static str>) {
    let bufs = buffers(node);
    let params = bufs
        .iter()
        .map(|(n, _)| format!("{}: &[f64]", san(n)))
        .collect::<Vec<_>>()
        .join(", ");
    (params, bufs.into_iter().map(|(n, _)| n).collect())
}

/// Nested `for` headers opening the CPU grid, and the matching closers.
fn grid_loops(
    grid: &[Axis],
    coord: &mut HashMap<Axis, String>,
    g: &mut Gen,
) -> (Vec<String>, Vec<String>) {
    let mut open = Vec::new();
    for &a in grid {
        let iv = g.fresh("i");
        coord.insert(a, iv.clone());
        open.push(format!("for {iv} in 0..{} {{", a.extent()));
    }
    (open, grid.iter().map(|_| "}".to_string()).collect())
}

/// A fused streaming kernel: grid over the free axes, stream the folded axis,
/// lift each element from the carrier's leaves (via [`value`], so in-body
/// contractions like QKᵀ become inner loops), combine, project.
fn emit_fused(
    fname: &str,
    carrier: &Carrier,
    stream: Axis,
    fold_node: &Node,
) -> (String, Vec<&'static str>) {
    assert_eq!(
        carrier.project.len(),
        1,
        "rustgen fused kernel needs a scalar projection"
    );
    let grid = fold_node.shape();
    let grid_ext: usize = grid.iter().map(|a| a.extent()).product::<usize>().max(1);
    let (params, args) = params_and_args(fold_node);

    let mut g = Gen::new();
    let mut coord: HashMap<Axis, String> = HashMap::new();
    let (open, close) = grid_loops(&grid, &mut coord, &mut g);

    let mut src = vec![
        format!("pub fn {fname}({params}) -> Vec<f64> {{"),
        format!("    let mut outb = vec![0.0f64; {grid_ext}];"),
    ];
    src.extend(open);

    let ident = carrier
        .identity
        .iter()
        .map(|v| RUST.lit(*v))
        .collect::<Vec<_>>()
        .join(", ");
    src.push(format!("let mut acc = [{ident}];"));

    let sv = g.fresh("s");
    let mut cs = coord.clone();
    cs.insert(stream, sv.clone());
    let mut sbody = Vec::new();
    let items: Vec<String> = carrier
        .leaves
        .iter()
        .map(|l| value(&RUST, l, &cs, &mut g, &mut sbody))
        .collect();
    src.push(format!("for {sv} in 0..{} {{", stream.extent()));
    src.extend(sbody);
    src.push(format!("let x = [{}];", items.join(", ")));
    src.push(format!(
        "let el = [{}];",
        carrier
            .into
            .iter()
            .map(|e| carrier_expr(&RUST, e))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    src.push(format!(
        "acc = [{}];",
        carrier
            .combine
            .iter()
            .map(|e| carrier_expr(&RUST, e))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    src.push("}".into());

    // Leaves a projection reads are constant along the stream: render them
    // at grid scope, where the stream variable no longer exists.
    let pitems: Vec<usize> = {
        let mut set = std::collections::BTreeSet::new();
        for i in crate::derive::items_of(&carrier.project[0]) {
            set.insert(i);
        }
        set.into_iter().collect()
    };
    let mut pv: HashMap<usize, String> = HashMap::new();
    for &i in &pitems {
        let leaf = &carrier.leaves[i];
        assert!(
            !crate::kernel_ir::all_axes(leaf).contains(&stream),
            "a projection may only read stream-invariant leaves"
        );
        let e = value(&RUST, leaf, &coord, &mut g, &mut src);
        let v = g.fresh("pv");
        src.push(format!("let {v} = {e};"));
        pv.insert(i, v);
    }
    let proj = carrier_expr_map(
        &RUST,
        &carrier.project[0],
        &|i| pv[&i].clone(),
        &|i| format!("acc[{i}]"),
        &|_| unreachable!("B slot in a projection"),
    );
    src.push(format!("outb[{}] = {proj};", offset(&grid, &coord)));
    src.extend(close);
    src.push("    outb".into());
    src.push("}".into());
    (src.join("\n"), args)
}

/// A straight-line kernel (elementwise cone, gather, monoidal scan-as-reduce):
/// grid over the output axes, write [`value`] of the spliced graph.
fn emit_pointwise(fname: &str, exec: &Node) -> (String, Vec<&'static str>) {
    let grid = exec.shape();
    let grid_ext: usize = grid.iter().map(|a| a.extent()).product::<usize>().max(1);
    let (params, args) = params_and_args(exec);

    let mut g = Gen::new();
    let mut coord: HashMap<Axis, String> = HashMap::new();
    let (open, close) = grid_loops(&grid, &mut coord, &mut g);

    let mut src = vec![
        format!("pub fn {fname}({params}) -> Vec<f64> {{"),
        format!("    let mut outb = vec![0.0f64; {grid_ext}];"),
    ];
    src.extend(open);
    let mut body = Vec::new();
    let v = value(&RUST, exec, &coord, &mut g, &mut body);
    src.extend(body);
    src.push(format!("outb[{}] = {v};", offset(&grid, &coord)));
    src.extend(close);
    src.push("    outb".into());
    src.push("}".into());
    (src.join("\n"), args)
}

// ── whole-schedule program ───────────────────────────────────────────────────

/// A generated program: the kernel functions + `run` driver, and the ordered
/// list of graph inputs `run` expects.
pub struct Program {
    pub source: String,
    pub inputs: Vec<(&'static str, Vec<Axis>)>,
    /// Every schedule output, in schedule order: `run` returns the single
    /// output's buffer, or a tuple of them when there are several (a decode
    /// step's cache updates + logits).
    pub outputs: Vec<(String, Vec<Axis>)>,
    /// The last output's axes (the “final answer” of a single-output
    /// schedule).
    pub output_axes: Vec<Axis>,
}

/// Lower a whole schedule to Rust: one function per stage plus a `run` driver
/// threading intermediates by name — the codegen analog of
/// [`Schedule::execute`].
pub fn emit_schedule(sched: &Schedule) -> Program {
    let mut fns: Vec<String> = Vec::new();
    let mut driver: Vec<String> = Vec::new();
    let mut produced: Vec<&'static str> = Vec::new();
    let mut inputs: Vec<(&'static str, Vec<Axis>)> = Vec::new();
    let note_inputs =
        |node: &Node, produced: &[&'static str], inputs: &mut Vec<(&'static str, Vec<Axis>)>| {
            for (n, axes) in buffers(node) {
                if !produced.contains(&n) && !inputs.iter().any(|(m, _)| *m == n) {
                    inputs.push((n, axes));
                }
            }
        };

    let mut produced_axes: HashMap<String, Vec<Axis>> = HashMap::new();
    // every buffer is an owned `Vec<f64>`; call args are `&name[..]`, slicing a
    // `Vec` intermediate and a `&[f64]` run parameter uniformly.
    let arglist = |args: &[&'static str]| -> String {
        args.iter()
            .map(|a| format!("&{}[..]", san(a)))
            .collect::<Vec<_>>()
            .join(", ")
    };

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
                let raw_fn = format!("k_{}_fold", san(&out));
                let (code, args) =
                    emit_fused(&raw_fn, &spec.carrier, spec.streaming_axis, fold_node);
                fns.push(code);
                driver.push(format!(
                    "    let {} = {raw_fn}({});",
                    san(&out),
                    arglist(&args)
                ));
                produced.push(leak(&out));
                if let Some(epi) = epilogue_node {
                    note_inputs(epi, &produced, &mut inputs);
                    let epi_fn = format!("k_{}_epi", san(&out));
                    let (code, args) = emit_pointwise(&epi_fn, epi);
                    fns.push(code);
                    driver.push(format!(
                        "    let {} = {epi_fn}({});",
                        san(&out),
                        arglist(&args)
                    ));
                }
                produced_axes.insert(
                    out.clone(),
                    epilogue_node.as_ref().unwrap_or(fold_node).shape(),
                );
            }
            Stage::Elementwise { output, exec, .. }
            | Stage::Gather { output, exec, .. }
            | Stage::Sequential { output, exec, .. } => {
                note_inputs(exec, &produced, &mut inputs);
                let fname = format!("k_{}", san(output));
                let (code, args) = emit_pointwise(&fname, exec);
                fns.push(code);
                driver.push(format!(
                    "    let {} = {fname}({});",
                    san(output),
                    arglist(&args)
                ));
                produced.push(leak(output));
                produced_axes.insert(output.clone(), exec.shape());
            }
            Stage::Infeasible { output, .. } => {
                panic!("rustgen: cannot emit an infeasible stage producing `{output}`")
            }
        }
    }

    let outputs: Vec<(String, Vec<Axis>)> = sched
        .outputs
        .iter()
        .map(|n| {
            let axes = produced_axes
                .get(n)
                .unwrap_or_else(|| panic!("rustgen: schedule output `{n}` was never produced"))
                .clone();
            (n.clone(), axes)
        })
        .collect();

    let run_params = inputs
        .iter()
        .map(|(n, _)| format!("{}: &[f64]", san(n)))
        .collect::<Vec<_>>()
        .join(", ");
    // one output → a plain buffer; several → a tuple in schedule-output order.
    let (ret_ty, ret_val) = if outputs.len() == 1 {
        ("Vec<f64>".to_string(), san(&outputs[0].0))
    } else {
        (
            format!("({})", vec!["Vec<f64>"; outputs.len()].join(", ")),
            format!(
                "({})",
                outputs
                    .iter()
                    .map(|(n, _)| san(n))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        )
    };
    let mut run = vec![format!("pub fn run({run_params}) -> {ret_ty} {{")];
    run.extend(driver);
    run.push(format!("    {ret_val}"));
    run.push("}".into());

    let mut source =
        String::from("#![allow(unused_parens, unused_variables, dead_code, clippy::all)]\n\n");
    source.push_str(&fns.join("\n\n"));
    source.push_str("\n\n");
    source.push_str(&run.join("\n"));
    source.push('\n');

    let output_axes = outputs.last().map(|(_, a)| a.clone()).unwrap_or_default();
    Program {
        source,
        inputs,
        outputs,
        output_axes,
    }
}

fn leak(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}
