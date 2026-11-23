//! `smtp` message body handling: plain text, HTML, Markdown, and alternatives.

use crate::t;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// How the `-b` value is interpreted.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum BodyFormat {
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
    pub(super) fn from_cli(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "auto" => Some(Self::Auto),
            "text" | "plain" | "txt" => Some(Self::Text),
            "html" | "htm" => Some(Self::Html),
            "md" | "markdown" => Some(Self::Markdown),
            _ => None,
        }
    }

    /// Format implied by a body file's extension.
    pub(super) fn from_extension(path: &Path) -> Option<Self> {
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
pub(super) struct MailBody {
    pub(super) plain: String,
    pub(super) html: Option<String>,
}

impl MailBody {
    pub(super) fn from_text(text: &str) -> Self {
        Self {
            plain: text.to_string(),
            html: None,
        }
    }

    pub(super) fn from_html(html: &str) -> Self {
        Self {
            plain: html_to_text(html),
            html: Some(wrap_html_document(html)),
        }
    }

    pub(super) fn from_markdown(markdown: &str) -> Self {
        Self::from_html(&render_markdown(markdown))
    }
}

/// Where a body came from, plus the format implied by its file extension.
#[derive(Debug)]
pub(super) struct BodySource {
    pub(super) content: String,
    pub(super) implied: BodyFormat,
    pub(super) file: Option<PathBuf>,
}

/// Read the `-b` body: a literal string, `-` for stdin, or a body file
/// (`.html`, `.htm`, `.md`, `.markdown`, `.txt` — read automatically when the
/// path exists). `--body-file` forces the file interpretation.
pub(super) fn read_body_source(raw: &str, file_arg: Option<&str>) -> Result<BodySource> {
    if let Some(path_str) = file_arg {
        let expanded = crate::utils::paths::expand_path_str(path_str);
        return read_body_file(Path::new(&expanded));
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
    let expanded_raw = crate::utils::paths::expand_path_str(raw);
    if BodyFormat::from_extension(Path::new(&expanded_raw)).is_some() {
        // A body-file extension means the value is a path, never a literal
        // body; a missing file is an error so a typo can't mail the filename.
        return read_body_file(Path::new(&expanded_raw));
    }
    Ok(BodySource {
        content: raw.to_string(),
        implied: BodyFormat::Auto,
        file: None,
    })
}

pub(super) fn read_body_file(path: &Path) -> Result<BodySource> {
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
pub(super) fn resolve_body(source: &BodySource, forced: BodyFormat) -> (MailBody, BodyFormat) {
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
pub(super) fn looks_like_html(text: &str) -> bool {
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
pub(super) fn render_markdown(markdown: &str) -> String {
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
pub(super) fn wrap_html_document(body: &str) -> String {
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
pub(super) fn html_to_text(html: &str) -> String {
    let doc = scraper::Html::parse_document(html);
    let mut out = String::new();
    html_text_walk(doc.tree.root(), &mut out);
    normalize_text(&out)
}

pub(super) fn html_text_walk(node: ego_tree::NodeRef<'_, scraper::node::Node>, out: &mut String) {
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
pub(super) fn normalize_text(raw: &str) -> String {
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
