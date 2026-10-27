use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.nssh.usage_title".into(), "Usage:".into());

    m.insert(
        "cmd.nssh.help_add".into(),
        "nssh add <name> --host <h> [--user u] [--port p] [--key path] [--pass-env VAR]".into(),
    );

    m.insert(
        "cmd.nssh.help_ssh".into(),
        "nssh ssh [name] [--cmd \"...\"] [--trust] [--ask-pass] [--timeout s]".into(),
    );

    m.insert(
        "cmd.nssh.help_cp".into(),
        "nssh cp <local> <name>:<remote> | <name>:<remote> <local>".into(),
    );

    m.insert(
        "cmd.nssh.help_scan".into(),
        "nssh scan <host> [--ports 22,80,8000-8010 | --top N] [--timeout ms]".into(),
    );

    m.insert(
        "cmd.nssh.help_manage".into(),
        "nssh ls | nssh info <name> | nssh rm <name> | nssh edit <name> [flags]".into(),
    );

    m.insert(
        "cmd.nssh.help_keygen".into(),
        "nssh keygen [args...]   Generate keys via ssh-keygen".into(),
    );

    m.insert(
        "cmd.nssh.help_help".into(),
        "nssh help   Show this help (bare nssh picks a profile)".into(),
    );

    m.insert("cmd.nssh.options_title".into(), "Options:".into());

    m.insert(
        "cmd.nssh.help_host".into(),
        "  --host <h>            Server hostname or IP (required for add)".into(),
    );

    m.insert(
        "cmd.nssh.help_user".into(),
        "  --user, -u <u>        Login user (default: current user)".into(),
    );

    m.insert(
        "cmd.nssh.help_port".into(),
        "  --port, -p <p>        SSH port (default: 22)".into(),
    );

    m.insert(
        "cmd.nssh.help_key".into(),
        "  --key <path>          Private key file (default: ~/.ssh/id_ed25519, id_rsa)".into(),
    );

    m.insert(
        "cmd.nssh.help_pass_env".into(),
        "  --pass-env <VAR>      deadlock pass-env name (never stored); falls back to env var VAR"
            .into(),
    );

    m.insert(
        "cmd.nssh.help_cmd".into(),
        "  --cmd \"...\"           Run one command instead of an interactive shell".into(),
    );

    m.insert(
        "cmd.nssh.help_trust".into(),
        "  --trust               Accept + record unknown host keys (default: fail closed)".into(),
    );

    m.insert(
        "cmd.nssh.help_ask_pass".into(),
        "  --ask-pass            Prompt for the password (hidden input)".into(),
    );

    m.insert(
        "cmd.nssh.help_timeout".into(),
        "  --timeout <n>         Connect timeout in seconds (ssh/cp) or ms (scan)".into(),
    );

    m.insert(
        "cmd.nssh.help_top".into(),
        "  --top <n>             Scan the top-N common ports (default: 20)".into(),
    );

    m.insert(
        "cmd.nssh.help_scan_ports".into(),
        "  --ports <spec>        Ports like 22,80,8000-8010 (max 2000 per range)".into(),
    );

    m.insert(
        "cmd.nssh.help_yes".into(),
        "  --yes, -y             Skip the delete confirmation".into(),
    );

    m.insert("cmd.nssh.examples_title".into(), "Examples:".into());

    m.insert(
        "cmd.nssh.example_add".into(),
        "nssh add prod --host 203.0.113.10 --user root --key ~/.ssh/id_rsa".into(),
    );

    m.insert(
        "cmd.nssh.example_ssh".into(),
        "nssh ssh prod --cmd \"uptime\"".into(),
    );

    m.insert(
        "cmd.nssh.example_cp".into(),
        "nssh cp ./app.tar.gz prod:/tmp/".into(),
    );

    m.insert(
        "cmd.nssh.example_scan".into(),
        "nssh scan example.com --top 20".into(),
    );

    m.insert(
        "cmd.nssh.missing_host".into(),
        "Missing value for --host".into(),
    );

    m.insert(
        "cmd.nssh.missing_user".into(),
        "Missing value for --user".into(),
    );

    m.insert(
        "cmd.nssh.missing_port".into(),
        "Missing value for --port".into(),
    );

    m.insert(
        "cmd.nssh.missing_key".into(),
        "Missing value for --key".into(),
    );

    m.insert(
        "cmd.nssh.missing_pass_env".into(),
        "Missing value for --pass-env".into(),
    );

    m.insert(
        "cmd.nssh.missing_cmd".into(),
        "Missing value for --cmd".into(),
    );

    m.insert(
        "cmd.nssh.missing_timeout".into(),
        "Missing value for --timeout".into(),
    );

    m.insert(
        "cmd.nssh.missing_ports".into(),
        "Missing value for --ports".into(),
    );

    m.insert(
        "cmd.nssh.missing_top".into(),
        "Missing value for --top".into(),
    );

    m.insert("cmd.nssh.invalid_port".into(), "Invalid port: '{}'".into());

    m.insert(
        "cmd.nssh.invalid_timeout".into(),
        "Invalid timeout: '{}'".into(),
    );

    m.insert(
        "cmd.nssh.invalid_top".into(),
        "Invalid --top value: '{}'".into(),
    );

    m.insert(
        "cmd.nssh.invalid_name".into(),
        "Invalid profile name '{}' (letters, digits, _ - . only)".into(),
    );

    m.insert("cmd.nssh.unknown_option".into(), "Unknown: {}".into());

    m.insert(
        "cmd.nssh.add_usage".into(),
        "Usage: nssh add <name> --host <h> [--user u] [--port p] [--key path] [--pass-env VAR]"
            .into(),
    );

    m.insert(
        "cmd.nssh.profile_exists".into(),
        "Profile '{}' exists (use nssh edit to change it)".into(),
    );

    m.insert("cmd.nssh.profile_saved".into(), "Profile saved: {}".into());

    m.insert(
        "cmd.nssh.profile_not_found".into(),
        "Profile '{}' not found".into(),
    );

    m.insert(
        "cmd.nssh.key_not_found".into(),
        "Key file '{}' does not exist (stored anyway)".into(),
    );

    m.insert(
        "cmd.nssh.pass_env_missing".into(),
        "pass-env '{}' is neither in the deadlock vault nor set in the environment (stored anyway)"
            .into(),
    );

    m.insert(
        "cmd.nssh.pass_env_registered".into(),
        "pass-env '{}' is registered in the deadlock vault.".into(),
    );

    m.insert(
        "cmd.nssh.pass_env_pending".into(),
        "pass-env '{}' will be resolved from the deadlock vault at connect time.".into(),
    );

    m.insert(
        "cmd.nssh.pass_env_from_vault".into(),
        "Using pass-env '{}' from the deadlock vault.".into(),
    );

    m.insert(
        "cmd.nssh.master_unlock_failed".into(),
        "Could not unlock the deadlock vault ({}); falling back to the SSH password prompt.".into(),
    );

    m.insert(
        "cmd.nssh.ls_empty".into(),
        "No profiles yet. Add one: nssh add <name> --host <h>".into(),
    );

    m.insert("cmd.nssh.ls_title".into(), "SSH profiles:".into());

    m.insert("cmd.nssh.info_title".into(), "Profile '{}':".into());

    m.insert(
        "cmd.nssh.info_usage".into(),
        "Usage: nssh info <name>".into(),
    );

    m.insert("cmd.nssh.info_host".into(), "host:".into());

    m.insert("cmd.nssh.info_user".into(), "user:".into());

    m.insert("cmd.nssh.info_port".into(), "port:".into());

    m.insert("cmd.nssh.info_key".into(), "key:".into());

    m.insert("cmd.nssh.info_pass".into(), "pass_env:".into());

    m.insert(
        "cmd.nssh.rm_usage".into(),
        "Usage: nssh rm <name> [--yes]".into(),
    );

    m.insert("cmd.nssh.rm_confirm".into(), "Delete profile '{}'?".into());

    m.insert("cmd.nssh.rm_deleted".into(), "Deleted profile: {}".into());

    m.insert("cmd.nssh.rm_cancelled".into(), "Cancelled.".into());

    m.insert("cmd.nssh.edit_usage".into(), "Usage: nssh edit <name> [--host h] [--user u] [--port p] [--key path|--no-key] [--pass-env VAR|--no-pass-env]".into());

    m.insert("cmd.nssh.edit_saved".into(), "Profile updated: {}".into());

    m.insert("cmd.nssh.pick_profile".into(), "Pick an SSH profile".into());

    m.insert("cmd.nssh.ask_pass".into(), "Password for {}@{}: ".into());

    m.insert(
        "cmd.nssh.pass_env_unset".into(),
        "Env var '{}' is empty or unset — will prompt if --ask-pass".into(),
    );

    m.insert(
        "cmd.nssh.ssh_connecting".into(),
        "Connecting to {}@{}:{}...".into(),
    );

    m.insert(
        "cmd.nssh.cmd_exit".into(),
        "Remote command exited with code {}".into(),
    );

    m.insert(
        "cmd.nssh.cp_usage".into(),
        "Usage: nssh cp <local> <name>:<remote> | nssh cp <name>:<remote> <local>".into(),
    );

    m.insert(
        "cmd.nssh.cp_no_remote".into(),
        "Exactly one side must be <profile>:<path>".into(),
    );

    m.insert(
        "cmd.nssh.cp_two_sides".into(),
        "Both sides are remote — copy one side at a time".into(),
    );

    m.insert(
        "cmd.nssh.cp_local_missing".into(),
        "Local file '{}' not found".into(),
    );

    m.insert(
        "cmd.nssh.cp_read_fail".into(),
        "Cannot read local file: {}".into(),
    );

    m.insert(
        "cmd.nssh.cp_too_large".into(),
        "File exceeds the 256MB v1 transfer guard".into(),
    );

    m.insert(
        "cmd.nssh.cp_uploading".into(),
        "Uploading {} to {}:{}...".into(),
    );

    m.insert(
        "cmd.nssh.cp_downloading".into(),
        "Downloading {}:{} to {}...".into(),
    );

    m.insert("cmd.nssh.cp_done".into(), "Done ({} bytes)".into());

    m.insert(
        "cmd.nssh.scan_usage".into(),
        "Usage: nssh scan <host> [--ports <spec> | --top <n>] [--timeout <ms>]".into(),
    );

    m.insert(
        "cmd.nssh.scan_start".into(),
        "Scanning {} ({} ports)...".into(),
    );

    m.insert("cmd.nssh.scan_title".into(), "Open ports on {}:".into());

    m.insert("cmd.nssh.scan_none".into(), "No open ports on {}".into());

    m.insert("cmd.nssh.scan_done".into(), "{} open of {} probed".into());

    m.insert(
        "cmd.nssh.keygen_missing".into(),
        "ssh-keygen not found: {}".into(),
    );

    m.insert(
        "cmd.nssh.keygen_exit".into(),
        "ssh-keygen exited with code {}".into(),
    );

    m.insert(
        "cmd.nssh.shell_hint".into(),
        "Interactive shell — Enter ~ . force-disconnects".into(),
    );

    m.insert("cmd.nssh.shell_back".into(), "Back in ntc shell".into());

    m.insert(
        "cmd.nssh.shell_forced".into(),
        "Force-disconnected (~.)".into(),
    );

    m.insert(
        "cmd.nssh.password_prompt".into(),
        "Password for {}@{}: ".into(),
    );

    m.insert(
        "cmd.nssh.password_incorrect".into(),
        "Incorrect. Password for {}@{}: ".into(),
    );

    m.insert(
        "cmd.nssh.auth_failed".into(),
        "SSH authentication failed for {}@{}.".into(),
    );

    m.insert(
        "cmd.nssh.pass_env_not_in_vault".into(),
        "pass-env '{}' is not in the deadlock vault - trying the environment".into(),
    );
}
