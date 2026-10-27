use crate::config::Config;
use crate::filetype::{is_supported_format_with_config, make_ignored_dir_filter, FormatConfig};
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;
use walkdir::WalkDir;

type TreeCacheKey = (String, usize, bool, bool, bool);
type TreeCacheValue = (Instant, TreeNode);
type TreeCache = std::sync::LazyLock<Mutex<HashMap<TreeCacheKey, TreeCacheValue>>>;

// Tree output cache: key = (root_path, max_depth, include_files, show_sizes, care_sizes)
// Value = (timestamp, cached tree)
static TREE_CACHE: TreeCache = std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

/// Per-file size cache. Populated by calculate_total_size/calculate_dir_size,
/// read by compute_tree_sizes to avoid redundant fs::metadata calls between
/// chained commands like `size --care && view --size --care`.
static FILE_SIZE_CACHE: std::sync::LazyLock<Mutex<HashMap<PathBuf, u64>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

fn get_cached_size(path: &Path) -> Option<u64> {
    FILE_SIZE_CACHE.lock().ok()?.get(path).copied()
}

fn set_cached_size(path: PathBuf, size: u64) {
    if let Ok(mut cache) = FILE_SIZE_CACHE.lock() {
        cache.insert(path, size);
    }
}

/// Clear the file-size cache.
pub fn invalidate_size_cache() {
    if let Ok(mut cache) = FILE_SIZE_CACHE.lock() {
        cache.clear();
    }
}

/// Invalidate the tree cache. Call this when the filesystem is known to have changed.
pub fn invalidate_tree_cache() {
    if let Ok(mut cache) = TREE_CACHE.lock() {
        cache.clear();
    }
    invalidate_size_cache();
}

/// Get a cached tree if available and fresh (TTL: 5 seconds).
fn get_cached_tree(key: &(String, usize, bool, bool, bool)) -> Option<TreeNode> {
    if let Ok(cache) = TREE_CACHE.lock() {
        if let Some((ts, tree)) = cache.get(key) {
            if ts.elapsed().as_secs() < 5 {
                return Some(tree.clone());
            }
        }
    }
    None
}

/// Store a tree in the cache.
fn set_cached_tree(key: (String, usize, bool, bool, bool), tree: TreeNode) {
    if let Ok(mut cache) = TREE_CACHE.lock() {
        cache.insert(key, (Instant::now(), tree));
    }
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub is_ignored: bool,
    pub is_supported: Option<bool>,
    pub children: Vec<TreeNode>,
    pub depth: usize,
    pub size: Option<u64>,
}

/// True when a file is excluded by the effective ignore lists (filename or
/// extension). Used to annotate ignored files instead of dropping them.
fn file_matches_ignore(
    name: &str,
    path: &Path,
    ignored_files: &HashSet<String>,
    ignored_extensions: &HashSet<String>,
) -> bool {
    ignored_files.contains(name)
        || path
            .extension()
            .map(|ext| ignored_extensions.contains(&ext.to_string_lossy().to_lowercase()))
            .unwrap_or(false)
}

/// Generate a hierarchical tree.
/// Previously this used a `HashMap<PathBuf, *mut TreeNode>` to build the tree
/// in a single pass. That caused undefined behaviour: pushing to a child's
/// `Vec<TreeNode>` could reallocate it, invalidating every raw pointer already
/// stored in the map.
///
/// The fix collects all entries into a flat `Vec` of `FlatNode` (holding a
/// parent index instead of a raw pointer), then assembles the owned `TreeNode`
/// tree bottom-up so no pointers are ever held across a Vec mutation.
pub fn generate_tree(
    root_path: &str,
    max_depth_override: Option<usize>,
    include_files: bool,
    pb: Option<&ProgressBar>,
) -> TreeNode {
    let max_depth = max_depth_override.unwrap_or(Config::global_get_max_depth());
    let root = Path::new(root_path);

    // Check tree cache
    let cache_key = (
        root_path.to_string(),
        max_depth,
        include_files,
        false,
        false,
    );
    if let Some(cached) = get_cached_tree(&cache_key) {
        return cached;
    }

    let root_name = root
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let igcare = Config::effective_igcare();
    // Fetch format config once for the entire walk — avoids 4 lock acquisitions per file.
    let fmt_cfg = FormatConfig::from_both();
    struct FlatNode {
        name: String,
        path: String,
        is_dir: bool,
        is_ignored: bool,
        is_supported: Option<bool>,
        depth: usize,
        parent_idx: usize, // index into `flat` (0 = root)
    }

    // Index 0 is the root sentinel.
    let mut flat: Vec<FlatNode> = Vec::new();
    flat.push(FlatNode {
        name: root_name.clone(),
        path: root_path.to_string(),
        is_dir: true,
        is_ignored: false,
        is_supported: None,
        depth: 0,
        parent_idx: 0,
    });

    // Map from canonical path → index in `flat`, so we can locate a parent.
    let mut path_to_idx: HashMap<PathBuf, usize> = HashMap::new();
    path_to_idx.insert(root.to_path_buf(), 0);

    let walker = WalkDir::new(root)
        .max_depth(max_depth)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        .into_iter()
        .filter_entry(make_ignored_dir_filter(&igcare.ignored_directory_names));

    for entry in walker {
        if let Some(pb) = pb {
            pb.inc(1);
        }
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        if entry.path() == root {
            continue;
        }

        let is_dir = entry.file_type().is_dir();
        if !is_dir && !include_files {
            continue;
        }

        let parent_path = match entry.path().parent() {
            Some(p) => p.to_path_buf(),
            None => continue,
        };
        let parent_idx = match path_to_idx.get(&parent_path) {
            Some(&idx) => idx,
            None => continue, // parent was filtered out
        };
        let depth = entry.depth();
        let name = entry.file_name().to_string_lossy().to_string();
        let path_str = entry.path().to_string_lossy().to_string();
        let is_supported = if is_dir {
            None
        } else {
            Some(is_supported_format_with_config(entry.path(), &fmt_cfg))
        };
        let is_ignored = !is_dir
            && file_matches_ignore(
                &name,
                entry.path(),
                &igcare.ignored_files,
                &igcare.ignored_extensions,
            );

        let idx = flat.len();
        flat.push(FlatNode {
            name: name.clone(),
            path: path_str.clone(),
            is_dir,
            is_ignored,
            is_supported,
            depth,
            parent_idx,
        });

        if is_dir {
            path_to_idx.insert(entry.path().to_path_buf(), idx);
        }
    }

    // ---- Phase 2: assemble owned TreeNode tree from the flat list ----
    //
    // Collect parent_indices BEFORE consuming `flat` so we can use the
    // parent_idx field that was stored during Phase 1. This avoids the
    // previous redundant path→index re-lookup and eliminates the dead_code
    // warning (parent_idx was written but never read after into_iter consumed
    // the vec).
    let parent_indices: Vec<usize> = flat.iter().map(|f| f.parent_idx).collect();
    let n = flat.len();

    // Convert each FlatNode into a TreeNode (children empty, size None for now).
    let mut nodes: Vec<Option<TreeNode>> = flat
        .into_iter()
        .map(|f| {
            Some(TreeNode {
                name: f.name,
                path: f.path,
                is_dir: f.is_dir,
                is_ignored: f.is_ignored,
                is_supported: f.is_supported,
                children: Vec::new(),
                depth: f.depth,
                size: None,
            })
        })
        .collect();

    // Move children into parents in reverse order (leaves first), so a parent
    // is never consumed before all its children have been moved into it.
    for i in (1..n).rev() {
        let Some(child) = nodes[i].take() else {
            continue;
        };
        let Some(parent) = nodes[parent_indices[i]].as_mut() else {
            continue;
        };
        parent.children.push(child);
    }
    let Some(mut root_node) = nodes[0].take() else {
        return TreeNode {
            name: String::new(),
            path: String::new(),
            is_dir: false,
            is_ignored: false,
            is_supported: None,
            depth: 0,
            size: None,
            children: vec![],
        };
    };

    // ---- Phase 3: sort children (dirs, files, ignored dirs, ignored files) ----
    fn sort_children(nodes: &mut [TreeNode]) {
        nodes.par_sort_by(|a, b| {
            tree_group_rank(a)
                .cmp(&tree_group_rank(b))
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
        for child in nodes.iter_mut() {
            sort_children(&mut child.children);
        }
    }

    // Inject ignored directories at root depth so users can see they exist
    if let Ok(entries) = std::fs::read_dir(Path::new(root_path)) {
        for entry in entries.filter_map(|e| e.ok()) {
            if let Ok(ft) = entry.file_type() {
                if ft.is_dir() {
                    let name_lower = entry.file_name().to_string_lossy().to_lowercase();
                    if igcare.ignored_directory_names.contains(&name_lower) {
                        let name_orig = entry.file_name().to_string_lossy().to_string();
                        root_node.children.push(TreeNode {
                            name: name_orig,
                            path: entry.path().to_string_lossy().to_string(),
                            is_dir: true,
                            is_ignored: true,
                            is_supported: None,
                            children: Vec::new(),
                            depth: 1,
                            size: None,
                        });
                    }
                }
            }
        }
    }

    // Sort after injection so the groups stay ordered as dirs, files,
    // ignored dirs, ignored files.
    sort_children(&mut root_node.children);

    // Cache the result for fast repeated lookups
    set_cached_tree(cache_key, root_node.clone());

    // Return the root node of the assembled tree
    root_node
}

/// Display group rank for tree children: normal directories, normal files,
/// ignored directories, ignored files.
fn tree_group_rank(node: &TreeNode) -> u8 {
    match (node.is_ignored, node.is_dir) {
        (false, true) => 0,
        (false, false) => 1,
        (true, true) => 2,
        (true, false) => 3,
    }
}

/// Sort every level by display group (dirs, files, ignored dirs, ignored
/// files). When `dirs_by_size` / `files_by_size` is set, entries of that
/// kind sort largest size first (children without a computed size count as
/// 0); otherwise they stay alphabetical. Used by the automatic navigation
/// tree when `config size` is ON.
pub fn sort_tree_by_size_groups(root: &mut TreeNode, dirs_by_size: bool, files_by_size: bool) {
    fn sort_level(children: &mut [TreeNode], dirs_by_size: bool, files_by_size: bool) {
        children.sort_by(|a, b| {
            let rank = tree_group_rank(a).cmp(&tree_group_rank(b));
            if rank != std::cmp::Ordering::Equal {
                return rank;
            }
            let by_size = if a.is_dir {
                dirs_by_size
            } else {
                files_by_size
            };
            let size = if by_size {
                b.size.unwrap_or(0).cmp(&a.size.unwrap_or(0))
            } else {
                std::cmp::Ordering::Equal
            };
            size.then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
        for child in children.iter_mut() {
            sort_level(&mut child.children, dirs_by_size, files_by_size);
        }
    }
    sort_level(&mut root.children, dirs_by_size, files_by_size);
}

/// Prune a tree in place so only files matching `exts` (see
/// `filetype::matches_any_extension`) remain. Directories survive only when
/// they still contain at least one kept descendant, so empty dirs vanish
/// from filtered views. The root itself is always kept. Returns the number
/// of matched files.
pub fn prune_tree_by_exts(root: &mut TreeNode, exts: &[String]) -> usize {
    fn keep(node: &mut TreeNode, exts: &[String]) -> (bool, usize) {
        if !node.is_dir {
            let matched = crate::filetype::matches_any_extension(&node.name, exts);
            return (matched, usize::from(matched));
        }
        let mut count = 0usize;
        node.children.retain_mut(|child| {
            let (keep_child, n) = keep(child, exts);
            count += n;
            keep_child
        });
        (!node.children.is_empty(), count)
    }

    // Apply to children only: the root is always kept even when empty.
    let mut count = 0usize;
    root.children.retain_mut(|child| {
        let (keep_child, n) = keep(child, exts);
        count += n;
        keep_child
    });
    count
}

/// Size a pruned (extension-filtered) tree bottom-up: files from metadata,
/// directories as the sum of their kept descendants (NOT the on-disk total,
/// which would include filtered-out files).
pub fn compute_filtered_sizes(node: &mut TreeNode) -> u64 {
    if !node.is_dir {
        let size = std::fs::metadata(&node.path).map(|m| m.len()).unwrap_or(0);
        node.size = Some(size);
        return size;
    }
    let total: u64 = node.children.iter_mut().map(compute_filtered_sizes).sum();
    node.size = Some(total);
    total
}

// =========================================================================
// Pre-computed tree sizing (replaces repeated calculate_dir_size calls)
// =========================================================================

/// Compute sizes for every node in the tree.
/// Each directory gets its TRUE recursive total size (walks all descendants).
/// Files get their individual size from metadata.
/// When `care` is true, all exclude rules are ignored (sizes include everything).
pub fn compute_tree_sizes(node: &mut TreeNode, pb: Option<&ProgressBar>, care: bool) {
    // Recurse into children first (bottom-up), parallel across siblings
    node.children.par_iter_mut().for_each(|child| {
        if child.is_dir {
            compute_tree_sizes(child, pb, care);
        } else {
            let path = Path::new(&child.path);
            child.size =
                get_cached_size(path).or_else(|| std::fs::metadata(path).map(|m| m.len()).ok());
            if let Some(s) = child.size {
                set_cached_size(PathBuf::from(&child.path), s);
            }
        }
    });

    if !node.is_dir {
        return;
    }

    let total = if care {
        calculate_total_size(Path::new(&node.path))
    } else {
        calculate_dir_size(Path::new(&node.path))
    };
    node.size = Some(total);

    if let Some(pb) = pb {
        pb.inc(1);
    }
}

// =========================================================================
// Legacy helpers (still used by the `size` command and CLI --size)
// =========================================================================

/// Count files (respects ignored dirs)
/// Count files (respects ignored dirs)
pub fn count_files(path: &Path) -> u64 {
    let ignored_dirs = Config::get_ignored_dirs_combined();
    walk_files(path, &ignored_dirs).len() as u64
}

fn walk_files(path: &Path, ignored_dirs: &HashSet<String>) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
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
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Calculate total size of a directory (parallel with rayon)
pub fn calculate_dir_size(path: &Path) -> u64 {
    let ignored_dirs = Config::get_ignored_dirs_combined();
    walk_files(path, &ignored_dirs)
        .par_iter()
        .map(|p| {
            if let Some(size) = get_cached_size(p) {
                return size;
            }
            let size = std::fs::metadata(p).map(|m| m.len()).unwrap_or(0);
            set_cached_size(p.clone(), size);
            size
        })
        .sum()
}

fn walk_files_all(path: &Path) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Calculate total size ignoring all exclude rules (cares about everything)
pub fn calculate_total_size(path: &Path) -> u64 {
    walk_files_all(path)
        .par_iter()
        .map(|p| {
            if let Some(size) = get_cached_size(p) {
                return size;
            }
            let size = std::fs::metadata(p).map(|m| m.len()).unwrap_or(0);
            set_cached_size(p.clone(), size);
            size
        })
        .sum()
}

/// Size of a single file (uses the shared per-file size cache).
pub fn calculate_file_size(path: &Path) -> u64 {
    if let Some(size) = get_cached_size(path) {
        return size;
    }
    let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    set_cached_size(path.to_path_buf(), size);
    size
}

/// Size of any `size`-command target: file → its length,
/// directory → walked total (`care` selects ignore-rules vs everything).
pub fn calculate_target_size(path: &Path, care: bool) -> u64 {
    if path.is_file() {
        return calculate_file_size(path);
    }
    if care {
        calculate_total_size(path)
    } else {
        calculate_dir_size(path)
    }
}

/// Calculate total size with progress bar (parallel)
pub fn calculate_dir_size_with_progress(path: &Path, pb: &ProgressBar) -> u64 {
    let ignored_dirs = Config::get_ignored_dirs_combined();
    walk_files(path, &ignored_dirs)
        .par_iter()
        .map(|p| {
            pb.inc(1);
            if let Some(size) = get_cached_size(p) {
                return size;
            }
            let size = std::fs::metadata(p).map(|m| m.len()).unwrap_or(0);
            set_cached_size(p.clone(), size);
            size
        })
        .sum()
}

/// Convert bytes to human-readable string
/// Sort key for `view --sort`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeSortKey {
    /// Largest first (ties: name A–Z).
    Size,
    /// Name A–Z (today's default).
    Char,
    /// Extension A–Z, then name A–Z.
    Ext,
}

/// Parse a minimum-size value for `view -s/-fs/-ds <size>`.
/// Case-insensitive `B/KB/MB/GB/TB/PB` (1024-based, matching
/// [`human_readable_size`]); fractions ok (`1.5MB`); a bare number
/// means bytes (`1024`). Returns bytes, or `None` when invalid.
pub fn parse_size_value(raw: &str) -> Option<u64> {
    let s = raw.trim();
    if s.is_empty() {
        return None;
    }
    const UNITS: &[(&str, u64)] = &[
        ("PB", 1024 * 1024 * 1024 * 1024 * 1024),
        ("TB", 1024 * 1024 * 1024 * 1024),
        ("GB", 1024 * 1024 * 1024),
        ("MB", 1024 * 1024),
        ("KB", 1024),
        ("B", 1),
    ];
    let upper = s.to_uppercase();
    for (suffix, mult) in UNITS {
        if let Some(num) = upper.strip_suffix(suffix) {
            let num = num.trim();
            if num.is_empty() {
                return None;
            }
            let val: f64 = num.parse().ok()?;
            if !val.is_finite() || val < 0.0 {
                return None;
            }
            let bytes = val * (*mult as f64);
            if bytes > u64::MAX as f64 {
                return None;
            }
            return Some(bytes as u64);
        }
    }
    // Bare number = bytes (integers only, like `--head` counts).
    let val: u64 = s.parse().ok()?;
    Some(val)
}

/// Prune a tree by minimum sizes: files below `file_min` are dropped,
/// directories below `dir_min` (by their computed `size`) are pruned.
/// Directories left empty by pruning vanish (same rule as
/// [`prune_tree_by_exts`]); the root is always kept. Sizes must already
/// be computed (see [`compute_tree_sizes`]/[`compute_filtered_sizes`).
/// Returns (kept_files, kept_dirs).
pub fn prune_tree_by_min_size(
    root: &mut TreeNode,
    file_min: Option<u64>,
    dir_min: Option<u64>,
) -> (usize, usize) {
    fn keep(
        node: &mut TreeNode,
        file_min: Option<u64>,
        dir_min: Option<u64>,
    ) -> (bool, usize, usize) {
        if !node.is_dir {
            let ok = match file_min {
                Some(min) => node.size.unwrap_or(0) >= min,
                None => true,
            };
            return (ok, usize::from(ok), 0);
        }
        let mut files = 0usize;
        let mut dirs = 0usize;
        node.children.retain_mut(|child| {
            let (keep_child, f, d) = keep(child, file_min, dir_min);
            files += f;
            dirs += d;
            keep_child
        });
        // Directory threshold applies to non-root nodes with a computed size.
        if node.depth > 0 {
            if let Some(min) = dir_min {
                if node.size.unwrap_or(0) < min {
                    return (false, 0, 0);
                }
            }
        }
        if node.children.is_empty() && (file_min.is_some() || dir_min.is_some()) {
            return (false, 0, 0);
        }
        (true, files, dirs + 1)
    }

    let mut files = 0usize;
    let mut dirs = 0usize;
    root.children.retain_mut(|child| {
        let (keep_child, f, d) = keep(child, file_min, dir_min);
        files += f;
        dirs += d;
        keep_child
    });
    (files, dirs)
}

/// Keep only files or only directories in a rendered tree.
/// `FilesOnly` drops every directory node (including nested files they
/// contain); `DirsOnly` drops every file node. The root is always kept.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TreeEntryFilter {
    #[default]
    All,
    FilesOnly,
    DirsOnly,
}

/// Prune `root` to files-only or dirs-only. Returns the number of kept
/// non-root entries. Exact-node semantics: nested entries under a removed
/// directory are removed with it.
pub fn prune_tree_by_entry_filter(root: &mut TreeNode, filter: TreeEntryFilter) -> usize {
    match filter {
        TreeEntryFilter::All => 0,
        TreeEntryFilter::FilesOnly => {
            root.children.retain(|c| !c.is_dir);
            root.children.len()
        }
        TreeEntryFilter::DirsOnly => {
            fn keep_dirs(node: &mut TreeNode) -> usize {
                let mut dirs = 0usize;
                node.children.retain_mut(|child| {
                    if child.is_dir {
                        dirs += 1 + keep_dirs(child);
                        true
                    } else {
                        false
                    }
                });
                dirs
            }
            keep_dirs(root)
        }
    }
}

/// Recursively sort a tree's children by `key`, keeping directories
/// grouped before files (today's layout rule).
pub fn sort_tree_by_key(root: &mut TreeNode, key: TreeSortKey) {
    fn ext_of(name: &str) -> String {
        match name.rfind('.') {
            Some(i) if i + 1 < name.len() => name[i + 1..].to_lowercase(),
            _ => String::new(),
        }
    }
    fn sort_level(children: &mut [TreeNode], key: TreeSortKey) {
        children.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => match key {
                TreeSortKey::Char => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                TreeSortKey::Ext => ext_of(&a.name)
                    .cmp(&ext_of(&b.name))
                    .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
                TreeSortKey::Size => b
                    .size
                    .unwrap_or(0)
                    .cmp(&a.size.unwrap_or(0))
                    .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
            },
        });
        for child in children.iter_mut() {
            sort_level(&mut child.children, key);
        }
    }
    sort_level(&mut root.children, key);
}

pub fn human_readable_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 {
        format!("{} {}", bytes, UNITS[unit_idx])
    } else {
        format!("{:.2} {}", size, UNITS[unit_idx])
    }
}

/// Format a Duration into a human-readable elapsed time string.
pub fn format_duration(d: std::time::Duration) -> String {
    let total_ms = d.as_millis();
    if total_ms < 1000 {
        format!("{}ms", total_ms)
    } else if total_ms < 60_000 {
        format!("{}.{:03}s", total_ms / 1000, total_ms % 1000)
    } else {
        let secs = total_ms / 1000;
        format!("{}m {:02}s", secs / 60, secs % 60)
    }
}

// =========================================================================
// Tree formatting (uses cached sizes when available)
// =========================================================================

fn format_tree_inner(
    node: &TreeNode,
    prefix: &str,
    is_last: bool,
    show_filesize: bool,
    show_dirsize: bool,
    care: bool,
    pb: Option<&ProgressBar>,
) -> String {
    let mut output = String::new();
    let connector = match node.depth {
        0 => "",
        _ if is_last => "└── ",
        _ => "├── ",
    };
    let mut suffix = String::new();
    if node.is_dir && node.depth > 0 {
        suffix.push_str(" [Directory]");
    }
    if node.depth > 0 && node.is_ignored {
        suffix.push_str(" [ignored]");
    }
    if node.is_dir && node.depth > 0 && show_dirsize {
        let size_str = if let Some(size) = node.size {
            if let Some(pb) = pb {
                pb.inc(1);
            }
            human_readable_size(size)
        } else {
            let size = if care {
                calculate_total_size(Path::new(&node.path))
            } else {
                calculate_dir_size(Path::new(&node.path))
            };
            if let Some(pb) = pb {
                pb.inc(1);
            }
            human_readable_size(size)
        };
        suffix.push_str(&format!(" ({})", size_str));
    } else if !node.is_dir && show_filesize {
        let size_str = if let Some(size) = node.size {
            human_readable_size(size)
        } else {
            human_readable_size(
                std::fs::metadata(Path::new(&node.path))
                    .map(|m| m.len())
                    .unwrap_or(0),
            )
        };
        suffix.push_str(&format!(" ({})", size_str));
    }

    output.push_str(&format!("{}{}{}{}\n", prefix, connector, node.name, suffix));

    for (i, child) in node.children.iter().enumerate() {
        let is_last_child = i == node.children.len() - 1;
        let new_prefix = match node.depth {
            0 => String::new(),
            _ if is_last => format!("{}    ", prefix),
            _ => format!("{}│   ", prefix),
        };
        output.push_str(&format_tree_inner(
            child,
            &new_prefix,
            is_last_child,
            show_filesize,
            show_dirsize,
            care,
            pb,
        ));
    }
    output
}

/// Format tree as string (no sizes)
pub fn format_tree(node: &TreeNode, prefix: &str, is_last: bool) -> String {
    format_tree_inner(node, prefix, is_last, false, false, false, None)
}

/// Format tree with optional file and/or directory sizes.
pub fn format_tree_with_sizes(
    node: &TreeNode,
    prefix: &str,
    is_last: bool,
    show_filesize: bool,
    show_dirsize: bool,
    care: bool,
    pb: Option<&ProgressBar>,
) -> String {
    format_tree_inner(node, prefix, is_last, show_filesize, show_dirsize, care, pb)
}

/// Append a git status letter (M/A/D/?/U) to file node names for any path
/// present in `status`. In-place annotation used right before formatting.
pub fn annotate_git_status(node: &mut TreeNode, status: &HashMap<PathBuf, char>) {
    if !node.is_dir {
        if let Some(code) = status.get(Path::new(&node.path)) {
            node.name = format!("{} {}", node.name, code);
        }
    }
    for child in &mut node.children {
        annotate_git_status(child, status);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(name: &str) -> TreeNode {
        TreeNode {
            name: name.to_string(),
            path: format!("/root/{}", name),
            is_dir: false,
            is_ignored: false,
            is_supported: None,
            children: Vec::new(),
            depth: 1,
            size: None,
        }
    }

    fn dir(name: &str, children: Vec<TreeNode>) -> TreeNode {
        TreeNode {
            name: name.to_string(),
            path: format!("/root/{}", name),
            is_dir: true,
            is_ignored: false,
            is_supported: None,
            children,
            depth: 1,
            size: None,
        }
    }

    #[test]
    fn prune_keeps_matches_and_nonempty_dirs() {
        let mut root = TreeNode {
            name: "root".to_string(),
            path: "/root".to_string(),
            is_dir: true,
            is_ignored: false,
            is_supported: None,
            children: vec![
                dir("src", vec![file("main.rs"), file("notes.txt")]),
                dir("empty", vec![file("photo.png")]),
                file("top.c"),
            ],
            depth: 0,
            size: None,
        };
        let n = prune_tree_by_exts(&mut root, &["rs".to_string(), "c".to_string()]);
        assert_eq!(n, 2);
        assert_eq!(root.children.len(), 2);
        let src = root.children.iter().find(|c| c.name == "src").unwrap();
        assert_eq!(src.children.len(), 1);
        assert_eq!(src.children[0].name, "main.rs");
        assert!(root.children.iter().any(|c| c.name == "top.c"));
        assert!(!root.children.iter().any(|c| c.name == "empty"));
    }

    #[test]
    fn prune_entry_filter_files_only_drops_dirs_exactly() {
        let mut root = TreeNode {
            name: "root".to_string(),
            path: "/root".to_string(),
            is_dir: true,
            is_ignored: false,
            is_supported: None,
            children: vec![
                dir("src", vec![file("main.rs")]),
                file("top.c"),
                dir("empty", vec![]),
            ],
            depth: 0,
            size: None,
        };
        let kept = prune_tree_by_entry_filter(&mut root, TreeEntryFilter::FilesOnly);
        assert_eq!(kept, 1);
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].name, "top.c");
    }

    #[test]
    fn prune_entry_filter_dirs_only_drops_files_recursively() {
        let mut root = TreeNode {
            name: "root".to_string(),
            path: "/root".to_string(),
            is_dir: true,
            is_ignored: false,
            is_supported: None,
            children: vec![
                dir(
                    "src",
                    vec![file("main.rs"), dir("nested", vec![file("x.rs")])],
                ),
                file("top.c"),
                dir("empty", vec![]),
            ],
            depth: 0,
            size: None,
        };
        let kept = prune_tree_by_entry_filter(&mut root, TreeEntryFilter::DirsOnly);
        assert_eq!(kept, 3);
        assert_eq!(root.children.len(), 2);
        let src = root.children.iter().find(|c| c.name == "src").unwrap();
        assert_eq!(src.children.len(), 1);
        assert_eq!(src.children[0].name, "nested");
        assert!(src.children[0].children.is_empty());
        assert!(root.children.iter().any(|c| c.name == "empty"));
    }

    #[test]
    fn parse_size_value_units() {
        assert_eq!(parse_size_value("1024"), Some(1024));
        assert_eq!(parse_size_value("0"), Some(0));
        assert_eq!(parse_size_value("10B"), Some(10));
        assert_eq!(parse_size_value("10KB"), Some(10 * 1024));
        assert_eq!(parse_size_value("1.5MB"), Some(1_572_864));
        assert_eq!(parse_size_value("2gb"), Some(2 * 1024 * 1024 * 1024));
        assert_eq!(parse_size_value("1TB"), Some(1024 * 1024 * 1024 * 1024));
        assert_eq!(parse_size_value(" 3 MB "), Some(3 * 1024 * 1024));
        assert_eq!(parse_size_value(""), None);
        assert_eq!(parse_size_value("MB"), None);
        assert_eq!(parse_size_value("-5"), None);
        assert_eq!(parse_size_value("abc"), None);
        assert_eq!(parse_size_value("10XB"), None);
    }

    fn sized_file(name: &str, size: u64) -> TreeNode {
        let mut n = file(name);
        n.size = Some(size);
        n
    }

    fn sized_dir(name: &str, size: u64, children: Vec<TreeNode>) -> TreeNode {
        let mut d = dir(name, children);
        d.size = Some(size);
        d
    }

    #[test]
    fn prune_by_min_size_drops_small_files_and_emptied_dirs() {
        let mut root = TreeNode {
            name: "root".to_string(),
            path: "/root".to_string(),
            is_dir: true,
            is_ignored: false,
            is_supported: None,
            children: vec![
                sized_dir(
                    "src",
                    500,
                    vec![sized_file("big.rs", 400), sized_file("tiny.rs", 100)],
                ),
                sized_dir("docs", 50, vec![sized_file("note.md", 50)]),
                sized_file("top.bin", 10),
            ],
            depth: 0,
            size: Some(560),
        };
        let (files, _) = prune_tree_by_min_size(&mut root, Some(200), None);
        assert_eq!(files, 1);
        // docs emptied → pruned; top.bin dropped; tiny.rs dropped.
        assert_eq!(root.children.len(), 1);
        let src = root.children.iter().find(|c| c.name == "src").unwrap();
        assert_eq!(src.children.len(), 1);
        assert_eq!(src.children[0].name, "big.rs");
    }

    #[test]
    fn prune_by_dir_min_prunes_small_subtrees() {
        let mut root = TreeNode {
            name: "root".to_string(),
            path: "/root".to_string(),
            is_dir: true,
            is_ignored: false,
            is_supported: None,
            children: vec![
                sized_dir("big", 5000, vec![sized_file("a.bin", 5000)]),
                sized_dir("small", 100, vec![sized_file("b.bin", 100)]),
            ],
            depth: 0,
            size: Some(5100),
        };
        prune_tree_by_min_size(&mut root, None, Some(1000));
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].name, "big");
    }

    #[test]
    fn prune_by_min_size_no_thresholds_keeps_all() {
        let mut root = TreeNode {
            name: "root".to_string(),
            path: "/root".to_string(),
            is_dir: true,
            is_ignored: false,
            is_supported: None,
            children: vec![file("a.txt")],
            depth: 0,
            size: None,
        };
        let (files, _) = prune_tree_by_min_size(&mut root, None, None);
        assert_eq!(files, 1);
        assert_eq!(root.children.len(), 1);
    }

    #[test]
    fn sort_tree_by_key_orders_children() {
        let mut root = TreeNode {
            name: "root".to_string(),
            path: "/root".to_string(),
            is_dir: true,
            is_ignored: false,
            is_supported: None,
            children: vec![
                sized_file("b.txt", 300),
                sized_dir("adir", 10, vec![]),
                sized_file("a.md", 100),
                sized_file("c.rs", 200),
            ],
            depth: 0,
            size: None,
        };
        sort_tree_by_key(&mut root, TreeSortKey::Size);
        let names: Vec<&str> = root.children.iter().map(|c| c.name.as_str()).collect();
        // Dirs first, then files largest-first.
        assert_eq!(names, vec!["adir", "b.txt", "c.rs", "a.md"]);
        sort_tree_by_key(&mut root, TreeSortKey::Ext);
        let names: Vec<&str> = root.children.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, vec!["adir", "a.md", "c.rs", "b.txt"]);
        sort_tree_by_key(&mut root, TreeSortKey::Char);
        let names: Vec<&str> = root.children.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, vec!["adir", "a.md", "b.txt", "c.rs"]);
    }

    #[test]
    fn prune_zero_matches_keeps_root_only() {
        let mut root = TreeNode {
            name: "root".to_string(),
            path: "/root".to_string(),
            is_dir: true,
            is_ignored: false,
            is_supported: None,
            children: vec![file("a.txt")],
            depth: 0,
            size: None,
        };
        assert_eq!(prune_tree_by_exts(&mut root, &["zzz".to_string()]), 0);
        assert!(root.children.is_empty());
        assert_eq!(root.name, "root");
    }

    #[test]
    fn file_matches_ignore_detects_name_and_extension() {
        let ignored_files = HashSet::from(["Cargo.lock".to_string()]);
        let ignored_extensions = HashSet::from(["log".to_string()]);
        assert!(file_matches_ignore(
            "Cargo.lock",
            Path::new("Cargo.lock"),
            &ignored_files,
            &ignored_extensions
        ));
        assert!(file_matches_ignore(
            "app.log",
            Path::new("app.log"),
            &ignored_files,
            &ignored_extensions
        ));
        assert!(!file_matches_ignore(
            "main.rs",
            Path::new("main.rs"),
            &ignored_files,
            &ignored_extensions
        ));
    }

    #[test]
    fn ignored_entries_sort_after_normal_entries_in_combined_tree() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir(tmp.path().join("src")).unwrap();
        std::fs::create_dir(tmp.path().join(".git")).unwrap();
        std::fs::write(tmp.path().join("main.rs"), "x").unwrap();
        std::fs::write(tmp.path().join("Cargo.lock"), "x").unwrap();
        let root_path = tmp.path().to_string_lossy().to_string();

        let mut cfg = Config::new();
        cfg.ignored_files.insert("Cargo.lock".to_string());
        let tree = Config::with_test_config(cfg, || generate_tree(&root_path, Some(1), true, None));
        let names: Vec<&str> = tree.children.iter().map(|c| c.name.as_str()).collect();
        // dirs -> files -> ignored dirs -> ignored files
        assert_eq!(names, vec!["src", "main.rs", ".git", "Cargo.lock"]);
        assert!(!tree.children[0].is_ignored);
        assert!(!tree.children[1].is_ignored);
        assert!(tree.children[2].is_ignored);
        assert!(tree.children[3].is_ignored);
    }

    #[test]
    fn sort_tree_by_size_groups_keeps_groups_and_orders_largest_first() {
        let mut ignored_dir = sized_dir(".git", 60, vec![]);
        ignored_dir.is_ignored = true;
        let mut ignored_file = sized_file("secret.env", 25);
        ignored_file.is_ignored = true;
        let mut root = TreeNode {
            name: "root".to_string(),
            path: "/root".to_string(),
            is_dir: true,
            is_ignored: false,
            is_supported: None,
            children: vec![
                sized_file("small.rs", 10),
                ignored_file,
                sized_dir("big", 500, vec![]),
                ignored_dir,
                sized_file("large.bin", 900),
                sized_dir("small", 100, vec![]),
            ],
            depth: 0,
            size: None,
        };
        sort_tree_by_size_groups(&mut root, true, true);
        let names: Vec<&str> = root.children.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "big",
                "small",
                "large.bin",
                "small.rs",
                ".git",
                "secret.env"
            ]
        );
    }

    #[test]
    fn format_tree_keeps_ignored_marker_with_size() {
        let ignored_dir = TreeNode {
            name: ".git".to_string(),
            path: "/root/.git".to_string(),
            is_dir: true,
            is_ignored: true,
            is_supported: None,
            children: vec![],
            depth: 1,
            size: Some(59_400_000),
        };
        let ignored_file = TreeNode {
            name: "secret.env".to_string(),
            path: "/root/secret.env".to_string(),
            is_dir: false,
            is_ignored: true,
            is_supported: None,
            children: vec![],
            depth: 1,
            size: Some(25),
        };
        let normal_dir = TreeNode {
            name: "src".to_string(),
            path: "/root/src".to_string(),
            is_dir: true,
            is_ignored: false,
            is_supported: None,
            children: vec![],
            depth: 1,
            size: Some(6_200_000),
        };
        let root = TreeNode {
            name: "root".to_string(),
            path: "/root".to_string(),
            is_dir: true,
            is_ignored: false,
            is_supported: None,
            children: vec![normal_dir, ignored_dir, ignored_file],
            depth: 0,
            size: None,
        };

        let out = format_tree_with_sizes(&root, "", true, true, true, true, None);
        assert!(out.contains(".git [Directory] [ignored] ("));
        assert!(out.contains("secret.env [ignored] ("));
        assert!(out.contains("src [Directory] ("));
    }
}
