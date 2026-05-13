---
applyTo: "src/models/**/*.rs"
---

# Schwab API model review instructions

- Structs should match Schwab API JSON field names via serde attributes and avoid silently dropping important response fields.
- Public request parameter structs should model documented query and path parameters accurately.
- Changes to request payload structs need tests that prove the serialized JSON or query shape.
- Avoid speculative fields unless Schwab API behavior, documentation, or fixtures show they are needed.
