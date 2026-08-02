//! Prototype: the general chunk carrier is a state transformation.
//!
//! A one-way machine has state `Q` and one-element transitions `Q -> Q`.
//! Every chunk denotes the composition of its element transitions. Those
//! generated transformations form the transition monoid, which is the
//! canonical merge carrier. This example demonstrates three consequences:
//!
//! 1. right-future (one-way) state need not itself admit concatenation;
//! 2. one scalar of sequential state may require a two-scalar chunk carrier;
//! 3. a one-scalar recurrence can have transformation complexity that grows
//!    without bound in a chosen presentation grammar.
//!
//! Run with:
//!
//! ```text
//! cargo run --example transition_monoid_carrier
//! ```

use std::fmt::Debug;

/// A finite presentation of transformations on a sequential state.
///
/// `compose(left, right)` means “run the left chunk, then the right chunk”.
/// The law that makes this a carrier is:
///
/// ```text
/// apply(compose(a,b), q) = apply(b, apply(a,q)).
/// ```
trait TransitionMonoid {
    type State: Clone + Debug;
    type Input;
    type Transform: Clone + Debug;

    fn identity(&self) -> Self::Transform;
    fn singleton(&self, input: &Self::Input) -> Self::Transform;
    fn compose(&self, left: &Self::Transform, right: &Self::Transform) -> Self::Transform;
    fn apply(&self, transform: &Self::Transform, state: &Self::State) -> Self::State;
}

fn fold_transform<M: TransitionMonoid>(monoid: &M, inputs: &[M::Input]) -> M::Transform {
    inputs.iter().fold(monoid.identity(), |acc, input| {
        monoid.compose(&acc, &monoid.singleton(input))
    })
}

fn tree_transform<M: TransitionMonoid>(monoid: &M, inputs: &[M::Input]) -> M::Transform {
    match inputs {
        [] => monoid.identity(),
        [input] => monoid.singleton(input),
        _ => {
            let middle = inputs.len() / 2;
            let left = tree_transform(monoid, &inputs[..middle]);
            let right = tree_transform(monoid, &inputs[middle..]);
            monoid.compose(&left, &right)
        }
    }
}

// ── right-future state is not automatically a monoid ───────────────────────

/// The observable “second symbol”, with `None` for streams shorter than two.
fn second_symbol(word: &[char]) -> Option<char> {
    word.get(1).copied()
}

fn words_up_to(alphabet: &[char], length: usize) -> Vec<Vec<char>> {
    let mut words = vec![Vec::new()];
    for _ in 0..length {
        let previous = words.clone();
        for prefix in previous {
            for &symbol in alphabet {
                let mut word = prefix.clone();
                word.push(symbol);
                words.push(word);
            }
        }
    }
    words.sort();
    words.dedup();
    words
}

fn append(left: &[char], right: &[char]) -> Vec<char> {
    left.iter().chain(right).copied().collect()
}

/// A concrete merge carrier for `second_symbol`: retain the first two
/// symbols of a chunk. Truncating after every concatenation is associative.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Prefix2(Vec<char>);

fn prefix2(word: &[char]) -> Prefix2 {
    Prefix2(word.iter().take(2).copied().collect())
}

fn merge_prefix2(left: &Prefix2, right: &Prefix2) -> Prefix2 {
    Prefix2(left.0.iter().chain(&right.0).take(2).copied().collect())
}

fn demonstrate_two_sided_contexts() {
    let suffixes = words_up_to(&['a', 'b'], 3);
    let (u, v) = (vec!['a'], vec!['b']);

    // No suffix can distinguish the one-symbol prefixes: whichever suffix
    // symbol arrives first becomes the second symbol in both streams.
    assert!(
        suffixes
            .iter()
            .all(|suffix| second_symbol(&append(&u, suffix)) == second_symbol(&append(&v, suffix)))
    );

    // A left context can distinguish them, so suffix equivalence is not a
    // two-sided congruence and cannot define chunk multiplication.
    let left = vec!['a'];
    assert_ne!(second_symbol(&append(&left, &u)), second_symbol(&append(&left, &v)));

    // The two-sided summary is composable.
    let chunks = [vec!['b'], vec!['a', 'b'], vec!['b', 'b', 'a']];
    let summaries: Vec<_> = chunks.iter().map(|chunk| prefix2(chunk)).collect();
    let left_grouped = merge_prefix2(&merge_prefix2(&summaries[0], &summaries[1]), &summaries[2]);
    let right_grouped = merge_prefix2(&summaries[0], &merge_prefix2(&summaries[1], &summaries[2]));
    assert_eq!(left_grouped, right_grouped);
    let whole: Vec<char> = chunks.iter().flatten().copied().collect();
    assert_eq!(left_grouped, prefix2(&whole));
    assert_eq!(second_symbol(&left_grouped.0), second_symbol(&whole));

    println!("right futures versus two-sided contexts");
    println!("  `a` and `b` have identical suffix futures for second_symbol");
    println!("  prepending `a` separates them: second(aa) != second(ab)");
    println!("  merge carrier: first-two-symbol prefix monoid");
}

// ── affine recurrence: state dimension != carrier dimension ────────────────

#[derive(Clone, Copy, Debug)]
struct Affine {
    scale: f64,
    bias: f64,
}

#[derive(Clone, Copy)]
struct AffineTransitions;

impl TransitionMonoid for AffineTransitions {
    type State = f64;
    type Input = Affine;
    type Transform = Affine;

    fn identity(&self) -> Self::Transform {
        Affine { scale: 1.0, bias: 0.0 }
    }

    fn singleton(&self, input: &Self::Input) -> Self::Transform {
        *input
    }

    fn compose(&self, left: &Self::Transform, right: &Self::Transform) -> Self::Transform {
        // right(left(q))
        Affine {
            scale: right.scale * left.scale,
            bias: right.scale * left.bias + right.bias,
        }
    }

    fn apply(&self, transform: &Self::Transform, state: &Self::State) -> Self::State {
        transform.scale * state + transform.bias
    }
}

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-12 * left.abs().max(right.abs()).max(1.0)
}

fn demonstrate_affine_transition_carrier() {
    let monoid = AffineTransitions;
    let steps = [
        Affine { scale: 2.0, bias: 1.0 },
        Affine { scale: -0.5, bias: 3.0 },
        Affine {
            scale: 1.25,
            bias: -2.0,
        },
        Affine { scale: 0.75, bias: 0.5 },
    ];
    let initial = 4.0;

    let sequential = steps.iter().fold(initial, |state, step| monoid.apply(step, &state));
    let left = fold_transform(&monoid, &steps);
    let tree = tree_transform(&monoid, &steps);
    let left_result = monoid.apply(&left, &initial);
    let tree_result = monoid.apply(&tree, &initial);

    assert!(close(sequential, left_result));
    assert!(close(sequential, tree_result));

    // Composition law on arbitrary incoming state: this is stronger than
    // agreement only at the chosen initial value.
    let a = fold_transform(&monoid, &steps[..2]);
    let b = fold_transform(&monoid, &steps[2..]);
    let composed = monoid.compose(&a, &b);
    for q in [-3.0, 0.0, 2.5, 100.0] {
        assert!(close(
            monoid.apply(&composed, &q),
            monoid.apply(&b, &monoid.apply(&a, &q))
        ));
    }

    println!("affine recurrence");
    println!("  sequential state: one scalar q");
    println!("  chunk carrier: two scalars (A, B) denoting q -> A*q + B");
    println!("  tree and sequential result: {tree_result}");
}

// ── presentation-relative failure witness ──────────────────────────────────

/// Under `q -> q^2 + x`, composing another generic step doubles the degree
/// of the chunk's polynomial transformation.
fn quadratic_chunk_degree(steps: usize) -> usize {
    (0..steps).fold(1usize, |degree, _| degree.checked_mul(2).expect("degree overflow"))
}

fn demonstrate_unbounded_polynomial_closure() {
    let degrees: Vec<_> = (0..=8).map(quadratic_chunk_degree).collect();
    assert_eq!(degrees, [1, 2, 4, 8, 16, 32, 64, 128, 256]);

    println!("nonlinear recurrence q -> q^2 + x");
    println!("  sequential state: one scalar q");
    println!("  chunk-transform polynomial degrees: {degrees:?}");
    println!("  conclusion: no fixed-degree polynomial carrier is closed under composition");
}

fn main() {
    println!("syntactic transition-monoid carrier prototype\n");
    demonstrate_two_sided_contexts();
    println!();
    demonstrate_affine_transition_carrier();
    println!();
    demonstrate_unbounded_polynomial_closure();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_witness_the_three_distinctions() {
        demonstrate_two_sided_contexts();
        demonstrate_affine_transition_carrier();
        demonstrate_unbounded_polynomial_closure();
    }
}
