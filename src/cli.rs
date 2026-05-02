use crate::config::Config;
use crate::filetype::is_supported_format;
use crate::output::cat_file;
use crate::report::{generate_report, generate_report_to, ReportFormat};
use anyhow::{bail, Result};
use clap::{Arg, ArgAction, Command};
use std::path::Path;

/// Pre‑process raw args to accept `-say`, `-print` as long options.
fn preprocess_args(args: Vec<String>) -> Vec<String> {
    args.into_iter()
        .map(|arg| {
            if arg == "-say" {
                "--say".to_string()
            } else if arg == "-print" {
                "--say".to_string() // same as --say
            } else {
                arg
            }
        })
        .collect()
}

/// Parse command-line arguments and execute the appropriate action.
/// Returns `true` if the program should continue to interactive mode,
/// `false` if it should exit after the command.
pub fn run_cli() -> Result<bool> {
    // Pre-process arguments so -say / -print become --say
    let raw_args: Vec<String> = std::env::args().collect();
    let args = preprocess_args(raw_args);

    // Define all possible known flags for --list / --fun
    let known_flags = vec![
        "-i, --input <path>          Input file or directory",
        "-o, --output <file>         Output filename",
        "--setO [path]               Show or set output directory",
        "--setD [depth]              Show or set max depth (2-10+)",
        "--setL [ON|OFF]             Show or set line numbers",
        "--setT [threads]            Show or set thread count",
        "-say, -print <text>         Print text to stdout",
        "--ignored                   Show ignored items",
        "--ignore <name>             Ignore a directory name",
        "--cared <name>              Stop ignoring a directory",
        "--ignoref <ext>             Ignore a file extension",
        "--caref <ext>               Care about a file extension",
        "--clear                     Clear the terminal screen",
        "--version                   Show version information",
        "--list, --fun               List all command-line functions",
        "--help                      Show help",
        "(no args)                   Launch interactive mode",
    ];

    let matches = Command::new("ntc")
        .author("Trivico")
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
        .arg(Arg::new("ignore").long("ignore").value_name("NAME").help("Ignore a directory name or extension").num_args(1))
        .arg(Arg::new("cared").long("cared").value_name("NAME").help("Stop ignoring a directory/extension").num_args(1))
        .arg(Arg::new("ignoref").long("ignoref").value_name("EXT").help("Ignore a file extension").num_args(1))
        .arg(Arg::new("caref").long("caref").value_name("EXT").help("Care about (and un-ignore) a file extension").num_args(1))
        .try_get_matches_from(args)?;
        

    // --- Handle --version ---
    if matches.get_flag("version") {
        println!("ntc 1.1.0");
        return Ok(false);
    }

    // --- Handle --clear ---
    if matches.get_flag("clear") {
        let _ = std::process::Command::new("cmd").args(&["/c", "cls"]).status();
        return Ok(false);
    }

    // --- Handle --list / --fun ---
    if matches.get_flag("list") {
        println!("ntc - Available command-line functions:\n");
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

    if matches.get_flag("ignored") {
        let dirs = Config::global_get_ignored_dirs();
        let ignored_exts = Config::global_get_ignored_extensions();
        let extra_exts = Config::global_get_extra_supported_extensions();
        println!("Ignored directories: {:?}", dirs);
        println!("Ignored extensions: {:?}", ignored_exts);
        println!("Extra supported extensions: {:?}", extra_exts);
        return Ok(false);
    }
    if let Some(name) = matches.get_one::<String>("ignore") {
        Config::global_add_ignored_dir(name);
        println!("Now ignoring: {}", name);
        return Ok(false);
    }
    if let Some(name) = matches.get_one::<String>("cared") {
        Config::global_remove_ignored_dir(name);
        println!("No longer ignoring: {}", name);
        return Ok(false);
    }
    if let Some(ext) = matches.get_one::<String>("ignoref") {
        Config::global_add_ignored_extension(ext);
        println!("Now ignoring extension: .{}", ext);
        return Ok(false);
    }
    if let Some(ext) = matches.get_one::<String>("caref") {
        Config::global_remove_ignored_extension(ext);
        Config::global_add_extra_supported_extension(ext); // make it supported
        println!("Now caring about extension: .{}", ext);
        return Ok(false);
    }

    // --- Handle setO, setD, setL, setT (may have optional values) ---
    if let Some(val) = matches.get_one::<String>("setO") {
        if val.is_empty() {
            println!("Current output path: {}", Config::global_get_output_path().display());
        } else {
            Config::global_set_output_path(Path::new(val));
            println!("Output path set to: {}", val);
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
                    println!("Max depth set to: {}", Config::global_get_max_depth());
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
                    println!("Line numbers: {}", if state { "ON" } else { "OFF" });
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
                    println!("Threads set to: {}", Config::global_get_num_threads());
                }
                None => bail!("Invalid thread count: {}. Must be a positive integer.", val),
            }
        }
        return Ok(false);
    } else if matches.contains_id("setT") {
        println!("Current threads: {}", Config::global_get_num_threads());
        return Ok(false);
    }

    // --- Handle -say / -print ---
    if let Some(text) = matches.get_one::<String>("say") {
        println!("{}", text);
        return Ok(false);
    }

    // --- Handle -i (input) ---
    if let Some(input_path) = matches.get_one::<String>("input") {
        let path = Path::new(input_path);

        if path.is_dir() {
            let output_file = matches.get_one::<String>("output");
            if let Some(output) = output_file {
                let format = detect_format_from_filename(output);
                generate_report_to(path, format, output)?;
            } else {
                let format = ReportFormat::Txt;
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
                    println!("File saved to: {}", output_path.display());
                } else {
                    cat_file(path, show_lines)?;
                }
            } else {
                println!("Skipped (not support format): {}", input_path);
            }
        } else {
            bail!("Path not found: {}", input_path);
        }
        return Ok(false);
    }

    // --- No arguments: Launch interactive mode ---
    Ok(true)
}

/// Detect report format from output filename extension
fn detect_format_from_filename(filename: &str) -> ReportFormat {
    let path = Path::new(filename);
    match path.extension().and_then(|e| e.to_str()) {
        Some("html" | "htm") => ReportFormat::Html,
        _ => ReportFormat::Txt,
    }
}

/// Print detailed help
fn print_help() {
    println!(r#"ntc 1.1.0 - Navigate, Tree, Cat
A combined directory tree viewer and file concatenator.

USAGE:
    ntc [OPTIONS]
    ntc -i <path> [-o <output>]
    ntc --setO [path]
    ntc --setD [depth]
    ntc --setL [ON|OFF]
    ntc --setT [threads]

OPTIONS:
    -i, --input <path>      Process a file or directory
    -o, --output <file>     Save output to specified file
    --setO [path]           Show or set the output directory (default: Desktop)
    --setD [depth]          Show or set max recursion depth (min: 2, default: 10)
    --setL [ON|OFF]         Show or toggle line numbers for file display
    --setT [threads]        Show or set number of threads (default: 4)
    -say, -print <text>     Print text to stdout
    --clear                 Clear the terminal screen
    --version               Show version information
    --list, --fun           List all command-line functions
    --help                  Show this help message

IGNORE/CARE OPTIONS:
    --ignored               Show currently ignored dirs, extensions, and extra supported
    --ignore <name>         Ignore a directory name (add to ignore list)
    --cared <name>          Stop ignoring a directory name
    --ignoref <ext>         Ignore a file extension
    --caref <ext>           Care about a file extension (un-ignore and add as supported)

EXAMPLES:
    ntc                         Launch interactive mode
    ntc -i src                 Generate report of src directory (default .txt)
    ntc -i src -o report.html  Generate HTML report of src directory
    ntc -i file.txt            Display file.txt contents
    ntc -i file.txt -o out.txt Save file.txt contents to out.txt
    ntc --setL ON              Enable line numbers
    ntc -say "Hello World"     Print Hello World
    ntc --clear                Clear the terminal
    ntc --ignore target        Ignore 'target' directory
    ntc --caref lock           Care about .lock files

For interactive commands, launch ntc without arguments."#);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_format_txt() {
        assert_eq!(detect_format_from_filename("report.txt"), ReportFormat::Txt);
        assert_eq!(detect_format_from_filename("report"), ReportFormat::Txt);
        assert_eq!(detect_format_from_filename("report.md"), ReportFormat::Txt);
    }

    #[test]
    fn test_detect_format_html() {
        assert_eq!(detect_format_from_filename("report.html"), ReportFormat::Html);
        assert_eq!(detect_format_from_filename("report.htm"), ReportFormat::Html);
    }

    #[test]
    fn test_print_help_doesnt_panic() {
        print_help();
    }
}