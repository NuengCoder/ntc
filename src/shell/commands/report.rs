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
use crate::output::{cat_file_with_syntax, print_error, print_success, print_warning};
use crate::report::{generate_report, ReportFormat};
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
                display_txtf_file(&fv.path, range, label)?;
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

pub fn cmd_report(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let format = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1).unwrap_or(&"").trim();

    match format {
        "txt" => generate_report_file(Path::new(subargs), ReportFormat::Txt)?,
        "json" => generate_report_dir(Path::new(subargs), ReportFormat::Json, "JSON")?,
        "md" => generate_report_dir(Path::new(subargs), ReportFormat::Md, "Markdown")?,
        "html" => generate_report_file(Path::new(subargs), ReportFormat::Html)?,
        "pdf" => generate_report_dir_only(Path::new(subargs), ReportFormat::Pdf, "PDF")?,
        "docx" => generate_report_dir_only(Path::new(subargs), ReportFormat::Docx, "DOCX")?,
        "xlsx" => generate_report_dir_only(Path::new(subargs), ReportFormat::Xlsx, "XLSX")?,
        _ => {
            println!("{}", t!("cmd.report.title").cyan().bold());
            println!("  {}", t!("cmd.report.help_txt"));
            println!("  {}", t!("cmd.report.help_json"));
            println!("  {}", t!("cmd.report.help_md"));
            println!("  {}", t!("cmd.report.help_html"));
            println!("  {}", t!("cmd.report.help_pdf"));
            println!("  {}", t!("cmd.report.help_docx"));
            println!("  {}", t!("cmd.report.help_xlsx"));
            println!();
            println!("{}", t!("cmd.report.examples").green());
            println!("  {}", t!("cmd.report.example_txt"));
            println!("  {}", t!("cmd.report.example_json"));
        }
    }
    Ok(false)
}

fn generate_report_file(target: &Path, format: ReportFormat) -> Result<()> {
    let target = if target.as_os_str().is_empty() {
        Path::new(".")
    } else {
        target
    };
    if target.is_dir() {
        generate_report(target, format)?;
    } else if target.is_file() {
        if is_supported_format(target) {
            let show_lines = Config::global_get_show_line_numbers();
            cat_file_with_syntax(target, show_lines)?;
        } else {
            print_warning(&t!("cmd.report.skipped_unsupported", target.display()));
        }
    }
    Ok(())
}

fn generate_report_dir(target: &Path, format: ReportFormat, label: &str) -> Result<()> {
    let target = if target.as_os_str().is_empty() {
        Path::new(".")
    } else {
        target
    };
    if target.is_dir() {
        generate_report(target, format)?;
    } else {
        print_error(&t!("cmd.report.dir_only", label));
    }
    Ok(())
}

fn generate_report_dir_only(target: &Path, format: ReportFormat, label: &str) -> Result<()> {
    let target = if target.as_os_str().is_empty() {
        Path::new(".")
    } else {
        target
    };
    if target.is_dir() {
        generate_report(target, format)?;
    } else {
        print_error(&t!("cmd.report.dir_only", label));
    }
    Ok(())
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
) -> Result<()> {
    if target.is_file() {
        if is_supported_format(target) {
            let show_lines = Config::global_get_show_line_numbers();
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
                show_file_selection_menu_range(nav, files, false, range)?;
            } else {
                display_txtf_file(&target, range, &f)?;
            }
        }
        None => {
            let files = list_supported_files(nav)?;
            show_file_selection_menu_range(nav, files, false, range)?;
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

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
