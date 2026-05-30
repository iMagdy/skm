# Installation

## Prerequisites

- **Rust**: 2021 edition or later (install via [rustup](https://rustup.rs/))
- **Git**: Any recent version

## Install from Source

```bash
# Clone the repository
git clone https://github.com/imagdy/skills.git
cd skills

# Build in release mode
cargo build --release

# The binary will be at target/release/skm
# Add to your PATH or run via cargo
```

## Install via Cargo

If published to crates.io:

```bash
cargo install skm
```

## Verify Installation

```bash
skm --version
skm --help
```

## Cross-Platform Notes

skm works on Linux, macOS, and Windows without platform-specific code paths.

**Platform considerations:**
- File operations use path-agnostic APIs (`std::path::Path/PathBuf`)
- The `.agents/skills/` directory is the canonical install location across all platforms
- Git operations shell out to the system `git` CLI, inheriting platform-specific configuration
- No platform-specific code paths are used in the implementation

**Windows-specific notes:**
- Ensure Git is installed and available in PATH
- Use forward slashes or escaped backslashes in paths

**macOS-specific notes:**
- Xcode Command Line Tools may be required for building from source

**Linux-specific notes:**
- Standard build tools (gcc, make) may be required for native dependencies

## Next Steps

- [Quick Start](../specs/002-project-docs/quickstart.md) — Get started with skm
- [Command Reference](commands.md) — Learn all available commands
