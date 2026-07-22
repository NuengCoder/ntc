use colored::*;

fn section_header(title: &str) {
    println!();
    println!("{}", title.cyan().bold());
}

fn usage(syntax: &str, desc: &str) {
    println!("  {:<38} {}", syntax, desc);
}

fn examples(list: &[(&str, &str)]) {
    println!("{}", "  Examples:".green());
    for (cmd, desc) in list {
        println!("    {:<38} # {}", cmd, desc);
    }
}

pub(super) fn print_interactive_help() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                     ntc {} - Interactive Help                 ║", env!("CARGO_PKG_VERSION"));
    println!("╚══════════════════════════════════════════════════════════════════╝");

    print_nav_help();
    print_config_help();
    print_editor_help();
    print_theme_help();
    print_teleport_help();
    print_ran_help();
    print_alias_help();
    print_view_help();
    print_report_help();
    print_backup_help();
    print_search_help();
    print_igcare_help();
    print_network_help();
    print_clipboard_help();
    print_qsc_help();
    print_todo_help();
    print_other_help();
    print_syscmd_help();
    print_fileops_help();
    print_tl_help();
    print_ntpkg_help();
    print_qr_help();
    print_pathman_help();
    print_mode_help();
    print_explorer_help();
    print_dino_help();
    print_math_help();
    print_python_help();
    print_encdec_help();
    print_fmt_help();
    print_image_help();
}

fn print_nav_help() {
    section_header("NAVIGATION COMMANDS:");
    usage("cd <path>",                "Navigate to a directory");
    usage("cd to <tp_name>",          "Teleport to a saved teleport point");
    usage("gos",                      "List subdirectories and pick one");
    usage("gosc",                     "Continuous navigation (0 to exit, v/s/vs for view)");
    usage("godrive",                  "List and select Windows drives");
    usage("back [n]",                 "Go back to parent directory (or n levels)");
    usage("fgo <pattern> [-d <n>]",   "Find files, navigate to selected file's parent dir");
    usage("where",                    "Show ntc executable and config locations");
    examples(&[
        ("cd C:\\Projects",           "Navigate to absolute path"),
        ("cd to work",                "Teleport to savepoint"),
        ("back 3",                    "Go up 3 directories"),
        ("fgo main.rs -d 5",          "Find file up to 5 levels deep"),
    ]);
}

fn print_config_help() {
    section_header("CONFIGURATION COMMANDS:");
    usage("config output [path]",      "Show/set output path");
    usage("config depth [n]",          "Show/set max depth (1-20)");
    usage("config linenumbers ON|OFF", "Show/set line numbers");
    usage("config color ON|OFF",       "Show/set color output");
    usage("config threads [n]",        "Show/set thread count");
    usage("config history [on|off|path]", "Show/set history");
    usage("config autosuggest ON|OFF", "Show/set autosuggest (ghost text)");
    usage("config watch ON|OFF",       "Enable/disable file watcher");
    usage("config watch trigger <a>",  "Auto-run alias when watcher fires");
    usage("config watch trigger off",  "Disable auto-run");
    usage("config qsc \"#c 0\"",       "Set QSC global hotkey");
    usage("config qsc off",            "Disable QSC hotkey");
    usage("showcg",                    "Show current configuration overview");
    usage("opencg",                    "Open config.toml in editor");
    usage("resetcg",                   "Reset config to defaults (with backup option)");
    usage("restorecg",                 "Restore config from backup");
    usage("local init",                "Create ntconfig.toml template (commented)");
    usage("local init --all",          "Export current settings to ntconfig.toml");
    usage("local deinit",              "Remove ntconfig.toml from current directory");
    usage("local help",                "Show local command usage");
    usage("showenv",                   "Show environment variables (first 5 PATH entries)");
    usage("showenv --all",             "Show all PATH entries");
    usage("showenv -d <n>",            "Show first n PATH entries");
    examples(&[
        ("config depth 4",             "Set max tree depth to 4"),
        ("config linenumbers ON",      "Enable line numbers in file display"),
        ("config watch trigger btr",   "Auto-run 'btr' alias on file change"),
        ("local init --all",           "Export global settings to local config"),
        ("showcg",                     "View all current settings"),
    ]);
}

fn print_editor_help() {
    section_header("EDITOR COMMANDS:");
    usage("ne, ntceditor <file>",      "Open file in built-in text editor");
    usage("ne, ntceditor <file>:<n>",  "Open file at line <n>");
    usage("ne, ntceditor",             "Open welcome screen in current directory");
    usage("ne --init <file>",          "Create an empty file template");
    usage("ne --super <file>",         "Create a super file template");
    usage("ne --hyper <name> --lang <l>", "Create C/C++ hyper project");
    examples(&[
        ("ne main.rs",                 "Open main.rs in editor"),
        ("ne src/main.rs:42",          "Open main.rs at line 42"),
        ("ne --init index.html",       "Create HTML template and open it"),
        ("ne --hyper myapp --lang cpp","Create C++ hyper project scaffold"),
    ]);
}

fn print_theme_help() {
    section_header("THEME COMMANDS:");
    usage("theme",                     "Interactive theme manager (menu)");
    usage("theme list",                "List all available themes");
    usage("theme current",             "Show current theme name and details");
    usage("theme <name>",              "Switch to a theme by name");
    usage("theme add <name>",          "Create a new theme (opens in editor)");
    usage("theme rm <name>",           "Remove a theme");
    usage("theme edit [name]",         "Edit a theme (or open theme directory)");
    usage("theme where",               "Show theme directory location");
    usage("theme info <name>",         "Show theme metadata and color counts");
    usage("theme export <name>",       "Export theme to .ntc_theme file");
    usage("theme import <file>",       "Import theme from .ntc_theme file");
    usage("theme rnm <old> to <new>",  "Rename a theme");
    usage("theme reload",              "Reload themes from disk");
    usage("theme make [name]",         "Create theme template in CWD");
    examples(&[
        ("theme list",                 "Show all installed themes"),
        ("theme dracula",              "Switch to the dracula theme"),
        ("theme export mytheme",       "Export current theme to mytheme.ntc_theme"),
        ("theme import theme.ntc_theme","Import a theme file"),
    ]);
}

fn print_teleport_help() {
    section_header("TELEPORT COMMANDS:");
    usage("tp",                        "Interactive menu");
    usage("tp add <name>",             "Save current directory");
    usage("tp add <name> <path>",      "Save specific path");
    usage("tp jump <name>",            "Teleport to saved location");
    usage("tp jump <number>",          "Teleport by number");
    usage("tp list",                   "List all teleport points");
    usage("tp info <name>",            "Show savepoint details");
    usage("tp rm <name>",              "Remove teleport point");
    usage("tp rnm <old> to <new>",     "Rename a savepoint");
    usage("tp cls",                    "Clear ALL savepoints");
    usage("@<name>",                   "Quick teleport shortcut");
    usage("tpb",                       "Teleport back to previous location (undo tp)");
    usage("tpb history",               "Show teleport navigation history");
    usage("tpb clear",                 "Clear teleport history");
    examples(&[
        ("tp add work C:\\Projects\\app","Save a specific path as 'work'"),
        ("tp jump work",               "Teleport to 'work' savepoint"),
        ("@work",                      "Quick teleport shortcut"),
        ("tpb",                        "Jump back to previous location"),
        ("tpb history",                "View navigation stack"),
    ]);
}

fn print_ran_help() {
    section_header("RAN / TASK RUNNER COMMANDS:");
    usage("ran",                       "Run the 'standalone' target (default)");
    usage("ran <target>",              "Run a specific target from NTCRANFILE.toml");
    usage("ran init",                  "Create a NTCRANFILE.toml template");
    usage("ran deinit",                "Remove NTCRANFILE.toml");
    usage("ran list",                  "List all targets in NTCRANFILE.toml");
    usage("ran help",                  "Show ran usage");
    examples(&[
        ("ran build",                  "Run the 'build' target"),
        ("ran list",                   "Show all available targets"),
        ("ran init",                   "Create a NTCRANFILE.toml template"),
    ]);
}

fn print_alias_help() {
    section_header("RUN ALIAS COMMANDS:");
    usage("ral add <name> \"<cmd>\"",  "Create alias");
    usage("ral add <name>(x) \"<cmd>\"","Create parameterised alias with $x");
    usage("ral add <name>({x=a,e=txt}) \"<cmd>\"","Alias with default values");
    usage("ral add <name>(x[],y) \"<cmd>\"","Alias with array params");
    usage("ral edit <name> \"<cmd>\"",  "Update alias");
    usage("ral rnm <old> to <new>",    "Rename alias");
    usage("ral rm <name>[, ...]",      "Remove alias(es)");
    usage("ral list",                  "Show all aliases");
    usage("ral info <name>",           "Show alias details");
    usage("ral cls",                   "Clear ALL aliases (with confirmation)");
    usage("ral export --all <name>",   "Export all aliases to <name>.ntc.ral");
    usage("ral export --select <name>","Select aliases to export");
    usage("ral import <file>",         "Import aliases from .ntc.ral file");
    usage("ral make [name]",           "Create alias template in CWD");
    usage("run <alias>",               "Execute alias with 'run'");
    usage("<alias>",                   "Execute alias directly");
    usage("<alias>(<arg>)",            "Execute parameterised alias");
    examples(&[
        ("ral add btr \"cargo build --release\"",  "Create a build alias"),
        ("ral add py(x) \"python $x.py\"",         "Create param alias"),
        ("py(hello)",                              "Run: python hello.py"),
        ("ral add runc(x[],y) \"gcc $x[].c -o $y\"","Create array param alias"),
        ("runc [add,main] app",                    "Run: gcc add.c main.c -o app"),
        ("ral list",                               "Show all aliases"),
    ]);
}

fn print_view_help() {
    section_header("VIEW COMMAND:");
    usage("view",                      "Show directory tree (uses config depth)");
    usage("view -s, --size",           "Show tree with sizes");
    usage("view -c, --cp",             "Show tree and copy to clipboard");
    usage("view -d, --depth <n>",      "Show tree with custom depth");
    usage("view --care",               "Show tree with care everything");
    usage("view --time, -t",           "Show tree with execution time");
    examples(&[
        ("view -s -d 3",               "Show tree with sizes, 3 levels deep"),
        ("view --cp",                  "Show tree and copy to clipboard"),
        ("view --care --time",         "Show all files with sizes and timing"),
    ]);
}

fn print_report_help() {
    section_header("REPORT COMMANDS:");
    usage("report txt [path]",         "Generate TXT report");
    usage("report json [path]",        "Generate JSON report");
    usage("report md [path]",          "Generate Markdown report");
    usage("report html [path]",        "Generate HTML report");
    usage("report pdf [path]",         "Generate PDF report");
    usage("report docx [path]",        "Generate DOCX report");
    usage("report xlsx [path]",        "Generate XLSX report (with dependency analysis)");
    usage("report <fmt> --cp",         "Copy report to clipboard instead of saving");
    usage("txtc <file>",               "Copy file content to clipboard");
    usage("txtf <file>",               "Display file content");
    examples(&[
        ("report md ..",               "Generate Markdown report of parent dir"),
        ("report pdf --cp",            "Generate PDF and copy to clipboard"),
        ("txtf main.rs",               "Display main.rs content"),
        ("txtc Cargo.toml",            "Copy Cargo.toml content to clipboard"),
    ]);
}

fn print_backup_help() {
    section_header("BACKUP COMMANDS:");
    usage("backup create",             "Create a backup of current project");
    usage("backup list",               "List all backups");
    usage("backup restore [num]",      "Restore a backup (interactive if no num)");
    usage("backup restore --choose",   "Pick a backup from an interactive menu");
    usage("backup restore --deleted",  "Restore only files that were deleted");
    usage("backup undo",               "Undo the last restore");
    usage("backup undo --cls",         "Clear undo history (asks confirmation)");
    usage("backup undo --force",       "Clear undo history (no confirmation)");
    usage("backup clear",              "Delete ALL backups (asks confirmation)");
    usage("backup clear --force",      "Delete ALL backups (no confirmation)");
    usage("backup recreate",           "Clear all + undo + create fresh backup");
    usage("backup diff [num]",         "Diff current project against a backup");
    examples(&[
        ("backup create",              "Snapshot current project"),
        ("backup list",                "Show all saved backups"),
        ("backup restore 3",           "Restore backup #3"),
        ("backup diff 2",              "Show changes since backup #2"),
    ]);
}

fn print_search_help() {
    section_header("SEARCH COMMANDS:");
    usage("gs <pattern> [-d <n>]",     "Search file contents (simple grep)");
    usage("gs -i <pattern> [-d <n>]",  "Interactive grep (browse results, Enter to open)");
    usage("fs <pattern> [-d <n>]",     "Search files by name (fuzzy fallback)");
    usage("ds <pattern> [-d <n>]",     "Search directories by name");
    usage("locate <pattern> [-d <n>]", "Search both files and directories");
    usage("fsc <pattern> [-d <n>]",    "Search files, pick one to display & optionally copy");
    examples(&[
        ("gs \"fn main\" -d 5",        "Search content 'fn main' up to 5 levels deep"),
        ("gs -i \"TODO\"",             "Interactive grep - browse and open results"),
        ("fs *.rs",                    "Find all .rs files"),
        ("ds src -d 3",                "Find all dirs named 'src' up to 3 deep"),
        ("locate config -d 10",        "Find all files and dirs named 'config'"),
    ]);
}

fn print_igcare_help() {
    section_header("IGNORE / CARE COMMANDS:");
    usage("ignore dir <name>[, ...]",  "Ignore directories by name");
    usage("ignore ext <ext>[, ...]",   "Ignore file extensions");
    usage("ignore file <name>[, ...]", "Ignore specific files");
    usage("care dir <name>[, ...]",    "Stop ignoring directories");
    usage("care ext <ext>[, ...]",     "Add extra supported extensions");
    usage("care file <name>[, ...]",   "Add extra supported files");
    usage("igcared",                   "Show all ignore/care settings");
    usage("igcare export --all <n>",   "Export all settings to <name>.ntc.igcare");
    usage("igcare export --select <n>","Select categories to export");
    usage("igcare import <file>",      "Import settings from .ntc.igcare file");
    usage("igcare make [name]",        "Create igcare template in CWD");
    examples(&[
        ("ignore dir target,node_modules","Ignore build and node dirs"),
        ("ignore ext log,tmp",           "Ignore .log and .tmp files"),
        ("care ext py",                  "Treat .py files as supported"),
        ("igcared",                      "Show current ignore/care state"),
    ]);
}

fn print_network_help() {
    section_header("SYSTEM & NETWORK COMMANDS:");
    usage("sys, top",                  "Live system monitor (CPU, memory, disk, processes)");
    usage("http <method> <url>",       "HTTP API client (-H, -d, --json, -q, -a, --bearer, -s, -o, -v, -t)");
    usage("dl <url>",                  "Download file (--progress, --resume, --stop, --cancel)");
    usage("up <file> <url>",           "Upload file (--post, -H, -d)");
    usage("ping <host>",               "ICMP ping (-c count)");
    usage("dns <host>",                "DNS lookup (-t record type: A, AAAA, ALL, MX, NS, etc)");
    usage("smtp",                      "SMTP email client (--id, -i, -u, -pwd, -s, -b, -f, --to)");
    usage("serve [port]",              "HTTP file server (--https, --tunnel, --spa, --web-dir)");
    examples(&[
        ("sys",                        "Launch live system monitor"),
        ("http GET https://api.example.com/users -H \"Authorization: Bearer xxx\"",  "API GET request"),
        ("http POST https://api.example.com/users -d '{\"name\":\"John\"}' --json",   "API POST with JSON body"),
        ("dl https://example.com/file.zip --progress",  "Download with progress bar"),
        ("serve 8080 --https",         "Start HTTPS file server on port 8080"),
        ("dns example.com -t ALL",     "Lookup all DNS records"),
    ]);
}

fn print_clipboard_help() {
    section_header("CLIPBOARD COMMANDS:");
    usage("clip",                      "Show clipboard history");
    usage("clip list",                 "Show clipboard history");
    usage("clip select <N>",           "Copy entry N back to clipboard");
    usage("clip clear",                "Clear clipboard history");
    usage("pathcp",                    "Copy current directory path to clipboard");
    examples(&[
        ("clip",                       "View clipboard history"),
        ("clip select 2",              "Restore entry #2 to clipboard"),
        ("pathcp",                     "Copy current dir path to clipboard"),
    ]);
}

fn print_qsc_help() {
    section_header("QSC (GLOBAL HOTKEY) COMMANDS:");
    usage("config qsc \"#c 0\"",       "Set hotkey (Ctrl+0)");
    usage("config qsc \"#c #s a\"",    "Set hotkey (Ctrl+Shift+A)");
    usage("config qsc off",            "Disable QSC hotkey");
    usage("qsc listen",                "Start QSC daemon (background process)");
    usage("qsc stop",                  "Stop QSC daemon");
    usage("qsc status",                "Check if QSC daemon is running");
    usage("qsc startup on|off",        "Register/unregister as startup app");
    examples(&[
        ("config qsc \"#c 0\"",        "Set global hotkey to Ctrl+0"),
        ("qsc listen",                 "Start the hotkey listener daemon"),
        ("qsc status",                 "Check if listener is active"),
    ]);
}

fn print_todo_help() {
    section_header("TODO COMMANDS:");
    usage("todo",                      "Show all tasks (from future_todo.txt in CWD)");
    usage("todo info [N]",             "Show all tasks or details of task N");
    usage("todo add \"name\"",          "Append a new task");
    usage("todo add \"name\" -> N",     "Insert task at position N");
    usage("todo rm <N>",               "Remove task N");
    usage("todo clear",                "Clear ALL tasks (with confirmation)");
    examples(&[
        ("todo add \"fix login bug\"",  "Add a new task"),
        ("todo rm 3",                  "Remove task #3"),
        ("todo info",                  "List all tasks"),
    ]);
}

fn print_other_help() {
    section_header("OTHER COMMANDS:");
    usage("init [--ran|--local]",      "Initialize project files (NTCRANFILE.toml + ntconfig.toml)");
    usage("deinit [--ran|--local]",    "Remove project files");
    usage("clear",                     "Clear screen");
    usage("version",                   "Show version");
    usage("help",                      "Show this help");
    usage("say <text> [--loop N] [-n|-t]","Print text (no file redirect)");
    usage("print <text> [--loop N] [-n|-t]","Alias for say");
    usage("tutorial",                  "Start the interactive step-by-step tutorial");
    usage("ui",                        "Show current UI mode");
    usage("exit, quit",                "Exit ntc");
    examples(&[
        ("say \"Hello, World!\"",      "Print a message"),
        ("say \"Hi\" --loop 3 -t",     "Print 'Hi Hi Hi' on one line"),
        ("tutorial",                   "Start the interactive tutorial"),
        ("init --all",                 "Create both ranfile and local config"),
    ]);
}

fn print_syscmd_help() {
    section_header("SYSTEM COMMANDS:");
    usage("<any command>",             "NTC passes unknown commands to your system");
    usage("",                          "shell — echo, git, dir/ls, ping, etc.");
    usage("",                          "all run natively alongside NTC commands.");
}

fn print_fileops_help() {
    section_header("FILE OPERATIONS:");
    usage("mkf <file>[, ...]",         "Create a new file");
    usage("mkd <dir>[, ...]",          "Create a new directory");
    usage("rmf <file>[, ...]",         "Delete a file (with confirmation)");
    usage("rmd <dir>[, ...]",          "Delete a directory (with confirmation)");
    usage("rnmf <old> <new>",          "Rename a file");
    usage("rnmd <old> <new>",          "Rename a directory");
    usage("cp <src> <dst>",            "Copy a file");
    examples(&[
        ("mkf index.html style.css",   "Create two new files"),
        ("mkd src/components",         "Create nested directory"),
        ("rnmf old.txt new.txt",       "Rename a file"),
    ]);
}

fn print_tl_help() {
    section_header("TOTAL LINE COMMANDS:");
    usage("tl file <path>",            "Count lines in a single file");
    usage("tl ext <ext>",              "Show all <ext> files and their total lines (respects igcare)");
    usage("tl dir <path>",             "Show supported extensions and total lines in directory");
    examples(&[
        ("tl file main.rs",            "Count lines in main.rs"),
        ("tl ext rs",                  "Count all .rs files in project"),
        ("tl dir .",                   "Count all supported files in current dir"),
    ]);
}

fn print_ntpkg_help() {
    section_header("PACKAGE MANAGER (ntpkg) + ntpx:");
    usage("ntpkg install <lang> [ver]",    "Download & install a runtime");
    usage("ntpkg uninstall <lang> [ver]",  "Remove an installed runtime");
    usage("ntpkg list",                    "Show installed runtimes");
    usage("ntpkg use <lang> <ver>",        "Activate a specific version");
    usage("ntpkg which <lang>",            "Show path to active runtime");
    usage("ntpkg search [keyword]",        "Search available packages");
    usage("ntpkg info <lang>",             "Show package info (version, location, binaries)");
    usage("ntpkg available [lang]",        "List languages or downloadable versions");
    usage("ntpkg help",                    "Show ntpkg usage");
    usage("ntpx <pkg> [bin] [args]",       "Run an installed package's binary");
    examples(&[
        ("ntpkg install python 3.12",      "Install Python 3.12"),
        ("ntpkg install java 21",          "Install Java JDK 21"),
        ("ntpkg list",                     "Show installed runtimes"),
        ("ntpkg search py",                "Search for packages matching 'py'"),
        ("ntpkg info python",              "Show details about installed Python"),
        ("ntpkg use python 3.12",          "Activate Python 3.12"),
        ("ntpkg which python",             "Show active Python path"),
        ("ntpx go version",                "Run go version"),
        ("ntpx msys2 gcc --version",       "Run gcc --version from MSYS2"),
    ]);
}

fn print_qr_help() {
    section_header("QR CODE COMMANDS:");
    usage("qr <text>",                "Encode text and display QR in terminal");
    usage("qr encode <text> -o <file>","Save QR as PNG image");
    usage("qr decode <image>",        "Decode QR code from image");
    examples(&[
        ("qr \"hello world\"",        "Generate and display a QR code"),
        ("qr encode \"hello\" -o qr.png","Save QR code to PNG file"),
        ("qr decode qr.png",          "Decode a QR code image"),
    ]);
}

fn print_pathman_help() {
    section_header("PATH MANAGER (Windows):");
    usage("usrpath show",                "Show user-level PATH entries");
    usage("usrpath add <path>",          "Add directory to user PATH");
    usage("usrpath add --cwd | --here",  "Add current directory to user PATH");
    usage("usrpath rm <path-or-number>", "Remove from user PATH (by path or index)");
    usage("usrpath move <n> <m>",        "Move entry #n to position #m");
    usage("syspath show",                "Show system-level PATH entries");
    usage("syspath add <path>",          "Add directory to system PATH");
    usage("syspath add --cwd | --here",  "Add current directory to system PATH");
    usage("syspath rm <path-or-number>", "Remove from system PATH (by path or index)");
    usage("syspath move <n> <m>",        "Move entry #n to position #m");
    examples(&[
        ("usrpath show",                 "Show user PATH entries"),
        ("usrpath add C:\\Tools",        "Add C:\\Tools to user PATH"),
        ("usrpath add --cwd",            "Add current directory to user PATH"),
        ("usrpath rm 3",                 "Remove entry #3 from user PATH"),
        ("usrpath move 3 1",             "Move entry #3 to the top"),
        ("syspath show",                 "Show system PATH entries (requires admin)"),
    ]);
}

fn print_mode_help() {
    section_header("WINDOWS MODE (Windows):");
    usage("mode admin",                  "Exit and re-launch ntc as Administrator");
    usage("mode normal",                 "Exit and re-launch ntc without elevation");
    examples(&[
        ("mode admin",                   "Re-launch with admin rights (UAC prompt)"),
    ]);
}

fn print_explorer_help() {
    section_header("FILE EXPLORER (NFE):");
    usage("explorer, nfe [PATH]",     "Launch the full-screen terminal file explorer");
    usage("",                          "h=help, l=go to line, b=parent dir, f=find");
    usage("",                          "/=filter, e=editor, r=rename, d=delete");
    usage("",                          "n=new file, m=new dir, i=hidden toggle");
    usage("",                          "s/sort toggle, S=sort reverse, F5=refresh");
    usage("",                          "q/Esc=quit, mouse=click/scroll");
    examples(&[
        ("explorer",                   "Open explorer in current directory"),
        ("nfe C:\\Projects",           "Open explorer in specified directory"),
    ]);
}

fn print_dino_help() {
    section_header("DINOSAUR GAME:");
    usage("dino",                     "Play Chrome-style dinosaur runner game");
    usage("",                         "Space/Up to jump, avoid obstacles");
}

fn print_math_help() {
    section_header("MATH COMMANDS:");
    usage("math <expr>",              "Evaluate a math expression");
    usage("math timer [sec]",         "Lap timer or countdown with alarm");
    usage("math fun add/rm/edit/info/ls","Manage custom math functions");
    usage("math file.ntc.math",       "Compile and run a math file");
    examples(&[
        ("math 3+4*5",                "Evaluates to 23"),
        ("math sin(PI/2)",            "Evaluates to 1"),
        ("math rand(1,100)",          "Random number between 1 and 100"),
        ("math toHex(255)",           "Convert 255 to FF"),
        ("math fun add sq(x) = x^2",  "Define custom function"),
        ("math sq(5)",                "Use custom function: 25"),
    ]);
}

fn print_python_help() {
    section_header("PYTHON COMMANDS:");
    usage("py_activate",              "Activate Python virtual environment (auto-detect)");
    usage("py_activate <path>",       "Activate specific venv path");
    usage("py_deactivate",            "Deactivate current virtual environment");
    examples(&[
        ("py_activate",               "Auto-detect and activate .venv or venv"),
        ("py_activate .venv",         "Activate .venv in current dir"),
        ("py_deactivate",             "Exit current virtual environment"),
    ]);
}

fn print_encdec_help() {
    section_header("ENCODE/DECODE COMMANDS:");
    usage("encode b2 <target> [-o <file>]","Binary encode (0101...)");
    usage("encode b64 <target> [-o <file>]","Base64 encode (file or raw text)");
    usage("encode hex <target> [-o <file>]","Hex encode");
    usage("encode url <target> [-o <file>]","URL percent-encode");
    usage("decode b2 <target> [-o <file>]","Binary decode");
    usage("decode b64 <target> [-o <file>]","Base64 decode");
    usage("decode hex <target> [-o <file>]","Hex decode");
    usage("decode url <target> [-o <file>]","URL percent-decode");
    usage("cvcode <from> <data> to <to>","Convert between formats");
    examples(&[
        ("encode b64 \"hello world\"",  "Base64 encode text"),
        ("encode hex \"hello\" -o out.txt","Save hex output to file"),
        ("decode b64 aGVsbG8=",         "Base64 decode"),
        ("cvcode b64 aGVsbG8= to hex",  "Convert base64 to hex"),
    ]);
}

fn print_fmt_help() {
    section_header("FORMAT CONVERTER:");
    usage("fmt json <file> [to yaml|toml]","Convert JSON to YAML or TOML");
    usage("fmt yaml <file> [to json|toml]","Convert YAML to JSON or TOML");
    usage("fmt toml <file> [to json|yaml]","Convert TOML to JSON or YAML");
    usage("(add -o <file> to save output)","");
    examples(&[
        ("fmt json data.json to yaml -o data.yaml","Convert JSON to YAML"),
        ("fmt toml Cargo.toml to json",            "View Cargo.toml as JSON"),
    ]);
}

fn print_image_help() {
    section_header("IMAGE COMMANDS:");
    usage("image",                     "Interactive image converter (pick from current dir)");
    usage("image <path>",              "Open image in default viewer");
    usage("image <path> to <fmt>",     "Convert and display output (json|md|html|txt|desc)");
    usage("image <path> out <fmt>",    "Convert and save to setO path");
    usage("image <path> --ocr",        "Enable OCR text extraction");
    usage("image <path> --layout",     "Enable UI layout detection");
    examples(&[
        ("image screenshot.png to json","Convert screenshot to AI-ready JSON"),
        ("image photo.jpg --ocr",      "Extract text from image"),
    ]);
}

pub(super) fn print_tp_help() {
    print_teleport_help();
}
