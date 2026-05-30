# Research: Skill Install Fallback

**Feature**: 003-skill-install-fallback
**Date**: Sat May 30 2026

## Research Questions

### 1. How to detect case-insensitive directory names on cross-platform filesystems?

**Decision**: Normalize directory names to lowercase before searching

**Rationale**: Using `std::path::Path` with `file_name()` and `.to_lowercase()` provides consistent behavior across platforms without relying on filesystem case-sensitivity properties.

**Alternatives considered**:
- Two explicit checks (skills then SKILLS): Rejected - misses mixed case variants like "Skills"
- Case-insensitive comparison function: Rejected - platform-dependent behavior, harder to test

### 2. How to extract skill names from .md files?

**Decision**: Strip `.md` extension, normalize hyphens and underscores to spaces

**Rationale**: Simple string manipulation provides readable names for the selection prompt. Example: `web-perf.md` → "web perf", `ui_ux_pro_max.md` → "ui ux pro max"

**Alternatives considered**:
- Parse first heading from markdown: Rejected - requires markdown parsing, inconsistent across files
- Use filename as-is: Rejected - less readable for users

### 3. How to handle skill discovery with mixed file types?

**Decision**: Collect `.md` files and subdirectories separately, then merge with deduplication by normalized name

**Rationale**: Allows both flat (single .md) and structured (directory with README) skill organization. Deduplication prevents confusion when same skill exists in both forms.

**Alternatives considered**:
- Only support .md files: Rejected - limits flexibility
- Only support directories: Rejected - limits simplicity
- Allow duplicates: Rejected - confusing UX

### 4. What prompt format to use for skill selection?

**Decision**: List numbered options, accept numeric input

**Rationale**: Standard CLI selection pattern, works with existing indicatif/terminal infrastructure.

**Alternatives considered**:
- Arrow key navigation: Rejected - more complex, requires additional dependencies
- Fuzzy matching: Rejected - overkill for typical use cases

## Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| std::fs | Built-in | Directory traversal, file reading |
| regex | 1.x | Name validation (existing) |
| indicatif | 0.17 | Progress bars (existing) |
| dialoguer | 0.11 | Interactive prompts (NEW) |

**Note**: Adding `dialoguer` dependency for interactive selection prompt. This is a well-maintained crate commonly used for CLI prompts.
