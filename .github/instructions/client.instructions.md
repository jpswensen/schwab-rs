---
applyTo: "src/client.rs"
---

# Client review instructions

- Preserve async method signatures and return `schwab::Result<T>`.
- Response bodies and connections must be properly consumed or dropped.
- HTTP errors should map to typed library errors with useful caller-facing context.
- Client configuration should be explicit and testable through `Config`. Do not add hidden environment or config-file behavior.
