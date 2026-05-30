# Quickstart: GitHub CI Pipeline

**Feature**: 005-github-ci-pipeline
**Date**: Sat May 30 2026

## Overview

This feature adds a GitHub Actions CI pipeline that automatically runs lint, test, and build checks on every pull request. It provides fast feedback to developers and enforces code quality before merge.

## Prerequisites

- Repository must be on GitHub
- Rust toolchain must be available (handled by CI)
- `Cargo.lock` must be committed (for caching)

## What Gets Created

```text
.github/
└── workflows/
    └── ci.yml    # CI pipeline workflow
```

## How It Works

1. Developer opens a pull request (or pushes a new commit)
2. GitHub Actions triggers the pipeline automatically
3. Three parallel jobs run: lint, test, build
4. Each job reports pass/fail status on the PR
5. Developer sees results and fixes any issues

## Pipeline Jobs

| Job | What It Does | Time (typical) |
|-----|--------------|----------------|
| lint | Runs `cargo clippy` with warnings as errors | ~30s |
| test | Runs `cargo test` for all tests | ~1min |
| build | Runs `cargo build --release` to verify compilation | ~30s |
| coverage | Runs `cargo-tarpaulin` to enforce >=95% line coverage | ~2min |

**Total pipeline time**: ~2-3 minutes (jobs run in parallel)

## Local Verification

Before pushing, verify locally:

```bash
# Lint check
cargo clippy -- -D warnings

# Test check
cargo test

# Build check
cargo build --release
```

## Configuration

### Branch Protection (Optional)

To require all checks before merge:

1. Go to Settings → Branches → Add rule
2. Set branch name pattern (e.g., `main`)
3. Enable "Require status checks to pass"
4. Select required checks: `lint`, `test`, `build`, `coverage`

### Timeout Adjustment

Default timeout is 15 minutes. To change:

1. Edit `.github/workflows/ci.yml`
2. Modify `timeout-minutes` on the desired job

### Cache Invalidation

Cache is automatically invalidated when `Cargo.lock` changes. To force:

1. Delete the Actions cache via Settings → Actions → Caches
2. Next run will rebuild from scratch

### Fork Pull Requests

By default, GitHub Actions does not run workflows on pull requests from forks for security reasons. To enable fork PRs:

1. Go to Settings → Actions → General
2. Under "Workflow permissions", select "Allow all actions and reusable workflows"
3. Under "Fork pull requests", enable "Run workflows from fork pull requests"

**Note**: Fork workflows run with read-only permissions and cannot access repository secrets. If your pipeline requires secrets (e.g., for deployment), fork PRs will need an alternative approach.

## Troubleshooting

| Problem | Solution |
|---------|----------|
| Pipeline doesn't trigger | Verify PR targets a branch; check workflow file syntax |
| Cache not working | Ensure `Cargo.lock` is committed |
| Timeout exceeded | Check for infinite loops or very slow tests |
| Fork PR not running | Enable "Allow all actors" in workflow triggers |

## Related

- **Spec**: [spec.md](spec.md)
- **Plan**: [plan.md](plan.md)
- **Research**: [research.md](research.md)
