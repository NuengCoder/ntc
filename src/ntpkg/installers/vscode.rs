#[cfg(any(windows, target_os = "macos"))]
use anyhow::Context;
use anyhow::Result;
use std::path::Path;
#[cfg(any(windows, target_os = "macos"))]
use std::process::Command;

use super::super::download::download_with_progress;
use crate::output::print_info;

pub(crate) fn install_vscode(
    _name: &str,
    _version: &str,
    _source: &super::super::types::PackageSource,
    _vdir: &Path,
) -> Result<()> {
    let (url, filename) = platform_vscode_url()?;

    let tmp_dir = std::env::temp_dir().join("ntc-install-vscode");
    if tmp_dir.exists() {
        let _ = std::fs::remove_dir_all(&tmp_dir);
    }
    std::fs::create_dir_all(&tmp_dir)?;

    let installer_path = tmp_dir.join(&filename);
    print_info(&format!("Downloading VS Code {}...", filename));
    download_with_progress(&url, &installer_path)?;

    #[cfg(windows)]
    {
        print_info(&format!(
            "VS Code installer downloaded to: {}",
            installer_path.display()
        ));
        println!("Opening installer. Follow the setup wizard to complete installation.");
        Command::new("cmd")
            .args(["/c", "start", "", &installer_path.to_string_lossy()])
            .spawn()
            .context("Failed to launch VS Code installer")?;
    }

    #[cfg(target_os = "macos")]
    {
        print_info(&format!(
            "VS Code downloaded to: {}",
            installer_path.display()
        ));
        println!("Opening in Finder. Drag the VS Code app to Applications.");
        Command::new("open")
            .arg(&installer_path)
            .spawn()
            .context("Failed to open VS Code archive")?;
    }

    #[cfg(not(any(windows, target_os = "macos")))]
    {
        println!(
            "VS Code installer downloaded to: {}",
            installer_path.display()
        );
        println!("On Linux, please install VS Code using your package manager:");
        println!("  sudo apt install code        # Debian/Ubuntu");
        println!("  sudo dnf install code        # Fedora");
        println!("  sudo pacman -S code           # Arch Linux");
        println!("Or download from: https://code.visualstudio.com/download");
    }

    println!();
    println!("VS Code is not managed by ntpkg (it runs independently after install).");

    Ok(())
}

fn platform_vscode_url() -> Result<(String, String)> {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        Ok((
            "https://update.code.visualstudio.com/latest/win32-x64-user/stable".to_string(),
            "VSCodeUserSetup-x64.exe".to_string(),
        ))
    }

    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    {
        Ok((
            "https://update.code.visualstudio.com/latest/win32-arm64-user/stable".to_string(),
            "VSCodeUserSetup-arm64.exe".to_string(),
        ))
    }

    #[cfg(target_os = "macos")]
    {
        Ok((
            "https://update.code.visualstudio.com/latest/darwin-universal/stable".to_string(),
            "VSCode-darwin-universal.zip".to_string(),
        ))
    }

    #[cfg(not(any(windows, target_os = "macos")))]
    {
        Ok((String::new(), String::new()))
    }
}
