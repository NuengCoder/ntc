use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::super::download::{download_with_progress, extract_archive, find_binary};
use super::super::manager::{load_manifest, platform_key, save_manifest};
use super::super::types::{bin_dir, ArchiveType, PackageSource};
use crate::output::{print_info, print_success};

pub(crate) fn install_docker(
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

    let archive_type = if url.ends_with(".zip") { ArchiveType::Zip } else { ArchiveType::TarGz };

    print_info("Extracting...");
    extract_archive(&archive_path, &tmp_dir, archive_type)?;

    let docker_bin = find_binary(&tmp_dir, "docker")
        .ok_or_else(|| anyhow::anyhow!("Could not find docker binary in archive"))?;

    let target_dir = bin_dir(name, version);
    fs::create_dir_all(&target_dir)?;

    let dest = target_dir.join(if cfg!(windows) { "docker.exe" } else { "docker" });
    fs::copy(&docker_bin, &dest)
        .with_context(|| format!("Failed to copy docker binary to {}", dest.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&dest, fs::Permissions::from_mode(0o755))?;
    }

    fs::write(vdir.join(".real_path"), target_dir.to_string_lossy().as_bytes())
        .context("Failed to write .real_path")?;

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}
