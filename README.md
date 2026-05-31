<p align="center">
  <img src="docs/assets/ktesio-banner.jpg" alt="Ktesio banner: Share, install, and manage agent skills" width="100%">
</p>

# Ktesio

[![CI](https://github.com/iMagdy/ktesio/actions/workflows/ci.yml/badge.svg)](https://github.com/iMagdy/ktesio/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/ktesio.svg)](https://crates.io/crates/ktesio)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Ktesio is a tiny Rust CLI for installing and sharing agent skills. It makes reusable agent instructions portable across projects by keeping a simple `skills.json` manifest, a reproducible `skills.lock`, and installed skills under `.agents/skills/`.

## Why Ktesio?

- **Portable skills**: move agent workflows between repositories without manual copy-paste.
- **Git-native distribution**: install skills from normal HTTPS or SSH git repositories.
- **Reproducible installs**: lock every installed skill to the exact commit that was fetched.
- **Friendly project state**: list, inspect, upgrade, export, and remove skills from one CLI.
- **Polished terminal UX**: color-coded statuses, icons, and progress bars keep git work readable.
- **Agent-ready layout**: installed content lands where coding agents already look for skills.

## 60-Second Quickstart

```bash
git clone https://github.com/iMagdy/ktesio.git
cd ktesio
cargo install --path .

# In another project:
kt init .
kt install docs:https://github.com/example/agent-docs.git
kt list
```

This creates:

```text
skills.json
skills.lock
.agents/skills/
```

During install and upgrade, Ktesio shows progress bars for long-running git work and hides raw `git clone` or `git fetch` output unless an error needs to be summarized.

## Install

From source:

```bash
git clone https://github.com/iMagdy/ktesio.git
cd ktesio
cargo install --path .
```

From crates.io:

```bash
cargo install ktesio
```

From a release archive, download the binary for your platform from [GitHub Releases](https://github.com/iMagdy/ktesio/releases), unpack it, and place `kt` on your `PATH`.

With Homebrew, install from the tap once a release is published:

```bash
brew install imagdy/tap/ktesio
```

## Commands

| Command | Purpose |
|---------|---------|
| `kt init <path>` | Create `skills.json` in a project |
| `kt install` | Install every skill declared in `skills.json` |
| `kt install <name:repo>` | Add and install one skill |
| `kt export` | Rebuild `skills.json` from installed skills |
| `kt upgrade` | Fetch latest commits for installed skills |
| `kt list` | Show installed, missing, and orphaned skills |
| `kt show <name>` | Show one skill's repo, commit, path, and status |
| `kt uninstall <name>` | Remove a skill from manifest, lockfile, and disk |
| `kt remove <name>` | Alias for `kt uninstall <name>` |

## Manifest

`skills.json` is intentionally small:

```json
{
  "skills": {
    "docs": {
      "repo": "https://github.com/example/agent-docs.git"
    }
  },
  "exports": {}
}
```

When another repository installs a skill repo, that repo can use `exports` to choose which local files or folders become installable skills. The top-level `skills` and `exports` keys are optional and default to empty objects. Ktesio installs only exported paths; if the source repo has no `skills.json`, it asks before falling back to selectable directories under `skills/` or `SKILLS/`.

## Documentation

- [Quickstart](docs/quickstart.md)
- [Installation](docs/installation.md)
- [Command reference](docs/commands.md)
- [Manifest format](docs/manifest.md)
- [Lockfile format](docs/lockfile.md)
- [Architecture](docs/architecture.md)
- [Testing](docs/testing.md)
- [Release process](docs/release-process.md)
- [GitHub project sync](docs/github-project-sync.md)
- [Troubleshooting](docs/troubleshooting.md)
- [Contributing](CONTRIBUTING.md)

## Project Status

Ktesio is early, useful, and intentionally conservative. The current package format is plain JSON plus git. Future work may add registries, richer metadata, and package signing without taking away the simple manifest workflow.

## License

Licensed under [Apache-2.0](LICENSE).
