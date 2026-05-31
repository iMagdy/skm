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
kt install docs:https://github.com/example/agent-docs.git
```

Or edit `skills.json` manually and run:

```bash
kt install
```

Installed files are placed under `.agents/skills/<name>/`, and `skills.lock` records the exact commit after a successful fetch and copy.

Source repos normally declare installable paths in their own `skills.json` `exports`. If a source repo has no `skills.json`, Ktesio warns, asks for confirmation, and can install one or more directories found under `skills/` or `SKILLS/`.

While installing, Ktesio shows a progress bar for cloning and file copy work. Raw git clone output stays hidden unless a failure needs a short summary.

## Inspect Project State

```bash
kt list
kt show docs
```

Status output is color-coded with small icons in terminals that support them.

## Export Installed Skills

If `.agents/skills/` and `skills.lock` already exist, rebuild `skills.json` with:

```bash
kt export
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
