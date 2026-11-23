//! `local` — manage the per-directory `ntconfig.toml`.

pub(crate) mod help;

pub(crate) mod completion;

use crate::config::Config;
use crate::navigator::Navigator;
use crate::output::{print_error, print_success, print_warning};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::io::{self, Write};

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::local_config::LOCAL_CONFIG_TEMPLATE;

pub(crate) fn cmd_local(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.split_whitespace().collect();
    let subcmd = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1..).unwrap_or(&[]).join(" ");

    match subcmd {
        "init" => cmd_local_init(&subargs, nav),
        "deinit" => cmd_local_deinit(nav),
        "help" | "-h" | "--help" => {
            cmd_local_help();
            Ok(false)
        }
        _ => {
            if subcmd.is_empty() {
                cmd_local_help();
            } else {
                print_error(&format!(
                    "Unknown local subcommand: '{}'. Use 'local help' for usage.",
                    subcmd
                ));
            }
            Ok(false)
        }
    }
}

fn cmd_local_init(args: &str, nav: &mut Navigator) -> Result<bool> {
    let current_dir = nav.current_path();
    let ntconfig_path = current_dir.join("ntconfig.toml");

    // `--all` used to copy the global settings into the local file, which
    // replaced the global behaviour with a stale copy. It is gone: a local
    // config can no longer shadow the global one wholesale.
    if args == "--all" || args == "-a" {
        print_error(&t!("cmd.config.local.init_all_removed"));
        return Ok(false);
    }

    if ntconfig_path.exists() {
        println!();
        println!("{}", t!("cmd.config.local.exists_warning").yellow().bold());
        println!("Path: {}", ntconfig_path.display().to_string().cyan());
        println!();
        print!("{} ", t!("cmd.config.local.overwrite_prompt").red());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("{}", t!("cmd.config.local.init_cancelled").dimmed());
            return Ok(false);
        }
    }

    match std::fs::write(&ntconfig_path, LOCAL_CONFIG_TEMPLATE) {
        Ok(_) => {
            print_success(&t!(
                "cmd.config.local.created_template",
                current_dir.display()
            ));
            println!();
            println!("{}", t!("cmd.config.local.template_hint").yellow());
            println!("{}", t!("cmd.config.local.template_edit_hint").dimmed());
            println!("{}", t!("cmd.config.local.precedence_hint").dimmed());
            println!();
            println!("{}", t!("cmd.config.local.tip_global").dimmed());
        }
        Err(e) => print_error(&t!("cmd.config.local.create_failed", e)),
    }

    Ok(false)
}

fn cmd_local_deinit(nav: &mut Navigator) -> Result<bool> {
    let ntconfig_path = nav.current_path().join("ntconfig.toml");

    if !ntconfig_path.exists() {
        print_warning(&t!("cmd.config.local_deinit.not_found"));
        return Ok(false);
    }

    println!();
    println!("{}", t!("cmd.config.local_deinit.title").red().bold());
    println!("{}", crate::utils::terminal::asciify(&"═".repeat(50)).red());
    println!(
        "{}",
        t!(
            "cmd.config.local_deinit.will_delete",
            ntconfig_path.display()
        )
    );
    println!("{}", t!("cmd.config.local_deinit.lose_settings"));
    println!("{}", t!("cmd.config.local_deinit.global_safe"));
    println!();
    print!("{} ", t!("cmd.config.local_deinit.confirm").red());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    if input != "y" && input != "yes" {
        println!("{}", t!("cmd.config.local_deinit.cancelled").dimmed());
        return Ok(false);
    }

    match std::fs::remove_file(&ntconfig_path) {
        Ok(_) => {
            print_success(&t!("cmd.config.local_deinit.done"));
            Config::reload_global();
        }
        Err(e) => print_error(&t!("cmd.config.local_deinit.remove_failed", e)),
    }

    Ok(false)
}

fn cmd_local_help() {
    println!();
    println!(
        "{}",
        "local — Manage local ntconfig.toml configuration"
            .cyan()
            .bold()
    );
    println!();
    println!("  local init                 Create a commented ntconfig.toml template");
    println!("  local deinit               Remove ntconfig.toml from current directory");
    println!("  local help                 Show this help");
    println!();
    println!("The local ntconfig.toml adds project-specific ignore/care and run alias");
    println!("settings for the current directory only. Local entries take priority;");
    println!("global entries keep working for everything the local file does not");
    println!("define, and the global config is never modified.");
    println!();
    println!("Use '-g' / '--global' on 'ral', 'ignore', 'care' and 'igcare import' to");
    println!("write to the global config from inside a project directory.");
    println!();
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "local",
    aliases: &[],
    help_topic: Some("local"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_local),
};

#[cfg(test)]
mod tests {
    use super::*;

    /// `local init` writes the commented template; `local init --all` is
    /// rejected and writes nothing.
    #[test]
    fn local_init_writes_template_and_rejects_all() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[], &[]);
        let mut nav = Navigator::new().unwrap();

        // The removed flag must not create a file.
        cmd_local_init("--all", &mut nav).unwrap();
        assert!(!std::path::Path::new("ntconfig.toml").exists());

        cmd_local_init("", &mut nav).unwrap();
        let content = std::fs::read_to_string("ntconfig.toml").unwrap();
        assert!(content.contains("run_aliases"), "got:\n{}", content);
        // The template stays fully commented out.
        assert!(
            !content
                .lines()
                .any(|l| l.starts_with("ignored_directory_names")),
            "got:\n{}",
            content
        );
        assert!(
            !content
                .lines()
                .any(|l| l.starts_with("cared_directory_names")),
            "got:\n{}",
            content
        );
        // The old "copy the global settings" hint is gone.
        assert!(!content.contains("local init --all"), "got:\n{}", content);
    }
}
