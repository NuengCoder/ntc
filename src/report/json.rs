use crate::config::Config;
use crate::explorer::{generate_tree, TreeNode};
use crate::filetype::{is_supported_format_with_config, FormatConfig};
use anyhow::Result;
use std::path::{Path};
use std::time::Instant;

pub struct JsonReportGenerator;

impl JsonReportGenerator {
    pub fn generate(dir_path: &Path, output_path: &Path, pretty: bool, depth: Option<usize>) -> Result<()> {
        let start = Instant::now();
        let dir_name = dir_path.file_name().unwrap_or_default().to_string_lossy();
        
        eprintln!("📊 Generating JSON report for: {}", dir_name);
        
        // Generate tree
        let tree = generate_tree(
            dir_path.to_string_lossy().as_ref(),
            depth,
            true,
            None,
        );
        
        // Build complete data structure
        let data = build_complete_json(&tree, dir_path, start.elapsed().as_secs_f64())?;
        
        // Convert to JSON string
        let json_string = if pretty {
            serde_json::to_string_pretty(&data)?
        } else {
            serde_json::to_string(&data)?
        };
        
        // Write to file
        std::fs::write(output_path, json_string)?;
        
        eprintln!("✅ JSON report saved to: {}", output_path.display());
        
        Ok(())
    }
    
    pub fn generate_to_string(dir_path: &Path, pretty: bool, depth: Option<usize>) -> Result<String> {
        let start = Instant::now();
        let tree = generate_tree(
            dir_path.to_string_lossy().as_ref(),
            depth,
            true,
            None,
        );
        
        let data = build_complete_json(&tree, dir_path, start.elapsed().as_secs_f64())?;
        
        if pretty {
            Ok(serde_json::to_string_pretty(&data)?)
        } else {
            Ok(serde_json::to_string(&data)?)
        }
    }
}

fn build_complete_json(tree: &TreeNode, dir_path: &Path, scan_time: f64) -> Result<serde_json::Value> {
    let fmt_cfg = FormatConfig::from_global();
    let (supported_files, unsupported_files) = collect_files_with_metadata(dir_path, &fmt_cfg)?;
    
    Ok(serde_json::json!({
        "metadata": {
            "name": tree.name,
            "path": dir_path.to_string_lossy(),
            "generated": chrono::Local::now().to_rfc3339(),
            "version": env!("CARGO_PKG_VERSION"),
            "scanner": "ntc",
            "scan_time_seconds": scan_time
        },
        "statistics": {
            "total_files": count_files(tree),
            "total_directories": count_dirs(tree),
            "total_size_bytes": crate::explorer::calculate_total_size(dir_path),
            "total_size_human": crate::explorer::human_readable_size(crate::explorer::calculate_total_size(dir_path)),
            "supported_files": supported_files.len(),
            "unsupported_files": unsupported_files.len()
        },
        "configuration": {
            "max_depth": Config::global_get_max_depth(),
            "show_line_numbers": Config::global_get_show_line_numbers(),
            "ignored_directories": Config::global_get_ignored_dirs().iter().collect::<Vec<_>>(),
            "ignored_extensions": fmt_cfg.ignored_extensions.iter().collect::<Vec<_>>(),
            "extra_extensions": fmt_cfg.extra_extensions.iter().collect::<Vec<_>>()
        },
        "tree": tree_to_json(tree),
        "supported_files": supported_files,
        "unsupported_files": unsupported_files
    }))
}

fn tree_to_json(node: &TreeNode) -> serde_json::Value {
    let mut children = Vec::new();
    for child in &node.children {
        children.push(tree_to_json(child));
    }
    
    let mut json = serde_json::json!({
        "name": node.name,
        "type": if node.is_dir { "directory" } else { "file" },
        "path": node.path,
        "depth": node.depth
    });
    
    if node.is_dir {
        if let Some(size) = node.size {
            json["size_bytes"] = serde_json::json!(size);
            json["size_human"] = serde_json::json!(crate::explorer::human_readable_size(size));
        }
        if !children.is_empty() {
            json["children"] = serde_json::json!(children);
        }
    } else {
        if let Ok(metadata) = std::fs::metadata(&node.path) {
            json["size_bytes"] = serde_json::json!(metadata.len());
            json["size_human"] = serde_json::json!(crate::explorer::human_readable_size(metadata.len()));
            json["modified"] = serde_json::json!(metadata.modified().ok().map(|t| format!("{:?}", t)));
        }
        json["is_supported"] = serde_json::json!(node.is_supported);
    }
    
    json
}

fn collect_files_with_metadata(dir_path: &Path, fmt_cfg: &FormatConfig) -> Result<(Vec<serde_json::Value>, Vec<serde_json::Value>)> {
    let mut supported = Vec::new();
    let mut unsupported = Vec::new();
    let ignored_dirs = Config::global_get_ignored_dirs();
    let max_depth = Config::global_get_max_depth();
    
    let walker = walkdir::WalkDir::new(dir_path)
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
        });
    
    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            let metadata = std::fs::metadata(path)?;
            
            let file_info = serde_json::json!({
                "name": path.file_name().unwrap_or_default().to_string_lossy(),
                "path": path.to_string_lossy(),
                "size_bytes": metadata.len(),
                "size_human": crate::explorer::human_readable_size(metadata.len()),
                "modified": format!("{:?}", metadata.modified().ok()),
                "extension": path.extension().and_then(|e| e.to_str()).unwrap_or("")
            });
            
            if is_supported_format_with_config(path, fmt_cfg) {
                supported.push(file_info);
            } else {
                unsupported.push(file_info);
            }
        }
    }
    
    Ok((supported, unsupported))
}

fn count_files(node: &TreeNode) -> u64 {
    let mut count = if !node.is_dir { 1 } else { 0 };
    for child in &node.children {
        count += count_files(child);
    }
    count
}

fn count_dirs(node: &TreeNode) -> u64 {
    let mut count = if node.is_dir && node.depth > 0 { 1 } else { 0 };
    for child in &node.children {
        count += count_dirs(child);
    }
    count
}

