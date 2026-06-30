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
    base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        pwd.as_bytes(),
    )
}

fn decode_password(encoded: &str) -> Result<String> {
    let bytes = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        encoded,
    )?;
    String::from_utf8(bytes).map_err(|e| anyhow::anyhow!("Invalid password encoding: {}", e))
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
    let mut remove_id = String::new();
    let mut to_addrs = Vec::new();
    let mut cc_addrs = Vec::new();
    let mut bcc_addrs = Vec::new();
    let mut subject = String::new();
    let mut body = String::new();
    let mut attach_paths = Vec::new();
    let mut is_creating = false;

    while i < tokens.len() {
        match tokens[i].as_str() {
            "-i" | "--server" => {
                i += 1;
                server = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing server after -i"))?.clone();
                is_creating = true;
            }
            "-p" | "--port" => {
                i += 1;
                port = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing port after -p"))?.parse()
                    .map_err(|_| anyhow::anyhow!("Invalid port number"))?;
            }
            "-u" | "--user" => {
                i += 1;
                user = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing username after -u"))?.clone();
            }
            "-pwd" | "--password" => {
                i += 1;
                password = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing password after -pwd"))?.clone();
            }
            "--id-ls" => list_profiles = true,
            "--id-info" => {
                i += 1;
                info_id = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing ID after --id-info"))?.clone();
            }
            "--id-rm" => {
                i += 1;
                remove_id = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing ID after --id-rm"))?.clone();
            }
            "--id" => {
                i += 1;
                profile_id = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing ID after --id"))?.clone();
            }
            "--to" | "-t" => {
                i += 1;
                let val = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing recipient after --to"))?.clone();
                to_addrs = val.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            }
            "--cc" | "-c" => {
                i += 1;
                let val = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing recipient after --cc"))?.clone();
                cc_addrs = val.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            }
            "--bcc" => {
                i += 1;
                let val = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing recipient after --bcc"))?.clone();
                bcc_addrs = val.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            }
            "-s" | "--subject" => {
                i += 1;
                subject = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing subject after -s"))?.clone();
            }
            "-b" | "--body" => {
                i += 1;
                body = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing body after -b"))?.clone();
            }
            "-f" | "--attach" => {
                i += 1;
                let val = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing path after -f"))?.clone();
                let paths: Vec<String> = val.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                attach_paths.extend(paths);
            }
            _ => {
                anyhow::bail!("Unknown option: {}", tokens[i]);
            }
        }
        i += 1;
    }

    // Dispatch subcommands
    if list_profiles {
        return cmd_smtp_ls();
    }
    if !info_id.is_empty() {
        return cmd_smtp_info(&info_id);
    }
    if !remove_id.is_empty() {
        return cmd_smtp_rm(&remove_id);
    }
    if is_creating {
        return cmd_smtp_create(&server, port, &user, &password);
    }
    if !profile_id.is_empty() {
        return cmd_smtp_send(&profile_id, &to_addrs, &cc_addrs, &bcc_addrs, &subject, &body, &attach_paths);
    }

    print_smtp_help();
    Ok(false)
}

fn cmd_smtp_create(server: &str, port: u16, user: &str, password: &str) -> Result<bool> {
    if server.is_empty() || user.is_empty() || password.is_empty() {
        anyhow::bail!("Server, username, and password are required. Usage: smtp -i <server> -p <port> -u <user> -pwd <password>");
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
    print_success(&format!("SMTP profile saved: {}", id.bold()));
    Ok(false)
}

fn cmd_smtp_ls() -> Result<bool> {
    let profiles = load_profiles();
    if profiles.is_empty() {
        println!("  {}  No SMTP profiles found.", "─".dimmed());
        println!("  Create one: {} -i <server> -p <port> -u <user> -pwd <password>", "smtp".yellow());
        return Ok(false);
    }

    println!("{}", "SMTP Profiles:".bold());
    println!("{}", "─────────────".dimmed());
    for p in &profiles {
        let masked = format!("{}***", &p.user[..p.user.len().min(3)]);
        println!("  {}  {} @ {}:{}  ({})", p.id.bold().green(), masked.dimmed(), p.server.cyan(), p.port, p.created.dimmed());
    }
    Ok(false)
}

fn cmd_smtp_info(id: &str) -> Result<bool> {
    let profiles = load_profiles();
    let profile = profiles.iter().find(|p| p.id == id || p.id.starts_with(id))
        .ok_or_else(|| anyhow::anyhow!("Profile not found: {}", id))?;

    let pwd_plain = decode_password(&profile.password).unwrap_or_else(|_| "***".to_string());

    println!("{}", "SMTP Profile:".bold());
    println!("  {}  {}", "ID:".dimmed(), profile.id.green());
    println!("  {}  {}", "Server:".dimmed(), profile.server.cyan());
    println!("  {}  {}", "Port:".dimmed(), profile.port.to_string().yellow());
    println!("  {}  {}", "User:".dimmed(), profile.user.bold());
    println!("  {}  {}", "Password:".dimmed(), pwd_plain.dimmed());
    println!("  {}  {}", "Created:".dimmed(), profile.created.dimmed());
    Ok(false)
}

fn cmd_smtp_rm(id: &str) -> Result<bool> {
    let mut profiles = load_profiles();
    let len_before = profiles.len();
    profiles.retain(|p| p.id != id && !p.id.starts_with(id));
    if profiles.len() == len_before {
        anyhow::bail!("Profile not found: {}", id);
    }
    save_profiles(&profiles)?;
    print_info(&format!("Removed SMTP profile: {}", id));
    Ok(false)
}

fn cmd_smtp_send(
    profile_id: &str,
    to_addrs: &[String],
    cc_addrs: &[String],
    bcc_addrs: &[String],
    subject: &str,
    body: &str,
    attach_paths: &[String],
) -> Result<bool> {
    let profiles = load_profiles();
    let profile = profiles.iter().find(|p| p.id == profile_id || p.id.starts_with(profile_id))
        .ok_or_else(|| anyhow::anyhow!("Profile not found: {}", profile_id))?;

    if to_addrs.is_empty() {
        anyhow::bail!("Recipient required. Use --to <email>");
    }
    if subject.is_empty() {
        anyhow::bail!("Subject required. Use -s <subject>");
    }

    let password_plain = decode_password(&profile.password)
        .context("Failed to decode stored password")?;

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
                print_warning(&format!("No files found in directory: {}", path_str));
            }
            attachment_files.extend(entries);
        } else if p.is_file() {
            if is_blocked_executable(&p) {
                print_warning(&format!("Skipping executable file: {}", path_str));
            } else {
                attachment_files.push(p);
            }
        } else if p.exists() {
            print_warning(&format!("Skipping (not a file or directory): {}", path_str));
        } else {
            print_warning(&format!("Path not found: {}", path_str));
        }
    }

    // Remove duplicate files while preserving order
    let mut seen = std::collections::HashSet::new();
    attachment_files.retain(|p| seen.insert(p.canonicalize().unwrap_or_else(|_| p.to_path_buf())));

    // Build the email
    let from: Mailbox = format!("<{}>", profile.user).parse()
        .context("Invalid sender email address")?;

    let mut builder = Message::builder()
        .from(from)
        .subject(subject);

    // Add recipients
    for addr in to_addrs {
        let mb: Mailbox = addr.parse().context(format!("Invalid recipient: {}", addr))?;
        builder = builder.to(mb);
    }
    for addr in cc_addrs {
        let mb: Mailbox = addr.parse().context(format!("Invalid CC: {}", addr))?;
        builder = builder.cc(mb);
    }
    for addr in bcc_addrs {
        let mb: Mailbox = addr.parse().context(format!("Invalid BCC: {}", addr))?;
        builder = builder.bcc(mb);
    }

    // Build multipart message
    let mut multipart = MultiPart::mixed()
        .singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_PLAIN)
                .body(body.to_string()),
        );

    for file_path in &attachment_files {
        let data = fs::read(file_path)
            .with_context(|| format!("Failed to read attachment: {}", file_path.display()))?;
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

    let msg = builder
        .multipart(multipart)
        .context("Failed to build email message")?;

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
            print_error(&format!("Failed to configure SMTP transport: {}", e));
            return Ok(false);
        }
    };

    print_info(&format!("Sending email to {}...", to_addrs.join(", ")));

    match mailer.send(&msg) {
        Ok(_) => {
            let file_count = attachment_files.len();
            let file_info = if file_count > 0 {
                format!(" ({} attachment{})", file_count, if file_count == 1 { "" } else { "s" })
            } else {
                String::new()
            };
            print_success(&format!("Email sent successfully{}", file_info));
        }
        Err(e) => {
            print_error(&format!("Failed to send email: {}", e));
        }
    }

    Ok(false)
}

fn is_blocked_executable(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()).unwrap_or(""),
        "exe" | "dll" | "msi" | "msp" | "mst" | "bat" | "cmd"
        | "ps1" | "psm1" | "psd1" | "scr" | "com" | "pif"
        | "apk" | "xapk" | "ipa" | "app" | "dmg" | "pkg"
        | "sh" | "bash" | "zsh" | "ksh" | "bin"
        | "deb" | "rpm" | "jar"
        | "vb" | "vbs" | "vbe"
        | "wsf" | "wsh"
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
    println!("{}", "SMTP Email Client".bold());
    println!();
    println!("{}", "Create a profile:".cyan().bold());
    println!("  smtp -i <server> -p <port> -u <user> -pwd <password>");
    println!("  Example: smtp -i smtp.gmail.com -p 587 -u me@gmail.com -pwd app123");
    println!();
    println!("{}", "List profiles:".cyan().bold());
    println!("  smtp --id-ls");
    println!();
    println!("{}", "Show profile info:".cyan().bold());
    println!("  smtp --id-info <id>");
    println!();
    println!("{}", "Delete a profile:".cyan().bold());
    println!("  smtp --id-rm <id>");
    println!();
    println!("{}", "Send email:".cyan().bold());
    println!("  smtp --id <id> --to <email> [--cc <email>] [--bcc <email>] -s <subject> -b <body> [-f <files>]");
    println!();
    println!("{}", "Attachments:".cyan().bold());
    println!("  -f accepts comma-separated file paths and directories (files are expanded recursively)");
    println!("  Example: smtp --id a1b2 --to user@test.com -s \"Report\" -b \"See attached\" -f \"./report.pdf,./images/\"");
    println!();
    println!("{}", "Ports:".cyan().bold());
    println!("  587  STARTTLS (default)");
    println!("  465  TLS (SMTPS)");
    println!("  25   Plain (if supported by server)");
}
