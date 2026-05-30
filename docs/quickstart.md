# Quickstart

This guide gets `skm` running from source and shows the full local workflow.

## Build skm

```bash
git clone https://github.com/iMagdy/skm.git
cd skm
cargo install --path .
```

This installs `skm` onto your Cargo binary path.

## Create a Skills Manifest

From the project where you want to use agent skills:

```bash
skm init .
```

This writes:

```json
{
  "skills": {},
  "exports": {}
}
```

## Install a Skill

Add and install one skill with:

```bash
skm install docs:https://github.com/example/agent-docs.git
```

Or edit `skills.json` manually and run:

```bash
skm install
```

Installed files are placed under `.agents/skills/<name>/`, and `skills.lock` records the exact commit after a successful fetch and copy.

Source repos normally declare installable paths in their own `skills.json` `exports`. If a source repo has no `skills.json`, `skm` warns, asks for confirmation, and can install one or more directories found under `skills/` or `SKILLS/`.

## Inspect Project State

```bash
skm list
skm show docs
```

## Export Installed Skills

If `.agents/skills/` and `skills.lock` already exist, rebuild `skills.json` with:

```bash
skm export
```

## Upgrade or Remove Skills

```bash
skm upgrade
skm remove docs
```

`skm remove` is an alias for `skm uninstall`.

## Next Steps

- Read the [command reference](commands.md).
- Learn the [manifest format](manifest.md).
- Check [troubleshooting](troubleshooting.md) for common git and path issues.
