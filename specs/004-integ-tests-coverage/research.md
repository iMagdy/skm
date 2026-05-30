# Research: Test Coverage & Integration Tests

**Feature**: 004-integ-tests-coverage
**Date**: Sat May 30 2026

## Research Tasks

### 1. Coverage Measurement Tooling

**Decision**: Use `cargo-tarpaulin` for line coverage measurement.

**Rationale**:
- Explicitly referenced in constitution Principle VI
- Industry standard for Rust coverage
- Supports CI integration with threshold gates
- Outputs LCOV format for reporting

**Alternatives Considered**:
- `cargo-cov`: Less maintained, limited CI integration
- `grcov`: Requires LLVM source-based coverage setup, more complex
- `kcov`: Binary instrumentation, not ideal for Rust

### 2. Integration Test Framework

**Decision**: Use `cargo test` with `#[ignore]` attribute for network-dependent tests.

**Rationale**:
- Native Rust test runner, no additional dependencies
- `#[ignore]` allows opt-in execution via `cargo test -- --ignored`
- Supports `#[cfg(integration)]` for compile-time segregation
- Compatible with CI retry mechanism (FR-015)

**Alternatives Considered**:
- `rstest`: Adds dependency, overkill for this use case
- `trycmd`: Better for CLI snapshot testing, not integration workflows
- Custom test harness: Unnecessary complexity

### 3. Fixture Repository Pinning

**Decision**: Pin fixture repos to commit SHAs with constants in test files.

**Rationale**:
- Ensures reproducible test runs (FR-010)
- Prevents test failures from upstream changes
- SHA can be updated deliberately when needed
- Matches real-world usage pattern

**Implementation**:
```rust
const AWESOME_COPILOT_SHA: &str = "118974fb72ec31524b002795c116fd66bde14bef";
const AGENT_SKILLS_SHA: &str = "180115660cfb8a86b808f117475a01f54caf3bc5";
```

### 4. Test Cleanup Strategy

**Decision**: Use `tempfile` crate for isolated test directories with automatic cleanup.

**Rationale**:
- FR-009 requires cleanup of cloned repositories and generated files
- `tempfile::TempDir` provides RAII-based cleanup
- Each test gets isolated directory, no cross-test interference
- Handles Ctrl+C and test panics gracefully

**Alternatives Considered**:
- Manual cleanup in `#[test]` functions: Risk of leaks on panic
- `std::fs::remove_dir_all`: Requires explicit calls, error-prone
- Global temp directory: Risk of cross-test contamination

### 5. CI Retry Mechanism

**Decision**: Implement retry logic in CI workflow, not in test code.

**Rationale**:
- FR-015 requires retry up to 3 times before failing
- CI-level retry handles all transient failures uniformly
- Test code remains clean and focused on assertions
- Compatible with GitHub Actions retry actions

**Implementation**: Use `nick-fields/retry@v3` or equivalent in CI workflow.

### 6. Coverage Gate Enforcement

**Decision**: Use `cargo-tarpaulin --fail-under 95` in CI pipeline.

**Rationale**:
- FR-012 requires hard gate at 95%
- `--fail-under` flag exits with non-zero code when threshold not met
- Can be combined with `--out Xml` for CI parsing
- Simple, explicit, constitution-aligned

### 7. Test File Organization

**Decision**: Separate integration test files by command/feature area.

**Rationale**:
- FR-008 requires tests be tagged/segregated
- Separate files allow selective execution
- Clear naming convention (`install_default.rs`, `install_fallback.rs`, `export.rs`)
- Each file focuses on one workflow path

### 8. Shallow Clone Strategy

**Decision**: Use `git clone --depth 1` for fixture repositories.

**Rationale**:
- Spec assumes shallow clones are sufficient
- Reduces network transfer and test execution time
- Meets 30s performance target (SC-003)
- Full history not needed for install/export testing

## Summary of Decisions

| Area | Decision | Key Benefit |
|------|----------|-------------|
| Coverage tool | cargo-tarpaulin | Constitution-aligned, CI-ready |
| Test framework | cargo test + #[ignore] | Native, no dependencies |
| Fixture pinning | Commit SHA constants | Reproducibility |
| Cleanup | tempfile crate | RAII-based, leak-proof |
| Retry | CI-level retry | Uniform, clean test code |
| Coverage gate | --fail-under 95 | Hard enforcement |
| File organization | Separate integration files | Selective execution |
| Clone strategy | Shallow clone | Performance |
