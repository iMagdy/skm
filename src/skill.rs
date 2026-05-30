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
