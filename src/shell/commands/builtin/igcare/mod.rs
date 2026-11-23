//! `igcare` — check, export, import, and scaffold ignore/care settings.

pub(crate) mod completion;
pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success};
use crate::shell::helpers::{igcare_export_all, igcare_export_select, igcare_import};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::config_scope::split_scope_flag;

pub fn cmd_igcare(args: &str, nav: &mut Navigator) -> Result<bool> {
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
            "check" => {
                let cwd = nav.current_path().to_path_buf();
                super::super::support::check::run(
                    "igcare",
                    subargs,
                    &cwd,
                    ".ntc.igcare",
                    super::super::support::check::formats::igcare,
                );
            }
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

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "igcare",
    aliases: &[],
    help_topic: Some("igcare"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_igcare),
};
