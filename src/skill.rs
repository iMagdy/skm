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
        copy_dir_recursive(source_repo, dest_dir)?;
        return Ok(());
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
    source_manifest: Option<&Manifest>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(m) = source_manifest {
        copy_skill_files(clone_dir, dest_dir, m)?;
    } else {
        copy_dir_recursive(clone_dir, dest_dir)?;
    }
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
        assert!(copy_skill_files(&source, &dest, &manifest).is_ok());
        assert!(dest.join("file.txt").exists());
        std::fs::remove_dir_all(&source).unwrap();
        std::fs::remove_dir_all(&dest).unwrap();
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
        assert!(copy_cloned_repo_to_dest(&src, &dst, Some(&m)).is_ok());
        assert!(dst.join("test/f.txt").exists());
        std::fs::remove_dir_all(&src).unwrap();
        std::fs::remove_dir_all(&dst).unwrap();
    }

    #[test]
    fn test_copy_cloned_repo_without_manifest() {
        let src = std::env::temp_dir().join("skm_test_clone_without_src");
        let dst = std::env::temp_dir().join("skm_test_clone_without_dst");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("f.txt"), "c").unwrap();
        assert!(copy_cloned_repo_to_dest(&src, &dst, None).is_ok());
        assert!(dst.join("f.txt").exists());
        std::fs::remove_dir_all(&src).unwrap();
        std::fs::remove_dir_all(&dst).unwrap();
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
}
