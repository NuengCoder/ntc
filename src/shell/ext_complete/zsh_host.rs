//! Persistent zsh host used by the integrated zsh mode.
//!
//! On macOS, typing bare `zsh` at the ntc prompt enters `ntc zsh [dir]>`:
//! a warm `zsh -f` child executes every line while ntc keeps its own line
//! editor, so aliases, functions, variables and the working directory
//! survive between commands.
//!
//! The host deliberately does not load zsh's completion system. Command
//! names come from the shell itself (aliases, builtins, functions, reserved
//! words) plus ntc's PATH scan; flags, subcommands and `brew` come from
//! ntc's curated tables; paths and `$`-variables are answered by simple zsh
//! globbing. That keeps the host fast and predictable without depending on
//! a user's `compinit` setup.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};

use super::platform;

/// One zsh completion candidate and the byte range of the line it replaces.
#[derive(Debug, Clone)]
pub struct Match {
    pub text: String,
    /// Byte index where the replacement starts.
    pub start: usize,
    /// Byte length being replaced.
    pub length: usize,
}

/// Set while the shell is in integrated zsh mode (see `ext_complete`).
pub(crate) static ZSH_MODE: AtomicBool = AtomicBool::new(false);
/// PID of the live host, 0 when none.
static HOST_PID: AtomicU32 = AtomicU32::new(0);
/// True when no zsh could be started.
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
/// the reply marker is `NTC_ZSH_TAG` from the environment. `eval` runs with
/// stdin pointed at `/dev/null` so an interactive command (`read`, `cat`)
/// cannot consume the request stream.
const BOOTSTRAP: &str = r#"
exec 2>&1
__ntc_tag=${NTC_ZSH_TAG:-@@NTC@@}
# The alias/function/builtin/parameter tables come from this module. Load it
# once: an implicit autoload per expansion both spams errors on a stripped
# build and can put the module name into a candidate list.
__ntc_has_parameter=0
zmodload zsh/parameter 2>/dev/null && __ntc_has_parameter=1
__ntc_b64() { print -rn -- "$1" | base64 | tr -d '\n'; }
__ntc_unb64() {
  local __ntc_out
  __ntc_out=$(print -rn -- "$1" | base64 -d 2>/dev/null) \
    || __ntc_out=$(print -rn -- "$1" | base64 -D 2>/dev/null)
  print -rn -- "$__ntc_out"
}
__ntc_reply() { print -r -- "$__ntc_tag$1"$'\t'"$2"; }
__ntc_name_count() {
  if (( __ntc_has_parameter )); then
    print -rn -- $(( ${#aliases} + ${#functions} ))
  else
    print -rn -- 0
  fi
}
__ntc_split() {
  __ntc_head=${1[1,$2]}
  __ntc_words=( ${=__ntc_head} )
  if [[ $__ntc_head == *' ' || $__ntc_head == *$'\t' ]]; then
    __ntc_words+=( '' )
  fi
  (( ${#__ntc_words} == 0 )) && __ntc_words=( '' )
  __ntc_cur=${__ntc_words[-1]}
  __ntc_start=$(( $2 - ${#__ntc_cur} ))
  (( __ntc_start < 0 )) && __ntc_start=0
}
__ntc_complete() {
  __ntc_split "$1" "$2"
  local -a __ntc_comp
  local __ntc_dollar=0
  if [[ $__ntc_cur == \$* ]]; then
    if (( __ntc_has_parameter )); then
      __ntc_comp=( ${(k)parameters} )
      # The host's own variables are not user variables.
      __ntc_comp=( ${__ntc_comp:#__ntc_*} )
      # The candidate replaces the whole `$FRAG` token, so keep the sigil.
      __ntc_dollar=1
    else
      __ntc_comp=()
    fi
  elif (( ${#__ntc_words} == 1 )); then
    if (( __ntc_has_parameter )); then
      __ntc_comp=( ${(k)aliases} ${(k)builtins} ${(k)functions} ${(k)reswords} )
      # The host's own helpers are not commands.
      __ntc_comp=( ${__ntc_comp:#__ntc_*} )
    else
      __ntc_comp=()
    fi
  elif [[ $__ntc_cur == -* ]]; then
    __ntc_comp=()
  else
    # Parameter expansion is literal in zsh, so `cur*` globs the typed prefix
    # followed by anything. `(N)` keeps an empty match silent.
    __ntc_comp=( ${__ntc_cur}*(N) )
  fi
  local __ntc_out="$__ntc_start"$'\t'"${#__ntc_cur}"
  local __ntc_cand __ntc_text
  for __ntc_cand in "${__ntc_comp[@]}"; do
    [[ -z $__ntc_cand ]] && continue
    if (( __ntc_dollar )); then
      __ntc_text="\$${__ntc_cand}"
    else
      __ntc_text=$__ntc_cand
    fi
    __ntc_out="${__ntc_out}"$'\t'"$(__ntc_b64 "$__ntc_text")"
  done
  __ntc_reply "$__ntc_id" "$__ntc_out"
}
while IFS= read -r __ntc_req; do
  [[ -z $__ntc_req ]] && continue
  __ntc_id=''
  __ntc_op=''
  __ntc_payload=''
  __ntc_extra=''
  IFS=$'\t' read -r __ntc_id __ntc_op __ntc_payload __ntc_extra <<< "$__ntc_req"
  case $__ntc_op in
    run)
      __ntc_script=$(__ntc_unb64 "$__ntc_payload")
      __ntc_before=$(__ntc_name_count)
      eval "$__ntc_script" </dev/null
      __ntc_code=$?
      __ntc_after=$(__ntc_name_count)
      __ntc_dirty=0
      [[ $__ntc_before != $__ntc_after ]] && __ntc_dirty=1
      __ntc_reply "$__ntc_id" "$__ntc_code"$'\t'"$__ntc_dirty"
      ;;
    state)
      __ntc_reply "$__ntc_id" "$(__ntc_b64 "$PWD")"
      ;;
    names)
      # Shell-local names only: scanning every PATH entry would block the
      # request protocol, and ntc adds PATH executables from its own cache.
      if (( __ntc_has_parameter )); then
        __ntc_name_arr=( ${(k)aliases} ${(k)builtins} ${(k)functions} ${(k)reswords} )
        __ntc_name_arr=( ${__ntc_name_arr:#__ntc_*} )
        __ntc_names=$( print -rl -- ${__ntc_name_arr} 2>/dev/null | LC_ALL=C sort -u )
      else
        __ntc_names=''
      fi
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
    format!("\x1eNTCZ{}x{}\x1e", std::process::id(), nanos)
}

fn spawn_host() -> Option<Host> {
    if !platform::is_macos() {
        return None;
    }
    let exe = which::which("zsh").ok()?;
    let marker = marker_for_session();
    let mut child = Command::new(exe)
        // `-f`: no user rc files, so a broken `.zshrc` cannot stall the host.
        .args(["-f", "-c", BOOTSTRAP])
        .env("NTC_ZSH_TAG", &marker)
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
        .name("ntc-zsh-host".to_string())
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

/// True when a zsh launcher exists. The integrated mode is macOS-only.
pub fn available() -> bool {
    if !platform::is_macos() {
        return false;
    }
    if UNAVAILABLE.load(Ordering::Relaxed) {
        return false;
    }
    which::which("zsh").is_ok()
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
    ZSH_MODE.store(false, Ordering::SeqCst);
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
pub fn enter_zsh_mode() -> Result<()> {
    if !platform::is_macos() {
        return Err(anyhow!(crate::t!("msg.setIS.zsh_macos_only")));
    }
    let mut slot = host_slot()
        .lock()
        .map_err(|_| anyhow!("zsh host lock poisoned"))?;
    if !ensure_host(&mut slot) {
        return Err(anyhow!(crate::t!("msg.setIS.zsh_missing")));
    }
    let host = slot.as_mut().expect("host just ensured");
    // Set the mode before warming up so Ctrl-C during startup can drop the
    // child through the shell's interrupt handler.
    ZSH_MODE.store(true, Ordering::SeqCst);
    if request(host, "state", "", None, Duration::from_secs(8), None).is_none() {
        ZSH_MODE.store(false, Ordering::SeqCst);
        let _ = slot.take();
        kill_host();
        return Err(anyhow!(crate::t!("msg.setIS.zsh_start_failed")));
    }
    drop(slot);
    let _ = command_names(Duration::from_secs(8));
    Ok(())
}

/// Allow a retry after `invalidate()` (PATH or config changed).
pub(crate) fn reset_probe() {
    UNAVAILABLE.store(false, Ordering::Relaxed);
}

pub fn leave_zsh_mode() {
    ZSH_MODE.store(false, Ordering::SeqCst);
}

pub fn zsh_mode_active() -> bool {
    ZSH_MODE.load(Ordering::SeqCst)
}

/// Run one line in the persistent host, streaming its output to stdout.
pub fn run(line: &str) -> Result<i32> {
    let mut slot = host_slot()
        .lock()
        .map_err(|_| anyhow!("zsh host lock poisoned"))?;
    if !ensure_host(&mut slot) {
        return Err(anyhow!(crate::t!("msg.setIS.zsh_missing")));
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
            Err(anyhow!(crate::t!("msg.setIS.zsh_exited")))
        }
        None => Err(anyhow!(crate::t!("msg.setIS.zsh_stopped"))),
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

/// Shell-local command names (aliases, builtins, functions, reserved words)
/// without touching the host, or `None` while the fetch is still pending.
/// PATH executables come from ntc's own cached scan, so they are not
/// repeated here.
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
        .name("ntc-zsh-names".to_string())
        .spawn(|| {
            let _ = command_names(Duration::from_secs(10));
            NAMES_PREFETCHING.store(false, Ordering::SeqCst);
        });
    if spawned.is_err() {
        NAMES_PREFETCHING.store(false, Ordering::SeqCst);
    }
}

/// Completion for a zsh command line, answered by the persistent host so
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
        // zsh's own completion appends a separator; the line editor never
        // does, so trim it from the candidate (`checkout ` → `checkout`).
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

/// Convert a character index (how zsh counts `${#…}`) to a byte index.
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
        assert_eq!(char_to_byte("abc", 3), Some(3));
        assert_eq!(char_to_byte("abc", 4), None);
        assert_eq!(char_to_byte("aé b", 2), Some(3));
    }

    #[test]
    fn quoting_state_is_tracked() {
        assert!(!inside_quotes(""));
        assert!(inside_quotes("echo 'unterminated"));
        assert!(!inside_quotes("echo 'done' "));
    }

    #[test]
    fn candidates_with_spaces_are_escaped() {
        assert_eq!(shell_escape("plain"), "plain");
        assert_eq!(shell_escape("My File"), "My\\ File");
    }

    #[test]
    fn complete_reply_is_parsed_into_byte_ranges() {
        let body = format!("4\t2\t{}", b64("checkout"));
        let matches = parse_complete("git ch", &body);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].text, "checkout");
        assert_eq!((matches[0].start, matches[0].length), (4, 2));
    }

    #[test]
    fn bootstrap_targets_the_zsh_host() {
        assert!(BOOTSTRAP.contains("${(k)aliases}"));
        assert!(BOOTSTRAP.contains("__ntc_reply"));
        // The host must not depend on a user's compinit setup.
        assert!(!BOOTSTRAP.contains("compinit"));
    }

    /// Real round-trip against the persistent host. Skips off macOS or when
    /// zsh is not installed.
    #[test]
    fn host_keeps_state_and_reports_errors() {
        if !available() {
            return;
        }
        let _lock = crate::utils::test_support::lock_globals();

        let old = std::env::current_dir().ok();
        assert_eq!(run("ntc_var=42").expect("assignment should run"), 0);
        assert_eq!(run("test \"$ntc_var\" = 42").expect("test should run"), 0);
        let _ = command_names(Duration::from_secs(10));
        assert_eq!(run("alias ntc_alias=true").expect("alias should run"), 0);
        assert!(
            cached_command_names().is_none(),
            "defining an alias must invalidate the name cache"
        );
        let names = command_names(Duration::from_secs(10)).expect("names request failed");
        assert!(
            names.iter().any(|n| n == "ntc_alias"),
            "new alias missing from {names:?}"
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
    fn enter_and_leave_zsh_mode() {
        if !available() {
            return;
        }
        let _lock = crate::utils::test_support::lock_globals();
        let _ = run("true");
        enter_zsh_mode().expect("host should start");
        assert!(zsh_mode_active());
        leave_zsh_mode();
        assert!(!zsh_mode_active());
        shutdown();
    }
}
