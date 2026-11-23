//! `serve theme` — browser UI for inspecting and editing theme TOML.
//!
//! The page is embedded (no build step) and talks to a small JSON API under
//! `/api`. All writes stay inside `ThemeManager::themes_dir()`; raw TOML is
//! parsed before it is accepted, so a broken save cannot make the shell
//! unable to load themes.

use super::share::public_base_url;
use crate::config::{Config, ServeService};
use crate::navigator::Navigator;
use crate::output::{print_error, print_info};
use crate::shell::commands::support::args::is_help_request;
use crate::shell::commands::support::net::http::{respond, url_decode};
use crate::shell::commands::support::net::tokenize;
use crate::shell::helpers::nav_view_snapshot_all;
use crate::shell::prompt_highlight::themed_field_spans;
use crate::t;
use crate::utils::syntax::{SyntaxHighlighter, TokenType};
use crate::utils::theme::style::ThemeColorize;
use crate::utils::theme::{Theme, ThemeManager};
use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const MAX_BODY: usize = 2 * 1024 * 1024;
const PAGE: &str = include_str!("assets/theme_web.html");
const HELP_GOLDEN: &str = include_str!("../../../../shell/help_golden/full.golden");
const EDITOR_SAMPLE: &str = "fn paint(theme: &Theme) {\n    // Live colors from the active theme\n    let accent = theme.syntax.function;\n    println!(\"editor preview: {accent:?}\");\n}\n";
const SYNTAX_SAMPLE: &str = "// syntax tokens\n#[derive(Debug)]\npub struct Theme { name: String }\n\nfn load(path: &str) -> Result<Theme, Error> {\n    let count = 42;\n    let pattern = r#\"^theme_[a-z]+$\"#;\n    if count > 0 { Ok(Theme { name: path.into() }) } else { Err(Error::Empty) }\n}\n";

pub fn cmd_serve_theme(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if is_help_request(&tokens) {
        print_theme_help();
        return Ok(false);
    }

    // Defaults come from `serve config theme` (alias: `config serve theme`).
    let mut port = Config::global_get_serve_port(ServeService::Theme);
    let mut bind = Config::global_get_serve_bind(ServeService::Theme);
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "-p" | "--port" => {
                i += 1;
                port = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.serve.missing_port")))?
                    .parse()
                    .map_err(|_| anyhow::anyhow!("{}", t!("cmd.serve.invalid_port")))?;
            }
            "-b" | "--bind" => {
                i += 1;
                bind = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.serve.missing_bind")))?
                    .clone();
            }
            other => {
                anyhow::bail!("{}", t!("cmd.serve.unknown_option", other));
            }
        }
        i += 1;
    }

    let themes_dir = ThemeManager::themes_dir().context("Failed to get themes directory")?;
    std::fs::create_dir_all(&themes_dir)
        .with_context(|| format!("Create {}", themes_dir.display()))?;

    let addr = format!("{}:{}", bind, port);
    let listener = TcpListener::bind(&addr).context(format!("Bind to {}", addr))?;
    print_info(&t!(
        "cmd.serve.theme_serving",
        display_path(&themes_dir),
        public_base_url(&bind, port)
    ));
    if !is_loopback(&bind) {
        print_info(&t!("cmd.serve.theme_local_warning"));
    }

    let shutdown = Arc::new(AtomicBool::new(false));
    crate::shell::commands::init_ctrlc_handler();
    crate::shell::commands::register_shutdown_flag(shutdown.clone());
    listener.set_nonblocking(true)?;
    while !shutdown.load(Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => {
                let dir = themes_dir.clone();
                std::thread::spawn(move || handle(stream, &dir));
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
            Err(e) => print_error(&format!("Accept: {}", e)),
        }
    }
    print_info(&t!("cmd.serve.stopped"));
    crate::shell::commands::unregister_shutdown_flag(&shutdown);
    crate::shell::commands::init_ctrlc_handler();
    Ok(false)
}

// ── Connection ────────────────────────────────────────────────────────────

fn handle(mut stream: TcpStream, themes_dir: &Path) {
    let _ = stream.set_nonblocking(false);
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }
    let request_line = request_line.trim_end().to_string();
    let mut parts = request_line.split_whitespace();
    let Some(method) = parts.next().map(|s| s.to_string()) else {
        return;
    };
    let Some(target) = parts.next().map(|s| s.to_string()) else {
        return;
    };
    let (raw_path, query) = target.split_once('?').unwrap_or((&target, ""));

    let mut content_length = 0usize;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() {
            return;
        }
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            if k.trim().eq_ignore_ascii_case("content-length") {
                content_length = v.trim().parse().unwrap_or(0);
            }
        }
    }
    if content_length > MAX_BODY {
        drop(reader);
        json_resp(
            &mut stream,
            413,
            json!({ "error": "request body too large" }),
        );
        return;
    }
    let mut body = vec![0u8; content_length];
    if content_length > 0 && reader.read_exact(&mut body).is_err() {
        return;
    }
    drop(reader);

    route(&mut stream, &method, raw_path, query, &body, themes_dir);
}

fn route(
    stream: &mut TcpStream,
    method: &str,
    raw_path: &str,
    query: &str,
    body: &[u8],
    themes_dir: &Path,
) {
    let path = url_decode(raw_path);
    match (method, path.as_str()) {
        ("GET", "/") | ("GET", "/index.html") => {
            respond(stream, 200, "text/html; charset=utf-8", PAGE.as_bytes());
        }
        ("GET", "/api/themes") => api_themes(stream),
        ("GET", "/api/theme") => api_theme(stream, query, themes_dir),
        ("GET", "/api/context") => api_context(stream),
        ("GET", "/api/help") => api_help(stream),
        ("POST", "/api/preview") => api_preview(stream, body),
        ("POST", "/api/parse") => api_parse(stream, body),
        ("POST", "/api/save") => api_save(stream, body, themes_dir),
        ("POST", "/api/update") => api_update(stream, body, themes_dir),
        ("POST", "/api/apply") => api_apply(stream, body),
        ("POST", "/api/create") => api_create(stream, body, themes_dir),
        ("POST", "/api/delete") => api_delete(stream, body),
        ("POST", "/api/reload") => api_reload(stream),
        _ => json_resp(stream, 404, json!({ "error": "not found" })),
    }
}

// ── API ───────────────────────────────────────────────────────────────────

fn api_themes(stream: &mut TcpStream) {
    let current = ThemeManager::current().name.clone();
    json_resp(
        stream,
        200,
        json!({ "themes": ThemeManager::list_themes(), "current": current }),
    );
}

fn api_theme(stream: &mut TcpStream, query: &str, themes_dir: &Path) {
    let name = query_param(query, "name").unwrap_or_default();
    let Some(path) = theme_path(themes_dir, &name) else {
        return json_resp(stream, 400, json!({ "error": "invalid theme name" }));
    };
    match read_theme_content(&path, &name) {
        Ok((content, exists)) => {
            let theme = match toml::from_str::<Theme>(&content) {
                Ok(theme) => theme,
                Err(e) => {
                    return json_resp(stream, 500, json!({ "error": e.to_string() }));
                }
            };
            json_resp(
                stream,
                200,
                json!({
                    "name": theme.name,
                    "path": display_path(&path),
                    "exists": exists,
                    "content": content,
                    "theme": theme,
                }),
            );
        }
        Err(e) => json_resp(stream, 404, json!({ "error": e })),
    }
}

fn api_context(stream: &mut TcpStream) {
    json_resp(stream, 200, json!({ "ok": true, "shell": shell_context() }));
}

/// The full help reference, styled by shell roles in the browser.
fn api_help(stream: &mut TcpStream) {
    let lines = HELP_GOLDEN
        .lines()
        .map(|line| {
            let role = if line.starts_with('╔') || line.starts_with('║') || line.starts_with('╚')
            {
                "help_header"
            } else if line.trim_end().ends_with(':') {
                "help_section"
            } else if line.starts_with("    ") {
                "help_example"
            } else {
                "normal"
            };
            json!({ "text": line, "role": role })
        })
        .collect::<Vec<_>>();
    json_resp(stream, 200, json!({ "ok": true, "lines": lines }));
}

fn api_preview(stream: &mut TcpStream, body: &[u8]) {
    let Some(content) = content_from_body(body) else {
        return json_resp(stream, 400, json!({ "error": "content is required" }));
    };
    let theme = match toml::from_str::<Theme>(&content) {
        Ok(theme) => theme,
        Err(e) => return json_resp(stream, 400, json!({ "error": e.to_string() })),
    };
    if !valid_theme_name(&theme.name) {
        return json_resp(stream, 400, json!({ "error": "invalid theme name" }));
    }
    let prompt = parse_body(body)
        .and_then(|value| {
            value
                .get("prompt")
                .and_then(|p| p.as_str())
                .map(str::to_string)
        })
        .unwrap_or_default();
    json_resp(
        stream,
        200,
        json!({
            "ok": true,
            "theme": theme,
            "shell": shell_context(),
            "prompt": prompt_segments(&prompt),
            "toml": syntax_lines(&content, "theme.ntc_theme"),
            "editor": syntax_lines(EDITOR_SAMPLE, "preview.rs"),
            "syntax": syntax_lines(SYNTAX_SAMPLE, "preview.rs"),
        }),
    );
}

fn api_parse(stream: &mut TcpStream, body: &[u8]) {
    let Some(content) = content_from_body(body) else {
        return json_resp(stream, 400, json!({ "error": "content is required" }));
    };
    match toml::from_str::<Theme>(&content) {
        Ok(theme) => {
            if !valid_theme_name(&theme.name) {
                return json_resp(stream, 400, json!({ "error": "invalid theme name" }));
            }
            json_resp(stream, 200, json!({ "ok": true, "theme": theme }));
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e.to_string() })),
    }
}

fn api_save(stream: &mut TcpStream, body: &[u8], themes_dir: &Path) {
    let Some(mut content) = content_from_body(body) else {
        return json_resp(stream, 400, json!({ "error": "content is required" }));
    };
    let initial = match toml::from_str::<Theme>(&content) {
        Ok(theme) => theme,
        Err(e) => return json_resp(stream, 400, json!({ "error": e.to_string() })),
    };
    if !valid_theme_name(&initial.name) {
        return json_resp(stream, 400, json!({ "error": "invalid theme name" }));
    }
    let Some(path) = theme_path(themes_dir, &initial.name) else {
        return json_resp(stream, 400, json!({ "error": "invalid theme name" }));
    };

    // Saving a partial theme fills in fields added by newer ntc releases.
    // Existing lines and comments are kept exactly as the user wrote them.
    let added = match ThemeManager::merge_missing_theme_fields(&content, &initial.name) {
        Ok((merged, missing)) => {
            content = merged;
            missing
        }
        Err(e) => return json_resp(stream, 400, json!({ "error": e.to_string() })),
    };

    let theme = match toml::from_str::<Theme>(&content) {
        Ok(theme) => theme,
        Err(e) => return json_resp(stream, 400, json!({ "error": e.to_string() })),
    };
    if let Err(e) = std::fs::write(&path, &content) {
        return json_resp(stream, 500, json!({ "error": e.to_string() }));
    }

    ThemeManager::reload_themes();
    let current = ThemeManager::current().name.clone();
    if current == theme.name {
        ThemeManager::set_theme(&theme.name);
    }
    json_resp(
        stream,
        200,
        json!({
            "ok": true,
            "name": theme.name,
            "path": display_path(&path),
            "content": content,
            "added": added,
            "current": ThemeManager::current().name.clone(),
        }),
    );
}

fn api_update(stream: &mut TcpStream, body: &[u8], themes_dir: &Path) {
    let Some(name) = name_from_body(body) else {
        return json_resp(stream, 400, json!({ "error": "name is required" }));
    };
    if !valid_theme_name(&name) {
        return json_resp(stream, 400, json!({ "error": "invalid theme name" }));
    }
    let Some(path) = theme_path(themes_dir, &name) else {
        return json_resp(stream, 400, json!({ "error": "invalid theme name" }));
    };
    if !path.is_file() {
        return json_resp(
            stream,
            404,
            json!({ "error": format!("theme '{}' has no theme file to update", name) }),
        );
    }
    let content = match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) => return json_resp(stream, 500, json!({ "error": e.to_string() })),
    };
    let (merged, added) = match ThemeManager::merge_missing_theme_fields(&content, &name) {
        Ok(merged) => merged,
        Err(e) => return json_resp(stream, 400, json!({ "error": e })),
    };
    if !added.is_empty() {
        if let Err(e) = std::fs::write(&path, &merged) {
            return json_resp(stream, 500, json!({ "error": e.to_string() }));
        }
        ThemeManager::reload_themes();
        let current = ThemeManager::current().name.clone();
        if current == name {
            ThemeManager::set_theme(&name);
        }
    }
    json_resp(
        stream,
        200,
        json!({
            "ok": true,
            "name": name,
            "path": display_path(&path),
            "added": added,
            "content": if added.is_empty() { content } else { merged },
            "current": ThemeManager::current().name.clone(),
        }),
    );
}

fn api_apply(stream: &mut TcpStream, body: &[u8]) {
    let Some(name) = name_from_body(body) else {
        return json_resp(stream, 400, json!({ "error": "name is required" }));
    };
    if !valid_theme_name(&name) {
        return json_resp(stream, 400, json!({ "error": "invalid theme name" }));
    }
    if !ThemeManager::set_theme(&name) {
        return json_resp(stream, 404, json!({ "error": "theme not found" }));
    }
    json_resp(
        stream,
        200,
        json!({ "ok": true, "current": ThemeManager::current().name.clone() }),
    );
}

fn api_create(stream: &mut TcpStream, body: &[u8], themes_dir: &Path) {
    let Some(name) = name_from_body(body) else {
        return json_resp(stream, 400, json!({ "error": "name is required" }));
    };
    if !valid_theme_name(&name) {
        return json_resp(stream, 400, json!({ "error": "invalid theme name" }));
    }
    let Some(path) = theme_path(themes_dir, &name) else {
        return json_resp(stream, 400, json!({ "error": "invalid theme name" }));
    };
    if path.exists() || ThemeManager::theme_exists(&name) {
        return json_resp(stream, 409, json!({ "error": "theme already exists" }));
    }
    let mut content = ThemeManager::get_example_theme_template();
    content = content.replace("name = \"mytheme\"", &format!("name = \"{}\"", name));
    if let Err(e) = std::fs::write(&path, &content) {
        return json_resp(stream, 500, json!({ "error": e.to_string() }));
    }
    ThemeManager::reload_themes();
    json_resp(
        stream,
        200,
        json!({
            "ok": true,
            "name": name,
            "path": display_path(&path),
            "content": content,
        }),
    );
}

fn api_delete(stream: &mut TcpStream, body: &[u8]) {
    let Some(name) = name_from_body(body) else {
        return json_resp(stream, 400, json!({ "error": "name is required" }));
    };
    match ThemeManager::remove_theme(&name) {
        Ok(()) => json_resp(
            stream,
            200,
            json!({
                "ok": true,
                "themes": ThemeManager::list_themes(),
                "current": ThemeManager::current().name.clone(),
            }),
        ),
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn api_reload(stream: &mut TcpStream) {
    ThemeManager::reload_themes();
    json_resp(
        stream,
        200,
        json!({
            "ok": true,
            "themes": ThemeManager::list_themes(),
            "current": ThemeManager::current().name.clone(),
        }),
    );
}

// ── Preview ───────────────────────────────────────────────────────────────

/// The shell context as semantic segments. The page applies the draft theme's
/// colors, while the content (path, tree, prompt) comes from the same
/// functions the real shell uses.
fn shell_context() -> Value {
    let cwd = std::env::current_dir().unwrap_or_default();
    let nav = Navigator::new().ok();
    let display_name = nav
        .as_ref()
        .map(|n| n.current_dir_name())
        .unwrap_or_else(|| "~".to_string());
    let venv = nav
        .as_ref()
        .map(|n| n.get_venv_prefix())
        .unwrap_or_default();
    let watcher_suffix = if Config::global_get_file_watcher_enabled() {
        " ~"
    } else {
        ""
    };
    let git_suffix = if Config::global_get_git_integration_enabled() {
        let fragment = crate::utils::git::prompt_fragment(&cwd);
        if fragment.is_empty() {
            String::new()
        } else {
            format!(" {}", fragment)
        }
    } else {
        String::new()
    };
    let prompt = format!(
        "{}ntc [{}{}{}]> ",
        venv, display_name, watcher_suffix, git_suffix
    );

    let snapshot = nav_view_snapshot_all(&cwd);
    let tree = snapshot
        .as_ref()
        .map(|snapshot| {
            snapshot
                .tree
                .lines()
                .map(|line| {
                    let role = if line.contains("[ignored]") {
                        "tree_ignored"
                    } else if line.contains("[Directory]") {
                        "tree_dir"
                    } else if line.trim().starts_with("├──") || line.trim().starts_with("└──")
                    {
                        "tree_branch"
                    } else {
                        "normal"
                    };
                    json!({ "text": line, "role": role })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let path = snapshot
        .as_ref()
        .map(|snapshot| snapshot.path.clone())
        .unwrap_or_else(|| crate::filetype::strip_unc_prefix(&cwd).to_string());

    json!({
        "cwd": path,
        "show_nav": snapshot.is_some(),
        "prompt": prompt,
        "separator": separator_segments(),
        "path": path,
        "tree": tree,
    })
}

fn separator_segments() -> Vec<Value> {
    let title = "Current Directory";
    let width = 71usize;
    let left = (width.saturating_sub(title.len())) / 2;
    let right = width.saturating_sub(title.len()).saturating_sub(left);
    vec![
        json!({ "text": "+", "role": "separator" }),
        json!({ "text": "-".repeat(left), "role": "separator" }),
        json!({ "text": title, "role": "help_header", "bold": true }),
        json!({ "text": "-".repeat(right), "role": "separator" }),
        json!({ "text": "+", "role": "separator" }),
    ]
}

fn prompt_segments(line: &str) -> Vec<Value> {
    if line.is_empty() {
        return Vec::new();
    }
    let spans = themed_field_spans(line);
    if spans.is_empty() {
        return vec![json!({ "text": line, "role": "normal" })];
    }
    let mut out = Vec::new();
    let mut cursor = 0usize;
    for (range, role) in spans {
        if range.start > cursor {
            out.push(json!({ "text": &line[cursor..range.start], "role": "normal" }));
        }
        if let Some(text) = line.get(range.clone()) {
            out.push(json!({ "text": text, "role": role }));
        }
        cursor = range.end;
    }
    if cursor < line.len() {
        out.push(json!({ "text": &line[cursor..], "role": "normal" }));
    }
    out
}

fn syntax_lines(content: &str, filename: &str) -> Vec<Value> {
    let mut highlighter = SyntaxHighlighter::for_path(Path::new(filename));
    content
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let tokens = highlighter.tokenize_line(idx, line);
            json!({ "segments": token_segments(line, tokens) })
        })
        .collect()
}

fn token_segments(line: &str, tokens: &[crate::utils::syntax::Token]) -> Vec<Value> {
    let mut out = Vec::new();
    let mut cursor = 0usize;
    for token in tokens {
        if token.start > cursor {
            if let Some(text) = line.get(cursor..token.start) {
                out.push(json!({ "text": text, "kind": "normal" }));
            }
        }
        let start = cursor.max(token.start);
        if token.end <= start {
            continue;
        }
        if let Some(text) = line.get(start..token.end) {
            out.push(json!({
                "text": text,
                "kind": token_type_name(token.token_type),
            }));
        }
        cursor = cursor.max(token.end);
    }
    if cursor < line.len() {
        if let Some(text) = line.get(cursor..) {
            out.push(json!({ "text": text, "kind": "normal" }));
        }
    }
    out
}

fn token_type_name(token: TokenType) -> &'static str {
    match token {
        TokenType::Normal => "normal",
        TokenType::Keyword => "keyword",
        TokenType::String => "string",
        TokenType::Comment => "comment",
        TokenType::Number => "number",
        TokenType::Type => "type",
        TokenType::Builtin => "builtin",
        TokenType::Function => "function",
        TokenType::Operator => "operator",
        TokenType::Punctuation => "punctuation",
        TokenType::Attribute => "attribute",
        TokenType::Macro => "macro_token",
        TokenType::Regex => "regex",
        TokenType::Tag => "tag",
        TokenType::Constant => "constant",
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────

fn json_resp(stream: &mut TcpStream, status: u16, value: Value) {
    let body = value.to_string();
    respond(
        stream,
        status,
        "application/json; charset=utf-8",
        body.as_bytes(),
    );
}

fn parse_body(body: &[u8]) -> Option<Value> {
    serde_json::from_slice(body).ok()
}

fn content_from_body(body: &[u8]) -> Option<String> {
    parse_body(body)?
        .get("content")?
        .as_str()
        .map(|s| s.to_string())
}

fn name_from_body(body: &[u8]) -> Option<String> {
    parse_body(body)?
        .get("name")?
        .as_str()
        .map(|s| s.to_string())
}

fn query_param(query: &str, key: &str) -> Option<String> {
    for pair in query.split('&') {
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        if url_decode(k) == key {
            return Some(url_decode(v));
        }
    }
    None
}

fn valid_theme_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

fn theme_path(themes_dir: &Path, name: &str) -> Option<PathBuf> {
    if !valid_theme_name(name) {
        return None;
    }
    Some(themes_dir.join(format!("{}.ntc_theme", name)))
}

fn read_theme_content(path: &Path, name: &str) -> Result<(String, bool), String> {
    if path.is_file() {
        return std::fs::read_to_string(path)
            .map(|content| (content, true))
            .map_err(|e| e.to_string());
    }
    let theme =
        ThemeManager::get_theme_info(name).ok_or_else(|| format!("theme '{}' not found", name))?;
    toml::to_string_pretty(&theme)
        .map(|content| (content, false))
        .map_err(|e| e.to_string())
}

fn is_loopback(bind: &str) -> bool {
    bind == "localhost" || bind == "::1" || bind.starts_with("127.")
}

/// Canonical Windows paths carry a `\\?\` verbatim prefix; strip it for display.
fn display_path(path: &Path) -> String {
    let shown = path.display().to_string();
    shown.strip_prefix("\\\\?\\").unwrap_or(&shown).to_string()
}

fn print_theme_help() {
    println!("{}", t!("cmd.serve.theme_title").bold());
    println!();
    println!("{}", t!("cmd.serve.usage_title").cyan().bold());
    println!("  {}", t!("cmd.serve.theme_usage"));
    println!();
    println!("{}", t!("cmd.serve.options_title").cyan().bold());
    println!(
        "  {}",
        t!(
            "cmd.serve.theme_help_port",
            Config::global_get_serve_port(ServeService::Theme)
        )
    );
    println!(
        "  {}",
        t!(
            "cmd.serve.theme_help_bind",
            Config::global_get_serve_bind(ServeService::Theme)
        )
    );
    println!("  {}", t!("cmd.serve.help_help"));
    println!();
    println!("{}", t!("cmd.serve.examples_title").cyan().bold());
    println!("  {}", t!("cmd.serve.theme_example"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_is_embedded() {
        assert!(PAGE.contains("NTC Theme Studio"));
        assert!(PAGE.contains("data-tab=\"shell\""));
        assert!(PAGE.contains("data-tab=\"editor\""));
        assert!(PAGE.contains("data-tab=\"syntax\""));
        assert!(PAGE.contains("data-tab=\"help\""));
        assert!(PAGE.contains("/api/help"));
        assert!(HELP_GOLDEN.contains("Interactive Help"));
        assert!(PAGE.contains("id=\"prompt-input\""));
        assert!(PAGE.contains("id=\"toml-highlight\""));
        assert!(PAGE.contains("id=\"toml-colors\""));
        assert!(PAGE.contains("id=\"import-btn\""));
        assert!(PAGE.contains("id=\"export-btn\""));
        assert!(PAGE.contains("id=\"update-btn\""));
        assert!(PAGE.contains("/api/update"));
        assert!(PAGE.contains("id=\"drop-overlay\""));
        assert!(PAGE.contains("window.addEventListener(\"drop\""));
    }

    #[test]
    fn theme_names_are_single_safe_components() {
        assert!(valid_theme_name("default"));
        assert!(valid_theme_name("my_theme-2"));
        assert!(!valid_theme_name(""));
        assert!(!valid_theme_name(".."));
        assert!(!valid_theme_name("a/b"));
        assert!(!valid_theme_name("a\\b"));
        assert!(!valid_theme_name("theme.toml"));
    }

    #[test]
    fn theme_paths_stay_inside_themes_dir() {
        let dir = Path::new("/tmp/ntc-themes");
        assert_eq!(theme_path(dir, "mine").unwrap(), dir.join("mine.ntc_theme"));
        assert!(theme_path(dir, "../outside").is_none());
        assert!(theme_path(dir, "a/b").is_none());
    }
}
