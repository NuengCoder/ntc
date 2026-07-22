use anyhow::Result;
use std::fs;
use std::path::Path;

use super::super::download::download_with_progress;
use super::super::manager::{load_manifest, platform_key, save_manifest};
use super::super::types::PackageSource;
use crate::output::{print_info, print_success};

pub(crate) fn install_kubectl(
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

    let bin_dir = vdir.join("bin");
    fs::create_dir_all(&bin_dir)?;

    let binary_name = if cfg!(windows) { "kubectl.exe" } else { "kubectl" };
    let target = bin_dir.join(binary_name);

    print_info(&format!("Downloading kubectl {}...", version));
    download_with_progress(url, &target)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&target, fs::Permissions::from_mode(0o755))?;
    }

    fs::write(vdir.join(".real_path"), bin_dir.to_string_lossy().as_bytes())?;

    print_success(&format!("kubectl {} installed at {}", version, target.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}
