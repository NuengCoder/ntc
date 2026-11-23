//! Named hint-list management (`ral hintn …`) plus template checks shared with
//! `ral add` / `ral edit`.

use crate::config::{Config, ConfigScope};
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::shell::alias::{
    bare_env_refs, find_invalid_array_sep, named_refs_in_groups, parse_value_list_raw,
    rename_named_in_spec, spec_references_named, ParamHint,
};
use crate::shell::commands::support::config_scope::split_scope_flag;
use crate::shell::hinter::ALIAS_GENERATION;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::Ordering;

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
pub(super) fn check_named_refs(hints: &[ParamHint]) -> bool {
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
    for (alias, spec) in Config::get_run_alias_hints_merged() {
        if spec_references_named(&spec, name) && !out.contains(&alias) {
            out.push(alias);
        }
    }
    out.sort();
    out
}

/// Rewrite `--hint-n` / `--select-n` references `old` → `new` in every
/// stored alias spec (global + local).
fn rewrite_hint_list_refs(old: &str, new: &str) {
    let (global_specs, local_specs) = Config::get_run_alias_hints_by_source();
    for (alias, spec) in global_specs {
        if spec_references_named(&spec, old) {
            let updated = rename_named_in_spec(&spec, old, new);
            let _ = Config::global_update_run_alias_hint(&alias, &updated);
        }
    }
    if Config::get_local_config_path().is_some() {
        for (alias, spec) in local_specs {
            if spec_references_named(&spec, old) {
                let updated = rename_named_in_spec(&spec, old, new);
                let _ = Config::add_run_alias_hint(&alias, &updated, ConfigScope::Auto);
            }
        }
    }
}

/// Render one `[global]` / `[local]` section of the `ral hintn ls` listing.
fn render_hint_list_section(
    title: &str,
    lists: &HashMap<String, Vec<String>>,
    path: Option<&Path>,
    empty_note: &str,
) -> String {
    let mut out = String::new();
    out.push_str("==================================================\n");
    out.push_str(&format!("{}\n", title.cyan().bold()));
    if let Some(p) = path {
        out.push_str(&format!(
            "{}\n",
            t!("alias.ral.config_path", p.display()).dimmed()
        ));
    }
    out.push_str("==================================================\n");
    if lists.is_empty() {
        out.push_str(&format!("  {}\n", empty_note.dimmed()));
        return out;
    }
    let mut sorted: Vec<_> = lists.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));
    for (i, (name, values)) in sorted.iter().enumerate() {
        out.push_str(&format!(
            "  {}. {} ({}) -> {}\n",
            (i + 1).to_string().yellow(),
            name.blue(),
            t!("alias.ral.hintn_value_count", values.len()).dimmed(),
            values.join(", ").dimmed()
        ));
    }
    out
}

/// `ral hintn add|rm|rnm|info|ls` — named value lists backing
/// `#param --hint-n <name>` / `--select-n <name>`.
pub(super) fn cmd_ral_hintn(args: &str) -> anyhow::Result<bool> {
    let (outer_scope, args) = split_scope_flag(args);
    let args = args.trim();
    if args.is_empty() {
        print_error(&t!("alias.ral.usage_hintn"));
        return Ok(false);
    }
    let (action, rest) = match args.split_once(' ') {
        Some((a, b)) => (a.to_lowercase(), b.trim()),
        None => (args.to_lowercase(), ""),
    };
    // The flag may also sit between the action and its arguments
    // (`ral hintn add -g name values`).
    let (inner_scope, rest) = split_scope_flag(rest);
    let scope = if outer_scope == ConfigScope::Global {
        outer_scope
    } else {
        inner_scope
    };
    match action.as_str() {
        "add" => {
            let trimmed = rest.trim_start();
            let name_end = trimmed
                .find(|c: char| c == ' ' || c == '\t')
                .unwrap_or(trimmed.len());
            let (name, values_raw) = (trimmed[..name_end].trim(), trimmed[name_end..].trim());
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
            let _ = Config::add_run_hint_list(name, &values, scope);
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
            let force = rest.split_whitespace().any(|w| w == "--force" || w == "-f");
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
                let _ = Config::remove_run_hint_list(&name, scope);
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
            Config::rename_run_hint_list(&old, new, scope);
            Config::reload_global();
            ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
            print_success(&t!("alias.ral.hintn_renamed", old, new.to_lowercase()));
        }
        "info" => {
            let name = rest.split_whitespace().next().unwrap_or("").to_lowercase();
            if name.is_empty() {
                print_error(&t!("alias.ral.usage_hintn_info"));
                return Ok(false);
            }
            let (global_lists, local_lists) = Config::get_run_hint_lists_by_source();
            // Local wins: the local list is the effective one.
            let local_values = local_lists.get(&name);
            let global_values = global_lists.get(&name);
            let store = local_values.or(global_values);
            if let Some(values) = store {
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
                let source = if local_values.is_some() {
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
            let (global_lists, local_lists) = Config::get_run_hint_lists_by_source();
            if global_lists.is_empty() && local_lists.is_empty() {
                print_info(&t!("alias.ral.hintn_empty"));
            } else {
                println!();
                print!(
                    "{}",
                    render_hint_list_section(
                        &t!("alias.ral.section_global"),
                        &global_lists,
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
                    render_hint_list_section(
                        &t!("alias.ral.section_local"),
                        &local_lists,
                        local_path.as_deref(),
                        &local_note,
                    )
                );
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
pub(super) fn check_array_separators(command: &str) -> bool {
    match find_invalid_array_sep(command) {
        Some((display, content)) => {
            let len = content.chars().count();
            print_error(&t!("alias.ral.arr_sep_error", display, len, content));
            false
        }
        None => true,
    }
}

/// Point out bare `$name` refs: in ral they are environment variables, so a
/// template that meant a parameter must use the braced `${name}` form.
pub(super) fn warn_bare_env_refs(command: &str) {
    let refs = bare_env_refs(command);
    if refs.is_empty() {
        return;
    }
    let list = refs
        .iter()
        .map(|n| format!("${}", n))
        .collect::<Vec<_>>()
        .join(", ");
    print_warning(&t!("alias.ral.bare_env_warning", list));
}
