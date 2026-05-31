# Release Process

Ktesio releases are driven by git tags.

## Tag Format

Use semantic version tags:

```text
vMAJOR.MINOR.PATCH
```

Example:

```bash
git tag v0.1.0
git push origin v0.1.0
```

## What the Tag Workflow Does

When a `v*` tag is pushed, `.github/workflows/release.yml`:

1. Builds Tier 1 CLI binaries for macOS Intel, macOS Apple Silicon, Windows x64, and Linux x64.
2. Archives each binary with a deterministic file name.
3. Generates per-asset `.sha256` files and one aggregate checksum file.
4. Creates a draft GitHub Release for the tag.
5. Uploads all release assets.
6. Publishes the `ktesio` crate to crates.io.
7. Publishes the GitHub Release with a clean asset table.
8. Updates the Homebrew tap formula for macOS Intel, macOS Apple Silicon, and Linux x64.
9. Opens a pull request updating `CHANGELOG.md` and `docs/RELEASE_NOTES.md`.

The docs PR happens after the tag because a tag points at an existing commit. The release page is updated immediately; repository docs are refreshed through the follow-up pull request.

## crates.io

Release tags publish the crate package to crates.io as:

```text
ktesio
```

The installed binary is `kt`, so users can install it with:

```bash
cargo install ktesio
```

Configure this repository secret before publishing a tag:

- `CARGO_REGISTRY_TOKEN`: crates.io API token with publish access to the `ktesio` crate.

The workflow verifies that `Cargo.toml` version matches the tag without the leading `v`. If the crate version is already published, the workflow skips the publish step so release reruns stay safe.

## Homebrew

Homebrew publishing updates a tap formula from the release checksums. By default, the workflow writes:

```text
Formula/ktesio.rb
```

to:

```text
iMagdy/homebrew-tap
```

Configure these repository settings before publishing a tag:

- `HOMEBREW_TAP_TOKEN` secret: token with write access to the tap repository.
- `HOMEBREW_TAP_REPOSITORY` variable: optional `owner/repo` override. Defaults to `<release-owner>/homebrew-tap`.
- `HOMEBREW_TAP_BRANCH` variable: optional target branch override. Defaults to `main`.

The generated formula installs the prebuilt macOS or Linux archive for the user's platform and declares `git` as a runtime dependency.

## Local Dry Run

Generate release notes without publishing anything:

```bash
python3 scripts/generate_release_docs.py v0.1.0 --output-dir target/release-docs-test
```

Update local docs for inspection:

```bash
python3 scripts/generate_release_docs.py v0.1.0 --update-files
```

## Asset Names

```text
ktesio-<tag>-x86_64-apple-darwin.tar.gz
ktesio-<tag>-aarch64-apple-darwin.tar.gz
ktesio-<tag>-x86_64-pc-windows-msvc.zip
ktesio-<tag>-x86_64-unknown-linux-gnu.tar.gz
ktesio-<tag>-checksums.txt
```
