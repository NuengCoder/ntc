//! Execution engine for `.ntc` scripts.
//!
//! Walks the statement tree from `parser`, keeping a stack of variable scopes
//! (the global frame plus one per function call), a table of user functions,
//! and counters for the summary line. Command lines are handed to `exec_fn`;
//! `input` reads through `read_fn` so the engine stays testable.

use crate::output::{print_error, print_info, print_success, print_warning};
use crate::shell::commands::builtin::scr::script::parser::{self, Kind, Stmt};
use crate::shell::commands::builtin::scr::script::{expr, interp};
use crate::t;

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

/// Upper bound on the iterations of a single loop. A typo in a condition
/// would otherwise hang the shell with no way out.
const MAX_LOOP_ITERATIONS: u64 = 1_000_000;

/// Upper bound on user-function recursion, well under the native stack.
const MAX_CALL_DEPTH: usize = 64;

/// What a block wants the engine to do next.
enum Flow {
    Normal,
    Break,
    Continue,
    Return(expr::Value),
    Exit,
}

/// A user function declared with `func`.
#[derive(Clone)]
struct FuncDef {
    params: Vec<String>,
    body: Vec<Stmt>,
}

/// Execute a `.ntc` script.
///
/// `exec_fn` runs a single (already interpolated) command line. It receives the
/// expanded command and whether it is a forced system command (`@` prefix).
/// Return `Ok(true)` on success, `Ok(false)` for a failed system command, or
/// `Err` for a hard failure.
///
/// `read_fn` receives a prompt and returns one line, or `None` at end of input.
pub(crate) fn run_script(
    path: &Path,
    args: &[String],
    continue_on_err: bool,
    exec_fn: &mut dyn FnMut(&str, bool) -> Result<bool>,
    read_fn: &mut dyn FnMut(&str) -> Option<String>,
) -> Result<()> {
    let display = path.display().to_string();

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            print_error(&t!("scr.not_found", display, e));
            return Ok(());
        }
    };
    let content = content.trim_start_matches('\u{feff}');
    let settings = super::directives::directives(content);
    let (stmts, parse_errors) = parser::parse(content);

    if !parse_errors.is_empty() {
        for error in &parse_errors {
            print_error(&t!("scr.parse_error", display, error.line, error.message));
        }
        print_error(&t!("scr.parse_failed", display, parse_errors.len()));
        return Ok(());
    }

    if stmts.is_empty() {
        print_warning(&t!("scr.no_commands", display));
        return Ok(());
    }

    let mut funcs = HashMap::new();
    hoist(&stmts, &mut funcs);

    let mut interp = Interp {
        display: display.clone(),
        args: args.to_vec(),
        continue_on_err,
        funcs,
        scopes: vec![HashMap::new()],
        executed: 0,
        errors: 0,
        depth: 0,
        aborted: false,
        show_exec: settings.show_exec,
        exec: exec_fn,
        read: read_fn,
    };

    let _ = interp.run_block(&stmts)?;

    if interp.aborted {
        return Ok(());
    }
    if interp.errors > 0 {
        print_warning(&t!("scr.done_errors", display, interp.errors));
    } else {
        print_success(&t!("scr.completed", display, interp.executed));
    }
    Ok(())
}

/// Collect every `func` definition before execution so a script can call a
/// function that is declared further down.
fn hoist(stmts: &[Stmt], funcs: &mut HashMap<String, FuncDef>) {
    for stmt in stmts {
        match &stmt.kind {
            Kind::Func { name, params, body } => {
                funcs.insert(
                    name.clone(),
                    FuncDef {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
            }
            Kind::If {
                branches,
                otherwise,
            } => {
                for branch in branches {
                    hoist(&branch.body, funcs);
                }
                if let Some(body) = otherwise {
                    hoist(body, funcs);
                }
            }
            Kind::While { body, .. } | Kind::For { body, .. } | Kind::Loop { body, .. } => {
                hoist(body, funcs);
            }
            _ => {}
        }
    }
}

struct Interp<'a> {
    display: String,
    args: Vec<String>,
    continue_on_err: bool,
    funcs: HashMap<String, FuncDef>,
    scopes: Vec<HashMap<String, String>>,
    executed: usize,
    errors: usize,
    depth: usize,
    aborted: bool,
    /// `set-showexc = off` silences the `[n] Executing: …` trace.
    show_exec: bool,
    exec: &'a mut dyn FnMut(&str, bool) -> Result<bool>,
    read: &'a mut dyn FnMut(&str) -> Option<String>,
}

impl Interp<'_> {
    // ── variables ───────────────────────────────────────────────────────

    fn lookup(&self, name: &str) -> Option<String> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    /// Resolve a name the way `$name` does: script args first, then scopes.
    fn resolve(&self, name: &str) -> Option<String> {
        if name == "@" || name == "*" {
            return Some(self.args.join(" "));
        }
        if let Ok(index) = name.parse::<usize>() {
            return Some(self.args.get(index.wrapping_sub(1)).cloned().unwrap_or_default());
        }
        self.lookup(name)
    }

    fn set_var(&mut self, name: &str, value: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), value);
        }
    }

    fn expand(&self, text: &str) -> String {
        interp::interpolate_with(text, &|name| self.resolve(name), &self.args)
    }

    fn eval(&self, src: &str) -> Result<expr::Value, String> {
        expr::eval(src, &|name| self.resolve(name))
    }

    /// Evaluate a value the way `set` does: plain text is interpolated and
    /// kept verbatim (so paths and spaces survive), anything with an operator
    /// goes through the expression evaluator.
    fn eval_value(&self, src: &str) -> Result<String, String> {
        let trimmed = src.trim();
        if expr::is_plain(trimmed) {
            return Ok(strip_quotes(&self.expand(trimmed)));
        }
        Ok(self.eval(trimmed)?.as_text())
    }

    fn eval_cond(&self, src: &str) -> Result<bool, String> {
        Ok(self.eval(src.trim())?.truthy())
    }

    // ── errors ──────────────────────────────────────────────────────────

    /// Report a runtime problem and decide whether the script keeps going.
    fn fail(&mut self, line: usize, message: impl Into<String>) -> Result<Flow> {
        self.errors += 1;
        let message = message.into();
        if self.continue_on_err {
            print_warning(&t!("scr.continue_warn", line));
            Ok(Flow::Normal)
        } else {
            print_error(&t!("scr.error_at", self.display, line, message));
            print_error(&t!(
                "scr.failed_abort",
                self.display,
                line,
                self.errors
            ));
            // The abort message already said everything; no summary follows.
            self.aborted = true;
            Ok(Flow::Exit)
        }
    }

    // ── statements ──────────────────────────────────────────────────────

    fn run_block(&mut self, stmts: &[Stmt]) -> Result<Flow> {
        for stmt in stmts {
            match self.run_stmt(stmt)? {
                Flow::Normal => {}
                other => return Ok(other),
            }
        }
        Ok(Flow::Normal)
    }

    fn run_stmt(&mut self, stmt: &Stmt) -> Result<Flow> {
        match &stmt.kind {
            Kind::Set { name, expr: src } => self.run_set(stmt.line, name, src),
            Kind::Cmd { text, force } => self.run_command(stmt.line, text, *force),
            Kind::If {
                branches,
                otherwise,
            } => {
                for branch in branches {
                    match self.eval_cond(&branch.cond) {
                        Ok(true) => return self.run_block(&branch.body),
                        Ok(false) => {}
                        Err(e) => {
                            return self.fail(stmt.line, t!("scr.err_expr", branch.cond, e));
                        }
                    }
                }
                match otherwise {
                    Some(body) => self.run_block(body),
                    None => Ok(Flow::Normal),
                }
            }
            Kind::While { cond, body } => self.run_while(stmt.line, cond, body),
            Kind::For { var, list, body } => self.run_for(stmt.line, var, list, body),
            Kind::Loop { count, body } => self.run_loop(stmt.line, count.as_deref(), body),
            // Definitions are hoisted; reaching one at runtime does nothing.
            Kind::Func { .. } => Ok(Flow::Normal),
            Kind::Call { name, args } => match self.invoke(name, args, stmt.line)? {
                Flow::Exit => Ok(Flow::Exit),
                _ => Ok(Flow::Normal),
            },
            Kind::Return(value) => {
                let value = match value {
                    Some(src) => match self.eval_value(src) {
                        Ok(v) => expr::Value::Text(v),
                        Err(e) => return self.fail(stmt.line, t!("scr.err_expr", src, e)),
                    },
                    None => expr::Value::Text(String::new()),
                };
                Ok(Flow::Return(value))
            }
            Kind::Input { name, prompt } => self.run_input(stmt.line, name, prompt.as_deref()),
            Kind::Break => Ok(Flow::Break),
            Kind::Continue => Ok(Flow::Continue),
            Kind::Exit => Ok(Flow::Exit),
        }
    }

    fn run_set(&mut self, line: usize, name: &str, src: &str) -> Result<Flow> {
        let trimmed = src.trim();

        // `set total = call add(1, 2)` captures a function's return value.
        if let Some(rest) = trimmed.strip_prefix("call ") {
            let Some((func, arg_srcs)) = parser::parse_call(rest) else {
                return self.fail(line, "`call` needs a function name");
            };
            return match self.invoke(&func, &arg_srcs, line)? {
                Flow::Return(value) => {
                    self.set_var(name, value.as_text());
                    Ok(Flow::Normal)
                }
                Flow::Exit => Ok(Flow::Exit),
                _ => Ok(Flow::Normal),
            };
        }

        match self.eval_value(trimmed) {
            Ok(value) => {
                self.set_var(name, value);
                Ok(Flow::Normal)
            }
            Err(e) => self.fail(line, t!("scr.err_expr", trimmed, e)),
        }
    }

    fn run_command(&mut self, line: usize, text: &str, force: bool) -> Result<Flow> {
        let expanded = self.expand(text);
        let trimmed = expanded.trim();
        if trimmed.is_empty() {
            return Ok(Flow::Normal);
        }

        if self.show_exec {
            println!();
            print_info(&t!("scr.executing", line, trimmed));
        }
        self.executed += 1;

        match (self.exec)(trimmed, force) {
            Ok(true) => Ok(Flow::Normal),
            Ok(false) => self.fail(line, t!("scr.cmd_failed")),
            Err(e) => self.fail(line, e.to_string()),
        }
    }

    fn run_while(&mut self, line: usize, cond: &str, body: &[Stmt]) -> Result<Flow> {
        let mut iterations = 0u64;
        loop {
            match self.eval_cond(cond) {
                Ok(true) => {}
                Ok(false) => return Ok(Flow::Normal),
                Err(e) => return self.fail(line, t!("scr.err_expr", cond, e)),
            }
            iterations += 1;
            if iterations > MAX_LOOP_ITERATIONS {
                return self.fail(line, t!("scr.err_loop_limit", MAX_LOOP_ITERATIONS));
            }
            match self.run_block(body)? {
                Flow::Break => return Ok(Flow::Normal),
                Flow::Continue | Flow::Normal => {}
                other => return Ok(other),
            }
        }
    }

    fn run_for(&mut self, line: usize, var: &str, list: &str, body: &[Stmt]) -> Result<Flow> {
        let items = match self.list_items(list) {
            Ok(items) => items,
            Err(e) => return self.fail(line, t!("scr.err_expr", list, e)),
        };

        for item in items {
            self.set_var(var, item);
            match self.run_block(body)? {
                Flow::Break => return Ok(Flow::Normal),
                Flow::Continue | Flow::Normal => {}
                other => return Ok(other),
            }
        }
        Ok(Flow::Normal)
    }

    fn run_loop(&mut self, line: usize, count: Option<&str>, body: &[Stmt]) -> Result<Flow> {
        let limit = match count {
            None => None,
            Some(src) => match self.eval(src.trim()) {
                Ok(value) => match value.as_int() {
                    Some(n) => Some(n.max(0) as u64),
                    None => {
                        return self.fail(
                            line,
                            t!("scr.err_expr", src, format!("'{}' is not a number", value)),
                        )
                    }
                },
                Err(e) => return self.fail(line, t!("scr.err_expr", src, e)),
            },
        };

        let mut iterations = 0u64;
        loop {
            if let Some(limit) = limit {
                if iterations >= limit {
                    return Ok(Flow::Normal);
                }
            }
            iterations += 1;
            if iterations > MAX_LOOP_ITERATIONS {
                return self.fail(line, t!("scr.err_loop_limit", MAX_LOOP_ITERATIONS));
            }
            match self.run_block(body)? {
                Flow::Break => return Ok(Flow::Normal),
                Flow::Continue | Flow::Normal => {}
                other => return Ok(other),
            }
        }
    }

    /// Resolve a `for` list: an expression when it has operators, otherwise
    /// the raw (interpolated) text, then split on whitespace and commas.
    fn list_items(&self, src: &str) -> Result<Vec<String>, String> {
        let text = if expr::is_plain(src) {
            self.expand(src.trim())
        } else {
            self.eval(src.trim())?.as_text()
        };
        Ok(text
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|item| !item.is_empty())
            .map(|item| item.to_string())
            .collect())
    }

    fn run_input(&mut self, line: usize, name: &str, prompt: Option<&str>) -> Result<Flow> {
        let prompt = match prompt {
            Some(src) => match self.eval_value(src) {
                Ok(text) => text,
                Err(e) => return self.fail(line, t!("scr.err_expr", src, e)),
            },
            None => t!("scr.input_prompt"),
        };
        match (self.read)(&prompt) {
            Some(value) => {
                self.set_var(name, value);
                Ok(Flow::Normal)
            }
            None => {
                print_warning(&t!("scr.input_eof", name));
                Ok(Flow::Exit)
            }
        }
    }

    fn invoke(&mut self, name: &str, arg_srcs: &[String], line: usize) -> Result<Flow> {
        let Some(def) = self.funcs.get(name).cloned() else {
            return self.fail(line, t!("scr.err_func_missing", name));
        };
        if def.params.len() != arg_srcs.len() {
            return self.fail(
                line,
                t!(
                    "scr.err_func_args",
                    name,
                    def.params.len(),
                    arg_srcs.len()
                ),
            );
        }
        if self.depth >= MAX_CALL_DEPTH {
            return self.fail(line, t!("scr.err_recursion", name, MAX_CALL_DEPTH));
        }

        let mut frame = HashMap::new();
        for (param, src) in def.params.iter().zip(arg_srcs) {
            match self.eval_value(src) {
                Ok(value) => {
                    frame.insert(param.clone(), value);
                }
                Err(e) => return self.fail(line, t!("scr.err_expr", src, e)),
            }
        }

        self.scopes.push(frame);
        self.depth += 1;
        let flow = self.run_block(&def.body);
        self.depth -= 1;
        self.scopes.pop();

        match flow? {
            Flow::Return(value) => Ok(Flow::Return(value)),
            Flow::Exit => Ok(Flow::Exit),
            // `break` / `continue` cannot cross a function boundary.
            Flow::Break | Flow::Continue => {
                self.fail(line, t!("scr.err_break_outside_loop", name))
            }
            Flow::Normal => Ok(Flow::Return(expr::Value::Text(String::new()))),
        }
    }
}

/// Remove one layer of surrounding matching quotes.
fn strip_quotes(value: &str) -> String {
    let v = value.trim();
    if v.len() >= 2 {
        let bytes = v.as_bytes();
        let first = bytes[0];
        let last = bytes[v.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return v[1..v.len() - 1].to_string();
        }
    }
    v.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Outcome {
        seen: Vec<(String, bool)>,
        prompt_count: usize,
    }

    /// Run a script body with a mock host. `input` lines come from `inputs`
    /// in order; running out behaves like end of input.
    fn run_full(content: &str, args: &[String], continue_on_err: bool, inputs: &[&str]) -> Outcome {
        let mut seen: Vec<(String, bool)> = Vec::new();
        let mut queue: Vec<String> = inputs.iter().map(|s| (*s).to_string()).collect();
        let mut prompts = 0usize;

        {
            let mut exec = |cmd: &str, force: bool| -> Result<bool> {
                seen.push((cmd.to_string(), force));
                Ok(!cmd.contains("FAIL"))
            };
            let mut read = |_prompt: &str| -> Option<String> {
                prompts += 1;
                if queue.is_empty() {
                    None
                } else {
                    Some(queue.remove(0))
                }
            };
            let dir = tempfile::tempdir().unwrap();
            let path = dir.path().join("ntc_script_test.ntc");
            std::fs::write(&path, content).unwrap();
            let _ = run_script(&path, args, continue_on_err, &mut exec, &mut read);
        }

        Outcome {
            seen,
            prompt_count: prompts,
        }
    }

    fn run(content: &str, args: &[String], continue_on_err: bool) -> Outcome {
        run_full(content, args, continue_on_err, &[])
    }

    fn cmds(outcome: &Outcome) -> Vec<String> {
        outcome.seen.iter().map(|(c, _)| c.clone()).collect()
    }

    #[test]
    fn test_executes_in_order() {
        let outcome = run("cd src\nsay hi\necho bye", &[], false);
        assert_eq!(cmds(&outcome), vec!["cd src", "say hi", "echo bye"]);
    }

    #[test]
    fn test_variable_interpolation() {
        let outcome = run("set dir = src\ncd $dir", &[], false);
        assert_eq!(outcome.seen[0].0, "cd src");
    }

    #[test]
    fn test_positional_args() {
        let outcome = run("say $1 $@", &["a".into(), "b".into()], false);
        assert_eq!(outcome.seen[0].0, "say a a b");
    }

    #[test]
    fn test_force_flag() {
        let outcome = run("@ echo forced", &[], false);
        assert!(outcome.seen[0].1);
    }

    #[test]
    fn test_non_force_flag() {
        let outcome = run("echo normal", &[], false);
        assert!(!outcome.seen[0].1);
    }

    #[test]
    fn test_stops_on_error_by_default() {
        let outcome = run("say one\nFAIL here\nsay three", &[], false);
        assert_eq!(cmds(&outcome), vec!["say one", "FAIL here"]);
    }

    #[test]
    fn test_continues_with_flag() {
        let outcome = run("say one\nFAIL here\nsay three", &[], true);
        assert_eq!(cmds(&outcome), vec!["say one", "FAIL here", "say three"]);
    }

    #[test]
    fn test_skip_empty_after_interpolation() {
        let outcome = run("$1\nsay ok", &[], false);
        assert_eq!(outcome.seen.len(), 1);
        assert_eq!(outcome.seen[0].0, "say ok");
    }

    #[test]
    fn arithmetic_updates_a_counter() {
        let outcome = run("set n = 1\nset n = $n + 4\nsay $n", &[], false);
        assert_eq!(outcome.seen[0].0, "say 5");
    }

    #[test]
    fn plain_values_keep_spacing_and_quotes() {
        let outcome = run(
            "set p = C:\\Users\\me\nset q = \"two words\"\nsay $p | $q",
            &[],
            false,
        );
        assert_eq!(outcome.seen[0].0, "say C:\\Users\\me | two words");
    }

    #[test]
    fn if_picks_the_first_true_branch() {
        let script = "set n = 2\nif $n == 1\n say one\nelse if $n == 2\n say two\nelse\n say other\nend";
        let outcome = run(script, &[], false);
        assert_eq!(cmds(&outcome), vec!["say two"]);
    }

    #[test]
    fn if_false_skips_the_body() {
        let outcome = run("if 0\n say never\nend\nsay after", &[], false);
        assert_eq!(cmds(&outcome), vec!["say after"]);
    }

    #[test]
    fn while_loop_accumulates() {
        let script = "set i = 0\nset sum = 0\nwhile $i < 4\n set sum = $sum + $i\n set i = $i + 1\nend\nsay $sum";
        let outcome = run(script, &[], false);
        assert_eq!(outcome.seen[0].0, "say 6");
    }

    #[test]
    fn for_loop_walks_words() {
        let outcome = run("for f in a b c\n say $f\nend", &[], false);
        assert_eq!(cmds(&outcome), vec!["say a", "say b", "say c"]);
    }

    #[test]
    fn for_loop_walks_an_expression() {
        let outcome = run("set list = \"x y\"\nfor f in $list\n say $f\nend", &[], false);
        assert_eq!(cmds(&outcome), vec!["say x", "say y"]);
    }

    #[test]
    fn loop_repeats_n_times() {
        let outcome = run("loop 3\n say again\nend", &[], false);
        assert_eq!(outcome.seen.len(), 3);
    }

    #[test]
    fn break_and_continue_work() {
        let script = "set i = 0\nwhile 1\n set i = $i + 1\n if $i == 2\n  continue\n end\n if $i == 4\n  break\n end\n say $i\nend";
        let outcome = run(script, &[], false);
        assert_eq!(cmds(&outcome), vec!["say 1", "say 3"]);
    }

    #[test]
    fn functions_take_arguments_and_return_values() {
        let script = "func add(a, b)\n return $a + $b\nend\nset total = call add(2, 3)\nsay $total\ncall add 1 1";
        let outcome = run(script, &[], false);
        assert_eq!(cmds(&outcome), vec!["say 5"]);
    }

    #[test]
    fn functions_can_be_called_before_they_are_defined() {
        let outcome = run("call hi\nfunc hi\n say hello\nend", &[], false);
        assert_eq!(cmds(&outcome), vec!["say hello"]);
    }

    #[test]
    fn function_locals_do_not_leak() {
        let script = "func f()\n set local = inner\nend\ncall f\nsay [$local]";
        let outcome = run(script, &[], false);
        assert_eq!(outcome.seen[0].0, "say [$local]");
    }

    #[test]
    fn input_reads_a_line() {
        let outcome = run_full("input name \"Who? \"\nsay hi $name", &[], false, &["Alice"]);
        assert_eq!(outcome.seen[0].0, "say hi Alice");
        assert_eq!(outcome.prompt_count, 1);
    }

    #[test]
    fn input_at_end_of_input_stops_the_script() {
        let outcome = run_full("input a\nsay never", &[], false, &[]);
        assert!(outcome.seen.is_empty());
    }

    #[test]
    fn nprintf_formats_into_a_variable() {
        let outcome = run(
            "set i = 7\nset label = nprintf(\"%03d\", $i)\nsay $label",
            &[],
            false,
        );
        assert_eq!(outcome.seen[0].0, "say 007");
    }

    #[test]
    fn exit_stops_the_script() {
        let outcome = run("say one\nexit\nsay two", &[], false);
        assert_eq!(cmds(&outcome), vec!["say one"]);
    }

    #[test]
    fn unknown_function_is_reported_not_fatal_to_the_process() {
        let outcome = run("call nope", &[], true);
        assert!(outcome.seen.is_empty());
    }

    #[test]
    fn directives_never_run_as_commands() {
        let outcome = run("set-pipe = fail\nset-showexc = 0\nsay hi", &[], false);
        assert_eq!(cmds(&outcome), vec!["say hi"]);
    }
}
