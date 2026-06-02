use crate::config::Config;
use crate::explorer::count_dirs_in_tree;
use crate::filetype::{FormatConfig, is_supported_format};
use crate::output::{cat_file, print_error, print_success, print_warning};
use crate::report::{generate_report, generate_report_to, ReportFormat};
use anyhow::{bail, Result};
use clap::{Arg, ArgAction, Command};
use colored::*;
use indicatif::ProgressBar;
use std::path::Path;

/// Pre‑process raw args to accept `-say`, `-print` as long options.
fn preprocess_args(args: Vec<String>) -> Vec<String> {
    args.into_iter()
        .map(|arg| match arg.as_str() {
            "-say" | "-print" => "--say".to_string(),
            _ => arg,
        })
        .collect()
}

// Add this helper function
fn print_logo() {
    println!();
    println!("  {}  {}  {}", 
        "N".blue().bold(), 
        "T".blue().bold(), 
        "C".blue().bold()
    );
    println!("  {} {} {}", 
        "Navigate".red(), 
        "Tree".green(), 
        "Cat".blue()
    );
    println!();
}

/// Parse command-line arguments and execute the appropriate action.
pub fn run_cli() -> Result<bool> {
    let raw_args: Vec<String> = std::env::args().collect();

    // --- Handle @teleport shortcut from command line ---
    // Check if first argument starts with @ (e.g., ntc @web)
    if raw_args.len() == 2 {
        let arg = &raw_args[1];
        if (arg.starts_with('@') || arg.starts_with('#')) && arg.len() > 1 {
            let tp_name = &arg[1..];
            use crate::navigator::Navigator;
            use crate::teleport::TeleportManager;

            // Get the path without canonicalizing
            if let Some(path) = TeleportManager::get_path(tp_name) {
                // Clean the path - remove Windows extended prefix if present
                let clean_path = path.to_string_lossy().to_string();
                let clean_path = if clean_path.starts_with(r"\\?\") {
                    Path::new(&clean_path[4..]).to_path_buf()
                } else {
                    path.clone()
                };
                
                // Create navigator and manually set current_dir
                let mut nav = Navigator::new()?;
                
                // Try to go to the cleaned path
                match nav.go_to(&clean_path) {
                    Ok(()) => {
                        println!("✓ Teleported to '{}' -> {}", tp_name, clean_path.display());
                        println!();
                        print_logo();
                        crate::shell::run_shell_with_nav(nav)?;
                        return Ok(false);
                    }
                    Err(e) => {
                        print_error(&format!("Failed to teleport to '{}': {}", tp_name, e));
                        return Ok(false);
                    }
                }
            } else {
                print_error(&format!("Teleport point not found: '{}'", tp_name));
                println!("Use 'ntc --tp-list' to see all savepoints.");
                return Ok(false);
            }
        }
    }
    let args = preprocess_args(raw_args);

    let known_flags = vec![
        "-i, --input <path>          Input file or directory",
        "-o, --output <file>         Output filename",
        "--cp                        Copy report to clipboard",
        "--setO [path]               Show or set output directory",
        "--setD [depth]              Show or set max depth (1-20)",
        "--setL [ON|OFF]             Show or set line numbers",
        "--setC [ON|OFF]             Show or set color output",
        "--setT [threads]            Show or set thread count",
        "--setH [path|ON|OFF]        Show or set history path/state",
        "--showcg                    Show current configuration overview",
        "--watch [ON|OFF]            Show or set file watcher state",
        "-say, -print <text>         Print text to stdout",
        "--size                      Show current directory size",
        "--view                      Quick view of current directory tree",
        "--view --size               Quick view with directory sizes",
        "--ignored                   Show ignored items",
        "--ignore <name>             Ignore a directory name",
        "--cared <name>              Stop ignoring a directory",
        "--ignoref <ext>             Ignore a file extension",
        "--caref <ext>               Care about a file extension",
        "--ignoren <file>            Ignore a specific file",
        "--caren <file>              Care about a specific file",
        "--clear                     Clear the terminal screen",
        "--version                   Show version information",
        "--where                     Show ntc executable and config location",
        "--list, --fun               List all command-line functions",
        "--help                      Show help",
        "--tp-add <name>             Save current directory as teleport point",
        "--tp-list                   List all teleport points", 
        "--tp-rm <name>              Remove teleport point",
        "ntc @<name>                 Launch and teleport to savepoint",
        "(no args)                   Launch interactive mode",
    ];

    let matches = Command::new("ntc")
        .author("NuengCoder")
        .about("Navigate, Tree, Cat - Directory tree viewer and file concatenator")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("PATH")
                .help("Input file or directory path")
                .num_args(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output filename for report")
                .num_args(1),
        )
        .arg(
            Arg::new("setO")
                .long("setO")
                .value_name("PATH")
                .help("Show or set the default output directory")
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("setD")
                .long("setD")
                .value_name("DEPTH")
                .help("Show or set max recursion depth")
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("setL")
                .long("setL")
                .value_name("STATE")
                .help("Show or set line number display (ON/OFF)")
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("setT")
                .long("setT")
                .value_name("THREADS")
                .help("Show or set number of threads")
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("setC")
                .long("setC")
                .value_name("STATE")
                .help("Show or set color output (ON/OFF)")
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("setH")
                .long("setH")
                .value_name("VALUE")
                .help("Show or set history (path/ON/OFF/default)")
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("showcg")
                .long("showcg")
                .help("Show current configuration overview")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("watch")
                .long("watch")
                .value_name("STATE")
                .help("Show or set file watcher (ON/OFF)")
                .num_args(0..=1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("say")
                .short('s')
                .long("say")
                .visible_alias("print")
                .value_name("TEXT")
                .help("Print text to stdout")
                .num_args(1),
        )
        .arg(
            Arg::new("clear")
                .long("clear")
                .help("Clear the terminal screen")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("version")
                .long("version")
                .help("Show version information")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("list")
                .long("list")
                .visible_alias("fun")
                .help("List all command-line functions")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("help_extra")
                .long("help")
                .help("Show detailed help")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("ignored").long("ignored").help("Show ignored items").action(ArgAction::SetTrue))
        .arg(Arg::new("ignore").long("ignore").value_name("NAME").help("Ignore one or more directory names").num_args(1..))
        .arg(Arg::new("cared").long("cared").value_name("NAME").help("Stop ignoring one or more directories").num_args(1..))
        .arg(Arg::new("ignoref").long("ignoref").value_name("EXT").help("Ignore one or more file extensions").num_args(1..))
        .arg(Arg::new("caref").long("caref").value_name("EXT").help("Care about one or more file extensions").num_args(1..))
        .arg(Arg::new("ignoren").long("ignoren").value_name("FILE").help("Ignore one or more specific files").num_args(1..))
        .arg(Arg::new("caren").long("caren").value_name("FILE").help("Care about one or more specific files").num_args(1..))
        .arg(
            Arg::new("size")
                .long("size")
                .help("Show current directory size")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("view_cli")
                .long("view")
                .help("Quick view of current directory tree")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("where_cli")
                .long("where")
                .help("Show ntc executable location")
                .action(ArgAction::SetTrue),
        )
        // In cli.rs, add these args to the Command builder:
        .arg(
            Arg::new("tp_add")
                .long("tp-add")
                .value_name("NAME")
                .help("Save current directory as teleport point")
                .num_args(1),
        )
        .arg(
            Arg::new("tp_list")
                .long("tp-list")
                .help("List all teleport points")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("tp_rm")
                .long("tp-rm")
                .value_name("NAME")
                .help("Remove teleport point")
                .num_args(1),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Output format: txt, html, json, md")
                .num_args(1)
                .value_parser(["txt", "html", "json", "md"]),
        )
        .arg(
            Arg::new("copy")
                .long("cp")
                .help("Copy report to clipboard instead of saving to file")
                .action(ArgAction::SetTrue),
        )
        .try_get_matches_from(args)?;

    // --- Handle --version ---
    if matches.get_flag("version") {
        println!("ntc {}", env!("CARGO_PKG_VERSION").green().bold());
        return Ok(false);
    }

    // --- Handle --where ---
    if matches.get_flag("where_cli") {
        let exe = std::env::current_exe().unwrap_or_default();
        let cwd = std::env::current_dir().unwrap_or_default();
        println!("ntc executable: {}", exe.display());
        println!("Current directory: {}", cwd.display());
        return Ok(false);
    }

    // --- Handle --clear ---
    if matches.get_flag("clear") {
        let _ = std::process::Command::new("cmd").args(&["/c", "cls"]).status();
        return Ok(false);
    }

    // --- Handle --list / --fun ---
    if matches.get_flag("list") {
        println!("ntc {} - Available command-line functions:\n", env!("CARGO_PKG_VERSION").green().bold());
        for flag in &known_flags {
            println!("  {}", flag);
        }
        return Ok(false);
    }

    // --- Handle --help ---
    if matches.get_flag("help_extra") {
        print_help();
        return Ok(false);
    }

    // --- Handle --showcg ---
    if matches.get_flag("showcg") {
        let w = 65;
        println!();
        println!("┌{}┐", "─".repeat(w));
        println!("│{:^w$}│", "Current Configuration", w = w);
        println!("├{}┤", "─".repeat(w));
        println!("│ {:<20} {:<42} │", "Output Path:", Config::global_get_output_path().display().to_string());
        println!("│ {:<20} {:<42} │", "Max Depth:", Config::global_get_max_depth().to_string());
        println!("│ {:<20} {:<42} │", "Line Numbers:", if Config::global_get_show_line_numbers() { "ON" } else { "OFF" });
        println!("│ {:<20} {:<42} │", "Threads:", Config::global_get_num_threads().to_string());
        println!("│ {:<20} {:<42} │", "History:", if Config::global_get_history_enabled() { "ON" } else { "OFF" });
        println!("│ {:<20} {:<42} │", "Watcher:", if Config::global_get_file_watcher_enabled() { "ON" } else { "OFF" });
        println!("│ {:<20} {:<42} │", "Color:", if Config::global_get_color_enabled() { "ON" } else { "OFF" });
        println!("└{}┘", "─".repeat(w));
        println!();
        return Ok(false);
    }

    // --- Handle --watch ---
    if let Some(val) = matches.get_one::<String>("watch") {
        if val.is_empty() {
            let enabled = Config::global_get_file_watcher_enabled();
            println!("File watcher: {}", if enabled { "ON" } else { "OFF" });
        } else {
            let upper = val.to_uppercase();
            if upper == "ON" {
                Config::global_set_file_watcher_enabled(true);
                print_success("File watcher: ON (restart ntc to activate)");
            } else if upper == "OFF" {
                Config::global_set_file_watcher_enabled(false);
                print_warning("File watcher: OFF (restart ntc to deactivate)");
            } else {
                print_error("Use --watch ON or --watch OFF");
            }
        }
        return Ok(false);
    } else if matches.contains_id("watch") {
        let enabled = Config::global_get_file_watcher_enabled();
        println!("File watcher: {}", if enabled { "ON" } else { "OFF" });
        return Ok(false);
    }

    // --- Handle --ignored ---
    if matches.get_flag("ignored") {
        let dirs = Config::global_get_ignored_dirs();
        let fmt_cfg = FormatConfig::from_global();
        println!("Ignored directories: {:?}", dirs);
        println!("Ignored extensions: {:?}", fmt_cfg.ignored_extensions);
        println!("Extra supported extensions: {:?}", fmt_cfg.extra_extensions);
        println!("Ignored files: {:?}", fmt_cfg.ignored_files);
        println!("Extra supported files: {:?}", fmt_cfg.extra_files);
        return Ok(false);
    }

    // --- Handle --ignore ---
    if let Some(names) = matches.get_many::<String>("ignore") {
        for name in names {
            Config::global_add_ignored_dir(name);
            print_success(&format!("Now ignoring directory: {}", name));
        }
        return Ok(false);
    }

    // --- Handle --cared ---
    if let Some(names) = matches.get_many::<String>("cared") {
        for name in names {
            Config::global_remove_ignored_dir(name);
            print_success(&format!("No longer ignoring directory: {}", name));
        }
        return Ok(false);
    }

    // --- Handle --ignoref ---
    if let Some(exts) = matches.get_many::<String>("ignoref") {
        for ext in exts {
            Config::global_add_ignored_extension(ext);
            print_success(&format!("Now ignoring .{} files", ext));
        }
        return Ok(false);
    }

    // --- Handle --caref ---
    if let Some(exts) = matches.get_many::<String>("caref") {
        for ext in exts {
            Config::global_add_extra_supported_extension(ext);
            print_success(&format!("Now caring about .{} files", ext));
        }
        return Ok(false);
    }

    // --- Handle --ignoren ---
    if let Some(files) = matches.get_many::<String>("ignoren") {
        for file in files {
            Config::global_add_ignored_file(file);
            print_success(&format!("Now ignoring file: {}", file));
        }
        return Ok(false);
    }

    // --- Handle --caren ---
    if let Some(files) = matches.get_many::<String>("caren") {
        for file in files {
            Config::global_add_extra_supported_file(file);
            print_success(&format!("Now caring about file: {}", file));
        }
        return Ok(false);
    }

    // --- Handle --setH ---
    if let Some(val) = matches.get_one::<String>("setH") {
        if val.is_empty() {
            let enabled = Config::global_get_history_enabled();
            let path = Config::global_get_history_path();
            println!("History: {}", if enabled { "ON" } else { "OFF" });
            match path {
                Some(p) => println!("History path: {}", p.display()),
                None => println!("History path: default"),
            }
        } else {
            let upper = val.to_uppercase();
            if upper == "ON" {
                Config::global_set_history_enabled(true);
                print_success("History: ON");
            } else if upper == "OFF" {
                Config::global_set_history_enabled(false);
                print_warning("History: OFF");
            } else if val == "default" {
                Config::global_set_history_path(None);
                print_success("History path reset to default");
            } else {
                let p = Path::new(val);
                Config::global_set_history_path(Some(p.to_path_buf()));
                print_success(&format!("History path set to: {}", p.display()));
            }
        }
        return Ok(false);
    } else if matches.contains_id("setH") {
        let enabled = Config::global_get_history_enabled();
        let path = Config::global_get_history_path();
        println!("History: {}", if enabled { "ON" } else { "OFF" });
        match path {
            Some(p) => println!("History path: {}", p.display()),
            None => println!("History path: default"),
        }
        return Ok(false);
    }

    // --- Handle --size ---
    if matches.get_flag("size") {
        let show_view = matches.get_flag("view_cli");
        use crate::navigator::Navigator;
        let nav = Navigator::new()?;
        let total = crate::explorer::calculate_dir_size(nav.current_path());
        println!("Path: {}", nav.display_path());
        println!("┌─────────────────────────────────────────┐");
        println!("│ Current Directory Size                  │");
        println!("│ Bytes: {:>32} │", format!("{}", total));
        println!("│ Human: {:>32} │", crate::explorer::human_readable_size(total));
        println!("└─────────────────────────────────────────┘");
        if show_view {
            println!();
            let total = crate::explorer::count_entries(
                &nav.current_path().to_string_lossy(),
                Some(1),
            );
            let tree_pb = indicatif::ProgressBar::new(total);
            tree_pb.set_style(
                indicatif::ProgressStyle::with_template("ViewD  [{bar:30}] {percent}% {msg}")
                    .unwrap()
                    .progress_chars("=> "),
            );
            tree_pb.set_message("Building tree...");

            let mut tree = crate::explorer::generate_tree(
                &nav.current_path().to_string_lossy(),
                Some(1),
                true,
                Some(&tree_pb),
            );
            tree_pb.finish_with_message("Done");

            let total_dirs = count_dirs_in_tree(&tree);

            let scan_pb = ProgressBar::new(total_dirs);
            scan_pb.set_style(
                indicatif::ProgressStyle::with_template("ScanB  [{bar:30}] {percent}% {msg}")
                    .unwrap()
                    .progress_chars("=> "),
            );
            scan_pb.set_message("Calculating sizes...");
            
            crate::explorer::compute_tree_sizes(&mut tree, Some(&scan_pb));

            let tree_str = crate::explorer::format_tree_with_sizes(&tree, "", true, true, Some(&scan_pb));
            scan_pb.finish_and_clear();
            println!("{}", tree_str);
        }
        return Ok(false);
    }

    // --- Handle --view (without --size) ---
    if matches.get_flag("view_cli") && !matches.get_flag("size") {
        use crate::navigator::Navigator;
        let nav = Navigator::new()?;

        let total = crate::explorer::count_entries(
            &nav.current_path().to_string_lossy(),
            Some(1),
        );
        let pb = indicatif::ProgressBar::new(total);
        pb.set_style(
            indicatif::ProgressStyle::with_template("ViewD  [{bar:30}] {percent}% {msg}")
                .unwrap()
                .progress_chars("=> "),
        );
        pb.set_message("Building tree...");

        let tree = crate::explorer::generate_tree(
            &nav.current_path().to_string_lossy(),
            Some(1),
            true,
            Some(&pb),
        );
        pb.finish_with_message("Done");

        let tree_str = crate::explorer::format_tree_with_sizes(&tree, "", true, false, None);
        println!("{}", tree_str);
        return Ok(false);
    }

    // --- Handle setO, setD, setL, setT ---
    if let Some(val) = matches.get_one::<String>("setO") {
        if val.is_empty() {
            println!("Current output path: {}", Config::global_get_output_path().display());
        } else {
            Config::global_set_output_path(Path::new(val));
            print_success(&format!("Output path set to: {}", val));
        }
        return Ok(false);
    } else if matches.contains_id("setO") {
        println!("Current output path: {}", Config::global_get_output_path().display());
        return Ok(false);
    }

    if let Some(val) = matches.get_one::<String>("setD") {
        if val.is_empty() {
            println!("Current max depth: {}", Config::global_get_max_depth());
        } else {
            match val.parse::<usize>() {
                Ok(depth) => {
                    Config::global_set_max_depth(depth);
                    print_success(&format!("Max depth set to: {}", Config::global_get_max_depth()));
                }
                Err(_) => bail!("Invalid depth value: {}. Must be a positive integer.", val),
            }
        }
        return Ok(false);
    } else if matches.contains_id("setD") {
        println!("Current max depth: {}", Config::global_get_max_depth());
        return Ok(false);
    }

    if let Some(val) = matches.get_one::<String>("setL") {
        if val.is_empty() {
            let state = if Config::global_get_show_line_numbers() { "ON" } else { "OFF" };
            println!("Line numbers: {}", state);
        } else {
            match Config::parse_line_numbers_state(val) {
                Some(state) => {
                    Config::global_set_show_line_numbers(state);
                    print_success(&format!("Line numbers: {}", if state { "ON" } else { "OFF" }));
                }
                None => bail!("Invalid value for setL: {}. Use ON or OFF.", val),
            }
        }
        return Ok(false);
    } else if matches.contains_id("setL") {
        let state = if Config::global_get_show_line_numbers() { "ON" } else { "OFF" };
        println!("Line numbers: {}", state);
        return Ok(false);
    }

    if let Some(val) = matches.get_one::<String>("setT") {
        if val.is_empty() {
            println!("Current threads: {}", Config::global_get_num_threads());
        } else {
            match Config::parse_num_threads(val) {
                Some(threads) => {
                    Config::global_set_num_threads(threads);
                    print_success(&format!("Threads set to: {}", Config::global_get_num_threads()));
                }
                None => bail!("Invalid thread count: {}. Must be a positive integer.", val),
            }
        }
        return Ok(false);
    } else if matches.contains_id("setT") {
        println!("Current threads: {}", Config::global_get_num_threads());
        return Ok(false);
    }

    // --- Handle --setC ---
    if let Some(val) = matches.get_one::<String>("setC") {
        if val.is_empty() {
            let state = if Config::global_get_color_enabled() { "ON" } else { "OFF" };
            println!("Color output: {}", state);
        } else {
            match Config::parse_line_numbers_state(val) {
                Some(state) => {
                    Config::global_set_color_enabled(state);
                    print_success(&format!("Color: {}", if state { "ON" } else { "OFF" }));
                }
                None => bail!("Invalid value for setC: {}. Use ON or OFF.", val),
            }
        }
        return Ok(false);
    } else if matches.contains_id("setC") {
        let state = if Config::global_get_color_enabled() { "ON" } else { "OFF" };
        println!("Color output: {}", state);
        return Ok(false);
    }

    // --- Handle -say / -print ---
    if let Some(text) = matches.get_one::<String>("say") {
        println!("{}", text.green());
        return Ok(false);
    }

    // --- Handle -i (input) ---
    if let Some(input_path) = matches.get_one::<String>("input") {
        let path = Path::new(input_path);
        let copy_to_clipboard = matches.get_flag("copy");

        if path.is_dir() {
            let output_file = matches.get_one::<String>("output");
            let format_str = matches.get_one::<String>("format").map(|s| s.as_str()).unwrap_or("txt");
            
            let format = match format_str {
                "html" => ReportFormat::Html,
                "json" => ReportFormat::Json,
                "md" => ReportFormat::Md,
                _ => ReportFormat::Txt,
            };
            
            if copy_to_clipboard {
                // Generate report to string and copy to clipboard
                let content = crate::report::generate_report_to_string(path, format)?;
                crate::output::copy_to_clipboard(&content, format_str)?;
                print_success(&format!("{} report copied to clipboard!", format_str.to_uppercase()));
            } else if let Some(output) = output_file {
                generate_report_to(path, format, output)?;
            } else {
                generate_report(path, format)?;
            }
        } else if path.is_file() {
            if is_supported_format(path) {
                let show_lines = Config::global_get_show_line_numbers();
                let output_file = matches.get_one::<String>("output");
                if let Some(output) = output_file {
                    let content = crate::output::cat_file_with_line_numbers(path, show_lines)?;
                    let output_path = crate::output::build_output_path(output);
                    crate::output::write_file(&output_path, &content)?;
                    print_success(&format!("File saved to: {}", output_path.display()));
                } else {
                    cat_file(path, show_lines)?;
                }
            } else {
                print_warning(&format!("Skipped (not support format): {}", input_path));
            }
        } else {
            bail!("Path not found: {}", input_path);
        }
        return Ok(false);
    }

    // --- Handle teleport CLI commands ---
    if matches.get_flag("tp_list") {
        crate::teleport::TeleportManager::list()?;
        return Ok(false);
    }

    if let Some(name) = matches.get_one::<String>("tp_add") {
        let current_dir = std::env::current_dir()?;
        crate::teleport::TeleportManager::add(name, current_dir)?;
        return Ok(false);
    }

    if let Some(name) = matches.get_one::<String>("tp_rm") {
        crate::teleport::TeleportManager::remove_by_name(name)?;
        return Ok(false);
    }

    // --- No arguments: Launch interactive mode ---
    Ok(true)
}

/// Print detailed help
fn print_help() {
    println!("ntc {} - Navigate, Tree, Cat", env!("CARGO_PKG_VERSION").green().bold());
    println!("A combined directory tree viewer and file concatenator.\n");
    println!("{}", "USAGE:".cyan().bold());
    println!("    ntc [OPTIONS]");
    println!("    ntc -i <path> [-o <output>]");
    println!("    ntc -i <path> --cp              Copy report to clipboard\n");
    println!("{}", "OPTIONS:".cyan().bold());
    println!("    -i, --input <path>      Process a file or directory");
    println!("    -o, --output <file>     Save output to specified file");
    println!("    -f, --format <FORMAT>   Output format: txt, html, json, md (default: txt)");
    println!("    --cp                    Copy report to clipboard instead of saving to file");
    println!("    --setO [path]           Show or set the output directory (default: Desktop)");
    println!("    --setD [depth]          Show or set max recursion depth (min: 1, max: 20)");
    println!("    --setL [ON|OFF]         Show or toggle line numbers for file display");
    println!("    --setC [ON|OFF]         Show or toggle color output");
    println!("    --setT [threads]        Show or set number of threads (default: 4)");
    println!("    --setH [path|ON|OFF]    Show/set history path or enable/disable");
    println!("    --showcg                Show current configuration overview");
    println!("    --watch [ON|OFF]        Show/set file watcher state");
    println!("    --where                 Show ntc executable and config location");
    println!("    -say, -print <text>     Print text to stdout");
    println!("    --size                  Show current directory size");
    println!("    --view                  Quick view of current directory tree");
    println!("    --view --size           Quick view with directory sizes");
    println!("    --clear                 Clear the terminal screen");
    println!("    --version               Show version information");
    println!("    --list, --fun           List all command-line functions");
    println!("    --help                  Show this help message\n");
    println!("{}", "IGNORE/CARE OPTIONS:".cyan().bold());
    println!("    --ignored               Show currently ignored items");
    println!("    --ignore <name>         Ignore a directory name");
    println!("    --cared <name>          Stop ignoring a directory name");
    println!("    --ignoref <ext>         Ignore a file extension");
    println!("    --caref <ext>           Care about a file extension");
    println!("    --ignoren <file>        Ignore a specific file (e.g., Cargo.lock)");
    println!("    --caren <file>          Care about a specific file\n");
    println!("{}", "TELEPORT SAVE POINTS:".cyan().bold());
    println!("    --tp-add <name>         Save current directory as teleport point");
    println!("    --tp-list               List all teleport points");
    println!("    --tp-rm <name>          Remove a teleport point");
    println!("    Note: Teleport navigation (jump/to) only works in interactive shell\n");
    println!("{}", "EXAMPLES:".cyan().bold());
    println!("    ntc                         Launch interactive mode");
    println!("    ntc @web                    Launch and teleport to 'web' savepoint");
    println!("    ntc -i src                  Generate report of src directory");
    println!("    ntc -i src -o report.html   Generate HTML report");
    println!("    ntc -i src --cp             Copy directory tree to clipboard");
    println!("    ntc -i src -f json --cp     Copy JSON report to clipboard");
    println!("    ntc -i file.txt             Display file contents");
    println!("    ntc --setL ON               Enable line numbers");
    println!("    ntc --showcg                Show configuration");
    println!("    ntc -say \"Hello World\"      Print Hello World");
    println!("    ntc --watch ON              Enable file watcher\n");
    println!("For interactive commands, launch ntc without arguments.");
    println!("{}", "INTERACTIVE-ONLY COMMANDS:".yellow().bold());
    println!("    Navigation: go, godrive, back, gos, gosc");
    println!("    Teleport: tp, tp jump, tp to, @name");
    println!("    Reports: txt, txt --cp, json --cp, md --cp");
    println!("    Configuration: showcg, opencg, resetcg, restorecg");
    println!("    Run Aliases: ral add, ral edit, ral list, ral rm, ral cls");
    println!("    These commands only work inside the interactive shell (run 'ntc' alone).\n");
}