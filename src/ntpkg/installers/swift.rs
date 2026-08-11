use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::super::download::{download_with_progress, extract_archive, find_binary};
use super::super::manager::{copy_recursive, load_manifest, platform_key, save_manifest};
use super::super::types::{ArchiveType, PackageSource};
use crate::output::{print_error, print_info, print_success};
use crate::t;

pub(crate) fn install_swift(
    name: &str,
    version: &str,
    source: &PackageSource,
    vdir: &Path,
) -> Result<()> {
    let platform_key = platform_key();
    let url = source
        .urls
        .get(&platform_key)
        .ok_or_else(|| anyhow::anyhow!("Swift is not available for platform '{platform_key}'"))?;

    let tmp_dir = std::env::temp_dir().join(format!("ntc-install-{name}-{version}"));
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    if url.ends_with(".tar.gz") {
        install_swift_linux(name, version, url, &tmp_dir, vdir)
    } else if url.ends_with(".pkg") {
        install_swift_macos(name, version, url, &tmp_dir, vdir)
    } else if url.ends_with(".exe") {
        install_swift_windows(name, version, url, &tmp_dir, vdir)
    } else {
        bail!("Unknown Swift archive format for URL: {url}")
    }
}

// ── Linux: download + extract tar.gz ─────────────────────────────────

fn install_swift_linux(
    name: &str,
    version: &str,
    url: &str,
    tmp_dir: &Path,
    vdir: &Path,
) -> Result<()> {
    let archive_name = format!("swift-{version}.tar.gz");
    let archive_path = tmp_dir.join(&archive_name);

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, tmp_dir, ArchiveType::TarGz)?;

    let swift_bin = find_binary(tmp_dir, "swift")
        .ok_or_else(|| anyhow::anyhow!("Could not find swift binary in archive"))?;

    // swift-{version}-RELEASE-ubuntu{...}/usr/bin/swift -> usr/
    let usr_dir = swift_bin
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| anyhow::anyhow!("Invalid Swift directory structure"))?;

    let target = vdir.join("usr");
    fs::create_dir_all(vdir)?;
    copy_recursive(usr_dir, &target)?;

    let bin_path = vdir.join("usr").join("bin");
    fs::write(
        vdir.join(".real_path"),
        bin_path.to_string_lossy().as_bytes(),
    )
    .context("Failed to write .real_path")?;

    let _ = fs::remove_dir_all(tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

// ── macOS: download + launch .pkg installer ──────────────────────────

fn install_swift_macos(
    name: &str,
    version: &str,
    url: &str,
    tmp_dir: &Path,
    vdir: &Path,
) -> Result<()> {
    let installer_path = tmp_dir.join(format!("swift-{version}.pkg"));

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &installer_path)?;

    print_info(&t!("ntpkg.launch_installer"));
    let status = Command::new("sudo")
        .args(["installer", "-pkg"])
        .arg(&installer_path)
        .args(["-target", "/"])
        .status()
        .context("Failed to launch Swift installer")?;

    if !status.success() {
        print_error(&format!(
            "Swift installer exited with code {:?}",
            status.code()
        ));
        bail!("Swift .pkg installation failed");
    }

    let toolchain_bin = find_macos_toolchain(version)?;

    fs::create_dir_all(vdir)?;
    fs::write(
        vdir.join(".real_path"),
        toolchain_bin.to_string_lossy().as_bytes(),
    )
    .context("Failed to write .real_path")?;

    let _ = fs::remove_dir_all(tmp_dir);

    print_success(&format!(
        "{name} {version} installed at {}",
        toolchain_bin.display()
    ));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

fn find_macos_toolchain(version: &str) -> Result<PathBuf> {
    let toolchains_dir = PathBuf::from("/Library/Developer/Toolchains");
    if !toolchains_dir.exists() {
        bail!("Swift toolchains directory not found at /Library/Developer/Toolchains");
    }

    let expected = toolchains_dir
        .join(format!("swift-{version}.xctoolchain"))
        .join("usr")
        .join("bin");
    if expected.exists() {
        return Ok(expected);
    }

    // Fallback: find the latest toolchain directory
    if let Ok(entries) = fs::read_dir(&toolchains_dir) {
        let mut candidates: Vec<PathBuf> = entries
            .flatten()
            .filter(|e| e.path().is_dir())
            .map(|e| e.path())
            .collect();
        candidates.sort();
        if let Some(latest) = candidates.last() {
            let bin = latest.join("usr").join("bin");
            if bin.exists() {
                return Ok(bin);
            }
        }
    }

    bail!(
        "Could not find Swift toolchain after installation.\n\
         Expected at: {}",
        expected.display()
    )
}

// ── Windows: download + launch .exe installer ────────────────────────

fn install_swift_windows(
    name: &str,
    version: &str,
    url: &str,
    tmp_dir: &Path,
    vdir: &Path,
) -> Result<()> {
    let installer_path = tmp_dir.join(format!("swift-{version}.exe"));

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &installer_path)?;

    print_info(&t!("ntpkg.launch_installer"));
    let script = format!(
        "Start-Process -FilePath '{}' -Wait -Verb RunAs",
        installer_path.to_string_lossy().replace('\'', "''")
    );
    let status = Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .status()
        .context("Failed to launch Swift installer")?;

    if let Some(code) = status.code() {
        if code != 0 && code != 1 {
            print_error(&format!("Swift installer exited with code {code:?}"));
        }
    }

    let swift_bin = find_windows_swift()?;

    fs::create_dir_all(vdir)?;
    fs::write(
        vdir.join(".real_path"),
        swift_bin.to_string_lossy().as_bytes(),
    )
    .context("Failed to write .real_path")?;

    let _ = fs::remove_dir_all(tmp_dir);

    print_success(&format!(
        "{name} {version} installed at {}",
        swift_bin.display()
    ));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

fn find_windows_swift() -> Result<PathBuf> {
    let search_dirs: Vec<PathBuf> = {
        let mut dirs = Vec::new();
        if let Ok(pf) = std::env::var("ProgramFiles") {
            dirs.push(PathBuf::from(pf).join("Swift"));
        }
        if let Ok(pfx86) = std::env::var("ProgramFiles(x86)") {
            dirs.push(PathBuf::from(pfx86).join("Swift"));
        }
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            dirs.push(PathBuf::from(local).join("Swift"));
        }
        dirs.push(PathBuf::from(r"C:\Program Files\Swift"));
        dirs.push(PathBuf::from(r"C:\Program Files (x86)\Swift"));
        dirs
    };

    for base in &search_dirs {
        let candidate = base.join("usr").join("bin").join("swift.exe");
        if candidate.exists() {
            return Ok(base.join("usr").join("bin"));
        }
    }

    for base in &search_dirs {
        if base.exists() {
            if let Ok(entries) = fs::read_dir(base) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let candidate = path.join("usr").join("bin").join("swift.exe");
                    if candidate.exists() {
                        return Ok(path.join("usr").join("bin"));
                    }
                }
            }
        }
    }

    bail!(
        "Could not find swift.exe after installation.\n\
         Searched in:\n  {}",
        search_dirs
            .iter()
            .map(|d| format!("  {}", d.display()))
            .collect::<Vec<_>>()
            .join("\n")
    )
}
