// src/utils/git.rs
// Git integration helpers — all git access shells out to the system `git`
// binary (ntc's ntpkg can install one). Results are TTL-cached so the shell
// prompt and file browsers don't spawn `git` on every keystroke.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{LazyLock, Mutex, OnceLock};
use std::time::{Duration, Instant};

/// Parsed subset of `git status --porcelain=v1 --branch`.
#[derive(Debug, Clone, Default)]
pub struct GitStatus {
    pub branch: Option<String>,
    pub upstream: Option<String>,
    pub ahead: i64,
    pub behind: i64,
    /// (index code, path) — staged changes
    pub staged: Vec<(char, String)>,
    /// (worktree code, path) — unstaged changes
    pub unstaged: Vec<(char, String)>,
    pub untracked: Vec<String>,
    pub conflicts: Vec<String>,
}

impl GitStatus {
    pub fn is_dirty(&self) -> bool {
        !self.staged.is_empty()
            || !self.unstaged.is_empty()
            || !self.untracked.is_empty()
            || !self.conflicts.is_empty()
    }
}

struct CacheEntry {
    updated: Instant,
    status: Option<GitStatus>,
}

static STATUS_CACHE: LazyLock<Mutex<HashMap<PathBuf, CacheEntry>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

const CACHE_TTL: Duration = Duration::from_secs(2);

/// The system git binary, located once and cached.
fn git_binary() -> Option<&'static PathBuf> {
    static BIN: OnceLock<Option<PathBuf>> = OnceLock::new();
    BIN.get_or_init(|| which::which("git").ok()).as_ref()
}

/// Whether a `git` binary is available on PATH.
pub fn git_available() -> bool {
    git_binary().is_some()
}

/// Run git in `dir` and capture stdout. Returns None on any failure.
pub fn git_output(dir: &Path, args: &[&str]) -> Option<String> {
    let bin = git_binary()?;
    let out = Command::new(bin)
        .args(args)
        .current_dir(dir)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// True if `dir` is inside a git work tree.
pub fn is_repo(dir: &Path) -> bool {
    git_output(dir, &["rev-parse", "--is-inside-work-tree"])
        .map(|s| s.trim() == "true")
        .unwrap_or(false)
}

/// Current status for the repository containing `dir` (cached, TTL 2s).
pub fn status(dir: &Path) -> Option<GitStatus> {
    if let Ok(cache) = STATUS_CACHE.lock() {
        if let Some(e) = cache.get(dir) {
            if e.updated.elapsed() < CACHE_TTL {
                return e.status.clone();
            }
        }
    }

    let parsed = status_uncached(dir);
    if let Ok(mut cache) = STATUS_CACHE.lock() {
        cache.insert(
            dir.to_path_buf(),
            CacheEntry {
                updated: Instant::now(),
                status: parsed.clone(),
            },
        );
    }
    parsed
}

fn status_uncached(dir: &Path) -> Option<GitStatus> {
    let out = git_output(dir, &["status", "--porcelain=v1", "--branch"])?;
    Some(parse_status(&out))
}

/// Parse `git status --porcelain=v1 --branch` output.
pub fn parse_status(out: &str) -> GitStatus {
    let mut st = GitStatus::default();
    let mut lines = out.lines();

    if let Some(first) = lines.next() {
        if let Some(rest) = first.strip_prefix("## ") {
            parse_branch_line(rest, &mut st);
        }
    }

    for line in lines {
        if let Some(path) = line.strip_prefix("?? ") {
            st.untracked.push(path.trim().to_string());
            continue;
        }
        let mut chars = line.chars();
        let x = chars.next().unwrap_or(' ');
        let y = chars.next().unwrap_or(' ');
        // Skip the single space separator
        let _sep = chars.next();
        let path = chars.as_str().trim();

        if x == 'U' || y == 'U' || (x == 'A' && y == 'A') || (x == 'D' && y == 'D') {
            st.conflicts.push(path.to_string());
            continue;
        }
        if x != ' ' && x != '?' {
            let p = strip_rename_target(path);
            st.staged.push((x, p));
        }
        if y != ' ' && y != '?' {
            let p = strip_rename_target(path);
            st.unstaged.push((y, p));
        }
    }
    st
}

fn parse_branch_line(rest: &str, st: &mut GitStatus) {
    // Forms:
    //   ## main
    //   ## main...origin/main [ahead 2]
    //   ## main...origin/main [ahead 2, behind 1]
    //   ## No commits yet on main
    //   ## HEAD (no branch)
    let mut head = rest;
    let paren = head.find('[');
    if let Some(p) = paren {
        let tracking = &head[p + 1..];
        if let Some(close) = tracking.find(']') {
            let inside = &tracking[..close];
            for part in inside.split(',') {
                let part = part.trim();
                if let Some(n) = part.strip_prefix("ahead ") {
                    st.ahead = n.trim().parse().unwrap_or(0);
                } else if let Some(n) = part.strip_prefix("behind ") {
                    st.behind = n.trim().parse().unwrap_or(0);
                }
            }
        }
        head = &head[..p];
    }

    let head = head.trim();
    if let Some(b) = head.strip_prefix("No commits yet on ") {
        st.branch = Some(b.trim().to_string());
        return;
    }
    if head == "HEAD (no branch)" {
        st.branch = None;
        return;
    }

    if let Some(sep) = head.find("...") {
        st.branch = Some(head[..sep].to_string());
        st.upstream = Some(head[sep + 3..].to_string());
    } else {
        st.branch = Some(head.to_string());
    }
}

/// Rename lines are `XY old -> new`; return the target path.
fn strip_rename_target(path: &str) -> String {
    if let Some(pos) = path.find(" -> ") {
        path[pos + 4..].to_string()
    } else {
        path.to_string()
    }
}

/// Plain-text prompt fragment like `(main*)` or `(main)` or empty.
pub fn prompt_fragment(dir: &Path) -> String {
    let Some(st) = status(dir) else {
        return String::new();
    };
    let Some(branch) = &st.branch else {
        return String::new();
    };
    let dirty = if st.is_dirty() { "*" } else { "" };
    format!("({branch}{dirty})")
}

/// Absolute path -> single status char for the browsers/editor.
/// Priority: conflict > staged > unstaged > untracked.
pub fn status_map(dir: &Path) -> HashMap<PathBuf, char> {
    let mut map: HashMap<PathBuf, char> = HashMap::new();
    let Some(st) = status(dir) else { return map };

    for p in &st.untracked {
        map.insert(resolve(dir, p), '?');
    }
    for (c, p) in &st.unstaged {
        map.insert(resolve(dir, p), *c);
    }
    for (c, p) in &st.staged {
        map.insert(resolve(dir, p), *c);
    }
    for p in &st.conflicts {
        map.insert(resolve(dir, p), 'U');
    }
    map
}

fn resolve(dir: &Path, rel: &str) -> PathBuf {
    let p = Path::new(rel);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        dir.join(p)
    }
}

/// Invalidate the status cache so the next query re-runs `git status`.
pub fn invalidate() {
    if let Ok(mut cache) = STATUS_CACHE.lock() {
        cache.clear();
    }
}

/// `git diff` output for a single file (empty when the file is unchanged).
pub fn diff_of_file(dir: &Path, path: &Path) -> Option<String> {
    git_output(dir, &["diff", "--", &path.to_string_lossy()])
}

pub struct BlameInfo {
    pub commit: String,
    pub author: String,
    pub time: String,
    pub summary: String,
}

/// Blame info for one line of `path` (1-indexed line number).
pub fn blame_line(dir: &Path, path: &Path, line: usize) -> Option<BlameInfo> {
    let range = format!("{line},{line}");
    let out = git_output(
        dir,
        &[
            "blame",
            "--line-porcelain",
            "-L",
            &range,
            "--",
            &path.to_string_lossy(),
        ],
    )?;

    let mut info = BlameInfo {
        commit: String::new(),
        author: String::new(),
        time: String::new(),
        summary: String::new(),
    };
    let mut lines = out.lines();
    if let Some(first) = lines.next() {
        let hash = first.split_whitespace().next().unwrap_or("").to_string();
        if !hash.is_empty() && !hash.starts_with('^') {
            info.commit = hash.chars().take(7).collect();
        } else if !hash.is_empty() {
            info.commit = format!("~{}", hash[1..].chars().take(7).collect::<String>());
        }
    }
    for l in lines {
        if let Some(v) = l.strip_prefix("author ") {
            info.author = v.to_string();
        } else if let Some(v) = l.strip_prefix("author-time ") {
            if let Ok(secs) = v.trim().parse::<i64>() {
                use chrono::{DateTime, Utc};
                let dt: DateTime<Utc> = DateTime::from_timestamp(secs, 0).unwrap_or_default();
                info.time = dt.format("%Y-%m-%d %H:%M").to_string();
            }
        } else if let Some(v) = l.strip_prefix("summary ") {
            info.summary = v.to_string();
        }
    }
    if info.commit.is_empty() {
        None
    } else {
        Some(info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_plain_branch() {
        let out = "## main\n?? new.txt\n";
        let st = parse_status(out);
        assert_eq!(st.branch.as_deref(), Some("main"));
        assert_eq!(st.untracked, vec!["new.txt".to_string()]);
        assert!(st.is_dirty());
    }

    #[test]
    fn parse_branch_with_tracking() {
        let out = "## main...origin/main [ahead 2]\n";
        let st = parse_status(out);
        assert_eq!(st.branch.as_deref(), Some("main"));
        assert_eq!(st.upstream.as_deref(), Some("origin/main"));
        assert_eq!(st.ahead, 2);
    }

    #[test]
    fn parse_clean_branch_not_dirty() {
        let out = "## feature\n";
        let st = parse_status(out);
        assert_eq!(st.branch.as_deref(), Some("feature"));
        assert!(!st.is_dirty());
    }

    #[test]
    fn parse_staged_unstaged() {
        let out = "## main\nM  modified.rs\n M worktree.rs\n";
        let st = parse_status(out);
        assert_eq!(st.staged, vec![('M', "modified.rs".to_string())]);
        assert_eq!(st.unstaged, vec![('M', "worktree.rs".to_string())]);
    }

    #[test]
    fn parse_rename_target() {
        let out = "## main\nR  old.rs -> new.rs\n";
        let st = parse_status(out);
        assert_eq!(st.staged, vec![('R', "new.rs".to_string())]);
    }

    #[test]
    fn parse_conflict() {
        let out = "## main\nUU both.rs\n";
        let st = parse_status(out);
        assert_eq!(st.conflicts, vec!["both.rs".to_string()]);
        assert!(st.is_dirty());
    }

    #[test]
    fn parse_detached_head() {
        let out = "## HEAD (no branch)\n";
        let st = parse_status(out);
        assert_eq!(st.branch, None);
    }

    #[test]
    fn parse_no_commits_yet() {
        let out = "## No commits yet on main\n";
        let st = parse_status(out);
        assert_eq!(st.branch.as_deref(), Some("main"));
    }

    #[test]
    fn prompt_fragment_dirty_and_clean() {
        assert_eq!(prompt_fragment_from(&parse_status("## main\n")), "(main)");
        assert_eq!(
            prompt_fragment_from(&parse_status("## main\nM  x.rs\n")),
            "(main*)"
        );
        assert_eq!(
            prompt_fragment_from(&parse_status("## HEAD (no branch)\n")),
            ""
        );
    }

    fn prompt_fragment_from(st: &GitStatus) -> String {
        let Some(branch) = &st.branch else {
            return String::new();
        };
        let dirty = if st.is_dirty() { "*" } else { "" };
        format!("({branch}{dirty})")
    }

    #[test]
    fn status_map_prefers_conflict() {
        let dir = Path::new("/tmp/repo");
        let mut st = GitStatus::default();
        st.staged.push(('M', "a.rs".to_string()));
        st.unstaged.push(('M', "b.rs".to_string()));
        st.untracked.push("c.rs".to_string());
        st.conflicts.push("a.rs".to_string());

        let map = status_map_from(dir, &st);
        assert_eq!(map.get(&PathBuf::from("/tmp/repo/a.rs")), Some(&'U'));
        assert_eq!(map.get(&PathBuf::from("/tmp/repo/b.rs")), Some(&'M'));
        assert_eq!(map.get(&PathBuf::from("/tmp/repo/c.rs")), Some(&'?'));
    }

    fn status_map_from(dir: &Path, st: &GitStatus) -> HashMap<PathBuf, char> {
        let mut map: HashMap<PathBuf, char> = HashMap::new();
        for p in &st.untracked {
            map.insert(resolve(dir, p), '?');
        }
        for (c, p) in &st.unstaged {
            map.insert(resolve(dir, p), *c);
        }
        for (c, p) in &st.staged {
            map.insert(resolve(dir, p), *c);
        }
        for p in &st.conflicts {
            map.insert(resolve(dir, p), 'U');
        }
        map
    }
}
