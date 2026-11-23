use super::body::*;
use super::config::load_smtp_file;
use super::send::build_message;
use super::tokenize;
use std::fs;
use std::path::PathBuf;

fn source(content: &str) -> BodySource {
    BodySource {
        content: content.to_string(),
        implied: BodyFormat::Auto,
        file: None,
    }
}

#[test]
fn only_one_outer_quote_pair_is_stripped() {
    assert_eq!(tokenize("-s \"SUBJECT\""), vec!["-s", "SUBJECT"]);
    assert_eq!(tokenize("-s 'SUBJECT'"), vec!["-s", "SUBJECT"]);
    assert_eq!(tokenize("-s '\"SUBJECT\"'"), vec!["-s", "\"SUBJECT\""]);
    assert_eq!(tokenize("-s \"'SUBJECT'\""), vec!["-s", "'SUBJECT'"]);
    assert_eq!(
        tokenize("-b \"hello world\" -t a@b.c"),
        vec!["-b", "hello world", "-t", "a@b.c"]
    );
}

#[test]
fn config_file_accepts_string_and_array_recipients() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("ntc_smtp.json");
    fs::write(
        &path,
        r#"{
                "server": "smtp.example.com",
                "port": 465,
                "user": "me@example.com",
                "password": "secret",
                "to": ["a@example.com", "b@example.com"],
                "files": ["report.pdf"]
            }"#,
    )
    .unwrap();
    let cfg = load_smtp_file(&path).unwrap();
    assert_eq!(cfg.server, "smtp.example.com");
    assert_eq!(cfg.port, 465);
    assert_eq!(cfg.to, vec!["a@example.com", "b@example.com"]);
    assert_eq!(cfg.files, vec!["report.pdf"]);

    fs::write(
        &path,
        r#"{ "server": "s", "user": "u", "password": "p", "to": "a@x, b@y" }"#,
    )
    .unwrap();
    let cfg = load_smtp_file(&path).unwrap();
    assert_eq!(cfg.to, vec!["a@x", "b@y"]);
    assert_eq!(cfg.port, 587);
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
