//! rust — version listing, download source and installer.

use super::super::download::download_with_progress;
use super::super::download::extract_archive;
use super::super::download::find_binary;
use super::super::manager::load_manifest;
use super::super::manager::save_manifest;
use super::super::types::bin_dir;
use super::super::types::ArchiveType;
use super::super::types::PackageSource;
use super::common::copy_recursive;
use super::common::platform_key;
use super::Pkg;
use crate::output::print_info;
use crate::output::print_success;
use crate::t;
use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;

fn fetch_rust_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/rust-lang/rust/tags?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch Rust versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let tags: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub tags JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for tag in tags {
        if let Some(name) = tag["name"].as_str() {
            if is_rust_version(name) && !versions.iter().any(|v| v == name) {
                versions.push(name.to_string());
            }
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn is_rust_version(s: &str) -> bool {
    if s.is_empty() || s.starts_with('0') {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn rust_source(version: &str) -> Option<PackageSource> {
    let manifest_url = format!("https://static.rust-lang.org/dist/channel-rust-{version}.toml");
    let response = ureq::get(&manifest_url).call().ok()?;
    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader().read_to_string(&mut raw).ok()?;

    let manifest: toml::Value = toml::from_str(&raw).ok()?;
    let date = manifest.get("date")?.as_str()?;

    let base = format!("https://static.rust-lang.org/dist/{date}");

    let target_map: [(&str, &str); 6] = [
        ("linux-x86_64", "x86_64-unknown-linux-gnu"),
        ("linux-aarch64", "aarch64-unknown-linux-gnu"),
        ("macos-x86_64", "x86_64-apple-darwin"),
        ("macos-aarch64", "aarch64-apple-darwin"),
        ("windows-x86_64", "x86_64-pc-windows-msvc"),
        ("windows-aarch64", "aarch64-pc-windows-msvc"),
    ];

    let mut urls = HashMap::new();
    for (key, target) in &target_map {
        let url = format!("{base}/rust-{version}-{target}.tar.gz");
        urls.insert(key.to_string(), url);
    }

    Some(PackageSource {
        name: "rust".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::TarGz,
    })
}

pub(crate) fn install_rust(
    name: &str,
    version: &str,
    source: &PackageSource,
    vdir: &Path,
) -> Result<()> {
    let platform_key = platform_key();
    let url = source
        .urls
        .get(&platform_key)
        .ok_or_else(|| anyhow::anyhow!("No download URL for platform '{platform_key}'"))?;

    let tmp_dir = std::env::temp_dir().join(format!("ntc-install-{name}-{version}"));
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    let archive_name = format!("{name}-{version}.tar.gz");
    let archive_path = tmp_dir.join(&archive_name);
    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let rustc_bin = find_binary(&tmp_dir, "rustc")
        .ok_or_else(|| anyhow::anyhow!("Could not find rustc binary in archive"))?;
    let cargo_bin = find_binary(&tmp_dir, "cargo")
        .ok_or_else(|| anyhow::anyhow!("Could not find cargo binary in archive"))?;

    let rustc_bin_dir = rustc_bin
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid rustc binary path"))?;
    let cargo_bin_dir = cargo_bin
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid cargo binary path"))?;

    let target = bin_dir(name, version);
    fs::create_dir_all(&target)?;
    copy_recursive(rustc_bin_dir, &target)?;
    copy_recursive(cargo_bin_dir, &target)?;

    fs::write(vdir.join(".real_path"), target.to_string_lossy().as_bytes()).ok();

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

pub(crate) static PKG: Pkg = Pkg {
    name: "rust",
    termux: Some("rust"),
    description_key: "ntpkg.src_rust",
    versions: fetch_rust_versions,
    source: rust_source,
    install: install_rust,
};
