//! Word tables for the `.ntc` script language service.
//!
//! Everything the editor knows about the language lives here: the block and
//! statement keywords, the expression functions, and the per-file directives.
//! Shell commands are not listed — they come from the live command registry so
//! completion can never drift from what the shell actually dispatches.

/// `(word, doc)` — block and statement keywords.
pub(crate) const KEYWORDS: &[(&str, &str)] = &[
    (
        "set",
        "set NAME = value — define a variable. Plain values are kept verbatim; \
         anything with an operator is evaluated as an expression.",
    ),
    (
        "if",
        "if <cond> … else if <cond> … else … end — run the first branch whose \
         condition is true.",
    ),
    ("else", "else — the remaining branch of an if."),
    (
        "while",
        "while <cond> … end — loop while the condition holds.",
    ),
    (
        "for",
        "for VAR in <list> … end — loop over words or an expression.",
    ),
    (
        "loop",
        "loop [count] … end — repeat count times; without a count it repeats \
         until break or exit.",
    ),
    (
        "func",
        "func NAME(params) … end — define a function. Definitions are hoisted, \
         so a function can be called before the line that declares it.",
    ),
    (
        "call",
        "call NAME(args) — call a function. `set x = call f()` captures its \
         return value.",
    ),
    ("return", "return [value] — leave a function with a value."),
    (
        "input",
        "input NAME [prompt] — read one line from the terminal into a variable.",
    ),
    ("break", "break — leave the innermost loop."),
    (
        "continue",
        "continue — skip the rest of this round and start the next one.",
    ),
    ("exit", "exit — stop the script."),
    (
        "end",
        "end — close an if / while / for / loop / func block (endif, fi, done \
         and } work too).",
    ),
    (
        "set-pipe",
        "set-pipe = on|off — may this script be piped or redirected? \
         off / 0 / false / fail refuses the pipeline before the script runs.",
    ),
    (
        "set-showexc",
        "set-showexc = on|off — print the `[n] Executing: …` trace before each \
         command? Default on.",
    ),
];

/// `(name, signature, doc)` — functions usable inside an expression.
pub(crate) const FUNCTIONS: &[(&str, &str, &str)] = &[
    (
        "nprintf",
        "nprintf(format, ...)",
        "Format values like the nprintf builtin and return the text, so it can \
         be stored: `set label = nprintf(\"%03d\", $i)`.",
    ),
    ("len", "len(text)", "Number of characters in a value."),
    ("upper", "upper(text)", "Uppercase a value."),
    ("lower", "lower(text)", "Lowercase a value."),
    (
        "trim",
        "trim(text)",
        "Strip leading and trailing whitespace.",
    ),
    ("str", "str(value)", "Turn any value into text."),
    (
        "int",
        "int(value)",
        "Parse a value as an integer; reports an error when it is not a number.",
    ),
    ("abs", "abs(number)", "Absolute value of an integer."),
    (
        "env",
        "env(name)",
        "Read an environment variable (empty when it is not set).",
    ),
];

/// Documentation for a keyword or directive.
#[cfg(test)]
pub(crate) fn keyword_doc(word: &str) -> Option<&'static str> {
    let lower = word.to_ascii_lowercase();
    KEYWORDS
        .iter()
        .find(|(name, _)| *name == lower)
        .map(|(_, doc)| *doc)
}

/// Signature and documentation for an expression function.
pub(crate) fn function_doc(name: &str) -> Option<(&'static str, &'static str)> {
    let lower = name.to_ascii_lowercase();
    FUNCTIONS
        .iter()
        .find(|(func, _, _)| *func == lower)
        .map(|(_, signature, doc)| (*signature, *doc))
}

/// Parameter names parsed out of an expression function's signature.
pub(crate) fn function_params(name: &str) -> Option<Vec<String>> {
    let (signature, _) = function_doc(name)?;
    let inner = signature.split_once('(')?.1.rsplit_once(')')?.0;
    Some(
        inner
            .split(',')
            .map(|param| param.trim().to_string())
            .filter(|param| !param.is_empty())
            .collect(),
    )
}

/// One-line description of a shell command, from the compact help table.
///
/// Returns `None` for a name that is not a builtin, which is how the service
/// tells "runs a command" apart from "runs nothing".
pub(crate) fn command_doc(name: &str) -> Option<String> {
    let registry = crate::shell::commands::registry_data::registry();
    let spec = registry.resolve(name)?;
    let canonical = spec.name;
    for (entry, _label, key) in crate::shell::commands::core::help::compact_entries() {
        if entry.eq_ignore_ascii_case(canonical) {
            return Some(crate::t!(key));
        }
    }
    Some(format!("{} — ntc command", canonical))
}

/// Every command name and alias the shell dispatches, in registry order.
pub(crate) fn command_names() -> impl Iterator<Item = &'static str> {
    crate::shell::commands::registry_data::registry().names()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords_cover_the_language() {
        for word in [
            "set",
            "if",
            "else",
            "while",
            "for",
            "loop",
            "func",
            "call",
            "return",
            "input",
            "break",
            "continue",
            "exit",
            "end",
            "set-pipe",
            "set-showexc",
        ] {
            assert!(keyword_doc(word).is_some(), "{word} has no doc");
        }
        assert!(keyword_doc("IF").is_some());
        assert!(keyword_doc("nope").is_none());
    }

    #[test]
    fn expression_functions_are_documented() {
        for name in [
            "nprintf", "len", "upper", "lower", "trim", "str", "int", "abs", "env",
        ] {
            let (signature, doc) = function_doc(name).expect(name);
            assert!(signature.starts_with(name), "{signature}");
            assert!(!doc.is_empty(), "{name} has no doc");
        }
    }

    #[test]
    fn shell_commands_come_from_the_live_registry() {
        assert!(command_doc("say").is_some());
        assert!(command_doc("nprintf").is_some());
        assert!(command_doc("no_such_command_xyz").is_none());
        assert!(command_names().any(|name| name == "say"));
    }
}
