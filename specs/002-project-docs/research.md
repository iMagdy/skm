# Research: Project Documentation

**Date**: 2026-05-30
**Feature**: 002-project-docs

## Research Items

### 1. Documentation Structure Best Practices

**Decision**: Single `docs/` directory with flat file structure (no subdirectories).

**Rationale**: The skm project has ~8 CLI commands and a straightforward architecture. A flat structure with topic-specific files (installation.md, commands.md, etc.) is easier to navigate than nested directories for this scale. Each file is self-contained with cross-references via relative links.

**Alternatives considered**:
- Nested directories per audience (user/, contributor/, maintainer/): Rejected — adds navigation overhead for a small doc set.
- Single large README: Rejected — violates the spec's requirement for multiple documentation files.

### 2. Markdown Conventions

**Decision**: GitHub-Flavored Markdown (GFM) with no YAML frontmatter.

**Rationale**: GFM is the most widely supported Markdown variant for open-source projects. No frontmatter keeps files simple and reduces contributor friction. Headings, code blocks, tables, and links are sufficient for all documentation needs.

**Alternatives considered**:
- YAML frontmatter for title/description: Rejected per clarification — adds complexity without value for this project.
- CommonMark strict: Rejected — GFM extensions (tables, task lists) are useful.

### 3. CI Link Validation

**Decision**: Document that CI checks for broken internal links and required file presence.

**Rationale**: The spec requires broken link detection (FR-016) and the edge cases section mandates it. A simple CI job using a link checker tool (e.g., markdown-link-check, lychee) validates internal links at build time.

**Alternatives considered**:
- Manual-only link checking: Rejected — inconsistent and error-prone.
- Full site generation (mdBook, Docusaurus): Rejected — overkill for plain Markdown files.

### 4. Root README Update Strategy

**Decision**: Root README serves as a concise entry point that links to `docs/` sections.

**Rationale**: Per clarification — root README stays skimmable (project overview, quick start, link to docs/), while `docs/` holds detailed content. This is the standard open-source pattern.

**Alternatives considered**:
- Duplicated content: Rejected — creates maintenance burden and drift risk.
- Minimal README with only badges: Rejected — users need a quick overview before diving into docs/.

### 5. Documentation Currency Enforcement

**Decision**: Constitution Principle VII requires docs to be updated in the same changeset as code changes. This is enforced via PR review checklist, not automation.

**Rationale**: Automated enforcement of "docs updated when code changes" is complex and fragile. The constitution already mandates this behavior; the docs/contributing.md file documents the PR review requirement.

**Alternatives considered**:
- CI check that diffs docs/ on every PR: Rejected — too noisy for docs-only changes, misses edge cases.
- Git hook that blocks commits without doc changes: Rejected — disrupts workflow for non-code changes.

## Open Questions

None — all technical decisions resolved.
