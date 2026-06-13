use std::sync::atomic::Ordering;
use crate::config::Config;
use crate::navigator::Navigator;
use crate::output::{print_error, print_success, print_info};
use crate::shell::hinter::ALIAS_GENERATION;
use crate::shell::alias::{
    expand_command_line, extract_param_names, inject_template_defaults,
    parse_call_syntax, parse_param_defaults,
};
use crate::shell::helpers::{ral_export_all, ral_export_select, ral_import};
use super::execute_system_command;

use anyhow::Result;
use colored::*;
use std::io::{self, Write};

pub fn cmd_ral(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("{}", "Run Alias (ral) Commands:".cyan().bold());
        println!("  ral add <name> <command>          Create a new run alias");
        println!("  ral add <name>(x) <command>       Create parameterised alias (use $x or ${{x}} in command)");
        println!("  ral add <name>({{x=a,e=txt}}) <cmd>   Create alias with default values");
        println!("  ral add <name>(x[],y) <cmd>      Create alias with array params (use $x[].ext or ${{x}}[].ext)");
        println!("  ral edit <name> <command>         Update an existing alias");
        println!("  ral rnm <old> to <new>            Rename an alias");
        println!("  ral rm <name>[, <name>, ...]      Remove alias(es)");
        println!("  ral info <name>                   Show alias details");
        println!("  ral list                          Show all aliases");
        println!("  ral cls                           Clear ALL aliases (asks confirmation)");
        println!("  ral export --all <name>           Export all aliases to <name>.ntc.ral");
        println!("  ral export --select <name>        Select aliases to export to <name>.ntc.ral");
        println!("  ral import <file>                 Import aliases from a .ntc.ral file");
        println!();
        println!("{}", "Examples:".green());
        println!("  ral add btr \"cargo build --release\"");
        println!("  ral rnm btr to build");
        println!("  ral add py \"python test.py\"");
        println!("  ral edit py \"python main.py\"");
        println!("  ral add run_file(x) \"python $x.py\"");
        println!("  ral add mkf({{x=a,e=txt}}) \"echo. > $x.$e\"");
        println!("  ral add runc(x[],y) \"cls && gcc -o $y.exe $x[].c && ./$y.exe && rm -rf $y.exe\"");
        println!("  ral list");
        println!("  ral rm py");
        println!();
        println!("{}", "Usage with run:".green());
        println!("  run btr              # Executes: cargo build --release");
        println!("  run py               # Executes: python test.py");
        println!("  run_file(hello)      # Executes: python hello.py");
        println!("  runc [add,minus,mul] math  # Executes: cls && gcc -o math.exe add.c minus.c mul.c && ...");
        println!();
        println!("{}", "Export/Import:".green());
        println!("  ral export --all myaliases       # Creates myaliases.ntc.ral");
        println!("  ral export --select myaliases    # Interactive pick & export");
        println!("  ral import myaliases.ntc.ral     # Import into current config");
    } else {
        let parts: Vec<&str> = args.splitn(2, ' ').collect();
        let subcmd = parts[0].to_lowercase();
        let subargs = parts.get(1).unwrap_or(&"").trim();

        match subcmd.as_str() {
            "add" => {
                if subargs.is_empty() {
                    print_error("Usage: ral add <name> <command>");
                    println!("Example: ral add btr \"cargo build --release\"");
                    println!("Example: ral add py(x) \"python $x.py\"");
                    println!("Example: ral add runc(x[],y) \"cls && gcc -o $y.exe $x[].c && ./$y.exe\"");
                } else {
                    let add_parts: Vec<&str> = subargs.splitn(2, ' ').collect();
                    if add_parts.len() < 2 {
                        print_error("Usage: ral add <name> <command>");
                        println!("Example: ral add btr \"cargo build --release\"");
                        println!("Example: ral add py(x) \"python $x.py\"");
                        println!("Example: ral add ktr(x) \"kotlinc $x.kt && kotlin ${{x}}Kt.class\"");
                        println!("Example: ral add runc(x[],y) \"cls && gcc -o $y.exe $x[].c && ./$y.exe\"");
                    } else {
                        let raw_name = add_parts[0];
                        let mut command = add_parts[1].to_string();
                        if command.starts_with('"') && command.ends_with('"') && command.len() >= 2 {
                            command = command[1..command.len()-1].to_string();
                        }
                        let (base_name, param_hint) = parse_call_syntax(raw_name);
                        let name = base_name;
                        if !crate::config::validate_alias_name(name) {
                            print_error(&format!("Invalid alias name: '{}'", name));
                            println!("Alias names cannot:");
                            println!("  - Start with @ or #");
                            println!("  - Be a reserved command (go, view, txt, etc.)");
                            return Ok(false);
                        }
                        let has_defaults = param_hint.is_some_and(|hint| {
                            let defaults = parse_param_defaults(hint);
                            if !defaults.is_empty() {
                                command = inject_template_defaults(&command, &defaults);
                                true
                            } else {
                                false
                            }
                        });
                        let is_param = param_hint.is_some();
                        let _ = Config::local_add_run_alias(name, &command);
                        Config::reload_global();
                        ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
                        if is_param {
                            if has_defaults {
                                println!("  Now you can run: {}(<arg>)  (defaults: {})", name.green(), "<arg>".cyan());
                            } else {
                                println!("  Now you can run: {}({})", name.green(), "<arg>".cyan());
                            }
                        } else {
                            println!("  Now you can run: {}", name.green());
                        }
                    }
                }
            }
            "edit" => {
                if subargs.is_empty() {
                    print_error("Usage: ral edit <name> <new_command>");
                    println!("Example: ral edit py \"python main.py\"");
                    println!("Example: ral edit py(x) \"python $x.py\"");
                } else {
                    let edit_parts: Vec<&str> = subargs.splitn(2, ' ').collect();
                    if edit_parts.len() < 2 {
                        print_error("Usage: ral edit <name> <new_command>");
                        println!("Example: ral edit py \"python main.py\"");
                        println!("Example: ral edit py(x) \"python $x.py\"");
                    } else {
                        let (base_name, _) = parse_call_syntax(edit_parts[0]);
                        let name = base_name;
                        let mut command = edit_parts[1].to_string();
                        if command.starts_with('"') && command.ends_with('"') && command.len() >= 2 {
                            command = command[1..command.len()-1].to_string();
                        }
                        if !crate::config::validate_alias_name(name) {
                            print_error(&format!("Invalid alias name: '{}'", name));
                            return Ok(false);
                        }
                        let _ = Config::local_update_run_alias(name, &command);
                        Config::reload_global();
                        ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
            "rm" | "remove" => {
                if subargs.is_empty() {
                    print_error("Usage: ral rm <name>[, <name>, ...]");
                    println!("Example: ral rm py, btr, myalias");
                } else {
                    for name in subargs.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
                        let _ = Config::local_remove_run_alias(name);
                    }
                    Config::reload_global();
                    ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
                }
            }
            "cls" | "clear" => {
                let aliases = Config::global_get_run_aliases();
                if aliases.is_empty() {
                    print_info("No run aliases to clear.");
                    return Ok(false);
                }

                let force = subargs == "--force" || subargs == "-f";

                if !force {
                    println!();
                    println!("{}", "⚠️  WARNING: This will delete ALL run aliases!".yellow().bold());
                    println!("{}", format!("You have {} alias(es) defined:", aliases.len()).yellow());

                    let is_local = Config::get_local_config_path().is_some();
                    if is_local {
                        println!("{}", "  (local config only - global aliases are safe)".dimmed());
                    }

                    let mut sorted: Vec<_> = aliases.keys().collect();
                    sorted.sort();
                    for (i, name) in sorted.iter().take(10).enumerate() {
                        println!("  {}. {}", i + 1, name);
                    }
                    if aliases.len() > 10 {
                        println!("  ... and {} more", aliases.len() - 10);
                    }

                    println!();
                    print!("{} ", "Are you sure? Type 'yes' to confirm: ".red());
                    io::stdout().flush()?;

                    let mut confirm = String::new();
                    io::stdin().read_line(&mut confirm)?;
                    let confirm = confirm.trim().to_lowercase();

                    if confirm != "yes" && confirm != "y" {
                        println!("{}", "Clear cancelled.".dimmed());
                        return Ok(false);
                    }
                }

                let _ = Config::local_clear_run_aliases();
                Config::reload_global();
                ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
            }
            "list" | "ls" => {
                let (aliases, is_local) = Config::get_run_aliases_with_source();
                if aliases.is_empty() {
                    print_info("No run aliases defined. Use 'ral add <name> <command>'");
                } else {
                    println!();
                    println!("{}", "==================================================".cyan());
                    if is_local {
                        println!("{}", "📌 Run Aliases (from local ntconfig.toml)".cyan().bold());
                        if let Some(path) = Config::get_local_config_path() {
                            println!("{}", format!("   Config: {}", path.display()).dimmed());
                        }
                    } else {
                        println!("{}", "📌 Run Aliases (global)".cyan().bold());
                    }
                    println!("{}", "==================================================".cyan());
                    let mut sorted: Vec<_> = aliases.iter().collect();
                    sorted.sort_by(|a, b| a.0.cmp(b.0));
                    for (i, (name, cmd)) in sorted.iter().enumerate() {
                        let is_valid = crate::config::validate_alias_name(name);
                        let name_display = if is_valid {
                            name.blue()
                        } else {
                            format!("{} (INVALID)", name).red()
                        };
                        let param_names = extract_param_names(cmd);
                        let cmd_display = if !param_names.is_empty() {
                            let params_str = if param_names.len() == 1 {
                                format!("({})", param_names[0])
                            } else {
                                format!("({})", param_names.join(", "))
                            };
                            format!("{} {} {}", cmd.dimmed(), "(parameterised)".cyan(), params_str.cyan())
                        } else {
                            cmd.dimmed().to_string()
                        };
                        println!("  {}. {} -> {}",
                            (i + 1).to_string().yellow(),
                            name_display,
                            cmd_display);
                    }
                    println!();
                    println!("{}", "Usage: <alias>  or  <alias>(<arg>) for parameterised aliases".green());
                    if is_local {
                        println!("{}", "💡 Tip: These aliases are project-specific (saved in ntconfig.toml)".dimmed());
                    }
                }
            }
            "info" => {
                if subargs.is_empty() {
                    print_error("Usage: ral info <alias>");
                    println!("Example: ral info btr");
                } else {
                    let aliases = Config::global_get_run_aliases();
                    let name_lower = subargs.to_lowercase();
                    if let Some(cmd) = aliases.get(&name_lower) {
                        println!();
                        println!("{}", "==================================================".cyan());
                        println!("{}", "📌 Alias Info".cyan().bold());
                        println!("{}", "==================================================".cyan());
                        println!("  {}: {}", "Name".yellow(), name_lower.blue().bold());
                        println!("  {}: {}", "Command".yellow(), cmd.dimmed());
                        let param_names = extract_param_names(cmd);
                        if !param_names.is_empty() {
                            let params_str = if param_names.len() == 1 {
                                format!("({})", param_names[0])
                            } else {
                                format!("({})", param_names.join(", "))
                            };
                            println!("  {}: {} {}", "Parameters".yellow(), params_str.cyan(), "(parameterised)".cyan());
                        } else {
                            println!("  {}: {}", "Parameters".yellow(), "none".dimmed());
                        }
                        let (_, is_local) = Config::get_run_aliases_with_source();
                        let source = if is_local {
                            "local (ntconfig.toml)".cyan().to_string()
                        } else {
                            "global".green().to_string()
                        };
                        println!("  {}: {}", "Source".yellow(), source);
                        println!();
                    } else {
                        print_error(&format!("Alias '{}' not found", subargs));
                    }
                }
            }
            "rnm" | "rename" => {
                if subargs.is_empty() {
                    print_error("Usage: ral rnm <old_name> to <new_name>");
                    println!("Example: ral rnm btr to build");
                } else {
                    let rnm_parts: Vec<&str> = subargs.splitn(3, ' ').collect();
                    if rnm_parts.len() < 3 || rnm_parts[1].to_lowercase() != "to" {
                        print_error("Usage: ral rnm <old_name> to <new_name>");
                        println!("Example: ral rnm btr to build");
                    } else {
                        let old_name = rnm_parts[0];
                        let new_name = rnm_parts[2];
                        if !crate::config::validate_alias_name(new_name) {
                            print_error(&format!("Invalid alias name: '{}'", new_name));
                            return Ok(false);
                        }
                        let aliases = Config::global_get_run_aliases();
                        if let Some(command) = aliases.get(&old_name.to_lowercase()) {
                            let command = command.clone();
                            let _ = Config::local_remove_run_alias(old_name);
                            let _ = Config::local_add_run_alias(new_name, &command);
                            Config::reload_global();
                            ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
                            print_success(&format!("Renamed alias '{}' to '{}'", old_name, new_name));
                        } else {
                            print_error(&format!("Alias '{}' not found", old_name));
                        }
                    }
                }
            }
            "export" => {
                if subargs.is_empty() {
                    print_error("Usage: ral export --all <name> | ral export --select <name>");
                    println!("Example: ral export --all myaliases");
                    println!("Example: ral export --select myaliases");
                } else {
                    let export_parts: Vec<&str> = subargs.splitn(2, ' ').collect();
                    if export_parts.len() < 2 {
                        print_error("Usage: ral export --all <name> | ral export --select <name>");
                    } else {
                        let flag = export_parts[0].to_lowercase();
                        let export_name = export_parts[1];
                        if export_name.is_empty() {
                            print_error("Export name cannot be empty.");
                        } else if flag == "--all" || flag == "-a" {
                            ral_export_all(export_name)?;
                        } else if flag == "--select" || flag == "-s" {
                            ral_export_select(nav, export_name)?;
                        } else {
                            print_error(&format!("Unknown export flag: {}. Use --all or --select", flag));
                        }
                    }
                }
            }
            "import" => {
                if subargs.is_empty() {
                    print_error("Usage: ral import <file>");
                    println!("Example: ral import myaliases.ntc.ral");
                } else {
                    ral_import(subargs)?;
                }
            }
            _ => {
                print_error(&format!("Unknown ral subcommand: {}", subcmd));
                println!("Type 'ral' for help.");
            }
        }
    }
    Ok(false)
}

pub fn cmd_run(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("Usage: run <command|alias> [args...]");
        println!();
        println!("{}", "Examples:".green());
        println!("  run python --version        # Run real command");
        println!("  run btr                     # Run alias");
        println!("  run py test.py              # Run alias with args");
        println!("  run_file(hello)             # Run parameterised alias");
    } else {
        let expanded_parts = expand_command_line(args);
        for cmd in expanded_parts {
            execute_system_command(&cmd, nav.current_path())?;
        }
    }
    Ok(false)
}
