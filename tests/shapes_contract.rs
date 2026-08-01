//! The contract for `shapes_and_indexing.md`, written before the
//! implementation — the origin `gamma.rs` records for itself ("began as a
//! model in tests/contract.rs"). Model types and laws as passing tests;
//! promotion moves the laws into `ir::Resolver` and `grad`, then deletes
//! this file.
//!
//! What the type system makes unwritable, by clause:
//!
//! - **Law 1, one answerer** — [`Correspondence`]'s fields are private and
//!   [`Correspondence::mint`] is the one constructor. A pass may query a
//!   map, never build one, so six mechanisms disagreeing about it becomes
//!   unwritable. In src this is `ir::Resolver` finished, not a second
//!   mechanism beside it. (Privacy binds at promotion: inside this one
//!   module a struct literal could bypass `mint`; what the file pins now
//!   is the vocabulary's shape, and that the zoo never needs the bypass.)
//! - **Law 3, structure stays stated** — [`AffineDim`] is coefficient
//!   terms and an offset. There is no division, no modulus, and no data
//!   term: a reshape cannot be erased into arithmetic here, and a gather
//!   cannot be written at all. The field list is the law.
//! - **Law 5, extents** — [`Extent::cap`] is total, so pricing and
//!   allocation never meet a panic; [`Extent::at`] is partial, so a
//!   concrete value exists only under a binding. Today's panicking
//!   `extent()` is absent from this vocabulary.
//! - **§5, earned Dense** — [`Adjoint::Dense`] is a verdict the classifier
//!   must earn (shared support within one index form). Bijective,
//!   injective, and broadcast maps cannot reach it, and every verdict is
//!   checked against the enumerated preimage structure of the actual
//!   relation, which is what keeps the verdict honest: the adjoint's
//!   value at a source point is the sum of the cotangent over that
//!   point's preimages, so preimage structure IS the recipe — all size 1
//!   and covering means one movement read, uniform fibers mean
//!   sum-then-move, anything else is the contraction. The check is
//!   pointwise over the zoo — the maps the pipeline actually mints — not
//!   a proof over the vocabulary; off the zoo the symbolic side can
//!   widen to `Dense`, the direction that costs work, never correctness
//!   (`off_the_zoo_the_symbolic_verdict_widens_to_dense`).

// ── Law 5: the one integer form ──────────────────────────────────────────────

/// An axis extent. `Sym` is bind-late: unknown until run, never above
/// `cap`. This replaces `Extent::Dynamic`, whose only operation today is
/// a panic (ir.rs:35).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Extent {
    Static(usize),
    Sym { id: u32, cap: usize },
}

impl Extent {
    /// Total — the allocation and pricing bound. tinygrad's precedent:
    /// symbolic buffers allocate at `vmax` and shrink at bind.
    fn cap(self) -> usize {
        match self {
            Extent::Static(n) => n,
            Extent::Sym { cap, .. } => cap,
        }
    }

    /// Partial — the concrete value exists only under a binding, and a
    /// binding above `cap` is a caller defect, refused rather than
    /// clamped: `cap` is a promise the allocator already spent.
    fn at(self, bindings: &[(u32, usize)]) -> Option<usize> {
        match self {
            Extent::Static(n) => Some(n),
            Extent::Sym { id, cap } => bindings
                .iter()
                .find(|(bound_id, _)| *bound_id == id)
                .map(|&(_, value)| value)
                .filter(|&value| value <= cap),
        }
    }
}

// ── Laws 1–3: the correspondence ─────────────────────────────────────────────

/// One functional-side dimension's index, affine in the other side's
/// coordinates. No `Div`, no `Mod`, no data term — a merge stores its
/// factorization as structure (the Write direction), never as arithmetic
/// to be re-factorized, and a gather is simply not a value of this type.
#[derive(Clone, Debug, PartialEq, Eq)]
struct AffineDim {
    terms: Vec<(i64, usize)>,
    offset: i64,
}

/// Which side the affine functions compute. `Reindex` stores `Source`
/// (read maps: source coords from output coords); `View` stores `Output`
/// (write maps: output coords from source coords). Same data shape both
/// ways — which is why [`Correspondence::transposed`] is free.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Functional {
    Source,
    Output,
}

/// The side a query speaks about, independent of what the node stores.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Side {
    Output,
    Source,
}

/// A movement's map: the one object behind `axis_refs_rc`,
/// `axis_aliases`, `support_below`, `relocate_axis`, `preserve_shape`,
/// and the grad shape helpers. Private fields; [`Correspondence::mint`]
/// is the only constructor.
#[derive(Clone, Debug, PartialEq)]
struct Correspondence {
    functional: Functional,
    /// Indexed by functional-side dimension.
    dims: Vec<AffineDim>,
    out_shape: Vec<usize>,
    src_shape: Vec<usize>,
}

impl Correspondence {
    /// The one minting site. A write map must be a mixed-radix partition of
    /// the source dimensions — each consumed exactly once, strides the
    /// suffix products, each output extent its group's product:
    /// `positional_view`'s own law, stated here in full because this model
    /// stores coefficients where `View` stores groups. All three clauses
    /// are what make every `View` bijective by construction and its
    /// adjoint exact; consumption alone is not enough (see
    /// `write_maps_must_fill_their_output`).
    fn mint(
        functional: Functional,
        dims: Vec<AffineDim>,
        out_shape: Vec<usize>,
        src_shape: Vec<usize>,
    ) -> Correspondence {
        let (functional_len, other_len) = match functional {
            Functional::Source => (src_shape.len(), out_shape.len()),
            Functional::Output => (out_shape.len(), src_shape.len()),
        };
        assert_eq!(
            dims.len(),
            functional_len,
            "every functional-side dimension mapped exactly once"
        );
        for dim in &dims {
            for &(coefficient, other_dim) in &dim.terms {
                assert!(other_dim < other_len, "term references a dimension the other side has");
                assert!(coefficient != 0, "a zero coefficient is a term that is not there");
            }
        }
        if functional == Functional::Output {
            let mut consumed = vec![false; other_len];
            for (mapped, &out_extent) in dims.iter().zip(&out_shape) {
                assert_eq!(mapped.offset, 0, "a write map is a recomposition — no offset");
                let mut sorted = mapped.terms.clone();
                sorted.sort_by_key(|&(coefficient, _)| -coefficient);
                let mut suffix_product = 1i64;
                for &(coefficient, source_dim) in sorted.iter().rev() {
                    assert!(
                        !std::mem::replace(&mut consumed[source_dim], true),
                        "write map consumes a source dimension twice"
                    );
                    assert_eq!(
                        coefficient, suffix_product,
                        "write map strides must be the group's suffix products"
                    );
                    suffix_product *= src_shape[source_dim] as i64;
                }
                assert_eq!(
                    out_extent as i64, suffix_product,
                    "write map output extent must equal its source group's product"
                );
            }
            assert!(
                consumed.into_iter().all(|c| c),
                "write map must consume every source dimension"
            );
        }
        Correspondence {
            functional,
            dims,
            out_shape,
            src_shape,
        }
    }

    fn functional_side(&self) -> Side {
        match self.functional {
            Functional::Source => Side::Source,
            Functional::Output => Side::Output,
        }
    }

    /// The identity component: `dim` carries the same loop variable as
    /// exactly one dimension of the other side — coefficient 1, offset 0,
    /// sole reader. This is `axis_refs_rc`'s pass-through rule
    /// (ir.rs:447-462) and every entry `axis_aliases` records, as one
    /// question answerable from either side.
    fn identity(&self, side: Side, dim: usize) -> Option<usize> {
        if side == self.functional_side() {
            let mapped = &self.dims[dim];
            match (mapped.terms.as_slice(), mapped.offset) {
                ([(1, other)], 0) if self.readers_of(*other) == 1 => Some(*other),
                _ => None,
            }
        } else {
            let mut readers = self
                .dims
                .iter()
                .enumerate()
                .filter(|(_, mapped)| mapped.terms.iter().any(|&(_, other)| other == dim));
            match (readers.next(), readers.next()) {
                (Some((functional_dim, mapped)), None)
                    if mapped.terms.as_slice() == [(1, dim)] && mapped.offset == 0 =>
                {
                    Some(functional_dim)
                }
                _ => None,
            }
        }
    }

    /// The dependency component: every other-side dimension whose
    /// coordinate `dim`'s index involves — `Resolver::support_below`'s
    /// question (ir.rs; born as `stream_below_*`, folded at M-A), from
    /// either side.
    fn support(&self, side: Side, dim: usize) -> Vec<usize> {
        if side == self.functional_side() {
            self.dims[dim].terms.iter().map(|&(_, other)| other).collect()
        } else {
            self.dims
                .iter()
                .enumerate()
                .filter(|(_, mapped)| mapped.terms.iter().any(|&(_, other)| other == dim))
                .map(|(functional_dim, _)| functional_dim)
                .collect()
        }
    }

    fn readers_of(&self, other_dim: usize) -> usize {
        self.dims
            .iter()
            .filter(|mapped| mapped.terms.iter().any(|&(_, other)| other == other_dim))
            .count()
    }

    /// Free, and the point of storing direction instead of normalizing it:
    /// the transpose swaps which side the same affine data describes. A
    /// merge written as a view IS a split read as a reindex — no
    /// arithmetic is recomputed, and by Law 3 none could be.
    fn transposed(mut self) -> Correspondence {
        std::mem::swap(&mut self.out_shape, &mut self.src_shape);
        self.functional = match self.functional {
            Functional::Source => Functional::Output,
            Functional::Output => Functional::Source,
        };
        self
    }
}

// ── §5: the adjoint, classified ──────────────────────────────────────────────

/// `shapes_and_indexing.md` §5's table as a type. In src, `Movement`
/// emits one `Reindex`/`View` node (today: `invert_view`, exact),
/// `Padded` the same plus zero-fill, `SumThenMove` a reduction then a
/// movement (today: `reduce_to`), and `Dense` the equality-mask
/// contraction that `transpose_reindex` currently emits for EVERY
/// reindex — including a `split`, whose verdict here is `Movement`.
#[derive(Debug, PartialEq)]
enum Adjoint {
    Movement,
    Padded,
    SumThenMove { summed: Vec<usize> },
    Dense,
}

impl Correspondence {
    /// The symbolic verdict, from coefficients and extents alone. The
    /// tests hold it equal to [`enumerated_verdict`] — the preimage
    /// structure of the actual relation — which is the whole claim: the
    /// classification is decidable without enumerating anything.
    fn adjoint_class(&self) -> Adjoint {
        // A write map is a mixed-radix partition of the source dimensions
        // — all three clauses enforced at the mint, so bijective by
        // construction and the adjoint is one movement (`positional_view`).
        if self.functional == Functional::Output {
            return Adjoint::Movement;
        }

        // Shared support: one output dimension read by two source
        // dimensions couples their fibers — the contraction is the math.
        if (0..self.out_shape.len()).any(|out_dim| self.readers_of(out_dim) > 1) {
            return Adjoint::Dense;
        }

        // Per-dimension verdicts over disjoint output dimensions.
        let mut all_bijective = true;
        for (source_dim, mapped) in self.dims.iter().enumerate() {
            match per_dim_class(mapped, &self.out_shape, self.src_shape[source_dim]) {
                PerDim::Bijective => {}
                PerDim::Injective => all_bijective = false,
                PerDim::Coupled => return Adjoint::Dense,
            }
        }

        // Output dimensions no source index involves: the map is constant
        // along them, and the adjoint sums their fibers first.
        let summed: Vec<usize> = (0..self.out_shape.len())
            .filter(|&out_dim| self.readers_of(out_dim) == 0)
            .collect();
        if !summed.is_empty() {
            return Adjoint::SumThenMove { summed };
        }
        if all_bijective {
            Adjoint::Movement
        } else {
            Adjoint::Padded
        }
    }
}

enum PerDim {
    Bijective,
    Injective,
    Coupled,
}

/// One source dimension's affine form, classified. Single term: injective
/// for any nonzero coefficient; bijective when the valid output range maps
/// exactly onto `[0, source_extent)`. Multiple terms: bijective exactly
/// when they form a mixed radix (each coefficient the product of the later
/// terms' extents, zero offset) — a stated factorization, split/merge —
/// and coupled otherwise (an overlapping window's `i + j`).
fn per_dim_class(mapped: &AffineDim, out_shape: &[usize], source_extent: usize) -> PerDim {
    match mapped.terms.as_slice() {
        [] => PerDim::Injective, // a constant index: one source point hit
        &[(coefficient, out_dim)] => {
            let out_extent = out_shape[out_dim] as i64;
            let hits = (0..out_extent)
                .filter(|x| {
                    let index = coefficient * x + mapped.offset;
                    (0..source_extent as i64).contains(&index)
                })
                .count();
            if coefficient.abs() == 1 && hits == source_extent {
                PerDim::Bijective
            } else {
                PerDim::Injective
            }
        }
        terms => {
            let mut sorted = terms.to_vec();
            sorted.sort_by_key(|&(coefficient, _)| -coefficient);
            let mut suffix_product = 1i64;
            let mut span = 1i64;
            for &(coefficient, out_dim) in sorted.iter().rev() {
                if coefficient != suffix_product {
                    return PerDim::Coupled;
                }
                suffix_product *= out_shape[out_dim] as i64;
                span *= out_shape[out_dim] as i64;
            }
            if mapped.offset == 0 && span == source_extent as i64 {
                PerDim::Bijective
            } else {
                PerDim::Coupled
            }
        }
    }
}

// ── the oracle: enumerated preimage structure ────────────────────────────────

fn points(shape: &[usize]) -> Vec<Vec<usize>> {
    let mut out = vec![Vec::new()];
    for &extent in shape {
        out = out
            .into_iter()
            .flat_map(|p| {
                (0..extent).map(move |coordinate| {
                    let mut q = p.clone();
                    q.push(coordinate);
                    q
                })
            })
            .collect();
    }
    out
}

impl Correspondence {
    /// Apply the stored affine functions to an other-side point. `None`
    /// is an out-of-bounds functional coordinate — a padded read.
    fn apply(&self, other: &[usize]) -> Option<Vec<usize>> {
        let functional_shape = match self.functional {
            Functional::Source => &self.src_shape,
            Functional::Output => &self.out_shape,
        };
        self.dims
            .iter()
            .zip(functional_shape)
            .map(|(mapped, &extent)| {
                let index = mapped
                    .terms
                    .iter()
                    .fold(mapped.offset, |acc, &(coefficient, other_dim)| {
                        acc + coefficient * other[other_dim] as i64
                    });
                (0..extent as i64).contains(&index).then_some(index as usize)
            })
            .collect()
    }

    /// The relation as (output point, source point) pairs — every valid
    /// read (Read maps) or placement (Write maps).
    fn relation(&self) -> Vec<(Vec<usize>, Vec<usize>)> {
        match self.functional {
            Functional::Source => points(&self.out_shape)
                .into_iter()
                .filter_map(|out| self.apply(&out).map(|src| (out, src)))
                .collect(),
            Functional::Output => points(&self.src_shape)
                .into_iter()
                .filter_map(|src| self.apply(&src).map(|out| (out, src)))
                .collect(),
        }
    }
}

/// The verdict read off the actual relation: preimage counts per source
/// point. All 1 and covering → one movement read. All ≤ 1 → movement
/// plus zero-fill. Uniform fibers of exactly the unread dimensions'
/// volume → sum then move. Anything else → the contraction.
fn enumerated_verdict(map: &Correspondence) -> Adjoint {
    let relation = map.relation();
    let mut preimages: std::collections::HashMap<Vec<usize>, usize> = std::collections::HashMap::new();
    for (_, src) in &relation {
        *preimages.entry(src.clone()).or_default() += 1;
    }
    let source_points = points(&map.src_shape);
    let counts: Vec<usize> = source_points
        .iter()
        .map(|src| preimages.get(src).copied().unwrap_or(0))
        .collect();

    // A summed dimension, read off the relation itself (not the
    // coefficients — the oracle must not restate the classifier): the
    // relation is invariant under sliding the output point along it.
    let pairs: std::collections::HashSet<(Vec<usize>, Vec<usize>)> = relation.iter().cloned().collect();
    let summed: Vec<usize> = (0..map.out_shape.len())
        .filter(|&out_dim| {
            relation.iter().all(|(out, src)| {
                (0..map.out_shape[out_dim]).all(|slid| {
                    let mut moved = out.clone();
                    moved[out_dim] = slid;
                    pairs.contains(&(moved, src.clone()))
                })
            })
        })
        .filter(|&out_dim| map.out_shape[out_dim] > 1) // extent 1: sliding is vacuous, no fiber to sum
        .collect();
    let fiber: usize = summed.iter().map(|&out_dim| map.out_shape[out_dim]).product();

    if !summed.is_empty() && counts.iter().all(|&c| c == fiber) {
        return Adjoint::SumThenMove { summed };
    }
    if counts.iter().all(|&c| c == 1) {
        return Adjoint::Movement;
    }
    if counts.iter().all(|&c| c <= 1) {
        return Adjoint::Padded;
    }
    Adjoint::Dense
}

// ── M-D, scoped: a rewrite states its frame ──────────────────────────────────

/// What a rank- or order-changing rewrite must return beside its node:
/// where each replacement dimension came from. The field is
/// position → origin, so `preserve_shape`'s extent-matching arm
/// (simplify.rs:317-321) — the one that contradicts `relocate_axis`'s
/// positional law — has nothing to match against; like `mint`'s privacy,
/// that becomes enforced (not merely stated) at promotion, when the type
/// lands behind a module boundary and rewrites must produce one.
struct Frame {
    /// Replacement dimension → original dimension; `None` is a minted
    /// dimension the rewrite introduced.
    dims: Vec<Option<usize>>,
}

// ── the zoo ──────────────────────────────────────────────────────────────────

fn read_map(dims: Vec<AffineDim>, out_shape: Vec<usize>, src_shape: Vec<usize>) -> Correspondence {
    Correspondence::mint(Functional::Source, dims, out_shape, src_shape)
}

fn term(coefficient: i64, dim: usize) -> AffineDim {
    AffineDim {
        terms: vec![(coefficient, dim)],
        offset: 0,
    }
}

/// `split(6 → 2×3)` as `Reindex` mints it (ir.rs:814-832): src0 = 3·out0 + out1.
fn split_6_into_2x3() -> Correspondence {
    read_map(
        vec![AffineDim {
            terms: vec![(3, 0), (1, 1)],
            offset: 0,
        }],
        vec![2, 3],
        vec![6],
    )
}

/// `flatten(2×3 → 6)` as `View` mints it: out0 = 3·src0 + src1, a write map.
fn merge_2x3_into_6() -> Correspondence {
    Correspondence::mint(
        Functional::Output,
        vec![AffineDim {
            terms: vec![(3, 0), (1, 1)],
            offset: 0,
        }],
        vec![6],
        vec![2, 3],
    )
}

// ── the laws, as passing tests ───────────────────────────────────────────────

#[test]
fn extent_cap_is_total_and_at_is_partial() {
    let kv = Extent::Sym { id: 0, cap: 1030 };
    assert_eq!(kv.cap(), 1030); // pricing and allocation, no binding needed
    assert_eq!(kv.at(&[(0, 517)]), Some(517)); // execution, bound
    assert_eq!(kv.at(&[]), None); // unbound is refusable, not a panic
    assert_eq!(kv.at(&[(0, 2048)]), None); // above cap: the promise was spent
    assert_eq!(Extent::Static(64).at(&[]), Some(64));
}

#[test]
fn transpose_is_free_and_involutive() {
    // A merge written as a View IS a split read as a Reindex: same affine
    // data, roles swapped. Nothing is recomputed to know it.
    assert_eq!(merge_2x3_into_6().transposed(), split_6_into_2x3());
    assert_eq!(split_6_into_2x3().transposed().transposed(), split_6_into_2x3());
}

#[test]
#[should_panic(expected = "consumes a source dimension twice")]
fn write_maps_must_be_partitions() {
    // positional_view's law, enforced at the mint: a write map reading
    // one source dimension twice is not a View.
    Correspondence::mint(Functional::Output, vec![term(1, 0), term(1, 0)], vec![2, 2], vec![2]);
}

#[test]
#[should_panic(expected = "output extent must equal its source group's product")]
fn write_maps_must_fill_their_output() {
    // The law's other half: consumption alone is not bijectivity. out=[5]
    // cannot hold the 2×3 mixed radix — source point (1,2) would land at
    // 5, out of range — so "bijective by construction" would be a lie the
    // classifier repeats (`Movement`) and the enumerated relation refutes
    // (`Padded`). The mint refuses the map instead.
    Correspondence::mint(
        Functional::Output,
        vec![AffineDim {
            terms: vec![(3, 0), (1, 1)],
            offset: 0,
        }],
        vec![5],
        vec![2, 3],
    );
}

#[test]
fn identity_and_support_answer_the_six_mechanisms() {
    let split = split_6_into_2x3();
    // axis_aliases' question: no identity through a split's transformed dim…
    assert_eq!(split.identity(Side::Source, 0), None);
    // …support_below's question: the stream lands on both drivers.
    assert_eq!(split.support(Side::Source, 0), vec![0, 1]);
    assert_eq!(split.support(Side::Output, 1), vec![0]);

    // A permuted dim keeps its loop variable, from either side.
    let transpose = read_map(vec![term(1, 1), term(1, 0)], vec![3, 2], vec![2, 3]);
    assert_eq!(transpose.identity(Side::Source, 0), Some(1));
    assert_eq!(transpose.identity(Side::Output, 0), Some(1));

    // A broadcast output dim supports nothing — it is a summed fiber.
    let broadcast = read_map(vec![term(1, 1)], vec![4, 3], vec![3]);
    assert_eq!(broadcast.support(Side::Output, 0), Vec::<usize>::new());
    assert_eq!(broadcast.identity(Side::Output, 1), Some(0));
}

#[test]
fn adjoint_verdicts_match_the_enumerated_preimage_structure() {
    let zoo: Vec<(&str, Correspondence, Adjoint)> = vec![
        (
            "permute",
            read_map(vec![term(1, 1), term(1, 0)], vec![3, 2], vec![2, 3]),
            Adjoint::Movement,
        ),
        (
            "flip",
            read_map(
                vec![AffineDim {
                    terms: vec![(-1, 0)],
                    offset: 4,
                }],
                vec![5],
                vec![5],
            ),
            Adjoint::Movement,
        ),
        (
            "pad", // out[x] = src[x-2] where valid: every source point read once
            read_map(
                vec![AffineDim {
                    terms: vec![(1, 0)],
                    offset: -2,
                }],
                vec![7],
                vec![3],
            ),
            Adjoint::Movement,
        ),
        (
            "slice", // reads {2,3} of 5: injective, not surjective → zero-fill
            read_map(
                vec![AffineDim {
                    terms: vec![(1, 0)],
                    offset: 2,
                }],
                vec![2],
                vec![5],
            ),
            Adjoint::Padded,
        ),
        (
            "strided slice",
            read_map(vec![term(2, 0)], vec![3], vec![6]),
            Adjoint::Padded,
        ),
        ("split", split_6_into_2x3(), Adjoint::Movement),
        ("merge (write)", merge_2x3_into_6(), Adjoint::Movement),
        (
            "broadcast",
            read_map(vec![term(1, 1)], vec![4, 3], vec![3]),
            Adjoint::SumThenMove { summed: vec![0] },
        ),
        (
            "overlapping window", // src0 = out0 + out1: shared support, the contraction is the math
            read_map(
                vec![AffineDim {
                    terms: vec![(1, 0), (1, 1)],
                    offset: 0,
                }],
                vec![3, 2],
                vec![4],
            ),
            Adjoint::Dense,
        ),
    ];
    for (name, map, expected) in zoo {
        assert_eq!(map.adjoint_class(), expected, "symbolic verdict for {name}");
        assert_eq!(enumerated_verdict(&map), expected, "enumerated structure for {name}");
    }
}

#[test]
fn off_the_zoo_the_symbolic_verdict_widens_to_dense() {
    // src0 = out0 + 3·out1 over out=[3,2]: a mixed radix whose span (6)
    // overshoots the source (3). The symbolic classifier, reading
    // coefficients alone, says Dense; the actual relation is a movement —
    // out1=1 is entirely out of range, out1=0 reads each source point
    // once. Where the two diverge, Dense is the safe side: it costs
    // work, never correctness. The zoo pins where they must agree; this
    // pins the direction of disagreement.
    let overshoot = read_map(
        vec![AffineDim {
            terms: vec![(1, 0), (3, 1)],
            offset: 0,
        }],
        vec![3, 2],
        vec![3],
    );
    assert_eq!(overshoot.adjoint_class(), Adjoint::Dense);
    assert_eq!(enumerated_verdict(&overshoot), Adjoint::Movement);
}

#[test]
fn a_frame_is_stated_not_guessed() {
    // The only way to build one is to say where each dimension came from.
    let stated = Frame {
        dims: vec![Some(1), Some(0), None],
    };
    assert_eq!(stated.dims.len(), 3);
    assert_eq!(stated.dims[2], None); // a minted dim is declared, not matched by extent
}
