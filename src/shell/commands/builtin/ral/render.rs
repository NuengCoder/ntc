//! Rendering for `ral` usage examples and the `ral ls` sections.

use crate::config::{Config, ConfigScope};
use crate::output::print_info;
use crate::shell::alias::extract_param_names_ral;
use crate::t;
use crate::utils::theme::style::{ShellRole, ThemeColorize};
use std::collections::HashMap;
use std::path::Path;

/// Usage examples for `ral add` / `ral edit`, shown when either is
/// invoked bare. Covers `#param --hint-*` / `#param --select-*` annotations
/// and the `$x[sep]` array separator form.
pub(super) fn print_ral_add_examples() {
    println!("  {}", t!("alias.ral.example_rest").green());
    println!("  {}", t!("alias.ral.example_hint_ext").green());
    println!("  {}", t!("alias.ral.example_hint_array").green());
    println!("  {}", t!("alias.ral.example_select_dir").green());
    println!("  {}", t!("alias.ral.example_hint_values").green());
    println!("  {}", t!("alias.ral.example_hintn").green());
    println!("  {}", t!("alias.ral.example_hint_sep").dimmed());
}

/// Human label of a config scope (used in confirmations and notes).
pub(super) fn scope_label(scope: ConfigScope) -> String {
    if scope.is_local() {
        t!("alias.ral.scope_local").to_string()
    } else {
        t!("alias.ral.scope_global").to_string()
    }
}

/// Note that a local alias of the same name takes priority over a global one.
pub(super) fn print_scope_note(name: &str, scope: ConfigScope) {
    if scope.is_global() && Config::get_local_run_aliases().contains_key(&name.to_lowercase()) {
        print_info(&t!("alias.ral.local_priority_note", name));
    }
}

/// Render one `[global]` / `[local]` section of the `ral ls` listing.
///
/// `shadowed_by` holds the entries of the other store: a name listed in both
/// sections is marked so the shadowing is visible instead of silent.
pub(super) fn render_alias_section(
    title: &str,
    aliases: &HashMap<String, String>,
    shadowed_by: &HashMap<String, String>,
    hint_specs: &HashMap<String, String>,
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
    if aliases.is_empty() {
        out.push_str(&format!("  {}\n", empty_note.dimmed()));
        return out;
    }
    let mut sorted: Vec<(&String, &String)> = aliases.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));
    let alias_color = crate::utils::theme::ThemeManager::current()
        .shell
        .role(ShellRole::RalLsAlias);
    for (i, (name, cmd)) in sorted.iter().enumerate() {
        let is_valid = crate::config::validate_alias_name(name);
        let name_display = if is_valid {
            alias_color.paint_ansi(name)
        } else {
            t!("alias.ral.invalid_suffix", name).red().to_string()
        };
        let param_names = extract_param_names_ral(cmd);
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
        let shadow = if shadowed_by.contains_key(*name) {
            format!("  {}", t!("alias.ral.shadowed").dimmed())
        } else {
            String::new()
        };
        out.push_str(&format!(
            "  {}. {} -> {}{}\n",
            (i + 1).to_string().yellow(),
            name_display,
            cmd_display,
            shadow
        ));
        if let Some(spec) = hint_specs.get(name.as_str()) {
            out.push_str(&format!("     ## {}\n", spec.dimmed()));
        }
    }
    out
}
