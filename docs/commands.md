# Command Reference

## skm init

Create a new skills manifest (`skills.json`) in the specified directory.

**Usage**: `skm init <path>`

**Options**:
- `--help` — Show help message
- `--version` — Show version

**Examples**:

```bash
# Initialize in current directory
skm init .

# Initialize in a specific directory
skm init ./my-project
```

**Behavior**:
- Creates `skills.json` with empty `skills` and `exports` objects
- Warns if `skills.json` already exists (does not overwrite)

---

## skm install

Install skills declared in the manifest.

**Usage**: `skm install [name:repo_url]`

**Options**:
- `--help` — Show help message
- `--version` — Show version

**Examples**:

```bash
# Install all skills from manifest
skm install

# Install a specific skill
skm install myskill:https://github.com/example/repo.git
```

**Behavior**:
- Reads `skills.json` and clones each skill into `.agents/skills/<name>/`
- Creates or updates `skills.lock` with commit SHAs
- Skips already-installed skills
- Reports errors for individual skills without aborting

---

## skm upgrade

Upgrade all installed skills to their latest versions.

**Usage**: `skm upgrade`

**Options**:
- `--help` — Show help message
- `--version` — Show version

**Examples**:

```bash
# Upgrade all skills
skm upgrade
```

**Behavior**:
- Fetches latest commits from each skill's default branch
- Updates skill directories and lockfile
- Reports errors for individual skills without aborting

---

## skm list

List all installed skills with their source and locked version.

**Usage**: `skm list`

**Options**:
- `--help` — Show help message
- `--version` — Show version

**Examples**:

```bash
# List installed skills
skm list
```

**Behavior**:
- Displays a table of skill names, source repos, and locked commit SHAs
- Flags stale or missing skills

---

## skm show

Show detailed information about a specific skill.

**Usage**: `skm show <package_name>`

**Options**:
- `--help` — Show help message
- `--version` — Show version

**Examples**:

```bash
# Show details for a specific skill
skm show myskill
```

**Behavior**:
- Displays repo URL, locked commit ID, and local path
- Reports error if skill not found

---

## skm uninstall / skm remove

Remove a skill from the manifest, lockfile, and disk.

**Usage**: `skm uninstall <package_name>` or `skm remove <package_name>`

**Options**:
- `--help` — Show help message
- `--version` — Show version

**Examples**:

```bash
# Uninstall a skill
skm uninstall myskill

# Or using remove
skm remove myskill
```

**Behavior**:
- Removes skill from `skills.json`
- Deletes skill directory from `.agents/skills/`
- Removes entry from `skills.lock`
- Reports error if skill not found

---

## Global Flags

All commands support:

- `--help` — Show help message
- `--version` — Show version
