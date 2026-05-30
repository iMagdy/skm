# Release Process

`skm` releases are driven by git tags.

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
6. Publishes the GitHub Release with a clean asset table.
7. Opens a pull request updating `CHANGELOG.md` and `docs/RELEASE_NOTES.md`.

The docs PR happens after the tag because a tag points at an existing commit. The release page is updated immediately; repository docs are refreshed through the follow-up pull request.

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
skm-<tag>-x86_64-apple-darwin.tar.gz
skm-<tag>-aarch64-apple-darwin.tar.gz
skm-<tag>-x86_64-pc-windows-msvc.zip
skm-<tag>-x86_64-unknown-linux-gnu.tar.gz
skm-<tag>-checksums.txt
```
