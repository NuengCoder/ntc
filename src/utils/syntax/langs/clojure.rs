//! Clojure lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"//").keywords(is_keyword);

pub(super) fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "def"
            | "defn"
            | "defmacro"
            | "defmethod"
            | "defmulti"
            | "defonce"
            | "defprotocol"
            | "defrecord"
            | "deftype"
            | "do"
            | "doseq"
            | "dotimes"
            | "doto"
            | "fn"
            | "for"
            | "if"
            | "if-let"
            | "if-not"
            | "import"
            | "let"
            | "letfn"
            | "loop"
            | "monitor-enter"
            | "monitor-exit"
            | "ns"
            | "proxy"
            | "recur"
            | "reify"
            | "require"
            | "set!"
            | "try"
            | "when"
            | "when-first"
            | "when-let"
            | "when-not"
            | "while"
    )
}
