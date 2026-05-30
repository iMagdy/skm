# Agent Notes

There is no active Speckit feature for this repository.

When working here:

- Treat `specs/` as historical product context unless the user explicitly activates a feature.
- Prefer the public docs in `README.md` and `docs/` for current user-facing behavior.
- Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --all-targets` before handing off code changes.
- Use `scripts/speckit_sync_issues.py --feature-dir <spec-dir> --dry-run` before syncing a Speckit feature to GitHub issues.
