use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "scr.calling_not_allowed".into(),
        "Nested scr calls are not allowed inside a script".into(),
    );

    m.insert("scr.cmd_failed".into(), "command failed".into());

    m.insert(
        "scr.completed".into(),
        "Script '{}' completed successfully ({} commands)".into(),
    );

    m.insert(
        "scr.continue_warn".into(),
        "Continuing after error at line {}".into(),
    );

    m.insert(
        "scr.desc_args".into(),
        " $1..$9, $@ Positional script arguments".into(),
    );

    m.insert(
        "scr.desc_force".into(),
        " @ <cmd> Force a system command (bypass ntc builtins)".into(),
    );

    m.insert(
        "scr.desc_interp".into(),
        " $var / ${var} / ${var:-default} Variable interpolation".into(),
    );

    m.insert(
        "scr.desc_set".into(),
        " set NAME = value Define a variable".into(),
    );

    m.insert(
        "scr.done_errors".into(),
        "Script '{}' finished with {} error(s)".into(),
    );

    m.insert("scr.error_at".into(), "Error at {}:{} \u{2014} {}".into());

    m.insert("scr.executing".into(), "[{}] Executing: {}".into());

    m.insert(
        "scr.failed_abort".into(),
        "Script '{}' aborted at line {} ({} error(s))".into(),
    );

    m.insert(
        "scr.help_continue".into(),
        " scr -c <file.ntc> [args...] Continue after errors".into(),
    );

    m.insert("scr.help_help".into(), " scr help Show this help".into());

    m.insert(
        "scr.help_init".into(),
        " scr init [name] Create a script template".into(),
    );

    m.insert(
        "scr.help_ls".into(),
        " scr ls List *.ntc scripts in the current directory".into(),
    );

    m.insert(
        "scr.help_run".into(),
        " scr <file.ntc> [args...] Run an automation script".into(),
    );

    m.insert(
        "scr.help_title".into(),
        "scr \u{2014} ntc automation script runner".into(),
    );

    m.insert("scr.help_usage".into(), "Usage:".into());

    m.insert("scr.init_created".into(), "Created template {}".into());

    m.insert("scr.init_exists".into(), "{} already exists".into());

    m.insert("scr.ls_empty".into(), "No *.ntc scripts found in {}".into());

    m.insert("scr.ls_title".into(), "AUTOMATION SCRIPTS:".into());

    m.insert(
        "scr.no_commands".into(),
        "{} contains no commands to run".into(),
    );

    m.insert("scr.not_found".into(), "Script not found: {} ({})".into());

    m.insert("scr.start".into(), "Running script {}...".into());

    m.insert("scr.syscmd".into(), "Executing: {}".into());

    m.insert(
        "scr.syscmd_exit".into(),
        "Command exited with code: {}".into(),
    );

    m.insert(
        "scr.syscmd_terminated".into(),
        "Command terminated (Ctrl+C)".into(),
    );

    // ── script language ─────────────────────────────────────────────────
    m.insert("scr.help_syntax_header".into(), "Syntax:".into());

    m.insert(
        "scr.help_direct".into(),
        " <file.ntc> [args...] Run a script without the scr prefix".into(),
    );

    m.insert(
        "scr.desc_expr".into(),
        " set NAME = <expr> Values, arithmetic, function calls".into(),
    );

    m.insert(
        "scr.desc_if".into(),
        " if <cond> / else if / else / end Conditional block".into(),
    );

    m.insert(
        "scr.desc_while".into(),
        " while <cond> / end Loop while a condition holds".into(),
    );

    m.insert(
        "scr.desc_for".into(),
        " for VAR in <list> / end Loop over words".into(),
    );

    m.insert(
        "scr.desc_loop".into(),
        " loop [count] / end Repeat count times (default: until break)".into(),
    );

    m.insert(
        "scr.desc_break".into(),
        " break / continue Leave a loop, or skip to the next round".into(),
    );

    m.insert(
        "scr.desc_func".into(),
        " func NAME(params) / end Define a function".into(),
    );

    m.insert(
        "scr.desc_call".into(),
        " call NAME(args) Call a function".into(),
    );

    m.insert(
        "scr.desc_return".into(),
        " return [value] Leave a function with a value".into(),
    );

    m.insert(
        "scr.desc_input".into(),
        " input NAME [prompt] Read a line into a variable".into(),
    );

    m.insert("scr.desc_exit".into(), " exit Stop the script".into());

    m.insert(
        "scr.desc_functions_header".into(),
        "Expression functions:".into(),
    );

    m.insert(
        "scr.desc_fn_nprintf".into(),
        " nprintf(fmt, ...) Format like the nprintf builtin".into(),
    );

    m.insert(
        "scr.desc_fn_strings".into(),
        " len / upper / lower / trim / str String helpers".into(),
    );

    m.insert(
        "scr.desc_fn_math".into(),
        " int / abs Numeric helpers".into(),
    );

    m.insert(
        "scr.desc_fn_env".into(),
        " env(NAME) Read an environment variable".into(),
    );

    m.insert("scr.input_prompt".into(), "> ".into());

    m.insert(
        "scr.input_eof".into(),
        "No input for '{}' \u{2014} stopping the script".into(),
    );

    m.insert("scr.parse_error".into(), "{}:{} \u{2014} {}".into());

    m.insert(
        "scr.parse_failed".into(),
        "Script '{}' has {} syntax error(s); nothing was run".into(),
    );

    m.insert(
        "scr.err_expr".into(),
        "invalid expression '{}': {}".into(),
    );

    m.insert(
        "scr.err_loop_limit".into(),
        "loop exceeded {} iterations and was stopped".into(),
    );

    m.insert("scr.err_func_missing".into(), "unknown function '{}'".into());

    m.insert(
        "scr.err_func_args".into(),
        "'{}' takes {} argument(s), got {}".into(),
    );

    m.insert(
        "scr.err_recursion".into(),
        "'{}' nested more than {} calls deep".into(),
    );

    m.insert(
        "scr.err_break_outside_loop".into(),
        "`break` / `continue` cannot leave function '{}'".into(),
    );

    m.insert(
        "scr.nested_too_deep".into(),
        "Scripts are nested more than {} levels deep; stopping".into(),
    );

    m.insert(
        "scr.pipe_refused".into(),
        "'{}' sets `set-pipe = fail`, so it cannot be piped or redirected".into(),
    );

    m.insert(
        "scr.desc_pipe".into(),
        " set-pipe = on|off May this script be piped or redirected".into(),
    );

    m.insert(
        "scr.desc_showexc".into(),
        " set-showexc = on|off Show the [n] Executing: trace".into(),
    );

}
