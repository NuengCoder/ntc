use crate::config::Config;
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

/// Check if a file is a support-format (text-based) file
pub fn is_supported_format(path: &Path) -> bool {
    // Get filename for specific file checks
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    // 0. Check specific ignored files (ignoren) - these are NEVER supported
    let ignored_files = Config::global_get_ignored_files();
    if ignored_files.contains(filename) {
        return false;
    }

    // 0b. Check specific extra supported files (caren) - these are ALWAYS supported
    let extra_files = Config::global_get_extra_supported_files();
    if extra_files.contains(filename) {
        return true;
    }

    // 1. Check ignored extensions (from config) – these are never supported.
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();
        let ignored = Config::global_get_ignored_extensions();
        if ignored.contains(&ext_lower) {
            return false;
        }
    }

    // 2. Check extra supported extensions (added by 'caref').
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();
        let extra = Config::global_get_extra_supported_extensions();
        if extra.contains(&ext_lower) {
            return true;
        }
    }

    // 3. Check built‑in text extensions.
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();
        if TEXT_EXTENSIONS.contains(&ext_lower.as_str()) {
            return true;
        }
    }

    // 4. Check special filenames without extensions (Dockerfile, Makefile, etc.)
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        let name_lower = name.to_lowercase();
        for &kw in &["dockerfile", "makefile", "readme", "license"] {
            if name_lower.starts_with(kw) {
                return true;
            }
        }
    }

    // 5. If the file doesn't exist, we've done all we can.
    if !path.is_file() {
        return false;
    }

    // 6. Heuristic: read the first 8KB; if no null bytes, treat as text.
    if let Ok(content) = std::fs::read(path) {
        let sample = &content[..content.len().min(8192)];
        if !sample.contains(&0) {
            return true;
        }
    }

    false
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

    #[test]
    fn test_txt_file_is_supported() {
        let path = Path::new("test.txt");
        assert!(is_supported_format(&path));
    }

    #[test]
    fn test_rs_file_is_supported() {
        let path = Path::new("main.rs");
        assert!(is_supported_format(&path));
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
        let path = temp.path().to_path_buf();
        assert!(!is_supported_format(&path));
        Ok(())
    }

    #[test]
    fn test_text_content_is_supported() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp = NamedTempFile::new()?;
        temp.write_all(b"Hello, World! This is text.")?;
        let path = temp.path().to_path_buf();
        assert!(is_supported_format(&path));
        Ok(())
    }

    #[test]
    fn test_directory_is_not_supported() {
        let path = Path::new("src");
        let status = get_file_status(&path);
        assert_ne!(status, FileStatus::Supported);
    }

    #[test]
    fn test_known_unsupported_png() {
        let path = Path::new("image.png");
        assert!(is_known_unsupported_format(&path));
    }

    #[test]
    fn test_known_unsupported_mp3() {
        let path = Path::new("song.mp3");
        assert!(is_known_unsupported_format(&path));
    }

    #[test]
    fn test_known_unsupported_docx() {
        let path = Path::new("document.docx");
        assert!(is_known_unsupported_format(&path));
    }

    #[test]
    fn test_rs_not_known_unsupported() {
        let path = Path::new("main.rs");
        assert!(!is_known_unsupported_format(&path));
    }
}