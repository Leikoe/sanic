use sanic::{
    Compile, CompileError, CpuDevice, Dtype, MapOp, Monoid, RunError, axis, input, map, reduce,
};

fn add() -> Monoid {
    Monoid::Add
}

#[test]
fn an_input_name_is_its_identity_within_the_compiled_roots() {
    let first = input(
        String::from("x"),
        [axis("first row", 2), axis("first col", 2)],
        Dtype::F32,
    );
    let second = input(
        String::from("x"),
        [axis("second row", 2), axis("second col", 2)],
        Dtype::F32,
    );
    let output = reduce(map(MapOp::Add, vec![first, second]), 1usize, add());

    let cpu = CpuDevice::new();
    let program = output.compile(&cpu).unwrap();
    assert_eq!(program.input_names().collect::<Vec<_>>(), ["x"]);

    let x = cpu
        .buffer([2, 2], Dtype::F32, vec![1.0, 2.0, 3.0, 4.0])
        .unwrap();
    let output = program.run([("x", &x)]);
    assert_eq!(output[0].data(), &[6.0, 14.0]);
}

#[test]
fn the_same_name_must_have_one_shape_and_dtype_per_program() {
    let left = input("x", [axis("d", 2)], Dtype::F32);
    let right = input("x", [axis("d", 3)], Dtype::F32);
    assert!(matches!(
        (left, right).compile(&CpuDevice::new()),
        Err(CompileError::InvalidInput(_))
    ));

    let left = input("x", [axis("d", 2)], Dtype::F32);
    let right = input("x", [axis("d", 2)], Dtype::F64);
    assert!(matches!(
        (left, right).compile(&CpuDevice::new()),
        Err(CompileError::InvalidInput(_))
    ));
}

#[test]
fn try_run_reports_named_binding_errors() {
    let output = input("x", [axis("d", 2)], Dtype::F32);
    let cpu = CpuDevice::new();
    let program = output.compile(&cpu).unwrap();
    let good = cpu.buffer([2], Dtype::F32, vec![1.0, 2.0]).unwrap();
    let wrong_shape = cpu.buffer([1], Dtype::F32, vec![1.0]).unwrap();
    let wrong_dtype = cpu.buffer([2], Dtype::F64, vec![1.0, 2.0]).unwrap();

    assert!(matches!(
        program.try_run([]),
        Err(RunError::MissingInput(name)) if name == "x"
    ));
    assert!(matches!(
        program.try_run([("y", &good)]),
        Err(RunError::UnknownInput(name)) if name == "y"
    ));
    assert!(matches!(
        program.try_run([("x", &good), ("x", &good)]),
        Err(RunError::DuplicateInput(name)) if name == "x"
    ));
    assert!(matches!(
        program.try_run([("x", &wrong_shape)]),
        Err(RunError::Shape { name, .. }) if name == "x"
    ));
    assert!(matches!(
        program.try_run([("x", &wrong_dtype)]),
        Err(RunError::Dtype { name, .. }) if name == "x"
    ));
}

#[test]
fn only_inputs_reachable_from_the_selected_roots_are_compiled() {
    let x = input("x", [axis("d", 2)], Dtype::F32);
    let _unrelated = input("y", [axis("d", 2)], Dtype::F32);

    let program = x.compile(&CpuDevice::new()).unwrap();
    assert_eq!(program.input_names().collect::<Vec<_>>(), ["x"]);
}

/// Two separately built, structurally identical public subtrees become ONE
/// node before compiler analysis. This lets a RoPE frequency table built once
/// per layer compile once for the whole model.
#[test]
fn structural_duplicates_canonicalize_to_one_node() {
    let d = axis("d", 8);
    let energy = || {
        let x = input("x", [d], Dtype::F32);
        reduce(map(MapOp::Mul, vec![x.clone(), x]), 0usize, add())
    };
    let (first, second) = (energy(), energy());
    assert!(!std::rc::Rc::ptr_eq(&first, &second));

    let canonical = sanic::canonicalize_many(&[first, second]);
    assert!(
        std::rc::Rc::ptr_eq(&canonical[0], &canonical[1]),
        "identical subtrees must share one node after canonicalization"
    );
}
