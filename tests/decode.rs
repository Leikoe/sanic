//! M6 — autoregressive decode with a persistent KV cache, proven equal to
//! full-attention prefill.
//!
//! One decode step is a THREE-output schedule ([`partition_many`]): the K and
//! V cache updates (`where(t == pos, new, cache)` on a computed position
//! mask — no new IR) and the logits (attention over the *updated* cache with
//! positions beyond `pos` masked out). The runtime ([`Session`]) persists the
//! caches across steps and commits each update after the whole step has
//! executed — the write-after-read discipline.
//!
//! The theorem, numerically: running T incremental steps, each attending
//! only over the cache, produces exactly the logits of one full causal
//! prefill over all T positions. Proven on the interpreter session AND on
//! compiled Rust, where the caches are `Vec`s living across a real host
//! loop and `run` returns the (ck, cv, logits) triple each step.

use std::path::PathBuf;
use std::process::Command;

use sanic::cost::DeviceProfile;
use sanic::interp::{Env, Value, eval};
use sanic::ir::*;
use sanic::partition::{Schedule, partition_many};
use sanic::runtime::Session;
use sanic::rustgen::emit_schedule;

/// The one tolerance policy (`verify::rel_tolerance`) at this file's chain
/// length: a decode step chains a handful of folds whose summed extents stay
/// under 64 in these toys.
const CHAIN_TERMS: usize = 64;

struct Lcg(u64);
impl Lcg {
    fn f(&mut self) -> f64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        ((x.wrapping_mul(0x2545F4914F6CDD1D) >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0
    }
}
fn rand_tensor(axes: &[Axis], rng: &mut Lcg) -> Value {
    let shape = axes.iter().map(|axis| axis.extent()).collect::<Vec<_>>();
    Value::from_shape_fn(&shape, |_| rng.f())
}

fn linear_vector(x: NodeRef, weight: NodeRef) -> NodeRef {
    reduce(map(MapOp::Mul, vec![weight, x]), 1usize, Monoid::Add)
}

/// The single-head attention LM decode step: cache updates + logits.
/// Axes: t = cache length (max sequence), dm = model, dk/dv = head, v = vocab.
fn decode_step(t: Axis, dm: Axis, dk: Axis, dv: Axis, v: Axis) -> Schedule {
    let x = input("x", [dm], Dtype::F32); // current token's embedding
    let pos = input("pos", [], Dtype::F32); // current position, as data

    let new_k = linear_vector(x.clone(), input("Wk", [dk, dm], Dtype::F32)); // [dk]
    let new_v = linear_vector(x.clone(), input("Wv", [dv, dm], Dtype::F32)); // [dv]
    let q = linear_vector(x, input("Wq", [dk, dm], Dtype::F32)); // [dk]

    // cache row writes: updated[t,·] = where(t == pos, new, cache[t,·])
    let cache_k = input("cache_k", [t, dk], Dtype::F32);
    let ck = map(
        MapOp::Where,
        vec![
            one_hot_like(cache_k.clone(), 0usize, pos.clone()),
            new_k,
            cache_k,
        ],
    ); // [t, dk]
    let cache_v = input("cache_v", [t, dv], Dtype::F32);
    let cv = map(
        MapOp::Where,
        vec![
            one_hot_like(cache_v.clone(), 0usize, pos.clone()),
            new_v,
            cache_v,
        ],
    ); // [t, dv]

    // attention over the updated cache; positions beyond `pos` masked out
    let scale = konst(1.0 / (dk.extent() as f64).sqrt());
    let scores = map(
        MapOp::Mul,
        vec![
            reduce(map(MapOp::Mul, vec![ck.clone(), q]), 1usize, Monoid::Add),
            scale,
        ],
    ); // [t]
    let future = map(MapOp::Lt, vec![pos, iota(t)]); // t > pos
    let masked = map(
        MapOp::Add,
        vec![
            scores,
            map(MapOp::Where, vec![future, konst(-1e30), konst(0.0)]),
        ],
    );
    let att = softmax(masked, 0usize);
    let out = reduce(
        map(MapOp::Mul, vec![unsqueeze(att, 1usize), cv.clone()]),
        0usize,
        Monoid::Add,
    ); // [dv]
    let logits = linear_vector(out, input("Wl", [v, dv], Dtype::F32)); // [v]

    partition_many(
        &[(ck, "ck_new"), (cv, "cv_new"), (logits, "logits")],
        &DeviceProfile::toy(),
    )
}

/// The prefill reference: full causal attention over all T positions at once,
/// evaluated by the oracle. Row `s` of its logits is what decode step `s`
/// must produce.
fn prefill_logits(s: Axis, t2: Axis, dm: Axis, dk: Axis, dv: Axis, v: Axis, env: &Env) -> Value {
    let x = input("X", [s, dm], Dtype::F32);
    let xt = rename(x.clone(), 0usize, t2);
    let q = matmul(
        x,
        transpose(input("Wq", [dk, dm], Dtype::F32), 0usize, 1usize),
    ); // [s, dk]
    let k = matmul(
        xt.clone(),
        transpose(input("Wk", [dk, dm], Dtype::F32), 0usize, 1usize),
    ); // [t2, dk]
    let vv = matmul(
        xt,
        transpose(input("Wv", [dv, dm], Dtype::F32), 0usize, 1usize),
    ); // [t2, dv]
    let scale = konst(1.0 / (dk.extent() as f64).sqrt());
    let scores = map(
        MapOp::Mul,
        vec![matmul(q, transpose(k, 0usize, 1usize)), scale],
    );
    let masked = map(
        MapOp::Add,
        vec![scores.clone(), causal_mask_like(scores, 0usize, 1usize)],
    );
    let att = softmax(masked, 1usize);
    let out = matmul(att, vv); // [s, dv]
    let logits = matmul(
        out,
        transpose(input("Wl", [v, dv], Dtype::F32), 0usize, 1usize),
    ); // [s, v]
    eval(&logits, env)
}

// ── the theorem, on the interpreter runtime ──────────────────────────────────
#[test]
fn incremental_decode_equals_prefill() {
    let steps = 6usize;
    let (t, s, t2, dm, dk, dv, v) = (
        axis("t", steps),
        axis("s", steps),
        axis("t2", steps),
        axis("dm", 8),
        axis("dk", 5),
        axis("dv", 6),
        axis("v", 10),
    );
    let mut rng = Lcg(0xDEC0DE);
    let wq = rand_tensor(&[dk, dm], &mut rng);
    let wk = rand_tensor(&[dk, dm], &mut rng);
    let wv = rand_tensor(&[dv, dm], &mut rng);
    let wl = rand_tensor(&[v, dv], &mut rng);
    let xs = rand_tensor(&[s, dm], &mut rng); // all T token embeddings

    // the reference: one full causal prefill
    let prefill_env: Env = [
        ("X", xs.clone()),
        ("Wq", wq.clone()),
        ("Wk", wk.clone()),
        ("Wv", wv.clone()),
        ("Wl", wl.clone()),
    ]
    .into_iter()
    .collect();
    let reference = prefill_logits(s, t2, dm, dk, dv, v, &prefill_env);

    // the decode session: persistent caches, one step per position
    let sched = decode_step(t, dm, dk, dv, v);
    assert!(
        sched.stages.len() >= 5,
        "expected a multi-kernel decode step:\n{}",
        sched.render()
    );

    let mut sess = Session::new();
    sess.bind("Wq", wq);
    sess.bind("Wk", wk);
    sess.bind("Wv", wv);
    sess.bind("Wl", wl);
    sess.bind(
        "cache_k",
        Value::from_shape_fn(&[t.extent(), dk.extent()], |_| 0.0),
    );
    sess.bind(
        "cache_v",
        Value::from_shape_fn(&[t.extent(), dv.extent()], |_| 0.0),
    );

    for p in 0..steps {
        let row = Value::from_shape_fn(&[dm.extent()], |c| xs.at_index(&[p, c[0]]));
        sess.bind("x", row);
        sess.bind_scalar("pos", p as f64);
        sess.step(&sched, &[("ck_new", "cache_k"), ("cv_new", "cache_v")]);

        let logits = sess.get("logits");
        for vi in 0..v.extent() {
            let got = logits.at_index(&[vi]);
            let want = reference.at_index(&[p, vi]);
            let tol = sanic::verify::rel_tolerance(Dtype::F64, CHAIN_TERMS)
                * (1.0 + got.abs().max(want.abs()));
            assert!(
                (got - want).abs() <= tol,
                "step {p}, vocab {vi}: decode {got} vs prefill {want}"
            );
        }
    }

    // and the caches now hold exactly the prefill K/V
    let k_ref = eval(
        &matmul(
            rename(input("X", [s, dm], Dtype::F32), 0usize, t),
            transpose(input("Wk", [dk, dm], Dtype::F32), 0usize, 1usize),
        ),
        &prefill_env,
    );
    let ck = sess.get("cache_k");
    for ti in 0..steps {
        for ki in 0..dk.extent() {
            let (a, b) = (ck.at_index(&[ti, ki]), k_ref.at_index(&[ti, ki]));
            assert!(
                (a - b).abs()
                    <= sanic::verify::rel_tolerance(Dtype::F64, CHAIN_TERMS) * (1.0 + a.abs()),
                "cache_k[{ti},{ki}]"
            );
        }
    }
}

// ── the same loop, compiled: caches persist across a real host loop ──────────
#[test]
fn incremental_decode_compiles_and_equals_prefill() {
    let steps = 5usize;
    let (t, s, t2, dm, dk, dv, v) = (
        axis("t", steps),
        axis("s", steps),
        axis("t2", steps),
        axis("dm", 6),
        axis("dk", 4),
        axis("dv", 5),
        axis("v", 8),
    );
    let mut rng = Lcg(0xDEC0DED);
    let wq = rand_tensor(&[dk, dm], &mut rng);
    let wk = rand_tensor(&[dk, dm], &mut rng);
    let wv = rand_tensor(&[dv, dm], &mut rng);
    let wl = rand_tensor(&[v, dv], &mut rng);
    let xs = rand_tensor(&[s, dm], &mut rng);

    let prefill_env: Env = [
        ("X", xs.clone()),
        ("Wq", wq.clone()),
        ("Wk", wk.clone()),
        ("Wv", wv.clone()),
        ("Wl", wl.clone()),
    ]
    .into_iter()
    .collect();
    let reference = prefill_logits(s, t2, dm, dk, dv, v, &prefill_env);
    let expected: Vec<f64> = (0..steps)
        .flat_map(|p| {
            (0..v.extent())
                .map(|vi| reference.at_index(&[p, vi]))
                .collect::<Vec<f64>>()
        })
        .collect();

    let sched = decode_step(t, dm, dk, dv, v);
    let program = emit_schedule(&sched);
    assert_eq!(
        program
            .outputs
            .iter()
            .map(|(n, _)| n.as_str())
            .collect::<Vec<_>>(),
        vec!["ck_new", "cv_new", "logits"],
        "run must return the cache updates and the logits"
    );

    // weights (baked), caches (mutable, persistent), x/pos (per step)
    let bake = |data: &[f64]| {
        data.iter()
            .map(|v| format!("{v:?}f64"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let mut main = String::from("\nfn main() {\n");
    for (name, tensor) in [("Wq", &wq), ("Wk", &wk), ("Wv", &wv), ("Wl", &wl)] {
        let axes: Vec<AxisRef> = program
            .inputs
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, a)| a.clone())
            .unwrap_or_else(|| tensor.axes.clone());
        main.push_str(&format!("    let b_{name}: Vec<f64> = vec![{}];\n", {
            assert_eq!(
                tensor.shape,
                axes.iter().map(|axis| axis.extent()).collect::<Vec<_>>(),
                "input {name} shape"
            );
            bake(&tensor.data)
        }));
    }
    main.push_str(&format!(
        "    let xs: Vec<f64> = vec![{}];\n",
        bake(&xs.data)
    ));
    main.push_str(&format!(
        "    let expected: Vec<f64> = vec![{}];\n",
        bake(&expected)
    ));
    main.push_str(&format!(
        "    let mut b_cache_k: Vec<f64> = vec![0.0; {}];\n",
        steps * dk.extent()
    ));
    main.push_str(&format!(
        "    let mut b_cache_v: Vec<f64> = vec![0.0; {}];\n",
        steps * dv.extent()
    ));
    main.push_str("    let mut got: Vec<f64> = Vec::new();\n");
    main.push_str(&format!("    for p in 0..{steps} {{\n"));
    main.push_str(&format!(
        "        let b_x: Vec<f64> = xs[p*{dm_n}..(p+1)*{dm_n}].to_vec();\n",
        dm_n = dm.extent()
    ));
    main.push_str("        let b_pos: Vec<f64> = vec![p as f64];\n");
    let args: Vec<String> = program
        .inputs
        .iter()
        .map(|(n, _)| format!("&b_{n}[..]"))
        .collect();
    main.push_str(&format!(
        "        let (ck, cv, lg) = run({});\n",
        args.join(", ")
    ));
    main.push_str("        b_cache_k = ck;\n        b_cache_v = cv;\n");
    main.push_str("        got.extend(lg);\n    }\n");
    main.push_str(
        "    assert_eq!(got.len(), expected.len());\n\
         \x20   let mut maxe = 0.0f64;\n\
         \x20   for (a, b) in got.iter().zip(&expected) { maxe = maxe.max((a - b).abs()); }\n\
         \x20   if maxe < 1e-9 { println!(\"OK {maxe:e}\"); } else { eprintln!(\"MISMATCH {maxe:e}\"); std::process::exit(1); }\n\
         }\n",
    );

    let dir: PathBuf = std::env::var("CLAUDE_JOB_DIR")
        .map(|d| PathBuf::from(d).join("tmp"))
        .unwrap_or_else(|_| std::env::temp_dir());
    std::fs::create_dir_all(&dir).unwrap();
    let src = dir.join("sanic_gen_decode.rs");
    let bin = dir.join("sanic_gen_decode");
    std::fs::write(&src, format!("{}{}", program.source, main)).unwrap();

    let out = Command::new("rustc")
        .args(["-O", "--edition", "2021", "-o"])
        .arg(&bin)
        .arg(&src)
        .output()
        .expect("failed to invoke rustc");
    assert!(
        out.status.success(),
        "generated decode program failed to COMPILE:\n{}\n--- source at {} ---",
        String::from_utf8_lossy(&out.stderr),
        src.display()
    );
    let run = Command::new(&bin).output().expect("run decode binary");
    assert!(
        run.status.success() && String::from_utf8_lossy(&run.stdout).contains("OK"),
        "compiled decode diverged from prefill:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
}
