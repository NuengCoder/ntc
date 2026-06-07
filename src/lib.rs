// src/lib.rs
// ntc core library
// Version: v1.9.0

// Top-level modules (multi-file directories)
pub mod cli;
pub mod editor;
pub mod report;
pub mod shell;
pub mod syntax;
pub mod utils;

// Re-export utils modules so existing paths like `crate::config::Config` still work
pub use utils::backup;
pub use utils::backup_diff;
pub use utils::backup_manifest;
pub use utils::config;
pub use utils::explorer;
pub use utils::filetype;
pub use utils::fuzzy;
pub use utils::navigator;
pub use utils::output;
pub use utils::search;
pub use utils::teleport;
pub use utils::watcher;

// Re-export key types for convenience
pub use config::Config;
pub use navigator::Navigator;
pub use teleport::TeleportManager;
pub use shell::run_shell_with_nav;