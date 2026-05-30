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

    let repo = entry.map(|e| e.repo.as_str()).or_else(|| lock.map(|l| l.repo.as_str())).unwrap_or("—");
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
