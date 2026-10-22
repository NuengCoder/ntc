// src/shell/stdin.rs
// Virtual stdin for the ntc shell's pipeline / redirection support.
//
// A pipeline stage (or a `<` / `<<` redirect) installs the input bytes here
// before the command runs. Built-in commands that opt in read the buffer, and
// external system commands receive it as their real stdin. When nothing was
// installed and the process stdin is piped (e.g. `echo hi | ntc encode b64`),
// the buffer is filled lazily from the process stdin.

use std::cell::RefCell;
use std::io::{IsTerminal, Read};

thread_local! {
    static INPUT: RefCell<Option<Vec<u8>>> = RefCell::new(None);
}

/// Install the input bytes for the command that is about to run.
pub(crate) fn set_input(bytes: Vec<u8>) {
    INPUT.with(|i| *i.borrow_mut() = Some(bytes));
}

/// Whether an explicit input buffer is currently installed.
pub(crate) fn has_input() -> bool {
    INPUT.with(|i| i.borrow().is_some())
}

/// Whether any input is available — an installed buffer, or piped process
/// stdin. Does not drain anything.
pub(crate) fn has_any_input() -> bool {
    if has_input() {
        return true;
    }
    !std::io::stdin().is_terminal()
}

/// Drop any installed input buffer.
pub(crate) fn clear() {
    INPUT.with(|i| *i.borrow_mut() = None);
}

/// Take the installed input, or — when none is installed — read the process
/// stdin if it is piped (not a terminal). Returns `None` when there is no
/// input at all.
pub(crate) fn take_input() -> Option<Vec<u8>> {
    let explicit = INPUT.with(|i| i.borrow_mut().take());
    if explicit.is_some() {
        return explicit;
    }
    if !std::io::stdin().is_terminal() {
        let mut buf = Vec::new();
        if std::io::stdin().lock().read_to_end(&mut buf).is_ok() && !buf.is_empty() {
            return Some(buf);
        }
    }
    None
}

/// Take the installed input as lossy UTF-8 text.
pub(crate) fn read_to_string() -> Option<String> {
    take_input().map(|b| String::from_utf8_lossy(&b).into_owned())
}
