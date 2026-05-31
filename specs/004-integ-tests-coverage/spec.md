# Feature Specification: Test Coverage & Integration Tests

**Feature Branch**: `004-integ-tests-coverage`
**Created**: Sat May 30 2026
**Status**: Draft
**Input**: User description: "complete tests to at least 95% as per constitution, add integration tests: use (https://github.com/iMagdy/awesome-copilot.git) for default install and export scenario as a remote repo. and use (https://github.com/iMagdy/agent-skills) for the scenario where skills.json is not found (the fallback)."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Achieve 95% Test Coverage (Priority: P1)

The project MUST meet the constitution's requirement of >=95% line coverage across all `src/` code. This involves identifying untested code paths, writing unit tests for uncovered logic, and ensuring all existing tests pass. Coverage will be measured using `cargo-tarpaulin` or equivalent line coverage tooling.

**Why this priority**: This is a non-negotiable constitutional requirement (Principle VI). No feature work or releases may proceed until this threshold is met.

**Independent Test**: Can be verified by running the coverage tool and confirming the reported percentage meets or exceeds 95%.

**Acceptance Scenarios**:

1. **Given** the full source tree, **When** coverage tooling is executed against `src/`, **Then** line coverage is reported at >=95%
2. **Given** a newly added module or function, **When** it is merged, **Then** it includes unit tests that cover its code paths
3. **Given** existing tests, **When** coverage tooling runs, **Then** no previously-tested code has regressed to uncovered status

---

### User Story 2 - Integration Test: Default Install & Export (Priority: P2)

Integration tests MUST validate the core install and export workflows against a real remote repository. The repository `https://github.com/iMagdy/awesome-copilot.git` will serve as the canonical test fixture for the default install path (repo with `skills.json`) and the export path (project exporting local skills).

**Why this priority**: Integration tests provide high-confidence validation of end-to-end workflows that unit tests cannot fully cover (network I/O, git operations, filesystem state).

**Independent Test**: Can be executed by running the integration test suite, which clones the fixture repo, performs install/export operations, and asserts expected outcomes.

**Acceptance Scenarios**:

1. **Given** a fresh project with no installed skills, **When** user runs `kt install` targeting `awesome-copilot`, **Then** the skill is cloned to `.agents/skills/` and appears in `skills.lock`
2. **Given** a project with skills installed, **When** user runs `kt export`, **Then** `skills.json` is generated/updated listing all locally available skills
3. **Given** a project with a valid `skills.json`, **When** user runs `kt install`, **Then** all listed skills are installed successfully from their remote sources
4. **Given** network connectivity, **When** install operation completes, **Then** the operation finishes within 30 seconds for a single skill

---

### User Story 3 - Integration Test: Fallback Discovery (Priority: P3)

Integration tests MUST validate the fallback skill discovery mechanism when `skills.json` is absent from a remote repository. The repository `https://github.com/iMagdy/agent-skills` will serve as the test fixture for this scenario, as it lacks a `skills.json` manifest but contains a `skills/` directory with discoverable `.md` files.

**Why this priority**: This validates the spec 003 fallback feature end-to-end, ensuring the auto-discovery path works correctly against a real repository.

**Independent Test**: Can be executed by running the integration test suite targeting the fallback fixture repo and asserting discovery behavior.

**Acceptance Scenarios**:

1. **Given** a fresh project, **When** user runs `kt install` targeting `agent-skills` (no `skills.json`), **Then** system displays warning about missing manifest and auto-discovers skills from `skills/` directory
2. **Given** auto-discovery triggered, **When** multiple `.md` files exist in `skills/`, **Then** user is prompted to select which skill to install
3. **Given** auto-discovery triggered, **When** exactly one skill is found, **Then** installation proceeds without prompting
4. **Given** auto-discovery, **When** skill is selected and installed, **Then** the skill appears in `.agents/skills/` and is functional

---

### Edge Cases

- What happens when the fixture repository is temporarily unreachable? → **Resolved**: CI retries failed integration tests up to 3 times before marking as failed. This handles transient network issues while still catching real failures.
- What happens when the fixture repository structure changes (skills renamed/removed)? → **Resolved**: Integration tests pin to a specific commit SHA of the fixture repos to ensure reproducibility.
- What happens when coverage drops below 95% due to new untested code? → **Resolved**: CI must fail; no merge allowed until coverage returns to >=95%.
- What happens when integration tests conflict with rate limits on GitHub? → **Resolved**: Tests should use shallow clones and clean up after themselves; consider caching fixture repos locally.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST achieve >=95% line coverage across `src/` as measured by `cargo-tarpaulin`
- **FR-002**: System MUST include unit tests for all public functions and significant private functions
- **FR-003**: System MUST include integration tests for the `install` command's default path (repo with `skills.json`)
- **FR-004**: System MUST include integration tests for the `install` command's fallback path (repo without `skills.json`)
- **FR-005**: System MUST include integration tests for the `export` command
- **FR-006**: Integration tests MUST use `https://github.com/iMagdy/awesome-copilot.git` as the fixture for default install/export scenarios
- **FR-007**: Integration tests MUST use `https://github.com/iMagdy/agent-skills` as the fixture for fallback discovery scenarios
- **FR-008**: Integration tests MUST be tagged/segregated so they can be run independently from unit tests
- **FR-009**: Integration tests MUST clean up cloned repositories and generated files after execution
- **FR-010**: Integration tests MUST pin fixture repos to specific commit SHAs for reproducibility
- **FR-011**: System MUST maintain existing unit test coverage while adding integration tests
- **FR-012**: Coverage reporting MUST be integrated into CI/CD pipeline with a hard gate at 95%
- **FR-013**: System MUST test error paths (network failure, invalid manifest, missing skills directory) in integration tests
- **FR-014**: System MUST verify `skills.lock` correctness after install operations in integration tests
- **FR-015**: CI MUST retry failed integration tests up to 3 times before marking as failed to handle transient external issues

### Key Entities

- **Coverage Report**: Output from `cargo-tarpaulin` showing line/function/branch coverage percentages across `src/`
- **Fixture Repository**: A pinned version of a real git repository used as test input (awesome-copilot, agent-skills)
- **Integration Test**: An end-to-end test that exercises CLI commands against real or simulated external state
- **Unit Test**: A focused test validating individual functions or modules in isolation

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `cargo-tarpaulin` reports >=95% line coverage for `src/`
- **SC-002**: All integration tests pass against pinned fixture commit SHAs
- **SC-003**: Default install workflow completes successfully in integration test within 30 seconds
- **SC-004**: Fallback discovery workflow correctly identifies and installs skills from `agent-skills` fixture
- **SC-005**: Export command correctly generates `skills.json` listing installed skills in integration test
- **SC-006**: Integration test suite runs in isolation without leaving artifacts in the working directory
- **SC-007**: CI pipeline blocks merges when coverage drops below 95%
- **SC-008**: All existing unit tests continue to pass after integration test additions

## Assumptions

- `cargo-tarpaulin` is available or can be installed in the CI environment
- The fixture repositories (awesome-copilot, agent-skills) are publicly accessible
- Fixture repositories will not be deleted or renamed (they are maintained by the project owner)
- Shallow clones (`--depth 1`) are sufficient for integration test fixtures
- Network access is available during integration test execution
- The project uses `cargo test` as the test runner (standard for Rust projects)
- Integration tests will be placed in `tests/` directory following Rust conventions
- The existing unit test suite provides a foundation to build upon for coverage gaps

## Clarifications

### Session 2026-05-30

- Q: How should the CI pipeline handle integration tests that fail due to external factors (network timeouts, GitHub rate limits, fixture repo unavailability)? → A: Retry up to 3 times, then fail
