use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.backup.clear".into(),
        "backup clear ## Delete ALL backups (asks confirmation)".into(),
    );

    m.insert(
        "help.backup.clear_force".into(),
        "backup clear --force ## Delete ALL backups (no confirmation)".into(),
    );

    m.insert(
        "help.backup.create".into(),
        "backup create ## Create a backup of current project".into(),
    );

    m.insert(
        "help.backup.diff".into(),
        "backup diff [num] ## Diff current project against a backup".into(),
    );

    m.insert(
        "help.backup.list".into(),
        "backup list ## List all backups".into(),
    );

    m.insert(
        "help.backup.recreate".into(),
        "backup recreate ## Clear all + undo + create fresh backup".into(),
    );

    m.insert(
        "help.backup.restore".into(),
        "backup restore [num] ## Restore a backup (interactive if no num)".into(),
    );

    m.insert(
        "help.backup.restore_choose".into(),
        "backup restore --choose ## Pick a backup from an interactive menu".into(),
    );

    m.insert(
        "help.backup.restore_deleted".into(),
        "backup restore --deleted ## Restore only files that were deleted".into(),
    );

    m.insert("help.backup.title".into(), "BACKUP COMMANDS:".into());

    m.insert(
        "help.backup.undo".into(),
        "backup undo ## Undo the last restore".into(),
    );

    m.insert(
        "help.backup.undo_cls".into(),
        "backup undo --cls ## Clear undo history (asks confirmation)".into(),
    );

    m.insert(
        "help.backup.undo_force".into(),
        "backup undo --force ## Clear undo history (no confirmation)".into(),
    );

    m.insert(
        "help.backup.example1".into(),
        "backup create ## Create a new backup".into(),
    );

    m.insert(
        "help.backup.example2".into(),
        "backup list ## List all backups".into(),
    );

    m.insert(
        "help.backup.example3".into(),
        "backup restore 1 ## Restore backup #1".into(),
    );

    m.insert(
        "help.backup.example4".into(),
        "backup diff 1 ## Diff against backup #1".into(),
    );
}
