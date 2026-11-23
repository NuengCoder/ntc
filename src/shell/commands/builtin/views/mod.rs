//! `views` — short, flat, optional newline-per-entry directory listing.

pub(crate) mod help;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::report_view::next_size_value;

pub fn cmd_views(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parsed = match crate::shell::helpers::parse_target_arg(args, nav) {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };
    let mut min: Option<u64> = None;
    let mut show_size = false;
    let mut sort = crate::explorer::TreeSortKey::Char;
    let mut newline = false;
    let mut trims: Vec<String> = Vec::new();
    let mut i = 0;
    while i < parsed.rest.len() {
        match parsed.rest[i].to_lowercase().as_str() {
            "-n" => newline = true,
            "--trim" => {
                let Some(v) = parsed.rest.get(i + 1) else {
                    print_error(&t!("cmd.views.missing_trim"));
                    println!("{}", t!("cmd.views.usage"));
                    return Ok(false);
                };
                if !v.is_empty() {
                    trims.push(v.to_lowercase());
                }
                i += 1;
            }
            other if other.starts_with("--trim=") => {
                let v = other.trim_start_matches("--trim=");
                if !v.is_empty() {
                    trims.push(v.to_lowercase());
                }
            }
            "-s" | "--size" => {
                show_size = true;
                if let Some(v) = next_size_value(&parsed.rest, i) {
                    min = Some(v);
                    i += 1;
                }
            }
            "--sort" => {
                let Some(key) = parsed.rest.get(i + 1).map(|s| s.to_lowercase()) else {
                    print_error(&t!("cmd.views.invalid_sort"));
                    println!("{}", t!("cmd.views.usage"));
                    return Ok(false);
                };
                sort = match key.as_str() {
                    "name" | "char" | "n" => crate::explorer::TreeSortKey::Char,
                    "ext" | "e" => crate::explorer::TreeSortKey::Ext,
                    "size" | "s" => crate::explorer::TreeSortKey::Size,
                    _ => {
                        print_error(&t!("cmd.views.invalid_sort_value", key));
                        println!("{}", t!("cmd.views.usage"));
                        return Ok(false);
                    }
                };
                i += 1;
            }
            other => {
                print_error(&t!("cmd.report.unknown_option", other));
                println!("{}", t!("cmd.views.usage"));
                return Ok(false);
            }
        }
        i += 1;
    }

    let target = parsed
        .path
        .unwrap_or_else(|| nav.current_path().to_path_buf());
    let mut tree =
        crate::explorer::generate_tree(target.to_string_lossy().as_ref(), Some(1), true, None);
    // Sizes are always computed, and always with care semantics.
    crate::explorer::compute_tree_sizes(&mut tree, None, true);
    if let Some(min) = min {
        tree.children.retain(|n| n.size.unwrap_or(0) >= min);
    }
    let ext_of = |name: &str| match name.rfind('.') {
        Some(i) if i + 1 < name.len() => name[i + 1..].to_lowercase(),
        _ => String::new(),
    };
    match sort {
        crate::explorer::TreeSortKey::Char => tree
            .children
            .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        crate::explorer::TreeSortKey::Ext => tree.children.sort_by(|a, b| {
            ext_of(&a.name)
                .cmp(&ext_of(&b.name))
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        }),
        crate::explorer::TreeSortKey::Size => tree.children.sort_by(|a, b| {
            b.size
                .unwrap_or(0)
                .cmp(&a.size.unwrap_or(0))
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        }),
    }

    if tree.children.is_empty() {
        println!("{}", t!("cmd.views.empty"));
        return Ok(false);
    }

    let theme = crate::utils::theme::ThemeManager::current();
    let c_dir = theme.shell.tree_dir.clone();
    let c_ignored = theme.shell.tree_ignored.clone();
    drop(theme);

    let paint = |node: &crate::explorer::TreeNode, text: &str| -> String {
        if node.is_ignored {
            c_ignored.paint_ansi(text)
        } else if node.is_dir {
            c_dir.paint_ansi_bold(text)
        } else {
            text.to_string()
        }
    };

    // Four display groups, in this order: plain dirs, plain files,
    // ignored dirs, ignored files. Empty groups are dropped.
    // When `--size` is set each entry is shown as `name (size)`; the
    // `visible` width includes the size so line-wrapping in bare mode
    // stays correct.
    // `--trim` removes its text from each name, and an entry whose name
    // trims away completely is dropped instead of printed blank.
    let mut groups: Vec<Vec<(usize, String)>> = Vec::new();
    for (is_dir, is_ignored) in [(true, false), (false, false), (true, true), (false, true)] {
        let names: Vec<(usize, String)> = tree
            .children
            .iter()
            .filter(|n| n.is_dir == is_dir && n.is_ignored == is_ignored)
            .filter_map(|n| {
                let base = trim_text(&n.name, &trims);
                if base.is_empty() {
                    return None;
                }
                let painted = paint(n, &base);
                if show_size {
                    let s = crate::explorer::human_readable_size(n.size.unwrap_or(0));
                    let text = format!("{} ({})", painted, s);
                    let visible = base.chars().count() + 1 + 1 + s.chars().count() + 1;
                    Some((visible, text))
                } else {
                    Some((base.chars().count(), painted))
                }
            })
            .collect();
        if !names.is_empty() {
            groups.push(names);
        }
    }

    // Everything filtered away: say so rather than printing a blank line.
    if groups.is_empty() {
        println!("{}", t!("cmd.views.empty"));
        return Ok(false);
    }

    // `-n`: one entry per line.
    if newline {
        for group in &groups {
            for (_, name) in group {
                println!("{}", name);
            }
        }
        return Ok(false);
    }

    // Bare `views`: names flow inline, groups joined by `...`, wrapped to
    // the terminal width. `--trim` applies to the separator too, so
    // `views --trim ...` prints the same listing as `views | trim ...`.
    let separator = trim_text("...", &trims);
    let width = crossterm::terminal::size()
        .map(|(w, _)| w as usize)
        .unwrap_or(100)
        .max(20);
    let mut line = String::new();
    let mut line_w = 0usize;
    let mut first_group = true;
    for group in groups {
        if !first_group && !separator.is_empty() {
            views_flow_push(
                &mut line,
                &mut line_w,
                separator.chars().count(),
                &separator,
                width,
            );
        }
        first_group = false;
        for (visible, name) in group {
            views_flow_push(&mut line, &mut line_w, visible, &name, width);
        }
    }
    if !line.is_empty() {
        println!("{}", line);
    }
    Ok(false)
}
fn trim_text(text: &str, trims: &[String]) -> String {
    if trims.is_empty() {
        return text.to_string();
    }
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < text.len() {
        let rest = &text[i..];
        let mut matched = 0;
        for pat in trims {
            if !pat.is_empty()
                && rest.len() >= pat.len()
                && rest.is_char_boundary(pat.len())
                && rest[..pat.len()].to_lowercase() == *pat
            {
                matched = pat.len();
                break;
            }
        }
        if matched > 0 {
            i += matched;
        } else {
            let ch = rest.chars().next().unwrap_or('\u{0}');
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    out
}

/// Append one already-painted word to the flowing line, starting a new
/// output line when the word would not fit. `visible` is the word's width
/// in characters, measured before painting so ANSI escapes never count.
fn views_flow_push(
    line: &mut String,
    line_w: &mut usize,
    visible: usize,
    word: &str,
    width: usize,
) {
    if *line_w == 0 {
        line.push_str(word);
        *line_w = visible;
    } else if *line_w + 1 + visible > width {
        println!("{}", line);
        line.clear();
        line.push_str(word);
        *line_w = visible;
    } else {
        line.push(' ');
        line.push_str(word);
        *line_w += 1 + visible;
    }
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "views",
    aliases: &[],
    help_topic: Some("views"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_views),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_text_removes_every_occurrence_case_insensitively() {
        let trims = vec!["src".to_string()];
        assert_eq!(trim_text("SRC_main_src.rs", &trims), "_main_.rs");
    }

    #[test]
    fn trim_text_applies_each_pattern() {
        let trims = vec!["-".to_string(), ".".to_string()];
        assert_eq!(trim_text("a-b-c.txt", &trims), "abctxt");
    }

    #[test]
    fn trim_text_can_empty_an_entry() {
        // A fully trimmed name must report empty so `views` can drop it,
        // instead of silently printing the name it was told to hide.
        let trims = vec!["src".to_string()];
        assert_eq!(trim_text("src", &trims), "");
    }

    #[test]
    fn trim_text_removes_the_group_separator() {
        // `views --trim ...` uses the same edit on the `...` between groups.
        let trims = vec!["...".to_string()];
        assert_eq!(trim_text("...", &trims), "");
        assert_eq!(trim_text("src", &trims), "src");
    }

    #[test]
    fn trim_text_does_not_split_multibyte_names() {
        let trims = vec!["é".to_string()];
        assert_eq!(trim_text("café.txt", &trims), "caf.txt");
    }
}
