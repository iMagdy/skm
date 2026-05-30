use std::path::Path;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::error::{InstallAlreadyExists, InstallInvalidFormat, ManifestNotFound};
use crate::git;
use crate::lockfile::{LockEntry, Lockfile};
use crate::manifest::Manifest;
use crate::skill;

pub fn run(target: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let project_root = std::env::current_dir()?;

    if let Some(t) = target {
        return run_single(&project_root, t);
    }

    run_bulk(&project_root)
}

fn run_bulk(project_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let manifest_path = project_root.join("skills.json");
    if !manifest_path.exists() {
        return Err(ManifestNotFound {
            message: "No skills.json found in current directory. Run 'skm init .' to create one."
                .to_string(),
        }
        .into());
    }

    let manifest = Manifest::load(&manifest_path)?;
    let mut lockfile = Lockfile::load(&project_root.join("skills.lock"))?;

    let mp = MultiProgress::new();
    let mut errors: Vec<String> = Vec::new();

    for (name, entry) in &manifest.skills {
        let skill_dir = git::skill_dir(project_root, name);

        if skill_dir.exists() && lockfile.contains(name) {
            eprintln!("Skill '{}' already installed, skipping", name);
            continue;
        }

        let pb = mp.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.set_message(format!("Cloning {}...", name));

        if let Err(e) = git::clone(&entry.repo, &skill_dir) {
            pb.finish_with_message(format!("Error cloning {}: {}", name, e));
            errors.push(format!("Error cloning {}: {}", name, e));
            continue;
        }

        pb.set_message(format!("Copying files for {}...", name));

        // Try to read source repo's manifest for exports
        let source_manifest_path = skill_dir.join("skills.json");
        let source_manifest = if source_manifest_path.exists() {
            Manifest::load(&source_manifest_path).ok()
        } else {
            None
        };

        if let Err(e) = skill::copy_cloned_repo_to_dest(
            &skill_dir,
            &skill_dir,
            source_manifest.as_ref(),
        ) {
            pb.finish_with_message(format!("Error copying files for {}: {}", name, e));
            errors.push(format!("Error copying files for {}: {}", name, e));
            continue;
        }

        let commit = git::rev_parse_head(&skill_dir).unwrap_or_default();
        lockfile.insert(
            name.clone(),
            LockEntry {
                commit,
                repo: entry.repo.clone(),
            },
        );

        pb.finish_with_message(format!("Installed {}", name));
    }

    lockfile.save(&project_root.join("skills.lock"))?;

    if !errors.is_empty() {
        eprintln!("\nErrors encountered:");
        for err in &errors {
            eprintln!("  {}", err);
        }
    }

    Ok(())
}

fn run_single(project_root: &Path, target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parts: Vec<&str> = target.splitn(2, ':').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err(InstallInvalidFormat {
            message: "Invalid format. Expected name:url (e.g., clap:https://github.com/clap-rs/clap.git)".to_string(),
        }
        .into());
    }

    let name = parts[0].to_string();
    let repo_url = parts[1].to_string();

    let manifest_path = project_root.join("skills.json");
    let mut manifest = if manifest_path.exists() {
        Manifest::load(&manifest_path)?
    } else {
        Manifest::new()
    };

    if manifest.has_skill(&name) {
        return Err(InstallAlreadyExists {
            message: format!("Skill '{}' already exists in manifest, skipping", name),
        }
        .into());
    }

    manifest.add_skill(name.clone(), repo_url.clone());
    manifest.save(&manifest_path)?;

    let skill_dir = git::skill_dir(project_root, &name);
    let mp = MultiProgress::new();
    let pb = mp.add(ProgressBar::new_spinner());
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message(format!("Cloning {}...", name));

    git::clone(&repo_url, &skill_dir)?;

    pb.set_message(format!("Copying files for {}...", name));

    let source_manifest_path = skill_dir.join("skills.json");
    let source_manifest = if source_manifest_path.exists() {
        Manifest::load(&source_manifest_path).ok()
    } else {
        None
    };

    skill::copy_cloned_repo_to_dest(&skill_dir, &skill_dir, source_manifest.as_ref())?;

    let commit = git::rev_parse_head(&skill_dir).unwrap_or_default();
    let mut lockfile = Lockfile::load(&project_root.join("skills.lock"))?;
    lockfile.insert(
        name.clone(),
        LockEntry {
            commit,
            repo: repo_url,
        },
    );
    lockfile.save(&project_root.join("skills.lock"))?;

    pb.finish_with_message(format!("Installed {}", name));
    println!("Installed {} from {}", name, project_root.display());

    Ok(())
}
