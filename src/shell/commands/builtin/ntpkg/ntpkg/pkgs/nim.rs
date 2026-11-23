//! nim — version listing, download source and installer.

use super::super::download::download_with_progress;
use super::super::download::extract_archive;
use super::super::download::find_binary;
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

fn fetch_nim_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/nim-lang/Nim/tags?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch Nim versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let tags: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub tags JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for tag_entry in tags {
        let tag = tag_entry["name"].as_str().unwrap_or("");
        let stripped = tag.strip_prefix('v').unwrap_or(tag);
        if is_nim_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
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

fn is_nim_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn nim_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://nim-lang.org/download/nim-{version}");

    let mut urls = HashMap::new();
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{base}-linux_x64.tar.xz"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{base}-linux_arm64.tar.xz"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{base}-macosx_x64.tar.xz"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{base}-macosx_arm64.tar.xz"),
    );
    urls.insert("windows-x86_64".to_string(), format!("{base}_x64.zip"));

    let archive_type = platform_nim_archive();

    Some(PackageSource {
        name: "nim".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

fn platform_nim_archive() -> ArchiveType {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        ArchiveType::TarXz
    }

    #[cfg(target_os = "windows")]
    {
        ArchiveType::Zip
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        ArchiveType::TarXz
    }
}

pub(crate) fn install_nim(
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

    let ext = match source.archive_type {
        super::super::types::ArchiveType::TarXz => "tar.xz",
        super::super::types::ArchiveType::TarGz => "tar.gz",
        super::super::types::ArchiveType::Zip => "zip",
        _ => "tar.xz",
    };
    let archive_name = format!("{name}-{version}.{ext}");

    let archive_path = tmp_dir.join(&archive_name);
    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let nim_bin = find_binary(&tmp_dir, "nim")
        .ok_or_else(|| anyhow::anyhow!("Could not find nim binary in archive"))?;
    let nim_root = nim_bin
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| anyhow::anyhow!("Invalid Nim directory structure"))?;

    fs::create_dir_all(vdir)?;
    copy_recursive(nim_root, vdir)?;

    let bin_path = vdir.join("bin");
    fs::write(
        vdir.join(".real_path"),
        bin_path.to_string_lossy().as_bytes(),
    )
    .ok();

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

pub(crate) static PKG: Pkg = Pkg {
    name: "nim",
    termux: Some("nim"),
    description_key: "ntpkg.src_nim",
    versions: fetch_nim_versions,
    source: nim_source,
    install: install_nim,
};
