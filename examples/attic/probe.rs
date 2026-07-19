use sanic::cost::DeviceProfile;
use sanic::ir::*;
use sanic::partition::partition;
use std::collections::HashMap;

fn main() {
    let (ne, fe, gi, ri, c, dm) = (axis("ne"), axis("fe"), axis("gi"), axis("ri"), axis("c"), axis("dm"));
    let ext: HashMap<Axis, f64> = [(ne, 128.0), (fe, 256.0), (gi, 8.0), (ri, 128.0), (c, 1024.0), (dm, 1024.0)]
        .into_iter()
        .collect();
    let xs = split(input("x", &[dm], Dtype::F32), dm, gi, ri, 128);
    let wg = gather(input("W", &[ne, fe, gi, ri], Dtype::I4), input("idx", &[], Dtype::F32), ne);
    let sg = gather(input("S", &[ne, fe, gi], Dtype::F32), input("idx2", &[], Dtype::F32), ne);
    let prod = map(MapOp::Mul, vec![map(MapOp::Mul, vec![wg, xs]), sg]);
    let y = reduce(flatten(prod, &[gi, ri], c), c, BinOp::Monoid(Monoid::Add));
    let sched = partition(&y, &DeviceProfile::toy(), &ext);
    println!("{}", sched.render());
}
