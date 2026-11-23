//! `smtp` single-file JSON configuration (`ntc_smtp.json`).

use crate::output::{print_success, print_warning};
use crate::t;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

// ── ntc_smtp.json (single-file config) ─────────────────────────────────────

/// The on-disk config written by `ntc smtp init` and read by `-r`.
#[derive(Debug, Default)]
pub(super) struct SmtpFileConfig {
    pub(super) server: String,
    pub(super) port: u16,
    pub(super) user: String,
    pub(super) password: String,
    pub(super) to: Vec<String>,
    pub(super) files: Vec<String>,
}

pub(super) fn split_recipients(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Read `server`, `port`, `user`, `password`, `to` and `files` from a JSON
/// config. `to` accepts one address, a comma-separated string, or an array;
/// `files` accepts an array (or a single string).
pub(super) fn load_smtp_file(path: &Path) -> Result<SmtpFileConfig> {
    if !path.exists() {
        anyhow::bail!("{}", t!("cmd.smtp.file_missing", path.display()));
    }
    let raw = fs::read_to_string(path)
        .with_context(|| t!("cmd.smtp.file_read_fail", path.display()).to_string())?;
    let value: serde_json::Value = serde_json::from_str(&raw)
        .with_context(|| t!("cmd.smtp.file_invalid", path.display()).to_string())?;
    let text = |key: &str| {
        value
            .get(key)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    };
    let mut cfg = SmtpFileConfig {
        server: text("server"),
        port: value
            .get("port")
            .and_then(|v| v.as_u64())
            .and_then(|p| u16::try_from(p).ok())
            .unwrap_or(587),
        user: text("user"),
        password: text("password"),
        to: match value.get("to") {
            Some(serde_json::Value::String(s)) => split_recipients(s),
            Some(serde_json::Value::Array(items)) => items
                .iter()
                .filter_map(|v| v.as_str())
                .flat_map(split_recipients)
                .collect(),
            _ => Vec::new(),
        },
        files: match value.get("files") {
            Some(serde_json::Value::String(s)) => split_recipients(s),
            Some(serde_json::Value::Array(items)) => items
                .iter()
                .filter_map(|v| v.as_str())
                .flat_map(split_recipients)
                .collect(),
            _ => Vec::new(),
        },
    };
    if cfg.server.trim().is_empty() || cfg.user.trim().is_empty() || cfg.password.is_empty() {
        anyhow::bail!("{}", t!("cmd.smtp.file_incomplete", path.display()));
    }
    if cfg.port == 0 {
        cfg.port = 587;
    }
    Ok(cfg)
}

const SMTP_FILE_TEMPLATE: &str = r#"{
  "_usage": [
    "Fill in your SMTP details; this file is git-ignored because it holds a password.",
    "Send:    ntc smtp -r ntc_smtp.json -s \"TITLE\" -b \"BODY\"",
    "Attach:  ntc smtp -r ntc_smtp.json -s \"REPORT\" -b \"See attached\" -f report.pdf",
    "to:      default recipients (one address, a comma-separated list, or an array).",
    "files:   paths attached to every send (files or directories)."
  ],
  "server": "smtp.gmail.com",
  "port": 587,
  "user": "you@gmail.com",
  "password": "your-app-password",
  "to": "you@gmail.com",
  "files": []
}
"#;

pub(super) fn cmd_smtp_init(path: &Path, force: bool) -> Result<bool> {
    if path.exists() && !force {
        print_warning(&t!("cmd.smtp.file_exists", path.display()));
        return Ok(false);
    }
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path, SMTP_FILE_TEMPLATE)
        .with_context(|| t!("cmd.smtp.file_write_fail", path.display()).to_string())?;
    print_success(&t!("cmd.smtp.file_created", path.display()));
    Ok(false)
}
