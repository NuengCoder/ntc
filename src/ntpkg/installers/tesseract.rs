use anyhow::Result;
#[cfg(target_os = "windows")]
use anyhow::Context;
#[cfg(target_os = "windows")]
use std::fs;
use std::path::Path;
#[cfg(target_os = "windows")]
use std::path::PathBuf;
#[cfg(target_os = "windows")]
use std::process::Command;

#[cfg(target_os = "windows")]
use super::super::download::download_with_progress;
#[cfg(target_os = "windows")]
use super::super::manager::platform_key;
#[cfg(target_os = "windows")]
use super::super::manager::{load_manifest, save_manifest};
use super::super::types::PackageSource;
use crate::output::print_info;
#[cfg(target_os = "windows")]
use crate::output::print_success;
use crate::t;

pub(crate) fn install_tesseract(
    name: &str,
    version: &str,
    source: &PackageSource,
    vdir: &Path,
) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        let url = source.urls.get(&platform_key());
        install_windows(name, version, url, vdir)
    }

    #[cfg(target_os = "linux")]
    {
        let _ = (name, version, source, vdir);
        install_linux()
    }

    #[cfg(target_os = "macos")]
    {
        let _ = (name, version, source, vdir);
        install_macos()
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        let _ = (name, version, source, vdir);
        print_info(&t!("ntpkg.tesseract_unsupported"));
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn install_windows(
    name: &str,
    version: &str,
    url: Option<&String>,
    vdir: &Path,
) -> Result<()> {
    let url = url.ok_or_else(|| anyhow::anyhow!("No download URL for tesseract on Windows"))?;

    let tmp_dir = std::env::temp_dir().join(format!("ntc-install-{name}-{version}"));
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    let fname = url.rsplit('/').next().unwrap_or("tesseract-setup.exe");
    let installer_path = tmp_dir.join(fname);

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &installer_path)?;

    fs::create_dir_all(vdir)?;

    print_info(&t!("ntpkg.tesseract_running_installer"));
    let installer_disp = installer_path.display();
    let script = format!(
        "Start-Process -FilePath '{}' -ArgumentList '/S' -Wait -Verb RunAs",
        installer_disp.to_string().replace('\'', "''")
    );
    let status = Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .status()
        .with_context(|| format!("Failed to launch {name} installer (elevated)"))?;

    if let Some(code) = status.code() {
        if code != 0 && code != 1 {
            anyhow::bail!("{name} installer exited with code {code:?}");
        }
    }

    let _ = fs::remove_dir_all(&tmp_dir);

    if let Some(exe) = find_system_tesseract() {
        let parent = exe.parent().unwrap_or(Path::new(""));
        fs::write(vdir.join(".real_path"), parent.to_string_lossy().as_bytes())
            .context("Failed to write .real_path")?;
        print_success(&format!("Tesseract registered at {}", parent.display()));
    } else {
        print_info(&t!("ntpkg.tesseract_not_found"));
    }

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn find_system_tesseract() -> Option<PathBuf> {
    let paths = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&paths) {
        let candidate = dir.join("tesseract.exe");
        if candidate.exists() {
            return Some(candidate);
        }
    }
    for base in [
        "C:\\Program Files\\Tesseract-OCR",
        "C:\\Program Files (x86)\\Tesseract-OCR",
    ] {
        let candidate = PathBuf::from(base).join("tesseract.exe");
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn install_linux() -> Result<()> {
    println!();
    print_info(&t!("ntpkg.tesseract_linux"));
    println!("  sudo apt install tesseract-ocr tesseract-ocr-eng tesseract-ocr-chi-sim");
    println!("  # or: sudo pacman -S tesseract");
    println!("  # or: sudo dnf install tesseract");
    Ok(())
}

#[cfg(target_os = "macos")]
fn install_macos() -> Result<()> {
    println!();
    print_info(&t!("ntpkg.tesseract_macos"));
    println!("  brew install tesseract");
    println!("  brew install tesseract-lang");
    Ok(())
}
