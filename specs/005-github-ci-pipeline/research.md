# Research: GitHub CI Pipeline

**Feature**: 005-github-ci-pipeline
**Date**: Sat May 30 2026

## Research Tasks

### 1. CI Platform Selection

**Decision**: Use GitHub Actions (native to the repository's hosting platform).

**Rationale**:
- Repository is hosted on GitHub (`github.com/iMagdy/skm`)
- Native integration with PR status checks — no external service configuration
- Free for public repositories, generous free tier for private
- Supports parallel jobs, caching, and matrix builds
- YAML-based workflow files are version-controlled with the codebase

**Alternatives Considered**:
- **CircleCI**: Requires external account setup and webhook configuration; more complex for simple pipelines
- **Travis CI**: Less active development, limited free tier for open source
- **Jenkins**: Requires self-hosted infrastructure, overkill for a single-repo project
- **GitLab CI**: Would require migrating to GitLab; unnecessary platform switch

### 2. Workflow Trigger Configuration

**Decision**: Use `pull_request` event with `opened`, `synchronize`, and `reopened` activity types.

**Rationale**:
- `opened` covers new PRs (FR-001)
- `synchronize` covers new commits pushed to PR branch (FR-007)
- `reopened` covers re-opening closed PRs
- `pull_request` (not `pull_request_target`) runs in the PR's context, safe for fork PRs
- Avoids `push` trigger to prevent duplicate runs on direct pushes

**Alternatives Considered**:
- `push` + `pull_request`: Would cause duplicate runs for branch pushes
- `workflow_dispatch`: Manual trigger only, doesn't meet automation requirement
- `pull_request_target`: Runs in base branch context, risky for fork contributions

### 3. Linting Strategy

**Decision**: Use `cargo clippy` for Rust linting with `-D warnings` flag.

**Rationale**:
- `cargo clippy` is the standard Rust linter, catches common mistakes
- `-D warnings` treats all warnings as errors (fail-fast)
- No additional tool installation needed — included with Rust toolchain
- Aligns with constitution Principle V (cross-platform compatibility)

**Alternatives Considered**:
- `rustfmt` only: Catches formatting but not logic/style issues
- Custom lint scripts: Maintenance burden, no benefit over clippy
- `cargo audit`: Security-focused, not style/lint — could add as separate job

### 4. Build Verification

**Decision**: Use `cargo build --release` to verify the project compiles.

**Rationale**:
- Verifies no compilation errors across all modules
- `--release` mode catches optimization-related issues
- Fast enough for a CLI project (~30s cold, ~5s incremental)
- Complements `cargo test` (tests may skip build if cached)

**Alternatives Considered**:
- `cargo check`: Faster but doesn't produce binary; misses linker errors
- `cargo build` (debug): May not catch release-only issues
- `cargo build --all-targets`: Includes tests/benches in build, redundant with test step

### 5. Caching Strategy

**Decision**: Use `actions/cache` for Cargo registry and target directory.

**Rationale**:
- Reduces cold-start time from ~60s to ~10s
- Cache key based on `Cargo.lock` ensures invalidation on dependency changes
- Standard GitHub Actions caching pattern for Rust projects
- Meets SC-001 (30s trigger) and SC-002 (5min feedback)

**Alternatives Considered**:
- No caching: Cold builds exceed performance targets
- `Swatinem/rust-cache`: Third-party action, adds dependency; manual cache works fine
- `actions/cache/restore` + `actions/cache/save`: More control but same outcome

### 6. Job Structure

**Decision**: Run lint, test, and build as three parallel jobs using a matrix strategy.

**Rationale**:
- Parallel execution meets SC-004 (minimize total duration)
- Each job reports independently (FR-005)
- Failure in one job doesn't block others (independent validation)
- Matrix strategy simplifies workflow file maintenance

**Alternatives Considered**:
- Single job with sequential steps: Slower, single point of failure
- Separate workflow files: Overcomplicated, harder to maintain
- Reusable workflows: Unnecessary abstraction for 3 simple jobs

### 7. Timeout Configuration

**Decision**: Set `timeout-minutes: 15` at the job level, configurable via workflow dispatch input.

**Rationale**:
- Default 15 minutes per clarification answer
- Prevents runaway jobs from consuming resources
- Configurable via `workflow_dispatch` for special cases
- Aligns with edge case handling in spec

**Alternatives Considered**:
- No timeout: Risk of infinite loops consuming resources
- 30 minutes: Too generous, wastes runner minutes
- Global workflow timeout: Less granular control

### 8. Cancellation Strategy

**Decision**: Use `concurrency` group with `cancel-in-progress: true` for the same PR.

**Rationale**:
- FR-008 requires cancelling superseded runs
- `concurrency` group keyed on `github.head_ref` ensures per-PR cancellation
- `cancel-in-progress: true` cancels previous runs when new commit arrives
- Doesn't affect other PRs (meets multi-PR concurrency requirement)

**Alternatives Considered**:
- Manual cancellation via API: Doesn't meet automatic requirement
- `cancel-in-progress: false`: Queues instead of cancelling, slower feedback
- Separate concurrency groups per job: Overcomplicated

## Summary of Decisions

| Area | Decision | Key Benefit |
|------|----------|-------------|
| Platform | GitHub Actions | Native integration, zero config |
| Triggers | `pull_request` (opened/synchronize/reopened) | Covers all PR lifecycle events |
| Lint | `cargo clippy -D warnings` | Standard Rust linting, fail-fast |
| Build | `cargo build --release` | Full compilation verification |
| Caching | `actions/cache` for Cargo | Reduces cold-start by ~50s |
| Job structure | 3 parallel jobs via matrix | Independent reporting, fast feedback |
| Timeout | 15min default, configurable | Resource protection |
| Cancellation | `concurrency` group per PR | Superseded runs cancelled automatically |
