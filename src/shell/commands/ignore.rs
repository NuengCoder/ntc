//! Shell commands for managing ignore/care filtering:
//! `ignore`, `care`, `igcared`, `igcare` (export/import).

use crate::config::{Config, ConfigScope};
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success};
use crate::shell::helpers::{igcare_export_all, igcare_export_select, igcare_import};
use crate::t;

use super::scope::split_scope_flag;
use anyhow::Result;
use colored::*;

fn apply_igcare(
    args: &str,
    usage: &str,
    action: fn(&str, ConfigScope) -> std::result::Result<(), Box<dyn std::error::Error>>,
    action_desc: &str,
    scope: ConfigScope,
) -> Result<bool> {
    if args.is_empty() {
        println!("{}", usage);
    } else {
        let mut count = 0;
        for item in args.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            if action(item, scope).is_ok() {
                count += 1;
            }
        }
        Config::reload_global();
        if count > 0 {
            print_success(&t!("cmd.ignore.applied", action_desc, count));
            print_info(&t!("cmd.ignore.scope_target", scope_label(scope)));
            print_info(&t!("cmd.ignore.check_hint"));
        }
    }
    Ok(false)
}

/// Human label of the config store a write targets.
fn scope_label(scope: ConfigScope) -> String {
    if scope.is_local() {
        t!("cmd.ignore.scope_local").to_string()
    } else {
        t!("cmd.ignore.scope_global").to_string()
    }
}

pub fn cmd_ignore(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let (scope, args) = split_scope_flag(args);
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let subcmd = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1).unwrap_or(&"").trim();

    match subcmd {
        "dir" => apply_igcare(
            subargs,
            &t!("cmd.ignore.dir_usage"),
            Config::add_ignored_dir,
            "Ignored directories:",
            scope,
        ),
        "ext" => apply_igcare(
            subargs,
            &t!("cmd.ignore.ext_usage"),
            Config::add_ignored_extension,
            "Ignored extensions:",
            scope,
        ),
        "file" => apply_igcare(
            subargs,
            &t!("cmd.ignore.file_usage"),
            Config::add_ignored_file,
            "Ignored files:",
            scope,
        ),
        _ => {
            println!("{}", t!("cmd.ignore.title").cyan().bold());
            println!("  {}", t!("cmd.ignore.dir_desc"));
            println!("  {}", t!("cmd.ignore.ext_desc"));
            println!("  {}", t!("cmd.ignore.file_desc"));
            println!();
            println!("{}", t!("cmd.ignore.scope_help").green());
            println!();
            println!("{}", t!("cmd.ignore.examples_title").green());
            println!("  {}", t!("cmd.ignore.example_dir").dimmed());
            println!("  {}", t!("cmd.ignore.example_ext").dimmed());
            println!("  {}", t!("cmd.ignore.example_file").dimmed());
            Ok(false)
        }
    }
}

pub fn cmd_care(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let (scope, args) = split_scope_flag(args);
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let subcmd = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1).unwrap_or(&"").trim();

    match subcmd {
        "dir" => apply_igcare(
            subargs,
            &t!("cmd.care.dir_usage"),
            Config::add_cared_dir,
            "Un-ignored directories:",
            scope,
        ),
        "ext" => apply_igcare(
            subargs,
            &t!("cmd.care.ext_usage"),
            Config::add_cared_extension,
            "Extra supported extensions:",
            scope,
        ),
        "file" => apply_igcare(
            subargs,
            &t!("cmd.care.file_usage"),
            Config::add_cared_file,
            "Extra supported files:",
            scope,
        ),
        _ => {
            println!("{}", t!("cmd.care.title").cyan().bold());
            println!("  {}", t!("cmd.care.dir_desc"));
            println!("  {}", t!("cmd.care.ext_desc"));
            println!("  {}", t!("cmd.care.file_desc"));
            println!();
            println!("{}", t!("cmd.care.scope_help").green());
            println!();
            println!("{}", t!("cmd.ignore.care_examples_title").green());
            println!("  {}", t!("cmd.ignore.care_example_dir").dimmed());
            println!("  {}", t!("cmd.ignore.care_example_ext").dimmed());
            println!("  {}", t!("cmd.ignore.care_example_file").dimmed());
            Ok(false)
        }
    }
}

/// `igcared` — show the ignore/care settings, split by source
/// (`[global]` and `[local]`).
pub fn cmd_igcared(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    let global = Config::global_igcare_sets();
    let local = Config::local_igcare_sets();
    let local_path = Config::get_local_config_path();

    println!();
    println!("{}", t!("cmd.igcared.section_global").cyan().bold());
    println!();
    print!("{}", render_igcare_section(&global, false));

    println!();
    println!("{}", t!("cmd.igcared.section_local").cyan().bold());
    if let Some(ref path) = local_path {
        println!(
            "{}",
            t!("cmd.igcared.local_config", path.display()).dimmed()
        );
        println!();
        print!("{}", render_igcare_section(&local, true));
    } else {
        println!();
        println!("  {}", t!("cmd.igcared.no_local").dimmed());
    }
    println!();
    println!("{}", t!("cmd.igcared.scope_help").dimmed());
    Ok(false)
}

/// Render the ignore/care rows of one store.
fn render_igcare_section(sets: &crate::config::IgcareSets, with_cared: bool) -> String {
    let mut out = String::new();
    out.push_str(&render_igcare_row(
        &t!("cmd.igcared.ignored_dirs"),
        &sets.ignored_directory_names,
        false,
    ));
    if with_cared {
        out.push_str(&render_igcare_row(
            &t!("cmd.igcared.cared_dirs"),
            &sets.cared_directory_names,
            false,
        ));
    }
    out.push_str(&render_igcare_row(
        &t!("cmd.igcared.ignored_exts"),
        &sets.ignored_extensions,
        true,
    ));
    if with_cared {
        out.push_str(&render_igcare_row(
            &t!("cmd.igcared.cared_exts"),
            &sets.cared_extensions,
            true,
        ));
    }
    out.push_str(&render_igcare_row(
        &t!("cmd.igcared.extra_exts"),
        &sets.extra_supported_extensions,
        true,
    ));
    out.push_str(&render_igcare_row(
        &t!("cmd.igcared.ignored_files"),
        &sets.ignored_files,
        false,
    ));
    if with_cared {
        out.push_str(&render_igcare_row(
            &t!("cmd.igcared.cared_files"),
            &sets.cared_files,
            false,
        ));
    }
    out.push_str(&render_igcare_row(
        &t!("cmd.igcared.extra_files"),
        &sets.extra_supported_files,
        false,
    ));
    out
}

/// Render one ignore/care row: label + one item per line.
fn render_igcare_row(
    label: &str,
    values: &std::collections::HashSet<String>,
    dotted: bool,
) -> String {
    let mut out = String::new();
    out.push_str(&format!("{}\n", label.yellow()));
    if values.is_empty() {
        out.push_str(&format!("  {}\n", t!("cmd.igcared.none").dimmed()));
        return out;
    }
    let mut sorted: Vec<&String> = values.iter().collect();
    sorted.sort();
    for item in sorted {
        if dotted {
            out.push_str(&format!("  - .{}\n", item.green()));
        } else {
            out.push_str(&format!("  - {}\n", item.green()));
        }
    }
    out
}

pub fn cmd_igcare(args: &str, _nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("{}", t!("cmd.igcare.main_title").cyan().bold());
        println!("  {}", t!("cmd.igcare.export_cmd"));
        println!("  {}", t!("cmd.igcare.export_select_cmd"));
        println!("  {}", t!("cmd.igcare.import_cmd"));
        println!();
        println!("{}", t!("cmd.igcare.examples_title").green());
        println!("  {}", t!("cmd.igcare.example_export_all"));
        println!("  {}", t!("cmd.igcare.example_export_select"));
        println!("  {}", t!("cmd.igcare.example_import"));
    } else {
        let (scope, args) = split_scope_flag(args);
        let ig_parts: Vec<&str> = args.splitn(2, ' ').collect();
        let subcmd = ig_parts[0].to_lowercase();
        let subargs = ig_parts.get(1).unwrap_or(&"").trim();

        match subcmd.as_str() {
            "export" => {
                if subargs.is_empty() {
                    print_error(&t!("cmd.igcare.export_usage"));
                } else {
                    let export_parts: Vec<&str> = subargs.splitn(2, ' ').collect();
                    if export_parts.len() < 2 {
                        print_error(&t!("cmd.igcare.export_usage"));
                    } else {
                        let flag = export_parts[0].to_lowercase();
                        let export_name = export_parts[1];
                        if export_name.is_empty() {
                            print_error(&t!("cmd.igcare.export_name_empty"));
                        } else if flag == "--all" || flag == "-a" {
                            igcare_export_all(export_name)?;
                        } else if flag == "--select" || flag == "-s" {
                            igcare_export_select(export_name)?;
                        } else {
                            print_error(&t!("cmd.igcare.unknown_flag", flag));
                        }
                    }
                }
            }
            "import" => {
                if subargs.is_empty() {
                    print_error(&t!("cmd.igcare.import_usage"));
                    println!("{}", t!("cmd.igcare.import_example"));
                } else {
                    igcare_import(subargs, scope)?;
                }
            }
            "make" | "mk" => {
                let name = if !subargs.is_empty() {
                    subargs
                } else {
                    "example"
                };
                let path = std::env::current_dir()
                    .unwrap_or_default()
                    .join(format!("{}.ntc.igcare", name));
                if path.exists() {
                    print_error(&t!("cmd.igcare.file_exists", path.display()));
                } else {
                    let content = r#"# ntc ignore/care settings template
# Import with: igcare import <file>

ignored_directory_names = []
ignored_extensions = []
extra_supported_extensions = []
ignored_files = []
extra_supported_files = []

# Cared names are never ignored, even when the global config ignores them.
cared_directory_names = []
cared_extensions = []
cared_files = []
"#;
                    match std::fs::write(&path, content) {
                        Ok(()) => {
                            print_success(&t!("cmd.igcare.template_created", path.display()));
                            print_info(&t!("cmd.igcare.template_hint"));
                        }
                        Err(e) => print_error(&t!("cmd.igcare.create_failed", e)),
                    }
                }
            }
            _ => {
                print_error(&t!("cmd.igcare.unknown_subcmd", subcmd));
                println!("{}", t!("cmd.igcare.type_help"));
            }
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_support::TempCwd;

    /// `igcared` shows a `[global]` and a `[local]` section.
    #[test]
    fn igcared_lists_global_and_local_sections() {
        let _cwd = TempCwd::with_fixture(
            &[(
                "ntconfig.toml",
                "ignored_directory_names = [\"dist\"]\ncared_directory_names = [\"target\"]\n",
            )],
            &[],
        );
        let mut cfg = Config::default();
        cfg.ignored_directory_names.insert("build".into());
        Config::with_test_config(cfg, || {
            let global = Config::global_igcare_sets();
            let local = Config::local_igcare_sets();
            let global_section = render_igcare_section(&global, false);
            let local_section = render_igcare_section(&local, true);
            assert!(global_section.contains("build"), "got:\n{}", global_section);
            assert!(local_section.contains("dist"), "got:\n{}", local_section);
            // The local "cared" list is shown as its own row.
            assert!(local_section.contains("target"), "got:\n{}", local_section);
        });
    }

    /// Without a local config the `[local]` section says so.
    #[test]
    fn igcared_reports_a_missing_local_config() {
        let _cwd = TempCwd::with_fixture(&[], &[]);
        let local = Config::local_igcare_sets();
        assert!(local.ignored_directory_names.is_empty());
        assert!(local.cared_directory_names.is_empty());
    }
}
