# Data Format Contract: skills.json

**Feature**: 001-skills-pkg-manager
**Date**: 2026-05-30

## Schema

```json
{
  "skills": {
    "<skill-name>": {
      "repo": "<git-clone-url>"
    }
  },
  "exports": {
    "<skill-name>": {
      "path": "<relative-directory-path>"
    }
  }
}
```

## Field Specifications

### `skills` (object, required)

Map of skill names to their import configuration.

- **Key**: Skill name — must match `^[a-zA-Z0-9_-]+$`, non-empty, unique
- **Value**: Object with required field `repo`

#### `repo` (string, required)

Git clone URL. Must be one of:
- HTTPS: `https://github.com/org/repo.git`
- SSH: `git@github.com:org/repo.git`
- SSH explicit: `ssh://git@github.com/org/repo.git`
- Local path: `/absolute/path/to/repo.git` or `../relative/repo.git`

### `exports` (object, required)

Map of exported skill names to their local directory paths.

- **Key**: Skill name — same validation as `skills` keys
- **Value**: Object with required field `path`

#### `path` (string, required)

Relative path from project root to the directory containing skill files.

- Must point to an existing directory
- Relative to the project root (where `skills.json` lives)

## Example

```json
{
  "skills": {
    "clap": {
      "repo": "https://github.com/clap-rs/clap.git"
    },
    "miette": {
      "repo": "git@github.com:zkat/miette.git"
    }
  },
  "exports": {
    "my-skill": {
      "path": "./skills/my-skill"
    }
  }
}
```

## Validation Rules

1. File MUST be valid JSON
2. Top-level MUST be an object
3. `skills` key MUST exist (empty object `{}` if no skills)
4. `exports` key MUST exist (empty object `{}` if no exports)
5. No duplicate skill names (case-sensitive)
6. All `repo` values MUST be non-empty strings
7. All `path` values MUST be non-empty strings pointing to existing directories
