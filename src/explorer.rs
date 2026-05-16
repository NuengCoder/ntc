use crate::config::Config;
use crate::filetype::{is_supported_format_with_config, FormatConfig};
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct TreeNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub is_supported: Option<bool>,
    pub children: Vec<TreeNode>,
    pub depth: usize,
}

/// Generate a hierarchical tree.
///
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
    let max_depth = max_depth_override.unwrap_or_else(|| Config::global_get_max_depth());
    let root = Path::new(root_path);
    let root_name = root
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let ignored_dirs = Config::global_get_ignored_dirs();
    // Fetch format config once for the entire walk — avoids 4 lock acquisitions per file.
    let fmt_cfg = FormatConfig::from_global();

    struct FlatNode {
        name: String,
        path: String,
        is_dir: bool,
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
        .filter_entry(|e| {
            if e.depth() == 0 {
                return true;
            }
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy().to_lowercase();
                if ignored_dirs.contains(&name) {
                    return false;
                }
            }
            true
        });

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

        let idx = flat.len();
        flat.push(FlatNode {
            name,
            path: path_str.clone(),
            is_dir,
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

    // Convert each FlatNode into a TreeNode (children empty for now).
    let mut nodes: Vec<Option<TreeNode>> = flat
        .into_iter()
        .map(|f| {
            Some(TreeNode {
                name: f.name,
                path: f.path,
                is_dir: f.is_dir,
                is_supported: f.is_supported,
                children: Vec::new(),
                depth: f.depth,
            })
        })
        .collect();

    // Move children into parents in reverse order (leaves first), so a parent
    // is never consumed before all its children have been moved into it.
    for i in (1..n).rev() {
        let child = nodes[i].take().unwrap();
        let parent = nodes[parent_indices[i]].as_mut().unwrap();
        parent.children.push(child);
    }

    let mut root_node = nodes[0].take().unwrap();

    // ---- Phase 3: sort children (dirs first, then alpha) ----
    fn sort_children(nodes: &mut [TreeNode]) {
        nodes.par_sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });
        for child in nodes.iter_mut() {
            sort_children(&mut child.children);
        }
    }
    sort_children(&mut root_node.children);

    root_node
}

/// Count files (respects ignored dirs)
pub fn count_files(path: &Path) -> u64 {
    let ignored_dirs = Config::global_get_ignored_dirs();
    WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            if e.depth() == 0 {
                return true;
            }
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy().to_lowercase();
                if ignored_dirs.contains(&name) {
                    return false;
                }
            }
            true
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .count() as u64
}

/// Calculate total size of a directory (parallel with rayon)
pub fn calculate_dir_size(path: &Path) -> u64 {
    let ignored_dirs = Config::global_get_ignored_dirs();

    let files: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            if e.depth() == 0 {
                return true;
            }
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy().to_lowercase();
                if ignored_dirs.contains(&name) {
                    return false;
                }
            }
            true
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

    // Use rayon parallel iterator for metadata reading
    files
        .par_iter()
        .map(|p| {
            std::fs::metadata(p)
                .map(|m| m.len())
                .unwrap_or(0)
        })
        .sum()
}

/// Calculate total size with progress bar (parallel)
pub fn calculate_dir_size_with_progress(path: &Path, pb: &ProgressBar) -> u64 {
    let ignored_dirs = Config::global_get_ignored_dirs();

    let files: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            if e.depth() == 0 {
                return true;
            }
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy().to_lowercase();
                if ignored_dirs.contains(&name) {
                    return false;
                }
            }
            true
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

    files
        .par_iter()
        .map(|p| {
            pb.inc(1);
            std::fs::metadata(p)
                .map(|m| m.len())
                .unwrap_or(0)
        })
        .sum()
}

/// Convert bytes to human-readable string
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

/// Format tree as string
pub fn format_tree(node: &TreeNode, prefix: &str, is_last: bool) -> String {
    let mut output = String::new();
    let connector = if node.depth == 0 {
        "".to_string()
    } else if is_last {
        "└── ".to_string()
    } else {
        "├── ".to_string()
    };

    let suffix = if node.is_dir && node.depth > 0 {
        " [Directory]"
    } else {
        ""
    };

    output.push_str(&format!("{}{}{}{}\n", prefix, connector, node.name, suffix));

    for (i, child) in node.children.iter().enumerate() {
        let is_last_child = i == node.children.len() - 1;
        let new_prefix = if node.depth == 0 {
            String::new()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };
        output.push_str(&format_tree(child, &new_prefix, is_last_child));
    }

    output
}

/// Format tree with optional directory sizes
pub fn format_tree_with_sizes(
    node: &TreeNode,
    prefix: &str,
    is_last: bool,
    show_sizes: bool,
    pb: Option<&ProgressBar>,
) -> String {
    let mut output = String::new();
    let connector = if node.depth == 0 {
        "".to_string()
    } else if is_last {
        "└── ".to_string()
    } else {
        "├── ".to_string()
    };

    let suffix = if node.is_dir && node.depth > 0 {
        if show_sizes {
            let dir_path = Path::new(&node.path);
            let size = calculate_dir_size(dir_path);
            if let Some(pb) = pb {
                pb.inc(1);
            }
            format!(" [Directory] ({})", human_readable_size(size))
        } else {
            " [Directory]".to_string()
        }
    } else {
        "".to_string()
    };

    output.push_str(&format!("{}{}{}{}\n", prefix, connector, node.name, suffix));

    for (i, child) in node.children.iter().enumerate() {
        let is_last_child = i == node.children.len() - 1;
        let new_prefix = if node.depth == 0 {
            String::new()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };
        output.push_str(&format_tree_with_sizes(
            child,
            &new_prefix,
            is_last_child,
            show_sizes,
            pb,
        ));
    }
    output
}

/// Count total entries for progress bar
pub fn count_entries(root_path: &str, max_depth_override: Option<usize>) -> u64 {
    let max_depth = max_depth_override.unwrap_or_else(|| Config::global_get_max_depth());
    let ignored_dirs = Config::global_get_ignored_dirs();

    WalkDir::new(root_path)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|e| {
            if e.depth() == 0 {
                return true;
            }
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy().to_lowercase();
                if ignored_dirs.contains(&name) {
                    return false;
                }
            }
            true
        })
        .count() as u64
}

/// Count directory nodes in a tree (excludes the root itself).
/// Used for sizing the progress bar when scanning directory sizes.
/// Lives here so both cli.rs and shell.rs can share it without duplication.
pub fn count_dirs_in_tree(node: &TreeNode) -> u64 {
    let mut count = if node.is_dir && node.depth > 0 { 1 } else { 0 };
    for child in &node.children {
        count += count_dirs_in_tree(child);
    }
    count
}