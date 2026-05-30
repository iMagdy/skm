# Research: Skills Package Manager CLI

**Feature**: 001-skills-pkg-manager
**Date**: 2026-05-30

## Research Questions

### R1: Git Operations Strategy

**Decision**: Shell out to `git` CLI via `std::process::Command`

**Rationale**:
- Handles auth (SSH keys, credential helpers), proxies, and edge cases automatically
- No additional dependency (git2/libgit2 is ~50MB compile-time, adds FFI complexity)
- User already requires git on PATH (stated assumption)
- Simpler error messages — git CLI produces human-readable output we can forward

**Alternatives considered**:
- `git2` crate (libgit2 bindings): Pure Rust, no git dependency, but complex API, auth edge cases, larger binary size. Overkill for clone/fetch/checkout operations.
- `gix` crate: Pure Rust git implementation, still maturing. Risk of incomplete feature support.

**Implementation notes**:
- Use `Command::new("git")` with `.arg("--porcelain"` for parseable output where needed
- For clone: `git clone --depth 1 <url> <dir>` (shallow clone for speed, full clone not needed since we only need HEAD)
- Wait — need full clone for `git fetch` during upgrade. Use full clone.
- For fetch+checkout during upgrade: `git -C <dir> fetch origin && git -C <dir> checkout origin/<default-branch>`
- For resolving HEAD commit: `git -C <dir> rev-parse HEAD`
- For resolving default branch: `git -C <dir> remote show origin` or parse `git symbolic-ref refs/remotes/origin/HEAD`

---

### R2: Manifest & Lockfile Serialization

**Decision**: `serde` + `serde_json` with `#[serde(rename_all = "snake_case")]`

**Rationale**:
- Industry standard for Rust JSON handling
- Derive macros minimize boilerplate
- `serde_json::Value` for flexible parsing when needed

**Alternatives considered**:
- Manual JSON parsing: Error-prone, unnecessary
- `toml` or `yaml` format: User specified JSON (`skills.json`)

**Implementation notes**:
- Manifest struct: `{ skills: HashMap<String, SkillEntry>, exports: HashMap<String, ExportEntry> }`
- Lockfile struct: `{ HashMap<String, LockEntry> }` (top-level object keyed by skill name)
- Use `serde_json::to_string_pretty` for human-readable output with 2-space indent

---

### R3: Progress Indicators

**Decision**: `indicatif` with `MultiProgress` for concurrent skill operations

**Rationale**:
- Already chosen by user
- `MultiProgress` shows multiple progress bars simultaneously during parallel installs
- Supports spinners for indeterminate operations, bars for determinate ones

**Implementation notes**:
- Clone operation: spinner with "Cloning <skill-name>..." message
- Post-clone file copy: progress bar with bytes count
- `MultiProgress::new()` wrapping a `ProgressBar` per skill
- Use `ProgressStyle::default_spinner()` for clone, `ProgressStyle::default_bar()` for file copy

---

### R4: Error Handling Architecture

**Decision**: `miette` for error types + `thiserror` for error enum definitions

**Rationale**:
- `miette` provides rich diagnostic reports (source snippets, help text, related errors)
- `thiserror` generates `Error` trait impl for custom enums
- Together: define error variants with `thiserror`, wrap in `miette::Result` for user-facing output

**Alternatives considered**:
- `anyhow`: Less structured, no source snippets. miette is strictly better for CLI tools.
- Raw `Box<dyn std::error::Error>`: Loses type information

**Implementation notes**:
```rust
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::init::already_exists))]
pub struct InitError {
    message: String,
    #[source_code]
    path: String,
}
```

---

### R5: File Copying from Exports

**Decision**: `std::fs` for file operations, `walkdir` for recursive directory traversal

**Rationale**:
- `std::fs::copy` and `std::fs::create_dir_all` handle basic operations
- `walkdir` simplifies recursive traversal of exported directories
- No heavy dependency needed

**Alternatives considered**:
- `fs_extra`: Adds `copy_items` but `walkdir` + `std::fs` is sufficient
- `shutil`-style wrappers: Overkill for this scope

**Implementation notes**:
- After cloning, read source repo's `skills.json`
- For each export entry: walk the path, copy files to `.agents/skills/<skill-name>/`
- Preserve directory structure relative to the export path
- Skip `.git` directory when copying

---

### R6: Parallel Skill Operations

**Decision**: Sequential for v1, async Tokio as future enhancement

**Rationale**:
- Sequential is simpler, sufficient for 10-50 skills
- Network I/O is the bottleneck, not CPU
- Can add `tokio` + `futures` later if needed without API changes

**Alternatives considered**:
- `rayon`: Parallel iterators, but doesn't help with I/O-bound network ops
- `tokio` from start: Over-engineering for v1; adds complexity to progress bar coordination

**Implementation notes**:
- Loop over skills sequentially
- Each skill: clone → read exports → copy files → update lockfile
- Progress bar updates after each skill completes
- If a skill fails, log error and continue with next skill (FR-012 partial failure handling)

---

## Decisions Summary

| Area | Decision | Key Dependency |
|------|----------|----------------|
| Git ops | Shell out to `git` CLI | std::process::Command |
| JSON | serde + serde_json | serde, serde_json |
| Progress | indicatif MultiProgress | indicatif |
| Errors | miette + thiserror | miette, thiserror |
| File copy | std::fs + walkdir | walkdir |
| Concurrency | Sequential v1 | — |
