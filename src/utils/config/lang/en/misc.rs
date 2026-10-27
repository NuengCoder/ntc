use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "misc.count.usage".into(),
        "Usage: count <needle> [text]   (or: <command> | count <needle>)".into(),
    );

    m.insert(
        "misc.trim.usage".into(),
        "Usage: trim <pattern> [text]   (or: <command> | trim <pattern> [...])".into(),
    );

    m.insert(
        "misc.trim.empty_pattern".into(),
        "trim: pattern must not be empty".into(),
    );

    m.insert(
        "misc.clear.type_help".into(),
        "Type 'help' for available commands, 'exit' to quit.".into(),
    );

    m.insert(
        "misc.clear.welcome".into(),
        "Welcome to ntc {} - Navigate, Toolkit, Center".into(),
    );

    m.insert(
        "misc.ds.example1".into(),
        " ds src # search using config depth".into(),
    );

    m.insert(
        "misc.ds.example2".into(),
        " ds src -d 3 # search up to 3 levels deep".into(),
    );

    m.insert(
        "misc.ds.example3".into(),
        " ds test # partial match: finds test_utils/ etc.".into(),
    );

    m.insert(
        "misc.ds.usage".into(),
        "Usage: ds <pattern> [-d <depth>]".into(),
    );

    m.insert(
        "misc.fs.example1".into(),
        " fs main.c # search using config depth".into(),
    );

    m.insert(
        "misc.fs.example2".into(),
        " fs main.c -d 5 # search up to 5 levels deep".into(),
    );

    m.insert(
        "misc.fs.example3".into(),
        " fs main # partial match: finds main_helper.c etc.".into(),
    );

    m.insert(
        "misc.fs.usage".into(),
        "Usage: fs <pattern> [-d <depth>]".into(),
    );

    m.insert("misc.fsc.copied".into(), "Copied '{}' to clipboard!".into());

    m.insert(
        "misc.fsc.copy_prompt".into(),
        "Copy to clipboard? (y/N): ".into(),
    );

    m.insert("misc.fsc.done_opt".into(), "0. Done".into());

    m.insert("misc.fsc.done_str".into(), "Done.".into());

    m.insert(
        "misc.fsc.example1".into(),
        " fsc main.rs # search then pick file to display".into(),
    );

    m.insert(
        "misc.fsc.example2".into(),
        " fsc main.rs -d 5 # search up to 5 levels deep".into(),
    );

    m.insert(
        "misc.fsc.found".into(),
        "\u{1F50D} Found {} file(s) for \"{}\":".into(),
    );

    m.insert("misc.fsc.invalid_input".into(), "Invalid input.".into());

    m.insert(
        "misc.fsc.invalid_selection".into(),
        "Invalid selection.".into(),
    );

    m.insert(
        "misc.fsc.not_file".into(),
        "Selected path is not a file.".into(),
    );

    m.insert(
        "misc.fsc.select_prompt".into(),
        "Select file to display: ".into(),
    );

    m.insert(
        "misc.fsc.skipped".into(),
        "Skipped (not support format): {}".into(),
    );

    m.insert(
        "misc.fsc.usage".into(),
        "Usage: fsc <pattern> [-d <depth>]".into(),
    );

    m.insert(
        "misc.gs.example1".into(),
        " gs main # search file contents using config depth".into(),
    );

    m.insert(
        "misc.gs.example2".into(),
        " gs main -d 5 # search up to 5 levels deep".into(),
    );

    m.insert(
        "misc.gs.example3".into(),
        " gs -i main # interactive mode (browse results, Enter to open)".into(),
    );

    m.insert(
        "misc.gs.example4".into(),
        " gs -i main -d 3 # interactive mode with depth limit".into(),
    );

    m.insert(
        "misc.gs.example5".into(),
        " gs main -d 5 -l 10 # show only first 10 matches".into(),
    );

    m.insert(
        "misc.gs.example6".into(),
        " gs main -p \"*.rs\" # only search files matching glob".into(),
    );

    m.insert(
        "misc.gs.example7".into(),
        " gs main -s size # sort by file size (line|path|size, bare -s = line)".into(),
    );

    m.insert(
        "misc.gs.example8".into(),
        " gs error\\|warning # OR search: match either term (a\\|b)".into(),
    );

    m.insert(
        "misc.gs.no_pipe_path".into(),
        "gs: -p can't be used with piped input (pipe carries text, not files)".into(),
    );

    m.insert(
        "misc.gs.pipe_sort_only_line".into(),
        "gs: only -s line works with piped input (path/size need files)".into(),
    );

    m.insert("misc.gs.searching".into(), "Searching contents...".into());

    m.insert(
        "misc.gs.many_results".into(),
        "gs: {} matches found, showing first 20,000. Narrow your search.".into(),
    );

    m.insert(
        "misc.gs.usage".into(),
        "Usage: gs <pattern> [-d <depth>] [-l <limit>] [-i] [-p <glob>]... [-s [line|path|size]]"
            .into(),
    );

    m.insert(
        "misc.qur.usage".into(),
        "Usage: qur <glob> [-s \"pattern\"] [-rp \"replacement\"] [-dp] [-d <depth>] [-l <limit>]"
            .into(),
    );

    m.insert(
        "misc.qur.example1".into(),
        " qur *.* # list files matching glob (recursive)".into(),
    );

    m.insert(
        "misc.qur.example2".into(),
        " qur *.rs -s \"TODO\" # summary per file".into(),
    );

    m.insert(
        "misc.qur.example3".into(),
        " qur *.* -s \"old\" -dp # detailed matches (like gs)".into(),
    );

    m.insert(
        "misc.qur.example4".into(),
        " qur *.md -s \"old\" -rp \"new\" -d 3 # preview, confirm, replace".into(),
    );

    m.insert("misc.qur.searching".into(), "Querying files...".into());

    m.insert(
        "misc.qur.no_files".into(),
        "qur: no files match '{}'.".into(),
    );

    m.insert(
        "misc.qur.no_matches".into(),
        "qur: no matches for '{}'.".into(),
    );

    m.insert(
        "misc.qur.files_header".into(),
        "Files matching query ({}):".into(),
    );

    m.insert(
        "misc.qur.summary_header".into(),
        "Matches for \"{}\" ({} lines in {} files):".into(),
    );

    m.insert(
        "misc.qur.summary_hint".into(),
        "Add -dp for detailed lines (like gs), or -rp \"new\" to replace.".into(),
    );

    m.insert(
        "misc.qur.replace_preview".into(),
        "Replace \"{}\" with \"{}\" — {} occurrences in {} files:".into(),
    );

    m.insert(
        "misc.qur.confirm".into(),
        "Replace {} occurrences in {} files? [y/N]:".into(),
    );

    m.insert("misc.qur.cancelled".into(), "Replace cancelled.".into());

    m.insert(
        "misc.qur.replaced".into(),
        "Replaced {} occurrences in {} files.".into(),
    );

    m.insert(
        "misc.qur.skipped".into(),
        "Skipped {} files (unreadable/write failed).".into(),
    );

    m.insert(
        "misc.qur.many_results".into(),
        "qur: {} matches found, showing first 20,000. Narrow your search.".into(),
    );

    m.insert(
        "misc.qur.replace_needs_select".into(),
        "qur: -rp needs -s \"<pattern>\" (e.g. qur *.* -s \"old\" -rp \"new\").".into(),
    );

    m.insert(
        "misc.qur.display_needs_select".into(),
        "qur: -dp needs -s \"<pattern>\" (e.g. qur *.* -s \"foo\" -dp).".into(),
    );

    m.insert(
        "misc.qur.empty_select".into(),
        "qur: -s needs a pattern (e.g. -s \"foo\").".into(),
    );

    m.insert(
        "misc.qur.empty_replace".into(),
        "qur: -rp needs a replacement (e.g. -rp \"bar\").".into(),
    );

    m.insert(
        "misc.qur.stdin_needs_select".into(),
        "qur: piped input needs -s \"<pattern>\" (e.g. cmd | qur -s \"foo\").".into(),
    );

    m.insert(
        "misc.head.usage".into(),
        "Usage: head [N] [file]  (N defaults to 10; reads piped input when no file)".into(),
    );

    m.insert(
        "misc.tail.usage".into(),
        "Usage: tail [N] [file]  (N defaults to 10; reads piped input when no file)".into(),
    );

    m.insert(
        "misc.betw.usage".into(),
        "Usage: betw A:B [file]  (A:B is 1-based inclusive; reads piped input when no file)".into(),
    );

    m.insert(
        "misc.window.bad_arg".into(),
        "{}: unexpected argument '{}'".into(),
    );

    m.insert(
        "misc.window.no_input".into(),
        "{}: no input (pipe something in or give a file)".into(),
    );

    m.insert(
        "misc.sort.usage".into(),
        "Usage: sort [-r] [-n] [-u] [-f] [file]  (reads piped input when no file)".into(),
    );

    m.insert(
        "misc.sort.bad_arg".into(),
        "sort: unknown option '{}'".into(),
    );

    m.insert(
        "misc.rev.usage".into(),
        "Usage: rev [file]  (reverses each line's characters; reads piped input when no file)"
            .into(),
    );

    m.insert(
        "misc.rev.bad_arg".into(),
        "rev: unexpected argument '{}' (usage: rev [file])".into(),
    );

    m.insert("misc.wi.not_dir".into(), "wi: not a directory: '{}'".into());

    m.insert("misc.wi.start_error".into(), "wi: cannot watch: {}".into());

    m.insert("misc.wi.usage".into(), "Usage: wi [path]".into());

    m.insert(
        "misc.locate.example1".into(),
        " locate main.c # search files and dirs (config depth)".into(),
    );

    m.insert(
        "misc.locate.example2".into(),
        " locate main.c -d 5 # search up to 5 levels deep".into(),
    );

    m.insert(
        "misc.locate.example3".into(),
        " locate main # partial match: finds main.c, main_helper/ etc.".into(),
    );

    m.insert(
        "misc.locate.header".into(),
        "\u{1F50D} {} for \"{}\" ({} result(s)):".into(),
    );

    m.insert(
        "misc.locate.label_didyoumean".into(),
        "Did you mean?".into(),
    );

    m.insert("misc.locate.label_exact".into(), "Exact matches".into());

    m.insert("misc.locate.label_partial".into(), "Partial matches".into());

    m.insert(
        "misc.locate.no_results".into(),
        "\n\u{1F50D} No results for \"{}\"\n".into(),
    );

    m.insert(
        "misc.locate.usage".into(),
        "Usage: locate <pattern> [-d <depth>]".into(),
    );

    m.insert("misc.mkd.created".into(), "Created directory: {}".into());

    m.insert("misc.mkd.error".into(), "mkd: {}: {}".into());

    m.insert("misc.mkd.missing".into(), "mkd: missing operand".into());

    m.insert("misc.mkf.created".into(), "Created file: {}".into());

    m.insert("misc.mkf.error".into(), "mkf: {}: {}".into());

    m.insert("misc.mkf.exists".into(), "Already exists: {}".into());

    m.insert("misc.mkf.missing".into(), "mkf: missing operand".into());

    m.insert(
        "misc.ne.already_in_editor".into(),
        "You are already in editor".into(),
    );

    m.insert(
        "misc.exit.in_editor".into(),
        "Cannot exit ntc inside ntceditor - close the editor to leave (Ctrl+Q hides the terminal)"
            .into(),
    );

    m.insert(
        "misc.exit.no_nested_ntc".into(),
        "Cannot stack an ntc shell inside ntceditor - this terminal already runs ntc".into(),
    );

    m.insert(
        "misc.ne.create_failed".into(),
        "Could not create template: {}".into(),
    );

    m.insert(
        "misc.ne.created_hyper".into(),
        "Created hyper project: {}".into(),
    );

    m.insert(
        "misc.ne.created_super".into(),
        "Created super template: {}".into(),
    );

    m.insert(
        "misc.ne.created_template".into(),
        "Created template: {}".into(),
    );

    m.insert("misc.ne.editor_error".into(), "Editor error: {}".into());

    m.insert(
        "misc.ne.file_missing".into(),
        "File not found: {}. Use --init <file> or --super <file> to create it.".into(),
    );

    m.insert(
        "misc.ne.hyper_error".into(),
        "Hyper project error: {}".into(),
    );

    m.insert(
        "misc.ne.missing_file".into(),
        "ne: missing file operand".into(),
    );

    m.insert(
        "misc.ne.one_file".into(),
        "Only one file can be created at a time".into(),
    );

    m.insert(
        "misc.ne.open_not_created".into(),
        "-I file must be one of the files being created: {}".into(),
    );

    m.insert(
        "misc.ne.usage_open".into(),
        "Usage: -I <file> must be the last argument (exactly one file)".into(),
    );

    m.insert(
        "misc.ne.opened_welcome".into(),
        "Opened welcome screen in {}".into(),
    );

    m.insert(
        "misc.ne.usage_hyper".into(),
        "Usage: ne --hyper <name> --lang <lang> (name must not contain extension)".into(),
    );

    m.insert(
        "misc.pathcp.copied".into(),
        "Path copied to clipboard: {}".into(),
    );

    m.insert("misc.pip.found".into(), "\u{2713} pip found at: {}".into());

    m.insert(
        "misc.pip.not_found".into(),
        "pip not found in venv/Scripts directory".into(),
    );

    m.insert("misc.rmd.error".into(), "rmd: {}: {}".into());

    m.insert("misc.rmd.missing".into(), "rmd: missing operand".into());

    m.insert(
        "misc.rmd.not_dir".into(),
        "rmd: '{}' is not a directory".into(),
    );

    m.insert(
        "misc.rmd.not_exist".into(),
        "rmd: '{}' does not exist".into(),
    );

    m.insert("misc.rmd.removed".into(), "Removed directory: {}".into());

    m.insert(
        "misc.rmd.no_match".into(),
        "rmd: no directories match '{}'".into(),
    );

    m.insert(
        "misc.rmd.pattern_matches".into(),
        "{} dir(s) match '{}':".into(),
    );

    m.insert("misc.rmd.confirm_delete".into(), "Delete {} dir(s)?".into());

    m.insert("misc.rmd.cancelled".into(), "Cancelled.".into());

    m.insert(
        "misc.rmd.deadlock_vault_admin_only".into(),
        "rmd: '{}' holds the deadlock vault and can only be removed by root/administrator".into(),
    );

    m.insert("misc.rmf.error".into(), "rmf: {}: {}".into());

    m.insert(
        "misc.rmf.is_dir".into(),
        "rmf: '{}' is a directory (use rmd instead)".into(),
    );

    m.insert("misc.rmf.missing".into(), "rmf: missing operand".into());

    m.insert(
        "misc.rmf.not_exist".into(),
        "rmf: '{}' does not exist".into(),
    );

    m.insert(
        "misc.rmf.no_match".into(),
        "rmf: no files match '{}'".into(),
    );

    m.insert(
        "misc.rmf.pattern_matches".into(),
        "{} file(s) match '{}':".into(),
    );

    m.insert(
        "misc.rmf.confirm_delete".into(),
        "Delete {} file(s)?".into(),
    );

    m.insert("misc.rmf.cancelled".into(), "Cancelled.".into());

    m.insert("misc.rmf.removed".into(), "Removed: {}".into());

    m.insert(
        "misc.rmf.deadlock_vault_admin_only".into(),
        "rmf: '{}' is the deadlock vault and can only be removed by root/administrator".into(),
    );

    m.insert("misc.rnmd.error".into(), "rnmd: {}".into());

    m.insert(
        "misc.rnmd.exists".into(),
        "rnmd: '{}' already exists".into(),
    );

    m.insert("misc.rnmd.missing".into(), "rnmd: missing operand".into());

    m.insert(
        "misc.rnmd.not_dir".into(),
        "rnmd: '{}' is not a directory".into(),
    );

    m.insert(
        "misc.rnmd.not_exist".into(),
        "rnmd: '{}' does not exist".into(),
    );

    m.insert(
        "misc.rnmd.renamed".into(),
        "Renamed directory '{}' -> '{}'".into(),
    );

    m.insert(
        "misc.rnmd.usage".into(),
        " Usage: rnmd <old_name> <new_name>".into(),
    );

    m.insert("misc.rnmf.error".into(), "rnmf: {}".into());

    m.insert(
        "misc.rnmf.exists".into(),
        "rnmf: '{}' already exists".into(),
    );

    m.insert(
        "misc.rnmf.is_dir".into(),
        "rnmf: '{}' is a directory (use rnmd instead)".into(),
    );

    m.insert("misc.rnmf.missing".into(), "rnmf: missing operand".into());

    m.insert(
        "misc.rnmf.not_exist".into(),
        "rnmf: '{}' does not exist".into(),
    );

    m.insert("misc.rnmf.renamed".into(), "Renamed '{}' -> '{}'".into());

    m.insert(
        "misc.rnmf.usage".into(),
        " Usage: rnmf <old_name> <new_name>".into(),
    );

    m.insert("misc.mv.error".into(), "mv: {}".into());

    m.insert(
        "misc.mv.exists".into(),
        "mv: target exists: '{}' (use -f to overwrite)".into(),
    );

    m.insert("misc.mv.missing".into(), "mv: missing operand".into());

    m.insert("misc.mv.moved".into(), "Moved '{}' -> '{}'".into());

    m.insert(
        "misc.mv.not_exist".into(),
        "mv: source not found: '{}'".into(),
    );

    m.insert("misc.mv.no_match".into(), "mv: no files match '{}'".into());

    m.insert(
        "misc.mv.dst_dir".into(),
        "mv: destination must be a directory: '{}'".into(),
    );

    m.insert(
        "misc.mv.wild_done".into(),
        "Moved {} file(s) to '{}'".into(),
    );

    m.insert(
        "misc.mv.usage".into(),
        "Usage: mv <src|@tp|pattern> <dst|@tp> [-f]".into(),
    );

    m.insert("misc.cp.error".into(), "cp: {}".into());

    m.insert(
        "misc.cp.exists".into(),
        "cp: target exists: '{}' (use -f to overwrite)".into(),
    );

    m.insert("misc.cp.missing".into(), "cp: missing operand".into());

    m.insert("misc.cp.copied".into(), "Copied '{}' -> '{}'".into());

    m.insert(
        "misc.cp.not_exist".into(),
        "cp: source not found: '{}'".into(),
    );

    m.insert("misc.cp.no_match".into(), "cp: no files match '{}'".into());

    m.insert(
        "misc.cp.dst_dir".into(),
        "cp: destination must be a directory: '{}'".into(),
    );

    m.insert(
        "misc.cp.wild_done".into(),
        "Copied {} file(s) to '{}'".into(),
    );

    m.insert(
        "misc.cp.same".into(),
        "cp: source and target are the same file: '{}'".into(),
    );

    m.insert(
        "misc.cp.usage".into(),
        "Usage: cp <src|@tp|pattern> <dst|@tp> [-f]".into(),
    );

    m.insert(
        "misc.say.example1".into(),
        " say \"hello world\" --loop 5 -n # 5 lines".into(),
    );

    m.insert(
        "misc.say.example2".into(),
        " say \"hello world\" --loop 3 -t # 1 line, 3 times".into(),
    );

    m.insert(
        "misc.say.loop_exceed".into(),
        "--loop value exceeds max ({}), clamping".into(),
    );

    m.insert(
        "misc.say.loop_invalid".into(),
        "Invalid --loop value '{}', using 1".into(),
    );

    m.insert(
        "misc.say.loop_requires".into(),
        "--loop requires a number, using 1".into(),
    );

    m.insert(
        "misc.say.loop_warning".into(),
        "--loop value cannot be less than 1, using 1".into(),
    );

    m.insert(
        "misc.say.usage".into(),
        "Usage: say <text> [--loop N] [-n|-t]".into(),
    );

    m.insert(
        "misc.show_env.activate_hint".into(),
        "Run 'python_activate' to activate a virtual environment".into(),
    );

    m.insert(
        "misc.show_env.active".into(),
        "Virtual environment is".into(),
    );

    m.insert("misc.show_env.and_more".into(), "... and {} more".into());

    m.insert(
        "misc.show_env.cwd_label".into(),
        "Current directory:".into(),
    );

    m.insert(
        "misc.show_env.deactivate_hint".into(),
        "Run 'deactivate' to exit the virtual environment".into(),
    );

    m.insert(
        "misc.show_env.not_active".into(),
        "Virtual environment is".into(),
    );

    m.insert("misc.show_env.not_set".into(), "(not set)".into());

    m.insert("misc.show_env.os_label".into(), "OS:".into());

    m.insert(
        "misc.show_env.other_header".into(),
        "\u{1F527} Other Python/Venv Variables:".into(),
    );

    m.insert(
        "misc.show_env.path_full".into(),
        "\u{1F4C2} PATH (full):".into(),
    );

    m.insert(
        "misc.show_env.path_header".into(),
        "\u{1F4C2} PATH (first few entries):".into(),
    );

    m.insert("misc.show_env.shell_label".into(), "Shell:".into());

    m.insert(
        "misc.show_env.summary_header".into(),
        "\u{1F4CA} Summary:".into(),
    );

    m.insert(
        "misc.show_env.sys_header".into(),
        "\u{1F4BB} System Information:".into(),
    );

    m.insert(
        "misc.show_env.venv_header".into(),
        "\u{1F40D} Python Virtual Environment:".into(),
    );

    m.insert("misc.size.bytes_label".into(), "Bytes:".into());

    m.insert(
        "misc.size.calculating".into(),
        "Calculating directory size...".into(),
    );

    m.insert("misc.size.human_label".into(), "Human:".into());

    m.insert("misc.size.path_label".into(), "Path:".into());

    m.insert("misc.size.time_label".into(), "Time:".into());

    m.insert("misc.size.title".into(), "Current Directory Size".into());

    m.insert(
        "misc.tp.example_info".into(),
        "Example: tp info my_project".into(),
    );

    m.insert("misc.tp.info_canonical".into(), "Canonical".into());

    m.insert("misc.tp.info_exists".into(), "Exists".into());

    m.insert("misc.tp.info_global".into(), "global".into());

    m.insert("misc.tp.info_index".into(), "Index".into());

    m.insert("misc.tp.info_name".into(), "Name".into());

    m.insert("misc.tp.info_path".into(), "Path".into());

    m.insert("misc.tp.info_source".into(), "Source".into());

    m.insert(
        "misc.tp.info_title".into(),
        "\u{1F4CC} Teleport Savepoint Info".into(),
    );

    m.insert(
        "misc.tp.invalid_format".into(),
        "Invalid format. Use: tp rnm <old> to <new>".into(),
    );

    m.insert(
        "misc.tp.not_found".into(),
        "Savepoint '{}' not found".into(),
    );

    m.insert(
        "misc.tp.type_help".into(),
        "Type 'tp help' for usage.".into(),
    );

    m.insert(
        "misc.tp.unknown_subcmd".into(),
        "Unknown tp subcommand: {}".into(),
    );

    m.insert(
        "misc.tp.usage_add".into(),
        "Usage: tp add <name> [path]".into(),
    );

    m.insert("misc.tp.usage_info".into(), "Usage: tp info <name>".into());

    m.insert(
        "misc.tp.usage_jump".into(),
        "Usage: tp jump <name|number>".into(),
    );

    m.insert(
        "misc.tp.usage_rm".into(),
        "Usage: tp rm <name|number>[, <name|number>, ...]".into(),
    );

    m.insert(
        "misc.tp.usage_rnm".into(),
        "Usage: tp rnm <old_name> to <new_name>".into(),
    );

    m.insert("misc.tpb.unknown".into(), "Unknown tpb option: {}".into());

    m.insert("misc.tpb.usage".into(), "Usage:".into());

    m.insert(
        "misc.tpb.usage_basic".into(),
        " tpb - Teleport back to previous location".into(),
    );

    m.insert(
        "misc.tpb.usage_clear".into(),
        " tpb clear - Clear teleport history".into(),
    );

    m.insert(
        "misc.tpb.usage_history".into(),
        " tpb history - Show teleport history".into(),
    );

    m.insert(
        "misc.ui.removed".into(),
        "The modern UI was removed. Only the classic shell is available.".into(),
    );

    m.insert("misc.venva.activated".into(), "Activated: {}".into());

    m.insert(
        "misc.venva.activating".into(),
        "Activating virtual environment: {}".into(),
    );

    m.insert(
        "misc.venva.active_info".into(),
        "Virtual environment is now active. pip and python commands will use this environment."
            .into(),
    );

    m.insert(
        "misc.venva.already_in".into(),
        "Already in a virtual environment. Use 'deactivate' to exit first.".into(),
    );

    m.insert("misc.venva.failed".into(), "Failed to activate: {}".into());

    m.insert(
        "misc.venva.no_script".into(),
        "No activate script found in: {}".into(),
    );

    m.insert(
        "misc.venva.not_found".into(),
        "No virtual environment found. Usage: python_activate <venv_name>".into(),
    );

    m.insert(
        "misc.venvd.deactivated".into(),
        "Deactivated virtual environment".into(),
    );

    m.insert(
        "misc.venvd.not_in_venv".into(),
        "Not in a virtual environment".into(),
    );
}
