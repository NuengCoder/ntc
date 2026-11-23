//! java — version listing, download source and installer.

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
use std::sync::Mutex;

/// Caches Java release data keyed by feature version (e.g., "21" -> release JSON array)
static JAVA_RELEASE_CACHE: Mutex<Option<HashMap<String, Vec<serde_json::Value>>>> =
    Mutex::new(None);

/// Returns supported Java feature versions (JDK 8, 11, 17, 21, 26).
fn fetch_java_versions() -> Result<Vec<String>> {
    Ok(vec![
        "8".to_string(),
        "11".to_string(),
        "17".to_string(),
        "21".to_string(),
        "26".to_string(),
    ])
}

/// Fetch available minor releases for a Java feature version.
pub(crate) fn list_java_minor_versions(feature: &str) -> Vec<String> {
    let url = format!("https://api.adoptium.net/v3/assets/feature_releases/{feature}/ga");

    let response = match ureq::get(&url).header("User-Agent", "ntc").call() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ntpkg: failed to fetch Java {feature} versions: {e}");
            return vec![];
        }
    };

    let mut body = response.into_body();
    let mut raw = String::new();
    if body.as_reader().read_to_string(&mut raw).is_err() {
        return vec![];
    }

    let releases: Vec<serde_json::Value> = match serde_json::from_str(&raw) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ntpkg: failed to parse Java {feature} releases JSON: {e}");
            return vec![];
        }
    };

    let mut versions: Vec<String> = Vec::new();
    for release in &releases {
        if let Some(name) = release["release_name"].as_str() {
            if !versions.iter().any(|v| v == name) {
                versions.push(name.to_string());
            }
        }
    }

    if let Ok(mut cache) = JAVA_RELEASE_CACHE.lock() {
        cache
            .get_or_insert_with(HashMap::new)
            .insert(feature.to_string(), releases);
    }

    versions
}

fn java_source(version: &str) -> Option<PackageSource> {
    let (os_key, arch_key) = adoptium_platform()?;

    // Try cache first
    let found_release = if let Ok(cache) = JAVA_RELEASE_CACHE.lock() {
        if let Some(ref map) = *cache {
            map.values()
                .flatten()
                .find(|r| r["release_name"].as_str() == Some(version))
                .cloned()
        } else {
            None
        }
    } else {
        None
    };

    let release = match found_release {
        Some(r) => r,
        None => {
            // Cache miss: extract feature version and fetch feature releases
            let feature = java_feature_from_version(version)?;
            let url = format!("https://api.adoptium.net/v3/assets/feature_releases/{feature}/ga");
            let response = ureq::get(&url).header("User-Agent", "ntc").call().ok()?;
            let mut body = response.into_body();
            let mut raw = String::new();
            body.as_reader().read_to_string(&mut raw).ok()?;
            let releases: Vec<serde_json::Value> = serde_json::from_str(&raw).ok()?;

            if let Ok(mut cache) = JAVA_RELEASE_CACHE.lock() {
                cache
                    .get_or_insert_with(HashMap::new)
                    .insert(feature.to_string(), releases.clone());
            }

            releases
                .into_iter()
                .find(|r| r["release_name"].as_str() == Some(version))?
        }
    };

    let binaries = release["binaries"].as_array()?;

    for binary in binaries {
        let b_os = binary["os"].as_str()?;
        let b_arch = binary["architecture"].as_str()?;
        let b_type = binary["image_type"].as_str()?;
        if b_os == os_key && b_arch == arch_key && b_type == "jdk" {
            let pkg = &binary["package"];
            let link = pkg["link"].as_str()?;
            let mut urls = HashMap::new();
            urls.insert(platform_key(), link.to_string());

            let archive_type = if link.ends_with(".zip") {
                ArchiveType::Zip
            } else {
                ArchiveType::TarGz
            };

            return Some(PackageSource {
                name: "java".to_string(),
                version: version.to_string(),
                urls,
                sha256: HashMap::new(),
                archive_type,
            });
        }
    }

    None
}

/// Extract feature version from a release name.
/// "jdk-21.0.11+10" -> "21", "jdk8u432-b06" -> "8"
fn java_feature_from_version(version: &str) -> Option<String> {
    let stripped = version
        .strip_prefix("jdk-")
        .or_else(|| version.strip_prefix("jdk"))?;
    if let Some(dot) = stripped.find('.') {
        Some(stripped[..dot].to_string())
    } else if let Some(u) = stripped.find('u') {
        Some(stripped[..u].to_string())
    } else if let Some(plus) = stripped.find('+') {
        Some(stripped[..plus].to_string())
    } else {
        // Bare number like "21"
        Some(stripped.to_string())
    }
}

fn adoptium_platform() -> Option<(&'static str, &'static str)> {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        Some(("windows", "x64"))
    }

    #[cfg(all(
        any(target_os = "linux", target_os = "android"),
        target_arch = "x86_64"
    ))]
    {
        Some(("linux", "x64"))
    }

    #[cfg(all(
        any(target_os = "linux", target_os = "android"),
        target_arch = "aarch64"
    ))]
    {
        Some(("linux", "aarch64"))
    }

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        Some(("mac", "x64"))
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        Some(("mac", "aarch64"))
    }

    #[cfg(not(any(
        all(target_os = "windows", target_arch = "x86_64"),
        all(
            any(target_os = "linux", target_os = "android"),
            any(target_arch = "x86_64", target_arch = "aarch64")
        ),
        all(
            target_os = "macos",
            any(target_arch = "x86_64", target_arch = "aarch64")
        ),
    )))]
    {
        None
    }
}

pub(crate) fn install_java(
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

    let java_bin = find_binary(&tmp_dir, "java")
        .ok_or_else(|| anyhow::anyhow!("Could not find java binary in archive"))?;
    let jdk_root = java_bin
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| anyhow::anyhow!("Invalid JDK directory structure"))?;

    fs::create_dir_all(vdir)?;
    copy_recursive(jdk_root, vdir)?;

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

pub(crate) static PKG: Pkg = Pkg {
    name: "java",
    termux: Some("openjdk-17"),
    description_key: "ntpkg.src_java",
    versions: fetch_java_versions,
    source: java_source,
    install: install_java,
};
