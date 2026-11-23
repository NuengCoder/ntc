//! Shared argument parsers for filesystem and content search commands.

use crate::config::Config;
use crate::search::{ContextSpec, MatchMode, MatchOptions};

/// Shared arg parser for fs / ds / locate.
/// Accepts:  <pattern> [-d <depth>] [--no-fuzzy|-no-fz]   (depth flag must be
/// at the end). Returns `(pattern, max_depth, fuzzy)`.
/// Handles double-quoted patterns: fs "my file.rs" -d 3
pub(crate) fn parse_search_args(args: &str) -> (String, usize, bool) {
    let default_depth = Config::global_get_max_depth();

    // Tokenize respecting double quotes
    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in args.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    parts.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }

    // `--no-fuzzy` / `-no-fz` disables the Jaro-Winkler fallback.
    let mut fuzzy = true;
    let mut kept: Vec<String> = Vec::with_capacity(parts.len());
    for part in parts {
        if part.eq_ignore_ascii_case("--no-fuzzy") || part.eq_ignore_ascii_case("-no-fz") {
            fuzzy = false;
        } else {
            kept.push(part);
        }
    }
    let parts = kept;

    // Look for -d <n> at the end: ["pattern", ..., "-d", "3"]
    let n = parts.len();
    if n >= 3 && parts[n - 2] == "-d" {
        if let Ok(depth) = parts[n - 1].parse::<usize>() {
            let pattern = parts[..n - 2].join(" ");
            return (pattern, depth, fuzzy);
        }
    }

    (parts.join(" "), default_depth, fuzzy)
}

/// Parsed arguments for `gs`.
pub(crate) struct GsArgs {
    pub pattern: String,
    pub max_depth: usize,
    pub limit: Option<usize>,
    pub interactive: bool,
    /// File-scope globs (`-p` / positional `*.*` / `dir/`).
    pub paths: Vec<String>,
    /// `None` = default order (path, then line); `Some` = explicit `-s`.
    pub sort: Option<GsSort>,
    /// Lines of context around each match (`-A` / `-B`).
    pub context: ContextSpec,
    /// Whether `-A`/`-B` was actually given (`-A 0 -B 0` is still explicit).
    pub context_requested: bool,
    /// `-x` / `-h` / `-g` / `-w` / `-z` matching rules.
    pub opts: MatchOptions,
    /// Two of `-h` / `-g` / `-w` were given: `gs` reports the conflict.
    pub mode_conflict: bool,
    /// Lowercase `-i`: the pager flag is `-I` now.
    pub legacy_interactive: bool,
}

/// Sort key for `gs -s|--sort`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GsSort {
    /// Match-line text, case-insensitive (default for bare `-s`).
    Line,
    /// File path, then line number.
    Path,
    /// File size ascending (small → large), tie-break path + line.
    Size,
}

fn tokenize_gs_args(args: &str) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in args.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    parts.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

/// Parse `gs` args in any flag order:
/// `gs <pattern> [-d <n>] [-l <n> | --limit <n> | --limit=<n>] [-I]
///   [-p <glob> | --path <glob> | --path=<glob>]... [-s [line|path|size] | --sort ...]
///   [-x] [-h|-g|-w] [-z]`
/// Positional path-like tokens (`*.*`, `dir/`, existing file/dir) after the
/// first pattern token act as `-p` filters; everything else joins the pattern.
pub(crate) fn parse_gs_args(args: &str) -> GsArgs {
    let default_depth = Config::global_get_max_depth();
    let parts = tokenize_gs_args(args);
    let mut pattern_parts: Vec<String> = Vec::new();
    let mut max_depth = default_depth;
    let mut limit: Option<usize> = None;
    let mut interactive = false;
    let mut paths: Vec<String> = Vec::new();
    let mut sort: Option<GsSort> = None;
    let mut context = ContextSpec::default();
    let mut context_requested = false;
    let mut opts = MatchOptions::default();
    let mut mode_conflict = false;
    let mut legacy_interactive = false;

    let mut i = 0;
    while i < parts.len() {
        match parts[i].as_str() {
            "-I" => {
                interactive = true;
            }
            "-i" => legacy_interactive = true,
            "-x" => opts.literal = true,
            "-z" => opts.case_sensitive = true,
            "-h" | "-g" | "-w" => {
                let mode = match parts[i].as_str() {
                    "-h" => MatchMode::StartsWith,
                    "-g" => MatchMode::EndsWith,
                    _ => MatchMode::Whole,
                };
                // `-h`, `-g` and `-w` are exclusive; a repeat of the same one
                // is harmless.
                if opts.mode != MatchMode::Contains && opts.mode != mode {
                    mode_conflict = true;
                } else {
                    opts.mode = mode;
                }
            }
            "-d" => {
                if let Some(v) = parts.get(i + 1).and_then(|s| s.parse::<usize>().ok()) {
                    max_depth = v;
                    i += 1;
                } else {
                    pattern_parts.push(parts[i].clone());
                }
            }
            "-l" | "--limit" => {
                if let Some(v) = parts.get(i + 1).and_then(|s| s.parse::<usize>().ok()) {
                    if v > 0 {
                        limit = Some(v);
                    }
                    i += 1;
                } else {
                    pattern_parts.push(parts[i].clone());
                }
            }
            s if s.starts_with("--limit=") => {
                if let Some(v) = s["--limit=".len()..].parse::<usize>().ok() {
                    if v > 0 {
                        limit = Some(v);
                    }
                } else {
                    pattern_parts.push(parts[i].clone());
                }
            }
            "-A" | "--after" => {
                if let Some(v) = parts.get(i + 1).and_then(|s| s.parse::<usize>().ok()) {
                    context.after = v;
                    context_requested = true;
                    i += 1;
                } else {
                    pattern_parts.push(parts[i].clone());
                }
            }
            "-B" | "--before" => {
                if let Some(v) = parts.get(i + 1).and_then(|s| s.parse::<usize>().ok()) {
                    context.before = v;
                    context_requested = true;
                    i += 1;
                } else {
                    pattern_parts.push(parts[i].clone());
                }
            }
            s if s.starts_with("--after=") => {
                if let Ok(v) = s["--after=".len()..].parse::<usize>() {
                    context.after = v;
                    context_requested = true;
                } else {
                    pattern_parts.push(parts[i].clone());
                }
            }
            s if s.starts_with("--before=") => {
                if let Ok(v) = s["--before=".len()..].parse::<usize>() {
                    context.before = v;
                    context_requested = true;
                } else {
                    pattern_parts.push(parts[i].clone());
                }
            }
            s if s.starts_with("-p=") || s.starts_with("--path=") => {
                paths.push(
                    s.split_once('=')
                        .map(|(_, v)| v)
                        .unwrap_or_default()
                        .to_string(),
                );
            }
            "-p" | "--path" => {
                if let Some(next) = parts.get(i + 1) {
                    if !next.starts_with('-') {
                        paths.push(next.clone());
                        i += 1;
                    } else {
                        pattern_parts.push(parts[i].clone());
                    }
                } else {
                    pattern_parts.push(parts[i].clone());
                }
            }
            s if s.starts_with("--sort=") => match parse_gs_sort_value(&s["--sort=".len()..]) {
                Some(k) => sort = Some(k),
                None => pattern_parts.push(parts[i].clone()),
            },
            "-s" | "--sort" => {
                if let Some(next) = parts.get(i + 1) {
                    match parse_gs_sort_value(next) {
                        Some(k) => {
                            sort = Some(k);
                            i += 1;
                        }
                        // A value other than line|path|size remains part of
                        // the search pattern; bare -s always means line sort.
                        None => sort = Some(GsSort::Line),
                    }
                } else {
                    sort = Some(GsSort::Line);
                }
            }
            other => {
                // Positional `gs <pattern> <*.* + dir/>`: tokens after the
                // pattern head that look like paths become `-p` filters.
                if !pattern_parts.is_empty() && gs_looks_like_path(other) {
                    paths.push(other.to_string());
                } else {
                    pattern_parts.push(other.to_string());
                }
            }
        }
        i += 1;
    }

    GsArgs {
        pattern: pattern_parts.join(" "),
        max_depth,
        limit,
        interactive,
        paths,
        sort,
        context,
        context_requested,
        opts,
        mode_conflict,
        legacy_interactive,
    }
}

/// `line|path|size` (case-insensitive).
fn parse_gs_sort_value(v: &str) -> Option<GsSort> {
    match v.to_lowercase().as_str() {
        "line" => Some(GsSort::Line),
        "path" => Some(GsSort::Path),
        "size" => Some(GsSort::Size),
        _ => None,
    }
}

/// Heuristic: `*.rs`, `src/`, `app.log`, existing file/dir.
fn gs_looks_like_path(tok: &str) -> bool {
    // An OR alternative (`a\|b`, `a|b`) belongs to the pattern even though the
    // escaped form carries a backslash — never treat it as an `-p` filter.
    if tok.contains('|') && !tok.contains('/') && !tok.contains('*') && !tok.contains('?') {
        return false;
    }
    if tok.contains('*') || tok.contains('?') || tok.contains('/') || tok.contains('\\') {
        return true;
    }
    if tok.starts_with('.') && tok.len() > 1 && !tok.contains(' ') {
        return true;
    }
    if !tok.contains(' ') {
        if let Some(dot) = tok.rfind('.') {
            let ext = &tok[dot + 1..];
            if !ext.is_empty() && ext.len() <= 5 && ext.chars().all(|c| c.is_ascii_alphanumeric()) {
                let stem = &tok[..dot];
                if !stem.is_empty()
                    && stem
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
                {
                    return true;
                }
            }
        }
    }
    let p = std::path::Path::new(tok);
    p.is_file() || p.is_dir()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gs_parses_path_and_sort_flags() {
        let a = parse_gs_args("main -p \"*.rs\" -s size");
        assert_eq!(a.pattern, "main");
        assert_eq!(a.paths, vec!["*.rs".to_string()]);
        assert_eq!(a.sort, Some(GsSort::Size));

        let a = parse_gs_args("main -p=*.rs --path=src/*.rs");
        assert_eq!(a.paths, vec!["*.rs".to_string(), "src/*.rs".to_string()]);

        let b = parse_gs_args("main -s");
        assert_eq!(b.pattern, "main");
        assert_eq!(b.sort, Some(GsSort::Line));

        let b = parse_gs_args("-s main");
        assert_eq!(b.pattern, "main");
        assert_eq!(b.sort, Some(GsSort::Line));

        let c = parse_gs_args("main -s path");
        assert_eq!(c.sort, Some(GsSort::Path));

        let c = parse_gs_args("--sort=size main");
        assert_eq!(c.pattern, "main");
        assert_eq!(c.sort, Some(GsSort::Size));

        let c = parse_gs_args("main -s yesterday");
        assert_eq!(c.pattern, "main yesterday");
        assert_eq!(c.sort, Some(GsSort::Line));

        let d = parse_gs_args("main -p *.rs -p src/");
        assert_eq!(d.paths, vec!["*.rs".to_string(), "src/".to_string()]);

        // Positional `gs <pattern> <*.* + dir/>` == `gs <pattern> -p ...`.
        let e = parse_gs_args("main *.rs");
        assert_eq!(e.pattern, "main");
        assert_eq!(e.paths, vec!["*.rs".to_string()]);

        let e = parse_gs_args("\"foo bar\"");
        assert_eq!(e.pattern, "foo bar");

        // Multi-word patterns still join; flags keep working in any order.
        let f = parse_gs_args("-s size -d 3 foo bar");
        assert_eq!(f.pattern, "foo bar");
        assert_eq!(f.sort, Some(GsSort::Size));
        assert_eq!(f.max_depth, 3);
        assert!(f.paths.is_empty());
    }

    #[test]
    fn parse_gs_args_accepts_limit_flags() {
        let a = parse_gs_args("main -d 5 -l 10");
        assert_eq!(a.pattern, "main");
        assert_eq!(a.max_depth, 5);
        assert_eq!(a.limit, Some(10));
        let b = parse_gs_args("--limit=7 main -I");
        assert_eq!(b.pattern, "main");
        assert_eq!(b.limit, Some(7));
        assert!(b.interactive);
    }

    #[test]
    fn parse_gs_args_reads_match_flags() {
        let a = parse_gs_args("main -x");
        assert_eq!(a.pattern, "main");
        assert!(a.opts.literal);
        assert!(!a.opts.case_sensitive);
        assert_eq!(a.opts.mode, MatchMode::Contains);

        let b = parse_gs_args("-z Main -h");
        assert_eq!(b.pattern, "Main");
        assert!(b.opts.case_sensitive);
        assert_eq!(b.opts.mode, MatchMode::StartsWith);

        assert_eq!(parse_gs_args("x -g").opts.mode, MatchMode::EndsWith);
        assert_eq!(parse_gs_args("x -w").opts.mode, MatchMode::Whole);
        assert_eq!(parse_gs_args("x -x -z -w").opts.mode, MatchMode::Whole);
        assert!(!parse_gs_args("x -x -z -w").mode_conflict);

        // `-h` / `-g` / `-w` are exclusive.
        let c = parse_gs_args("x -h -g");
        assert!(c.mode_conflict);
        assert_eq!(c.opts.mode, MatchMode::StartsWith);
        assert!(parse_gs_args("x -w -h").mode_conflict);

        // The pager flag is `-I`; lowercase `-i` is reported, not searched.
        assert!(parse_gs_args("-I").interactive);
        let d = parse_gs_args("-i");
        assert!(d.legacy_interactive);
        assert!(!d.interactive);
        assert!(d.pattern.is_empty());
    }

    #[test]
    fn parse_gs_args_allows_interactive_without_pattern() {
        let a = parse_gs_args("-I");
        assert!(a.interactive);
        assert!(a.pattern.is_empty());

        let b = parse_gs_args("-I -d 3");
        assert!(b.interactive);
        assert!(b.pattern.is_empty());
        assert_eq!(b.max_depth, 3);
    }

    #[test]
    fn parse_gs_args_accepts_context_flags() {
        let a = parse_gs_args("main -A 3 -B 2");
        assert_eq!(a.pattern, "main");
        assert_eq!(a.context.after, 3);
        assert_eq!(a.context.before, 2);
        assert!(a.context.is_active());

        let b = parse_gs_args("--after=1 --before=4 main");
        assert_eq!(b.pattern, "main");
        assert_eq!((b.context.before, b.context.after), (4, 1));

        // Long names work in any order, like the other flags.
        let c = parse_gs_args("-B 2 main --after 5");
        assert_eq!(c.pattern, "main");
        assert_eq!((c.context.before, c.context.after), (2, 5));

        assert!(!parse_gs_args("main").context.is_active());
        assert!(!parse_gs_args("main").context_requested);
        assert!(a.context_requested);
        assert!(parse_gs_args("main -A 0 -B 0").context_requested);

        // A missing value stays pattern text, exactly like -d and -l.
        let d = parse_gs_args("main -A");
        assert_eq!(d.pattern, "main -A");
        assert!(!d.context.is_active());
    }

    #[test]
    fn parse_gs_args_keeps_or_alternatives_in_the_pattern() {
        let a = parse_gs_args("error\\|warning -d 2");
        assert_eq!(a.pattern, "error\\|warning");
        assert_eq!(a.max_depth, 2);

        // A later OR alternative stays pattern text even though it carries a
        // backslash, which otherwise marks a path filter.
        let b = parse_gs_args("error warning\\|fatal");
        assert_eq!(b.pattern, "error warning\\|fatal");
        assert!(b.paths.is_empty());

        // Real path filters are still recognised after an OR pattern.
        let c = parse_gs_args("error\\|warning -p \"*.rs\"");
        assert_eq!(c.pattern, "error\\|warning");
        assert_eq!(c.paths, vec!["*.rs".to_string()]);
    }
}
