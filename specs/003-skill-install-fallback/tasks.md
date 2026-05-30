# Tasks: Skill Install Fallback

**Input**: Design documents from `/specs/003-skill-install-fallback/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Tests included for all user stories (as per constitution requirement for >=95% coverage).

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project dependencies and error types

- [x] T001 Add dialoguer dependency to Cargo.toml for interactive prompts
- [x] T002 [P] Add new error types to src/error.rs: DiscoveryError, SkillsDirectoryEmpty, UserCancelled

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core discovery module that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T003 Create src/discovery.rs module with DiscoveredSkill struct, SkillType enum, and DiscoveryResult struct per data-model.md
- [x] T004 [P] Implement find_skills_directory function: normalize dir names to lowercase, search for "skills" (FR-012)
- [x] T005 [P] Implement discover_skills function: scan directory for .md files and subdirectories (FR-005, FR-006)
- [x] T006 Implement normalize_skill_name function: strip .md extension, replace hyphens/underscores with spaces
- [x] T007 Implement deduplicate_skills function: keep first occurrence, collect duplicate warnings (FR-014)
- [x] T008 Create tests/unit/discovery.rs with unit tests for all discovery functions

**Checkpoint**: Foundation ready - discovery logic complete and tested

---

## Phase 3: User Story 1 - Install Skill from Repo with Missing Manifest (Priority: P1) 🎯 MVP

**Goal**: Enable installing skills from repos without skills.json by discovering skills from skills/ directory

**Independent Test**: Attempt install from repo without skills.json but with skills/ directory containing multiple .md files. User sees warning and selection prompt, then can install chosen skill.

### Implementation for User Story 1

- [x] T009 [US1] Modify src/cli/install.rs run_bulk function to check for manifest existence (FR-001)
- [x] T010 [US1] Add fallback path in run_bulk when manifest not found: call discovery functions (FR-002)
- [x] T011 [US1] Implement display_warning_messages function: show missing manifest warning and discovery in progress (FR-003, FR-004)
- [x] T012 [US1] Implement prompt_user_selection function using dialoguer: display numbered list, accept numeric input (FR-007, FR-008)
- [x] T013 [US1] Handle empty skills directory case: display error message (FR-011)
- [x] T014 [US1] Handle user cancellation: display "Installation cancelled" message (FR-013)
- [x] T015 [US1] Integrate selection result with git::clone and skill::copy_cloned_repo_to_dest (FR-009)
- [x] T016 [US1] Create tests/integration/install_fallback.rs with integration tests for full fallback flow

**Checkpoint**: User Story 1 fully functional - can install from repos without manifest

---

## Phase 4: User Story 2 - Warning Communication (Priority: P2)

**Goal**: Clear communication when auto-discovery is used instead of manifest

**Independent Test**: Trigger any auto-discovery scenario and verify warning messages appear before selection prompt.

### Implementation for User Story 2

- [x] T017 [P] [US2] Verify warning messages in src/cli/install.rs match CLI contract output format (contracts/cli.md)
- [x] T018 [US2] Add integration tests for warning message verification in tests/integration/install_fallback.rs

**Checkpoint**: User Story 2 complete - all warning scenarios tested

---

## Phase 5: User Story 3 - Single Skill Auto-Selection (Priority: P3)

**Goal**: Skip selection prompt when only one skill is discovered

**Independent Test**: Attempt install from repo with exactly one skill in skills/ directory - should install without prompting.

### Implementation for User Story 3

- [x] T019 [US3] Add single-skill auto-select logic in src/cli/install.rs: if skills.len() == 1, skip prompt (FR-010)
- [x] T020 [US3] Add integration test for single-skill auto-selection in tests/integration/install_fallback.rs

**Checkpoint**: All user stories complete

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final improvements and validation

- [x] T021 [P] Update docs/README.md with fallback installation documentation
- [x] T022 [P] Run quickstart.md validation against actual CLI behavior
- [x] T023 Run cargo test and verify >=95% coverage with cargo-tarpaulin
- [x] T024 Run cargo clippy and fix any warnings
- [x] T025 Verify cross-platform path handling with std::path::Path APIs

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-5)**: All depend on Foundational phase completion
  - US1 must complete before US2 and US3 (they build on US1 infrastructure)
  - US2 and US3 can potentially run in parallel after US1
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Depends on US1 (uses same warning infrastructure)
- **User Story 3 (P3)**: Depends on US1 (uses same selection infrastructure)

### Within Each User Story

- Models/structs before services/functions
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- T002 [P]: Error types can be added in parallel with module creation
- T004, T005 [P]: Directory finding and skill discovery can be implemented in parallel
- T017, T018 [P]: Warning message refinements can be done in parallel
- T022, T023 [P]: Documentation tasks can be done in parallel

---

## Parallel Example: User Story 1

```bash
# Sequential dependency chain:
T003 (discovery.rs module) → T006 (normalize_skill_name) → T007 (deduplicate_skills)

# Parallel within US1:
T009, T010 (manifest check + fallback) can be done together
T013, T014 (error cases) can be done together
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T002)
2. Complete Phase 2: Foundational (T003-T008) - CRITICAL
3. Complete Phase 3: User Story 1 (T009-T016)
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Demo fallback installation capability

### Incremental Delivery

1. Complete Setup + Foundational → Discovery module ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo (better UX)
4. Add User Story 3 → Test independently → Deploy/Demo (convenience)
5. Each story adds value without breaking previous stories

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Constitution requires >=95% test coverage - all new code must have tests
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
