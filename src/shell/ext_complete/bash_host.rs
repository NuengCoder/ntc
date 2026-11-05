//! Persistent bash host used by the integrated bash mode.
//!
//! On desktop Linux, typing `bash` at the ntc prompt enters
//! `ntc <distro> [dir]>`: a warm `bash` child executes every line while ntc
//! keeps its own line editor, so variables, functions, aliases and the
//! working directory survive between commands. The host never renders a
//! prompt; the bootstrap loop only reads id-tagged requests and writes
//! id-tagged replies (command output is passed through untouched).

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};

use super::platform;

/// One bash completion candidate and the byte range of the line it replaces.
#[derive(Debug, Clone)]
pub struct Match {
    pub text: String,
    /// Byte index where the replacement starts.
    pub start: usize,
    /// Byte length being replaced.
    pub length: usize,
}

/// Set while the shell is in integrated bash mode (see `ext_complete`).
pub(crate) static BASH_MODE: AtomicBool = AtomicBool::new(false);
/// PID of the live host, 0 when none.
static HOST_PID: AtomicU32 = AtomicU32::new(0);
/// True when no bash could be started.
static UNAVAILABLE: AtomicBool = AtomicBool::new(false);
/// Command names fetched once per session for the fast local prefix layer.
static NAMES: OnceLock<Mutex<Option<Arc<Vec<String>>>>> = OnceLock::new();
static NAMES_PREFETCHING: AtomicBool = AtomicBool::new(false);
static NAMES_GENERATION: AtomicU64 = AtomicU64::new(0);

struct Host {
    child: Child,
    stdin: ChildStdin,
    rx: Receiver<String>,
    next_id: u64,
    /// Reply marker, unique per session so command output cannot collide.
    marker: String,
}

/// Bootstrap run inside the child. Every request is one tab-separated line;
/// the reply marker is `NTC_BASH_TAG` from the environment. `eval` runs with
/// stdin pointed at `/dev/null` so an interactive command (`read`, `cat`)
/// cannot consume the request stream.
const BOOTSTRAP: &str = r#"
set +m
shopt -s expand_aliases
exec 2>&1
__ntc_tag=${NTC_BASH_TAG:-@@NTC@@}
__ntc_b64() { printf '%s' "$1" | base64 | tr -d '\n'; }
__ntc_unb64() { printf '%s' "$1" | base64 -d 2>/dev/null; }
__ntc_reply() { printf '%s%s\t%s\n' "$__ntc_tag" "$1" "$2"; }
for __ntc_f in /usr/share/bash-completion/bash_completion /etc/bash_completion /usr/local/share/bash-completion/bash_completion; do
  if [ -f "$__ntc_f" ]; then . "$__ntc_f" >/dev/null 2>&1; break; fi
done
__ntc_split() {
  COMP_LINE=$1
  COMP_POINT=$2
  COMP_WORDS=()
  COMPREPLY=()
  local __ntc_head=${COMP_LINE:0:COMP_POINT}
  local IFS=$' \t\n'
  read -r -a COMP_WORDS <<< "$__ntc_head"
  if [ -n "$__ntc_head" ] && [ "${__ntc_head: -1}" = " " ]; then COMP_WORDS+=(""); fi
  if [ ${#COMP_WORDS[@]} -eq 0 ]; then COMP_WORDS=(""); fi
  COMP_CWORD=$(( ${#COMP_WORDS[@]} - 1 ))
  local __ntc_cur=${COMP_WORDS[COMP_CWORD]}
  local __ntc_wb=${COMP_WORDBREAKS-}
  local __ntc_i __ntc_c
  for (( __ntc_i=${#__ntc_cur}-1; __ntc_i>=0; __ntc_i-- )); do
    __ntc_c=${__ntc_cur:__ntc_i:1}
    case $__ntc_wb in
      *"$__ntc_c"*)
        COMP_WORDS[COMP_CWORD]=${__ntc_cur:0:__ntc_i+1}
        COMP_WORDS+=("${__ntc_cur:__ntc_i+1}")
        COMP_CWORD=$((COMP_CWORD+1))
        break;;
    esac
  done
}
__ntc_complete() {
  local __ntc_line=$1 __ntc_point=$2
  __ntc_split "$__ntc_line" "$__ntc_point"
  local __ntc_cmd=${COMP_WORDS[0]}
  local __ntc_cur=${COMP_WORDS[COMP_CWORD]}
  local __ntc_prev=''
  [ "$COMP_CWORD" -ge 1 ] && __ntc_prev=${COMP_WORDS[COMP_CWORD-1]}
  local __ntc_start=$(( __ntc_point - ${#__ntc_cur} ))
  [ "$__ntc_start" -lt 0 ] && __ntc_start=0
  local __ntc_spec
  __ntc_spec=$(complete -p -- "$__ntc_cmd" 2>/dev/null) || __ntc_spec=''
  if [ -z "$__ntc_spec" ]; then
    # bash-completion >= 2.12 lazy-loads a command's script through
    # `_comp_complete_load`; 2.11 (Ubuntu 24.04) only ships
    # `__load_completion` and its `-D` handler `_completion_loader`.  Without
    # this fallback the completion file is never sourced and every command
    # that relies on it (git, docker, ...) degrades to file names.
    if declare -F _comp_complete_load >/dev/null 2>&1; then
      _comp_complete_load "$__ntc_cmd" >/dev/null 2>&1
    elif declare -F __load_completion >/dev/null 2>&1; then
      __load_completion "$__ntc_cmd" >/dev/null 2>&1
    elif declare -F _completion_loader >/dev/null 2>&1; then
      _completion_loader "$__ntc_cmd" >/dev/null 2>&1
    fi
    __ntc_spec=$(complete -p -- "$__ntc_cmd" 2>/dev/null) || __ntc_spec=''
  fi
  local __ntc_deflt=0 __ntc_dirs=0 __ntc_plus=0
  local __ntc_fn='' __ntc_cfn=''
  if [ -n "$__ntc_spec" ]; then
    eval "set -- ${__ntc_spec#complete }" 2>/dev/null
    while [ $# -gt 0 ]; do
      case $1 in
        -F) __ntc_fn=${2-}; shift 2 2>/dev/null || shift;;
        -C) __ntc_cfn=${2-}; shift 2 2>/dev/null || shift;;
        -o) case ${2-} in
               default|bashdefault) __ntc_deflt=1;;
               dirnames) __ntc_dirs=1;;
               plusdirs) __ntc_plus=1;;
             esac
             shift 2 2>/dev/null || shift;;
        -P|-S|-X) shift 2 2>/dev/null || shift;;
        *) shift;;
      esac
    done
  fi
  if [ -n "$__ntc_fn" ] && declare -F -- "$__ntc_fn" >/dev/null 2>&1; then
    COMPREPLY=()
    "$__ntc_fn" "$__ntc_cmd" "$__ntc_cur" "$__ntc_prev" >/dev/null 2>&1
  elif [ -n "$__ntc_cfn" ]; then
    COMPREPLY=( $(eval "$__ntc_cfn" "$__ntc_cmd" "$__ntc_cur" "$__ntc_prev" 2>/dev/null) )
  fi
  if [ ${#COMPREPLY[@]} -eq 0 ]; then
    if [ -z "$__ntc_spec" ] || [ $__ntc_deflt -eq 1 ] || [ $__ntc_dirs -eq 1 ] || [ $__ntc_plus -eq 1 ]; then
      if [ $__ntc_dirs -eq 1 ] && [ $__ntc_deflt -eq 0 ]; then
        COMPREPLY=( $(compgen -d -- "$__ntc_cur") )
      else
        COMPREPLY=( $(compgen -f -- "$__ntc_cur") )
      fi
    fi
    if [ $__ntc_plus -eq 1 ]; then
      COMPREPLY+=( $(compgen -d -- "$__ntc_cur") )
    fi
  fi
  local __ntc_out="$__ntc_start"$'\t'"${#__ntc_cur}"
  local __ntc_cand __ntc_word
  for __ntc_cand in "${COMPREPLY[@]}"; do
    __ntc_word=${__ntc_cand%%$'\t'*}
    [ -z "$__ntc_word" ] && continue
    __ntc_out+=$'\t'$(__ntc_b64 "$__ntc_word")
  done
  __ntc_reply "$__ntc_id" "$__ntc_out"
}
while IFS= read -r __ntc_req; do
  [ -z "$__ntc_req" ] && continue
  __ntc_id=''
  __ntc_op=''
  __ntc_payload=''
  __ntc_extra=''
  IFS=$'\t' read -r __ntc_id __ntc_op __ntc_payload __ntc_extra <<< "$__ntc_req"
  case $__ntc_op in
    run)
      __ntc_script=$(__ntc_unb64 "$__ntc_payload")
      __ntc_before=$( { compgen -A function; compgen -A alias; } | LC_ALL=C sort -u | wc -l )
      eval "$__ntc_script" </dev/null
      __ntc_code=$?
      __ntc_after=$( { compgen -A function; compgen -A alias; } | LC_ALL=C sort -u | wc -l )
      # A command that defined or removed a function/alias makes the name
      # cache stale; report it so ntc refetches (a plain `ls` must not).
      __ntc_dirty=0
      [ "$__ntc_before" != "$__ntc_after" ] && __ntc_dirty=1
      __ntc_reply "$__ntc_id" "$__ntc_code"$'\t'"$__ntc_dirty"
      ;;
    state)
      __ntc_reply "$__ntc_id" "$(__ntc_b64 "$PWD")"
      ;;
    names)
      # Shell-local names only. `compgen -c` walks every PATH entry, which can
      # take a minute on a network/drvfs PATH, and the reply must not block
      # the run/complete requests queued behind it; ntc adds PATH executables
      # from its own cached scan.
      __ntc_names=$( { compgen -A alias; compgen -A builtin; compgen -A function; compgen -A keyword; } 2>/dev/null | LC_ALL=C sort -u )
      __ntc_reply "$__ntc_id" "$(__ntc_b64 "$__ntc_names")"
      ;;
    complete)
      __ntc_complete "$(__ntc_unb64 "$__ntc_payload")" "$__ntc_extra"
      ;;
  esac
done
"#;

fn marker_for_session() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64 + d.as_secs())
        .unwrap_or(0);
    format!("\x1eNTC{}x{}\x1e", std::process::id(), nanos)
}

fn spawn_host() -> Option<Host> {
    if !platform::is_bash_platform() {
        return None;
    }
    let exe = which::which("bash").ok()?;
    let marker = marker_for_session();
    let mut child = Command::new(exe)
        .args(["--noprofile", "--norc", "-c", BOOTSTRAP])
        .env("NTC_BASH_TAG", &marker)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        // The bootstrap redirects stderr into stdout, so errors keep their
        // place in the output stream.
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    HOST_PID.store(child.id(), Ordering::SeqCst);

    let stdin = child.stdin.take()?;
    let stdout = child.stdout.take()?;
    let (tx, rx) = mpsc::channel::<String>();
    std::thread::Builder::new()
        .name("ntc-bash-host".to_string())
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
        marker,
    })
}

fn host_slot() -> &'static Mutex<Option<Host>> {
    static HOST: OnceLock<Mutex<Option<Host>>> = OnceLock::new();
    HOST.get_or_init(|| Mutex::new(None))
}

/// True when a bash launcher exists. The integrated mode needs a bash
/// platform (desktop Linux or Termux); other builds never enter it.
pub fn available() -> bool {
    if !platform::is_bash_platform() {
        return false;
    }
    if UNAVAILABLE.load(Ordering::Relaxed) {
        return false;
    }
    which::which("bash").is_ok()
}

fn ensure_host(slot: &mut Option<Host>) -> bool {
    if let Some(h) = slot.as_mut() {
        match h.child.try_wait() {
            Ok(None) => return true,
            _ => {
                // Died (or was killed by Ctrl-C): drop it and start over.
                let _ = h.child.wait();
                HOST_PID.store(0, Ordering::SeqCst);
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
    let prefix = format!("{}{}\t", host.marker, id);
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
                if !line.starts_with(&host.marker) {
                    if let Some(cb) = passthrough.as_mut() {
                        cb(&line);
                    }
                }
            }
            Err(_) => return None,
        }
    }
}

/// Kill the host (Ctrl-C during a long command) and mark it dead. The next
/// command transparently starts a fresh one, losing shell state.
///
/// Deliberately does not take the host mutex: the shell thread holds it for
/// the whole duration of a running command, and Ctrl-C must still get through.
pub fn kill_host() {
    let pid = HOST_PID.swap(0, Ordering::SeqCst);
    if pid == 0 {
        return;
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
    BASH_MODE.store(false, Ordering::SeqCst);
    kill_host();
    if let Ok(mut slot) = host_slot().lock() {
        if let Some(mut h) = slot.take() {
            let _ = h.child.wait();
        }
    }
}

/// True when the host process is still running. A line like `exit 3` or
/// `exec ls` closes it, and the shell must leave the mode.
pub fn host_alive() -> bool {
    let Ok(mut slot) = host_slot().lock() else {
        return false;
    };
    match slot.as_mut() {
        Some(h) => matches!(h.child.try_wait(), Ok(None)),
        None => false,
    }
}

/// Start (or reuse) the host and switch the integrated mode on. The warm-up
/// round-trip makes a broken host visible here instead of as a missing ghost
/// on every later keystroke; command names are fetched here too so the first
/// word is answered locally.
pub fn enter_bash_mode() -> Result<()> {
    if !platform::is_bash_platform() {
        return Err(anyhow!(crate::t!("msg.setIS.bash_linux_only")));
    }
    let mut slot = host_slot()
        .lock()
        .map_err(|_| anyhow!("bash host lock poisoned"))?;
    if !ensure_host(&mut slot) {
        return Err(anyhow!(crate::t!("msg.setIS.bash_missing")));
    }
    let host = slot.as_mut().expect("host just ensured");
    // Set the mode before warming up so Ctrl-C during startup can drop the
    // child through the shell's interrupt handler.
    BASH_MODE.store(true, Ordering::SeqCst);
    if request(host, "state", "", None, Duration::from_secs(8), None).is_none() {
        BASH_MODE.store(false, Ordering::SeqCst);
        let _ = slot.take();
        kill_host();
        return Err(anyhow!(crate::t!("msg.setIS.bash_start_failed")));
    }
    drop(slot);
    let _ = command_names(Duration::from_secs(8));
    Ok(())
}

/// Allow a retry after `invalidate()` (PATH or config changed).
pub(crate) fn reset_probe() {
    UNAVAILABLE.store(false, Ordering::Relaxed);
}

pub fn leave_bash_mode() {
    BASH_MODE.store(false, Ordering::SeqCst);
}

pub fn bash_mode_active() -> bool {
    BASH_MODE.load(Ordering::SeqCst)
}

/// Run one line in the persistent host, streaming its output to stdout.
pub fn run(line: &str) -> Result<i32> {
    let mut slot = host_slot()
        .lock()
        .map_err(|_| anyhow!("bash host lock poisoned"))?;
    if !ensure_host(&mut slot) {
        return Err(anyhow!(crate::t!("msg.setIS.bash_missing")));
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
        Some(reply) => {
            let mut fields = reply.split('\t');
            let code = fields
                .next()
                .and_then(|v| v.trim().parse::<i32>().ok())
                .unwrap_or(0);
            if fields.next().map(|v| v.trim() == "1").unwrap_or(false) {
                // A function or alias was defined/removed in this line.
                invalidate_names();
            }
            Ok(code)
        }
        None if host.child.try_wait().ok().flatten().is_some() => {
            Err(anyhow!(crate::t!("msg.setIS.bash_exited")))
        }
        None => Err(anyhow!(crate::t!("msg.setIS.bash_stopped"))),
    }
}

/// Current location of the host, used to keep ntc's directory in sync.
pub fn cwd() -> Option<String> {
    let mut slot = host_slot().lock().ok()?;
    if !ensure_host(&mut slot) {
        return None;
    }
    let host = slot.as_mut()?;
    let raw = request(host, "state", "", None, Duration::from_millis(750), None)?;
    Some(unb64(&raw))
}

fn names_slot() -> &'static Mutex<Option<Arc<Vec<String>>>> {
    NAMES.get_or_init(|| Mutex::new(None))
}

/// Shell-local command names (builtins, functions, aliases, keywords) without
/// touching the host, or `None` while the fetch is still pending. PATH
/// executables come from ntc's own cached scan, so they are not repeated here.
pub fn cached_command_names() -> Option<Arc<Vec<String>>> {
    names_slot().lock().ok()?.clone()
}

/// Forget the command-name cache (the session defined or removed a command).
pub fn invalidate_names() {
    NAMES_GENERATION.fetch_add(1, Ordering::SeqCst);
    if let Ok(mut slot) = names_slot().lock() {
        *slot = None;
    }
}

fn command_names(timeout: Duration) -> Option<Arc<Vec<String>>> {
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
    let mut names: Vec<String> = unb64(&payload)
        .lines()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(str::to_string)
        .collect();
    if names.is_empty() {
        return None;
    }
    names.sort_by_cached_key(|name| name.to_ascii_lowercase());
    names.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    let names = Arc::new(names);
    if generation == NAMES_GENERATION.load(Ordering::SeqCst) {
        if let Ok(mut cache) = names_slot().lock() {
            *cache = Some(Arc::clone(&names));
        }
    }
    Some(names)
}

/// Warm the command-name cache without blocking the caller.
pub fn prefetch_command_names() {
    if cached_command_names().is_some() || UNAVAILABLE.load(Ordering::Relaxed) {
        return;
    }
    if NAMES_PREFETCHING.swap(true, Ordering::SeqCst) {
        return;
    }
    let spawned = std::thread::Builder::new()
        .name("ntc-bash-names".to_string())
        .spawn(|| {
            let _ = command_names(Duration::from_secs(10));
            NAMES_PREFETCHING.store(false, Ordering::SeqCst);
        });
    if spawned.is_err() {
        NAMES_PREFETCHING.store(false, Ordering::SeqCst);
    }
}

/// Completion for a bash command line, answered by the persistent host so
/// aliases, functions, variables and `cd` are taken into account.
pub fn complete(line: &str, pos: usize, budget: Duration) -> Option<Vec<Match>> {
    let point = line.get(..pos)?.chars().count();
    let mut slot = host_slot().lock().ok()?;
    if !ensure_host(&mut slot) {
        return None;
    }
    let host = slot.as_mut()?;
    let payload = request(host, "complete", &b64(line), Some(point), budget, None)?;
    Some(parse_complete(line, &payload))
}

/// Parse `<start char>\t<length chars>\t<b64 candidate>…` from the host.
fn parse_complete(line: &str, body: &str) -> Vec<Match> {
    let mut fields = body.split('\t');
    let start_char = match fields.next().and_then(|v| v.trim().parse::<usize>().ok()) {
        Some(n) => n,
        None => return Vec::new(),
    };
    let length_chars = match fields.next().and_then(|v| v.trim().parse::<usize>().ok()) {
        Some(n) => n,
        None => return Vec::new(),
    };
    let Some(start) = char_to_byte(line, start_char) else {
        return Vec::new();
    };
    let Some(end) = char_to_byte(line, start_char + length_chars) else {
        return Vec::new();
    };
    let quoted = inside_quotes(&line[..start]);
    let escaped = line[..start].ends_with('\\');
    let mut out = Vec::new();
    for raw in fields {
        let text = unb64(raw);
        // Bash's own readline adds the separator; the line editor never
        // appends one, so trim it from the candidate (`checkout ` → `checkout`).
        let text = text.trim_end_matches([' ', '\t']);
        if text.is_empty() {
            continue;
        }
        let text = if quoted || escaped {
            text.to_string()
        } else {
            shell_escape(text)
        };
        out.push(Match {
            text,
            start,
            length: end - start,
        });
    }
    out
}

/// Convert a character index (how bash counts `${#…}`) to a byte index.
fn char_to_byte(s: &str, chars: usize) -> Option<usize> {
    if chars == 0 {
        return Some(0);
    }
    let mut seen = 0;
    for (byte, _) in s.char_indices() {
        if seen == chars {
            return Some(byte);
        }
        seen += 1;
    }
    if seen == chars {
        Some(s.len())
    } else {
        None
    }
}

/// True when `prefix` leaves an unterminated `'` or `"` quote open.
fn inside_quotes(prefix: &str) -> bool {
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for ch in prefix.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        match quote {
            Some(q) => {
                if q == '"' && ch == '\\' {
                    escaped = true;
                } else if ch == q {
                    quote = None;
                }
            }
            None => match ch {
                '\\' => escaped = true,
                '\'' | '"' => quote = Some(ch),
                _ => {}
            },
        }
    }
    quote.is_some()
}

/// Backslash-escape the characters that would otherwise split or expand a
/// completion candidate (`My File` → `My\ File`).
fn shell_escape(text: &str) -> String {
    const META: &str = " \t\n\"'`$&;|<>(){}[]*?!";
    if !text.chars().any(|c| META.contains(c)) {
        return text.to_string();
    }
    let mut out = String::with_capacity(text.len() + 4);
    for ch in text.chars() {
        if META.contains(ch) {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_offsets_convert_to_bytes() {
        assert_eq!(char_to_byte("abc", 0), Some(0));
        assert_eq!(char_to_byte("abc", 2), Some(2));
        assert_eq!(char_to_byte("abc", 3), Some(3));
        assert_eq!(char_to_byte("abc", 4), None);
        // Multi-byte characters count as one, like bash's `${#…}`.
        assert_eq!(char_to_byte("aé b", 2), Some(3));
        assert_eq!(char_to_byte("aé b", 4), Some(5));
    }

    #[test]
    fn quoting_state_is_tracked() {
        assert!(!inside_quotes(""));
        assert!(inside_quotes("echo 'un terminated"));
        assert!(!inside_quotes("echo 'done' "));
        assert!(inside_quotes("echo \"open"));
        assert!(!inside_quotes("echo \\\"closed\\\""));
    }

    #[test]
    fn candidates_with_spaces_are_escaped() {
        assert_eq!(shell_escape("plain"), "plain");
        assert_eq!(shell_escape("My File"), "My\\ File");
        assert_eq!(shell_escape("a*b"), "a\\*b");
    }

    #[test]
    fn complete_reply_is_parsed_into_byte_ranges() {
        let body = format!("4\t2\t{}", b64("checkout"));
        let matches = parse_complete("git ch", &body);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].text, "checkout");
        assert_eq!((matches[0].start, matches[0].length), (4, 2));
        assert_eq!(
            format!(
                "{}{}{}",
                &"git ch"[..matches[0].start],
                matches[0].text,
                &"git ch"[matches[0].start + matches[0].length..]
            ),
            "git checkout"
        );
    }

    /// Real round-trip against the persistent host. Skips off desktop Linux
    /// or when bash is not installed.
    #[test]
    fn host_keeps_state_and_reports_errors() {
        if !available() {
            return;
        }
        let _lock = crate::utils::test_support::lock_globals();

        let old = std::env::current_dir().ok();
        // Variables, aliases and functions survive between lines.
        assert_eq!(run("ntc_var=42").expect("assignment should run"), 0);
        assert_eq!(run("test \"$ntc_var\" = 42").expect("test should run"), 0);
        // A line that defines a name drops the name cache so the next
        // completion refetches it; a plain line must not.
        let _ = command_names(Duration::from_secs(10));
        assert_eq!(run("alias ntc_alias='true'").expect("alias should run"), 0);
        assert!(
            cached_command_names().is_none(),
            "defining an alias must invalidate the name cache"
        );
        let names = command_names(Duration::from_secs(10)).expect("names request failed");
        assert!(
            names.iter().any(|n| n == "ntc_alias"),
            "new alias missing from {names:?}"
        );
        assert_eq!(run("ntc_alias").expect("alias should expand"), 0);
        assert!(
            cached_command_names().is_some(),
            "a plain command must keep the name cache"
        );
        assert_eq!(
            run("ntc_fn() { return 7; }; ntc_fn").expect("function should run"),
            7
        );

        assert_eq!(run("cd /tmp").expect("cd should run"), 0);
        let cwd = cwd().expect("cwd request failed");
        assert!(cwd.ends_with("tmp"), "unexpected cwd {cwd:?}");

        assert_eq!(run("false").expect("false should run"), 1);
        assert_eq!(
            run("ntc_missing_command_xyz").expect("missing should run"),
            127
        );

        if let Some(old) = old {
            let _ = run(&format!(
                "cd '{}'",
                old.to_string_lossy().replace('\'', "'\\''")
            ));
        }
        shutdown();
    }

    #[test]
    fn enter_and_leave_bash_mode() {
        if !available() {
            return;
        }
        let _lock = crate::utils::test_support::lock_globals();
        // Warm the host first so the mode window stays tiny: parallel tests
        // that read `bash_mode_active` must not observe it.
        let _ = run("true");
        enter_bash_mode().expect("host should start");
        assert!(bash_mode_active());
        leave_bash_mode();
        assert!(!bash_mode_active());
        shutdown();
    }

    #[test]
    fn completion_after_a_new_alias_does_not_block() {
        if !available() {
            return;
        }
        let _lock = crate::utils::test_support::lock_globals();
        assert_eq!(run("alias ntc_fixture=true").expect("alias"), 0);
        let start = Instant::now();
        let matches = complete("ntc_fixture /", 12, Duration::from_secs(5));
        assert!(matches.is_some(), "host stopped answering after an alias");
        assert!(start.elapsed() < Duration::from_secs(5));
        // A plain command must still run afterwards.
        assert_eq!(run("true").expect("run after completion"), 0);
        shutdown();
    }

    #[test]
    fn command_names_are_shell_local_and_cheap() {
        if !available() {
            return;
        }
        let _lock = crate::utils::test_support::lock_globals();
        assert_eq!(run("alias ntc_fixture_alias=true").expect("alias"), 0);
        invalidate_names();
        let start = Instant::now();
        let names = command_names(Duration::from_secs(10)).expect("names request failed");
        let elapsed = start.elapsed();
        assert!(names.iter().any(|n| n == "cd"), "no builtin in {names:?}");
        assert!(
            names.iter().any(|n| n == "ntc_fixture_alias"),
            "no session alias in {names:?}"
        );
        // PATH executables are deliberately excluded: scanning a network or
        // drvfs PATH can take minutes and would block the command protocol.
        assert!(elapsed < Duration::from_secs(5), "names took {elapsed:?}");
        shutdown();
    }

    #[test]
    fn host_completes_git_subcommands() {
        if !available() || !super::super::bash_completion_available() {
            return;
        }
        let _lock = crate::utils::test_support::lock_globals();
        let matches = complete("git ch", 6, Duration::from_secs(30)).expect("host did not answer");
        assert!(
            matches.iter().any(|m| m.text == "checkout"),
            "no checkout in {:?}",
            matches.iter().map(|m| &m.text).collect::<Vec<_>>()
        );
        shutdown();
    }
}
