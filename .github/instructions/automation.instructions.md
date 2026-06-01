---
applyTo: ".github/workflows/**"
---

# GitHub Actions review instructions

- Validate GitHub Actions syntax, minimum permissions, and secret handling.
- Actions should be pinned consistently with the existing workflow style.
- Third-party tools installed inside workflows should also be pinned to explicit versions and avoid fallback installers when they execute repository code or run in a job with secrets.
- Avoid workflow changes that spend extra CI minutes without a clear project benefit.
- Build verification should use `cargo clippy` and `cargo test` for this library repository, with coverage and unused-dependency checks where the workflow already defines them.
