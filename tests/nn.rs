use sanic::nn::functional::scaled_dot_product_attention;
use sanic::{Buffer, Compile, CpuDevice, Dtype, axis, input};

fn assert_close(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    assert!(
        actual
            .iter()
            .zip(expected)
            .all(|(actual, expected)| (actual - expected).abs() < 1e-9),
        "actual={actual:?}, expected={expected:?}"
    );
}

#[test]
fn scaled_dot_product_attention_uses_torch_layout_and_default_scale() {
    let sequence = axis("sequence", 2);
    let features = axis("features", 2);
    let output = scaled_dot_product_attention(
        input("q", [sequence, features], Dtype::F32),
        input("k", [sequence, features], Dtype::F32),
        input("v", [sequence, features], Dtype::F32),
        None,
        0.0,
        false,
        None,
        false,
    );

    let cpu = CpuDevice::new();
    let program = output.compile(&cpu).unwrap();
    assert_eq!(program.kernel_count(), 1);
    let q = cpu
        .buffer([2, 2], Dtype::F32, [1.0, 0.0, 0.0, 1.0])
        .unwrap();
    let k = cpu
        .buffer([2, 2], Dtype::F32, [1.0, 0.0, 0.0, 1.0])
        .unwrap();
    let v = cpu
        .buffer([2, 2], Dtype::F32, [2.0, 3.0, 5.0, 7.0])
        .unwrap();
    let outputs = program.run([("q", q), ("k", k), ("v", v)]);

    let high = (1.0 / 2.0_f64.sqrt()).exp();
    let selected = high / (high + 1.0);
    let other = 1.0 - selected;
    assert_close(
        outputs[0].data(),
        &[
            selected * 2.0 + other * 5.0,
            selected * 3.0 + other * 7.0,
            other * 2.0 + selected * 5.0,
            other * 3.0 + selected * 7.0,
        ],
    );
}

#[test]
fn additive_mask_and_explicit_scale_match_torch_semantics() {
    let query_sequence = axis("query_sequence", 2);
    let key_sequence = axis("key_sequence", 3);
    let features = axis("features", 1);
    let output = scaled_dot_product_attention(
        input("q", [query_sequence, features], Dtype::F32),
        input("k", [key_sequence, features], Dtype::F32),
        input("v", [key_sequence, features], Dtype::F32),
        Some(input("mask", [query_sequence, key_sequence], Dtype::F32)),
        0.0,
        false,
        Some(2.0),
        false,
    );

    let cpu = CpuDevice::new();
    let program = output.compile(&cpu).unwrap();
    assert_eq!(program.kernel_count(), 1);
    let q = cpu.buffer([2, 1], Dtype::F32, [1.0, 1.0]).unwrap();
    let k = cpu.buffer([3, 1], Dtype::F32, [1.0, 2.0, 3.0]).unwrap();
    let v = cpu.buffer([3, 1], Dtype::F32, [10.0, 20.0, 30.0]).unwrap();
    let mask = cpu
        .buffer([2, 3], Dtype::F32, [0.0, -1e30, -1e30, 0.0, 0.0, -1e30])
        .unwrap();
    let outputs = program.run([("q", q), ("k", k), ("v", v), ("mask", mask)]);

    let weight = 2.0_f64.exp() / (1.0 + 2.0_f64.exp());
    assert_close(
        outputs[0].data(),
        &[10.0, 10.0 * (1.0 - weight) + 20.0 * weight],
    );
}

#[test]
fn causal_attention_uses_an_upper_left_mask_for_rectangular_inputs() {
    let query_sequence = axis("query_sequence", 2);
    let key_sequence = axis("key_sequence", 3);
    let features = axis("features", 2);
    let value_features = axis("value_features", 1);
    let output = scaled_dot_product_attention(
        input("q", [query_sequence, features], Dtype::F32),
        input("k", [key_sequence, features], Dtype::F32),
        input("v", [key_sequence, value_features], Dtype::F32),
        None,
        0.0,
        true,
        Some(0.5),
        false,
    );

    let cpu = CpuDevice::new();
    let program = output.compile(&cpu).unwrap();
    assert_eq!(program.kernel_count(), 1);
    let q = cpu
        .buffer([2, 2], Dtype::F32, [0.0, 0.0, 0.0, 0.0])
        .unwrap();
    let k = cpu
        .buffer([3, 2], Dtype::F32, [0.0, 0.0, 0.0, 0.0, 0.0, 0.0])
        .unwrap();
    let v = cpu.buffer([3, 1], Dtype::F32, [10.0, 20.0, 90.0]).unwrap();
    let outputs = program.run([("q", q), ("k", k), ("v", v)]);

    assert_close(outputs[0].data(), &[10.0, 15.0]);
}

#[test]
fn enable_gqa_repeats_key_and_value_heads_like_torch() {
    let query_heads = axis("query_heads", 4);
    let kv_heads = axis("kv_heads", 2);
    let sequence = axis("sequence", 1);
    let features = axis("features", 1);
    let output = scaled_dot_product_attention(
        input("q", [query_heads, sequence, features], Dtype::F32),
        input("k", [kv_heads, sequence, features], Dtype::F32),
        input("v", [kv_heads, sequence, features], Dtype::F32),
        None,
        0.0,
        false,
        None,
        true,
    );

    let cpu = CpuDevice::new();
    let program = output.compile(&cpu).unwrap();
    let q = cpu
        .buffer([4, 1, 1], Dtype::F32, [1.0, 1.0, 1.0, 1.0])
        .unwrap();
    let k = cpu.buffer([2, 1, 1], Dtype::F32, [1.0, 2.0]).unwrap();
    let v = cpu.buffer([2, 1, 1], Dtype::F32, [10.0, 20.0]).unwrap();
    let outputs = program.run([("q", q), ("k", k), ("v", v)]);

    assert_eq!(outputs[0].shape(), &[4, 1, 1]);
    assert_close(outputs[0].data(), &[10.0, 10.0, 20.0, 20.0]);
}

#[test]
#[should_panic(expected = "nonzero dropout is not supported yet")]
fn nonzero_dropout_is_rejected_explicitly() {
    let sequence = axis("sequence", 1);
    let features = axis("features", 1);
    let _ = scaled_dot_product_attention(
        input("q", [sequence, features], Dtype::F32),
        input("k", [sequence, features], Dtype::F32),
        input("v", [sequence, features], Dtype::F32),
        None,
        0.1,
        false,
        None,
        false,
    );
}
