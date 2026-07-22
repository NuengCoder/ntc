use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(target_os = "macos")]
use super::super::download::{extract_archive, find_binary};
use super::super::download::download_with_progress;
use super::super::types::PackageSource;
use super::super::manager::{load_manifest, platform_key, save_manifest};
use crate::output::{print_info, print_success};

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
        let tmp_dir = std::env::temp_dir().join("ntc-install-cloudflared");
        if tmp_dir.exists() {
            let _ = fs::remove_dir_all(&tmp_dir);
        }
        fs::create_dir_all(&tmp_dir)?;

        let archive_path = tmp_dir.join("cloudflared.tgz");
        print_info(&format!("Downloading cloudflared {}...", version));
        download_with_progress(url, &archive_path)?;

        print_info("Extracting...");
        extract_archive(&archive_path, &tmp_dir, super::super::types::ArchiveType::TarGz)?;

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
    fs::write(vdir.join(".real_path"), bin_dir.to_string_lossy().as_bytes())?;

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}
