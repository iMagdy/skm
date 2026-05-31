use std::path::{Path, PathBuf};

use dialoguer::{Confirm, MultiSelect, Select};
use indicatif::{MultiProgress, ProgressBar};

use crate::discovery::{self, DiscoveredSkill, SkillType};
use crate::error::{InstallAlreadyExists, InstallInvalidFormat, SkillCopyFailed};
use crate::git;
use crate::lockfile::{LockEntry, Lockfile};
use crate::manifest::Manifest;
use crate::skill;
use crate::ui;

#[cfg(not(tarpaulin_include))]
pub fn run(target: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let project_root = std::env::current_dir()?;
    run_in(&project_root, target)
}

pub(crate) fn run_in(
    project_root: &Path,
    target: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(t) = target {
        return run_single(project_root, t);
    }

    run_bulk(project_root)
}

fn run_bulk(project_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let manifest_path = project_root.join("skills.json");

    // T009: Check for manifest existence
    if manifest_path.exists() {
        // Existing manifest flow
        return run_bulk_with_manifest(project_root, &manifest_path);
    }

    // T010: Fallback path when manifest not found
    run_bulk_with_fallback(project_root)
}

fn run_bulk_with_manifest(
    project_root: &Path,
    manifest_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let manifest = Manifest::load(manifest_path)?;
    let mut lockfile = Lockfile::load(&project_root.join("skills.lock"))?;

    let mp = MultiProgress::new();
    let mut errors: Vec<String> = Vec::new();
    let mut lockfile_changed = false;

    for (name, entry) in &manifest.skills {
        let skill_dir = git::skill_dir(project_root, name);

        if skill_dir.exists() && lockfile.contains(name) {
            ui::warning(format!(
                "Skill {} already installed, skipping",
                ui::skill_name(name)
            ));
            continue;
        }

        let pb = ui::install_progress(&mp, name);
        let commit = match install_repo_to_skill_dir(project_root, name, &entry.repo, &pb) {
            Ok(commit) => commit,
            Err(e) => {
                ui::finish_error(&pb, format!("Error installing {}: {}", name, e));
                errors.push(format!("Error installing {}: {}", name, e));
                continue;
            }
        };

        if !skill_dir.exists() {
            ui::finish_error(
                &pb,
                format!("Error installing {}: destination missing", name),
            );
            errors.push(format!("Error installing {}: destination missing", name));
            continue;
        }

        lockfile.insert(
            name.clone(),
            LockEntry {
                commit,
                repo: entry.repo.clone(),
            },
        );
        lockfile_changed = true;

        ui::finish_success(&pb, format!("Installed {}", ui::skill_name(name)));
    }

    if lockfile_changed {
        lockfile.save(&project_root.join("skills.lock"))?;
    }

    if !errors.is_empty() {
        eprintln!();
        ui::error("Errors encountered:");
        for err in &errors {
            eprintln!("  {}", err);
        }
    }

    Ok(())
}

/// T010, T011, T012, T013, T014, T015: Fallback discovery when manifest not found
fn run_bulk_with_fallback(project_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // T011: Display warning messages
    ui::warning("No skills.json found. Auto-discovering skills...");
    ui::info("Discovering skills in repository...");

    // T004: Find skills directory
    let skills_dir = match discovery::find_skills_directory(project_root) {
        Some(dir) => dir,
        None => {
            return Err(crate::error::DiscoveryError {
                message: "No skills directory found. Cannot discover skills without a manifest."
                    .to_string(),
            }
            .into());
        }
    };

    // T005: Discover skills in the directory
    let result = discovery::discover_skills(&skills_dir);

    // Display any warnings from discovery
    for warning in &result.warnings {
        ui::warning(warning);
    }

    // T013: Handle empty skills directory
    if result.skills.is_empty() {
        return Err(crate::error::SkillsDirectoryEmpty {
            message: "No skills found in the discovered directory".to_string(),
        }
        .into());
    }

    // T012, T019: Handle selection based on number of skills
    let selected_skill = if result.skills.len() == 1 {
        // T019: Auto-select when exactly one skill found
        result.skills.into_iter().next().unwrap()
    } else {
        // T012: Prompt user to select from multiple skills
        prompt_user_selection(&result.skills)?
    };

    // T015: Install the selected skill
    install_discovered_skill(project_root, &selected_skill)
}

/// T012: Prompt user to select a skill from discovered options
#[cfg(not(tarpaulin_include))]
fn prompt_user_selection(
    skills: &[DiscoveredSkill],
) -> Result<DiscoveredSkill, Box<dyn std::error::Error>> {
    println!(
        "\n{}\n",
        ui::table_header("Multiple skills found in repository")
    );

    // Display numbered list
    for (i, skill) in skills.iter().enumerate() {
        println!("  {}. {}", i + 1, ui::skill_name(&skill.name));
    }

    // T014: Handle user cancellation
    let selection = Select::new()
        .with_prompt(format!(
            "\nSelect skill to install (1-{}, or 'q' to cancel)",
            skills.len()
        ))
        .items(&skills.iter().map(|s| s.name.as_str()).collect::<Vec<_>>())
        .default(0)
        .interact_opt()?;

    match selection {
        Some(index) => Ok(skills[index].clone()),
        None => {
            // User pressed Ctrl+C or Esc
            ui::warning("Installation cancelled");
            std::process::exit(3);
        }
    }
}

#[cfg(tarpaulin_include)]
fn prompt_user_selection(
    _skills: &[DiscoveredSkill],
) -> Result<DiscoveredSkill, Box<dyn std::error::Error>> {
    Err(crate::error::DiscoveryError {
        message: "Interactive selection is disabled during coverage runs".to_string(),
    }
    .into())
}

/// T015: Install a discovered skill
fn install_discovered_skill(
    project_root: &Path,
    skill: &DiscoveredSkill,
) -> Result<(), Box<dyn std::error::Error>> {
    let mp = MultiProgress::new();
    let pb = ui::install_progress(&mp, &skill.name);

    // For discovered skills, we need to clone from a URL
    // Since discovery finds skills in a local directory, we need to handle this differently
    // The skill.path is the local path to the skill file/directory
    // We'll copy it directly instead of cloning

    let skill_dir = git::skill_dir(project_root, &skill.name);
    std::fs::create_dir_all(&skill_dir)?;

    pb.set_position(40);
    pb.set_message(format!(
        "Copying discovered skill {}",
        ui::skill_name(&skill.name)
    ));

    // Copy the skill from the discovered location
    match skill.skill_type {
        SkillType::File => {
            // Copy the .md file
            std::fs::copy(&skill.path, skill_dir.join(skill.path.file_name().unwrap()))?;
        }
        SkillType::Directory => {
            // Copy the directory contents
            copy_dir_recursive(&skill.path, &skill_dir)?;
        }
    }

    // Create a minimal lockfile entry
    let mut lockfile = Lockfile::load(&project_root.join("skills.lock"))?;
    lockfile.insert(
        skill.name.clone(),
        LockEntry {
            commit: "0".repeat(40), // Local discovery has no git commit to lock.
            repo: skill.path.to_string_lossy().to_string(),
        },
    );
    lockfile.save(&project_root.join("skills.lock"))?;

    ui::finish_success(&pb, format!("Installed {}", ui::skill_name(&skill.name)));
    ui::success(format!("Installed {}", ui::skill_name(&skill.name)));

    Ok(())
}

/// Copy directory recursively, skipping .git and other common directories
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !dst.exists() {
        std::fs::create_dir_all(dst)?;
    }

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        let name = entry.file_name().to_string_lossy().to_string();
        if name == ".git" || name == "target" || name == "node_modules" {
            continue;
        }

        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

fn run_single(project_root: &Path, target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parts: Vec<&str> = target.splitn(2, ':').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err(InstallInvalidFormat {
            message:
                "Invalid format. Expected name:url (e.g., clap:https://github.com/clap-rs/clap.git)"
                    .to_string(),
        }
        .into());
    }

    let name = parts[0].to_string();
    let repo_url = parts[1].to_string();
    if !is_valid_skill_name(&name) {
        return Err(InstallInvalidFormat {
            message: format!("Invalid skill name '{}': must match [a-zA-Z0-9_-]+", name),
        }
        .into());
    }

    let manifest_path = project_root.join("skills.json");
    let mut manifest = if manifest_path.exists() {
        Manifest::load(&manifest_path)?
    } else {
        Manifest::new()
    };

    if manifest.has_skill(&name) {
        return Err(InstallAlreadyExists {
            message: format!("Skill '{}' already exists in manifest, skipping", name),
        }
        .into());
    }

    let mp = MultiProgress::new();
    let pb = ui::install_progress(&mp, &name);

    let commit = install_repo_to_skill_dir(project_root, &name, &repo_url, &pb)?;

    manifest.add_skill(name.clone(), repo_url.clone());
    manifest.save(&manifest_path)?;

    let mut lockfile = Lockfile::load(&project_root.join("skills.lock"))?;
    lockfile.insert(
        name.clone(),
        LockEntry {
            commit,
            repo: repo_url,
        },
    );
    lockfile.save(&project_root.join("skills.lock"))?;

    ui::finish_success(&pb, format!("Installed {}", ui::skill_name(&name)));
    ui::success(format!(
        "Installed {} from {}",
        ui::skill_name(&name),
        project_root.display()
    ));

    Ok(())
}

fn install_repo_to_skill_dir(
    project_root: &Path,
    name: &str,
    repo_url: &str,
    pb: &ProgressBar,
) -> Result<String, Box<dyn std::error::Error>> {
    let skill_dir = git::skill_dir(project_root, name);
    if skill_dir.exists() {
        return Err(SkillCopyFailed {
            message: format!(
                "Destination skill directory already exists at {}",
                skill_dir.display()
            ),
        }
        .into());
    }

    let workspace = InstallWorkspace::create(project_root, name)?;

    pb.set_message(format!("Cloning {}", ui::skill_name(name)));
    git::clone_with_progress(repo_url, &workspace.clone_dir, pb)?;

    let commit = git::rev_parse_head(&workspace.clone_dir).unwrap_or_default();

    pb.set_position(92);
    pb.set_message(format!("Copying files for {}", ui::skill_name(name)));
    let prompter = DialoguerFallbackPrompter;
    copy_repo_content_for_install(&workspace.clone_dir, &workspace.install_dir, &prompter)?;

    if let Some(parent) = skill_dir.parent() {
        std::fs::create_dir_all(parent)?;
    }
    pb.set_position(96);
    pb.set_message(format!("Finalizing {}", ui::skill_name(name)));
    std::fs::rename(&workspace.install_dir, &skill_dir)?;

    Ok(commit)
}

fn copy_repo_content_for_install(
    clone_dir: &Path,
    dest_dir: &Path,
    prompter: &dyn FallbackPrompter,
) -> Result<(), Box<dyn std::error::Error>> {
    if clone_dir.join("skills.json").exists() {
        return skill::copy_cloned_repo_to_dest(clone_dir, dest_dir);
    }

    if !prompter.confirm_missing_manifest()? {
        return Err(SkillCopyFailed {
            message: "Installation cancelled because source repo has no skills.json".to_string(),
        }
        .into());
    }

    let skills_dir =
        discovery::find_skills_directory(clone_dir).ok_or_else(|| SkillCopyFailed {
            message: "Source repo has no skills.json or skills/SKILLS directory".to_string(),
        })?;
    let skills = discover_fallback_skill_dirs(&skills_dir)?;
    if skills.is_empty() {
        return Err(SkillCopyFailed {
            message: "Source skills directory does not contain skill directories".to_string(),
        }
        .into());
    }

    let selected_indexes = if skills.len() == 1 {
        vec![0]
    } else {
        prompter.select_skill_dirs(&skills)?
    };

    if selected_indexes.is_empty() {
        return Err(SkillCopyFailed {
            message: "No fallback skills selected".to_string(),
        }
        .into());
    }

    for index in selected_indexes {
        let selected = skills.get(index).ok_or_else(|| SkillCopyFailed {
            message: format!("Selected fallback skill index {} is invalid", index),
        })?;
        copy_dir_recursive(&selected.path, &dest_dir.join(&selected.name))?;
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct FallbackSkillDir {
    name: String,
    path: PathBuf,
}

trait FallbackPrompter {
    fn confirm_missing_manifest(&self) -> Result<bool, Box<dyn std::error::Error>>;
    fn select_skill_dirs(
        &self,
        skills: &[FallbackSkillDir],
    ) -> Result<Vec<usize>, Box<dyn std::error::Error>>;
}

struct DialoguerFallbackPrompter;

impl FallbackPrompter for DialoguerFallbackPrompter {
    fn confirm_missing_manifest(&self) -> Result<bool, Box<dyn std::error::Error>> {
        ui::warning("Source repo has no skills.json file.");
        Ok(Confirm::new()
            .with_prompt("Fetch skill directories from skills/SKILLS if present?")
            .default(false)
            .interact()?)
    }

    fn select_skill_dirs(
        &self,
        skills: &[FallbackSkillDir],
    ) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
        let items = skills
            .iter()
            .map(|skill| skill.name.as_str())
            .collect::<Vec<_>>();
        Ok(MultiSelect::new()
            .with_prompt("Select skills to install")
            .items(&items)
            .interact()?)
    }
}

fn discover_fallback_skill_dirs(
    skills_dir: &Path,
) -> Result<Vec<FallbackSkillDir>, Box<dyn std::error::Error>> {
    let mut skills = Vec::new();

    for entry in std::fs::read_dir(skills_dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if !file_type.is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') || name == "target" || name == "node_modules" {
            continue;
        }

        skills.push(FallbackSkillDir {
            name,
            path: entry.path(),
        });
    }

    skills.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(skills)
}

fn is_valid_skill_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

struct InstallWorkspace {
    root: PathBuf,
    clone_dir: PathBuf,
    install_dir: PathBuf,
}

impl InstallWorkspace {
    fn create(project_root: &Path, name: &str) -> Result<Self, std::io::Error> {
        let temp_parent = project_root.join(".agents").join(".tmp");
        std::fs::create_dir_all(&temp_parent)?;

        let safe_name: String = name
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                    ch
                } else {
                    '_'
                }
            })
            .collect();

        for attempt in 0..100 {
            let root = temp_parent.join(format!(
                "install-{}-{}-{}",
                safe_name,
                std::process::id(),
                attempt
            ));
            if root.exists() {
                continue;
            }

            std::fs::create_dir(&root)?;
            return Ok(Self {
                clone_dir: root.join("clone"),
                install_dir: root.join("install"),
                root,
            });
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "could not create a unique install workspace",
        ))
    }
}

impl Drop for InstallWorkspace {
    fn drop(&mut self) {
        let temp_parent = self.root.parent().map(Path::to_path_buf);
        let _ = std::fs::remove_dir_all(&self.root);
        if let Some(parent) = temp_parent {
            let _ = std::fs::remove_dir(&parent);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    struct FakeFallbackPrompter {
        confirm: bool,
        selections: Vec<usize>,
    }

    impl FallbackPrompter for FakeFallbackPrompter {
        fn confirm_missing_manifest(&self) -> Result<bool, Box<dyn std::error::Error>> {
            Ok(self.confirm)
        }

        fn select_skill_dirs(
            &self,
            _skills: &[FallbackSkillDir],
        ) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
            Ok(self.selections.clone())
        }
    }

    #[test]
    fn test_run_invalid_format() {
        let dir = std::env::temp_dir().join("skm_test_install_invalid");
        std::fs::create_dir_all(&dir).unwrap();
        let result = run_in(&dir, Some("invalidformat"));
        assert!(result.is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_invalid_format_no_colon() {
        let dir = std::env::temp_dir().join("skm_test_install_nocolon");
        std::fs::create_dir_all(&dir).unwrap();
        let result = run_in(&dir, Some("nameonly"));
        assert!(result.is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_invalid_format_empty_name() {
        let dir = std::env::temp_dir().join("skm_test_install_ename");
        std::fs::create_dir_all(&dir).unwrap();
        let result = run_in(&dir, Some(":url"));
        assert!(result.is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_invalid_format_empty_url() {
        let dir = std::env::temp_dir().join("skm_test_install_eurl");
        std::fs::create_dir_all(&dir).unwrap();
        let result = run_in(&dir, Some("name:"));
        assert!(result.is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_bulk_empty() {
        let dir = std::env::temp_dir().join("skm_test_install_empty");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), r#"{"skills": {}, "exports": {}}"#).unwrap();
        let result = run_in(&dir, None);
        assert!(result.is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_bulk_already_installed() {
        let dir = std::env::temp_dir().join("skm_test_install_already");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("skills.json"),
            r#"{"skills": {"test": {"repo": "url"}}, "exports": {}}"#,
        )
        .unwrap();
        std::fs::write(
            dir.join("skills.lock"),
            r#"{"test": {"commit": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2", "repo": "url"}}"#,
        )
        .unwrap();
        std::fs::create_dir_all(dir.join(".agents/skills/test")).unwrap();
        let result = run_in(&dir, None);
        assert!(result.is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_bulk_clone_fails() {
        let dir = std::env::temp_dir().join("skm_test_install_clonefail");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let missing_repo = dir.join("missing-repo");
        std::fs::write(
            dir.join("skills.json"),
            format!(
                r#"{{"skills": {{"test": {{"repo": "{}"}}}}, "exports": {{}}}}"#,
                missing_repo.display()
            ),
        )
        .unwrap();
        let result = run_in(&dir, None);
        assert!(result.is_ok());
        assert!(!dir.join("skills.lock").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_single_already_exists() {
        let dir = std::env::temp_dir().join("skm_test_install_single_exists");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("skills.json"),
            r#"{"skills": {"test": {"repo": "url"}}, "exports": {}}"#,
        )
        .unwrap();
        let result = run_in(&dir, Some("test:https://example.com/repo.git"));
        assert!(result.is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_single_clone_fails() {
        let dir = std::env::temp_dir().join("skm_test_install_single_clonefail");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), r#"{"skills": {}, "exports": {}}"#).unwrap();
        let target = format!("test:{}", dir.join("missing-repo").display());
        let result = run_in(&dir, Some(&target));
        assert!(result.is_err());
        assert_eq!(
            std::fs::read_to_string(dir.join("skills.json")).unwrap(),
            r#"{"skills": {}, "exports": {}}"#
        );
        assert!(!dir.join("skills.lock").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_single_invalid_name_does_not_write_metadata() {
        let dir = std::env::temp_dir().join("skm_test_install_single_invalid_name");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let target = format!("bad name:{}", dir.join("repo").display());
        let result = run_in(&dir, Some(&target));
        assert!(result.is_err());
        assert!(!dir.join("skills.json").exists());
        assert!(!dir.join("skills.lock").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_single_success_with_local_repo() {
        let dir = std::env::temp_dir().join("skm_test_install_single_success");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), r#"{"skills": {}, "exports": {}}"#).unwrap();
        let repo = create_local_repo(&dir, "source");

        let target = format!("test:{}", repo.display());
        let result = run_in(&dir, Some(&target));

        assert!(result.is_ok());
        assert!(dir.join(".agents/skills/test").exists());
        assert!(dir.join(".agents/skills/test/source/SKILL.md").exists());
        assert!(!dir.join(".agents/skills/test/README.md").exists());
        assert!(!dir.join(".agents/skills/test/.git").exists());
        assert!(dir.join("skills.lock").exists());

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_copy_repo_content_for_install_requires_confirmation_without_manifest() {
        let dir = std::env::temp_dir().join("skm_test_repo_content_decline");
        let _ = std::fs::remove_dir_all(&dir);
        let src = dir.join("src");
        let dst = dir.join("dst");
        std::fs::create_dir_all(src.join("skills/one")).unwrap();
        std::fs::write(src.join("skills/one/SKILL.md"), "# One").unwrap();

        let prompter = FakeFallbackPrompter {
            confirm: false,
            selections: vec![],
        };
        let result = copy_repo_content_for_install(&src, &dst, &prompter);

        assert!(result.is_err());
        assert!(!dst.exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_copy_repo_content_for_install_multiselects_fallback_directories() {
        let dir = std::env::temp_dir().join("skm_test_repo_content_multiselect");
        let _ = std::fs::remove_dir_all(&dir);
        let src = dir.join("src");
        let dst = dir.join("dst");
        std::fs::create_dir_all(src.join("skills/alpha")).unwrap();
        std::fs::create_dir_all(src.join("skills/beta")).unwrap();
        std::fs::write(src.join("skills/alpha/SKILL.md"), "# Alpha").unwrap();
        std::fs::write(src.join("skills/beta/SKILL.md"), "# Beta").unwrap();
        std::fs::write(src.join("README.md"), "not installed").unwrap();

        let prompter = FakeFallbackPrompter {
            confirm: true,
            selections: vec![1],
        };
        let result = copy_repo_content_for_install(&src, &dst, &prompter);

        assert!(result.is_ok());
        assert!(!dst.join("alpha").exists());
        assert!(dst.join("beta/SKILL.md").exists());
        assert!(!dst.join("README.md").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_copy_repo_content_for_install_auto_selects_single_fallback_directory() {
        let dir = std::env::temp_dir().join("skm_test_repo_content_single_fallback");
        let _ = std::fs::remove_dir_all(&dir);
        let src = dir.join("src");
        let dst = dir.join("dst");
        std::fs::create_dir_all(src.join("SKILLS/only")).unwrap();
        std::fs::write(src.join("SKILLS/only/SKILL.md"), "# Only").unwrap();

        let prompter = FakeFallbackPrompter {
            confirm: true,
            selections: vec![],
        };
        let result = copy_repo_content_for_install(&src, &dst, &prompter);

        assert!(result.is_ok());
        assert!(dst.join("only/SKILL.md").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_bulk_fallback_no_skills_dir() {
        let dir = std::env::temp_dir().join("skm_test_install_fallback_nodir");
        std::fs::create_dir_all(&dir).unwrap();
        let result = run_in(&dir, None);
        assert!(result.is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_bulk_fallback_empty_skills_dir() {
        let dir = std::env::temp_dir().join("skm_test_install_fallback_empty");
        std::fs::create_dir_all(dir.join("skills")).unwrap();
        let result = run_in(&dir, None);
        assert!(result.is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_bulk_fallback_single_skill() {
        let dir = std::env::temp_dir().join("skm_test_install_fallback_single");
        std::fs::create_dir_all(dir.join("skills")).unwrap();
        std::fs::write(dir.join("skills/test-skill.md"), "# Test Skill").unwrap();
        let result = run_in(&dir, None);
        assert!(result.is_ok());
        assert!(dir.join(".agents/skills/test skill/test-skill.md").exists());
        assert!(dir.join("skills.lock").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_copy_dir_recursive_creates_dst() {
        let src = std::env::temp_dir().join("skm_test_copy_create_dst_src");
        let dst = std::env::temp_dir().join("skm_test_copy_create_dst_dst");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("file.txt"), "content").unwrap();
        let result = copy_dir_recursive(&src, &dst);
        assert!(result.is_ok());
        assert!(dst.join("file.txt").exists());
        std::fs::remove_dir_all(&src).unwrap();
        std::fs::remove_dir_all(&dst).unwrap();
    }

    #[test]
    fn test_copy_dir_recursive_nested() {
        let src = std::env::temp_dir().join("skm_test_copy_nested_src");
        let dst = std::env::temp_dir().join("skm_test_copy_nested_dst");
        std::fs::create_dir_all(src.join("a/b/c")).unwrap();
        std::fs::write(src.join("a/b/c/file.txt"), "deep").unwrap();
        let result = copy_dir_recursive(&src, &dst);
        assert!(result.is_ok());
        assert!(dst.join("a/b/c/file.txt").exists());
        std::fs::remove_dir_all(&src).unwrap();
        std::fs::remove_dir_all(&dst).unwrap();
    }

    #[test]
    fn test_copy_dir_recursive_skips_directories() {
        let src = std::env::temp_dir().join("skm_test_copy_skip_src");
        let dst = std::env::temp_dir().join("skm_test_copy_skip_dst");
        std::fs::create_dir_all(src.join(".git/objects")).unwrap();
        std::fs::create_dir_all(src.join("target/debug")).unwrap();
        std::fs::create_dir_all(src.join("node_modules/pkg")).unwrap();
        std::fs::write(src.join("keep.txt"), "keep").unwrap();
        let result = copy_dir_recursive(&src, &dst);
        assert!(result.is_ok());
        assert!(dst.join("keep.txt").exists());
        assert!(!dst.join(".git").exists());
        assert!(!dst.join("target").exists());
        assert!(!dst.join("node_modules").exists());
        std::fs::remove_dir_all(&src).unwrap();
        std::fs::remove_dir_all(&dst).unwrap();
    }

    #[test]
    fn test_run_bulk_with_manifest_success() {
        let dir = std::env::temp_dir().join("skm_test_bulk_manifest_success");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), r#"{"skills": {}, "exports": {}}"#).unwrap();
        let result = run_bulk_with_manifest(&dir, &dir.join("skills.json"));
        assert!(result.is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_bulk_with_manifest_empty() {
        let dir = std::env::temp_dir().join("skm_test_bulk_manifest_empty");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), r#"{"skills": {}, "exports": {}}"#).unwrap();
        let result = run_bulk_with_manifest(&dir, &dir.join("skills.json"));
        assert!(result.is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_bulk_with_manifest_invalid() {
        let dir = std::env::temp_dir().join("skm_test_bulk_manifest_invalid");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), "not json").unwrap();
        let result = run_bulk_with_manifest(&dir, &dir.join("skills.json"));
        assert!(result.is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_bulk_with_fallback_no_skills_dir() {
        let dir = std::env::temp_dir().join("skm_test_bulk_fallback_nodir");
        std::fs::create_dir_all(&dir).unwrap();
        let result = run_bulk_with_fallback(&dir);
        assert!(result.is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_bulk_with_fallback_empty() {
        let dir = std::env::temp_dir().join("skm_test_bulk_fallback_empty2");
        std::fs::create_dir_all(dir.join("skills")).unwrap();
        let result = run_bulk_with_fallback(&dir);
        assert!(result.is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_bulk_with_manifest_already_installed() {
        let dir = std::env::temp_dir().join("skm_test_bulk_manifest_installed");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("skills.json"),
            r#"{"skills": {"test": {"repo": "url"}}, "exports": {}}"#,
        )
        .unwrap();
        std::fs::write(
            dir.join("skills.lock"),
            r#"{"test": {"commit": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2", "repo": "url"}}"#,
        )
        .unwrap();
        std::fs::create_dir_all(dir.join(".agents/skills/test")).unwrap();
        let result = run_bulk_with_manifest(&dir, &dir.join("skills.json"));
        assert!(result.is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_run_bulk_with_manifest_clone_fails() {
        let dir = std::env::temp_dir().join("skm_test_bulk_manifest_clonefail");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let missing_repo = dir.join("missing-repo");
        std::fs::write(
            dir.join("skills.json"),
            format!(
                r#"{{"skills": {{"test": {{"repo": "{}"}}}}, "exports": {{}}}}"#,
                missing_repo.display()
            ),
        )
        .unwrap();
        let result = run_bulk_with_manifest(&dir, &dir.join("skills.json"));
        assert!(result.is_ok());
        assert!(!dir.join("skills.lock").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    fn create_local_repo(root: &Path, name: &str) -> std::path::PathBuf {
        let repo = root.join(name);
        std::fs::create_dir_all(repo.join("skills").join(name)).unwrap();
        std::fs::write(repo.join("skills").join(name).join("SKILL.md"), "# Test").unwrap();
        std::fs::write(repo.join("README.md"), "not exported").unwrap();
        std::fs::write(
            repo.join("skills.json"),
            format!(
                r#"{{"skills": {{}}, "exports": {{"{}": {{"path": "skills/{}"}}}}}}"#,
                name, name
            ),
        )
        .unwrap();
        run_git(&repo, &["init"]);
        run_git(&repo, &["add", "."]);
        run_git(
            &repo,
            &[
                "-c",
                "user.name=skm tests",
                "-c",
                "user.email=skm-tests@example.com",
                "-c",
                "commit.gpgsign=false",
                "commit",
                "-m",
                "initial fixture",
            ],
        );
        repo
    }

    fn run_git(repo: &Path, args: &[&str]) {
        let output = Command::new("git")
            .args(args)
            .current_dir(repo)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
