# Quickstart: Skill Install Fallback

**Feature**: 003-skill-install-fallback
**Date**: Sat May 30 2026

## Overview

This feature adds fallback discovery to `skm install`, allowing skill installation from repositories that don't have a `skills.json` manifest file.

## How It Works

When you run `skm install` and no `skills.json` is found:

1. **Warning displayed**: `Warning: No skills.json found. Auto-discovering skills...`
2. **Directory search**: System searches for `skills/` directory (case-insensitive)
3. **Skill discovery**: Scans for `.md` files and subdirectories
4. **Selection prompt** (if multiple found): Presents numbered list
5. **Installation**: Selected skill is installed

## Usage Examples

### Install from repo with manifest (unchanged)

```bash
cd my-project
skm install
# Reads skills.json, installs all listed skills
```

### Install from repo without manifest (NEW)

```bash
cd repo-without-manifest
skm install

# Output:
# Warning: No skills.json found. Auto-discovering skills...
# Discovering skills in repository...
# 
# Multiple skills found:
#   1. web perf
#   2. ui ux pro max
#   3. agents sdk
#
# Select skill to install (1-3, or 'q' to cancel): 2
# Installing ui ux pro max...
# Installed ui ux pro max
```

### Single skill auto-selected

```bash
cd repo-with-one-skill
skm install

# Output:
# Warning: No skills.json found. Auto-discovering skills...
# Discovering skills in repository...
# Installing web perf...
# Installed web perf
```

## Error Cases

### Empty skills directory

```bash
cd repo-with-empty-skills
skm install

# Output:
# Warning: No skills.json found. Auto-discovering skills...
# Error: No skills found in the discovered directory
```

### User cancels selection

```bash
# When prompted for selection, enter 'q' or press Enter:
Select skill to install (1-3, or 'q' to cancel): q
Installation cancelled
```

## Notes

- Skill names are extracted from filenames: `web-perf.md` → "web perf"
- Duplicate names are automatically removed (first occurrence kept)
- The `skills/` directory is searched case-insensitively (finds `SKILLS`, `Skills`, etc.)
