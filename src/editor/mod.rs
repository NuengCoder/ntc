// ── submodules ──────────────────────────────────────────────────────────────

mod types;
mod unicode;
mod helpers;
mod core;
mod hex;
mod edit;
mod modes;
mod render;
mod mouse;
mod run;
mod template;
mod editor_config;
mod completion;
mod recovery;
mod commands;
pub(crate) mod terminal;

// ── re-exports for sibling modules ──────────────────────────────────────────

pub(crate) use types::{
    Editor, Focus, Mode, Snapshot, CursorPos, Tab, HexViewer, GOTO_LINE,
    MIN_SIDEBAR_WIDTH,
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

pub use template::{edit_file, edit_file_with_session, init_file, generate_template, generate_super_template, init_super_file, generate_welcome_content, init_hyper_project};
