//! NtcScr lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"#")
    .with_annotation_at()
    .keywords(is_keyword);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        // block and statement keywords
        "set"
            | "if"
            | "else"
            | "while"
            | "for"
            | "loop"
            | "func"
            | "function"
            | "fn"
            | "call"
            | "return"
            | "input"
            | "break"
            | "continue"
            | "exit"
            | "end"
            | "in"
            // per-file directives (matched whole, hyphen included)
            | "set-pipe"
            | "set-showexc"
            // `scr` subcommands, kept so `scr ls` reads well
            | "init"
            | "list"
            | "ls"
            | "help"
            | "run"
    )
}
