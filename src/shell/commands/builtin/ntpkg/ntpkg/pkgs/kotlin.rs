//! kotlin — version listing, download source and installer.

use super::super::download::download_with_progress;
use super::super::download::extract_archive;
use super::super::manager::load_manifest;
use super::super::manager::save_manifest;
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

fn fetch_kotlin_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/JetBrains/kotlin/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch Kotlin versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        let stripped = tag.strip_prefix('v').unwrap_or(tag);
        if is_kotlin_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
            versions.push(stripped.to_string());
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn is_kotlin_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn kotlin_source(version: &str) -> Option<PackageSource> {
    let url = format!(
        "https://github.com/JetBrains/kotlin/releases/download/v{version}/kotlin-compiler-{version}.zip"
    );
    let mut urls = HashMap::new();
    urls.insert("linux-x86_64".to_string(), url.clone());
    urls.insert("linux-aarch64".to_string(), url.clone());
    urls.insert("macos-x86_64".to_string(), url.clone());
    urls.insert("macos-aarch64".to_string(), url.clone());
    urls.insert("windows-x86_64".to_string(), url.clone());
    urls.insert("windows-aarch64".to_string(), url);

    Some(PackageSource {
        name: "kotlin".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::Zip,
    })
}

pub(crate) fn install_kotlin(
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

    let archive_name = format!("kotlin-compiler-{version}.zip");

    let tmp_dir = std::env::temp_dir().join(format!("ntc-install-{name}-{version}"));
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    let archive_path = tmp_dir.join(&archive_name);
    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let kotlin_bin = find_kotlin_binary(&tmp_dir)
        .ok_or_else(|| anyhow::anyhow!("Could not find kotlin binary in archive"))?;
    let kotlin_root = kotlin_bin
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| anyhow::anyhow!("Invalid Kotlin directory structure"))?;

    fs::create_dir_all(vdir)?;
    copy_recursive(kotlin_root, vdir)?;

    let bin_path = vdir.join("bin");
    fs::write(
        vdir.join(".real_path"),
        bin_path.to_string_lossy().as_bytes(),
    )
    .context("Failed to write .real_path")?;

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

fn find_kotlin_binary(dir: &Path) -> Option<std::path::PathBuf> {
    let target = if cfg!(windows) {
        "kotlin.bat"
    } else {
        "kotlin"
    };
    find_file_recursive(dir, target)
}

fn find_file_recursive(dir: &Path, name: &str) -> Option<std::path::PathBuf> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(found) = find_file_recursive(&path, name) {
                    return Some(found);
                }
            } else if path.file_name().and_then(|n| n.to_str()) == Some(name) {
                return Some(path);
            }
        }
    }
    None
}

pub(crate) static PKG: Pkg = Pkg {
    name: "kotlin",
    termux: Some("kotlin"),
    description_key: "ntpkg.src_kotlin",
    versions: fetch_kotlin_versions,
    source: kotlin_source,
    install: install_kotlin,
};
