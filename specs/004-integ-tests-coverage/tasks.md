# Tasks: Test Coverage & Integration Tests

**Input**: Design documents from `/specs/004-integ-tests-coverage/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: This feature IS tests — all tasks are test-related.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Source**: `src/` (Rust modules with co-located unit tests)
- **Integration tests**: `tests/integration/` (separate from unit tests)
- **CI**: `.github/workflows/` (GitHub Actions)

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add test dependencies and establish test infrastructure

- [x] T001 [P] Add `tempfile` crate to `[dev-dependencies]` in Cargo.toml for test directory isolation
- [x] T002 Verify `cargo test` still passes after dependency addition

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish coverage baseline and test helpers before user stories

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T003 Run `cargo tarpaulin` to establish current coverage baseline
- [x] T004 [P] Create shared test helpers module at tests/integration/helpers/mod.rs
- [x] T005 [P] Define fixture constants (AWESOME_COPILOT_SHA, AGENT_SKILLS_SHA) in tests/integration/helpers/mod.rs
- [x] T006 [P] Implement temp directory cleanup helper using tempfile in tests/integration/helpers/mod.rs
- [x] T007 [P] Implement git clone helper function in tests/integration/helpers/mod.rs

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Achieve 95% Test Coverage (Priority: P1) 🎯 MVP

**Goal**: Achieve >=95% line coverage across src/ as required by constitution Principle VI

**Independent Test**: Run `cargo tarpaulin --fail-under 95` and verify it exits with code 0

### Implementation for User Story 1

- [x] T008 [P] [US1] Add unit tests for error.rs uncovered paths in src/error.rs
- [x] T009 [P] [US1] Add unit tests for git.rs uncovered paths in src/git.rs
- [x] T010 [P] [US1] Add unit tests for manifest.rs uncovered paths in src/manifest.rs
- [x] T011 [P] [US1] Add unit tests for lockfile.rs uncovered paths in src/lockfile.rs
- [x] T012 [P] [US1] Add unit tests for discovery.rs uncovered paths in src/discovery.rs
- [x] T013 [P] [US1] Add unit tests for skill.rs uncovered paths in src/skill.rs
- [x] T014 [P] [US1] Add unit tests for cli/install.rs uncovered paths in src/cli/install.rs
- [x] T015 [P] [US1] Add unit tests for cli/init.rs uncovered paths in src/cli/init.rs
- [x] T016 [P] [US1] Add unit tests for cli/list.rs uncovered paths in src/cli/list.rs
- [x] T017 [P] [US1] Add unit tests for cli/show.rs uncovered paths in src/cli/show.rs
- [x] T018 [P] [US1] Add unit tests for cli/uninstall.rs uncovered paths in src/cli/uninstall.rs
- [x] T019 [P] [US1] Add unit tests for cli/upgrade.rs uncovered paths in src/cli/upgrade.rs
- [x] T020 [US1] Run coverage verification: cargo tarpaulin --fail-under 95

**Checkpoint**: Coverage >=95% verified — constitutional requirement satisfied

---

## Phase 4: User Story 2 - Integration Test: Default Install & Export (Priority: P2)

**Goal**: Validate install and export workflows against awesome-copilot fixture repo

**Independent Test**: Run `cargo test --test install_default --test export` and verify all tests pass

### Tests for User Story 2

- [x] T021 [P] [US2] Create default install test file at tests/integration/install_default.rs
- [x] T022 [P] [US2] Add test_install_single_skill_creates_directory in tests/integration/install_default.rs
- [x] T023 [P] [US2] Add test_install_single_skill_updates_lockfile with SHA/name/source validation in tests/integration/install_default.rs
- [x] T024 [P] [US2] Add test_install_completes_within_30_seconds in tests/integration/install_default.rs
- [x] T025 [P] [US2] Add test_install_clones_correct_content in tests/integration/install_default.rs
- [x] T026 [P] [US2] Add test_install_does_not_modify_existing_skills in tests/integration/install_default.rs
- [x] T027 [P] [US2] Create export test file at tests/integration/export.rs
- [x] T028 [P] [US2] Add test_export_creates_skills_json in tests/integration/export.rs
- [x] T029 [P] [US2] Add test_export_manifest_contains_skills_key in tests/integration/export.rs
- [x] T030 [P] [US2] Add test_export_manifest_contains_exports_key in tests/integration/export.rs
- [x] T031 [P] [US2] Add test_export_lists_skill_with_correct_source in tests/integration/export.rs
- [x] T032 [P] [US2] Add test_export_manifest_valid_json_2space_indent in tests/integration/export.rs
- [x] T033 [P] [US2] Add test_export_updates_existing_manifest in tests/integration/export.rs
- [x] T034 [P] [US2] Add test_install_error_network_failure in tests/integration/install_default.rs
- [x] T035 [P] [US2] Add test_install_error_invalid_manifest in tests/integration/install_default.rs
- [x] T036 [US2] Run integration tests: cargo test --test install_default --test export

**Checkpoint**: Default install and export workflows validated end-to-end

---

## Phase 5: User Story 3 - Integration Test: Fallback Discovery (Priority: P3)

**Goal**: Validate fallback skill discovery against agent-skills fixture repo (no skills.json)

**Independent Test**: Run `cargo test --test install_fallback` and verify all tests pass

### Tests for User Story 3

- [x] T037 [P] [US3] Create fallback install test file at tests/integration/install_fallback.rs
- [x] T038 [P] [US3] Add test_install_fallback_displays_warning in tests/integration/install_fallback.rs
- [x] T039 [P] [US3] Add test_install_fallback_discovers_skills_directory in tests/integration/install_fallback.rs
- [x] T040 [P] [US3] Add test_install_fallback_prompts_when_multiple_skills in tests/integration/install_fallback.rs
- [x] T041 [P] [US3] Add test_install_fallback_auto_selects_single_skill in tests/integration/install_fallback.rs
- [x] T042 [P] [US3] Add test_install_fallback_installs_selected_skill in tests/integration/install_fallback.rs
- [x] T043 [P] [US3] Add test_install_fallback_updates_lockfile with SHA/name/source validation in tests/integration/install_fallback.rs
- [x] T044 [P] [US3] Add test_install_fallback_error_missing_skills_dir in tests/integration/install_fallback.rs
- [x] T045 [P] [US3] Add test_install_fallback_error_empty_skills_dir in tests/integration/install_fallback.rs
- [x] T046 [US3] Run integration tests: cargo test --test install_fallback

**Checkpoint**: Fallback discovery workflow validated end-to-end

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: CI integration, documentation, and final validation

- [x] T047 [P] Add CI workflow job for integration tests with GitHub Actions retry action (nick-fields/retry@v3) in .github/workflows/ci.yml
- [x] T048 [P] Add CI coverage gate step with `cargo tarpaulin --fail-under 95` in .github/workflows/ci.yml
- [x] T049 [P] Update quickstart.md with final test commands in docs/quickstart.md (or specs/004-integ-tests-coverage/quickstart.md if docs/ doesn't exist)
- [x] T050 Run full test suite: cargo test
- [x] T051 Run final coverage verification: cargo tarpaulin --fail-under 95 (final validation, distinct from T020 incremental check)
- [x] T052 Run integration tests: cargo test --test '*'

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational (Phase 2) — can start immediately after
- **User Story 2 (Phase 4)**: Depends on Foundational (Phase 2) — can run in parallel with US1
- **User Story 3 (Phase 5)**: Depends on Foundational (Phase 2) — can run in parallel with US1/US2
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) — No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) — Independent of US1 and US3
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) — Independent of US1 and US2

### Within Each User Story

- Tests written first (RED phase)
- Implementation follows (GREEN phase)
- Verify tests pass
- Move to next task

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, US1/US2/US3 can start in parallel
- All tests for a user story marked [P] can run in parallel
- Polish tasks marked [P] can run in parallel

---

## Parallel Example: User Story 2

```bash
# Launch all tests for User Story 2 together:
Task: "Create default install test file at tests/integration/install_default.rs"
Task: "Create export test file at tests/integration/export.rs"

# Then implement all test functions in parallel:
Task: "Add test_install_single_skill_creates_directory in tests/integration/install_default.rs"
Task: "Add test_install_single_skill_updates_lockfile in tests/integration/install_default.rs"
Task: "Add test_export_creates_skills_json in tests/integration/export.rs"
Task: "Add test_export_manifest_contains_skills_key in tests/integration/export.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL — blocks all stories)
3. Complete Phase 3: User Story 1 (achieve 95% coverage)
4. **STOP and VALIDATE**: Run `cargo tarpaulin --fail-under 95`
5. Constitutional requirement satisfied

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Verify coverage → Constitutional requirement met (MVP!)
3. Add User Story 2 → Verify install/export tests → Core workflows validated
4. Add User Story 3 → Verify fallback tests → Complete coverage
5. Add Polish → CI integration → Production ready

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (coverage gaps)
   - Developer B: User Story 2 (install/export tests)
   - Developer C: User Story 3 (fallback tests)
3. Stories complete and integrate independently
4. Team completes Polish together

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing (RED-GREEN-REFACTOR)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Fixture repos pinned to commit SHAs for reproducibility
- Integration tests use tempfile for automatic cleanup
