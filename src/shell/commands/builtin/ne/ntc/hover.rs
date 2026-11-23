//! Hover text for a call site in a `.ntc` script.
//!
//! The editor shows one line while the cursor is inside `name(...)`, the same
//! way it does for MQL: the active parameter of a function declared in this
//! file, or the signature of an expression function such as `nprintf(...)`.

use super::extract;
use super::lexicon;

/// Describe a function call the cursor is inside, naming the active parameter.
pub(crate) fn call_hover(name: &str, active: usize, lines: &[String]) -> Option<String> {
    for func in extract::functions(lines) {
        if func.name != name {
            continue;
        }
        if func.params.is_empty() {
            return Some(func.signature());
        }
        let index = active.min(func.params.len() - 1);
        return Some(format!("{}: {}", func.params[index], func.signature()));
    }
    let (signature, _) = lexicon::function_doc(name)?;
    Some(signature.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sl(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn call_hover_names_the_active_parameter() {
        let lines = sl(&["func add(a, b)", "end"]);
        assert_eq!(call_hover("add", 1, &lines).unwrap(), "b: func add(a, b)");
        assert_eq!(call_hover("add", 9, &lines).unwrap(), "b: func add(a, b)");
        assert_eq!(call_hover("add", 0, &lines).unwrap(), "a: func add(a, b)");
    }

    #[test]
    fn a_function_without_parameters_shows_its_signature() {
        let lines = sl(&["func tick()", "end"]);
        assert_eq!(call_hover("tick", 0, &lines).unwrap(), "func tick()");
    }

    #[test]
    fn expression_functions_fall_back_to_the_signature() {
        assert!(call_hover("len", 0, &[]).unwrap().starts_with("len("));
        assert!(call_hover("nprintf", 2, &[])
            .unwrap()
            .starts_with("nprintf("));
    }

    #[test]
    fn unknown_names_stay_quiet() {
        assert!(call_hover("nope", 0, &[]).is_none());
    }
}
