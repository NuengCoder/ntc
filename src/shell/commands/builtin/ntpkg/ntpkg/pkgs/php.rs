//! php — version listing, download source and installer.

use super::super::download::download_with_progress;
use super::super::download::extract_archive;
use super::super::download::find_binary;
use super::super::manager::load_manifest;
use super::super::manager::save_manifest;
use super::super::types::ArchiveType;
use super::super::types::PackageSource;
use super::common::copy_recursive;
#[cfg(not(windows))]
use super::common::find_system_binary;
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

fn fetch_php_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/php/php-src/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch PHP versions from GitHub")?;

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
        let stripped = tag.strip_prefix("php-").unwrap_or(tag);
        if is_php_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
            versions.push(stripped.to_string());
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn is_php_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn php_source(version: &str) -> Option<PackageSource> {
    let mut urls = HashMap::new();

    // ── Windows: prebuilt zip ──
    let vs = if php_uses_vs17(version) {
        "vs17"
    } else {
        "vs16"
    };
    urls.insert(
        "windows-x86_64".to_string(),
        format!("https://downloads.php.net/~windows/releases/php-{version}-nts-Win32-{vs}-x64.zip"),
    );

    // ── Linux / macOS: source tarball ──
    let source_url = format!("https://www.php.net/distributions/php-{version}.tar.gz");
    urls.insert("linux-x86_64".to_string(), source_url.clone());
    urls.insert("linux-aarch64".to_string(), source_url.clone());
    urls.insert("macos-x86_64".to_string(), source_url.clone());
    urls.insert("macos-aarch64".to_string(), source_url);

    Some(PackageSource {
        name: "php".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::TarGz,
    })
}

fn php_uses_vs17(version: &str) -> bool {
    let parts: Vec<u32> = version.split('.').filter_map(|p| p.parse().ok()).collect();
    if parts.len() >= 2 {
        // PHP 8.4+ uses vs17, older versions use vs16
        parts[0] > 8 || (parts[0] == 8 && parts[1] >= 4)
    } else {
        false
    }
}

pub(crate) fn install_php(
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

    if url.ends_with(".zip") {
        install_php_windows(name, version, url, vdir)
    } else {
        install_php_source(name, version, url, vdir)
    }
}

// ── Windows: download + extract prebuilt .zip ────────────────────────

fn install_php_windows(name: &str, version: &str, url: &str, vdir: &Path) -> Result<()> {
    let tmp_dir = std::env::temp_dir().join(format!("ntc-install-{name}-{version}"));
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    let archive_name = format!("php-{version}.zip");
    let archive_path = tmp_dir.join(&archive_name);

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, ArchiveType::Zip)?;

    let php_exe = find_binary(&tmp_dir, "php")
        .ok_or_else(|| anyhow::anyhow!("Could not find php.exe in archive"))?;
    let extract_root = php_exe
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid PHP directory structure"))?;

    fs::create_dir_all(vdir)?;
    copy_recursive(extract_root, vdir)?;

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

// ── Linux / macOS: detect system PHP or build from source ────────────

fn install_php_source(name: &str, version: &str, url: &str, vdir: &Path) -> Result<()> {
    if let Some(sys_path) = detect_system_php(version)? {
        if prompt_yes_no(
            &format!(
                "PHP {version} is already installed at {}. Register with ntpkg?",
                sys_path.display()
            ),
            true,
        )? {
            fs::create_dir_all(vdir)?;
            fs::write(
                vdir.join(".real_path"),
                sys_path.to_string_lossy().as_bytes(),
            )
            .context("Failed to write .real_path")?;
            print_success(&format!("{} {} registered (system install)", name, version));

            let mut manifest = load_manifest()?;
            manifest.add_version(name, version);
            manifest.activate(name, version);
            save_manifest(&manifest)?;
            return Ok(());
        }
        print_info(&t!("ntpkg.php_skip_register"));
        return Ok(());
    }

    build_php_from_source(name, version, url, vdir)
}

#[cfg(not(windows))]
fn detect_system_php(version: &str) -> Result<Option<PathBuf>> {
    let php_path = match find_system_binary("php") {
        Some(p) => p,
        None => return Ok(None),
    };

    let output = Command::new(&php_path)
        .arg("--version")
        .output()
        .context("Failed to check system PHP version")?;

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let first_line = stdout.lines().next().unwrap_or("").trim();

    if first_line.starts_with("PHP ") {
        let sys_ver = first_line
            .trim_start_matches("PHP ")
            .split_whitespace()
            .next()
            .unwrap_or("");
        if sys_ver == version {
            if let Some(parent) = php_path.parent() {
                return Ok(Some(parent.to_path_buf()));
            }
        }
    }

    Ok(None)
}

#[cfg(windows)]
fn detect_system_php(_version: &str) -> Result<Option<PathBuf>> {
    Ok(None)
}

#[cfg(not(any(windows, target_os = "macos")))]
fn build_php_from_source(name: &str, version: &str, url: &str, vdir: &Path) -> Result<()> {
    let build_dir = std::env::temp_dir().join(format!("ntc-build-{name}-{version}"));
    if build_dir.exists() {
        let _ = fs::remove_dir_all(&build_dir);
    }
    fs::create_dir_all(&build_dir)?;

    let tarball = build_dir.join(format!("php-{version}.tar.gz"));

    print_info(&format!("Downloading {name} {version} source..."));
    download_with_progress(url, &tarball)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&tarball, &build_dir, ArchiveType::TarGz)?;

    let source_dir = find_php_source_dir(&build_dir, version)?;

    check_php_build_deps()?;

    print_info(&t!("ntpkg.configuring"));
    let status = Command::new("./configure")
        .arg(format!("--prefix={}", vdir.display()))
        .arg("--enable-cli")
        .arg("--disable-cgi")
        .current_dir(&source_dir)
        .status()
        .context("Failed to run ./configure")?;

    if !status.success() {
        bail!("{name} ./configure failed (exit code {:?})", status.code());
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

    print_success(&format!("{name} {version} built at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

#[cfg(target_os = "macos")]
fn build_php_from_source(name: &str, version: &str, url: &str, vdir: &Path) -> Result<()> {
    let build_dir = std::env::temp_dir().join(format!("ntc-build-{name}-{version}"));
    if build_dir.exists() {
        let _ = fs::remove_dir_all(&build_dir);
    }
    fs::create_dir_all(&build_dir)?;

    let tarball = build_dir.join(format!("php-{version}.tar.gz"));

    print_info(&format!("Downloading {name} {version} source..."));
    download_with_progress(url, &tarball)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&tarball, &build_dir, ArchiveType::TarGz)?;

    let source_dir = find_php_source_dir(&build_dir, version)?;

    print_info(&t!("ntpkg.configuring"));
    let status = Command::new("./configure")
        .arg(format!("--prefix={}", vdir.display()))
        .arg("--enable-cli")
        .arg("--disable-cgi")
        .current_dir(&source_dir)
        .status()
        .context("Failed to run ./configure")?;

    if !status.success() {
        bail!("{name} ./configure failed (exit code {:?})", status.code());
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

    print_success(&format!("{name} {version} built at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

#[cfg(windows)]
fn build_php_from_source(_name: &str, _version: &str, _url: &str, _vdir: &Path) -> Result<()> {
    bail!("PHP source builds are not supported on Windows. Use 'ntpkg install php' on Linux or macOS, or download the prebuilt Windows binary directly.")
}

#[cfg(not(windows))]
fn find_php_source_dir(build_dir: &Path, version: &str) -> Result<PathBuf> {
    let expected = build_dir.join(format!("php-{version}"));
    if expected.is_dir() {
        return Ok(expected);
    }

    if let Ok(entries) = fs::read_dir(build_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.starts_with("php-") {
                    return Ok(path);
                }
            }
        }
    }

    bail!(
        "Could not find PHP source directory in {}",
        build_dir.display()
    )
}

#[cfg(not(any(windows, target_os = "macos")))]
fn check_php_build_deps() -> Result<()> {
    let tools = ["gcc", "make"];
    for tool in &tools {
        if find_system_binary(tool).is_none() {
            let hint = if find_system_binary("apt-get").is_some()
                || find_system_binary("apt").is_some()
            {
                "sudo apt install build-essential libxml2-dev sqlite3-dev"
            } else if find_system_binary("dnf").is_some() || find_system_binary("yum").is_some() {
                "sudo dnf groupinstall 'Development Tools' && sudo dnf install libxml2-devel sqlite-devel"
            } else if find_system_binary("pacman").is_some() {
                "sudo pacman -S base-devel libxml2 sqlite"
            } else {
                "install gcc, make, libxml2-dev and sqlite3-dev"
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

#[cfg(target_os = "macos")]
fn parallel_jobs() -> usize {
    if let Ok(output) = Command::new("sysctl").args(["-n", "hw.ncpu"]).output() {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            if let Ok(n) = s.trim().parse::<usize>() {
                return n.max(1);
            }
        }
    }
    2
}

pub(crate) static PKG: Pkg = Pkg {
    name: "php",
    termux: Some("php"),
    description_key: "ntpkg.src_php",
    versions: fetch_php_versions,
    source: php_source,
    install: install_php,
};
