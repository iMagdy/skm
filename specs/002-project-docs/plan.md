# Implementation Plan: Project Documentation

**Branch**: `002-project-docs` | **Date**: 2026-05-30 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-project-docs/spec.md`

## Summary

Create a comprehensive `docs/` directory for the skm project containing user-facing and contributor-facing documentation. The documentation set covers installation, command reference, contributing guidelines, architecture, testing, manifest format, and lockfile format. Documentation is written in plain Markdown (no frontmatter), targets intermediate developers, and is validated by a CI job checking for broken links and required file presence. The root README.md serves as a concise entry point linking into `docs/`.

## Technical Context

**Language/Version**: Rust (2021 edition or later)
**Primary Dependencies**: None (documentation only — no code changes)
**Storage**: N/A (files in `docs/` directory)
**Testing**: N/A for doc content; CI validates link integrity and file presence
**Target Platform**: Cross-platform (Linux, macOS, Windows)
**Project Type**: CLI tool — documentation feature
**Performance Goals**: N/A
**Constraints**: Plain Markdown, no YAML frontmatter, no changelog (git history is version record)
**Scale/Scope**: 7-8 Markdown files in `docs/`, root README updated to link into docs/

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. CLI-First | ✅ PASS | Docs document all CLI commands with `--help` and `--version` behavior |
| II. Git-Based Storage | ✅ PASS | Docs explain git-based skill storage and lockfile format |
| III. Manifest-Driven | ✅ PASS | Docs include `skills.json` manifest format documentation |
| IV. Graceful Degradation | ✅ PASS | Docs document error handling patterns (FR-015) |
| V. Cross-Platform | ✅ PASS | Docs document cross-platform requirements (FR-014) |
| VI. Test Coverage >= 95% | ✅ PASS | Testing guide documents coverage verification (FR-012) |
| VII. Documentation Currency | ✅ PASS | This feature IS the documentation; constitution requires docs to stay current |

**Gate Result**: PASS — No violations.

## Project Structure

### Documentation (this feature)

```text
specs/002-project-docs/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
docs/
├── README.md            # Table of contents / index (FR-002)
├── installation.md      # Installation guide (FR-003)
├── commands.md          # Command reference (FR-004)
├── contributing.md      # Contributing guidelines (FR-005)
├── architecture.md      # Architecture guide (FR-006)
├── testing.md           # Testing & coverage guide (FR-007)
├── manifest.md          # skills.json format (FR-008)
└── lockfile.md          # skills.lock format (FR-009)

README.md                # Updated to link into docs/ (root entry point)
```

**Structure Decision**: Standard open-source docs layout with a `docs/` directory at project root. Each file is a standalone Markdown document covering a single topic. The root README.md is updated to link into `docs/` sections, keeping it concise while `docs/` holds depth.

## Complexity Tracking

> No Constitution Check violations — this section is empty.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
