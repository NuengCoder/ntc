//! Perl lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"#").keywords(is_keyword);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "__END__"
            | "__FILE__"
            | "__LINE__"
            | "abs"
            | "and"
            | "caller"
            | "cmp"
            | "continue"
            | "croak"
            | "die"
            | "do"
            | "else"
            | "elsif"
            | "eval"
            | "exit"
            | "for"
            | "foreach"
            | "given"
            | "goto"
            | "grep"
            | "if"
            | "last"
            | "local"
            | "map"
            | "my"
            | "next"
            | "no"
            | "or"
            | "our"
            | "package"
            | "print"
            | "printf"
            | "redo"
            | "return"
            | "say"
            | "sort"
            | "state"
            | "sub"
            | "unless"
            | "until"
            | "use"
            | "wantarray"
            | "when"
            | "while"
            | "xor"
            | "y"
    )
}
