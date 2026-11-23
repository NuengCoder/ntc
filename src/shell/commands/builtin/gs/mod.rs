//! `gs` — search file contents, or piped text.

pub(crate) mod completion;
pub(crate) mod help;

pub(crate) mod interactive;

use crate::navigator::Navigator;
use crate::output::{print_error, print_warning};
use crate::shell::helpers::run_with_spinner;
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::search::{parse_gs_args, GsSort};

/// With `-A`/`-B` on, only the first 10 matching blocks are printed unless
/// `-l <n>` says otherwise. Without context, `gs` keeps returning everything.
const DEFAULT_CONTEXT_MATCHES: usize = 10;

pub(crate) fn cmd_gs(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.trim().is_empty() {
        print_error(&t!("misc.gs.usage"));
        // When piped, keep stdout clean (examples would land in the pipe/file).
        if !crate::shell::stdin::has_any_input() {
            println!("{}", t!("misc.gs.example1"));
            println!("{}", t!("misc.gs.example2"));
            println!("{}", t!("misc.gs.example3"));
            println!("{}", t!("misc.gs.example4"));
            println!("{}", t!("misc.gs.example5"));
            println!("{}", t!("misc.gs.example6"));
            println!("{}", t!("misc.gs.example7"));
            println!("{}", t!("misc.gs.example8"));
            println!("{}", t!("misc.gs.example9"));
            println!("{}", t!("misc.gs.example10"));
        }
        return Ok(false);
    }
    let parsed = parse_gs_args(args);
    if parsed.legacy_interactive {
        print_error(&t!("misc.gs.i_renamed"));
        return Ok(false);
    }
    if parsed.mode_conflict {
        print_error(&t!("misc.gs.mode_conflict"));
        return Ok(false);
    }
    let opts = parsed.opts;
    let pattern = parsed.pattern.trim().to_string();
    let max_depth = parsed.max_depth;

    // Piped / redirected input: search the provided text (same
    // case-insensitive substring semantics as the file search) and print the
    // matching lines grep-style, instead of walking the filesystem. Reading
    // yields `None` for a terminal (or an empty pipe), so `ntc gs pat` still
    // walks files when nothing was given on stdin.
    let piped_text = crate::shell::stdin::read_to_string();
    let piped = piped_text.is_some();

    // `-p` scopes the filesystem walk — it makes no sense on piped text.
    if piped && !parsed.paths.is_empty() {
        print_error(&t!("misc.gs.no_pipe_path"));
        return Ok(false);
    }

    let context = parsed.context;

    // `gs -i` with no pattern opens the pager's own pattern prompt instead of
    // failing with usage; piped text and non-interactive runs still need one.
    if pattern.is_empty() {
        if !piped && parsed.interactive {
            let root = nav.current_path().to_path_buf();
            let request = interactive::SearchRequest {
                root,
                max_depth,
                paths: parsed.paths,
                sort: parsed.sort,
                context,
                context_requested: parsed.context_requested,
                opts,
            };
            return interactive::run_gs_interactive(Vec::new(), &request, "");
        }
        print_error(&t!("misc.gs.usage"));
        return Ok(false);
    }
    let default_limit = if context.is_active() {
        Some(DEFAULT_CONTEXT_MATCHES)
    } else {
        None
    };

    if let Some(text) = piped_text {
        let mut matches = crate::search::search_text_lines_opts(&text, &pattern, opts);
        if let Some(sort) = parsed.sort {
            match sort {
                // `-s path|size` need file metadata that piped text lacks.
                GsSort::Path | GsSort::Size => {
                    print_error(&t!("misc.gs.pipe_sort_only_line"));
                    return Ok(false);
                }
                GsSort::Line => {
                    matches.sort_by(|a, b| {
                        a.line
                            .to_lowercase()
                            .cmp(&b.line.to_lowercase())
                            .then_with(|| a.line_number.cmp(&b.line_number))
                    });
                }
            }
        }
        if let Some(lim) = parsed.limit.or(default_limit) {
            if matches.len() > lim {
                matches.truncate(lim);
            }
        }
        if matches.len() > 20_000 {
            matches.truncate(20_000);
        }
        if context.is_active() {
            print!(
                "{}",
                crate::search::format_text_matches_with_context_opts(
                    &text, &matches, &pattern, context, opts
                )
            );
        } else {
            print!(
                "{}",
                crate::search::format_text_matches_opts(&matches, &pattern, opts)
            );
        }
        return Ok(false);
    }

    let root = nav.current_path().to_path_buf();
    let request = interactive::SearchRequest {
        root: root.clone(),
        max_depth,
        paths: parsed.paths,
        sort: parsed.sort,
        context,
        context_requested: parsed.context_requested,
        opts,
    };
    // Spinner in background thread so broad `-d` searches show progress
    // instead of looking crashed. Heavy-file guards live in search_content.
    let mut results = run_with_spinner(&t!("misc.gs.searching"), || {
        search_matches(&request, &pattern)
    });
    let total_found = results.len();
    if results.len() > 20_000 {
        print_warning(&t!("misc.gs.many_results", results.len()));
        results.truncate(20_000);
    }
    // -l/--limit is non-interactive only (paged UI has its own 10/page).
    if !parsed.interactive {
        if let Some(lim) = parsed.limit.or(default_limit) {
            if results.len() > lim {
                results.truncate(lim);
            }
        }
    }
    if parsed.interactive {
        return interactive::run_gs_interactive(results, &request, &pattern);
    }
    let output = if context.is_active() {
        crate::search::format_content_results_with_context_opts(
            &results,
            &pattern,
            max_depth,
            &root,
            total_found,
            context,
            opts,
        )
    } else {
        crate::search::format_content_results_opts(
            &results,
            &pattern,
            max_depth,
            &root,
            total_found,
            opts,
        )
    };
    print!("{}", output);
    Ok(false)
}

/// Shared search used by `gs` and the interactive pager's re-search (`f`).
/// Sorting is applied here; truncation stays with the caller.
pub(crate) fn search_matches(
    request: &interactive::SearchRequest,
    pattern: &str,
) -> Vec<crate::search::ContentMatch> {
    let mut results = if request.paths.is_empty() {
        crate::search::search_content_opts(&request.root, pattern, request.max_depth, request.opts)
    } else {
        let files = super::qur::resolve_qur_files(&request.root, &request.paths, request.max_depth);
        crate::search::search_content_in_files_opts(&files, pattern, request.opts)
    };
    if let Some(sort) = request.sort {
        let content_sort = match sort {
            GsSort::Line => crate::search::ContentSort::Line,
            GsSort::Path => crate::search::ContentSort::Path,
            GsSort::Size => crate::search::ContentSort::Size,
        };
        crate::search::sort_content_matches(&mut results, content_sort);
    }
    results
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "gs",
    aliases: &[],
    help_topic: Some("gs"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_gs),
};
