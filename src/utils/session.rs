use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

/// Maximum number of per-directory tab snapshots kept in `session.toml`.
/// Oldest directories are pruned once the limit is exceeded so the file
/// cannot grow without bound.
pub const MAX_SESSION_DIRS: usize = 24;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditorSession {
    pub current_file: PathBuf,
    pub cursor_y: usize,
    pub cursor_byte: usize,
    pub scroll: usize,
    pub scroll_x: usize,
    pub buffer_stack: Vec<PathBuf>,
    pub buffer_idx: usize,
    pub sidebar_open: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TabSnapshot {
    pub path: PathBuf,
    pub cursor_y: usize,
    pub cursor_byte: usize,
    pub scroll: usize,
    pub scroll_x: usize,
    pub modified: bool,
    #[serde(default)]
    pub undo_stack: Vec<crate::editor::Snapshot>,
    #[serde(default)]
    pub redo_stack: Vec<crate::editor::Snapshot>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionSnapshot {
    pub tabs: Vec<TabSnapshot>,
    pub active_tab: usize,
    pub buffer_stack: Vec<PathBuf>,
    pub buffer_idx: usize,
    pub sidebar_open: bool,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionState {
    pub last_directory: Option<PathBuf>,
    pub editor_session: Option<EditorSession>,
    #[serde(default)]
    pub nav_stack: Vec<PathBuf>,
    /// Editor tab snapshots scoped to the directory the editor was started
    /// from, keyed by the normalized form of that directory. Opening the
    /// editor in a different directory therefore never shows another
    /// directory's tabs.
    #[serde(default)]
    pub session_snapshots: BTreeMap<String, SessionSnapshot>,
}

/// Normalized key for a working directory: the Windows `\\?\` extended
/// prefix is stripped (and separators/case unified on Windows), so the same
/// directory always addresses the same snapshot bucket.
pub fn dir_key(dir: &Path) -> String {
    let s = crate::filetype::strip_unc_prefix(dir);
    #[cfg(windows)]
    {
        s.replace('\\', "/").trim_end_matches('/').to_lowercase()
    }
    #[cfg(not(windows))]
    {
        s.trim_end_matches('/').to_string()
    }
}

/// The directory the editor was started from (the process CWD, which the
/// shell keeps in sync with its navigator).
pub fn editor_cwd() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Snapshot whose tabs should be restored when the editor starts in `cwd`:
/// the bucket for exactly that directory, still within `max_age_hours`, with
/// at least one tab. Snapshots saved from other directories are never
/// considered, so one directory's tabs can't leak into another.
pub fn snapshot_for_restore(
    state: &SessionState,
    cwd: &Path,
    max_age_hours: u64,
) -> Option<SessionSnapshot> {
    let snapshot = state.snapshot_for(cwd)?;
    if snapshot.tabs.is_empty() {
        return None;
    }
    let is_recent = chrono::NaiveDateTime::parse_from_str(&snapshot.timestamp, "%Y-%m-%d %H:%M:%S")
        .ok()
        .map(|dt| {
            let now = chrono::Local::now().naive_local();
            let age = now.signed_duration_since(dt).num_hours();
            age < max_age_hours as i64
        })
        .unwrap_or(false);
    if is_recent {
        Some(snapshot.clone())
    } else {
        None
    }
}

impl SessionState {
    fn session_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("ntc").join("session.toml"))
    }

    fn empty() -> Self {
        SessionState {
            last_directory: None,
            editor_session: None,
            nav_stack: Vec::new(),
            session_snapshots: BTreeMap::new(),
        }
    }

    /// Read a session file from `path`, or `None` when it is missing or
    /// unreadable (a corrupt file must never break startup).
    fn load_from(path: &Path) -> Option<Self> {
        let content = fs::read_to_string(path).ok()?;
        toml::from_str::<SessionState>(&content).ok()
    }

    /// Write this session to `path`, creating the parent directory.
    fn save_to(&self, path: &Path) {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(toml_str) = toml::to_string_pretty(self) {
            let _ = fs::write(path, toml_str);
        }
    }

    pub fn load() -> Self {
        Self::session_path()
            .and_then(|path| Self::load_from(&path))
            .unwrap_or_else(Self::empty)
    }

    /// Tab snapshot stored for `dir`, if any.
    pub fn snapshot_for(&self, dir: &Path) -> Option<&SessionSnapshot> {
        self.session_snapshots.get(&dir_key(dir))
    }

    /// Store (or replace) the tab snapshot for `dir`, pruning the oldest
    /// buckets beyond `MAX_SESSION_DIRS`.
    pub fn set_snapshot(&mut self, dir: &Path, snapshot: SessionSnapshot) {
        self.session_snapshots.insert(dir_key(dir), snapshot);
        self.prune_snapshots();
    }

    /// Drop the oldest directory snapshots once the cap is exceeded.
    fn prune_snapshots(&mut self) {
        while self.session_snapshots.len() > MAX_SESSION_DIRS {
            let oldest = self
                .session_snapshots
                .iter()
                .min_by(|a, b| a.1.timestamp.cmp(&b.1.timestamp))
                .map(|(k, _)| k.clone());
            match oldest {
                Some(key) => {
                    self.session_snapshots.remove(&key);
                }
                None => break,
            }
        }
    }

    pub fn save(&self) {
        if let Some(path) = Self::session_path() {
            self.save_to(&path);
        }
    }

    // ---- Global singleton ----
    pub fn global() -> &'static RwLock<SessionState> {
        static SESSION: std::sync::LazyLock<RwLock<SessionState>> =
            std::sync::LazyLock::new(|| RwLock::new(SessionState::load()));
        &SESSION
    }

    pub fn read_global() -> std::sync::RwLockReadGuard<'static, SessionState> {
        Self::global().read().unwrap_or_else(|e| e.into_inner())
    }

    pub fn write_global() -> std::sync::RwLockWriteGuard<'static, SessionState> {
        Self::global().write().unwrap_or_else(|e| e.into_inner())
    }

    pub fn save_global() {
        Self::read_global().save();
    }

    /// Write the tab snapshot for the current editor CWD.
    pub fn save_snapshot(snapshot: SessionSnapshot) {
        let cwd = editor_cwd();
        let mut session = Self::write_global();
        session.set_snapshot(&cwd, snapshot);
        session.save();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(tab: &str, timestamp: &str) -> SessionSnapshot {
        SessionSnapshot {
            tabs: vec![TabSnapshot {
                path: PathBuf::from(tab),
                cursor_y: 0,
                cursor_byte: 0,
                scroll: 0,
                scroll_x: 0,
                modified: false,
                undo_stack: Vec::new(),
                redo_stack: Vec::new(),
            }],
            active_tab: 0,
            buffer_stack: vec![PathBuf::from(tab)],
            buffer_idx: 0,
            sidebar_open: false,
            timestamp: timestamp.to_string(),
        }
    }

    fn empty_state() -> SessionState {
        SessionState::empty()
    }

    #[test]
    fn session_file_keeps_one_bucket_per_directory() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("session.toml");
        let now = now_stamp();
        let mut state = empty_state();
        state.set_snapshot(Path::new("/proj/a"), snapshot("/proj/a/one.rs", &now));
        state.set_snapshot(Path::new("/proj/b"), snapshot("/proj/b/two.rs", &now));
        state.save_to(&path);

        let reloaded = SessionState::load_from(&path).expect("session file loads");
        let a = snapshot_for_restore(&reloaded, Path::new("/proj/a"), 24).unwrap();
        let b = snapshot_for_restore(&reloaded, Path::new("/proj/b"), 24).unwrap();
        assert_eq!(a.tabs[0].path, PathBuf::from("/proj/a/one.rs"));
        assert_eq!(b.tabs[0].path, PathBuf::from("/proj/b/two.rs"));
        assert!(snapshot_for_restore(&reloaded, Path::new("/proj/c"), 24).is_none());
    }

    #[test]
    fn snapshots_are_scoped_per_directory() {
        let mut state = empty_state();
        state.set_snapshot(
            Path::new("/proj/a"),
            snapshot("/proj/a/one.rs", "2026-01-01 10:00:00"),
        );
        state.set_snapshot(
            Path::new("/proj/b"),
            snapshot("/proj/b/two.rs", "2026-01-01 11:00:00"),
        );

        let a = state.snapshot_for(Path::new("/proj/a")).unwrap();
        let b = state.snapshot_for(Path::new("/proj/b")).unwrap();
        assert_eq!(a.tabs[0].path, PathBuf::from("/proj/a/one.rs"));
        assert_eq!(b.tabs[0].path, PathBuf::from("/proj/b/two.rs"));
        assert!(state.snapshot_for(Path::new("/proj/c")).is_none());
    }

    fn now_stamp() -> String {
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }

    #[test]
    fn restore_only_picks_the_current_directory() {
        let mut state = empty_state();
        let now = now_stamp();
        state.set_snapshot(Path::new("/proj/a"), snapshot("/proj/a/one.rs", &now));

        let restored = snapshot_for_restore(&state, Path::new("/proj/a"), 24)
            .expect("directory A has tabs to restore");
        assert_eq!(restored.tabs[0].path, PathBuf::from("/proj/a/one.rs"));
        assert!(snapshot_for_restore(&state, Path::new("/proj/b"), 24).is_none());
    }

    #[test]
    fn restore_skips_stale_and_empty_snapshots() {
        let mut state = empty_state();
        state.set_snapshot(
            Path::new("/proj/old"),
            snapshot("x.rs", "2000-01-01 00:00:00"),
        );
        assert!(snapshot_for_restore(&state, Path::new("/proj/old"), 24).is_none());

        let mut empty = snapshot("x.rs", &now_stamp());
        empty.tabs.clear();
        state.set_snapshot(Path::new("/proj/empty"), empty);
        assert!(snapshot_for_restore(&state, Path::new("/proj/empty"), 24).is_none());
    }

    #[test]
    fn directory_key_ignores_trailing_separator_and_case() {
        let mut state = empty_state();
        let dir = if cfg!(windows) {
            "C:/Work/Proj"
        } else {
            "/work/proj"
        };
        state.set_snapshot(Path::new(dir), snapshot("x.txt", "2026-01-01 10:00:00"));
        let variant = format!("{}/", dir);
        assert!(state.snapshot_for(Path::new(&variant)).is_some());
    }

    #[test]
    fn snapshot_map_round_trips_through_toml() {
        let mut state = empty_state();
        let dir = if cfg!(windows) { "C:/a.b/c" } else { "/a.b/c" };
        state.set_snapshot(Path::new(dir), snapshot("x.txt", "2026-01-01 10:00:00"));
        let toml_str = toml::to_string_pretty(&state).expect("serialize session");
        let back: SessionState = toml::from_str(&toml_str).expect("deserialize session");
        assert!(back.snapshot_for(Path::new(dir)).is_some());
    }

    #[test]
    fn legacy_session_file_still_loads() {
        // Pre-per-directory files carried a single top-level
        // `session_snapshot`; it must not break loading (it is ignored, since
        // the directory it belonged to is unknown).
        let legacy = r#"
last_directory = "C:\\proj"

[session_snapshot]
active_tab = 0
buffer_stack = []
buffer_idx = 0
sidebar_open = false
timestamp = "2026-01-01 10:00:00"

[[session_snapshot.tabs]]
path = "C:\\proj\\one.rs"
cursor_y = 0
cursor_byte = 0
scroll = 0
scroll_x = 0
modified = false
"#;
        let state: SessionState = toml::from_str(legacy).expect("legacy session parses");
        assert!(state.session_snapshots.is_empty());
        assert!(state.snapshot_for(Path::new("C:/proj")).is_none());
    }

    #[test]
    fn pruning_drops_oldest_directories() {
        let mut state = empty_state();
        for i in 0..MAX_SESSION_DIRS + 5 {
            let dir = format!("/proj/{i}");
            let ts = format!("2026-01-01 00:00:{:02}", i % 60);
            state.set_snapshot(Path::new(&dir), snapshot("x.txt", &ts));
        }
        assert_eq!(state.session_snapshots.len(), MAX_SESSION_DIRS);
    }
}
