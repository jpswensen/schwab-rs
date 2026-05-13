---
applyTo: "**/*.rs"
---

# Rust review instructions

- Prefer small, focused functions with explicit error handling via `Result` and the `?` operator.
- Use `thiserror` for defining library error types and wrap errors with enough context for callers.
- Preserve async patterns for API calls and cancellation via `tokio`.
- Keep public items documented with useful doc comments.
- Do not add process-level behavior such as `process::exit`, stdout writes, or hidden environment/config reads to library modules.
- Request builders should keep path parameters URL-encoded, query parameters named exactly as Schwab documents them, and request bodies typed with serde.
- Response structs should expose useful typed fields to callers instead of forcing callers to parse raw JSON.
- Do not suggest style-only changes that `rustfmt` or `clippy` already enforces.
