# Architecture

Ktesio is a single-binary Rust CLI. It keeps domain logic small and file-based so users can understand and repair project state manually when needed.

## Modules

```text
src/
├── main.rs          # clap command parsing and dispatch
├── cli/             # command handlers
├── discovery.rs     # fallback local skill discovery
├── error.rs         # miette/thiserror diagnostics
├── git.rs           # git CLI wrapper functions
├── install_target.rs # git URL, local path, and GitHub shorthand resolution
├── lockfile.rs      # skills.lock load/save/validation
├── manifest.rs      # skills.json load/save/validation
├── skills_sh.rs     # skills.sh search client, normalization, and retries
├── skill.rs         # copy and remove skill files
└── ui.rs            # shared terminal colors, icons, statuses, and progress bars
```

## Command Flow

### Install

```text
read skills.json
for each dependency:
  clone repo into a temporary workspace with quiet git output and progress updates
  apply rev selector when present
  read source skills.json
  copy only selected published paths into a staged install directory
  if source skills.json is missing:
    ask before discovering directories under skills/, SKILLS/, or .agents/skills/
    copy selected directories into the staged install directory
  move staged content into .agents/skills/<name>/
  record HEAD commit in skills.lock after successful copy
write skills.lock only when entries changed
```

When no manifest is present, `kt install` looks for a local `skills/`, `SKILLS/`, or `.agents/skills/` directory and installs a discovered skill as a fallback.

GitHub shorthand such as `owner/repo` is resolved before cloning. For manifest dependencies, the dependency key is the source repo's published skill name.

### Search

```text
read query and limit
use authenticated skills.sh API when KTESIO_SKILLS_SH_API_KEY exists
otherwise use the public skills.sh search endpoint
retry 429, 503, and transient transport errors up to 3 total attempts
normalize results into GitHub install targets when possible
optionally install the selected result through the normal install flow
```

### Publish

```text
load existing skills.json or create an empty manifest
select local path dependencies or repo-local skill paths
write selected entries to publish
save skills.json
```

### Upgrade

```text
read skills.lock or skills.json
for each skill directory:
  git fetch origin with quiet git output
  resolve default branch
  checkout origin/<default-branch>
  update commit in skills.lock
write skills.lock
```

## Design Choices

- Ktesio shells out to `git` instead of using libgit2 so user SSH keys, credential helpers, proxies, and platform git config work normally.
- Git clone, fetch, and checkout output is captured so users see Ktesio progress bars instead of raw git progress. Failure messages include the useful git summary line.
- The manifest and lockfile are JSON because they are easy to inspect, diff, and repair.
- Partial failures are collected and reported after a command finishes processing remaining skills.
- Tests use local temporary git repositories instead of network fixtures.

## See Also

- [Manifest format](manifest.md)
- [Lockfile format](lockfile.md)
- [Testing](testing.md)
