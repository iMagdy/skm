use crate::git;
use crate::lockfile::Lockfile;
use crate::manifest::Manifest;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let project_root = std::env::current_dir()?;
    let manifest_path = project_root.join("skills.json");
    let lockfile_path = project_root.join("skills.lock");

    let manifest = if manifest_path.exists() {
        Manifest::load(&manifest_path)?
    } else {
        Manifest::new()
    };

    let lockfile = Lockfile::load(&lockfile_path)?;

    if manifest.skills.is_empty() && lockfile.entries().is_empty() {
        println!("No skills installed. Run 'skm install' to add skills.");
        return Ok(());
    }

    println!(
        "{:<20} {:<45} {:<42} {}",
        "NAME", "REPO", "COMMIT", "STATUS"
    );
    println!("{}", "-".repeat(120));

    for (name, entry) in &manifest.skills {
        let lock = lockfile.entry(name);
        let commit = lock.map(|l| l.commit.as_str()).unwrap_or("—");
        let dir = git::skill_dir(&project_root, name);
        let status = if dir.exists() {
            "installed"
        } else if lock.is_some() {
            "missing"
        } else {
            "not locked"
        };

        println!("{:<20} {:<45} {:<42} {}", name, entry.repo, commit, status);
    }

    // Show orphaned lockfile entries
    for (name, lock) in lockfile.entries() {
        if !manifest.skills.contains_key(name) {
            println!(
                "{:<20} {:<45} {:<42} {}",
                name, lock.repo, lock.commit, "orphaned"
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
        let dir = std::env::temp_dir().join("skm_test_list_empty");
        std::fs::create_dir_all(&dir).unwrap();
        std::env::set_current_dir(&dir).unwrap();
        let result = run();
        assert!(result.is_ok());
        std::env::set_current_dir("/Users/imagdy/dev/skills").unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_list_with_manifest() {
        let dir = std::env::temp_dir().join("skm_test_list_manifest");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("skills.json"),
            r#"{"skills": {"test": {"repo": "url"}}, "exports": {}}"#,
        )
        .unwrap();
        std::fs::create_dir_all(dir.join(".agents/skills/test")).unwrap();
        std::env::set_current_dir(&dir).unwrap();
        let result = run();
        assert!(result.is_ok());
        std::env::set_current_dir("/Users/imagdy/dev/skills").unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_list_with_lockfile() {
        let dir = std::env::temp_dir().join("skm_test_list_lockfile");
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
        std::env::set_current_dir(&dir).unwrap();
        let result = run();
        assert!(result.is_ok());
        std::env::set_current_dir("/Users/imagdy/dev/skills").unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_list_orphaned() {
        let dir = std::env::temp_dir().join("skm_test_list_orphaned");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), r#"{"skills": {}, "exports": {}}"#).unwrap();
        std::fs::write(
            dir.join("skills.lock"),
            r#"{"orphan": {"commit": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2", "repo": "url"}}"#,
        )
        .unwrap();
        std::env::set_current_dir(&dir).unwrap();
        let result = run();
        assert!(result.is_ok());
        std::env::set_current_dir("/Users/imagdy/dev/skills").unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
