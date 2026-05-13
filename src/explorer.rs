use crate::config::Config;
use crate::filetype::is_supported_format;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use indicatif::ProgressBar;
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

    let mut root_node = TreeNode {
        name: root_name,
        path: root_path.to_string(),
        is_dir: true,
        is_supported: None,
        children: Vec::new(),
        depth: 0,
    };

    let mut node_map: HashMap<PathBuf, *mut TreeNode> = HashMap::new();
    node_map.insert(root.to_path_buf(), &mut root_node);

    let ignored_dirs = Config::global_get_ignored_dirs();

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
        if let Ok(entry) = entry {
            if entry.path() == root {
                continue;
            }

            let is_dir = entry.file_type().is_dir();

            // Skip files when include_files is false (navigation mode)
            if !is_dir && !include_files {
                continue;
            }

            let parent_path = entry.path().parent().unwrap().to_path_buf();
            let depth = entry.depth();
            let name = entry.file_name().to_string_lossy().to_string();
            let path = entry.path().to_string_lossy().to_string();
            let is_supported = if is_dir {
                None
            } else {
                Some(is_supported_format(entry.path()))
            };

            let child = TreeNode {
                name,
                path,
                is_dir,
                is_supported,
                children: Vec::new(),
                depth,
            };

            // Attach child to parent
            if let Some(parent_ptr) = node_map.get(&parent_path).cloned() {
                unsafe {
                    (*parent_ptr).children.push(child);
                    let last_idx = (*parent_ptr).children.len() - 1;
                    if is_dir {
                        let child_ptr =
                            &mut (&mut (*parent_ptr).children)[last_idx] as *mut TreeNode;
                        node_map.insert(
                            PathBuf::from(&(&(*parent_ptr).children)[last_idx].path),
                            child_ptr,
                        );
                    }
                }
            }
        }
        if let Some(pb) = pb {
            pb.inc(1);
        }
    }

    // Sort: directories first, then files
    fn sort_children(nodes: &mut [TreeNode]) {
        nodes.sort_by(|a, b| match (a.is_dir, b.is_dir) {
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

pub fn count_files(path: &Path) -> u64 {
    let ignored_dirs = Config::global_get_ignored_dirs();
    WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            if e.depth() == 0 { return true; }
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy().to_lowercase();
                if ignored_dirs.contains(&name) { return false; }
            }
            true
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .count() as u64
}

/// Calculate total size of a directory recursively in bytes (respects ignored dirs)
pub fn calculate_dir_size(path: &Path) -> u64 {
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
        .map(|e| e.metadata().map(|m| m.len()).unwrap_or(0))
        .sum()
}

pub fn calculate_dir_size_with_progress(path: &Path, pb: &ProgressBar) -> u64 {
    let ignored_dirs = Config::global_get_ignored_dirs();
    
    WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            if e.depth() == 0 { return true; }
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy().to_lowercase();
                if ignored_dirs.contains(&name) { return false; }
            }
            true
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            pb.inc(1);
            e.metadata().map(|m| m.len()).unwrap_or(0)
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

/// Format tree with optional directory sizes (for view --size)
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
            pb
        ));
    }
    output
}

/// Count total entries for progress bar (respects same filters as generate_tree)
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
