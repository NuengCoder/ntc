use crate::config::Config;
use crate::filetype::is_supported_format;
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
pub fn generate_tree(
    root_path: &str,
    max_depth_override: Option<usize>,
    include_files: bool,
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
            // Skip hidden entries
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
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
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        })
        .count() as u64
}
