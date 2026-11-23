//! `tp` — manage and jump to teleport savepoints.

pub(crate) mod completion;
pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::shell::help::print_tp_help_section;
use crate::t;
use crate::teleport::TeleportManager;
use crate::utils::paths;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_tp(args: &str, nav: &mut Navigator) -> Result<bool> {
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
                        let expanded = paths::expand_path_str(add_parts[1]);
                        let path = std::path::Path::new(&expanded);
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
                print_tp_help_section();
            }
            _ => {
                print_error(&t!("misc.tp.unknown_subcmd", subcmd));
                println!("{}", t!("misc.tp.type_help"));
            }
        }
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "tp",
    aliases: &[],
    help_topic: Some("tp"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_tp),
};
