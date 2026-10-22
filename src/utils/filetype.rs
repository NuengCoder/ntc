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
    "ntc",
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
    "vhdx", "img", "ex4", "ex5", "spv", "spirv", "o", "obj", "a", "lib", "pyc", "pyo",
    "class", "sqlite", "db", "db3",
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
        let eff = Config::effective_igcare();
        Self {
            ignored_files: eff.ignored_files,
            extra_files: eff.extra_supported_files,
            ignored_extensions: eff.ignored_extensions,
            extra_extensions: eff.extra_supported_extensions,
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
/// shell). Fetches the effective config (global + local ntconfig.toml) from
/// the global lock on every call — fine for one-off checks, but use
/// `is_supported_format_with_config` inside walks.
pub fn is_supported_format(path: &Path) -> bool {
    let cfg = FormatConfig::from_both();
    is_supported_format_with_config(path, &cfg)
}

/// Every extension the hinter can complete: built-in text extensions plus
/// known-unsupported ones plus user-configured extra extensions
/// (global + local). Sorted and deduplicated, all lowercase.
pub(crate) fn all_known_extensions() -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(TEXT_EXTENSIONS.len() + 64);
    out.extend(TEXT_EXTENSIONS.iter().map(|s| s.to_string()));
    out.extend(KNOWN_UNSUPPORTED_EXTENSIONS.iter().map(|s| s.to_string()));
    out.extend(Config::get_extra_supported_extensions_combined());
    out.sort();
    out.dedup();
    out
}

/// Normalize one user-typed extension: trim whitespace, strip leading dots,
/// lowercase. Returns `None` when nothing usable remains.
pub(crate) fn normalize_extension(raw: &str) -> Option<String> {
    let cleaned = raw.trim().trim_start_matches('.').trim().to_lowercase();
    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned)
    }
}

/// Split a raw extension argument into normalized extensions.
/// Accepts space- and/or comma-separated values: `rs,c`, `rs c`, `rs, c`,
/// `.RS`. Deduplicated, order-preserving.
pub(crate) fn split_extensions(raw: &str) -> Vec<String> {
    let mut out = Vec::new();
    for part in raw.split([',', ' ', '\t']) {
        if let Some(ext) = normalize_extension(part) {
            if !out.contains(&ext) {
                out.push(ext);
            }
        }
    }
    out
}

/// True when `file_name` ends with one of `exts` (`.rs` suffix match,
/// case-insensitive). Multi-dot filters like `ntc.ral` work naturally, and
/// extension-less names match themselves (`Dockerfile` matches `dockerfile`).
pub(crate) fn matches_any_extension(file_name: &str, exts: &[String]) -> bool {
    let lower = file_name.to_lowercase();
    exts.iter()
        .any(|e| lower == *e || lower.ends_with(&format!(".{}", e)))
}

/// Display an extension list as `.rs, .c` for messages and footers.
pub(crate) fn format_ext_list(exts: &[String]) -> String {
    exts.iter()
        .map(|e| format!(".{}", e))
        .collect::<Vec<_>>()
        .join(", ")
}

/// True when `tok` ends an `-ext` value run: flags, `@tp`, anything with a
/// path separator, or a token resolving to an existing directory under `cwd`
/// (existing files don't stop it — `view` targets must be directories, so
/// `view -ext rs c` keeps working even if a file named `c` exists).
pub(crate) fn is_ext_value_stop(tok: &str, cwd: &Path) -> bool {
    if tok.is_empty() || tok.starts_with('-') || tok.starts_with('@') {
        return true;
    }
    if tok.contains('/') || tok.contains('\\') {
        return true;
    }
    let p = Path::new(tok);
    let full = if p.is_absolute() {
        p.to_path_buf()
    } else {
        cwd.join(p)
    };
    full.is_dir()
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

/// True when `path` should be hidden from the editor file finder:
/// known-binary extensions (exe, zip, gz, ex4, ex5, … — case-insensitive),
/// or extensionless files whose first 8 KB contain a null byte (compiled
/// outputs like `app` / `a.out`). Unknown extensions are kept so new text
/// formats are never punished.
pub fn is_finder_hidden(path: &Path) -> bool {
    if is_known_unsupported_format(path) {
        return true;
    }
    if path.extension().is_none() && path.is_file() {
        if let Ok(content) = std::fs::read(path) {
            let sample = &content[..content.len().min(8192)];
            if sample.contains(&0) {
                return true;
            }
        }
    }
    false
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

    #[test]
    fn test_split_extensions_accepts_commas_spaces_dots() {
        assert_eq!(split_extensions("rs"), vec!["rs"]);
        assert_eq!(split_extensions("rs,c"), vec!["rs", "c"]);
        assert_eq!(split_extensions("rs c"), vec!["rs", "c"]);
        assert_eq!(split_extensions("rs, c"), vec!["rs", "c"]);
        assert_eq!(split_extensions(".RS, .C"), vec!["rs", "c"]);
        assert_eq!(split_extensions("rs rs c"), vec!["rs", "c"]);
        assert!(split_extensions("  , ").is_empty());
    }

    #[test]
    fn test_matches_any_extension_suffix_and_bare() {
        let exts = vec!["rs".to_string(), "ntc.ral".to_string()];
        assert!(matches_any_extension("main.rs", &exts));
        assert!(matches_any_extension("LIB.RS", &exts));
        assert!(matches_any_extension("a.ntc.ral", &exts));
        assert!(!matches_any_extension("main.rs.bak", &exts));
        assert!(!matches_any_extension("main.py", &exts));
        assert!(matches_any_extension("Dockerfile", &["dockerfile".to_string()]));
    }

    #[test]
    fn test_unsupported_file_name_blocklist() {
        // Binaries/archives/media are suppressed for txtc/txtf hints.
        for name in [
            "setup.exe",
            "SETUP.EXE",
            "backup.zip",
            "data.tar.gz",
            "photo.png",
            "movie.mkv",
            "app.apk",
            "lib.dll",
        ] {
            assert!(is_finder_hidden(Path::new(name)), "{}", name);
        }
        // Text sources, extensionless names and bare words pass through.
        for name in ["main.rs", "Makefile", "Cargo.toml", "notes", "img", "a.txt"] {
            assert!(!is_finder_hidden(Path::new(name)), "{}", name);
        }
    }

    #[test]
    fn test_all_known_extensions_sorted_unique() {
        let all = all_known_extensions();
        assert!(all.contains(&"rs".to_string()));
        assert!(all.contains(&"zip".to_string()));
        let mut sorted = all.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(all, sorted);
    }

    #[test]
    fn test_format_ext_list() {
        assert_eq!(
            format_ext_list(&["rs".to_string(), "c".to_string()]),
            ".rs, .c"
        );
    }

    #[test]
    fn test_finder_hidden_binaries() {
        for name in [
            "app.exe",
            "setup.EXE",
            "backup.zip",
            "data.tar.gz",
            "expert.ex4",
            "expert.EX5",
            "lib.so",
            "lib.dll",
            "mod.pyc",
            "data.sqlite",
        ] {
            assert!(
                is_finder_hidden(Path::new(name)),
                "{} should be hidden",
                name
            );
        }
    }

    #[test]
    fn test_finder_keeps_text() {
        let dir = tempfile::tempdir().unwrap();
        let txt = dir.path().join("notes.txt");
        fs::write(&txt, "hello").unwrap();
        assert!(!is_finder_hidden(&txt));
        // Extensionless text is kept…
        let no_ext = dir.path().join("README");
        fs::write(&no_ext, "plain text, no nulls").unwrap();
        assert!(!is_finder_hidden(&no_ext));
        // …while extensionless binaries are hidden.
        let bin = dir.path().join("compiled_app");
        fs::write(&bin, b"MZ\x90\x00binary\x00data").unwrap();
        assert!(is_finder_hidden(&bin));
        // Unknown extensions are never punished.
        assert!(!is_finder_hidden(Path::new("weird.zzz9")));
    }
}
