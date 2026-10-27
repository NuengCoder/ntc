use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("editor.already_root".into(), "Already at root".into());

    m.insert(
        "editor.auto_save_off".into(),
        "Auto-save on exit: OFF".into(),
    );

    m.insert("editor.auto_save_on".into(), "Auto-save on exit: ON".into());

    m.insert(
        "editor.cleared_cursors".into(),
        "Cleared extra cursors".into(),
    );

    m.insert("editor.clip_copied_chars".into(), "Copied {} chars".into());

    m.insert("editor.clip_copied_line".into(), "Copied line".into());

    m.insert("editor.clip_cut_chars".into(), "Cut {} chars".into());

    m.insert("editor.clip_cut_line".into(), "Cut line".into());

    m.insert("editor.clip_empty".into(), "Clipboard is empty".into());

    m.insert("editor.clip_yanked".into(), "Line yanked".into());

    m.insert("editor.clipboard_empty".into(), "Clipboard is empty".into());

    m.insert("editor.close_btn".into(), "\u{2715}".into());

    m.insert(
        "editor.cmd_already_at_root".into(),
        "Already at root".into(),
    );

    m.insert(
        "editor.cmd_auto_off".into(),
        "Auto-save on exit: OFF".into(),
    );

    m.insert("editor.cmd_auto_on".into(), "Auto-save on exit: ON".into());

    m.insert("editor.cmd_cd_prefix".into(), "cd:".into());

    m.insert("editor.cmd_color_off".into(), "Color: OFF".into());

    m.insert("editor.cmd_color_on".into(), "Color: ON".into());

    m.insert("editor.cmd_created_dir".into(), "Created directory:".into());

    m.insert("editor.cmd_current_theme".into(), "Current theme:".into());

    m.insert(
        "editor.cmd_exited_hex".into(),
        "Exited hex mode (changes discarded)".into(),
    );

    m.insert(
        "editor.cmd_gos_only".into(),
        "gos: only {} subdirectories".into(),
    );

    m.insert("editor.cmd_gos_usage".into(), "gos: usage gos <N>".into());

    m.insert(
        "editor.cmd_goto_usage".into(),
        "goto: usage goto <addr> or goto 0x<hex>".into(),
    );

    m.insert(
        "editor.cmd_hex_save_failed".into(),
        "Hex save failed:".into(),
    );

    m.insert("editor.cmd_hex_saved".into(), "Hex saved".into());

    m.insert("editor.cmd_jumped_to_line".into(), "Jumped to line".into());

    m.insert(
        "editor.cmd_jumped_to_offset".into(),
        "Jumped to offset".into(),
    );

    m.insert("editor.cmd_mkd_failed".into(), "mkd:".into());

    m.insert(
        "editor.cmd_mkd_missing".into(),
        "mkd: missing operand".into(),
    );

    m.insert("editor.cmd_mkdir_failed".into(), "mkdir:".into());

    m.insert(
        "editor.cmd_mkdir_missing".into(),
        "mkdir: missing operand".into(),
    );

    m.insert("editor.cmd_mkf_failed".into(), "mkf:".into());

    m.insert(
        "editor.cmd_mkf_missing".into(),
        "mkf: missing operand".into(),
    );

    m.insert(
        "editor.cmd_no_cwd".into(),
        "Cannot get current directory".into(),
    );

    m.insert("editor.cmd_no_subdirs".into(), "No subdirectories".into());

    m.insert("editor.cmd_no_tp".into(), "No teleport savepoints".into());

    m.insert("editor.cmd_not_dir".into(), "is not a directory".into());

    m.insert("editor.cmd_not_exist".into(), "does not exist".into());

    m.insert("editor.cmd_removed".into(), "Removed:".into());

    m.insert("editor.cmd_removed_dir".into(), "Removed directory:".into());

    m.insert("editor.cmd_rmd".into(), "rmd:".into());

    m.insert(
        "editor.cmd_rmd_missing".into(),
        "rmd: missing operand".into(),
    );

    m.insert("editor.cmd_rmf".into(), "rmf:".into());

    m.insert("editor.cmd_rmf_failed".into(), "rmf:".into());

    m.insert(
        "editor.cmd_rmf_missing".into(),
        "rmf: missing operand".into(),
    );

    m.insert(
        "editor.cmd_run_missing".into(),
        "run: missing command".into(),
    );

    m.insert(
        "editor.nerun_no_file".into(),
        "run: save the file before running".into(),
    );

    m.insert(
        "editor.nerun_save_failed".into(),
        "run: save failed: {}".into(),
    );

    m.insert(
        "editor.nerun_no_runner".into(),
        "run: no runner for .{} — edit {}".into(),
    );

    m.insert("editor.nerun_running".into(), "Running {}: {}".into());

    m.insert("editor.cmd_save_failed".into(), "Save failed:".into());

    m.insert("editor.cmd_saved".into(), "Saved".into());

    m.insert(
        "editor.cmd_syntax_off".into(),
        "Syntax highlighting: OFF".into(),
    );

    m.insert(
        "editor.cmd_syntax_on".into(),
        "Syntax highlighting: ON".into(),
    );

    m.insert(
        "editor.cmd_theme_list_hint".into(),
        "Use 'theme list' to see available themes.".into(),
    );

    m.insert(
        "editor.cmd_theme_not_found".into(),
        "Theme not found".into(),
    );

    m.insert(
        "editor.cmd_theme_switched".into(),
        "Switched to theme".into(),
    );

    m.insert("editor.cmd_themes".into(), "Themes:".into());

    m.insert("editor.cmd_touch_failed".into(), "touch:".into());

    m.insert(
        "editor.cmd_touch_missing".into(),
        "touch: missing operand".into(),
    );

    m.insert("editor.cmd_tp_add_error".into(), "tp-add:".into());

    m.insert(
        "editor.cmd_tp_add_missing".into(),
        "tp-add: missing name".into(),
    );

    m.insert("editor.cmd_tp_added".into(), "Savepoint created".into());

    m.insert("editor.cmd_tp_error".into(), "tp:".into());

    m.insert("editor.cmd_tp_jumped".into(), "Teleported to".into());

    m.insert("editor.cmd_tp_list".into(), "Teleports:".into());

    m.insert(
        "editor.cmd_tp_not_found".into(),
        "Savepoint not found:".into(),
    );

    m.insert("editor.cmd_unknown".into(), "Unknown command:".into());

    m.insert(
        "editor.cmd_visual_hint".into(),
        "Visual: arrows select, Alt+hjkl/arrows moves selection, Esc exits".into(),
    );

    m.insert(
        "editor.visual_dedent_blocked".into(),
        "Alt+h needs indent on every selected line".into(),
    );

    m.insert(
        "editor.visual_backspace_no_space".into(),
        "Backspace removes only a space before the selection".into(),
    );

    m.insert("editor.color_off".into(), "Color: OFF".into());

    m.insert("editor.color_on".into(), "Color: ON".into());

    m.insert("editor.compl_builtin".into(), "[built-in]".into());

    m.insert("editor.compl_math_abs".into(), "absolute value".into());

    m.insert(
        "editor.compl_math_acos".into(),
        "arccosine (radians)".into(),
    );

    m.insert(
        "editor.compl_math_acot".into(),
        "arccotangent (radians)".into(),
    );

    m.insert(
        "editor.compl_math_acsc".into(),
        "arccosecant (radians)".into(),
    );

    m.insert(
        "editor.compl_math_asec".into(),
        "arcsecant (radians)".into(),
    );

    m.insert("editor.compl_math_asin".into(), "arcsine (radians)".into());

    m.insert(
        "editor.compl_math_atan".into(),
        "arctangent (radians)".into(),
    );

    m.insert("editor.compl_math_avg".into(), "average of values".into());

    m.insert("editor.compl_math_ceil".into(), "ceiling (round up)".into());

    m.insert("editor.compl_math_cos".into(), "cosine (radians)".into());

    m.insert("editor.compl_math_cot".into(), "cotangent (radians)".into());

    m.insert("editor.compl_math_csc".into(), "cosecant (radians)".into());

    m.insert(
        "editor.compl_math_floor".into(),
        "floor (round down)".into(),
    );

    m.insert("editor.compl_math_ln".into(), "natural logarithm".into());

    m.insert("editor.compl_math_log10".into(), "base-10 logarithm".into());

    m.insert("editor.compl_math_log2".into(), "base-2 logarithm".into());

    m.insert("editor.compl_math_max".into(), "maximum of values".into());

    m.insert("editor.compl_math_min".into(), "minimum of values".into());

    m.insert("editor.compl_math_pow".into(), "power (x^y)".into());

    m.insert("editor.compl_math_rand".into(), "random number".into());

    m.insert("editor.compl_math_round".into(), "round to nearest".into());

    m.insert("editor.compl_math_sec".into(), "secant (radians)".into());

    m.insert("editor.compl_math_sin".into(), "sine (radians)".into());

    m.insert("editor.compl_math_sqrt".into(), "square root".into());

    m.insert("editor.compl_math_sum".into(), "sum of values".into());

    m.insert("editor.compl_math_tan".into(), "tangent (radians)".into());

    m.insert(
        "editor.compl_math_tobinary".into(),
        "convert to binary".into(),
    );

    m.insert(
        "editor.compl_math_todecimal".into(),
        "convert to decimal".into(),
    );

    m.insert(
        "editor.compl_math_tohb".into(),
        "convert to human-readable bytes".into(),
    );

    m.insert("editor.compl_math_tohex".into(), "convert to hex".into());

    m.insert(
        "editor.compl_math_tooctal".into(),
        "convert to octal".into(),
    );

    m.insert("editor.compl_other".into(), "[other]".into());

    m.insert("editor.compl_print_values".into(), "print value(s)".into());

    m.insert("editor.compl_return_value".into(), "return a value".into());

    m.insert("editor.completion_title".into(), "completions".into());

    m.insert("editor.copied_line".into(), "Copied line".into());

    m.insert("editor.cut_line".into(), "Cut line".into());

    m.insert(
        "editor.edit_added_cursor".into(),
        "Added cursor ({} total)".into(),
    );

    m.insert(
        "editor.edit_already_selected".into(),
        "Already selected".into(),
    );

    m.insert(
        "editor.edit_no_more_occurrences".into(),
        "No more occurrences".into(),
    );

    m.insert("editor.edit_wrap".into(), "wrap".into());

    m.insert(
        "editor.exited_hex".into(),
        "Exited hex mode (changes discarded)".into(),
    );

    m.insert(
        "editor.exited_multicursor".into(),
        "Exited multi-cursor mode".into(),
    );

    m.insert(
        "editor.ff_prompt_find_empty".into(),
        "Find: Type to search...".into(),
    );

    m.insert("editor.ff_prompt_find_prefix".into(), "Find:".into());

    m.insert("editor.ff_prompt_gs_prefix".into(), "gs:".into());

    m.insert(
        "editor.file_changed_reload".into(),
        "File changed externally \u{2014} reloaded".into(),
    );

    m.insert("editor.gosc_exit_hint".into(), "0=exit ".into());

    m.insert(
        "editor.gosc_hint".into(),
        " Enter=go -N=back N Esc=close ".into(),
    );

    m.insert(
        "editor.gosc_title".into(),
        " gosc \u{2014} Navigate Continuously ".into(),
    );

    m.insert("editor.gs_loading".into(), " finding... {} pls wait".into());

    m.insert("editor.help_screen".into(), " NORMAL MODE (vim-style verbs + motion)\n i=insert v=visual q=query x=save n=file m=dir c=copy e=nfe f=find b=files t=term u=encoding g=gmode :=cmd ?=help 0=exit(dirty-checked)\n hjkl/awsd/arrows move \u{B7} Home/End line \u{B7} PgUp/PgDn page \u{B7} ^J=jump\n\n GMODE (press g, type, Enter)\n t/T=next/prev tab qt=close tab aqt=close all N=goto line q=exit u=undo r=redo n=file m=dir i=insert v=visual e=nfe Esc=cancel\n\n NFE SIDEBAR (press e)\n hjkl/arrows/wasd move \u{B7} Enter=dir-cd/file-open(stay) \u{B7} i=insert v=visual \u{B7} Esc=back\n\n INSERT MODE (editing)\n type to insert \u{B7} arrows move \u{B7} Esc=normal ^S=save ^Z=undo ^R=redo ^D=multi ^F=find ^N=files ^P=paste ^T=term ^G=gmode ^E=nfe ^W=visual ^:=cmd \u{B7} Home/End line \u{B7} PgUp/PgDn page \u{B7} F3/Sh+F3 find next/prev\n\n VISUAL MODE (selection)\n hjkl/arrows move (Shift selects) \u{B7} Alt+hjkl/arrows moves selection (j/k) or indents it (h/l) \u{B7} s=save ^L=select line d=next match (keeps the old one lit) w=clear those q=query i=insert u=undo r=redo a=select-all toggle c=copy x=cut p=paste :=cmd e=nfe f=find g=gmode \u{B7} Esc=exit\n\n COMMAND MODE\n :<number> go to line number\n :l N or :line N go to line N\n :q or :quit quit editor (with save prompt)\n :wq save and quit\n :w or :write save file\n :auto on enable auto-save\n :auto off disable auto-save\n :syntax on enable syntax highlighting\n :syntax off disable syntax highlighting\n :color on|off toggle color output\n :cd <path> change directory (Navigator)\n :back [N] go to parent directory (N levels)\n :gos <N> navigate to Nth subdirectory\n :gosc list numbered subdirectories\n :mkdir <dir> create directory\n :mkd <dir> create directory (alias for :mkdir)\n :touch <file> create new file\n :mkf <file> create new file (alias for :touch)\n :rmd <target> recursively delete directory\n :rmf <target> force delete file or directory\n :rnmf <src> <dst> rename file\n :rnmd <src> <dst> rename directory\n :ne <file> navigate & edit (create if missing)\n :ne . show CWD in sidebar\n :e open file explorer (nfe)\n :theme <name> switch to theme by name\n :theme list list available themes\n :theme show current theme\n :git diff or :gd show git diff of current file\n :git blame or :blame show blame of current line\n :tp <name> jump to teleport savepoint\n :tp-add <name> save CWD as teleport\n :gs <pattern> search file contents across project\n :run run current file via nerun.toml in terminal (extension-mapped, editable)\n :u undo last op\n :r redo last op\n :! <cmd> run command in terminal\n Esc cancel\n\n QUERY MODE (q in Normal, Ctrl+Alt+Q in Insert, :x in Command — leading / optional)\n /s 'text' select next occurrence after cursor (wraps); bare /s selects word under cursor\n /l N go to line N (cursor to line start)\n /sa 'text' select all occurrences in buffer\n /sal select all lines\n /sl N select line N; /sl A B range A..B (any order); /sl A , B , … discrete lines\n /rp <find> <replace> replace in selection (selection required; quote multi-word: /rp 'a b' 'c')\n /add <text> to #end|#start|#hm append/prepend on selected lines\n /rm <text> from #end|#start|#hm strip prefix/suffix on selected lines (selection required)\n /set <pat> to #end|#start|#hm align first occurrence of pat (selection required)\n /c | /cs clear selection\n /z [#hm|#end|#start] add cursor per selected line, then Insert (default #hm)\n /up uppercase selection (alias /upper)\n /low lowercase selection (alias /lower)\n /qal add <n> <q> add query alias (e.g. /qal sw /rp ${1} ${2})\n /qal rm <n> remove alias\n /qal info <n> show alias\n /qal edit <n> <q> update alias\n /qal rnm <a> to <b> rename alias\n /qal list list aliases\n <name> [args] run alias directly (no / needed)\n /undo undo\n /redo redo\n /exit /quit /Esc exit Query Mode\n ; chain multiple commands\n ${1} ${2} alias positional params\n ${1:-default} alias param with default\n ${1[]} alias array param (all remaining args)\n\n TERMINAL / RUN PANEL\n Ctrl+T toggle terminal panel\n :run <command> execute command, opens persistent run panel\n mouse click panel to focus, click editor to return\n Esc (in panel) run: close | terminal: return to editor\n drag \u{2500}\u{2500}\u{2500} row resize panel height".into());

    m.insert(
        "editor.hex_cannot_read".into(),
        "hex: cannot read file: {}".into(),
    );

    m.insert(
        "editor.hex_mode_hint".into(),
        "Hex mode — 0-9/a-f edit bytes  u=undo  Ctrl+R=redo  :=command  Esc=exit".into(),
    );

    m.insert("editor.hex_saved".into(), "Hex saved".into());

    m.insert(
        "editor.hex_unsaved".into(),
        "Unsaved hex changes. Use :w to save, :q! to discard.".into(),
    );

    m.insert(
        "editor.hex_unsaved_warning".into(),
        "Unsaved hex changes. Use :w to save, :q! to discard.".into(),
    );

    m.insert(
        "editor.hint_command_actions".into(),
        "Enter=exec Tab=next hint \u{2192}=accept \u{2190}\u{2192}=caret Esc=cancel".into(),
    );

    m.insert(
        "editor.hint_command_nav".into(),
        "cd|back|gos|gs|mkdir|touch|ne|tp|tp-add".into(),
    );

    m.insert(
        "editor.hint_file_finder_results".into(),
        "results \u{2191}\u{2193}=nav Enter=open Esc=close".into(),
    );

    m.insert(
        "editor.hint_find".into(),
        "Find (file.ext:<line> jumps to a line): {}".into(),
    );

    m.insert("editor.hint_find_prefix".into(), "Find \u{2014}".into());

    m.insert("editor.hint_gosc".into(), "Go to: {}".into());

    m.insert("editor.hint_gosc_close".into(), "Esc=close".into());

    m.insert(
        "editor.hint_gosc_nav".into(),
        "dirs type number+Enter=nav 0=exit".into(),
    );

    m.insert("editor.hint_gosc_prefix".into(), "gosc \u{2014}".into());

    m.insert("editor.hint_gs_close".into(), "Esc=close".into());

    m.insert("editor.hint_gs_done".into(), "Done: {} matches".into());

    m.insert("editor.hint_gs_done_prefix".into(), "gs \u{2014}".into());

    m.insert(
        "editor.hint_gs_loading".into(),
        "gs \u{2014} searching...".into(),
    );

    m.insert("editor.hint_help".into(), " Help \u{2014} Esc/q=close \u{00B7} hjkl/awsd/arrows move \u{00B7} Alt+\u{2190}\u{2192}=pan".into());

    m.insert(
        "editor.hint_hex_actions".into(),
        "0-9/a-f=edit u=undo ^R=redo :=cmd :w=save Esc=exit".into(),
    );

    m.insert(
        "editor.hint_hex_edit".into(),
        "Hex edit — type hex bytes".into(),
    );

    m.insert("editor.hint_insert".into(), " Insert mode \u{2014} Esc=normal ^S=save ^Z=undo ^R=redo ^D=multi ^F=find ^N=files ^P=paste ^T=term ^G=gmode ^E=nfe ^W=visual ^:=cmd".into());

    m.insert("editor.hint_insert_msg".into(), " Esc=normal".into());

    m.insert(
        "editor.hint_insert_multi".into(),
        " Insert* \u{2014} Esc=exit multi-cursor ^D=nextOcc".into(),
    );

    m.insert(
        "editor.hint_insert_multi_msg".into(),
        " Esc=exit multi-cursor".into(),
    );

    m.insert("editor.hint_jump".into(), "Go to line \u{2014}".into());

    m.insert(
        "editor.hint_jump_actions".into(),
        "Enter=jump Esc=cancel".into(),
    );

    m.insert("editor.hint_gmode".into(), "g".into());

    m.insert("editor.hint_gmode_actions".into(), "t/T=tabs t<n>=goto tab qt/aqt=close N=goto q=exit u=undo r=redo n=file m=dir i=insert v=visual e=nfe Enter=run Tab=next hint \u{2192}=accept Esc=cancel".into());

    m.insert(
        "editor.hint_create_actions".into(),
        "Enter=create Esc=cancel".into(),
    );

    m.insert(
        "editor.term_run_usage".into(),
        "Usage: ! <command> (run in terminal)".into(),
    );

    m.insert(
        "editor.gmode_unknown".into(),
        "Unknown gmode command: '{}'".into(),
    );

    m.insert("editor.hint_normal".into(), " i=insert v=visual f=find b=files e=nfe t=term g=gmode x=save n=file m=dir c=copy Sh=select :cmd ?=help 0=exit".into());

    m.insert(
        "editor.hint_normal_msg".into(),
        " i=insert v=visual :cmd ?=help 0=exit".into(),
    );

    m.insert("editor.hint_query".into(), "Query: {}".into());

    m.insert("editor.hint_query_actions".into(), "Actions: /s next, /l goto, /sa all, /sal lines, /sl line, /rp replace, /add, /rm, /set, /z cursors, /up, /low, /qal, /c clear · Tab=next hint \u{2192}=accept".into());

    m.insert(
        "editor.query_z_usage".into(),
        "/z [#hm|#end|#start] (default #hm)".into(),
    );

    m.insert(
        "editor.query_cursors_added".into(),
        "Added {} cursor(s)".into(),
    );

    m.insert("editor.hint_query_prefix".into(), "/{} — {}".into());

    m.insert("editor.hint_results".into(), "Results: {}".into());

    m.insert(
        "editor.hint_results_so_far".into(),
        "Results: {} so far".into(),
    );

    m.insert(
        "editor.hint_run_empty".into(),
        "Run output \u{2014} (empty)".into(),
    );

    m.insert(
        "editor.hint_run_entry".into(),
        "Enter=exec Esc=cancel".into(),
    );

    m.insert(
        "editor.hint_run_output".into(),
        "Run output \u{2014} lines".into(),
    );

    m.insert(
        "editor.hint_run_scroll".into(),
        ":=cmd \u{2191}\u{2193}/j/k=scroll Esc=close".into(),
    );

    m.insert("editor.hint_search".into(), "Search:".into());

    m.insert(
        "editor.hint_search_actions".into(),
        "Enter=next Shift+Enter=prev Esc=close".into(),
    );

    m.insert(
        "editor.hint_term_actions".into(),
        "lines PgUp/PgDn=scroll Ctrl+Shift+C=copy Ctrl+C=interrupt".into(),
    );

    m.insert("editor.hint_term_close".into(), "Esc/q=close".into());

    m.insert("editor.hint_term_exited".into(), "Terminal exited".into());

    m.insert("editor.hint_term_lines".into(), "Terminal \u{2014}".into());

    m.insert("editor.hint_terminal".into(), "Terminal — {} — {}".into());

    m.insert("editor.hint_visual".into(), " hjkl=move Sh=select i=insert s=save ^L=line d=next q=query w=clear u=undo r=redo a=all c=copy x=cut p=paste Esc=exit".into());

    m.insert("editor.hint_visual_no_msg".into(), " Visual mode \u{2014} hjkl/arrows move \u{00B7} Shift selects \u{00B7} s=save ^L=select line d=next match q=query w=clear marks \u{00B7} a=all c=copy x=cut p=paste".into());

    m.insert("editor.hover_title".into(), " info ".into());

    m.insert(
        "editor.hyper_created".into(),
        "Hyper project '{}' created successfully!".into(),
    );

    m.insert(
        "editor.hyper_creating".into(),
        "Creating hyper project '{}' (C)...".into(),
    );

    m.insert(
        "editor.hyper_dir_exists".into(),
        "Directory '{}' already exists.".into(),
    );

    m.insert(
        "editor.hyper_name_with_ext".into(),
        "Project name '{}' contains an extension. Use a name without dots.".into(),
    );

    m.insert(
        "editor.hyper_unsupported_lang".into(),
        "Unsupported language '{}'. Only 'c' is supported.".into(),
    );

    m.insert("editor.line_deleted".into(), "Line deleted".into());

    m.insert("editor.line_selected".into(), "Line selected".into());

    m.insert("editor.line_yanked".into(), "Line yanked".into());

    m.insert("editor.max_tabs".into(), "Max tabs ({}) reached".into());

    m.insert("editor.mkd_missing".into(), "mkd: missing operand".into());

    m.insert(
        "editor.mkdir_missing".into(),
        "mkdir: missing operand".into(),
    );

    m.insert("editor.mkf_missing".into(), "mkf: missing operand".into());

    m.insert("editor.mode.command".into(), "[Command]".into());

    m.insert("editor.mode.finder".into(), "[Finder]".into());

    m.insert("editor.mode.gosc".into(), "[Gosc]".into());

    m.insert("editor.mode.help".into(), "[Help]".into());

    m.insert("editor.mode.hex".into(), "[HEX]".into());

    m.insert("editor.mode.insert".into(), "[Insert]".into());

    m.insert("editor.mode.insert_star".into(), "[Insert *]".into());

    m.insert("editor.mode.jump".into(), "[Jump]".into());

    m.insert("editor.mode.normal".into(), "[Normal]".into());

    m.insert("editor.mode.query".into(), "[Query]".into());

    m.insert("editor.mode.run".into(), "[Run]".into());

    m.insert("editor.mode.search".into(), "[Search]".into());

    m.insert("editor.mode.term".into(), "[Term]".into());

    m.insert("editor.mode.visual".into(), "[Visual]".into());

    m.insert(
        "editor.mode_cleared_cursors".into(),
        "Cleared extra cursors".into(),
    );

    m.insert("editor.mode_command".into(), "[Command]".into());

    m.insert(
        "editor.mode_exited_multi".into(),
        "Exited multi-cursor mode".into(),
    );

    m.insert("editor.mode_finder".into(), "[Finder]".into());

    m.insert("editor.mode_encoding".into(), "[Encoding]".into());

    m.insert("editor.mode_git_diff".into(), "[GitDiff]".into());

    m.insert(
        "editor.git_no_diff".into(),
        "git: no diff for this file".into(),
    );

    m.insert(
        "editor.git_no_blame".into(),
        "git: no blame info for this line".into(),
    );

    m.insert(
        "editor.git_disabled".into(),
        "git integration is disabled (config git ON)".into(),
    );

    m.insert("editor.git_quit_hint".into(), "Esc/q to close".into());

    m.insert("editor.mode_gosc".into(), "[Gosc]".into());

    m.insert("editor.mode_help".into(), "[Help]".into());

    m.insert("editor.mode_hex".into(), "[HEX]".into());

    m.insert("editor.mode_insert".into(), "[Insert]".into());

    m.insert("editor.mode_insert_multi".into(), "[Insert *]".into());

    m.insert("editor.mode_jump".into(), "[Jump]".into());

    m.insert(
        "editor.mode_jump_range".into(),
        "Line {} out of range (1-{})".into(),
    );

    m.insert("editor.mode_normal".into(), "[Normal]".into());

    m.insert("editor.mode_gmode".into(), "[Gmode]".into());

    m.insert("editor.mode_nfe".into(), "[Nfe]".into());

    m.insert("editor.hint_nfe".into(), " hjkl/wasd/arrows move · Enter open · n new file · m new dir · r rename · d delete · i/v edit · :=size/tabs · Esc back".into());

    m.insert("editor.hint_nfe_cmd_actions".into(), "Enter=apply Tab=next hint \u{2192}=accept \u{2190}\u{2192}=caret Esc=cancel \u{00B7} :i/:v edit".into());

    m.insert("editor.nfe_unknown".into(), "Unknown nfe command: '{}' (try <number> | size full|def|N% | n <file> | m <dir> | rmd <dir> | rmf <file> | i | v)".into());

    m.insert("editor.nfe_jumped".into(), "Jumped to entry {}: {}".into());

    m.insert(
        "editor.nfe_jump_range".into(),
        "No sidebar entry {} (1-{})".into(),
    );

    m.insert(
        "editor.nfe_rm_usage".into(),
        "Usage: rmd <dir> | rmf <file> (names relative to sidebar root)".into(),
    );

    m.insert(
        "editor.nfe_rmd_not_dir".into(),
        "Not a directory: {}".into(),
    );

    m.insert(
        "editor.nfe_rmf_is_dir".into(),
        "Is a directory (use rmd): {}".into(),
    );

    m.insert("editor.nfe_rm_not_exist".into(), "Not found: {}".into());

    m.insert("editor.nfe_removed".into(), "Removed: {}".into());

    m.insert("editor.nfe_rm_failed".into(), "Remove failed: {}".into());

    m.insert(
        "editor.nfe_rm_has_unsaved".into(),
        "Unsaved edits in '{}' — save first".into(),
    );

    m.insert(
        "editor.nfe_confirm".into(),
        "Delete '{}' (y) confirm, (n) cancel? ".into(),
    );

    m.insert(
        "editor.nfe_rm_root".into(),
        "Cannot delete the sidebar root.".into(),
    );

    m.insert(
        "editor.nfe_new_usage".into(),
        "Usage: n <file> | m <dir> (names relative to sidebar root)".into(),
    );

    m.insert("editor.nfe_created_file".into(), "Created file: {}".into());

    m.insert(
        "editor.nfe_created_dir".into(),
        "Created directory: {}".into(),
    );

    m.insert(
        "editor.nfe_create_exists".into(),
        "Already exists: {}".into(),
    );

    m.insert(
        "editor.nfe_create_failed".into(),
        "Create failed: {}".into(),
    );

    m.insert("editor.nfe_box_new_file".into(), "New file".into());

    m.insert("editor.nfe_box_new_dir".into(), "New folder".into());

    m.insert("editor.nfe_box_rename".into(), "Rename".into());

    m.insert(
        "editor.nfe_box_actions".into(),
        "Enter=ok Esc=cancel".into(),
    );

    m.insert("editor.nfe_renamed".into(), "Renamed to: {}".into());

    m.insert(
        "editor.nfe_rename_root".into(),
        "Select a file or folder to rename".into(),
    );

    m.insert(
        "editor.nfe_rename_failed".into(),
        "Rename failed: {}".into(),
    );

    m.insert(
        "editor.nfe_size_usage".into(),
        "Usage: size full|def|N% (got '{}')".into(),
    );

    m.insert("editor.nfe_size_set".into(), "Sidebar width: {}".into());

    m.insert("editor.nfe_size_default".into(), "default (30 cols)".into());

    m.insert("editor.nfe_size_full".into(), "full".into());

    m.insert("editor.mode_query".into(), "[Query]".into());

    m.insert("editor.mode_run".into(), "[Run]".into());

    m.insert("editor.mode_run_err".into(), "run: {}".into());

    m.insert("editor.mode_search".into(), "[Search]".into());

    m.insert("editor.mode_term_copied".into(), "Copied {} chars".into());

    m.insert("editor.mode_term_busy_esc".into(), "A command is still running - Esc returns to the editor and keeps the terminal (Ctrl+Q hides it)".into());

    m.insert("editor.mode_terminal".into(), "[Term]".into());

    m.insert(
        "editor.mode_terminal_err".into(),
        "Terminal error: {}".into(),
    );

    m.insert("editor.mode_visual".into(), "[Visual]".into());

    m.insert(
        "editor.mode_visual_start".into(),
        "Visual mode — arrow keys to select, Esc to exit".into(),
    );

    m.insert(
        "editor.mode_zoom_hint".into(),
        "Zoom: Ctrl++ / Ctrl+- in terminal emulator".into(),
    );

    m.insert("editor.modified_marker".into(), " \u{25CF}".into());

    m.insert("editor.mouse_closed_all".into(), "Closed all tabs".into());

    m.insert("editor.nav_line_selected".into(), "Line selected".into());

    m.insert("editor.nav_selected_all".into(), "Selected all".into());

    m.insert(
        "editor.nav_split_cursor".into(),
        "Split cursor ({} total)".into(),
    );

    m.insert("editor.no_subdirs".into(), "No subdirectories".into());

    m.insert(
        "editor.no_teleports".into(),
        "No teleport savepoints".into(),
    );

    m.insert("editor.nothing_redo".into(), "Nothing to redo".into());

    m.insert("editor.nothing_undo".into(), "Nothing to undo".into());

    m.insert("editor.opened_file".into(), "Opened {}".into());

    m.insert("editor.panel_executing".into(), "EXECUTING:".into());

    m.insert("editor.panel_lines".into(), "lines".into());

    m.insert("editor.panel_run".into(), "RUN:".into());

    m.insert(
        "editor.panel_run_hint".into(),
        "j/k scroll, Esc/q close".into(),
    );

    m.insert("editor.panel_run_output".into(), "RUN OUTPUT".into());

    m.insert("editor.panel_terminal".into(), "TERMINAL".into());

    m.insert(
        "editor.pattern_not_found".into(),
        "Pattern not found: {}".into(),
    );

    m.insert(
        "editor.query_add_format".into(),
        "Format: /add <text> to #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_add_position".into(),
        "Position must be #end, #start, or #hm".into(),
    );

    m.insert(
        "editor.query_add_usage".into(),
        "/add <text> to #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_added_text".into(),
        "Added text to {} line(s)".into(),
    );

    m.insert(
        "editor.query_rm_usage".into(),
        "/rm <text> from #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_rm_format".into(),
        "Format: /rm <text> from #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_removed_text".into(),
        "Removed from {} line(s), skipped {}".into(),
    );

    m.insert(
        "editor.query_alias_not_found".into(),
        "Alias not found: {}".into(),
    );

    m.insert(
        "editor.query_empty_pattern".into(),
        "Empty find pattern".into(),
    );

    m.insert(
        "editor.query_lowercased".into(),
        "Lowercased selection".into(),
    );

    m.insert(
        "editor.query_no_matches".into(),
        "No matches for '{}'".into(),
    );

    m.insert(
        "editor.query_no_pattern_matches".into(),
        "No lines contain '{}'".into(),
    );

    m.insert(
        "editor.query_qal_add_usage".into(),
        "/qal add <name> <query>".into(),
    );

    m.insert("editor.query_qal_added".into(), "Alias '{}' added".into());

    m.insert(
        "editor.query_qal_edit_usage".into(),
        "/qal edit <name> <new_query>".into(),
    );

    m.insert(
        "editor.query_qal_help".into(),
        "/qal: add|rm|info|edit|rnm|list|run".into(),
    );

    m.insert(
        "editor.query_qal_info_usage".into(),
        "/qal info <name>".into(),
    );

    m.insert("editor.query_qal_list".into(), "Aliases: {}".into());

    m.insert(
        "editor.query_qal_no_aliases".into(),
        "No query aliases".into(),
    );

    m.insert(
        "editor.query_qal_not_found".into(),
        "Alias '{}' not found".into(),
    );

    m.insert(
        "editor.query_qal_removed".into(),
        "Alias '{}' removed".into(),
    );

    m.insert(
        "editor.query_qal_renamed".into(),
        "Renamed '{}' to '{}'".into(),
    );

    m.insert("editor.query_qal_rm_usage".into(), "/qal rm <name>".into());

    m.insert(
        "editor.query_qal_rnm_usage".into(),
        "/qal rnm <old> to <new>".into(),
    );

    m.insert(
        "editor.query_qal_updated".into(),
        "Alias '{}' updated".into(),
    );

    m.insert(
        "editor.query_l_usage".into(),
        "/l <line> — go to line".into(),
    );

    m.insert(
        "editor.query_qal_exists".into(),
        "Alias '{}' already exists (use /qal edit)".into(),
    );

    m.insert(
        "editor.query_qal_reserved".into(),
        "Reserved name: '{}'".into(),
    );

    m.insert(
        "editor.query_replace_format".into(),
        "Format: /p <find> to <replace>".into(),
    );

    m.insert(
        "editor.query_rp_usage".into(),
        "/rp <find> <replace> — replace in selection".into(),
    );

    m.insert(
        "editor.query_s_usage".into(),
        "/s \"text\" — select next occurrence (bare /s = word under cursor)".into(),
    );

    m.insert(
        "editor.query_sa_usage".into(),
        "/sa \"text\" — select all occurrences".into(),
    );

    m.insert(
        "editor.query_selected_lines".into(),
        "Selected {} line(s)".into(),
    );

    m.insert(
        "editor.query_selected_next".into(),
        "Selected next '{}'".into(),
    );

    m.insert(
        "editor.query_selected_occurrences".into(),
        "Selected {} occurrence(s) of '{}'".into(),
    );

    m.insert("editor.query_selected_word".into(), "Selected word".into());

    m.insert(
        "editor.query_selection_required".into(),
        "Selection required (select with /s, /sa, /sl or /sal first)".into(),
    );

    m.insert(
        "editor.query_sl_usage".into(),
        "/sl <line> [line] | <line> , ... — select lines".into(),
    );

    m.insert(
        "editor.query_replace_usage".into(),
        "/p <find> to <replace>".into(),
    );

    m.insert(
        "editor.query_replaced_count".into(),
        "Replaced {} occurrence(s)".into(),
    );

    m.insert(
        "editor.query_selected_all_lines".into(),
        "Selected all lines".into(),
    );

    m.insert(
        "editor.query_selected_line".into(),
        "Selected line {}".into(),
    );

    m.insert(
        "editor.query_selected_lines_range".into(),
        "Selected lines {}-{}".into(),
    );

    m.insert(
        "editor.query_selection_cleared".into(),
        "Selection cleared".into(),
    );

    m.insert(
        "editor.query_set_alignment".into(),
        "Set alignment on {} line(s)".into(),
    );

    m.insert(
        "editor.query_set_format".into(),
        "Format: /set <pattern> to #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_set_usage".into(),
        "/set <pattern> to #end|#start|#hm".into(),
    );

    m.insert("editor.query_unknown".into(), "Unknown query: {}".into());

    m.insert(
        "editor.query_uppercased".into(),
        "Uppercased selection".into(),
    );

    m.insert("editor.redo".into(), "Redo".into());

    m.insert("editor.rmd_missing".into(), "rmd: missing operand".into());

    m.insert("editor.rmf_missing".into(), "rmf: missing operand".into());

    m.insert(
        "editor.run_exited".into(),
        "Process exited with code {}".into(),
    );

    m.insert("editor.run_missing".into(), "run: missing command".into());

    m.insert("editor.run_terminated".into(), "Process terminated".into());

    m.insert(
        "editor.save_auto_error".into(),
        "Auto-save error: {}".into(),
    );

    m.insert("editor.save_failed".into(), "Save failed: {}".into());

    m.insert("editor.encoding_changed".into(), "Encoding changed".into());

    m.insert("editor.enc_title".into(), "Encoding".into());

    m.insert(
        "editor.enc_hint".into(),
        "↑↓ select · Enter confirm · Esc cancel".into(),
    );

    m.insert("editor.hint_encoding".into(), "Encoding:".into());

    m.insert(
        "editor.encoding_reload_error".into(),
        "Reload failed: {}".into(),
    );

    m.insert(
        "editor.encoding_applied_on_save".into(),
        "applied on next save".into(),
    );

    m.insert("editor.save_prompt".into(), "Save file [Y|N]?".into());

    m.insert("editor.saved".into(), "Saved \u{2713}".into());

    m.insert("editor.saved_full".into(), " Saved ✓".into());

    m.insert("editor.saved_marker".into(), " \u{2713}".into());

    m.insert("editor.scratch".into(), "[scratch]".into());

    m.insert("editor.scratch_label".into(), "[scratch]".into());

    m.insert(
        "editor.search_pattern_not_found".into(),
        "Pattern not found: {}".into(),
    );

    m.insert("editor.selected_all".into(), "Selected all".into());

    m.insert("editor.status_modified".into(), "modified".into());

    m.insert("editor.status_modified_tag".into(), " \u{25cf}".into());

    m.insert("editor.status_saved".into(), "saved".into());

    m.insert("editor.status_saved_tag".into(), " \u{2713}".into());

    m.insert(
        "editor.syntax_off".into(),
        "Syntax highlighting: OFF".into(),
    );

    m.insert("editor.syntax_on".into(), "Syntax highlighting: ON".into());

    m.insert("editor.tab_cannot_open".into(), "Cannot open {}".into());

    m.insert(
        "editor.tab_error_reading".into(),
        "Error reading {}: {}".into(),
    );

    m.insert(
        "editor.tab_max_reached".into(),
        "Max tabs ({}) reached".into(),
    );

    m.insert("editor.tab_jump_range".into(), "No tab {} (1-{})".into());

    m.insert("editor.tab_no_tabs".into(), "No tabs to jump to".into());

    m.insert("editor.tab_opened".into(), "Opened {}".into());

    m.insert("editor.term_title".into(), "Terminal".into());

    m.insert("editor.tmpl_last_saved".into(), "Last saved: {}".into());

    m.insert(
        "editor.tmpl_recover_prompt".into(),
        "Recover unsaved changes? [Y/N]: ".into(),
    );

    m.insert(
        "editor.tmpl_recovery_found".into(),
        "Crash recovery file found for: {}".into(),
    );

    m.insert(
        "editor.touch_missing".into(),
        "touch: missing operand".into(),
    );

    m.insert(
        "editor.tp_add_missing".into(),
        "tp-add: missing name".into(),
    );

    m.insert("editor.undo".into(), "Undo".into());

    m.insert("editor.unknown_cmd".into(), "Unknown command: {}".into());

    m.insert("editor.untitled".into(), "untitled".into());

    m.insert("editor.untitled_label".into(), "untitled".into());

    m.insert(
        "editor.unsaved_switch".into(),
        "Unsaved changes in {} \u{2014} :w to save first (press again to switch anyway)".into(),
    );

    m.insert(
        "editor.visual_hint".into(),
        "Visual mode \u{2014} arrow keys to select, Esc to exit".into(),
    );

    m.insert(
        "editor.welcome.title".into(),
        "A fast, terminal-based code editor built in Rust.".into(),
    );

    m.insert(
        "editor.zoom_hint".into(),
        "Zoom: Ctrl++ / Ctrl+- in terminal emulator".into(),
    );
}
