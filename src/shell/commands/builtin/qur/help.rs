//! Help sections owned by the `qur` command module: the `qur` reference and
//! `qur` contextual help.

use crate::shell::help::format::print_section;
use crate::shell::help::outln;
use crate::t;

pub(crate) fn print_qur_help_section() {
    print_section(
        "help.qur.title",
        &["help.qur.usage"],
        &[],
        &["help.qur.example1"],
    );
}

pub(crate) fn print_qur_context_help(parts: &[&str], lowers: &[String]) -> bool {
    use crate::shell::hinter::subcommands_for;
    // Bare `qur`: needs a file glob first.
    if parts.len() == 1 {
        outln!("{}", t!("help.context.qur.needs_glob"));
        for s in subcommands_for(&[parts[0]]) {
            outln!("{}", s);
        }
        return true;
    }
    let has = |flags: &[&str]| {
        lowers
            .iter()
            .any(|t| flags.contains(&t.as_str()) || t.starts_with("--limit="))
    };
    let has_select = has(&["-s", "--select"]);
    let has_replace = has(&["-rp", "--replace"]);
    let has_display = has(&["-dp", "--display"]);
    // Value-taking flag awaiting its value (`-s`, `-rp`, `-d`, `-l`) → show slot.
    if let Some(last) = lowers.last().map(|s| s.as_str()) {
        match last {
            "-s" | "--select" => {
                outln!("{}", t!("help.context.qur.select_value"));
                return true;
            }
            "-rp" | "--replace" => {
                outln!("{}", t!("help.context.qur.replace_value"));
                return true;
            }
            "-d" | "--depth" => {
                outln!("{}", t!("help.context.qur.depth_value"));
                return true;
            }
            "-l" | "--limit" => {
                outln!("{}", t!("help.context.qur.limit_value"));
                return true;
            }
            _ => {}
        }
    }
    if !has_select {
        // Glob given, no `-s` yet: next step is select or depth/limit.
        outln!("{}", t!("help.context.qur.next_select"));
        return true;
    }
    if has_replace || has_display {
        // Select + display/replace present: only depth/limit (or Enter) remain.
        outln!("{}", t!("help.context.qur.next_depth"));
        return true;
    }
    // Select present without display/replace: suggest both next steps.
    outln!("{}", t!("help.context.qur.next_display_replace"));
    true
}
