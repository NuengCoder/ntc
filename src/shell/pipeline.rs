// src/shell/pipeline.rs
// Command-line pipeline & redirection support for the ntc shell.
//
// Supported operators (only when they are whitespace-delimited standalone
// tokens, and never inside "..." / '...' quotes or [...] array literals):
//
//   a | b        pipe a's stdout into b's stdin
//   a && b       run b only if a succeeded
//   a || b       run b only if a failed
//   cmd > file   write stdout to file (overwrite)
//   cmd >> file  append stdout to file
//   cmd < file   read stdin from file
//   cmd <<EOF    read stdin from the following lines until a line == EOF
//   cmd <<< text (here-string) read stdin from the literal text token
//
// Requiring surrounding spaces protects commands that use `<` / `>` / `|`
// as data, e.g. `nsheet data.csv --filter age>30` stays intact.
//
// Redirection/piped output is captured in memory, stripped of ANSI colour
// codes, and then written to the next stage or target file — so redirected
// files are clean plain text.

use crate::navigator::Navigator;
use crate::watcher;
use anyhow::{anyhow, Result};
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use super::stdin;

const OPERATORS: &[&str] = &["&&", "||", "|", "<", "<<", "<<<", ">", ">>"];

fn is_operator(tok: &str) -> bool {
    OPERATORS.contains(&tok)
}

/// Canonical operator token (`|`, `>>`, …) for a tokenizer token.
fn operator_kind(tok: &str) -> Option<&'static str> {
    OPERATORS.iter().copied().find(|op| *op == tok)
}

/// Command separators used by alias expansion (redirects excluded, so a
/// filename after `>` is never mistaken for a command).
fn is_command_sep(tok: &str) -> bool {
    matches!(tok, "&&" | "||" | "|")
}

// ── failure status side-channel ────────────────────────────────────────────
// Used by `&&` / `||` to decide whether the next segment runs. Set by the
// system-command runner (non-zero exit) and when a command returns a hard
// error. Conservative: built-ins that print an error and return Ok are not
// detected (documented limitation).

static FAILED: AtomicBool = AtomicBool::new(false);

/// Mark the currently running command as failed.
pub(crate) fn mark_failed() {
    FAILED.store(true, Ordering::SeqCst);
}

pub(crate) fn reset_failed() {
    FAILED.store(false, Ordering::SeqCst);
}

pub(crate) fn take_failed() -> bool {
    FAILED.swap(false, Ordering::SeqCst)
}

// ── diagnostics suppression ────────────────────────────────────────────────
// While a stage's output is being captured we suppress the shell's own
// chatter (`Executing: ...`, `completed`, ...) so piped/redirected output
// stays clean.

thread_local! {
    static QUIET_DEPTH: Cell<usize> = Cell::new(0);
    static HEREDOCS: RefCell<Vec<Vec<u8>>> = RefCell::new(Vec::new());
}

/// Whether shell diagnostics should currently be silenced.
pub(crate) fn is_quiet() -> bool {
    QUIET_DEPTH.with(|d| d.get() > 0)
}

struct QuietGuard;

impl QuietGuard {
    fn new() -> Self {
        QUIET_DEPTH.with(|d| d.set(d.get() + 1));
        QuietGuard
    }
}

impl Drop for QuietGuard {
    fn drop(&mut self) {
        QUIET_DEPTH.with(|d| d.set(d.get().saturating_sub(1)));
    }
}

// ── tokenizer ───────────────────────────────────────────────────────────────
// Whitespace split that keeps quoted spans and `[...]` array literals as a
// single token and preserves the original text (quotes included).

fn tokenize(input: &str) -> Vec<String> {
    tokenize_spans(input)
        .into_iter()
        .map(|(t, _, _)| t)
        .collect()
}

/// Tokenizer that also reports each token's byte span in the input, so
/// callers (the hinter) can slice the original line without rebuilding it.
fn tokenize_spans(input: &str) -> Vec<(String, usize, usize)> {
    let mut toks: Vec<(String, usize, usize)> = Vec::new();
    let mut cur = String::new();
    let mut cur_start = 0usize;
    let mut chars = input.char_indices().peekable();
    let mut in_double = false;
    let mut in_single = false;
    let mut in_bracket = false;

    while let Some((i, ch)) = chars.next() {
        if cur.is_empty() {
            cur_start = i;
        }
        if in_bracket {
            cur.push(ch);
            if ch == ']' {
                in_bracket = false;
                toks.push((std::mem::take(&mut cur), cur_start, i + ch.len_utf8()));
            }
            continue;
        }
        if in_double {
            match ch {
                '\\' => {
                    cur.push(ch);
                    if let Some(&(_, n)) = chars.peek() {
                        cur.push(n);
                        chars.next();
                    }
                }
                '"' => {
                    in_double = false;
                    cur.push(ch);
                }
                _ => cur.push(ch),
            }
            continue;
        }
        if in_single {
            cur.push(ch);
            if ch == '\'' {
                in_single = false;
            }
            continue;
        }
        match ch {
            '"' => {
                in_double = true;
                cur.push(ch);
            }
            '\'' => {
                in_single = true;
                cur.push(ch);
            }
            '[' => {
                in_bracket = true;
                cur.push(ch);
            }
            '<' if matches!(chars.peek(), Some(&(_, '<'))) => {
                // Heredoc operators may sit attached to their delimiter
                // (`<<EOF`, `<<<text`). Treat them as standalone operators
                // so the delimiter becomes the next token.
                if !cur.is_empty() {
                    toks.push((std::mem::take(&mut cur), cur_start, i));
                }
                let (j, _) = chars.next().unwrap();
                let mut end = j + 1;
                if matches!(chars.peek(), Some(&(_, '<'))) {
                    let (k, _) = chars.next().unwrap();
                    end = k + 1;
                    toks.push(("<<<".to_string(), i, end));
                } else {
                    toks.push(("<<".to_string(), i, end));
                }
            }
            ' ' | '\t' | '\r' | '\n' => {
                if !cur.is_empty() {
                    toks.push((std::mem::take(&mut cur), cur_start, i));
                }
            }
            _ => cur.push(ch),
        }
    }
    if !cur.is_empty() {
        let end = input.len();
        toks.push((cur, cur_start, end));
    }
    toks
}

/// The last top-level operator in `line` and the text after it.
///
/// Operators inside quotes or `[...]` arrays are data, and only standalone
/// tokens count (`a|b`, `a>b`, `age>30` never split) — the same rules the
/// executor uses, so completion always matches what would actually run.
/// Attached heredoc operators (`<<EOF`) leave the delimiter in the tail.
pub(crate) fn active_operator(line: &str) -> Option<(&'static str, &str)> {
    let spans = tokenize_spans(line);
    let (op, end) = spans
        .iter()
        .rev()
        .find_map(|(t, _, end)| operator_kind(t).map(|op| (op, *end)))?;
    Some((op, &line[end..]))
}

/// Strip a single pair of surrounding matching quotes.
fn unquote(t: &str) -> String {
    let t = t.trim();
    let b = t.as_bytes();
    if t.len() >= 2
        && ((b[0] == b'"' && b[t.len() - 1] == b'"') || (b[0] == b'\'' && b[t.len() - 1] == b'\''))
    {
        return t[1..t.len() - 1].to_string();
    }
    t.to_string()
}

/// Remove ANSI escape sequences from captured output so redirected files and
/// piped data stay clean.
pub(crate) fn strip_ansi_bytes(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        if input[i] == 0x1b {
            i += 1;
            if i < input.len() && input[i] == b'[' {
                i += 1;
                while i < input.len() && !(0x40..=0x7e).contains(&input[i]) {
                    i += 1;
                }
                if i < input.len() {
                    i += 1;
                }
            } else if i < input.len() {
                i += 1;
            }
        } else {
            out.push(input[i]);
            i += 1;
        }
    }
    out
}

// ── parsed model ────────────────────────────────────────────────────────────

pub(crate) enum InSpec {
    File(String),
    Heredoc(Vec<u8>),
}

pub(crate) enum OutSpec {
    Truncate(String),
    Append(String),
}

pub(crate) struct Stage {
    pub cmd: String,
    pub input: Option<InSpec>,
    pub output: Option<OutSpec>,
}

enum LogicOp {
    And,
    Or,
}

type Segment = (Option<LogicOp>, Vec<Stage>);

/// True when the line contains any top-level operator token.
pub(crate) fn has_operators(input: &str) -> bool {
    tokenize(input).iter().any(|t| is_operator(t))
}

/// Split a line on command separators (`&&`, `||`, `|`) for alias expansion.
/// Returns `(separator, segment_text)` pairs; the first separator is `None`.
pub(crate) fn split_commands(input: &str) -> Vec<(Option<String>, String)> {
    let toks = tokenize(input);
    let mut out: Vec<(Option<String>, Vec<String>)> = vec![(None, Vec::new())];
    for t in toks {
        if is_command_sep(&t) {
            out.push((Some(t), Vec::new()));
        } else {
            out.last_mut().unwrap().1.push(t);
        }
    }
    out.into_iter()
        .map(|(sep, toks)| (sep, toks.join(" ")))
        .collect()
}

/// Parse a fully expanded line into logical segments, each a pipeline of
/// stages with their redirections.
fn parse(input: &str) -> Vec<Segment> {
    let toks = tokenize(input);

    // Split on && / || into logical groups, preserving the rest of the tokens.
    let mut groups: Vec<(Option<LogicOp>, Vec<String>)> = vec![(None, Vec::new())];
    for t in toks {
        match t.as_str() {
            "&&" => groups.push((Some(LogicOp::And), Vec::new())),
            "||" => groups.push((Some(LogicOp::Or), Vec::new())),
            _ => groups.last_mut().unwrap().1.push(t),
        }
    }

    groups
        .into_iter()
        .map(|(op, toks)| (op, parse_stages(toks)))
        .collect()
}

fn parse_stages(toks: Vec<String>) -> Vec<Stage> {
    let mut stages: Vec<Stage> = Vec::new();
    let mut words: Vec<String> = Vec::new();
    let mut input: Option<InSpec> = None;
    let mut output: Option<OutSpec> = None;

    let mut i = 0;
    while i < toks.len() {
        let t = &toks[i];
        match t.as_str() {
            "|" => {
                stages.push(Stage {
                    cmd: words.join(" "),
                    input: input.take(),
                    output: output.take(),
                });
                words.clear();
            }
            "<" => match toks.get(i + 1) {
                Some(target) if !is_operator(target) => {
                    input = Some(InSpec::File(unquote(target)));
                    i += 1;
                }
                _ => words.push(t.clone()),
            },
            "<<" => match toks.get(i + 1) {
                Some(target) if !is_operator(target) => {
                    input = Some(InSpec::Heredoc(pop_heredoc()));
                    let _ = target;
                    i += 1;
                }
                _ => words.push(t.clone()),
            },
            "<<<" => match toks.get(i + 1) {
                Some(text) if !is_operator(text) => {
                    input = Some(InSpec::Heredoc(unquote(text).into_bytes()));
                    i += 1;
                }
                _ => words.push(t.clone()),
            },
            ">" => match toks.get(i + 1) {
                Some(target) if !is_operator(target) => {
                    output = Some(OutSpec::Truncate(unquote(target)));
                    i += 1;
                }
                _ => words.push(t.clone()),
            },
            ">>" => match toks.get(i + 1) {
                Some(target) if !is_operator(target) => {
                    output = Some(OutSpec::Append(unquote(target)));
                    i += 1;
                }
                _ => words.push(t.clone()),
            },
            _ => words.push(t.clone()),
        }
        i += 1;
    }

    stages.push(Stage {
        cmd: words.join(" "),
        input,
        output,
    });
    stages
}

// ── heredoc supply ──────────────────────────────────────────────────────────

fn pop_heredoc() -> Vec<u8> {
    HEREDOCS.with(|h| {
        let mut b = h.borrow_mut();
        if b.is_empty() {
            Vec::new()
        } else {
            b.remove(0)
        }
    })
}

/// Install the heredoc bodies (in reading order) for the next `run`.
pub(crate) fn set_heredoc_bodies(bodies: Vec<Vec<u8>>) {
    HEREDOCS.with(|h| {
        let mut b = h.borrow_mut();
        b.clear();
        b.extend(bodies);
    });
}

/// The delimiters of every top-level `<<` heredoc, in reading order.
pub(crate) fn scan_heredoc_delims(input: &str) -> Vec<String> {
    let toks = tokenize(input);
    let mut out = Vec::new();
    let mut i = 0;
    while i < toks.len() {
        if toks[i] == "<<" {
            if let Some(d) = toks.get(i + 1) {
                if !is_operator(d) {
                    out.push(unquote(d));
                    i += 1;
                }
            }
        }
        i += 1;
    }
    out
}

/// Whether the line declares a `<<` heredoc.
pub(crate) fn has_heredoc(input: &str) -> bool {
    !scan_heredoc_delims(input).is_empty()
}

// ── execution ───────────────────────────────────────────────────────────────

/// Run a full command line, handling `&&` / `||` sequencing and pipelines.
/// Returns `(exit_shell, success)`.
pub(crate) fn run(
    input: &str,
    nav: &mut Navigator,
    watcher_handle: &Option<Arc<watcher::WatcherHandle>>,
) -> Result<(bool, bool)> {
    let segments = parse(input);
    let mut prev_ok = true;

    for (op, stages) in segments {
        if let Some(op) = op {
            match op {
                LogicOp::And if !prev_ok => continue,
                LogicOp::Or if prev_ok => continue,
                _ => {}
            }
        }
        // Nothing to run (e.g. a blank segment from a trailing operator).
        if stages
            .iter()
            .all(|s| s.cmd.trim().is_empty() && s.output.is_none())
        {
            continue;
        }

        let (exit, ok) = run_segment(stages, nav, watcher_handle)?;
        prev_ok = ok;
        if exit {
            return Ok((true, ok));
        }
    }
    Ok((false, prev_ok))
}

fn run_segment(
    stages: Vec<Stage>,
    nav: &mut Navigator,
    watcher_handle: &Option<Arc<watcher::WatcherHandle>>,
) -> Result<(bool, bool)> {
    let n = stages.len();
    let mut carry: Option<Vec<u8>> = None;
    let mut exit = false;
    let mut ok = true;

    for (i, stage) in stages.iter().enumerate() {
        let is_last = i + 1 == n;

        // Resolve this stage's input.
        let input_bytes: Option<Vec<u8>> = match &stage.input {
            Some(InSpec::File(p)) => Some(read_input_file(p, nav)?),
            Some(InSpec::Heredoc(b)) => Some(b.clone()),
            None => carry.clone(),
        };

        if stage.cmd.trim().is_empty() {
            // A bare redirect (`> file`) with no command: pass input through.
            if let Some(ref spec) = stage.output {
                write_output_file(spec, input_bytes.as_deref().unwrap_or(&[]), nav)?;
            }
            carry = input_bytes;
            continue;
        }

        let capture = !is_last || stage.output.is_some();
        reset_failed();

        let exit_cell = Cell::new(false);
        let err_cell: RefCell<Option<anyhow::Error>> = RefCell::new(None);
        let mut captured: Option<Vec<u8>> = None;

        {
            let _quiet = if capture {
                Some(QuietGuard::new())
            } else {
                None
            };

            if let Some(ref bytes) = input_bytes {
                stdin::set_input(bytes.clone());
            } else {
                stdin::clear();
            }

            let mut execute =
                || match super::commands::execute_single_command(&stage.cmd, nav, watcher_handle) {
                    Ok(e) => exit_cell.set(e),
                    Err(err) => *err_cell.borrow_mut() = Some(err),
                };

            if capture {
                captured = Some(crate::output::with_captured_output_bytes(execute));
            } else {
                execute();
            }

            stdin::clear();
        }

        if let Some(err) = err_cell.into_inner() {
            return Err(err);
        }

        exit = exit_cell.get();
        ok = !take_failed();

        if let Some(bytes) = captured {
            let clean = strip_ansi_bytes(&bytes);
            if let Some(ref spec) = stage.output {
                write_output_file(spec, &clean, nav)?;
            } else {
                carry = Some(clean);
            }
        } else if let Some(ref spec) = stage.output {
            // Streamed stage that still redirected (defensive): write nothing.
            write_output_file(spec, &[], nav)?;
        }

        if exit {
            break;
        }
    }

    Ok((exit, ok))
}

fn read_input_file(raw: &str, nav: &Navigator) -> Result<Vec<u8>> {
    let path = resolve_path(raw, nav);
    std::fs::read(&path).map_err(|e| anyhow!("cannot read '{}': {}", path.display(), e))
}

fn write_output_file(spec: &OutSpec, bytes: &[u8], nav: &Navigator) -> Result<()> {
    let (raw, append) = match spec {
        OutSpec::Truncate(p) => (p, false),
        OutSpec::Append(p) => (p, true),
    };
    let path = resolve_path(raw, nav);
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create(true);
    if append {
        opts.append(true);
    } else {
        opts.truncate(true);
    }
    let mut f = opts
        .open(&path)
        .map_err(|e| anyhow!("cannot write '{}': {}", path.display(), e))?;
    use std::io::Write;
    f.write_all(bytes)
        .map_err(|e| anyhow!("cannot write '{}': {}", path.display(), e))?;
    Ok(())
}

fn resolve_path(raw: &str, nav: &Navigator) -> PathBuf {
    let p = PathBuf::from(raw);
    if p.is_absolute() {
        p
    } else {
        nav.current_path().join(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer_keeps_quotes_and_brackets() {
        let t = tokenize(r#"say "a b" [x, y] z"#);
        assert_eq!(t, vec!["say", "\"a b\"", "[x, y]", "z"]);
    }

    #[test]
    fn operator_must_be_spaced() {
        assert!(!has_operators("nsheet data.csv --filter age>30"));
        assert!(!has_operators("math a>b"));
        assert!(has_operators("say hi > out.txt"));
        assert!(has_operators("gs main | say"));
        assert!(has_operators("a && b"));
        assert!(has_operators("a || b"));
    }

    #[test]
    fn operator_inside_quotes_is_data() {
        assert!(!has_operators(r#"say "a > b""#));
        assert!(!has_operators(r#"say "x | y""#));
    }

    #[test]
    fn attached_heredoc_operator_splits() {
        let t = tokenize("say <<EOF");
        assert_eq!(t, vec!["say", "<<", "EOF"]);
        assert_eq!(scan_heredoc_delims("say <<EOF"), vec!["EOF"]);
        assert!(has_operators("say <<EOF"));
        // `a<b` stays a single data token.
        assert!(!has_operators("math a<b"));
    }

    #[test]
    fn split_commands_separates_logic_and_pipes() {
        let parts = split_commands("a | b && c || d");
        let seps: Vec<_> = parts.iter().map(|(o, _)| o.clone()).collect();
        assert_eq!(
            seps,
            vec![None, Some("|".into()), Some("&&".into()), Some("||".into())]
        );
        assert_eq!(parts[0].1, "a");
        assert_eq!(parts[1].1, "b");
        assert_eq!(parts[2].1, "c");
        assert_eq!(parts[3].1, "d");
    }

    #[test]
    fn parse_pipeline_and_redirects() {
        set_heredoc_bodies(vec![]);
        let segs = parse("gs main > out.txt | sort");
        assert_eq!(segs.len(), 1);
        let stages = &segs[0].1;
        assert_eq!(stages.len(), 2);
        assert_eq!(stages[0].cmd, "gs main");
        match &stages[0].output {
            Some(OutSpec::Truncate(p)) => assert_eq!(p, "out.txt"),
            _ => panic!("expected truncate redirect"),
        }
        assert_eq!(stages[1].cmd, "sort");
    }

    #[test]
    fn parse_heredoc_consumes_body() {
        set_heredoc_bodies(vec![b"line1\nline2\n".to_vec()]);
        let segs = parse("say <<EOF");
        let stages = &segs[0].1;
        match &stages[0].input {
            Some(InSpec::Heredoc(b)) => assert_eq!(b, b"line1\nline2\n"),
            _ => panic!("expected heredoc input"),
        }
    }

    #[test]
    fn parse_logic_ops_and_append() {
        set_heredoc_bodies(vec![]);
        let segs = parse("a >> f && b || c");
        assert_eq!(segs.len(), 3);
        assert!(segs[0].0.is_none());
        assert_eq!(segs[0].1[0].cmd, "a");
        match &segs[0].1[0].output {
            Some(OutSpec::Append(p)) => assert_eq!(p, "f"),
            _ => panic!("expected append redirect"),
        }
        assert!(matches!(segs[1].0.as_ref(), Some(LogicOp::And)));
        assert_eq!(segs[1].1[0].cmd, "b");
        assert!(matches!(segs[2].0.as_ref(), Some(LogicOp::Or)));
        assert_eq!(segs[2].1[0].cmd, "c");
    }

    #[test]
    fn parse_input_file_and_here_string() {
        set_heredoc_bodies(vec![]);
        let segs = parse("sort < in.txt");
        match &segs[0].1[0].input {
            Some(InSpec::File(p)) => assert_eq!(p, "in.txt"),
            _ => panic!("expected file input"),
        }
        let segs = parse("say <<< hello");
        match &segs[0].1[0].input {
            Some(InSpec::Heredoc(b)) => assert_eq!(b, b"hello"),
            _ => panic!("expected here-string input"),
        }
    }

    #[test]
    fn quoted_operators_are_not_split_as_commands() {
        let parts = split_commands(r#"say "a && b | c""#);
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].0, None);
        assert_eq!(parts[0].1, r#"say "a && b | c""#);
    }

    #[test]
    fn strip_ansi_removes_colour_codes() {
        let raw = b"\x1b[31mred\x1b[0m plain";
        assert_eq!(strip_ansi_bytes(raw), b"red plain".to_vec());
    }

    #[test]
    fn active_operator_finds_last_operator_and_tail() {
        assert_eq!(
            active_operator("gs main | sort -u"),
            Some(("|", " sort -u"))
        );
        assert_eq!(
            active_operator("gs main > out.txt"),
            Some((">", " out.txt"))
        );
        assert_eq!(active_operator("gs main >> log"), Some((">>", " log")));
        assert_eq!(active_operator("sort < in.txt"), Some(("<", " in.txt")));
        assert_eq!(active_operator("say <<EOF"), Some(("<<", "EOF")));
        assert_eq!(active_operator("say <<< hi"), Some(("<<<", " hi")));
        assert_eq!(active_operator("a && b"), Some(("&&", " b")));
        assert_eq!(active_operator("a || b"), Some(("||", " b")));
        // The last operator wins.
        assert_eq!(
            active_operator("gs main > out.txt | sort"),
            Some(("|", " sort"))
        );
        // No operator, or one acting as data.
        assert_eq!(active_operator("gs main"), None);
        assert_eq!(active_operator(r#"say "a > b""#), None);
        assert_eq!(active_operator("nsheet x --filter age>30"), None);
        assert_eq!(active_operator("math a|b"), None);
        assert_eq!(active_operator("[a > b] say"), None);
    }

    #[test]
    fn attached_heredoc_keeps_delimiter_in_tail() {
        assert_eq!(active_operator("say <<"), Some(("<<", "")));
        assert_eq!(active_operator("say <<EOF"), Some(("<<", "EOF")));
        assert_eq!(active_operator("say <<<hello"), Some(("<<<", "hello")));
    }
}
