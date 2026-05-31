<!-- SYNC IMPACT REPORT
Version change: 1.0.0 → 1.1.0
Modified principles: N/A
Added principles:
  - VI. Test Coverage >= 95% (non-negotiable)
  - VII. Documentation Currency
Removed sections: N/A
Templates requiring updates:
  - .specify/templates/plan-template.md ✅ aligned
  - .specify/templates/spec-template.md ✅ aligned
  - .specify/templates/tasks-template.md ✅ aligned
Follow-up TODOs: None
-->

# Ktesio Constitution

## Core Principles

### I. CLI-First

Every feature MUST be accessible via the `kt` command-line interface.
No feature may exist only as a library API without a corresponding CLI subcommand.
CLI output MUST go to stdout; errors and diagnostics MUST go to stderr.
All commands MUST support `--help` and `--version` flags (provided by clap).

### II. Git-Based Storage

Skills are git repositories. The tool MUST support both HTTPS and SSH clone URLs.
A `skills.lock` file MUST map each installed skill to its exact commit SHA for reproducibility.
The lockfile MUST be updated on every `install` and `upgrade` operation.
Upgrades MUST pull from the default branch HEAD; no version pinning in v1.

### III. Manifest-Driven

`skills.json` is the single source of truth for which skills a project imports and exports.
The manifest MUST contain both `skills` (imports) and `exports` (local dirs) as top-level keys.
The manifest MUST be valid JSON with 2-space indentation for human readability.
Skill names MUST match `^[a-zA-Z0-9_-]+$` and MUST be unique within the manifest.

### IV. Graceful Degradation

Partial failures MUST NOT abort the entire operation. If one skill fails to clone or upgrade,
the tool MUST continue with remaining skills and report all errors at the end.
All error messages MUST include the specific failure reason and a suggested remediation action.
The tool MUST detect and report malformed JSON, duplicate names, and missing manifests
with clear, actionable diagnostics.

### V. Cross-Platform Compatibility

The tool MUST work on Linux, macOS, and Windows without platform-specific code paths.
File operations MUST use path-agnostic APIs (std::path::Path/PathBuf).
The `.agents/skills/` directory is the canonical install location across all platforms.

### VI. Test Coverage >= 95% (NON-NEGOTIABLE)

Test coverage MUST NOT drop below 95% at any time. This is a hard gate — no merge,
no release, no exception. Coverage is measured by `cargo-tarpaulin` or equivalent
line coverage tool against the `src/` directory. New code MUST include tests before
merging. Coverage regressions MUST be fixed before any other work proceeds.

### VII. Documentation Currency

The `docs/` directory MUST always reflect the current code and implementation status.
When a feature is added, changed, or removed, the corresponding documentation MUST
be updated in the same changeset. Stale documentation is treated as a bug.
The `README.md` MUST list all available commands with current examples.
The `quickstart.md` MUST match the actual CLI behavior at all times.

## Technology Constraints

- **Language**: Rust (2021 edition or later)
- **CLI framework**: clap 4 with derive macros
- **Error handling**: miette for diagnostics, thiserror for error types
- **Progress indicators**: indicatif for long-running operations
- **Serialization**: serde + serde_json for manifest and lockfile
- **Git operations**: Shell out to `git` CLI (not libgit2) for auth/proxy compatibility
- **Testing**: cargo test with unit and integration test separation
- **Performance targets**: Init <5s, single skill install <30s, list <1s

## Development Workflow

- All changes MUST be committed with descriptive messages following conventional commits
- Integration tests MUST cover each CLI command's happy path and error paths
- The `specs/` directory MUST contain the feature specification, plan, and tasks before implementation
- Task files MUST be organized by user story for independent implementation
- The `checklists/` directory MUST pass all items before proceeding to planning

## Governance

This constitution is the authoritative source for project conventions.
When conflicts arise between this document and other guidance, this constitution prevails.
Amendments require: (1) updated constitution file, (2) version bump, (3) sync impact report.
MAJOR version: principle removed or fundamentally redefined.
MINOR version: new principle added or material guidance expansion.
PATCH version: wording clarification, typo fix, non-semantic refinement.

**Version**: 1.1.0 | **Ratified**: 2026-05-30 | **Last Amended**: 2026-05-30
