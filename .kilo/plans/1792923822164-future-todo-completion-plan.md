# Implementation Plan: Complete future_todo.txt tasks + i18n fixes

## Status: Tasks 1-5, 8-9 already verified as implemented in code. Remaining work below.

## Context

All 10 items in `future_todo.txt` have been inspected. Tasks 1-5, 8, and 9 are
already implemented in the source. The remaining concrete work is: register
`clip` as a pipe filter (task 6a), fix `clip clear` error handling (task 7),
add missing i18n keys to zh/th/lo language packs, clean up build warnings
(dead code + unused imports), update tests, then run `ntc sm todo done`
(task 10).

---

## 1. Register `clip` in pipe filter commands (task 6a)

**File:** `src/shell/hinter.rs:2934-2935`

Add `"clip"` to `PIPE_FILTER_COMMANDS`:

```rust
const PIPE_FILTER_COMMANDS: &[&str] =
    &["head", "tail", "betw", "gs", "sort", "rev", "encode", "qur", "clip"];
```

**Why:** `help.pipe.filters` in en.rs already lists `clip` as a pipe filter, and
`cmd_clip` already handles piped stdin (clipboard.rs:86-98). But `clip` is
missing from `PIPE_FILTER_COMMANDS`, so Tab completion never suggests it after
`|` and its flags/files don't get normal completion.

**Test update:** `pipe_stage_offers_only_the_pipe_filters` at hinter.rs:4641
must add `"clip".to_string()` to the expected `filters` vector.

---

## 2. Fix `clip clear` error handling (task 7)

**File:** `src/shell/commands/clipboard.rs:204-223`

`cmd_clear()` currently does:
```rust
save_history(&[])?;
let _ = crate::output::set_clipboard_text("");  // error silently ignored
```

**Bug:** If `set_clipboard_text("")` fails (e.g. clipboard daemon unavailable),
the system clipboard retains old content. Then `cmd_list()` at clipboard.rs:128-131
auto-captures the system clipboard on next invocation, "rescuing" the old
content into history — making `clip clear` appear to fail.

**Fix:** Handle the error explicitly:
```rust
save_history(&[])?;
if let Err(e) = crate::output::set_clipboard_text("") {
    print_warning(&t!("cmd.clipboard.clear_error", e));
}
```

This requires a new i18n key `cmd.clipboard.clear_error` (add to en.rs first,
then add to zh/th/lo).

---

## 3. Fix multi-line clipboard copy (task 6b)

**Investigation result:** The `cmd_clip` code (clipboard.rs:86-98) already
reads stdin via `crate::shell::stdin::read_to_string()` which uses
`take_input()` → `read_to_end()` into a `Vec<u8>`, then
`String::from_utf8_lossy`. Multi-line content is handled correctly — newlines
are preserved, then `normalize_newlines()` converts CRLF→LF.

**No code change needed.** The "critical bug" the user suspected likely stems
from `clip` not being registered in `PIPE_FILTER_COMMANDS` (item 1 above), so
users couldn't discover `cmd | clip` via Tab completion. After registering it,
multi-line pipe-to-clip works as-is.

---

## 4. Add missing i18n keys to zh/th/lo language packs

The `add_keys.py` script (lang/add_keys.py) defines translations for new keys
but the script has not been applied to the actual `.rs` lang pack files. The
following keys are missing from zh.rs, th/*.rs, lo/*.rs (or have wrong values):

### 4a. Missing from all 3 non-English packs

Run `add_keys.py` OR manually add these keys to each pack with the translations
from the script:

| Key | en.rs line | Status in zh/th/lo |
|-----|-----------|-------------------|
| `editor.nfe_confirm` | 3988 | **Missing** |
| `editor.nfe_rm_root` | 3992 | **Missing** |
| `editor.hint_nfe` | 3966 | **Present but outdated** — zh/th/lo versions omit `n new file · m new dir · r rename · d delete` |
| `msg.setTD.status` | 7510 | **Missing** |
| `msg.setTD.set` | 7511 | **Missing** |
| `msg.setTD.error` | 7512 | **Missing** |
| `cmd.clipboard.copied_piped` | 1383 | **Missing** |
| `help.report.clip` | 5037 | **Missing** |
| `help.report.clip_example` | — | **Missing** |
| `cmd.report.help_clip` | 2317 | **Missing** |
| `cmd.report.legacy_removed` | 2319 | **Missing** |
| `cmd.report.example_project` | 2289 | **Missing** |
| `cmd.report.example_clip` | 2291 | **Missing** |
| `cmd.report.clip_header` | 2293 | **Missing** |
| `cmd.report.clip_meta_date` | 2295 | **Missing** |
| `cmd.report.clip_meta_lines` | 2297 | **Missing** |
| `cmd.report.clip_meta_chars` | 2299 | **Missing** |
| `cmd.todo.help_format` | 3088 | **Missing** |
| `cmd.todo.unknown_format` | 3090 | **Missing** |
| `cmd.todo.parse_failed` | 3092 | **Missing** |
| `help.config.todo` | 7371 | **Missing** |
| `cmd.clipboard.clear_error` | **New key** (from item 2) | **Missing** |

### 4b. Wrong value in zh/th/lo

`cmd.clipboard.clear_prompt` is set to the "cancelled" message instead of the
actual prompt in all three non-English packs:

- **zh.rs:1436,1438** (duplicate!): `"清除已取消。"` (means "Clear cancelled.")
  → should be `"清除剪贴板历史（{} 条）？[y/N] "` (matching pattern of
  `cmd.todo.clear_prompt` at todo.rs:616)
- **th/part03.rs:14**: `"ยกเลิกการล้าง."` → should be
  `"ล้างประวัติคลิปบอร์ด ({} รายการ)? [y/N] "`
- **lo/part04.rs:85**: `"ຍົກເລີກການລຶບ."` → should be
  `"ລືບປະຫວັດສະທັງນ້ຳ {} ລາຍການ? [y/N] "`

### 4c. Missing `clip` in `help.pipe.filters`

- **zh.rs:5080**: `"cmd | head|tail|betw|gs|sort|rev|encode|qur ## ..."` → add `clip`
- **th/part07.rs:350**: same — add `clip`
- **lo/part12.rs:703**: same — add `clip`
- en.rs:5136 already has `clip` ✓

### 4d. Phantom key: `editor.nfe_confirm_cancelled`

Defined in en.rs:3990 but **never called** in code — no `t!("editor.nfe_confirm_cancelled")`
exists. The `n`/Esc handler in `nfe.rs:256-276` silently clears state without
printing. Decision: either (a) use this key in the handler, or (b) remove it
from all lang packs. **Recommendation:** Remove from all packs — no message
needed on cancel, consistent with current behavior.

### Implementation approach

The `add_keys.py` script can be run to add missing keys to zh/th/lo, but it
**does not fix wrong values** (`clear_prompt`) or **update existing keys**
(`help.pipe.filters`, `editor.hint_nfe`). Those require manual edits. Run the
script first, then manually fix the 3 wrong-value keys and 2 update-existing keys.

After all key changes, update the i18n test constants in `mod.rs` tests to
include the new keys (see item 7 below).

---

## 5. Remove dead code (build warnings cleanup)

### 5a. `get_clip_entry()` — clipboard.rs:36
Never called anywhere. Remove the function entirely. Also removes the only
use of `ClipEntry` outside its definition — but `ClipEntry` is still used by
`push_clip`, `load_history`, `save_history`, `cmd_list`, `cmd_select`, so
keep the struct and its derives.

### 5b. `NfeConfirm.name` field — types.rs:114
Set at nfe.rs:245 (`name: node.name.clone()`) but **never read**. The
`handle_nfe_confirm_key` function at nfe.rs:266-267 only uses `c.path` and
`c.is_dir`. Remove the `name` field from the struct and its initializer at nfe.rs:245.

### 5c. `generate_report_file`, `generate_report_dir`, `generate_report_dir_only` — report.rs:748,767,781
Defined but never called. Remove all three functions.

### 5d. Unused imports — clipboard.rs:7
`use std::io::{self, Read, Write};`
- `self` (as `io::`) — not used
- `Read` — not used (stdin::read_to_string is a custom function, not a trait method)
- `Write` — only used via local `use std::io::Write as _` at line 186
Fix: change to `use std::io::Write;` and remove the local import at line 186.
Or simply remove the top-level import entirely and keep the local one.

---

## 6. Update tests

### 6a. Hinter test — hinter.rs:4641
`pipe_stage_offers_only_the_pipe_filters`: add `"clip".to_string()` to the
expected `filters` vector.

### 6b. i18n tests — lang/mod.rs
The test constants in mod.rs (lines 158-1375) verify key sets exist in all 4
lang packs. Add new test constants and test functions for the newly added keys:

- `CLIPBOARD_KEYS` — keys from `cmd.clipboard.*` used in clipboard.rs
- `REPORT_CLIP_KEYS` — `cmd.report.clip*` keys used in report.rs:688-725
- `TODO_FORMAT_KEYS` — `msg.setTD.*` and `cmd.todo.help_format` / `cmd.todo.unknown_format` / `help.config.todo` keys
- `NFE_KEYS` — `editor.nfe_confirm`, `editor.nfe_rm_root`, `editor.hint_nfe` (exclude `nfe_confirm_cancelled` per 4d decision)

Each test function follows the existing pattern (e.g. `time_keys_exist_in_all_languages`).

---

## 7. Verify build

```sh
cargo check 2>&1 | grep -i warning
```

Expected: zero warnings after removing all dead code and unused imports.

---

## 8. Run the final todo command (task 10)

```sh
cargo run -- sm todo done
```

---

## Files to modify (implementation agent)

| File | Changes |
|------|---------|
| `src/shell/hinter.rs` | Add `"clip"` to `PIPE_FILTER_COMMANDS` (L2935); update test (L4641) |
| `src/shell/commands/clipboard.rs` | Fix `cmd_clear` error handling (L217); remove dead `get_clip_entry` (L36); fix imports (L7) |
| `src/editor/types.rs` | Remove `name` field from `NfeConfirm` (L114) |
| `src/editor/modes/nfe.rs` | Remove `name:` from `NfeConfirm` initializer (L245) |
| `src/shell/commands/report.rs` | Remove dead functions (L748, 767, 781) |
| `src/utils/config/lang/en.rs` | Add `cmd.clipboard.clear_error` key |
| `src/utils/config/lang/zh.rs` | Run add_keys.py; fix `clear_prompt`; add `clip` to `help.pipe.filters`; remove duplicate `clear_prompt` |
| `src/utils/config/lang/th/part03.rs` | Fix `clear_prompt` value |
| `src/utils/config/lang/th/part05.rs` | Update `editor.hint_nfe` (if needed by add_keys.py) |
| `src/utils/config/lang/th/part07.rs` | Add `clip` to `help.pipe.filters` |
| `src/utils/config/lang/lo/part04.rs` | Fix `clear_prompt` value |
| `src/utils/config/lang/lo/part10.rs` | Update `editor.hint_nfe` (if needed) |
| `src/utils/config/lang/lo/part12.rs` | Add `clip` to `help.pipe.filters` |
| `src/utils/config/lang/add_keys.py` | Remove `editor.nfe_confirm_cancelled` entry |
| `src/utils/config/lang/mod.rs` | Add test constants and test functions for new i18n keys |

## Risk assessment

- **Low risk:** Adding `"clip"` to `PIPE_FILTER_COMMANDS` — only affects Tab
  completion behavior, not runtime execution. `cmd_clip` already handles piped
  stdin.
- **Low risk:** i18n key additions — only affects display text for non-English
  users. English is unaffected.
- **Medium risk:** `cmd_clear` error handling change — changes behavior when
  `set_clipboard_text("")` fails. Previously: silently ignored. New: prints
  warning but still clears history. This is strictly better.
- **Low risk:** Dead code removal — no functional change, only warnings cleanup.
- **Validation:** Run `cargo check` for zero warnings, run existing test suite
  (`cargo test`), and verify `ntc sm todo done` executes.
