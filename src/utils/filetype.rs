use crate::config::Config;
use std::collections::HashSet;
use std::path::Path;
use walkdir::DirEntry;

/// Text file extensions that we can safely display
const TEXT_EXTENSIONS: &[&str] = &[
    "txt",
    "md",
    "rs",
    "py",
    "js",
    "ts",
    "html",
    "css",
    "xml",
    "yaml",
    "yml",
    "toml",
    "cfg",
    "conf",
    "c",
    "cpp",
    "h",
    "hpp",
    "java",
    "go",
    "rb",
    "php",
    "sh",
    "bat",
    "sql",
    "r",
    "swift",
    "kt",
    "scala",
    "lua",
    "perl",
    "csv",
    "gitignore",
    "dockerfile",
    "makefile",
    "readme",
    "license",
    "jsx",
    "jsp",
    "tsx",
    "dart",
    "cs",
    "kts",
    "mq4",
    "mq5",
    "mqh",
    "c3",
    "nim",
    "jai",
    "zig",
    "m",
    "iss",
    "ntc.ral",
    "ntc.igcare",
    "ntc.math",
    "ntc_theme",
    "vert",
    "frag",
    "vs",
    "fs",
    "gs",
    "vsh",
    "fsh",
    "gsh",
    "csh",
    "glsl",
    "geom",
    "comp",
    "tesc",
    "tese",
    "rgen",
    "rint",
    "rahit",
    "rchit",
    "rmiss",
    "rcall",
    "mesh",
    "task",
    "hlsl",
    "spvasm",
];

/// Known file extensions that are unsupported but should appear in tree
const KNOWN_UNSUPPORTED_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "ico", "svg", "webp", "mp3", "wav", "ogg", "flac", "aac",
    "wma", "m4a", "mp4", "avi", "mkv", "mov", "wmv", "flv", "webm", "doc", "docx", "pdf", "xls",
    "xlsx", "ppt", "pptx", "zip", "rar", "7z", "tar", "gz", "exe", "dll", "so", "bin", "iso",
    "dmg", "vmdk", "qcow2", "apk", "ipa", "jar", "war", "ear", "msi", "deb", "rpm", "qcow2", "vhd",
    "vhdx", "img", "ex4", "ex5", "spv", "spirv",
];

/// A snapshot of the config sets needed for format-checking.
/// Fetch this once before a walk and pass it to
/// `is_supported_format_with_config` for every file — avoids acquiring the
/// global RwLock and cloning four HashSets on every single file.
pub struct FormatConfig {
    pub ignored_files: HashSet<String>,
    pub extra_files: HashSet<String>,
    pub ignored_extensions: HashSet<String>,
    pub extra_extensions: HashSet<String>,
}

impl FormatConfig {
    /// Load all four sets from the global config in one go (4 lock acquisitions).
    pub fn from_global() -> Self {
        let cfg = Config::read_global();
        Self {
            ignored_files: cfg.ignored_files.clone(),
            extra_files: cfg.extra_supported_files.clone(),
            ignored_extensions: cfg.ignored_extensions.clone(),
            extra_extensions: cfg.extra_supported_extensions.clone(),
        }
    }

    /// Load all four sets combining both global AND local (ntconfig.toml) configs.
    /// Local config stays independent — its values are never written into global.
    pub fn from_both() -> Self {
        Self {
            ignored_files: Config::get_ignored_files_combined(),
            extra_files: Config::get_extra_supported_files_combined(),
            ignored_extensions: Config::get_ignored_extensions_combined(),
            extra_extensions: Config::get_extra_supported_extensions_combined(),
        }
    }
}

/// Check if a file is a supported (text-based) format, using a pre-fetched
/// `FormatConfig`. Use this inside directory walks so the config is only
/// read from the global lock once per walk, not once per file.
pub fn is_supported_format_with_config(path: &Path, cfg: &FormatConfig) -> bool {
    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    // 0. Specific ignored files — never supported.
    if cfg.ignored_files.contains(filename) {
        return false;
    }

    // 0b. Specific extra-supported files — always supported.
    if cfg.extra_files.contains(filename) {
        return true;
    }

    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();

        // 1. Ignored extensions — never supported.
        if cfg.ignored_extensions.contains(&ext_lower) {
            return false;
        }

        // 2. Extra supported extensions.
        if cfg.extra_extensions.contains(&ext_lower) {
            return true;
        }

        // 3. Built-in text extensions.
        if TEXT_EXTENSIONS.contains(&ext_lower.as_str()) {
            return true;
        }
    }

    // 4. Special filenames without extensions (Dockerfile, Makefile, etc.)
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        let name_lower = name.to_lowercase();
        for &kw in &["dockerfile", "makefile", "readme", "license"] {
            if name_lower.starts_with(kw) {
                return true;
            }
        }
    }

    // 5. File must exist for the heuristic below.
    if !path.is_file() {
        return false;
    }

    // 6. Heuristic: read first 8 KB; if no null bytes, treat as text.
    if let Ok(content) = std::fs::read(path) {
        let sample = &content[..content.len().min(8192)];
        if !sample.contains(&0) {
            return true;
        }
    }

    false
}

/// Convenience wrapper for single-file checks (e.g. `txt <file>` in the
/// shell). Fetches config from the global lock on every call — fine for
/// one-off checks, but use `is_supported_format_with_config` inside walks.
pub fn is_supported_format(path: &Path) -> bool {
    let cfg = FormatConfig::from_global();
    is_supported_format_with_config(path, &cfg)
}

/// Check if a file has a known-unsupported extension (images, audio, documents, etc.)
pub fn is_known_unsupported_format(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();
        KNOWN_UNSUPPORTED_EXTENSIONS.contains(&ext_lower.as_str())
    } else {
        false
    }
}

/// Create a WalkDir `filter_entry` closure that filters out ignored directories.
/// `ignored_dirs` is a set of lowercase directory names to exclude.
/// The closure returns `true` to include the entry, `false` to exclude it.
/// The root entry (depth 0) is always included.
pub fn make_ignored_dir_filter(ignored_dirs: &HashSet<String>) -> impl Fn(&DirEntry) -> bool + '_ {
    move |e: &DirEntry| {
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
    }
}

/// Strip Windows extended-length prefix (`\\?\`) from a path for display.
/// On non-Windows platforms this is a no-op string conversion.
pub fn strip_unc_prefix(path: &Path) -> String {
    let s = path.to_string_lossy();
    #[cfg(windows)]
    if let Some(stripped) = s.strip_prefix(r"\\?\") {
        return stripped.to_string();
    }
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_supported_format_txt() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("test.txt");
        fs::write(&file, "hello").unwrap();
        assert!(is_supported_format(&file));
    }

    #[test]
    fn test_is_supported_format_rs() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("main.rs");
        fs::write(&file, "fn main() {}").unwrap();
        assert!(is_supported_format(&file));
    }

    #[test]
    fn test_is_supported_format_binary() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("data.bin");
        // Write bytes containing a null byte (binary content)
        let mut data = vec![0u8; 100];
        data[0] = 0xFF;
        fs::write(&file, &data).unwrap();
        assert!(!is_supported_format(&file));
    }

    #[test]
    fn test_is_supported_format_dockerfile() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("Dockerfile");
        fs::write(&file, "FROM ubuntu").unwrap();
        assert!(is_supported_format(&file));
    }

    #[test]
    fn test_is_supported_format_makefile() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("Makefile");
        fs::write(&file, "all:").unwrap();
        assert!(is_supported_format(&file));
    }

    #[test]
    fn test_is_known_unsupported_format() {
        let dir = tempfile::tempdir().unwrap();
        let png = dir.path().join("image.png");
        let jpg = dir.path().join("photo.jpg");
        let zip = dir.path().join("archive.zip");
        let pdf = dir.path().join("doc.pdf");
        let mp3 = dir.path().join("song.mp3");

        assert!(is_known_unsupported_format(&png));
        assert!(is_known_unsupported_format(&jpg));
        assert!(is_known_unsupported_format(&zip));
        assert!(is_known_unsupported_format(&pdf));
        assert!(is_known_unsupported_format(&mp3));
    }

    #[test]
    fn test_is_not_known_unsupported_format() {
        let txt = Path::new("file.txt");
        let rs = Path::new("main.rs");
        assert!(!is_known_unsupported_format(txt));
        assert!(!is_known_unsupported_format(rs));
    }

    #[test]
    fn test_format_config_snapshot() {
        let cfg = FormatConfig::from_global();
        assert!(cfg.ignored_files.is_empty() || !cfg.ignored_files.is_empty());
        assert!(!cfg.extra_extensions.contains("rs"));
    }
}
