//! Help rendering: the compact `help`, the full reference, per-topic sections
//! and Cisco-style contextual help.
//!
//! Command-specific help text lives in each command module
//! (`commands::builtin::<cmd>::help`); the registry in
//! [`crate::shell::commands::builtin`] decides topic lookup and order.

use std::cell::RefCell;

thread_local! {
    /// Test-only sink. When set, help output is captured instead of printed.
    static HELP_SINK: RefCell<Option<String>> = const { RefCell::new(None) };
}

pub(crate) fn write_help_line(args: std::fmt::Arguments<'_>) {
    HELP_SINK.with(|sink| {
        if let Some(out) = sink.borrow_mut().as_mut() {
            use std::fmt::Write;
            let _ = out.write_fmt(args);
            out.push('\n');
        } else {
            println!("{}", args);
        }
    });
}

/// Render help through the same code path as the shell, keeping the bytes in
/// a `String`. Used by the golden tests in [`tests`].
#[cfg(test)]
fn capture_help<T>(f: impl FnOnce() -> T) -> (T, String) {
    HELP_SINK.with(|sink| *sink.borrow_mut() = Some(String::new()));
    let result = f();
    let out = HELP_SINK.with(|sink| sink.borrow_mut().take().unwrap_or_default());
    (result, out)
}

macro_rules! outln {
    () => {
        $crate::shell::help::write_help_line(format_args!(""))
    };
    ($($arg:tt)*) => {
        $crate::shell::help::write_help_line(format_args!($($arg)*))
    };
}

pub(crate) use outln;

pub(crate) mod format;

mod compact;
mod contextual;

#[cfg(test)]
mod tests;

pub(crate) use self::compact::{print_compact_help, print_interactive_help};
pub(crate) use self::contextual::{print_contextual_help, strip_help_suffix};
pub(crate) use crate::shell::commands::builtin::deadlock::help::print_deadlock_help_section;
pub(crate) use crate::shell::commands::builtin::ndb::help::print_ndb_help_section;
pub(crate) use crate::shell::commands::builtin::tp::help::print_tp_help_section;

/// Print a single help section for `help <topic>` / `<cmd> -h`.
/// Returns false when the topic is unknown.
pub(crate) fn print_topic_help(topic: &str) -> bool {
    let Some(section) = crate::shell::commands::core::help::find_topic(topic) else {
        return false;
    };
    (section.render)();
    true
}

/// Print the generic "run this" row used by contextual help.
pub(crate) fn print_cr() {
    outln!("{}", crate::t!("help.context.cr"));
}
