# CLI Contract: Ktesio

**Feature**: 001-skills-pkg-manager
**Date**: 2026-05-30

## Binary

```
kt [COMMAND]
```

Global flags:
- `--help` / `-h`: Show help
- `--version` / `-V`: Show version

Exit codes:
- `0`: Success
- `1`: General error (manifest not found, git error, etc.)
- `2`: Usage error (invalid arguments)

---

## Commands

### `kt init <path>`

Initialize a new `skills.json` manifest at the given path.

**Arguments**:
| Arg | Required | Description |
|-----|----------|-------------|
| `path` | Yes | Directory path where `skills.json` will be created (typically `.`) |

**Behavior**:
- Creates `skills.json` with `{ "skills": {}, "exports": {} }`
- If `skills.json` already exists: prints warning, exits `0`
- If path doesn't exist: prints error, exits `1`

**Stdout**: `Created skills.json at <path>`
**Stderr** (warning): `skills.json already exists at <path>, skipping`
**Stderr** (error): `Error: path <path> does not exist`

---

### `kt install`

Install all skills declared in `skills.json`.

**Arguments**: None

**Behavior**:
- Reads `skills.json` from current directory
- For each skill in `skills`:
  1. Clone repo into `.agents/skills/<name>/`
  2. Read `skills.json` from cloned repo for exports
  3. Copy exported files/dirs into `.agents/skills/<name>/`
  4. Record commit SHA in `skills.lock`
- Skips already-installed skills (directory exists + lock entry exists)
- Continues on individual skill failure, reports errors at end

**Stdout**: Progress indicators per skill
**Stderr** (error per skill): `Error cloning <name>: <reason>`
**Stderr** (missing manifest): `Error: no skills.json found in current directory`

---

### `kt install <name:url>`

Install a single skill by name and git URL.

**Arguments**:
| Arg | Required | Description |
|-----|----------|-------------|
| `name:url` | Yes | Skill name and repo URL separated by `:` (e.g., `clap:https://github.com/clap-rs/clap.git`) |

**Behavior**:
- Parses `name` and `url` from argument
- Creates `skills.json` if it doesn't exist (with empty `skills` and `exports`)
- If skill name already exists in manifest: warns and exits `0`
- Adds skill to `skills.json`, clones, copies exports, updates lockfile

**Stdout**: Progress indicator, `Installed <name> from <url>`
**Stderr** (warning): `Skill <name> already exists in manifest, skipping`
**Stderr** (error): `Error: invalid format, expected name:url`

---

### `kt upgrade`

Upgrade all installed skills to latest commit on default branch.

**Arguments**: None

**Behavior**:
- Reads `skills.lock` for installed skills
- For each skill: `git fetch origin && git checkout origin/<default-branch>`
- Resolves new HEAD commit SHA
- Updates `skills.lock` with new commit hash
- If lockfile missing: creates entries for all skills in manifest
- Continues on individual failure, reports errors at end

**Stdout**: Progress indicators per skill
**Stderr** (error per skill): `Error upgrading <name>: <reason>`

---

### `kt list`

List all installed skills.

**Arguments**: None

**Behavior**:
- Reads `skills.json` and `skills.lock`
- Checks `.agents/skills/` for each skill
- Displays table with columns: NAME, REPO, COMMIT, STATUS

**Stdout**:
```
NAME    REPO                                    COMMIT       STATUS
clap    https://github.com/clap-rs/clap.git    abc123def    installed
miette  https://github.com/zkat/miette.git     789abc012    missing
```

STATUS values: `installed`, `missing` (lock entry exists but dir missing)

**Stdout** (no skills): `No skills installed. Run 'kt install' to add skills.`

---

### `kt show <package_name>`

Show details for a specific skill.

**Arguments**:
| Arg | Required | Description |
|-----|----------|-------------|
| `package_name` | Yes | Name of the skill to inspect |

**Behavior**:
- Looks up skill in `skills.lock`
- Checks `.agents/skills/<name>/` exists
- Displays: name, repo URL, locked commit, local path, status

**Stdout**:
```
Name:    clap
Repo:    https://github.com/clap-rs/clap.git
Commit:  abc123def456...
Path:    .agents/skills/clap/
Status:  installed
```

**Stderr** (not found): `Error: skill '<name>' not found`

---

### `kt uninstall <package_name>` / `kt remove <package_name>`

Remove a skill from the project.

**Arguments**:
| Arg | Required | Description |
|-----|----------|-------------|
| `package_name` | Yes | Name of the skill to remove |

**Behavior**:
- Removes skill from `skills.json` `skills` object
- Removes entry from `skills.lock`
- Deletes `.agents/skills/<name>/` directory (force, no confirmation)
- If skill not in manifest: prints error, exits `1`

**Stdout**: `Uninstalled <name>`
**Stderr** (not found): `Error: skill '<name>' not found in manifest`
