# Implementation Plan: Test Coverage & Integration Tests

**Branch**: `004-integ-tests-coverage` | **Date**: Sat May 30 2026 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/004-integ-tests-coverage/spec.md`

## Summary

Achieve >=95% line coverage across `src/` as required by constitution Principle VI, and add integration tests for the `install` and `export` commands using real remote repositories as test fixtures. Integration tests validate both the default install path (repo with `skills.json`) and the fallback discovery path (repo without `skills.json`).

## Technical Context

**Language/Version**: Rust (2021 edition or later)
**Primary Dependencies**: clap 4, miette 7, indicatif 0.17, serde/serde_json 1, thiserror 2, walkdir 2, regex 1, dialoguer 0.11
**Storage**: Filesystem (`.agents/skills/`, `skills.lock`, `skills.json`)
**Testing**: cargo test with unit/integration separation, cargo-tarpaulin for coverage
**Target Platform**: Cross-platform (Linux, macOS, Windows)
**Project Type**: CLI tool (agentic skills package manager)
**Performance Goals**: Single skill install <30s
**Constraints**: 95% line coverage hard gate, cross-platform compatibility
**Scale/Scope**: ~15 source files, ~89 existing unit tests

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. CLI-First | ✅ PASS | Feature adds tests, not new CLI commands |
| II. Git-Based Storage | ✅ PASS | Tests validate git clone operations |
| III. Manifest-Driven | ✅ PASS | Tests validate skills.json handling |
| IV. Graceful Degradation | ✅ PASS | Tests validate error path behavior |
| V. Cross-Platform | ✅ PASS | Tests use path-agnostic APIs |
| VI. Test Coverage >=95% | ✅ PASS | Feature directly addresses this requirement |
| VII. Documentation Currency | ✅ PASS | quickstart.md will be updated |

**Gate Result**: PASS — no violations requiring justification.

## Project Structure

### Documentation (this feature)

```text
specs/004-integ-tests-coverage/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (via /speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── cli/
│   ├── install.rs       # Unit tests (existing), integration test target
│   ├── init.rs          # Unit tests (existing)
│   ├── list.rs          # Unit tests (existing)
│   ├── show.rs          # Unit tests (existing)
│   ├── uninstall.rs     # Unit tests (existing)
│   ├── upgrade.rs       # Unit tests (existing)
│   └── mod.rs
├── discovery.rs         # Unit tests (existing), integration test target
├── error.rs
├── git.rs               # Unit tests (existing)
├── lockfile.rs          # Unit tests (existing), integration test target
├── main.rs
├── manifest.rs          # Unit tests (existing), integration test target
└── skill.rs             # Unit tests (existing)

tests/
├── integration/
│   ├── install_default.rs    # NEW: Default install path tests
│   ├── install_fallback.rs   # NEW: Fallback discovery tests
│   └── export.rs             # NEW: Export command tests
└── unit/
    └── (existing unit tests remain in src/ modules)
```

**Structure Decision**: Follow existing Rust convention with unit tests co-located in source modules and integration tests in `tests/integration/`. This maintains the project's established pattern.

## Complexity Tracking

No constitution violations — no complexity tracking required.
