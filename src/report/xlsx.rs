use crate::config::Config;
use crate::explorer::human_readable_size;
use crate::filetype::{is_supported_format_with_config, FormatConfig};
use anyhow::{Context, Result};
use rust_xlsxwriter::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const MAX_SCAN_SIZE: u64 = 1_048_576;

struct FileInfo {
    path: PathBuf,
    ext: String,
    size: u64,
    lines: u64,
    modified: String,
    imports: Vec<String>,
    imported_by: Vec<String>,
}

fn collect_all_files(dir_path: &Path, max_depth: usize) -> Vec<FileInfo> {
    let ignored_dirs = Config::global_get_ignored_dirs();
    let fmt_cfg = FormatConfig::from_global();

    let mut raw: Vec<PathBuf> = Vec::new();
    let walker = WalkDir::new(dir_path)
        .max_depth(max_depth)
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
        if entry.file_type().is_file() {
            raw.push(entry.path().to_path_buf());
        }
    }
    raw.sort();

    let mut files: Vec<FileInfo> = Vec::new();
    for p in &raw {
        let ext = p.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
        let size = fs::metadata(p).map(|m| m.len()).unwrap_or(0);
        let modified = fs::metadata(p)
            .and_then(|m| m.modified())
            .ok()
            .map(|t| {
                let dt: chrono::DateTime<chrono::Local> = t.into();
                dt.format("%Y-%m-%d %H:%M:%S").to_string()
            })
            .unwrap_or_default();
        let lines = if is_supported_format_with_config(p, &fmt_cfg) && size <= MAX_SCAN_SIZE {
            count_lines(p)
        } else {
            0
        };
        files.push(FileInfo {
            path: p.clone(),
            ext,
            size,
            lines,
            modified,
            imports: Vec::new(),
            imported_by: Vec::new(),
        });
    }
    files
}

fn count_lines(path: &Path) -> u64 {
    match fs::read_to_string(path) {
        Ok(s) => s.lines().count() as u64,
        Err(_) => 0,
    }
}

fn relative_path<'a>(file: &'a Path, root: &'a Path) -> &'a Path {
    file.strip_prefix(root).unwrap_or(file)
}

/// Build a map: "module/path" -> actual file path.
/// All paths use `/` as separator.
/// Files under `src/`, `lib/`, `app/` are also registered without that prefix
/// so that `use crate::config::Config` matches `src/config.rs`.
fn build_module_map(files: &[FileInfo], root: &Path) -> HashMap<String, PathBuf> {
    let mut map: HashMap<String, PathBuf> = HashMap::new();

    let source_roots = ["src", "lib", "app"];

    for fi in files {
        let rel = relative_path(&fi.path, root);
        let stem = rel.file_stem().map(|s| s.to_string_lossy()).unwrap_or_default();
        let parent = rel.parent().map(|p| p.to_string_lossy().replace('\\', "/")).unwrap_or_default();

        // Primary module path: e.g. "src/config", "src/report/txt"
        let mod_path = if parent.is_empty() || parent == "." || parent == "\\" {
            stem.to_string()
        } else {
            format!("{}/{}", parent, stem)
        };
        map.entry(mod_path.clone()).or_insert_with(|| fi.path.clone());

        // For mod.rs / index.js style files, register the parent directory as well
        if stem == "mod" || stem == "index" {
            if !parent.is_empty() && parent != "." && parent != "\\" {
                map.entry(parent.clone()).or_insert_with(|| fi.path.clone());
            }
        }

        // Also register aliases without common source root prefixes
        for root_dir in &source_roots {
            let prefix = format!("{}/", root_dir);
            if let Some(suffix) = mod_path.strip_prefix(&prefix) {
                map.entry(suffix.to_string()).or_insert_with(|| fi.path.clone());
                // Also register parent-only alias for mod.rs under source root
                if (stem == "mod" || stem == "index") && !suffix.contains('/') {
                    map.entry(suffix.to_string()).or_insert_with(|| fi.path.clone());
                }
            }
        }
    }
    map
}

/// Find the file that an import path resolves to, using prefix matching.
fn resolve_import(import: &str, module_map: &HashMap<String, PathBuf>) -> Option<PathBuf> {
    let import = import.trim();

    // Normalize relative JS/TS/Python imports: strip leading "./" and "../"
    let import = if let Some(rest) = import.strip_prefix("./") {
        rest
    } else if let Some(rest) = import.strip_prefix("../") {
        // Last-resort fallback: skip relative parent navigation for simplicity
        rest
    } else {
        import
    };

    // Strip file extension if present (e.g. "file.rs" -> "file", "path/file.js" -> "path/file")
    let import_no_ext = match import.rfind('.') {
        Some(pos) => &import[..pos],
        None => import,
    };

    // 1. Direct match
    if let Some(p) = module_map.get(import) {
        return Some(p.clone());
    }
    if let Some(p) = module_map.get(import_no_ext) {
        return Some(p.clone());
    }

    // 2. Try with common extensions appended
    let exts = ["rs", "py", "ts", "tsx", "js", "jsx", "c", "h", "cpp", "hpp",
                "dart", "kt", "kts", "swift", "cs", "go", "zig", "php", "java"];
    for ext in &exts {
        let with_ext = format!("{}.{}", import_no_ext, ext);
        if let Some(p) = module_map.get(&with_ext) {
            return Some(p.clone());
        }
    }

    // 3. Prefix matching: find the longest module path that is a prefix of the import
    let mut best: Option<(usize, PathBuf)> = None;
    for (mod_path, file_path) in module_map {
        if import.starts_with(mod_path) {
            let prefix_len = mod_path.len();
            let next_char = import[prefix_len..].chars().next();
            if next_char.is_none() || next_char == Some('/') {
                let replace = best.as_ref().map_or(true, |(len, _)| prefix_len > *len);
                if replace {
                    best = Some((prefix_len, file_path.clone()));
                }
            }
        }
    }

    best.map(|(_, p)| p)
}

fn scan_dependencies(files: &mut Vec<FileInfo>, root: &Path) {
    let module_map = build_module_map(files, root);
    let mut import_map: HashMap<usize, HashSet<usize>> = HashMap::new();

    for i in 0..files.len() {
        let p = &files[i].path;
        let ext = files[i].ext.as_str();
        let content = match fs::read_to_string(p) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let patterns = match ext {
            "rs" => scan_rust(&content),
            "py" => scan_python(&content),
            "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" => scan_jsts(&content),
            "c" | "h" | "cpp" | "hpp" | "cxx" | "hxx" | "cc" | "hh" => scan_cpp(&content),
            "dart" => scan_dart(&content),
            "kt" | "kts" => scan_kotlin(&content),
            "swift" => scan_swift(&content),
            "cs" => scan_csharp(&content),
            "go" => scan_go(&content),
            "zig" => scan_zig(&content),
            "php" => scan_php(&content),
            "java" => scan_java(&content),
            _ => Vec::new(),
        };

        let mut resolved = HashSet::new();
        for imp in &patterns {
            if let Some(target) = resolve_import(imp, &module_map) {
                if let Some(j) = files.iter().position(|f| f.path == target) {
                    resolved.insert(j);
                }
            }
        }
        import_map.insert(i, resolved);
    }

    for (i, refs) in &import_map {
        for &j in refs {
            let imp_path = relative_path(&files[j].path, root).to_string_lossy().to_string();
            files[*i].imports.push(imp_path.clone());
            let src_path = relative_path(&files[*i].path, root).to_string_lossy().to_string();
            files[j].imported_by.push(src_path);
        }
    }
}

fn scan_rust(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("use ") {
            // Take the part before ;, {, space, or tab — but NOT : (which is part of ::)
            let end = rest.find(|c| c == ';' || c == '{' || c == ' ' || c == '\t').unwrap_or(rest.len());
            let clean = rest[..end].trim().trim_end_matches(':');
            if clean.starts_with("crate::") {
                out.push(clean.strip_prefix("crate::").unwrap_or(clean).replace("::", "/"));
            } else if clean.starts_with("super::") {
                let relative = clean.strip_prefix("super::").unwrap_or(clean);
                out.push(relative.replace("::", "/"));
            }
        }
        if let Some(rest) = t.strip_prefix("mod ") {
            let end = rest.find(|c| c == ';' || c == ' ' || c == '\t').unwrap_or(rest.len());
            let name = rest[..end].trim();
            out.push(name.to_string());
        }
    }
    out
}

fn scan_python(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("import ") {
            for token in rest.split(',') {
                let name = token.trim().split_whitespace().next().unwrap_or(token.trim());
                if !name.starts_with('.') {
                    out.push(name.replace('.', "/"));
                }
            }
        }
        if let Some(rest) = t.strip_prefix("from ") {
            let parts: Vec<&str> = rest.splitn(2, " import ").collect();
            if parts.len() == 2 {
                let module = parts[0].trim().replace('.', "/");
                if !module.starts_with('.') {
                    out.push(module);
                }
            }
        }
    }
    out
}

fn scan_jsts(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if let Some(pos) = t.find(" from ") {
            let after = &t[pos + 6..];
            let q = after.trim();
            let path = q.trim_matches(&['"', '\'', ';', ' '][..]);
            if path.starts_with('.') {
                out.push(path.to_string());
            }
        }
        if let Some(pos) = t.find("require(") {
            let after = &t[pos + 8..];
            if let Some(end) = after.find(')') {
                let path = after[..end].trim().trim_matches(&['"', '\'', ' '][..]);
                if path.starts_with('.') {
                    out.push(path.to_string());
                }
            }
        }
    }
    out
}

fn scan_cpp(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("#include ") {
            let path = rest.trim().trim_matches(&['"', '<', '>', ' '][..]);
            if !path.contains('<') {
                for sep in &['/', '\\'] {
                    if path.contains(*sep) {
                        out.push(path.to_string());
                        break;
                    }
                }
                if !path.contains('/') && !path.contains('\\') {
                    out.push(path.to_string());
                }
            }
        }
    }
    out
}

fn scan_dart(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("import ") {
            let path = rest.trim().trim_matches(&['"', '\'', ';', ' '][..]);
            if path.starts_with('.') {
                out.push(path.to_string());
            }
        }
        if let Some(rest) = t.strip_prefix("part ") {
            let path = rest.trim().trim_matches(&['"', '\'', ';', ' '][..]);
            out.push(path.to_string());
        }
    }
    out
}

fn scan_kotlin(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("import ") {
            let path = rest.trim().trim_end_matches(&[';', ' '][..]);
            if !path.contains('*') {
                out.push(path.replace('.', "/"));
            }
        }
    }
    out
}

fn scan_swift(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("import ") {
            let module = rest.trim().to_string();
            if !module.starts_with("Foundation") && !module.starts_with("UIKit")
                && !module.starts_with("Swift") && !module.starts_with("Dispatch")
            {
                out.push(module);
            }
        }
    }
    out
}

fn scan_csharp(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("using ") {
            let path = rest.trim().trim_end_matches(&[';', ' '][..]);
            if !path.contains('*') && !path.starts_with("System") && !path.starts_with("Microsoft") {
                out.push(path.replace('.', "/"));
            }
        }
    }
    out
}

fn scan_go(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut in_block = false;
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("import (") { in_block = true; continue; }
        if in_block {
            if t.starts_with(')') { in_block = false; continue; }
            let path = t.trim().trim_matches(&['"', ' '][..]);
            if path.starts_with('.') || path.contains('/') {
                out.push(path.rsplit('/').next().unwrap_or(path).to_string());
            }
        }
        if let Some(rest) = t.strip_prefix("import ") {
            if !rest.starts_with('(') {
                let path = rest.trim().trim_matches(&['"', ' '][..]);
                if path.contains('/') {
                    out.push(path.rsplit('/').next().unwrap_or(path).to_string());
                }
            }
        }
    }
    out
}

fn scan_zig(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if let Some(pos) = t.find("@import(") {
            let after = &t[pos + 8..];
            if let Some(end) = after.find(')') {
                let path = after[..end].trim().trim_matches(&['"', '\'', ' '][..]);
                out.push(path.to_string());
            }
        }
    }
    out
}

fn scan_php(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        for kw in &["require ", "require_once ", "include ", "include_once "] {
            if let Some(rest) = t.strip_prefix(kw) {
                let path = rest.trim().trim_matches(&['"', '\'', '(', ')', ';', ' '][..]);
                out.push(path.to_string());
            }
        }
        if let Some(rest) = t.strip_prefix("use ") {
            let path = rest.trim().trim_end_matches(&[';', ' '][..]);
            if !path.contains('*') {
                out.push(path.replace('\\', "/"));
            }
        }
    }
    out
}

fn scan_java(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("import ") {
            let path = rest.trim().trim_end_matches(&[';', ' '][..]);
            if !path.contains('*') {
                out.push(path.replace('.', "/"));
            }
        }
    }
    out
}

pub fn generate_xlsx_report(dir_path: &Path, output_path: &Path) -> Result<()> {
    let max_depth = Config::global_get_max_depth();
    let root = dir_path;

    let mut files = collect_all_files(root, max_depth);
    scan_dependencies(&mut files, root);

    let mut wb = Workbook::new();

    let header_fmt = Format::new()
        .set_bold()
        .set_font_color(Color::White)
        .set_background_color(Color::RGB(0x2E75B6));

    let ws = wb.add_worksheet().set_name("Files")?;
    ws.set_column_width(0, 45)?;
    ws.set_column_width(1, 10)?;
    ws.set_column_width(2, 14)?;
    ws.set_column_width(3, 12)?;
    ws.set_column_width(4, 60)?;
    ws.set_column_width(5, 60)?;
    ws.set_column_width(6, 22)?;

    let headers = [
        "File Name",
        "Extension",
        "Size",
        "Lines",
        "Imports (→)",
        "Imported By (←)",
        "Last Modified",
    ];
    for (col, h) in headers.iter().enumerate() {
        ws.write_string_with_format(0, col as u16, *h, &header_fmt)?;
    }

    ws.autofilter(0, 0, files.len() as u32, 6)?;

    for (i, fi) in files.iter().enumerate() {
        let row = (i + 1) as u32;
        let rel = relative_path(&fi.path, root).to_string_lossy();
        ws.write_string(row, 0, rel.as_ref())?;
        ws.write_string(row, 1, &fi.ext)?;
        ws.write_string(row, 2, &human_readable_size(fi.size))?;
        ws.write_number(row, 3, fi.lines as f64)?;
        ws.write_string(row, 4, fi.imports.join(", "))?;
        ws.write_string(row, 5, fi.imported_by.join(", "))?;
        ws.write_string(row, 6, &fi.modified)?;
    }

    wb.save(output_path)
        .with_context(|| format!("Failed to save XLSX report to {}", output_path.display()))?;

    Ok(())
}
