use sanic::{GraphBuilder, Tensor, TensorExpr, axis};

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
    let x = builder.input(&[batch, input]);
    let weight = builder.input(&[output, input]);
    let bias = builder.input(&[output]);
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

    let first = graph.run([first, weights.clone(), bias.clone()]);
    let second = graph.run([second, weights, bias]);

    assert_eq!(first[0].shape, vec![2, 2]);
    assert_eq!(first[0].data, vec![-2.75, 6.5, -5.75, 25.0]);
    assert_eq!(second[0].data, vec![0.25, 2.0, 6.25, 1.5]);
}

#[test]
fn input_ids_are_dense_and_follow_construction_order() {
    let a = axis("a", 2);
    let mut builder = GraphBuilder::new();
    let first = builder.input(&[a]);
    let second = builder.input(&[a]);
    let third = builder.input(&[a]);

    assert_eq!(first.input_id().unwrap().index(), 0);
    assert_eq!(second.input_id().unwrap().index(), 1);
    assert_eq!(third.input_id().unwrap().index(), 2);
    assert!(first.clone().exp().input_id().is_none());
}

#[test]
fn graph_keeps_multiple_declared_outputs() {
    let n = axis("n", 3);
    let mut builder = GraphBuilder::new();
    let x = builder.input(&[n]);
    let graph = builder.finish([x.sum(n), x.prod(n)]);
    let input = Tensor::from_fn(&[n], |coord| [2.0, 3.0, 5.0][coord[&n]]);

    let outputs = graph.run([input]);
    assert_eq!(outputs.len(), 2);
    assert_eq!(outputs[0].data, vec![10.0]);
    assert_eq!(outputs[1].data, vec![30.0]);
}

#[test]
#[should_panic(expected = "different GraphBuilder")]
fn finish_rejects_a_foreign_input() {
    let n = axis("n", 2);
    let mut left = GraphBuilder::new();
    let x = left.input(&[n]);
    let mut right = GraphBuilder::new();
    let y = right.input(&[n]);

    let _ = left.finish([x + y]);
}
