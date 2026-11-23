//! `ral` verb handlers: add, edit, rm, cls, list, info, rnm, export, import,
//! make and the `hintn` delegation.

use super::hints::{check_array_separators, check_named_refs, cmd_ral_hintn, warn_bare_env_refs};
use super::render::{print_ral_add_examples, print_scope_note, render_alias_section, scope_label};
use crate::config::{Config, ConfigScope};
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success};
use crate::shell::alias::{
    extract_param_names_ral, inject_template_defaults, parse_call_syntax, parse_param_defaults,
    serialize_hint_spec, split_hint_annotations,
};
use crate::shell::commands::support::config_scope::{split_scope_flag, split_scope_flag_prefix};
use crate::shell::commands::support::remote_catalog;
use crate::shell::helpers::{ral_export_all, ral_export_select, ral_import};
use crate::shell::hinter::ALIAS_GENERATION;
use crate::t;
use crate::utils::interactive::{InteractiveMenu, MenuItem};
use crate::utils::theme::style::ThemeColorize;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::atomic::Ordering;

fn ral_install_remote() -> Result<()> {
    print_info(&t!("alias.ral.install_fetching"));
    let entries = remote_catalog::list("ntc_ral", ".ntc.ral")?;
    if entries.is_empty() {
        print_info(&t!("alias.ral.install_none"));
        return Ok(());
    }

    let items: Vec<MenuItem> = entries
        .iter()
        .map(|entry| MenuItem::with_detail(entry.name.clone(), entry.size_label()))
        .collect();
    let title = t!("alias.ral.install_title");
    let menu = InteractiveMenu::new(&title)
        .items(items)
        .prompt(t!("alias.ral.install_prompt", entries.len()));
    let Some(index) = menu.run()? else {
        return Ok(());
    };

    let entry = &entries[index];
    print_info(&t!("alias.ral.install_downloading", &entry.name));
    let bytes = remote_catalog::download(&entry.url)?;
    let content = String::from_utf8(bytes).context("downloaded ral file is not UTF-8")?;

    let path = remote_catalog::temp_path("ral", "ntc.ral");
    std::fs::write(&path, content).with_context(|| format!("write {}", path.display()))?;
    let imported = Config::import_run_aliases_from_file(&path, ConfigScope::Global)
        .map_err(|e| anyhow::anyhow!("{}", e));
    let _ = std::fs::remove_file(&path);
    imported?;

    Config::reload_global();
    ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
    print_success(&t!("alias.ral.installed", &entry.name));
    Ok(())
}

pub fn cmd_ral(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("{}", t!("alias.ral.title").cyan().bold());
        println!("{}", t!("alias.ral.add_cmd"));
        println!("{}", t!("alias.ral.param_cmd"));
        println!("{}", t!("alias.ral.rest_cmd"));
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
        println!("{}", t!("alias.ral.install_cmd"));
        println!();
        println!("{}", t!("alias.ral.examples_header").green());
        println!("  ral add btr \"cargo build --release\"");
        println!("  ral rnm btr to build");
        println!("  ral add py \"python test.py\"");
        println!("  ral edit py \"python main.py\"");
        println!("  ral add run_file(x) \"python ${{x}}.py\"");
        println!("  ral add mkfile \"echo. > ${{x:-a}}.${{e:-txt}}\"");
        println!("  ral add hi \"echo $*{{x}}\"   # hi \"wow nice\" -> echo wow nice");
        println!(
            "  ral add runc(x[],y) \"cls && gcc -o ${{y}}.exe $x[].c && ./${{y}}.exe && rm -rf ${{y}}.exe\""
        );
        println!("  ral add tld \"tl dir ${{x:-.}}\" #x --hint-dir");
        println!("  ral add tldsel \"tl dir ${{x:-.}}\" #x --select-dir");
        println!("  ral add show \"txtf ${{x}}\" #x --select-ext rs,txt");
        println!("  ral add srv \"ssh ${{p}}\" #p --hint [8080,5050,5000]");
        println!("  ral hintn add port [8080,5050,5000,55555,56789,38533]");
        println!("  ral add srvn \"ssh ${{p}}\" #p --hint-n port");
        println!("  ral add proj \"cd $HOME/proj\"  # bare $NAME = env var at run time");
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
            "install" => {
                if let Err(e) = ral_install_remote() {
                    print_error(&t!("alias.ral.install_failed", e));
                }
            }
            "check" => {
                let cwd = nav.current_path().to_path_buf();
                super::super::super::support::check::run(
                    "ral",
                    subargs,
                    &cwd,
                    ".ntc.ral",
                    super::super::super::support::check::formats::ral,
                );
            }
            "add" => {
                let (mut scope, subargs) = split_scope_flag(subargs);
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
                        // `ral add ll -g "cmd"` — the scope flag may follow
                        // the alias name.
                        let (mid_scope, rest) = split_scope_flag_prefix(&command);
                        if mid_scope == ConfigScope::Global {
                            scope = mid_scope;
                        }
                        command = rest.to_string();
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
                        warn_bare_env_refs(&command);
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
                        let _ = Config::add_run_alias(name, &command, scope);
                        if hints.is_empty() {
                            let _ = Config::remove_run_alias_hint(name, scope);
                        } else {
                            let spec = serialize_hint_spec(&hints);
                            let _ = Config::add_run_alias_hint(name, &spec, scope);
                            println!("  {}", t!("alias.ral.hints_saved", spec.cyan()));
                        }
                        print_scope_note(name, scope);
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
                let (mut scope, subargs) = split_scope_flag(subargs);
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
                        // `ral edit ll -g "cmd"` — the scope flag may follow
                        // the alias name.
                        let (mid_scope, rest) = split_scope_flag_prefix(&command);
                        if mid_scope == ConfigScope::Global {
                            scope = mid_scope;
                        }
                        command = rest.to_string();
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
                        warn_bare_env_refs(&command);
                        if !check_named_refs(&hints) {
                            return Ok(false);
                        }
                        let _ = Config::update_run_alias(name, &command, scope);
                        if !hints.is_empty() {
                            let spec = serialize_hint_spec(&hints);
                            let _ = Config::add_run_alias_hint(name, &spec, scope);
                            println!("  {}", t!("alias.ral.hints_saved", spec.cyan()));
                        }
                        print_scope_note(name, scope);
                        Config::reload_global();
                        ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
            "rm" | "remove" => {
                let (scope, subargs) = split_scope_flag(subargs);
                if subargs.is_empty() {
                    print_error(&t!("alias.ral.usage_rm"));
                } else {
                    for name in subargs
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                    {
                        let store = match Config::remove_run_alias(name, scope) {
                            Ok(store) => store,
                            Err(_) => continue,
                        };
                        match store {
                            crate::config::AliasStore::Local => {
                                if Config::get_global_run_aliases()
                                    .contains_key(&name.to_lowercase())
                                {
                                    print_info(&t!("alias.ral.global_active_again", name));
                                }
                            }
                            crate::config::AliasStore::Global => {}
                            crate::config::AliasStore::NotFound => {
                                if Config::get_global_run_aliases()
                                    .contains_key(&name.to_lowercase())
                                {
                                    print_error(&t!("alias.ral.global_only_hint", name, name));
                                }
                            }
                        }
                    }
                    Config::reload_global();
                    ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
                }
            }
            "cls" | "clear" => {
                let (scope, subargs) = split_scope_flag(subargs);
                let (global_aliases, local_aliases) = Config::get_run_aliases_by_source();
                let aliases = if scope.is_local() {
                    local_aliases
                } else {
                    global_aliases
                };
                if aliases.is_empty() {
                    print_info(&t!("alias.ral.no_aliases"));
                    return Ok(false);
                }

                let force = subargs == "--force" || subargs == "-f";

                if !force {
                    println!();
                    println!("{}", t!("alias.ral.cls_warning").yellow().bold());
                    println!("{}", t!("alias.ral.alias_count", aliases.len()).yellow());
                    println!(
                        "{}",
                        t!("alias.ral.cls_target", scope_label(scope)).dimmed()
                    );

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

                let _ = Config::clear_run_aliases(scope);
                Config::reload_global();
                ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
            }
            "list" | "ls" => {
                let (global_aliases, local_aliases) = Config::get_run_aliases_by_source();
                if global_aliases.is_empty() && local_aliases.is_empty() {
                    print_info(&t!("alias.ral.empty"));
                } else {
                    let hint_specs = Config::get_run_alias_hints_merged();
                    println!();
                    print!(
                        "{}",
                        render_alias_section(
                            &t!("alias.ral.section_global"),
                            &global_aliases,
                            &local_aliases,
                            &hint_specs,
                            None,
                            &t!("alias.ral.none"),
                        )
                    );
                    println!();
                    let local_path = Config::get_local_config_path();
                    let local_note = if local_path.is_some() {
                        t!("alias.ral.none")
                    } else {
                        t!("alias.ral.no_local")
                    };
                    print!(
                        "{}",
                        render_alias_section(
                            &t!("alias.ral.section_local"),
                            &local_aliases,
                            &HashMap::new(),
                            &hint_specs,
                            local_path.as_deref(),
                            &local_note,
                        )
                    );
                    println!();
                    println!("{}", t!("alias.ral.usage_hint").green());
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
                    let (global_aliases, local_aliases) = Config::get_run_aliases_by_source();
                    let name_lower = name.to_lowercase();
                    // Local wins: the local entry is the effective one.
                    let local_cmd = local_aliases.get(&name_lower);
                    let global_cmd = global_aliases.get(&name_lower);
                    let effective = local_cmd.or(global_cmd);
                    if let Some(cmd) = effective {
                        let is_local = local_cmd.is_some();
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
                        let param_names = extract_param_names_ral(cmd);
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
                        let hints = Config::get_run_alias_hints_merged();
                        if let Some(spec) = hints.get(&name_lower) {
                            println!("  {}: {}", t!("alias.ral.info_hints").yellow(), spec.cyan());
                        }
                        let source = if is_local {
                            t!("alias.ral.info_source_local").cyan().to_string()
                        } else {
                            t!("alias.ral.info_source_global").green().to_string()
                        };
                        println!("  {}: {}", t!("alias.ral.info_source").yellow(), source);
                        // The global store may hold the same name — show it as
                        // the shadowed entry instead of hiding it.
                        if is_local {
                            if let Some(other) = global_cmd {
                                println!(
                                    "  {}: {}",
                                    t!("alias.ral.shadowed_global").yellow(),
                                    other.dimmed()
                                );
                            }
                        }
                        println!();
                        if edit_mode {
                            let escaped = cmd.replace('"', "\\\"");
                            let mut snippet = format!("ral edit {} \"{}\"", name, escaped);
                            // Include the stored hint/select spec so the
                            // pasted command round-trips (`ral edit` replaces
                            // hints only when annotations are present).
                            let edit_hints = Config::get_run_alias_hints_merged();
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
                let (scope, subargs) = split_scope_flag(subargs);
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
                        let (global_aliases, local_aliases) = Config::get_run_aliases_by_source();
                        let old_lower = old_name.to_lowercase();
                        // Rename in the store that owns the alias: the local
                        // one wins, the global one is only touched on request.
                        let target = if scope.is_global() {
                            global_aliases.get(&old_lower).cloned()
                        } else if local_aliases.contains_key(&old_lower) {
                            local_aliases.get(&old_lower).cloned()
                        } else {
                            global_aliases.get(&old_lower).cloned()
                        };
                        match target {
                            Some(command) => {
                                let target_scope = if scope.is_global() {
                                    ConfigScope::Global
                                } else if local_aliases.contains_key(&old_lower) {
                                    ConfigScope::Auto
                                } else {
                                    ConfigScope::Global
                                };
                                let _ = Config::remove_run_alias(old_name, target_scope);
                                let _ = Config::add_run_alias(new_name, &command, target_scope);
                                Config::rename_run_alias_hint(old_name, new_name, target_scope);
                                Config::reload_global();
                                ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
                                print_success(&t!("alias.ral.renamed", old_name, new_name));
                            }
                            None => {
                                print_error(&t!("alias.ral.not_found", old_name));
                            }
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
                let (scope, subargs) = split_scope_flag(subargs);
                if subargs.is_empty() {
                    print_error(&t!("alias.ral.usage_import"));
                } else {
                    ral_import(subargs, scope)?;
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
# show = "txtf ${x}"            # ${x} is a parameter
# hi = "echo $*{x}"              # one arg, spaces kept: hi "wow nice" -> echo wow nice
# proj = "cd $HOME/projects"     # bare $NAME is an environment variable

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
