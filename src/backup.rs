// src/backup.rs
// Production-grade backup system for ntc
// Version: v1.8.2
// Cross-platform: Windows, Linux, macOS

use crate::backup_manifest::{
    BackupManifest, BackupIndex, BackupSummary, UndoState,
    SkipReason, MAX_BACKUP_FILE_SIZE, display_path,
};
use crate::config::Config;
use crate::filetype::FormatConfig;
use crate::output::{print_error, print_info, print_success, print_warning};
use anyhow::Result;
use sha2::{Sha256, Digest};

use std::collections::HashSet;
use std::fs;
use std::io::{self, Write, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// ============================================================================
// Project hash
// ============================================================================

/// Compute a stable 16-hex-char hash for a project path (for backup folder naming).
/// Uses the full canonical path to ensure uniqueness across different locations.
/// 16 chars = 64 bits of entropy — negligible collision risk for personal use.
pub fn compute_project_hash(project_root: &Path) -> String {
    let canonical = project_root
        .canonicalize()
        .unwrap_or_else(|_| project_root.to_path_buf());
    let path_str = canonical.to_string_lossy();
    format!("{:x}", Sha256::digest(path_str.as_bytes()))
        .chars()
        .take(16)
        .collect()
}

// ============================================================================
// BackupManager
// ============================================================================

pub struct BackupManager;

impl BackupManager {

    // ========================================================================
    // BACKUP CREATION
    // ========================================================================

    /// Create a new backup of the current project.
    ///
    /// Writes to a temporary directory first, then renames atomically so that
    /// a killed process never leaves a partial `bkup_N` with no manifest.
    pub fn create_backup(project_root: &Path) -> Result<PathBuf> {
        print_info("Creating backup...");

        let project_hash   = compute_project_hash(project_root);
        let project_backup_dir = BackupIndex::get_project_backup_dir(&project_hash);
        let backup_number  = Self::next_backup_number(&project_backup_dir);
        let final_path     = BackupIndex::get_backup_path(&project_hash, backup_number);

        // Write to a temp directory first; rename at the end for atomicity.
        let tmp_path = project_backup_dir.join(format!(".tmp_bkup_{}", backup_number));
        if tmp_path.exists() {
            fs::remove_dir_all(&tmp_path)?;
        }
        fs::create_dir_all(&tmp_path)?;

        let mut manifest = BackupManifest::new(
            project_root.canonicalize()?,
            project_hash.clone(),
            backup_number,
        );

        let ignored_dirs = Config::global_get_ignored_dirs();
        let fmt_cfg      = FormatConfig::from_global();

        let walker = WalkDir::new(project_root)
            .into_iter()
            .filter_entry(|e| {
                if e.depth() == 0 { return true; }
                if e.file_type().is_dir() {
                    let name = e.file_name().to_string_lossy().to_lowercase();
                    return !ignored_dirs.contains(&name);
                }
                true
            });

        let mut total_files  = 0usize;
        let mut skipped_count = 0usize;

        for entry in walker.filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() { continue; }

            // Safe strip_prefix — skip entry if somehow outside root (e.g. broken symlink)
            let rel_path = match entry.path().strip_prefix(project_root) {
                Ok(p)  => p.to_path_buf(),
                Err(_) => {
                    print_warning(&format!(
                        "Skipping {} (could not resolve relative path)",
                        entry.path().display()
                    ));
                    continue;
                }
            };

            let metadata = match fs::metadata(entry.path()) {
                Ok(m)  => m,
                Err(e) => {
                    print_warning(&format!("Skipping {} (metadata error: {})", rel_path.display(), e));
                    continue;
                }
            };
            let size = metadata.len();

            // --- size limit ---
            if size > MAX_BACKUP_FILE_SIZE {
                let size_mb = size / (1024 * 1024);
                print_warning(&format!(
                    "Skipped {} ({} MB exceeds 50 MB limit)",
                    rel_path.display(), size_mb
                ));
                manifest.add_skipped_file(
                    rel_path,
                    SkipReason::TooLarge,
                    Some(format!("{} MB", size_mb)),
                );
                skipped_count += 1;
                continue;
            }

            // --- ignored filename ---
            let file_name = entry.file_name().to_string_lossy();
            if fmt_cfg.ignored_files.contains(&file_name.to_string()) {
                manifest.add_skipped_file(rel_path, SkipReason::IgnoredByUser, None);
                skipped_count += 1;
                continue;
            }

            // --- ignored extension ---
            let ext = entry.path().extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            if fmt_cfg.ignored_extensions.contains(&ext.to_lowercase()) {
                manifest.add_skipped_file(
                    rel_path,
                    SkipReason::IgnoredByConfig,
                    Some(format!(".{} extension ignored", ext)),
                );
                skipped_count += 1;
                continue;
            }

            // --- copy + hash in a single file read ---
            let dest_path = tmp_path.join(&rel_path);
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let (hash, _bytes_written) = Self::copy_and_hash(entry.path(), &dest_path)?;

            // modified time — graceful fallback to 0 rather than aborting the backup
            let modified = metadata.modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            // is_new: file doesn't currently exist at the project root
            // (always false during create_backup — the source IS the project root)
            // Tracked here for future incremental-backup support.
            manifest.add_file(rel_path, hash, size, modified, false);
            total_files += 1;
        }

        // Finalize manifest and write everything into tmp dir
        manifest.finalize();
        manifest.save(&tmp_path.join(".manifest.json"))?;
        manifest.save_ignore_txt(&tmp_path)?;

        // Atomic rename: tmp -> final
        fs::rename(&tmp_path, &final_path)?;

        // Update global index (after rename so final_path exists)
        Self::update_global_index(&project_hash, backup_number, &manifest)?;

        if skipped_count > 0 {
            print_warning(&format!(
                "Skipped {} file(s) — see ignore.txt for details",
                skipped_count
            ));
        }

        print_success(&format!(
            "Backup #{} created: {} ({} files, {})",
            backup_number,
            display_path(&final_path),
            total_files,
            manifest.total_size_human
        ));

        Ok(final_path)
    }

    /// Copy a file from `src` to `dst` while computing its SHA-256 hash,
    /// avoiding a second full file read just for hashing.
    /// Returns `(hex_hash, bytes_written)`.
    fn copy_and_hash(src: &Path, dst: &Path) -> Result<(String, u64)> {
        let mut src_file = fs::File::open(src)?;
        let mut dst_file = fs::File::create(dst)?;
        let mut hasher   = Sha256::new();
        let mut buffer   = [0u8; 65536]; // 64 KB chunks
        let mut total    = 0u64;

        loop {
            let n = src_file.read(&mut buffer)?;
            if n == 0 { break; }
            hasher.update(&buffer[..n]);
            dst_file.write_all(&buffer[..n])?;
            total += n as u64;
        }

        Ok((format!("{:x}", hasher.finalize()), total))
    }

    /// Determine the next backup number for a project directory.
    fn next_backup_number(project_backup_dir: &Path) -> usize {
        if !project_backup_dir.exists() { return 1; }

        let mut max_num = 0usize;
        if let Ok(entries) = fs::read_dir(project_backup_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if let Some(num_str) = name.strip_prefix("bkup_") {
                        if let Ok(num) = num_str.parse::<usize>() {
                            max_num = max_num.max(num);
                        }
                    }
                }
            }
        }
        max_num + 1
    }

    /// Update the global backup index after a successful backup.
    fn update_global_index(
        project_hash: &str,
        backup_number: usize,
        manifest: &BackupManifest,
    ) -> Result<()> {
        let index_path = BackupIndex::get_ntc_home().join("backups").join("index.json");

        let mut index: BackupIndex = if index_path.exists() {
            let content = fs::read_to_string(&index_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            BackupIndex::default()
        };

        let backups = index.projects
            .entry(project_hash.to_string())
            .or_default();
        if !backups.contains(&backup_number) {
            backups.push(backup_number);
            backups.sort_unstable();
        }

        let key = BackupIndex::summary_key(project_hash, backup_number);
        index.summaries.insert(key, BackupSummary {
            backup_number,
            created_at: manifest.created_at,
            total_size_human: manifest.total_size_human.clone(),
            file_count: manifest.files.len(),
        });

        if let Some(parent) = index_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&index_path, serde_json::to_string_pretty(&index)?)?;

        Ok(())
    }

    // ========================================================================
    // BACKUP LISTING & DISPLAY
    // ========================================================================

    /// List all backups for the current project (newest first).
    /// Returns `Vec<(backup_number, date_string, size_string, file_count)>`.
    pub fn list_backups(
        project_root: &Path,
    ) -> Result<Vec<(usize, String, String, usize)>> {
        let project_hash = compute_project_hash(project_root);
        let index_path   = BackupIndex::get_ntc_home().join("backups").join("index.json");

        if !index_path.exists() { return Ok(vec![]); }

        let index: BackupIndex = serde_json::from_str(&fs::read_to_string(index_path)?)?;

        let mut backups = Vec::new();
        if let Some(nums) = index.projects.get(&project_hash) {
            for num in nums {
                let key = BackupIndex::summary_key(&project_hash, *num);
                if let Some(s) = index.summaries.get(&key) {
                    let date = chrono::DateTime::from_timestamp(s.created_at as i64, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "Unknown date".to_string());
                    backups.push((*num, date, s.total_size_human.clone(), s.file_count));
                }
            }
        }

        // Newest first
        backups.sort_by(|a, b| b.0.cmp(&a.0));
        Ok(backups)
    }

    /// True if any backups exist for this project.
    pub fn has_backups(project_root: &Path) -> bool {
        Self::list_backups(project_root)
            .map(|l| !l.is_empty())
            .unwrap_or(false)
    }

    /// Print the backup storage location for this project.
    pub fn show_backup_location(project_root: &Path) {
        let project_hash = compute_project_hash(project_root);
        let backup_dir   = BackupIndex::get_project_backup_dir(&project_hash);
        print_info(&format!("Backups stored in: {}", display_path(&backup_dir)));
    }

    // ========================================================================
    // BACKUP CLEARING
    // ========================================================================

    /// Clear all backups for current project (with confirmation in shell.rs).
    pub fn clear_backups(project_root: &Path) -> Result<()> {
        let project_hash      = compute_project_hash(project_root);
        let project_backup_dir = BackupIndex::get_project_backup_dir(&project_hash);

        if !project_backup_dir.exists() {
            print_warning("No backups found for this project.");
            return Ok(());
        }

        let backups = Self::list_backups(project_root)?;
        let count   = backups.len();

        fs::remove_dir_all(&project_backup_dir)?;

        // Scrub from global index
        let index_path = BackupIndex::get_ntc_home().join("backups").join("index.json");
        if index_path.exists() {
            let mut index: BackupIndex =
                serde_json::from_str(&fs::read_to_string(&index_path)?)?;
            index.projects.remove(&project_hash);
            let prefix = format!("{}{}", project_hash, '#');
            index.summaries.retain(|k, _| !k.starts_with(&prefix));
            fs::write(&index_path, serde_json::to_string_pretty(&index)?)?;
        }

        print_success(&format!("Cleared {} backup(s) for this project.", count));
        Ok(())
    }

    // ========================================================================
    // RESTORE (with full undo support)
    // ========================================================================

    /// Collect all current project files as a set of relative path strings.
    /// Respects the same ignored-dirs and ignored-extensions config as backups.
    fn collect_current_file_set(project_root: &Path) -> HashSet<String> {
        let ignored_dirs = Config::global_get_ignored_dirs();
        let fmt_cfg      = FormatConfig::from_global();
        let mut set      = HashSet::new();

        let walker = WalkDir::new(project_root)
            .into_iter()
            .filter_entry(|e| {
                if e.depth() == 0 { return true; }
                if e.file_type().is_dir() {
                    let name = e.file_name().to_string_lossy().to_lowercase();
                    return !ignored_dirs.contains(&name);
                }
                true
            });

        for entry in walker.filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() { continue; }
            let rel = match entry.path().strip_prefix(project_root) {
                Ok(p)  => p.to_path_buf(),
                Err(_) => continue,
            };
            let file_name = entry.file_name().to_string_lossy();
            if fmt_cfg.ignored_files.contains(&file_name.to_string()) { continue; }
            let ext = entry.path().extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            if fmt_cfg.ignored_extensions.contains(&ext.to_lowercase()) { continue; }

            set.insert(rel.to_string_lossy().to_string());
        }
        set
    }

    /// Restore a backup by number.
    ///
    /// Before restoring:
    /// - Files that will be **overwritten** are copied to the undo directory.
    /// - Files that will be **created new** (in backup, absent in project) are recorded.
    /// - Files that will be **deleted** (in project, absent in backup) are copied to undo
    ///   and deleted, so the project matches the backup snapshot exactly.
    ///
    /// This ensures `undo_last_restore` is fully reversible in all three cases.
    pub fn restore_backup(
        project_root: &Path,
        backup_number: usize,
        interactive: bool,
    ) -> Result<()> {
        let project_hash  = compute_project_hash(project_root);
        let backup_path   = BackupIndex::get_backup_path(&project_hash, backup_number);
        let manifest_path = backup_path.join(".manifest.json");

        if !manifest_path.exists() {
            print_error(&format!("Backup #{} not found.", backup_number));
            return Ok(());
        }

        let manifest = BackupManifest::load(&manifest_path)?;

        // Build a set of backup file paths (normalised to forward-slash strings for
        // cross-platform comparison).
        let backup_file_set: HashSet<String> = manifest.files
            .iter()
            .map(|e| e.rel_path.to_string_lossy().replace('\\', "/"))
            .collect();

        // Collect current project files (same ignore rules as backup).
        let current_file_set = Self::collect_current_file_set(project_root);

        // Classify each file in the manifest as overwrite or new
        let mut to_overwrite: Vec<PathBuf> = Vec::new();
        let mut to_create:    Vec<PathBuf> = Vec::new();

        for entry in &manifest.files {
            let target = project_root.join(&entry.rel_path);
            if target.exists() {
                to_overwrite.push(entry.rel_path.clone());
            } else {
                to_create.push(entry.rel_path.clone());
            }
        }

        // FIX: Compute files that exist in the project but NOT in the backup.
        // These need to be deleted so the restore is a true point-in-time snapshot.
        let mut to_delete: Vec<PathBuf> = Vec::new();
        for current_rel in &current_file_set {
            let normalised = current_rel.replace('\\', "/");
            if !backup_file_set.contains(&normalised) {
                to_delete.push(PathBuf::from(current_rel));
            }
        }

        // Interactive confirmation
        if interactive && (!to_overwrite.is_empty() || !to_create.is_empty() || !to_delete.is_empty()) {
            println!();
            if !to_overwrite.is_empty() {
                println!("⚠  The following files will be OVERWRITTEN ({}):", to_overwrite.len());
                for p in to_overwrite.iter().take(10) {
                    println!("  ~ {}", p.display());
                }
                if to_overwrite.len() > 10 {
                    println!("  ... and {} more", to_overwrite.len() - 10);
                }
            }
            if !to_create.is_empty() {
                println!("⚠  The following files will be CREATED ({}):", to_create.len());
                for p in to_create.iter().take(10) {
                    println!("  + {}", p.display());
                }
                if to_create.len() > 10 {
                    println!("  ... and {} more", to_create.len() - 10);
                }
            }
            // FIX: show the user which extra files will be deleted
            if !to_delete.is_empty() {
                println!("⚠  The following files will be DELETED (not in backup) ({}):", to_delete.len());
                for p in to_delete.iter().take(10) {
                    println!("  - {}", p.display());
                }
                if to_delete.len() > 10 {
                    println!("  ... and {} more", to_delete.len() - 10);
                }
            }
            println!();
            print!("Continue with restore? (y/N): ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
                print_info("Restore cancelled.");
                return Ok(());
            }
        }

        // --- Build undo snapshot ---
        let undo_dir       = BackupIndex::get_undo_dir(&project_hash);
        let undo_files_dir = BackupIndex::get_undo_files_dir(&project_hash);

        // Replace any previous (unused) undo state
        if undo_dir.exists() {
            fs::remove_dir_all(&undo_dir)?;
        }
        fs::create_dir_all(&undo_files_dir)?;

        // Save originals of files that will be overwritten
        for rel_path in &to_overwrite {
            let src = project_root.join(rel_path);
            let dst = undo_files_dir.join(rel_path);
            if let Some(parent) = dst.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&src, &dst)?;
        }

        // FIX: Also save originals of files that will be deleted so undo can restore them.
        // We reuse the undo_files_dir with a sub-prefix to avoid collisions.
        let undo_deleted_dir = undo_dir.join("deleted");
        if !to_delete.is_empty() {
            fs::create_dir_all(&undo_deleted_dir)?;
            for rel_path in &to_delete {
                let src = project_root.join(rel_path);
                let dst = undo_deleted_dir.join(rel_path);
                if let Some(parent) = dst.parent() {
                    fs::create_dir_all(parent)?;
                }
                if let Err(e) = fs::copy(&src, &dst) {
                    print_warning(&format!(
                        "Could not snapshot {} for undo ({}), continuing",
                        rel_path.display(), e
                    ));
                }
            }
        }

        let undo_state = UndoState {
            restored_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            restored_from: backup_number,
            overwritten_files: to_overwrite.clone(),
            new_files_created: to_create.clone(),
            // FIX: record deleted files so undo knows to restore them
            deleted_files: to_delete.clone(),
        };
        fs::write(
            BackupIndex::get_undo_state_path(&project_hash),
            serde_json::to_string_pretty(&undo_state)?,
        )?;

        // --- Perform the restore ---
        let mut restored_count = 0usize;
        let mut failed_count   = 0usize;

        // 1. Copy backup files into project (overwrite + create)
        for entry in &manifest.files {
            let src = backup_path.join(&entry.rel_path);
            let dst = project_root.join(&entry.rel_path);

            if let Some(parent) = dst.parent() {
                fs::create_dir_all(parent)?;
            }

            match fs::copy(&src, &dst) {
                Ok(_)  => restored_count += 1,
                Err(e) => {
                    print_warning(&format!(
                        "Failed to restore {}: {}",
                        entry.rel_path.display(), e
                    ));
                    failed_count += 1;
                }
            }
        }

        // FIX: 2. Delete files that exist in the project but not in the backup
        let mut deleted_count = 0usize;
        for rel_path in &to_delete {
            let target = project_root.join(rel_path);
            if target.exists() {
                match fs::remove_file(&target) {
                    Ok(_)  => deleted_count += 1,
                    Err(e) => {
                        print_warning(&format!(
                            "Failed to delete extra file {}: {}",
                            rel_path.display(), e
                        ));
                        failed_count += 1;
                    }
                }
            }
        }

        let delete_msg = if deleted_count > 0 {
            format!(", {} extra file(s) removed", deleted_count)
        } else {
            String::new()
        };

        print_success(&format!(
            "Restored backup #{} ({} files{}{})",
            backup_number,
            restored_count,
            delete_msg,
            if failed_count > 0 { format!(", {} failed", failed_count) } else { String::new() }
        ));

        if !to_overwrite.is_empty() || !to_create.is_empty() || !to_delete.is_empty() {
            print_info("Previous state saved. Use 'unpd' to undo this restore.");
        }

        Ok(())
    }

    // ========================================================================
    // UNDO
    // ========================================================================

    /// Undo the last restore:
    /// - Restores overwritten files from undo storage.
    /// - Deletes files that were created new by the restore.
    /// - FIX: Restores files that were deleted by the restore (extra files).
    /// - Only cleans up undo storage after fully successful undo.
    pub fn undo_last_restore(project_root: &Path) -> Result<()> {
        let project_hash      = compute_project_hash(project_root);
        let undo_dir          = BackupIndex::get_undo_dir(&project_hash);
        let undo_metadata_path = BackupIndex::get_undo_state_path(&project_hash);
        let undo_files_dir    = BackupIndex::get_undo_files_dir(&project_hash);
        let undo_deleted_dir  = undo_dir.join("deleted");

        if !undo_metadata_path.exists() {
            print_warning("No restore operation to undo.");
            return Ok(());
        }

        let undo_state: UndoState =
            serde_json::from_str(&fs::read_to_string(&undo_metadata_path)?)?;

        let mut restored_count = 0usize;
        let mut failed         = false;

        // Restore overwritten files
        for rel_path in &undo_state.overwritten_files {
            let src = undo_files_dir.join(rel_path);
            let dst = project_root.join(rel_path);

            if src.exists() {
                if let Some(parent) = dst.parent() {
                    fs::create_dir_all(parent)?;
                }
                match fs::copy(&src, &dst) {
                    Ok(_)  => restored_count += 1,
                    Err(e) => {
                        print_warning(&format!(
                            "Could not restore {}: {}",
                            rel_path.display(), e
                        ));
                        failed = true;
                    }
                }
            } else {
                print_warning(&format!(
                    "Could not restore {} (original missing from undo storage)",
                    rel_path.display()
                ));
                failed = true;
            }
        }

        // Delete files that the restore created (they didn't exist before)
        let mut deleted_count = 0usize;
        for rel_path in &undo_state.new_files_created {
            let target = project_root.join(rel_path);
            if target.exists() {
                match fs::remove_file(&target) {
                    Ok(_)  => deleted_count += 1,
                    Err(e) => {
                        print_warning(&format!(
                            "Could not delete restored file {}: {}",
                            rel_path.display(), e
                        ));
                        failed = true;
                    }
                }
            }
        }

        // FIX: Restore files that the restore deleted (extra project files)
        let mut re_restored_count = 0usize;
        for rel_path in &undo_state.deleted_files {
            let src = undo_deleted_dir.join(rel_path);
            let dst = project_root.join(rel_path);

            if src.exists() {
                if let Some(parent) = dst.parent() {
                    fs::create_dir_all(parent)?;
                }
                match fs::copy(&src, &dst) {
                    Ok(_)  => re_restored_count += 1,
                    Err(e) => {
                        print_warning(&format!(
                            "Could not re-restore deleted file {}: {}",
                            rel_path.display(), e
                        ));
                        failed = true;
                    }
                }
            } else {
                print_warning(&format!(
                    "Could not re-restore {} (snapshot missing from undo storage)",
                    rel_path.display()
                ));
                failed = true;
            }
        }

        // Only clean up undo storage if everything succeeded
        if !failed {
            fs::remove_dir_all(&undo_dir)?;
        } else {
            print_warning("Some files could not be restored. Undo storage has been kept.");
        }

        let date = chrono::DateTime::from_timestamp(undo_state.restored_at as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "unknown date".to_string());

        print_success(&format!(
            "Undid restore of backup #{} ({} restored, {} deleted, {} re-restored)",
            undo_state.restored_from, restored_count, deleted_count, re_restored_count
        ));
        print_info(&format!("Restore was performed on: {}", date));

        Ok(())
    }

    /// Clear undo history without restoring (frees disk space).
    pub fn clear_undo_history(project_root: &Path) -> Result<()> {
        let project_hash = compute_project_hash(project_root);
        let undo_dir     = BackupIndex::get_undo_dir(&project_hash);

        if undo_dir.exists() {
            let size = Self::calculate_dir_size(&undo_dir);
            fs::remove_dir_all(&undo_dir)?;
            print_success(&format!(
                "Undo history cleared (freed {})",
                crate::explorer::human_readable_size(size)
            ));
        } else {
            print_warning("No undo history found.");
        }

        Ok(())
    }

    /// Calculate total size of a directory tree (for cleanup messages).
    fn calculate_dir_size(path: &Path) -> u64 {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter_map(|e| e.metadata().ok())
            .map(|m| m.len())
            .sum()
    }
}