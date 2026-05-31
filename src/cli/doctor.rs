use std::path::Path;

use crate::error::DoctorUnhealthy;
use crate::git;
use crate::lockfile::Lockfile;
use crate::manifest::Manifest;
use crate::ui;

#[cfg(not(tarpaulin_include))]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let project_root = std::env::current_dir()?;
    run_in(&project_root)
}

pub(crate) fn run_in(project_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let report = check_project(project_root);
    print_report(&report);

    if report.errors.is_empty() {
        Ok(())
    } else {
        Err(DoctorUnhealthy {
            message: format!("Project has {} skill health error(s)", report.errors.len()),
        }
        .into())
    }
}

#[derive(Debug, Default)]
struct DoctorReport {
    errors: Vec<String>,
    warnings: Vec<String>,
}

fn check_project(project_root: &Path) -> DoctorReport {
    let mut report = DoctorReport::default();

    if !git::is_git_available() {
        report
            .errors
            .push("git is not available on PATH; install git before installing skills".to_string());
    }

    let manifest_path = project_root.join("skills.json");
    let manifest = if manifest_path.exists() {
        match Manifest::load(&manifest_path) {
            Ok(manifest) => Some(manifest),
            Err(err) => {
                report.errors.push(format!(
                    "skills.json is invalid: {}. Fix the manifest JSON.",
                    err
                ));
                None
            }
        }
    } else {
        report
            .warnings
            .push("skills.json is missing; run 'kt init .' or 'kt export'.".to_string());
        None
    };

    let lockfile_path = project_root.join("skills.lock");
    let lockfile = match Lockfile::load(&lockfile_path) {
        Ok(lockfile) => Some(lockfile),
        Err(err) => {
            report.errors.push(format!(
                "skills.lock is invalid: {}. Regenerate or repair it.",
                err
            ));
            None
        }
    };

    if let Some(manifest) = &manifest {
        for (name, export) in &manifest.exports {
            let export_path = project_root.join(&export.path);
            if !export_path.exists() {
                report.errors.push(format!(
                    "export '{}' points to missing path '{}'; create it or run 'kt export add {} <path>'.",
                    name, export.path, name
                ));
            }
        }
    }

    if let (Some(manifest), Some(lockfile)) = (&manifest, &lockfile) {
        for name in manifest.skills.keys() {
            let skill_dir = git::skill_dir(project_root, name);
            if !skill_dir.exists() && lockfile.contains(name) {
                report.errors.push(format!(
                    "skill '{}' is locked but missing from disk; run 'kt install'.",
                    name
                ));
            } else if !skill_dir.exists() {
                report.warnings.push(format!(
                    "skill '{}' is declared but not installed; run 'kt install'.",
                    name
                ));
            } else if !lockfile.contains(name) {
                report.warnings.push(format!(
                    "skill '{}' is installed but not locked; run 'kt install' or 'kt upgrade'.",
                    name
                ));
            }
        }

        for name in lockfile.entries().keys() {
            if !manifest.skills.contains_key(name) {
                report.warnings.push(format!(
                    "skill '{}' is orphaned in skills.lock; run 'kt export' to restore it or 'kt uninstall {}'.",
                    name, name
                ));
            }
        }
    }

    let skills_root = project_root.join(".agents").join("skills");
    if skills_root.exists() {
        match std::fs::read_dir(&skills_root) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let Ok(file_type) = entry.file_type() else {
                        continue;
                    };
                    if !file_type.is_dir() {
                        continue;
                    }
                    let name = entry.file_name().to_string_lossy().to_string();
                    let known = manifest
                        .as_ref()
                        .map(|manifest| manifest.skills.contains_key(&name))
                        .unwrap_or(false)
                        || lockfile
                            .as_ref()
                            .map(|lockfile| lockfile.contains(&name))
                            .unwrap_or(false);
                    if !known {
                        report.warnings.push(format!(
                            "installed directory '{}' is untracked; run 'kt export' if it should be kept.",
                            name
                        ));
                    }
                }
            }
            Err(err) => report.errors.push(format!(
                "cannot read .agents/skills: {}; check directory permissions.",
                err
            )),
        }
    }

    report
}

fn print_report(report: &DoctorReport) {
    if report.errors.is_empty() && report.warnings.is_empty() {
        ui::success("Project skill state looks healthy.");
        return;
    }

    if !report.errors.is_empty() {
        ui::error("Errors:");
        for error in &report.errors {
            eprintln!("  - {}", error);
        }
    }

    if !report.warnings.is_empty() {
        ui::warning("Warnings:");
        for warning in &report.warnings {
            eprintln!("  - {}", warning);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doctor_healthy_empty_manifest() {
        let dir = std::env::temp_dir().join("ktesio_test_doctor_healthy");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), r#"{"skills": {}, "exports": {}}"#).unwrap();

        let result = check_project(&dir);

        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_doctor_run_in_returns_ok_when_healthy() {
        let dir = std::env::temp_dir().join("ktesio_test_doctor_run_healthy");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), r#"{"skills": {}, "exports": {}}"#).unwrap();

        let result = run_in(&dir);

        assert!(result.is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_doctor_reports_missing_manifest_warning() {
        let dir = std::env::temp_dir().join("ktesio_test_doctor_missing_manifest");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let result = check_project(&dir);

        assert!(result
            .warnings
            .iter()
            .any(|warning| warning.contains("skills.json is missing")));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_doctor_print_report_handles_errors_and_warnings() {
        let report = DoctorReport {
            errors: vec!["broken".to_string()],
            warnings: vec!["stale".to_string()],
        };

        print_report(&report);
    }

    #[test]
    fn test_doctor_reports_missing_locked_skill() {
        let dir = std::env::temp_dir().join("ktesio_test_doctor_missing");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("skills.json"),
            r#"{"skills": {"docs": {"repo": "url"}}, "exports": {}}"#,
        )
        .unwrap();
        std::fs::write(
            dir.join("skills.lock"),
            r#"{"docs": {"commit": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "repo": "url"}}"#,
        )
        .unwrap();

        let result = check_project(&dir);

        assert!(result
            .errors
            .iter()
            .any(|error| error.contains("missing from disk")));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_doctor_reports_invalid_manifest() {
        let dir = std::env::temp_dir().join("ktesio_test_doctor_bad_manifest");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), "not json").unwrap();

        let result = check_project(&dir);

        assert!(result
            .errors
            .iter()
            .any(|error| error.contains("skills.json is invalid")));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_doctor_reports_invalid_lockfile() {
        let dir = std::env::temp_dir().join("ktesio_test_doctor_bad_lock");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), r#"{"skills": {}, "exports": {}}"#).unwrap();
        std::fs::write(dir.join("skills.lock"), "not json").unwrap();

        let result = check_project(&dir);

        assert!(result
            .errors
            .iter()
            .any(|error| error.contains("skills.lock is invalid")));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_doctor_reports_missing_export_path() {
        let dir = std::env::temp_dir().join("ktesio_test_doctor_missing_export");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("skills.json"),
            r#"{"skills": {}, "exports": {"docs": {"path": "skills/docs"}}}"#,
        )
        .unwrap();

        let result = check_project(&dir);

        assert!(result
            .errors
            .iter()
            .any(|error| error.contains("export 'docs'")));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_doctor_reports_declared_not_installed() {
        let dir = std::env::temp_dir().join("ktesio_test_doctor_manifest_only");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("skills.json"),
            r#"{"skills": {"docs": {"repo": "url"}}, "exports": {}}"#,
        )
        .unwrap();

        let result = check_project(&dir);

        assert!(result
            .warnings
            .iter()
            .any(|warning| warning.contains("declared but not installed")));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_doctor_reports_installed_not_locked() {
        let dir = std::env::temp_dir().join("ktesio_test_doctor_not_locked");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".agents/skills/docs")).unwrap();
        std::fs::write(
            dir.join("skills.json"),
            r#"{"skills": {"docs": {"repo": "url"}}, "exports": {}}"#,
        )
        .unwrap();

        let result = check_project(&dir);

        assert!(result
            .warnings
            .iter()
            .any(|warning| warning.contains("installed but not locked")));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_doctor_reports_orphaned_lockfile_entry() {
        let dir = std::env::temp_dir().join("ktesio_test_doctor_orphan");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), r#"{"skills": {}, "exports": {}}"#).unwrap();
        std::fs::write(
            dir.join("skills.lock"),
            r#"{"docs": {"commit": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "repo": "url"}}"#,
        )
        .unwrap();

        let result = check_project(&dir);

        assert!(result
            .warnings
            .iter()
            .any(|warning| warning.contains("orphaned")));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_doctor_reports_untracked_directory() {
        let dir = std::env::temp_dir().join("ktesio_test_doctor_untracked");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".agents/skills/local")).unwrap();
        std::fs::write(dir.join("skills.json"), r#"{"skills": {}, "exports": {}}"#).unwrap();

        let result = check_project(&dir);

        assert!(result
            .warnings
            .iter()
            .any(|warning| warning.contains("untracked")));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_doctor_run_in_returns_error_when_unhealthy() {
        let dir = std::env::temp_dir().join("ktesio_test_doctor_run_error");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("skills.json"), "not json").unwrap();

        let result = run_in(&dir);

        assert!(result.is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
