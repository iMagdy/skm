# skm

A command-line tool for managing agentic skills — specialized instructions and workflows for AI coding agents.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/iMagdy/skm.git
cd skm

# Build
cargo build --release

# Initialize a project
skm init .

# Install skills
skm install
```

## Commands

| Command | Description |
|---------|-------------|
| `skm init` | Create a new skills manifest |
| `skm install` | Install skills from the manifest |
| `skm upgrade` | Upgrade installed skills to latest |
| `skm list` | List installed skills |
| `skm show` | Show skill details |
| `skm uninstall` | Remove a skill |

## Documentation

For detailed documentation, see [docs/](docs/):

- [Installation Guide](docs/installation.md)
- [Command Reference](docs/commands.md)
- [Contributing](docs/contributing.md)
- [Architecture](docs/architecture.md)
- [Testing](docs/testing.md)

## License

See [LICENSE](LICENSE) for details.
