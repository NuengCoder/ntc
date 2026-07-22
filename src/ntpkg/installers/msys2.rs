use anyhow::Result;
use std::fs;
use std::path::Path;

#[cfg(windows)]
use anyhow::{bail, Context};
#[cfg(windows)]
use std::path::PathBuf;
#[cfg(windows)]
use std::process::Command;
#[cfg(windows)]
use super::super::download::download_with_progress;
#[cfg(windows)]
use super::super::manager::platform_key;
use super::super::manager::{load_manifest, save_manifest};
use super::super::types::PackageSource;
use crate::output::print_info;
#[cfg(windows)]
use crate::output::print_success;
#[cfg(windows)]
use crate::utils::interactive::prompt_yes_no;

pub(crate) fn install_msys2(
    name: &str,
    version: &str,
    source: &PackageSource,
    vdir: &Path,
) -> Result<()> {
    #[cfg(windows)]
    {
        let platform_key = platform_key();
        let url = source
            .urls
            .get(&platform_key)
            .ok_or_else(|| anyhow::anyhow!("No download URL for platform '{platform_key}'"))?;

        let tmp_dir = std::env::temp_dir().join("ntc-install-msys2");
        if tmp_dir.exists() {
            let _ = fs::remove_dir_all(&tmp_dir);
        }
        fs::create_dir_all(&tmp_dir)?;

        let installer_path = tmp_dir.join("msys2-installer.exe");
        print_info(&format!("Downloading {name} {version}..."));
        download_with_progress(url, &installer_path)?;
        print_success(&format!("Downloaded to {}", installer_path.display()));

        if prompt_yes_no("Start MSYS2 installer", true)? {
            print_info("Launching MSYS2 installer. Complete the setup, then close the installer window...");
            super::launch_installer_and_wait(&installer_path, name, version)?;

            let msys2_root = detect_msys2_root()?;
            let ucrt64_bin = msys2_root.join("ucrt64").join("bin");
            let gcc_path = ucrt64_bin.join("gcc.exe");

            if !gcc_path.exists() {
                print_info("GCC not found yet. Run pacman to install it:");
                println!("  {} \\usr\\bin\\bash.exe --login -c \"pacman -S mingw-w64-ucrt-x86_64-gcc\"", msys2_root.display());
                if prompt_yes_no("Open MSYS2 terminal to install GCC now?", true)? {
                    print_info("Running: pacman -S mingw-w64-ucrt-x86_64-gcc");
                    let bash = msys2_root.join("usr").join("bin").join("bash.exe");
                    let status = Command::new(&bash)
                        .args(["--login", "-c", "pacman -S --noconfirm mingw-w64-ucrt-x86_64-gcc"])
                        .status()
                        .context("Failed to run pacman")?;
                    if !status.success() {
                        print_info("GCC installation may not have completed. You can install it later via pacman.");
                    }
                }
            }

            let gcc_version = if gcc_path.exists() {
                detect_gcc_version(&msys2_root)?
            } else {
                "latest".to_string()
            };

            print_success(&format!("MSYS2 detected at {}", msys2_root.display()));

            fs::create_dir_all(vdir)?;
            fs::write(vdir.join(".real_path"), ucrt64_bin.to_string_lossy().as_bytes())
                .context("Failed to write .real_path")?;
            fs::write(vdir.join(".msys2_root"), msys2_root.to_string_lossy().as_bytes())
                .ok();

            let mut manifest = load_manifest()?;
            manifest.add_version(name, &gcc_version);
            manifest.activate(name, &gcc_version);
            save_manifest(&manifest)?;

            print_info("MSYS2 installed! Install optional packages via pacman:");
            println!("  ntc ntpx msys2 bash --login -c \"pacman -S mingw-w64-ucrt-x86_64-SDL2\"");
            println!("  ntc ntpx msys2 bash --login -c \"pacman -S mingw-w64-ucrt-x86_64-qt6-base\"");
            println!("  ntc ntpx msys2 bash --login -c \"pacman -S mingw-w64-ucrt-x86_64-raylib\"");
            println!("  ntc ntpx msys2 bash --login -c \"pacman -S mingw-w64-ucrt-x86_64-vulkan-loader mingw-w64-ucrt-x86_64-vulkan-headers\"");
        }

        let _ = fs::remove_dir_all(&tmp_dir);
    }

    #[cfg(not(windows))]
    {
        let _ = (name, version, source, vdir);
        print_info("On this platform, install GCC with your package manager:");
        println!("  Ubuntu/Debian: sudo apt install build-essential");
        println!("  macOS:         xcode-select --install");
        println!("  Fedora:        sudo dnf install gcc gcc-c++");
        println!("  Arch:          sudo pacman -S base-devel");

        fs::create_dir_all(vdir)?;
        let mut manifest = load_manifest()?;
        manifest.add_version(name, "system");
        manifest.activate(name, "system");
        save_manifest(&manifest)?;
    }

    Ok(())
}

#[cfg(windows)]
fn detect_msys2_root() -> Result<PathBuf> {
    let default = PathBuf::from(r"C:\msys64");
    if default.join("ucrt64").join("bin").join("gcc.exe").exists() {
        return Ok(default);
    }

    let script = r#"
        Get-ItemProperty HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\* |
            Where-Object { $_.DisplayName -like "*MSYS2*" } |
            Select-Object -ExpandProperty InstallLocation
    "#;
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", script])
        .output()
        .context("Failed to query registry for MSYS2")?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            let p = PathBuf::from(trimmed);
            if p.join("ucrt64").join("bin").join("gcc.exe").exists() {
                return Ok(p);
            }
            if p.join("usr").join("bin").join("bash.exe").exists() {
                return Ok(p);
            }
        }
    }

    if default.join("usr").join("bin").join("bash.exe").exists() {
        return Ok(default);
    }

    let user_input = crate::utils::interactive::prompt_text("Enter MSYS2 installation path (e.g., C:\\msys64)")?;
    let p = PathBuf::from(user_input.trim());
    if p.join("usr").join("bin").join("bash.exe").exists() {
        Ok(p)
    } else {
        bail!("MSYS2 not found at {}. Make sure MSYS2 is installed correctly.", p.display());
    }
}

#[cfg(windows)]
fn detect_gcc_version(msys2_root: &Path) -> Result<String> {
    let bash = msys2_root.join("usr").join("bin").join("bash.exe");
    let output = Command::new(&bash)
        .args(["--login", "-c", "gcc --version"])
        .output()
        .context("Failed to run gcc --version")?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if let Some(v) = extract_semver(line) {
            return Ok(v);
        }
    }
    bail!("Could not detect GCC version from: {}", stdout);
}

#[cfg(windows)]
fn extract_semver(s: &str) -> Option<String> {
    for word in s.split_whitespace() {
        let parts: Vec<&str> = word.split('.').collect();
        if parts.len() == 3 {
            if let (Ok(_), Ok(_), Ok(_)) = (
                parts[0].parse::<u32>(),
                parts[1].parse::<u32>(),
                parts[2].parse::<u32>(),
            ) {
                return Some(word.trim_end_matches(',').trim_end_matches(')').to_string());
            }
        }
    }
    None
}
