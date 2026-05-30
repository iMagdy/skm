use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{GitCheckoutFailed, GitCloneFailed, GitFetchFailed, GitRevParseFailed};

pub fn is_git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn clone(url: &str, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(dest)
        .status()
        .map_err(|e| GitCloneFailed {
            message: format!("Failed to run git clone: {}", e),
        })?;

    if !status.success() {
        return Err(GitCloneFailed {
            message: format!("git clone failed for {}", url),
        }
        .into());
    }

    Ok(())
}

pub fn fetch(repo_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("fetch")
        .arg("origin")
        .status()
        .map_err(|e| GitFetchFailed {
            message: format!("Failed to run git fetch: {}", e),
        })?;

    if !status.success() {
        return Err(GitFetchFailed {
            message: format!("git fetch failed in {}", repo_dir.display()),
        }
        .into());
    }

    Ok(())
}

pub fn checkout_default_branch(repo_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let default_branch = resolve_default_branch(repo_dir)?;

    let status = Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("checkout")
        .arg(format!("origin/{}", default_branch))
        .status()
        .map_err(|e| GitCheckoutFailed {
            message: format!("Failed to run git checkout: {}", e),
        })?;

    if !status.success() {
        return Err(GitCheckoutFailed {
            message: format!(
                "git checkout origin/{} failed in {}",
                default_branch,
                repo_dir.display()
            ),
        }
        .into());
    }

    Ok(())
}

pub fn rev_parse_head(repo_dir: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .map_err(|e| GitRevParseFailed {
            message: format!("Failed to run git rev-parse: {}", e),
        })?;

    if !output.status.success() {
        return Err(GitRevParseFailed {
            message: format!("git rev-parse HEAD failed in {}", repo_dir.display()),
        }
        .into());
    }

    let sha = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(sha)
}

pub fn resolve_default_branch(repo_dir: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("symbolic-ref")
        .arg("refs/remotes/origin/HEAD")
        .output()
        .map_err(|e| GitRevParseFailed {
            message: format!("Failed to resolve default branch: {}", e),
        })?;

    if output.status.success() {
        let refname = String::from_utf8_lossy(&output.stdout).trim().to_string();
        // refs/remotes/origin/main -> main
        if let Some(branch) = refname.strip_prefix("refs/remotes/origin/") {
            return Ok(branch.to_string());
        }
    }

    // Fallback: check common branch names
    for branch in &["main", "master"] {
        let check = Command::new("git")
            .arg("-C")
            .arg(repo_dir)
            .arg("rev-parse")
            .arg("--verify")
            .arg(format!("refs/heads/{}", branch))
            .output();

        if let Ok(out) = check {
            if out.status.success() {
                return Ok(branch.to_string());
            }
        }
    }

    Ok("main".to_string())
}

pub fn skill_dir(project_root: &Path, name: &str) -> PathBuf {
    project_root.join(".agents").join("skills").join(name)
}

pub fn is_installed(project_root: &Path, name: &str) -> bool {
    skill_dir(project_root, name).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_git_available() {
        assert!(is_git_available());
    }

    #[test]
    fn test_skill_dir() {
        let root = Path::new("/project");
        let dir = skill_dir(root, "my-skill");
        assert_eq!(dir, PathBuf::from("/project/.agents/skills/my-skill"));
    }

    #[test]
    fn test_is_installed() {
        let dir = std::env::temp_dir().join("skm_test_is_installed");
        std::fs::create_dir_all(dir.join(".agents/skills/test")).unwrap();
        assert!(is_installed(&dir, "test"));
        assert!(!is_installed(&dir, "other"));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_clone_invalid_url() {
        let dir = std::env::temp_dir().join("skm_test_clone_invalid");
        let result = clone("https://invalid.example.com/repo.git", &dir);
        assert!(result.is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_fetch_invalid_dir() {
        let dir = std::env::temp_dir().join("skm_test_fetch_invalid");
        let result = fetch(&dir);
        assert!(result.is_err());
    }

    #[test]
    fn test_rev_parse_head_invalid_dir() {
        let dir = std::env::temp_dir().join("skm_test_revparse_invalid");
        let result = rev_parse_head(&dir);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_default_branch_invalid_dir() {
        let dir = std::env::temp_dir().join("skm_test_resolve_invalid");
        let result = resolve_default_branch(&dir);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "main");
    }

    #[test]
    fn test_checkout_default_branch_invalid_dir() {
        let dir = std::env::temp_dir().join("skm_test_checkout_invalid");
        let result = checkout_default_branch(&dir);
        assert!(result.is_err());
    }

    #[test]
    fn test_clone_to_existing_dir() {
        let dir = std::env::temp_dir().join("skm_test_clone_existing");
        std::fs::create_dir_all(&dir).unwrap();
        let result = clone("https://invalid.example.com/repo.git", &dir);
        assert!(result.is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_fetch_non_git_dir() {
        let dir = std::env::temp_dir().join("skm_test_fetch_non_git");
        std::fs::create_dir_all(&dir).unwrap();
        let result = fetch(&dir);
        assert!(result.is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
