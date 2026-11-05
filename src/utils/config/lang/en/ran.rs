use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "ran.already_exists".into(),
        "{} already exists in {}".into(),
    );

    m.insert(
        "ran.calling_not_allowed".into(),
        "Calling 'ran' from within a NTCRANFILE target is not allowed".into(),
    );

    m.insert(
        "ran.completed".into(),
        "Command completed successfully.".into(),
    );

    m.insert("ran.created".into(), "Created {}".into());

    m.insert(
        "ran.desc_aliases".into(),
        " Commands are expanded through ntc's run-alias system, so aliases work too.".into(),
    );

    m.insert(
        "ran.desc_aliases2".into(),
        " Prefix with 'run ' to bypass alias expansion and run directly as a system command."
            .into(),
    );

    m.insert(
        "ran.desc_deps".into(),
        " Targets can depend on other targets. Dependencies run first, in order.".into(),
    );

    m.insert(
        "ran.desc_note".into(),
        " Calling 'ran' from within a NTCRANFILE target command is not allowed.".into(),
    );

    m.insert(
        "ran.desc_reserved".into(),
        " init, deinit, help, list, ls, check, fmt \u{2014} these cannot be used as target names."
            .into(),
    );

    m.insert(
        "ran.desc_vars".into(),
        " Define variables under [vars] and reference them with $(VAR_NAME) in commands.".into(),
    );

    m.insert(
        "ran.desc_vars2".into(),
        " They work like Make variables but use $(...) syntax.".into(),
    );

    m.insert("ran.error_prefix".into(), "ran: {}".into());

    m.insert(
        "ran.exec_failed".into(),
        "Failed to execute command: {}".into(),
    );

    m.insert("ran.executing".into(), "Executing: {}".into());

    m.insert(
        "ran.exit_code".into(),
        "Command exited with code: {}".into(),
    );

    m.insert(
        "ran.get_cwd_failed".into(),
        "Failed to get current directory: {}".into(),
    );

    m.insert(
        "ran.help_deinit_fmt".into(),
        " ran deinit Remove {} from the current directory".into(),
    );

    m.insert("ran.help_help".into(), " ran help Show this help".into());

    m.insert(
        "ran.help_init_fmt".into(),
        " ran init Create a {} in the current directory".into(),
    );

    m.insert(
        "ran.help_list".into(),
        " ran list List all available targets".into(),
    );

    m.insert(
        "ran.help_standalone".into(),
        " ran Run the 'standalone' target (default)".into(),
    );

    m.insert(
        "ran.help_target".into(),
        " ran <target> Run a specific target".into(),
    );

    m.insert(
        "ran.help_title".into(),
        "ran \u{2014} ntc task runner (like Make but ntc-native)".into(),
    );

    m.insert("ran.help_usage".into(), "Usage:".into());

    m.insert("ran.not_found".into(), "No {} found in {}".into());

    m.insert("ran.remove_failed".into(), "Failed to remove {}: {}".into());

    m.insert("ran.removed".into(), "Removed {}".into());

    m.insert("ran.section_aliases".into(), "Run aliases:".into());

    m.insert("ran.section_deps".into(), "Dependencies:".into());

    m.insert("ran.section_note".into(), "Note:".into());

    m.insert(
        "ran.section_reserved".into(),
        "Reserved target names:".into(),
    );

    m.insert("ran.section_vars".into(), "Variables:".into());

    m.insert(
        "ran.terminated".into(),
        "Command terminated (Ctrl+C)".into(),
    );

    m.insert("ran.check_title".into(), "ran check interpreting {}".into());

    m.insert("ran.check_ok".into(), "{} no problem".into());

    m.insert(
        "ran.check_problems".into(),
        "{} problem(s) found in {}".into(),
    );

    m.insert("ran.check_error".into(), "error".into());

    m.insert("ran.check_hint".into(), "hint".into());

    m.insert(
        "ran.help_check_fmt".into(),
        " ran check Interpret {} and report every problem with its line".into(),
    );

    m.insert(
        "ran.help_fmt".into(),
        " ran fmt <toml|json|yaml> Convert the ranfile (-o <file>, -y to skip the prompt)".into(),
    );

    m.insert(
        "ran.fmt.usage".into(),
        "Usage: ran fmt <toml|json|yaml> [-o <file>] [-y]".into(),
    );

    m.insert(
        "ran.fmt.unknown".into(),
        "Unknown format '{}'. Use toml, json or yaml.".into(),
    );

    m.insert(
        "ran.fmt.current".into(),
        "Ranfile format preference: {} (ran init creates {})".into(),
    );

    m.insert(
        "ran.fmt.no_file".into(),
        "No ranfile here. Preference is now {}; run `ran init` to create {}.".into(),
    );

    m.insert("ran.fmt.already".into(), "{} is already {}.".into());

    m.insert(
        "ran.fmt.confirm".into(),
        "Convert {} to {} \u{2192} {}? [y|n] ".into(),
    );

    m.insert(
        "ran.fmt.skipped".into(),
        "Nothing written. `ran init` will create {}.".into(),
    );

    m.insert(
        "ran.fmt.exists".into(),
        "{} already exists \u{2014} remove it or pass -o <file>.".into(),
    );

    m.insert("ran.fmt.written".into(), "Wrote {}".into());

    m.insert(
        "ran.fmt.old_kept".into(),
        "{} is still there; `ran` now prefers {} (delete the old file to avoid two ranfiles)."
            .into(),
    );

    m.insert(
        "ran.fmt.no_terminal".into(),
        "Not a terminal: pass -y to confirm the conversion.".into(),
    );
}
