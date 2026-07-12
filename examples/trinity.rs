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
use sanic::metal::{MetalBuf, MetalDevice, program_dispatches};
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
fn rms_head(x: Node, w: &'static str, j2: Axis, rr: Axis) -> Node {
    let ms = map(
        MapOp::Mul,
        vec![
            reduce(
                reduce(
                    map(MapOp::Mul, vec![x.clone(), x.clone()]),
                    rr,
                    BinOp::Monoid(Monoid::Add),
                ),
                j2,
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

/// GPT-NeoX RoPE via a computed 2×2 half-flip: `x·cos + rotate_half(x)·sin`,
/// with cos/sin synthesized from `pos` and iota over the frequency axis —
/// no tables in memory.
fn rope(x: Node, pos: Node, j2: Axis, j2p: Axis, rr: Axis) -> Node {
    let c = -(ROPE_THETA.ln()) / RR as f64; // inv_freq[r] = exp(r·c)
    let freq = map(MapOp::Exp, vec![map(MapOp::Mul, vec![iota(rr), konst(c)])]);
    let ang = map(MapOp::Mul, vec![pos, freq]); // [rr]
    let cosv = map(MapOp::Cos, vec![ang.clone()]);
    let sinv = map(MapOp::Sin, vec![ang]);
    // M[j2, j2p]: [[0, -1], [1, 0]] from two index comparisons
    let m = map(
        MapOp::Sub,
        vec![
            map(MapOp::Lt, vec![iota(j2p), iota(j2)]),
            map(MapOp::Lt, vec![iota(j2), iota(j2p)]),
        ],
    );
    let rot = reduce(
        map(MapOp::Mul, vec![m, rename(x.clone(), j2, j2p)]),
        j2p,
        BinOp::Monoid(Monoid::Add),
    );
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

/// The same W4 matvec with the weight/scale GATHERED by a routed expert
/// index — MoE's data-dependent weight selection, still one fused fold.
#[allow(clippy::too_many_arguments)]
fn w4_matvec_expert(
    x: Node,
    w: &'static str,
    s: &'static str,
    ne: Axis,
    idx: Node,
    out_axes: &[Axis],
    x_axis: Axis,
    gi: Axis,
    ri: Axis,
    flat: Axis,
) -> Node {
    let xs = split(x, x_axis, gi, ri, GROUP);
    let mut w_axes = vec![ne];
    w_axes.extend(out_axes);
    w_axes.extend([gi, ri]);
    let mut s_axes = vec![ne];
    s_axes.extend(out_axes);
    s_axes.push(gi);
    let wg = gather(input_dt(w, &w_axes, Dtype::I4), idx.clone(), ne);
    let sg = gather(input(s, &s_axes), idx, ne);
    let prod = map(
        MapOp::Mul,
        vec![map(MapOp::Mul, vec![wg, xs]), sg],
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
        let (t, hk, qg, j2, j2p, rr, rv, dq, dmv) = (
            axis("t"),
            axis("hk"),
            axis("qg"),
            axis("j2"),
            axis("j2p"),
            axis("rr"),
            axis("rv"),
            axis("dq"),
            axis("dmv"),
        );
        for (a, n) in [
            (t, T_MAX),
            (hk, HK),
            (qg, QG),
            (j2, J2),
            (j2p, J2),
            (rr, RR),
            (rv, RV),
            (dq, HD),
            (dmv, DM),
        ] {
            ext.insert(a, n);
        }

        let x = input(leak(format!("xd{l}")), &prev_axes);
        let xn = rms(x.clone(), nm("ln_in"), dm, DM);

        // ── attention: GQA as axis structure, QK-norm, RoPE-or-NoPE ──
        let q = matmul(xn.clone(), input(nm("wq"), &[hk, qg, j2, rr, dm]), dm);
        let k = matmul(xn.clone(), input(nm("wk"), &[hk, j2, rr, dm]), dm);
        let v = matmul(xn.clone(), input(nm("wv"), &[hk, rv, dm]), dm);
        let mut qn = rms_head(q, nm("qn"), j2, rr); // [hk, qg, j2, rr]
        let mut kn = rms_head(k, nm("kn"), j2, rr); // [hk, j2, rr]
        if is_sliding {
            let j2q = axis("j2q");
            ext.insert(j2q, J2);
            qn = rope(qn, pos.clone(), j2, j2p, rr);
            kn = rope(kn, pos.clone(), j2, j2q, rr);
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
            let ne = axis("ne");
            ext.insert(ne, NE);
            // Routing rounds are NAMED SCHEDULE ROOTS, not one nested
            // expression: each round's masked scores and argmax materialize
            // and the next round reads them back. Nesting them instead makes
            // every graph walker re-visit the shared chain ~3^8 times.
            let score = sigmoid(matmul(xn2.clone(), input(nm("router"), &[ne, dm]), dm));
            roots.push((score, nm("score")));
            let score_in = input(nm("score"), &[ne]);
            let cur0 = map(
                MapOp::Add,
                vec![score_in.clone(), input(nm("ebias"), &[ne])],
            );
            roots.push((cur0, leak(format!("cur0_{l}"))));
            let mut cur_in = input(leak(format!("cur0_{l}")), &[ne]);

            let mut idxs = Vec::new();
            let mut ws: Vec<Node> = Vec::new();
            for j in 0..TOPK {
                let idx = argmax(cur_in.clone(), ne);
                let idx_name = leak(format!("idx{j}_{l}"));
                roots.push((idx, idx_name));
                let idx_in = input(idx_name, &[]);
                ws.push(gather(score_in.clone(), idx_in.clone(), ne));
                if j + 1 < TOPK {
                    let nxt = map(
                        MapOp::Where,
                        vec![
                            one_hot(ne, idx_in.clone()),
                            konst(f64::NEG_INFINITY),
                            cur_in,
                        ],
                    );
                    let nname = leak(format!("cur{}_{l}", j + 1));
                    roots.push((nxt, nname));
                    cur_in = input(nname, &[ne]);
                }
                idxs.push(idx_in);
            }
            let mut wsum = konst(1e-20);
            for w in &ws {
                wsum = map(MapOp::Add, vec![wsum, w.clone()]);
            }

            let mut expert = |xin: Node, idx: Option<Node>, l: usize| -> Node {
                let nm2 = |p: &str| leak(format!("{p}_{l}"));
                let (fe, ge, re, e1, e2, e3) = (
                    axis("fe"),
                    axis("ge"),
                    axis("re"),
                    axis("e1"),
                    axis("e2"),
                    axis("e3"),
                );
                for (a, n) in
                    [(fe, FE), (ge, FE / GROUP), (re, GROUP), (e1, DM), (e2, DM), (e3, FE)]
                {
                    ext.insert(a, n);
                }
                let (g_y, u_y, d_y);
                match &idx {
                    Some(ix) => {
                        g_y = w4_matvec_expert(
                            xin.clone(), nm2("weg"), nm2("seg"), ne, ix.clone(), &[fe], dm, gi, ri, e1,
                        );
                        u_y = w4_matvec_expert(
                            xin.clone(), nm2("weu"), nm2("seu"), ne, ix.clone(), &[fe], dm, gi, ri, e2,
                        );
                        let act = map(MapOp::Mul, vec![silu(g_y), u_y]);
                        d_y = w4_matvec_expert(
                            act, nm2("wed"), nm2("sed"), ne, ix.clone(), &[dm], fe, ge, re, e3,
                        );
                    }
                    None => {
                        g_y = w4_matvec(xin.clone(), nm2("wsg"), nm2("ssg"), &[fe], dm, gi, ri, e1);
                        u_y = w4_matvec(xin.clone(), nm2("wsu"), nm2("ssu"), &[fe], dm, gi, ri, e2);
                        let act = map(MapOp::Mul, vec![silu(g_y), u_y]);
                        d_y = w4_matvec(act, nm2("wsd"), nm2("ssd"), &[dm], fe, ge, re, e3);
                    }
                }
                d_y
            };

            let mut acc = expert(xn2.clone(), None, l); // the shared expert
            for (j, idx) in idxs.iter().enumerate() {
                let coef = map(
                    MapOp::Mul,
                    vec![
                        map(MapOp::Div, vec![ws[j].clone(), wsum.clone()]),
                        konst(ROUTE_SCALE),
                    ],
                );
                let contrib = map(
                    MapOp::Mul,
                    vec![expert(xn2.clone(), Some(idx.clone()), l), coef],
                );
                acc = map(MapOp::Add, vec![acc, contrib]);
            }
            acc
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
    let expert_cat_w = |proj: &str| {
        let mut out = Vec::with_capacity(size_hint / 2);
        for e in 0..NE {
            out.extend_from_slice(st.raw(&lp(&format!("mlp.experts.{e}.{proj}.weight_packed"))));
        }
        Payload::Bytes(out)
    };
    let expert_cat_s = |proj: &str| {
        let mut out: Vec<f32> = Vec::new();
        for e in 0..NE {
            out.extend(st.f32(&lp(&format!("mlp.experts.{e}.{proj}.weight_scale"))));
        }
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
        "wsg" => Payload::Bytes(st.raw(&lp("mlp.shared_experts.gate_proj.weight_packed")).to_vec()),
        "ssg" => Payload::F32(st.f32(&lp("mlp.shared_experts.gate_proj.weight_scale"))),
        "wsu" => Payload::Bytes(st.raw(&lp("mlp.shared_experts.up_proj.weight_packed")).to_vec()),
        "ssu" => Payload::F32(st.f32(&lp("mlp.shared_experts.up_proj.weight_scale"))),
        "wsd" => Payload::Bytes(st.raw(&lp("mlp.shared_experts.down_proj.weight_packed")).to_vec()),
        "ssd" => Payload::F32(st.f32(&lp("mlp.shared_experts.down_proj.weight_scale"))),
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


    let step = |bufs: &mut HashMap<String, MetalBuf>, id: usize, pos: usize| {
        g.write_f64(&bufs["id"], &[id as f64]);
        g.write_f64(&bufs["pos"], &[pos as f64]);
        g.run(&program_dispatches(&program, bufs, &pipes));
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
