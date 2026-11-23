//! `where` — show ntc's executable path, its data locations, or resolve a binary through PATH.

pub(crate) mod help;

use crate::config::Config as NtcConfig;
use crate::explorer::human_readable_size;
use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::path::{Path, PathBuf};

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

/// `where` arguments once `-a` / `--all` is separated from the query.
struct WhereArgs {
    all: bool,
    binary: Option<String>,
}

pub(crate) fn cmd_where(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parsed = parse_where_args(args);
    if let Some(binary) = parsed.binary.as_deref() {
        return where_binary(binary, parsed.all);
    }

    // Bare `where` is just the executable path. `--all` keeps the old panel
    // and adds every store ntc owns under its config directory.
    if !parsed.all {
        let exe = std::env::current_exe().unwrap_or_default();
        println!("{}", exe.display());
        return Ok(false);
    }

    print_ntc_locations(nav);
    Ok(false)
}

fn parse_where_args(args: &str) -> WhereArgs {
    let mut all = false;
    let mut words = Vec::new();
    for token in args.split_whitespace() {
        if matches!(token, "-a" | "--all") {
            all = true;
        } else {
            words.push(token);
        }
    }
    let rest = words.join(" ");
    WhereArgs {
        all,
        binary: where_query(&rest).map(str::to_string),
    }
}

fn print_ntc_locations(nav: &Navigator) {
    let exe = std::env::current_exe().unwrap_or_default();
    let home = crate::utils::paths::config_home().join("ntc");
    let config_path = home.join("config.toml");
    let history = NtcConfig::read_global().resolve_history_path();

    let a = |s: &str| crate::utils::terminal::asciify(s);
    println!();
    println!(
        "{}",
        a("╔══════════════════════════════════════════════════════════════════╗").cyan()
    );
    println!(
        "{}",
        a(&format!("{:^66}", t!("cmd.nav.where.title"))).cyan()
    );
    println!(
        "{}",
        a("╚══════════════════════════════════════════════════════════════════╝").cyan()
    );
    println!();

    print_path_row(&t!("cmd.nav.where.executable"), &exe);
    print_path_row(&t!("cmd.nav.where.config_file"), &config_path);
    if config_path.exists() {
        println!("  {}", t!("nav.config_exists").trim_start().green());
        if let Ok(metadata) = std::fs::metadata(&config_path) {
            println!(
                "  {} {}",
                t!("cmd.nav.where.config_size").dimmed(),
                human_readable_size(metadata.len()).dimmed()
            );
        }
    } else {
        println!("  {}", t!("cmd.nav.where.config_missing").yellow());
    }
    print_path_row(
        &t!("cmd.nav.where.current_dir"),
        Path::new(&nav.display_path()),
    );
    print_path_row(&t!("cmd.nav.where.home"), &home);
    print_path_row(
        &t!("cmd.nav.where.editor_config"),
        &home.join("editor.toml"),
    );
    print_path_row(&t!("cmd.nav.where.sessions"), &home.join("session.toml"));
    print_path_row(
        &t!("cmd.nav.where.packages"),
        &crate::shell::commands::builtin::ntpkg::ntpkg::types::package_dir(),
    );
    print_path_row(
        &t!("cmd.nav.where.deadlock"),
        &crate::shell::commands::builtin::deadlock::deadlock::vault::vault_path(),
    );
    print_path_row(&t!("cmd.nav.where.themes"), &home.join("themes"));
    match history {
        Some(path) => print_path_row(&t!("cmd.nav.where.history"), &path),
        None => print_text_row(&t!("cmd.nav.where.history"), &t!("cmd.nav.where.disabled")),
    }
    print_path_row(&t!("cmd.nav.where.nssh"), &home.join("nssh.toml"));
    print_path_row(&t!("cmd.nav.where.skills"), &home.join("skills"));
    print_path_row(&t!("cmd.nav.where.recovery"), &home.join("crash_recovery"));
    print_path_row(&t!("cmd.nav.where.tutorial"), &home.join("tutorial"));
    println!();
}

fn print_path_row(label: &str, path: &Path) {
    println!(
        "  {} {}",
        label.green().bold(),
        path.display().to_string().cyan()
    );
}

fn print_text_row(label: &str, value: &str) {
    println!("  {} {}", label.green().bold(), value.cyan());
}

/// Trim surrounding whitespace/quotes from a `where` argument.
/// `None` means "no binary given" (show ntc locations instead).
fn where_query(args: &str) -> Option<&str> {
    let trimmed = args.trim();
    let unquoted = trimmed
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .or_else(|| {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
        })
        .unwrap_or(trimmed)
        .trim();
    if unquoted.is_empty() {
        None
    } else {
        Some(unquoted)
    }
}

/// Every path on `PATH` that provides `binary`, in PATH order. The first
/// entry is the one the OS shell would run. PATHEXT/extension handling is
/// delegated to the `which` crate; repeated PATH entries are reported once.
fn search_path(binary: &str) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = Vec::new();
    if let Ok(paths) = which::which_all(binary) {
        for path in paths {
            if !found.iter().any(|seen| same_path(seen, &path)) {
                found.push(path);
            }
        }
    }
    found
}

/// PATH comparison: Windows paths are case-insensitive.
fn same_path(a: &Path, b: &Path) -> bool {
    #[cfg(windows)]
    {
        a.to_string_lossy()
            .eq_ignore_ascii_case(&b.to_string_lossy())
    }
    #[cfg(not(windows))]
    {
        a == b
    }
}

fn where_binary(binary: &str, all: bool) -> Result<bool> {
    let matches = search_path(binary);

    if matches.is_empty() {
        print_error(&t!("cmd.nav.where.not_found", binary));
        return Ok(false);
    }

    let shown = if all {
        matches.as_slice()
    } else {
        &matches[..1]
    };
    println!();
    println!("{}", t!("cmd.nav.where.path_title", binary).cyan().bold());
    for (i, path) in shown.iter().enumerate() {
        let marker = if i == 0 {
            "".green().bold()
        } else {
            "•".dimmed()
        };
        println!("  {} {}", marker, path.display().to_string().cyan());
    }
    println!();
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "where",
    aliases: &[],
    help_topic: Some("where"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_where),
};

#[cfg(test)]
mod tests {
    use super::{parse_where_args, search_path, where_query};

    #[test]
    fn where_query_requires_a_binary_name() {
        assert_eq!(where_query(""), None);
        assert_eq!(where_query("   "), None);
        assert_eq!(where_query("\"\""), None);
        assert_eq!(where_query("git"), Some("git"));
        assert_eq!(where_query("  git  "), Some("git"));
        assert_eq!(where_query("\"git\""), Some("git"));
        assert_eq!(where_query("'node'"), Some("node"));
    }

    #[test]
    fn all_flag_is_parsed_anywhere() {
        let parsed = parse_where_args("git");
        assert!(!parsed.all);
        assert_eq!(parsed.binary.as_deref(), Some("git"));

        let parsed = parse_where_args("--all");
        assert!(parsed.all);
        assert_eq!(parsed.binary, None);

        let parsed = parse_where_args("git -a");
        assert!(parsed.all);
        assert_eq!(parsed.binary.as_deref(), Some("git"));

        let parsed = parse_where_args("-a \"git\"");
        assert!(parsed.all);
        assert_eq!(parsed.binary.as_deref(), Some("git"));
    }

    #[test]
    fn search_path_finds_nothing_for_an_unknown_binary() {
        assert!(search_path("ntc-no-such-binary-xyzzy").is_empty());
    }
}
