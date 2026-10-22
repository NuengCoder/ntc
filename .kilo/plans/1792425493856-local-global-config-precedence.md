# Local config precedence, explicit global scope, source-aware listings

Fixes `future_todo.txt` item 1: *"local feature have some problem cause the prompt didn't follow the local config at all but aleast global still work"*.

## Root cause (verified in code)

- `Config::global_get_run_aliases()` (`src/utils/config/globals.rs:278`) returns the **global map only**. Nothing ever reads `LocalConfig.run_aliases` back.
- `Config::get_run_aliases_with_source()` (`src/utils/config/globals.rs:1176`) has a doc comment claiming a merge that does not exist; it returns the global map plus a bool.
- Every consumer therefore sees global only: alias expansion (`src/shell/alias.rs:1306`, `:1611`), `ral ls/info/rnm/cls` (`src/shell/commands/alias_commands.rs:537,583,665,760`), completion (`src/shell/hinter.rs:208,213,425,491,2558`), export (`src/shell/helpers.rs:741,753`), `config watch trigger` (`src/shell/commands/config_commands.rs:447`).
  → **A local alias is write-only: it is saved to `ntconfig.toml` and never used.**
- igcare *does* merge (union) via `Config::get_ignored_dirs_combined()` etc. (`src/utils/config/mod.rs:604-676`), but a local `care dir X` cannot cancel a global ignore of `X` → "care" looks broken inside a project.
- `local init --all` (`src/shell/commands/config_commands.rs:1004,1024-1095`) copies the global settings into the local file, i.e. it *replaces* global behaviour with a stale copy. Removed per request.

## Decisions (confirmed with the user)

1. **Local wins on name collision.** The global entry is kept intact in the global config, still listed under `[global]`, and becomes active again when the local entry is removed.
2. **Local `care` un-ignores a globally ignored name, for that project only.** The global config file is never modified.
3. **The global store is only reachable through an explicit `-g` / `--global` flag.** Inside a project directory every default write goes to the local file. The hinter suggests `-g|--global` **only** when the cwd has an `ntconfig.toml`.
4. **`local init --all` (and `-a`) is removed.** `local init` always writes the commented template.
5. **`ral ls` and `igcared` print `[global]` and `[local]` sections.**

## Precedence model (the single source of truth)

Two stores, unchanged locations:

| Store | Struct | File |
|---|---|---|
| Global | `Config` (`src/utils/config/mod.rs:120-239`) | `dirs::config_dir()/ntc/config.toml` |
| Local | `LocalConfig` (`src/utils/config/mod.rs:800-812`) | `<cwd>/ntconfig.toml` |

**ral (maps, per key):** effective = global overlaid by local → local wins, global entry preserved.
**igcare (sets, per dimension):**

```
effective_ignored(X)  = (global_ignored ∪ local_ignored) − local_cared
effective_extra(X)    =  global_extra ∪ local_extra ∪ local_cared
```

`local_cared` is the new "un-ignore here" list. It is the only way a local file may beat a global ignore, and it never mutates the global file.

## Task list

### 1. Config model — `src/utils/config/mod.rs`

1. Add three fields to `LocalConfig` (`Option<Vec<String>>`, same style as the existing ones):
   `cared_directory_names`, `cared_extensions`, `cared_files`.
2. Rework the effective-set helpers to the model above (each currently unions global+local):
   - `is_dir_ignored` (`:552`), `is_extension_ignored` (`:569`), `is_file_ignored` (`:587`)
   - `get_ignored_dirs_combined` (`:604`), `get_ignored_extensions_combined` (`:619`), `get_ignored_files_combined` (`:634`)
   - `get_extra_supported_extensions_combined` (`:649`), `get_extra_supported_files_combined` (`:664`)
   Each must subtract the local cared set from the ignore union and add it to the extra union.
3. Add per-source getters for display (raw, not effective):
   `Config::global_igcare_sets()` / `Config::local_igcare_sets()` returning an `IgcareSets` struct
   `{ ignored_directory_names, ignored_extensions, extra_supported_extensions, ignored_files, extra_supported_files, cared_directory_names, cared_extensions, cared_files }`
   (all `HashSet<String>`; the global one always has empty cared sets — the global scope has no layer above it).

`FormatConfig::from_both()` (`src/utils/filetype.rs:126`) is built from these getters, so every tree/walk/report picks up the new semantics with no change.
Change `is_supported_format()` (`src/utils/filetype.rs:200`) to use `from_both()` instead of `from_global()` so single-file checks (`txt`, `report`, hinter) also honour the local config. Leave `from_global()` itself untouched.

### 2. Scope-aware writes — `src/utils/config/globals.rs`

1. Add:
   ```rust
   #[derive(Debug, Clone, Copy, PartialEq, Eq)]
   pub enum ConfigScope { Auto, Global }   // Auto = local file if cwd has one, else global
   ```
2. **Rename the routed `local_*` write family and add a `scope: ConfigScope` parameter** (the `local_` prefix is now wrong — the scope decides the target). New names:
   - `add_run_alias(name, cmd, scope)`, `remove_run_alias(name, scope)`, `update_run_alias(name, cmd, scope)`, `clear_run_aliases(scope)`
   - `add_run_alias_hint` / `remove_run_alias_hint` / `rename_run_alias_hint`
   - `add_run_hint_list` / `remove_run_hint_list` / `rename_run_hint_list`
   - `add_ignored_dir` / `remove_ignored_dir` / `add_ignored_extension` / `remove_ignored_extension` / `add_ignored_file` / `remove_ignored_file` / `add_extra_supported_extension` / `add_extra_supported_file`
   - **new** `add_cared_dir` / `add_cared_extension` / `add_cared_file` (local scope only; with `Global` scope a "care" is expressed as *remove from the global ignore set* + *add to the global extra set*)
   - `import_run_aliases_from_file(path, scope)`, `import_igcare_from_file(path, scope)`
   The `global_*` helpers stay as they are (they are the global-store primitives).
3. `remove_run_alias` must report where the alias lived so the caller can print the right message: return an enum `RemovedFrom { Local, Global, NotFound }`.
4. `add_cared_*` in the local scope must also remove the name from the matching local ignore list; `add_ignored_*` in the local scope must remove the name from the matching local cared list (they are mutually exclusive, last command wins).
5. **Invariant to enforce everywhere**: a write with `scope = Auto` inside a project directory never touches `config.toml`. Only `scope = Global` writes the global file, and it is only reachable from the explicit `-g/--global` flag.
6. New read helpers:
   - `get_run_aliases_merged() -> HashMap<String,String>` (global overlaid by local)
   - `get_run_aliases_by_source() -> (HashMap /*global*/, HashMap /*local*/)`
   - `get_run_alias_hints_merged()` and `get_run_alias_hints_by_source()`
   - delete `get_run_aliases_with_source()` (`:1176`) and its false doc comment; update its 2 call sites.

### 3. Flag parsing helper

Add a small helper (e.g. `src/shell/commands/scope.rs` or a function in `alias_commands.rs` shared with `ignore.rs`):

```rust
pub(crate) fn split_scope_flag(args: &str) -> (ConfigScope, &str)
```

Rules: strip a **leading** standalone `-g`/`--global` token, else a **trailing** standalone token; return the rest untouched. Quoted command bodies are safe (`"ls -g"` does not end with a bare `-g` token, and the leading-token rule only fires before the alias name).
Invalid scope flags (e.g. `--glob`) fall through to the normal "unknown argument" handling.

### 4. `ral` commands — `src/shell/commands/alias_commands.rs`

- `cmd_ral` (`:334`): parse the scope flag for `add`, `edit`, `rm`/`remove`, `cls`/`clear`, `rnm`/`rename`, `hintn add|rm|rnm`, `import`.
- `add` / `edit`: write to the scoped store. When writing to the global store while a local alias of the same name exists, print a note that the local one takes priority.
- `rm`: in a project dir (Auto scope) remove **only** the local entry; if the alias exists only globally, print
  `Alias 'x' is defined in the global config, not in this project's ntconfig.toml. Use 'ral rm -g x' to remove it from the global config.`
  and do **not** touch the global store. If a local entry was removed and a global entry with the same name still exists, note that the global one is active again.
- `cls`: clears the scoped store only. The confirm prompt must name the store being cleared (local vs global). Never clears the global store implicitly.
- `rnm`: renames in the store that owns the alias (local first). If the alias is global-only and scope is Auto → error suggesting `-g`.
- `ls` / `list` (`:582`) — **new two-section layout**:
  ```
  ==================================================
  [global]
  ==================================================
    1. btr -> cargo build --release
    2. py  -> python test.py

  ==================================================
  [local]  C:\path\to\ntconfig.toml
  ==================================================
    1. test -> cargo test
  ```
  - `[global]` = the global map, `[local]` = the local map; both headers always printed.
  - If there is no `ntconfig.toml` in the cwd, the `[local]` section prints `(no local config in this directory)`; if the file exists but has no aliases, it prints `(none)`.
  - Numbering restarts at 1 per section; entries sorted by name.
  - A name present in both sections is marked `(shadowed by local)` next to the global entry.
  - Hint specs (`## spec`) come from the merged hint map.
- `info` (`:647`): show the effective command, its source store (`local (ntconfig.toml)` / `global`), and — when the name exists in the other store — that other entry marked as shadowed.
- `export --all|--select` → export the **effective merged** set (+ merged hints + referenced hint lists). Read-only, no scope flag.
- `import <file>` → Auto scope; `-g` → global.
- `hintn add|rm|rnm` → scoped; `hintn ls` keeps its current merged list but its title should follow the new `[global]`/`[local]` section style (or state that the list is merged).

### 5. igcare commands — `src/shell/commands/ignore.rs`

- `cmd_ignore` / `cmd_care` (`:38`, `:77`): parse the scope flag (leading or trailing standalone token) and route through the scoped helpers.
  - Local scope: `ignore` adds to the local ignore list and drops the name from the local cared list; `care` adds to the local **cared** list and drops it from the local ignore list → the name is not ignored in this project even if the global ignores it.
  - Global scope (`-g` or no local config): `ignore` adds to the global ignore set; `care` removes the name from the global ignore set and (for ext/file) adds it to the global extra-supported set, so "care" genuinely un-ignores in the global scope too.
  - The global file is never written by a default (Auto) command inside a project.
- `cmd_igcared` (`:116`) — **new two-section layout**:
  ```
  [global]
    Ignored directories: build, node_modules, target
    Ignored extensions: log
    Extra extensions: (none)
    Ignored files: (none)
    Extra files: (none)

  [local]  C:\path\to\ntconfig.toml
    Ignored directories: dist
    Cared directories: target
    Ignored extensions: (none)
    Cared extensions: log
    Extra extensions: (none)
    Ignored files: (none)
    Cared files: (none)
    Extra files: (none)
  ```
  - Values sorted and comma-joined (replaces the current `{:?}` set dump, e.g. `{"target", ".git"}`).
  - `[local]` always printed; `(no local config in this directory)` when there is no local file, `(none)` for empty categories.
  - The `Cared *` rows exist only in the local section.
- `igcare export --all|--select` (`src/shell/helpers.rs:971,994`): export the effective sets **plus** the local cared lists, so a round-trip preserves the semantics. `write_igcare_file` / `write_igcare_file_selective` gain the three `cared_*` keys.
- `igcare import <file>`: scope-aware; the importer (`globals.rs:1443`) must understand the new `cared_*` keys.

### 6. Runtime reads — switch to the merged maps

| File | Line | Change |
|---|---|---|
| `src/shell/alias.rs` | 1306, 1611, 1635 | `global_get_run_aliases()` → `get_run_aliases_merged()`; hints → `get_run_alias_hints_merged()` |
| `src/shell/hinter.rs` | 208, 213, 425, 491, 495, 2558 | same |
| `src/shell/commands/alias_commands.rs` | 537, 583, 665, 708, 716, 730, 760 | same (+ per-source variants where the source matters) |
| `src/shell/commands/config_commands.rs` | 447 | `config watch trigger` validates against the merged map |
| `src/shell/helpers.rs` | 741, 753, 886, 925 | export + hint-list reference scan use the merged maps |
| `src/shell/commands/alias_commands.rs` | 37, 58, 887 (helpers.rs) | hand-merged hint scans → use the merged getter |

### 7. `local init` / `init --local` — `src/shell/commands/config_commands.rs`

- `cmd_local_init` (`:1001`): delete the `export_all` branch (`:1004`, `:1024-1095`). Always write the commented template. If `--all`/`-a` is passed → print an error explaining the flag was removed and that `local init` is the template command; do not write the file.
- `cmd_local_help` (`:1225`): drop the `local init --all` line; document the new precedence (local first, global untouched) and the `-g/--global` flag.
- `cmd_init` local branch (`:1408-1467`): use the same template text as `local init` (single source of truth — extract one `const LOCAL_CONFIG_TEMPLATE: &str` and use it in both places).
- Template content: drop the "For quick setup, run: local init --all" line, add the commented `cared_directory_names` / `cared_extensions` / `cared_files` blocks, and state the precedence rule in the header comment.
- `cmd_config` help block (`:65`) and `src/shell/help.rs:183`: drop the `help.config.local_init_all` entry.

### 8. Hinter flag hints — `src/shell/hinter.rs`

`subcommands_for` (`:855`):
- `"ral"` with `parts[1]` in `add|edit|rm|remove|cls|clear|rnm|rename|import` → return `["-g", "--global"]`.
- `"ral" "hintn"` with `parts[2]` in `add|rm|remove|rnm|rename` → same.
- `"ignore"` / `"care"` with `parts[1]` in `dir|ext|file` → same.
- **Only when `Config::get_local_config_path().is_some()`** (the cwd has an `ntconfig.toml`). In a directory without one the flag is not suggested (the default target is already global there).
- `"local"` (`:1116`) → drop `"--all"`, `"-a"` from the hint list.

### 9. i18n — `src/utils/config/lang/{en,zh,th,lo}`

New keys (all four packs): `alias.ral.section_global` = `"[global]"`, `alias.ral.section_local` = `"[local]"`,
`cmd.igcared.section_global`, `cmd.igcared.section_local`, `cmd.igcared.cared_dirs`, `cmd.igcared.cared_exts`, `cmd.igcared.cared_files`,
`alias.ral.none`, `cmd.igcared.none`, `cmd.igcared.no_local`, `alias.ral.shadowed`,
`alias.ral.global_only_hint`, `alias.ral.global_active_again`, `alias.ral.local_priority_note`,
`alias.ral.scope_flag_help`, `cmd.ignore.scope_flag_help`, `cmd.care.scope_flag_help`,
`cmd.config.local.init_all_removed`, `cmd.config.local.precedence_hint`.

Removed keys (delete from all four packs and stop using them): `help.config.local_init_all`,
`cmd.config.local.created_export`, `cmd.config.local.export_valid`, `cmd.config.local.export_ignores`,
`cmd.config.local.export_extras`, `cmd.config.local.export_aliases`, `cmd.config.local.template_all_hint`,
`alias.ral.list_title_global`, `alias.ral.list_title_local`, `cmd.igcared.title_combined`, `cmd.igcared.title_global`, `cmd.igcared.combined_hint`.

Add a `LOCAL_SCOPE_KEYS` list + a `local_scope_keys_exist_in_all_languages` test in `src/utils/config/lang/mod.rs` (the existing per-feature pattern), asserting the new keys exist in en/zh/th/lo.

### 10. Tests

Existing infra: `src/utils/test_support.rs` → `TempCwd` (chdir + fixtures), `IsolatedConfigHome` (isolated global config), `Config::with_test_config` (global singleton swap); `src/utils/output/capture.rs` → `with_captured_output` for asserting printed output.

- `src/utils/config/globals.rs` tests (extend the existing module, `local_ntconfig()` helper at `:1542`):
  - a local alias is visible through `get_run_aliases_merged()`
  - a local alias wins over a global one with the same name, and the global map is unchanged
  - a default (Auto) write inside a project does not modify the global config file
  - `scope = Global` writes to the global config
  - `remove_run_alias` in a project reports `Global`/not-found and leaves the global entry intact
  - a local cared dir/ext cancels a global ignore
- `src/utils/config/mod.rs` tests: `is_dir_ignored` / `get_ignored_dirs_combined` respect the local cared list.
- `src/shell/commands/alias_commands.rs` (new `#[cfg(test)] mod tests`): `ral ls` output contains `[global]` and `[local]` and lists both aliases; `igcared` output contains both sections and the cared row.
- `src/shell/commands/config_commands.rs`: `local init` writes the template; `local init --all` is rejected and writes nothing.
- `src/shell/hinter.rs`: `-g`/`--global` is hinted inside a local dir and not hinted outside one.

### 11. Docs

- `future_todo.txt`: tick item 1.
- `CHANGELOG.md`: new entry — local aliases now resolve, `-g/--global` scope flag, `[global]`/`[local]` listings, `local init --all` removed (breaking), local `care` can un-ignore a global ignore.
- `example.ntc.igcare`: add the commented `cared_*` keys; `src/shell/commands/ignore.rs:225` (`igcare make` template) likewise.

## Validation

```
cargo fmt
cargo clippy --all-targets --all-features
cargo test --all-features
```

Manual smoke test (in a scratch dir with a `ntconfig.toml`):

1. `ral add -g g1 "echo global"` → global config; `ral add l1 "echo local"` → local file.
2. `ral ls` → `[global]` lists `g1`, `[local]` lists `l1`; the global file on disk still contains only `g1`.
3. `g1` runs the global command, `l1` runs the local one.
4. Add `g1` to the local file too → `ral ls` shows `g1` in both sections, marked shadowed; running `g1` uses the local command; the global file is unchanged.
5. `ral rm g1` in the project → removes only the local entry, prints the note that the global one is active again; `ral rm -g g1` removes the global one.
6. `care dir target` in the project where the global ignores `target` → `target` is no longer skipped by the tree/reports, and the global config still ignores it.
7. `igcared` → `[global]` and `[local]` sections with the cared row.
8. `local init --all` → error, no file written; `local init` → template.

## Risks / notes

- The working tree has many uncommitted changes (in-flight work on the local config feature, `src/fake/`, `src/nms/`, `src/shell/pipeline.rs`, …). Implement on top of it; do not revert unrelated files.
- `Config::reload_global()` only reloads the **global** file (`src/utils/config/mod.rs:528`); it is called after most local writes and is a no-op for local state. Local reads hit `ntconfig.toml` on every call, so no local reload is needed. Renaming this function is out of scope.
- `ntconfig.toml` is discovered from the process CWD (`get_local_config_path`, `globals.rs:850`), not the navigator path. In the interactive shell they stay in sync (`src/utils/navigator.rs:93`); out of scope to unify.
- The `db_aliases` and `fake_groups` dimensions keep their existing local-wins merge and do **not** get the `-g` flag.
- New `LocalConfig` keys are additive and `Option`-typed, so existing `ntconfig.toml` files keep parsing (unknown keys are already ignored, and `deny_unknown_fields` is not set).

## Out of scope

- Renaming `Config::reload_global()`.
- A `--local` counterpart to `-g/--global` (the default is already local inside a project).
- Applying the scope flag to `db_aliases` / `fake_groups` / `ral export`.
- Changing how `ntconfig.toml` is discovered.
