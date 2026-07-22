use colored::*;
use std::io::{self, Write};

/// Pre‑process raw args to accept `-say`, `-print`, `-image` as long options,
/// and convert `image <path> to <format>` syntax into structured args.
pub(super) fn preprocess_args(args: Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-say" | "-print" => out.push("--say".to_string()),
            "-image" => {
                out.push("--image".to_string());
                // Check if the next tokens follow: <path> to <format>
                if i + 3 < args.len() && args[i + 2] == "to" {
                    // image <path> to <format>
                    out.push(args[i + 1].clone());           // <path>
                    out.push("--image-to".to_string());      // our internal flag
                    out.push(args[i + 3].clone());           // <format>
                    i += 4;
                    continue;
                } else if i + 1 < args.len() {
                    // image <path> (no to/format, just eat the path)
                    out.push(args[i + 1].clone());
                    i += 2;
                    continue;
                }
            }
            "-o" => out.push("--output".to_string()),
            _ => out.push(arg.clone()),
        }
        i += 1;
    }
    out
}

pub(super) fn print_logo() {
    println!();
    println!("  {}  {}  {}", 
        "N".blue().bold(), 
        "T".blue().bold(), 
        "C".blue().bold()
    );
    println!("  {} {} {}", 
        "Navigate".red(), 
        "Toolkit".green(), 
        "Center".blue()
    );
    println!();
}

/// Print detailed help
pub(super) fn print_help() {
    println!("{}", "ntc <version> — Navigate, Toolkit, Center".green().bold());
    println!("A combine of Navigation, Project Toolkit Center hub.\n");

    print_basic_usage();
    print_report_flags();
    print_config_flags();
    print_editor_flags();
    print_igcare_flags();
    print_teleport_flags();
    print_alias_export_flags();
    print_misc_flags();
    print_cli_examples();
    print_shell_cli_commands();
    print_interactive_only_note();
}

fn print_basic_usage() {
    println!("{}", "USAGE:".cyan().bold());
    println!("    ntc [OPTIONS]");
    println!("    ntc -i <path> [-o <output>]");
    println!("    ntc -i <path> --cp              Copy report to clipboard\n");
}

fn print_report_flags() {
    println!("{}", "REPORT OPTIONS:".cyan().bold());
    println!("    -i, --input <path>      Process a file or directory");
    println!("    -o, --output <file>     Save output to specified file");
    println!("    -f, --format <FORMAT>   Output format: txt, html, json, md, pdf, docx, xlsx (default: txt)");
    println!("    --cp                    Copy report to clipboard instead of saving to file");
    println!("    --size                  Show current directory size");
    println!("    --view                  Quick view of current directory tree");
    println!("    --view --size           Quick view with directory sizes\n");
}

fn print_config_flags() {
    println!("{}", "CONFIGURATION OPTIONS:".cyan().bold());
    println!("    --setO [path]           Show or set the output directory (default: Desktop)");
    println!("    --setD [depth]          Show or set max recursion depth (min: 1, max: 20)");
    println!("    --setL [ON|OFF]         Show or toggle line numbers for file display");
    println!("    --setC [ON|OFF]         Show or toggle color output");
    println!("    --setT [threads]        Show or set number of threads (default: 4)");
    println!("    --setH [path|ON|OFF]    Show/set history path or enable/disable");
    println!("    --setA [ON|OFF]         Show/set autosuggest (ghost text)");
    println!("    --showcg                Show current configuration overview");
    println!("    --watch [ON|OFF]        Show/set file watcher state");
    println!("    --where                 Show ntc executable and config location\n");
}

fn print_editor_flags() {
    println!("{}", "EDITOR OPTIONS:".cyan().bold());
    println!("    -e, --edit <file>       Open file in built-in text editor");
    println!("    --init <file>           Create starter template and open in editor");
    println!("    --hyper <name> --lang c Create C/C++ hyper project scaffold\n");
}

fn print_igcare_flags() {
    println!("{}", "IGNORE/CARE OPTIONS:".cyan().bold());
    println!("    --ignored               Show currently ignored items");
    println!("    --ignore <name>         Ignore a directory name");
    println!("    --cared <name>          Stop ignoring a directory name");
    println!("    --ignoref <ext>         Ignore a file extension");
    println!("    --caref <ext>           Care about a file extension");
    println!("    --ignoren <file>        Ignore a specific file (e.g., Cargo.lock)");
    println!("    --caren <file>          Care about a specific file\n");
}

fn print_teleport_flags() {
    println!("{}", "TELEPORT SAVE POINTS:".cyan().bold());
    println!("    --tp-add <name>         Save current directory as teleport point");
    println!("    --tp-list               List all teleport points");
    println!("    --tp-rm <name>          Remove a teleport point\n");
}

fn print_alias_export_flags() {
    println!("{}", "ALIAS / IGNORE/CARE EXPORT:".cyan().bold());
    println!("    --ral-export-all <name>     Export all run aliases to <name>.ntc.ral");
    println!("    --ral-export-select <name>  Select aliases to export");
    println!("    --ral-import <file>         Import aliases from .ntc.ral file");
    println!("    --igcare-export-all <name>  Export all ignore/care settings");
    println!("    --igcare-export-select <name> Select categories to export");
    println!("    --igcare-import <file>      Import settings from .ntc.igcare file\n");
}

fn print_misc_flags() {
    println!("{}", "OTHER OPTIONS:".cyan().bold());
    println!("    -say, -print <text>     Print text to stdout");
    println!("    -image <path> to <fmt>  Convert image to document for AI (json|md|html|txt)");
    println!("    -x, --explorer [PATH]   Launch NTC File Explorer TUI");
    println!("    --dino                  Play the dinosaur runner game");
    println!("    --clear                 Clear the terminal screen");
    println!("    --version               Show version information");
    println!("    --math <EXPR>           Evaluate a math expression (e.g. --math \"3+4*5\")");
    println!("    --list, --fun           List all command-line functions");
    println!("    --help                  Show this help message");
    println!("    --generate-completions <SHELL>  Generate completions (bash|zsh|fish|powershell)\n");
}

fn print_cli_examples() {
    println!("{}", "EXAMPLES:".cyan().bold());
    println!("    ntc                         Launch interactive shell");
    println!("    ntc @web                    Launch and teleport to 'web' savepoint");
    println!("    ntc --math \"3+4*5\"          Evaluate math expression");
    println!("    ntc --math \"sin(PI/2)\"      Math with functions and constants");
    println!("    ntc -i src                  Generate TXT report of src directory");
    println!("    ntc -i src --cp             Copy directory tree to clipboard");
    println!("    ntc -i src -f json --cp     Copy JSON report to clipboard");
    println!("    ntc -i src -f pdf           Generate PDF report from src");
    println!("    ntc -i src -f docx          Generate DOCX report from src");
    println!("    ntc -i src -f xlsx          Generate XLSX report (with dependency analysis)");
    println!("    ntc -i file.txt             Display file contents");
    println!("    ntc --setL ON               Enable line numbers");
    println!("    ntc --showcg                Show configuration");
    println!("    ntc -say \"Hello World\"      Print Hello World");
    println!("    ntc --watch ON              Enable file watcher");
    println!("    ntc -image screenshot.png to json   Convert image to JSON for AI\n");
}

fn print_shell_cli_commands() {
    println!("{}", "SHELL COMMANDS (also work from CLI):".cyan().bold());
    println!("    ntc pathcp              Copy path to clipboard");
    println!("    ntc say \"hello\" --loop 3  Print text with loop");
    println!("    ntc tl dir .            Count lines in directory");
    println!("    ntc ne                  Open text editor");
    println!("    ntc size                Show directory size");
    println!("    ntc view                Show directory tree");
    println!("    ntc serve 8080          Start HTTP server on port 8080");
    println!("    ntc dl https://example.com/file.zip --progress   Download with progress");
    println!("    ntc encode b64 hello    Base64 encode a string");
    println!("    ntc qr \"text\"           Generate QR code in terminal");
    println!("    ntc math 3+4*5          Evaluate expression");
    println!("    ntc ral add btr \"cargo build --release\"   Add alias from CLI\n");
}

fn print_interactive_only_note() {
    println!("{}", "INTERACTIVE-ONLY (run 'ntc' with no args):".yellow().bold());
    println!("    Navigation: cd, back, gos, gosc");
    println!("    Teleport: tp jump, tp to, @name");
    println!("    Tutorial: tutorial — Start interactive step-by-step tutorial\n");
}

/// Generate shell completion script for the given shell type.
pub(super) fn generate_completions(shell: &str) {
    let script = match shell.to_lowercase().as_str() {
        "bash" => include_str!("completions/ntc.bash"),
        "zsh" => include_str!("completions/ntc.zsh"),
        "fish" => include_str!("completions/ntc.fish"),
        "powershell" => include_str!("completions/ntc.ps1"),
        _ => {
            eprintln!("Unsupported shell: {}. Use: bash, zsh, fish, powershell", shell);
            return;
        }
    };
    if let Err(e) = io::stdout().write_all(script.as_bytes()) {
        eprintln!("Failed to write completions: {}", e);
    }
    let _ = io::stdout().flush();
}