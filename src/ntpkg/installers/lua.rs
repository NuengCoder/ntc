use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::super::download::{download_with_progress, extract_archive};
use super::super::manager::{find_system_binary, load_manifest, platform_key, save_manifest};
use super::super::types::{bin_dir, ArchiveType, PackageSource};
use crate::output::{print_info, print_success};

pub(crate) fn install_lua(
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

    fs::create_dir_all(vdir)?;

    let tarball = vdir.join(format!("lua-{version}.tar.gz"));

    print_info(&format!("Downloading {name} {version} source..."));
    download_with_progress(url, &tarball)?;

    build_lua_from_source(&tarball, name, version)?;

    let _ = fs::remove_file(&tarball);
    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

fn build_lua_from_source(
    tarball: &Path,
    name: &str,
    version: &str,
) -> Result<()> {
    let build_dir = std::env::temp_dir().join(format!("ntc-build-lua-{version}"));
    if build_dir.exists() {
        let _ = fs::remove_dir_all(&build_dir);
    }
    fs::create_dir_all(&build_dir)?;

    print_info("Extracting...");
    extract_archive(tarball, &build_dir, ArchiveType::TarGz)?;

    let source_dir = find_source_dir(&build_dir, version)?;

    check_build_deps()?;

    let make_target = detect_make_target();

    print_info(&format!("Building (make {make_target})..."));
    let status = Command::new("make")
        .arg(&make_target)
        .current_dir(&source_dir)
        .status()
        .context("Failed to run make")?;

    if !status.success() {
        bail!("make failed (exit code {:?})", status.code());
    }

    let target = bin_dir(name, version);
    fs::create_dir_all(&target)?;

    let src_dir = source_dir.join("src");
    let exe_suffix = if cfg!(windows) { ".exe" } else { "" };

    for file in ["lua", "luac"] {
        let src = src_dir.join(format!("{file}{exe_suffix}"));
        if src.exists() {
            let _ = fs::copy(&src, target.join(format!("{file}{exe_suffix}")));
        }
    }

    if cfg!(windows) {
        let short = lua_short_version(version);
        let dll_name = format!("lua{short}.dll");
        let dll_src = src_dir.join(&dll_name);
        if dll_src.exists() {
            let _ = fs::copy(&dll_src, target.join(&dll_name));
        }
    }

    let _ = fs::remove_dir_all(&build_dir);

    Ok(())
}

fn find_source_dir(build_dir: &Path, version: &str) -> Result<PathBuf> {
    let expected = build_dir.join(format!("lua-{version}"));
    if expected.is_dir() {
        return Ok(expected);
    }

    if let Ok(entries) = fs::read_dir(build_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.starts_with("lua-") || name.starts_with("Lua-") {
                    return Ok(path);
                }
            }
        }
    }

    bail!(
        "Could not find Lua source directory in {}",
        build_dir.display()
    )
}

#[cfg(not(windows))]
fn check_build_deps() -> Result<()> {
    let tools = ["gcc", "make", "cc"];
    for tool in &tools {
        if find_system_binary(tool).is_none() {
            let hint = if find_system_binary("pacman").is_some() {
                "sudo pacman -S base-devel"
            } else if find_system_binary("apt-get").is_some() || find_system_binary("apt").is_some() {
                "sudo apt install build-essential"
            } else if find_system_binary("dnf").is_some() {
                "sudo dnf groupinstall 'Development Tools'"
            } else if find_system_binary("brew").is_some() {
                "brew install gcc make"
            } else if find_system_binary("pkg").is_some() {
                "pkg install build-essential"
            } else {
                "install gcc, make and related build tools"
            };

            bail!(
                "Build tool '{tool}' not found.\n  Install build dependencies:\n    {hint}"
            );
        }
    }
    Ok(())
}

#[cfg(windows)]
fn check_build_deps() -> Result<()> {
    let tools = ["gcc", "make", "cc"];
    for tool in &tools {
        if find_system_binary(tool).is_none() {
            let hint = "Install MSYS2 via 'ntpkg install msys2', then run: pacman -S mingw-w64-ucrt-x86_64-gcc make";
            bail!(
                "Build tool '{tool}' not found.\n  Install build dependencies:\n    {hint}"
            );
        }
    }
    Ok(())
}

#[cfg(not(windows))]
fn detect_make_target() -> String {
    if cfg!(target_os = "macos") {
        "macosx".to_string()
    } else {
        "linux".to_string()
    }
}

#[cfg(windows)]
fn detect_make_target() -> String {
    "mingw".to_string()
}

fn lua_short_version(version: &str) -> String {
    let parts: Vec<&str> = version.splitn(3, '.').collect();
    format!("{}{}", parts[0], parts[1])
}
