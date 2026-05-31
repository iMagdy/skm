# Troubleshooting

## `skills.json` Not Found

Run:

```bash
kt init .
```

Then add skills manually or with `kt install <name:repo>`.

## Git Clone Fails

Ktesio hides raw git clone progress during normal installs, then prints the useful git error line when a clone fails.

Check that:

- `git` is installed and on `PATH`.
- The repo URL is correct.
- SSH keys or credential helpers are configured for private repositories.
- Your network can reach the remote.

If you need full git diagnostics, run the equivalent `git clone <repo-url>` manually from the same shell.

## Skill Is Listed as Missing

`kt list` reports `missing` when `skills.lock` has an entry but `.agents/skills/<name>/` is absent.

Fix it with:

```bash
kt install
```

## Skill Is Listed as Orphaned

`orphaned` means `skills.lock` has an entry that is no longer in `skills.json`.

Options:

- Run `kt export` if the skill should be restored to the manifest.
- Remove the stale lock entry by uninstalling or editing the lockfile.

## Project State Looks Wrong

Run:

```bash
kt doctor
```

`kt doctor` checks the manifest, lockfile, installed directories, local export paths, orphaned entries, and git availability, then prints repair hints.

## Release Workflow Did Not Update Docs

The tag workflow publishes the GitHub Release first, then opens a pull request for `CHANGELOG.md` and `docs/RELEASE_NOTES.md`.

Check the release workflow logs and open pull requests for a branch named like:

```text
release-docs/<tag>
```

## Speckit Issue Sync Cannot Find the Project

Confirm the project title and owner:

```bash
gh project list --owner iMagdy
```

Then refresh auth if needed:

```bash
gh auth refresh -s project
```
