//! Bat lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"::").keywords(is_keyword);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "echo"
            | "set"
            | "if"
            | "else"
            | "for"
            | "in"
            | "do"
            | "goto"
            | "call"
            | "exit"
            | "rem"
            | "pause"
            | "shift"
            | "choice"
            | "type"
            | "copy"
            | "move"
            | "del"
            | "ren"
            | "mkdir"
            | "rmdir"
            | "cd"
            | "pushd"
            | "popd"
            | "setlocal"
            | "endlocal"
            | "title"
            | "color"
            | "prompt"
            | "path"
            | "assoc"
            | "ftype"
            | "ver"
            | "vol"
            | "label"
            | "defined"
            | "exist"
            | "errorlevel"
            | "cmdextversion"
            | "not"
            | " EQU "
            | " NEQ "
            | " LSS "
            | " LEQ "
            | " GTR "
            | " GEQ "
    )
}
