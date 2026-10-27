use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cli.completions.shell.bash".into(), "bash".into());

    m.insert("cli.completions.shell.fish".into(), "fish".into());

    m.insert(
        "cli.completions.shell.powershell".into(),
        "powershell".into(),
    );

    m.insert("cli.completions.shell.zsh".into(), "zsh".into());

    m.insert(
        "cli.example.cat_file".into(),
        " ntc -i file.txt Display file contents".into(),
    );

    m.insert(
        "cli.example.image".into(),
        " ntc -image screenshot.png to json Convert image to JSON for AI\n".into(),
    );

    m.insert(
        "cli.example.interactive".into(),
        " ntc Launch interactive shell".into(),
    );

    m.insert(
        "cli.example.math_arithmetic".into(),
        " ntc --math \"3+4*5\" Evaluate math expression".into(),
    );

    m.insert(
        "cli.example.math_func".into(),
        " ntc --math \"sin(PI/2)\" Math with functions and constants".into(),
    );

    m.insert(
        "cli.example.report_cp".into(),
        " ntc -i src --cp Copy directory tree to clipboard".into(),
    );

    m.insert(
        "cli.example.report_docx".into(),
        " ntc -i src -f docx Generate DOCX report from src".into(),
    );

    m.insert(
        "cli.example.report_json_cp".into(),
        " ntc -i src -f json --cp Copy JSON report to clipboard".into(),
    );

    m.insert(
        "cli.example.report_pdf".into(),
        " ntc -i src -f pdf Generate PDF report from src".into(),
    );

    m.insert(
        "cli.example.report_txt".into(),
        " ntc -i src Generate TXT report of src directory".into(),
    );

    m.insert(
        "cli.example.report_xlsx".into(),
        " ntc -i src -f xlsx Generate XLSX report (with dependency analysis)".into(),
    );

    m.insert(
        "cli.example.say".into(),
        " ntc -say \"Hello World\" Print Hello World".into(),
    );

    m.insert(
        "cli.example.setL".into(),
        " ntc --setL ON Enable line numbers".into(),
    );

    m.insert(
        "cli.example.showcg".into(),
        " ntc --showcg Show configuration".into(),
    );

    m.insert(
        "cli.example.teleport_at".into(),
        " ntc @web Launch and teleport to 'web' savepoint".into(),
    );

    m.insert(
        "cli.example.watch".into(),
        " ntc --watch ON Enable file watcher".into(),
    );

    m.insert(
        "cli.flag.cared".into(),
        " --cared <name> Stop ignoring a directory name".into(),
    );

    m.insert(
        "cli.flag.caref".into(),
        " --caref <ext> Care about a file extension".into(),
    );

    m.insert(
        "cli.flag.caren".into(),
        " --caren <file> Care about a specific file\n".into(),
    );

    m.insert(
        "cli.flag.clear".into(),
        " --clear Clear the terminal screen".into(),
    );

    m.insert(
        "cli.flag.copy".into(),
        " --cp Copy report to clipboard instead of saving to file".into(),
    );

    m.insert(
        "cli.flag.edit".into(),
        " -e, --edit <file> Open file in built-in text editor".into(),
    );

    m.insert(
        "cli.flag.explorer".into(),
        " -x, --explorer [PATH] Launch NTC File Explorer TUI".into(),
    );

    m.insert(
        "cli.flag.format".into(),
        " -f, --format <FORMAT> Output format: txt, html, json, md, pdf, docx, xlsx (default: txt)"
            .into(),
    );

    m.insert(
        "cli.flag.generate_completions".into(),
        " --generate-completions <SHELL> Generate completions (bash|zsh|fish|powershell)\n".into(),
    );

    m.insert(
        "cli.flag.help_flag".into(),
        " --help Show this help message".into(),
    );

    m.insert(
        "cli.flag.hyper".into(),
        " --hyper <name> --lang c Create C/C++ hyper project scaffold\n".into(),
    );

    m.insert(
        "cli.flag.igcare_export_all".into(),
        " --igcare-export-all <name> Export all ignore/care settings".into(),
    );

    m.insert(
        "cli.flag.igcare_export_select".into(),
        " --igcare-export-select <name> Select categories to export".into(),
    );

    m.insert(
        "cli.flag.igcare_import".into(),
        " --igcare-import <file> Import settings from .ntc.igcare file\n".into(),
    );

    m.insert(
        "cli.flag.ignore".into(),
        " --ignore <name> Ignore a directory name".into(),
    );

    m.insert(
        "cli.flag.ignored".into(),
        " --ignored Show currently ignored items".into(),
    );

    m.insert(
        "cli.flag.ignoref".into(),
        " --ignoref <ext> Ignore a file extension".into(),
    );

    m.insert(
        "cli.flag.ignoren".into(),
        " --ignoren <file> Ignore a specific file (e.g., Cargo.lock)".into(),
    );

    m.insert(
        "cli.flag.image".into(),
        " -image <path> to <fmt> Convert image to document for AI (json|md|html|txt)".into(),
    );

    m.insert(
        "cli.flag.init".into(),
        " --init <file> Create starter template and open in editor".into(),
    );

    m.insert(
        "cli.flag.input".into(),
        " -i, --input <path> Process a file or directory".into(),
    );

    m.insert(
        "cli.flag.list".into(),
        " --list, --fun List all command-line functions".into(),
    );

    m.insert(
        "cli.flag.math".into(),
        " --math <EXPR> Evaluate a math expression (e.g. --math \"3+4*5\")".into(),
    );

    m.insert(
        "cli.flag.output".into(),
        " -o, --output <file> Save output to specified file".into(),
    );

    m.insert(
        "cli.flag.ral_export_all".into(),
        " --ral-export-all <name> Export all run aliases to <name>.ntc.ral".into(),
    );

    m.insert(
        "cli.flag.ral_export_select".into(),
        " --ral-export-select <name> Select aliases to export".into(),
    );

    m.insert(
        "cli.flag.ral_import".into(),
        " --ral-import <file> Import aliases from .ntc.ral file".into(),
    );

    m.insert(
        "cli.flag.say".into(),
        " -say, -print <text> Print text to stdout".into(),
    );

    m.insert(
        "cli.flag.setA".into(),
        " --setA [ON|OFF] Show/set autosuggest (ghost text)".into(),
    );

    m.insert(
        "cli.flag.setC".into(),
        " --setC [ON|OFF] Show or toggle color output".into(),
    );

    m.insert(
        "cli.flag.setD".into(),
        " --setD [depth] Show or set max recursion depth (min: 1, max: 20)".into(),
    );

    m.insert(
        "cli.flag.setH".into(),
        " --setH [path|ON|OFF] Show/set history path or enable/disable".into(),
    );

    m.insert(
        "cli.flag.setL".into(),
        " --setL [ON|OFF] Show or toggle line numbers for file display".into(),
    );

    m.insert(
        "cli.flag.setO".into(),
        " --setO [path] Show or set the output directory (default: Desktop)".into(),
    );

    m.insert(
        "cli.flag.setT".into(),
        " --setT [threads] Show or set number of threads (default: 4)".into(),
    );

    m.insert(
        "cli.flag.showcg".into(),
        " --showcg Show current configuration overview".into(),
    );

    m.insert(
        "cli.flag.size".into(),
        " --size Show current directory size".into(),
    );

    m.insert(
        "cli.flag.tp_add".into(),
        " --tp-add <name> Save current directory as teleport point".into(),
    );

    m.insert(
        "cli.flag.tp_list".into(),
        " --tp-list List all teleport points".into(),
    );

    m.insert(
        "cli.flag.tp_rm".into(),
        " --tp-rm <name> Remove a teleport point\n".into(),
    );

    m.insert(
        "cli.flag.version".into(),
        " --version Show version information".into(),
    );

    m.insert(
        "cli.flag.view".into(),
        " --view Quick view of current directory tree".into(),
    );

    m.insert(
        "cli.flag.view_size".into(),
        " --view --size Quick view with directory sizes\n".into(),
    );

    m.insert(
        "cli.flag.watch_flag".into(),
        " --watch [ON|OFF] Show/set file watcher state".into(),
    );

    m.insert(
        "cli.flag.where".into(),
        " --where Show ntc executable and config location\n".into(),
    );

    m.insert(
        "cli.help.description".into(),
        "A combine of Navigation, Project Toolkit Center hub.\n".into(),
    );

    m.insert(
        "cli.help.title".into(),
        "ntc <version> \u{2014} Navigate, Toolkit, Center".into(),
    );

    m.insert("cli.helpers.basic_usage".into(), " ntc [OPTIONS]\n ntc -i <path> [-o <output>]\n ntc -i <path> --cp Copy report to clipboard\n".into());

    m.insert("cli.helpers.cli_examples".into(), " ntc Launch interactive shell\n ntc @web Launch and teleport to 'web' savepoint\n ntc --math \"3+4*5\" Evaluate math expression\n ntc --math \"sin(PI/2)\" Math with functions and constants\n ntc -i src Generate TXT report of src directory\n ntc -i src --cp Copy directory tree to clipboard\n ntc -i src -f json --cp Copy JSON report to clipboard\n ntc -i src -f pdf Generate PDF report from src\n ntc -i src -f docx Generate DOCX report from src\n ntc -i src -f xlsx Generate XLSX report (with dependency analysis)\n ntc -i file.txt Display file contents\n ntc --setL ON Enable line numbers\n ntc --showcg Show configuration\n ntc -say \"Hello World\" Print Hello World\n ntc --watch ON Enable file watcher\n ntc -image screenshot.png to json Convert image to JSON for AI\n".into());

    m.insert(
        "cli.helpers.config_header".into(),
        "CONFIGURATION OPTIONS:".into(),
    );

    m.insert("cli.helpers.config_options".into(), " --setO [path] Show or set the output directory (default: Desktop)\n --setD [depth] Show or set max recursion depth (min: 1, max: 20)\n --setL [ON|OFF] Show or toggle line numbers for file display\n --setC [ON|OFF] Show or toggle color output\n --setT [threads] Show or set number of threads (default: 4)\n --setH [path|ON|OFF] Show/set history path or enable/disable\n --setA [ON|OFF] Show/set autosuggest (ghost text)\n --showcg Show current configuration overview\n --watch [ON|OFF] Show/set file watcher state\n --where Show ntc executable and config location\n".into());

    m.insert("cli.helpers.editor_header".into(), "EDITOR OPTIONS:".into());

    m.insert("cli.helpers.editor_options".into(), " -e, --edit <file> Open file in built-in text editor\n --init <file> Create starter template and open in editor\n --hyper <name> --lang c Create C/C++ hyper project scaffold\n".into());

    m.insert("cli.helpers.examples_header".into(), "EXAMPLES:".into());

    m.insert(
        "cli.helpers.export_header".into(),
        "ALIAS / IGNORE/CARE EXPORT:".into(),
    );

    m.insert("cli.helpers.export_options".into(), " --ral-export-all <name> Export all run aliases to <name>.ntc.ral\n --ral-export-select <name> Select aliases to export\n --ral-import <file> Import aliases from .ntc.ral file\n --igcare-export-all <name> Export all ignore/care settings\n --igcare-export-select <name> Select categories to export\n --igcare-import <file> Import settings from .ntc.igcare file\n".into());

    m.insert("cli.helpers.igcare_options".into(), " --ignored Show currently ignored items\n --ignore <name> Ignore a directory name\n --cared <name> Stop ignoring a directory name\n --ignoref <ext> Ignore a file extension\n --caref <ext> Care about a file extension\n --ignoren <file> Ignore a specific file (e.g., Cargo.lock)\n --caren <file> Care about a specific file\n".into());

    m.insert(
        "cli.helpers.ignore_header".into(),
        "IGNORE/CARE OPTIONS:".into(),
    );

    m.insert("cli.helpers.interactive_note".into(), "INTERACTIVE-ONLY (run 'ntc' with no args):\n Navigation: cd, back, gos, gosc\n Teleport: tp jump, tp to, @name\n Tutorial: tutorial \u{2014} Start interactive step-by-step tutorial\n".into());

    m.insert("cli.helpers.other_header".into(), "OTHER OPTIONS:".into());

    m.insert("cli.helpers.other_options".into(), " -say, -print <text> Print text to stdout\n -image <path> to <fmt> Convert image to document for AI (json|md|html|txt)\n -x, --explorer [PATH] Launch NTC File Explorer TUI\n --clear Clear the terminal screen\n --version Show version information\n --math <EXPR> Evaluate a math expression (e.g. --math \"3+4*5\")\n --list, --fun List all command-line functions\n --help Show this help message\n --generate-completions <SHELL> Generate completions (bash|zsh|fish|powershell)\n".into());

    m.insert("cli.helpers.report_header".into(), "REPORT OPTIONS:".into());

    m.insert("cli.helpers.report_options".into(), " -i, --input <path> Process a file or directory\n -o, --output <file> Save output to specified file\n -f, --format <FORMAT> Output format: txt, html, json, md, pdf, docx, xlsx (default: txt)\n --cp Copy report to clipboard instead of saving to file\n --size Show current directory size\n --view Quick view of current directory tree\n --view --size Quick view with directory sizes\n".into());

    m.insert("cli.helpers.shell_commands".into(), " ntc pathcp Copy path to clipboard\n ntc say \"hello\" --loop 3 Print text with loop\n ntc tl dir . Count lines in directory\n ntc ne Open text editor\n ntc size Show directory size\n ntc view Show directory tree\n ntc serve files --port 8080 Start files drive server on port 8080\n ntc dl https://example.com/file.zip --progress Download with progress\n ntc encode b64 hello Base64 encode a string\n ntc qr \"text\" Generate QR code in terminal\n ntc math 3+4*5 Evaluate expression\n ntc ral add btr \"cargo build --release\" Add alias from CLI\n".into());

    m.insert(
        "cli.helpers.shell_header".into(),
        "SHELL COMMANDS (also work from CLI):".into(),
    );

    m.insert(
        "cli.helpers.tagline".into(),
        "A combine of Navigation, Project Toolkit Center hub.\n".into(),
    );

    m.insert(
        "cli.helpers.tp_header".into(),
        "TELEPORT SAVE POINTS:".into(),
    );

    m.insert("cli.helpers.tp_options".into(), " --tp-add <name> Save current directory as teleport point\n --tp-list List all teleport points\n --tp-rm <name> Remove a teleport point\n".into());

    m.insert("cli.helpers.usage_header".into(), "USAGE:".into());

    m.insert("cli.new.usage_header".into(), "Usage:".into());

    m.insert(
        "cli.new.usage_shell".into(),
        "  ntc                       Launch interactive shell".into(),
    );

    m.insert(
        "cli.new.usage_cmd".into(),
        "  ntc <command> [args...]   Run a shell command non-interactively".into(),
    );

    m.insert(
        "cli.new.usage_new".into(),
        "  ntc -n, --new [command]   Open a new terminal (run command there, stay interactive)"
            .into(),
    );

    m.insert(
        "cli.new.usage_upgrade".into(),
        "  ntc --upgrade             Upgrade ntc to the latest version".into(),
    );

    m.insert(
        "cli.new.usage_version".into(),
        "  ntc --version, -v         Show version information".into(),
    );

    m.insert(
        "cli.new.usage_help".into(),
        "  ntc --help, -h            Show this help".into(),
    );

    m.insert("cli.new.examples_header".into(), "Examples:".into());

    m.insert(
        "cli.new.ex_help".into(),
        "  ntc help                  Show shell help".into(),
    );

    m.insert(
        "cli.new.ex_version".into(),
        "  ntc version               Show version via shell".into(),
    );

    m.insert(
        "cli.new.ex_alias".into(),
        "  ntc t                     Run ral alias 't' (e.g. taskmgr)".into(),
    );

    m.insert(
        "cli.new.ex_new".into(),
        "  ntc -n                    Open a new interactive terminal".into(),
    );

    m.insert(
        "cli.new.ex_new_cmd".into(),
        "  ntc -n t                  Open a new terminal, run 't', stay interactive".into(),
    );

    m.insert("cli.new.opened".into(), "Opened new terminal.".into());

    m.insert(
        "cli.new.no_terminal".into(),
        "no terminal found (tried: {})".into(),
    );

    m.insert(
        "cli.new.unsupported".into(),
        "ntc -n/--new is only supported on Windows, Linux and macOS".into(),
    );

    m.insert(
        "cli.interactive.navigation".into(),
        " Navigation: cd, back, gos, gosc".into(),
    );

    m.insert(
        "cli.interactive.teleport".into(),
        " Teleport: tp jump, tp to, @name".into(),
    );

    m.insert(
        "cli.interactive.tutorial".into(),
        " Tutorial: tutorial \u{2014} Start interactive step-by-step tutorial\n".into(),
    );

    m.insert(
        "cli.known_flag.cared".into(),
        "--cared <name> Stop ignoring a directory".into(),
    );

    m.insert(
        "cli.known_flag.caref".into(),
        "--caref <ext> Care about a file extension".into(),
    );

    m.insert(
        "cli.known_flag.caren".into(),
        "--caren <file> Care about a specific file".into(),
    );

    m.insert(
        "cli.known_flag.clear".into(),
        "--clear Clear the terminal screen".into(),
    );

    m.insert(
        "cli.known_flag.cp".into(),
        "--cp Copy report to clipboard".into(),
    );

    m.insert(
        "cli.known_flag.explorer".into(),
        "-x, --explorer [PATH] Launch NTC File Explorer TUI".into(),
    );

    m.insert(
        "cli.known_flag.generate_completions".into(),
        "--generate-completions <SHELL> Generate shell completions (bash|zsh|fish|powershell)"
            .into(),
    );

    m.insert("cli.known_flag.help_flag".into(), "--help Show help".into());

    m.insert(
        "cli.known_flag.igcare_export_all".into(),
        "--igcare-export-all <name> Export all ignore/care settings to <name>.ntc.igcare".into(),
    );

    m.insert(
        "cli.known_flag.igcare_export_select".into(),
        "--igcare-export-select <name> Select categories to export to <name>.ntc.igcare".into(),
    );

    m.insert(
        "cli.known_flag.igcare_import".into(),
        "--igcare-import <file> Import ignore/care settings from .ntc.igcare file".into(),
    );

    m.insert(
        "cli.known_flag.ignore".into(),
        "--ignore <name> Ignore a directory name".into(),
    );

    m.insert(
        "cli.known_flag.ignored".into(),
        "--ignored Show ignored items".into(),
    );

    m.insert(
        "cli.known_flag.ignoref".into(),
        "--ignoref <ext> Ignore a file extension".into(),
    );

    m.insert(
        "cli.known_flag.ignoren".into(),
        "--ignoren <file> Ignore a specific file".into(),
    );

    m.insert(
        "cli.known_flag.input".into(),
        "-i, --input <path> Input file or directory".into(),
    );

    m.insert(
        "cli.known_flag.list".into(),
        "--list, --fun List all command-line functions".into(),
    );

    m.insert(
        "cli.known_flag.math".into(),
        "--math <EXPR> Evaluate a math expression (e.g. --math \"3+4*5\")".into(),
    );

    m.insert(
        "cli.known_flag.no_args".into(),
        "(no args) Launch interactive mode".into(),
    );

    m.insert(
        "cli.known_flag.output".into(),
        "-o, --output <file> Output filename".into(),
    );

    m.insert(
        "cli.known_flag.ral_export_all".into(),
        "--ral-export-all <name> Export all run aliases to <name>.ntc.ral".into(),
    );

    m.insert(
        "cli.known_flag.ral_export_select".into(),
        "--ral-export-select <name> Select run aliases to export to <name>.ntc.ral".into(),
    );

    m.insert(
        "cli.known_flag.ral_import".into(),
        "--ral-import <file> Import run aliases from .ntc.ral file".into(),
    );

    m.insert(
        "cli.known_flag.say".into(),
        "-say, -print <text> Print text to stdout".into(),
    );

    m.insert(
        "cli.known_flag.setC".into(),
        "--setC [ON|OFF] Show or set color output".into(),
    );

    m.insert(
        "cli.known_flag.setD".into(),
        "--setD [depth] Show or set max depth (1-20)".into(),
    );

    m.insert(
        "cli.known_flag.setH".into(),
        "--setH [path|ON|OFF] Show or set history path/state".into(),
    );

    m.insert(
        "cli.known_flag.setL".into(),
        "--setL [ON|OFF] Show or set line numbers".into(),
    );

    m.insert(
        "cli.known_flag.setO".into(),
        "--setO [path] Show or set output directory".into(),
    );

    m.insert(
        "cli.known_flag.setT".into(),
        "--setT [threads] Show or set thread count".into(),
    );

    m.insert(
        "cli.known_flag.showcg".into(),
        "--showcg Show current configuration overview".into(),
    );

    m.insert(
        "cli.known_flag.size".into(),
        "--size Show current directory size".into(),
    );

    m.insert(
        "cli.known_flag.path".into(),
        "--path <path|@tp> Target path or teleport savepoint (use with --size or --view)".into(),
    );

    m.insert(
        "cli.known_flag.teleport_at".into(),
        "ntc @<name> Launch and teleport to savepoint".into(),
    );

    m.insert(
        "cli.known_flag.tp_add".into(),
        "--tp-add <name> Save current directory as teleport point".into(),
    );

    m.insert(
        "cli.known_flag.tp_list".into(),
        "--tp-list List all teleport points".into(),
    );

    m.insert(
        "cli.known_flag.tp_rm".into(),
        "--tp-rm <name> Remove teleport point".into(),
    );

    m.insert(
        "cli.known_flag.upgrade".into(),
        "--upgrade Check for updates and upgrade ntc".into(),
    );

    m.insert(
        "cli.known_flag.version".into(),
        "--version Show version information".into(),
    );

    m.insert(
        "cli.known_flag.view".into(),
        "--view Quick view of current directory tree".into(),
    );

    m.insert(
        "cli.known_flag.view_size".into(),
        "--view --size Quick view with directory sizes".into(),
    );

    m.insert(
        "cli.known_flag.watch".into(),
        "--watch [ON|OFF] Show or set file watcher state".into(),
    );

    m.insert(
        "cli.known_flag.where".into(),
        "--where Show ntc executable and config location".into(),
    );

    m.insert("cli.logo.c".into(), "C".into());

    m.insert("cli.logo.center".into(), "Center".into());

    m.insert("cli.logo.letters_format".into(), " {} {} {}".into());

    m.insert("cli.logo.n".into(), "N".into());

    m.insert("cli.logo.navigate".into(), "Navigate".into());

    m.insert("cli.logo.subtitle_format".into(), " {} {} {}".into());

    m.insert("cli.logo.t".into(), "T".into());

    m.insert("cli.logo.toolkit".into(), "Toolkit".into());

    m.insert(
        "cli.run.about".into(),
        "Navigate, Tree, Cat - Directory tree viewer and file concatenator".into(),
    );

    m.insert("cli.run.help_cared".into(), "cared".into());

    m.insert("cli.run.help_caref".into(), "caref".into());

    m.insert("cli.run.help_caren".into(), "caren".into());

    m.insert(
        "cli.run.help_clear".into(),
        "Clear the terminal screen".into(),
    );

    m.insert(
        "cli.run.help_copy".into(),
        "Copy report to clipboard instead of saving to file".into(),
    );

    m.insert(
        "cli.run.help_edit".into(),
        "Open file in built-in text editor".into(),
    );

    m.insert(
        "cli.run.help_explorer".into(),
        "Launch NTC File Explorer TUI (optional starting path)".into(),
    );

    m.insert(
        "cli.run.help_format".into(),
        "Output format: txt, html, json, md".into(),
    );

    m.insert(
        "cli.run.help_generate_completions".into(),
        "Generate shell completion script (bash|zsh|fish|powershell)".into(),
    );

    m.insert(
        "cli.run.help_helpextra".into(),
        "Show extended help for subcommands".into(),
    );

    m.insert(
        "cli.run.help_hyper".into(),
        "Create hyper project scaffold (DSA library)".into(),
    );

    m.insert(
        "cli.run.help_igcare_export_all".into(),
        "Export all ignore/care settings to <name>.ntc.igcare".into(),
    );

    m.insert(
        "cli.run.help_igcare_export_select".into(),
        "Select categories to export to <name>.ntc.igcare".into(),
    );

    m.insert(
        "cli.run.help_igcare_import".into(),
        "Import ignore/care settings from .ntc.igcare file".into(),
    );

    m.insert(
        "cli.run.help_ignore".into(),
        "Show ntc executable location".into(),
    );

    m.insert("cli.run.help_ignored".into(), "ignoren".into());

    m.insert("cli.run.help_ignoref".into(), "ignoref".into());

    m.insert("cli.run.help_ignoren".into(), "ignoren".into());

    m.insert(
        "cli.run.help_image".into(),
        "Convert image to document (JSON, MD, HTML, TXT)".into(),
    );

    m.insert(
        "cli.run.help_image_to".into(),
        "Output format for image conversion: json, md, html, txt".into(),
    );

    m.insert(
        "cli.run.help_init".into(),
        "Create starter file from template and open in editor".into(),
    );

    m.insert(
        "cli.run.help_input".into(),
        "Input file or directory path".into(),
    );

    m.insert(
        "cli.run.help_lang".into(),
        "Language for hyper project".into(),
    );

    m.insert(
        "cli.run.help_list".into(),
        "List all command-line functions".into(),
    );

    m.insert(
        "cli.run.help_lsp".into(),
        "Start the ntc-math LSP server over stdio".into(),
    );

    m.insert(
        "cli.run.help_lsp_log".into(),
        "Path to write LSP debug log".into(),
    );

    m.insert(
        "cli.run.help_math".into(),
        "Evaluate a math expression".into(),
    );

    m.insert(
        "cli.run.help_mode".into(),
        "Switch between normal and admin mode".into(),
    );

    m.insert("cli.run.help_modn".into(), "modn".into());

    m.insert(
        "cli.run.help_output".into(),
        "Output filename for report".into(),
    );

    m.insert(
        "cli.run.help_path".into(),
        "Target path or teleport savepoint (use with --size or --view)".into(),
    );

    m.insert(
        "cli.run.help_ral_export_all".into(),
        "Export all run aliases to <name>.ntc.ral".into(),
    );

    m.insert(
        "cli.run.help_ral_export_select".into(),
        "Select run aliases to export to <name>.ntc.ral".into(),
    );

    m.insert(
        "cli.run.help_ral_import".into(),
        "Import run aliases from .ntc.ral file".into(),
    );

    m.insert("cli.run.help_say".into(), "Print text to stdout".into());

    m.insert(
        "cli.run.help_seta".into(),
        "Show or set autosuggest (ghost text) (ON/OFF)".into(),
    );

    m.insert(
        "cli.run.help_setc".into(),
        "Show or set color output (ON/OFF)".into(),
    );

    m.insert(
        "cli.run.help_setd".into(),
        "Show or set max recursion depth".into(),
    );

    m.insert(
        "cli.run.help_seth".into(),
        "Show or set history (path/ON/OFF/default)".into(),
    );

    m.insert(
        "cli.run.help_setl".into(),
        "Show or set line number display (ON/OFF)".into(),
    );

    m.insert(
        "cli.run.help_seto".into(),
        "Show or set the default output directory".into(),
    );

    m.insert(
        "cli.run.help_sett".into(),
        "Show or set number of threads".into(),
    );

    m.insert(
        "cli.run.help_showcg".into(),
        "Show current configuration overview".into(),
    );

    m.insert(
        "cli.run.help_size".into(),
        "Show size of a file or directory".into(),
    );

    m.insert(
        "cli.run.help_time".into(),
        "Show elapsed time (use with --size or --view)".into(),
    );

    m.insert(
        "cli.run.help_tp_add".into(),
        "Save current directory as teleport point".into(),
    );

    m.insert(
        "cli.run.help_tp_list".into(),
        "List all teleport points".into(),
    );

    m.insert("cli.run.help_tp_rm".into(), "Remove teleport point".into());

    m.insert(
        "cli.run.help_upgrade".into(),
        "Check for updates and upgrade ntc to the latest version".into(),
    );

    m.insert(
        "cli.run.help_version".into(),
        "Show version information".into(),
    );

    m.insert(
        "cli.run.help_view".into(),
        "Quick view of current directory tree".into(),
    );

    m.insert(
        "cli.run.help_watch".into(),
        "Show or set file watcher (ON/OFF)".into(),
    );

    m.insert(
        "cli.run.help_where".into(),
        "Show ntc executable location".into(),
    );

    m.insert("cli.section.config".into(), "CONFIGURATION OPTIONS:".into());

    m.insert("cli.section.editor".into(), "EDITOR OPTIONS:".into());

    m.insert("cli.section.examples".into(), "EXAMPLES:".into());

    m.insert(
        "cli.section.export".into(),
        "ALIAS / IGNORE/CARE EXPORT:".into(),
    );

    m.insert("cli.section.igcare".into(), "IGNORE/CARE OPTIONS:".into());

    m.insert(
        "cli.section.interactive_only".into(),
        "INTERACTIVE-ONLY (run 'ntc' with no args):".into(),
    );

    m.insert("cli.section.other".into(), "OTHER OPTIONS:".into());

    m.insert("cli.section.report".into(), "REPORT OPTIONS:".into());

    m.insert(
        "cli.section.shell_commands".into(),
        "SHELL COMMANDS (also work from CLI):".into(),
    );

    m.insert(
        "cli.section.teleport".into(),
        "TELEPORT SAVE POINTS:".into(),
    );

    m.insert(
        "cli.shell_cmd.dl".into(),
        " ntc dl https://example.com/file.zip --progress Download with progress".into(),
    );

    m.insert(
        "cli.shell_cmd.encode".into(),
        " ntc encode b64 hello Base64 encode a string".into(),
    );

    m.insert(
        "cli.shell_cmd.math".into(),
        " ntc math 3+4*5 Evaluate expression".into(),
    );

    m.insert("cli.shell_cmd.ne".into(), " ntc ne Open text editor".into());

    m.insert(
        "cli.shell_cmd.pathcp".into(),
        " ntc pathcp Copy path to clipboard".into(),
    );

    m.insert(
        "cli.shell_cmd.qr".into(),
        " ntc qr \"text\" Generate QR code in terminal".into(),
    );

    m.insert(
        "cli.shell_cmd.ral_add".into(),
        " ntc ral add btr \"cargo build --release\" Add alias from CLI\n".into(),
    );

    m.insert(
        "cli.shell_cmd.say_loop".into(),
        " ntc say \"hello\" --loop 3 Print text with loop".into(),
    );

    m.insert(
        "cli.shell_cmd.serve".into(),
        " ntc serve files --port 8080 Start files drive server on port 8080".into(),
    );

    m.insert(
        "cli.shell_cmd.size".into(),
        " ntc size Show directory size".into(),
    );

    m.insert(
        "cli.shell_cmd.tl".into(),
        " ntc tl dir . Count lines in directory".into(),
    );

    m.insert(
        "cli.shell_cmd.view".into(),
        " ntc view Show directory tree".into(),
    );

    m.insert("cli.usage.basic".into(), " ntc [OPTIONS]".into());

    m.insert(
        "cli.usage.clipboard".into(),
        " ntc -i <path> --cp Copy report to clipboard\n".into(),
    );

    m.insert("cli.usage.header".into(), "USAGE:".into());

    m.insert(
        "cli.usage.input".into(),
        " ntc -i <path> [-o <output>]".into(),
    );
}
