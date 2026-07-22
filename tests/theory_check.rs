//! Adversarial verification of kernel_fusion_theory.md §6 ("sliding the
//! cut"), the part that diagnoses the decode graph's 5-kernel attention.
//! Findings are recorded as the document's Appendix B.
//!
//! Claim A ("attention resolves at the first widened cut"): VERIFIED for the
//! masked decode cone — computed iota/position mask, softmax, ·V derive one
//! (m, ℓ, o) carrier whose single kernel matches the interpreter — and
//! LOCATED as a vocabulary gap for the GQA head repeat, which the bisection
//! below pins as the decline that keeps the llama graph's widest cut at the
//! score boundary (F_sem ∖ F_syn: the repeat acts on a free axis).
//!
//! Claim B ("widening the cut past a linear consumer can only shrink the
//! Nerode dimension"): FALSE as stated. It holds for post-composition, but a
//! consumer may be linear AND merge a sibling branch of the same stream —
//! convexity pulls the sibling into the widened cut, and the dimension is
//! then the product carrier's, which can GROW: Σx (dim 1) widened past the
//! linear Add against Σ i·xᵢ measures dim 2. The counterexample requires
//! mixed prefix LENGTHS in the futures matrix; fixed-length prefixes hide
//! position-carrying state behind column centering.
#![cfg(target_os = "macos")]

use sanic::derive::derive;
use sanic::emit_metal::emit_fused_metal_with;
use sanic::interp::{Env, Value, eval};
use sanic::ir::*;
use sanic::metal::{Dispatch, MetalDevice};
use sanic::nn::functional::scaled_dot_product_attention;

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

fn rand_tensor(axes: &[Axis], rng: &mut Lcg) -> Value {
    Value::from_shape_fn(
        &axes.iter().map(|axis| axis.extent()).collect::<Vec<_>>(),
        |_| rng.f(),
    )
}

// ── Claim A: the whole decode-attention cone derives and runs ────────────────

/// Bisect which ingredient of the decode cone makes `derive` decline.
#[test]
fn decode_cone_bisection() {
    let (kv_heads, cache, head_dim) = (axis("kv_heads", 2), axis("cache", 24), axis("head_dim", 8));
    let sequence = axis("sequence", 1);
    for (label, query_heads, masked) in [
        ("gqa+mask", axis("query_heads", 4), true),
        ("gqa only", axis("query_heads", 4), false),
        ("mask only", axis("query_heads", 2), true),
        ("bare", axis("query_heads", 2), false),
    ] {
        let key = input("k", [kv_heads, cache, head_dim], Dtype::F32);
        let stream = source_axis(&key, 1);
        let mask = masked.then(|| {
            let position = input("position", [], Dtype::F32);
            let visible = map(
                MapOp::Lt,
                vec![iota(cache), map(MapOp::Add, vec![position, konst(1.0)])],
            );
            map(
                MapOp::Where,
                vec![visible, konst(0.0), konst(f64::NEG_INFINITY)],
            )
        });
        let attention = scaled_dot_product_attention(
            input("q", [query_heads, sequence, head_dim], Dtype::F32),
            key,
            input("v", [kv_heads, cache, head_dim], Dtype::F32),
            mask,
            0.0,
            false,
            None,
            true,
        );
        match derive(&attention, stream) {
            Ok(carrier) => eprintln!(
                "{label:10} DERIVES: {} slots [{}]",
                carrier.kinds.len(),
                carrier.rules.join("+")
            ),
            Err(decline) => eprintln!("{label:10} declines: {decline}"),
        }
    }
}

/// §6's claim, verified for the derivable form: the masked decode cone
/// WITHOUT the GQA head repeat — computed iota/position mask, softmax, ·V —
/// derives one (m, ℓ, o) carrier and its single kernel matches the
/// interpreter. `decode_cone_bisection` pins the GQA repeat as the open
/// F_syn gap that keeps the llama graph's widest cut at the score boundary.
#[test]
fn decode_attention_cone_derives_as_one_kernel() {
    let (query_heads, kv_heads, cache, head_dim) = (
        axis("query_heads", 2),
        axis("kv_heads", 2),
        axis("cache", 24),
        axis("head_dim", 8),
    );
    let sequence = axis("sequence", 1);
    let mut rng = Lcg(0xD0C5);
    let env: Env = [
        (
            "q",
            rand_tensor(&[query_heads, sequence, head_dim], &mut rng),
        ),
        ("k", rand_tensor(&[kv_heads, cache, head_dim], &mut rng)),
        ("v", rand_tensor(&[kv_heads, cache, head_dim], &mut rng)),
        ("position", Value::from_shape_fn(&[], |_| 2.0)),
    ]
    .into_iter()
    .collect();

    let key = input("k", [kv_heads, cache, head_dim], Dtype::F32);
    let stream = source_axis(&key, 1);
    let position = input("position", [], Dtype::F32);
    let visible = map(
        MapOp::Lt,
        vec![iota(cache), map(MapOp::Add, vec![position, konst(1.0)])],
    );
    let mask = map(
        MapOp::Where,
        vec![visible, konst(0.0), konst(f64::NEG_INFINITY)],
    );
    let attention = scaled_dot_product_attention(
        input("q", [query_heads, sequence, head_dim], Dtype::F32),
        key,
        input("v", [kv_heads, cache, head_dim], Dtype::F32),
        Some(mask),
        0.0,
        false,
        None,
        true,
    );

    // the widened cut: the ENTIRE cone, exactly what §6 says to offer
    let carrier = match derive(&attention, stream) {
        Ok(carrier) => carrier,
        Err(decline) => panic!("the widened decode cone DECLINED: {decline}"),
    };
    eprintln!(
        "decode cone carrier: {} slots, rules [{}]",
        carrier.kinds.len(),
        carrier.rules.join("+")
    );

    let reference = eval(&attention, &env);
    let kernel = emit_fused_metal_with("decode_cone", &carrier, stream, &attention, None);
    let Some(device) = MetalDevice::open() else {
        eprintln!("skipping GPU check: no Metal device");
        return;
    };
    let pipelines = device.compile(&kernel.msl);
    let inputs = kernel
        .inputs
        .iter()
        .map(|(name, _)| device.from_f64(&env[name as &str].data))
        .collect();
    let output = device.alloc_f32(kernel.grid_size);
    device.run(&[Dispatch {
        pipe: pipelines.get(&kernel.name),
        inputs,
        output: output.clone(),
        grid: kernel.grid_size,
        argbuf: None,
    }]);
    let got = device.read_f32(&output, reference.data.len());
    let error = got
        .iter()
        .zip(&reference.data)
        .map(|(got, expected)| (*got as f64 - expected).abs() / (1.0 + expected.abs()))
        .fold(0.0f64, |worst, e| {
            std::cmp::max_by(worst, e, f64::total_cmp)
        });
    eprintln!("decode cone one-kernel GPU error vs interp: {error:e}");
    assert!(error < 2e-3, "one-kernel decode cone off by {error:e}");
}

// ── Claim B: rank across a widening step (harness as in completeness.rs) ─────

fn futures_matrix(h: &dyn Fn(&[f64]) -> f64, seed: u64) -> Vec<Vec<f64>> {
    let mut rng = Lcg(seed);
    let quantized = |rng: &mut Lcg| (rng.f() * 4.0).round() / 2.0;
    // VARIED prefix lengths: state that carries the position (a fold whose
    // step reads the index) is invisible to a fixed-length futures matrix —
    // every column absorbs the shared length term at centering.
    let prefixes: Vec<Vec<f64>> = (0..40)
        .map(|row| (0..4 + row % 5).map(|_| quantized(&mut rng)).collect())
        .collect();
    let suffixes: Vec<Vec<f64>> = (0..12)
        .map(|_| (0..3).map(|_| quantized(&mut rng)).collect())
        .collect();
    prefixes
        .iter()
        .map(|prefix| {
            suffixes
                .iter()
                .map(|suffix| {
                    let xs: Vec<f64> = prefix.iter().chain(suffix).copied().collect();
                    h(&xs)
                })
                .collect()
        })
        .collect()
}

/// Singular values by one-sided Jacobi, as in the completeness suite.
fn singular_values(mut a: Vec<Vec<f64>>) -> Vec<f64> {
    if a[0].len() > a.len() {
        a = (0..a[0].len())
            .map(|j| a.iter().map(|row| row[j]).collect())
            .collect();
    }
    let n = a[0].len();
    for _ in 0..60 {
        let mut off = 0.0f64;
        for p in 0..n {
            for q in p + 1..n {
                let (mut app, mut aqq, mut apq) = (0.0, 0.0, 0.0);
                for row in &a {
                    app += row[p] * row[p];
                    aqq += row[q] * row[q];
                    apq += row[p] * row[q];
                }
                let scale = (app * aqq).sqrt().max(1e-300);
                off = off.max(apq.abs() / scale);
                if apq.abs() <= 1e-15 * scale {
                    continue;
                }
                let zeta = (aqq - app) / (2.0 * apq);
                let t = zeta.signum() / (zeta.abs() + (1.0 + zeta * zeta).sqrt());
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = c * t;
                for row in a.iter_mut() {
                    let (rp, rq) = (row[p], row[q]);
                    row[p] = c * rp - s * rq;
                    row[q] = s * rp + c * rq;
                }
            }
        }
        if off < 1e-13 {
            break;
        }
    }
    let mut sv: Vec<f64> = (0..n)
        .map(|j| a.iter().map(|row| row[j] * row[j]).sum::<f64>().sqrt())
        .collect();
    sv.sort_by(|x, y| y.partial_cmp(x).unwrap());
    sv
}

fn affine_rank(h: &dyn Fn(&[f64]) -> f64, seed: u64) -> usize {
    let mut matrix = futures_matrix(h, seed);
    let rows = matrix.len() as f64;
    for j in 0..matrix[0].len() {
        let mean = matrix.iter().map(|row| row[j]).sum::<f64>() / rows;
        for row in matrix.iter_mut() {
            row[j] -= mean;
        }
    }
    let sv = singular_values(matrix);
    let top = sv[0].max(1e-300);
    sv.iter().take_while(|&&v| v / top > 1e-8).count()
}

#[test]
fn widening_past_a_linear_consumer_can_grow_the_dimension() {
    // cut A: h = Σ xᵢ — one state slot
    let narrow = |xs: &[f64]| xs.iter().sum::<f64>();
    // widened past the LINEAR consumer Add(·, Σ i·xᵢ): convexity pulls the
    // sibling branch of the same stream into the cut
    let widened = |xs: &[f64]| {
        xs.iter().sum::<f64>()
            + xs.iter()
                .enumerate()
                .map(|(i, x)| i as f64 * x)
                .sum::<f64>()
    };
    let narrow_rank = affine_rank(&narrow, 0x5EED);
    let widened_rank = affine_rank(&widened, 0x5EED);
    eprintln!("Σx rank (centered): {narrow_rank}; Add(Σx, Σi·x) rank: {widened_rank}");
    assert!(
        widened_rank > narrow_rank,
        "expected the widened cut's dimension to grow ({narrow_rank} -> {widened_rank})"
    );
}
