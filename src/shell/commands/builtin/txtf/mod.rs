//! `txtf` — display a file or pick one interactively with range flags.

pub(crate) mod help;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::output::{print_error, print_warning};
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::file_picker::show_file_selection_menu_range;
use super::super::support::file_picker::{
    list_supported_files, list_supported_files_in, show_file_selection_menu,
};
use super::super::support::report_txtf::{display_txtf_file, parse_txtf_args};
use super::super::support::report_view::resolve_view_operand;

pub fn cmd_txtf(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.trim().is_empty() {
        let files = list_supported_files(nav)?;
        show_file_selection_menu(nav, files, false)?;
        return Ok(false);
    }
    let parsed = match parse_txtf_args(args) {
        Ok(p) => p,
        Err((key, arg)) => {
            print_error(&t!(key.as_str(), arg));
            println!("{}", t!("help.txtf.usage"));
            return Ok(false);
        }
    };
    let range =
        match crate::output::parse_file_range(parsed.head, parsed.tail, parsed.betw.as_deref()) {
            Ok(r) => r,
            Err(()) => {
                print_error(&t!(
                    "cmd.report.invalid_range",
                    parsed.betw.clone().unwrap_or_default()
                ));
                println!("{}", t!("help.txtf.usage"));
                return Ok(false);
            }
        };
    match parsed.file {
        Some(f) => {
            let target = match resolve_view_operand(&f, nav) {
                Some(p) => p,
                None => {
                    let list_name = f
                        .strip_prefix('@')
                        .unwrap_or(&f)
                        .split('/')
                        .next()
                        .unwrap_or(&f);
                    print_error(&t!("cmd.target.tp_not_found", list_name));
                    return Ok(false);
                }
            };
            if target.is_dir() {
                let files = list_supported_files_in(&target)?;
                if files.is_empty() {
                    print_warning(&t!("cmd.report.no_files_in_dir", f));
                    return Ok(false);
                }
                show_file_selection_menu_range(nav, files, false, range, parsed.lines)?;
            } else {
                display_txtf_file(&target, range, &f, parsed.lines)?;
            }
        }
        None => {
            let files = list_supported_files(nav)?;
            show_file_selection_menu_range(nav, files, false, range, parsed.lines)?;
        }
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "txtf",
    aliases: &[],
    help_topic: Some("txtf"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_txtf),
};
