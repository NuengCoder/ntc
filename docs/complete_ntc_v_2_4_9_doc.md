# ntc 2.4.9 — Complete Project and Command Documentation

This document describes **ntc 2.4.9**: what the program is, why the code is
organised the way it is, and every command it ships — what each command does and
which files implement it.

It is a reference for users who want to understand the tool and for developers
who want to change it. Line-level behaviour lives in `help <command>` inside the
program and in `CHANGELOG.md`; this document explains the shape of the whole
project and the role of each part.

---

## 1. What ntc is

ntc (Navigate · Toolkit · Center) is a **cross-platform interactive shell and
command toolkit**: one binary that replaces the usual pile of small utilities
with a consistent, colourful CLI. It runs on Windows, Linux, macOS and Android
(Termux, arm64), in any terminal, and it can:

- navigate quickly (`tp`, `tpb`, `gos`, `gosc`, `back`, `cd`, `nfe`),
- inspect and edit files (`view`, `ne`, `txtf`, `txtc`, `todo`, `nfe`),
- filter text in pipelines (`head`, `tail`, `betw`, `gs`, `sort`, `rev`,
  `count`, `trim`, `line`, `qur`),
- download/upload/serve (`dl`, `up`, `http`, `serve`),
- use developer helpers (`git`, `ral`, `ran`, `scr`, `ntpkg`, `spec`, `sys`,
  `ndb`, `skill`, …),
- run a real interactive shell: aliases, pipelines, redirection, tab
  completion, ghost hints, themes, local per-project configuration.

Everything is one Rust crate (`ntc`). The shell is the product: `ntc` with no
arguments opens it, while `ntc <command> …` runs a single command and exits.
`ntc --help`, `ntc --version`, `ntc --here`, `ntc --path <dir>`, `ntc --new`
and `ntc --upgrade` are the command-line surface
(`src/cli/run.rs`).

Recognisable building blocks:

| Piece | Role |
| --- | --- |
| Interactive shell | rustyline editor loop with hints, highlighting and history (`src/shell/entry.rs`) |
| Command registry | one static table that dispatch, help, completion and alias validation read (`src/shell/commands/`) |
| Pipeline engine | `\|`, `&&`, `\|\|`, `;`, `<`, `<<`, `<<<`, `>`, `>>` (`src/shell/pipeline.rs`) |
| Config | global + local `ntconfig.toml`, typed getters/setters (`src/utils/config/`) |
| Language packs | English, Thai, Lao, Chinese strings (`src/utils/config/lang/`) |
| Themes | `ntc_theme` files and built-in themes (`src/utils/theme/`) |
| Navigation core | `Navigator`, teleport savepoints, session state, watcher (`src/utils/`) |

---

## 2. Entry points and process flow

| File | Purpose |
| --- | --- |
| `src/main.rs` | Binary entry. Ensures default themes exist, loads the global config, sizes the rayon thread pool from `config threads`, then calls `cli::run_cli()`. |
| `src/lib.rs` | Library crate: exposes `cli`, `shell`, `utils` and re-exports the commonly used modules/`Config`/`Navigator`/`TeleportManager`. |
| `src/cli/mod.rs` | CLI module root; re-exports `run_cli`. |
| `src/cli/run.rs` | Parses process arguments, handles `--version`, `--upgrade`, `--here`, `-p/--path`, `-n/--new`, `--help`, and one-shot `ntc <command> [args...]` (arguments are re-quoted so quoted formats survive), returning “start the shell?” plus an exit code. |
| `src/cli/helpers.rs` | Small CLI helpers such as the minimal `--help` screen and launching a new terminal window. |
| `src/cli/completions/` | Shell completion script generation for the host shells. |
| `src/shell/entry.rs` | The interactive loop: rustyline editor, prompt, key bindings, history, plugin of hinter/highlighter, session restore/save. |
| `src/shell/mod.rs` | Shell module root and re-exports (`run_shell`, `run_shell_with_nav`). |

Command-line dispatch order for one typed line
(`src/shell/commands/mod.rs::execute_single_command`):

```text
alias expansion (src/shell/alias.rs, ral)
  → tutorial guard (builtin/tutorial)
  → built-in registry (commands::registry_data::registry())
  → fallback (cmd_fallback): `@teleport`, math expression, OS command
  → if none matched: "command not found"
```

A line containing a top-level operator is handled by `src/shell/pipeline.rs`
before any of that: it splits the line into logical segments and pipeline
stages, captures each stage’s stdout in memory, strips ANSI when forwarding,
and sets the failure side-channel that `&&` / `||` read.

---

## 3. Why the tree looks like this

The top level of `src/` is deliberately tiny: `main.rs`, `lib.rs`, and three
directories. Each directory answers one question.

```text
src/
├── main.rs          process entry
├── lib.rs           library surface
├── cli/             "what did the user ask the executable to do?"
├── shell/           "how does an interactive session work?"
└── utils/           "what shared engines do commands build on?"
```

### 3.1 `src/cli/` — the executable’s front door

Only argument parsing and process-level concerns live here. Keeping it separate
means the shell never has to know whether it was started by a human, a script,
or `ntc <command>`.

### 3.2 `src/shell/` — the session and the command framework

| Path | Responsibility |
| --- | --- |
| `shell/commands/` | Command modules plus the shared framework (specs, registry, completion, help wiring) |
| `shell/pipeline.rs` | Tokenizer, operator parser, redirection, capture, ANSI stripping, failure status |
| `shell/stdin.rs` | Virtual stdin installed by pipeline stages and `<` / `<<` / `<<<` redirects |
| `shell/alias.rs` | `ral` alias expansion across `&&`, `\|\|`, `;`, `\|` |
| `shell/help/` | Compact overview, full reference, per-topic sections, contextual help, golden tests |
| `shell/hinter/` | Ghost-text suggestions: `tree.rs` (command/subcommand walking), `providers.rs` (data sources), `files.rs` (paths) |
| `shell/ext_complete/` | Completion of external command arguments (bash/zsh/PowerShell hosts, platform data) |
| `shell/prompt_highlight.rs` | Syntax colouring of the line being typed |
| `shell/idle.rs` | The dimmed teaching hint shown on an idle, empty prompt |
| `shell/update_check.rs` | Background “new version available” check |
| `shell/venv.rs` | Python virtualenv detection for `py_activate` / `py_deactivate` |

### 3.3 `src/shell/commands/` — the command framework

```text
commands/
├── mod.rs              pipeline glue, tutorial guard, fallback dispatch
├── core/
│   ├── spec.rs         CommandSpec, Handler, FlagPolicy, CommandSource
│   ├── registry.rs     Registry (lookup, alias map, reserved-name checks)
│   ├── completion.rs   CompletionSpec tree type
│   └── help.rs         help topic/contextual-help types
├── builtin/
│   ├── mod.rs          BUILTIN_SPECS, HELP_TOPICS, HELP_ORDER, COMPACT_HELP,
│   │                   CONTEXTUAL_HELP — the only command tables
│   ├── <cmd>/          one module per command: mod.rs + help.rs (+ completion.rs)
│   └── <cmd>/<engine>/ large engines nested in their wrapper (ndb/ndb, ne/editor,
│                       nfe/nfe, ntpkg/ntpkg, skill/skill, serve/*)
├── support/            shared engines used by several commands
├── registry_data.rs    process-wide `OnceLock<Registry>` built from BUILTIN_SPECS
└── plugin/             the plugin seam (manifest, registry) — empty in 2.4.9
```

**`CommandSpec` is the single source of truth.** Each command module owns a
`static SPEC` with its canonical name, aliases, help topic, flag policy,
contextual-help flag, completion tree, whether the second word is a subcommand
for highlighting, its `CommandSource`, and its handler. The registry is built
from `BUILTIN_SPECS` and then used by:

- **dispatch** (`execute_single_command`),
- **help topic lookup and order** (`HELP_TOPICS`, `HELP_ORDER`),
- **compact help** (`COMPACT_HELP` for the one-line overview),
- **tab completion and hints** (`CompletionSpec`),
- **alias-name validation**, so a user alias can never shadow a command.

That is *why* the framework exists: before it, dispatch, help and completion
each carried their own name list and could drift. Adding a command now means
adding one module and one line to `BUILTIN_SPECS`, plus its help entry — not
editing three tables.

**Handlers** are `Handler::Legacy(fn(&str, &mut Navigator) -> Result<bool>)`;
returning `Ok(true)` asks the shell to exit. Failure is reported either by the
handler or by `print_error`, which also feeds the pipeline failure status.

**Shared support engines** live in `commands/support/` so two commands never
grow two copies of the same logic. Notable ones:

| Support module | Used by |
| --- | --- |
| `support/text.rs` | `head`, `tail`, `betw` (tokenizer, file/stdin reader, line slicing) |
| `support/codec.rs` | `encode`, `decode`, `cvcode` |
| `support/path_env.rs` | `usrpath`, `syspath` (per-OS PATH backends) |
| `support/file_ops.rs` | `cp`, `mv`, `rmf`, `rmd`, `rnmf`, `rnmd`, `mkf`, `mkd` |
| `support/igcare.rs` | `ignore`, `care`, `igcare`, `igcared` |
| `support/local_config.rs` | `local` and local `ntconfig.toml` handling |
| `support/todo.rs` | `todo` storage across txt/md/json/toml/yaml/html |
| `support/report_*.rs` | `report`, `txtc`, `txtf`, `view` rendering |
| `support/net/` | `ping`, `trace`, `dns`, `smtp`, `http` network helpers |
| `support/fmt/` | `fmt` format conversion |
| `support/system.rs` | External command execution and shutdown/CTRL-C plumbing |
| `support/upgrade.rs` | `ntc --upgrade` self-update |
| `support/check/` | `doctor` environment checks |

**`commands/plugin/`** is a forward seam: manifest parsing, a registry type and
collision rules, with an intentionally empty process-wide registry. Nothing
loads a plugin in 2.4.9 and the `plugin` name is reserved. See
`docs/architecture/plugins.md` for the out-of-process JSON-RPC contract.

### 3.4 `src/utils/` — engines without a shell dependency

| Path | Responsibility |
| --- | --- |
| `utils/config/` | `Config` (global/local `ntconfig.toml`), typed accessors, setters, validation |
| `utils/config/lang/{en,th,lo,zh}/` | The four bundled language packs (`help/`, `cmd/`, `cli.rs`, …) |
| `utils/navigator.rs` | Working-directory state, `@teleport` resolution, directory helpers |
| `utils/teleport.rs` | Savepoints (`tp`, `cd`, `cd to`, `tpb` history) |
| `utils/session.rs` | Session file: per-directory history buckets, last directory |
| `utils/paths.rs` | `~`, `$VAR`, `${VAR}`, `%VAR%` expansion |
| `utils/output/` | Coloured output helpers, file reading, line slicing, range parsing |
| `utils/search.rs`, `utils/fuzzy.rs` | Name/content search and Jaro-Winkler fallback |
| `utils/filetype.rs`, `utils/syntax/` | Extension/type tables and the shared syntax highlighter/theme roles |
| `utils/theme/` | Theme loading, merging, colour conversion (`ThemeColor::to_hex`), styling |
| `utils/interactive.rs` | Shared interactive picker/menu used by many commands |
| `utils/explorer.rs` | Explorer state shared with `nfe` |
| `utils/watcher.rs` | Filesystem watcher backing `wi` |
| `utils/qsc.rs` | Quick command listener state |
| `utils/backup*.rs` | Project snapshots, diff, manifests |
| `utils/git.rs` | Git command helpers |
| `utils/os_env.rs` | OS-level environment reads/writes |
| `utils/terminal.rs`, `utils/ansi_text.rs` | Terminal capability and ANSI-aware text handling |
| `utils/serve_info.rs` | The `/info` page of `serve` |
| `utils/frame_log.rs` | Frame/rendering debug log |
| `utils/test_support.rs` | Test-only config/path fixtures |

Why a separate `utils/`? These engines are pure library code: they must be
testable without a terminal and reusable from any command (and from `lib.rs`’s
public surface). Commands stay thin; engines stay shared.

---

## 4. Command catalogue (113 built-in commands in 2.4.9)

Every path below is relative to `src/shell/commands/builtin/`. Each directory
contains `mod.rs` (the command body and its `SPEC`) and `help.rs` (its help
section); some also carry `completion.rs`, and large engines nest one level
deeper inside the same command folder.

| Command | Aliases | What it does | Implemented in |
| --- | --- | --- | --- |
| `arc` | — | Create, list and extract `.zip` / `.tar.gz` / `.tar` archives | `arc/` |
| `autoup` | — | List/add/remove/disable Windows startup entries | `autoup/` |
| `back` | `b` | Go back to the parent directory (or *n* levels) | `back/` |
| `backup` | — | Create/list/diff/restore project backups, undo, clear, recreate | `backup/` (+ `utils/backup*.rs`) |
| `betw` | — | Print lines A to B (1-based, inclusive) | `betw/` (+ `support/text.rs`) |
| `care` | — | Un-ignore directories, extensions and files | `care/` (+ `support/igcare.rs`) |
| `cd` | — | Change directory; `cd to <tp>` jumps to a savepoint | `cd/` |
| `clear` | — | Clear the terminal screen | `clear/` |
| `clip` | — | Show/copy/clear clipboard history | `clip/` |
| `config` | `cfg` | Show or change every ntc setting | `config/` (+ `utils/config/`) |
| `count` | — | Count occurrences of text; `-l` counts piped lines | `count/` |
| `cp` | — | Copy files or directories (teleport-aware) | `cp/` (+ `support/file_ops.rs`) |
| `cvcode` | — | Convert data between formats | `cvcode/` (+ `support/codec.rs`) |
| `deadlock` | — | Encrypted password vault with its own shell | `deadlock/` |
| `decode` | — | Decode hex, base64, URL and binary | `decode/` (+ `support/codec.rs`) |
| `deinit` | — | Remove project files (`ran`, `local`, `opencode`), asking before each | `deinit/` |
| `dl` | — | Download a file (resume, progress, stop) | `dl/` |
| `dns` | — | DNS lookup | `dns/` (+ `support/net/`) |
| `doctor` | — | Check config, PATH, ranfile and ntpkg health | `doctor/` (+ `support/check/`) |
| `ds` | — | Search directories by name | `ds/` (+ `utils/search.rs`) |
| `encode` | — | Encode hex, base64, URL and binary | `encode/` (+ `support/codec.rs`) |
| `exit` | `quit`, `esc` | Exit ntc | `exit/` |
| `fake` | — | Generate fake data (people, tech, education types) | `fake/` |
| `fgo` | — | Find files and jump to the chosen one | `fgo/` |
| `fmt` | — | Convert between file formats | `fmt/` (+ `support/fmt/`) |
| `fs` | — | Search files by name (fuzzy fallback) | `fs/` (+ `utils/search.rs`) |
| `fsc` | — | Search files and open the chosen one | `fsc/` |
| `git` | — | Git shortcuts: status, branch, log and friends | `git/` (+ `utils/git.rs`) |
| `godrive` | `god` | Switch drives (Windows) or `/mnt` mounts (Unix) | `godrive/` |
| `gos` | — | Pick a subdirectory from a menu and enter it | `gos/` |
| `gosc` | — | Browse subdirectories continuously | `gosc/` |
| `gs` | — | Search file contents with match filters, context and an interactive pager | `gs/` |
| `head` | — | Print the first N lines of a file or piped input | `head/` (+ `support/text.rs`) |
| `help` | — | Compact overview, `help <command>`, `help --all` | `help/` (+ `shell/help/`) |
| `http` | — | Make an HTTP request | `http/` (+ `support/net/`) |
| `igcare` | — | Import/export ignore and care rules | `igcare/` (+ `support/igcare.rs`) |
| `igcared` | — | Show all ignore/care settings | `igcared/` |
| `ignore` | — | Ignore directories, files and extensions | `ignore/` |
| `image` | `img` | Show images inline in the terminal | `image/` |
| `init` | — | Initialise project files (`ran`, `local`, `opencode`) | `init/` |
| `ipcfg` | — | Show IP configuration | `ipcfg/` |
| `line` | — | Prefix every input line with a running number | `line/` |
| `local` | — | Manage the per-directory `ntconfig.toml` | `local/` (+ `support/local_config.rs`) |
| `locate` | — | Search files and directories | `locate/` |
| `math` | — | Evaluate a math expression | `math/` |
| `mkd` | — | Create directories (teleport-aware) | `mkd/` |
| `mkf` | — | Create files (teleport-aware) | `mkf/` |
| `mode` | — | Relaunch in admin / normal mode, keeping the directory | `mode/` |
| `mv` | `move` | Move or rename files and directories | `mv/` |
| `ndb` | — | Browse and query SQLite databases (grid, schema, SQL console) | `ndb/` + `ndb/ndb/` |
| `ne` | `ntceditor` | Built-in editor with syntax highlighting and an `.ntc` language service | `ne/` + `ne/editor/`, `ne/mql/`, `ne/ntc/` |
| `nfe` | (`explorer` help topic) | Full-screen file explorer: find mode, live filter, command mode | `nfe/` + `nfe/nfe/` |
| `nms` | — | Terminal music player | `nms/` |
| `nprintf` | — | Bash-style `printf`: conversions, flags, width, precision, escapes | `nprintf/` |
| `nsheet` | — | View spreadsheets | `nsheet/` |
| `nssh` | — | SSH profiles and file copy | `nssh/` |
| `ntpkg` | — | Install and manage tools (including Stockfish) | `ntpkg/` + `ntpkg/ntpkg/` |
| `ntpx` | — | Run a binary installed by `ntpkg` | `ntpx/` |
| `opencg` | `editcfg`, `editcg` | Open `config.toml` in the editor | `opencg/` |
| `pathcp` | — | Copy the current path to the clipboard | `pathcp/` |
| `ping` | — | Ping a host with portable flags | `ping/` (+ `support/net/`) |
| `py_activate` | — | Activate a Python virtualenv | `py_activate/` (+ `shell/venv.rs`) |
| `py_deactivate` | — | Deactivate the Python virtualenv | `py_deactivate/` |
| `qr` | — | Make and read QR codes | `qr/` |
| `qsc` | — | Quick command listener | `qsc/` (+ `utils/qsc.rs`) |
| `qur` | — | Query files and replace text | `qur/` |
| `ral` | — | Parameterised aliases, with `#param --hint-*` annotations | `ral/` |
| `ran` | — | Run tasks and builds from `NTCRANFILE.toml` | `ran/` |
| `report` | — | Project reports (`report project <fmt> [path] --name <file>`) | `report/` (+ `support/report_project.rs`) |
| `resetcg` | `reset-config` | Reset the configuration to defaults | `resetcg/` |
| `restorecg` | `restore-config` | Restore the configuration from its backup | `restorecg/` |
| `rev` | — | Reverse each input line | `rev/` |
| `rmd` | — | Delete directories | `rmd/` |
| `rmf` | — | Delete files | `rmf/` |
| `rnmd` | — | Rename a directory | `rnmd/` |
| `rnmf` | — | Rename a file | `rnmf/` |
| `run` | `r` | Execute an alias/command through the shell | `run/` |
| `say` | — | Print text | `say/` |
| `scan` | — | Scan extensions, ports, IPs and web paths (`scan waep`) | `scan/` |
| `scr` | `script` | Run `.ntc` scripts: blocks, loops, functions, expressions | `scr/` |
| `serve` | — | Serve files or an app: `files`, `http-server`, `chess`, `git`, `arc`, `theme`, `ndb`, `backup`, `fake`, `config`, `mdview` | `serve/` (one module per service + `serve/assets/`) |
| `showcg` | — | Show the current configuration | `showcg/` (+ `support/config_display.rs`) |
| `showenv` | — | Show environment, NTC variables and PATH | `showenv/` |
| `size` | — | Directory sizes | `size/` |
| `skill` | — | Download, install and manage AI skills for harnesses | `skill/` + `skill/skill/` |
| `smtp` | — | Send email through an SMTP client | `smtp/` (+ `support/net/`) |
| `sort` | — | Sort lines (reverse, numeric, unique, case-insensitive) | `sort/` |
| `spec` | — | Show computer specifications (CPU, GPU, disks, memory) | `spec/` |
| `sys` | `top` | Live system monitor: CPU/MEM gauges, disks, process tree | `sys/` |
| `syspath` | — | Manage the system PATH per platform | `syspath/` (+ `support/path_env.rs`) |
| `tail` | — | Print the last N lines of a file or piped input | `tail/` (+ `support/text.rs`) |
| `theme` | — | Switch, add, update and export themes | `theme/` (+ `utils/theme/`) |
| `time` | — | Time a command or run a countdown | `time/` |
| `tl` | — | Count lines in files (optional directory) | `tl/` |
| `tmprt` | — | Temperature and weather | `tmprt/` |
| `todo` | `td` | Project todo list in txt/md/json/toml/yaml/html, with move/replace | `todo/` (+ `support/todo.rs`) |
| `tp` | — | Teleport savepoints: add, list, remove, jump | `tp/` (+ `utils/teleport.rs`) |
| `tpb` | — | Teleport back through the history stack | `tpb/` |
| `trace` | `tracert`, `traceroute` | Trace the route to a host (TTL-walk fallback) | `trace/` (+ `support/net/`) |
| `trim` | — | Strip a pattern from lines | `trim/` |
| `tutorial` | `guide` | Guided, sandboxed hands-on lessons | `tutorial/` |
| `txtc` | — | Copy file content to the clipboard (`--line` / `--no-line`) | `txtc/` (+ `support/report_txtc.rs`) |
| `txtf` | — | Display file content | `txtf/` (+ `support/report_txtf.rs`) |
| `ui` | — | Show the current UI mode | `ui/` |
| `up` | — | Upload a file | `up/` |
| `usrpath` | — | Manage the user PATH per platform | `usrpath/` (+ `support/path_env.rs`) |
| `version` | — | Show the ntc version | `version/` |
| `view` | — | Directory and file trees | `view/` (+ `support/report_view.rs`) |
| `viewd` | — | Directories-only tree | `viewd/` |
| `viewf` | — | Files-only tree | `viewf/` |
| `views` | — | Short flat directory listing | `views/` |
| `where` | — | Print the ntc executable path, or find binaries on `PATH` | `where/` |
| `wi` | — | Live file-change feed | `wi/` (+ `utils/watcher.rs`) |

> `explorer` is the help topic for `nfe`, and `pipe` is a help topic without a
> command. Neither adds a registry entry.

### 4.1 Help topics

`help` with no argument prints the compact overview; `help <command>` prints one
section; `help --all` prints every section in alphabetical order. Each section is
owned by its command module’s `help.rs` and registered in
`HELP_TOPICS`/`HELP_ORDER` in `builtin/mod.rs`. The non-command `pipe` topic is
rendered by `betw/help.rs`. Help text is translated through the four language
packs (`src/utils/config/lang/<lang>/help/<topic>.rs`), and the rendered output
is locked by golden snapshots (`src/shell/help_golden/*.golden`, regenerated with
`NTC_UPDATE_HELP_GOLDEN=1 cargo test --lib golden_tests`).

---

## 5. Pipelines and redirection

`src/shell/pipeline.rs` implements a bash-like pipeline layer. Operators only
count as operators when they are **standalone whitespace-separated tokens** and
are not inside quotes or `[...]` arrays, so `age>30` or a quoted `|` stays data.

| Operator | Meaning |
| --- | --- |
| `a \| b` | Pipe `a`’s stdout into `b`’s stdin |
| `a && b` | Run `b` only if `a` succeeded |
| `a \|\| b` | Run `b` only if `a` failed |
| `a ; b` | Run `b` whatever `a` did (spaces required) |
| `cmd < file` | Read stdin from a file |
| `cmd <<EOF … EOF` | Heredoc: read stdin from the following lines |
| `cmd <<< text` | Here-string: read stdin from the literal token |
| `cmd > file` | Write stdout to a file (overwrite) |
| `cmd >> file` | Append stdout to a file |

Piped/redirected output is captured in memory, stripped of ANSI escapes, and
then forwarded, so files stay clean. Each stage can read the installed virtual
stdin (`shell/stdin.rs`); system commands receive the same bytes as real stdin.
Failure is tracked in a thread-local flag that `&&` / `||` and the process exit
code consult.

### 5.1 Pipe filters

These commands read piped input when no file is given, so they compose:

```text
head  tail  betw  gs  sort  rev  count  trim  line  encode  decode  qur  clip
```

Their flags complete after the pipe as well.

---

## 6. Shell syntax essentials

| Feature | Where it lives |
| --- | --- |
| Quoting: `"…"` keeps spaces together; `'…'` is preserved by the pipeline tokenizer | `shell/pipeline.rs`, per-command tokenizers |
| `$NAME` / `${NAME}` expansion before the line runs (local ntc → global ntc → OS env) | `utils/paths.rs`, `utils/config/` |
| `~` expansion in every path argument | `utils/paths.rs` |
| Aliases: `ral` templates with `${x}`, `${x:-default}`, `${x[]}`, `${x[,]}`, `$*{x}` and bare `$NAME` environment references | `shell/alias.rs`, `builtin/ral/` |
| Task runner: `ran <target>` from `NTCRANFILE.toml`; `ran check` diagnoses the file | `builtin/ran/` |
| Scripts: `scr file.ntc`, `run file.ntc`, or simply `file.ntc`; blocks, loops, functions, expressions, `set-pipe`, `set-showexc` | `builtin/scr/` |
| Teleport: `@name/path` resolves through `tp` savepoints | `utils/navigator.rs`, `utils/teleport.rs` |
| Session restore: the shell reopens in the last directory, with per-directory history buckets | `utils/session.rs`, `shell/entry.rs` |
| Tab completion + ghost hints, including inside pipeline stages | `shell/hinter/` |
| Idle teaching hints (`config idle`) | `shell/idle.rs` |
| Tutorial sandbox and lesson checks | `builtin/tutorial/` |

---

## 7. Configuration, themes and language

- **Config files.** Global config under the ntc config directory; local
  `ntconfig.toml` in a project (created with `local init`) overrides it.
  `config` is the entry point; the typed model, validation and setters live in
  `src/utils/config/`.
- **Environment variables.** `config env NAME value` stores in local/global
  config, `-os` for the session, `-osg` persists at OS level;
  `config rmenv` removes. Lookup order is local → global → OS.
- **Themes.** `theme` switches/adds/exports; theme files and colour conversion
  live in `src/utils/theme/`. Editors (`ne`, `serve mdview`) and the monitor
  read the same `syntax.*` roles.
- **Language packs.** English (`en`), Thai (`th`), Lao (`lo`) and Chinese
  (`zh`) under `src/utils/config/lang/`. Every user-visible string — CLI
  messages, command output, help sections and usage screens — is looked up with
  the `t!` macro from the active pack, falling back to English.

---

## 8. Services behind `serve`

`serve` is a small web application host. The service table is shared by the
dispatcher, `serve` help, `showcg` and `config serve`:

| Service | What it serves |
| --- | --- |
| `files` | File upload/download web UI plus `/info` recipes |
| `http-server` | Static directory server |
| `chess` | Browser chess board with Stockfish bots |
| `git` | Tabbed git dashboard (changes, history, branches, diff) |
| `arc` | Archive browsing/creation |
| `theme` | Theme TOML editor with live preview |
| `ndb` | SQLite browser with optional confirmed writes and backups |
| `backup` | Snapshot management GUI |
| `fake` | Fake-data generator GUI |
| `config` | Configuration GUI |
| `mdview` | Markdown editor with live, theme-aware preview |

Each service is a module under `src/shell/commands/builtin/serve/`; shared
browser assets live in `serve/assets/`. `--tunnel`, `--share`, `--qr`, `--port`,
`--bind` and `--auth` are handled uniformly by `serve/mod.rs`, `tunnel.rs`,
`share.rs` and `service.rs`.

---

## 9. Plugin seam (reserved, not user-visible)

`src/shell/commands/plugin/` ships:

- `manifest.rs` — `PluginManifest`/`PluginCommand` TOML parsing and validation
  (reverse-DNS id, required fields, `abi == HOST_ABI`, locale-keyed summaries,
  unknown-field rejection);
- `registry.rs` — `PluginRegistry` with duplicate-id and reserved-name checks;
- `mod.rs` — the process-wide, deliberately empty registry.

No plugin directory is read, no subprocess is spawned and no `plugin` command
exists in 2.4.9. The full transport/trust contract is in
`docs/architecture/plugins.md`.

---

## 10. Adding or changing a command

1. **Create the module** under `src/shell/commands/builtin/<name>/` with
   `mod.rs` (body + `SPEC`) and `help.rs` (a `print_<name>_help_section` using
   `shell/help/format::print_section`). Put shared logic in
   `commands/support/` rather than duplicating it in the command.
2. **Register it** in `builtin/mod.rs`: the `pub(crate) mod <name>;` line,
   `&<name>::SPEC` in `BUILTIN_SPECS`, a `HelpTopic` in `HELP_TOPICS`, the name
   in `HELP_ORDER`, a row in `COMPACT_HELP`, and the name in
   `builtin/help/completion.rs`.
3. **Add the strings** to all four language packs:
   `src/utils/config/lang/<lang>/help/<name>.rs` (registered in that pack’s
   `help/mod.rs` and invoked from `<lang>/mod.rs`) and any `misc.*` usage keys.
4. **Document flags** in the usage strings; completion is data-driven from
   `CompletionSpec`, so add flags/subcommands there when the command has them.
5. **Test**: unit-test parsing/filtering in `mod.rs`, then run
   `cargo test --lib` and refresh the help goldens with
   `NTC_UPDATE_HELP_GOLDEN=1 cargo test --lib golden_tests`.

Nothing else needs editing: dispatch, help lookup, alias validation, prompt
highlighting and completion all read the registry.

---

## 11. Verification and quality gates

| Check | Command |
| --- | --- |
| Formatting | `cargo fmt --check` |
| Compile | `cargo build` (and `cargo build --no-default-features` where relevant) |
| Tests | `cargo test --lib` |
| Help output stability | `cargo test --lib golden_tests` (refresh with `NTC_UPDATE_HELP_GOLDEN=1`) |
| No emoji in CLI/UI/installers | `tests/no_emoji.rs` |
| Plugin seam invariants | `plugin::*` and `commands::*` tests plus `registry_data` tests |

The help goldens are the guard that help text, ordering and command coverage
cannot drift silently; the registry tests guard unique names/aliases and that
every alias resolves to its canonical command.

---

## 12. Quick file map

```text
src/main.rs                          process entry
src/lib.rs                           library surface + compatibility re-exports
src/cli/{mod,run,helpers}.rs         argument parsing, one-shot commands, completions/
src/shell/entry.rs                   interactive rustyline loop, keys, history
src/shell/pipeline.rs                tokenizer, operators, redirection, capture
src/shell/stdin.rs                   virtual stdin for pipes/redirects
src/shell/alias.rs                   ral expansion across separators
src/shell/help/                      compact/full/topic/contextual help + goldens
src/shell/hinter/                    ghost-text completion (tree/providers/files)
src/shell/prompt_highlight.rs        line colouring
src/shell/commands/mod.rs            dispatch, tutorial guard, fallback
src/shell/commands/builtin/mod.rs    BUILTIN_SPECS / HELP_TOPICS / HELP_ORDER / COMPACT_HELP
src/shell/commands/builtin/<cmd>/    one module per command
src/shell/commands/core/             spec, registry, completion, help types
src/shell/commands/support/          shared engines
src/shell/commands/plugin/           plugin seam (empty registry)
src/shell/commands/registry_data.rs  process-wide registry
src/utils/config/                    Config, setters, lang packs
src/utils/theme/                     themes and colour conversion
src/utils/{navigator,teleport,session,paths,watcher}.rs  navigation core
src/utils/output/                    output, file reading, line slicing
src/utils/search.rs, fuzzy.rs        search backends
```
