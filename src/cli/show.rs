use crate::error::SkillNotFound;
use crate::git;
use crate::lockfile::Lockfile;
use crate::manifest::Manifest;

pub fn run(package_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let project_root = std::env::current_dir()?;
    let manifest_path = project_root.join("skills.json");
    let lockfile_path = project_root.join("skills.lock");

    let manifest = if manifest_path.exists() {
        Manifest::load(&manifest_path)?
    } else {
        Manifest::new()
    };

    let lockfile = Lockfile::load(&lockfile_path)?;

    let entry = manifest.skills.get(package_name);
    let lock = lockfile.entry(package_name);

    if entry.is_none() && lock.is_none() {
        return Err(SkillNotFound {
            message: format!("Error: skill '{}' not found", package_name),
        }
        .into());
    }

    let repo = entry
        .map(|e| e.repo.as_str())
        .or_else(|| lock.map(|l| l.repo.as_str()))
        .unwrap_or("—");
    let commit = lock.map(|l| l.commit.as_str()).unwrap_or("—");
    let dir = git::skill_dir(&project_root, package_name);
    let status = if dir.exists() {
        "installed"
    } else if lock.is_some() {
        "missing"
    } else {
        "not installed"
    };

    println!("Name:    {}", package_name);
    println!("Repo:    {}", repo);
    println!("Commit:  {}", commit);
    println!("Path:    {}", dir.display());
    println!("Status:  {}", status);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_not_found() {
        let dir = std::env::temp_dir().join("skm_test_show_notfound");
        std::fs::create_dir_all(&dir).unwrap();
        std::env::set_current_dir(&dir).unwrap();
        let result = run("nonexistent");
        assert!(result.is_err());
        std::env::set_current_dir("/Users/imagdy/dev/skills").unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_show_found() {
        let dir = std::env::temp_dir().join("skm_test_show_found");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("skills.json"),
            r#"{"skills": {"test": {"repo": "url"}}, "exports": {}}"#,
        )
        .unwrap();
        std::fs::create_dir_all(dir.join(".agents/skills/test")).unwrap();
        std::env::set_current_dir(&dir).unwrap();
        let result = run("test");
        assert!(result.is_ok());
        std::env::set_current_dir("/Users/imagdy/dev/skills").unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_show_in_lockfile_only() {
        let dir = std::env::temp_dir().join("skm_test_show_lockonly");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), r#"{"skills": {}, "exports": {}}"#).unwrap();
        std::fs::write(
            dir.join("skills.lock"),
            r#"{"test": {"commit": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2", "repo": "url"}}"#,
        )
        .unwrap();
        std::env::set_current_dir(&dir).unwrap();
        let result = run("test");
        assert!(result.is_ok());
        std::env::set_current_dir("/Users/imagdy/dev/skills").unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
