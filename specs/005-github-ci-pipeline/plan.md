# Implementation Plan: GitHub CI Pipeline

**Branch**: `005-github-ci-pipeline` | **Date**: Sat May 30 2026 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/005-github-ci-pipeline/spec.md`

## Summary

Add a GitHub Actions CI pipeline that triggers on every pull request (opened, synchronize, reopened) and runs three parallel jobs: lint (cargo clippy), test (cargo test), and build (cargo build --release). Each job reports independent pass/fail status on the PR. Superseded runs are cancelled automatically via concurrency groups. Pipeline has a 15-minute timeout and uses Cargo caching for fast feedback.

## Technical Context

**Language/Version**: YAML (GitHub Actions workflow)
**Primary Dependencies**: GitHub Actions (`actions/checkout@v4`, `actions/cache@v4`, `dtolnay/rust-toolchain@stable`)
**Storage**: N/A (no persistent storage needed)
**Testing**: cargo test (existing test suite)
**Target Platform**: GitHub Actions runners (ubuntu-latest)
**Project Type**: CI/CD configuration (workflow file addition)
**Performance Goals**: <30s trigger, <5min total feedback, <15min timeout
**Constraints**: Parallel execution, per-PR cancellation, fork PR support
**Scale/Scope**: 1 workflow file, 3 jobs, ~100 lines of YAML

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. CLI-First | ✅ PASS | Feature adds CI, not new CLI commands |
| II. Git-Based Storage | ✅ PASS | No storage changes |
| III. Manifest-Driven | ✅ PASS | No manifest changes |
| IV. Graceful Degradation | ✅ PASS | Pipeline handles failures per-job with clear reporting |
| V. Cross-Platform | ✅ PASS | GitHub Actions supports Linux/macOS/Windows runners |
| VI. Test Coverage >=95% | ✅ PASS | Pipeline enforces test execution; coverage gate can be added later |
| VII. Documentation Currency | ✅ PASS | quickstart.md created, README will be updated |

**Gate Result**: PASS — no violations requiring justification.

## Project Structure

### Documentation (this feature)

```text
specs/005-github-ci-pipeline/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── workflow-schema.md
└── tasks.md             # Phase 2 output (via /speckit.tasks)
```

### Source Code (repository root)

```text
.github/
└── workflows/
    └── ci.yml           # NEW: CI pipeline workflow

src/                     # Existing (unchanged)
├── cli/
│   ├── install.rs
│   ├── init.rs
│   ├── list.rs
│   ├── show.rs
│   ├── uninstall.rs
│   ├── upgrade.rs
│   └── mod.rs
├── discovery.rs
├── error.rs
├── git.rs
├── lockfile.rs
├── main.rs
├── manifest.rs
└── skill.rs

tests/                   # Existing (unchanged)
├── helpers/
│   └── mod.rs
├── install_fallback.rs
├── install_default.rs
└── export.rs
```

**Structure Decision**: Single new file `.github/workflows/ci.yml` added to repository root. No changes to existing source code or test structure. This is a pure CI configuration addition.

## Complexity Tracking

No constitution violations — no complexity tracking required.
