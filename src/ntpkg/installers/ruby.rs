use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(not(windows))]
use std::process::Command;

use super::super::download::download_with_progress;
#[cfg(not(windows))]
use super::super::download::extract_archive;
use super::super::manager::{load_manifest, platform_key, save_manifest};
use super::super::types::PackageSource;
#[cfg(not(windows))]
use super::super::manager::find_system_binary;
#[cfg(not(windows))]
use super::super::types::ArchiveType;
use crate::output::{print_info, print_success};

pub(crate) fn install_ruby(
    name: &str,
    version: &str,
    source: &PackageSource,
    vdir: &Path,
) -> Result<()> {
    #[cfg(windows)]
    {
        install_ruby_windows(name, version, source, vdir)?;
    }

    #[cfg(not(windows))]
    {
        install_ruby_unix(name, version, source, vdir)?;
    }

    Ok(())
}

#[cfg(windows)]
fn install_ruby_windows(
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

    let installer_path = vdir.join(format!("rubyinstaller-{version}-x64.exe"));

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &installer_path)?;

    print_success(&format!("Downloaded to {}", installer_path.display()));

    if crate::utils::interactive::prompt_yes_no(
        &format!("Start {name} {version} installer"),
        true,
    )? {
        print_info("Launching installer. Complete the setup, then close the installer window...");
        super::launch_installer_and_wait(&installer_path, name, version)?;

        let real_dir = find_ruby_install_dir(version)?;
        print_success(&format!("Detected {name} installed at {}", real_dir.display()));

        fs::write(vdir.join(".real_path"), real_dir.to_string_lossy().as_bytes())
            .context("Failed to write .real_path")?;
    }

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

#[cfg(windows)]
fn find_ruby_install_dir(version: &str) -> Result<PathBuf> {
    let short = ruby_short_version(version);

    let search_dirs: Vec<PathBuf> = {
        let mut dirs = Vec::new();
        dirs.push(PathBuf::from(format!(r"C:\Ruby{short}")));
        dirs.push(PathBuf::from(format!(r"C:\Ruby{short}-x64")));
        if let Ok(ref pf) = std::env::var("ProgramFiles") {
            dirs.push(PathBuf::from(pf).join(format!("Ruby{short}")));
            dirs.push(PathBuf::from(pf).join(format!("Ruby{short}-x64")));
        }
        if let Ok(pfx86) = std::env::var("ProgramFiles(x86)") {
            dirs.push(PathBuf::from(pfx86).join(format!("Ruby{short}")));
        }
        dirs
    };

    for base in &search_dirs {
        if base.join("bin").join("ruby.exe").exists() {
            return Ok(base.join("bin"));
        }
        if base.join("ruby.exe").exists() {
            return Ok(base.clone());
        }
    }

    let script = r#"
        Get-ItemProperty HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*,
                          HKLM:\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\* |
            Where-Object { $_.DisplayName -like "*Ruby*" } |
            Select-Object -ExpandProperty InstallLocation
    "#;
    if let Ok(output) = std::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", script])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let p = PathBuf::from(trimmed);
                if p.join("bin").join("ruby.exe").exists() {
                    return Ok(p.join("bin"));
                }
            }
        }
    }

    bail!(
        "Could not find ruby.exe after installation.\n\
         Searched in:\n  {}",
        search_dirs
            .iter()
            .map(|d| format!("  {}", d.display()))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

#[cfg(not(windows))]
fn install_ruby_unix(
    name: &str,
    version: &str,
    source: &PackageSource,
    vdir: &Path,
) -> Result<()> {
    if let Some(sys_path) = detect_system_ruby(version)? {
        fs::create_dir_all(vdir)?;
        fs::write(vdir.join(".real_path"), sys_path.to_string_lossy().as_bytes())
            .context("Failed to write .real_path")?;
        print_success(&format!("Using system {name} {version} at {}", sys_path.display()));

        let mut manifest = load_manifest()?;
        manifest.add_version(name, version);
        manifest.activate(name, version);
        save_manifest(&manifest)?;
        return Ok(());
    }

    let platform_key = platform_key();
    let url = source
        .urls
        .get(&platform_key)
        .ok_or_else(|| anyhow::anyhow!("No download URL for platform '{platform_key}'"))?;

    print_info(&format!("Downloading {name} {version} source..."));
    let tarball = vdir.join(format!("ruby-{version}.tar.gz"));
    download_with_progress(url, &tarball)?;

    build_ruby_from_source(&tarball, vdir, name, version)?;

    let _ = fs::remove_file(&tarball);
    print_success(&format!("{name} {version} built at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

#[cfg(not(windows))]
fn detect_system_ruby(version: &str) -> Result<Option<PathBuf>> {
    let ruby_path = match find_system_binary("ruby") {
        Some(p) => p,
        None => return Ok(None),
    };

    let output = Command::new(&ruby_path)
        .arg("--version")
        .output()
        .context("Failed to check system Ruby version")?;

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let version_str = stdout.trim();

    if version_str.starts_with("ruby ") {
        let sys_ver = version_str
            .trim_start_matches("ruby ")
            .split(' ')
            .next()
            .unwrap_or("");
        if sys_ver == version {
            if let Some(parent) = ruby_path.parent() {
                return Ok(Some(parent.to_path_buf()));
            }
        }
    }

    Ok(None)
}

#[cfg(not(windows))]
fn build_ruby_from_source(
    tarball: &Path,
    prefix_dir: &Path,
    name: &str,
    version: &str,
) -> Result<()> {
    let build_dir = std::env::temp_dir().join(format!("ntc-build-ruby-{version}"));
    if build_dir.exists() {
        let _ = fs::remove_dir_all(&build_dir);
    }
    fs::create_dir_all(&build_dir)?;

    print_info("Extracting source tarball...");
    extract_archive(tarball, &build_dir, ArchiveType::TarGz)?;

    let source_dir = find_source_dir(&build_dir, version)?;

    check_build_deps()?;

    print_info("Configuring (./configure)...");
    let status = Command::new("./configure")
        .arg(format!("--prefix={}", prefix_dir.display()))
        .arg("--disable-install-doc")
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

    print_info("Installing (make install)...");
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

#[cfg(not(windows))]
fn find_source_dir(build_dir: &Path, version: &str) -> Result<PathBuf> {
    let expected = build_dir.join(format!("ruby-{version}"));
    if expected.is_dir() {
        return Ok(expected);
    }

    if let Ok(entries) = fs::read_dir(build_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.starts_with("ruby-") || name.starts_with("Ruby-") {
                    return Ok(path);
                }
            }
        }
    }

    bail!(
        "Could not find Ruby source directory in {}",
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
                "sudo apt install build-essential libssl-dev libyaml-dev zlib1g-dev"
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

#[cfg(not(windows))]
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

#[cfg(windows)]
fn ruby_short_version(version: &str) -> String {
    let parts: Vec<&str> = version.splitn(3, '.').collect();
    format!("{}{}", parts[0], parts[1])
}
