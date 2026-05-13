# Security Policy

`schwab-rs` is an unofficial project. It is not affiliated with, endorsed by, or sponsored by Charles Schwab & Co., Inc., Schwab brokerage services, or thinkorswim.

## Supported versions

This crate is pre-1.0. Security fixes are expected to land on the `main` branch until the first public release defines a longer-term support policy.

## Reporting a vulnerability

Please do not open a public issue for suspected security vulnerabilities. Email the maintainer instead, or use GitHub private vulnerability reporting if it is enabled for this repository.

Include enough detail to reproduce the issue, including the affected crate version or commit, operating system, feature flags, and a minimal example when possible. Do not include live Schwab credentials, OAuth authorization codes, refresh tokens, access tokens, account numbers, or other sensitive financial data.

## Token and credential handling

`schwab-rs` handles OAuth client secrets, authorization codes, access tokens, and refresh tokens. Treat token files and example output as sensitive material:

- Store credentials in environment variables or a secret manager, not in source files.
- Do not commit token files such as `schwab-token.json`.
- Redact tokens, authorization codes, account numbers, and client secrets from logs, issues, crash reports, and test fixtures.
- Prefer the provided auth helpers for token storage because they use atomic writes and owner-only permissions on Unix.

If you accidentally expose a token or client secret, revoke or rotate it in the Schwab developer portal before sharing logs or reproduction details.
