---
description: Idempotently sync Speckit user stories and tasks to GitHub issues and the GitHub Project.
---

## User Input

```text
$ARGUMENTS
```

## Outline

1. Resolve the feature directory:
   - Prefer an explicit `--feature-dir <path>` argument.
   - Otherwise use `.specify/feature.json` when present.
   - Stop if no feature directory can be resolved.
2. Run a dry run first:

   ```bash
   python3 scripts/speckit_sync_issues.py --feature-dir <feature-dir> --dry-run
   ```

3. Confirm the dry-run grouping looks right.
4. Run the live sync:

   ```bash
   python3 scripts/speckit_sync_issues.py \
     --feature-dir <feature-dir> \
     --repo iMagdy/skm \
     --project-owner iMagdy \
     --project-title skm
   ```

5. Report:
   - Feature directory.
   - Issue groups created or updated.
   - Location of `<feature-dir>/issue-map.json`.
   - Any GitHub auth or project lookup errors.

## Behavior

The sync script creates one issue per user story plus one shared infrastructure issue. It stores issue numbers, URLs, GitHub Project item IDs, and task-to-issue mapping in `issue-map.json`.

The script is idempotent: if `issue-map.json` already maps a story to an issue, that issue is edited instead of duplicated.
