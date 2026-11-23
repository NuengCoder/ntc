//! scala — version listing, download source and installer.

use super::super::download::download_with_progress;
use super::super::download::extract_archive;
use super::super::manager::load_manifest;
use super::super::manager::save_manifest;
use super::super::types::ArchiveType;
use super::super::types::PackageSource;
use super::common::copy_recursive;
use super::common::fetch_github_releases;
use super::common::platform_key;
use super::Pkg;
use crate::output::print_info;
use crate::output::print_success;
use crate::t;
use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn fetch_scala_versions() -> Result<Vec<String>> {
    let mut versions: Vec<String> = Vec::new();

    // Fetch Scala 2 from scala/scala (tags v2.x.x)
    if let Ok(v2) = fetch_github_releases("scala", "scala") {
        for v in v2 {
            if v.starts_with('2') && is_scala_valid_version(&v) {
                if !versions.contains(&v) {
                    versions.push(v);
                }
            }
        }
    }

    // Fetch Scala 3 from scala/scala3 (tags 3.x.x)
    if let Ok(v3) = fetch_github_releases("scala", "scala3") {
        for v in v3 {
            if v.starts_with('3') && is_scala3_valid_version(&v) {
                if !versions.contains(&v) {
                    versions.push(v);
                }
            }
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse().ok())
            .collect();
        let b_parts: Vec<u32> = b
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse().ok())
            .collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn is_scala_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn is_scala3_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() < 2 || parts.len() > 4 {
        return false;
    }
    // Must start with digit
    if !parts[0].chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    // Allow suffixes like -RC1, -M1 in the last part
    let last = parts.last().copied().unwrap_or("");
    let base = last.split('-').next().unwrap_or("");
    if !base.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    true
}

fn scala_source(version: &str) -> Option<PackageSource> {
    let is_scala2 = version.starts_with('2');
    let (ext, archive_type) = if cfg!(windows) {
        ("zip", ArchiveType::Zip)
    } else if is_scala2 {
        ("tgz", ArchiveType::TarGz)
    } else {
        ("tar.gz", ArchiveType::TarGz)
    };

    let (repo, tag_prefix, archive_prefix) = if is_scala2 {
        ("scala/scala", "v", "scala")
    } else {
        ("scala/scala3", "", "scala3")
    };

    let url = format!(
        "https://github.com/{repo}/releases/download/{tag_prefix}{version}/{archive_prefix}-{version}.{ext}"
    );

    let mut urls = HashMap::new();
    urls.insert("linux-x86_64".to_string(), url.clone());
    urls.insert("linux-aarch64".to_string(), url.clone());
    urls.insert("macos-x86_64".to_string(), url.clone());
    urls.insert("macos-aarch64".to_string(), url.clone());
    urls.insert("windows-x86_64".to_string(), url.clone());
    urls.insert("windows-aarch64".to_string(), url);

    Some(PackageSource {
        name: "scala".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

pub(crate) fn install_scala(
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

    let archive_name = url.split('/').last().unwrap_or("scala-archive");
    let archive_name = if archive_name.is_empty() {
        "scala-archive"
    } else {
        archive_name
    };

    let tmp_dir = std::env::temp_dir().join(format!("ntc-install-{name}-{version}"));
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    let archive_path = tmp_dir.join(archive_name);
    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let scala_bin = find_scala_binary(&tmp_dir)
        .ok_or_else(|| anyhow::anyhow!("Could not find scala binary in archive"))?;

    let scala_root = scala_bin
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| anyhow::anyhow!("Invalid Scala directory structure"))?;

    fs::create_dir_all(vdir)?;
    copy_recursive(scala_root, vdir)?;

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

fn find_scala_binary(dir: &Path) -> Option<std::path::PathBuf> {
    let target = if cfg!(windows) { "scala.bat" } else { "scala" };
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
    name: "scala",
    termux: Some("scala"),
    description_key: "ntpkg.src_scala",
    versions: fetch_scala_versions,
    source: scala_source,
    install: install_scala,
};
