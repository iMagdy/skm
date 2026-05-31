use std::path::Path;

use crate::error::InitPathNotFound;
use crate::manifest::Manifest;
use crate::ui;

pub fn run(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir = Path::new(path);
    if !dir.exists() {
        return Err(InitPathNotFound {
            message: format!("Error: path '{}' does not exist", path),
        }
        .into());
    }

    let manifest_path = dir.join("skills.json");
    if manifest_path.exists() {
        ui::warning(format!(
            "skills.json already exists at {}, skipping",
            manifest_path.display()
        ));
        return Ok(());
    }

    let manifest = Manifest::new();
    manifest.save(&manifest_path)?;

    ui::success(format!(
        "Created skills.json at {}",
        manifest_path.display()
    ));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_new() {
        let dir = std::env::temp_dir().join("ktesio_test_init_new");
        std::fs::create_dir_all(&dir).unwrap();
        let result = run(dir.to_str().unwrap());
        assert!(result.is_ok());
        assert!(dir.join("skills.json").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_init_already_exists() {
        let dir = std::env::temp_dir().join("ktesio_test_init_exists");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), "{}").unwrap();
        let result = run(dir.to_str().unwrap());
        assert!(result.is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_init_path_not_found() {
        let result = run("/nonexistent/path");
        assert!(result.is_err());
    }
}
