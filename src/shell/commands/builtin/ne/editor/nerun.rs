use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Editable run configuration, stored at `{config_dir}/ntc/nerun.toml`.
///
/// `:run` (bare) resolves the active tab's file extension against
/// `[[runner]]` entries and executes the mapped command in the embedded
/// terminal panel. The file is user-editable; no code rebuild needed.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NeRunConfig {
    #[serde(default)]
    pub options: NeRunOptions,
    #[serde(default)]
    pub runner: Vec<NeRunner>,
    #[serde(default)]
    pub fallback: Option<NeFallback>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeRunOptions {
    /// Save the buffer before running (default: true).
    #[serde(default = "default_true")]
    pub save_before_run: bool,
    /// Echo the resolved command into the terminal as typed input
    /// (default: true). The shell itself echoes it; this flag is kept
    /// so users can disable the extra `> cmd` status line if desired.
    #[serde(default = "default_true")]
    pub echo_command: bool,
    // NOTE: the embedded terminal always runs the ntc shell
    // (`TerminalState::spawn_ntc_shell`); there is intentionally no
    // `shell` option. A stale `shell = ...` key in an existing
    // nerun.toml is ignored by serde.
}

impl Default for NeRunOptions {
    fn default() -> Self {
        Self {
            save_before_run: true,
            echo_command: true,
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeRunner {
    /// Human-readable name shown in the status bar.
    #[serde(default)]
    pub name: String,
    /// File extensions (without dot, case-insensitive) this runner handles.
    pub extensions: Vec<String>,
    /// Command template. Placeholders:
    /// `$CWF` / `$FILE` = absolute current working file,
    /// `$DIR` = parent directory, `$NAME` = file stem, `$EXT` = extension.
    pub command: String,
    /// Working directory template (default: `$DIR`). Placeholders expanded.
    #[serde(default)]
    pub cwd: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeFallback {
    pub command: String,
}

/// Fully resolved command ready to send to the terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRun {
    pub name: String,
    pub command: String,
    pub cwd: PathBuf,
}

impl NeRunConfig {
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("ntc").join("nerun.toml"))
    }

    /// Load from disk, or return defaults if missing/unparseable.
    /// Missing file is auto-created with an editable MetaEditor example.
    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(cfg) = toml::from_str::<Self>(&content) {
                        return cfg;
                    }
                }
            } else {
                let cfg = Self::default_config();
                Self::write_default(&path, &cfg);
                return cfg;
            }
        }
        Self::default_config()
    }

    fn default_config() -> Self {
        Self {
            options: NeRunOptions::default(),
            runner: vec![
                NeRunner {
                    name: "mql4".into(),
                    extensions: vec!["mq4".into(), "mqh".into()],
                    command: "\"C:\\Program Files (x86)\\Forex4you MetaTrader 4\\metaeditor.exe\" /compile:\"$CWF\""
                        .into(),
                    cwd: Some("$DIR".into()),
                },
                NeRunner {
                    name: "mql5".into(),
                    extensions: vec!["mq5".into()],
                    command: "\"C:\\Program Files (x86)\\Forex4you MetaTrader 4\\metaeditor.exe\" /compile:\"$CWF\""
                        .into(),
                    cwd: Some("$DIR".into()),
                },
            ],
            fallback: None,
        }
    }

    fn write_default(path: &Path, cfg: &Self) {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let header = "# nerun.toml — editable :run configuration for ntcEditor.\n\
                      # `:run` compiles the current working file ($CWF) using the\n\
                      # first [[runner]] whose `extensions` match the file extension.\n\
                      # Placeholders: $CWF / $FILE (absolute file), $DIR (parent),\n\
                      # $NAME (stem), $EXT (extension without dot).\n\
                      # Output goes to the embedded terminal panel.\n\n";
        if let Ok(body) = toml::to_string_pretty(cfg) {
            let _ = fs::write(path, format!("{header}{body}"));
        }
    }

    /// Find the first runner handling `ext` (case-insensitive, dot optional).
    pub fn find_for_extension(&self, ext: &str) -> Option<&NeRunner> {
        let want = ext.trim().trim_start_matches('.').to_lowercase();
        if want.is_empty() {
            return None;
        }
        self.runner.iter().find(|r| {
            r.extensions
                .iter()
                .any(|e| e.trim().trim_start_matches('.').to_lowercase() == want)
        })
    }

    /// Resolve the command + cwd for `file_path`, or `None` if no runner
    /// matches (caller falls back to `fallback` / error message).
    pub fn resolve_for(&self, file_path: &Path) -> Option<ResolvedRun> {
        let ext = file_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default();
        let runner = self.find_for_extension(ext)?;
        let command = expand_placeholders(&runner.command, file_path);
        let cwd_raw = runner.cwd.as_deref().unwrap_or("$DIR");
        let cwd_expanded = expand_placeholders(cwd_raw, file_path);
        let cwd = {
            let p = PathBuf::from(&cwd_expanded);
            if p.is_absolute() {
                p
            } else if let Some(parent) = absolute_file(file_path).parent() {
                parent.join(&p)
            } else {
                std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
            }
        };
        // Guard against empty cwd (e.g. scratch file without parent).
        let cwd = if cwd.as_os_str().is_empty() {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        } else {
            cwd
        };
        Some(ResolvedRun {
            name: runner.name.clone(),
            command,
            cwd,
        })
    }

    /// Resolve with `fallback` support: extension runner first,
    /// then `[fallback]` template, else `None`.
    pub fn resolve_with_fallback(&self, file_path: &Path) -> Option<ResolvedRun> {
        if let Some(r) = self.resolve_for(file_path) {
            return Some(r);
        }
        let fb = self.fallback.as_ref()?;
        let cwd_raw = "$DIR";
        Some(ResolvedRun {
            name: "fallback".into(),
            command: expand_placeholders(&fb.command, file_path),
            cwd: PathBuf::from(expand_placeholders(cwd_raw, file_path)),
        })
    }
}

/// Absolute form of `file_path` (join with process CWD when relative).
pub fn absolute_file(file_path: &Path) -> PathBuf {
    if file_path.is_absolute() {
        file_path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(file_path)
    }
}

/// Expand `$CWF` / `$FILE` / `$DIR` / `$NAME` / `$EXT` in `template`.
/// Unknown placeholders are left untouched.
pub fn expand_placeholders(template: &str, file_path: &Path) -> String {
    let abs = absolute_file(file_path);
    let file = abs.to_string_lossy().to_string();
    let dir = abs
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let name = abs
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let ext = abs
        .extension()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    // Longest tokens first to avoid prefix collisions.
    template
        .replace("$CWF", &file)
        .replace("$FILE", &file)
        .replace("$DIR", &dir)
        .replace("$NAME", &name)
        .replace("$EXT", &ext)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> NeRunConfig {
        NeRunConfig::default_config()
    }

    #[test]
    fn matches_mq4_case_insensitive() {
        let c = cfg();
        assert!(c.find_for_extension("mq4").is_some());
        assert!(c.find_for_extension("MQ4").is_some());
        assert!(c.find_for_extension(".mq5").is_some());
        assert!(c.find_for_extension("rs").is_none());
        assert!(c.find_for_extension("").is_none());
    }

    #[test]
    fn expands_cwf_and_dir() {
        // Platform-independent: absolute_file() joins relative paths with
        // the process CWD (on Windows `/tmp/...` is not absolute).
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let path = cwd.join("bot").join("EA.mq4");
        let out = expand_placeholders("\"$CWF\" in $DIR name=$NAME ext=$EXT", &path);
        let file = path.to_string_lossy().to_string();
        let dir = path.parent().unwrap().to_string_lossy().to_string();
        assert_eq!(out, format!("\"{file}\" in {dir} name=EA ext=mq4"));
    }

    #[test]
    fn file_alias_matches_cwf() {
        let path = Path::new("/tmp/a.mq4");
        assert_eq!(
            expand_placeholders("$FILE", path),
            expand_placeholders("$CWF", path)
        );
    }

    #[test]
    fn resolve_keeps_quoted_metaeditor_path() {
        let c = cfg();
        let r = c
            .resolve_for(Path::new("C:/exa/Trailing.mq4"))
            .expect("mq4 runner");
        assert_eq!(r.name, "mql4");
        assert!(r.command.contains("metaeditor.exe"), "{r:?}");
        assert!(r.command.contains("Trailing.mq4"), "{r:?}");
    }

    #[test]
    fn unknown_extension_resolves_none() {
        let c = cfg();
        assert!(c.resolve_for(Path::new("/tmp/main.rs")).is_none());
    }
}
