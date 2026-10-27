use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "autoup.added".into(),
        "Added startup entry: {} \u{2192} {}".into(),
    );

    m.insert(
        "autoup.already_disabled".into(),
        "Entry '{}' is already disabled.".into(),
    );

    m.insert(
        "autoup.already_enabled".into(),
        "Entry '{}' is already enabled.".into(),
    );

    m.insert(
        "autoup.already_exists".into(),
        "Entry '{}' already exists in user startup.".into(),
    );

    m.insert(
        "autoup.cmd_empty".into(),
        "Command value cannot be empty.".into(),
    );

    m.insert(
        "autoup.disabled".into(),
        "Disabled startup entry: {}".into(),
    );

    m.insert(
        "autoup.disabled_system".into(),
        "Disabled system startup entry: {}".into(),
    );

    m.insert("autoup.enabled".into(), "Enabled startup entry: {}".into());

    m.insert(
        "autoup.enabled_system".into(),
        "Enabled system startup entry: {}".into(),
    );

    m.insert(
        "autoup.help_add".into(),
        " autoup add <name> \"<command>\" Add a startup entry (HKCU)".into(),
    );

    m.insert(
        "autoup.help_disable".into(),
        " autoup disable <name> Disable a startup entry".into(),
    );

    m.insert(
        "autoup.help_enable".into(),
        " autoup enable <name> Re-enable a disabled entry".into(),
    );

    m.insert(
        "autoup.help_help".into(),
        " autoup help Show this help".into(),
    );

    m.insert(
        "autoup.help_list".into(),
        " autoup list List all startup entries from all sources".into(),
    );

    m.insert(
        "autoup.help_rm".into(),
        " autoup rm <name> Remove a startup entry".into(),
    );

    m.insert(
        "autoup.hklm_hint".into(),
        "'{}' is a system entry (HKLM). Run 'mode admin' first, then retry.".into(),
    );

    m.insert(
        "autoup.hklm_hint_short".into(),
        "'{}' is a system entry. Run 'mode admin' first, then retry.".into(),
    );

    m.insert(
        "autoup.list_empty".into(),
        "(No startup entries found)".into(),
    );

    m.insert("autoup.list_title".into(), "Windows Startup Entries".into());

    m.insert(
        "autoup.list_total".into(),
        "Total: {} [A]:{} [D]:{} (CU:{} LM:{} SF:{})".into(),
    );

    m.insert(
        "autoup.not_found".into(),
        "Startup entry not found: {}".into(),
    );

    m.insert(
        "autoup.reg_delete_fail".into(),
        "Failed to delete registry entry.".into(),
    );

    m.insert(
        "autoup.reg_disable_fail".into(),
        "Failed to set disabled state.".into(),
    );

    m.insert(
        "autoup.reg_exec_fail".into(),
        "Failed to run reg.exe: {}".into(),
    );

    m.insert(
        "autoup.reg_query_fail".into(),
        "Failed to query registry: {}".into(),
    );

    m.insert(
        "autoup.reg_write_fail".into(),
        "Failed to write registry entry. Try running as Administrator.".into(),
    );

    m.insert("autoup.removed".into(), "Removed startup entry: {}".into());

    m.insert(
        "autoup.removed_system".into(),
        "Removed system startup entry: {}".into(),
    );

    m.insert(
        "autoup.sf_disable_hint".into(),
        "'{}' is in the Startup folder. Move the shortcut out to disable it.".into(),
    );

    m.insert(
        "autoup.sf_rm_hint".into(),
        "'{}' is in the Startup folder. Delete the shortcut file manually.".into(),
    );

    m.insert(
        "autoup.source_cu".into(),
        " [CU] Current User (HKCU\\...\\Run)".into(),
    );

    m.insert(
        "autoup.source_lm".into(),
        " [LM] Local Machine (HKLM\\...\\Run) \u{2014} requires admin (mode admin)".into(),
    );

    m.insert(
        "autoup.source_sf".into(),
        " [SF] Startup Folder (%APPDATA%\\...\\Startup) \u{2014} read-only".into(),
    );

    m.insert("autoup.source_tags".into(), "Source tags:".into());

    m.insert(
        "autoup.usage_add".into(),
        "Usage: autoup add <name> \"<command>\"".into(),
    );

    m.insert(
        "autoup.usage_disable".into(),
        "Usage: autoup disable <name>".into(),
    );

    m.insert(
        "autoup.usage_enable".into(),
        "Usage: autoup enable <name>".into(),
    );

    m.insert("autoup.usage_examples".into(), "Examples:".into());

    m.insert("autoup.usage_rm".into(), "Usage: autoup rm <name>".into());

    m.insert("autoup.usage_title".into(), "Usage:".into());

    m.insert(
        "autoup.use_rm_first".into(),
        "Use 'autoup rm' first if you want to replace it.".into(),
    );

    m.insert(
        "autoup.will_run".into(),
        "The entry will run on your next login.".into(),
    );

    m.insert(
        "autoup.windows_only".into(),
        "autoup is a Windows-only feature.".into(),
    );
}
