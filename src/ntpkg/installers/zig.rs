use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::super::download::{download_with_progress, extract_archive};
use super::super::manager::{copy_recursive, load_manifest, platform_key, save_manifest};
use super::super::types::{ArchiveType, PackageSource};
use crate::output::{print_info, print_success};
use crate::t;

pub(crate) fn install_zig(
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

    let archive_type = if url.ends_with(".zip") {
        ArchiveType::Zip
    } else {
        ArchiveType::TarXz
    };

    let fname = url.rsplit_once('/').map(|(_, f)| f).unwrap_or("archive");
    let archive_path = tmp_dir.join(fname);
    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, archive_type)?;

    let zig_bin = find_zig_binary(&tmp_dir)
        .ok_or_else(|| anyhow::anyhow!("Could not find zig binary in archive"))?;
    let zig_root = zig_bin
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid Zig directory structure"))?;

    fs::create_dir_all(vdir)?;
    copy_recursive(zig_root, vdir)?;

    fs::write(vdir.join(".real_path"), vdir.to_string_lossy().as_bytes())
        .context("Failed to write .real_path")?;

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

fn find_zig_binary(dir: &Path) -> Option<std::path::PathBuf> {
    let target = if cfg!(windows) { "zig.exe" } else { "zig" };
    find_file_recursive(dir, target)
}

fn find_file_recursive(dir: &Path, name: &str) -> Option<std::path::PathBuf> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(found) = find_file_recursive(&path, name) {
                    return Some(found);
                }
            } else if path.file_name().and_then(|n| n.to_str()) == Some(name) {
                return Some(path);
            }
        }
    }
    None
}
