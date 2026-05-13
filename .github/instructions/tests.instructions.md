---
applyTo: "tests/**/*.rs"
---

# Test review instructions

- Use standard `assert!`, `assert_eq!`, `assert_ne!` macros for assertions.
- Mock HTTP with `mockito` and validate expected request method, path, query, headers, and body inline.
- Prefer descriptive test function names that explain the scenario being tested.
- Keep generated data inline unless there is a clear reason to introduce fixture files.
- Do not request coverage-only tests when critical behavior is already covered.
