// ── submodules ──────────────────────────────────────────────────────────────

mod types;
mod unicode;
mod helpers;
mod core;
mod edit;
mod modes;
mod render;
mod mouse;
mod run;
mod template;
mod editor_config;
mod completion;

// ── re-exports for sibling modules ──────────────────────────────────────────

pub(crate) use types::{
    Editor, Mode, Snapshot, CursorPos,
    MAX_UNDO, SIDEBAR_WIDTH, MAX_BUFFERS,
};
pub(crate) use completion::CompletionItem;

pub(crate) use unicode::{
    char_col_width, byte_to_col, col_to_byte,
    prev_char_byte, next_char_byte,
    next_word_byte, prev_word_byte,
    is_ctrl_char, auto_pair,
};

pub(crate) use helpers::{
    gutter_width, set_status_bar, prompt_yes_no, sel_range,
};

// ── public API ──────────────────────────────────────────────────────────────

pub use template::{edit_file, edit_file_with_session, init_file, generate_template};
