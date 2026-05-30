use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{LockfileInvalid, LockfileNotFound};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LockEntry {
    pub commit: String,
    pub repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Lockfile {
    #[serde(flatten)]
    pub entries: HashMap<String, LockEntry>,
}

impl Lockfile {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(path).map_err(|_| LockfileNotFound {
            message: format!("Cannot read skills.lock at {}", path.display()),
        })?;

        Self::parse(&content)
    }

    pub fn parse(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if content.trim().is_empty() {
            return Ok(Self::new());
        }

        let entries: HashMap<String, LockEntry> =
            serde_json::from_str(content).map_err(|e| LockfileInvalid {
                message: format!("Invalid skills.lock: {}", e),
            })?;

        for (name, entry) in &entries {
            if entry.commit.len() != 40 || !entry.commit.chars().all(|c| c.is_ascii_hexdigit())
            {
                return Err(LockfileInvalid {
                    message: format!(
                        "Invalid commit hash for skill '{}': must be 40 hex characters",
                        name
                    ),
                }
                .into());
            }
        }

        Ok(Self { entries })
    }

    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        let content = serde_json::to_string_pretty(&self.entries)?;
        fs::write(path, content)
    }

    pub fn entry(&self, name: &str) -> Option<&LockEntry> {
        self.entries.get(name)
    }

    pub fn insert(&mut self, name: String, entry: LockEntry) {
        self.entries.insert(name, entry);
    }

    pub fn remove(&mut self, name: &str) -> bool {
        self.entries.remove(name).is_some()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.entries.contains_key(name)
    }

    pub fn entries(&self) -> &HashMap<String, LockEntry> {
        &self.entries
    }
}
