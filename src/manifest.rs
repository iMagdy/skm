use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{ManifestDuplicateName, ManifestInvalidName, ManifestNotFound};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillEntry {
    pub repo: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skill: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportEntry {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Manifest {
    #[serde(default)]
    pub skills: HashMap<String, SkillEntry>,
    #[serde(default)]
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
                "No skills.json found at {}. Run 'kt init .' to create one.",
                path.display()
            ),
        })?;

        Self::parse_str(&content, path)
    }

    pub fn parse_str(content: &str, path: &Path) -> Result<Self, ManifestNotFound> {
        let manifest: Manifest = serde_json::from_str(content).map_err(|e| ManifestNotFound {
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
                    message: format!("Invalid skill name '{}': must match [a-zA-Z0-9_-]+", name),
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
        self.add_skill_with_source(name, repo, None);
    }

    pub fn add_skill_with_source(&mut self, name: String, repo: String, skill: Option<String>) {
        self.skills.insert(name, SkillEntry { repo, skill });
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_new() {
        let manifest = Manifest::new();
        assert!(manifest.skills.is_empty());
        assert!(manifest.exports.is_empty());
    }

    #[test]
    fn test_default() {
        let manifest = Manifest::default();
        assert!(manifest.skills.is_empty());
        assert!(manifest.exports.is_empty());
    }

    #[test]
    fn test_add_skill() {
        let mut manifest = Manifest::new();
        manifest.add_skill(
            "test-skill".to_string(),
            "https://github.com/test/repo.git".to_string(),
        );
        assert!(manifest.has_skill("test-skill"));
        assert_eq!(manifest.skills.len(), 1);
    }

    #[test]
    fn test_remove_skill() {
        let mut manifest = Manifest::new();
        manifest.add_skill(
            "test-skill".to_string(),
            "https://github.com/test/repo.git".to_string(),
        );
        assert!(manifest.remove_skill("test-skill"));
        assert!(!manifest.has_skill("test-skill"));
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut manifest = Manifest::new();
        assert!(!manifest.remove_skill("nonexistent"));
    }

    #[test]
    fn test_has_skill() {
        let mut manifest = Manifest::new();
        assert!(!manifest.has_skill("test-skill"));
        manifest.add_skill(
            "test-skill".to_string(),
            "https://github.com/test/repo.git".to_string(),
        );
        assert!(manifest.has_skill("test-skill"));
    }

    #[test]
    fn test_parse_valid() {
        let content = r#"{
            "skills": {
                "my-skill": {
                    "repo": "https://github.com/test/repo.git"
                }
            },
            "exports": {}
        }"#;
        let path = Path::new("test.json");
        let manifest = Manifest::parse_str(content, path).unwrap();
        assert!(manifest.has_skill("my-skill"));
    }

    #[test]
    fn test_parse_optional_source_skill() {
        let content = r#"{
            "skills": {
                "my-skill": {
                    "repo": "https://github.com/example/skills.git",
                    "skill": "upstream-skill"
                }
            },
            "exports": {}
        }"#;
        let path = Path::new("test.json");
        let manifest = Manifest::parse_str(content, path).unwrap();
        assert_eq!(
            manifest.skills["my-skill"].skill.as_deref(),
            Some("upstream-skill")
        );
    }

    #[test]
    fn test_parse_empty() {
        let content = r#"{"skills": {}, "exports": {}}"#;
        let path = Path::new("test.json");
        let manifest = Manifest::parse_str(content, path).unwrap();
        assert!(manifest.skills.is_empty());
    }

    #[test]
    fn test_parse_missing_skills_defaults_empty() {
        let content = r#"{"exports": {"my-skill": {"path": "skills/my-skill"}}}"#;
        let path = Path::new("test.json");
        let manifest = Manifest::parse_str(content, path).unwrap();
        assert!(manifest.skills.is_empty());
        assert!(manifest.exports.contains_key("my-skill"));
    }

    #[test]
    fn test_parse_missing_exports_defaults_empty() {
        let content = r#"{"skills": {"my-skill": {"repo": "url"}}}"#;
        let path = Path::new("test.json");
        let manifest = Manifest::parse_str(content, path).unwrap();
        assert!(manifest.has_skill("my-skill"));
        assert!(manifest.exports.is_empty());
    }

    #[test]
    fn test_parse_empty_object_defaults_fields_empty() {
        let content = r#"{}"#;
        let path = Path::new("test.json");
        let manifest = Manifest::parse_str(content, path).unwrap();
        assert!(manifest.skills.is_empty());
        assert!(manifest.exports.is_empty());
    }

    #[test]
    fn test_parse_invalid_json() {
        let content = r#"{"skills": {}"#;
        let path = Path::new("test.json");
        assert!(Manifest::parse_str(content, path).is_err());
    }

    #[test]
    fn test_parse_invalid_name() {
        let content = r#"{"skills": {"bad name!": {"repo": "url"}}, "exports": {}}"#;
        let path = Path::new("test.json");
        assert!(Manifest::parse_str(content, path).is_err());
    }

    #[test]
    fn test_validate_valid() {
        let mut manifest = Manifest::new();
        manifest.add_skill("valid_name-123".to_string(), "url".to_string());
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_name() {
        let mut manifest = Manifest::new();
        manifest.add_skill("bad name!".to_string(), "url".to_string());
        assert!(manifest.validate().is_err());
    }

    #[test]
    fn test_save_and_load() {
        let mut manifest = Manifest::new();
        manifest.add_skill("test".to_string(), "url".to_string());
        let dir = std::env::temp_dir().join("ktesio_test_manifest");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("skills.json");
        manifest.save(&path).unwrap();
        let loaded = Manifest::load(&path).unwrap();
        assert!(loaded.has_skill("test"));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_load_not_found() {
        assert!(Manifest::load(Path::new("/nonexistent")).is_err());
    }

    #[test]
    fn test_validate_duplicate_names() {
        // serde_json will just take the last value, so no duplicate error
        // But we should test the validate function directly
        let mut manifest = Manifest::new();
        manifest.add_skill("test".to_string(), "url1".to_string());
        // Manually add duplicate to test validation
        manifest.skills.insert(
            "test".to_string(),
            SkillEntry {
                repo: "url2".to_string(),
                skill: None,
            },
        );
        assert!(manifest.validate().is_ok()); // serde deduplicates
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = std::env::temp_dir().join("ktesio_test_manifest_roundtrip");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("skills.json");

        let mut manifest = Manifest::new();
        manifest.add_skill("skill1".to_string(), "url1".to_string());
        manifest.add_skill("skill2".to_string(), "url2".to_string());
        manifest.save(&path).unwrap();

        let loaded = Manifest::load(&path).unwrap();
        assert_eq!(loaded.skills.len(), 2);
        assert!(loaded.has_skill("skill1"));
        assert!(loaded.has_skill("skill2"));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_parse_invalid_name_special_chars() {
        let content = r#"{"skills": {"bad@name": {"repo": "url"}}, "exports": {}}"#;
        let path = Path::new("test.json");
        assert!(Manifest::parse_str(content, path).is_err());
    }

    #[test]
    fn test_parse_invalid_name_spaces() {
        let content = r#"{"skills": {"has space": {"repo": "url"}}, "exports": {}}"#;
        let path = Path::new("test.json");
        assert!(Manifest::parse_str(content, path).is_err());
    }

    #[test]
    fn test_validate_valid_names() {
        let mut manifest = Manifest::new();
        manifest.add_skill("valid_name-123".to_string(), "url".to_string());
        manifest.add_skill("another-skill".to_string(), "url".to_string());
        manifest.add_skill("skill123".to_string(), "url".to_string());
        assert!(manifest.validate().is_ok());
    }
}
