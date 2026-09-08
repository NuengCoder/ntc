use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::sources;
use super::types::{bin_dir, manifest_path, package_dir, version_dir, PackageManifest};
#[cfg(not(windows))]
use crate::output::print_success;
use crate::output::{print_info, print_warning};
use crate::t;
use crate::utils::interactive::prompt_yes_no;

use super::installers;

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
    Some(match name {
        "python" => "python",
        "node" => "nodejs",
        "cloudflare" => "cloudflared",
        "git" => "git",
        "gh" => "gh",
        "java" => "openjdk-17",
        "kotlin" => "kotlin",
        "php" => "php",
        "zig" => "zig",
        "go" => "golang",
        "rust" => "rust",
        "ruby" => "ruby",
        "lua" => "lua54",
        "nim" => "nim",
        "dotnet" => "dotnet-sdk-10.0",
        "flutter" => "flutter",
        "kubectl" => "kubectl",
        "helm" => "helm",
        "kind" => "kind",
        "kustomize" => "kustomize",
        "docker" => "docker-cli",
        "docker-compose" => "docker-compose",
        "act" => "act",
        "scala" => "scala",
        "sqlite" => "sqlite",
        "ffmpeg" => "ffmpeg",
        "imagemagick" => "imagemagick",
        "tesseract" => "tesseract",
        "nginx" => "nginx",
        _ => return None,
    })
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

/// Install a language runtime.
pub fn install_package(name: &str, version: &str) -> Result<()> {
    if is_termux() {
        print_termux_recommendation(name);
        return Ok(());
    }

    let source = sources::get_source(name, version)
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

    if name == "cloudflare" {
        return installers::install_cloudflared(name, version, &source, &vdir);
    }

    if name == "node" {
        return installers::install_prebuilt_node(name, version, &source, &vdir);
    }

    if name == "git" {
        return installers::install_git(name, version, &source, &vdir);
    }

    if name == "gh" {
        return installers::install_gh(name, version, &source, &vdir);
    }

    if name == "java" {
        return installers::install_java(name, version, &source, &vdir);
    }

    if name == "kotlin" {
        return installers::install_kotlin(name, version, &source, &vdir);
    }

    if name == "swift" {
        return installers::install_swift(name, version, &source, &vdir);
    }

    if name == "php" {
        return installers::install_php(name, version, &source, &vdir);
    }

    if name == "zig" {
        return installers::install_zig(name, version, &source, &vdir);
    }

    if name == "go" {
        return installers::install_go(name, version, &source, &vdir);
    }

    if name == "rust" {
        return installers::install_rust(name, version, &source, &vdir);
    }

    if name == "msys2" {
        return installers::install_msys2(name, version, &source, &vdir);
    }

    if name == "ruby" {
        return installers::install_ruby(name, version, &source, &vdir);
    }

    if name == "lua" {
        return installers::install_lua(name, version, &source, &vdir);
    }

    if name == "nim" {
        return installers::install_nim(name, version, &source, &vdir);
    }

    if name == "nginx" {
        return installers::install_nginx(name, version, &source, &vdir);
    }

    if name == "dotnet" {
        return installers::install_dotnet(name, version, &source, &vdir);
    }

    if name == "flutter" {
        return installers::install_flutter(name, version, &source, &vdir);
    }

    if name == "kubectl" {
        return installers::install_kubectl(name, version, &source, &vdir);
    }

    if name == "helm" {
        return installers::install_helm(name, version, &source, &vdir);
    }

    if name == "kind" {
        return installers::install_kind(name, version, &source, &vdir);
    }

    if name == "kustomize" {
        return installers::install_kustomize(name, version, &source, &vdir);
    }

    if name == "docker" {
        return installers::install_docker(name, version, &source, &vdir);
    }

    if name == "docker-compose" {
        return installers::install_docker_compose(name, version, &source, &vdir);
    }

    if name == "act" {
        return installers::install_act(name, version, &source, &vdir);
    }

    if name == "scala" {
        return installers::install_scala(name, version, &source, &vdir);
    }

    if name == "devops-tool" {
        return installers::install_devops_tool(name, version, &source, &vdir);
    }

    if name == "opencode" {
        return installers::install_opencode(name, version, &source, &vdir);
    }

    if name == "mimocode" {
        return installers::install_mimocode(name, version, &source, &vdir);
    }

    if name == "omniroute" {
        return installers::install_omniroute(name, version, &source, &vdir);
    }

    if name == "kilocode" {
        return installers::install_kilocode(name, version, &source, &vdir);
    }

    if name == "freebuff" {
        return installers::install_freebuff(name, version, &source, &vdir);
    }

    if name == "vscode" {
        return installers::install_vscode(name, version, &source, &vdir);
    }

    if name == "sqlite" {
        return installers::install_sqlite(name, version, &source, &vdir);
    }

    if name == "sqlitestudio" {
        return installers::install_sqlitestudio(name, version, &source, &vdir);
    }

    if name == "ffmpeg" {
        return installers::install_ffmpeg(name, version, &source, &vdir);
    }

    if name == "imagemagick" {
        return installers::install_imagemagick(name, version, &source, &vdir);
    }

    if name == "tesseract" {
        return installers::install_tesseract(name, version, &source, &vdir);
    }

    #[cfg(not(windows))]
    {
        if let Some(sys_path) = installers::python::detect_system_python(version)? {
            fs::create_dir_all(&vdir)?;
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
            installers::python::download_and_launch_installer(name, version, &source, &vdir)?;
        }

        #[cfg(not(target_os = "macos"))]
        {
            installers::python::build_from_source(name, version, &source, &vdir)?;
        }
    }

    #[cfg(windows)]
    {
        installers::python::download_and_launch_installer(name, version, &source, &vdir)?;
    }

    Ok(())
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
        let installer_in_vdir = installers::find_installer_in_dir(&vdir, name, version);
        if let Some(installer_path) = installer_in_vdir {
            if prompt_yes_no(
                &format!("Launch {name} installer to uninstall (click Uninstall in the window)"),
                true,
            )? {
                print_info(
                    "Launching uninstaller. Complete uninstallation, then close the window...",
                );
                installers::launch_installer_and_wait(&installer_path, name, version)?;
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
        for name in super::super::sources::list_supported_languages() {
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
