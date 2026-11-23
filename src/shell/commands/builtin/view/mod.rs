//! `view` — directory tree with size, sort, filter, and file-preview modes.

pub(crate) mod completion;
pub(crate) mod help;

use crate::config::Config;
use crate::navigator::Navigator;
use crate::output::print_error;
use crate::shell::helpers::show_tree_at_ex;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::report_txtf::display_txtf_file;
use super::super::support::report_view::{
    extract_ext_filter, parse_view_flags, split_view_file_args, ViewMode,
};

pub fn cmd_view(args: &str, nav: &mut Navigator) -> Result<bool> {
    cmd_view_impl(ViewMode::All, args, nav)
}
fn view_usage_key(mode: ViewMode) -> &'static str {
    match mode {
        ViewMode::All => "help.view.file_usage",
        ViewMode::Files => "help.viewf.file_usage",
        ViewMode::Dirs => "help.viewd.usage",
    }
}

fn view_tree_usage_key(mode: ViewMode) -> &'static str {
    match mode {
        ViewMode::All => "cmd.report.view_usage",
        ViewMode::Files => "help.viewf.tree",
        ViewMode::Dirs => "help.viewd.usage",
    }
}

pub(super) fn cmd_view_impl(mode: ViewMode, args: &str, nav: &mut Navigator) -> Result<bool> {
    let cwd = nav.current_path().to_path_buf();

    // `view -` displays piped / redirected input.
    if mode == ViewMode::All && args.trim() == "-" {
        if let Some(text) = crate::shell::stdin::read_to_string() {
            let show_lines = Config::global_get_show_line_numbers();
            if show_lines {
                for (i, line) in text.lines().enumerate() {
                    println!("{:>5} | {}", i + 1, line);
                }
            } else {
                print!("{}", text);
                if !text.is_empty() && !text.ends_with('\n') {
                    println!();
                }
            }
            return Ok(false);
        }
    }

    // `viewd` never filters by extension (directory rows only).
    let (exts, ext_seen, cleaned) = if mode == ViewMode::Dirs {
        (Vec::new(), false, args.to_string())
    } else {
        extract_ext_filter(args, &cwd)
    };
    if ext_seen && exts.is_empty() {
        print_error(&t!("cmd.report.ext_usage"));
        return Ok(false);
    }

    // `<view|viewf> <file>` displays the file like `txtf` (only
    // display-range flags allowed). Extension filters never apply to files.
    if mode != ViewMode::Dirs && !ext_seen {
        match split_view_file_args(&cleaned, nav, mode) {
            Some(Err(())) => {
                println!("{}", t!(view_usage_key(mode)));
                return Ok(false);
            }
            Some(Ok(fv)) => {
                let range =
                    match crate::output::parse_file_range(fv.head, fv.tail, fv.betw.as_deref()) {
                        Ok(r) => r,
                        Err(()) => {
                            print_error(&t!(
                                "cmd.report.invalid_range",
                                fv.betw.clone().unwrap_or_default()
                            ));
                            println!("{}", t!(view_usage_key(mode)));
                            return Ok(false);
                        }
                    };
                let label = fv.path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                display_txtf_file(&fv.path, range, label, None)?;
                return Ok(false);
            }
            None => {}
        }
    }

    let parsed = match mode {
        ViewMode::Dirs => crate::shell::helpers::parse_target_arg(&cleaned, nav),
        _ => crate::shell::helpers::parse_target_arg_allow_file(&cleaned, nav),
    };
    let parsed = match parsed {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };
    let parts: Vec<String> = parsed.rest;
    let f = match parse_view_flags(&parts) {
        Ok(f) => f,
        Err((key, arg)) => {
            print_error(&t!(key.as_str(), arg));
            println!("{}", t!(view_tree_usage_key(mode)));
            return Ok(false);
        }
    };

    let unsupported = match mode {
        ViewMode::All => f.file_only && f.dir_only,
        ViewMode::Files => f.dir_only || f.depth_override.is_some(),
        ViewMode::Dirs => f.file_only || f.dir_only || f.head.is_some() || f.tail.is_some(),
    };
    if unsupported {
        let flag = if f.file_only && f.dir_only {
            "-f/-dir"
        } else if mode == ViewMode::Files && f.depth_override.is_some() {
            "-d"
        } else {
            "flag"
        };
        print_error(&t!("cmd.report.unknown_option", flag));
        println!("{}", t!(view_tree_usage_key(mode)));
        return Ok(false);
    }

    // A file target with tree-only flags (e.g. `--ext`) is a usage error.
    if let Some(target) = &parsed.path {
        if target.is_file() {
            println!("{}", t!(view_usage_key(mode)));
            return Ok(false);
        }
    }

    let (show_filesize, show_dirsize, copy_to_clipboard, care_sizes, show_time) = (
        f.show_filesize,
        f.show_dirsize,
        f.copy_to_clipboard,
        f.care_sizes,
        f.show_time,
    );
    let (file_min, dir_min, sort, head, tail) = (f.file_min, f.dir_min, f.sort, f.head, f.tail);
    let entry_filter = match mode {
        ViewMode::Files => crate::explorer::TreeEntryFilter::FilesOnly,
        ViewMode::Dirs => crate::explorer::TreeEntryFilter::DirsOnly,
        ViewMode::All if f.file_only => crate::explorer::TreeEntryFilter::FilesOnly,
        ViewMode::All if f.dir_only => crate::explorer::TreeEntryFilter::DirsOnly,
        ViewMode::All => crate::explorer::TreeEntryFilter::All,
    };
    let max_depth = match mode {
        ViewMode::Files => 1,
        _ => f.depth_override.unwrap_or(Config::global_get_max_depth()),
    };
    // Directory sizes need the file nodes even when `viewd` hides them.
    let need_dir_sizes = show_dirsize
        || dir_min.is_some()
        || care_sizes
        || sort == Some(crate::explorer::TreeSortKey::Size);
    let include_files = mode != ViewMode::Dirs || need_dir_sizes;
    let start = std::time::Instant::now();
    let filter = if exts.is_empty() {
        None
    } else {
        Some(exts.as_slice())
    };
    let opts = crate::shell::helpers::TreeViewOpts {
        file_min,
        dir_min,
        sort,
        head,
        tail,
        entry_filter,
        sort_dirs_by_size: false,
        sort_files_by_size: false,
    };
    let target = parsed
        .path
        .unwrap_or_else(|| nav.current_path().to_path_buf());
    let matched = show_tree_at_ex(
        &target,
        Some(max_depth),
        true,
        include_files,
        show_filesize,
        show_dirsize,
        copy_to_clipboard,
        care_sizes,
        filter,
        &opts,
    );
    if filter.is_some() && matched == 0 {
        println!(
            "{}",
            t!("cmd.scan.no_files", crate::filetype::format_ext_list(&exts))
        );
    }
    if show_time {
        let elapsed = crate::explorer::format_duration(start.elapsed());
        let time_label = t!("cmd.report.time_label");
        let w = time_label.len() + elapsed.len() + 8;
        println!("┌{}┐", "─".repeat(w));
        println!("│ {} {} ", time_label.cyan(), elapsed.cyan());
        println!("└{}┘", "─".repeat(w));
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "view",
    aliases: &[],
    help_topic: Some("view"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_view),
};
