use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

use super::super::download::{download_with_progress, extract_archive, find_binary};
use super::super::manager::{
    copy_recursive, find_system_binary, load_manifest, platform_key, save_manifest,
};
use super::super::types::{bin_dir, ArchiveType, PackageSource};
use crate::output::{print_info, print_success};
use crate::t;
use crate::utils::interactive::prompt_yes_no;

pub(crate) fn install_gh(
    name: &str,
    version: &str,
    source: &PackageSource,
    vdir: &Path,
) -> Result<()> {
    if let Some(gh_path) = find_system_binary("gh") {
        let system_ver = get_gh_version(&gh_path)?;

        if !prompt_yes_no(
            &format!(
                "GitHub CLI {system_ver} is already installed at {}. Register with ntpkg?",
                gh_path.parent().unwrap().display()
            ),
            true,
        )? {
            print_info(&t!("ntpkg.skip_register", "gh"));
            return Ok(());
        }

        fs::create_dir_all(vdir)?;
        if let Some(parent) = gh_path.parent() {
            fs::write(vdir.join(".real_path"), parent.to_string_lossy().as_bytes())
                .context("Failed to write .real_path")?;
        }

        let mut manifest = load_manifest()?;
        manifest.add_version(name, &system_ver);
        manifest.activate(name, &system_ver);
        save_manifest(&manifest)?;

        print_success(&format!(
            "{} {} registered (system install)",
            name, system_ver
        ));
        return Ok(());
    }

    print_info(&format!("Downloading GitHub CLI {version}..."));
    install_prebuilt_gh(name, version, source, vdir)
}

pub(crate) fn get_gh_version(gh_path: &Path) -> Result<String> {
    let output = Command::new(gh_path)
        .arg("--version")
        .output()
        .context("Failed to run gh --version")?;

    if !output.status.success() {
        bail!("gh --version failed (exit code {:?})", output.status.code());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().next().unwrap_or("").trim();

    if let Some(ver) = line.strip_prefix("gh version ") {
        let clean = ver.split_whitespace().next().unwrap_or(ver);
        return Ok(clean.to_string());
    }

    bail!("Unrecognized gh --version output: {line}")
}

pub(crate) fn install_prebuilt_gh(
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
    let archive_name = format!("gh_{version}_{platform_key}.{ext}");
    let archive_path = std::env::temp_dir()
        .join(format!("ntc-install-gh-{version}"))
        .join(&archive_name);

    let tmp_dir = archive_path.parent().unwrap().to_path_buf();
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let gh_bin = find_binary(&tmp_dir, "gh")
        .ok_or_else(|| anyhow::anyhow!("Could not find gh binary in archive"))?;
    let extracted_bin = gh_bin
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
