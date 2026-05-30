# Quick Start: skm

**Date**: 2026-05-30
**Feature**: 002-project-docs

## What is skm?

skm is a command-line tool for managing agentic skills. Skills are git repositories that provide specialized instructions and workflows for AI coding agents. skm fetches, installs, upgrades, and manages these skills in your project.

## Prerequisites

- Rust 2021 edition or later
- Git installed and configured

## Install

```bash
# Clone the repository
git clone https://github.com/imagdy/skills.git
cd skills

# Build the project
cargo build --release

# The binary will be at target/release/skm
# Add it to your PATH or run via cargo
```

## Initialize a Project

```bash
# Create a skills manifest in your project
skm init .

# This creates skills.json with empty skills and exports
cat skills.json
```

## Install Skills

```bash
# Install all skills from the manifest
skm install

# Install a specific skill
skm install myskill:https://github.com/example/repo.git

# List installed skills
skm list
```

## Upgrade & Uninstall

```bash
# Upgrade all installed skills to latest versions
skm upgrade

# Show details for a specific skill
skm show myskill

# Remove a skill
skm uninstall myskill
```

## Next Steps

- [Installation Guide](installation.md) — Detailed platform-specific instructions
- [Command Reference](commands.md) — Full documentation for all commands
- [Manifest Format](manifest.md) — How to configure `skills.json`
- [Contributing](contributing.md) — Help improve skm
