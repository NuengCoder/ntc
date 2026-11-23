//! NtcIgcare lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"#")
    .with_toml_fields()
    .keywords(is_keyword)
    .builtins(is_builtin);

pub(super) fn is_keyword(word: &str) -> bool {
    {
        matches!(word, "true" | "false")
    }
}

pub(super) fn is_builtin(word: &str) -> bool {
    matches!(
        word,
        "ignored_directory_names"
            | "ignored_extensions"
            | "extra_supported_extensions"
            | "ignored_files"
            | "extra_supported_files"
    )
}
