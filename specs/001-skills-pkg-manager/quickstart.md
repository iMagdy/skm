# Quickstart: Skills Package Manager CLI

**Feature**: 001-skills-pkg-manager
**Date**: 2026-05-30

## Prerequisites

- Rust toolchain (rustup, cargo)
- Git (on PATH)

## Setup

```bash
# Clone the repo
git clone <repo-url> && cd <repo-name>

# Build
cargo build --release

# Run
cargo run -- --help
```

## Development

```bash
# Build (debug)
cargo build

# Run tests
cargo test

# Run a specific command
cargo run -- init .
cargo run -- install clap:https://github.com/clap-rs/clap.git
cargo run -- list
```

## Project Layout

```
src/
├── main.rs          # Entry point, clap CLI definition
├── cli/             # One file per subcommand
├── manifest.rs      # skills.json read/write
├── lockfile.rs      # skills.lock read/write
├── git.rs           # Git clone/fetch/checkout wrappers
├── skill.rs         # Export copying logic
└── error.rs         # miette error types
```

## Key Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
miette = { version = "7", features = ["fancy"] }
indicatif = "0.17"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
walkdir = "2"
```

## Testing

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# With output
cargo test -- --nocapture
```
