# Command Reference

All commands support `--help`. `kt --version` prints the package version.

## `kt init <path>`

Create a `skills.json` manifest in a directory.

```bash
kt init .
```

Behavior:

- Creates `skills.json` with empty `skills` and `exports` objects.
- Leaves an existing manifest untouched.
- Fails if the target directory does not exist.

## `kt install`

Install every skill declared in `skills.json`. If `skills` is omitted, it is treated as empty.

```bash
kt install
```

Behavior:

- Fetches each manifest entry into a temporary workspace.
- Shows a progress bar while cloning each repo and copying exported content.
- Suppresses raw git clone progress; clone failures are summarized after the skill finishes.
- Installs only paths declared in the source repo's `skills.json` `exports`.
- If the source repo has no `skills.json`, asks before discovering directories under `skills/` or `SKILLS/`; multiple directories can be selected.
- Records resolved commits in `skills.lock` only for successful installs.
- Continues after individual clone/copy failures and reports all errors.

## `kt install <name:repo>`

Add one skill to `skills.json` and install it.

```bash
kt install docs:https://github.com/example/agent-docs.git
```

The `repo` value can be an HTTPS URL, SSH URL, or local git path.

Ktesio updates `skills.json` and `skills.lock` only after the repo is fetched and installable content is copied successfully. A bad target, failed clone, missing exports, cancelled fallback, or missing fallback `skills/` directory leaves those files unchanged.

The single-skill install flow uses the same progress bar and quiet git output as bulk install.

## `kt install <repo>`

Install one or more exports from a source repository without naming the package first.

```bash
kt install https://github.com/example/agent-docs.git
kt install --all https://github.com/example/agent-docs.git
```

Behavior:

- Reads the source repo's `skills.json` `exports` and derives destination names from export names.
- Prompts when multiple exports are available.
- `--all` installs every export without prompting.
- `--yes` accepts safe defaults, such as a single obvious fallback skill.
- `--no-input` fails instead of prompting when a choice is required.
- Repos without `skills.json` can use fallback discovery from `.md` files or directories under `skills/` or `SKILLS/`.

## `kt export`

Rebuild `skills.json` from installed skills.

```bash
kt export
```

Behavior:

- Reads `skills.lock` first.
- Adds untracked directories under `.agents/skills/` using their local paths.
- Preserves existing `exports`.
- Creates an empty manifest if no skills are installed.

## `kt export add <name> <path>`

Add or update one local export in `skills.json`.

```bash
kt export add docs skills/docs
```

Behavior:

- Creates `skills.json` if needed.
- Preserves existing imported skills and exports.
- Fails if the name is invalid, the path does not exist, or the path is outside the project.

## `kt upgrade`

Fetch latest commits for installed skills.

```bash
kt upgrade
```

Behavior:

- Uses `skills.lock` when present, otherwise falls back to `skills.json`.
- Runs git fetch and checks out the resolved default branch behind a progress bar.
- Updates `skills.lock` for successful upgrades.
- Reports per-skill errors without stopping the whole command.
- Suppresses raw git fetch and checkout output unless a failure needs a short git error summary.

## `kt list`

Show all known skills.

```bash
kt list
```

Statuses:

- `installed`: manifest or lockfile entry exists and files are present.
- `missing`: lockfile entry exists but files are missing.
- `not locked`: manifest entry exists but no lock entry exists.
- `orphaned`: lockfile entry exists but no manifest entry exists.

The table uses icons and color-coded status labels when the terminal supports them.

Use `kt list --json` for machine-readable output. JSON entries include `name`, `repo`, `commit`, `path`, and `status`.

## `kt show <name>`

Show one skill.

```bash
kt show docs
```

Output includes name, repo, commit, local path, and status.

Status is color-coded the same way as `kt list`.

Use `kt show <name> --json` for machine-readable output with `name`, `repo`, `commit`, `path`, and `status`.

## `kt doctor`

Validate project skill state.

```bash
kt doctor
```

Behavior:

- Checks `skills.json`, `skills.lock`, `.agents/skills/`, local export paths, orphaned lock entries, missing installed directories, and git availability.
- Exits successfully when no errors are found.
- Prints actionable warnings and errors for repairable project state.

## `kt uninstall <name>`

Remove a skill from manifest, lockfile, and disk.

```bash
kt uninstall docs
```

## `kt remove <name>`

Alias for `kt uninstall <name>`.

```bash
kt remove docs
```
