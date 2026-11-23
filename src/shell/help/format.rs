//! Shared formatting primitives for help sections.

use crate::t;
use crate::utils::theme::style::ThemeColorize;

/// Print one command section: header, usage rows, notes, then examples.
pub(crate) fn print_section(
    title: &str,
    usage_keys: &[&str],
    note_keys: &[&str],
    example_keys: &[&str],
) {
    section_header(title);
    for key in usage_keys {
        usage(key);
    }
    for key in note_keys {
        usable_note(key);
    }
    if !example_keys.is_empty() {
        outln!("{}", t!("help.examples_header").green());
        for key in example_keys {
            example_row(key);
        }
    }
}

pub(crate) fn section_header(key: &str) {
    outln!();
    let theme = crate::utils::theme::ThemeManager::current();
    outln!(
        "{}",
        theme
            .shell
            .paint_bold(crate::utils::theme::ShellRole::HelpHeader, &t!(key))
    );
}

pub(crate) fn usage(key: &str) {
    let val = t!(key);
    if let Some(pos) = val.find(" ## ") {
        let (head, desc) = (&val[..pos], &val[pos + 4..]);
        if head.chars().count() <= USAGE_COLUMN {
            outln!("  {:<38} {}", head, desc);
        } else {
            // A long usage line would push the description so far right that the
            // two read as one sentence, so keep the description on its own line.
            outln!("  {}", head);
            outln!("    {}", desc);
        }
    } else {
        outln!("  {}", val);
    }
}

/// Width of the usage column in `usage()` and `example_row()`.
const USAGE_COLUMN: usize = 38;

pub(crate) fn example_row(key: &str) {
    let val = t!(key);
    if let Some(pos) = val.find(" ## ") {
        outln!("    {:<38} # {}", &val[..pos], &val[pos + 4..]);
    } else {
        outln!("    {}", val);
    }
}
/// Print an indented note row (no `##` split).
pub(crate) fn usable_note(key: &str) {
    outln!("  {}", t!(key));
}
