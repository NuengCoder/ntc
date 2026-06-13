use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::editor::editor_config::EditorConfig;
use crate::session::EditorSession;
use crate::syntax::SyntaxHighlighter;

// ── constants ────────────────────────────────────────────────────────────────

/// Maximum undo history depth.
pub(crate) const MAX_UNDO: usize = 200;

/// Width of the left sidebar (file explorer) in characters.
pub(crate) const SIDEBAR_WIDTH: usize = 30;

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
        };
        s.rebuild_tree();
        s
    }

    pub(crate) fn rebuild_tree(&mut self) {
        self.root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
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
        self.selected = self.selected.min(self.nodes.len().saturating_sub(1));
    }

    fn add_children(out: &mut Vec<FileNode>, dir: &std::path::Path, depth: usize, expanded: bool) {
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
/// When activated, the state is swapped onto the Editor's direct fields
/// so that existing code working with `self.lines`, `self.cursor_y`, etc.
/// continues to work unchanged.
#[derive(Clone)]
pub(crate) struct Tab {
    pub(crate) path: PathBuf,
    pub(crate) lines: Vec<String>,
    pub(crate) modified: bool,
    pub(crate) cursor_y: usize,
    pub(crate) cursor_byte: usize,
    pub(crate) scroll: usize,
    pub(crate) scroll_x: usize,
    pub(crate) selection_anchor: Option<(usize, usize)>,
    pub(crate) undo_stack: Vec<Snapshot>,
    pub(crate) redo_stack: Vec<Snapshot>,
    pub(crate) search_query: String,
    pub(crate) search_matches: Vec<(usize, usize, usize)>,
    pub(crate) search_match_idx: usize,
    pub(crate) dirty_start: usize,
    pub(crate) dirty_end: usize,
    pub(crate) prev_cursor_y: usize,
    pub(crate) prev_cursor_byte: usize,
    pub(crate) extra_cursors: Vec<CursorPos>,
    pub(crate) last_added_cursor_idx: Option<usize>,
    pub(crate) edit_count: usize,
    pub(crate) dragging_vscroll: bool,
    pub(crate) dragging_hscroll: bool,
    pub(crate) syntax: crate::syntax::SyntaxHighlighter,
    pub(crate) buffer_stack: Vec<PathBuf>,
    pub(crate) buffer_idx: usize,
}

impl Tab {
    /// Capture the current editor state into a Tab snapshot.
    pub(crate) fn from_editor(e: &Editor) -> Self {
        Self {
            path: e.path.clone(),
            lines: e.lines.clone(),
            modified: e.modified,
            cursor_y: e.cursor_y,
            cursor_byte: e.cursor_byte,
            scroll: e.scroll,
            scroll_x: e.scroll_x,
            selection_anchor: e.selection_anchor,
            undo_stack: e.undo_stack.clone(),
            redo_stack: e.redo_stack.clone(),
            search_query: e.search_query.clone(),
            search_matches: e.search_matches.clone(),
            search_match_idx: e.search_match_idx,
            dirty_start: e.dirty_start,
            dirty_end: e.dirty_end,
            prev_cursor_y: e.prev_cursor_y,
            prev_cursor_byte: e.prev_cursor_byte,
            extra_cursors: e.extra_cursors.clone(),
            last_added_cursor_idx: e.last_added_cursor_idx,
            edit_count: e.edit_count,
            dragging_vscroll: e.dragging_vscroll,
            dragging_hscroll: e.dragging_hscroll,
            syntax: e.syntax.clone(),
            buffer_stack: e.buffer_stack.clone(),
            buffer_idx: e.buffer_idx,
        }
    }

    /// Restore editor state from this Tab snapshot.
    /// Clipboard is NOT restored from the tab — it stays shared globally.
    pub(crate) fn apply_to(&self, e: &mut Editor) {
        e.path = self.path.clone();
        e.lines = self.lines.clone();
        e.modified = self.modified;
        e.cursor_y = self.cursor_y;
        e.cursor_byte = self.cursor_byte;
        e.scroll = self.scroll;
        e.scroll_x = self.scroll_x;
        e.selection_anchor = self.selection_anchor;
        e.undo_stack = self.undo_stack.clone();
        e.redo_stack = self.redo_stack.clone();
        e.search_query = self.search_query.clone();
        e.search_matches = self.search_matches.clone();
        e.search_match_idx = self.search_match_idx;
        e.dirty_start = self.dirty_start;
        e.dirty_end = self.dirty_end;
        e.prev_cursor_y = self.prev_cursor_y;
        e.prev_cursor_byte = self.prev_cursor_byte;
        e.extra_cursors = self.extra_cursors.clone();
        e.last_added_cursor_idx = self.last_added_cursor_idx;
        e.edit_count = self.edit_count;
        e.dragging_vscroll = self.dragging_vscroll;
        e.dragging_hscroll = self.dragging_hscroll;
        e.syntax = self.syntax.clone();
        e.buffer_stack = self.buffer_stack.clone();
        e.buffer_idx = self.buffer_idx;
    }
}

// ── Editor ───────────────────────────────────────────────────────────────────

pub(crate) struct Editor {
    pub(crate) lines: Vec<String>,
    pub(crate) path: std::path::PathBuf,
    pub(crate) modified: bool,

    // Cursor
    pub(crate) cursor_y: usize,
    /// Byte offset into the current line (NOT char index).
    pub(crate) cursor_byte: usize,

    // Scroll
    pub(crate) scroll: usize,
    pub(crate) scroll_x: usize,

    // Terminal dimensions
    pub(crate) term_h: usize,
    pub(crate) term_w: usize,

    // Selection: anchor is the OTHER end of the selection from the cursor.
    // When None, no selection active.
    pub(crate) selection_anchor: Option<(usize, usize)>,

    // Internal clipboard (always available cross-platform).
    pub(crate) clipboard: String,

    // Undo / redo stacks.
    pub(crate) undo_stack: Vec<Snapshot>,
    pub(crate) redo_stack: Vec<Snapshot>,

    // Editor mode.
    pub(crate) mode: Mode,

    // Search state.
    pub(crate) search_query: String,
    pub(crate) search_matches: Vec<(usize, usize, usize)>,
    pub(crate) search_match_idx: usize,

    // Status message (shown briefly in the hint area).
    pub(crate) status_msg: Option<String>,

    // Command mode buffer.
    pub(crate) cmd_buf: String,

    // Help screen scroll offset.
    pub(crate) help_scroll: usize,

    // Dirty region tracking
    pub(crate) dirty_start: usize,
    pub(crate) dirty_end: usize,
    pub(crate) prev_cursor_y: usize,
    pub(crate) prev_cursor_byte: usize,

    // Paste batching (Windows: chars arrive as individual events)
    pub(crate) paste_buf: String,

    // Persistent editor configuration (loaded from editor.toml)
    pub(crate) editor_cfg: EditorConfig,

    // Syntax highlighting engine
    pub(crate) syntax: SyntaxHighlighter,

    // Multi-cursor: extra cursors beyond the primary (cursor_y, cursor_byte)
    pub(crate) extra_cursors: Vec<CursorPos>,
    // Index into extra_cursors of the most recently added cursor (for jump-to-last)
    pub(crate) last_added_cursor_idx: Option<usize>,

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
    pub(crate) run_scroll: usize,
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

    // Scroll bar drag state
    pub(crate) dragging_vscroll: bool,
    pub(crate) dragging_hscroll: bool,

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
            cursor_y: 0,
            cursor_byte: 0,
            scroll: 0,
            scroll_x: 0,
            term_h: th,
            term_w: tw,
            selection_anchor: None,
            clipboard: String::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            mode: Mode::Normal,
            search_query: String::new(),
            search_matches: Vec::new(),
            search_match_idx: 0,
            status_msg: None,
            cmd_buf: String::new(),
            help_scroll: 0,
            dirty_start: 0,
            dirty_end: 0,
            prev_cursor_y: 0,
            prev_cursor_byte: 0,
            paste_buf: String::new(),
            editor_cfg: EditorConfig::load(),
            syntax,
            extra_cursors: Vec::new(),
            last_added_cursor_idx: None,
            sidebar: Sidebar::new(),
            ff_query: String::new(),
            ff_results: Vec::new(),
            ff_idx: 0,
            gosc_dirs: Vec::new(),
            gosc_buf: String::new(),
            run_lines: Vec::new(),
            run_scroll: 0,
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
            dragging_vscroll: false,
            dragging_hscroll: false,
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

    // ── session persistence ──────────────────────────────────────────────────

    pub(crate) fn capture_session(&self) -> EditorSession {
        EditorSession {
            current_file: self.path.clone(),
            cursor_y: self.cursor_y,
            cursor_byte: self.cursor_byte,
            scroll: self.scroll,
            scroll_x: self.scroll_x,
            buffer_stack: self.buffer_stack.clone(),
            buffer_idx: self.buffer_idx,
            sidebar_open: self.sidebar.open,
        }
    }

    pub(crate) fn restore_from_session(&mut self, session: &EditorSession) {
        let max = self.lines.len().saturating_sub(1);
        self.cursor_y = session.cursor_y.min(max);
        self.cursor_byte = session.cursor_byte.min(self.current().len());
        self.scroll = session.scroll.min(max);
        self.scroll_x = session.scroll_x;
        self.sidebar.open = session.sidebar_open;
        if !session.buffer_stack.is_empty() {
            self.buffer_stack = session.buffer_stack.clone();
            self.buffer_idx = session.buffer_idx.min(self.buffer_stack.len().saturating_sub(1));
        }
    }
}
