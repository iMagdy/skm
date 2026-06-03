---
title: Manifest Format
description: The skills.json shape for declaring installed skill dependencies and published local skills.
---

# Manifest Format

`skills.json` is the project manifest. It declares skill dependencies this project uses and local skills this repo publishes for others.

## Shape

```json
{
  "dependencies": {
    "docs": {
      "repo": "https://github.com/example/agent-docs.git",
      "rev": "branch:main"
    },
    "local-docs": {
      "path": ".agents/skills/local-docs"
    }
  },
  "publish": [
    "local-docs",
    {
      "skill": "extra-docs",
      "path": "skills/extra-docs",
      "deprecated": true
    }
  ]
}
```

## Fields

| Field | Type | Required | Meaning |
|-------|------|----------|---------|
| `dependencies` | object | no | Skills this project uses; defaults to `{}` |
| `dependencies.<name>.repo` | string | yes, for remote deps | Git clone URL, local git path, or supported shorthand |
| `dependencies.<name>.path` | string | yes, for local deps | Local path to a skill used by this project |
| `dependencies.<name>.rev` | string | no | Source selector: `commit:<sha>`, `branch:<name>`, or `tag:<name>` |
| `publish` | array | no | Local skills this repo exposes to other projects; defaults to `[]` |
| `publish[]` string | string | no | Publish a local path dependency by name |
| `publish[].skill` | string | yes, for object entries | Published skill name |
| `publish[].path` | string | yes, for object entries | Path inside this repo to copy when installed elsewhere |
| `publish[].deprecated` | bool | no | Warn whenever this published skill is installed |

Skill names must match:

```text
^[a-zA-Z0-9_-]+$
```

Each dependency must declare exactly one of `repo` or `path`.

## Minimal Manifest

```json
{}
```

`kt init` writes both top-level keys for readability:

```json
{
  "dependencies": {},
  "publish": []
}
```

When `.agents/skills/` already contains installed skills, `kt init` adopts those directories into `dependencies`. Known public skills are recorded as remote dependencies when they can be resolved through an existing lock entry or an exact skills.sh match; unmatched custom skills are recorded as local path dependencies. Adopted skills are not added to `publish` automatically.

## Dependency Example

```json
{
  "dependencies": {
    "docs": {
      "repo": "https://github.com/example/agent-docs.git",
      "rev": "tag:v1.2.0"
    },
    "review": {
      "repo": "git@github.com:example/review-skill.git"
    },
    "local-docs": {
      "path": ".agents/skills/local-docs"
    }
  },
  "publish": []
}
```

For remote dependencies, the dependency key is the published skill name in the source repo. `skills.lock` records the resolved commit after install.

## Publish Example

```json
{
  "dependencies": {
    "my-skill": {
      "path": ".agents/skills/my-skill"
    }
  },
  "publish": [
    "my-skill",
    {
      "skill": "extra-skill",
      "path": "skills/extra-skill"
    }
  ]
}
```

String publish entries reference local path dependencies. Object publish entries can expose any repo-local file or directory. When a repo with `publish` entries is installed, Ktesio copies only the selected published path into the destination skill directory.

If a source repo has no `skills.json`, Ktesio asks before falling back to directories under `skills/`, `SKILLS/`, or `.agents/skills/`. Repos with a `skills.json` but no `publish` entries are not installable by fallback.

## See Also

- [Command reference](commands.md)
- [Lockfile format](lockfile.md)
