use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub const DEFAULT_PREVIEW_WIDTH_PCT: usize = 40;

/// Find mode stops after this many hits so typing stays responsive.
pub const FIND_LIMIT: usize = 200;
/// And it never descends deeper than this.
pub const FIND_MAX_DEPTH: usize = 20;

/// Path as painted in the UI: the Windows verbatim (`\\?\`) prefix is
/// stripped so paths read normally, and separators are left as the OS
/// renders them elsewhere.
pub fn display_path(path: &Path) -> String {
    crate::filetype::strip_unc_prefix(path)
}

/// Ordering for one list of entries: the active sort column, then the name
/// as a tie-break so `S` always reverses both the directory and the file
/// block (directories have no size/extension of their own).
pub fn compare_entries(a: &FileEntry, b: &FileEntry, mode: SortMode, desc: bool) -> Ordering {
    let primary = match mode {
        SortMode::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        SortMode::Size => a.size.cmp(&b.size),
        SortMode::Date => a.modified.cmp(&b.modified),
        SortMode::Type => a.extension().cmp(&b.extension()),
    };
    let cmp = primary.then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    if desc {
        cmp.reverse()
    } else {
        cmp
    }
}

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

/// Where keystrokes go. The listing shown is always `entries`; each mode
/// only decides what a key does and what the bottom bar says.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NfeMode {
    /// Plain directory listing.
    Browse,
    /// Recursive find from the directory `f` was pressed in: the results
    /// (root-relative paths) replace the listing.
    Find,
    /// Live filter box (`/`): narrows the listing or the find results as
    /// the text is typed.
    Filter,
    /// Command prompt (`g` / `:`): `file`, `dir`, `line`.
    Prompt,
}

#[derive(Clone)]
pub struct FileEntry {
    pub name: String,
    /// Text painted in the list: the bare name in a directory listing, the
    /// find-root-relative path in find mode.
    pub display: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: SystemTime,
}

/// Viewer zoom stops (zoom_idx indexes here): 0.25x shrink for
/// overview, 1x fit, up to 8x inspection. `+`/`=`/`V` step in,
/// `-` steps out, `0` resets to fit.
pub const ZOOM_STOPS: [f32; 6] = [0.25, 0.5, 1.0, 2.0, 4.0, 8.0];
/// Index of the 1x fit stop.
pub const ZOOM_FIT_IDX: u8 = 2;

/// Cache key for the built viewer pixel sequences: any field change
/// re-encodes, otherwise the previous blit is left on screen.
#[derive(Debug, Clone, PartialEq)]
pub struct SixelBlitKey {
    pub path: PathBuf,
    pub zoom: u8,
    pub ox: u32,
    pub oy: u32,
    pub cols: usize,
    pub rows: usize,
    pub cw: u32,
    pub ch: u32,
}

/// Panel pixel-preview cache key: file + panel geometry + cell size.
pub type PanelPixelKey = (PathBuf, usize, usize, u32, u32);

/// Built panel pixel payload (blitted verbatim on cache miss).
#[derive(Debug, Clone)]
pub enum PanelBlit {
    Sixel(String),
    Kitty(Vec<String>),
}

impl FileEntry {
    pub fn from_path(path: &Path) -> Option<Self> {
        let name = path.file_name()?.to_string_lossy().to_string();
        let meta = std::fs::metadata(path).ok()?;
        Some(Self {
            display: name.clone(),
            name,
            path: path.to_path_buf(),
            is_dir: meta.is_dir(),
            size: meta.len(),
            modified: meta.modified().unwrap_or(SystemTime::UNIX_EPOCH),
        })
    }

    /// Paint a different row text (find results show their relative path).
    pub fn with_display(mut self, display: impl Into<String>) -> Self {
        self.display = display.into();
        self
    }

    pub fn extension(&self) -> &str {
        self.path.extension().and_then(|e| e.to_str()).unwrap_or("")
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
    /// Inline half-block image preview (when the selected entry is an
    /// image and was loaded with Enter). Cleared with `preview_content`.
    pub preview_image: Option<crate::nfe::image::ImagePreview>,
    /// Image preview density (session toggle with `v`).
    pub preview_mode: crate::nfe::image::ImageMode,

    // Fullscreen image viewer (`V` on an image, `Esc` back).
    pub viewer_active: bool,
    pub viewer_path: Option<PathBuf>,
    pub viewer_grid: Option<crate::nfe::image::ImagePreview>,
    /// Sample dimensions the grid was built from (pan/visibility math).
    pub viewer_samp: (u32, u32),
    /// Zoom factor: 1 = fit, 2..=3 = magnify (V cycles).
    /// Zoom stop index into [`ZOOM_STOPS`] (default = fit).
    pub zoom_idx: u8,
    /// Pan offsets in sample pixels (clamped to the sample).
    pub viewer_ox: u32,
    pub viewer_oy: u32,
    /// Decoded source kept for re-gridding on zoom/resize.
    #[cfg(feature = "image")]
    pub viewer_src: Option<crate::nfe::image::DecodedImage>,
    /// Cached Sixel sequence + the exact key it was built for.
    /// Re-blit only on change (encode runs ~100ms at 1MP).
    pub viewer_sixel: Option<(SixelBlitKey, String)>,
    /// Cached Kitty chunks + key (same change discipline as Sixel).
    pub viewer_kitty: Option<(SixelBlitKey, Vec<String>)>,
    /// Panel pixel payload cache ((key, blit)) and the shown key.
    /// `shown == cache key` means pixels are on screen: skip painting.
    /// Any mismatch erases the old rect first (bg repaint + Kitty
    /// delete), then blits. Text repaints can never clear pixels.
    pub panel_seq: Option<(PanelPixelKey, PanelBlit)>,
    pub panel_shown: Option<PanelPixelKey>,
    /// Instance backend override: None = auto (cached probe).
    /// Tests pin Some(Sextant) so suites never touch the terminal.
    pub backend_override: Option<crate::nfe::sixel::PixelBackend>,
    pub term_w: usize,
    pub term_h: usize,
    pub running: bool,
    pub status_msg: Option<String>,
    pub status_msg_timer: std::time::Instant,

    // Help overlay
    pub help_visible: bool,

    // Input mode + prompt / find / filter state
    pub mode: NfeMode,
    /// True while `entries` holds find results instead of `current_dir`.
    pub in_find: bool,
    pub find_root: PathBuf,
    pub find_query: String,
    pub find_results: Vec<FileEntry>,
    /// Find mode focus: `false` = typing the query, `true` = walking the
    /// results with hjkl/arrows.
    pub find_nav: bool,
    pub saved_selected: usize,
    pub saved_scroll: usize,
    /// `g` / `:` command line: buffer, caret and Tab-cycle cursor.
    pub prompt_buf: String,
    pub prompt_cursor: usize,
    pub prompt_cycle: usize,
    pub prompt_cycle_start: usize,
    pub prompt_cycle_tokens: Vec<String>,

    // Git status: entry name -> status char (M/A/D/?/U)
    pub git_status: std::collections::HashMap<String, char>,
}

impl NfeState {
    pub fn new(path: Option<&Path>) -> Self {
        let dir = path
            .and_then(|p| {
                if p.exists() && p.is_dir() {
                    Some(p.to_path_buf())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
        // Store absolute paths only: relative dirs break parent
        // navigation (`"src".parent()` is `Some("")` — an empty dir
        // with no escape) and every CWD-relative resolution after it.
        let dir = std::fs::canonicalize(&dir).unwrap_or(dir);
        let find_root = dir.clone();
        let mut s = Self {
            current_dir: dir,
            entries: Vec::new(),
            dirs: Vec::new(),
            files: Vec::new(),
            selected: 0,
            scroll: 0,
            preview_scroll: 0,
            // Dotfiles/dot-dirs are first-class entries: `.env`, `.github`,
            // `.bashrc`, … show unless the user hides them with `i`.
            show_hidden: true,
            sort_mode: SortMode::Name,
            sort_desc: false,
            filter: String::new(),
            focus: PanelFocus::List,
            preview_content: Vec::new(),
            preview_file: None,
            preview_image: None,
            preview_mode: crate::nfe::sixel::default_image_mode(&crate::nfe::sixel::snapshot_env()),
            viewer_active: false,
            viewer_path: None,
            viewer_grid: None,
            viewer_samp: (0, 0),
            zoom_idx: ZOOM_FIT_IDX,
            viewer_ox: 0,
            viewer_oy: 0,
            #[cfg(feature = "image")]
            viewer_src: None,
            viewer_sixel: None,
            viewer_kitty: None,
            panel_seq: None,
            panel_shown: None,
            backend_override: None,
            term_w: 80,
            term_h: 24,
            running: true,
            status_msg: None,
            status_msg_timer: std::time::Instant::now(),
            help_visible: false,
            mode: NfeMode::Browse,
            in_find: false,
            find_root,
            find_query: String::new(),
            find_results: Vec::new(),
            find_nav: false,
            saved_selected: 0,
            saved_scroll: 0,
            prompt_buf: String::new(),
            prompt_cursor: 0,
            prompt_cycle: usize::MAX,
            prompt_cycle_start: 0,
            prompt_cycle_tokens: Vec::new(),
            git_status: std::collections::HashMap::new(),
        };
        s.reload();
        s
    }

    pub fn reload(&mut self) {
        // Defensive recovery: never present an unreadable directory with
        // no escape — fall back to CWD/home instead.
        if !self.current_dir.is_dir() {
            let fallback = std::env::current_dir()
                .ok()
                .filter(|p| p.is_dir())
                .or_else(dirs::home_dir)
                .unwrap_or_else(|| PathBuf::from("."));
            self.current_dir = std::fs::canonicalize(&fallback).unwrap_or(fallback);
            self.selected = 0;
            self.scroll = 0;
            self.set_status("Recovered to a readable directory".to_string());
        }
        let (mut dirs, mut files): (Vec<FileEntry>, Vec<FileEntry>) =
            match std::fs::read_dir(&self.current_dir) {
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

        let (mode, desc) = (self.sort_mode, self.sort_desc);
        dirs.sort_by(|a, b| compare_entries(a, b, mode, desc));
        files.sort_by(|a, b| compare_entries(a, b, mode, desc));

        self.dirs = dirs;
        self.files = files;
        self.rebuild_entries();

        self.refresh_git_status();
    }

    /// Rebuild `entries` from the current base (directory listing, or the
    /// find results) with the live filter applied. Cheap: no directory
    /// re-read, no git probe — safe to call on every keystroke.
    pub fn rebuild_entries(&mut self) {
        let filter = self.filter.to_lowercase();
        // Rows are matched on the painted text: the plain name in a
        // listing, the relative path in find mode.
        let keep = |e: &FileEntry| filter.is_empty() || e.display.to_lowercase().contains(&filter);

        let mut entries = Vec::new();
        if self.in_find {
            entries.extend(self.find_results.iter().filter(|e| keep(e)).cloned());
        } else {
            entries.extend(self.dirs.iter().filter(|e| keep(e)).cloned());
            entries.extend(self.files.iter().filter(|e| keep(e)).cloned());
        }

        self.entries = entries;
        self.selected = self.selected.min(self.entries.len().saturating_sub(1));
        self.scroll = self.scroll.min(self.entries.len().saturating_sub(1));
        self.preview_content.clear();
        self.preview_file = None;
        self.preview_image = None;
        self.panel_seq = None;
    }

    fn refresh_git_status(&mut self) {
        self.git_status.clear();
        if !crate::config::Config::global_get_git_integration_enabled() {
            return;
        }
        let map = crate::utils::git::status_map(&self.current_dir);
        for (path, code) in map {
            if let Some(name) = path.file_name() {
                self.git_status
                    .insert(name.to_string_lossy().to_string(), code);
            }
        }
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
                let path = entry.path.clone();
                self.adopt_dir(&path);
            }
        }
    }

    /// Move to `path` as a plain directory listing: find mode is left,
    /// the filter box is emptied (a filter belongs to the listing it was
    /// typed in) and the selection goes to the top.
    fn adopt_dir(&mut self, path: &Path) {
        self.mode = NfeMode::Browse;
        self.leave_find();
        self.filter.clear();
        self.current_dir = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        self.selected = 0;
        self.scroll = 0;
        self.preview_content.clear();
        self.preview_image = None;
        self.reload();
    }

    pub fn go_to_parent(&mut self) {
        let Some(parent) = self.current_dir.parent() else {
            return;
        };
        // Never adopt an empty/relative dead end (`"src".parent()` is
        // `Some("")`): fall back to CWD, then home, with a message.
        let target = parent.to_path_buf();
        if target.as_os_str().is_empty() || !target.is_dir() {
            let fallback = std::env::current_dir()
                .ok()
                .filter(|p| p.is_dir())
                .or_else(dirs::home_dir)
                .unwrap_or_else(|| PathBuf::from("."));
            let fallback = std::fs::canonicalize(&fallback).unwrap_or(fallback);
            self.adopt_dir(&fallback);
            self.set_status("Recovered to a readable directory".to_string());
            return;
        }
        let prev_name = self
            .current_dir
            .file_name()
            .map(|n| n.to_string_lossy().to_string());
        self.adopt_dir(&target);
        if let Some(name) = prev_name {
            if let Some(idx) = self.entries.iter().position(|e| e.name == name) {
                self.selected = idx;
                self.scroll_to_selected();
            }
        }
    }

    pub fn go_to_dir(&mut self, path: &Path) {
        if path.exists() && path.is_dir() {
            self.adopt_dir(path);
        } else {
            self.set_status(format!("No such directory: {}", path.display()));
        }
    }

    /// Resolve goto-mode input to a directory: `~` expands to home,
    /// absolute paths are used as-is, everything else resolves against
    /// the current directory (never the process CWD).
    pub fn resolve_goto(&self, input: &str) -> PathBuf {
        let trimmed = input.trim();
        if let Some(rest) = trimmed.strip_prefix('~') {
            if let Some(home) = dirs::home_dir() {
                let rest = rest.trim_start_matches(['/', '\\']);
                if rest.is_empty() {
                    return home;
                }
                return home.join(rest);
            }
        }
        let p = Path::new(trimmed);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            self.current_dir.join(p)
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
        self.term_w
            .saturating_sub(self.preview_width())
            .saturating_sub(num_w + 2)
    }

    pub fn line_number_width(&self) -> usize {
        3
    }

    pub fn set_status(&mut self, msg: String) {
        self.status_msg = Some(msg);
        self.status_msg_timer = std::time::Instant::now();
    }

    pub fn toggle_preview_mode(&mut self) {
        use crate::nfe::image::ImageMode;
        self.preview_mode = match self.preview_mode {
            ImageMode::Sextant => ImageMode::Half,
            ImageMode::Half => ImageMode::Sextant,
        };
        let label = match self.preview_mode {
            ImageMode::Sextant => "sextant micro-blocks",
            ImageMode::Half => "half blocks",
        };
        // Re-grid the current image (if any) at the new density.
        if let Some(path) = self.preview_file.clone() {
            if crate::nfe::image::is_image_file(&path) {
                self.preview_scroll = 0;
                self.load_preview_content();
            }
        }
        self.set_status(format!("Image preview: {}", label));
    }

    // ── Fullscreen image viewer ──

    /// Viewport grid (cells) for the viewer: full width, all rows but
    /// the title and hint lines.
    pub fn viewer_dims(&self) -> (usize, usize) {
        (self.term_w.max(1), self.term_h.saturating_sub(2).max(1))
    }

    /// Active pixel backend for the viewer: instance override wins,
    /// else the cached auto-detection (with reason for the status line).
    pub fn viewer_backend(&self) -> crate::nfe::sixel::PixelBackend {
        match self.backend_override {
            Some(b) => b,
            None => crate::nfe::sixel::detect_cached().0,
        }
    }

    /// Open the fullscreen viewer for the selected image.
    /// Returns false (with a status message) for non-images or when
    /// decoding is unavailable / fails.
    pub fn open_viewer(&mut self) -> bool {
        let entry = match self.current_entry().cloned() {
            Some(e) if !e.is_dir && crate::nfe::image::is_image_file(&e.path) => e,
            _ => {
                self.set_status("V opens the fullscreen image view (select an image)".to_string());
                return false;
            }
        };
        #[cfg(feature = "image")]
        {
            match crate::nfe::image::decode_for_viewer(&entry.path) {
                Ok(src) => {
                    self.viewer_src = Some(src);
                    self.viewer_path = Some(entry.path);
                    self.zoom_idx = ZOOM_FIT_IDX;
                    self.viewer_ox = 0;
                    self.viewer_oy = 0;
                    self.viewer_active = true;
                    self.regrid_viewer();
                    // Self-diagnosing backend: name + detection reason so
                    // a fallback is never a mystery (or a silent bug).
                    let (backend, reason) = match self.backend_override {
                        Some(b) => (b, "override"),
                        None => crate::nfe::sixel::detect_cached(),
                    };
                    let label = match backend {
                        crate::nfe::sixel::PixelBackend::Kitty => "kitty",
                        crate::nfe::sixel::PixelBackend::Sixel => "sixel",
                        crate::nfe::sixel::PixelBackend::Sextant => "sextants",
                    };
                    // Legacy console gets an actionable hint, not just a
                    // backend name: text sextants work, real pixels need the
                    // OS viewer (`o`).
                    if backend == crate::nfe::sixel::PixelBackend::Sextant
                        && crate::nfe::sixel::is_legacy_console(&crate::nfe::sixel::snapshot_env())
                    {
                        self.set_status(format!(
                            "Image view: {} ({}) — legacy console: o opens the image in the OS viewer",
                            label, reason
                        ));
                    } else {
                        self.set_status(format!("Image view: {} ({})", label, reason));
                    }
                    true
                }
                Err(e) => {
                    self.set_status(format!("Cannot open image: {}", e));
                    false
                }
            }
        }
        #[cfg(not(feature = "image"))]
        {
            let _ = entry;
            self.set_status("Image view unavailable in this build".to_string());
            false
        }
    }

    pub fn close_viewer(&mut self) {
        self.viewer_active = false;
        self.viewer_path = None;
        self.viewer_grid = None;
        self.viewer_samp = (0, 0);
        self.viewer_sixel = None;
        // NOTE: viewer_kitty is intentionally kept: the next normal
        // render consumes it to emit the Kitty delete (Kitty placements
        // persist as terminal objects — text repaint cannot erase them).
        #[cfg(feature = "image")]
        {
            self.viewer_src = None;
        }
    }

    /// Current zoom factor from the stop table.
    pub fn zoom_factor(&self) -> f32 {
        ZOOM_STOPS[self.zoom_idx.min(ZOOM_STOPS.len() as u8 - 1) as usize]
    }

    /// Human zoom label (`0.5x`, `2x`).
    pub fn zoom_label(&self) -> String {
        format!("{}x", self.zoom_factor())
    }

    /// Step zoom in (`+1`), out (`-1`); pan resets each step.
    /// Clamped at both ends with a status note.
    pub fn step_viewer_zoom(&mut self, dir: i8) {
        let max = ZOOM_STOPS.len() as u8 - 1;
        let next = (self.zoom_idx as i8 + dir).clamp(0, max as i8) as u8;
        if next == self.zoom_idx {
            self.set_status(if dir > 0 {
                "Viewer zoom: max (8x)".to_string()
            } else {
                "Viewer zoom: min (0.25x)".to_string()
            });
            return;
        }
        self.zoom_idx = next;
        self.viewer_ox = 0;
        self.viewer_oy = 0;
        self.regrid_viewer();
        self.set_status(format!("Viewer zoom: {}", self.zoom_label()));
    }

    // ── External open + WT relaunch ──

    /// OS command that opens `path` in the default viewer, split for
    /// testability (never spawned here): (program, args).
    pub fn external_open_cmd(path: &Path) -> (String, Vec<String>) {
        #[cfg(windows)]
        {
            // `start` needs a dummy title arg when the path is quoted.
            (
                "cmd".to_string(),
                vec![
                    "/C".to_string(),
                    "start".to_string(),
                    "".to_string(),
                    path.to_string_lossy().to_string(),
                ],
            )
        }
        #[cfg(target_os = "macos")]
        {
            ("open".to_string(), vec![path.to_string_lossy().to_string()])
        }
        #[cfg(all(not(windows), not(target_os = "macos")))]
        {
            (
                "xdg-open".to_string(),
                vec![path.to_string_lossy().to_string()],
            )
        }
    }

    /// Open the selected image in the OS default viewer (real pixels
    /// on every terminal, including conhost). Detached spawn.
    pub fn open_external(&mut self) -> bool {
        let path = match self.current_entry().cloned() {
            Some(e) if !e.is_dir && crate::nfe::image::is_image_file(&e.path) => e.path,
            _ => {
                self.set_status("o opens the selected image externally".to_string());
                return false;
            }
        };
        let (prog, args) = Self::external_open_cmd(&path);
        match std::process::Command::new(&prog).args(&args).spawn() {
            Ok(_) => {
                self.set_status(format!("Opened externally: {}", path.display()));
                true
            }
            Err(e) => {
                self.set_status(format!("Cannot open externally ({}): {}", prog, e));
                false
            }
        }
    }

    /// Reset zoom to fit.
    pub fn reset_viewer_zoom(&mut self) {
        self.zoom_idx = ZOOM_FIT_IDX;
        self.viewer_ox = 0;
        self.viewer_oy = 0;
        self.regrid_viewer();
        self.set_status(format!("Viewer zoom: {}", self.zoom_label()));
    }

    /// Pan by cells (clamped to the sample). When nothing can move
    /// (fitted image), says so — arrows at 1x are correct no-ops,
    /// not broken keys.
    pub fn pan_viewer(&mut self, dx: i32, dy: i32) {
        let (cols, rows) = self.viewer_dims();
        let (sx, sy) = match self.preview_mode {
            crate::nfe::image::ImageMode::Sextant => (2, 3),
            crate::nfe::image::ImageMode::Half => (1, 2),
        };
        let max_ox = self
            .viewer_samp
            .0
            .saturating_sub((cols as u32).saturating_mul(sx));
        let max_oy = self
            .viewer_samp
            .1
            .saturating_sub((rows as u32).saturating_mul(sy));
        let (nox, noy) = (
            (self.viewer_ox as i32 + dx * sx as i32).clamp(0, max_ox as i32) as u32,
            (self.viewer_oy as i32 + dy * sy as i32).clamp(0, max_oy as i32) as u32,
        );
        if (nox, noy) == (self.viewer_ox, self.viewer_oy) && (dx, dy) != (0, 0) {
            self.set_status("Nothing to pan — zoom in to move (+)".to_string());
            return;
        }
        self.viewer_ox = nox;
        self.viewer_oy = noy;
    }

    /// Fit scale for the current viewport (aspect-correct in sample
    /// pixels: 2x3 per cell for sextants, 1x2 for half blocks).
    fn viewer_fit_scale(&self, orig_w: u32, orig_h: u32) -> f32 {
        let (cols, rows) = self.viewer_dims();
        let (sx, sy) = match self.preview_mode {
            crate::nfe::image::ImageMode::Sextant => (2.0, 3.0),
            crate::nfe::image::ImageMode::Half => (1.0, 2.0),
        };
        let vw = (cols as f32 * sx).max(1.0);
        let vh = (rows as f32 * sy).max(1.0);
        (vw / orig_w.max(1) as f32).min(vh / orig_h.max(1) as f32)
    }

    /// Crop the cached decode to the visible orig rect (from the
    /// text-sample pan model) and fit it into `tw × th` device pixels.
    /// Shared by the Sixel and Kitty pixel paths. Returns the fitted
    /// image plus its pixel size, or `None` when the rect is empty.
    #[cfg(feature = "image")]
    fn viewer_fitted_crop(
        &self,
        cols: usize,
        rows: usize,
        tw: u32,
        th: u32,
    ) -> Option<(image::RgbImage, u32, u32)> {
        use crate::nfe::image::ImageMode;
        let src = self.viewer_src.as_ref()?;
        // Visible fraction from the text-sample pan model.
        let (sx, sy) = match self.preview_mode {
            ImageMode::Sextant => (2.0f32, 3.0f32),
            ImageMode::Half => (1.0f32, 2.0f32),
        };
        let (sw, sh) = (
            self.viewer_samp.0.max(1) as f32,
            self.viewer_samp.1.max(1) as f32,
        );
        let fx0 = (self.viewer_ox as f32 / sw).clamp(0.0, 1.0);
        let fy0 = (self.viewer_oy as f32 / sh).clamp(0.0, 1.0);
        let fx1 = ((self.viewer_ox as f32 + cols as f32 * sx) / sw).clamp(0.0, 1.0);
        let fy1 = ((self.viewer_oy as f32 + rows as f32 * sy) / sh).clamp(0.0, 1.0);
        // Crop the cached decode to the visible orig rect.
        let (ow, oh) = (src.orig_w.max(1), src.orig_h.max(1));
        let cx = (fx0 * ow as f32) as u32;
        let cy = (fy0 * oh as f32) as u32;
        let cw_px = ((fx1 - fx0) * ow as f32) as u32;
        let ch_px = ((fy1 - fy0) * oh as f32) as u32;
        if cw_px == 0 || ch_px == 0 {
            return None;
        }
        let cx = cx.min(ow - 1);
        let cy = cy.min(oh - 1);
        let cw_px = cw_px.min(ow - cx).max(1);
        let ch_px = ch_px.min(oh - cy).max(1);
        let cropped = image::imageops::crop_imm(&src.pixels, cx, cy, cw_px, ch_px).to_image();
        // Fit the crop into the target (aspect may differ after pan).
        let s = (tw as f32 / cw_px as f32).min(th as f32 / ch_px as f32);
        let dw = ((cw_px as f32 * s) as u32).clamp(1, tw);
        let dh = ((ch_px as f32 * s) as u32).clamp(1, th);
        let small =
            image::imageops::resize(&cropped, dw, dh, image::imageops::FilterType::Lanczos3);
        Some((small, dw, dh))
    }

    /// Target device pixels for a `cols × rows` cell viewport,
    /// capped to the encode budget (returns scaled tw/th).
    fn viewport_px(cols: usize, rows: usize, cw: u32, ch: u32) -> (u32, u32) {
        let mut tw = (cols as u32).saturating_mul(cw).max(1);
        let mut th = (rows as u32).saturating_mul(ch).max(1);
        let budget: u64 = 4_000_000;
        let total = tw as u64 * th as u64;
        if total > budget {
            let s = (budget as f64 / total as f64).sqrt();
            tw = ((tw as f64 * s) as u32).max(1);
            th = ((th as f64 * s) as u32).max(1);
        }
        (tw, th)
    }

    /// Build (or reuse the cached) Sixel sequence for the viewer at
    /// the current zoom/pan/size. Returns the sequence plus the cell
    /// padding that centers it, or `None` when nothing changed since
    /// the last blit (leave the screen alone) or when unavailable.
    /// `bg` fills register 0 (letterbox). Cell size is a parameter so
    /// tests never query the terminal.
    #[cfg(feature = "image")]
    pub fn sixel_seq(
        &mut self,
        cols: usize,
        rows: usize,
        cw: u32,
        ch: u32,
        bg: (u8, u8, u8),
    ) -> Option<(String, crate::nfe::image::BlitGeom)> {
        let path = self.viewer_path.clone()?;
        let _ = self.viewer_src.as_ref()?;
        let key = SixelBlitKey {
            path,
            zoom: self.zoom_idx,
            ox: self.viewer_ox,
            oy: self.viewer_oy,
            cols,
            rows,
            cw,
            ch,
        };
        if let Some((k, _)) = &self.viewer_sixel {
            if *k == key {
                return None;
            }
        }
        let (tw, th) = Self::viewport_px(cols, rows, cw, ch);
        let (small, dw, dh) = self.viewer_fitted_crop(cols, rows, tw, th)?;
        let px: Vec<(u8, u8, u8)> = small.pixels().map(|p| (p[0], p[1], p[2])).collect();
        // Quantize a thumbnail (palettes from 65K px are visually
        // identical to full-res ones, but median-cut sorts far less).
        let tscale = (256.0 / dw.max(dh) as f32).min(1.0);
        let tw = ((dw as f32 * tscale) as u32).max(1);
        let th = ((dh as f32 * tscale) as u32).max(1);
        let thumb = image::imageops::resize(&small, tw, th, image::imageops::FilterType::Lanczos3);
        let tpx: Vec<(u8, u8, u8)> = thumb.pixels().map(|p| (p[0], p[1], p[2])).collect();
        let pal = crate::nfe::sixel::median_cut(&tpx, 255);
        let idx = crate::nfe::sixel::map_to_palette(&px, &pal);
        let seq = crate::nfe::sixel::encode_sixel(&idx, dw as usize, dh as usize, &pal, bg);
        // Center within the viewport (cells).
        let pad_x = cols.saturating_sub((dw / cw.max(1)) as usize) / 2;
        let pad_y = rows.saturating_sub((dh / ch.max(1)) as usize) / 2;
        let geom = crate::nfe::image::BlitGeom {
            pad_x,
            pad_y,
            w: dw.div_ceil(cw.max(1)) as usize,
            h: dh.div_ceil(ch.max(1)) as usize,
        };
        self.viewer_sixel = Some((key, seq.clone()));
        Some((seq, geom))
    }

    /// Build (or reuse the cached) Kitty chunk sequences for the viewer.
    /// Returns the chunks plus centering padding, or `None` when
    /// unchanged since the last blit. PNG payload at viewport device
    /// pixels; `c`/`r` force the cell fit on display.
    #[cfg(feature = "image")]
    pub fn kitty_seq(
        &mut self,
        cols: usize,
        rows: usize,
        cw: u32,
        ch: u32,
    ) -> Option<(Vec<String>, crate::nfe::image::BlitGeom)> {
        use std::io::Cursor;
        let path = self.viewer_path.clone()?;
        let _ = self.viewer_src.as_ref()?;
        let key = SixelBlitKey {
            path,
            zoom: self.zoom_idx,
            ox: self.viewer_ox,
            oy: self.viewer_oy,
            cols,
            rows,
            cw,
            ch,
        };
        if let Some((k, _)) = &self.viewer_kitty {
            if *k == key {
                return None;
            }
        }
        let (tw, th) = Self::viewport_px(cols, rows, cw, ch);
        let (small, dw, dh) = self.viewer_fitted_crop(cols, rows, tw, th)?;
        let mut png = Vec::new();
        image::DynamicImage::ImageRgb8(small)
            .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
            .ok()?;
        let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png);
        let chunks = crate::nfe::kitty::transmit_chunks(&b64, cols, rows);
        let pad_x = cols.saturating_sub((dw / cw.max(1)) as usize) / 2;
        let pad_y = rows.saturating_sub((dh / ch.max(1)) as usize) / 2;
        let geom = crate::nfe::image::BlitGeom {
            pad_x,
            pad_y,
            w: dw.div_ceil(cw.max(1)) as usize,
            h: dh.div_ceil(ch.max(1)) as usize,
        };
        self.viewer_kitty = Some((key, chunks.clone()));
        Some((chunks, geom))
    }

    /// Fit-decode `path` into at most `w × h` cells at `cw × ch`
    /// device pixels (aspect kept). Shared by both panel pixel paths.
    /// Returns the fitted image plus its pixel size.
    #[cfg(feature = "image")]
    fn panel_fitted(
        &self,
        path: &Path,
        w: usize,
        h: usize,
        cw: u32,
        ch: u32,
    ) -> Option<(image::RgbImage, u32, u32)> {
        let tw = (w as u32).saturating_mul(cw).max(1);
        let th = (h as u32).saturating_mul(ch).max(1);
        let img = image::open(path).ok()?;
        let (ow, oh) = {
            use image::GenericImageView;
            img.dimensions()
        };
        if ow == 0 || oh == 0 {
            return None;
        }
        let s = (tw as f32 / ow as f32).min(th as f32 / oh as f32);
        let dw = ((ow as f32 * s) as u32).clamp(1, tw);
        let dh = ((oh as f32 * s) as u32).clamp(1, th);
        let small = image::imageops::resize(
            &img.to_rgb8(),
            dw,
            dh,
            image::imageops::FilterType::Lanczos3,
        );
        Some((small, dw, dh))
    }

    /// Build the panel pixel payload for `path` at `w × h` cells and
    /// cache it under its key. Sextant backend builds nothing (the
    /// text grid path handles it). Overwrites any previous entry.
    #[cfg(feature = "image")]
    pub fn build_panel_seq(
        &mut self,
        path: &Path,
        w: usize,
        h: usize,
        cw: u32,
        ch: u32,
        bg: (u8, u8, u8),
    ) {
        use crate::nfe::sixel::PixelBackend;
        let backend = self.viewer_backend();
        if backend == PixelBackend::Sextant {
            return;
        }
        let key: PanelPixelKey = (path.to_path_buf(), w, h, cw, ch);
        let blit = match backend {
            PixelBackend::Kitty => {
                let (small, _, _) = match self.panel_fitted(path, w, h, cw, ch) {
                    Some(v) => v,
                    None => return,
                };
                let png = match crate::nfe::kitty::encode_png(&small) {
                    Ok(b) => b,
                    Err(_) => return,
                };
                let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png);
                Some(PanelBlit::Kitty(crate::nfe::kitty::transmit_chunks(
                    &b64, w, h,
                )))
            }
            PixelBackend::Sixel => {
                let (small, dw, dh) = match self.panel_fitted(path, w, h, cw, ch) {
                    Some(v) => v,
                    None => return,
                };
                let px: Vec<(u8, u8, u8)> = small.pixels().map(|p| (p[0], p[1], p[2])).collect();
                let tscale = (256.0 / dw.max(dh) as f32).min(1.0);
                let tw = ((dw as f32 * tscale) as u32).max(1);
                let th = ((dh as f32 * tscale) as u32).max(1);
                let thumb =
                    image::imageops::resize(&small, tw, th, image::imageops::FilterType::Lanczos3);
                let tpx: Vec<(u8, u8, u8)> = thumb.pixels().map(|p| (p[0], p[1], p[2])).collect();
                let pal = crate::nfe::sixel::median_cut(&tpx, 255);
                let idx = crate::nfe::sixel::map_to_palette(&px, &pal);
                Some(PanelBlit::Sixel(crate::nfe::sixel::encode_sixel(
                    &idx,
                    dw as usize,
                    dh as usize,
                    &pal,
                    bg,
                )))
            }
            PixelBackend::Sextant => None,
        };
        if let Some(blit) = blit {
            self.panel_seq = Some((key, blit));
        }
    }

    /// Rebuild the viewer grid from the cached decode (zoom/resize).
    /// The sample + pan math always runs (the Sixel path derives its
    /// crop from it); the text grid is skipped when Sixel is active.
    pub fn regrid_viewer(&mut self) {
        #[cfg(feature = "image")]
        {
            use crate::nfe::image::ImageMode;
            use crate::nfe::sixel::PixelBackend;
            let Some(src) = self.viewer_src.clone() else {
                self.viewer_grid = None;
                return;
            };
            let (cols, rows) = self.viewer_dims();
            let (sx, sy) = match self.preview_mode {
                ImageMode::Sextant => (2u32, 3u32),
                ImageMode::Half => (1u32, 2u32),
            };
            let scale = self.viewer_fit_scale(src.orig_w, src.orig_h) * self.zoom_factor();
            let samp_w = ((src.orig_w as f32 * scale) as u32).clamp(1, 4096);
            let samp_h = ((src.orig_h as f32 * scale) as u32).clamp(1, 4096);
            self.viewer_samp = (samp_w, samp_h);
            // Clamp pan into the (possibly resized) sample.
            let max_ox = samp_w.saturating_sub((cols as u32).saturating_mul(sx));
            let max_oy = samp_h.saturating_sub((rows as u32).saturating_mul(sy));
            self.viewer_ox = self.viewer_ox.min(max_ox);
            self.viewer_oy = self.viewer_oy.min(max_oy);
            if self.viewer_backend() == PixelBackend::Sixel {
                self.viewer_grid = None;
                return;
            }
            self.viewer_grid = Some(match self.preview_mode {
                ImageMode::Sextant => crate::nfe::image::grid_sextants_from(
                    &src,
                    samp_w,
                    samp_h,
                    self.viewer_ox,
                    self.viewer_oy,
                    cols,
                    rows,
                ),
                ImageMode::Half => crate::nfe::image::grid_half_from(
                    &src,
                    samp_w,
                    samp_h,
                    self.viewer_ox,
                    self.viewer_oy,
                    cols,
                    rows,
                ),
            });
        }
        #[cfg(not(feature = "image"))]
        {
            self.viewer_grid = None;
        }
    }

    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.reload();
    }

    /// `s`: next sort column. Both blocks (directories, files) and the
    /// find results are re-sorted so the header always tells the truth.
    pub fn cycle_sort(&mut self) {
        self.sort_mode = self.sort_mode.next();
        self.apply_sort();
        self.set_status(format!("Sort: {}", self.sort_label()));
    }

    /// `S`: flip the sort direction. Directories reverse too.
    pub fn toggle_sort_desc(&mut self) {
        self.sort_desc = !self.sort_desc;
        self.apply_sort();
        self.set_status(format!("Sort: {}", self.sort_label()));
    }

    /// `Name^` / `Size v` — the header label for the active sort.
    pub fn sort_label(&self) -> String {
        let arrow = if self.sort_desc { "v" } else { "^" };
        format!("{} {}", self.sort_mode.label(), arrow)
    }

    pub fn refresh(&mut self) {
        self.reload();
    }

    // ── Help overlay ──

    pub fn toggle_help(&mut self) {
        self.help_visible = !self.help_visible;
    }

    // ── Filter box (`/`) ──

    /// Open the filter box. Whatever text is already there stays live, so
    /// `/` refines an active filter instead of starting over.
    pub fn start_filter(&mut self) {
        self.mode = NfeMode::Filter;
    }

    /// Leave the filter box but keep the filter applied.
    pub fn finish_filter(&mut self) {
        if self.mode == NfeMode::Filter {
            self.mode = self.listing_mode();
        }
    }

    /// Cancel the filter box and show the whole listing again.
    pub fn cancel_filter(&mut self) {
        self.filter.clear();
        self.selected = 0;
        self.scroll = 0;
        self.rebuild_entries();
        self.finish_filter();
    }

    /// The mode the listing belongs to (find results keep find mode).
    pub fn listing_mode(&self) -> NfeMode {
        if self.in_find {
            NfeMode::Find
        } else {
            NfeMode::Browse
        }
    }

    /// Append to the filter and re-filter at once — no Enter needed.
    pub fn filter_push(&mut self, c: char) {
        self.filter.push(c);
        self.refilter();
    }

    pub fn filter_backspace(&mut self) {
        self.filter.pop();
        self.refilter();
    }

    fn refilter(&mut self) {
        self.selected = 0;
        self.scroll = 0;
        self.rebuild_entries();
    }

    // ── Recursive find mode (`f`) ──

    /// Enter find mode: the search starts at the directory `f` was pressed
    /// in, and its results (shown as relative paths) replace the listing.
    pub fn start_find(&mut self) {
        if self.in_find {
            return;
        }
        self.saved_selected = self.selected;
        self.saved_scroll = self.scroll;
        self.in_find = true;
        self.mode = NfeMode::Find;
        self.find_nav = false;
        self.find_root = self.current_dir.clone();
        self.find_query.clear();
        self.find_results.clear();
        self.filter.clear();
        self.entries.clear();
        self.selected = 0;
        self.scroll = 0;
        self.preview_content.clear();
        self.preview_file = None;
        self.preview_image = None;
        self.panel_seq = None;
    }

    /// Leave find mode and restore the directory listing it replaced.
    pub fn cancel_find(&mut self) {
        self.mode = NfeMode::Browse;
        self.leave_find();
    }

    /// Drop the find result set and rebuild the directory listing.
    fn leave_find(&mut self) {
        if !self.in_find {
            return;
        }
        self.in_find = false;
        self.find_query.clear();
        self.find_results.clear();
        self.find_nav = false;
        self.selected = self.saved_selected;
        self.scroll = self.saved_scroll;
        self.rebuild_entries();
    }

    /// Re-run the search for the current query. Bounded to [`FIND_LIMIT`]
    /// hits and [`FIND_MAX_DEPTH`] levels so typing stays responsive.
    pub fn update_find(&mut self) {
        self.find_results.clear();
        self.entries.clear();
        self.selected = 0;
        self.scroll = 0;
        self.preview_content.clear();
        self.preview_file = None;
        self.preview_image = None;
        if self.find_query.is_empty() {
            return;
        }
        let query = self.find_query.to_lowercase();
        let root = self.find_root.clone();
        let mut results = Vec::new();
        Self::find_recursive(&root, &root, &query, self.show_hidden, &mut results, 0);
        self.find_results = results;
        self.rebuild_entries();
    }

    /// Move the find selection by `delta` rows (PageUp/PageDown use the
    /// page height).
    pub fn find_move(&mut self, delta: isize) {
        if self.entries.is_empty() {
            return;
        }
        let last = self.entries.len() - 1;
        let next = (self.selected as isize + delta).clamp(0, last as isize) as usize;
        self.selected = next;
        self.scroll_to_selected();
    }

    /// Navigate to the selected find result. Directories are entered;
    /// files get their parent listed with the file selected. Either way
    /// find mode ends, so the explorer is back to a normal listing.
    pub fn accept_find(&mut self) {
        let Some(entry) = self.current_entry().cloned() else {
            self.set_status("Nothing to open — type to search first".to_string());
            return;
        };
        if entry.is_dir {
            self.go_to_dir(&entry.path);
            return;
        }
        let Some(parent) = entry.path.parent().map(|p| p.to_path_buf()) else {
            self.set_status(format!("Cannot open: {}", entry.path.display()));
            return;
        };
        let name = entry.name.clone();
        self.go_to_dir(&parent);
        if let Some(idx) = self.entries.iter().position(|e| e.name == name) {
            self.selected = idx;
            self.scroll_to_selected();
        }
        self.set_status(format!("Found: {}", display_path(&entry.path)));
    }

    fn find_recursive(
        root: &Path,
        dir: &Path,
        query: &str,
        show_hidden: bool,
        results: &mut Vec<FileEntry>,
        depth: usize,
    ) {
        if depth > FIND_MAX_DEPTH || results.len() >= FIND_LIMIT {
            return;
        }
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            if results.len() >= FIND_LIMIT {
                return;
            }
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if !show_hidden && name.starts_with('.') {
                continue;
            }
            if name.to_lowercase().contains(query) {
                if let Some(fe) = FileEntry::from_path(&path) {
                    // The row shows where the hit lives, relative to the
                    // directory find mode was started from.
                    let display = path
                        .strip_prefix(root)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .replace('\\', "/");
                    results.push(fe.with_display(display));
                }
            }
            if path.is_dir() {
                Self::find_recursive(root, &path, query, show_hidden, results, depth + 1);
            }
        }
    }

    // ── Line jump (`line` command, `l` shortcut) ──

    /// Move the selection to row `N` (1-based) or by `+N` / `-N`.
    /// Returns false when the text is not a line target.
    pub fn jump_line(&mut self, input: &str) -> bool {
        let input = input.trim();
        if let Some(rest) = input.strip_prefix('+') {
            match rest.parse::<usize>() {
                Ok(n) => {
                    self.scroll_down(n);
                    true
                }
                Err(_) => false,
            }
        } else if let Some(rest) = input.strip_prefix('-') {
            match rest.parse::<usize>() {
                Ok(n) => {
                    self.scroll_up(n);
                    true
                }
                Err(_) => false,
            }
        } else {
            match input.parse::<usize>() {
                Ok(n) if n > 0 => {
                    let idx = n
                        .saturating_sub(1)
                        .min(self.entries.len().saturating_sub(1));
                    self.selected = idx;
                    self.scroll_to_selected();
                    true
                }
                _ => false,
            }
        }
    }

    // ── Sorting ──

    /// Sort the active base list (directories + files, or find results).
    fn sort_base(&mut self) {
        let (mode, desc) = (self.sort_mode, self.sort_desc);
        self.dirs.sort_by(|a, b| compare_entries(a, b, mode, desc));
        self.files.sort_by(|a, b| compare_entries(a, b, mode, desc));
        self.find_results
            .sort_by(|a, b| compare_entries(a, b, mode, desc));
    }

    /// Re-sort and rebuild the visible list, keeping the cursor on the
    /// same entry when it is still listed.
    fn apply_sort(&mut self) {
        let keep = self.current_entry().map(|e| e.path.clone());
        self.sort_base();
        self.rebuild_entries();
        if let Some(path) = keep {
            if let Some(idx) = self.entries.iter().position(|e| e.path == path) {
                self.selected = idx;
            }
        }
        self.scroll_to_selected();
    }

    /// First entry under `current_dir` matching `name`: an exact
    /// (case-insensitive) name wins anywhere in the tree, otherwise the
    /// first substring hit. `want_dir` restricts the search to dirs
    /// (`Some(true)`), files (`Some(false)`) or anything (`None`).
    pub(crate) fn find_match(&self, name: &str, want_dir: Option<bool>) -> Option<PathBuf> {
        let needle = name.trim().to_lowercase();
        if needle.is_empty() {
            return None;
        }
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((self.current_dir.clone(), 0usize));
        let mut partial: Option<PathBuf> = None;
        while let Some((dir, depth)) = queue.pop_front() {
            if depth > FIND_MAX_DEPTH {
                continue;
            }
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                if want_dir.map(|w| w == is_dir).unwrap_or(true) {
                    let candidate = entry.file_name().to_string_lossy().to_lowercase();
                    if candidate == needle {
                        return Some(path);
                    }
                    if partial.is_none() && candidate.contains(&needle) {
                        partial = Some(path.clone());
                    }
                }
                if is_dir {
                    queue.push_back((path, depth + 1));
                }
            }
        }
        partial
    }

    /// List `path`'s parent and put the cursor on `path`.
    pub(crate) fn reveal_file(&mut self, path: &Path) {
        let Some(parent) = path.parent().map(|p| p.to_path_buf()) else {
            self.set_status(format!("Cannot open: {}", path.display()));
            return;
        };
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        self.go_to_dir(&parent);
        if let Some(idx) = self.entries.iter().position(|e| e.name == name) {
            self.selected = idx;
            self.scroll_to_selected();
        }
        self.set_status(format!("Opened: {}", display_path(path)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_support as tsupport;

    fn tree(tag: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("ntc_nfe_{}_{}", tag, std::process::id()));
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::create_dir_all(root.join("docs")).unwrap();
        root
    }

    #[test]
    fn new_starts_absolute() {
        let root = tree("abs");
        let s = NfeState::new(Some(&root));
        assert!(s.current_dir.is_absolute(), "{:?}", s.current_dir);
        assert!(s.entries.iter().any(|e| e.name == "src"));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn dotfiles_and_dot_dirs_are_visible_by_default() {
        let root = tree("dotfiles");
        std::fs::write(root.join(".env"), "A=1").unwrap();
        std::fs::create_dir_all(root.join(".github")).unwrap();
        let s = NfeState::new(Some(&root));
        assert!(s.show_hidden);
        assert!(s.entries.iter().any(|e| e.name == ".env"));
        assert!(
            s.entries.iter().any(|e| e.name == ".github" && e.is_dir),
            "{:?}",
            s.entries.iter().map(|e| &e.name).collect::<Vec<_>>()
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn goto_relative_resolves_against_current_dir() {
        // The exact reported repro: `g` → `src` [enter] from the repo
        // root must land absolute, and `b` must return to the true
        // parent with entries — never to an empty dir.
        let root = tree("goto");
        let mut s = NfeState::new(Some(&root));
        let target = s.resolve_goto("src");
        assert_eq!(target, s.current_dir.join("src"));
        s.go_to_dir(&target);
        assert!(s.current_dir.is_absolute(), "{:?}", s.current_dir);
        assert_eq!(s.current_dir.file_name().unwrap(), "src");
        s.go_to_parent();
        assert_eq!(
            s.current_dir,
            std::fs::canonicalize(&root).unwrap(),
            "{:?}",
            s.current_dir
        );
        assert!(s.entries.iter().any(|e| e.name == "src"));
        assert!(s.entries.iter().any(|e| e.name == "docs"));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn goto_invalid_keeps_dir_and_warns() {
        let root = tree("bad");
        let mut s = NfeState::new(Some(&root));
        let before = s.current_dir.clone();
        s.go_to_dir(&s.resolve_goto("no_such_dir_xyz"));
        assert_eq!(s.current_dir, before);
        assert!(s.status_msg.is_some());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn parent_of_relative_path_recovers() {
        // Simulates a legacy relative current_dir: `b` must recover to
        // a readable directory instead of sticking at `""`.
        // Locks process globals: the recovery reads process CWD, which a
        // parallel CWD-moving test could otherwise yank away on CI.
        let _g = tsupport::lock_globals();
        let mut s = NfeState::new(None);
        s.current_dir = PathBuf::from("src");
        s.go_to_parent();
        assert!(!s.current_dir.as_os_str().is_empty(), "{:?}", s.current_dir);
        assert!(s.current_dir.is_dir(), "{:?}", s.current_dir);
        assert!(!s.entries.is_empty() || s.status_msg.is_some());
    }

    #[test]
    fn resolve_goto_absolute_and_nested() {
        let root = tree("abs2");
        let s = NfeState::new(Some(&root));
        let abs = std::fs::canonicalize(root.join("docs")).unwrap();
        assert_eq!(s.resolve_goto(&abs.to_string_lossy()), abs);
        assert_eq!(
            s.resolve_goto("src/../docs"),
            s.current_dir.join("src/../docs")
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn viewer_ignores_non_images() {
        let root = tree("viewtxt");
        std::fs::write(root.join("note.txt"), "hello").unwrap();
        let mut s = NfeState::new(Some(&root));
        s.selected = s.entries.iter().position(|e| e.name == "note.txt").unwrap();
        assert!(!s.open_viewer());
        assert!(!s.viewer_active);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[cfg(feature = "image")]
    fn write_gradient(path: &Path, w: u32, h: u32) {
        let mut img = image::RgbImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                img.put_pixel(
                    x,
                    y,
                    image::Rgb([(x * 255 / w.max(1)) as u8, (y * 255 / h.max(1)) as u8, 128]),
                );
            }
        }
        img.save(path).unwrap();
    }

    #[cfg(feature = "image")]
    #[test]
    fn panel_preview_uses_sextants_by_default() {
        let root = tree("viewgrad");
        write_gradient(&root.join("g.png"), 200, 200);
        let mut s = NfeState::new(Some(&root));
        // Pin density: the constructor default follows the host env
        // (conhost gets Half), but this test needs sextants.
        s.preview_mode = crate::nfe::image::ImageMode::Sextant;
        s.term_w = 120;
        s.term_h = 30;
        s.selected = s.entries.iter().position(|e| e.name == "g.png").unwrap();
        s.load_preview_content();
        let img = s.preview_image.as_ref().expect("image preview loads");
        // A gradient must produce real sextant glyphs, not just ▀/space.
        assert!(img
            .rows
            .iter()
            .flatten()
            .any(|c| { c.ch != '▀' && c.ch != ' ' && c.ch != '█' }));
        // Toggle to half blocks: only ▀/space remain.
        s.toggle_preview_mode();
        assert_eq!(s.preview_mode, crate::nfe::image::ImageMode::Half);
        let img = s.preview_image.as_ref().expect("half preview loads");
        assert!(img
            .rows
            .iter()
            .flatten()
            .all(|c| c.ch == '▀' || c.ch == ' '));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[cfg(feature = "image")]
    #[test]
    fn panel_pixel_builds_per_backend() {
        use crate::nfe::sixel::PixelBackend;
        let root = tree("panelpx");
        write_gradient(&root.join("g.png"), 120, 90);
        let mut s = NfeState::new(Some(&root));
        let path = root.join("g.png");
        // Kitty backend builds chunk payloads.
        s.backend_override = Some(PixelBackend::Kitty);
        s.build_panel_seq(&path, 40, 20, 8, 16, (0, 0, 0));
        let (key, blit) = s.panel_seq.clone().expect("kitty panel payload");
        assert_eq!(key.0, path);
        match blit {
            PanelBlit::Kitty(chunks) => {
                assert!(!chunks.is_empty());
                assert!(chunks[0].starts_with("\x1b_Ga=T,f=100,q=2,"));
            }
            PanelBlit::Sixel(_) => panic!("expected kitty chunks"),
        }
        // Sixel backend builds a DCS sequence for the same file.
        s.backend_override = Some(PixelBackend::Sixel);
        s.build_panel_seq(&path, 40, 20, 8, 16, (0, 0, 0));
        match &s.panel_seq.as_ref().expect("sixel panel payload").1 {
            PanelBlit::Sixel(seq) => {
                assert!(seq.starts_with("\x1bPq"));
                assert!(seq.ends_with("\x1b\\"));
            }
            PanelBlit::Kitty(_) => panic!("expected sixel"),
        }
        // Sextant backend builds nothing (text grid path owns it).
        s.backend_override = Some(PixelBackend::Sextant);
        s.panel_seq = None;
        s.build_panel_seq(&path, 40, 20, 8, 16, (0, 0, 0));
        assert!(s.panel_seq.is_none());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[cfg(feature = "image")]
    #[test]
    fn kitty_seq_caches_like_sixel() {
        let root = tree("viewkit");
        write_gradient(&root.join("g.png"), 120, 90);
        let mut s = NfeState::new(Some(&root));
        s.backend_override = Some(crate::nfe::sixel::PixelBackend::Kitty);
        s.term_w = 100;
        s.term_h = 40;
        s.selected = s.entries.iter().position(|e| e.name == "g.png").unwrap();
        assert!(s.open_viewer());
        let first = s.kitty_seq(100, 38, 8, 16);
        assert!(first.is_some());
        // Same key -> None (leave the screen alone).
        assert!(s.kitty_seq(100, 38, 8, 16).is_none());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[cfg(feature = "image")]
    #[test]
    fn sixel_seq_caches_and_rebuilds_on_change() {
        use crate::nfe::image::ImageMode;
        let root = tree("viewsix");
        write_gradient(&root.join("g.png"), 120, 90);
        let mut s = NfeState::new(Some(&root));
        // Pin the backend: suites must never probe the terminal.
        // (sixel_seq itself is backend-independent and tested directly.)
        s.backend_override = Some(crate::nfe::sixel::PixelBackend::Sextant);
        s.term_w = 100;
        s.term_h = 40;
        s.selected = s.entries.iter().position(|e| e.name == "g.png").unwrap();
        assert!(s.open_viewer());
        // First build returns the sequence with centering geometry.
        let first = s.sixel_seq(100, 38, 8, 16, (0, 0, 0));
        assert!(first.is_some());
        let (seq, _) = first.unwrap();
        assert!(seq.starts_with("\x1bPq"));
        assert!(seq.ends_with("\x1b\\"));
        // Same key -> None (leave the screen alone).
        assert!(s.sixel_seq(100, 38, 8, 16, (0, 0, 0)).is_none());
        // Zoom change -> rebuild with a different payload.
        s.step_viewer_zoom(1);
        let zoomed = s.sixel_seq(100, 38, 8, 16, (0, 0, 0));
        assert!(zoomed.is_some());
        assert_ne!(zoomed.unwrap().0, seq);
        // Density toggle does not affect sixel pixels.
        s.preview_mode = ImageMode::Half;
        let _ = std::fs::remove_dir_all(&root);
    }

    #[cfg(feature = "image")]
    #[test]
    fn zoom_stops_step_and_clamp() {
        let root = tree("zoomstops");
        write_gradient(&root.join("g.png"), 120, 90);
        let mut s = NfeState::new(Some(&root));
        s.backend_override = Some(crate::nfe::sixel::PixelBackend::Sextant);
        s.selected = s.entries.iter().position(|e| e.name == "g.png").unwrap();
        assert!(s.open_viewer());
        assert_eq!(s.zoom_label(), "1x");
        s.step_viewer_zoom(-1);
        assert_eq!(s.zoom_label(), "0.5x");
        s.step_viewer_zoom(-1);
        assert_eq!(s.zoom_label(), "0.25x");
        // Clamped at the bottom with a status note.
        s.step_viewer_zoom(-1);
        assert_eq!(s.zoom_label(), "0.25x");
        assert!(s.status_msg.as_deref().unwrap_or("").contains("min"));
        s.reset_viewer_zoom();
        assert_eq!(s.zoom_label(), "1x");
        // Pan at fit explains itself instead of silently nothing.
        s.term_w = 200;
        s.term_h = 60;
        s.regrid_viewer();
        s.pan_viewer(-4, 0);
        assert!(s.status_msg.as_deref().unwrap_or("").contains("zoom in"));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[cfg(feature = "image")]
    #[test]
    fn viewer_open_zoom_pan_close() {
        let root = tree("viewfull");
        write_gradient(&root.join("g.png"), 300, 200);
        let mut s = NfeState::new(Some(&root));
        // Pin text backend: suites must never probe the terminal.
        s.backend_override = Some(crate::nfe::sixel::PixelBackend::Sextant);
        s.term_w = 200;
        s.term_h = 60;
        s.selected = s.entries.iter().position(|e| e.name == "g.png").unwrap();
        assert!(s.open_viewer());
        assert!(s.viewer_active);
        let (cols, rows) = s.viewer_dims();
        assert_eq!((cols, rows), (200, 58));
        let grid = s.viewer_grid.as_ref().expect("viewer grid built");
        assert_eq!(grid.cells_high(), rows);
        assert_eq!(grid.cells_wide(), cols);
        // Zoom steps through stops and rebuilds the grid.
        s.step_viewer_zoom(1);
        assert_eq!(s.zoom_factor(), 2.0);
        s.step_viewer_zoom(1);
        assert_eq!(s.zoom_factor(), 4.0);
        s.step_viewer_zoom(-1);
        assert_eq!(s.zoom_factor(), 2.0);
        s.reset_viewer_zoom();
        assert_eq!(s.zoom_factor(), 1.0);
        assert_eq!(s.zoom_label(), "1x");
        // Pan clamps inside the sample (never underflows).
        s.step_viewer_zoom(1); // 2x: likely pannable on 300x200
        s.pan_viewer(10_000, 10_000);
        let (sw, sh) = s.viewer_samp;
        assert!(s.viewer_ox <= sw && s.viewer_oy <= sh);
        s.close_viewer();
        assert!(!s.viewer_active);
        assert!(s.viewer_grid.is_none());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn external_open_cmd_per_os() {
        let (prog, args) = NfeState::external_open_cmd(Path::new("a.png"));
        #[cfg(windows)]
        {
            assert_eq!(prog, "cmd");
            assert_eq!(args[0], "/C");
            assert_eq!(args[1], "start");
            assert!(args[3].ends_with("a.png"));
        }
        #[cfg(target_os = "macos")]
        {
            assert_eq!(prog, "open");
            assert!(args[0].ends_with("a.png"));
        }
        #[cfg(all(not(windows), not(target_os = "macos")))]
        {
            assert_eq!(prog, "xdg-open");
            assert!(args[0].ends_with("a.png"));
        }
    }

    #[test]
    fn zoom_out_reaches_quarter() {
        let root = tree("zoomout");
        let mut s = NfeState::new(Some(&root));
        s.step_viewer_zoom(-1);
        assert_eq!(s.zoom_factor(), 0.5);
        assert_eq!(s.zoom_label(), "0.5x");
        s.step_viewer_zoom(-1);
        assert_eq!(s.zoom_factor(), 0.25);
        s.reset_viewer_zoom();
        assert_eq!(s.zoom_factor(), 1.0);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn filter_is_live_and_cleared_on_directory_change() {
        let root = tree("filtercd");
        std::fs::create_dir_all(root.join("sub")).unwrap();
        std::fs::write(root.join("sub").join("inner.txt"), "x").unwrap();
        std::fs::write(root.join("keep.txt"), "x").unwrap();
        let mut s = NfeState::new(Some(&root));
        s.start_filter();
        s.filter_push('s');
        assert_eq!(s.mode, NfeMode::Filter);
        assert_eq!(s.filter, "s");
        // Live: no Enter was needed for `entries` to shrink.
        assert!(!s.entries.is_empty());
        assert!(s
            .entries
            .iter()
            .all(|e| e.name.to_lowercase().contains('s')));
        // Entering a directory clears the filter of the listing it came from.
        let idx = s.entries.iter().position(|e| e.name == "sub").unwrap();
        s.selected = idx;
        s.enter_dir();
        assert!(s.filter.is_empty());
        assert_eq!(s.mode, NfeMode::Browse);
        assert!(s.entries.iter().any(|e| e.name == "inner.txt"));
        // Backspace then Esc clears the filter box itself.
        s.start_filter();
        s.filter_push('i');
        assert_eq!(s.filter, "i");
        s.cancel_filter();
        assert!(s.filter.is_empty());
        assert_eq!(s.mode, NfeMode::Browse);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn sort_direction_reverses_directories_too() {
        let root = tree("sortdir");
        std::fs::create_dir_all(root.join("adir")).unwrap();
        std::fs::create_dir_all(root.join("zdir")).unwrap();
        std::fs::write(root.join("a_file.txt"), "x").unwrap();
        std::fs::write(root.join("z_file.txt"), "x").unwrap();
        let mut s = NfeState::new(Some(&root));
        let dirs: Vec<String> = s.dirs.iter().map(|e| e.name.clone()).collect();
        let files: Vec<String> = s.files.iter().map(|e| e.name.clone()).collect();
        assert_eq!(dirs, vec!["adir", "docs", "src", "zdir"]);
        assert_eq!(files, vec!["a_file.txt", "z_file.txt"]);
        assert_eq!(s.sort_label(), "Name ^");

        s.toggle_sort_desc();
        assert_eq!(s.sort_label(), "Name v");
        let rev_dirs: Vec<String> = s.dirs.iter().map(|e| e.name.clone()).collect();
        let rev_files: Vec<String> = s.files.iter().map(|e| e.name.clone()).collect();
        assert_eq!(rev_dirs, dirs.iter().rev().cloned().collect::<Vec<_>>());
        assert_eq!(rev_files, files.iter().rev().cloned().collect::<Vec<_>>());
        // The visible list keeps the dirs block first, both reversed.
        assert_eq!(s.entries.first().map(|e| e.name.as_str()), Some("zdir"));

        // Size/date/type give directories no useful key: the name
        // tie-break keeps the direction visible anyway.
        s.toggle_sort_desc();
        s.sort_mode = SortMode::Size;
        s.toggle_sort_desc();
        let size_dirs: Vec<String> = s.dirs.iter().map(|e| e.name.clone()).collect();
        assert_eq!(size_dirs, dirs.iter().rev().cloned().collect::<Vec<_>>());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn find_mode_lists_root_relative_paths_and_navigates() {
        let root = tree("findmode");
        std::fs::create_dir_all(root.join("src").join("deep")).unwrap();
        std::fs::write(root.join("main.rs"), "x").unwrap();
        std::fs::write(root.join("src").join("main.rs"), "x").unwrap();
        std::fs::write(root.join("src").join("deep").join("main.rs"), "x").unwrap();
        let mut s = NfeState::new(Some(&root));
        s.term_w = 100;
        s.term_h = 30;
        let listing: Vec<String> = s.entries.iter().map(|e| e.name.clone()).collect();

        s.start_find();
        assert!(s.in_find);
        assert_eq!(s.mode, NfeMode::Find);
        assert_eq!(s.find_root, std::fs::canonicalize(&root).unwrap());
        assert!(s.entries.is_empty(), "find mode starts with no results");

        s.find_query.push_str("main.rs");
        s.update_find();
        let displays: Vec<String> = s.entries.iter().map(|e| e.display.clone()).collect();
        assert!(displays.contains(&"main.rs".to_string()), "{displays:?}");
        assert!(
            displays.contains(&"src/main.rs".to_string()),
            "{displays:?}"
        );
        assert!(
            displays.contains(&"src/deep/main.rs".to_string()),
            "{displays:?}"
        );
        // Rows are find-root relative: never absolute, always `/`.
        assert!(s.entries.iter().all(|e| !e.display.starts_with('/')));
        assert!(s.entries.iter().all(|e| !e.display.contains('\\')));

        // Enter on a file lists its directory with the file selected and
        // leaves find mode.
        s.selected = s
            .entries
            .iter()
            .position(|e| e.display == "src/main.rs")
            .unwrap();
        s.accept_find();
        assert!(!s.in_find);
        assert_eq!(s.mode, NfeMode::Browse);
        assert_eq!(s.current_dir.file_name().unwrap(), "src");
        assert_eq!(s.current_entry().map(|e| e.name.as_str()), Some("main.rs"));

        // Esc restores the original listing (same directory, same rows).
        let mut s = NfeState::new(Some(&root));
        s.start_find();
        s.find_query.push_str("main");
        s.update_find();
        s.find_move(3);
        s.cancel_find();
        assert!(!s.in_find);
        assert_eq!(s.mode, NfeMode::Browse);
        assert_eq!(
            s.entries.iter().map(|e| e.name.clone()).collect::<Vec<_>>(),
            listing
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn find_results_can_be_filtered_live() {
        let root = tree("findfilter");
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::write(root.join("main.rs"), "x").unwrap();
        std::fs::write(root.join("src").join("main.rs"), "x").unwrap();
        let mut s = NfeState::new(Some(&root));
        s.start_find();
        s.find_query.push_str("main.rs");
        s.update_find();
        assert_eq!(s.entries.len(), 2);
        // `/` narrows the hits by path as they are typed.
        s.start_filter();
        s.filter_push('s');
        s.filter_push('r');
        assert_eq!(s.entries.len(), 1);
        assert_eq!(s.entries[0].display, "src/main.rs");
        s.finish_filter();
        assert_eq!(s.mode, NfeMode::Find);
        assert_eq!(s.entries.len(), 1);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn command_prompt_hints_verbs_and_arguments() {
        let root = tree("prompt");
        std::fs::create_dir_all(root.join("target")).unwrap();
        std::fs::write(root.join("hello.txt"), "x").unwrap();
        let mut s = NfeState::new(Some(&root));

        s.start_prompt("");
        assert_eq!(s.mode, NfeMode::Prompt);
        s.prompt_push('f');
        // The first argument hints the verb list.
        assert_eq!(s.prompt_ghost(), Some("ile ".to_string()));
        s.prompt_accept();
        assert_eq!(s.prompt_buf, "file ");
        // Then `file ` hints the files of the current directory.
        assert!(s.prompt_ghost().is_some());
        s.prompt_push('h');
        assert_eq!(s.prompt_ghost(), Some("ello.txt".to_string()));
        s.prompt_accept();
        assert_eq!(s.prompt_buf, "file hello.txt");
        // A complete token has no resting hint; Tab can still cycle verbs.
        assert_eq!(s.prompt_ghost(), None);
        s.run_prompt();
        assert_eq!(s.mode, NfeMode::Browse);
        assert_eq!(
            s.current_entry().map(|e| e.name.as_str()),
            Some("hello.txt")
        );

        // `dir ` hints directories only (a trailing `/` continues the path).
        let mut s = NfeState::new(Some(&root));
        s.start_prompt("dir ");
        let ghost = s.prompt_ghost().expect("dir hint");
        assert!(
            ghost == "docs/" || ghost == "src/" || ghost == "target/",
            "{ghost}"
        );
        s.prompt_push('t');
        s.prompt_accept();
        assert_eq!(s.prompt_buf, "dir target/");
        s.run_prompt();
        assert_eq!(s.mode, NfeMode::Browse);
        assert_eq!(s.current_dir.file_name().unwrap(), "target");

        // `line ` never hints a number.
        let mut s = NfeState::new(Some(&root));
        s.start_prompt("line ");
        assert_eq!(s.prompt_ghost(), None);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn command_prompt_runs_file_dir_and_line() {
        let root = tree("promptrun");
        std::fs::create_dir_all(root.join("sub")).unwrap();
        std::fs::write(root.join("sub").join("deep.txt"), "x").unwrap();
        std::fs::write(root.join("top.txt"), "x").unwrap();
        let mut s = NfeState::new(Some(&root));
        assert!(s.entries.len() >= 1);

        // `file` with a bare name finds the file in a subdirectory.
        s.start_prompt("file deep.txt");
        s.run_prompt();
        assert_eq!(s.current_dir.file_name().unwrap(), "sub");
        assert_eq!(s.current_entry().map(|e| e.name.as_str()), Some("deep.txt"));

        // `dir` accepts nested paths and teleport-free names.
        s.start_prompt("dir ..");
        s.run_prompt();
        assert_eq!(s.current_dir, std::fs::canonicalize(&root).unwrap());

        // `line` moves the selection.
        let len = s.entries.len();
        assert!(len >= 3, "{len}");
        s.start_prompt("line 2");
        s.run_prompt();
        assert_eq!(s.selected, 1);
        s.start_prompt("line +1");
        s.run_prompt();
        assert_eq!(s.selected, 2);
        s.start_prompt("line -1");
        s.run_prompt();
        assert_eq!(s.selected, 1);

        // Errors keep the prompt open with the text intact.
        s.start_prompt("line nope");
        s.run_prompt();
        assert_eq!(s.mode, NfeMode::Prompt);
        s.start_prompt("bogus 1");
        s.run_prompt();
        assert_eq!(s.mode, NfeMode::Prompt);
        s.start_prompt("file");
        s.run_prompt();
        assert_eq!(s.mode, NfeMode::Prompt);
        s.start_prompt("file ghost.txt");
        s.run_prompt();
        assert_eq!(s.mode, NfeMode::Prompt);
        // Esc leaves the prompt, not the app.
        s.cancel_prompt();
        assert_eq!(s.mode, NfeMode::Browse);
        assert!(s.running);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn command_prompt_tab_cycles_and_right_accepts() {
        let root = tree("prompttab");
        let mut s = NfeState::new(Some(&root));
        s.start_prompt("d");
        // Only `dir ` matches `d`.
        assert_eq!(s.prompt_ghost(), Some("ir ".to_string()));
        s.prompt_accept_or_right();
        assert_eq!(s.prompt_buf, "dir ");
        // Tab walks the candidates without touching the buffer.
        s.start_prompt("li");
        let ghost = s.prompt_ghost();
        assert!(ghost.is_some());
        let before = s.prompt_buf.clone();
        s.prompt_tab();
        assert_eq!(s.prompt_buf, before);
        s.prompt_accept_or_end();
        assert!(s.prompt_buf.starts_with("li"));
        assert!(s.prompt_buf.len() > before.len() || s.prompt_buf == "line ");
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Frame bytes with ANSI escapes removed (visual text only).
    fn strip_ansi(text: &str) -> String {
        let mut out = String::new();
        let mut chars = text.chars().peekable();
        while let Some(c) = chars.next() {
            if c != '\x1b' {
                out.push(c);
                continue;
            }
            if chars.peek() == Some(&'[') {
                chars.next();
                for n in chars.by_ref() {
                    if n.is_ascii_alphabetic() || n == '~' {
                        break;
                    }
                }
            }
        }
        out
    }

    #[test]
    fn render_paints_find_command_and_filter_lines() {
        let root = tree("renderlines");
        std::fs::write(root.join("main.rs"), "fn main() {}\n").unwrap();
        let mut s = NfeState::new(Some(&root));
        s.term_w = 100;
        s.term_h = 30;
        s.backend_override = Some(crate::nfe::sixel::PixelBackend::Sextant);

        // Find mode: the listing is replaced by root-relative hits and the
        // bottom bar carries the query, the count and the key hints.
        let mut out: Vec<u8> = Vec::new();
        s.start_find();
        s.find_query.push_str("main");
        s.update_find();
        s.render(&mut out).unwrap();
        let text = String::from_utf8_lossy(&out);
        assert!(text.contains("find: "), "{text}");
        assert!(text.contains("main"), "{text}");
        assert!(text.contains("hit(s)"), "{text}");
        assert!(text.contains("main.rs"), "{text}");

        // Command prompt: label + buffer, then the dimmed hint.
        let mut out: Vec<u8> = Vec::new();
        s.cancel_find();
        s.start_prompt("fi");
        s.render(&mut out).unwrap();
        let text = String::from_utf8_lossy(&out);
        assert!(text.contains("g: fi"), "{text}");
        // The caret block splits the ghost in the raw stream, so check the
        // visible text (escapes removed).
        assert!(strip_ansi(&text).contains("g: file"), "{text}");

        // Filter box: live text plus the match counter.
        let mut out: Vec<u8> = Vec::new();
        s.cancel_prompt();
        s.start_filter();
        s.filter_push('m');
        s.render(&mut out).unwrap();
        let text = String::from_utf8_lossy(&out);
        assert!(text.contains("filter: m"), "{text}");
        assert!(text.contains("match(es)"), "{text}");
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Extract the bottom row of a rendered frame (raw escapes included) so
    /// the caret can be asserted without a terminal.
    fn last_row(bytes: &[u8]) -> String {
        let text = String::from_utf8_lossy(bytes).to_string();
        // Every `MoveTo(0, ROW)` on the status row starts a repaint of it.
        let row = 29usize; // term_h - 1 for the fixtures below
        let marker = format!("\x1b[{};1H", row + 1);
        match text.rfind(&marker) {
            Some(i) => text[i..].to_string(),
            None => String::new(),
        }
    }

    #[test]
    fn command_prompt_paints_one_block_caret() {
        let root = tree("caret");
        std::fs::write(root.join("alpha.txt"), "x").unwrap();
        std::fs::write(root.join("beta.txt"), "x").unwrap();
        let mut s = NfeState::new(Some(&root));
        s.term_w = 100;
        s.term_h = 30;
        s.backend_override = Some(crate::nfe::sixel::PixelBackend::Sextant);

        // Caret at end of line, ghost showing: the first ghost cell is inverted.
        let mut out: Vec<u8> = Vec::new();
        s.start_prompt("fi");
        s.render(&mut out).unwrap();
        let row = last_row(&out);
        assert_eq!(row.matches("\x1b[7m").count(), 1, "{row:?}");
        let after = row.split("\x1b[7m").nth(1).unwrap_or("");
        assert!(
            after.starts_with('l'),
            "caret should sit on the ghost: {row:?}"
        );

        // Mid-buffer caret inverts the character under it, not the first one.
        let mut out: Vec<u8> = Vec::new();
        s.start_prompt("file alpha.txt");
        s.prompt_cursor = 5; // just after the word "file"
        s.render(&mut out).unwrap();
        let row = last_row(&out);
        assert_eq!(row.matches("\x1b[7m").count(), 1, "{row:?}");
        let after = row.split("\x1b[7m").nth(1).unwrap_or("");
        assert!(after.starts_with('a'), "caret should sit on 'a': {row:?}");

        // Caret at end of a real line (no ghost left): an inverted blank.
        let mut out: Vec<u8> = Vec::new();
        s.start_prompt("line");
        s.prompt_cursor = 4;
        s.render(&mut out).unwrap();
        let row = last_row(&out);
        assert_eq!(row.matches("\x1b[7m").count(), 1, "{row:?}");
        let after = row.split("\x1b[7m").nth(1).unwrap_or("");
        assert!(
            after.starts_with(' '),
            "caret should be an inverted blank: {row:?}"
        );

        // Result focus in find mode is a list, so no caret is painted.
        let mut out: Vec<u8> = Vec::new();
        s.cancel_prompt();
        s.start_find();
        s.find_query.push_str("alpha");
        s.update_find();
        s.find_nav = true;
        s.render(&mut out).unwrap();
        let row = last_row(&out);
        assert!(row.contains("find: alpha"), "{row:?}");
        assert!(
            !row.contains("\x1b[7m"),
            "no caret in result focus: {row:?}"
        );
        let _ = std::fs::remove_dir_all(&root);
    }
}
