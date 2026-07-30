//! Structural correctness: the shape of the code, held to the stated design.
//!
//! Execution oracles prove what the compiler *computes*; nothing so far
//! proved what the code *is* — which module may reach which, which layer may
//! name what, how much acknowledged debt exists and where. Those are design
//! facts, stated in `number_systems_and_representations.md`, CLAUDE.md's
//! standing rules, and `ir.rs`'s own doctrine — and a design fact that no
//! test holds is a comment.
//!
//! Everything here reads `src/` as text, comments and string literals
//! stripped, test modules excluded (acceptance tests may name anything).
//! Three kinds of clause:
//!
//! 1. **Direction** — the expression layer (pure math: `ir`, `derive`,
//!    `simplify`, `grad`, `interp`, `numeric`, …) never reaches the program
//!    layer (`partition`, `plan`, `cost`, emitters, `compile`, …). The
//!    reverse dependency is the whole design; this pins it.
//! 2. **Vocabulary** — no core rule names a frontend operation. The standing
//!    rule: core gets general math laws only; a rule naming `argmax` or
//!    `softmax` is the red flag. Enforced at zero, because it IS zero.
//! 3. **The debt ledger** — the acknowledged violations, pinned at their
//!    exact counts with the milestone that retires each. Equality, not an
//!    upper bound: growth is a regression, and shrinkage is milestone
//!    progress that must update the ledger in the same commit. The ledger
//!    is the single honest list of where the design and the code disagree.

use std::fs;

/// Source of `src/{name}.rs` with line comments, (nested) block comments and
/// string literals removed, truncated at the first `#[cfg(test)]` — test
/// modules sit at file ends by convention and may name anything.
///
/// Limitations, accepted: raw strings and `'"'` char literals would confuse
/// it; neither occurs in `src/` today, and if one ever does, the failure is
/// a miscount here, not silence.
fn code_of(name: &str) -> String {
    let path = format!("{}/src/{name}.rs", env!("CARGO_MANIFEST_DIR"));
    let source = fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path}: {error}"));
    let source = match source.find("#[cfg(test)]") {
        Some(cut) => &source[..cut],
        None => &source[..],
    };
    // Byte-wise on purpose: comments contain multi-byte glyphs (ℕ, 𝔹, Γ),
    // and every delimiter tested for is ASCII, which no UTF-8 continuation
    // byte can collide with — so byte processing is boundary-safe.
    let bytes = source.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    let mut depth = 0usize;
    while i < bytes.len() {
        let pair = (bytes[i], bytes.get(i + 1).copied());
        if depth > 0 {
            match pair {
                (b'*', Some(b'/')) => {
                    depth -= 1;
                    i += 2;
                }
                (b'/', Some(b'*')) => {
                    depth += 1;
                    i += 2;
                }
                _ => i += 1,
            }
            continue;
        }
        match pair {
            (b'/', Some(b'/')) => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            (b'/', Some(b'*')) => {
                depth = 1;
                i += 2;
            }
            (b'"', _) => {
                i += 1;
                while i < bytes.len() && bytes[i] != b'"' {
                    i += if bytes[i] == b'\\' { 2 } else { 1 };
                }
                i += 1;
            }
            _ => {
                out.push(bytes[i]);
                i += 1;
            }
        }
    }
    String::from_utf8(out).expect("stripping only removes whole ASCII-delimited spans")
}

/// Occurrences of `needle` in `code` not preceded by an identifier
/// character — `softmax` matches in `softmax(x)` and `fn softmax`, not in
/// `presoftmax`.
fn named(code: &str, needle: &str) -> usize {
    code.match_indices(needle)
        .filter(|(at, _)| {
            *at == 0
                || !code[..*at]
                    .chars()
                    .next_back()
                    .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
        })
        .count()
}

/// The pure half: math, analysis, and the oracle. No device, no schedule,
/// no buffer widths — a fact about one of these files is a fact about the
/// mathematics.
const EXPRESSION_LAYER: &[&str] = &[
    "scalar", "ir", "tensor", "nn", "interp", "analyze", "derive", "simplify", "grad", "numeric", "verify",
];

/// The deciding half: everything parameterised by a device or owning an
/// execution order.
const PROGRAM_LAYER: &[&str] = &[
    "cost",
    "plan",
    "partition",
    "codegen",
    "emit_metal",
    "rustgen",
    "metal",
    "compile",
    "graph",
    "runtime",
];

#[test]
fn the_expression_layer_never_reaches_the_program_layer() {
    for file in EXPRESSION_LAYER {
        let code = code_of(file);
        for module in PROGRAM_LAYER {
            let reference = format!("crate::{module}");
            assert_eq!(
                code.matches(reference.as_str()).count(),
                0,
                "{file}.rs references {reference}: the expression layer is pure \
                 mathematics and must not know the program layer exists \
                 (number_systems_and_representations.md §2, §8)"
            );
        }
    }
}

/// The standing rule, mechanically: core gets general math laws only. The
/// IR's frontend section (`ir.rs`), `tensor.rs` and `nn.rs` *define* these
/// operations as compositions — everyone else must treat them as the
/// anonymous graphs they lower to. `derive.rs`'s own boast is the spirit:
/// the FlashAttention accumulator "is never written down anywhere in this
/// repository; it is constructed by this fold."
#[test]
fn no_core_rule_names_a_frontend_operation() {
    const CORE: &[&str] = &[
        "scalar",
        "interp",
        "analyze",
        "derive",
        "simplify",
        "grad",
        "numeric",
        "verify",
        "cost",
        "plan",
        "partition",
        "codegen",
        "emit_metal",
        "rustgen",
        "metal",
        "compile",
        "graph",
        "runtime",
    ];
    const FRONTEND_OPERATIONS: &[&str] = &[
        "argmax",
        "topk",
        "one_hot",
        "softmax",
        "attention",
        "rope",
        "rms_norm",
        "silu",
        "flash",
        "gelu",
        "embedding",
        "causal",
        "cumsum",
    ];
    for file in CORE {
        let code = code_of(file);
        for op in FRONTEND_OPERATIONS {
            assert_eq!(
                named(&code, op),
                0,
                "{file}.rs names `{op}` outside comments and tests: a core rule \
                 naming a frontend operation is the red flag — express it as a \
                 general law or decline"
            );
        }
    }
}

/// `ir.rs`'s doctrine, held: *"there is no second graph IR."* Stage bodies
/// reuse the one `Node`; annotations are derived, memoised side tables; the
/// Γ design (M12) adds a bindings table, not a second term type.
#[test]
fn there_is_exactly_one_expression_ir() {
    let mut definitions = 0;
    for file in EXPRESSION_LAYER.iter().chain(PROGRAM_LAYER) {
        definitions += code_of(file).matches("enum Node").count();
    }
    assert_eq!(definitions, 1, "a second node enum is a second IR");
}

#[test]
fn the_structural_debt_ledger_is_exact() {
    // (file, needle, count, retired by). Equality on purpose: if reality
    // changed, this ledger changes in the same commit — growth is a
    // regression caught, shrinkage is milestone progress recorded.
    const LEDGER: &[(&str, &str, usize, &str)] = &[
        // What remains of the boundary policy in partition: the mint's
        // preferred width, produced-buffer pricing, and the «div» probe.
        // The policy itself retires with M14's per-value declarations.
        ("partition", "dev.storage", 3, "M14"),
        // Allocation width, output/readback dtype stamps, nanscan and
        // bandwidth accounting, all from one global. M12 moves them to Γ.
        ("compile", "program.storage", 10, "M12"),
        // State buffers allocated and wrapped at the device global, and the
        // declared-dtype equality check (plus its error message). M12 makes
        // the check "can this dtype carry that expression's system".
        ("graph", "device.storage()", 4, "M12"),
        // The one-knob policy, living on device types until M14 retires it
        // for per-value declarations (and renames DeviceSpecs to
        // DeviceSpecs, leaving hardware facts only).
        ("cost", "with_storage", 1, "M14"),
        ("metal", "with_storage", 1, "M14"),
        // RETIRED by M12b, kept at zero as the doctrine it became: the
        // expression IR mentions no storage, anywhere. An Input is a name
        // and a shape; identity is structure; Γ owns every width.
        ("ir", "dtype", 0, "retired — keep it so"),
    ];
    for (file, needle, expected, retires) in LEDGER {
        let count = code_of(file).matches(needle).count();
        assert_eq!(
            count, *expected,
            "{file}.rs contains {count} `{needle}` (ledger says {expected}, retired by {retires}): \
             if this is milestone progress, update the ledger in this commit; \
             if it is growth, the design forbids it"
        );
    }
}
