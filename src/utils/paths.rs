// src/utils/paths.rs
// Shared locations for files ntc owns under the platform config directory.

use std::path::PathBuf;

/// Environment variable that overrides the platform config root.
///
/// It exists mainly so tests can point every on-disk store at a tempdir: on
/// Windows `dirs::config_dir()` calls the Known Folder API and ignores both
/// `APPDATA` and `XDG_CONFIG_HOME`, so env-based isolation silently failed.
/// It doubles as a portable-install escape hatch.
pub const CONFIG_HOME_ENV: &str = "NTC_CONFIG_HOME";

/// Platform config root, honoring [`CONFIG_HOME_ENV`].
pub fn config_home() -> PathBuf {
    std::env::var_os(CONFIG_HOME_ENV)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .or_else(dirs::config_dir)
        .unwrap_or_else(|| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_home_honors_the_override() {
        let _home = crate::utils::test_support::IsolatedConfigHome::new();
        assert_eq!(
            config_home(),
            std::env::var_os(CONFIG_HOME_ENV)
                .map(PathBuf::from)
                .unwrap()
        );
    }
}
