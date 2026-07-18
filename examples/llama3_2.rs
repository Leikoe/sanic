//! Llama 3.2 1B graph construction.
//!
//! ```text
//! cargo run --example llama3_2
//! ```
//!
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

struct Llama3_2 {
    graph: Graph,
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

fn rms_norm(x: TensorExpr, weight: TensorExpr, hidden_dim: Axis) -> TensorExpr {
    let mean_square = (&x * &x).sum(hidden_dim) / hidden_dim.extent() as f64;
    x * weight / (mean_square + EPS).sqrt()
}

fn llama3_inv_freq(frequency: Axis) -> TensorExpr {
    let inv_freq = (TensorExpr::iota(frequency)
        * (-(ROPE_THETA.ln()) / (frequency.extent() * 2) as f64))
        .exp();
    let wave_length = (2.0 * std::f64::consts::PI) / inv_freq.clone();
    let low_wave_length = ROPE_ORIGINAL_CONTEXT / ROPE_LOW_FREQ_FACTOR;
    let high_wave_length = ROPE_ORIGINAL_CONTEXT / ROPE_HIGH_FREQ_FACTOR;
    let smooth = (ROPE_ORIGINAL_CONTEXT / wave_length.clone() - ROPE_LOW_FREQ_FACTOR)
        / (ROPE_HIGH_FREQ_FACTOR - ROPE_LOW_FREQ_FACTOR);
    let scaled = inv_freq.clone() / ROPE_FACTOR;
    let blended = (1.0 - smooth.clone()) * scaled.clone() + smooth * inv_freq.clone();

    // HF's `llama3` RoPE scaling: low frequencies are divided by 32, high
    // frequencies are unchanged, and the band between them is interpolated.
    TensorExpr::scalar(low_wave_length).lt(&wave_length).select(
        &scaled,
        &wave_length
            .lt(&TensorExpr::scalar(high_wave_length))
            .select(&inv_freq, &blended),
    )
}

fn rope(x: TensorExpr, position: Axis, head_dim: Axis) -> TensorExpr {
    let (x, half, frequency) = x.split(
        head_dim,
        "rope_half",
        "rope_frequency",
        head_dim.extent() / 2,
    );
    let inv_freq = llama3_inv_freq(frequency);
    let angle = TensorExpr::iota(position) * inv_freq;
    let sign = TensorExpr::iota(half) * 2.0 - 1.0;
    let rotated = x.flip(half) * sign;
    let rotated = x * angle.clone().cos() + rotated * angle.sin();
    rotated.flatten(&[half, frequency], "head_dim").0
}

fn block(graph: &mut GraphBuilder, axes: &Axes, layer: usize, x: TensorExpr) -> TensorExpr {
    let name = |suffix: &str| format!("model.layers.{layer}.{suffix}");
    let attn_input = rms_norm(
        x.clone(),
        graph.input(
            name("input_layernorm.weight"),
            [axes.hidden_dim],
            Dtype::BF16,
        ),
        axes.hidden_dim,
    );
    let q_weight = graph.input(
        name("self_attn.q_proj.weight"),
        [
            axes.kv_head,
            axes.query_group,
            axes.head_dim,
            axes.hidden_dim,
        ],
        Dtype::BF16,
    );
    let q = rope(
        attn_input.matmul(&q_weight, axes.hidden_dim),
        axes.sequence,
        axes.head_dim,
    );
    let key_input = attn_input.rename(axes.sequence, axes.key_sequence);
    let k_weight = graph.input(
        name("self_attn.k_proj.weight"),
        [axes.kv_head, axes.head_dim, axes.hidden_dim],
        Dtype::BF16,
    );
    let k = rope(
        key_input.clone().matmul(&k_weight, axes.hidden_dim),
        axes.key_sequence,
        axes.head_dim,
    );
    let v = key_input.matmul(
        &graph.input(
            name("self_attn.v_proj.weight"),
            [axes.kv_head, axes.head_dim, axes.hidden_dim],
            Dtype::BF16,
        ),
        axes.hidden_dim,
    );
    let scores = q.matmul(&k, axes.head_dim) / (axes.head_dim.extent() as f64).sqrt();
    let attention = (scores + TensorExpr::causal_mask(axes.sequence, axes.key_sequence))
        .softmax(axes.key_sequence)
        .matmul(&v, axes.key_sequence);
    let (attention, packed_heads) = attention.flatten(
        &[axes.kv_head, axes.query_group, axes.head_dim],
        "packed_heads",
    );
    let attention = attention.matmul(
        &graph.input(
            name("self_attn.o_proj.weight"),
            [axes.hidden_dim, packed_heads],
            Dtype::BF16,
        ),
        packed_heads,
    );
    let residual = x + attention;

    let mlp_input = rms_norm(
        residual.clone(),
        graph.input(
            name("post_attention_layernorm.weight"),
            [axes.hidden_dim],
            Dtype::BF16,
        ),
        axes.hidden_dim,
    );
    let gate = mlp_input.matmul(
        &graph.input(
            name("mlp.gate_proj.weight"),
            [axes.intermediate, axes.hidden_dim],
            Dtype::BF16,
        ),
        axes.hidden_dim,
    );
    let up = mlp_input.matmul(
        &graph.input(
            name("mlp.up_proj.weight"),
            [axes.intermediate, axes.hidden_dim],
            Dtype::BF16,
        ),
        axes.hidden_dim,
    );
    let down = (gate.silu() * up).matmul(
        &graph.input(
            name("mlp.down_proj.weight"),
            [axes.hidden_dim, axes.intermediate],
            Dtype::BF16,
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
        let tokens = graph.input("tokens", &[axes.sequence], Dtype::F32);
        let embedding = graph.input(
            "model.embed_tokens.weight",
            [axes.vocab, axes.hidden_dim],
            Dtype::BF16,
        );
        let mut x = embedding.embedding(&tokens, axes.vocab);
        for layer in 0..config.layers {
            x = block(&mut graph, &axes, layer, x);
        }
        let x = rms_norm(
            x,
            graph.input("model.norm.weight", [axes.hidden_dim], Dtype::BF16),
            axes.hidden_dim,
        );

        // `tie_word_embeddings` is true: reuse the embedding expression,
        // rather than inserting a second input for a nonexistent lm_head.
        let logits = x.matmul(&embedding, axes.hidden_dim);
        Self {
            graph: graph.finish([logits]),
        }
    }
}

fn main() {
    let model = Llama3_2::build(1);

    println!(
        "Llama 3.2 1B: {} graph inputs, output {:?}",
        model.graph.input_count(),
        model.graph.output_shapes()
    );
}
