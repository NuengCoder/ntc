//! GitHub repo metadata, codeload tarball download and a guarded extractor.
//!
//! Nothing downloaded here is ever executed: the bytes are unpacked into the
//! library as text/assets only, and archive entries that try to escape the
//! destination (absolute paths, `..`, symlinks) are rejected or skipped.

use anyhow::{bail, Context, Result};
use std::io::Read;
use std::path::{Component, Path};

const API_REPO: &str = "https://api.github.com/repos";
const CODELOAD: &str = "https://codeload.github.com";
const USER_AGENT: &str = "ntc-skill";
pub const MAX_TARBALL_BYTES: u64 = 50 * 1024 * 1024;
const MAX_ENTRIES: usize = 20_000;
const MAX_FILE_BYTES: u64 = 8 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct Fetched {
    pub used_ref: String,
    pub commit: Option<String>,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct RepoMeta {
    pub default_branch: Option<String>,
    pub commit: Option<String>,
}

fn read_limited(mut response: ureq::Body, url: &str) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut reader = response.as_reader();
    let mut chunk = vec![0u8; 64 * 1024];
    loop {
        let n = reader
            .read(&mut chunk)
            .with_context(|| format!("read {url}"))?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
        if buf.len() as u64 > MAX_TARBALL_BYTES {
            bail!(
                "download from {} is too large (limit {} MB)",
                url,
                MAX_TARBALL_BYTES / (1024 * 1024)
            );
        }
    }
    Ok(buf)
}

/// Repository metadata; failures are not fatal (callers fall back to `main`).
pub fn repo_meta(owner: &str, repo: &str) -> RepoMeta {
    let url = format!("{API_REPO}/{owner}/{repo}");
    let raw = (|| -> Result<String> {
        let mut response = ureq::get(&url)
            .header("User-Agent", USER_AGENT)
            .header("Accept", "application/vnd.github+json")
            .call()
            .with_context(|| format!("request {url}"))?;
        response
            .body_mut()
            .read_to_string()
            .with_context(|| format!("read {url}"))
    })();
    let Ok(raw) = raw else {
        return RepoMeta::default();
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&raw) else {
        return RepoMeta::default();
    };
    RepoMeta {
        default_branch: value
            .get("default_branch")
            .and_then(|v| v.as_str())
            .map(str::to_string),
        commit: value
            .get("commit")
            .and_then(|c| c.get("sha"))
            .and_then(|v| v.as_str())
            .map(str::to_string),
    }
}

fn download_ref(owner: &str, repo: &str, git_ref: &str) -> Result<Vec<u8>> {
    let mut last = None;
    for kind in ["heads", "tags"] {
        let url = format!("{CODELOAD}/{owner}/{repo}/tar.gz/refs/{kind}/{git_ref}");
        let attempt = (|| -> Result<Vec<u8>> {
            let response = ureq::get(&url)
                .header("User-Agent", USER_AGENT)
                .call()
                .with_context(|| format!("download {url}"))?;
            read_limited(response.into_body(), &url)
        })();
        match attempt {
            Ok(bytes) => return Ok(bytes),
            Err(e) => last = Some(e),
        }
    }
    Err(last.unwrap_or_else(|| anyhow::anyhow!("could not download {owner}/{repo} at '{git_ref}'")))
}

/// Download the repo tarball. Tries the requested ref as a branch then a tag;
/// without an explicit ref it tries the default branch, then `main`/`master`.
pub fn fetch_skills(owner: &str, repo: &str, wanted_ref: Option<&str>) -> Result<Fetched> {
    let meta = repo_meta(owner, repo);
    let mut refs: Vec<String> = Vec::new();
    if let Some(wanted) = wanted_ref {
        refs.push(wanted.to_string());
    } else if let Some(branch) = meta.default_branch.clone() {
        refs.push(branch);
    }
    for fallback in ["main", "master"] {
        if !refs.iter().any(|r| r == fallback) {
            refs.push(fallback.to_string());
        }
    }

    let mut last = None;
    for git_ref in &refs {
        match download_ref(owner, repo, git_ref) {
            Ok(bytes) => {
                let commit = if Some(git_ref.as_str()) == meta.default_branch.as_deref() {
                    meta.commit.clone()
                } else {
                    None
                };
                return Ok(Fetched {
                    used_ref: git_ref.clone(),
                    commit,
                    bytes,
                });
            }
            Err(e) => last = Some(e),
        }
    }
    Err(last.unwrap_or_else(|| anyhow::anyhow!("could not download {owner}/{repo}")))
}

fn safe_entry_path(path: &Path) -> Result<()> {
    if path.is_absolute() {
        bail!("archive entry '{}' uses an absolute path", path.display());
    }
    for component in path.components() {
        match component {
            Component::ParentDir => {
                bail!("archive entry '{}' escapes the destination", path.display())
            }
            Component::RootDir | Component::Prefix(_) => {
                bail!("archive entry '{}' uses an absolute path", path.display())
            }
            _ => {}
        }
    }
    Ok(())
}

/// Extract a `.tar.gz` with traversal guards. Symlinks and hard links are
/// skipped, oversized files are dropped, and the entry count is capped.
pub fn extract_tar_gz(bytes: &[u8], dest: &Path) -> Result<()> {
    use flate2::read::GzDecoder;

    std::fs::create_dir_all(dest).with_context(|| format!("create {}", dest.display()))?;
    let decoder = GzDecoder::new(bytes);
    let mut archive = tar::Archive::new(decoder);
    let entries = archive.entries().context("read skill tarball")?;

    let mut count = 0usize;
    for entry in entries {
        let mut entry = entry.context("read skill tarball entry")?;
        count += 1;
        if count > MAX_ENTRIES {
            bail!("archive has too many entries (limit {MAX_ENTRIES})");
        }
        let path = entry.path().context("archive entry path")?.into_owned();
        safe_entry_path(&path)?;

        let kind = entry.header().entry_type();
        if kind.is_symlink() || kind.is_hard_link() {
            continue;
        }
        if kind.is_dir() {
            std::fs::create_dir_all(dest.join(&path))?;
            continue;
        }
        if !kind.is_file() {
            continue;
        }
        if entry.size() > MAX_FILE_BYTES {
            continue;
        }
        let target = dest.join(&path);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)?;
        }
        entry
            .unpack(&target)
            .with_context(|| format!("extract {}", path.display()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a tarball with raw header bytes: `append_data` refuses `..` and
    /// absolute names, but the extractor has to defend against exactly those.
    fn archive(entries: &[(&str, Option<&str>)]) -> Vec<u8> {
        let gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        let mut builder = tar::Builder::new(gz);
        for (name, link) in entries {
            let mut header = tar::Header::new_gnu();
            header.set_size(if link.is_some() { 0 } else { 4 });
            header.set_mode(0o644);
            header.set_entry_type(if link.is_some() {
                tar::EntryType::Symlink
            } else {
                tar::EntryType::Regular
            });
            {
                let bytes = header.as_mut_bytes();
                let name_bytes = name.as_bytes();
                let n = name_bytes.len().min(100);
                bytes[..n].copy_from_slice(&name_bytes[..n]);
                if let Some(target) = link {
                    let link_bytes = target.as_bytes();
                    let n = link_bytes.len().min(100);
                    bytes[157..157 + n].copy_from_slice(&link_bytes[..n]);
                }
            }
            header.set_cksum();
            if link.is_some() {
                builder.append(&header, &[][..]).expect("append symlink");
            } else {
                builder.append(&header, &b"data"[..]).expect("append file");
            }
        }
        builder
            .into_inner()
            .expect("finish tar")
            .finish()
            .expect("finish gz")
    }

    #[test]
    fn extracts_regular_files() {
        let dir = tempfile::tempdir().unwrap();
        let bytes = archive(&[("repo-main/skills/demo/SKILL.md", None)]);
        extract_tar_gz(&bytes, dir.path()).unwrap();
        assert!(dir.path().join("repo-main/skills/demo/SKILL.md").is_file());
    }

    #[test]
    fn rejects_parent_dir_entries() {
        let dir = tempfile::tempdir().unwrap();
        let bytes = archive(&[("../escape.txt", None)]);
        let err = extract_tar_gz(&bytes, dir.path()).unwrap_err().to_string();
        assert!(err.contains("escapes"), "{err}");
        assert!(!dir.path().parent().unwrap().join("escape.txt").exists());
    }

    #[test]
    fn rejects_absolute_entries() {
        let dir = tempfile::tempdir().unwrap();
        let bytes = archive(&[("/tmp/escape.txt", None)]);
        let err = extract_tar_gz(&bytes, dir.path()).unwrap_err().to_string();
        assert!(err.contains("absolute"), "{err}");
    }

    #[test]
    fn skips_symlink_entries() {
        let dir = tempfile::tempdir().unwrap();
        let bytes = archive(&[("repo-main/evil", Some("/etc/passwd"))]);
        extract_tar_gz(&bytes, dir.path()).unwrap();
        assert!(!dir.path().join("repo-main/evil").exists());
    }
}
