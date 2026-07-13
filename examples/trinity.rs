//! Trinity-Nano (AFMoE, 5.5B) from `../nanoinfer` — a real MoE LLM, W4A16
//! quantized, compiled by sanic and dispatched on the Apple GPU.
//!
//! ```text
//! cargo run --release --example trinity            # validate vs reference + stream
//! cargo run --release --example trinity -- --n 40  # cap generated tokens
//! ```
//!
//! Why this model is the stress test: 56 layers, **grouped-query attention**
//! (8 query heads sharing 2 KV heads — expressed as pure axis structure
//! `[hk, qg]`, no repeat_kv materialization), **128-expert MoE** with
//! bias-corrected sigmoid routing (top-8 = the `topk` composition; expert
//! weights fetched by `gather` over the expert axis — data-dependent weight
//! selection), QK-RMSNorm, sigmoid-gated attention output, RoPE on sliding
//! layers only (NoPE on every 4th), μP embedding scale, and **packed int4
//! weights read directly from device memory** (the typed-storage milestone:
//! nibbles unpack inside the GEMM folds, per-128-group scales fused in).
//!
//! The 3.8 GB compressed-tensors checkpoint stays packed end to end — this
//! machine (16 GB) could not hold the model any other way. Weights load
//! straight from `../nanoinfer/Trinity-Nano-Preview-W4A16/model.safetensors`;
//! logits are validated against the HF reference the nanoinfer project
//! generated (`weights/trinity_reference.json`), then tokens stream.
//!
//! Context is capped at `T_MAX` ≤ the 2048 sliding window, so sliding and
//! full attention coincide (both = causal); long-context ring buffers are
//! future work.

use std::collections::HashMap;

use sanic::cost::Device;
#[cfg(target_os = "macos")]
use sanic::metal::{MetalBuf, MetalDevice, MetalGraph, program_dispatches};
use sanic::interp::Extents;
use sanic::ir::*;
use sanic::partition::{Schedule, partition_many};
use sanic::safetensors::{Json, StFile, parse_json};

const NANOINFER: &str = "../nanoinfer";
const N_LAYER: usize = 56;
const N_DENSE: usize = 2;
const DM: usize = 1024;
const HK: usize = 2; // kv heads
const QG: usize = 4; // query heads per kv head (8 total)
const HD: usize = 128; // head dim = J2 × RR
const J2: usize = 2;
const RR: usize = 64;
const RV: usize = 128; // value head dim
const FD: usize = 3072; // dense intermediate
const FE: usize = 256; // expert intermediate
const NE: usize = 128; // experts
const TOPK: usize = 8;
const VOCAB: usize = 200192;
const T_MAX: usize = 256;
const EPS: f64 = 1e-5;
const ROPE_THETA: f64 = 10000.0;
const ROUTE_SCALE: f64 = 2.826;
const GROUP: usize = 128; // w4 quantization group size

fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

// ── the graph ────────────────────────────────────────────────────────────────

/// RMSNorm over one axis (the hidden dim): `x · rsqrt(mean(x²) + ε) · w`.
fn rms(x: Node, w: &'static str, dm: Axis, n: usize) -> Node {
    let ms = map(
        MapOp::Mul,
        vec![
            reduce(
                map(MapOp::Mul, vec![x.clone(), x.clone()]),
                dm,
                BinOp::Monoid(Monoid::Add),
            ),
            konst(1.0 / n as f64),
        ],
    );
    let inv = map(
        MapOp::Recip,
        vec![map(MapOp::Sqrt, vec![map(MapOp::Add, vec![ms, konst(EPS)])])],
    );
    map(
        MapOp::Mul,
        vec![map(MapOp::Mul, vec![x, inv]), input(w, &[dm])],
    )
}

/// Per-head QK RMSNorm: the head dim is the (j2, rr) pair; weight [j2, rr].
/// The mean-square folds the FLATTENED pair in one kernel — nested reduces
/// would cost a kernel per axis.
fn rms_head(x: Node, w: &'static str, j2: Axis, rr: Axis, hd: Axis) -> Node {
    let ms = map(
        MapOp::Mul,
        vec![
            reduce(
                flatten(map(MapOp::Mul, vec![x.clone(), x.clone()]), &[j2, rr], hd),
                hd,
                BinOp::Monoid(Monoid::Add),
            ),
            konst(1.0 / HD as f64),
        ],
    );
    let inv = map(
        MapOp::Recip,
        vec![map(MapOp::Sqrt, vec![map(MapOp::Add, vec![ms, konst(EPS)])])],
    );
    map(
        MapOp::Mul,
        vec![map(MapOp::Mul, vec![x, inv]), input(w, &[j2, rr])],
    )
}

/// GPT-NeoX RoPE via a computed half-flip: `x·cos + rotate_half(x)·sin`,
/// with cos/sin synthesized from `pos` and iota over the frequency axis —
/// no tables in memory, and NO fold: `rotate_half` is a `Reindex`
/// (`src j2 = 1 − out j2`) times a computed sign, pure index arithmetic
/// that fuses into whatever consumes it.
fn rope(x: Node, pos: Node, j2: Axis, rr: Axis) -> Node {
    let c = -(ROPE_THETA.ln()) / RR as f64; // inv_freq[r] = exp(r·c)
    let freq = map(MapOp::Exp, vec![map(MapOp::Mul, vec![iota(rr), konst(c)])]);
    let ang = map(MapOp::Mul, vec![pos, freq]); // [rr]
    let cosv = map(MapOp::Cos, vec![ang.clone()]);
    let sinv = map(MapOp::Sin, vec![ang]);
    // rotate_half = cat(−x₂, x₁): read the OTHER half, negate the FIRST —
    // sign[j2] = 2·j2 − 1 (−1 on half 0, +1 on half 1)
    let flipped = reindex(x.clone(), vec![(j2, vec![(-1, j2)], 1)], false);
    let sign = map(
        MapOp::Sub,
        vec![map(MapOp::Mul, vec![konst(2.0), iota(j2)]), konst(1.0)],
    );
    let rot = map(MapOp::Mul, vec![sign, flipped]);
    map(
        MapOp::Add,
        vec![
            map(MapOp::Mul, vec![x, cosv]),
            map(MapOp::Mul, vec![rot, sinv]),
        ],
    )
}

fn sigmoid(x: Node) -> Node {
    map(
        MapOp::Recip,
        vec![map(
            MapOp::Add,
            vec![konst(1.0), map(MapOp::Exp, vec![map(MapOp::Neg, vec![x])])],
        )],
    )
}

/// A W4A16 matvec: `y[o] = Σ_i q4[o,i]·x[i]·scale[o, i/G]`, the contraction
/// split into (group, lane) axes so the per-group scale is pure axis
/// structure, then flattened back to one streamed fold. The int4 unpack
/// happens in the kernel's load — the weight never exists dequantized.
fn w4_matvec(
    x: Node,
    w: &'static str,
    s: &'static str,
    out_axes: &[Axis],
    x_axis: Axis,
    gi: Axis,
    ri: Axis,
    flat: Axis,
) -> Node {
    let xs = split(x, x_axis, gi, ri, GROUP);
    let mut w_axes = out_axes.to_vec();
    w_axes.extend([gi, ri]);
    let mut s_axes = out_axes.to_vec();
    s_axes.push(gi);
    let prod = map(
        MapOp::Mul,
        vec![
            map(MapOp::Mul, vec![input_dt(w, &w_axes, Dtype::I4), xs]),
            input(s, &s_axes),
        ],
    );
    reduce(flatten(prod, &[gi, ri], flat), flat, BinOp::Monoid(Monoid::Add))
}

struct Model {
    sched: Schedule,
    ext: Extents,
}

fn build() -> Model {
    let (dm, vv) = (axis("dm"), axis("v"));
    let mut ext: Extents = [(dm, DM), (vv, VOCAB)].into_iter().collect();
    let pos = input("pos", &[]);

    // μP-scaled token embedding, read from an f16 table
    let tok = gather(input_dt("embed", &[vv, dm], Dtype::F16), input("id", &[]), vv);
    let x0 = map(MapOp::Mul, vec![tok, konst((DM as f64).sqrt())]);

    let mut roots: Vec<(Node, &'static str)> = vec![(x0.clone(), "xd0")];
    let mut prev_axes = output_axes(&x0);

    for l in 0..N_LAYER {
        let nm = |p: &str| leak(format!("{p}_{l}"));
        let is_sliding = l % 4 != 3;
        let (t, hk, qg, j2, rr, rv, dq, dmv, hq, hkn) = (
            axis("t"),
            axis("hk"),
            axis("qg"),
            axis("j2"),
            axis("rr"),
            axis("rv"),
            axis("dq"),
            axis("dmv"),
            axis("hq"),
            axis("hkn"),
        );
        for (a, n) in [
            (t, T_MAX),
            (hk, HK),
            (qg, QG),
            (j2, J2),
            (rr, RR),
            (rv, RV),
            (dq, HD),
            (dmv, DM),
            (hq, HD),
            (hkn, HD),
        ] {
            ext.insert(a, n);
        }

        let x = input(leak(format!("xd{l}")), &prev_axes);
        let xn = rms(x.clone(), nm("ln_in"), dm, DM);

        // ── attention: GQA as axis structure, QK-norm, RoPE-or-NoPE ──
        let q = matmul(xn.clone(), input(nm("wq"), &[hk, qg, j2, rr, dm]), dm);
        let k = matmul(xn.clone(), input(nm("wk"), &[hk, j2, rr, dm]), dm);
        let v = matmul(xn.clone(), input(nm("wv"), &[hk, rv, dm]), dm);
        let mut qn = rms_head(q, nm("qn"), j2, rr, hq); // [hk, qg, j2, rr]
        let mut kn = rms_head(k, nm("kn"), j2, rr, hkn); // [hk, j2, rr]
        if is_sliding {
            qn = rope(qn, pos.clone(), j2, rr);
            kn = rope(kn, pos.clone(), j2, rr);
        }
        let ck = map(
            MapOp::Where,
            vec![
                one_hot(t, pos.clone()),
                kn,
                input(nm("cache_k"), &[t, hk, j2, rr]),
            ],
        );
        let cv = map(
            MapOp::Where,
            vec![
                one_hot(t, pos.clone()),
                v,
                input(nm("cache_v"), &[t, hk, rv]),
            ],
        );
        roots.push((ck.clone(), nm("ckN")));
        roots.push((cv.clone(), nm("cvN")));

        let qf = flatten(qn, &[j2, rr], dq); // [hk, qg, dq]
        let ckf = flatten(ck, &[j2, rr], dq); // [t, hk, dq]
        let scores = map(
            MapOp::Mul,
            vec![matmul(qf, ckf, dq), konst(1.0 / (HD as f64).sqrt())],
        ); // [hk, qg, t]
        let future = map(MapOp::Lt, vec![pos.clone(), iota(t)]);
        let masked = map(
            MapOp::Add,
            vec![
                scores,
                map(MapOp::Where, vec![future, konst(-1e30), konst(0.0)]),
            ],
        );
        let ctx = matmul(softmax(masked, t), cv, t); // [hk, qg, rv]
        let gate = matmul(xn, input(nm("wg"), &[hk, qg, rv, dm]), dm);
        let gated = map(MapOp::Mul, vec![ctx, sigmoid(gate)]);
        let flat = flatten(gated, &[hk, qg, rv], dmv);
        let attn_out = matmul(flat, input(nm("wo"), &[dm, dmv]), dmv); // [dm]
        let x1 = map(MapOp::Add, vec![x, rms(attn_out, nm("ln_pa"), dm, DM)]);

        // ── MLP: dense SwiGLU (l<2) or 128-expert MoE, all weights int4 ──
        let xn2 = rms(x1.clone(), nm("ln_pm"), dm, DM);
        let (gi, ri) = (axis("gi"), axis("ri"));
        ext.insert(gi, DM / GROUP);
        ext.insert(ri, GROUP);

        let mlp_out = if l < N_DENSE {
            let (fd, gd, rd, c1, c2, c3) = (
                axis("fd"),
                axis("gd"),
                axis("rd"),
                axis("c1"),
                axis("c2"),
                axis("c3"),
            );
            for (a, n) in [(fd, FD), (gd, FD / GROUP), (rd, GROUP), (c1, DM), (c2, DM), (c3, FD)]
            {
                ext.insert(a, n);
            }
            let gate_y = w4_matvec(xn2.clone(), nm("wgate"), nm("sgate"), &[fd], dm, gi, ri, c1);
            let up_y = w4_matvec(xn2.clone(), nm("wup"), nm("sup"), &[fd], dm, gi, ri, c2);
            let act = map(MapOp::Mul, vec![silu(gate_y), up_y]);
            w4_matvec(act, nm("wdown"), nm("sdown"), &[dm], fd, gd, rd, c3)
        } else {
            // ── MoE as a router + grouped GEMMs ──────────────────────────
            // The 8 routed experts AND the shared expert are one `slot` axis
            // (extent 9); the stacked weights carry the shared expert as
            // index 128, and ONE vector-indexed gather selects all nine
            // weight sets. Three grouped folds do all experts at once.
            let ne = axis("ne");
            ext.insert(ne, NE + 1); // experts 0..127 + the shared expert
            let router_in = input(nm("router"), &[axis("nr"), dm]);
            let nr = output_axes(&router_in)[0];
            ext.insert(nr, NE);
            let score = sigmoid(matmul(xn2.clone(), router_in, dm));
            roots.push((score, nm("score")));
            let score_in = input(nm("score"), &[nr]);
            // Selection is on bias-corrected scores; the k-best tuple monoid
            // makes every rank ONE independent fold over them — no
            // mask-the-winner chain, no per-round materialized cuts. The
            // route WEIGHTS re-gather the raw sigmoid scores below.
            let biased = map(
                MapOp::Add,
                vec![score_in.clone(), input(nm("ebias"), &[nr])],
            );

            let mut idxs = Vec::new();
            let mut ws: Vec<Node> = Vec::new();
            for j in 0..TOPK {
                let idx = reduce(
                    biased.clone(),
                    nr,
                    BinOp::TopK {
                        k: TOPK as u8,
                        rank: j as u8,
                        idx: true,
                    },
                );
                let idx_name = leak(format!("idx{j}_{l}"));
                roots.push((idx, idx_name));
                let idx_in = input(idx_name, &[]);
                ws.push(gather(score_in.clone(), idx_in.clone(), nr));
                idxs.push(idx_in);
            }
            let mut wsum = konst(1e-20);
            for w in &ws {
                wsum = map(MapOp::Add, vec![wsum, w.clone()]);
            }

            // slot vectors: idx_all = [topk indices…, 128 (shared)],
            // coef = [route-normalized-and-scaled weights…, 1.0]
            let sl = axis("sl");
            ext.insert(sl, TOPK + 1);
            let mut idx_all = map(
                MapOp::Mul,
                vec![one_hot(sl, konst(TOPK as f64)), konst(NE as f64)],
            );
            let mut coef = one_hot(sl, konst(TOPK as f64)); // shared slot = 1.0
            for (j, idx_in) in idxs.iter().enumerate() {
                let here = one_hot(sl, konst(j as f64));
                idx_all = map(
                    MapOp::Add,
                    vec![idx_all, map(MapOp::Mul, vec![here.clone(), idx_in.clone()])],
                );
                let cj = map(
                    MapOp::Mul,
                    vec![
                        map(MapOp::Div, vec![ws[j].clone(), wsum.clone()]),
                        konst(ROUTE_SCALE),
                    ],
                );
                coef = map(MapOp::Add, vec![coef, map(MapOp::Mul, vec![here, cj])]);
            }
            roots.push((idx_all, leak(format!("idxall_{l}"))));
            roots.push((coef, leak(format!("coef_{l}"))));
            let idx_in = input(leak(format!("idxall_{l}")), &[sl]);
            let coef_in = input(leak(format!("coef_{l}")), &[sl]);

            // grouped gate/up/down: gather-by-slot over the stacked weights,
            // dequantize in-body, one fold per projection for ALL slots
            let (fe, ge, re, e1, e2, e3) = (
                axis("fe"),
                axis("ge"),
                axis("re"),
                axis("e1"),
                axis("e2"),
                axis("e3"),
            );
            for (a, n) in [
                (fe, FE),
                (ge, FE / GROUP),
                (re, GROUP),
                (e1, DM),
                (e2, DM),
                (e3, FE),
            ] {
                ext.insert(a, n);
            }
            let xs = split(xn2.clone(), dm, gi, ri, GROUP);
            let grouped = |w: &'static str, s: &'static str, out_ax: Axis, gx: Axis, rx: Axis, x: Node, fl: Axis| {
                let wsel = gather(
                    input_dt(w, &[ne, out_ax, gx, rx], Dtype::I4),
                    idx_in.clone(),
                    ne,
                );
                let ssel = gather(input(s, &[ne, out_ax, gx]), idx_in.clone(), ne);
                let prod = map(MapOp::Mul, vec![map(MapOp::Mul, vec![wsel, x]), ssel]);
                reduce(flatten(prod, &[gx, rx], fl), fl, BinOp::Monoid(Monoid::Add))
            };
            let gate_y = grouped(nm("weg"), nm("seg"), fe, gi, ri, xs.clone(), e1); // [sl, fe]
            let up_y = grouped(nm("weu"), nm("seu"), fe, gi, ri, xs.clone(), e2); // [sl, fe]
            let act = map(MapOp::Mul, vec![silu(gate_y), up_y]);
            let act_s = split(act, fe, ge, re, GROUP); // [sl, ge, re]
            let down_y = grouped(nm("wed"), nm("sed"), dm, ge, re, act_s, e3); // [sl, dm]
            // weighted combine over the slot axis — one more fold
            reduce(
                map(MapOp::Mul, vec![down_y, coef_in]),
                sl,
                BinOp::Monoid(Monoid::Add),
            )
        };

        let x2 = map(MapOp::Add, vec![x1, rms(mlp_out, nm("ln_pmlp"), dm, DM)]);
        prev_axes = output_axes(&x2);
        roots.push((x2, leak(format!("xd{}", l + 1))));
    }

    // Materialize the final norm: fused, its deferred normalizer would make
    // the 200k-vocab axis a per-slot "column" the planner prices as
    // SRAM-resident (a real cost-driven-cuts gap — the fusion is legal but
    // slower AND unplannable; the cut is better on every axis).
    let xf = rms(
        input(leak(format!("xd{N_LAYER}")), &prev_axes),
        "fnorm",
        dm,
        DM,
    );
    roots.push((xf, "xfinal"));
    let logits = matmul(
        input("xfinal", &[dm]),
        input_dt("lm_head", &[vv, dm], Dtype::F16),
        dm,
    );
    roots.push((logits, "logits"));

    let ext_f: HashMap<Axis, f64> = ext.iter().map(|(&a, &n)| (a, n as f64)).collect();
    let t0 = std::time::Instant::now();
    let sched = partition_many(&roots, &Device::toy(), &ext_f);
    println!(
        "partitioned: {} kernels in {:.1}s",
        sched.stages.len(),
        t0.elapsed().as_secs_f32()
    );
    Model { sched, ext }
}

// ── weights: name → data straight out of the checkpoint ─────────────────────

fn f16_bits(x: f32) -> u16 {
    let b = x.to_bits();
    let sign = ((b >> 16) & 0x8000) as u32;
    let exp = (((b >> 23) & 0xFF) as i32) - 127 + 15;
    let man = b & 0x7F_FFFF;
    if exp >= 31 {
        return (sign | 0x7C00) as u16;
    }
    if exp <= 0 {
        return sign as u16; // flush denormals — weights are far from 6e-5
    }
    let base = sign | ((exp as u32) << 10) | (man >> 13);
    (base + ((man >> 12) & 1)) as u16
}

enum Payload {
    F32(Vec<f32>),
    Bytes(Vec<u8>),
    ZeroF32(usize),
    PerStep,
}

/// Fetch one graph input from the checkpoint: attention/norms as f32, embed
/// and lm_head converted bf16→f16, quantized weights as their PACKED BYTES
/// (experts concatenated e-major so the gather's expert axis strides them).
fn fetch(st: &StFile, name: &str, size_hint: usize) -> Payload {
    let (base, l) = match name.rfind('_') {
        Some(i) if name[i + 1..].chars().all(|c| c.is_ascii_digit()) => {
            (&name[..i], name[i + 1..].parse::<usize>().unwrap())
        }
        _ => (name, usize::MAX),
    };
    let lp = |s: &str| format!("model.layers.{l}.{s}");
    let f16_of = |key: &str| {
        let f = st.f32(key);
        let mut out = Vec::with_capacity(f.len() * 2);
        for v in f {
            out.extend_from_slice(&f16_bits(v).to_le_bytes());
        }
        Payload::Bytes(out)
    };
    // experts 0..127 stacked e-major, the SHARED expert appended as 128 —
    // one slot axis serves routed and shared through the same gather
    let expert_cat_w = |proj: &str| {
        let mut out = Vec::with_capacity(size_hint / 2);
        for e in 0..NE {
            out.extend_from_slice(st.raw(&lp(&format!("mlp.experts.{e}.{proj}.weight_packed"))));
        }
        out.extend_from_slice(st.raw(&lp(&format!("mlp.shared_experts.{proj}.weight_packed"))));
        Payload::Bytes(out)
    };
    let expert_cat_s = |proj: &str| {
        let mut out: Vec<f32> = Vec::new();
        for e in 0..NE {
            out.extend(st.f32(&lp(&format!("mlp.experts.{e}.{proj}.weight_scale"))));
        }
        out.extend(st.f32(&lp(&format!("mlp.shared_experts.{proj}.weight_scale"))));
        Payload::F32(out)
    };

    match base {
        "id" | "pos" => Payload::PerStep,
        "embed" => f16_of("model.embed_tokens.weight"),
        "lm_head" => f16_of("lm_head.weight"),
        "fnorm" => Payload::F32(st.f32("model.norm.weight")),
        "cache_k" | "cache_v" => Payload::ZeroF32(size_hint),
        "ln_in" => Payload::F32(st.f32(&lp("input_layernorm.weight"))),
        "ln_pa" => Payload::F32(st.f32(&lp("post_attention_layernorm.weight"))),
        "ln_pm" => Payload::F32(st.f32(&lp("pre_mlp_layernorm.weight"))),
        "ln_pmlp" => Payload::F32(st.f32(&lp("post_mlp_layernorm.weight"))),
        "qn" => Payload::F32(st.f32(&lp("self_attn.q_norm.weight"))),
        "kn" => Payload::F32(st.f32(&lp("self_attn.k_norm.weight"))),
        "wq" => Payload::F32(st.f32(&lp("self_attn.q_proj.weight"))),
        "wk" => Payload::F32(st.f32(&lp("self_attn.k_proj.weight"))),
        "wv" => Payload::F32(st.f32(&lp("self_attn.v_proj.weight"))),
        "wg" => Payload::F32(st.f32(&lp("self_attn.gate_proj.weight"))),
        "wo" => Payload::F32(st.f32(&lp("self_attn.o_proj.weight"))),
        "router" => Payload::F32(st.f32(&lp("mlp.router.gate.weight"))),
        "ebias" => Payload::F32(st.f32(&lp("mlp.expert_bias"))),
        "wgate" => Payload::Bytes(st.raw(&lp("mlp.gate_proj.weight_packed")).to_vec()),
        "sgate" => Payload::F32(st.f32(&lp("mlp.gate_proj.weight_scale"))),
        "wup" => Payload::Bytes(st.raw(&lp("mlp.up_proj.weight_packed")).to_vec()),
        "sup" => Payload::F32(st.f32(&lp("mlp.up_proj.weight_scale"))),
        "wdown" => Payload::Bytes(st.raw(&lp("mlp.down_proj.weight_packed")).to_vec()),
        "sdown" => Payload::F32(st.f32(&lp("mlp.down_proj.weight_scale"))),
        "weg" => expert_cat_w("gate_proj"),
        "seg" => expert_cat_s("gate_proj"),
        "weu" => expert_cat_w("up_proj"),
        "seu" => expert_cat_s("up_proj"),
        "wed" => expert_cat_w("down_proj"),
        "sed" => expert_cat_s("down_proj"),
        other => panic!("no fetch rule for input `{other}`"),
    }
}

// ── tokenizer decode (tokenizer.json vocab, GPT-2 byte remap) ────────────────

fn load_vocab(path: &str) -> HashMap<usize, Vec<u8>> {
    let src = std::fs::read_to_string(path).expect("read tokenizer.json");
    let root = parse_json(&src).expect("parse tokenizer.json");
    let vocab = root
        .get("model")
        .and_then(|m| m.get("vocab"))
        .expect("tokenizer.json: model.vocab");
    let Json::Obj(kvs) = vocab else { panic!("vocab not an object") };
    let mut inv: HashMap<u32, u8> = HashMap::new();
    let mut n = 0u32;
    for b in 0u32..256 {
        let printable =
            (33..=126).contains(&b) || (161..=172).contains(&b) || (174..=255).contains(&b);
        if printable {
            inv.insert(b, b as u8);
        } else {
            inv.insert(256 + n, b as u8);
            n += 1;
        }
    }
    let mut out: HashMap<usize, Vec<u8>> = kvs
        .iter()
        .map(|(tok, id)| {
            let bytes: Vec<u8> = tok
                .chars()
                .map(|c| inv.get(&(c as u32)).copied().unwrap_or(b'?'))
                .collect();
            (id.as_num().unwrap() as usize, bytes)
        })
        .collect();
    if let Some(Json::Arr(added)) = root.get("added_tokens") {
        for a in added {
            let id = a.get("id").and_then(Json::as_num).unwrap() as usize;
            let content = a.get("content").and_then(Json::as_str).unwrap();
            out.insert(id, content.as_bytes().to_vec());
        }
    }
    out
}

struct StreamPrinter {
    pending: Vec<u8>,
}
impl StreamPrinter {
    fn new() -> Self {
        StreamPrinter { pending: Vec::new() }
    }
    fn push(&mut self, bytes: &[u8]) {
        use std::io::Write;
        self.pending.extend_from_slice(bytes);
        let valid = match std::str::from_utf8(&self.pending) {
            Ok(s) => s.len(),
            Err(e) => e.valid_up_to(),
        };
        if valid > 0 {
            let out = std::io::stdout();
            let mut lock = out.lock();
            lock.write_all(&self.pending[..valid]).ok();
            lock.flush().ok();
            self.pending.drain(..valid);
        }
    }
    fn finish(&mut self) {
        use std::io::Write;
        self.pending.clear();
        println!();
        std::io::stdout().flush().ok();
    }
}

// ── the GPU host ─────────────────────────────────────────────────────────────


// ── main ─────────────────────────────────────────────────────────────────────

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("the trinity example dispatches on Metal — run it on macOS");
}

#[cfg(target_os = "macos")]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let max_new: usize = args
        .iter()
        .position(|a| a == "--n")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(20);

    let model_dir = format!("{NANOINFER}/Trinity-Nano-Preview-W4A16");
    let st_path = format!("{model_dir}/model.safetensors");
    if !std::path::Path::new(&st_path).exists() {
        eprintln!("checkpoint not found at {st_path}");
        return;
    }
    let reference = std::fs::read_to_string("weights/trinity_reference.json")
        .ok()
        .map(|s| parse_json(&s).expect("parse trinity_reference.json"));
    let prompt_ids: Vec<usize> = reference
        .as_ref()
        .and_then(|r| r.get("prompt_ids"))
        .and_then(Json::as_arr)
        .map(|a| a.iter().map(|v| v.as_num().unwrap() as usize).collect())
        .unwrap_or_else(|| vec![0, 581, 4533, 323, 6364, 351]);
    let vocab = load_vocab(&format!("{NANOINFER}/Trinity-Nano-MLX-bf16/tokenizer.json"));

    println!("Trinity-Nano (afmoe): 56 layers, GQA 8/2, 128-expert MoE, W4A16");
    let model = build();
    if args.iter().any(|a| a == "--stats") {
        use sanic::partition::Stage;
        let mut folds = 0usize;
        let mut maps = 0usize;
        let mut gathers = 0usize;
        let mut seqs = 0usize;
        let mut gather_elems = 0usize;
        let mut by_tag: HashMap<&'static str, usize> = HashMap::new();
        let tag = |out: &str, by: &mut HashMap<&'static str, usize>| {
            let t = if out.starts_with("idx") || out.starts_with("cur") || out.starts_with("score") {
                "routing (topk rounds)"
            } else if out.starts_with("ck") || out.starts_with("cv") || out.starts_with("cache") {
                "kv-cache writes"
            } else if out.starts_with("xd") || out.starts_with("xf") || out == "logits" {
                "layer outputs / logits"
            } else {
                "intermediates (t*)"
            };
            *by.entry(t).or_default() += 1;
        };
        for st in &model.sched.stages {
            match st {
                Stage::Fused { spec, .. } => {
                    folds += 1;
                    tag(&spec.output_name, &mut by_tag);
                }
                Stage::Elementwise { output, .. } => {
                    maps += 1;
                    tag(output, &mut by_tag);
                }
                Stage::Gather { output, exec, .. } => {
                    gathers += 1;
                    gather_elems += output_axes(exec)
                        .iter()
                        .map(|a| model.ext[a])
                        .product::<usize>();
                    tag(output, &mut by_tag);
                }
                Stage::Sequential { output, .. } => {
                    seqs += 1;
                    tag(output, &mut by_tag);
                }
                Stage::Infeasible { .. } => {}
            }
        }
        println!(
            "stages: {} total = {folds} folds + {maps} elementwise + {gathers} gathers + {seqs} sequential",
            model.sched.stages.len()
        );
        println!("gathered/materialized elements across gather stages: {gather_elems}");
        for (k, v) in &by_tag {
            println!("  {k}: {v}");
        }
        // per-layer slice: everything between xd10 and xd11 outputs
        let mut in_layer = false;
        let mut layer_stages: Vec<String> = Vec::new();
        for st in &model.sched.stages {
            let out = match st {
                Stage::Fused { spec, .. } => spec.output_name.clone(),
                Stage::Elementwise { output, .. }
                | Stage::Gather { output, .. }
                | Stage::Sequential { output, .. } => output.clone(),
                Stage::Infeasible { output, .. } => output.clone(),
            };
            if out == "xd10" {
                in_layer = true;
                continue;
            }
            if out == "xd11" {
                layer_stages.push(out);
                break;
            }
            if in_layer {
                let kind = match st {
                    Stage::Fused { .. } => "fold",
                    Stage::Elementwise { .. } => "map",
                    Stage::Gather { .. } => "gather",
                    Stage::Sequential { .. } => "seq",
                    Stage::Infeasible { .. } => "infeasible",
                };
                layer_stages.push(format!("{kind}:{out}"));
            }
        }
        println!("one MoE layer (xd10→xd11) = {} stages:", layer_stages.len());
        println!("  {}", layer_stages.join(" "));
        return;
    }
    let t0 = std::time::Instant::now();
    let program = sanic::emit_metal::emit_schedule_metal(&model.sched, &model.ext);
    println!(
        "emitted {} kernels, {:.1} MB MSL in {:.1}s",
        program.stages.len(),
        program.msl.len() as f64 / 1e6,
        t0.elapsed().as_secs_f32()
    );

    const MSL_HEADER: &str = "#include <metal_stdlib>\nusing namespace metal;\n\n\
inline float w4(device const uchar* p, uint i) {\n\
    return (float)(int)((p[i >> 1] >> ((i & 1u) << 2)) & 0xFu) - 8.0f;\n\
}\n\n";
    let Some(g) = MetalDevice::open() else {
        eprintln!("no Metal device");
        return;
    };
    let pipes = g.compile_chunked(&program.msl, MSL_HEADER, 96, true);

    // ── upload: every input straight from the checkpoint, typed ──
    let st = StFile::open(std::path::Path::new(&st_path)).expect("open safetensors");
    let t0 = std::time::Instant::now();
    let mut bufs: HashMap<String, MetalBuf> = HashMap::new();
    let mut gpu_bytes = 0usize;
    for (name, axes) in &program.inputs {
        let size: usize = axes.iter().map(|a| model.ext[a]).product::<usize>().max(1);
        let buf = match fetch(&st, name, size) {
            Payload::PerStep => g.alloc_f32(1),
            Payload::ZeroF32(n) => {
                gpu_bytes += n * 4;
                g.alloc_f32(n)
            }
            Payload::F32(v) => {
                assert_eq!(v.len(), size, "{name}: size mismatch");
                gpu_bytes += v.len() * 4;
                g.from_f32(&v)
            }
            Payload::Bytes(b) => {
                gpu_bytes += b.len();
                g.from_bytes(&b)
            }
        };
        bufs.insert(name.to_string(), buf);
    }
    drop(st);
    for (n, size) in &program.buffers {
        gpu_bytes += *size * 4;
        bufs.insert(n.clone(), g.alloc_f32(*size));
    }
    println!(
        "uploaded {:.2} GB to the GPU in {:.1}s",
        gpu_bytes as f64 / 1e9,
        t0.elapsed().as_secs_f32()
    );


    let graphs: std::cell::RefCell<[Option<MetalGraph>; 2]> = Default::default();
    let step = |bufs: &mut HashMap<String, MetalBuf>, id: usize, pos: usize| {
        g.write_f64(&bufs["id"], &[id as f64]);
        g.write_f64(&bufs["pos"], &[pos as f64]);
        // graph replay: swap commits flip bindings with period two, so
        // one captured graph per step parity covers every step — encode
        // twice, then it's one executeCommandsInBuffer per token
        let mut gr = graphs.borrow_mut();
        let slot = &mut gr[pos % 2];
        if slot.is_none() {
            *slot = Some(g.capture(&program_dispatches(&program, bufs, &pipes)));
        }
        g.run_graph(slot.as_ref().unwrap());
        for l in 0..N_LAYER {
            for (upd, cache) in [
                (format!("ckN_{l}"), format!("cache_k_{l}")),
                (format!("cvN_{l}"), format!("cache_v_{l}")),
            ] {
                let new = bufs[&upd].clone();
                let old = bufs.insert(cache, new).unwrap();
                bufs.insert(upd, old);
            }
        }
    };

    // ── --time: warmed-up, stage-level timing of one mid layer ──
    if args.iter().any(|a| a == "--time") {
        g.write_f64(&bufs["id"], &[prompt_ids[0] as f64]);
        g.write_f64(&bufs["pos"], &[0.0]);
        let ds = program_dispatches(&program, &bufs, &pipes);
        g.run(&ds); // full warmup (pages every buffer in)
        let t0 = std::time::Instant::now();
        g.run(&ds);
        println!("warm full step: {:.0} ms", t0.elapsed().as_secs_f64() * 1e3);
        // locate layer 30's slice
        let s30 = program.stages.iter().position(|s| s.output == "xd30").unwrap() + 1;
        let e30 = program.stages.iter().position(|s| s.output == "xd31").unwrap() + 1;
        let t0 = std::time::Instant::now();
        for _ in 0..5 {
            g.run(&ds[s30..e30]);
        }
        println!(
            "layer 30 slice ({} stages): {:.2} ms warm",
            e30 - s30,
            t0.elapsed().as_secs_f64() * 1e3 / 5.0
        );
        let mut rows = Vec::new();
        for i in s30..e30 {
            let t0 = std::time::Instant::now();
            for _ in 0..3 {
                g.run(&ds[i..i + 1]);
            }
            rows.push((
                t0.elapsed().as_secs_f64() / 3.0,
                program.stages[i].kernel.clone(),
                program.stages[i].grid_size,
            ));
        }
        rows.sort_by(|a, b| b.0.total_cmp(&a.0));
        println!("stage times (incl ~sync floor), slowest first:");
        for (t, k, gsz) in rows.iter().take(20) {
            println!("  {:>8.2} ms  grid {:>8}  {}", t * 1e3, gsz, k);
        }
        return;
    }

    // ── prompt, then streaming generation ──
    let mut stream = StreamPrinter::new();
    for &id in &prompt_ids {
        stream.push(vocab.get(&id).map(|v| v.as_slice()).unwrap_or(b"?"));
    }
    // per-position sample rows from the reference: catches errors that GROW
    // with position (a RoPE or mask bug) vs flat precision noise
    let ref_sample: Vec<Vec<f64>> = reference
        .as_ref()
        .and_then(|r| r.get("logits_sample"))
        .and_then(Json::as_arr)
        .map(|rows| {
            rows.iter()
                .map(|row| {
                    row.as_arr()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_num().unwrap())
                        .collect()
                })
                .collect()
        })
        .unwrap_or_default();
    let t0 = std::time::Instant::now();
    println!();
    let mut prompt_logits = Vec::new();
    for (p, &tid) in prompt_ids.iter().enumerate() {
        step(&mut bufs, tid, p);
        prompt_logits = g.read_f32(&bufs["logits"], VOCAB);
        if let Some(refs) = ref_sample.get(p) {
            let d = refs
                .iter()
                .enumerate()
                .map(|(i, &rv)| (prompt_logits[i] as f64 - rv).abs())
                .fold(0.0f64, f64::max);
            println!("  pos {p}: max |Δlogit| over sampled row = {d:.4}");
        }
    }
    println!(
        "[prompt {} tok in {:.1}s]",
        prompt_ids.len(),
        t0.elapsed().as_secs_f32()
    );

    if let Some(r) = &reference {
        let ref_last: Vec<f64> = r
            .get("logits_last")
            .and_then(Json::as_arr)
            .unwrap()
            .iter()
            .map(|v| v.as_num().unwrap())
            .collect();
        let mut max_abs = 0.0f64;
        for (tok, &rv) in ref_last.iter().enumerate() {
            max_abs = max_abs.max((prompt_logits[tok] as f64 - rv).abs());
        }
        let our_argmax = (0..VOCAB)
            .max_by(|&a, &b| prompt_logits[a].total_cmp(&prompt_logits[b]))
            .unwrap();
        let hf_argmax = ref_last
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(b.1))
            .unwrap()
            .0;
        println!(
            "vs HF reference (bf16 compute): max |Δlogit| = {max_abs:.4}; \
             argmax {our_argmax} vs {hf_argmax} — {}",
            if our_argmax == hf_argmax { "MATCH" } else { "MISMATCH" }
        );
    }

    // stream the prompt text again as a prefix for readability
    for &id in &prompt_ids {
        stream.push(vocab.get(&id).map(|v| v.as_slice()).unwrap_or(b"?"));
    }
    let ref_gen: Vec<usize> = reference
        .as_ref()
        .and_then(|r| r.get("generated_ids"))
        .and_then(Json::as_arr)
        .map(|a| a.iter().map(|v| v.as_num().unwrap() as usize).collect())
        .unwrap_or_default();

    let mut ids = prompt_ids.clone();
    let mut cur = prompt_logits;
    let t0 = std::time::Instant::now();
    let mut generated = Vec::new();
    let mut flips = Vec::new();
    loop {
        let next = (0..VOCAB)
            .max_by(|&a, &b| cur[a].total_cmp(&cur[b]))
            .unwrap();
        // when we diverge from the reference, record OUR top-2 gap: a
        // near-tie is legitimate bf16-vs-f32 noise, a wide gap is a bug
        if let Some(&r) = ref_gen.get(generated.len())
            && r != next
        {
            let second = (0..VOCAB)
                .filter(|&i| i != next)
                .max_by(|&a, &b| cur[a].total_cmp(&cur[b]))
                .unwrap();
            flips.push((generated.len(), next, r, cur[next] - cur[second]));
        }
        ids.push(next);
        generated.push(next);
        stream.push(vocab.get(&next).map(|v| v.as_slice()).unwrap_or(b"?"));
        if generated.len() >= max_new || ids.len() >= T_MAX {
            break;
        }
        step(&mut bufs, next, ids.len() - 1);
        cur = g.read_f32(&bufs["logits"], VOCAB);
    }
    stream.finish();
    println!(
        "[{} tokens in {:.1}s — {:.0} ms/token, {:.1} tok/s]",
        generated.len(),
        t0.elapsed().as_secs_f32(),
        t0.elapsed().as_secs_f32() * 1000.0 / generated.len() as f32,
        generated.len() as f32 / t0.elapsed().as_secs_f32()
    );
    if !ref_gen.is_empty() {
        let n = generated.len().min(ref_gen.len());
        println!(
            "greedy tokens vs HF reference: {}/{} match{}",
            generated[..n]
                .iter()
                .zip(&ref_gen[..n])
                .filter(|(a, b)| a == b)
                .count(),
            n,
            if generated[..n] == ref_gen[..n] { " — SEQUENCE MATCH" } else { "" }
        );
        if let Some((at, ours, theirs, gap)) = flips.first() {
            println!(
                "first divergence at token {at}: ours {ours} vs HF {theirs}, our top-2 \
                 gap = {gap:.3} ({}) — later tokens condition on it and differ freely",
                if *gap < 1.5 { "near-tie: legitimate bf16-vs-f32 flip" } else { "DECISIVE — investigate" }
            );
        }
    }
}
