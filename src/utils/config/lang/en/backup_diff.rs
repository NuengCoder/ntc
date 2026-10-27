use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("backup_diff.cancel_opt".into(), "0. Cancel".into());

    m.insert("backup_diff.cancelled".into(), "Diff cancelled.".into());

    m.insert(
        "backup_diff.comparing".into(),
        "Comparing current project with backup #{}...".into(),
    );

    m.insert(
        "backup_diff.invalid_input".into(),
        "Invalid input: {}".into(),
    );

    m.insert(
        "backup_diff.invalid_selection".into(),
        "Invalid selection: {}".into(),
    );

    m.insert(
        "backup_diff.none".into(),
        "No backups found for this project. Use 'bkup' to create one.".into(),
    );

    m.insert(
        "backup_diff.not_found".into(),
        "Backup #{} not found.".into(),
    );

    m.insert(
        "backup_diff.open_manually".into(),
        "Open the file manually: {}".into(),
    );

    m.insert(
        "backup_diff.saved_to".into(),
        "Diff report saved to: {}".into(),
    );

    m.insert(
        "backup_diff.select_prompt".into(),
        "Select backup (1-{}): ".into(),
    );

    m.insert(
        "backup_diff.select_title".into(),
        "📊 Select a backup to diff against current project".into(),
    );
}
