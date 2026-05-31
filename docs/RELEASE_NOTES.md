# Release Notes

Release notes are generated when a version tag is published.

The tag workflow updates the GitHub Release immediately and then opens a pull request that refreshes this file and the root `CHANGELOG.md`.

## v0.1.1

Comparison: Initial release history

| Platform | Target | Archive | Checksum |
|----------|--------|---------|----------|
| macOS Intel | `x86_64-apple-darwin` | [ktesio-v0.1.1-x86_64-apple-darwin.tar.gz](https://github.com/iMagdy/ktesio/releases/download/v0.1.1/ktesio-v0.1.1-x86_64-apple-darwin.tar.gz) | [sha256](https://github.com/iMagdy/ktesio/releases/download/v0.1.1/ktesio-v0.1.1-x86_64-apple-darwin.tar.gz.sha256) |
| macOS Apple Silicon | `aarch64-apple-darwin` | [ktesio-v0.1.1-aarch64-apple-darwin.tar.gz](https://github.com/iMagdy/ktesio/releases/download/v0.1.1/ktesio-v0.1.1-aarch64-apple-darwin.tar.gz) | [sha256](https://github.com/iMagdy/ktesio/releases/download/v0.1.1/ktesio-v0.1.1-aarch64-apple-darwin.tar.gz.sha256) |
| Windows x64 | `x86_64-pc-windows-msvc` | [ktesio-v0.1.1-x86_64-pc-windows-msvc.zip](https://github.com/iMagdy/ktesio/releases/download/v0.1.1/ktesio-v0.1.1-x86_64-pc-windows-msvc.zip) | [sha256](https://github.com/iMagdy/ktesio/releases/download/v0.1.1/ktesio-v0.1.1-x86_64-pc-windows-msvc.zip.sha256) |
| Linux x64 | `x86_64-unknown-linux-gnu` | [ktesio-v0.1.1-x86_64-unknown-linux-gnu.tar.gz](https://github.com/iMagdy/ktesio/releases/download/v0.1.1/ktesio-v0.1.1-x86_64-unknown-linux-gnu.tar.gz) | [sha256](https://github.com/iMagdy/ktesio/releases/download/v0.1.1/ktesio-v0.1.1-x86_64-unknown-linux-gnu.tar.gz.sha256) |
| All | checksums | [ktesio-v0.1.1-checksums.txt](https://github.com/iMagdy/ktesio/releases/download/v0.1.1/ktesio-v0.1.1-checksums.txt) | - |

### Features

- improve cli visuals and help ([052ca93](https://github.com/iMagdy/ktesio/commit/052ca93))
- add release automation and open source polish ([f8ef392](https://github.com/iMagdy/ktesio/commit/f8ef392))
- Add GitHub CI pipeline for PR checks (#4) ([171de35](https://github.com/iMagdy/ktesio/commit/171de35))
- integrate GitHub issue tracking into task implementation and PR workflow ([4b4fdb7](https://github.com/iMagdy/ktesio/commit/4b4fdb7))
- add integration tests and improve unit test coverage ([2f6bf07](https://github.com/iMagdy/ktesio/commit/2f6bf07))
- add skill install fallback discovery ([7e6b43f](https://github.com/iMagdy/ktesio/commit/7e6b43f))
- add comprehensive documentation and test coverage ([b5fc1a5](https://github.com/iMagdy/ktesio/commit/b5fc1a5))
- implement agentic skills package manager CLI ([73e6ac3](https://github.com/iMagdy/ktesio/commit/73e6ac3))

### Fixes

- allow partial skill manifests ([a5f5dc4](https://github.com/iMagdy/ktesio/commit/a5f5dc4))
- install exported skill content safely ([2f525b2](https://github.com/iMagdy/ktesio/commit/2f525b2))

### Documentation

- mark dependabot updates merged ([a73ebeb](https://github.com/iMagdy/ktesio/commit/a73ebeb))
- clarify solo maintainer branch policy ([36a1b74](https://github.com/iMagdy/ktesio/commit/36a1b74))
- add repository audit checklist ([d978ca7](https://github.com/iMagdy/ktesio/commit/d978ca7))
- correct repository name and path in quick start instructions ([10da4ff](https://github.com/iMagdy/ktesio/commit/10da4ff))
- add test coverage and documentation currency principles (v1.1.0) ([c72c185](https://github.com/iMagdy/ktesio/commit/c72c185))

### Tests

- increase coverage for cli helpers (#7) ([7f29853](https://github.com/iMagdy/ktesio/commit/7f29853))

### CI

- publish only release asset files ([00e7fd3](https://github.com/iMagdy/ktesio/commit/00e7fd3))
- publish crate before release artifacts ([88438e5](https://github.com/iMagdy/ktesio/commit/88438e5))
- identify crates io release check ([dc2f96d](https://github.com/iMagdy/ktesio/commit/dc2f96d))
- use current intel macos release runner ([ad0e63d](https://github.com/iMagdy/ktesio/commit/ad0e63d))
- exempt dependabot prs from dco by author ([537742b](https://github.com/iMagdy/ktesio/commit/537742b))
- align dco checks with automation ([5682357](https://github.com/iMagdy/ktesio/commit/5682357))
- publish release artifacts to ghcr homebrew and crates (#6) ([d571bd5](https://github.com/iMagdy/ktesio/commit/d571bd5))

### Maintenance

- prepare 0.1.1 release ([285f059](https://github.com/iMagdy/ktesio/commit/285f059))
- rename project to ktesio (#10) ([d2cfa1f](https://github.com/iMagdy/ktesio/commit/d2cfa1f))
- bump cargo dependency group ([3ac0ab6](https://github.com/iMagdy/ktesio/commit/3ac0ab6))
- bump github actions group ([4a0ed6e](https://github.com/iMagdy/ktesio/commit/4a0ed6e))
- use canonical apache license text ([e5acc16](https://github.com/iMagdy/ktesio/commit/e5acc16))
- harden repository governance ([c1463f4](https://github.com/iMagdy/ktesio/commit/c1463f4))

### Other Changes

- Add license, homepage, repository, and readme to Cargo.toml ([d636953](https://github.com/iMagdy/ktesio/commit/d636953))
- apply code formatting and update Rust edition to 2024 ([32fbc59](https://github.com/iMagdy/ktesio/commit/32fbc59))
- speckit ([8d14960](https://github.com/iMagdy/ktesio/commit/8d14960))
- Initial commit from Specify template ([76d7354](https://github.com/iMagdy/ktesio/commit/76d7354))
