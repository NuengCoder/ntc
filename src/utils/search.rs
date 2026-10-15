// src/search.rs
// File and directory search for ntc
// Version: v1.8.1
// Used by: fs (file search) and ds (directory search) commands
// Cross-platform: Windows, Linux, macOS

use crate::config::Config;
use crate::filetype::{make_ignored_dir_filter, strip_unc_prefix, FormatConfig};
use crate::fuzzy::{top_fuzzy_dir_matches, top_fuzzy_matches, MAX_FUZZY_SUGGESTIONS};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Check whether a pattern contains wildcard characters (`*`, `%`, `_`, `$`).
pub fn has_wildcards(pattern: &str) -> bool {
    pattern.contains('*') || pattern.contains('%') || pattern.contains('_') || pattern.contains('$')
}

/// Match a name against a wildcard pattern.
/// Supports:
///   `*` or `%` — match any sequence of characters (including empty)
///   `_`        — match any single character
///   `$`        — anchor at end of string (pattern must end with `$`)
/// Matching is case-insensitive.
pub fn matches_wildcard(pattern: &str, name: &str) -> bool {
    let pattern = pattern.to_lowercase();
    let name = name.to_lowercase();

    let has_end_anchor = pattern.ends_with('$');
    let pat = if has_end_anchor {
        &pattern[..pattern.len() - 1]
    } else {
        pattern.as_str()
    };

    let pat_chars: Vec<char> = pat.chars().collect();
    let name_chars: Vec<char> = name.chars().collect();

    // Simple recursive/iterative matching
    fn wildcard_match(pat: &[char], name: &[char]) -> bool {
        if pat.is_empty() {
            return name.is_empty();
        }

        let pc = pat[0];
        if pc == '*' || pc == '%' {
            // Try matching 0 or more characters
            let rest = &pat[1..];
            for i in 0..=name.len() {
                if wildcard_match(rest, &name[i..]) {
                    return true;
                }
            }
            false
        } else if pc == '_' {
            if name.is_empty() {
                return false;
            }
            wildcard_match(&pat[1..], &name[1..])
        } else {
            if name.is_empty() || pc != name[0] {
                return false;
            }
            wildcard_match(&pat[1..], &name[1..])
        }
    }

    if has_end_anchor {
        // Match entire string exactly
        wildcard_match(pat_chars.as_slice(), name_chars.as_slice())
    } else {
        // Match anywhere in the string (like a glob without start anchor)
        for start in 0..=name_chars.len() {
            if wildcard_match(pat_chars.as_slice(), &name_chars[start..]) {
                return true;
            }
        }
        false
    }
}

// ============================================================================
// Types
// ============================================================================

/// Result of a file/directory search
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Full path to the file or directory
    pub full_path: PathBuf,
    /// Just the name (e.g., "main.c" or "src")
    pub name: String,
    /// Whether this is an exact match, partial match, or fuzzy suggestion
    pub match_kind: MatchKind,
    /// Similarity score (0.0–1.0) — 1.0 for exact/partial, fuzzy score for fuzzy
    pub score: f64,
}

/// How the result was matched
#[derive(Debug, Clone, PartialEq)]
pub enum MatchKind {
    /// Full filename equality (case-insensitive)
    Exact,
    /// Filename contains the pattern (case-insensitive)
    Partial,
    /// Fuzzy (Jaro-Winkler) match
    Fuzzy,
}

impl SearchResult {
    pub fn exact(full_path: PathBuf, name: String) -> Self {
        Self {
            full_path,
            name,
            match_kind: MatchKind::Exact,
            score: 1.0,
        }
    }

    pub fn partial(full_path: PathBuf, name: String) -> Self {
        Self {
            full_path,
            name,
            match_kind: MatchKind::Partial,
            score: 1.0,
        }
    }

    pub fn fuzzy(full_path: PathBuf, name: String, score: f64) -> Self {
        Self {
            full_path,
            name,
            match_kind: MatchKind::Fuzzy,
            score,
        }
    }

    /// True for exact or partial matches (not fuzzy)
    pub fn is_exact(&self) -> bool {
        self.match_kind == MatchKind::Exact
    }
}

// ============================================================================
// Internal helpers
// ============================================================================

/// A candidate entry collected during the walk.
/// Keeps path separate from name so duplicate names (same name, different dirs)
/// are preserved — unlike a HashMap<name, path> which would drop duplicates.
struct Candidate {
    name: String,
    full_path: PathBuf,
}

/// Walk `root` up to `max_depth`, yielding file entries while respecting
/// the global ignored-dirs and ignored-extensions/files config.
///
/// `max_depth`:
///   0  = current directory only (no subdirectories)
///   n  = recurse up to n levels deep
///   usize::MAX = unlimited depth
///
/// Note: WalkDir's own max_depth(0) means root only (no children at all).
/// We translate: our 0 → WalkDir 1 (files in root dir), our n → WalkDir n+1.
pub(crate) fn walk_files<'a>(
    root: &'a Path,
    max_depth: usize,
    ignored_dirs: &'a std::collections::HashSet<String>,
    fmt_cfg: &'a FormatConfig,
) -> impl Iterator<Item = (String, PathBuf)> + 'a {
    let walkdir_depth = max_depth.saturating_add(1);

    WalkDir::new(root)
        .max_depth(walkdir_depth)
        .into_iter()
        .filter_entry(make_ignored_dir_filter(ignored_dirs))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(move |e| {
            let file_name = e.file_name().to_string_lossy().to_string();

            // Skip ignored filenames
            if fmt_cfg.ignored_files.contains(&file_name) {
                return None;
            }
            // Skip ignored extensions
            if let Some(ext) = e.path().extension().and_then(|x| x.to_str()) {
                if fmt_cfg.ignored_extensions.contains(&ext.to_lowercase()) {
                    return None;
                }
            }

            Some((file_name, e.path().to_path_buf()))
        })
}

/// Walk `root` up to `max_depth`, yielding directory entries while respecting
/// the global ignored-dirs config. Skips the root itself (depth 0).
fn walk_dirs<'a>(
    root: &'a Path,
    max_depth: usize,
    ignored_dirs: &'a std::collections::HashSet<String>,
) -> impl Iterator<Item = (String, PathBuf)> + 'a {
    let walkdir_depth = max_depth.saturating_add(1);

    WalkDir::new(root)
        .max_depth(walkdir_depth)
        .into_iter()
        .filter_entry(make_ignored_dir_filter(ignored_dirs))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir() && e.depth() > 0)
        .map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            let path = e.path().to_path_buf();
            (name, path)
        })
}

// ============================================================================
// Public search API
// ============================================================================

/// Search for files matching a pattern in the current directory tree.
///
/// Match priority (returns the best tier found, never mixes tiers):
/// 1. **Exact**   — full filename equality (case-insensitive)
/// 2. **Partial** — filename contains the pattern (case-insensitive)
/// 3. **Fuzzy**   — Jaro-Winkler similarity ≥ 0.75, top 3
///
/// # Arguments
/// * `root`      - Starting directory (CWD) — never searches above this
/// * `pattern`   - Filename or partial name to search for (case-insensitive)
/// * `max_depth` - 0 = current dir only, n = recurse n levels, usize::MAX = unlimited
pub fn search_files(root: &Path, pattern: &str, max_depth: usize) -> Vec<SearchResult> {
    let pattern_lower = pattern.to_lowercase();
    let ignored_dirs = Config::get_ignored_dirs_combined();
    let fmt_cfg = FormatConfig::from_both();
    let use_wildcard = has_wildcards(pattern);

    if use_wildcard {
        let mut wild_matches: Vec<SearchResult> = Vec::new();
        for (file_name, full_path) in walk_files(root, max_depth, &ignored_dirs, &fmt_cfg) {
            if matches_wildcard(pattern, &file_name) {
                wild_matches.push(SearchResult::exact(full_path, file_name));
            }
        }
        wild_matches.sort_by_cached_key(|a| a.name.to_lowercase());
        return wild_matches;
    }

    let mut exact_matches: Vec<SearchResult> = Vec::new();
    let mut partial_matches: Vec<SearchResult> = Vec::new();
    let mut fuzzy_candidates: Vec<Candidate> = Vec::new();

    for (file_name, full_path) in walk_files(root, max_depth, &ignored_dirs, &fmt_cfg) {
        let file_name_lower = file_name.to_lowercase();

        if file_name_lower == pattern_lower {
            exact_matches.push(SearchResult::exact(full_path, file_name));
        } else if file_name_lower.contains(&pattern_lower) {
            partial_matches.push(SearchResult::partial(full_path, file_name));
        } else {
            fuzzy_candidates.push(Candidate {
                name: file_name,
                full_path,
            });
        }
    }

    // Return the best tier that has results
    if !exact_matches.is_empty() {
        exact_matches.sort_by_cached_key(|a| a.name.to_lowercase());
        return exact_matches;
    }

    if !partial_matches.is_empty() {
        partial_matches.sort_by_cached_key(|a| a.name.to_lowercase());
        return partial_matches;
    }

    // Fuzzy fallback — collect names for scoring, keep Candidate for path lookup
    let candidate_names: Vec<String> = fuzzy_candidates.iter().map(|c| c.name.clone()).collect();
    let fuzzy_hits = top_fuzzy_matches(&candidate_names, pattern, MAX_FUZZY_SUGGESTIONS);

    // fuzzy_hits gives us (name, score); look up the matching Candidate(s).
    // Multiple files can share the same name in different subdirectories —
    // include all of them rather than only the first (no HashMap collision).
    let mut results: Vec<SearchResult> = Vec::new();
    for (hit_name, score) in fuzzy_hits {
        for candidate in &fuzzy_candidates {
            if &candidate.name == hit_name {
                results.push(SearchResult::fuzzy(
                    candidate.full_path.clone(),
                    candidate.name.clone(),
                    score,
                ));
            }
        }
    }

    results
}

/// Search for directories matching a pattern in the current directory tree.
///
/// Match priority (returns the best tier found, never mixes tiers):
/// 1. **Exact**   — full directory name equality (case-insensitive)
/// 2. **Partial** — directory name contains the pattern (case-insensitive)
/// 3. **Fuzzy**   — Jaro-Winkler similarity ≥ 0.72, top 3
///
/// # Arguments
/// * `root`      - Starting directory (CWD) — never searches above this
/// * `pattern`   - Directory name or partial name to search for (case-insensitive)
/// * `max_depth` - 0 = current dir only, n = recurse n levels, usize::MAX = unlimited
pub fn search_directories(root: &Path, pattern: &str, max_depth: usize) -> Vec<SearchResult> {
    let pattern_lower = pattern.to_lowercase();
    let ignored_dirs = Config::get_ignored_dirs_combined();
    let use_wildcard = has_wildcards(pattern);

    if use_wildcard {
        let mut wild_matches: Vec<SearchResult> = Vec::new();
        for (dir_name, full_path) in walk_dirs(root, max_depth, &ignored_dirs) {
            if matches_wildcard(pattern, &dir_name) {
                wild_matches.push(SearchResult::exact(full_path, dir_name));
            }
        }
        wild_matches.sort_by_cached_key(|a| a.name.to_lowercase());
        return wild_matches;
    }

    let mut exact_matches: Vec<SearchResult> = Vec::new();
    let mut partial_matches: Vec<SearchResult> = Vec::new();
    let mut fuzzy_candidates: Vec<Candidate> = Vec::new();

    for (dir_name, full_path) in walk_dirs(root, max_depth, &ignored_dirs) {
        let dir_name_lower = dir_name.to_lowercase();

        if dir_name_lower == pattern_lower {
            exact_matches.push(SearchResult::exact(full_path, dir_name));
        } else if dir_name_lower.contains(&pattern_lower) {
            partial_matches.push(SearchResult::partial(full_path, dir_name));
        } else {
            fuzzy_candidates.push(Candidate {
                name: dir_name,
                full_path,
            });
        }
    }

    if !exact_matches.is_empty() {
        exact_matches.sort_by_cached_key(|a| a.name.to_lowercase());
        return exact_matches;
    }

    if !partial_matches.is_empty() {
        partial_matches.sort_by_cached_key(|a| a.name.to_lowercase());
        return partial_matches;
    }

    let candidate_names: Vec<String> = fuzzy_candidates.iter().map(|c| c.name.clone()).collect();
    let fuzzy_hits = top_fuzzy_dir_matches(&candidate_names, pattern, MAX_FUZZY_SUGGESTIONS);

    let mut results: Vec<SearchResult> = Vec::new();
    for (hit_name, score) in fuzzy_hits {
        for candidate in &fuzzy_candidates {
            if &candidate.name == hit_name {
                results.push(SearchResult::fuzzy(
                    candidate.full_path.clone(),
                    candidate.name.clone(),
                    score,
                ));
            }
        }
    }

    results
}

/// Search for both files and directories matching a pattern in the current tree.
///
/// Combines results from both `search_files` and `search_directories`, keeping
/// the same tiered matching: exact → partial → fuzzy. If the best tier is exact,
/// only exact file+dir results are returned; if partial, only partial; else fuzzy.
///
/// # Arguments
/// * `root`      - Starting directory (CWD)
/// * `pattern`   - Name or partial name to search for (case-insensitive)
/// * `max_depth` - 0 = current dir only, n = recurse n levels, usize::MAX = unlimited
pub fn search_all(root: &Path, pattern: &str, max_depth: usize) -> Vec<SearchResult> {
    let files = search_files(root, pattern, max_depth);
    let dirs = search_directories(root, pattern, max_depth);

    let has_exact = files.iter().any(|r| r.match_kind == MatchKind::Exact)
        || dirs.iter().any(|r| r.match_kind == MatchKind::Exact);
    let has_partial = files.iter().any(|r| r.match_kind == MatchKind::Partial)
        || dirs.iter().any(|r| r.match_kind == MatchKind::Partial);

    if has_exact {
        let mut results: Vec<SearchResult> = files
            .into_iter()
            .chain(dirs)
            .filter(|r| r.match_kind == MatchKind::Exact)
            .collect();
        results.sort_by_cached_key(|a| a.name.to_lowercase());
        return results;
    }

    if has_partial {
        let mut results: Vec<SearchResult> = files
            .into_iter()
            .chain(dirs)
            .filter(|r| r.match_kind == MatchKind::Partial)
            .collect();
        results.sort_by_cached_key(|a| a.name.to_lowercase());
        return results;
    }

    // Fuzzy — combine all fuzzy results, sorted by score descending
    let mut results: Vec<SearchResult> = files
        .into_iter()
        .chain(dirs)
        .filter(|r| r.match_kind == MatchKind::Fuzzy)
        .collect();
    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    results
}

/// Result of a content search (grep)
#[derive(Debug, Clone)]
pub struct ContentMatch {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub line: String,
}

/// Max file size (bytes) scanned by content search. Larger files are skipped
/// so a broad `gs -d <n>` waits on the spinner instead of OOM/crashing.
pub const MAX_GS_FILE_BYTES: u64 = 8 * 1024 * 1024;
/// Max matches kept per file so one huge generated file can't explode memory.
const MAX_GS_MATCHES_PER_FILE: usize = 2_000;

/// Search file contents for a pattern (case-insensitive).
/// Only searches files that pass the ignore/care filters.
/// Returns matches sorted by file path then line number.
pub fn search_content(root: &Path, pattern: &str, max_depth: usize) -> Vec<ContentMatch> {
    if pattern.is_empty() {
        return Vec::new();
    }
    let pattern_lower = pattern.to_lowercase();
    let ignored_dirs = Config::get_ignored_dirs_combined();
    let fmt_cfg = FormatConfig::from_both();

    let paths: Vec<PathBuf> = walk_files(root, max_depth, &ignored_dirs, &fmt_cfg)
        .map(|(_, p)| p)
        .collect();

    let mut results: Vec<ContentMatch> = paths
        .par_iter()
        .filter_map(|path| {
            // Skip huge files before reading: prevents OOM on broad searches.
            if let Ok(md) = std::fs::metadata(path) {
                if md.len() > MAX_GS_FILE_BYTES {
                    return None;
                }
            }
            let content = std::fs::read_to_string(path).ok()?;
            let mut matches: Vec<ContentMatch> = Vec::new();
            for (i, line) in content.lines().enumerate() {
                if matches.len() >= MAX_GS_MATCHES_PER_FILE {
                    break;
                }
                // Cheap length pre-check before allocating lowercase string.
                if line.len() < pattern.len() {
                    continue;
                }
                if line.to_lowercase().contains(&pattern_lower) {
                    matches.push(ContentMatch {
                        file_path: path.clone(),
                        line_number: i + 1,
                        line: line.to_string(),
                    });
                }
            }
            if matches.is_empty() {
                None
            } else {
                Some(matches)
            }
        })
        .flatten()
        .collect();

    results.sort_by(|a, b| {
        a.file_path
            .cmp(&b.file_path)
            .then_with(|| a.line_number.cmp(&b.line_number))
    });
    results
}

/// Search file contents for a pattern (case-insensitive) within an
/// explicit file list (already glob-filtered by the caller, e.g. `qur`).
/// Same guards as [`search_content`]: skips huge files and non-UTF8 files.
/// Returns matches sorted by file path then line number.
pub fn search_content_in_files(files: &[PathBuf], pattern: &str) -> Vec<ContentMatch> {
    if pattern.is_empty() {
        return Vec::new();
    }
    let pattern_lower = pattern.to_lowercase();
    let mut results: Vec<ContentMatch> = files
        .par_iter()
        .filter_map(|path| {
            if let Ok(md) = std::fs::metadata(path) {
                if md.len() > MAX_GS_FILE_BYTES {
                    return None;
                }
            }
            let content = std::fs::read_to_string(path).ok()?;
            let mut matches: Vec<ContentMatch> = Vec::new();
            for (i, line) in content.lines().enumerate() {
                if matches.len() >= MAX_GS_MATCHES_PER_FILE {
                    break;
                }
                if line.len() < pattern.len() {
                    continue;
                }
                if line.to_lowercase().contains(&pattern_lower) {
                    matches.push(ContentMatch {
                        file_path: path.clone(),
                        line_number: i + 1,
                        line: line.to_string(),
                    });
                }
            }
            if matches.is_empty() {
                None
            } else {
                Some(matches)
            }
        })
        .flatten()
        .collect();

    results.sort_by(|a, b| {
        a.file_path
            .cmp(&b.file_path)
            .then_with(|| a.line_number.cmp(&b.line_number))
    });
    results
}

/// Display path for `gs` results: relative to the search root
/// (`sub_dir/file.ext`), never the full path / CWD prefix.
/// Falls back to file name when the path is not under `root`.
pub fn display_content_path(path: &Path, root: &Path) -> String {
    let rel = path
        .strip_prefix(root)
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|_| {
            path.file_name()
                .map(PathBuf::from)
                .unwrap_or_else(|| path.to_path_buf())
        });
    let s = rel.to_string_lossy().replace('\\', "/");
    let s = if s.is_empty() {
        path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| strip_unc_prefix(path))
    } else {
        s
    };
    // Strip any lingering UNC prefix (Windows) without re-adding absoluteness.
    s.strip_prefix(r"\\?\")
        .map(|x| x.to_string())
        .unwrap_or(s)
}

/// Highlight every (case-insensitive) occurrence of `pattern` in `line` red.
/// Uses explicit ANSI codes so output is red even when piped/redirected.
pub fn highlight_pattern(line: &str, pattern: &str) -> String {
    if pattern.is_empty() {
        return line.to_string();
    }
    const RED: &str = "\x1b[31m";
    const RESET: &str = "\x1b[0m";
    let lower_line = line.to_lowercase();
    let lower_pat = pattern.to_lowercase();
    let mut out = String::with_capacity(line.len() + 16);
    let mut i = 0usize;
    while i < line.len() {
        if lower_line[i..].starts_with(&lower_pat) {
            let end = (i + pattern.len()).min(line.len());
            // Keep char-boundary safety for non-ASCII text.
            let mut e = end;
            while e < line.len() && !line.is_char_boundary(e) {
                e += 1;
            }
            let mut s = i;
            while s < e && !line.is_char_boundary(s) {
                s += 1;
            }
            out.push_str(RED);
            out.push_str(&line[s..e]);
            out.push_str(RESET);
            i = e;
        } else {
            let ch = line[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    out
}

fn truncate_for_display(line: &str, max_chars: usize) -> String {
    let t = line.trim();
    if t.chars().count() > max_chars {
        format!("{}…", t.chars().take(max_chars).collect::<String>())
    } else {
        t.to_string()
    }
}

/// Format content search results for display.
///
/// `root` is used to render relative paths (`sub/file.ext:line:content`).
/// `total_found` is the count before any `-l/--limit` truncation so the
/// header can report `showing X of Y`.
pub fn format_content_results(
    results: &[ContentMatch],
    pattern: &str,
    max_depth: usize,
    root: &Path,
    total_found: usize,
) -> String {
    let depth_label = if max_depth == usize::MAX {
        "unlimited".to_string()
    } else {
        max_depth.to_string()
    };

    let mut output = String::new();
    output.push_str(&format!(
        "\n🔍 Searching file contents for \"{}\" (max depth: {})...\n\n",
        pattern, depth_label
    ));

    if results.is_empty() {
        output.push_str("  No matches found.\n");
        return output;
    }

    if total_found > results.len() {
        output.push_str(&format!(
            "  Content matches (showing {} of {}):\n",
            results.len(),
            total_found
        ));
    } else {
        output.push_str(&format!("  Content matches ({}):\n", total_found));
    }
    for (i, m) in results.iter().enumerate() {
        let path_str = display_content_path(&m.file_path, root);
        let truncated = truncate_for_display(&m.line, 120);
        let highlighted = highlight_pattern(&truncated, pattern);
        output.push_str(&format!(
            "  {}. {}:{}: {}\n",
            i + 1,
            path_str,
            m.line_number,
            highlighted
        ));
    }

    output
}

// ============================================================================
// Display helpers
// ============================================================================

/// Strip Windows extended-path prefix (\\?\) for clean terminal output.
pub fn display_search_path(path: &Path) -> String {
    strip_unc_prefix(path)
}

/// Format search results for display (used by shell.rs).
pub fn format_search_results(
    results: &[SearchResult],
    pattern: &str,
    max_depth: usize,
    is_files: bool,
) -> String {
    let item_type = if is_files { "files" } else { "directories" };
    let depth_label = if max_depth == usize::MAX {
        "unlimited".to_string()
    } else {
        max_depth.to_string()
    };

    let mut output = String::new();
    output.push_str(&format!(
        "\n🔍 Searching {} for \"{}\" (max depth: {})...\n\n",
        item_type, pattern, depth_label
    ));

    if results.is_empty() {
        output.push_str("  No matches found.\n");
        return output;
    }

    let has_exact = results.iter().any(|r| r.match_kind == MatchKind::Exact);
    let has_partial = results.iter().any(|r| r.match_kind == MatchKind::Partial);

    if has_exact {
        output.push_str(&format!("  Exact matches ({}):\n", results.len()));
        for (i, result) in results.iter().enumerate() {
            output.push_str(&format!(
                "  {}. {}\n",
                i + 1,
                display_search_path(&result.full_path)
            ));
        }
    } else if has_partial {
        output.push_str(&format!("  Partial matches ({}):\n", results.len()));
        for (i, result) in results.iter().enumerate() {
            output.push_str(&format!(
                "  {}. {}\n",
                i + 1,
                display_search_path(&result.full_path)
            ));
        }
    } else {
        output.push_str("  No exact match found.\n");
        output.push_str("  Did you mean?\n");
        for (i, result) in results.iter().enumerate() {
            output.push_str(&format!(
                "  {}. {} ({:.0}% match)\n",
                i + 1,
                display_search_path(&result.full_path),
                result.score * 100.0
            ));
        }
    }

    output
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_exact_file_match() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("main.c"), "").unwrap();

        let results = search_files(dir.path(), "main.c", 0);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].match_kind, MatchKind::Exact);
        assert_eq!(results[0].name, "main.c");
    }

    #[test]
    fn test_partial_file_match() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("main_helper.c"), "").unwrap();
        std::fs::write(dir.path().join("main_test.c"), "").unwrap();

        // "main" is not an exact match but both files contain "main"
        let results = search_files(dir.path(), "main", 0);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.match_kind == MatchKind::Partial));
    }

    #[test]
    fn test_fuzzy_file_match() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("main.c"), "").unwrap();

        // "mian.c" has no exact or partial match, but fuzzy should find "main.c"
        let results = search_files(dir.path(), "mian.c", 0);
        assert!(!results.is_empty());
        assert_eq!(results[0].match_kind, MatchKind::Fuzzy);
        assert_eq!(results[0].name, "main.c");
        assert!(results[0].score >= 0.85);
    }

    #[test]
    fn test_exact_beats_partial() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("main.c"), "").unwrap();
        std::fs::write(dir.path().join("main_helper.c"), "").unwrap();

        // "main.c" is exact — should only return exact, not partial "main_helper.c"
        let results = search_files(dir.path(), "main.c", 0);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].match_kind, MatchKind::Exact);
    }

    #[test]
    fn test_partial_beats_fuzzy() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("main_helper.c"), "").unwrap();
        std::fs::write(dir.path().join("mian.c"), "").unwrap(); // fuzzy candidate

        // "main" matches "main_helper.c" partially — fuzzy "mian.c" should not appear
        let results = search_files(dir.path(), "main", 0);
        assert!(results.iter().all(|r| r.match_kind == MatchKind::Partial));
        assert!(results.iter().any(|r| r.name == "main_helper.c"));
        assert!(results.iter().all(|r| r.name != "mian.c"));
    }

    #[test]
    fn test_exact_dir_match() {
        let dir = tempdir().unwrap();
        std::fs::create_dir(dir.path().join("src")).unwrap();

        let results = search_directories(dir.path(), "src", 1);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].match_kind, MatchKind::Exact);
        assert_eq!(results[0].name, "src");
    }

    #[test]
    fn test_partial_dir_match() {
        let dir = tempdir().unwrap();
        std::fs::create_dir(dir.path().join("src_old")).unwrap();
        std::fs::create_dir(dir.path().join("src_new")).unwrap();

        let results = search_directories(dir.path(), "src", 1);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.match_kind == MatchKind::Partial));
    }

    #[test]
    fn test_depth_zero_current_dir_only() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("level1");
        std::fs::create_dir(&subdir).unwrap();
        std::fs::write(subdir.join("deep.c"), "").unwrap();
        std::fs::write(dir.path().join("shallow.c"), "").unwrap();

        // depth 0 = current dir only
        let results = search_files(dir.path(), "shallow.c", 0);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "shallow.c");

        // "deep.c" is one level down — not found at depth 0
        let results_deep = search_files(dir.path(), "deep.c", 0);
        assert!(results_deep.is_empty());

        // depth 1 finds it
        let results_depth1 = search_files(dir.path(), "deep.c", 1);
        assert_eq!(results_depth1.len(), 1);
    }

    #[test]
    fn test_respects_ignored_dirs() {
        let dir = tempdir().unwrap();
        let target = dir.path().join("target");
        std::fs::create_dir(&target).unwrap();
        std::fs::write(target.join("build.rs"), "").unwrap();

        Config::global_add_ignored_dir("target");

        let results = search_files(dir.path(), "build.rs", 2);
        assert!(results.is_empty());
    }

    #[test]
    fn test_duplicate_names_different_dirs() {
        let dir = tempdir().unwrap();
        let a = dir.path().join("a");
        let b = dir.path().join("b");
        std::fs::create_dir(&a).unwrap();
        std::fs::create_dir(&b).unwrap();
        std::fs::write(a.join("main.c"), "").unwrap();
        std::fs::write(b.join("main.c"), "").unwrap();

        // Both files named "main.c" should be returned as exact matches
        let results = search_files(dir.path(), "main.c", 1);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.match_kind == MatchKind::Exact));
    }

    #[test]
    fn test_display_path_strips_windows_prefix() {
        #[cfg(windows)]
        {
            let path = Path::new(r"\\?\C:\Users\ntc\main.c");
            assert_eq!(display_search_path(path), r"C:\Users\ntc\main.c");
        }
        #[cfg(not(windows))]
        {
            let path = Path::new("/home/user/main.c");
            assert_eq!(display_search_path(path), "/home/user/main.c");
        }
    }

    #[test]
    fn test_format_results_exact() {
        let dir = tempdir().unwrap();
        let results = vec![SearchResult::exact(
            dir.path().join("main.c"),
            "main.c".to_string(),
        )];
        let output = format_search_results(&results, "main.c", 2, true);
        assert!(output.contains("Exact matches"));
        assert!(output.contains("main.c"));
        assert!(output.contains("files"));
    }

    #[test]
    fn test_format_results_partial() {
        let dir = tempdir().unwrap();
        let results = vec![SearchResult::partial(
            dir.path().join("main_helper.c"),
            "main_helper.c".to_string(),
        )];
        let output = format_search_results(&results, "main", 2, true);
        assert!(output.contains("Partial matches"));
        assert!(output.contains("main_helper.c"));
    }

    #[test]
    fn test_format_results_fuzzy() {
        let dir = tempdir().unwrap();
        let results = vec![SearchResult::fuzzy(
            dir.path().join("main.c"),
            "main.c".to_string(),
            0.92,
        )];
        let output = format_search_results(&results, "mian.c", 2, true);
        assert!(output.contains("Did you mean?"));
        assert!(output.contains("92% match"));
    }

    #[test]
    fn test_format_results_empty() {
        let output = format_search_results(&[], "ghost.c", 2, true);
        assert!(output.contains("No matches found"));
    }
}
