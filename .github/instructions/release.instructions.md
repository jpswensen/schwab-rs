---
applyTo: ".github/workflows/cd.yml"
---

# Release review instructions

- Release automation uses `release-plz` for changelog generation and crate publishing.
- This repository is a Rust library, so releases publish to crates.io, not binary artifacts.
- Do not add binary build matrices unless the repository gains a `main.rs` binary target.
