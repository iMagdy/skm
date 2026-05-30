---
description: Create a GitHub PR for the current feature branch that links all related issues for the spec. Runs as a post-implementation hook.
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Pre-Execution Checks

**Check for extension hooks (before PR creation)**:
- Check if `.specify/extensions.yml` exists in the project root.
- If it exists, read it and look for entries under the `hooks.before_create_pr` key
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

1. Run `.specify/scripts/bash/check-prerequisites.sh --json --require-tasks --include-tasks` from repo root and parse FEATURE_DIR. All paths must be absolute.

2. **Validate prerequisites**:
   - Check that `FEATURE_DIR/issue-map.json` exists. If not, report error: "No issue map found. Run `/speckit.taskstoissues` first to create GitHub issues." and STOP.
   - Check that we are on a feature branch (not main/master) by running:
     ```bash
     git branch --show-current
     ```
   - Verify the remote is GitHub using:
     ```bash
     gh repo view --json nameWithOwner -q '.nameWithOwner'
     ```

3. **Load issue map**: Read `FEATURE_DIR/issue-map.json` and extract:
   - `repo`: The GitHub repository (OWNER/REPO)
   - `issues`: Object mapping story labels to issue metadata (number, title, url)
   - `feature`: The feature name

4. **Build PR body**: Construct a pull request body that:
   - Has a clear title: `feat: [FEATURE_NAME]`
   - Includes a summary section referencing the spec
   - Lists all related issues with `Closes #N` syntax for auto-closing:

     ```markdown
     ## Summary

     Implements [FEATURE_NAME] as defined in [SPEC_PATH].

     ## Related Issues

     Closes #123 (User Story 1)
     Closes #124 (User Story 2)
     Closes #125 (Setup & Infrastructure)

     ## Changes

     - [List key implementation areas]

     ## Testing

     - [How to verify the implementation]
     ```

5. **Create PR**: Use `gh` CLI to create the pull request:

   ```bash
   gh pr create \
     --title "feat: [FEATURE_NAME]" \
     --body "$PR_BODY" \
     --label "feature"
   ```

   - Capture the PR URL from the output
   - If PR creation fails (e.g., PR already exists), check with:
     ```bash
     gh pr list --head <BRANCH_NAME> --json number,url
     ```
   - If a PR already exists, update it instead:
     ```bash
     gh pr edit <PR_NUMBER> --body "$PR_BODY"
     ```

6. **Report**: Output the PR details:

   ```text
   Pull Request created: [PR_URL]

   Linked Issues:
   - Closes #123 (User Story 1)
   - Closes #124 (User Story 2)
   - Closes #125 (Setup & Infrastructure)
   ```

> [!CAUTION]
> UNDER NO CIRCUMSTANCES EVER CREATE PRs IN REPOSITORIES THAT DO NOT MATCH THE REMOTE URL

## Post-Execution Checks

**Check for extension hooks (after PR creation)**:
Check if `.specify/extensions.yml` exists in the project root.
- If it exists, read it and look for entries under the `hooks.after_create_pr` key
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
