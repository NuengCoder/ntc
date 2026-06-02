use crate::backup::BackupManager;
use crate::backup_manifest::{BackupIndex, BackupManifest};
use crate::config::Config;
use crate::filetype::FormatConfig;
use crate::output::{print_error, print_info, print_success};
use anyhow::Result;
use colored::*;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq)]
pub enum FileStatus {
    Modified,
    New,
    Deleted,
    Unchanged,
}

#[derive(Debug, Clone)]
pub struct FileDiff {
    pub rel_path: PathBuf,
    pub status: FileStatus,
    pub backup_hash: Option<String>,
    pub current_hash: Option<String>,
    pub side_by_side: Vec<SideBySideLine>,
}

#[derive(Debug, Clone)]
pub struct SideBySideLine {
    pub left_num: Option<usize>,
    pub left_content: Option<String>,
    pub right_num: Option<usize>,
    pub right_content: Option<String>,
    pub kind: SideBySideKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SideBySideKind {
    Unchanged,
    Modified,
    Added,
    Removed,
}

#[derive(Debug, Clone)]
enum DiffOp {
    Equal(usize, usize),   // FIX: store indices, not content strings
    Insert(usize),         // FIX: index into current
    Delete(usize),         // FIX: index into backup
}

pub struct BackupDiff;

impl BackupDiff {
    pub fn run_diff_interactive(project_root: &Path) -> Result<()> {
        let backups = BackupManager::list_backups(project_root)?;
        if backups.is_empty() {
            print_info("No backups found for this project. Use 'bkup' to create one.");
            return Ok(());
        }

        println!();
        println!("{}", "==================================================".cyan());
        println!("{}", "📊 Select a backup to diff against current project".cyan().bold());
        println!("{}", "==================================================".cyan());
        for (i, (num, date, size, file_count)) in backups.iter().enumerate() {
            println!(
                "  {}. Backup #{} — {} — {} — {} files",
                i + 1, num, date, size, file_count
            );
        }
        println!("  {}", "0. Cancel".red());
        println!();
        print!("{} ", format!("Select backup (1-{}): ", backups.len()).green());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "0" || input.is_empty() {
            println!("{}", "Diff cancelled.".dimmed());
            return Ok(());
        }

        match input.parse::<usize>() {
            Ok(n) if n >= 1 && n <= backups.len() => {
                let backup_num = backups[n - 1].0;
                Self::generate_diff(project_root, backup_num)
            }
            Ok(_) => {
                print_error(&format!("Invalid selection: {}", input));
                Ok(())
            }
            Err(_) => {
                print_error(&format!("Invalid input: {}", input));
                Ok(())
            }
        }
    }

    pub fn generate_diff(project_root: &Path, backup_number: usize) -> Result<()> {
        print_info(&format!("Comparing current project with backup #{}...", backup_number));

        let project_hash = crate::backup::compute_project_hash(project_root);
        let backup_path = BackupIndex::get_backup_path(&project_hash, backup_number);
        let manifest_path = backup_path.join(".manifest.json");

        if !manifest_path.exists() {
            print_error(&format!("Backup #{} not found.", backup_number));
            return Ok(());
        }

        let manifest = BackupManifest::load(&manifest_path)?;
        let project_name = project_root
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "project".to_string());

        let ignored_dirs = Config::global_get_ignored_dirs();
        let fmt_cfg = FormatConfig::from_global();

        let mut backup_files: HashMap<String, String> = HashMap::new();
        for entry in &manifest.files {
            // FIX: normalise path separators so Windows backups diff correctly on Linux
            let key = entry.rel_path.to_string_lossy().replace('\\', "/");
            backup_files.insert(key, entry.hash.clone());
        }

        let mut current_files: HashMap<String, String> = HashMap::new();
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
            let rel_path = match entry.path().strip_prefix(project_root) {
                Ok(p) => p.to_path_buf(),
                Err(_) => continue,
            };
            // FIX: normalise separators for consistent cross-platform comparison
            let rel_str = rel_path.to_string_lossy().replace('\\', "/");

            let file_name = entry.file_name().to_string_lossy();
            if fmt_cfg.ignored_files.contains(&file_name.to_string()) {
                continue;
            }
            let ext = entry.path().extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            if fmt_cfg.ignored_extensions.contains(&ext.to_lowercase()) {
                continue;
            }

            if let Ok(content) = fs::read(entry.path()) {
                let hash = format!("{:x}", Sha256::digest(&content));
                current_files.insert(rel_str, hash);
            }
        }

        let mut diffs: Vec<FileDiff> = Vec::new();

        for (rel_path_str, backup_hash) in &backup_files {
            let rel_path = PathBuf::from(rel_path_str);
            if let Some(current_hash) = current_files.get(rel_path_str) {
                if backup_hash == current_hash {
                    diffs.push(FileDiff {
                        rel_path,
                        status: FileStatus::Unchanged,
                        backup_hash: Some(backup_hash.clone()),
                        current_hash: Some(current_hash.clone()),
                        side_by_side: Vec::new(),
                    });
                } else {
                    let is_binary = crate::filetype::is_known_unsupported_format(&project_root.join(&rel_path));
                    let side_by_side = if is_binary {
                        Vec::new()
                    } else {
                        Self::compute_side_by_side(
                            &backup_path.join(&rel_path),
                            &project_root.join(&rel_path),
                        )
                    };
                    diffs.push(FileDiff {
                        rel_path,
                        status: FileStatus::Modified,
                        backup_hash: Some(backup_hash.clone()),
                        current_hash: Some(current_hash.clone()),
                        side_by_side,
                    });
                }
            } else {
                diffs.push(FileDiff {
                    rel_path,
                    status: FileStatus::Deleted,
                    backup_hash: Some(backup_hash.clone()),
                    current_hash: None,
                    side_by_side: Vec::new(),
                });
            }
        }

        for (rel_path_str, current_hash) in &current_files {
            if !backup_files.contains_key(rel_path_str) {
                diffs.push(FileDiff {
                    rel_path: PathBuf::from(rel_path_str),
                    status: FileStatus::New,
                    backup_hash: None,
                    current_hash: Some(current_hash.clone()),
                    side_by_side: Vec::new(),
                });
            }
        }

        diffs.sort_by(|a, b| {
            let order = |s: &FileStatus| match s {
                FileStatus::Modified => 0,
                FileStatus::New => 1,
                FileStatus::Deleted => 2,
                FileStatus::Unchanged => 3,
            };
            order(&a.status).cmp(&order(&b.status))
                .then(a.rel_path.cmp(&b.rel_path))
        });

        let html = Self::generate_html(&project_name, backup_number, &diffs, &manifest);

        let output_filename = format!("ntc_{}_backup_{}.html", project_name, backup_number);
        let output_path = crate::output::build_output_path(&output_filename);
        fs::write(&output_path, html)?;

        print_success(&format!("Diff report saved to: {}", output_path.display()));
        Self::open_in_browser(&output_path);

        Ok(())
    }

    /// ── Myers diff algorithm ─────────────────────────────────────────────
    /// Computes the shortest edit script (SES) between two line sequences.
    /// FIX: Returns index-based DiffOp values (not content strings) to avoid
    /// the content-search scan that was vulnerable to duplicate lines and
    /// index-out-of-bounds panics.
    fn myers_diff(n: usize, m: usize, backup: &[String], current: &[String]) -> Vec<DiffOp> {
        if n == 0 && m == 0 {
            return Vec::new();
        }
        if n == 0 {
            return (0..m).map(DiffOp::Insert).collect();
        }
        if m == 0 {
            return (0..n).map(DiffOp::Delete).collect();
        }

        let max = n + m;
        let offset = max;
        let size = 2 * max + 1;
        let mut v = vec![-1isize; size];
        v[offset + 1] = 0;

        let mut trace: Vec<Vec<isize>> = Vec::new();
        let mut ses_len = max; // FIX: default to max so backtrack is bounded even without break

        'outer: for d in 0..=max {
            trace.push(v.clone());
            let d_signed = d as isize;

            for k in (-d_signed..=d_signed).step_by(2) {
                let idx = (k + offset as isize) as usize;

                let x = if k == -d_signed || (k != d_signed
                    && v[(k - 1 + offset as isize) as usize]
                       < v[(k + 1 + offset as isize) as usize])
                {
                    v[(k + 1 + offset as isize) as usize]
                } else {
                    v[(k - 1 + offset as isize) as usize] + 1
                };

                let mut cx = x;
                let mut cy = cx - k;

                while (cx as usize) < n && (cy as usize) < m
                    && backup[cx as usize] == current[cy as usize]
                {
                    cx += 1;
                    cy += 1;
                }

                v[idx] = cx;

                if cx as usize >= n && cy as usize >= m {
                    ses_len = d;
                    break 'outer;
                }
            }
        }

        // Backtrack — reconstruct edit operations as index pairs
        let mut ops: Vec<DiffOp> = Vec::new();
        let mut x = n as isize;
        let mut y = m as isize;

        for d in (0..=ses_len).rev() {
            let v_slice = &trace[d];
            let d_signed = d as isize;
            let k_val = x - y;

            let prev_k = if k_val == -d_signed || (k_val != d_signed
                && v_slice[(k_val - 1 + offset as isize) as usize]
                   < v_slice[(k_val + 1 + offset as isize) as usize])
            {
                k_val + 1
            } else {
                k_val - 1
            };

            let prev_x = v_slice[(prev_k + offset as isize) as usize];
            let prev_y = prev_x - prev_k;

            // Diagonal moves backwards — these are Equal ops
            while x > prev_x && y > prev_y {
                x -= 1;
                y -= 1;
                // FIX: store indices directly instead of content
                ops.push(DiffOp::Equal(x as usize, y as usize));
            }

            if d > 0 {
                if x == prev_x {
                    y -= 1;
                    // FIX: store index into current
                    ops.push(DiffOp::Insert(y as usize));
                } else {
                    x -= 1;
                    // FIX: store index into backup
                    ops.push(DiffOp::Delete(x as usize));
                }
            }

            x = prev_x;
            y = prev_y;
        }

        ops.reverse();
        ops
    }

    /// FIX: Rewritten to use index-based DiffOps.
    /// - No more content-scan while loops that could panic or skip past duplicate lines.
    /// - CRLF stripping applied when reading file lines so cross-platform diffs are clean.
    /// - Adjacent Delete+Insert pairs are now fused into Modified rows.
    fn compute_side_by_side(backup_path: &Path, current_path: &Path) -> Vec<SideBySideLine> {
        let backup_content  = fs::read_to_string(backup_path).unwrap_or_default();
        let current_content = fs::read_to_string(current_path).unwrap_or_default();

        // FIX: strip trailing \r so Windows CRLF files compare cleanly on Linux/macOS
        let backup_lines: Vec<String> = backup_content
            .lines()
            .map(|s| s.trim_end_matches('\r').to_string())
            .collect();
        let current_lines: Vec<String> = current_content
            .lines()
            .map(|s| s.trim_end_matches('\r').to_string())
            .collect();

        let n = backup_lines.len();
        let m = current_lines.len();

        // FIX: use index-based Myers diff
        let ops = Self::myers_diff(n, m, &backup_lines, &current_lines);

        let mut result: Vec<SideBySideLine> = Vec::new();

        // FIX: process ops using stored indices — no scanning, no bounds risk
        let mut i = 0;
        while i < ops.len() {
            match &ops[i] {
                DiffOp::Equal(bi, ci) => {
                    result.push(SideBySideLine {
                        left_num:  Some(bi + 1),
                        left_content:  Some(backup_lines[*bi].clone()),
                        right_num: Some(ci + 1),
                        right_content: Some(current_lines[*ci].clone()),
                        kind: SideBySideKind::Unchanged,
                    });
                    i += 1;
                }
                // FIX: fuse adjacent Delete+Insert into a single Modified row
                DiffOp::Delete(bi) => {
                    if i + 1 < ops.len() {
                        if let DiffOp::Insert(ci) = &ops[i + 1] {
                            result.push(SideBySideLine {
                                left_num:  Some(bi + 1),
                                left_content:  Some(backup_lines[*bi].clone()),
                                right_num: Some(ci + 1),
                                right_content: Some(current_lines[*ci].clone()),
                                kind: SideBySideKind::Modified,
                            });
                            i += 2; // consume both Delete and Insert
                            continue;
                        }
                    }
                    result.push(SideBySideLine {
                        left_num:  Some(bi + 1),
                        left_content:  Some(backup_lines[*bi].clone()),
                        right_num: None,
                        right_content: None,
                        kind: SideBySideKind::Removed,
                    });
                    i += 1;
                }
                DiffOp::Insert(ci) => {
                    result.push(SideBySideLine {
                        left_num:  None,
                        left_content:  None,
                        right_num: Some(ci + 1),
                        right_content: Some(current_lines[*ci].clone()),
                        kind: SideBySideKind::Added,
                    });
                    i += 1;
                }
            }
        }

        result
    }

    fn generate_html(
        project_name: &str,
        backup_number: usize,
        diffs: &[FileDiff],
        manifest: &BackupManifest,
    ) -> String {
        let mut modified_count = 0usize;
        let mut new_count = 0usize;
        let mut deleted_count = 0usize;
        let mut unchanged_count = 0usize;

        for d in diffs {
            match d.status {
                FileStatus::Modified => modified_count += 1,
                FileStatus::New => new_count += 1,
                FileStatus::Deleted => deleted_count += 1,
                FileStatus::Unchanged => unchanged_count += 1,
            }
        }

        let mut files_html = String::new();
        for (idx, diff) in diffs.iter().enumerate() {
            let (icon, status_class, status_label) = match diff.status {
                FileStatus::Modified => ("⚠️", "status-modified", "Modified"),
                FileStatus::New => ("🆕", "status-new", "New!"),
                FileStatus::Deleted => ("🗑️", "status-deleted", "Deleted!"),
                FileStatus::Unchanged => ("ℹ️", "status-unchanged", "Unchanged"),
            };

            let file_label = diff.rel_path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();

            let is_binary_skipped = diff.status == FileStatus::Modified && diff.side_by_side.is_empty();
            let side_by_side_html = if is_binary_skipped {
                format!(
                    r#"<div class="binary-notice">📦 Binary file — diff not available</div>"#
                )
            } else if diff.status == FileStatus::Modified && !diff.side_by_side.is_empty() {
                let mut rows = String::new();
                for sbs in &diff.side_by_side {
                    let (left_cls, right_cls) = match sbs.kind {
                        SideBySideKind::Unchanged => ("", ""),
                        // FIX: Modified now rendered correctly (both cells highlighted)
                        SideBySideKind::Modified => ("sbs-removed", "sbs-added"),
                        SideBySideKind::Added => ("sbs-empty", "sbs-added"),
                        SideBySideKind::Removed => ("sbs-removed", "sbs-empty"),
                    };

                    let left_num = sbs.left_num.map(|n| n.to_string()).unwrap_or_default();
                    let left_content = sbs.left_content.as_deref()
                        .map(|s| s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;"))
                        .unwrap_or_default();
                    let right_num = sbs.right_num.map(|n| n.to_string()).unwrap_or_default();
                    let right_content = sbs.right_content.as_deref()
                        .map(|s| s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;"))
                        .unwrap_or_default();

                    // Show direction arrow only for pure Add/Remove rows
                    let empty_marker = match sbs.kind {
                        SideBySideKind::Added   => r#"<span class="empty-marker">⟵ added</span>"#,
                        SideBySideKind::Removed => r#"<span class="empty-marker">⟶ removed</span>"#,
                        _ => "",
                    };

                    // For Added rows: put the arrow in the LEFT (empty) cell
                    // For Removed rows: put the arrow in the RIGHT (empty) cell
                    let (left_extra, right_extra) = match sbs.kind {
                        SideBySideKind::Added   => (empty_marker, ""),
                        SideBySideKind::Removed => ("", empty_marker),
                        _ => ("", ""),
                    };

                    rows.push_str(&format!(
                        r#"<div class="sbs-row {left_cls} {right_cls}">
                            <div class="sbs-cell left"><span class="ln">{left_num}</span><span class="lc">{left_content}{left_extra}</span></div>
                            <div class="sbs-gutter"></div>
                            <div class="sbs-cell right"><span class="ln">{right_num}</span><span class="lc">{right_content}{right_extra}</span></div>
                        </div>"#,
                        left_cls = left_cls,
                        right_cls = right_cls,
                        left_num = left_num,
                        left_content = left_content,
                        left_extra = left_extra,
                        right_num = right_num,
                        right_content = right_content,
                        right_extra = right_extra,
                    ));
                }

                let line_count = diff.side_by_side.len();
                format!(
                    r#"<div class="sbs-container" id="sbs-{}">
                        <div class="sbs-header" onclick="toggleSbs('sbs-{}')">
                            <span class="sbs-toggle">▼</span>
                            <span class="sbs-title">Side-by-side diff ({} lines)</span>
                        </div>
                        <div class="sbs-body">
                            <div class="sbs-panel-header">
                                <div class="sbs-panel-title">Backup — {}</div>
                                <div class="sbs-panel-title">Current — {}</div>
                            </div>
                            <div class="sbs-grid">{}</div>
                        </div>
                    </div>"#,
                    idx, idx, line_count,
                    file_label, file_label,
                    rows
                )
            } else {
                String::new()
            };

            let rel_path_str = diff.rel_path.to_string_lossy();
            files_html.push_str(&format!(
                r#"<div class="file-entry {}">
                    <div class="file-info" onclick="toggleFile('file-{}')">
                        <span class="file-toggle">▶</span>
                        <span class="file-icon">{}</span>
                        <span class="file-path">{}</span>
                        <span class="file-status-label {}">{}</span>
                    </div>
                    <div class="file-details" id="file-{}" style="display:none">
                        {}
                    </div>
                </div>"#,
                status_class, idx, icon, rel_path_str, status_class, status_label, idx, side_by_side_html
            ));
        }

        let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let backup_date = chrono::DateTime::from_timestamp(manifest.created_at as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>ntc Diff — {} vs Backup #{}</title>
<style>
* {{ margin: 0; padding: 0; box-sizing: border-box; }}
body {{ font-family: 'Segoe UI', system-ui, -apple-system, sans-serif; background: #0d1117; color: #c9d1d9; padding: 20px; }}
.container {{ max-width: 1320px; margin: 0 auto; }}
.header {{ background: linear-gradient(135deg, #161b22, #0d1117); border: 1px solid #30363d; border-radius: 8px; padding: 24px; margin-bottom: 20px; }}
.header h1 {{ font-size: 24px; color: #58a6ff; margin-bottom: 8px; }}
.header .meta {{ color: #8b949e; font-size: 14px; }}
.header .meta span {{ margin-right: 20px; }}
.summary {{ display: flex; gap: 12px; margin-bottom: 20px; flex-wrap: wrap; }}
.summary-item {{ flex: 1; min-width: 140px; padding: 16px; border-radius: 8px; text-align: center; font-size: 14px; }}
.summary-item .count {{ font-size: 28px; font-weight: bold; display: block; }}
.summary-modified {{ background: rgba(187,128,9,0.15); border: 1px solid rgba(187,128,9,0.4); }}
.summary-modified .count {{ color: #d29922; }}
.summary-new {{ background: rgba(63,185,80,0.15); border: 1px solid rgba(63,185,80,0.4); }}
.summary-new .count {{ color: #3fb950; }}
.summary-deleted {{ background: rgba(248,81,73,0.15); border: 1px solid rgba(248,81,73,0.4); }}
.summary-deleted .count {{ color: #f85149; }}
.summary-unchanged {{ background: rgba(88,166,255,0.1); border: 1px solid rgba(88,166,255,0.25); }}
.summary-unchanged .count {{ color: #58a6ff; }}
.file-entry {{ border: 1px solid #30363d; border-radius: 6px; margin-bottom: 4px; overflow: hidden; }}
.file-entry.status-modified {{ border-left: 4px solid #d29922; }}
.file-entry.status-new {{ border-left: 4px solid #3fb950; }}
.file-entry.status-deleted {{ border-left: 4px solid #f85149; }}
.file-entry.status-unchanged {{ border-left: 4px solid #58a6ff; }}
.file-info {{ padding: 10px 16px; cursor: pointer; display: flex; align-items: center; gap: 8px; transition: background 0.15s; user-select: none; }}
.file-info:hover {{ background: #161b22; }}
.file-toggle {{ font-size: 10px; color: #8b949e; transition: transform 0.2s; }}
.file-toggle.open {{ transform: rotate(90deg); }}
.file-icon {{ font-size: 16px; }}
.file-path {{ flex: 1; font-family: 'Consolas', 'Courier New', monospace; font-size: 14px; }}
.file-status-label {{ font-size: 11px; font-weight: 600; padding: 2px 8px; border-radius: 10px; text-transform: uppercase; }}
.status-modified .file-status-label {{ background: rgba(187,128,9,0.2); color: #d29922; }}
.status-new .file-status-label {{ background: rgba(63,185,80,0.2); color: #3fb950; }}
.status-deleted .file-status-label {{ background: rgba(248,81,73,0.2); color: #f85149; }}
.status-unchanged .file-status-label {{ background: rgba(88,166,255,0.15); color: #58a6ff; }}
.file-details {{ padding: 0; }}

/* ── Side-by-side ────────────────────────────────────── */
.binary-notice {{ padding: 12px 16px; color: #8b949e; font-size: 13px; text-align: center; }}
.sbs-container {{ border-top: 1px solid #30363d; }}
.sbs-header {{ padding: 8px 16px; cursor: pointer; font-size: 13px; color: #8b949e; display: flex; align-items: center; gap: 6px; user-select: none; }}
.sbs-header:hover {{ background: #161b22; }}
.sbs-toggle {{ font-size: 10px; transition: transform 0.15s; }}
.sbs-toggle.open {{ transform: rotate(180deg); }}
.sbs-title {{ font-weight: 500; }}
.sbs-body {{ display: none; }}
.sbs-body.open {{ display: block; }}
.sbs-panel-header {{ display: flex; border-bottom: 1px solid #21262d; background: #161b22; }}
.sbs-panel-title {{ flex: 1; padding: 6px 16px; font-size: 12px; font-weight: 600; color: #8b949e; font-family: 'Consolas', 'Courier New', monospace; }}
.sbs-panel-title:first-child {{ border-right: 1px solid #21262d; }}
.sbs-grid {{ overflow-x: auto; }}
.sbs-row {{ display: flex; min-height: 22px; font-family: 'Consolas', 'Courier New', monospace; font-size: 12px; line-height: 1.5; }}
.sbs-row:hover {{ background: rgba(255,255,255,0.02); }}
.sbs-cell {{ flex: 1; display: flex; padding: 0; overflow: hidden; white-space: pre; }}
.sbs-cell .ln {{ min-width: 44px; text-align: right; padding: 0 8px; color: #484f58; user-select: none; flex-shrink: 0; }}
.sbs-cell .lc {{ flex: 1; padding: 0 4px; white-space: pre-wrap; word-break: break-all; }}
.sbs-gutter {{ width: 0; flex-shrink: 0; }}
.empty-marker {{ color: #484f58; font-size: 11px; font-style: italic; }}

/* row background colors */
.sbs-row.sbs-removed {{ background: rgba(248,81,73,0.12); }}
.sbs-row.sbs-added   {{ background: rgba(63,185,80,0.12); }}
.sbs-row.sbs-removed .left  {{ background: rgba(248,81,73,0.15); }}
.sbs-row.sbs-added   .right {{ background: rgba(63,185,80,0.15); }}
/* Modified: both cells highlighted in their respective colours */
.sbs-row.sbs-removed.sbs-added {{ background: transparent; }}
.sbs-row.sbs-removed.sbs-added .left  {{ background: rgba(248,81,73,0.18); }}
.sbs-row.sbs-removed.sbs-added .right {{ background: rgba(63,185,80,0.18); }}

.footer {{ text-align: center; padding: 20px; color: #484f58; font-size: 12px; margin-top: 20px; border-top: 1px solid #30363d; }}
.search-bar {{ margin-bottom: 16px; }}
.search-bar input {{ width: 100%; padding: 8px 12px; background: #0d1117; border: 1px solid #30363d; border-radius: 6px; color: #c9d1d9; font-size: 14px; outline: none; }}
.search-bar input:focus {{ border-color: #58a6ff; }}
.search-bar input::placeholder {{ color: #484f58; }}
</style>
</head>
<body>
<div class="container">
    <div class="header">
        <h1>📊 {} vs Backup #{}</h1>
        <div class="meta">
            <span>📅 Generated: {}</span>
            <span>📦 Backup date: {}</span>
            <span>📁 {} total files</span>
        </div>
    </div>
    <div class="summary">
        <div class="summary-item summary-modified">
            <span class="count">{}</span>
            Modified
        </div>
        <div class="summary-item summary-new">
            <span class="count">{}</span>
            New
        </div>
        <div class="summary-item summary-deleted">
            <span class="count">{}</span>
            Deleted
        </div>
        <div class="summary-item summary-unchanged">
            <span class="count">{}</span>
            Unchanged
        </div>
    </div>
    <div class="search-bar">
        <input type="text" id="searchInput" placeholder="🔍 Search files..." oninput="filterFiles()">
    </div>
    <div id="file-list">
        {}
    </div>
    <div class="footer">
        Generated by ntc v{} | {}
    </div>
</div>
<script>
function toggleSbs(id) {{
    var body = document.getElementById(id).querySelector('.sbs-body');
    var toggle = document.getElementById(id).querySelector('.sbs-toggle');
    if (body.classList.contains('open')) {{
        body.classList.remove('open');
        toggle.classList.remove('open');
    }} else {{
        body.classList.add('open');
        toggle.classList.add('open');
    }}
}}
function toggleFile(id) {{
    var details = document.getElementById(id);
    var toggle = details.previousElementSibling.querySelector('.file-toggle');
    if (details.style.display === 'none' || details.style.display === '') {{
        details.style.display = 'block';
        toggle.classList.add('open');
    }} else {{
        details.style.display = 'none';
        toggle.classList.remove('open');
    }}
}}
function filterFiles() {{
    var input = document.getElementById('searchInput').value.toLowerCase();
    var entries = document.getElementById('file-list').getElementsByClassName('file-entry');
    for (var i = 0; i < entries.length; i++) {{
        var path = entries[i].querySelector('.file-path').textContent.toLowerCase();
        entries[i].style.display = path.includes(input) ? '' : 'none';
    }}
}}
</script>
</body>
</html>"#,
            project_name, backup_number,
            project_name, backup_number,
            date, backup_date, diffs.len(),
            modified_count, new_count, deleted_count, unchanged_count,
            files_html,
            env!("CARGO_PKG_VERSION"), date
        )
    }

    fn open_in_browser(path: &Path) {
        let url = path.to_string_lossy();
        #[cfg(windows)]
        {
            let _ = std::process::Command::new("cmd")
                .args(["/C", "start", "", &url])
                .status();
        }
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("open").arg(&*url).status();
        }
        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("xdg-open").arg(&*url).status();
        }
        #[cfg(target_os = "android")]
        {
            crate::output::print_info(&format!("Open the file manually: {}", url));
        }
    }
}