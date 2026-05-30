# Implementation Plan: Skills Package Manager CLI

**Branch**: `001-skills-pkg-manager` | **Date**: 2026-05-30 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-skills-pkg-manager/spec.md`

## Summary

Build a Rust CLI tool (`skm`) that manages agentic skills as git-based packages. The tool reads a `skills.json` manifest to declare skill imports (git repos) and exports (local dirs), clones skill repos, uses the source repo's exports to determine which files to copy into `.agents/skills/`, and maintains a `skills.lock` file for reproducible installations. Commands: `init`, `install`, `upgrade`, `list`, `show`, `uninstall`/`remove`.

## Technical Context

**Language/Version**: Rust (latest stable, 2021 edition)
**Primary Dependencies**: clap (CLI argument parsing), miette (error reporting/diagnostics), indicatif (progress bars)
**Storage**: Filesystem — `skills.json` manifest, `skills.lock` lockfile, `.agents/skills/` directories
**Testing**: cargo test (unit + integration)
**Target Platform**: Cross-platform CLI (Linux, macOS, Windows)
**Project Type**: CLI tool (binary crate)
**Performance Goals**: Init <5s, single skill install <30s, list <1s
**Constraints**: Requires git on PATH; network access for clone/fetch; public repos or pre-configured SSH keys
**Scale/Scope**: Single project root; ~10-50 skills per project; individual skill repos <100MB

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Constitution is a template (not yet customized). No gates to evaluate — proceed to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/001-skills-pkg-manager/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (not created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── main.rs              # Entry point, CLI arg parsing with clap
├── cli/
│   ├── mod.rs
│   ├── init.rs          # skm init command
│   ├── install.rs       # skm install (bulk + single)
│   ├── upgrade.rs       # skm upgrade command
│   ├── list.rs          # skm list command
│   ├── show.rs          # skm show command
│   └── uninstall.rs     # skm uninstall/remove command
├── manifest.rs          # skills.json parsing and serialization
├── lockfile.rs          # skills.lock parsing and serialization
├── git.rs               # Git operations (clone, fetch, checkout, resolve HEAD)
├── skill.rs             # Skill entity, file copying from exports
└── error.rs             # Custom error types with miette diagnostics

tests/
├── integration/
│   ├── init_test.rs
│   ├── install_test.rs
│   ├── upgrade_test.rs
│   ├── list_test.rs
│   ├── show_test.rs
│   └── uninstall_test.rs
└── unit/
    ├── manifest_test.rs
    ├── lockfile_test.rs
    └── git_test.rs
```

**Structure Decision**: Single binary crate with flat module structure. CLI commands in `src/cli/` each handle one subcommand. Core logic (manifest, lockfile, git, skill) in `src/`. Tests mirror source structure.

## Complexity Tracking

> No Constitution violations — constitution is a template, no gates defined.
