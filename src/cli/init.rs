use std::path::Path;

use crate::error::InitPathNotFound;
use crate::manifest::Manifest;

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
        eprintln!(
            "skills.json already exists at {}, skipping",
            manifest_path.display()
        );
        return Ok(());
    }

    let manifest = Manifest::new();
    manifest.save(&manifest_path)?;

    println!("Created skills.json at {}", manifest_path.display());
    Ok(())
}
