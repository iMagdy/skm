use std::fs;
use std::path::Path;

use crate::error::SkillCopyFailed;
use crate::manifest::Manifest;

pub fn copy_skill_files(
    source_repo: &Path,
    dest_dir: &Path,
    manifest: &Manifest,
) -> Result<(), Box<dyn std::error::Error>> {
    if manifest.exports.is_empty() {
        return Err(SkillCopyFailed {
            message: "Source skills.json does not declare any exports".to_string(),
        }
        .into());
    }

    for (name, export) in &manifest.exports {
        let src_path = source_repo.join(&export.path);
        if !src_path.exists() {
            return Err(SkillCopyFailed {
                message: format!(
                    "Export path '{}' does not exist in source repo",
                    export.path
                ),
            }
            .into());
        }

        let dst_path = dest_dir.join(name);
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).map_err(|e| SkillCopyFailed {
                message: format!(
                    "Failed to copy '{}' to '{}': {}",
                    src_path.display(),
                    dst_path.display(),
                    e
                ),
            })?;
        }
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
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
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

pub fn remove_skill_dir(project_root: &Path, name: &str) -> Result<(), std::io::Error> {
    let dir = crate::git::skill_dir(project_root, name);
    if dir.exists() {
        fs::remove_dir_all(dir)?;
    }
    Ok(())
}

pub fn copy_cloned_repo_to_dest(
    clone_dir: &Path,
    dest_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let source_manifest_path = clone_dir.join("skills.json");
    if !source_manifest_path.exists() {
        return Err(SkillCopyFailed {
            message: "Source repo has no skills.json".to_string(),
        }
        .into());
    }

    let source_manifest = Manifest::load(&source_manifest_path)?;
    if source_manifest.exports.is_empty() {
        return Err(SkillCopyFailed {
            message: "Source skills.json does not declare any exports".to_string(),
        }
        .into());
    }

    copy_skill_files(clone_dir, dest_dir, &source_manifest)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{ExportEntry, Manifest};

    #[test]
    fn test_copy_skill_files_empty_exports() {
        let source = std::env::temp_dir().join("skm_test_empty_exports_src");
        let dest = std::env::temp_dir().join("skm_test_empty_exports_dst");
        std::fs::create_dir_all(&source).unwrap();
        std::fs::write(source.join("file.txt"), "content").unwrap();
        let manifest = Manifest::new();
        assert!(copy_skill_files(&source, &dest, &manifest).is_err());
        assert!(!dest.join("file.txt").exists());
        std::fs::remove_dir_all(&source).unwrap();
        let _ = std::fs::remove_dir_all(&dest);
    }

    #[test]
    fn test_copy_skill_files_with_exports() {
        let source = std::env::temp_dir().join("skm_test_exports_src");
        let dest = std::env::temp_dir().join("skm_test_exports_dst");
        std::fs::create_dir_all(source.join("skills/test")).unwrap();
        std::fs::write(source.join("skills/test/f.txt"), "c").unwrap();
        let mut manifest = Manifest::new();
        manifest.exports.insert(
            "test".to_string(),
            ExportEntry {
                path: "skills/test".to_string(),
            },
        );
        assert!(copy_skill_files(&source, &dest, &manifest).is_ok());
        assert!(dest.join("test/f.txt").exists());
        std::fs::remove_dir_all(&source).unwrap();
        std::fs::remove_dir_all(&dest).unwrap();
    }

    #[test]
    fn test_copy_skill_files_export_not_found() {
        let source = std::env::temp_dir().join("skm_test_notfound_src");
        let dest = std::env::temp_dir().join("skm_test_notfound_dst");
        std::fs::create_dir_all(&source).unwrap();
        let mut manifest = Manifest::new();
        manifest.exports.insert(
            "x".to_string(),
            ExportEntry {
                path: "nope".to_string(),
            },
        );
        assert!(copy_skill_files(&source, &dest, &manifest).is_err());
        std::fs::remove_dir_all(&source).unwrap();
        let _ = std::fs::remove_dir_all(&dest);
    }

    #[test]
    fn test_copy_dir_recursive() {
        let src = std::env::temp_dir().join("skm_test_recursive_src");
        let dst = std::env::temp_dir().join("skm_test_recursive_dst");
        std::fs::create_dir_all(src.join("sub")).unwrap();
        std::fs::write(src.join("a.txt"), "1").unwrap();
        std::fs::write(src.join("sub/b.txt"), "2").unwrap();
        assert!(copy_dir_recursive(&src, &dst).is_ok());
        assert!(dst.join("a.txt").exists());
        assert!(dst.join("sub/b.txt").exists());
        std::fs::remove_dir_all(&src).unwrap();
        std::fs::remove_dir_all(&dst).unwrap();
    }

    #[test]
    fn test_copy_dir_recursive_skips_git() {
        let src = std::env::temp_dir().join("skm_test_skip_git_src");
        let dst = std::env::temp_dir().join("skm_test_skip_git_dst");
        std::fs::create_dir_all(src.join(".git/obj")).unwrap();
        std::fs::write(src.join("f.txt"), "x").unwrap();
        assert!(copy_dir_recursive(&src, &dst).is_ok());
        assert!(dst.join("f.txt").exists());
        assert!(!dst.join(".git").exists());
        std::fs::remove_dir_all(&src).unwrap();
        std::fs::remove_dir_all(&dst).unwrap();
    }

    #[test]
    fn test_copy_dir_recursive_skips_target() {
        let src = std::env::temp_dir().join("skm_test_skip_target_src");
        let dst = std::env::temp_dir().join("skm_test_skip_target_dst");
        std::fs::create_dir_all(src.join("target/debug")).unwrap();
        std::fs::write(src.join("f.txt"), "x").unwrap();
        assert!(copy_dir_recursive(&src, &dst).is_ok());
        assert!(dst.join("f.txt").exists());
        assert!(!dst.join("target").exists());
        std::fs::remove_dir_all(&src).unwrap();
        std::fs::remove_dir_all(&dst).unwrap();
    }

    #[test]
    fn test_copy_cloned_repo_with_manifest() {
        let src = std::env::temp_dir().join("skm_test_clone_with_src");
        let dst = std::env::temp_dir().join("skm_test_clone_with_dst");
        std::fs::create_dir_all(src.join("skills/test")).unwrap();
        std::fs::write(src.join("skills/test/f.txt"), "c").unwrap();
        let mut m = Manifest::new();
        m.exports.insert(
            "test".to_string(),
            ExportEntry {
                path: "skills/test".to_string(),
            },
        );
        std::fs::write(src.join("skills.json"), serde_json::to_string(&m).unwrap()).unwrap();
        std::fs::write(src.join("README.md"), "not exported").unwrap();
        assert!(copy_cloned_repo_to_dest(&src, &dst).is_ok());
        assert!(dst.join("test/f.txt").exists());
        assert!(!dst.join("README.md").exists());
        std::fs::remove_dir_all(&src).unwrap();
        std::fs::remove_dir_all(&dst).unwrap();
    }

    #[test]
    fn test_copy_cloned_repo_with_exports_only_manifest() {
        let src = std::env::temp_dir().join("skm_test_clone_exports_only_src");
        let dst = std::env::temp_dir().join("skm_test_clone_exports_only_dst");
        std::fs::create_dir_all(src.join("skills/test")).unwrap();
        std::fs::write(src.join("skills/test/SKILL.md"), "content").unwrap();
        std::fs::write(
            src.join("skills.json"),
            r#"{"exports": {"test": {"path": "skills/test"}}}"#,
        )
        .unwrap();

        assert!(copy_cloned_repo_to_dest(&src, &dst).is_ok());
        assert!(dst.join("test/SKILL.md").exists());

        std::fs::remove_dir_all(&src).unwrap();
        std::fs::remove_dir_all(&dst).unwrap();
    }

    #[test]
    fn test_copy_cloned_repo_without_manifest() {
        let src = std::env::temp_dir().join("skm_test_clone_without_src");
        let dst = std::env::temp_dir().join("skm_test_clone_without_dst");
        std::fs::create_dir_all(src.join("skills/fallback")).unwrap();
        std::fs::write(src.join("skills/fallback/SKILL.md"), "c").unwrap();
        std::fs::write(src.join("README.md"), "not a skill").unwrap();
        assert!(copy_cloned_repo_to_dest(&src, &dst).is_err());
        assert!(!dst.join("fallback/SKILL.md").exists());
        assert!(!dst.join("README.md").exists());
        std::fs::remove_dir_all(&src).unwrap();
        let _ = std::fs::remove_dir_all(&dst);
    }

    #[test]
    fn test_copy_cloned_repo_without_manifest_or_skills_dir_fails() {
        let src = std::env::temp_dir().join("skm_test_clone_no_exports_src");
        let dst = std::env::temp_dir().join("skm_test_clone_no_exports_dst");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("README.md"), "not a skill").unwrap();
        assert!(copy_cloned_repo_to_dest(&src, &dst).is_err());
        std::fs::remove_dir_all(&src).unwrap();
        let _ = std::fs::remove_dir_all(&dst);
    }

    #[test]
    fn test_remove_skill_dir() {
        let root = std::env::temp_dir().join("skm_test_rm_dir");
        std::fs::create_dir_all(root.join(".agents/skills/test")).unwrap();
        assert!(remove_skill_dir(&root, "test").is_ok());
        assert!(!root.join(".agents/skills/test").exists());
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn test_remove_skill_dir_not_exists() {
        let root = std::env::temp_dir().join("skm_test_rm_dir_ne");
        std::fs::create_dir_all(&root).unwrap();
        assert!(remove_skill_dir(&root, "x").is_ok());
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn test_copy_skill_files_with_file_export() {
        let source = std::env::temp_dir().join("skm_test_file_export_src");
        let dest = std::env::temp_dir().join("skm_test_file_export_dst");
        std::fs::create_dir_all(&source).unwrap();
        std::fs::create_dir_all(&dest).unwrap();
        std::fs::write(source.join("skill.md"), "content").unwrap();
        let mut manifest = Manifest::new();
        manifest.exports.insert(
            "my-skill".to_string(),
            ExportEntry {
                path: "skill.md".to_string(),
            },
        );
        let result = copy_skill_files(&source, &dest, &manifest);
        assert!(
            result.is_ok(),
            "copy_skill_files failed: {:?}",
            result.err()
        );
        assert!(dest.join("my-skill").exists());
        std::fs::remove_dir_all(&source).unwrap();
        std::fs::remove_dir_all(&dest).unwrap();
    }

    #[test]
    fn test_copy_skill_files_file_export_reports_copy_error() {
        let source = std::env::temp_dir().join("skm_test_file_export_error_src");
        let dest = std::env::temp_dir().join("skm_test_file_export_error_dst");
        let _ = std::fs::remove_dir_all(&source);
        let _ = std::fs::remove_dir_all(&dest);
        std::fs::create_dir_all(&source).unwrap();
        std::fs::write(source.join("skill.md"), "content").unwrap();
        let mut manifest = Manifest::new();
        manifest.exports.insert(
            "my-skill".to_string(),
            ExportEntry {
                path: "skill.md".to_string(),
            },
        );

        let result = copy_skill_files(&source, &dest, &manifest);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to copy"));
        std::fs::remove_dir_all(&source).unwrap();
    }

    #[test]
    fn test_copy_cloned_repo_with_empty_exports_manifest() {
        let src = std::env::temp_dir().join("skm_test_clone_empty_exports_src");
        let dst = std::env::temp_dir().join("skm_test_clone_empty_exports_dst");
        let _ = std::fs::remove_dir_all(&src);
        let _ = std::fs::remove_dir_all(&dst);
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("skills.json"), r#"{"skills": {}, "exports": {}}"#).unwrap();

        let result = copy_cloned_repo_to_dest(&src, &dst);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("does not declare any exports"));
        std::fs::remove_dir_all(&src).unwrap();
        let _ = std::fs::remove_dir_all(&dst);
    }

    #[test]
    fn test_copy_dir_recursive_skips_node_modules() {
        let src = std::env::temp_dir().join("skm_test_skip_nm_src");
        let dst = std::env::temp_dir().join("skm_test_skip_nm_dst");
        std::fs::create_dir_all(src.join("node_modules/pkg")).unwrap();
        std::fs::write(src.join("f.txt"), "x").unwrap();
        assert!(copy_dir_recursive(&src, &dst).is_ok());
        assert!(dst.join("f.txt").exists());
        assert!(!dst.join("node_modules").exists());
        std::fs::remove_dir_all(&src).unwrap();
        std::fs::remove_dir_all(&dst).unwrap();
    }
}
