# schwab-rs agent guide

Rust client library for the Charles Schwab API plus the `schwab-agent` JSON CLI.

## Start here

Read only this section first. Open the task-specific files below only when your change touches that area.

- Keep source/docs/fixtures ASCII unless a wire-format fixture requires Unicode.
- Do not use `--all-features` for routine checks; it enables live `test_online` tests.
- Never leak bearer tokens, credentials, account hashes, balances, order IDs, or HTTP response bodies in logs, errors, Debug output, tests, or docs.
- Public library code must not read hidden config, write user-facing output, or call `process::exit`.
- When behavior changes, update only the docs that describe that behavior. There is one agent guide: this file. Do not add nested `AGENTS.md` files.

## Checks

Use the smallest check that proves your change, then run broader checks before shipping.

```bash
make check          # fmt + clippy + test + doc
make fmt-fix        # apply rustfmt
make clippy         # default, decimal, no-default, no-default+decimal
make test           # default, decimal, no-default, no-default+decimal
make doc            # rustdoc checks
make coverage       # nightly llvm-cov, 90% project gate
make patch-coverage # diff-cover patch gate against PATCH_COVERAGE_BASE
make audit          # cargo audit
make machete        # unused dependency check
```

MSRV: Rust 1.96. Edition: 2024.

## Discover by task

### Public library/API changes

Read:

- `src/lib.rs` for module exports and public surface
- `src/client.rs` for HTTP plumbing and API base handling
- `src/market_data_api.rs` or `src/trader_api.rs` for endpoint method patterns
- `src/config.rs` and `src/error.rs` for config, Result, and redaction behavior

Rules:

- Public async API methods use `&self`, return `schwab::Result<T>`, and have `#[instrument(skip_all)]`.
- `src/lib.rs` denies missing docs; public `Result` methods need `# Errors`.
- Rustdoc examples must compile with default and `decimal` features.

### Models/serde changes

Read:

- `src/models/mod.rs` for `Number` and re-exports
- `src/models/enums.rs` for enum serde conventions
- `src/models/market_data.rs` or `src/models/trader.rs` for neighboring model shape
- `src/models/streaming/` for streaming field-index models
- `tests/fixtures/` for deserialization examples

Rules:

- Use `Number` for numeric fields; never hard-code `f64` or `Decimal` in models.
- Response model fields are `Option<T>` because Schwab returns partial data.
- Enums in `src/models/enums.rs` are `#[non_exhaustive]`; serde dispatch enums that cannot handle unknown variants are the exception.

### Order behavior

Read:

- `src/order_builder/` for library order payload construction and conversion
- `src/bin/schwab-agent/order/` for CLI order workflows
- `src/bin/schwab-agent/verify.rs` for post-action verification

Rules:

- Order methods/builders must not invent safety shortcuts or silently mutate caller payloads.
- Mutable CLI order commands keep the config guard, preview/digest flow where applicable, canonical account-hash resolution, owner-only preview files, and post-action verification.

### Auth/security changes

Read:

- `src/auth.rs` for OAuth, token stores, and provider behavior
- `src/bin/schwab-agent/auth/` for CLI auth commands
- `src/config.rs`, `src/error.rs`, and `src/bin/schwab-agent/error/` for redaction and error mapping

Rules:

- OAuth callback URLs stay restricted to `https://127.0.0.1`.
- Token files and directories stay owner-only.
- Raw HTTP response bodies stay redacted in Debug/error output.

### Streaming changes

Read:

- `src/streaming_api.rs`
- `src/stream_session/`
- `src/models/streaming/`
- `tests/fixtures/streaming_*.json`

### CLI changes

Read:

- `src/bin/schwab-agent/cli.rs` for clap definitions
- `src/bin/schwab-agent/discovery.rs` and `src/bin/schwab-agent/config.rs` for schema/doctor/config output
- `src/bin/schwab-agent/error/` for stable error codes and exit codes
- the command module under `src/bin/schwab-agent/`
- `src/bin/schwab-agent/SKILL.md` for the LLM command contract
- `tests/cli_smoke.rs` for offline binary behavior checks

Rules:

- `schwab-agent` emits raw JSON payloads; errors use stable `ErrorBody` JSON.
- Shell completions are the raw-stdout exception.
- Keep command behavior, aliases, exit codes, JSON fields, and examples synchronized with `README.md` and `src/bin/schwab-agent/SKILL.md`.

### CI/release/tooling changes

Read only the file you are changing, plus the paired config if relevant:

- `.github/workflows/`
- `release-plz.toml`
- `dist-workspace.toml`
- `cliff.toml`
- `codecov.yml`
- `renovate.json`
