//! Catalog listing and download helpers for the community `ntc_ral` and
//! `ntc_theme` folders in the ntc GitHub repository.

use anyhow::{bail, Context, Result};
use serde_json::Value;
use std::io::Read;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const API_CONTENTS: &str = "https://api.github.com/repos/NuengCoder/ntc/contents";
const RAW_BASE: &str = "https://raw.githubusercontent.com/NuengCoder/ntc/main";
const USER_AGENT: &str = "ntc-install";
const MAX_FILE_BYTES: u64 = 4 * 1024 * 1024;

#[derive(Debug, Clone)]
pub(crate) struct Entry {
    pub name: String,
    pub url: String,
    pub size: u64,
}

impl Entry {
    pub(crate) fn size_label(&self) -> String {
        if self.size == 0 {
            String::new()
        } else if self.size >= 1024 {
            format!("{:.1} KB", self.size as f64 / 1024.0)
        } else {
            format!("{} B", self.size)
        }
    }
}

fn read_limited(mut body: ureq::Body, url: &str) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut reader = body.as_reader();
    let mut chunk = vec![0u8; 64 * 1024];
    loop {
        let n = reader
            .read(&mut chunk)
            .with_context(|| format!("read {url}"))?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
        if buf.len() as u64 > MAX_FILE_BYTES {
            bail!(
                "download from {} is too large (limit {} MB)",
                url,
                MAX_FILE_BYTES / (1024 * 1024)
            );
        }
    }
    Ok(buf)
}

/// List regular files ending in `suffix` from `dir` on the main branch.
pub(crate) fn list(dir: &str, suffix: &str) -> Result<Vec<Entry>> {
    let url = format!("{API_CONTENTS}/{dir}?ref=main");
    let mut response = ureq::get(&url)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github+json")
        .call()
        .with_context(|| format!("request {url}"))?;
    let raw = response
        .body_mut()
        .read_to_string()
        .with_context(|| format!("read {url}"))?;
    let value: Value = serde_json::from_str(&raw).with_context(|| format!("parse {url}"))?;
    let items = value
        .as_array()
        .context("unexpected GitHub contents response")?;

    let mut entries = Vec::new();
    for item in items {
        if item.get("type").and_then(Value::as_str) != Some("file") {
            continue;
        }
        let Some(name) = item.get("name").and_then(Value::as_str) else {
            continue;
        };
        if !name.ends_with(suffix) {
            continue;
        }
        let url = item
            .get("download_url")
            .and_then(Value::as_str)
            .filter(|url| !url.is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| format!("{RAW_BASE}/{dir}/{name}"));
        let size = item.get("size").and_then(Value::as_u64).unwrap_or(0);
        entries.push(Entry {
            name: name.to_string(),
            url,
            size,
        });
    }

    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(entries)
}

/// Download a catalog file, guarding against unexpectedly large responses.
pub(crate) fn download(url: &str) -> Result<Vec<u8>> {
    let response = ureq::get(url)
        .header("User-Agent", USER_AGENT)
        .call()
        .with_context(|| format!("download {url}"))?;
    read_limited(response.into_body(), url)
}

/// A unique temporary path for importing a downloaded catalog file.
pub(crate) fn temp_path(prefix: &str, extension: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    std::env::temp_dir().join(format!(
        "ntc_{prefix}_{}_{}.{extension}",
        std::process::id(),
        stamp
    ))
}
