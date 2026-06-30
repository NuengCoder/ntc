use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::download::{download_with_progress, extract_archive, find_binary};
use super::sources;
use super::types::{
    bin_dir, manifest_path, package_dir, version_dir, ArchiveType, PackageManifest,
};
use crate::output::{print_info, print_success};
use crate::utils::interactive::prompt_yes_no;

/// Install a language runtime.
pub fn install_package(name: &str, version: &str) -> Result<()> {
    let source = sources::get_source(name, version)
        .ok_or_else(|| anyhow::anyhow!("Unknown package '{name}' or version '{version}'"))?;

    let vdir = version_dir(name, version);
    if vdir.exists() {
        bail!("{} {} is already installed at {}", name, version, vdir.display());
    }

    // ── Cloudflared: single binary, install to ~/.config/ntc/bin/ ──
    if name == "cloudflare" {
        return install_cloudflared(name, version, &source, &vdir);
    }

    // ── Node.js: prebuilt binary (same flow on all platforms) ──
    if name == "node" {
        return install_prebuilt_node(name, version, &source, &vdir);
    }

    // ── Non-Windows (Linux, Termux, macOS): try system Python first ──
    #[cfg(not(windows))]
    {
        if let Some(sys_path) = detect_system_python(version)? {
            fs::create_dir_all(&vdir)?;
            fs::write(vdir.join(".real_path"), sys_path.to_string_lossy().as_bytes())
                .context("Failed to write .real_path")?;
            print_success(&format!("Using system {name} {version} at {}", sys_path.display()));

            let mut manifest = load_manifest()?;
            manifest.add_version(name, version);
            manifest.activate(name, version);
            save_manifest(&manifest)?;
            return Ok(());
        }

        // System Python not found or version mismatch

        #[cfg(target_os = "macos")]
        {
            // macOS: download and launch .pkg installer
            download_and_launch_installer(name, version, &source, &vdir)?;
        }

        #[cfg(not(target_os = "macos"))]
        {
            // Linux / Termux: build from source
            build_from_source(name, version, &source, &vdir)?;
        }
    }

    // ── Windows installer flow ──
    #[cfg(windows)]
    {
        download_and_launch_installer(name, version, &source, &vdir)?;
    }

    Ok(())
}

// ── Node.js: download & extract prebuilt binary ────────────────────────

fn install_prebuilt_node(
    name: &str,
    version: &str,
    source: &super::types::PackageSource,
    vdir: &Path,
) -> Result<()> {
    let platform_key = platform_key();
    let url = source
        .urls
        .get(&platform_key)
        .ok_or_else(|| anyhow::anyhow!("No download URL for platform '{platform_key}'"))?;

    let ext = match source.archive_type {
        ArchiveType::TarGz => "tar.gz",
        ArchiveType::Zip => "zip",
        _ => "tar.gz",
    };
    let archive_name = format!("{name}-{version}.{ext}");

    let tmp_dir = std::env::temp_dir().join(format!("ntc-install-{name}-{version}"));
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    let archive_path = tmp_dir.join(&archive_name);
    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info("Extracting...");
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let node_bin = find_binary(&tmp_dir, "node")
        .ok_or_else(|| anyhow::anyhow!("Could not find node binary in archive"))?;
    let extracted_bin = node_bin
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid binary path"))?;

    let target = bin_dir(name, version);
    fs::create_dir_all(&target)?;
    copy_recursive(extracted_bin, &target)?;

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

fn copy_recursive(src: &Path, dst: &Path) -> Result<()> {
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

/// Uninstall a package version.
pub fn uninstall_package(name: &str, version: &str) -> Result<()> {
    let vdir = version_dir(name, version);
    if !vdir.exists() {
        bail!("{} {} is not installed", name, version);
    }

    let is_system_python = vdir.join(".real_path").exists();

    if is_system_python {
        let real_path = fs::read_to_string(vdir.join(".real_path")).unwrap_or_default();
        print_info(&format!("Installed at {} (system package)", real_path.trim()));
        if !prompt_yes_no("Remove from ntpkg tracking only (system Python stays installed)", true)? {
            return Ok(());
        }
    } else {
        // Check for installer file (Windows/macOS)
        let installer_in_vdir = find_installer_in_dir(&vdir, name, version);
        if let Some(installer_path) = installer_in_vdir {
            if prompt_yes_no(
                &format!("Launch {name} installer to uninstall (click Uninstall in the window)"),
                true,
            )? {
                print_info("Launching uninstaller. Complete uninstallation, then close the window...");
                launch_installer_and_wait(&installer_path, name, version)?;
            }
        }
    }

    fs::remove_dir_all(&vdir)
        .with_context(|| format!("Failed to remove {}", vdir.display()))?;

    let mut manifest = load_manifest()?;
    manifest.remove_version(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

/// List all installed packages.
pub fn list_packages() -> Result<Vec<(String, String, bool)>> {
    let manifest = load_manifest()?;
    let mut result = Vec::new();

    for (name, versions) in &manifest.installed {
        let active = manifest.active.get(name);
        for version in versions {
            let is_active = active == Some(version);
            result.push((name.clone(), version.clone(), is_active));
        }
    }

    result.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    Ok(result)
}

/// Set the active version for a package.
pub fn use_package(name: &str, version: &str) -> Result<()> {
    let mut manifest = load_manifest()?;

    if !manifest.is_installed(name, version) {
        bail!(
            "{} {} is not installed. Use 'ntpkg install {} {}' first",
            name,
            version,
            name,
            version
        );
    }

    manifest.activate(name, version);
    save_manifest(&manifest)?;
    Ok(())
}

/// Show the path to an installed package version.
pub fn which_package(name: &str, version: Option<&str>) -> Result<PathBuf> {
    let manifest = load_manifest()?;

    let ver = match version {
        Some(v) => v.to_string(),
        None => manifest
            .active
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No active version for '{name}'"))?,
    };

    if !manifest.is_installed(name, &ver) {
        bail!("{} {} is not installed", name, ver);
    }

    Ok(resolve_bin_dir(name, &ver))
}

/// Get active versions for PATH injection.
pub fn active_bin_dirs() -> Result<Vec<PathBuf>> {
    let manifest = load_manifest()?;
    let mut dirs = Vec::new();

    for (name, version) in &manifest.active {
        if manifest.is_installed(name, version) {
            dirs.push(resolve_bin_dir(name, version));
        }
    }

    Ok(dirs)
}

// ── Resolve install dir (supports .real_path for system/installer packages) ──

fn resolve_bin_dir(name: &str, version: &str) -> PathBuf {
    let vdir = version_dir(name, version);
    let real_path_file = vdir.join(".real_path");
    if real_path_file.exists() {
        if let Ok(content) = fs::read_to_string(&real_path_file) {
            let trimmed = content.trim();
            let p = PathBuf::from(trimmed);
            if p.exists() {
                return p;
            }
        }
    }
    bin_dir(name, version)
}

// ── System Python detection (Linux / Unix / Termux) ───────────────────

/// Check if a matching Python version is already installed on the system.
/// Returns the path to its bin/ directory if found.
#[cfg(not(windows))]
fn detect_system_python(version: &str) -> Result<Option<PathBuf>> {
    // Find python3 binary
    let python_path = match find_system_binary("python3") {
        Some(p) => p,
        None => match find_system_binary("python") {
            Some(p) => p,
            None => return Ok(None),
        },
    };

    // Check version
    let output = Command::new(&python_path)
        .arg("--version")
        .output()
        .context("Failed to check system Python version")?;

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let version_str = stdout.trim();

    // Parse "Python 3.14.6" from output
    if version_str.starts_with("Python ") {
        let sys_ver = version_str.trim_start_matches("Python ").trim();
        if sys_ver == version {
            // Return the directory containing the binary
            if let Some(parent) = python_path.parent() {
                return Ok(Some(parent.to_path_buf()));
            }
        }
    }

    Ok(None)
}

/// Find a binary in PATH.
#[allow(dead_code)]
fn find_system_binary(name: &str) -> Option<PathBuf> {
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

// ── Build Python from source (Linux / Unix / Termux) ──────────────────

/// Download, extract, configure, make, and install Python into `prefix_dir`.
/// Only used on Linux / Termux (not Windows, not macOS).
#[cfg(not(any(windows, target_os = "macos")))]
fn build_python_from_source(tarball: &Path, prefix_dir: &Path, name: &str, version: &str) -> Result<()> {
    let build_dir = std::env::temp_dir().join(format!("ntc-build-python-{version}"));
    if build_dir.exists() {
        let _ = fs::remove_dir_all(&build_dir);
    }
    fs::create_dir_all(&build_dir)?;

    print_info("Extracting source tarball...");
    extract_archive(tarball, &build_dir, ArchiveType::TarGz)?;

    // Find the extracted source directory (Python-X.Y.Z/)
    let source_dir = find_source_dir(&build_dir, version)?;

    // Check for required build tools
    check_build_deps()?;

    // Configure
    print_info("Configuring (./configure)...");
    let status = Command::new("./configure")
        .arg(format!("--prefix={}", prefix_dir.display()))
        .current_dir(&source_dir)
        .status()
        .context("Failed to run ./configure")?;

    if !status.success() {
        bail!("{} ./configure failed (exit code {:?})", name, status.code());
    }

    // Make (parallel)
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

    // Make install
    print_info("Installing (make install)...");
    let status = Command::new("make")
        .arg("install")
        .current_dir(&source_dir)
        .status()
        .context("Failed to run make install")?;

    if !status.success() {
        bail!("make install failed (exit code {:?})", status.code());
    }

    // Cleanup build directory
    let _ = fs::remove_dir_all(&build_dir);

    Ok(())
}

/// Find the top-level source directory inside an extracted Python tarball.
#[cfg(not(any(windows, target_os = "macos")))]
fn find_source_dir(build_dir: &Path, version: &str) -> Result<PathBuf> {
    // Expected: <build_dir>/Python-X.Y.Z/
    let expected = build_dir.join(format!("Python-{version}"));
    if expected.is_dir() {
        return Ok(expected);
    }

    // Fallback: scan first-level subdirectories
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

/// Check that essential build tools are available.
#[cfg(not(any(windows, target_os = "macos")))]
fn check_build_deps() -> Result<()> {
    let tools = ["gcc", "make", "cc"];
    for tool in &tools {
        if find_system_binary(tool).is_none() {
            // Try common package manager hints
            let hint = if find_system_binary("pacman").is_some() {
                "sudo pacman -S base-devel"
            } else if find_system_binary("apt-get").is_some() || find_system_binary("apt").is_some() {
                "sudo apt install build-essential libssl-dev zlib1g-dev"
            } else if find_system_binary("dnf").is_some() {
                "sudo dnf groupinstall 'Development Tools'"
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

/// Get the number of parallel jobs for make.
#[cfg(not(any(windows, target_os = "macos")))]
fn parallel_jobs() -> usize {
    // Try nproc first
    if let Ok(output) = Command::new("nproc").output() {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            if let Ok(n) = s.trim().parse::<usize>() {
                return n.max(1);
            }
        }
    }
    // Fallback: try getconf
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

// ── Platform-specific install helpers ──────────────────────────────────

/// Download and launch a platform installer (.exe or .pkg).
#[allow(dead_code)]
fn download_and_launch_installer(
    name: &str,
    version: &str,
    source: &super::types::PackageSource,
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
    let installer_path = vdir.join(format!("{name}-{version}{ext}"));

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &installer_path)?;

    print_success(&format!("Downloaded to {}", installer_path.display()));

    if prompt_yes_no(&format!("Start {name} {version} installer"), true)? {
        print_info("Launching installer. Complete the setup, then close the installer window...");
        launch_installer_and_wait(&installer_path, name, version)?;

        let real_dir = find_python_install_dir()?;
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

/// Build Python from source tarball on Linux / Termux (not macOS/Windows).
#[cfg(not(any(windows, target_os = "macos")))]
fn build_from_source(
    name: &str,
    version: &str,
    source: &super::types::PackageSource,
    vdir: &Path,
) -> Result<()> {
    let platform_key = platform_key();
    let url = source
        .urls
        .get(&platform_key)
        .ok_or_else(|| anyhow::anyhow!("No download URL for platform '{platform_key}'"))?;

    print_info(&format!("Downloading {name} {version} source..."));
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

// ── Cloudflared: download single binary to ~/.config/ntc/bin/ ────

fn install_cloudflared(
    name: &str,
    version: &str,
    source: &super::types::PackageSource,
    vdir: &Path,
) -> Result<()> {
    let platform_key = platform_key();
    let url = source
        .urls
        .get(&platform_key)
        .ok_or_else(|| anyhow::anyhow!("No download URL for platform '{platform_key}'"))?;

    let bin_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("bin");
    fs::create_dir_all(&bin_dir)?;

    #[cfg(windows)]
    {
        let target = bin_dir.join("cloudflared.exe");
        print_info(&format!("Downloading cloudflared {}...", version));
        download_with_progress(url, &target)?;

        // cloudflared.exe is directly usable
        print_success(&format!("cloudflared installed at {}", target.display()));
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        let target = bin_dir.join("cloudflared");
        print_info(&format!("Downloading cloudflared {}...", version));
        download_with_progress(url, &target)?;

        // Mark executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&target, fs::Permissions::from_mode(0o755));
        }
        print_success(&format!("cloudflared installed at {}", target.display()));
    }

    #[cfg(target_os = "macos")]
    {
        let tmp_dir = std::env::temp_dir().join("ntc-install-cloudflared");
        if tmp_dir.exists() {
            let _ = fs::remove_dir_all(&tmp_dir);
        }
        fs::create_dir_all(&tmp_dir)?;

        let archive_path = tmp_dir.join("cloudflared.tgz");
        print_info(&format!("Downloading cloudflared {}...", version));
        download_with_progress(url, &archive_path)?;

        print_info("Extracting...");
        extract_archive(&archive_path, &tmp_dir, ArchiveType::TarGz)?;

        // Find cloudflared binary in extracted files
        let cf_bin = find_binary(&tmp_dir, "cloudflared")
            .ok_or_else(|| anyhow::anyhow!("Could not find cloudflared binary in archive"))?;

        let target = bin_dir.join("cloudflared");
        fs::copy(&cf_bin, &target)?;
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&target, fs::Permissions::from_mode(0o755));

        let _ = fs::remove_dir_all(&tmp_dir);
        print_success(&format!("cloudflared installed at {}", target.display()));
    }

    // Also create vdir as marker (for ntpkg list/uninstall)
    fs::create_dir_all(vdir)?;
    fs::write(vdir.join(".real_path"), bin_dir.to_string_lossy().as_bytes())?;

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

// ── Installer helpers (Windows / macOS) ───────────────────────────────

/// Launch an installer (.exe/.pkg) and wait for the user to finish.
/// On Windows, uses ShellExecute (via PowerShell) to request administrator elevation.
fn launch_installer_and_wait(installer: &Path, name: &str, _version: &str) -> Result<()> {
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
                eprintln!("ntpkg: {} installer exited with code {:?}", name, status.code());
            }
        }
    }

    #[cfg(not(windows))]
    {
        let status = Command::new(installer)
            .status()
            .with_context(|| format!("Failed to launch {} installer", name))?;

        if !status.success() {
            eprintln!("ntpkg: {} installer exited with code {:?}", name, status.code());
        }
    }

    Ok(())
}

/// Find a Python installer (.exe) in the given directory.
fn find_installer_in_dir(dir: &Path, name: &str, _version: &str) -> Option<PathBuf> {
    if !dir.is_dir() {
        return None;
    }
    let ext = if cfg!(windows) { "exe" } else { "pkg" };

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
                    if fname.starts_with(&format!("{name}-")) && fname.ends_with(&format!(".{ext}")) {
                        return Some(path);
                    }
                }
            }
        }
    }
    None
}

/// Find the real Python install directory after the installer completes.
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

// ── Manifest persistence ──────────────────────────────────────────────

fn load_manifest() -> Result<PackageManifest> {
    let path = manifest_path();
    if path.exists() {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        let manifest: PackageManifest = toml::from_str(&content)
            .with_context(|| format!("Failed to parse {}", path.display()))?;
        Ok(manifest)
    } else {
        Ok(PackageManifest::new())
    }
}

fn save_manifest(manifest: &PackageManifest) -> Result<()> {
    let dir = package_dir();
    fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create packages dir {}", dir.display()))?;

    let path = manifest_path();
    let content = toml::to_string_pretty(manifest)
        .with_context(|| "Failed to serialize manifest")?;
    fs::write(&path, &content)
        .with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

fn platform_key() -> String {
    let os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "linux") {
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
