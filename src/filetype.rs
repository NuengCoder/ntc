use crate::config::Config;
use std::collections::HashSet;
use std::path::Path;

/// Text file extensions that we can safely display
const TEXT_EXTENSIONS: &[&str] = &[
    "txt", "md", "rs", "py", "js", "ts", "html", "css", "xml", "yaml", "yml",
    "toml", "cfg", "conf", "c", "cpp", "h", "hpp", "java", "go", "rb", "php",
    "sh", "bat", "sql", "r", "swift", "kt", "scala", "lua", "perl",
    "csv", "gitignore", "dockerfile", "makefile", "readme", "license",
    "jsx", "jsp", "tsx", "dart", "cs", "kts", "mq4", "mq5", "mqh"
];

/// Known file extensions that are unsupported but should appear in tree
const KNOWN_UNSUPPORTED_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "ico", "svg", "webp",
    "mp3", "wav", "ogg", "flac", "aac", "wma", "m4a",
    "mp4", "avi", "mkv", "mov", "wmv", "flv", "webm",
    "doc", "docx", "pdf", "xls", "xlsx", "ppt", "pptx",
    "zip", "rar", "7z", "tar", "gz",
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
        Self {
            ignored_files: Config::global_get_ignored_files(),
            extra_files: Config::global_get_extra_supported_files(),
            ignored_extensions: Config::global_get_ignored_extensions(),
            extra_extensions: Config::global_get_extra_supported_extensions(),
        }
    }
}

/// Check if a file is a supported (text-based) format, using a pre-fetched
/// `FormatConfig`. Use this inside directory walks so the config is only
/// read from the global lock once per walk, not once per file.
pub fn is_supported_format_with_config(path: &Path, cfg: &FormatConfig) -> bool {
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

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

/// Get the support status for display
pub fn get_file_status(path: &Path) -> FileStatus {
    if !path.is_file() {
        FileStatus::NotAFile
    } else if is_supported_format(path) {
        FileStatus::Supported
    } else {
        FileStatus::NotSupported
    }
}

#[derive(Debug, PartialEq)]
pub enum FileStatus {
    Supported,
    NotSupported,
    NotAFile,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn test_cfg() -> FormatConfig {
        FormatConfig {
            ignored_files: HashSet::new(),
            extra_files: HashSet::new(),
            ignored_extensions: HashSet::new(),
            extra_extensions: HashSet::new(),
        }
    }

    #[test]
    fn test_txt_file_is_supported() {
        assert!(is_supported_format(Path::new("test.txt")));
    }

    #[test]
    fn test_rs_file_is_supported() {
        assert!(is_supported_format(Path::new("main.rs")));
    }

    #[test]
    fn test_with_config_ignored_file() {
        let mut cfg = test_cfg();
        cfg.ignored_files.insert("secret.txt".to_string());
        assert!(!is_supported_format_with_config(Path::new("secret.txt"), &cfg));
    }

    #[test]
    fn test_with_config_extra_file() {
        let mut cfg = test_cfg();
        cfg.extra_files.insert("Makefile".to_string());
        assert!(is_supported_format_with_config(Path::new("Makefile"), &cfg));
    }

    #[test]
    fn test_with_config_ignored_extension() {
        let mut cfg = test_cfg();
        cfg.ignored_extensions.insert("rs".to_string());
        assert!(!is_supported_format_with_config(Path::new("main.rs"), &cfg));
    }

    #[test]
    fn test_with_config_extra_extension() {
        let mut cfg = test_cfg();
        cfg.extra_extensions.insert("xyz".to_string());
        assert!(is_supported_format_with_config(Path::new("data.xyz"), &cfg));
    }

    #[test]
    fn test_exe_file_is_not_supported_by_extension() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp = NamedTempFile::new()?;
        temp.write_all(b"\x00\x01\x02\x03")?;
        let exe_path = temp.path().with_extension("exe");
        std::fs::rename(temp.path(), &exe_path)?;

        let status = get_file_status(&exe_path);
        assert_eq!(status, FileStatus::NotSupported);
        Ok(())
    }

    #[test]
    fn test_null_bytes_detected_as_binary() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp = NamedTempFile::new()?;
        temp.write_all(&[0, 1, 2, 3, 0, 5])?;
        assert!(!is_supported_format(temp.path()));
        Ok(())
    }

    #[test]
    fn test_text_content_is_supported() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp = NamedTempFile::new()?;
        temp.write_all(b"Hello, World! This is text.")?;
        assert!(is_supported_format(temp.path()));
        Ok(())
    }

    #[test]
    fn test_directory_is_not_supported() {
        let status = get_file_status(Path::new("src"));
        assert_ne!(status, FileStatus::Supported);
    }

    #[test]
    fn test_known_unsupported_png() {
        assert!(is_known_unsupported_format(Path::new("image.png")));
    }

    #[test]
    fn test_known_unsupported_mp3() {
        assert!(is_known_unsupported_format(Path::new("song.mp3")));
    }

    #[test]
    fn test_known_unsupported_docx() {
        assert!(is_known_unsupported_format(Path::new("document.docx")));
    }

    #[test]
    fn test_rs_not_known_unsupported() {
        assert!(!is_known_unsupported_format(Path::new("main.rs")));
    }
}