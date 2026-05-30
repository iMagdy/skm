# Architecture

This guide describes the codebase structure, module responsibilities, and key data flows in skm.

## Module Overview

```text
src/
├── main.rs          # Entry point — CLI parsing with clap
├── cli/             # Command implementations
│   ├── mod.rs       # Module declarations
│   ├── init.rs      # skm init — create manifest
│   ├── install.rs   # skm install — clone skills
│   ├── list.rs      # skm list — display installed skills
│   ├── show.rs      # skm show — inspect a skill
│   ├── uninstall.rs # skm uninstall — remove a skill
│   └── upgrade.rs   # skm upgrade — update skills
├── error.rs         # Error types (thiserror)
├── git.rs           # Git operations (shells out to git CLI)
├── lockfile.rs      # Lockfile read/write (serde_json)
├── manifest.rs      # Manifest read/write (serde_json)
└── skill.rs         # Skill data types
```

## Module Responsibilities

### main.rs

- Parses CLI arguments using `clap` derive macros
- Dispatches to the appropriate command handler
- Handles top-level errors

### cli/

Each module implements a single CLI command:

- **init.rs**: Creates `skills.json` with empty structure
- **install.rs**: Reads manifest, clones repos, copies exports, updates lockfile
- **upgrade.rs**: Fetches latest commits, updates skill directories and lockfile
- **list.rs**: Reads lockfile, displays installed skills in a table
- **show.rs**: Displays details for a specific skill
- **uninstall.rs**: Removes skill from manifest, disk, and lockfile

### error.rs

- Defines error types using `thiserror`
- Uses `miette` for diagnostic output with context

### git.rs

- Shells out to the `git` CLI for all git operations
- Supports both HTTPS and SSH clone URLs
- Handles authentication failures with clear error messages

### lockfile.rs

- Reads and writes `skills.lock`
- Maps skill names to commit SHAs

### manifest.rs

- Reads and writes `skills.json`
- Validates skill name format and uniqueness

### skill.rs

- Defines the `Skill` data type
- Contains shared constants and utilities

## Data Flow: skm install

```text
1. Parse CLI arguments
2. Read skills.json (manifest.rs)
3. For each skill in manifest:
   a. Check if already installed → skip if yes
   b. Clone repo to .agents/skills/<name>/ (git.rs)
   c. Read source repo's skills.json exports
   d. Copy exported files/dirs to .agents/skills/<name>/
   e. Record commit SHA
4. Write/update skills.lock (lockfile.rs)
5. Report results (successes and failures)
```

## Data Flow: skm upgrade

```text
1. Parse CLI arguments
2. Read skills.lock (lockfile.rs)
3. For each skill in lockfile:
   a. Fetch latest commits from default branch (git.rs)
   b. Update skill directory to latest commit
   c. Record new commit SHA
4. Write updated skills.lock (lockfile.rs)
5. Report results (successes and failures)
```

## Design Decisions

### Shell out to git CLI

skm shells out to the `git` CLI rather than using libgit2. This ensures compatibility with the user's git configuration, including authentication, proxies, and SSH keys.

### Graceful Degradation

Partial failures do not abort the entire operation. If one skill fails to clone or upgrade, skm continues with remaining skills and reports all errors at the end. This behavior is implemented in the install and upgrade command handlers.

**Error handling patterns:**
- All error messages include the specific failure reason
- All error messages include a suggested remediation action
- Malformed JSON in `skills.json` is detected and reported with line/column info
- Duplicate skill names are detected and rejected
- Missing manifests are reported with a clear error message

### Manifest as Single Source of Truth

`skills.json` is the authoritative source for which skills a project uses. The lockfile is derived from the manifest and git operations.

## See Also

- [Command Reference](commands.md) — Detailed command documentation
- [Manifest Format](manifest.md) — `skills.json` structure
- [Lockfile Format](lockfile.md) — `skills.lock` structure
