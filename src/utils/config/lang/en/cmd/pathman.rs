use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.pathman.access_denied".into(),
        "Access denied — administrator privileges required.".into(),
    );

    m.insert("cmd.pathman.added_to_path".into(), "Added to {}: {}".into());

    m.insert(
        "cmd.pathman.already_in_path".into(),
        "Already in PATH: {}".into(),
    );

    m.insert("cmd.pathman.empty_path".into(), "║  {}".into());

    m.insert(
        "cmd.pathman.example_add".into(),
        "  {} add C:\\Tools".into(),
    );

    m.insert(
        "cmd.pathman.example_add_cwd".into(),
        "  {} add --cwd".into(),
    );

    m.insert(
        "cmd.pathman.example_move".into(),
        "  {} move 3 1               Move entry #3 to the top".into(),
    );

    m.insert(
        "cmd.pathman.example_rm_idx".into(),
        "  {} rm 3                    Remove entry #3 from the list".into(),
    );

    m.insert(
        "cmd.pathman.example_rm_path".into(),
        "  {} rm C:\\OldTools".into(),
    );

    m.insert("cmd.pathman.example_show".into(), "  {} show".into());

    m.insert("cmd.pathman.examples_title".into(), "Examples:".into());

    m.insert(
        "cmd.pathman.help_add".into(),
        "  {} add <path>               Add a directory to PATH".into(),
    );

    m.insert(
        "cmd.pathman.help_add_cwd".into(),
        "  {} add --cwd | --here       Add current directory to PATH".into(),
    );

    m.insert(
        "cmd.pathman.help_move".into(),
        "  {} move <n> <m>             Move entry #n to position #m".into(),
    );

    m.insert(
        "cmd.pathman.help_rm".into(),
        "  {} rm <path-or-number>      Remove from PATH (by path or index)".into(),
    );

    m.insert(
        "cmd.pathman.help_show".into(),
        "  {} show                     Show PATH entries with indices".into(),
    );

    m.insert(
        "cmd.pathman.idx_out_of_range".into(),
        "Index out of range: {} (1-{})".into(),
    );

    m.insert(
        "cmd.pathman.indices_numbers".into(),
        "Indices must be numbers".into(),
    );

    m.insert(
        "cmd.pathman.mode_admin_hint".into(),
        "then run the command again. Or use: mode admin".into(),
    );

    m.insert(
        "cmd.pathman.move_usage".into(),
        "Usage: {} move <n> <m>".into(),
    );

    m.insert(
        "cmd.pathman.moved".into(),
        "Moved to position #{} in {}: {}".into(),
    );

    m.insert(
        "cmd.pathman.new_terminal".into(),
        "Changes will apply to new terminal windows.".into(),
    );

    m.insert(
        "cmd.pathman.not_found_in_path".into(),
        "Not found in PATH: {}".into(),
    );

    m.insert(
        "cmd.pathman.path_update_fail".into(),
        "Failed to update PATH: {}".into(),
    );

    m.insert(
        "cmd.pathman.reg_query_fail".into(),
        "Failed to query registry: {}".into(),
    );

    m.insert(
        "cmd.pathman.reg_write_fail".into(),
        "Failed to write registry: {}".into(),
    );

    m.insert(
        "cmd.pathman.removed_from_path".into(),
        "Removed from {}: {}".into(),
    );

    m.insert(
        "cmd.pathman.rm_usage".into(),
        "Usage: usrpath rm <path-or-number>".into(),
    );

    m.insert(
        "cmd.pathman.run_as_admin".into(),
        "Run your terminal as Administrator (right-click → \\\"Run as administrator\\\"),".into(),
    );

    m.insert(
        "cmd.pathman.same_source_target".into(),
        "Source and target are the same \u{2014} nothing to move".into(),
    );

    m.insert("cmd.pathman.system_level".into(), "System".into());

    m.insert(
        "cmd.pathman.total_entries".into(),
        "  Total: {} entries".into(),
    );

    m.insert("cmd.pathman.usage_title".into(), "Usage:".into());

    m.insert("cmd.pathman.user_level".into(), "User".into());

    m.insert(
        "cmd.pathman.windows_only".into(),
        "usrpath/syspath is a Windows-only feature.".into(),
    );
}
