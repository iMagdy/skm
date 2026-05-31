mod helpers;

use std::path::{Path, PathBuf};
use std::process::Command;

use helpers::{run_kt_command, run_kt_command_output, TestContext};

#[test]
fn test_install_all_from_repo_installs_each_export() {
    let ctx = TestContext::new();
    let repo = create_multi_export_repo(&ctx.project_dir, "source");

    let result = run_kt_command(
        &["install", "--all", repo.to_str().unwrap()],
        &ctx.project_dir,
    );

    assert!(
        result.is_ok(),
        "kt install --all failed: {:?}",
        result.err()
    );
    assert!(ctx.skills_dir().join("alpha/SKILL.md").exists());
    assert!(ctx.skills_dir().join("beta/SKILL.md").exists());

    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ctx.manifest()).unwrap()).unwrap();
    assert!(manifest["skills"].get("alpha").is_some());
    assert!(manifest["skills"].get("beta").is_some());

    let lockfile = std::fs::read_to_string(ctx.lockfile()).unwrap();
    assert!(lockfile.contains("alpha"));
    assert!(lockfile.contains("beta"));
}

#[test]
fn test_install_repo_no_input_refuses_ambiguous_exports() {
    let ctx = TestContext::new();
    let repo = create_multi_export_repo(&ctx.project_dir, "source");

    let result = run_kt_command(
        &["install", "--no-input", repo.to_str().unwrap()],
        &ctx.project_dir,
    );

    assert!(result.is_err(), "ambiguous install should fail");
    assert!(!ctx.manifest().exists());
    assert!(!ctx.lockfile().exists());
}

#[test]
fn test_install_all_from_repo_without_manifest_installs_fallback_files() {
    let ctx = TestContext::new();
    let repo = create_no_manifest_file_skill_repo(&ctx.project_dir, "fallback");

    let result = run_kt_command(
        &["install", "--all", repo.to_str().unwrap()],
        &ctx.project_dir,
    );

    assert!(
        result.is_ok(),
        "fallback file install failed: {:?}",
        result.err()
    );
    assert!(ctx.skills_dir().join("file-skill/file-skill.md").exists());
}

#[test]
fn test_list_and_show_json_output() {
    let ctx = TestContext::new();
    let repo = create_multi_export_repo(&ctx.project_dir, "source");
    run_kt_command(
        &["install", "--all", repo.to_str().unwrap()],
        &ctx.project_dir,
    )
    .expect("install should succeed");

    let list_output =
        run_kt_command_output(&["list", "--json"], &ctx.project_dir).expect("list should work");
    let list: serde_json::Value = serde_json::from_str(&list_output.stdout).unwrap();
    assert_eq!(list.as_array().unwrap().len(), 2);
    assert_eq!(list[0]["status"], "installed");

    let show_output = run_kt_command_output(&["show", "alpha", "--json"], &ctx.project_dir)
        .expect("show should work");
    let show: serde_json::Value = serde_json::from_str(&show_output.stdout).unwrap();
    assert_eq!(show["name"], "alpha");
    assert_eq!(show["status"], "installed");
}

#[test]
fn test_export_add_preserves_imports_and_adds_export() {
    let ctx = TestContext::new();
    std::fs::create_dir_all(ctx.project_dir.join("skills/local")).unwrap();
    std::fs::write(ctx.project_dir.join("skills/local/SKILL.md"), "# Local").unwrap();
    std::fs::write(
        ctx.manifest(),
        r#"{"skills": {"docs": {"repo": "url"}}, "exports": {}}"#,
    )
    .unwrap();

    let result = run_kt_command(
        &["export", "add", "local", "skills/local"],
        &ctx.project_dir,
    );

    assert!(result.is_ok(), "export add failed: {:?}", result.err());
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ctx.manifest()).unwrap()).unwrap();
    assert_eq!(manifest["skills"]["docs"]["repo"], "url");
    assert_eq!(manifest["exports"]["local"]["path"], "skills/local");
}

#[test]
fn test_doctor_reports_missing_locked_skill() {
    let ctx = TestContext::new();
    std::fs::write(
        ctx.manifest(),
        r#"{"skills": {"docs": {"repo": "url"}}, "exports": {}}"#,
    )
    .unwrap();
    std::fs::write(
        ctx.lockfile(),
        r#"{"docs": {"commit": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "repo": "url"}}"#,
    )
    .unwrap();

    let result = run_kt_command(&["doctor"], &ctx.project_dir);

    assert!(
        result.is_err(),
        "doctor should fail for missing locked skill"
    );
}

fn create_multi_export_repo(root: &Path, name: &str) -> PathBuf {
    let repo = root.join(format!("{name}-repo"));
    std::fs::create_dir_all(repo.join("skills/alpha")).unwrap();
    std::fs::create_dir_all(repo.join("skills/beta")).unwrap();
    std::fs::write(repo.join("skills/alpha/SKILL.md"), "# Alpha").unwrap();
    std::fs::write(repo.join("skills/beta/SKILL.md"), "# Beta").unwrap();
    std::fs::write(
        repo.join("skills.json"),
        r#"{
  "skills": {},
  "exports": {
    "alpha": { "path": "skills/alpha" },
    "beta": { "path": "skills/beta" }
  }
}"#,
    )
    .unwrap();
    run_git(&repo, &["init"]);
    run_git(&repo, &["add", "."]);
    run_git(
        &repo,
        &[
            "-c",
            "user.name=ktesio tests",
            "-c",
            "user.email=ktesio-tests@example.com",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-m",
            "initial fixture",
        ],
    );
    repo
}

fn create_no_manifest_file_skill_repo(root: &Path, name: &str) -> PathBuf {
    let repo = root.join(format!("{name}-repo"));
    std::fs::create_dir_all(repo.join("skills")).unwrap();
    std::fs::write(repo.join("skills/file-skill.md"), "# File Skill").unwrap();
    run_git(&repo, &["init"]);
    run_git(&repo, &["add", "."]);
    run_git(
        &repo,
        &[
            "-c",
            "user.name=ktesio tests",
            "-c",
            "user.email=ktesio-tests@example.com",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-m",
            "initial fixture",
        ],
    );
    repo
}

fn run_git(repo: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}
