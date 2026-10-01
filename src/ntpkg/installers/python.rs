use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(not(windows))]
use std::process::Command;

use super::super::download::download_with_progress;
#[cfg(not(any(windows, target_os = "macos")))]
use super::super::download::extract_archive;
#[cfg(not(windows))]
use super::super::manager::find_system_binary;
use super::super::manager::{load_manifest, platform_key, save_manifest};
use super::super::types::{ArchiveType, PackageSource};
use crate::output::{print_info, print_success};
use crate::t;
use crate::utils::interactive::prompt_yes_no;

/// Check if a matching Python version is already installed on the system.
/// Returns the path to its bin/ directory if found.
#[cfg(not(windows))]
pub(crate) fn detect_system_python(version: &str) -> Result<Option<PathBuf>> {
    let python_path = match find_system_binary("python3") {
        Some(p) => p,
        None => match find_system_binary("python") {
            Some(p) => p,
            None => return Ok(None),
        },
    };

    let output = Command::new(&python_path)
        .arg("--version")
        .output()
        .context("Failed to check system Python version")?;

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let version_str = stdout.trim();

    if version_str.starts_with("Python ") {
        let sys_ver = version_str.trim_start_matches("Python ").trim();
        if sys_ver == version {
            if let Some(parent) = python_path.parent() {
                return Ok(Some(parent.to_path_buf()));
            }
        }
    }

    Ok(None)
}

/// Download, extract, configure, make, and install Python into `prefix_dir`.
/// Only used on Linux / Termux (not Windows, not macOS).
#[cfg(not(any(windows, target_os = "macos")))]
fn build_python_from_source(
    tarball: &Path,
    prefix_dir: &Path,
    name: &str,
    version: &str,
) -> Result<()> {
    let build_dir = std::env::temp_dir().join(format!("ntc-build-python-{version}"));
    if build_dir.exists() {
        let _ = fs::remove_dir_all(&build_dir);
    }
    fs::create_dir_all(&build_dir)?;

    print_info(&t!("ntpkg.extracting_source"));
    // The Linux/macOS source tarball is .tar.xz, not .tar.gz.
    extract_archive(tarball, &build_dir, ArchiveType::TarXz)?;

    let source_dir = find_source_dir(&build_dir, version)?;

    check_build_deps()?;

    print_info(&t!("ntpkg.configuring"));
    let status = Command::new("./configure")
        .arg(format!("--prefix={}", prefix_dir.display()))
        .current_dir(&source_dir)
        .status()
        .context("Failed to run ./configure")?;

    if !status.success() {
        bail!(
            "{} ./configure failed (exit code {:?})",
            name,
            status.code()
        );
    }

    let jobs = parallel_jobs();
    print_info(&format!("Building (make -j{jobs})..."));
    let status = Command::new("make")
        .arg(format!("-j{jobs}"))
        .current_dir(&source_dir)
        .status()
        .context("Failed to run make")?;

    if !status.success() {
        bail!("make failed (exit code {:?})", status.code());
    }

    print_info(&t!("ntpkg.installing"));
    let status = Command::new("make")
        .arg("install")
        .current_dir(&source_dir)
        .status()
        .context("Failed to run make install")?;

    if !status.success() {
        bail!("make install failed (exit code {:?})", status.code());
    }

    let _ = fs::remove_dir_all(&build_dir);

    Ok(())
}

#[cfg(not(any(windows, target_os = "macos")))]
fn find_source_dir(build_dir: &Path, version: &str) -> Result<PathBuf> {
    let expected = build_dir.join(format!("Python-{version}"));
    if expected.is_dir() {
        return Ok(expected);
    }

    if let Ok(entries) = fs::read_dir(build_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.starts_with("Python-") || name.starts_with("python-") {
                    return Ok(path);
                }
            }
        }
    }

    bail!(
        "Could not find Python source directory in {}",
        build_dir.display()
    )
}

#[cfg(not(any(windows, target_os = "macos")))]
fn check_build_deps() -> Result<()> {
    let tools = ["gcc", "make", "cc"];
    for tool in &tools {
        if find_system_binary(tool).is_none() {
            let hint = if find_system_binary("pacman").is_some() {
                "sudo pacman -S base-devel"
            } else if find_system_binary("apt-get").is_some() || find_system_binary("apt").is_some()
            {
                "sudo apt install build-essential libssl-dev zlib1g-dev"
            } else if find_system_binary("dnf").is_some() {
                "sudo dnf groupinstall 'Development Tools'"
            } else if find_system_binary("pkg").is_some() {
                "pkg install build-essential"
            } else {
                "install gcc, make and related build tools"
            };

            bail!("Build tool '{tool}' not found.\n  Install build dependencies:\n    {hint}");
        }
    }
    Ok(())
}

#[cfg(not(any(windows, target_os = "macos")))]
fn parallel_jobs() -> usize {
    if let Ok(output) = Command::new("nproc").output() {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            if let Ok(n) = s.trim().parse::<usize>() {
                return n.max(1);
            }
        }
    }
    if let Ok(output) = Command::new("getconf").arg("_NPROCESSORS_ONLN").output() {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            if let Ok(n) = s.trim().parse::<usize>() {
                return n.max(1);
            }
        }
    }
    2
}

/// Build Python from source tarball on Linux / Termux (not macOS/Windows).
#[cfg(not(any(windows, target_os = "macos")))]
pub(crate) fn build_from_source(
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

    print_info(&format!("Downloading {name} {version} source..."));
    fs::create_dir_all(vdir)
        .with_context(|| format!("Failed to create {}", vdir.display()))?;
    let tarball = vdir.join(format!("Python-{version}.tar.xz"));
    download_with_progress(url, &tarball)?;

    build_python_from_source(&tarball, vdir, name, version)?;

    let _ = fs::remove_file(&tarball);
    print_success(&format!("{name} {version} built at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

/// Download and launch a platform installer (.exe or .pkg).
#[allow(dead_code)]
pub(crate) fn download_and_launch_installer(
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
        ArchiveType::Msi => ".exe",
        ArchiveType::Pkg => ".pkg",
        _ => ".exe",
    };
    // vdir does not exist yet on a fresh install — create it before the
    // download or fs::write fails with "Failed to write downloaded file".
    fs::create_dir_all(vdir)
        .with_context(|| format!("Failed to create {}", vdir.display()))?;
    let installer_path = vdir.join(format!("{name}-{version}{ext}"));

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &installer_path)?;

    print_success(&format!("Downloaded to {}", installer_path.display()));

    if prompt_yes_no(&format!("Start {name} {version} installer"), true)? {
        print_info(&t!("ntpkg.launch_installer"));
        super::launch_installer_and_wait(&installer_path, name, version)?;

        let real_dir = find_python_install_dir()?;
        print_success(&format!(
            "Detected {name} installed at {}",
            real_dir.display()
        ));

        fs::write(
            vdir.join(".real_path"),
            real_dir.to_string_lossy().as_bytes(),
        )
        .context("Failed to write .real_path")?;
    }

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

#[cfg(windows)]
#[allow(dead_code)]
fn find_python_install_dir() -> Result<PathBuf> {
    let search_dirs: Vec<PathBuf> = {
        let mut dirs = Vec::new();

        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            dirs.push(PathBuf::from(local).join("Programs").join("Python"));
        }
        if let Ok(pf) = std::env::var("ProgramFiles") {
            dirs.push(PathBuf::from(pf).join("Python"));
        }
        if let Ok(pfx86) = std::env::var("ProgramFiles(x86)") {
            dirs.push(PathBuf::from(pfx86).join("Python"));
        }
        dirs.push(PathBuf::from(r"C:\Program Files\Python"));
        dirs.push(PathBuf::from(r"C:\Program Files (x86)\Python"));

        dirs
    };

    for base in &search_dirs {
        if !base.exists() {
            continue;
        }
        if let Ok(entries) = fs::read_dir(base) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                if path.join("python.exe").exists() {
                    return Ok(path);
                }
                if let Ok(inner) = fs::read_dir(&path) {
                    for sub in inner.flatten() {
                        let sub_path = sub.path();
                        if sub_path.is_dir() && sub_path.join("python.exe").exists() {
                            return Ok(sub_path);
                        }
                    }
                }
            }
        }
    }

    bail!(
        "Could not find python.exe after installation.\n\
         Searched in:\n  {}",
        search_dirs
            .iter()
            .map(|d| format!("  {}", d.display()))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

#[cfg(not(windows))]
#[allow(dead_code)]
fn find_python_install_dir() -> Result<PathBuf> {
    let candidates = [
        PathBuf::from("/usr/local/bin"),
        PathBuf::from("/usr/bin"),
        PathBuf::from("/opt/homebrew/bin"),
    ];
    for c in &candidates {
        if c.join("python3").exists() || c.join("python").exists() {
            return Ok(c.clone());
        }
    }
    bail!("Could not find Python installation on this system")
}
