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
    let tok = gather(input_dt("embed", &[vv, dm], Dtype::BF16), input("id", &[]), vv);
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
        let q = matmul(xn.clone(), input_dt(nm("wq"), &[hk, qg, j2, rr, dm], Dtype::BF16), dm);
        let k = matmul(xn.clone(), input_dt(nm("wk"), &[hk, j2, rr, dm], Dtype::BF16), dm);
        let v = matmul(xn.clone(), input_dt(nm("wv"), &[hk, rv, dm], Dtype::BF16), dm);
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
        let gate = matmul(xn, input_dt(nm("wg"), &[hk, qg, rv, dm], Dtype::BF16), dm);
        let gated = map(MapOp::Mul, vec![ctx, sigmoid(gate)]);
        let flat = flatten(gated, &[hk, qg, rv], dmv);
        let attn_out = matmul(flat, input_dt(nm("wo"), &[dm, dmv], Dtype::BF16), dmv); // [dm]
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
            // Selection is on bias-corrected scores; ALL EIGHT ranks are ONE
            // fold (`topk_all`): the k-best slots are shared across the rank
            // queries and the rank one-hot is read at project time, so the
            // eight index kernels this layer used to launch are one kernel
            // over the rank-axis grid. The route WEIGHTS re-gather the raw
            // sigmoid scores below.
            let biased = map(
                MapOp::Add,
                vec![score_in.clone(), input(nm("ebias"), &[nr])],
            );
            let rk = axis("rk");
            ext.insert(rk, TOPK);
            roots.push((topk_all(biased, nr, TOPK, rk, true), nm("ranks")));
            let ranks_in = input(nm("ranks"), &[rk]);

            let mut idxs = Vec::new();
            let mut ws: Vec<Node> = Vec::new();
            for j in 0..TOPK {
                let idx_in = gather(ranks_in.clone(), konst(j as f64), rk);
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

    // The final norm FUSED into the 200k-vocab head is legal but
    // unplannable (its deferred normalizer prices a per-slot column as
    // SRAM-resident) — the partitioner now places that cut itself: the
    // plan-failure retry cuts the normalizer's Div, the norm becomes its
    // own stages, and the head re-derives as a plain GEMV. This root used
    // to carry the cut by hand.
    let logits = matmul(
        rms(
            input(leak(format!("xd{N_LAYER}")), &prev_axes),
            "fnorm",
            dm,
            DM,
        ),
        input_dt("lm_head", &[vv, dm], Dtype::BF16),
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
        "embed" => Payload::Bytes(st.raw("model.embed_tokens.weight").to_vec()),
        "lm_head" => Payload::Bytes(st.raw("lm_head.weight").to_vec()),
        "fnorm" => Payload::F32(st.f32("model.norm.weight")),
        "cache_k" | "cache_v" => Payload::ZeroF32(size_hint),
        "ln_in" => Payload::F32(st.f32(&lp("input_layernorm.weight"))),
        "ln_pa" => Payload::F32(st.f32(&lp("post_attention_layernorm.weight"))),
        "ln_pm" => Payload::F32(st.f32(&lp("pre_mlp_layernorm.weight"))),
        "ln_pmlp" => Payload::F32(st.f32(&lp("post_mlp_layernorm.weight"))),
        "qn" => Payload::F32(st.f32(&lp("self_attn.q_norm.weight"))),
        "kn" => Payload::F32(st.f32(&lp("self_attn.k_norm.weight"))),
        // the checkpoint stores these bf16; f16 on device halves their
        // bytes/step (bf16→f16 is exact in f16's normal range)
        "wq" => Payload::Bytes(st.raw(&lp("self_attn.q_proj.weight")).to_vec()),
        "wk" => Payload::Bytes(st.raw(&lp("self_attn.k_proj.weight")).to_vec()),
        "wv" => Payload::Bytes(st.raw(&lp("self_attn.v_proj.weight")).to_vec()),
        "wg" => Payload::Bytes(st.raw(&lp("self_attn.gate_proj.weight")).to_vec()),
        "wo" => Payload::Bytes(st.raw(&lp("self_attn.o_proj.weight")).to_vec()),
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
    if args.iter().any(|a| a == "--dump") {
        use sanic::partition::Stage;
        // Normalized per-stage signatures for schedule diffing across
        // compiler versions: intermediate numbers and layer suffixes masked.
        let norm = |s: &str| {
            let mut out = String::new();
            let mut chars = s.chars().peekable();
            while let Some(c) = chars.next() {
                if c.is_ascii_digit() {
                    while chars.peek().is_some_and(|d| d.is_ascii_digit()) {
                        chars.next();
                    }
                    out.push('N');
                } else {
                    out.push(c);
                }
            }
            out
        };
        for st in &model.sched.stages {
            match st {
                Stage::Fused { spec, epilogue, .. } => println!(
                    "F axis={} slots={} rules={:?} tiles={}x{}x{} eps={:?} in={:?} out={}",
                    spec.streaming_axis.label(),
                    spec.carrier.slots,
                    spec.carrier.rules,
                    spec.tile_m,
                    spec.tile_n,
                    spec.tile_c,
                    epilogue,
                    spec.input_names.iter().map(|n| norm(n)).collect::<Vec<_>>(),
                    norm(&spec.output_name),
                ),
                Stage::Elementwise { ops, inputs, output, .. } => println!(
                    "E ops={:?} in={:?} out={}",
                    ops,
                    inputs.iter().map(|n| norm(n)).collect::<Vec<_>>(),
                    norm(output),
                ),
                Stage::Gather { inputs, output, .. } => println!(
                    "G in={:?} out={}",
                    inputs.iter().map(|n| norm(n)).collect::<Vec<_>>(),
                    norm(output),
                ),
                Stage::Sequential { op, inputs, output, .. } => println!(
                    "S op={op} in={:?} out={}",
                    inputs.iter().map(|n| norm(n)).collect::<Vec<_>>(),
                    norm(output),
                ),
                Stage::Infeasible { output, .. } => println!("I out={}", norm(output)),
            }
        }
        return;
    }
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
    let program =
        sanic::emit_metal::emit_schedule_metal_on(&Device::m1_pro(), &model.sched, &model.ext);
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

    // ── --tune: measure the legal schedules per canonical kernel class on
    // the real buffers and overrule the analytical chooser where the
    // silicon clearly disagrees, then re-emit and recompile (dedup makes
    // the recompile ~0.2 s) ──
    let mut program = program;
    let mut pipes = pipes;
    if args.iter().any(|a| a == "--tune") {
        // a mid-window position so the honest-window kernels time on a
        // realistic live range — and ONE full pass first, so every
        // intermediate holds real values: tuned kernels read routing
        // indices, and a gather over an uninitialized buffer is an
        // out-of-bounds address, not a slow measurement
        g.write_f64(&bufs["pos"], &[(T_MAX / 2) as f64]);
        g.write_f64(&bufs["id"], &[prompt_ids[0] as f64]);
        g.run(&program_dispatches(&program, &bufs, &pipes));
        let t0 = std::time::Instant::now();
        let overrides =
            sanic::metal::tune_schedules(&g, &model.sched, &Device::m1_pro(), &model.ext, &bufs);
        println!(
            "tuned: {} stages overruled in {:.1}s",
            overrides.len(),
            t0.elapsed().as_secs_f32()
        );
        if !overrides.is_empty() {
            // end-to-end gate: the tuned program must reproduce the
            // analytic program's logits on a full step, or it is discarded
            // — per-kernel verification samples one data point; the step
            // is the oracle that counts
            let ds0 = program_dispatches(&program, &bufs, &pipes);
            g.run(&ds0);
            let l0 = g.read_f32(&bufs["logits"], VOCAB);
            let tuned = sanic::emit_metal::emit_schedule_metal_over(
                &Device::m1_pro(),
                &model.sched,
                &model.ext,
                &overrides,
            );
            let tuned_pipes = g.compile_chunked(&tuned.msl, MSL_HEADER, 96, true);
            let ds1 = program_dispatches(&tuned, &bufs, &tuned_pipes);
            g.run(&ds1);
            let l1 = g.read_f32(&bufs["logits"], VOCAB);
            let bad = l0
                .iter()
                .zip(&l1)
                .any(|(a, b)| !((a - b).abs() <= 2e-3 * (1.0 + a.abs().max(b.abs()))));
            if bad {
                eprintln!(
                    "tune: tuned program MISMATCHES the analytic program on a \
                     full step — overrides DISCARDED (emitter bug: isolate \
                     with the per-class report above)"
                );
            } else {
                program = tuned;
                pipes = tuned_pipes;
            }
        }
    }

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

    // ── --bench: GPU-timestamped per-kernel-class profile of one step ──
    // Wall clocks per dispatch drown µs kernels in the ~0.3 ms sync floor;
    // command-buffer GPU timestamps don't. Each stage is timed as R
    // back-to-back repeats in ONE command buffer (hazard tracking on the
    // shared output serializes them), then classified by which weight it
    // reads. Caveat: repeats keep a small weight hot in the SLC, so tiny
    // stages read as lower bounds — the full-step replay time is ground
    // truth, and the sum-of-stages line shows the residual.
    if args.iter().any(|a| a == "--bench") {
        g.write_f64(&bufs["id"], &[prompt_ids[0] as f64]);
        g.write_f64(&bufs["pos"], &[0.0]);
        let ds = program_dispatches(&program, &bufs, &pipes);
        g.run(&ds); // warmup: pages every weight in
        let full_enc = (0..5).map(|_| g.run_timed(&ds)).fold(f64::MAX, f64::min);
        let graph = g.capture(&ds);
        g.run_graph(&graph);
        let full_graph = (0..5)
            .map(|_| g.run_graph_timed(&graph))
            .fold(f64::MAX, f64::min);
        println!(
            "full step GPU time: {:.1} ms encoder-per-dispatch, {:.1} ms graph replay",
            full_enc * 1e3,
            full_graph * 1e3
        );

        // classify a stage by the weight it reads (falling back to output)
        let classify = |st: &sanic::emit_metal::MetalStageInfo| -> &'static str {
            let has = |p: &str| st.inputs.iter().any(|n| n.starts_with(p));
            if has("lm_head") {
                "lm_head matvec f16 [1024->200192]"
            } else if st.inputs.iter().any(|n| n == "embed") {
                "embed gather"
            } else if has("fnorm") {
                "final norm"
            } else if has("wq_") {
                "attn q proj f32 [1024->1024] (rms fused)"
            } else if has("wk_") {
                "attn k proj f32 [1024->256] (rms fused)"
            } else if has("wv_") {
                "attn v proj f32 [1024->256] (rms fused)"
            } else if has("wg_") {
                "attn gate proj f32 [1024->1024] (rms fused)"
            } else if has("wo_") {
                "attn o proj f32 [1024->1024]"
            } else if has("weg_") {
                "moe gate grouped int4 9x[1024->256]"
            } else if has("weu_") {
                "moe up grouped int4 9x[1024->256]"
            } else if has("wed_") {
                "moe down grouped int4 9x[256->1024]"
            } else if has("wgate_") || has("wup_") {
                "dense gate/up int4 [1024->3072]"
            } else if has("wdown_") {
                "dense down int4 [3072->1024]"
            } else if has("router_") {
                "router score fold [1024->128]"
            } else if st.output.starts_with("idxall") || st.output.starts_with("coef") {
                "route combine maps"
            } else if st.output.starts_with("idx") {
                "topk rank folds (8/layer)"
            } else if st.output.starts_with("ckN") || st.output.starts_with("cvN") {
                "kv cache write [256 rows]"
            } else if has("ckN_") || has("cvN_") || has("cache_k") || has("cache_v") {
                "attention core (flash fold, T=256)"
            } else if st.output.starts_with("xd") {
                "residual/epilogue maps"
            } else {
                "other elementwise"
            }
        };
        // bytes a stage touches; expert stacks and embed are gather-indexed,
        // so count only the rows actually read
        let stage_bytes = |st: &sanic::emit_metal::MetalStageInfo| -> f64 {
            let mut b = bufs[&st.output].byte_len() as f64;
            for n in &st.inputs {
                let full = bufs[n].byte_len() as f64;
                b += if n.starts_with("we") || n.starts_with("se") {
                    full * (TOPK + 1) as f64 / (NE + 1) as f64
                } else if n == "embed" {
                    (DM * 2) as f64
                } else {
                    full
                };
            }
            b
        };

        const R: usize = 16;
        let mut per_class: HashMap<&'static str, (usize, f64, f64)> = HashMap::new();
        let mut rows: Vec<(f64, usize, String, &'static str)> = Vec::new();
        for (i, st) in program.stages.iter().enumerate() {
            let reps: Vec<_> = (0..R).map(|_| ds[i].clone()).collect();
            let t = g.run_timed(&reps) / R as f64;
            let cls = classify(st);
            let e = per_class.entry(cls).or_default();
            e.0 += 1;
            e.1 += t;
            e.2 += stage_bytes(st);
            rows.push((t, st.grid_size, st.kernel.clone(), cls));
        }
        let total: f64 = per_class.values().map(|v| v.1).sum();
        println!(
            "sum of per-stage times: {:.1} ms (vs {:.1} ms replayed step)",
            total * 1e3,
            full_graph * 1e3
        );
        let mut classes: Vec<_> = per_class.into_iter().collect();
        classes.sort_by(|a, b| b.1.1.total_cmp(&a.1.1));
        println!("per class:  ms/step     n   MB/step    GB/s   (share)");
        for (cls, (n, t, bytes)) in &classes {
            println!(
                "  {:>9.2}  {:>6}  {:>8.1}  {:>6.1}   {:>4.1}%  {}",
                t * 1e3,
                n,
                bytes / 1e6,
                bytes / t / 1e9,
                t / total * 100.0,
                cls
            );
        }
        rows.sort_by(|a, b| b.0.total_cmp(&a.0));
        println!("top stages:");
        for (t, grid, k, cls) in rows.iter().take(24) {
            println!("  {:>9.3} ms  grid {:>8}  {:<28} {}", t * 1e3, grid, k, cls);
        }
        // one representative kernel per class (its slowest instance), plus
        // the full MSL, for reading the generated code side by side with
        // MLX's kernels
        let mut seen: HashMap<&'static str, ()> = HashMap::new();
        println!("representative kernels (slowest of each class):");
        for (t, grid, k, cls) in &rows {
            if seen.insert(cls, ()).is_none() {
                println!("  {:<44} {:>9.3} ms  grid {:>8}  {}", cls, t * 1e3, grid, k);
            }
        }
        std::fs::write("weights/trinity_kernels.msl", &program.msl).ok();
        println!("MSL written to weights/trinity_kernels.msl");
        return;
    }

    // ── --proto: hand-scheduled variants of the three dominant kernel
    // classes, timed against the emitted kernels on the SAME buffers and
    // verified against their outputs. These are the measured blueprint for
    // the codegen schedule change (simdgroup-cooperative folds): same
    // carrier math, different thread assignment — nothing here the planner
    // doesn't already know (roles, split-merge legality).
    if args.iter().any(|a| a == "--proto") {
        const PROTO_MSL: &str = r#"
#include <metal_stdlib>
using namespace metal;

// q projection with fused RMSNorm, MLX-qmv schedule: 2 simdgroups x 4 rows,
// lanes split dm 32-way with float4 loads, x*lnw computed once per lane and
// reused across the 4 rows, simd_sum epilogue. (emitted: 1 thread = 1 row,
// scalar loads, whole 1024-fold serial per thread)
[[max_total_threads_per_threadgroup(64)]]
kernel void proto_qproj(
    device const float* x   [[buffer(0)]],
    device const float* lnw [[buffer(1)]],
    device const float* w   [[buffer(2)]],
    device float* out       [[buffer(3)]],
    uint3 tgid [[threadgroup_position_in_grid]],
    uint sgid  [[simdgroup_index_in_threadgroup]],
    uint lane  [[thread_index_in_simdgroup]]
) {
    const int row0 = tgid.x * 8 + sgid * 4;
    float acc[4] = {0.0f, 0.0f, 0.0f, 0.0f};
    float sx2 = 0.0f;
    for (int k = lane * 4; k < 1024; k += 128) {
        float4 xv = *(device const float4*)(x + k);
        float4 lv = *(device const float4*)(lnw + k);
        float4 xn = xv * lv;
        sx2 += dot(xv, xv);
        for (int r = 0; r < 4; r++) {
            float4 wv = *(device const float4*)(w + (row0 + r) * 1024 + k);
            acc[r] += dot(xn, wv);
        }
    }
    float inv = rsqrt(simd_sum(sx2) * 0.0009765625f + 1e-5f);
    for (int r = 0; r < 4; r++) {
        float v = simd_sum(acc[r]);
        if (lane == 0) out[row0 + r] = v * inv;
    }
}

// decode attention, MLX sdpa_vector schedule on sanic's buffers/layout:
// one threadgroup per (hk, qg); 32 simdgroups split the key axis, 32 lanes
// split the head dim; the score is computed ONCE per key (simd_sum) instead
// of once per rv lane; the 32 partial (m, l, o) carriers merge through
// threadgroup memory -- run_carrier_split's stage 2, fused. Streams keys
// 0..=pos instead of the whole T_MAX window (the masked tail folds the
// monoid identity, so skipping it is algebraically free).
kernel void proto_flash(
    device const float* q    [[buffer(0)]],
    device const float* ck   [[buffer(1)]],
    device const float* pos  [[buffer(2)]],
    device const float* vnew [[buffer(3)]],
    device const float* cv   [[buffer(4)]],
    device float* out        [[buffer(5)]],
    uint3 tgid [[threadgroup_position_in_grid]],
    uint sgid  [[simdgroup_index_in_threadgroup]],
    uint lane  [[thread_index_in_simdgroup]]
) {
    constexpr int BN = 32, BD = 32, PT = 4;
    threadgroup float outputs[BN * BD];
    threadgroup float max_scores[BN];
    threadgroup float sum_exp_scores[BN];
    const int hk = tgid.x / 4, qg = tgid.x % 4;
    const int P = (int)(pos[0] + 0.5f);
    float qv[PT], kv[PT], o[PT];
    device const float* qp = q + hk * 512 + qg * 128 + lane * PT;
    for (int j = 0; j < PT; j++) { qv[j] = 0.08838834764831843f * qp[j]; o[j] = 0.0f; }
    float m = -INFINITY, l = 0.0f;
    for (int i = sgid; i <= P; i += BN) {
        device const float* kp = ck + i * 256 + hk * 128 + lane * PT;
        for (int j = 0; j < PT; j++) kv[j] = kp[j];
        float s = 0.0f;
        for (int j = 0; j < PT; j++) s += qv[j] * kv[j];
        s = simd_sum(s);
        float nm = max(m, s);
        float factor = exp(m - nm), es = exp(s - nm);
        m = nm;
        l = l * factor + es;
        device const float* vp = (i == P) ? (vnew + hk * 128 + lane * PT)
                                          : (cv + i * 256 + hk * 128 + lane * PT);
        for (int j = 0; j < PT; j++) o[j] = o[j] * factor + es * vp[j];
    }
    if (lane == 0) { max_scores[sgid] = m; sum_exp_scores[sgid] = l; }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    m = max_scores[lane];
    float nm = simd_max(m);
    float factor = exp(m - nm);
    float l_all = simd_sum(sum_exp_scores[lane] * factor);
    for (int j = 0; j < PT; j++) {
        outputs[lane * BD + sgid] = o[j];
        threadgroup_barrier(mem_flags::mem_threadgroup);
        o[j] = simd_sum(outputs[sgid * BD + lane] * factor);
        threadgroup_barrier(mem_flags::mem_threadgroup);
    }
    if (lane == 0) {
        for (int j = 0; j < PT; j++)
            out[hk * 512 + qg * 128 + sgid * PT + j] = o[j] / l_all;
    }
}

// grouped MoE gate/up fold, qmv schedule for packed int4: a threadgroup
// owns 8 fe-rows of ONE slot (the expert index loads once, not per
// element); each lane pulls 8 nibbles as one uint and the group scale
// once per 128-block; x reused across the 4 rows of a simdgroup.
[[max_total_threads_per_threadgroup(64)]]
kernel void proto_moe_gate(
    device const uchar* wq  [[buffer(0)]],
    device const float* idx [[buffer(1)]],
    device const float* x   [[buffer(2)]],
    device const float* sc  [[buffer(3)]],
    device float* out       [[buffer(4)]],
    uint3 tgid [[threadgroup_position_in_grid]],
    uint sgid  [[simdgroup_index_in_threadgroup]],
    uint lane  [[thread_index_in_simdgroup]]
) {
    const int slot = tgid.x / 32;
    const int fe0 = (tgid.x % 32) * 8 + sgid * 4;
    const uint e = (uint)(idx[slot] + 0.5f);
    device const uchar* wbase = wq + e * 131072;
    device const float* sbase = sc + e * 2048;
    float acc[4] = {0.0f, 0.0f, 0.0f, 0.0f};
    for (int blk = 0; blk < 4; blk++) {
        const int k = blk * 256 + lane * 8;
        float xv[8];
        float sx = 0.0f;
        for (int j = 0; j < 8; j++) { xv[j] = x[k + j]; sx += xv[j]; }
        const int gidx = k >> 7;
        for (int r = 0; r < 4; r++) {
            const int row = fe0 + r;
            uint u = *(device const uint*)(wbase + row * 512 + (k >> 1));
            float p = 0.0f;
            for (int j = 0; j < 8; j++)
                p += (float)((u >> (4 * j)) & 0xFu) * xv[j];
            acc[r] += (p - 8.0f * sx) * sbase[row * 8 + gidx];
        }
    }
    for (int r = 0; r < 4; r++) {
        float v = simd_sum(acc[r]);
        if (lane == 0) out[(fe0 + r) * 9 + slot] = v;
    }
}
"#;
        g.write_f64(&bufs["id"], &[prompt_ids[0] as f64]);
        g.write_f64(&bufs["pos"], &[0.0]);
        let ds = program_dispatches(&program, &bufs, &pipes);
        g.run(&ds); // populate every intermediate (routing indices included)
        let proto_pipes = g.compile(PROTO_MSL);

        let stage_of = |pred: &dyn Fn(&sanic::emit_metal::MetalStageInfo) -> bool| {
            program.stages.iter().position(|s| pred(s)).expect("stage")
        };
        let qi = stage_of(&|s| {
            s.inputs.iter().any(|n| n == "wq_0") && s.kernel.ends_with("_fold")
        });
        let fi = stage_of(&|s| {
            s.inputs.iter().any(|n| n == "cache_v_1") && !s.output.starts_with("cvN")
        });
        let mi = stage_of(&|s| s.inputs.iter().any(|n| n == "weg_2"));
        println!(
            "prototype targets: qproj={} flash={} moe={}",
            program.stages[qi].kernel, program.stages[fi].kernel, program.stages[mi].kernel
        );

        let make = |kernel: &str, idx: usize, grid: usize, out_elems: usize| {
            let st = &program.stages[idx];
            sanic::metal::Dispatch {
                pipe: proto_pipes.get(kernel),
                inputs: st.inputs.iter().map(|n| bufs[n].clone()).collect(),
                output: g.alloc_f32(out_elems),
                grid,
            }
        };
        let protos = [
            ("proto_qproj", make("proto_qproj", qi, 8192, 1024), qi, 1024),
            ("proto_flash", make("proto_flash", fi, 8192, 1024), fi, 1024),
            ("proto_moe_gate", make("proto_moe_gate", mi, 18432, 2304), mi, 2304),
        ];

        // end-to-end: substitute the proto into ALL 56 flash stages (same
        // output buffers, so downstream stages are untouched) and replay
        // the whole step — checks the per-stage accounting adds up
        g.write_f64(&bufs["pos"], &[15.0]);
        let mut ds2 = program_dispatches(&program, &bufs, &pipes);
        let mut subbed = 0;
        for (i, st) in program.stages.iter().enumerate() {
            if st.inputs.iter().any(|n| n.starts_with("cache_v_"))
                && !st.output.starts_with("cvN")
            {
                ds2[i] = sanic::metal::Dispatch {
                    pipe: proto_pipes.get("proto_flash"),
                    inputs: st.inputs.iter().map(|n| bufs[n].clone()).collect(),
                    output: bufs[&st.output].clone(),
                    grid: 8192,
                };
                subbed += 1;
            }
        }
        let base = program_dispatches(&program, &bufs, &pipes);
        g.run(&base);
        let t_base = (0..5).map(|_| g.run_timed(&base)).fold(f64::MAX, f64::min);
        g.run(&ds2);
        let t_sub = (0..5).map(|_| g.run_timed(&ds2)).fold(f64::MAX, f64::min);
        let gb = g.capture(&ds2);
        g.run_graph(&gb);
        let t_sub_graph = (0..5)
            .map(|_| g.run_graph_timed(&gb))
            .fold(f64::MAX, f64::min);
        println!(
            "full step, {subbed} flash stages substituted: {:.1} ms -> {:.1} ms ({:.1} ms as graph replay)",
            t_base * 1e3,
            t_sub * 1e3,
            t_sub_graph * 1e3
        );

        const R: usize = 32;
        for pos in [0usize, 15, 255] {
            g.write_f64(&bufs["pos"], &[pos as f64]);
            println!("pos = {pos} (window = {}):", pos + 1);
            for (name, d, idx, n) in &protos {
                // verify: emitted then proto on identical inputs
                g.run(&ds[*idx..idx + 1]);
                g.run(std::slice::from_ref(d));
                let want = g.read_f32(&bufs[&program.stages[*idx].output], *n);
                let got = g.read_f32(&d.output, *n);
                let err = want
                    .iter()
                    .zip(&got)
                    .map(|(a, b)| (a - b).abs() as f64 / (a.abs() as f64).max(1.0))
                    .fold(0.0f64, f64::max);
                let te = {
                    let reps: Vec<_> = (0..R).map(|_| ds[*idx].clone()).collect();
                    g.run_timed(&reps) / R as f64
                };
                let tp = {
                    let reps: Vec<_> = (0..R).map(|_| d.clone()).collect();
                    g.run_timed(&reps) / R as f64
                };
                println!(
                    "  {name:<16} emitted {:>8.1} us -> proto {:>7.1} us  ({:>5.1}x)  max rel err {err:.2e}",
                    te * 1e6,
                    tp * 1e6,
                    te / tp
                );
            }
        }
        return;
    }

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
