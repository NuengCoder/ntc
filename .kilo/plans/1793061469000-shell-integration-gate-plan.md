# Implementation Plan: `config integration_shell on\|off` — the gate for foreign-shell completion

Todo item (`future_todo.md`, task 2):

> config integration_shell on|off (default off, if on is will turn on the hinter,
> tab, right key completion for that shell like `p` <--here now could hint
> `powershell` after `powershell ` <could-hint-something>

## Why this is the right thing to do first

Tasks 3–12 in `future_todo.md` each say *"(hinter + tab change hint + right
completion)"* for one environment:

> powershell · cmd · zsh · bash · termux · fedora_sh · debian_sh · arch_sh ·
> fish_sh · kali_sh

Tab-cycling and Right-arrow/End acceptance are **already generic** and need no
per-shell work: `TabCycleHandler` only advances an index
(`hinter.rs:2148-2177`) and Right/End just return `Cmd::CompleteHint`
(`entry.rs:63-82`, `entry.rs:190-198`). Everything a shell integration needs is
*which words `candidates_for()` returns*. So if task 2 builds

1. the **gate** (`config integration_shell on|off`, default off), and
2. the **registry** the ten integrations register into,

then each of tasks 3–12 collapses into one table entry plus its own tests and
language keys, instead of ten separate passes through the hinter, the config
command, the config listing and the i18n tests. Building them in the other order
would mean writing that plumbing ten times, then retro-fitting a gate around it.

## Interpretation of the todo line

Decoded against the code:

- **`integration_shell`** is not a shell *name*; it is a boolean master switch.
  Default **off** means the shell adds nothing to completions — byte-identical to
  today's behaviour for a user who never touches it.
- **"for that shell"** = for whichever foreign shell the typed command names, not
  for a shell selected in advance.
- **`p` → hint `powershell`** = level 1: with the gate on, foreign-shell command
  names join the top-level command candidates, so the existing prefix filter
  (`hinter.rs:1953-1962`) turns `p` into `powershell`.
- **`after powershell <could-hint-something>`** = level 2: once the command word
  is complete, the hinter offers *that shell's* own verbs/flags.
- **"hinter + tab change hint + right completion"** = the three existing surfaces
  (ghost text, Tab cycling, Right/End accept) must all see the new candidates.
  They will, automatically, once `candidates_for()` returns them.

## Current state (verified)

**Where completion comes from**

| Surface | Location |
|---|---|
| ghost text entry point | `hinter.rs:3124` `impl Hinter for NtcHinter::hint()` |
| candidate list | `hinter.rs:1938` `NtcHinter::candidates_for()` |
| level-1 command names | `hinter.rs:18` `builtin_commands()` → `hinter.rs:3109` `build_command_set()` (cached in `NtcHinter::commands`, refreshed via `ALIAS_GENERATION`) |
| level-2+ verbs/flags | `hinter.rs:957` `subcommands_for(parts)`, consulted in the flags branch (`hinter.rs:2017`) and the general branch (`hinter.rs:2105`) |
| Tab cycling | `hinter.rs:2148` `TabCycleHandler`; bound in `entry.rs:186-193` |
| Right / End / Ctrl+E accept | rustyline default + `entry.rs:63-82` `AcceptHintOrEnd`; bound in `entry.rs:195-198` |
| live config read per keystroke | already done: `hinter.rs:3125` reads `Config::global_get_autosuggest_enabled()` |
| config-toggle test harness | `Config::with_test_config(cfg, …)` — used at `hinter.rs:3352`, `3392`, `3458`; `hint_test_config()` at `hinter.rs:4080` |

**The config-toggle pattern** (mirror `git`, which is the closest precedent since
it is also an "integration"):

| Step | Location |
|---|---|
| field + serde default | `utils/config/mod.rs:232` `git_integration_enabled`, `mod.rs:419` `default_git_integration_enabled()`, `mod.rs:504` |
| getter / setter | `utils/config/globals.rs:227-235` |
| `config <name>` dispatch arm | `shell/commands/config_commands.rs:30` |
| handler fn | `config_commands.rs:189` `config_git()` |
| listing line in `config` | `config_commands.rs:44` `t!("help.config.git")` |
| `showcg` label | `config_commands.rs:~870` `t!("msg.showcg.label_*")` |
| language keys | `lang/en/help/config.rs`, `lang/en/msg.rs` |
| key-presence test | `lang/mod.rs` — one `const *_KEYS` + one `#[test]` per feature, e.g. `GIT_KEYS` / `git_keys_exist_in_all_languages` |

**Shell/OS detection available today**

- `utils/terminal.rs:124` `TerminalCapabilities { color, is_terminal, is_termux }`
  and `terminal.rs:184` `termux_detected()`.
- `ntpkg/manager.rs:17` `is_termux()` and `utils/output/clipboard.rs:12`
  `is_termux()` — the same check exists in **three** places.
- **No distro detection exists anywhere.** `fedora_sh`, `debian_sh`, `arch_sh`
  and `kali_sh` cannot be told apart without new code.

**Where foreign commands actually run**

`shell/commands/mod.rs:401` `run_system_command()` hardcodes `cmd /C <args>` on
Windows and `sh -c <args>` elsewhere; `capture_system_command()` at
`mod.rs:443` repeats it. There is no notion of *which* shell, and
`editor/terminal.rs:495` states foreign shells are deliberately never spawned.
So "integration" here is **completion only** — it must not change how commands
are executed.

## Design

### 1. The gate — `config integration_shell on|off`

Exactly the `git` pattern, no new mechanism:

- `utils/config/mod.rs`: `pub shell_integration_enabled: bool` with
  `#[serde(default = "default_shell_integration_enabled")]` returning `false`,
  plus the field in the `Default`/constructor literal — so existing `config.toml`
  files keep loading and stay off.
- `utils/config/globals.rs`: `global_get_shell_integration_enabled()` /
  `global_set_shell_integration_enabled(bool)`.
- `shell/commands/config_commands.rs`: dispatch arm
  `"integration_shell" | "ishell" | "shell" => config_integration_shell(subargs)`;
  handler mirrors `config_git` (line 189) — bare prints status, `ON`/`OFF` set +
  print, anything else prints the error key.
- Config listing: one line `println!("  {}", t!("help.config.integration_shell"));`
  in the listing block near `config_commands.rs:44`.
- `showcg`: one label row next to the other `msg.showcg.label_*` rows (~line 877).
- Language keys in all four packs (en is master; th/lo/zh may fall back):
  `help.config.integration_shell`, `msg.setIShell.status`,
  `msg.setIShell.on`, `msg.setIShell.off`, `msg.setIShell.error`,
  `msg.showcg.label_integration_shell`.
- `lang/mod.rs`: a new `const ISHELL_KEYS: &[&str]` + `#[test]
  fn ishell_keys_exist_in_all_languages()` following the existing
  `GIT_KEYS` test, so a missing translation fails CI instead of leaking a key.

Naming note: `config shell` would collide conceptually with the existing
`config qsc` / `config serve` noun-style subcommands. Recommend the primary name
be `integration_shell` exactly as the todo writes it, with `ishell` as the short
alias and no bare `shell` alias.

### 2. The registry the ten integrations plug into

New module `src/shell/integrations/mod.rs` (registered in `shell/mod.rs`). One
descriptor per environment; the ten follow-up tasks each add exactly one entry:

```rust
pub(crate) struct ShellIntegration {
    /// Command word that opens this shell's completion context (`powershell`).
    pub name: &'static str,
    /// Other spellings that resolve to it: `pwsh`, `ps`, or a full path's stem.
    pub aliases: &'static [&'static str],
    /// Tokens offered after `<name> ` (subcommands / verbs).
    pub verbs: &'static [&'static str],
    /// Tokens offered for a `-` / `--` fragment once `<name> ` is complete.
    pub flags: &'static [&'static str],
    /// Environments the entry applies to; `Any` for a plain user-typed shell.
    pub applies_to: &'static [Env],
}

pub(crate) fn all() -> &'static [ShellIntegration]
pub(crate) fn is_available(i: &ShellIntegration) -> bool   // uses section 4
pub(crate) fn names() -> Vec<String>                        // level-1 pool
pub(crate) fn candidates_for(parts: &[&str], last_frag: &str) -> Vec<String>
```

The registry starts **empty** in this task. An empty registry plus the gate is a
complete, shippable, behaviour-preserving change: turning `integration_shell` on
with no entries registered adds no candidates at all. Each of tasks 3–12 then
adds one `ShellIntegration` literal and one focused test.

### 3. Hinter wiring — the only code path that changes

All of it inside `NtcHinter::candidates_for()`, guarded by one live config read
(the `autosuggest` precedent at `hinter.rs:3125`), so the toggle applies on the
very next keystroke with no cache invalidation and no REPL restart:

```rust
let shells_on = Config::global_get_shell_integration_enabled();
```

- **Level 1** (the `parts.len() == 1 && !trailing_space` branch, ~`hinter.rs:1953`):
  after filtering `self.commands`, also filter `integrations::names()` with the
  same `starts_with` + longer-than-typed rule and merge. Deliberately *not* added
  to the cached `build_command_set()`: that cache is rebuilt from
  `ALIAS_GENERATION` and merging shell names there would need the toggle to bump
  a counter. Keeping it in the live path is both simpler and correct when the
  user flips the flag mid-session.
- **Level 2+**: extend in the two places `subcommands_for()` is consulted — the
  `-`/`--` fragment branch (~`hinter.rs:2017`) for `flags`, and the general
  branch (~`hinter.rs:2105`) for `verbs` — by calling
  `integrations::candidates_for(parent_parts, last_frag)` and extending `out`.
- **Resolution**: match `parent_parts[0]` on its file stem, lowercased, against
  `name` then `aliases`, so `powershell`, `powershell.exe` and
  `C:\Windows\System32\...\powershell.exe` all resolve.
- **Do not** touch `pipe_stage_commands()` (`hinter.rs:2972`): a foreign shell is
  not a pipe filter, and authoritative prefixes (`|`) should stay unchanged.
- **Do not** touch `run_system_command()` / `capture_system_command()`. The gate
  changes what is *suggested*, never what is *run*.

### 4. Why Tab, Right and End need no changes

The three surfaces are already candidate-agnostic:

- ghost text renders `candidates_for()`'s current pick (`hinter.rs:3124-3147`);
- `TabCycleHandler` only mutates `CycleState.index` and returns `Cmd::Repaint`
  (`hinter.rs:2153-2177`), bound to Tab/BackTab at `entry.rs:186-193`;
- Right-arrow is rustyline's own `CompleteHint` when a hint is showing, and
  End/Ctrl+E are routed to the same `Cmd::CompleteHint` via `AcceptHintOrEnd`
  (`entry.rs:63-82`, `entry.rs:195-198`).

so "hinter + tab change hint + right completion" is satisfied for all ten
environments by feeding the registry. This task should assert it **once**, with a
hinter test that a registered entry produces ghost text, survives a Tab cycle,
and is accepted — rather than re-asserting it ten times later.

### 5. Environment detection (shared by the five non-plain-shell entries)

`powershell`, `cmd`, `zsh`, `bash` and `fish` are plain user-typed commands and
apply anywhere. `termux`, `fedora_sh`, `debian_sh`, `arch_sh` and `kali_sh` are
*environments*, and `utils/terminal.rs` only knows `is_termux` — there is no
distro detection anywhere in the tree today.

Add one small helper rather than a fifth copy of the termux check:

```rust
// src/utils/environment.rs  (new)
pub enum Env { Any, Windows, Linux, MacOs, Termux, Fedora, Debian, Arch, Kali }
pub fn os() -> Os                    // cfg!(windows) / macos / linux
pub fn is_termux() -> bool           // existing PREFIX check, moved here
pub fn distro() -> Option<Distro>    // /etc/os-release ID= + ID_LIKE=
```

`/etc/os-release` `ID=`/`ID_LIKE=` covers all four families
(`fedora`/`rhel`, `debian`/`ubuntu`, `arch`, `kali`). The three existing
`is_termux()` copies (`ntpkg/manager.rs:17`, `utils/output/clipboard.rs:12`,
`utils/terminal.rs:184`) should delegate to it — but that consolidation is
optional here and can be left to the task that first needs distro data, to keep
this task's diff to the gate + registry + hinter wiring.

## Files

| File | Change |
|---|---|
| `src/utils/config/mod.rs` | `shell_integration_enabled` field, serde default `false`, constructor literal |
| `src/utils/config/globals.rs` | getter + setter, next to `git_integration_*` (line 227) |
| `src/shell/commands/config_commands.rs` | dispatch arm + `config_integration_shell()`, listing line, `showcg` row |
| `src/shell/integrations/mod.rs` | **new** — `ShellIntegration`, `all()`, `names()`, `candidates_for()`, `is_available()` |
| `src/shell/mod.rs` | `mod integrations;` |
| `src/shell/hinter.rs` | live gate read; level-1 merge; two level-2+ call sites; unit tests |
| `src/utils/environment.rs` | **new** (optional in this task) — `Env`, `is_termux()`, `distro()` |
| `src/shell/mod.rs` / `src/utils/mod.rs` | register the new modules |
| `src/utils/config/lang/{en,zh,th,lo}/help/config.rs` | `help.config.integration_shell` |
| `src/utils/config/lang/{en,zh,th,lo}/msg.rs` | `msg.setIShell.*`, `msg.showcg.label_integration_shell` |
| `src/utils/config/lang/mod.rs` | `ISHELL_KEYS` + key-presence test |
| `future_todo.md` | tick task 2 when done |

## Tests to add

1. `hinter.rs` — gate **off**: a registered shell name is *not* offered
   (`p` → no `powershell`), and `powershell ` offers nothing. Uses
   `Config::with_test_config(hint_test_config(), …)`.
2. `hinter.rs` — gate **on**: `p` → `powershell`; `powershell -` → that entry's
   flags; `powershell <verb-frag>` → its verbs.
3. `hinter.rs` — path/basename resolution: `powershell.exe` and an absolute path
   resolve to the same entry.
4. `hinter.rs` — the ten-way contract asserted once: for a registered entry, the
   candidate appears as ghost text, survives `TabCycleHandler` (index advances
   and wraps) and is what Right/End would accept.
5. `integrations/mod.rs` — with the registry empty, every function returns an
   empty pool (so shipping the gate alone cannot change behaviour).
6. `config_commands.rs` — `config integration_shell` prints status, `on`/`off`
   set and report, junk reports the error; round-trips through the config file.
7. `lang/mod.rs` — the new key-presence test.

## Verification

1. `cargo check --all-targets` clean.
2. `cargo test --lib` — full suite (1095 tests as of the lang refactor) plus the
   new ones; the `hinter` module has a large existing suite that must stay green,
   especially `output_redirect_never_hints`, `pipe_stage_filters_complete_their_flags_and_files`
   and `help_completes_canonical_topics_only`, which pin the level-1/level-2
   behaviour this task edits.
3. `cargo test --lib -- lang` — i18n key presence in all four packs.
4. Manual, via the built binary with a throwaway config dir
   (`APPDATA=<tmp> ntc.exe`): `config integration_shell` → `OFF`; type `p` at the
   prompt → no `powershell` ghost; `config integration_shell on`; type `p` → ghost
   `powershell`; `powershell ` → nothing until an entry is registered; `Tab` cycles
   without inserting; `→`/`End` accepts.
5. Confirm the default is off for a **pre-existing** config file (no
   `shell_integration_enabled` key) — this is the one real upgrade risk.

## Risks

- **Upgrade path**: the field must be `#[serde(default)]`-backed; a missing key in
  an existing `config.toml` must not fail to parse. Covered by test 5.
- **Level-1 noise**: once entries are registered, shell names compete with ntc's
  builtins in the same candidate pool. `filter_prefix` keeps it strictly
  prefix-matched, and the gate is off by default, so the blast radius is a user
  who opted in.
- **Name collisions**: `cmd` (task 4) and `fish`/`bash` may already exist as
  builtins or as user aliases. `build_command_set()` merges aliases and
  `dedup_sorted` runs at the end, so duplicates collapse; the integration must
  only ever *add* candidates and never shadow an ntc builtin's own semantics.
- **Windows-only entries**: `cmd` is meaningful only on Windows. `applies_to` +
  `is_available()` keeps it out of the pool elsewhere rather than offering a
  command that cannot run.
- **Scope creep**: this plan deliberately keeps execution untouched. If the intent
  is instead that `integration_shell` should change *which* shell runs unknown
  commands (`run_system_command`'s hardcoded `cmd /C` / `sh -c`), that is a
  different, larger change and should be a separate item — see question 1 below.

## How tasks 3–12 plug in

For each of `powershell`, `cmd`, `zsh`, `bash`, `termux`, `fedora_sh`,
`debian_sh`, `arch_sh`, `fish_sh`, `kali_sh`:

1. one `ShellIntegration { name, aliases, verbs, flags, applies_to }` literal in
   `integrations/mod.rs`;
2. a test that its verbs/flags appear under the gate and disappear without it;
3. language keys only if the integration prints text of its own (these entries
   are completion-only, so most need none).

No changes to `hinter.rs`, `entry.rs`, the config command or the config listing.

## Open questions

1. **Is `integration_shell` completion-only, or should it also change execution?**
   The todo's example is purely about hinting, so this plan assumes completion
   only and leaves `run_system_command()` alone. If the intent is that ntc should
   hand unknown commands to a *chosen* shell instead of the hardcoded `cmd`/`sh`,
   the toggle needs a shell-name form too (`config integration_shell powershell`)
   and switches become a different task.
2. **Should the five environment entries be gated on detection at all?**
   Detecting distros means new `/etc/os-release` parsing; alternatively
   `fedora_sh`/`debian_sh`/`arch_sh`/`kali_sh` are just user-typed names like
   `zsh`, and only `termux` uses detection (which already exists).
   Recommendation: treat all ten as user-typed names for completion purposes, and
   let detection decide only *availability* where a command genuinely cannot run.
3. **Should the value survive `config integration_shell` with no argument in
   `showcg`?** The plan adds a `showcg` row for parity with the other toggles;
   say so if that is noise.
4. **Alias spelling** — `ishell` as the short form, or something else?
