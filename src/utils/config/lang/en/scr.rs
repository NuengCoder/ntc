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
}
