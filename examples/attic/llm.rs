//! An LLM forward pass, expressed in the IR and split into kernels.
//!
//! One transformer block plus the logits head, built from the compute basis
//! and the structural `View` operator — embedding gather, RMSNorm, multi-head
//! QKV projections, scaled attention with a COMPUTED causal mask, head
//! flatten, output projection with residual, SwiGLU MLP with residual,
//! logits GEMM. `partition` splits the graph into kernels with one rule:
//! **the derive frontier is the fusion boundary**.
//!
//! What to look for in the schedule:
//!   * the causal mask is index arithmetic (`iota` + compare + `where`) fused
//!     into the flash lift — no mask tensor, no mask traffic;
//!   * ε and 1/n are literals, so the pre-attention norm fuses INTO the Q
//!     projection GEMM (Σx² rides as a second slot, the normalizer is
//!     deferred); the K/V side reads a materialized norm through a `rename`
//!     view — one copy, two index spaces;
//!   * the attention core survives as ONE fused kernel — the (m, ℓ, o)
//!     FlashAttention fold with head `h` in the grid and QKᵀ in-body;
//!   * heads are merged by a `flatten` view (free) and the output projection
//!     streams the flattened axis; residuals ride as an epilogue and inside
//!     the logits GEMM's lift; silu·gate·up fuses into the down GEMM.
//!
//!     cargo run --example llm

use std::collections::HashMap;

use sanic::cost::DeviceProfile;
use sanic::ir::*;
use sanic::partition::partition;

fn add_r() -> BinOp {
    BinOp::Monoid(Monoid::Add)
}

/// RMSNorm: x · g / √(mean(x²) + ε), with ε and 1/n as literals.
fn rmsnorm(x: Node, g: Node, n: f64, ax: Axis) -> Node {
    let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), ax, add_r());
    let mean = map(MapOp::Mul, vec![ss, konst(1.0 / n)]);
    let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
    map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom])
}

fn main() {
    let (v, s, t, dm, h, dk, dv, dmv, f) = (
        axis("v"),   // vocabulary
        axis("s"),   // query positions
        axis("t"),   // key/value positions (same buffer, viewed)
        axis("dm"),  // model dim
        axis("h"),   // heads
        axis("dk"),  // per-head query/key dim
        axis("dv"),  // per-head value dim
        axis("dmv"), // flattened h·dv
        axis("f"),   // FFN hidden dim
    );
    let dev = DeviceProfile::toy();
    let extents: HashMap<Axis, f64> = [
        (v, 32000.0),
        (s, 2048.0),
        (t, 2048.0),
        (dm, 1024.0),
        (h, 8.0),
        (dk, 128.0),
        (dv, 128.0),
        (dmv, 1024.0),
        (f, 4096.0),
    ]
    .into_iter()
    .collect();

    // ── prologue: token embedding — a data-dependent gather ─────────────────
    let embed = embedding(input("E", &[v, dm], Dtype::F32), input("ids", &[s], Dtype::F32), v);
    println!("── prologue: x = E[ids] ──\n");
    print!("{}", partition(&embed, &dev, &extents).render());

    // ── one transformer block + logits head, as a single graph ──────────────
    let x = input("X", &[s, dm], Dtype::F32);
    let g1 = input("g1", &[dm], Dtype::F32);
    let g2 = input("g2", &[dm], Dtype::F32);

    // pre-attention norm; the key/value side is a *view* of the same tensor
    // over the key position variable.
    let xn = rmsnorm(x.clone(), g1, 1024.0, dm);
    let xn_kv = rename(xn.clone(), s, t);

    // multi-head QKV projections (h rides along as a free axis)
    let q = matmul(xn, input("Wq", &[h, dk, dm], Dtype::F32), dm); // [s, h, dk]
    let k = matmul(xn_kv.clone(), input("Wk", &[h, dk, dm], Dtype::F32), dm); // [t, h, dk]
    let vv = matmul(xn_kv, input("Wv", &[h, dv, dm], Dtype::F32), dm); // [t, h, dv]

    // attention: scale is a literal, the causal mask is COMPUTED from the
    // position indices — no mask tensor anywhere.
    let scores = matmul(q, k, dk); // [s, h, t]
    let scaled = map(MapOp::Mul, vec![scores, konst(1.0 / (128.0f64).sqrt())]);
    let masked = map(MapOp::Add, vec![scaled, causal_mask(s, t)]);
    let attn = matmul(softmax(masked, t), vv, t); // [s, h, dv]

    // merge heads (free), project back to model dim, first residual
    let flat = flatten(attn, &[h, dv], dmv); // [s, dmv]
    let o = matmul(flat, input("Wo", &[dmv, dm], Dtype::F32), dmv); // [s, dm]
    let res1 = map(MapOp::Add, vec![o, x]);

    // pre-MLP norm, SwiGLU, second residual
    let hn = rmsnorm(res1.clone(), g2, 1024.0, dm);
    let gate = matmul(hn.clone(), input("Wg", &[f, dm], Dtype::F32), dm); // [s, f]
    let up = matmul(hn, input("Wu", &[f, dm], Dtype::F32), dm); // [s, f]
    let act = map(MapOp::Mul, vec![silu(gate), up]);
    let mlp = reduce(
        map(MapOp::Mul, vec![act, input("Wd", &[f, dm], Dtype::F32)]),
        f,
        add_r(),
    ); // [s, dm]
    let y = map(MapOp::Add, vec![mlp, res1]);

    // logits head
    let logits = matmul(y, input("W_lm", &[v, dm], Dtype::F32), dm); // [s, v]

    println!("\n── transformer block (multi-head, computed causal mask) + logits head ──\n");
    let sched = partition(&logits, &dev, &extents);
    print!("{}", sched.render());
    println!(
        "\n{} kernels; planned fold time ≈ {:.3} ms on the toy device",
        sched.kernel_count(),
        sched.fused_cost() * 1e3
    );
}
