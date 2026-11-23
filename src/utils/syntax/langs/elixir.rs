//! Elixir lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"--").keywords(is_keyword);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "after"
            | "and"
            | "case"
            | "catch"
            | "cond"
            | "def"
            | "defdelegate"
            | "defexception"
            | "defguard"
            | "defimpl"
            | "defmacro"
            | "defmodule"
            | "defn"
            | "defp"
            | "defprotocol"
            | "defstruct"
            | "deftype"
            | "do"
            | "else"
            | "end"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "in"
            | "nil"
            | "not"
            | "or"
            | "quote"
            | "raise"
            | "receive"
            | "require"
            | "rescue"
            | "return"
            | "send"
            | "super"
            | "throw"
            | "true"
            | "try"
            | "unquote"
            | "unless"
            | "unquote_splicing"
            | "when"
            | "with"
    )
}
