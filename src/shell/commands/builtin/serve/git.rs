//! `serve git` — browser UI for the repository the command was started in.
//!
//! Like `serve arc`, a set of embedded assets (no build step) with a local API
//! on top of the system `git` binary. Every action returns the equivalent
//! `git` command, so the CLI stays the source of truth.
//!
//! The page is a hash-routed dashboard: a left rail of tabs, a content pane and
//! an inspector column. Assets live in `assets/git/` and are served from an
//! allowlist of `include_str!` constants, never from disk, so there is no
//! traversal surface.
//!
//! Size note: the route table stays in this one file because each endpoint maps
//! one-to-one to a `git` invocation and shares the same repository/state
//! guards; the presentation layer is what got split up.

use crate::config::{Config, ServeService};
use crate::navigator::Navigator;
use crate::output::{print_error, print_info};
use crate::shell::commands::support::args::is_help_request;
use crate::shell::commands::support::net::http::{respond, respond_no_store, url_decode};
use crate::shell::commands::support::net::tokenize;
use crate::t;
use crate::utils::git::{self, GitStatus};
use crate::utils::theme::style::ThemeColorize;
use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

const MAX_BODY: usize = 1024 * 1024;
const MAX_DIFF: usize = 200_000;
const MAX_MESSAGE: usize = 4096;
const LOG_LIMIT: usize = 30;
const INCOMING_LIMIT: usize = 30;
const PAGE: &str = include_str!("assets/git/index.html");
const INFO_PAGE: &str = include_str!("assets/git_info.html");
const APP_CSS: &str = include_str!("assets/git/app.css");
const APP_JS: &str = include_str!("assets/git/app.js");
const DIFF_JS: &str = include_str!("assets/git/diff.js");

/// Allowlist of tab modules. Every `tabs.register` call in the page has an
/// entry here, and nothing else is reachable.
const TAB_ASSETS: &[(&str, &str)] = &[
    ("tabs/repository.js", include_str!("assets/git/tabs/repository.js")),
    ("tabs/changes.js", include_str!("assets/git/tabs/changes.js")),
    ("tabs/history.js", include_str!("assets/git/tabs/history.js")),
    ("tabs/branches.js", include_str!("assets/git/tabs/branches.js")),
    ("tabs/remotes.js", include_str!("assets/git/tabs/remotes.js")),
    ("tabs/stash.js", include_str!("assets/git/tabs/stash.js")),
    ("tabs/tags.js", include_str!("assets/git/tabs/tags.js")),
    ("tabs/actions.js", include_str!("assets/git/tabs/actions.js")),
];

const JS_TYPE: &str = "text/javascript; charset=utf-8";

pub fn cmd_serve_git(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if is_help_request(&tokens) {
        print_git_help();
        return Ok(false);
    }

    // Defaults come from `serve config git` (alias: `config serve git`).
    let mut port = Config::global_get_serve_port(ServeService::Git);
    let mut bind = Config::global_get_serve_bind(ServeService::Git);
    let mut dir_arg: Option<String> = None;
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "-p" | "--port" => {
                i += 1;
                port = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.serve.missing_port")))?
                    .parse()
                    .map_err(|_| anyhow::anyhow!("{}", t!("cmd.serve.invalid_port")))?;
            }
            "-b" | "--bind" => {
                i += 1;
                bind = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.serve.missing_bind")))?
                    .clone();
            }
            other if other.starts_with('-') => {
                anyhow::bail!("{}", t!("cmd.serve.unknown_option", other));
            }
            other => {
                if dir_arg.is_some() {
                    anyhow::bail!("{}", t!("cmd.serve.unknown_option", other));
                }
                dir_arg = Some(other.to_string());
            }
        }
        i += 1;
    }

    let start = match dir_arg {
        Some(d) => {
            let p = PathBuf::from(&d);
            if p.is_absolute() {
                p
            } else {
                nav.current_path().join(p)
            }
        }
        None => nav.current_path().to_path_buf(),
    };
    let start = start
        .canonicalize()
        .with_context(|| t!("cmd.serve.git_no_repo", start.display()))?;
    if !start.is_dir() {
        anyhow::bail!("{}", t!("cmd.serve.git_no_repo", display_path(&start)));
    }
    if !git::git_available() {
        anyhow::bail!("{}", t!("cmd.serve.git_no_git"));
    }
    let root = git::repo_root(&start)
        .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.serve.git_no_repo", display_path(&start))))?;
    let root = root.canonicalize().unwrap_or(root);

    let addr = format!("{}:{}", bind, port);
    let listener = TcpListener::bind(&addr).context(format!("Bind to {}", addr))?;
    print_info(&t!(
        "cmd.serve.git_serving",
        display_path(&root),
        format!("http://{}", addr)
    ));
    if !is_loopback(&bind) {
        print_info(&t!("cmd.serve.git_local_warning"));
    }

    let shutdown = Arc::new(AtomicBool::new(false));
    crate::shell::commands::init_ctrlc_handler();
    crate::shell::commands::register_shutdown_flag(shutdown.clone());
    listener.set_nonblocking(true)?;
    let root = Arc::new(root);
    while !shutdown.load(std::sync::atomic::Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => {
                let root = root.clone();
                std::thread::spawn(move || handle(stream, &root));
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
            Err(e) => print_error(&format!("Accept: {}", e)),
        }
    }
    print_info(&t!("cmd.serve.stopped"));
    crate::shell::commands::unregister_shutdown_flag(&shutdown);
    crate::shell::commands::init_ctrlc_handler();
    Ok(false)
}

// ── Connection ────────────────────────────────────────────────────────────

fn handle(mut stream: TcpStream, root: &Path) {
    let _ = stream.set_nonblocking(false);
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }
    let request_line = request_line.trim_end().to_string();
    let mut parts = request_line.split_whitespace();
    let Some(method) = parts.next().map(|s| s.to_string()) else {
        return;
    };
    let Some(target) = parts.next().map(|s| s.to_string()) else {
        return;
    };
    let (raw_path, query) = target.split_once('?').unwrap_or((&target, ""));

    let mut content_length = 0usize;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() {
            return;
        }
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            if k.trim().eq_ignore_ascii_case("content-length") {
                content_length = v.trim().parse().unwrap_or(0);
            }
        }
    }
    if content_length > MAX_BODY {
        drop(reader);
        json_resp(
            &mut stream,
            413,
            json!({ "error": "request body too large" }),
        );
        return;
    }
    let mut body = vec![0u8; content_length];
    if content_length > 0 && reader.read_exact(&mut body).is_err() {
        return;
    }
    drop(reader);

    route(&mut stream, &method, raw_path, query, &body, root);
}

fn route(
    stream: &mut TcpStream,
    method: &str,
    raw_path: &str,
    query: &str,
    body: &[u8],
    root: &Path,
) {
    let path = url_decode(raw_path);
    match (method, path.as_str()) {
        ("GET", "/") | ("GET", "/index.html") => {
            respond(stream, 200, "text/html; charset=utf-8", PAGE.as_bytes());
        }
        ("GET", "/app.css") => respond_no_store(stream, "text/css; charset=utf-8", APP_CSS.as_bytes()),
        ("GET", "/app.js") => respond_no_store(stream, JS_TYPE, APP_JS.as_bytes()),
        ("GET", "/diff.js") => respond_no_store(stream, JS_TYPE, DIFF_JS.as_bytes()),
        ("GET", asset) if asset.starts_with("/tabs/") => {
            match tab_asset(asset.trim_start_matches('/')) {
                Some(body) => respond_no_store(stream, JS_TYPE, body.as_bytes()),
                None => json_resp(stream, 404, json!({ "error": "not found" })),
            }
        }
        ("GET", "/info") | ("GET", "/help") | ("GET", "/about") => serve_info(stream, root),
        ("GET", "/api/status") => api_status(stream, root),
        ("GET", "/api/diff") => api_diff(stream, query, root),
        ("GET", "/api/log") => api_log(stream, root),
        ("GET", "/api/commit") => api_commit_detail(stream, query, root),
        ("GET", "/api/branches") => api_branches(stream, root),
        ("POST", "/api/branch/switch") => api_branch_switch(stream, body, root),
        ("POST", "/api/branch/create") => api_branch_create(stream, body, root),
        ("POST", "/api/branch/delete") => api_branch_delete(stream, body, root),
        ("POST", "/api/branch/rename") => api_branch_rename(stream, body, root),
        ("POST", "/api/branch/merge") => api_branch_merge(stream, body, root),
        ("POST", "/api/branch/rebase") => api_branch_rebase(stream, body, root),
        ("POST", "/api/branch/rebase/continue") => api_branch_rebase_continue(stream, root),
        ("POST", "/api/fetch") => api_fetch(stream, body, root),
        ("GET", "/api/incoming") => api_incoming(stream, query, root),
        ("POST", "/api/push") => api_push(stream, root),
        ("POST", "/api/pull") => api_pull(stream, root),
        ("POST", "/api/publish") => api_publish(stream, body, root),
        ("GET", "/api/stashes") => api_stashes(stream, root),
        ("POST", "/api/stash") => api_stash_push(stream, body, root),
        ("POST", "/api/stash/apply") => api_stash_apply(stream, body, root),
        ("POST", "/api/stash/drop") => api_stash_drop(stream, body, root),
        ("GET", "/api/tags") => api_tags(stream, root),
        ("POST", "/api/tag/create") => api_tag_create(stream, body, root),
        ("POST", "/api/tag/delete") => api_tag_delete(stream, body, root),
        ("POST", "/api/tag/push") => api_tag_push(stream, body, root),
        ("POST", "/api/discard") => api_discard(stream, body, root),
        ("POST", "/api/conflict/resolve") => api_conflict_resolve(stream, body, root),
        ("POST", "/api/stage") => api_stage(stream, body, root),
        ("POST", "/api/unstage") => api_unstage(stream, body, root),
        ("POST", "/api/commit") => api_commit(stream, body, root),
        _ => json_resp(stream, 404, json!({ "error": "not found" })),
    }
}

// ── API ───────────────────────────────────────────────────────────────────

/// A merge leaves MERGE_HEAD behind until the merge commit is made.
fn merge_in_progress(root: &Path) -> bool {
    git::git_output(root, &["rev-parse", "--verify", "--quiet", "MERGE_HEAD"]).is_some()
}

/// REBASE_HEAD outlives the rebase, so look for the state directories instead.
fn rebase_in_progress(root: &Path) -> bool {
    ["rebase-merge", "rebase-apply"].iter().any(|name| {
        git::git_output(root, &["rev-parse", "--git-path", name])
            .map(|out| {
                let path = PathBuf::from(out.trim());
                let path = if path.is_absolute() {
                    path
                } else {
                    root.join(path)
                };
                path.exists()
            })
            .unwrap_or(false)
    })
}

/// Look up a tab module by its exact path (`tabs/changes.js`). Anything not in
/// the allowlist — including `..` segments — misses.
fn tab_asset(name: &str) -> Option<&'static str> {
    TAB_ASSETS
        .iter()
        .find(|(key, _)| *key == name)
        .map(|(_, body)| *body)
}

fn serve_info(stream: &mut TcpStream, root: &Path) {
    let html = INFO_PAGE.replace("{{ROOT}}", &html_escape(&display_path(root)));
    respond(stream, 200, "text/html; charset=utf-8", html.as_bytes());
}

fn api_status(stream: &mut TcpStream, root: &Path) {
    let Some(st) = git::status(root) else {
        return json_resp(stream, 500, json!({ "error": "git status failed" }));
    };
    json_resp(
        stream,
        200,
        json!({
            "root": display_path(root),
            "branch": st.branch,
            "upstream": st.upstream,
            "ahead": st.ahead,
            "behind": st.behind,
            "clean": !st.is_dirty(),
            "staged_count": st.staged.len(),
            "files": file_rows(&st),
            "conflicts": st.conflicts,
            "merging": merge_in_progress(root),
            "rebasing": rebase_in_progress(root),
            "remotes": remotes(root),
        }),
    );
}

fn api_diff(stream: &mut TcpStream, query: &str, root: &Path) {
    let rel = query_param(query, "path").unwrap_or_default();
    let Some(st) = git::status(root) else {
        return json_resp(stream, 500, json!({ "error": "git status failed" }));
    };
    if !is_changed_path(&st, &rel) {
        return json_resp(stream, 404, json!({ "error": "path is not changed" }));
    }

    let staged =
        git::git_output(root, &["diff", "--no-color", "--cached", "--", &rel]).unwrap_or_default();
    let worktree = git::git_output(root, &["diff", "--no-color", "--", &rel]).unwrap_or_default();
    let (staged, staged_cut) = truncate_diff(&staged);
    let (worktree, worktree_cut) = truncate_diff(&worktree);

    json_resp(
        stream,
        200,
        json!({
            "path": rel,
            "staged": staged,
            "worktree": worktree,
            "truncated": staged_cut || worktree_cut,
            "untracked": st.untracked.iter().any(|p| p == &rel),
            "conflict": st.conflicts.iter().any(|p| p == &rel),
        }),
    );
}

fn commit_rows(out: &str) -> Vec<Value> {
    let mut commits: Vec<Value> = Vec::new();
    for line in out.lines() {
        let mut fields = line.split('\u{1f}');
        let hash = fields.next().unwrap_or("").to_string();
        if hash.is_empty() {
            continue;
        }
        commits.push(json!({
            "hash": hash,
            "author": fields.next().unwrap_or(""),
            "when": fields.next().unwrap_or(""),
            "summary": fields.next().unwrap_or(""),
        }));
    }
    commits
}

fn api_log(stream: &mut TcpStream, root: &Path) {
    let limit = LOG_LIMIT.to_string();
    let out = git::git_output(
        root,
        &[
            "log",
            "-n",
            &limit,
            "--pretty=format:%h%x1f%an%x1f%ar%x1f%s",
        ],
    );
    let commits = commit_rows(&out.unwrap_or_default());
    json_resp(
        stream,
        200,
        json!({ "commits": commits, "empty": commits.is_empty() }),
    );
}

fn is_valid_commit_hash(hash: &str) -> bool {
    (4..=64).contains(&hash.len()) && hash.chars().all(|c| c.is_ascii_hexdigit())
}

fn api_commit_detail(stream: &mut TcpStream, query: &str, root: &Path) {
    let Some(hash) = query_param(query, "hash") else {
        return json_resp(stream, 400, json!({ "error": "hash is required" }));
    };
    if !is_valid_commit_hash(&hash) {
        return json_resp(stream, 400, json!({ "error": "invalid commit hash" }));
    }
    let Some(meta) = git::git_output(
        root,
        &["show", "-s", "--format=%H%x1f%an%x1f%ar%x1f%s", &hash],
    ) else {
        return json_resp(stream, 404, json!({ "error": "unknown commit" }));
    };
    let meta = meta.trim();
    let mut fields = meta.splitn(4, '\u{1f}');
    let full_hash = fields.next().unwrap_or("");
    let author = fields.next().unwrap_or("");
    let when = fields.next().unwrap_or("");
    let summary = fields.next().unwrap_or("");
    if full_hash.is_empty() {
        return json_resp(stream, 404, json!({ "error": "unknown commit" }));
    }

    let raw = git::git_output(
        root,
        &[
            "show",
            "--name-status",
            "--format=",
            "--find-renames",
            "-z",
            &hash,
        ],
    )
    .unwrap_or_default();
    let (files, truncated) = parse_name_status(&raw);

    json_resp(
        stream,
        200,
        json!({
            "hash": full_hash,
            "author": author,
            "when": when,
            "summary": summary,
            "files": files,
            "truncated": truncated,
        }),
    )
}

fn local_branches(root: &Path) -> Vec<String> {
    let mut branches = Vec::new();
    if let Some(out) = git::git_output(
        root,
        &["for-each-ref", "--format=%(refname:short)", "refs/heads"],
    ) {
        for line in out.lines() {
            let name = line.trim();
            if !name.is_empty() {
                branches.push(name.to_string());
            }
        }
    }
    branches
}

fn api_branches(stream: &mut TcpStream, root: &Path) {
    let st = git::status(root);
    let current = st.as_ref().and_then(|s| s.branch.clone());
    let detached = st.as_ref().map(|s| s.branch.is_none()).unwrap_or(true);
    json_resp(
        stream,
        200,
        json!({
            "current": current,
            "detached": detached,
            "branches": local_branches(root),
        }),
    )
}

fn api_branch_switch(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(name) = parse_name(body) else {
        return json_resp(stream, 400, json!({ "error": "invalid JSON body" }));
    };
    if !is_valid_branch_name(&name) {
        return json_resp(stream, 400, json!({ "error": "invalid branch name" }));
    }
    if !local_branches(root).iter().any(|b| b == &name) {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("unknown branch: {}", name) }),
        );
    }
    match git::run_git(root, &["checkout", &name]) {
        Ok(_) => {
            git::invalidate();
            json_resp(
                stream,
                200,
                json!({
                    "ok": true,
                    "command": format!("git checkout {}", shell_quote(&name)),
                }),
            )
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn api_branch_create(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(name) = parse_name(body) else {
        return json_resp(stream, 400, json!({ "error": "invalid JSON body" }));
    };
    if !is_valid_branch_name(&name) {
        return json_resp(stream, 400, json!({ "error": "invalid branch name" }));
    }
    let reference = format!("refs/heads/{}", name);
    if git::git_output(root, &["rev-parse", "--verify", "--quiet", &reference]).is_some() {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("branch already exists: {}", name) }),
        );
    }
    match git::run_git(root, &["checkout", "-b", &name]) {
        Ok(_) => {
            git::invalidate();
            json_resp(
                stream,
                200,
                json!({
                    "ok": true,
                    "command": format!("git checkout -b {}", shell_quote(&name)),
                }),
            )
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn current_branch(root: &Path) -> Option<String> {
    let out = git::git_output(root, &["symbolic-ref", "--quiet", "--short", "HEAD"])?;
    let name = out.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

fn api_branch_delete(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(name) = parse_name(body) else {
        return json_resp(stream, 400, json!({ "error": "invalid JSON body" }));
    };
    if !is_valid_branch_name(&name) {
        return json_resp(stream, 400, json!({ "error": "invalid branch name" }));
    }
    if !local_branches(root).iter().any(|b| b == &name) {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("unknown branch: {}", name) }),
        );
    }
    if current_branch(root).as_deref() == Some(name.as_str()) {
        return json_resp(
            stream,
            400,
            json!({ "error": "cannot delete the current branch" }),
        );
    }
    match git::run_git(root, &["branch", "-d", &name]) {
        Ok(_) => {
            git::invalidate();
            json_resp(
                stream,
                200,
                json!({
                    "ok": true,
                    "command": format!("git branch -d {}", shell_quote(&name)),
                }),
            )
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn api_branch_rename(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some((name, new_name)) = parse_rename(body) else {
        return json_resp(
            stream,
            400,
            json!({ "error": "name and new_name are required" }),
        );
    };
    if !is_valid_branch_name(&name) || !is_valid_branch_name(&new_name) {
        return json_resp(stream, 400, json!({ "error": "invalid branch name" }));
    }
    let branches = local_branches(root);
    if !branches.iter().any(|b| b == &name) {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("unknown branch: {}", name) }),
        );
    }
    if name == new_name {
        return json_resp(
            stream,
            400,
            json!({ "error": "new name is the same as the old name" }),
        );
    }
    if branches.iter().any(|b| b == &new_name) {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("branch already exists: {}", new_name) }),
        );
    }
    match git::run_git(root, &["branch", "-m", &name, &new_name]) {
        Ok(_) => {
            git::invalidate();
            json_resp(
                stream,
                200,
                json!({
                    "ok": true,
                    "command": format!(
                        "git branch -m {} {}",
                        shell_quote(&name),
                        shell_quote(&new_name)
                    ),
                }),
            )
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn remotes(root: &Path) -> Vec<String> {
    let mut names = Vec::new();
    if let Some(out) = git::git_output(root, &["remote"]) {
        for line in out.lines() {
            let name = line.trim();
            if !name.is_empty() {
                names.push(name.to_string());
            }
        }
    }
    names
}

fn api_branch_merge(stream: &mut TcpStream, body: &[u8], root: &Path) {
    api_branch_integrate(stream, body, root, false)
}

fn api_branch_rebase(stream: &mut TcpStream, body: &[u8], root: &Path) {
    api_branch_integrate(stream, body, root, true)
}

/// Merge a branch into the current one, or rebase the current branch onto it.
/// Conflicts come back as git's own message and show up in the Conflicts panel.
fn api_branch_integrate(stream: &mut TcpStream, body: &[u8], root: &Path, rebase: bool) {
    let Some(name) = parse_name(body) else {
        return json_resp(stream, 400, json!({ "error": "branch name is required" }));
    };
    if !is_valid_branch_name(&name) {
        return json_resp(stream, 400, json!({ "error": "invalid branch name" }));
    }
    let Some(current) = current_branch(root) else {
        return json_resp(
            stream,
            400,
            json!({ "error": "detached HEAD: check out a branch first" }),
        );
    };
    if !local_branches(root).iter().any(|b| b == &name) {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("unknown branch: {}", name) }),
        );
    }
    if current == name {
        return json_resp(
            stream,
            400,
            json!({ "error": "cannot integrate a branch into itself" }),
        );
    }

    let args: Vec<&str> = if rebase {
        vec!["rebase", &name]
    } else {
        vec!["merge", "--no-edit", &name]
    };
    match git::run_git(root, &args) {
        Ok(_) => {
            git::invalidate();
            let command = if rebase {
                format!("git rebase {}", shell_quote(&name))
            } else {
                format!("git merge --no-edit {}", shell_quote(&name))
            };
            json_resp(stream, 200, json!({ "ok": true, "command": command }))
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn api_branch_rebase_continue(stream: &mut TcpStream, root: &Path) {
    if !rebase_in_progress(root) {
        return json_resp(stream, 400, json!({ "error": "no rebase in progress" }));
    }
    git::invalidate();
    let Some(st) = git::status(root) else {
        return json_resp(stream, 500, json!({ "error": "git status failed" }));
    };
    if !st.conflicts.is_empty() {
        return json_resp(
            stream,
            400,
            json!({ "error": "resolve and stage every conflict before continuing" }),
        );
    }
    // core.editor=true keeps `--continue` from waiting for an editor.
    match git::run_git(root, &["-c", "core.editor=true", "rebase", "--continue"]) {
        Ok(_) => {
            git::invalidate();
            json_resp(
                stream,
                200,
                json!({ "ok": true, "command": "git rebase --continue" }),
            )
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

/// Push and pull need a branch with an upstream; git itself decides whether
/// there is anything to transfer.
fn upstream_ref(root: &Path) -> Option<String> {
    let out = git::git_output(
        root,
        &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"],
    )?;
    let name = out.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

fn check_sync_ready(stream: &mut TcpStream, root: &Path) -> bool {
    if current_branch(root).is_none() {
        json_resp(
            stream,
            400,
            json!({ "error": "detached HEAD: check out a branch first" }),
        );
        return false;
    }
    if upstream_ref(root).is_none() {
        json_resp(
            stream,
            400,
            json!({ "error": "branch has no upstream; set one with git push -u <remote> <branch>" }),
        );
        return false;
    }
    true
}

fn api_fetch(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(remote) = parse_remote(body) else {
        return json_resp(stream, 400, json!({ "error": "remote is required" }));
    };
    if !remotes(root).iter().any(|r| r == &remote) {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("unknown remote: {}", remote) }),
        );
    }
    match git::run_git_net(root, &["fetch", &remote]) {
        Ok(_) => {
            git::invalidate();
            json_resp(
                stream,
                200,
                json!({
                    "ok": true,
                    "command": format!("git fetch {}", shell_quote(&remote)),
                }),
            )
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

/// Commits a pull would bring in: everything on the tracked branch that HEAD
/// does not have yet. Fetch first, since remote-tracking refs only move then.
fn api_incoming(stream: &mut TcpStream, query: &str, root: &Path) {
    let Some(remote) = query_param(query, "remote") else {
        return json_resp(stream, 400, json!({ "error": "remote is required" }));
    };
    if !remotes(root).iter().any(|r| r == &remote) {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("unknown remote: {}", remote) }),
        );
    }
    let Some(branch) = current_branch(root) else {
        return json_resp(
            stream,
            400,
            json!({ "error": "detached HEAD: check out a branch first" }),
        );
    };

    let prefix = format!("{}/", remote);
    let target = upstream_ref(root)
        .filter(|u| u.starts_with(&prefix))
        .unwrap_or_else(|| format!("{}/{}", remote, branch));
    let verify = format!("refs/remotes/{}", target);
    if git::git_output(root, &["rev-parse", "--verify", "--quiet", &verify]).is_none() {
        return json_resp(
            stream,
            200,
            json!({
                "target": target,
                "incoming": [],
                "count": 0,
                "missing": true,
                "truncated": false,
            }),
        );
    }

    let range = format!("HEAD..{}", target);
    let count = git::git_output(root, &["rev-list", "--count", &range])
        .unwrap_or_default()
        .trim()
        .parse::<usize>()
        .unwrap_or(0);
    let limit = INCOMING_LIMIT.to_string();
    let out = git::git_output(
        root,
        &[
            "log",
            "-n",
            &limit,
            "--pretty=format:%h%x1f%an%x1f%ar%x1f%s",
            &range,
        ],
    )
    .unwrap_or_default();
    let incoming = commit_rows(&out);

    json_resp(
        stream,
        200,
        json!({
            "target": target,
            "incoming": incoming,
            "count": count,
            "missing": false,
            "truncated": count > incoming.len(),
        }),
    )
}

fn api_push(stream: &mut TcpStream, root: &Path) {
    if !check_sync_ready(stream, root) {
        return;
    }
    match git::run_git_net(root, &["push"]) {
        Ok(_) => {
            git::invalidate();
            json_resp(stream, 200, json!({ "ok": true, "command": "git push" }))
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn api_pull(stream: &mut TcpStream, root: &Path) {
    if !check_sync_ready(stream, root) {
        return;
    }
    // --ff-only keeps a GUI pull from creating an unresolved merge commit.
    match git::run_git_net(root, &["pull", "--ff-only"]) {
        Ok(_) => {
            git::invalidate();
            json_resp(
                stream,
                200,
                json!({ "ok": true, "command": "git pull --ff-only" }),
            )
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

/// Publish the current branch: push it to a remote and set its upstream, so
/// Push and Pull start working. Unlike `api_push`, this needs no upstream yet.
fn api_publish(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(remote) = parse_remote(body) else {
        return json_resp(stream, 400, json!({ "error": "remote is required" }));
    };
    let Some(branch) = current_branch(root) else {
        return json_resp(
            stream,
            400,
            json!({ "error": "detached HEAD: check out a branch first" }),
        );
    };
    if let Some(upstream) = upstream_ref(root) {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("branch already tracks {}; use Push", upstream) }),
        );
    }
    if !remotes(root).iter().any(|r| r == &remote) {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("unknown remote: {}", remote) }),
        );
    }
    match git::run_git_net(root, &["push", "-u", &remote, &branch]) {
        Ok(_) => {
            git::invalidate();
            json_resp(
                stream,
                200,
                json!({
                    "ok": true,
                    "command": format!(
                        "git push -u {} {}",
                        shell_quote(&remote),
                        shell_quote(&branch)
                    ),
                }),
            )
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn stash_entries(root: &Path) -> Vec<(String, String)> {
    let out = git::git_output(root, &["stash", "list", "--format=%gd%x1f%s"]).unwrap_or_default();
    out.lines()
        .filter_map(|line| {
            let (reference, message) = line.split_once('\u{1f}')?;
            if reference.is_empty() {
                None
            } else {
                Some((reference.to_string(), message.to_string()))
            }
        })
        .collect()
}

fn stash_ref(index: usize) -> String {
    format!("stash@{{{}}}", index)
}

fn api_stashes(stream: &mut TcpStream, root: &Path) {
    let stashes: Vec<Value> = stash_entries(root)
        .into_iter()
        .map(|(reference, message)| json!({ "ref": reference, "message": message }))
        .collect();
    json_resp(
        stream,
        200,
        json!({ "stashes": stashes, "empty": stashes.is_empty() }),
    );
}

fn api_stash_push(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let message = parse_body(body)
        .and_then(|v| {
            v.get("message")
                .and_then(|m| m.as_str())
                .map(|s| s.trim().to_string())
        })
        .unwrap_or_default();
    if message.chars().count() > 200 {
        return json_resp(stream, 400, json!({ "error": "stash message is too long" }));
    }
    if message.chars().any(|c| c.is_control()) {
        return json_resp(
            stream,
            400,
            json!({ "error": "stash message cannot contain control characters" }),
        );
    }
    // Fresh status: edits made outside the browser may be newer than the cache.
    git::invalidate();
    let Some(st) = git::status(root) else {
        return json_resp(stream, 500, json!({ "error": "git status failed" }));
    };
    // `git stash push` reports success without saving anything on a clean tree.
    if st.staged.is_empty() && st.unstaged.is_empty() && st.conflicts.is_empty() {
        return json_resp(
            stream,
            400,
            json!({ "error": "nothing to stash (untracked files are left alone)" }),
        );
    }

    let mut args: Vec<&str> = vec!["stash", "push"];
    if !message.is_empty() {
        args.push("-m");
        args.push(&message);
    }
    match git::run_git(root, &args) {
        Ok(_) => {
            git::invalidate();
            let mut command = "git stash push".to_string();
            if !message.is_empty() {
                command.push_str(" -m ");
                command.push_str(&shell_quote(&message));
            }
            json_resp(stream, 200, json!({ "ok": true, "command": command }))
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn parse_stash_index(body: &[u8]) -> Option<usize> {
    let value = parse_body(body)?;
    usize::try_from(value.get("index")?.as_u64()?).ok()
}

fn api_stash_apply(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(index) = parse_stash_index(body) else {
        return json_resp(stream, 400, json!({ "error": "stash index is required" }));
    };
    if index >= stash_entries(root).len() {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("unknown stash: {}", stash_ref(index)) }),
        );
    }
    let drop = parse_body(body)
        .and_then(|v| v.get("drop").and_then(|d| d.as_bool()))
        .unwrap_or(false);
    let action = if drop { "pop" } else { "apply" };
    let reference = stash_ref(index);
    match git::run_git(root, &["stash", action, &reference]) {
        Ok(_) => {
            git::invalidate();
            json_resp(
                stream,
                200,
                json!({
                    "ok": true,
                    "command": format!("git stash {} {}", action, reference),
                }),
            )
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn api_stash_drop(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(index) = parse_stash_index(body) else {
        return json_resp(stream, 400, json!({ "error": "stash index is required" }));
    };
    if index >= stash_entries(root).len() {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("unknown stash: {}", stash_ref(index)) }),
        );
    }
    let reference = stash_ref(index);
    match git::run_git(root, &["stash", "drop", &reference]) {
        Ok(_) => json_resp(
            stream,
            200,
            json!({ "ok": true, "command": format!("git stash drop {}", reference) }),
        ),
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn tag_entries(root: &Path) -> Vec<(String, String, String)> {
    let out = git::git_output(
        root,
        &[
            "for-each-ref",
            "--sort=-creatordate",
            "--format=%(refname:short)%00%(if)%(*objectname)%(then)%(*objectname:short)%(else)%(objectname:short)%(end)%00%(contents:subject)",
            "refs/tags",
        ],
    )
    .unwrap_or_default();
    out.lines()
        .filter_map(|line| {
            let mut fields = line.split('\0');
            let name = fields.next().unwrap_or("").to_string();
            if name.is_empty() {
                return None;
            }
            Some((
                name,
                fields.next().unwrap_or("").to_string(),
                fields.next().unwrap_or("").to_string(),
            ))
        })
        .collect()
}

fn api_tags(stream: &mut TcpStream, root: &Path) {
    let tags: Vec<Value> = tag_entries(root)
        .into_iter()
        .map(|(name, hash, summary)| json!({ "name": name, "hash": hash, "summary": summary }))
        .collect();
    json_resp(
        stream,
        200,
        json!({ "tags": tags, "empty": tags.is_empty() }),
    );
}

fn parse_tag_create(body: &[u8]) -> Option<(String, String)> {
    let value = parse_body(body)?;
    let name = value.get("name")?.as_str()?.trim().to_string();
    if name.is_empty() {
        return None;
    }
    let message = value
        .get("message")
        .and_then(|m| m.as_str())
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    Some((name, message))
}

fn api_tag_create(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some((name, message)) = parse_tag_create(body) else {
        return json_resp(stream, 400, json!({ "error": "tag name is required" }));
    };
    // Tags follow the same ref-naming rules as branches.
    if !is_valid_branch_name(&name) {
        return json_resp(stream, 400, json!({ "error": "invalid tag name" }));
    }
    if message.chars().count() > 500 {
        return json_resp(stream, 400, json!({ "error": "tag message is too long" }));
    }
    if message.chars().any(|c| c.is_control()) {
        return json_resp(
            stream,
            400,
            json!({ "error": "tag message cannot contain control characters" }),
        );
    }
    let reference = format!("refs/tags/{}", name);
    if git::git_output(root, &["rev-parse", "--verify", "--quiet", &reference]).is_some() {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("tag already exists: {}", name) }),
        );
    }

    let mut args: Vec<&str> = vec!["tag"];
    if !message.is_empty() {
        args.push("-a");
        args.push(&name);
        args.push("-m");
        args.push(&message);
    } else {
        args.push(&name);
    }
    match git::run_git(root, &args) {
        Ok(_) => {
            let mut command = "git tag".to_string();
            if !message.is_empty() {
                command.push_str(" -a");
            }
            command.push(' ');
            command.push_str(&shell_quote(&name));
            if !message.is_empty() {
                command.push_str(" -m ");
                command.push_str(&shell_quote(&message));
            }
            json_resp(stream, 200, json!({ "ok": true, "command": command }))
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn api_tag_delete(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(name) = parse_name(body) else {
        return json_resp(stream, 400, json!({ "error": "tag name is required" }));
    };
    if !is_valid_branch_name(&name) {
        return json_resp(stream, 400, json!({ "error": "invalid tag name" }));
    }
    if !tag_entries(root).iter().any(|(n, _, _)| n == &name) {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("unknown tag: {}", name) }),
        );
    }
    match git::run_git(root, &["tag", "-d", &name]) {
        Ok(_) => json_resp(
            stream,
            200,
            json!({
                "ok": true,
                "command": format!("git tag -d {}", shell_quote(&name)),
            }),
        ),
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn api_tag_push(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(remote) = parse_remote(body) else {
        return json_resp(stream, 400, json!({ "error": "remote is required" }));
    };
    if !remotes(root).iter().any(|r| r == &remote) {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("unknown remote: {}", remote) }),
        );
    }
    if tag_entries(root).is_empty() {
        return json_resp(stream, 400, json!({ "error": "no tags to push" }));
    }
    match git::run_git_net(root, &["push", &remote, "--tags"]) {
        Ok(_) => json_resp(
            stream,
            200,
            json!({
                "ok": true,
                "command": format!("git push {} --tags", shell_quote(&remote)),
            }),
        ),
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

/// Discard unstaged working-tree changes for the selected paths. Staged
/// changes are left in the index, so `git restore --staged` is still needed to
/// unstage them.
fn api_discard(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(paths) = parse_paths(body) else {
        return json_resp(
            stream,
            400,
            json!({ "error": "paths must be a non-empty list" }),
        );
    };
    // Fresh status: edits made outside the browser may be newer than the cache.
    git::invalidate();
    let Some(st) = git::status(root) else {
        return json_resp(stream, 500, json!({ "error": "git status failed" }));
    };
    for path in &paths {
        if st.untracked.iter().any(|p| p == path) {
            return json_resp(
                stream,
                400,
                json!({ "error": format!("untracked file cannot be discarded: {}", path) }),
            );
        }
        if st.conflicts.iter().any(|p| p == path) {
            return json_resp(
                stream,
                400,
                json!({ "error": format!("resolve the conflict in {} first", path) }),
            );
        }
        if !st.unstaged.iter().any(|(_, p)| p == path) {
            return json_resp(
                stream,
                400,
                json!({ "error": format!("no unstaged changes in {}; unstage it first", path) }),
            );
        }
    }

    let mut args: Vec<&str> = vec!["restore", "--"];
    args.extend(paths.iter().map(|s| s.as_str()));
    match git::run_git(root, &args) {
        Ok(_) => {
            git::invalidate();
            let mut command = "git restore --".to_string();
            for path in &paths {
                command.push(' ');
                command.push_str(&shell_quote(path));
            }
            json_resp(stream, 200, json!({ "ok": true, "command": command }))
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn parse_conflict(body: &[u8]) -> Option<(String, String)> {
    let value = parse_body(body)?;
    let path = value.get("path")?.as_str()?.trim().to_string();
    let side = value.get("side")?.as_str()?.trim().to_lowercase();
    if path.is_empty() || path.starts_with('-') || side.is_empty() {
        None
    } else {
        Some((path, side))
    }
}

/// Resolve one conflicted file by taking a whole side, then stage it so git
/// treats the conflict as resolved.
fn api_conflict_resolve(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some((path, side)) = parse_conflict(body) else {
        return json_resp(
            stream,
            400,
            json!({ "error": "path and side are required" }),
        );
    };
    let flag = match side.as_str() {
        "ours" => "--ours",
        "theirs" => "--theirs",
        _ => {
            return json_resp(
                stream,
                400,
                json!({ "error": "side must be ours or theirs" }),
            )
        }
    };
    git::invalidate();
    let Some(st) = git::status(root) else {
        return json_resp(stream, 500, json!({ "error": "git status failed" }));
    };
    if !st.conflicts.iter().any(|p| p == &path) {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("not a conflicted file: {}", path) }),
        );
    }

    if let Err(e) = git::run_git(root, &["checkout", flag, "--", &path]) {
        return json_resp(stream, 400, json!({ "error": e }));
    }
    match git::run_git(root, &["add", "--", &path]) {
        Ok(_) => {
            git::invalidate();
            json_resp(
                stream,
                200,
                json!({
                    "ok": true,
                    "command": format!(
                        "git checkout {} -- {} && git add -- {}",
                        flag,
                        shell_quote(&path),
                        shell_quote(&path)
                    ),
                }),
            )
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn api_stage(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(paths) = parse_paths(body) else {
        return json_resp(
            stream,
            400,
            json!({ "error": "paths must be a non-empty list" }),
        );
    };
    let Some(st) = git::status(root) else {
        return json_resp(stream, 500, json!({ "error": "git status failed" }));
    };
    if let Some(bad) = paths.iter().find(|p| !is_changed_path(&st, p)) {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("not a changed file: {}", bad) }),
        );
    }

    let mut args: Vec<&str> = vec!["add", "--"];
    args.extend(paths.iter().map(|s| s.as_str()));
    match git::run_git(root, &args) {
        Ok(_) => {
            git::invalidate();
            let mut command = "git add --".to_string();
            for path in &paths {
                command.push(' ');
                command.push_str(&shell_quote(path));
            }
            json_resp(stream, 200, json!({ "ok": true, "command": command }))
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn api_unstage(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(paths) = parse_paths(body) else {
        return json_resp(
            stream,
            400,
            json!({ "error": "paths must be a non-empty list" }),
        );
    };
    let Some(st) = git::status(root) else {
        return json_resp(stream, 500, json!({ "error": "git status failed" }));
    };
    let staged: Vec<&String> = st.staged.iter().map(|(_, p)| p).collect();
    if let Some(bad) = paths.iter().find(|p| !staged.contains(p)) {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("not staged: {}", bad) }),
        );
    }

    // A repository with no first commit has nothing to restore *from*, so the
    // index entry is removed instead.
    let has_head = git::git_output(root, &["rev-parse", "--verify", "--quiet", "HEAD"]).is_some();
    let mut args: Vec<&str> = if has_head {
        vec!["restore", "--staged", "--"]
    } else {
        vec!["rm", "--cached", "-q", "--"]
    };
    args.extend(paths.iter().map(|s| s.as_str()));
    match git::run_git(root, &args) {
        Ok(_) => {
            git::invalidate();
            let prefix = if has_head {
                "git restore --staged --"
            } else {
                "git rm --cached --"
            };
            let mut command = prefix.to_string();
            for path in &paths {
                command.push(' ');
                command.push_str(&shell_quote(path));
            }
            json_resp(stream, 200, json!({ "ok": true, "command": command }))
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn api_commit(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(value) = parse_body(body) else {
        return json_resp(stream, 400, json!({ "error": "invalid JSON body" }));
    };
    let message = value
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    if message.is_empty() {
        return json_resp(stream, 400, json!({ "error": "commit message is empty" }));
    }
    if message.chars().count() > MAX_MESSAGE {
        return json_resp(
            stream,
            400,
            json!({ "error": format!("commit message is too long (max {} chars)", MAX_MESSAGE) }),
        );
    }

    match git::run_git(root, &["commit", "-m", message]) {
        Ok(out) => {
            git::invalidate();
            let summary = out
                .lines()
                .map(str::trim)
                .find(|l| !l.is_empty())
                .unwrap_or("commit created")
                .to_string();
            json_resp(
                stream,
                200,
                json!({
                    "ok": true,
                    "summary": summary,
                    "command": format!("git commit -m {}", shell_quote(message)),
                }),
            )
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────

struct Row {
    index: Option<char>,
    worktree: Option<char>,
    untracked: bool,
    conflict: bool,
}

fn row_mut<'a>(map: &'a mut BTreeMap<String, Row>, path: &str) -> &'a mut Row {
    map.entry(path.to_string()).or_insert(Row {
        index: None,
        worktree: None,
        untracked: false,
        conflict: false,
    })
}

/// One JSON row per changed path, merging index/worktree/untracked/conflict
/// state the way `git status --short` pairs its two columns.
fn file_rows(st: &GitStatus) -> Vec<Value> {
    let mut map: BTreeMap<String, Row> = BTreeMap::new();
    for (c, p) in &st.staged {
        row_mut(&mut map, p).index = Some(*c);
    }
    for (c, p) in &st.unstaged {
        row_mut(&mut map, p).worktree = Some(*c);
    }
    for p in &st.untracked {
        row_mut(&mut map, p).untracked = true;
    }
    for p in &st.conflicts {
        let r = row_mut(&mut map, p);
        r.conflict = true;
        r.index = Some('U');
        r.worktree = Some('U');
    }

    map.into_iter()
        .map(|(path, r)| {
            let code = r
                .index
                .or(r.worktree)
                .unwrap_or(if r.untracked { '?' } else { ' ' });
            json!({
                "path": path,
                "code": code.to_string(),
                "index": r.index.map(|c| c.to_string()),
                "worktree": r.worktree.map(|c| c.to_string()),
                "untracked": r.untracked,
                "conflict": r.conflict,
            })
        })
        .collect()
}

fn is_changed_path(st: &GitStatus, path: &str) -> bool {
    path == path.trim()
        && !path.is_empty()
        && (st.staged.iter().any(|(_, p)| p == path)
            || st.unstaged.iter().any(|(_, p)| p == path)
            || st.untracked.iter().any(|p| p == path)
            || st.conflicts.iter().any(|p| p == path))
}

/// Parse `git show --name-status -z` output: NUL-separated status/path tokens,
/// with renames and copies carrying the old path before the new one.
fn parse_name_status(raw: &str) -> (Vec<Value>, bool) {
    const MAX_FILES: usize = 500;
    let tokens: Vec<&str> = raw.split('\0').filter(|t| !t.is_empty()).collect();
    let mut files = Vec::new();
    let mut truncated = false;
    let mut i = 0;
    while i < tokens.len() {
        let status = tokens[i];
        let mut chars = status.chars();
        let kind = chars.next().unwrap_or(' ');
        if !"AMDRCTUXB".contains(kind) || !chars.all(|c| c.is_ascii_digit()) {
            i += 1;
            continue;
        }
        let rename = kind == 'R' || kind == 'C';
        if rename {
            if i + 2 >= tokens.len() {
                break;
            }
            if files.len() == MAX_FILES {
                truncated = true;
                break;
            }
            files.push(json!({
                "status": status,
                "old": tokens[i + 1],
                "path": tokens[i + 2],
            }));
            i += 3;
        } else {
            if i + 1 >= tokens.len() {
                break;
            }
            if files.len() == MAX_FILES {
                truncated = true;
                break;
            }
            files.push(json!({ "status": status, "path": tokens[i + 1] }));
            i += 2;
        }
    }
    (files, truncated)
}

fn truncate_diff(text: &str) -> (String, bool) {
    if text.len() <= MAX_DIFF {
        return (text.to_string(), false);
    }
    let mut cut = MAX_DIFF;
    while !text.is_char_boundary(cut) {
        cut -= 1;
    }
    let mut out = text[..cut].to_string();
    out.push_str("\n… diff truncated\n");
    (out, true)
}

fn parse_paths(body: &[u8]) -> Option<Vec<String>> {
    let value = parse_body(body)?;
    let mut paths: Vec<String> = Vec::new();
    for p in value.get("paths")?.as_array()? {
        let p = p.as_str()?.to_string();
        if !paths.contains(&p) {
            paths.push(p);
        }
    }
    if paths.is_empty() {
        None
    } else {
        Some(paths)
    }
}

fn parse_rename(body: &[u8]) -> Option<(String, String)> {
    let value = parse_body(body)?;
    let name = value.get("name")?.as_str()?.trim();
    let new_name = value.get("new_name")?.as_str()?.trim();
    if name.is_empty() || new_name.is_empty() {
        None
    } else {
        Some((name.to_string(), new_name.to_string()))
    }
}

fn parse_name(body: &[u8]) -> Option<String> {
    let value = parse_body(body)?;
    let name = value.get("name")?.as_str()?.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

fn parse_remote(body: &[u8]) -> Option<String> {
    let value = parse_body(body)?;
    let remote = value.get("remote")?.as_str()?.trim();
    // Remote names are matched against `git remote`, which also rules out
    // anything git would read as an option.
    if remote.is_empty() || remote.starts_with('-') {
        None
    } else {
        Some(remote.to_string())
    }
}

/// Reject names git would treat as an option or refuse as a ref.
fn is_valid_branch_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 200 {
        return false;
    }
    if name.starts_with('-') || name.starts_with('.') || name.ends_with('/') {
        return false;
    }
    if name.ends_with('.') || name.ends_with(".lock") {
        return false;
    }
    if name.contains("..") || name.contains("//") || name.contains("@{") || name == "HEAD" {
        return false;
    }
    !name.chars().any(|c| {
        c.is_whitespace() || c.is_control() || matches!(c, '~' | '^' | ':' | '?' | '*' | '[' | '\\')
    })
}

fn parse_body(body: &[u8]) -> Option<Value> {
    serde_json::from_slice(body).ok()
}

fn query_param(query: &str, key: &str) -> Option<String> {
    for pair in query.split('&') {
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        if url_decode(k) == key {
            return Some(url_decode(v));
        }
    }
    None
}

fn json_resp(stream: &mut TcpStream, status: u16, value: Value) {
    let body = value.to_string();
    respond(
        stream,
        status,
        "application/json; charset=utf-8",
        body.as_bytes(),
    );
}

fn shell_quote(value: &str) -> String {
    let plain = !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '/' | ':' | '+'));
    if plain {
        value.to_string()
    } else {
        format!("\"{}\"", value.replace('"', "\\\""))
    }
}

fn is_loopback(bind: &str) -> bool {
    bind == "localhost" || bind == "::1" || bind.starts_with("127.")
}

/// Canonical Windows paths carry a `\\?\` verbatim prefix; strip it for display.
fn display_path(path: &Path) -> String {
    let shown = path.display().to_string();
    shown.strip_prefix("\\\\?\\").unwrap_or(&shown).to_string()
}

fn html_escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

fn print_git_help() {
    println!("{}", t!("cmd.serve.git_title").bold());
    println!();
    println!("{}", t!("cmd.serve.usage_title").cyan().bold());
    println!("  {}", t!("cmd.serve.git_usage"));
    println!();
    println!("{}", t!("cmd.serve.options_title").cyan().bold());
    println!(
        "  {}",
        t!(
            "cmd.serve.git_help_port",
            Config::global_get_serve_port(ServeService::Git)
        )
    );
    println!(
        "  {}",
        t!(
            "cmd.serve.git_help_bind",
            Config::global_get_serve_bind(ServeService::Git)
        )
    );
    println!("  {}", t!("cmd.serve.help_help"));
    println!();
    println!("{}", t!("cmd.serve.examples_title").cyan().bold());
    println!("  {}", t!("cmd.serve.git_example"));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn status_with(
        staged: &[(&str, char)],
        unstaged: &[(&str, char)],
        untracked: &[&str],
        conflicts: &[&str],
    ) -> GitStatus {
        let mut st = GitStatus::default();
        st.staged = staged.iter().map(|(p, c)| (*c, p.to_string())).collect();
        st.unstaged = unstaged.iter().map(|(p, c)| (*c, p.to_string())).collect();
        st.untracked = untracked.iter().map(|p| p.to_string()).collect();
        st.conflicts = conflicts.iter().map(|p| p.to_string()).collect();
        st
    }

    #[test]
    fn rows_merge_index_and_worktree_state() {
        let st = status_with(
            &[("staged.rs", 'M')],
            &[("staged.rs", 'M'), ("worktree.rs", 'M')],
            &["new.rs"],
            &["both.rs"],
        );
        let rows = file_rows(&st);
        assert_eq!(rows.len(), 4);
        let by_path = |p: &str| rows.iter().find(|r| r["path"] == p).unwrap().clone();
        assert_eq!(by_path("staged.rs")["index"], "M");
        assert_eq!(by_path("staged.rs")["worktree"], "M");
        assert_eq!(by_path("worktree.rs")["code"], "M");
        assert_eq!(by_path("new.rs")["code"], "?");
        assert_eq!(by_path("both.rs")["conflict"], true);
    }

    #[test]
    fn only_changed_paths_are_accepted() {
        let st = status_with(&[("a.rs", 'M')], &[], &["b.rs"], &[]);
        assert!(is_changed_path(&st, "a.rs"));
        assert!(is_changed_path(&st, "b.rs"));
        assert!(!is_changed_path(&st, "other.rs"));
        assert!(!is_changed_path(&st, "-rf"));
        assert!(!is_changed_path(&st, " a.rs"));
    }

    #[test]
    fn diff_truncation_stays_on_char_boundaries() {
        let (short, cut) = truncate_diff("diff");
        assert_eq!(short, "diff");
        assert!(!cut);

        let long = "é".repeat(MAX_DIFF);
        let (cut_text, cut) = truncate_diff(&long);
        assert!(cut);
        assert!(cut_text.len() <= MAX_DIFF + 32);
        assert!(cut_text.contains("truncated"));
    }

    #[test]
    fn paths_payload_requires_a_non_empty_list() {
        assert_eq!(
            parse_paths(br#"{"paths":["a","a","b"]}"#),
            Some(vec!["a".to_string(), "b".to_string()])
        );
        assert!(parse_paths(br#"{"paths":[]}"#).is_none());
        assert!(parse_paths(br#"{"paths":"a"}"#).is_none());
        assert!(parse_paths(b"not json").is_none());
    }

    #[test]
    fn branch_names_are_validated() {
        for ok in [
            "main",
            "feature/new-ui",
            "fix-123",
            "v1.2_hotfix",
            "用户/分支",
        ] {
            assert!(is_valid_branch_name(ok), "{} should be valid", ok);
        }
        for bad in [
            "",
            "-rf",
            "--force",
            ".hidden",
            "a b",
            "a~b",
            "a^b",
            "a:b",
            "a?b",
            "a*b",
            "a[b",
            "a\\b",
            "a..b",
            "a//b",
            "@{-1}",
            "feature/",
            "topic.lock",
            "HEAD",
        ] {
            assert!(!is_valid_branch_name(bad), "{:?} should be invalid", bad);
        }
    }

    #[test]
    fn branch_payload_needs_a_name() {
        assert_eq!(
            parse_name(br#"{"name":" main "}"#),
            Some("main".to_string())
        );
        assert!(parse_name(br#"{"name":"  "}"#).is_none());
        assert!(parse_name(br#"{"name":1}"#).is_none());
        assert!(parse_name(b"nope").is_none());
    }

    #[test]
    fn conflict_payload_needs_a_path_and_side() {
        assert_eq!(
            parse_conflict(br#"{"path":" a.txt ","side":" Ours "}"#),
            Some(("a.txt".to_string(), "ours".to_string()))
        );
        assert!(parse_conflict(br#"{"path":"a.txt","side":""}"#).is_none());
        assert!(parse_conflict(br#"{"path":"-x","side":"ours"}"#).is_none());
        assert!(parse_conflict(br#"{"path":"a.txt"}"#).is_none());
        assert!(parse_conflict(b"nope").is_none());
    }

    #[test]
    fn tag_payload_needs_a_name_and_optional_message() {
        assert_eq!(
            parse_tag_create(br#"{"name":" v1.0 ","message":" release "}"#),
            Some(("v1.0".to_string(), "release".to_string()))
        );
        assert_eq!(
            parse_tag_create(br#"{"name":"v1.0"}"#),
            Some(("v1.0".to_string(), String::new()))
        );
        assert!(parse_tag_create(br#"{"name":"  "}"#).is_none());
        assert!(parse_tag_create(b"nope").is_none());
    }

    #[test]
    fn log_rows_split_on_unit_separator() {
        let rows = commit_rows(
            "abc1234\x1fAnn\x1f2 days ago\x1ffix x\n\ndef5678\x1fBob\x1fnow\x1fadd y\n",
        );
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0]["hash"], "abc1234");
        assert_eq!(rows[0]["author"], "Ann");
        assert_eq!(rows[0]["when"], "2 days ago");
        assert_eq!(rows[0]["summary"], "fix x");
        assert_eq!(rows[1]["hash"], "def5678");
        assert!(commit_rows("").is_empty());
    }

    #[test]
    fn stash_index_payload_is_a_number() {
        assert_eq!(parse_stash_index(br#"{"index":0}"#), Some(0));
        assert_eq!(parse_stash_index(br#"{"index":3}"#), Some(3));
        assert!(parse_stash_index(br#"{"index":-1}"#).is_none());
        assert!(parse_stash_index(br#"{"index":"0"}"#).is_none());
        assert!(parse_stash_index(br#"{"ref":"stash@{0}"}"#).is_none());
        assert!(parse_stash_index(b"nope").is_none());
    }

    #[test]
    fn remote_payload_needs_a_plain_name() {
        assert_eq!(
            parse_remote(br#"{"remote":" origin "}"#),
            Some("origin".to_string())
        );
        assert!(parse_remote(br#"{"remote":"  "}"#).is_none());
        assert!(parse_remote(br#"{"remote":"--upload-pack=x"}"#).is_none());
        assert!(parse_remote(br#"{"name":"origin"}"#).is_none());
        assert!(parse_remote(b"nope").is_none());
    }

    #[test]
    fn rename_payload_requires_both_names() {
        assert_eq!(
            parse_rename(br#"{"name":" old ","new_name":" new "}"#),
            Some(("old".to_string(), "new".to_string()))
        );
        assert!(parse_rename(br#"{"name":"old"}"#).is_none());
        assert!(parse_rename(br#"{"name":"old","new_name":"  "}"#).is_none());
        assert!(parse_rename(b"nope").is_none());
    }

    #[test]
    fn commit_details_parse_name_status() {
        let (files, truncated) = parse_name_status("M\0a.txt\0R100\0old.txt\0new.txt\0A\0b.txt\0");
        assert!(!truncated);
        assert_eq!(files.len(), 3);
        assert_eq!(files[0]["status"], "M");
        assert_eq!(files[0]["path"], "a.txt");
        assert!(files[0].get("old").is_none());
        assert_eq!(files[1]["status"], "R100");
        assert_eq!(files[1]["old"], "old.txt");
        assert_eq!(files[1]["path"], "new.txt");
        assert_eq!(files[2]["status"], "A");
        assert_eq!(parse_name_status("").0.len(), 0);
    }

    #[test]
    fn commit_hashes_are_validated() {
        assert!(is_valid_commit_hash("8335f77"));
        assert!(is_valid_commit_hash(&"a".repeat(40)));
        assert!(!is_valid_commit_hash("abc"));
        assert!(!is_valid_commit_hash("--help"));
        assert!(!is_valid_commit_hash("HEAD~1"));
        assert!(!is_valid_commit_hash(&"a".repeat(65)));
    }

    #[test]
    fn page_is_embedded() {
        assert!(PAGE.contains("NTC Git"));
        assert!(PAGE.contains("href=\"/about\""));
    }

    #[test]
    fn every_tab_module_is_embedded_and_allowlisted() {
        assert!(PAGE.contains("/app.css"));
        assert!(PAGE.contains("/app.js"));
        assert!(PAGE.contains("/diff.js"));
        assert!(APP_CSS.contains("--rail"));
        assert!(APP_JS.contains("NTC.tabs"));
        assert!(DIFF_JS.contains("NTC.diff"));
        for (name, body) in TAB_ASSETS {
            assert!(
                body.contains("NTC.tabs.register"),
                "{} does not register a tab",
                name
            );
            assert!(
                PAGE.contains(&format!("src=\"/{}\"", name)),
                "{} is not loaded by index.html",
                name
            );
            assert_eq!(tab_asset(name), Some(*body));
        }
    }

    #[test]
    fn unknown_tab_assets_are_not_reachable() {
        for bad in [
            "tabs/nope.js",
            "tabs/../app.js",
            "../app.js",
            "tabs/changes.js/..",
            "app.js",
            "",
        ] {
            assert!(tab_asset(bad).is_none(), "{:?} should not resolve", bad);
        }
    }

    #[test]
    fn tab_ids_are_unique_and_match_their_file_names() {
        let mut ids: Vec<String> = Vec::new();
        for (name, body) in TAB_ASSETS {
            let register = body.split("NTC.tabs.register").last().unwrap_or("");
            let id = register
                .split("id: \"")
                .nth(1)
                .and_then(|rest| rest.split('"').next())
                .unwrap_or("");
            assert!(!id.is_empty(), "{} has no tab id", name);
            assert_eq!(*name, format!("tabs/{}.js", id));
            assert!(!ids.contains(&id.to_string()), "duplicate tab id {}", id);
            ids.push(id.to_string());
        }
        for expected in ["changes", "history", "branches", "remotes", "stash", "tags"] {
            assert!(ids.contains(&expected.to_string()), "missing tab {}", expected);
        }
    }

    #[test]
    fn shell_loads_before_the_tab_modules() {
        let app = PAGE.find("/app.js").expect("app.js is loaded");
        let first_tab = PAGE.find("/tabs/").expect("tab modules are loaded");
        assert!(app < first_tab, "app.js must define NTC before the tabs");
    }

    #[test]
    fn info_page_is_embedded() {
        assert!(INFO_PAGE.contains("About this service"));
        assert!(INFO_PAGE.contains("Rules and safety"));
        assert!(INFO_PAGE.contains("{{ROOT}}"));
    }

    #[test]
    fn info_page_escapes_the_repository_path() {
        assert_eq!(
            html_escape("a&b <c> \"d\""),
            "a&amp;b &lt;c&gt; &quot;d&quot;"
        );
    }
}
