use crate::output::{print_error, print_info, print_success};
use crate::t;
use anyhow::{anyhow, Context, Result};
use std::io::Read;

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

    let tmp_dir = std::env::temp_dir().join("ntc_update");
    std::fs::create_dir_all(&tmp_dir).context(t!("cmd.upgrade.tmpdir_fail").to_string())?;

    let tmp_name = if cfg!(windows) {
        "ntc_new.exe"
    } else {
        "ntc_new"
    };
    let tmp_exe = tmp_dir.join(tmp_name);

    if let Err(e) = download_release_binary(&latest, &tmp_exe) {
        print_error(&t!("cmd.upgrade.download_fail", e));
        return Ok(false);
    }

    print_success(&t!("cmd.upgrade.download_complete"));

    if let Err(e) = install_binary(&tmp_exe, &exe_path, &latest) {
        print_error(&t!("cmd.upgrade.install_fail", e));

        let hint = platform_install_hint(&exe_path);
        if !hint.is_empty() {
            print_info(&hint);
        }
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

fn download_release_binary(version: &str, dest: &std::path::Path) -> Result<()> {
    let asset_urls = get_asset_urls(version)?;

    if asset_urls.is_empty() {
        return Err(anyhow!("{}", t!("cmd.upgrade.no_assets", version)));
    }

    let url = &asset_urls[0];
    print_info(&t!("cmd.upgrade.downloading_url", url));

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

    std::fs::write(dest, &buf).context(t!("cmd.upgrade.write_fail").to_string())?;
    Ok(())
}

fn get_asset_urls(version: &str) -> Result<Vec<String>> {
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

    let target_hint = platform_asset_hint();

    let mut candidates: Vec<String> = Vec::new();
    let mut fallbacks: Vec<String> = Vec::new();

    for asset in assets {
        let name = asset["name"].as_str().unwrap_or("");
        let browser_url = asset["browser_download_url"].as_str().unwrap_or("");

        if browser_url.is_empty() {
            continue;
        }

        let lower = name.to_lowercase();

        if target_hint.iter().any(|h| lower.contains(h)) {
            candidates.push(browser_url.to_string());
        } else if !lower.contains("source")
            && !lower.contains("tar.gz")
            && !lower.contains("tar.xz")
        {
            fallbacks.push(browser_url.to_string());
        }
    }

    if !candidates.is_empty() {
        return Ok(candidates);
    }

    if !fallbacks.is_empty() {
        return Ok(fallbacks);
    }

    for asset in assets {
        if let Some(url) = asset["browser_download_url"].as_str() {
            candidates.push(url.to_string());
        }
    }

    Ok(candidates)
}

fn platform_asset_hint() -> Vec<&'static str> {
    #[cfg(target_os = "windows")]
    {
        vec!["windows", ".exe"]
    }
    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    {
        vec!["linux", "unknown-linux"]
    }
    #[cfg(target_os = "android")]
    {
        vec!["android", "linux-android"]
    }
    #[cfg(target_os = "macos")]
    {
        vec!["darwin", "apple-darwin", "macos"]
    }
    #[cfg(not(any(
        target_os = "windows",
        target_os = "linux",
        target_os = "android",
        target_os = "macos"
    )))]
    {
        vec![]
    }
}

fn platform_install_hint(_exe_path: &std::path::Path) -> String {
    #[cfg(target_os = "windows")]
    {
        String::new()
    }
    #[cfg(all(unix, not(target_os = "android")))]
    {
        t!("cmd.upgrade.sudo_hint", _exe_path.display())
    }
    #[cfg(target_os = "android")]
    {
        String::new()
    }
    #[cfg(not(any(unix, windows)))]
    {
        String::new()
    }
}

fn install_binary(
    tmp_path: &std::path::Path,
    dst_path: &std::path::Path,
    version: &str,
) -> Result<bool> {
    let result = install_binary_impl(tmp_path, dst_path);

    if result.is_ok() {
        println!();
        print_success(&t!("cmd.upgrade.updated", version));
        print_info(&t!("cmd.upgrade.restart"));
        println!();
    }

    result
}

#[cfg(windows)]
fn install_binary_impl(tmp_path: &std::path::Path, dst_path: &std::path::Path) -> Result<bool> {
    let ps_script = format!(
        r#"Start-Sleep -Seconds 1; try {{ Copy-Item -LiteralPath '{src}' -Destination '{dst}' -Force; Write-Host 'Update complete' }} catch {{ Write-Host 'Update failed: $_' }}"#,
        src = tmp_path.display(),
        dst = dst_path.display(),
    );

    let _ = std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-WindowStyle",
            "Hidden",
            "-Command",
            &ps_script,
        ])
        .spawn();

    print_info(&t!("cmd.upgrade.after_restart"));
    println!();

    std::process::exit(0);
}

#[cfg(all(unix, not(target_os = "android")))]
fn install_binary_impl(tmp_path: &std::path::Path, dst_path: &std::path::Path) -> Result<bool> {
    use std::os::unix::fs::PermissionsExt;

    std::fs::copy(tmp_path, dst_path).context(t!("cmd.upgrade.copy_fail").to_string())?;

    let perms = std::fs::Permissions::from_mode(0o755);
    std::fs::set_permissions(dst_path, perms).context(t!("cmd.upgrade.perm_fail").to_string())?;

    Ok(false)
}

#[cfg(target_os = "android")]
fn install_binary_impl(tmp_path: &std::path::Path, dst_path: &std::path::Path) -> Result<bool> {
    use std::os::unix::fs::PermissionsExt;

    std::fs::copy(tmp_path, dst_path).context(t!("cmd.upgrade.copy_fail_android").to_string())?;

    let perms = std::fs::Permissions::from_mode(0o755);
    std::fs::set_permissions(dst_path, perms).context(t!("cmd.upgrade.perm_fail").to_string())?;

    Ok(false)
}

#[cfg(not(any(unix, windows)))]
fn install_binary_impl(_tmp_path: &std::path::Path, _dst_path: &std::path::Path) -> Result<bool> {
    Err(anyhow!("{}", t!("cmd.upgrade.unsupported_platform")))
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
