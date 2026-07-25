//! The public stepped-execution API: state declared in the graph, feedback
//! wired by compilation, no capture/replay in sight.

#![cfg(target_os = "macos")]

use sanic::ir::{Dtype, axis};
use sanic::{Graph, MetalDevice};

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

    let program = graph.compile_for(&device).unwrap();
    let mut machine = program.instantiate(&device, std::iter::empty()).unwrap();
    for _ in 0..3 {
        machine.step().unwrap();
    }
    let y = device.read_tensor_f32(&machine.output("y"));
    assert_eq!(y, vec![4.0; 4], "the third step must see s == 2");
}

// A bf16 target rejects an f32 state declaration instead of silently
// binding a mis-sized buffer.
#[test]
fn storage_mismatch_is_a_compile_error() {
    let Some(device) = MetalDevice::open() else {
        eprintln!("no Metal device; skipping");
        return;
    };
    let device = device.with_storage(Dtype::BF16);
    let mut graph = Graph::new();
    let s = graph.state("s", [axis("d", 4)], Dtype::F32);
    graph.update(&s, &s + 1.0);
    let err = graph.compile_for(&device).err().expect("must not compile");
    assert!(err.to_string().contains("boundaries"), "{err}");
}
