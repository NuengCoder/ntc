use colored::*;

/// Pre‑process raw args to accept `-say`, `-print` as long options.
pub(super) fn preprocess_args(args: Vec<String>) -> Vec<String> {
    args.into_iter()
        .map(|arg| match arg.as_str() {
            "-say" | "-print" => "--say".to_string(),
            _ => arg,
        })
        .collect()
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
        "Tree".green(), 
        "Cat".blue()
    );
    println!();
}

/// Print detailed help
pub(super) fn print_help() {
    println!("ntc {} - Navigate, Tree, Cat", env!("CARGO_PKG_VERSION").green().bold());
    println!("A combined directory tree viewer and file concatenator.\n");
    println!("{}", "USAGE:".cyan().bold());
    println!("    ntc [OPTIONS]");
    println!("    ntc -i <path> [-o <output>]");
    println!("    ntc -i <path> --cp              Copy report to clipboard\n");
    println!("{}", "OPTIONS:".cyan().bold());
    println!("    -i, --input <path>      Process a file or directory");
    println!("    -o, --output <file>     Save output to specified file");
    println!("    -f, --format <FORMAT>   Output format: txt, html, json, md, pdf, docx, xlsx (default: txt)");
    println!("    --cp                    Copy report to clipboard instead of saving to file");
    println!("    --setO [path]           Show or set the output directory (default: Desktop)");
    println!("    --setD [depth]          Show or set max recursion depth (min: 1, max: 20)");
    println!("    --setL [ON|OFF]         Show or toggle line numbers for file display");
    println!("    --setC [ON|OFF]         Show or toggle color output");
    println!("    --setT [threads]        Show or set number of threads (default: 4)");
    println!("    --setH [path|ON|OFF]    Show/set history path or enable/disable");
    println!("    --showcg                Show current configuration overview");
    println!("    --watch [ON|OFF]        Show/set file watcher state");
    println!("    --where                 Show ntc executable and config location");
    println!("    -say, -print <text>     Print text to stdout");
    println!("    --size                  Show current directory size");
    println!("    --view                  Quick view of current directory tree");
    println!("    --view --size           Quick view with directory sizes");
    println!("    --view --care           Quick view with tree caring everything (sizes too)");
    println!("    --clear                 Clear the terminal screen");
    println!("    --version               Show version information");
    println!("    --math <EXPR>           Evaluate a math expression (e.g. --math \"3+4*5\")");
    println!("    --list, --fun           List all command-line functions");
    println!("    --help                  Show this help message\n");
    println!("{}", "IGNORE/CARE OPTIONS:".cyan().bold());
    println!("    --ignored               Show currently ignored items");
    println!("    --ignore <name>         Ignore a directory name");
    println!("    --cared <name>          Stop ignoring a directory name");
    println!("    --ignoref <ext>         Ignore a file extension");
    println!("    --caref <ext>           Care about a file extension");
    println!("    --ignoren <file>        Ignore a specific file (e.g., Cargo.lock)");
    println!("    --caren <file>          Care about a specific file\n");
    println!("{}", "TELEPORT SAVE POINTS:".cyan().bold());
    println!("    --tp-add <name>         Save current directory as teleport point");
    println!("    --tp-list               List all teleport points");
    println!("    --tp-rm <name>          Remove a teleport point");
    println!("    Note: Teleport navigation (jump/to) only works in interactive shell\n");
    println!("{}", "EXAMPLES:".cyan().bold());
    println!("    ntc                         Launch interactive mode");
    println!("    ntc @web                    Launch and teleport to 'web' savepoint");
    println!("    ntc --math \"3+4*5\"          Evaluate math expression");
    println!("    ntc --math \"sin(PI/2)\"      Math with functions and constants");
    println!("    ntc -i src                  Generate report of src directory");
    println!("    ntc -i src -o report.html   Generate HTML report");
    println!("    ntc -i src --cp             Copy directory tree to clipboard");
    println!("    ntc -i src -f json --cp     Copy JSON report to clipboard");
    println!("    ntc -i src -f pdf           Generate PDF report");
    println!("    ntc -i src -f docx          Generate DOCX report");
    println!("    ntc -i src -f xlsx          Generate XLSX report (with dependency analysis)");
    println!("    ntc -i file.txt             Display file contents");
    println!("    ntc --setL ON               Enable line numbers");
    println!("    ntc --showcg                Show configuration");
    println!("    ntc -say \"Hello World\"      Print Hello World");
    println!("    ntc --watch ON              Enable file watcher\n");
    println!("For interactive commands, launch ntc without arguments.");
    println!("{}", "INTERACTIVE-ONLY COMMANDS:".yellow().bold());
    println!("    Navigation: go, godrive, back, gos, gosc");
    println!("    Teleport: tp, tp jump, tp to, @name");
    println!("    Reports: txt, txt --cp, json --cp, md --cp");
    println!("    Configuration: showcg, opencg, resetcg, restorecg");
    println!("    Run Aliases: ral add, ral edit, ral list, ral rm, ral cls, ral export, ral import");
    println!("    Ignore/Care: igcare, igcare export, igcare import, ignoresc, caresc");
    println!("    Math: math <expr>, math fun, math timer, math <file>.ntc.math");
    println!("    These commands only work inside the interactive shell (run 'ntc' alone).\n");
}