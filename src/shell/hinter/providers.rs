//! Dynamic value providers: teleport, alias, ran, script, skill, ntpkg,
//! nssh, fake, theme and environment names, plus alias param-hint slots.

use super::files::*;
use super::filter_prefix;
use crate::config::Config;
use crate::shell::commands::builtin::ntpkg::ntpkg;
use crate::shell::commands::builtin::ran::ranfile::parser as ran_parser;
use crate::teleport::TeleportManager;

/// Returns the list of nssh profile names for commands that accept them
/// (`nssh ssh/info/rm/edit <name>`, bare `nssh <name>`, `nssh cp <name>:…`).
pub(crate) fn nssh_profile_names_for_command(parts: &[&str]) -> Vec<String> {
    if parts.is_empty() {
        return vec![];
    }
    if parts[0].to_lowercase() != "nssh" {
        return vec![];
    }
    let Ok(profiles) = crate::shell::commands::builtin::nssh::nssh::store::list_profiles() else {
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
pub(crate) enum TpEntryMode {
    All,
    Files,
    Dirs,
}

/// Which entry kinds a `@tp/sub` completion should list for `cmd`.
pub(crate) fn tp_entry_mode_for(cmd: &str, parent_parts: &[&str]) -> TpEntryMode {
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
pub(crate) fn txtf_file_slot_open(parent_parts: &[&str]) -> bool {
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
pub(crate) fn tp_subpath_candidates(
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

/// Candidates for one `#param --hint-*` group (union over its kinds).
pub(crate) fn hint_kind_candidates(kinds: &[crate::shell::alias::ParamHintKind]) -> Vec<String> {
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
            // `--hint-table*` needs an open `ndb` database: nothing here.
            K::Table | K::TableColName(_) => {}
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Candidates for one `#param --select-*` group (union over its kinds).
pub(crate) fn select_kind_candidates(
    kinds: &[crate::shell::alias::ParamSelectKind],
) -> Vec<String> {
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
pub(crate) fn alias_slot(
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

pub(crate) const HINT_FLAGS: &[&str] = &[
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

pub(crate) const SELECT_FLAGS: &[&str] = &[
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
pub(crate) fn hint_list_names() -> Vec<String> {
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
    match crate::shell::commands::builtin::scr::script::list_scripts(&cwd) {
        Ok(scripts) => scripts
            .into_iter()
            .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .collect(),
        Err(_) => vec![],
    }
}

/// Skill names for `skill add/remove/edit/delete <TAB>`, from the registry.
pub(crate) fn skill_names_for_command(parts: &[&str]) -> Vec<String> {
    if parts.is_empty() || !parts[0].eq_ignore_ascii_case("skill") {
        return vec![];
    }
    let Some(sub) = parts.get(1) else {
        return vec![];
    };
    match sub.to_lowercase().as_str() {
        "add" | "remove" | "rm" | "delete" | "del" | "uninstall" | "u" | "edit" | "sub" => {
            crate::shell::commands::builtin::skill::skill::installed_names()
        }
        _ => vec![],
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
                ntpkg::registry::names().map(|s| s.to_string()).collect()
            } else if parts.len() == 3 {
                // Suggest cached versions for the specified language
                let lang = parts[2];
                ntpkg::registry::peek_cached_versions(lang)
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
                ntpkg::registry::names().map(|s| s.to_string()).collect()
            } else {
                vec![]
            }
        }
        _ => vec![],
    }
}

pub(crate) fn installed_package_names(depth: usize) -> Vec<String> {
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
    let group_names = || crate::shell::commands::builtin::fake::fake::group::group_names();
    let type_names = || -> Vec<String> {
        crate::shell::commands::builtin::fake::fake::types::TYPES
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
            return crate::shell::commands::builtin::fake::fake::locale::Locale::all()
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

/// Every environment variable name ntc can resolve, sorted and deduped:
/// session (`config env … -os`), local `ntconfig.toml`, the global ntc
/// config and the OS/process environment.
pub(crate) fn all_env_names() -> Vec<String> {
    let mut names: Vec<String> = std::env::vars_os()
        .filter_map(|(k, _)| k.into_string().ok())
        .collect();
    names.extend(crate::utils::paths::session_env().into_keys());
    let (global, local) = Config::get_env_by_source();
    names.extend(global.into_keys());
    names.extend(local.into_keys());
    names.sort();
    names.dedup();
    names
}

/// Candidates for a `$NAME` / `${NAME}` fragment: every available variable,
/// so a bare `$` lists what can expand while `$H` completes `$HOME`,
/// `$HOMEPATH`, … Returns `None` when the active word is not a plain variable
/// reference (`$PATH/sub`, `cost$5`), leaving the usual completion in charge.
pub(crate) fn env_var_candidates(typed: &str) -> Option<Vec<String>> {
    let word = typed.split_whitespace().last().unwrap_or("");
    let braced = word.starts_with("${");
    let prefix = if braced {
        word.get(2..)?
    } else {
        word.strip_prefix('$')?
    };
    if !prefix
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return None;
    }
    let mut out: Vec<String> = all_env_names()
        .into_iter()
        .filter(|n| n.starts_with(prefix))
        .map(|n| {
            if braced {
                format!("${{{}}}", n)
            } else {
                format!("${}", n)
            }
        })
        .collect();
    // A fully typed reference ghosts nothing more.
    out.retain(|c| c.len() > word.len());
    out.sort();
    out.dedup();
    Some(out)
}

/// Conversions `nprintf` accepts, in the order the ghost cycles them.
const NPRINTF_CONVERSIONS: &[&str] = &[
    "s", "d", "i", "u", "o", "x", "X", "f", "F", "e", "E", "g", "G", "a", "A", "c", "b", "q", "%",
];

/// Complete backslash escapes offered after a trailing `\`.
const NPRINTF_ESCAPES: &[&str] = &[
    "n", "t", "r", "\\", "\"", "'", "a", "b", "e", "E", "f", "v", "c",
];

/// The incomplete piece of an `nprintf` format that the ghost can finish.
enum NprintfTrigger<'a> {
    /// A conversion spec starting at `%`, including flags/width typed so far.
    Conversion(&'a str),
    /// A backslash waiting for its escape character.
    Escape,
}

/// Return the format text when the cursor is still inside `nprintf`'s first
/// argument. Quoted formats may contain spaces, so this scans quotes instead
/// of splitting on whitespace. The opening quote is stripped from an unclosed
/// quoted format; a fully closed format returns `None` (nothing left to type).
fn nprintf_active_format(input: &str) -> Option<&str> {
    fn skip_ws(input: &str, mut i: usize) -> usize {
        while i < input.len() {
            let ch = input[i..].chars().next().unwrap();
            if !ch.is_whitespace() {
                break;
            }
            i += ch.len_utf8();
        }
        i
    }

    let mut i = skip_ws(input, 0);
    let cmd_start = i;
    while i < input.len() {
        let ch = input[i..].chars().next()?;
        if ch.is_whitespace() {
            break;
        }
        i += ch.len_utf8();
    }
    if !input[cmd_start..i].eq_ignore_ascii_case("nprintf") {
        return None;
    }
    i = skip_ws(input, i);
    if i >= input.len() {
        return None;
    }

    // `nprintf -- <format>`: the option terminator is not the format.
    if input[i..].starts_with("--") {
        let after = i + 2;
        let ends = after == input.len()
            || input[after..]
                .chars()
                .next()
                .is_some_and(char::is_whitespace);
        if ends {
            i = skip_ws(input, after);
            if i >= input.len() {
                return None;
            }
        }
    }

    let start = i;
    let mut quote: Option<char> = None;
    while i < input.len() {
        let ch = input[i..].chars().next()?;
        match quote {
            Some(q) => {
                if ch == q {
                    quote = None;
                    i += ch.len_utf8();
                } else if q == '"' && ch == '\\' {
                    // A backslash keeps the next quote from closing the word.
                    i += ch.len_utf8();
                    if i < input.len() {
                        i += input[i..].chars().next()?.len_utf8();
                    }
                } else {
                    i += ch.len_utf8();
                }
            }
            None => {
                if ch == '"' || ch == '\'' {
                    quote = Some(ch);
                    i += ch.len_utf8();
                } else if ch.is_whitespace() {
                    // The format argument ended; later arguments are free text.
                    return None;
                } else {
                    i += ch.len_utf8();
                }
            }
        }
    }

    let text = &input[start..];
    let first = text.chars().next()?;
    if quote == Some(first) {
        Some(&text[first.len_utf8()..])
    } else if quote.is_none() && matches!(first, '"' | '\'') && text.ends_with(first) {
        None
    } else {
        Some(text)
    }
}

/// True when `spec` is an incomplete conversion spec: flags, width,
/// precision and length modifiers are accepted, but no conversion character
/// has been typed yet.
fn nprintf_incomplete_spec(spec: &str) -> bool {
    let mut chars = spec.chars().peekable();
    while matches!(chars.peek(), Some('-' | '+' | ' ' | '#' | '0' | '\'')) {
        chars.next();
    }
    match chars.peek() {
        Some('*') => {
            chars.next();
        }
        Some(c) if c.is_ascii_digit() => {
            while matches!(chars.peek(), Some(c) if c.is_ascii_digit()) {
                chars.next();
            }
        }
        _ => {}
    }
    if chars.peek() == Some(&'.') {
        chars.next();
        if chars.peek() == Some(&'*') {
            chars.next();
        } else {
            while matches!(chars.peek(), Some(c) if c.is_ascii_digit()) {
                chars.next();
            }
        }
    }
    while matches!(chars.peek(), Some('h' | 'l' | 'L' | 'j' | 'z' | 't')) {
        chars.next();
    }
    chars.peek().is_none()
}

/// Identify the unfinished conversion or escape at the end of the active
/// `nprintf` format. Returns `None` unless the cursor is right after a lone
/// `%`/partial spec or an odd number of backslashes.
fn nprintf_trigger(input: &str) -> Option<NprintfTrigger<'_>> {
    let format = nprintf_active_format(input)?;
    let trailing_slashes = format.chars().rev().take_while(|c| *c == '\\').count();
    if trailing_slashes % 2 == 1 {
        return Some(NprintfTrigger::Escape);
    }

    let percent = format.rfind('%')?;
    let percent_run = format[..=percent]
        .chars()
        .rev()
        .take_while(|c| *c == '%')
        .count();
    // `%%` is a finished literal percent; only an unpaired percent opens a spec.
    if percent_run % 2 == 0 {
        return None;
    }
    let spec = &format[percent + 1..];
    if !nprintf_incomplete_spec(spec) {
        return None;
    }
    Some(NprintfTrigger::Conversion(&format[percent..]))
}

/// Ghost candidates inside `nprintf`'s format string: conversions after `%`
/// and escapes after `\`. Returns `None` outside an unfinished format token.
pub(crate) fn nprintf_format_candidates(typed: &str) -> Option<Vec<String>> {
    match nprintf_trigger(typed)? {
        NprintfTrigger::Conversion(prefix) => Some(
            NPRINTF_CONVERSIONS
                .iter()
                .map(|conv| format!("{prefix}{conv}"))
                .collect(),
        ),
        NprintfTrigger::Escape => Some(
            NPRINTF_ESCAPES
                .iter()
                .map(|escape| format!("\\{escape}"))
                .collect(),
        ),
    }
}

/// Remainder for a semantic `nprintf` candidate (`%s`, `%-8d`, `\n`) when the
/// format is quoted or has literal text before the trigger, so the candidate
/// itself does not start with the last whitespace-delimited word.
pub(crate) fn nprintf_ghost_remainder(typed: &str, pick: &str) -> Option<String> {
    let prefix = match nprintf_trigger(typed)? {
        NprintfTrigger::Conversion(prefix) => prefix,
        NprintfTrigger::Escape => "\\",
    };
    let rest = pick.strip_prefix(prefix)?;
    if rest.is_empty() {
        None
    } else {
        Some(rest.to_string())
    }
}

/// Theme names for `theme use <name>` and the other name-taking subcommands.
pub(crate) fn theme_names_for_command(parts: &[&str]) -> Vec<String> {
    if parts.len() < 2 {
        return vec![];
    }
    if parts[0].to_lowercase() != "theme" {
        return vec![];
    }
    match parts[1].to_lowercase().as_str() {
        "use" | "info" | "edit" | "update" | "rm" | "remove" | "rnm" | "rename" | "export" => {
            crate::utils::theme::ThemeManager::list_themes()
        }
        _ => vec![],
    }
}
