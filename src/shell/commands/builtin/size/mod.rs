//! `size` — show the byte size of a file or directory tree.

pub(crate) mod completion;
pub(crate) mod help;

use crate::explorer::{calculate_target_size, format_duration, human_readable_size};
use crate::filetype::strip_unc_prefix;
use crate::navigator::Navigator;
use crate::shell::helpers::{parse_target_arg_allow_file, run_with_spinner};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_size(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parsed = match parse_target_arg_allow_file(args, nav) {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };
    let show_path = parsed.path.is_some();
    let target = parsed
        .path
        .unwrap_or_else(|| nav.current_path().to_path_buf());

    let mut care = false;
    let mut show_time = false;
    for arg in &parsed.rest {
        match arg.as_str() {
            "--care" => care = true,
            "--time" | "-t" => show_time = true,
            _ => {}
        }
    }
    let start = std::time::Instant::now();
    // Files skip the directory walk (and spinner): a single metadata call.
    if target.is_file() {
        let total = calculate_target_size(&target, care);
        let elapsed = start.elapsed();
        let human_str = human_readable_size(total);
        let display = strip_unc_prefix(&target);
        if show_time {
            println!("{} {} ({})", display, human_str, format_duration(elapsed));
        } else {
            println!("{} {}", display, human_str);
        }
        return Ok(false);
    }
    let total = if care {
        run_with_spinner(&t!("misc.size.calculating"), || {
            calculate_target_size(&target, true)
        })
    } else {
        run_with_spinner(&t!("misc.size.calculating"), || {
            calculate_target_size(&target, false)
        })
    };
    let elapsed = start.elapsed();
    let bytes_str = format!("{}", total);
    let human_str = human_readable_size(total);
    let time_str = format_duration(elapsed);
    let path_str = if show_path {
        strip_unc_prefix(&target)
    } else {
        String::new()
    };
    let max_label = 8;
    let mut max_value = bytes_str.len().max(human_str.len()).max(time_str.len());
    if show_path {
        max_value = max_value.max(path_str.len());
    }
    let w = max_label + max_value + 10;

    let bytes_label = t!("misc.size.bytes_label");
    let human_label = t!("misc.size.human_label");
    let time_label = t!("misc.size.time_label");
    let title_label = t!("misc.size.title");
    let a = |s: &str| crate::utils::terminal::asciify(s);
    let v = a("│");
    let h = a("─");
    println!();
    println!("{}{}{}", a("┌"), h.repeat(w), a("┐"));
    println!("{}{:^w$}", v, title_label.cyan().bold(), w = w);
    if show_path {
        let path_label = t!("misc.size.path_label");
        println!("{}{}{}", a("├"), h.repeat(w), a("┤"));
        println!(
            "{} {} {:<w$} ",
            v,
            path_label.yellow(),
            path_str.cyan(),
            w = w - path_label.len() - 4
        );
    }
    println!("{}{}{}", a("├"), h.repeat(w), a("┤"));
    println!(
        "{} {} {:<w$} ",
        v,
        bytes_label.yellow(),
        bytes_str.yellow(),
        w = w - bytes_label.len() - 4
    );
    println!(
        "{} {} {:<w$} ",
        v,
        human_label.green().bold(),
        human_str.green().bold(),
        w = w - human_label.len() - 4
    );
    if show_time {
        println!("{}{}{}", a("├"), h.repeat(w), a("┤"));
        println!(
            "{} {} {:<w$} ",
            v,
            time_label.cyan(),
            time_str.cyan(),
            w = w - time_label.len() - 4
        );
    }
    println!("{}{}{}", a("└"), h.repeat(w), a("┘"));
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "size",
    aliases: &[],
    help_topic: Some("size"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_size),
};
