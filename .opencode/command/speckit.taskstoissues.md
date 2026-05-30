---
description: Convert existing tasks into actionable, dependency-ordered GitHub issues for the feature based on available design artifacts. Creates issues with task checklists and saves an issue map for progress tracking.
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Pre-Execution Checks

**Check for extension hooks (before tasks-to-issues conversion)**:
- Check if `.specify/extensions.yml` exists in the project root.
- If it exists, read it and look for entries under the `hooks.before_taskstoissues` key
- If the YAML cannot be parsed or is invalid, skip hook checking silently and continue normally
- Filter out hooks where `enabled` is explicitly `false`. Treat hooks without an `enabled` field as enabled by default.
- For each remaining hook, do **not** attempt to interpret or evaluate hook `condition` expressions:
  - If the hook has no `condition` field, or it is null/empty, treat the hook as executable
  - If the hook defines a non-empty `condition`, skip the hook and leave condition evaluation to the HookExecutor implementation
- For each executable hook, output the following based on its `optional` flag:
  - **Optional hook** (`optional: true`):
    ```
    ## Extension Hooks

    **Optional Pre-Hook**: {extension}
    Command: `/{command}`
    Description: {description}

    Prompt: {prompt}
    To execute: `/{command}`
    ```
  - **Mandatory hook** (`optional: false`):
    ```
    ## Extension Hooks

    **Automatic Pre-Hook**: {extension}
    Executing: `/{command}`
    EXECUTE_COMMAND: {command}

    Wait for the result of the hook command before proceeding to the Outline.
    ```
- If no hooks are registered or `.specify/extensions.yml` does not exist, skip silently

## Outline

1. Run `.specify/scripts/bash/check-prerequisites.sh --json --require-tasks --include-tasks` from repo root and parse FEATURE_DIR and AVAILABLE_DOCS list. All paths must be absolute. For single quotes in args like "I'm Groot", use escape syntax: e.g 'I'\''m Groot' (or double-quote if possible: "I'm Groot").
1. From the executed script, extract the path to **tasks**.
1. Get the Git remote by running:

```bash
gh repo view --json nameWithOwner -q '.nameWithOwner'
```

> [!CAUTION]
> ONLY PROCEED TO NEXT STEPS IF THE COMMAND SUCCEEDS (indicates a GitHub remote)

1. Read the tasks.md file and parse all tasks. Each task follows the format:
   ```text
   - [ ] T001 [P] [US1] Description with file path
   ```
1. Group tasks by their user story label ([US1], [US2], etc.). Tasks without a story label belong to shared phases (Setup, Foundational, Polish).
1. For each user story group, create a **single GitHub issue** using `gh` CLI:

   ```bash
   gh issue create \
     --title "[FEATURE_NAME] User Story N: [STORY_TITLE]" \
     --label "story" \
     --body "## User Story N: [STORY_TITLE]

   **Priority**: P1/P2/P3
   **Spec**: [SPEC_PATH]

   ### Tasks

   - [ ] T001 Description
   - [ ] T005 [P] Description
   ...

   ### Acceptance Criteria

   - [Criteria from spec.md for this story]
   "
   ```

   For tasks without a user story (Setup, Foundational, Polish), create a **single shared issue**:

   ```bash
   gh issue create \
     --title "[FEATURE_NAME] Setup & Infrastructure" \
     --label "infrastructure" \
     --body "## Setup & Infrastructure Tasks

   ### Tasks

   - [ ] T001 Description
   - [ ] T002 Description
   ...
   "
   ```

1. After creating each issue, capture the issue number from the output.
1. Save an **issue map** to `.specify/issue-map.json` in the feature directory:

   ```json
   {
     "feature": "FEATURE_NAME",
     "repo": "OWNER/REPO",
     "created_at": "ISO_TIMESTAMP",
     "issues": {
       "US1": { "number": 123, "title": "...", "url": "..." },
       "US2": { "number": 124, "title": "...", "url": "..." },
       "shared": { "number": 125, "title": "...", "url": "..." }
     },
     "task_to_issue": {
       "T001": 125,
       "T002": 125,
       "T005": 123,
       "T006": 123,
       "T010": 124
     }
   }
   ```

   This mapping is used by `/speckit.implement` to update issue checkboxes as tasks are completed.

1. **Report**: Output a summary table:

   ```text
   | Story | Issue # | Title | Tasks |
   |-------|---------|-------|-------|
   | US1   | #123    | ...   | 8     |
   | US2   | #124    | ...   | 6     |
   | Shared| #125    | ...   | 5     |
   ```

> [!CAUTION]
> UNDER NO CIRCUMSTANCES EVER CREATE ISSUES IN REPOSITORIES THAT DO NOT MATCH THE REMOTE URL

## Post-Execution Checks

**Check for extension hooks (after tasks-to-issues conversion)**:
Check if `.specify/extensions.yml` exists in the project root.
- If it exists, read it and look for entries under the `hooks.after_taskstoissues` key
- If the YAML cannot be parsed or is invalid, skip hook checking silently and continue normally
- Filter out hooks where `enabled` is explicitly `false`. Treat hooks without an `enabled` field as enabled by default.
- For each remaining hook, do **not** attempt to interpret or evaluate hook `condition` expressions:
  - If the hook has no `condition` field, or it is null/empty, treat the hook as executable
  - If the hook defines a non-empty `condition`, skip the hook and leave condition evaluation to the HookExecutor implementation
- For each executable hook, output the following based on its `optional` flag:
  - **Optional hook** (`optional: true`):
    ```
    ## Extension Hooks

    **Optional Hook**: {extension}
    Command: `/{command}`
    Description: {description}

    Prompt: {prompt}
    To execute: `/{command}`
    ```
  - **Mandatory hook** (`optional: false`):
    ```
    ## Extension Hooks

    **Automatic Hook**: {extension}
    Executing: `/{command}`
    EXECUTE_COMMAND: {command}
    ```
- If no hooks are registered or `.specify/extensions.yml` does not exist, skip silently
