# Contract: Export Command

**Feature**: 004-integ-tests-coverage
**Date**: Sat May 30 2026

## Command Interface

```bash
skm export
```

### Parameters

None — command operates on current project state.

### Behavior Contract

**Preconditions**:
- Current directory contains `.agents/skills/` with installed skills
- User has write permission to current directory

**Postconditions**:
- `skills.json` created or updated in current directory
- Manifest contains `skills` (imports) and `exports` keys
- Each installed skill listed with correct source URL

### Test Assertions

| # | Assertion | Priority |
|---|-----------|----------|
| 1 | `skills.json` created in current directory | P1 |
| 2 | Manifest contains `skills` key | P1 |
| 3 | Manifest contains `exports` key | P1 |
| 4 | Each skill listed with correct source | P1 |
| 5 | Manifest is valid JSON with 2-space indent | P2 |
| 6 | Existing manifest updated, not overwritten | P2 |

### Manifest Schema

```json
{
  "skills": {
    "<skill-name>": {
      "source": "<git-url>",
      "branch": "main"
    }
  },
  "exports": {}
}
```

### Error Cases

| Error | Expected Behavior |
|-------|-------------------|
| No skills installed | Warning: "No skills to export" |
| Permission denied | Error with path suggestion |
| Invalid existing manifest | Error with JSON parse details |

## Integration Test Scenario

1. Clone `awesome-copilot` fixture
2. Install skill from fixture
3. Run `skm export`
4. Verify `skills.json` contains installed skill
5. Verify manifest is valid JSON
6. Verify skill source matches fixture URL
