//! `showenv` — show venv, PATH, ntc and OS environment details.

pub(crate) mod help;

pub(crate) mod completion;

use crate::config::Config;
use crate::navigator::Navigator;
use crate::t;
use crate::utils::paths;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_show_env(args: &str, _nav: &mut Navigator) -> Result<bool> {
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
                    ""
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
                        ""
                    } else {
                        "  "
                    };
                println!("  {}. {} {}", (i + 1).to_string().yellow(), marker, entry);
            }
        }
    }
    println!();

    // NTC-managed environment variables (local → global → session)
    println!("{}", t!("misc.show_env.ntc_header").green().bold());
    let (global_env, local_env) = Config::get_env_by_source();
    let session_env = paths::session_env();
    let mut any_ntc = false;
    for (label_key, map) in [
        ("misc.show_env.ntc_local", &local_env),
        ("misc.show_env.ntc_global", &global_env),
        ("misc.show_env.ntc_session", &session_env),
    ] {
        if map.is_empty() {
            continue;
        }
        any_ntc = true;
        println!("  {}", t!(label_key).yellow().bold());
        let mut names: Vec<&String> = map.keys().collect();
        names.sort();
        for name in names {
            println!(
                "    {} = {}",
                name.cyan(),
                map.get(name)
                    .map(String::as_str)
                    .unwrap_or_default()
                    .dimmed()
            );
        }
    }
    if !any_ntc {
        println!("  {}", t!("misc.show_env.ntc_empty").dimmed());
    }
    println!();

    // OS / process environment
    println!("{}", t!("misc.show_env.os_header").blue().bold());
    let mut os_vars: Vec<(String, String)> = std::env::vars().collect();
    os_vars.sort_by(|a, b| a.0.cmp(&b.0));
    let os_limit = if show_all {
        os_vars.len()
    } else {
        depth.unwrap_or(os_vars.len().min(20))
    };
    for (name, value) in os_vars.iter().take(os_limit) {
        let upper = name.to_uppercase();
        let sensitive = ["KEY", "TOKEN", "SECRET", "PASSWORD", "PASSWD"]
            .iter()
            .any(|needle| upper.contains(needle));
        let shown = if sensitive && !show_all {
            t!("misc.show_env.os_masked")
        } else {
            value.clone()
        };
        println!("  {} = {}", name.cyan(), shown.dimmed());
    }
    if os_vars.len() > os_limit {
        println!(
            "  {}",
            t!(
                "misc.show_env.os_more",
                (os_vars.len() - os_limit).to_string()
            )
            .dimmed()
        );
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
            "",
            t!("misc.show_env.active"),
            "ACTIVE".green().bold()
        );
        println!("  {}", t!("misc.show_env.deactivate_hint"));
    } else {
        println!(
            "  {} {} {}",
            "",
            t!("misc.show_env.not_active"),
            "NOT ACTIVE".red().bold()
        );
        println!("  {}", t!("misc.show_env.activate_hint"));
    }
    println!();

    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "showenv",
    aliases: &[],
    help_topic: Some("showenv"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_show_env),
};
