# Feature Specification: Skills Package Manager CLI

**Feature Branch**: `001-skills-pkg-manager`  
**Created**: 2026-05-30  
**Status**: Draft  
**Input**: User description: "Initialize a Rust CLI using clap, miette, and indicatif. The goal of the tool is to be an agentic skills package manager that reads a skills.json file, fetches skill imports from git repos, and manages installed skills."

## User Scenarios & Testing

### User Story 1 - Initialize a Skills Manifest (Priority: P1)

A developer working in a new repository wants to declare which agentic skills their project imports and exports. They run `skm init .` in the project root, which creates a `skills.json` file with the correct structure for specifying imports (git clone URLs) and exports (local folder paths).

**Why this priority**: Without a manifest file, no other commands can function. This is the foundational entry point for all users.

**Independent Test**: Can be fully tested by running `skm init .` in an empty directory and verifying the `skills.json` file is created with the expected structure.

**Acceptance Scenarios**:

1. **Given** a directory with no existing `skills.json`, **When** the user runs `skm init .`, **Then** a `skills.json` file is created at the project root with empty `skills` and `exports` objects (`{ "skills": {}, "exports": {} }`).
2. **Given** a directory that already has a `skills.json`, **When** the user runs `skm init .`, **Then** the command warns that the file already exists and does not overwrite it.
3. **Given** a non-existent directory path, **When** the user runs `skm init ./nonexistent`, **Then** the command fails with a clear error message.

---

### User Story 2 - Install All Declared Skills (Priority: P1)

A developer has a `skills.json` listing several skill imports (each with a git clone URL). They run `skm install` in the project root. The CLI reads the manifest, clones each skill repository, reads the source repo's `skills.json` exports to determine which files/dirs to copy into `.agents/skills/<skill-name>/`, and creates a `skills.lock` file mapping each skill to its resolved commit ID.

**Why this priority**: This is the core value proposition — getting skills installed into the project.

**Independent Test**: Can be tested by creating a `skills.json` with a known public git repo as an import, running `skm install`, and verifying the skill appears in `.agents/skills/` and a `skills.lock` file is created.

**Acceptance Scenarios**:

1. **Given** a valid `skills.json` with one or more imports, **When** the user runs `skm install`, **Then** each skill is cloned into `.agents/skills/<skill-name>/` and a `skills.lock` file is created.
2. **Given** a valid `skills.json` where a skill is already installed (exists in `.agents/skills/`), **When** the user runs `skm install`, **Then** the CLI skips the already-installed skill and reports it as already present.
3. **Given** a `skills.json` with an invalid or unreachable git URL, **When** the user runs `skm install`, **Then** the CLI reports the error for that specific skill and continues installing remaining skills.
4. **Given** no `skills.json` in the current directory, **When** the user runs `skm install`, **Then** the CLI fails with a clear error message indicating no manifest was found.

---

### User Story 3 - Install a Specific Skill (Priority: P2)

A developer wants to add a single skill to their project without editing `skills.json` manually. They run `skm install <package_name:repo_clone_url>` (e.g., `skm install clap:https://github.com/clap-rs/clap.git`). The CLI adds the skill to `skills.json`, clones it into `.agents/skills/`, and updates the lockfile.

**Why this priority**: Convenience for adding individual skills; less critical than bulk install but a common workflow.

**Independent Test**: Can be tested by running `skm install myskill:https://github.com/example/repo.git` and verifying the skill is added to `skills.json`, cloned to `.agents/skills/myskill/`, and recorded in `skills.lock`.

**Acceptance Scenarios**:

1. **Given** a valid `skills.json`, **When** the user runs `skm install myskill:https://github.com/example/repo.git`, **Then** the skill is added to the imports in `skills.json`, cloned into `.agents/skills/myskill/`, and recorded in `skills.lock`.
2. **Given** a skill name that already exists in `skills.json`, **When** the user runs `skm install myskill:<url>`, **Then** the CLI warns the skill already exists and does not duplicate it.
3. **Given** no `skills.json` in the current directory, **When** the user runs `skm install myskill:<url>`, **Then** the CLI creates a new `skills.json` with the skill listed as an import.

---

### User Story 4 - List Installed Skills (Priority: P2)

A developer wants to see which skills are currently installed in their project. They run `skm list` and see a table of skill names, their source repository, and the locked commit ID.

**Why this priority**: Essential for visibility into the project's skill state; needed before upgrading or uninstalling.

**Independent Test**: Can be tested by running `skm list` after installing skills and verifying the output lists all installed skills with their details.

**Acceptance Scenarios**:

1. **Given** a project with installed skills, **When** the user runs `skm list`, **Then** the CLI displays a formatted table listing each skill's name, source repo, and locked commit.
2. **Given** a project with no installed skills, **When** the user runs `skm list`, **Then** the CLI displays a message indicating no skills are installed.
3. **Given** a project with a `skills.lock` but missing skill directories, **When** the user runs `skm list`, **Then** the CLI indicates which skills are stale or missing from disk.

---

### User Story 5 - Show Skill Details (Priority: P3)

A developer wants to inspect a specific skill's details (repo URL, locked version, description). They run `skm show <package_name>` and see detailed information about that skill.

**Why this priority**: Useful for debugging and verification; lower priority than install/upgrade workflows.

**Independent Test**: Can be tested by running `skm show <skill-name>` on an installed skill and verifying the output includes the repo URL and commit info.

**Acceptance Scenarios**:

1. **Given** an installed skill, **When** the user runs `skm show <package_name>`, **Then** the CLI displays the skill's repo URL, locked commit ID, and local path.
2. **Given** a skill name that does not exist, **When** the user runs `skm show <package_name>`, **Then** the CLI reports the skill was not found.

---

### User Story 6 - Upgrade Skills to Latest Versions (Priority: P2)

A developer wants to update all installed skills to their latest versions. They run `skm upgrade`, which fetches the latest commits for each skill's repo, updates the skill directory, and refreshes the lockfile with new commit IDs.

**Why this priority**: Keeping skills up-to-date is important for security and features; critical for ongoing maintenance.

**Independent Test**: Can be tested by running `skm upgrade` after the upstream repo has new commits and verifying the lockfile is updated with new commit hashes.

**Acceptance Scenarios**:

1. **Given** installed skills with outdated lock entries, **When** the user runs `skm upgrade`, **Then** each skill is updated to the latest commit on the default branch (HEAD) and `skills.lock` is refreshed.
2. **Given** a skill whose upstream repo is unreachable, **When** the user runs `skm upgrade`, **Then** the CLI reports the error for that skill and continues upgrading others.
3. **Given** no `skills.lock` file exists, **When** the user runs `skm upgrade`, **Then** the CLI treats all skills as needing initial lock entry creation.

---

### User Story 7 - Uninstall a Skill (Priority: P2)

A developer wants to remove a skill from their project. They run `skm uninstall <package_name>` (or `skm remove <package_name>`), which removes the skill from `skills.json`, deletes its directory from `.agents/skills/`, and removes its entry from `skills.lock`.

**Why this priority**: Cleanup and dependency management; necessary for maintaining a lean project.

**Independent Test**: Can be tested by running `skm uninstall <skill-name>` and verifying the skill is removed from `skills.json`, `.agents/skills/`, and `skills.lock`.

**Acceptance Scenarios**:

1. **Given** an installed skill, **When** the user runs `skm uninstall <package_name>`, **Then** the skill is removed from `skills.json`, its directory is deleted, and its lock entry is removed.
2. **Given** a skill that does not exist in the manifest, **When** the user runs `skm uninstall <package_name>`, **Then** the CLI reports the skill was not found.
3. **Given** an installed skill with a dirty working directory in `.agents/skills/<name>/`, **When** the user runs `skm uninstall <package_name>`, **Then** the CLI removes the directory regardless of its state.

---

### Edge Cases

- **Malformed JSON**: `skm install` and `skm upgrade` MUST detect invalid JSON in `skills.json` and report a clear error with line/column info. `skm init` MUST NOT produce malformed JSON.
- **Lockfile drift**: `skm install` SHOULD warn when `skills.lock` contains skills not in `skills.json` (stale entries). `skm list` SHOULD flag stale entries with status `orphaned`.
- **Unwritable directory**: If `.agents/skills/` is not writable, `skm install` MUST fail with a clear error indicating permission issue and suggest checking directory permissions.
- **Auth required**: If a git clone URL requires authentication and fails, `skm` MUST report the auth failure and suggest configuring SSH keys or credential helpers.
- **Duplicate names**: `skills.json` MUST NOT contain duplicate skill names. `skm install <name:url>` MUST warn and skip if the name already exists.
- **Network unavailable**: `skm install` and `skm upgrade` MUST report network errors per skill and continue with remaining skills (FR-012 partial failure handling).
- **Untracked directories**: If a directory exists in `.agents/skills/` but has no corresponding entry in `skills.lock`, `skm list` MUST display it with status `untracked`.
- **Empty/missing exports**: When a source repo's `skills.json` has no `exports` (or `exports` is empty), `skm install` MUST copy the entire cloned repo contents into `.agents/skills/<name>/` as a fallback.

## Requirements

### Functional Requirements

- **FR-001**: System MUST provide a `skm init <path>` command that creates a `skills.json` manifest file with the correct structure.
- **FR-002**: System MUST provide a `skm install` command that reads `skills.json` and clones all declared imports into `.agents/skills/<skill-name>/`.
- **FR-003**: System MUST provide a `skm install <name:repo_url>` command that adds a single skill to the manifest, clones it, and updates the lockfile.
- **FR-004**: System MUST create and maintain a `skills.lock` file that maps each installed skill to its resolved git commit ID.
- **FR-005**: System MUST provide a `skm upgrade` command that fetches the latest version of each skill and updates the lockfile.
- **FR-006**: System MUST provide a `skm list` command that displays all installed skills with their source and locked version.
- **FR-007**: System MUST provide a `skm show <package_name>` command that displays detailed information about a specific skill.
- **FR-008**: System MUST provide `skm uninstall <package_name>` and `skm remove <package_name>` commands that remove a skill from the manifest, lockfile, and disk.
- **FR-009**: System MUST support both HTTPS and SSH git clone URL formats.
- **FR-010**: System MUST display progress indicators during long-running operations (cloning, fetching, upgrading).
- **FR-011**: System MUST produce clear, actionable error messages for all failure modes.
- **FR-012**: System MUST handle partial failures gracefully (e.g., one skill fails to clone but others succeed).
- **FR-013**: System MUST detect and report when `skills.json` is missing, malformed, or contains duplicate skill names. Validation MUST include JSON parse errors with line/column info and name uniqueness checks.
- **FR-014**: System MUST skip already-installed skills during `skm install` and report them as already present.
- **FR-015**: System MUST read the source repo's `skills.json` exports after cloning to determine which files/dirs to copy into `.agents/skills/<skill-name>/`.

### Key Entities

- **Skill**: A named unit of functionality imported from a git repository. Identified by name, git clone URL, and locked commit ID.
- **Manifest (`skills.json`)**: A file at the project root with this structure:
  - Root keys: `skills` and `exports`
  - `skills`: object keyed by skill name, each value `{ "repo": "<git-clone-url>" }` — skills to import/install
  - `exports`: object keyed by skill name, each value `{ "path": "<local-directory>" }` — local directories this project exports as skills. When another project installs this repo, the exports tell `skm` which files/dirs to copy from the cloned repo.
- **Lockfile (`skills.lock`)**: A file mapping each installed skill to `{ "commit": "<sha>", "repo": "<url>" }`. Ensures reproducible installations.
- **Skill Directory (`.agents/skills/<name>/`)**: The local directory where a skill's files are cloned into.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Users can initialize a new project's skill manifest in under 5 seconds.
- **SC-002**: `skm install` completes installing a single skill from a public git repo in under 30 seconds on a standard connection.
- **SC-003**: `skm list` displays all installed skills in a readable table format within 1 second.
- **SC-004**: All error messages include the specific failure reason and a suggested remediation action.
- **SC-005**: Users can complete the full init-install-upgrade-uninstall workflow without manual file editing.

## Clarifications

### Session 2026-05-30

- Q: What structure should the manifest and lockfile use? → A: Object keyed by skill name for both `skills` and `exports` at root level. `skills` maps name to `{ "repo": "<url>" }`, `exports` maps name to `{ "path": "<local-dir>" }`. Lockfile maps skill name to `{ "commit": "<sha>", "repo": "<url>" }`.
- Q: How should `skm upgrade` resolve the "latest" version? → A: Always pull latest commit from the repo's default branch (HEAD). No version pinning.
- Q: How do exports work beyond being declared in the manifest? → A: Exports are declarative metadata in a source repo's `skills.json`. When `skm install` clones a repo, the exports in that repo's manifest guide which files/dirs to copy into `.agents/skills/`. `skm publish` is out of scope for now.

## Assumptions

- The CLI targets a single project root (the directory where `skm` is invoked).
- Git is available on the user's system and configured with network access.
- Skills are git repositories that can be cloned without special authentication (public repos or pre-configured SSH keys).
- The `.agents/skills/` directory is the canonical location for installed skills.
- `skills.json` and `skills.lock` are both located at the project root.
- The CLI is run locally, not in a CI/CD environment (though it should work there too).
- Only one skill with a given name can exist in the manifest at a time.
