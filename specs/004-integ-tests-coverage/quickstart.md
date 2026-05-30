# Quickstart: Test Coverage & Integration Tests

**Feature**: 004-integ-tests-coverage
**Date**: Sat May 30 2026

## Overview

This feature ensures the project meets the constitution's 95% coverage requirement and adds integration tests for the install and export commands using real remote repositories.

## Prerequisites

- Rust toolchain installed
- cargo-tarpaulin: `cargo install cargo-tarpaulin`
- Network access (for integration tests)

## Running Unit Tests

```bash
# Run all unit tests
cargo test

# Run with coverage
cargo tarpaulin --out Html
```

## Running Integration Tests

```bash
# Run all integration tests (requires network)
cargo test --test '*' --features integration

# Run specific integration test
cargo test --test install_default
cargo test --test install_fallback
cargo test --test export

# Run ignored tests (network-dependent)
cargo test -- --ignored
```

## Checking Coverage

```bash
# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# Enforce 95% threshold (CI)
cargo tarpaulin --fail-under 95
```

## Test Structure

```text
tests/
├── integration/
│   ├── install_default.rs    # Default install path tests
│   ├── install_fallback.rs   # Fallback discovery tests
│   └── export.rs             # Export command tests
└── unit/
    └── (tests co-located in src/ modules)
```

## Fixture Repositories

| Repository | URL | SHA | Purpose |
|------------|-----|-----|---------|
| awesome-copilot | https://github.com/iMagdy/awesome-copilot.git | 118974fb... | Default install/export |
| agent-skills | https://github.com/iMagdy/agent-skills | 18011566... | Fallback discovery |

## CI Integration

Integration tests are retried up to 3 times before failing to handle transient network issues. Coverage gate enforces >=95% line coverage.
