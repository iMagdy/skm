# Data Model: Project Documentation

**Date**: 2026-05-30
**Feature**: 002-project-docs

## Entities

### Documentation Set

The collection of all `.md` files in `docs/` covering installation, usage, architecture, contributing, and reference material.

| Field | Type | Description |
|-------|------|-------------|
| directory | path | `docs/` at project root |
| files | set | Collection of Markdown documentation files |
| index | file | `docs/README.md` — table of contents |

**Validation rules**:
- FR-016: CI must verify all required files exist (installation.md, commands.md, contributing.md, architecture.md, testing.md, manifest.md, lockfile.md)
- FR-010: Content must be clear and concise, accessible to intermediate developers

### Documentation File

A single Markdown file in `docs/` covering one topic.

| Field | Type | Description |
|-------|------|-------------|
| filename | string | Matches `^[a-z-]+\.md$` pattern |
| heading | string | Top-level heading (H1) matching the topic |
| content | markdown | GFM Markdown body with code blocks, tables, links |

**Validation rules**:
- No YAML frontmatter (per clarification)
- Links must use relative paths to other `docs/` files
- Code blocks must use language-specific fencing (```rust, ```bash, ```json)

### Table of Contents (docs/README.md)

The index document linking to all documentation sections.

| Field | Type | Description |
|-------|------|-------------|
| title | heading | "Ktesio Documentation" |
| sections | list | Categorized links to documentation files |

**Structure**:
```markdown
# Ktesio Documentation

## Getting Started
- [Installation](installation.md)
- [Quick Start](../specs/002-project-docs/quickstart.md)

## Reference
- [Command Reference](commands.md)
- [Manifest Format](manifest.md)
- [Lockfile Format](lockfile.md)

## Development
- [Contributing](contributing.md)
- [Architecture](architecture.md)
- [Testing](testing.md)
```

### Root README.md

The project entry point that links into `docs/`.

| Field | Type | Description |
|-------|------|-------------|
| overview | paragraph | What Ktesio does (1-2 sentences) |
| quick_start | section | Minimal setup instructions |
| docs_link | link | "For detailed documentation, see [docs/](docs/)" |

**Validation rules**:
- Must list all available commands with current examples (Constitution Principle VII)
- Must link to `docs/` for detailed content
