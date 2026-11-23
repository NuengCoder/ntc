//! stockfish — version listing, download source and installer.

use super::super::download::download_with_progress;
use super::super::download::extract_archive;
use super::super::manager::load_manifest;
use super::super::manager::save_manifest;
use super::super::types::bin_dir;
use super::super::types::ArchiveType;
use super::super::types::PackageSource;
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
use std::path::PathBuf;

fn fetch_stockfish_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/official-stockfish/Stockfish/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch Stockfish versions from GitHub")?;

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
        // Release tags look like `sf_19` / `sf_17.1`; development
        // snapshots (`stockfish-dev-...`) are intentionally skipped.
        let Some(stripped) = tag.strip_prefix("sf_") else {
            continue;
        };
        if is_stockfish_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
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

fn is_stockfish_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() > 2 {
        return false;
    }
    parts
        .iter()
        .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

fn stockfish_major(version: &str) -> u32 {
    version
        .split('.')
        .next()
        .and_then(|p| p.parse().ok())
        .unwrap_or(0)
}

/// Release asset name for a platform. Stockfish 19 and newer publish
/// universal binaries; older releases ship one binary per CPU feature set.
fn stockfish_asset_name(
    os: &str,
    arch: &str,
    major: u32,
    avx2: bool,
    bmi2: bool,
) -> Option<&'static str> {
    if major >= 19 {
        return match (os, arch) {
            ("windows", "x86_64") => Some("stockfish-windows-x86-64-universal.zip"),
            ("windows", "aarch64") => Some("stockfish-windows-arm64-universal.zip"),
            ("linux", "x86_64") => Some("stockfish-linux-x86-64-universal.tar.gz"),
            ("linux", "aarch64") => Some("stockfish-linux-arm64-universal.tar.gz"),
            ("macos", _) => Some("stockfish-macos-universal.tar.gz"),
            _ => None,
        };
    }

    match (os, arch) {
        ("windows", "x86_64") if avx2 => Some("stockfish-windows-x86-64-avx2.zip"),
        ("windows", "x86_64") if bmi2 => Some("stockfish-windows-x86-64-bmi2.zip"),
        ("windows", "x86_64") => Some("stockfish-windows-x86-64.zip"),
        ("windows", "aarch64") => Some("stockfish-windows-armv8.zip"),
        ("linux", "x86_64") if avx2 => Some("stockfish-ubuntu-x86-64-avx2.tar"),
        ("linux", "x86_64") if bmi2 => Some("stockfish-ubuntu-x86-64-bmi2.tar"),
        ("linux", "x86_64") => Some("stockfish-ubuntu-x86-64.tar"),
        ("macos", "aarch64") => Some("stockfish-macos-m1-apple-silicon.tar"),
        ("macos", "x86_64") if avx2 => Some("stockfish-macos-x86-64-avx2.tar"),
        ("macos", "x86_64") => Some("stockfish-macos-x86-64.tar"),
        _ => None,
    }
}

fn stockfish_archive_for(name: &str) -> ArchiveType {
    let n = name.to_ascii_lowercase();
    if n.ends_with(".zip") {
        ArchiveType::Zip
    } else if n.ends_with(".tar.gz") || n.ends_with(".tgz") {
        ArchiveType::TarGz
    } else {
        ArchiveType::Tar
    }
}

#[cfg(target_arch = "x86_64")]
fn stockfish_cpu_features() -> (bool, bool) {
    (
        std::is_x86_feature_detected!("avx2"),
        std::is_x86_feature_detected!("bmi2"),
    )
}

#[cfg(not(target_arch = "x86_64"))]
fn stockfish_cpu_features() -> (bool, bool) {
    (false, false)
}

fn stockfish_source(version: &str) -> Option<PackageSource> {
    let major = stockfish_major(version);
    let key = platform_key();
    let (os, arch) = key.split_once('-')?;
    let (avx2, bmi2) = stockfish_cpu_features();

    let mut urls = HashMap::new();
    if let Some(name) = stockfish_asset_name(os, arch, major, avx2, bmi2) {
        urls.insert(
            key.clone(),
            format!(
                "https://github.com/official-stockfish/Stockfish/releases/download/sf_{version}/{name}"
            ),
        );
    }

    let archive_type = urls
        .get(&key)
        .map(|u| stockfish_archive_for(u))
        .unwrap_or(ArchiveType::Zip);

    Some(PackageSource {
        name: "stockfish".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

pub(crate) fn install_stockfish(
    name: &str,
    version: &str,
    source: &PackageSource,
    vdir: &Path,
) -> Result<()> {
    let platform_key = platform_key();
    let url = source.urls.get(&platform_key).ok_or_else(|| {
        anyhow::anyhow!("Stockfish has no prebuilt binary for the '{platform_key}' platform")
    })?;

    let tmp_dir = std::env::temp_dir().join(format!("ntc-install-{name}-{version}"));
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    let fname = url.rsplit_once('/').map(|(_, f)| f).unwrap_or("archive");
    let archive_path = tmp_dir.join(fname);

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, archive_kind(url))?;

    let found = find_stockfish_binary(&tmp_dir).ok_or_else(|| {
        anyhow::anyhow!("Could not find the Stockfish binary in the downloaded archive")
    })?;

    let target_dir = bin_dir(name, version);
    fs::create_dir_all(&target_dir)?;
    let dest = target_dir.join(if cfg!(windows) {
        "stockfish.exe"
    } else {
        "stockfish"
    });
    fs::copy(&found, &dest)
        .with_context(|| format!("Failed to copy Stockfish binary to {}", dest.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&dest, fs::Permissions::from_mode(0o755))?;
    }

    fs::write(
        vdir.join(".real_path"),
        target_dir.to_string_lossy().as_bytes(),
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

fn archive_kind(url: &str) -> ArchiveType {
    let u = url.to_ascii_lowercase();
    if u.ends_with(".zip") {
        ArchiveType::Zip
    } else if u.ends_with(".tar.gz") || u.ends_with(".tgz") {
        ArchiveType::TarGz
    } else {
        ArchiveType::Tar
    }
}

/// Official archives name the engine after the build
/// (`stockfish-windows-x86-64-universal.exe`, `stockfish-ubuntu-x86-64`),
/// so accept any `stockfish`/`stockfish-*` executable as well as the
/// plain name some builds use.
fn find_stockfish_binary(dir: &Path) -> Option<PathBuf> {
    let mut files = Vec::new();
    collect_files(dir, &mut files);

    let exact = if cfg!(windows) {
        "stockfish.exe"
    } else {
        "stockfish"
    };
    if let Some(hit) = files
        .iter()
        .find(|p| p.file_name().and_then(|n| n.to_str()) == Some(exact))
    {
        return Some(hit.clone());
    }

    files.into_iter().find(|p| {
        let Some(name) = p.file_name().and_then(|n| n.to_str()) else {
            return false;
        };
        let n = name.to_ascii_lowercase();
        if cfg!(windows) {
            n.starts_with("stockfish-") && n.ends_with(".exe")
        } else {
            n.starts_with("stockfish-") && !n.contains('.')
        }
    })
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files(&path, out);
            } else {
                out.push(path);
            }
        }
    }
}

pub(crate) static PKG: Pkg = Pkg {
    name: "stockfish",
    termux: Some("stockfish"),
    description_key: "ntpkg.src_stockfish",
    versions: fetch_stockfish_versions,
    source: stockfish_source,
    install: install_stockfish,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stockfish_versions_are_parsed_strictly() {
        assert!(is_stockfish_valid_version("19"));
        assert!(is_stockfish_valid_version("17.1"));
        assert!(!is_stockfish_valid_version(""));
        assert!(!is_stockfish_valid_version("dev-1"));
        assert!(!is_stockfish_valid_version("17.1.2"));
        assert!(!is_stockfish_valid_version("17."));
        assert_eq!(stockfish_major("19"), 19);
        assert_eq!(stockfish_major("17.1"), 17);
    }

    #[test]
    fn stockfish_assets_follow_release_naming() {
        // Stockfish 19+ ships universal binaries per OS.
        assert_eq!(
            stockfish_asset_name("windows", "x86_64", 19, false, false),
            Some("stockfish-windows-x86-64-universal.zip")
        );
        assert_eq!(
            stockfish_asset_name("windows", "aarch64", 19, false, false),
            Some("stockfish-windows-arm64-universal.zip")
        );
        assert_eq!(
            stockfish_asset_name("linux", "x86_64", 19, false, false),
            Some("stockfish-linux-x86-64-universal.tar.gz")
        );
        assert_eq!(
            stockfish_asset_name("linux", "aarch64", 19, false, false),
            Some("stockfish-linux-arm64-universal.tar.gz")
        );
        assert_eq!(
            stockfish_asset_name("macos", "x86_64", 19, false, false),
            Some("stockfish-macos-universal.tar.gz")
        );
        assert_eq!(
            stockfish_asset_name("macos", "aarch64", 19, false, false),
            Some("stockfish-macos-universal.tar.gz")
        );

        // Older releases stay on the per-CPU asset names.
        assert_eq!(
            stockfish_asset_name("windows", "x86_64", 17, true, true),
            Some("stockfish-windows-x86-64-avx2.zip")
        );
        assert_eq!(
            stockfish_asset_name("windows", "x86_64", 17, false, true),
            Some("stockfish-windows-x86-64-bmi2.zip")
        );
        assert_eq!(
            stockfish_asset_name("windows", "x86_64", 17, false, false),
            Some("stockfish-windows-x86-64.zip")
        );
        assert_eq!(
            stockfish_asset_name("linux", "x86_64", 17, false, false),
            Some("stockfish-ubuntu-x86-64.tar")
        );
        assert_eq!(
            stockfish_asset_name("macos", "aarch64", 17, false, false),
            Some("stockfish-macos-m1-apple-silicon.tar")
        );
        assert_eq!(
            stockfish_asset_name("macos", "x86_64", 17, false, false),
            Some("stockfish-macos-x86-64.tar")
        );
        // No prebuilt Linux arm64 engine before the universal builds.
        assert_eq!(
            stockfish_asset_name("linux", "aarch64", 17, false, false),
            None
        );

        assert_eq!(
            stockfish_archive_for("stockfish-windows-x86-64-universal.zip"),
            ArchiveType::Zip
        );
        assert_eq!(
            stockfish_archive_for("stockfish-macos-universal.tar.gz"),
            ArchiveType::TarGz
        );
        assert_eq!(
            stockfish_archive_for("stockfish-ubuntu-x86-64.tar"),
            ArchiveType::Tar
        );
    }

    #[test]
    fn stockfish_source_targets_current_platform() {
        let src = stockfish_source("19").expect("stockfish source should resolve");
        let key = platform_key();
        let Some(url) = src.urls.get(&key) else {
            // Architectures without a prebuilt engine may stay empty.
            return;
        };
        assert!(
            url.starts_with(
                "https://github.com/official-stockfish/Stockfish/releases/download/sf_19/"
            ),
            "unexpected stockfish URL: {url}"
        );
        assert!(url.contains("stockfish-"));
        assert_eq!(src.archive_type, stockfish_archive_for(url));
    }
}
