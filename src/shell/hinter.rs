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
        "ls",
        "report",
        "txtc",
        "txtf",
        "config",
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
        "git",
        "ndb",
        "nsheet",
        "nssh",
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
    ]
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

/// Returns possible subcommand completions for a command path.
/// `parts` is the prefix path (the words before the one being typed).
///   - If `parts.len() == 1` (e.g. `["ral"]`), returns the subcommands of `ral`.
///   - If `parts.len() >= 2` (e.g. `["ral", "export"]`), drills deeper.
pub(crate) fn subcommands_for(parts: &[&str]) -> Vec<&'static str> {
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
                    "add", "edit", "rm", "remove", "list", "ls", "info", "rnm", "rename", "export",
                    "import", "make", "mk", "cls", "clear",
                ]
            } else {
                match parts[1].to_lowercase().as_str() {
                    "export" => vec!["--all", "-a", "--select", "-s"],
                    "info" => vec!["--edit", "-e"],
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
        // ran init / ran deinit / ran list
        "ran" if parts.len() == 1 => vec!["init", "deinit", "help", "list", "ls"],
        // scr init / scr ls / scr help
        "scr" | "script" if parts.len() == 1 => vec!["init", "ls", "list", "help"],
        // nssh add / ssh / cp / scan / ls / info / rm / edit / keygen
        "nssh" if parts.len() == 1 => vec![
            "add", "ssh", "cp", "scan", "ls", "list", "info", "rm", "edit", "keygen",
        ],
        "nssh" if parts.len() >= 2 => match parts[1].to_lowercase().as_str() {
            "add" | "edit" => vec![
                "--host", "--user", "-u", "--port", "-p", "--key", "--no-key", "--pass-env",
                "--no-pass-env",
            ],
            "ssh" => vec!["--cmd", "--trust", "--ask-pass", "--timeout"],
            "cp" => vec!["--trust", "--ask-pass", "--timeout"],
            "scan" => vec!["--ports", "--top", "--timeout"],
            "rm" => vec!["--yes", "-y"],
            _ => vec![],
        },
        // config output / config depth / config linenumbers / config color / config autosuggest / config threads / config history / config watch
        "config" => {
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
                    "lang",
                    "language",
                    "git",
                    "dir_view",
                    "dv",
                ]
            } else {
                match parts[1].to_lowercase().as_str() {
                    "watch" | "w" if parts.len() == 2 => vec!["ON", "OFF", "trigger", "limit"],
                    "watch" | "w" if parts.len() >= 3 => vec!["off"],
                    "lang" | "language" if parts.len() == 2 => vec!["english", "thai", "lao", "chinese"],
                    "git" if parts.len() == 2 => vec!["ON", "OFF"],
                    "dir_view" | "dv" if parts.len() == 2 => vec!["ON", "OFF"],
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
            "-c",
            "--cp",
            "--care",
            "-d",
            "--depth",
            "-t",
            "--time",
            "-p",
            "--path",
            "-ext",
            "--ext",
        ],
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
        // ne --init <file> / ne --super <file> / ne --hyper <name> --lang <lang>
        "ne" | "ntceditor" if parts.len() == 1 => vec!["--init", "--super", "--hyper"],
        "ne" | "ntceditor" if parts.len() >= 2 && parts[1] == "--hyper" => vec!["--lang"],
        // clip list / select / clear
        "clip" if parts.len() == 1 => vec!["list", "ls", "select", "sel", "clear", "cl"],
        // qsc listen / stop / status
        "qsc" if parts.len() == 1 => vec!["listen", "start", "stop", "status", "stat", "startup"],
        // todo info / add / rm / clear
        "todo" if parts.len() == 1 => {
            vec!["info", "add", "done", "undo", "edit", "rm", "remove", "clear", "cl", "list", "ls"]
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
            if last == "-d"
                || last == "--depth"
                || last == "-p"
                || last == "--path"
                || last == "-ext"
                || last == "--ext"
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
                    "-c",
                    "--cp",
                    "--care",
                    "-d",
                    "--depth",
                    "-t",
                    "--time",
                    "-p",
                    "--path",
                    "-ext",
                    "--ext",
                ]
            }
        }
        // scan ext <ext...> [-d|--depth <n>] [-s|--size] [--care]
        // scan port [host] [--top N | --ports <spec> | --all] [--timeout MS]
        // scan ip [cidr] [--top N | --ports <spec>] [--timeout MS]
        "scan" if parts.len() == 1 => vec!["ext", "port", "ip", "help"],
        "scan" if parts.len() >= 2 && parts[1].to_lowercase() == "ext" => {
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
        // fmt json / yaml / toml
        "fmt" if parts.len() == 1 => vec!["json", "yaml", "toml"],
        // qr encode / decode
        "qr" if parts.len() == 1 => vec!["encode", "en", "decode", "de"],
        // image display / show / to / out / --ocr / --layout
        "image" | "img" if parts.len() == 1 => {
            vec!["display", "show", "to", "out", "--ocr", "--layout"]
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
        // say flags
        "say" | "print" if parts.len() == 1 => vec!["--loop", "-n", "-t"],
        // tmprt com / weather / key / city / sensors
        "tmprt" if parts.len() == 1 => vec![
            "com", "computer", "hw", "hardware", "weather", "w", "key", "city", "sensors",
            "sensor",
        ],
        // tmprt sensors <sub>
        "tmprt" if parts.len() >= 2 && matches!(parts[1], "sensors" | "sensor") => {
            vec!["status", "install", "uninstall", "debug"]
        }
        // git subcommands
        "git" if parts.len() == 1 => vec![
            "status", "st", "branch", "b", "log", "l", "diff", "d", "add", "a", "commit", "push",
            "pull", "clone", "checkout", "co", "merge", "fetch", "remote", "init", "stash",
            "rebase", "tag", "help", "h",
        ],
        // `help <topic>`: one canonical name per `print_topic_help`
        // section (no aliases).
        "help" if parts.len() == 1 => vec![
            "nav", "config", "editor", "theme", "teleport", "ran", "scr", "alias", "view",
            "size", "report", "backup", "search", "igcare", "network", "tmprt", "git",
            "clipboard", "qsc", "todo", "arc", "scan", "other", "syscmd", "fileops", "tl",
            "ntpkg", "qr", "pathman", "autoup", "mode", "explorer", "math", "python", "encdec",
            "fmt", "image", "ndb", "nsheet", "nssh",
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

    /// Text after the last `&&` outside double quotes: the segment
    /// being typed in a chained line (`ran reset && tl …` → `" tl "`).
    /// Untrimmed — the trailing space tells a filled slot (`ran `)
    /// apart from a half-typed word (`ran`). Single `&`, quoted `&&`
    /// and unclosed quotes all mean "no chain": the whole line.
    pub(crate) fn chain_tail(typed: &str) -> &str {
        let mut in_quotes = false;
        let mut last_sep: Option<usize> = None;
        let mut it = typed.char_indices().peekable();
        while let Some((i, c)) = it.next() {
            if c == '"' {
                in_quotes = !in_quotes;
            } else if c == '&' && !in_quotes {
                if matches!(it.peek(), Some((_, '&'))) {
                    last_sep = Some(i);
                }
            }
        }
        match last_sep {
            Some(i) => &typed[i + 2..],
            None => typed,
        }
    }

    /// Full candidate words for the word being typed in `typed`
    /// (cursor assumed at end of `typed`). Used both for ghost text and Tab cycling.
    pub(crate) fn candidates_for(&self, typed: &str) -> Vec<String> {
        if typed.is_empty() {
            return vec![];
        }
        // Chained lines (`a && b`): complete the active segment only.
        let typed = Self::chain_tail(typed);
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
                return filter_prefix(
                    vec!["--help".to_string(), "-h".to_string()],
                    word,
                );
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
            let mut flags: Vec<String> =
                vec!["--help".to_string(), "-h".to_string()];
            flags.extend(
                subcommands_for(parent_parts)
                    .into_iter()
                    .map(|s| s.to_string()),
            );
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
        let is_bare_cd =
            parent_parts.len() == 1 && parent_parts[0].to_lowercase() == "cd";

        // @<name> inside a command argument (bare `@` lists all).
        // Like above, the `@` stays in the comparison fragment.
        if let Some(tp_part) = last_frag.strip_prefix('@') {
            if is_bare_cd {
                return vec![];
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
        // godrive/god completes drive letters
        if parent_parts.len() == 1 && matches!(parent_parts[0].to_lowercase().as_str(), "godrive" | "god") {
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
        out.extend(fs_candidates_for(parent_parts, last_frag));

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

/// Image extensions completed for `image|img <file>` and
/// `image|img display|show <file>`. Directories always pass so
/// `dir/<Tab>` traversal keeps working (handled by the `ext_filter`
/// logic in `fs_candidates_for`); non-image files are suppressed.
const IMAGE_EXTS: &[&str] = &[
    ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".ico", ".tiff", ".tif", ".webp",
];

/// True when `view`'s optional target-path slot is already filled by a bare
/// token in `args`. Flag tokens, the values after value-taking flags
/// (`-d`/`--depth`, `-p`/`--path`) and `-ext` value runs don't count —
/// mirroring `extract_target_spec` in `helpers.rs` plus the ext pre-parse
/// in `report.rs`.
fn view_path_taken(args: &[&str]) -> bool {
    const VALUE_FLAGS: &[&str] = &["-d", "--depth", "-p", "--path"];
    const MULTI_FLAGS: &[&str] = &["-ext", "--ext"];
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let mut i = 0;
    while i < args.len() {
        let lower = args[i].to_lowercase();
        if VALUE_FLAGS.contains(&lower.as_str()) {
            i += 2; // skip the flag and its value
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
        if tok.starts_with('-') || tok.starts_with('@') || tok.is_empty() {
            i += 1;
            continue;
        }
        return true; // bare path token
    }
    false
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
        if l == "-ext" || l == "--ext" {
            return root == "view";
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
    let (dirs_only, files_only, ext_filter, readable_only): (
        bool,
        bool,
        Option<&[&str]>,
        bool,
    ) = match root.as_str() {
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
        "mkd" | "rmd" => {
            (true, false, None, false)
        }
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
            (false, false, None, false)
        }
        // `txtc`/`txtf <file>` display readable files: suppress known
        // binaries/archives (`.exe`, `.zip`, …); dirs pass for traversal.
        "txtc" | "txtf" => {
            if parent_parts.len() > 1 {
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
        // `mkf`/`rmf` take comma-separated file lists (`rmf a, b`) —
        // every slot completes files.
        "mkf" | "rmf" => {
            (false, true, None, false)
        }
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
        // General path-takers accept files and dirs.
        "size" | "report" | "explorer" | "nfe" | "mv" | "move" | "cp" | "ne" | "ntceditor" => {
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
    // Plain `*` stays literal — only the `*.` form triggers this.
    if root == "rmf" && file_part.starts_with("*.") {
        return star_ext_candidates(&base, dir_part, &file_part[2..]);
    }

    let lower = file_part.to_lowercase();
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&base) {
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
            if readable_only
                && !is_dir
                && crate::filetype::is_known_unsupported_file_name(&name)
            {
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

/// `*.<ext>` pattern candidates for `rmf` from the extensions of the
/// files inside `base`. `stub` is the text after `*.` (`""` for bare
/// `*.`). Returns full last-word replacements; exact-length matches are
/// excluded so the ghost only ever shows an untyped remainder.
fn star_ext_candidates(base: &std::path::Path, dir_part: &str, stub: &str) -> Vec<String> {
    let lower_stub = stub.to_lowercase();
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
            if ext.is_empty() || !ext.starts_with(&lower_stub) {
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
    exts.into_iter()
        .map(|e| {
            if dir_part.is_empty() {
                format!("*.{}", e)
            } else {
                format!("{}/*.{}", dir_part, e)
            }
        })
        .collect()
}

fn build_command_set() -> HashSet<String> {
    let mut cmds: HashSet<String> = builtin_commands()
        .into_iter()
        .map(|s| s.to_string())
        .collect();
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

        // The ghost is the remainder after the word already typed.
        let frag: &str = if typed.ends_with(' ') || typed.ends_with('\t') {
            ""
        } else {
            typed.split_whitespace().last().unwrap_or("")
        };
        match pick.get(frag.len()..) {
            Some(rest) if !rest.is_empty() => Some(rest.to_string()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }

    #[test]
    fn view_path_taken_skips_flags_and_values() {
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
    fn view_after_flags_still_completes_paths() {
        // Path slot still completes after flags (repo root is non-empty).
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
        // Every candidate ends with `/` (repo root always has `src/`).
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
        let c = fs_candidates_for(&["txtc"], "");
        assert!(c.iter().any(|x| x == "src/"), "got: {:?}", c);
        for x in &c {
            if x.ends_with('/') {
                continue;
            }
            assert!(
                !crate::filetype::is_known_unsupported_file_name(x),
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
        let c = fs_candidates_for(&["rmf", "a.txt,"], "");
        assert!(
            !c.iter().any(|x| x.ends_with('/')),
            "dir offered: {:?}",
            c
        );
        let c = fs_candidates_for(&["mkf", "a.txt", "b.txt,"], "");
        assert!(
            !c.iter().any(|x| x.ends_with('/')),
            "dir offered: {:?}",
            c
        );
        // `mkd`/`rmd` take comma-separated dir lists — dirs at any depth.
        let c = fs_candidates_for(&["rmd", "old,"], "");
        assert!(c.iter().any(|x| x == "src/"), "got: {:?}", c);
        assert!(c.iter().all(|x| x.ends_with('/')), "got: {:?}", c);
    }

    #[test]
    fn rmf_star_dot_completes_extensions() {
        // `rmf *.<Tab>` offers `*.<ext>` for extensions present in the dir
        // (repo root has `.toml`, `.rs`, …); a typed stub filters them.
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
        assert!(
            c2.iter().all(|x| x.starts_with("*.")),
            "got: {:?}",
            c2
        );
        assert!(c2.contains(&"*.toml".to_string()), "got: {:?}", c2);
        assert!(c2.len() <= c.len());
        // A fully-typed pattern ghosts nothing.
        assert!(fs_candidates_for(&["rmf"], "*.toml").is_empty());
    }

    #[test]
    fn chain_tail_splits_unquoted_ampersands() {
        assert_eq!(NtcHinter::chain_tail("ran reset && tl "), " tl ");
        assert_eq!(NtcHinter::chain_tail("ran reset && tl"), " tl");
        assert_eq!(NtcHinter::chain_tail("tl"), "tl");
        // Single `&` is literal, quoted `&&` is literal, unclosed
        // quotes swallow the rest — all mean "no chain".
        assert_eq!(NtcHinter::chain_tail("a & b"), "a & b");
        assert_eq!(NtcHinter::chain_tail("say \"a && b"), "say \"a && b");
        assert_eq!(
            NtcHinter::chain_tail("say \"a && b\" && tl "),
            " tl "
        );
        assert_eq!(NtcHinter::chain_tail("&&"), "");
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
        assert!(
            h.candidates_for("tl ext && scan ext r")
                .contains(&"rs".to_string())
        );
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
            assert_eq!(
                h.candidates_for("tp jump @w"),
                vec!["@work".to_string()]
            );
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
        assert_eq!(strip_help_suffix("cd to foo -h"), Some("cd to foo".to_string()));
        assert_eq!(strip_help_suffix("cd"), None);
        assert_eq!(strip_help_suffix("ignore dir foo"), None);
    }

    #[test]
    fn help_flag_is_suggested_for_dash_frag() {
        let h = NtcHinter::new();
        assert!(h.candidates_for("ignore --").contains(&"--help".to_string()));
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
        assert!(h.candidates_for("serve --a").contains(&"--auth".to_string()));
        // Fully typed flags ghost nothing (ghost is the untyped remainder).
        assert!(h.candidates_for("serve --auth").is_empty());
        assert!(h.candidates_for("serve --").contains(&"--auth".to_string()));
        // Sibling flags keep working alongside it.
        assert!(h.candidates_for("serve --t").contains(&"--tunnel".to_string()));
        // Share flags (internet-first LAN + QR).
        assert!(h.candidates_for("serve --s").contains(&"--share".to_string()));
        assert!(h.candidates_for("serve --").contains(&"--share".to_string()));
        assert!(h.candidates_for("serve --").contains(&"--qr".to_string()));
        assert!(h.candidates_for("serve --").contains(&"--no-qr".to_string()));
        // Never-implemented flags must not be suggested (`--https` used to
        // ghost here while cmd_serve rejected it as unknown).
        assert!(!h.candidates_for("serve --").contains(&"--https".to_string()));
        assert!(h.candidates_for("serve --ht").is_empty());
    }

    #[test]
    fn nsheet_flags_include_open_view() {
        let h = NtcHinter::new();
        assert!(h.candidates_for("nsheet o").contains(&"open".to_string()));
        assert!(h.candidates_for("nsheet --f").contains(&"--filter".to_string()));
        assert!(h.candidates_for("nsheet --").contains(&"--sort".to_string()));
        assert!(h.candidates_for("nsheet --").contains(&"--export".to_string()));
        assert!(h.candidates_for("nsheet --xyz").is_empty());
    }

    #[test]
    fn nssh_flags_include_subcommands() {
        let h = NtcHinter::new();
        assert!(h.candidates_for("nssh ").contains(&"ssh".to_string()));
        assert!(h.candidates_for("nssh s").contains(&"ssh".to_string()));
        assert!(h.candidates_for("nssh ssh --").contains(&"--cmd".to_string()));
        assert!(h.candidates_for("nssh ssh --c").contains(&"--cmd".to_string()));
        assert!(h.candidates_for("nssh add --h").contains(&"--host".to_string()));
        assert!(h.candidates_for("nssh scan --t").contains(&"--top".to_string()));
        assert!(h.candidates_for("nssh scan --xyz").is_empty());
    }

    #[test]
    fn image_slots_filter_to_images_plus_dirs() {
        // Every non-dir candidate must be an image file; dirs pass through
        // for `dir/<Tab>` traversal.
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
        // Repo root has non-image files (Cargo.toml) that must be suppressed,
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
    fn help_completes_canonical_topics_only() {
        let h = NtcHinter::new();
        let c = h.candidates_for("help ");
        for t in [
            "nav", "config", "image", "arc", "scan", "teleport", "nssh", "nsheet",
        ] {
            assert!(c.contains(&t.to_string()), "missing topic {}: {:?}", t, c);
        }
        // Aliases are not suggested — canonical names only.
        assert!(!c.contains(&"cd".to_string()), "got: {:?}", c);
        assert!(!c.contains(&"img".to_string()), "got: {:?}", c);
        // Prefix + case-insensitive filtering.
        assert_eq!(h.candidates_for("help im"), vec!["image".to_string()]);
        assert_eq!(h.candidates_for("help IM"), vec!["image".to_string()]);
        assert_eq!(h.candidates_for("help ar"), vec!["arc".to_string()]);
        // Topic filled: no further hints.
        assert!(h.candidates_for("help image ").is_empty());
    }
}
