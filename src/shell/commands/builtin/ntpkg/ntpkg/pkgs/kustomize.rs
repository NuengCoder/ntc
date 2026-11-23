//! kustomize — version listing, download source and installer.

use super::super::download::download_with_progress;
use super::super::download::extract_archive;
use super::super::download::find_binary;
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

fn fetch_kustomize_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/kubernetes-sigs/kustomize/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch kustomize versions from GitHub")?;

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
        if let Some(stripped) = tag.strip_prefix("kustomize/v") {
            if is_kustomize_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
                versions.push(stripped.to_string());
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

fn is_kustomize_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn kustomize_source(version: &str) -> Option<PackageSource> {
    let base = format!(
        "https://github.com/kubernetes-sigs/kustomize/releases/download/kustomize/v{version}"
    );

    let mut urls = HashMap::new();
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{base}/kustomize_v{version}_linux_amd64.tar.gz"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{base}/kustomize_v{version}_linux_arm64.tar.gz"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{base}/kustomize_v{version}_darwin_amd64.tar.gz"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{base}/kustomize_v{version}_darwin_arm64.tar.gz"),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("{base}/kustomize_v{version}_windows_amd64.zip"),
    );

    Some(PackageSource {
        name: "kustomize".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::Zip,
    })
}

pub(crate) fn install_kustomize(
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

    let fname = url.rsplit_once('/').map(|(_, f)| f).unwrap_or("archive");
    let archive_path = tmp_dir.join(fname);

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    let archive_type = if url.ends_with(".zip") {
        ArchiveType::Zip
    } else {
        ArchiveType::TarGz
    };

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, archive_type)?;

    let kustomize_bin = find_binary(&tmp_dir, "kustomize")
        .ok_or_else(|| anyhow::anyhow!("Could not find kustomize binary in archive"))?;

    let target_dir = bin_dir(name, version);
    fs::create_dir_all(&target_dir)?;

    let dest = target_dir.join(if cfg!(windows) {
        "kustomize.exe"
    } else {
        "kustomize"
    });
    fs::copy(&kustomize_bin, &dest)
        .with_context(|| format!("Failed to copy kustomize binary to {}", dest.display()))?;

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

pub(crate) static PKG: Pkg = Pkg {
    name: "kustomize",
    termux: Some("kustomize"),
    description_key: "ntpkg.src_kustomize",
    versions: fetch_kustomize_versions,
    source: kustomize_source,
    install: install_kustomize,
};
