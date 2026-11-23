//! GoMod lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"//")
    .with_block_comments()
    .keywords(is_keyword);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "module"
            | "go"
            | "replace"
            | "exclude"
            | "retract"
            | "require"
            | "toolchain"
            | "tool"
            | "godebug"
            | "ignore"
    )
}
