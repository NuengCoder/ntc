//! Persistent PowerShell host used by `integration_shell`.
//!
//! A warm child process is started on first use and driven over a tiny
//! line protocol (`<id>\t<op>\t<payload>`), so completion stays in the
//! low-millisecond range and `ntc ps` keeps its state (`$x`, functions,
//! `$PWD`) between commands. PowerShell itself never renders a prompt:
//! the bootstrap loop only reads requests and writes id-tagged replies.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};

/// One completion match as reported by `CommandCompletion.CompleteInput`.
#[derive(Debug, Clone)]
pub struct PsMatch {
    pub text: String,
    /// UTF-16 code-unit index where the replacement starts.
    pub replacement_index: usize,
    /// UTF-16 code-unit length being replaced.
    pub replacement_length: usize,
}

/// Reply-to-request marker. Anything else on stdout is command output.
const MARKER: &str = "@@NTC@@";

/// Set while the shell is in `ntc ps` mode (see `ext_complete`).
pub(crate) static PS_MODE: AtomicBool = AtomicBool::new(false);
/// PID of the live host, 0 when none.
static HOST_PID: AtomicU32 = AtomicU32::new(0);
/// True when the host could not be started (missing PowerShell).
static UNAVAILABLE: AtomicBool = AtomicBool::new(false);
/// Command names fetched once per session for the fast local prefix layer.
static NAMES: OnceLock<Mutex<Option<Arc<Vec<String>>>>> = OnceLock::new();
/// At most one background name fetch at a time.
static NAMES_PREFETCHING: AtomicBool = AtomicBool::new(false);
/// Bumped when the name cache is invalidated so an in-flight fetch cannot
/// repopulate it with stale data.
static NAMES_GENERATION: AtomicU64 = AtomicU64::new(0);

struct Host {
    child: Child,
    stdin: ChildStdin,
    rx: Receiver<String>,
    next_id: u64,
}

/// Bootstrap run inside PowerShell. Receives `-EncodedCommand` (UTF-16LE
/// base64), so no host-side quoting rules leak in here.
const BOOTSTRAP: &str = r#"
$ErrorActionPreference = 'Continue'
try {
    [Console]::InputEncoding = New-Object System.Text.UTF8Encoding $false
    [Console]::OutputEncoding = New-Object System.Text.UTF8Encoding $false
    $OutputEncoding = [Console]::OutputEncoding
} catch { }
function __ntc_esc([string]$s) {
    $sb = New-Object System.Text.StringBuilder
    foreach ($ch in $s.ToCharArray()) {
        $code = [int]$ch
        if ($ch -eq '"') { [void]$sb.Append('\"') }
        elseif ($ch -eq '\') { [void]$sb.Append('\\') }
        elseif ($code -eq 10) { [void]$sb.Append('\n') }
        elseif ($code -eq 13) { [void]$sb.Append('\r') }
        elseif ($code -eq 9) { [void]$sb.Append('\t') }
        elseif ($code -lt 32) { [void]$sb.Append('\u' + $code.ToString('x4')) }
        else { [void]$sb.Append($ch) }
    }
    $sb.ToString()
}
function __ntc_reply([string]$id, [string]$body) {
    [Console]::Out.WriteLine('@@NTC@@' + $id + "`t" + $body)
    [Console]::Out.Flush()
}
while ($true) {
    $req = [Console]::In.ReadLine()
    if ($null -eq $req) { break }
    if ($req.Length -eq 0) { continue }
    $p = $req.Split("`t")
    if ($p.Length -lt 3) { continue }
    $id = $p[0]
    $op = $p[1]
    $payload = $p[2]
    $text = ''
    try { $text = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String($payload)) } catch { $text = '' }
    switch ($op) {
        'complete' {
            $pos = 0
            if ($p.Length -ge 4) { try { $pos = [int]$p[3] } catch { $pos = 0 } }
            try {
                $res = [System.Management.Automation.CommandCompletion]::CompleteInput($text, $pos, $null)
                $ri = $res.ReplacementIndex
                $rl = $res.ReplacementLength
                $parts = @()
                foreach ($m in $res.CompletionMatches) {
                    $parts += '{"t":"' + (__ntc_esc $m.CompletionText) + '","ri":' + $ri + ',"rl":' + $rl + '}'
                }
                __ntc_reply $id ('[' + ($parts -join ',') + ']')
            } catch {
                __ntc_reply $id '[]'
            }
        }
        'run' {
            $code = 0
            try {
                Invoke-Expression $text
                if (-not $?) { $code = 1 }
            } catch {
                [Console]::Out.WriteLine($_.ToString())
                [Console]::Out.Flush()
                $code = 1
            }
            __ntc_reply $id ([string]$code)
        }
        'state' {
            $loc = ''
            try { $loc = $PWD.Path } catch { }
            $b64 = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes($loc))
            __ntc_reply $id $b64
        }
        'names' {
            try {
                $names = @(Get-Command -CommandType Cmdlet,Function,Alias -ErrorAction SilentlyContinue |
                    ForEach-Object { $_.Name } | Sort-Object -Unique)
                $b64 = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes(($names -join "`n")))
                __ntc_reply $id $b64
            } catch {
                __ntc_reply $id ''
            }
        }
    }
}
"#;

fn spawn_host() -> Option<Host> {
    let exe = which::which("pwsh")
        .or_else(|_| which::which("powershell"))
        .or_else(|_| which::which("powershell.exe"))
        .ok()?;

    let encoded = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        utf16le(BOOTSTRAP),
    );

    let mut child = Command::new(exe)
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-EncodedCommand",
            &encoded,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        // PowerShell writes CLIXML error records to stderr; the protocol
        // reports failures on stdout instead, so keep the prompt clean.
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    let pid = child.id();
    HOST_PID.store(pid, Ordering::SeqCst);

    let stdin = child.stdin.take()?;
    let stdout = child.stdout.take()?;
    let (tx, rx) = mpsc::channel::<String>();
    std::thread::Builder::new()
        .name("ntc-ps-host".to_string())
        .spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        if tx.send(line).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        })
        .ok()?;

    Some(Host {
        child,
        stdin,
        rx,
        next_id: 1,
    })
}

fn utf16le(s: &str) -> Vec<u8> {
    s.encode_utf16().flat_map(|u| u.to_le_bytes()).collect()
}

fn host_slot() -> &'static Mutex<Option<Host>> {
    static HOST: OnceLock<Mutex<Option<Host>>> = OnceLock::new();
    HOST.get_or_init(|| Mutex::new(None))
}

/// True when some PowerShell launcher exists on this machine. PowerShell
/// integration is Windows-only: `pwsh` installed on Linux/macOS must not turn
/// on hints or `ntc ps`.
pub fn available() -> bool {
    if !crate::shell::ext_complete::platform::is_windows() {
        return false;
    }
    if UNAVAILABLE.load(Ordering::Relaxed) {
        return false;
    }
    which::which("pwsh")
        .or_else(|_| which::which("powershell"))
        .or_else(|_| which::which("powershell.exe"))
        .is_ok()
}

fn names_slot() -> &'static Mutex<Option<Arc<Vec<String>>>> {
    NAMES.get_or_init(|| Mutex::new(None))
}

/// The cached command-name list, without touching the host. `None` while the
/// one-time fetch is still pending (or after `invalidate_names`).
pub fn cached_command_names() -> Option<Arc<Vec<String>>> {
    names_slot().lock().ok()?.clone()
}

/// Forget the command-name cache (PATH/config changed).
pub fn invalidate_names() {
    NAMES_GENERATION.fetch_add(1, Ordering::SeqCst);
    if let Ok(mut slot) = names_slot().lock() {
        *slot = None;
    }
}

/// Fetch every cmdlet/function/alias name once and cache it for the fast local
/// prefix layer. Returns the cache when another caller has already filled it.
pub fn command_names(timeout: Duration) -> Option<Arc<Vec<String>>> {
    if let Some(names) = cached_command_names() {
        return Some(names);
    }
    let generation = NAMES_GENERATION.load(Ordering::SeqCst);
    let mut slot = host_slot().lock().ok()?;
    if !ensure_host(&mut slot) {
        return None;
    }
    let host = slot.as_mut()?;
    let payload = request(host, "names", "", None, timeout, None)?;
    let text = unb64(&payload);
    let mut names: Vec<String> = text
        .lines()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        // Cross-toolchain binaries are on PATH, so `Get-Command` reports them;
        // they must never be hinted as command names.
        .filter(|name| !super::is_target_triple_name(name))
        .map(str::to_string)
        .collect();
    if names.is_empty() {
        return None;
    }
    // PowerShell is case-insensitive; keep one canonical spelling per name.
    names.sort_by_key(|name| name.to_ascii_lowercase());
    names.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    let names = Arc::new(names);
    if generation == NAMES_GENERATION.load(Ordering::SeqCst) {
        if let Ok(mut cache) = names_slot().lock() {
            *cache = Some(Arc::clone(&names));
        }
    }
    Some(names)
}

/// Warm the command-name cache without blocking the caller. Used when a
/// `powershell …` launcher first asks for argument completion.
pub fn prefetch_command_names() {
    if cached_command_names().is_some() || UNAVAILABLE.load(Ordering::Relaxed) {
        return;
    }
    if NAMES_PREFETCHING.swap(true, Ordering::SeqCst) {
        return;
    }
    let spawned = std::thread::Builder::new()
        .name("ntc-ps-names".to_string())
        .spawn(|| {
            let _ = command_names(Duration::from_secs(10));
            NAMES_PREFETCHING.store(false, Ordering::SeqCst);
        });
    if spawned.is_err() {
        NAMES_PREFETCHING.store(false, Ordering::SeqCst);
    }
}

fn ensure_host(slot: &mut Option<Host>) -> bool {
    if let Some(h) = slot.as_mut() {
        match h.child.try_wait() {
            Ok(None) => return true,
            _ => {
                // Died (or was killed by Ctrl-C): drop it and start over.
                let _ = h.child.wait();
                *slot = None;
            }
        }
    }
    if !available() {
        UNAVAILABLE.store(true, Ordering::Relaxed);
        return false;
    }
    match spawn_host() {
        Some(h) => {
            *slot = Some(h);
            true
        }
        None => {
            UNAVAILABLE.store(true, Ordering::Relaxed);
            false
        }
    }
}

fn b64(s: &str) -> String {
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, s.as_bytes())
}

fn unb64(s: &str) -> String {
    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, s.trim())
        .ok()
        .and_then(|b| String::from_utf8(b).ok())
        .unwrap_or_default()
}

/// Send one request and wait for the reply with its id. Non-matching lines
/// (command output left over from an earlier line, stale replies) are
/// returned to the caller through `passthrough` when it is `Some`.
fn request(
    host: &mut Host,
    op: &str,
    payload: &str,
    extra: Option<usize>,
    timeout: Duration,
    mut passthrough: Option<&mut dyn FnMut(&str)>,
) -> Option<String> {
    let id = host.next_id;
    host.next_id = host.next_id.wrapping_add(1).max(1);
    let line = match extra {
        Some(n) => format!("{}\t{}\t{}\t{}", id, op, payload, n),
        None => format!("{}\t{}\t{}", id, op, payload),
    };
    if host.stdin.write_all(line.as_bytes()).is_err() {
        return None;
    }
    if host.stdin.write_all(b"\n").is_err() || host.stdin.flush().is_err() {
        return None;
    }
    let prefix = format!("{}{}\t", MARKER, id);
    let deadline = Instant::now() + timeout;
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return None;
        }
        match host.rx.recv_timeout(remaining) {
            Ok(line) => {
                if let Some(rest) = line.strip_prefix(&prefix) {
                    return Some(rest.to_string());
                }
                if !line.starts_with(MARKER) {
                    if let Some(cb) = passthrough.as_mut() {
                        cb(&line);
                    }
                }
            }
            Err(_) => return None,
        }
    }
}

/// Kill the host (Ctrl-C during a long `ntc ps` command) and mark it dead.
/// The next request transparently starts a fresh one.
///
/// Deliberately does not take the host mutex: the shell thread holds it for
/// the whole duration of a running command, and Ctrl-C must still get
/// through. Killing by PID closes the child's stdout, which unblocks the
/// reader and makes the pending request fail.
pub fn kill_host() {
    let pid = HOST_PID.swap(0, Ordering::SeqCst);
    if pid == 0 {
        return;
    }
    #[cfg(windows)]
    {
        let _ = Command::new("taskkill")
            .args(["/F", "/T", "/PID", &pid.to_string()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    #[cfg(not(windows))]
    {
        let _ = Command::new("kill")
            .args(["-TERM", &pid.to_string()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

/// Drop the host at shell exit.
pub fn shutdown() {
    PS_MODE.store(false, Ordering::SeqCst);
    kill_host();
    if let Ok(mut slot) = host_slot().lock() {
        if let Some(mut h) = slot.take() {
            let _ = h.child.wait();
        }
    }
}

/// Start (or reuse) the host and switch `ntc ps` mode on. The warm-up
/// round-trip makes a cold PowerShell start (or a broken host) visible here
/// instead of as timeouts on every later keystroke; the command-name list is
/// fetched here too so the first `g` is answered locally.
pub fn enter_ps_mode() -> Result<()> {
    if !crate::shell::ext_complete::platform::is_windows() {
        return Err(anyhow!(crate::t!("msg.setIS.ps_windows_only")));
    }
    let mut slot = host_slot()
        .lock()
        .map_err(|_| anyhow!("ps host lock poisoned"))?;
    if !ensure_host(&mut slot) {
        return Err(anyhow!("PowerShell was not found on this system"));
    }
    let host = slot.as_mut().expect("host just ensured");
    // Set the mode before warming up so Ctrl-C during startup can drop the
    // child through the shell's interrupt handler.
    PS_MODE.store(true, Ordering::SeqCst);
    if request(host, "state", "", None, Duration::from_secs(8), None).is_none() {
        PS_MODE.store(false, Ordering::SeqCst);
        let _ = slot.take();
        let pid = HOST_PID.swap(0, Ordering::SeqCst);
        if pid != 0 {
            #[cfg(windows)]
            let _ = Command::new("taskkill")
                .args(["/F", "/T", "/PID", &pid.to_string()])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
        return Err(anyhow!("PowerShell host failed to start"));
    }
    drop(slot);
    // One-time fetch: the first `g` must not wait for CommandCompletion.
    let _ = command_names(Duration::from_secs(8));
    Ok(())
}

/// Allow a retry after `invalidate()` (PATH or config changed).
pub(crate) fn reset_probe() {
    UNAVAILABLE.store(false, Ordering::Relaxed);
}

pub fn leave_ps_mode() {
    PS_MODE.store(false, Ordering::SeqCst);
}

pub fn ps_mode_active() -> bool {
    PS_MODE.load(Ordering::SeqCst)
}

/// Completion for a PowerShell command line. Returns `None` when the input
/// is not valid UTF-16-boundary aligned or the host did not answer in time.
pub fn complete(line: &str, pos: usize, budget: Duration) -> Option<Vec<PsMatch>> {
    let pos16 = byte_to_utf16(line, pos)?;
    let mut slot = host_slot().lock().ok()?;
    if !ensure_host(&mut slot) {
        return None;
    }
    let host = slot.as_mut()?;
    let payload = request(host, "complete", &b64(line), Some(pos16), budget, None)?;
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&payload).ok()?;
    let mut out = Vec::with_capacity(parsed.len());
    for m in parsed {
        let text = m
            .get("t")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if text.is_empty() {
            continue;
        }
        let ri = m.get("ri").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let rl = m.get("rl").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        out.push(PsMatch {
            text,
            replacement_index: ri,
            replacement_length: rl,
        });
    }
    Some(out)
}

/// Run one line in the persistent host, streaming its output to stdout.
pub fn run(line: &str) -> Result<i32> {
    let mut slot = host_slot()
        .lock()
        .map_err(|_| anyhow!("ps host lock poisoned"))?;
    if !ensure_host(&mut slot) {
        return Err(anyhow!("PowerShell was not found on this system"));
    }
    let host = slot.as_mut().unwrap();
    let mut print = |s: &str| {
        println!("{}", s);
    };
    let reply = request(
        host,
        "run",
        &b64(line),
        None,
        Duration::from_secs(60 * 60 * 24),
        Some(&mut print),
    );
    match reply {
        Some(code) => Ok(code.trim().parse::<i32>().unwrap_or(0)),
        None => Err(anyhow!("PowerShell host stopped responding")),
    }
}

/// Current location of the host, used to keep ntc's directory in sync.
pub fn cwd() -> Option<String> {
    let mut slot = host_slot().lock().ok()?;
    if !ensure_host(&mut slot) {
        return None;
    }
    let host = slot.as_mut()?;
    let raw = request(host, "state", "", None, Duration::from_millis(500), None)?;
    Some(unb64(&raw))
}

fn byte_to_utf16(s: &str, byte: usize) -> Option<usize> {
    if byte > s.len() || !s.is_char_boundary(byte) {
        return None;
    }
    let mut units = 0;
    for (b, ch) in s.char_indices() {
        if b >= byte {
            return Some(units);
        }
        units += ch.len_utf16();
    }
    Some(units)
}

/// Convert a UTF-16 code-unit offset back to a byte offset.
pub fn utf16_to_byte(s: &str, units: usize) -> Option<usize> {
    let mut seen = 0;
    for (b, ch) in s.char_indices() {
        if seen >= units {
            return Some(b);
        }
        seen += ch.len_utf16();
    }
    if seen == units {
        Some(s.len())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf16_offsets_round_trip() {
        assert_eq!(utf16_to_byte("Get-Chi", 7), Some(7));
        assert_eq!(utf16_to_byte("abc", 0), Some(0));
        assert_eq!(utf16_to_byte("abc", 3), Some(3));
        assert_eq!(utf16_to_byte("abc", 4), None);
        // Astral characters are two UTF-16 units but four bytes.
        assert_eq!(utf16_to_byte("a𝄞", 3), Some(5));
    }

    /// PowerShell completes case-insensitively; lowercase input must offer
    /// canonical cmdlet names. Skips when this machine has no PowerShell.
    #[test]
    fn host_completes_lowercase_cmdlets() {
        if !available() {
            return;
        }
        let _lock = crate::utils::test_support::lock_globals();
        let matches = complete("get-c", 5, Duration::from_secs(30)).expect("host did not answer");
        assert!(
            matches
                .iter()
                .any(|m| m.text.to_ascii_lowercase().starts_with("get-c")),
            "no Get-* cmdlet in {:?}",
            matches.iter().map(|m| &m.text).collect::<Vec<_>>()
        );
        let matches = complete("sel", 3, Duration::from_secs(30)).expect("host did not answer");
        assert!(
            matches
                .iter()
                .any(|m| m.text.to_ascii_lowercase().starts_with("select-")),
            "no Select-* cmdlet in {:?}",
            matches.iter().map(|m| &m.text).collect::<Vec<_>>()
        );
    }

    #[test]
    fn command_names_cache_has_cmdlets() {
        if !available() {
            return;
        }
        let _lock = crate::utils::test_support::lock_globals();
        let names = command_names(Duration::from_secs(30)).expect("names request failed");
        assert!(
            names
                .iter()
                .any(|n| n.eq_ignore_ascii_case("Get-ChildItem")),
            "Get-ChildItem missing from {} names",
            names.len()
        );
        assert!(
            names.iter().any(|n| n.eq_ignore_ascii_case("Get-Content")),
            "Get-Content missing from {} names",
            names.len()
        );
    }

    #[test]
    fn enter_and_leave_ps_mode() {
        if !available() {
            return;
        }
        let _lock = crate::utils::test_support::lock_globals();
        enter_ps_mode().expect("host should start");
        assert!(ps_mode_active());
        leave_ps_mode();
        assert!(!ps_mode_active());
    }

    /// Real host round-trip: completion, state persistence and cwd. Skips
    /// silently when this machine has no PowerShell.
    #[test]
    fn host_completes_and_keeps_state() {
        if !available() {
            return;
        }
        let _lock = crate::utils::test_support::lock_globals();
        let matches = complete("Get-Chi", 7, Duration::from_secs(30)).expect("host did not answer");
        assert!(
            matches.iter().any(|m| m.text.starts_with("Get-ChildItem")),
            "no Get-ChildItem in {:?}",
            matches.iter().map(|m| &m.text).collect::<Vec<_>>()
        );

        assert_eq!(run("$ntcTest = 41 + 1").unwrap(), 0);
        assert_eq!(run("$ntcTest").unwrap(), 0);

        assert_eq!(run("Set-Location $env:TEMP").unwrap(), 0);
        let cwd = cwd().expect("state query failed");
        let expected = std::env::var("TEMP").unwrap_or_default();
        assert!(
            !expected.is_empty() && cwd.to_lowercase().ends_with(&expected.to_lowercase()),
            "cwd {:?} did not follow TEMP {:?}",
            cwd,
            expected
        );

        // Warm round-trips must stay well inside the keystroke budget.
        let start = Instant::now();
        for _ in 0..10 {
            let _ = complete("Get-Chi", 7, Duration::from_secs(5));
        }
        let per_call = start.elapsed() / 10;
        println!("warm completion: {:?}/call", per_call);
        // Production no longer blocks on this path (it runs in the background
        // worker), so the bound guards gross regressions, not the keystroke
        // budget itself. Keep it loose enough for a loaded CI machine.
        assert!(per_call < Duration::from_millis(250), "{:?}", per_call);
    }
}
