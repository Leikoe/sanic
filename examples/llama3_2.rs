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

// Off macOS everything below is unreachable — `main` is the stub at the foot of
// the file — so its model definition is dead by construction, not by mistake.
// A file-level `cfg` like the Metal tests use would leave the target with no
// `main` at all.
#![cfg_attr(not(target_os = "macos"), allow(dead_code, unused_imports))]

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use safetensors::SafeTensors;
use sanic::nn::ops::{attention, rms_norm, rope, update_cache};
use sanic::{Axis, Dtype, Graph, Node, NodeRef, Tensor, axis};
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

fn projection(graph: &mut Graph, x: &Tensor, name: String, input_dim: Axis, output_dim: Axis) -> Tensor {
    // The term is a free variable; that the checkpoint stores this weight
    // bf16 is the graph's declaration, not the mathematics'.
    let weight = graph.input(name, [output_dim, input_dim], Dtype::BF16);
    x.matmul(weight.transpose(0usize, 1usize))
}

/// Llama 3's long-context rope schedule: plain `θ^(-i/frequencies)` inverse
/// frequencies, with low-frequency wavelengths scaled down and a smooth
/// blend across the transition band. Model policy, so it lives here.
fn llama3_inv_freq(frequency: Axis) -> Tensor {
    let inv_freq = (Tensor::iota(frequency) * (-ROPE_THETA.ln() / frequency.extent() as f64)).exp();
    let wave_length = 2.0 * std::f64::consts::PI / &inv_freq;
    let low_wave_length = ROPE_ORIGINAL_CONTEXT / ROPE_LOW_FREQ_FACTOR;
    let high_wave_length = ROPE_ORIGINAL_CONTEXT / ROPE_HIGH_FREQ_FACTOR;
    let smooth =
        (ROPE_ORIGINAL_CONTEXT / &wave_length - ROPE_LOW_FREQ_FACTOR) / (ROPE_HIGH_FREQ_FACTOR - ROPE_LOW_FREQ_FACTOR);
    let scaled = &inv_freq / ROPE_FACTOR;
    let blended = (1.0 - &smooth) * &scaled + smooth * &inv_freq;
    Tensor::scalar(low_wave_length)
        .lt(&wave_length)
        .select(scaled, wave_length.lt(high_wave_length).select(inv_freq, blended))
}

/// One decoder layer. Cache state is declared on `graph`; the layer reads
/// its updated value (this step's row is visible to this step's attention)
/// and the declaration carries it to the next step.
fn decode_block(
    graph: &mut Graph,
    axes: &Axes,
    cache_sequence: Axis,
    layer: usize,
    x: Tensor,
    position: &Tensor,
    cache_dtype: Dtype,
) -> Tensor {
    let name = |suffix: &str| format!("model.layers.{layer}.{suffix}");
    let norm_weight = |graph: &mut Graph, name: String| graph.input(name, [axes.hidden], Dtype::BF16);
    let attn_input = rms_norm(&x, &norm_weight(graph, name("input_layernorm.weight")), EPS);

    let query_projection = axis("query_projection", axes.query_heads.extent() * axes.head_dim.extent());
    let kv_projection = axis("kv_projection", axes.kv_heads.extent() * axes.head_dim.extent());

    let q = projection(
        graph,
        &attn_input,
        name("self_attn.q_proj.weight"),
        axes.hidden,
        query_projection,
    )
    .split(-1isize, axes.query_heads, axes.head_dim)
    .transpose(0usize, 1usize);
    let q = rope(&q, position, axes.sequence, axes.head_dim, llama3_inv_freq);

    let k = projection(
        graph,
        &attn_input,
        name("self_attn.k_proj.weight"),
        axes.hidden,
        kv_projection,
    )
    .split(-1isize, axes.kv_heads, axes.head_dim)
    .transpose(0usize, 1usize);
    let k = rope(&k, position, axes.sequence, axes.head_dim, llama3_inv_freq);

    let v = projection(
        graph,
        &attn_input,
        name("self_attn.v_proj.weight"),
        axes.hidden,
        kv_projection,
    )
    .split(-1isize, axes.kv_heads, axes.head_dim)
    .transpose(0usize, 1usize);

    // Each cache is stored with the axis its fold CONTRACTS innermost, so both
    // folds walk memory contiguously. K is contracted over head_dim by the
    // scores fold; V is contracted over cache_sequence by the output fold, so
    // V is stored transposed. The cost is that V's row write scatters over
    // head_dim — 64 lines instead of one — which is 64 values per layer and
    // nothing next to what the fold saves.
    let key_shape = [axes.kv_heads, cache_sequence, axes.head_dim];
    let key_state = graph.state(format!("cache.{layer}.key"), key_shape, cache_dtype);
    let key_cache = update_cache(&key_state, &k, position, 1usize);
    graph.update(&key_state, key_cache.clone());
    let value_shape = [axes.kv_heads, axes.head_dim, cache_sequence];
    let value_state = graph.state(format!("cache.{layer}.value"), value_shape, cache_dtype);
    let value_cache = update_cache(&value_state, &v.transpose(1usize, 2usize), position, 2usize);
    graph.update(&value_state, value_cache.clone());

    let visible = Tensor::iota(cache_sequence).lt(position + 1.0);
    let mask = visible.select(0.0, f64::NEG_INFINITY);
    let value_rows = value_cache.transpose(1usize, 2usize);
    let attended = attention(&q, &key_cache, &value_rows, Some(&mask), None, true)
        .transpose(0usize, 1usize)
        .flatten(&[1usize, 2usize][..], axes.hidden);
    let attended = projection(
        graph,
        &attended,
        name("self_attn.o_proj.weight"),
        axes.hidden,
        axes.hidden,
    );
    let residual = x + attended;

    let mlp_input = rms_norm(
        &residual,
        &norm_weight(graph, name("post_attention_layernorm.weight")),
        EPS,
    );
    let gate = projection(
        graph,
        &mlp_input,
        name("mlp.gate_proj.weight"),
        axes.hidden,
        axes.intermediate,
    );
    let up = projection(
        graph,
        &mlp_input,
        name("mlp.up_proj.weight"),
        axes.hidden,
        axes.intermediate,
    );
    let down = projection(
        graph,
        &(gate.silu() * up),
        name("mlp.down_proj.weight"),
        axes.intermediate,
        axes.hidden,
    );
    residual + down
}

fn build_decode(config: Config, context_length: usize, cache_dtype: Dtype) -> Graph {
    assert!(context_length > 0);
    let axes = Axes::new(config, 1);
    let cache_sequence = axis("cache_sequence", context_length);
    let mut graph = Graph::new();
    let position = graph.input("position", [], Dtype::F32);
    let tokens = graph.input("tokens", [axes.sequence], Dtype::F32);
    let embedding = graph.input("model.embed_tokens.weight", [axes.vocab, axes.hidden], Dtype::BF16);

    let mut x = embedding.gather(&tokens, 0usize);
    for layer in 0..config.layers {
        x = decode_block(&mut graph, &axes, cache_sequence, layer, x, &position, cache_dtype);
    }
    let x = rms_norm(&x, &graph.input("model.norm.weight", [axes.hidden], Dtype::BF16), EPS);
    // Greedy sampling belongs on the GPU: picking the argmax host-side means
    // reading the whole vocabulary back and scanning it every token, which is
    // 128,256 values to learn one. Emitting the index instead makes the step's
    // useful output a single number. `logits` stays an output because the
    // argmax already materializes it, so naming it costs nothing and keeps
    // LLAMA3_2_DEBUG_LOGITS able to rank the scores.
    // Producer before consumer: `token` folds over `logits`, and a root
    // reachable from a later root is reused through its materialization. The
    // other order asks the partitioner to rebuild the vocabulary projection as
    // its own fold, which it correctly refuses.
    let logits = x.matmul(embedding.transpose(0usize, 1usize));
    if std::env::var_os("LLAMA3_2_DEBUG_LOGITS").is_some() {
        graph.output("logits", logits.clone());
    }
    // No pin: the law knows this is an index into a 128,256-entry
    // vocabulary — ℕ̄, bounded, +∞ fold identity — and mints u32 with a
    // saturating store on its own. The pin this replaced (#21) predates the
    // law; a caller may still outrank the mint, but no longer has to.
    graph.output("token", sanic::argmax(logits.node().clone(), -1isize).into());
    graph
}

fn cached_model_dir() -> Result<PathBuf, String> {
    if let Some(path) = std::env::var_os("LLAMA3_2_MODEL_DIR") {
        return Ok(path.into());
    }
    let home = std::env::var_os("HOME").ok_or("HOME is not set")?;
    let repository = PathBuf::from(home).join(".cache/huggingface/hub/models--meta-llama--Llama-3.2-1B");
    let revision = std::fs::read_to_string(repository.join("refs/main"))
        .map_err(|error| format!("Llama 3.2 is not in the Hugging Face cache: {error}"))?;
    Ok(repository.join("snapshots").join(revision.trim()))
}

fn input_specs(roots: &[NodeRef], declared: &HashMap<String, Dtype>) -> HashMap<String, (Vec<usize>, Dtype)> {
    fn visit(
        node: &NodeRef,
        declared: &HashMap<String, Dtype>,
        specs: &mut HashMap<String, (Vec<usize>, Dtype)>,
        seen: &mut HashSet<*const Node>,
    ) {
        if !seen.insert(Arc::as_ptr(node)) {
            return;
        }
        match node.as_ref() {
            Node::Input { name, shape } => {
                let shape = shape.iter().copied().map(Axis::extent).collect();
                let dtype = declared.get(*name).copied().unwrap_or(Dtype::F32);
                let declaration = (shape, dtype);
                if let Some(previous) = specs.insert(name.to_string(), declaration.clone()) {
                    assert_eq!(previous, declaration, "incompatible input `{name}`");
                }
            }
            Node::Const { .. } | Node::Iota { .. } => {}
            Node::Coordinate { src, .. } => visit(src, declared, specs, seen),
            Node::Map { inputs, .. } => {
                for input in inputs {
                    visit(input, declared, specs, seen);
                }
            }
            Node::Reduce { src, .. } | Node::Scan { src, .. } | Node::View { src, .. } | Node::Reindex { src, .. } => {
                visit(src, declared, specs, seen)
            }
            Node::Gather { src, index, .. } => {
                visit(src, declared, specs, seen);
                visit(index, declared, specs, seen);
            }
        }
    }

    let mut specs = HashMap::new();
    let mut seen = HashSet::new();
    for root in roots {
        visit(root, declared, &mut specs, &mut seen);
    }
    specs
}

fn validate_checkpoint(
    roots: &[NodeRef],
    declared: &HashMap<String, Dtype>,
    checkpoint: &SafeTensors,
) -> Result<HashMap<String, (Vec<usize>, Dtype)>, String> {
    let specs = input_specs(roots, declared);
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
                return Err(format!("unsupported checkpoint dtype {other:?} for `{name}`"));
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

    let mut file = std::fs::File::open(path).map_err(|error| format!("open {}: {error}", path.display()))?;
    let mut header_length = [0u8; 8];
    file.read_exact(&mut header_length)
        .map_err(|error| format!("read {} header: {error}", path.display()))?;
    let data_start = 8 + u64::from_le_bytes(header_length) as usize;
    let pad = data_start.next_multiple_of(4) - data_start;

    const PAGE: usize = 16384;
    let file_length = std::fs::metadata(path).map_err(|error| error.to_string())?.len() as usize;
    let capacity = (pad + file_length).div_ceil(PAGE).max(1) * PAGE;
    let layout = std::alloc::Layout::from_size_align(capacity, PAGE).map_err(|error| error.to_string())?;
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
    bf16_storage: bool,
}

fn usage() -> &'static str {
    "usage: cargo run --release --example llama3_2 -- \"prompt\" -n <tokens> [--bf16]"
}

fn parse_arguments(arguments: impl IntoIterator<Item = String>) -> Result<Arguments, String> {
    let mut prompt = None;
    let mut new_tokens = None;
    let mut bf16_storage = false;
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
            "--bf16" => bf16_storage = true,
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
        bf16_storage,
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

/// One executed step: the index the GPU sampled, the scores it sampled from
/// (read only when something asks), and the replay's GPU seconds.
#[cfg(target_os = "macos")]
struct Step {
    token: sanic::MetalBuffer,
    /// Present only when something asked for the scores; the vocabulary is not
    /// otherwise materialized as an output.
    logits: Option<sanic::MetalBuffer>,
    seconds: f64,
}

#[cfg(target_os = "macos")]
fn run_metal() -> Result<(), String> {
    use std::io::Write;

    let arguments = parse_arguments(std::env::args().skip(1))?;
    let model_dir = cached_model_dir()?;
    let checkpoint = model_dir.join("model.safetensors");
    let tokenizer =
        Tokenizer::from_file(model_dir.join("tokenizer.json")).map_err(|error| format!("load tokenizer: {error}"))?;
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
    let storage = if arguments.bf16_storage {
        Dtype::BF16
    } else {
        Dtype::F32
    };
    let graph = build_decode(Config::LLAMA_3_2_1B, context_length, storage);
    eprintln!("built graph in {:.2}s", started.elapsed().as_secs_f32());

    let started = std::time::Instant::now();
    eprintln!("reading cached BF16 checkpoint...");
    let (checkpoint_tensors, region) = open_checkpoint_zero_copy(&checkpoint)?;
    let roots = graph.roots();
    let specs = validate_checkpoint(&roots, graph.declarations(), &checkpoint_tensors)
        .map_err(|error| format!("invalid cached checkpoint: {error}"))?;
    eprintln!(
        "read and validated checkpoint in {:.2}s",
        started.elapsed().as_secs_f32()
    );

    let device = sanic::MetalDevice::open()
        .ok_or("no Metal device is available")?
        .with_storage(storage);
    let started = std::time::Instant::now();
    eprintln!("compiling decode program...");
    let program = graph.compile_for(&device).map_err(|error| error.to_string())?;
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
        if name.starts_with("cache.") {
            continue; // state buffers belong to the machine
        }
        let (shape, dtype) = specs
            .get(name)
            .ok_or_else(|| format!("compiled input `{name}` has no graph declaration"))?;
        let buffer = if name == "tokens" || name == "position" {
            device
                .tensor_from_raw(
                    device.alloc_elems(shape.iter().product(), *dtype),
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
            .filter(|name| { *name != "tokens" && *name != "position" && !name.starts_with("cache.") })
            .count(),
        zero_copy_bytes as f64 / 1e9,
        started.elapsed().as_secs_f32()
    );

    let started = std::time::Instant::now();
    let mut machine = program
        .instantiate(&device, buffers.iter().map(|(name, buffer)| (name.as_str(), buffer)))
        .map_err(|error| error.to_string())?;
    eprintln!(
        "instantiated {} dispatches in {:.2}s",
        program.kernel_count(),
        started.elapsed().as_secs_f32()
    );

    let want_logits = std::env::var_os("LLAMA3_2_DEBUG_LOGITS").is_some();
    let tokens_buffer = buffers["tokens"].clone();
    let position_buffer = buffers["position"].clone();
    // The step's useful result is the sampled index, not the vocabulary.
    let mut step = |token: u32, position: usize| -> Result<Step, String> {
        device.write_f64(tokens_buffer.raw(), &[token as f64]);
        device.write_f64(position_buffer.raw(), &[position as f64]);
        let seconds = machine.step().map_err(|error| error.to_string())?;
        Ok(Step {
            token: machine.output("token"),
            logits: want_logits.then(|| machine.output("logits")),
            seconds,
        })
    };

    eprintln!("prefilling {} tokens...", prompt_tokens.len());
    let started = std::time::Instant::now();
    let mut last: Option<Step> = None;
    let mut prefill_gpu_seconds = 0.0f64;
    for (position, &token) in prompt_tokens.iter().enumerate() {
        let sampled = step(token, position)?;
        prefill_gpu_seconds += sampled.seconds;
        last = Some(sampled);
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
    // Debug levels print per-step lines on stderr; streaming partial lines
    // to stdout would interleave with them, so hold the text until the end.
    let streaming = std::env::var("SANIC_DEBUG")
        .ok()
        .and_then(|level| level.parse::<u32>().ok())
        .unwrap_or(0)
        < 2;
    let mut held_back = String::new();
    let mut emit = |text: &str| -> Result<(), String> {
        if streaming {
            print!("{text}");
            std::io::stdout().flush().map_err(|error| error.to_string())?;
        } else {
            held_back.push_str(text);
        }
        Ok(())
    };
    emit(&arguments.prompt)?;

    let started = std::time::Instant::now();
    let mut generated = 0usize;
    let mut decode_steps = 0usize;
    let mut decode_gpu_seconds = 0.0f64;
    while generated < arguments.new_tokens {
        let sampled = last.as_ref().expect("a step ran");
        if let Some(scores) = sampled.logits.as_ref() {
            let scores = device.read_tensor_f32(scores);
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
        // one value, chosen on the device
        let next = device.read_tensor_f32(&sampled.token)[0] as u32;
        generated += 1;
        if let Some(text) = stream
            .step(next)
            .map_err(|error| format!("decode token {next}: {error}"))?
        {
            emit(&text)?;
        }
        if next == 128_001 || generated == arguments.new_tokens {
            break;
        }
        let sampled = step(next, prompt_tokens.len() + generated - 1)?;
        decode_gpu_seconds += sampled.seconds;
        decode_steps += 1;
        last = Some(sampled);
    }
    if streaming {
        println!();
    } else {
        println!("{held_back}");
    }
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
    use sanic::interp::{Env, eval};
    use sanic::{Compile, CpuDevice};

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
        let graph = build_decode(config, 3, Dtype::F32);
        // The two caches are laid out DIFFERENTLY on purpose, and this pins it:
        // each stores the axis its fold contracts innermost. K is contracted
        // over head_dim, so it is [kv_heads, cache_sequence, head_dim]; V is
        // contracted over cache_sequence, so it is [kv_heads, head_dim,
        // cache_sequence]. Here that is [2, 3, 2] against [2, 2, 3].
        //
        // The step's only other root is the sampled index — one number, not the
        // 16-entry vocabulary it was chosen from. `logits` is a root only when
        // LLAMA3_2_DEBUG_LOGITS asks for the scores.
        let roots = [vec![2, 3, 2], vec![2, 2, 3], vec![1]];
        assert_eq!(
            graph
                .roots()
                .iter()
                .map(|root| root.shape().into_iter().map(Axis::extent).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            roots
        );

        let program = graph.roots().compile(&CpuDevice::new()).unwrap();
        assert_eq!(program.output_shapes(), &roots);
        assert_eq!(program.input_names().len(), 15);
    }

    #[test]
    fn command_line_is_prompt_then_token_count() {
        let args = parse_arguments(["hello".into(), "-n".into(), "7".into()]).unwrap();
        assert_eq!(args.prompt, "hello");
        assert_eq!(args.new_tokens, 7);
    }
}
