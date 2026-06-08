// src/lsp/mod.rs
// Minimal LSP server for .ntc.math files using manual JSON-RPC over stdio.

use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::path::Path;

use anyhow::Result;
use serde_json::Value;

use crate::config::Config;
use crate::math;

// ── document store ──────────────────────────────────────────────────────────

struct DocumentStore {
    docs: HashMap<String, String>, // uri → content
}

impl DocumentStore {
    fn new() -> Self {
        Self {
            docs: HashMap::new(),
        }
    }

    fn open(&mut self, uri: &str, text: &str) {
        self.docs.insert(uri.to_string(), text.to_string());
    }

    fn change(&mut self, uri: &str, text: &str) {
        self.docs.insert(uri.to_string(), text.to_string());
    }

    fn close(&mut self, uri: &str) {
        self.docs.remove(uri);
    }

    fn get(&self, uri: &str) -> Option<&str> {
        self.docs.get(uri).map(|s| s.as_str())
    }
}

// ── LSP helpers ─────────────────────────────────────────────────────────────

fn byte_to_lsp_position(text: &str, byte_offset: usize) -> (u32, u32) {
    let byte_offset = byte_offset.min(text.len());
    let prefix = &text[..byte_offset];
    let line = prefix.matches('\n').count() as u32;
    let col = byte_offset - prefix.rfind('\n').map(|i| i + 1).unwrap_or(0);
    (line, col as u32)
}

fn make_diagnostic(
    text: &str,
    message: &str,
    byte_offset: usize,
    severity: u32,
) -> Value {
    let (line, col) = byte_to_lsp_position(text, byte_offset);
    // Estimate range: from the error position to end of line
    let line_end = text[byte_offset..].find('\n').map(|i| byte_offset + i).unwrap_or(text.len());
    let (end_line, end_col) = byte_to_lsp_position(text, line_end);

    serde_json::json!({
        "range": {
            "start": { "line": line, "character": col },
            "end": { "line": end_line, "character": end_col }
        },
        "severity": severity,
        "message": message,
        "source": "ntc-math"
    })
}

fn builtin_hover(name: &str) -> Option<&'static str> {
    match name {
        "sin" => Some("sin(x) → sine of x (radians)"),
        "cos" => Some("cos(x) → cosine of x (radians)"),
        "tan" => Some("tan(x) → tangent of x (radians)"),
        "cot" => Some("cot(x) → cotangent of x (radians)"),
        "sec" => Some("sec(x) → secant of x (radians)"),
        "csc" => Some("csc(x) → cosecant of x (radians)"),
        "asin" | "arcsin" => Some("asin(x) / arcsin(x) → inverse sine"),
        "acos" | "arccos" => Some("acos(x) / arccos(x) → inverse cosine"),
        "atan" | "arctan" => Some("atan(x) / arctan(x) → inverse tangent"),
        "acot" | "arccot" => Some("acot(x) / arccot(x) → inverse cotangent"),
        "asec" | "arcsec" => Some("asec(x) / arcsec(x) → inverse secant"),
        "acsc" | "arccsc" => Some("acsc(x) / arccsc(x) → inverse cosecant"),
        "sqrt" => Some("sqrt(x) → square root"),
        "pow" => Some("pow(x, y) → x raised to power y"),
        "abs" => Some("abs(x) → absolute value"),
        "floor" => Some("floor(x) → round down"),
        "ceil" | "ceiling" => Some("ceil(x) / ceiling(x) → round up"),
        "round" => Some("round(x) → nearest integer"),
        "ln" | "log" => Some("ln(x) / log(x) → natural logarithm"),
        "log2" => Some("log2(x) → base-2 logarithm"),
        "log10" => Some("log10(x) → base-10 logarithm"),
        "sum" => Some("sum(a, b, c, ...) → sum of values"),
        "min" => Some("min(a, b, c, ...) → minimum value"),
        "max" => Some("max(a, b, c, ...) → maximum value"),
        "avg" | "average" | "mean" => Some("avg(x, ...) / average(x, ...) / mean(x, ...) → arithmetic mean"),
        "rand" | "random" => Some("rand(min, max) → random integer in [min, max]"),
        "tobinary" => Some("toBinary(n) → binary string representation"),
        "tohex" => Some("toHex(n) → hex string representation"),
        "to8" | "tooctal" => Some("to8(n) / toOctal(n) → octal string representation"),
        "todecimal" => Some("toDecimal(n) → numeric value"),
        "tohb" | "tohumanbytes" | "tohumanreadable" => {
            Some("toHB(n) / toHumanBytes(n) → human-readable byte size")
        }
        _ => None,
    }
}

fn constant_value(name: &str) -> Option<f64> {
    match name {
        "PI" | "pi" => Some(std::f64::consts::PI),
        "E" | "e" | "EXP" | "exp" => Some(std::f64::consts::E),
        "PHI" | "phi" => Some(1.6180339887498948482045868343656381177),
        "TAU" | "tau" => Some(std::f64::consts::TAU),
        _ => None,
    }
}

fn is_ntc_math_uri(uri: &str) -> bool {
    uri.ends_with(".ntc.math") || uri.ends_with(".math")
}

// ── JSON-RPC message I/O ─────────────────────────────────────────────────────

fn read_message(reader: &mut impl BufRead) -> Result<Option<Value>> {
    let mut header = String::new();
    loop {
        header.clear();
        if reader.read_line(&mut header)? == 0 {
            return Ok(None);
        }
        let trimmed = header.trim();
        if trimmed.is_empty() {
            break;
        }
    }

    // Read Content-Length (we don't read Content-Type)
    let mut content_length: usize = 0;
    loop {
        header.clear();
        if reader.read_line(&mut header)? == 0 {
            return Ok(None);
        }
        let trimmed = header.trim();
        if trimmed.is_empty() {
            break;
        }
        if let Some(len_str) = trimmed.strip_prefix("Content-Length: ") {
            content_length = len_str.trim().parse::<usize>()?;
        }
    }

    if content_length == 0 {
        return Ok(None);
    }

    let mut buf = vec![0u8; content_length];
    let mut read = 0;
    while read < content_length {
        let n = reader.read(&mut buf[read..])?;
        if n == 0 {
            return Ok(None);
        }
        read += n;
    }

    let body = String::from_utf8(buf)?;
    Ok(Some(serde_json::from_str(&body)?))
}

fn write_message(writer: &mut impl Write, msg: &Value) -> Result<()> {
    let body = serde_json::to_string(msg)?;
    write!(writer, "Content-Length: {}\r\n\r\n{}", body.len(), body)?;
    writer.flush()?;
    Ok(())
}

// ── server loop ──────────────────────────────────────────────────────────────

pub fn run_server() -> Result<()> {
    let mut store = DocumentStore::new();
    let mut reader = io::BufReader::new(io::stdin());
    let mut writer = io::stdout();

    loop {
        let msg = match read_message(&mut reader)? {
            Some(m) => m,
            None => return Ok(()),
        };

        let method = msg["method"].as_str().unwrap_or("");
        let _is_notification = msg.get("id").map_or(true, |v| v.is_null());
        let id = msg.get("id").cloned();

        match method {
            "initialize" => {
                let caps = serde_json::json!({
                    "capabilities": {
                        "textDocumentSync": {
                            "openClose": true,
                            "change": 2, // Full text sync
                            "save": true
                        },
                        "completionProvider": {
                            "triggerCharacters": [],
                            "resolveProvider": false
                        },
                        "hoverProvider": true,
                        "definitionProvider": true
                    },
                    "serverInfo": {
                        "name": "ntc-math-lsp",
                        "version": "2.1.0"
                    }
                });
                if let Some(req_id) = &id {
                    write_message(
                        &mut writer,
                        &serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "result": caps
                        }),
                    )?;
                }
            }

            "shutdown" => {
                if let Some(req_id) = &id {
                    write_message(
                        &mut writer,
                        &serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "result": null
                        }),
                    )?;
                }
            }

            "exit" => {
                return Ok(());
            }

            "textDocument/didOpen" => {
                if let Some(params) = msg.get("params") {
                    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
                    let text = params["textDocument"]["text"].as_str().unwrap_or("");
                    if is_ntc_math_uri(uri) {
                        store.open(uri, text);
                        publish_diagnostics(&mut writer, &store, uri)?;
                    }
                }
            }

            "textDocument/didChange" => {
                if let Some(params) = msg.get("params") {
                    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
                    if is_ntc_math_uri(uri) {
                        if let Some(content_changes) = params["contentChanges"].as_array() {
                            if let Some(change) = content_changes.last() {
                                let text = change["text"].as_str().unwrap_or("");
                                store.change(uri, text);
                            }
                        }
                        publish_diagnostics(&mut writer, &store, uri)?;
                    }
                }
            }

            "textDocument/didSave" => {
                if let Some(params) = msg.get("params") {
                    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
                    if is_ntc_math_uri(uri) {
                        if let Some(text) = params.get("text").and_then(|t| t.as_str()) {
                            store.change(uri, text);
                        }
                        publish_diagnostics(&mut writer, &store, uri)?;
                    }
                }
            }

            "textDocument/didClose" => {
                if let Some(params) = msg.get("params") {
                    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
                    if is_ntc_math_uri(uri) {
                        store.close(uri);
                        // Clear diagnostics on close
                        let diag_notif = serde_json::json!({
                            "jsonrpc": "2.0",
                            "method": "textDocument/publishDiagnostics",
                            "params": {
                                "uri": uri,
                                "diagnostics": []
                            }
                        });
                        write_message(&mut writer, &diag_notif)?;
                    }
                }
            }

            "textDocument/completion" => {
                if let Some(req_id) = &id {
                    let mut items = Vec::new();

                    // Built-in functions
                    for name in math::builtin_function_names() {
                        items.push(serde_json::json!({
                            "label": name,
                            "kind": 3, // Function
                            "detail": builtin_hover(name).unwrap_or(""),
                            "insertText": name
                        }));
                    }

                    // Constants
                    for name in math::constant_names() {
                        if let Some(val) = constant_value(name) {
                            items.push(serde_json::json!({
                                "label": name,
                                "kind": 21, // Constant
                                "detail": format!("= {}", val),
                                "insertText": name
                            }));
                        }
                    }

                    // User-defined functions from config
                    let cfg = Config::global();
                    let cfg_guard = cfg.read().unwrap();
                    for (name, def) in &cfg_guard.math_functions {
                        let detail = format!("user-defined: {}", def);
                        items.push(serde_json::json!({
                            "label": name,
                            "kind": 3, // Function
                            "detail": detail,
                            "insertText": name
                        }));
                    }
                    drop(cfg_guard);

                    write_message(
                        &mut writer,
                        &serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "result": {
                                "isIncomplete": false,
                                "items": items
                            }
                        }),
                    )?;
                }
            }

            "textDocument/hover" => {
                if let Some(req_id) = &id {
                    let params = msg.get("params");
                    let uri = params.and_then(|p| p["textDocument"]["uri"].as_str()).unwrap_or("");
                    let pos_line = params.and_then(|p| p["position"]["line"].as_u64()).unwrap_or(0) as usize;
                    let pos_col = params.and_then(|p| p["position"]["character"].as_u64()).unwrap_or(0) as usize;

                    let mut hover_contents: Option<String> = None;

                    if is_ntc_math_uri(uri) {
                        if let Some(text) = store.get(uri) {
                            // Find the word at the given position
                            let byte_offset = pos_to_byte_offset(text, pos_line, pos_col);
                            if let Some(word) = word_at_offset(text, byte_offset) {
                                let lower = word.to_lowercase();

                                // Check built-in functions
                                if let Some(doc) = builtin_hover(&lower) {
                                    hover_contents = Some(format!("**{}**  \n{}", word, doc));
                                }

                                // Check constants
                                if hover_contents.is_none() {
                                    if let Some(val) = constant_value(word) {
                                        hover_contents = Some(format!("**{}** = {}", word, val));
                                    }
                                }

                                // Check user-defined functions
                                if hover_contents.is_none() {
                                    let cfg = Config::global();
                                    let cfg_guard = cfg.read().unwrap();
                                    if let Some(def) = cfg_guard.math_functions.get(&lower) {
                                        let arrow = def.find("=>").unwrap_or(0);
                                        let params = def[..arrow].trim();
                                        let body = def[arrow + 2..].trim();
                                        hover_contents = Some(format!(
                                            "**{}({})**  \n```\n{}\n```",
                                            word, params, body
                                        ));
                                    }
                                    drop(cfg_guard);
                                }
                            }
                        }
                    }

                    let result = if let Some(contents) = hover_contents {
                        serde_json::json!({
                            "contents": {
                                "kind": "markdown",
                                "value": contents
                            }
                        })
                    } else {
                        serde_json::json!(null)
                    };

                    write_message(
                        &mut writer,
                        &serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "result": result
                        }),
                    )?;
                }
            }

            "textDocument/definition" => {
                if let Some(req_id) = &id {
                    let params = msg.get("params");
                    let uri = params.and_then(|p| p["textDocument"]["uri"].as_str()).unwrap_or("");
                    let pos_line = params.and_then(|p| p["position"]["line"].as_u64()).unwrap_or(0) as usize;
                    let pos_col = params.and_then(|p| p["position"]["character"].as_u64()).unwrap_or(0) as usize;

                    let mut locations = Vec::new();

                    if is_ntc_math_uri(uri) {
                        if let Some(text) = store.get(uri) {
                            let byte_offset = pos_to_byte_offset(text, pos_line, pos_col);
                            if let Some(word) = word_at_offset(text, byte_offset) {
                                let lower = word.to_lowercase();

                                // Check user-defined functions (defined in config.toml)
                                let cfg = Config::global();
                                let cfg_guard = cfg.read().unwrap();
                                if cfg_guard.math_functions.contains_key(&lower) {
                                    // Point to the config file where the function is defined
                                    let config_path = dirs::config_dir().map(|d| d.join("ntc").join("config.toml"));
                                    if let Some(path) = config_path {
                                        let file_uri = path_to_uri(&path);
                                        // We don't know the exact line, so point to line 0
                                        locations.push(serde_json::json!({
                                            "uri": file_uri,
                                            "range": {
                                                "start": { "line": 0, "character": 0 },
                                                "end": { "line": 0, "character": 0 }
                                            }
                                        }));
                                    }
                                }
                                drop(cfg_guard);
                            }
                        }
                    }

                    let result = if locations.is_empty() {
                        serde_json::json!(null)
                    } else {
                        serde_json::json!(locations)
                    };

                    write_message(
                        &mut writer,
                        &serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "result": result
                        }),
                    )?;
                }
            }

            // For any unhandled method, respond with null
            _ => {
                if let Some(req_id) = &id {
                    write_message(
                        &mut writer,
                        &serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": req_id,
                            "result": null
                        }),
                    )?;
                }
            }
        }
    }
}

// ── diagnostics publishing ───────────────────────────────────────────────────

fn publish_diagnostics(
    writer: &mut impl Write,
    store: &DocumentStore,
    uri: &str,
) -> Result<()> {
    let mut diagnostics = Vec::new();
    let content = store.get(uri).unwrap_or("");

    // Validate each non-empty, non-comment line
    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }
        // Compute the byte offset of this line's start within the full content
        let line_byte_offset = content
            .lines()
            .take(line_idx)
            .map(|l| l.len() + 1) // +1 for newline
            .sum::<usize>();

        // Validate this line
        match math::validate(trimmed) {
            Ok(()) => {}
            Err((msg, offset_in_line)) => {
                let absolute_offset = line_byte_offset + offset_in_line;
                diagnostics.push(make_diagnostic(content, &msg, absolute_offset, 1)); // Error severity
            }
        }
    }

    let notif = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/publishDiagnostics",
        "params": {
            "uri": uri,
            "diagnostics": diagnostics
        }
    });

    write_message(writer, &notif)?;
    Ok(())
}

// ── position helpers ─────────────────────────────────────────────────────────

fn pos_to_byte_offset(text: &str, line: usize, col: usize) -> usize {
    let mut byte_offset = 0;
    for _ in 0..line {
        match text[byte_offset..].find('\n') {
            Some(n) => byte_offset += n + 1,
            None => return text.len(),
        }
    }
    byte_offset + col.min(text[byte_offset..].len())
}

fn word_at_offset(text: &str, byte_offset: usize) -> Option<&str> {
    if byte_offset >= text.len() {
        return None;
    }
    let prefix = &text[..byte_offset];
    let suffix = &text[byte_offset..];

    let start = prefix
        .chars()
        .rev()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .map(|c| c.len_utf8())
        .sum::<usize>();

    let end = suffix
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .map(|c| c.len_utf8())
        .sum::<usize>();

    if start == 0 && end == 0 {
        return None;
    }

    Some(&text[byte_offset - start..byte_offset + end])
}

fn path_to_uri(path: &Path) -> String {
    let abs = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_default()
            .join(path)
    };
    let path_str = abs.to_string_lossy().replace('\\', "/");
    if path_str.starts_with('/') {
        format!("file://{}", path_str)
    } else {
        format!("file:///{}", path_str)
    }
}
