# rust-serverless-data-engineering

Companion repo for the Coursera course "Rust Serverless Data Engineering" — part of the
`paiml/rust-de-specialization` umbrella. The headliner is `recho/`, a `cargo lambda`-deployable
echo handler that demonstrates idempotent, contract-checked serverless data pipelines in Rust,
plus side-by-side `scripting-demos/` ports (Python, JavaScript, Ruby) so students can see the
same shape across languages.

## Contract-first workflow — `pmat comply` is the gate

This repo follows a **contract-first** discipline. Every change to `recho/src/lib.rs` lands by:

1. Naming or updating the **provable contract** below (one paragraph, falsifiable).
2. Adding/adjusting the tests in `mod tests` so every clause is exercised.
3. Running `pmat comply check` locally — must report `Status: COMPLIANT`.
4. Pushing — CI re-runs `cargo llvm-cov` with `--fail-under-{lines,functions,regions} 100`.

If a change cannot be expressed as a delta to the contract, it does not belong in this repo.
Run `pmat comply` before completing any task to verify compliance.

## Provable contract — what this repo guarantees

**Provable contract:** for every payload `e` accepted by `function_handler(e, rng)`, exactly one
of the following holds and is observable from the return value alone:

1. `e.payload.message` is `Some(m)` and `m.trim().to_lowercase() == "echo"` →
   the result is `Ok(Response { message })` with `message ∈ {"🌑 dark ECHO", "💡 light ECHO"}`,
   chosen deterministically from `rng` (no other source of nondeterminism).
2. `e.payload.message` is `Some(m)` and the normalized form is not `"echo"` →
   the result is `Ok(Response { message: format!("You said: {}. Say 'echo' to play the tunnel game!", normalized) })`.
3. `e.payload.message` is `None` →
   the result is `Err(_)` whose `Display` contains the literal substring `Missing 'message' field`.

The CI coverage gate (100% lines/regions/functions on `recho/src/lib.rs`) discharges every branch
of this contract on every push. `main.rs` is the only file outside the gate (runtime entry, no logic).

This contract is the agent-facing entry point for the repo: every change to `recho/src/lib.rs` must
preserve all three clauses or update them deliberately along with the tests in `mod tests`.

## Hard rules

- **CI is the source of truth.** The `ci` workflow gates on fmt + clippy + tests + 100%
  line/region/function coverage on `recho/src/lib.rs`. `main.rs` is excluded (runtime entry
  only — no logic). Do not relax the gate; add tests instead.
- **All Rust logic lives in `recho/src/lib.rs`.** `main.rs` is a 9-line `tokio::main` shim that
  wires `recho::function_handler` into `lambda_runtime::run`. Anything testable goes in `lib.rs`.
- **Inject randomness.** `handle_echo<R: Rng>` and `function_handler<R: Rng>` take `&mut R` so
  tests can drive `rand::rngs::mock::StepRng` deterministically. Never call `thread_rng()` from
  inside a function whose branches must be tested — push the RNG to the caller.
- **`scripting-demos/*` are pedagogy, not production.** They exist to compare shapes across
  languages. Don't add new logic there — add it to `recho/`.

## Code search policy

**MANDATORY: Use `pmat query` for ALL code search. NEVER use grep, rg, find, fd.**

| Task | Command |
|------|---------|
| Find by intent | `pmat query "echo handler" --limit 10` |
| Find with faults | `pmat query "unwrap" --faults --exclude-tests` |
| Literal search | `pmat query --literal "lambda_runtime" --limit 10` |
| Regex search | `pmat query --regex "fn\s+test_\w+" --limit 10` |
| Coverage gaps | `pmat query --coverage-gaps --exclude-tests` |
| Cross-repo | `pmat query "lambda" --include-project ../rust-de-specialization` |

Read code via `pmat query "<concept>" --include-source --limit 1`, not raw file reads, except
for non-code files (TOML, YAML, Markdown, JSON).

## Workflow

```bash
# Build + test + 100% coverage gate (matches CI exactly)
cd recho
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo llvm-cov --all-features \
  --ignore-filename-regex 'main\.rs' \
  --fail-under-lines 100 \
  --fail-under-functions 100 \
  --fail-under-regions 100 \
  --summary-only

# Deploy via cargo-lambda
cargo install cargo-lambda  # one-time
cargo lambda build --release
cargo lambda deploy

# Invoke (after deploy)
../tunnel.sh "echo"
```

## Companion to

- Specialization umbrella: <https://github.com/paiml/rust-de-specialization>
- Course catalog (course-studio): <https://github.com/paiml/course-studio>
