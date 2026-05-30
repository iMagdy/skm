use crate::error::SkillNotFound;
use crate::lockfile::Lockfile;
use crate::manifest::Manifest;
use crate::skill;

pub fn run(package_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let project_root = std::env::current_dir()?;
    let manifest_path = project_root.join("skills.json");

    let mut manifest = if manifest_path.exists() {
        Manifest::load(&manifest_path)?
    } else {
        return Err(SkillNotFound {
            message: format!("Error: skill '{}' not found in manifest", package_name),
        }
        .into());
    };

    if !manifest.has_skill(package_name) {
        return Err(SkillNotFound {
            message: format!("Error: skill '{}' not found in manifest", package_name),
        }
        .into());
    }

    manifest.remove_skill(package_name);
    manifest.save(&manifest_path)?;

    let lockfile_path = project_root.join("skills.lock");
    let mut lockfile = Lockfile::load(&lockfile_path)?;
    lockfile.remove(package_name);
    lockfile.save(&lockfile_path)?;

    skill::remove_skill_dir(&project_root, package_name)?;

    println!("Uninstalled {}", package_name);
    Ok(())
}
