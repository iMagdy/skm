# Tasks: Ktesio CLI

**Input**: Design documents from `/specs/001-skills-pkg-manager/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Not requested in spec — omitted per rules.

**Organization**: Tasks grouped by user story for independent implementation.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

---

## Phase 1: Setup

**Purpose**: Initialize Rust project structure and dependencies

- [x] T001 Initialize Cargo project with binary crate structure per plan.md
- [x] T002 [P] Add dependencies to Cargo.toml: clap 4 (derive), miette 7 (fancy), indicatif 0.17, serde 1 (derive), serde_json 1, thiserror 2, walkdir 2
- [x] T003 [P] Create src/cli/mod.rs with module declarations for all subcommands
- [x] T004 [P] Create src/error.rs with all error types (InitError, ManifestError, LockError, GitError, SkillError) using miette diagnostics and thiserror derives

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 Implement Manifest struct and read/write functions in src/manifest.rs per contracts/manifest-format.md
- [x] T006 Implement Lockfile struct and read/write functions in src/lockfile.rs per contracts/lockfile-format.md
- [x] T007 [P] Implement git operations module in src/git.rs: clone, fetch, checkout, rev-parse HEAD, resolve default branch
- [x] T008 [P] Implement skill file copy module in src/skill.rs: read source repo exports, copy files/dirs, skip .git, fallback to full copy when exports empty/missing
- [x] T009 [P] Add manifest validation in src/manifest.rs: JSON parse errors with line/column, duplicate name detection, name format validation (`^[a-zA-Z0-9_-]+$`)

**Checkpoint**: Foundation ready — user story implementation can begin

---

## Phase 3: User Story 1 — Initialize a Skills Manifest (Priority: P1) 🎯 MVP

**Goal**: `kt init .` creates a valid `skills.json` with empty `skills` and `exports` objects

**Independent Test**: Run `kt init .` in empty dir, verify `skills.json` is created with `{ "skills": {}, "exports": {} }`

### Implementation for User Story 1

- [x] T010 [US1] Implement kt init command in src/cli/init.rs: parse path arg, create skills.json, handle exists/not-found errors
- [x] T011 [US1] Wire init subcommand into clap CLI in src/main.rs
- [ ] T012 [US1] Add integration test for init in tests/integration/init_test.rs: create, exists-warning, not-found-error

**Checkpoint**: `kt init .` works — can create manifest

---

## Phase 4: User Story 2 — Install All Declared Skills (Priority: P1)

**Goal**: `kt install` reads skills.json, clones all skills, copies exports, creates skills.lock

**Independent Test**: Create skills.json with a public repo, run `kt install`, verify skill in .agents/skills/ and skills.lock exists

### Implementation for User Story 2

- [x] T013 [US2] Implement kt install (bulk) command in src/cli/install.rs: read manifest, loop skills, clone, copy exports, write lockfile, skip already-installed, handle partial failures
- [x] T014 [US2] Wire install subcommand (no args) into clap CLI in src/main.rs
- [x] T015 [US2] Add progress indicators using indicatif MultiProgress in src/cli/install.rs: spinner per skill during clone
- [ ] T016 [US2] Add integration test for bulk install in tests/integration/install_test.rs: install from manifest, skip existing, partial failure, missing manifest

**Checkpoint**: `kt install` fully works — core value proposition delivered

---

## Phase 5: User Story 3 — Install a Specific Skill (Priority: P2)

**Goal**: `kt install <name:url>` adds one skill to manifest, clones it, updates lockfile

**Independent Test**: Run `kt install myskill:https://github.com/example/repo.git`, verify added to skills.json, cloned, and locked

### Implementation for User Story 3

- [x] T017 [US3] Extend src/cli/install.rs to parse `name:url` argument, add skill to manifest, create manifest if missing, warn if duplicate
- [x] T018 [US3] Wire install subcommand (with arg) into clap CLI in src/main.rs
- [ ] T019 [US3] Add integration test for single install in tests/integration/install_test.rs: install single, duplicate warning, auto-create manifest

**Checkpoint**: `kt install <name:url>` works — can add individual skills

---

## Phase 6: User Story 4 — List Installed Skills (Priority: P2)

**Goal**: `kt list` displays table of installed skills with name, repo, commit, status

**Independent Test**: Run `kt list` after installing skills, verify table output with correct status

### Implementation for User Story 4

- [x] T020 [US4] Implement kt list command in src/cli/list.rs: read manifest + lockfile, check disk, format table, handle empty case
- [x] T021 [US4] Wire list subcommand into clap CLI in src/main.rs
- [ ] T022 [US4] Add integration test for list in tests/integration/list_test.rs: list installed, list empty, list with missing dirs

**Checkpoint**: `kt list` shows installed skills with status

---

## Phase 7: User Story 5 — Show Skill Details (Priority: P3)

**Goal**: `kt show <name>` displays repo URL, commit, path, status for one skill

**Independent Test**: Run `kt show <installed-skill>`, verify details printed; run `kt show <unknown>`, verify error

### Implementation for User Story 5

- [x] T023 [US5] Implement kt show command in src/cli/show.rs: lookup in lockfile, check disk, format output, handle not-found
- [x] T024 [US5] Wire show subcommand into clap CLI in src/main.rs
- [ ] T025 [US5] Add integration test for show in tests/integration/show_test.rs: show installed, show not-found

**Checkpoint**: `kt show <name>` inspects individual skills

---

## Phase 8: User Story 6 — Upgrade Skills (Priority: P2)

**Goal**: `kt upgrade` fetches latest HEAD for each skill, updates lockfile

**Independent Test**: Run `kt upgrade` after upstream has new commits, verify lockfile commit hashes updated

### Implementation for User Story 6

- [x] T026 [US6] Implement kt upgrade command in src/cli/upgrade.rs: read lockfile (fallback to manifest), fetch+checkout per skill, resolve new HEAD, update lockfile, handle partial failures
- [x] T027 [US6] Wire upgrade subcommand into clap CLI in src/main.rs
- [x] T028 [US6] Add progress indicators using indicatif MultiProgress in src/cli/upgrade.rs: spinner per skill during fetch
- [ ] T029 [US6] Add integration test for upgrade in tests/integration/upgrade_test.rs: upgrade all, upgrade with unreachable repo, upgrade without lockfile

**Checkpoint**: `kt upgrade` keeps skills current

---

## Phase 9: User Story 7 — Uninstall a Skill (Priority: P2)

**Goal**: `kt uninstall <name>` / `kt remove <name>` removes skill from manifest, lockfile, and disk

**Independent Test**: Run `kt uninstall <skill>`, verify removed from skills.json, skills.lock, and .agents/skills/

### Implementation for User Story 7

- [x] T030 [US7] Implement kt uninstall command in src/cli/uninstall.rs: remove from manifest, remove from lockfile, delete directory, handle not-found
- [x] T031 [US7] Wire uninstall subcommand (alias: remove) into clap CLI in src/main.rs
- [ ] T032 [US7] Add integration test for uninstall in tests/integration/uninstall_test.rs: uninstall success, uninstall not-found

**Checkpoint**: `kt uninstall` / `kt remove` cleans up skills

---

## Phase 10: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T033 [P] Ensure all error messages include failure reason + remediation per FR-011/SC-004
- [ ] T034 [P] Validate malformed JSON handling: test skills.json with syntax errors, missing keys, wrong types in tests/integration/
- [ ] T035 [P] Validate duplicate name detection: test skills.json with duplicate skill names in tests/integration/
- [ ] T036 [P] Validate network error handling: test clone/fetch failures with unreachable repos in tests/integration/
- [ ] T037 [P] Validate stale/untracked directory detection in `kt list` in tests/integration/
- [ ] T038 [P] Validate auth failure handling: test git URLs requiring auth in tests/integration/
- [ ] T039 Benchmark init command to verify SC-001 (<5s)
- [ ] T040 Benchmark single skill install to verify SC-002 (<30s)
- [ ] T041 Benchmark list command to verify SC-003 (<1s)
- [ ] T042 Run quickstart.md end-to-end validation: cargo build, kt init, kt install, kt list, kt show, kt upgrade, kt uninstall

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Foundational
- **US2 (Phase 4)**: Depends on Foundational + US1 (needs manifest format, but can use test fixtures)
- **US3–US7 (Phases 5–9)**: Each depends on Foundational; independent of each other after US2 provides installed skills

### User Story Dependencies

- **US1 (init)**: No story dependencies — needs only Foundational
- **US2 (install)**: Depends on US1 conceptually (needs manifest), but can test with fixtures
- **US3 (install specific)**: Depends on US1 (creates manifest)
- **US4 (list)**: Depends on US2 (needs installed skills to list)
- **US5 (show)**: Depends on US2 (needs installed skills to show)
- **US6 (upgrade)**: Depends on US2 (needs installed skills to upgrade)
- **US7 (uninstall)**: Depends on US2 (needs installed skills to uninstall)

### Parallel Opportunities

- T002, T003, T004 (Setup) — parallel
- T007, T008 (Foundational) — parallel
- T010, T011 (US1) — parallel after T005
- US3, US4, US5, US6, US7 — can all start in parallel once US2 is done

---

## Parallel Example: User Story 2

```bash
# After Foundational phase completes, US2 tasks run sequentially:
Task: T013 — Implement bulk install in src/cli/install.rs
Task: T014 — Wire into main.rs
Task: T015 — Add progress indicators
Task: T016 — Integration tests
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: US1 (init)
4. Complete Phase 4: US2 (install)
5. **STOP and VALIDATE**: `kt init .` + `kt install` workflow works
6. Deploy/demo if ready

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. US1 + US2 → MVP: init + install workflow
3. US3 → Add single-skill install convenience
4. US4 + US5 → Add visibility (list, show)
5. US6 → Add upgrade workflow
6. US7 → Add cleanup workflow
7. Polish → Production-ready

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story
- Each user story phase is independently completable after Foundational
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
