use sanic::nn::functional::scaled_dot_product_attention;
use sanic::{Buffer, Compile, CpuDevice, Dtype, axis, input};

fn main() {
    let sequence = axis("sequence", 2);
    let features = axis("features", 2);

    let q = input("q", [sequence, features], Dtype::F32);
    let k = input("k", [sequence, features], Dtype::F32);
    let v = input("v", [sequence, features], Dtype::F32);
    let output = scaled_dot_product_attention(q, k, v, None, 0.0, false, None, false);

    let cpu = CpuDevice::new();
    let program = output.compile(&cpu).expect("attention should compile");
    println!("compiled to {} kernel", program.kernel_count());
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
    println!("{:?}: {:?}", outputs[0].shape(), outputs[0].data());
}
