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
    let line_start = prefix.rfind('\n').map(|i| i + 1).unwrap_or(0);
    // Convert byte offset within line to UTF-16 code units
    let line_text = &text[line_start..byte_offset];
    let col: u32 = line_text.chars().map(|c| c.len_utf16() as u32).sum();
    (line, col)
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
        "PHI" | "phi" => Some(1.618_033_988_749_895),
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
            content_length = len_str.trim().parse::<usize>().unwrap_or(0);
        }
    }

    if content_length == 0 || content_length > 10_485_760 {
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
    run_server_inner(None)
}

// ── diagnostics publishing ───────────────────────────────────────────────────

fn publish_diagnostics(
    writer: &mut impl Write,
    store: &DocumentStore,
    uri: &str,
    _logger: &LspLogger,
) -> Result<()> {
    let mut diagnostics = Vec::new();
    let content = store.get(uri).unwrap_or("");

    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }
        let line_byte_offset = content
            .lines()
            .take(line_idx)
            .map(|l| l.len() + {
                let rest = &content[l.len()..];
                if rest.starts_with("\r\n") { 2 } else { 1 }
            })
            .sum::<usize>();

        match math::validate(trimmed) {
            Ok(()) => {}
            Err((msg, offset_in_line)) => {
                let absolute_offset = line_byte_offset + offset_in_line;
                diagnostics.push(make_diagnostic(content, &msg, absolute_offset, 1));
            }
        }
    }

    _logger.log(format_args!("publishing {} diagnostics for {}", diagnostics.len(), uri));

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
    // Convert UTF-16 code unit column to byte offset within the line
    let line_text = &text[byte_offset..];
    let line_end = line_text.find('\n').unwrap_or(line_text.len());
    let line_slice = &line_text[..line_end];
    let mut utf16_units = 0usize;
    for (i, ch) in line_slice.char_indices() {
        let w = ch.len_utf16();
        if utf16_units + w > col {
            return byte_offset + i;
        }
        utf16_units += w;
    }
    byte_offset + line_end
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

// ── optional debug logging ────────────────────────────────────────────────────

pub struct LspLogger {
    file: Option<std::fs::File>,
}

impl LspLogger {
    pub fn new(path: Option<&Path>) -> Self {
        let file = path.and_then(|p| {
            if let Some(parent) = p.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            std::fs::File::create(p).ok()
        });
        Self { file }
    }

    pub fn log(&self, msg: std::fmt::Arguments<'_>) {
        use std::io::Write;
        if let Some(ref f) = self.file {
            let mut f = f;
            let _ = writeln!(f, "[{}] {}", chrono::offset::Local::now().format("%H:%M:%S%.3f"), msg);
            let _ = f.flush();
        }
    }
}

pub fn run_server_with_logger(log_path: Option<&Path>) -> Result<()> {
    run_server_inner(log_path)
}

fn run_server_inner(log_path: Option<&Path>) -> Result<()> {
    let logger = LspLogger::new(log_path);
    let mut store = DocumentStore::new();
    let mut reader = io::BufReader::new(io::stdin());
    let mut writer = io::stdout();

    logger.log(format_args!("LSP server starting"));

    loop {
        logger.log(format_args!("waiting for message..."));
        let msg = match read_message(&mut reader)? {
            Some(m) => m,
            None => {
                logger.log(format_args!("stdin closed, shutting down"));
                return Ok(());
            }
        };

        logger.log(format_args!("received: {}", serde_json::to_string(&msg).unwrap_or_default()));

        let method = msg["method"].as_str().unwrap_or("");
        let id = msg.get("id").cloned();

        match method {
            "initialize" => {
                let caps = serde_json::json!({
                    "capabilities": {
                        "textDocumentSync": {
                            "openClose": true,
                            "change": 2,
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
                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": caps
                    });
                    write_message(&mut writer, &resp)?;
                    logger.log(format_args!("sent: {}", serde_json::to_string(&resp).unwrap_or_default()));
                }
            }

            "shutdown" => {
                logger.log(format_args!("shutdown requested"));
                if let Some(req_id) = &id {
                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": null
                    });
                    write_message(&mut writer, &resp)?;
                    logger.log(format_args!("sent: shutdown response"));
                }
            }

            "exit" => {
                logger.log(format_args!("exit requested, shutting down"));
                return Ok(());
            }

            "textDocument/didOpen" => {
                if let Some(params) = msg.get("params") {
                    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
                    let text = params["textDocument"]["text"].as_str().unwrap_or("");
                    if is_ntc_math_uri(uri) {
                        store.open(uri, text);
                        publish_diagnostics(&mut writer, &store, uri, &logger)?;
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
                        publish_diagnostics(&mut writer, &store, uri, &logger)?;
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
                        publish_diagnostics(&mut writer, &store, uri, &logger)?;
                    }
                }
            }

            "textDocument/didClose" => {
                if let Some(params) = msg.get("params") {
                    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
                    if is_ntc_math_uri(uri) {
                        store.close(uri);
                        let diag_notif = serde_json::json!({
                            "jsonrpc": "2.0",
                            "method": "textDocument/publishDiagnostics",
                            "params": {
                                "uri": uri,
                                "diagnostics": []
                            }
                        });
                        write_message(&mut writer, &diag_notif)?;
                        logger.log(format_args!("cleared diagnostics for {}", uri));
                    }
                }
            }

            "textDocument/completion" => {
                if let Some(req_id) = &id {
                    let mut items = Vec::new();

                    let params = msg.get("params");
                    let uri = params.and_then(|p| p["textDocument"]["uri"].as_str()).unwrap_or("");
                    let is_math = is_ntc_math_uri(uri);

                    if is_math {
                        for name in math::builtin_function_names() {
                            items.push(serde_json::json!({
                                "label": name,
                                "kind": 3,
                                "detail": builtin_hover(name).unwrap_or(""),
                                "insertText": name
                            }));
                        }

                        for name in math::constant_names() {
                            if let Some(val) = constant_value(name) {
                                items.push(serde_json::json!({
                                    "label": name,
                                    "kind": 21,
                                    "detail": format!("= {}", val),
                                    "insertText": name
                                }));
                            }
                        }

                        let cfg_guard = Config::read_global();
                        for (name, def) in &cfg_guard.math_functions {
                            items.push(serde_json::json!({
                                "label": name,
                                "kind": 3,
                                "detail": format!("user-defined: {}", def),
                                "insertText": name
                            }));
                        }
                        drop(cfg_guard);
                    }

                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": {
                            "isIncomplete": false,
                            "items": items
                        }
                    });
                    write_message(&mut writer, &resp)?;
                    logger.log(format_args!("completions: {} items for {}", items.len(), uri));
                }
            }

            "$cancellation" | "$/cancelRequest" => {
                logger.log(format_args!("cancellation request"));
            }

            "textDocument/hover" => {
                if let Some(req_id) = &id {
                    let params = msg.get("params");
                    let uri = params.and_then(|p| p["textDocument"]["uri"].as_str()).unwrap_or("");
                    let pos_line: usize = params.and_then(|p| p["position"]["line"].as_u64()).and_then(|v| v.try_into().ok()).unwrap_or(0);
                    let pos_col: usize = params.and_then(|p| p["position"]["character"].as_u64()).and_then(|v| v.try_into().ok()).unwrap_or(0);

                    let mut hover_contents: Option<String> = None;

                    if is_ntc_math_uri(uri) {
                        if let Some(text) = store.get(uri) {
                            let byte_offset = pos_to_byte_offset(text, pos_line, pos_col);
                            if let Some(word) = word_at_offset(text, byte_offset) {
                                let lower = word.to_lowercase();

                                if let Some(doc) = builtin_hover(&lower) {
                                    hover_contents = Some(format!("**{}**  \n{}", word, doc));
                                }

                                if hover_contents.is_none() {
                                    if let Some(val) = constant_value(word) {
                                        hover_contents = Some(format!("**{}** = {}", word, val));
                                    }
                                }

                                if hover_contents.is_none() {
                                    let cfg_guard = Config::read_global();
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

                    let result = if let Some(ref contents) = hover_contents {
                        serde_json::json!({
                            "contents": {
                                "kind": "markdown",
                                "value": contents
                            }
                        })
                    } else {
                        serde_json::json!(null)
                    };

                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": result
                    });
                    write_message(&mut writer, &resp)?;
                    logger.log(format_args!("hover for {}:{}:{} -> {:?}", uri, pos_line, pos_col, hover_contents));
                }
            }

            "textDocument/definition" => {
                if let Some(req_id) = &id {
                    let params = msg.get("params");
                    let uri = params.and_then(|p| p["textDocument"]["uri"].as_str()).unwrap_or("");
                    let pos_line: usize = params.and_then(|p| p["position"]["line"].as_u64()).and_then(|v| v.try_into().ok()).unwrap_or(0);
                    let pos_col: usize = params.and_then(|p| p["position"]["character"].as_u64()).and_then(|v| v.try_into().ok()).unwrap_or(0);

                    let mut locations = Vec::new();

                    if is_ntc_math_uri(uri) {
                        if let Some(text) = store.get(uri) {
                            let byte_offset = pos_to_byte_offset(text, pos_line, pos_col);
                            if let Some(word) = word_at_offset(text, byte_offset) {
                                let lower = word.to_lowercase();

                                let cfg_guard = Config::read_global();
                                if cfg_guard.math_functions.contains_key(&lower) {
                                    let config_path = dirs::config_dir().map(|d| d.join("ntc").join("config.toml"));
                                    if let Some(path) = config_path {
                                        let file_uri = path_to_uri(&path);
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

                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": result
                    });
                    write_message(&mut writer, &resp)?;
                    logger.log(format_args!("definition for {}:{}:{} -> {} locations", uri, pos_line, pos_col, locations.len()));
                }
            }

            _ => {
                logger.log(format_args!("unhandled method: {}", method));
                if let Some(req_id) = &id {
                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": null
                    });
                    write_message(&mut writer, &resp)?;
                }
            }
        }
    }
}
