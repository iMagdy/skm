# Lockfile Format

`skills.lock` records the exact source and commit for installed skills.

## Shape

```json
{
  "docs": {
    "commit": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2",
    "repo": "https://github.com/example/agent-docs.git"
  }
}
```

## Purpose

- Reproduce installs across machines.
- Show exactly which commit is installed.
- Preserve repo URLs for `skm export`.

## Behavior

- `skm install` creates or updates `skills.lock`.
- `skm upgrade` updates commits after successful fetch/checkout.
- `skm uninstall` and `skm remove` remove lock entries.
- `skm list` flags entries not present in `skills.json` as `orphaned`.

For locally discovered skills, `skm` records a zero commit (`0000000000000000000000000000000000000000`) because there is no remote commit to lock.

## See Also

- [Manifest format](manifest.md)
- [Command reference](commands.md)
