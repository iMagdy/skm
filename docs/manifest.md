# Manifest Format

The `skills.json` file is the single source of truth for which skills a project imports and exports.

## Structure

```json
{
  "skills": {
    "<skill-name>": {
      "repo": "<git-clone-url>"
    }
  },
  "exports": {
    "<skill-name>": {
      "path": "<local-directory>"
    }
  }
}
```

## Fields

### Root Level

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `skills` | object | Yes | Skills to import/install from git repositories |
| `exports` | object | Yes | Local directories this project exports as skills |

### skills.\*

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `<skill-name>` | string | Yes | Unique skill name (must match `^[a-zA-Z0-9_-]+$`) |
| `repo` | string | Yes | Git clone URL (HTTPS or SSH) |

### exports.\*

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `<skill-name>` | string | Yes | Unique skill name |
| `path` | string | Yes | Local directory path to export |

## Examples

### Minimal Manifest

```json
{
  "skills": {},
  "exports": {}
}
```

### Manifest with Imports

```json
{
  "skills": {
    "my-skill": {
      "repo": "https://github.com/example/my-skill.git"
    },
    "another-skill": {
      "repo": "git@github.com:example/another-skill.git"
    }
  },
  "exports": {}
}
```

### Manifest with Exports

```json
{
  "skills": {
    "external-skill": {
      "repo": "https://github.com/example/external-skill.git"
    }
  },
  "exports": {
    "my-local-skill": {
      "path": "./skills/my-local-skill"
    }
  }
}
```

## Validation Rules

- Skill names must match `^[a-zA-Z0-9_-]+$`
- Skill names must be unique within the manifest
- `skills` and `exports` must both be present as top-level keys
- JSON must be valid with 2-space indentation

## See Also

- [Lockfile Format](lockfile.md) — How installed skills are tracked
- [Command Reference](commands.md) — `skm init` and `skm install` commands
