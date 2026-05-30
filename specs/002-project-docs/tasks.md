# Tasks: Project Documentation

**Input**: Design documents from `/specs/002-project-docs/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Not applicable — this is a documentation-only feature. Validation is via CI link checking and manual review.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the docs/ directory structure and foundational files

- [X] T001 Initialize docs/ directory structure per implementation plan
- [X] T002 [P] Create docs/README.md with table of contents skeleton in docs/README.md
- [X] T003 Update root README.md to link into docs/ as entry point in README.md

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core documentation files that all user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T004 [P] Write installation guide covering all supported platforms in docs/installation.md
- [X] T005 [P] Write command reference documenting all skm subcommands in docs/commands.md
- [X] T006 [P] Write manifest format documentation with field descriptions in docs/manifest.md
- [X] T007 [P] Write lockfile format documentation with field descriptions in docs/lockfile.md

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Discover and Navigate Documentation (Priority: P1) 🎯 MVP

**Goal**: A developer new to skm can find, read, and navigate the documentation to understand what skm does, how to install it, and how to use it.

**Independent Test**: Verify docs/ contains a table of contents, all linked files exist, and each document covers its stated topic without gaps.

### Validation for User Story 1

> Phase 2 created initial content. Phase 3 validates against contract specs and adds examples/completeness.

- [X] T008 [P] [US1] Validate docs/README.md table of contents links all sections per contracts/documentation-contracts.md in docs/README.md
- [X] T009 [US1] Validate docs/installation.md covers all platforms and includes verification steps per contracts/documentation-contracts.md in docs/installation.md
- [X] T010 [US1] Validate docs/commands.md includes examples for every subcommand per contracts/documentation-contracts.md in docs/commands.md
- [X] T011 [US1] Add complete JSON examples to docs/manifest.md with valid 2-space indented samples in docs/manifest.md
- [X] T012 [US1] Add complete JSON examples to docs/lockfile.md with valid 2-space indented samples in docs/lockfile.md

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Contribute to the Project (Priority: P2)

**Goal**: An open source contributor can understand the project's architecture, development workflow, and coding standards to submit meaningful contributions.

**Independent Test**: Verify contributor docs explain dev setup, testing commands, and PR process. A new contributor should be able to clone the repo and run tests by following the docs alone.

### Implementation for User Story 2

- [X] T013 [P] [US2] Write contributing guide with dev setup, coding standards, and PR workflow in docs/contributing.md
- [X] T014 [US2] Document testing guide with test execution and coverage verification in docs/testing.md

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Understand the Architecture (Priority: P3)

**Goal**: A developer extending skm can understand the codebase architecture, module structure, and data flows to make informed contributions.

**Independent Test**: Verify architecture docs accurately describe the module layout, data flow between manifest/lockfile/git operations, and key design patterns.

### Implementation for User Story 3

- [X] T015 [P] [US3] Write architecture guide describing module structure and data flows in docs/architecture.md
- [X] T016 [US3] Document data flow for `skm install` command sequence in docs/architecture.md
- [X] T017 [US3] Document data flow for `skm upgrade` command sequence in docs/architecture.md

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: User Story 4 - Verify Test Coverage Compliance (Priority: P1)

**Goal**: A maintainer can verify the project meets the constitution's >=95% test coverage requirement and enforce it in CI.

**Independent Test**: Run the documented coverage commands and confirm they produce output showing >=95% line coverage against src/.

### Implementation for User Story 4

- [X] T018 [P] [US4] Document cargo-tarpaulin or equivalent coverage measurement in docs/testing.md
- [X] T019 [P] [US4] Document CI coverage enforcement as a gate before merge in docs/testing.md
- [X] T020 [US4] Document steps to investigate and fix coverage gaps in docs/testing.md
- [X] T021 [US4] Document CI job for broken link checking and file presence validation in docs/testing.md

**Checkpoint**: All user stories complete — documentation set fully implemented

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final validation and cross-cutting improvements

- [X] T022 Validate all internal links between docs/ files are correct
- [X] T023 Verify docs/README.md table of contents links resolve to valid files
- [X] T024 Verify root README.md links into docs/ correctly
- [X] T025 Cross-reference command documentation against actual src/cli/ implementation
- [X] T026 Validate JSON examples in manifest.md and lockfile.md are valid
- [X] T027 Run quickstart.md validation — follow steps and verify they work
- [X] T028 [P] Document graceful degradation behavior and error handling patterns in docs/architecture.md (FR-015)
- [X] T029 [P] Document cross-platform compatibility requirements and platform-specific considerations in docs/installation.md (FR-014)
- [X] T030 [P] Define documentation review process for accuracy verification in docs/contributing.md (SC-005)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - US1 and US4 (both P1) can proceed in parallel
  - US2 (P2) can proceed after Foundational
  - US3 (P3) can proceed after Foundational
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Independent of US1
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Independent of US1/US2
- **User Story 4 (P1)**: Can start after Foundational (Phase 2) - Independent of US1/US2/US3

### Within Each User Story

- Documentation files are independent and can be written in any order
- No code dependencies between stories
- Each story can be validated independently

### Parallel Opportunities

- All Phase 1 tasks marked [P] can run in parallel
- All Phase 2 tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel
- US1 and US4 can run in parallel (both P1, independent)
- US2 and US3 can run in parallel (both independent)
- All tasks within a story marked [P] can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all foundational doc tasks for US1 together:
Task: "Finalize docs/README.md with complete table of contents in docs/README.md"
Task: "Review and finalize docs/installation.md in docs/installation.md"
Task: "Review and finalize docs/commands.md in docs/commands.md"
Task: "Review and finalize docs/manifest.md in docs/manifest.md"
Task: "Review and finalize docs/lockfile.md in docs/lockfile.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test that docs/ contains a working table of contents with all linked files
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Discover & Navigate)
   - Developer B: User Story 2 (Contribute)
   - Developer C: User Story 4 (Test Coverage)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Documentation is plain Markdown — no YAML frontmatter
- All JSON examples must be valid and use 2-space indentation
