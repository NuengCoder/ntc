use anyhow::{Context, Result};
use colored::*;
use lettre::{
    message::{header, Mailbox, Message, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    SmtpTransport, Transport,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;

// ── message body: plain text, HTML and Markdown ──────────────────────────────

/// How the `-b` value is interpreted.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum BodyFormat {
    /// Detect from the file extension, else from the content.
    Auto,
    /// Send as-is as the text/plain body.
    Text,
    /// Send as an HTML body (a text/plain alternative is derived from it).
    Html,
    /// Render Markdown to HTML and send it as an HTML body.
    Markdown,
}

impl BodyFormat {
    /// Parse a `--body-format` value.
    fn from_cli(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "auto" => Some(Self::Auto),
            "text" | "plain" | "txt" => Some(Self::Text),
            "html" | "htm" => Some(Self::Html),
            "md" | "markdown" => Some(Self::Markdown),
            _ => None,
        }
    }

    /// Format implied by a body file's extension.
    fn from_extension(path: &Path) -> Option<Self> {
        match path
            .extension()
            .and_then(|e| e.to_str())?
            .to_ascii_lowercase()
            .as_str()
        {
            "html" | "htm" => Some(Self::Html),
            "md" | "markdown" => Some(Self::Markdown),
            "txt" | "text" => Some(Self::Text),
            _ => None,
        }
    }
}

/// The body ready for the message builder: the text/plain part always exists
/// and the text/html part is present for HTML/Markdown bodies, so the client
/// can render (and align) the HTML version.
#[derive(Debug, Clone, PartialEq)]
struct MailBody {
    plain: String,
    html: Option<String>,
}

impl MailBody {
    fn from_text(text: &str) -> Self {
        Self {
            plain: text.to_string(),
            html: None,
        }
    }

    fn from_html(html: &str) -> Self {
        Self {
            plain: html_to_text(html),
            html: Some(wrap_html_document(html)),
        }
    }

    fn from_markdown(markdown: &str) -> Self {
        Self::from_html(&render_markdown(markdown))
    }
}

/// Where a body came from, plus the format implied by its file extension.
#[derive(Debug)]
struct BodySource {
    content: String,
    implied: BodyFormat,
    file: Option<PathBuf>,
}

/// Read the `-b` body: a literal string, `-` for stdin, or a body file
/// (`.html`, `.htm`, `.md`, `.markdown`, `.txt` — read automatically when the
/// path exists). `--body-file` forces the file interpretation.
fn read_body_source(raw: &str, file_arg: Option<&str>) -> Result<BodySource> {
    if let Some(path_str) = file_arg {
        return read_body_file(Path::new(path_str));
    }
    if raw == "-" {
        let content = crate::shell::stdin::read_to_string()
            .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.body_stdin_empty")))?;
        return Ok(BodySource {
            content,
            implied: BodyFormat::Auto,
            file: None,
        });
    }
    if BodyFormat::from_extension(Path::new(raw)).is_some() {
        // A body-file extension means the value is a path, never a literal
        // body; a missing file is an error so a typo can't mail the filename.
        return read_body_file(Path::new(raw));
    }
    Ok(BodySource {
        content: raw.to_string(),
        implied: BodyFormat::Auto,
        file: None,
    })
}

fn read_body_file(path: &Path) -> Result<BodySource> {
    if !path.is_file() {
        anyhow::bail!("{}", t!("cmd.smtp.body_file_missing", path.display()));
    }
    let content = fs::read_to_string(path)
        .with_context(|| t!("cmd.smtp.body_read_fail", path.display()).to_string())?;
    Ok(BodySource {
        content,
        implied: BodyFormat::from_extension(path).unwrap_or(BodyFormat::Auto),
        file: Some(path.to_path_buf()),
    })
}

/// Apply the explicit format, else the file extension, else auto-detection
/// (inline HTML when the content contains real tags, plain text otherwise).
fn resolve_body(source: &BodySource, forced: BodyFormat) -> (MailBody, BodyFormat) {
    let format = if forced != BodyFormat::Auto {
        forced
    } else if source.implied != BodyFormat::Auto {
        source.implied
    } else if looks_like_html(&source.content) {
        BodyFormat::Html
    } else {
        BodyFormat::Text
    };
    let body = match format {
        BodyFormat::Html => MailBody::from_html(&source.content),
        BodyFormat::Markdown => MailBody::from_markdown(&source.content),
        BodyFormat::Auto | BodyFormat::Text => MailBody::from_text(&source.content),
    };
    (body, format)
}

/// True when the string contains at least one real HTML element. Parsing as a
/// fragment keeps text like `a < b and c > d` from being mistaken for markup.
fn looks_like_html(text: &str) -> bool {
    use scraper::node::Node as DomNode;
    // A fragment always gains an `<html>` wrapper, which is not markup the
    // user wrote; any other element means the body really is HTML.
    scraper::Html::parse_fragment(text)
        .tree
        .root()
        .descendants()
        .filter_map(|node| match node.value() {
            DomNode::Element(el) => Some(el.name().to_lowercase()),
            _ => None,
        })
        .any(|name| !matches!(name.as_str(), "html" | "head" | "body"))
}

/// Markdown → HTML with the GitHub extensions enabled. Raw HTML inside the
/// Markdown is preserved so alignment/styling tags survive.
fn render_markdown(markdown: &str) -> String {
    let mut options = comrak::Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.render.r#unsafe = true;
    comrak::markdown_to_html(markdown, &options)
}

/// Email-client CSS: a centered card with readable typography, plus borders
/// for tables and code blocks so rendered Markdown looks finished.
const HTML_MAIL_STYLE: &str = "body{margin:0;padding:0;background:#f4f4f5}\n\
.ntc-mail{max-width:680px;margin:24px auto;padding:28px 32px;background:#fff;border-radius:10px;\n\
font-family:-apple-system,'Segoe UI',Roboto,Helvetica,Arial,sans-serif;font-size:15px;\n\
line-height:1.6;color:#1f2328}\n\
.ntc-mail h1,.ntc-mail h2,.ntc-mail h3{line-height:1.25;margin:1.2em 0 .5em}\n\
.ntc-mail p{margin:0 0 1em}\n\
.ntc-mail a{color:#2563eb}\n\
.ntc-mail table{border-collapse:collapse;margin:1em 0}\n\
.ntc-mail th,.ntc-mail td{border:1px solid #d0d7de;padding:6px 10px}\n\
.ntc-mail th{background:#f6f8fa;text-align:left}\n\
.ntc-mail pre{background:#f6f8fa;padding:12px;border-radius:6px;overflow-x:auto;font-size:13px}\n\
.ntc-mail code{font-family:'SFMono-Regular',Consolas,'Liberation Mono',Menlo,monospace}\n\
.ntc-mail blockquote{margin:1em 0;padding:2px 14px;border-left:3px solid #d0d7de;color:#57606a}\n\
.ntc-mail img{max-width:100%;height:auto}\n\
.ntc-mail hr{border:0;border-top:1px solid #d0d7de;margin:1.5em 0}";

/// Wrap a body fragment in a complete HTML mail document (UTF-8, styled, with
/// the content in a centered card). Documents that already ship their own
/// `<html>`/`<body>` are sent untouched.
fn wrap_html_document(body: &str) -> String {
    let lower = body.to_ascii_lowercase();
    if lower.contains("<html") || lower.contains("<body") {
        return body.to_string();
    }
    format!(
        "<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\" />\n\
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n\
<style>\n{style}\n</style>\n</head>\n<body>\n<div class=\"ntc-mail\">\n{body}\n</div>\n</body>\n</html>\n",
        style = HTML_MAIL_STYLE,
        body = body,
    )
}

/// Tags whose content never belongs in the plain-text alternative.
const HTML_TEXT_SKIP_TAGS: &[&str] = &[
    "head", "script", "style", "meta", "title", "link", "noscript", "template",
];

/// Block-level tags that start a new line in the plain-text alternative.
const HTML_TEXT_BLOCK_TAGS: &[&str] = &[
    "p",
    "div",
    "section",
    "article",
    "header",
    "footer",
    "main",
    "aside",
    "nav",
    "figure",
    "figcaption",
    "address",
    "center",
    "blockquote",
    "pre",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "ul",
    "ol",
    "dl",
    "dt",
    "dd",
    "table",
    "thead",
    "tbody",
    "tfoot",
    "form",
    "fieldset",
];

/// Best-effort text/plain alternative for an HTML body: block tags become
/// line breaks, list items get a `- ` bullet, table cells are tab-separated and
/// all markup (with scripts/styles) is dropped.
fn html_to_text(html: &str) -> String {
    let doc = scraper::Html::parse_document(html);
    let mut out = String::new();
    html_text_walk(doc.tree.root(), &mut out);
    normalize_text(&out)
}

fn html_text_walk(node: ego_tree::NodeRef<'_, scraper::node::Node>, out: &mut String) {
    use scraper::node::Node as DomNode;
    match node.value() {
        DomNode::Text(text) => out.push_str(&text.to_string()),
        DomNode::Element(el) => {
            let name = el.name().to_lowercase();
            if HTML_TEXT_SKIP_TAGS.contains(&name.as_str()) {
                return;
            }
            let block = HTML_TEXT_BLOCK_TAGS.contains(&name.as_str());
            match name.as_str() {
                "br" => out.push('\n'),
                "hr" => {
                    out.push('\n');
                    out.push_str("---");
                    out.push('\n');
                }
                "li" => {
                    if !out.is_empty() && !out.ends_with('\n') {
                        out.push('\n');
                    }
                    out.push_str("- ");
                }
                "tr" => out.push('\n'),
                "td" | "th" => out.push('\t'),
                _ if block => out.push('\n'),
                _ => {}
            }
            for child in node.children() {
                html_text_walk(child, out);
            }
            if block || name == "tr" {
                out.push('\n');
            }
        }
        _ => {
            for child in node.children() {
                html_text_walk(child, out);
            }
        }
    }
}

/// Trim each line, drop whitespace-only lines and collapse blank runs.
fn normalize_text(raw: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if lines.last().map_or(true, |l| l.is_empty()) {
                continue;
            }
            lines.push(String::new());
        } else {
            lines.push(trimmed.to_string());
        }
    }
    while lines.last().map_or(false, |l| l.is_empty()) {
        lines.pop();
    }
    lines.join("\n")
}

#[derive(Serialize, Deserialize, Clone)]
struct SmtpProfile {
    id: String,
    server: String,
    port: u16,
    user: String,
    password: String,
    created: String,
}

fn profiles_db_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("smtp_profiles.json")
}

fn load_profiles() -> Vec<SmtpProfile> {
    let path = profiles_db_path();
    if !path.exists() {
        return Vec::new();
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_profiles(profiles: &[SmtpProfile]) -> Result<()> {
    let path = profiles_db_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let data = serde_json::to_string_pretty(profiles)?;
    fs::write(&path, data)?;
    Ok(())
}

fn generate_id() -> String {
    let id: u32 = rand::random();
    format!("{:08x}", id)
}

fn encode_password(pwd: &str) -> String {
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, pwd.as_bytes())
}

fn decode_password(encoded: &str) -> Result<String> {
    let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encoded)?;
    String::from_utf8(bytes)
        .map_err(|e| anyhow::anyhow!("{}", t!("cmd.smtp.invalid_pwd_encoding", e)))
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;
    for ch in input.chars() {
        match ch {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            ' ' | '\t' if !in_single && !in_double => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

pub fn cmd_smtp(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() || tokens[0] == "help" || tokens[0] == "--help" || tokens[0] == "-h" {
        print_smtp_help();
        return Ok(false);
    }

    let mut i = 0;
    let mut server = String::new();
    let mut port: u16 = 587;
    let mut user = String::new();
    let mut password = String::new();
    let mut profile_id = String::new();
    let mut list_profiles = false;
    let mut info_id = String::new();
    let mut reveal_password = false;
    let mut remove_id = String::new();
    let mut to_addrs = Vec::new();
    let mut cc_addrs = Vec::new();
    let mut bcc_addrs = Vec::new();
    let mut subject = String::new();
    let mut body = String::new();
    let mut body_file: Option<String> = None;
    let mut body_format = BodyFormat::Auto;
    let mut attach_paths = Vec::new();
    let mut is_creating = false;

    while i < tokens.len() {
        match tokens[i].as_str() {
            "-i" | "--server" => {
                i += 1;
                server = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_server")))?
                    .clone();
                is_creating = true;
            }
            "-p" | "--port" => {
                i += 1;
                port = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_port")))?
                    .parse()
                    .map_err(|_| anyhow::anyhow!("{}", t!("cmd.smtp.invalid_port")))?;
            }
            "-u" | "--user" => {
                i += 1;
                user = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_user")))?
                    .clone();
            }
            "-pwd" | "--password" => {
                i += 1;
                password = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_password")))?
                    .clone();
            }
            "--id-ls" => list_profiles = true,
            "--id-info" => {
                i += 1;
                info_id = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_id_info")))?
                    .clone();
            }
            // With --id-info: show the app password after unlocking deadlock.
            "--deadlock" => reveal_password = true,
            "--id-rm" => {
                i += 1;
                remove_id = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_id_rm")))?
                    .clone();
            }
            "--id" => {
                i += 1;
                profile_id = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_id")))?
                    .clone();
            }
            "--to" | "-t" => {
                i += 1;
                let val = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_recipient_to")))?
                    .clone();
                to_addrs = val
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            "--cc" | "-c" => {
                i += 1;
                let val = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_recipient_cc")))?
                    .clone();
                cc_addrs = val
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            "--bcc" => {
                i += 1;
                let val = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_recipient_bcc")))?
                    .clone();
                bcc_addrs = val
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            "-s" | "--subject" => {
                i += 1;
                subject = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_subject")))?
                    .clone();
            }
            "-b" | "--body" => {
                i += 1;
                body = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_body")))?
                    .clone();
            }
            "--body-file" => {
                i += 1;
                body_file = Some(
                    tokens
                        .get(i)
                        .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_body_file")))?
                        .clone(),
                );
            }
            "-bf" | "--body-format" => {
                i += 1;
                let value = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_body_format")))?;
                body_format = BodyFormat::from_cli(value).ok_or_else(|| {
                    anyhow::anyhow!("{}", t!("cmd.smtp.invalid_body_format", value))
                })?;
            }
            "-f" | "--attach" => {
                i += 1;
                let val = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_path")))?
                    .clone();
                let paths: Vec<String> = val
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                attach_paths.extend(paths);
            }
            "--" => break,
            _ => {
                anyhow::bail!("{}", t!("cmd.smtp.unknown_option", tokens[i]));
            }
        }
        i += 1;
    }

    // Dispatch subcommands
    if list_profiles {
        return cmd_smtp_ls();
    }
    if !info_id.is_empty() {
        return cmd_smtp_info(&info_id, reveal_password);
    }
    if !remove_id.is_empty() {
        return cmd_smtp_rm(&remove_id);
    }
    if is_creating {
        return cmd_smtp_create(&server, port, &user, &password);
    }
    if !profile_id.is_empty() {
        return cmd_smtp_send(
            &profile_id,
            &to_addrs,
            &cc_addrs,
            &bcc_addrs,
            &subject,
            &body,
            body_file.as_deref(),
            body_format,
            &attach_paths,
        );
    }

    print_smtp_help();
    Ok(false)
}

fn cmd_smtp_create(server: &str, port: u16, user: &str, password: &str) -> Result<bool> {
    if server.is_empty() || user.is_empty() || password.is_empty() {
        anyhow::bail!("{}", t!("cmd.smtp.create_usage"));
    }

    let mut profiles = load_profiles();
    let id = generate_id();
    let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();

    profiles.push(SmtpProfile {
        id: id.clone(),
        server: server.to_string(),
        port,
        user: user.to_string(),
        password: encode_password(password),
        created: now,
    });

    save_profiles(&profiles)?;
    print_success(&t!("cmd.smtp.profile_saved", id.bold()));
    Ok(false)
}

fn cmd_smtp_ls() -> Result<bool> {
    let profiles = load_profiles();
    if profiles.is_empty() {
        println!("  {}  {}", "─".dimmed(), t!("cmd.smtp.no_profiles"));
        println!(
            "  {} {}",
            t!("cmd.smtp.create_one_hint").yellow(),
            t!("cmd.smtp.create_syntax")
        );
        return Ok(false);
    }

    println!("{}", t!("cmd.smtp.profiles_title").bold());
    println!("{}", "─────────────".dimmed());
    for p in &profiles {
        let masked = format!("{}***", &p.user[..p.user.len().min(3)]);
        println!(
            "  {}  {} @ {}:{}  ({})",
            p.id.bold().green(),
            masked.dimmed(),
            p.server.cyan(),
            p.port,
            p.created.dimmed()
        );
    }
    Ok(false)
}

fn cmd_smtp_info(id: &str, reveal_password: bool) -> Result<bool> {
    let profiles = load_profiles();
    let profile = profiles
        .iter()
        .find(|p| p.id == id || p.id.starts_with(id))
        .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.profile_not_found", id)))?;

    // The app password is never printed by default: `--deadlock` unlocks the
    // deadlock vault to prove the user knows the master password first.
    let pwd_display = if reveal_password {
        crate::deadlock::unlock_required()?;
        decode_password(&profile.password).unwrap_or_else(|_| "***".to_string())
    } else {
        t!("cmd.smtp.password_hidden")
    };

    println!("{}", t!("cmd.smtp.profile_label").bold());
    println!(
        "  {}  {}",
        t!("cmd.smtp.id_label").dimmed(),
        profile.id.green()
    );
    println!(
        "  {}  {}",
        t!("cmd.smtp.server_label").dimmed(),
        profile.server.cyan()
    );
    println!(
        "  {}  {}",
        t!("cmd.smtp.port_label").dimmed(),
        profile.port.to_string().yellow()
    );
    println!(
        "  {}  {}",
        t!("cmd.smtp.user_label").dimmed(),
        profile.user.bold()
    );
    println!(
        "  {}  {}",
        t!("cmd.smtp.password_label").dimmed(),
        pwd_display.dimmed()
    );
    println!(
        "  {}  {}",
        t!("cmd.smtp.created_label").dimmed(),
        profile.created.dimmed()
    );
    Ok(false)
}

fn cmd_smtp_rm(id: &str) -> Result<bool> {
    let mut profiles = load_profiles();
    let len_before = profiles.len();
    profiles.retain(|p| p.id != id && !p.id.starts_with(id));
    if profiles.len() == len_before {
        anyhow::bail!("{}", t!("cmd.smtp.profile_not_found", id));
    }
    save_profiles(&profiles)?;
    print_info(&t!("cmd.smtp.removed_profile", id));
    Ok(false)
}

fn cmd_smtp_send(
    profile_id: &str,
    to_addrs: &[String],
    cc_addrs: &[String],
    bcc_addrs: &[String],
    subject: &str,
    body: &str,
    body_file: Option<&str>,
    body_format: BodyFormat,
    attach_paths: &[String],
) -> Result<bool> {
    let profiles = load_profiles();
    let profile = profiles
        .iter()
        .find(|p| p.id == profile_id || p.id.starts_with(profile_id))
        .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.profile_not_found", profile_id)))?;

    if to_addrs.is_empty() {
        anyhow::bail!("{}", t!("cmd.smtp.need_recipient"));
    }
    if subject.is_empty() {
        anyhow::bail!("{}", t!("cmd.smtp.need_subject"));
    }

    let password_plain =
        decode_password(&profile.password).context(t!("cmd.smtp.decode_failed").to_string())?;

    // Resolve the body first: an unreadable body file must fail before the
    // SMTP connection, not after it.
    let body_source = read_body_source(body, body_file)?;
    let (mail_body, body_kind) = resolve_body(&body_source, body_format);
    if let Some(path) = &body_source.file {
        let note = match body_kind {
            BodyFormat::Markdown => t!("cmd.smtp.body_md_from_file", path.display()),
            BodyFormat::Html => t!("cmd.smtp.body_html_from_file", path.display()),
            BodyFormat::Auto | BodyFormat::Text => {
                t!("cmd.smtp.body_text_from_file", path.display())
            }
        };
        print_info(&note);
    }

    // Collect attachment files
    let mut attachment_files: Vec<PathBuf> = Vec::new();
    for path_str in attach_paths {
        let p = PathBuf::from(path_str);
        if p.is_dir() {
            let entries: Vec<_> = walkdir::WalkDir::new(&p)
                .max_depth(5)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file() && !is_blocked_executable(e.path()))
                .map(|e| e.path().to_path_buf())
                .collect();
            if entries.is_empty() {
                print_warning(&t!("cmd.smtp.no_files_in_dir", path_str));
            }
            attachment_files.extend(entries);
        } else if p.is_file() {
            if is_blocked_executable(&p) {
                print_warning(&t!("cmd.smtp.skipping_exec", path_str));
            } else {
                attachment_files.push(p);
            }
        } else if p.exists() {
            print_warning(&t!("cmd.smtp.skipping_not_file", path_str));
        } else {
            print_warning(&t!("cmd.smtp.path_not_found", path_str));
        }
    }

    // Remove duplicate files while preserving order
    let mut seen = std::collections::HashSet::new();
    attachment_files.retain(|p| seen.insert(p.canonicalize().unwrap_or_else(|_| p.to_path_buf())));

    let msg = build_message(
        &profile.user,
        to_addrs,
        cc_addrs,
        bcc_addrs,
        subject,
        &mail_body,
        &attachment_files,
    )?;

    // Connect and send
    let creds = Credentials::new(profile.user.clone(), password_plain);

    let transport_result = if profile.port == 465 {
        SmtpTransport::relay(&profile.server)
            .map(|b| b.credentials(creds).port(profile.port).build())
    } else if profile.port == 587 {
        SmtpTransport::starttls_relay(&profile.server)
            .map(|b| b.credentials(creds).port(profile.port).build())
    } else {
        SmtpTransport::starttls_relay(&profile.server)
            .or_else(|_| SmtpTransport::relay(&profile.server))
            .map(|b| b.credentials(creds).port(profile.port).build())
    };

    let mailer = match transport_result {
        Ok(m) => m,
        Err(e) => {
            print_error(&t!("cmd.smtp.transport_fail", e));
            return Ok(false);
        }
    };

    print_info(&t!("cmd.smtp.sending", to_addrs.join(", ")));

    match mailer.send(&msg) {
        Ok(_) => {
            let file_count = attachment_files.len();
            let file_info = if file_count > 0 {
                t!("cmd.smtp.with_attachments", file_count)
            } else {
                String::new()
            };
            print_success(&t!("cmd.smtp.send_ok", file_info));
        }
        Err(e) => {
            print_error(&t!("cmd.smtp.send_fail", e));
        }
    }

    Ok(false)
}

/// Assemble the MIME message: `text/plain` for plain bodies, and
/// `multipart/alternative` (plain + HTML) whenever an HTML version exists, so
/// HTML clients render — and align — the HTML body while plain clients still
/// get readable text. Attachments wrap the body in `multipart/mixed`.
fn build_message(
    from_addr: &str,
    to_addrs: &[String],
    cc_addrs: &[String],
    bcc_addrs: &[String],
    subject: &str,
    mail_body: &MailBody,
    attachment_files: &[PathBuf],
) -> Result<Message> {
    let from: Mailbox = format!("<{}>", from_addr)
        .parse()
        .context(t!("cmd.smtp.invalid_sender").to_string())?;

    let mut builder = Message::builder().from(from).subject(subject);

    for addr in to_addrs {
        let mb: Mailbox = addr
            .parse()
            .context(t!("cmd.smtp.invalid_recipient", addr).to_string())?;
        builder = builder.to(mb);
    }
    for addr in cc_addrs {
        let mb: Mailbox = addr
            .parse()
            .context(t!("cmd.smtp.invalid_cc", addr).to_string())?;
        builder = builder.cc(mb);
    }
    for addr in bcc_addrs {
        let mb: Mailbox = addr
            .parse()
            .context(t!("cmd.smtp.invalid_bcc", addr).to_string())?;
        builder = builder.bcc(mb);
    }

    let plain_part = SinglePart::builder()
        .header(header::ContentType::TEXT_PLAIN)
        .body(mail_body.plain.clone());
    let alternative = mail_body.html.as_ref().map(|html| {
        MultiPart::alternative()
            .singlepart(plain_part.clone())
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(html.clone()),
            )
    });

    if attachment_files.is_empty() {
        return match alternative {
            Some(alt) => builder
                .multipart(alt)
                .context(t!("cmd.smtp.build_fail").to_string()),
            None => builder
                .singlepart(plain_part)
                .context(t!("cmd.smtp.build_fail").to_string()),
        };
    }

    // mixed: the (possibly alternative) body first, then the attachments
    let mut multipart = MultiPart::mixed().build();
    multipart = match alternative {
        Some(alt) => multipart.multipart(alt),
        None => multipart.singlepart(plain_part),
    };

    for file_path in attachment_files {
        let data = fs::read(file_path)
            .with_context(|| t!("cmd.smtp.read_attach_fail", file_path.display()).to_string())?;
        let filename = file_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "attachment".to_string());

        let content_type = guess_mime(file_path);
        let ct = header::ContentType::parse(&content_type)
            .unwrap_or_else(|_| header::ContentType::parse("application/octet-stream").unwrap());

        multipart = multipart.singlepart(
            SinglePart::builder()
                .header(ct)
                .header(header::ContentDisposition::attachment(&filename))
                .body(data),
        );
    }

    builder
        .multipart(multipart)
        .context(t!("cmd.smtp.build_fail").to_string())
}

fn is_blocked_executable(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()).unwrap_or(""),
        "exe"
            | "dll"
            | "msi"
            | "msp"
            | "mst"
            | "bat"
            | "cmd"
            | "ps1"
            | "psm1"
            | "psd1"
            | "scr"
            | "com"
            | "pif"
            | "apk"
            | "xapk"
            | "ipa"
            | "app"
            | "dmg"
            | "pkg"
            | "sh"
            | "bash"
            | "zsh"
            | "ksh"
            | "bin"
            | "deb"
            | "rpm"
            | "jar"
            | "vb"
            | "vbs"
            | "vbe"
            | "wsf"
            | "wsh"
    )
}

fn guess_mime(path: &Path) -> String {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "txt" | "md" | "rst" => "text/plain",
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "pdf" => "application/pdf",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "zip" => "application/zip",
        "gz" | "gzip" => "application/gzip",
        "tar" => "application/x-tar",
        "mp3" => "audio/mpeg",
        "mp4" => "video/mp4",
        "doc" | "docx" => "application/msword",
        "xls" | "xlsx" => "application/vnd.ms-excel",
        "csv" => "text/csv",
        "yaml" | "yml" => "application/x-yaml",
        "toml" => "application/toml",
        "rtf" => "application/rtf",
        _ => "application/octet-stream",
    }
    .to_string()
}

fn print_smtp_help() {
    println!("{}", t!("cmd.smtp.help_title").bold());
    println!();
    println!("{}", t!("cmd.smtp.help_create_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_create_syntax"));
    println!("  {}", t!("cmd.smtp.help_create_example"));
    println!();
    println!("{}", t!("cmd.smtp.help_list_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_list_syntax"));
    println!();
    println!("{}", t!("cmd.smtp.help_info_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_info_syntax"));
    println!("  {}", t!("cmd.smtp.help_info_deadlock"));
    println!();
    println!("{}", t!("cmd.smtp.help_rm_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_rm_syntax"));
    println!();
    println!("{}", t!("cmd.smtp.help_send_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_send_syntax"));
    println!();
    println!("{}", t!("cmd.smtp.help_body_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_body_inline"));
    println!("  {}", t!("cmd.smtp.help_body_file"));
    println!("  {}", t!("cmd.smtp.help_body_format"));
    println!("  {}", t!("cmd.smtp.help_body_stdin"));
    println!();
    println!("{}", t!("cmd.smtp.help_attachments_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_attachments_desc"));
    println!("  {}", t!("cmd.smtp.help_attachments_example"));
    println!();
    println!("{}", t!("cmd.smtp.help_ports_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_port_587"));
    println!("  {}", t!("cmd.smtp.help_port_465"));
    println!("  {}", t!("cmd.smtp.help_port_25"));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source(content: &str) -> BodySource {
        BodySource {
            content: content.to_string(),
            implied: BodyFormat::Auto,
            file: None,
        }
    }

    #[test]
    fn inline_html_is_detected() {
        assert!(looks_like_html("<p align=\"center\">Hi</p>"));
        assert!(looks_like_html("<h1>Title</h1>"));
        assert!(looks_like_html("<br>"));
        assert!(!looks_like_html("Hello world"));
        assert!(!looks_like_html("a < b and c > d"));
        assert!(!looks_like_html("mail me at a@b.com"));
    }

    #[test]
    fn inline_html_body_keeps_markup_and_derives_plain_text() {
        let (body, kind) = resolve_body(&source("<h1>Hello</h1><p>World</p>"), BodyFormat::Auto);
        assert_eq!(kind, BodyFormat::Html);
        let html = body.html.expect("html part");
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("class=\"ntc-mail\""));
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert_eq!(body.plain, "Hello\n\nWorld");
    }

    #[test]
    fn plain_text_stays_plain() {
        let (body, kind) = resolve_body(&source("hello there"), BodyFormat::Auto);
        assert_eq!(kind, BodyFormat::Text);
        assert_eq!(body.plain, "hello there");
        assert!(body.html.is_none());
    }

    #[test]
    fn markdown_renders_gfm_and_keeps_raw_html() {
        let md = "# Title\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n<p style=text-align:center>mid</p>";
        let (body, kind) = resolve_body(&source(md), BodyFormat::Markdown);
        assert_eq!(kind, BodyFormat::Markdown);
        let html = body.html.expect("html part");
        assert!(html.contains("<h1>Title</h1>"));
        assert!(html.contains("<table>"));
        assert!(html.contains("text-align:center"));
        assert!(body.plain.contains("Title"));
        assert!(!body.plain.contains("<table>"));
    }

    #[test]
    fn body_files_are_read_and_typed_by_extension() {
        let dir = tempfile::tempdir().unwrap();
        let html_path = dir.path().join("letter.html");
        fs::write(&html_path, "<p align=center>Hi</p>").unwrap();
        let md_path = dir.path().join("letter.md");
        fs::write(&md_path, "# Hi\n\n**bold**\n").unwrap();
        let txt_path = dir.path().join("letter.txt");
        fs::write(&txt_path, "plain body").unwrap();

        let src = read_body_source(html_path.to_str().unwrap(), None).unwrap();
        let (body, kind) = resolve_body(&src, BodyFormat::Auto);
        assert_eq!(kind, BodyFormat::Html);
        assert!(body.html.unwrap().contains("<p align=center>Hi</p>"));

        let src = read_body_source(md_path.to_str().unwrap(), None).unwrap();
        let (body, kind) = resolve_body(&src, BodyFormat::Auto);
        assert_eq!(kind, BodyFormat::Markdown);
        assert!(body.html.unwrap().contains("<strong>bold</strong>"));

        let src = read_body_source(txt_path.to_str().unwrap(), None).unwrap();
        let (body, kind) = resolve_body(&src, BodyFormat::Auto);
        assert_eq!(kind, BodyFormat::Text);
        assert_eq!(body.plain, "plain body");
        assert!(body.html.is_none());
    }

    #[test]
    fn body_file_flag_forces_file_interpretation() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("body");
        fs::write(&path, "# md by flag\n").unwrap();
        let src = read_body_source("ignored literal", Some(path.to_str().unwrap())).unwrap();
        let (body, kind) = resolve_body(&src, BodyFormat::Markdown);
        assert_eq!(kind, BodyFormat::Markdown);
        assert!(body.html.unwrap().contains("md by flag"));
    }

    #[test]
    fn missing_body_file_is_an_error() {
        let err = read_body_source("does-not-exist.md", None)
            .unwrap_err()
            .to_string();
        assert!(err.contains("does-not-exist.md"), "unexpected error: {err}");
    }

    #[test]
    fn forced_text_keeps_the_source_verbatim() {
        let (body, kind) = resolve_body(&source("<p>Hi</p>"), BodyFormat::Text);
        assert_eq!(kind, BodyFormat::Text);
        assert_eq!(body.plain, "<p>Hi</p>");
        assert!(body.html.is_none());
    }

    #[test]
    fn full_html_documents_are_not_wrapped_twice() {
        let doc = "<!DOCTYPE html><html><body><p>x</p></body></html>";
        assert_eq!(wrap_html_document(doc), doc);

        let wrapped = wrap_html_document("<p>x</p>");
        assert!(wrapped.starts_with("<!DOCTYPE html>"));
        assert!(wrapped.contains("<div class=\"ntc-mail\">"));
        assert!(wrapped.contains("<p>x</p>"));
    }

    fn formatted_message(body: &MailBody, attachments: &[PathBuf]) -> String {
        let msg = build_message(
            "sender@example.com",
            &["to@example.com".to_string()],
            &[],
            &[],
            "Subject",
            body,
            attachments,
        )
        .expect("message builds");
        String::from_utf8_lossy(&msg.formatted()).to_string()
    }

    #[test]
    fn html_body_sends_multipart_alternative() {
        let (body, _) = resolve_body(
            &source("<p style=text-align:center>Hi</p>"),
            BodyFormat::Auto,
        );
        let formatted = formatted_message(&body, &[]);
        assert!(formatted.contains("Content-Type: multipart/alternative"));
        assert!(formatted.contains("text/plain; charset=utf-8"));
        assert!(formatted.contains("text/html; charset=utf-8"));
        assert!(formatted.contains("text-align:center"));
        assert!(formatted.contains("<!DOCTYPE html>"));
        let plain = formatted.find("text/plain").expect("plain part");
        let html = formatted.find("text/html").expect("html part");
        assert!(plain < html, "plain part must come first:\n{formatted}");
    }

    #[test]
    fn plain_body_stays_single_part() {
        let formatted = formatted_message(&MailBody::from_text("hello"), &[]);
        assert!(formatted.contains("Content-Type: text/plain; charset=utf-8"));
        assert!(!formatted.contains("multipart/alternative"));
    }

    #[test]
    fn attachments_wrap_the_html_body_in_mixed() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("report.txt");
        fs::write(&path, "data").unwrap();
        let formatted = formatted_message(&MailBody::from_html("<p>x</p>"), &[path]);
        assert!(formatted.contains("Content-Type: multipart/mixed"));
        assert!(formatted.contains("multipart/alternative"));
        assert!(formatted.contains("Content-Disposition: attachment"));
    }

    #[test]
    fn html_to_text_separates_blocks_lists_and_tables() {
        assert_eq!(
            html_to_text("<h1>Title</h1><p>One</p><ul><li>a</li><li>b</li></ul>"),
            "Title\n\nOne\n\n- a\n- b"
        );
        assert_eq!(
            html_to_text("<table><tr><td>a</td><td>b</td></tr></table>"),
            "a\tb"
        );
        assert_eq!(
            html_to_text("<p>Hi<style>p{}</style><br>there</p>"),
            "Hi\nthere"
        );
    }
}
