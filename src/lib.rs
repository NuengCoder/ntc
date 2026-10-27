// src/lib.rs
// ntc core library
// Version: v2.3.0

// Top-level modules (multi-file directories)
pub mod cli;
pub mod deadlock;
pub mod editor;
pub mod fake;
pub mod image2doc;
pub mod lsp;
pub mod math;
pub mod mql;
pub mod ndb;
pub mod netscan;
pub mod nfe;
pub mod nms;
pub mod nsheet;
pub mod nssh;
pub mod ntpkg;
pub mod ranfile;
pub mod report;
pub mod script;
pub mod shell;
pub mod syntax;
pub mod tutorial;
pub mod utils;

// Re-export utils modules so existing paths like `crate::config::Config` still work
// (consumed internally via `use crate::...` and externally via `use ntc::...`).
// The flat re-exports are kept as a compatibility layer; the canonical home for
// each module is `crate::utils::<name>`.
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
pub use utils::session;
pub use utils::teleport;
pub use utils::watcher;

// Re-export key types for convenience
pub use config::Config;
pub use navigator::Navigator;
pub use shell::run_shell_with_nav;
pub use teleport::TeleportManager;
