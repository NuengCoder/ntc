use super::execute_system_command;
use crate::config::Config;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success};
use crate::shell::alias::{
    expand_command_line_interactive, extract_param_names, find_invalid_array_sep,
    inject_template_defaults, named_refs_in_groups, parse_call_syntax, parse_param_defaults,
    parse_value_list_raw, rename_named_in_spec, serialize_hint_spec, spec_references_named,
    split_hint_annotations, ParamHint,
};

/// Reject `--hint-n` / `--select-n` references to hint lists that do not
/// exist. Returns the missing list name, if any.
fn missing_named_list(hints: &[ParamHint]) -> Option<String> {
    let store = Config::get_run_hint_lists_merged();
    named_refs_in_groups(hints)
        .into_iter()
        .find(|n| !store.contains_key(n))
}

/// Print the unknown-list error. Returns true when every referenced
/// hint list exists.
fn check_named_refs(hints: &[ParamHint]) -> bool {
    if let Some(name) = missing_named_list(hints) {
        print_error(&t!("alias.ral.unknown_hint_list", name));
        println!("{}", t!("alias.ral.hintn_create_hint", name));
        false
    } else {
        true
    }
}

/// Alias names (sorted) whose hint specs reference hint list `name`.
/// Scans both the global and the local spec stores.
fn hint_list_referenced_by(name: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for (alias, spec) in Config::global_get_run_alias_hints() {
        if spec_references_named(&spec, name) && !out.contains(&alias) {
            out.push(alias);
        }
    }
    if let Some(local) = Config::get_local_config() {
        if let Some(map) = local.run_alias_hints {
            for (alias, spec) in map {
                if spec_references_named(&spec, name) && !out.contains(&alias) {
                    out.push(alias);
                }
            }
        }
    }
    out.sort();
    out
}

/// Rewrite `--hint-n` / `--select-n` references `old` → `new` in every
/// stored alias spec (global + local).
fn rewrite_hint_list_refs(old: &str, new: &str) {
    for (alias, spec) in Config::global_get_run_alias_hints() {
        if spec_references_named(&spec, old) {
            let updated = rename_named_in_spec(&spec, old, new);
            let _ = Config::global_update_run_alias_hint(&alias, &updated);
        }
    }
    if let Some(local) = Config::get_local_config() {
        if let Some(map) = local.run_alias_hints {
            for (alias, spec) in map {
                if spec_references_named(&spec, old) {
                    let updated = rename_named_in_spec(&spec, old, new);
                    let _ = Config::local_add_run_alias_hint(&alias, &updated);
                }
            }
        }
    }
}

/// `ral hintn add|rm|rnm|info|ls` — named value lists backing
/// `#param --hint-n <name>` / `--select-n <name>`.
fn cmd_ral_hintn(args: &str) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        print_error(&t!("alias.ral.usage_hintn"));
        return Ok(false);
    }
    let (action, rest) = match args.split_once(' ') {
        Some((a, b)) => (a.to_lowercase(), b.trim()),
        None => (args.to_lowercase(), ""),
    };
    match action.as_str() {
        "add" => {
            let trimmed = rest.trim_start();
            let name_end = trimmed
                .find(|c: char| c == ' ' || c == '\t')
                .unwrap_or(trimmed.len());
            let (name, values_raw) =
                (trimmed[..name_end].trim(), trimmed[name_end..].trim());
            if name.is_empty() || values_raw.is_empty() {
                print_error(&t!("alias.ral.usage_hintn_add"));
                return Ok(false);
            }
            if !crate::config::validate_alias_name(name) {
                print_error(&t!("alias.ral.invalid_name", name));
                return Ok(false);
            }
            let values = match parse_value_list_raw(values_raw) {
                Ok(v) => v,
                Err(()) => {
                    print_error(&t!("alias.ral.usage_hintn_add"));
                    return Ok(false);
                }
            };
            let _ = Config::local_add_run_hint_list(name, &values);
            Config::reload_global();
            ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
            print_success(&t!(
                "alias.ral.hintn_added",
                name.to_lowercase(),
                values.len(),
                values.join(", ")
            ));
        }
        "rm" | "remove" => {
            if rest.is_empty() {
                print_error(&t!("alias.ral.usage_hintn_rm"));
                return Ok(false);
            }
            let force = rest
                .split_whitespace()
                .any(|w| w == "--force" || w == "-f");
            let names: Vec<String> = rest
                .split([',', ' ', '\t'])
                .map(|s| s.trim().trim_matches(',').to_lowercase())
                .filter(|s| !s.is_empty() && s != "--force" && s != "-f")
                .collect();
            if names.is_empty() {
                print_error(&t!("alias.ral.usage_hintn_rm"));
                return Ok(false);
            }
            for name in names {
                let store = Config::get_run_hint_lists_merged();
                if !store.contains_key(&name) {
                    print_error(&t!("alias.ral.hintn_not_found", name));
                    continue;
                }
                let refs = hint_list_referenced_by(&name);
                if !refs.is_empty() && !force {
                    print_error(&t!("alias.ral.hintn_referenced", name, refs.join(", ")));
                    continue;
                }
                let _ = Config::local_remove_run_hint_list(&name);
                print_success(&t!("alias.ral.hintn_removed", name));
            }
            Config::reload_global();
            ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
        }
        "rnm" | "rename" => {
            let parts: Vec<&str> = rest.splitn(3, ' ').collect();
            if parts.len() < 3 || parts[1].to_lowercase() != "to" {
                print_error(&t!("alias.ral.usage_hintn_rnm"));
                return Ok(false);
            }
            let old = parts[0].trim().to_lowercase();
            let new = parts[2].trim();
            if old.is_empty() || new.is_empty() {
                print_error(&t!("alias.ral.usage_hintn_rnm"));
                return Ok(false);
            }
            if !crate::config::validate_alias_name(new) {
                print_error(&t!("alias.ral.invalid_name", new));
                return Ok(false);
            }
            let store = Config::get_run_hint_lists_merged();
            if !store.contains_key(&old) {
                print_error(&t!("alias.ral.hintn_not_found", old));
                return Ok(false);
            }
            rewrite_hint_list_refs(&old, new);
            Config::local_rename_run_hint_list(&old, new);
            Config::reload_global();
            ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
            print_success(&t!("alias.ral.hintn_renamed", old, new.to_lowercase()));
        }
        "info" => {
            let name = rest
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_lowercase();
            if name.is_empty() {
                print_error(&t!("alias.ral.usage_hintn_info"));
                return Ok(false);
            }
            let (store, is_local) = Config::get_run_hint_lists_with_source();
            if let Some(values) = store.get(&name) {
                println!();
                println!(
                    "{}",
                    "==================================================".cyan()
                );
                println!("{}", t!("alias.ral.hintn_info_title").cyan().bold());
                println!(
                    "{}",
                    "==================================================".cyan()
                );
                println!(
                    "  {}: {}",
                    t!("alias.ral.hintn_info_name").yellow(),
                    name.blue().bold()
                );
                println!(
                    "  {}: {}",
                    t!("alias.ral.hintn_info_values").yellow(),
                    values.join(", ").dimmed()
                );
                let refs = hint_list_referenced_by(&name);
                if refs.is_empty() {
                    println!(
                        "  {}: {}",
                        t!("alias.ral.hintn_info_used_by").yellow(),
                        t!("alias.ral.info_none").dimmed()
                    );
                } else {
                    println!(
                        "  {}: {}",
                        t!("alias.ral.hintn_info_used_by").yellow(),
                        refs.join(", ").cyan()
                    );
                }
                let source = if is_local {
                    t!("alias.ral.info_source_local").cyan().to_string()
                } else {
                    t!("alias.ral.info_source_global").green().to_string()
                };
                println!("  {}: {}", t!("alias.ral.info_source").yellow(), source);
                println!();
            } else {
                print_error(&t!("alias.ral.hintn_not_found", name));
            }
        }
        "ls" | "list" => {
            let (store, is_local) = Config::get_run_hint_lists_with_source();
            if store.is_empty() {
                print_info(&t!("alias.ral.hintn_empty"));
            } else {
                println!();
                println!(
                    "{}",
                    "==================================================".cyan()
                );
                if is_local {
                    println!("{}", t!("alias.ral.hintn_list_title_local").cyan().bold());
                } else {
                    println!(
                        "{}",
                        t!("alias.ral.hintn_list_title_global").cyan().bold()
                    );
                }
                println!(
                    "{}",
                    "==================================================".cyan()
                );
                let mut sorted: Vec<_> = store.iter().collect();
                sorted.sort_by(|a, b| a.0.cmp(b.0));
                for (i, (name, values)) in sorted.iter().enumerate() {
                    println!(
                        "  {}. {} ({}) -> {}",
                        (i + 1).to_string().yellow(),
                        name.blue(),
                        t!("alias.ral.hintn_value_count", values.len()).dimmed(),
                        values.join(", ").dimmed()
                    );
                }
                println!();
            }
        }
        _ => {
            print_error(&t!("alias.ral.usage_hintn"));
        }
    }
    Ok(false)
}

/// Reject templates with multi-char array brackets (`$x[ab]`).
/// Returns true when clean.
fn check_array_separators(command: &str) -> bool {
    match find_invalid_array_sep(command) {
        Some((display, content)) => {
            let len = content.chars().count();
            print_error(&t!("alias.ral.arr_sep_error", display, len, content));
            false
        }
        None => true,
    }
}
use crate::shell::helpers::{ral_export_all, ral_export_select, ral_import};
use crate::shell::hinter::ALIAS_GENERATION;
use crate::t;
use std::sync::atomic::Ordering;

use anyhow::Result;
use colored::*;
use std::io::{self, Write};

/// Usage examples for `ral add` / `ral edit`, shown when either is
/// invoked bare. Covers `#param --hint-*` / `#param --select-*` annotations
/// and the `$x[sep]` array separator form.
fn print_ral_add_examples() {
    println!(
        "  {}",
        t!("alias.ral.example_hint_ext").green()
    );
    println!(
        "  {}",
        t!("alias.ral.example_hint_array").green()
    );
    println!(
        "  {}",
        t!("alias.ral.example_select_dir").green()
    );
    println!(
        "  {}",
        t!("alias.ral.example_hint_values").green()
    );
    println!(
        "  {}",
        t!("alias.ral.example_hintn").green()
    );
    println!(
        "  {}",
        t!("alias.ral.example_hint_sep").dimmed()
    );
}

pub fn cmd_ral(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("{}", t!("alias.ral.title").cyan().bold());
        println!("{}", t!("alias.ral.add_cmd"));
        println!("{}", t!("alias.ral.param_cmd"));
        println!("{}", t!("alias.ral.default_cmd"));
        println!("{}", t!("alias.ral.array_cmd"));
        println!("{}", t!("alias.ral.hint_cmd"));
        println!("{}", t!("alias.ral.select_cmd"));
        println!("{}", t!("alias.ral.hintn_cmd"));
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
        println!("  ral add tld \"tl dir ${{x:-.}}\" #x --hint-dir");
        println!("  ral add tldsel \"tl dir ${{x:-.}}\" #x --select-dir");
        println!("  ral add show \"txtf $x\" #x --select-ext rs,txt");
        println!("  ral add srv \"ssh $p\" #p --hint [8080,5050,5000]");
        println!("  ral hintn add port [8080,5050,5000,55555,56789,38533]");
        println!("  ral add srvn \"ssh $p\" #p --hint-n port");
        println!("  ral add cop \"cp ${{x}} ${{y}}\" #x --hint-all #y --hint-tp --hint-dir");
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
                    print_ral_add_examples();
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
                        // Trailing `#param --hint-*` annotations are stored
                        // separately (never in the executed command).
                        let (clean_command, hints) = match split_hint_annotations(&command) {
                            Ok((c, h)) => (c, h),
                            Err(code) => {
                                print_error(&t!("alias.ral.hint_error", code));
                                println!("{}", t!("alias.ral.hint_usage"));
                                return Ok(false);
                            }
                        };
                        command = clean_command;
                        if !check_array_separators(&command) {
                            return Ok(false);
                        }
                        if !check_named_refs(&hints) {
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
                        let is_param = param_hint.is_some() || !hints.is_empty();
                        let _ = Config::local_add_run_alias(name, &command);
                        if hints.is_empty() {
                            let _ = Config::local_remove_run_alias_hint(name);
                        } else {
                            let spec = serialize_hint_spec(&hints);
                            let _ = Config::local_add_run_alias_hint(name, &spec);
                            println!("  {}", t!("alias.ral.hints_saved", spec.cyan()));
                        }
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
                    print_ral_add_examples();
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
                        // Annotations present → replace hints; absent →
                        // keep existing hints untouched.
                        let (clean_command, hints) = match split_hint_annotations(&command) {
                            Ok((c, h)) => (c, h),
                            Err(code) => {
                                print_error(&t!("alias.ral.hint_error", code));
                                println!("{}", t!("alias.ral.hint_usage"));
                                return Ok(false);
                            }
                        };
                        command = clean_command;
                        if !check_array_separators(&command) {
                            return Ok(false);
                        }
                        if !check_named_refs(&hints) {
                            return Ok(false);
                        }
                        let _ = Config::local_update_run_alias(name, &command);
                        if !hints.is_empty() {
                            let spec = serialize_hint_spec(&hints);
                            let _ = Config::local_add_run_alias_hint(name, &spec);
                            println!("  {}", t!("alias.ral.hints_saved", spec.cyan()));
                        }
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
                        let _ = Config::local_remove_run_alias_hint(name);
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
                    let hint_specs = Config::global_get_run_alias_hints();
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
                        if let Some(spec) = hint_specs.get(name.as_str()) {
                            println!("     ## {}", spec.dimmed());
                        }
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
                        let hints = Config::global_get_run_alias_hints();
                        if let Some(spec) = hints.get(&name_lower) {
                            println!(
                                "  {}: {}",
                                t!("alias.ral.info_hints").yellow(),
                                spec.cyan()
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
                            let mut snippet = format!("ral edit {} \"{}\"", name, escaped);
                            // Include the stored hint/select spec so the
                            // pasted command round-trips (`ral edit` replaces
                            // hints only when annotations are present).
                            let edit_hints = Config::global_get_run_alias_hints();
                            if let Some(spec) = edit_hints.get(&name_lower) {
                                snippet.push(' ');
                                snippet.push_str(spec);
                            }
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
                            Config::local_rename_run_alias_hint(old_name, new_name);
                            Config::reload_global();
                            ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
                            print_success(&t!("alias.ral.renamed", old_name, new_name));
                        } else {
                            print_error(&t!("alias.ral.not_found", old_name));
                        }
                    }
                }
            }
            "hintn" => {
                cmd_ral_hintn(subargs)?;
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
                    let content = r##"# ntc run aliases template
# Import with: ral import <file>

[run_aliases]
# btr = "cargo build --release"
# py = "python main.py"
# hello = "echo Hello, World!"
# tldsel = "tl dir ${x:-.}"
# show = "txtf $x"

# Per-alias completion/selection specs (`ral add <name> "<cmd>" #x --hint-dir ...).
# Uncomment to ship hints with this file:
# [run_alias_hints]
# tldsel = "#x --select-dir"
# show = "#x --select-ext rs,txt"

# Named value lists for `#x --hint-n <name>` / `--select-n <name>`
# (manage with: ral hintn add <name> [values]).
# Uncomment to ship named lists with this file:
# [run_hint_lists]
# port = ["8080", "5050", "5000"]
"##;
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
        let expanded_parts = match expand_command_line_interactive(args, nav)? {
            None => return Ok(false),
            Some(cmds) => cmds,
        };
        for cmd in expanded_parts {
            execute_system_command(&cmd, nav.current_path())?;
        }
    }
    Ok(false)
}
