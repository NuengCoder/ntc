//! node — version listing, download source and installer.

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

fn fetch_node_versions() -> Result<Vec<String>> {
    let url = "https://nodejs.org/dist/index.json";
    let response = ureq::get(url)
        .call()
        .context("Failed to fetch Node.js version list")?;

    let mut body = response.into_body();
    let mut reader = body.as_reader();
    let mut raw = String::new();
    reader
        .read_to_string(&mut raw)
        .context("Failed to read Node.js version response")?;

    let entries: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse Node.js version JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for entry in entries {
        let ver_str = match entry.get("version").and_then(|v| v.as_str()) {
            Some(v) => v,
            None => continue,
        };
        let stripped = ver_str.strip_prefix('v').unwrap_or(ver_str);
        if !is_node_valid_version(stripped) {
            continue;
        }
        if !versions.iter().any(|v| v == stripped) {
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

fn is_node_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn node_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://nodejs.org/dist/v{version}/");

    let entry = platform_node_archive(version)?;
    let url = format!("{base}{}", entry.1);

    let mut urls = HashMap::new();
    urls.insert(entry.0.to_string(), url);
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{}node-v{version}-linux-x64.tar.gz", base),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{}node-v{version}-linux-arm64.tar.gz", base),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{}node-v{version}-darwin-x64.tar.gz", base),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{}node-v{version}-darwin-arm64.tar.gz", base),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("{}node-v{version}-win-x64.zip", base),
    );
    urls.insert(
        "windows-aarch64".to_string(),
        format!("{}node-v{version}-win-arm64.zip", base),
    );

    Some(PackageSource {
        name: "node".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: entry.2,
    })
}

fn platform_node_archive(version: &str) -> Option<(&'static str, String, ArchiveType)> {
    #[cfg(all(
        any(target_os = "linux", target_os = "android"),
        target_arch = "x86_64"
    ))]
    {
        let f = format!("node-v{version}-linux-x64.tar.gz");
        Some(("linux-x86_64", f, ArchiveType::TarGz))
    }

    #[cfg(all(
        any(target_os = "linux", target_os = "android"),
        target_arch = "aarch64"
    ))]
    {
        let f = format!("node-v{version}-linux-arm64.tar.gz");
        Some(("linux-aarch64", f, ArchiveType::TarGz))
    }

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        let f = format!("node-v{version}-darwin-x64.tar.gz");
        Some(("macos-x86_64", f, ArchiveType::TarGz))
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        let f = format!("node-v{version}-darwin-arm64.tar.gz");
        Some(("macos-aarch64", f, ArchiveType::TarGz))
    }

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        let f = format!("node-v{version}-win-x64.zip");
        Some(("windows-x86_64", f, ArchiveType::Zip))
    }

    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    {
        let f = format!("node-v{version}-win-arm64.zip");
        Some(("windows-aarch64", f, ArchiveType::Zip))
    }

    #[cfg(not(any(
        all(
            any(target_os = "linux", target_os = "android"),
            any(target_arch = "x86_64", target_arch = "aarch64")
        ),
        all(
            target_os = "macos",
            any(target_arch = "x86_64", target_arch = "aarch64")
        ),
        all(
            target_os = "windows",
            any(target_arch = "x86_64", target_arch = "aarch64")
        ),
    )))]
    {
        None
    }
}

pub(crate) fn install_prebuilt_node(
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

    let ext = match source.archive_type {
        ArchiveType::TarGz => "tar.gz",
        ArchiveType::Zip => "zip",
        _ => "tar.gz",
    };
    let archive_name = format!("{name}-{version}.{ext}");

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

    let node_bin = find_binary(&tmp_dir, "node")
        .ok_or_else(|| anyhow::anyhow!("Could not find node binary in archive"))?;
    let extracted_bin = node_bin
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid binary path"))?;

    let target = bin_dir(name, version);
    fs::create_dir_all(&target)?;
    copy_recursive(extracted_bin, &target)?;

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

pub(crate) static PKG: Pkg = Pkg {
    name: "node",
    termux: Some("nodejs"),
    description_key: "ntpkg.src_node",
    versions: fetch_node_versions,
    source: node_source,
    install: install_prebuilt_node,
};
