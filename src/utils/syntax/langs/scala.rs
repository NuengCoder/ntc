//! Scala lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"//")
    .with_block_comments()
    .with_annotation_at()
    .with_upper_heuristic()
    .keywords(is_keyword);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "abstract"
            | "case"
            | "catch"
            | "class"
            | "def"
            | "do"
            | "else"
            | "enum"
            | "export"
            | "extends"
            | "false"
            | "final"
            | "finally"
            | "for"
            | "forSome"
            | "given"
            | "if"
            | "implicit"
            | "import"
            | "lazy"
            | "match"
            | "new"
            | "null"
            | "object"
            | "override"
            | "package"
            | "private"
            | "protected"
            | "return"
            | "sealed"
            | "super"
            | "then"
            | "throw"
            | "trait"
            | "true"
            | "try"
            | "type"
            | "using"
            | "val"
            | "var"
            | "while"
            | "with"
            | "yield"
    )
}
