use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success};
use crate::shell::commands::builtin::nssh::nssh::ssh as transport;
use crate::shell::commands::builtin::nssh::nssh::store;
use crate::t;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::time::Duration;

use super::auth::{build_auth, rt};

/// Split `<profile>:<path>`. Only a *known* profile name counts as remote
/// (so `C:\x` on Windows stays local).
pub(super) fn split_remote(spec: &str) -> Option<(String, String)> {
    let (maybe_name, path) = spec.split_once(':')?;
    if !store::valid_profile_name(maybe_name) {
        return None;
    }
    // Windows drive letter (`C:\...`, `C:/...`) is never remote.
    if maybe_name.len() == 1 && path.starts_with(['\\', '/']) {
        return None;
    }
    let profiles = store::list_profiles().ok()?;
    profiles
        .contains_key(maybe_name)
        .then(|| (maybe_name.to_string(), path.to_string()))
}

fn progress_bar(len: u64) -> ProgressBar {
    let bar = ProgressBar::new(len);
    bar.set_style(
        ProgressStyle::with_template("{wide_bar} {bytes}/{total_bytes} {msg}")
            .unwrap_or_else(|_| ProgressStyle::default_bar()),
    );
    bar
}

pub(super) fn cmd_cp(tokens: &[String], nav: &mut Navigator) -> Result<bool> {
    // nssh cp <src> <dst> [--trust] [--ask-pass] [--timeout <s>]
    let mut positional: Vec<String> = Vec::new();
    let mut trust = false;
    let mut _ask_pass = false;
    let mut timeout = Duration::from_secs(30);
    let mut i = 1;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "--trust" => {
                trust = true;
                i += 1;
            }
            "--ask-pass" => {
                _ask_pass = true;
                i += 1;
            }
            "--timeout" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.nssh.missing_timeout")))?;
                let s: u64 = v
                    .parse()
                    .map_err(|_| anyhow::anyhow!("{}", t!("cmd.nssh.invalid_timeout", v)))?;
                timeout = Duration::from_secs(s.max(1));
                i += 2;
            }
            _ => {
                if tokens[i].starts_with('-') {
                    print_error(&t!("cmd.nssh.unknown_option", tokens[i]));
                    return Ok(false);
                }
                positional.push(tokens[i].clone());
                i += 1;
            }
        }
    }
    if positional.len() != 2 {
        print_error(&t!("cmd.nssh.cp_usage"));
        return Ok(false);
    }
    let cwd = nav.current_path().to_path_buf();
    let resolve_local = |s: &str| -> PathBuf {
        let expanded = crate::utils::paths::expand_path_str(s);
        let p = Path::new(&expanded);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            cwd.join(p)
        }
    };
    let src_remote = split_remote(&positional[0]);
    let dst_remote = split_remote(&positional[1]);
    match (src_remote, dst_remote) {
        (None, None) => {
            print_error(&t!("cmd.nssh.cp_no_remote"));
            Ok(false)
        }
        (Some(_), Some(_)) => {
            print_error(&t!("cmd.nssh.cp_two_sides"));
            Ok(false)
        }
        (None, Some((name, rpath))) => {
            // upload local -> remote
            let local = resolve_local(&positional[0]);
            if !local.is_file() {
                print_error(&t!("cmd.nssh.cp_local_missing", local.display()));
                return Ok(false);
            }
            let profile = store::get_profile(&name)
                .map_err(|e| anyhow::anyhow!("{}", e))?
                .expect("profile checked by split_remote");
            let remote = expand_remote_path(&rpath, local.file_name().and_then(|n| n.to_str()));
            let auth = build_auth(&profile)?;
            let runtime = rt()?;
            runtime.block_on(cp_upload(&profile, &auth, &local, &remote, trust, timeout))?;
            Ok(false)
        }
        (Some((name, rpath)), None) => {
            // download remote -> local
            let profile = store::get_profile(&name)
                .map_err(|e| anyhow::anyhow!("{}", e))?
                .expect("profile checked by split_remote");
            let mut local = resolve_local(&positional[1]);
            if local.is_dir() || positional[1].ends_with(['/', '\\']) {
                if let Some(base) = Path::new(&rpath).file_name() {
                    local = local.join(base);
                }
            }
            let auth = build_auth(&profile)?;
            let runtime = rt()?;
            runtime.block_on(cp_download(&profile, &auth, &rpath, &local, trust, timeout))?;
            Ok(false)
        }
    }
}

/// `remote/` + local basename when the target is a directory.
pub(super) fn expand_remote_path(rpath: &str, basename: Option<&str>) -> String {
    if rpath.ends_with('/') {
        match basename {
            Some(b) => format!("{}{}", rpath, b),
            None => rpath.to_string(),
        }
    } else {
        rpath.to_string()
    }
}

const CP_SIZE_GUARD: u64 = 256 * 1024 * 1024;

async fn cp_upload(
    profile: &store::Profile,
    auth: &transport::SshAuth,
    local: &Path,
    remote: &str,
    trust: bool,
    timeout: Duration,
) -> Result<()> {
    let data =
        std::fs::read(local).map_err(|e| anyhow::anyhow!("{}", t!("cmd.nssh.cp_read_fail", e)))?;
    if data.len() as u64 > CP_SIZE_GUARD {
        return Err(anyhow::anyhow!("{}", t!("cmd.nssh.cp_too_large")));
    }
    print_info(&t!(
        "cmd.nssh.cp_uploading",
        local.display(),
        profile.host,
        remote
    ));
    let conn = transport::connect(
        &profile.host,
        profile.port,
        &profile.user,
        auth,
        trust,
        timeout,
    )
    .await?;
    let sftp = transport::sftp(&conn).await?;
    let bar = progress_bar(data.len() as u64);
    // Chunked write so the progress bar moves on big files.
    let mut f = sftp
        .create(remote.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("sftp create failed: {}", e))?;
    {
        use tokio::io::AsyncWriteExt as _;
        for chunk in data.chunks(256 * 1024) {
            f.write_all(chunk)
                .await
                .map_err(|e| anyhow::anyhow!("sftp write failed: {}", e))?;
            bar.inc(chunk.len() as u64);
        }
        f.shutdown()
            .await
            .map_err(|e| anyhow::anyhow!("sftp close failed: {}", e))?;
    }
    bar.finish_and_clear();
    print_success(&t!("cmd.nssh.cp_done", data.len()));
    Ok(())
}

async fn cp_download(
    profile: &store::Profile,
    auth: &transport::SshAuth,
    remote: &str,
    local: &Path,
    trust: bool,
    timeout: Duration,
) -> Result<()> {
    print_info(&t!(
        "cmd.nssh.cp_downloading",
        profile.host,
        remote,
        local.display()
    ));
    let conn = transport::connect(
        &profile.host,
        profile.port,
        &profile.user,
        auth,
        trust,
        timeout,
    )
    .await?;
    let sftp = transport::sftp(&conn).await?;
    let data = sftp
        .read(remote.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("sftp read failed: {}", e))?;
    if let Some(parent) = local.parent() {
        std::fs::create_dir_all(parent).map_err(|e| anyhow::anyhow!("{}", e))?;
    }
    std::fs::write(local, &data).map_err(|e| anyhow::anyhow!("{}", e))?;
    print_success(&t!("cmd.nssh.cp_done", data.len()));
    Ok(())
}
