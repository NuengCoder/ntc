//! `registry.toml` models plus atomic load/save for the skill library.
//!
//! The registry is the only source of truth for what ntc downloaded and where
//! each skill was copied. Unknown fields are ignored on read so a future
//! version (or a hand edit) never bricks the command.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Registry {
    #[serde(default)]
    pub bundles: BTreeMap<String, BundleRecord>,
    #[serde(default)]
    pub skills: BTreeMap<String, SkillRecord>,
}

/// The batch a `download` fetched (`superpowers`, `frontenddev`, ...).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleRecord {
    pub source: String,
    #[serde(rename = "ref")]
    pub git_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    pub installed_at: String,
    #[serde(default)]
    pub skills: Vec<String>,
}

/// One skill folder in the library.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRecord {
    pub bundle: String,
    pub source: String,
    /// Path inside the downloaded repo, e.g. `skills/brainstorming`.
    pub path: String,
    #[serde(default)]
    pub description: String,
    /// Hash of `SKILL.md` at install time, used to detect hand edits.
    pub sha256: String,
    pub installed_at: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<AddedRecord>,
}

/// A copy of the skill in an AI harness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddedRecord {
    pub harness: String,
    pub scope: String,
    pub path: String,
    pub added_at: String,
}

impl Registry {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.is_file() {
            return Ok(Self::default());
        }
        let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
        toml::from_str(&text).with_context(|| format!("parse {}", path.display()))
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let data = toml::to_string_pretty(self).context("serialize skill registry")?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
        }
        let tmp = path.with_extension("toml.tmp");
        fs::write(&tmp, data).with_context(|| format!("write {}", tmp.display()))?;
        fs::rename(&tmp, path)
            .or_else(|_| {
                fs::copy(&tmp, path)?;
                fs::remove_file(&tmp)?;
                Ok::<(), std::io::Error>(())
            })
            .with_context(|| format!("replace {}", path.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record() -> SkillRecord {
        SkillRecord {
            bundle: "superpowers".into(),
            source: "obra/superpowers".into(),
            path: "skills/brainstorming".into(),
            description: "Think before writing code".into(),
            sha256: "abc".into(),
            installed_at: "2026-11-03T00:00:00Z".into(),
            added: vec![AddedRecord {
                harness: "opencode".into(),
                scope: "project".into(),
                path: "C:/work/.opencode/skills/brainstorming".into(),
                added_at: "2026-11-03T00:01:00Z".into(),
            }],
        }
    }

    #[test]
    fn roundtrip_keeps_bundles_skills_and_added() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("registry.toml");
        let mut reg = Registry::default();
        reg.bundles.insert(
            "superpowers".into(),
            BundleRecord {
                source: "obra/superpowers".into(),
                git_ref: "main".into(),
                commit: Some("abc123".into()),
                installed_at: "2026-11-03T00:00:00Z".into(),
                skills: vec!["brainstorming".into()],
            },
        );
        reg.skills.insert("brainstorming".into(), record());
        reg.save(&path).unwrap();

        let loaded = Registry::load(&path).unwrap();
        assert_eq!(loaded.bundles["superpowers"].skills, ["brainstorming"]);
        assert_eq!(loaded.skills["brainstorming"].sha256, "abc");
        assert_eq!(loaded.skills["brainstorming"].added.len(), 1);
        assert_eq!(loaded.skills["brainstorming"].added[0].harness, "opencode");
    }

    #[test]
    fn missing_registry_is_empty_not_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let reg = Registry::load(&dir.path().join("registry.toml")).unwrap();
        assert!(reg.skills.is_empty());
        assert!(reg.bundles.is_empty());
    }

    #[test]
    fn unknown_fields_are_ignored() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("registry.toml");
        fs::write(
            &path,
            "future = 1\n[skills.demo]\nbundle = 'demo'\nsource = 'owner/repo'\npath = 'skills/demo'\ndescription = 'Demo'\nsha256 = 'abc'\ninstalled_at = 'now'\nfuture = true\n",
        )
        .unwrap();
        let reg = Registry::load(&path).unwrap();
        assert_eq!(reg.skills["demo"].description, "Demo");
    }

    #[test]
    fn corrupt_registry_names_the_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("registry.toml");
        fs::write(&path, "not = [valid").unwrap();
        let err = Registry::load(&path).unwrap_err().to_string();
        assert!(err.contains("registry.toml"), "unexpected error: {err}");
    }
}
