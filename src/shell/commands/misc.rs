use super::{execute_system_command, parse_search_args};
use crate::config::Config;
use crate::explorer::{
    calculate_dir_size, calculate_total_size, format_duration, human_readable_size,
};
use crate::filetype::{is_supported_format, strip_unc_prefix};
use crate::navigator::{clear_screen, Navigator};
use crate::output::{cat_file, print_error, print_info, print_success, print_warning};
use crate::search;
use crate::session::SessionState;
use crate::shell::help::{print_interactive_help, print_tp_help};
use crate::shell::helpers::{parse_target_arg, resolve_file_operand, run_with_spinner, show_tree};
use crate::shell::venv::clear_venv_environment;
use crate::teleport::TeleportManager;
use crate::watcher;

use crate::t;
use anyhow::Result;
use colored::*;
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub fn cmd_clear(_args: &str, nav: &mut Navigator) -> Result<bool> {
    clear_screen();
    println!();
    if Config::global_get_dir_view_enabled() {
        #[cfg(not(target_os = "android"))]
        {
            use colored::Colorize;
            let theme = crate::utils::theme::ThemeManager::current();
            let a = |s: &str| crate::utils::terminal::asciify(s);
            let version = env!("CARGO_PKG_VERSION");
            println!(
                "{}",
                a("╔══════════════════════════════════════════════════════════════════╗")
            );
            println!(
                "{}",
                a(&format!("{:^66}", t!("misc.clear.welcome", version)))
                    .color(theme.shell.help_header.to_colored())
                    .bold()
            );
            println!(
                "{}",
                a("╚══════════════════════════════════════════════════════════════════╝")
            );
        }
        #[cfg(target_os = "android")]
        {
            println!(
                "{}",
                crate::utils::terminal::asciify(
                    "╔══════════════════════════════════════════════════════════════════╗"
                )
            );
            println!(
                "{}",
                crate::utils::terminal::asciify(&format!(
                    "{:^66}",
                    t!("misc.clear.welcome", env!("CARGO_PKG_VERSION"))
                ))
            );
            println!(
                "{}",
                crate::utils::terminal::asciify(
                    "╚══════════════════════════════════════════════════════════════════╝"
                )
            );
        }
    }
    println!("{}", t!("misc.clear.type_help").dimmed());
    if Config::global_get_dir_view_enabled() {
        show_tree(nav, Some(1), false, false, false, false, false, false);
    }
    Ok(false)
}

pub fn cmd_math(args: &str, _nav: &mut Navigator) -> Result<bool> {
    crate::math::run(args)?;
    Ok(false)
}

pub fn cmd_version(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    #[cfg(not(target_os = "android"))]
    {
        use colored::Colorize;
        let theme = crate::utils::theme::ThemeManager::current();
        println!(
            "ntc {}",
            env!("CARGO_PKG_VERSION")
                .color(theme.shell.success.to_colored())
                .bold()
        );
    }
    #[cfg(target_os = "android")]
    {
        println!("ntc {}", env!("CARGO_PKG_VERSION"));
    }
    Ok(false)
}

pub fn cmd_help(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let topic = args.trim();
    if topic.is_empty() {
        print_interactive_help();
    } else if !crate::shell::help::print_topic_help(topic) {
        print_error(&t!("help.unknown_topic", topic));
    }
    Ok(false)
}

pub fn cmd_tutorial(_args: &str, nav: &mut Navigator) -> Result<bool> {
    crate::tutorial::run_tutorial(nav)?;
    Ok(false)
}

pub fn cmd_exit(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    Ok(true)
}

pub fn cmd_ne(args: &str, nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    let raw_path = if let Some(hyper_args) = args.strip_prefix("--hyper ") {
        let mut parts = hyper_args.trim().splitn(3, ' ');
        let name = parts.next().unwrap_or("").trim();
        let mut lang = "c";
        if let Some(p) = parts.next() {
            if p == "--lang" {
                if let Some(l) = parts.next() {
                    lang = l.trim();
                }
            }
        }
        if name.is_empty() || name.contains('.') {
            print_error(&t!("misc.ne.usage_hyper"));
            return Ok(false);
        }
        match crate::editor::init_hyper_project(name, lang) {
            Ok(_) => eprintln!("{}", t!("misc.ne.created_hyper", name)),
            Err(e) => print_error(&t!("misc.ne.hyper_error", e)),
        }
        return Ok(false);
    } else if let Some(super_path) = args.strip_prefix("--super ") {
        let p = std::path::PathBuf::from(super_path.trim());
        let _ = crate::editor::init_super_file(&p);
        eprintln!("{}", t!("misc.ne.created_super", p.display()));
        p
    } else if let Some(init_path) = args.strip_prefix("--init ") {
        let p = std::path::PathBuf::from(init_path.trim());
        let _ = crate::editor::init_file(&p);
        eprintln!("{}", t!("misc.ne.created_template", p.display()));
        p
    } else if args.is_empty() || args == "." {
        let cwd = nav.current_path();
        let welcome = cwd.join("welcome.ntcEditor");
        let _ = crate::editor::refresh_welcome_file(&welcome);
        print_info(&t!("misc.ne.opened_welcome", nav.current_path().display()));
        welcome
    } else {
        // Support "file:line" syntax (e.g. main.rs:42, src/main.rs:42)
        let trimmed = args.trim();
        if let Some((file_part, line_part)) = trimmed.rsplit_once(':') {
            if let Ok(line) = line_part.parse::<usize>() {
                if line > 0 {
                    let p = std::path::PathBuf::from(file_part);
                    crate::editor::GOTO_LINE.store(line, std::sync::atomic::Ordering::SeqCst);
                    p
                } else {
                    std::path::PathBuf::from(trimmed)
                }
            } else {
                std::path::PathBuf::from(trimmed)
            }
        } else {
            std::path::PathBuf::from(trimmed)
        }
    };
    let path = if raw_path.is_dir() {
        let welcome = raw_path.join("welcome.ntcEditor");
        let _ = crate::editor::refresh_welcome_file(&welcome);
        welcome
    } else {
        raw_path
    };
    let restored = SessionState::read_global().editor_session.clone();
    match crate::editor::edit_file_with_session(&path, restored) {
        Ok((_, captured)) => {
            SessionState::write_global().editor_session = captured;
            SessionState::save_global();
        }
        Err(e) => print_error(&t!("misc.ne.editor_error", e)),
    }
    Ok(false)
}

pub fn cmd_size(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parsed = match parse_target_arg(args, nav) {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };
    let show_path = parsed.path.is_some();
    let target = parsed
        .path
        .unwrap_or_else(|| nav.current_path().to_path_buf());

    let mut care = false;
    let mut show_time = false;
    for arg in &parsed.rest {
        match arg.as_str() {
            "--care" => care = true,
            "--time" | "-t" => show_time = true,
            _ => {}
        }
    }
    let start = std::time::Instant::now();
    let total = if care {
        run_with_spinner(&t!("misc.size.calculating"), || {
            calculate_total_size(&target)
        })
    } else {
        run_with_spinner(&t!("misc.size.calculating"), || {
            calculate_dir_size(&target)
        })
    };
    let elapsed = start.elapsed();
    let bytes_str = format!("{}", total);
    let human_str = human_readable_size(total);
    let time_str = format_duration(elapsed);
    let path_str = if show_path {
        strip_unc_prefix(&target)
    } else {
        String::new()
    };
    let max_label = 8;
    let mut max_value = bytes_str.len().max(human_str.len()).max(time_str.len());
    if show_path {
        max_value = max_value.max(path_str.len());
    }
    let w = max_label + max_value + 10;

    let bytes_label = t!("misc.size.bytes_label");
    let human_label = t!("misc.size.human_label");
    let time_label = t!("misc.size.time_label");
    let title_label = t!("misc.size.title");
    let a = |s: &str| crate::utils::terminal::asciify(s);
    let v = a("│");
    let h = a("─");
    println!();
    println!("{}{}{}", a("┌"), h.repeat(w), a("┐"));
    println!("{}{:^w$}", v, title_label.cyan().bold(), w = w);
    if show_path {
        let path_label = t!("misc.size.path_label");
        println!("{}{}{}", a("├"), h.repeat(w), a("┤"));
        println!(
            "{} {} {:<w$} ",
            v,
            path_label.yellow(),
            path_str.cyan(),
            w = w - path_label.len() - 4
        );
    }
    println!("{}{}{}", a("├"), h.repeat(w), a("┤"));
    println!(
        "{} {} {:<w$} ",
        v,
        bytes_label.yellow(),
        bytes_str.yellow(),
        w = w - bytes_label.len() - 4
    );
    println!(
        "{} {} {:<w$} ",
        v,
        human_label.green().bold(),
        human_str.green().bold(),
        w = w - human_label.len() - 4
    );
    if show_time {
        println!("{}{}{}", a("├"), h.repeat(w), a("┤"));
        println!(
            "{} {} {:<w$} ",
            v,
            time_label.cyan(),
            time_str.cyan(),
            w = w - time_label.len() - 4
        );
    }
    println!("{}{}{}", a("└"), h.repeat(w), a("┘"));
    Ok(false)
}

pub fn cmd_tp(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        TeleportManager::interactive_menu(nav)?;
    } else {
        let tp_parts: Vec<&str> = args.splitn(2, ' ').collect();
        let subcmd = tp_parts[0].to_lowercase();
        let subargs = tp_parts.get(1).unwrap_or(&"").trim();

        match subcmd.as_str() {
            "add" => {
                if subargs.is_empty() {
                    println!("{}", t!("misc.tp.usage_add"));
                } else {
                    let add_parts: Vec<&str> = subargs.splitn(2, ' ').collect();
                    let name = add_parts[0];
                    if add_parts.len() > 1 {
                        let path = std::path::Path::new(add_parts[1]);
                        TeleportManager::add(name, path.to_path_buf())?;
                    } else {
                        TeleportManager::add_current(nav, name)?;
                    }
                }
            }
            "jump" | "to" => {
                if subargs.is_empty() {
                    println!("{}", t!("misc.tp.usage_jump"));
                } else if let Ok(num) = subargs.parse::<usize>() {
                    TeleportManager::jump_by_index(nav, num)?;
                } else {
                    TeleportManager::jump_by_name(nav, subargs)?;
                }
            }
            "info" => {
                if subargs.is_empty() {
                    print_error(&t!("misc.tp.usage_info"));
                    println!("{}", t!("misc.tp.example_info"));
                } else {
                    let teleports = TeleportManager::get_all();
                    let name_lower = subargs.to_lowercase();
                    if let Some(path) = teleports.get(&name_lower) {
                        println!();
                        println!(
                            "{}",
                            "==================================================".cyan()
                        );
                        println!("{}", t!("misc.tp.info_title").cyan().bold());
                        println!(
                            "{}",
                            "==================================================".cyan()
                        );
                        println!(
                            "  {}: {}",
                            t!("misc.tp.info_name").yellow(),
                            name_lower.blue().bold()
                        );
                        println!(
                            "  {}: {}",
                            t!("misc.tp.info_path").yellow(),
                            path.display().to_string().dimmed()
                        );
                        let exists = if path.exists() {
                            "yes".green()
                        } else {
                            "no".red()
                        };
                        println!("  {}: {}", t!("misc.tp.info_exists").yellow(), exists);
                        if let Ok(canonical) = std::fs::canonicalize(path) {
                            println!(
                                "  {}: {}",
                                t!("misc.tp.info_canonical").yellow(),
                                canonical.display().to_string().dimmed()
                            );
                        }
                        let total = teleports.len();
                        let sorted: Vec<&String> = teleports.keys().collect();
                        if let Some(pos) = sorted.iter().position(|k| *k == &name_lower) {
                            println!(
                                "  {}: {} / {}",
                                t!("misc.tp.info_index").yellow(),
                                (pos + 1).to_string().yellow(),
                                total.to_string().yellow()
                            );
                        }
                        println!(
                            "  {}: {}",
                            t!("misc.tp.info_source").yellow(),
                            t!("misc.tp.info_global").green()
                        );
                        println!();
                    } else {
                        print_error(&t!("misc.tp.not_found", subargs));
                    }
                }
            }
            "list" | "ls" => {
                TeleportManager::list()?;
            }
            "rm" => {
                if subargs.is_empty() {
                    println!("{}", t!("misc.tp.usage_rm"));
                } else {
                    for arg in subargs
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                    {
                        if let Ok(num) = arg.parse::<usize>() {
                            TeleportManager::remove_by_index(num)?;
                        } else {
                            TeleportManager::remove_by_name(arg)?;
                        }
                    }
                }
            }
            "rnm" | "rename" => {
                if subargs.is_empty() {
                    println!("{}", t!("misc.tp.usage_rnm"));
                } else {
                    let parts: Vec<&str> = subargs.splitn(4, ' ').collect();
                    if parts.len() >= 3 && parts[1].to_lowercase() == "to" {
                        TeleportManager::rename(parts[0], parts[2])?;
                    } else {
                        print_error(&t!("misc.tp.invalid_format"));
                    }
                }
            }
            "cls" => {
                TeleportManager::clear_all()?;
            }
            "help" => {
                print_tp_help();
            }
            _ => {
                print_error(&t!("misc.tp.unknown_subcmd", subcmd));
                println!("{}", t!("misc.tp.type_help"));
            }
        }
    }
    Ok(false)
}

pub fn cmd_gs(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.trim().is_empty() {
        print_error(&t!("misc.gs.usage"));
        println!("{}", t!("misc.gs.example1"));
        println!("{}", t!("misc.gs.example2"));
        println!("{}", t!("misc.gs.example3"));
        println!("{}", t!("misc.gs.example4"));
        println!("{}", t!("misc.gs.example5"));
        return Ok(false);
    }
    let parsed = super::parse_gs_args(args);
    if parsed.pattern.trim().is_empty() {
        print_error(&t!("misc.gs.usage"));
        return Ok(false);
    }
    let pattern = parsed.pattern;
    let max_depth = parsed.max_depth;
    let root = nav.current_path().to_path_buf();
    // Spinner in background thread so broad `-d` searches show progress
    // instead of looking crashed. Heavy-file guards live in search_content.
    let results = run_with_spinner(&t!("misc.gs.searching"), || {
        crate::search::search_content(&root, &pattern, max_depth)
    });
    let total_found = results.len();
    let mut results = results;
    if results.len() > 20_000 {
        print_warning(&t!("misc.gs.many_results", results.len()));
        results.truncate(20_000);
    }
    // -l/--limit is non-interactive only (paged UI has its own 10/page).
    if !parsed.interactive {
        if let Some(lim) = parsed.limit {
            if results.len() > lim {
                results.truncate(lim);
            }
        }
    }
    if parsed.interactive {
        return super::gs_interactive::run_gs_interactive(results, &root, &pattern);
    }
    let output =
        crate::search::format_content_results(&results, &pattern, max_depth, &root, total_found);
    print!("{}", output);
    Ok(false)
}

pub fn cmd_fs(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("misc.fs.usage"));
        println!("{}", t!("misc.fs.example1"));
        println!("{}", t!("misc.fs.example2"));
        println!("{}", t!("misc.fs.example3"));
        return Ok(false);
    }
    let (pattern, max_depth) = parse_search_args(args);
    let results = crate::search::search_files(nav.current_path(), &pattern, max_depth);
    let output = crate::search::format_search_results(&results, &pattern, max_depth, true);
    print!("{}", output);
    Ok(false)
}

pub fn cmd_ds(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("misc.ds.usage"));
        println!("{}", t!("misc.ds.example1"));
        println!("{}", t!("misc.ds.example2"));
        println!("{}", t!("misc.ds.example3"));
        return Ok(false);
    }
    let (pattern, max_depth) = parse_search_args(args);
    let results = crate::search::search_directories(nav.current_path(), &pattern, max_depth);
    let output = crate::search::format_search_results(&results, &pattern, max_depth, false);
    print!("{}", output);
    Ok(false)
}

pub fn cmd_fsc(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("misc.fsc.usage"));
        println!("{}", t!("misc.fsc.example1"));
        println!("{}", t!("misc.fsc.example2"));
        return Ok(false);
    }
    let (pattern, max_depth) = parse_search_args(args);
    let results = crate::search::search_files(nav.current_path(), &pattern, max_depth);
    if results.is_empty() {
        let output = crate::search::format_search_results(&results, &pattern, max_depth, true);
        print!("{}", output);
        return Ok(false);
    }
    loop {
        println!();
        println!(
            "{}",
            t!("misc.fsc.found", results.len(), pattern).cyan().bold()
        );
        for (i, r) in results.iter().enumerate() {
            let path_str = r.full_path.to_string_lossy();
            println!("  {}. {}", (i + 1).to_string().yellow(), path_str.dimmed());
        }
        println!("  {}", t!("misc.fsc.done_opt").red());
        println!();
        print!("{} ", t!("misc.fsc.select_prompt").green());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input == "0" || input.is_empty() {
            println!("{}", t!("misc.fsc.done_str").dimmed());
            return Ok(false);
        }
        if let Ok(n) = input.parse::<usize>() {
            if n >= 1 && n <= results.len() {
                let target = &results[n - 1].full_path;
                if target.is_file() {
                    if is_supported_format(target) {
                        let show_lines = Config::global_get_show_line_numbers();
                        cat_file(target, show_lines)?;
                        println!();
                        print!("{} ", t!("misc.fsc.copy_prompt").yellow());
                        io::stdout().flush()?;
                        let mut copy_choice = String::new();
                        io::stdin().read_line(&mut copy_choice)?;
                        let copy_choice = copy_choice.trim().to_lowercase();
                        if copy_choice == "y" || copy_choice == "yes" {
                            let content =
                                crate::output::cat_file_with_line_numbers(target, show_lines)?;
                            crate::output::copy_to_clipboard(&content, "TXT")?;
                            print_success(&t!("misc.fsc.copied", target.display()));
                        }
                    } else {
                        print_warning(&t!("misc.fsc.skipped", target.display()));
                    }
                } else {
                    print_warning(&t!("misc.fsc.not_file"));
                }
            } else {
                print_error(&t!("misc.fsc.invalid_selection"));
            }
        } else {
            print_error(&t!("misc.fsc.invalid_input"));
        }
        println!();
    }
}

pub fn cmd_locate(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("misc.locate.usage"));
        println!("{}", t!("misc.locate.example1"));
        println!("{}", t!("misc.locate.example2"));
        println!("{}", t!("misc.locate.example3"));
        return Ok(false);
    }
    let (pattern, max_depth) = parse_search_args(args);
    let results = search::search_all(nav.current_path(), &pattern, max_depth);
    if results.is_empty() {
        println!("{}", t!("misc.locate.no_results", pattern).dimmed());
    } else {
        let has_exact = results
            .iter()
            .any(|r| r.match_kind == search::MatchKind::Exact);
        let has_partial = results
            .iter()
            .any(|r| r.match_kind == search::MatchKind::Partial);
        let label = if has_exact {
            t!("misc.locate.label_exact")
        } else if has_partial {
            t!("misc.locate.label_partial")
        } else {
            t!("misc.locate.label_didyoumean")
        };
        println!();
        println!(
            "{}",
            t!("misc.locate.header", label, pattern, results.len())
                .cyan()
                .bold()
        );
        for (i, r) in results.iter().enumerate() {
            let icon = if r.full_path.is_dir() { "📁" } else { "📄" };
            if r.match_kind == search::MatchKind::Fuzzy {
                println!(
                    "  {}. {} {} ({:.0}% match)",
                    (i + 1).to_string().yellow(),
                    icon,
                    r.full_path.to_string_lossy().dimmed(),
                    r.score * 100.0
                );
            } else {
                println!(
                    "  {}. {} {}",
                    (i + 1).to_string().yellow(),
                    icon,
                    r.full_path.to_string_lossy().dimmed()
                );
            }
        }
        println!();
    }
    Ok(false)
}

pub fn cmd_tpb(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        TeleportManager::teleport_back(nav)?;
    } else {
        let args_lower = args.to_lowercase();
        match args_lower.as_str() {
            "history" | "hist" | "h" => {
                TeleportManager::show_history()?;
            }
            "clear" | "cls" => {
                TeleportManager::clear_stack();
            }
            _ => {
                print_error(&t!("misc.tpb.unknown", args));
                println!("{}", t!("misc.tpb.usage"));
                println!("{}", t!("misc.tpb.usage_basic"));
                println!("{}", t!("misc.tpb.usage_history"));
                println!("{}", t!("misc.tpb.usage_clear"));
            }
        }
    }
    Ok(false)
}

fn split_args(args: &str) -> Vec<String> {
    args.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn cmd_mkf(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let items = split_args(args);
    if items.is_empty() {
        print_error(&t!("misc.mkf.missing"));
        return Ok(false);
    }
    for item in &items {
        let path = std::path::Path::new(item);
        if path.exists() {
            print_warning(&t!("misc.mkf.exists", item));
            continue;
        }
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
        match std::fs::write(path, "") {
            Ok(()) => print_success(&t!("misc.mkf.created", item)),
            Err(e) => print_error(&t!("misc.mkf.error", item, e)),
        }
    }
    Ok(false)
}

pub fn cmd_mkd(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let items = split_args(args);
    if items.is_empty() {
        print_error(&t!("misc.mkd.missing"));
        return Ok(false);
    }
    for item in &items {
        if let Err(e) = std::fs::create_dir_all(std::path::Path::new(item)) {
            print_error(&t!("misc.mkd.error", item, e));
        } else {
            print_success(&t!("misc.mkd.created", item));
        }
    }
    Ok(false)
}

pub fn cmd_rmd(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let items = split_args(args);
    if items.is_empty() {
        print_error(&t!("misc.rmd.missing"));
        return Ok(false);
    }
    for item in &items {
        let path = std::path::Path::new(item);
        if !path.exists() {
            print_error(&t!("misc.rmd.not_exist", item));
        } else if !path.is_dir() {
            print_error(&t!("misc.rmd.not_dir", item));
        } else if let Err(e) = std::fs::remove_dir_all(path) {
            print_error(&t!("misc.rmd.error", item, e));
        } else {
            print_success(&t!("misc.rmd.removed", item));
        }
    }
    Ok(false)
}

/// Expand one `rmf` operand containing `*` wildcards (`*.log`,
/// `test*`, `dir/*.tmp`) into the matching files. Directories never
/// match — `rmf` only removes files. Pure for testability.
fn expand_rmf_pattern(item: &str, cwd: &Path) -> Vec<PathBuf> {
    let raw = Path::new(item);
    let (base, pat) = match raw.parent().filter(|p| !p.as_os_str().is_empty()) {
        Some(dir) => {
            let base = if dir.is_absolute() {
                dir.to_path_buf()
            } else {
                cwd.join(dir)
            };
            let pat = raw
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(item);
            (base, pat.to_string())
        }
        None => (cwd.to_path_buf(), item.to_string()),
    };
    // End-anchored so `*.log` means "ends with .log", not "contains".
    let anchored = format!("{}$", pat);
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&base) {
        for entry in entries.flatten() {
            if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                continue;
            }
            let name = match entry.file_name().into_string() {
                Ok(n) => n,
                Err(_) => continue,
            };
            if crate::search::matches_wildcard(&anchored, &name) {
                out.push(base.join(name));
            }
        }
    }
    out.sort();
    out
}

/// Delete the files matched by one `rmf` wildcard operand: single match
/// goes directly, multiple matches list + confirm first.
fn remove_rmf_pattern(item: &str) -> Result<()> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let matches = expand_rmf_pattern(item, &cwd);
    if matches.is_empty() {
        print_error(&t!("misc.rmf.no_match", item));
        return Ok(());
    }
    if matches.len() > 1 {
        println!();
        println!(
            "{}",
            t!("misc.rmf.pattern_matches", matches.len(), item)
                .yellow()
                .bold()
        );
        for (i, p) in matches.iter().enumerate() {
            println!("  {}. {}", (i + 1).to_string().yellow(), p.display());
        }
        println!();
        let ok = crate::utils::interactive::prompt_yes_no(
            &t!("misc.rmf.confirm_delete", matches.len()),
            false,
        )
        .unwrap_or(false);
        if !ok {
            println!("{}", t!("misc.rmf.cancelled").dimmed());
            return Ok(());
        }
    }
    for path in &matches {
        match std::fs::remove_file(path) {
            Ok(()) => print_success(&t!("misc.rmf.removed", path.display())),
            Err(e) => print_error(&t!("misc.rmf.error", path.display(), e)),
        }
    }
    Ok(())
}

pub fn cmd_rmf(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let items = split_args(args);
    if items.is_empty() {
        print_error(&t!("misc.rmf.missing"));
        return Ok(false);
    }
    for item in &items {
        if item.contains('*') {
            remove_rmf_pattern(item)?;
            continue;
        }
        let path = std::path::Path::new(item);
        if !path.exists() {
            print_error(&t!("misc.rmf.not_exist", item));
        } else if path.is_dir() {
            print_error(&t!("misc.rmf.is_dir", item));
        } else {
            match std::fs::remove_file(path) {
                Ok(()) => print_success(&t!("misc.rmf.removed", item)),
                Err(e) => print_error(&t!("misc.rmf.error", item, e)),
            }
        }
    }
    Ok(false)
}

pub fn cmd_rnmf(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        print_error(&t!("misc.rnmf.missing"));
        println!("{}", t!("misc.rnmf.usage"));
        return Ok(false);
    }
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.len() < 2 {
        print_error(&t!("misc.rnmf.missing"));
        println!("{}", t!("misc.rnmf.usage"));
        return Ok(false);
    }
    let old = parts[0];
    let new = parts[1];
    let old_path = std::path::Path::new(old);
    let new_path = std::path::Path::new(new);
    if !old_path.exists() {
        print_error(&t!("misc.rnmf.not_exist", old));
    } else if old_path.is_dir() {
        print_error(&t!("misc.rnmf.is_dir", old));
    } else if new_path.exists() {
        print_error(&t!("misc.rnmf.exists", new));
    } else {
        match std::fs::rename(old_path, new_path) {
            Ok(()) => print_success(&t!("misc.rnmf.renamed", old, new)),
            Err(e) => print_error(&t!("misc.rnmf.error", e)),
        }
    }
    Ok(false)
}

pub fn cmd_rnmd(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        print_error(&t!("misc.rnmd.missing"));
        println!("{}", t!("misc.rnmd.usage"));
        return Ok(false);
    }
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.len() < 2 {
        print_error(&t!("misc.rnmd.missing"));
        println!("{}", t!("misc.rnmd.usage"));
        return Ok(false);
    }
    let old = parts[0];
    let new = parts[1];
    let old_path = std::path::Path::new(old);
    let new_path = std::path::Path::new(new);
    if !old_path.exists() {
        print_error(&t!("misc.rnmd.not_exist", old));
    } else if !old_path.is_dir() {
        print_error(&t!("misc.rnmd.not_dir", old));
    } else if new_path.exists() {
        print_error(&t!("misc.rnmd.exists", new));
    } else {
        match std::fs::rename(old_path, new_path) {
            Ok(()) => print_success(&t!("misc.rnmd.renamed", old, new)),
            Err(e) => print_error(&t!("misc.rnmd.error", e)),
        }
    }
    Ok(false)
}

pub fn cmd_ui(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    println!("{}", t!("misc.ui.removed"));
    Ok(false)
}

/// `mv <src> <dst> [-f]` — move/rename files or directories.
/// `dst` may be a teleport savepoint (`mv report.pdf @docs`).
/// Quoted paths with spaces supported; `-f/--force` overwrites the target.
/// Falls back to copy+delete when `rename` fails (e.g. cross-device).
pub fn cmd_mv(args: &str, nav: &mut Navigator) -> Result<bool> {
    let (operands, force) = parse_mv_args(args);
    if operands.len() < 2 {
        print_error(&t!("misc.mv.missing"));
        println!("{}", t!("misc.mv.usage"));
        return Ok(false);
    }
    let src = resolve_plain(&operands[0], nav);
    let mut dst = match resolve_file_operand(&operands[1], nav) {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };

    if !src.exists() {
        print_error(&t!("misc.mv.not_exist", operands[0]));
        return Ok(false);
    }
    // `mv file existing_dir/` moves inside the directory (never refuses).
    if dst.is_dir() {
        match src.file_name() {
            Some(name) => dst.push(name),
            None => {
                print_error(&t!("misc.mv.error", "invalid source name"));
                return Ok(false);
            }
        }
    }
    if dst.exists() && !force {
        print_error(&t!("misc.mv.exists", operands[1]));
        return Ok(false);
    }
    if force && dst.exists() {
        let cleared = if dst.is_dir() && !src.is_dir() {
            // Never replace a directory with a file.
            print_error(&t!("misc.mv.error", "cannot overwrite directory with file"));
            return Ok(false);
        } else if dst.is_dir() {
            std::fs::remove_dir_all(&dst).is_ok()
        } else {
            std::fs::remove_file(&dst).is_ok()
        };
        if !cleared {
            print_error(&t!("misc.mv.error", "cannot clear existing target"));
            return Ok(false);
        }
    }

    match std::fs::rename(&src, &dst) {
        Ok(()) => print_success(&t!("misc.mv.moved", operands[0], operands[1])),
        Err(_) => match move_by_copy(&src, &dst) {
            Ok(()) => print_success(&t!("misc.mv.moved", operands[0], operands[1])),
            Err(e) => print_error(&t!("misc.mv.error", e)),
        },
    }
    Ok(false)
}

/// `cp <src> <dst> [-f]` — copy files or directories.
/// `dst` may be a teleport savepoint (`cp report.pdf @docs`).
/// Quoted paths with spaces supported; `-f/--force` overwrites the target.
pub fn cmd_cp(args: &str, nav: &mut Navigator) -> Result<bool> {
    let (operands, force) = parse_mv_args(args);
    if operands.len() < 2 {
        print_error(&t!("misc.cp.missing"));
        println!("{}", t!("misc.cp.usage"));
        return Ok(false);
    }
    let src = resolve_plain(&operands[0], nav);
    let mut dst = match resolve_file_operand(&operands[1], nav) {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };

    if !src.exists() {
        print_error(&t!("misc.cp.not_exist", operands[0]));
        return Ok(false);
    }
    // `cp file existing_dir/` copies inside the directory (never refuses).
    if dst.is_dir() {
        match src.file_name() {
            Some(name) => dst.push(name),
            None => {
                print_error(&t!("misc.cp.error", "invalid source name"));
                return Ok(false);
            }
        }
    }
    if same_file(&src, &dst) {
        print_error(&t!("misc.cp.same", operands[0]));
        return Ok(false);
    }
    if dst.exists() && !force {
        print_error(&t!("misc.cp.exists", operands[1]));
        return Ok(false);
    }
    if force && dst.exists() {
        let cleared = if dst.is_dir() && !src.is_dir() {
            // Never replace a directory with a file.
            print_error(&t!("misc.cp.error", "cannot overwrite directory with file"));
            return Ok(false);
        } else if dst.is_dir() {
            std::fs::remove_dir_all(&dst).is_ok()
        } else {
            std::fs::remove_file(&dst).is_ok()
        };
        if !cleared {
            print_error(&t!("misc.cp.error", "cannot clear existing target"));
            return Ok(false);
        }
    }

    match copy_path(&src, &dst) {
        Ok(()) => print_success(&t!("misc.cp.copied", operands[0], operands[1])),
        Err(e) => print_error(&t!("misc.cp.error", e)),
    }
    Ok(false)
}

/// Plain path operand: absolute or relative to the shell's directory.
fn resolve_plain(spec: &str, nav: &Navigator) -> PathBuf {
    let p = PathBuf::from(spec);
    if p.is_absolute() {
        p
    } else {
        nav.current_path().join(p)
    }
}

/// True when both paths point at the same file (guards `cp` onto itself).
fn same_file(a: &Path, b: &Path) -> bool {
    if a == b {
        return true;
    }
    match (std::fs::canonicalize(a), std::fs::canonicalize(b)) {
        (Ok(x), Ok(y)) => x == y,
        _ => false,
    }
}

/// Copy `src` to `dst` then remove `src` (cross-device `mv` fallback).
fn move_by_copy(src: &Path, dst: &Path) -> std::io::Result<()> {
    copy_path(src, dst)?;
    if src.is_dir() {
        std::fs::remove_dir_all(src)?;
    } else {
        std::fs::remove_file(src)?;
    }
    Ok(())
}

/// Copy a file or directory tree to `dst` (no removal).
fn copy_path(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        copy_dir_all(src, dst)?;
    } else {
        if let Some(parent) = dst.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::copy(src, dst)?;
    }
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in walkdir::WalkDir::new(src).min_depth(1) {
        let entry = entry.map_err(|e| {
            std::io::Error::other(format!("walk {}: {}", src.display(), e))
        })?;
        let rel = entry.path().strip_prefix(src).map_err(std::io::Error::other)?;
        let target = dst.join(rel);
        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&target)?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}

/// Split `mv` args into operands + force flag.
/// Respects double quotes: `mv "my file.txt" sub/ -f`.
fn parse_mv_args(args: &str) -> (Vec<String>, bool) {
    let mut operands: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut force = false;
    let push_token = |tok: String, operands: &mut Vec<String>, force: &mut bool| {
        if tok == "-f" || tok == "--force" {
            *force = true;
        } else if !tok.is_empty() {
            operands.push(tok);
        }
    };
    for ch in args.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    push_token(std::mem::take(&mut current), &mut operands, &mut force);
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        push_token(current, &mut operands, &mut force);
    }
    (operands, force)
}

// In src/shell/commands/misc.rs - Replace the cmd_venva function:

pub fn cmd_venva(args: &str, nav: &mut Navigator) -> Result<bool> {
    use crate::shell::venv::*;

    // Check if already in a venv
    if env::var("VIRTUAL_ENV").is_ok() {
        print_warning(&t!("misc.venva.already_in"));
        return Ok(false);
    }

    let current_path = nav.current_path().to_path_buf();

    let venv_path = if args.is_empty() {
        // Auto-detect venv in current directory
        let venv_names = &["venv", ".venv", "env"];
        let mut found = None;

        for name in venv_names {
            let venv_dir = current_path.join(name);
            if venv_dir.exists() {
                found = Some(venv_dir);
                break;
            }
        }

        if found.is_none() {
            print_error(&t!("misc.venva.not_found"));
            return Ok(false);
        }
        found.unwrap()
    } else {
        let path = Path::new(args);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            current_path.join(args)
        }
    };

    // Find the activate script
    let activate_script = get_activate_script_path(&venv_path);
    let activate_path = match activate_script {
        Some(p) => p,
        None => {
            print_error(&t!("misc.venva.no_script", venv_path.display()));
            return Ok(false);
        }
    };

    print_info(&t!("misc.venva.activating", venv_path.display()));

    // Parse and apply the environment
    #[cfg(windows)]
    let changes = parse_batch_activate(&activate_path);
    #[cfg(not(windows))]
    let changes = parse_bash_activate(&activate_path);

    match changes {
        Ok(env_changes) => {
            apply_venv_environment(env_changes);

            // Update the navigator's venv name
            let new_venv_name = detect_active_venv();
            nav.update_venv_name(new_venv_name);

            // Verify activation by checking if pip is in PATH
            check_pip_in_path(&venv_path);

            print_success(&t!("misc.venva.activated", venv_path.display()));
            print_info(&t!("misc.venva.active_info"));
        }
        Err(e) => {
            print_error(&t!("misc.venva.failed", e));
        }
    }

    Ok(false)
}

// In src/shell/commands/misc.rs - Add/enhance this function:

pub fn cmd_show_env(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    let show_all = args == "--all" || args == "-a";
    let depth: Option<usize> = if let Some(depth_str) = args.strip_prefix("-d ") {
        depth_str.parse::<usize>().ok()
    } else if let Some(depth_str) = args.strip_prefix("--depth ") {
        depth_str.parse::<usize>().ok()
    } else {
        None
    };

    println!();
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║                         Environment Variables                    ║".cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════════╝".cyan()
    );
    println!();

    // Core venv variables (always shown)
    println!("{}", t!("misc.show_env.venv_header").green().bold());
    match std::env::var("VIRTUAL_ENV") {
        Ok(val) => println!("  VIRTUAL_ENV = {}", val.cyan()),
        Err(_) => println!("  VIRTUAL_ENV = {}", t!("misc.show_env.not_set").dimmed()),
    }
    println!();

    // PATH is always useful to see
    println!("{}", t!("misc.show_env.path_header").yellow().bold());
    if let Ok(path) = std::env::var("PATH") {
        let entries: Vec<&str> = path.split(if cfg!(windows) { ';' } else { ':' }).collect();
        let max_display = depth.unwrap_or(5);

        for (i, entry) in entries.iter().enumerate().take(max_display) {
            let marker =
                if entry.contains("venv") || entry.contains(".venv") || entry.contains("env") {
                    "🐍"
                } else {
                    "  "
                };
            println!(
                "  {}. {} {}",
                (i + 1).to_string().yellow(),
                marker,
                entry.dimmed()
            );
        }

        if entries.len() > max_display {
            let remaining = entries.len() - max_display;
            println!(
                "  {}",
                t!("misc.show_env.and_more", remaining.to_string()).dimmed()
            );
        }

        if show_all {
            println!();
            println!("{}", t!("misc.show_env.path_full").yellow().bold());
            for (i, entry) in entries.iter().enumerate() {
                let marker =
                    if entry.contains("venv") || entry.contains(".venv") || entry.contains("env") {
                        "🐍"
                    } else {
                        "  "
                    };
                println!("  {}. {} {}", (i + 1).to_string().yellow(), marker, entry);
            }
        }
    }
    println!();

    // System information
    println!("{}", t!("misc.show_env.sys_header").blue().bold());
    println!(
        "  {} {}",
        t!("misc.show_env.os_label").cyan(),
        std::env::consts::OS.cyan()
    );
    println!(
        "  {} ntc v{}",
        t!("misc.show_env.shell_label").cyan(),
        env!("CARGO_PKG_VERSION").cyan()
    );
    println!(
        "  {} {}",
        t!("misc.show_env.cwd_label").cyan(),
        _nav.display_path().dimmed()
    );
    println!();

    // Optional: Show other venv-related variables if they exist
    let other_vars = [
        "PYTHONHOME",
        "PYTHONPATH",
        "_OLD_VIRTUAL_PATH",
        "_OLD_VIRTUAL_PYTHONHOME",
        "CONDA_DEFAULT_ENV",
        "CONDA_PREFIX",
    ];

    let mut has_others = false;
    for var in &other_vars {
        if let Ok(val) = std::env::var(var) {
            if !has_others {
                println!("{}", t!("misc.show_env.other_header").magenta().bold());
                has_others = true;
            }
            println!("  {} = {}", var.cyan(), val.dimmed());
        }
    }

    if has_others {
        println!();
    }

    // Summary
    println!("{}", t!("misc.show_env.summary_header").cyan().bold());
    if std::env::var("VIRTUAL_ENV").is_ok() {
        println!(
            "  {} {} {}",
            "✅",
            t!("misc.show_env.active"),
            "ACTIVE".green().bold()
        );
        println!("  {}", t!("misc.show_env.deactivate_hint"));
    } else {
        println!(
            "  {} {} {}",
            "❌",
            t!("misc.show_env.not_active"),
            "NOT ACTIVE".red().bold()
        );
        println!("  {}", t!("misc.show_env.activate_hint"));
    }
    println!();

    Ok(false)
}

// Helper function to verify pip is accessible
fn check_pip_in_path(venv_path: &Path) {
    #[cfg(windows)]
    let pip_path = venv_path.join("Scripts").join("pip.exe");
    #[cfg(not(windows))]
    let pip_path = venv_path.join("bin").join("pip");

    if pip_path.exists() {
        print_info(&t!("misc.pip.found", pip_path.display()));
    } else {
        print_warning(&t!("misc.pip.not_found"));
    }
}

pub fn cmd_venvd(_args: &str, nav: &mut Navigator) -> Result<bool> {
    if env::var("VIRTUAL_ENV").is_err() {
        print_info(&t!("misc.venvd.not_in_venv"));
        return Ok(false);
    }
    clear_venv_environment();
    let current_path = nav.current_path().to_path_buf();
    nav.go_to(&current_path)?; // Refresh display
    print_success(&t!("misc.venvd.deactivated"));
    Ok(false)
}

pub fn cmd_pathcp(_args: &str, nav: &mut Navigator) -> Result<bool> {
    let path = nav.current_path().to_string_lossy().to_string();
    crate::output::copy_to_clipboard(&path, "path")?;
    print_success(&t!("misc.pathcp.copied", path));
    Ok(false)
}

const MAX_SAY_LOOP: i32 = 10000;

fn tokenize_quoted(input: &str) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in input.chars() {
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

pub fn cmd_say(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize_quoted(args);

    let mut text = String::new();
    let mut loop_count: i32 = 1;
    let mut use_tabs = false; // -t → space-separated single line; else newline-separated

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        if token == "--loop" {
            i += 1;
            if i < tokens.len() {
                if let Ok(n) = tokens[i].parse::<i32>() {
                    if n <= 0 {
                        print_warning(&t!("misc.say.loop_warning"));
                        loop_count = 1;
                    } else if n > MAX_SAY_LOOP {
                        print_warning(&t!("misc.say.loop_exceed", MAX_SAY_LOOP));
                        loop_count = MAX_SAY_LOOP;
                    } else {
                        loop_count = n;
                    }
                } else {
                    print_warning(&t!("misc.say.loop_invalid", tokens[i]));
                }
            } else {
                print_warning(&t!("misc.say.loop_requires"));
            }
        } else if token == "-n" {
            use_tabs = false;
        } else if token == "-t" {
            use_tabs = true;
        } else if token == "--" {
            break;
        } else {
            if !text.is_empty() {
                text.push(' ');
            }
            text.push_str(token);
        }
        i += 1;
    }

    if text.is_empty() {
        print_error(&t!("misc.say.usage"));
        print_info(&t!("misc.say.example1"));
        print_info(&t!("misc.say.example2"));
        return Ok(false);
    }

    if use_tabs {
        let line = (0..loop_count)
            .map(|_| text.as_str())
            .collect::<Vec<&str>>()
            .join(" ");
        println!("{}", line);
    } else {
        for _ in 0..loop_count {
            println!("{}", text);
        }
    }

    Ok(false)
}

pub fn cmd_fallback(
    input: &str,
    nav: &mut Navigator,
    _watcher_handle: &Option<Arc<watcher::WatcherHandle>>,
) -> Result<bool> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0].to_lowercase();

    if cmd.starts_with('@') && cmd.len() > 1 {
        let tp_name = &cmd[1..];
        if TeleportManager::get_all().contains_key(&tp_name.to_lowercase()) {
            TeleportManager::jump_by_name(nav, tp_name)?;
            return Ok(false);
        }
    }

    // Try math expression evaluation for inputs that look like math
    let trimmed = input.trim();
    if crate::math::is_math_expression(trimmed) {
        if let Err(e) = crate::math::run(trimmed) {
            print_error(&format!("{}", e));
        }
        return Ok(false);
    }

    let cmd_part = input.trim();
    if !cmd_part.is_empty() {
        if let Err(e) = execute_system_command(cmd_part, nav.current_path()) {
            print_error(&format!("{}", e));
            return Ok(false);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mv_args_basic_and_force() {
        let (ops, force) = parse_mv_args("a.txt b.txt");
        assert_eq!(ops, vec!["a.txt", "b.txt"]);
        assert!(!force);
        let (ops, force) = parse_mv_args("a.txt sub/ -f");
        assert_eq!(ops, vec!["a.txt", "sub/"]);
        assert!(force);
        let (ops, force) = parse_mv_args("--force \"my file.txt\" \"new name.txt\"");
        assert_eq!(ops, vec!["my file.txt", "new name.txt"]);
        assert!(force);
    }

    #[test]
    fn parse_mv_args_needs_two_operands() {
        let (ops, _) = parse_mv_args("only-one");
        assert_eq!(ops.len(), 1);
        let (ops, _) = parse_mv_args("");
        assert!(ops.is_empty());
    }

    #[test]
    fn rmf_pattern_expands_to_matching_files_only() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.log"), "a").unwrap();
        std::fs::write(dir.path().join("b.log"), "b").unwrap();
        std::fs::write(dir.path().join("c.txt"), "c").unwrap();
        std::fs::create_dir(dir.path().join("d.log")).unwrap();

        // `*.log` matches files only — the `d.log` directory is skipped.
        let got = expand_rmf_pattern("*.log", dir.path());
        assert_eq!(
            got,
            vec![
                dir.path().join("a.log"),
                dir.path().join("b.log"),
            ]
        );
        // Prefix form and no-match form.
        assert_eq!(expand_rmf_pattern("a*", dir.path()).len(), 1);
        assert!(expand_rmf_pattern("*.zzz", dir.path()).is_empty());

        // `sub/*.log` scopes into a subdirectory.
        std::fs::create_dir(dir.path().join("sub")).unwrap();
        std::fs::write(dir.path().join("sub/e.log"), "e").unwrap();
        let got = expand_rmf_pattern("sub/*.log", dir.path());
        assert_eq!(got, vec![dir.path().join("sub/e.log")]);
    }
}
