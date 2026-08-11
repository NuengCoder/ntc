use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

#[cfg(windows)]
use super::super::download::{download_with_progress, extract_archive, find_binary};
use super::super::manager::find_system_binary;
#[cfg(windows)]
use super::super::manager::{copy_recursive, platform_key};
use super::super::manager::{load_manifest, save_manifest};
#[cfg(windows)]
use super::super::types::bin_dir;
use super::super::types::PackageSource;
use crate::output::{print_info, print_success};
use crate::t;
use crate::utils::interactive::prompt_yes_no;

pub(crate) fn install_git(
    name: &str,
    version: &str,
    #[allow(unused_variables)] source: &PackageSource,
    vdir: &Path,
) -> Result<()> {
    if let Some(git_path) = find_system_binary("git") {
        let system_ver = get_git_version(&git_path)?;

        if !prompt_yes_no(
            &format!(
                "Git {system_ver} is already installed at {}. Register with ntpkg?",
                git_path.parent().unwrap().display()
            ),
            true,
        )? {
            print_info(&t!("ntpkg.skip_register", "git"));
            return Ok(());
        }

        fs::create_dir_all(vdir)?;
        if let Some(parent) = git_path.parent() {
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

    #[cfg(windows)]
    {
        print_info(&t!("ntpkg.git_not_found"));
        return install_mingit(name, version, source, vdir);
    }

    #[cfg(not(windows))]
    {
        bail!(
            "Git is not installed. Please install it via your system package manager:\n  \
             Debian/Ubuntu:  sudo apt install git\n  \
             Fedora:         sudo dnf install git\n  \
             Arch:           sudo pacman -S git\n  \
             macOS:          brew install git\n\n  \
             Then run: ntpkg install git {version}"
        );
    }
}

pub(crate) fn get_git_version(git_path: &Path) -> Result<String> {
    let output = Command::new(git_path)
        .arg("--version")
        .output()
        .context("Failed to run git --version")?;

    if !output.status.success() {
        bail!(
            "git --version failed (exit code {:?})",
            output.status.code()
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let version_str = stdout.trim();

    if version_str.starts_with("git version ") {
        let ver = version_str.trim_start_matches("git version ").trim();
        let clean_ver = if let Some(pos) = ver.find(".windows.") {
            &ver[..pos]
        } else {
            ver
        };
        return Ok(clean_ver.to_string());
    }

    bail!("Unrecognized git --version output: {version_str}")
}

#[cfg(windows)]
pub(crate) fn install_mingit(
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

    let tmp_dir = std::env::temp_dir().join(format!("ntc-install-git-{version}"));
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    let archive_name = format!("MinGit-{version}-64-bit.zip");
    let archive_path = tmp_dir.join(&archive_name);

    print_info(&format!("Downloading Git {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let git_bin = find_binary(&tmp_dir, "git")
        .ok_or_else(|| anyhow::anyhow!("Could not find git.exe in MinGit archive"))?;
    let extracted_bin = git_bin
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid binary path"))?;

    let target = bin_dir(name, version);
    fs::create_dir_all(&target)?;
    copy_recursive(extracted_bin, &target)?;

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("Git {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}
