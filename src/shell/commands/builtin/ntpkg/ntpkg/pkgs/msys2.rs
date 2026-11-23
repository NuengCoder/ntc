//! msys2 — version listing, download source and installer.

#[cfg(windows)]
use super::super::download::download_with_progress;
use super::super::manager::load_manifest;
use super::super::manager::save_manifest;
use super::super::types::ArchiveType;
use super::super::types::PackageSource;
#[cfg(windows)]
use super::common::launch_installer_and_wait;
#[cfg(windows)]
use super::common::platform_key;
use super::Pkg;
use crate::output::print_info;
#[cfg(windows)]
use crate::output::print_success;
use crate::t;
#[cfg(windows)]
use crate::utils::interactive::prompt_yes_no;
#[cfg(windows)]
use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;
#[cfg(windows)]
use std::path::PathBuf;
#[cfg(windows)]
use std::process::Command;

fn fetch_msys2_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/msys2/msys2-installer/releases?per_page=30";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch MSYS2 releases from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse MSYS2 releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        if let Some(tag) = release["tag_name"].as_str() {
            if !tag.contains("nightly") && !versions.iter().any(|v| v == tag) {
                versions.push(tag.to_string());
            }
        }
    }

    versions.sort_by(|a, b| b.cmp(a));
    let mut all = vec!["latest".to_string()];
    all.extend(versions);
    Ok(all)
}

fn msys2_source(version: &str) -> Option<PackageSource> {
    // For "latest", fetch the actual tag from the latest release
    let tag = if version == "latest" {
        let url = "https://api.github.com/repos/msys2/msys2-installer/releases/latest";
        let resp = ureq::get(url).header("User-Agent", "ntc").call().ok()?;
        let mut body = resp.into_body();
        let mut raw = String::new();
        body.as_reader().read_to_string(&mut raw).ok()?;
        let v: serde_json::Value = serde_json::from_str(&raw).ok()?;
        v["tag_name"].as_str()?.to_string()
    } else {
        version.to_string()
    };

    let date_str = tag.replace('-', "");
    let url = format!(
        "https://github.com/msys2/msys2-installer/releases/download/{tag}/msys2-x86_64-{date_str}.exe"
    );

    let mut urls = HashMap::new();
    urls.insert("windows-x86_64".to_string(), url);

    Some(PackageSource {
        name: "msys2".to_string(),
        version: tag,
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::Msi,
    })
}

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
        print_info(&t!("ntpkg.downloading", format!("{name} {version}")));
        download_with_progress(url, &installer_path)?;
        print_success(&format!("Downloaded to {}", installer_path.display()));

        if prompt_yes_no("Start MSYS2 installer", true)? {
            print_info(&t!("ntpkg.launch_installer"));
            launch_installer_and_wait(&installer_path, name, version)?;

            let msys2_root = detect_msys2_root()?;
            let ucrt64_bin = msys2_root.join("ucrt64").join("bin");
            let gcc_path = ucrt64_bin.join("gcc.exe");

            if !gcc_path.exists() {
                print_info(&t!("ntpkg.msys2_gcc_not_found"));
                println!(
                    "  {} \\usr\\bin\\bash.exe --login -c \"pacman -S mingw-w64-ucrt-x86_64-gcc\"",
                    msys2_root.display()
                );
                if prompt_yes_no("Open MSYS2 terminal to install GCC now?", true)? {
                    print_info(&t!("ntpkg.msys2_running_pacman"));
                    let bash = msys2_root.join("usr").join("bin").join("bash.exe");
                    let status = Command::new(&bash)
                        .args([
                            "--login",
                            "-c",
                            "pacman -S --noconfirm mingw-w64-ucrt-x86_64-gcc",
                        ])
                        .status()
                        .context("Failed to run pacman")?;
                    if !status.success() {
                        print_info(&t!("ntpkg.msys2_gcc_incomplete"));
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
            fs::write(
                vdir.join(".real_path"),
                ucrt64_bin.to_string_lossy().as_bytes(),
            )
            .context("Failed to write .real_path")?;
            fs::write(
                vdir.join(".msys2_root"),
                msys2_root.to_string_lossy().as_bytes(),
            )
            .ok();

            let mut manifest = load_manifest()?;
            manifest.add_version(name, &gcc_version);
            manifest.activate(name, &gcc_version);
            save_manifest(&manifest)?;

            print_info(&t!("ntpkg.msys2_installed"));
            println!("  ntc ntpx msys2 bash --login -c \"pacman -S mingw-w64-ucrt-x86_64-SDL2\"");
            println!(
                "  ntc ntpx msys2 bash --login -c \"pacman -S mingw-w64-ucrt-x86_64-qt6-base\""
            );
            println!("  ntc ntpx msys2 bash --login -c \"pacman -S mingw-w64-ucrt-x86_64-raylib\"");
            println!("  ntc ntpx msys2 bash --login -c \"pacman -S mingw-w64-ucrt-x86_64-vulkan-loader mingw-w64-ucrt-x86_64-vulkan-headers\"");
        }

        let _ = fs::remove_dir_all(&tmp_dir);
    }

    #[cfg(not(windows))]
    {
        let _ = (name, version, source, vdir);
        print_info(&t!("ntpkg.msys2_linux_gcc"));
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

    let user_input =
        crate::utils::interactive::prompt_text("Enter MSYS2 installation path (e.g., C:\\msys64)")?;
    let p = PathBuf::from(user_input.trim());
    if p.join("usr").join("bin").join("bash.exe").exists() {
        Ok(p)
    } else {
        bail!(
            "MSYS2 not found at {}. Make sure MSYS2 is installed correctly.",
            p.display()
        );
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

pub(crate) static PKG: Pkg = Pkg {
    name: "msys2",
    termux: None,
    description_key: "ntpkg.src_msys2",
    versions: fetch_msys2_versions,
    source: msys2_source,
    install: install_msys2,
};
