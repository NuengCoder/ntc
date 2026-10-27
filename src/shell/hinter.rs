use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use rustyline::hint::Hinter;
use rustyline::{Cmd, ConditionalEventHandler, Context, Event, EventContext, RepeatCount};

use crate::config::Config;
use crate::ntpkg;
use crate::ranfile::parser as ran_parser;
use crate::teleport::TeleportManager;

/// Generation counter bumped whenever run aliases are modified.
/// The hinter's `hint()` method checks this and refreshes its cache when stale.
pub(crate) static ALIAS_GENERATION: AtomicU64 = AtomicU64::new(0);

/// Built-in commands for the ntc shell (lowercase).
fn builtin_commands() -> Vec<&'static str> {
    vec![
        "cd",
        "godrive",
        "god",
        "back",
        "b",
        "view",
        "viewf",
        "viewd",
        "views",
        "ls",
        "report",
        "txtc",
        "txtf",
        "head",
        "tail",
        "betw",
        "sort",
        "rev",
        "count",
        "trim",
        "config",
        "cfg",
        "clear",
        "version",
        "where",
        "gos",
        "gosc",
        "ral",
        "ran",
        "scr",
        "script",
        "run",
        "r",
        "showcg",
        "help",
        "exit",
        "quit",
        "ignore",
        "care",
        "igcared",
        "size",
        "tp",
        "opencg",
        "resetcg",
        "restorecg",
        "local",
        "ne",
        "ntceditor",
        "igcare",
        "esc",
        "backup",
        "gs",
        "qur",
        "wi",
        "fs",
        "ds",
        "fgo",
        "fsc",
        "locate",
        "math",
        "theme",
        "tpb",
        "tutorial",
        "init",
        "deinit",
        "explorer",
        "nfe",
        "rnmf",
        "rnmd",
        "mv",
        "move",
        "cp",
        "image",
        "img",
        "py_activate",
        "py_deactivate",
        "showenv",
        "ntpkg",
        "ntpx",
        "tl",
        "http",
        "dl",
        "up",
        "ping",
        "dns",
        "smtp",
        "serve",
        "spec",
        "clip",
        "qsc",
        "todo",
        "arc",
        "scan",
        "ipcfg",
        "encode",
        "decode",
        "cvcode",
        "fmt",
        "qr",
        "usrpath",
        "syspath",
        "autoup",
        "mode",
        "pathcp",
        "say",
        "print",
        "tmprt",
        "time",
        "git",
        "ndb",
        "nms",
        "nsheet",
        "nssh",
        "fake",
        "deadlock",
        "mkf",
        "mkd",
        "rmf",
        "rmd",
        "sys",
        "top",
        "td",
        "guide",
        "reset-config",
        "restore-config",
        "editcfg",
        "editcg",
    ]
}

/// True when `name` is one of the shell's built-in commands.
pub(crate) fn is_builtin_command(name: &str) -> bool {
    builtin_commands()
        .iter()
        .any(|command| command.eq_ignore_ascii_case(name))
}

/// Returns the list of nssh profile names for commands that accept them
/// (`nssh ssh/info/rm/edit <name>`, bare `nssh <name>`, `nssh cp <name>:…`).
pub(crate) fn nssh_profile_names_for_command(parts: &[&str]) -> Vec<String> {
    if parts.is_empty() {
        return vec![];
    }
    if parts[0].to_lowercase() != "nssh" {
        return vec![];
    }
    let Ok(profiles) = crate::nssh::store::list_profiles() else {
        return vec![];
    };
    let mut names: Vec<String> = profiles.into_keys().collect();
    names.sort();
    if parts.len() >= 2 && matches!(parts[1].to_lowercase().as_str(), "cp") {
        // Remote side of `cp` takes `<name>:` — trailing colon included.
        return names.into_iter().map(|n| format!("{}:", n)).collect();
    }
    if parts.len() == 1
        || matches!(
            parts[1].to_lowercase().as_str(),
            "ssh" | "info" | "rm" | "edit"
        )
    {
        return names;
    }
    vec![]
}

/// Returns the list of teleport save-point names for commands that accept them
/// as arguments (cd, tp jump/to/rm/rnm/info).
pub(crate) fn teleport_names_for_command(parts: &[&str]) -> Vec<String> {
    if parts.is_empty() {
        return vec![];
    }

    let root = parts[0].to_lowercase();
    match root.as_str() {
        // cd can take a teleport name as argument
        "cd" => TeleportManager::get_all().into_keys().collect(),
        // tp subcommands that accept a teleport name as argument
        "tp" if parts.len() >= 2 => match parts[1].to_lowercase().as_str() {
            "jump" | "to" | "rm" | "info" => TeleportManager::get_all().into_keys().collect(),
            "rnm" | "rename" => {
                // rnm <old> to <new> — suggest alias names for the first argument
                // parent_parts = ["tp", "rnm"], last_part = the name being typed
                TeleportManager::get_all().into_keys().collect()
            }
            _ => vec![],
        },
        _ => vec![],
    }
}

/// Returns the list of run alias names for commands that accept them
/// (ral rm, ral rnm, ral info, ral edit).
pub(crate) fn alias_names_for_command(parts: &[&str]) -> Vec<String> {
    if parts.is_empty() {
        return vec![];
    }

    let root = parts[0].to_lowercase();
    if root != "ral" || parts.len() < 2 {
        return vec![];
    }

    match parts[1].to_lowercase().as_str() {
        "rm" | "remove" | "info" | "edit" => {
            let aliases = Config::get_run_aliases_merged();
            aliases.into_keys().collect()
        }
        "rnm" | "rename" => {
            // rnm <old> to <new> — suggest alias names for the first argument
            let aliases = Config::get_run_aliases_merged();
            aliases.into_keys().collect()
        }
        _ => vec![],
    }
}

/// Returns the list of named hint-list names for `ral hintn` subcommands
/// that take one (`rm`, `info`, and the first slot of `rnm`).
pub(crate) fn hint_list_names_for_command(parts: &[&str]) -> Vec<String> {
    if parts.len() < 3 {
        return vec![];
    }
    if parts[0].to_lowercase() != "ral" || parts[1].to_lowercase() != "hintn" {
        return vec![];
    }
    match parts[2].to_lowercase().as_str() {
        "rm" | "remove" | "info" => hint_list_names(),
        "rnm" | "rename" => {
            // rnm <old> to <new> — names only for the first argument
            if parts.len() == 3 {
                hint_list_names()
            } else {
                vec![]
            }
        }
        _ => vec![],
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TpEntryMode {
    All,
    Files,
    Dirs,
}

/// Which entry kinds a `@tp/sub` completion should list for `cmd`.
fn tp_entry_mode_for(cmd: &str, parent_parts: &[&str]) -> TpEntryMode {
    match cmd {
        "viewf" | "txtf" => TpEntryMode::Files,
        "viewd" => TpEntryMode::Dirs,
        "view" => {
            let file = parent_parts
                .iter()
                .any(|t| matches!(t.to_lowercase().as_str(), "-f" | "--file"));
            let dir = parent_parts
                .iter()
                .any(|t| matches!(t.to_lowercase().as_str(), "-dir" | "--directory"));
            match (file, dir) {
                (true, false) => TpEntryMode::Files,
                (false, true) => TpEntryMode::Dirs,
                _ => TpEntryMode::All,
            }
        }
        _ => TpEntryMode::All,
    }
}

/// True when `txtf`'s optional file slot is still open.
fn txtf_file_slot_open(parent_parts: &[&str]) -> bool {
    if let Some(last) = parent_parts.last().map(|s| s.to_lowercase()) {
        if last == "--head" || last == "--tail" || last == "--betw" {
            return false;
        }
    }
    let mut i = 1;
    while i < parent_parts.len() {
        let lower = parent_parts[i].to_lowercase();
        if lower == "--head" || lower == "--tail" || lower == "--betw" {
            i += 2; // skip the flag and its value
            continue;
        }
        if !parent_parts[i].starts_with('-') {
            return false;
        }
        i += 1;
    }
    true
}

/// Content completion for `@tp/rest`:
/// longest savepoint-name prefix (mirrors `resolve_subpath`), remaining
/// path listed from the savepoint root (files plain, dirs with `/`).
/// Returns `None` when this frag is not a subpath shape, the slot is not
/// open, or no savepoint prefix matches (caller falls back to
/// savepoint-name completion).
fn tp_subpath_candidates(
    parent_parts: &[&str],
    last_frag: &str,
    tp_part: &str,
) -> Option<Vec<String>> {
    if !tp_part.contains('/') || parent_parts.is_empty() {
        return None;
    }
    let cmd = parent_parts[0].to_lowercase();
    let mode = match cmd.as_str() {
        // mv/cp src slot only: zero non-flag operands so far
        // (`-f`/`--force` skipped like `parse_mv_args`).
        "mv" | "move" | "cp" => {
            let operands_given = parent_parts[1..]
                .iter()
                .filter(|t| !t.starts_with('-'))
                .count();
            if operands_given > 0 {
                return None;
            }
            TpEntryMode::All
        }
        // Target path is already filled (flags/values don't count).
        "view" | "viewf" | "viewd" | "views" => {
            if in_ext_slot(parent_parts) || view_path_taken(&parent_parts[1..]) {
                return None;
            }
            tp_entry_mode_for(&cmd, parent_parts)
        }
        "txtf" => {
            if !txtf_file_slot_open(parent_parts) {
                return None;
            }
            TpEntryMode::Files
        }
        // `mkf`/`rmf` take file lists and `mkd`/`rmd` directory lists; every
        // comma-separated operand slot resolves `@tp/sub` (files mode still
        // offers directories with a trailing `/` so sub-paths stay reachable).
        "mkf" | "rmf" => TpEntryMode::Files,
        "mkd" | "rmd" => TpEntryMode::Dirs,
        _ => return None,
    };
    let names = TeleportManager::get_all();
    // Split positions, longest head first (`@a/b/c` prefers savepoint
    // `a/b` over `a` — same rule as `resolve_subpath`).
    let mut positions: Vec<usize> = tp_part.match_indices('/').map(|(i, _)| i).collect();
    positions.sort_unstable_by(|a, b| b.cmp(a));
    for pos in positions {
        let head = &tp_part[..pos];
        if head.is_empty() {
            continue;
        }
        if let Some(root) = names.get(&head.to_lowercase()) {
            let rest = &tp_part[pos + 1..];
            let (sub, _partial) = match rest.rfind('/') {
                Some(idx) => (&rest[..idx], &rest[idx + 1..]),
                None => ("", rest),
            };
            let dir = if sub.is_empty() {
                root.clone()
            } else {
                root.join(sub)
            };
            // Displayed prefix preserves the typed head verbatim so the
            // ghost still starts with the full fragment.
            let display_base = if sub.is_empty() {
                format!("@{}/", head)
            } else {
                format!("@{}/{}/", head, sub)
            };
            let mut out = Vec::new();
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let name = match entry.file_name().to_str() {
                        Some(n) => n.to_string(),
                        None => continue,
                    };
                    let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                    // Files mode still offers directories with a trailing
                    // slash so users can traverse to a target directory.
                    if mode == TpEntryMode::Dirs && !is_dir {
                        continue;
                    }
                    if is_dir {
                        out.push(format!("{}{}/", display_base, name));
                    } else {
                        out.push(format!("{}{}", display_base, name));
                    }
                }
            }
            return Some(filter_prefix(out, last_frag));
        }
    }
    None
}

/// All plain file names in CWD, optionally filtered.
fn cwd_files(matching: impl Fn(&str) -> bool) -> Vec<String> {
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => return vec![],
    };
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&cwd) {
        for entry in entries.flatten() {
            let is_file = entry.file_type().map(|t| t.is_file()).unwrap_or(false);
            if !is_file {
                continue;
            }
            if let Some(name) = entry.file_name().to_str() {
                if matching(name) {
                    out.push(name.to_string());
                }
            }
        }
    }
    out.sort();
    out
}

/// All subdirectory names in CWD, sorted.
fn cwd_dirs() -> Vec<String> {
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => return vec![],
    };
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&cwd) {
        for entry in entries.flatten() {
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            if !is_dir {
                continue;
            }
            if let Some(name) = entry.file_name().to_str() {
                out.push(name.to_string());
            }
        }
    }
    out.sort();
    out
}

/// Stems of CWD files matching `exts` (empty = all files), sorted/deduped.
/// Shared stem rule: see `crate::shell::alias::file_stem_matching`.
fn cwd_file_stems(exts: &[String]) -> Vec<String> {
    let names = cwd_files(|n| {
        exts.is_empty() || {
            let lower = n.to_lowercase();
            exts.iter()
                .any(|e| lower == *e || lower.ends_with(&format!(".{}", e)))
        }
    });
    let mut out: Vec<String> = names
        .iter()
        .filter_map(|n| crate::shell::alias::file_stem_matching(n, exts))
        .collect();
    out.sort();
    out.dedup();
    out
}

/// Candidates for one `#param --hint-*` group (union over its kinds).
fn hint_kind_candidates(kinds: &[crate::shell::alias::ParamHintKind]) -> Vec<String> {
    use crate::shell::alias::ParamHintKind as K;
    let mut out: Vec<String> = Vec::new();
    for kind in kinds {
        match kind {
            K::Tp => out.extend(TeleportManager::get_all().into_keys()),
            K::Ext(exts) if !exts.is_empty() => {
                out.extend(cwd_files(|n| {
                    let lower = n.to_lowercase();
                    exts.iter()
                        .any(|e| lower == *e || lower.ends_with(&format!(".{}", e)))
                }));
            }
            K::Ext(_) | K::File => out.extend(cwd_files(|_| true)),
            K::ExtName(exts) => out.extend(cwd_file_stems(exts)),
            K::FileName => out.extend(cwd_file_stems(&[])),
            K::Spf => out.extend(cwd_files(|n| {
                crate::filetype::is_supported_format(std::path::Path::new(n))
            })),
            K::Values(vals) => out.extend(vals.iter().cloned()),
            K::Named(name) => {
                out.extend(crate::shell::alias::named_list_values(name));
            }
            K::Dir => {
                out.push(".".to_string());
                out.extend(cwd_dirs());
            }
            K::Fd => {
                out.push(".".to_string());
                out.extend(cwd_dirs());
                out.extend(cwd_files(|_| true));
            }
            K::All => {
                out.push(".".to_string());
                out.extend(cwd_dirs());
                out.extend(cwd_files(|_| true));
                out.extend(TeleportManager::get_all().into_keys());
                out.extend(Config::get_run_aliases_merged().into_keys());
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Candidates for one `#param --select-*` group (union over its kinds).
fn select_kind_candidates(kinds: &[crate::shell::alias::ParamSelectKind]) -> Vec<String> {
    use crate::shell::alias::ParamSelectKind as K;
    let mut out: Vec<String> = Vec::new();
    for kind in kinds {
        match kind {
            K::Tp => out.extend(TeleportManager::get_all().into_keys()),
            K::Ext(exts) if !exts.is_empty() => {
                out.extend(cwd_files(|n| {
                    let lower = n.to_lowercase();
                    exts.iter()
                        .any(|e| lower == *e || lower.ends_with(&format!(".{}", e)))
                }));
            }
            K::Ext(_) | K::File => out.extend(cwd_files(|_| true)),
            K::ExtName(exts) => out.extend(cwd_file_stems(exts)),
            K::FileName => out.extend(cwd_file_stems(&[])),
            K::Values(vals) => out.extend(vals.iter().cloned()),
            K::Named(name) => {
                out.extend(crate::shell::alias::named_list_values(name));
            }
            K::Dir => {
                out.push(".".to_string());
                out.extend(cwd_dirs());
            }
            K::Fd => {
                out.push(".".to_string());
                out.extend(cwd_dirs());
                out.extend(cwd_files(|_| true));
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Param-hint completion for alias invocations.
///
/// Both `alias arg…` (positional, Nth arg) and `alias(a, b…)` (Nth comma
/// slot) resolve the Nth template placeholder (first-appearance order)
/// and complete it from its `#param --hint-*` / `--select-*` annotation.
/// Bracket groups `[...]` count as a single slot: inside an unclosed
/// bracket the *elements* complete against the current element text
/// (candidates carry only the frag's own bracket head so the ghost
/// stays exact); a closed `]` ends the slot and hinting stops.
/// Returns `None` when the root is not an alias or the slot has no hint
/// (caller keeps default behavior); `Some` replaces it.
pub(crate) fn alias_arg_candidates_for(
    parent_parts: &[&str],
    last_frag: &str,
    trailing_space: bool,
) -> Option<Vec<String>> {
    if parent_parts.is_empty() {
        return None;
    }
    let (alias, index, head) = alias_slot(parent_parts, last_frag, trailing_space)?;
    let aliases = Config::get_run_aliases_merged();
    let template = aliases.get(&alias)?;
    let order = crate::shell::alias::placeholder_idents(template);
    let (pname, _) = order.get(index)?;
    let hints = Config::get_run_alias_hints_merged();
    let spec = hints.get(&alias)?;
    let groups = crate::shell::alias::parse_hint_spec(spec);
    let group = groups.iter().find(|g| g.param == *pname)?;
    let mut out = hint_kind_candidates(&group.kinds);
    out.extend(select_kind_candidates(&group.selects));
    out.sort();
    out.dedup();
    Some(out.into_iter().map(|c| format!("{}{}", head, c)).collect())
}

/// Resolve `(alias-name, slot-index, element-head)` for completion.
/// `head` comes from the in-progress fragment only — text through its
/// last `[`/`,` (empty when the fragment has no delimiter) — so picks
/// always start with the full frag and the ghost stays exact. Parent
/// tokens affect slot counting alone and never leak into the head.
/// Fresh slots (trailing space) use bare candidates.
fn alias_slot(
    parent_parts: &[&str],
    last_frag: &str,
    trailing_space: bool,
) -> Option<(String, usize, String)> {
    // Arg token stream after the alias name (`name(` prefix stripped
    // for the call form), plus the in-progress fragment when present.
    let mut stream: Vec<(String, bool)> = Vec::new();
    let alias = match parent_parts[0].find('(') {
        Some(p) => {
            let name = parent_parts[0][..p].trim_end().to_lowercase();
            stream.push((parent_parts[0][p + 1..].to_string(), false));
            for t in &parent_parts[1..] {
                stream.push((t.to_string(), false));
            }
            name
        }
        None => {
            for t in &parent_parts[1..] {
                stream.push((t.to_string(), false));
            }
            parent_parts[0].to_lowercase()
        }
    };
    if !trailing_space {
        stream.push((last_frag.to_string(), true));
    }
    // Walk complete tokens, grouping `[...]` spans into single slots.
    // The in-progress fragment (if any) resolves to its own head: text
    // through its last `[`/`,` plus following spaces, or bare when it
    // carries no delimiter — so picks always start with the full frag
    // and the ghost stays exact.
    let mut slots_done = 0usize;
    let mut depth = 0i32;
    for (tok, partial) in stream.iter() {
        let opens = tok.chars().filter(|&c| c == '[').count() as i32;
        let closes = tok.chars().filter(|&c| c == ']').count() as i32;
        if *partial {
            let mut cut = tok
                .rfind(|c| c == '[' || c == ',')
                .map(|p| p + 1)
                .unwrap_or(0);
            while tok[cut..].starts_with(|c| c == ' ' || c == '\t') {
                cut += 1;
            }
            return Some((alias, slots_done, tok[..cut].to_string()));
        }
        if depth > 0 {
            depth += opens - closes;
            if depth <= 0 {
                depth = 0;
                slots_done += 1;
            }
            continue;
        }
        if opens > closes {
            depth = opens - closes;
            continue;
        }
        // No open brackets: a closed `[...]` group or plain token.
        slots_done += 1;
    }
    // Trailing space (or empty stream): fresh slot, bare candidates.
    Some((alias, slots_done, String::new()))
}

const HINT_FLAGS: &[&str] = &[
    "--hint-tp",
    "--hint-ext",
    "--hint-ext-name",
    "--hint-spf",
    "--hint-file",
    "--hint-file-name",
    "--hint-dir",
    "--hint-fd",
    "--hint-all",
    "--hint",
    "--hint-n",
];

const SELECT_FLAGS: &[&str] = &[
    "--select-dir",
    "--select-ext",
    "--select-ext-name",
    "--select-file",
    "--select-file-name",
    "--select-fd",
    "--select-tp",
    "--select",
    "--select-n",
];

/// Names of all named hint value lists (`ral hintn` store, merged).
fn hint_list_names() -> Vec<String> {
    let mut names: Vec<String> = Config::get_run_hint_lists_merged().into_keys().collect();
    names.sort();
    names
}

/// Ghost completion while typing `ral add`/`ral edit` annotation tails:
/// `#param` names from the command template's placeholders, then the
/// `--hint-*` / `--select-*` flags after each `#param`.
/// `parent_parts` = tokens before the fragment, e.g.
/// `["ral", "add", "tld", "\"tl", "dir", "${x:-.}\""]`.
/// Returns ready-to-filter candidates (empty when N/A).
pub(crate) fn ral_hint_candidates_for(parent_parts: &[&str], last_frag: &str) -> Vec<String> {
    if parent_parts.len() < 3 {
        return vec![];
    }
    if parent_parts[0].to_lowercase() != "ral" {
        return vec![];
    }
    match parent_parts[1].to_lowercase().as_str() {
        "add" | "edit" => {}
        _ => return vec![],
    }
    // Template text = everything after the alias name (quotes harmless:
    // `$` scanning ignores them).
    let template = parent_parts[2..].join(" ");
    let idents = crate::shell::alias::placeholder_idents(&template);
    if idents.is_empty() {
        return vec![];
    }
    if last_frag.starts_with('#') {
        return idents
            .iter()
            .map(|(n, arr)| {
                if *arr {
                    format!("#{}[]", n)
                } else {
                    format!("#{}", n)
                }
            })
            .collect();
    }
    // Right after `--hint-n` / `--select-n`: complete list names
    // (unless a new flag is being typed).
    if !last_frag.starts_with('-') {
        if let Some(last) = parent_parts.last() {
            let lower = last.to_lowercase();
            if lower == "--hint-n" || lower == "--select-n" {
                return hint_list_names();
            }
        }
    }
    if last_frag.starts_with("--hint") {
        return HINT_FLAGS.iter().map(|s| s.to_string()).collect();
    }
    if last_frag.starts_with("--select") {
        return SELECT_FLAGS.iter().map(|s| s.to_string()).collect();
    }
    if last_frag.starts_with("--") {
        return HINT_FLAGS
            .iter()
            .chain(SELECT_FLAGS.iter())
            .map(|s| s.to_string())
            .collect();
    }
    // Just finished a `#param` naming a real placeholder: offer flags.
    if let Some(last) = parent_parts.last() {
        if let Some((name, _, _)) = crate::shell::alias::parse_hint_param(last) {
            if idents.iter().any(|(n, _)| n == &name) {
                return HINT_FLAGS
                    .iter()
                    .chain(SELECT_FLAGS.iter())
                    .map(|s| s.to_string())
                    .collect();
            }
        }
    }
    vec![]
}

/// Returns the list of ran target names from NTCRANFILE.toml for `ran <target>`.
pub(crate) fn ran_target_names_for_command(parts: &[&str]) -> Vec<String> {
    if parts.len() != 1 {
        return vec![];
    }
    if parts[0].to_lowercase() != "ran" {
        return vec![];
    }
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => return vec![],
    };
    match ran_parser::parse(&cwd) {
        Ok(ranfile) => ranfile.targets.into_keys().collect(),
        Err(_) => vec![],
    }
}

/// Returns the list of `*.ntc` script names in cwd for `scr <script>`.
pub(crate) fn scr_scripts_for_command(parts: &[&str]) -> Vec<String> {
    if parts.len() != 1 {
        return vec![];
    }
    let root = parts[0].to_lowercase();
    if root != "scr" && root != "script" {
        return vec![];
    }
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => return vec![],
    };
    match crate::script::list_scripts(&cwd) {
        Ok(scripts) => scripts
            .into_iter()
            .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .collect(),
        Err(_) => vec![],
    }
}

/// Returns the list of ntpkg/ntpx argument completions (languages, versions, installed packages).
pub(crate) fn ntpkg_names_for_command(parts: &[&str]) -> Vec<String> {
    if parts.len() < 2 {
        return vec![];
    }

    let root = parts[0].to_lowercase();
    if root != "ntpkg" && root != "ntpx" {
        return vec![];
    }

    if root == "ntpx" {
        // ntpx <pkg> [binary?] [args...]
        // Suggest installed package names at depth 1
        if parts.len() == 2 {
            return installed_package_names(2);
        }
        return vec![];
    }

    let sub = parts[1].to_lowercase();
    match sub.as_str() {
        // ntpkg install <lang> [version]
        "install" | "i" => {
            if parts.len() == 2 {
                // Suggest supported languages
                ntpkg::sources::list_supported_languages()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            } else if parts.len() == 3 {
                // Suggest cached versions for the specified language
                let lang = parts[2];
                ntpkg::sources::peek_cached_versions(lang)
            } else {
                vec![]
            }
        }
        // ntpkg uninstall <lang> [version]
        "uninstall" | "rm" => installed_package_names(parts.len()),
        // ntpkg use <lang> <version>
        "use" => installed_package_names(parts.len()),
        // ntpkg which <lang> [version]
        "which" => installed_package_names(parts.len()),
        // ntpkg available <lang>
        "available" | "avail" => {
            if parts.len() == 2 {
                ntpkg::sources::list_supported_languages()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            } else {
                vec![]
            }
        }
        _ => vec![],
    }
}

fn installed_package_names(depth: usize) -> Vec<String> {
    if depth != 2 {
        return vec![];
    }
    match ntpkg::manager::list_packages() {
        Ok(pkgs) => {
            let mut names: Vec<String> = pkgs.into_iter().map(|(n, _, _)| n).collect();
            names.sort();
            names.dedup();
            names
        }
        Err(_) => vec![],
    }
}

/// Returns group names (for `fake gen -g` / `fake group <name>`) or type
/// names (for `fake gen <types>` / `fake group add|edit` columns).
pub(crate) fn fake_names_for_command(parts: &[&str], _frag: &str) -> Vec<String> {
    if parts.is_empty() || parts[0].to_lowercase() != "fake" {
        return vec![];
    }
    let group_names = || crate::fake::group::group_names();
    let type_names = || -> Vec<String> {
        crate::fake::types::TYPES
            .iter()
            .map(|t| t.name.to_string())
            .collect()
    };

    // `fake gen -g <TAB>` (or `--group`) completes saved group names.
    if let Some(last) = parts.last() {
        let l = last.to_lowercase();
        if l == "-g" || l == "--group" {
            return group_names();
        }
        if l == "-l" || l == "--locale" {
            return crate::fake::locale::Locale::all()
                .iter()
                .map(|x| x.code().to_string())
                .collect();
        }
    }

    if parts.len() < 2 {
        return vec![];
    }
    match parts[1].to_lowercase().as_str() {
        "gen" | "generate" | "g" => type_names(),
        "types" | "type" | "t" => type_names(),
        "group" | "grp" | "groups" => {
            if parts.len() < 3 {
                return vec![];
            }
            match parts[2].to_lowercase().as_str() {
                "rm" | "remove" | "delete" | "info" | "show" | "rnm" | "rename" => group_names(),
                // `group add|edit <name> <cols>`: the name is free text, the
                // column spec takes type names.
                "add" | "a" | "edit" | "ed" if parts.len() >= 4 => type_names(),
                _ => vec![],
            }
        }
        _ => vec![],
    }
}

/// Returns possible subcommand completions for a command path.
/// `parts` is the prefix path (the words before the one being typed).
///   - If `parts.len() == 1` (e.g. `["ral"]`), returns the subcommands of `ral`.
///   - If `parts.len() >= 2` (e.g. `["ral", "export"]`), drills deeper.
/// `-g` / `--global` hints.
///
/// Only offered inside a directory that has an `ntconfig.toml`: there the
/// default target of a write is the local file, so the flag is the explicit
/// way to reach the global config. Outside such a directory the default is
/// already the global config and the flag would be noise.
fn global_scope_flags() -> Vec<&'static str> {
    if Config::get_local_config_path().is_some() {
        vec!["-g", "--global"]
    } else {
        vec![]
    }
}

pub(crate) fn subcommands_for(parts: &[&str]) -> Vec<&'static str> {
    if parts.is_empty() {
        return vec![];
    }

    let root = parts[0].to_lowercase();
    match root.as_str() {
        // ral add / ral edit / ral rm / ral rnm / ral list / ral hintn ...
        //   ral export --all / ral export --select
        //   ral hintn add / rm / rnm / info / ls
        "ral" => {
            if parts.len() == 1 {
                vec![
                    "add", "edit", "rm", "remove", "list", "ls", "info", "rnm", "rename", "export",
                    "import", "make", "mk", "cls", "clear", "hintn",
                ]
            } else {
                match parts[1].to_lowercase().as_str() {
                    "export" => vec!["--all", "-a", "--select", "-s"],
                    "info" => vec!["--edit", "-e"],
                    "add" | "edit" | "rm" | "remove" | "cls" | "clear" | "rnm" | "rename"
                    | "import" => global_scope_flags(),
                    "hintn" => {
                        if parts.len() == 2 {
                            vec!["add", "rm", "remove", "rnm", "rename", "info", "list", "ls"]
                        } else {
                            match parts[2].to_lowercase().as_str() {
                                "rm" | "remove" => {
                                    let mut flags = vec!["--force", "-f"];
                                    flags.extend(global_scope_flags());
                                    flags
                                }
                                "add" | "rnm" | "rename" => global_scope_flags(),
                                _ => vec![],
                            }
                        }
                    }
                    _ => vec![],
                }
            }
        }
        // tp add / tp jump / tp list / tp info / tp rm / tp rnm / tp cls / tp help
        //   tp jump <name> and tp to <name> suggest teleports via teleport_names_for_command
        "tp" => vec![
            "add", "jump", "to", "info", "list", "ls", "rm", "rnm", "rename", "cls", "help",
        ],
        // (watch is now a subcommand of config)
        // tpb history / tpb clear
        "tpb" => vec!["history", "hist", "h", "clear", "cls"],
        "theme" => vec![
            "add", "rm", "remove", "edit", "info", "export", "import", "make", "mk", "where", "wh",
            "reload", "n", "number", "rnm", "rename", "list", "ls",
        ],
        // igcare export / igcare import / igcare export --all / igcare export --select
        "igcare" => {
            if parts.len() == 1 {
                vec!["export", "import", "make", "mk"]
            } else {
                match parts[1].to_lowercase().as_str() {
                    "export" => vec!["--all", "-a", "--select", "-s"],
                    _ => vec![],
                }
            }
        }
        // serve files / serve http-server / serve chess / serve config
        // (+ flags, like nsheet: one arm carries both so `serve --a`
        // still ghosts `--auth`)
        "serve" if parts.len() == 1 => vec![
            "files",
            "http-server",
            "chess",
            "config",
            "help",
            "--port",
            "-p",
            "--bind",
            "-b",
            "--auth",
            "--tunnel",
            "-t",
            "--share",
            "--qr",
            "--no-qr",
            "--lock",
            "--spa",
            "--web-dir",
            "-w",
            "--help",
            "-h",
        ],
        // serve config <service> [--port-default N] [--bind-default ADDR]
        "serve" if parts.len() >= 2 && parts[1].eq_ignore_ascii_case("config") => {
            if parts.len() == 2 {
                vec!["files", "http-server", "chess", "help"]
            } else {
                vec!["--port-default", "--bind-default"]
            }
        }
        // `gs <pattern> [-p <glob>] [-s [line|path|size]] [-d] [-l] [-i]`:
        // flags only — pattern/path values are free text.
        "gs" => vec!["-p", "--path", "-s", "--sort", "-d", "-l", "--limit", "-i"],
        "serve" if parts.len() >= 2 && parts[1].eq_ignore_ascii_case("files") => {
            vec![
                "--port",
                "-p",
                "--bind",
                "-b",
                "--auth",
                "--lock",
                "--tunnel",
                "-t",
                "--share",
                "--qr",
                "--no-qr",
                "--spa",
                "--web-dir",
                "-w",
                "--help",
                "-h",
            ]
        }
        // fake gen / fake types / fake group ...
        //   fake group add|edit|rm|rnm|ls|info|clear
        "fake" => {
            if parts.len() == 1 {
                vec!["gen", "types", "locales", "group", "help"]
            } else {
                match parts[1].to_lowercase().as_str() {
                    "gen" | "generate" | "g" => vec![
                        "-n", "--count", "-f", "--format", "-s", "--seed", "-o", "--output", "-g",
                        "--group", "-l", "--locale", "-u", "--unique",
                    ],
                    "types" | "type" | "t" => vec!["-s", "--search"],
                    "locales" | "locale" | "loc" => vec![],
                    "group" | "grp" | "groups" => {
                        if parts.len() == 2 {
                            vec![
                                "add", "edit", "rm", "remove", "rnm", "rename", "ls", "list",
                                "info", "clear", "export", "import", "help",
                            ]
                        } else {
                            match parts[2].to_lowercase().as_str() {
                                "add" | "a" | "edit" | "ed" => vec![],
                                "rm" | "remove" | "delete" => vec!["--force", "-f"],
                                "export" | "exp" => vec!["--all", "-a", "-o", "--output"],
                                "import" | "imp" => vec!["--force", "-f", "--prefix", "-p"],
                                _ => vec![],
                            }
                        }
                    }
                    _ => vec![],
                }
            }
        }
        // ran init / ran deinit / ran list
        "ran" if parts.len() == 1 => vec!["init", "deinit", "help", "list", "ls"],
        // scr init / scr ls / scr help
        "scr" | "script" if parts.len() == 1 => vec!["init", "ls", "list", "help"],
        // deadlock ls / add / rmpw / master-pass / forget / destroy / help / exit
        "deadlock" if parts.len() == 1 => vec![
            "ls",
            "add",
            "rmpw",
            "master-pass",
            "forget",
            "destroy",
            "help",
            "exit",
        ],
        // nssh add / ssh / cp / scan / ls / info / rm / edit / keygen
        "nssh" if parts.len() == 1 => vec![
            "add", "ssh", "cp", "scan", "ls", "list", "info", "rm", "edit", "keygen",
        ],
        "nssh" if parts.len() >= 2 => match parts[1].to_lowercase().as_str() {
            "add" | "edit" => vec![
                "--host",
                "--user",
                "-u",
                "--port",
                "-p",
                "--key",
                "--no-key",
                "--pass-env",
                "--no-pass-env",
            ],
            "ssh" => vec!["--cmd", "--trust", "--ask-pass", "--timeout"],
            "cp" => vec!["--trust", "--ask-pass", "--timeout"],
            "scan" => vec!["--ports", "--top", "--timeout"],
            "rm" => vec!["--yes", "-y"],
            _ => vec![],
        },
        // config output / config depth / config linenumbers / config color / config autosuggest / config threads / config history / config watch
        "config" | "cfg" => {
            if parts.len() == 1 {
                vec![
                    "output",
                    "out",
                    "depth",
                    "d",
                    "linenumbers",
                    "ln",
                    "line",
                    "color",
                    "c",
                    "autosuggest",
                    "as",
                    "threads",
                    "t",
                    "history",
                    "h",
                    "watch",
                    "w",
                    "qsc",
                    "serve",
                    "lang",
                    "language",
                    "git",
                    "dir_view",
                    "dv",
                    "file_view",
                    "fv",
                    "todo",
                    "td",
                    "size",
                    "s",
                    "update_check",
                    "uc",
                    "intro_view",
                    "iv",
                ]
            } else {
                match parts[1].to_lowercase().as_str() {
                    "watch" | "w" if parts.len() == 2 => vec!["ON", "OFF", "trigger", "limit"],
                    "watch" | "w" if parts.len() >= 3 => vec!["off"],
                    "lang" | "language" if parts.len() == 2 => {
                        vec!["english", "thai", "lao", "chinese"]
                    }
                    "git" if parts.len() == 2 => vec!["ON", "OFF"],
                    "dir_view" | "dv" if parts.len() == 2 => vec!["ON", "OFF"],
                    "file_view" | "fv" if parts.len() == 2 => vec!["ON", "OFF"],
                    "todo" | "td" if parts.len() == 2 => vec!["format", "fmt"],
                    "todo" | "td" if parts.len() == 3 => {
                        vec!["txt", "md", "json", "toml", "yaml", "html"]
                    }
                    "size" | "s" if parts.len() == 2 => {
                        vec!["ON", "OFF", "all", "file", "dir"]
                    }
                    "size" | "s" if parts.len() == 3 => vec!["ON", "OFF"],
                    "update_check" | "uc" if parts.len() == 2 => vec!["ON", "OFF"],
                    "intro_view" | "iv" if parts.len() == 2 => vec!["ON", "OFF"],
                    "serve" if parts.len() == 2 => {
                        vec!["files", "http-server", "chess", "help"]
                    }
                    "serve" if parts.len() >= 3 => {
                        vec!["--port-default", "--bind-default"]
                    }
                    _ => vec![],
                }
            }
        }
        // backup create / backup restore / backup undo / backup clear / backup recreate / backup diff / backup list
        "backup" if parts.len() == 1 => vec![
            "create", "cr", "restore", "rs", "undo", "ud", "clear", "cl", "recreate", "re", "diff",
            "list", "ls",
        ],
        // backup restore --choose / --deleted / --force
        "backup" if parts.len() >= 2 && matches!(parts[1], "restore" | "rs") => {
            vec!["--choose", "--deleted", "--force", "-f"]
        }
        // backup undo --cls / backup undo --force
        "backup" if parts.len() >= 2 && matches!(parts[1], "undo" | "ud") => {
            vec!["--cls", "--clear", "--force", "-f"]
        }
        // backup clear --force
        "backup" if parts.len() >= 2 && matches!(parts[1], "clear" | "cl") => vec!["--force", "-f"],
        // backup recreate --force
        "backup" if parts.len() >= 2 && matches!(parts[1], "recreate" | "re") => {
            vec!["--force", "-f"]
        }
        // report project <fmt> ... / report clip <n>
        "report" if parts.len() == 1 => vec!["project", "clip"],
        // report clip <n> - no inline completions
        "report" if parts.len() == 2 && parts[1] == "clip" => vec![],
        // report project <fmt> [path] [--name <file>]
        "report" if parts.len() >= 2 && parts[1] == "project" => {
            let mut out = Vec::new();
            if parts.len() == 2 {
                out.extend(["txt", "json", "md", "html", "pdf", "docx", "xlsx"]);
            }
            out.extend(["--name", "-n"]);
            out
        }
        // ignore dir / ignore ext / ignore file
        "ignore" if parts.len() == 1 => vec!["dir", "ext", "file"],
        // ignore dir <name> / ignore ext <ext> / ignore file <name>
        "ignore" if parts.len() >= 2 && matches!(parts[1], "dir" | "ext" | "file") => {
            global_scope_flags()
        }
        // care dir / care ext / care file
        "care" if parts.len() == 1 => vec!["dir", "ext", "file"],
        // care dir <name> / care ext <ext> / care file <name>
        "care" if parts.len() >= 2 && matches!(parts[1], "dir" | "ext" | "file") => {
            global_scope_flags()
        }
        // local init/deinit/help
        "local" => vec!["init", "deinit", "help"],
        // init --all / init --ran / init --local
        "init" if parts.len() == 1 => vec![
            "--all",
            "-a",
            "--ran",
            "--local",
            "--opencode",
            "-oc",
            "--force",
            "-f",
        ],
        // deinit --all / deinit --ran / deinit --local
        "deinit" if parts.len() == 1 => vec![
            "--all",
            "-a",
            "--ran",
            "--local",
            "--opencode",
            "-oc",
            "--force",
            "-f",
        ],
        // showenv --all / showenv -d
        "showenv" if parts.len() == 1 => vec!["--all", "-a", "-d", "--depth"],
        // size --care / size -p <path> / size --path <path>
        "size" if parts.len() == 1 => vec!["--care", "-p", "--path"],
        // mv -f / mv --force / cp -f / cp --force
        "mv" | "move" | "cp" if parts.len() == 1 => vec!["-f", "--force"],
        // view flags
        "view" if parts.len() == 1 => vec![
            "-s",
            "--size",
            "-fs",
            "--filesize",
            "-ds",
            "--dirsize",
            "--head",
            "--tail",
            "--betw",
            "--sort",
            "-f",
            "--file",
            "-dir",
            "--directory",
            "-c",
            "--care",
            "-cp",
            "--copy",
            "-d",
            "--depth",
            "-t",
            "--time",
            "-p",
            "--path",
            "-ext",
            "--ext",
            "-e",
        ],
        // views flags (flat ls-style listing)
        "views" if parts.len() == 1 => vec!["-n", "-s", "--size", "--sort"],
        "views" if parts.len() >= 2 => {
            let last = parts[parts.len() - 1].to_lowercase();
            if last == "-s" || last == "--size" || last == "--sort" {
                vec![]
            } else {
                vec!["-n", "-s", "--size", "--sort"]
            }
        }
        // ui (now a no-op after modern mode removal)
        "ui" => vec!["classic"],
        // ntpkg install / ntpkg uninstall / ntpkg list / ntpkg use / ntpkg which / ntpkg available / ntpkg search / ntpkg info
        "ntpkg" => vec![
            "install",
            "i",
            "uninstall",
            "rm",
            "list",
            "ls",
            "search",
            "info",
            "use",
            "which",
            "available",
            "avail",
            "help",
        ],
        "ntpx" => vec![], // Completions come from ntpkg_names_for_command
        // tl file / tl ext / tl dir
        "tl" if parts.len() == 1 => vec!["file", "ext", "dir"],
        "tl" if parts.len() == 2 && matches!(parts[1], "ext" | "dir") => {
            vec!["-s", "-n", "-l", "-r"]
        }
        // qur flags (must match parse_qur_args — no phantom flags).
        // A value-taking flag awaiting its value offers nothing.
        "qur" if parts.len() >= 1 => {
            let last = parts[parts.len() - 1].to_lowercase();
            if last == "-s"
                || last == "--select"
                || last == "-rp"
                || last == "--replace"
                || last == "-d"
                || last == "--depth"
                || last == "-l"
                || last == "--limit"
            {
                vec![]
            } else {
                vec![
                    "-s",
                    "--select",
                    "-rp",
                    "--replace",
                    "-dp",
                    "--display",
                    "-d",
                    "--depth",
                    "-l",
                    "--limit",
                ]
            }
        }
        // nms play / scan / help + scan flags (must match cmd_nms's parser).
        "nms" if parts.len() == 1 => vec!["play", "scan", "help"],
        "nms" if parts.len() >= 2 && parts[1].to_lowercase() == "scan" => {
            vec!["-d", "--depth", "-s", "--size", "-l", "--long", "--care"]
        }
        // nsheet open / view + flags (must match cmd_nsheet's parser)
        "nsheet" => vec![
            "open",
            "view",
            "-d",
            "--delimiter",
            "--header",
            "--no-header",
            "-l",
            "--limit",
            "--all",
            "--sort",
            "--desc",
            "--filter",
            "--export",
            "-o",
            "--output",
            "--help",
            "-h",
        ],
        // dl flags
        "dl" if !parts.is_empty() => vec![
            "--progress",
            "--resume",
            "--stop",
            "--cancel",
            "-o",
            "--output",
        ],
        // ping flags
        "ping" if !parts.is_empty() => vec!["-c", "--count"],
        // dns flags
        "dns" if !parts.is_empty() => vec!["-t", "--type"],
        // serve chess flags (no --auth/--spa: rejected by cmd_serve_chess)
        "serve" if parts.len() >= 2 && parts[1].eq_ignore_ascii_case("chess") => {
            vec![
                "--port",
                "-p",
                "--bind",
                "-b",
                "--tunnel",
                "-t",
                "--share",
                "--qr",
                "--no-qr",
                "--web-dir",
                "-w",
                "--help",
                "-h",
            ]
        }
        // serve http-server flags (no --spa/--web-dir: rejected by cmd_serve_http)
        "serve" if parts.len() >= 2 && parts[1].eq_ignore_ascii_case("http-server") => {
            vec![
                "--port", "-p", "--bind", "-b", "--auth", "--tunnel", "-t", "--share", "--qr",
                "--no-qr", "--help", "-h",
            ]
        }
        // serve flags (must match cmd_serve's parser — no phantom flags)
        "serve" if !parts.is_empty() => vec![
            "--port",
            "-p",
            "--bind",
            "-b",
            "--auth",
            "--tunnel",
            "-t",
            "--share",
            "--qr",
            "--no-qr",
            "--spa",
            "--web-dir",
            "-w",
            "--help",
            "-h",
        ],
        // smtp flags
        "smtp" if parts.len() == 1 => vec!["-i", "--server", "--id-ls"],
        "smtp" if parts.len() >= 2 => vec![
            "-i",
            "--server",
            "-p",
            "--port",
            "-u",
            "--user",
            "-pwd",
            "--password",
            "--id",
            "--id-ls",
            "--id-info",
            "--deadlock",
            "--id-rm",
            "-t",
            "--to",
            "-c",
            "--cc",
            "--bcc",
            "-s",
            "--subject",
            "-b",
            "--body",
            "--body-file",
            "-bf",
            "--body-format",
            "-f",
            "--attach",
            "-h",
            "--help",
        ],
        // up flags
        "up" if !parts.is_empty() => vec!["-H", "--header", "-d", "--data", "--post"],
        // http methods then flags
        "http" if parts.len() == 1 => vec!["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD"],
        "http" if parts.len() >= 2 => vec![
            "-H",
            "--header",
            "-d",
            "--data",
            "--json",
            "-q",
            "--query",
            "-a",
            "--auth",
            "--bearer",
            "-t",
            "--timeout",
            "-s",
            "--silent",
            "-o",
            "--output",
            "-v",
            "--verbose",
            "--no-pretty",
            "-h",
            "--help",
        ],
        // ne --init|--super <file> [...] [-I <file>] / ne --hyper <name> --lang <lang> [-I]
        "ne" | "ntceditor" if parts.len() == 1 => vec!["--init", "--super", "--hyper"],
        "ne" | "ntceditor" if parts.len() >= 2 && parts[1] == "--hyper" => vec!["--lang", "-I"],
        "ne" | "ntceditor" if parts.len() >= 2 && matches!(parts[1], "--init" | "--super") => {
            vec!["-I"]
        }
        // clip list / select / clear
        "clip" if parts.len() == 1 => vec!["list", "ls", "select", "sel", "clear", "cl"],
        // qsc listen / stop / status
        "qsc" if parts.len() == 1 => vec!["listen", "start", "stop", "status", "stat", "startup"],
        // todo info / add / rm / clear
        "todo" | "td" if parts.len() == 1 => {
            vec![
                "info", "add", "done", "undo", "edit", "rm", "remove", "clear", "cl", "list", "ls",
            ]
        }
        // arc create / extract / list
        "arc" if parts.len() == 1 => {
            vec!["create", "extract", "list", "help"]
        }
        // `arc create <out> …`: the output slot takes a free-form archive
        // name (no ghost hint — usage comes from `arc create --help`);
        // deeper positions are input paths (handled by fs candidates).
        // `arc list|extract <archive> [dest]`: paths only, no subcommands.
        "arc" if parts.len() >= 2 => vec![],
        // view flags at any depth (`view -ds --<Tab>` keeps suggesting).
        // A value-taking flag awaiting its value offers nothing.
        "view" if parts.len() >= 2 => {
            let last = parts[parts.len() - 1].to_lowercase();
            // `-s`/`-fs`/`-ds` stay suggestible: bare they take a path
            // next, valued they take effect — only the always-valued
            // flags suppress ghosting while awaiting a value.
            if last == "-d"
                || last == "--depth"
                || last == "-p"
                || last == "--path"
                || last == "-ext"
                || last == "--ext"
                || last == "--head"
                || last == "--tail"
                || last == "--betw"
                || last == "--sort"
                || last == "-e"
            {
                vec![]
            } else {
                vec![
                    "-s",
                    "--size",
                    "-fs",
                    "--filesize",
                    "-ds",
                    "--dirsize",
                    "--head",
                    "--tail",
                    "--betw",
                    "--sort",
                    "-f",
                    "--file",
                    "-dir",
                    "--directory",
                    "-c",
                    "--care",
                    "-cp",
                    "--copy",
                    "-d",
                    "--depth",
                    "-t",
                    "--time",
                    "-p",
                    "--path",
                    "-ext",
                    "--ext",
                    "-e",
                ]
            }
        }
        // viewf flags (fixed depth 1, files only).
        "viewf" if parts.len() == 1 => vec![
            "-s",
            "--size",
            "-fs",
            "--filesize",
            "--sort",
            "-e",
            "-ext",
            "--ext",
            "-c",
            "--care",
            "-cp",
            "--copy",
            "-t",
            "--time",
            "-p",
            "--path",
            "--head",
            "--tail",
            "--betw",
        ],
        "viewf" if parts.len() >= 2 => {
            let last = parts[parts.len() - 1].to_lowercase();
            if last == "--head"
                || last == "--tail"
                || last == "--betw"
                || last == "--sort"
                || last == "-e"
                || last == "-ext"
                || last == "--ext"
                || last == "-p"
                || last == "--path"
            {
                vec![]
            } else {
                vec![
                    "-s",
                    "--size",
                    "-fs",
                    "--filesize",
                    "--sort",
                    "-e",
                    "-ext",
                    "--ext",
                    "-c",
                    "--care",
                    "-cp",
                    "--copy",
                    "-t",
                    "--time",
                    "-p",
                    "--path",
                    "--head",
                    "--tail",
                    "--betw",
                ]
            }
        }
        // viewd flags (directories only).
        "viewd" if parts.len() == 1 => vec![
            "-d",
            "--depth",
            "-s",
            "--size",
            "-ds",
            "--dirsize",
            "-c",
            "--care",
            "--sort",
            "-t",
            "--time",
            "-cp",
            "--copy",
            "-p",
            "--path",
        ],
        "viewd" if parts.len() >= 2 => {
            let last = parts[parts.len() - 1].to_lowercase();
            if last == "-d"
                || last == "--depth"
                || last == "--sort"
                || last == "-p"
                || last == "--path"
            {
                vec![]
            } else {
                vec![
                    "-d",
                    "--depth",
                    "-s",
                    "--size",
                    "-ds",
                    "--dirsize",
                    "-c",
                    "--care",
                    "--sort",
                    "-t",
                    "--time",
                    "-cp",
                    "--copy",
                    "-p",
                    "--path",
                ]
            }
        }
        // txtf display-range flags (must match parse_txtf_args).
        // A value-taking flag awaiting its value offers nothing.
        "txtf" if parts.len() >= 1 => {
            let last = parts[parts.len() - 1].to_lowercase();
            if last == "--head" || last == "--tail" || last == "--betw" {
                vec![]
            } else {
                vec!["--head", "--tail", "--betw", "--line", "--no-line"]
            }
        }
        // scan ext <ext...> [-d|--depth <n>] [-s|--size] [--care]
        // scan files [pattern] [-d|--depth <n>] [-s|--size] [--care]
        // scan dir [pattern] [-d|--depth <n>] [-s|--size] [--care]
        // scan port [host] [--top N | --ports <spec> | --all] [--timeout MS]
        // scan ip [cidr] [--top N | --ports <spec>] [--timeout MS]
        "scan" if parts.len() == 1 => vec!["ext", "files", "dir", "port", "ip", "help"],
        "scan"
            if parts.len() >= 2
                && matches!(parts[1].to_lowercase().as_str(), "ext" | "files" | "dir") =>
        {
            vec!["-d", "--depth", "-s", "--size", "--care"]
        }
        "scan" if parts.len() >= 2 && parts[1].to_lowercase() == "port" => {
            vec!["--top", "--ports", "--all", "--timeout"]
        }
        "scan" if parts.len() >= 2 && parts[1].to_lowercase() == "ip" => {
            vec!["--top", "--ports", "--timeout"]
        }
        // encode b2 / b10 / b64 / hex / url
        "encode" if parts.len() == 1 => vec!["b2", "b10", "b64", "base64", "hex", "url"],
        // decode b2 / b10 / b64 / hex / url
        "decode" if parts.len() == 1 => vec!["b2", "b10", "b64", "base64", "hex", "url"],
        // encode/decode <codec> <text|file>: the output-path flag.
        "encode" | "decode" if parts.len() >= 2 => vec!["-o", "--output"],
        // fmt <file> … : after the source file, offer the keyword/flag
        // slots (`to <ext>` conversion, `-o` output path).
        "fmt" if parts.len() == 2 => vec!["to", "-o", "--output"],
        // fmt … to <ext>: the conversion-target slot.
        "fmt" if parts.len() >= 3 && parts[parts.len() - 1].to_lowercase() == "to" => {
            vec![
                "json", "yaml", "toml", "md", "html", "xml", "docx", "txt", "pdf", "csv", "tsv",
                "xlsx",
            ]
        }
        // qr encode / decode
        "qr" if parts.len() == 1 => vec!["encode", "en", "decode", "de"],
        // image display / show / to / out / --ocr / --layout / --sixel
        "image" | "img" if parts.len() == 1 => {
            vec![
                "display", "show", "to", "out", "--ocr", "--layout", "--sixel",
            ]
        }
        // image display|show [--sixel] [<path> [--sixel]]: the only flag
        // the display tail accepts (cf. parse_display_tail in image.rs).
        "image" | "img"
            if parts.len() >= 2
                && matches!(parts[1].to_lowercase().as_str(), "display" | "show") =>
        {
            vec!["--sixel"]
        }
        // mode subcommands
        "mode" if parts.len() == 1 => vec!["admin", "normal"],
        // usrpath / syspath subcommands
        "usrpath" | "syspath" if parts.len() == 1 => {
            vec!["show", "ls", "add", "rm", "remove", "move", "mv"]
        }
        "usrpath" | "syspath" if parts.len() >= 2 && matches!(parts[1], "add") => {
            vec!["--cwd", "--here"]
        }
        // autoup subcommands
        "autoup" if parts.len() == 1 => vec![
            "list", "ls", "add", "rm", "remove", "disable", "enable", "help",
        ],
        // sort flags
        "sort" if parts.len() == 1 => vec![
            "--reverse",
            "-r",
            "--numeric",
            "-n",
            "--unique",
            "-u",
            "--ignore-case",
            "-f",
        ],
        // say flags
        "say" | "print" if parts.len() == 1 => vec!["--loop", "-n", "-t"],
        // tmprt com / weather / key / city / sensors
        "tmprt" if parts.len() == 1 => vec![
            "com", "computer", "hw", "hardware", "weather", "w", "key", "city", "sensors", "sensor",
        ],
        // tmprt sensors <sub>
        "tmprt" if parts.len() >= 2 && matches!(parts[1], "sensors" | "sensor") => {
            vec!["status", "install", "uninstall", "debug"]
        }
        // time command forms
        "time" if parts.len() == 1 => vec![
            "run", "-r", "--run", "compare", "bench", "count", "help", "--help",
        ],
        "time" if parts.len() == 2 && parts[1] == "compare" => {
            vec!["-w", "--ignore-whitespace", "--with"]
        }
        "time" if parts.len() == 2 && parts[1] == "bench" => vec!["-n", "--runs"],
        // git subcommands
        "git" if parts.len() == 1 => vec![
            "status", "st", "branch", "b", "log", "l", "diff", "d", "add", "a", "commit", "push",
            "pull", "clone", "checkout", "co", "merge", "fetch", "remote", "init", "stash",
            "rebase", "tag", "help", "h",
        ],
        // `help <topic>`: one canonical name per `print_topic_help`
        // section (no aliases).
        "help" if parts.len() == 1 => vec![
            "nav",
            "config",
            "editor",
            "theme",
            "teleport",
            "ran",
            "scr",
            "alias",
            "view",
            "size",
            "report",
            "backup",
            "search",
            "igcare",
            "network",
            "tmprt",
            "time",
            "git",
            "clipboard",
            "qsc",
            "todo",
            "arc",
            "scan",
            "other",
            "syscmd",
            "pipe",
            "fileops",
            "tl",
            "ntpkg",
            "qr",
            "pathman",
            "autoup",
            "mode",
            "explorer",
            "math",
            "python",
            "encdec",
            "fmt",
            "image",
            "ndb",
            "nms",
            "nsheet",
            "nssh",
            "fake",
            "deadlock",
        ],
        // Default: no known subcommands
        _ => vec![],
    }
}

/// Tab-cycle cursor: remembers the line the ghost was computed for and which
/// candidate index is currently shown. Shared between the hinter (which reads
/// it) and the Tab key handler (which advances it).
#[derive(Debug, Default)]
pub(crate) struct CycleState {
    line: String,
    pos: usize,
    index: isize,
}

/// A rustyline `Hinter` that suggests autocompletion as ghost text.
/// Tab cycles through all candidates as ghost text; Right-arrow accepts.
pub(crate) struct NtcHinter {
    commands: RwLock<HashSet<String>>,
    generation: AtomicU64,
    cycle: Arc<Mutex<CycleState>>,
}

impl NtcHinter {
    pub fn new() -> Self {
        NtcHinter {
            commands: RwLock::new(build_command_set()),
            generation: AtomicU64::new(ALIAS_GENERATION.load(Ordering::Relaxed)),
            cycle: Arc::new(Mutex::new(CycleState::default())),
        }
    }

    fn ensure_fresh(&self) {
        let current = ALIAS_GENERATION.load(Ordering::Relaxed);
        let cached = self.generation.load(Ordering::Relaxed);
        if current != cached {
            let mut cmds = self.commands.write().unwrap();
            *cmds = build_command_set();
            self.generation.store(current, Ordering::Relaxed);
        }
    }

    /// Handle for the Tab key handler so it can advance the same cycle state.
    pub(crate) fn tab_handler(&self, delta: isize) -> TabCycleHandler {
        TabCycleHandler {
            cycle: Arc::clone(&self.cycle),
            delta,
        }
    }

    /// Tail of the line after the last top-level operator, plus that
    /// operator (`&&`, `||`, `|`, `>`, `>>`, `<`, `<<`, `<<<`). Untrimmed —
    /// the trailing space tells a filled slot (`ran `) apart from a
    /// half-typed word (`ran`). Operators inside quotes or `[...]` arrays,
    /// and unspaced ones (`a|b`), do not split.
    pub(crate) fn stage_tail(typed: &str) -> (&str, Option<&'static str>) {
        match super::pipeline::active_operator(typed) {
            Some((op, tail)) => (tail, Some(op)),
            None => (typed, None),
        }
    }

    /// Full candidate words for the word being typed in `typed`
    /// (cursor assumed at end of `typed`). Used both for ghost text and Tab cycling.
    pub(crate) fn candidates_for(&self, typed: &str) -> Vec<String> {
        if typed.is_empty() {
            return vec![];
        }
        // Complete only the active segment/slot after the last operator.
        let (tail, op) = Self::stage_tail(typed);
        match op {
            // `>` takes a free-form output path: never hint it.
            Some(">") => return vec![],
            // `>>`/`<` append to / read from an existing text file.
            Some(">>") | Some("<") => return redirect_target_candidates(tail),
            // Heredocs use the EOF delimiter; here-strings take free text.
            Some("<<") => return heredoc_delim_candidates(tail),
            Some("<<<") => return vec![],
            Some("|") => {
                if let Some(cands) = pipe_stage_commands(tail) {
                    return cands;
                }
            }
            _ => {}
        }
        let typed = tail;
        if typed.is_empty() {
            return vec![];
        }
        let trailing_space = typed.ends_with(' ') || typed.ends_with('\t');
        let parts: Vec<&str> = typed.split_whitespace().collect();
        if parts.is_empty() {
            return vec![];
        }

        // Any `--help` / `-h` token in the line switches to contextual-help
        // mode (handled on Enter). Suppress ghost text so it doesn't fight
        // the help output, and let `--` still suggest `--help` below.
        if parts.iter().any(|p| is_help_token(p)) {
            return vec![];
        }

        if parts.len() == 1 && !trailing_space {
            // ── Level 1: top-level commands ──
            let word = parts[0];
            if word.starts_with('-') {
                return filter_prefix(vec!["--help".to_string(), "-h".to_string()], word);
            }
            if let Some(tp_part) = word.strip_prefix('@') {
                // Bare `@` lists every save-point; `@fr` filters.
                // (`filter_prefix` with an empty frag returns all names.)
                // The `@` stays in the comparison: filtering `@names`
                // by the bare stub (`"@shop"` vs `"s"`) never matches.
                return filter_prefix(
                    TeleportManager::get_all()
                        .into_keys()
                        .map(|n| format!("@{}", n))
                        .collect(),
                    &format!("@{}", tp_part),
                );
            }
            let lower = word.to_lowercase();
            let cmds = self.commands.read().unwrap();
            let mut out: Vec<String> = cmds
                .iter()
                .filter(|c| c.starts_with(&lower) && c.len() > word.len())
                .cloned()
                .collect();
            out.sort();
            return out;
        }

        // ── Level 2+: subcommands / teleport / alias / ran / scr / ntpkg / theme / fs ──
        let (parent_parts, last_frag): (&[&str], &str) = if trailing_space {
            (&parts[..], "")
        } else {
            (&parts[..parts.len() - 1], parts[parts.len() - 1])
        };
        if parent_parts.is_empty() {
            return vec![];
        }

        // `cmd -` / `cmd --d` suggests the universal help flag plus the
        // command's own flags (e.g. `view -d` -> `-ds`,
        // `view --d` -> `--depth`/`--dirsize`). Flags are case-sensitive.
        if last_frag.starts_with('-') {
            let mut flags: Vec<String> = vec!["--help".to_string(), "-h".to_string()];
            flags.extend(
                subcommands_for(parent_parts)
                    .into_iter()
                    .map(|s| s.to_string()),
            );
            flags.extend(ral_hint_candidates_for(parent_parts, last_frag));
            let mut out: Vec<String> = flags
                .into_iter()
                .filter(|n| n.starts_with(last_frag) && n.len() > last_frag.len())
                .collect();
            out.sort();
            out.dedup();
            return out;
        }

        let mut out: Vec<String> = Vec::new();

        // Bare `cd` takes only subdirectory paths (`cd @name` is not a
        // teleport jump — only `cd to <name>` is), so no bookmark/@ names here.
        let is_bare_cd = parent_parts.len() == 1 && parent_parts[0].to_lowercase() == "cd";

        // @<name> inside a command argument (bare `@` lists all).
        // Like above, the `@` stays in the comparison fragment.
        if let Some(tp_part) = last_frag.strip_prefix('@') {
            if is_bare_cd {
                return vec![];
            }
            // `@tp/rest` lists the savepoint's contents (files plain, dirs
            // with trailing `/`) for the commands whose resolvers accept
            // sub-paths: `mv`/`move`/`cp` src, `view`/`viewf`/`viewd`,
            // `txtf`, and `mkf`/`mkd`/`rmf`/`rmd`. Other commands and their
            // filled slots keep the savepoint-name behavior below.
            if let Some(cands) = tp_subpath_candidates(parent_parts, last_frag, tp_part) {
                return dedup_sorted(cands);
            }
            out.extend(filter_prefix(
                TeleportManager::get_all()
                    .into_keys()
                    .map(|n| format!("@{}", n))
                    .collect(),
                &format!("@{}", tp_part),
            ));
            return dedup_sorted(out);
        }

        if !is_bare_cd {
            out.extend(filter_prefix(
                teleport_names_for_command(parent_parts),
                last_frag,
            ));
        }
        out.extend(filter_prefix(
            alias_names_for_command(parent_parts),
            last_frag,
        ));
        out.extend(filter_prefix(
            hint_list_names_for_command(parent_parts),
            last_frag,
        ));
        out.extend(filter_prefix(
            ran_target_names_for_command(parent_parts),
            last_frag,
        ));
        out.extend(filter_prefix(
            scr_scripts_for_command(parent_parts),
            last_frag,
        ));
        out.extend(filter_prefix(
            ntpkg_names_for_command(parent_parts),
            last_frag,
        ));
        out.extend(filter_prefix(
            theme_names_for_command(parent_parts),
            last_frag,
        ));
        out.extend(filter_prefix(
            nssh_profile_names_for_command(parent_parts),
            last_frag,
        ));
        out.extend(filter_prefix(
            ext_names_for_command(parent_parts, last_frag),
            last_frag,
        ));
        out.extend(filter_prefix(
            fake_names_for_command(parent_parts, last_frag),
            last_frag,
        ));
        out.extend(filter_prefix(
            fmt_output_candidates_for(parent_parts, last_frag),
            last_frag,
        ));
        // godrive/god completes drive letters
        if parent_parts.len() == 1
            && matches!(parent_parts[0].to_lowercase().as_str(), "godrive" | "god")
        {
            out.extend(filter_prefix(
                crate::navigator::Navigator::list_drives()
                    .into_iter()
                    .map(|d| d.to_string())
                    .collect(),
                last_frag,
            ));
        }
        out.extend(filter_prefix(
            subcommands_for(parent_parts)
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            last_frag,
        ));
        out.extend(filter_prefix(
            ral_hint_candidates_for(parent_parts, last_frag),
            last_frag,
        ));
        // `ignore`/`care` value slots complete the names they store: dirs,
        // files or the extensions present in the working directory.
        if let Some(cands) = igcare_value_candidates_for(parent_parts, last_frag) {
            return dedup_sorted(cands);
        }

        // Alias invocations with `#param --hint-*` annotations complete
        // the hinted slot; unhinted slots (or non-aliases) keep the
        // default filesystem behavior.
        if let Some(cands) = alias_arg_candidates_for(parent_parts, last_frag, trailing_space) {
            out.extend(filter_prefix(cands, last_frag));
        } else {
            out.extend(fs_candidates_for(parent_parts, last_frag));
        }

        dedup_sorted(out)
    }
}

/// Tab key handler: advances the ghost-text cycle index and repaints the line
/// so the hinter shows the next candidate. Never modifies the buffer itself,
/// so Tab stays on the same line; Right-arrow accepts the ghost.
pub(crate) struct TabCycleHandler {
    cycle: Arc<Mutex<CycleState>>,
    delta: isize,
}

impl ConditionalEventHandler for TabCycleHandler {
    fn handle(
        &self,
        _evt: &Event,
        _n: RepeatCount,
        _positive: bool,
        ctx: &EventContext,
    ) -> Option<Cmd> {
        let line = ctx.line();
        let pos = ctx.pos();
        if line.is_empty() {
            return Some(Cmd::Noop);
        }
        if let Ok(mut c) = self.cycle.lock() {
            if c.line != line || c.pos != pos {
                c.line = line.to_string();
                c.pos = pos;
                c.index = 0;
            } else {
                c.index = c.index.wrapping_add(self.delta);
            }
        }
        Some(Cmd::Repaint)
    }
}

/// Keep candidates that start with `frag` (case-insensitive) and are longer
/// than what was typed. `names` are full candidate words.
pub(crate) fn filter_prefix(names: Vec<String>, frag: &str) -> Vec<String> {
    if frag.is_empty() {
        let mut out = names;
        out.retain(|n| !n.is_empty());
        return out;
    }
    let lower = frag.to_lowercase();
    names
        .into_iter()
        .filter(|n| n.to_lowercase().starts_with(&lower) && n.len() > frag.len())
        .collect()
}

pub(crate) fn dedup_sorted(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v.dedup();
    v
}

/// True when a single token is a contextual-help trigger (`--help` / `-h`).
pub(crate) fn is_help_token(tok: &str) -> bool {
    matches!(tok.to_lowercase().as_str(), "--help" | "-h")
}

/// If `input` ends with a help token as its last whitespace-separated word,
/// return the prefix before it (trimmed). Otherwise return None.
///
/// Examples: `cd --help` -> `Some("cd")`, `ignore dir --help` -> `Some("ignore dir")`,
/// `ignore dir foo --help` -> `Some("ignore dir foo")`, `cd` -> None.
pub(crate) fn strip_help_suffix(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }
    let last = parts.last().copied().unwrap_or("");
    if !is_help_token(last) {
        return None;
    }
    parts.pop();
    Some(parts.join(" "))
}

/// Theme save-point names for `theme <sub> <name>` (info/edit/rm/rnm/export).
pub(crate) fn theme_names_for_command(parts: &[&str]) -> Vec<String> {
    if parts.len() < 2 {
        return vec![];
    }
    if parts[0].to_lowercase() != "theme" {
        return vec![];
    }
    match parts[1].to_lowercase().as_str() {
        "info" | "edit" | "rm" | "remove" | "rnm" | "rename" | "export" => {
            crate::utils::theme::ThemeManager::list_themes()
        }
        _ => vec![],
    }
}

/// Archive extensions completed for `arc list` / `arc extract`.
const ARCHIVE_EXTS: &[&str] = &[".zip", ".tar.gz", ".tgz", ".tar"];

/// Sheet extensions completed for `nsheet open|view <file>` (bare `nsheet <file>` too).
const SHEET_EXTS: &[&str] = &[".csv", ".tsv", ".json", ".jsonl", ".ndjson", ".txt"];

/// Extensions completed for `fmt <file>`'s source-file slot. `pdf` and
/// `xlsx` are output-only, so they are not offered as inputs.
const FMT_SOURCE_EXTS: &[&str] = &[
    ".json",
    ".yml",
    ".yaml",
    ".toml",
    ".md",
    ".markdown",
    ".html",
    ".htm",
    ".xml",
    ".docx",
    ".txt",
    ".csv",
    ".tsv",
];

/// Extension palette offered by the `-o`/`--output` filename slot.
const FMT_EXTS: &[&str] = &[
    ".json",
    ".yml",
    ".yaml",
    ".toml",
    ".md",
    ".markdown",
    ".html",
    ".htm",
    ".xml",
    ".docx",
    ".txt",
    ".pdf",
    ".csv",
    ".tsv",
    ".xlsx",
];

/// Image extensions completed for `image|img <file>` and
/// `image|img display|show <file>`. Directories always pass so
/// `dir/<Tab>` traversal keeps working (handled by the `ext_filter`
/// logic in `fs_candidates_for`); non-image files are suppressed.
const IMAGE_EXTS: &[&str] = &[
    ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".ico", ".tiff", ".tif", ".webp",
];

/// Audio + video extensions completed for `nms play <file>`.
///
/// `.ts` is deliberately omitted (it clashes with TypeScript); `.m2ts` stays.
const MEDIA_EXTS: &[&str] = &[
    ".mp3", ".wav", ".wave", ".flac", ".m4a", ".aac", ".ogg", ".oga", ".opus", ".wma", ".aiff",
    ".aif", ".aifc", ".mka", ".weba", ".mp4a", ".mp4", ".m4v", ".mkv", ".webm", ".mov", ".avi",
    ".mpg", ".mpeg", ".wmv", ".flv", ".3gp", ".m2ts",
];

/// True when `view`'s optional target-path slot is already filled by a bare
/// token in `args`. Flag tokens, the values after value-taking flags
/// (`-d`/`--depth`, `-p`/`--path`, `--head`/`--tail`/`--sort`) and `-ext`
/// value runs don't count — mirroring `extract_target_spec` in
/// `helpers.rs` plus the ext pre-parse in `report.rs`. `-s`/`-fs`/`-ds`
/// consume the next token only when it parses as a size value.
fn view_path_taken(args: &[&str]) -> bool {
    const VALUE_FLAGS: &[&str] = &[
        "-d", "--depth", "-p", "--path", "--head", "--tail", "--betw", "--sort",
    ];
    const OPTIONAL_SIZE_FLAGS: &[&str] = &["-s", "--size", "-fs", "--filesize", "-ds", "--dirsize"];
    const MULTI_FLAGS: &[&str] = &["-ext", "--ext", "-e"];
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let mut i = 0;
    while i < args.len() {
        let lower = args[i].to_lowercase();
        if VALUE_FLAGS.contains(&lower.as_str()) {
            i += 2; // skip the flag and its value
            continue;
        }
        if OPTIONAL_SIZE_FLAGS.contains(&lower.as_str())
            && i + 1 < args.len()
            && crate::explorer::parse_size_value(args[i + 1]).is_some()
        {
            i += 2; // `-s 10MB`: value consumed, not a path
            continue;
        }
        if MULTI_FLAGS.contains(&lower.as_str()) {
            i += 1;
            while i < args.len() && !crate::filetype::is_ext_value_stop(args[i], &cwd) {
                i += 1;
            }
            continue;
        }
        let tok = args[i];
        if tok.starts_with('-') || tok.is_empty() {
            i += 1;
            continue;
        }
        return true; // bare path token (including `@tp/sub`)
    }
    false
}

/// True when `qur` is currently in a file-glob slot (files should complete).
/// `parent_parts` excludes the fragment being typed. Globs come before
/// content flags: once `-s`/`-rp`/`-dp` appears, later slots are flags/values,
/// not more globs. A trailing value-taking flag means its value is expected.
fn qur_glob_slot(parent_parts: &[&str]) -> bool {
    const VALUE_FLAGS: &[&str] = &[
        "-s",
        "--select",
        "-rp",
        "--replace",
        "-d",
        "--depth",
        "-l",
        "--limit",
    ];
    const CONTENT_FLAGS: &[&str] = &["-s", "--select", "-rp", "--replace", "-dp", "--display"];
    if parent_parts.is_empty() {
        return false;
    }
    // Trailing value flag → its value is being typed, not a glob.
    if let Some(last) = parent_parts.last().map(|s| s.to_lowercase()) {
        if VALUE_FLAGS.contains(&last.as_str()) || last.starts_with("--limit=") {
            return false;
        }
    }
    // Any content flag already present (with or without value yet) →
    // glob phase is over; next slots are flags/values.
    for tok in &parent_parts[1..] {
        let lower = tok.to_lowercase();
        if CONTENT_FLAGS.contains(&lower.as_str()) {
            return false;
        }
        // Skip the value right after a value flag.
        let _ = lower;
    }
    // Skip values after value flags when scanning for stray flags:
    // `qur *.rs -d 3` still has no content flag → glob slot stays open
    // for extra globs (`qur *.rs *.md -d 3`), which parse_qur_args allows.
    let mut i = 1;
    while i < parent_parts.len() {
        let lower = parent_parts[i].to_lowercase();
        if VALUE_FLAGS.contains(&lower.as_str()) {
            i += 2;
            continue;
        }
        if lower.starts_with("--limit=") || lower == "-dp" || lower == "--display" {
            return false;
        }
        i += 1;
    }
    true
}

/// True when the next word after `parts` (fragment excluded) is an extension
/// value: the last parent token is an ext flag (`-ext`/`--ext`), an ext
/// subcommand (`scan`/`tl` + `ext`), or a bare ext already following one —
/// with no intervening flag or path.
fn in_ext_slot(parts: &[&str]) -> bool {
    if parts.len() < 2 {
        return false;
    }
    let root = parts[0].to_lowercase();
    let mut i = parts.len() - 1;
    loop {
        let l = parts[i].to_lowercase();
        if l == "-ext" || l == "--ext" || l == "-e" {
            return matches!(root.as_str(), "view" | "viewf");
        }
        if l == "ext" && i == 1 && (root == "scan" || root == "tl") {
            return true;
        }
        if l.starts_with('-') || l.starts_with('@') {
            return false;
        }
        if l.contains('/') || l.contains('\\') {
            return false;
        }
        if i <= 1 {
            return false;
        }
        i -= 1;
    }
}

/// Extension-name candidates for `scan ext`, `tl ext` and `view -ext`.
/// `frag` is the word being typed; comma-aware (`rs,c` completes after the
/// comma and returns full `rs,…` replacements). Empty frag → no candidates
/// (flags/subcommands already cover the bare slot).
pub(crate) fn ext_names_for_command(parts: &[&str], frag: &str) -> Vec<String> {
    if frag.is_empty() || !in_ext_slot(parts) {
        return vec![];
    }
    let (prefix, stub) = match frag.rfind(',') {
        Some(idx) => (&frag[..idx + 1], &frag[idx + 1..]),
        None => ("", frag),
    };
    let lower = stub.to_lowercase();
    crate::filetype::all_known_extensions()
        .into_iter()
        .filter(|e| e.starts_with(&lower) && e.len() > stub.len())
        .map(|e| format!("{}{}", prefix, e))
        .collect()
}

/// Value hints for `ignore|care dir|file <name>` and `ignore|care ext <ext>`.
///
/// Names are listed bare — never with a trailing `/`, since the config stores
/// directory and file names verbatim and a `src/` entry would never match —
/// and a comma inside one token completes its next element
/// (`ignore dir src,do<Tab>` → `src,docs`). `ext`
/// offers the extensions of the files in the working directory (the useful
/// palette when choosing what to ignore), lowercased like the stored value.
///
/// `ignore` only offers items that are not ignored yet; `care` only offers
/// items that already are (the effective global + local ignore sets), so the
/// two commands never hint a no-op. `None` when the line is not an
/// ignore/care value slot.
fn igcare_value_candidates_for(parts: &[&str], frag: &str) -> Option<Vec<String>> {
    if parts.len() < 2 || !matches!(parts[0].to_lowercase().as_str(), "ignore" | "care") {
        return None;
    }
    let ignoring = parts[0].eq_ignore_ascii_case("ignore");
    let kind = parts[1].to_lowercase();
    if !matches!(kind.as_str(), "dir" | "file" | "ext") {
        return None;
    }
    // Comma-separated lists: keep everything up to the last comma as prefix.
    let (prefix, stub) = match frag.rfind(',') {
        Some(idx) => (&frag[..idx + 1], frag[idx + 1..].trim_start()),
        None => ("", frag),
    };
    let lower = stub.to_lowercase();
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let names: Vec<String> = match kind.as_str() {
        "ext" => {
            let ignored = Config::get_ignored_extensions_combined();
            dir_extensions(&cwd, &lower)
                .into_iter()
                .filter(|e| ignored.contains(e) != ignoring)
                .collect()
        }
        "dir" => {
            let ignored = Config::get_ignored_dirs_combined();
            cwd_dirs()
                .into_iter()
                .filter(|d| ignored.contains(&d.to_lowercase()) != ignoring)
                .collect()
        }
        _ => {
            let ignored = Config::get_ignored_files_combined();
            cwd_files(|_| true)
                .into_iter()
                .filter(|f| ignored.contains(f) != ignoring)
                .collect()
        }
    };
    Some(
        names
            .into_iter()
            .filter(|n| {
                let l = n.to_lowercase();
                l.starts_with(&lower) && l.len() > stub.len()
            })
            .map(|n| format!("{}{}", prefix, n))
            .collect(),
    )
}

/// Output-filename candidates for `fmt … -o <name>` / `fmt … --output <name>`
/// (the word slot immediately after the flag). Returns full last-word
/// replacements that keep the typed basename and only grow the extension:
/// `o.j` → `o.json …`; a dotless stub `j` → `j.json …`; a bare `-o <Tab>`
/// suggests a default name, `out.json …`.
pub(crate) fn fmt_output_candidates_for(parts: &[&str], frag: &str) -> Vec<String> {
    let in_output_slot = parts.len() >= 2
        && parts[0].to_lowercase() == "fmt"
        && matches!(
            parts[parts.len() - 1].to_lowercase().as_str(),
            "-o" | "--output"
        );
    if !in_output_slot {
        return vec![];
    }
    // Everything up to the last dot stays as the basename; the stub after
    // it filters the extension palette.
    let (stem, partial) = match frag.rfind('.') {
        Some(idx) => (frag[..idx].to_string(), frag[idx + 1..].to_lowercase()),
        None => (
            if frag.is_empty() { "out" } else { frag }.to_string(),
            String::new(),
        ),
    };
    FMT_EXTS
        .iter()
        .map(|e| e.trim_start_matches('.'))
        .filter(|e| e.starts_with(&partial))
        .map(|e| format!("{}.{}", stem, e))
        .collect()
}

/// Filesystem candidates for commands that take paths.
/// Returns full last-word replacements (e.g. `src/main`, `docs/`).
pub(crate) fn fs_candidates_for(parent_parts: &[&str], last_frag: &str) -> Vec<String> {
    if parent_parts.is_empty() {
        return vec![];
    }
    let root = parent_parts[0].to_lowercase();

    // (dirs_only, files_only, extension filter, readable_only)
    // - dirs_only: directories only, files never pass (`cd`, `ds`).
    // - files_only: files only, directories never pass (`fs`, `mkf`).
    // - ext_filter: files must match one of these suffixes; directories
    //   always pass so `dir/<Tab>` traversal keeps working.
    // - readable_only: files must not be known binaries/archives
    //   (`txtc`/`txtf`); directories pass for traversal.
    let (dirs_only, files_only, ext_filter, readable_only): (bool, bool, Option<&[&str]>, bool) =
        match root.as_str() {
            "cd" => {
                // `cd to <name>` is a teleport jump — no filesystem
                if parent_parts.len() >= 2 && parent_parts[1].to_lowercase() == "to" {
                    return vec![];
                }
                if parent_parts.len() > 1 {
                    return vec![];
                }
                (true, false, None, false)
            }
            // `tl file <path>` / `tl dir <path>` take paths; `tl ext` takes an extension
            "tl" => {
                if in_ext_slot(parent_parts) {
                    // Extension slot — ext names complete, not files.
                    return vec![];
                }
                if parent_parts.len() != 2
                    || !matches!(parent_parts[1].to_lowercase().as_str(), "file" | "dir")
                {
                    return vec![];
                }
                (false, false, None, false)
            }
            // `mkd`/`rmd` take comma-separated dir lists (`mkd a, b`) —
            // every slot completes dirs.
            "mkd" | "rmd" => (true, false, None, false),
            "rnmd" => {
                if parent_parts.len() > 1 {
                    return vec![];
                }
                (true, false, None, false)
            }
            // `ds <pattern>` searches directories — dirs only.
            "ds" => {
                if parent_parts.len() > 1 {
                    return vec![];
                }
                (true, false, None, false)
            }
            // `view` takes flags plus one optional target path in any order
            // (`view -ds <path>`, `view <path> --dirsize`, …).
            "view" => {
                if in_ext_slot(parent_parts) {
                    // Extension slot — ext names complete, not files.
                    return vec![];
                }
                if view_path_taken(&parent_parts[1..]) {
                    return vec![];
                }
                let file = parent_parts[1..]
                    .iter()
                    .any(|t| matches!(t.to_lowercase().as_str(), "-f" | "--file"));
                let dir = parent_parts[1..]
                    .iter()
                    .any(|t| matches!(t.to_lowercase().as_str(), "-dir" | "--directory"));
                match (file, dir) {
                    (true, false) => (false, true, None, false),
                    (false, true) => (true, false, None, false),
                    _ => (false, false, None, false),
                }
            }
            // `viewf` takes a file or directory target (fixed depth 1).
            "viewf" => {
                if in_ext_slot(parent_parts) || view_path_taken(&parent_parts[1..]) {
                    return vec![];
                }
                (false, false, None, false)
            }
            // `viewd` takes a directory target.
            "viewd" => {
                if view_path_taken(&parent_parts[1..]) {
                    return vec![];
                }
                (true, false, None, false)
            }
            // `views` takes one directory target after its flags.
            "views" => {
                if view_path_taken(&parent_parts[1..]) {
                    return vec![];
                }
                let awaiting = parent_parts
                    .last()
                    .map(|t| matches!(t.to_lowercase().as_str(), "--sort" | "-s" | "--size"))
                    .unwrap_or(false)
                    && parent_parts.len() > 1;
                if awaiting {
                    return vec![];
                }
                (true, false, None, false)
            }
            // `txtc`/`txtf <file>` display readable files: suppress known
            // binaries/archives (`.exe`, `.zip`, …); dirs pass for traversal.
            "txtc" => {
                if parent_parts.len() > 1 {
                    return vec![];
                }
                (false, false, None, true)
            }
            // `txtf [--head N] [--tail N] [--betw A:B] [file]`: flags in
            // any order; no file completion while a flag awaits its value
            // or once the file slot is filled.
            "txtf" => {
                if let Some(last) = parent_parts.last().map(|s| s.to_lowercase()) {
                    if last == "--head" || last == "--tail" || last == "--betw" {
                        return vec![];
                    }
                }
                let mut i = 1;
                let mut bare = 0;
                while i < parent_parts.len() {
                    let lower = parent_parts[i].to_lowercase();
                    if lower == "--head" || lower == "--tail" || lower == "--betw" {
                        i += 2; // skip the flag and its value
                        continue;
                    }
                    if !parent_parts[i].starts_with('-') {
                        bare += 1;
                    }
                    i += 1;
                }
                if bare > 0 {
                    return vec![];
                }
                (false, false, None, true)
            }
            // `head`/`tail`/`betw` take an optional count/range then an
            // optional file; offer files until that file slot is filled.
            "head" | "tail" | "betw" => {
                let mut bare = 0;
                for tok in parent_parts.iter().skip(1) {
                    if !tok.starts_with('-') {
                        bare += 1;
                    }
                }
                if bare >= 2 {
                    return vec![];
                }
                (false, false, None, true)
            }
            // `sort`/`rev` take an optional single file; offer files until it
            // is given (flags are skipped).
            "sort" | "rev" => {
                let mut bare = 0;
                for tok in parent_parts.iter().skip(1) {
                    if !tok.starts_with('-') {
                        bare += 1;
                    }
                }
                if bare >= 1 {
                    return vec![];
                }
                (false, false, None, true)
            }
            // Pattern-search commands take a pattern, not a path, but
            // filename completion helps compose it — files only.
            "fs" | "fsc" | "fgo" | "locate" | "gs" => {
                if parent_parts.len() > 1 {
                    return vec![];
                }
                (false, true, None, false)
            }
            // `qur <glob> [-s <pat>] [-rp <new>] [-dp] [-d <n>] [-l <n>]`:
            // files only in the glob slot — never for `-s`/`-rp` values
            // (content patterns) nor `-d`/`-l` values (numbers), and once a
            // content flag is present the next slot is flags, not more globs.
            "qur" => {
                if !qur_glob_slot(parent_parts) {
                    return vec![];
                }
                (false, true, None, false)
            }
            // `mkf`/`rmf` take comma-separated file lists (`rmf a, b`) —
            // every slot completes files.
            "mkf" | "rmf" => (false, true, None, false),
            // `rnmf <old> <new>`: only the existing-file slot completes
            // (the new name must not exist).
            "rnmf" => {
                if parent_parts.len() > 1 {
                    return vec![];
                }
                (false, true, None, false)
            }
            // `arc create <out> <inputs…>` / `arc list|extract <archive> [dest]`
            "arc" => {
                if parent_parts.len() < 2 {
                    return vec![];
                }
                match parent_parts[1].to_lowercase().as_str() {
                    // Output slot is a free-form archive name — no hint.
                    // Deeper positions are input files/dirs.
                    "create" | "c" | "mk" => {
                        if parent_parts.len() == 2 {
                            return vec![];
                        }
                        (false, false, None, false)
                    }
                    "list" | "ls" | "l" => {
                        if parent_parts.len() == 2 {
                            (false, false, Some(ARCHIVE_EXTS), false)
                        } else {
                            return vec![];
                        }
                    }
                    "extract" | "x" | "unzip" => {
                        if parent_parts.len() == 2 {
                            (false, false, Some(ARCHIVE_EXTS), false)
                        } else if parent_parts.len() == 3 {
                            // destination directory
                            (true, false, None, false)
                        } else {
                            return vec![];
                        }
                    }
                    _ => return vec![],
                }
            }
            // `nssh cp <local|name:remote>`: complete local paths only when the
            // fragment is not a remote `name:…` spec.
            "nssh" => {
                if parent_parts.len() == 2
                    && matches!(parent_parts[1].to_lowercase().as_str(), "cp")
                    && !last_frag.contains(':')
                {
                    (false, false, None, false)
                } else {
                    return vec![];
                }
            }
            // `nsheet open|view <file>` and bare `nsheet <file>` take sheet files.
            // Later positions are flags/values — no file completion there.
            "nsheet" => {
                if parent_parts.len() == 1 {
                    (false, false, Some(SHEET_EXTS), false)
                } else if parent_parts.len() == 2
                    && matches!(parent_parts[1].to_lowercase().as_str(), "open" | "view")
                {
                    (false, false, Some(SHEET_EXTS), false)
                } else {
                    return vec![];
                }
            }
            // `nms play <file>` completes audio/video media files.
            "nms" => {
                if parent_parts.len() == 2 && parent_parts[1].to_lowercase() == "play" {
                    (false, false, Some(MEDIA_EXTS), false)
                } else {
                    return vec![];
                }
            }
            // `image <TAB>` and `image display|show <TAB>` complete
            // image files + dirs; deeper positions (flags/values)
            // complete nothing.
            "image" | "img"
                if parent_parts.len() == 1
                    || (parent_parts.len() == 2
                        && matches!(
                            parent_parts[1].to_lowercase().as_str(),
                            "display" | "show"
                        )) =>
            {
                (false, false, Some(IMAGE_EXTS), false)
            }
            "image" | "img" => {
                return vec![];
            }
            // `fmt <TAB>` completes the source-file slot with the
            // supported formats; deeper positions (keyword/flag slots)
            // are handled by `subcommands_for` / the `-o` slot helper.
            "fmt" => {
                if parent_parts.len() == 1 {
                    (false, false, Some(FMT_SOURCE_EXTS), false)
                } else {
                    return vec![];
                }
            }
            // General path-takers accept files and dirs. For `cp`/`mv` only
            // the src slot completes (dst is a free-form name/dir) and
            // `-f`/`--force` flags never consume it.
            "mv" | "move" | "cp" => {
                let operands = parent_parts[1..]
                    .iter()
                    .filter(|t| !t.starts_with('-'))
                    .count();
                if operands > 0 {
                    return vec![];
                }
                (false, false, None, false)
            }
            "size" | "report" | "explorer" | "nfe" | "ne" | "ntceditor" => {
                if parent_parts.len() > 1 {
                    return vec![];
                }
                (false, false, None, false)
            }
            _ => return vec![],
        };

    // Split `dir/prefix` so `cd src/ma` lists inside `src/`
    let (dir_part, file_part) = match last_frag.rfind('/') {
        Some(idx) => (&last_frag[..idx], &last_frag[idx + 1..]),
        None => ("", last_frag),
    };
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => return vec![],
    };
    let base = if dir_part.is_empty() {
        cwd
    } else {
        let p = cwd.join(dir_part);
        if !p.is_dir() {
            return vec![];
        }
        p
    };

    // `rmf *.<Tab>`: complete `*.<ext>` delete patterns from the file
    // extensions present in the listed directory (`*.log`, `sub/*.tmp`).
    // `cp`/`mv` pattern sources (`cp *.py <Tab>`) behave the same, and a
    // bare `*` also offers `*` itself so Tab cycles `*` → `*.py` → ….
    if matches!(root.as_str(), "rmf" | "mv" | "move" | "cp")
        && (file_part == "*" || file_part.starts_with("*."))
    {
        return star_pattern_candidates(&base, dir_part, file_part);
    }

    list_path_candidates(
        &base,
        dir_part,
        file_part,
        dirs_only,
        files_only,
        ext_filter,
        readable_only,
    )
}

/// Shared directory listing for command slots and `>>`/`<` redirect
/// targets. `base` must be an existing directory; names are filtered by
/// `file_part`, the per-command options, and (when `readable_only`) hidden
/// when they are binaries, images, audio/video or archives.
fn list_path_candidates(
    base: &std::path::Path,
    dir_part: &str,
    file_part: &str,
    dirs_only: bool,
    files_only: bool,
    ext_filter: Option<&[&str]>,
    readable_only: bool,
) -> Vec<String> {
    let lower = file_part.to_lowercase();
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(base) {
        for entry in entries.flatten() {
            let name = match entry.file_name().into_string() {
                Ok(n) => n,
                Err(_) => continue,
            };
            if name.starts_with('.') && !lower.starts_with('.') {
                continue;
            }
            if !name.to_lowercase().starts_with(&lower) {
                continue;
            }
            if name.len() <= file_part.len() && !file_part.is_empty() {
                continue;
            }
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            if dirs_only && !is_dir {
                continue;
            }
            if files_only && is_dir {
                continue;
            }
            if let Some(exts) = ext_filter {
                if !is_dir && !exts.iter().any(|e| name.to_lowercase().ends_with(e)) {
                    continue;
                }
            }
            if readable_only && !is_dir && crate::filetype::is_finder_hidden(&entry.path()) {
                continue;
            }
            let display = if dir_part.is_empty() {
                name
            } else {
                format!("{}/{}", dir_part, name)
            };
            if is_dir {
                out.push(format!("{}/", display));
            } else {
                out.push(display);
            }
        }
    }
    out
}

/// Commands offered at the start of a piped stage (`cmd | `), in ghost
/// order. Only these are completed after a pipe — no other command falls
/// back to generic completion.
const PIPE_FILTER_COMMANDS: &[&str] = &[
    "head", "tail", "betw", "gs", "sort", "rev", "count", "trim", "encode", "qur", "clip",
];

/// Pipe-stage command completion. `Some` means the stage is still in its
/// command slot (or names an unsupported command), so the caller must not
/// fall back to generic completion. `None` means one of the pipe filters
/// is already present and its own flags/files should complete normally.
fn pipe_stage_commands(tail: &str) -> Option<Vec<String>> {
    let trimmed = tail.trim();
    if trimmed.is_empty() {
        return Some(PIPE_FILTER_COMMANDS.iter().map(|s| s.to_string()).collect());
    }
    let first = trimmed.split_whitespace().next().unwrap_or("");
    let command_given =
        tail.ends_with(' ') || tail.ends_with('\t') || trimmed.split_whitespace().count() > 1;
    if !command_given {
        // Still typing the command name: prefix-match in declared order.
        return Some(filter_prefix(
            PIPE_FILTER_COMMANDS.iter().map(|s| s.to_string()).collect(),
            first,
        ));
    }
    if PIPE_FILTER_COMMANDS.contains(&first.to_lowercase().as_str()) {
        None
    } else {
        Some(vec![])
    }
}

/// Completion for `>>` (append) and `<` (input) targets. Readable files
/// only — binary, image, audio, video and archive files are hidden — plus
/// directories so `sub/<Tab>` traversal keeps working. Nothing once the
/// single target token is already present.
fn redirect_target_candidates(tail: &str) -> Vec<String> {
    let trimmed = tail.trim();
    let target_given = !trimmed.is_empty()
        && (tail.ends_with(' ') || tail.ends_with('\t') || trimmed.split_whitespace().count() > 1);
    if target_given {
        return vec![];
    }
    let (dir_part, file_part) = match trimmed.rfind('/') {
        Some(idx) => (&trimmed[..idx], &trimmed[idx + 1..]),
        None => ("", trimmed),
    };
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => return vec![],
    };
    let base = if dir_part.is_empty() {
        cwd
    } else {
        let p = cwd.join(dir_part);
        if !p.is_dir() {
            return vec![];
        }
        p
    };
    list_path_candidates(&base, dir_part, file_part, false, false, None, true)
}

/// Completion for the `<<` heredoc delimiter: `EOF` only.
fn heredoc_delim_candidates(tail: &str) -> Vec<String> {
    let trimmed = tail.trim();
    if trimmed.is_empty() {
        return vec!["EOF".to_string()];
    }
    if tail.ends_with(' ') || tail.ends_with('\t') || trimmed.split_whitespace().count() > 1 {
        return vec![];
    }
    filter_prefix(vec!["EOF".to_string()], trimmed)
}

/// `*`-pattern candidates for `rmf`/`cp`/`mv` from the extensions of the
/// files inside `base`. `frag` is the typed last word (`*`, `*.`, `*.py`).
/// Returns full last-word replacements. A bare `*` also offers `*` itself so
/// Tab cycles `*` → `*.py` → …; exact-length matches are excluded so the
/// ghost only ever shows an untyped remainder.
fn star_pattern_candidates(base: &std::path::Path, dir_part: &str, frag: &str) -> Vec<String> {
    let render = |ext: &str| {
        if dir_part.is_empty() {
            format!("*.{}", ext)
        } else {
            format!("{}/*.{}", dir_part, ext)
        }
    };
    let mut out: Vec<String> = Vec::new();
    // Bare `*`: offer `*` plus every `*.<ext>` present.
    if frag == "*" {
        out.push(if dir_part.is_empty() {
            "*".to_string()
        } else {
            format!("{}/*", dir_part)
        });
        for ext in dir_extensions(base, "") {
            out.push(render(&ext));
        }
        return out;
    }
    // `*.` / `*.stub`: only matching extensions.
    let stub = frag.strip_prefix("*.").unwrap_or("").to_lowercase();
    for ext in dir_extensions(base, &stub) {
        out.push(render(&ext));
    }
    out
}

/// Sorted, deduped extensions of the non-hidden files directly inside
/// `base`, filtered by `stub` (empty = all). Extensions are lowercased and
/// exact-stub-length matches are dropped so a fully-typed pattern ghosts
/// nothing.
fn dir_extensions(base: &std::path::Path, stub: &str) -> Vec<String> {
    let mut exts: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(base) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(true) {
                continue;
            }
            let name = match entry.file_name().into_string() {
                Ok(n) => n,
                Err(_) => continue,
            };
            if name.starts_with('.') {
                continue;
            }
            let Some(dot) = name.rfind('.') else {
                continue;
            };
            let ext = name[dot + 1..].to_lowercase();
            if ext.is_empty() || !ext.starts_with(stub) {
                continue;
            }
            // Exact-length matches ghost nothing — skip them.
            if ext.len() <= stub.len() {
                continue;
            }
            if !exts.contains(&ext) {
                exts.push(ext);
            }
        }
    }
    exts.sort();
    exts
}

fn build_command_set() -> HashSet<String> {
    let mut cmds: HashSet<String> = builtin_commands()
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let aliases = Config::get_run_aliases_merged();
    for name in aliases.keys() {
        cmds.insert(name.clone());
    }
    cmds
}

impl Hinter for NtcHinter {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<String> {
        if !Config::global_get_autosuggest_enabled() {
            return None;
        }

        self.ensure_fresh();
        let mut pos = pos.min(line.len());
        while pos > 0 && !line.is_char_boundary(pos) {
            pos -= 1;
        }
        let typed = &line[..pos];

        if typed.is_empty() {
            return None;
        }

        let cands = self.candidates_for(typed);
        if cands.is_empty() {
            return None;
        }

        // Show the currently-cycled candidate as ghost text. Typing resets to
        // the first candidate; Tab advances (see TabCycleHandler).
        let pick: String = {
            let mut cyc = self.cycle.lock().unwrap();
            if cyc.line != typed || cyc.pos != pos {
                cyc.line = typed.to_string();
                cyc.pos = pos;
                cyc.index = 0;
            }
            cands[(cyc.index.rem_euclid(cands.len() as isize)) as usize].clone()
        };

        // The ghost is the remainder after the active word, computed from
        // the current segment so `cmd |` ghosts `" head"` (space included).
        ghost_remainder(typed, &pick)
    }
}

/// Ghost text for `pick`: the part after the word already typed in the
/// active segment. When the cursor sits right after an operator (`cmd |`),
/// the ghost carries the separating space so accepting it produces a valid
/// operator token.
fn ghost_remainder(typed: &str, pick: &str) -> Option<String> {
    let (tail, _) = NtcHinter::stage_tail(typed);
    if tail.is_empty() || tail.ends_with(' ') || tail.ends_with('\t') {
        let sep = if tail.is_empty()
            && !typed.is_empty()
            && !typed.ends_with(' ')
            && !typed.ends_with('\t')
        {
            " "
        } else {
            ""
        };
        let out = format!("{}{}", sep, pick);
        return if out.is_empty() { None } else { Some(out) };
    }
    let frag = tail.split_whitespace().last().unwrap_or("");
    match pick.get(frag.len()..) {
        Some(rest) if !rest.is_empty() => Some(rest.to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_support::TempCwd;

    #[test]
    fn l1_prefix_matches_commands() {
        let h = NtcHinter::new();
        let c = h.candidates_for("t");
        assert!(c.contains(&"tl".to_string()));
        assert!(c.contains(&"theme".to_string()));
        assert!(c.contains(&"todo".to_string()));
        // Exact-length matches are excluded (ghost only shows the remainder).
        assert!(!c.iter().any(|x| x == "t"));
    }

    #[test]
    fn gs_flags_are_suggested() {
        let h = NtcHinter::new();
        let c = h.candidates_for("gs -");
        for flag in ["-p", "--path", "-s", "--sort", "-d", "-l", "--limit", "-i"] {
            assert!(c.contains(&flag.to_string()), "missing {flag}: {c:?}");
        }
    }

    /// `-g` / `--global` is only offered inside a folder that has an
    /// `ntconfig.toml` (there the default target is the local file).
    #[test]
    fn global_scope_flag_is_hinted_only_inside_a_local_dir() {
        {
            let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[], &[]);
            let flags = subcommands_for(&["ral", "add"]);
            assert!(!flags.contains(&"-g"), "{flags:?}");
            assert!(!flags.contains(&"--global"), "{flags:?}");
            let flags = subcommands_for(&["ignore", "dir"]);
            assert!(!flags.contains(&"-g"), "{flags:?}");
        }
        {
            let _cwd =
                crate::utils::test_support::TempCwd::with_fixture(&[("ntconfig.toml", "")], &[]);
            let flags = subcommands_for(&["ral", "add"]);
            assert!(flags.contains(&"-g"), "{flags:?}");
            assert!(flags.contains(&"--global"), "{flags:?}");
            let flags = subcommands_for(&["ral", "rm"]);
            assert!(flags.contains(&"-g"), "{flags:?}");
            let flags = subcommands_for(&["ral", "hintn", "add"]);
            assert!(flags.contains(&"-g"), "{flags:?}");
            let flags = subcommands_for(&["ignore", "dir"]);
            assert!(flags.contains(&"-g"), "{flags:?}");
            let flags = subcommands_for(&["care", "ext"]);
            assert!(flags.contains(&"-g"), "{flags:?}");
        }
    }

    #[test]
    fn tl_cycles_all_subcommands() {
        let h = NtcHinter::new();
        assert_eq!(h.candidates_for("tl "), vec!["dir", "ext", "file"]);
        assert_eq!(h.candidates_for("tl d"), vec!["dir"]);
        assert_eq!(h.candidates_for("tl e"), vec!["ext"]);
    }

    #[test]
    fn view_flag_fragments_ghost_size_flags() {
        let h = NtcHinter::new();
        // `view -d` ghosts `-ds`; the exact `-d` itself is excluded.
        let c = h.candidates_for("view -d");
        assert!(c.contains(&"-ds".to_string()), "got: {:?}", c);
        assert!(!c.iter().any(|x| x == "-d"));
        // `view --d` ghosts `--depth` and `--dirsize`.
        let c = h.candidates_for("view --d");
        assert!(c.contains(&"--depth".to_string()), "got: {:?}", c);
        assert!(c.contains(&"--dirsize".to_string()), "got: {:?}", c);
        // A second flag is still suggested after the first one.
        let c = h.candidates_for("view -ds --");
        assert!(c.contains(&"--depth".to_string()), "got: {:?}", c);
        // Dash-only still offers the universal help flag.
        let c = h.candidates_for("view -");
        assert!(c.contains(&"--help".to_string()), "got: {:?}", c);
        // New display flags ghost too.
        let c = h.candidates_for("view --s");
        assert!(c.contains(&"--sort".to_string()), "got: {:?}", c);
        assert!(c.contains(&"--size".to_string()), "got: {:?}", c);
        let c = h.candidates_for("view --h");
        assert!(c.contains(&"--head".to_string()), "got: {:?}", c);
        // A value-taking flag awaiting its value offers no subcommands.
        assert!(subcommands_for(&["view", "--head"]).is_empty());
        assert!(subcommands_for(&["view", "--sort"]).is_empty());
        assert!(!subcommands_for(&["view", "-s"]).is_empty());
    }

    #[test]
    fn view_path_taken_skips_flags_and_values() {
        // `src` must resolve to a real dir for the `-ext` value-stop check,
        // so run inside an isolated repo-like fixture (also serializes the
        // process-global CWD against parallel tests on CI).
        let _cwd = TempCwd::with_repo_fixture();
        assert!(!view_path_taken(&[]));
        assert!(!view_path_taken(&["-ds"]));
        assert!(!view_path_taken(&["-d", "2"]));
        assert!(!view_path_taken(&["-p", "src"]));
        assert!(!view_path_taken(&["--dirsize", "--depth", "3"]));
        assert!(!view_path_taken(&["-ext", "rs", "c"]));
        assert!(!view_path_taken(&["-ext"]));
        assert!(view_path_taken(&["src"]));
        assert!(view_path_taken(&["-ds", "src"]));
        assert!(view_path_taken(&["src", "-ds"]));
        assert!(view_path_taken(&["-ext", "rs", "src", "-ds"]));
        // Optional size values are skipped; bare size flags are not.
        assert!(!view_path_taken(&["-s", "10MB"]));
        assert!(!view_path_taken(&["--head", "5"]));
        assert!(view_path_taken(&["-s", "src"]));
        assert!(!view_path_taken(&["-s"]));
        assert!(!view_path_taken(&["-s", "10MB", "-d", "2"]));
    }

    #[test]
    fn ext_slot_detection() {
        assert!(in_ext_slot(&["scan", "ext"]));
        assert!(in_ext_slot(&["scan", "ext", "rs"]));
        assert!(in_ext_slot(&["tl", "ext"]));
        assert!(in_ext_slot(&["view", "-ext"]));
        assert!(in_ext_slot(&["view", "src", "--ext", "rs"]));
        assert!(!in_ext_slot(&["scan"]));
        assert!(!in_ext_slot(&["scan", "ext", "rs", "-d"]));
        assert!(!in_ext_slot(&["view", "src"]));
        assert!(!in_ext_slot(&["view", "-ds"]));
        assert!(!in_ext_slot(&["tl", "file"]));
    }

    #[test]
    fn ext_names_complete_and_comma_aware() {
        let h = NtcHinter::new();
        // `scan ext r` suggests `rs` (and friends), not files.
        let c = h.candidates_for("scan ext r");
        assert!(c.contains(&"rs".to_string()), "got: {:?}", c);
        // After a complete ext, the next slot still completes.
        let c = h.candidates_for("scan ext rs c");
        assert!(c.contains(&"cs".to_string()), "got: {:?}", c);
        assert!(c.contains(&"cpp".to_string()), "got: {:?}", c);
        // Comma form completes after the comma with full replacements.
        let c = h.candidates_for("scan ext rs,c");
        assert!(c.contains(&"rs,cs".to_string()), "got: {:?}", c);
        assert!(!c.iter().any(|x| x == "cs"));
        // `view -ext` and the `tl ext` bonus behave the same.
        let c = h.candidates_for("view -ext p");
        assert!(c.contains(&"py".to_string()), "got: {:?}", c);
        let c = h.candidates_for("tl ext j");
        assert!(c.contains(&"js".to_string()), "got: {:?}", c);
        // Bare slots stay quiet (flags/subcommands cover them).
        assert!(ext_names_for_command(&["scan", "ext"], "").is_empty());
    }

    #[test]
    fn mv_cp_tp_slash_lists_savepoint_contents_in_src_only() {
        let _cwd = TempCwd::with_fixture(
            &[("tp1/file.txt", "hello"), ("tp1/sub/deep.txt", "deep")],
            &["tp1", "tp1/sub", "tp2"],
        );
        let root = std::env::current_dir().unwrap();
        let mut cfg = hint_test_config();
        cfg.teleports.insert("tp1".to_string(), root.join("tp1"));
        cfg.teleports.insert("tp2".to_string(), root.join("tp2"));
        Config::with_test_config(cfg, || {
            let h = NtcHinter::new();
            // `@tp/` lists files plain and dirs with trailing `/`.
            let c = h.candidates_for("cp @tp1/");
            assert!(c.contains(&"@tp1/file.txt".to_string()), "got: {:?}", c);
            assert!(c.contains(&"@tp1/sub/".to_string()), "got: {:?}", c);
            assert!(
                !c.contains(&"@tp1/sub/deep.txt".to_string()),
                "got: {:?}",
                c
            );
            // Partial name filters.
            let c = h.candidates_for("cp @tp1/f");
            assert!(c.contains(&"@tp1/file.txt".to_string()), "got: {:?}", c);
            assert!(!c.iter().any(|s| s == "@tp1/sub/"), "got: {:?}", c);
            // Nested traversal.
            let c = h.candidates_for("cp @tp1/sub/d");
            assert!(c.contains(&"@tp1/sub/deep.txt".to_string()), "got: {:?}", c);
            // Flags don't consume the src slot.
            let c = h.candidates_for("cp -f @tp1/");
            assert!(c.contains(&"@tp1/file.txt".to_string()), "got: {:?}", c);
            // Dst slots keep existing behavior (nothing).
            assert!(h.candidates_for("cp file @tp2/").is_empty());
            let c = h.candidates_for("mv @tp1/ ");
            assert!(c.is_empty(), "got: {:?}", c);
            // Unknown savepoints and other commands are untouched.
            assert!(h.candidates_for("cp @ghost/").is_empty());
            assert!(h.candidates_for("size @tp1/").is_empty());
        });
    }

    #[test]
    fn view_and_txtf_tp_subpaths_respect_entry_modes() {
        let _cwd = TempCwd::with_fixture(
            &[("tp1/file.txt", "hello"), ("tp1/sub/deep.txt", "deep")],
            &["tp1", "tp1/sub"],
        );
        let root = std::env::current_dir().unwrap();
        let mut cfg = hint_test_config();
        cfg.teleports.insert("tp1".to_string(), root.join("tp1"));
        Config::with_test_config(cfg, || {
            let h = NtcHinter::new();
            // viewf / txtf offer files (dirs stay traversable).
            for line in ["viewf @tp1/", "txtf @tp1/", "txtf --head 5 @tp1/"] {
                let c = h.candidates_for(line);
                assert!(
                    c.contains(&"@tp1/file.txt".to_string()),
                    "got: {:?} for {}",
                    c,
                    line
                );
            }
            // Directory traversal stays available for txtf's picker mode.
            let c = h.candidates_for("txtf @tp1/");
            assert!(c.contains(&"@tp1/sub/".to_string()), "got: {:?}", c);
            let c = fs_candidates_for(&["txtf"], "tp1/");
            assert!(c.contains(&"tp1/sub/".to_string()), "got: {:?}", c);
            assert!(c.contains(&"tp1/file.txt".to_string()), "got: {:?}", c);
            // viewd hides files but keeps directories.
            let c = h.candidates_for("viewd @tp1/");
            assert!(c.contains(&"@tp1/sub/".to_string()), "got: {:?}", c);
            assert!(!c.contains(&"@tp1/file.txt".to_string()), "got: {:?}", c);
            // Filled slots offer only flags, never more paths.
            for line in ["viewf @tp1/file.txt ", "txtf @tp1/file.txt "] {
                let c = h.candidates_for(line);
                assert!(
                    !c.iter().any(|s| s.starts_with("@tp1/")),
                    "got: {:?} for {}",
                    c,
                    line
                );
            }
        });
    }

    #[test]
    fn view_variants_offer_their_flags() {
        let h = NtcHinter::new();
        for line in ["viewf --", "viewd --"] {
            let c = h.candidates_for(line);
            assert!(
                c.contains(&"--care".to_string()),
                "got: {:?} for {}",
                c,
                line
            );
            assert!(
                c.contains(&"--copy".to_string()),
                "got: {:?} for {}",
                c,
                line
            );
        }
        assert!(h
            .candidates_for("view --")
            .contains(&"--directory".to_string()));
    }

    #[test]
    fn mv_cp_at_savepoints_and_first_slot() {
        let _cwd = TempCwd::with_repo_fixture();
        let mut cfg = hint_test_config();
        cfg.teleports
            .insert("docs".to_string(), std::path::PathBuf::from("C:/docs"));
        cfg.teleports
            .insert("dl".to_string(), std::path::PathBuf::from("C:/dl"));
        Config::with_test_config(cfg, || {
            let h = NtcHinter::new();
            // `@` completes savepoints for mv/cp operands...
            let c = h.candidates_for("cp @");
            assert!(c.contains(&"@docs".to_string()), "got: {:?}", c);
            assert!(c.contains(&"@dl".to_string()), "got: {:?}", c);
            let c = h.candidates_for("mv @do");
            assert!(c.contains(&"@docs".to_string()), "got: {:?}", c);
            assert!(!c.contains(&"@dl".to_string()), "got: {:?}", c);
            // ...and the first plain slot lists files and dirs.
            let c = h.candidates_for("cp ");
            assert!(c.contains(&"notes.txt".to_string()), "got: {:?}", c);
            assert!(c.contains(&"src/".to_string()), "got: {:?}", c);
            let c = h.candidates_for("mv ");
            assert!(c.contains(&"Cargo.toml".to_string()), "got: {:?}", c);
            assert!(c.contains(&"docs/".to_string()), "got: {:?}", c);
        });
    }

    #[test]
    fn scan_files_dir_subcommands_and_flags() {
        let h = NtcHinter::new();
        // New subcommands complete after `scan`.
        let c = h.candidates_for("scan ");
        assert!(c.contains(&"files".to_string()), "got: {:?}", c);
        assert!(c.contains(&"dir".to_string()), "got: {:?}", c);
        assert!(c.contains(&"ext".to_string()), "got: {:?}", c);
        // Same flag set as `scan ext`.
        for cmd in ["scan files --", "scan dir --", "scan ext --"] {
            let c = h.candidates_for(cmd);
            assert!(
                c.contains(&"--care".to_string()),
                "got: {:?} for {}",
                c,
                cmd
            );
            assert!(
                c.contains(&"--depth".to_string()),
                "got: {:?} for {}",
                c,
                cmd
            );
            assert!(
                c.contains(&"--size".to_string()),
                "got: {:?} for {}",
                c,
                cmd
            );
        }
        // Pattern slots do NOT get ext-name ghosts.
        assert!(!in_ext_slot(&["scan", "files"]));
        assert!(!in_ext_slot(&["scan", "dir"]));
        assert!(ext_names_for_command(&["scan", "files"], "r").is_empty());
    }

    #[test]
    fn view_after_flags_still_completes_paths() {
        // Path slot still completes after flags (fixture dir is non-empty).
        let _cwd = TempCwd::with_repo_fixture();
        assert!(!fs_candidates_for(&["view", "-ds"], "").is_empty());
        assert!(!fs_candidates_for(&["view", "-d", "2", "--dirsize"], "").is_empty());
        // Once the path slot is filled, no more path hints.
        assert!(fs_candidates_for(&["view", "-ds", "src"], "").is_empty());
    }

    #[test]
    fn arc_output_slot_offers_no_hint() {
        let h = NtcHinter::new();
        assert!(h.candidates_for("arc create ").is_empty());
        assert!(fs_candidates_for(&["arc", "create"], "").is_empty());
    }

    #[test]
    fn arc_list_extract_filter_to_archives() {
        for parent in [&["arc", "list"][..], &["arc", "extract"][..]] {
            for c in fs_candidates_for(parent, "") {
                if c.ends_with('/') {
                    continue; // directories pass through for traversal
                }
                let l = c.to_lowercase();
                assert!(
                    l.ends_with(".zip")
                        || l.ends_with(".tar.gz")
                        || l.ends_with(".tgz")
                        || l.ends_with(".tar"),
                    "non-archive offered: {}",
                    c
                );
            }
        }
        // Destination slot exists and only completes directories.
        for c in fs_candidates_for(&["arc", "extract", "some.zip"], "") {
            assert!(c.ends_with('/'), "non-dir dest offered: {}", c);
        }
    }

    #[test]
    fn cd_to_suggests_teleports_not_dirs() {
        let h = NtcHinter::new();
        // `cd to ` is a teleport jump — filesystem entries end with '/'.
        for c in h.candidates_for("cd to ") {
            assert!(!c.ends_with('/'), "unexpected fs entry: {}", c);
        }
    }

    #[test]
    fn cd_bare_lists_dirs_only() {
        let h = NtcHinter::new();
        // Bare `cd` completes subdirectories only — every candidate ends
        // with '/' and no bookmark names appear.
        for c in h.candidates_for("cd ") {
            assert!(c.ends_with('/'), "unexpected non-dir entry: {}", c);
        }
    }

    #[test]
    fn cd_at_names_are_not_completed() {
        let h = NtcHinter::new();
        // `cd @name` is not a teleport jump, so no @ candidates after `cd`.
        assert!(h.candidates_for("cd @").is_empty());
        assert!(h.candidates_for("cd @work").is_empty());
    }

    #[test]
    fn ds_lists_dirs_only() {
        // Every candidate ends with `/` (fixture always has `src/`).
        let _cwd = TempCwd::with_repo_fixture();
        let c = fs_candidates_for(&["ds"], "");
        assert!(c.iter().any(|x| x == "src/"), "got: {:?}", c);
        assert!(
            c.iter().all(|x| x.ends_with('/')),
            "non-dir offered: {:?}",
            c
        );
    }

    #[test]
    fn fs_and_mkf_list_files_only() {
        // No directory entries at all; regular files show up.
        let _cwd = TempCwd::with_repo_fixture();
        for parent in [&["fs"][..], &["fsc"][..], &["mkf"][..], &["rmf"][..]] {
            let c = fs_candidates_for(parent, "");
            assert!(
                !c.iter().any(|x| x.ends_with('/')),
                "dir offered for {:?}: {:?}",
                parent,
                c
            );
        }
        let c = fs_candidates_for(&["fs"], "");
        assert!(
            c.iter().any(|x| x == "Cargo.toml"),
            "expected files, got: {:?}",
            c
        );
    }

    #[test]
    fn txtc_suppresses_binaries_but_keeps_dirs() {
        // Dirs pass for `dir/<Tab>` traversal; known binaries never show.
        let _cwd = TempCwd::with_repo_fixture();
        let c = fs_candidates_for(&["txtc"], "");
        assert!(c.iter().any(|x| x == "src/"), "got: {:?}", c);
        for x in &c {
            if x.ends_with('/') {
                continue;
            }
            assert!(
                !crate::filetype::is_finder_hidden(std::path::Path::new(x)),
                "binary offered: {}",
                x
            );
        }
        // Same for the `txtf` twin.
        let c = fs_candidates_for(&["txtf"], "");
        assert!(c.iter().any(|x| x == "src/"), "got: {:?}", c);
    }

    #[test]
    fn non_image_anchors_survive_own_filters() {
        // Guards the earlier regression where every command inherited the
        // image extension filter: non-image anchors must show up.
        let _cwd = TempCwd::with_repo_fixture();
        assert!(fs_candidates_for(&["fs"], "").contains(&"Cargo.toml".to_string()));
        assert!(fs_candidates_for(&["txtc"], "").contains(&"Cargo.toml".to_string()));
        assert!(fs_candidates_for(&["mkf"], "").contains(&"Cargo.toml".to_string()));
        // …while the image slot itself still filters to images + dirs.
        for x in fs_candidates_for(&["image"], "") {
            if x.ends_with('/') {
                continue;
            }
            let l = x.to_lowercase();
            assert!(
                IMAGE_EXTS.iter().any(|e| l.ends_with(e)),
                "non-image offered: {}",
                x
            );
        }
    }

    #[test]
    fn fileops_complete_every_operand_slot() {
        // `rmf`/`mkf` take comma-separated file lists — slots past the
        // first complete files too (`rmf a.txt, b<Tab>`).
        let _cwd = TempCwd::with_repo_fixture();
        let c = fs_candidates_for(&["rmf", "a.txt,"], "");
        assert!(!c.iter().any(|x| x.ends_with('/')), "dir offered: {:?}", c);
        let c = fs_candidates_for(&["mkf", "a.txt", "b.txt,"], "");
        assert!(!c.iter().any(|x| x.ends_with('/')), "dir offered: {:?}", c);
        // `mkd`/`rmd` take comma-separated dir lists — dirs at any depth.
        let c = fs_candidates_for(&["rmd", "old,"], "");
        assert!(c.iter().any(|x| x == "src/"), "got: {:?}", c);
        assert!(c.iter().all(|x| x.ends_with('/')), "got: {:?}", c);
    }

    #[test]
    fn igcare_values_complete_bare_names() {
        // `ignore`/`care` store plain names, so no `src/` trailing slash.
        let _cwd = TempCwd::with_fixture(
            &[
                ("Cargo.toml", "[package]\nname = \"fixture\"\n"),
                ("app.toml", "x = 1\n"),
                ("notes.txt", "hello\n"),
                ("photo.png", "fakepng"),
                ("src/main.rs", "fn main() {}\n"),
            ],
            &["src", "docs", "target"],
        );
        let mut cfg = Config::default();
        cfg.ignored_extensions.insert("toml".into());
        cfg.ignored_files.insert("notes.txt".into());
        Config::with_test_config(cfg, || {
            let h = NtcHinter::new();
            let c = h.candidates_for("ignore dir s");
            assert!(c.contains(&"src".to_string()), "got: {:?}", c);
            assert!(!c.iter().any(|x| x == "src/"), "got: {:?}", c);
            // Bare slots list everything that is not ignored yet.
            let c = h.candidates_for("ignore dir ");
            assert!(c.contains(&"docs".to_string()), "got: {:?}", c);
            assert!(!c.contains(&"target".to_string()), "got: {:?}", c);
            // `care` offers only the already-ignored items.
            let c = h.candidates_for("care dir ");
            assert!(c.contains(&"target".to_string()), "got: {:?}", c);
            assert!(!c.contains(&"src".to_string()), "got: {:?}", c);
            // Files and extensions follow the same split.
            let c = h.candidates_for("ignore file ");
            assert!(c.contains(&"Cargo.toml".to_string()), "got: {:?}", c);
            assert!(!c.contains(&"notes.txt".to_string()), "got: {:?}", c);
            let c = h.candidates_for("care file no");
            assert!(c.contains(&"notes.txt".to_string()), "got: {:?}", c);
            let c = h.candidates_for("ignore ext t");
            assert!(c.contains(&"txt".to_string()), "got: {:?}", c);
            assert!(!c.contains(&"toml".to_string()), "got: {:?}", c);
            let c = h.candidates_for("care ext t");
            assert!(c.contains(&"toml".to_string()), "got: {:?}", c);
            assert!(!c.contains(&"txt".to_string()), "got: {:?}", c);
            // Comma-separated slots keep the typed prefix.
            let c = h.candidates_for("ignore dir src,d");
            assert!(c.contains(&"src,docs".to_string()), "got: {:?}", c);
            // Non-value slots stay untouched.
            assert!(igcare_value_candidates_for(&["ignore"], "").is_none());
            assert!(igcare_value_candidates_for(&["ignore", "foo"], "").is_none());
            assert!(igcare_value_candidates_for(&["fs"], "").is_none());
        });
    }

    #[test]
    fn fileops_tp_subpaths_respect_entry_modes() {
        let _cwd = TempCwd::with_fixture(
            &[("tp1/file.txt", "hello"), ("tp1/sub/deep.txt", "deep")],
            &["tp1", "tp1/sub"],
        );
        let root = std::env::current_dir().unwrap();
        let mut cfg = hint_test_config();
        cfg.teleports.insert("tp1".to_string(), root.join("tp1"));
        Config::with_test_config(cfg, || {
            let h = NtcHinter::new();
            // mkf / rmf offer files; directories stay traversable.
            for line in ["mkf @tp1/", "rmf @tp1/", "mkf a.txt, @tp1/"] {
                let c = h.candidates_for(line);
                assert!(
                    c.contains(&"@tp1/file.txt".to_string()),
                    "got: {:?} for {}",
                    c,
                    line
                );
            }
            // mkd / rmd offer directories only.
            for line in ["mkd @tp1/", "rmd @tp1/"] {
                let c = h.candidates_for(line);
                assert!(
                    c.contains(&"@tp1/sub/".to_string()),
                    "got: {:?} for {}",
                    c,
                    line
                );
                assert!(
                    !c.contains(&"@tp1/file.txt".to_string()),
                    "got: {:?} for {}",
                    c,
                    line
                );
            }
            // Nested traversal.
            let c = h.candidates_for("rmf @tp1/sub/d");
            assert!(c.contains(&"@tp1/sub/deep.txt".to_string()), "got: {:?}", c);
        });
    }

    #[test]
    fn rmf_star_dot_completes_extensions() {
        // `rmf *.<Tab>` offers `*.<ext>` for extensions present in the dir
        // (fixture has `.toml`, `.rs`, …); a typed stub filters them.
        let _cwd = TempCwd::with_repo_fixture();
        let c = fs_candidates_for(&["rmf"], "*.");
        assert!(!c.is_empty(), "no *-candidates offered");
        assert!(
            c.iter().all(|x| x.starts_with("*.")),
            "non-pattern offered: {:?}",
            c
        );
        assert!(c.contains(&"*.toml".to_string()), "got: {:?}", c);
        let c2 = fs_candidates_for(&["rmf"], "*.t");
        assert!(!c2.is_empty(), "stub filtered everything");
        assert!(c2.iter().all(|x| x.starts_with("*.")), "got: {:?}", c2);
        assert!(c2.contains(&"*.toml".to_string()), "got: {:?}", c2);
        assert!(c2.len() <= c.len());
        // A fully-typed pattern ghosts nothing.
        assert!(fs_candidates_for(&["rmf"], "*.toml").is_empty());
    }

    #[test]
    fn mv_cp_star_completes_extensions() {
        // `cp *` offers `*` itself plus every `*.<ext>` present, so Tab
        // cycles `*` -> `*.toml` -> `*.txt` -> ….
        let _cwd = TempCwd::with_repo_fixture();
        let c = fs_candidates_for(&["cp"], "*");
        assert!(c.contains(&"*".to_string()), "got: {:?}", c);
        assert!(c.contains(&"*.toml".to_string()), "got: {:?}", c);
        assert!(c.contains(&"*.txt".to_string()), "got: {:?}", c);
        assert!(c.contains(&"*.png".to_string()), "got: {:?}", c);
        // A typed stub filters and drops the bare `*`.
        let c2 = fs_candidates_for(&["mv"], "*.t");
        assert!(!c2.contains(&"*".to_string()), "got: {:?}", c2);
        assert!(c2.contains(&"*.toml".to_string()), "got: {:?}", c2);
        assert!(c2.contains(&"*.txt".to_string()), "got: {:?}", c2);
        // Fully-typed pattern, dst slot and non-pattern forms stay quiet.
        assert!(fs_candidates_for(&["cp"], "*.toml").is_empty());
        assert!(fs_candidates_for(&["cp", "a.txt"], "*.py").is_empty());
        assert!(fs_candidates_for(&["cp"], "*a").is_empty());
        // End-to-end through the hinter, with a leading flag.
        let h = NtcHinter::new();
        let c = h.candidates_for("cp -f *");
        assert!(c.contains(&"*".to_string()), "got: {:?}", c);
        assert!(c.contains(&"*.toml".to_string()), "got: {:?}", c);
    }

    #[test]
    fn stage_tail_splits_top_level_operators() {
        assert_eq!(NtcHinter::stage_tail("ran reset && tl ").0, " tl ");
        assert_eq!(NtcHinter::stage_tail("ran reset && tl").0, " tl");
        assert_eq!(NtcHinter::stage_tail("tl").0, "tl");
        // Pipes and redirects split too; the operator is reported.
        assert_eq!(NtcHinter::stage_tail("gs main | sort").0, " sort");
        assert_eq!(NtcHinter::stage_tail("gs main | sort").1, Some("|"));
        assert_eq!(NtcHinter::stage_tail("a || b").1, Some("||"));
        assert_eq!(NtcHinter::stage_tail("a > out.txt").1, Some(">"));
        assert_eq!(NtcHinter::stage_tail("a >> out.txt").1, Some(">>"));
        assert_eq!(NtcHinter::stage_tail("a < in.txt").1, Some("<"));
        assert_eq!(NtcHinter::stage_tail("a <<EOF").0, "EOF");
        assert_eq!(NtcHinter::stage_tail("a <<< hi").1, Some("<<<"));
        // Single `&` is literal, quoted operators are literal, unclosed
        // quotes swallow the rest — all mean "no operator".
        assert_eq!(NtcHinter::stage_tail("a & b").0, "a & b");
        assert_eq!(NtcHinter::stage_tail("say \"a && b").0, "say \"a && b");
        assert_eq!(NtcHinter::stage_tail("say \"a && b\" && tl ").0, " tl ");
        assert_eq!(NtcHinter::stage_tail("&&").0, "");
        // Unspaced operators are data, exactly like the executor.
        assert_eq!(NtcHinter::stage_tail("math a|b").0, "math a|b");
        assert_eq!(
            NtcHinter::stage_tail("nsheet x --filter age>30").0,
            "nsheet x --filter age>30"
        );
    }

    #[test]
    fn chained_segments_complete_like_single_lines() {
        let h = NtcHinter::new();
        // Second segment behaves as if typed alone (any command).
        assert_eq!(
            h.candidates_for("cd / && tl "),
            vec!["dir".to_string(), "ext".to_string(), "file".to_string()]
        );
        assert_eq!(h.candidates_for("tl d && tl e"), vec!["ext".to_string()]);
        // Depth-sensitive logic (ext slot) works past `&&` too.
        assert!(h
            .candidates_for("tl ext && scan ext r")
            .contains(&"rs".to_string()));
        // `--help` suppression is per-segment now.
        assert_eq!(
            h.candidates_for("view --help && tl "),
            vec!["dir".to_string(), "ext".to_string(), "file".to_string()]
        );
        // Empty tail stays quiet, like an empty line.
        assert!(h.candidates_for("ran reset &&").is_empty());
        assert!(h.candidates_for("ran reset && ").is_empty());
    }

    #[test]
    fn at_fragment_filters_teleport_names() {
        // Regression: `@s` never suggested `@shop` — candidates carry
        // the `@` but were filtered by the stripped stub.
        let mut cfg = Config::default();
        cfg.teleports
            .insert("shop".to_string(), std::path::PathBuf::from("/tmp/shop"));
        cfg.teleports
            .insert("work".to_string(), std::path::PathBuf::from("/tmp/work"));
        Config::with_test_config(cfg, || {
            let h = NtcHinter::new();
            // The reported case: `@s` ghosts `@shop`.
            assert_eq!(h.candidates_for("@s"), vec!["@shop".to_string()]);
            // Bare `@` lists everything; fully-typed ghosts nothing.
            let all = h.candidates_for("@");
            assert!(all.contains(&"@shop".to_string()), "got: {:?}", all);
            assert!(all.contains(&"@work".to_string()), "got: {:?}", all);
            assert!(h.candidates_for("@shop").is_empty());
            // Case-insensitive, both positions.
            assert_eq!(h.candidates_for("@S"), vec!["@shop".to_string()]);
            assert_eq!(h.candidates_for("tp jump @w"), vec!["@work".to_string()]);
            // `cd @…` stays quiet (bare cd takes subdirs only).
            assert!(h.candidates_for("cd @s").is_empty());
        });
    }

    #[test]
    fn bare_at_sign_lists_teleports() {
        let h = NtcHinter::new();
        // Whatever the store holds, every candidate is an `@name`.
        // (Empty store → empty list; never filesystem noise.)
        for c in h.candidates_for("@") {
            assert!(c.starts_with('@'), "unexpected: {}", c);
        }
        // `cd @` stays quiet (bare cd takes subdirs only).
        assert!(h.candidates_for("cd @").is_empty());
    }

    #[test]
    fn filter_prefix_is_case_insensitive() {
        let out = filter_prefix(vec!["Downloads".to_string(), "docs".to_string()], "d");
        assert_eq!(out, vec!["Downloads", "docs"]);
        let out = filter_prefix(vec!["dir".to_string(), "ext".to_string()], "");
        assert_eq!(out, vec!["dir", "ext"]);
    }

    #[test]
    fn help_suffix_strips_at_any_depth() {
        assert_eq!(strip_help_suffix("cd --help"), Some("cd".to_string()));
        assert_eq!(
            strip_help_suffix("ignore dir --help"),
            Some("ignore dir".to_string())
        );
        assert_eq!(
            strip_help_suffix("ignore dir foo --help"),
            Some("ignore dir foo".to_string())
        );
        assert_eq!(
            strip_help_suffix("cd to foo -h"),
            Some("cd to foo".to_string())
        );
        assert_eq!(strip_help_suffix("cd"), None);
        assert_eq!(strip_help_suffix("ignore dir foo"), None);
    }

    #[test]
    fn help_flag_is_suggested_for_dash_frag() {
        let h = NtcHinter::new();
        assert!(h
            .candidates_for("ignore --")
            .contains(&"--help".to_string()));
        assert_eq!(h.candidates_for("ignore --h"), vec!["--help".to_string()]);
        assert!(h.candidates_for("cd -").contains(&"--help".to_string()));
    }

    #[test]
    fn help_token_suppresses_ghost() {
        let h = NtcHinter::new();
        assert!(h.candidates_for("ignore --help").is_empty());
        assert!(h.candidates_for("ignore dir --help").is_empty());
        assert!(h.candidates_for("cd to foo --help").is_empty());
        assert!(h.candidates_for("ignore --help ").is_empty());
    }

    #[test]
    fn serve_flags_include_auth() {
        let h = NtcHinter::new();
        // `serve --a` ghosts `--auth` (and `--auth` alone lists flags).
        assert!(h
            .candidates_for("serve --a")
            .contains(&"--auth".to_string()));
        // Fully typed flags ghost nothing (ghost is the untyped remainder).
        assert!(h.candidates_for("serve --auth").is_empty());
        assert!(h.candidates_for("serve --").contains(&"--auth".to_string()));
        // Sibling flags keep working alongside it.
        assert!(h
            .candidates_for("serve --t")
            .contains(&"--tunnel".to_string()));
        // Share flags (internet-first LAN + QR).
        assert!(h
            .candidates_for("serve --s")
            .contains(&"--share".to_string()));
        assert!(h
            .candidates_for("serve --")
            .contains(&"--share".to_string()));
        assert!(h.candidates_for("serve --").contains(&"--qr".to_string()));
        assert!(h
            .candidates_for("serve --")
            .contains(&"--no-qr".to_string()));
        // Never-implemented flags must not be suggested (`--https` used to
        // ghost here while cmd_serve rejected it as unknown).
        assert!(!h
            .candidates_for("serve --")
            .contains(&"--https".to_string()));
        assert!(h.candidates_for("serve --ht").is_empty());
        // `--lock` (serve files read-only) completes on both arms.
        assert!(h.candidates_for("serve --").contains(&"--lock".to_string()));
        assert!(h
            .candidates_for("serve files --")
            .contains(&"--lock".to_string()));
        assert!(h
            .candidates_for("serve files --l")
            .contains(&"--lock".to_string()));
    }

    #[test]
    fn serve_config_subcommands_complete() {
        let h = NtcHinter::new();
        // `serve c` suggests the config subcommand alongside others.
        assert!(h.candidates_for("serve c").contains(&"config".to_string()));
        assert!(h
            .candidates_for("serve config ")
            .contains(&"files".to_string()));
        assert!(h
            .candidates_for("serve config ")
            .contains(&"http-server".to_string()));
        // Depth-2 flag slots suggest the setter flags.
        assert!(h
            .candidates_for("serve config files --")
            .contains(&"--port-default".to_string()));
        assert!(h
            .candidates_for("serve config files --")
            .contains(&"--bind-default".to_string()));
        // Same from the `config serve` alias side.
        assert!(h.candidates_for("config s").contains(&"serve".to_string()));
        assert!(h
            .candidates_for("config serve ")
            .contains(&"files".to_string()));
    }

    #[test]
    fn serve_chess_flags_exclude_auth() {
        let h = NtcHinter::new();
        // `serve chess` rejects --auth/--spa: never suggested.
        assert!(h
            .candidates_for("serve chess --")
            .contains(&"--port".to_string()));
        assert!(h
            .candidates_for("serve chess --")
            .contains(&"--web-dir".to_string()));
        assert!(!h
            .candidates_for("serve chess --")
            .contains(&"--auth".to_string()));
        assert!(!h
            .candidates_for("serve chess --")
            .contains(&"--spa".to_string()));
        // `serve http-server` rejects --spa/--web-dir: never suggested.
        assert!(h
            .candidates_for("serve http-server --")
            .contains(&"--auth".to_string()));
        assert!(!h
            .candidates_for("serve http-server --")
            .contains(&"--spa".to_string()));
        assert!(!h
            .candidates_for("serve http-server --")
            .contains(&"--web-dir".to_string()));
    }

    #[test]
    fn nsheet_flags_include_open_view() {
        let h = NtcHinter::new();
        assert!(h.candidates_for("nsheet o").contains(&"open".to_string()));
        assert!(h
            .candidates_for("nsheet --f")
            .contains(&"--filter".to_string()));
        assert!(h
            .candidates_for("nsheet --")
            .contains(&"--sort".to_string()));
        assert!(h
            .candidates_for("nsheet --")
            .contains(&"--export".to_string()));
        assert!(h.candidates_for("nsheet --xyz").is_empty());
    }

    #[test]
    fn nssh_flags_include_subcommands() {
        let h = NtcHinter::new();
        assert!(h.candidates_for("nssh ").contains(&"ssh".to_string()));
        assert!(h.candidates_for("nssh s").contains(&"ssh".to_string()));
        assert!(h
            .candidates_for("nssh ssh --")
            .contains(&"--cmd".to_string()));
        assert!(h
            .candidates_for("nssh ssh --c")
            .contains(&"--cmd".to_string()));
        assert!(h
            .candidates_for("nssh add --h")
            .contains(&"--host".to_string()));
        assert!(h
            .candidates_for("nssh scan --t")
            .contains(&"--top".to_string()));
        assert!(h.candidates_for("nssh scan --xyz").is_empty());
    }

    fn hint_test_config() -> Config {
        let mut cfg = Config::default();
        cfg.run_aliases
            .insert("tld".to_string(), "tl dir ${x:-.}".to_string());
        cfg.run_alias_hints
            .insert("tld".to_string(), "#x --hint-dir".to_string());
        cfg.run_aliases
            .insert("pyc".to_string(), "python ${x:-test}".to_string());
        cfg.run_alias_hints
            .insert("pyc".to_string(), "#x --hint-ext py,toml".to_string());
        cfg.run_aliases
            .insert("cop".to_string(), "cp ${x} ${y}".to_string());
        cfg.run_alias_hints.insert(
            "cop".to_string(),
            "#x --hint-all #y --hint-tp --hint-dir".to_string(),
        );
        cfg.run_aliases
            .insert("plain".to_string(), "echo ${x}".to_string());
        cfg
    }

    #[test]
    fn alias_hint_dir_lists_dot_and_dirs() {
        let _cwd = TempCwd::with_repo_fixture();
        Config::with_test_config(hint_test_config(), || {
            let c = alias_arg_candidates_for(&["tld"], "", true).unwrap();
            assert!(c.contains(&".".to_string()), "got: {:?}", c);
            assert!(c.contains(&"src".to_string()), "got: {:?}", c);
            assert!(c.contains(&"docs".to_string()), "got: {:?}", c);
            // Dir hint: no files.
            assert!(!c.contains(&"notes.txt".to_string()), "got: {:?}", c);
            // End-to-end ghost path.
            let h = NtcHinter::new();
            assert!(h.candidates_for("tld ").contains(&"src".to_string()));
            assert!(h.candidates_for("tld s").contains(&"src".to_string()));
        });
    }

    #[test]
    fn alias_hint_ext_filters_files() {
        let _cwd = TempCwd::with_repo_fixture();
        Config::with_test_config(hint_test_config(), || {
            let c = alias_arg_candidates_for(&["pyc"], "", true).unwrap();
            assert!(c.contains(&"app.toml".to_string()), "got: {:?}", c);
            assert!(!c.contains(&"notes.txt".to_string()), "got: {:?}", c);
            assert!(!c.contains(&"photo.png".to_string()), "got: {:?}", c);
        });
    }

    #[test]
    fn alias_hint_positional_and_call_form_index() {
        let _cwd = TempCwd::with_repo_fixture();
        Config::with_test_config(hint_test_config(), || {
            // Second positional slot → y's hints (tp + dir).
            let c = alias_arg_candidates_for(&["cop", "a.txt"], "", true).unwrap();
            assert!(c.contains(&".".to_string()), "got: {:?}", c);
            assert!(c.contains(&"src".to_string()), "got: {:?}", c);
            // Call form `cop(a, b` → same second slot (the `b`
            // fragment lives outside parent parts, like candidates_for).
            let c = alias_arg_candidates_for(&["cop(a,"], "b", false).unwrap();
            assert!(c.contains(&".".to_string()), "got: {:?}", c);
            let h = NtcHinter::new();
            assert!(h.candidates_for("cop(a, ").contains(&".".to_string()));
            // First slot → x's hints (all: files present).
            let c = alias_arg_candidates_for(&["cop"], "", true).unwrap();
            assert!(c.contains(&"notes.txt".to_string()), "got: {:?}", c);
            // Out-of-range slot → None (default behavior).
            assert!(alias_arg_candidates_for(&["cop", "a", "b", "c"], "", true).is_none());
        });
    }

    #[test]
    fn alias_hint_bracket_elements_complete() {
        let _cwd = TempCwd::with_repo_fixture();
        let mut cfg = hint_test_config();
        cfg.run_aliases
            .insert("runc".to_string(), "gcc -o ${o}.exe $x[]".to_string());
        cfg.run_alias_hints
            .insert("runc".to_string(), "#x[] --hint-ext c".into());
        // tld's dir hints observe brackets too (elements = dirs).
        Config::with_test_config(cfg, || {
            // `runc o [ma` — element slot of the array param, head kept
            // so the ghost preserves the bracket.
            let c = alias_arg_candidates_for(&["runc", "o"], "[ma", false).unwrap();
            assert!(c.iter().all(|s| s.starts_with('[')), "got: {:?}", c);
            // `tld [s` — dir elements with head.
            let c = alias_arg_candidates_for(&["tld"], "[s", false).unwrap();
            assert!(c.contains(&"[src".to_string()), "got: {:?}", c);
            // Multi-element group `tld [src, d` — still slot 0; the
            // bare frag completes bare elements (delimiters live in
            // parent tokens, so they must not leak into the head).
            let c = alias_arg_candidates_for(&["tld", "[src,"], "d", false).unwrap();
            assert!(c.contains(&"docs".to_string()), "got: {:?}", c);
            assert!(!c.iter().any(|s| s.starts_with('[')), "got: {:?}", c);
            // Same, but the bracket lives in the frag itself.
            let c = alias_arg_candidates_for(&["tld"], "[src, d", false).unwrap();
            assert!(c.contains(&"[src, docs".to_string()), "got: {:?}", c);
            // Trailing space inside brackets → bare elements.
            let c = alias_arg_candidates_for(&["tld", "[src,"], "", true).unwrap();
            assert!(c.contains(&"docs".to_string()), "got: {:?}", c);
            assert!(!c.iter().any(|s| s.starts_with('[')), "got: {:?}", c);
            // Closed bracket ends the slot → past-the-end, no hints.
            assert!(alias_arg_candidates_for(&["runc", "o", "[a.c]"], "", true).is_none());
            // End-to-end: ghost keeps the bracket text.
            let h = NtcHinter::new();
            let c = h.candidates_for("tld [");
            assert!(!c.is_empty(), "expected bracket-headed ghosts");
            assert!(c.iter().all(|s| s.starts_with('[')), "got: {:?}", c);
        });
    }

    #[test]
    fn ral_hint_suggests_values_named_and_hintn() {
        let h = NtcHinter::new();
        // Bare --hint / --select and their -n forms are suggested.
        let c = h.candidates_for("ral add tld \"tl dir $x\" #x --sel");
        assert!(c.contains(&"--select-dir".to_string()), "got: {:?}", c);
        assert!(c.contains(&"--select-n".to_string()), "got: {:?}", c);
        assert!(c.contains(&"--select".to_string()), "got: {:?}", c);
        let c = h.candidates_for("ral add tld \"tl dir $x\" #x --hin");
        assert!(c.contains(&"--hint-n".to_string()), "got: {:?}", c);
        assert!(c.contains(&"--hint".to_string()), "got: {:?}", c);
        // hintn subcommands complete.
        let c = h.candidates_for("ral ");
        assert!(c.contains(&"hintn".to_string()), "got: {:?}", c);
        let c = h.candidates_for("ral hintn ");
        assert!(c.contains(&"add".to_string()), "got: {:?}", c);
        assert!(c.contains(&"ls".to_string()), "got: {:?}", c);
        // After --hint-n / --select-n, list names complete.
        let mut cfg = hint_test_config();
        cfg.run_hint_lists.insert(
            "port".to_string(),
            vec!["8080".to_string(), "5050".to_string()],
        );
        Config::with_test_config(cfg, || {
            let h = NtcHinter::new();
            let c = h.candidates_for("ral add srv \"ssh $p\" #p --hint-n ");
            assert!(c.contains(&"port".to_string()), "got: {:?}", c);
            let c = h.candidates_for("ral add srv \"ssh $p\" #p --hint-n p");
            assert!(c.contains(&"port".to_string()), "got: {:?}", c);
            // hintn rm/info complete list names too.
            let c = h.candidates_for("ral hintn rm ");
            assert!(c.contains(&"port".to_string()), "got: {:?}", c);
            let c = h.candidates_for("ral hintn info p");
            assert!(c.contains(&"port".to_string()), "got: {:?}", c);
        });
    }

    #[test]
    fn alias_hint_values_and_named_complete() {
        let _cwd = TempCwd::with_repo_fixture();
        let mut cfg = hint_test_config();
        cfg.run_aliases
            .insert("srv".to_string(), "ssh $p".to_string());
        cfg.run_alias_hints
            .insert("srv".to_string(), "#p --hint [8080,5050]".into());
        cfg.run_aliases
            .insert("srvn".to_string(), "ssh $p".to_string());
        cfg.run_alias_hints
            .insert("srvn".to_string(), "#p --hint-n port".into());
        cfg.run_aliases
            .insert("srvboth".to_string(), "ssh $p".to_string());
        cfg.run_alias_hints.insert(
            "srvboth".to_string(),
            "#p --hint [8080] --select-n port".into(),
        );
        cfg.run_hint_lists.insert(
            "port".to_string(),
            vec!["8080".to_string(), "5050".to_string()],
        );
        Config::with_test_config(cfg, || {
            // Inline values complete verbatim.
            let c = alias_arg_candidates_for(&["srv"], "", true).unwrap();
            assert!(c.contains(&"8080".to_string()), "got: {:?}", c);
            assert!(c.contains(&"5050".to_string()), "got: {:?}", c);
            // Named refs resolve through the store.
            let c = alias_arg_candidates_for(&["srvn"], "", true).unwrap();
            assert!(c.contains(&"8080".to_string()), "got: {:?}", c);
            assert!(c.contains(&"5050".to_string()), "got: {:?}", c);
            // Mixed inline + named union.
            let c = alias_arg_candidates_for(&["srvboth"], "", true).unwrap();
            assert!(c.contains(&"8080".to_string()), "got: {:?}", c);
            assert!(c.contains(&"5050".to_string()), "got: {:?}", c);
        });
        // Dangling named refs complete nothing (no fallback leak).
        let mut cfg2 = hint_test_config();
        cfg2.run_aliases
            .insert("dangle".to_string(), "ssh $p".to_string());
        cfg2.run_alias_hints
            .insert("dangle".to_string(), "#p --hint-n ghost".into());
        Config::with_test_config(cfg2, || {
            let c = alias_arg_candidates_for(&["dangle"], "", true).unwrap();
            assert!(c.is_empty(), "got: {:?}", c);
        });
    }

    #[test]
    fn alias_hint_missing_falls_back() {
        let _cwd = TempCwd::with_repo_fixture();
        Config::with_test_config(hint_test_config(), || {
            // Unaliased root → None.
            assert!(alias_arg_candidates_for(&["nosuchalias"], "", true).is_none());
            assert!(alias_arg_candidates_for(&["nosuchalias", "x"], "", true).is_none());
            // Alias without hints → None (today's empty behavior kept).
            assert!(alias_arg_candidates_for(&["plain"], "", true).is_none());
            // Builtins never route here even if named alike (reserved).
            assert!(alias_arg_candidates_for(&["view"], "", true).is_none());
        });
    }

    #[test]
    fn ral_hint_suggests_params_then_flags() {
        let h = NtcHinter::new();
        // `#` after a template with placeholders → #param names.
        let c = h.candidates_for("ral add tld \"tl dir ${x:-.}\" #");
        assert!(c.contains(&"#x".to_string()), "got: {:?}", c);
        // Array placeholders keep their brackets.
        let c = h.candidates_for("ral edit runc \"gcc $x[]\" #");
        assert!(c.contains(&"#x[]".to_string()), "got: {:?}", c);
        // `--hint` prefix → the hint flags.
        let c = h.candidates_for("ral add tld \"tl dir $x\" #x --hint-");
        for f in [
            "--hint-tp",
            "--hint-ext",
            "--hint-ext-name",
            "--hint-spf",
            "--hint-file",
            "--hint-file-name",
            "--hint-dir",
            "--hint-fd",
            "--hint-all",
        ] {
            assert!(c.contains(&f.to_string()), "missing {} in {:?}", f, c);
        }
        // `--select` prefix → the select flags.
        let c = h.candidates_for("ral add tld \"tl dir $x\" #x --select-");
        for f in [
            "--select-dir",
            "--select-ext",
            "--select-ext-name",
            "--select-file",
            "--select-file-name",
            "--select-fd",
            "--select-tp",
        ] {
            assert!(c.contains(&f.to_string()), "missing {} in {:?}", f, c);
        }
        // Just-finished #param → flags as well.
        let c = h.candidates_for("ral add tld \"tl dir $x\" #x ");
        assert!(c.contains(&"--hint-dir".to_string()), "got: {:?}", c);
        // No placeholders → nothing extra.
        let c = h.candidates_for("ral add btr \"cargo build\" #");
        assert!(!c.iter().any(|s| s.starts_with('#')), "got: {:?}", c);
        // Non add/edit roots stay quiet.
        let c = h.candidates_for("ral list #");
        assert!(!c.iter().any(|s| s.starts_with('#')), "got: {:?}", c);
    }

    #[test]
    fn hint_name_kinds_yield_file_stems() {
        use crate::shell::alias::ParamHintKind as K;
        let _cwd = TempCwd::with_repo_fixture();
        // `--hint-file-name` = stems of every CWD file.
        let c = hint_kind_candidates(&[K::FileName]);
        assert_eq!(c, vec!["Cargo", "app", "notes", "photo"]);
        // `--hint-ext-name toml` = stems of matching files only.
        let c = hint_kind_candidates(&[K::ExtName(vec!["toml".into()])]);
        assert_eq!(c, vec!["Cargo", "app"]);
        // Multi-extension union, sorted/deduped.
        let c = hint_kind_candidates(&[K::ExtName(vec!["txt".into(), "png".into()])]);
        assert_eq!(c, vec!["notes", "photo"]);
        // Non-matching extension → empty.
        assert!(hint_kind_candidates(&[K::ExtName(vec!["rs".into()])]).is_empty());
        // Bare `--hint-ext-name` behaves like `--hint-file-name`.
        assert_eq!(
            hint_kind_candidates(&[K::ExtName(vec![])]),
            vec!["Cargo", "app", "notes", "photo"]
        );
    }

    #[test]
    fn image_slots_filter_to_images_plus_dirs() {
        // Every non-dir candidate must be an image file; dirs pass through
        // for `dir/<Tab>` traversal.
        let _cwd = TempCwd::with_repo_fixture();
        for parent in [
            &["image"][..],
            &["img"][..],
            &["image", "display"][..],
            &["image", "show"][..],
            &["img", "display"][..],
        ] {
            for c in fs_candidates_for(parent, "") {
                if c.ends_with('/') {
                    continue;
                }
                let l = c.to_lowercase();
                assert!(
                    IMAGE_EXTS.iter().any(|e| l.ends_with(e)),
                    "non-image offered for {:?}: {}",
                    parent,
                    c
                );
            }
        }
        // Fixture has non-image files (Cargo.toml) that must be suppressed,
        // and `src/` dir that must still pass through.
        let c = fs_candidates_for(&["image"], "");
        assert!(
            !c.iter().any(|x| x == "Cargo.toml"),
            "non-image file offered: {:?}",
            c
        );
        assert!(
            c.iter().any(|x| x == "src/"),
            "dirs must pass image filter: {:?}",
            c
        );
        // Deeper positions (flags/values) complete nothing.
        assert!(fs_candidates_for(&["image", "a.png", "to"], "").is_empty());
        assert!(fs_candidates_for(&["image", "display", "a.png"], "").is_empty());
    }

    #[test]
    fn fmt_source_slot_offers_convertible_files_only() {
        // Directories plus fmt-supported files only.
        let _cwd = TempCwd::with_fixture(
            &[
                ("README.md", "# hi\n"),
                ("Cargo.toml", "[package]\n"),
                ("notes.txt", "hello\n"),
                ("data.docx", "fake"),
                ("book.pdf", "fake"),
                ("book.xlsx", "fake"),
                ("image.png", "fake"),
                ("docs/raw.bin", "fake"),
            ],
            &["docs"],
        );
        let c = fs_candidates_for(&["fmt"], "");
        assert!(
            c.iter().any(|x| x == "README.md"),
            "md file must be offered: {:?}",
            c
        );
        assert!(
            c.iter().any(|x| x == "Cargo.toml"),
            "toml file must be offered: {:?}",
            c
        );
        assert!(
            c.iter().any(|x| x == "data.docx"),
            "docx file must be offered: {:?}",
            c
        );
        assert!(
            c.iter().any(|x| x == "notes.txt"),
            "txt file must be offered: {:?}",
            c
        );
        assert!(
            !c.iter().any(|x| x == "image.png"),
            "unsupported ext must be suppressed: {:?}",
            c
        );
        assert!(
            !c.iter().any(|x| x == "book.pdf" || x == "book.xlsx"),
            "write-only formats must not be offered as sources: {:?}",
            c
        );
        assert!(
            c.iter().any(|x| x == "docs/"),
            "dirs must pass for traversal: {:?}",
            c
        );
        // `fmt docs/` traversal filters inside the directory too.
        let c = fs_candidates_for(&["fmt"], "docs/");
        assert!(
            !c.iter().any(|x| x.ends_with(".bin")),
            "unsupported file listed inside traversal: {:?}",
            c
        );
        // Deeper positions (keyword/flag slots) complete no filesystem paths.
        assert!(fs_candidates_for(&["fmt", "x.md", "to"], "").is_empty());
        assert!(fs_candidates_for(&["fmt", "x.md", "-o"], "").is_empty());
    }

    #[test]
    fn fmt_to_keyword_and_target_slots() {
        let h = NtcHinter::new();
        // After the source file: the keyword/flag slots.
        let c = h.candidates_for("fmt README.md ");
        assert!(c.contains(&"to".to_string()), "got: {:?}", c);
        assert!(c.contains(&"-o".to_string()), "got: {:?}", c);
        assert!(c.contains(&"--output".to_string()), "got: {:?}", c);
        // While typing the keyword.
        assert!(h
            .candidates_for("fmt README.md t")
            .contains(&"to".to_string()));
        // The `to` target slot lists every conversion format…
        for fmt_ in [
            "json", "yaml", "toml", "md", "html", "xml", "docx", "txt", "pdf", "csv", "tsv", "xlsx",
        ] {
            assert!(
                h.candidates_for("fmt README.md to ")
                    .contains(&fmt_.to_string()),
                "missing {fmt_}: got {:?}",
                h.candidates_for("fmt README.md to ")
            );
        }
        // …with prefix filtering.
        let c = h.candidates_for("fmt README.md to j");
        assert_eq!(c, vec!["json".to_string()]);
        // No filesystem paths in the `to` slot.
        assert!(!h
            .candidates_for("fmt README.md to ")
            .contains(&"README.md".to_string()));
    }

    #[test]
    fn fmt_output_slot_completes_full_names() {
        let h = NtcHinter::new();
        // Bare `-o ` suggests `out.<ext>` filenames.
        let c = h.candidates_for("fmt x.md to json -o ");
        assert!(c.contains(&"out.json".to_string()), "got: {:?}", c);
        assert!(c.contains(&"out.docx".to_string()), "got: {:?}", c);
        assert!(c.contains(&"out.pdf".to_string()), "got: {:?}", c);
        assert!(c.contains(&"out.xlsx".to_string()), "got: {:?}", c);
        // A dotless stub grows a dot with the whole palette.
        let c = h.candidates_for("fmt x.md to json -o j");
        assert!(c.contains(&"j.json".to_string()), "got: {:?}", c);
        // Partial extension keeps the typed basename.
        let c = h.candidates_for("fmt x.md to json -o o.j");
        assert!(c.contains(&"o.json".to_string()), "got: {:?}", c);
        assert!(!c.contains(&"o.html".to_string()), "got: {:?}", c);
        // `md` prefix matches both `.md` and `.markdown` (case-insensitive).
        let c = h.candidates_for("fmt x.md to json --output n.m");
        assert!(c.contains(&"n.md".to_string()), "got: {:?}", c);
        assert!(c.contains(&"n.markdown".to_string()), "got: {:?}", c);
        // Slotted after other words? Never outside the fmt root or the
        // flag slot.
        assert!(h.candidates_for("scan src -o ").is_empty());
    }

    #[test]
    fn image_sixel_flag_ghosts_after_path() {
        let h = NtcHinter::new();
        // Flag-first and flag-after-path both offer --sixel…
        assert!(h
            .candidates_for("image display --")
            .contains(&"--sixel".to_string()));
        assert!(h
            .candidates_for("image display a.png --")
            .contains(&"--sixel".to_string()));
        assert!(h
            .candidates_for("image show a.png --six")
            .contains(&"--sixel".to_string()));
        // …but other subcommands never do.
        assert!(!h
            .candidates_for("image foo --")
            .contains(&"--sixel".to_string()));
        // Top level still lists it alongside the other flags.
        assert!(h
            .candidates_for("image --")
            .contains(&"--sixel".to_string()));
    }

    #[test]
    fn qur_flags_include_select_replace_display() {
        let h = NtcHinter::new();
        // Flag fragments ghost qur flags (plus universal --help/-h).
        assert!(h
            .candidates_for("qur --s")
            .contains(&"--select".to_string()));
        assert!(h
            .candidates_for("qur --")
            .contains(&"--display".to_string()));
        assert!(h
            .candidates_for("qur --")
            .contains(&"--replace".to_string()));
        assert!(h.candidates_for("qur -").contains(&"-s".to_string()));
        assert!(h.candidates_for("qur -").contains(&"-rp".to_string()));
        assert!(h.candidates_for("qur -").contains(&"-dp".to_string()));
        assert!(h.candidates_for("qur --xyz").is_empty());
        // Value-taking flag awaiting its value offers no subcommands.
        assert!(subcommands_for(&["qur", "-s"]).is_empty());
        assert!(subcommands_for(&["qur", "-d"]).is_empty());
        assert!(!subcommands_for(&["qur"]).is_empty());
    }

    #[test]
    fn qur_glob_slot_only_before_content_flags() {
        // Glob slot: files complete.
        assert!(qur_glob_slot(&["qur"]));
        assert!(qur_glob_slot(&["qur", "*.rs"]));
        assert!(qur_glob_slot(&["qur", "*.rs", "-d", "3"]));
        // Value expected: no files.
        assert!(!qur_glob_slot(&["qur", "-s"]));
        assert!(!qur_glob_slot(&["qur", "*.rs", "-s"]));
        assert!(!qur_glob_slot(&["qur", "*.rs", "-rp"]));
        assert!(!qur_glob_slot(&["qur", "*.rs", "-d"]));
        // Content flag present: glob phase over.
        assert!(!qur_glob_slot(&["qur", "*.rs", "-s", "foo"]));
        assert!(!qur_glob_slot(&["qur", "*.rs", "-s", "foo", "-dp"]));
        // fs candidates mirror the slot.
        assert!(fs_candidates_for(&["qur", "-s"], "").is_empty());
        assert!(fs_candidates_for(&["qur", "*.rs", "-s", "foo"], "").is_empty());
    }

    #[test]
    fn txtf_flags_include_head_tail_betw() {
        let h = NtcHinter::new();
        assert!(h.candidates_for("txtf --h").contains(&"--head".to_string()));
        assert!(h.candidates_for("txtf --").contains(&"--betw".to_string()));
        assert!(h.candidates_for("txtf --xyz").is_empty());
        // Value-taking flag awaiting its value offers no subcommands.
        assert!(subcommands_for(&["txtf", "--head"]).is_empty());
        assert!(!subcommands_for(&["txtf"]).is_empty());
        // view --betw ghosts like the other valued flags.
        assert!(h.candidates_for("view --b").contains(&"--betw".to_string()));
        assert!(subcommands_for(&["view", "--betw"]).is_empty());
    }

    #[test]
    fn help_completes_canonical_topics_only() {
        let h = NtcHinter::new();
        let c = h.candidates_for("help ");
        for t in [
            "nav", "config", "image", "arc", "scan", "teleport", "nssh", "nsheet", "nms", "pipe",
            "fake", "time",
        ] {
            assert!(c.contains(&t.to_string()), "missing topic {}: {:?}", t, c);
        }
        // Aliases are not suggested — canonical names only.
        assert!(!c.contains(&"cd".to_string()), "got: {:?}", c);
        assert!(!c.contains(&"img".to_string()), "got: {:?}", c);
        assert!(!c.contains(&"pipes".to_string()), "got: {:?}", c);
        // Prefix + case-insensitive filtering.
        assert_eq!(h.candidates_for("help im"), vec!["image".to_string()]);
        assert_eq!(h.candidates_for("help IM"), vec!["image".to_string()]);
        assert_eq!(h.candidates_for("help ar"), vec!["arc".to_string()]);
        assert_eq!(h.candidates_for("help pi"), vec!["pipe".to_string()]);
        assert_eq!(h.candidates_for("help PI"), vec!["pipe".to_string()]);
        assert_eq!(h.candidates_for("help fa"), vec!["fake".to_string()]);
        // Topic filled: no further hints.
        assert!(h.candidates_for("help image ").is_empty());
        assert!(h.candidates_for("help pipe ").is_empty());
    }

    #[test]
    fn time_completions_offer_command_modes() {
        let h = NtcHinter::new();
        for candidate in ["run", "-r", "--run", "count", "help"] {
            assert!(
                h.candidates_for("time ").contains(&candidate.to_string()),
                "missing {candidate}"
            );
        }
        assert!(h.candidates_for("time c").contains(&"count".to_string()));
    }

    #[test]
    fn fake_completions_offer_subcommands_and_types() {
        let subs = subcommands_for(&["fake"]);
        for s in ["gen", "types", "locales", "group", "help"] {
            assert!(subs.contains(&s), "fake missing subcommand {s}: {subs:?}");
        }
        let gen_flags = subcommands_for(&["fake", "gen"]);
        assert!(gen_flags.contains(&"-n"));
        assert!(gen_flags.contains(&"--group"));
        assert!(gen_flags.contains(&"--unique"));
        assert!(gen_flags.contains(&"--locale"));

        let group_subs = subcommands_for(&["fake", "group"]);
        assert!(group_subs.contains(&"export"));
        assert!(group_subs.contains(&"import"));

        let types = fake_names_for_command(&["fake", "gen"], "");
        assert!(types.iter().any(|t| t == "name"), "got: {types:?}");
        assert!(types.iter().any(|t| t == "email"), "got: {types:?}");

        let locales = fake_names_for_command(&["fake", "gen", "-l"], "");
        assert!(locales.iter().any(|l| l == "en_US"), "got: {locales:?}");
        assert!(locales.iter().any(|l| l == "th_TH"), "got: {locales:?}");

        // `fake gen -g <TAB>` / `fake group rm <TAB>` suggest saved group
        // names; the count depends on config, so only assert they don't panic.
        let _ = fake_names_for_command(&["fake", "gen", "-g"], "");
        let _ = fake_names_for_command(&["fake", "group", "rm"], "");
    }

    #[test]
    fn pipe_stage_offers_only_the_pipe_filters() {
        let h = NtcHinter::new();
        let filters = vec![
            "head".to_string(),
            "tail".to_string(),
            "betw".to_string(),
            "gs".to_string(),
            "sort".to_string(),
            "rev".to_string(),
            "count".to_string(),
            "trim".to_string(),
            "encode".to_string(),
            "qur".to_string(),
            "clip".to_string(),
        ];
        assert_eq!(h.candidates_for("gs main | "), filters);
        // Cursor immediately after the pipe (no trailing space) still lists them.
        assert_eq!(h.candidates_for("gs main |"), filters);
        assert_eq!(h.candidates_for("gs main | s"), vec!["sort".to_string()]);
        assert_eq!(h.candidates_for("gs main | r"), vec!["rev".to_string()]);
        assert_eq!(h.candidates_for("gs main | be"), vec!["betw".to_string()]);
        assert_eq!(h.candidates_for("gs main | tr"), vec!["trim".to_string()]);
        assert_eq!(h.candidates_for("gs main | e"), vec!["encode".to_string()]);
        assert_eq!(h.candidates_for("gs main | q"), vec!["qur".to_string()]);
        // No fallback for other commands (they never complete in a pipe).
        assert!(h.candidates_for("gs main | say ").is_empty());
        assert!(h.candidates_for("gs main | sa").is_empty());
        assert!(h.candidates_for("gs main | view a").is_empty());
        assert!(h.candidates_for("gs main | -").is_empty());
    }

    #[test]
    fn pipe_stage_filters_complete_their_flags_and_files() {
        let h = NtcHinter::new();
        for flag in [
            "-r",
            "--reverse",
            "-n",
            "--numeric",
            "-u",
            "--unique",
            "-f",
            "--ignore-case",
        ] {
            assert!(
                h.candidates_for("gs main | sort -")
                    .contains(&flag.to_string()),
                "sort missing {flag}"
            );
        }
        for flag in ["-p", "--path", "-s", "--sort", "-d", "-l", "--limit", "-i"] {
            assert!(
                h.candidates_for("gs main | gs -")
                    .contains(&flag.to_string()),
                "gs missing {flag}"
            );
        }
        for flag in [
            "-s",
            "--select",
            "-rp",
            "--replace",
            "-dp",
            "--display",
            "-d",
            "--depth",
            "-l",
            "--limit",
        ] {
            assert!(
                h.candidates_for("gs main | qur -")
                    .contains(&flag.to_string()),
                "qur missing {flag}"
            );
        }
        for codec in ["b2", "b10", "b64", "base64", "hex", "url"] {
            assert!(
                h.candidates_for("gs main | encode ")
                    .contains(&codec.to_string()),
                "encode missing {codec}"
            );
        }
        assert!(h
            .candidates_for("gs main | encode b64 -")
            .contains(&"-o".to_string()));
        assert!(h
            .candidates_for("gs main | encode b64 -")
            .contains(&"--output".to_string()));
        // Head/tail/betw/sort take files; `head`/`tail`/`betw` have no flags.
        assert!(h
            .candidates_for("gs main | head -")
            .contains(&"--help".to_string()));
        assert!(!h
            .candidates_for("gs main | head --")
            .contains(&"--tail".to_string()));
    }

    #[test]
    fn pipe_stage_files_use_existing_command_completion() {
        let _cwd = TempCwd::with_repo_fixture();
        let h = NtcHinter::new();
        assert!(h
            .candidates_for("gs main | head ")
            .contains(&"notes.txt".to_string()));
        assert!(h
            .candidates_for("gs main | sort ")
            .contains(&"Cargo.toml".to_string()));
        assert!(h
            .candidates_for("gs main | qur ")
            .contains(&"Cargo.toml".to_string()));
    }

    #[test]
    fn output_redirect_never_hints() {
        let _cwd = TempCwd::with_repo_fixture();
        let h = NtcHinter::new();
        assert!(h.candidates_for("gs main > ").is_empty());
        assert!(h.candidates_for("gs main > out").is_empty());
        assert!(h.candidates_for("gs main >").is_empty());
    }

    #[test]
    fn append_and_input_redirects_hint_text_files_only() {
        let _cwd = TempCwd::with_repo_fixture();
        let h = NtcHinter::new();
        for prefix in ["gs main >> ", "gs main < ", "gs main >>", "gs main <"] {
            let c = h.candidates_for(prefix);
            assert!(
                c.contains(&"Cargo.toml".to_string()),
                "{prefix} missing text file: {c:?}"
            );
            assert!(
                c.contains(&"docs/".to_string()),
                "{prefix} missing traversal dir: {c:?}"
            );
            assert!(
                !c.iter().any(|x| x == "photo.png"),
                "{prefix} offered an image: {c:?}"
            );
        }
        // Prefix filtering and nested traversal.
        assert_eq!(
            h.candidates_for("gs main >> no"),
            vec!["notes.txt".to_string()]
        );
        assert!(h
            .candidates_for("gs main < src/")
            .contains(&"src/main.rs".to_string()));
        // Slot filled: nothing more to hint.
        assert!(h.candidates_for("gs main >> notes.txt ").is_empty());
        assert!(h.candidates_for("gs main < notes.txt ").is_empty());
    }

    #[test]
    fn heredoc_delimiter_is_always_eof() {
        let h = NtcHinter::new();
        assert_eq!(h.candidates_for("say << "), vec!["EOF".to_string()]);
        assert_eq!(h.candidates_for("say <<"), vec!["EOF".to_string()]);
        assert_eq!(h.candidates_for("say <<E"), vec!["EOF".to_string()]);
        assert_eq!(h.candidates_for("say <<EOF"), Vec::<String>::new());
        assert_eq!(h.candidates_for("say <<EOF "), Vec::<String>::new());
        assert!(h.candidates_for("say <<XYZ").is_empty());
        // Here-strings take free text: no hint.
        assert!(h.candidates_for("say <<< ").is_empty());
        assert!(h.candidates_for("say <<< hi").is_empty());
    }

    #[test]
    fn ghost_remainder_adds_a_space_after_a_bare_operator() {
        assert_eq!(
            ghost_remainder("gs main |", "head"),
            Some(" head".to_string())
        );
        assert_eq!(
            ghost_remainder("gs main | ", "head"),
            Some("head".to_string())
        );
        assert_eq!(
            ghost_remainder("gs main | s", "sort"),
            Some("ort".to_string())
        );
        assert_eq!(ghost_remainder("say <<", "EOF"), Some(" EOF".to_string()));
        assert_eq!(
            ghost_remainder("ran reset &&", "tl"),
            Some(" tl".to_string())
        );
    }

    #[test]
    fn double_pipe_segments_complete_commands() {
        let h = NtcHinter::new();
        assert!(h
            .candidates_for("gs main || tl ")
            .contains(&"ext".to_string()));
        assert_eq!(h.candidates_for("gs main || tl d"), vec!["dir".to_string()]);
    }
}
