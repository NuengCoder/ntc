use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::editor::editor_config::EditorConfig;
use crate::session::EditorSession;
use crate::syntax::SyntaxHighlighter;

// ── constants ────────────────────────────────────────────────────────────────

/// Maximum undo history depth.
pub(crate) const MAX_UNDO: usize = 200;

/// Minimum width of the left sidebar (file explorer) in characters.
pub(crate) const MIN_SIDEBAR_WIDTH: usize = 15;

/// Default width of the left sidebar (file explorer) in characters.
pub(crate) const DEFAULT_SIDEBAR_WIDTH: usize = 30;

/// Maximum number of recently opened files to keep in the buffer stack.
pub(crate) const MAX_BUFFERS: usize = 16;

/// Maximum number of open tabs.
pub(crate) const MAX_TABS: usize = 32;

/// Maximum lines to retain in the run output buffer.
pub(crate) const MAX_RUN_LINES: usize = 10_000;

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
    Search,
    Visual,
    Command,
    Help,
    FileFinder,
    Gosc,
    Run,
}

// ── snapshot for undo/redo ────────────────────────────────────────────────────

#[derive(Clone)]
pub(crate) struct Snapshot {
    pub(crate) lines: Vec<String>,
    pub(crate) cursor_y: usize,
    pub(crate) cursor_byte: usize,
    pub(crate) extra_cursors: Vec<CursorPos>,
}

// ── cursor ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
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
}

impl SearchState {
    pub(crate) fn new() -> Self {
        Self {
            query: String::new(),
            matches: Vec::new(),
            match_idx: 0,
        }
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
    pub(crate) width: usize,
    pub(crate) dragging_border: bool,
    // File watcher for live reload
    pub(crate) watcher_rx: Option<std::sync::mpsc::Receiver<()>>,
    pub(crate) _watcher: Option<notify::RecommendedWatcher>,
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
            width: DEFAULT_SIDEBAR_WIDTH,
            dragging_border: false,
            watcher_rx: None,
            _watcher: None,
        };
        s.rebuild_tree();
        s.start_watcher();
        s
    }

    /// Start watching the root directory for changes.
    fn start_watcher(&mut self) {
        use notify::{Watcher, RecursiveMode};
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();
        let path = self.root.clone();

        let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(_event) = res {
                let _ = tx.send(());
            }
        }).ok();

        if let Some(ref mut w) = watcher {
            let _ = w.watch(&path, RecursiveMode::NonRecursive);
            self.watcher_rx = Some(rx);
            self._watcher = watcher.take();
        }
    }

    /// Check for directory changes and rebuild tree if needed.
    /// Returns true if the tree was rebuilt.
    pub(crate) fn check_changed(&mut self) -> bool {
        if let Some(rx) = &self.watcher_rx {
            if rx.try_recv().is_ok() {
                self.rebuild_tree();
                return true;
            }
        }
        false
    }

    pub(crate) fn rebuild_tree(&mut self) {
        self.root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        // Save expanded directory paths before clearing
        let expanded: std::collections::HashSet<std::path::PathBuf> = self.nodes
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
        Self::add_children(&mut self.nodes, &self.root, 0, false);
        // Restore expanded state
        for node in &mut self.nodes {
            if node.is_dir && expanded.contains(&node.path) {
                node.expanded = true;
            }
        }
        self.selected = self.selected.min(self.nodes.len().saturating_sub(1));
    }

    fn add_children(out: &mut Vec<FileNode>, dir: &std::path::Path, depth: usize, expanded: bool) {
        // Limit recursion depth to avoid stack overflow on deeply nested dirs
        const MAX_DEPTH: usize = 64;
        if depth >= MAX_DEPTH {
            return;
        }
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
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
        if expanded {
            out.extend(dirs);
            out.extend(files);
        } else {
            // Show dirs (collapsed) and files at this level
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
        Self::add_children(&mut children, &node.path, node.depth + 1, true);
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
    pub(crate) ff_results: Vec<(String, std::path::PathBuf, f64)>,
    pub(crate) ff_idx: usize,

    // Gosc mode state
    pub(crate) gosc_dirs: Vec<String>,
    pub(crate) gosc_buf: String,

    // Run mode state
    pub(crate) run_lines: Vec<String>,
    pub(crate) run_cmd_buf: String,
    pub(crate) run_executing: Option<String>,
    pub(crate) run_rx: Option<std::sync::mpsc::Receiver<RunMessage>>,
    pub(crate) run_child: Option<std::process::Child>,

    // Recently opened files buffer stack
    pub(crate) buffer_stack: Vec<std::path::PathBuf>,
    pub(crate) buffer_idx: usize,

    // Completion state
    pub(crate) completion_items: Vec<super::CompletionItem>,
    pub(crate) completion_idx: usize,
    pub(crate) completion_visible: bool,
    pub(crate) completion_prefix: String,

    // Crash recovery: counter to periodically save recovery snapshots
    pub(crate) edit_count: usize,

    // File watcher for live reload on external changes
    pub(crate) file_watcher_rx: Option<std::sync::mpsc::Receiver<()>>,
    pub(crate) _file_watcher: Option<notify::RecommendedWatcher>,

    // ── Multi-tab support ─────────────────────────────────────────────────
    pub(crate) tabs: Vec<Tab>,
    pub(crate) active_tab: usize,
    // Set by mouse handler when close button on last tab is clicked
    pub(crate) pending_quit: bool,
}

impl Editor {
    pub(crate) fn new(path: &Path) -> Result<Self> {
        let content: Vec<String> = if path.exists() {
            let file = std::fs::File::open(path)
                .with_context(|| format!("Cannot open {}", path.display()))?;
            BufReader::new(file)
                .lines()
                .collect::<std::io::Result<Vec<_>>>()
                .with_context(|| format!("Cannot read {}", path.display()))?
        } else {
            vec![]
        };
        let lines = if content.is_empty() {
            vec![String::new()]
        } else {
            content
        };
        let (tw, th) = crate::editor::helpers::term_size();
        let mut syntax = crate::syntax::SyntaxHighlighter::new(path.extension().and_then(|e| e.to_str()));
        syntax.resize_cache(lines.len());
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
            editor_cfg: EditorConfig::load(),
            syntax,
            sidebar: Sidebar::new(),
            ff_query: String::new(),
            ff_results: Vec::new(),
            ff_idx: 0,
            gosc_dirs: Vec::new(),
            gosc_buf: String::new(),
            run_lines: Vec::new(),
            run_cmd_buf: String::new(),
            run_executing: None,
            run_rx: None,
            run_child: None,
            buffer_stack: vec![path.to_path_buf()],
            buffer_idx: 0,
            completion_items: Vec::new(),
            completion_idx: 0,
            completion_visible: false,
            completion_prefix: String::new(),
            edit_count: 0,
            file_watcher_rx: None,
            _file_watcher: None,
            tabs: Vec::new(),
            active_tab: 0,
            pending_quit: false,
        })
    }

    /// Finalise setup after construction: register the first tab and sync
    /// the syntax highlighter used by Tab::from_editor.
    pub(crate) fn init_tabs(&mut self) {
        self.tabs.push(Tab::from_editor(self));
        self.active_tab = 0;
    }

    /// Start watching the current file for external changes.
    pub(crate) fn start_file_watcher(&mut self) {
        use notify::{Watcher, RecursiveMode};
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();
        let path = self.path.clone();

        let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                if matches!(event.kind, notify::EventKind::Modify(_)) {
                    let _ = tx.send(());
                }
            }
        }).ok();

        if let Some(ref mut w) = watcher {
            let _ = w.watch(&path, RecursiveMode::NonRecursive);
            self.file_watcher_rx = Some(rx);
            self._file_watcher = watcher.take();
        }
    }

    /// Check for external file changes and reload if needed.
    /// Returns true if the file was reloaded.
    pub(crate) fn check_file_changed(&mut self) -> bool {
        if let Some(rx) = &self.file_watcher_rx {
            if rx.try_recv().is_ok() {
                // Reload file content
                if let Ok(content) = std::fs::read_to_string(&self.path) {
                    let new_lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
                    let new_lines = if new_lines.is_empty() {
                        vec![String::new()]
                    } else {
                        new_lines
                    };
                    self.lines = new_lines;
                    self.syntax.resize_cache(self.lines.len());
                    self.mark_all_dirty();
                    self.status_msg = Some("File changed externally — reloaded".into());
                    return true;
                }
            }
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
            self.buffer_idx = session.buffer_idx.min(self.buffer_stack.len().saturating_sub(1));
        }
    }
}
