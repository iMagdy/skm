---
title: Getting Started
description: Build Ktesio from source and run the local skill manifest workflow end to end.
---

# Quickstart

This guide gets Ktesio running from source and shows the full local workflow.

## Build Ktesio

```bash
git clone https://github.com/iMagdy/ktesio.git
cd ktesio
cargo install --path .
```

This installs Ktesio onto your Cargo binary path.

## Create a Skills Manifest

From the project where you want to use agent skills:

```bash
kt init .
```

For a new project, this writes:

```json
{
  "dependencies": {},
  "publish": []
}
```

If `.agents/skills/` already contains installed skills, `kt init .` adopts them as dependencies. Known public skills are recorded as remote dependencies when they can be resolved; unmatched custom skills become local path dependencies. Nothing is published automatically.

## Install a Skill

Add and install one skill with:

```bash
kt install docs:https://github.com/example/agent-docs.git
kt install docs:example/agent-docs
```

If the source repo declares multiple published skills, install from the repo directly:

```bash
kt install https://github.com/example/agent-docs.git
kt install example/agent-docs/docs
kt install example/agent-docs --skill docs
kt install --all https://github.com/example/agent-docs.git
```

Search public skill listings with:

```bash
kt search tests
kt search tests --install
```

Search uses skills.sh for discovery and still installs by cloning git repositories. Ktesio respects skills.sh rate limits with bounded retries.

Or edit `skills.json` manually and run:

```bash
kt install
```

Installed files are placed under `.agents/skills/<name>/`, and `skills.lock` records the exact commit after a successful fetch and copy.

Source repos normally declare installable paths in their own `skills.json` `publish` list. If a source repo has no `skills.json`, Ktesio warns, asks for confirmation, and can install one or more directories found under `skills/`, `SKILLS/`, or `.agents/skills/`.

While installing, Ktesio shows a progress bar for cloning and file copy work. Raw git clone output stays hidden unless a failure needs a short summary.

## Inspect Project State

```bash
kt list
kt list --json
kt show docs
kt doctor
```

Status output is color-coded with small icons in terminals that support them.

## Publish Local Skills

To expose a local skill from your repo, run:

```bash
kt publish add docs skills/docs
```

## Upgrade or Remove Skills

```bash
kt upgrade
kt remove docs
```

`kt remove` is an alias for `kt uninstall`.

## Next Steps

- Read the [command reference](commands.md).
- Learn the [manifest format](manifest.md).
- Check [troubleshooting](troubleshooting.md) for common git and path issues.
