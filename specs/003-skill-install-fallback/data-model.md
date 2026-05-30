# Data Model: Skill Install Fallback

**Feature**: 003-skill-install-fallback
**Date**: Sat May 30 2026

## Entities

### DiscoveredSkill

Represents a skill found during fallback discovery.

| Field | Type | Description |
|-------|------|-------------|
| name | String | Normalized display name (file/dir name, cleaned) |
| path | PathBuf | Original path in the repository |
| skill_type | SkillType | Whether source is file or directory |

### SkillType (enum)

| Variant | Description |
|---------|-------------|
| File | Single `.md` file |
| Directory | Directory containing skill files |

### DiscoveryResult

Result of the fallback discovery process.

| Field | Type | Description |
|-------|------|-------------|
| skills | Vec\<DiscoveredSkill\> | List of discovered skills (deduplicated) |
| warnings | Vec\<String\> | Warnings encountered during discovery |

## State Transitions

```
Install Request
    │
    ▼
Check for skills.json
    │
    ├─── Found ──→ Use manifest (existing flow)
    │
    └─── Not Found ──→ Begin Fallback Discovery
                          │
                          ▼
                    Search for skills/ directory
                    (normalize names to lowercase)
                          │
                          ├─── Found ──→ Scan contents
                          │                  │
                          │                  ├─── Multiple ──→ Prompt user to select
                          │                  │                      │
                          │                  │                      ▼
                          │                  │                Install selected skill
                          │                  │
                          │                  ├─── One ──→ Auto-select and install
                          │                  │
                          │                  └─── Empty ──→ Error: No skills found
                          │
                          └─── Not Found ──→ Error: No skills directory
```

## Validation Rules

1. **Skill name normalization**: Strip `.md` extension, replace hyphens/underscores with spaces
2. **Deduplication**: Keep first occurrence when normalized names match
3. **Empty directory**: Treated as "no skills found" error
4. **Cancellation**: User input "q" or empty input cancels installation
