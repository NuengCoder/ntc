//! Mechanics shared by more than one package: platform detection, copying,
//! installer launching, version validation and GitHub release fetching.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::super::types::{ArchiveType, PackageSource};
use crate::output::print_error;

/// `os-arch` key used by every `PackageSource.urls` map.
pub(crate) fn platform_key() -> String {
    let os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(any(target_os = "linux", target_os = "android")) {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "unknown"
    };

    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else if cfg!(target_arch = "x86") {
        "x86"
    } else {
        "unknown"
    };

    format!("{os}-{arch}")
}

pub(crate) fn copy_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_recursive(&src_path, &dst_path)?;
        } else {
            let _ = fs::copy(&src_path, &dst_path);
        }
    }
    Ok(())
}

pub(crate) fn find_system_binary(name: &str) -> Option<PathBuf> {
    let paths = std::env::var_os("PATH")?;
    let exe_name = if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    };
    for dir in std::env::split_paths(&paths) {
        let candidate = dir.join(&exe_name);
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

/// `x.y.z`-style version with three numeric segments.
pub(crate) fn is_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    if !parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit())) {
        return false;
    }
    true
}

pub(crate) fn fetch_github_releases(owner: &str, repo: &str) -> Result<Vec<String>> {
    let url = format!("https://api.github.com/repos/{owner}/{repo}/releases?per_page=100");
    let response = ureq::get(&url)
        .header("User-Agent", "ntc")
        .call()
        .with_context(|| format!("Failed to fetch {owner}/{repo} versions from GitHub"))?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        let stripped = tag.strip_prefix('v').unwrap_or(tag);
        if !stripped.is_empty() && !versions.iter().any(|v| v == stripped) {
            versions.push(stripped.to_string());
        }
    }

    Ok(versions)
}

/// A source for packages that are installed by an external tool (npm) or that
/// bundle several binaries: no direct download, no checksums.
pub(crate) fn empty_source(name: &'static str, version: &str) -> Option<PackageSource> {
    Some(PackageSource {
        name: name.to_string(),
        version: version.to_string(),
        urls: HashMap::new(),
        sha256: HashMap::new(),
        archive_type: ArchiveType::Zip,
    })
}

/// Launch an installer (.exe/.pkg) and wait for the user to finish.
/// On Windows, uses ShellExecute (via PowerShell) to request administrator elevation.
pub(crate) fn launch_installer_and_wait(
    installer: &Path,
    name: &str,
    _version: &str,
) -> Result<()> {
    #[cfg(windows)]
    {
        let installer_disp = installer.display();
        let script = format!(
            "Start-Process -FilePath '{}' -Wait -Verb RunAs",
            installer_disp.to_string().replace('\'', "''")
        );
        let status = Command::new("powershell")
            .args(["-NoProfile", "-Command", &script])
            .status()
            .with_context(|| format!("Failed to launch {} installer (elevated)", name))?;

        if let Some(code) = status.code() {
            if code != 0 && code != 1 {
                print_error(&format!(
                    "{} installer exited with code {:?}",
                    name,
                    status.code()
                ));
            }
        }
    }

    #[cfg(not(windows))]
    {
        let status = Command::new(installer)
            .status()
            .with_context(|| format!("Failed to launch {} installer", name))?;

        if !status.success() {
            print_error(&format!(
                "{} installer exited with code {:?}",
                name,
                status.code()
            ));
        }
    }

    Ok(())
}

/// Find an installer (.exe/.pkg) in the given directory.
pub(crate) fn find_installer_in_dir(dir: &Path, name: &str, _version: &str) -> Option<PathBuf> {
    if !dir.is_dir() {
        return None;
    }
    let ext = if cfg!(windows) { "exe" } else { "pkg" };

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
                    if fname.starts_with(&format!("{name}-")) && fname.ends_with(&format!(".{ext}"))
                    {
                        return Some(path);
                    }
                }
            }
        }
    }
    None
}
