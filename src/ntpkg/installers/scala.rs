use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::super::download::{download_with_progress, extract_archive};
use super::super::manager::{copy_recursive, load_manifest, platform_key, save_manifest};
use super::super::types::PackageSource;
use crate::output::{print_info, print_success};

pub(crate) fn install_scala(
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

    let archive_name = url.split('/').last().unwrap_or("scala-archive");
    let archive_name = if archive_name.is_empty() { "scala-archive" } else { archive_name };

    let tmp_dir = std::env::temp_dir().join(format!("ntc-install-{name}-{version}"));
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    let archive_path = tmp_dir.join(archive_name);
    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info("Extracting...");
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let scala_bin = find_scala_binary(&tmp_dir)
        .ok_or_else(|| anyhow::anyhow!("Could not find scala binary in archive"))?;

    let scala_root = scala_bin
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| anyhow::anyhow!("Invalid Scala directory structure"))?;

    fs::create_dir_all(vdir)?;
    copy_recursive(scala_root, vdir)?;

    let bin_path = vdir.join("bin");
    fs::write(vdir.join(".real_path"), bin_path.to_string_lossy().as_bytes())
        .context("Failed to write .real_path")?;

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

fn find_scala_binary(dir: &Path) -> Option<std::path::PathBuf> {
    let target = if cfg!(windows) { "scala.bat" } else { "scala" };
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
            } else if path
                .file_name()
                .and_then(|n| n.to_str()) == Some(name)
            {
                return Some(path);
            }
        }
    }
    None
}
