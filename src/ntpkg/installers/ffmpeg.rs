use anyhow::Result;
use std::fs;
use std::path::Path;

use super::super::download::{download_with_progress, extract_archive, find_binary};
use super::super::manager::{copy_recursive, load_manifest, platform_key, save_manifest};
use super::super::types::PackageSource;
use crate::output::{print_info, print_success};
use crate::t;

pub(crate) fn install_ffmpeg(
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

    let ext = match source.archive_type {
        super::super::types::ArchiveType::Zip => "zip",
        _ => "tar.xz",
    };
    let archive_name = format!("{name}-{version}.{ext}");
    let archive_path = tmp_dir.join(&archive_name);

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let ffmpeg_bin = find_binary(&tmp_dir, "ffmpeg")
        .ok_or_else(|| anyhow::anyhow!("Could not find ffmpeg binary in archive"))?;
    let bin_dir = ffmpeg_bin
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid binary path"))?;

    fs::create_dir_all(vdir)?;
    copy_recursive(bin_dir, vdir)?;

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}
