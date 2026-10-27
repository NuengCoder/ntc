use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.upgrade.after_restart".into(),
        "The new version will be ready after restart.".into(),
    );

    m.insert(
        "cmd.upgrade.ahead".into(),
        "Local version ({}) is ahead of latest release ({}).".into(),
    );

    m.insert(
        "cmd.upgrade.already_latest".into(),
        "ntc {} is already the latest version.".into(),
    );

    m.insert(
        "cmd.upgrade.api_parse_fail".into(),
        "Failed to parse GitHub API response".into(),
    );

    m.insert(
        "cmd.upgrade.api_read_fail".into(),
        "Failed to read GitHub API response".into(),
    );

    m.insert(
        "cmd.upgrade.api_request_fail".into(),
        "GitHub API request failed: {}".into(),
    );

    m.insert(
        "cmd.upgrade.check_fail".into(),
        "Failed to check for updates: {}".into(),
    );

    m.insert(
        "cmd.upgrade.checking".into(),
        "Checking for updates (current version: {})...".into(),
    );

    m.insert(
        "cmd.upgrade.copy_fail".into(),
        "Failed to copy binary: {}".into(),
    );

    m.insert(
        "cmd.upgrade.copy_fail_android".into(),
        "Failed to copy binary. Check disk space or permissions.".into(),
    );

    m.insert(
        "cmd.upgrade.dev_build".into(),
        "You may be running a development build.".into(),
    );

    m.insert(
        "cmd.upgrade.download_complete".into(),
        "Download complete. Installing update...".into(),
    );

    m.insert(
        "cmd.upgrade.download_fail".into(),
        "Download failed: {}".into(),
    );

    m.insert(
        "cmd.upgrade.downloading".into(),
        "Downloading update...".into(),
    );

    m.insert(
        "cmd.upgrade.downloading_url".into(),
        "Downloading: {}".into(),
    );

    m.insert(
        "cmd.upgrade.exe_path_fail".into(),
        "Failed to get executable path: {}".into(),
    );

    m.insert(
        "cmd.upgrade.install_fail".into(),
        "Installation failed: {}".into(),
    );

    m.insert(
        "cmd.upgrade.missing_tag".into(),
        "Missing 'tag_name' in GitHub release response".into(),
    );

    m.insert(
        "cmd.upgrade.new_version".into(),
        "New version available: ntc {}".into(),
    );

    m.insert(
        "cmd.upgrade.no_assets_release".into(),
        "No assets found in release v{}".into(),
    );

    m.insert(
        "cmd.upgrade.perm_fail".into(),
        "Failed to set executable permissions: {}".into(),
    );

    m.insert(
        "cmd.upgrade.progress_bytes".into(),
        "Downloaded: {} bytes".into(),
    );

    m.insert("cmd.upgrade.progress_pct".into(), "Progress: {}%".into());

    m.insert(
        "cmd.upgrade.restart".into(),
        "Restart ntc to use the new version.".into(),
    );

    m.insert(
        "cmd.upgrade.stream_fail".into(),
        "Failed to read download stream".into(),
    );

    m.insert(
        "cmd.upgrade.sudo_hint".into(),
        "Run with sudo/administrator privileges".into(),
    );

    m.insert(
        "cmd.upgrade.time_remaining".into(),
        "Time remaining: {:>3} seconds  ".into(),
    );

    m.insert(
        "cmd.upgrade.tmpdir_fail".into(),
        "Failed to create temp directory: {}".into(),
    );

    m.insert(
        "cmd.upgrade.unsupported_platform".into(),
        "Unsupported platform: {}".into(),
    );

    m.insert(
        "cmd.upgrade.updated".into(),
        "ntc has been updated to v{}!".into(),
    );

    m.insert(
        "cmd.upgrade.write_fail".into(),
        "Failed to write downloaded file".into(),
    );

    m.insert("cmd.upgrade.no_arch_asset".into(), "No prebuilt ntc binary for this architecture ({}). Build from source: cargo build --release".into());

    m.insert(
        "cmd.upgrade.no_asset_match".into(),
        "No release asset matches platform key '{}'".into(),
    );

    m.insert(
        "cmd.upgrade.extracting".into(),
        "Extracting binary...".into(),
    );

    m.insert(
        "cmd.upgrade.extract_fail".into(),
        "Failed to read/parse downloaded archive: {}".into(),
    );

    m.insert(
        "cmd.upgrade.no_file_in_archive".into(),
        "Could not find '{}' inside the downloaded archive".into(),
    );

    m.insert(
        "cmd.upgrade.no_installer_asset".into(),
        "Could not find a Windows installer asset to update ntc".into(),
    );

    m.insert(
        "cmd.upgrade.installer_launch".into(),
        "Launching the ntc installer (silent)...".into(),
    );

    m.insert(
        "cmd.upgrade.installer_launch_fail".into(),
        "Failed to launch the installer: {}".into(),
    );

    m.insert(
        "cmd.upgrade.home_fail".into(),
        "Could not determine your home directory".into(),
    );

    m.insert(
        "cmd.upgrade.installed_user_dir".into(),
        "ntc installed to {} (user install, no admin/sudo required).".into(),
    );

    m.insert(
        "cmd.upgrade.path_exported".into(),
        "PATH updated in {}".into(),
    );
}
