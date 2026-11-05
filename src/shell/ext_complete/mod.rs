//! External-shell integration for the ntc shell.
//!
//! Three features share this module, behind two independent gates:
//!
//! 1. `config integration_shell on` — installed foreign-shell names join
//!    level-1 ghost completion, launcher arguments are completed, and
//!    `ntc ps` mode is available. PowerShell is Windows-only and bash is
//!    desktop-Linux-only; the other shells keep their installed-only
//!    behavior (`platform::supports_shell` is the single gate).
//! 2. `config code_tools on` — curated programming binaries/runtimes that
//!    are installed on `PATH` join level-1 completion and get basic flag and
//!    subcommand hints.
//! 3. `ntc ps [dir]>` — `powershell` / `pwsh` / `powershell.exe` + Enter
//!    keeps ntc in control: lines are executed by a persistent PowerShell
//!    host while ntc's hinter/Tab/accept keys drive PowerShell's completion.
//!    Command names come from a local cache (fetched once); everything else
//!    is completed by one background host worker, so keystrokes never block.
//! 4. `ntc <distro> [dir]>` — on desktop Linux, `bash` + Enter does the same
//!    with a persistent bash host (`arch`, `ubuntu`, `alpine`, … from
//!    `os-release`), so variables, functions and the cwd survive.
//! 5. `ntc zsh [dir]>` — on macOS, `zsh` + Enter does the same with a
//!    persistent zsh host. `brew`, the macOS package manager, joins level-1
//!    completion and gets subcommand/flag hints with the shell integration.
//!
//! Both name registries are **installed-only**: an arbitrary executable on
//! `PATH` is never offered. Providers return **full-line** candidates, so a
//! completion may rewrite text before the cursor; the hinter only keeps those
//! that extend what is already typed (a ghost can only append).

mod bash_host;
mod code_tools;
pub(crate) mod platform;
mod ps_host;
mod shells;
mod zsh_host;

pub(crate) use shells::{bash_completion_available, bash_completion_install_hint};

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use crate::config::Config;

pub use bash_host::cwd as bash_cwd;
pub(crate) use bash_host::kill_host as bash_kill_host;
pub use bash_host::{
    bash_mode_active, enter_bash_mode, host_alive as bash_host_alive, leave_bash_mode,
    run as bash_run_line,
};
pub use ps_host::cwd as ps_cwd;
pub use ps_host::{enter_ps_mode, kill_host, leave_ps_mode, ps_mode_active, run as run_line};
pub use zsh_host::cwd as zsh_cwd;
pub(crate) use zsh_host::kill_host as zsh_kill_host;
pub use zsh_host::{
    enter_zsh_mode, host_alive as zsh_host_alive, leave_zsh_mode, run as zsh_run_line,
    zsh_mode_active,
};

/// Drop every persistent host at shell exit.
pub(crate) fn shutdown_all() {
    ps_host::shutdown();
    bash_host::shutdown();
    zsh_host::shutdown();
}

/// Budget for one *background* PowerShell completion. It never blocks a
/// keypress, so it can be generous enough for `CommandCompletion` to finish.
const ASYNC_BUDGET: Duration = Duration::from_millis(750);
/// Dynamic tool data (branches, scripts, …) is re-queried after this long.
const DYNAMIC_TTL: Duration = Duration::from_secs(5);
/// Cache of full-line candidates, keyed by (line, cursor).
const CACHE_LIMIT: usize = 96;

/// Bumped when the config flag flips so cached PATH names are dropped.
static GENERATION: AtomicU64 = AtomicU64::new(0);
/// Bumped when PowerShell state changes (`ntc ps` ran a line) so background
/// completions started before the change cannot repopulate the cache.
static CACHE_EPOCH: AtomicU64 = AtomicU64::new(0);

struct CacheEntry {
    at: Instant,
    lines: Vec<String>,
}

fn cache() -> &'static Mutex<HashMap<(String, usize), CacheEntry>> {
    static CACHE: OnceLock<Mutex<HashMap<(String, usize), CacheEntry>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn store_cached(before: &str, pos: usize, lines: Vec<String>) {
    if let Ok(mut c) = cache().lock() {
        if c.len() >= CACHE_LIMIT {
            c.clear();
        }
        c.insert(
            (before.to_string(), pos),
            CacheEntry {
                at: Instant::now(),
                lines,
            },
        );
    }
}

/// One queued host completion. Newer keystrokes replace the pending request,
/// so a fast typist never builds up a backlog of stale work.
struct PendingCompletion {
    before: String,
    pos: usize,
    epoch: u64,
    kind: CompletionKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CompletionKind {
    Ps,
    Bash,
    Zsh,
}

fn pending_slot() -> &'static Mutex<Option<PendingCompletion>> {
    static PENDING: OnceLock<Mutex<Option<PendingCompletion>>> = OnceLock::new();
    PENDING.get_or_init(|| Mutex::new(None))
}

static COMPLETION_WORKER_RUNNING: AtomicBool = AtomicBool::new(false);

/// Queue `before` for completion and make sure one worker is draining the
/// queue. Returns immediately — the current keypress shows the previous
/// keystroke's candidates (if any).
fn schedule_completion(before: &str, pos: usize, kind: CompletionKind) {
    if let Ok(mut pending) = pending_slot().lock() {
        *pending = Some(PendingCompletion {
            before: before.to_string(),
            pos,
            epoch: CACHE_EPOCH.load(Ordering::SeqCst),
            kind,
        });
    }
    if COMPLETION_WORKER_RUNNING.swap(true, Ordering::SeqCst) {
        return;
    }
    let spawned = std::thread::Builder::new()
        .name("ntc-host-complete".to_string())
        .spawn(completion_worker_loop);
    if spawned.is_err() {
        COMPLETION_WORKER_RUNNING.store(false, Ordering::SeqCst);
    }
}

fn completion_worker_loop() {
    loop {
        let request = pending_slot()
            .lock()
            .ok()
            .and_then(|mut pending| pending.take());
        let Some(request) = request else {
            COMPLETION_WORKER_RUNNING.store(false, Ordering::SeqCst);
            // A request may have arrived between taking and clearing the flag.
            let has_work = pending_slot()
                .lock()
                .map(|pending| pending.is_some())
                .unwrap_or(false);
            if has_work && !COMPLETION_WORKER_RUNNING.swap(true, Ordering::SeqCst) {
                continue;
            }
            break;
        };
        // Host state changed since this line was queued: the answer would
        // describe the previous cwd/variables.
        if request.epoch != CACHE_EPOCH.load(Ordering::SeqCst) {
            continue;
        }
        let lines: Vec<String> = match request.kind {
            CompletionKind::Ps => {
                let Some(matches) = ps_host::complete(&request.before, request.pos, ASYNC_BUDGET)
                else {
                    continue;
                };
                matches
                    .iter()
                    .filter_map(|m| rebuild_line(&request.before, m))
                    .collect()
            }
            CompletionKind::Bash => {
                let Some(matches) = bash_host::complete(&request.before, request.pos, ASYNC_BUDGET)
                else {
                    continue;
                };
                matches
                    .iter()
                    .filter_map(|m| rebuild_bash_line(&request.before, m))
                    .collect()
            }
            CompletionKind::Zsh => {
                let Some(matches) = zsh_host::complete(&request.before, request.pos, ASYNC_BUDGET)
                else {
                    continue;
                };
                matches
                    .iter()
                    .filter_map(|m| rebuild_byte_line(&request.before, m))
                    .collect()
            }
        };
        let lines = dedup_lines(lines, &request.before);
        if lines.is_empty() || request.epoch != CACHE_EPOCH.load(Ordering::SeqCst) {
            continue;
        }
        store_cached(&request.before, request.pos, lines);
    }
}

fn path_cache() -> &'static Mutex<Option<(u64, Arc<std::collections::HashSet<String>>)>> {
    static PATHS: OnceLock<Mutex<Option<(u64, Arc<std::collections::HashSet<String>>)>>> =
        OnceLock::new();
    PATHS.get_or_init(|| Mutex::new(None))
}

/// True when `config integration_shell on`.
pub(crate) fn integration_enabled() -> bool {
    Config::global_get_integration_shell_enabled()
}

/// True when `config code_tools on`.
pub(crate) fn code_tools_enabled() -> bool {
    Config::global_get_code_tools_enabled()
}

/// Installed foreign-shell names for level-1 completion.
pub(crate) fn shell_names() -> Vec<String> {
    shells::installed_names()
}

/// Installed curated code-tool names for level-1 completion.
pub(crate) fn code_tool_names() -> Vec<String> {
    code_tools::installed_names()
}

/// macOS's package manager, hinted with the shell integration (zsh mode)
/// instead of the programming-tools gate.
pub(crate) fn package_manager_name() -> Option<&'static str> {
    if platform::is_macos() && is_installed("brew") {
        Some("brew")
    } else {
        None
    }
}

/// Drop every cached value (called when a flag changes).
pub(crate) fn invalidate() {
    GENERATION.fetch_add(1, Ordering::SeqCst);
    CACHE_EPOCH.fetch_add(1, Ordering::SeqCst);
    if let Ok(mut c) = cache().lock() {
        c.clear();
    }
    if let Ok(mut p) = path_cache().lock() {
        *p = None;
    }
    shells::invalidate();
    code_tools::invalidate();
    ps_host::reset_probe();
    ps_host::invalidate_names();
    bash_host::reset_probe();
    bash_host::invalidate_names();
    zsh_host::reset_probe();
    zsh_host::invalidate_names();
    // Level-1 completion caches registry names alongside aliases.
    super::hinter::bump_generation();
}

/// Forget delegated completion answers without touching the installed-name
/// cache. Called after each host command: the cwd, variables and functions
/// moved, so answers cached for the old state must not survive.
pub(crate) fn clear_line_cache() {
    CACHE_EPOCH.fetch_add(1, Ordering::SeqCst);
    if let Ok(mut c) = cache().lock() {
        c.clear();
    }
    shells::invalidate();
    code_tools::invalidate();
}

/// Lowercased executable stems found on `PATH`, cached until the config
/// generation changes. Both curated registries match against this set, so
/// only installed names are ever hinted.
fn path_stems() -> Arc<std::collections::HashSet<String>> {
    let current = GENERATION.load(Ordering::SeqCst);
    if let Ok(guard) = path_cache().lock() {
        if let Some((generation, names)) = guard.as_ref() {
            if *generation == current {
                return Arc::clone(names);
            }
        }
    }
    let names = Arc::new(scan_path());
    if let Ok(mut guard) = path_cache().lock() {
        *guard = Some((current, Arc::clone(&names)));
    }
    names
}

/// True when `name` (any casing, launcher extension ignored) is installed.
pub(crate) fn is_installed(name: &str) -> bool {
    path_stems().contains(&name.to_ascii_lowercase())
}

/// Target-triple prefixes used by cross toolchains. Binaries named after
/// their target (`x86_64-w64-mingw32-gcc`, `i686-linux-gnu-gcc`) are never
/// useful command hints: they only bury the real `gcc` under an architecture
/// prefix. Filtered on every platform and in every host shell.
const TARGET_TRIPLE_PREFIXES: &[&str] = &[
    "x86_64",
    "x86-64",
    "i686",
    "i586",
    "i486",
    "i386",
    "aarch64",
    "arm64",
    "armv6",
    "armv7",
    "arm-linux",
    "arm-none",
    "mips",
    "mipsel",
    "powerpc",
    "ppc64",
    "riscv64",
    "s390x",
    "sparc",
];

/// True when `name` names a cross-toolchain binary (`x86_64*`, `i686*`, …).
pub(crate) fn is_target_triple_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    TARGET_TRIPLE_PREFIXES
        .iter()
        .any(|prefix| lower.starts_with(prefix))
}

/// Extensions that hold data, not programs. WSL's DrvFs marks every Windows
/// file executable, so the execute-bit test alone would offer `file.png`,
/// `file.dll` or `file.mof` as runnable commands. Windows launchers (`.exe`,
/// `.com`) are in the list too: a mounted drive under WSL or Termux does not
/// make them Linux commands, so they are never hinted as one.
#[cfg(not(windows))]
const NON_PROGRAM_EXTENSIONS: &[&str] = &[
    "md", "markdown", "txt", "text", "rst", "pdf", "doc", "docx", "rtf", "odt", "log", "mof",
    "ini", "cfg", "conf", "toml", "yaml", "yml", "json", "jsonc", "json5", "xml", "xsl", "xsd",
    "html", "htm", "css", "scss", "less", "csv", "tsv", "nfo", "lock", "bak", "tmp", "swp", "orig",
    "rej", "patch", "diff", "map", "png", "jpg", "jpeg", "gif", "bmp", "svg", "ico", "webp", "tif",
    "tiff", "psd", "xcf", "mp3", "mp4", "mkv", "avi", "mov", "wav", "flac", "ogg", "webm", "zip",
    "gz", "tgz", "tar", "xz", "bz2", "7z", "rar", "zst", "lz4", "cab", "msi", "iso", "deb", "rpm",
    "pkg", "dmg", "apk", "jar", "war", "dll", "so", "dylib", "a", "lib", "o", "obj", "exp", "pdb",
    "ilk", "def", "class", "pyc", "pyo", "exe", "com",
];

/// True when a file name looks like a program rather than a data file.
#[cfg(not(windows))]
fn is_program_file_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    // Versioned shared objects (`libncurses.so.6`) end in a numeric suffix.
    if lower.contains(".so.") || lower.contains(".dylib.") {
        return false;
    }
    match name.rsplit_once('.') {
        // A leading dot is not an extension (`.profile` is a script).
        Some((stem, ext)) if !stem.is_empty() => !NON_PROGRAM_EXTENSIONS
            .iter()
            .any(|known| ext.eq_ignore_ascii_case(known)),
        _ => true,
    }
}

fn scan_path() -> std::collections::HashSet<String> {
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let Some(path) = std::env::var_os("PATH") else {
        return seen;
    };
    let extensions = executable_extensions();
    for dir in std::env::split_paths(&path) {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let is_file = entry.file_type().map(|t| t.is_file()).unwrap_or(false);
            if !is_file {
                continue;
            }
            let name = entry.file_name();
            let Some(name) = name.to_str() else {
                continue;
            };
            if name.starts_with('.') {
                continue;
            }
            let Some(stem) = executable_stem(&entry, name, &extensions) else {
                continue;
            };
            if stem.is_empty() || is_target_triple_name(stem) {
                continue;
            }
            seen.insert(stem.to_ascii_lowercase());
        }
    }
    seen
}

fn executable_extensions() -> Vec<String> {
    if cfg!(windows) {
        let raw = std::env::var("PATHEXT").unwrap_or_else(|_| {
            ".COM;.EXE;.BAT;.CMD;.VBS;.VBE;.JS;.JSE;.WSF;.WSH;.MSC;.PS1".to_string()
        });
        raw.split(';')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_start_matches('.').to_lowercase())
            .collect()
    } else {
        Vec::new()
    }
}

/// Windows: strip a `PATHEXT` suffix (any casing). Non-executable files
/// (`README.md`, `*.dll`) are skipped rather than offered as commands.
#[cfg(windows)]
fn executable_stem<'a>(
    _entry: &std::fs::DirEntry,
    name: &'a str,
    extensions: &[String],
) -> Option<&'a str> {
    let lower = name.to_ascii_lowercase();
    for ext in extensions {
        let suffix = format!(".{}", ext);
        if lower.len() > suffix.len() && lower.ends_with(&suffix) {
            return name.get(..name.len() - suffix.len());
        }
    }
    None
}

/// Unix: keep names with an executable bit and a program-like extension.
#[cfg(not(windows))]
fn executable_stem<'a>(
    entry: &std::fs::DirEntry,
    name: &'a str,
    _extensions: &[String],
) -> Option<&'a str> {
    use std::os::unix::fs::PermissionsExt;
    let mode = entry.metadata().ok()?.permissions().mode();
    if mode & 0o111 == 0 {
        return None;
    }
    if !is_program_file_name(name) {
        return None;
    }
    Some(name)
}

/// PowerShell launcher spellings that mean "integrate", not "spawn".
pub(crate) fn is_powershell(cmd: &str) -> bool {
    let base = cmd
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(cmd)
        .to_ascii_lowercase();
    matches!(
        base.as_str(),
        "powershell" | "pwsh" | "powershell.exe" | "pwsh.exe"
    )
}

/// Bare `bash` (optionally pathed) means "enter bash mode", not "spawn".
pub(crate) fn is_bash(cmd: &str) -> bool {
    let base = cmd
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(cmd)
        .to_ascii_lowercase();
    base == "bash"
}

/// Bare `zsh` (optionally pathed) means "enter zsh mode", not "spawn".
pub(crate) fn is_zsh(cmd: &str) -> bool {
    let base = cmd
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(cmd)
        .to_ascii_lowercase();
    base == "zsh"
}

/// True when a curated tool owns `base` on this line. `brew` is macOS's
/// package manager: it ships with the zsh integration, so either gate is
/// enough for it (`code_tools` also covers it for Linuxbrew users).
fn code_tool_owned(base: &str) -> bool {
    if !code_tools::owns(base) {
        return false;
    }
    if code_tools_enabled() {
        return true;
    }
    base.eq_ignore_ascii_case("brew")
        && platform::is_macos()
        && integration_enabled()
        && is_installed("brew")
}

/// Full-line candidates for `line[..pos]`, or `None` when no provider owns
/// the command (the caller then keeps ntc's own completion).
pub(crate) fn line_candidates(line: &str, pos: usize) -> Option<Vec<String>> {
    if !ps_mode_active()
        && !bash_mode_active()
        && !zsh_mode_active()
        && !integration_enabled()
        && !code_tools_enabled()
    {
        return None;
    }
    let pos = pos.min(line.len());
    if !line.is_char_boundary(pos) {
        return None;
    }
    let before = &line[..pos];
    if before.trim().is_empty() {
        return None;
    }

    if let Ok(c) = cache().lock() {
        if let Some(entry) = c.get(&(before.to_string(), pos)) {
            if entry.at.elapsed() < DYNAMIC_TTL {
                return Some(entry.lines.clone());
            }
        }
    }

    let lines = compute(before, pos)?;
    let lines = dedup_lines(lines, before);
    store_cached(before, pos, lines.clone());
    Some(lines)
}

/// Slow-completion fallback: reuse the candidates cached for a shorter
/// prefix of the same line and keep only the ones that still extend it.
/// The ghost then trails one keystroke instead of vanishing while the
/// external engine catches up.
fn stale_fallback(before: &str) -> Option<Vec<String>> {
    let cache = cache().lock().ok()?;
    let before_lower = before.to_ascii_lowercase();
    let mut best: Option<&CacheEntry> = None;
    for ((line, _), entry) in cache.iter() {
        if line.len() < before.len() && before_lower.starts_with(&line.to_ascii_lowercase()) {
            if best.map(|b| b.at < entry.at).unwrap_or(true) {
                best = Some(entry);
            }
        }
    }
    let entry = best?;
    let lines: Vec<String> = entry
        .lines
        .iter()
        .filter(|l| l.len() > before.len() && l.to_ascii_lowercase().starts_with(&before_lower))
        .cloned()
        .collect();
    if lines.is_empty() {
        None
    } else {
        Some(lines)
    }
}

/// True when an external provider owns this command line, so its candidates
/// take priority over ntc's filesystem fallback.
pub(crate) fn owns_line(line: &str) -> bool {
    if ps_mode_active() || bash_mode_active() || zsh_mode_active() {
        return true;
    }
    let Some((start, end)) = first_word(line) else {
        return false;
    };
    let cmd = &line[start..end];
    let base = command_base(cmd);
    if shells::owns(&base) {
        return integration_enabled();
    }
    code_tool_owned(&base)
}

/// Executable name without directory or launcher extension, lowercased.
fn command_base(cmd: &str) -> String {
    let base = Path::new(cmd)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(cmd)
        .to_ascii_lowercase();
    base.strip_suffix(".exe")
        .or_else(|| base.strip_suffix(".cmd"))
        .or_else(|| base.strip_suffix(".bat"))
        .unwrap_or(&base)
        .to_string()
}

fn compute(before: &str, pos: usize) -> Option<Vec<String>> {
    // Host modes: the foreign shell owns the whole line, operators included.
    if ps_mode_active() {
        return ps_line_candidates(before, pos);
    }
    if bash_mode_active() {
        return bash_line_candidates(before, pos);
    }
    if zsh_mode_active() {
        return zsh_line_candidates(before, pos);
    }

    let (cmd_start, cmd_end) = first_word(before)?;
    let cmd = &before[cmd_start..cmd_end];
    let base = command_base(cmd);

    if shells::owns(&base) {
        if !integration_enabled() {
            return None;
        }
        // Launcher flags (`powershell -NoP`, `cmd /C`, `bash -`) belong to
        // the shell itself; other arguments are delegated to its engine.
        if let Some(cands) = shells::complete(&base, before, pos) {
            return Some(cands);
        }
        if is_powershell(&base) {
            // Feed only the argument tail to PowerShell, then paste it back, so
            // `powershell Get-Chi` completes `Get-ChildItem` rather than trying
            // to complete the `powershell` launcher itself.
            if cmd_end >= pos {
                return Some(Vec::new());
            }
            let args = &before[cmd_end..];
            let args_pos = pos - cmd_end;
            let cands = ps_line_candidates(args, args_pos)?;
            let prefix = &before[..cmd_end];
            return Some(
                cands
                    .into_iter()
                    .map(|c| format!("{}{}", prefix, c))
                    .collect(),
            );
        }
        return None;
    }

    if code_tool_owned(&base) {
        return code_tools::complete(&base, before, pos);
    }
    None
}

fn ps_line_candidates(before: &str, pos: usize) -> Option<Vec<String>> {
    // Fast path: a plain command token is completed from the local name
    // cache, so `g` / `Get-` never wait for `CommandCompletion`.
    if let Some(cands) = local_command_candidates(
        before,
        pos,
        ps_host::cached_command_names(),
        ps_host::prefetch_command_names,
    ) {
        return Some(cands);
    }
    // Everything else (parameters, paths, variables, expressions) runs in the
    // background; this keystroke keeps the previous answer instead of blocking.
    schedule_completion(before, pos, CompletionKind::Ps);
    stale_fallback(before)
}

fn bash_line_candidates(before: &str, pos: usize) -> Option<Vec<String>> {
    // The persistent host answers in about a millisecond, but a plain command
    // token is still cheaper from the local cache and keeps `g`/`git` instant.
    if let Some(cands) = local_command_candidates(
        before,
        pos,
        bash_host::cached_command_names(),
        bash_host::prefetch_command_names,
    ) {
        return Some(cands);
    }
    schedule_completion(before, pos, CompletionKind::Bash);
    stale_fallback(before)
}

fn zsh_line_candidates(before: &str, pos: usize) -> Option<Vec<String>> {
    if let Some(cands) = local_command_candidates(
        before,
        pos,
        zsh_host::cached_command_names(),
        zsh_host::prefetch_command_names,
    ) {
        return Some(cands);
    }
    // Curated flags and subcommands (`grep -`, `brew ins`, `git ch`, …) are
    // answered in Rust, so zsh hints never depend on the user's `compinit`
    // setup. In zsh mode they belong to the shell integration (the host has no
    // other argument engine), so `integration_shell on` is enough. Paths and
    // `$`-variables fall through to the host.
    if let Some((start, end)) = first_word(before) {
        let cmd = &before[start..end];
        let base = command_base(cmd);
        if code_tools::owns(&base) && (code_tools_enabled() || integration_enabled()) {
            if let Some(cands) = code_tools::complete(&base, before, pos) {
                if !cands.is_empty() {
                    return Some(cands);
                }
            }
        }
    }
    schedule_completion(before, pos, CompletionKind::Zsh);
    stale_fallback(before)
}

/// Local, zero-round-trip completion for the first word of a host line.
///
/// Only a single unquoted token is handled; once a space is typed the line is
/// a parameter/path context and goes through the host. Leading whitespace is
/// preserved (the `powershell -Command` launcher path passes `" Get-Chi"`).
fn local_command_candidates(
    before: &str,
    pos: usize,
    names: Option<Arc<Vec<String>>>,
    prefetch: fn(),
) -> Option<Vec<String>> {
    if pos != before.len() {
        return None;
    }
    let start = before.find(|c: char| !c.is_whitespace())?;
    let token = &before[start..];
    if token.is_empty() || token.chars().any(char::is_whitespace) {
        return None;
    }
    // Operators, variables, script blocks, paths and flags are not command
    // names; let the shell handle their richer syntax.
    if token.starts_with([
        '-', '$', '@', '"', '\'', '&', '.', '/', '\\', ':', '(', '{', '[', '`',
    ]) {
        return None;
    }

    let Some(names) = names else {
        // First use: warm the cache and let the background host answer now.
        prefetch();
        return None;
    };

    let lower = token.to_ascii_lowercase();
    let prefix = &before[..start];
    let mut out = prefix_matches(before, start, token, &names);
    // Both shells also complete executables on PATH; reuse ntc's existing scan
    // so `git` / `python` stay instant inside the host modes too.
    for stem in path_stems().iter() {
        if stem.len() > token.len() && stem.starts_with(&lower) {
            out.push(format!("{}{}", prefix, stem));
        }
    }
    if out.is_empty() {
        // No local match: a module-qualified or otherwise unusual command may
        // still exist, so give the host a chance.
        None
    } else {
        Some(out)
    }
}

/// Case-insensitive prefix matches against a command-name list, with the
/// typed prefix (including leading whitespace) preserved.
fn prefix_matches(before: &str, start: usize, token: &str, names: &[String]) -> Vec<String> {
    let lower = token.to_ascii_lowercase();
    let prefix = &before[..start];
    names
        .iter()
        .filter(|name| name.len() > token.len() && name.to_ascii_lowercase().starts_with(&lower))
        .map(|name| format!("{}{}", prefix, name))
        .collect()
}

/// Rebuild a full line from a PowerShell replacement applied to `before`.
fn rebuild_line(before: &str, m: &ps_host::PsMatch) -> Option<String> {
    let start = ps_host::utf16_to_byte(before, m.replacement_index)?;
    let end = ps_host::utf16_to_byte(before, m.replacement_index + m.replacement_length)?;
    if end < start {
        return None;
    }
    Some(format!("{}{}{}", &before[..start], m.text, &before[end..]))
}

/// Rebuild a full line from a bash replacement (already byte-indexed).
fn rebuild_bash_line(before: &str, m: &bash_host::Match) -> Option<String> {
    byte_replacement(before, m.start, m.length, &m.text)
}

/// Rebuild a full line from a zsh replacement (already byte-indexed).
fn rebuild_byte_line(before: &str, m: &zsh_host::Match) -> Option<String> {
    byte_replacement(before, m.start, m.length, &m.text)
}

fn byte_replacement(before: &str, start: usize, length: usize, text: &str) -> Option<String> {
    let end = start.checked_add(length)?;
    if end > before.len() || !before.is_char_boundary(start) || !before.is_char_boundary(end) {
        return None;
    }
    Some(format!("{}{}{}", &before[..start], text, &before[end..]))
}

fn dedup_lines(mut lines: Vec<String>, typed: &str) -> Vec<String> {
    // PowerShell completes case-insensitively (`get-c` → `Get-Content`), so
    // compare full lines without regard to case.
    let typed_lower = typed.to_ascii_lowercase();
    lines.retain(|l| l.len() > typed.len() && l.to_ascii_lowercase().starts_with(&typed_lower));
    // A host engine can still hand back a cross-toolchain binary; never hint
    // one, whichever shell answered.
    lines.retain(|l| !hints_target_triple(l));
    // Progressive ghosts want a short, cmdlet-like candidate first (`g` should
    // grow toward `Get`, not toward a long alias or path fragment).
    lines.sort_by_cached_key(|line| rank_line(line));
    lines.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    lines
}

/// True when the token a candidate would insert is a cross-toolchain binary.
fn hints_target_triple(line: &str) -> bool {
    let token = line
        .rsplit([' ', '\t', '|', ';', '&'])
        .next()
        .unwrap_or(line);
    let name = token.rsplit(['/', '\\']).next().unwrap_or(token);
    is_target_triple_name(name)
}

/// Ordering for command candidates: canonical PowerShell verb-noun names
/// first, then other hyphenated names (executables such as `gcc-ar`), then
/// plain words; within a group shortest wins, then alphabetical. The first
/// item is the ghost's initial pick.
fn rank_line(line: &str) -> (u8, usize, String) {
    let token = line
        .rsplit([' ', '\t', '|', ';', '&'])
        .next()
        .unwrap_or(line);
    let group = if token.starts_with('-') || !token.contains('-') {
        2
    } else if token
        .chars()
        .next()
        .map(|c| c.is_ascii_uppercase())
        .unwrap_or(false)
    {
        // `Get-Acl`, `Set-Item`: the canonical PowerShell spelling.
        0
    } else {
        // `gcc-ar`, `git-lfs`: a hyphenated executable, still stageable.
        1
    };
    (group, line.len(), line.to_ascii_lowercase())
}

/// Byte range of the first whitespace-separated word (quote-aware).
fn first_word(s: &str) -> Option<(usize, usize)> {
    let bytes = s.as_bytes();
    let start = bytes.iter().position(|b| !b.is_ascii_whitespace())?;
    let mut quote: Option<u8> = None;
    let mut i = start;
    while i < bytes.len() {
        let b = bytes[i];
        match quote {
            Some(q) if b == q => quote = None,
            Some(_) => {}
            None if b == b'"' || b == b'\'' => quote = Some(b),
            None if b.is_ascii_whitespace() => return Some((start, i)),
            None => {}
        }
        i += 1;
    }
    Some((start, s.len()))
}

/// Start of the token being typed (`s.len()` after a trailing space).
fn last_token_start(s: &str) -> usize {
    let trimmed_len = s.trim_end().len();
    if trimmed_len == 0 {
        return 0;
    }
    if s.ends_with(char::is_whitespace) {
        return s.len();
    }
    s[..trimmed_len]
        .rfind(char::is_whitespace)
        .map(|i| i + 1)
        .unwrap_or(0)
}

/// Turn token candidates (`git` subcommands, branch names, …) into full
/// lines by replacing the token being typed.
fn tokens_to_lines(before: &str, tokens: Vec<String>) -> Vec<String> {
    let start = last_token_start(before);
    let frag = &before[start..];
    let lower = frag.to_lowercase();
    tokens
        .into_iter()
        .filter(|t| {
            let tl = t.to_lowercase();
            tl.starts_with(&lower) && t.len() > frag.len()
        })
        .map(|t| format!("{}{}", &before[..start], t))
        .collect()
}

fn cwd() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Run a one-shot command and return its stdout lines. Used by the dynamic
/// tool providers (git branches, npm scripts, …).
fn run_capture(program: &str, args: &[&str]) -> Option<String> {
    let out = std::process::Command::new(program)
        .args(args)
        .current_dir(cwd())
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

fn nonempty_lines(text: &str) -> Vec<String> {
    text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_is_quote_aware() {
        assert_eq!(first_word("  git status"), Some((2, 5)));
        assert_eq!(first_word("\"C:\\Program Files\\x.exe\" a"), Some((0, 24)));
        assert_eq!(first_word("   "), None);
    }

    #[test]
    fn last_token_start_handles_trailing_space() {
        assert_eq!(last_token_start("git "), 4);
        assert_eq!(last_token_start("git ch"), 4);
        assert_eq!(last_token_start("git"), 0);
        assert_eq!(last_token_start(""), 0);
    }

    #[test]
    fn tokens_become_full_lines() {
        let lines = tokens_to_lines("git ch", vec!["checkout".into(), "cherry-pick".into()]);
        assert_eq!(lines, vec!["git checkout", "git cherry-pick"]);
        let lines = tokens_to_lines("git ", vec!["status".into()]);
        assert_eq!(lines, vec!["git status"]);
        // Exact matches are dropped (no ghost to show).
        assert!(tokens_to_lines("git status", vec!["status".into()]).is_empty());
    }

    #[test]
    fn ps_replacement_rebuilds_the_line() {
        let m = ps_host::PsMatch {
            text: "Get-ChildItem".into(),
            replacement_index: 0,
            replacement_length: 7,
        };
        assert_eq!(
            rebuild_line("Get-Chi", &m).as_deref(),
            Some("Get-ChildItem")
        );
        let m = ps_host::PsMatch {
            text: "ChildItem".into(),
            replacement_index: 4,
            replacement_length: 3,
        };
        assert_eq!(
            rebuild_line("Get-Chi", &m).as_deref(),
            Some("Get-ChildItem")
        );
    }

    #[test]
    fn powershell_spellings_and_ownership() {
        for name in [
            "powershell",
            "pwsh",
            "powershell.exe",
            "PWSh",
            "C:\\x\\pwsh.exe",
        ] {
            assert!(is_powershell(name), "{name}");
        }
        assert!(!is_powershell("powershellx"));
        assert!(!is_powershell("get-process"));
    }

    #[test]
    fn package_manager_is_macos_only() {
        if platform::is_macos() {
            assert_eq!(package_manager_name().is_some(), is_installed("brew"));
        } else {
            assert_eq!(package_manager_name(), None);
        }
    }

    #[test]
    fn zsh_spellings_enter_the_host_mode() {
        for name in ["zsh", "ZSH", "/bin/zsh"] {
            assert!(is_zsh(name), "{name}");
        }
        assert!(!is_zsh("zshx"));
        assert!(!is_zsh("bash"));
        assert!(!is_bash("zsh"));
        assert!(is_bash("bash"));
    }

    #[test]
    fn git_subcommands_complete_with_code_tools() {
        // The host modes are process-global flags; serialize with the tests
        // that toggle them.
        let _lock = crate::utils::test_support::lock_globals();
        let mut cfg = Config::new();
        cfg.code_tools_enabled = true;
        Config::with_test_config(cfg, || {
            invalidate();
            let cands = line_candidates("git ch", 6).expect("git is handled");
            assert!(cands.contains(&"git checkout".to_string()), "{cands:?}");
        });
        let off = Config::new();
        Config::with_test_config(off, || {
            invalidate();
            assert!(line_candidates("git ch", 6).is_none());
        });
    }

    #[test]
    fn integration_shell_does_not_enable_code_tools() {
        let _lock = crate::utils::test_support::lock_globals();
        let mut cfg = Config::new();
        cfg.integration_shell_enabled = true;
        Config::with_test_config(cfg, || {
            invalidate();
            assert!(line_candidates("g++ -", 5).is_none());
            // Shell launchers are handled by the other gate.
            let cands = line_candidates("cmd /", 5).expect("cmd is handled");
            assert!(cands.contains(&"cmd /C".to_string()), "{cands:?}");
        });
    }

    #[test]
    fn code_tools_flag_hints_work() {
        let _lock = crate::utils::test_support::lock_globals();
        let mut cfg = Config::new();
        cfg.code_tools_enabled = true;
        Config::with_test_config(cfg, || {
            invalidate();
            let cands = line_candidates("g++ -", 5).expect("g++ is handled");
            assert!(cands.contains(&"g++ -std=c++17".to_string()), "{cands:?}");
            let cands = line_candidates("python -", 8).expect("python is handled");
            assert!(cands.contains(&"python -m".to_string()), "{cands:?}");
            // Common shell utilities are curated too.
            let cands = line_candidates("grep -", 6).expect("grep is handled");
            assert!(cands.contains(&"grep --recursive".to_string()), "{cands:?}");
            let cands = line_candidates("rm --", 5).expect("rm is handled");
            assert!(cands.contains(&"rm --force".to_string()), "{cands:?}");
        });
    }

    #[test]
    fn arch_prefixed_binaries_are_never_hinted() {
        assert!(is_target_triple_name("x86_64-linux-gnu-gcc"));
        assert!(is_target_triple_name("X86-64-w64-mingw32-gcc"));
        assert!(is_target_triple_name("i686-w64-mingw32-gcc"));
        assert!(!is_target_triple_name("gcc"));
        assert!(!is_target_triple_name("xclip"));
        let out = dedup_lines(vec!["x86_64-linux-gnu-gcc".into()], "x86");
        assert!(out.is_empty(), "{out:?}");
        let out = dedup_lines(vec!["x86_64".into()], "x");
        assert!(out.is_empty(), "{out:?}");
    }

    #[test]
    fn dedup_keeps_only_extensions_of_typed_text() {
        let out = dedup_lines(
            vec![
                "git checkout".into(),
                "git checkout".into(),
                "git status".into(),
            ],
            "git ch",
        );
        assert_eq!(out, vec!["git checkout"]);
    }

    /// PowerShell completes case-insensitively: lowercased input must keep
    /// canonical cmdlet casing.
    #[test]
    fn dedup_is_case_insensitive() {
        let out = dedup_lines(
            vec![
                "Get-Content".into(),
                "get-content".into(),
                "Get-Command".into(),
            ],
            "get-c",
        );
        assert_eq!(out, vec!["Get-Command", "Get-Content"]);
    }

    #[test]
    fn local_prefix_prefers_a_short_cmdlet_for_the_first_ghost() {
        let names = vec![
            "gci".to_string(),
            "Get-ChildItem".to_string(),
            "Get-Acl".to_string(),
        ];
        let candidates = prefix_matches("g", 0, "g", &names);
        let candidates = dedup_lines(candidates, "g");
        assert_eq!(candidates.first().map(String::as_str), Some("Get-Acl"));
    }

    #[test]
    fn local_prefix_preserves_launcher_whitespace_and_case() {
        let names = vec!["Get-ChildItem".to_string()];
        let candidates = prefix_matches(" Get-chi", 1, "Get-chi", &names);
        assert_eq!(candidates, vec![" Get-ChildItem"]);
    }

    #[test]
    fn ps_mode_bare_g_has_a_local_candidate() {
        if !ps_host::available() {
            return;
        }
        let _lock = crate::utils::test_support::lock_globals();
        ps_host::enter_ps_mode().expect("host should start");
        clear_line_cache();
        let cands = line_candidates("g", 1).expect("g should be handled");
        let first = cands.first().expect("no candidate for g");
        assert!(
            first.to_ascii_lowercase().starts_with("get-"),
            "expected a Get-* cmdlet first, got {first:?}"
        );
        ps_host::leave_ps_mode();
        clear_line_cache();
    }

    #[test]
    fn real_powershell_names_complete_a_bare_g() {
        if !ps_host::available() {
            return;
        }
        let _lock = crate::utils::test_support::lock_globals();
        let names = ps_host::command_names(Duration::from_secs(30)).expect("names request failed");
        let candidates = prefix_matches("g", 0, "g", &names);
        let candidates = dedup_lines(candidates, "g");
        let first = candidates.first().expect("no candidate for g");
        assert!(
            first.to_ascii_lowercase().starts_with("get-"),
            "expected a Get-* cmdlet first, got {first:?}"
        );
    }

    #[test]
    fn rank_prefers_verb_noun_over_aliases_and_paths() {
        assert!(
            rank_line("gci") > rank_line("Get-Acl"),
            "verb-noun cmdlet should rank before an alias"
        );
        assert!(
            rank_line("gcc-ar") > rank_line("Get-Acl"),
            "hyphenated executable should rank after a canonical cmdlet"
        );
        assert!(rank_line("Get-Acl") < rank_line("Get-ChildItem"));
    }

    #[test]
    fn installed_names_are_curated_and_installed_only() {
        let _lock = crate::utils::test_support::lock_globals();
        let tmp = tempfile::tempdir().expect("tempdir");
        let exe = |stem: &str| {
            if cfg!(windows) {
                tmp.path().join(format!("{}.exe", stem))
            } else {
                tmp.path().join(stem)
            }
        };
        for stem in ["ntcfixture", "Rscript", "bash"] {
            let fixture = exe(stem);
            std::fs::write(&fixture, "").expect("write fixture");
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&fixture).unwrap().permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&fixture, perms).unwrap();
            }
        }
        // Data files next to them must not be offered as a command. Their
        // execute bit is set below on Unix: WSL's DrvFs marks every Windows
        // file executable, which is exactly the case being guarded.
        for name in [
            "README.md",
            "file.png",
            "file.dll",
            "file.mof",
            "libfoo.so",
            "libncurses.so.6",
            "bundle.tar.gz",
        ] {
            let path = tmp.path().join(name);
            std::fs::write(&path, "data").expect("write data file");
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&path).unwrap().permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&path, perms).unwrap();
            }
        }
        // Cross-toolchain binaries are installed programs, but never hints.
        for stem in [
            "x86_64-linux-gnu-gcc",
            "x86-64-w64-mingw32-gcc",
            "i686-w64-mingw32-gcc",
        ] {
            let path = exe(stem);
            std::fs::write(&path, "").expect("write arch fixture");
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&path).unwrap().permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&path, perms).unwrap();
            }
        }

        // Scan only the fixture directory: the host PATH may itself contain
        // documentation (WSL on drvfs marks every file executable), which
        // would make the assertions below describe the machine, not the code.
        let old = std::env::var_os("PATH");
        std::env::set_var("PATH", tmp.path());
        invalidate();

        assert!(is_installed("ntcfixture"));
        // Curated names keep their canonical casing even when the file on
        // disk uses different casing.
        let tools = code_tools::installed_names();
        assert!(tools.iter().any(|n| n == "Rscript"), "{tools:?}");
        assert!(!tools.iter().any(|n| n == "ntcfixture"), "{tools:?}");
        let shells = shells::installed_names();
        // bash is hinted only on a bash platform (desktop Linux or Termux);
        // elsewhere the fixture must stay out even though the executable
        // exists on PATH.
        assert_eq!(
            shells.iter().any(|n| n == "bash"),
            platform::is_bash_platform(),
            "{shells:?}"
        );
        assert!(!shells.iter().any(|n| n == "ntcfixture"), "{shells:?}");
        let stems = path_stems();
        assert!(!stems.iter().any(|n| n.starts_with("readme")));
        for name in [
            "file.png",
            "file.dll",
            "file.mof",
            "libfoo.so",
            "libncurses.so.6",
            "bundle.tar.gz",
            "x86_64-linux-gnu-gcc",
            "x86-64-w64-mingw32-gcc",
            "i686-w64-mingw32-gcc",
        ] {
            assert!(!stems.contains(name), "{name} leaked into {stems:?}");
        }
        // Real programs in the same directory keep being hinted.
        assert!(stems.contains("ntcfixture"), "{stems:?}");

        match old {
            Some(v) => std::env::set_var("PATH", v),
            None => std::env::remove_var("PATH"),
        }
        invalidate();
    }
}
