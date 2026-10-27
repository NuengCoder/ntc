use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "clap.arg.cared.help".into(),
        "Stop ignoring one or more directories".into(),
    );

    m.insert(
        "clap.arg.caref.help".into(),
        "Care about one or more file extensions".into(),
    );

    m.insert(
        "clap.arg.caren.help".into(),
        "Care about one or more specific files".into(),
    );

    m.insert(
        "clap.arg.clear.help".into(),
        "Clear the terminal screen".into(),
    );

    m.insert(
        "clap.arg.copy.help".into(),
        "Copy report to clipboard instead of saving to file".into(),
    );

    m.insert(
        "clap.arg.edit.help".into(),
        "Open file in built-in text editor".into(),
    );

    m.insert(
        "clap.arg.explorer.help".into(),
        "Launch NTC File Explorer TUI (optional starting path)".into(),
    );

    m.insert(
        "clap.arg.format_cli.help".into(),
        "Output format: txt, html, json, md".into(),
    );

    m.insert(
        "clap.arg.generate_completions.help".into(),
        "Generate shell completion script (bash|zsh|fish|powershell)".into(),
    );

    m.insert("clap.arg.help_arg.help".into(), "Show detailed help".into());

    m.insert(
        "clap.arg.hyper.help".into(),
        "Create hyper project scaffold (DSA library)".into(),
    );

    m.insert(
        "clap.arg.igcare_export_all.help".into(),
        "Export all ignore/care settings to <name>.ntc.igcare".into(),
    );

    m.insert(
        "clap.arg.igcare_export_select.help".into(),
        "Select categories to export to <name>.ntc.igcare".into(),
    );

    m.insert(
        "clap.arg.igcare_import.help".into(),
        "Import ignore/care settings from .ntc.igcare file".into(),
    );

    m.insert(
        "clap.arg.ignore.help".into(),
        "Ignore one or more directory names".into(),
    );

    m.insert("clap.arg.ignored.help".into(), "Show ignored items".into());

    m.insert(
        "clap.arg.ignoref.help".into(),
        "Ignore one or more file extensions".into(),
    );

    m.insert(
        "clap.arg.ignoren.help".into(),
        "Ignore one or more specific files".into(),
    );

    m.insert(
        "clap.arg.image.help".into(),
        "Convert image to document (JSON, MD, HTML, TXT)".into(),
    );

    m.insert(
        "clap.arg.image_to.help".into(),
        "Output format for image conversion: json, md, html, txt".into(),
    );

    m.insert(
        "clap.arg.init.help".into(),
        "Create starter file from template and open in editor".into(),
    );

    m.insert(
        "clap.arg.input.help".into(),
        "Input file or directory path".into(),
    );

    m.insert(
        "clap.arg.lang_cli.help".into(),
        "Language for hyper project".into(),
    );

    m.insert(
        "clap.arg.list.help".into(),
        "List all command-line functions".into(),
    );

    m.insert(
        "clap.arg.lsp.help".into(),
        "Start the ntc-math LSP server over stdio".into(),
    );

    m.insert(
        "clap.arg.lsp_log.help".into(),
        "Path to write LSP debug log".into(),
    );

    m.insert(
        "clap.arg.math.help".into(),
        "Evaluate a math expression".into(),
    );

    m.insert(
        "clap.arg.output.help".into(),
        "Output filename for report".into(),
    );

    m.insert(
        "clap.arg.ral_export_all.help".into(),
        "Export all run aliases to <name>.ntc.ral".into(),
    );

    m.insert(
        "clap.arg.ral_export_select.help".into(),
        "Select run aliases to export to <name>.ntc.ral".into(),
    );

    m.insert(
        "clap.arg.ral_import.help".into(),
        "Import run aliases from .ntc.ral file".into(),
    );

    m.insert("clap.arg.say.help".into(), "Print text to stdout".into());

    m.insert(
        "clap.arg.setA.help".into(),
        "Show or set autosuggest (ghost text) (ON/OFF)".into(),
    );

    m.insert(
        "clap.arg.setC.help".into(),
        "Show or set color output (ON/OFF)".into(),
    );

    m.insert(
        "clap.arg.setD.help".into(),
        "Show or set max recursion depth".into(),
    );

    m.insert(
        "clap.arg.setH.help".into(),
        "Show or set history (path/ON/OFF/default)".into(),
    );

    m.insert(
        "clap.arg.setL.help".into(),
        "Show or set line number display (ON/OFF)".into(),
    );

    m.insert(
        "clap.arg.setO.help".into(),
        "Show or set the default output directory".into(),
    );

    m.insert(
        "clap.arg.setT.help".into(),
        "Show or set number of threads".into(),
    );

    m.insert(
        "clap.arg.showcg.help".into(),
        "Show current configuration overview".into(),
    );

    m.insert(
        "clap.arg.size_cli.help".into(),
        "Show current directory size".into(),
    );

    m.insert(
        "clap.arg.time.help".into(),
        "Show elapsed time (use with --size or --view)".into(),
    );

    m.insert(
        "clap.arg.tp_add.help".into(),
        "Save current directory as teleport point".into(),
    );

    m.insert(
        "clap.arg.tp_list.help".into(),
        "List all teleport points".into(),
    );

    m.insert("clap.arg.tp_rm.help".into(), "Remove teleport point".into());

    m.insert(
        "clap.arg.upgrade.help".into(),
        "Check for updates and upgrade ntc to the latest version".into(),
    );

    m.insert(
        "clap.arg.version.help".into(),
        "Show version information".into(),
    );

    m.insert(
        "clap.arg.view_cli.help".into(),
        "Quick view of current directory tree".into(),
    );

    m.insert(
        "clap.arg.watch_cli.help".into(),
        "Show or set file watcher (ON/OFF)".into(),
    );

    m.insert(
        "clap.arg.where_cli.help".into(),
        "Show ntc executable location".into(),
    );

    m.insert("clap.value_name.depth".into(), "DEPTH".into());

    m.insert("clap.value_name.expr".into(), "EXPR".into());

    m.insert("clap.value_name.ext".into(), "EXT".into());

    m.insert("clap.value_name.file".into(), "FILE".into());

    m.insert("clap.value_name.format_cli".into(), "FORMAT".into());

    m.insert("clap.value_name.image".into(), "IMAGE".into());

    m.insert("clap.value_name.lang".into(), "LANG".into());

    m.insert("clap.value_name.name".into(), "NAME".into());

    m.insert("clap.value_name.path".into(), "PATH".into());

    m.insert("clap.value_name.project".into(), "PROJECT".into());

    m.insert("clap.value_name.shell".into(), "SHELL".into());

    m.insert("clap.value_name.state".into(), "STATE".into());

    m.insert("clap.value_name.text".into(), "TEXT".into());

    m.insert("clap.value_name.threads".into(), "THREADS".into());

    m.insert("clap.value_name.value".into(), "VALUE".into());
}
