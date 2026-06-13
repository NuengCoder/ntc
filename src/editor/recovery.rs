use std::fs;
use std::path::{Path, PathBuf};

use chrono::Local;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::Editor;

/// Recovery file contents saved to disk for crash recovery.
#[derive(Serialize, Deserialize)]
pub(crate) struct RecoveryData {
    pub original_path: String,
    pub lines: Vec<String>,
    pub cursor_y: usize,
    pub cursor_byte: usize,
    pub scroll: usize,
    pub scroll_x: usize,
    pub timestamp: String,
}

/// Directory where crash recovery files are stored.
fn recovery_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("ntc").join("crash_recovery"))
}

/// Compute a safe filename for a given path (SHA-256 hash).
fn hash_path(path: &Path) -> String {
    let mut hasher = Sha256::new();
    hasher.update(path.to_string_lossy().as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Path to the recovery file for a given original file path.
fn recovery_file_for(path: &Path) -> Option<PathBuf> {
    recovery_dir().map(|dir| dir.join(hash_path(path)).with_extension("recovery"))
}

impl RecoveryData {
    /// Check if a crash recovery file exists for the given path.
    pub fn check_recovery(path: &Path) -> Option<RecoveryData> {
        let rpath = recovery_file_for(path)?;
        if !rpath.exists() {
            return None;
        }
        let content = fs::read_to_string(&rpath).ok()?;
        let data: RecoveryData = serde_json::from_str(&content).ok()?;
        Some(data)
    }

    /// Remove a recovery file by original path (after successful recovery or discard).
    pub fn remove_recovery(path: &Path) {
        if let Some(rpath) = recovery_file_for(path) {
            let _ = fs::remove_file(&rpath);
        }
    }
}

impl Editor {
    /// Save a crash recovery snapshot to disk.
    /// Called periodically during editing so that if the editor panics,
    /// unsaved work can be recovered on the next launch.
    pub(crate) fn save_recovery_snapshot(&self) {
        let Some(rpath) = recovery_file_for(&self.path) else {
            return;
        };
        if let Some(parent) = rpath.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let data = RecoveryData {
            original_path: self.path.to_string_lossy().to_string(),
            lines: self.lines.clone(),
            cursor_y: self.cursor_y,
            cursor_byte: self.cursor_byte,
            scroll: self.scroll,
            scroll_x: self.scroll_x,
            timestamp: Local::now().to_rfc3339(),
        };
        if let Ok(json) = serde_json::to_string(&data) {
            let _ = fs::write(&rpath, json);
        }
    }

    /// Delete the crash recovery file for the currently open file (on clean exit).
    pub(crate) fn clear_recovery_snapshot(&self) {
        if let Some(rpath) = recovery_file_for(&self.path) {
            let _ = fs::remove_file(&rpath);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_recovery_data_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        let mut file = fs::File::create(&path).unwrap();
        writeln!(file, "Hello, World!").unwrap();

        // Save recovery data
        let rpath = recovery_file_for(&path).unwrap();
        if let Some(parent) = rpath.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let data = RecoveryData {
            original_path: path.to_string_lossy().to_string(),
            lines: vec!["Hello, World!".to_string()],
            cursor_y: 0,
            cursor_byte: 5,
            scroll: 0,
            scroll_x: 0,
            timestamp: "2026-01-01T00:00:00+00:00".to_string(),
        };
        let json = serde_json::to_string(&data).unwrap();
        fs::write(&rpath, &json).unwrap();

        // Read it back
        let loaded = RecoveryData::check_recovery(&path).unwrap();
        assert_eq!(loaded.original_path, path.to_string_lossy());
        assert_eq!(loaded.lines, vec!["Hello, World!".to_string()]);
        assert_eq!(loaded.cursor_byte, 5);

        // Clean up
        RecoveryData::remove_recovery(&path);
        assert!(!rpath.exists());
    }

    #[test]
    fn test_no_recovery_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent.txt");
        assert!(RecoveryData::check_recovery(&path).is_none());
    }

    #[test]
    fn test_recovery_file_hash_is_deterministic() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        let rpath1 = recovery_file_for(&path);
        let rpath2 = recovery_file_for(&path);
        assert_eq!(rpath1, rpath2);
    }

    #[test]
    fn test_different_paths_different_hashes() {
        let dir = tempdir().unwrap();
        let path_a = dir.path().join("a.txt");
        let path_b = dir.path().join("b.txt");
        let rpath_a = recovery_file_for(&path_a);
        let rpath_b = recovery_file_for(&path_b);
        assert_ne!(rpath_a, rpath_b);
    }
}
