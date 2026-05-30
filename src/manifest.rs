use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{ManifestDuplicateName, ManifestInvalidName, ManifestNotFound};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillEntry {
    pub repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportEntry {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Manifest {
    pub skills: HashMap<String, SkillEntry>,
    pub exports: HashMap<String, ExportEntry>,
}

impl Manifest {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
            exports: HashMap::new(),
        }
    }

    pub fn load(path: &Path) -> Result<Self, ManifestNotFound> {
        let content = fs::read_to_string(path).map_err(|_| ManifestNotFound {
            message: format!(
                "No skills.json found at {}. Run 'skm init .' to create one.",
                path.display()
            ),
        })?;

        Self::parse_str(&content, path)
    }

    pub fn parse_str(content: &str, path: &Path) -> Result<Self, ManifestNotFound> {
        let manifest: Manifest =
            serde_json::from_str(content).map_err(|e| ManifestNotFound {
                message: format!("Invalid skills.json at {}: {}", path.display(), e),
            })?;

        manifest.validate().map_err(|e| ManifestNotFound {
            message: e.to_string(),
        })?;

        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let name_re = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();

        for name in self.skills.keys() {
            if !name_re.is_match(name) {
                return Err(ManifestInvalidName {
                    message: format!(
                        "Invalid skill name '{}': must match [a-zA-Z0-9_-]+",
                        name
                    ),
                }
                .into());
            }
        }

        let mut seen = std::collections::HashSet::new();
        for name in self.skills.keys() {
            if !seen.insert(name.clone()) {
                return Err(ManifestDuplicateName {
                    message: format!("Duplicate skill name: '{}'", name),
                }
                .into());
            }
        }

        Ok(())
    }

    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)
    }

    pub fn add_skill(&mut self, name: String, repo: String) {
        self.skills.insert(name, SkillEntry { repo });
    }

    pub fn remove_skill(&mut self, name: &str) -> bool {
        self.skills.remove(name).is_some()
    }

    pub fn has_skill(&self, name: &str) -> bool {
        self.skills.contains_key(name)
    }
}

impl Default for Manifest {
    fn default() -> Self {
        Self::new()
    }
}
