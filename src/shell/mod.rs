// src/shell/mod.rs
// ntc shell module — split from the original single-file shell.rs

mod alias;
pub(crate) mod commands;
pub(crate) mod entry;
pub(crate) mod helpers;
mod help;

// Re-exports for external callers (lib.rs / main.rs)
pub use entry::run_shell;
pub use entry::run_shell_with_nav;
pub(crate) use helpers::show_tree;