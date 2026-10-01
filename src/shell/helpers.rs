use crate::config::Config;
use crate::explorer::{
    calculate_total_size, format_tree, format_tree_with_sizes, generate_tree, human_readable_size,
};
use crate::filetype::strip_unc_prefix;
use crate::navigator::Navigator;
use crate::output::{print_error, print_separator, print_success, print_warning};
use crate::t;
use crate::teleport::TeleportManager;

use anyhow::{bail, Result};
use colored::*;
use std::collections::HashSet;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// ============================================================================
// Target path resolution for size / view (paths and teleport savepoints)
// ============================================================================

/// Result of extracting a target path specifier from command args.
pub(crate) struct TargetArg {
    pub path: Option<PathBuf>,
    pub rest: Vec<String>,
}

/// Tokenize args respecting double-quoted sections (so quoted paths survive).
fn tokenize_args(args: &str) -> Vec<String> {
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

/// Pull a target path specifier out of `size`/`view` args.
/// Recognizes `-p <val>` / `--path <val>`, `@name`, a bare non-flag token,
/// and tokens after `--`. `value_flags` are flags that take a value (e.g.
/// `-d`/`--depth`) and are preserved in `rest` along with their value so the
/// caller's flag parser can consume them unchanged.
fn extract_target_spec(args: &str, value_flags: &[&str]) -> (Option<String>, Vec<String>) {
    let tokens = tokenize_args(args);
    let mut target: Option<String> = None;
    let mut rest: Vec<String> = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let tok = &tokens[i];
        match tok.as_str() {
            "-p" | "--path" => {
                if i + 1 < tokens.len() {
                    if target.is_none() {
                        target = Some(tokens[i + 1].clone());
                    }
                    i += 1;
                }
            }
            "--" => {
                for j in (i + 1)..tokens.len() {
                    if target.is_none() && !tokens[j].is_empty() {
                        target = Some(tokens[j].clone());
                    }
                }
                break;
            }
            _ => {
                if tok.starts_with('@') && tok.len() > 1 {
                    if target.is_none() {
                        target = Some(tok.clone());
                    }
                } else if value_flags.contains(&tok.as_str()) {
                    rest.push(tok.clone());
                    if i + 1 < tokens.len() {
                        rest.push(tokens[i + 1].clone());
                        i += 1;
                    }
                } else if tok.starts_with('-') {
                    rest.push(tok.clone());
                } else if target.is_none() {
                    target = Some(tok.clone());
                } else {
                    rest.push(tok.clone());
                }
            }
        }
        i += 1;
    }
    (target, rest)
}

/// Resolve a path specifier to a concrete directory.
/// `@name` always means a teleport savepoint. Otherwise the disk path wins;
/// a matching teleport name is used as a fallback.
pub(crate) fn resolve_target_spec(spec: &str, nav: &Navigator) -> Result<PathBuf> {
    let name = spec.strip_prefix('@').unwrap_or(spec);

    if spec.starts_with('@') {
        if let Some(p) = TeleportManager::get_path(name) {
            return Ok(p);
        }
        print_error(&t!("cmd.target.tp_not_found", name));
        bail!(t!("cmd.target.tp_not_found", name));
    }

    let p = Path::new(spec);
    let full = if p.is_absolute() {
        p.to_path_buf()
    } else {
        nav.current_path().join(p)
    };

    if full.exists() {
        if full.is_dir() {
            return Ok(full);
        }
        print_error(&t!("cmd.target.not_dir", strip_unc_prefix(&full)));
        bail!(t!("cmd.target.not_dir", strip_unc_prefix(&full)));
    }

    if let Some(tp) = TeleportManager::get_path(name) {
        return Ok(tp);
    }

    print_error(&t!("cmd.target.not_found", spec));
    bail!(t!("cmd.target.not_found", spec));
}

/// Parse `size`/`view` args: extract an optional target path and return the
/// remaining flag tokens for the caller's existing parser. On failure a
/// localized error has already been printed, so callers just stop.
pub(crate) fn parse_target_arg(args: &str, nav: &Navigator) -> Result<TargetArg> {
    let (spec, rest) = extract_target_spec(args, &["-d", "--depth"]);
    let path = match spec {
        Some(s) => Some(resolve_target_spec(&s, nav)?),
        None => None,
    };
    Ok(TargetArg { path, rest })
}

/// Resolve one file operand for `cp`/`mv`.
/// `@name` strictly means a teleport savepoint (unknown → localized error).
/// Anything else is an absolute path or relative to the shell's directory.
/// Unlike `resolve_target_spec` there is no directory gate and no
/// bare-name fallback — callers validate existence/type themselves.
pub(crate) fn resolve_file_operand(spec: &str, nav: &Navigator) -> Result<PathBuf> {
    if let Some(name) = spec.strip_prefix('@') {
        if name.is_empty() {
            print_error(&t!("cmd.target.tp_not_found", spec));
            bail!(t!("cmd.target.tp_not_found", spec));
        }
        if let Some(p) = TeleportManager::get_path(name) {
            return Ok(p);
        }
        print_error(&t!("cmd.target.tp_not_found", name));
        bail!(t!("cmd.target.tp_not_found", name));
    }
    let p = Path::new(spec);
    Ok(if p.is_absolute() {
        p.to_path_buf()
    } else {
        nav.current_path().join(p)
    })
}

// ============================================================================
// Helper functions (gosc, show_tree, file menus, spinner, system command)
// ============================================================================

pub(crate) fn show_tree(
    nav: &Navigator,
    max_depth_override: Option<usize>,
    show_progress: bool,
    include_files: bool,
    show_filesize: bool,
    show_dirsize: bool,
    copy_to_clipboard: bool,
    care_sizes: bool,
) {
    show_tree_at(
        nav.current_path(),
        max_depth_override,
        show_progress,
        include_files,
        show_filesize,
        show_dirsize,
        copy_to_clipboard,
        care_sizes,
        None,
    );
}

/// Same as `show_tree` but renders the tree for `target` instead of the
/// navigator's current directory. When `ext_filter` is `Some`, only files
/// matching those extensions are shown (empty dirs pruned) and directory
/// sizes become the sum of the kept descendants. Returns the number of
/// matched files (0 when no filter is active).
pub(crate) fn show_tree_at(
    target: &Path,
    max_depth_override: Option<usize>,
    show_progress: bool,
    include_files: bool,
    show_filesize: bool,
    show_dirsize: bool,
    copy_to_clipboard: bool,
    care_sizes: bool,
    ext_filter: Option<&[String]>,
) -> usize {
    println!();
    print_separator("Current Directory");
    println!("Path: {}", strip_unc_prefix(target).cyan());
    println!();

    let mut tree = if show_progress {
        run_with_spinner(&t!("helpers.building_tree"), || {
            generate_tree(
                &target.to_string_lossy(),
                max_depth_override,
                include_files,
                None,
            )
        })
    } else {
        generate_tree(
            &target.to_string_lossy(),
            max_depth_override,
            include_files,
            None,
        )
    };

    let matched = match ext_filter {
        Some(exts) if !exts.is_empty() => crate::explorer::prune_tree_by_exts(&mut tree, exts),
        _ => 0,
    };
    let filtered = matches!(ext_filter, Some(exts) if !exts.is_empty());

    let show_any_size = show_filesize || show_dirsize;
    // Annotate file entries with git status symbols when in a repo
    if crate::config::Config::global_get_git_integration_enabled()
        && crate::utils::git::is_repo(target)
    {
        let status = crate::utils::git::status_map(target);
        if !status.is_empty() {
            crate::explorer::annotate_git_status(&mut tree, &status);
        }
    }
    let tree_str = if show_any_size {
        if filtered {
            // Filtered view: directory sizes are the sum of kept files,
            // not the on-disk totals (which include filtered-out files).
            if show_progress {
                run_with_spinner(&t!("helpers.calculating_sizes"), || {
                    crate::explorer::compute_filtered_sizes(&mut tree);
                });
            } else {
                crate::explorer::compute_filtered_sizes(&mut tree);
            }
        } else if show_progress {
            run_with_spinner(&t!("helpers.calculating_sizes"), || {
                crate::explorer::compute_tree_sizes(&mut tree, None, care_sizes);
            });
        } else {
            crate::explorer::compute_tree_sizes(&mut tree, None, care_sizes);
        }
        format_tree_with_sizes(
            &tree,
            "",
            true,
            show_filesize,
            show_dirsize,
            care_sizes,
            None,
        )
    } else {
        format_tree(&tree, "", true)
    };

    if copy_to_clipboard {
        let _ = crate::output::copy_to_clipboard(&tree_str, "Tree");
    }

    #[cfg(not(target_os = "android"))]
    {
        use colored::Colorize;
        let theme = crate::utils::theme::ThemeManager::current();
        for line in tree_str.lines() {
            if line.contains("[ignored]") {
                println!("{}", line.color(theme.shell.tree_ignored.to_colored()));
            } else if line.contains("[Directory]") {
                println!("{}", line.color(theme.shell.tree_dir.to_colored()));
            } else if line.trim().starts_with("├──") || line.trim().starts_with("└──") {
                println!("{}", line.color(theme.shell.tree_branch.to_colored()));
            } else {
                println!("{}", line);
            }
        }
    }
    #[cfg(target_os = "android")]
    {
        for line in tree_str.lines() {
            if line.contains("[ignored]") {
                println!("{}", line);
            } else if line.contains("[Directory]") {
                println!("{}", line);
            } else if line.trim().starts_with("├──") || line.trim().starts_with("└──") {
                println!("{}", line);
            } else {
                println!("{}", line);
            }
        }
    }

    matched
}

pub(super) fn gosc_loop(nav: &mut Navigator) -> Result<()> {
    let a = |s: &str| crate::utils::terminal::asciify(s);
    loop {
        let dirs = nav.list_subdirs()?;
        crate::navigator::clear_screen();
        show_tree(nav, Some(1), false, false, false, false, false, false);

        println!();
        println!(
            "{}",
            a("╔══════════════════════════════════════════════════╗")
        );
        println!(
            "{}",
            a(&format!("║{:^50}║", t!("gosc.panel.title").cyan().bold()))
        );
        println!(
            "{}",
            a("╠══════════════════════════════════════════════════╣")
        );
        println!("║ {} {}", "-1".yellow(), t!("gosc.panel.back_1"));
        println!("║ {} {}", "-2".yellow(), t!("gosc.panel.back_2"));
        println!("║ {} {}", "-n".yellow(), t!("gosc.panel.back_n"));
        println!("║ {} {}", "v".yellow(), t!("gosc.panel.view_care"));
        println!("║ {} {}", "s".yellow(), t!("gosc.panel.size_care"));
        println!("║ {} {}", "vs".yellow(), t!("gosc.panel.view_size_care"));
        println!("║ {} {}", "0".red(), t!("gosc.panel.exit"));
        println!(
            "{}",
            a("╠──────────────────────────────────────────────────╣")
        );

        if dirs.is_empty() {
            println!("║ {}", t!("gosc.panel.empty").dimmed());
        } else {
            for (i, name) in &dirs {
                println!("║ {}. {}", i.to_string().yellow(), name.bright_purple());
            }
        }

        println!(
            "{}",
            a("╚──────────────────────────────────────────────────╝")
        );
        println!();
        print!("{} ", t!("gosc.prompt").green().bold());

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "0" {
            println!("{}", t!("gosc.exiting").dimmed());
            break Ok(());
        }

        match input {
            "s" => {
                println!();
                println!("{}", t!("gosc.output_zone").cyan());
                let total = run_with_spinner(&t!("gosc.calculating_size"), || {
                    calculate_total_size(nav.current_path())
                });
                let bytes_str = format!("{}", total);
                let human_str = human_readable_size(total);
                let w = bytes_str.len().max(human_str.len()) + 18;
                let v = a("│");
                let h = a("─");
                println!("{}{}{}", a("┌"), h.repeat(w), a("┐"));
                println!(
                    "{}{:^w$}{}",
                    v,
                    t!("gosc.size_title").cyan().bold(),
                    v,
                    w = w
                );
                println!("{}{}{}", a("├"), h.repeat(w), a("┤"));
                println!(
                    "{} {} {:<pad$} {}",
                    v,
                    t!("gosc.size_bytes"),
                    bytes_str.yellow(),
                    v,
                    pad = w - 9
                );
                println!(
                    "{} {} {:<pad$} {}",
                    v,
                    t!("gosc.size_human"),
                    human_str.green().bold(),
                    v,
                    pad = w - 9
                );
                println!("{}{}{}", a("└"), h.repeat(w), a("┘"));
                println!("{}", t!("gosc.output_zone_end").cyan());
                println!();
                print!("{} ", t!("gosc.press_enter").dimmed());
                let mut dummy = String::new();
                std::io::stdin().read_line(&mut dummy)?;
                continue;
            }
            "v" => {
                println!();
                println!("{}", t!("gosc.output_zone").cyan());
                show_tree(nav, Some(1), false, true, false, false, false, true);
                println!("{}", t!("gosc.output_zone_end").cyan());
                println!();
                print!("{} ", t!("gosc.press_enter").dimmed());
                let mut dummy = String::new();
                std::io::stdin().read_line(&mut dummy)?;
                continue;
            }
            "vs" => {
                println!();
                println!("{}", t!("gosc.output_zone").cyan());
                show_tree(nav, Some(1), false, true, true, true, false, true);
                println!("{}", t!("gosc.output_zone_end").cyan());
                println!();
                print!("{} ", t!("gosc.press_enter").dimmed());
                let mut dummy = String::new();
                std::io::stdin().read_line(&mut dummy)?;
                continue;
            }
            _ => {}
        }

        if let Some(back_str) = input.strip_prefix('-') {
            match back_str.parse::<usize>() {
                Ok(n) if n > 0 => {
                    for _ in 0..n {
                        let _ = nav.go_back();
                    }
                }
                Err(_) => {
                    print_error(&t!("helpers.invalid_back_count", back_str));
                }
                Ok(_) => {}
            }
            continue;
        }

        match input.parse::<usize>() {
            Ok(num) => {
                if let Some((_, name)) = dirs.iter().find(|(i, _)| *i == num) {
                    let new_path = nav.current_path().join(name);
                    let _ = nav.go_to(&new_path);
                } else {
                    print_error(&t!("helpers.invalid_num", num));
                }
            }
            Err(_) => {
                print_error(&t!("helpers.invalid_input", input));
            }
        }
    }
}

pub(super) fn open_with_fallback(config_path: &std::path::Path) {
    #[cfg(windows)]
    {
        let path_str = config_path.to_str().unwrap_or("");
        if !path_str.is_empty() {
            let status = std::process::Command::new("cmd")
                .args(["/C", "start", "", path_str])
                .status();
            if status.is_ok() {
                return;
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        let status = std::process::Command::new("open").arg(config_path).status();
        if status.is_ok() {
            return;
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(editor) = std::env::var("VISUAL").or_else(|_| std::env::var("EDITOR")) {
            let parts: Vec<&str> = editor.split_whitespace().collect();
            if let Some(cmd) = parts.first() {
                let mut proc = std::process::Command::new(cmd);
                for arg in &parts[1..] {
                    proc.arg(arg);
                }
                proc.arg(config_path);
                if proc.status().is_ok() {
                    return;
                }
            }
        }
        if std::process::Command::new("xdg-open")
            .arg(config_path)
            .status()
            .is_ok()
        {
            return;
        }
        for editor in &["vim", "nano", "nvim", "emacs"] {
            if std::process::Command::new(editor)
                .arg(config_path)
                .status()
                .is_ok()
            {
                return;
            }
        }
    }
    crate::output::print_info(&t!("helpers.no_editor"));
    let _ = crate::editor::edit_file(config_path);
}

/// Run a closure with an animated spinner in a background thread.
/// The spinner is cleared when `f` returns.
pub(super) fn run_with_spinner<F, T>(msg: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    let done = Arc::new(AtomicBool::new(false));
    let done_clone = done.clone();
    let msg = msg.to_string();

    let msg_clone = msg.clone();
    let handle = thread::spawn(move || {
        let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let mut i = 0;
        while !done_clone.load(Ordering::Relaxed) {
            print!("\r{} {} ", frames[i], msg_clone);
            let _ = io::stdout().flush();
            thread::sleep(Duration::from_millis(80));
            i = (i + 1) % frames.len();
        }
    });

    let result = f();

    done.store(true, Ordering::Relaxed);
    let _ = handle.join();
    // Clear spinner line with spaces then go back to start
    let clear_len = msg.len() + 4; // spinner char + space + msg + space
    print!("\r{:1$}\r", "", clear_len);
    let _ = io::stdout().flush();

    result
}

// ============================================================================
// Export/Import helpers for ral and igcare
// ============================================================================

fn export_path(name: &str, ext: &str) -> std::path::PathBuf {
    let out = Config::global_get_output_path();
    out.join(format!("{}.{}", name, ext))
}

pub(crate) fn ral_export_all(name: &str) -> Result<()> {
    let aliases = Config::global_get_run_aliases();
    if aliases.is_empty() {
        print_warning(&t!("helpers.no_ral_to_export"));
        return Ok(());
    }
    let path = export_path(name, "ntc.ral");
    write_ral_file(&path, &aliases, None)?;
    Ok(())
}

pub(crate) fn ral_export_select(nav: &mut Navigator, name: &str) -> Result<()> {
    let aliases = Config::global_get_run_aliases();
    if aliases.is_empty() {
        print_warning(&t!("helpers.no_ral_to_export"));
        return Ok(());
    }

    let mut sorted: Vec<(&String, &String)> = aliases.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));

    let mut selected: std::collections::HashSet<usize> = std::collections::HashSet::new();

    loop {
        crate::navigator::clear_screen();
        show_tree(nav, Some(1), false, false, false, false, false, false);

        println!();
        println!(
            "{}",
            crate::utils::terminal::asciify("╔══════════════════════════════════════════════════╗")
        );
        println!(
            "{}",
            crate::utils::terminal::asciify(&format!(
                "║{:^50}║",
                t!("ral_export.title").cyan().bold()
            ))
        );
        println!(
            "{}",
            crate::utils::terminal::asciify("╠══════════════════════════════════════════════════╣")
        );
        println!("║ {} {}     ", "<n>".yellow(), t!("ral_export.select"));
        println!(
            "║ {} {}                    ",
            "a".yellow(),
            t!("ral_export.all")
        );
        println!(
            "║ {} {} ({})           ",
            "0".red(),
            t!("ral_export.finish"),
            if selected.is_empty() {
                t!("ral_export.cancel").red()
            } else {
                format!("{} {}", selected.len(), t!("ral_export.selected")).green()
            }
        );
        println!(
            "{}",
            crate::utils::terminal::asciify("╠──────────────────────────────────────────────────╣")
        );

        for (i, (aname, cmd)) in sorted.iter().enumerate() {
            let idx = i + 1;
            let mark = if selected.contains(&idx) {
                format!(" {}", t!("ral_export.selected_mark"))
                    .green()
                    .to_string()
            } else {
                String::new()
            };
            println!(
                "║ {}. {} -> {}{}",
                idx.to_string().yellow(),
                aname.blue(),
                cmd.dimmed(),
                mark
            );
        }

        println!(
            "{}",
            crate::utils::terminal::asciify("╚══════════════════════════════════════════════════╝")
        );
        println!();
        print!("{} ", t!("ral_export.prompt").green().bold());

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input.is_empty() {
            continue;
        }

        if input == "0" {
            if selected.is_empty() {
                println!("{}", t!("ral_export.export_cancelled").dimmed());
                return Ok(());
            }
            // Export selected aliases
            let mut map: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();
            let mut sel_indices: Vec<usize> = selected.iter().cloned().collect();
            sel_indices.sort();
            for idx in sel_indices {
                let (aname, cmd) = sorted[idx - 1];
                map.insert(aname.clone(), cmd.clone());
            }
            let path = export_path(name, "ntc.ral");
            write_ral_file(&path, &map, Some(map.len()))?;
            return Ok(());
        }

        if input == "a" {
            let path = export_path(name, "ntc.ral");
            write_ral_file(&path, &aliases, None)?;
            return Ok(());
        }

        if let Ok(num) = input.parse::<usize>() {
            if num >= 1 && num <= sorted.len() {
                if selected.contains(&num) {
                    selected.remove(&num);
                } else {
                    selected.insert(num);
                }
            } else {
                print_error(&t!("helpers.invalid_num_range", num, sorted.len()));
            }
        } else {
            print_error(&t!("helpers.invalid_input", input));
        }
    }
}

fn write_ral_file(
    path: &std::path::Path,
    aliases: &std::collections::HashMap<String, String>,
    count: Option<usize>,
) -> Result<()> {
    let mut content = String::from("# ntc run aliases export\n");
    content.push_str("# Import with: ral import <file>\n\n");
    content.push_str("[run_aliases]\n");

    let mut sorted: Vec<(&String, &String)> = aliases.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));
    for (name, cmd) in &sorted {
        content.push_str(&format!("{} = {:?}\n", name, cmd));
    }

    std::fs::write(path, &content)?;

    if let Some(cnt) = count {
        print_success(&t!("helpers.exported_count", cnt, path.display()));
    } else {
        print_success(&t!("helpers.exported_all", sorted.len(), path.display()));
    }
    Ok(())
}

pub(crate) fn ral_import(path_str: &str) -> Result<()> {
    let path = std::path::Path::new(path_str);
    if !path.exists() {
        print_error(&t!("helpers.file_not_found", path.display()));
        return Ok(());
    }
    Config::import_run_aliases_from_file(path)
        .map_err(|e| anyhow::anyhow!("Failed to import: {}", e))?;
    Config::reload_global();
    print_success(&t!("helpers.imported_ral", path.display()));
    Ok(())
}

pub(crate) fn igcare_export_all(name: &str) -> Result<()> {
    let path = export_path(name, "ntc.igcare");
    // Export combined (global + local) settings
    let combined = CombinedIgcare {
        ignored_directory_names: Config::get_ignored_dirs_combined(),
        ignored_extensions: Config::get_ignored_extensions_combined(),
        extra_supported_extensions: Config::get_extra_supported_extensions_combined(),
        ignored_files: Config::get_ignored_files_combined(),
        extra_supported_files: Config::get_extra_supported_files_combined(),
    };
    write_igcare_file(&path, &combined)?;
    print_success(&t!("helpers.exported_igcare_all", path.display()));
    Ok(())
}

struct CombinedIgcare {
    ignored_directory_names: HashSet<String>,
    ignored_extensions: HashSet<String>,
    extra_supported_extensions: HashSet<String>,
    ignored_files: HashSet<String>,
    extra_supported_files: HashSet<String>,
}

pub(crate) fn igcare_export_select(name: &str) -> Result<()> {
    // Use combined (global + local) settings for export
    let combined = CombinedIgcare {
        ignored_directory_names: Config::get_ignored_dirs_combined(),
        ignored_extensions: Config::get_ignored_extensions_combined(),
        extra_supported_extensions: Config::get_extra_supported_extensions_combined(),
        ignored_files: Config::get_ignored_files_combined(),
        extra_supported_files: Config::get_extra_supported_files_combined(),
    };

    let category_labels = [
        t!("igcare_export.ignored_dirs"),
        t!("igcare_export.ignored_extensions"),
        t!("igcare_export.extra_supported_extensions"),
        t!("igcare_export.ignored_files"),
        t!("igcare_export.extra_supported_files"),
    ];

    let category_fields: [&std::collections::HashSet<String>; 5] = [
        &combined.ignored_directory_names,
        &combined.ignored_extensions,
        &combined.extra_supported_extensions,
        &combined.ignored_files,
        &combined.extra_supported_files,
    ];

    let total = category_fields.len();
    let mut selected: std::collections::HashSet<usize> = std::collections::HashSet::new();

    loop {
        crate::navigator::clear_screen();

        println!();
        println!(
            "{}",
            crate::utils::terminal::asciify("╔══════════════════════════════════════════════════╗")
        );
        println!(
            "{}",
            crate::utils::terminal::asciify(&format!(
                "║{:^50}║",
                t!("igcare_export.title").cyan().bold()
            ))
        );
        println!(
            "{}",
            crate::utils::terminal::asciify("╠══════════════════════════════════════════════════╣")
        );
        println!("║ {} {}     ", "<n>".yellow(), t!("igcare_export.select"));
        println!(
            "║ {} {}                    ",
            "a".yellow(),
            t!("igcare_export.all")
        );
        println!(
            "║ {} {} ({})           ",
            "0".red(),
            t!("igcare_export.finish"),
            if selected.is_empty() {
                t!("igcare_export.cancel").red()
            } else {
                format!("{} {}", selected.len(), t!("igcare_export.selected")).green()
            }
        );
        println!(
            "{}",
            crate::utils::terminal::asciify("╠──────────────────────────────────────────────────╣")
        );

        for (i, label) in category_labels.iter().enumerate() {
            let idx = i + 1;
            let count = category_fields[i].len();
            let count_str = format!(" ({})", t!("igcare_export.items", count)).yellow();
            let mark = if selected.contains(&idx) {
                format!(" {}", t!("igcare_export.selected_mark"))
                    .green()
                    .to_string()
            } else {
                String::new()
            };
            println!(
                "║ {}. {}{}{}",
                idx.to_string().yellow(),
                label.cyan(),
                count_str,
                mark
            );
        }

        println!(
            "{}",
            crate::utils::terminal::asciify("╚══════════════════════════════════════════════════╝")
        );
        println!();
        print!("{} ", t!("igcare_export.prompt").green().bold());

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input.is_empty() {
            continue;
        }

        if input == "0" {
            if selected.is_empty() {
                println!("{}", t!("helpers.export_cancelled").dimmed());
                return Ok(());
            }
            let mut sel: Vec<usize> = selected.iter().cloned().collect();
            sel.sort();
            let path = export_path(name, "ntc.igcare");
            write_igcare_file_selective(&path, &combined, &sel)?;
            print_success(&t!(
                "helpers.exported_categories",
                sel.len(),
                path.display()
            ));
            return Ok(());
        }

        if input == "a" {
            let path = export_path(name, "ntc.igcare");
            write_igcare_file(&path, &combined)?;
            print_success(&t!("helpers.exported_igcare_all", path.display()));
            return Ok(());
        }

        if let Ok(num) = input.parse::<usize>() {
            if num >= 1 && num <= total {
                if selected.contains(&num) {
                    selected.remove(&num);
                } else {
                    selected.insert(num);
                }
            } else {
                print_error(&t!("helpers.invalid_num_range", num, total));
            }
        } else {
            print_error(&t!("helpers.invalid_input", input));
        }
    }
}

fn format_set(items: &std::collections::HashSet<String>) -> String {
    if items.is_empty() {
        return "[]".to_string();
    }
    let mut sorted: Vec<&String> = items.iter().collect();
    sorted.sort();
    let mut result = String::from("[\n");
    for item in sorted {
        result.push_str(&format!("    {:?},\n", item));
    }
    result.push(']');
    result
}

fn write_igcare_file(path: &std::path::Path, cfg: &CombinedIgcare) -> Result<()> {
    let mut content = String::from("# ntc ignore/care settings export\n");
    content.push_str("# Import with: igcare import <file>\n\n");
    content.push_str(&format!(
        "ignored_directory_names = {}\n\n",
        format_set(&cfg.ignored_directory_names)
    ));
    content.push_str(&format!(
        "ignored_extensions = {}\n\n",
        format_set(&cfg.ignored_extensions)
    ));
    content.push_str(&format!(
        "extra_supported_extensions = {}\n\n",
        format_set(&cfg.extra_supported_extensions)
    ));
    content.push_str(&format!(
        "ignored_files = {}\n\n",
        format_set(&cfg.ignored_files)
    ));
    content.push_str(&format!(
        "extra_supported_files = {}\n",
        format_set(&cfg.extra_supported_files)
    ));

    std::fs::write(path, &content)?;
    Ok(())
}

fn write_igcare_file_selective(
    path: &std::path::Path,
    cfg: &CombinedIgcare,
    selections: &[usize],
) -> Result<()> {
    let category_fields: Vec<(&str, &std::collections::HashSet<String>)> = vec![
        ("ignored_directory_names", &cfg.ignored_directory_names),
        ("ignored_extensions", &cfg.ignored_extensions),
        (
            "extra_supported_extensions",
            &cfg.extra_supported_extensions,
        ),
        ("ignored_files", &cfg.ignored_files),
        ("extra_supported_files", &cfg.extra_supported_files),
    ];

    let mut content = String::from("# ntc ignore/care settings export (selected categories)\n");
    content.push_str("# Import with: igcare import <file>\n\n");

    for sel in selections {
        if *sel >= 1 && *sel <= category_fields.len() {
            let (field_name, items) = category_fields[*sel - 1];
            content.push_str(&format!("{} = {}\n\n", field_name, format_set(items)));
        }
    }

    std::fs::write(path, &content)?;
    Ok(())
}

pub(crate) fn igcare_import(path_str: &str) -> Result<()> {
    let path = std::path::Path::new(path_str);
    if !path.exists() {
        print_error(&t!("helpers.file_not_found", path.display()));
        return Ok(());
    }
    Config::import_igcare_from_file(path)
        .map_err(|e| anyhow::anyhow!("Failed to import: {}", e))?;
    Config::reload_global();
    print_success(&t!("helpers.imported_igcare", path.display()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_file_operand_plain_paths() {
        let nav = Navigator::new().unwrap();
        let rel = resolve_file_operand("sub/file.txt", &nav).unwrap();
        assert_eq!(rel, nav.current_path().join("sub/file.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn resolve_file_operand_absolute_stays() {
        let nav = Navigator::new().unwrap();
        let abs = resolve_file_operand("C:/Windows", &nav).unwrap();
        assert_eq!(abs, PathBuf::from("C:/Windows"));
    }

    #[test]
    fn resolve_file_operand_unknown_savepoint_errors() {
        let nav = Navigator::new().unwrap();
        assert!(resolve_file_operand("@definitely-not-a-savepoint-xyz", &nav).is_err());
        assert!(resolve_file_operand("@", &nav).is_err());
    }
}

