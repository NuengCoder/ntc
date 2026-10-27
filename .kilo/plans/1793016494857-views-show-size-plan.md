# Implementation Plan: `views --size` should show sizes

## Goal

Make the `views` command display file/directory sizes alongside names when `--size`
(or `-s`) is passed — with or without a minimum-size filter value. This mirrors the
existing `view --size` behavior where the flag always toggles size display, and a
following value adds filtering on top.

Currently `--size <min>` only **filters** entries; it never **shows** sizes. The todo
in `future_todo.md` is: `views -n --size should show size`.

## Root Cause

`cmd_views()` in `src/shell/commands/report.rs:384` has three problems:

1. **No `show_size` flag** — there is no boolean tracking "user wants sizes displayed".
   The only effect of `--size` is setting `min: Option<u64>` for filtering.
2. **Pre-validation is too strict** (lines 387–401) — errors on `views --size src`
   where a valid path follows `--size` (because it doesn't start with `-` and doesn't
   parse as a size). This prevents show-only mode with a path target.
3. **Main-loop `--size` arm is too strict** (lines 413–427) — calls `parse_size_value`
   on the next token and errors if it returns `None`. This breaks
   `views --size --sort name` (next token `--sort` fails the check) and makes bare
   `--size` a no-op.

The `view` command handles this correctly via `parse_view_flags()` (report.rs:258)
which uses `next_size_value()` (report.rs:100) — a helper that only consumes the
next token when it actually parses as a size.

## Changes

### 1. `src/shell/commands/report.rs` — `cmd_views()`

**1a. Remove the pre-validation loop** (lines 387–401):
```rust
// REMOVE this block:
let toks: Vec<String> = tokenize_spans(args)
    ...
for (i, tok) in toks.iter().enumerate() { ... }
```
`extract_target_spec()` (helpers.rs:104-110) already handles `--size` via
`is_optional_size_flag()` — it only consumes the next token when it parses as a
size; otherwise the token becomes the target path. The pre-validation is redundant
for the `<min>` case and harmful for the show-only case.

Trade-off: `views --size 10XB` (typo) will produce "path not found: 10XB" from
`resolve_target_spec` instead of "Invalid size: 10XB". This is consistent with
`view --size 10XB` and is acceptable.

**1b. Add `show_size` flag** alongside the existing local variables:
```rust
let mut min: Option<u64> = None;
let mut show_size = false;   // NEW
let mut sort = ...;
let mut newline = false;
```

**1c. Rewrite the `"-s" | "--size"` match arm** (lines 413–427):
```rust
"-s" | "--size" => {
    show_size = true;
    if let Some(v) = next_size_value(&parsed.rest, i) {
        min = Some(v);
        i += 1;
    }
}
```
This reuses the existing `next_size_value()` helper (line 100) which returns
`Some(u64)` only when the following token parses via `parse_size_value()`. If the
next token is a flag, a path, or absent, `show_size` is set but `min` stays `None`
— show-only mode.

**1d. Update the groups construction** (lines 506–519) to append sizes when
`show_size` is true:

```rust
let mut groups: Vec<Vec<(usize, String)>> = Vec::new();
for (is_dir, is_ignored) in [(true, false), (false, false), (true, true), (false, true)] {
    let names: Vec<(usize, String)> = tree
        .children
        .iter()
        .filter(|n| n.is_dir == is_dir && n.is_ignored == is_ignored)
        .map(|n| {
            let painted = paint(n, &n.name);
            if show_size {
                let s = crate::explorer::human_readable_size(n.size.unwrap_or(0));
                let text = format!("{} ({})", painted, s);
                let visible = n.name.chars().count() + 1 + 1 + s.chars().count() + 1;
                (visible, text)
            } else {
                (n.name.chars().count(), painted)
            }
        })
        .collect();
    if !names.is_empty() {
        groups.push(names);
    }
}
```

The `visible` width accounts for `name` + ` ` + `(` + `size_str` + `)` =
`name.chars().count() + size_str.chars().count() + 3`. This is only used by
`views_flow_push()` for line-wrapping in bare mode; the `-n` mode ignores it.

**1e. The `-n` and bare-mode print loops** (lines 522–552) need **no changes** —
they already iterate over `(visible, text)` tuples and print `text`, which now
includes the size when `show_size` is true.

### 2. `src/utils/config/lang/en.rs` — help/usage strings

**`help.views.size`** (line 5452) — update to indicate show behavior:
```
views -s, --size [min-size] ## Show sizes; with value (10KB, 1.5MB), hide smaller
```
(Mirrors `help.view.size` at line 5333: `view -s, --size [min-size] ## Show sizes; with value (10KB, 1.5MB), hide smaller`)

**`help.views.note`** (line 5455) — append a sentence about size display:
```
No [Directory]/[ignored] tags - colours show the kind. Depth is fixed at 1 and sizes are always computed with --care semantics. Use --size to display sizes.
```

**`cmd.views.usage`** (line 5459) — update bracket to show optional value:
```
Usage: views [path] [-n] [--size [min-size>] [--sort <name|ext|size>]
```
(Mirrors `cmd.report.view_usage` at line 2408 pattern: `[-s|--size [min-size]]`)

### 3. `src/utils/config/lang/zh.rs` and other packs — no changes needed

The `help.views.*` and `cmd.views.*` keys are only defined in `en.rs`. The `th` and
`lo` packs start from `super::en::build_map()` and override only specific keys; they
do not override the views keys. `zh.rs` also inherits from English. So updating
`en.rs` is sufficient — the new strings will cascade to all packs.

## Verification

1. `cargo check` — compiles without errors
2. `cargo test --lib -- report::tests` — existing tests still pass
3. `cargo test --lib -- lang` — i18n key-presence tests still pass (no keys added/removed)
4. Manual smoke: `views --size` shows sizes inline; `views -n --size` shows sizes per line;
   `views --size 1KB` filters and shows; `views --size --sort size` doesn't error

## Risks

- **Removing pre-validation**: `views --size 10XB` gives "path not found" instead of
  "Invalid size". Acceptable — consistent with `view` command.
- **`cmd.views.invalid_size` key**: becomes unused after code change. Can be removed from
  en.rs and `VIEWS_COUNT_KEYS` in `lang/mod.rs:446`, or left in place harmlessly.
  **Recommendation:** Remove to avoid dead code.
- **Display width**: bare `views --size` lines are longer (names + sizes). Line wrapping
  via `views_flow_push` handles this correctly via the `visible` width, but entries may
  wrap more aggressively. Acceptable for a flat listing.

## Files to modify

| File | Change |
|------|--------|
| `src/shell/commands/report.rs` | Remove pre-validation loop; add `show_size` flag; rewrite `--size` arm using `next_size_value`; update groups construction to append sizes |
| `src/utils/config/lang/en.rs` | Update `help.views.size`, `help.views.note`, `cmd.views.usage` strings |
