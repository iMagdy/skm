# Contract: GitHub Actions Workflow Schema

**Feature**: 005-github-ci-pipeline
**Date**: Sat May 30 2026

## Overview

This contract defines the structure and behavior of the GitHub Actions CI workflow. It specifies the expected inputs, outputs, and configuration for the pipeline.

## Workflow File

**Path**: `.github/workflows/ci.yml`

### Trigger Contract

```yaml
on:
  pull_request:
    types: [opened, synchronize, reopened]
```

| Field | Value | Requirement |
|-------|-------|-------------|
| event | `pull_request` | MUST trigger on PR events only |
| types | `opened`, `synchronize`, `reopened` | MUST cover all PR lifecycle events |

### Concurrency Contract

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.head_ref }}
  cancel-in-progress: true
```

| Field | Value | Requirement |
|-------|-------|-------------|
| group | `${{ github.workflow }}-${{ github.head_ref }}` | MUST be unique per PR branch |
| cancel-in-progress | `true` | MUST cancel superseded runs |

### Job Contract: Lint

| Field | Value | Requirement |
|-------|-------|-------------|
| name | `lint` | MUST be `lint` for branch protection |
| runs-on | `ubuntu-latest` | MUST use GitHub-hosted runner |
| timeout-minutes | `15` | MUST not exceed 15 minutes |

**Steps**:

| Step | Action/Command | Purpose |
|------|----------------|---------|
| Checkout | `actions/checkout@v4` | Fetch repository code |
| Install Rust | `dtolnay/rust-toolchain@stable` | Ensure consistent Rust version |
| Cache | `actions/cache@v4` | Cache Cargo dependencies |
| Clippy | `cargo clippy -- -D warnings` | Lint with warnings as errors |

### Job Contract: Test

| Field | Value | Requirement |
|-------|-------|-------------|
| name | `test` | MUST be `test` for branch protection |
| runs-on | `ubuntu-latest` | MUST use GitHub-hosted runner |
| timeout-minutes | `15` | MUST not exceed 15 minutes |

**Steps**:

| Step | Action/Command | Purpose |
|------|----------------|---------|
| Checkout | `actions/checkout@v4` | Fetch repository code |
| Install Rust | `dtolnay/rust-toolchain@stable` | Ensure consistent Rust version |
| Cache | `actions/cache@v4` | Cache Cargo dependencies |
| Test | `cargo test` | Run all unit and integration tests |

### Job Contract: Build

| Field | Value | Requirement |
|-------|-------|-------------|
| name | `build` | MUST be `build` for branch protection |
| runs-on | `ubuntu-latest` | MUST use GitHub-hosted runner |
| timeout-minutes | `15` | MUST not exceed 15 minutes |

**Steps**:

| Step | Action/Command | Purpose |
|------|----------------|---------|
| Checkout | `actions/checkout@v4` | Fetch repository code |
| Install Rust | `dtolnay/rust-toolchain@stable` | Ensure consistent Rust version |
| Cache | `actions/cache@v4` | Cache Cargo dependencies |
| Build | `cargo build --release` | Verify compilation succeeds |

### Job Contract: Coverage

| Field | Value | Requirement |
|-------|-------|-------------|
| name | `coverage` | MUST be `coverage` for branch protection |
| runs-on | `ubuntu-latest` | MUST use GitHub-hosted runner |
| timeout-minutes | `15` | MUST not exceed 15 minutes |

**Steps**:

| Step | Action/Command | Purpose |
|------|----------------|---------|
| Checkout | `actions/checkout@v4` | Fetch repository code |
| Install Rust | `dtolnay/rust-toolchain@stable` | Ensure consistent Rust version |
| Cache | `actions/cache@v4` | Cache Cargo dependencies |
| Coverage | `cargo install cargo-tarpaulin && cargo tarpaulin --fail-under 95` | Enforce >=95% line coverage gate |

### Cache Contract

| Field | Value | Requirement |
|-------|-------|-------------|
| key | `${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}` | MUST invalidate on dependency changes |
| path | `~/.cargo/registry`, `~/.cargo/git`, `target/` | MUST cache standard Cargo directories |
| restore-keys | `${{ runner.os }}-cargo-` | MUST support partial cache hits |

## Status Reporting Contract

The pipeline MUST report four independent check statuses to the pull request:

| Check Name | Description | Required for Merge |
|------------|-------------|-------------------|
| `lint` | Clippy linting results | Configurable via branch protection |
| `test` | Test execution results | Configurable via branch protection |
| `build` | Compilation results | Configurable via branch protection |
| `coverage` | Line coverage >=95% gate | Configurable via branch protection |

## Error Handling Contract

| Scenario | Expected Behavior |
|----------|-------------------|
| Lint failure | Job fails, check shows error output |
| Test failure | Job fails, check shows failed test names |
| Build failure | Job fails, check shows compiler errors |
| Coverage below 95% | Job fails, check shows coverage report with uncovered lines |
| Timeout | Job cancelled, check shows timeout message |
| Cache miss | Job proceeds with cold build (no failure) |
| Network error | Job may retry automatically (GitHub Actions built-in) |
