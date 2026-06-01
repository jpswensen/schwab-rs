# schwab-rs review instructions

Review this repository as a Rust client library plus the `schwab-agent` structured JSON CLI for Charles Schwab APIs. The public library API should be predictable for client applications: typed request parameters, typed response structs, async methods, and explicit errors instead of process-level behavior. The CLI should stay machine-readable, safe around trading actions, and isolated under `src/bin/schwab-agent/`.

Focus on bugs, security, data loss, broken API contracts, and project conventions. Do not nitpick formatting or style that `rustfmt` or `clippy` already handles.

## Project invariants

- Crate name is `schwab`, published from `Cargo.toml` at the repository root.
- Public modules live under `src/` with `client`, `config`, `error`, `models`, `market_data_api`, `trader_api`, `auth`, `options`, `order_builder`, and `query`.
- Binary code for `schwab-agent` lives under `src/bin/schwab-agent/` and may read documented CLI config or environment variables. Do not move CLI behavior into public library modules.
- Library code must not call `process::exit`, write user-facing output, read hidden config files, or inspect environment variables unless a public option documents that behavior.
- Public async methods take `&self` and return `schwab::Result<T>`.
- Request paths, query parameters, serde field attributes, and response structs must match the Schwab API specification.
- Preserve typed errors from `src/error.rs` (via `thiserror`) and wrap errors with enough context for callers to handle them.
- Keep public items documented with useful doc comments.

## Security and account safety

- Flag credential, bearer token, account hash, or secret exposure in logs, errors, tests, docs, or generated output.
- Verify account-scoped trader calls keep account hash values in path parameters only when required by the API.
- Order placement, replacement, cancellation, and preview methods must not invent safety shortcuts or silently mutate payloads.
- CLI mutable order paths must keep the `i-also-like-to-live-dangerously` config guard, resolve account selectors to canonical Schwab account hashes, and verify post-action state.
- Prefer trading safety over avoiding small read-only calls. Extra account or order discovery is acceptable when it prevents ambiguous mutable behavior.
- Avoid silent fallback behavior around HTTP status handling, body decoding, or token application. Return clear errors instead.

## schwab-agent CLI expectations

- Command output is raw JSON data payloads; errors use `ErrorBody` with stable `code`, `message`, `category`, `retryable`, and `hint` fields.
- Compact market outputs remain row-based by default. `market quote --all-fields` and `market history --all-fields` are legacy detailed-output escape hatches.
- Compact quote rows must preserve per-symbol errors instead of dropping failed symbols.
- Validate requested quote/history fields and other cheap user input before auth or API calls when possible.
- Position output must include actionable identifiers such as symbols, CUSIPs, or instrument IDs when Schwab provides them.
- Tests that mutate environment variables must restore them with panic-safe guards.

## Testing expectations

- Use standard `assert!`, `assert_eq!`, `assert_ne!` macros.
- Mock HTTP with `mockito` and validate expected request method, path, query, headers, and body.
- Prefer descriptive test function names that explain the scenario being tested.
- Keep generated response data inline unless fixtures clearly improve readability.
- Verify request methods, paths, query parameters, headers, and decoded response fields.

## Build and lint expectations

- CI runs `make check`, which runs `cargo fmt --check`, `cargo clippy` (with and without the `decimal` feature), `cargo test` (with and without the `decimal` feature), and `cargo doc`.
- CI also runs pinned `cargo-llvm-cov` coverage with a 90% line threshold, pinned `cargo-machete`, no install-action fallback, and an optional Codecov upload gated by a non-secret presence flag with the token scoped only to the upload step.
- Coverage and patch-coverage must not enable `test_online`; generated `lcov.info` is ignored and should not be committed.
- Release automation is split: `release-plz` owns release PRs and tags, while cargo-dist owns GitHub Releases, binary artifacts, and crates.io publishing through Trusted Publishing.
- Clippy allow attributes require a specific lint name and an explanation.
- US English spelling is enforced.
