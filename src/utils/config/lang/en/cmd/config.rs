use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.config.deinit.confirm".into(),
        "Delete {}? (y/N): ".into(),
    );

    m.insert(
        "cmd.config.deinit.local_not_found".into(),
        "No ntconfig.toml found, skipping.".into(),
    );

    m.insert(
        "cmd.config.deinit.local_remove_failed".into(),
        "Failed to remove ntconfig.toml: {}".into(),
    );

    m.insert(
        "cmd.config.deinit.local_removed".into(),
        "ntconfig.toml removed.".into(),
    );

    m.insert(
        "cmd.config.deinit.not_found".into(),
        "No {} found, skipping.".into(),
    );

    m.insert(
        "cmd.config.deinit.remove_failed".into(),
        "Failed to remove {}: {}".into(),
    );

    m.insert("cmd.config.deinit.removed".into(), "Removed {}".into());

    m.insert("cmd.config.deinit.skipped".into(), "Skipped.".into());

    m.insert(
        "cmd.config.deinit.usage".into(),
        "Usage: deinit [--all | --ran | --local | --opencode] [--force]".into(),
    );

    m.insert(
        "cmd.config.deinit.opencode_not_found".into(),
        "No .opencode scaffold found, skipping.".into(),
    );

    m.insert(
        "cmd.config.example_depth".into(),
        " config depth 4  # Set max tree depth to 4".into(),
    );

    m.insert(
        "cmd.config.example_history".into(),
        " config history off  # Disable history".into(),
    );

    m.insert(
        "cmd.config.example_linenumbers".into(),
        " config linenumbers ON  # Enable line numbers in file display".into(),
    );

    m.insert(
        "cmd.config.example_output".into(),
        " config output C:\\reports  # Set output path".into(),
    );

    m.insert(
        "cmd.config.example_watch".into(),
        " config watch ON  # Enable file watcher".into(),
    );

    m.insert("cmd.config.examples_header".into(), "Examples:".into());

    m.insert(
        "cmd.config.init.create_failed".into(),
        "Failed to create {}: {}".into(),
    );

    m.insert("cmd.config.init.created".into(), "Created {}".into());

    m.insert(
        "cmd.config.init.exists_warn".into(),
        "{} already exists, skipping.".into(),
    );

    m.insert(
        "cmd.config.init.local_exists".into(),
        "ntconfig.toml already exists, skipping.".into(),
    );

    m.insert(
        "cmd.config.init.usage".into(),
        "Usage: init [--all | --ran | --local | --opencode] [--force]".into(),
    );

    m.insert(
        "cmd.config.init.opencode_ready".into(),
        "OpenCode scaffold ready: {}".into(),
    );

    m.insert("cmd.config.lang.active".into(), "{} (active)".into());

    m.insert(
        "cmd.config.lang.available".into(),
        "Available languages:".into(),
    );

    m.insert(
        "cmd.config.lang.coming_soon".into(),
        "{} (set, coming soon)".into(),
    );

    m.insert("cmd.config.lang.set".into(), "Language set to: {}".into());

    m.insert(
        "cmd.config.lang.set_coming_soon".into(),
        "Language set to: {} (coming soon)".into(),
    );

    m.insert(
        "cmd.config.lang.unsupported".into(),
        "Unsupported language '{}'. Available: {}".into(),
    );

    m.insert(
        "cmd.config.local.create_failed".into(),
        "Failed to create ntconfig.toml: {}".into(),
    );

    m.insert(
        "cmd.config.local.created_template".into(),
        "Created ntconfig.toml template in: {}".into(),
    );

    m.insert(
        "cmd.config.local.exists_warning".into(),
        "\u{26a0}\u{fe0f} ntconfig.toml already exists!".into(),
    );

    m.insert(
        "cmd.config.local.init_cancelled".into(),
        "Initialisation cancelled.".into(),
    );

    m.insert(
        "cmd.config.local.overwrite_prompt".into(),
        "Overwrite? (y/N): ".into(),
    );

    m.insert(
        "cmd.config.local.template_edit_hint".into(),
        " Edit the file and remove '#' from lines you want to enable".into(),
    );

    m.insert(
        "cmd.config.local.template_hint".into(),
        "\u{1f4dd} This is a COMMENTED TEMPLATE - no active settings yet".into(),
    );

    m.insert(
        "cmd.config.local.tip_global".into(),
        "\u{1f4a1} Tip: Global settings (teleports, output path, depth) stay global".into(),
    );

    m.insert(
        "cmd.config.local_deinit.cancelled".into(),
        "Deinitialisation cancelled.".into(),
    );

    m.insert(
        "cmd.config.local_deinit.confirm".into(),
        "Delete ntconfig.toml? (y/N): ".into(),
    );

    m.insert(
        "cmd.config.local_deinit.done".into(),
        "ntconfig.toml removed. Global config is now the only active config.".into(),
    );

    m.insert(
        "cmd.config.local_deinit.global_safe".into(),
        "Global config will NOT be affected.".into(),
    );

    m.insert(
        "cmd.config.local_deinit.lose_settings".into(),
        "Local ignore/care and run alias settings will be lost.".into(),
    );

    m.insert(
        "cmd.config.local_deinit.not_found".into(),
        "No ntconfig.toml found in the current directory.".into(),
    );

    m.insert(
        "cmd.config.local_deinit.remove_failed".into(),
        "Failed to remove ntconfig.toml: {}".into(),
    );

    m.insert(
        "cmd.config.local_deinit.title".into(),
        "\u{26A0}\u{FE0F} LOCAL CONFIG REMOVAL".into(),
    );

    m.insert(
        "cmd.config.local_deinit.will_delete".into(),
        "This will delete: {}".into(),
    );

    m.insert(
        "cmd.config.opencg.created".into(),
        "Created config file at: {}".into(),
    );

    m.insert(
        "cmd.config.opencg.opening".into(),
        "Opening config: {}".into(),
    );

    m.insert(
        "cmd.config.qsc.disabled".into(),
        "QSC hotkey disabled.".into(),
    );

    m.insert(
        "cmd.config.qsc.hotkey_set".into(),
        "QSC hotkey set to: {}".into(),
    );

    m.insert(
        "cmd.config.qsc.hotkey_status".into(),
        "QSC hotkey: {}".into(),
    );

    m.insert(
        "cmd.config.qsc.invalid_format".into(),
        "Invalid QSC format: '{}'. Use '#c #s 0' style.".into(),
    );

    m.insert(
        "cmd.config.qsc.not_configured".into(),
        "not configured".into(),
    );

    m.insert(
        "cmd.config.qsc.usage1".into(),
        "Usage: config qsc \"#c 0\"".into(),
    );

    m.insert(
        "cmd.config.qsc.usage2".into(),
        " config qsc \"#c #s a\"".into(),
    );

    m.insert(
        "cmd.config.qsc.usage3".into(),
        " config qsc \"#c #s #a F1\"".into(),
    );

    m.insert("cmd.config.qsc.usage4".into(), " config qsc off".into());

    m.insert(
        "cmd.config.resetcg.backup_created".into(),
        "Backup created: {}".into(),
    );

    m.insert(
        "cmd.config.resetcg.backup_failed".into(),
        "Failed to create backup: {}".into(),
    );

    m.insert(
        "cmd.config.resetcg.backup_info".into(),
        "Backup saved. Use 'restorecg' to restore if needed.".into(),
    );

    m.insert(
        "cmd.config.resetcg.cancelled".into(),
        "Reset cancelled.".into(),
    );

    m.insert(
        "cmd.config.resetcg.config_file_label".into(),
        "Config file:".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_aliases".into(),
        " \u{2022} Run aliases \u{2192} Cleared".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_custom_ignores".into(),
        " \u{2022} Custom ignores \u{2192} Removed".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_depth".into(),
        " \u{2022} Max depth \u{2192} 2".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_history".into(),
        " \u{2022} History \u{2192} OFF".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_ignored_dirs".into(),
        " \u{2022} Ignored dirs \u{2192} target, build, venv, node_modules, installer, logs, .git"
            .into(),
    );

    m.insert(
        "cmd.config.resetcg.default_linenumbers".into(),
        " \u{2022} Line numbers \u{2192} OFF".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_output".into(),
        " \u{2022} Output path \u{2192} Desktop".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_teleports".into(),
        " \u{2022} Teleports \u{2192} Cleared".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_threads".into(),
        " \u{2022} Threads \u{2192} 4".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_watcher".into(),
        " \u{2022} File watcher \u{2192} OFF".into(),
    );

    m.insert(
        "cmd.config.resetcg.not_found".into(),
        "Config file not found. Nothing to reset.".into(),
    );

    m.insert(
        "cmd.config.resetcg.opt_backup".into(),
        " \u{2022} Type 'backup' - Create backup before reset".into(),
    );

    m.insert(
        "cmd.config.resetcg.opt_no".into(),
        " \u{2022} Type 'no' - Cancel".into(),
    );

    m.insert(
        "cmd.config.resetcg.opt_yes".into(),
        " \u{2022} Type 'yes' - Reset config (no backup)".into(),
    );

    m.insert("cmd.config.resetcg.options_label".into(), "Options:".into());

    m.insert(
        "cmd.config.resetcg.reset_done".into(),
        "Config reset to defaults!".into(),
    );

    m.insert(
        "cmd.config.resetcg.serialize_failed".into(),
        "Failed to serialize default config".into(),
    );

    m.insert(
        "cmd.config.resetcg.warning_desc".into(),
        "This will RESET your configuration to DEFAULT values:".into(),
    );

    m.insert(
        "cmd.config.resetcg.warning_title".into(),
        "\u{26A0}\u{FE0F} CONFIG RESET WARNING".into(),
    );

    m.insert(
        "cmd.config.resetcg.write_failed".into(),
        "Failed to write config: {}".into(),
    );

    m.insert(
        "cmd.config.resetcg.your_choice".into(),
        "Your choice: ".into(),
    );

    m.insert(
        "cmd.config.restorecg.backed_up".into(),
        "Current config backed up to: {}".into(),
    );

    m.insert("cmd.config.restorecg.cancel_opt".into(), "0. Cancel".into());

    m.insert(
        "cmd.config.restorecg.cancelled".into(),
        "Restore cancelled.".into(),
    );

    m.insert(
        "cmd.config.restorecg.failed".into(),
        "Failed to restore: {}".into(),
    );

    m.insert(
        "cmd.config.restorecg.invalid".into(),
        "Invalid selection.".into(),
    );

    m.insert(
        "cmd.config.restorecg.none".into(),
        "No backup files found.".into(),
    );

    m.insert(
        "cmd.config.restorecg.reloaded".into(),
        "Config reloaded! Run 'showcg' to see the restored settings.".into(),
    );

    m.insert(
        "cmd.config.restorecg.restored".into(),
        "Restored config from: {}".into(),
    );

    m.insert(
        "cmd.config.restorecg.title".into(),
        "\u{1F4CB} Available Backups:".into(),
    );

    m.insert("cmd.config.mgmt_header".into(), "Management:".into());

    m.insert("cmd.config.settings_header".into(), "Settings:".into());

    m.insert(
        "cmd.config.showcg.lang_coming_soon".into(),
        "{} (coming soon)".into(),
    );

    m.insert("cmd.config.showcg.lang_label".into(), "Language:".into());

    m.insert("cmd.config.showcg.qsc_label".into(), "QSC Hotkey:".into());

    m.insert(
        "cmd.config.title".into(),
        "config \u{2014} View or change configuration settings".into(),
    );

    m.insert(
        "cmd.config.watch.alias_not_found".into(),
        "Alias '{}' not found. Use 'ral list' to see aliases.".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_cleared".into(),
        "Watch trigger alias cleared.".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_label".into(),
        "Trigger alias:".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_off_usage".into(),
        "config watch trigger off Disable auto-run".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_set".into(),
        "Watch trigger set to alias: '{}'".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_usage".into(),
        "config watch trigger <alias> Auto-run alias on change".into(),
    );

    m.insert(
        "cmd.config.watch.limit_usage".into(),
        "config watch limit <n> Rows shown in change box".into(),
    );

    m.insert("cmd.config.watch.usage".into(), "Use: config watch ON | config watch OFF | config watch trigger <alias> | config watch trigger off | config watch limit <n>".into());

    m.insert(
        "cmd.config.watch.usage_line".into(),
        "Usage: config watch ON|OFF".into(),
    );

    m.insert(
        "cmd.config.serve.title".into(),
        "Serve Config (per-service defaults)".into(),
    );

    m.insert("cmd.config.serve.status".into(), "  {}: {}:{}".into());

    m.insert("cmd.config.serve.usage".into(), "Usage: serve config [files|http-server|chess] [--port-default <port>] [--bind-default <addr>]  (alias: config serve …)".into());

    m.insert(
        "cmd.config.serve.example_show".into(),
        "  serve config                  Show all services".into(),
    );

    m.insert(
        "cmd.config.serve.example_show_one".into(),
        "  serve config files            Show files defaults".into(),
    );

    m.insert(
        "cmd.config.serve.example_set".into(),
        "  serve config files --port-default 9000   Set files default port".into(),
    );

    m.insert(
        "cmd.config.serve.saved".into(),
        "Serve {} defaults saved: {}:{}".into(),
    );

    m.insert(
        "cmd.config.serve.invalid_service".into(),
        "Unknown service '{}' (use files|http-server|chess)".into(),
    );

    m.insert(
        "cmd.config.serve.invalid_port".into(),
        "Invalid port '{}' (use 1-65535)".into(),
    );

    m.insert(
        "cmd.config.serve.empty_bind".into(),
        "Missing value for --bind-default".into(),
    );

    m.insert(
        "cmd.config.serve.unknown_option".into(),
        "Unknown: {}".into(),
    );

    m.insert("cmd.config.local.init_all_removed".into(), "'local init --all' was removed - a local config can no longer copy the global settings. Use 'local init' for the template.".into());

    m.insert("cmd.config.local.precedence_hint".into(), "Local entries take priority; global entries keep working and the global config is never changed.".into());
}
