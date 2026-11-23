//! Kotlin lexer traits and word tables.

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
            | "annotation"
            | "as"
            | "break"
            | "by"
            | "catch"
            | "class"
            | "companion"
            | "const"
            | "continue"
            | "crossinline"
            | "data"
            | "do"
            | "else"
            | "enum"
            | "false"
            | "final"
            | "finally"
            | "for"
            | "fun"
            | "if"
            | "import"
            | "in"
            | "inline"
            | "inner"
            | "interface"
            | "is"
            | "lateinit"
            | "noinline"
            | "null"
            | "object"
            | "open"
            | "operator"
            | "out"
            | "override"
            | "package"
            | "private"
            | "protected"
            | "public"
            | "reified"
            | "return"
            | "sealed"
            | "super"
            | "suspend"
            | "tailrec"
            | "this"
            | "throw"
            | "true"
            | "try"
            | "typealias"
            | "val"
            | "var"
            | "vararg"
            | "when"
            | "while"
            | "context"
            | "value"
    )
}
