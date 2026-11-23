//! Help sections owned by the `time` command module: the `time` reference and `time` contextual help.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::shell::help::print_cr;
use crate::t;

pub(crate) fn print_time_help_section() {
    section_header("help.time.title");
    usage("help.time.run");
    usage("help.time.compare");
    usage("help.time.bench");
    usage("help.time.count");
    usage("help.time.help");
    outln!("  {}", crate::t!("help.time.note"));
}

pub(crate) fn print_time_context_help(parts: &[&str], lowers: &[String]) -> bool {
    let last = lowers.last().map(|s| s.as_str()).unwrap_or("");
    match last {
        "--count" => {
            outln!("{}", t!("help.time.count"));
            return true;
        }
        "--bench" => {
            outln!("{}", t!("help.time.bench"));
            return true;
        }
        "--compare" | "--with" => {
            outln!("{}", t!("help.time.compare"));
            return true;
        }
        _ => {}
    }
    if parts.len() == 1 {
        for flag in ["--count", "--bench", "--compare", "--help"] {
            outln!("{}", flag);
        }
        return true;
    }
    print_cr();
    true
}
