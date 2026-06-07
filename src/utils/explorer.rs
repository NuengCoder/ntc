use crate::config::Config;
use crate::filetype::{is_supported_format_with_config, FormatConfig};
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
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
              is_ignored: false,
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
 
     // Inject ignored directories at root depth so users can see they exist
     if let Ok(entries) = std::fs::read_dir(Path::new(root_path)) {
         for entry in entries.filter_map(|e| e.ok()) {
             if let Ok(ft) = entry.file_type() {
                 if ft.is_dir() {
                     let name_lower = entry.file_name().to_string_lossy().to_lowercase();
                     if ignored_dirs.contains(&name_lower) {
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

     // Return the root node of the assembled tree
      root_node
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
             child.size = std::fs::metadata(&child.path).map(|m| m.len()).ok();
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
    let ignored_dirs = Config::global_get_ignored_dirs();
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
    let ignored_dirs = Config::global_get_ignored_dirs();
    walk_files(path, &ignored_dirs)
        .par_iter()
        .map(|p| std::fs::metadata(p).map(|m| m.len()).unwrap_or(0))
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
        .map(|p| std::fs::metadata(p).map(|m| m.len()).unwrap_or(0))
        .sum()
}

 /// Calculate total size with progress bar (parallel)
 pub fn calculate_dir_size_with_progress(path: &Path, pb: &ProgressBar) -> u64 {
    let ignored_dirs = Config::global_get_ignored_dirs();
    walk_files(path, &ignored_dirs)
        .par_iter()
        .map(|p| {
            pb.inc(1);
            std::fs::metadata(p).map(|m| m.len()).unwrap_or(0)
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
 
// =========================================================================
// Tree formatting (uses cached sizes when available)
// =========================================================================
 
fn format_tree_inner(
    node: &TreeNode,
    prefix: &str,
    is_last: bool,
    show_sizes: bool,
    care: bool,
    pb: Option<&ProgressBar>,
) -> String {
     let mut output = String::new();
     let connector = match node.depth {
         0 => "",
         _ if is_last => "└── ",
         _ => "├── ",
     };
      let suffix = if node.is_dir && node.depth > 0 && node.is_ignored && !care {
          " [ignored]".to_string()
      } else if node.is_dir && node.depth > 0 {
          if show_sizes {
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
             format!(" [Directory] ({})", size_str)
         } else {
             " [Directory]".to_string()
         }
     } else {
         String::new()
     };
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
            show_sizes,
            care,
            pb,
        ));
    }
    output
}
 
/// Format tree as string (no sizes)
pub fn format_tree(node: &TreeNode, prefix: &str, is_last: bool) -> String {
    format_tree_inner(node, prefix, is_last, false, false, None)
}

 /// Format tree with optional directory sizes.
pub fn format_tree_with_sizes(
    node: &TreeNode,
    prefix: &str,
    is_last: bool,
    show_sizes: bool,
    care: bool,
    pb: Option<&ProgressBar>,
) -> String {
    format_tree_inner(node, prefix, is_last, show_sizes, care, pb)
}
 
/// Count total entries for progress bar
pub fn count_entries(root_path: &str, max_depth_override: Option<usize>) -> u64 {
    let max_depth = max_depth_override.unwrap_or(Config::global_get_max_depth());
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
