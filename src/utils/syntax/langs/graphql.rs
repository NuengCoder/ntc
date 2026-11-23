//! Graphql lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"#")
    .keywords(is_keyword)
    .type_names(is_type_name)
    .builtins(is_builtin)
    .constants(is_constant);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "query"
            | "mutation"
            | "subscription"
            | "fragment"
            | "on"
            | "true"
            | "false"
            | "null"
            | "schema"
            | "type"
            | "interface"
            | "union"
            | "enum"
            | "input"
            | "scalar"
            | "directive"
            | "implements"
            | "extend"
            | "repeatable"
    )
}

pub(super) fn is_type_name(word: &str) -> bool {
    matches!(word, "Int" | "Float" | "String" | "Boolean" | "ID")
}

pub(super) fn is_builtin(word: &str) -> bool {
    matches!(
        word,
        "Query"
            | "Mutation"
            | "Subscription"
            | "__schema"
            | "__type"
            | "__typename"
            | "__Type"
            | "__Field"
            | "__InputValue"
            | "__EnumValue"
            | "__Directive"
            | "include"
            | "skip"
            | "deprecated"
            | "specifiedBy"
    )
}

pub(super) fn is_constant(word: &str) -> bool {
    matches!(word, "true" | "false" | "null")
}
