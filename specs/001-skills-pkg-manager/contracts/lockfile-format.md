# Data Format Contract: skills.lock

**Feature**: 001-skills-pkg-manager
**Date**: 2026-05-30

## Schema

```json
{
  "<skill-name>": {
    "commit": "<full-sha-hash>",
    "repo": "<git-clone-url>"
  }
}
```

## Field Specifications

### Top-level keys

Skill names — must match corresponding keys in `skills.json` `skills` object.

### `commit` (string, required)

Full SHA-1 git commit hash. Must be exactly 40 hex characters (`^[0-9a-f]{40}$`).

### `repo` (string, required)

Git clone URL. Must match the `repo` value from `skills.json` for the same skill.

## Example

```json
{
  "clap": {
    "commit": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2",
    "repo": "https://github.com/clap-rs/clap.git"
  },
  "miette": {
    "commit": "f6e5d4c3b2a1f6e5d4c3b2a1f6e5d4c3b2a1f6e5",
    "repo": "git@github.com:zkat/miette.git"
  }
}
```

## Validation Rules

1. File MUST be valid JSON
2. Top-level MUST be an object
3. All `commit` values MUST be exactly 40 hex characters
4. All `repo` values MUST be non-empty strings
5. Skill names SHOULD correspond to entries in `skills.json` (mismatches produce warnings)
