use std::collections::HashMap;

use sanic::{Dtype, Extent, GraphBuilder, Tensor, TensorExpr, axis};

fn bindings(inputs: impl IntoIterator<Item = (&'static str, Tensor)>) -> HashMap<String, Tensor> {
    inputs
        .into_iter()
        .map(|(name, tensor)| (name.to_owned(), tensor))
        .collect()
}

fn linear(
    x: TensorExpr,
    weight: TensorExpr,
    bias: Option<TensorExpr>,
    contract: sanic::Axis,
) -> TensorExpr {
    let output = x.matmul(&weight, contract);
    match bias {
        Some(bias) => output + bias,
        None => output,
    }
}

#[test]
fn function_builds_a_reusable_graph() {
    let (batch, input, output) = (axis("batch", 2), axis("input", 3), axis("output", 2));
    let mut builder = GraphBuilder::new();
    let x = builder.input("x", &[batch, input], Dtype::F64);
    let weight = builder.input("weight", &[output, input], Dtype::F64);
    let bias = builder.input("bias", &[output], Dtype::F64);
    let graph = builder.finish([linear(x, weight, Some(bias), input)]);

    assert_eq!(graph.input_count(), 3);
    assert_eq!(graph.output_count(), 1);

    let weights = Tensor::from_fn(&[output, input], |coord| {
        [[1.0, 0.0, -1.0], [0.5, 2.0, 0.0]][coord[&output]][coord[&input]]
    });
    let bias = Tensor::from_fn(&[output], |coord| [0.25, -0.5][coord[&output]]);

    let first = Tensor::from_fn(&[batch, input], |coord| {
        [[2.0, 3.0, 5.0], [7.0, 11.0, 13.0]][coord[&batch]][coord[&input]]
    });
    let second = Tensor::from_fn(&[batch, input], |coord| {
        [[1.0, 1.0, 1.0], [4.0, 0.0, -2.0]][coord[&batch]][coord[&input]]
    });

    let first = graph.run(&bindings([
        ("x", first),
        ("weight", weights.clone()),
        ("bias", bias.clone()),
    ]));
    let second = graph.run(&bindings([
        ("x", second),
        ("weight", weights),
        ("bias", bias),
    ]));

    assert_eq!(first[0].shape, vec![2, 2]);
    assert_eq!(first[0].data, vec![-2.75, 6.5, -5.75, 25.0]);
    assert_eq!(second[0].data, vec![0.25, 2.0, 6.25, 1.5]);
}

#[test]
#[should_panic(expected = "declared more than once")]
fn input_names_must_be_unique() {
    let a = axis("a", 2);
    let mut builder = GraphBuilder::new();
    let _ = builder.input("x", &[a], Dtype::F64);
    let _ = builder.input("x", &[a], Dtype::F64);
}

#[test]
fn graph_keeps_multiple_declared_outputs() {
    let n = axis("n", 3);
    let mut builder = GraphBuilder::new();
    let x = builder.input("x", &[n], Dtype::F64);
    let graph = builder.finish([x.sum(n), x.prod(n)]);
    let input = Tensor::from_fn(&[n], |coord| [2.0, 3.0, 5.0][coord[&n]]);

    let outputs = graph.run(&bindings([("x", input)]));
    assert_eq!(outputs.len(), 2);
    assert_eq!(outputs[0].data, vec![10.0]);
    assert_eq!(outputs[1].data, vec![30.0]);
}

#[test]
fn composed_operations_build_from_tensor_expr_receivers() {
    let (vocab, sequence, feature) = (axis("vocab", 4), axis("sequence", 2), axis("feature", 2));
    let mut builder = GraphBuilder::new();
    let table = builder.input("table", &[vocab, feature], Dtype::F64);
    let ids = builder.input("ids", &[sequence], Dtype::F64);
    let scores = builder.input("scores", &[vocab], Dtype::F64);
    let embedding = table.embedding(&ids, vocab);
    let one_hot = ids.one_hot(vocab);
    let (best, best_index) = scores.topk(vocab, 1).pop().unwrap();
    let graph = builder.finish([embedding, one_hot, best, best_index]);

    let table = Tensor::from_fn(&[vocab, feature], |coord| {
        [[1.0, 2.0], [3.0, 5.0], [7.0, 11.0], [13.0, 17.0]][coord[&vocab]][coord[&feature]]
    });
    let ids = Tensor::from_fn(&[sequence], |coord| [2.0, 0.0][coord[&sequence]]);
    let scores = Tensor::from_fn(&[vocab], |coord| [1.0, 9.0, 9.0, 2.0][coord[&vocab]]);
    let outputs = graph.run(&bindings([
        ("table", table),
        ("ids", ids),
        ("scores", scores),
    ]));

    assert_eq!(outputs[0].data, vec![7.0, 1.0, 11.0, 2.0]);
    assert_eq!(
        outputs[1].data,
        vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]
    );
    assert_eq!(outputs[2].data, vec![9.0]);
    assert_eq!(outputs[3].data, vec![1.0]);
}

#[test]
fn sigmoid_is_composed_from_tensor_expr_operations() {
    let n = axis("n", 3);
    let mut builder = GraphBuilder::new();
    let x = builder.input("x", &[n], Dtype::F64);
    let graph = builder.finish([x.sigmoid()]);
    let input = Tensor::from_fn(&[n], |coord| [-1.0, 0.0, 1.0][coord[&n]]);

    let output = graph.run(&bindings([("x", input)]));
    let expected = [0.268_941_421_369_995_1, 0.5, 0.731_058_578_630_004_9];
    for (got, expected) in output[0].data.iter().zip(expected) {
        assert!((got - expected).abs() < 1e-12);
    }
}

#[test]
fn dynamic_extent_is_resolved_for_each_run() {
    let sequence = axis("sequence", Extent::Dynamic);
    let mut builder = GraphBuilder::new();
    let x = builder.input("x", [sequence], Dtype::F64);
    let graph = builder.finish([&x * 2.0, x.sum(sequence)]);

    assert_eq!(graph.output_shapes()[0].axes()[0].extent, Extent::Dynamic);
    assert_eq!(graph.output_shapes()[0].rank(), 1);
    assert_eq!(graph.output_shapes()[0].element_count(), None);

    let run = |data: Vec<f64>| {
        let input = Tensor::from_data(&[sequence], vec![data.len()], data);
        graph.run(&bindings([("x", input)]))
    };

    let two = run(vec![2.0, 3.0]);
    assert_eq!(two[0].shape, vec![2]);
    assert_eq!(two[0].data, vec![4.0, 6.0]);
    assert_eq!(two[1].data, vec![5.0]);

    let four = run(vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(four[0].shape, vec![4]);
    assert_eq!(four[0].data, vec![2.0, 4.0, 6.0, 8.0]);
    assert_eq!(four[1].data, vec![10.0]);
}

#[test]
#[should_panic(expected = "was not bound")]
fn run_reports_missing_named_bindings() {
    let n = axis("n", 2);
    let mut builder = GraphBuilder::new();
    let x = builder.input("x", &[n], Dtype::F64);
    let y = builder.input("y", &[n], Dtype::F64);
    let graph = builder.finish([x + y]);
    let x = Tensor::from_fn(&[n], |coord| coord[&n] as f64);

    let _ = graph.run(&bindings([("x", x)]));
}

#[test]
#[should_panic(expected = "has no input named")]
fn run_rejects_unknown_named_bindings() {
    let n = axis("n", 2);
    let mut builder = GraphBuilder::new();
    let x = builder.input("x", &[n], Dtype::F64);
    let graph = builder.finish([x]);
    let input = Tensor::from_fn(&[n], |coord| coord[&n] as f64);

    let _ = graph.run(&bindings([("unknown", input)]));
}

#[test]
#[should_panic(expected = "different GraphBuilder")]
fn finish_rejects_a_foreign_input() {
    let n = axis("n", 2);
    let mut left = GraphBuilder::new();
    let x = left.input("x", &[n], Dtype::F64);
    let mut right = GraphBuilder::new();
    let y = right.input("x", &[n], Dtype::F64);

    let _ = left.finish([x + y]);
}

#[test]
fn flip_is_an_affine_view_over_one_axis() {
    let n = axis("n", 4);
    let mut builder = GraphBuilder::new();
    let x = builder.input("x", &[n], Dtype::F64);
    let graph = builder.finish([x.flip(n)]);

    let input = Tensor::from_fn(&[n], |coord| [2.0, 3.0, 5.0, 7.0][coord[&n]]);
    assert_eq!(
        graph.run(&bindings([("x", input)]))[0].data,
        vec![7.0, 5.0, 3.0, 2.0]
    );
}
