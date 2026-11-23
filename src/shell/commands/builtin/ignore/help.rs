//! Help sections owned by the `ignore` command module: the `ignore` reference
//! and the shared `ignore`/`care` contextual help.

use crate::shell::help::format::print_section;
use crate::shell::help::outln;
use crate::shell::help::print_cr;

pub(crate) fn print_ignore_help_section() {
    print_section(
        "help.ignore.title",
        &["help.ignore.dir", "help.ignore.ext", "help.ignore.file"],
        &[],
        &["help.ignore.example1", "help.ignore.example2"],
    );
}

pub(crate) fn print_ignore_care_help(parts: &[&str], lowers: &[String]) -> bool {
    let root = lowers[0].as_str();
    if parts.len() == 1 {
        for s in crate::shell::hinter::subcommands_for(&[parts[0]]) {
            outln!("{}", s);
        }
        return true;
    }
    if parts.len() == 2 {
        match parts[1].to_lowercase().as_str() {
            "dir" => outln!("{} dir <dir_name>", root),
            "ext" => outln!("{} ext <ext>", root),
            "file" => outln!("{} file <file_name>", root),
            _ => {
                // Unknown subcommand: re-list valid choices (corrective).
                for s in crate::shell::hinter::subcommands_for(&[parts[0]]) {
                    outln!("{}", s);
                }
            }
        }
        return true;
    }
    // Value already supplied (`ignore dir foo`, possibly comma-separated
    // with extra words): complete.
    print_cr();
    true
}
