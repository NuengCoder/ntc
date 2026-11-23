//! `fsc` — find a file and view or copy it interactively.

pub(crate) mod help;

use crate::config::Config;
use crate::filetype::is_supported_format;
use crate::navigator::Navigator;
use crate::output::{cat_file, print_error, print_success, print_warning};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::io::{self, Write};

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::search::parse_search_args;

pub(crate) fn cmd_fsc(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("misc.fsc.usage"));
        println!("{}", t!("misc.fsc.example1"));
        println!("{}", t!("misc.fsc.example2"));
        return Ok(false);
    }
    let (pattern, max_depth, fuzzy) = parse_search_args(args);
    let results = crate::search::search_files_opts(nav.current_path(), &pattern, max_depth, fuzzy);
    if results.is_empty() {
        let output = crate::search::format_search_results(&results, &pattern, max_depth, true);
        print!("{}", output);
        return Ok(false);
    }
    loop {
        println!();
        println!(
            "{}",
            t!("misc.fsc.found", results.len(), pattern).cyan().bold()
        );
        for (i, r) in results.iter().enumerate() {
            let path_str = r.full_path.to_string_lossy();
            println!("  {}. {}", (i + 1).to_string().yellow(), path_str.dimmed());
        }
        println!("  {}", t!("misc.fsc.done_opt").red());
        println!();
        let labels: Vec<String> = results
            .iter()
            .map(|result| {
                result
                    .full_path
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_else(|| result.full_path.display().to_string())
            })
            .collect();
        let Some(input) = crate::utils::interactive::read_number_with_hint(
            &t!("misc.fsc.select_prompt"),
            |text| text.green().to_string(),
            &labels,
        )?
        else {
            println!("{}", t!("misc.fsc.done_str").dimmed());
            return Ok(false);
        };
        let input = input.trim();
        if input == "0" || input.is_empty() {
            println!("{}", t!("misc.fsc.done_str").dimmed());
            return Ok(false);
        }
        if let Ok(n) = input.parse::<usize>() {
            if n >= 1 && n <= results.len() {
                let target = &results[n - 1].full_path;
                if target.is_file() {
                    if is_supported_format(target) {
                        let show_lines = Config::global_get_show_line_numbers();
                        cat_file(target, show_lines)?;
                        println!();
                        print!("{} ", t!("misc.fsc.copy_prompt").yellow());
                        io::stdout().flush()?;
                        let mut copy_choice = String::new();
                        io::stdin().read_line(&mut copy_choice)?;
                        let copy_choice = copy_choice.trim().to_lowercase();
                        if copy_choice == "y" || copy_choice == "yes" {
                            let content =
                                crate::output::cat_file_with_line_numbers(target, show_lines)?;
                            crate::output::copy_to_clipboard(&content, "TXT")?;
                            print_success(&t!("misc.fsc.copied", target.display()));
                        }
                    } else {
                        print_warning(&t!("misc.fsc.skipped", target.display()));
                    }
                } else {
                    print_warning(&t!("misc.fsc.not_file"));
                }
            } else {
                print_error(&t!("misc.fsc.invalid_selection"));
            }
        } else {
            print_error(&t!("misc.fsc.invalid_input"));
        }
        println!();
    }
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "fsc",
    aliases: &[],
    help_topic: Some("fsc"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_fsc),
};
