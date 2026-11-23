//! cloudflare — version listing, download source and installer.

use super::super::download::download_with_progress;
#[cfg(target_os = "macos")]
use super::super::download::extract_archive;
#[cfg(target_os = "macos")]
use super::super::download::find_binary;
use super::super::manager::load_manifest;
use super::super::manager::save_manifest;
use super::super::types::ArchiveType;
use super::super::types::PackageSource;
use super::common::platform_key;
use super::Pkg;
use crate::output::print_info;
use crate::output::print_success;
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn cloudflare_source(version: &str) -> Option<PackageSource> {
    let base = "https://github.com/cloudflare/cloudflared/releases/latest/download";

    let (platform_key, filename, archive_type) = platform_cloudflare_archive()?;

    let url = format!("{base}/{filename}");
    let mut urls = std::collections::HashMap::new();
    urls.insert(platform_key.to_string(), url);

    Some(PackageSource {
        name: "cloudflare".to_string(),
        version: version.to_string(),
        urls,
        sha256: std::collections::HashMap::new(),
        archive_type,
    })
}

fn platform_cloudflare_archive() -> Option<(&'static str, String, ArchiveType)> {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        Some((
            "windows-x86_64",
            "cloudflared-windows-amd64.exe".to_string(),
            ArchiveType::Zip,
        ))
    }

    #[cfg(all(
        any(target_os = "linux", target_os = "android"),
        target_arch = "x86_64"
    ))]
    {
        Some((
            "linux-x86_64",
            "cloudflared-linux-amd64".to_string(),
            ArchiveType::TarGz,
        ))
    }

    #[cfg(all(
        any(target_os = "linux", target_os = "android"),
        target_arch = "aarch64"
    ))]
    {
        Some((
            "linux-aarch64",
            "cloudflared-linux-arm64".to_string(),
            ArchiveType::TarGz,
        ))
    }

    #[cfg(all(
        target_os = "macos",
        any(target_arch = "x86_64", target_arch = "aarch64")
    ))]
    {
        Some((
            "macos-universal",
            "cloudflared-darwin-amd64.tgz".to_string(),
            ArchiveType::TarGz,
        ))
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

fn latest_versions() -> Result<Vec<String>> {
    Ok(vec!["latest".to_string()])
}

pub(crate) fn install_cloudflared(
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

    let bin_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("bin");
    fs::create_dir_all(&bin_dir)?;

    #[cfg(windows)]
    {
        let target = bin_dir.join("cloudflared.exe");
        print_info(&format!("Downloading cloudflared {}...", version));
        download_with_progress(url, &target)?;

        print_success(&format!("cloudflared installed at {}", target.display()));
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        let target = bin_dir.join("cloudflared");
        print_info(&format!("Downloading cloudflared {}...", version));
        download_with_progress(url, &target)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&target, fs::Permissions::from_mode(0o755));
        }
        print_success(&format!("cloudflared installed at {}", target.display()));
    }

    #[cfg(target_os = "macos")]
    {
        use crate::t;
        let tmp_dir = std::env::temp_dir().join("ntc-install-cloudflared");
        if tmp_dir.exists() {
            let _ = fs::remove_dir_all(&tmp_dir);
        }
        fs::create_dir_all(&tmp_dir)?;

        let archive_path = tmp_dir.join("cloudflared.tgz");
        print_info(&format!("Downloading cloudflared {}...", version));
        download_with_progress(url, &archive_path)?;

        print_info(&t!("ntpkg.extracting"));
        extract_archive(
            &archive_path,
            &tmp_dir,
            super::super::types::ArchiveType::TarGz,
        )?;

        let cf_bin = find_binary(&tmp_dir, "cloudflared")
            .ok_or_else(|| anyhow::anyhow!("Could not find cloudflared binary in archive"))?;

        let target = bin_dir.join("cloudflared");
        fs::copy(&cf_bin, &target)?;
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&target, fs::Permissions::from_mode(0o755));

        let _ = fs::remove_dir_all(&tmp_dir);
        print_success(&format!("cloudflared installed at {}", target.display()));
    }

    fs::create_dir_all(vdir)?;
    fs::write(
        vdir.join(".real_path"),
        bin_dir.to_string_lossy().as_bytes(),
    )?;

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

pub(crate) static PKG: Pkg = Pkg {
    name: "cloudflare",
    termux: Some("cloudflared"),
    description_key: "ntpkg.src_cloudflare",
    versions: latest_versions,
    source: cloudflare_source,
    install: install_cloudflared,
};
