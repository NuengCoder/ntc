// ── submodules ──────────────────────────────────────────────────────────────

mod commands;
mod completion;
mod core;
mod edit;
mod editor_config;
mod helpers;
mod hex;
mod modes;
mod mouse;
mod recovery;
mod render;
mod run;
mod template;
pub(crate) mod terminal;
mod types;
mod unicode;

// ── re-exports for sibling modules ──────────────────────────────────────────

pub(crate) use completion::CompletionItem;
pub(crate) use types::{
    CursorPos, Editor, EditorEncoding, Focus, HexViewer, Mode, Snapshot, Tab, ALL_ENCODINGS,
    GOTO_LINE, MIN_SIDEBAR_WIDTH,
};

pub(crate) use unicode::{
    auto_pair, byte_to_col, char_col_width, ci_find, col_to_byte, is_ctrl_char, next_char_byte,
    next_word_byte, prev_char_byte, prev_word_byte,
};

pub(crate) use helpers::{gutter_width, prompt_yes_no, sel_range, set_status_bar};
pub(crate) use core::save_lines_to;

// ── public API ──────────────────────────────────────────────────────────────

pub use template::{
    edit_file, edit_file_with_session, generate_super_template, generate_template,
    generate_welcome_content, init_file, init_hyper_project, init_super_file,
};
