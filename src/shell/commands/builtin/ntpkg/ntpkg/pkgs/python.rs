//! python — version listing, download source and installer.

use super::super::download::download_with_progress;
#[cfg(not(any(windows, target_os = "macos")))]
use super::super::download::extract_archive;
use super::super::manager::load_manifest;
use super::super::manager::save_manifest;
use super::super::types::ArchiveType;
use super::super::types::PackageSource;
#[cfg(not(windows))]
use super::common::find_system_binary;
use super::common::is_valid_version;
use super::common::launch_installer_and_wait;
use super::common::platform_key;
use super::Pkg;
use crate::output::print_info;
use crate::output::print_success;
use crate::t;
use crate::utils::interactive::prompt_yes_no;
use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
#[cfg(not(windows))]
use std::process::Command;

fn fetch_python_versions() -> Result<Vec<String>> {
    let url = "https://www.python.org/ftp/python/";
    let response = ureq::get(url)
        .call()
        .context("Failed to fetch Python version list from python.org")?;

    let mut body = response.into_body();
    let mut html = String::new();
    body.as_reader()
        .read_to_string(&mut html)
        .context("Failed to read python.org response body")?;

    let mut versions: Vec<String> = Vec::new();
    let mut search_start = 0;

    while let Some(href_start) = html[search_start..].find("<a href=\"") {
        let start = search_start + href_start + 9; // after <a href="
        if let Some(quote_end) = html[start..].find('"') {
            let url_target = &html[start..start + quote_end];

            // Look for version-like directory entries ending with /
            if url_target.ends_with('/') {
                let candidate = url_target.trim_end_matches('/');
                if is_valid_version(candidate) && !versions.iter().any(|v| v == candidate) {
                    versions.push(candidate.to_string());
                }
            }

            search_start = start + quote_end + 1;
        } else {
            break;
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn python_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://www.python.org/ftp/python/{version}/");

    let (platform_key, filename, archive_type) = platform_python_archive(version)?;

    let url = format!("{base}{filename}");
    let mut urls = HashMap::new();
    urls.insert(platform_key.to_string(), url.clone());
    // The Linux source tarball is arch-independent — register it for both
    // arch keys so aarch64 Linux doesn't hit "No download URL for platform".
    if platform_key.starts_with("linux-") {
        urls.insert("linux-x86_64".to_string(), url.clone());
        urls.insert("linux-aarch64".to_string(), url);
    }

    Some(PackageSource {
        name: "python".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

#[allow(unused_variables)]
fn platform_python_archive(version: &str) -> Option<(&'static str, String, ArchiveType)> {
    #[cfg(target_os = "windows")]
    {
        // ARM64 Windows has its own installer (verified on python.org FTP);
        // x64 emulation works but the native build is the right default.
        if cfg!(target_arch = "aarch64") {
            let f = format!("python-{version}-arm64.exe");
            Some(("windows-aarch64", f, ArchiveType::Msi))
        } else {
            let f = format!("python-{version}-amd64.exe");
            Some(("windows-x86_64", f, ArchiveType::Msi))
        }
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        let f = format!("Python-{version}.tar.xz");
        Some(("linux-x86_64", f, ArchiveType::TarXz))
    }

    #[cfg(target_os = "macos")]
    {
        let f = format!("python-{version}-macos11.pkg");
        Some(("macos-universal", f, ArchiveType::Pkg))
    }

    #[cfg(not(any(
        target_os = "windows",
        target_os = "linux",
        target_os = "android",
        target_os = "macos"
    )))]
    {
        None
    }
}

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
    fs::create_dir_all(vdir).with_context(|| format!("Failed to create {}", vdir.display()))?;
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
    fs::create_dir_all(vdir).with_context(|| format!("Failed to create {}", vdir.display()))?;
    let installer_path = vdir.join(format!("{name}-{version}{ext}"));

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &installer_path)?;

    print_success(&format!("Downloaded to {}", installer_path.display()));

    if prompt_yes_no(&format!("Start {name} {version} installer"), true)? {
        print_info(&t!("ntpkg.launch_installer"));
        launch_installer_and_wait(&installer_path, name, version)?;

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

/// Install Python: reuse a matching system interpreter when possible,
/// otherwise run the platform installer (Windows/macOS) or build from source.
pub(crate) fn install_python(
    name: &str,
    version: &str,
    source: &PackageSource,
    vdir: &Path,
) -> Result<()> {
    #[cfg(not(windows))]
    {
        if let Some(sys_path) = detect_system_python(version)? {
            fs::create_dir_all(vdir)?;
            fs::write(
                vdir.join(".real_path"),
                sys_path.to_string_lossy().as_bytes(),
            )
            .context("Failed to write .real_path")?;
            print_success(&format!(
                "Using system {name} {version} at {}",
                sys_path.display()
            ));

            let mut manifest = load_manifest()?;
            manifest.add_version(name, version);
            manifest.activate(name, version);
            save_manifest(&manifest)?;
            return Ok(());
        }

        #[cfg(target_os = "macos")]
        {
            download_and_launch_installer(name, version, source, vdir)?;
        }

        #[cfg(not(target_os = "macos"))]
        {
            build_from_source(name, version, source, vdir)?;
        }
    }

    #[cfg(windows)]
    {
        download_and_launch_installer(name, version, source, vdir)?;
    }

    Ok(())
}

pub(crate) static PKG: Pkg = Pkg {
    name: "python",
    termux: Some("python"),
    description_key: "ntpkg.src_python",
    versions: fetch_python_versions,
    source: python_source,
    install: install_python,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn python_source_matches_platform_archive() {
        let src = python_source("3.14.5").expect("python source should resolve");
        if cfg!(target_os = "windows") {
            assert_eq!(src.archive_type, ArchiveType::Msi);
            let (key, suffix) = if cfg!(target_arch = "aarch64") {
                ("windows-aarch64", "python-3.14.5-arm64.exe")
            } else {
                ("windows-x86_64", "python-3.14.5-amd64.exe")
            };
            let url = src.urls.get(key).expect("windows python URL missing");
            assert!(
                url.ends_with(suffix),
                "unexpected python URL for {key}: {url}"
            );
        } else if cfg!(any(target_os = "linux", target_os = "android")) {
            // Source tarball is .tar.xz and must be extracted as TarXz
            // (a TarGz decoder on xz data fails).
            assert_eq!(src.archive_type, ArchiveType::TarXz);
            for key in ["linux-x86_64", "linux-aarch64"] {
                let url = src.urls.get(key).expect("linux python URL missing");
                assert!(
                    url.ends_with("Python-3.14.5.tar.xz"),
                    "unexpected python URL for {key}: {url}"
                );
            }
        } else if cfg!(target_os = "macos") {
            assert_eq!(src.archive_type, ArchiveType::Pkg);
        }
    }
}
