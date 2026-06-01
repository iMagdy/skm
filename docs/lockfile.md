# Lockfile Format

`skills.lock` records the exact source and commit for installed skills.

## Shape

```json
{
  "docs": {
    "commit": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2",
    "repo": "https://github.com/example/agent-docs.git",
    "skill": "docs"
  }
}
```

## Purpose

- Reproduce installs across machines.
- Show exactly which commit is installed.
- Preserve repo URLs for list/show/upgrade.
- Preserve the exact source published/fallback skill when a multi-skill repo is installed interactively.

## Behavior

- `kt install` creates or updates `skills.lock` only after a repo fetch and content copy succeeds.
- `kt upgrade` updates commits after successful fetch/checkout.
- `kt uninstall` and `kt remove` remove lock entries.
- `kt list` flags entries not present in `skills.json` as `orphaned`.

For locally discovered skills, Ktesio records a zero commit (`0000000000000000000000000000000000000000`) because there is no remote commit to lock.

The optional `skill` field is omitted for installs where the dependency name and source published skill name are the same.

## See Also

- [Manifest format](manifest.md)
- [Command reference](commands.md)
