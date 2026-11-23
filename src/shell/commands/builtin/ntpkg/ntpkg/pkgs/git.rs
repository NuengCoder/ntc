//! git — version listing, download source and installer.

#[cfg(windows)]
use super::super::download::download_with_progress;
#[cfg(windows)]
use super::super::download::extract_archive;
#[cfg(windows)]
use super::super::download::find_binary;
use super::super::manager::load_manifest;
use super::super::manager::save_manifest;
#[cfg(windows)]
use super::super::types::bin_dir;
use super::super::types::ArchiveType;
use super::super::types::PackageSource;
#[cfg(windows)]
use super::common::copy_recursive;
use super::common::find_system_binary;
use super::common::is_valid_version;
#[cfg(windows)]
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
use std::process::Command;
use std::sync::Mutex;

/// Caches full git tag for a given version (e.g., "2.45.0" -> "v2.45.0.windows.1")
static GIT_TAG_CACHE: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

fn fetch_git_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/git-for-windows/git/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch Git versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    let mut tag_map: HashMap<String, String> = HashMap::new();

    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        if let Some(ver) = parse_git_tag(tag) {
            if !versions.iter().any(|v| v == &ver) {
                versions.push(ver.clone());
                tag_map.insert(ver, tag.to_string());
            }
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    if let Ok(mut cache) = GIT_TAG_CACHE.lock() {
        *cache = Some(tag_map);
    }

    Ok(versions)
}

fn parse_git_tag(tag: &str) -> Option<String> {
    let stripped = tag.strip_prefix('v')?;
    let dot_windows = stripped.find(".windows.")?;
    let ver = &stripped[..dot_windows];
    if is_valid_version(ver) {
        Some(ver.to_string())
    } else {
        None
    }
}

fn git_source(version: &str) -> Option<PackageSource> {
    #[cfg(windows)]
    let full_tag = GIT_TAG_CACHE
        .lock()
        .ok()
        .and_then(|c| c.as_ref()?.get(version).cloned())
        .unwrap_or_else(|| format!("v{}.windows.1", version));

    #[cfg(windows)]
    {
        let url = format!(
            "https://github.com/git-for-windows/git/releases/download/{full_tag}/MinGit-{version}-64-bit.zip"
        );
        let mut urls = HashMap::new();
        urls.insert("windows-x86_64".to_string(), url);
        Some(PackageSource {
            name: "git".to_string(),
            version: version.to_string(),
            urls,
            sha256: HashMap::new(),
            archive_type: ArchiveType::Zip,
        })
    }

    #[cfg(not(windows))]
    {
        let mut urls = HashMap::new();
        urls.insert("unix".to_string(), String::new());
        Some(PackageSource {
            name: "git".to_string(),
            version: version.to_string(),
            urls,
            sha256: HashMap::new(),
            archive_type: ArchiveType::TarGz,
        })
    }
}

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

pub(crate) static PKG: Pkg = Pkg {
    name: "git",
    termux: Some("git"),
    description_key: "ntpkg.src_git",
    versions: fetch_git_versions,
    source: git_source,
    install: install_git,
};
