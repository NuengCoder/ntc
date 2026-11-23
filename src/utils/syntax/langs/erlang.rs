//! Erlang lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"%")
    .keywords(is_keyword)
    .type_names(is_type_name);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "after"
            | "and"
            | "andalso"
            | "band"
            | "begin"
            | "bnot"
            | "bor"
            | "bsl"
            | "bsr"
            | "bxor"
            | "case"
            | "catch"
            | "cond"
            | "div"
            | "end"
            | "fun"
            | "if"
            | "let"
            | "not"
            | "of"
            | "or"
            | "orelse"
            | "receive"
            | "rem"
            | "try"
            | "when"
            | "xor"
    )
}

pub(super) fn is_type_name(word: &str) -> bool {
    matches!(
        word,
        "integer"
            | "float"
            | "boolean"
            | "atom"
            | "string"
            | "list"
            | "tuple"
            | "map"
            | "binary"
            | "function"
            | "pid"
            | "port"
            | "reference"
            | "any"
            | "dynamic"
    )
}
