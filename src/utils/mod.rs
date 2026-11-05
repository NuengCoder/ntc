// src/utils/mod.rs
// ntc utility modules — all core library modules that were previously top-level .rs files

pub mod ansi_text;
pub mod backup;
pub mod backup_diff;
pub mod backup_manifest;
pub mod config;
pub mod explorer;
pub mod filetype;
pub mod frame_log;
pub mod fuzzy;
pub mod git;
pub mod interactive;
pub mod navigator;
pub mod os_env;
pub mod output;
pub mod paths;
pub mod qsc;
pub mod search;
pub mod serve_info;
pub mod session;
pub mod teleport;
pub mod terminal;
#[cfg(test)]
pub mod test_support;
pub mod theme;
pub mod watcher;
