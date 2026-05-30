# Feature Specification: Skill Install Fallback

**Feature Branch**: `003-skill-install-fallback`
**Created**: Sat May 30 2026
**Status**: Draft
**Input**: User description: "When installing a skill from a repo, and if skills.json file does not exist on that repo, skm should fallback to skills (or SKILLS) directory if exists in that remote repo, and then if there are many .md files (many skills) or many dirs in that skills/ folder, prompt the user to select which skill from the available skills they want to install. Also show a warning that this remote repo does not have skills.json file, and that skm is currently trying to auto discover skills in that repo."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Install Skill from Repo with Missing Manifest (Priority: P1)

When a user attempts to install a skill from a remote repository that does not contain a `skills.json` manifest file, the system should automatically discover available skills by scanning for a `skills/` or `SKILLS/` directory in the repository root. If multiple skills are found (either as `.md` files or subdirectories), the user should be prompted to select which skill to install, with a clear warning about the auto-discovery process.

**Why this priority**: This is the core functionality - enabling skill installation from repos that lack the standard manifest format, which significantly expands the ecosystem of installable skills.

**Independent Test**: Can be fully tested by attempting to install from a repo without `skills.json` but with a `skills/` directory containing multiple `.md` files. The user sees a warning and selection prompt, then can install their chosen skill.

**Acceptance Scenarios**:

1. **Given** a remote repository without `skills.json` but with a `skills/` directory containing 3 `.md` files, **When** user runs skill install, **Then** system displays warning about missing manifest and prompts user to select from the 3 discovered skills
2. **Given** a remote repository without `skills.json` but with a `SKILLS/` directory containing 2 subdirectories, **When** user runs skill install, **Then** system displays warning and prompts user to select from the 2 discovered skill directories
3. **Given** a remote repository without `skills.json` and without `skills/` or `SKILLS/` directories, **When** user runs skill install, **Then** system displays error indicating no installable skills were found

---

### User Story 2 - Warning Communication (Priority: P2)

The system must clearly communicate to users when auto-discovery is being used instead of the standard manifest approach, ensuring users understand the non-standard installation path and can make informed decisions.

**Why this priority**: Clear communication builds trust and helps users understand what's happening during the installation process.

**Independent Test**: Can be tested by triggering any auto-discovery scenario and verifying the warning message appears before the selection prompt.

**Acceptance Scenarios**:

1. **Given** auto-discovery is triggered due to missing `skills.json`, **When** system begins discovery, **Then** a warning message is displayed indicating the repo lacks a manifest file
2. **Given** auto-discovery is triggered, **When** system begins scanning, **Then** a message is displayed indicating skills are being auto-discovered

---

### User Story 3 - Single Skill Auto-Selection (Priority: P3)

When auto-discovery finds exactly one skill, the system should proceed with installation without requiring user selection, streamlining the experience for repos with a single skill.

**Why this priority**: This improves user experience by eliminating unnecessary prompts when there's only one option.

**Independent Test**: Can be tested by attempting to install from a repo without `skills.json` but with exactly one skill in the `skills/` directory.

**Acceptance Scenarios**:

1. **Given** auto-discovery finds exactly one skill, **When** system discovers it, **Then** system proceeds with installation without prompting for selection
2. **Given** auto-discovery finds exactly one skill, **When** installation completes, **Then** user sees confirmation of the installed skill

---

### Edge Cases

- What happens when the `skills/` directory exists but is empty? → **Resolved**: Display error "No skills found in the discovered directory" (FR-011)
- What happens when the `skills/` directory contains a mix of `.md` files and subdirectories? → **Resolved**: Both types are discovered and presented in the selection prompt (FR-005, FR-006)
- What happens when a discovered skill has an invalid or missing name/title? → **Resolved**: Use filename as-is after normalization; no validation required (FR-007)
- What happens when the user cancels the selection prompt? → **Resolved**: Cancel installation gracefully, display "Installation cancelled" message (FR-013)
- What happens when the remote repository is unreachable or authentication fails? → **Out of scope**: Existing git clone error handling applies; no fallback-specific handling needed
- What happens when discovered skills have conflicting names? → **Resolved**: Deduplicate by keeping first occurrence, skip duplicates with warning (FR-014)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST attempt to locate `skills.json` in the remote repository root before installation
- **FR-002**: System MUST fallback to scanning for `skills/` or `SKILLS/` directory when `skills.json` is not found
- **FR-003**: System MUST display a warning message indicating the repository lacks a `skills.json` manifest file
- **FR-004**: System MUST display a message indicating auto-discovery is in progress
- **FR-005**: System MUST discover skills as `.md` files directly in the `skills/` directory
- **FR-006**: System MUST discover skills as subdirectories within the `skills/` directory
- **FR-007**: System MUST present a selection prompt when multiple skills are discovered, displaying skill names extracted from file/directory names (strip `.md`, normalize hyphens/underscores to spaces)
- **FR-008**: System MUST allow users to select one skill from the discovered options by name
- **FR-009**: System MUST proceed with installation of the selected skill
- **FR-010**: System MUST auto-select when exactly one skill is discovered
- **FR-011**: System MUST display an error when no skills are found (either no skills directory exists, or it exists but contains no `.md` files or subdirectories)
- **FR-012**: System MUST normalize directory names to lowercase before searching for `skills` (finds any case variant)
- **FR-013**: System MUST cancel installation gracefully when user cancels the selection prompt, displaying "Installation cancelled" message
- **FR-014**: System MUST deduplicate discovered skills by name, keeping the first occurrence and skipping duplicates with a warning message

### Key Entities

- **Skill**: A discrete unit of functionality that can be installed, represented as a `.md` file or subdirectory. Skills are named by extracting from file/directory names: strip `.md` extension, normalize hyphens and underscores to spaces
- **Manifest**: The `skills.json` file that provides structured metadata about available skills
- **Discovery**: The process of scanning a repository for installable skills when no manifest exists

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can successfully install skills from repositories lacking `skills.json` when a `skills/` directory exists
- **SC-002**: Warning messages appear immediately (no perceivable delay) after detecting missing manifest
- **SC-003**: Selection prompts correctly list all discovered skills with accurate names
- **SC-004**: Auto-discovery works for repositories with both `skills/` and `SKILLS/` directories
- **SC-005**: 100% of discovered skills are selectable and installable through the fallback mechanism
- **SC-006**: Error messages clearly communicate when no skills are discoverable

## Clarifications

### Session 2026-05-30

- Q: How should the system determine skill names when displaying the selection prompt? → A: Extract from file/directory names (strip `.md`, normalize hyphens/underscores to spaces)
- Q: What should happen when the `skills/` or `SKILLS/` directory exists but is empty? → A: Display error: "No skills found in the discovered directory" (same as no skills found)
- Q: What should happen when the user cancels the selection prompt? → A: Cancel installation, display "Installation cancelled" message, exit gracefully
- Q: What should happen when discovered skills have conflicting names? → A: Deduplicate by keeping the first occurrence and ignoring duplicates, displaying a warning
- Q: How should the system handle case sensitivity for directory names? → A: Normalize directory names to lowercase before searching for `skills` (finds any case variant)

## Assumptions

- The `skills/` or `SKILLS/` directory, when present, contains the actual skill files or directories
- Skill names are derived from filenames, not file content
- Subdirectories in `skills/` represent individual skills
- Users have network access to the remote repository
- The repository structure follows common conventions for skill organization
- Case sensitivity handling for directory names follows platform conventions (e.g., case-insensitive on macOS/Windows)
