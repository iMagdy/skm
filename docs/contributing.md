# Contributing

Thank you for your interest in contributing to skm! This guide will help you get started.

## Development Setup

### Prerequisites

- **Rust**: 2021 edition or later (install via [rustup](https://rustup.rs/))
- **Git**: Any recent version

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/imagdy/skills.git
cd skills

# Build the project
cargo build

# Run tests
cargo test
```

## Project Structure

```text
src/
├── cli/            # CLI command implementations
│   ├── init.rs     # skm init command
│   ├── install.rs  # skm install command
│   ├── list.rs     # skm list command
│   ├── show.rs     # skm show command
│   ├── uninstall.rs # skm uninstall command
│   └── upgrade.rs  # skm upgrade command
├── error.rs        # Error types (thiserror)
├── git.rs          # Git operations (shelling out to git CLI)
├── lockfile.rs     # Lockfile read/write
├── main.rs         # Entry point
├── manifest.rs     # Manifest read/write
└── skill.rs        # Skill data types
```

## Coding Standards

- Follow Rust standard style (use `cargo fmt` before committing)
- Run `cargo clippy` and address all warnings
- Use `miette` for error diagnostics, `thiserror` for error types
- Use `indicatif` for progress indicators on long-running operations
- All error messages MUST include a suggested remediation action

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Test Coverage

The project requires >=95% test coverage (Constitution Principle VI). See [testing.md](testing.md) for how to measure and verify coverage.

## Pull Request Process

1. **Fork** the repository
2. **Create** a feature branch from `main`
3. **Make** your changes following the coding standards
4. **Add** tests for new functionality
5. **Run** `cargo test` and `cargo clippy`
6. **Commit** with descriptive messages (conventional commits)
7. **Push** to your fork
8. **Open** a pull request

### PR Requirements

- All tests pass
- No clippy warnings
- Test coverage >=95% (for new code)
- Documentation updated if applicable
- Descriptive commit messages

## Documentation

When adding or changing features, update the corresponding documentation in `docs/`. The `docs/` directory must always reflect the current code state (Constitution Principle VII).

### Documentation Review Process

To verify documentation accuracy:

1. **Command examples**: Run each documented command and verify it works as described
2. **File references**: Verify all linked files exist and are accessible
3. **JSON examples**: Validate JSON snippets are syntactically correct
4. **Cross-references**: Check that "See Also" links resolve to valid documents
5. **Completeness**: Ensure all CLI commands are documented with examples

### Review Checklist

- [ ] All command examples run successfully
- [ ] All internal links resolve to valid files
- [ ] All JSON examples are syntactically correct
- [ ] No broken "See Also" links
- [ ] All CLI commands documented in commands.md

## Questions?

Open an issue or start a discussion on GitHub.
