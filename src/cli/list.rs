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
