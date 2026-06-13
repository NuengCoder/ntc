# Changelog

## [2.2.0] - 2026-06-13

### Added

#### 🎯 ntcEditor — Major Enhancements

**Multi-Tab System**
- Full multi-tab editor with tab bar, tab switching (Ctrl+PgUp/PgDn), and tab close buttons (✕)
- Scratch buffer auto-open — closing the last tab creates a new scratch instead of quitting
- Buffer switching via Ctrl+Tab / Ctrl+Shift+Tab (recently opened files stack)
- Crash recovery with periodic SHA-256-hashed snapshots and auto-recovery prompt
- Persistent editor sessions — cursor position, scroll, buffer stack restored on re-entry
- File finder (Ctrl+P / Ctrl+T) — fuzzy file search across project, opens in new tab

**Editor Commands & Run Mode**
- Command mode (`:q`, `:w`, `:wq`, `:q!`) for quitting/saving
- `:gs <pattern>` — grep file contents across project, results navigable via file finder
- `:run <command>` — execute shell commands with live streaming output in split panel
- `:gosc` — continuous subdirectory navigation within editor

**Syntax Highlighting — 18 New Languages**
- **Inno Setup (.iss)**: Pascal keywords, section headers, preprocessor directives, 80+ InnoSetup-specific properties/functions, `'…'` strings, `//` / `{…}` / `(*…*)` comments
- **Lock files (.lock)**: TOML-like `#` comments, `[[…]]` headers, key=value, `true`/`false` keywords
- Full syntax highlighting now covers 45+ languages with dedicated TokenType::Constant (gold)

**Editor UI/UX**
- Sidebar file explorer (Ctrl+B) with expand/collapse folders, directory icons (▶/▼)
- Sidebar vertical scroll bar with `█`/`░` thumb/track at rightmost column — click-to-jump and mouse wheel support
- Multi-cursor editing: Ctrl+D to add cursor at next occurrence, Ctrl+G to jump through matches
- Editor config (`editor.toml`) with `auto_save`, `syntax_enabled`, `color_enabled` settings
- Auto-completion popup for `.ntc.math` files with Tab/Enter/Esc navigation

#### 🔍 Wildcard File/Directory Search
- `fs`, `ds`, `fsc`, `fgo`, `locate` now support wildcard patterns (`*`, `%`, `_`, `$`)
- `*` or `%` matches any sequence, `_` matches single char, `$` anchors at end
- Examples: `fs *.rs`, `ds src$`, `fs main_*`
- When wildcards detected, uses glob-like matching instead of exact/partial/fuzzy tiers

#### 📄 New `gs` Command — Grep-Like Content Search
- `gs <pattern> [-d <n>]` — search file contents recursively with case-insensitive matching
- Results show file path, line number, and truncated matching line
- Respects ignore/care settings and supported file types

#### 🔧 `ran` — Make-like Native Task Runner
- `ran <target>` — execute targets with dependency resolution from `NTCRANFILE.toml`
- `ran init` / `ran deinit` / `ran list` / `ran help` — subcommand management
- `ran help` shows template and usage docs
- Topological sort for dependency chains with cycle detection
- Variable expansion via `$(VAR_NAME)` syntax
- Rich TOML template with comments and examples
- Integrated with top-level `init`/`deinit` commands

#### 🎨 Fully Configurable Theme System
- **1110-line theme engine** with `ThemeManager` singleton for live theme switching
- `theme` command: interactive menu + direct mode (`list`, `current`, `<name>`, `add`, `rm`, `edit`, `info`, `export`, `import`, `rnm`, `reload`)
- **15 syntax colors**: keyword, string, comment, number, type, builtin, function, operator, punctuation, attribute, macro_token, regex, tag, constant, normal
- **20 shell colors**: prompt, success/error/warning/info, tree, separator, help, teleport, alias, command_output
- **26 editor colors**: backgrounds (editor, gutter, status, hint, sidebar, run_panel), text, cursor, selection, search matches, border, scrollbar, sidebar items
- **3 built-in themes**: `default` (dark), `light`, and `example` template
- Theme export/import to `.ntc_theme` files
- Live theme switching — instant redraw without restart

#### ⚙️ Configuration & New Commands
- **`init`** — creates `NTCRANFILE.toml` + `ntconfig.toml` with `--ran`, `--local`, `--all` flags
- **`deinit`** — removes project files with per-file prompts
- **`setc ON|OFF`** — toggle color output
- **`seta ON|OFF`** — toggle autosuggest ghost text
- **`watch` enhanced** — `watch trigger <alias>` and `watch trigger off` for auto-run
- **`size --care`** — calculate total directory size including ignored dirs
- **`tp` extended** — added `info`, `rnm`, `cls`, `help` subcommands
- **`tpb`** — teleport back with history (undo last teleport)
- **Export/Import** for run aliases (`ral export --all/--select`, `ral import`) and ignore/care settings (`igcare export --all/--select`, `igcare import`)
- **`ne --init <file>`** — create template file by extension before opening in editor

### Fixed
- **Tab close button (✕) click detection**: off-by-1 offset per tab due to `full_w` = `label_chars + 2` vs actual rendered width `label_chars + 1` — each tab's close zone now aligns correctly
- **Truncated tabs** no longer expose phantom clickable close areas
- **RwLock deadlock** in config system where write-holders attempted to acquire read lock
- **Windows extended-path prefix** (`\\?\`) stripped in all search display outputs
- **Duplicate filenames** in different directories now all appear in search results
- **Search depth** correctly starts from CWD and never goes above current directory
- **Depth 0** search now returns current directory only (no recursion)

### Changed
- **Shell architecture** completely refactored: monolithic `shell.rs` split into modular `commands/` (9 submodules), `alias/`, `helpers.rs`, `entry.rs`
- **Command structure**: `run`/`ral` consolidated into `run`/`ran`/`ral` — three distinct systems (alias execution, task runner, alias management)
- **`Ctrl+Q`** now calls `close_tab()` which handles scratch auto-open automatically
- **File watcher** completely re-architected: ignore-aware, debounced batching (400ms), filetype-aware event classification, clean `poll()` API
- **Search engine**: tiered matching (exact > partial > fuzzy, never mixed), Jaro-Winkler fuzzy fallback, respects all ignore/care settings
- **Config system**: local `ntconfig.toml` support, extra_supported_files/extensions, watch_trigger_alias, `validate_alias_name()` now covers all reserved names
- **LSP server**: enhanced hover documentation for built-in functions and user-defined functions
- Reserved commands list updated: `gs`, `fs`, `ds`, `fgo`, `fsc`, `locate`, `dino`, `math`, `init`, `deinit`

### Performance & Maintainability
- Shell module refactored into clean submodule structure for better maintainability
- File watcher with debounced event batching reduces CPU usage
- Search engine optimized to never mix match tiers, reducing false positives
- Crash recovery system with atomic snapshots prevents data loss
- Theme system with atomic `THEME_CHANGED` flag avoids unnecessary redraws
- `has_pascal_comments()` / `has_preprocessor()` extension points in syntax system
- Pascal comment state tracking in SyntaxHighlighter
- 103 unit tests pass cleanly with zero warnings

## [2.1.0] - 2026-06-07

### Added

#### Math Expression Evaluator
- `math <expr>` — Evaluate math expressions with built-in functions and constants
- `math timer [sec]` — Lap timer or countdown with alarm
- `math fun add/edit/rm/info/ls` — Manage user-defined math functions
- `math <file>.ntc.math` — Run .ntc.math script files
- Built-in functions: sin, cos, tan, and all trig (including arc/ inverse variants)
- Math functions: sqrt, pow, abs, floor, ceil, round, ln/log, log2, log10
- Aggregate: sum, min, max, avg/average/mean
- Random: rand(min, max)
- Conversions: toBinary, toHex, toOctal, toDecimal, toHumanBytes
- Constants: PI, E, PHI, TAU
- String literals with `"..."` support and escape sequences
- `print()` function — works like Python's print (accepts strings and numbers)
- `return` keyword for file-mode scripts
- `--math <EXPR>` flag for command-line evaluation

#### Run Alias (RAL) Export/Import
- `ral export --all <name>` — Export all aliases to `<name>.ntc.ral`
- `ral export --select <name>` — Interactive pick & export to `.ntc.ral`
- `ral import <file.ntc.ral>` — Import aliases from `.ntc.ral` file

#### Ignore/Care (IGCARE) Export/Import
- `igcare export --all <name>` — Export all ignore/care settings to `<name>.ntc.igcare`
- `igcare export --select <name>` — Interactive category pick & export to `.ntc.igcare`
- `igcare import <file.ntc.igcare>` — Import settings from `.ntc.igcare` file

#### Dinosaur Runner Game
- `dino` — Play a Chrome-style dinosaur runner game in the terminal
- Jump with space/up to avoid obstacles
- Score tracking and persistent high score (saved to `dino.toml`)
- Game over detection and restart prompt

#### ntcEditor — In-editor Auto-Completion
- Auto-completion popup for .ntc.math files
- Tab/Enter/Esc to navigate and apply completions
- Completions include all built-in functions, constants, user-defined functions, `print()`, and `return`
- Smart popup positioning near cursor

#### ntcEditor — LSP Server
- Built-in LSP server for .ntc.math files (`--lsp` flag)
- Diagnostics (syntax errors with position)
- Completions (functions, constants, user-defined)
- Hover documentation
- Go-to-definition for user-defined functions

#### ntcEditor — NtcMath Syntax Highlighting
- Full syntax highlighting for .ntc.math files
- Dedicated TokenType::Constant (gold) for PI, E, PHI, TAU
- Keywords (return, true, false) and built-in functions highlighted

### Changed
- `view`, `txt`, `pdf`, `docx`, `xlsx` report modes now also support .ntc.ral and .ntc.igcare files
- Internal module structure refactored for extensibility
- Help documentation updated with all new commands (dino, math, igcare, ral export/import)

### Technical Improvements
- String token support in math tokenizer (Token::Str, Expr::Str) with escape sequences
- Byte-offset tracking in math tokenizer for precise LSP diagnostics
- Grammar validation function (`validate()`) used by LSP
- Completion engine with exact-match-first + alphabetical sorting
- Manual JSON-RPC LSP implementation without external crate dependencies

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