use std::path::Path;

use crate::error::{ManifestInvalidName, SkillCopyFailed};
use crate::git;
use crate::lockfile::Lockfile;
use crate::manifest::{ExportEntry, Manifest};
use crate::ui;

#[cfg(not(tarpaulin_include))]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let project_root = std::env::current_dir()?;
    run_in(&project_root)
}

#[cfg(not(tarpaulin_include))]
pub fn run_add(name: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let project_root = std::env::current_dir()?;
    run_add_in(&project_root, name, path)
}

pub(crate) fn run_in(project_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let manifest_path = project_root.join("skills.json");
    let mut manifest = if manifest_path.exists() {
        Manifest::load(&manifest_path)?
    } else {
        Manifest::new()
    };

    let lockfile = Lockfile::load(&project_root.join("skills.lock"))?;
    let mut exported = 0usize;

    for (name, entry) in lockfile.entries() {
        manifest.add_skill_with_source(name.clone(), entry.repo.clone(), entry.skill.clone());
        exported += 1;
    }

    let skills_root = project_root.join(".agents").join("skills");
    if skills_root.exists() {
        for entry in std::fs::read_dir(&skills_root)? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }

            let name = entry.file_name().to_string_lossy().to_string();
            if manifest.has_skill(&name) {
                continue;
            }

            let path = git::skill_dir(project_root, &name);
            manifest.add_skill(name, path.to_string_lossy().to_string());
            exported += 1;
        }
    }

    manifest.save(&manifest_path)?;

    if exported == 0 {
        ui::info("No skills to export. Created an empty skills.json manifest.");
    } else {
        ui::success(format!(
            "Exported {} skill{} to {}",
            exported,
            if exported == 1 { "" } else { "s" },
            manifest_path.display()
        ));
    }

    Ok(())
}

pub(crate) fn run_add_in(
    project_root: &Path,
    name: &str,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !is_valid_skill_name(name) {
        return Err(ManifestInvalidName {
            message: format!("Invalid export name '{}': must match [a-zA-Z0-9_-]+", name),
        }
        .into());
    }

    let raw_path = Path::new(path);
    let absolute_path = if raw_path.is_absolute() {
        raw_path.to_path_buf()
    } else {
        project_root.join(raw_path)
    };

    if !absolute_path.exists() {
        return Err(SkillCopyFailed {
            message: format!("Export path '{}' does not exist", path),
        }
        .into());
    }

    let project_root = project_root.canonicalize()?;
    let absolute_path = absolute_path.canonicalize()?;
    if !absolute_path.starts_with(&project_root) {
        return Err(SkillCopyFailed {
            message: format!("Export path '{}' is outside the project", path),
        }
        .into());
    }

    let relative_path = absolute_path.strip_prefix(&project_root)?.to_string_lossy();
    let manifest_path = project_root.join("skills.json");
    let mut manifest = if manifest_path.exists() {
        Manifest::load(&manifest_path)?
    } else {
        Manifest::new()
    };
    manifest.exports.insert(
        name.to_string(),
        ExportEntry {
            path: relative_path.to_string(),
        },
    );
    manifest.save(&manifest_path)?;
    ui::success(format!(
        "Exported {} from {}",
        ui::skill_name(name),
        relative_path
    ));

    Ok(())
}

fn is_valid_skill_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lockfile::{LockEntry, Lockfile};

    #[test]
    fn test_export_creates_manifest_when_empty() {
        let dir = std::env::temp_dir().join("ktesio_test_export_empty");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let result = run_in(&dir);

        assert!(result.is_ok());
        assert!(dir.join("skills.json").exists());

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_export_uses_lockfile_entries() {
        let dir = std::env::temp_dir().join("ktesio_test_export_lockfile");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let mut lockfile = Lockfile::new();
        lockfile.insert(
            "example".to_string(),
            LockEntry {
                commit: "a".repeat(40),
                repo: "https://github.com/example/skill.git".to_string(),
                skill: None,
            },
        );
        lockfile.save(&dir.join("skills.lock")).unwrap();

        let result = run_in(&dir);

        assert!(result.is_ok());
        let manifest = Manifest::load(&dir.join("skills.json")).unwrap();
        assert_eq!(
            manifest.skills["example"].repo,
            "https://github.com/example/skill.git"
        );

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_export_keeps_existing_exports() {
        let dir = std::env::temp_dir().join("ktesio_test_export_keeps_exports");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("skills.json"),
            r#"{"skills": {}, "exports": {"local": {"path": "skills/local"}}}"#,
        )
        .unwrap();

        let result = run_in(&dir);

        assert!(result.is_ok());
        let manifest = Manifest::load(&dir.join("skills.json")).unwrap();
        assert!(manifest.exports.contains_key("local"));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_export_adds_untracked_skill_directory() {
        let dir = std::env::temp_dir().join("ktesio_test_export_untracked");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".agents/skills/local")).unwrap();

        let result = run_in(&dir);

        assert!(result.is_ok());
        let manifest = Manifest::load(&dir.join("skills.json")).unwrap();
        assert!(manifest.has_skill("local"));
        assert!(manifest.skills["local"]
            .repo
            .ends_with(".agents/skills/local"));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_export_skips_files_in_skills_directory() {
        let dir = std::env::temp_dir().join("ktesio_test_export_skips_files");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".agents/skills")).unwrap();
        std::fs::write(dir.join(".agents/skills/not-a-skill.md"), "# no").unwrap();

        let result = run_in(&dir);

        assert!(result.is_ok());
        let manifest = Manifest::load(&dir.join("skills.json")).unwrap();
        assert!(!manifest.has_skill("not-a-skill.md"));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_export_add_creates_manifest() {
        let dir = std::env::temp_dir().join("ktesio_test_export_add_create");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("skills/docs")).unwrap();

        let result = run_add_in(&dir, "docs", "skills/docs");

        assert!(result.is_ok());
        let manifest = Manifest::load(&dir.join("skills.json")).unwrap();
        assert_eq!(manifest.exports["docs"].path, "skills/docs");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_export_add_updates_existing_export_and_preserves_imports() {
        let dir = std::env::temp_dir().join("ktesio_test_export_add_update");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("skills/new")).unwrap();
        std::fs::write(
            dir.join("skills.json"),
            r#"{"skills": {"remote": {"repo": "url"}}, "exports": {"docs": {"path": "old"}}}"#,
        )
        .unwrap();

        let result = run_add_in(&dir, "docs", "skills/new");

        assert!(result.is_ok());
        let manifest = Manifest::load(&dir.join("skills.json")).unwrap();
        assert_eq!(manifest.skills["remote"].repo, "url");
        assert_eq!(manifest.exports["docs"].path, "skills/new");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_export_add_rejects_invalid_name() {
        let dir = std::env::temp_dir().join("ktesio_test_export_add_bad_name");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("skills/docs")).unwrap();

        let result = run_add_in(&dir, "bad name", "skills/docs");

        assert!(result.is_err());
        assert!(!dir.join("skills.json").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_export_add_rejects_missing_path() {
        let dir = std::env::temp_dir().join("ktesio_test_export_add_missing");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let result = run_add_in(&dir, "docs", "skills/docs");

        assert!(result.is_err());
        assert!(!dir.join("skills.json").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_export_add_rejects_path_outside_project() {
        let dir = std::env::temp_dir().join("ktesio_test_export_add_outside");
        let outside = std::env::temp_dir().join("ktesio_test_export_add_outside_file");
        let _ = std::fs::remove_dir_all(&dir);
        let _ = std::fs::remove_file(&outside);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(&outside, "# Outside").unwrap();

        let result = run_add_in(&dir, "docs", outside.to_str().unwrap());

        assert!(result.is_err());
        assert!(!dir.join("skills.json").exists());
        std::fs::remove_dir_all(&dir).unwrap();
        std::fs::remove_file(&outside).unwrap();
    }
}
