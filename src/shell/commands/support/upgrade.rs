use crate::output::{print_error, print_info, print_success};
use crate::t;
use anyhow::{anyhow, Context, Result};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

const REPO: &str = "NuengCoder/ntc";
const GITHUB_API: &str = "https://api.github.com";

pub fn cmd_upgrade() -> Result<bool> {
    let current = env!("CARGO_PKG_VERSION");

    print_info(&t!("cmd.upgrade.checking", current));

    let latest = match fetch_latest_release() {
        Ok(v) => v,
        Err(e) => {
            print_error(&t!("cmd.upgrade.check_fail", e));
            return Ok(false);
        }
    };

    if latest == current {
        print_success(&t!("cmd.upgrade.already_latest", current));
        return Ok(false);
    }

    if !is_newer(&latest, current) {
        print_info(&t!("cmd.upgrade.ahead", current, latest));
        print_info(&t!("cmd.upgrade.dev_build"));
        return Ok(false);
    }

    println!();
    print_info(&t!("cmd.upgrade.new_version", latest));
    print_info(&t!("cmd.upgrade.downloading"));

    let exe_path = std::env::current_exe().context(t!("cmd.upgrade.exe_path_fail").to_string())?;
    let exe_path = resolve_executable_path(&exe_path);

    let tmp_dir = std::env::temp_dir().join("ntc_update");
    std::fs::create_dir_all(&tmp_dir).context(t!("cmd.upgrade.tmpdir_fail").to_string())?;

    let bin_key = match platform_binary_key() {
        Ok(k) => k,
        Err(e) => {
            print_error(&e.to_string());
            return Ok(false);
        }
    };

    let assets = match fetch_assets(&latest) {
        Ok(a) => a,
        Err(e) => {
            print_error(&t!("cmd.upgrade.check_fail", e));
            return Ok(false);
        }
    };

    let bin_url = match pick_binary_asset(&assets, &bin_key) {
        Some(u) => u,
        None => {
            #[cfg(windows)]
            {
                if let Some(inst) = pick_installer_asset(&assets) {
                    return launch_installer_download(&inst).map(|_| false);
                }
            }
            print_error(&t!("cmd.upgrade.no_asset_match", bin_key));
            return Ok(false);
        }
    };

    let exe_names: &[&str] = if cfg!(windows) {
        &["ntc.exe", "ntc"]
    } else {
        &["ntc"]
    };

    let tmp_exe = tmp_dir.join(if cfg!(windows) {
        "ntc_new.exe"
    } else {
        "ntc_new"
    });

    if let Err(e) = download_extract_binary(&bin_url, &tmp_exe, exe_names) {
        print_error(&t!("cmd.upgrade.download_fail", e));
        return Ok(false);
    }

    print_success(&t!("cmd.upgrade.download_complete"));

    if let Err(e) = install_binary(&tmp_exe, &exe_path, &assets, &latest) {
        print_error(&t!("cmd.upgrade.install_fail", e));
        return Ok(false);
    }

    Ok(false)
}

fn fetch_latest_release() -> Result<String> {
    let url = format!("{GITHUB_API}/repos/{REPO}/releases/latest");

    let response = ureq::get(&url)
        .header("User-Agent", "ntc")
        .call()
        .map_err(|e| anyhow!("{}", t!("cmd.upgrade.api_request_fail", e)))?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context(t!("cmd.upgrade.api_read_fail").to_string())?;

    let json: serde_json::Value =
        serde_json::from_str(&raw).context(t!("cmd.upgrade.api_parse_fail").to_string())?;

    let tag = json["tag_name"]
        .as_str()
        .ok_or_else(|| anyhow!("{}", t!("cmd.upgrade.missing_tag")))?;

    Ok(tag.trim_start_matches('v').to_string())
}

fn fetch_assets(version: &str) -> Result<Vec<(String, String)>> {
    let url = format!("{GITHUB_API}/repos/{REPO}/releases/tags/v{version}");

    let response = ureq::get(&url)
        .header("User-Agent", "ntc")
        .call()
        .map_err(|e| anyhow!("{}", t!("cmd.upgrade.api_request_fail", e)))?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context(t!("cmd.upgrade.api_read_fail").to_string())?;

    let json: serde_json::Value =
        serde_json::from_str(&raw).context(t!("cmd.upgrade.api_parse_fail").to_string())?;

    let assets = json["assets"]
        .as_array()
        .ok_or_else(|| anyhow!("{}", t!("cmd.upgrade.no_assets_release", version)))?;

    let mut out = Vec::new();
    for asset in assets {
        if let (Some(name), Some(url)) = (
            asset["name"].as_str(),
            asset["browser_download_url"].as_str(),
        ) {
            if !name.is_empty() && !url.is_empty() {
                out.push((url.to_string(), name.to_string()));
            }
        }
    }

    Ok(out)
}

fn download_to_vec(url: &str) -> Result<Vec<u8>> {
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .map_err(|e| anyhow!("{}", t!("cmd.upgrade.download_fail", e)))?;

    let total = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);

    let mut downloaded: u64 = 0;
    let mut buf = Vec::new();
    let mut body = response.into_body();
    let mut reader = body.as_reader();
    let mut chunk = vec![0u8; 8192];

    loop {
        let n = reader
            .read(&mut chunk)
            .context(t!("cmd.upgrade.stream_fail").to_string())?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
        downloaded += n as u64;
        if total > 0 {
            let pct = downloaded as f64 / total as f64 * 100.0;
            print!(
                "\r  {}",
                t!("cmd.upgrade.progress_pct", pct, downloaded, total)
            );
        } else {
            print!("\r  {}", t!("cmd.upgrade.progress_bytes", downloaded));
        }
    }
    println!();

    Ok(buf)
}

fn download_extract_binary(url: &str, tmp_exe: &Path, exe_names: &[&str]) -> Result<()> {
    print_info(&t!("cmd.upgrade.downloading_url", url));

    let buf = download_to_vec(url)?;
    let lower = url.to_lowercase();

    let bytes = if lower.ends_with(".zip") {
        print_info(&t!("cmd.upgrade.extracting"));
        extract_zip(&buf, exe_names)?
    } else if lower.ends_with(".tar.gz") || lower.ends_with(".tgz") || lower.ends_with(".gz") {
        print_info(&t!("cmd.upgrade.extracting"));
        extract_tar_gz(&buf, exe_names)?
    } else {
        buf
    };

    std::fs::write(tmp_exe, &bytes).context(t!("cmd.upgrade.write_fail").to_string())?;
    Ok(())
}

fn extract_zip(buf: &[u8], exe_names: &[&str]) -> Result<Vec<u8>> {
    let mut archive = zip::ZipArchive::new(Cursor::new(buf))
        .map_err(|e| anyhow!("{}", t!("cmd.upgrade.extract_fail", e)))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| anyhow!("{}", t!("cmd.upgrade.extract_fail", e)))?;
        if file.is_dir() {
            continue;
        }
        let name = file.name().to_string();
        let base = Path::new(&name)
            .file_name()
            .map(|b| b.to_string_lossy().into_owned())
            .unwrap_or_default();
        if exe_names.contains(&base.as_str()) {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)
                .context(t!("cmd.upgrade.extract_fail").to_string())?;
            return Ok(bytes);
        }
    }

    Err(anyhow!(
        "{}",
        t!("cmd.upgrade.no_file_in_archive", exe_names[0])
    ))
}

fn extract_tar_gz(buf: &[u8], exe_names: &[&str]) -> Result<Vec<u8>> {
    use flate2::read::GzDecoder;

    let decoder = GzDecoder::new(Cursor::new(buf));
    let mut archive = tar::Archive::new(decoder);

    let entries = archive
        .entries()
        .map_err(|e| anyhow!("{}", t!("cmd.upgrade.extract_fail", e)))?;

    for entry in entries {
        let mut file = entry.map_err(|e| anyhow!("{}", t!("cmd.upgrade.extract_fail", e)))?;
        if !file.header().entry_type().is_file() {
            continue;
        }
        let path = file
            .path()
            .map(|p| p.into_owned())
            .map_err(|e| anyhow!("{}", t!("cmd.upgrade.extract_fail", e)))?;
        let base = path
            .file_name()
            .map(|b| b.to_string_lossy().into_owned())
            .unwrap_or_default();
        if exe_names.contains(&base.as_str()) {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)
                .context(t!("cmd.upgrade.extract_fail").to_string())?;
            return Ok(bytes);
        }
    }

    Err(anyhow!(
        "{}",
        t!("cmd.upgrade.no_file_in_archive", exe_names[0])
    ))
}

fn pick_binary_asset(assets: &[(String, String)], bin_key: &str) -> Option<String> {
    let mut fallback: Option<String> = None;
    for (url, name) in assets {
        let lower = name.to_lowercase();
        if !lower.contains(bin_key) || lower.contains("installer") || lower.ends_with(".deb") {
            continue;
        }
        // Exclude binaries for other platforms
        #[cfg(all(target_os = "linux", not(target_os = "android")))]
        if lower.contains("android") || lower.contains("windows") || lower.contains("macos") {
            continue;
        }
        #[cfg(target_os = "android")]
        if !lower.contains("android") {
            continue;
        }
        #[cfg(windows)]
        if !lower.contains("windows") {
            continue;
        }
        #[cfg(target_os = "macos")]
        if !lower.contains("macos") {
            continue;
        }
        #[cfg(windows)]
        {
            if lower.ends_with(".zip") {
                return Some(url.clone());
            }
            if fallback.is_none() {
                fallback = Some(url.clone());
            }
        }
        #[cfg(not(windows))]
        {
            if lower.ends_with(".tar.gz") || lower.ends_with(".tgz") {
                return Some(url.clone());
            }
            if fallback.is_none() {
                fallback = Some(url.clone());
            }
        }
    }
    fallback
}

#[cfg(windows)]
fn pick_installer_asset(assets: &[(String, String)]) -> Option<String> {
    assets
        .iter()
        .map(|(u, n)| (u, n.to_lowercase()))
        .find(|(_, n)| n.contains("installer") && n.ends_with(".exe"))
        .map(|(u, _)| u.clone())
}

#[cfg(windows)]
fn launch_installer_download(installer_url: &str) -> Result<()> {
    let tmp_dir = std::env::temp_dir().join("ntc_update");
    std::fs::create_dir_all(&tmp_dir).context(t!("cmd.upgrade.tmpdir_fail").to_string())?;

    let installer_path = tmp_dir.join("ntc-installer.exe");
    let buf =
        download_to_vec(installer_url).context(t!("cmd.upgrade.download_fail").to_string())?;
    std::fs::write(&installer_path, &buf).context(t!("cmd.upgrade.write_fail").to_string())?;

    print_info(&t!("cmd.upgrade.installer_launch"));
    std::process::Command::new(&installer_path)
        .args(["/VERYSILENT", "/SUPPRESSMSGBOXES", "/NORESTART", "/SP-"])
        .spawn()
        .context(t!("cmd.upgrade.installer_launch_fail").to_string())?;

    std::process::exit(0);
}

fn platform_binary_key() -> Result<String> {
    #[cfg(windows)]
    {
        let key = windows_asset_key(std::env::consts::ARCH).ok_or_else(|| {
            anyhow!(
                "{}",
                t!("cmd.upgrade.no_arch_asset", std::env::consts::ARCH)
            )
        })?;
        Ok(key.to_string())
    }
    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    {
        match std::env::consts::ARCH {
            "x86_64" => Ok("linux-x86_64".to_string()),
            "aarch64" => Ok("linux-aarch64".to_string()),
            _ => Err(anyhow!(
                "{}",
                t!("cmd.upgrade.no_arch_asset", std::env::consts::ARCH)
            )),
        }
    }
    #[cfg(target_os = "android")]
    {
        match std::env::consts::ARCH {
            "aarch64" => Ok("aarch64-linux-android".to_string()),
            "x86_64" => Ok("x86_64-linux-android".to_string()),
            "arm" => Ok("armv7-linux-androideabi".to_string()),
            _ => Err(anyhow!(
                "{}",
                t!("cmd.upgrade.no_arch_asset", std::env::consts::ARCH)
            )),
        }
    }
    #[cfg(target_os = "macos")]
    {
        match std::env::consts::ARCH {
            "aarch64" => Ok("macos-aarch64".to_string()),
            "x86_64" => Ok("macos-x86_64".to_string()),
            _ => Err(anyhow!(
                "{}",
                t!("cmd.upgrade.no_arch_asset", std::env::consts::ARCH)
            )),
        }
    }
    #[cfg(not(any(
        windows,
        target_os = "linux",
        target_os = "android",
        target_os = "macos"
    )))]
    {
        Err(anyhow!(
            "{}",
            t!("cmd.upgrade.unsupported_platform", std::env::consts::ARCH)
        ))
    }
}

#[cfg(windows)]
fn windows_asset_key(arch: &str) -> Option<&'static str> {
    match arch {
        "x86_64" => Some("windows-x86_64"),
        "aarch64" => Some("windows-arm64"),
        _ => None,
    }
}

fn resolve_executable_path(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(windows)]
fn dir_is_writable(dir: &Path) -> bool {
    let probe = dir.join(format!(".ntc_write_probe_{}", std::process::id()));
    let ok = std::fs::File::create(&probe).is_ok();
    if ok {
        let _ = std::fs::remove_file(&probe);
    }
    ok
}

fn install_binary(
    tmp_path: &Path,
    dst_path: &Path,
    assets: &[(String, String)],
    version: &str,
) -> Result<bool> {
    let result = install_binary_impl(tmp_path, dst_path, assets, version);

    if result.is_ok() {
        println!();
        print_success(&t!("cmd.upgrade.updated", version));
        print_info(&t!("cmd.upgrade.restart"));
        println!();
    }

    result
}

#[cfg(windows)]
fn install_binary_impl(
    tmp_path: &Path,
    dst_path: &Path,
    assets: &[(String, String)],
    _version: &str,
) -> Result<bool> {
    let dst_dir = dst_path
        .parent()
        .ok_or_else(|| anyhow!("{}", t!("cmd.upgrade.exe_path_fail", dst_path.display())))?;

    if dir_is_writable(dst_dir) {
        let tmp_dir = std::env::temp_dir().join("ntc_update");
        let script_path = tmp_dir.join("upgrade.ps1");

        let ps_script = format!(
            r#"Start-Sleep -Seconds 2
$src = '{src}'
$dst = '{dst}'
$maxRetries = 5
for ($i = 0; $i -lt $maxRetries; $i++) {{
    try {{
        $f = [System.IO.File]::Open($dst, 'Open', 'ReadWrite', 'None')
        $f.Close()
        break
    }} catch {{
        Start-Sleep -Seconds 2
    }}
}}
try {{ Move-Item -LiteralPath $src -Destination $dst -Force }}
catch {{ Copy-Item -LiteralPath $src -Destination $dst -Force }}
Remove-Item -LiteralPath $src -ErrorAction SilentlyContinue"#,
            src = tmp_path.display(),
            dst = dst_path.display(),
        );

        std::fs::write(&script_path, &ps_script)
            .context(t!("cmd.upgrade.write_fail").to_string())?;

        let _ = std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-WindowStyle",
                "Hidden",
                "-File",
                &script_path.to_string_lossy(),
            ])
            .spawn();

        print_info(&t!("cmd.upgrade.after_restart"));
        println!();

        std::process::exit(0);
    }

    let installer_url = pick_installer_asset(assets)
        .ok_or_else(|| anyhow!("{}", t!("cmd.upgrade.no_installer_asset")))?;

    launch_installer_download(&installer_url).map(|_| false)
}

#[cfg(unix)]
fn replace_binary_at(tmp_path: &Path, dst_path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let dst_dir = dst_path
        .parent()
        .ok_or_else(|| anyhow!("{}", t!("cmd.upgrade.exe_path_fail", dst_path.display())))?;

    let staging = dst_dir.join(format!(".ntc_new_{}", std::process::id()));

    let cleanup = || {
        let _ = std::fs::remove_file(&staging);
    };

    std::fs::copy(tmp_path, &staging).map_err(|e| anyhow!("{}", t!("cmd.upgrade.copy_fail", e)))?;

    if let Err(e) = std::fs::set_permissions(&staging, std::fs::Permissions::from_mode(0o755)) {
        cleanup();
        return Err(anyhow!("{}", t!("cmd.upgrade.perm_fail", e)));
    }

    std::fs::rename(&staging, dst_path)
        .map_err(|e| anyhow!("{}", t!("cmd.upgrade.copy_fail", e)))?;

    Ok(())
}

#[cfg(all(unix, not(target_os = "android")))]
fn install_binary_impl(
    tmp_path: &Path,
    dst_path: &Path,
    _assets: &[(String, String)],
    _version: &str,
) -> Result<bool> {
    match replace_binary_at(tmp_path, dst_path) {
        Ok(()) => Ok(false),
        Err(_direct_fail) => {
            let home = dirs::home_dir().context(t!("cmd.upgrade.home_fail").to_string())?;
            let bin_dir = home.join(".local").join("bin");
            std::fs::create_dir_all(&bin_dir).context(t!("cmd.upgrade.home_fail").to_string())?;
            let bin_path = bin_dir.join("ntc");
            replace_binary_at(tmp_path, &bin_path)?;

            #[cfg(target_os = "macos")]
            {
                let _ = std::process::Command::new("xattr")
                    .arg("-cr")
                    .arg(&bin_path)
                    .status();
            }

            ensure_path_export(&bin_dir)?;
            print_success(&t!("cmd.upgrade.installed_user_dir", bin_path.display()));
            println!();
            Ok(false)
        }
    }
}

#[cfg(target_os = "android")]
fn install_binary_impl(
    tmp_path: &Path,
    dst_path: &Path,
    _assets: &[(String, String)],
    _version: &str,
) -> Result<bool> {
    let backup = dst_path.with_extension("ntc_bak");
    let had_backup = std::fs::copy(dst_path, &backup).is_ok();

    let result = replace_binary_at(tmp_path, dst_path);

    if let Err(e) = result {
        if had_backup {
            let _ = std::fs::copy(&backup, dst_path);
            let _ = std::fs::remove_file(&backup);
        }
        return Err(e).context(t!("cmd.upgrade.copy_fail_android").to_string());
    }

    // Verify the new binary works
    let ok = std::process::Command::new(dst_path)
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !ok && had_backup {
        let _ = std::fs::copy(&backup, dst_path);
    }
    let _ = std::fs::remove_file(&backup);

    Ok(false)
}

#[cfg(not(any(unix, windows)))]
fn install_binary_impl(
    _tmp_path: &Path,
    _dst_path: &Path,
    _assets: &[(String, String)],
    _version: &str,
) -> Result<bool> {
    Err(anyhow!(
        "{}",
        t!("cmd.upgrade.unsupported_platform", "unknown")
    ))
}

#[cfg(all(unix, not(target_os = "android")))]
fn ensure_path_export(bin_dir: &Path) -> Result<()> {
    use std::fs;

    let export_line = format!("export PATH=\"{}\":$PATH", bin_dir.display());
    let home = dirs::home_dir().context(t!("cmd.upgrade.home_fail").to_string())?;

    let shell = std::env::var("SHELL").unwrap_or_default();
    let mut candidates = vec![home.join(".bashrc"), home.join(".profile")];
    if shell.contains("zsh") {
        candidates.insert(0, home.join(".zshrc"));
    }
    if cfg!(target_os = "macos") {
        candidates.push(home.join(".zprofile"));
    }

    let existing: Vec<PathBuf> = candidates.into_iter().filter(|p| p.exists()).collect();
    let targets = if existing.is_empty() {
        vec![home.join(".bashrc")]
    } else {
        existing
    };

    for rc in &targets {
        let content = fs::read_to_string(rc).unwrap_or_default();
        if content.contains(&export_line) {
            continue;
        }
        let mut updated = content;
        if !updated.ends_with('\n') {
            updated.push('\n');
        }
        updated.push_str(&export_line);
        updated.push('\n');
        match fs::write(rc, updated) {
            Ok(_) => print_info(&t!("cmd.upgrade.path_exported", rc.display())),
            Err(e) => print_info(&format!("  (could not update {}: {})", rc.display(), e)),
        }
    }

    Ok(())
}

fn is_newer(latest: &str, current: &str) -> bool {
    let latest_parts: Vec<u32> = latest.split('.').filter_map(|s| s.parse().ok()).collect();
    let current_parts: Vec<u32> = current.split('.').filter_map(|s| s.parse().ok()).collect();

    for (l, c) in latest_parts.iter().zip(current_parts.iter()) {
        if l > c {
            return true;
        }
        if l < c {
            return false;
        }
    }

    latest_parts.len() > current_parts.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn windows_arch_key_mapping() {
        assert_eq!(windows_asset_key("x86_64"), Some("windows-x86_64"));
        assert_eq!(windows_asset_key("aarch64"), Some("windows-arm64"));
        assert_eq!(windows_asset_key("arm"), None);
        assert_eq!(windows_asset_key("i686"), None);
    }

    #[cfg(windows)]
    #[test]
    fn windows_asset_selection_picks_matching_arch_only() {
        let assets = vec![
            (
                "https://x/ntc-installer-2.4.2.exe".to_string(),
                "ntc-installer-2.4.2.exe".to_string(),
            ),
            (
                "https://x/ntc-v2.4.2-windows-arm64.zip".to_string(),
                "ntc-v2.4.2-windows-arm64.zip".to_string(),
            ),
            (
                "https://x/ntc-v2.4.2-windows-x86_64.zip".to_string(),
                "ntc-v2.4.2-windows-x86_64.zip".to_string(),
            ),
            (
                "https://x/ntc_2.4.2-1_amd64.deb".to_string(),
                "ntc_2.4.2-1_amd64.deb".to_string(),
            ),
        ];

        assert_eq!(
            pick_binary_asset(&assets, "windows-x86_64").as_deref(),
            Some("https://x/ntc-v2.4.2-windows-x86_64.zip")
        );
        assert_eq!(
            pick_binary_asset(&assets, "windows-arm64").as_deref(),
            Some("https://x/ntc-v2.4.2-windows-arm64.zip")
        );
        assert_eq!(pick_binary_asset(&assets, "windows-i686"), None);
        assert_eq!(
            pick_installer_asset(&assets).as_deref(),
            Some("https://x/ntc-installer-2.4.2.exe")
        );
    }

    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    #[test]
    fn linux_asset_selection_never_picks_android_or_other_arch() {
        let assets = vec![
            (
                "https://x/ntc-v2.4.2-aarch64-linux-android.tar.gz".to_string(),
                "ntc-v2.4.2-aarch64-linux-android.tar.gz".to_string(),
            ),
            (
                "https://x/ntc-v2.4.2-linux-x86_64.tar.gz".to_string(),
                "ntc-v2.4.2-linux-x86_64.tar.gz".to_string(),
            ),
            (
                "https://x/ntc-v2.4.2-macos-aarch64.tar.gz".to_string(),
                "ntc-v2.4.2-macos-aarch64.tar.gz".to_string(),
            ),
        ];

        assert_eq!(
            pick_binary_asset(&assets, "linux-x86_64").as_deref(),
            Some("https://x/ntc-v2.4.2-linux-x86_64.tar.gz")
        );
        assert_eq!(pick_binary_asset(&assets, "linux-aarch64"), None);
    }

    #[test]
    fn progress_pct_format_replaces_all_placeholders() {
        let pct = 50.0_f64;
        let downloaded: u64 = 12345678;
        let total: u64 = 24691356;
        let result = crate::t!("cmd.upgrade.progress_pct", pct, downloaded, total);
        assert!(
            !result.contains("{"),
            "template placeholder was not replaced: {result}"
        );
        assert!(
            result.contains("50"),
            "percentage value missing from output: {result}"
        );
    }

    #[test]
    fn progress_bytes_format_replaces_all_placeholders() {
        let downloaded: u64 = 12345678;
        let result = crate::t!("cmd.upgrade.progress_bytes", downloaded);
        assert!(
            !result.contains("{"),
            "template placeholder was not replaced: {result}"
        );
        assert!(
            result.contains("12345678"),
            "downloaded value missing from output: {result}"
        );
    }

    #[test]
    fn is_newer_recognizes_newer_version() {
        assert!(is_newer("2.5.0", "2.4.3"));
        assert!(is_newer("3.0.0", "2.9.9"));
        assert!(is_newer("2.4.4", "2.4.3"));
    }

    #[test]
    fn is_newer_rejects_older_or_equal_version() {
        assert!(!is_newer("2.4.3", "2.4.3"));
        assert!(!is_newer("2.4.2", "2.4.3"));
        assert!(!is_newer("1.9.9", "2.0.0"));
    }

    #[test]
    fn is_newer_handles_different_component_counts() {
        assert!(is_newer("2.4.3.1", "2.4.3"));
        assert!(!is_newer("2.4.3", "2.4.3.1"));
    }

    #[test]
    fn copy_fail_format_includes_error() {
        let result = crate::t!("cmd.upgrade.copy_fail", "Permission denied (os error 5)");
        assert!(
            result.contains("Permission denied"),
            "error message not included: {result}"
        );
        assert!(
            !result.contains("{}"),
            "unreplaced placeholder in output: {result}"
        );
    }

    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    #[test]
    fn linux_excludes_android_windows_macos_assets() {
        let assets = vec![
            (
                "https://x/ntc-v2.4.3-aarch64-linux-android.tar.gz".to_string(),
                "ntc-v2.4.3-aarch64-linux-android.tar.gz".to_string(),
            ),
            (
                "https://x/ntc-v2.4.3-linux-x86_64.tar.gz".to_string(),
                "ntc-v2.4.3-linux-x86_64.tar.gz".to_string(),
            ),
            (
                "https://x/ntc-v2.4.3-linux-aarch64.tar.gz".to_string(),
                "ntc-v2.4.3-linux-aarch64.tar.gz".to_string(),
            ),
            (
                "https://x/ntc-v2.4.3-windows-x86_64.zip".to_string(),
                "ntc-v2.4.3-windows-x86_64.zip".to_string(),
            ),
            (
                "https://x/ntc-v2.4.3-macos-aarch64.tar.gz".to_string(),
                "ntc-v2.4.3-macos-aarch64.tar.gz".to_string(),
            ),
        ];
        // Should pick linux-x86_64, not android/windows/macos
        assert_eq!(
            pick_binary_asset(&assets, "linux-x86_64").as_deref(),
            Some("https://x/ntc-v2.4.3-linux-x86_64.tar.gz")
        );
        // Should pick linux-aarch64, not android
        assert_eq!(
            pick_binary_asset(&assets, "linux-aarch64").as_deref(),
            Some("https://x/ntc-v2.4.3-linux-aarch64.tar.gz")
        );
    }

    #[cfg(windows)]
    #[test]
    fn windows_excludes_linux_android_macos_assets() {
        let assets = vec![
            (
                "https://x/ntc-v2.4.3-windows-x86_64.zip".to_string(),
                "ntc-v2.4.3-windows-x86_64.zip".to_string(),
            ),
            (
                "https://x/ntc-v2.4.3-linux-x86_64.tar.gz".to_string(),
                "ntc-v2.4.3-linux-x86_64.tar.gz".to_string(),
            ),
            (
                "https://x/ntc-v2.4.3-aarch64-linux-android.tar.gz".to_string(),
                "ntc-v2.4.3-aarch64-linux-android.tar.gz".to_string(),
            ),
            (
                "https://x/ntc-v2.4.3-macos-aarch64.tar.gz".to_string(),
                "ntc-v2.4.3-macos-aarch64.tar.gz".to_string(),
            ),
        ];
        assert_eq!(
            pick_binary_asset(&assets, "windows-x86_64").as_deref(),
            Some("https://x/ntc-v2.4.3-windows-x86_64.zip")
        );
    }
}
