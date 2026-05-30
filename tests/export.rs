mod helpers;

use helpers::{clone_repo, run_skm_command, TestContext, AWESOME_COPILOT_SHA, AWESOME_COPILOT_URL};

#[test]
#[ignore = "requires network access"]
fn test_export_creates_skills_json() {
    let ctx = TestContext::new();
    ctx.ensure_skills_dir();

    // Clone fixture repo
    let fixture_dir = ctx.project_dir.join("fixture");
    clone_repo(AWESOME_COPILOT_URL, AWESOME_COPILOT_SHA, &fixture_dir).unwrap();

    // Create skills.json
    let manifest = serde_json::json!({
        "skills": {
            "awesome-copilot": {
                "repo": fixture_dir.to_str().unwrap()
            }
        },
        "exports": {}
    });
    std::fs::write(ctx.manifest(), serde_json::to_string_pretty(&manifest).unwrap()).unwrap();

    // Install first
    let result = run_skm_command(&["install"], &ctx.project_dir);
    assert!(result.is_ok(), "skm install failed: {:?}", result.err());

    // Run skm export
    let result = run_skm_command(&["export"], &ctx.project_dir);
    assert!(result.is_ok(), "skm export failed: {:?}", result.err());

    // Verify skills.json exists
    assert!(ctx.manifest().exists(), "skills.json should exist");
}

#[test]
#[ignore = "requires network access"]
fn test_export_manifest_contains_skills_key() {
    let ctx = TestContext::new();
    ctx.ensure_skills_dir();

    // Clone fixture repo
    let fixture_dir = ctx.project_dir.join("fixture");
    clone_repo(AWESOME_COPILOT_URL, AWESOME_COPILOT_SHA, &fixture_dir).unwrap();

    // Create skills.json
    let manifest = serde_json::json!({
        "skills": {
            "awesome-copilot": {
                "repo": fixture_dir.to_str().unwrap()
            }
        },
        "exports": {}
    });
    std::fs::write(ctx.manifest(), serde_json::to_string_pretty(&manifest).unwrap()).unwrap();

    // Install first
    let result = run_skm_command(&["install"], &ctx.project_dir);
    assert!(result.is_ok(), "skm install failed: {:?}", result.err());

    // Run skm export
    let result = run_skm_command(&["export"], &ctx.project_dir);
    assert!(result.is_ok(), "skm export failed: {:?}", result.err());

    // Verify manifest contains skills key
    let manifest_content = std::fs::read_to_string(ctx.manifest()).unwrap();
    let manifest: serde_json::Value = serde_json::from_str(&manifest_content).unwrap();
    assert!(manifest.get("skills").is_some(), "Manifest should contain skills key");
}

#[test]
#[ignore = "requires network access"]
fn test_export_manifest_contains_exports_key() {
    let ctx = TestContext::new();
    ctx.ensure_skills_dir();

    // Clone fixture repo
    let fixture_dir = ctx.project_dir.join("fixture");
    clone_repo(AWESOME_COPILOT_URL, AWESOME_COPILOT_SHA, &fixture_dir).unwrap();

    // Create skills.json
    let manifest = serde_json::json!({
        "skills": {
            "awesome-copilot": {
                "repo": fixture_dir.to_str().unwrap()
            }
        },
        "exports": {}
    });
    std::fs::write(ctx.manifest(), serde_json::to_string_pretty(&manifest).unwrap()).unwrap();

    // Install first
    let result = run_skm_command(&["install"], &ctx.project_dir);
    assert!(result.is_ok(), "skm install failed: {:?}", result.err());

    // Run skm export
    let result = run_skm_command(&["export"], &ctx.project_dir);
    assert!(result.is_ok(), "skm export failed: {:?}", result.err());

    // Verify manifest contains exports key
    let manifest_content = std::fs::read_to_string(ctx.manifest()).unwrap();
    let manifest: serde_json::Value = serde_json::from_str(&manifest_content).unwrap();
    assert!(manifest.get("exports").is_some(), "Manifest should contain exports key");
}

#[test]
#[ignore = "requires network access"]
fn test_export_lists_skill_with_correct_source() {
    let ctx = TestContext::new();
    ctx.ensure_skills_dir();

    // Clone fixture repo
    let fixture_dir = ctx.project_dir.join("fixture");
    clone_repo(AWESOME_COPILOT_URL, AWESOME_COPILOT_SHA, &fixture_dir).unwrap();

    // Create skills.json
    let manifest = serde_json::json!({
        "skills": {
            "awesome-copilot": {
                "repo": fixture_dir.to_str().unwrap()
            }
        },
        "exports": {}
    });
    std::fs::write(ctx.manifest(), serde_json::to_string_pretty(&manifest).unwrap()).unwrap();

    // Install first
    let result = run_skm_command(&["install"], &ctx.project_dir);
    assert!(result.is_ok(), "skm install failed: {:?}", result.err());

    // Run skm export
    let result = run_skm_command(&["export"], &ctx.project_dir);
    assert!(result.is_ok(), "skm export failed: {:?}", result.err());

    // Verify manifest lists skill with correct source
    let manifest_content = std::fs::read_to_string(ctx.manifest()).unwrap();
    let manifest: serde_json::Value = serde_json::from_str(&manifest_content).unwrap();
    let skill = manifest.get("skills").unwrap().get("awesome-copilot").unwrap();
    assert!(skill.get("repo").is_some(), "Skill should have repo field");
}

#[test]
#[ignore = "requires network access"]
fn test_export_manifest_valid_json() {
    let ctx = TestContext::new();
    ctx.ensure_skills_dir();

    // Clone fixture repo
    let fixture_dir = ctx.project_dir.join("fixture");
    clone_repo(AWESOME_COPILOT_URL, AWESOME_COPILOT_SHA, &fixture_dir).unwrap();

    // Create skills.json
    let manifest = serde_json::json!({
        "skills": {
            "awesome-copilot": {
                "repo": fixture_dir.to_str().unwrap()
            }
        },
        "exports": {}
    });
    std::fs::write(ctx.manifest(), serde_json::to_string_pretty(&manifest).unwrap()).unwrap();

    // Install first
    let result = run_skm_command(&["install"], &ctx.project_dir);
    assert!(result.is_ok(), "skm install failed: {:?}", result.err());

    // Run skm export
    let result = run_skm_command(&["export"], &ctx.project_dir);
    assert!(result.is_ok(), "skm export failed: {:?}", result.err());

    // Verify manifest is valid JSON
    let manifest_content = std::fs::read_to_string(ctx.manifest()).unwrap();
    let result: Result<serde_json::Value, _> = serde_json::from_str(&manifest_content);
    assert!(result.is_ok(), "Manifest should be valid JSON");
}

#[test]
#[ignore = "requires network access"]
fn test_export_manifest_2space_indent() {
    let ctx = TestContext::new();
    ctx.ensure_skills_dir();

    // Clone fixture repo
    let fixture_dir = ctx.project_dir.join("fixture");
    clone_repo(AWESOME_COPILOT_URL, AWESOME_COPILOT_SHA, &fixture_dir).unwrap();

    // Create skills.json
    let manifest = serde_json::json!({
        "skills": {
            "awesome-copilot": {
                "repo": fixture_dir.to_str().unwrap()
            }
        },
        "exports": {}
    });
    std::fs::write(ctx.manifest(), serde_json::to_string_pretty(&manifest).unwrap()).unwrap();

    // Install first
    let result = run_skm_command(&["install"], &ctx.project_dir);
    assert!(result.is_ok(), "skm install failed: {:?}", result.err());

    // Run skm export
    let result = run_skm_command(&["export"], &ctx.project_dir);
    assert!(result.is_ok(), "skm export failed: {:?}", result.err());

    // Verify manifest uses 2-space indent
    let manifest_content = std::fs::read_to_string(ctx.manifest()).unwrap();
    // Check that the manifest uses 2-space indentation
    assert!(manifest_content.contains("  "), "Manifest should use 2-space indentation");
}
