trait Streamable {
    type Item; // one element along the axis
    type Acc; // the CARRIER — the running state (this is the whole game)
    type Out; // the final answer

    fn into_acc(x: Self::Item) -> Self::Acc; // one element -> state
    fn combine(a: Self::Acc, b: Self::Acc) -> Self::Acc; // ⊗  MUST be associative
    fn identity() -> Self::Acc; // e:  combine(e, a) == a
    fn project(a: Self::Acc) -> Self::Out; // state -> answer, at the very end
}

struct Sum;
impl Streamable for Sum {
    type Item = f64;
    type Acc = f64;
    type Out = f64;
    fn into_acc(x: Self::Item) -> Self::Acc {
        x
    }
    fn combine(a: Self::Acc, b: Self::Acc) -> Self::Acc {
        a + b // associative ✓
    }
    fn identity() -> Self::Acc {
        0.0
    }
    fn project(a: Self::Acc) -> Self::Out {
        a // Acc and Out are the same type
    }
}

struct Mean;
impl Streamable for Mean {
    type Item = f64;
    type Acc = (f64, u64); // (sum, count)  <-- Acc ≠ Out: this is the strengthening
    type Out = f64;
    fn into_acc(x: Self::Item) -> Self::Acc {
        (x, 1)
    }
    fn combine(a: Self::Acc, b: Self::Acc) -> Self::Acc {
        (a.0 + b.0, a.1 + b.1)
    }
    fn identity() -> Self::Acc {
        (0.0, 0)
    }
    fn project(a: (f64, u64)) -> f64 {
        a.0 / a.1 as f64
    } // divide only at the end
}

struct Softmax;
impl Streamable for Softmax {
    type Item = f64;
    type Acc = (f64, f64); // (running_max, sum_of_exp_relative_to_max)
    type Out = Vec<f64>; // (in practice you re-walk to emit weights; denom is the Acc)
    fn into_acc(x: Self::Item) -> Self::Acc {
        (x, 1.0)
    }
    fn combine(a: Self::Acc, b: Self::Acc) -> Self::Acc {
        let m = a.0.max(b.0);
        (m, a.1 * (a.0 - m).exp() + b.1 * (b.0 - m).exp()) // rescale both to new max
    }
    fn identity() -> Self::Acc {
        (f64::NEG_INFINITY, 0.0)
    }
    fn project(a: (f64, f64)) -> Vec<f64> {
        /* normalize by a.1 */
        vec![]
    }
}

struct FlashAttention;
impl Streamable for FlashAttention {
    type Item = (f64, f64); // (score, value) for one key
    type Acc = (f64, f64, f64); // (max, denom, weighted_output)  <-- one more field
    type Out = f64;
    fn into_acc((s, v): Self::Item) -> Self::Acc {
        (s, 1.0, v)
    }
    fn combine(a: Self::Acc, b: Self::Acc) -> Self::Acc {
        let m = a.0.max(b.0);
        let (ra, rb) = ((a.0 - m).exp(), (b.0 - m).exp());
        (m, a.1 * ra + b.1 * rb, a.2 * ra + b.2 * rb) // rescale denom AND output together
    }
    fn identity() -> Self::Acc {
        (f64::NEG_INFINITY, 0.0, 0.0)
    }
    fn project(a: (f64, f64, f64)) -> f64 {
        a.2 / a.1 // normalize at the very end
    }
}

fn stream<Op: Streamable>(values: impl Iterator<Item = Op::Item>) -> Op::Out {
    let mut acc = Op::identity();
    for v in values {
        acc = Op::combine(acc, Op::into_acc(v));
    }
    Op::project(acc)
}

#[test]
fn e() {
    let values = vec![1.0, 2.0, 3.0];
    let result = stream::<Softmax>(values.into_iter());
    dbg!(result);
}
