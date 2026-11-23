//! ffmpeg — version listing, download source and installer.

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
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn ffmpeg_source(_version: &str) -> Option<PackageSource> {
    let mut urls = HashMap::new();
    urls.insert(
        "windows-x86_64".to_string(),
        "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip"
            .to_string(),
    );
    urls.insert(
        "linux-x86_64".to_string(),
        "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linux64-gpl.tar.xz"
            .to_string(),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linuxarm64-gpl.tar.xz"
            .to_string(),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-macos64-gpl.tar.xz"
            .to_string(),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-macosarm64-gpl.tar.xz"
            .to_string(),
    );

    let archive_type = platform_ffmpeg_archive();

    Some(PackageSource {
        name: "ffmpeg".to_string(),
        version: "latest".to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

fn platform_ffmpeg_archive() -> ArchiveType {
    #[cfg(target_os = "windows")]
    {
        ArchiveType::Zip
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        ArchiveType::TarXz
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        ArchiveType::TarXz
    }
}

fn latest_versions() -> Result<Vec<String>> {
    Ok(vec!["latest".to_string()])
}

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

pub(crate) static PKG: Pkg = Pkg {
    name: "ffmpeg",
    termux: Some("ffmpeg"),
    description_key: "ntpkg.src_ffmpeg",
    versions: latest_versions,
    source: ffmpeg_source,
    install: install_ffmpeg,
};
