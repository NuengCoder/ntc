//! Help sections owned by the `cd` command module: the `cd` reference and `cd`
//! contextual help.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::shell::help::print_cr;

pub(crate) fn print_cd_help_section() {
    print_section(
        "help.cd.title",
        &["help.cd.usage", "help.cd.cd_tp"],
        &[],
        &["help.cd.example1", "help.cd.example2"],
    );
}

pub(crate) fn print_cd_help(parts: &[&str], lowers: &[String]) -> bool {
    if parts.len() == 1 {
        outln!("cd <path>");
        outln!("cd to <tp-alias>");
        return true;
    }
    if parts.len() == 2 {
        if lowers[1] == "to" {
            let names = crate::shell::hinter::teleport_names_for_command(&["cd"]);
            if names.is_empty() {
                outln!("cd to <tp-alias>");
            } else {
                let mut sorted = names;
                sorted.sort();
                for n in sorted {
                    outln!("{}", n);
                }
            }
            return true;
        }
        // `cd <path>` already complete (path may contain spaces; full args
        // are treated as one path by cmd_cd).
        print_cr();
        return true;
    }
    // `cd to <name>` (or `cd <path...>`): complete, Enter runs it.
    print_cr();
    true
}
