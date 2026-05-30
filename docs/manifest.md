# Manifest Format

`skills.json` is the project manifest. It declares imported skills and local exports.

## Shape

```json
{
  "skills": {
    "docs": {
      "repo": "https://github.com/example/agent-docs.git"
    }
  },
  "exports": {
    "local-docs": {
      "path": "skills/local-docs"
    }
  }
}
```

## Fields

| Field | Type | Required | Meaning |
|-------|------|----------|---------|
| `skills` | object | yes | Skills this project imports |
| `exports` | object | yes | Local files or directories this repo exposes to other projects |
| `skills.<name>.repo` | string | yes | Git clone URL or local git path |
| `exports.<name>.path` | string | yes | Path inside this repo to copy when installed elsewhere |

Skill names must match:

```text
^[a-zA-Z0-9_-]+$
```

## Minimal Manifest

```json
{
  "skills": {},
  "exports": {}
}
```

## Import Example

```json
{
  "skills": {
    "docs": {
      "repo": "https://github.com/example/agent-docs.git"
    },
    "review": {
      "repo": "git@github.com:example/review-skill.git"
    }
  },
  "exports": {}
}
```

## Export Example

```json
{
  "skills": {},
  "exports": {
    "my-skill": {
      "path": "skills/my-skill"
    }
  }
}
```

When a repo with exports is installed, `skm` copies only the exported paths into the destination skill directory. Other repository files, including `.git`, docs, fixtures, and unrelated source files, are not installed.

If an installed repo has no `skills.json`, `skm` asks before falling back to directories under `skills/` or `SKILLS/`. Repos with a `skills.json` but no `exports` are not installable by fallback.

## See Also

- [Command reference](commands.md)
- [Lockfile format](lockfile.md)
