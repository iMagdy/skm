# Data Model: Ktesio CLI

**Feature**: 001-skills-pkg-manager
**Date**: 2026-05-30

## Entities

### SkillEntry

Represents a single skill import in the manifest.

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Skill identifier (used as directory name and map key) |
| `repo` | String | Git clone URL (HTTPS or SSH) |

**Validation rules**:
- Name MUST be non-empty, alphanumeric + hyphens/underscores only (`^[a-zA-Z0-9_-]+$`)
- Name MUST be unique across all skills in the manifest
- Repo MUST be a valid git clone URL (starts with `https://`, `http://`, `git@`, or `ssh://`)

**Source**: `skills.json` → `skills` object keys

---

### ExportEntry

Represents a local directory exported as a skill.

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Exported skill name (map key) |
| `path` | String | Relative path to the local directory containing the skill files |

**Validation rules**:
- Name MUST follow same rules as SkillEntry name
- Path MUST point to an existing directory
- Path SHOULD be relative to the project root

**Source**: `skills.json` → `exports` object keys

---

### Manifest

The `skills.json` file structure.

```json
{
  "skills": {
    "skill-name": {
      "repo": "https://github.com/org/repo.git"
    }
  },
  "exports": {
    "my-skill": {
      "path": "./skills/my-skill"
    }
  }
}
```

**Location**: Project root (where Ktesio is invoked)
**Format**: JSON, 2-space indentation
**Cardinality**: One per project

**State transitions**:
1. Created by `kt init` (empty `skills` and `exports`)
2. Modified by `kt install <name:url>` (adds to `skills`)
3. Modified by `kt uninstall <name>` (removes from `skills`)
4. Read by `kt install`, `kt upgrade`, `kt list`, `kt show`, `kt uninstall`

---

### LockEntry

Maps an installed skill to its resolved git commit.

| Field | Type | Description |
|-------|------|-------------|
| `commit` | String | Full SHA-1 commit hash (40 chars) |
| `repo` | String | Git clone URL (mirrors from manifest) |

**Validation rules**:
- Commit MUST be a valid 40-character hex SHA
- Repo MUST match the URL in `skills.json` (detect drift)

---

### Lockfile

The `skills.lock` file structure.

```json
{
  "skill-name": {
    "commit": "abc123def456...",
    "repo": "https://github.com/org/repo.git"
  }
}
```

**Location**: Project root (alongside `skills.json`)
**Format**: JSON, 2-space indentation
**Cardinality**: One per project

**State transitions**:
1. Created by `kt install` (populates entries for each installed skill)
2. Updated by `kt upgrade` (refreshes commit hashes)
3. Modified by `kt uninstall` (removes entry)
4. Read by `kt list`, `kt show`, `kt upgrade`

---

### InstalledSkill

Runtime representation of a skill on disk.

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Skill identifier |
| `repo` | String | Git clone URL |
| `commit` | String | Locked commit SHA |
| `path` | PathBuf | Absolute path to `.agents/skills/<name>/` |

**Derived from**: Lockfile + filesystem check

**State transitions**:
1. Not installed (no directory, no lock entry)
2. Installed (directory exists, lock entry exists)
3. Stale (lock entry exists but directory missing — detected by `kt list`)
4. Uninstalled (directory deleted, lock entry removed, manifest entry removed)

---

## Relationships

```text
Manifest (skills.json)
├── skills: HashMap<String, SkillEntry>
│   └── 1 SkillEntry → 1 LockEntry (via name key)
└── exports: HashMap<String, ExportEntry>
    └── ExportEntry.path → filesystem directory

Lockfile (skills.lock)
└── HashMap<String, LockEntry>
    └── LockEntry.repo ↔ SkillEntry.repo (should match)

InstalledSkill
├── name ↔ Manifest.skills key
├── name ↔ Lockfile key
├── commit = LockEntry.commit
└── path = .agents/skills/<name>/
```

---

## File Layout on Disk

```
<project-root>/
├── skills.json              # Manifest
├── skills.lock              # Lockfile
└── .agents/
    └── skills/
        ├── skill-a/         # Cloned from skill-a's repo
        │   ├── .git/
        │   └── [skill files from exports]
        └── skill-b/
            ├── .git/
            └── [skill files from exports]
```
