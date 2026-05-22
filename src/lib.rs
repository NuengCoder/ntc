pub mod config;
pub mod filetype;
pub mod navigator;
pub mod explorer;
pub mod report;
pub mod output;
pub mod cli;
pub mod shell;
pub mod watcher;
pub mod teleport;

// Re-export key types for convenience
pub use navigator::Navigator;
pub use config::Config;
pub use teleport::TeleportManager;
pub use shell::run_shell_with_nav;