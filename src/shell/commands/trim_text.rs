// src/shell/commands/trim_text.rs
//! `trim <pattern> [text]` — remove every occurrence of a pattern.
//!
//!   trim " ..." "nice ... bro ..."       -> nice bro
//!   echo nice ... bro ... | trim " ..."  -> nice bro
//!
//! Inline form: the first token is the pattern and the remaining tokens
//! (joined with one space) are the text. When input is piped/redirected,
//! every token is a pattern and the text comes from stdin. Patterns are
//! plain substrings (case-sensitive, removed non-overlapping). A trailing
//! CR/LF is stripped and one newline is printed, so both forms produce the
//! same line.

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::shell::stdin;
use crate::t;
use anyhow::Result;

/// Quote-aware whitespace split (double quotes only, like the other
/// shell arg parsers) so a multi-word pattern or text stays one token.
fn tokenize(args: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for ch in args.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            c if c.is_whitespace() && !in_quotes => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            _ => cur.push(ch),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// Remove every non-overlapping occurrence of each pattern, in order.
pub(crate) fn apply_trim(text: &str, patterns: &[String]) -> String {
    let mut out = text.to_string();
    for pattern in patterns {
        if pattern.is_empty() {
            continue;
        }
        out = out.replace(pattern.as_str(), "");
    }
    out
}

/// Decide how the tokens split between patterns and text.
///
/// * `explicit_input` — a pipeline/redirect buffer is installed before the
///   command runs, so *every* token is a pattern and the text comes from
///   stdin (`cmd | trim "a" "b"`).
/// * `lazy_input` — process stdin is piped (e.g. `echo x | ntc trim "a"`);
///   with a single token that is the pattern and stdin is the text.
///
/// Otherwise (inline form) the first token is the pattern and the remaining
/// tokens joined with one space are the text.
fn split_args(
    tokens: &[String],
    explicit_input: bool,
    lazy_input: bool,
) -> Option<(Vec<String>, Option<String>)> {
    let piped = explicit_input || (tokens.len() == 1 && lazy_input);
    if piped {
        Some((tokens.to_vec(), None))
    } else if tokens.len() >= 2 {
        Some((vec![tokens[0].clone()], Some(tokens[1..].join(" "))))
    } else {
        None
    }
}

pub(crate) fn cmd_trim(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() {
        print_error(&t!("misc.trim.usage"));
        return Ok(false);
    }

    let Some((patterns, inline_text)) =
        split_args(&tokens, stdin::has_input(), stdin::has_any_input())
    else {
        print_error(&t!("misc.trim.usage"));
        return Ok(false);
    };
    let text = match inline_text {
        Some(text) => text,
        None => match stdin::read_to_string() {
            Some(text) => text,
            None => {
                print_error(&t!("misc.trim.usage"));
                return Ok(false);
            }
        },
    };

    if patterns.iter().any(|p| p.is_empty()) {
        print_error(&t!("misc.trim.empty_pattern"));
        return Ok(false);
    }

    let trimmed = apply_trim(&text, &patterns);
    let trimmed = trimmed.trim_end_matches(['\r', '\n']);
    println!("{trimmed}");
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pats(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn removes_every_occurrence() {
        assert_eq!(apply_trim("nice ... bro ...", &pats(&[" ..."])), "nice bro");
    }

    #[test]
    fn inline_and_piped_forms_match() {
        let inline = apply_trim("nice ... bro ...", &pats(&[" ..."]));
        let piped = apply_trim("nice ... bro ...\r\n", &pats(&[" ..."]));
        assert_eq!(inline, "nice bro");
        assert_eq!(piped.trim_end_matches(['\r', '\n']), inline);
    }

    #[test]
    fn applies_multiple_patterns() {
        assert_eq!(
            apply_trim("a foo b bar c", &pats(&["foo", "bar"])),
            "a  b  c"
        );
    }

    #[test]
    fn is_case_sensitive_and_non_overlapping() {
        assert_eq!(apply_trim("Foo foo", &pats(&["foo"])), "Foo ");
        assert_eq!(apply_trim("aaaa", &pats(&["aa"])), "");
    }

    #[test]
    fn empty_patterns_are_ignored() {
        assert_eq!(apply_trim("keep me", &pats(&[""])), "keep me");
    }

    #[test]
    fn tokenizes_quoted_text() {
        assert_eq!(
            tokenize("\" ...\" \"nice ... bro ...\""),
            vec![" ...".to_string(), "nice ... bro ...".to_string()]
        );
    }

    #[test]
    fn inline_form_needs_text_and_splits_first_token() {
        assert!(split_args(&tokenize(""), false, false).is_none());
        assert!(split_args(&tokenize("\" ...\""), false, false).is_none());
        assert_eq!(
            split_args(&tokenize("\" ...\" \"nice ... bro ...\""), false, false),
            Some((pats(&[" ..."]), Some("nice ... bro ...".to_string())))
        );
    }

    #[test]
    fn piped_form_treats_all_tokens_as_patterns() {
        assert_eq!(
            split_args(&tokenize("\" ...\""), false, true),
            Some((pats(&[" ..."]), None))
        );
        assert_eq!(
            split_args(&tokenize("foo bar"), true, false),
            Some((pats(&["foo", "bar"]), None))
        );
    }

    #[test]
    fn inline_text_wins_over_lazy_stdin_with_two_tokens() {
        // `ntc trim "a" "text"` still works when process stdin is not a TTY.
        assert_eq!(
            split_args(&tokenize("a \"some text\""), false, true),
            Some((pats(&["a"]), Some("some text".to_string())))
        );
    }
}
