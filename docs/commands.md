---
title: Command Reference
description: Every kt command, its expected arguments, and the workflows each command supports.
---

# Command Reference

All commands support `--help`. `kt --version` prints the package version.

## Update Checks

When a `kt` subcommand runs, Ktesio checks whether a newer GitHub Release is
available. The check uses an hourly cache, so repeated commands only contact
GitHub after the cached result is at least one hour old.

If an update is available, Ktesio prints a short notice to stderr that asks you
to run `kt self-update`. Machine-readable JSON output on stdout remains
unchanged.

Disable automatic update checks with:

```bash
KTESIO_NO_UPDATE_CHECK=1 kt list
```

## `kt self-update`

Update the `kt` binary itself.

```bash
kt self-update
```

Behavior:

- Detects the current install channel automatically.
- Uses `brew upgrade imagdy/tap/ktesio` for Homebrew installs.
- Uses `cargo install ktesio --force` for Cargo installs.
- Downloads, verifies, extracts, and replaces the current binary for manual release installs.
- Fails with an actionable Cargo install recommendation when no prebuilt binary exists for the current platform.
- Skips the passive update notice while the explicit update command is running.

## `kt search <query>`

Search public skill listings from skills.sh.

```bash
kt search tests
kt search "react native" --limit 10
kt search tests --json
kt search tests --install
```

Behavior:

- Uses the public skills.sh search endpoint by default.
- Uses the documented authenticated skills.sh API when `KTESIO_SKILLS_SH_API_KEY` is set.
- Prints install commands for GitHub-backed results, such as `kt install owner/repo/skill`.
- Marks unsupported sources as `not installable yet`.
- Retries rate limits, temporary service failures, and transient network errors up to 3 total attempts.
- For `429 Too Many Requests`, respects `Retry-After` or `X-RateLimit-Reset` before falling back to short exponential backoff with jitter.
- Shows friendly retry and final failure messages instead of raw HTTP or JSON errors.

Ktesio uses the skills.sh public API responsibly as described in the skills.sh terms, respects rate limits, and will use the documented authenticated API once API access is available.

## `kt init <path>`

Create a `skills.json` manifest in a directory.

```bash
kt init .
```

Behavior:

- Creates `skills.json` with `dependencies` and `publish` fields.
- Scans existing `.agents/skills/*` directories.
- Shows per-skill progress while looking up public matches, cloning matched repos to resolve commits, and falling back to local path dependencies.
- Uses existing `skills.lock` entries or exact public skills.sh matches to adopt known installed skills as remote dependencies and lock their current commit.
- Records unmatched installed skills as local path dependencies.
- Does not automatically publish adopted local skills.
- Leaves an existing manifest untouched.
- Fails if the target directory does not exist.

## `kt install`

Install every dependency declared in `skills.json`. If `dependencies` is omitted, it is treated as empty.

```bash
kt install
```

Behavior:

- Fetches each remote dependency into a temporary workspace.
- Shows a progress bar while cloning each repo and copying published content.
- Suppresses raw git clone progress; clone failures are summarized after the skill finishes.
- Installs only paths declared in the source repo's `skills.json` `publish` list.
- Supports local path dependencies with `dependencies.<name>.path`.
- Supports optional `rev` selectors: `commit:<sha>`, `branch:<name>`, or `tag:<name>`.
- If the source repo has no `skills.json`, asks before discovering directories under `skills/`, `SKILLS/`, or `.agents/skills/`; multiple directories can be selected.
- Records resolved commits in `skills.lock` only for successful installs.
- Continues after individual clone/copy failures and reports all errors.

## `kt install <name:repo>`

Add one skill to `skills.json` and install it.

```bash
kt install docs:https://github.com/example/agent-docs.git
kt install docs:example/agent-docs
kt install docs:example/agent-docs --ssh
kt install docs:example/agent-docs/review
```

The `repo` value can be an HTTPS URL, SSH URL, local git path, GitHub `owner/repo` shorthand, or GitHub `owner/repo/skill` shorthand. GitHub shorthand resolves to HTTPS by default; `--ssh` resolves shorthand to an SSH clone URL.

Ktesio updates `skills.json` and `skills.lock` only after the repo is fetched and installable content is copied successfully. A bad target, failed clone, missing published skill, cancelled fallback, or missing fallback skill directory leaves those files unchanged.

The single-skill install flow uses the same progress bar and quiet git output as bulk install.

## `kt install <repo>`

Install one or more published skills from a source repository without naming the package first.

```bash
kt install https://github.com/example/agent-docs.git
kt install example/agent-docs
kt install example/agent-docs/review
kt install example/agent-docs --skill review
kt install --all https://github.com/example/agent-docs.git
```

Behavior:

- Reads the source repo's `skills.json` `publish` list and derives destination names from published skill names.
- Prompts when multiple published skills are available.
- `--all` installs every published skill without prompting.
- `--skill <name>` installs one matching published or fallback-discovered skill.
- `--yes` accepts safe defaults, such as a single obvious fallback skill.
- `--no-input` fails instead of prompting when a choice is required.
- Repos without `skills.json` can use fallback discovery from `.md` files or directories under `skills/`, `SKILLS/`, or `.agents/skills/`.

## `kt publish`

Publish local skills from this repo.

```bash
kt publish
```

Behavior:

- Selects local path dependencies or directories under `.agents/skills/`.
- Writes selected skill names to the `publish` list.
- If an untracked `.agents/skills/<name>` directory is selected, first records it as a local path dependency.
- Does not publish remote dependencies.
- Creates a manifest if needed.

## `kt publish add <name> <path>`

Add or update one published local skill in `skills.json`.

```bash
kt publish add docs skills/docs
```

Behavior:

- Creates `skills.json` if needed.
- Preserves existing dependencies and publish entries.
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

- Checks `skills.json`, `skills.lock`, `.agents/skills/`, published local paths, orphaned lock entries, missing installed directories, and git availability.
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
