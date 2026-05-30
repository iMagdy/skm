# Testing

This guide explains how to run tests, measure coverage, and verify compliance with the constitution's >=95% coverage requirement.

## Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test '*'
```

## Measuring Coverage

The project uses `cargo-tarpaulin` for line coverage measurement against the `src/` directory.

### Install cargo-tarpaulin

```bash
cargo install cargo-tarpaulin
```

### Generate Coverage Report

```bash
# Generate coverage report
cargo tarpaulin --out Stdout

# Generate HTML report
cargo tarpaulin --out Html

# Generate coverage for specific output format
cargo tarpaulin --out Xml
```

### Coverage Threshold

The constitution requires >=95% line coverage (Principle VI). This is a non-negotiable gate — no merge, no release, no exception.

```bash
# Check coverage meets threshold
cargo tarpaulin --fail-under 95
```

## Coverage Enforcement

Coverage is enforced in CI as a gate before merge. The CI pipeline runs:

```bash
cargo tarpaulin --fail-under 95 --out Stdout
```

If coverage drops below 95%, the CI job fails and the PR cannot be merged.

## Investigating Coverage Gaps

### Identify Untested Code

```bash
# Generate HTML report to see uncovered lines
cargo tarpaulin --out Html

# Open the report
open tarpaulin-report.html
```

### Common Coverage Gaps

1. **Error paths**: Ensure all error conditions are tested
2. **Edge cases**: Test boundary conditions and invalid inputs
3. **Platform-specific code**: Test cross-platform behavior if applicable
4. **Git operations**: Code that shells out to git CLI requires actual git repos for testing

### Fixing Coverage Issues

1. Identify uncovered lines in the HTML report
2. Write tests that exercise those code paths
3. Re-run coverage to verify the fix
4. Ensure total coverage remains >=95%

## CI Pipeline

The CI pipeline includes:

1. **Build check**: `cargo build --release`
2. **Test suite**: `cargo test`
3. **Clippy lint**: `cargo clippy -- -D warnings`
4. **Format check**: `cargo fmt --check`
5. **Coverage gate**: `cargo tarpaulin --fail-under 95`

All checks must pass before a PR can be merged.

## See Also

- [Contributing](contributing.md) — Development workflow and PR process
- [Architecture](architecture.md) — Codebase structure
