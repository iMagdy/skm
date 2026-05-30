# Tasks: GitHub CI Pipeline

**Input**: Design documents from `/specs/005-github-ci-pipeline/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Not requested for this feature. CI pipeline is validated by running it on a real PR.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the workflow directory and base file structure

- [x] T001 Create `.github/workflows/` directory structure
- [x] T002 Create `.github/workflows/ci.yml` with workflow name and trigger configuration in `.github/workflows/ci.yml`

---

## Phase 2: User Story 1 - Automated Quality Gate on Pull Requests (Priority: P1) MVP

**Goal**: CI pipeline triggers on PR creation and runs lint, test, build checks with pass/fail status, individual check visibility, and automatic re-trigger on push

**Independent Test**: Create a PR with passing code → pipeline reports success with individual check statuses. Push new commit → pipeline re-triggers. Create a PR with lint error → pipeline reports failure.

### Implementation for User Story 1

- [x] T003 [US1] Define `pull_request` trigger with `opened`, `synchronize`, `reopened` types in `.github/workflows/ci.yml`
- [x] T004 [US1] Add concurrency group `${{ github.workflow }}-${{ github.head_ref }}` with `cancel-in-progress: true` in `.github/workflows/ci.yml`
- [x] T005 [P] [US1] Implement `lint` job with `actions/checkout@v4`, `dtolnay/rust-toolchain@stable`, `actions/cache@v4`, and `cargo clippy -- -D warnings` in `.github/workflows/ci.yml`
- [x] T006 [P] [US1] Implement `test` job with `actions/checkout@v4`, `dtolnay/rust-toolchain@stable`, `actions/cache@v4`, and `cargo test` in `.github/workflows/ci.yml`
- [x] T007 [P] [US1] Implement `build` job with `actions/checkout@v4`, `dtolnay/rust-toolchain@stable`, `actions/cache@v4`, and `cargo build --release` in `.github/workflows/ci.yml`
- [x] T008 [US1] Add `timeout-minutes: 15` to all three jobs in `.github/workflows/ci.yml`
- [x] T009 [US1] Configure cache key `${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}` with restore-keys fallback in `.github/workflows/ci.yml`
- [x] T010 [US1] Ensure each job has descriptive `name` field (`lint`, `test`, `build`) for individual check visibility in `.github/workflows/ci.yml`

**Checkpoint**: At this point, User Story 1 should be fully functional — pipeline triggers on PRs, reports individual check status with logs, and re-triggers on new pushes

---

## Phase 3: User Story 2 - Branch Protection Enforcement (Priority: P2)

**Goal**: Document how to configure branch protection rules requiring all checks to pass

**Independent Test**: Enable branch protection → PR with failing check cannot be merged.

### Implementation for User Story 2

- [x] T011 [US2] Add branch protection documentation to `specs/005-github-ci-pipeline/quickstart.md` with required check names (`lint`, `test`, `build`)

**Note**: Branch protection is configured via GitHub repository settings, not in the workflow file. This task documents the configuration steps.

**Checkpoint**: User Stories 1 AND 2 should both work — pipeline runs with full visibility and branch protection can be configured

---

## Phase 4: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, validation, and coverage enforcement

- [x] T012 [P] Add coverage gate job with `cargo-tarpaulin --fail-under 95` to enforce constitution Principle VI in `.github/workflows/ci.yml`
- [x] T013 [P] Add CI status badge to `README.md` with workflow URL
- [x] T014 Validate `quickstart.md` instructions match actual workflow behavior
- [x] T015 [P] Add fork PR documentation to `specs/005-github-ci-pipeline/quickstart.md` with instructions for enabling fork workflows
- [x] T016 Update `docs/` directory if applicable (per constitution Principle VII)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **User Story 1 (Phase 2)**: Depends on Setup completion - MVP
- **User Story 2 (Phase 3)**: Depends on User Story 1 (documents check names from same workflow)
- **Polish (Phase 4)**: Depends on User Story 1 being complete; T012 (coverage gate) can be added in parallel with US2

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Setup - No dependencies on other stories
- **User Story 2 (P2)**: Extends US1 (documents branch protection using check names from US1)

### Within Each User Story

- Trigger configuration before jobs (defines when pipeline runs)
- Jobs can be implemented in parallel (lint, test, build are independent)
- Caching added after jobs are defined
- Timeout added last (applies to all jobs)

### Parallel Opportunities

- T005, T006, T007 (lint, test, build jobs) can be implemented in parallel
- T012, T013, T015 (coverage gate, badge, fork docs) can be implemented in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all three jobs together (after trigger is defined):
Task: "Implement lint job in .github/workflows/ci.yml"
Task: "Implement test job in .github/workflows/ci.yml"
Task: "Implement build job in .github/workflows/ci.yml"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (create directory and file)
2. Complete Phase 2: User Story 1 (implement all three jobs with visibility and re-trigger)
3. **STOP and VALIDATE**: Create a test PR to verify pipeline triggers, reports individual check status, and re-triggers on push
4. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup → Workflow file created
2. Add User Story 1 → Test with real PR → Deploy (MVP!)
3. Add User Story 2 → Document branch protection → Done
4. Polish → Add coverage gate, badge, fork docs → Complete

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- This is a CI configuration feature — all tasks modify `.github/workflows/ci.yml` except documentation tasks
- GitHub Actions provides status reporting, log access, and cancellation automatically — many "requirements" are fulfilled by the platform itself
- Coverage gate (T012) enforces constitution Principle VI (>=95% line coverage)
