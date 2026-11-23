//! Help sections owned by the `tp` command module: the `tp` reference and `tp`
//! contextual help.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::shell::help::print_cr;

pub(crate) fn print_tp_help_section() {
    print_section(
        "help.tp.title",
        &[
            "help.tp.menu",
            "help.tp.add",
            "help.tp.add_path",
            "help.tp.jump",
            "help.tp.jump_num",
            "help.tp.list",
            "help.tp.info",
            "help.tp.rm",
            "help.tp.rnm",
            "help.tp.cls",
            "help.tp.at",
        ],
        &[],
        &[
            "help.tp.example1",
            "help.tp.example2",
            "help.tp.example3",
            "help.tp.example4",
            "help.tp.example5",
        ],
    );
}

pub(crate) fn print_tp_context_help(parts: &[&str], lowers: &[String]) -> bool {
    use crate::shell::hinter::{subcommands_for, teleport_names_for_command};
    if parts.len() == 1 {
        for s in subcommands_for(&[parts[0]]) {
            outln!("{}", s);
        }
        return true;
    }
    let sub = lowers[1].as_str();
    if parts.len() == 2 {
        match sub {
            "jump" | "to" | "info" | "rm" => {
                let names = teleport_names_for_command(parts);
                if names.is_empty() {
                    outln!("tp {} <tp-alias>", sub);
                } else {
                    let mut sorted = names;
                    sorted.sort();
                    for n in sorted {
                        outln!("{}", n);
                    }
                }
            }
            "add" => outln!("tp add <name> [<path>]"),
            "rnm" | "rename" => outln!("tp rnm <old> to <new>"),
            "list" | "ls" | "cls" | "clear" | "help" => print_cr(),
            _ => {
                for s in subcommands_for(&[parts[0]]) {
                    outln!("{}", s);
                }
            }
        }
        return true;
    }
    // len >= 3: value-taking subcommands are complete once a value is given.
    match sub {
        "jump" | "to" | "info" | "rm" | "add" => print_cr(),
        "rnm" | "rename" => {
            // `tp rnm <old> to <new>` needs 5 tokens; fewer means still
            // incomplete — show usage, otherwise <cr>.
            if parts.len() >= 5 {
                print_cr();
            } else {
                outln!("tp rnm <old> to <new>");
            }
        }
        _ => print_cr(),
    }
    true
}
