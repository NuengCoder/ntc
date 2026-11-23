use crate::utils::syntax::langs;
use crate::utils::syntax::types::SyntaxLanguage;

pub(super) fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

pub(super) fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

pub(super) fn line_comment_prefix(lang: SyntaxLanguage) -> &'static [u8] {
    langs::spec(lang).line_comment
}

pub(super) fn has_block_comments(lang: SyntaxLanguage) -> bool {
    langs::spec(lang).block_comments
}

pub(super) fn has_xml_comments(lang: SyntaxLanguage) -> bool {
    langs::spec(lang).xml_comments
}

pub(super) fn has_ps_block_comments(lang: SyntaxLanguage) -> bool {
    langs::spec(lang).ps_block_comments
}

pub(super) fn has_fsharp_block_comments(lang: SyntaxLanguage) -> bool {
    langs::spec(lang).fsharp_block_comments
}

pub(super) fn has_pascal_comments(lang: SyntaxLanguage) -> bool {
    langs::spec(lang).pascal_comments
}

pub(super) fn has_preprocessor(lang: SyntaxLanguage) -> bool {
    langs::spec(lang).preprocessor
}

pub(super) fn is_char_literal_supported(lang: SyntaxLanguage) -> bool {
    langs::spec(lang).char_literals
}

pub(super) fn has_regex_literals(lang: SyntaxLanguage) -> bool {
    langs::spec(lang).regex_literals
}

pub(super) fn has_tag_syntax(lang: SyntaxLanguage) -> bool {
    langs::spec(lang).tag_syntax
}

pub(super) fn is_rust_attr_start(lang: SyntaxLanguage) -> bool {
    lang == SyntaxLanguage::Rust
}

pub(super) fn has_annotation_at(lang: SyntaxLanguage) -> bool {
    langs::spec(lang).annotation_at
}

pub(super) fn has_css_at_rules(lang: SyntaxLanguage) -> bool {
    langs::spec(lang).css_at_rules
}

pub(super) fn is_css_at_rule(word: &str) -> bool {
    (langs::spec(SyntaxLanguage::Css).at_rule)(word)
}

pub(super) fn is_css_property(word: &str) -> bool {
    (langs::spec(SyntaxLanguage::Css).css_property)(word)
}

pub(super) fn is_keyword(lang: SyntaxLanguage, word: &str) -> bool {
    (langs::spec(lang).keyword)(word)
}

pub(super) fn is_type_name(lang: SyntaxLanguage, word: &str) -> bool {
    match langs::spec(lang).type_name {
        Some(table) => table(word),
        // Languages with no type table keep the legacy rule: a PascalCase
        // identifier reads as a type. All-caps (`JSON`) and one-letter words
        // stay normal so constants and builtins keep their own colour.
        None => {
            let mut chars = word.chars();
            matches!(chars.next(), Some(c) if c.is_uppercase())
                && !chars.all(|c| c.is_uppercase() || c == '_')
        }
    }
}

pub(super) fn is_type_name_upper_heuristic(lang: SyntaxLanguage) -> bool {
    langs::spec(lang).upper_heuristic
}

pub(super) fn is_builtin(lang: SyntaxLanguage, word: &str) -> bool {
    match langs::spec(lang).builtin {
        Some(table) => table(word),
        // Shared literals for languages without a builtin table.
        None => matches!(
            word,
            "self" | "Self" | "this" | "super" | "null" | "nil" | "true" | "false" | "None"
        ),
    }
}

pub(super) fn is_function_call(_word: &str, next_byte: Option<u8>) -> bool {
    next_byte == Some(b'(')
}

pub(super) fn has_toml_fields(lang: SyntaxLanguage) -> bool {
    langs::spec(lang).toml_fields
}

pub(super) fn is_constant(lang: SyntaxLanguage, word: &str) -> bool {
    (langs::spec(lang).constant)(word)
}
