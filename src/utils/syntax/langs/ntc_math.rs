//! NtcMath lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"#")
    .keywords(is_keyword)
    .builtins(is_builtin)
    .constants(is_constant);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(word, "return" | "true" | "false")
}

pub(super) fn is_builtin(word: &str) -> bool {
    matches!(
        word,
        "sin"
            | "cos"
            | "tan"
            | "cot"
            | "sec"
            | "csc"
            | "asin"
            | "acos"
            | "atan"
            | "acot"
            | "asec"
            | "acsc"
            | "arcsin"
            | "arccos"
            | "arctan"
            | "arccot"
            | "arcsec"
            | "arccsc"
            | "sqrt"
            | "pow"
            | "abs"
            | "floor"
            | "ceil"
            | "ceiling"
            | "round"
            | "ln"
            | "log"
            | "log2"
            | "log10"
            | "sum"
            | "min"
            | "max"
            | "avg"
            | "average"
            | "mean"
            | "rand"
            | "random"
            | "tobinary"
            | "tohex"
            | "to8"
            | "tooctal"
            | "todecimal"
            | "tohb"
            | "tohumanbytes"
            | "tohumanreadable"
    )
}

pub(super) fn is_constant(word: &str) -> bool {
    matches!(
        word,
        "PI" | "pi" | "E" | "e" | "EXP" | "exp" | "PHI" | "phi" | "TAU" | "tau"
    )
}
