//! Greedy Llama 3.2 1B generation on the public positional graph API.
//!
//! It discovers `meta-llama/Llama-3.2-1B` in the Hugging Face cache, loads the
//! snapshot's `tokenizer.json`, binds its BF16 checkpoint zero-copy on Metal,
//! and runs one compiled KV-cache decode step repeatedly. Projection matrices
//! stay in their checkpoint-native `[output, input]` storage; transpose is a
//! graph view.
//!
//! ```text
//! cargo run --release --example llama3_2 -- "The capital of France is" -n 16
//! ```

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use safetensors::SafeTensors;
use sanic::nn::functional::scaled_dot_product_attention;
use sanic::{
    Axis, Compile, Dtype, MapOp, Monoid, Node, NodeRef, ViewDim, axis, coordinate, flatten, gather,
    input, iota, konst, map, matmul, positional_reindex, positional_view, reduce, silu, split,
    transpose,
};
use tokenizers::Tokenizer;

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

impl Config {
    const LLAMA_3_2_1B: Self = Self {
        vocab_size: 128_256,
        layers: 16,
        hidden_dim: 2_048,
        query_heads: 32,
        kv_heads: 8,
        head_dim: 64,
        intermediate_dim: 8_192,
    };
}

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
    fn new(config: Config, sequence_length: usize) -> Self {
        assert_eq!(config.query_heads % config.kv_heads, 0);
        assert_eq!(config.hidden_dim, config.query_heads * config.head_dim);
        assert_eq!(config.head_dim % 2, 0);
        Self {
            vocab: axis("vocab", config.vocab_size),
            sequence: axis("sequence", sequence_length),
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
    assert!(dim <= source.len());
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
    assert!(dim < shape.len());
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
    let checkpoint_weight = input(name, [output_dim, input_dim], Dtype::BF16);
    matmul(x, transpose(checkpoint_weight, 0usize, 1usize))
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
        input(name("input_layernorm.weight"), [axes.hidden], Dtype::BF16),
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

    let key_name = format!("cache.{layer}.key");
    let value_name = format!("cache.{layer}.value");
    let key_cache = update_cache(
        input(
            key_name,
            [axes.kv_heads, cache_sequence, axes.head_dim],
            Dtype::F32,
        ),
        k,
        position.clone(),
    );
    let value_cache = update_cache(
        input(
            value_name,
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
            name("post_attention_layernorm.weight"),
            [axes.hidden],
            Dtype::BF16,
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
    cache_names: Vec<String>,
    logits_index: usize,
}

fn build_decode(config: Config, context_length: usize) -> DecodeGraph {
    assert!(context_length > 0);
    let axes = Axes::new(config, 1);
    let cache_sequence = axis("cache_sequence", context_length);
    let position = input("position", [], Dtype::F32);
    let tokens = input("tokens", [axes.sequence], Dtype::F32);
    let embedding = input(
        "model.embed_tokens.weight",
        [axes.vocab, axes.hidden],
        Dtype::BF16,
    );

    let mut x = gather(embedding.clone(), tokens, 0usize);
    let mut cache_roots = Vec::with_capacity(config.layers * 2);
    let mut cache_names = Vec::with_capacity(config.layers * 2);
    for layer in 0..config.layers {
        let decoded = decode_block(&axes, cache_sequence, layer, x, position.clone());
        x = decoded.x;
        cache_roots.push(decoded.key_cache);
        cache_names.push(format!("cache.{layer}.key"));
        cache_roots.push(decoded.value_cache);
        cache_names.push(format!("cache.{layer}.value"));
    }
    let x = rms_norm(
        x,
        input("model.norm.weight", [axes.hidden], Dtype::BF16),
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

fn cached_model_dir() -> Result<PathBuf, String> {
    if let Some(path) = std::env::var_os("LLAMA3_2_MODEL_DIR") {
        return Ok(path.into());
    }
    let home = std::env::var_os("HOME").ok_or("HOME is not set")?;
    let repository =
        PathBuf::from(home).join(".cache/huggingface/hub/models--meta-llama--Llama-3.2-1B");
    let revision = std::fs::read_to_string(repository.join("refs/main"))
        .map_err(|error| format!("Llama 3.2 is not in the Hugging Face cache: {error}"))?;
    Ok(repository.join("snapshots").join(revision.trim()))
}

fn input_specs(roots: &[NodeRef]) -> HashMap<String, (Vec<usize>, Dtype)> {
    fn visit(
        node: &NodeRef,
        specs: &mut HashMap<String, (Vec<usize>, Dtype)>,
        seen: &mut HashSet<*const Node>,
    ) {
        if !seen.insert(Arc::as_ptr(node)) {
            return;
        }
        match node.as_ref() {
            Node::Input { name, shape, dtype } => {
                let shape = shape.iter().copied().map(Axis::extent).collect();
                let declaration = (shape, *dtype);
                if let Some(previous) = specs.insert(name.to_string(), declaration.clone()) {
                    assert_eq!(previous, declaration, "incompatible input `{name}`");
                }
            }
            Node::Const { .. } | Node::Iota { .. } => {}
            Node::Coordinate { src, .. } => visit(src, specs, seen),
            Node::Map { inputs, .. } => {
                for input in inputs {
                    visit(input, specs, seen);
                }
            }
            Node::Reduce { src, .. }
            | Node::Scan { src, .. }
            | Node::View { src, .. }
            | Node::Reindex { src, .. } => visit(src, specs, seen),
            Node::Gather { src, index, .. } => {
                visit(src, specs, seen);
                visit(index, specs, seen);
            }
        }
    }

    let mut specs = HashMap::new();
    let mut seen = HashSet::new();
    for root in roots {
        visit(root, &mut specs, &mut seen);
    }
    specs
}

fn validate_checkpoint(
    roots: &[NodeRef],
    checkpoint: &SafeTensors,
) -> Result<HashMap<String, (Vec<usize>, Dtype)>, String> {
    let specs = input_specs(roots);
    for (name, (expected_shape, expected_dtype)) in &specs {
        if name == "tokens" || name == "position" || name.starts_with("cache.") {
            continue;
        }
        let tensor = checkpoint
            .tensor(name)
            .map_err(|_| format!("checkpoint is missing `{name}`"))?;
        if tensor.shape() != expected_shape {
            return Err(format!(
                "`{name}` has checkpoint shape {:?}; graph expects {expected_shape:?}",
                tensor.shape()
            ));
        }
        let expected_dtype = match expected_dtype {
            Dtype::BF16 => safetensors::Dtype::BF16,
            Dtype::F32 => safetensors::Dtype::F32,
            other => {
                return Err(format!(
                    "unsupported checkpoint dtype {other:?} for `{name}`"
                ));
            }
        };
        if tensor.dtype() != expected_dtype {
            return Err(format!(
                "`{name}` has checkpoint dtype {:?}; graph expects {expected_dtype:?}",
                tensor.dtype()
            ));
        }
    }
    Ok(specs)
}

/// Read the checkpoint into a page-aligned, LEAKED allocation and parse it in
/// place: `MetalDevice::from_bytes_nocopy` wraps the region without copying,
/// and every tensor binds at its in-file offset. The bytes live for the
/// process — a model's weights do anyway.
///
/// A file whose header length is not a multiple of 4 puts every tensor at a
/// misaligned byte offset, and device buffers cannot bind there. All tensors
/// share the parity, so ONE lead pad realigns the whole data section.
fn open_checkpoint_zero_copy(path: &Path) -> Result<(SafeTensors<'static>, &'static [u8]), String> {
    use std::io::Read;

    let mut file =
        std::fs::File::open(path).map_err(|error| format!("open {}: {error}", path.display()))?;
    let mut header_length = [0u8; 8];
    file.read_exact(&mut header_length)
        .map_err(|error| format!("read {} header: {error}", path.display()))?;
    let data_start = 8 + u64::from_le_bytes(header_length) as usize;
    let pad = data_start.next_multiple_of(4) - data_start;

    const PAGE: usize = 16384;
    let file_length = std::fs::metadata(path)
        .map_err(|error| error.to_string())?
        .len() as usize;
    let capacity = (pad + file_length).div_ceil(PAGE).max(1) * PAGE;
    let layout =
        std::alloc::Layout::from_size_align(capacity, PAGE).map_err(|error| error.to_string())?;
    let pointer = unsafe { std::alloc::alloc_zeroed(layout) };
    if pointer.is_null() {
        return Err("page-aligned checkpoint allocation failed".into());
    }
    let region: &'static mut [u8] = unsafe { std::slice::from_raw_parts_mut(pointer, capacity) };
    region[pad..pad + 8].copy_from_slice(&header_length);
    file.read_exact(&mut region[pad + 8..pad + file_length])
        .map_err(|error| format!("read {}: {error}", path.display()))?;
    let region: &'static [u8] = region;
    let tensors = SafeTensors::deserialize(&region[pad..pad + file_length])
        .map_err(|error| format!("parse {}: {error}", path.display()))?;
    Ok((tensors, region))
}

struct Arguments {
    prompt: String,
    new_tokens: usize,
}

fn usage() -> &'static str {
    "usage: cargo run --release --example llama3_2 -- \"prompt\" -n <tokens>"
}

fn parse_arguments(arguments: impl IntoIterator<Item = String>) -> Result<Arguments, String> {
    let mut prompt = None;
    let mut new_tokens = None;
    let mut arguments = arguments.into_iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "-h" | "--help" => return Err(usage().to_string()),
            "-n" | "--num-tokens" => {
                let value = arguments
                    .next()
                    .ok_or_else(|| format!("{} requires a value\n{}", argument, usage()))?;
                let count = value
                    .parse::<usize>()
                    .map_err(|error| format!("invalid token count `{value}`: {error}"))?;
                if new_tokens.replace(count).is_some() {
                    return Err(format!("token count was provided twice\n{}", usage()));
                }
            }
            option if option.starts_with('-') => {
                return Err(format!("unknown option `{option}`\n{}", usage()));
            }
            value => {
                if prompt.replace(value.to_string()).is_some() {
                    return Err(format!("more than one prompt was provided\n{}", usage()));
                }
            }
        }
    }
    Ok(Arguments {
        prompt: prompt.ok_or_else(|| format!("prompt is required\n{}", usage()))?,
        new_tokens: new_tokens.ok_or_else(|| format!("-n is required\n{}", usage()))?,
    })
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("the llama3_2 example requires Metal and currently runs only on macOS");
}

#[cfg(target_os = "macos")]
fn main() {
    if let Err(error) = run_metal() {
        eprintln!("llama3_2: {error}");
        std::process::exit(2);
    }
}

#[cfg(target_os = "macos")]
fn run_metal() -> Result<(), String> {
    use std::io::Write;

    let arguments = parse_arguments(std::env::args().skip(1))?;
    let model_dir = cached_model_dir()?;
    let checkpoint = model_dir.join("model.safetensors");
    let tokenizer = Tokenizer::from_file(model_dir.join("tokenizer.json"))
        .map_err(|error| format!("load tokenizer: {error}"))?;
    let encoding = tokenizer
        .encode(arguments.prompt.as_str(), true)
        .map_err(|error| format!("tokenize prompt: {error}"))?;
    let prompt_tokens = encoding.get_ids().to_vec();
    if prompt_tokens.is_empty() {
        return Err("the tokenizer produced an empty prompt".into());
    }
    if arguments.new_tokens == 0 {
        println!("{}", arguments.prompt);
        return Ok(());
    }

    let context_length = prompt_tokens.len() + arguments.new_tokens;
    let started = std::time::Instant::now();
    eprintln!(
        "building one-token decode graph ({} prompt + {} generated tokens)...",
        prompt_tokens.len(),
        arguments.new_tokens
    );
    let graph = build_decode(Config::LLAMA_3_2_1B, context_length);
    eprintln!("built graph in {:.2}s", started.elapsed().as_secs_f32());

    let started = std::time::Instant::now();
    eprintln!("reading cached BF16 checkpoint...");
    let (checkpoint_tensors, region) = open_checkpoint_zero_copy(&checkpoint)?;
    let specs = validate_checkpoint(&graph.roots, &checkpoint_tensors)
        .map_err(|error| format!("invalid cached checkpoint: {error}"))?;
    eprintln!(
        "read and validated checkpoint in {:.2}s",
        started.elapsed().as_secs_f32()
    );

    let device = sanic::MetalDevice::open().ok_or("no Metal device is available")?;
    let started = std::time::Instant::now();
    eprintln!("compiling decode program...");
    let program = graph
        .roots
        .compile(&device)
        .map_err(|error| error.to_string())?;
    eprintln!(
        "compiled {} kernels in {:.2}s",
        program.kernel_count(),
        started.elapsed().as_secs_f32()
    );

    let started = std::time::Instant::now();
    eprintln!("binding BF16 weights zero-copy...");
    let checkpoint_buffer = device
        .from_bytes_nocopy(region)
        .ok_or("checkpoint allocation is not suitable for zero-copy Metal binding")?;
    let mut buffers = HashMap::new();
    let mut zero_copy_bytes = 0usize;
    for name in program.input_names() {
        let (shape, dtype) = specs
            .get(name)
            .ok_or_else(|| format!("compiled input `{name}` has no graph declaration"))?;
        let buffer = if name == "tokens" || name == "position" {
            device
                .tensor_from_raw(
                    device.alloc_f32(shape.iter().product()),
                    shape.clone(),
                    *dtype,
                )
                .map_err(|error| error.to_string())?
        } else if name.starts_with("cache.") {
            device
                .tensor_from_raw(
                    device.alloc_f32(shape.iter().product()),
                    shape.clone(),
                    *dtype,
                )
                .map_err(|error| error.to_string())?
        } else {
            let data = checkpoint_tensors
                .tensor(name)
                .map_err(|error| format!("read `{name}` from checkpoint: {error}"))?
                .data();
            let offset = data.as_ptr() as usize - region.as_ptr() as usize;
            let raw = if offset % 4 == 0 {
                zero_copy_bytes += data.len();
                checkpoint_buffer.slice(offset)
            } else {
                device.from_bytes(data)
            };
            device
                .tensor_from_raw(raw, shape.clone(), *dtype)
                .map_err(|error| error.to_string())?
        };
        buffers.insert(name.to_string(), buffer);
    }
    eprintln!(
        "bound {} tensors ({:.2} GB zero-copy) in {:.2}s",
        specs
            .keys()
            .filter(|name| {
                *name != "tokens" && *name != "position" && !name.starts_with("cache.")
            })
            .count(),
        zero_copy_bytes as f64 / 1e9,
        started.elapsed().as_secs_f32()
    );

    // Freeze both cache-binding parities for repeated execution: each cache
    // output is the NEXT step's cache input.
    let feedback = graph
        .cache_names
        .iter()
        .enumerate()
        .map(|(cache, name)| {
            let output = if cache < graph.logits_index {
                cache
            } else {
                cache + 1
            };
            (output, name.as_str())
        })
        .collect::<Vec<_>>();
    let started = std::time::Instant::now();
    let mut replay = program
        .capture(
            buffers.iter().map(|(name, buffer)| (name.as_str(), buffer)),
            &feedback,
        )
        .map_err(|error| error.to_string())?;
    eprintln!(
        "prepared {} dispatches for two replay bindings in {:.2}s",
        program.kernel_count(),
        started.elapsed().as_secs_f32()
    );

    let tokens_buffer = buffers["tokens"].clone();
    let position_buffer = buffers["position"].clone();
    let mut step = |token: u32, position: usize| -> Result<(sanic::MetalBuffer, f64), String> {
        device.write_f64(tokens_buffer.raw(), &[token as f64]);
        device.write_f64(position_buffer.raw(), &[position as f64]);
        let (outputs, seconds) = replay.step_timed().map_err(|error| error.to_string())?;
        Ok((outputs[graph.logits_index].clone(), seconds))
    };

    eprintln!("prefilling {} tokens...", prompt_tokens.len());
    let started = std::time::Instant::now();
    let mut logits = None;
    let mut prefill_gpu_seconds = 0.0f64;
    for (position, &token) in prompt_tokens.iter().enumerate() {
        let (output, seconds) = step(token, position)?;
        logits = Some(output);
        prefill_gpu_seconds += seconds;
    }
    eprintln!(
        "prefill finished in {:.2}s ({:.1} ms/tok GPU replay)",
        started.elapsed().as_secs_f32(),
        1e3 * prefill_gpu_seconds / prompt_tokens.len() as f64
    );

    let mut stream = tokenizer.decode_stream(true);
    for &token in &prompt_tokens {
        stream
            .step(token)
            .map_err(|error| format!("initialize decoder: {error}"))?;
    }
    print!("{}", arguments.prompt);
    std::io::stdout()
        .flush()
        .map_err(|error| error.to_string())?;

    let started = std::time::Instant::now();
    let mut generated = 0usize;
    let mut decode_steps = 0usize;
    let mut decode_gpu_seconds = 0.0f64;
    while generated < arguments.new_tokens {
        let scores = device.read_tensor_f32(logits.as_ref().unwrap());
        if std::env::var_os("LLAMA3_2_DEBUG_LOGITS").is_some() {
            let mut ranked = scores.iter().copied().enumerate().collect::<Vec<_>>();
            ranked.sort_by(|(_, left), (_, right)| right.total_cmp(left));
            eprintln!(
                "step {generated}: {}",
                ranked
                    .iter()
                    .take(3)
                    .map(|(token, score)| format!("{token}:{score:.6}"))
                    .collect::<Vec<_>>()
                    .join("  ")
            );
        }
        let next = scores
            .iter()
            .enumerate()
            .max_by(|(_, left), (_, right)| left.total_cmp(right))
            .map(|(index, _)| index as u32)
            .ok_or("model returned empty logits")?;
        generated += 1;
        if let Some(text) = stream
            .step(next)
            .map_err(|error| format!("decode token {next}: {error}"))?
        {
            print!("{text}");
            std::io::stdout()
                .flush()
                .map_err(|error| error.to_string())?;
        }
        if next == 128_001 || generated == arguments.new_tokens {
            break;
        }
        let (output, seconds) = step(next, prompt_tokens.len() + generated - 1)?;
        logits = Some(output);
        decode_gpu_seconds += seconds;
        decode_steps += 1;
    }
    println!();
    let elapsed = started.elapsed().as_secs_f32();
    if decode_steps == 0 {
        eprintln!("generated {generated} token from the prefill logits");
    } else {
        eprintln!(
            "generated {generated} tokens in {elapsed:.2}s ({:.2} decode tok/s, {:.1} ms/tok GPU replay)",
            decode_steps as f32 / elapsed,
            1e3 * decode_gpu_seconds / decode_steps as f64
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sanic::CpuDevice;
    use sanic::interp::{Env, eval};

    #[test]
    fn rope_frequencies_use_even_head_coordinates() {
        let frequencies = axis("frequency", 32);
        let resolved = eval(&llama3_inv_freq(frequencies), &Env::new());

        // Llama's arange(0, head_dim, 2) / head_dim is equivalent to
        // frequency_index / (head_dim / 2). The first nonzero frequency is
        // still above the scaling transition, so it is unchanged by llama3
        // long-context scaling.
        let expected = (-ROPE_THETA.ln() / 32.0).exp();
        assert!((resolved.data[1] - expected).abs() < 1e-12);
    }

    #[test]
    fn compact_decode_builds_caches_and_compiles_through_functional_sdpa() {
        let config = Config {
            vocab_size: 16,
            layers: 1,
            hidden_dim: 8,
            query_heads: 4,
            kv_heads: 2,
            head_dim: 2,
            intermediate_dim: 16,
        };
        let graph = build_decode(config, 3);
        assert_eq!(
            graph
                .roots
                .iter()
                .map(|root| root
                    .shape()
                    .into_iter()
                    .map(Axis::extent)
                    .collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [vec![2, 3, 2], vec![2, 3, 2], vec![1, 16]]
        );

        let program = graph.roots.compile(&CpuDevice::new()).unwrap();
        assert_eq!(
            program.output_shapes(),
            &[vec![2, 3, 2], vec![2, 3, 2], vec![1, 16]]
        );
        assert_eq!(program.input_names().len(), 15);
    }

    #[test]
    fn command_line_is_prompt_then_token_count() {
        let args = parse_arguments(["hello".into(), "-n".into(), "7".into()]).unwrap();
        assert_eq!(args.prompt, "hello");
        assert_eq!(args.new_tokens, 7);
    }
}
