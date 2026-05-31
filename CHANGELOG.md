# Changelog

## [1.8.0] - 2026-05-31

### Added

#### Backup & Restore System
- `bkup` - Create full project backup (respects ignore/care filters, skips files >50MB)
- `bkup --where` / `-w` - Show backup storage location (~/.ntc/backups/)
- `bkup --cls` - Delete all backups for current project (with confirmation)
- `bkup --force` - Delete all backups without confirmation (for scripting)
- `pldw` - Interactive restore menu showing numbered backups with date and size
- `pldw <number>` - Restore backup by number directly
- `unpd` - Undo last restore (restores overwritten files, deletes newly created ones)
- `unpd --cls` - Clear undo history (with confirmation)
- `unpd --force` - Clear undo history without confirmation
- Atomic backup creation (temp directory + atomic rename) - no partial backups
- `ignore.txt` generation - detailed report of skipped files with reasons (too_large, ignored_by_config, ignored_by_user)

#### File & Directory Search
- `fs <pattern>` - Search files with exact → partial → fuzzy matching (case-insensitive)
- `fs <pattern> -d <n>` - Search files with custom depth (overrides setD config)
- `ds <pattern>` - Search directories with exact → partial → fuzzy matching
- `ds <pattern> -d <n>` - Search directories with custom depth
- Jaro-Winkler fuzzy matching with thresholds (0.75 for files, 0.72 for directories)
- Top 3 fuzzy suggestions when no exact or partial match found

#### View Command Enhancements
- `view -s` / `view --size` - Show directory tree with folder sizes
- `view -d <n>` / `view --depth <n>` - Show tree with custom depth (overrides setD)
- `view -s -d <n>` - Combine both size display and custom depth

#### Teleport & Navigation
- `go to <tp_name>` - Teleport to savepoint directly from go command
- `cd to <tp_name>` - Teleport to savepoint directly from cd command
- `tp to <name>` - Same as `tp jump <name>` (alias for consistency)

#### Run Aliases (RAL) Enhancements
- Multi-argument parameterised aliases: `ral add runc(x,y) "gcc -o $y $x.c && ./$y"`
- Support for `$x`, `$y`, `$z`, etc. placeholders (any alphanumeric names)
- Example: `runc(main,program)` expands to `gcc -o program main.c && ./program`

#### Platform Support
- macOS Universal Binary (Intel + Apple Silicon)
- Termux (Android) full support
- Cross-platform backup/restore (Windows, Linux, macOS, Termux)

#### Installer Improvements (Windows)
- Optional desktop shortcut during installation
- Improved PATH handling

### Changed
- View command now accepts flags (`-s`, `--size`, `-d`, `--depth`) instead of just `view --size`
- Reserved commands list updated: added `bkup`, `pldw`, `unpd`, `fs`, `ds`
- Help documentation (`help` command) updated with all new commands

### Fixed
- Search now correctly starts from CWD (never goes above current directory)
- Depth 0 handling for search (current directory only, no recursion)
- Duplicate filename handling in search results (multiple files with same name in different directories now all appear)
- Windows extended path prefix (`\\?\`) stripped in all display outputs
- Atomic backup prevents partial backups on process interruption
- `strip_prefix` error handling for broken symlinks

### Technical Improvements
- Typed `SkipReason` enum (prevents string typos)
- `new_files` tracking in manifest (enables proper undo of newly created files)
- `copy_and_hash()` single-pass file copy + hashing (performance improvement)
- SHA256 hashing for file integrity (prepares for future diff features)
- `BackupIndex.summary_key()` with `#` separator (unambiguous project hash + backup number)
- `--force` flag for non-interactive clearing (bkup --force, unpd --force)

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

## [1.5.0] - 2026-05-20

### Added
- Teleport savepoints (`tp add`, `tp jump`, `tp list`, `tp rm`, `tp cls`)
- Run aliases (`ral add`, `ral edit`, `ral rm`, `ral list`)
- `@<name>` quick teleport shortcut
- `run` / `r` command for executing system commands
- `&&` chaining for multiple commands

## [1.4.0] - 2026-05-17

### Added
- `gos` - List subdirectories and pick one interactively
- `gosc` - Continuous navigation mode (0 to exit)

## [1.3.0] - 2026-05-14

### Added
- Ignore/care system for directories (`ignore`, `cared`)
- Ignore/care for file extensions (`ignoref`, `caref`)
- Ignore/care for specific files (`ignoren`, `caren`)
- `ignored` command to show all ignored items

## [1.2.0] - 2026-05-11

### Added
- HTML report generation (`html` command)
- Better error handling and user feedback

### Fixed
- Various bugs from previous version

## [1.1.0] - 2026-05-08

### Added
- `back` command with support for `back <n>` (go back n levels)
- Configuration commands (`setO`, `setD`, `setL`, `setT`, `setH`)
- `where` command to show locations
- `clear` command

## [1.0.0] - 2026-05-01

### Added
- Initial release
- Interactive directory navigation (`go`, `cd`)
- Directory tree viewer (`view`)
- TXT report generation (`txt`)
- File content viewing (`txtc`, `txtf`)

---

For more information: https://github.com/NuengCoder/ntc