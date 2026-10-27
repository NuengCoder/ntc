# Implementation Plan: split `lang/` `partNN.rs` files into feature-named modules

> **Status: implemented.** Per-namespace granularity, all four packs in one pass.
> Each pack is now `lang/<pack>/` with 132 files (1 `mod.rs` + 46 feature files +
> `cmd/` 35 + `help/` 50); `<pack>.rs`, `partNN.rs` and `zh_pt1.rs` are gone.
> `lang/en.rs` lost 313 redundant duplicate inserts, `lang/zh.rs` 314; no key or
> translation text was dropped. `lang/mod.rs` is untouched.
>
> Verified: `cargo check --all-targets` clean; `cargo test --lib` 1095/1095 pass
> (34 are the `lang` key-presence tests); every pack's decoded key/value map is
> byte-identical to the pre-refactor state, and vs `git HEAD` the only differences
> are the pre-existing uncommitted translation edits (17 en / 5 th / 6 lo, all in
> the nssh pass-env, clipboard-clear and editor-hint clusters, none empty).
> The generator is idempotent (a re-run produces a byte-identical tree).
> End-to-end: the built binary renders `--help` and `help` correctly after
> `config lang` for english, thai, lao and chinese.
>
> The "Decisions needed" section below is kept for the record; all six were
> resolved as recommended, with granularity = per-namespace.

Todo item (`future_todo.md`, task 1):

> refactor lang feature here: `src/utils/config/lang/` for all of them and align
> them cleanly each part and make them have the `feature_name.rs` instead of
> `part0-n.rs`

## Goal

Re-cut every language pack in `src/utils/config/lang/` so that **the same feature
files exist in every pack with the same names and the same key partition**, and
`part01.rs … part20.rs` are replaced by `feature_name.rs` files.

Definition of done:

- No `partNN.rs` files remain.
- `en/`, `zh/`, `th/`, `lo/` all expose the identical module list.
- The exact same features files exist in all four packs (empty `fill()` allowed
  where a pack has no translations for that feature yet — the English fallback
  already covers this).
- Key set per pack is byte-for-byte preserved.
- `lang/mod.rs` loader, `t!` macro and key-presence tests are unchanged.

## Current state (measured)

| Path | Lines | `m.insert` calls | Notes |
|---|---|---|---|
| `lang/mod.rs` | 1540 | — | loader + `t!` macro + ~30 key-presence tests |
| `lang/en.rs` | 8010 | 4359 | monolithic; master pack; 4046 unique keys, **313 redundant duplicate inserts** |
| `lang/zh.rs` | 7636 | 4166 | monolithic; 1 orphan key `backup.restore_cancelled_2` |
| `lang/zh_pt1.rs` | 84 | 69 | **dead code** — not declared in `mod.rs`, no `include!`; all 69 keys already in `zh.rs` |
| `lang/th.rs` + `lang/th/part01..11.rs` | 30 + 6500 | 4002 | `th.rs` only declares the parts and calls `fill()` |
| `lang/lo.rs` + `lang/lo/part01..20.rs` | 49 + 8200 | 3906 | same shape as `th` |

Key facts that shape the work:

1. **The part files are arbitrary line-range chunks, not features.** e.g.
   `th/part01.rs` = alias + backup + autoup + cmd; `lo/part02.rs` = clap + backup
   + backup_diff. `th` and `lo` boundaries do not correspond to each other or to
   any feature — this is exactly what the task asks to fix.
2. **Keys can span lines.** Many `lo` entries are
   `m.insert(\n    "key".into(),\n    "value".into(),\n);`. Line-oriented regex on
   `m.insert("` silently misses them (grep found ~1400 of 3906 in `lo`). Tooling
   must parse `m.insert(\s*"key"`.
3. **No key collisions today**: 0 duplicate keys inside `th`, 0 inside `lo`,
   0 orphans (every `th`/`lo` key exists in `en`). So `en` is a strict superset
   and re-partitioning cannot change behaviour as long as the partition has no
   key assigned to two files.
4. **Coverage is very uneven** (keys per feature, en / zh / th / lo):

   | feature | en | zh | th | lo |
   |---|---|---|---|---|
   | cli (+clap) | 312 | 312 | 304 | 304 |
   | msg group | 158 | 141 | 158 | 150 |
   | misc group | 237 | 210 | 233 | 233 |
   | mode group | 44 | 44 | 40 | 40 |
   | theme | 60 | 60 | 60 | 60 |
   | math | 48 | 48 | 47 | 47 |
   | nav group | 59 | 59 | 56 | 56 |
   | report | 7 | 7 | 7 | 7 |
   | net group | 70 | 70 | 70 | 70 |
   | encdec group | 62 | 62 | 57 | 57 |
   | backup group | 159 | 160 | 159 | 159 |
   | alias group | 130 | 130 | 130 | 130 |
   | git group | 96 | 96 | 96 | 96 |
   | igcare group | 32 | 32 | 31 | 31 |
   | ntpkg | 189 | 189 | 152 | 152 |
   | ndb | 151 | 151 | 151 | 151 |
   | deadlock | 42 | **0** | 42 | **0** |
   | editor | 527 | 506 | 364 | 362 |
   | cmd | 1297 | 1231 | 1175 | 1133 |
   | help | 679 | 658 | 670 | 668 |
   | **total** | 4359 | 4166 | 4002 | 3906 |

   `zh` has zero `deadlock` entries and ~195 keys fewer than `en`; `lo` is the
   least complete. Both already rely on the `super::en::build_map()` fallback, so
   missing feature files are expected and must not be an error.

## Target layout

```
src/utils/config/lang/
  mod.rs                    # unchanged: loader, t! macro, LANG_PACK, tests
  en/
    mod.rs                  # build_map(): HashMap::new() + <feature>::fill(&mut m)
    cli.rs  clap.rs  msg.rs  ...
    cmd/<command>.rs
    help/<topic>.rs
  zh/
    mod.rs                  # build_map(): super::en::build_map() + <feature>::fill(&mut m)
    ... same file list ...
  th/ ... same file list ...
  lo/ ... same file list ...
```

Every file has the same shape as today's part files:

```rust
use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ndb.banner".into(), "ndb {} - NTC Database shell".into());
    // ...
}
```

`<lang>/mod.rs`:

```rust
use std::collections::HashMap;

pub(crate) mod cli;
pub(crate) mod clap;
// ...
pub(crate) mod cmd { pub(crate) mod serve; pub(crate) mod nssh; /* ... */ }

pub fn build_map() -> HashMap<String, String> {
    let mut m = super::en::build_map();   // (en: HashMap::new())
    cli::fill(&mut m);
    clap::fill(&mut m);
    // ...
    cmd::serve::fill(&mut m);
    m
}
```

Order: fixed and identical across packs, so `en/`, `zh/`, `th/`, `lo/` lists read
the same way.

## The feature set

Two granularity levels are on the table (see **Decisions needed**). Both use the
same partition rule, so keys never move between two files.

### Level 1 — top-level namespaces (identical rule for all packs)

49 `en` top-level namespaces, mapped to files:

| File | Owns top-level namespaces |
|---|---|
| `cli.rs` | `cli` |
| `clap.rs` | `clap` |
| `ndb.rs` | `ndb` |
| `deadlock.rs` | `deadlock` |
| `alias.rs` | `alias`, `ral_export` |
| `backup.rs` | `backup`, `backup_diff`, `autoup` |
| `git.rs` | `git`, `ran`, `ranfile`, `scr` |
| `editor.rs` | `editor` |
| `ntpkg.rs` | `ntpkg` |
| `theme.rs` | `theme` |
| `math.rs` | `math` |
| `nav.rs` | `nav`, `navigator`, `teleport` |
| `report.rs` | `report` |
| `net.rs` | `dl`, `dns`, `ping`, `image2doc` |
| `encdec.rs` | `encode`, `decode`, `gosc`, `fmt` |
| `igcare.rs` | `igcare_export`, `config`, `nfe` |
| `msg.rs` | `msg`, `status`, `error`, `output`, `helpers`, `entry` |
| `misc.rs` | `misc`, `qsc` |
| `mode.rs` | `mode`, `interactive`, `tutorial`, `up`, `watcher`, `cvcode` |
| `cmd/` | every `cmd.<name>.*` key, one file per `<name>` |
| `help/` | every `help.<topic>.*` key, one file per `<topic>` |

`cmd/` second levels (37): `arc`, `care`, `clipboard`, `completed`, `config`,
`fake`, `fmt`, `gs`, `http`, `igcare`, `igcar`, `igcared`, `ignore`, `image`,
`image_disabled`, `ipcfg`, `mode`, `nav`, `nms`, `nssh`, `nsheet`, `pathman`,
`qr`, `qsc`, `report`, `scan`, `serve`, `smtp`, `spec`, `target`, `terminated`,
`time`, `tl`, `tmprt`, `todo`, `upgrade`, `views`.

`help/` second levels (50): `alias`, `arc`, `autoup`, `backup`, `clipboard`,
`config`, `context`, `deadlock`, `editor`, `encdec`, `examples_header`,
`explorer`, `fake`, `fileops`, `fmt`, `git`, `igcare`, `image`, `math`, `mode`,
`nav`, `ndb`, `network`, `nms`, `nsheet`, `nssh`, `ntpkg`, `other`, `pathman`,
`pipe`, `python`, `qr`, `qsc`, `ran`, `report`, `scan`, `scr`, `search`, `size`,
`syscmd`, `teleport`, `theme`, `time`, `tl`, `tmprt`, `todo`, `unknown_topic`,
`view`, `viewd`, `viewf`, `views`.

### Level 2 — smaller, feature-grouped files

If ~137 files per pack is too many, the same partition collapses to ~20 files per
pack by merging the small namespaces:

| File | Owns |
|---|---|
| `cli.rs` | `cli`, `clap` |
| `msg.rs` | `msg`, `status`, `error`, `output`, `helpers`, `entry` |
| `misc.rs` | `misc`, `qsc`, `nfe` |
| `mode.rs` | `mode`, `interactive`, `tutorial`, `up`, `watcher`, `cvcode` |
| `theme.rs` | `theme` |
| `math.rs` | `math` |
| `nav.rs` | `nav`, `navigator`, `teleport` |
| `report.rs` | `report` |
| `net.rs` | `dl`, `dns`, `ping`, `image2doc` |
| `encdec.rs` | `encode`, `decode`, `gosc`, `fmt` |
| `backup.rs` | `backup`, `backup_diff`, `autoup` |
| `alias.rs` | `alias`, `ral_export` |
| `git.rs` | `git`, `ran`, `ranfile`, `scr` |
| `igcare.rs` | `igcare_export`, `config` |
| `ntpkg.rs` | `ntpkg` |
| `ndb.rs` | `ndb` |
| `deadlock.rs` | `deadlock` |
| `editor.rs` | `editor` |
| `cmd.rs` | all `cmd.*` (≈2600 lines / pack — large) |
| `help.rs` | all `help.*` (≈1400 lines / pack) |

The two levels can be mixed: grouped top-level files **plus** `cmd/` and `help/`
subtrees split per command (≈105 files/pack). This is likely the best balance —
it keeps the two mega-namespaces navigable while avoiding 30 trivial files.

## Migration steps

1. **Freeze the current key sets** — dump each pack's keys before touching
   anything:
   `python` script or a temporary `#[test]` that writes sorted `key<TAB>value`
   lists for `en`/`zh`/`th`/`lo` to `/tmp/lang-before/*.txt`.
2. **Delete `lang/zh_pt1.rs`** — dead code, all keys already in `zh.rs`.
3. **Build the key→file map** from the namespace lists above (pure function of
   the key's `cmd`/`help` second level, else the top-level namespace).
4. **Parse and bucket** every `m.insert(...)` from `en.rs`, `zh.rs`,
   `th/part*.rs`, `lo/part*.rs` with a multi-line-aware parser
   (`m\.insert\(\s*"([^"]+)"`), assigning each insertion to a feature file.
   Keep values byte-exact (Thai / Lao / Chinese text, `\r\n`, BOM).
5. **De-duplicate** while porting: `en.rs` has 313 keys inserted twice with
   identical values; keep one. Report any duplicate whose two values differ
   (none found so far for `en`) rather than silently dropping one.
6. **Emit** `<lang>/<feature>.rs` and `<lang>/mod.rs` for all four packs, and
   `cmd/` / `help/` as `mod.rs` + per-feature files (or a single `cmd.rs`).
7. **Delete** `en.rs`, `zh.rs`, `zh_pt1.rs`, `th.rs`, `th/part*.rs`, `lo.rs`,
   `lo/part*.rs`.
8. **Leave `lang/mod.rs` alone** — its `pub mod en; pub mod zh; pub mod th;
   pub mod lo;`, `load()`, `t!` and the ~30 `*_keys_exist_in_all_languages`
   tests keep working unchanged, since they only call `build_map()`.
9. Normalize line endings and encoding deliberately (see Risks) — do not let the
   tooling rewrite CRLF/LF mix or drop the `en.rs`/`zh.rs` BOM by accident.

## Verification

1. `cargo check` — compiles.
2. `cargo test --lib -- lang` — the key-presence tests in `lang/mod.rs`
   (`time_keys_exist_in_all_languages`, `qur_…`, `scr_…`, `git_…`, `dir_view_…`,
   `views_and_count_…`, `intro_view_…`, `file_view_size_…`, `ndb_…`,
   `deadlock_…`, `smtp_deadlock_…`, `share_…`, `lock_…`, `serve_chess_…`,
   `serve_config_…`, `nsheet_…`, `nssh_…`, `nms_…`, `ral_hintn_…`,
   `local_scope_…`, `scan_files_…`, `cli_new_…`, `fake_…`,
   `opencode_init_…`, `where_binary_…`, `rmd_wildcard_…`, `clipboard_…`,
   `report_clip_…`, `todo_format_…`) all pass — they assert against
   `build_map()` for all four packs.
3. **Key-set equality**: diff the "before" dumps against "after" dumps per pack.
   Must be identical except for the deliberately removed `en` duplicates and the
   deleted `zh_pt1.rs`.
4. **Value equality**: additionally compare `key<TAB>value` (not just keys) per
   pack, to prove no translation text was lost or swapped between features.
5. `cargo test` — full suite, since `t!()` call sites are untouched this should
   be a no-op, but it catches accidental key renames.
6. Spot-check a launch: `cargo run` / the built binary with each
   `config lang set <english|chinese|thai|lao>` and confirm shell help renders.

## Risks

- **Encoding / line endings.** `zh.rs`, `th.rs`, `th/part*.rs` use CRLF; `lo.rs`,
  `lo/part*.rs` and `en.rs` use LF. `zh*.rs` carry a UTF-8 BOM. A naive
  read/write round-trip will change every line. Decide: preserve per-file as-is,
  or normalize everything to LF + no BOM (a large, noisy diff). Recommendation:
  preserve byte-exact; only the file set changes.
- **Duplicate `en` keys.** 313 exist, identical values, adjacent lines (e.g.
  `cmd.http.authorization_label` at en.rs:1679 and :1681). If ported verbatim
  they land in the same feature file and `HashMap` behaviour is unchanged, so
  they are safe to keep — but they are dead weight and should be collapsed.
- **`zh` orphan key** `backup.restore_cancelled_2` has no English counterpart.
  Keep it (harmless) and note it; do not "fix" it as part of a mechanical split.
- **Order-dependence.** `build_map()` is last-write-wins. Today no key is
  duplicated across files in `th`/`lo`, and the partition assigns each key to
  exactly one file, so ordering cannot change output. The uniqueness check in
  step 5 must run before emitting, to keep that guarantee.
- **Diff size.** Even at the coarser granularity this replaces ~34 files with
  ~80 and rewrites ~35k lines of mostly non-ASCII text. Review should rely on the
  key/value equality checks rather than a line-by-line read.
- **Unrelated files.** `future_todo.md` and `lang/mod.rs` stay untouched, so this
  refactor does not collide with the other todo items (shell integrations).

## Files

Deleted: `lang/en.rs`, `lang/zh.rs`, `lang/zh_pt1.rs`, `lang/th.rs`,
`lang/th/part01..11.rs`, `lang/lo.rs`, `lang/lo/part01..20.rs`.

Added: `lang/{en,zh,th,lo}/mod.rs` + the feature files listed above, identical
file names per pack.

Unchanged: `lang/mod.rs`.

## Decisions needed

1. **Granularity** — per-namespace (`cmd/`, `help/` subtrees, ~137 files/pack) vs
   grouped (~20 files/pack) vs the hybrid (grouped top-level + `cmd/` & `help/`
   per command, ~105 files/pack).
2. **Split `en.rs` and `zh.rs` too?** Required for "all of them" to truly align,
   but it is the largest part of the diff (8010 + 7636 lines). Alternative:
   convert only `th`/`lo` now and file the `en`/`zh` split separately.
3. **Delete `zh_pt1.rs`** — recommended, it is unreferenced and 100% redundant.
4. **Collapse the 313 duplicate `en` keys** — recommended.
5. **Keep `en/` importing nothing** (`HashMap::new()`) — `en` is the master; all
   other packs start from `super::en::build_map()`. Confirm no behaviour change
   wanted here.
