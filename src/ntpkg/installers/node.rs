use anyhow::Result;
use std::fs;
use std::path::Path;

use super::super::download::{download_with_progress, extract_archive, find_binary};
use super::super::types::{bin_dir, ArchiveType, PackageSource};
use super::super::manager::{copy_recursive, load_manifest, platform_key, save_manifest};
use crate::output::{print_info, print_success};

pub(crate) fn install_prebuilt_node(
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

    print_info("Extracting...");
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let node_bin = find_binary(&tmp_dir, "node")
        .ok_or_else(|| anyhow::anyhow!("Could not find node binary in archive"))?;
    let extracted_bin = node_bin
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
