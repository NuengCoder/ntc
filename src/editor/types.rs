use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime};

use anyhow::{Context, Result};
use std::sync::atomic::AtomicUsize;

use crate::editor::editor_config::EditorConfig;
use crate::session::EditorSession;
use crate::session::SessionSnapshot;
use crate::session::TabSnapshot;
use crate::syntax::SyntaxHighlighter;

/// Set by cmd_ne when opening a file via `file:line`.
/// Editor::init_tabs reads and clears it to jump to the target line.
pub(crate) static GOTO_LINE: AtomicUsize = AtomicUsize::new(0);
/// Set by `gs -i` Enter: transiently highlight the target line until the
/// next key press or mouse click (mouse scroll does NOT clear it).
pub static GOTO_HIGHLIGHT: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
/// Pattern that produced the `gs -i` jump (reserved for substring emphasis).
static GOTO_PATTERN: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

/// Store the pattern for the pending `gs -i` jump highlight.
pub fn set_goto_pattern(pat: &str) {
    if let Ok(mut g) = GOTO_PATTERN.lock() {
        g.clear();
        g.push_str(pat);
    }
}

/// Take (and clear) the pending `gs -i` jump pattern.
pub(crate) fn take_goto_pattern() -> String {
    GOTO_PATTERN
        .lock()
        .map(|mut g| std::mem::take(&mut *g))
        .unwrap_or_default()
}

// ── constants ────────────────────────────────────────────────────────────────

// (Sidebar mouse drag retired with the nfe keyboard mode; width is fixed.)

/// Default width of the left sidebar (file explorer) in characters.
pub(crate) const DEFAULT_SIDEBAR_WIDTH: usize = 30;

/// Message sent from the reader thread to the run output handler.
#[derive(Clone, Debug)]
pub(crate) enum RunMessage {
    /// A line of process output.
    Line(String),
    /// Signal that the process has finished and all output has been read.
    Done,
}

// ── editor mode ──────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum Mode {
    Normal,
    Insert,
    InsertMulti,
    Search,
    Visual,
    Command,
    Help,
    FileFinder,
    Gosc,
    Jump,
    Query,
    Encoding,
    GitDiff,
    Gmode,
    Nfe,
}

/// Which panel has keyboard focus.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Focus {
    Editor,
    Run,
    Terminal,
}

// ── snapshot for undo/redo ────────────────────────────────────────────────────

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Snapshot {
    pub(crate) lines: Vec<String>,
    pub(crate) cursor_y: usize,
    pub(crate) cursor_byte: usize,
    pub(crate) extra_cursors: Vec<CursorPos>,
}

// ── cursor ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct CursorPos {
    pub(crate) y: usize,
    pub(crate) byte: usize,
    pub(crate) anchor: Option<(usize, usize)>,
}

// ── cursor state (sub-struct for Editor) ─────────────────────────────────────

#[derive(Clone)]
pub(crate) struct CursorState {
    pub(crate) y: usize,
    pub(crate) byte: usize,
    pub(crate) selection_anchor: Option<(usize, usize)>,
    pub(crate) extra_cursors: Vec<CursorPos>,
    pub(crate) last_added_cursor_idx: Option<usize>,
    pub(crate) prev_y: usize,
    pub(crate) prev_byte: usize,
}

impl CursorState {
    pub(crate) fn new() -> Self {
        Self {
            y: 0,
            byte: 0,
            selection_anchor: None,
            extra_cursors: Vec::new(),
            last_added_cursor_idx: None,
            prev_y: 0,
            prev_byte: 0,
        }
    }
}

// ── search state (sub-struct for Editor) ─────────────────────────────────────

#[derive(Clone)]
pub(crate) struct SearchState {
    pub(crate) query: String,
    pub(crate) matches: Vec<(usize, usize, usize)>,
    pub(crate) match_idx: usize,
    /// Set to true when lines change, triggering a rebuild on next search
    pub(crate) dirty: bool,
    /// Cached query that matches were built from
    pub(crate) cached_query: String,
}

impl SearchState {
    pub(crate) fn new() -> Self {
        Self {
            query: String::new(),
            matches: Vec::new(),
            match_idx: 0,
            dirty: false,
            cached_query: String::new(),
        }
    }

    /// Invalidate the search cache when the document changes.
    pub(crate) fn mark_dirty(&mut self) {
        self.dirty = true;
    }
}

// ── scroll state (sub-struct for Editor) ─────────────────────────────────────

#[derive(Clone)]
pub(crate) struct ScrollState {
    /// Vertical scroll offset (was `scroll`)
    pub(crate) offset: usize,
    /// Horizontal scroll offset (was `scroll_x`)
    pub(crate) x: usize,
    /// Help screen scroll offset (was `help_scroll`)
    pub(crate) help: usize,
    /// Run panel scroll offset (was `run_scroll`)
    pub(crate) run: usize,
    /// Terminal panel horizontal scroll offset
    pub(crate) terminal_scroll_x: usize,
    pub(crate) dragging_vscroll: bool,
    pub(crate) dragging_hscroll: bool,
}

impl ScrollState {
    pub(crate) fn new() -> Self {
        Self {
            offset: 0,
            x: 0,
            help: 0,
            run: 0,
            terminal_scroll_x: 0,
            dragging_vscroll: false,
            dragging_hscroll: false,
        }
    }
}

// ── file tree node ──────────────────────────────────────────────────────────

#[derive(Clone)]
pub(crate) struct FileNode {
    pub(crate) name: String,
    pub(crate) path: std::path::PathBuf,
    pub(crate) is_dir: bool,
    pub(crate) expanded: bool,
    pub(crate) depth: usize,
}

/// Left-side file explorer sidebar state.
pub(crate) struct Sidebar {
    pub(crate) open: bool,
    pub(crate) scroll: usize,
    pub(crate) selected: usize,
    pub(crate) nodes: Vec<FileNode>,
    pub(crate) root: std::path::PathBuf,
    pub(crate) dragging_border: bool,
    // File watcher for live reload (Windows only)
    #[cfg(windows)]
    pub(crate) watcher_rx: Option<std::sync::mpsc::Receiver<()>>,
    #[cfg(windows)]
    pub(crate) _watcher: Option<notify::RecommendedWatcher>,
    // Directory listing cache: path -> (timestamp, dirs, files)
    pub(crate) dir_cache: HashMap<PathBuf, (Instant, Vec<FileNode>, Vec<FileNode>)>,
    // Cache TTL in seconds (default: 5)
    pub(crate) cache_ttl_secs: u64,
}

impl Sidebar {
    pub(crate) fn new() -> Self {
        let root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let mut s = Self {
            open: false,
            scroll: 0,
            selected: 0,
            nodes: Vec::new(),
            root,
            dragging_border: false,
            #[cfg(windows)]
            watcher_rx: None,
            #[cfg(windows)]
            _watcher: None,
            dir_cache: HashMap::new(),
            cache_ttl_secs: 5,
        };
        s.rebuild_tree();
        #[cfg(windows)]
        s.start_watcher();
        s
    }

    /// Start watching the root directory for changes (Windows only).
    #[cfg(windows)]
    fn start_watcher(&mut self) {
        use notify::{RecursiveMode, Watcher};
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();
        let path = self.root.clone();

        let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(_event) = res {
                let _ = tx.send(());
            }
        })
        .ok();

        if let Some(ref mut w) = watcher {
            let _ = w.watch(&path, RecursiveMode::NonRecursive);
            self.watcher_rx = Some(rx);
            self._watcher = watcher.take();
        }
    }

    /// Invalidate all cached directory listings.
    #[allow(dead_code)]
    pub(crate) fn invalidate_cache(&mut self) {
        self.dir_cache.clear();
    }

    /// Check for directory changes and rebuild tree if needed.
    /// Returns true if the tree was rebuilt.
    #[cfg(windows)]
    pub(crate) fn check_changed(&mut self) -> bool {
        if let Some(rx) = &self.watcher_rx {
            if rx.try_recv().is_ok() {
                self.invalidate_cache();
                self.rebuild_tree();
                return true;
            }
        }
        false
    }

    #[cfg(not(windows))]
    pub(crate) fn check_changed(&mut self) -> bool {
        false
    }

    pub(crate) fn rebuild_tree(&mut self) {
        self.root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        // Save expanded directory paths before clearing
        let expanded: std::collections::HashSet<std::path::PathBuf> = self
            .nodes
            .iter()
            .filter(|n| n.is_dir && n.expanded)
            .map(|n| n.path.clone())
            .collect();
        self.nodes.clear();
        // ".." entry for parent directory
        if let Some(parent) = self.root.parent() {
            self.nodes.push(FileNode {
                name: "..".into(),
                path: parent.to_path_buf(),
                is_dir: true,
                expanded: false,
                depth: 0,
            });
        }
        Self::add_children(
            &mut self.nodes,
            &self.root,
            0,
            false,
            &mut self.dir_cache,
            self.cache_ttl_secs,
        );
        // Restore expanded state — properly populate children of expanded dirs
        let mut i = 0;
        while i < self.nodes.len() {
            if self.nodes[i].is_dir
                && !self.nodes[i].expanded
                && expanded.contains(&self.nodes[i].path)
            {
                let mut children = Vec::new();
                Self::add_children(
                    &mut children,
                    &self.nodes[i].path,
                    self.nodes[i].depth + 1,
                    true,
                    &mut self.dir_cache,
                    self.cache_ttl_secs,
                );
                self.nodes[i].expanded = true;
                self.nodes.splice(i + 1..i + 1, children);
            }
            i += 1;
        }
        self.selected = self.selected.min(self.nodes.len().saturating_sub(1));
    }

    fn get_cached_children(
        cache: &mut HashMap<PathBuf, (Instant, Vec<FileNode>, Vec<FileNode>)>,
        dir: &std::path::Path,
        cache_ttl: u64,
    ) -> Option<(Vec<FileNode>, Vec<FileNode>)> {
        if let Some((timestamp, cached_dirs, cached_files)) = cache.get(dir) {
            if timestamp.elapsed().as_secs() < cache_ttl {
                return Some((cached_dirs.clone(), cached_files.clone()));
            }
        }
        None
    }

    fn read_children(dir: &std::path::Path, depth: usize) -> (Vec<FileNode>, Vec<FileNode>) {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return (Vec::new(), Vec::new()),
        };
        let mut dirs: Vec<FileNode> = Vec::new();
        let mut files: Vec<FileNode> = Vec::new();
        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            if let Ok(ft) = entry.file_type() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with('.') {
                    continue;
                }
                let path = entry.path();
                if ft.is_dir() {
                    dirs.push(FileNode {
                        name,
                        path,
                        is_dir: true,
                        expanded: false,
                        depth,
                    });
                } else {
                    files.push(FileNode {
                        name,
                        path,
                        is_dir: false,
                        expanded: false,
                        depth,
                    });
                }
            }
        }
        dirs.sort_by_key(|a| a.name.to_lowercase());
        files.sort_by_key(|a| a.name.to_lowercase());
        (dirs, files)
    }

    fn add_children(
        out: &mut Vec<FileNode>,
        dir: &std::path::Path,
        depth: usize,
        expanded: bool,
        cache: &mut HashMap<PathBuf, (Instant, Vec<FileNode>, Vec<FileNode>)>,
        cache_ttl: u64,
    ) {
        // Limit recursion depth to avoid stack overflow on deeply nested dirs
        const MAX_DEPTH: usize = 64;
        if depth >= MAX_DEPTH {
            return;
        }
        let (dirs, files) = if let Some(cached) = Self::get_cached_children(cache, dir, cache_ttl) {
            cached
        } else {
            let (dirs, files) = Self::read_children(dir, depth);
            cache.insert(
                dir.to_path_buf(),
                (Instant::now(), dirs.clone(), files.clone()),
            );
            (dirs, files)
        };
        if expanded {
            out.extend(dirs);
            out.extend(files);
        } else {
            for d in dirs {
                out.push(FileNode {
                    expanded: false,
                    ..d
                });
            }
            out.extend(files);
        }
    }

    pub(crate) fn expand(&mut self, idx: usize) {
        if idx >= self.nodes.len() || !self.nodes[idx].is_dir || self.nodes[idx].expanded {
            return;
        }
        self.nodes[idx].expanded = true;
        let node = self.nodes[idx].clone();
        let mut children = Vec::new();
        Self::add_children(
            &mut children,
            &node.path,
            node.depth + 1,
            true,
            &mut self.dir_cache,
            self.cache_ttl_secs,
        );
        let insert_at = idx + 1;
        self.nodes.splice(insert_at..insert_at, children);
    }

    pub(crate) fn collapse(&mut self, idx: usize) {
        if idx >= self.nodes.len() || !self.nodes[idx].is_dir || !self.nodes[idx].expanded {
            return;
        }
        self.nodes[idx].expanded = false;
        let depth = self.nodes[idx].depth;
        let mut remove_end = idx + 1;
        while remove_end < self.nodes.len() && self.nodes[remove_end].depth > depth {
            remove_end += 1;
        }
        self.nodes.drain(idx + 1..remove_end);
    }

    pub(crate) fn toggle_expand(&mut self, idx: usize) {
        if idx >= self.nodes.len() || !self.nodes[idx].is_dir {
            return;
        }
        if self.nodes[idx].expanded {
            self.collapse(idx);
        } else {
            self.expand(idx);
        }
    }
}

// ── Tab (per-file state for multi-tab support) ──────────────────────────────

/// Text encoding of the active buffer. Detected on load (BOM sniffing) and
/// selectable at runtime via Ctrl+U in normal mode.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum EditorEncoding {
    Utf8,
    Utf16Le,
    Utf16Be,
    Windows1252,
}

impl EditorEncoding {
    pub(crate) fn label(&self) -> &'static str {
        match self {
            Self::Utf8 => "UTF-8",
            Self::Utf16Le => "UTF-16LE",
            Self::Utf16Be => "UTF-16BE",
            Self::Windows1252 => "Windows-1252",
        }
    }

    /// Byte order mark prepended when saving in this encoding. UTF-8 and
    /// Windows-1252 have no BOM (a UTF-8 BOM would change file bytes on save);
    /// UTF-16 requires one for unambiguous detection.
    pub(crate) fn bom(&self) -> &'static [u8] {
        match self {
            Self::Utf8 => b"",
            Self::Utf16Le => b"\xff\xfe",
            Self::Utf16Be => b"\xfe\xff",
            Self::Windows1252 => b"",
        }
    }
}

/// All encodings offered by the Ctrl+U picker, in display order.
pub(crate) const ALL_ENCODINGS: [EditorEncoding; 4] = [
    EditorEncoding::Utf8,
    EditorEncoding::Utf16Le,
    EditorEncoding::Utf16Be,
    EditorEncoding::Windows1252,
];


/// Holds the complete state of one editor tab.
/// When a tab is inactive, its state is stored here.
/// When activated, the state is swapped onto the Editor's direct fields.
///
/// To add a new field: add a `pub(crate) field_name: Type` line below AND
/// add it to the `Editor` struct + `Editor::new()`. The macro keeps
/// `from_editor` and `apply_to` in sync automatically.
macro_rules! define_tab {
    ($($field:ident: $ty:ty),+ $(,)?) => {
        #[derive(Clone)]
        pub(crate) struct Tab {
            $( pub(crate) $field: $ty, )+
        }

        impl Tab {
            pub(crate) fn from_editor(e: &Editor) -> Self {
                Self { $( $field: e.$field.clone(), )+ }
            }
            pub(crate) fn apply_to(&self, e: &mut Editor) {
                $( e.$field = self.$field.clone(); )+
            }
        }
    }
}

define_tab! {
    path: PathBuf,
    lines: Vec<String>,
    modified: bool,
    cursor: CursorState,
    undo_stack: Vec<Snapshot>,
    redo_stack: Vec<Snapshot>,
    search: SearchState,
    dirty_start: usize,
    dirty_end: usize,
    edit_count: usize,
    sv: ScrollState,
    syntax: crate::syntax::SyntaxHighlighter,
    buffer_stack: Vec<PathBuf>,
    buffer_idx: usize,
    jump_buf: String,
    gmode_buf: String,
    prev_mode: Mode,
    cursor_visible: bool,
    last_blink: Instant,
    encoding: EditorEncoding,
}

// ── gs search: doubly circular linked list ──────────────────────────────────

pub(crate) struct GsResultNode {
    pub(crate) display: String,
    pub(crate) path: std::path::PathBuf,
    pub(crate) line_number: usize,
    pub(crate) prev: Option<usize>,
    pub(crate) next: Option<usize>,
}

pub(crate) struct GsResultList {
    pub(crate) nodes: Vec<GsResultNode>,
    pub(crate) head: Option<usize>,
    pub(crate) tail: Option<usize>,
    pub(crate) current: Option<usize>,
}

impl GsResultList {
    pub(crate) fn new() -> Self {
        Self {
            nodes: Vec::new(),
            head: None,
            tail: None,
            current: None,
        }
    }

    pub(crate) fn clear(&mut self) {
        self.nodes.clear();
        self.head = None;
        self.tail = None;
        self.current = None;
    }

    pub(crate) fn push_back(
        &mut self,
        display: String,
        path: std::path::PathBuf,
        line_number: usize,
    ) {
        let idx = self.nodes.len();
        let prev = self.tail;
        self.nodes.push(GsResultNode {
            display,
            path,
            line_number,
            prev,
            next: None,
        });
        if let Some(t) = self.tail {
            self.nodes[t].next = Some(idx);
        }
        self.tail = Some(idx);
        if self.head.is_none() {
            self.head = Some(idx);
        }
    }

    pub(crate) fn build_circular(&mut self) {
        if let (Some(h), Some(t)) = (self.head, self.tail) {
            self.nodes[t].next = Some(h);
            self.nodes[h].prev = Some(t);
            self.current = Some(h);
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.nodes.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub(crate) fn current_path(&self) -> Option<&std::path::Path> {
        self.current.map(|i| self.nodes[i].path.as_path())
    }

    pub(crate) fn current_line(&self) -> Option<usize> {
        self.current.map(|i| self.nodes[i].line_number)
    }

    pub(crate) fn current_index(&self) -> Option<usize> {
        self.current
    }

    pub(crate) fn move_next(&mut self) {
        if let Some(cur) = self.current {
            self.current = self.nodes[cur].next;
        }
    }

    pub(crate) fn move_prev(&mut self) {
        if let Some(cur) = self.current {
            self.current = self.nodes[cur].prev;
        }
    }

    pub(crate) fn nth_display(&self, idx: usize) -> Option<&str> {
        self.nodes.get(idx).map(|n| n.display.as_str())
    }
}

/// Normalized key for tab path comparison: strips the Windows `\\?\`
/// extended prefix (and separators/case on Windows) so snapshot-stored
/// paths match freshly requested ones.
pub(crate) fn tab_path_key(p: &Path) -> String {
    let s = crate::filetype::strip_unc_prefix(p);
    #[cfg(windows)]
    {
        s.replace('\\', "/").to_lowercase()
    }
    #[cfg(not(windows))]
    {
        s
    }
}

pub(crate) struct GsSearchState {
    pub(crate) query: String,
    pub(crate) results: GsResultList,
    pub(crate) loading: bool,
    pub(crate) loading_pattern: String,
}

impl GsSearchState {
    pub(crate) fn new() -> Self {
        Self {
            query: String::new(),
            results: GsResultList::new(),
            loading: false,
            loading_pattern: String::new(),
        }
    }
}

// ── Editor ───────────────────────────────────────────────────────────────────

pub(crate) struct Editor {
    pub(crate) lines: Vec<String>,
    pub(crate) path: std::path::PathBuf,
    pub(crate) modified: bool,

    // Terminal dimensions
    pub(crate) term_h: usize,
    pub(crate) term_w: usize,

    // Internal clipboard (always available cross-platform).
    pub(crate) clipboard: String,

    // Undo / redo stacks.
    pub(crate) undo_stack: Vec<Snapshot>,
    pub(crate) redo_stack: Vec<Snapshot>,

    // Editor mode.
    pub(crate) mode: Mode,

    // Status message (shown briefly in the hint area).
    pub(crate) status_msg: Option<String>,

    // Command mode buffer.
    pub(crate) cmd_buf: String,

    // Dirty region tracking
    pub(crate) dirty_start: usize,
    pub(crate) dirty_end: usize,

    // Sub-structs for grouped state
    pub(crate) cursor: CursorState,
    pub(crate) search: SearchState,
    pub(crate) sv: ScrollState,

    // Paste batching (Windows: chars arrive as individual events)
    pub(crate) paste_buf: String,

    // Persistent editor configuration (loaded from editor.toml)
    pub(crate) editor_cfg: EditorConfig,

    // Syntax highlighting engine
    pub(crate) syntax: SyntaxHighlighter,

    // Left-side file explorer sidebar
    pub(crate) sidebar: Sidebar,

    // File finder (Ctrl+P) state
    pub(crate) ff_query: String,
    pub(crate) ff_results: Vec<(String, std::path::PathBuf, f64, usize)>,
    pub(crate) ff_idx: usize,
    /// Cached file index for fast file finder: (timestamp, files)
    pub(crate) file_index: Option<(std::time::Instant, Vec<(String, std::path::PathBuf)>)>,

    // Gs content search state
    pub(crate) gs_state: GsSearchState,

    // Gosc mode state
    pub(crate) gosc_dirs: Vec<String>,
    pub(crate) gosc_buf: String,

    // Query mode state
    pub(crate) query_buf: String,
    pub(crate) qal_aliases: HashMap<String, String>,

    // Run mode state
    pub(crate) run_lines: Vec<String>,
    pub(crate) run_cmd_buf: String,
    pub(crate) run_executing: Option<String>,
    pub(crate) last_run_cmd: Option<String>,
    pub(crate) run_rx: Option<std::sync::mpsc::Receiver<RunMessage>>,
    pub(crate) run_child: Option<std::process::Child>,
    pub(crate) stored_run_split: usize,
    pub(crate) dragging_run_border: bool,
    pub(crate) run_panel_open: bool,
    pub(crate) focus: Focus,

    // Recently opened files buffer stack
    pub(crate) buffer_stack: Vec<std::path::PathBuf>,
    pub(crate) buffer_idx: usize,

    // Hover popup (normal mode, word under cursor)
    pub(crate) hover_text: Option<String>,
    pub(crate) hover_sig_active: bool,

    // Crash recovery: counter to periodically save recovery snapshots
    pub(crate) edit_count: usize,

    // File watcher for live reload on external changes (Windows only).
    // `disk_fingerprint` is the cross-platform fallback AND the primary
    // change signal: (mtime, len) of the open file the last time the
    // buffer agreed with disk. Polled every main-loop iteration, so it
    // also catches atomic saves (write-temp + rename) that never emit a
    // `Modify` notify event, and works where notify is unavailable.
    #[cfg(windows)]
    pub(crate) file_watcher_rx: Option<std::sync::mpsc::Receiver<()>>,
    #[cfg(windows)]
    pub(crate) _file_watcher: Option<notify::RecommendedWatcher>,
    pub(crate) disk_fingerprint: Option<(SystemTime, u64)>,
    /// Disk changed while the buffer has unsaved edits: warn instead of
    /// clobbering; `:e!` force-reloads.
    pub(crate) external_change_pending: bool,

    // ── Multi-tab support ─────────────────────────────────────────────────
    pub(crate) tabs: Vec<Tab>,
    pub(crate) active_tab: usize,
    // Set by mouse handler when close button on last tab is clicked
    pub(crate) pending_quit: bool,

    // Jump mode state
    pub(crate) jump_buf: String,
    pub(crate) gmode_buf: String,
    // Nfe command prompt input (None = navigating)
    pub(crate) nfe_cmd: Option<String>,
    // VSCode-style completion list + signature help (insert only)
    pub(crate) completion: Option<super::complete::CompletionState>,
    pub(crate) sighelp: Option<super::complete::SigHelpState>,
    pub(crate) prev_mode: Mode,

    // Cursor blinking
    pub(crate) cursor_visible: bool,
    pub(crate) last_blink: Instant,

    // Session auto-save timer
    pub(crate) last_session_save: Instant,

    // Terminal state for embedded terminal
    pub(crate) terminal: Option<super::terminal::TerminalState>,

    // Text encoding of the active buffer
    pub(crate) encoding: EditorEncoding,

    // Selected row in the Ctrl+U encoding picker
    pub(crate) encoding_sel: usize,

    // Git diff overlay state
    pub(crate) git_diff: Option<GitDiffState>,

    /// Transient `gs -i` jump highlight: whole-line bg on this 0-based row.
    /// Cleared on any key press or mouse click (scroll excluded).
    pub(crate) gs_jump_highlight: Option<usize>,
}

#[derive(Clone)]
pub(crate) struct GitDiffState {
    pub(crate) lines: Vec<String>,
    pub(crate) scroll: usize,
}

impl Editor {
    pub(crate) fn new(path: &Path) -> Result<Self> {
        let (content, encoding) = if path.exists() {
            let (lines, enc) = crate::editor::core::read_file_lines(path)
                .with_context(|| format!("Cannot read {}", path.display()))?;
            (lines, enc)
        } else {
            (vec![], EditorEncoding::Utf8)
        };
        let lines = if content.is_empty() {
            vec![String::new()]
        } else {
            content
        };
        let (tw, th) = crate::editor::helpers::term_size();
        let editor_cfg = EditorConfig::load();
        let mut syntax =
            crate::syntax::SyntaxHighlighter::new(path.extension().and_then(|e| e.to_str()));
        syntax.resize_cache(lines.len());
        // Pre-tokenize entire file if small enough for instant scrolling
        if editor_cfg.syntax_enabled && lines.len() <= 5000 {
            syntax.pre_tokenize_up_to(lines.len(), &lines);
        }
        Ok(Self {
            lines,
            path: path.to_path_buf(),
            modified: false,
            term_h: th,
            term_w: tw,
            clipboard: String::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            mode: Mode::Normal,
            status_msg: None,
            cmd_buf: String::new(),
            dirty_start: 0,
            dirty_end: 0,
            cursor: CursorState::new(),
            search: SearchState::new(),
            sv: ScrollState::new(),
            paste_buf: String::new(),
            editor_cfg,
            syntax,
            sidebar: Sidebar::new(),
            ff_query: String::new(),
            ff_results: Vec::new(),
            ff_idx: 0,
            file_index: None,
            gs_state: GsSearchState::new(),
            gosc_dirs: Vec::new(),
            gosc_buf: String::new(),
            query_buf: String::new(),
            qal_aliases: HashMap::new(),
            run_lines: Vec::new(),
            run_cmd_buf: String::new(),
            run_executing: None,
            last_run_cmd: None,
            run_rx: None,
            run_child: None,
            stored_run_split: (th * 2 / 3).max(5),
            dragging_run_border: false,
            run_panel_open: false,
            focus: Focus::Editor,
            buffer_stack: vec![path.to_path_buf()],
            buffer_idx: 0,
            hover_text: None,
            hover_sig_active: false,
            edit_count: 0,
            #[cfg(windows)]
            file_watcher_rx: None,
            #[cfg(windows)]
            _file_watcher: None,
            disk_fingerprint: Self::fingerprint_static(path),
            external_change_pending: false,
            tabs: Vec::new(),
            active_tab: 0,
            pending_quit: false,
            jump_buf: String::new(),
            gmode_buf: String::new(),
            nfe_cmd: None,
            completion: None,
            sighelp: None,
            prev_mode: Mode::Normal,
            cursor_visible: true,
            last_blink: Instant::now(),
            last_session_save: Instant::now(),
            terminal: None,
            encoding,
            encoding_sel: 0,
            git_diff: None,
            gs_jump_highlight: None,
        })
    }

    /// Finalise setup after construction: register the first tab and sync
    /// the syntax highlighter used by Tab::from_editor.
    pub(crate) fn init_tabs(&mut self) {
        self.tabs.push(Tab::from_editor(self));
        self.active_tab = 0;
    }

    /// Jump the cursor to line `line` (1-based). Called before run().
    pub(crate) fn goto_line(&mut self, line: usize) {
        if line > 0 && !self.lines.is_empty() {
            self.cursor.y = (line.saturating_sub(1)).min(self.lines.len().saturating_sub(1));
            self.cursor.byte = 0;
            self.scroll_visible();
        }
    }

    /// Jump to `line` (1-based), center it vertically, and flag a transient
    /// whole-line highlight (used by `gs -i` Enter).
    pub(crate) fn goto_line_centered_highlight(&mut self, line: usize) {
        if self.lines.is_empty() {
            return;
        }
        let y = line
            .saturating_sub(1)
            .min(self.lines.len().saturating_sub(1));
        self.cursor.y = y;
        self.cursor.byte = 0;
        self.sv.x = 0;
        let rows = self.visible_text_rows().max(1);
        let max_offset = self.lines.len().saturating_sub(rows);
        self.sv.offset = y.saturating_sub(rows / 2).min(max_offset);
        self.gs_jump_highlight = Some(y);
        self.mark_all_dirty();
    }

    /// Clear the transient `gs -i` jump highlight (key press / mouse click).
    pub(crate) fn clear_gs_jump_highlight(&mut self) {
        if self.gs_jump_highlight.take().is_some() {
            self.mark_all_dirty();
        }
    }

    /// On-disk fingerprint (mtime, byte length) for `path`, or `None`
    /// when the file does not exist (scratch / not yet created).
    fn fingerprint_static(path: &Path) -> Option<(SystemTime, u64)> {
        std::fs::metadata(path)
            .ok()
            .and_then(|m| m.modified().ok().map(|t| (t, m.len())))
    }

    /// Record the current on-disk state as "agreed with buffer". Call
    /// after every load, save, and successful external reload.
    pub(crate) fn snapshot_disk_state(&mut self) {
        self.disk_fingerprint = Self::fingerprint_static(&self.path);
        self.external_change_pending = false;
    }

    /// Re-arm the notify watcher for the current file (startup watches
    /// only the first file; tab switches and `:ne` change `self.path`).
    /// Also re-snapshots the disk fingerprint for the new file.
    pub(crate) fn restart_file_watcher(&mut self) {
        #[cfg(windows)]
        {
            self.file_watcher_rx = None;
            self._file_watcher = None;
            self.start_file_watcher();
        }
        self.snapshot_disk_state();
    }

    /// Start watching the current file for external changes (Windows only).
    /// Forwards every non-access event kind: atomic saves (vscode, vim,
    /// MetaEditor) replace the file via temp + rename, which surfaces as
    /// `Create`/`Remove`, never as `Modify`.
    #[cfg(windows)]
    pub(crate) fn start_file_watcher(&mut self) {
        use notify::{RecursiveMode, Watcher};
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();

        let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                if !matches!(event.kind, notify::EventKind::Access(_)) {
                    let _ = tx.send(());
                }
            }
        })
        .ok();

        if let Some(ref mut w) = watcher {
            let _ = w.watch(&self.path, RecursiveMode::NonRecursive);
            self.file_watcher_rx = Some(rx);
            self._file_watcher = watcher.take();
        }
    }

    #[cfg(not(windows))]
    pub(crate) fn start_file_watcher(&mut self) {}

    /// Load the on-disk version of the current file, discarding buffer
    /// edits. Used by `:e!` and by the clean auto-reload path.
    pub(crate) fn reload_from_disk(&mut self) -> bool {
        let content = match std::fs::read(&self.path) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        // Decode like the file loader (BOM/UTF-16 aware), falling back
        // to lossy UTF-8 for anything else.
        let new_lines: Vec<String> =
            match crate::editor::core::read_file_lines(&self.path) {
                Ok((lines, enc)) => {
                    self.encoding = enc;
                    lines
                }
                Err(_) => String::from_utf8_lossy(&content)
                    .lines()
                    .map(|l| l.to_string())
                    .collect(),
            };
        self.lines = if new_lines.is_empty() {
            vec![String::new()]
        } else {
            new_lines
        };
        self.cursor.y = self.cursor.y.min(self.lines.len().saturating_sub(1));
        self.cursor.byte = 0;
        self.cursor.selection_anchor = None;
        self.cursor.extra_cursors.clear();
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.modified = false;
        self.dismiss_popups();
        self.syntax.resize_cache(self.lines.len());
        self.snapshot_disk_state();
        self.mark_all_dirty();
        true
    }

    /// Check for external file changes and reload if needed.
    /// Polled every main-loop iteration. Detection is fingerprint-first
    /// (mtime + length), so atomic saves and non-Windows platforms work
    /// without notify events; the notify channel only hurries the poll.
    /// Returns true if the file was reloaded.
    ///
    /// - Buffer clean   → silent reload, cursor kept when possible.
    /// - Buffer dirty   → never clobber: warn in the status bar and set
    ///   `external_change_pending` (`:e!` discards edits and reloads).
    /// - Same content   → fingerprint refresh, no reload (covers our own
    ///   saves racing the watcher).
    pub(crate) fn check_file_changed(&mut self) -> bool {
        if self.is_scratch_path(&self.path) {
            return false;
        }
        #[cfg(windows)]
        let notified = match &self.file_watcher_rx {
            Some(rx) => {
                let mut n = false;
                while rx.try_recv().is_ok() {
                    n = true;
                }
                n
            }
            None => false,
        };
        #[cfg(not(windows))]
        let notified = false;
        let fp = Self::fingerprint_static(&self.path);
        if !notified && fp == self.disk_fingerprint {
            return false;
        }
        // The file vanished (deleted / moved away mid-rename): warn once
        // per fingerprint change, keep the buffer.
        if fp.is_none() && self.disk_fingerprint.is_some() {
            self.disk_fingerprint = None;
            self.status_msg = Some("File deleted externally — buffer kept".into());
            self.mark_all_dirty();
            return false;
        }
        let disk_lines: Vec<String> = match crate::editor::core::read_file_lines(&self.path) {
            Ok((lines, _)) => lines,
            Err(_) => {
                // Unreadable (locked mid-save, permissions): retry later
                // without updating the fingerprint.
                return false;
            }
        };
        if disk_lines == self.lines {
            self.disk_fingerprint = fp;
            self.external_change_pending = false;
            return false;
        }
        if self.modified {
            self.external_change_pending = true;
            self.disk_fingerprint = fp;
            self.status_msg = Some(
                "File changed externally — buffer has unsaved edits (:e! to load disk version)".into(),
            );
            self.mark_all_dirty();
            return false;
        }
        if self.reload_from_disk() {
            self.status_msg = Some("File changed externally — reloaded".into());
            return true;
        }
        false
    }

    // ── session persistence ──────────────────────────────────────────────────

    pub(crate) fn capture_session(&self) -> EditorSession {
        EditorSession {
            current_file: self.path.clone(),
            cursor_y: self.cursor.y,
            cursor_byte: self.cursor.byte,
            scroll: self.sv.offset,
            scroll_x: self.sv.x,
            buffer_stack: self.buffer_stack.clone(),
            buffer_idx: self.buffer_idx,
            sidebar_open: self.sidebar.open,
        }
    }

    pub(crate) fn restore_from_session(&mut self, session: &EditorSession) {
        let max = self.lines.len().saturating_sub(1);
        self.cursor.y = session.cursor_y.min(max);
        self.cursor.byte = session.cursor_byte.min(self.current().len());
        self.sv.offset = session.scroll.min(max);
        self.sv.x = session.scroll_x;
        self.sidebar.open = session.sidebar_open;
        if !session.buffer_stack.is_empty() {
            self.buffer_stack = session.buffer_stack.clone();
            self.buffer_idx = session
                .buffer_idx
                .min(self.buffer_stack.len().saturating_sub(1));
        }
    }

    /// Capture the full state of all tabs for session persistence.
    pub(crate) fn capture_full_session(&mut self) -> SessionSnapshot {
        self.save_active_tab();
        let tabs: Vec<TabSnapshot> = self
            .tabs
            .iter()
            .map(|tab| TabSnapshot {
                path: tab.path.clone(),
                cursor_y: tab.cursor.y,
                cursor_byte: tab.cursor.byte,
                scroll: tab.sv.offset,
                scroll_x: tab.sv.x,
                modified: tab.modified,
                undo_stack: tab.undo_stack.clone(),
                redo_stack: tab.redo_stack.clone(),
            })
            .collect();
        SessionSnapshot {
            tabs,
            active_tab: self.active_tab,
            buffer_stack: self.buffer_stack.clone(),
            buffer_idx: self.buffer_idx,
            sidebar_open: self.sidebar.open,
            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    /// Find the tab index for a given path.
    /// Compares UNC-normalized paths so `\\?\C:\x` (canonicalized on
    /// Windows) matches plain `C:\x` request paths.
    pub(crate) fn find_tab_by_path(&self, path: &Path) -> Option<usize> {
        let want = tab_path_key(path);
        self.tabs.iter().position(|t| tab_path_key(&t.path) == want)
    }

    /// Restore tab `idx`'s cursor/scroll/undo state from `tab_snap`. Cursor is
    /// clamped against its OWN line (char-boundary), not the last line.
    pub(crate) fn restore_tab_from_snapshot_path(&mut self, idx: usize, tab_snap: &TabSnapshot) {
        if idx >= self.tabs.len() {
            return;
        }
        let max_y = self.tabs[idx].lines.len().saturating_sub(1);
        let cy = tab_snap.cursor_y.min(max_y);
        self.tabs[idx].cursor.y = cy;
        let clen = self.tabs[idx].lines[cy].len();
        let mut cb = tab_snap.cursor_byte.min(clen);
        while cb > 0 && !self.tabs[idx].lines[cy].is_char_boundary(cb) {
            cb -= 1;
        }
        self.tabs[idx].cursor.byte = cb;
        self.tabs[idx].sv.offset = tab_snap.scroll.min(max_y);
        self.tabs[idx].sv.x = tab_snap.scroll_x;
        self.tabs[idx].modified = tab_snap.modified;
        self.tabs[idx].undo_stack = tab_snap.undo_stack.clone();
        self.tabs[idx].redo_stack = tab_snap.redo_stack.clone();
    }

    /// Normal-mode cursor maintenance: Normal mode is always popup-free.
    /// Clears any insert-mode leftovers (`sighelp`, hover line/flag) and
    /// repaints when something was visible so no ghost box survives the
    /// mode switch or cursor move. (Word-hover popups were removed; the
    /// signature info lives in insert-mode sig-help only.)
    pub(crate) fn update_hover(&mut self) {
        self.hover_sig_active = false;
        let mut changed = false;
        if self.hover_text.take().is_some() {
            changed = true;
        }
        if self.sighelp.is_some() {
            self.sighelp = None;
            changed = true;
        }
        if changed {
            self.mark_all_dirty();
        }
    }
}
