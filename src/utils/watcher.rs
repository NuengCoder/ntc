// watcher.rs — Re-architected file watcher for ntc
//
// Architecture:
//   - Ignore-aware: skips events from ignored dirs (target/, .git/, etc.)
//   - Batched debounce: all events in the 400ms window collapsed into one WatchSummary
//   - Filetype-aware: labels events with a human-readable category
//   - Watch-trigger alias: optional auto-run of a named alias on change
//   - Clean poll() API: shell.rs calls poll() once per loop, gets Option<WatchSummary>

use crate::config::Config;
use anyhow::Result;
use colored::*;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// How long to wait after the last event before reporting (debounce window)
const DEBOUNCE_MS: u64 = 400;

// ─────────────────────────────────────────────────────────────────────────────
// Public data types
// ─────────────────────────────────────────────────────────────────────────────

/// The kind of filesystem change.
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeKind {
    Created,
    Modified,
    Deleted,
    Other,
}

impl ChangeKind {
    fn label(&self) -> &'static str {
        match self {
            ChangeKind::Created  => "created",
            ChangeKind::Modified => "modified",
            ChangeKind::Deleted  => "deleted",
            ChangeKind::Other    => "changed",
        }
    }

    fn color_label(&self) -> colored::ColoredString {
        match self {
            ChangeKind::Created  => "created".green(),
            ChangeKind::Modified => "modified".yellow(),
            ChangeKind::Deleted  => "deleted".red(),
            ChangeKind::Other    => "changed".cyan(),
        }
    }
}

/// One meaningful change event (after filtering + enrichment).
#[derive(Debug, Clone)]
pub struct WatchEvent {
    pub kind: ChangeKind,
    /// File/dir name only (e.g. "main.rs")
    pub name: String,
    /// Human-readable type label (e.g. "Rust source", "Config file")
    pub file_type: String,
}

impl WatchEvent {
    /// Format as a compact display line, e.g. `[modified] main.rs  Rust source`
    pub fn display(&self) -> String {
        format!(
            "[{}] {}  {}",
            self.kind.color_label(),
            self.name.bold(),
            self.file_type.dimmed()
        )
    }
}

/// What shell.rs receives after one debounce window fires.
pub struct WatchSummary {
    /// Individual events (deduplicated by name+kind)
    pub events: Vec<WatchEvent>,
    /// Alias to auto-run, taken from config at poll time (if any)
    pub trigger_alias: Option<String>,
}

impl WatchSummary {
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Print the summary box to stdout.
    pub fn print_change_box(&self) {
        // Box inner width: 44 visible chars between ║  …  ║ (2 spaces each side)
        const INNER: usize = 44;
        println!();
        println!("  ╔════════════════════════════════════════════════╗");
        println!("  ║{}║", format!("{:^48}", "← file watcher detected").cyan().bold());
        println!("  ╠════════════════════════════════════════════════╣");
        let display_events = &self.events[..self.events.len().min(MAX_DISPLAY_EVENTS)];
        for ev in display_events {
            // Measure using plain text (no ANSI), pad with spaces, then print colored
            let plain = format!("[{}] {}  {}", ev.kind.label(), ev.name, ev.file_type);
            let pad = INNER.saturating_sub(plain.len());
            println!(
                "  ║  [{}] {}  {}{}  ║",
                ev.kind.color_label(),
                ev.name.bold(),
                ev.file_type.dimmed(),
                " ".repeat(pad),
            );
        }

        if self.events.len() > MAX_DISPLAY_EVENTS {
            let msg = format!("… and {} more", self.events.len() - MAX_DISPLAY_EVENTS);
            let pad = INNER.saturating_sub(msg.len());
            println!("  ║  {}{}  ║", msg.dimmed(), " ".repeat(pad));
        }

        println!("  ╚════════════════════════════════════════════════╝");
    }

    /// Print the "tree refreshed" footer.
    pub fn print_refresh_box(&self) {
        println!("  ╔════════════════════════════════════════════════╗");
        println!("  ║{}║", format!("{:^48}", "watcher → tree refreshed").green().bold());
        println!("  ╚════════════════════════════════════════════════╝"); 
    }
}

// Max events shown in the box before "… and N more"
const MAX_DISPLAY_EVENTS: usize = 8;

// ─────────────────────────────────────────────────────────────────────────────
// Internal pending-event buffer (lives in the notify callback thread)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct PendingEvent {
    kind: ChangeKind,
    name: String,
    full_path: PathBuf,
}

// ─────────────────────────────────────────────────────────────────────────────
// WatcherHandle — the public handle shell.rs holds
// ─────────────────────────────────────────────────────────────────────────────

pub struct WatcherHandle {
    /// Set to true by the notify callback when a relevant event arrives.
    changed: Arc<AtomicBool>,
    /// Timestamp of the most-recent relevant event (for debounce).
    last_event: Arc<Mutex<Instant>>,
    /// Buffered events accumulated since the last poll().
    pending: Arc<Mutex<Vec<PendingEvent>>>,
    /// The underlying notify watcher (must be kept alive).
    watcher: Mutex<notify::RecommendedWatcher>,
    /// Currently watched path.
    current_path: Mutex<PathBuf>,
}

impl WatcherHandle {
    /// Call once per shell loop iteration.
    ///
    /// Returns `Some(WatchSummary)` when the debounce window has expired and
    /// there are pending events; `None` otherwise.
    pub fn poll(&self) -> Option<WatchSummary> {
        // Fast path: nothing pending.
        if !self.changed.load(Ordering::Acquire) {
            return None;
        }

        // Debounce: wait until at least DEBOUNCE_MS since the last event.
        let elapsed = self.last_event.lock().unwrap().elapsed();
        if elapsed < Duration::from_millis(DEBOUNCE_MS) {
            return None;
        }

        // Drain the pending buffer.
        let raw: Vec<PendingEvent> = {
            let mut lock = self.pending.lock().unwrap();
            std::mem::take(&mut *lock)
        };
        self.changed.store(false, Ordering::Release);

        if raw.is_empty() {
            return None;
        }

        // Enrich and deduplicate.
        let events = enrich_and_dedup(raw);

        // Read trigger alias from config at poll time (cheap read-lock).
        let trigger_alias = Config::global().read().unwrap()
            .watch_trigger_alias
            .clone();

        Some(WatchSummary { events, trigger_alias })
    }

    /// Update the watched path (called by shell.rs after every navigation).
    pub fn update_path(&self, new_path: &Path) -> Result<()> {
        let old = self.current_path.lock().unwrap().clone();
        if old == new_path {
            return Ok(());
        }
        let mut watcher = self.watcher.lock().unwrap();
        let _ = watcher.unwatch(&old);
        watcher.watch(new_path, RecursiveMode::Recursive)?;
        *self.current_path.lock().unwrap() = new_path.to_path_buf();
        Ok(())
    }

    /// Current watched path (for the prompt indicator).
    pub fn watched_path(&self) -> PathBuf {
        self.current_path.lock().unwrap().clone()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// start_watcher — public constructor
// ─────────────────────────────────────────────────────────────────────────────

/// Start watching `path` recursively.  Returns a `WatcherHandle` to poll from
/// the shell loop.
pub fn start_watcher(path: &Path) -> Result<WatcherHandle> {
    let changed      = Arc::new(AtomicBool::new(false));
    let last_event   = Arc::new(Mutex::new(Instant::now()));
    let pending: Arc<Mutex<Vec<PendingEvent>>> = Arc::new(Mutex::new(Vec::new()));

    let changed_c    = changed.clone();
    let last_event_c = last_event.clone();
    let pending_c    = pending.clone();

    let mut watcher = notify::recommended_watcher(
        move |res: Result<Event, notify::Error>| {
            let Ok(event) = res else { return };

            let kind = match event.kind {
                EventKind::Create(_) => ChangeKind::Created,
                EventKind::Modify(_) => ChangeKind::Modified,
                EventKind::Remove(_) => ChangeKind::Deleted,
                _                    => return, // access, meta, other — ignore
            };

            // Filter: skip paths that belong to an ignored directory segment.
            let ignored = Config::global_get_ignored_dirs();
            for path in &event.paths {
                if is_in_ignored_dir(path, &ignored) {
                    continue;
                }
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("?")
                    .to_string();

                let mut lock = pending_c.lock().unwrap();
                lock.push(PendingEvent {
                    kind: kind.clone(),
                    name,
                    full_path: path.clone(),
                });
            }

            *last_event_c.lock().unwrap() = Instant::now();
            changed_c.store(true, Ordering::Release);
        },
    )?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    Ok(WatcherHandle {
        changed,
        last_event,
        pending,
        watcher: Mutex::new(watcher),
        current_path: Mutex::new(path.to_path_buf()),
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper: ignored-dir check
// ─────────────────────────────────────────────────────────────────────────────

/// Returns true if any component of `path` matches an ignored directory name.
fn is_in_ignored_dir(path: &Path, ignored: &std::collections::HashSet<String>) -> bool {
    path.components().any(|c| {
        let s = c.as_os_str().to_string_lossy();
        ignored.contains(s.as_ref())
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper: enrich + deduplicate raw events
// ─────────────────────────────────────────────────────────────────────────────

fn enrich_and_dedup(raw: Vec<PendingEvent>) -> Vec<WatchEvent> {
    let mut seen: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    let mut out: Vec<WatchEvent> = Vec::new();

    for ev in raw {
        let key = (ev.kind.label().to_string(), ev.name.clone());
        if seen.contains(&key) {
            continue;
        }
        seen.insert(key);

        let file_type = classify_file_type(&ev.full_path);
        out.push(WatchEvent {
            kind: ev.kind,
            name: ev.name,
            file_type,
        });

        if out.len() >= MAX_DISPLAY_EVENTS + 1 {
            // Keep collecting for the count, but stop enriching.
            break;
        }
    }

    out
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper: file-type classification (no lock needed — pure logic)
// ─────────────────────────────────────────────────────────────────────────────

fn classify_file_type(path: &Path) -> String {
    // Directory check first
    if path.is_dir() || path.extension().is_none() && !path.is_file() {
        return "directory".to_string();
    }

    let name_lower = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Special filenames (no extension)
    for kw in &["dockerfile", "makefile", "readme", "license", "gitignore", ".env"] {
        if name_lower.starts_with(kw) {
            return capitalize(kw);
        }
    }

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        // Rust
        "rs"             => "Rust source".to_string(),
        // Web
        "js" | "mjs"     => "JavaScript".to_string(),
        "ts"             => "TypeScript".to_string(),
        "jsx" | "tsx"    => "React component".to_string(),
        "html" | "htm"   => "HTML".to_string(),
        "css" | "scss"   => "Stylesheet".to_string(),
        // Config / data
        "toml"           => "Config file (TOML)".to_string(),
        "yaml" | "yml"   => "Config file (YAML)".to_string(),
        "json"           => "JSON".to_string(),
        "xml"            => "XML".to_string(),
        "env"            => "Env file".to_string(),
        "ini" | "cfg" | "conf" => "Config file".to_string(),
        // Docs
        "md" | "mdx"     => "Markdown".to_string(),
        "txt"            => "Text file".to_string(),
        // Systems / compiled
        "c"              => "C source".to_string(),
        "cpp" | "cc" | "cxx" => "C++ source".to_string(),
        "h" | "hpp"      => "Header file".to_string(),
        "go"             => "Go source".to_string(),
        "py"             => "Python".to_string(),
        "java"           => "Java".to_string(),
        "kt" | "kts"     => "Kotlin".to_string(),
        "swift"          => "Swift".to_string(),
        "cs"             => "C# source".to_string(),
        "dart"           => "Dart".to_string(),
        "rb"             => "Ruby".to_string(),
        "php"            => "PHP".to_string(),
        "lua"            => "Lua".to_string(),
        "sh" | "bash"    => "Shell script".to_string(),
        "bat" | "cmd"    => "Batch script".to_string(),
        "ps1"            => "PowerShell".to_string(),
        "sql"            => "SQL".to_string(),
        "r"              => "R script".to_string(),
        "scala"          => "Scala".to_string(),
        // Assets
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "svg" | "webp"
                         => "Image".to_string(),
        "mp3" | "wav" | "ogg" | "flac" => "Audio".to_string(),
        "mp4" | "avi" | "mkv" | "mov"  => "Video".to_string(),
        // Archives
        "zip" | "tar" | "gz" | "rar" | "7z" => "Archive".to_string(),
        // Binaries
        "exe" | "dll" | "so" | "dylib"  => "Binary".to_string(),
        "lock"           => "Lock file".to_string(),
        // MQL
        "mq4" | "mq5"   => "MQL source".to_string(),
        "mqh"            => "MQL header".to_string(),
        "ntc.ral" | "ntc.igcare" => "NTC source".to_string(),
        _                => format!(".{} file", ext),
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None    => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}