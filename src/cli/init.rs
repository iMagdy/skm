use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::InitPathNotFound;
use crate::git;
use crate::lockfile::{LockEntry, Lockfile};
use crate::manifest::Manifest;
use crate::skills_sh::{self, SkillSearchResult};
use crate::ui;

const LOCAL_COMMIT: &str = "0000000000000000000000000000000000000000";

#[derive(Debug, Clone)]
struct RemoteAdoption {
    repo: String,
    commit: String,
    source_skill: Option<String>,
}

pub fn run(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    run_with_remote_resolver(path, resolve_known_skill)
}

fn run_with_remote_resolver<F>(
    path: &str,
    mut resolve_remote: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(&str) -> Result<Option<RemoteAdoption>, Box<dyn std::error::Error>>,
{
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

    let mut manifest = Manifest::new();
    let lockfile_path = dir.join("skills.lock");
    let mut lockfile = Lockfile::load(&lockfile_path)?;
    let lockfile_changed =
        adopt_existing_local_skills(dir, &mut manifest, &mut lockfile, &mut resolve_remote)?;
    manifest.save(&manifest_path)?;
    if lockfile_changed {
        lockfile.save(&lockfile_path)?;
    }

    ui::success(format!(
        "Created skills.json at {}",
        manifest_path.display()
    ));
    Ok(())
}

fn adopt_existing_local_skills(
    dir: &Path,
    manifest: &mut Manifest,
    lockfile: &mut Lockfile,
    resolve_remote: &mut impl FnMut(&str) -> Result<Option<RemoteAdoption>, Box<dyn std::error::Error>>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let skills_root = dir.join(".agents").join("skills");
    if !skills_root.exists() {
        return Ok(false);
    }

    let mut local_adopted = 0usize;
    let mut remote_adopted = 0usize;
    let mut lockfile_changed = false;
    let mut remote_resolution_enabled = true;

    for entry in std::fs::read_dir(skills_root)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if !is_valid_skill_name(&name) || manifest.has_dependency(&name) {
            continue;
        }

        if let Some(entry) = lockfile.entry(&name) {
            if is_remote_lock_entry(entry) {
                manifest.add_remote_dependency(name.clone(), entry.repo.clone(), None);
                remote_adopted += 1;
                continue;
            }
        }

        if remote_resolution_enabled {
            match resolve_remote(&name) {
                Ok(Some(remote)) if is_valid_commit(&remote.commit) => {
                    let source_skill = remote
                        .source_skill
                        .filter(|source_skill| source_skill != &name);
                    lockfile.insert(
                        name.clone(),
                        LockEntry {
                            commit: remote.commit,
                            repo: remote.repo.clone(),
                            skill: source_skill,
                        },
                    );
                    manifest.add_remote_dependency(name.clone(), remote.repo, None);
                    remote_adopted += 1;
                    lockfile_changed = true;
                    continue;
                }
                Ok(_) => {}
                Err(error) => {
                    remote_resolution_enabled = false;
                    ui::warning(format!(
                        "Remote adoption unavailable after checking '{}': {}. Remaining existing skills will be adopted as local path dependencies.",
                        name, error
                    ));
                }
            }
        }

        manifest.add_local_dependency(name.clone(), format!(".agents/skills/{name}"));
        local_adopted += 1;
    }

    if remote_adopted > 0 {
        ui::info(format!(
            "Resolved {} existing skill director{} as remote dependencies.",
            remote_adopted,
            if remote_adopted == 1 { "y" } else { "ies" }
        ));
    }

    if local_adopted > 0 {
        ui::info(format!(
            "Adopted {} existing local skill director{} as dependencies.",
            local_adopted,
            if local_adopted == 1 { "y" } else { "ies" }
        ));
    }

    Ok(lockfile_changed)
}

fn resolve_known_skill(name: &str) -> Result<Option<RemoteAdoption>, Box<dyn std::error::Error>> {
    let mut notify = |message| ui::warning(message);
    let results = skills_sh::search(name, 10, &mut notify)?;
    let Some(result) = exact_installable_match(name, &results) else {
        return Ok(None);
    };
    let Some(repo) = result.repo.clone() else {
        return Ok(None);
    };

    let commit = resolve_remote_head(&repo, name)?;
    Ok(Some(RemoteAdoption {
        repo,
        commit,
        source_skill: Some(result.skill.clone()),
    }))
}

fn exact_installable_match<'a>(
    name: &str,
    results: &'a [SkillSearchResult],
) -> Option<&'a SkillSearchResult> {
    results
        .iter()
        .filter(|result| result.installable && result.repo.is_some())
        .find(|result| result.skill == name)
}

fn resolve_remote_head(repo: &str, name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let clone_dir = std::env::temp_dir().join(format!(
        "ktesio-init-adopt-{}-{}-{}",
        sanitize_temp_name(name),
        std::process::id(),
        unique_suffix()
    ));
    let _ = std::fs::remove_dir_all(&clone_dir);

    let result = (|| {
        git::clone_silent(repo, &clone_dir)?;
        git::rev_parse_head(&clone_dir)
    })();

    let _ = std::fs::remove_dir_all(&clone_dir);
    result
}

fn is_remote_lock_entry(entry: &LockEntry) -> bool {
    !entry.repo.trim().is_empty() && entry.commit != LOCAL_COMMIT && is_valid_commit(&entry.commit)
}

fn is_valid_commit(commit: &str) -> bool {
    commit.len() == 40 && commit.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn sanitize_temp_name(name: &str) -> String {
    name.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect()
}

fn unique_suffix() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default()
}

fn is_valid_skill_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
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

    #[test]
    fn test_init_adopts_existing_skill_from_resolver_as_remote_dependency_and_lock() {
        let dir = std::env::temp_dir().join("ktesio_test_init_remote_adopt");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".agents/skills/docs")).unwrap();

        let result = run_with_remote_resolver(dir.to_str().unwrap(), |name| {
            assert_eq!(name, "docs");
            Ok(Some(RemoteAdoption {
                repo: "https://github.com/example/docs.git".to_string(),
                commit: "a".repeat(40),
                source_skill: Some("docs".to_string()),
            }))
        });

        assert!(result.is_ok());
        let manifest = Manifest::load(&dir.join("skills.json")).unwrap();
        assert_eq!(
            manifest.dependencies["docs"].repo.as_deref(),
            Some("https://github.com/example/docs.git")
        );
        let lockfile = Lockfile::load(&dir.join("skills.lock")).unwrap();
        assert_eq!(lockfile.entry("docs").unwrap().commit, "a".repeat(40));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_init_adopts_unmatched_existing_skill_as_local_dependency() {
        let dir = std::env::temp_dir().join("ktesio_test_init_local_adopt");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".agents/skills/local")).unwrap();

        let result = run_with_remote_resolver(dir.to_str().unwrap(), |_| Ok(None));

        assert!(result.is_ok());
        let manifest = Manifest::load(&dir.join("skills.json")).unwrap();
        assert_eq!(
            manifest.dependencies["local"].path.as_deref(),
            Some(".agents/skills/local")
        );
        assert!(manifest.publish.is_empty());
        assert!(!dir.join("skills.lock").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
