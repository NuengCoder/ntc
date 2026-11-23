//! C lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"//")
    .with_block_comments()
    .with_char_literals()
    .keywords(is_keyword)
    .type_names(is_type_name);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "auto"
            | "break"
            | "case"
            | "char"
            | "const"
            | "continue"
            | "default"
            | "do"
            | "double"
            | "else"
            | "enum"
            | "extern"
            | "float"
            | "for"
            | "goto"
            | "if"
            | "inline"
            | "int"
            | "long"
            | "register"
            | "return"
            | "short"
            | "signed"
            | "sizeof"
            | "static"
            | "struct"
            | "switch"
            | "typedef"
            | "union"
            | "unsigned"
            | "void"
            | "volatile"
            | "while"
            | "_Bool"
            | "_Complex"
            | "_Imaginary"
            | "restrict"
            | "_Atomic"
            | "_Thread_local"
            | "_Noreturn"
            | "_Static_assert"
            | "_Alignas"
            | "_Alignof"
            | "_Generic"
    )
}

pub(super) fn is_type_name(word: &str) -> bool {
    matches!(
        word,
        "size_t"
            | "ssize_t"
            | "int8_t"
            | "int16_t"
            | "int32_t"
            | "int64_t"
            | "uint8_t"
            | "uint16_t"
            | "uint32_t"
            | "uint64_t"
            | "intptr_t"
            | "uintptr_t"
            | "ptrdiff_t"
            | "wchar_t"
            | "FILE"
            | "va_list"
    )
}
