//! R lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"#").keywords(is_keyword);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "break"
            | "else"
            | "FALSE"
            | "for"
            | "function"
            | "if"
            | "Inf"
            | "in"
            | "NA"
            | "NaN"
            | "next"
            | "NULL"
            | "repeat"
            | "return"
            | "TRUE"
            | "while"
    )
}
