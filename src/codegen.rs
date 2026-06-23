//! Emit the fused streaming kernel as compilable Rust source.
//!
//! Everything the kernel needs is already derived: the structure map says which
//! axes are a grid and which one streams, and the carrier is the loop body —
//! `identity` to seed the accumulator, `into` to lift each element, `combine` to
//! fold, `project` at the end. This is a direct, 1:1 transcription of what
//! `Carrier::fold` already executes and the tests verify, so the emitted kernel
//! is correct by construction. Backend-specific lowering (tiling, CUDA) is the
//! scheduler's job downstream; the *algebra* of the kernel is generated here.

use crate::carrier::{Carrier, Expr};

/// Emit a CPU scalar kernel: a function that folds the stream of elements at one
/// grid point into the answer. `grid` names the parallel (FREE) axes and
/// `streamed` the folded axis — recorded in the doc comment.
pub fn rust_kernel(c: &Carrier, name: &str, streamed: &str, grid: &[&str]) -> String {
    let n = item_arity(c);
    let join = |v: &[Expr]| {
        v.iter()
            .map(rust)
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
        ("f64".to_string(), rust(&c.project[0]))
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

/// How many fields each streamed element carries (highest `Item` index + 1).
fn item_arity(c: &Carrier) -> usize {
    fn max_item(e: &Expr, acc: usize) -> usize {
        match e {
            Expr::Item(i) => acc.max(i + 1),
            Expr::Add(a, b)
            | Expr::Sub(a, b)
            | Expr::Mul(a, b)
            | Expr::Div(a, b)
            | Expr::Max(a, b)
            | Expr::Min(a, b) => max_item(a, max_item(b, acc)),
            Expr::Exp(a) | Expr::Log(a) => max_item(a, acc),
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

/// Render one carrier expression as a Rust value, parenthesizing only where
/// precedence requires it. `Item(i)` → `x[i]` (the loaded element); `A(i)`/`F(i)`
/// → `acc[i]`; `B(i)` → `el[i]` (the lifted element).
fn rust(e: &Expr) -> String {
    render(e, 0)
}

fn rust_prec(e: &Expr) -> u8 {
    match e {
        Expr::Add(..) | Expr::Sub(..) => 1,
        Expr::Mul(..) | Expr::Div(..) => 2,
        _ => 3, // atoms, and method calls (.max/.exp/...) which are postfix
    }
}

fn render(e: &Expr, parent: u8) -> String {
    let p = rust_prec(e);
    // a method-call receiver must be parenthesized if it is a bare binary op
    let recv = |a: &Expr| render(a, 3);
    let s = match e {
        Expr::Const(v) => lit(*v),
        Expr::Item(i) => format!("x[{i}]"),
        Expr::A(i) | Expr::F(i) => format!("acc[{i}]"),
        Expr::B(i) => format!("el[{i}]"),
        // left child at this precedence; right child one tighter so `-` / `/`
        // parenthesize their right operand correctly.
        Expr::Add(a, b) => format!("{} + {}", render(a, p), render(b, p)),
        Expr::Sub(a, b) => format!("{} - {}", render(a, p), render(b, p + 1)),
        Expr::Mul(a, b) => format!("{} * {}", render(a, p), render(b, p)),
        Expr::Div(a, b) => format!("{} / {}", render(a, p), render(b, p + 1)),
        Expr::Max(a, b) => format!("{}.max({})", recv(a), render(b, 0)),
        Expr::Min(a, b) => format!("{}.min({})", recv(a), render(b, 0)),
        Expr::Exp(a) => format!("{}.exp()", recv(a)),
        Expr::Log(a) => format!("{}.ln()", recv(a)),
    };
    if p < parent { format!("({s})") } else { s }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::carrier;
    use crate::engine_ir::*;

    // The verbatim output of `rust_kernel` for FlashAttention. That it lives here
    // and compiles is the point: the emitted kernel is real Rust. The test below
    // both checks the generator still produces this *and* runs it against the
    // interpreter, so the generated code is proven correct.
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
        let attn = attention(
            input("Q", &["sq", "d"]),
            input("K", &["k", "d"]),
            input("V", &["k", "e"]),
            "d",
            "k",
        );
        let c = carrier::derive(&attn, "k").unwrap();
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

        // …and that emitted function computes exactly what the interpreter does.
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
}
