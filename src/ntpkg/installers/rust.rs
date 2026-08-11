use anyhow::Result;
use std::fs;
use std::path::Path;

use super::super::download::{download_with_progress, extract_archive, find_binary};
use super::super::manager::{copy_recursive, load_manifest, platform_key, save_manifest};
use super::super::types::{bin_dir, PackageSource};
use crate::output::{print_info, print_success};
use crate::t;

pub(crate) fn install_rust(
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

    let archive_name = format!("{name}-{version}.tar.gz");
    let archive_path = tmp_dir.join(&archive_name);
    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let rustc_bin = find_binary(&tmp_dir, "rustc")
        .ok_or_else(|| anyhow::anyhow!("Could not find rustc binary in archive"))?;
    let cargo_bin = find_binary(&tmp_dir, "cargo")
        .ok_or_else(|| anyhow::anyhow!("Could not find cargo binary in archive"))?;

    let rustc_bin_dir = rustc_bin
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid rustc binary path"))?;
    let cargo_bin_dir = cargo_bin
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid cargo binary path"))?;

    let target = bin_dir(name, version);
    fs::create_dir_all(&target)?;
    copy_recursive(rustc_bin_dir, &target)?;
    copy_recursive(cargo_bin_dir, &target)?;

    fs::write(vdir.join(".real_path"), target.to_string_lossy().as_bytes()).ok();

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}
