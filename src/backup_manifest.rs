// src/backup_manifest.rs
// Production-grade manifest handling for ntc backup system
// Version: v1.8.2
// Cross-platform: Windows, Linux, macOS

use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::collections::HashMap;

/// Maximum file size to include in backup (50 MB)
pub const MAX_BACKUP_FILE_SIZE: u64 = 50 * 1024 * 1024;

/// Manifest format version — bump when the schema changes
const MANIFEST_VERSION: &str = "1.1";

/// Separator used in summary keys: "{project_hash}#{backup_number}"
/// Must be a character that cannot appear in a hex hash or a decimal number.
const SUMMARY_KEY_SEP: char = '#';

// ============================================================================
// Core types
// ============================================================================

/// Represents a single file entry in a backup manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupFileEntry {
    /// Relative path from project root (e.g., "src/main.c")
    /// Uses forward slashes on all platforms for portability.
    pub rel_path: PathBuf,
    /// SHA-256 hex digest of file content (for diff/verification)
    pub hash: String,
    /// File size in bytes
    pub size: u64,
    /// Last modification time (Unix timestamp seconds, 0 if unavailable)
    pub modified: u64,
}

/// Represents a file that was skipped during backup (with reason)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkippedFileEntry {
    /// Relative path from project root
    pub rel_path: PathBuf,
    /// Reason: "too_large" | "ignored_by_config" | "ignored_by_user"
    pub reason: SkipReason,
    /// Additional info (e.g. "65 MB" for too_large)
    pub detail: Option<String>,
}

/// Typed skip reason (avoids raw string typos)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SkipReason {
    TooLarge,
    IgnoredByConfig,
    IgnoredByUser,
}

impl SkipReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            SkipReason::TooLarge       => "TOO_LARGE",
            SkipReason::IgnoredByConfig => "IGNORED_BY_CONFIG",
            SkipReason::IgnoredByUser   => "IGNORED_BY_USER",
        }
    }
}

/// Complete backup manifest for a single backup snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifest {
    /// Manifest format version (e.g. "1.1") — bump on schema changes
    pub version: String,
    /// Timestamp when backup was created (Unix seconds)
    pub created_at: u64,
    /// Absolute path to the original project root (canonicalized)
    pub project_root: PathBuf,
    /// Unique hash of the project root path (16 hex chars of SHA-256)
    /// 64 bits of entropy — negligible collision risk for personal use (<100 projects)
    pub project_hash: String,
    /// All files included in this backup
    pub files: Vec<BackupFileEntry>,
    /// Files that were skipped (reasons recorded in ignore.txt)
    pub skipped_files: Vec<SkippedFileEntry>,
    /// Total size of all backed-up files in bytes
    pub total_size: u64,
    /// Backup number (1, 2, 3, ...)
    pub backup_number: usize,
    /// Human-readable size string (e.g. "4.2 MB") — cached for quick display
    pub total_size_human: String,
    /// Paths of files that are NEW in this backup (didn't exist in project before).
    /// Tracked so that undo can delete them rather than just restoring overwritten ones.
    pub new_files: Vec<PathBuf>,
}

/// Global index tracking all backups across all projects.
/// Stored at ~/.ntc/backups/index.json
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupIndex {
    /// Maps project_hash -> sorted list of backup numbers
    pub projects: HashMap<String, Vec<usize>>,
    /// Lightweight summaries keyed by "{project_hash}#{backup_number}"
    /// Note: summaries are a cache of manifest data. If a manifest is ever
    /// amended externally, the summary here may be stale.
    pub summaries: HashMap<String, BackupSummary>,
}

/// Lightweight summary for listing backups without loading full manifests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSummary {
    pub backup_number: usize,
    pub created_at: u64,
    pub total_size_human: String,
    pub file_count: usize,
}

/// State saved before a restore (for undo functionality).
/// Stores actual file contents in a separate directory alongside metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoState {
    /// Timestamp when restore was performed (Unix seconds)
    pub restored_at: u64,
    /// Which backup number was restored (source)
    pub restored_from: usize,
    /// Files that existed before the restore and were overwritten
    /// (originals are saved in the undo/files/ directory)
    pub overwritten_files: Vec<PathBuf>,
    /// Files that were created brand-new by the restore
    /// (no original to save — these should be deleted on undo)
    pub new_files_created: Vec<PathBuf>,
    /// FIX: Files that existed in the project but NOT in the backup and were
    /// deleted by the restore to make it a true point-in-time snapshot.
    /// Originals are saved in the undo/deleted/ directory so they can be
    /// re-created on undo.
    #[serde(default)]
    pub deleted_files: Vec<PathBuf>,
}

// ============================================================================
// BackupManifest impl
// ============================================================================

impl BackupManifest {
    /// Create a new empty manifest
    pub fn new(project_root: PathBuf, project_hash: String, backup_number: usize) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            version: MANIFEST_VERSION.to_string(),
            created_at: now,
            project_root,
            project_hash,
            files: Vec::new(),
            skipped_files: Vec::new(),
            total_size: 0,
            backup_number,
            total_size_human: "0 B".to_string(),
            new_files: Vec::new(),
        }
    }

    /// Add a successfully backed-up file
    pub fn add_file(
        &mut self,
        rel_path: PathBuf,
        hash: String,
        size: u64,
        modified: u64,
        is_new: bool,
    ) {
        if is_new {
            self.new_files.push(rel_path.clone());
        }
        self.files.push(BackupFileEntry { rel_path, hash, size, modified });
        self.total_size += size;
    }

    /// Add a skipped file entry
    pub fn add_skipped_file(
        &mut self,
        rel_path: PathBuf,
        reason: SkipReason,
        detail: Option<String>,
    ) {
        self.skipped_files.push(SkippedFileEntry { rel_path, reason, detail });
    }

    /// Recalculate human-readable size — call after all files are added
    pub fn finalize(&mut self) {
        self.total_size_human = crate::explorer::human_readable_size(self.total_size);
    }

    /// Generate the content of ignore.txt (human-readable skipped files report)
    pub fn generate_ignore_txt(&self) -> String {
        if self.skipped_files.is_empty() {
            return "# No files were skipped during this backup.\n".to_string();
        }

        let mut out = String::new();
        out.push_str("# ntc Backup - Skipped Files Report\n");
        out.push_str(&format!(
            "# Backup #{} created at {}\n",
            self.backup_number, self.created_at
        ));
        out.push_str("# ================================================\n\n");

        for s in &self.skipped_files {
            let detail = s.detail.as_deref().map(|d| format!(" ({})", d)).unwrap_or_default();
            out.push_str(&format!(
                "[{}] {}{}\n",
                s.reason.as_str(),
                s.rel_path.display(),
                detail
            ));
        }

        out
    }

    /// Write ignore.txt alongside the manifest
    pub fn save_ignore_txt(&self, backup_dir: &Path) -> anyhow::Result<()> {
        std::fs::write(backup_dir.join("ignore.txt"), self.generate_ignore_txt())?;
        Ok(())
    }

    /// Load manifest from a JSON file
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    /// Save manifest to a JSON file (creates parent directories as needed)
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}

// ============================================================================
// BackupIndex impl
// ============================================================================

impl BackupIndex {
    /// ntc home directory (~/.ntc on Unix, %USERPROFILE%\.ntc on Windows)
    pub fn get_ntc_home() -> PathBuf {
        dirs::home_dir()
            .expect("Could not determine home directory")
            .join(".ntc")
    }

    /// Backup directory for a specific project
    pub fn get_project_backup_dir(project_hash: &str) -> PathBuf {
        Self::get_ntc_home().join("backups").join(project_hash)
    }

    /// Path to a specific backup snapshot directory
    pub fn get_backup_path(project_hash: &str, backup_number: usize) -> PathBuf {
        Self::get_project_backup_dir(project_hash)
            .join(format!("bkup_{}", backup_number))
    }

    /// Undo root directory for a project
    pub fn get_undo_dir(project_hash: &str) -> PathBuf {
        Self::get_ntc_home().join("unpd").join(project_hash)
    }

    /// Undo state metadata file
    pub fn get_undo_state_path(project_hash: &str) -> PathBuf {
        Self::get_undo_dir(project_hash).join("state.json")
    }

    /// Undo files directory (actual pre-restore file contents for overwritten files)
    pub fn get_undo_files_dir(project_hash: &str) -> PathBuf {
        Self::get_undo_dir(project_hash).join("files")
    }

    /// Build the summary HashMap key for a given project hash + backup number.
    /// Uses '#' as separator — cannot appear in a hex hash or decimal number,
    /// so there is no ambiguity between e.g. hash "abc1" + num 2  vs  hash "abc" + num 12.
    pub fn summary_key(project_hash: &str, backup_number: usize) -> String {
        format!("{}{}{}", project_hash, SUMMARY_KEY_SEP, backup_number)
    }
}

// ============================================================================
// Display helper
// ============================================================================

/// Strip Windows extended-path prefix (\\?\) for clean terminal output.
/// Single source of truth — imported by backup.rs and search.rs.
pub fn display_path(path: &Path) -> String {
    let s = path.to_string_lossy();
    #[cfg(windows)]
    if s.starts_with(r"\\?\") {
        return s[4..].to_string();
    }
    s.to_string()
}