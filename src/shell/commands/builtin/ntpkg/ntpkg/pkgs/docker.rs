//! docker — version listing, download source and installer.

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

fn fetch_docker_versions() -> Result<Vec<String>> {
    let url = "https://download.docker.com/linux/static/stable/x86_64/";
    let response = ureq::get(url)
        .call()
        .context("Failed to fetch Docker versions from download.docker.com")?;

    let mut body = response.into_body();
    let mut html = String::new();
    body.as_reader()
        .read_to_string(&mut html)
        .context("Failed to read download.docker.com response body")?;

    let mut versions: Vec<String> = Vec::new();
    let mut search_start = 0;

    while let Some(href_start) = html[search_start..].find("<a href=\"") {
        let start = search_start + href_start + 9;
        if let Some(quote_end) = html[start..].find('"') {
            let url_target = &html[start..start + quote_end];

            if let Some(rest) = url_target.strip_prefix("docker-") {
                if let Some(ver) = rest.strip_suffix(".tgz") {
                    if is_docker_valid_version(ver) && !versions.iter().any(|v| v == ver) {
                        versions.push(ver.to_string());
                    }
                }
            }

            search_start = start + quote_end + 1;
        } else {
            break;
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn is_docker_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    // Accept semver like 27.0.0 or 24.0.8
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn docker_source(version: &str) -> Option<PackageSource> {
    let mut urls = HashMap::new();

    urls.insert(
        "linux-x86_64".to_string(),
        format!("https://download.docker.com/linux/static/stable/x86_64/docker-{version}.tgz"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("https://download.docker.com/linux/static/stable/aarch64/docker-{version}.tgz"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("https://download.docker.com/mac/static/stable/x86_64/docker-{version}.tgz"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("https://download.docker.com/mac/static/stable/aarch64/docker-{version}.tgz"),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("https://download.docker.com/win/static/stable/x86_64/docker-{version}.zip"),
    );

    Some(PackageSource {
        name: "docker".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::TarGz,
    })
}

pub(crate) fn install_docker(
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

    let docker_bin = find_binary(&tmp_dir, "docker")
        .ok_or_else(|| anyhow::anyhow!("Could not find docker binary in archive"))?;

    let target_dir = bin_dir(name, version);
    fs::create_dir_all(&target_dir)?;

    let dest = target_dir.join(if cfg!(windows) {
        "docker.exe"
    } else {
        "docker"
    });
    fs::copy(&docker_bin, &dest)
        .with_context(|| format!("Failed to copy docker binary to {}", dest.display()))?;

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
    name: "docker",
    termux: Some("docker-cli"),
    description_key: "ntpkg.src_docker",
    versions: fetch_docker_versions,
    source: docker_source,
    install: install_docker,
};
