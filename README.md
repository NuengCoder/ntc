<p align="center">
  <img src="https://img.shields.io/badge/version-2.2.0-blue.svg" alt="Version 2.2.0"/>
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS%20%7C%20Android-lightgrey.svg" alt="Platforms"/>
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License"/>
</p>

<h1 align="center">ntc — Navigate, Toolkit, Center</h1>

<p align="center">
  <b>All-in-one terminal productivity tool</b><br>
  Navigation · Search · Backup · Editor · Reports · Math · Task Runner · Themes
</p>

---

## Demo

<p align="center">
  <img src="assets/nice_gen.gif" alt="ntc demo GIF" width="700"/>
</p>

<p align="center">
  <i>
    <a href="https://github.com/NuengCoder/ntc">GitHub</a> ·
    <a href="https://github.com/NuengCoder/ntc/releases">Releases</a>
  </i>
</p>

---

## Features at a Glance

| Category | Capabilities |
|----------|-------------|
| **Navigation** | `go`/`cd`, `back`, `gos`/`gosc`, `godrive`, teleport savepoints (`tp`), teleport back (`tpb`) |
| **File Search** | `fs` (wildcards + fuzzy + exact + partial), `ds` (dirs), `locate`, `gs` (grep), `fgo`/`fsc` |
| **Backup** | `bkup` (atomic snapshots), `pldw` (restore), `unpd` (undo), `diff` |
| **Editor** | Multi-tab, 45+ language syntax highlighting, sidebar, mouse support, undo/redo, multi-cursor, search, LSP, auto-completion, run mode, crash recovery |
| **Task Runner** | `ran` — Make-like dependency-based task runner with `NTCRANFILE.toml` |
| **Reports** | TXT, JSON, MD, HTML, PDF, DOCX, XLSX |
| **Math** | Expression evaluator, built-in functions, user functions, timer, `.ntc.math` scripts |
| **Aliases** | `ral` (parameterised, array args, defaults), `&&` chaining, recursive expansion, export/import |
| **Config** | `setO`/`setD`/`setL`/`setC`/`setA`/`setT`/`setH`, local `ntconfig.toml`, `init`/`deinit` |
| **Themes** | Full theme system — syntax, shell, editor colors. Export/import `.ntc_theme` files |
| **Watcher** | File watcher with debounced events, ignore-aware, trigger alias auto-run |
| **Filters** | `ignore`/`cared` (dirs, extensions, filenames), export/import `.ntc.igcare` |
| **Fun** | Dinosaur runner game (`dino`) |
| **LSP** | Built-in LSP server for `.ntc.math` files (hover, completions, diagnostics, go-to-definition) |

---

## Quick Demo

```bash
# Navigation with tree view
go /path/to/project
view -s -d 3                   # Tree with sizes, depth 3

# Wildcard file search
fs *.rs                        # All Rust files
ds src$                        # Directories ending with "src"
gs "TODO"                      # Grep file contents

# Built-in text editor with syntax highlighting (45+ langs)
ne file.iss                    # Inno Setup script
ne file.lock                   # Lock file
ne --init main.rs              # Create Rust file from template

# Editor mouse features
# - Click tab ✕ to close tabs
# - Ctrl+Click / middle-click sidebar scroll bar to jump
# - Scroll wheel over sidebar to scroll

# Multi-tab editor
ne file1.rs                    # Opens in tab 1
ne file2.py                    # Opens in tab 2
Ctrl+Tab                       # Switch between tabs

# Backup & diff
bkup
# ... make changes ...
diff 1                         # Show diff vs backup #1

# Task runner (Make-like)
ran init                       # Create NTCRANFILE.toml template
ran build                      # Run "build" target with deps
ran list                       # Show all targets

# Math expression evaluator
math 3+4*5                     # → 23
math sin(PI/2)                 # → 1
math script.ntc.math           # Run .ntc.math file

# Dinosaur runner game
dino                           # Press space to jump

# Theme management
theme list                     # List available themes
theme default                  # Switch to default theme
theme add mydark               # Create new theme in editor

# Project config
init                           # Create NTCRANFILE.toml + ntconfig.toml
deinit                         # Remove project config files

# Reports
pdf
docx
xlsx

# Multi-argument alias
ral add runc(x,y) "gcc -o $y $x.c && ./$y"
runc(main,program)
```

---

## Installation

| Platform | Method |
|----------|--------|
| **Windows** | Inno Setup installer, manual `.exe` |
| **Linux (Debian)** | `sudo dpkg -i ntc_2.2.0-1_amd64.deb` |
| **Linux (other)** | Extract `ntc-v2.2.0-linux-x86_64.tar.gz` to PATH |
| **macOS** | Extract `ntc-v2.2.0-macos-universal.tar.gz` to PATH |
| **Termux** | Extract `ntc-v2.2.0-aarch64-linux-android.tar.gz` to `$PREFIX/bin/` |
| **Any (npm)** | `npm install -g @nuengcoder/ntc` |

---

## Full Command Reference

### 1. Navigation (Interactive Mode)

| Command | Description |
|---------|-------------|
| `go <path>` / `cd <path>` | Change to a directory |
| `go to <tp_name>` / `cd to <tp_name>` | Teleport to a saved teleport point |
| `back` | Go to parent directory |
| `back <n>` | Go up n levels |
| `godrive` | List and select Windows drives (Windows only) |
| `gos` | Show numbered list of subdirectories to choose from |
| `gosc` | Continuous navigation mode (0 to exit) |
| `@<name>` | Quick teleport shortcut |

**Teleport Savepoints:**

| Command | Description |
|---------|-------------|
| `tp add <name>` | Save current directory |
| `tp add <name> <path>` | Save specific path |
| `tp jump <name>` / `tp to <name>` | Teleport by name |
| `tp jump <number>` | Teleport by number (from list) |
| `tp list` | Show all savepoints |
| `tp rm <name>` / `tp rm <number>` | Remove a savepoint |
| `tp cls` | Clear ALL savepoints (asks confirmation) |
| `tp info <name>` | Show savepoint details |
| `tp rnm <old> to <new>` | Rename a savepoint |
| `tp help` | Show teleport help |
| `tpb` | Teleport back to previous location (undo last tp) |
| `tpb history` | Show teleport navigation history |
| `tpb clear` | Clear teleport history |

### 2. Directory Viewing

| Command | Description |
|---------|-------------|
| `view` | Show directory tree (uses configured depth) |
| `view -s` / `view --size` | Show tree with folder sizes |
| `view -d <n>` / `view --depth <n>` | Show tree with custom depth (1-20) |
| `view -s -d <n>` | Show tree with sizes and custom depth |
| `size` | Show total size of current directory |
| `size --care` | Calculate total directory size including ignored dirs |

### 3. File & Directory Search

**Search files:** `fs <pattern> [-d <n>]` — Exact → Partial → Fuzzy matching (case-insensitive).  
**Search directories:** `ds <pattern> [-d <n>]`  
**Combined search:** `locate <pattern> [-d <n>]` — Both files and directories.  
**Search + navigate:** `fgo <pattern> [-d <n>]` — Search, select a file, navigate to its parent directory.  
**Search + display:** `fsc <pattern> [-d <n>]` — Search, select a file, display its contents.  
**Grep content search:** `gs <pattern> [-d <n>]` — Search file contents (case-insensitive substring match). Results show file path, line number, and matching line.

**Wildcard / Glob Support** (fs, ds, locate, fgo, fsc):
- `*` or `%` — matches any sequence of characters (including empty)
- `_` — matches any single character
- `$` — anchors the pattern to the end
- Examples: `fs *.rs`, `ds src$`, `fs main_*`
- When wildcards are detected, exact/partial/fuzzy matching is bypassed

**Matching algorithm:**
1. Wildcard patterns (`*`, `%`, `_`, `$`) — glob-style matching
2. Exact matches (full filename equality, case-insensitive)
3. Partial matches (filename contains the pattern)
4. Fuzzy matches (Jaro-Winkler similarity >=75% for files, >=72% for directories)
5. Top 3 fuzzy suggestions when no exact or partial match found

### 4. Backup & Restore

| Command | Description |
|---------|-------------|
| `bkup` | Create a backup of current project (skips files >50MB, respects ignore/care) |
| `bkup --where` | Show backup storage location (`~/.ntc/backups/`) |
| `bkup --cls` | Delete ALL backups for this project (asks confirmation) |
| `bkup --force` | Delete ALL backups (no confirmation, for scripting) |
| `pldw` | Interactive restore menu (shows numbered backups with date and size) |
| `pldw <number>` | Restore backup by number directly |
| `unpd` | Undo the last restore (restores overwritten files, deletes newly created ones) |
| `unpd --cls` | Clear undo history (asks confirmation) |
| `unpd --force` | Clear undo history (no confirmation) |
| `diff` | Interactive backup diff menu |
| `diff <number>` | Show diff between current state and backup #N |

Features: Atomic backup creation (temp directory + atomic rename), SHA-256 hashing, `ignore.txt` generation with skip reasons (too_large, ignored_by_config, ignored_by_user).

### 5. Task Runner — `ran` (Make-like)

| Command | Description |
|---------|-------------|
| `ran init` | Create NTCRANFILE.toml template |
| `ran init --all` | Create with sample targets |
| `ran <target>` | Execute target with dependency resolution |
| `ran build` | Run build target (auto-runs dependencies first) |
| `ran test` | Run test target |
| `ran list` / `ran ls` | List all targets with dependency chains |
| `ran help` | Show ranfile syntax and examples |
| `ran deinit` | Remove NTCRANFILE.toml |

**NTCRANFILE.toml format:**
```toml
[vars]
CC = "gcc"
FLAGS = "-Wall -O2"

[targets.build]
deps = ["clean"]
cmd = "{{CC}} {{FLAGS}} -o program main.c"

[targets.test]
deps = ["build"]
cmd = "./program --test"
```

Features: Topological sort, cycle detection, variable expansion (`$(VAR_NAME)` / `{{VAR_NAME}}`), alias expansion integration.

### 6. Built-in Text Editor — `ne` (ntcEditor)

| Command | Description |
|---------|-------------|
| `ne` | Open scratch buffer in current directory |
| `ne <file>` | Open file for editing |
| `ne <path>/<file>` | Open file at specific path |
| `ne <dir>` | Open scratch buffer inside directory |
| `ne --init <file>` | Create file from template (supports .rs, .py, .c, etc.) |
| `ntceditor <file>` | Same as `ne` |

**Editor Features:**
- **Multi-tab editing** with tab bar, tab switching (Ctrl+PgUp/PgDn), and buffer stack (Ctrl+Tab)
- **Tab close buttons** (✕) — click any tab's ✕ to close it; closing the last tab creates a scratch buffer
- **Sidebar file explorer** (Ctrl+B) with expand/collapse folders and directory icons (▶/▼)
- **Sidebar vertical scroll bar** — `█`/`░` thumb/track with click-to-jump and mouse wheel support
- **Multi-cursor editing** — Ctrl+D to add cursor at next occurrence, Ctrl+G to jump through matches
- **Undo/redo** with unlimited history (Ctrl+Z / Ctrl+Y / Ctrl+Shift+Z)
- **Mouse support** (click to position cursor, scroll, drag scroll bars)
- **Line numbers** and gutter with configurable width
- **Syntax highlighting** for 45+ languages including Inno Setup (.iss) and lock files (.lock)
- **Clipboard operations** (cut, copy, paste) with multi-line support
- **Word wrap** and horizontal scrolling
- **Auto-indentation** and bracket pairing
- **Search within file** (Ctrl+F) with next/previous navigation (Ctrl+G / Ctrl+Shift+G)
- **`gs <pattern>`** in command mode — grep file contents across project
- **`:run <command>`** — execute shell commands with live streaming output in split panel
- **File finder** (Ctrl+P / Ctrl+T) — fuzzy search files across project and open in new tab
- **Command mode** (`:q` quit, `:w` save, `:wq` save & quit, `:q!` force quit)
- **Crash recovery** — periodic SHA-256 hashed snapshots with auto-recovery prompt
- **Persistent sessions** — cursor, scroll, buffer stack restored on re-entry
- **Auto-completion** for `.ntc.math` files (functions, constants, keywords)
- **Configurable** tab width, color theme, auto-save, syntax toggle (editor.toml)
- **Template creation** — `ne --init file.rs` creates file from template by extension

**Editor Key Bindings:**

| Key | Action |
|-----|--------|
| Arrow keys / PgUp/PgDn | Move cursor / scroll |
| Home / End | Start / end of line |
| Ctrl+Home / Ctrl+End | Start / end of file |
| Ctrl+Left / Ctrl+Right | Jump words |
| Backspace / Delete | Delete characters |
| Enter | New line |
| Tab / Shift+Tab | Indent / Outdent |
| Ctrl+S | Save file |
| Ctrl+Z | Undo |
| Ctrl+Y / Ctrl+Shift+Z | Redo |
| Shift+Arrow | Select text |
| Ctrl+A | Select all |
| Ctrl+C / Ctrl+X / Ctrl+V | Copy / Cut / Paste |
| Ctrl+F | Find in file |
| Ctrl+G / Ctrl+Shift+G | Find next / previous |
| Ctrl+O | Open file |
| Ctrl+N | New buffer |
| Ctrl+W | Close buffer |
| Ctrl+Tab / Ctrl+Shift+Tab | Switch to next/previous buffer |
| Ctrl+B | Toggle sidebar |
| Ctrl+P / Ctrl+T | File finder |
| Ctrl+D | Add cursor at next occurrence |
| Ctrl+Q | Close tab (with scratch auto-open) |

### 7. Reports

**Supported formats:** TXT, JSON, MD, HTML, PDF, DOCX, XLSX

**Command Line Mode:**
```
ntc -i <directory>              Generate TXT report
ntc -i <directory> -f json      Generate JSON report
ntc -i <directory> -f md        Generate Markdown report
ntc -i <directory> -f html      Generate HTML report
ntc -i <directory> -f pdf       Generate PDF report
ntc -i <directory> -f docx      Generate DOCX report
ntc -i <directory> -f xlsx      Generate XLSX report
ntc -i <directory> -o out.txt   Save to specific file
ntc -i <directory> --cp         Copy report to clipboard
```

**Interactive Mode:**
```
txt [directory] [--cp]          TXT report
json [--cp]                     JSON report
md [--cp]                       Markdown report
html                            HTML report
pdf                             PDF report
docx                            DOCX report
xlsx                            XLSX report
```

**Statistics included:** Total files/directories, total size (bytes + human-readable), supported vs unsupported files, scan time, configuration used.

### 8. Math Expression Evaluator

| Command | Description |
|---------|-------------|
| `math <expression>` | Evaluate a math expression |
| `math 3+4*5` | → 23 |
| `math sin(PI/2)` | → 1 |
| `math sqrt(144)` | → 12 |
| `math print("the answer is", 42)` | → the answer is 42 |
| `math rand(1,100)` | Random number between 1-100 |
| `math timer` | Start lap timer |
| `math timer 10` | Countdown 10 seconds |
| `math <file>.ntc.math` | Compile and run a math script file |
| `ntc --math "3+4*5"` | Evaluate from command line |

**Built-in functions:**
- Print: `print(values...)` — like Python's print (accepts strings and numbers)
- Trig: `sin`, `cos`, `tan`, `cot`, `sec`, `csc`
- Inverse: `arcsin`/`asin`, `arccos`/`acos`, `arctan`/`atan`, `arccot`/`acot`, `arcsec`/`asec`, `arccsc`/`acsc`
- Math: `sqrt`, `pow(x,y)`, `abs`, `floor`, `ceil`, `ceiling`, `round`, `ln`/`log`, `log2`, `log10`
- Aggregate: `sum`, `min`, `max`, `avg`/`average`/`mean`
- Random: `rand(min, max)`
- Convert: `toBinary`, `toHex`, `to8`/`toOctal`, `toDecimal`, `toHB`/`toHumanBytes`

**Constants:** `PI`, `E`/`EXP`, `PHI`, `TAU`

**User-defined functions:**
```
math fun add square(x) = x^2
math square(5)                            # → 25
math fun edit <name> = <body>             # Update
math fun rm <name>                        # Remove
math fun info <name>                      # Show details
math fun ls                               # List all
```

**Features:** Named arguments, string literals with escape sequences, `return` keyword for file-mode scripts, LSP server for .ntc.math.

### 9. Run Aliases — `ral`

| Command | Description |
|---------|-------------|
| `ral add <name> "<command>"` | Create new alias |
| `ral add <name>(x,y) "<cmd $x $y>"` | Create parameterised alias (multiple args) |
| `ral edit <name> "<new_command>"` | Update existing alias |
| `ral rnm <old> to <new>` | Rename an alias |
| `ral rm <name>` | Remove alias |
| `ral list` | Show all aliases |
| `ral cls` | Clear ALL aliases (asks confirmation) |
| `ral cls --force` | Clear ALL aliases (no confirmation) |
| `ral export --all <name>` | Export all aliases to `<name>.ntc.ral` |
| `ral export --select <name>` | Select which aliases to export |
| `ral import <file.ntc.ral>` | Import aliases from .ntc.ral file |

**Examples:**
```
ral add ll "ls -la"
ral add py "python test.py"
ral add runc(x,y) "gcc -o $y $x.c && ./$y"
runc(main,program)                  # Compiles main.c -> program.exe and runs it
```

Features: `&&` chaining (use quotes), recursive alias expansion (up to 5 levels), parameterised with `$x`, `$y`, `$z` placeholders.

### 10. Configuration

**Settings:**
| Command | Description |
|---------|-------------|
| `setO <path>` | Set output directory for reports |
| `setD <number>` | Set max tree depth (1-20) |
| `setL ON\|OFF` | Enable/disable line numbers |
| `setC ON\|OFF` | Enable/disable color output |
| `setA ON\|OFF` | Enable/disable autosuggest ghost text |
| `setT <number>` | Set thread count (1-64) |
| `setH ON\|OFF` | Enable/disable command history |
| `setH <path>` | Set custom history file path |
| `setH default` | Reset history to default |
| `showcg` | Show current configuration overview |
| `watch ON\|OFF` | Enable/disable file watcher |

**Config File Management:**
| Command | Description |
|---------|-------------|
| `opencg` | Open config.toml (external editor, fallback to built-in) |
| `resetcg` | Reset config to defaults (with backup) |
| `restorecg` | Restore config from backup |
| `gencg` | Create ntconfig.toml template |
| `gencg --all` | Export current settings to ntconfig.toml |
| `where` | Show executable and config file locations |

**Project Initialization:**
| Command | Description |
|---------|-------------|
| `init` | Create both NTCRANFILE.toml + ntconfig.toml |
| `init --ran` | Create only NTCRANFILE.toml |
| `init --local` | Create only ntconfig.toml |
| `init --all` | Create both with sample content |
| `deinit` | Remove project files (asks per file) |
| `deinit --ran` | Remove only NTCRANFILE.toml |
| `deinit --local` | Remove only ntconfig.toml |

**Local Project Config (ntconfig.toml):**
Place an `ntconfig.toml` file in any directory to override:
- Ignored directories, extensions, and files
- Extra supported files and extensions
- Run aliases (project-specific commands)

Global settings (teleports, output path, max depth, etc.) remain unaffected.

### 11. Theme System

61 configurable colors across 3 categories:

| Category | Colors |
|----------|--------|
| **Syntax (15)** | keyword, string, comment, number, type, builtin, function, operator, punctuation, attribute, macro_token, regex, tag, constant, normal |
| **Shell UI (20)** | prompt_bracket, prompt_path, prompt_watcher, prompt_arrow, success, error, warning, info, tree_branch, tree_dir, tree_file, tree_ignored, tree_size, command_output, separator, help_header, help_section, help_example, teleport_name, alias_name, alias_command |
| **Editor UI (26)** | editor_bg, gutter_bg, status_bg, hint_bg, sidebar_bg, sidebar_selected_bg, run_panel_bg, gutter_text, status_text, status_modified, hint_text, cursor_bg, cursor_text, extra_cursor_bg, selection_bg, search_match_bg, search_current_bg, border, scrollbar, scrollbar_thumb, sidebar_dir, sidebar_file, sidebar_current, sidebar_selected, run_header_fg, run_output_fg |

**Commands:**
```
theme                     Interactive theme manager (numbered menu)
theme list                List all available themes
theme current             Show current theme name and author
theme <name>              Switch to a theme
theme add <name>          Create a new theme (opens in editor)
theme rm <name>           Remove a custom theme
theme edit <name>         Edit a theme in the editor
theme info <name>         Show theme metadata and color counts
theme export <name>       Export theme to <name>.ntc_theme file
theme import <file>       Import theme from .ntc_theme file
theme rnm <old> to <new>  Rename a theme
theme reload              Reload all themes from disk
```

**Built-in themes:** `default` (dark), `light`, `example` (template)

**Color formats:** Named (black, red, green, ...), RGB (`{ r = 255, g = 128, b = 0 }`), ANSI (`{ code = 196 }`)

**Storage:** `{config_dir}/ntc/themes/` as `.ntc_theme` files. Selection in `theme.toml`.

### 12. UI Modes

```
ui classic    # Traditional shell interface with minimal formatting
ui modern     # Modern boxed interface with borders, git branch, scrollable output
ui            # Show current mode
```

Note: Restart ntc after changing UI mode.

### 13. Ignore/Care System

| Command | Description |
|---------|-------------|
| `ignored` | Show all ignored directories, extensions, files |
| `ignore <name>` | Ignore directory (e.g., "target", "node_modules") |
| `cared <name>` | Stop ignoring a directory |
| `ignoref <ext>` | Ignore .ext files (e.g., "log", "tmp") |
| `caref <ext>` | Stop ignoring .ext files |
| `ignoren <filename>` | Ignore specific file (e.g., "Cargo.lock") |
| `caren <filename>` | Stop ignoring a specific file |
| `igcare export --all <name>` | Export all settings to `<name>.ntc.igcare` |
| `igcare export --select <name>` | Select categories to export |
| `igcare import <file.ntc.igcare>` | Import settings from .ntc.igcare file |

Commands auto-save to `ntconfig.toml` if present, otherwise to global config.

### 14. File Watcher

```
watch ON|OFF                       # Enable/disable file watcher
watch trigger <alias>              # Set alias to auto-run on file change
watch trigger off                  # Disable trigger alias
```

Features: Debounced event batching (400ms), ignore-aware, filetype-aware event classification.

### 15. Dinosaur Runner Game

```
dino    # Play Chrome-style dinosaur runner game in terminal
```

**How to play:** Space or Up Arrow to jump. Avoid obstacles (cacti and birds). Score increases over time. High score saved to `dino.toml`.

### 16. System Commands

| Command | Description |
|---------|-------------|
| `run <command>` / `r <command>` | Execute system command |
| `cmd1 && cmd2` | Chain multiple commands |
| `clear` | Clear terminal screen |
| `version` | Show version information |
| `help` | Show interactive help |
| `exit` / `quit` | Exit ntc |

### 17. Viewing Files

| Command | Description |
|---------|-------------|
| `txt <file>` | Display file contents (text files: .txt, .md, .rs, .py, .c, .js, .html, .json, .toml, etc.) |
| `txtc <file>` | Copy file content to clipboard |
| `txtf <file>` | Display file with selection menu |

### 18. LSP Server

```
ntc --lsp                         # Start ntc-math LSP server over stdio
ntc --lsp-log <path>              # Start with debug logging
```

Features: Diagnostics (syntax errors with position), completions (functions, constants, user-defined), hover documentation, go-to-definition for user-defined functions.

### 19. Shell Completions

```
ntc --generate-completions bash        # Generate bash completions
ntc --generate-completions zsh         # Generate zsh completions
ntc --generate-completions fish        # Generate fish completions
ntc --generate-completions powershell  # Generate PowerShell completions
```

### 20. Additional CLI Flags

| Flag | Description |
|------|-------------|
| `--say <text>` / `--print <text>` | Print text to stdout |
| `--size` | Show current directory size |
| `--view` | Quick view of current directory tree |
| `--view --size` | Quick view with directory sizes |
| `--clear` | Clear the terminal screen |
| `--version` | Show version information |
| `--where` | Show ntc executable and config location |
| `--list` / `--fun` | List all command-line functions |
| `--help` | Show detailed help |

### 21. Two Modes of Operation

**Mode 1: Command Line (Single Shot)**
```
ntc --version
ntc -i . -o report.txt
ntc -i . -f pdf
ntc -i . -f docx -o report.docx
ntc -i . -f xlsx
ntc --help
```

**Mode 2: Interactive Shell**
```
ntc     # Launch interactive shell, type commands at "ntc [path]>"
```

---

## Platform Differences

**Windows-only:** `godrive` command, Inno Setup installer, Windows-style paths (C:\Users)
**Linux/WSL-only:** .deb package, config at `~/.config/ntc/`, Unix-style paths
**macOS:** Universal binary (Intel + Apple Silicon), config at `~/.config/ntc/`
**Termux (Android):** pkg install, config at `$PREFIX/var/lib/ntc/`
**Cross-platform:** All navigation, backup/restore, search, reports, teleport, aliases, editor, syntax highlighting

**Config file locations:**
- Windows: `%APPDATA%\ntc\config.toml`
- Linux/macOS: `~/.config/ntc/config.toml`
- Termux: `$PREFIX/var/lib/ntc/config.toml`

---

## Quick Reference Card

| Action | Command |
|--------|---------|
| Enter interactive mode | `ntc` |
| Change directory | `go /path` |
| Teleport to savepoint | `go to name` |
| View tree | `view` |
| View with sizes & depth | `view -s -d 5` |
| Create backup | `bkup` |
| Restore backup | `pldw` |
| Undo restore | `unpd` |
| Backup diff | `diff` |
| Search files | `fs pattern` |
| Search directories | `ds pattern` |
| Combined search | `locate pattern` |
| Search + navigate | `fgo pattern` |
| Search + display | `fsc pattern` |
| Grep content | `gs pattern` |
| Open built-in editor | `ne file` |
| Create file from template | `ne --init file.rs` |
| Text report | `txt` |
| Copy tree to clipboard | `txt --cp` |
| JSON report | `json` |
| Markdown report | `md` |
| HTML report | `html` |
| PDF report | `pdf` |
| DOCX report | `docx` |
| XLSX report | `xlsx` |
| Save teleport | `tp add name` |
| Jump to teleport | `@name` |
| Create alias | `ral add name "cmd"` |
| Create param alias | `ral add name(x,y) "cmd $x $y"` |
| Export aliases | `ral export --all name` |
| Import aliases | `ral import file.ntc.ral` |
| Export ignore/care | `igcare export --all name` |
| Import ignore/care | `igcare import file.ntc.igcare` |
| Math expression | `math 3+4*5` |
| Math script file | `math file.ntc.math` |
| Math user function | `math fun add name(x) = body` |
| Dinosaur game | `dino` |
| Run alias | `name` |
| Show config | `showcg` |
| Open config file | `opencg` |
| Toggle color | `setC ON/OFF` |
| Reset config | `resetcg` |
| Create local config | `gencg` |
| Show locations | `where` |
| Init project | `init` |
| Deinit project | `deinit` |
| Task runner | `ran <target>` |
| List task targets | `ran list` |
| Theme list | `theme list` |
| Switch theme | `theme default` |
| Exit | `exit` |

---

## Tips & Best Practices

1. Use `bkup` before making major changes to your project
2. Use `pldw` to restore and `unpd` to revert if you made a mistake
3. Use `diff 1` to see what changed since your last backup
4. Use `fs` with fuzzy matching for quick file finding (even with typos!)
5. Use `fgo` to search for a file and jump straight to its directory
6. Use `fsc` to search for a file and view its contents immediately
7. Use `locate` for combined file + directory search
8. Use `ne` (ntcEditor) for quick file edits without leaving the shell
9. Use `ne --init <file>` to create new files from templates
10. Use `go to <tp_name>` for instant navigation to frequently used directories
11. Use `view -s -d <n>` to quickly see folder sizes at specific depth
12. Use teleport points for frequently accessed directories
13. Create run aliases for long or complex commands
14. Set line numbers ON when reviewing code files
15. Use `--cp` to quickly share directory structures
16. Ignore build directories (target, node_modules) for cleaner trees
17. Use `gosc` for rapid directory traversal
18. Chain commands with `&&` for automation
19. Export reports in JSON for integration with other tools
20. Use PDF/DOCX/XLSX for professional report distribution
21. Use `ntconfig.toml` for project-specific settings
22. Back up your config with `resetcg` before major changes
23. Use `theme export` to share custom themes across machines
24. Use `gs` to quickly find where a function or variable is used across the project

---

## Troubleshooting

**Command not found:** Ensure ntc is in your PATH. Check config file exists.

**Clipboard copy not working (Linux):** Install xclip (`sudo apt install xclip`) or wl-clipboard.

**File watcher not working:** Run `watch ON` and restart ntc.

**Teleport points lost:** Check config file exists. Use `tp list` to verify.

**Local config not loading:** Verify `ntconfig.toml` exists and has valid TOML syntax.

**Backup failed (file too large):** ntc skips files >50 MB by default. See `ignore.txt` in backup directory.

**Undo not working:** `unpd` only works after a `pldw` restore. Undo history clears after successful undo.

**Editor not opening:** Ensure file path is valid and writable. Use `ne` without arguments to test.

**Report generation failed (PDF/DOCX/XLSX):** Ensure output directory exists and is writable. Check available disk space.

---

## Version History

- **v2.2.0** — ntcEditor tab close buttons (✕), scratch auto-open, Inno Setup (.iss) syntax highlighting, lock file (.lock) syntax highlighting, sidebar vertical scroll bar, top-level init/deinit commands, `fs`/`ds`/`locate` wildcard patterns (`*`, `%`, `_`, `$`), `gs` grep content search, `ran` task runner, theme system (61 colors, 3 built-in themes, export/import), `tpb` teleport back, `setA` autosuggest toggle, `setC` color toggle, `ral`/`igcare` export/import, `size --care`, `ne --init`, `watch trigger`, shell refactored into modular commands
- **v2.1.0** — Math expression evaluator (built-in functions, constants, user functions, `print()`, `.ntc.math` script files), Dinosaur runner game (`dino`), RAL export/import (.ntc.ral), IGCARE export/import (.ntc.igcare), ntcEditor auto-completion for .ntc.math, LSP server for .ntc.math, NtcMath syntax highlighting
- **v1.9.0** — Internal refactoring and library export support
- **v1.8.0** — Backup & restore system (bkup, pldw, unpd, diff), fuzzy file/directory search (fs, ds), view flags (-s, -d), go to / cd to teleport, RAL multiple arguments, 50MB file limit, Termux compatibility
- **v1.7.0** — Local project configs (ntconfig.toml), config management (opencg, resetcg, restorecg, gencg), && chaining, recursive alias expansion
- **v1.6.0** — JSON/MD reports, clipboard copy, Linux support
- **v1.5.0** — Teleport points, run aliases
- **v1.4.0** — gos / gosc
- **v1.3.0** — Ignore/care filter system
- **v1.2.0** — HTML reports
- **v1.1.0** — back command, configuration settings
- **v1.0.0** — Initial release: cd, tree view, txt report

---

## Supported File Extensions (Default)

txt, md, rs, py, c, cpp, js, html, css, json, toml, yaml, xml, sh, java, go, rb, php, sql, ts, tsx, jsx, swift, kt, scala, dart, r, lua, hs, ex, exs, zig, nim, iss, lock, ntc.ral, ntc.igcare, ntc.math, and many more

---

## Documentation

- **[Full User Guide](use.txt)** — Complete command reference with examples
- **[Changelog](CHANGELOG.md)** — Version history and release notes
- **`help`** inside ntc — Interactive command reference
- **`tutorial`** inside ntc — Step-by-step interactive tutorial

## License

MIT

---

For more information: https://github.com/NuengCoder/ntc