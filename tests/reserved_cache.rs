//! The reserved KV-cache length must not change decode logits.
//!
//! The decode graph masks unwritten cache slots with an additive `-inf`
//! before softmax, so its semantics are independent of how many slots the
//! cache reserves beyond the visible prefix. Metal's DEFAULT fast math
//! deleted exactly that mask: the shader compiler assumes no ±inf occurs and
//! folds `score + (visible ? 0 : -INFINITY)` to `score`, so every reserved
//! slot leaked `exp(0)` into the softmax denominator and Llama's logits
//! depended on `-n`. The mask survives when bound as an input buffer — only
//! the computed literal form is vulnerable — which is why the buffer-mask
//! GPU tests never caught it. `MetalDevice::compile` now uses relaxed math.
#![cfg(target_os = "macos")]

use std::collections::HashMap;

use sanic::interp::{Env, Value, eval};
use sanic::nn::functional::scaled_dot_product_attention;
use sanic::{
    Axis, Compile, Dtype, MapOp, Monoid, NodeRef, ViewDim, axis, coordinate, flatten, gather,
    input, iota, konst, map, matmul, positional_reindex, positional_view, reduce, silu, split,
    transpose,
};

const EPS: f64 = 1e-5;
const ROPE_THETA: f64 = 500_000.0;
const ROPE_FACTOR: f64 = 32.0;
const ROPE_ORIGINAL_CONTEXT: f64 = 8_192.0;
const ROPE_LOW_FREQ_FACTOR: f64 = 1.0;
const ROPE_HIGH_FREQ_FACTOR: f64 = 4.0;

#[derive(Clone, Copy)]
struct Config {
    vocab_size: usize,
    layers: usize,
    hidden_dim: usize,
    query_heads: usize,
    kv_heads: usize,
    head_dim: usize,
    intermediate_dim: usize,
}

/// Llama 3.2's decode block at toy extents, GQA included.
const COMPACT: Config = Config {
    vocab_size: 48,
    layers: 2,
    hidden_dim: 32,
    query_heads: 4,
    kv_heads: 2,
    head_dim: 8,
    intermediate_dim: 64,
};

struct Axes {
    vocab: Axis,
    sequence: Axis,
    hidden: Axis,
    query_heads: Axis,
    kv_heads: Axis,
    head_dim: Axis,
    intermediate: Axis,
}

impl Axes {
    fn new(config: Config) -> Self {
        Self {
            vocab: axis("vocab", config.vocab_size),
            sequence: axis("sequence", 1),
            hidden: axis("hidden", config.hidden_dim),
            query_heads: axis("query_heads", config.query_heads),
            kv_heads: axis("kv_heads", config.kv_heads),
            head_dim: axis("head_dim", config.head_dim),
            intermediate: axis("intermediate", config.intermediate_dim),
        }
    }
}

fn unary(op: MapOp, x: NodeRef) -> NodeRef {
    map(op, vec![x])
}

fn binary(op: MapOp, left: NodeRef, right: NodeRef) -> NodeRef {
    map(op, vec![left, right])
}

fn add(left: NodeRef, right: NodeRef) -> NodeRef {
    binary(MapOp::Add, left, right)
}

fn sub(left: NodeRef, right: NodeRef) -> NodeRef {
    binary(MapOp::Sub, left, right)
}

fn mul(left: NodeRef, right: NodeRef) -> NodeRef {
    binary(MapOp::Mul, left, right)
}

fn div(left: NodeRef, right: NodeRef) -> NodeRef {
    binary(MapOp::Div, left, right)
}

fn unsqueeze(src: NodeRef, dim: usize) -> NodeRef {
    let source = src.shape();
    let mut dims = Vec::with_capacity(source.len() + 1);
    for output_dim in 0..=source.len() {
        if output_dim == dim {
            dims.push(ViewDim {
                sources: Vec::new(),
                axis: axis("singleton", 1),
            });
        } else {
            let source_dim = if output_dim < dim {
                output_dim
            } else {
                output_dim - 1
            };
            dims.push(ViewDim {
                sources: vec![source_dim],
                axis: source[source_dim],
            });
        }
    }
    positional_view(src, dims)
}

fn flip(src: NodeRef, dim: usize) -> NodeRef {
    let shape = src.shape();
    let map = shape
        .iter()
        .enumerate()
        .map(|(source_dim, source_axis)| {
            if source_dim == dim {
                (
                    source_dim,
                    vec![(-1, source_dim)],
                    source_axis.extent() as i64 - 1,
                )
            } else {
                (source_dim, vec![(1, source_dim)], 0)
            }
        })
        .collect();
    positional_reindex(src, shape, map, false)
}

fn rms_norm(x: NodeRef, weight: NodeRef, hidden_dim: usize) -> NodeRef {
    let square = mul(x.clone(), x.clone());
    let mean_square = mul(
        reduce(square, -1isize, Monoid::Add),
        konst(1.0 / hidden_dim as f64),
    );
    let denominator = unary(
        MapOp::Sqrt,
        add(unsqueeze(mean_square, x.shape().len() - 1), konst(EPS)),
    );
    div(mul(x, weight), denominator)
}

fn llama3_inv_freq(frequency: Axis) -> NodeRef {
    let exponent = mul(
        iota(frequency),
        konst(-ROPE_THETA.ln() / frequency.extent() as f64),
    );
    let inv_freq = unary(MapOp::Exp, exponent);
    let wave_length = div(konst(2.0 * std::f64::consts::PI), inv_freq.clone());
    let low_wave_length = ROPE_ORIGINAL_CONTEXT / ROPE_LOW_FREQ_FACTOR;
    let high_wave_length = ROPE_ORIGINAL_CONTEXT / ROPE_HIGH_FREQ_FACTOR;
    let smooth = div(
        sub(
            div(konst(ROPE_ORIGINAL_CONTEXT), wave_length.clone()),
            konst(ROPE_LOW_FREQ_FACTOR),
        ),
        konst(ROPE_HIGH_FREQ_FACTOR - ROPE_LOW_FREQ_FACTOR),
    );
    let scaled = div(inv_freq.clone(), konst(ROPE_FACTOR));
    let blended = add(
        mul(sub(konst(1.0), smooth.clone()), scaled.clone()),
        mul(smooth, inv_freq.clone()),
    );
    map(
        MapOp::Where,
        vec![
            binary(MapOp::Lt, konst(low_wave_length), wave_length.clone()),
            scaled,
            map(
                MapOp::Where,
                vec![
                    binary(MapOp::Lt, wave_length, konst(high_wave_length)),
                    inv_freq,
                    blended,
                ],
            ),
        ],
    )
}

fn rope_at(x: NodeRef, position: NodeRef, sequence: Axis, head_dim: Axis) -> NodeRef {
    let pair = axis("rope_pair", 2);
    let frequency = axis("rope_frequency", head_dim.extent() / 2);
    let x = split(x, -1isize, pair, frequency);

    let position = add(mul(iota(sequence), konst(0.0)), position);
    let position = unsqueeze(position, 1);
    let angle = unsqueeze(mul(position, llama3_inv_freq(frequency)), 1);
    let sign = unsqueeze(sub(mul(iota(pair), konst(2.0)), konst(1.0)), 1);
    let rotated = mul(flip(x.clone(), x.shape().len() - 2), sign);
    let result = add(
        mul(x, unary(MapOp::Cos, angle.clone())),
        mul(rotated, unary(MapOp::Sin, angle)),
    );
    let rank = result.shape().len();
    flatten(result, &[rank - 2, rank - 1][..], head_dim)
}

fn update_cache(cache: NodeRef, current: NodeRef, position: NodeRef) -> NodeRef {
    let cache_index = coordinate(cache.clone(), 1usize);
    let at_position = mul(
        binary(
            MapOp::Lt,
            cache_index.clone(),
            add(position.clone(), konst(1.0)),
        ),
        binary(MapOp::Lt, position, add(cache_index, konst(1.0))),
    );
    map(MapOp::Where, vec![at_position, current, cache])
}

fn projection(x: NodeRef, name: String, input_dim: Axis, output_dim: Axis) -> NodeRef {
    let weight = input(leak(name), [output_dim, input_dim], Dtype::F32);
    matmul(x, transpose(weight, 0usize, 1usize))
}

fn leak(name: String) -> &'static str {
    Box::leak(name.into_boxed_str())
}

struct DecodeBlock {
    x: NodeRef,
    key_cache: NodeRef,
    value_cache: NodeRef,
}

fn decode_block(
    axes: &Axes,
    cache_sequence: Axis,
    layer: usize,
    x: NodeRef,
    position: NodeRef,
) -> DecodeBlock {
    let name = |suffix: &str| format!("model.layers.{layer}.{suffix}");
    let attn_input = rms_norm(
        x.clone(),
        input(
            leak(name("input_layernorm.weight")),
            [axes.hidden],
            Dtype::F32,
        ),
        axes.hidden.extent(),
    );

    let query_projection = axis(
        "query_projection",
        axes.query_heads.extent() * axes.head_dim.extent(),
    );
    let q = projection(
        attn_input.clone(),
        name("self_attn.q_proj.weight"),
        axes.hidden,
        query_projection,
    );
    let q = split(q, -1isize, axes.query_heads, axes.head_dim);
    let q = rope_at(
        transpose(q, 0usize, 1usize),
        position.clone(),
        axes.sequence,
        axes.head_dim,
    );

    let kv_projection = axis(
        "kv_projection",
        axes.kv_heads.extent() * axes.head_dim.extent(),
    );
    let k = projection(
        attn_input.clone(),
        name("self_attn.k_proj.weight"),
        axes.hidden,
        kv_projection,
    );
    let k = split(k, -1isize, axes.kv_heads, axes.head_dim);
    let k = rope_at(
        transpose(k, 0usize, 1usize),
        position.clone(),
        axes.sequence,
        axes.head_dim,
    );

    let v = projection(
        attn_input,
        name("self_attn.v_proj.weight"),
        axes.hidden,
        kv_projection,
    );
    let v = transpose(
        split(v, -1isize, axes.kv_heads, axes.head_dim),
        0usize,
        1usize,
    );

    let key_cache = update_cache(
        input(
            leak(format!("cache.{layer}.key")),
            [axes.kv_heads, cache_sequence, axes.head_dim],
            Dtype::F32,
        ),
        k,
        position.clone(),
    );
    let value_cache = update_cache(
        input(
            leak(format!("cache.{layer}.value")),
            [axes.kv_heads, cache_sequence, axes.head_dim],
            Dtype::F32,
        ),
        v,
        position.clone(),
    );

    let visible = binary(
        MapOp::Lt,
        iota(cache_sequence),
        add(position.clone(), konst(1.0)),
    );
    let mask = map(
        MapOp::Where,
        vec![visible, konst(0.0), konst(f64::NEG_INFINITY)],
    );
    let attention = scaled_dot_product_attention(
        q,
        key_cache.clone(),
        value_cache.clone(),
        Some(mask),
        0.0,
        false,
        None,
        true,
    );
    let attention = transpose(attention, 0usize, 1usize);
    let attention = flatten(attention, &[1usize, 2usize][..], axes.hidden);
    let attention = projection(
        attention,
        name("self_attn.o_proj.weight"),
        axes.hidden,
        axes.hidden,
    );
    let residual = add(x, attention);

    let mlp_input = rms_norm(
        residual.clone(),
        input(
            leak(name("post_attention_layernorm.weight")),
            [axes.hidden],
            Dtype::F32,
        ),
        axes.hidden.extent(),
    );
    let gate = projection(
        mlp_input.clone(),
        name("mlp.gate_proj.weight"),
        axes.hidden,
        axes.intermediate,
    );
    let up = projection(
        mlp_input,
        name("mlp.up_proj.weight"),
        axes.hidden,
        axes.intermediate,
    );
    let down = projection(
        mul(silu(gate), up),
        name("mlp.down_proj.weight"),
        axes.intermediate,
        axes.hidden,
    );

    DecodeBlock {
        x: add(residual, down),
        key_cache,
        value_cache,
    }
}

struct DecodeGraph {
    roots: Vec<NodeRef>,
    cache_names: Vec<&'static str>,
    logits_index: usize,
}

fn build_decode(config: Config, context_length: usize) -> DecodeGraph {
    let axes = Axes::new(config);
    let cache_sequence = axis("cache_sequence", context_length);
    let position = input("position", [], Dtype::F32);
    let tokens = input("tokens", [axes.sequence], Dtype::F32);
    let embedding = input(
        "model.embed_tokens.weight",
        [axes.vocab, axes.hidden],
        Dtype::F32,
    );

    let mut x = gather(embedding.clone(), tokens, 0usize);
    let mut cache_roots = Vec::with_capacity(config.layers * 2);
    let mut cache_names = Vec::with_capacity(config.layers * 2);
    for layer in 0..config.layers {
        let decoded = decode_block(&axes, cache_sequence, layer, x, position.clone());
        x = decoded.x;
        cache_roots.push(decoded.key_cache);
        cache_names.push(leak(format!("cache.{layer}.key")));
        cache_roots.push(decoded.value_cache);
        cache_names.push(leak(format!("cache.{layer}.value")));
    }
    let x = rms_norm(
        x,
        input("model.norm.weight", [axes.hidden], Dtype::F32),
        axes.hidden.extent(),
    );
    let logits = matmul(x, transpose(embedding, 0usize, 1usize));
    let logits_index = cache_roots.len();
    cache_roots.push(logits);

    DecodeGraph {
        roots: cache_roots,
        cache_names,
        logits_index,
    }
}

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

/// Weights seeded per input NAME, so every reserved extent binds identical
/// values and their logits are directly comparable.
fn weight_env(graph: &DecodeGraph, context_length: usize, config: Config) -> Env {
    let mut env = Env::new();
    for name in input_names(&graph.roots) {
        let shape: Vec<usize> = if name == "position" {
            vec![]
        } else if name == "tokens" {
            vec![1]
        } else if name.starts_with("cache.") {
            vec![config.kv_heads, context_length, config.head_dim]
        } else if name == "model.embed_tokens.weight" {
            vec![config.vocab_size, config.hidden_dim]
        } else if name.ends_with("layernorm.weight") || name == "model.norm.weight" {
            vec![config.hidden_dim]
        } else if name.ends_with("q_proj.weight") {
            vec![config.query_heads * config.head_dim, config.hidden_dim]
        } else if name.ends_with("k_proj.weight") || name.ends_with("v_proj.weight") {
            vec![config.kv_heads * config.head_dim, config.hidden_dim]
        } else if name.ends_with("o_proj.weight") {
            vec![config.hidden_dim, config.hidden_dim]
        } else if name.ends_with("gate_proj.weight") || name.ends_with("up_proj.weight") {
            vec![config.intermediate_dim, config.hidden_dim]
        } else if name.ends_with("down_proj.weight") {
            vec![config.hidden_dim, config.intermediate_dim]
        } else {
            panic!("unknown input `{name}`")
        };
        let value = if name == "position" || name == "tokens" || name.starts_with("cache.") {
            Value::from_shape_fn(&shape, |_| 0.0)
        } else {
            let mut rng = Lcg(0xC0FFEE ^ name.bytes().fold(0u64, |h, b| h * 31 + b as u64));
            let _ = rng.f();
            Value::from_shape_fn(&shape, |_| rng.f() * 0.25)
        };
        env.insert(name, value);
    }
    env
}

fn input_names(roots: &[NodeRef]) -> Vec<&'static str> {
    use std::collections::HashSet;
    use std::rc::Rc;
    fn visit(
        node: &NodeRef,
        names: &mut Vec<&'static str>,
        seen: &mut HashSet<*const sanic::Node>,
    ) {
        use sanic::Node;
        if !seen.insert(Rc::as_ptr(node)) {
            return;
        }
        match node.as_ref() {
            Node::Input { name, .. } => names.push(name),
            Node::Const { .. } | Node::Iota { .. } => {}
            Node::Coordinate { src, .. }
            | Node::Reduce { src, .. }
            | Node::Scan { src, .. }
            | Node::View { src, .. }
            | Node::Reindex { src, .. } => visit(src, names, seen),
            Node::Map { inputs, .. } => {
                for input in inputs {
                    visit(input, names, seen);
                }
            }
            Node::Gather { src, index, .. } => {
                visit(src, names, seen);
                visit(index, names, seen);
            }
        }
    }
    let mut names = Vec::new();
    let mut seen = HashSet::new();
    for root in roots {
        visit(root, &mut names, &mut seen);
    }
    names.sort();
    names.dedup();
    names
}

/// Prefill `tokens` through the interpreter, threading cache feedback, and
/// return every root's value at every step.
fn interp_prefill(graph: &DecodeGraph, env: &mut Env, tokens: &[f64]) -> Vec<Vec<Vec<f64>>> {
    let mut steps = Vec::new();
    for (position, &token) in tokens.iter().enumerate() {
        env.insert("tokens", Value::from_shape_fn(&[1], |_| token));
        env.insert("position", Value::from_shape_fn(&[], |_| position as f64));
        let outputs: Vec<Value> = graph.roots.iter().map(|root| eval(root, env)).collect();
        for (cache, name) in graph.cache_names.iter().enumerate() {
            env.insert(name, outputs[cache].clone());
        }
        steps.push(outputs.into_iter().map(|value| value.data).collect());
    }
    steps
}

/// Prefill the same tokens through the compiled Metal replay path the real
/// example uses, and return every root's value at every step.
fn metal_prefill(
    graph: &DecodeGraph,
    env: &Env,
    tokens: &[f64],
    device: &sanic::MetalDevice,
) -> Vec<Vec<Vec<f32>>> {
    let program = graph.roots.compile(device).expect("Metal compile");
    let mut buffers = HashMap::new();
    for name in program.input_names() {
        let value = &env[name];
        let raw = device.from_f64(&value.data);
        let tensor = device
            .tensor_from_raw(raw, value.shape.clone(), Dtype::F32)
            .expect("tensor binding");
        buffers.insert(name.to_string(), tensor);
    }
    let feedback = graph
        .cache_names
        .iter()
        .enumerate()
        .map(|(cache, name)| (cache, *name))
        .collect::<Vec<_>>();
    let mut replay = program
        .capture(
            buffers.iter().map(|(name, buffer)| (name.as_str(), buffer)),
            &feedback,
        )
        .expect("capture");
    let tokens_buffer = buffers["tokens"].clone();
    let position_buffer = buffers["position"].clone();
    let mut steps = Vec::new();
    for (position, &token) in tokens.iter().enumerate() {
        device.write_f64(tokens_buffer.raw(), &[token]);
        device.write_f64(position_buffer.raw(), &[position as f64]);
        let outputs = replay.step().expect("replay step");
        steps.push(
            outputs
                .iter()
                .map(|output| device.read_tensor_f32(output))
                .collect(),
        );
    }
    steps
}

/// The minimal spelling of the fast-math mask deletion: attention whose
/// additive mask is COMPUTED from `iota` over the cache axis and a runtime
/// `position` scalar, exactly as a KV-cache decode graph builds it. The
/// buffer-bound mask already has a passing GPU test; this pins the computed
/// literal form.
#[test]
fn computed_iota_mask_reaches_the_fused_attention_fold() {
    use sanic::cost::DeviceProfile;
    use sanic::emit_metal::emit_schedule_metal_on;
    use sanic::metal::program_dispatches;
    use sanic::partition::partition;

    let (heads, cache, features) = (axis("heads", 4), axis("cache", 9), axis("features", 8));
    let sequence = axis("sequence", 1);
    let mut rng = Lcg(0x5EED);
    let env: Env = [
        ("q", rand_tensor(&[heads, sequence, features], &mut rng)),
        ("k", rand_tensor(&[heads, cache, features], &mut rng)),
        ("v", rand_tensor(&[heads, cache, features], &mut rng)),
        ("position", Value::from_shape_fn(&[], |_| 1.0)),
    ]
    .into_iter()
    .collect();

    let position = input("position", [], Dtype::F32);
    let visible = binary(MapOp::Lt, iota(cache), add(position, konst(1.0)));
    let mask = map(
        MapOp::Where,
        vec![visible, konst(0.0), konst(f64::NEG_INFINITY)],
    );
    let attention = scaled_dot_product_attention(
        input("q", [heads, sequence, features], Dtype::F32),
        input("k", [heads, cache, features], Dtype::F32),
        input("v", [heads, cache, features], Dtype::F32),
        Some(mask),
        0.0,
        false,
        None,
        false,
    );

    let reference = eval(&attention, &env);
    let schedule = partition(&attention, &DeviceProfile::m1_pro());
    let program = emit_schedule_metal_on(&DeviceProfile::m1_pro(), &schedule);
    let Some(device) = sanic::metal::MetalDevice::open() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let pipelines = device.compile(&program.msl);
    let mut buffers = HashMap::new();
    for (name, _) in &program.inputs {
        buffers.insert(name.to_string(), device.from_f64(&env[*name].data));
    }
    for (name, size) in &program.buffers {
        buffers.insert(name.clone(), device.alloc_f32(*size));
    }
    device.run(&program_dispatches(&program, &buffers, &pipelines));
    let output = &program.stages.last().expect("stages").output;
    let got = device.read_f32(&buffers[output], reference.data.len());
    let error = got
        .iter()
        .zip(&reference.data)
        .map(|(got, expected)| (*got as f64 - expected).abs() / (1.0 + expected.abs()))
        .fold(0.0f64, f64::max);
    assert!(error < 2e-3, "computed-mask attention off by {error:e}");
}

#[test]
fn reserved_cache_extent_does_not_change_decode_logits() {
    let prompt: Vec<f64> = vec![7.0, 3.0, 21.0, 40.0, 11.0, 30.0];
    let extents = [6usize, 9, 16, 22];

    let Some(device) = sanic::MetalDevice::open() else {
        eprintln!("skipping: no Metal device");
        return;
    };

    let mut interp_reference: Option<Vec<f64>> = None;
    let mut failures = Vec::new();
    for context_length in extents {
        let graph = build_decode(COMPACT, context_length);
        let env = weight_env(&graph, context_length, COMPACT);
        let interp = interp_prefill(&graph, &mut env.clone(), &prompt);
        let metal = metal_prefill(&graph, &env, &prompt, &device);

        // (a) the interpreter itself must not depend on the reserved extent
        let final_logits = &interp.last().unwrap()[graph.logits_index];
        if let Some(reference) = &interp_reference {
            let drift = reference
                .iter()
                .zip(final_logits)
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f64, f64::max);
            if drift > 1e-9 {
                failures.push(format!("interp drift {drift:e} at extent {context_length}"));
            }
        } else {
            interp_reference = Some(final_logits.clone());
        }

        // (b) Metal must agree with the interpreter on every root of every
        // step — cache roots localize a failure to the layer that broke
        let mut worst = 0.0f64;
        for (step, (interp_roots, metal_roots)) in interp.iter().zip(&metal).enumerate() {
            for (root, (expected, got)) in interp_roots.iter().zip(metal_roots).enumerate() {
                let error = expected
                    .iter()
                    .zip(got)
                    .map(|(expected, got)| (*got as f64 - expected).abs() / (1.0 + expected.abs()))
                    .fold(0.0f64, f64::max);
                worst = worst.max(error);
                if error > 2e-3 {
                    let name = graph.cache_names.get(root).copied().unwrap_or("logits");
                    eprintln!("extent {context_length} step {step}: {name} err {error:e}");
                }
            }
        }
        eprintln!("extent {context_length}: metal-vs-interp max rel err = {worst:e}");
        if worst > 2e-3 {
            failures.push(format!(
                "metal mismatch {worst:e} at extent {context_length}"
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
