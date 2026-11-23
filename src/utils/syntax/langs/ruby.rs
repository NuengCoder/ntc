//! Ruby lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"#")
    .keywords(is_keyword)
    .builtins(is_builtin);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "BEGIN"
            | "END"
            | "alias"
            | "and"
            | "begin"
            | "break"
            | "case"
            | "class"
            | "def"
            | "defined?"
            | "do"
            | "else"
            | "elsif"
            | "end"
            | "ensure"
            | "false"
            | "for"
            | "if"
            | "in"
            | "module"
            | "next"
            | "nil"
            | "not"
            | "or"
            | "redo"
            | "rescue"
            | "retry"
            | "return"
            | "self"
            | "super"
            | "then"
            | "true"
            | "undef"
            | "unless"
            | "until"
            | "when"
            | "while"
            | "yield"
    )
}

pub(super) fn is_builtin(word: &str) -> bool {
    matches!(
        word,
        "puts"
            | "print"
            | "p"
            | "require"
            | "include"
            | "extend"
            | "attr_accessor"
            | "attr_reader"
            | "attr_writer"
            | "raise"
            | "fail"
            | "catch"
            | "throw"
            | "lambda"
            | "proc"
            | "block_given?"
            | "iterator?"
            | "loop"
            | "sleep"
            | "system"
            | "exec"
            | "spawn"
            | "fork"
            | "trap"
            | "Signal"
            | "trace"
            | "warn"
            | "abort"
            | "exit"
            | "at_exit"
            | "load"
            | "autoload"
            | "private"
            | "protected"
            | "public"
            | "module_function"
    )
}
