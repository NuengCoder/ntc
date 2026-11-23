//! Per-file configuration read before a script runs.
//!
//! Two directives sit next to the commands but are not commands themselves:
//!
//! ```text
//! set-pipe = fail        this script refuses to be piped or redirected
//! set-showexc = off      hide the `[n] Executing: …` trace lines
//! ```
//!
//! Both take the same truth vocabulary, so `set-pipe = 0`, `= false`, `= off`
//! and `= fail` all mean "no", and `= 1`, `= true` and `= on` all mean "yes".
//! They are read from the file *before* execution (and by the pipeline, which
//! needs to know whether a stage may be piped at all), so they are scanned
//! separately from the statement parser and never run as commands.

/// Effective settings for one script file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Directives {
    /// May this script's output be piped or redirected? Default: yes.
    pub pipe: bool,
    /// Print the `[n] Executing: …` line before each command? Default: yes.
    pub show_exec: bool,
}

impl Default for Directives {
    fn default() -> Self {
        Self {
            pipe: true,
            show_exec: true,
        }
    }
}

/// Interpret a directive value.
///
/// `fail` / `error` count as "off" because the value is read as a switch: a
/// script that says `set-pipe = fail` means "piping fails here".
pub(crate) fn parse_bool(value: &str) -> Option<bool> {
    let value = value
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .trim()
        .to_ascii_lowercase();
    match value.as_str() {
        "1" | "true" | "on" | "yes" | "y" | "enable" | "enabled" | "pass" | "ok" => Some(true),
        "0" | "false" | "off" | "no" | "n" | "disable" | "disabled" | "fail" | "error" => {
            Some(false)
        }
        _ => None,
    }
}

/// Split a directive line into its name and raw value.
///
/// Returns `None` for anything that is not `set-pipe` / `set-showexc`, so the
/// parser can leave ordinary commands alone.
pub(crate) fn split_directive(text: &str) -> Option<(&'static str, &str)> {
    let trimmed = text.trim_start();
    let (head, rest) = match trimmed.find(char::is_whitespace) {
        Some(index) => (&trimmed[..index], trimmed[index..].trim_start()),
        None => (trimmed, ""),
    };
    // `set-pipe=off` and `set-pipe = off` both work.
    let (word, inline) = match head.split_once('=') {
        Some((word, value)) => (word, Some(value)),
        None => (head, None),
    };
    let name = match word.to_ascii_lowercase().as_str() {
        "set-pipe" => "set-pipe",
        "set-showexc" => "set-showexc",
        _ => return None,
    };
    let value = match inline {
        Some(value) => value.trim(),
        None => rest.trim_start().strip_prefix('=').unwrap_or("").trim(),
    };
    Some((name, value))
}

/// Scan a script body for directives. The last occurrence of each wins, and an
/// unrecognised value is ignored here — `scr check` reports it.
pub(crate) fn directives(content: &str) -> Directives {
    let mut out = Directives::default();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }
        let Some((name, value)) = split_directive(line) else {
            continue;
        };
        let Some(flag) = parse_bool(value) else {
            continue;
        };
        match name {
            "set-pipe" => out.pipe = flag,
            "set-showexc" => out.show_exec = flag,
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_allow_piping_and_show_the_trace() {
        let d = Directives::default();
        assert!(d.pipe);
        assert!(d.show_exec);
    }

    #[test]
    fn every_spelling_of_no_is_false() {
        for value in ["0", "false", "fail", "off", "no", "n", "FALSE", " Off "] {
            assert_eq!(parse_bool(value), Some(false), "{value}");
        }
    }

    #[test]
    fn every_spelling_of_yes_is_true() {
        for value in ["1", "true", "on", "yes", "y", "TRUE", " On "] {
            assert_eq!(parse_bool(value), Some(true), "{value}");
        }
    }

    #[test]
    fn quotes_and_junk() {
        assert_eq!(parse_bool("\"off\""), Some(false));
        assert_eq!(parse_bool("'on'"), Some(true));
        assert_eq!(parse_bool("maybe"), None);
        assert_eq!(parse_bool(""), None);
    }

    #[test]
    fn directives_are_read_from_the_file() {
        let d = directives("set-pipe = fail\nset-showexc = 0\nsay hi\n");
        assert!(!d.pipe);
        assert!(!d.show_exec);

        let d = directives("# comment\nset-showexc = on\n");
        assert!(d.pipe);
        assert!(d.show_exec);
    }

    #[test]
    fn the_last_value_wins() {
        let d = directives("set-pipe = 0\nset-pipe = 1\n");
        assert!(d.pipe);
    }

    #[test]
    fn a_bad_value_leaves_the_default() {
        let d = directives("set-pipe = maybe\nset-showexc = 2\n");
        assert!(d.pipe);
        assert!(d.show_exec);
    }

    #[test]
    fn ordinary_set_is_not_a_directive() {
        assert!(split_directive("set name = world").is_none());
        assert!(split_directive("set-pipe = off").is_some());
        assert!(split_directive("set-showexc=1").is_some());
    }

    #[test]
    fn missing_value_is_an_empty_value() {
        let (name, value) = split_directive("set-pipe").unwrap();
        assert_eq!(name, "set-pipe");
        assert_eq!(value, "");
        assert_eq!(parse_bool(value), None);
    }
}
