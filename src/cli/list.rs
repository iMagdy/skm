use std::path::Path;

use crate::git;
use crate::lockfile::Lockfile;
use crate::manifest::Manifest;
use crate::ui;

#[cfg(not(tarpaulin_include))]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let project_root = std::env::current_dir()?;
    run_in(&project_root)
}

pub(crate) fn run_in(project_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let manifest_path = project_root.join("skills.json");
    let lockfile_path = project_root.join("skills.lock");

    let manifest = if manifest_path.exists() {
        Manifest::load(&manifest_path)?
    } else {
        Manifest::new()
    };

    let lockfile = Lockfile::load(&lockfile_path)?;

    if manifest.skills.is_empty() && lockfile.entries().is_empty() {
        ui::info("No skills installed. Run 'kt install' to add skills.");
        return Ok(());
    }

    println!(
        "{} {} {} {}",
        ui::padded(ui::table_header("NAME"), "NAME", 20),
        ui::padded(ui::table_header("REPO"), "REPO", 45),
        ui::padded(ui::table_header("COMMIT"), "COMMIT", 42),
        ui::table_header("STATUS")
    );
    println!("{}", "-".repeat(120));

    for (name, entry) in &manifest.skills {
        let lock = lockfile.entry(name);
        let commit = lock.map(|l| l.commit.as_str()).unwrap_or("—");
        let dir = git::skill_dir(project_root, name);
        let status = if dir.exists() {
            "installed"
        } else if lock.is_some() {
            "missing"
        } else {
            "not locked"
        };

        println!(
            "{} {} {} {}",
            ui::padded(ui::skill_name(name), name, 20),
            ui::padded(&entry.repo, &entry.repo, 45),
            ui::padded(commit, commit, 42),
            ui::status_label(status)
        );
    }

    // Show orphaned lockfile entries
    for (name, lock) in lockfile.entries() {
        if !manifest.skills.contains_key(name) {
            println!(
                "{} {} {} {}",
                ui::padded(ui::skill_name(name), name, 20),
                ui::padded(&lock.repo, &lock.repo, 45),
                ui::padded(&lock.commit, &lock.commit, 42),
                ui::status_label("orphaned")
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_empty() {
        let dir = std::env::temp_dir().join("ktesio_test_list_empty");
        std::fs::create_dir_all(&dir).unwrap();
        let result = run_in(&dir);
        assert!(result.is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_list_with_manifest() {
        let dir = std::env::temp_dir().join("ktesio_test_list_manifest");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("skills.json"),
            r#"{"skills": {"test": {"repo": "url"}}, "exports": {}}"#,
        )
        .unwrap();
        std::fs::create_dir_all(dir.join(".agents/skills/test")).unwrap();
        let result = run_in(&dir);
        assert!(result.is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_list_with_lockfile() {
        let dir = std::env::temp_dir().join("ktesio_test_list_lockfile");
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
        let result = run_in(&dir);
        assert!(result.is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_list_orphaned() {
        let dir = std::env::temp_dir().join("ktesio_test_list_orphaned");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), r#"{"skills": {}, "exports": {}}"#).unwrap();
        std::fs::write(
            dir.join("skills.lock"),
            r#"{"orphan": {"commit": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2", "repo": "url"}}"#,
        )
        .unwrap();
        let result = run_in(&dir);
        assert!(result.is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_list_missing_status() {
        let dir = std::env::temp_dir().join("ktesio_test_list_missing");
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
        // Don't create the skill directory
        let result = run_in(&dir);
        assert!(result.is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_list_not_locked_status() {
        let dir = std::env::temp_dir().join("ktesio_test_list_notlocked");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("skills.json"),
            r#"{"skills": {"test": {"repo": "url"}}, "exports": {}}"#,
        )
        .unwrap();
        // No lockfile
        let result = run_in(&dir);
        assert!(result.is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
