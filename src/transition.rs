//! Finite-dimensional transition-family closure.
//!
//! A chunk carrier for a sequential state machine denotes the transformation
//! performed by the whole chunk. For a homogeneous linear state of dimension
//! `n`, this module starts from singleton transformations
//!
//! ```text
//! T(x) = I + sum_i coordinate_i(x) D_i
//! ```
//!
//! closes the direction span under matrix multiplication, and emits scalar
//! `into`, `combine`, `identity`, and `project` programs. If
//! `D_j D_i = sum_k mu[k,j,i] D_k`, program-order composition is
//!
//! ```text
//! c_k(left <> right)
//!   = c_k(left) + c_k(right)
//!     + sum_i,j mu[k,j,i] c_i(left) c_j(right).
//! ```
//!
//! Associativity is therefore inherited from matrix composition. Unlike the
//! primitive and indexed carriers derived elsewhere, these carriers are not
//! generally commutative: their certificate licenses only order-preserving
//! reassociation.

use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::derive::{ComponentConstruction, Expr, LawCertificate, LogicalStateSchema, StateComponent};

type DenseVectors = Vec<Vec<f64>>;

/// A square, row-major matrix used to present a state transformation.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    dimension: usize,
    entries: Vec<f64>,
}

impl Matrix {
    /// Construct a square matrix after validating its shape and coefficients.
    pub fn new(dimension: usize, entries: Vec<f64>) -> Result<Self, TransitionClosureError> {
        if dimension == 0 {
            return Err(TransitionClosureError::ZeroStateDimension);
        }
        if entries.len() != dimension * dimension {
            return Err(TransitionClosureError::MatrixShape {
                dimension,
                entries: entries.len(),
            });
        }
        if entries.iter().any(|value| !value.is_finite()) {
            return Err(TransitionClosureError::NonFiniteCoefficient);
        }
        Ok(Self { dimension, entries })
    }

    /// Construct a matrix from a fixed-size row array.
    pub fn from_rows<const N: usize>(rows: [[f64; N]; N]) -> Self {
        assert!(N > 0, "a transition matrix must be nonempty");
        let entries = rows.into_iter().flatten().collect();
        Self::new(N, entries).expect("fixed-size matrix has a valid shape")
    }

    pub fn identity(dimension: usize) -> Self {
        assert!(dimension > 0, "a transition matrix must be nonempty");
        let mut entries = vec![0.0; dimension * dimension];
        for index in 0..dimension {
            entries[index * dimension + index] = 1.0;
        }
        Self { dimension, entries }
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn entries(&self) -> &[f64] {
        &self.entries
    }

    pub fn apply(&self, state: &[f64]) -> Vec<f64> {
        assert_eq!(state.len(), self.dimension, "state dimension must match matrix");
        (0..self.dimension)
            .map(|row| {
                (0..self.dimension)
                    .map(|column| self.entries[row * self.dimension + column] * state[column])
                    .sum()
            })
            .collect()
    }

    fn multiply(&self, right: &Self) -> Self {
        assert_eq!(self.dimension, right.dimension);
        let n = self.dimension;
        let mut entries = vec![0.0; n * n];
        for row in 0..n {
            for inner in 0..n {
                let left = self.entries[row * n + inner];
                for column in 0..n {
                    entries[row * n + column] += left * right.entries[inner * n + column];
                }
            }
        }
        Self { dimension: n, entries }
    }

    fn add_scaled(&mut self, scale: f64, direction: &Self) {
        assert_eq!(self.dimension, direction.dimension);
        for (value, direction) in self.entries.iter_mut().zip(&direction.entries) {
            *value += scale * direction;
        }
    }
}

/// A singleton transition family affine in symbolic item coordinates.
///
/// `directions[i]` is paired with `singleton_coordinates[i]`. Directions are
/// measured from the identity transformation, so a singleton denotes
/// `I + sum_i coordinate_i(item) * directions[i]`.
#[derive(Debug, Clone)]
pub struct TransitionFamily {
    pub state_dimension: usize,
    pub directions: Vec<Matrix>,
    pub singleton_coordinates: Vec<Expr>,
    pub initial_state: Vec<f64>,
    pub observation: Vec<f64>,
}

/// Failure to construct a finite-dimensional executable transition carrier.
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionClosureError {
    ZeroStateDimension,
    MatrixShape { dimension: usize, entries: usize },
    MatrixDimension { expected: usize, actual: usize },
    CoordinateCount { directions: usize, coordinates: usize },
    StateVectorDimension { expected: usize, actual: usize },
    ObservationDimension { expected: usize, actual: usize },
    InvalidTolerance(f64),
    NonFiniteCoefficient,
    DependentDirection(usize),
    ClosureDimensionExceeded { maximum: usize },
    NumericalClosureFailure,
}

impl Display for TransitionClosureError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroStateDimension => write!(formatter, "transition state dimension must be positive"),
            Self::MatrixShape { dimension, entries } => write!(
                formatter,
                "a {dimension}x{dimension} matrix needs {} entries, got {entries}",
                dimension * dimension
            ),
            Self::MatrixDimension { expected, actual } => {
                write!(
                    formatter,
                    "matrix dimension {actual} does not match state dimension {expected}"
                )
            }
            Self::CoordinateCount {
                directions,
                coordinates,
            } => write!(
                formatter,
                "singleton coordinate count {coordinates} does not match direction count {directions}"
            ),
            Self::StateVectorDimension { expected, actual } => {
                write!(formatter, "initial-state dimension {actual} does not match {expected}")
            }
            Self::ObservationDimension { expected, actual } => {
                write!(formatter, "observation dimension {actual} does not match {expected}")
            }
            Self::InvalidTolerance(tolerance) => {
                write!(
                    formatter,
                    "closure tolerance must be finite and positive, got {tolerance}"
                )
            }
            Self::NonFiniteCoefficient => {
                write!(formatter, "transition presentation contains a non-finite coefficient")
            }
            Self::DependentDirection(index) => {
                write!(formatter, "input transition direction {index} is linearly dependent")
            }
            Self::ClosureDimensionExceeded { maximum } => write!(
                formatter,
                "transition closure exceeded the ambient matrix dimension {maximum}"
            ),
            Self::NumericalClosureFailure => write!(formatter, "could not resolve transition structure constants"),
        }
    }
}

impl Error for TransitionClosureError {}

/// An executable scalar presentation of a closed transition family.
#[derive(Debug, Clone)]
pub struct GeneratedCarrier {
    pub into: Vec<Expr>,
    pub combine: Vec<Expr>,
    pub identity: Vec<f64>,
    pub project: Vec<Expr>,
    pub schema: LogicalStateSchema,
    pub laws: LawCertificate,
    state_dimension: usize,
    directions: Vec<Matrix>,
}

impl GeneratedCarrier {
    pub fn slot_count(&self) -> usize {
        self.schema.len()
    }

    pub fn state_dimension(&self) -> usize {
        self.state_dimension
    }

    pub fn directions(&self) -> &[Matrix] {
        &self.directions
    }

    /// These carriers may be reassociated, but arbitrary chunk reordering is
    /// not licensed unless a stronger construction proves commutativity.
    pub fn mergeable_out_of_order(&self) -> bool {
        self.laws.is_associative() && self.laws.is_commutative()
    }

    /// Decode carrier coordinates back to the represented state transform.
    pub fn decoded_transform(&self, coordinates: &[f64]) -> Matrix {
        assert_eq!(coordinates.len(), self.slot_count());
        let mut transform = Matrix::identity(self.state_dimension);
        for (coordinate, direction) in coordinates.iter().zip(&self.directions) {
            transform.add_scaled(*coordinate, direction);
        }
        transform
    }

    pub fn lift(&self, item: &[f64]) -> Vec<f64> {
        let env = EvalEnv {
            item: Some(item),
            a: None,
            b: None,
            f: None,
        };
        self.into.iter().map(|expression| eval(expression, &env)).collect()
    }

    /// Compose the left chunk followed by the right chunk.
    pub fn merge(&self, left: &[f64], right: &[f64]) -> Vec<f64> {
        assert_eq!(left.len(), self.slot_count());
        assert_eq!(right.len(), self.slot_count());
        let env = EvalEnv {
            item: None,
            a: Some(left),
            b: Some(right),
            f: None,
        };
        self.combine.iter().map(|expression| eval(expression, &env)).collect()
    }

    pub fn project(&self, state: &[f64]) -> Vec<f64> {
        assert_eq!(state.len(), self.slot_count());
        let env = EvalEnv {
            item: None,
            a: None,
            b: None,
            f: Some(state),
        };
        self.project.iter().map(|expression| eval(expression, &env)).collect()
    }

    pub fn fold_acc(&self, items: &[Vec<f64>]) -> Vec<f64> {
        items.iter().fold(self.identity.clone(), |state, item| {
            self.merge(&state, &self.lift(item))
        })
    }

    pub fn fold(&self, items: &[Vec<f64>]) -> Vec<f64> {
        self.project(&self.fold_acc(items))
    }

    pub fn tree_fold(&self, items: &[Vec<f64>]) -> Vec<f64> {
        fn tree(carrier: &GeneratedCarrier, items: &[Vec<f64>]) -> Vec<f64> {
            match items {
                [] => carrier.identity.clone(),
                [item] => carrier.lift(item),
                _ => {
                    let middle = items.len() / 2;
                    let left = tree(carrier, &items[..middle]);
                    let right = tree(carrier, &items[middle..]);
                    carrier.merge(&left, &right)
                }
            }
        }
        self.project(&tree(self, items))
    }
}

impl TransitionFamily {
    /// Close this presentation under program-order composition.
    pub fn close(self, tolerance: f64) -> Result<GeneratedCarrier, TransitionClosureError> {
        self.validate(tolerance)?;

        let mut basis = Vec::with_capacity(self.state_dimension * self.state_dimension);
        for (index, direction) in self.directions.into_iter().enumerate() {
            if span_coordinates(&direction, &basis, tolerance)?.is_some() {
                return Err(TransitionClosureError::DependentDirection(index));
            }
            basis.push(direction);
        }

        close_multiplicative_span(&mut basis, tolerance)?;
        let structure = structure_constants(&basis, tolerance)?;

        let original_coordinates = self.singleton_coordinates.len();
        let into = (0..basis.len())
            .map(|index| {
                if index < original_coordinates {
                    self.singleton_coordinates[index].clone()
                } else {
                    Expr::Const(0.0)
                }
            })
            .collect();
        let combine: Vec<_> = (0..basis.len())
            .map(|output| combine_expression(output, &structure))
            .collect();
        let schema = LogicalStateSchema {
            components: combine
                .iter()
                .enumerate()
                .map(|(slot, expression)| StateComponent {
                    span: Vec::new(),
                    dependencies: combine_dependencies(expression, slot),
                    construction: ComponentConstruction::GeneratedTransition,
                })
                .collect(),
        };

        let mut projection = Expr::Const(dot(&self.observation, &self.initial_state));
        for (slot, direction) in basis.iter().enumerate() {
            let coefficient = dot(&self.observation, &direction.apply(&self.initial_state));
            projection = add(projection, scale(coefficient, Expr::F(slot)));
        }

        Ok(GeneratedCarrier {
            into,
            combine,
            identity: vec![0.0; basis.len()],
            project: vec![projection],
            schema,
            laws: LawCertificate::transition_composition(),
            state_dimension: self.state_dimension,
            directions: basis,
        })
    }

    fn validate(&self, tolerance: f64) -> Result<(), TransitionClosureError> {
        if self.state_dimension == 0 {
            return Err(TransitionClosureError::ZeroStateDimension);
        }
        if !tolerance.is_finite() || tolerance <= 0.0 {
            return Err(TransitionClosureError::InvalidTolerance(tolerance));
        }
        if self.directions.len() != self.singleton_coordinates.len() {
            return Err(TransitionClosureError::CoordinateCount {
                directions: self.directions.len(),
                coordinates: self.singleton_coordinates.len(),
            });
        }
        for direction in &self.directions {
            if direction.dimension != self.state_dimension {
                return Err(TransitionClosureError::MatrixDimension {
                    expected: self.state_dimension,
                    actual: direction.dimension,
                });
            }
            if direction.entries.iter().any(|value| !value.is_finite()) {
                return Err(TransitionClosureError::NonFiniteCoefficient);
            }
        }
        if self.initial_state.len() != self.state_dimension {
            return Err(TransitionClosureError::StateVectorDimension {
                expected: self.state_dimension,
                actual: self.initial_state.len(),
            });
        }
        if self.observation.len() != self.state_dimension {
            return Err(TransitionClosureError::ObservationDimension {
                expected: self.state_dimension,
                actual: self.observation.len(),
            });
        }
        if self
            .initial_state
            .iter()
            .chain(&self.observation)
            .any(|value| !value.is_finite())
        {
            return Err(TransitionClosureError::NonFiniteCoefficient);
        }
        Ok(())
    }
}

/// Generate the two-coordinate carrier for `q' = scale(item) * q + bias(item)`.
///
/// The raw coordinates are `(A - 1, B)`; decoding adds the identity back, so
/// the represented chunk transformation is `q -> A*q + B`.
pub fn affine_transition_carrier(
    scale_expression: Expr,
    bias_expression: Expr,
    initial_state: f64,
) -> Result<GeneratedCarrier, TransitionClosureError> {
    TransitionFamily {
        state_dimension: 2,
        directions: vec![
            Matrix::from_rows([[1.0, 0.0], [0.0, 0.0]]),
            Matrix::from_rows([[0.0, 1.0], [0.0, 0.0]]),
        ],
        singleton_coordinates: vec![
            Expr::Sub(Box::new(scale_expression), Box::new(Expr::Const(1.0))),
            bias_expression,
        ],
        initial_state: vec![initial_state, 1.0],
        observation: vec![1.0, 0.0],
    }
    .close(1e-10)
}

fn close_multiplicative_span(basis: &mut Vec<Matrix>, tolerance: f64) -> Result<(), TransitionClosureError> {
    let maximum = basis
        .first()
        .map(|matrix| matrix.dimension * matrix.dimension)
        .unwrap_or(0);
    loop {
        let mut new_direction = None;
        'products: for right in 0..basis.len() {
            for left in 0..basis.len() {
                let product = basis[right].multiply(&basis[left]);
                if span_coordinates(&product, basis, tolerance)?.is_none() {
                    new_direction = Some(normalized_residual(&product, basis, tolerance)?);
                    break 'products;
                }
            }
        }
        let Some(direction) = new_direction else {
            return Ok(());
        };
        if basis.len() == maximum {
            return Err(TransitionClosureError::ClosureDimensionExceeded { maximum });
        }
        basis.push(direction);
    }
}

/// `structure[k][right][left]` is the coefficient of basis `k` in
/// `basis[right] * basis[left]`.
fn structure_constants(basis: &[Matrix], tolerance: f64) -> Result<Vec<Vec<Vec<f64>>>, TransitionClosureError> {
    let size = basis.len();
    let mut structure = vec![vec![vec![0.0; size]; size]; size];
    for right in 0..size {
        for left in 0..size {
            let product = basis[right].multiply(&basis[left]);
            let coordinates =
                span_coordinates(&product, basis, tolerance)?.ok_or(TransitionClosureError::NumericalClosureFailure)?;
            for output in 0..size {
                structure[output][right][left] = clean(coordinates[output], tolerance);
            }
        }
    }
    Ok(structure)
}

fn combine_expression(output: usize, structure: &[Vec<Vec<f64>>]) -> Expr {
    let mut expression = add(Expr::A(output), Expr::B(output));
    for (right, coefficients) in structure[output].iter().enumerate() {
        for (left, &coefficient) in coefficients.iter().enumerate() {
            let product = multiply(Expr::A(left), Expr::B(right));
            expression = add(expression, scale(coefficient, product));
        }
    }
    expression
}

fn add(left: Expr, right: Expr) -> Expr {
    match (&left, &right) {
        (Expr::Const(value), _) if *value == 0.0 => right,
        (_, Expr::Const(value)) if *value == 0.0 => left,
        _ => Expr::Add(Box::new(left), Box::new(right)),
    }
}

fn multiply(left: Expr, right: Expr) -> Expr {
    match (&left, &right) {
        (Expr::Const(value), _) | (_, Expr::Const(value)) if *value == 0.0 => Expr::Const(0.0),
        (Expr::Const(value), _) if *value == 1.0 => right,
        (_, Expr::Const(value)) if *value == 1.0 => left,
        _ => Expr::Mul(Box::new(left), Box::new(right)),
    }
}

fn scale(coefficient: f64, expression: Expr) -> Expr {
    multiply(Expr::Const(coefficient), expression)
}

fn clean(value: f64, tolerance: f64) -> f64 {
    if value.abs() <= tolerance {
        0.0
    } else if (value - value.round()).abs() <= tolerance {
        value.round()
    } else {
        value
    }
}

fn dot(left: &[f64], right: &[f64]) -> f64 {
    left.iter().zip(right).map(|(left, right)| left * right).sum()
}

fn norm(values: &[f64]) -> f64 {
    dot(values, values).sqrt()
}

/// Coordinates in `basis`, or `None` when `target` has an independent
/// residual. Modified Gram-Schmidt is sufficient here because the ambient
/// dimension is at most the square of the (small) sequential-state dimension.
fn span_coordinates(
    target: &Matrix,
    basis: &[Matrix],
    tolerance: f64,
) -> Result<Option<Vec<f64>>, TransitionClosureError> {
    if basis.is_empty() {
        return Ok((norm(&target.entries) <= tolerance).then(Vec::new));
    }
    let (q, r) = qr_basis(basis, tolerance)?;
    let y: Vec<_> = q.iter().map(|column| dot(column, &target.entries)).collect();
    let mut coordinates = vec![0.0; basis.len()];
    for row in (0..basis.len()).rev() {
        let tail: f64 = (row + 1..basis.len())
            .map(|column| r[row][column] * coordinates[column])
            .sum();
        coordinates[row] = (y[row] - tail) / r[row][row];
    }
    let mut residual = target.entries.clone();
    for (coefficient, direction) in coordinates.iter().zip(basis) {
        for (value, basis_value) in residual.iter_mut().zip(&direction.entries) {
            *value -= coefficient * basis_value;
        }
    }
    let scale = norm(&target.entries).max(1.0);
    Ok((norm(&residual) <= tolerance * scale).then_some(coordinates))
}

fn normalized_residual(target: &Matrix, basis: &[Matrix], tolerance: f64) -> Result<Matrix, TransitionClosureError> {
    let (q, _) = qr_basis(basis, tolerance)?;
    let mut residual = target.entries.clone();
    for column in q {
        let coefficient = dot(&column, &residual);
        for (value, basis_value) in residual.iter_mut().zip(column) {
            *value -= coefficient * basis_value;
        }
    }
    let length = norm(&residual);
    if length <= tolerance * norm(&target.entries).max(1.0) {
        return Err(TransitionClosureError::NumericalClosureFailure);
    }
    for value in &mut residual {
        *value /= length;
    }
    Matrix::new(target.dimension, residual)
}

fn qr_basis(basis: &[Matrix], tolerance: f64) -> Result<(DenseVectors, DenseVectors), TransitionClosureError> {
    let size = basis.len();
    let mut q: Vec<Vec<f64>> = Vec::with_capacity(size);
    let mut r = vec![vec![0.0; size]; size];
    for column in 0..size {
        let mut vector = basis[column].entries.clone();
        for row in 0..column {
            r[row][column] = dot(&q[row], &vector);
            for (value, basis_value) in vector.iter_mut().zip(&q[row]) {
                *value -= r[row][column] * basis_value;
            }
        }
        r[column][column] = norm(&vector);
        if r[column][column] <= tolerance {
            return Err(TransitionClosureError::DependentDirection(column));
        }
        for value in &mut vector {
            *value /= r[column][column];
        }
        q.push(vector);
    }
    Ok((q, r))
}

fn combine_dependencies(expression: &Expr, own_slot: usize) -> Vec<usize> {
    let mut references = Vec::new();
    fn walk(expression: &Expr, references: &mut Vec<usize>) {
        match expression {
            Expr::A(slot) | Expr::B(slot) => {
                if !references.contains(slot) {
                    references.push(*slot);
                }
            }
            Expr::Add(left, right)
            | Expr::Sub(left, right)
            | Expr::Mul(left, right)
            | Expr::Div(left, right)
            | Expr::Max(left, right)
            | Expr::Min(left, right)
            | Expr::Lt(left, right) => {
                walk(left, references);
                walk(right, references);
            }
            Expr::Exp(value)
            | Expr::Log(value)
            | Expr::Sqrt(value)
            | Expr::Tanh(value)
            | Expr::Sin(value)
            | Expr::Cos(value) => walk(value, references),
            Expr::Where(condition, yes, no) => {
                walk(condition, references);
                walk(yes, references);
                walk(no, references);
            }
            Expr::Const(_) | Expr::Item(_) | Expr::F(_) => {}
        }
    }
    walk(expression, &mut references);
    references.retain(|slot| *slot != own_slot);
    references.sort_unstable();
    references
}

struct EvalEnv<'a> {
    item: Option<&'a [f64]>,
    a: Option<&'a [f64]>,
    b: Option<&'a [f64]>,
    f: Option<&'a [f64]>,
}

fn eval(expression: &Expr, env: &EvalEnv<'_>) -> f64 {
    match expression {
        Expr::Const(value) => *value,
        Expr::Item(index) => env.item.expect("Item outside into")[*index],
        Expr::A(index) => env.a.expect("A outside combine")[*index],
        Expr::B(index) => env.b.expect("B outside combine")[*index],
        Expr::F(index) => env.f.expect("F outside project")[*index],
        Expr::Add(left, right) => eval(left, env) + eval(right, env),
        Expr::Sub(left, right) => eval(left, env) - eval(right, env),
        Expr::Mul(left, right) => eval(left, env) * eval(right, env),
        Expr::Div(left, right) => eval(left, env) / eval(right, env),
        Expr::Max(left, right) => eval(left, env).max(eval(right, env)),
        Expr::Min(left, right) => eval(left, env).min(eval(right, env)),
        Expr::Lt(left, right) => (eval(left, env) < eval(right, env)) as u8 as f64,
        Expr::Exp(value) => eval(value, env).exp(),
        Expr::Log(value) => eval(value, env).ln(),
        Expr::Sqrt(value) => eval(value, env).sqrt(),
        Expr::Tanh(value) => eval(value, env).tanh(),
        Expr::Sin(value) => eval(value, env).sin(),
        Expr::Cos(value) => eval(value, env).cos(),
        Expr::Where(condition, yes, no) => {
            if eval(condition, env) != 0.0 {
                eval(yes, env)
            } else {
                eval(no, env)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derive::{AssociativityEvidence, MergeOrderEvidence};

    fn close(left: f64, right: f64) -> bool {
        (left - right).abs() <= 1e-10 * left.abs().max(right.abs()).max(1.0)
    }

    #[test]
    fn affine_closure_generates_the_two_coordinate_program() {
        let carrier = affine_transition_carrier(Expr::Item(0), Expr::Item(1), 4.0).unwrap();
        assert_eq!(carrier.state_dimension(), 2);
        assert_eq!(carrier.slot_count(), 2);
        assert_eq!(carrier.laws.associativity, AssociativityEvidence::TransitionComposition);
        assert_eq!(carrier.laws.merge_order, MergeOrderEvidence::ProgramOrder);
        assert!(!carrier.mergeable_out_of_order());
        assert!(carrier.schema.components[0].dependencies.is_empty());
        assert_eq!(carrier.schema.components[1].dependencies, [0]);
        assert!(
            carrier
                .schema
                .components
                .iter()
                .all(|component| component.construction == ComponentConstruction::GeneratedTransition)
        );

        let items = vec![vec![2.0, 1.0], vec![-0.5, 3.0], vec![1.25, -2.0], vec![0.75, 0.5]];
        let expected = items.iter().fold(4.0, |state, item| item[0] * state + item[1]);
        assert!(close(carrier.fold(&items)[0], expected));
        assert!(close(carrier.tree_fold(&items)[0], expected));

        let coordinates = carrier.fold_acc(&items);
        let transform = carrier.decoded_transform(&coordinates);
        assert!(close(transform.apply(&[4.0, 1.0])[0], expected));

        // The first two chunks make the textbook program-order law:
        // (A1,B1) <> (A2,B2) = (A2*A1, A2*B1+B2).
        let first = carrier.lift(&items[0]);
        let second = carrier.lift(&items[1]);
        let merged = carrier.decoded_transform(&carrier.merge(&first, &second));
        assert!(close(merged.entries()[0], -1.0));
        assert!(close(merged.entries()[1], 2.5));

        let third = carrier.lift(&items[2]);
        let left_grouped = carrier.merge(&carrier.merge(&first, &second), &third);
        let right_grouped = carrier.merge(&first, &carrier.merge(&second, &third));
        assert!(
            left_grouped
                .iter()
                .zip(right_grouped)
                .all(|(left, right)| close(*left, right))
        );
    }

    #[test]
    fn closure_adds_missing_product_directions() {
        // E01 * E12 = E02, so the two singleton directions generate a third.
        let family = TransitionFamily {
            state_dimension: 3,
            directions: vec![
                Matrix::from_rows([[0.0, 1.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]),
                Matrix::from_rows([[0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, 0.0]]),
            ],
            singleton_coordinates: vec![Expr::Item(0), Expr::Item(1)],
            initial_state: vec![0.0, 0.0, 1.0],
            observation: vec![1.0, 0.0, 0.0],
        };
        let carrier = family.close(1e-10).unwrap();
        assert_eq!(carrier.slot_count(), 3);

        let left = carrier.lift(&[0.0, 2.0]);
        let right = carrier.lift(&[3.0, 0.0]);
        let composed = carrier.merge(&left, &right);
        assert!(close(
            carrier.decoded_transform(&composed).apply(&[0.0, 0.0, 1.0])[0],
            6.0
        ));
    }

    #[test]
    fn program_order_is_observably_noncommutative() {
        let carrier = affine_transition_carrier(Expr::Item(0), Expr::Item(1), 0.0).unwrap();
        let scale = carrier.lift(&[2.0, 0.0]);
        let bias = carrier.lift(&[1.0, 3.0]);
        assert_ne!(carrier.merge(&scale, &bias), carrier.merge(&bias, &scale));
    }
}
