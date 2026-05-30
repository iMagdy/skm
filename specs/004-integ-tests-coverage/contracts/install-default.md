# Contract: Install Command (Default Path)

**Feature**: 004-integ-tests-coverage
**Date**: Sat May 30 2026

## Command Interface

```bash
skm install <source>
```

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| source | String | Yes | Git repository URL or skill name |

### Behavior Contract

**Preconditions**:
- Source repository is accessible via git
- Repository contains `skills.json` manifest
- User has write permission to `.agents/skills/` directory

**Postconditions**:
- Skill files cloned to `.agents/skills/<skill-name>/`
- `skills.lock` updated with commit SHA
- `skills.json` imports updated if applicable

### Test Assertions

| # | Assertion | Priority |
|---|-----------|----------|
| 1 | Skill directory created at `.agents/skills/<name>/` | P1 |
| 2 | `skills.lock` contains correct commit SHA | P1 |
| 3 | Clone completes within 30 seconds | P2 |
| 4 | Skill files match source repository content | P1 |
| 5 | Existing skills not modified | P2 |

### Error Cases

| Error | Expected Behavior |
|-------|-------------------|
| Network timeout | Error message with remediation |
| Invalid URL | Error message with format hint |
| Permission denied | Error message with path suggestion |
| Manifest malformed | Error with JSON parse details |

## Integration Test Fixture

```rust
const FIXTURE: TestFixture = TestFixture {
    name: "awesome-copilot",
    url: "https://github.com/iMagdy/awesome-copilot.git",
    commit_sha: "118974fb72ec31524b002795c116fd66bde14bef",
    has_manifest: true,
    skills_dir: None,
};
```
