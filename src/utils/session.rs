use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;

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
    #[serde(default)]
    pub session_snapshot: Option<SessionSnapshot>,
}

impl SessionState {
    fn session_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("ntc").join("session.toml"))
    }

    pub fn load() -> Self {
        if let Some(path) = Self::session_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(session) = toml::from_str::<SessionState>(&content) {
                        return session;
                    }
                }
            }
        }
        SessionState {
            last_directory: None,
            editor_session: None,
            nav_stack: Vec::new(),
            session_snapshot: None,
        }
    }

    pub fn save(&self) {
        if let Some(path) = Self::session_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(toml_str) = toml::to_string_pretty(self) {
                let _ = fs::write(&path, toml_str);
            }
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
}
