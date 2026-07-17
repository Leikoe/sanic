//! A compact Llama 3 graph built with Sanic's explicit frontend.
//!
//! Model helpers consume and return `TensorExpr`: they build a graph, never
//! execute data. `GraphBuilder::finish` freezes the output roots into a
//! reusable `Graph`, whose inputs are later bound by dense input ID order.
//!
//!     cargo run --example llama3

use sanic::{Axis, Graph, GraphBuilder, TensorExpr, axis};

#[derive(Clone, Copy)]
struct Llama3Config {
    vocab_size: usize,
    sequence_length: usize,
    layers: usize,
    model_width: usize,
    query_heads: usize,
    kv_heads: usize,
    head_dim: usize,
    ffn_width: usize,
}

impl Llama3Config {
    fn frontend_fixture() -> Self {
        Llama3Config {
            vocab_size: 128_256,
            sequence_length: 2_048,
            // The graph has two layers while frontend ergonomics settles.
            layers: 2,
            model_width: 4_096,
            query_heads: 32,
            kv_heads: 8,
            head_dim: 128,
            ffn_width: 14_336,
        }
    }
}

struct Llama3 {
    config: Llama3Config,
    vocab: Axis,
    sequence: Axis,
    key_sequence: Axis,
    model: Axis,
    kv_head: Axis,
    query_group: Axis,
    head_dim: Axis,
    ffn: Axis,
}

impl Llama3 {
    fn new(config: Llama3Config) -> Self {
        assert_eq!(config.query_heads % config.kv_heads, 0);
        assert_eq!(config.model_width, config.query_heads * config.head_dim);

        Llama3 {
            vocab: axis("vocab", config.vocab_size),
            sequence: axis("sequence", config.sequence_length),
            key_sequence: axis("key_sequence", config.sequence_length),
            model: axis("model", config.model_width),
            kv_head: axis("kv_head", config.kv_heads),
            query_group: axis("query_group", config.query_heads / config.kv_heads),
            head_dim: axis("head_dim", config.head_dim),
            ffn: axis("ffn", config.ffn_width),
            config,
        }
    }

    fn parameter(&self, graph: &mut GraphBuilder, axes: &[Axis]) -> TensorExpr {
        graph.input(axes)
    }

    fn rms_norm(&self, x: TensorExpr, gain: TensorExpr) -> TensorExpr {
        let mean_square = (&x * &x).sum(self.model) / self.model.extent as f64;
        (x * gain) / (mean_square + 1e-5).sqrt()
    }

    fn block(&self, graph: &mut GraphBuilder, x: TensorExpr) -> TensorExpr {
        let attn_norm = self.rms_norm(x.clone(), self.parameter(graph, &[self.model]));
        let attn_kv = attn_norm.rename(self.sequence, self.key_sequence);

        // A K/V head and its query-head group are distinct named axes, so GQA
        // is ordinary contraction and broadcasting rather than a special op.
        let q = attn_norm.matmul(
            &self.parameter(
                graph,
                &[self.kv_head, self.query_group, self.head_dim, self.model],
            ),
            self.model,
        );
        let k = attn_kv.clone().matmul(
            &self.parameter(graph, &[self.kv_head, self.head_dim, self.model]),
            self.model,
        );
        let v = attn_kv.matmul(
            &self.parameter(graph, &[self.kv_head, self.head_dim, self.model]),
            self.model,
        );
        let scores = q.matmul(&k, self.head_dim) / (self.head_dim.extent as f64).sqrt();
        let attention = (scores + graph.causal_mask(self.sequence, self.key_sequence))
            .softmax(self.key_sequence)
            .matmul(&v, self.key_sequence);

        let (attention, packed) = attention.flatten(
            &[self.kv_head, self.query_group, self.head_dim],
            "attention_packed",
        );
        let attention_out = attention.matmul(&self.parameter(graph, &[self.model, packed]), packed);
        let residual = attention_out + x;

        let mlp_norm = self.rms_norm(residual.clone(), self.parameter(graph, &[self.model]));
        let gate = mlp_norm.matmul(&self.parameter(graph, &[self.ffn, self.model]), self.model);
        let up = mlp_norm.matmul(&self.parameter(graph, &[self.ffn, self.model]), self.model);
        let down =
            (gate.silu() * up).matmul(&self.parameter(graph, &[self.model, self.ffn]), self.ffn);
        down + residual
    }

    fn build(&self) -> Graph {
        let mut graph = GraphBuilder::new();
        let tokens = graph.input(&[self.sequence]);
        let embedding = self.parameter(&mut graph, &[self.vocab, self.model]);
        let mut x = embedding.gather(&tokens, self.vocab);

        for _ in 0..self.config.layers {
            x = self.block(&mut graph, x);
        }

        let x = self.rms_norm(x, self.parameter(&mut graph, &[self.model]));
        let logits = x.matmul(
            &self.parameter(&mut graph, &[self.vocab, self.model]),
            self.model,
        );
        graph.finish([logits])
    }
}

fn main() {
    let model = Llama3::new(Llama3Config::frontend_fixture());
    let graph = model.build();
    println!(
        "Llama 3 graph: {} inputs, output shapes {:?}",
        graph.input_count(),
        graph.output_shapes()
    );
}
