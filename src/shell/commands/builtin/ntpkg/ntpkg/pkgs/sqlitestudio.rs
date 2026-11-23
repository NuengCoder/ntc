//! sqlitestudio — version listing, download source and installer.

use super::super::download::download_with_progress;
use super::super::download::extract_archive;
use super::super::download::find_binary;
use super::super::manager::load_manifest;
use super::super::manager::save_manifest;
use super::super::types::bin_dir;
use super::super::types::ArchiveType;
use super::super::types::PackageSource;
use super::common::copy_recursive;
use super::common::fetch_github_releases;
use super::common::platform_key;
use super::Pkg;
use crate::output::print_info;
use crate::output::print_success;
use crate::t;
use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::path::Path;

fn fetch_sqlitestudio_versions() -> Result<Vec<String>> {
    let mut versions = fetch_github_releases("pawelsalawa", "sqlitestudio")?;
    // Filter out versions >= 4.0.0 which use "letos" asset naming
    versions.retain(|v| {
        let parts: Vec<u32> = v.split('.').filter_map(|s| s.parse().ok()).collect();
        parts.first().copied().unwrap_or(0) < 4
    });
    let mut all = vec!["latest".to_string()];
    all.extend(versions);
    Ok(all)
}

fn sqlitestudio_source(version: &str) -> Option<PackageSource> {
    let ver = if version == "latest" {
        let versions = fetch_sqlitestudio_versions().ok()?;
        // First entry beyond "latest" is the newest version
        versions.get(1)?.clone()
    } else {
        version.to_string()
    };

    let base = format!(
        "https://github.com/pawelsalawa/sqlitestudio/releases/download/{ver}/sqlitestudio-{ver}"
    );

    let mut urls = std::collections::HashMap::new();

    if cfg!(windows) {
        urls.insert(
            "windows-x86_64".to_string(),
            format!("{base}-windows-x64.zip"),
        );
        urls.insert(
            "windows-aarch64".to_string(),
            format!("{base}-windows-x64.zip"),
        );
    } else if cfg!(target_os = "linux") {
        urls.insert(
            "linux-x86_64".to_string(),
            format!("{base}-linux-x64.tar.xz"),
        );
        // No linux-arm64 binary provided upstream
    } else if cfg!(target_os = "macos") {
        urls.insert("macos-x86_64".to_string(), format!("{base}-macos-x64.dmg"));
        urls.insert("macos-aarch64".to_string(), format!("{base}-macos-x64.dmg"));
    }

    Some(PackageSource {
        name: "sqlitestudio".to_string(),
        version: ver,
        urls,
        sha256: std::collections::HashMap::new(),
        archive_type: ArchiveType::Zip,
    })
}

pub(crate) fn install_sqlitestudio(
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

    let tmp_dir = std::env::temp_dir().join(format!("ntc-install-{name}-{version}"));
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    let fname = url.rsplit_once('/').map(|(_, f)| f).unwrap_or("archive");
    let archive_path = tmp_dir.join(fname);

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    if url.ends_with(".dmg") {
        anyhow::bail!("macOS .dmg format is not supported by ntpkg. Install SQLiteStudio manually from https://sqlitestudio.pl");
    }
    let archive_type = if url.ends_with(".zip") {
        ArchiveType::Zip
    } else {
        ArchiveType::TarXz
    };

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, archive_type)?;

    let binary_name = if cfg!(windows) {
        "SQLiteStudio.exe"
    } else {
        "SQLiteStudio"
    };
    let sqlitestudio_bin = find_binary(&tmp_dir, "SQLiteStudio")
        .ok_or_else(|| anyhow::anyhow!("Could not find {binary_name} in archive"))?;

    let target_dir = bin_dir(name, version);
    fs::create_dir_all(&target_dir)?;

    // Copy entire extracted directory so the app has access to its resources
    let extracted_root = sqlitestudio_bin
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid directory structure"))?;
    copy_recursive(extracted_root, &target_dir)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let target_bin = target_dir.join(binary_name);
        if target_bin.exists() {
            fs::set_permissions(&target_bin, fs::Permissions::from_mode(0o755))?;
        }
    }

    fs::write(
        vdir.join(".real_path"),
        target_dir.to_string_lossy().as_bytes(),
    )
    .context("Failed to write .real_path")?;

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

pub(crate) static PKG: Pkg = Pkg {
    name: "sqlitestudio",
    termux: None,
    description_key: "ntpkg.src_sqlitestudio",
    versions: fetch_sqlitestudio_versions,
    source: sqlitestudio_source,
    install: install_sqlitestudio,
};
