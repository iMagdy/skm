# CLI Contract: Skill Install Fallback

**Feature**: 003-skill-install-fallback
**Date**: Sat May 30 2026

## Command Interface

The `skm install` command behavior is modified. No new CLI arguments are added.

### Existing Behavior (Unchanged)

```bash
# Install all skills from manifest
skm install

# Install single skill from URL
skm install name:url
```

### New Behavior (Fallback)

When `skills.json` is not found:

```bash
# Triggers fallback discovery
skm install
```

## Output Contract

### stdout

| Condition | Output |
|-----------|--------|
| Manifest found | (existing behavior) |
| Fallback: single skill found | `Installing {name}...` |
| Fallback: skill selected | `Installing {name}...` |
| Fallback: skill installed | `Installed {name}` |

### stderr

| Condition | Output |
|-----------|--------|
| Manifest not found | `Warning: No skills.json found. Auto-discovering skills...` |
| Skills directory empty | `Error: No skills found in the discovered directory` |
| Discovery in progress | `Discovering skills in repository...` |
| Duplicate skipped | `Warning: Duplicate skill '{name}' skipped` |
| User cancelled | `Installation cancelled` |
| Clone failed | `Error cloning {name}: {reason}` |

## Interactive Prompt

When multiple skills discovered:

```
Multiple skills found in repository:

  1. web perf
  2. ui ux pro max
  3. agents sdk

Select skill to install (1-3, or 'q' to cancel):
```

**Input validation**:
- Accepts: numeric (1-N), "q" or "Q" to cancel, empty to cancel
- Rejects: non-numeric, out-of-range numbers (re-prompts)

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (generic) |
| 2 | Manifest not found AND no skills directory |
| 3 | User cancelled |
