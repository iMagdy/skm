# Contract: Documentation File Structure

**Date**: 2026-05-30
**Feature**: 002-project-docs

## Overview

This contract defines the structure and required content for each documentation file in `docs/`. These are not API contracts but structural contracts that each documentation file must satisfy.

---

## Contract 1: Installation Guide (`docs/installation.md`)

**Purpose**: Document how to install Ktesio on all supported platforms.

**Required sections**:
1. `# Installation` — Top-level heading
2. `## Prerequisites` — System requirements (Rust, Git)
3. `## Install from Source` — Build from git repository
4. `## Install via Cargo` — `cargo install` instructions (if published)
5. `## Verify Installation` — How to confirm Ktesio is working

**Required examples**:
- At least one `cargo install` or `cargo build` command
- At least one `kt --version` verification command

**Acceptance criteria**:
- A developer can install Ktesio on Linux, macOS, or Windows by following the steps
- All code blocks use language-specific fencing (```bash)

---

## Contract 2: Command Reference (`docs/commands.md`)

**Purpose**: Document every Ktesio subcommand with usage syntax, options, and examples.

**Required sections**:
1. `# Command Reference` — Top-level heading
2. One section per command (`## kt init`, `## kt install`, etc.)

**Per-command structure**:
```
## kt <command>

<One-line description>

**Usage**: `kt <command> [options] [args]`

**Options**:
- `--help` — Show help
- `--version` — Show version
- <command-specific options>

**Examples**:
```bash
kt <command> <example-args>
```
```

**Required commands** (from spec.md):
- `kt init`
- `kt install`
- `kt upgrade`
- `kt list`
- `kt show`
- `kt uninstall` / `kt remove`

**Acceptance criteria**:
- Every CLI command documented in the spec has at least one usage example (SC-003)
- All commands document `--help` and `--version` behavior (FR-013)

---

## Contract 3: Contributing Guide (`docs/contributing.md`)

**Purpose**: Explain the development setup, coding standards, testing requirements, and PR workflow.

**Required sections**:
1. `# Contributing` — Top-level heading
2. `## Development Setup` — Prerequisites and build steps
3. `## Project Structure` — Overview of `src/` layout
4. `## Coding Standards` — Rust style, formatting, conventions
5. `## Testing` — How to run tests and check coverage
6. `## Pull Request Process` — Workflow and review criteria

**Acceptance criteria**:
- A new contributor can clone the repo, build, and run tests by following the docs alone (SC-002)
- Documents the >=95% test coverage requirement (FR-012)

---

## Contract 4: Architecture Guide (`docs/architecture.md`)

**Purpose**: Describe the module structure, key data flows, and design decisions.

**Required sections**:
1. `# Architecture` — Top-level heading
2. `## Module Overview` — Description of each file in `src/`
3. `## Data Flow` — How commands process manifests and lockfiles
4. `## Design Decisions` — Key architectural choices and rationale

**Acceptance criteria**:
- Accurately describes each module's responsibility (SC, User Story 3)
- Documents the data flow for `kt install` and `kt upgrade`

---

## Contract 5: Testing Guide (`docs/testing.md`)

**Purpose**: Explain how to run tests, measure coverage, and interpret results.

**Required sections**:
1. `# Testing` — Top-level heading
2. `## Running Tests` — `cargo test` commands
3. `## Measuring Coverage` — `cargo-tarpaulin` or equivalent
4. `## Coverage Thresholds` — >=95% requirement and enforcement
5. `## Investigating Coverage Gaps` — How to identify untested code

**Acceptance criteria**:
- A maintainer can verify >=95% coverage in under 5 minutes (SC-006)
- Documents CI coverage enforcement (User Story 4)

---

## Contract 6: Manifest Format (`docs/manifest.md`)

**Purpose**: Document the `skills.json` manifest format with field descriptions and examples.

**Required sections**:
1. `# Manifest Format` — Top-level heading
2. `## Structure` — JSON schema overview
3. `## Fields` — Description of each field
4. `## Examples` — Complete example manifest

**Acceptance criteria**:
- Documents both `skills` and `exports` top-level keys (Constitution Principle III)
- Includes valid JSON example with 2-space indentation

---

## Contract 7: Lockfile Format (`docs/lockfile.md`)

**Purpose**: Document the `skills.lock` lockfile format with field descriptions and examples.

**Required sections**:
1. `# Lockfile Format` — Top-level heading
2. `## Purpose` — Why lockfiles exist (reproducibility)
3. `## Structure` — JSON schema overview
4. `## Fields` — Description of each field
5. `## Examples` — Complete example lockfile

**Acceptance criteria**:
- Documents commit SHA mapping (Constitution Principle II)
- Explains when lockfile is updated (install, upgrade operations)

---

## Contract 8: CI Documentation Validation

**Purpose**: Document the CI job that validates documentation quality.

**Required content** (in `docs/testing.md` or `docs/contributing.md`):
1. CI checks for broken internal links
2. CI verifies required documentation files exist
3. CI runs on every PR that modifies `docs/` or Markdown files

**Acceptance criteria**:
- Documents the CI validation job (FR-016)
- Explains how to run link checking locally
