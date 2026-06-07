// src/search.rs
// File and directory search for ntc
// Version: v1.8.1
// Used by: fs (file search) and ds (directory search) commands
// Cross-platform: Windows, Linux, macOS

use crate::config::Config;
use crate::filetype::FormatConfig;
use crate::fuzzy::{top_fuzzy_matches, top_fuzzy_dir_matches, MAX_FUZZY_SUGGESTIONS};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
        Self { full_path, name, match_kind: MatchKind::Exact, score: 1.0 }
    }

    pub fn partial(full_path: PathBuf, name: String) -> Self {
        Self { full_path, name, match_kind: MatchKind::Partial, score: 1.0 }
    }

    pub fn fuzzy(full_path: PathBuf, name: String, score: f64) -> Self {
        Self { full_path, name, match_kind: MatchKind::Fuzzy, score }
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
fn walk_files<'a>(
    root: &'a Path,
    max_depth: usize,
    ignored_dirs: &'a std::collections::HashSet<String>,
    fmt_cfg: &'a FormatConfig,
) -> impl Iterator<Item = (String, PathBuf)> + 'a {
    let walkdir_depth = max_depth.saturating_add(1);

    WalkDir::new(root)
        .max_depth(walkdir_depth)
        .into_iter()
        .filter_entry(move |e| {
            if e.depth() == 0 {
                return true;
            }
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy().to_lowercase();
                return !ignored_dirs.contains(&name);
            }
            true
        })
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
        .filter_entry(move |e| {
            if e.depth() == 0 {
                return true;
            }
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy().to_lowercase();
                return !ignored_dirs.contains(&name);
            }
            true
        })
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
    let ignored_dirs = Config::global_get_ignored_dirs();
    let fmt_cfg = FormatConfig::from_global();

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
            fuzzy_candidates.push(Candidate { name: file_name, full_path });
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
    let ignored_dirs = Config::global_get_ignored_dirs();

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
            fuzzy_candidates.push(Candidate { name: dir_name, full_path });
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
        let mut results: Vec<SearchResult> = files.into_iter()
            .chain(dirs.into_iter())
            .filter(|r| r.match_kind == MatchKind::Exact)
            .collect();
        results.sort_by_cached_key(|a| a.name.to_lowercase());
        return results;
    }

    if has_partial {
        let mut results: Vec<SearchResult> = files.into_iter()
            .chain(dirs.into_iter())
            .filter(|r| r.match_kind == MatchKind::Partial)
            .collect();
        results.sort_by_cached_key(|a| a.name.to_lowercase());
        return results;
    }

    // Fuzzy — combine all fuzzy results, sorted by score descending
    let mut results: Vec<SearchResult> = files.into_iter()
        .chain(dirs.into_iter())
        .filter(|r| r.match_kind == MatchKind::Fuzzy)
        .collect();
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    results
}

// ============================================================================
// Display helpers
// ============================================================================

/// Strip Windows extended-path prefix (\\?\) for clean terminal output.
pub fn display_search_path(path: &Path) -> String {
    let s = path.to_string_lossy();
    #[cfg(windows)]
    if let Some(stripped) = s.strip_prefix(r"\\?\") {
        return stripped.to_string();
    }
    s.to_string()
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

    let has_exact   = results.iter().any(|r| r.match_kind == MatchKind::Exact);
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
        let results = vec![
            SearchResult::exact(dir.path().join("main.c"), "main.c".to_string()),
        ];
        let output = format_search_results(&results, "main.c", 2, true);
        assert!(output.contains("Exact matches"));
        assert!(output.contains("main.c"));
        assert!(output.contains("files"));
    }

    #[test]
    fn test_format_results_partial() {
        let dir = tempdir().unwrap();
        let results = vec![
            SearchResult::partial(dir.path().join("main_helper.c"), "main_helper.c".to_string()),
        ];
        let output = format_search_results(&results, "main", 2, true);
        assert!(output.contains("Partial matches"));
        assert!(output.contains("main_helper.c"));
    }

    #[test]
    fn test_format_results_fuzzy() {
        let dir = tempdir().unwrap();
        let results = vec![
            SearchResult::fuzzy(dir.path().join("main.c"), "main.c".to_string(), 0.92),
        ];
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