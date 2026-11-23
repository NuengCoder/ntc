//! `smtp` message construction and delivery.

use super::body::{read_body_source, resolve_body, BodyFormat, MailBody};
use super::config::load_smtp_file;
use super::profiles::{decode_password, load_profiles};
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use anyhow::{Context, Result};
use lettre::{
    message::{header, Mailbox, Message, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    SmtpTransport, Transport,
};
use std::fs;
use std::path::{Path, PathBuf};

pub(super) fn cmd_smtp_send(
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
    let password_plain =
        decode_password(&profile.password).context(t!("cmd.smtp.decode_failed").to_string())?;
    send_mail(
        &profile.server,
        profile.port,
        &profile.user,
        password_plain,
        to_addrs,
        cc_addrs,
        bcc_addrs,
        subject,
        body,
        body_file,
        body_format,
        attach_paths,
    )
}

/// Send using a `ntc_smtp.json` file (`-r`). Command-line recipients and
/// attachments add to whatever the file already lists.
#[allow(clippy::too_many_arguments)]
pub(super) fn cmd_smtp_send_file(
    file: &str,
    to_addrs: &[String],
    cc_addrs: &[String],
    bcc_addrs: &[String],
    subject: &str,
    body: &str,
    body_file: Option<&str>,
    body_format: BodyFormat,
    attach_paths: &[String],
) -> Result<bool> {
    let expanded = crate::utils::paths::expand_path_str(file);
    let cfg = load_smtp_file(Path::new(&expanded))?;
    let mut to = to_addrs.to_vec();
    if to.is_empty() {
        to = cfg.to.clone();
    }
    let mut attachments = cfg.files.clone();
    attachments.extend(attach_paths.iter().cloned());
    send_mail(
        &cfg.server,
        cfg.port,
        &cfg.user,
        cfg.password.clone(),
        &to,
        cc_addrs,
        bcc_addrs,
        subject,
        body,
        body_file,
        body_format,
        &attachments,
    )
}

#[allow(clippy::too_many_arguments)]
pub(super) fn send_mail(
    server: &str,
    port: u16,
    user: &str,
    password_plain: String,
    to_addrs: &[String],
    cc_addrs: &[String],
    bcc_addrs: &[String],
    subject: &str,
    body: &str,
    body_file: Option<&str>,
    body_format: BodyFormat,
    attach_paths: &[String],
) -> Result<bool> {
    if to_addrs.is_empty() {
        anyhow::bail!("{}", t!("cmd.smtp.need_recipient"));
    }
    if subject.is_empty() {
        anyhow::bail!("{}", t!("cmd.smtp.need_subject"));
    }

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
        user,
        to_addrs,
        cc_addrs,
        bcc_addrs,
        subject,
        &mail_body,
        &attachment_files,
    )?;

    // Connect and send
    let creds = Credentials::new(user.to_string(), password_plain);

    let transport_result = if port == 465 {
        SmtpTransport::relay(server).map(|b| b.credentials(creds).port(port).build())
    } else if port == 587 {
        SmtpTransport::starttls_relay(server).map(|b| b.credentials(creds).port(port).build())
    } else {
        SmtpTransport::starttls_relay(server)
            .or_else(|_| SmtpTransport::relay(server))
            .map(|b| b.credentials(creds).port(port).build())
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
pub(super) fn build_message(
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

pub(super) fn is_blocked_executable(path: &Path) -> bool {
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

pub(super) fn guess_mime(path: &Path) -> String {
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
