//! imagemagick — version listing, download source and installer.

use super::super::download::download_with_progress;
#[cfg(any(target_os = "windows", target_os = "linux"))]
use super::super::manager::load_manifest;
#[cfg(any(target_os = "windows", target_os = "linux"))]
use super::super::manager::save_manifest;
use super::super::types::ArchiveType;
use super::super::types::PackageSource;
use super::common::platform_key;
use super::Pkg;
use crate::output::print_info;
use crate::output::print_success;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::t;
#[cfg(target_os = "windows")]
use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
#[cfg(target_os = "windows")]
use std::path::PathBuf;
#[cfg(target_os = "windows")]
use std::process::Command;

fn imagemagick_source(version: &str) -> Option<PackageSource> {
    let resolved_ver = if version == "latest" {
        fetch_latest_imagemagick_version()?
    } else {
        version.to_string()
    };

    let mut urls = HashMap::new();
    // Windows: .exe installer
    urls.insert(
        "windows-x86_64".to_string(),
        format!(
            "https://github.com/ImageMagick/ImageMagick/releases/download/{resolved_ver}/ImageMagick-{resolved_ver}-Q16-HDRI-x64-dll.exe"
        ),
    );
    // Linux: AppImage (gcc build)
    urls.insert(
        "linux-x86_64".to_string(),
        format!(
            "https://github.com/ImageMagick/ImageMagick/releases/download/{resolved_ver}/ImageMagick-{resolved_ver}-gcc-x86_64.AppImage"
        ),
    );
    // macOS: not directly supported; site recommends Homebrew

    let archive_type = platform_imagemagick_archive();

    Some(PackageSource {
        name: "imagemagick".to_string(),
        version: resolved_ver,
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

fn fetch_latest_imagemagick_version() -> Option<String> {
    let url = "https://api.github.com/repos/ImageMagick/ImageMagick/releases?per_page=10";
    let response = ureq::get(url).header("User-Agent", "ntc").call().ok()?;
    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader().read_to_string(&mut raw).ok()?;

    let releases: Vec<serde_json::Value> = serde_json::from_str(&raw).ok()?;

    for release in &releases {
        if let Some(tag) = release["tag_name"].as_str() {
            if tag.starts_with("7.") || tag.starts_with("6.") {
                return Some(tag.to_string());
            }
        }
    }

    None
}

fn platform_imagemagick_archive() -> ArchiveType {
    #[cfg(target_os = "windows")]
    {
        ArchiveType::Pkg
    }

    #[cfg(target_os = "linux")]
    {
        ArchiveType::Pkg
    }

    #[cfg(target_os = "macos")]
    {
        ArchiveType::Pkg
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        ArchiveType::Pkg
    }
}

fn latest_versions() -> Result<Vec<String>> {
    Ok(vec!["latest".to_string()])
}

pub(crate) fn install_imagemagick(
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

    let fname = url.rsplit('/').next().unwrap_or("imagemagick.bin");
    let archive_path = tmp_dir.join(fname);

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    install_platform(name, version, &archive_path, vdir)?;

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed"));
    Ok(())
}

#[cfg(target_os = "windows")]
fn install_platform(name: &str, version: &str, installer: &Path, vdir: &Path) -> Result<()> {
    fs::create_dir_all(vdir)?;
    let installer_disp = installer.display();
    print_info(&format!(
        "Launching ImageMagick installer: {}",
        installer_disp
    ));
    println!("Follow the installer wizard to complete installation.");
    println!("Make sure to enable 'Install portable binary to PATH' option.");

    let script = format!(
        "Start-Process -FilePath '{}' -Wait -Verb RunAs",
        installer_disp.to_string().replace('\'', "''")
    );
    Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .status()
        .with_context(|| format!("Failed to launch {} installer (elevated)", name))?;

    let magick_path = find_system_magick();
    if let Some(path) = magick_path {
        let parent = path.parent().unwrap_or(Path::new(""));
        fs::write(vdir.join(".real_path"), parent.to_string_lossy().as_bytes())
            .context("Failed to write .real_path")?;
        print_success(&format!("ImageMagick registered at {}", parent.display()));
    } else {
        print_info(&t!("ntpkg.magick_not_found"));
        print_info(&t!("ntpkg.magick_manual_reg"));
    }

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn find_system_magick() -> Option<PathBuf> {
    let paths = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&paths) {
        let candidate = dir.join("magick.exe");
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn install_platform(name: &str, version: &str, appimage: &Path, vdir: &Path) -> Result<()> {
    fs::create_dir_all(vdir)?;
    let dest = vdir.join("magick");
    fs::copy(appimage, &dest)?;
    fs::set_permissions(&dest, fs::Permissions::from_mode(0o755))?;

    let real_path = vdir.to_path_buf();
    fs::write(
        vdir.join(".real_path"),
        real_path.to_string_lossy().as_bytes(),
    )?;

    print_info(&format!(
        "{} AppImage installed. Run via '{}'",
        name,
        dest.display()
    ));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn install_platform(name: &str, _version: &str, _appimage: &Path, _vdir: &Path) -> Result<()> {
    println!();
    print_info(&t!("ntpkg.magick_macos"));
    println!("  Install via Homebrew: brew install imagemagick");
    println!("  Or build from source: https://imagemagick.org/download/");
    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn install_platform(_name: &str, _version: &str, _installer: &Path, _vdir: &Path) -> Result<()> {
    print_info(&format!(
        "ImageMagick installation is not supported on this platform"
    ));
    Ok(())
}

pub(crate) static PKG: Pkg = Pkg {
    name: "imagemagick",
    termux: Some("imagemagick"),
    description_key: "ntpkg.src_imagemagick",
    versions: latest_versions,
    source: imagemagick_source,
    install: install_imagemagick,
};
