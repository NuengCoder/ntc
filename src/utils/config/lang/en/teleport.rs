use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "teleport.added".into(),
        "Savepoint '{}' created -> {}".into(),
    );

    m.insert(
        "teleport.all_cleared".into(),
        "All savepoints cleared.".into(),
    );

    m.insert(
        "teleport.back_hint".into(),
        "Use 'tpb' to return to previous location.".into(),
    );

    m.insert("teleport.back_opt".into(), "b. Teleport Back".into());

    m.insert("teleport.cancel_opt".into(), "0. Cancel".into());

    m.insert(
        "teleport.choose_prompt".into(),
        "Enter number to teleport (1-{}) or b/h/0: ".into(),
    );

    m.insert("teleport.clear_cancelled".into(), "Clear cancelled.".into());

    m.insert(
        "teleport.clear_confirm".into(),
        "\u{26A0} Are you sure you want to delete ALL savepoints? (y/N): ".into(),
    );

    m.insert(
        "teleport.clear_hint".into(),
        "Use 'tp clear' to clear history.".into(),
    );

    m.insert(
        "teleport.fs_failed".into(),
        "Failed to access file system: {}".into(),
    );

    m.insert(
        "teleport.history_cleared".into(),
        "Teleport history cleared.".into(),
    );

    m.insert("teleport.history_opt".into(), "h. Show History".into());

    m.insert(
        "teleport.history_title".into(),
        "\u{1F4DC} Teleport History (most recent last)".into(),
    );

    m.insert(
        "teleport.invalid_char".into(),
        "Invalid character in name: '{}'".into(),
    );

    m.insert(
        "teleport.invalid_index".into(),
        "Invalid index: {}. Use 1-{}".into(),
    );

    m.insert(
        "teleport.invalid_input_interactive".into(),
        "Invalid input: {}. Enter a number, 'b', or 'h'.".into(),
    );

    m.insert(
        "teleport.invalid_name".into(),
        "Invalid name: '{}' is reserved or empty".into(),
    );

    m.insert(
        "teleport.jump_failed".into(),
        "Could not teleport: '{}'".into(),
    );

    m.insert(
        "teleport.jump_hint".into(),
        "Use 'tp jump <name>' to teleport.".into(),
    );

    m.insert("teleport.jumped".into(), "Teleported to '{}' -> {}".into());

    m.insert(
        "teleport.list_title".into(),
        "\u{1F4CC} Your Teleport Savepoints (global)".into(),
    );

    m.insert(
        "teleport.menu_title".into(),
        "\u{1F4CC} Teleport - Your Savepoints (global)".into(),
    );

    m.insert(
        "teleport.name_exists".into(),
        "Cannot rename: '{}' already exists as a savepoint".into(),
    );

    m.insert(
        "teleport.name_taken".into(),
        "Savepoint '{}' already exists. Use 'tp edit' to update it.".into(),
    );

    m.insert(
        "teleport.name_taken_edit".into(),
        "Savepoint '{}' already exists".into(),
    );

    m.insert(
        "teleport.no_history_first".into(),
        "No teleport history. Use 'tp jump <name>' first to create history.".into(),
    );

    m.insert(
        "teleport.no_history_start".into(),
        "No teleport history. Use 'tp jump <name>' to start building history.".into(),
    );

    m.insert(
        "teleport.no_savepoints_to_clear".into(),
        "No savepoints to clear.".into(),
    );

    m.insert(
        "teleport.no_savepoints_yet".into(),
        "No savepoints yet. Use 'tp add <name>' to create one.".into(),
    );

    m.insert(
        "teleport.not_found".into(),
        "Savepoint not found: '{}'".into(),
    );

    m.insert(
        "teleport.overwrite_cancelled".into(),
        "Overwrite cancelled.".into(),
    );

    m.insert(
        "teleport.overwrite_prompt".into(),
        "Savepoint '{}' already exists at a different location. Overwrite? (y/N): ".into(),
    );

    m.insert(
        "teleport.prev_gone".into(),
        "Previous location no longer exists: {}".into(),
    );

    m.insert("teleport.removed".into(), "Removed savepoint: '{}'".into());

    m.insert(
        "teleport.removing_from_history".into(),
        "Removing from history. Use 'tp history' to see remaining entries.".into(),
    );

    m.insert(
        "teleport.rename_failed".into(),
        "Failed to rename savepoint: {}".into(),
    );

    m.insert(
        "teleport.rename_same".into(),
        "New name is the same as the old name".into(),
    );

    m.insert(
        "teleport.renamed".into(),
        "Renamed savepoint '{}' \u{2192} '{}'".into(),
    );

    m.insert(
        "teleport.renamed_with_path".into(),
        "Renamed savepoint '{}' -> '{}' (-> {})".into(),
    );

    m.insert(
        "teleport.teleport_cancelled".into(),
        "Teleport cancelled.".into(),
    );

    m.insert(
        "teleport.teleported_back".into(),
        "Teleported back to: {}".into(),
    );

    m.insert(
        "teleport.tpb_hint".into(),
        "Use 'tpb' to go back to the previous location.".into(),
    );

    m.insert(
        "teleport.updated".into(),
        "Savepoint '{}' updated: {} -> {}".into(),
    );
}
