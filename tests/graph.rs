//! The public stepped-execution API: state declared in the graph, feedback
//! wired by compilation, no capture/replay in sight.

#![cfg(target_os = "macos")]

use sanic::ir::{Dtype, axis};
use sanic::{Graph, MetalDevice, Tensor};

// A state carries across steps: s starts zeroed, each step outputs 2·s and
// then advances s by one. The third step must therefore see s == 2.
#[test]
fn a_stateful_counter_steps_on_metal() {
    let Some(device) = MetalDevice::open() else {
        eprintln!("no Metal device; skipping");
        return;
    };
    let d = axis("d", 4);
    let mut graph = Graph::new();
    let s = graph.state("s", [d], Dtype::F32);
    graph.output("y", &s * 2.0);
    graph.update(&s, &s + 1.0);

    let program = graph.compile_for(&device, Default::default()).unwrap();
    let mut machine = program.instantiate(&device, std::iter::empty()).unwrap();
    for _ in 0..3 {
        machine.step().unwrap();
    }
    let y = device.read_tensor_f32(&machine.output("y"));
    assert_eq!(y, vec![4.0; 4], "the third step must see s == 2");
}

// A successor of the shape `where(c, x, s)` agrees with the state `s`
// everywhere c is zero, so it is written into s's own buffer and the points it
// does not store are simply left alone. What proves the alias sound is that
// earlier steps' rows are still there afterwards: nothing copies them forward
// any more, so if the buffers were not really one, this would read back zeros.
#[test]
fn a_row_write_state_keeps_what_earlier_steps_wrote() {
    let Some(device) = MetalDevice::open() else {
        eprintln!("no Metal device; skipping");
        return;
    };
    let mut graph = Graph::new();
    let state = graph.state("s", [axis("seq", 4)], Dtype::F32);
    let position = Tensor::input("position", []);
    let index = state.coordinate(0usize);
    let here = index.lt(&position + 1.0) * position.lt(&index + 1.0);
    let successor = here.select(&position + 10.0, &state);
    graph.update(&state, successor.clone());
    graph.output("s", successor);

    let program = graph.compile_for(&device, Default::default()).unwrap();
    let position_buffer = device
        .tensor_from_raw(device.alloc_elems(1, Dtype::F32), vec![], Dtype::F32)
        .unwrap();
    let mut machine = program
        .instantiate(&device, std::iter::once(("position", &position_buffer)))
        .unwrap();
    for position in 0..3 {
        device.write_f64(position_buffer.raw(), &[position as f64]);
        machine.step().unwrap();
    }
    let s = device.read_tensor_f32(&machine.output("s"));
    assert_eq!(
        s,
        vec![10.0, 11.0, 12.0, 0.0],
        "each step writes its own row and leaves the others standing"
    );
}

// A bf16 target rejects an f32 state declaration instead of silently
// binding a mis-sized buffer.
#[test]
fn storage_mismatch_is_a_compile_error() {
    let Some(device) = MetalDevice::open() else {
        eprintln!("no Metal device; skipping");
        return;
    };

    let mut graph = Graph::new();
    let s = graph.state("s", [axis("d", 4)], Dtype::F32);
    graph.update(&s, &s + 1.0);
    let err = graph
        .compile_for(&device, sanic::cost::Policy { boundary: Dtype::BF16 })
        .err()
        .expect("must not compile");
    assert!(err.to_string().contains("boundaries"), "{err}");
}
