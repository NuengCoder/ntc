//! gh — version listing, download source and installer.

use super::super::download::download_with_progress;
use super::super::download::extract_archive;
use super::super::download::find_binary;
use super::super::manager::load_manifest;
use super::super::manager::save_manifest;
use super::super::types::bin_dir;
use super::super::types::ArchiveType;
use super::super::types::PackageSource;
use super::common::copy_recursive;
use super::common::find_system_binary;
use super::common::platform_key;
use super::Pkg;
use crate::output::print_info;
use crate::output::print_success;
use crate::t;
use crate::utils::interactive::prompt_yes_no;
use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;

fn fetch_gh_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/cli/cli/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch gh versions from GitHub")?;

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
        if is_gh_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
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

fn is_gh_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn gh_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://github.com/cli/cli/releases/download/v{version}");

    let mut urls = HashMap::new();
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{base}/gh_{version}_linux_amd64.tar.gz"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{base}/gh_{version}_linux_arm64.tar.gz"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{base}/gh_{version}_macOS_amd64.zip"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{base}/gh_{version}_macOS_arm64.zip"),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("{base}/gh_{version}_windows_amd64.zip"),
    );

    Some(PackageSource {
        name: "gh".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::Zip,
    })
}

pub(crate) fn install_gh(
    name: &str,
    version: &str,
    source: &PackageSource,
    vdir: &Path,
) -> Result<()> {
    if let Some(gh_path) = find_system_binary("gh") {
        let system_ver = get_gh_version(&gh_path)?;

        if !prompt_yes_no(
            &format!(
                "GitHub CLI {system_ver} is already installed at {}. Register with ntpkg?",
                gh_path.parent().unwrap().display()
            ),
            true,
        )? {
            print_info(&t!("ntpkg.skip_register", "gh"));
            return Ok(());
        }

        fs::create_dir_all(vdir)?;
        if let Some(parent) = gh_path.parent() {
            fs::write(vdir.join(".real_path"), parent.to_string_lossy().as_bytes())
                .context("Failed to write .real_path")?;
        }

        let mut manifest = load_manifest()?;
        manifest.add_version(name, &system_ver);
        manifest.activate(name, &system_ver);
        save_manifest(&manifest)?;

        print_success(&format!(
            "{} {} registered (system install)",
            name, system_ver
        ));
        return Ok(());
    }

    print_info(&format!("Downloading GitHub CLI {version}..."));
    install_prebuilt_gh(name, version, source, vdir)
}

pub(crate) fn get_gh_version(gh_path: &Path) -> Result<String> {
    let output = Command::new(gh_path)
        .arg("--version")
        .output()
        .context("Failed to run gh --version")?;

    if !output.status.success() {
        bail!("gh --version failed (exit code {:?})", output.status.code());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().next().unwrap_or("").trim();

    if let Some(ver) = line.strip_prefix("gh version ") {
        let clean = ver.split_whitespace().next().unwrap_or(ver);
        return Ok(clean.to_string());
    }

    bail!("Unrecognized gh --version output: {line}")
}

pub(crate) fn install_prebuilt_gh(
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
    let archive_name = format!("gh_{version}_{platform_key}.{ext}");
    let archive_path = std::env::temp_dir()
        .join(format!("ntc-install-gh-{version}"))
        .join(&archive_name);

    let tmp_dir = archive_path.parent().unwrap().to_path_buf();
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let gh_bin = find_binary(&tmp_dir, "gh")
        .ok_or_else(|| anyhow::anyhow!("Could not find gh binary in archive"))?;
    let extracted_bin = gh_bin
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
    name: "gh",
    termux: Some("gh"),
    description_key: "ntpkg.src_gh",
    versions: fetch_gh_versions,
    source: gh_source,
    install: install_gh,
};
