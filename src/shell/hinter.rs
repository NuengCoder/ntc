use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

use rustyline::hint::Hinter;
use rustyline::Context;

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
        "cd", "godrive", "god", "back", "b", "view", "ls",
        "report", "txtc", "txtf",
        "config", "clear", "version", "where",
        "gos", "gosc", "ral", "ran", "run", "r", "showcg", "help", "exit", "quit",
        "ignore", "care", "igcared",
        "size", "tp", "opencg", "resetcg", "restorecg", "local",
        "ne", "ntceditor", "igcare",
        "esc", "backup", "gs", "fs", "ds", "fgo", "fsc", "locate",
        "dino", "math", "theme" , "tpb", "tutorial", "init", "deinit",
        "explorer", "nfe",
        "rnmf", "rnmd",
        "image", "img" , "py_activate", "py_deactivate", "showenv",
        "ntpkg", "ntpx", "tl", "http", "dl", "up", "ping", "dns", "smtp", "serve", "clip", "qsc",
        "todo", "encode", "decode", "cvcode", "fmt", "qr",
        "usrpath", "syspath", "mode",
        "pathcp", "say", "print",
    ]
}

/// Returns the list of teleport save-point names for commands that accept them
/// as arguments (cd, tp jump/to/rm/rnm/info).
fn teleport_names_for_command(parts: &[&str]) -> Vec<String> {
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
fn alias_names_for_command(parts: &[&str]) -> Vec<String> {
    if parts.is_empty() {
        return vec![];
    }

    let root = parts[0].to_lowercase();
    if root != "ral" || parts.len() < 2 {
        return vec![];
    }

    match parts[1].to_lowercase().as_str() {
        "rm" | "remove" | "info" | "edit" => {
            let aliases = Config::global_get_run_aliases();
            aliases.into_keys().collect()
        }
        "rnm" | "rename" => {
            // rnm <old> to <new> — suggest alias names for the first argument
            let aliases = Config::global_get_run_aliases();
            aliases.into_keys().collect()
        }
        _ => vec![],
    }
}

/// Returns the list of ran target names from NTCRANFILE.toml for `ran <target>`.
fn ran_target_names_for_command(parts: &[&str]) -> Vec<String> {
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

/// Returns the list of ntpkg/ntpx argument completions (languages, versions, installed packages).
fn ntpkg_names_for_command(parts: &[&str]) -> Vec<String> {
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

/// Returns possible subcommand completions for a command path.
/// `parts` is the prefix path (the words before the one being typed).
///   - If `parts.len() == 1` (e.g. `["ral"]`), returns the subcommands of `ral`.
///   - If `parts.len() >= 2` (e.g. `["ral", "export"]`), drills deeper.
fn subcommands_for(parts: &[&str]) -> Vec<&'static str> {
    if parts.is_empty() {
        return vec![];
    }

    let root = parts[0].to_lowercase();
    match root.as_str() {
        // ral add / ral edit / ral rm / ral rnm / ral list / ...
        //   ral export --all / ral export --select
        "ral" => {
            if parts.len() == 1 {
                vec![
                    "add", "edit", "rm", "remove",
                    "list", "ls", "info",
                    "rnm", "rename", "export", "import",
                    "make", "mk",
                    "cls", "clear",
                ]
            } else {
                match parts[1].to_lowercase().as_str() {
                    "export" => vec!["--all", "-a", "--select", "-s"],
                    _ => vec![],
                }
            }
        }
        // tp add / tp jump / tp list / tp info / tp rm / tp rnm / tp cls / tp help
        //   tp jump <name> and tp to <name> suggest teleports via teleport_names_for_command
        "tp" => vec![
            "add", "jump", "to", "info", "list", "ls",
            "rm", "rnm", "rename", "cls", "help",
        ],
        // (watch is now a subcommand of config)
        // tpb history / tpb clear
        "tpb" => vec!["history", "hist", "h", "clear", "cls"],
        "theme" => vec![
            "add", "rm", "remove", "edit", "info", "export", "import",
            "make", "mk", "where", "wh", "reload",
            "n", "number", "rnm", "rename", "list" , "ls" ,
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
        // ran init / ran deinit / ran list
        "ran" if parts.len() == 1 => vec!["init", "deinit", "help", "list", "ls"],
        // config output / config depth / config linenumbers / config color / config autosuggest / config threads / config history / config watch
        "config" => {
            if parts.len() == 1 {
                vec!["output", "out", "depth", "d", "linenumbers", "ln", "line", "color", "c", "autosuggest", "as", "threads", "t", "history", "h", "watch", "w", "qsc"]
            } else {
                match parts[1].to_lowercase().as_str() {
                    "watch" | "w" if parts.len() == 2 => vec!["ON", "OFF", "trigger"],
                    "watch" | "w" if parts.len() >= 3 => vec!["off"],
                    _ => vec![],
                }
            }
        }
        // backup create / backup restore / backup undo / backup clear / backup recreate / backup diff / backup list
        "backup" if parts.len() == 1 => vec!["create", "cr", "restore", "rs", "undo", "ud", "clear", "cl", "recreate", "re", "diff", "list", "ls"],
        // backup restore --choose / --deleted / --force
        "backup" if parts.len() >= 2 && matches!(parts[1], "restore" | "rs") => vec!["--choose", "--deleted", "--force", "-f"],
        // backup undo --cls / backup undo --force
        "backup" if parts.len() >= 2 && matches!(parts[1], "undo" | "ud") => vec!["--cls", "--clear", "--force", "-f"],
        // backup clear --force
        "backup" if parts.len() >= 2 && matches!(parts[1], "clear" | "cl") => vec!["--force", "-f"],
        // backup recreate --force
        "backup" if parts.len() >= 2 && matches!(parts[1], "recreate" | "re") => vec!["--force", "-f"],
        // report txt / report json / report md / report html / report pdf / report docx / report xlsx
        "report" if parts.len() == 1 => vec!["txt", "json", "md", "html", "pdf", "docx", "xlsx"],
        // ignore dir / ignore ext / ignore file
        "ignore" if parts.len() == 1 => vec!["dir", "ext", "file"],
        // care dir / care ext / care file
        "care" if parts.len() == 1 => vec!["dir", "ext", "file"],
        // local init/deinit/help
        "local" => vec!["init", "--all", "-a", "deinit", "help"],
        // init --all / init --ran / init --local
        "init" if parts.len() == 1 => vec!["--all", "-a", "--ran", "--local"],
        // deinit --all / deinit --ran / deinit --local
        "deinit" if parts.len() == 1 => vec!["--all", "-a", "--ran", "--local"],
        // showenv --all / showenv -d
        "showenv" if parts.len() == 1 => vec!["--all", "-a", "-d", "--depth"],
        // size --care
        "size" if parts.len() == 1 => vec!["--care"],
        // view flags
        "view" if parts.len() == 1 => vec!["-s", "--size", "-c", "--cp", "--care", "-d", "--depth"],
        // ui (now a no-op after modern mode removal)
        "ui" => vec!["classic"],
        // ntpkg install / ntpkg uninstall / ntpkg list / ntpkg use / ntpkg which / ntpkg available / ntpkg search / ntpkg info
        "ntpkg" => vec![
            "install", "i",
            "uninstall", "rm",
            "list", "ls",
            "search",
            "info",
            "use",
            "which",
            "available", "avail",
            "help",
        ],
        "ntpx" => vec![], // Completions come from ntpkg_names_for_command
        // tl file / tl ext / tl dir
        "tl" if parts.len() == 1 => vec!["file", "ext", "dir"],
        // dl flags
        "dl" if !parts.is_empty() => vec!["--progress", "--resume", "--stop", "--cancel", "-o", "--output"],
        // ping flags
        "ping" if !parts.is_empty() => vec!["-c", "--count"],
        // dns flags
        "dns" if !parts.is_empty() => vec!["-t", "--type"],
        // serve flags
        "serve" if !parts.is_empty() => vec!["--port", "-p", "--bind", "-b", "--https", "-s", "--tunnel", "-t", "--spa", "--web-dir", "-w", "--help", "-h"],
        // smtp flags
        "smtp" if parts.len() == 1 => vec!["-i", "--server", "--id-ls"],
        "smtp" if parts.len() >= 2 => vec![
            "-i", "--server", "-p", "--port", "-u", "--user",
            "-pwd", "--password", "--id", "--id-ls", "--id-info",
            "--id-rm", "-t", "--to", "-c", "--cc", "--bcc",
            "-s", "--subject", "-b", "--body", "-f", "--attach",
            "-h", "--help",
        ],
        // up flags
        "up" if !parts.is_empty() => vec!["-H", "--header", "-d", "--data", "--post"],
        // http methods then flags
        "http" if parts.len() == 1 => vec!["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD"],
        "http" if parts.len() >= 2 => vec![
            "-H", "--header", "-d", "--data", "--json",
            "-q", "--query", "-a", "--auth", "--bearer",
            "-t", "--timeout", "-s", "--silent",
            "-o", "--output", "-v", "--verbose",
            "--no-pretty", "-h", "--help",
        ],
        // ne --init <file> / ne --super <file> / ne --hyper <name> --lang <lang>
        "ne" | "ntceditor" if parts.len() == 1 => vec!["--init", "--super", "--hyper"],
        "ne" | "ntceditor" if parts.len() >= 2 && parts[1] == "--hyper" => vec!["--lang"],
        // clip list / select / clear
        "clip" if parts.len() == 1 => vec!["list", "ls", "select", "sel", "clear", "cl"],
        // qsc listen / stop / status
        "qsc" if parts.len() == 1 => vec!["listen", "start", "stop", "status", "stat", "startup"],
        // todo info / add / rm / clear
        "todo" if parts.len() == 1 => vec!["info", "add", "rm", "remove", "clear", "cl", "list", "ls"],
        // encode b2 / b64 / hex / url
        "encode" if parts.len() == 1 => vec!["b2", "b64", "base64", "hex", "url"],
        // decode b2 / b64 / hex / url
        "decode" if parts.len() == 1 => vec!["b2", "b64", "base64", "hex", "url"],
        // fmt json / yaml / toml
        "fmt" if parts.len() == 1 => vec!["json", "yaml", "toml"],
        // qr encode / decode
        "qr" if parts.len() == 1 => vec!["encode", "en", "decode", "de"],
        // mode subcommands
        "mode" if parts.len() == 1 => vec!["admin", "normal"],
        // usrpath / syspath subcommands
        "usrpath" | "syspath" if parts.len() == 1 => vec!["show", "ls", "add", "rm", "remove", "move", "mv"],
        "usrpath" | "syspath" if parts.len() >= 2 && matches!(parts[1], "add") => vec!["--cwd", "--here"],
        // say flags
        "say" | "print" if parts.len() == 1 => vec!["--loop", "-n", "-t"],
        // Default: no known subcommands
        _ => vec![],
    }
}

/// A rustyline `Hinter` that suggests autocompletion as ghost text.
pub(crate) struct NtcHinter {
    commands: RwLock<HashSet<String>>,
    generation: AtomicU64,
}

impl NtcHinter {
    pub fn new() -> Self {
        NtcHinter {
            commands: RwLock::new(build_command_set()),
            generation: AtomicU64::new(ALIAS_GENERATION.load(Ordering::Relaxed)),
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
}

fn build_command_set() -> HashSet<String> {
    let mut cmds: HashSet<String> =
        builtin_commands().into_iter().map(|s| s.to_string()).collect();
    let aliases = Config::global_get_run_aliases();
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
        let pos = pos.min(line.len());
        let typed = &line[..pos];

        if typed.is_empty() {
            return None;
        }

        // If the typed text ends with a space, the user is between arguments;
        // wait until they start typing the next word to avoid distracting suggestions.
        if typed.ends_with(' ') {
            return None;
        }

        // Get whitespace-separated parts of what's been typed so far
        let parts: Vec<&str> = typed.split_whitespace().collect();

        if parts.is_empty() {
            return None;
        }

        if parts.len() == 1 {
            // ── Level 1: suggest top-level commands ──────────────────────

            // Special case: @<name> suggests teleport save-points
            if let Some(teleport_part) = typed.strip_prefix('@') {
                // everything after @
                if teleport_part.is_empty() {
                    return None;
                }
                let teleports = TeleportManager::get_all();
                let mut matches: Vec<String> = teleports
                    .keys()
                    .filter(|name| {
                        name.starts_with(&teleport_part.to_lowercase())
                            && name.len() > teleport_part.len()
                    })
                    .map(|name| format!("@{}", name))
                    .collect();
                matches.sort();
                if matches.len() == 1 {
                    let rest = &matches[0][typed.len()..];
                    return Some(rest.to_string());
                }
                return None;
            }

            let lower = parts[0].to_lowercase();
            let cmds = self.commands.read().unwrap();
            let mut matches: Vec<&str> = cmds
                .iter()
                .filter(|c| c.starts_with(&lower) && c.len() > typed.len())
                .map(|s| s.as_str())
                .collect();

            matches.sort();

            if matches.len() == 1 {
                let rest = &matches[0][typed.len()..];
                Some(rest.to_string())
            } else {
                None
            }
        } else {
            // ── Level 2+: suggest subcommands / teleport names / alias names ──
            let last_part = parts[parts.len() - 1];
            let parent_parts = &parts[..parts.len() - 1];

            // Check if the parent command accepts teleport names
            let teleports = teleport_names_for_command(parent_parts);

            // Check if the parent command accepts run alias names
            let aliases = alias_names_for_command(parent_parts);

            // Also check if typed is @<name> inside a command argument
            if let Some(teleport_part) = last_part.strip_prefix('@') {
                if !teleport_part.is_empty() {
                    let all_teleports = TeleportManager::get_all();
                    let mut matches: Vec<String> = all_teleports
                        .keys()
                        .filter(|name| {
                            name.starts_with(&teleport_part.to_lowercase())
                                && name.len() > teleport_part.len()
                        })
                        .map(|name| format!("@{}", name))
                        .collect();
                    matches.sort();
                    if matches.len() == 1 {
                        // Compute where the last word begins in the typed buffer
                        let last_word_start = typed.len() - last_part.len();
                        let rest = &matches[0][typed.len() - last_word_start..];
                        return Some(rest.to_string());
                    }
                    return None;
                }
            }

            // Match against teleport names (for commands like cd, tp jump, tp to, tp rm, etc.)
            if !teleports.is_empty() {
                let lower = last_part.to_lowercase();
                let mut matches: Vec<&str> = teleports
                    .iter()
                    .filter(|name| name.starts_with(&lower) && name.len() > last_part.len())
                    .map(|s| s.as_str())
                    .collect();
                matches.sort();
                matches.dedup();

                if matches.len() == 1 {
                    let last_word_start = typed.len() - last_part.len();
                    let rest = &matches[0][typed.len() - last_word_start..];
                    return Some(rest.to_string());
                }
                // If no unique teleport match, fall through to alias names
            }

            // Check if the parent command accepts ran target names
            let ran_targets = ran_target_names_for_command(parent_parts);

            // Match against run alias names (for commands like ral rm, ral info, ral edit, etc.)
            if !aliases.is_empty() {
                let lower = last_part.to_lowercase();
                let mut matches: Vec<&str> = aliases
                    .iter()
                    .filter(|name| name.starts_with(&lower) && name.len() > last_part.len())
                    .map(|s| s.as_str())
                    .collect();
                matches.sort();
                matches.dedup();

                if matches.len() == 1 {
                    let last_word_start = typed.len() - last_part.len();
                    let rest = &matches[0][typed.len() - last_word_start..];
                    return Some(rest.to_string());
                }
                // If no unique alias match, fall through to ran targets
            }

            // Match against ran target names (for commands like ran build, ran test, etc.)
            if !ran_targets.is_empty() {
                let lower = last_part.to_lowercase();
                let mut matches: Vec<&str> = ran_targets
                    .iter()
                    .filter(|name| name.starts_with(&lower) && name.len() > last_part.len())
                    .map(|s| s.as_str())
                    .collect();
                matches.sort();
                matches.dedup();

                if matches.len() == 1 {
                    let last_word_start = typed.len() - last_part.len();
                    let rest = &matches[0][typed.len() - last_word_start..];
                    return Some(rest.to_string());
                }
                // If no unique ran target match, fall through to ntpkg names
            }

            // Match against ntpkg argument names (languages, versions, installed packages)
            let ntpkg_names = ntpkg_names_for_command(parent_parts);
            if !ntpkg_names.is_empty() {
                let lower = last_part.to_lowercase();
                let mut matches: Vec<&str> = ntpkg_names
                    .iter()
                    .filter(|name| name.starts_with(&lower) && name.len() > last_part.len())
                    .map(|s| s.as_str())
                    .collect();
                matches.sort();
                matches.dedup();

                if matches.len() == 1 {
                    let last_word_start = typed.len() - last_part.len();
                    let rest = &matches[0][typed.len() - last_word_start..];
                    return Some(rest.to_string());
                }
                // If no unique ntpkg name match, fall through to subcommands
            }

            let subs = subcommands_for(parent_parts);
            if subs.is_empty() {
                return None;
            }

            // Find subcommands that start with the last typed fragment
            let lower = last_part.to_lowercase();
            let mut matches: Vec<&str> = subs
                .iter()
                .copied()
                .filter(|c| {
                    let c_lower = c.to_lowercase();
                    c_lower.starts_with(&lower) && c.len() > last_part.len()
                })
                .collect();

            matches.sort();
            matches.dedup();

            if matches.len() == 1 {
                // Compute offset to where the last word begins in the typed buffer
                let last_word_start = typed.len() - last_part.len();
                let rest = &matches[0][typed.len() - last_word_start..];
                Some(rest.to_string())
            } else {
                None
            }
        }
    }
}