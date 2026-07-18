//! Llama 3.2 1B: explicit graph construction and checkpoint binding.
//!
//! ```text
//! cargo run --example llama3_2
//! ```
//!
//! By default the example finds the checkpoint in Hugging Face's cache. Set
//! `SANIC_LLAMA3_2_DIR` to point at a different snapshot directory.

use std::path::{Path, PathBuf};

use sanic::safetensors::StFile;
use sanic::{Axis, Dtype, Graph, GraphBuilder, TensorExpr, axis};

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

struct Binding {
    checkpoint_name: String,
    axes: Vec<Axis>,
}

struct Llama3_2 {
    graph: Graph,
    bindings: Vec<Binding>,
}

struct Axes {
    vocab: Axis,
    sequence: Axis,
    key_sequence: Axis,
    hidden_dim: Axis,
    kv_head: Axis,
    query_group: Axis,
    head_dim: Axis,
    intermediate: Axis,
}

impl Axes {
    fn new(config: Config, sequence_length: usize) -> Self {
        assert_eq!(config.query_heads % config.kv_heads, 0);
        assert_eq!(config.hidden_dim, config.query_heads * config.head_dim);
        Self {
            vocab: axis("vocab", config.vocab_size),
            sequence: axis("sequence", sequence_length),
            key_sequence: axis("key_sequence", sequence_length),
            hidden_dim: axis("hidden_dim", config.hidden_dim),
            kv_head: axis("kv_head", config.kv_heads),
            query_group: axis("query_group", config.query_heads / config.kv_heads),
            head_dim: axis("head_dim", config.head_dim),
            intermediate: axis("intermediate", config.intermediate_dim),
        }
    }
}

fn parameter(
    graph: &mut GraphBuilder,
    bindings: &mut Vec<Binding>,
    checkpoint_name: impl Into<String>,
    axes: &[Axis],
) -> TensorExpr {
    let checkpoint_name = checkpoint_name.into();
    let tensor = graph.input_dt(checkpoint_name.clone(), axes, Dtype::BF16);
    bindings.push(Binding {
        checkpoint_name,
        axes: axes.to_vec(),
    });
    tensor
}

fn rms_norm(x: TensorExpr, weight: TensorExpr, hidden_dim: Axis) -> TensorExpr {
    let mean_square = (&x * &x).sum(hidden_dim) / hidden_dim.extent as f64;
    x * weight / (mean_square + EPS).sqrt()
}

fn llama3_inv_freq(graph: &GraphBuilder, frequency: Axis) -> TensorExpr {
    let inv_freq =
        (graph.iota(frequency) * (-(ROPE_THETA.ln()) / (frequency.extent * 2) as f64)).exp();
    let wave_length = (2.0 * std::f64::consts::PI) / inv_freq.clone();
    let low_wave_length = ROPE_ORIGINAL_CONTEXT / ROPE_LOW_FREQ_FACTOR;
    let high_wave_length = ROPE_ORIGINAL_CONTEXT / ROPE_HIGH_FREQ_FACTOR;
    let smooth = (ROPE_ORIGINAL_CONTEXT / wave_length.clone() - ROPE_LOW_FREQ_FACTOR)
        / (ROPE_HIGH_FREQ_FACTOR - ROPE_LOW_FREQ_FACTOR);
    let scaled = inv_freq.clone() / ROPE_FACTOR;
    let blended = (1.0 - smooth.clone()) * scaled.clone() + smooth * inv_freq.clone();

    // HF's `llama3` RoPE scaling: low frequencies are divided by 32, high
    // frequencies are unchanged, and the band between them is interpolated.
    graph.scalar(low_wave_length).lt(&wave_length).select(
        &scaled,
        &wave_length
            .lt(&graph.scalar(high_wave_length))
            .select(&inv_freq, &blended),
    )
}

fn rope(graph: &GraphBuilder, x: TensorExpr, position: Axis, head_dim: Axis) -> TensorExpr {
    let (x, half, frequency) =
        x.split(head_dim, "rope_half", "rope_frequency", head_dim.extent / 2);
    let inv_freq = llama3_inv_freq(graph, frequency);
    let angle = graph.iota(position) * inv_freq;
    let sign = graph.iota(half) * 2.0 - 1.0;
    let rotated = x.flip(half) * sign;
    let rotated = x * angle.clone().cos() + rotated * angle.sin();
    rotated.flatten(&[half, frequency], "head_dim").0
}

fn block(
    graph: &mut GraphBuilder,
    bindings: &mut Vec<Binding>,
    axes: &Axes,
    layer: usize,
    x: TensorExpr,
) -> TensorExpr {
    let name = |suffix: &str| format!("model.layers.{layer}.{suffix}");
    let attn_input = rms_norm(
        x.clone(),
        parameter(
            graph,
            bindings,
            name("input_layernorm.weight"),
            &[axes.hidden_dim],
        ),
        axes.hidden_dim,
    );
    let q_weight = parameter(
        graph,
        bindings,
        name("self_attn.q_proj.weight"),
        &[
            axes.kv_head,
            axes.query_group,
            axes.head_dim,
            axes.hidden_dim,
        ],
    );
    let q = rope(
        graph,
        attn_input.matmul(&q_weight, axes.hidden_dim),
        axes.sequence,
        axes.head_dim,
    );
    let key_input = attn_input.rename(axes.sequence, axes.key_sequence);
    let k_weight = parameter(
        graph,
        bindings,
        name("self_attn.k_proj.weight"),
        &[axes.kv_head, axes.head_dim, axes.hidden_dim],
    );
    let k = rope(
        graph,
        key_input.clone().matmul(&k_weight, axes.hidden_dim),
        axes.key_sequence,
        axes.head_dim,
    );
    let v = key_input.matmul(
        &parameter(
            graph,
            bindings,
            name("self_attn.v_proj.weight"),
            &[axes.kv_head, axes.head_dim, axes.hidden_dim],
        ),
        axes.hidden_dim,
    );
    let scores = q.matmul(&k, axes.head_dim) / (axes.head_dim.extent as f64).sqrt();
    let attention = (scores + graph.causal_mask(axes.sequence, axes.key_sequence))
        .softmax(axes.key_sequence)
        .matmul(&v, axes.key_sequence);
    let (attention, packed_heads) = attention.flatten(
        &[axes.kv_head, axes.query_group, axes.head_dim],
        "packed_heads",
    );
    let attention = attention.matmul(
        &parameter(
            graph,
            bindings,
            name("self_attn.o_proj.weight"),
            &[axes.hidden_dim, packed_heads],
        ),
        packed_heads,
    );
    let residual = x + attention;

    let mlp_input = rms_norm(
        residual.clone(),
        parameter(
            graph,
            bindings,
            name("post_attention_layernorm.weight"),
            &[axes.hidden_dim],
        ),
        axes.hidden_dim,
    );
    let gate = mlp_input.matmul(
        &parameter(
            graph,
            bindings,
            name("mlp.gate_proj.weight"),
            &[axes.intermediate, axes.hidden_dim],
        ),
        axes.hidden_dim,
    );
    let up = mlp_input.matmul(
        &parameter(
            graph,
            bindings,
            name("mlp.up_proj.weight"),
            &[axes.intermediate, axes.hidden_dim],
        ),
        axes.hidden_dim,
    );
    let down = (gate.silu() * up).matmul(
        &parameter(
            graph,
            bindings,
            name("mlp.down_proj.weight"),
            &[axes.hidden_dim, axes.intermediate],
        ),
        axes.intermediate,
    );
    residual + down
}

impl Llama3_2 {
    fn build(sequence_length: usize) -> Self {
        let config = Config::LLAMA_3_2_1B;
        let axes = Axes::new(config, sequence_length);
        let mut graph = GraphBuilder::new();
        let tokens = graph.input("tokens", &[axes.sequence]);
        let mut bindings = Vec::new();
        let embedding = parameter(
            &mut graph,
            &mut bindings,
            "model.embed_tokens.weight",
            &[axes.vocab, axes.hidden_dim],
        );
        let mut x = embedding.gather(&tokens, axes.vocab);
        for layer in 0..config.layers {
            x = block(&mut graph, &mut bindings, &axes, layer, x);
        }
        let x = rms_norm(
            x,
            parameter(
                &mut graph,
                &mut bindings,
                "model.norm.weight",
                &[axes.hidden_dim],
            ),
            axes.hidden_dim,
        );

        // `tie_word_embeddings` is true: reuse the embedding expression,
        // rather than inserting a second input for a nonexistent lm_head.
        let logits = x.matmul(&embedding, axes.hidden_dim);
        Self {
            graph: graph.finish([logits]),
            bindings,
        }
    }

    fn verify_checkpoint(&self, checkpoint: &StFile) {
        for binding in &self.bindings {
            let (dtype, shape) = checkpoint.meta(&binding.checkpoint_name);
            let expected_elements: usize = binding.axes.iter().map(|axis| axis.extent).product();
            let actual_elements: usize = shape.iter().product();
            assert_eq!(
                dtype, "BF16",
                "{} has unexpected dtype",
                binding.checkpoint_name
            );
            assert_eq!(
                actual_elements, expected_elements,
                "{} has an incompatible shape {:?}",
                binding.checkpoint_name, shape
            );
        }
    }
}

fn default_model_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("SANIC_LLAMA3_2_DIR") {
        return dir.into();
    }
    let cache = std::env::var_os("HF_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache/huggingface"))
        })
        .expect("set HF_HOME or HOME so the Hugging Face cache can be found");
    let snapshots = cache
        .join("hub")
        .join("models--meta-llama--Llama-3.2-1B")
        .join("snapshots");
    std::fs::read_dir(&snapshots)
        .unwrap_or_else(|_| panic!("no Llama 3.2 1B snapshot in {}", snapshots.display()))
        .flatten()
        .map(|entry| entry.path())
        .find(|path| path.join("model.safetensors").is_file())
        .expect("no snapshot contains model.safetensors")
}

fn main() {
    let model_dir = default_model_dir();
    let checkpoint_path = model_dir.join("model.safetensors");
    let checkpoint = StFile::open(Path::new(&checkpoint_path)).expect("open Llama 3.2 checkpoint");
    let model = Llama3_2::build(1);
    model.verify_checkpoint(&checkpoint);

    println!(
        "Llama 3.2 1B: {} graph inputs ({} checkpoint parameters), output {:?}",
        model.graph.input_count(),
        model.bindings.len(),
        model.graph.output_shapes()
    );
    println!(
        "checkpoint bindings verified at {}",
        checkpoint_path.display()
    );
}
