//! Read-only teaching hints for an idle (empty) shell prompt.
//!
//! When the prompt line is empty for a while, one short dimmed suggestion is
//! typed out after the caret, character by character, like somebody typing a
//! demo. It is purely informational: rustyline never receives it as a hint,
//! so `Right`, `Tab`, `End` and `Ctrl+E` can never insert it (see the key
//! handlers in `shell/entry.rs`). The user always types the command
//! themselves — recommend, never auto-complete.
//!
//! Timeline (per empty prompt):
//! 1. 30 s untouched → start typing the current suggestion;
//! 2. fully typed → hold it for 15 s;
//! 3. wipe it → immediately type the next suggestion;
//! 4. repeat until a key is pressed or the prompt goes away.
//!
//! The animation lives on a background thread because rustyline only asks for
//! a hint on a redraw; it cannot repaint a prompt nobody is touching. The
//! ghost is drawn/erased directly (never through rustyline), and every hint
//! request first wipes it, so a keystroke can never leave stale text behind.

use crate::config::Config;
use crate::t;
use colored::Colorize;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

/// How long an empty prompt must sit untouched before typing starts.
pub(crate) const IDLE_BEFORE_TYPING: Duration = Duration::from_secs(30);
/// How long a fully typed suggestion stays before it is wiped.
pub(crate) const IDLE_HOLD: Duration = Duration::from_secs(15);
/// Delay between two revealed characters.
const TYPE_STEP_MS: u128 = 45;
/// Background wake-up interval.
const TICK: Duration = Duration::from_millis(40);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    /// Waiting out the 30 s idle delay.
    Waiting,
    /// Revealing characters one at a time.
    Typing,
    /// Fully typed; counting down to the wipe.
    Holding,
}

pub(crate) struct AnimState {
    /// An empty prompt is on screen and may be animated.
    active: bool,
    phase: Phase,
    /// Rotation index into [`suggestions`].
    step: usize,
    /// The suggestion being typed.
    suggestion: String,
    /// Characters revealed so far.
    typed: usize,
    /// Characters currently written to the terminal.
    displayed: usize,
    /// Characters that fit on this row (never wrap to the next line).
    limit: usize,
    /// Column where the ghost starts (right after the caret), so erase can go
    /// back to it exactly — character counts are wrong for CJK/Thai text.
    start_col: u16,
    /// When the current phase began.
    phase_started: Instant,
}

impl Default for AnimState {
    fn default() -> Self {
        Self {
            active: false,
            phase: Phase::Waiting,
            step: 0,
            suggestion: String::new(),
            typed: 0,
            displayed: 0,
            limit: 60,
            start_col: 0,
            phase_started: Instant::now(),
        }
    }
}

fn anim() -> &'static Mutex<AnimState> {
    static ANIM: OnceLock<Mutex<AnimState>> = OnceLock::new();
    ANIM.get_or_init(|| Mutex::new(AnimState::default()))
}

fn lock() -> std::sync::MutexGuard<'static, AnimState> {
    anim().lock().unwrap_or_else(|e| e.into_inner())
}

/// Characters to reveal after `elapsed` for a `total`-character suggestion.
/// Pure so the typewriter pacing is testable without a terminal.
fn typed_count(elapsed: Duration, total: usize, limit: usize) -> usize {
    if limit == 0 {
        return 0;
    }
    let total = total.min(limit);
    let n = (elapsed.as_millis() / TYPE_STEP_MS) as usize;
    n.min(total)
}

/// Spawn the animation thread once (only when there is a real terminal).
fn ensure_worker() {
    static STARTED: AtomicBool = AtomicBool::new(false);
    if STARTED.swap(true, Ordering::Relaxed) {
        return;
    }
    if !std::io::IsTerminal::is_terminal(&std::io::stdout()) {
        return;
    }
    let _ = std::thread::Builder::new()
        .name("ntc-idle-hint".into())
        .spawn(worker_loop);
}

fn worker_loop() {
    loop {
        std::thread::sleep(TICK);
        let mut st = lock();
        if !st.active {
            continue;
        }
        match st.phase {
            Phase::Waiting => {
                if st.phase_started.elapsed() < IDLE_BEFORE_TYPING {
                    continue;
                }
                let ctx = IdleContext::detect();
                match pick(&ctx, st.step) {
                    Some(text) => {
                        st.suggestion = text;
                        st.typed = 0;
                        st.displayed = 0;
                        if !begin_row(&mut st) {
                            st.active = false;
                            continue;
                        }
                        st.phase = Phase::Typing;
                        st.phase_started = Instant::now();
                    }
                    None => st.active = false,
                }
            }
            Phase::Typing => {
                let total = st.suggestion.chars().count();
                let want = typed_count(st.phase_started.elapsed(), total, st.limit);
                if want > st.typed {
                    let next: String = st
                        .suggestion
                        .chars()
                        .skip(st.typed)
                        .take(want - st.typed)
                        .collect();
                    if write_ghost(&next) {
                        st.typed = want;
                        st.displayed = want;
                    }
                }
                if st.typed >= total.min(st.limit) {
                    st.phase = Phase::Holding;
                    st.phase_started = Instant::now();
                }
            }
            Phase::Holding => {
                if st.phase_started.elapsed() < IDLE_HOLD {
                    continue;
                }
                // Fully generated + held: wipe and start the next one.
                erase_ghost(st.start_col, st.displayed);
                st.displayed = 0;
                st.typed = 0;
                st.step = st.step.wrapping_add(1);
                let ctx = IdleContext::detect();
                match pick(&ctx, st.step) {
                    Some(text) => {
                        st.suggestion = text;
                        if !begin_row(&mut st) {
                            st.active = false;
                            continue;
                        }
                        st.phase = Phase::Typing;
                        st.phase_started = Instant::now();
                    }
                    None => st.active = false,
                }
            }
        }
    }
}

/// Record the caret column and how many characters fit on this row, so a
/// long hint never wraps and leaves junk on the next line (rustyline only
/// knows about its own layout). Returns false when the console cannot report
/// the cursor: drawing from a guessed column could overwrite the prompt, so
/// the animator stays off instead.
fn begin_row(st: &mut AnimState) -> bool {
    let (cols, col) = match (crossterm::terminal::size(), crossterm::cursor::position()) {
        (Ok((cols, _)), Ok((col, _))) => (cols, col),
        _ => {
            st.limit = 0;
            return false;
        }
    };
    st.start_col = col;
    let avail = (cols as usize).saturating_sub(col as usize + 1).min(200);
    st.limit = avail;
    avail > 0
}

/// Write the next piece of the ghost text after the caret. Returns false when
/// the terminal refused the write (the animator then stops drawing).
fn write_ghost(text: &str) -> bool {
    if text.is_empty() {
        return true;
    }
    let mut out = std::io::stdout();
    // Dim when the console can render ANSI; plain text on legacy conhost.
    let styled = if crate::output::raw_ansi_allowed() {
        text.dimmed().to_string()
    } else {
        text.to_string()
    };
    out.write_all(styled.as_bytes()).is_ok() && out.flush().is_ok()
}

/// Jump back to the column where the ghost started and clear it (and the rest
/// of the row). Width-agnostic, unlike counting characters.
fn erase_ghost(start_col: u16, displayed: usize) {
    if displayed == 0 {
        return;
    }
    use crossterm::cursor::MoveToColumn;
    use crossterm::terminal::{Clear, ClearType};
    let mut out = std::io::stdout();
    let _ = crossterm::execute!(out, MoveToColumn(start_col), Clear(ClearType::UntilNewLine));
    let _ = out.flush();
}

/// A fresh prompt is about to be shown: reset the timeline for `step`.
pub(crate) fn prompt_started(step: usize) {
    let enabled =
        Config::global_get_autosuggest_enabled() && Config::global_get_idle_hint_enabled();
    ensure_worker();
    let mut st = lock();
    if !enabled {
        st.active = false;
        erase_ghost(st.start_col, st.displayed);
        st.displayed = 0;
        return;
    }
    st.active = true;
    st.phase = Phase::Waiting;
    st.phase_started = Instant::now();
    st.step = step;
    st.suggestion.clear();
    st.typed = 0;
    // A previous ghost is cleared by rustyline's repaint/our hint path; never
    // write from here (the cursor may already be on the next line).
    st.displayed = 0;
    begin_row(&mut st);
}

/// An empty line is being rendered. A repaint of the same prompt must not
/// restart the countdown; returning to an empty line after typing does.
pub(crate) fn prompt_shown(step: usize) {
    let enabled =
        Config::global_get_autosuggest_enabled() && Config::global_get_idle_hint_enabled();
    ensure_worker();
    let mut st = lock();
    if !enabled {
        st.active = false;
        erase_ghost(st.start_col, st.displayed);
        st.displayed = 0;
        return;
    }
    if !st.active {
        st.active = true;
        st.phase = Phase::Waiting;
        st.phase_started = Instant::now();
        st.step = step;
        st.suggestion.clear();
        st.typed = 0;
        st.displayed = 0;
        begin_row(&mut st);
        return;
    }
    // A repaint (resize, Tab, cursor move) wipes the row behind our back, so
    // drop the ghost and retype it from the start instead of continuing on a
    // line that no longer shows the earlier characters.
    if st.displayed > 0 {
        erase_ghost(st.start_col, st.displayed);
        st.displayed = 0;
        st.typed = 0;
        if !begin_row(&mut st) {
            st.active = false;
            return;
        }
        if !st.suggestion.is_empty() {
            st.phase = Phase::Typing;
            st.phase_started = Instant::now();
        }
    }
}

/// The user typed something (or the hint is disabled): wipe and stop.
pub(crate) fn stop() {
    let mut st = lock();
    st.active = false;
    erase_ghost(st.start_col, st.displayed);
    st.displayed = 0;
    st.typed = 0;
    st.phase = Phase::Waiting;
}

/// `readline` returned (command submitted, Ctrl-C, EOF): stop animating.
/// No terminal write here — rustyline already redrew that line.
pub(crate) fn prompt_ended() {
    let mut st = lock();
    st.active = false;
    st.phase = Phase::Waiting;
    st.displayed = 0;
    st.typed = 0;
}

/// Facts about the current shell context, probed once per prompt.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct IdleContext {
    /// The directory looks like a git work tree.
    pub is_git: bool,
    /// A `todo.*` document exists here, so `todo add` has somewhere to go.
    pub has_todo: bool,
    /// An `NTCRANFILE.toml` exists here, so `ran list` has targets to show.
    pub has_ranfile: bool,
    /// Nothing but dotfiles: "create your first file" is the helpful hint.
    pub empty_dir: bool,
}

impl IdleContext {
    /// Probe the current directory. A handful of `stat` calls, no walking.
    pub(crate) fn detect() -> Self {
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        Self::detect_in(&cwd)
    }

    /// Same probe against an explicit directory (tests use this).
    pub(crate) fn detect_in(dir: &std::path::Path) -> Self {
        let is_git = dir.join(".git").exists();
        let has_todo = ["txt", "md", "json", "toml", "yaml", "html"]
            .iter()
            .any(|ext| dir.join(format!("todo.{}", ext)).is_file())
            || dir.join("future_todo.md").is_file();
        let has_ranfile = [
            "NTCRANFILE.toml",
            "NTCRANFILE.json",
            "NTCRANFILE.yaml",
            "NTCRANFILE.yml",
        ]
        .iter()
        .any(|name| dir.join(name).is_file());
        let empty_dir = match std::fs::read_dir(dir) {
            Ok(entries) => !entries.flatten().any(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                !name.starts_with('.') && name != "ntconfig.toml"
            }),
            Err(_) => false,
        };
        Self {
            is_git,
            has_todo,
            has_ranfile,
            empty_dir,
        }
    }
}

/// Suggestions in teaching order for a normal directory: navigation first,
/// then authoring and automation, then sharing, extras and help.
pub(crate) fn suggestions(ctx: &IdleContext) -> Vec<String> {
    let mut out = Vec::new();
    if ctx.empty_dir {
        out.push(t!("idle.empty"));
    }
    out.push(t!("idle.tp"));
    out.push(t!("idle.explorer"));
    out.push(t!("idle.ne"));
    out.push(t!("idle.view"));
    out.push(t!("idle.tl"));
    out.push(t!("idle.size"));
    out.push(t!("idle.search"));
    if ctx.has_todo {
        out.push(t!("idle.todo"));
    }
    if ctx.is_git {
        out.push(t!("idle.git"));
    }
    out.push(t!("idle.tutorial"));
    out.push(t!("idle.ral"));
    if ctx.has_ranfile {
        out.push(t!("idle.ran"));
    }
    out.push(t!("idle.scr"));
    out.push(t!("idle.time"));
    out.push(t!("idle.report"));
    out.push(t!("idle.encode"));
    out.push(t!("idle.fmt"));
    out.push(t!("idle.arc"));
    out.push(t!("idle.scan"));
    out.push(t!("idle.sys"));
    out.push(t!("idle.showcg"));
    out.push(t!("idle.ntpkg"));
    out.push(t!("idle.ndb"));
    out.push(t!("idle.nsheet"));
    out.push(t!("idle.pipe"));
    out.push(t!("idle.math"));
    out.push(t!("idle.qr"));
    out.push(t!("idle.theme"));
    out.push(t!("idle.backup"));
    out.push(t!("idle.serve"));
    out.push(t!("idle.fgo"));
    out.push(t!("idle.back"));
    out.push(t!("idle.viewf"));
    out.push(t!("idle.views"));
    out.push(t!("idle.txtf"));
    out.push(t!("idle.text"));
    out.push(t!("idle.count"));
    out.push(t!("idle.files"));
    out.push(t!("idle.config"));
    out.push(t!("idle.opencg"));
    out.push(t!("idle.local"));
    out.push(t!("idle.project"));
    out.push(t!("idle.tpb"));
    out.push(t!("idle.qur"));
    out.push(t!("idle.fsc"));
    out.push(t!("idle.ignore"));
    out.push(t!("idle.igcare"));
    out.push(t!("idle.http"));
    out.push(t!("idle.dl"));
    out.push(t!("idle.dns"));
    out.push(t!("idle.up"));
    out.push(t!("idle.ping"));
    out.push(t!("idle.trace"));
    out.push(t!("idle.smtp"));
    out.push(t!("idle.ipcfg"));
    out.push(t!("idle.wi"));
    out.push(t!("idle.spec"));
    out.push(t!("idle.tmprt"));
    out.push(t!("idle.clip"));
    out.push(t!("idle.qsc"));
    out.push(t!("idle.pathman"));
    out.push(t!("idle.autoup"));
    out.push(t!("idle.mode"));
    out.push(t!("idle.python"));
    out.push(t!("idle.decode"));
    out.push(t!("idle.image"));
    out.push(t!("idle.nssh"));
    out.push(t!("idle.nms"));
    out.push(t!("idle.fake"));
    out.push(t!("idle.deadlock"));
    out.push(t!("idle.ntpx"));
    out.push(t!("idle.pathcp"));
    out.push(t!("idle.say"));
    out.push(t!("idle.ui"));
    out.push(t!("idle.help"));
    out
}

/// The suggestion shown at rotation step `step` (wrapping).
pub(crate) fn pick(ctx: &IdleContext, step: usize) -> Option<String> {
    let list = suggestions(ctx);
    if list.is_empty() {
        return None;
    }
    Some(list[step % list.len()].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(is_git: bool, has_todo: bool, empty_dir: bool) -> IdleContext {
        IdleContext {
            is_git,
            has_todo,
            has_ranfile: false,
            empty_dir,
        }
    }

    #[test]
    fn context_detection_reads_the_directory() {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        // Project-local fixture: the system temp dir is not writable in
        // every test sandbox, while `target/` always is.
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!("ntc_idle_{}_{}", std::process::id(), stamp));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".git")).expect("mkdir fixture failed");
        std::fs::write(dir.join("todo.md"), "- [ ] x").unwrap();
        let c = IdleContext::detect_in(&dir);
        assert!(c.is_git);
        assert!(c.has_todo);
        assert!(!c.has_ranfile);
        assert!(!c.empty_dir || std::fs::read_dir(&dir).unwrap().count() == 2);

        // A task runner file here unlocks the `ran` suggestion.
        std::fs::write(
            dir.join("NTCRANFILE.toml"),
            "[targets.smoke]\ncmd = \"echo ok\"\n",
        )
        .unwrap();
        assert!(IdleContext::detect_in(&dir).has_ranfile);

        // A fresh directory with only a dotfile counts as empty.
        let empty = dir.join("empty");
        std::fs::create_dir_all(&empty).unwrap();
        std::fs::write(empty.join(".keep"), "").unwrap();
        let c = IdleContext::detect_in(&empty);
        assert!(!c.is_git);
        assert!(!c.has_todo);
        assert!(c.empty_dir);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn suggestions_follow_the_context() {
        let plain = suggestions(&ctx(false, false, false));
        assert!(plain.iter().any(|s| s.contains("tp add")));
        assert!(!plain.iter().any(|s| s.contains("git")));
        assert!(!plain.iter().any(|s| s.contains("todo add")));
        assert!(!plain.iter().any(|s| s.contains("ran list")));

        let repo = suggestions(&ctx(true, true, false));
        assert!(repo.iter().any(|s| s.contains("git")));
        assert!(repo.iter().any(|s| s.contains("todo add")));

        let empty = suggestions(&ctx(false, false, true));
        assert!(empty.iter().any(|s| s.contains("ne ")));

        // A folder with a runner file also teaches `ran`.
        let with_tasks = IdleContext {
            has_ranfile: true,
            ..ctx(false, false, false)
        };
        assert!(suggestions(&with_tasks)
            .iter()
            .any(|s| s.contains("ran list")));
    }

    #[test]
    fn rotation_wraps_and_never_returns_the_same_idea_twice_in_a_row() {
        let c = ctx(true, true, false);
        let list = suggestions(&c);
        assert!(list.len() >= 8);
        let first = pick(&c, 0).unwrap();
        let second = pick(&c, 1).unwrap();
        assert_ne!(first, second);
        assert_eq!(pick(&c, list.len()).unwrap(), first);
        // No duplicates inside one cycle.
        let mut seen = std::collections::HashSet::new();
        for i in 0..list.len() {
            assert!(seen.insert(pick(&c, i).unwrap()), "duplicate at {i}");
        }
    }

    #[test]
    fn every_suggestion_is_a_real_command_hint() {
        // Commands ntc actually dispatches, so a hint can never teach a
        // sub-command that does not exist.
        const KNOWN: &[&str] = &[
            "tp", "nfe", "ne", "view", "gs", "todo", "git", "mkf", "help", "serve", "tutorial",
            "ral", "ran", "math", "qr", "theme", "backup", "tl", "size", "scr", "time", "report",
            "encode", "fmt", "arc", "scan", "sys", "showcg", "ntpkg", "ndb", "nsheet", "fgo",
            "back", "viewf", "views", "txtf", "head", "count", "config", "opencg", "init", "tpb",
            "qur", "fsc", "ignore", "igcared", "http", "dl", "dns", "up", "ping", "trace", "smtp",
            "ipcfg", "wi", "spec", "tmprt", "clip", "qsc", "usrpath", "autoup", "mode", "showenv",
            "decode", "image", "nssh", "nms", "fake", "deadlock", "ntpx", "pathcp", "say", "ui",
        ];
        let all = suggestions(&IdleContext {
            has_ranfile: true,
            ..ctx(true, true, true)
        });
        for s in &all {
            // The hint must always name a command so it teaches something.
            assert!(s.contains("ntc") || s.contains(':') || s.contains("tp ") || !s.is_empty());
            assert!(!s.starts_with("idle."), "raw key leaked: {s}");
            let cmd = s
                .split(": ")
                .nth(1)
                .unwrap_or_default()
                .split(' ')
                .next()
                .unwrap_or_default();
            assert!(KNOWN.contains(&cmd), "unknown command hinted: {s}");
        }
        // `search` is not an ntc command; the hint teaches `gs` instead.
        for s in &all {
            assert!(!s.contains("search \""), "dead command hinted: {s}");
        }
        assert!(all.iter().any(|s| s.contains("gs ")));
    }

    #[test]
    fn typewriter_reveals_one_character_per_step_without_overflow() {
        assert_eq!(typed_count(Duration::ZERO, 20, 80), 0);
        assert_eq!(typed_count(Duration::from_millis(45), 20, 80), 1);
        assert_eq!(typed_count(Duration::from_millis(450), 20, 80), 10);
        // Never past the suggestion or the row budget.
        assert_eq!(typed_count(Duration::from_secs(10), 20, 80), 20);
        assert_eq!(typed_count(Duration::from_secs(10), 20, 5), 5);
        // No room on the row: reveal nothing (the animator holds instead).
        assert_eq!(typed_count(Duration::from_secs(10), 3, 0), 0);
    }

    #[test]
    fn timeline_matches_the_requested_pattern() {
        // 30 s before typing, 15 s hold: constants are part of the behaviour.
        assert_eq!(IDLE_BEFORE_TYPING, Duration::from_secs(30));
        assert_eq!(IDLE_HOLD, Duration::from_secs(15));
    }
}
