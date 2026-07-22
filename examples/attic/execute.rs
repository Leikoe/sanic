//! Lower a whole transformer block to kernels, then *execute* the schedule on
//! real tensors and check it against the naive reference — the compiler front
//! to back, producing correct numbers.
//!
//!     cargo run --example execute

use std::collections::HashMap;

use sanic::cost::DeviceProfile;
use sanic::interp::{Env, Extents, Value, eval};
use sanic::ir::*;
use sanic::partition::partition;

struct Lcg(u64);
impl Lcg {
    fn f(&mut self) -> f64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        ((x.wrapping_mul(0x2545F4914F6CDD1D) >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0
    }
}

fn add_r() -> Monoid {
    Monoid::Add
}

fn rmsnorm(x: Node, g: Node, n: f64, ax: Axis) -> Node {
    let ss = reduce(map(MapOp::Mul, vec![x.clone(), x.clone()]), ax, add_r());
    let mean = map(MapOp::Mul, vec![ss, konst(1.0 / n)]);
    let denom = map(MapOp::Sqrt, vec![map(MapOp::Add, vec![mean, konst(1e-5)])]);
    map(MapOp::Div, vec![map(MapOp::Mul, vec![x, g]), denom])
}

fn main() {
    let (v, s, t, dm, h, dk, dv, dmv, f) = (
        axis("v"),
        axis("s"),
        axis("t"),
        axis("dm"),
        axis("h"),
        axis("dk"),
        axis("dv"),
        axis("dmv"),
        axis("f"),
    );
    let ext: Extents = [
        (v, 32),
        (s, 8),
        (t, 8),
        (dm, 16),
        (h, 2),
        (dk, 8),
        (dv, 8),
        (dmv, 16),
        (f, 24),
    ]
    .into_iter()
    .collect();
    let f64ext: HashMap<Axis, f64> = ext.iter().map(|(&a, &n)| (a, n as f64)).collect();
    let n = ext[&dm] as f64;

    let mut rng = Lcg(0xADE1A1DE);
    let mut r = |axes: &[Axis]| Value::from_fn(axes, &ext, |_| rng.f());
    let env: Env = [
        ("X", r(&[s, dm])),
        ("g1", r(&[dm])),
        ("g2", r(&[dm])),
        ("Wq", r(&[h, dk, dm])),
        ("Wk", r(&[h, dk, dm])),
        ("Wv", r(&[h, dv, dm])),
        ("Wo", r(&[dmv, dm])),
        ("Wg", r(&[f, dm])),
        ("Wu", r(&[f, dm])),
        ("Wd", r(&[f, dm])),
        ("W_lm", r(&[v, dm])),
    ]
    .into_iter()
    .collect();

    // one transformer block + logits head (same graph as examples/llm.rs)
    let x = input("X", &[s, dm], Dtype::F32);
    let xn = rmsnorm(x.clone(), input("g1", &[dm], Dtype::F32), n, dm);
    let xn_kv = rename(xn.clone(), s, t);
    let q = matmul(xn, input("Wq", &[h, dk, dm], Dtype::F32), dm);
    let k = matmul(xn_kv.clone(), input("Wk", &[h, dk, dm], Dtype::F32), dm);
    let vv = matmul(xn_kv, input("Wv", &[h, dv, dm], Dtype::F32), dm);
    let scores = matmul(q, k, dk);
    let scaled = map(
        MapOp::Mul,
        vec![scores, konst(1.0 / (ext[&dk] as f64).sqrt())],
    );
    let masked = map(MapOp::Add, vec![scaled, causal_mask(s, t)]);
    let attn = matmul(softmax(masked, t), vv, t);
    let flat = flatten(attn, &[h, dv], dmv);
    let o = matmul(flat, input("Wo", &[dmv, dm], Dtype::F32), dmv);
    let res1 = map(MapOp::Add, vec![o, x]);
    let hn = rmsnorm(res1.clone(), input("g2", &[dm], Dtype::F32), n, dm);
    let gate = matmul(hn.clone(), input("Wg", &[f, dm], Dtype::F32), dm);
    let up = matmul(hn, input("Wu", &[f, dm], Dtype::F32), dm);
    let act = map(MapOp::Mul, vec![silu(gate), up]);
    let mlp = reduce(
        map(MapOp::Mul, vec![act, input("Wd", &[f, dm], Dtype::F32)]),
        f,
        add_r(),
    );
    let yb = map(MapOp::Add, vec![mlp, res1]);
    let logits = matmul(yb, input("W_lm", &[v, dm], Dtype::F32), dm);

    // lower → schedule
    let sched = partition(&logits, &DeviceProfile::toy(), &f64ext);
    println!("{}", sched.render());

    // execute the schedule, and compute the naive reference
    let out = sched.execute(&env, &ext);
    let reference = eval(&logits, &env, &ext);

    let reference = reference.permuted_to(&out.axes);
    let max_abs = out
        .data
        .iter()
        .zip(&reference.data)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f64, f64::max);

    println!(
        "\nexecuted {} kernels over a [{}×{}] logits output",
        sched.kernel_count(),
        ext[&s],
        ext[&v],
    );
    println!("max |executed − reference| = {max_abs:e}   (derived kernels, no template)");
    assert!(max_abs < 1e-9, "schedule execution diverged from reference");
    println!("✓ the lowered schedule reproduces the naive graph to machine precision");
}
