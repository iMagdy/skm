# Implementation Plan: Skill Install Fallback

**Branch**: `003-skill-install-fallback` | **Date**: Sat May 30 2026 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/003-skill-install-fallback/spec.md`

## Summary

Add fallback discovery mechanism to `skm install` that enables installing skills from repositories lacking `skills.json` manifest. When a manifest is not found, the system searches for a `skills/` directory (case-insensitive), discovers available skills from `.md` files and subdirectories, deduplicates by name, and prompts the user to select which skill to install.

## Technical Context

**Language/Version**: Rust 1.75+
**Primary Dependencies**: clap 4, miette, indicatif, serde/serde_json, regex
**Storage**: Git-based (filesystem + git CLI)
**Testing**: cargo test
**Target Platform**: Linux, macOS, Windows (cross-platform)
**Project Type**: CLI tool
**Performance Goals**: Single skill install <30s (existing target)
**Constraints**: Path-agnostic APIs, graceful degradation
**Scale/Scope**: Single skill install operation

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. CLI-First | ✅ | Feature is CLI-driven via `skm install` |
| II. Git-Based Storage | ✅ | Skills remain git repositories |
| III. Manifest-Driven | ⚠️ | **JUSTIFIED**: Adds fallback when manifest is missing; existing manifest flow unchanged |
| IV. Graceful Degradation | ✅ | Error handling for empty dirs, cancelled prompts |
| V. Cross-Platform Compatibility | ✅ | Using `std::path::Path` APIs |
| VI. Test Coverage >= 95% | ✅ | Tests required for new code |
| VII. Documentation Currency | ✅ | Docs update required |

**Gate Result**: PASS with justification for Principle III deviation

## Project Structure

### Documentation (this feature)

```text
specs/003-skill-install-fallback/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── cli/
│   └── install.rs       # Modified: add fallback discovery
├── discovery.rs         # New: skill discovery module
├── error.rs             # Modified: add new error types
├── skill.rs             # Existing: skill file operations
├── manifest.rs          # Existing: manifest handling
├── git.rs               # Existing: git operations
└── main.rs              # Existing: entry point

tests/
├── integration/
│   └── install_fallback.rs  # New: integration tests
└── unit/
    └── discovery.rs         # New: unit tests
```

**Structure Decision**: Single project structure (Option 1). New `discovery.rs` module handles fallback logic, keeping `install.rs` focused on orchestration.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Principle III deviation | User need to install from repos without manifest | Would require all repos to maintain skills.json, limiting ecosystem |
