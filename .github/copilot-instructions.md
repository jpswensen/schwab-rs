# schwab-rs review instructions

Review this repository as a Rust client library for Charles Schwab APIs. The public API should be predictable for client applications: typed request parameters, typed response structs, async methods, and explicit errors instead of process-level behavior.

Focus on bugs, security, data loss, broken API contracts, and project conventions. Do not nitpick formatting or style that `rustfmt` or `clippy` already handles.

## Project invariants

- Crate name is `schwab`, published from `Cargo.toml` at the repository root.
- Public modules live under `src/` with `client`, `config`, `error`, `models`, `market_data_api`, `trader_api`, `auth`, `options`, `order_builder`, and `query`.
- Library code must not call `process::exit`, write user-facing output, read hidden config files, or inspect environment variables unless a public option documents that behavior.
- Public async methods take `&self` and return `schwab::Result<T>`.
- Request paths, query parameters, serde field attributes, and response structs must match the Schwab API specification.
- Preserve typed errors from `src/error.rs` (via `thiserror`) and wrap errors with enough context for callers to handle them.
- Keep public items documented with useful doc comments.

## Security and account safety

- Flag credential, bearer token, account hash, or secret exposure in logs, errors, tests, docs, or generated output.
- Verify account-scoped trader calls keep account hash values in path parameters only when required by the API.
- Order placement, replacement, cancellation, and preview methods must not invent safety shortcuts or silently mutate payloads.
- Avoid silent fallback behavior around HTTP status handling, body decoding, or token application. Return clear errors instead.

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
- Release automation uses `release-plz` because this repository is a library.
- Clippy allow attributes require a specific lint name and an explanation.
- US English spelling is enforced.
