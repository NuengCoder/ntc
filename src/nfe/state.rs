use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub const DEFAULT_PREVIEW_WIDTH_PCT: usize = 40;

#[derive(Clone, Copy, PartialEq)]
pub enum SortMode {
    Name,
    Size,
    Date,
    Type,
}

impl SortMode {
    pub fn next(&self) -> Self {
        match self {
            SortMode::Name => SortMode::Size,
            SortMode::Size => SortMode::Date,
            SortMode::Date => SortMode::Type,
            SortMode::Type => SortMode::Name,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            SortMode::Name => "Name",
            SortMode::Size => "Size",
            SortMode::Date => "Date",
            SortMode::Type => "Type",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PanelFocus {
    List,
    Preview,
}

#[derive(Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: SystemTime,
}

impl FileEntry {
    pub fn from_path(path: &Path) -> Option<Self> {
        let name = path.file_name()?.to_string_lossy().to_string();
        let meta = std::fs::metadata(path).ok()?;
        Some(Self {
            name,
            path: path.to_path_buf(),
            is_dir: meta.is_dir(),
            size: meta.len(),
            modified: meta.modified().unwrap_or(SystemTime::UNIX_EPOCH),
        })
    }

    pub fn extension(&self) -> &str {
        self.path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
    }
}

pub struct NfeState {
    pub current_dir: PathBuf,
    pub entries: Vec<FileEntry>,
    pub dirs: Vec<FileEntry>,
    pub files: Vec<FileEntry>,
    pub selected: usize,
    pub scroll: usize,
    pub preview_scroll: usize,
    pub show_hidden: bool,
    pub sort_mode: SortMode,
    pub sort_desc: bool,
    pub filter: String,
    pub focus: PanelFocus,
    pub preview_content: Vec<String>,
    pub preview_file: Option<PathBuf>,
    pub term_w: usize,
    pub term_h: usize,
    pub running: bool,
    pub status_msg: Option<String>,
    pub status_msg_timer: std::time::Instant,

    // Help overlay
    pub help_visible: bool,

    // Go-to-line prompt
    pub line_prompt: String,
    pub line_prompt_active: bool,

    // Recursive find
    pub find_query: String,
    pub find_results: Vec<FileEntry>,
    pub find_active: bool,
}

impl NfeState {
    pub fn new(path: Option<&Path>) -> Self {
        let dir = path
            .and_then(|p| if p.exists() && p.is_dir() { Some(p.to_path_buf()) } else { None })
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
        let mut s = Self {
            current_dir: dir,
            entries: Vec::new(),
            dirs: Vec::new(),
            files: Vec::new(),
            selected: 0,
            scroll: 0,
            preview_scroll: 0,
            show_hidden: false,
            sort_mode: SortMode::Name,
            sort_desc: false,
            filter: String::new(),
            focus: PanelFocus::List,
            preview_content: Vec::new(),
            preview_file: None,
            term_w: 80,
            term_h: 24,
            running: true,
            status_msg: None,
            status_msg_timer: std::time::Instant::now(),
            help_visible: false,
            line_prompt: String::new(),
            line_prompt_active: false,
            find_query: String::new(),
            find_results: Vec::new(),
            find_active: false,
        };
        s.reload();
        s
    }

    pub fn reload(&mut self) {
        let (mut dirs, mut files): (Vec<FileEntry>, Vec<FileEntry>) = match std::fs::read_dir(&self.current_dir) {
            Ok(entries) => entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| {
                    if self.show_hidden {
                        true
                    } else {
                        !p.file_name()
                            .map(|n| n.to_string_lossy().starts_with('.'))
                            .unwrap_or(false)
                    }
                })
                .filter_map(|p| FileEntry::from_path(&p))
                .partition(|e| e.is_dir),
            Err(_) => (Vec::new(), Vec::new()),
        };

        let sort_fn = |a: &FileEntry, b: &FileEntry, mode: SortMode, desc: bool| -> Ordering {
            let cmp = match mode {
                SortMode::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortMode::Size => a.size.cmp(&b.size),
                SortMode::Date => a.modified.cmp(&b.modified),
                SortMode::Type => a.extension().cmp(&b.extension()),
            };
            if desc { cmp.reverse() } else { cmp }
        };

        let mode = self.sort_mode;
        let desc = self.sort_desc;
        dirs.sort_by(|a, b| sort_fn(a, b, mode, desc));
        files.sort_by(|a, b| sort_fn(a, b, mode, desc));

        let filter = self.filter.to_lowercase();
        let all: Vec<FileEntry> = if filter.is_empty() {
            let mut all = Vec::with_capacity(dirs.len() + files.len());
            all.extend(dirs.clone());
            all.extend(files.clone());
            all
        } else {
            let mut all: Vec<FileEntry> = dirs
                .clone()
                .into_iter()
                .chain(files.clone())
                .filter(|e| e.name.to_lowercase().contains(&filter))
                .collect();
            all.sort_by(|a, b| sort_fn(a, b, mode, desc));
            all
        };

        self.dirs = dirs;
        self.files = files;
        self.entries = all;
        self.selected = self.selected.min(self.entries.len().saturating_sub(1));
        self.scroll = self.scroll.min(self.entries.len().saturating_sub(1));
        self.preview_content.clear();
        self.preview_file = None;
    }

    pub fn current_entry(&self) -> Option<&FileEntry> {
        self.entries.get(self.selected)
    }

    pub fn selected_is_dir(&self) -> bool {
        self.current_entry().map(|e| e.is_dir).unwrap_or(false)
    }

    pub fn enter_dir(&mut self) {
        if let Some(entry) = self.current_entry() {
            if entry.is_dir {
                self.current_dir = entry.path.clone();
                self.selected = 0;
                self.scroll = 0;
                self.preview_content.clear();
                self.reload();
            }
        }
    }

    pub fn go_to_parent(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            let prev_name = self.current_dir.file_name().map(|n| n.to_string_lossy().to_string());
            self.current_dir = parent.to_path_buf();
            self.reload();
            if let Some(name) = prev_name {
                if let Some(idx) = self.entries.iter().position(|e| e.name == name) {
                    self.selected = idx;
                    self.scroll_to_selected();
                }
            }
        }
    }

    pub fn go_to_dir(&mut self, path: &Path) {
        if path.exists() && path.is_dir() {
            self.current_dir = path.to_path_buf();
            self.selected = 0;
            self.scroll = 0;
            self.reload();
        }
    }

    pub fn scroll_up(&mut self, n: usize) {
        if self.selected > 0 {
            self.selected = self.selected.saturating_sub(n);
            self.scroll_to_selected();
        }
    }

    pub fn scroll_down(&mut self, n: usize) {
        if self.selected + 1 < self.entries.len() {
            self.selected = (self.selected + n).min(self.entries.len().saturating_sub(1));
            self.scroll_to_selected();
        }
    }

    pub fn scroll_to_selected(&mut self) {
        let list_h = self.list_height();
        if self.selected < self.scroll {
            self.scroll = self.selected;
        } else if self.selected >= self.scroll + list_h {
            self.scroll = self.selected.saturating_sub(list_h) + 1;
        }
    }

    pub fn list_height(&self) -> usize {
        self.term_h.saturating_sub(4)
    }

    pub fn preview_width(&self) -> usize {
        self.term_w * DEFAULT_PREVIEW_WIDTH_PCT / 100
    }

    pub fn list_width(&self) -> usize {
        let num_w = self.line_number_width();
        self.term_w.saturating_sub(self.preview_width()).saturating_sub(num_w + 2)
    }

    pub fn line_number_width(&self) -> usize {
        3
    }

    pub fn set_status(&mut self, msg: String) {
        self.status_msg = Some(msg);
        self.status_msg_timer = std::time::Instant::now();
    }

    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.reload();
    }

    pub fn cycle_sort(&mut self) {
        self.sort_mode = self.sort_mode.next();
        self.reload();
    }

    pub fn toggle_sort_desc(&mut self) {
        self.sort_desc = !self.sort_desc;
        self.reload();
    }

    pub fn refresh(&mut self) {
        self.reload();
    }

    // ── Help overlay ──

    pub fn toggle_help(&mut self) {
        self.help_visible = !self.help_visible;
    }

    // ── Go-to-Line prompt ──

    pub fn start_line_prompt(&mut self) {
        self.line_prompt = String::new();
        self.line_prompt_active = true;
    }

    pub fn cancel_line_prompt(&mut self) {
        self.line_prompt.clear();
        self.line_prompt_active = false;
    }

    pub fn execute_line_jump(&mut self) {
        let input = self.line_prompt.trim();
        if input.is_empty() {
            self.cancel_line_prompt();
            return;
        }
        if let Some(rest) = input.strip_prefix('+') {
            if let Ok(n) = rest.parse::<usize>() {
                self.scroll_down(n);
            }
        } else if let Some(rest) = input.strip_prefix('-') {
            if let Ok(n) = rest.parse::<usize>() {
                self.scroll_up(n);
            }
        } else if let Ok(n) = input.parse::<usize>() {
            if n > 0 {
                let idx = n.saturating_sub(1).min(self.entries.len().saturating_sub(1));
                self.selected = idx;
                self.scroll_to_selected();
            }
        }
        self.cancel_line_prompt();
    }

    // ── Recursive Find ──

    pub fn start_find(&mut self) {
        self.find_query = String::new();
        self.find_results.clear();
        self.find_active = true;
    }

    pub fn cancel_find(&mut self) {
        self.find_query.clear();
        self.find_results.clear();
        self.find_active = false;
        self.reload();
    }

    pub fn update_find(&mut self) {
        if self.find_query.is_empty() {
            self.find_results.clear();
            return;
        }
        let query = self.find_query.to_lowercase();
        let show_hidden = self.show_hidden;
        let root = self.current_dir.clone();
        let mut results = Vec::new();

        if let Ok(entries) = std::fs::read_dir(&root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !show_hidden {
                    if let Some(name) = path.file_name() {
                        if name.to_string_lossy().starts_with('.') {
                            continue;
                        }
                    }
                }
                let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                if name.to_lowercase().contains(&query) {
                    if let Some(fe) = FileEntry::from_path(&path) {
                        results.push(fe);
                    }
                }
                if path.is_dir() {
                    Self::find_recursive(&path, &query, show_hidden, &mut results, 0);
                }
            }
        }

        results.truncate(200);
        self.find_results = results;
    }

    fn find_recursive(dir: &Path, query: &str, show_hidden: bool, results: &mut Vec<FileEntry>, depth: usize) {
        if depth > 20 {
            return;
        }
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !show_hidden {
                    if let Some(name) = path.file_name() {
                        if name.to_string_lossy().starts_with('.') {
                            continue;
                        }
                    }
                }
                let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                if name.to_lowercase().contains(query) && results.len() < 200 {
                    if let Some(fe) = FileEntry::from_path(&path) {
                        results.push(fe);
                    }
                }
                if path.is_dir() && results.len() < 200 {
                    Self::find_recursive(&path, query, show_hidden, results, depth + 1);
                }
            }
        }
    }
}
