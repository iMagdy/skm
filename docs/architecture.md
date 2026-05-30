# Architecture

`skm` is a single-binary Rust CLI. It keeps domain logic small and file-based so users can understand and repair project state manually when needed.

## Modules

```text
src/
├── main.rs          # clap command parsing and dispatch
├── cli/             # command handlers
├── discovery.rs     # fallback local skill discovery
├── error.rs         # miette/thiserror diagnostics
├── git.rs           # git CLI wrapper functions
├── lockfile.rs      # skills.lock load/save/validation
├── manifest.rs      # skills.json load/save/validation
└── skill.rs         # copy and remove skill files
```

## Command Flow

### Install

```text
read skills.json
for each skill:
  clone repo into .agents/skills/<name>/
  read source skills.json if present
  copy exported paths when exports exist
  record HEAD commit in skills.lock
write skills.lock
```

When no manifest is present, `skm install` looks for a local `skills/` directory and installs a discovered skill as a fallback.

### Export

```text
load existing skills.json or create an empty manifest
read skills.lock
add locked skills back into skills.json
scan .agents/skills/ for untracked local directories
save skills.json
```

### Upgrade

```text
read skills.lock or skills.json
for each skill directory:
  git fetch origin
  resolve default branch
  checkout origin/<default-branch>
  update commit in skills.lock
write skills.lock
```

## Design Choices

- `skm` shells out to `git` instead of using libgit2 so user SSH keys, credential helpers, proxies, and platform git config work normally.
- The manifest and lockfile are JSON because they are easy to inspect, diff, and repair.
- Partial failures are collected and reported after a command finishes processing remaining skills.
- Tests use local temporary git repositories instead of network fixtures.

## See Also

- [Manifest format](manifest.md)
- [Lockfile format](lockfile.md)
- [Testing](testing.md)
