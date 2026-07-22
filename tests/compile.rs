use sanic::cost::DeviceProfile;
use sanic::derive::{SlotKind, derive};
use sanic::partition::{Stage, partition, partition_many};
use sanic::{
    Buffer, Compile, CompileError, CpuDevice, Dtype, Extent, MapOp, Monoid, ViewDim, argmax, axis,
    axis_refs, input, map, matmul, positional_view, reduce, topk,
};

fn add() -> Monoid {
    Monoid::Add
}

#[test]
fn positional_dimensions_do_not_alias_equal_metadata() {
    let d = axis("d", 2);
    let x = input("x", [d, d], Dtype::F32);
    let rows = reduce(x, 1usize, add());

    let cpu = CpuDevice::new();
    let program = rows.compile(&cpu).unwrap();
    let x = cpu
        .buffer([2, 2], Dtype::F32, vec![1.0, 2.0, 3.0, 4.0])
        .unwrap();
    let outputs = program.run([("x", x)]);

    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].shape(), &[2]);
    assert_eq!(outputs[0].data(), &[3.0, 7.0]);
}

#[test]
fn tuple_roots_share_named_inputs_and_return_a_vec_in_order() {
    let row = axis("row", 2);
    let col = axis("col", 2);
    let x = input("x", [row, col], Dtype::F32);
    let by_row = reduce(x.clone(), 1usize, add());
    let by_col = reduce(x, 0usize, add());

    let cpu = CpuDevice::new();
    let program = (&by_row, &by_col).compile(&cpu).unwrap();
    let x = cpu
        .buffer([2, 2], Dtype::F32, vec![1.0, 2.0, 3.0, 4.0])
        .unwrap();
    let outputs = program.run([("x", &x)]);

    assert_eq!(outputs.len(), 2);
    assert_eq!(outputs[0].data(), &[3.0, 7.0]);
    assert_eq!(outputs[1].data(), &[4.0, 6.0]);
}

#[test]
fn map_broadcasts_positionally() {
    let row = axis("row", 2);
    let col = axis("col", 3);
    let x = input("x", [row, col], Dtype::F32);
    let b = input("b", [col], Dtype::F32);
    let y = map(MapOp::Add, vec![x, b]);

    let cpu = CpuDevice::new();
    let program = y.compile(&cpu).unwrap();
    let x = cpu
        .buffer([2, 3], Dtype::F32, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
        .unwrap();
    let b = cpu.buffer([3], Dtype::F32, vec![10.0, 20.0, 30.0]).unwrap();
    let outputs = program.run([("x", &x), ("b", &b)]);

    assert_eq!(outputs[0].data(), &[11.0, 22.0, 33.0, 14.0, 25.0, 36.0]);
}

#[test]
fn broadcasting_preserves_output_order_when_the_short_operand_is_first() {
    let row = axis("row", 2);
    let col = axis("col", 3);
    let b = input("b", [col], Dtype::F32);
    let x = input("x", [row, col], Dtype::F32);
    let y = map(MapOp::Sub, vec![b, x]);

    let cpu = CpuDevice::new();
    let program = y.compile(&cpu).unwrap();
    let b = cpu.buffer([3], Dtype::F32, [10.0, 20.0, 30.0]).unwrap();
    let x = cpu
        .buffer([2, 3], Dtype::F32, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
        .unwrap();
    let outputs = program.run([("b", &b), ("x", &x)]);

    assert_eq!(outputs[0].shape(), &[2, 3]);
    assert_eq!(outputs[0].data(), &[9.0, 18.0, 27.0, 6.0, 15.0, 24.0]);
}

#[test]
fn positional_view_can_permute_storage_order() {
    let row = axis("row", 2);
    let col = axis("col", 3);
    let x = input("x", [row, col], Dtype::F32);
    let transposed = positional_view(
        x,
        vec![
            ViewDim {
                sources: vec![1],
                axis: col,
            },
            ViewDim {
                sources: vec![0],
                axis: row,
            },
        ],
    );

    let cpu = CpuDevice::new();
    let program = transposed.compile(&cpu).unwrap();
    let x = cpu
        .buffer([2, 3], Dtype::F32, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
        .unwrap();
    let outputs = program.run([("x", x)]);

    assert_eq!(outputs[0].shape(), &[3, 2]);
    assert_eq!(outputs[0].data(), &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
}

#[test]
fn compilation_rejects_dynamic_shapes_at_its_boundary() {
    let x = input("x", [axis("dynamic", Extent::Dynamic)], Dtype::F32);
    assert!(matches!(
        x.compile(&CpuDevice::new()),
        Err(CompileError::DynamicShapesNotYetSupported)
    ));
}

#[test]
fn matmul_contracts_by_position_even_when_axis_metadata_repeats() {
    let d = axis("d", 2);
    let left = input("left", [d, d], Dtype::F32);
    let right = input("right", [d, d], Dtype::F32);
    let output = matmul(left, right);

    let cpu = CpuDevice::new();
    let program = output.compile(&cpu).unwrap();
    let left = cpu
        .buffer([2, 2], Dtype::F32, vec![1.0, 2.0, 3.0, 4.0])
        .unwrap();
    let right = cpu
        .buffer([2, 2], Dtype::F32, vec![5.0, 6.0, 7.0, 8.0])
        .unwrap();
    let outputs = program.run([("left", &left), ("right", &right)]);

    assert_eq!(outputs[0].shape(), &[2, 2]);
    assert_eq!(outputs[0].data(), &[19.0, 22.0, 43.0, 50.0]);
}

#[test]
fn positional_argmax_is_one_generic_key_payload_fold() {
    let item = axis("item", 8);
    let x = input("x", [item], Dtype::F32);
    let stream = axis_refs(&x)[0];
    let index = argmax(x, -1isize);

    let carrier = derive(&index, stream).expect("argmax composition should derive");
    assert!(matches!(
        carrier.kinds.as_slice(),
        [
            SlotKind::Plain(Monoid::Max),
            SlotKind::AtExtremum {
                key_slot: 0,
                key: Monoid::Max,
                ties: Monoid::Min,
            }
        ]
    ));
    let schedule = partition(&index, &DeviceProfile::toy());
    assert!(matches!(
        schedule.stages.as_slice(),
        [Stage::Fused { spec, .. }] if spec.carrier.slots == 2
    ));

    let cpu = CpuDevice::new();
    let program = index.compile(&cpu).unwrap();
    let x = cpu
        .buffer([8], Dtype::F32, [2.0, 5.0, 5.0, 1.0, 5.0, 2.0, 0.0, 5.0])
        .unwrap();
    let outputs = program.run([("x", x)]);
    assert_eq!(outputs[0].shape(), &[]);
    assert_eq!(outputs[0].data(), &[1.0]);
}

#[test]
fn positional_topk_composition_returns_descending_values_and_indices() {
    let item = axis("item", 8);
    let pairs = topk(input("x", [item], Dtype::F32), 0usize, 3);
    let roots = pairs
        .iter()
        .flat_map(|(value, index)| [value.clone(), index.clone()])
        .collect::<Vec<_>>();

    let schedule = partition_many(
        &[
            (pairs[0].0.clone(), "v0"),
            (pairs[0].1.clone(), "i0"),
            (pairs[1].0.clone(), "v1"),
            (pairs[1].1.clone(), "i1"),
            (pairs[2].0.clone(), "v2"),
            (pairs[2].1.clone(), "i2"),
        ],
        &DeviceProfile::toy(),
    );
    assert!(
        schedule
            .stages
            .iter()
            .all(|stage| !matches!(stage, Stage::Infeasible { .. })),
        "the generic repeated-selection graph must remain schedulable"
    );
    let cpu = CpuDevice::new();
    let program = roots.compile(&cpu).unwrap();
    let x = cpu
        .buffer([8], Dtype::F32, [2.0, 9.0, 5.0, 7.0, 3.0, 6.0, 1.0, 4.0])
        .unwrap();
    let outputs = program.run([("x", x)]);
    let got = outputs
        .iter()
        .map(|output| output.data()[0])
        .collect::<Vec<_>>();
    assert_eq!(got, [9.0, 1.0, 7.0, 3.0, 6.0, 5.0]);
}

#[cfg(target_os = "macos")]
#[test]
fn the_same_program_api_executes_on_metal() {
    let Some(metal) = sanic::MetalDevice::open() else {
        return;
    };
    let d = axis("d", 2);
    let output = reduce(input("x", [d, d], Dtype::F32), 1usize, add());
    let program = output.compile(&metal).unwrap();
    let x = metal
        .tensor_from_f64([2, 2], Dtype::F32, &[1.0, 2.0, 3.0, 4.0])
        .unwrap();

    let outputs = program.run([("x", &x)]);
    assert_eq!(outputs[0].shape(), &[2]);
    assert_eq!(metal.read_tensor_f32(&outputs[0]), [3.0, 7.0]);
}

#[cfg(target_os = "macos")]
#[test]
fn direct_attention_is_one_metal_kernel() {
    use sanic::nn::functional::scaled_dot_product_attention;

    let Some(metal) = sanic::MetalDevice::open() else {
        return;
    };
    let sequence = axis("sequence", 2);
    let features = axis("features", 2);
    let q = input("q", [sequence, features], Dtype::F32);
    let k = input("k", [sequence, features], Dtype::F32);
    let v = input("v", [sequence, features], Dtype::F32);
    let output = scaled_dot_product_attention(q, k, v, None, 0.0, false, None, false);
    let program = output.compile(&metal).unwrap();
    assert_eq!(program.kernel_count(), 1);

    let q = metal
        .tensor_from_f64([2, 2], Dtype::F32, &[1.0, 0.0, 0.0, 1.0])
        .unwrap();
    let k = metal
        .tensor_from_f64([2, 2], Dtype::F32, &[1.0, 0.0, 0.0, 1.0])
        .unwrap();
    let v = metal
        .tensor_from_f64([2, 2], Dtype::F32, &[2.0, 3.0, 5.0, 7.0])
        .unwrap();
    let outputs = program.run([("q", q), ("k", k), ("v", v)]);
    let actual = metal.read_tensor_f32(&outputs[0]);
    let expected = [2.990_715, 4.320_953_4, 4.009_285, 5.679_046_6];
    // the f32-Metal-vs-reference comparison reads the one policy in F32
    assert!(
        actual
            .iter()
            .zip(expected)
            .all(|(actual, expected): (&f32, f32)| {
                let tol =
                    sanic::verify::rel_tolerance(Dtype::F32, 2) * (1.0 + expected.abs() as f64);
                ((actual - expected).abs() as f64) < tol
            })
    );
}

#[cfg(target_os = "macos")]
#[test]
fn captured_replay_feeds_outputs_back_as_inputs() {
    let Some(metal) = sanic::MetalDevice::open() else {
        return;
    };
    let d = axis("d", 4);
    let state = input("state", [d], Dtype::F32);
    let delta = input("delta", [d], Dtype::F32);
    let program = map(MapOp::Add, vec![state, delta]).compile(&metal).unwrap();

    let state = metal
        .tensor_from_f64([4], Dtype::F32, &[1.0, 2.0, 3.0, 4.0])
        .unwrap();
    let delta = metal
        .tensor_from_f64([4], Dtype::F32, &[10.0, 10.0, 10.0, 10.0])
        .unwrap();
    let mut replay = program
        .capture([("state", &state), ("delta", &delta)], &[(0, "state")])
        .unwrap();

    // the fed-back state accumulates delta on every replayed step
    assert_eq!(
        metal.read_tensor_f32(&replay.step().unwrap()[0]),
        [11.0, 12.0, 13.0, 14.0]
    );
    assert_eq!(
        metal.read_tensor_f32(&replay.step().unwrap()[0]),
        [21.0, 22.0, 23.0, 24.0]
    );
    // bindings are frozen but their contents are live: a CPU write into
    // `delta` is visible to the next replay (the token-id pattern)
    metal.write_f64(delta.raw(), &[100.0, 100.0, 100.0, 100.0]);
    assert_eq!(
        metal.read_tensor_f32(&replay.step().unwrap()[0]),
        [121.0, 122.0, 123.0, 124.0]
    );
}

#[cfg(target_os = "macos")]
#[test]
fn captured_replay_without_feedback_reruns_one_graph_over_live_bindings() {
    let Some(metal) = sanic::MetalDevice::open() else {
        return;
    };
    let d = axis("d", 2);
    let output = reduce(input("x", [d, d], Dtype::F32), 1usize, add());
    let program = output.compile(&metal).unwrap();
    let x = metal
        .tensor_from_f64([2, 2], Dtype::F32, &[1.0, 2.0, 3.0, 4.0])
        .unwrap();
    let mut replay = program.capture([("x", &x)], &[]).unwrap();

    assert_eq!(
        metal.read_tensor_f32(&replay.step().unwrap()[0]),
        [3.0, 7.0]
    );
    assert_eq!(
        metal.read_tensor_f32(&replay.step().unwrap()[0]),
        [3.0, 7.0]
    );
    metal.write_f64(x.raw(), &[10.0, 20.0, 30.0, 40.0]);
    assert_eq!(
        metal.read_tensor_f32(&replay.step().unwrap()[0]),
        [30.0, 70.0]
    );
}

#[cfg(target_os = "macos")]
#[test]
fn capture_rejects_bad_feedback_wiring() {
    use sanic::RunError;

    let Some(metal) = sanic::MetalDevice::open() else {
        return;
    };
    let d = axis("d", 4);
    let state = input("state", [d], Dtype::F32);
    let delta = input("delta", [d], Dtype::F32);
    let program = map(MapOp::Add, vec![state, delta]).compile(&metal).unwrap();
    let state = metal.tensor_from_f64([4], Dtype::F32, &[0.0; 4]).unwrap();
    let delta = metal.tensor_from_f64([4], Dtype::F32, &[0.0; 4]).unwrap();

    let out_of_range = program.capture([("state", &state), ("delta", &delta)], &[(1, "state")]);
    assert!(matches!(out_of_range, Err(RunError::Feedback(_))));
    let unknown_input = program.capture([("state", &state), ("delta", &delta)], &[(0, "missing")]);
    assert!(matches!(unknown_input, Err(RunError::Feedback(_))));
}
