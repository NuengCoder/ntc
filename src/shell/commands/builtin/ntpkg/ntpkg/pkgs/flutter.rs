//! flutter — version listing, download source and installer.

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

fn fetch_flutter_versions() -> Result<Vec<String>> {
    let url = "https://storage.googleapis.com/flutter_infra_release/releases/releases_linux.json";
    let response = ureq::get(url)
        .call()
        .context("Failed to fetch Flutter versions from Google Storage")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read Flutter releases JSON")?;

    let releases: serde_json::Value =
        serde_json::from_str(&raw).context("Failed to parse Flutter releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    if let Some(arr) = releases["releases"].as_array() {
        for entry in arr {
            if let Some(ver) = entry["version"].as_str() {
                if is_flutter_valid_version(ver) && !versions.iter().any(|v| v == ver) {
                    versions.push(ver.to_string());
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

fn is_flutter_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn flutter_source(version: &str) -> Option<PackageSource> {
    let mut urls = HashMap::new();
    urls.insert(
        "windows-x86_64".to_string(),
        format!("https://storage.googleapis.com/flutter_infra_release/releases/stable/windows/flutter_windows_{version}-stable.zip"),
    );
    urls.insert(
        "linux-x86_64".to_string(),
        format!("https://storage.googleapis.com/flutter_infra_release/releases/stable/linux/flutter_linux_{version}-stable.tar.xz"),
    );
    let mac_url = format!("https://storage.googleapis.com/flutter_infra_release/releases/stable/macos/flutter_macos_{version}-stable.zip");
    urls.insert("macos-x86_64".to_string(), mac_url.clone());
    urls.insert("macos-aarch64".to_string(), mac_url);

    let archive_type = platform_flutter_archive();

    Some(PackageSource {
        name: "flutter".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

fn platform_flutter_archive() -> ArchiveType {
    #[cfg(any(target_os = "linux"))]
    {
        ArchiveType::TarXz
    }

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        ArchiveType::Zip
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        ArchiveType::Zip
    }
}

pub(crate) fn install_flutter(
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

    let archive_path = tmp_dir.join("archive");
    print_info(&format!("Downloading {name} SDK {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let _ = fs::remove_file(&archive_path);

    let flutter_root = tmp_dir.join("flutter");
    if !flutter_root.is_dir() {
        anyhow::bail!("Expected 'flutter' directory in archive, not found");
    }

    fs::create_dir_all(vdir)?;
    copy_recursive(&flutter_root, vdir)?;

    let bin_path = vdir.join("bin");
    fs::write(
        vdir.join(".real_path"),
        bin_path.to_string_lossy().as_bytes(),
    )
    .ok();

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!(
        "{name} SDK {version} installed at {}",
        vdir.display()
    ));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

pub(crate) static PKG: Pkg = Pkg {
    name: "flutter",
    termux: Some("flutter"),
    description_key: "ntpkg.src_flutter",
    versions: fetch_flutter_versions,
    source: flutter_source,
    install: install_flutter,
};
