# Command Reference

All commands support `--help`. `skm --version` prints the package version.

## `skm init <path>`

Create a `skills.json` manifest in a directory.

```bash
skm init .
```

Behavior:

- Creates `skills.json` with empty `skills` and `exports` objects.
- Leaves an existing manifest untouched.
- Fails if the target directory does not exist.

## `skm install`

Install every skill declared in `skills.json`. If `skills` is omitted, it is treated as empty.

```bash
skm install
```

Behavior:

- Fetches each manifest entry into a temporary workspace.
- Shows a progress bar while cloning each repo and copying exported content.
- Suppresses raw git clone progress; clone failures are summarized after the skill finishes.
- Installs only paths declared in the source repo's `skills.json` `exports`.
- If the source repo has no `skills.json`, asks before discovering directories under `skills/` or `SKILLS/`; multiple directories can be selected.
- Records resolved commits in `skills.lock` only for successful installs.
- Continues after individual clone/copy failures and reports all errors.

## `skm install <name:repo>`

Add one skill to `skills.json` and install it.

```bash
skm install docs:https://github.com/example/agent-docs.git
```

The `repo` value can be an HTTPS URL, SSH URL, or local git path.

`skm` updates `skills.json` and `skills.lock` only after the repo is fetched and installable content is copied successfully. A bad target, failed clone, missing exports, cancelled fallback, or missing fallback `skills/` directory leaves those files unchanged.

The single-skill install flow uses the same progress bar and quiet git output as bulk install.

## `skm export`

Rebuild `skills.json` from installed skills.

```bash
skm export
```

Behavior:

- Reads `skills.lock` first.
- Adds untracked directories under `.agents/skills/` using their local paths.
- Preserves existing `exports`.
- Creates an empty manifest if no skills are installed.

## `skm upgrade`

Fetch latest commits for installed skills.

```bash
skm upgrade
```

Behavior:

- Uses `skills.lock` when present, otherwise falls back to `skills.json`.
- Runs git fetch and checks out the resolved default branch behind a progress bar.
- Updates `skills.lock` for successful upgrades.
- Reports per-skill errors without stopping the whole command.
- Suppresses raw git fetch and checkout output unless a failure needs a short git error summary.

## `skm list`

Show all known skills.

```bash
skm list
```

Statuses:

- `installed`: manifest or lockfile entry exists and files are present.
- `missing`: lockfile entry exists but files are missing.
- `not locked`: manifest entry exists but no lock entry exists.
- `orphaned`: lockfile entry exists but no manifest entry exists.

The table uses icons and color-coded status labels when the terminal supports them.

## `skm show <name>`

Show one skill.

```bash
skm show docs
```

Output includes name, repo, commit, local path, and status.

Status is color-coded the same way as `skm list`.

## `skm uninstall <name>`

Remove a skill from manifest, lockfile, and disk.

```bash
skm uninstall docs
```

## `skm remove <name>`

Alias for `skm uninstall <name>`.

```bash
skm remove docs
```
