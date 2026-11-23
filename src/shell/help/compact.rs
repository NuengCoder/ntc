//! Default `help`, plus the full reference behind `help --all`.

use crate::t;
use crate::utils::theme::style::ThemeColorize;

/// Default `help`: one line per command area (what it is), no subcommands or
/// examples. `help --all` still prints the full reference.
pub(crate) fn print_compact_help() {
    outln!("╔══════════════════════════════════════════════════════════════════╗");
    outln!(
        "║{}║",
        format!(
            "{:^66}",
            format!(
                "ntc {} - {}",
                env!("CARGO_PKG_VERSION"),
                t!("help.compact.title")
            )
        )
    );
    outln!("╚══════════════════════════════════════════════════════════════════╝");
    for (_, label, key) in crate::shell::commands::core::help::compact_entries() {
        outln!("  {:<14} {}", label, t!(key));
    }
    outln!();
    outln!("{}", t!("help.compact.footer").dimmed());
}

/// The full reference behind `help --all` (and `help <topic>` sections).
pub(crate) fn print_interactive_help() {
    outln!("╔══════════════════════════════════════════════════════════════════╗");
    outln!(
        "║                     ntc {} - Interactive Help                 ║",
        env!("CARGO_PKG_VERSION")
    );
    outln!("╚══════════════════════════════════════════════════════════════════╝");

    for section in crate::shell::commands::core::help::topics_in_order() {
        (section.render)();
    }
}
