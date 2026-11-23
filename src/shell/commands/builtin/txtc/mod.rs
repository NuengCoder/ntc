//! `txtc` — copy a file's contents to the clipboard.

pub(crate) mod help;

pub(crate) mod completion;

use crate::filetype::is_supported_format;
use crate::navigator::Navigator;
use crate::output::{print_error, print_success, print_warning};
use crate::t;
use anyhow::Result;
use std::path::Path;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::file_picker::{list_supported_files, show_file_selection_menu_range};
use super::super::support::report_txtc::{parse_txtc_args, txtc_show_lines};

pub fn cmd_txtc(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parsed = match parse_txtc_args(args) {
        Ok(p) => p,
        Err((key, arg)) => {
            print_error(&t!(&key, arg));
            return Ok(false);
        }
    };
    if let Some(ref file) = parsed.file {
        let expanded = crate::utils::paths::expand_path_str(file);
        let target = Path::new(&expanded);
        if target.is_file() {
            if is_supported_format(target) {
                // `--line` / `--no-line` override the config default, exactly
                // like `txtf`.
                let show_lines = txtc_show_lines(parsed.lines);
                let content = crate::output::cat_file_with_line_numbers(target, show_lines)?;

                #[cfg(target_os = "android")]
                {
                    use crate::output::print_info;
                    print_info(&t!("cmd.report.copying_clipboard", target.display()));
                    match crate::output::copy_to_clipboard(&content, "TXT") {
                        Ok(()) => {}
                        Err(e) => {
                            print_error(&t!("cmd.report.copy_failed", e));
                            let output_file = crate::output::build_output_path(&format!(
                                "copied_{}.txt",
                                target.file_name().unwrap_or_default().to_string_lossy()
                            ));
                            if let Ok(()) = crate::output::write_file(&output_file, &content) {
                                print_success(&t!(
                                    "cmd.report.content_saved",
                                    output_file.display()
                                ));
                                print_info(&t!("cmd.report.neovim_hint", output_file.display()));
                            }
                        }
                    }
                }

                #[cfg(not(target_os = "android"))]
                {
                    crate::output::copy_to_clipboard(&content, "TXT")?;
                    print_success(&t!("cmd.report.copied_clipboard", target.display()));
                }
            } else {
                print_warning(&t!("cmd.report.skipped_unsupported", file));
            }
        } else {
            print_error(&t!("cmd.report.file_not_found", file));
        }
    } else {
        // No file on the line: pick one interactively, same line-number rule.
        let files = list_supported_files(nav)?;
        show_file_selection_menu_range(nav, files, true, None, parsed.lines)?;
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "txtc",
    aliases: &[],
    help_topic: Some("txtc"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_txtc),
};
