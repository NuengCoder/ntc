//! `smtp` profile storage and profile subcommands.

use crate::output::{print_info, print_success};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub(super) struct SmtpProfile {
    pub(super) id: String,
    pub(super) server: String,
    pub(super) port: u16,
    pub(super) user: String,
    pub(super) password: String,
    pub(super) created: String,
}

pub(super) fn profiles_db_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("smtp_profiles.json")
}

pub(super) fn load_profiles() -> Vec<SmtpProfile> {
    let path = profiles_db_path();
    if !path.exists() {
        return Vec::new();
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub(super) fn save_profiles(profiles: &[SmtpProfile]) -> Result<()> {
    let path = profiles_db_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let data = serde_json::to_string_pretty(profiles)?;
    fs::write(&path, data)?;
    Ok(())
}

pub(super) fn generate_id() -> String {
    let id: u32 = rand::random();
    format!("{:08x}", id)
}

pub(super) fn encode_password(pwd: &str) -> String {
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, pwd.as_bytes())
}

pub(super) fn decode_password(encoded: &str) -> Result<String> {
    let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encoded)?;
    String::from_utf8(bytes)
        .map_err(|e| anyhow::anyhow!("{}", t!("cmd.smtp.invalid_pwd_encoding", e)))
}

pub(super) fn cmd_smtp_create(server: &str, port: u16, user: &str, password: &str) -> Result<bool> {
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

pub(super) fn cmd_smtp_ls() -> Result<bool> {
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

pub(super) fn cmd_smtp_info(id: &str, reveal_password: bool) -> Result<bool> {
    let profiles = load_profiles();
    let profile = profiles
        .iter()
        .find(|p| p.id == id || p.id.starts_with(id))
        .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.profile_not_found", id)))?;

    // The app password is never printed by default: `--deadlock` unlocks the
    // deadlock vault to prove the user knows the master password first.
    let pwd_display = if reveal_password {
        crate::shell::commands::builtin::deadlock::deadlock::unlock_required()?;
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

pub(super) fn cmd_smtp_rm(id: &str) -> Result<bool> {
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
