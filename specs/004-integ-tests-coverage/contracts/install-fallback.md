# Contract: Install Command (Fallback Path)

**Feature**: 004-integ-tests-coverage
**Date**: Sat May 30 2026

## Command Interface

```bash
kt install <source>
```

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| source | String | Yes | Git repository URL |

### Behavior Contract (Fallback)

**Preconditions**:
- Source repository is accessible via git
- Repository does NOT contain `skills.json` manifest
- Repository contains `skills/` or `SKILLS/` directory with discoverable skills

**Postconditions**:
- Warning displayed about missing manifest
- Skills discovered from `skills/` directory
- User prompted to select skill (if multiple found)
- Selected skill installed to `.agents/skills/`
- `skills.lock` updated with commit SHA

### Test Assertions

| # | Assertion | Priority |
|---|-----------|----------|
| 1 | Warning message displayed about missing manifest | P1 |
| 2 | Auto-discovery scans `skills/` directory | P1 |
| 3 | Selection prompt shown when multiple skills found | P1 |
| 4 | Auto-select when exactly one skill found | P2 |
| 5 | Selected skill installed correctly | P1 |
| 6 | `skills.lock` updated with correct SHA | P1 |

### Discovery Rules

| Scenario | Expected Behavior |
|----------|-------------------|
| `skills/` contains 3 `.md` files | Prompt to select from 3 |
| `skills/` contains 2 subdirectories | Prompt to select from 2 |
| `skills/` contains 1 `.md` file | Auto-select, no prompt |
| `skills/` exists but empty | Error: "No skills found" |
| No `skills/` or `SKILLS/` | Error: "No installable skills" |

### Error Cases

| Error | Expected Behavior |
|-------|-------------------|
| Network timeout | Error message with remediation |
| No skills discovered | Error with discovery details |
| User cancels selection | "Installation cancelled" message |

## Integration Test Fixture

```rust
const FIXTURE: TestFixture = TestFixture {
    name: "agent-skills",
    url: "https://github.com/iMagdy/agent-skills",
    commit_sha: "180115660cfb8a86b808f117475a01f54caf3bc5",
    has_manifest: false,
    skills_dir: Some("skills"),
};
```
