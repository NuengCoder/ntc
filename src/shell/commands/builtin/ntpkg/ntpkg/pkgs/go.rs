//! go — version listing, download source and installer.

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

fn fetch_go_versions() -> Result<Vec<String>> {
    let url = "https://go.dev/dl/?mode=json";
    let response = ureq::get(url)
        .call()
        .context("Failed to fetch Go version list from go.dev")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read go.dev response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse go.dev JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        if release["stable"].as_bool().unwrap_or(false) {
            if let Some(v) = release["version"].as_str() {
                let cleaned = v.strip_prefix("go").unwrap_or(v).to_string();
                if !versions.iter().any(|x| x == &cleaned) {
                    versions.push(cleaned);
                }
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

fn go_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://go.dev/dl/go{version}");

    let mut urls = HashMap::new();
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{base}.linux-amd64.tar.gz"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{base}.linux-arm64.tar.gz"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{base}.darwin-amd64.tar.gz"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{base}.darwin-arm64.tar.gz"),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("{base}.windows-amd64.zip"),
    );
    urls.insert(
        "windows-aarch64".to_string(),
        format!("{base}.windows-arm64.zip"),
    );

    let archive_type = platform_go_archive();

    Some(PackageSource {
        name: "go".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

fn platform_go_archive() -> ArchiveType {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        ArchiveType::TarGz
    }

    #[cfg(target_os = "windows")]
    {
        ArchiveType::Zip
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        ArchiveType::TarGz
    }
}

pub(crate) fn install_go(
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
        super::super::types::ArchiveType::TarGz => "tar.gz",
        super::super::types::ArchiveType::Zip => "zip",
        _ => "tar.gz",
    };
    let archive_name = format!("{name}-{version}.{ext}");

    let archive_path = tmp_dir.join(&archive_name);
    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let go_bin = find_binary(&tmp_dir, "go")
        .ok_or_else(|| anyhow::anyhow!("Could not find go binary in archive"))?;
    let go_root = go_bin
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| anyhow::anyhow!("Invalid Go directory structure"))?;

    fs::create_dir_all(vdir)?;
    copy_recursive(go_root, vdir)?;

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
    name: "go",
    termux: Some("golang"),
    description_key: "ntpkg.src_go",
    versions: fetch_go_versions,
    source: go_source,
    install: install_go,
};
