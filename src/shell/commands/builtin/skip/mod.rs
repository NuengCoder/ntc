//! `skip` — inverse of `head`: drop lines from the front of the input.
//!
//!   skip [N] [file]          drop the first N lines (default 10)
//!   skip -x <lines> [file]   drop the listed 1-based line numbers
//!
//! `skip 2` keeps everything after the first two lines, while
//! `skip -x 2,5,8` keeps everything except lines 2, 5 and 8. Input is the
//! named file (which wins) or piped / redirected stdin, exactly like `head`
//! and `tail`, so the command composes as a pipeline filter.

pub(crate) mod help;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::text::{read_source, tokenize};
use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use anyhow::Result;
use std::collections::BTreeSet;

/// Default drop count for a bare `skip` (the inverse of `head`'s default).
const DEFAULT_COUNT: usize = 10;

/// Which lines a `skip` invocation drops.
#[derive(Debug, PartialEq, Eq)]
enum Mode {
    /// Drop the first N lines.
    First(usize),
    /// Drop exactly these 1-based line numbers.
    Only(Vec<usize>),
}

#[derive(Debug, PartialEq, Eq)]
struct Parsed {
    mode: Mode,
    file: Option<String>,
}

/// Why parsing failed; the caller turns each into a localized message.
#[derive(Debug, PartialEq, Eq)]
enum ParseError {
    /// `-x` was given without any line number.
    Usage,
    /// Unexpected or malformed token.
    BadArg(String),
}

/// Parse `[N] [file]` or `-x <lines> [file]`. The `-x` list accepts commas,
/// spaces or both (`2,5,8`, `2 5 8`, `2 , 5 , 8`); the first non-numeric
/// token names the file, and a second bare number is an error rather than a
/// silently ignored argument.
fn parse(args: &str) -> std::result::Result<Parsed, ParseError> {
    let mut exclude = false;
    let mut count: Option<usize> = None;
    let mut only: Vec<usize> = Vec::new();
    let mut file: Option<String> = None;

    for tok in tokenize(args) {
        match tok.as_str() {
            "-x" | "--exclude" => {
                if exclude {
                    return Err(ParseError::BadArg(tok));
                }
                exclude = true;
            }
            // A lone comma is a list separator, so `2 , 5 , 8` works.
            "," => {}
            t if t.starts_with('-') && t.len() > 1 => return Err(ParseError::BadArg(tok)),
            _ if exclude => {
                // A comma-separated token is always a list, so a typo inside
                // it (`2,abc,3`) is an error instead of becoming a filename.
                let is_list = tok.contains(',');
                for part in tok.split(',') {
                    if part.is_empty() {
                        continue;
                    }
                    match part.parse::<usize>() {
                        Ok(n) if n >= 1 => only.push(n),
                        Ok(_) => return Err(ParseError::BadArg(part.to_string())),
                        Err(_) if !is_list && file.is_none() => file = Some(part.to_string()),
                        Err(_) => return Err(ParseError::BadArg(part.to_string())),
                    }
                }
            }
            _ => {
                // Only one optional file is accepted, before or after the count.
                if file.is_some() {
                    return Err(ParseError::BadArg(tok));
                }
                match count {
                    None => match tok.parse::<usize>() {
                        Ok(n) => count = Some(n),
                        Err(_) => file = Some(tok),
                    },
                    Some(_) => {
                        // A second bare number is ambiguous, not a file.
                        if tok.parse::<usize>().is_ok() {
                            return Err(ParseError::BadArg(tok));
                        }
                        file = Some(tok);
                    }
                }
            }
        }
    }

    if exclude {
        if only.is_empty() {
            return Err(ParseError::Usage);
        }
        only.sort_unstable();
        only.dedup();
        return Ok(Parsed {
            mode: Mode::Only(only),
            file,
        });
    }

    Ok(Parsed {
        mode: Mode::First(count.unwrap_or(DEFAULT_COUNT)),
        file,
    })
}

/// Render the surviving lines, each with a trailing newline (the same shape
/// `head` / `tail` print, so the output composes).
fn filter(lines: &[&str], mode: &Mode) -> String {
    let listed: BTreeSet<usize> = match mode {
        Mode::Only(nums) => nums.iter().copied().collect(),
        Mode::First(_) => BTreeSet::new(),
    };
    let mut out = String::new();
    for (idx, line) in lines.iter().enumerate() {
        let dropped = match mode {
            Mode::First(n) => idx < *n,
            Mode::Only(_) => listed.contains(&(idx + 1)),
        };
        if !dropped {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

pub(crate) fn cmd_skip(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parsed = match parse(args) {
        Ok(p) => p,
        Err(ParseError::Usage) => {
            print_error(&t!("misc.skip.usage"));
            return Ok(false);
        }
        Err(ParseError::BadArg(bad)) => {
            print_error(&t!("misc.window.bad_arg", "skip", bad));
            println!("{}", t!("misc.skip.usage"));
            return Ok(false);
        }
    };

    let text = match read_source(&parsed.file) {
        Some(text) => text,
        None => {
            if parsed.file.is_none() {
                print_error(&t!("misc.window.no_input", "skip"));
                println!("{}", t!("misc.skip.usage"));
            }
            return Ok(false);
        }
    };

    let lines: Vec<&str> = text.lines().collect();
    print!("{}", filter(&lines, &parsed.mode));
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "skip",
    aliases: &[],
    help_topic: Some("skip"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: false,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_skip),
};

#[cfg(test)]
mod tests {
    use super::*;

    fn parsed(args: &str) -> Parsed {
        parse(args).expect("expected parse to succeed")
    }

    #[test]
    fn bare_skip_drops_the_first_ten_lines() {
        assert_eq!(
            parsed(""),
            Parsed {
                mode: Mode::First(DEFAULT_COUNT),
                file: None,
            }
        );
        assert_eq!(parsed("0").mode, Mode::First(0));
        assert_eq!(parsed("2").mode, Mode::First(2));
    }

    #[test]
    fn skip_takes_one_count_then_a_file() {
        assert_eq!(
            parsed("3 app.log"),
            Parsed {
                mode: Mode::First(3),
                file: Some("app.log".to_string()),
            }
        );
        assert_eq!(
            parsed("app.log"),
            Parsed {
                mode: Mode::First(DEFAULT_COUNT),
                file: Some("app.log".to_string()),
            }
        );
        assert_eq!(
            parsed("5 \"my file.txt\""),
            Parsed {
                mode: Mode::First(5),
                file: Some("my file.txt".to_string()),
            }
        );
    }

    #[test]
    fn exclude_list_accepts_commas_spaces_and_both() {
        assert_eq!(parsed("-x 2,5,8").mode, Mode::Only(vec![2, 5, 8]));
        assert_eq!(parsed("-x 2 5 8").mode, Mode::Only(vec![2, 5, 8]));
        assert_eq!(parsed("-x 2 , 5 , 8").mode, Mode::Only(vec![2, 5, 8]));
        assert_eq!(parsed("--exclude 5,2,5,8").mode, Mode::Only(vec![2, 5, 8]));
        assert_eq!(
            parsed("-x 2 5 app.log"),
            Parsed {
                mode: Mode::Only(vec![2, 5]),
                file: Some("app.log".to_string()),
            }
        );
        assert_eq!(
            parsed("-x 2,5 app.log"),
            Parsed {
                mode: Mode::Only(vec![2, 5]),
                file: Some("app.log".to_string()),
            }
        );
        // The flag may follow the file too.
        assert_eq!(
            parsed("app.log -x 3"),
            Parsed {
                mode: Mode::Only(vec![3]),
                file: Some("app.log".to_string()),
            }
        );
    }

    #[test]
    fn bad_arguments_are_rejected() {
        assert_eq!(parse("-x"), Err(ParseError::Usage));
        assert_eq!(parse("-x 0"), Err(ParseError::BadArg("0".into())));
        assert_eq!(parse("2 5"), Err(ParseError::BadArg("5".into())));
        assert_eq!(
            parse("a.txt b.txt"),
            Err(ParseError::BadArg("b.txt".into()))
        );
        assert_eq!(parse("-X 2"), Err(ParseError::BadArg("-X".into())));
        assert_eq!(parse("-x 2 -x 3"), Err(ParseError::BadArg("-x".into())));
        assert_eq!(parse("-x 2,abc,3"), Err(ParseError::BadArg("abc".into())));
    }

    #[test]
    fn filters_drop_the_requested_lines() {
        let lines = ["a", "b", "c", "d", "e"];
        assert_eq!(filter(&lines, &Mode::First(2)), "c\nd\ne\n");
        assert_eq!(filter(&lines, &Mode::First(0)), "a\nb\nc\nd\ne\n");
        assert_eq!(filter(&lines, &Mode::First(9)), "");
        assert_eq!(filter(&lines, &Mode::Only(vec![2, 5])), "a\nc\nd\n");
        // Out-of-range line numbers are simply never matched.
        assert_eq!(filter(&lines, &Mode::Only(vec![99])), "a\nb\nc\nd\ne\n");
        assert_eq!(filter(&[], &Mode::First(1)), "");
    }
}
