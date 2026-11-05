# Changelog

## [Unreleased]

### New Features

#### search — `--no-fuzzy`
- `fs`, `ds`, `fsc` and `locate` accept `--no-fuzzy` (or `-no-fz`) to skip the Jaro-Winkler fallback and keep exact/partial matches only; the flag is documented in each usage line and in `help search`

#### spec — multi-GPU, Android CPU and disk usage
- One row per GPU adapter instead of a joined list, so multi-GPU machines no longer show a value cut short by the column width
- Android CPU brand falls back to `getprop ro.soc.model` / `ro.board.platform` / `ro.hardware` and `/proc/cpuinfo` (`Hardware`, `Processor`) when `sysinfo` reports no brand; cores/threads fall back to the logical count instead of `0 / 0`
- Each disk row now shows used/total space, the used percentage and free space (for example `200 GB/300 GB 67% 100 GB free`) instead of only the drive size; the "free" label follows the active language

#### deadlock — `clear`
- `clear` / `cls` inside the vault shell wipes the terminal and leaves the vault untouched

#### ndb — dbal parameter hints
- `dbal add|edit` accepts the same trailing `#param --hint-*` annotations as `ral`, including the ndb-only `--hint-table` and `--hint-table-col-name [table]`; hints are stored alongside the alias (local `ntconfig.toml` when present) and survive `dbal rnm`, while `dbal rm` / `dbal cls` remove them
- The ndb hinter completes `dbal run <alias> <TAB>` and bare `<alias> <TAB>` from the open database: `--hint-table` offers table names, `--hint-table-col-name users` offers that table's columns, and a bare variant offers every column; `dbal add|edit … #t --hint-` completes the two flags
- `dbal info` prints the stored hint spec under the params line

#### ntpkg — Stockfish package
- `ntpkg install stockfish` lists official releases (`sf_19`, `sf_18`, `sf_17.1`, ...) and installs the engine from `github.com/official-stockfish/Stockfish`; Stockfish 19+ universal binaries are used, with per-CPU assets for older releases
- The engine lands in the package version directory as `stockfish` / `stockfish.exe` and speaks UCI, so `ntpx stockfish` runs it directly; unsupported architectures report a clear prebuilt-binary error instead of a broken download
- `serve chess` reuses engines installed with `ntpkg install stockfish` instead of downloading a second copy, and its Uninstall action points at `ntpkg uninstall` for that engine
- Every installed Stockfish build (each `ntpkg` version, the ntc download, a `PATH` binary) is now selectable in the bot screen and the choice is remembered; it no longer follows `ntpkg use`, and a machine with no engine is pointed at `ntc ntpkg install stockfish`
- Uncompressed `.tar` release assets are supported by the installer archive layer

#### skill — install AI skills into your harness
- `skill download <alias|owner/repo[/skill]|url>` (also `skill download --url <url>`) fetches `SKILL.md` folders from GitHub into `<config>/ntc/skills/library/`, with built-in aliases `superpowers` (obra/superpowers) and `frontenddev` (frontend-design from anthropics/skills)
- `skill add <name> to <opencode|claude|codex|agents>` copies a skill or a whole bundle into the harness's project folder, `--global` uses the personal folder; `skill remove` / `skill delete` undo the copy or the library entry
- `skill` lists skills and where each one is installed, `skill edit` opens the exact `SKILL.md` in ntceditor, and `skill help` documents the verbs
- Downloads are capped and extracted with traversal/symlink guards; nothing is executed. A recorded `sha256` blocks removing a copy you edited by hand unless `--force` is passed
- `skill avail` lists every supported source: `superpowers`, `frontenddev`, `mcp-builder`, `webapp-testing`, `skill-creator` (anthropics/skills), `sentry-fix-issues` (getsentry/sentry-agent-skills) and `static-analysis` (trailofbits/skills); the `agenticskills.io` listing URL resolves to the same alias
- Four harness families joined `opencode`/`claude`/`codex`/`agents`: `freebuff`, `kilocode`, `mimocode`, `workbuddy` and `trae`, each with a project `.<harness>/skills` folder and a `--global` personal one
- `skill ls` now shows one line per downloaded bundle (with its copy list) and `skill ls --all` keeps the per-skill view; `skill sub <name> ls` lists the skills inside a bundle and says so when there are none
- `skill info <harness>` scans this repo's harness folder (and the global one) and reports every skill it holds, marking copies ntc manages
- `skill download --name <name>` installs a skill under a different name (the folder and `SKILL.md` frontmatter are renamed together); `skill install`/`skill i` are aliases of `skill download`, `skill uninstall`/`skill u` of `skill delete`

#### serve chess — highlight toggles and AI bots
- Settings gained on/off switches for the board overlays: `Pinned pieces` hides the gray king pins and aqua relative pins, `Hanging danger` hides the orange attacked squares; checks and last-move highlights always stay visible
- `vs AI Bot` is live: pick Stockfish, download the engine once from its official GPLv3 release (progress shown, nothing bundled with ntc), then choose your side, difficulty (easy / normal / hard / master) and clock
- The engine runs locally over UCI: the board locks while it thinks, Undo takes back your move plus the bot reply, Resign uses your colour, and a crashed or timed-out engine plays a legal fallback move instead of stalling the game

#### fake — expanded type catalogue
- 20 new types: `major`, `programming_language`, `adobe_tools`, `os`, `hacker_tools`, `dev_tools`, `emotion`, `weather`, `ai_llm`, `ai_harness`, `computer_brand`, `ram`, `disk`, `cpu`, `gpu`, `school_subject`, `dsa`, `sql`, `nosql` and `website`
- `fake types` groups the new entries under `tech` and `education`; `sql`/`nosql` return database products, `disk` returns drive types and `website` returns popular domains
- `major`, `emotion`, `weather` and `school_subject` now follow `--locale`: ZH/TH/LO emit native-script values and `en_GB` uses British wording such as `Maths`
- `website` grew to ~190 domains, `programming_language` gained `Goose` and `Mojo`, and `ai_llm` / `ai_harness` were refreshed with 2026 models and coding-agent harnesses

#### tutorial — hands-on lessons in a guided sandbox
- `tutorial` (or `ntc tutorial`) now creates `<config>/ntc/tutorial/`, moves the shell there and opens a lesson menu: `cd`, `view`, `ne`, `ral`, `ran`, each a folder with its own fixtures and a `task.txt`
- A lesson never runs the command for you: every step states a goal and is checked from what you actually typed, `tutorial hint` reveals one nudge, `tutorial skip` moves on, and `tutorial status` / `tutorial restart` track the current step
- Tutorial mode restricts the shell: only lab commands run inside the workspace, and stepping outside blocks everything except `cd`, `tutorial`, `help` and `exit`; `tutorial exit` returns to the directory you started from
- The prompt shows `tutorial [lesson step/total]` while the mode is on
- Every step now ends with one extra idea: after `Step complete.` the shell prints `Also try: <command>` (for example `view -d 2` after `view notes.txt`, or `back` after `cd ..`) with a command that stays inside the lab

#### config — `ghost_typing` switch for the idle ghost
- `config ghost_typing [on|off]` (aliases `config gt`, `config ghost-typing`) is a second name for `config idle`: the same switch, so `cfg gt off` stops the idle ghost typing and `cfg gt on` keeps the behaviour unchanged

#### shell — more idle suggestions
- The rotation gained `tutorial`, `ral add`, `ran list`, `gs <text> | sort -u | head 10`, `math 3+4*5`, `qr <text>`, `theme list` and `backup create`, translated in all four bundled packs
- `ran list` only shows when the folder actually has an `NTCRANFILE.toml`, the same context rule already used for `todo add` and `git status`

### Fixes

#### serve — `/info` follows the URL you used
- The `/info` page builds its links from the request `Host` (and `X-Forwarded-Host` / `X-Forwarded-Proto`), so opening it through a cloudflared tunnel shows the tunnel URL instead of `http://127.0.0.1:<port>`; the host is sanitised before being echoed
- `GET` on an existing write-only API endpoint (e.g. `/api/upload`) answers `405 Method Not Allowed` instead of `404`, so endpoint discovery can tell the path exists

#### txtc — `--no-line` honoured by the picker
- The interactive picker used when `txtc` runs without a file ignored `--line`/`--no-line`; it now applies the same override rule as `txtf`, and the renderer is covered by a test so a configured default can never leak line numbers into a `--no-line` copy

#### scan waep — base-relative results, useful help, real errors
- Results are shown relative to the scanned base (`scan waep http://host/api` → `/files`, not `/api/files`); wordlist entries that repeat the base are re-based, and the default list gained `/upload-chunk`, `/create-folder` and `/download-token`
- `405` counts as a hit (the path exists but refuses the method), and recursion builds child paths from the parent base instead of treating the printed path as absolute
- `scan waep` with no arguments, and `scan waep --help`, print usage, examples, every flag and the option notes; the same flag block appears in `help scan`
- Invalid ports/whitespace in the target, an emptied wordlist and a run where every probe failed now report a clear error instead of "no paths answered"

#### shell — the idle hint types itself
- Idle hints wait 30 s on an untouched empty prompt, type the suggestion one character at a time, hold it for 15 s, wipe it and start the next one until a key is pressed
- The `search` suggestion was replaced with `gs` (ntc has no `search` command)

## [2.4.8] - 2026-10-30

### New Features

#### nfe — Find Mode, Command Mode and Live Filter
- `f` now switches the explorer into **find mode**: the directory listing is replaced by every hit below the directory `f` was pressed in, each row painted as a path relative to that root (`src/shell/http.rs`), with the preview panel still following the selection
- Typing narrows the search live; `Tab` switches between typing and walking the results, where `hjkl` and the arrows move, `Enter`/`l`/`Right` open the selected hit (directory: enter it; file: list its parent with the file selected) and `h`/`Left`/`Esc` leaves find mode
- `/` is now a live filter box: the listing (or the find results) narrows as the text is typed, `Enter` keeps the filter and `Esc` clears it
- `g` and `:` open the new command mode with only three verbs, all relative to the current directory: `file <file_name>`, `dir <dir_name>` and `line <+N|-N|N>`. The first word is hinted (`file`/`dir`/`line`), `file ` hints file names, `dir ` hints directories, `line ` hints nothing; `Tab` cycles the hint, `Right`/`Ctrl+E`/`End` accept it, `Enter` navigates (`l` is a shortcut for `line `)
- The editor's `nfe:` prompt gained the same three verbs with the same hints; `g` opens that prompt too
- Changing directory clears the filter, and `S` reverses the directory block as well as the files (size/date/type sorting now falls back to the name, so the direction is always visible)
- Command mode, find mode and the filter box paint a block caret where typing resumes, so `Left`/`Home`/`Delete` edits are visible
- The unused `W` (reopen in Windows Terminal) binding, its help entry and the viewer hint are gone

#### todo — `move` and `replace`
- `todo move|mv <select-n> <target-n>` moves a task: `todo mv 5 1` turns `[1,2,3,4,5]` into `[5,1,2,3,4]`
- `todo replace|rp <select-n> <target-n>` swaps two tasks: `todo rp 5 1` turns `[1,2,3,4,5]` into `[5,2,3,4,1]`
- Both are 1-based, validate the range, no-op on equal indices, and work for every todo format (txt/md/json/toml/yaml/html)

#### txtc — line-number flags
- `txtc --line <file>` / `txtc --no-line <file>` (short: `-l`, `-nl`) override `config linenumbers` for one copy, exactly like `txtf`
- Flags may come before or after the file, and the interactive picker honours them too

#### ping / trace — network diagnostics
- `ping` keeps delegating to the OS tool but now maps `-c/--count`, `--forever`, `-i/--interval`, `-w/--timeout`, `-s/--size`, `-t/--ttl`, `-4`/`-6` and `-q/--quiet` to the platform spelling (unsupported flags warn instead of failing)
- New `trace` (aliases `tracert`, `traceroute`): `trace <host> [-m N] [-w MS] [-q N] [-n] [-4|-6] [--no-fallback]`, using `tracert` on Windows and `traceroute`/`tracepath` elsewhere; without a traceroute it falls back to a TTL walk (`one ping per hop`) and explains that in the output

#### serve — endpoints on startup and an `/info` page
- `serve files` prints the four URLs a client needs: the web UI, the upload endpoint (`/api/upload`), the download base (`/download/root/`) and the new info page
- `GET /info` (aliases `/help`, `/about`) serves a self-contained page explaining what the server is, its mode/auth state, and copy/paste upload and download recipes for the browser, `curl`, `wget`, PowerShell, `ntc up`, `ntc dl` and `ntc http`; the web UI header links to it
- `GET /download` and `/download/` now redirect to `/download/root/`, so the URL printed at startup works when pasted whole

#### scan waep — web path enumeration
- `scan waep <host|host:port|url[/base]>` (alias `scan web`) discovers web paths from a built-in ~120-entry list or `-w <wordlist>`: `scan waep example.com` reports `/api`, `/upload`, `/download`, ... and `scan waep example.com/api` reports `/api/v1`, `/api/v2`, ...
- Flags: `-x/--ext`, `-m/--method GET|HEAD`, `-s/--status`, `--all`, `-r/--recursive`, `-d/--depth`, `-c/--concurrency`, `--timeout`, `-H/--header`, `-A/--user-agent`, `-o/--output`, `-q/--quiet`
- Redirects are reported with their `Location` and never followed; the response body is not downloaded; hits print as `path status ms content-type` with the summary and elapsed time

#### shell — idle-mode teaching hints
- A blank prompt shows one dimmed, **read-only** suggestion that teaches a command (`tp add <dir>  then: @<dir>`, `nfe`, `ne <file> --init`, `view -d 2`, `search`, `todo add`, `serve files --lock`, `help`, ...), picked from the directory context (git repo, existing todo list, empty folder) and rotated once per prompt
- The hint can never be inserted: `Right`, `Tab`, `End` and `Ctrl+E` are no-ops for it on an empty line, so the user always types the command
- `config idle [on|off]` toggles it (`config idle` reports the current state)

#### installers — per-user build for standard accounts
- The shared installer body lives in `packaging/ntc_install.issi`; `installer.iss` and `win7amd.iss` are thin wrappers that define what to package
- `iscc installer.iss` / `iscc win7amd.iss` build the existing all-users installers (elevation requested once, PATH in HKLM); `iscc /DUSERINSTALL ...` builds `-user` variants with an `asInvoker` manifest that install with **no UAC** into `%LOCALAPPDATA%\Programs\ntc` and write only the per-user PATH entry
- Both modes use `DefaultInstallDir` (Program Files vs LocalAppData), `ChangesEnvironment=yes`, `SetupLogging=yes` and `CloseApplications=yes`; the install log records the chosen mode and warns when `ntc.exe` reports a different version than the script
- `ran` gained `build-user`, `build-win7`, `build-win7-user`, `cpwinu` and `cpwin7u`, and `pack`/`rmpack` track the new `-user` artifacts

### Fixes

#### ascii-only UI
- Removed every emoji from the CLI output, help text, translations, installers and both web UIs; pictographs became ASCII (`[ok]`, `x`, `[!]`, `i`, `[dir]`, `[file]`, `[search]`, `*`, `+`/`-`, `^`/`v`), and the file lists in `serve` now show a text badge (the extension, `DIR` or `FILE`) plus word buttons instead of emoji icons
- `nfe`'s help overlay lists find mode, the command verbs and the live filter, and the bottom bar uses the same prompt renderer for `/`, `f` and `g`/`:`
- A `tests/no_emoji.rs` guard now fails the build when an emoji is added back to `src/`, both web UIs or the installers (the `fake emoji` dataset and the East Asian width table are allowed)

### Internal

#### test and build hygiene
- New unit tests cover the todo reorder transforms, the `txtc` argument parser, the `netdiag` argv builders and TTL reply parsing, the `serve` route table and info page, the `webpath` target/wordlist/URL/classification logic plus a local-socket enumeration fixture, the nfe caret rendering and the idle-hint rotation

#### env — NTC Environment Variables
- `config env NAME value` stores a variable in the local `ntconfig.toml`, or in the global config when the current directory has no local file; `-g` forces global, `-l` forces local, `-os` applies only to the current session and `-osg` persists it at the OS level too (`HKCU\Environment` on Windows, `~/.config/ntc/env.sh` on Unix)
- `config env NAME` prints a variable together with the store it came from; `config rmenv NAME` removes it (`-d` is an alias, `-all` clears every store plus the session)
- Lookup precedence is local ntc → global ntc → process/OS environment, so an NTC variable always wins while an unset one falls through to the OS
- PATH and other core variables (`HOME`, `USERPROFILE`, `SYSTEMROOT`, …) are protected: they cannot be overwritten or removed through `config env`
- `showenv` now lists the local, global and session NTC variables as well as the OS environment (`--all` reveals full values, `-d <n>` limits the PATH list)
- Typing `$` in the shell hints every variable that can expand — OS, global ntc, local ntc and session all count — and `$H` completes `$HOME`, `$HOMEPATH`, … (`${` completes the braced form)
- A `$NAME` / `${NAME}` typed on the command line is expanded before the line runs, so `print $PATH` prints the value like `echo $PATH` in a POSIX shell; `ral add`/`ral edit` keep their `$` text literal because those references belong to the alias

#### paths — `~` and Environment-Variable Expansion
- Every path-taking command expands `~`, `$NAME`, `${NAME}` (and `%NAME%` on Windows) before resolving: `cd ~/proj`, `tl $HOME/app.log`, `txtc ${SRC}/main.rs`, `nsheet $DATA/report.xlsx`, `serve --dir $WEB`
- Expansion follows the same precedence as `config env` (local ntc → global ntc → process/OS environment); an unknown name is left literal so the usual “not found” error is shown
- Surrounding quotes are stripped, so `cd "~/My Folder"` works as expected

#### cd / tp — Teleport History on Every `cd`
- Bare `cd` returns to the home directory, and `cd ~` / `cd $VAR` resolve through path expansion like any other target
- Every successful plain `cd` pushes the previous directory onto the teleport history stack, so `cd` participates in `tpb` exactly like a `tp` jump does
- `tp add <name> <path>` expands `~` and variables before saving the savepoint

#### ral — Environment References and Braced Parameters
- In ral templates a bare `$NAME` is now an **environment** reference resolved at run time, while `${x}`, `${x:-default}`, `${x[]}`, `${x[,]}` and `$*{x}` are the parameter forms
- `$x[]` stays accepted as a legacy array parameter for backwards compatibility, and `dbal` keeps bare `$a` as parameters, so existing `ndb` aliases are unchanged
- `ral add` / `ral edit` warn when a bare `$x` looks like an intended parameter and suggest the braced `${x}` form
- Help, the no-arg `ral` output, `ral make` templates and hints document the new reference modes (English, Chinese, Thai and Lao)

#### ran check — NTCRANFILE Interpreter
- `ran check` reads `NTCRANFILE.toml` the way `ran` will and reports **every** problem as `file:line:column`, the offending line and a fix hint; a clean file prints `NTCRANFILE.toml no problem`
- A single `\` where TOML needs `\\` is diagnosed with the corrected line to paste back (`cmd = "packaging\build.cmd"` → `packaging\\build.cmd`), including escapes that are legal TOML but silently become control characters (`\b`, `\t`, `\n`, `\f`, `\r`)
- Stray text sitting where a `key = "value"` line belongs is reported at its own line instead of a bare “failed to parse”
- Also checks reserved target names (`check` is now reserved alongside `init`, `deinit`, `help`, `list`, `ls`), missing dependencies, circular dependencies, targets with no `cmd` and no `deps`, and `$(VAR)` references that are not defined under `[vars]`

#### sys — Monitor Rebuilt Around Panels, Rows and Keys
- `d` shows/hides the disk rows, `p` shows/hides the process list and `c` still toggles the CPU grid; with everything off the body says which key brings a panel back
- The monitor now owns a scrollable body: the response bar sits on the second-to-last row and the hint bar on the last row, with `?=help` as its first entry; `?` opens a key overlay
- Key map: `j`/`Down` and `k`/`Up` move the process selection (and scroll the view instead when the process list is hidden), `x` and `Ctrl+K` kill the selected PID, `g`/`G` jump to the first/last row, `PgUp`/`PgDn` scroll the body a page without touching the selection, `s` cycles the sort column, `S` reverses it, `F` filters, `Enter` expands a tree, `q` quits and `Esc` clears the filter before quitting
- Sorting is now one comparator for roots *and* children, so expanding a tree keeps the active column/direction instead of always ordering children by name; the header marks the active column with `^`/`v` and the hint bar repeats `sort: <col> <dir>`
- The selection is tracked by PID, so it survives refreshes, re-sorts and filtering; if the process disappears the selection clamps to the nearest row
- `j/k` and `PgUp/PgDn` are separate scroll units: with the process list visible the selection is kept on screen, otherwise the keys scroll the whole body
- Body rows are built as sanitised lines (control bytes escaped, tabs expanded) and clipped with ANSI-aware truncation, replacing the byte slices that could panic on non-ASCII process names; killing suppresses the PID immediately and reports it in the response bar

#### editor — Rendering Safety (no ANSI leaks, no ghost rows)
- Buffer text is painted from a cell list that expands tabs to the configured `tab_size` and turns control bytes into visible placeholders (`^[`, `^?`), so a file containing raw escape bytes can no longer drive the terminal
- Cursor, selection, horizontal scrolling, the status-bar column and mouse clicks all use the same display-width maths, so tabs, wide CJK characters and placeholders stay aligned
- Run-panel output is stripped of ANSI colours, escaped and clipped by visible width; diff lines, sidebar names, tab labels, popup titles and the status/hint bars use ANSI-aware truncation instead of byte slicing
- The hint bar's ghost suggestion and prompt caret are clipped the same way, so a long prompt can no longer cut a multi-byte character in half
- Any layout change (resize, sidebar, run or terminal panel, tab switch, popup) triggers a full text repaint, and the alternate screen is cleared on entry, so rows painted under the previous layout no longer survive as ghosts
- Normal-mode hint bar now lists `?=help` first (English, Chinese, Thai and Lao)

#### termux — Termux Integration
- bash hints, bash-completion delegation and the `ntc termux [dir]>` integrated mode now work on Termux/Android instead of being rejected as non-desktop Linux; `bash -c …` and `bash script.sh` keep the one-shot path
- `config integration_shell` reports the Termux environment with its package manager `pkg`, and the missing `bash-completion` hint becomes `pkg install bash-completion`; the scripts are also looked up under `$PREFIX`
- On Linux and Termux a mounted Windows drive no longer offers `.exe` / `.com` files as command hints (WSL's drvfs marks every file executable, so they looked runnable)

#### dl — Pause, Resume and History
- `dl <url>` always shows the progress bar; `--progress` is accepted as a no-op so old notes and aliases keep running
- `dl --history` prints the record table that `--stop` used to print
- `dl --stop` now lists the transfers that are still running and lets the user pick one to stop; the killed record becomes `stopped` and keeps its `.part` file
- `dl --resume` without a URL lists the unfinished transfers (stopped, failed or interrupted, with their staged size) and resumes the one the user picks
- Transfers are staged as `<name>.part` and renamed only on success, so an interrupted download is resumable instead of leaving a truncated file under the real name
- Each download records itself as `downloading` with its PID before the first byte, so a second ntc can see and stop it

#### help — Compact Overview by Default
- Bare `help` now prints one line per command area (`cd`, `config`, `net`, …) with a one-line explanation, instead of every subcommand, note and example
- `help <command>` still shows the full section for that topic, and `help -a` / `help --all` restores the previous long reference
- The compact descriptions are translated (English, Chinese, Thai and Lao)

#### serve files — Root-Relative URLs and Controllable Downloads
- `GET /` redirects to `/root/`, and the browse tree lives under `/root/<dir>`, so changing directory yields `/root/dir_name` and a copied URL reopens the same folder
- Files download from `/download/root/<path>` (for example `/download/root/hello.c`); the bare `/download/<path>` spelling is accepted too, and the old `/api/download?id=…` keeps working
- Downloads honour `Range: bytes=…`, so the web UI can pause a transfer and resume it where it stopped
- While a file is downloading its row shows **pause / cancel / percentage** instead of rename and delete; once the transfer ends (or is cancelled) the row returns to normal
- Both the in-page download and the browser-native link expose **copy link address**, which copies the `/download/root/…` URL

#### views — `--trim`
- `views --trim <text>` (repeatable, `--trim=<text>` works too) removes that text from every name before the theme colour is applied, so entries keep their colour instead of dropping to plain text
- `--trim` also applies to the `...` group separator, so `views --trim ...` prints the same listing as `views | trim ...` — the separator text was previously untouched, which made trims that only match it look like they did nothing
- Matching is case-insensitive, and an entry whose name trims away completely is dropped instead of being printed under its original name

#### gs — Compact Search Header
- `gs` now prints `Searching Contents "pattern" (depth: N)...` with no blank line before the hits, instead of the emoji header; the result rows and the `Content matches (N):` line are unchanged
- Match highlighting honours `NO_COLOR` and `TERM=dumb`, while still staying red when the output is piped or redirected

### Bug Fixes

#### editor — Box Drawing and Symbols Are One Cell Again
- `╔══╗`, `──`, `│`, arrows, `>`/`<` and the other East Asian *Ambiguous* characters are single-width again: a missing upper bound in the width table counted every code point above U+1100 as double-width, so these glyphs were laid out two cells apart
- Because the skipped cell was never painted over, it kept the text of the previous frame — which is why a box-drawing line showed stray characters inside it (`╔r═s=`); each glyph now owns one column and the line repaints cleanly
- Only the East Asian Wide/Fullwidth blocks are still two cells (CJK, Hangul, kana, fullwidth forms, emoji), so tab stops, the cursor, selections, search hits and mouse clicks stay aligned

#### sys — Plain When Colour Is Off, Readable Core Grid
- `setc OFF` (and `NO_COLOR` / `TERM=dumb`) now produces a fully plain monitor: the gauges, header, response bar, hint bar, help overlay and the selection highlight stop emitting colour instead of staying bright
- The CPU grid numbers cores from 1 (`C 1`, `C 2`, …) as the OS does, and each cell is `C 1 [  3%]` followed by a two-space gap, so gauges no longer run together
- The gauge is 17 cells wide for every percentage, so cores keep their columns while the numbers change

#### dns — Lookups No Longer Stall on Dead Resolvers
- One query is now sent to every known resolver at once and the first usable answer wins, on a single 1.5 s budget shared by all of them, instead of 2 s per attempt × 2 retries × N servers: `dns -t MX` drops from ~16 s to ~0.2 s, `-t ALL` answers in ~0.6 s
- Link-local IPv6 resolvers (`fe80::/10`), which need a scope id and could only ever time out, are skipped
- The resolver list is read once per process rather than re-enumerating adapters for every record queried by `-t ALL`
- An authoritative `rcode` answer (NXDOMAIN, REFUSED, …) still returns promptly — a 300 ms grace window lets a resolver that knows an internal name answer `NOERROR` first, instead of waiting out the deadline

## [Unreleased]

### New Features

#### check - Interpret Format Files Before You Run Them
- `ral`, `math`, `igcare`, `theme`, `fake` and `scr` now take `check [file]`, the way `ran check` already did: each format is read the way its command reads it and every problem prints as `file:line:col: message` with the offending line and a hint
- `math check` with no argument validates every `*.ntc.math` file in the current directory; the same holds for `*.ntc.ral`, `*.ntc.igcare`, `*.ntc_theme`, `*.ntc_fake` and `*.ntc`
- `ral check` rejects invalid alias names, names that differ only by case, multi-character `$x[...]` separators, unclosed `${` placeholders and bare `$NAME` refs (which are environment variables at run time), and warns about hint specs with no matching alias
- `igcare check` flags dot-prefixed extensions, path-like entries in the name lists, duplicates and unknown keys
- `theme check` reports TOML colour errors and unknown top-level keys; `fake check` validates bundles (TOML or JSON), column aliases, `ref:` targets and type names
- `scr check` reports unterminated quotes, `set` lines without `=`, empty `@`, unclosed `${` and a trailing `$`
- All seven checks share one reporter, so `ran check` keeps working with the common layout
- Hints: `math <TAB>` ghosts the `.ntc.math` file in the working directory before any subcommand, `<cmd> check <TAB>` offers only that command's format file, and format files complete for `ral import`, `igcare import`, `theme import`, `fake group import`, `math <file>`, `scr <file>` and `ran check`

#### dns - Typed Records
- `dns -t MX|NS|CNAME|TXT|PTR|SOA` is answered by a built-in UDP resolver instead of dumping system `nslookup`/`dig` output, so each type prints only its own records (`dns example.com -t SOA` shows mname/rname/serial/refresh/retry/expire/minimum)
- Unknown record types are rejected with the supported list, and `-t ALL` now covers A, AAAA, MX, NS, TXT and SOA
- `dns -t PTR` accepts an IP literal (reversed into `in-addr.arpa`/`ip6.arpa`) or a domain name; resolvers come from `/etc/resolv.conf` or the Windows adapter list, with `1.1.1.1`/`8.8.8.8` only as a last resort
- `dig`/`nslookup` are used only when no server answers at all

#### up - Multipart Upload, Auth and Transport Fallback
- The URL no longer has to be the second argument: whichever positional starts with `http://`/`https://` is the URL, so `up -d "hi" https://host` and `up https://host file.txt` both work
- Files upload as `multipart/form-data` with the original filename in a `file` field by default, which is what `ntc serve files` `/api/upload` expects; `--raw` sends the bytes as the body and `--put` sends a raw `PUT`
- A `405`/`415`/`501` answer retries once with the other transport and reports which method succeeded (`--no-fallback` disables the retry)
- New `-a/--auth user:pass` (Basic), `--bearer <token>`, `-F/--form name=value` extra multipart fields and `--timeout <seconds>` (default 300)

#### integration_shell — Shell Hints + `ntc ps`
- New `config integration_shell ON|OFF` (default `OFF`, alias `is`): only **installed foreign shells** (`powershell`, `pwsh`, `cmd`, `bash`, `sh`, `zsh`, `fish`) join level-1 ghost completion — arbitrary `PATH` executables are never offered
- Launcher switches complete too: `powershell -` ghost `-NoProfile`/`-Command`, `bash -` ghost `-c`/`--norc`, `cmd /` ghost `/C`/`/K`
- `bash`/`fish` arguments are delegated to their own completion engines; `sh`/`zsh`/`cmd` fall back to ntc's file completion after the launcher
- Typing `powershell` / `pwsh` / `powershell.exe` and pressing Enter enters `ntc ps [dir]>` instead of spawning a nested shell: a persistent PowerShell host executes every line (variables, functions and `cd` survive), while ntc keeps the line editor — its ghost hint, Tab cycle, and Right/End/Ctrl-E accept replace PSReadLine's Tab completion
- `ntc ps` now preserves native PowerShell completion case-insensitively, so `get-c` ghosts `Get-Content`/`Get-Command` and `sel` ghosts `Select-Object`/`Select-String`
- `exit` inside `ntc ps` returns to the normal ntc prompt; `cd` inside PowerShell moves ntc's directory and tree view with it; `cls` still clears the screen; Ctrl-C drops the host (state is lost) so a runaway command can always be aborted
- Delegated completion is cached per line; command names (`g`, `Get-`, `gci`) are answered from a local cache fetched once when `ntc ps` starts, so they never wait on the host
- Ghosts grow in stages: `g` → `Get` → `Get-` → `Get-ChildItem` (and the same boundary rule applies to ntc's own hyphenated candidates); Right/End/Ctrl-E accept one stage, while Tab cycles and previews whole candidate names
- Completion that needs PowerShell itself (parameters, paths, variables) runs in one background worker with latest-request coalescing; a keypress shows the previous answer instead of ever blocking, and stale results are dropped after a PS command changes state
- PowerShell hints and `ntc ps` are now **Windows-only**: a `pwsh` installed on Linux/macOS runs as a normal command and never takes over the prompt
- bash argument hints are **desktop-Linux-only**: `target_os = "android"`, Termux and a Linux binary running under proot are all rejected instead of being hinted
- On desktop Linux, typing `bash` and pressing Enter enters `ntc <distro> [dir]>` instead of spawning a nested shell: the label is the lowercase `ID` from `os-release` (`arch`, `ubuntu`, `alpine`, `debian`, `kali`, `fedora`, …), falling back to `linux`. A persistent bash host executes every line, so variables, functions, aliases and `cd` survive between commands while ntc keeps its line editor
- Completion inside `ntc <distro>` is answered by that live shell, so bash-completion functions, `COMP_WORDBREAKS` (`git checkout:path`, `--opt=value`), options and files behave as in bash; plain command names still come from a local cache so the first keystroke never waits
- `exit` inside the bash mode returns to `ntc [dir]>`, `cd` moves ntc's directory and tree view with it, Ctrl-C drops the host (state is lost) so a runaway command can always be aborted, and `bash -c …`, `bash script.sh` keep the ordinary one-shot path
- On desktop Linux, `config integration_shell` reports the detected distro and package manager (pacman/dnf/apt/apk/zypper/xbps/emerge), and when `bash-completion` is missing it prints the matching install command (`sudo pacman -S bash-completion`, `sudo apt install bash-completion`, …)

#### code_tools — Programming Tool Completion
- New `config code_tools ON|OFF` (default `OFF`, alias `ct`): curated programming binaries, compilers and runtimes that are **installed on `PATH`** join level-1 completion (`g++`, `python`, `java`, `cargo`, `go`, `dotnet`, `kubectl`, `sqlite3`, …); arbitrary executables are never hinted
- Every curated tool gets basic flag completion when the fragment starts with `-`: `g++ -` → `-o`, `-Wall`, `-std=c++17`; `java -` → `-jar`, `-cp`; `python -` → `-m`, `-c`; `node -` → `-e`, `--inspect`
- Common subcommands complete at the first slot (`go build`, `pip install`, `dotnet build`, `terraform plan`, …)
- The rich argument engines for `git`, `cargo`, `npm` and `docker` (branches, crate targets, scripts, containers) now live behind this gate; with it off, `git` falls back to ntc's own git hints

#### txtf — Directory Picker
- `txtf <dir>` now opens the existing file picker for that directory instead of reporting “not found”; selection displays the chosen supported file
- Works with disk paths and teleport paths, including `txtf @tp`, `txtf @tp/subdir` and nested savepoints
- `--head`, `--tail` and `--betw` still apply to the selected file; empty directories report a clear warning
- Hints already traverse these directories; `help txtf` and the tutorial now document the directory form

#### fmt — Plain Text, PDF and Spreadsheet Families
- `fmt` now accepts `.txt` as a document source and target, so plain text converts to Markdown, HTML, XML, DOCX, document JSON and PDF
- PDF output is supported via `fmt <file> to pdf -o <file.pdf>` (write-only; PDF input is rejected with a clear message)
- New sheet family: `.csv`, `.tsv` and `.xlsx` (write-only) convert to CSV, TSV, XLSX, JSON, Markdown or HTML; sheets render as document tables for md/html
- `fmt <file> to json` stays contextual: data files emit data JSON, documents emit document JSON, and sheets emit an array of row objects
- Hints, `help fmt`, usage text and error messages updated in English, Chinese, Thai and Lao

#### where — PATH Lookup
- `where <binary>` now resolves a program through `PATH` like `which`: every match is printed in PATH order, with the first one (the copy the shell would run) marked. PATHEXT is handled on Windows, and repeated PATH entries are shown once
- Bare `where` still prints ntc's executable, config file and current directory; `--help`/`-h` are unchanged

#### rmd — Wildcard Directory Deletion
- `rmd *` removes every subdirectory of the current directory; other `*` patterns work too (`rmd old*`, `rmd parent/*`)
- Bulk deletion always lists the matched directories and asks for confirmation (defaults to No), so a wildcard can never wipe the working directory silently
- Files never match — `rmd` only ever removes directories; literal `rmd <dir>` keeps its existing one-shot behaviour

#### fileops — `@tp` Savepoints for `mkf`/`mkd`/`rmf`/`rmd`
- `mkf`, `mkd`, `rmf` and `rmd` now resolve teleport savepoints in every operand, like `cp`/`mv` already did: `mkf @proj/notes.md`, `mkd @proj/build`, `rmf @proj/*.log`, `rmd @proj/old`
- `*` wildcard operands expand inside the savepoint (`rmf @proj/*.log`, `rmd @proj/old*`), from the savepoint root or any `@tp/sub` path
- Completion follows suit: `mkf`/`rmf` ghost the savepoint's files (directories stay traversable), `mkd`/`rmd` ghost its directories, and every comma-separated slot completes

#### rev — Reverse-Each-Line Filter
- New `rev [file]` pipe filter: prints every line with its characters reversed (Unix `rev`), reading an explicit file or piped/redirected input — e.g. `view app.log | rev`, `rev data.txt`
- Line endings are preserved (a missing final newline stays missing, CRLF stays CRLF) and reversal is per Unicode character, so non-ASCII text stays intact; output is plain and ANSI-stripped on redirect
- Registered as a pipe-stage command: `| ` now ghosts `rev`, and `rev <Tab>` completes text files; documented in `help pipe` (localized in all four languages)

#### trim — Pattern-Removal Filter
- New `trim <pattern> [text]` pipe filter: removes every non-overlapping occurrence of `<pattern>` and prints the result — `trim " ..." "nice ... bro ..."` → `nice bro`
- Reads piped/redirected input instead of inline text, where every argument is a pattern: `echo nice ... bro ... | trim " ..."` gives the same output
- Patterns are case-sensitive substrings; a trailing CR/LF is normalised so the inline and piped forms print the same line
- Registered as a pipe-stage command (`| ` ghosts it, `trim` needs no file completion), documented in `help pipe`, and localized in English, Chinese, Thai and Lao

#### editor — Template Creation, `-I` and Multi-File `ne`
- `ne --init|--super <file> [<file> ...]` now only creates templates; the editor is no longer launched automatically
- Append `-I <file>` (last argument, exactly one file) to create everything and open that one file in the editor; `ne --hyper <name> -I` opens the new project directory
- `--init` and `--super` accept multiple files (space- or comma-separated)
- Plain `ne <file>` now requires an existing file and points at `--init` / `--super` when it is missing
- Editor `:ne --init|--super|--hyper` creates templates/projects without opening a tab; plain `:ne <file>` keeps opening existing files only, one at a time
- Normal `n`, gmode `n` and nfe `n` create file templates without opening a new tab
- New `html` / `htm` templates for both `--init` and `--super`

#### local — Local Config Precedence, `-g/--global` Scope & Split Listings
- **The local config is finally applied**: `ral` aliases, hint specs and hint lists stored in a project's `ntconfig.toml` were written but never read back. They are now merged with the global config everywhere — alias expansion (`run`, `ral`, `ran`, `scr`), completion, `ral ls|info|rnm|cls`, `ral export` and `config watch trigger`
- **Local wins, global keeps working**: a local entry takes priority for the same name, while every global name the local file does not define still works. A local command never modifies the global config
- **`igcare`**: the local file can now *un-ignore* a globally ignored name through the new `cared_directory_names` / `cared_extensions` / `cared_files` lists — `care dir target` in a project works even when the global config ignores `target`, and the global config stays untouched
- **New `-g` / `--global` scope flag** for `ral add|edit|rm|cls|rnm|hintn|import`, `ignore dir|ext|file`, `care dir|ext|file` and `igcare import`: inside a project the default target is the local file, `-g` targets the global config deliberately. The completion hints offer the flag only inside folders that have an `ntconfig.toml`
- **`ignore` / `care` value completion**: `ignore dir|file <Tab>` and `care dir|file <Tab>` ghost the directory/file names in the working directory (bare names — the config matches them verbatim, so no trailing `/`), `ignore ext|care ext <Tab>` ghosts the extensions actually present, and every comma-separated slot completes (`ignore dir src,do<Tab>` → `src,docs`)
- `ral ls` and `igcared` now print a `[global]` and a `[local]` section (the local one shows the file path); a name present in both is marked `(shadowed by local)`, and `igcared` lists the local "cared" rows
- `ral rm` inside a project never removes a global alias — it points at `ral rm -g <name>` instead; `ral cls` only clears the local store
- **`local init --all` removed** (breaking): copying the global settings into the local file replaced the global behaviour with a stale copy. `local init` writes the commented template (now with the `cared_*` blocks and the precedence rule) and shares it with `init --local`; the flag is rejected with a clear message

#### shell — Pipes, Redirection & Logical Operators
- `a | b` pipes one command's stdout into the next command's stdin — built-in ntc commands now participate, not just system commands (e.g. `gs main | say`, `cat log.txt | encode b64`)
- `a && b` runs `b` only when `a` succeeds, and `a || b` runs `b` only when `a` fails (bash semantics); both short-circuit
- Redirection: `cmd > file` (overwrite), `cmd >> file` (append), `cmd < file` (read stdin), and `cmd <<EOF … EOF` heredocs (multi-line; works both interactively at a `heredoc>` prompt and one-shot as `ntc 'cmd <<EOF'` with the body on stdin); here-strings `cmd <<< text` are also supported
- Operators must be **whitespace-delimited** standalone tokens (`cmd > file`), so data that uses `<` / `>` / `|` — e.g. `nsheet data.csv --filter age>30` — is untouched; `<<EOF` is the one exception (delimiter may be attached). Attached forms (`cmd>file`) are still handed to the OS shell for system commands
- Redirection/piped output is captured at the OS handle level (so `println!`, `colored`, and child processes are all included), stripped of ANSI colour codes, then written to the target — redirected files stay clean, plain text
- Shell chatter (`Executing: …`, `completed`) is suppressed while a stage is piped or redirected, so captured output is pure data
- Commands that read stdin: `say` (no text), `math -`, `view -`, and `encode`/`decode` (empty target or `-`); piped process stdin is also consumed when ntc itself is invoked in a pipe (`echo hi | ntc encode b64`)
- Piped/redirected input is now searchable: `cmd | gs <pattern>` and `cmd | qur -s <pattern>` treat the piped text as their content and print matching lines grep-style (case-insensitive substring, identical to normal `gs`), instead of searching the filesystem — e.g. `gs main | gs todo`, `view app.log | gs error`
- New line-window filters `head [N] [file]`, `tail [N] [file]` and `betw A:B [file]` — they print the first/last N lines (default 10) or the inclusive A..B slice, reading an explicit file argument or piped/redirected input (file wins), so they compose in pipelines (`view app.log | head 20`, `gs error | tail 5`, `csvinfo | betw 3:7`); output is plain (no line numbers/header) and ANSI is stripped on redirect
- New `sort [-r] [-n] [-u] [-f] [file]` line filter — sorts piped/redirected text or a file lexicographically (default) or numerically (`-n`), with reverse (`-r`), unique (`-u`) and case-insensitive (`-f`) options, so `gs error | sort -u | head 10` works; output is plain and ANSI-stripped on redirect
- TUI/full-screen commands (`ne`, `sys`, `nms play`, menus) should not be piped or redirected
- New `help pipe` topic (also `help redirection` / `help pipeline`) and a Pipelines section in the interactive help; localized in all four languages

#### ral — `$*{x}` One-Arg Capture (Spaces & Quotes)
- New rest-capture placeholder `$*{x}` (also `$*{x:-default}`): binds a single argument while keeping spaces, so a quoted `hi "wow nice"` expands to `echo wow nice`; when it is the **last** placeholder in the template it also acts as a catch-all, joining every remaining argument (`hi wow nice` → `echo wow nice`)
- Multiple `$*{}` placeholders split positionally — earlier ones bind exactly one argument, only the last consumes the rest (e.g. `-s "$*{title}" -b "$*{body}"` with `"Good Evening" "How are you"` → `-s "Good Evening" -b "How are you"`)
- Positional args are now tokenized quote-aware (a `"…"` span is one argument with the quotes stripped, `\"` is a literal quote); existing `${x}` / `$x` / `$x[]` / `$x[sep]` behaviour is unchanged
- Works with `ral add` defaults (`ral add hi(x=there) "echo $*{x}"` → `echo $*{x:-there}`), completion hints/selects, and is honoured everywhere aliases expand (interactive shell, `ntc <cmd>`, `run`, `ran`, `scr`); the `ral` usage/example strings are localized in all four languages

#### init — OpenCode Scaffold (`init -oc|--opencode`)
- `init --opencode` / `init -oc` — scaffold an OpenCode workspace in the current directory: creates `.opencode/`, `.opencode/tools/` and `.opencode/agents/`, then writes the bundled `tools/ntc.ts` custom tool, the `agents/ntc-plan.md` + `agents/ntc-impl.md` agents, and `.opencode/opencode.jsonc`
- The `ntc` tool lets OpenCode run any one-shot ntc command (search, report, tasks, git, …) in the project; the `ntc-plan` / `ntc-impl` agents are ntc-aware planning and implementation agents usable in any project
- `init --all` (`-a`) now also scaffolds `.opencode/` alongside `NTCRANFILE.toml` and `ntconfig.toml`
- Existing files are left untouched and reported; `--force` (`-f`) re-writes only the template files (never the directory)
- `deinit --opencode` / `deinit -oc` removes the scaffold files and prunes the `tools/`, `agents/` and `.opencode/` directories only when empty, so user files survive; `--force` skips the confirmation
- `init` / `deinit` completion hints updated; new usage strings localized in all four languages
- Both scaffolded agents (`ntc-plan`, `ntc-impl`) now send a completion email via the user's `ntc send-email -s "…" -b "…"` alias as their final step (plain `[ntc-plan]` / `[ntc-impl]` subject prefix); if the send fails they stop and report that the `send-email` SMTP alias may not be set globally

#### fake — Deterministic Fake-Data Generator
- `fake gen <types> [-n N] [-f fmt] [-s seed] [-o file] [-g group]` — generate rows from a comma-separated list of types
- 100+ types across person / contact / address / geo / date-time / numeric / identifier / network / web / finance / company / text / security / file / misc — `name`, `email`, `username`, `city`, `country`, `latitude`, `uuid`, `ulid`, `nanoid`, `ipv4`/`ipv6`, `mac_address`, `user_agent`, `credit_card` (Luhn-valid), `iban` (checksum-valid), `btc_address`/`eth_address`, `price`, `job_title`, `password`, `otp4/6/8/12`, `color_hex`, `semver`, `hash_md5`/`hash_sha256`, `image_url`/`avatar_url`, `isbn`/`ean` (checksum-valid) and more
- `fake types [-s|--search] [pattern]` — grouped listing of every type, with fuzzy search (`fake types emial` → `email`)
- Output formats `json | txt | sql | csv | xlsx` (`-f`, or inferred from the `-o <file.ext>` extension); `txt` is an aligned table on stdout, `sql` emits `INSERT INTO <group> (cols) VALUES …;` and `xlsx` requires `-o`
- `-s/--seed N` makes output reproducible; `-n/--count` defaults to 1
- `fake group add|edit <name> [alias as type, …]` — save column groups where the alias becomes the SQL/JSON column name; plus `ls [glob]`, `info`, `rm`, `rnm <old> <new>` and `clear [-f]`
- `fake gen -g <group> …` generates straight from a saved group (mutually exclusive with `<types>`), columns in left-to-right order
- Groups are stored in the config like `dbal`/`ral`: merged reads (global overlaid by a local `ntconfig.toml`, local wins) and writes routed to the local file when present, else the global one
- Password tiers `password_ez` / `password_mid` / `password_hard` with guaranteed character classes, plus the full hash family `hash_sha1` / `hash_sha224` / `hash_sha384` / `hash_sha512` / `hash_sha3_256` / `hash_sha3_512` / `hash_crc32` (random hex of the matching length)
- `-u/--unique [cols]` — make a whole row unique (bare `-u`) or just the named columns; unsatisfiable constraints abort with a clear error. Rejected together with `-g` (declare uniqueness on the columns instead)
- Group columns take a trailing `!` to stay unique (`contract as phone_number !`) and `ref:<group>.<column>` to borrow values from another group (generated first with the same `-n`, cycle-safe)
- `-l/--locale <code>` — locale-aware `first_name`/`last_name`/`name`/`city`/`state`/`street`/`address`/`phone_number`/`postal_code` pools for `en_US` (default), `en_GB`, `zh_CN`, `th_TH` and `lo_LA`
- `fake group export <name>|--all [-o file]` and `fake group import <file> [--force] [--prefix p]` — share groups as a `.ntc_fake` bundle (TOML, or JSON when the file ends `.json`)
- `fake` / `fake help` / `help fake` usage; subcommand, group-name, type-name and locale completion hints; `fake` reserved as an alias name; fully localized in all four languages
- `email`/`username` now derive from the same row's `name`/`first_name`/`last_name` regardless of column order — `fake gen name email` prints `Kenneth Kelly` next to `kenneth.kelly28@quill19.info`; standalone `email`/`username` still work by generating their own name
- Non-Latin names keep their script for display types but are **romanized to ASCII** for `email`/`username` (e.g. `--locale th_TH` yields `niran.bounmee28@…`, not `นิรันดร์.บุญมี28@…`); each locale carries native+roman name pairs
- `name` respects locale word order (e.g. `王伟` for `zh_CN` instead of `伟 王`)
- New types: `id` (sequential 1..n row id, seed-stable) and `id_card` (locale-aware national ID with a valid check digit — Thai 13-digit mod-11, Chinese 18-char ISO 7064, `en_US` `###-##-####`, `en_GB` NINO shape, `lo_LA` 9-digit)
- `fake locales` — list the supported `--locale` codes with a sample name/city/phone row
- Locale pools enlarged (~30 first/last names, more cities/states/streets per locale) to cut duplicate output

#### nms — NTC Music: TUI Sound Player & Media Scan
- `nms play <file>` — fullscreen sound player: a live waveform of bottom-anchored `O` "candles" colored by level (green low / yellow mid / red high), plus a high/low level meter, elapsed time and remaining-time countdown
- Player keys: `space` pause/resume, `←/→` seek 5s, `↑/↓` (or `+`/`-`) volume, `a` replay from 0s, `q`/`Esc` quit
- When a track finishes the player freezes: the waveform stops drawing `O`s and the animation halts; press `a` to replay from 0s (the screen clears and re-renders) or `q` to quit
- Video files (`mp4`, `mkv`, `webm`, `mov`, `avi`, …) play their **audio track only** — no video is rendered; the header labels them `VIDEO · audio only`
- Graceful on headless machines: when no sound driver/device is available the player prints a clear localized error and returns to the shell instead of crashing (`OutputStream`/device open is wrapped, never panics)
- `nms scan [-d|--depth <n>] [-s|--size] [-l|--long <dur>] [--care]` — lists audio (mp3, wav, flac, m4a, aac, ogg, opus, wma, aiff, …) and video files under the current directory in the same style as `scan ext`, honoring the configured depth and ignore rules (`--care` overrides them)
- `nms scan -l 25s` / `-l 1mn` / `-l 1h` — keep only media **longer than** the given duration; each file's duration is decoded in parallel and shown next to the name, and files with an unreadable/unknown duration are skipped
- `.ts` is treated as TypeScript, not media (MPEG transport streams use `.m2ts`), so `nms scan`/`nms play` no longer pick up TypeScript sources
- `nms` / `nms help` / `help nms` usage; `play`/`scan`/`help` completion and flag hints; `nms` reserved as an alias name; fully localized in all four languages
- Playback is in-process via `rodio` + symphonia (default features cover mp3/wav/flac/m4a/aac/ogg/vorbis/mp4) with `cpal` output — no external player or binary
- Player internals: streaming decode (no whole-file load), a lock-free amplitude tap feeding the UI, and a `TerminalGuard` that restores the terminal on quit or panic; command-level tests need no audio hardware
- Build note: Linux needs ALSA headers (`libasound2-dev` + `pkg-config`); Windows (WASAPI) and macOS (CoreAudio) need nothing extra

#### ne — VSCode-Style MQL Completion
- Two-box popup for `.mq4/.mq5/.mqh` (insert mode): Box1 candidates (12 rows, kind tags) + Box2 detail, one line below the cursor, flipped above when space runs out
- Kinds `[F]`unction `[V]`ar/type `[A]`rray `[K]`eyword/constant `[O]`bject `[S]`truct `[E]`num(+members) `[M]`acro; priority F>V>A>K>O>S>E>M with round-robin fill; user symbols first; case-insensitive prefix filter
- Per-dialect builtins (`.mq4` vs `.mq5`, `.mqh` shows the union); user functions show signature + `//` doc comments; rich builtin docs where available
- Enter accepts (`name()` lands the cursor inside), Up/Down cycle, Esc dismisses, Tab always indents; `(`/`)` overtype-aware with signature help
- Signature help (Box2-sized, above cursor): opens on `(` and follows the cursor inside `(…)` (nesting/string/comment aware), active parameter highlighted and updated on `,`
- Zero new theme keys or language strings (all popup text is code/data)
- Pure priority sort over the full match list with a 12-row scrolling window (no cap); `o.` member completion (methods + vars with access filtering, private excluded); full wrapped signatures everywhere with Yellow active param; ghost-box repaints via dirty discipline; PgUp/PgDn page the selection

#### ne — Vim-Style Keymap Redesign
- Single-letter verbs in Normal: `i` insert, `v` visual, `q` query, `n` save, `c` fast copy, `e` nfe explorer, `f` search, `b` file finder, `t` terminal (Esc/`Ctrl+Q` hides it), `u` encoding picker, `g` gmode, `:` command, `?` help, `0` exit (dirty-checked like `:q`)
- Motion everywhere: `hjkl` + `awsd` + arrows (arrows-only in Insert so typing still works); Shift+direction selects in Visual; `Ctrl+J` enters Jump from all text modes
- New `gmode` prompt (`g`, type, Enter): `t`/`T` next/prev tab, `qt`/`aqt` close tab/all, `<number>` goto line, `q` exit
- New `nfe` keyboard sidebar (`e` to enter): `hjkl`/arrows/`wasd` move, Enter opens dirs/files in place, Esc back
- Nfe `:` size prompt (`size full|def|N%`, persisted percent width); tab-bar tabs are clickable to switch buffers in Normal/Insert/Visual/Nfe without changing mode, with per tab and far-right for close-all; Shift+direction selects in Normal; `H` opens Help; terminal/run panels take keyboard focus on open
- Visual: `a` select-all toggle, `c`/`x` copy/cut, `y` yank line, `p` paste-over-selection; quit is `:q`/`0`/gmode-`q` only
- Mouse is now optional (`mouse_enabled` in `editor.toml`, default on) and covers the text field, sidebar, tab bar (click to switch/close) and terminal panel
- Removed for redesign: hex editor (`:hex`), completion popup UI, `Ctrl+O/W/B/N/G/H`, `Ctrl+Tab` buffers, `Ctrl+PgUp/PgDn` tabs
- Refreshed welcome screen, in-editor hint bars, help screen and tutorial for the new map (all four languages)

#### gs — OR Patterns (`a\|b`)
- `gs` patterns now support **OR**: `gs error\|warning` matches lines containing `error` *or* `warning` (case-insensitive, same substring semantics as a single term). The escaped pipe is the shell-safe form; `gs "error|warning"` and the space-free `gs error|warning` work too, since `|` only splits a pipeline when it stands alone
- Every alternative is highlighted in the results (and in the interactive `-i` pager); at a shared position the longest alternative wins, so `foo\|foobar` never produces nested markers
- Applies to file searches (`search_content`, `search_content_in_files`), piped/redirected text (`cat app.log | gs error\|warning`) and `-p` glob filters keep working (`gs error\|warning -p "*.rs"`); an OR alternative is never mistaken for a path filter. Backslashes that do not precede `|` stay literal, so Windows paths still search fine
- New `gs` usage example (and `help search` row) for OR patterns, localized in English, Chinese, Thai and Lao

#### smtp — HTML & Markdown Bodies
- `-b` now accepts **HTML** and **Markdown** as well as plain text: `-b "<h1 style=text-align:center>Hi</h1>"` is detected as HTML automatically, `-b letter.html` / `-b letter.md` read the body from a file (Markdown is rendered to HTML with GFM tables, strikethrough, autolinks, task lists and footnotes), `-b notes.txt` sends a text file, and `-b -` reads the body from stdin
- New `--body-file <path>` (force the file interpretation for any path) and `--body-format auto|text|html|md` (`-bf`) to force how the body is parsed; a missing body file errors out **before** the SMTP connection instead of mailing the filename
- HTML bodies send as `multipart/alternative` (HTML + a plain-text fallback derived by stripping markup, keeping lists and table cells), so HTML clients render and **align** the body (`align` / `style=text-align` / tables) while plain clients stay readable; HTML fragments are wrapped in a styled, centered mail card, and documents that already contain `<html>`/`<body>` are sent untouched
- Attachments still wrap everything in `multipart/mixed`, now around the alternative body; plain bodies are unchanged (`text/plain` single part)
- `smtp help` gained a Message body section; flag completion hints include `--body-file`, `-bf` and `--body-format`; new strings localized in English, Chinese, Thai and Lao

#### ne — Session Tabs Scoped per Directory
- Editor tabs are now saved **per working directory**: exit `ne` from `A` with tabs open, start the shell in `A` again and those tabs come back; start it in `B` and only `B`'s own tabs (or just the requested file) appear — `A`'s tabs never leak into `B`
- `session.toml` stores one snapshot bucket per directory (up to 24, oldest pruned), so opening another project no longer overwrites the previous project's tab set; the existing `session_max_age_hours` staleness rule and missing-file skipping still apply
- Legacy single-snapshot `session.toml` files load fine (the old unscoped snapshot is discarded rather than shown under the wrong directory)

#### nssh — Native SSH Profiles, Shell, SFTP & Port Scan
- `nssh add <name> --host <h> [--user u] [--port p] [--key path] [--pass-env VAR]` — profiles in `~/.config/ntc/nssh.toml` (0600 on Unix); passwords never stored, only env-var references
- `nssh ssh [name] [--cmd "..."] [--trust] [--ask-pass] [--timeout s]` — pure-Rust `russh` transport (`ring` backend, no C/OpenSSL toolchain): default + explicit key auth, hidden password prompt, OpenSSH `known_hosts` verification (changed keys always fail, unknown keys need `--trust`), one-shot exec with exit-code footer or interactive PTY shell; bare `nssh` / `nssh <name>` picks/opens directly
- `nssh cp <local> <name>:<remote>` / `nssh cp <name>:<remote> <local>` — SFTP upload (chunked, progress bar) / download; remote side detected only for known profile names (Windows `C:\…` stays local); 256MB v1 guard
- `nssh scan <host> [--ports 22,80,8000-8010 | --top N] [--timeout ms]` — parallel TCP connect scan (rayon) with service labels
- `nssh ls|info|rm|edit`, `nssh keygen` passthrough, profile-name + flag completion, `help nssh` topic; fully localized in all four languages
- Interactive shell input layer: local crossterm events translated to canonical xterm bytes (arrows/modifiers, Home/End, PgUp/PgDn, Ins/Del, F-keys, Enter, Esc, Backspace=DEL, Ctrl+Z=0x1A, Alt combos, SGR mouse, bracketed paste) — identical on every Windows terminal; `~.` force-disconnect escape with hint + back-in-ntc banner; local `$TERM` propagated and standard PTY modes sent

#### nsheet — Terminal Sheet Viewer (CSV/TSV/JSON/JSONL)
- `nsheet open <file>` — fullscreen TUI: cell cursor (`arrows`/`hjkl`, `PgUp`/`PgDn`, `g`/`G`), `/` search + `n`/`N` jump, `s` sort prompt (`!` suffix = desc), `f` filter prompt, `c` clear, `e` export to `.json`/`.md`, `Enter` full-cell popup, `q`/`Esc` quit
- `nsheet view <file> [-l <n>|--all] [--sort <col> [--desc]] [--filter <expr>] [--export <json|md> [-o <file>]]` — non-interactive box-table print (reuses `ndb` renderer, default 50 rows)
- Dependency-free parsing: RFC-4180-style quoting, delimiter auto-detect (`, ; tab |`) with `-d` override, header auto-detect with `--header/--no-header`, ragged-row normalization; JSON arrays + JSONL with first-seen key order
- Filters: `col~text`, `col=text`, `col!=text`, `col>n`, `col<n`, `col>=n`, `col<=n` joined by `&&`/`||`; numeric-aware sorting; column by name (case-insensitive) or 1-based index
- Shell completion: `open`/`view` + all flags, sheet-extension file filter (`.csv/.tsv/.json/.jsonl/.ndjson/.txt`); `help nsheet` topic; fully localized in all four languages

#### serve chess — Hotseat Chess Prototype (2-player)
- `serve chess [--port 8000] [--bind addr] [--tunnel] [--share]` — new React app (`ntc_chess_web/`, embedded like `ntc_web`); no `--auth` by design
- Hand-rolled Rust rules engine (no chess crate): legal move gen, check/checkmate/stalemate, castling, en passant, promotion picker, pawn double-push, threefold/50-move/insufficient-material auto-draws, server-side countdown clocks, resign/draw-agreement/rematch, game abandonment
- Server-generated SAN history (`e4 e5 Nf3 O-O Bxf4+ Qxf7#`) with file/rank disambiguation, served per move; text piece glyphs (Q/K/P/N/R/B) with brown×egg board, A–H/1–8 coordinates, yellow last-move, red check, black-dot targets
- Screens: 2 Player, vs AI (soon), Online (soon), Settings (7 sfx volume sliders incl. Check + default clock, persisted), Help, Tutorial; sound engine plays `/sounds/*.wav` and no-ops on 404 so the future PNG/WAV pack drops in with zero code changes
- `chess.db` (next to `serve.db`): settings table live, rooms/players tables reserved for online play
- Media pack wired: 12 transparent PNGs (`public/pieces/wP…bK.png`, 256×256) render on the brown×egg board via the `PieceGlyph` seam; 6 WAVs (`public/sounds/*.wav`) play on move/capture/promote/castle/win events with per-name volume sliders
- `check.wav`: 7th sound for non-capture checks (`sfx.check` setting + Settings slider, default 80); capture+check keeps the capture sound, mate keeps `win`
- Orange "hanging piece" highlight: server computes per-occupied-square attackers-vs-defenders after every move (kings excluded, check stays red) and serves `hot: [...]` in game state; board renders orange below red-check, above yellow last-move
- Hot v2: cheaper-piece-eyes-pricier-piece always glows (pawn→N/B/R/Q, N/B→R/Q, R→Q) even when defended, plus a danger ring around both kings (neighbor squares, even empty, glow only on genuine overloads — the king itself counts as one defender)
- Pins: absolutely-pinned pieces glow gray (`pinned: [...]` in state, below orange in priority so hot never changes color); a pinned piece's attacks count only on its pinner's square and only with real attack geometry behind them
- Captured trays render actual piece images (strongest first, Q R B N P); promotion picker shows the mover's-color piece images; material lead (`+N`, P1 N3 B3 R5 Q9, counted from the live board so promotions stay exact) sits beside the leading side's clock
- Undo takebacks (one ply per press, hold to rewind with reverse-slide animation, works after game over too) via snapshot stack restoring board/history/trays/clocks/status; Flip swaps to Black's perspective (persisted per-device, 2P screen — future AI reuses it)
- Clickable history: any played move previews its frozen board/trays (view-only, board locks, Now returns to live); served from the snapshot stack via `GET /api/chess/position?ply=K`
- Tap-deselect + hold-drag: re-pressing a selected piece clears it; holding lifts the piece to follow the pointer (touch-safe via pointer capture), dropping on a target moves, elsewhere snaps back — purely client-side, so opponents never see it
- Drop reliability: drops route through fetch-then-route (a press that outruns the dots fetch over a tunnel still moves); drag renders at most once per frame with memoized squares so phones don't jank
- Per-device sessions: each browser owns an isolated hotseat game via `ntc_chess` cookie (laptop and phone no longer share a lobby); 30-min idle sweep + 64-session cap, zero frontend changes (same-origin cookies); also the future Online foundation (named joinable rooms instead of cookies)
- HTTP keep-alive (chess): persistent per-connection request loop (HTTP/1.1 default, explicit `close` honored, 100-request cap, 15s idle timeout, Ctrl+C-aware) so tunnel polls/moves reuse one warm connection instead of re-handshaking every second
- Theme refresh: wK.png favicon + menu logo, gray background with silver borders, real Back-button navigation (inline-SVG chevron) on all screens
- Snappy ~140ms slide animation: mover ghosts from→to on every move (castling ghosts the rook too, promotions slide as pawns), captures vanish instantly, `prefers-reduced-motion` snaps
- Stale-poll guard: in-flight polls abort on every mutation and late responses that rewind the move count (or un-terminate a finished game) are dropped — the previous move's ghost can no longer replay before the current one
- Optimistic move ghost: the slide starts on click from the local board (no POST wait), holds at the destination until the response confirms it (pixel-identical handoff), and snaps back with a notice only on error; rapid moves replace mid-flight ghosts, resign/draw/new/rematch clear everything

#### serve http-server — Read-Only Static Server for the CWD
- `serve http-server [dir] [--port 8000] [--bind addr] [--auth user:pass] [--tunnel] [--share [--qr|--no-qr]]` — python-`http.server`-style hosting of the current directory (or `[dir]`): GET/HEAD only, directory listing (python-style table, dirs first) + streamed file downloads with `mime_guess` types
- Path-traversal safe: per-request canonicalization must stay under the served root (covers `..`, `%2e%2e`, symlink escapes) else `403`; missing paths are `404`; non-GET/HEAD is `405`
- `--auth` gates every route (no login page for a raw file server); `--spa/--web-dir` rejected with a hint (they belong to `serve files`)
- `serve` is now a dispatcher: `serve files` (existing file-drive UI, unchanged) and `serve http-server`; bare `serve` runs nothing and prints the service list (`files|http-server|chess|config`); the old `serve [port]` legacy alias is removed — use `serve files`; `serve chess` (online multiplayer) is stubbed as coming soon
- Bare `serve files` / `serve http-server` (no args) start with configured defaults instead of printing help; per-service help examples updated to the `serve <service>` form
- `serve http-server` prints a python-style access log per request (`127.0.0.1 - GET /hello.txt 200`, always on, http-server only) and shows a clean path in the Serving banner (no `\\?\` prefix on Windows)
- `serve config [files|http-server] [--port-default N] [--bind-default ADDR]` (alias: `config serve …`) — per-service port/bind defaults persisted in `config.toml` (`[serve.files]`, `[serve.http-server]`, `[serve.chess]`); bare call shows all services, bare service name shows one; both servers use them when `--port`/`--bind` are absent; `showcg` gains `Serve files:`/`Serve http:`/`Serve chess:` rows; shell completion included; old configs without `[serve]` fall back to 8080/0.0.0.0
#### image display --sixel — Force Sixel Rendering
- `image display|show <path> [--sixel]` — flag accepted before or after the path; forces the sixel encoder regardless of terminal detection, with a warning when detection says otherwise; bare `image display --sixel` keeps the interactive picker; two-path and unknown-flag errors unchanged; shell completion (including after the path) + help in all four languages

#### serve --share — Internet-First LAN Sharing with QR
- `serve [port] --share [path] [--qr|--no-qr]` — advertisement-only; auth/routing unchanged
- Step 1 checks internet via quick TCP probes (`8.8.8.8:53`, `1.1.1.1:53`, ~900ms each); offline prints localhost only with a localized warning (no LAN, no QR)
- Step 2 (online only) discovers the primary LAN IPv4 via the default-route UDP trick (std-only, Termux-safe), prioritizes `192.168.x.x` > `10.x.x.x` > `172.16-31.x.x`, prints `http://<lan>:<port>` plus a terminal QR (reuse of `qr` renderer); `--bind 127.0.0.1` warns LAN peers can't reach it
- Fully localized in all four languages; hinter completion for `--share/--qr/--no-qr`

#### scan — Extension-Filtered File Listing
- `scan ext <ext...> [-d|--depth <n>] [-s|--size]` — list files by extension (space- and/or comma-separated, dots optional, case-insensitive); no line counting
- Bare output is paths only; `-s` adds human sizes plus a total footer; `-d` limits walk depth (default: config depth)
- Respects ignore/care like `tl`; results sorted alphabetically; fully localized in all four languages

#### view -ext — Filtered Tree
- `view -ext rs c` / `view --ext rs c` — tree shows only matching files, in any flag position (`view -ext rs -ds -d 2`, `view src -ext rs`)
- Empty directories are pruned; `-ds`/`-fs` sizes become sums of kept files; zero matches prints a localized "No .rs, .c files found"
- Extension hinter for `scan ext`, `tl ext` and `view -ext` (comma-aware: `rs,c` completes after the comma)

#### arc — Archive Manager
- `arc create <out.zip|tar.gz|tgz|tar> <files...>` — create archives (dirs recursed, quote-aware paths)
- `arc list <file>` — list contents with human sizes + total count
- `arc extract <file> [dest]` — extract with zip-slip guard (`enclosed_name` / `unpack_in`); default dest strips extension next to the archive
- Shell completion: `arc create <out>` output slot stays hint-free, deeper positions complete files+dirs; `arc list`/`extract` filter to `.zip`/`.tar`/`.tar.gz`/`.tgz` (+dirs for traversal), extract dest completes dirs only; `arc create --help` shows usage + example
- `view` flag ghosts: `view -d` → `-ds`, `view --d` → `--depth`/`--dirsize`; path completion now works after flags (`view -ds <path>`)
- Fully localized (`cmd.arc.*` / `help.arc.*`) in English, Chinese, Thai and Lao

#### todo v2-lite
- Tasks stored as `- [ ]` / `- [x]` lines in per-CWD `future_todo.txt`; legacy bare lines auto-migrate on next save
- `todo done <N>` / `todo undo <N>` — toggle with already-done/open hints
- `todo edit <N> "new name"` — rename + reprioritize in place
- Priority suffix `!` / `!!` (e.g. `todo add "Ship !!"`) — red/yellow tags in list view
- New localized help (`help.todo.done/undo/edit`) in all four languages

####  ndb — NTC Database Shell
- `ndb` — nested interactive shell to "crack" open `.db` files (SQLite)
- On launch it auto-lists every `.db`/`.sqlite`/`.sqlite3` file in the current directory with its size and table count
- `interface <file.db>` (or a list number) opens a database: the prompt switches from `ndb [<cwd>]>` to `ndb [<file.db>]>` and the table tree is shown immediately
- Database-level commands: `help`, `viewf` (schema tree of tables + columns), `tables`, `tablef <table>` (rows, like `txtf`), `schema <table>`, `sql <query>`, `interface`, `exit`
- All databases open **read-only**: `SELECT`, `PRAGMA`, `EXPLAIN`, `WITH` and `VALUES` are allowed, writes are blocked at the keyword check and again by SQLite (`SQLITE_OPEN_READ_ONLY`)
- Row output is a width-aware box table (50 rows by default, `-l <n>` / `--all` to change), with `NULL`, BLOB and truncation indicators
- `exit` closes the file (back to `ndb [<cwd>]>`), a second `exit` returns to the ntc shell; `Ctrl-D` does the same
- Ghost-text hints + `Tab`/`Shift-Tab` completion (commands, `.db` files, table names, `FROM`/`JOIN` tables); `clear`/`cls` re-shows the current context
- Schema insight: `info` (file/engine/object counts), `schema <table>` shows indexes + foreign keys + triggers, `indexes [table]`, `tables [pattern]` filter
- Query UX: `more`/`next` pagination, per-query timing footer, multi-line `sql` mode (end with `;`)
- `dbal` — third alias dimension (`qal` editor / `ral` shell / `dbal` ndb): save named read-only queries (`add|edit|rnm|rm|info|list|run|cls`), run via `dbal run <name>` or the bare name; stored global + local `ntconfig.toml`
- `dbal` params (`$x`, `${x}`, `${x:-default}`, ral-style): `dbal run q 2026-01-01`, `q('2026-01-01')`; missing values error, `;`-smuggled statements rejected before execution
- `&&` sequencing in `ndb`: `interface app.db && tables && top_users -l 5` (quote-aware for `'` and `"`; state updates apply between segments, `exit` short-circuits)
- Safe writes (opt-in, additive): `edit on|off|status` reopens the handle read-write in place (default stays `edit off`, fresh `interface` opens always start locked); `write YES <INSERT|UPDATE|DELETE|REPLACE ...>` runs one row-level statement in a transaction (unconfirmed `write` only previews, `sql`/`dbal` stay read-only forever, DDL stays blocked); auto-backup `app.db.bak.<timestamp>` before the first write per session + manual `backup`/`bak`
- Fully localized (`ndb.*` / `help.ndb.*`) in English, Chinese, Thai and Lao

#### serve --auth — Login for the file server
- `serve files 8080 --auth admin:555@ntcstorage` — every `/api/*` route requires HTTP Basic login (split on the first `:`, so `@` and later `:` stay literal); static `/` stays public so the login page always loads
- Wrong/missing credentials get `401 {"error":"Unauthorized"}` + `WWW-Authenticate: Basic realm="ntc serve"`; the check runs before any body is read, so unauthenticated uploads can't burn bandwidth or disk
- Web UI: login dialog (`Login required` → username + password → `Unlock`), credentials kept in `sessionStorage` (cleared on tab close + `Logout` button), every fetch/XHR carries the header, downloads go through authenticated fetch-blob, any `401` re-opens the login overlay
- No `--auth` = exactly the old behavior (additive-only); localized (`cmd.serve.*`) in English, Chinese, Thai and Lao

### Localization

#### views — Help & Errors in Chinese, Thai and Lao
- `help views` and every `views` error message (`invalid size`, `missing --sort`, `empty directory`) are now translated in Chinese, Thai and Lao instead of falling back to English
- Adds a regression test that fails when those packs silently inherit the English text

### Bug Fixes

#### trim — stdin colors are removed
- `trim` now strips ANSI escapes (SGR colors, OSC-8 hyperlinks, other ESC strings) from piped/redirected stdin before applying patterns, so `some-tool --color | trim "…"` produces plain text
- The shared ANSI stripper used by pipeline capture learned OSC/DCS/APC termination, so hyperlink URLs no longer leak into redirected output
- Inline `trim <pattern> <text>` stays literal; CRLF and newline handling is unchanged

#### npm installer — Termux/Android install regression (2.4.5)
- `npm install -g @nuengcoder/ntc` on Termux no longer bails out with `ntc: unsupported platform android-arm64.` Termux's Node reports `process.platform === 'android'` (not `'linux'`), which the 2.4.5 rewrite stopped matching once the explicit `android-arm64` mapping was dropped
- Android detection now accepts both `process.platform === 'android'` and `'linux'` (plus the kernel release string, `PREFIX` and install path), and maps Node arches to the Rust target assets: `arm64` → `aarch64-linux-android`, `x64` → `x86_64-linux-android`, `arm` → `armv7-linux-androideabi`
- Asset selection is now an ordered candidate list: the Android build first, then the same-arch Linux build. A missing/renamed manual asset on android-x64/arm falls back to the Linux one instead of hard-404ing, and a 404 no longer burns the three download retries
- New `NTC_NPM_ASSET=<filename|url>` escape hatch to force a specific release asset, and the failure messages now print `platform` / `arch` / `os.release()` / `PREFIX` diagnostics
- `packaging/npm/scripts/selftest.js` gains a platform→asset matrix covering Termux arm64/x64/arm, Termux reporting `linux`, Linux arm64/x64, Windows x64/arm64, macOS arm64/x64, unsupported platforms and the env override, so the 2.4.5 regression can't return silently

### Build & Packaging

#### Android — API 26 requirement for AAudio
- The Android target now builds with `--platform 26` (Android 8.0+) instead of `--platform 23`; `cpal 0.18.2` uses AAudio and its NDK dependency requires `api-level-26`, so the linker could not find `libaaudio.so` in the older NDK sysroot
- `sanitize_sheet_name` is now compiled only for non-Android targets, matching `write_xlsx`, which removes the Android dead-code warning
- `android.txt` documents the NDK/API 26 prerequisite

#### Linux — ALSA build prerequisites
- Linux builds (and `cargo deb`, which builds the release binary) require `pkg-config` and ALSA development headers for `nms` audio (`cpal` → `alsa-sys`); run `sudo apt-get install build-essential pkg-config libasound2-dev` on Debian/Ubuntu
- `linux.txt` now lists Debian/Ubuntu, Fedora and Arch package commands, and `install.sh` fails early with the correct command instead of a pkg-config panic

## [2.4.0] - 2026-07-22

### Major Features

#### ntpkg — Package Manager for Language Runtimes
- `ntpkg install <lang> [version]` — Download & install any supported runtime
- `ntpkg uninstall <lang> [version]` — Remove an installed runtime
- `ntpkg list` — Show all installed runtimes
- `ntpkg use <lang> <version>` — Activate a specific version
- `ntpkg which <lang>` — Show path to the active runtime
- `ntpkg available [lang]` — List supported languages or available versions
- `ntpx <pkg> [binary] [args]` — Run any installed package's binary directly
- **Supported packages (30+):** python, node, git, gh, java, kotlin, swift, php, zig, go, rust, msys2, ruby, lua, dotnet, flutter, kubectl, helm, kind, kustomize, docker, docker-compose, act, scala, opencode, mimocode, kilocode, omniroute, cloudflare, sqlite, sqlitestudio
- Platform-aware downloads with automatic arch detection
- Version management with interactive selection menus

#### Command Architecture Redesign
- `backup` — unified command with subcommands:
  - `backup create` — create snapshot (replaces `bkup`)
  - `backup restore` — interactive restore (replaces `pldw`)
  - `backup undo` — undo last restore (replaces `unpd`)
  - `backup diff` — show diff vs backup (replaces `diff`)
  - `backup clear`, `backup list`, `backup recreate`
- `report` — unified reporting with subcommands:
  - `report txt`, `report json`, `report md`, `report html`, `report pdf`, `report docx`, `report xlsx`
  - Replaces top-level `txt`/`json`/`md`/`html`/`pdf`/`docx`/`xlsx` commands
- `config` — consolidated configuration:
  - `config show`, `config open`, `config reset`, `config restore`, `config local`, `config init`, `config deinit`

#### Windows PATH Management
- `usrpath show` / `syspath show` — display PATH entries with numbered indices
- `usrpath add <path>` / `syspath add <path>` — add directory to PATH
- `usrpath rm <n|path>` / `syspath rm <n|path>` — remove by index or path
- `usrpath move <n> <m>` / `syspath move <n> <m>` — reorder PATH entries
- `mode admin` — re-launch ntc as Administrator
- `mode normal` — return to normal user mode

### New Shell Commands

#### QR Code
- `qr` — generate QR codes from text or data
- `qsc` — scan and decode QR codes from images

#### Text Utilities
- `encode` — encode text to various formats
- `decode` — decode from various formats
- `cvcode` — convert between encoding formats
- `fmt` — format structured data (json, yaml, toml)

#### Productivity
- `todo` — simple task management (add, list, done, clear)
- `clipboard` — clipboard operations (copy, paste, clear)

#### Network & System
- `serve` — HTTP file server with React web UI (browse, upload, download)
- `smtp` — SMTP email client (send, compose with attachments)
- `http` — HTTP client (GET, POST, PUT, DELETE with headers/body)
- `dl` — file downloader with progress bar
- `up` — file uploader
- `dns` — DNS lookup tool
- `ping` — ICMP ping utility
- `sys` / `top` — real-time system monitoring dashboard (CPU, memory, processes)

### Editor Enhancements
- **Hex Editor Mode** — binary file editing with hex/ASCII side-by-side view
- **Hyper Template** — full C project scaffolding with DSA library, OS abstraction, debug utilities
- **Rich Template System** — extensible template engine for code generation
- **Terminal Mode** — integrated terminal panel within editor
- **Welcome Screen** — improved intro with keybinding reference
- **Framebuffer Renderer** — optimized rendering pipeline
- **Template Management** — `template edit`, create, list templates

### MQL Module
- Mini Query Language for data extraction and analysis
- MQL check, extract, hover, and lexicon tools

### Infrastructure & Internal
- `ntc --upgrade` — self-upgrade mechanism (checks GitHub releases, downloads & replaces binary)
- GitHub Actions CI workflow — automated build and test on push
- Cross-platform termios patches for terminal handling
- `rust-toolchain.toml` — pinned Rust toolchain version

### Breaking Changes
- `bkup` → `backup create` (alias removed; use `backup create`)
- `pldw` → `backup restore` (alias removed; use `backup restore`)
- `unpd` → `backup undo` (alias removed; use `backup undo`)
- `diff` → `backup diff` (alias removed; use `backup diff`)
- `txt` → `report txt` (alias removed; use `report txt`)
- `json` → `report json`
- `md` → `report md`
- `html` → `report html`
- `pdf` → `report pdf`
- `docx` → `report docx`
- `xlsx` → `report xlsx`
- `showcg` → `config show`
- `opencg` → `config open`
- `resetcg` → `config reset`
- `restorecg` → `config restore`
- `gencg` → `config local`
- `setO` / `setD` / `setL` / `setC` / `setA` / `setT` / `setH` → `config set <key> <value>`

### Refactoring & Code Quality
- Editor core split into submodules: clipboard, edit, nav, save, search, tabs, tests, undo
- Editor modes refactored into dedicated modules: command, finder, gosc, help, insert, jump, normal, query, run, search, terminal, visual
- Render system modularized with framebuffer support
- Output system split into capture, clipboard, display, and theme modules
- Config system refactored into `utils::config::globals` and `utils::config::mod`
- Theme manager extracted into `utils::theme::manager`
- Shell commands modularized (30+ command modules)

### Bug Fixes
- Various editor rendering and cursor positioning fixes
- Cross-platform clipboard handling improvements
- Backup system atomicity and error handling
- PATH normalization (trailing slash, case-insensitive dedup)
- Watcher debounce and filetype classification fixes
- Syntax highlighting edge cases for Pascal, lock files, and Inno Setup

---

## [2.3.0] - 2026-06-15

### Added

#### ntcEditor Enhancements

**Editor Camera & Cursor**
- Editor camera now always follows the primary cursor (auto-centers vertically)
- `Ctrl+M` in Insert Mode — move cursor to middle of current line
- Pre-tokenize look-ahead for next 10 lines beyond visible viewport (smoother scrolling)
- Enter key smart indent — copies leading whitespace from previous line

**Welcome Screen**
- Replaced `.scratch` with `welcome.ntcEditor` — shows intro text with keybindings
- Welcome screen displays getting started guide, navigation, and file operations

#### Theme System Enhancements

- `theme edit` (no name) — opens theme directory in system file manager
- `theme where` — show theme directory location and list theme files
- `theme make [name]` — create theme template in current directory
- Theme directory support for Windows, macOS, Linux, Android

#### gosc Continuous Navigation

- `v` — view with care and files (depth 1)
- `s` — show total directory size with care
- `vs` — view with care, files, and sizes (depth 1)
- Output zone display below gosc menu for view/size results

#### Shell Commands

- `ral make [name]` — create run alias template in current directory
- `igcare make [name]` — create ignore/care template in current directory
- Ignore/care commands now print success messages and info tips
- `theme make [name]` — create theme template in current directory

#### Image to Document

- Basic image2doc feature for converting images to text/markdown/json
- Android support for `termux-open` in image viewer

#### Internal Improvements

- Generic `InteractiveMenu` helper for consistent UI across commands
- Unified report commands (txt/json/md/html/pdf/docx/xlsx)
- Unified ignore/care command functions
- Split `render.rs` (1424 lines) into modular files (mod/bars/panels/popups)

### Fixed

- Down key behavior — now moves cursor down normally (reverted broken change)
- Home key now syncs horizontal scroll (`scroll_x = 0`)
- Cut (Ctrl+X) and backspace now properly mark dirty regions for re-render
- `theme make` command no longer conflicts with theme name switching
- Ignore/care commands now provide feedback after operations
- Clippy warnings fixed (identical if/else blocks, redundant closures)

---

## [2.2.0] - 2026-06-13

### Added

#### ntcEditor — Major Enhancements

**Multi-Tab System**
- Full multi-tab editor with tab bar, tab switching (Ctrl+PgUp/PgDn), and tab close buttons ()
- Scratch buffer auto-open — closing the last tab creates a new scratch instead of quitting
- Buffer switching via Ctrl+Tab / Ctrl+Shift+Tab (recently opened files stack)
- Crash recovery with periodic SHA-256-hashed snapshots and auto-recovery prompt
- Persistent editor sessions — cursor position, scroll, buffer stack restored on re-entry
- File finder (Ctrl+P / Ctrl+N) — fuzzy file search across project, opens in new tab

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
- Sidebar file explorer (Ctrl+B) with expand/collapse folders, directory icons (+/-)
- Sidebar vertical scroll bar with `█`/`░` thumb/track at rightmost column — click-to-jump and mouse wheel support
- Multi-cursor editing: Ctrl+D to add cursor at next occurrence, Ctrl+G to jump through matches
- Editor config (`editor.toml`) with `auto_save`, `syntax_enabled`, `color_enabled` settings
- Auto-completion popup for `.ntc.math` files with Tab/Enter/Esc navigation

#### Wildcard File/Directory Search
- `fs`, `ds`, `fsc`, `fgo`, `locate` now support wildcard patterns (`*`, `%`, `_`, `$`)
- `*` or `%` matches any sequence, `_` matches single char, `$` anchors at end
- Examples: `fs *.rs`, `ds src$`, `fs main_*`
- When wildcards detected, uses glob-like matching instead of exact/partial/fuzzy tiers

#### New `gs` Command — Grep-Like Content Search
- `gs <pattern> [-d <n>]` — search file contents recursively with case-insensitive matching
- Results show file path, line number, and truncated matching line
- Respects ignore/care settings and supported file types

#### `ran` — Make-like Native Task Runner
- `ran <target>` — execute targets with dependency resolution from `NTCRANFILE.toml`
- `ran init` / `ran deinit` / `ran list` / `ran help` — subcommand management
- `ran help` shows template and usage docs
- Topological sort for dependency chains with cycle detection
- Variable expansion via `$(VAR_NAME)` syntax
- Rich TOML template with comments and examples
- Integrated with top-level `init`/`deinit` commands

#### Fully Configurable Theme System
- **1110-line theme engine** with `ThemeManager` singleton for live theme switching
- `theme` command: interactive menu + direct mode (`list`, `current`, `<name>`, `add`, `rm`, `edit`, `info`, `export`, `import`, `rnm`, `reload`)
- **15 syntax colors**: keyword, string, comment, number, type, builtin, function, operator, punctuation, attribute, macro_token, regex, tag, constant, normal
- **20 shell colors**: prompt, success/error/warning/info, tree, separator, help, teleport, alias, command_output
- **26 editor colors**: backgrounds (editor, gutter, status, hint, sidebar, run_panel), text, cursor, selection, search matches, border, scrollbar, sidebar items
- **3 built-in themes**: `default` (dark), `light`, and `example` template
- Theme export/import to `.ntc_theme` files
- Live theme switching — instant redraw without restart

#### Configuration & New Commands
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
- **Tab close button () click detection**: off-by-1 offset per tab due to `full_w` = `label_chars + 2` vs actual rendered width `label_chars + 1` — each tab's close zone now aligns correctly
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