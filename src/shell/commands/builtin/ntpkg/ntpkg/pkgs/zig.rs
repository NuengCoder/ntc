//! zig — version listing, download source and installer.

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

fn fetch_zig_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/ziglang/zig/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch Zig versions from GitHub")?;

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
        if is_zig_valid_version(tag) && !versions.iter().any(|v| v == tag) {
            versions.push(tag.to_string());
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn is_zig_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn zig_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://ziglang.org/download/{version}");

    let mut urls = HashMap::new();

    let platforms = [
        ("linux-x86_64", false),
        ("linux-aarch64", false),
        ("macos-x86_64", false),
        ("macos-aarch64", false),
        ("windows-x86_64", true),
        ("windows-aarch64", true),
    ];

    for (platform, is_zip) in &platforms {
        let ext = if *is_zip { "zip" } else { "tar.xz" };
        let filename = zig_filename(version, platform, ext);
        urls.insert(platform.to_string(), format!("{base}/{filename}"));
    }

    Some(PackageSource {
        name: "zig".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::TarXz,
    })
}

fn zig_filename(version: &str, platform: &str, ext: &str) -> String {
    let (os, arch) = platform.split_once('-').unwrap_or((platform, ""));
    let prefix = if uses_new_zig_naming(version) {
        format!("{arch}-{os}")
    } else {
        format!("{os}-{arch}")
    };
    format!("zig-{prefix}-{version}.{ext}")
}

fn uses_new_zig_naming(version: &str) -> bool {
    let parts: Vec<u32> = version.split('.').filter_map(|p| p.parse().ok()).collect();
    if parts.len() >= 2 {
        let major = parts[0];
        let minor = parts[1];
        let patch = parts.get(2).copied().unwrap_or(0);
        (major > 0) || (minor > 14) || (minor == 14 && patch >= 1)
    } else {
        false
    }
}

pub(crate) fn install_zig(
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

    let archive_type = if url.ends_with(".zip") {
        ArchiveType::Zip
    } else {
        ArchiveType::TarXz
    };

    let fname = url.rsplit_once('/').map(|(_, f)| f).unwrap_or("archive");
    let archive_path = tmp_dir.join(fname);
    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, archive_type)?;

    let zig_bin = find_zig_binary(&tmp_dir)
        .ok_or_else(|| anyhow::anyhow!("Could not find zig binary in archive"))?;
    let zig_root = zig_bin
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid Zig directory structure"))?;

    fs::create_dir_all(vdir)?;
    copy_recursive(zig_root, vdir)?;

    fs::write(vdir.join(".real_path"), vdir.to_string_lossy().as_bytes())
        .context("Failed to write .real_path")?;

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

fn find_zig_binary(dir: &Path) -> Option<std::path::PathBuf> {
    let target = if cfg!(windows) { "zig.exe" } else { "zig" };
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
    name: "zig",
    termux: Some("zig"),
    description_key: "ntpkg.src_zig",
    versions: fetch_zig_versions,
    source: zig_source,
    install: install_zig,
};
