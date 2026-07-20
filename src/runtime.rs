//! Stateful execution: persistent buffers, committed in-place updates, and
//! the decode loop — the runtime half of "mutable state / KV-cache".
//!
//! The IR stays pure: there is no `Store` node and no aliasing inside a
//! graph. State lives *here*, at the runtime boundary. A [`Session`] owns
//! named buffers (weights, caches, activations); one [`Session::step`]
//! executes a — possibly multi-output — [`Schedule`] against them, exposes
//! every output as a buffer, and then **commits** requested outputs over
//! existing buffers.
//!
//! Commit-after-execute is the write-after-read discipline (tinygrad's
//! `AFTER(view, STORE(...))`) made structural instead of scheduled: every
//! read a step performs completes before any write lands, because the writes
//! are buffer swaps applied only once the whole schedule has run. A commit
//! moves the new tensor over the old name — O(1), no copy, and no aliasing
//! proof is ever needed, which is why the update kernels themselves can stay
//! pure functions (`updated[t,d] = where(t == pos, new_k[d], cache[t,d])`).
//!
//! The KV-cache decode loop is exactly this shape: caches are session
//! buffers of a fixed maximum length; the step's graph reads the cache,
//! writes the current position's K/V row (a `where` on a computed position
//! mask — no new IR), attends over the updated cache with positions beyond
//! `pos` masked out, and produces logits; the runtime then commits the
//! updated caches for the next step. `tests/decode.rs` proves N incremental
//! steps equal one full-attention prefill — on the interpreter and on
//! compiled Rust with buffers persisting across a real host loop.
//!
//! A caveat for long-lived processes: COMPILATION (not execution) interns
//! temporary buffer names with `Box::leak` (the `leak` helpers in
//! `partition`/`compile`/`rustgen`), so a process that keeps compiling new
//! schedules leaks a few strings per compilation, without bound. A session
//! that compiles its schedules once and then steps forever — the decode
//! loop — does not grow. Interning into a session-owned arena is the fix if
//! recompile-per-request ever becomes a real shape here.

use crate::interp::{Env, Value};
use crate::partition::Schedule;

/// A stateful execution session: named persistent buffers plus step
/// execution with commit-after-execute updates.
#[derive(Default)]
pub struct Session {
    /// The persistent buffers, by name — plus, after a step, that step's
    /// intermediates (`t0`, `t1`, …), which are scratch: a later schedule may
    /// overwrite them.
    pub env: Env,
}

impl Session {
    pub fn new() -> Session {
        Session::default()
    }

    /// Create or replace a buffer (weights at load time, caches at init).
    pub fn bind(&mut self, name: &'static str, t: Value) {
        self.env.insert(name, t);
    }

    /// A 0-dimensional buffer — how a step is fed its position.
    pub fn bind_scalar(&mut self, name: &'static str, v: f64) {
        self.env.insert(name, Value::scalar(v));
    }

    pub fn get(&self, name: &str) -> &Value {
        self.env
            .get(name)
            .unwrap_or_else(|| panic!("session has no buffer `{name}`"))
    }

    /// Execute one step, then commit each `(produced, dest)` pair by moving
    /// the produced tensor over the destination buffer.
    ///
    /// Two checks keep the discipline honest: a schedule output may not
    /// share a name with a buffer the schedule *reads* (the write-after-read
    /// hazard — an in-place result must go through a commit, never land
    /// mid-schedule where a later stage could read the new value as the
    /// old), and a commit over an existing buffer must preserve its shape (a
    /// cache row write may not silently resize the cache).
    pub fn step(&mut self, sched: &Schedule, commits: &[(&'static str, &'static str)]) {
        let reads = sched.reads();
        for out in &sched.outputs {
            assert!(
                !reads.iter().any(|r| r == out),
                "schedule output `{out}` would overwrite a buffer the schedule reads; \
                 name step outputs freshly and commit them instead"
            );
            // a previous step's uncommitted result under this name is stale
            self.env.remove(out.as_str());
        }
        sched.execute_env(&mut self.env);
        for (from, dest) in commits {
            let produced = self
                .env
                .remove(from)
                .unwrap_or_else(|| panic!("step produced no buffer `{from}` to commit"));
            if let Some(old) = self.env.get(dest) {
                assert_eq!(
                    old.shape, produced.shape,
                    "commit `{from}` → `{dest}` would change the buffer shape"
                );
            }
            self.env.insert(dest, produced);
        }
    }
}
