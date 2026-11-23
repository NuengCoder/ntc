use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::pkgs::common;
use super::registry;
use super::types::{bin_dir, manifest_path, package_dir, version_dir, PackageManifest};
use crate::output::{print_info, print_warning};
use crate::t;
use crate::utils::interactive::prompt_yes_no;

/// Detect whether ntc is running inside Termux (Android).
pub(crate) fn is_termux() -> bool {
    if let Ok(prefix) = std::env::var("PREFIX") {
        if prefix.contains("com.termux") {
            return true;
        }
    }
    Path::new("/data/data/com.termux/files/usr/bin/pkg").exists()
}

/// Termux `pkg install` name for a runtime whose official linux-aarch64
/// build is glibc-linked (and thus unreliable on Android/bionic).
/// Returns `None` for packages with no package available in the official
/// Termux repos.
fn termux_pkg_hint(name: &str) -> Option<&'static str> {
    registry::find(name).and_then(|pkg| pkg.termux)
}

/// Print the `pkg install` recommendation for a Termux install.
pub(crate) fn print_termux_recommendation(name: &str) {
    match termux_pkg_hint(name) {
        Some(pkg) => {
            print_warning(&t!("ntpkg.termux_use_pkg"));
            print_info(&t!("ntpkg.termux_cmd", pkg));
        }
        None => print_warning(&t!("ntpkg.termux_no_pkg", name)),
    }
}

/// The platform's official package manager, as far as `ntpkg` can tell.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum SystemPackageManager {
    Apt,
    Apk,
    Brew,
    Dnf,
    Yum,
    Pacman,
    Termux,
}

impl SystemPackageManager {
    fn install_words(self) -> &'static str {
        match self {
            Self::Apt => "apt install",
            Self::Apk => "apk add",
            Self::Brew => "brew install",
            Self::Dnf => "dnf install",
            Self::Yum => "yum install",
            Self::Pacman => "pacman -S",
            Self::Termux => "pkg install",
        }
    }
}

fn command_exists(bin: &str) -> bool {
    std::env::var_os("PATH").is_some_and(|paths| {
        std::env::split_paths(&paths).any(|dir| {
            let path = dir.join(bin);
            path.is_file() || (cfg!(windows) && dir.join(format!("{bin}.exe")).is_file())
        })
    })
}

/// Detect the package manager an install should be delegated to.
pub(crate) fn detect_system_package_manager() -> Option<SystemPackageManager> {
    if is_termux() {
        return Some(SystemPackageManager::Termux);
    }
    if cfg!(target_os = "macos") {
        return Some(SystemPackageManager::Brew);
    }
    [
        ("apt", SystemPackageManager::Apt),
        ("dnf", SystemPackageManager::Dnf),
        ("yum", SystemPackageManager::Yum),
        ("pacman", SystemPackageManager::Pacman),
        ("apk", SystemPackageManager::Apk),
    ]
    .into_iter()
    .find(|(bin, _)| command_exists(bin))
    .map(|(_, manager)| manager)
}

/// Package name to hand to `manager`; most names match, a few do not.
fn system_package_name(manager: SystemPackageManager, name: &str) -> String {
    let mapped = match manager {
        SystemPackageManager::Apt => match name {
            "node" => "nodejs",
            "python" => "python3",
            "java" => "default-jdk",
            "sqlite" => "sqlite3",
            "lua" => "lua5.4",
            "dotnet" => "dotnet-sdk-8.0",
            "docker" => "docker.io",
            "docker-compose" => "docker-compose-v2",
            "cloudflare" => "cloudflared",
            "tesseract" => "tesseract-ocr",
            other => other,
        },
        SystemPackageManager::Dnf | SystemPackageManager::Yum => match name {
            "node" => "nodejs",
            "python" => "python3",
            "java" => "java-17-openjdk",
            "dotnet" => "dotnet-sdk-8.0",
            "docker" => "moby-engine",
            "cloudflare" => "cloudflared",
            "tesseract" => "tesseract",
            "imagemagick" => "ImageMagick",
            other => other,
        },
        SystemPackageManager::Pacman => match name {
            "node" => "nodejs",
            "java" => "jdk-openjdk",
            "gh" => "github-cli",
            "docker-compose" => "docker-compose",
            "dotnet" => "dotnet-sdk",
            "rust" => "rustup",
            "cloudflare" => "cloudflared",
            other => other,
        },
        SystemPackageManager::Apk => match name {
            "node" => "nodejs",
            "python" => "python3",
            "java" => "openjdk17",
            "lua" => "lua5.4",
            "docker" => "docker-cli",
            "cloudflare" => "cloudflared",
            "tesseract" => "tesseract-ocr",
            other => other,
        },
        SystemPackageManager::Brew => match name {
            "java" => "openjdk",
            "cloudflare" => "cloudflared",
            "tesseract" => "tesseract",
            other => other,
        },
        SystemPackageManager::Termux => termux_pkg_hint(name).unwrap_or(name),
    };
    mapped.to_string()
}

#[cfg(unix)]
fn is_root() -> bool {
    // Safety: `geteuid` takes no arguments and cannot fail.
    unsafe { libc::geteuid() == 0 }
}

#[cfg(not(unix))]
fn is_root() -> bool {
    false
}

fn system_install_command(manager: SystemPackageManager, name: &str) -> String {
    let package = system_package_name(manager, name);
    let sudo = !matches!(
        manager,
        SystemPackageManager::Brew | SystemPackageManager::Termux
    ) && !is_root();
    let prefix = if sudo { "sudo " } else { "" };
    format!("{}{} {}", prefix, manager.install_words(), package)
}

/// Non-Windows installs go through the platform package manager unless the
/// caller explicitly passes `--force`.
pub(crate) fn print_system_recommendation(name: &str) {
    let Some(manager) = detect_system_package_manager() else {
        print_warning(&t!("ntpkg.system_no_manager"));
        print_info(&t!("ntpkg.system_force_hint", name));
        return;
    };
    if manager == SystemPackageManager::Termux {
        print_termux_recommendation(name);
        print_info(&t!("ntpkg.system_force_hint", name));
        return;
    }
    print_warning(&t!("ntpkg.system_use_pkg"));
    print_info(&t!(
        "ntpkg.system_cmd",
        system_install_command(manager, name)
    ));
    print_info(&t!("ntpkg.system_force_hint", name));
}

/// Install a language runtime.
pub fn install_package(name: &str, version: &str) -> Result<()> {
    let pkg = registry::find(name)
        .ok_or_else(|| anyhow::anyhow!("Unknown package '{name}' or version '{version}'"))?;
    let source = registry::get_source(name, version)
        .ok_or_else(|| anyhow::anyhow!("Unknown package '{name}' or version '{version}'"))?;

    let vdir = version_dir(name, version);
    if vdir.exists() {
        bail!(
            "{} {} is already installed at {}",
            name,
            version,
            vdir.display()
        );
    }

    (pkg.install)(name, version, &source, &vdir)
}

/// Uninstall a package version.
pub fn uninstall_package(name: &str, version: &str) -> Result<()> {
    let vdir = version_dir(name, version);
    if !vdir.exists() {
        bail!("{} {} is not installed", name, version);
    }

    let is_system_package = vdir.join(".real_path").exists()
        && fs::read_to_string(vdir.join(".real_path"))
            .ok()
            .and_then(|s| {
                let trimmed = s.trim().to_string();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            })
            .map(|p| !PathBuf::from(p).starts_with(&vdir))
            .unwrap_or(false);

    if is_system_package {
        let real_path = fs::read_to_string(vdir.join(".real_path")).unwrap_or_default();
        print_info(&format!(
            "Installed at {} (system package)",
            real_path.trim()
        ));
        if !prompt_yes_no(
            "Remove from ntpkg tracking only (system package stays installed)",
            true,
        )? {
            return Ok(());
        }
    } else {
        let installer_in_vdir = common::find_installer_in_dir(&vdir, name, version);
        if let Some(installer_path) = installer_in_vdir {
            if prompt_yes_no(
                &format!("Launch {name} installer to uninstall (click Uninstall in the window)"),
                true,
            )? {
                print_info(
                    "Launching uninstaller. Complete uninstallation, then close the window...",
                );
                common::launch_installer_and_wait(&installer_path, name, version)?;
            }
        }
    }

    fs::remove_dir_all(&vdir).with_context(|| format!("Failed to remove {}", vdir.display()))?;

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

pub(crate) fn load_manifest() -> Result<PackageManifest> {
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

pub(crate) fn save_manifest(manifest: &PackageManifest) -> Result<()> {
    let dir = package_dir();
    fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create packages dir {}", dir.display()))?;

    let path = manifest_path();
    let content =
        toml::to_string_pretty(manifest).with_context(|| "Failed to serialize manifest")?;
    fs::write(&path, &content).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

/// Find and execute a binary from an installed package.
pub fn execute_package(name: &str, binary: &str, args: &[String]) -> Result<i32> {
    let manifest = load_manifest()?;

    let version = manifest
        .active
        .get(name)
        .ok_or_else(|| anyhow::anyhow!("No active version for '{name}'"))?;

    if !manifest.is_installed(name, version) {
        bail!("{} {} is not installed", name, version);
    }

    let vdir = version_dir(name, version);

    let bin_dir = if vdir.join(".real_path").exists() {
        let content =
            fs::read_to_string(vdir.join(".real_path")).context("Failed to read .real_path")?;
        PathBuf::from(content.trim())
    } else {
        bin_dir(name, version)
    };

    let binary_path = if cfg!(windows) {
        let exe = bin_dir.join(format!("{}.exe", binary));
        if exe.exists() {
            exe
        } else {
            bin_dir.join(format!("{}.bat", binary))
        }
    } else {
        bin_dir.join(binary)
    };

    if !binary_path.exists() {
        let available: Vec<String> = if let Ok(entries) = fs::read_dir(&bin_dir) {
            entries
                .flatten()
                .filter(|e| e.path().is_file())
                .map(|e| e.file_name().to_string_lossy().to_string())
                .collect()
        } else {
            vec![]
        };
        bail!(
            "Binary '{}' not found in '{}' (package {})\n  Available: {}",
            binary,
            bin_dir.display(),
            name,
            available.join(", ")
        );
    }

    let status = Command::new(&binary_path)
        .args(args)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .with_context(|| format!("Failed to execute {}", binary_path.display()))?;

    Ok(status.code().unwrap_or(1))
}

#[cfg(test)]
mod tests {
    use super::termux_pkg_hint;

    #[test]
    fn termux_hints_cover_all_supported_packages() {
        let explicit_none = [
            "swift",
            "msys2",
            "devops-tool",
            "opencode",
            "mimocode",
            "kilocode",
            "freebuff",
            "omniroute",
            "vscode",
            "sqlitestudio",
        ];
        for name in super::super::registry::names() {
            let hint = termux_pkg_hint(name);
            if explicit_none.contains(&name) {
                assert!(hint.is_none(), "{name} should have no pkg fallback");
            } else {
                assert!(hint.is_some(), "{name} is missing a Termux pkg hint");
            }
        }
    }

    #[test]
    fn termux_hint_renames() {
        assert_eq!(termux_pkg_hint("node"), Some("nodejs"));
        assert_eq!(termux_pkg_hint("go"), Some("golang"));
        assert_eq!(termux_pkg_hint("lua"), Some("lua54"));
        assert_eq!(termux_pkg_hint("dotnet"), Some("dotnet-sdk-10.0"));
        assert_eq!(termux_pkg_hint("docker"), Some("docker-cli"));
        assert_eq!(termux_pkg_hint("cloudflare"), Some("cloudflared"));
        assert_eq!(termux_pkg_hint("java"), Some("openjdk-17"));
        assert_eq!(termux_pkg_hint("msys2"), None);
        assert_eq!(termux_pkg_hint("opencode"), None);
    }
}
