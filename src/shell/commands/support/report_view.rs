//! Shared parsing and display helpers for `view`/`viewf`/`viewd`.

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use crate::teleport::TeleportManager;
use std::path::{Path, PathBuf};

pub(crate) fn tokenize_spans(args: &str) -> Vec<(String, usize, usize)> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut start: Option<usize> = None;
    let mut in_quotes = false;
    for (idx, ch) in args.char_indices() {
        match ch {
            '"' => {
                if start.is_none() {
                    start = Some(idx);
                }
                in_quotes = !in_quotes;
            }
            c if c.is_whitespace() && !in_quotes => {
                if let Some(s) = start.take() {
                    out.push((std::mem::take(&mut cur), s, idx));
                }
            }
            _ => {
                if start.is_none() {
                    start = Some(idx);
                }
                cur.push(ch);
            }
        }
    }
    if let Some(s) = start {
        out.push((cur, s, args.len()));
    }
    out
}

/// Split `-ext`/`--ext <ext...>` out of raw view args before target parsing
/// (which would otherwise mistake the first extension for a target path).
/// Returns (extensions, ext-flag-seen, cleaned-args with ext tokens blanked).
pub(crate) fn extract_ext_filter(args: &str, cwd: &Path) -> (Vec<String>, bool, String) {
    let toks = tokenize_spans(args);
    let mut exts: Vec<String> = Vec::new();
    let mut seen = false;
    let mut remove: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;
    while i < toks.len() {
        let lower = toks[i].0.to_lowercase();
        if lower == "-d" || lower == "--depth" || lower == "-p" || lower == "--path" {
            // Skip value-taking flags and their values (mirrors extract_target_spec).
            i += 2;
            continue;
        }
        if lower == "-ext" || lower == "--ext" || lower == "-e" {
            seen = true;
            remove.push((toks[i].1, toks[i].2));
            i += 1;
            while i < toks.len() && !crate::filetype::is_ext_value_stop(&toks[i].0, cwd) {
                remove.push((toks[i].1, toks[i].2));
                for e in crate::filetype::split_extensions(&toks[i].0) {
                    if !exts.contains(&e) {
                        exts.push(e);
                    }
                }
                i += 1;
            }
            continue;
        }
        i += 1;
    }
    let mut cleaned = args.to_string();
    for (s, e) in remove {
        cleaned.replace_range(s..e, &" ".repeat(e - s));
    }
    (exts, seen, cleaned)
}

/// Optional minimum-size value after `-s`/`-fs`/`-ds`: the next token
/// when it parses via [`crate::explorer::parse_size_value`].
/// A following path or flag means the size flag stays bare (show-only).
pub(crate) fn next_size_value(parts: &[String], i: usize) -> Option<u64> {
    parts
        .get(i + 1)
        .and_then(|v| crate::explorer::parse_size_value(v))
}

/// Required positive line count after `--head`/`--tail`.
pub(crate) fn next_count(parts: &[String], i: usize) -> Option<usize> {
    parts
        .get(i + 1)
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&n| n > 0)
}

/// Which entries `view`/`viewf`/`viewd` render.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ViewMode {
    All,
    Files,
    Dirs,
}

/// A `<view|viewf> <file>` request: display the file like `txtf`.
/// Only `--head`/`--tail`/`--betw` may accompany a file target.
#[derive(Debug)]
pub(crate) struct FileView {
    pub(crate) path: PathBuf,
    pub(crate) head: Option<usize>,
    pub(crate) tail: Option<usize>,
    pub(crate) betw: Option<String>,
}

/// Resolve a `view`/`txtf` operand: `@name[/sub...]` teleport paths or a
/// disk path relative to the navigator's directory.
pub(crate) fn resolve_view_operand(name: &str, nav: &Navigator) -> Option<PathBuf> {
    let expanded = crate::utils::paths::expand_path_str(name);
    let name = expanded.as_str();
    if name.starts_with('@') {
        return TeleportManager::resolve_subpath(name);
    }
    let p = Path::new(name);
    Some(if p.is_absolute() {
        p.to_path_buf()
    } else {
        nav.current_path().join(p)
    })
}

/// Split `view`/`viewf` args for a file target: the first bare token
/// resolving to an existing file (absolute, `nav`-relative, or `@tp/sub`).
/// Returns `None` when this is not a file view (directory flow continues).
/// `Some(Err(()))` means a file was targeted but flags are invalid —
/// the caller prints usage.
pub(crate) fn split_view_file_args(
    cleaned: &str,
    nav: &Navigator,
    mode: ViewMode,
) -> Option<Result<FileView, ()>> {
    let toks: Vec<String> = tokenize_spans(cleaned)
        .into_iter()
        .map(|(s, _, _)| s)
        .collect();
    let mut file: Option<String> = None;
    let mut head: Option<usize> = None;
    let mut tail: Option<usize> = None;
    let mut betw: Option<String> = None;
    let mut bad = false;
    let mut i = 0;
    while i < toks.len() {
        match toks[i].as_str() {
            "--head" => match next_count(&toks, i) {
                Some(n) => {
                    head = Some(n);
                    i += 1;
                }
                // Swallow the malformed value too so it isn't
                // mistaken for a second bare token below.
                None => {
                    bad = true;
                    i += 1;
                }
            },
            "--tail" => match next_count(&toks, i) {
                Some(n) => {
                    tail = Some(n);
                    i += 1;
                }
                None => {
                    bad = true;
                    i += 1;
                }
            },
            "--betw" => match toks.get(i + 1) {
                Some(v) => {
                    betw = Some(v.clone());
                    i += 1;
                }
                None => bad = true,
            },
            // `-f` is a harmless no-op on a file target; `-dir` requires
            // a directory.
            "-f" | "--file" if mode != ViewMode::Dirs => {}
            "-dir" | "--directory" => bad = true,
            // Any other flag (incl. a well-formed `--sort size`) is not
            // valid with a file target; consume `--sort`'s value so it
            // isn't mistaken for a second bare token.
            "--sort" => {
                bad = true;
                i += 1;
            }
            other if other.starts_with('-') => bad = true,
            other => {
                if file.is_some() {
                    // Two bare tokens: not a file view (directory flow
                    // reports the second as an unknown option, as before).
                    return None;
                }
                file = Some(other.to_string());
            }
        }
        i += 1;
    }
    let name = file?;
    let disk = resolve_view_operand(&name, nav)?;
    if !disk.is_file() {
        return None;
    }
    // A file target only allows display-range flags; anything else
    // (including malformed ranges) is a usage error.
    if bad {
        return Some(Err(()));
    }
    Some(Ok(FileView {
        path: disk,
        head,
        tail,
        betw,
    }))
}

/// Parsed `view` display flags (pure, unit-testable).
#[derive(Debug, Default)]
pub(crate) struct ViewFlags {
    pub show_filesize: bool,
    pub show_dirsize: bool,
    pub depth_override: Option<usize>,
    pub copy_to_clipboard: bool,
    pub care_sizes: bool,
    pub show_time: bool,
    pub file_min: Option<u64>,
    pub dir_min: Option<u64>,
    pub sort: Option<crate::explorer::TreeSortKey>,
    pub head: Option<usize>,
    pub tail: Option<usize>,
    pub file_only: bool,
    pub dir_only: bool,
}

/// Parse `view` flag tokens. `Err((key, arg))` is a lang key plus the
/// offending value for `print_error(&t!(key, arg))`.
pub(crate) fn parse_view_flags(parts: &[String]) -> Result<ViewFlags, (String, String)> {
    let mut f = ViewFlags::default();
    let mut i = 0;
    while i < parts.len() {
        match parts[i].as_str() {
            "-s" | "--size" => {
                // Optional minimum-size value: `-s 10MB` filters and shows,
                // bare `-s` keeps the old show-only behavior.
                if let Some(min) = next_size_value(parts, i) {
                    f.file_min = Some(min);
                    f.dir_min = Some(min);
                    f.show_filesize = true;
                    f.show_dirsize = true;
                    i += 1;
                } else {
                    f.show_filesize = true;
                    f.show_dirsize = true;
                }
            }
            "-fs" | "--filesize" => {
                if let Some(min) = next_size_value(parts, i) {
                    f.file_min = Some(min);
                    f.show_filesize = true;
                    i += 1;
                } else {
                    f.show_filesize = true;
                }
            }
            "-ds" | "--dirsize" => {
                if let Some(min) = next_size_value(parts, i) {
                    f.dir_min = Some(min);
                    f.show_dirsize = true;
                    i += 1;
                } else {
                    f.show_dirsize = true;
                }
            }
            "--head" => match next_count(parts, i) {
                Some(n) => {
                    f.head = Some(n);
                    i += 1;
                }
                None => {
                    return Err((
                        "cmd.report.invalid_count".to_string(),
                        parts.get(i + 1).cloned().unwrap_or_default(),
                    ));
                }
            },
            "--tail" => match next_count(parts, i) {
                Some(n) => {
                    f.tail = Some(n);
                    i += 1;
                }
                None => {
                    return Err((
                        "cmd.report.invalid_count".to_string(),
                        parts.get(i + 1).cloned().unwrap_or_default(),
                    ));
                }
            },
            "--sort" => match parts.get(i + 1).map(|s| s.to_lowercase()) {
                Some(k) if k == "size" => {
                    f.sort = Some(crate::explorer::TreeSortKey::Size);
                    i += 1;
                }
                Some(k) if k == "char" => {
                    f.sort = Some(crate::explorer::TreeSortKey::Char);
                    i += 1;
                }
                Some(k) if k == "ext" => {
                    f.sort = Some(crate::explorer::TreeSortKey::Ext);
                    i += 1;
                }
                _ => {
                    return Err((
                        "cmd.report.invalid_sort".to_string(),
                        parts.get(i + 1).cloned().unwrap_or_default(),
                    ));
                }
            },
            "-c" | "--care" => f.care_sizes = true,
            "-cp" | "--copy" => f.copy_to_clipboard = true,
            "-f" | "--file" => f.file_only = true,
            "-dir" | "--directory" => f.dir_only = true,
            "--time" | "-t" => f.show_time = true,
            "-d" | "--depth" => {
                if i + 1 < parts.len() {
                    if let Ok(depth) = parts[i + 1].parse::<usize>() {
                        f.depth_override = Some(depth);
                        i += 1;
                    } else {
                        print_error(&t!("cmd.report.invalid_depth", parts[i + 1]));
                    }
                }
            }
            other => {
                return Err(("cmd.report.unknown_option".to_string(), other.to_string()));
            }
        }
        i += 1;
    }
    Ok(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ext_filter_splits_exts_and_keeps_path() {
        let cwd = PathBuf::from("/tmp/ntc_ext_test");
        let (exts, seen, cleaned) = extract_ext_filter("-ext rs c", &cwd);
        assert_eq!(exts, vec!["rs", "c"]);
        assert!(seen);
        assert!(!cleaned.contains("-ext"));

        let (exts, seen, cleaned) = extract_ext_filter("src -ext rs --dirsize", &cwd);
        assert_eq!(exts, vec!["rs"]);
        assert!(seen);
        assert!(cleaned.contains("src"));
        assert!(cleaned.contains("--dirsize"));
    }

    #[test]
    fn ext_filter_stops_at_existing_dir() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("src")).unwrap();
        let (exts, _, cleaned) = extract_ext_filter("-ext rs src", dir.path());
        assert_eq!(exts, vec!["rs"]);
        assert!(cleaned.contains("src"));
    }

    #[test]
    fn parse_view_flags_size_thresholds() {
        let v = |s: &[&str]| s.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        // Bare flags keep show-only behavior.
        let f = parse_view_flags(&v(&["-s"])).unwrap();
        assert!(f.show_filesize && f.show_dirsize);
        assert!(f.file_min.is_none() && f.dir_min.is_none());
        // Valued flags filter and show.
        let f = parse_view_flags(&v(&["-s", "10MB"])).unwrap();
        assert_eq!(f.file_min, Some(10 * 1024 * 1024));
        assert_eq!(f.dir_min, Some(10 * 1024 * 1024));
        assert!(f.show_filesize && f.show_dirsize);
        let f = parse_view_flags(&v(&["-fs", "1KB"])).unwrap();
        assert_eq!(f.file_min, Some(1024));
        assert!(f.dir_min.is_none());
        assert!(f.show_filesize && !f.show_dirsize);
        let f = parse_view_flags(&v(&["-ds", "2GB"])).unwrap();
        assert_eq!(f.dir_min, Some(2 * 1024 * 1024 * 1024));
        assert!(f.file_min.is_none());
        assert!(!f.show_filesize && f.show_dirsize);
        // Non-size next token stays bare (path handled upstream).
        let f = parse_view_flags(&v(&["-s"])).unwrap();
        assert!(f.file_min.is_none());
    }

    #[test]
    fn parse_view_flags_head_tail_sort() {
        let v = |s: &[&str]| s.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        let f = parse_view_flags(&v(&["--head", "5"])).unwrap();
        assert_eq!(f.head, Some(5));
        assert!(f.tail.is_none());
        let f = parse_view_flags(&v(&["--tail", "3"])).unwrap();
        assert_eq!(f.tail, Some(3));
        let f = parse_view_flags(&v(&["--sort", "size"])).unwrap();
        assert_eq!(f.sort, Some(crate::explorer::TreeSortKey::Size));
        let f = parse_view_flags(&v(&["--sort", "EXT"])).unwrap();
        assert_eq!(f.sort, Some(crate::explorer::TreeSortKey::Ext));
        // Both head+tail parse (head wins at render).
        let f = parse_view_flags(&v(&["--head", "5", "--tail", "3"])).unwrap();
        assert_eq!((f.head, f.tail), (Some(5), Some(3)));
        // Errors carry keys.
        assert!(parse_view_flags(&v(&["--head", "0"])).is_err());
        assert!(parse_view_flags(&v(&["--head"])).is_err());
        assert!(parse_view_flags(&v(&["--tail", "x"])).is_err());
        assert!(parse_view_flags(&v(&["--sort", "date"])).is_err());
        assert!(parse_view_flags(&v(&["--nope"])).is_err());
    }

    #[test]
    fn parse_view_flags_copy_care_and_kind_aliases() {
        let v = |s: &[&str]| s.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        // `-c` is care now; copy moved to `-cp`/`--copy`.
        let f = parse_view_flags(&v(&["-c"])).unwrap();
        assert!(f.care_sizes && !f.copy_to_clipboard);
        let f = parse_view_flags(&v(&["-cp"])).unwrap();
        assert!(f.copy_to_clipboard && !f.care_sizes);
        let f = parse_view_flags(&v(&["--copy"])).unwrap();
        assert!(f.copy_to_clipboard);
        // Legacy `--cp` is gone.
        assert!(parse_view_flags(&v(&["--cp"])).is_err());
        // Exact-node kind filters.
        let f = parse_view_flags(&v(&["-f"])).unwrap();
        assert!(f.file_only && !f.dir_only);
        let f = parse_view_flags(&v(&["--directory"])).unwrap();
        assert!(f.dir_only && !f.file_only);
    }

    #[test]
    fn split_view_file_args_routes_files_only() {
        let _cwd =
            crate::utils::test_support::TempCwd::with_fixture(&[("a.txt", "1\n2\n3\n")], &["sub"]);
        let nav = Navigator::new().unwrap();
        // File with range flags, either order.
        for args in ["a.txt --head 2", "--tail 2 a.txt", "a.txt --betw 1:2"] {
            match split_view_file_args(args, &nav, ViewMode::All) {
                Some(Ok(fv)) => assert!(fv.path.is_file()),
                other => panic!("{} should route to file, got {:?}", args, other.is_some()),
            }
        }
        // Directories and missing paths stay in directory flow.
        assert!(split_view_file_args("sub", &nav, ViewMode::All).is_none());
        assert!(split_view_file_args("ghost.txt", &nav, ViewMode::All).is_none());
        assert!(split_view_file_args("--head 5", &nav, ViewMode::All).is_none());
        // Size/sort/ext flags with a file are usage errors.
        assert!(matches!(
            split_view_file_args("a.txt -s", &nav, ViewMode::All),
            Some(Err(()))
        ));
        assert!(matches!(
            split_view_file_args("a.txt --sort size", &nav, ViewMode::All),
            Some(Err(()))
        ));
        assert!(matches!(
            split_view_file_args("a.txt --head x", &nav, ViewMode::All),
            Some(Err(()))
        ));
        // Two bare tokens: directory flow (unknown-option there).
        assert!(split_view_file_args("a.txt sub", &nav, ViewMode::All).is_none());
        // `-f` is a no-op on files; `-dir` is invalid.
        assert!(matches!(
            split_view_file_args("a.txt -f", &nav, ViewMode::Files),
            Some(Ok(_))
        ));
        assert!(matches!(
            split_view_file_args("a.txt -dir", &nav, ViewMode::All),
            Some(Err(()))
        ));
    }

    #[test]
    fn split_view_file_args_resolves_teleport_subpaths() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(
            &[("tp1/dir/file.txt", "hello"), ("tp1/top.txt", "top")],
            &["tp1", "tp1/dir"],
        );
        let root = std::env::current_dir().unwrap();
        let mut cfg = crate::config::Config::default();
        cfg.teleports.insert("tp1".to_string(), root.join("tp1"));
        crate::config::Config::with_test_config(cfg, || {
            let nav = Navigator::new().unwrap();
            match split_view_file_args("@tp1/top.txt", &nav, ViewMode::All) {
                Some(Ok(fv)) => assert_eq!(fv.path, root.join("tp1").join("top.txt")),
                _ => panic!("@tp1/top.txt should route to file"),
            }
            match split_view_file_args("@tp1/dir/file.txt", &nav, ViewMode::Files) {
                Some(Ok(fv)) => {
                    assert_eq!(fv.path, root.join("tp1").join("dir").join("file.txt"))
                }
                _ => panic!("nested teleport file should route to file"),
            }
            assert!(split_view_file_args("@tp1/dir", &nav, ViewMode::All).is_none());
            assert!(split_view_file_args("@ghost/file.txt", &nav, ViewMode::All).is_none());
        });
    }

    #[test]
    fn txtf_resolves_disk_and_teleport_directories() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(
            &[
                ("sub/note.txt", "hello"),
                ("tp1/dir/file.txt", "file"),
                ("tp1/top.txt", "top"),
            ],
            &["sub", "tp1", "tp1/dir"],
        );
        let root = std::env::current_dir().unwrap();
        let mut cfg = crate::config::Config::default();
        cfg.teleports.insert("tp1".to_string(), root.join("tp1"));
        crate::config::Config::with_test_config(cfg, || {
            let nav = Navigator::new().unwrap();
            assert!(resolve_view_operand("sub", &nav).unwrap().is_dir());
            assert!(resolve_view_operand("@tp1", &nav).unwrap().is_dir());
            assert!(resolve_view_operand("@tp1/dir", &nav).unwrap().is_dir());
            assert!(resolve_view_operand("@tp1/dir/file.txt", &nav)
                .unwrap()
                .is_file());
        });
    }

    #[test]
    fn ext_filter_flags_value_flags_are_skipped() {
        let cwd = PathBuf::from("/tmp/ntc_ext_test");
        let (exts, seen, cleaned) = extract_ext_filter("-d 2 -ext rs -p somewhere", &cwd);
        assert_eq!(exts, vec!["rs"]);
        assert!(seen);
        assert!(cleaned.contains("-d"));
        assert!(cleaned.contains("-p"));

        // Bare `-ext` with no extensions is reported via the seen flag.
        let (exts, seen, _) = extract_ext_filter("-ext -ds", &cwd);
        assert!(seen);
        assert!(exts.is_empty());

        // `-e` is the short extension alias.
        let (exts, seen, cleaned) = extract_ext_filter("-e rs,md", &cwd);
        assert_eq!(exts, vec!["rs", "md"]);
        assert!(seen);
        assert!(!cleaned.contains("-e"));
    }
}
