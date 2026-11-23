//! `deinit` — remove the run file, local config, and/or OpenCode scaffold.

pub(crate) mod help;

pub(crate) mod completion;

use crate::config::Config;
use crate::navigator::Navigator;
use crate::output::{print_error, print_success, print_warning};
use crate::shell::commands::builtin::ran::ranfile::parser;
use crate::shell::commands::builtin::ran::ranfile::types::RanFormat;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::init_scope::parse_deinit_scope;
use super::super::support::opencode::{confirm_delete, prune_opencode};

pub(crate) fn cmd_deinit(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let trimmed = args.trim();
    let lowered: Vec<String> = trimmed
        .split_whitespace()
        .map(|t| t.to_ascii_lowercase())
        .collect();
    if lowered
        .iter()
        .any(|t| matches!(t.as_str(), "all" | "-a" | "--all" | "-f" | "--force"))
    {
        print_error(&t!("cmd.config.deinit.flags_removed"));
        return Ok(false);
    }
    let scope = match parse_deinit_scope(trimmed) {
        Some(scope) => scope,
        None => {
            print_error(&t!("cmd.config.deinit.usage"));
            return Ok(false);
        }
    };

    if scope.ran {
        let cwd = match std::env::current_dir() {
            Ok(d) => d,
            Err(e) => {
                print_error(&t!(
                    "cmd.config.deinit.remove_failed",
                    "current directory",
                    e
                ));
                return Ok(false);
            }
        };
        let mut found = false;
        for fmt in RanFormat::ALL {
            for name in fmt.file_names() {
                let path = cwd.join(name);
                if !path.exists() {
                    continue;
                }
                found = true;
                if confirm_delete(&path, false)? {
                    match std::fs::remove_file(&path) {
                        Ok(_) => print_success(&t!("cmd.config.deinit.removed", path.display())),
                        Err(e) => {
                            print_error(&t!("cmd.config.deinit.remove_failed", path.display(), e))
                        }
                    }
                } else {
                    println!("{}", t!("cmd.config.deinit.skipped").dimmed());
                }
            }
        }
        if !found {
            print_warning(&t!("cmd.config.deinit.not_found", parser::ranfile_names()));
        }
    }

    if scope.local {
        let cwd = match std::env::current_dir() {
            Ok(d) => d,
            Err(e) => {
                print_error(&t!(
                    "cmd.config.deinit.remove_failed",
                    "current directory",
                    e
                ));
                return Ok(false);
            }
        };
        let path = cwd.join("ntconfig.toml");
        if !path.exists() {
            print_warning(&t!("cmd.config.deinit.local_not_found"));
        } else if confirm_delete(&path, false)? {
            match std::fs::remove_file(&path) {
                Ok(_) => {
                    print_success(&t!("cmd.config.deinit.local_removed"));
                    Config::reload_global();
                }
                Err(e) => print_error(&t!("cmd.config.deinit.local_remove_failed", e)),
            }
        } else {
            println!("{}", t!("cmd.config.deinit.skipped").dimmed());
        }
    }

    if scope.opencode {
        cmd_deinit_opencode()?;
    }

    Ok(false)
}

/// Remove the bundled OpenCode scaffold files, then prune the `tools/`,
/// `agents/` and `.opencode/` directories only if they are empty. Files added
/// by the user (and their directories) are left untouched.
fn cmd_deinit_opencode() -> Result<()> {
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            print_error(&t!(
                "cmd.config.deinit.remove_failed",
                "current directory",
                e
            ));
            return Ok(());
        }
    };
    let base = cwd.join(".opencode");
    if !base.exists() {
        print_warning(&t!("cmd.config.deinit.opencode_not_found"));
        return Ok(());
    }

    // No `--force` escape hatch: a removal cannot be rolled back.
    if !confirm_delete(&base, false)? {
        println!("{}", t!("cmd.config.deinit.skipped").dimmed());
        return Ok(());
    }

    prune_opencode(&base);
    Ok(())
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "deinit",
    aliases: &[],
    help_topic: Some("deinit"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_deinit),
};
