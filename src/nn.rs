//! Neural-network functions composed from the positional tensor IR.

/// Stateless neural-network operations.
pub mod functional {
    use crate::ir::{
        Axis, Extent, MapOp, NodeRef, causal_mask_like, flatten, konst, map, matmul,
        positional_reindex, softmax, transpose,
    };

    /// Scaled dot-product attention with the same argument order and tensor
    /// layout as `torch.nn.functional.scaled_dot_product_attention`.
    ///
    /// Shapes are:
    ///
    /// - query: `[N, ..., Hq, L, E]`
    /// - key: `[N, ..., H, S, E]`
    /// - value: `[N, ..., H, S, Ev]`
    /// - output: `[N, ..., Hq, L, Ev]`
    ///
    /// `attn_mask` is currently an additive floating-point mask broadcastable
    /// to `[N, ..., Hq, L, S]`. Boolean mask storage and stochastic graph
    /// operations have not landed yet, so `dropout_p` must currently be zero.
    ///
    /// PyTorch makes `scale` and `enable_gqa` keyword-only. Rust has no keyword
    /// or default arguments, so callers pass all eight arguments explicitly.
    #[allow(clippy::too_many_arguments)]
    pub fn scaled_dot_product_attention(
        query: NodeRef,
        mut key: NodeRef,
        mut value: NodeRef,
        attn_mask: Option<NodeRef>,
        dropout_p: f64,
        is_causal: bool,
        scale: Option<f64>,
        enable_gqa: bool,
    ) -> NodeRef {
        assert!(
            (0.0..=1.0).contains(&dropout_p),
            "scaled_dot_product_attention: dropout_p must be between 0 and 1"
        );
        assert_eq!(
            dropout_p, 0.0,
            "scaled_dot_product_attention: nonzero dropout is not supported yet"
        );
        assert!(
            !(is_causal && attn_mask.is_some()),
            "scaled_dot_product_attention: attn_mask and is_causal cannot both be set"
        );

        let query_shape = query.shape();
        let key_shape = key.shape();
        let value_shape = value.shape();
        assert!(
            query_shape.len() >= 2 && key_shape.len() >= 2 && value_shape.len() >= 2,
            "scaled_dot_product_attention: query, key, and value must have rank >= 2"
        );

        let query_features = query_shape[query_shape.len() - 1];
        let key_features = key_shape[key_shape.len() - 1];
        let key_sequence = key_shape[key_shape.len() - 2];
        let value_sequence = value_shape[value_shape.len() - 2];
        assert_eq!(
            query_features.extent, key_features.extent,
            "scaled_dot_product_attention: query and key feature extents must match"
        );
        assert_eq!(
            key_sequence.extent, value_sequence.extent,
            "scaled_dot_product_attention: key and value sequence extents must match"
        );

        if enable_gqa {
            assert!(
                query_shape.len() >= 3 && key_shape.len() >= 3 && value_shape.len() >= 3,
                "scaled_dot_product_attention: GQA requires query, key, and value rank >= 3"
            );
            let query_heads = query_shape[query_shape.len() - 3];
            let key_heads = key_shape[key_shape.len() - 3];
            let value_heads = value_shape[value_shape.len() - 3];
            assert_eq!(
                key_heads.extent, value_heads.extent,
                "scaled_dot_product_attention: key and value head extents must match for GQA"
            );
            key = repeat_interleave_heads(key, query_heads);
            value = repeat_interleave_heads(value, query_heads);
        }

        let scale = scale.unwrap_or_else(|| match query_features.extent {
            Extent::Static(features) => 1.0 / (features as f64).sqrt(),
            Extent::Dynamic => panic!(
                "scaled_dot_product_attention: default scale requires a static feature extent"
            ),
        });

        let key = transpose(key, -2isize, -1isize);
        let mut scores = map(MapOp::Mul, vec![matmul(query, key), konst(scale)]);

        if is_causal {
            scores = map(
                MapOp::Add,
                vec![scores.clone(), causal_mask_like(scores, -2isize, -1isize)],
            );
        } else if let Some(mask) = attn_mask {
            scores = map(MapOp::Add, vec![scores, mask]);
        }

        matmul(softmax(scores, -1isize), value)
    }

    fn repeat_interleave_heads(src: NodeRef, target: Axis) -> NodeRef {
        let source = src.shape();
        let head_dim = source.len() - 3;
        let source_heads = source[head_dim];
        let (source_extent, target_extent) = match (source_heads.extent, target.extent) {
            (Extent::Static(source), Extent::Static(target)) => (source, target),
            _ => panic!("scaled_dot_product_attention: GQA currently requires static head extents"),
        };
        assert!(
            target_extent % source_extent == 0,
            "scaled_dot_product_attention: query heads must be divisible by key/value heads"
        );
        let repeats = target_extent / source_extent;
        if repeats == 1 {
            return src;
        }

        let repeat = Axis {
            name: "gqa_repeat",
            extent: Extent::Static(repeats),
        };
        let mut expanded_shape = source.clone();
        expanded_shape.insert(head_dim + 1, repeat);
        let map = source
            .iter()
            .enumerate()
            .map(|(source_dim, _)| {
                let output_dim = if source_dim <= head_dim {
                    source_dim
                } else {
                    source_dim + 1
                };
                (source_dim, vec![(1, output_dim)], 0)
            })
            .collect();
        let expanded = positional_reindex(src, expanded_shape, map, false);
        flatten(
            expanded,
            &[head_dim, head_dim + 1][..],
            Axis {
                name: target.name,
                extent: target.extent,
            },
        )
    }
}

pub use functional::scaled_dot_product_attention;
