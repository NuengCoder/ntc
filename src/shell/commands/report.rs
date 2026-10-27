//! Shell commands for directory tree display (`view`) and report generation
//! (`report` with subcommands: txt, json, md, html, pdf, docx, xlsx).
//! Also provides `txtc` (copy file) and `txtf` (display file).

use super::{
    list_supported_files, list_supported_files_in, show_file_selection_menu,
    show_file_selection_menu_range,
};
use crate::config::Config;
use crate::filetype::is_supported_format;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::report::{generate_report_to, ReportFormat};
use crate::shell::helpers::show_tree_at_ex;
use crate::t;
use crate::teleport::TeleportManager;

use colored::*;

use anyhow::Result;
use std::path::{Path, PathBuf};

/// Quote-aware tokenizer that also returns byte spans (which include any
/// surrounding quotes, so spans can be blanked out of the original string).
fn tokenize_spans(args: &str) -> Vec<(String, usize, usize)> {
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
fn extract_ext_filter(args: &str, cwd: &Path) -> (Vec<String>, bool, String) {
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
fn next_size_value(parts: &[String], i: usize) -> Option<u64> {
    parts
        .get(i + 1)
        .and_then(|v| crate::explorer::parse_size_value(v))
}

/// Required positive line count after `--head`/`--tail`.
fn next_count(parts: &[String], i: usize) -> Option<usize> {
    parts
        .get(i + 1)
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&n| n > 0)
}

/// Which entries `view`/`viewf`/`viewd` render.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    All,
    Files,
    Dirs,
}

/// A `<view|viewf> <file>` request: display the file like `txtf`.
/// Only `--head`/`--tail`/`--betw` may accompany a file target.
#[derive(Debug)]
struct FileView {
    path: PathBuf,
    head: Option<usize>,
    tail: Option<usize>,
    betw: Option<String>,
}

/// Resolve a `view`/`txtf` operand: `@name[/sub...]` teleport paths or a
/// disk path relative to the navigator's directory.
fn resolve_view_operand(name: &str, nav: &Navigator) -> Option<PathBuf> {
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
fn split_view_file_args(
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

pub fn cmd_view(args: &str, nav: &mut Navigator) -> Result<bool> {
    cmd_view_impl(ViewMode::All, args, nav)
}

pub fn cmd_viewf(args: &str, nav: &mut Navigator) -> Result<bool> {
    cmd_view_impl(ViewMode::Files, args, nav)
}

pub fn cmd_viewd(args: &str, nav: &mut Navigator) -> Result<bool> {
    cmd_view_impl(ViewMode::Dirs, args, nav)
}

/// `views [path] [-n] [--size <min>] [--sort <name|ext|size>]` — the short
/// `view`: a flat `ls`-style listing of one directory level instead of the
/// tree. There are no `[Directory]` / `[ignored]` tags (colour carries
/// that), the depth is fixed at 1, sizes are always computed with `--care`
/// semantics, and `--size` hides entries below the given size.
///
/// Bare `views` prints names only, flowing inline in four groups (dirs,
/// files, ignored dirs, ignored files) joined by `...` and wrapped to the
/// terminal width. `-n` switches to one entry per line.
pub fn cmd_views(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parsed = match crate::shell::helpers::parse_target_arg(args, nav) {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };
    let mut min: Option<u64> = None;
    let mut show_size = false;
    let mut sort = crate::explorer::TreeSortKey::Char;
    let mut newline = false;
    let mut i = 0;
    while i < parsed.rest.len() {
        match parsed.rest[i].to_lowercase().as_str() {
            "-n" => newline = true,
            "-s" | "--size" => {
                show_size = true;
                if let Some(v) = next_size_value(&parsed.rest, i) {
                    min = Some(v);
                    i += 1;
                }
            }
            "--sort" => {
                let Some(key) = parsed.rest.get(i + 1).map(|s| s.to_lowercase()) else {
                    print_error(&t!("cmd.views.invalid_sort"));
                    println!("{}", t!("cmd.views.usage"));
                    return Ok(false);
                };
                sort = match key.as_str() {
                    "name" | "char" | "n" => crate::explorer::TreeSortKey::Char,
                    "ext" | "e" => crate::explorer::TreeSortKey::Ext,
                    "size" | "s" => crate::explorer::TreeSortKey::Size,
                    _ => {
                        print_error(&t!("cmd.views.invalid_sort_value", key));
                        println!("{}", t!("cmd.views.usage"));
                        return Ok(false);
                    }
                };
                i += 1;
            }
            other => {
                print_error(&t!("cmd.report.unknown_option", other));
                println!("{}", t!("cmd.views.usage"));
                return Ok(false);
            }
        }
        i += 1;
    }

    let target = parsed
        .path
        .unwrap_or_else(|| nav.current_path().to_path_buf());
    let mut tree =
        crate::explorer::generate_tree(target.to_string_lossy().as_ref(), Some(1), true, None);
    // Sizes are always computed, and always with care semantics.
    crate::explorer::compute_tree_sizes(&mut tree, None, true);
    if let Some(min) = min {
        tree.children.retain(|n| n.size.unwrap_or(0) >= min);
    }
    let ext_of = |name: &str| match name.rfind('.') {
        Some(i) if i + 1 < name.len() => name[i + 1..].to_lowercase(),
        _ => String::new(),
    };
    match sort {
        crate::explorer::TreeSortKey::Char => tree
            .children
            .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        crate::explorer::TreeSortKey::Ext => tree.children.sort_by(|a, b| {
            ext_of(&a.name)
                .cmp(&ext_of(&b.name))
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        }),
        crate::explorer::TreeSortKey::Size => tree.children.sort_by(|a, b| {
            b.size
                .unwrap_or(0)
                .cmp(&a.size.unwrap_or(0))
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        }),
    }

    if tree.children.is_empty() {
        println!("{}", t!("cmd.views.empty"));
        return Ok(false);
    }

    let theme = crate::utils::theme::ThemeManager::current();
    let c_dir = theme.shell.tree_dir.clone();
    let c_ignored = theme.shell.tree_ignored.clone();
    drop(theme);

    let paint = |node: &crate::explorer::TreeNode, text: &str| -> String {
        if node.is_ignored {
            c_ignored.paint_ansi(text)
        } else if node.is_dir {
            c_dir.paint_ansi(text)
        } else {
            text.to_string()
        }
    };

    // Four display groups, in this order: plain dirs, plain files,
    // ignored dirs, ignored files. Empty groups are dropped.
    // When `--size` is set each entry is shown as `name (size)`; the
    // `visible` width includes the size so line-wrapping in bare mode
    // stays correct.
    let mut groups: Vec<Vec<(usize, String)>> = Vec::new();
    for (is_dir, is_ignored) in [(true, false), (false, false), (true, true), (false, true)] {
        let names: Vec<(usize, String)> = tree
            .children
            .iter()
            .filter(|n| n.is_dir == is_dir && n.is_ignored == is_ignored)
            .map(|n| {
                let painted = paint(n, &n.name);
                if show_size {
                    let s = crate::explorer::human_readable_size(n.size.unwrap_or(0));
                    let text = format!("{} ({})", painted, s);
                    let visible = n.name.chars().count() + 1 + 1 + s.chars().count() + 1;
                    (visible, text)
                } else {
                    (n.name.chars().count(), painted)
                }
            })
            .collect();
        if !names.is_empty() {
            groups.push(names);
        }
    }

    // `-n`: one entry per line.
    if newline {
        for group in &groups {
            for (_, name) in group {
                println!("{}", name);
            }
        }
        return Ok(false);
    }

    // Bare `views`: names flow inline, groups joined by `...`, wrapped to
    // the terminal width.
    let width = crossterm::terminal::size()
        .map(|(w, _)| w as usize)
        .unwrap_or(100)
        .max(20);
    let mut line = String::new();
    let mut line_w = 0usize;
    let mut first_group = true;
    for group in groups {
        if !first_group {
            views_flow_push(&mut line, &mut line_w, 3, "...", width);
        }
        first_group = false;
        for (visible, name) in group {
            views_flow_push(&mut line, &mut line_w, visible, &name, width);
        }
    }
    if !line.is_empty() {
        println!("{}", line);
    }
    Ok(false)
}

/// Append one already-painted word to the flowing line, starting a new
/// output line when the word would not fit. `visible` is the word's width
/// in characters, measured before painting so ANSI escapes never count.
fn views_flow_push(
    line: &mut String,
    line_w: &mut usize,
    visible: usize,
    word: &str,
    width: usize,
) {
    if *line_w == 0 {
        line.push_str(word);
        *line_w = visible;
    } else if *line_w + 1 + visible > width {
        println!("{}", line);
        line.clear();
        line.push_str(word);
        *line_w = visible;
    } else {
        line.push(' ');
        line.push_str(word);
        *line_w += 1 + visible;
    }
}

fn view_usage_key(mode: ViewMode) -> &'static str {
    match mode {
        ViewMode::All => "help.view.file_usage",
        ViewMode::Files => "help.viewf.file_usage",
        ViewMode::Dirs => "help.viewd.usage",
    }
}

fn view_tree_usage_key(mode: ViewMode) -> &'static str {
    match mode {
        ViewMode::All => "cmd.report.view_usage",
        ViewMode::Files => "help.viewf.tree",
        ViewMode::Dirs => "help.viewd.usage",
    }
}

fn cmd_view_impl(mode: ViewMode, args: &str, nav: &mut Navigator) -> Result<bool> {
    let cwd = nav.current_path().to_path_buf();

    // `view -` displays piped / redirected input.
    if mode == ViewMode::All && args.trim() == "-" {
        if let Some(text) = crate::shell::stdin::read_to_string() {
            let show_lines = Config::global_get_show_line_numbers();
            if show_lines {
                for (i, line) in text.lines().enumerate() {
                    println!("{:>5} | {}", i + 1, line);
                }
            } else {
                print!("{}", text);
                if !text.is_empty() && !text.ends_with('\n') {
                    println!();
                }
            }
            return Ok(false);
        }
    }

    // `viewd` never filters by extension (directory rows only).
    let (exts, ext_seen, cleaned) = if mode == ViewMode::Dirs {
        (Vec::new(), false, args.to_string())
    } else {
        extract_ext_filter(args, &cwd)
    };
    if ext_seen && exts.is_empty() {
        print_error(&t!("cmd.report.ext_usage"));
        return Ok(false);
    }

    // `<view|viewf> <file>` displays the file like `txtf` (only
    // display-range flags allowed). Extension filters never apply to files.
    if mode != ViewMode::Dirs && !ext_seen {
        match split_view_file_args(&cleaned, nav, mode) {
            Some(Err(())) => {
                println!("{}", t!(view_usage_key(mode)));
                return Ok(false);
            }
            Some(Ok(fv)) => {
                let range =
                    match crate::output::parse_file_range(fv.head, fv.tail, fv.betw.as_deref()) {
                        Ok(r) => r,
                        Err(()) => {
                            print_error(&t!(
                                "cmd.report.invalid_range",
                                fv.betw.clone().unwrap_or_default()
                            ));
                            println!("{}", t!(view_usage_key(mode)));
                            return Ok(false);
                        }
                    };
                let label = fv.path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                display_txtf_file(&fv.path, range, label, None)?;
                return Ok(false);
            }
            None => {}
        }
    }

    let parsed = match mode {
        ViewMode::Dirs => crate::shell::helpers::parse_target_arg(&cleaned, nav),
        _ => crate::shell::helpers::parse_target_arg_allow_file(&cleaned, nav),
    };
    let parsed = match parsed {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };
    let parts: Vec<String> = parsed.rest;
    let f = match parse_view_flags(&parts) {
        Ok(f) => f,
        Err((key, arg)) => {
            print_error(&t!(key.as_str(), arg));
            println!("{}", t!(view_tree_usage_key(mode)));
            return Ok(false);
        }
    };

    let unsupported = match mode {
        ViewMode::All => f.file_only && f.dir_only,
        ViewMode::Files => f.dir_only || f.depth_override.is_some(),
        ViewMode::Dirs => f.file_only || f.dir_only || f.head.is_some() || f.tail.is_some(),
    };
    if unsupported {
        let flag = if f.file_only && f.dir_only {
            "-f/-dir"
        } else if mode == ViewMode::Files && f.depth_override.is_some() {
            "-d"
        } else {
            "flag"
        };
        print_error(&t!("cmd.report.unknown_option", flag));
        println!("{}", t!(view_tree_usage_key(mode)));
        return Ok(false);
    }

    // A file target with tree-only flags (e.g. `--ext`) is a usage error.
    if let Some(target) = &parsed.path {
        if target.is_file() {
            println!("{}", t!(view_usage_key(mode)));
            return Ok(false);
        }
    }

    let (show_filesize, show_dirsize, copy_to_clipboard, care_sizes, show_time) = (
        f.show_filesize,
        f.show_dirsize,
        f.copy_to_clipboard,
        f.care_sizes,
        f.show_time,
    );
    let (file_min, dir_min, sort, head, tail) = (f.file_min, f.dir_min, f.sort, f.head, f.tail);
    let entry_filter = match mode {
        ViewMode::Files => crate::explorer::TreeEntryFilter::FilesOnly,
        ViewMode::Dirs => crate::explorer::TreeEntryFilter::DirsOnly,
        ViewMode::All if f.file_only => crate::explorer::TreeEntryFilter::FilesOnly,
        ViewMode::All if f.dir_only => crate::explorer::TreeEntryFilter::DirsOnly,
        ViewMode::All => crate::explorer::TreeEntryFilter::All,
    };
    let max_depth = match mode {
        ViewMode::Files => 1,
        _ => f.depth_override.unwrap_or(Config::global_get_max_depth()),
    };
    // Directory sizes need the file nodes even when `viewd` hides them.
    let need_dir_sizes = show_dirsize
        || dir_min.is_some()
        || care_sizes
        || sort == Some(crate::explorer::TreeSortKey::Size);
    let include_files = mode != ViewMode::Dirs || need_dir_sizes;
    let start = std::time::Instant::now();
    let filter = if exts.is_empty() {
        None
    } else {
        Some(exts.as_slice())
    };
    let opts = crate::shell::helpers::TreeViewOpts {
        file_min,
        dir_min,
        sort,
        head,
        tail,
        entry_filter,
        sort_dirs_by_size: false,
        sort_files_by_size: false,
    };
    let target = parsed
        .path
        .unwrap_or_else(|| nav.current_path().to_path_buf());
    let matched = show_tree_at_ex(
        &target,
        Some(max_depth),
        true,
        include_files,
        show_filesize,
        show_dirsize,
        copy_to_clipboard,
        care_sizes,
        filter,
        &opts,
    );
    if filter.is_some() && matched == 0 {
        println!(
            "{}",
            t!("cmd.scan.no_files", crate::filetype::format_ext_list(&exts))
        );
    }
    if show_time {
        let elapsed = crate::explorer::format_duration(start.elapsed());
        let time_label = t!("cmd.report.time_label");
        let w = time_label.len() + elapsed.len() + 8;
        println!("┌{}┐", "─".repeat(w));
        println!("│ {} {} ", time_label.cyan(), elapsed.cyan());
        println!("└{}┘", "─".repeat(w));
    }
    Ok(false)
}

/// `report project <fmt> [path] [--name <output>]` request.
#[derive(Debug)]
pub(crate) struct ProjectReport {
    pub format: ReportFormat,
    pub path: PathBuf,
    pub name: Option<String>,
}

/// Parse a report format name (case-insensitive).
fn parse_report_format(raw: &str) -> Option<ReportFormat> {
    match raw.to_lowercase().as_str() {
        "txt" | "text" => Some(ReportFormat::Txt),
        "html" | "htm" => Some(ReportFormat::Html),
        "json" => Some(ReportFormat::Json),
        "md" | "markdown" => Some(ReportFormat::Md),
        "pdf" => Some(ReportFormat::Pdf),
        "docx" => Some(ReportFormat::Docx),
        "xlsx" => Some(ReportFormat::Xlsx),
        _ => None,
    }
}

/// Parse `project <fmt> [path] [--name <output>]` (the part after
/// `report project`). `Err` is a lang key for the usage message.
pub(crate) fn parse_project_report(args: &str) -> Result<ProjectReport, &'static str> {
    let usage = "cmd.report.project_usage";
    let toks: Vec<String> = tokenize_spans(args)
        .into_iter()
        .map(|(s, _, _)| s)
        .collect();
    let mut format: Option<ReportFormat> = None;
    let mut path: Option<String> = None;
    let mut name: Option<String> = None;
    let mut i = 0;
    while i < toks.len() {
        match toks[i].as_str() {
            "--name" | "-n" | "-o" | "--output" => match toks.get(i + 1) {
                Some(v) if !v.trim().is_empty() => {
                    name = Some(v.clone());
                    i += 1;
                }
                _ => return Err(usage),
            },
            other if other.starts_with('-') => return Err(usage),
            other => {
                if format.is_none() {
                    format = Some(parse_report_format(other).ok_or(usage)?);
                } else if path.is_none() {
                    path = Some(other.to_string());
                } else {
                    return Err(usage);
                }
            }
        }
        i += 1;
    }
    Ok(ProjectReport {
        format: format.ok_or(usage)?,
        path: PathBuf::from(path.unwrap_or_else(|| ".".to_string())),
        name,
    })
}

/// Output path for `--name`: used as given, with the format extension
/// appended when the name has none. Parent directories are created.
fn project_output_path(name: &str, format: ReportFormat) -> PathBuf {
    let mut out = PathBuf::from(name);
    if out.extension().is_none() {
        out.set_extension(format.extension());
    }
    if let Some(parent) = out.parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    out
}

pub fn cmd_report(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let format = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1).unwrap_or(&"").trim();

    if format.eq_ignore_ascii_case("project") {
        let req = match parse_project_report(subargs) {
            Ok(r) => r,
            Err(key) => {
                print_error(&t!(key));
                println!("{}", t!("cmd.report.project_usage"));
                return Ok(false);
            }
        };
        if req.path.is_file() {
            print_error(&t!(
                "cmd.report.dir_only",
                req.format.extension().to_uppercase()
            ));
            return Ok(false);
        }
        let out = match &req.name {
            Some(name) => project_output_path(name, req.format),
            None => {
                // Same default naming as `report <fmt>`.
                let dir_name = req
                    .path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "root".to_string());
                crate::output::build_output_path(&format!(
                    "{}.{}",
                    dir_name,
                    req.format.extension()
                ))
            }
        };
        generate_report_to(&req.path, req.format, &out.to_string_lossy())?;
        return Ok(false);
    }

    if format.eq_ignore_ascii_case("clip") {
        if subargs.is_empty() {
            print_error(&t!("cmd.clipboard.select_usage"));
            return Ok(false);
        }
        let n: usize = match subargs.split_whitespace().next().unwrap_or("").parse() {
            Ok(n) => n,
            Err(_) => {
                print_error(&t!("cmd.clipboard.select_usage"));
                return Ok(false);
            }
        };
        let entries = super::clipboard::load_history_public();
        if entries.is_empty() {
            print_info(&t!("cmd.clipboard.empty"));
            return Ok(false);
        }
        if n == 0 || n > entries.len() {
            print_error(&t!("cmd.clipboard.entry_not_found", n, entries.len()));
            return Ok(false);
        }
        let entry = &entries[n - 1];
        let date = chrono::DateTime::from_timestamp(entry.ts as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "?".to_string());
        let line_count = entry.content.lines().count().max(1);
        let char_count = entry.content.chars().count();
        println!("{}", t!("cmd.report.clip_header", n).cyan().bold());
        println!("  {}: {}", t!("cmd.report.clip_meta_date"), date);
        println!("  {}: {}", t!("cmd.report.clip_meta_lines"), line_count);
        println!("  {}: {}", t!("cmd.report.clip_meta_chars"), char_count);
        println!();
        print!("{}", entry.content);
        if !entry.content.is_empty() && !entry.content.ends_with('\n') {
            println!();
        }
        return Ok(false);
    }

    // Legacy `report txt|json|md|html|pdf|docx|xlsx <path>` shortcuts are removed.
    // Use `report project <fmt> [path] [--name ...]` instead.
    if matches!(
        format,
        "txt" | "json" | "md" | "html" | "pdf" | "docx" | "xlsx"
    ) {
        print_error(&t!("cmd.report.legacy_removed"));
        println!("  {}", t!("cmd.report.help_project"));
        return Ok(false);
    }

    println!("{}", t!("cmd.report.title").cyan().bold());
    println!("  {}", t!("cmd.report.help_project"));
    println!("  {}", t!("cmd.report.help_clip"));
    println!();
    println!("{}", t!("cmd.report.examples").green());
    println!("  {}", t!("cmd.report.example_project"));
    println!("  {}", t!("cmd.report.example_clip"));
    Ok(false)
}

pub fn cmd_txtc(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        let files = list_supported_files(nav)?;
        show_file_selection_menu(nav, files, true)?;
    } else {
        let target = Path::new(args);
        if target.is_file() {
            if is_supported_format(target) {
                let show_lines = Config::global_get_show_line_numbers();
                let content = crate::output::cat_file_with_line_numbers(target, show_lines)?;

                #[cfg(target_os = "android")]
                {
                    use crate::output::print_info;
                    print_info(&t!("cmd.report.copying_clipboard", target.display()));
                    match crate::output::copy_to_clipboard(&content, "TXT") {
                        Ok(()) => {}
                        Err(e) => {
                            print_error(&t!("cmd.report.copy_failed", e));
                            let output_file = crate::output::build_output_path(&format!(
                                "copied_{}.txt",
                                target.file_name().unwrap_or_default().to_string_lossy()
                            ));
                            if let Ok(()) = crate::output::write_file(&output_file, &content) {
                                print_success(&t!(
                                    "cmd.report.content_saved",
                                    output_file.display()
                                ));
                                print_info(&t!("cmd.report.neovim_hint", output_file.display()));
                            }
                        }
                    }
                }

                #[cfg(not(target_os = "android"))]
                {
                    crate::output::copy_to_clipboard(&content, "TXT")?;
                    print_success(&t!("cmd.report.copied_clipboard", target.display()));
                }
            } else {
                print_warning(&t!("cmd.report.skipped_unsupported", args));
            }
        } else {
            print_error(&t!("cmd.report.file_not_found", args));
        }
    }
    Ok(false)
}

/// Parsed `txtf` args: an optional file plus display range flags.
/// Flags may come before or after the file (`txtf --head 5 a.txt`).
#[derive(Debug, Default)]
pub(crate) struct TxtfArgs {
    pub file: Option<String>,
    pub head: Option<usize>,
    pub tail: Option<usize>,
    pub betw: Option<String>,
    /// `--line` / `--no-line`: override the configured line numbers.
    pub lines: Option<bool>,
}

/// Parse `txtf` args. `Err((key, arg))` is a lang key plus the
/// offending value for `print_error(&t!(key, arg))`.
pub(crate) fn parse_txtf_args(args: &str) -> Result<TxtfArgs, (String, String)> {
    let mut out = TxtfArgs::default();
    let toks: Vec<String> = tokenize_spans(args)
        .into_iter()
        .map(|(s, _, _)| s)
        .collect();
    let mut i = 0;
    while i < toks.len() {
        match toks[i].as_str() {
            "--head" => match next_count(&toks, i) {
                Some(n) => {
                    out.head = Some(n);
                    i += 1;
                }
                None => {
                    return Err((
                        "cmd.report.invalid_count".to_string(),
                        toks.get(i + 1).cloned().unwrap_or_default(),
                    ));
                }
            },
            "--tail" => match next_count(&toks, i) {
                Some(n) => {
                    out.tail = Some(n);
                    i += 1;
                }
                None => {
                    return Err((
                        "cmd.report.invalid_count".to_string(),
                        toks.get(i + 1).cloned().unwrap_or_default(),
                    ));
                }
            },
            "--betw" => match toks.get(i + 1) {
                Some(v) => {
                    out.betw = Some(v.clone());
                    i += 1;
                }
                None => {
                    return Err(("cmd.report.invalid_range".to_string(), String::new()));
                }
            },
            "--line" | "--lines" | "-l" => out.lines = Some(true),
            "--no-line" | "--no-lines" | "-nl" => out.lines = Some(false),
            other => {
                if out.file.is_some() {
                    return Err(("cmd.report.unknown_option".to_string(), other.to_string()));
                }
                out.file = Some(other.to_string());
            }
        }
        i += 1;
    }
    Ok(out)
}

/// Display one file for `txtf` / `view <file>`, honoring `range`
/// (original line numbers kept). `label` is echoed in messages.
pub(crate) fn display_txtf_file(
    target: &Path,
    range: Option<crate::output::FileLineRange>,
    label: &str,
    lines: Option<bool>,
) -> Result<()> {
    if target.is_file() {
        if is_supported_format(target) {
            let show_lines = lines.unwrap_or_else(Config::global_get_show_line_numbers);
            crate::output::cat_file_with_syntax_range(target, show_lines, range)?;
        } else {
            print_warning(&t!("cmd.report.skipped_unsupported", label));
        }
    } else {
        print_error(&t!("cmd.report.file_not_found", label));
    }
    Ok(())
}

pub fn cmd_txtf(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.trim().is_empty() {
        let files = list_supported_files(nav)?;
        show_file_selection_menu(nav, files, false)?;
        return Ok(false);
    }
    let parsed = match parse_txtf_args(args) {
        Ok(p) => p,
        Err((key, arg)) => {
            print_error(&t!(key.as_str(), arg));
            println!("{}", t!("help.report.txtf"));
            return Ok(false);
        }
    };
    let range =
        match crate::output::parse_file_range(parsed.head, parsed.tail, parsed.betw.as_deref()) {
            Ok(r) => r,
            Err(()) => {
                print_error(&t!(
                    "cmd.report.invalid_range",
                    parsed.betw.clone().unwrap_or_default()
                ));
                println!("{}", t!("help.report.txtf"));
                return Ok(false);
            }
        };
    match parsed.file {
        Some(f) => {
            let target = match resolve_view_operand(&f, nav) {
                Some(p) => p,
                None => {
                    let list_name = f
                        .strip_prefix('@')
                        .unwrap_or(&f)
                        .split('/')
                        .next()
                        .unwrap_or(&f);
                    print_error(&t!("cmd.target.tp_not_found", list_name));
                    return Ok(false);
                }
            };
            if target.is_dir() {
                let files = list_supported_files_in(&target)?;
                if files.is_empty() {
                    print_warning(&t!("cmd.report.no_files_in_dir", f));
                    return Ok(false);
                }
                show_file_selection_menu_range(nav, files, false, range, parsed.lines)?;
            } else {
                display_txtf_file(&target, range, &f, parsed.lines)?;
            }
        }
        None => {
            let files = list_supported_files(nav)?;
            show_file_selection_menu_range(nav, files, false, range, parsed.lines)?;
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn project_report_args() {
        let p = parse_project_report("html").unwrap();
        assert_eq!(p.format, ReportFormat::Html);
        assert_eq!(p.path, PathBuf::from("."));
        assert!(p.name.is_none());

        let p = parse_project_report("md src --name out").unwrap();
        assert_eq!(p.format, ReportFormat::Md);
        assert_eq!(p.path, PathBuf::from("src"));
        assert_eq!(p.name.as_deref(), Some("out"));

        // Flags may come before the path.
        let p = parse_project_report("json --name r1 src").unwrap();
        assert_eq!(p.path, PathBuf::from("src"));
        assert_eq!(p.name.as_deref(), Some("r1"));

        // Formats are case-insensitive; quoted names keep their spaces.
        let p = parse_project_report("HTML -n \"my report\"").unwrap();
        assert_eq!(p.format, ReportFormat::Html);
        assert_eq!(p.name.as_deref(), Some("my report"));

        // Errors: missing/invalid format, extra target, dangling flag.
        assert!(parse_project_report("").is_err());
        assert!(parse_project_report("nope").is_err());
        assert!(parse_project_report("txt a b").is_err());
        assert!(parse_project_report("txt --name").is_err());
        assert!(parse_project_report("txt --bogus").is_err());
    }

    #[test]
    fn project_output_names_get_the_format_extension() {
        assert_eq!(
            project_output_path("out", ReportFormat::Html),
            PathBuf::from("out.html")
        );
        // An explicit extension wins.
        assert_eq!(
            project_output_path("out.txt", ReportFormat::Html),
            PathBuf::from("out.txt")
        );
        let dir = tempfile::tempdir().unwrap();
        let named = dir.path().join("nested/out");
        assert_eq!(
            project_output_path(&named.to_string_lossy(), ReportFormat::Md),
            dir.path().join("nested/out.md")
        );
        assert!(dir.path().join("nested").is_dir());
    }

    #[test]
    fn project_report_writes_the_named_file() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
        std::fs::create_dir_all(dir.path().join("sub")).unwrap();
        std::fs::write(dir.path().join("sub/notes.txt"), "hi").unwrap();
        let out = dir.path().join("out/report");
        let args = format!(
            "project html {} --name {}",
            dir.path().display(),
            out.display()
        );
        let mut nav = Navigator::new().unwrap();
        cmd_report(&args, &mut nav).unwrap();
        let file = dir.path().join("out/report.html");
        assert!(file.is_file(), "missing {}", file.display());
        let html = std::fs::read_to_string(&file).unwrap();
        // The tree is a collapsible widget, not an ASCII <pre> dump.
        assert!(html.contains("ntree-root"));
        assert!(html.contains("<details><summary>"));
        assert!(html.contains("main.rs"));
        assert!(html.contains("sub"));
    }

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
    fn parse_txtf_args_file_and_flags_any_order() {
        let p = parse_txtf_args("a.txt").unwrap();
        assert_eq!(p.file.as_deref(), Some("a.txt"));
        assert!(p.head.is_none() && p.tail.is_none() && p.betw.is_none());
        let p = parse_txtf_args("a.txt --head 5").unwrap();
        assert_eq!((p.file.as_deref(), p.head), (Some("a.txt"), Some(5)));
        let p = parse_txtf_args("--tail 3 --betw 2:4 a.txt").unwrap();
        assert_eq!(p.tail, Some(3));
        assert_eq!(p.betw.as_deref(), Some("2:4"));
        let p = parse_txtf_args("--head 5").unwrap();
        assert!(p.file.is_none());
        assert_eq!(p.head, Some(5));
        // Quoted names with spaces survive.
        let p = parse_txtf_args("\"my file.txt\" --tail 2").unwrap();
        assert_eq!(p.file.as_deref(), Some("my file.txt"));
        // Line-number overrides for one call only.
        assert_eq!(parse_txtf_args("a.txt --line").unwrap().lines, Some(true));
        assert_eq!(
            parse_txtf_args("--no-line a.txt").unwrap().lines,
            Some(false)
        );
        assert_eq!(parse_txtf_args("a.txt").unwrap().lines, None);
        // Errors carry keys.
        assert!(parse_txtf_args("--head 0").is_err());
        assert!(parse_txtf_args("--head").is_err());
        assert!(parse_txtf_args("--tail x").is_err());
        assert!(parse_txtf_args("--betw").is_err());
        assert!(parse_txtf_args("a.txt b.txt").is_err());
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
