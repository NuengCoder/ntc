use anyhow::{Context, Result};
#[cfg(windows)]
use std::fs;
use std::path::Path;

use crate::output::{print_info, print_success};
use crate::t;

#[cfg(not(windows))]
use super::super::download::{download_with_progress, extract_archive};
#[cfg(windows)]
use super::super::download::{download_with_progress, extract_archive, find_binary};
#[cfg(windows)]
use super::super::manager::{copy_recursive, load_manifest, platform_key, save_manifest};
#[cfg(not(windows))]
use super::super::manager::{load_manifest, save_manifest};
#[cfg(not(windows))]
use super::super::types::ArchiveType;
#[cfg(windows)]
use super::super::types::PackageSource;
#[cfg(not(windows))]
use crate::utils::interactive::{prompt_yes_no, InteractiveMenu, MenuItem};

pub(crate) fn install_nginx(
    name: &str,
    version: &str,
    #[allow(unused_variables)] source: &super::super::types::PackageSource,
    vdir: &Path,
) -> Result<()> {
    #[cfg(windows)]
    {
        install_windows(name, version, source, vdir)
    }

    #[cfg(target_os = "linux")]
    {
        show_system_hint_linux();
        choose_source_build(name, version, vdir)
    }

    #[cfg(target_os = "macos")]
    {
        show_system_hint_macos();
        choose_source_build(name, version, vdir)
    }

    #[cfg(target_os = "android")]
    {
        install_android(name, version, vdir)
    }

    #[cfg(not(any(
        target_os = "windows",
        target_os = "linux",
        target_os = "macos",
        target_os = "android"
    )))]
    {
        let _ = (name, version, vdir);
        println!();
        print_info(&t!("ntpkg.nginx_unsupported"));
        Ok(())
    }
}

#[cfg(windows)]
fn install_windows(name: &str, version: &str, source: &PackageSource, vdir: &Path) -> Result<()> {
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

    let archive_path = tmp_dir.join(format!("{name}-{version}.zip"));
    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let nginx_bin = find_binary(&tmp_dir, "nginx")
        .ok_or_else(|| anyhow::anyhow!("Could not find nginx.exe in archive"))?;
    let dist_root = nginx_bin
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid nginx directory structure"))?;

    fs::create_dir_all(vdir)?;
    copy_recursive(dist_root, vdir)?;

    fs::write(vdir.join(".real_path"), vdir.to_string_lossy().as_bytes())
        .context("Failed to write .real_path")?;

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn show_system_hint_linux() {
    println!();
    print_info(&t!("ntpkg.nginx_system_hint_linux"));
    println!("  sudo apt install nginx            # Debian/Ubuntu");
    println!("  sudo pacman -S nginx              # Arch");
    println!("  sudo dnf install nginx            # Fedora");
}

#[cfg(target_os = "macos")]
fn show_system_hint_macos() {
    println!();
    print_info(&t!("ntpkg.nginx_system_hint_macos"));
    println!("  brew install nginx");
}

#[cfg(target_os = "android")]
fn install_android(name: &str, version: &str, vdir: &Path) -> Result<()> {
    println!();
    print_info(&t!("ntpkg.nginx_termux"));
    println!("  pkg install nginx");
    choose_source_build(name, version, vdir)
}

/// Offer to build nginx from the official source tarball, or use the system
/// package manager commands already printed above.
#[cfg(not(windows))]
fn choose_source_build(name: &str, version: &str, vdir: &Path) -> Result<()> {
    let items = vec![
        MenuItem::with_detail(
            &t!("ntpkg.nginx_option_source"),
            format!("nginx-{version}.tar.gz"),
        ),
        MenuItem::simple(&t!("ntpkg.nginx_option_system")),
    ];
    let menu_title = t!("ntpkg.nginx_choose_method");
    let menu = InteractiveMenu::new(&menu_title)
        .items(items)
        .prompt(t!("ntpkg.enter_number", 2));
    match menu.run()? {
        Some(0) => build_from_source(name, version, vdir),
        Some(1) => {
            print_info(&t!("ntpkg.nginx_run_commands_system"));
            Ok(())
        }
        _ => {
            print_info(&t!("ntpkg.skip_register", name));
            Ok(())
        }
    }
}

/// Build nginx from the official source tarball into `vdir`.
#[cfg(not(windows))]
fn build_from_source(name: &str, version: &str, vdir: &Path) -> Result<()> {
    if !prompt_yes_no(&t!("ntpkg.nginx_build_confirm", version), true)? {
        print_info(&t!("ntpkg.skip_register", name));
        return Ok(());
    }

    print_info(&t!("ntpkg.nginx_build_deps"));

    let url = format!("https://nginx.org/download/{name}-{version}.tar.gz");

    let tmp_dir = std::env::temp_dir().join(format!("ntc-install-{name}-{version}"));
    if tmp_dir.exists() {
        let _ = std::fs::remove_dir_all(&tmp_dir);
    }
    std::fs::create_dir_all(&tmp_dir)?;

    let archive_path = tmp_dir.join(format!("{name}-{version}.tar.gz"));
    print_info(&format!("Downloading {name} {version} source..."));
    download_with_progress(&url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, ArchiveType::TarGz)?;

    let src_dir = tmp_dir.join(format!("{name}-{version}"));
    if !src_dir.exists() {
        anyhow::bail!(
            "Unexpected source layout: {} not found after extraction",
            src_dir.display()
        );
    }

    print_info(&t!("ntpkg.nginx_configuring"));
    run_in_dir(
        &src_dir,
        "sh",
        &[
            "-c".to_string(),
            format!("./configure --prefix={}", vdir.display()),
        ],
    )
    .context("Failed to run ./configure (are build deps installed? see hint above)")?;

    print_info(&t!("ntpkg.nginx_compiling"));
    run_in_dir(&src_dir, "make", &[]).context("Failed to run make")?;
    run_in_dir(&src_dir, "make", &["install".to_string()]).context("Failed to run make install")?;

    let sbin = vdir.join("sbin");
    if sbin.join("nginx").exists() {
        std::fs::write(vdir.join(".real_path"), sbin.to_string_lossy().as_bytes())
            .context("Failed to write .real_path")?;
    }

    let _ = std::fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

#[cfg(not(windows))]
fn run_in_dir(dir: &Path, program: &str, args: &[String]) -> Result<()> {
    use std::process::Command;
    let status = Command::new(program)
        .args(args)
        .current_dir(dir)
        .status()
        .with_context(|| format!("Failed to run '{program}' in {}", dir.display()))?;
    if !status.success() {
        anyhow::bail!("'{program}' exited with {:?}", status.code());
    }
    Ok(())
}
