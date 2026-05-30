# GitHub Project Sync

Speckit story and task files can be synced to GitHub issues and the GitHub Project titled `skm`.

## Default Mapping

- One issue per user story.
- One shared issue for setup, foundational, and polish tasks.
- Each task becomes a checkbox in the related issue body.
- The feature directory stores `issue-map.json` with issue numbers, URLs, project item IDs, and task-to-issue mapping.

The generated map keeps enough GitHub metadata for later runs to update instead of duplicate:

```json
{
  "feature": "006-example",
  "repo": "iMagdy/skm",
  "project": {
    "owner": "iMagdy",
    "title": "skm",
    "number": 1
  },
  "issues": {
    "US1": {
      "number": 12,
      "title": "[006-example] US1: Example Story",
      "url": "https://github.com/iMagdy/skm/issues/12",
      "project_item_id": "PVTI_example"
    }
  },
  "task_to_issue": {
    "T001": 12
  }
}
```

## Dry Run

Use dry run before writing to GitHub:

```bash
python3 scripts/speckit_sync_issues.py --feature-dir specs/<active-feature> --dry-run
```

## Live Sync

```bash
python3 scripts/speckit_sync_issues.py \
  --feature-dir specs/<active-feature> \
  --repo iMagdy/skm \
  --project-owner iMagdy \
  --project-title skm
```

The script verifies that the current GitHub remote matches `iMagdy/skm` before creating or editing issues.

## GitHub Auth

The `gh` token needs access to:

- Read repository metadata.
- Create and edit issues.
- Create labels.
- Read and add items to GitHub Projects.

If project sync fails, refresh auth with project scope:

```bash
gh auth refresh -s project
```

## Idempotency

If `issue-map.json` already exists, the script edits existing issues instead of creating duplicates. It also pulls checked boxes from mapped GitHub issues back into `tasks.md` when that can be done without clearing local progress, then re-renders issue checkboxes from the current `tasks.md`.
