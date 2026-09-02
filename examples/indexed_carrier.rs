//! Prototype: construct coupled carriers from one indexed-monoid rule.
//!
//! Run with:
//!
//! ```text
//! cargo run --example indexed_carrier
//! ```
//!
//! The state is deliberately opaque to the generic code. Its fields are a
//! representation choice, not independently classified "slots". The reusable
//! construction is:
//!
//!   (key, payload) <> (key', payload')
//!     = (joined,
//!        transport(key -> joined, payload)
//!          + transport(key' -> joined, payload'))
//!
//! `ExpShifted` and extremum-filtered payloads are two transport instances.
//! The fiber operation receives the joined key, so a family may use genuinely
//! different identities, operations, and representations at different keys.

use std::fmt::Debug;

/// An associative operation with an identity.
trait Monoid {
    type Value: Clone + Debug;

    fn identity(&self) -> Self::Value;
    fn combine(&self, left: &Self::Value, right: &Self::Value) -> Self::Value;
}

/// A key-dependent family `F : Key -> Mon`.
///
/// Rust stores the possible fiber representations in one payload envelope,
/// while `identity_at` and `combine_at` select the lawful representation and
/// operation for the requested key.
trait MonoidFamily<Key> {
    type Payload: Clone + Debug;

    fn identity_at(&self, key: &Key) -> Self::Payload;
    fn combine_at(&self, key: &Key, left: &Self::Payload, right: &Self::Payload) -> Self::Payload;
}

/// The key algebra used to choose a common fiber for two payloads.
trait JoinSemilattice {
    type Key: Clone + Debug;

    fn bottom(&self) -> Self::Key;
    fn join(&self, left: &Self::Key, right: &Self::Key) -> Self::Key;
}

/// Move a payload from one key's fiber into another.
///
/// The indexed-monoid construction assumes three laws:
///
/// - transporting to the same key is the identity;
/// - transports compose;
/// - every transport is a payload-monoid homomorphism.
trait Transport<Key, Payload> {
    fn move_to(&self, from: &Key, to: &Key, payload: &Payload) -> Payload;
}

#[derive(Clone, Debug)]
struct IndexedState<Key, Payload> {
    key: Key,
    payload: Payload,
}

/// The total monoid obtained from a semilattice-indexed payload monoid.
struct IndexedMonoid<Keys, Fibers, Movement> {
    keys: Keys,
    fibers: Fibers,
    movement: Movement,
}

impl<Keys, Fibers, Movement> Monoid for IndexedMonoid<Keys, Fibers, Movement>
where
    Keys: JoinSemilattice,
    Fibers: MonoidFamily<Keys::Key>,
    Movement: Transport<Keys::Key, Fibers::Payload>,
{
    type Value = IndexedState<Keys::Key, Fibers::Payload>;

    fn identity(&self) -> Self::Value {
        let key = self.keys.bottom();
        IndexedState {
            payload: self.fibers.identity_at(&key),
            key,
        }
    }

    fn combine(&self, left: &Self::Value, right: &Self::Value) -> Self::Value {
        let joined = self.keys.join(&left.key, &right.key);
        let left_payload = self.movement.move_to(&left.key, &joined, &left.payload);
        let right_payload = self.movement.move_to(&right.key, &joined, &right.payload);
        IndexedState {
            payload: self.fibers.combine_at(&joined, &left_payload, &right_payload),
            key: joined,
        }
    }
}

/// A list homomorphism into one whole algebraic state, followed by a projection.
///
/// `State` has no field or slot semantics here. Only its monoid operation is
/// observable to the generic fold machinery.
struct Carrier<Algebra, Lift, Project> {
    algebra: Algebra,
    lift: Lift,
    project: Project,
}

impl<Algebra, Lift, Project> Carrier<Algebra, Lift, Project>
where
    Algebra: Monoid,
{
    fn fold_state<Input>(&self, inputs: &[Input]) -> Algebra::Value
    where
        Lift: Fn(&Input) -> Algebra::Value,
    {
        let values: Vec<_> = inputs.iter().map(&self.lift).collect();
        fold_left(&self.algebra, &values)
    }

    fn tree_state<Input>(&self, inputs: &[Input]) -> Algebra::Value
    where
        Lift: Fn(&Input) -> Algebra::Value,
    {
        let values: Vec<_> = inputs.iter().map(&self.lift).collect();
        fold_tree(&self.algebra, &values)
    }

    fn evaluate<Input, Output>(&self, inputs: &[Input]) -> Output
    where
        Lift: Fn(&Input) -> Algebra::Value,
        Project: Fn(&Algebra::Value) -> Output,
    {
        (self.project)(&self.fold_state(inputs))
    }
}

#[derive(Clone, Copy)]
struct MaximumScore;

impl JoinSemilattice for MaximumScore {
    type Key = f64;

    fn bottom(&self) -> Self::Key {
        f64::NEG_INFINITY
    }

    fn join(&self, left: &Self::Key, right: &Self::Key) -> Self::Key {
        left.max(*right)
    }
}

#[derive(Clone, Copy)]
struct MaximumKey;

impl JoinSemilattice for MaximumKey {
    type Key = i32;

    fn bottom(&self) -> Self::Key {
        i32::MIN
    }

    fn join(&self, left: &Self::Key, right: &Self::Key) -> Self::Key {
        (*left).max(*right)
    }
}

#[derive(Clone, Copy)]
struct AddVector<const WIDTH: usize>;

impl<const WIDTH: usize> Monoid for AddVector<WIDTH> {
    type Value = [f64; WIDTH];

    fn identity(&self) -> Self::Value {
        [0.0; WIDTH]
    }

    fn combine(&self, left: &Self::Value, right: &Self::Value) -> Self::Value {
        std::array::from_fn(|index| left[index] + right[index])
    }
}

impl<Key, const WIDTH: usize> MonoidFamily<Key> for AddVector<WIDTH> {
    type Payload = [f64; WIDTH];

    fn identity_at(&self, _key: &Key) -> Self::Payload {
        self.identity()
    }

    fn combine_at(&self, _key: &Key, left: &Self::Payload, right: &Self::Payload) -> Self::Payload {
        self.combine(left, right)
    }
}

#[derive(Clone, Copy)]
struct MinimumIndex;

impl Monoid for MinimumIndex {
    type Value = usize;

    fn identity(&self) -> Self::Value {
        usize::MAX
    }

    fn combine(&self, left: &Self::Value, right: &Self::Value) -> Self::Value {
        (*left).min(*right)
    }
}

impl<Key> MonoidFamily<Key> for MinimumIndex {
    type Payload = usize;

    fn identity_at(&self, _key: &Key) -> Self::Payload {
        self.identity()
    }

    fn combine_at(&self, _key: &Key, left: &Self::Payload, right: &Self::Payload) -> Self::Payload {
        self.combine(left, right)
    }
}

/// Rebase additive sufficient statistics when the reference score changes.
struct ExponentialRebase;

impl<const WIDTH: usize> Transport<f64, [f64; WIDTH]> for ExponentialRebase {
    fn move_to(&self, from: &f64, to: &f64, payload: &[f64; WIDTH]) -> [f64; WIDTH] {
        if from == to {
            return *payload;
        }
        let scale = (from - to).exp();
        payload.map(|value| value * scale)
    }
}

/// Keep a payload exactly when its key survives the extremum join.
struct KeepWinner<Payload> {
    discarded: Payload,
}

impl<Key, Payload> Transport<Key, Payload> for KeepWinner<Payload>
where
    Key: PartialEq,
    Payload: Clone,
{
    fn move_to(&self, from: &Key, to: &Key, payload: &Payload) -> Payload {
        if from == to {
            payload.clone()
        } else {
            self.discarded.clone()
        }
    }
}

type StableStatistics<const WIDTH: usize> = IndexedMonoid<MaximumScore, AddVector<WIDTH>, ExponentialRebase>;
type EarliestMaximum = IndexedMonoid<MaximumKey, MinimumIndex, KeepWinner<usize>>;

fn stable_statistics<const WIDTH: usize>() -> StableStatistics<WIDTH> {
    IndexedMonoid {
        keys: MaximumScore,
        fibers: AddVector,
        movement: ExponentialRebase,
    }
}

fn earliest_maximum() -> EarliestMaximum {
    IndexedMonoid {
        keys: MaximumKey,
        fibers: MinimumIndex,
        movement: KeepWinner {
            discarded: MinimumIndex.identity(),
        },
    }
}

fn lift_weighted_value(input: &(f64, [f64; 2])) -> IndexedState<f64, [f64; 3]> {
    let (score, value) = input;
    IndexedState {
        key: *score,
        payload: [1.0, value[0], value[1]],
    }
}

fn stable_projection(state: &IndexedState<f64, [f64; 3]>) -> [f64; 2] {
    [state.payload[1] / state.payload[0], state.payload[2] / state.payload[0]]
}

/// Decode the stable coordinates into the ordinary additive sufficient
/// statistics `(sum(exp(score)), sum(exp(score) * value))`.
fn decode_statistics(state: &IndexedState<f64, [f64; 3]>) -> [f64; 3] {
    if state.key == f64::NEG_INFINITY {
        return [0.0; 3];
    }
    let scale = state.key.exp();
    state.payload.map(|value| scale * value)
}

fn fold_left<M: Monoid>(monoid: &M, values: &[M::Value]) -> M::Value {
    values
        .iter()
        .fold(monoid.identity(), |state, value| monoid.combine(&state, value))
}

fn fold_tree<M: Monoid>(monoid: &M, values: &[M::Value]) -> M::Value {
    match values {
        [] => monoid.identity(),
        [value] => value.clone(),
        _ => {
            let middle = values.len() / 2;
            let left = fold_tree(monoid, &values[..middle]);
            let right = fold_tree(monoid, &values[middle..]);
            monoid.combine(&left, &right)
        }
    }
}

fn assert_commutative_monoid_laws<M, Equal>(name: &str, monoid: &M, samples: &[M::Value], equal: Equal)
where
    M: Monoid,
    Equal: Fn(&M::Value, &M::Value) -> bool,
{
    let identity = monoid.identity();
    for value in samples {
        assert!(equal(&monoid.combine(&identity, value), value), "{name}: left identity");
        assert!(
            equal(&monoid.combine(value, &identity), value),
            "{name}: right identity"
        );
    }

    for left in samples {
        for right in samples {
            assert!(
                equal(&monoid.combine(left, right), &monoid.combine(right, left)),
                "{name}: commutativity"
            );
            for third in samples {
                let left_grouped = monoid.combine(&monoid.combine(left, right), third);
                let right_grouped = monoid.combine(left, &monoid.combine(right, third));
                assert!(equal(&left_grouped, &right_grouped), "{name}: associativity");
            }
        }
    }
}

/// Check that decoding is a monoid homomorphism from a representation into
/// its semantic algebra. This is the contract for numerical refinements such
/// as online max rebasing.
fn assert_representation_refinement<Representation, Semantic, Decode, Equal>(
    representation: &Representation,
    semantic: &Semantic,
    samples: &[Representation::Value],
    decode: Decode,
    equal: Equal,
) where
    Representation: Monoid,
    Semantic: Monoid,
    Decode: Fn(&Representation::Value) -> Semantic::Value,
    Equal: Fn(&Semantic::Value, &Semantic::Value) -> bool,
{
    assert!(
        equal(&decode(&representation.identity()), &semantic.identity()),
        "representation identity must decode to semantic identity"
    );
    for left in samples {
        for right in samples {
            let represented_merge = decode(&representation.combine(left, right));
            let semantic_merge = semantic.combine(&decode(left), &decode(right));
            assert!(
                equal(&represented_merge, &semantic_merge),
                "decoding must commute with merge"
            );
        }
    }
}

fn close(left: f64, right: f64) -> bool {
    let scale = left.abs().max(right.abs()).max(1.0);
    (left - right).abs() <= 1e-12 * scale
}

fn close_vector<const WIDTH: usize>(left: &[f64; WIDTH], right: &[f64; WIDTH]) -> bool {
    left.iter().zip(right).all(|(left, right)| close(*left, *right))
}

fn close_stable_state<const WIDTH: usize>(
    left: &IndexedState<f64, [f64; WIDTH]>,
    right: &IndexedState<f64, [f64; WIDTH]>,
) -> bool {
    left.key == right.key && close_vector(&left.payload, &right.payload)
}

fn demonstrate_stable_statistics() {
    let carrier = Carrier {
        algebra: stable_statistics::<3>(),
        lift: lift_weighted_value,
        project: stable_projection,
    };
    let ordinary_addition = AddVector::<3>;
    let inputs = [
        (1.0, [2.0, -1.0]),
        (-2.0, [4.0, 3.0]),
        (3.0, [-1.0, 5.0]),
        (0.5, [7.0, 2.0]),
    ];
    let elements: Vec<_> = inputs.iter().map(lift_weighted_value).collect();

    assert_commutative_monoid_laws("exponential rebase", &carrier.algebra, &elements, close_stable_state);
    assert_representation_refinement(
        &carrier.algebra,
        &ordinary_addition,
        &elements,
        decode_statistics,
        close_vector,
    );

    let sequential = carrier.fold_state(&inputs);
    let parallel = carrier.tree_state(&inputs);
    assert!(close_stable_state(&sequential, &parallel));

    // Scores this large make a direct exp/sum overflow, but rebasing keeps the
    // projected weighted average finite.
    let large_scores = [(10_000.0, [1.0, 2.0]), (9_999.0, [5.0, -2.0]), (9_998.0, [-1.0, 4.0])];
    let stable = carrier.tree_state(&large_scores);
    let projected = carrier.evaluate(&large_scores);
    assert!(projected.iter().all(|value| value.is_finite()));

    println!("stable statistics");
    println!("  state: {stable:?}");
    println!("  weighted average: {projected:?}");
    println!("  laws: identity, commutativity, associativity, refinement");
}

fn demonstrate_extremal_filter() {
    let carrier = Carrier {
        algebra: earliest_maximum(),
        lift: |&(key, index): &(i32, usize)| IndexedState { key, payload: index },
        project: |state: &IndexedState<i32, usize>| (state.key, state.payload),
    };
    let inputs = [(7, 0), (11, 1), (11, 2), (4, 3)];
    let elements: Vec<_> = inputs.iter().map(&carrier.lift).collect();

    assert_commutative_monoid_laws("winner transport", &carrier.algebra, &elements, |left, right| {
        left.key == right.key && left.payload == right.payload
    });

    let sequential = carrier.fold_state(&inputs);
    let parallel = carrier.tree_state(&inputs);
    let projected = carrier.evaluate(&inputs);
    assert_eq!(sequential.key, parallel.key);
    assert_eq!(sequential.payload, parallel.payload);
    assert_eq!(projected, (11, 1));

    println!("extremal filtering");
    println!("  state: {parallel:?}");
    println!(
        "  interpretation: maximum key {}, earliest tied index {}",
        projected.0, projected.1
    );
    println!("  laws: identity, commutativity, associativity");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Activation {
    Bottom,
    Live,
}

#[derive(Clone, Copy)]
struct ActivationKeys;

impl JoinSemilattice for ActivationKeys {
    type Key = Activation;

    fn bottom(&self) -> Self::Key {
        Activation::Bottom
    }

    fn join(&self, left: &Self::Key, right: &Self::Key) -> Self::Key {
        (*left).max(*right)
    }
}

/// One payload envelope holding two genuinely different fiber
/// representations: the bottom fiber is a unit value, while the live fiber
/// contains an additive counter.
#[derive(Clone, Debug, PartialEq, Eq)]
enum ActivationPayload {
    Bottom,
    Live(u64),
}

#[derive(Clone, Copy)]
struct ActivationFibers;

impl MonoidFamily<Activation> for ActivationFibers {
    type Payload = ActivationPayload;

    fn identity_at(&self, key: &Activation) -> Self::Payload {
        match key {
            Activation::Bottom => ActivationPayload::Bottom,
            Activation::Live => ActivationPayload::Live(0),
        }
    }

    fn combine_at(&self, key: &Activation, left: &Self::Payload, right: &Self::Payload) -> Self::Payload {
        match (key, left, right) {
            (Activation::Bottom, ActivationPayload::Bottom, ActivationPayload::Bottom) => ActivationPayload::Bottom,
            (Activation::Live, ActivationPayload::Live(left), ActivationPayload::Live(right)) => {
                ActivationPayload::Live(left + right)
            }
            _ => panic!("payload does not belong to the selected fiber"),
        }
    }
}

#[derive(Clone, Copy)]
struct Activate;

impl Transport<Activation, ActivationPayload> for Activate {
    fn move_to(&self, from: &Activation, to: &Activation, payload: &ActivationPayload) -> ActivationPayload {
        match (from, to, payload) {
            (Activation::Bottom, Activation::Bottom, ActivationPayload::Bottom) => ActivationPayload::Bottom,
            (Activation::Bottom, Activation::Live, ActivationPayload::Bottom) => ActivationPayload::Live(0),
            (Activation::Live, Activation::Live, ActivationPayload::Live(value)) => ActivationPayload::Live(*value),
            _ => panic!("transport must follow the semilattice order"),
        }
    }
}

fn demonstrate_key_dependent_fibers() {
    let family = IndexedMonoid {
        keys: ActivationKeys,
        fibers: ActivationFibers,
        movement: Activate,
    };
    assert_eq!(family.identity().payload, ActivationPayload::Bottom);

    let values = [
        IndexedState {
            key: Activation::Live,
            payload: ActivationPayload::Live(2),
        },
        IndexedState {
            key: Activation::Live,
            payload: ActivationPayload::Live(5),
        },
    ];
    let result = fold_tree(&family, &values);
    assert_eq!(result.key, Activation::Live);
    assert_eq!(result.payload, ActivationPayload::Live(7));

    println!("key-dependent fibers");
    println!("  bottom representation: unit payload");
    println!("  live representation: additive counter {result:?}");
}

fn main() {
    println!("indexed-monoid carrier prototype\n");
    demonstrate_stable_statistics();
    println!();
    demonstrate_extremal_filter();
    println!();
    demonstrate_key_dependent_fibers();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_construction_covers_uniform_and_key_dependent_fibers() {
        demonstrate_stable_statistics();
        demonstrate_extremal_filter();
        demonstrate_key_dependent_fibers();
    }
}
