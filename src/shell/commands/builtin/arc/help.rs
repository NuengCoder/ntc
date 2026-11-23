//! Help sections owned by the `arc` command module: the archive reference and `arc` contextual help.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_arc_help_section() {
    section_header("help.arc.title");
    usage("help.arc.create");
    usage("help.arc.extract");
    usage("help.arc.list");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.arc.example1");
    example_row("help.arc.example2");
    example_row("help.arc.example3");
}

pub(crate) fn print_arc_context_help(parts: &[&str], lowers: &[String]) -> bool {
    use crate::shell::hinter::subcommands_for;
    if parts.len() == 1 {
        for s in subcommands_for(&[parts[0]]) {
            outln!("{}", s);
        }
        return true;
    }
    match lowers[1].as_str() {
        "create" | "c" | "mk" => {
            outln!("arc create <out.zip|tar.gz|tgz|tar> <files...>");
            outln!("  e.g. arc create backup.zip src Cargo.toml");
        }
        "list" | "ls" | "l" => outln!("arc list <archive>"),
        "extract" | "x" | "unzip" => outln!("arc extract <archive> [dest]"),
        _ => {
            // Unknown subcommand: re-list valid choices (corrective).
            for s in subcommands_for(&[parts[0]]) {
                outln!("{}", s);
            }
        }
    }
    true
}
