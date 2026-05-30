use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub const AWESOME_COPILOT_URL: &str = "https://github.com/iMagdy/awesome-copilot.git";
pub const AWESOME_COPILOT_SHA: &str = "118974fb72ec31524b002795c116fd66bde14bef";

pub const AGENT_SKILLS_URL: &str = "https://github.com/iMagdy/agent-skills";
pub const AGENT_SKILLS_SHA: &str = "180115660cfb8a86b808f117475a01f54caf3bc5";

pub struct TestContext {
    pub temp_dir: TempDir,
    pub project_dir: PathBuf,
}

impl TestContext {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let project_dir = temp_dir.path().to_path_buf();
        Self {
            temp_dir,
            project_dir,
        }
    }

    pub fn skills_dir(&self) -> PathBuf {
        self.project_dir.join(".agents").join("skills")
    }

    pub fn lockfile(&self) -> PathBuf {
        self.project_dir.join("skills.lock")
    }

    pub fn manifest(&self) -> PathBuf {
        self.project_dir.join("skills.json")
    }

    pub fn ensure_skills_dir(&self) {
        std::fs::create_dir_all(self.skills_dir()).expect("Failed to create skills directory");
    }
}

pub fn clone_repo(url: &str, sha: &str, dest: &Path) -> Result<(), String> {
    let output = std::process::Command::new("git")
        .args(["clone", "--depth", "1", url, dest.to_str().unwrap()])
        .output()
        .map_err(|e| format!("Failed to execute git clone: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git clone failed: {}", stderr));
    }

    let checkout_output = std::process::Command::new("git")
        .args(["-C", dest.to_str().unwrap(), "checkout", sha])
        .output()
        .map_err(|e| format!("Failed to execute git checkout: {}", e))?;

    if !checkout_output.status.success() {
        let stderr = String::from_utf8_lossy(&checkout_output.stderr);
        return Err(format!("git checkout failed: {}", stderr));
    }

    Ok(())
}

pub fn run_skm_command(args: &[&str], working_dir: &Path) -> Result<String, String> {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_skm"))
        .args(args)
        .current_dir(working_dir)
        .output()
        .map_err(|e| format!("Failed to execute skm: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        return Err(format!("skm failed: {}\n{}", stdout, stderr));
    }

    Ok(stdout)
}
