# Lockfile Format

The `skills.lock` file maps each installed skill to its exact commit SHA for reproducible installations.

## Purpose

- Ensures reproducible installations across environments
- Records the exact version of each installed skill
- Updated on every `install` and `upgrade` operation

## Structure

```json
{
  "<skill-name>": {
    "commit": "<sha>",
    "repo": "<url>"
  }
}
```

## Fields

| Field | Type | Description |
|-------|------|-------------|
| `<skill-name>` | string | Skill name matching the manifest |
| `commit` | string | Full commit SHA of the installed version |
| `repo` | string | Git clone URL (from manifest) |

## Examples

### Minimal Lockfile

```json
{}
```

### Lockfile with Installed Skills

```json
{
  "my-skill": {
    "commit": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2",
    "repo": "https://github.com/example/my-skill.git"
  },
  "another-skill": {
    "commit": "f6e5d4c3b2a1f6e5d4c3b2a1f6e5d4c3b2a1f6e5",
    "repo": "git@github.com:example/another-skill.git"
  }
}
```

## Behavior

- Created by `skm install` when skills are first installed
- Updated by `skm install` and `skm upgrade` operations
- Entry removed by `skm uninstall` when a skill is removed
- `skm list` reads the lockfile to display installed skill versions

## Stale Entries

If `skills.lock` contains skills not in `skills.json`, `skm list` flags them as `orphaned`. If a skill directory exists in `.agents/skills/` but has no lockfile entry, it is flagged as `untracked`.

## See Also

- [Manifest Format](manifest.md) — How skills are declared
- [Command Reference](commands.md) — `skm install`, `skm upgrade`, `skm uninstall` commands
