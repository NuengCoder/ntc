//! Rustyline integration: ghost-text hinting, Tab cycling and the candidate
//! orchestrator. Command-tree data lives in `tree`, dynamic values in
//! `providers`, filesystem completion in `files`.

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::SystemTime;

use rustyline::hint::Hinter;
use rustyline::{Cmd, ConditionalEventHandler, Context, Event, EventContext, RepeatCount};

use crate::config::Config;
use crate::teleport::TeleportManager;

mod files;
mod providers;
#[cfg(test)]
mod tests;
mod tree;

// The orchestrator and its tests use the submodule helpers directly. These
// are private imports, not re-exports.
use files::*;
use providers::*;
use tree::*;

// Explicit crate-visible surface: only these helpers are used outside
// `hinter`, so the re-exports stay narrow instead of globbing whole modules.
pub(crate) use providers::{
    alias_names_for_command, ntpkg_names_for_command, ran_target_names_for_command,
    scr_scripts_for_command, teleport_names_for_command, theme_names_for_command,
};
pub(crate) use tree::{build_command_set, is_builtin_command, subcommands_for};

/// Generation counter bumped whenever run aliases are modified.
/// The hinter's `hint()` method checks this and refreshes its cache when stale.
pub(crate) static ALIAS_GENERATION: AtomicU64 = AtomicU64::new(0);

/// Force the hinter to rebuild its command set on the next keystroke.
/// Used when `integration_shell` or `code_tools` flips (registry names
/// join/leave level 1).
pub(crate) fn bump_generation() {
    ALIAS_GENERATION.fetch_add(1, Ordering::Relaxed);
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

/// Identity of the local `ntconfig.toml` as seen by the hinter. The file can
/// appear, disappear or be edited while the shell is running (for example via
/// `local init` or an external editor), so command names must be rebuilt when
/// this changes even though the alias-generation counter did not move.
#[derive(PartialEq)]
struct LocalConfigStamp {
    path: Option<PathBuf>,
    modified: Option<SystemTime>,
    len: u64,
}

fn local_config_stamp() -> LocalConfigStamp {
    let path = Config::get_local_config_path();
    let metadata = path.as_ref().and_then(|p| std::fs::metadata(p).ok());
    LocalConfigStamp {
        path,
        modified: metadata.as_ref().and_then(|m| m.modified().ok()),
        len: metadata.map(|m| m.len()).unwrap_or(0),
    }
}

/// A rustyline `Hinter` that suggests autocompletion as ghost text.
/// Tab cycles through all candidates as ghost text; Right-arrow accepts.
pub(crate) struct NtcHinter {
    commands: RwLock<HashSet<String>>,
    generation: AtomicU64,
    local_config: Mutex<LocalConfigStamp>,
    cycle: Arc<Mutex<CycleState>>,
    /// Rotation index for the idle-mode (empty prompt) teaching hints.
    idle_step: Arc<AtomicU64>,
}

impl NtcHinter {
    pub fn new() -> Self {
        NtcHinter {
            commands: RwLock::new(build_command_set()),
            generation: AtomicU64::new(ALIAS_GENERATION.load(Ordering::Relaxed)),
            local_config: Mutex::new(local_config_stamp()),
            cycle: Arc::new(Mutex::new(CycleState::default())),
            idle_step: Arc::new(AtomicU64::new(0)),
        }
    }

    fn ensure_fresh(&self) {
        let current = ALIAS_GENERATION.load(Ordering::Relaxed);
        let cached = self.generation.load(Ordering::Relaxed);
        let stamp = local_config_stamp();
        let stamp_changed = match self.local_config.lock() {
            Ok(cached_stamp) => *cached_stamp != stamp,
            Err(_) => true,
        };
        if current != cached || stamp_changed {
            let mut cmds = self.commands.write().unwrap();
            *cmds = build_command_set();
            self.generation.store(current, Ordering::Relaxed);
            if let Ok(mut cached_stamp) = self.local_config.lock() {
                *cached_stamp = stamp;
            }
        }
    }

    /// Candidate words plus the byte offset of their base inside `typed`.
    /// `usize::MAX` marks ordinary last-word candidates (ghosted relative to
    /// the active word); any other value means full-line candidates whose
    /// base starts at that offset.
    fn ghost_candidates(&self, typed: &str, _pos: usize) -> (Vec<String>, usize) {
        if crate::shell::ext_complete::ps_mode_active()
            || crate::shell::ext_complete::bash_mode_active()
            || crate::shell::ext_complete::zsh_mode_active()
        {
            let cands =
                crate::shell::ext_complete::line_candidates(typed, typed.len()).unwrap_or_default();
            // The whole line belongs to the host shell; full-line candidates
            // replace text from the start, so the base is the line's beginning.
            return (cands, 0);
        }
        let (tail, _) = Self::stage_tail(typed);
        let offset = typed.len() - tail.len();
        if crate::shell::ext_complete::owns_line(tail) {
            if let Some(cands) = crate::shell::ext_complete::line_candidates(tail, tail.len()) {
                if !cands.is_empty() {
                    return (cands, offset);
                }
            }
            // Owned but no external answer: keep ntc's own candidates.
            return (self.candidates_for(typed), usize::MAX);
        }
        let cands = self.candidates_for(typed);
        if !cands.is_empty() {
            return (cands, usize::MAX);
        }
        // No ntc candidate: let an external shell/tool answer instead.
        match crate::shell::ext_complete::line_candidates(tail, tail.len()) {
            Some(cands) => (cands, offset),
            None => (Vec::new(), usize::MAX),
        }
    }

    /// Advance the idle-hint rotation: the entry loop calls this once before
    /// each `readline`, so a new prompt shows the next suggestion while a
    /// redraw of the same prompt keeps the current one. `prompt_width` is the
    /// visible width of the prompt about to be drawn, so the ghost can be
    /// placed right after the caret without querying the terminal.
    pub(crate) fn next_idle_hint(&self, prompt_width: usize) {
        let step = self.idle_step.fetch_add(1, Ordering::Relaxed) as usize + 1;
        crate::shell::idle::prompt_started(step, prompt_width);
    }

    /// Handle for the Tab key handler so it can advance the same cycle state.
    pub(crate) fn tab_handler(&self, delta: isize) -> TabCycleHandler {
        TabCycleHandler {
            cycle: Arc::clone(&self.cycle),
            delta,
        }
    }

    /// Tail of the line after the last top-level operator, plus that
    /// operator (`&&`, `||`, `;`, `|`, `>`, `>>`, `<`, `<<`, `<<<`). Untrimmed —
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
        self.ensure_fresh();
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

        // Inside `nprintf`'s format string, `%` completes a conversion and a
        // trailing `\` completes an escape.
        if let Some(cands) = nprintf_format_candidates(typed) {
            return cands;
        }

        // `$` / `$NA` / `${NA`: every variable that expands here — OS,
        // global ntc, local ntc and session all count.
        if let Some(cands) = env_var_candidates(typed) {
            return cands;
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

        // `<cmd> check ` only ever takes the format file — no subcommands, no
        // filesystem noise.
        if !last_frag.starts_with('-') {
            if let Some(cands) = check_only_candidates(parent_parts, last_frag) {
                return dedup_sorted(cands);
            }
        }

        // Format files come before every other candidate at the positions
        // that take one, so `math <TAB>` ghosts the `.ntc.math` file in the
        // working directory before `timer` / `fun` / `check`.
        let format_first = if last_frag.starts_with('-') {
            Vec::new()
        } else {
            format_file_candidates(parent_parts, last_frag)
        };

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
            skill_names_for_command(parent_parts),
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
        // godrive/god completes drive letters (Windows) or /mnt mount names
        if parent_parts.len() == 1
            && matches!(parent_parts[0].to_lowercase().as_str(), "godrive" | "god")
        {
            out.extend(filter_prefix(
                crate::navigator::Navigator::drive_roots()
                    .into_iter()
                    .map(|r| r.label)
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

        if format_first.is_empty() {
            return dedup_sorted(out);
        }
        // Keep the order (no sort) so the format file stays the ghost text.
        let mut ordered: Vec<String> = Vec::new();
        for name in format_first.into_iter().chain(out) {
            if !ordered.contains(&name) {
                ordered.push(name);
            }
        }
        ordered
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

impl Hinter for NtcHinter {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<String> {
        if !Config::global_get_autosuggest_enabled() {
            // Autosuggest off also means no idle animation (and stop one that
            // may still be running after the setting was toggled).
            crate::shell::idle::stop();
            return None;
        }

        self.ensure_fresh();
        let mut pos = pos.min(line.len());
        while pos > 0 && !line.is_char_boundary(pos) {
            pos -= 1;
        }
        let typed = &line[..pos];

        if typed.is_empty() {
            // Idle mode: a blank prompt teaches one command at a time. The
            // text is typed out by the idle animator (30 s delay, typewriter
            // reveal, 15 s hold, next idea) and drawn after the caret, so
            // rustyline gets no hint here — Right/Tab/End can never insert it.
            let step = self.idle_step.load(Ordering::Relaxed) as usize;
            crate::shell::idle::prompt_shown(step);
            return None;
        }
        // Anything typed means the ghost must go away immediately.
        crate::shell::idle::stop();

        // `ntc ps` mode hands the whole line to PowerShell — ntc operators,
        // `--help` suppression and `@teleport` do not apply there.
        let (cands, base_len) = self.ghost_candidates(typed, pos);

        if cands.is_empty() {
            return None;
        }

        // Show the currently-cycled candidate as ghost text. Typing resets to
        // the first candidate; Tab advances (see TabCycleHandler).
        let (pick, cycling): (String, bool) = {
            let mut cyc = self.cycle.lock().unwrap();
            if cyc.line != typed || cyc.pos != pos {
                cyc.line = typed.to_string();
                cyc.pos = pos;
                cyc.index = 0;
            }
            let index = cyc.index.rem_euclid(cands.len() as isize);
            (cands[index as usize].clone(), index != 0)
        };

        // Staging is a preview for the first candidate only. While Tab is
        // cycling, show the whole remainder so candidates stay distinguishable
        // (`Get-Acl` and `Get-ChildItem` would otherwise both ghost `et`).
        if base_len != usize::MAX {
            // Full-line candidate: only the part after the base is ghosted.
            // PowerShell matches case-insensitively, so `get-c` keeps the
            // canonical `Get-Content` remainder.
            let base = &typed[base_len..];
            let rest = strip_prefix_ci(&pick, base)?;
            let rest = if cycling {
                rest
            } else {
                stage_rest(rest, &pick)
            };
            return if rest.is_empty() {
                None
            } else {
                Some(rest.to_string())
            };
        }

        // The ghost is the remainder after the active word, computed from
        // the current segment so `cmd |` ghosts `" head"` (space included).
        let rest =
            ghost_remainder(typed, &pick).or_else(|| nprintf_ghost_remainder(typed, &pick))?;
        let staged = if cycling {
            rest.as_str()
        } else {
            stage_rest(&rest, &pick)
        };
        if staged.is_empty() {
            None
        } else {
            Some(staged.to_string())
        }
    }
}

/// Ghost only the next hyphen-delimited stage of a command name.
///
/// A `-` inside a verb-noun name is a natural acceptance boundary:
/// `g` + `Get-Acl` → `et`, then `Get` + `Get-Acl` → `-`, then
/// `Get-` + `Get-Acl` → `Acl`. Flags (leading `-`) and path-like candidates
/// are left alone so their existing ghost behavior is unchanged.
fn stage_rest<'a>(rest: &'a str, candidate: &str) -> &'a str {
    if candidate.starts_with('-') || candidate.contains(['/', '\\', '.']) {
        return rest;
    }
    if rest.starts_with('-') {
        return &rest[..1];
    }
    match rest.find('-') {
        Some(i) if i > 0 => &rest[..i],
        _ => rest,
    }
}

/// Remove `prefix` from `s` without regard to ASCII case. The returned slice
/// starts at the same byte offset, so canonical casing survives.
fn strip_prefix_ci<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    let head = s.get(..prefix.len())?;
    if head.eq_ignore_ascii_case(prefix) {
        s.get(prefix.len()..)
    } else {
        None
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
