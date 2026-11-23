//! Haskell lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"--")
    .with_upper_heuristic()
    .keywords(is_keyword);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "case"
            | "class"
            | "data"
            | "default"
            | "deriving"
            | "do"
            | "else"
            | "family"
            | "forall"
            | "foreign"
            | "hiding"
            | "if"
            | "import"
            | "in"
            | "infix"
            | "infixl"
            | "infixr"
            | "instance"
            | "let"
            | "mdo"
            | "module"
            | "newtype"
            | "of"
            | "packed"
            | "proc"
            | "qualified"
            | "rec"
            | "then"
            | "type"
            | "where"
    )
}
