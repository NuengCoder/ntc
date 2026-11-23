//! Bash lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"#").keywords(is_keyword);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "case"
            | "do"
            | "done"
            | "elif"
            | "else"
            | "esac"
            | "fi"
            | "for"
            | "function"
            | "if"
            | "in"
            | "select"
            | "then"
            | "time"
            | "until"
            | "while"
    )
}
