//! Java lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"//")
    .with_block_comments()
    .with_char_literals()
    .with_annotation_at()
    .with_upper_heuristic()
    .keywords(is_keyword)
    .type_names(is_type_name);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "abstract"
            | "assert"
            | "boolean"
            | "break"
            | "byte"
            | "case"
            | "catch"
            | "char"
            | "class"
            | "const"
            | "continue"
            | "default"
            | "do"
            | "double"
            | "else"
            | "enum"
            | "extends"
            | "false"
            | "final"
            | "finally"
            | "float"
            | "for"
            | "goto"
            | "if"
            | "implements"
            | "import"
            | "instanceof"
            | "int"
            | "interface"
            | "long"
            | "native"
            | "new"
            | "null"
            | "package"
            | "private"
            | "protected"
            | "public"
            | "return"
            | "short"
            | "static"
            | "strictfp"
            | "super"
            | "switch"
            | "synchronized"
            | "this"
            | "throw"
            | "throws"
            | "transient"
            | "true"
            | "try"
            | "void"
            | "volatile"
            | "while"
            | "var"
            | "record"
            | "sealed"
            | "permits"
            | "yield"
            | "non-sealed"
    )
}

pub(super) fn is_type_name(word: &str) -> bool {
    matches!(
        word,
        "byte"
            | "short"
            | "int"
            | "long"
            | "float"
            | "double"
            | "boolean"
            | "char"
            | "String"
            | "Object"
            | "Class"
            | "Throwable"
            | "Exception"
            | "Error"
            | "RuntimeException"
            | "Iterable"
            | "Comparable"
            | "Serializable"
            | "Runnable"
            | "Callable"
            | "List"
            | "Set"
            | "Map"
            | "Collection"
            | "ArrayList"
            | "LinkedList"
            | "HashMap"
            | "LinkedHashMap"
            | "HashSet"
            | "LinkedHashSet"
            | "Optional"
            | "Stream"
            | "Record"
            | "var"
    )
}
