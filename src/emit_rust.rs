//! Emit the derived kernel as compilable Rust source.
//!
//! There is nothing to invent here: the carrier already *is* the loop body —
//! `identity` seeds the accumulator, `into` lifts each element, `combine`
//! folds, `project` finishes. This is a 1:1 transcription of what
//! `Carrier::fold` executes, so the emitted kernel is exactly as correct as
//! the derivation the tests already verify. (The tests below paste the
//! emitted source back into this file, compile it, and run it against the
//! interpreter.)

use crate::derive::{Carrier, Expr};

/// Emit a CPU scalar kernel: fold the stream of elements at one grid point
/// into the answer. `grid` names the parallel axes and `streamed` the folded
/// one — recorded in the doc comment.
pub fn rust_kernel(c: &Carrier, name: &str, streamed: &str, grid: &[&str]) -> String {
    let n = item_arity(c);
    let names = Names {
        item: "x",
        acc: "acc",
        el: "el",
    };
    let join = |v: &[Expr]| {
        v.iter()
            .map(|e| rust(e, &names))
            .collect::<Vec<_>>()
            .join(",\n            ")
    };
    let identity = c
        .identity
        .iter()
        .map(|v| lit(*v))
        .collect::<Vec<_>>()
        .join(", ");

    let (ret_ty, ret) = if c.project.len() == 1 {
        ("f64".to_string(), rust(&c.project[0], &names))
    } else {
        (
            format!("[f64; {}]", c.project.len()),
            format!("[{}]", join(&c.project)),
        )
    };

    format!(
        "/// Fused streaming kernel — grid over {{{grid}}}, stream over `{streamed}`.\n\
         /// Acc = {slots} scalars; generated from the derived carrier.\n\
         pub fn {name}(elements: impl IntoIterator<Item = [f64; {n}]>) -> {ret_ty} {{\n\
         \x20   let mut acc = [{identity}];\n\
         \x20   for x in elements {{\n\
         \x20       let el = [\n            {into},\n        ];\n\
         \x20       acc = [\n            {combine},\n        ];\n\
         \x20   }}\n\
         \x20   {ret}\n\
         }}",
        grid = grid.join(", "),
        slots = c.slots,
        into = join(&c.into),
        combine = join(&c.combine),
    )
}

/// Emit a tiled kernel: `tile` lanes stay resident while the streamed axis is
/// folded once, updating all of them per step. The resident state is
/// `tile × |Acc|` scalars — exactly the SRAM term the planner sized when it
/// chose `tile`. Only `project` differs per lane at the end.
pub fn tiled_kernel(c: &Carrier, name: &str, streamed: &str, tile: usize) -> String {
    let n = item_arity(c);
    let names = Names {
        item: "x",
        acc: "a",
        el: "el",
    };
    let join = |v: &[Expr]| {
        v.iter()
            .map(|e| rust(e, &names))
            .collect::<Vec<_>>()
            .join(",\n                ")
    };
    let identity = c
        .identity
        .iter()
        .map(|v| lit(*v))
        .collect::<Vec<_>>()
        .join(", ");
    let project = rust(&c.project[0], &names); // tiled emitter targets scalar-out folds

    format!(
        "/// Tiled fused kernel — {tile} lanes resident across the `{streamed}` stream\n\
         /// ({tile} × {slots} = {resident} scalars in SRAM, the tile the planner chose).\n\
         pub fn {name}(stream: impl IntoIterator<Item = [[f64; {n}]; {tile}]>) -> [f64; {tile}] {{\n\
         \x20   const TILE: usize = {tile};\n\
         \x20   let mut acc = [[{identity}]; TILE];\n\
         \x20   for step in stream {{\n\
         \x20       for lane in 0..TILE {{\n\
         \x20           let x = step[lane];\n\
         \x20           let el = [{into}];\n\
         \x20           let a = acc[lane];\n\
         \x20           acc[lane] = [\n                {combine},\n            ];\n\
         \x20       }}\n\
         \x20   }}\n\
         \x20   let mut out = [0.0f64; TILE];\n\
         \x20   for lane in 0..TILE {{\n\
         \x20       let a = acc[lane];\n\
         \x20       out[lane] = {project};\n\
         \x20   }}\n\
         \x20   out\n\
         }}",
        slots = c.slots,
        resident = tile * c.slots,
        into = c
            .into
            .iter()
            .map(|e| rust(e, &names))
            .collect::<Vec<_>>()
            .join(", "),
        combine = join(&c.combine),
    )
}

/// Fields per streamed element (highest `Item` index + 1).
fn item_arity(c: &Carrier) -> usize {
    fn max_item(e: &Expr, acc: usize) -> usize {
        match e {
            Expr::Item(i) => acc.max(i + 1),
            Expr::Add(a, b)
            | Expr::Sub(a, b)
            | Expr::Mul(a, b)
            | Expr::Div(a, b)
            | Expr::Max(a, b)
            | Expr::Min(a, b)
            | Expr::Lt(a, b) => max_item(a, max_item(b, acc)),
            Expr::Exp(a) | Expr::Log(a) | Expr::Sqrt(a) | Expr::Sin(a) | Expr::Cos(a) => {
                max_item(a, acc)
            }
            Expr::Where(c, a, b) => max_item(c, max_item(a, max_item(b, acc))),
            _ => acc,
        }
    }
    c.into.iter().fold(0, |a, e| max_item(e, a))
}

fn lit(v: f64) -> String {
    if v == f64::NEG_INFINITY {
        "f64::NEG_INFINITY".into()
    } else if v == f64::INFINITY {
        "f64::INFINITY".into()
    } else {
        format!("{v:?}f64")
    }
}

/// The Rust variable each field role reads from. `Item` is the loaded
/// element, `A`/`F` the accumulator, `B` the lifted element.
struct Names {
    item: &'static str,
    acc: &'static str,
    el: &'static str,
}

/// Render one carrier expression as Rust, parenthesizing only where
/// precedence requires it.
fn rust(e: &Expr, n: &Names) -> String {
    render(e, 0, n)
}

fn rust_prec(e: &Expr) -> u8 {
    match e {
        Expr::Add(..) | Expr::Sub(..) => 1,
        Expr::Mul(..) | Expr::Div(..) => 2,
        _ => 3, // atoms, and postfix method calls (.max/.exp/…)
    }
}

fn render(e: &Expr, parent: u8, n: &Names) -> String {
    let p = rust_prec(e);
    // a method-call receiver must be parenthesized if it is a bare binary op
    let recv = |a: &Expr| render(a, 3, n);
    let s = match e {
        Expr::Const(v) => lit(*v),
        Expr::Item(i) => format!("{}[{i}]", n.item),
        Expr::A(i) | Expr::F(i) => format!("{}[{i}]", n.acc),
        Expr::B(i) => format!("{}[{i}]", n.el),
        // left child at this precedence; right child one tighter so `-` / `/`
        // parenthesize their right operand correctly.
        Expr::Add(a, b) => format!("{} + {}", render(a, p, n), render(b, p, n)),
        Expr::Sub(a, b) => format!("{} - {}", render(a, p, n), render(b, p + 1, n)),
        Expr::Mul(a, b) => format!("{} * {}", render(a, p, n), render(b, p, n)),
        Expr::Div(a, b) => format!("{} / {}", render(a, p, n), render(b, p + 1, n)),
        Expr::Max(a, b) => format!("{}.max({})", recv(a), render(b, 0, n)),
        Expr::Min(a, b) => format!("{}.min({})", recv(a), render(b, 0, n)),
        Expr::Exp(a) => format!("{}.exp()", recv(a)),
        Expr::Log(a) => format!("{}.ln()", recv(a)),
        Expr::Sqrt(a) => format!("{}.sqrt()", recv(a)),
        Expr::Sin(a) => format!("{}.sin()", recv(a)),
        Expr::Cos(a) => format!("{}.cos()", recv(a)),
        Expr::Lt(a, b) => format!("(({} < {}) as u8 as f64)", render(a, 0, n), render(b, 0, n)),
        // `where(a < b, …)` renders as a direct comparison, not a
        // round-trip through the 0.0/1.0 encoding.
        Expr::Where(c, a, b) => {
            let cond = match c.as_ref() {
                Expr::Lt(l, r) => format!("{} < {}", render(l, 0, n), render(r, 0, n)),
                other => format!("{} != 0.0", render(other, 0, n)),
            };
            format!(
                "(if {} {{ {} }} else {{ {} }})",
                cond,
                render(a, 0, n),
                render(b, 0, n)
            )
        }
    };
    if p < parent { format!("({s})") } else { s }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derive::derive;
    use crate::ir::*;

    // The verbatim output of `rust_kernel` for FlashAttention. That it lives
    // here and compiles is the point: the emitted kernel is real Rust. The
    // test below checks the generator still produces it *and* runs it against
    // the interpreter.
    #[rustfmt::skip]
    fn flash_attention(elements: impl IntoIterator<Item = [f64; 2]>) -> f64 {
        let mut acc = [f64::NEG_INFINITY, 0.0f64, 0.0f64];
        for x in elements {
            let el = [x[0], 1.0f64, x[1]];
            acc = [
                acc[0].max(el[0]),
                acc[1] * (acc[0] - acc[0].max(el[0])).exp() + el[1] * (el[0] - acc[0].max(el[0])).exp(),
                acc[2] * (acc[0] - acc[0].max(el[0])).exp() + el[2] * (el[0] - acc[0].max(el[0])).exp(),
            ];
        }
        acc[2] / acc[1]
    }

    #[test]
    fn emitted_flash_kernel_is_real_and_correct() {
        let (sq, k, d, e) = (axis("sq", 8), axis("k", 64), axis("d", 64), axis("e", 64));
        let attn = attention(
            input("Q", &[sq, d], Dtype::F32),
            input("K", &[k, d], Dtype::F32),
            input("V", &[k, e], Dtype::F32),
            d,
            k,
        );
        let c = derive(&attn, k).unwrap();
        let src = rust_kernel(&c, "flash_attention", "k", &["sq", "e"]);

        // the generator still emits the function pasted above (key lines)
        assert!(src.contains(
            "pub fn flash_attention(elements: impl IntoIterator<Item = [f64; 2]>) -> f64"
        ));
        assert!(src.contains("let mut acc = [f64::NEG_INFINITY, 0.0f64, 0.0f64];"));
        assert!(src.contains(
            "acc[1] * (acc[0] - acc[0].max(el[0])).exp() + el[1] * (el[0] - acc[0].max(el[0])).exp()"
        ));
        assert!(src.trim_end().ends_with("acc[2] / acc[1]\n}"));
        assert!(src.contains("stream over `k`"));

        // …and that emitted function computes what the interpreter does.
        let mut s = 0xda3e39cb94b95bdbu64;
        let mut rnd = || {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            (s >> 11) as f64 / (1u64 << 53) as f64 * 4.0 - 2.0
        };
        let items: Vec<[f64; 2]> = (0..50).map(|_| [rnd(), rnd()]).collect();
        let via_kernel = flash_attention(items.iter().copied());
        let rows: Vec<Vec<f64>> = items.iter().map(|p| p.to_vec()).collect();
        let via_interp = c.fold(&rows)[0];
        assert!((via_kernel - via_interp).abs() < 1e-12);
    }

    // The verbatim output of `tiled_kernel(.., tile = 2)`: two query lanes
    // resident across the key stream — 2 × 3 = 6 scalars in SRAM.
    #[rustfmt::skip]
    fn flash_attention_tiled(stream: impl IntoIterator<Item = [[f64; 2]; 2]>) -> [f64; 2] {
        const TILE: usize = 2;
        let mut acc = [[f64::NEG_INFINITY, 0.0f64, 0.0f64]; TILE];
        for step in stream {
            for lane in 0..TILE {
                let x = step[lane];
                let el = [x[0], 1.0f64, x[1]];
                let a = acc[lane];
                acc[lane] = [
                    a[0].max(el[0]),
                    a[1] * (a[0] - a[0].max(el[0])).exp() + el[1] * (el[0] - a[0].max(el[0])).exp(),
                    a[2] * (a[0] - a[0].max(el[0])).exp() + el[2] * (el[0] - a[0].max(el[0])).exp(),
                ];
            }
        }
        let mut out = [0.0f64; TILE];
        for lane in 0..TILE {
            let a = acc[lane];
            out[lane] = a[2] / a[1];
        }
        out
    }

    #[test]
    fn emitted_tiled_kernel_is_real_and_correct() {
        let (sq, k, d, e) = (axis("sq", 8), axis("k", 64), axis("d", 64), axis("e", 64));
        let attn = attention(
            input("Q", &[sq, d], Dtype::F32),
            input("K", &[k, d], Dtype::F32),
            input("V", &[k, e], Dtype::F32),
            d,
            k,
        );
        let c = derive(&attn, k).unwrap();
        let src = tiled_kernel(&c, "flash_attention_tiled", "k", 2);

        // the generator still emits the function pasted above (key lines)
        assert!(src.contains("const TILE: usize = 2;"));
        assert!(src.contains("let mut acc = [[f64::NEG_INFINITY, 0.0f64, 0.0f64]; TILE];"));
        assert!(src.contains("2 × 3 = 6 scalars in SRAM"));

        // two lanes with independent key streams; each equals a single fold.
        let mut s = 0x243f6a8885a308d3u64;
        let mut rnd = || {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            (s >> 11) as f64 / (1u64 << 53) as f64 * 4.0 - 2.0
        };
        let lane0: Vec<[f64; 2]> = (0..32).map(|_| [rnd(), rnd()]).collect();
        let lane1: Vec<[f64; 2]> = (0..32).map(|_| [rnd(), rnd()]).collect();
        let stream: Vec<[[f64; 2]; 2]> = (0..32).map(|i| [lane0[i], lane1[i]]).collect();
        let out = flash_attention_tiled(stream);

        let fold = |xs: &[[f64; 2]]| {
            let rows: Vec<Vec<f64>> = xs.iter().map(|p| p.to_vec()).collect();
            c.fold(&rows)[0]
        };
        assert!((out[0] - fold(&lane0)).abs() < 1e-12);
        assert!((out[1] - fold(&lane1)).abs() < 1e-12);
    }
}
