//! kubectl — version listing, download source and installer.

use super::super::download::download_with_progress;
use super::super::manager::load_manifest;
use super::super::manager::save_manifest;
use super::super::types::ArchiveType;
use super::super::types::PackageSource;
use super::common::platform_key;
use super::Pkg;
use crate::output::print_info;
use crate::output::print_success;
use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;

fn fetch_kubectl_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/kubernetes/kubernetes/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch kubectl versions from GitHub")?;

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
        if is_kubectl_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
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

fn is_kubectl_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    // kubernetes tags look like "1.32.0" (3 parts) or "1.32.0-alpha.1" etc
    // we only take stable 3-part semver
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn kubectl_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://dl.k8s.io/release/v{version}");

    let mut urls = HashMap::new();
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{base}/bin/linux/amd64/kubectl"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{base}/bin/linux/arm64/kubectl"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{base}/bin/darwin/amd64/kubectl"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{base}/bin/darwin/arm64/kubectl"),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("{base}/bin/windows/amd64/kubectl.exe"),
    );

    Some(PackageSource {
        name: "kubectl".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::Zip,
    })
}

pub(crate) fn install_kubectl(
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

    let bin_dir = vdir.join("bin");
    fs::create_dir_all(&bin_dir)?;

    let binary_name = if cfg!(windows) {
        "kubectl.exe"
    } else {
        "kubectl"
    };
    let target = bin_dir.join(binary_name);

    print_info(&format!("Downloading kubectl {}...", version));
    download_with_progress(url, &target)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&target, fs::Permissions::from_mode(0o755))?;
    }

    fs::write(
        vdir.join(".real_path"),
        bin_dir.to_string_lossy().as_bytes(),
    )?;

    print_success(&format!(
        "kubectl {} installed at {}",
        version,
        target.display()
    ));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

pub(crate) static PKG: Pkg = Pkg {
    name: "kubectl",
    termux: Some("kubectl"),
    description_key: "ntpkg.src_kubectl",
    versions: fetch_kubectl_versions,
    source: kubectl_source,
    install: install_kubectl,
};
