use super::execute_system_command;
use crate::config::Config;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success};
use crate::shell::alias::{
    expand_command_line, extract_param_names, inject_template_defaults, parse_call_syntax,
    parse_param_defaults,
};
use crate::shell::helpers::{ral_export_all, ral_export_select, ral_import};
use crate::shell::hinter::ALIAS_GENERATION;
use crate::t;
use std::sync::atomic::Ordering;

use anyhow::Result;
use colored::*;
use std::io::{self, Write};

pub fn cmd_ral(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("{}", t!("alias.ral.title").cyan().bold());
        println!("{}", t!("alias.ral.add_cmd"));
        println!("{}", t!("alias.ral.param_cmd"));
        println!("{}", t!("alias.ral.default_cmd"));
        println!("{}", t!("alias.ral.array_cmd"));
        println!("{}", t!("alias.ral.edit_cmd"));
        println!("{}", t!("alias.ral.rnm_cmd"));
        println!("{}", t!("alias.ral.rm_cmd"));
        println!("{}", t!("alias.ral.info_cmd"));
        println!("{}", t!("alias.ral.list_cmd"));
        println!("{}", t!("alias.ral.cls_cmd"));
        println!("{}", t!("alias.ral.make_cmd"));
        println!("{}", t!("alias.ral.export_all_cmd"));
        println!("{}", t!("alias.ral.export_select_cmd"));
        println!("{}", t!("alias.ral.import_cmd"));
        println!();
        println!("{}", t!("alias.ral.examples_header").green());
        println!("  ral add btr \"cargo build --release\"");
        println!("  ral rnm btr to build");
        println!("  ral add py \"python test.py\"");
        println!("  ral edit py \"python main.py\"");
        println!("  ral add run_file(x) \"python $x.py\"");
        println!("  ral add mkf({{x=a,e=txt}}) \"echo. > $x.$e\"");
        println!(
            "  ral add runc(x[],y) \"cls && gcc -o $y.exe $x[].c && ./$y.exe && rm -rf $y.exe\""
        );
        println!("  ral list");
        println!("  ral rm py");
        println!();
        println!("{}", t!("alias.ral.usage_with_run").green());
        println!("{}", t!("alias.ral.run_example1"));
        println!("{}", t!("alias.ral.run_example2"));
        println!("{}", t!("alias.ral.run_example3"));
        println!("{}", t!("alias.ral.run_example4"));
        println!();
        println!("{}", t!("alias.ral.export_import_header").green());
        println!("{}", t!("alias.ral.export_example1"));
        println!("{}", t!("alias.ral.export_example2"));
        println!("{}", t!("alias.ral.export_example3"));
    } else {
        let parts: Vec<&str> = args.splitn(2, ' ').collect();
        let subcmd = parts[0].to_lowercase();
        let subargs = parts.get(1).unwrap_or(&"").trim();

        match subcmd.as_str() {
            "add" => {
                if subargs.is_empty() {
                    print_error(&t!("alias.ral.usage_add"));
                } else {
                    let add_parts: Vec<&str> = subargs.splitn(2, ' ').collect();
                    if add_parts.len() < 2 {
                        print_error(&t!("alias.ral.usage_add"));
                    } else {
                        let raw_name = add_parts[0];
                        let mut command = add_parts[1].to_string();
                        if command.starts_with('"') && command.ends_with('"') && command.len() >= 2
                        {
                            command = command[1..command.len() - 1].to_string();
                        }
                        let (base_name, param_hint) = parse_call_syntax(raw_name);
                        let name = base_name;
                        if !crate::config::validate_alias_name(name) {
                            print_error(&t!("alias.ral.invalid_name", name));
                            println!("{}", t!("alias.ral.name_rules"));
                            println!("{}", t!("alias.ral.name_rule_at"));
                            println!("{}", t!("alias.ral.name_rule_reserved"));
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
                                println!(
                                    "  {}",
                                    t!("alias.ral.now_run_defaults", name.green(), "<arg>".cyan())
                                );
                            } else {
                                println!(
                                    "  {}",
                                    t!("alias.ral.now_run_arg", name.green(), "<arg>".cyan())
                                );
                            }
                        } else {
                            println!("  {}", t!("alias.ral.now_run", name.green()));
                        }
                    }
                }
            }
            "edit" => {
                if subargs.is_empty() {
                    print_error(&t!("alias.ral.usage_edit"));
                } else {
                    let edit_parts: Vec<&str> = subargs.splitn(2, ' ').collect();
                    if edit_parts.len() < 2 {
                        print_error(&t!("alias.ral.usage_edit"));
                    } else {
                        let (base_name, _) = parse_call_syntax(edit_parts[0]);
                        let name = base_name;
                        let mut command = edit_parts[1].to_string();
                        if command.starts_with('"') && command.ends_with('"') && command.len() >= 2
                        {
                            command = command[1..command.len() - 1].to_string();
                        }
                        if !crate::config::validate_alias_name(name) {
                            print_error(&t!("alias.ral.invalid_name", name));
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
                    print_error(&t!("alias.ral.usage_rm"));
                } else {
                    for name in subargs
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                    {
                        let _ = Config::local_remove_run_alias(name);
                    }
                    Config::reload_global();
                    ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
                }
            }
            "cls" | "clear" => {
                let aliases = Config::global_get_run_aliases();
                if aliases.is_empty() {
                    print_info(&t!("alias.ral.no_aliases"));
                    return Ok(false);
                }

                let force = subargs == "--force" || subargs == "-f";

                if !force {
                    println!();
                    println!("{}", t!("alias.ral.cls_warning").yellow().bold());
                    println!("{}", t!("alias.ral.alias_count", aliases.len()).yellow());

                    let is_local = Config::get_local_config_path().is_some();
                    if is_local {
                        println!("{}", t!("alias.ral.local_config_only").dimmed());
                    }

                    let mut sorted: Vec<_> = aliases.keys().collect();
                    sorted.sort();
                    for (i, name) in sorted.iter().take(10).enumerate() {
                        println!("  {}. {}", i + 1, name);
                    }
                    if aliases.len() > 10 {
                        println!("  {}", t!("alias.ral.and_more", aliases.len() - 10));
                    }

                    println!();
                    print!("{} ", t!("alias.ral.cls_prompt").red());
                    io::stdout().flush()?;

                    let mut confirm = String::new();
                    io::stdin().read_line(&mut confirm)?;
                    let confirm = confirm.trim().to_lowercase();

                    if confirm != "yes" && confirm != "y" {
                        println!("{}", t!("alias.ral.cls_cancelled").dimmed());
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
                    print_info(&t!("alias.ral.empty"));
                } else {
                    println!();
                    println!(
                        "{}",
                        "==================================================".cyan()
                    );
                    if is_local {
                        println!("{}", t!("alias.ral.list_title_local").cyan().bold());
                        if let Some(path) = Config::get_local_config_path() {
                            println!("{}", t!("alias.ral.config_path", path.display()).dimmed());
                        }
                    } else {
                        println!("{}", t!("alias.ral.list_title_global").cyan().bold());
                    }
                    println!(
                        "{}",
                        "==================================================".cyan()
                    );
                    let mut sorted: Vec<_> = aliases.iter().collect();
                    sorted.sort_by(|a, b| a.0.cmp(b.0));
                    for (i, (name, cmd)) in sorted.iter().enumerate() {
                        let is_valid = crate::config::validate_alias_name(name);
                        let name_display = if is_valid {
                            name.blue()
                        } else {
                            t!("alias.ral.invalid_suffix", name).red()
                        };
                        let param_names = extract_param_names(cmd);
                        let cmd_display = if !param_names.is_empty() {
                            let params_str = if param_names.len() == 1 {
                                format!("({})", param_names[0])
                            } else {
                                format!("({})", param_names.join(", "))
                            };
                            format!(
                                "{} {} {}",
                                cmd.dimmed(),
                                "(parameterised)".cyan(),
                                params_str.cyan()
                            )
                        } else {
                            cmd.dimmed().to_string()
                        };
                        println!(
                            "  {}. {} -> {}",
                            (i + 1).to_string().yellow(),
                            name_display,
                            cmd_display
                        );
                    }
                    println!();
                    println!("{}", t!("alias.ral.usage_hint").green());
                    if is_local {
                        println!("{}", t!("alias.ral.tip_local").dimmed());
                    }
                }
            }
            "info" => {
                if subargs.is_empty() {
                    print_error(&t!("alias.ral.usage_info"));
                } else {
                    let mut edit_mode = false;
                    let mut name_parts: Vec<&str> = Vec::new();
                    for tok in subargs.split_whitespace() {
                        if tok == "--edit" || tok == "-e" {
                            edit_mode = true;
                        } else {
                            name_parts.push(tok);
                        }
                    }
                    if name_parts.is_empty() {
                        print_error(&t!("alias.ral.usage_info"));
                        return Ok(false);
                    }
                    let name = name_parts.join(" ");
                    let aliases = Config::global_get_run_aliases();
                    let name_lower = name.to_lowercase();
                    if let Some(cmd) = aliases.get(&name_lower) {
                        println!();
                        println!(
                            "{}",
                            "==================================================".cyan()
                        );
                        println!("{}", t!("alias.ral.info_title").cyan().bold());
                        println!(
                            "{}",
                            "==================================================".cyan()
                        );
                        println!(
                            "  {}: {}",
                            t!("alias.ral.info_name").yellow(),
                            name_lower.blue().bold()
                        );
                        println!(
                            "  {}: {}",
                            t!("alias.ral.info_command").yellow(),
                            cmd.dimmed()
                        );
                        let param_names = extract_param_names(cmd);
                        if !param_names.is_empty() {
                            let params_str = if param_names.len() == 1 {
                                format!("({})", param_names[0])
                            } else {
                                format!("({})", param_names.join(", "))
                            };
                            println!(
                                "  {}: {} {}",
                                t!("alias.ral.info_params").yellow(),
                                params_str.cyan(),
                                "(parameterised)".cyan()
                            );
                        } else {
                            println!(
                                "  {}: {}",
                                t!("alias.ral.info_params").yellow(),
                                t!("alias.ral.info_none").dimmed()
                            );
                        }
                        let (_, is_local) = Config::get_run_aliases_with_source();
                        let source = if is_local {
                            t!("alias.ral.info_source_local").cyan().to_string()
                        } else {
                            t!("alias.ral.info_source_global").green().to_string()
                        };
                        println!("  {}: {}", t!("alias.ral.info_source").yellow(), source);
                        println!();
                        if edit_mode {
                            let escaped = cmd.replace('"', "\\\"");
                            let snippet = format!("ral edit {} \"{}\"", name, escaped);
                            crate::output::copy_to_clipboard(&snippet, "ral edit")?;
                            print_success(&t!("alias.ral.info_edit_copied"));
                            println!("  {}", snippet.green().bold());
                            println!("  {}", t!("alias.ral.info_edit_hint").dimmed());
                            println!();
                        }
                    } else {
                        print_error(&t!("alias.ral.not_found", name));
                    }
                }
            }
            "rnm" | "rename" => {
                if subargs.is_empty() {
                    print_error(&t!("alias.ral.usage_rnm"));
                } else {
                    let rnm_parts: Vec<&str> = subargs.splitn(3, ' ').collect();
                    if rnm_parts.len() < 3 || rnm_parts[1].to_lowercase() != "to" {
                        print_error(&t!("alias.ral.usage_rnm"));
                    } else {
                        let old_name = rnm_parts[0];
                        let new_name = rnm_parts[2];
                        if !crate::config::validate_alias_name(new_name) {
                            print_error(&t!("alias.ral.invalid_name", new_name));
                            return Ok(false);
                        }
                        let aliases = Config::global_get_run_aliases();
                        if let Some(command) = aliases.get(&old_name.to_lowercase()) {
                            let command = command.clone();
                            let _ = Config::local_remove_run_alias(old_name);
                            let _ = Config::local_add_run_alias(new_name, &command);
                            Config::reload_global();
                            ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
                            print_success(&t!("alias.ral.renamed", old_name, new_name));
                        } else {
                            print_error(&t!("alias.ral.not_found", old_name));
                        }
                    }
                }
            }
            "export" => {
                if subargs.is_empty() {
                    print_error(&t!("alias.ral.usage_export"));
                } else {
                    let export_parts: Vec<&str> = subargs.splitn(2, ' ').collect();
                    if export_parts.len() < 2 {
                        print_error(&t!("alias.ral.usage_export"));
                    } else {
                        let flag = export_parts[0].to_lowercase();
                        let export_name = export_parts[1];
                        if export_name.is_empty() {
                            print_error(&t!("alias.ral.export_name_empty"));
                        } else if flag == "--all" || flag == "-a" {
                            ral_export_all(export_name)?;
                        } else if flag == "--select" || flag == "-s" {
                            ral_export_select(nav, export_name)?;
                        } else {
                            print_error(&t!("alias.ral.unknown_flag", flag));
                        }
                    }
                }
            }
            "import" => {
                if subargs.is_empty() {
                    print_error(&t!("alias.ral.usage_import"));
                } else {
                    ral_import(subargs)?;
                }
            }
            "make" | "mk" => {
                let path = std::env::current_dir().unwrap_or_default().join(format!(
                    "{}.ntc.ral",
                    if !subargs.is_empty() {
                        subargs
                    } else {
                        "example"
                    }
                ));
                if path.exists() {
                    print_error(&t!("alias.ral.file_exists", path.display()));
                } else {
                    let content = r#"# ntc run aliases template
# Import with: ral import <file>

[run_aliases]
# btr = "cargo build --release"
# py = "python main.py"
# hello = "echo Hello, World!"
"#;
                    match std::fs::write(&path, content) {
                        Ok(()) => {
                            print_success(&t!("alias.ral.template_created", path.display()));
                            print_info(&t!("alias.ral.template_info"));
                        }
                        Err(e) => print_error(&t!("alias.ral.create_failed", e)),
                    }
                }
            }
            _ => {
                print_error(&t!("alias.ral.unknown_subcmd", subcmd));
                println!("{}", t!("alias.ral.type_ral_help"));
            }
        }
    }
    Ok(false)
}

pub fn cmd_run(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("{}", t!("alias.run.usage"));
        println!();
        println!("{}", t!("alias.run.examples_header").green());
        println!("{}", t!("alias.run.example_real"));
        println!("{}", t!("alias.run.example_alias"));
        println!("{}", t!("alias.run.example_alias_args"));
        println!("{}", t!("alias.run.example_param"));
    } else {
        let expanded_parts = expand_command_line(args);
        for cmd in expanded_parts {
            execute_system_command(&cmd, nav.current_path())?;
        }
    }
    Ok(false)
}
