# Changelog

## [1.7.0] - 2026-05-26

### Added
- Local project configuration (`ntconfig.toml`) for per-directory settings
- `opencg` - Open config file in default editor (respects $EDITOR)
- `resetcg` - Reset config to defaults with backup option
- `restorecg` - Restore config from backup files
- `gencg` - Create ntconfig.toml template
- `gencg --all` - Export current settings to ntconfig.toml
- `ral cls` - Clear all run aliases (with confirmation)
- Recursive alias expansion (up to 5 levels deep)
- Support for `&&` inside aliases (use quotes: `ral add fb "dal && frr"`)
- Smart CRUD - ignore/care and ral commands auto-detect local vs global config

### Changed
- Config merging: ntconfig.toml only overrides ignore/care and run_aliases
- Teleports always global (never overridden by local config)
- `ignored` and `ral list` now show config source (local/global)
- Improved TOML formatting for exported configs (proper arrays with `[]`)

### Fixed
- Alias expansion now works with nested aliases and `&&` chains
- `run` command properly expands aliases before execution
- `gencg` produces valid TOML syntax (no `{:?}` debug output)

## [1.6.0] - 2026-05-23
### Added
- JSON report generation (`json` command)
- Markdown report generation (`md` command)
- Modern HTML reports with better styling
- Clipboard copy support (`--cp` flag) for txt, json, md reports
- Linux/WSL support with .deb and .tar.gz packages
- `where` command shows executable and config file locations
- Config file path display

### Fixed
- Run aliases now work without `run` prefix (e.g., `py` instead of `run py`)
- Improved cross-platform compatibility

### Changed
- Enhanced help documentation
- Better error messages for unsupported operations

## [1.5.0] - Previous Release
- Initial TP (teleport) features
- Basic navigation
- TXT reports