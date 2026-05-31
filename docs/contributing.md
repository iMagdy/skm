# Contributing Guide

This page is the hands-on development guide. For project rules and DCO details, see [../CONTRIBUTING.md](../CONTRIBUTING.md).

## Setup

```bash
git clone https://github.com/iMagdy/ktesio.git
cd ktesio
cargo build
cargo test --all-targets
```

## Development Loop

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
python3 scripts/check_docs.py
```

## Adding CLI Behavior

- Update `src/main.rs` command parsing.
- Add or update a module under `src/cli/`.
- Add unit tests for command logic with explicit project roots.
- Add integration tests under `tests/` for user-facing workflows.
- Update [commands.md](commands.md) and [quickstart.md](quickstart.md) when behavior changes.

## Test Fixtures

Integration tests use local temporary git repositories through `tests/helpers/mod.rs`. Avoid network-only tests in the default suite.

## Pull Requests

- Keep changes focused.
- Use conventional commit messages.
- Sign commits with `git commit -s`.
- Include docs and tests in the same change when behavior changes.
- Make sure CI passes before requesting review.
