//! Help sections owned by the `scan` command module: the scan reference and `scan` contextual help.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_scan_help_section() {
    section_header("help.scan.title");
    usage("help.scan.ext");
    usage("help.scan.files");
    usage("help.scan.dir");
    usage("help.scan.depth");
    usage("help.scan.size");
    usage("help.scan.care");
    usage("help.scan.port");
    usage("help.scan.ip");
    usage("help.scan.pid");
    usage("help.scan.waep");
    usage("help.scan.waep_base");
    usage("help.scan.waep_flags");
    usage("help.scan.waep_recursive");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.scan.example1");
    example_row("help.scan.example2");
    example_row("help.scan.example3");
    example_row("help.scan.example4");
    example_row("help.scan.example5");
    example_row("help.scan.example6");
    example_row("help.scan.example7");
    example_row("help.scan.example8");
    example_row("help.scan.example9");
    example_row("help.scan.example10");
}

pub(crate) fn print_scan_context_help(parts: &[&str], lowers: &[String]) -> bool {
    use crate::shell::hinter::subcommands_for;
    if parts.len() == 1 {
        for s in subcommands_for(&[parts[0]]) {
            outln!("{}", s);
        }
        return true;
    }
    match lowers[1].as_str() {
        "ext" => outln!("scan ext <ext...> [-d|--depth <n>] [-s|--size] [--care]"),
        "port" => outln!("scan port [host] [--top N | --ports <spec> | --all] [--timeout MS]"),
        "ip" => outln!("scan ip [cidr] [--top N | --ports <spec>] [--timeout MS]"),
        "pid" => outln!(
            "scan pid [[user@]host[:port]|profile] [--user <u>] [--port <p>] [--key <path>] [--pass-env <var>] [--trust] [--timeout <s>] [--sort memory|pid|name] [--desc] [--filter <text>] [--min-memory <size>] [--max-memory <size>] [--offset <n>] [--limit <n>]"
        ),
        // waep is flag-heavy: show usage, examples, flags and option notes
        // instead of collapsing to the subcommand list.
        "waep" | "web" => {
            usage("help.scan.waep");
            usage("help.scan.waep_base");
            usage("help.scan.waep_flags");
            usage("help.scan.waep_recursive");
            outln!("{}", t!("cmd.scan.waep_flags_title").green());
            for line in t!("cmd.scan.waep_flags").lines() {
                outln!("  {}", line.trim_start());
            }
            outln!("{}", t!("cmd.scan.waep_options_title").green());
            for line in t!("cmd.scan.waep_options").lines() {
                outln!("  {}", line.trim_start());
            }
        }
        _ => {
            // Unknown subcommand: re-list valid choices (corrective).
            for s in subcommands_for(&[parts[0]]) {
                outln!("{}", s);
            }
        }
    }
    true
}
