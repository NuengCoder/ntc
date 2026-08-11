use crate::t;
use colored::*;
use std::io::{self, Write};

/// Pre‑process raw args to accept `-say`, `-print`, `-image` as long options,
/// and convert `image <path> to <format>` syntax into structured args.
pub(super) fn preprocess_args(args: Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-say" | "-print" => out.push("--say".to_string()),
            "-image" => {
                out.push("--image".to_string());
                // Check if the next tokens follow: <path> to <format>
                if i + 3 < args.len() && args[i + 2] == "to" {
                    // image <path> to <format>
                    out.push(args[i + 1].clone()); // <path>
                    out.push("--image-to".to_string()); // our internal flag
                    out.push(args[i + 3].clone()); // <format>
                    i += 4;
                    continue;
                } else if i + 1 < args.len() {
                    // image <path> (no to/format, just eat the path)
                    out.push(args[i + 1].clone());
                    i += 2;
                    continue;
                }
            }
            "-o" => out.push("--output".to_string()),
            _ => out.push(arg.clone()),
        }
        i += 1;
    }
    out
}

pub(super) fn print_logo() {
    println!();
    println!(
        "  {}  {}  {}",
        "N".blue().bold(),
        "T".blue().bold(),
        "C".blue().bold()
    );
    println!(
        "  {} {} {}",
        "Navigate".red(),
        "Toolkit".green(),
        "Center".blue()
    );
    println!();
}

/// Print detailed help
pub(super) fn print_help() {
    println!(
        "{}",
        "ntc <version> — Navigate, Toolkit, Center".green().bold()
    );
    println!("{}\n", t!("cli.helpers.tagline"));

    print_basic_usage();
    print_report_flags();
    print_config_flags();
    print_editor_flags();
    print_igcare_flags();
    print_teleport_flags();
    print_alias_export_flags();
    print_misc_flags();
    print_cli_examples();
    print_shell_cli_commands();
    print_interactive_only_note();
}

fn print_basic_usage() {
    println!("{}", t!("cli.helpers.usage_header").cyan().bold());
    println!("{}", t!("cli.helpers.basic_usage"));
}

fn print_report_flags() {
    println!("{}", t!("cli.helpers.report_header").cyan().bold());
    println!("{}", t!("cli.helpers.report_options"));
}

fn print_config_flags() {
    println!("{}", t!("cli.helpers.config_header").cyan().bold());
    println!("{}", t!("cli.helpers.config_options"));
}

fn print_editor_flags() {
    println!("{}", t!("cli.helpers.editor_header").cyan().bold());
    println!("{}", t!("cli.helpers.editor_options"));
}

fn print_igcare_flags() {
    println!("{}", t!("cli.helpers.ignore_header").cyan().bold());
    println!("{}", t!("cli.helpers.igcare_options"));
}

fn print_teleport_flags() {
    println!("{}", t!("cli.helpers.tp_header").cyan().bold());
    println!("{}", t!("cli.helpers.tp_options"));
}

fn print_alias_export_flags() {
    println!("{}", t!("cli.helpers.export_header").cyan().bold());
    println!("{}", t!("cli.helpers.export_options"));
}

fn print_misc_flags() {
    println!("{}", t!("cli.helpers.other_header").cyan().bold());
    println!("{}", t!("cli.helpers.other_options"));
}

fn print_cli_examples() {
    println!("{}", t!("cli.helpers.examples_header").cyan().bold());
    println!("{}", t!("cli.helpers.cli_examples"));
}

fn print_shell_cli_commands() {
    println!("{}", t!("cli.helpers.shell_header").cyan().bold());
    println!("{}", t!("cli.helpers.shell_commands"));
}

fn print_interactive_only_note() {
    println!("{}", t!("cli.helpers.interactive_note").yellow().bold());
}

/// Generate shell completion script for the given shell type.
pub(super) fn generate_completions(shell: &str) {
    let script = match shell.to_lowercase().as_str() {
        "bash" => include_str!("completions/ntc.bash"),
        "zsh" => include_str!("completions/ntc.zsh"),
        "fish" => include_str!("completions/ntc.fish"),
        "powershell" => include_str!("completions/ntc.ps1"),
        _ => {
            eprintln!(
                "Unsupported shell: {}. Use: bash, zsh, fish, powershell",
                shell
            );
            return;
        }
    };
    if let Err(e) = io::stdout().write_all(script.as_bytes()) {
        eprintln!("Failed to write completions: {}", e);
    }
    let _ = io::stdout().flush();
}
