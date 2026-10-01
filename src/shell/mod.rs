// src/shell/mod.rs
// ntc shell module — split from the original single-file shell.rs

pub(crate) mod alias;
pub(crate) mod commands;
pub(crate) mod entry;
pub(crate) mod help;
pub(crate) mod helpers;
mod hinter;
pub(crate) mod venv;

// Re-exports for external callers (lib.rs / main.rs)
pub use entry::run_shell;
pub use entry::run_shell_with_nav;
pub(crate) use helpers::show_tree;
pub use venv::detect_active_venv;
