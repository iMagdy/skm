# Installation

Ktesio is a Rust CLI. It works on macOS, Linux, and Windows as long as `git` is available on `PATH`.

## Prerequisites

- Rust 2021 toolchain or newer from [rustup](https://rustup.rs/)
- Git

## Install from Source

```bash
git clone https://github.com/iMagdy/ktesio.git
cd ktesio
cargo install --path .
```

Verify:

```bash
kt --version
kt --help
```

## Install from crates.io

```bash
cargo install ktesio
```

The crates.io package is named `ktesio`; it installs the `kt` binary.

## Install from a Release

Download the archive for your platform from [GitHub Releases](https://github.com/iMagdy/ktesio/releases), then unpack it and place the `kt` binary on your `PATH`.

Release archives use this naming pattern:

```text
ktesio-<tag>-<target>.tar.gz
ktesio-<tag>-<target>.zip
```

Each release also includes `.sha256` files and an aggregate checksum file.

## Install with Homebrew

After a release is published to the Homebrew tap:

```bash
brew install imagdy/tap/ktesio
```

The formula installs the prebuilt macOS or Linux release archive for your platform.

## Platform Notes

- macOS may require Xcode Command Line Tools when building from source.
- Windows users should install Git for Windows and make sure `git.exe` is on `PATH`.
- Linux users may need standard build tools for Rust crates.

## Next Steps

- [Quickstart](quickstart.md)
- [Command reference](commands.md)
