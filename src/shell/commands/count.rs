// src/shell/commands/count.rs
//! `count <needle> [text]` — count how many times `needle` occurs.
//!
//!   count 'the target' 'the target is so nice and the target is so good'  -> 2
//!   gs error | count error                                               -> piped input
//!
//! Output is just the number so the command composes in pipelines.

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::shell::stdin;
use crate::t;
use anyhow::Result;

/// Quote-aware whitespace split (double quotes only, like the other
/// shell arg parsers) so a multi-word needle or text stays one token.
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

/// Non-overlapping occurrences of `needle` in `haystack`.
pub(crate) fn count_occurrences(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut rest = haystack;
    while let Some(idx) = rest.find(needle) {
        count += 1;
        rest = &rest[idx + needle.len()..];
    }
    count
}

pub(crate) fn cmd_count(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    let Some(needle) = tokens.first() else {
        print_error(&t!("misc.count.usage"));
        return Ok(false);
    };
    let has_text = tokens.len() > 1;
    let haystack = if has_text {
        tokens[1..].join(" ")
    } else {
        match stdin::read_to_string() {
            Some(text) => text,
            None => {
                print_error(&t!("misc.count.usage"));
                return Ok(false);
            }
        }
    };
    println!("{}", count_occurrences(&haystack, needle));
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_non_overlapping_occurrences() {
        assert_eq!(
            count_occurrences(
                "the target is so nice and the target is so good",
                "the target"
            ),
            2
        );
        assert_eq!(count_occurrences("aaaa", "aa"), 2);
        assert_eq!(count_occurrences("aaaa", "a"), 4);
        assert_eq!(count_occurrences("abc", "z"), 0);
        assert_eq!(count_occurrences("abc", ""), 0);
    }

    #[test]
    fn tokenizes_quoted_text() {
        let toks = tokenize("\"the target\" \"the target is good\"");
        assert_eq!(
            toks,
            vec!["the target".to_string(), "the target is good".to_string()]
        );
    }
}
