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

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionState {
    pub last_directory: Option<PathBuf>,
    pub editor_session: Option<EditorSession>,
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
        }
    }

    pub fn save(&self) {
        if let Some(path) = Self::session_path() {
            let _ = fs::create_dir_all(path.parent().unwrap());
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

    pub fn save_global() {
        Self::global().read().unwrap().save();
    }
}
