use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("backup.and_more".into(), " ... and {} more".into());

    m.insert(
        "backup.available_title".into(),
        " Available Backups (newest first)".into(),
    );

    m.insert("backup.cancel_opt".into(), " 0. Cancel".into());

    m.insert("backup.cancelled".into(), "Clear cancelled.".into());

    m.insert(
        "backup.clear_history_prompt".into(),
        "\u{26A0} Clear undo history? This cannot be undone. (y/N):".into(),
    );

    m.insert("backup.cleared".into(), "All backups deleted.".into());

    m.insert(
        "backup.cleared_count".into(),
        "Cleared {} backup(s) for this project.".into(),
    );

    m.insert("backup.confirm".into(), "Type 'yes' to confirm: ".into());

    m.insert(
        "backup.create_hint".into(),
        "Use 'backup create' to create your first backup.".into(),
    );

    m.insert(
        "backup.created".into(),
        "Backup created: #{} \u{2014} {} \u{2014} {} files".into(),
    );

    m.insert(
        "backup.created_detail".into(),
        "Backup #{} created: {} ({} files, {})".into(),
    );

    m.insert("backup.creating".into(), "Creating backup...".into());

    m.insert(
        "backup.delete_extra_failed".into(),
        "Failed to delete extra file {}: {}".into(),
    );

    m.insert(
        "backup.deleted_since".into(),
        "{} has been deleted since the backup.".into(),
    );

    m.insert(
        "backup.entry_file_count".into(),
        " Backup #{} \u{2014} {} \u{2014} {} \u{2014} {} files".into(),
    );

    m.insert(
        "backup.exists".into(),
        "Backups exist: {} backup(s) at {}".into(),
    );

    m.insert(
        "backup.failed_to_backup".into(),
        "Failed to backup {}: {}".into(),
    );

    m.insert(
        "backup.file_deleted".into(),
        "{} was deleted after the backup.".into(),
    );

    m.insert(
        "backup.file_modified".into(),
        "{} has changed since the last save.".into(),
    );

    m.insert(
        "backup.help_clear".into(),
        " backup clear Delete all backups (with confirmation)".into(),
    );

    m.insert(
        "backup.help_clear_force".into(),
        " backup clear --force Delete all backups (no confirmation)".into(),
    );

    m.insert(
        "backup.help_create".into(),
        " backup create Create a new backup".into(),
    );

    m.insert(
        "backup.help_diff".into(),
        " backup diff [num] Diff current project against a backup".into(),
    );

    m.insert("backup.help_examples".into(), "Examples:".into());

    m.insert(
        "backup.help_list".into(),
        " backup list List all backups".into(),
    );

    m.insert(
        "backup.help_recreate".into(),
        " backup recreate Clear all + undo history + create fresh".into(),
    );

    m.insert(
        "backup.help_recreate_force".into(),
        " backup recreate --force Same, skip all confirmations".into(),
    );

    m.insert(
        "backup.help_restore".into(),
        " backup restore [num] Restore a backup (interactive if no num)".into(),
    );

    m.insert(
        "backup.help_restore_choose".into(),
        " backup restore --choose Pick a backup from an interactive menu".into(),
    );

    m.insert(
        "backup.help_restore_choose_deleted".into(),
        " backup restore --choose --deleted Pick + restore-deleted".into(),
    );

    m.insert(
        "backup.help_restore_deleted".into(),
        " backup restore --deleted Restore only files deleted since the backup".into(),
    );

    m.insert(
        "backup.help_title".into(),
        "backup \u{2014} Manage project backups".into(),
    );

    m.insert(
        "backup.help_undo".into(),
        " backup undo Undo last restore".into(),
    );

    m.insert(
        "backup.help_undo_cls".into(),
        " backup undo --cls Clear undo history".into(),
    );

    m.insert(
        "backup.help_undo_force".into(),
        " backup undo --force Clear undo history (no confirm)".into(),
    );

    m.insert("backup.invalid_arg".into(), "Invalid argument: {}".into());

    m.insert(
        "backup.invalid_diff_arg".into(),
        "Invalid argument: {}. Usage: backup diff <number>".into(),
    );

    m.insert(
        "backup.invalid_selection".into(),
        "Invalid selection: {}".into(),
    );

    m.insert(
        "backup.list_entry".into(),
        " #{} \u{2014} {} \u{2014} {} \u{2014} {} files".into(),
    );

    m.insert("backup.list_location".into(), "Location: {}".into());

    m.insert(
        "backup.list_title".into(),
        "Backups for this project: {} found".into(),
    );

    m.insert("backup.menu_item".into(), "Backup #{}".into());

    m.insert(
        "backup.modified_since".into(),
        "{} has been modified since the backup.".into(),
    );

    m.insert(
        "backup.no_deleted_files".into(),
        "No deleted files to restore \u{2014} project already matches the backup snapshot.".into(),
    );

    m.insert(
        "backup.none".into(),
        "No backups found for this project.".into(),
    );

    m.insert(
        "backup.not_found_num".into(),
        "Backup #{} not found.".into(),
    );

    m.insert(
        "backup.pick_detail".into(),
        "{} \u{2014} {} \u{2014} {} files".into(),
    );

    m.insert(
        "backup.pick_prompt".into(),
        "Enter number (1-{}) or 0: ".into(),
    );

    m.insert(
        "backup.pick_title".into(),
        "Select a backup to restore from".into(),
    );

    m.insert(
        "backup.previous_state_saved".into(),
        "Previous state saved. Use 'unpd' to undo this restore.".into(),
    );

    m.insert("backup.recreate_proceed".into(), "Proceed? (y/N): ".into());

    m.insert(
        "backup.recreate_prompt".into(),
        "No existing backups found. Creating fresh backup anyway? (y/N): ".into(),
    );

    m.insert(
        "backup.recreate_step1".into(),
        " 1. Delete ALL existing backups".into(),
    );

    m.insert(
        "backup.recreate_step2".into(),
        " 2. Clear undo history".into(),
    );

    m.insert(
        "backup.recreate_step3".into(),
        " 3. Create a fresh backup of the current state".into(),
    );

    m.insert(
        "backup.recreate_will_title".into(),
        "\u{26A0}\u{FE0F} RECREATE will:".into(),
    );

    m.insert(
        "backup.recreated".into(),
        "Project recreated: old backups cleared, fresh backup created.".into(),
    );

    m.insert(
        "backup.restore_add_count".into(),
        "{} file(s) will be restored.".into(),
    );

    m.insert(
        "backup.restore_cancelled".into(),
        "Restore cancelled.".into(),
    );

    m.insert(
        "backup.restore_confirm".into(),
        "Proceed with restore? (y/N): ".into(),
    );

    m.insert(
        "backup.restore_create_title".into(),
        "\u{26A0} The following files will be CREATED ({}):".into(),
    );

    m.insert(
        "backup.restore_delete_title".into(),
        "\u{26A0} The following files will be DELETED (not in backup) ({}):".into(),
    );

    m.insert(
        "backup.restore_deleted_confirm".into(),
        "Continue with restore-deleted? (y/N): ".into(),
    );

    m.insert(
        "backup.restore_deleted_title".into(),
        " {} files will be restored (were present in backup but are gone now):".into(),
    );

    m.insert(
        "backup.restore_failed".into(),
        "Failed to restore {}: {}".into(),
    );

    m.insert(
        "backup.restore_overwrite_count".into(),
        "{} file(s) will be overwritten.".into(),
    );

    m.insert(
        "backup.restore_overwrite_title".into(),
        "\u{26A0} The following files will be OVERWRITTEN ({}):".into(),
    );

    m.insert(
        "backup.restore_preview_title".into(),
        "Restore Preview".into(),
    );

    m.insert(
        "backup.restored_backup".into(),
        "Restore complete: Backup #{} \u{2014} {} files restored".into(),
    );

    m.insert(
        "backup.restored_date".into(),
        "Restore was performed on: {}".into(),
    );

    m.insert(
        "backup.restored_deleted".into(),
        "Restored deleted files from Backup #{}: {} file(s) restored".into(),
    );

    m.insert(
        "backup.restored_result".into(),
        "Restored backup #{} ({})".into(),
    );

    m.insert(
        "backup.select_entry".into(),
        " {}. Backup #{} \u{2014} {} \u{2014} {} \u{2014} {} files".into(),
    );

    m.insert(
        "backup.select_prompt".into(),
        "Select backup to restore (1-{}): ".into(),
    );

    m.insert(
        "backup.skip_large".into(),
        "Skipped {} ({} MB exceeds 50 MB limit)".into(),
    );

    m.insert(
        "backup.skip_metadata".into(),
        "Skipping {} (metadata error: {})".into(),
    );

    m.insert(
        "backup.skip_rel_path".into(),
        "Skipping {} (could not resolve relative path)".into(),
    );

    m.insert(
        "backup.skip_symlink".into(),
        "Skipping {} (symlink \u{2014} not following)".into(),
    );

    m.insert(
        "backup.skipped_count".into(),
        "Skipped {} file(s) \u{2014} see ignore.txt for details".into(),
    );

    m.insert("backup.skipped_error".into(), "Skipped {} ({})".into());

    m.insert(
        "backup.skipped_large".into(),
        "Skipped {} ({} \u{2014} too large)".into(),
    );

    m.insert(
        "backup.skipped_unsupported".into(),
        "Skipped {} (unsupported file type)".into(),
    );

    m.insert(
        "backup.snapshot_failed".into(),
        "Could not snapshot {} for undo ({}), continuing".into(),
    );

    m.insert(
        "backup.some_restore_failed".into(),
        "Some files could not be restored. Undo storage has been kept.".into(),
    );

    m.insert("backup.stored_in".into(), "Backups stored in: {}".into());

    m.insert(
        "backup.undo_cleared".into(),
        "Undo history cleared (freed {})".into(),
    );

    m.insert(
        "backup.undo_complete".into(),
        "Undo complete: Restored {} file(s) from before last restore".into(),
    );

    m.insert(
        "backup.undo_complete_detail".into(),
        "Undid restore of backup #{} ({} restored, {} deleted, {} re-restored)".into(),
    );

    m.insert(
        "backup.undo_delete_failed".into(),
        "Could not delete restored file {}: {}".into(),
    );

    m.insert(
        "backup.undo_missing_original".into(),
        "Could not restore {} (original missing from undo storage)".into(),
    );

    m.insert(
        "backup.undo_missing_snapshot".into(),
        "Could not re-restore {} (snapshot missing from undo storage)".into(),
    );

    m.insert(
        "backup.undo_no_history".into(),
        "No undo history found.".into(),
    );

    m.insert(
        "backup.undo_none".into(),
        "No restore operation to undo.".into(),
    );

    m.insert(
        "backup.undo_rerestore_failed".into(),
        "Could not re-restore deleted file {}: {}".into(),
    );

    m.insert(
        "backup.unknown_clear_option".into(),
        "Unknown backup clear option: {}".into(),
    );

    m.insert(
        "backup.unknown_create_option".into(),
        "Unknown backup create option: {}".into(),
    );

    m.insert(
        "backup.unknown_recreate_option".into(),
        "Unknown backup recreate option: {}".into(),
    );

    m.insert(
        "backup.unknown_undo_option".into(),
        "Unknown backup undo option: {}".into(),
    );

    m.insert(
        "backup.usage_clear".into(),
        "Usage: backup clear [--force]".into(),
    );

    m.insert(
        "backup.usage_recreate".into(),
        "Usage: backup recreate [--force]".into(),
    );

    m.insert(
        "backup.usage_restore".into(),
        "Usage: backup restore [--choose] [--deleted] [num]".into(),
    );

    m.insert(
        "backup.warning_count".into(),
        "You have {} backup(s):".into(),
    );

    m.insert(
        "backup.warning_title".into(),
        "\u{26A0}\u{FE0F} WARNING: This will delete ALL backups for this project!".into(),
    );

    m.insert(
        "backup.would_overwrite".into(),
        "{} would be overwritten (use --force to bypass)".into(),
    );

    m.insert(
        "backup.would_overwrite_deleted".into(),
        "Restoring deleted file: {}".into(),
    );
}
