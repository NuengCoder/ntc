// src/lib.rs
// ntc core library
// Version: v1.8.0
pub mod backup;
pub mod backup_manifest;
pub mod cli;
pub mod config;
pub mod explorer;
pub mod filetype;
pub mod fuzzy;
pub mod navigator;
pub mod output;
pub mod report;
pub mod search;
pub mod shell;
pub mod teleport;
pub mod watcher;

// Neovim/Android helpers (conditional compilation handled inside)
pub mod nvim;

// Re-export key types for convenience
pub use config::Config;
pub use navigator::Navigator;
pub use teleport::TeleportManager;
pub use shell::run_shell_with_nav;