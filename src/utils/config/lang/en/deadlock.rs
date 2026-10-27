use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "deadlock.no_args".into(),
        "deadlock opens its own shell and takes no arguments (use --help to see the commands)."
            .into(),
    );

    m.insert(
        "deadlock.created".into(),
        "Created a new vault at {}.".into(),
    );

    m.insert(
        "deadlock.default_master_hint".into(),
        "Default master password: {} - you will be asked to change it.".into(),
    );

    m.insert("deadlock.master_prompt".into(), "master-pass: ".into());

    m.insert(
        "deadlock.wrong_master".into(),
        "Wrong master password (attempt {}/3).".into(),
    );

    m.insert(
        "deadlock.too_many_attempts".into(),
        "Too many wrong master passwords - aborting.".into(),
    );

    m.insert(
        "deadlock.force_change_notice".into(),
        "The default master password is still in use; set a new one now.".into(),
    );

    m.insert(
        "deadlock.new_master_prompt".into(),
        "new master-pass: ".into(),
    );

    m.insert(
        "deadlock.confirm_master_prompt".into(),
        "confirm new master-pass: ".into(),
    );

    m.insert(
        "deadlock.empty_master".into(),
        "The master password cannot be empty.".into(),
    );

    m.insert(
        "deadlock.default_master_rejected".into(),
        "Pick a master password other than the default.".into(),
    );

    m.insert(
        "deadlock.master_mismatch".into(),
        "Passwords do not match; try again.".into(),
    );

    m.insert(
        "deadlock.master_changed".into(),
        "Master password changed; the vault was re-encrypted.".into(),
    );

    m.insert(
        "deadlock.old_master_prompt".into(),
        "old master-pass: ".into(),
    );

    m.insert(
        "deadlock.wrong_master_once".into(),
        "Wrong master password.".into(),
    );

    m.insert("deadlock.unlocked".into(), "Vault unlocked.".into());

    m.insert(
        "deadlock.add_usage".into(),
        "Usage: add <pass-env> <real-password>".into(),
    );

    m.insert(
        "deadlock.invalid_name".into(),
        "Invalid pass-env name '{}' (letters, digits, _ - . only).".into(),
    );

    m.insert("deadlock.value_prompt".into(), "real-password: ".into());

    m.insert(
        "deadlock.empty_password".into(),
        "The password cannot be empty.".into(),
    );

    m.insert("deadlock.added".into(), "Saved pass-env '{}'.".into());

    m.insert("deadlock.updated".into(), "Updated pass-env '{}'.".into());

    m.insert(
        "deadlock.ls_title".into(),
        "PASS-ENV -> REAL PASSWORD:".into(),
    );

    m.insert(
        "deadlock.ls_empty".into(),
        "The vault is empty. Add one with: add <pass-env> <real-password>".into(),
    );

    m.insert(
        "deadlock.rmpw_usage".into(),
        "Usage: rmpw <pass-env>".into(),
    );

    m.insert("deadlock.removed".into(), "Removed pass-env '{}'.".into());

    m.insert(
        "deadlock.not_found".into(),
        "No pass-env named '{}'.".into(),
    );

    m.insert(
        "deadlock.unknown_cmd".into(),
        "Unknown deadlock command '{}'.".into(),
    );

    m.insert(
        "deadlock.hint_help".into(),
        "Type 'help' to list deadlock commands.".into(),
    );

    m.insert("deadlock.bye".into(), "deadlock closed.".into());

    m.insert(
        "deadlock.editor_fail".into(),
        "Cannot start the line editor: {}".into(),
    );

    m.insert("deadlock.keychain_registered".into(), "Master registered in the OS credential store (nssh can unlock the vault; use 'forget' to remove it).".into());

    m.insert(
        "deadlock.keychain_unavailable".into(),
        "OS credential store unavailable - nssh will ask for the master password.".into(),
    );

    m.insert(
        "deadlock.forget_done".into(),
        "Removed the master password from the OS credential store.".into(),
    );

    m.insert(
        "deadlock.forget_failed".into(),
        "Could not remove the master password from the OS credential store: {}".into(),
    );

    m.insert(
        "deadlock.destroy_confirm".into(),
        "Delete the vault and every stored secret permanently?".into(),
    );

    m.insert(
        "deadlock.destroy_cancelled".into(),
        "Vault deletion cancelled.".into(),
    );

    m.insert("deadlock.destroyed".into(), "Vault deleted.".into());

    m.insert(
        "deadlock.destroy_failed".into(),
        "Could not delete the vault: {}".into(),
    );

    m.insert(
        "deadlock.os_protected".into(),
        "Vault file is protected against deletion by the OS.".into(),
    );

    m.insert("deadlock.os_unprotected".into(), "OS-level deletion protection is unavailable on this platform/account (file permissions still apply).".into());

    m.insert(
        "deadlock.vault_missing".into(),
        "No deadlock vault yet - run `deadlock` once to create it.".into(),
    );

    m.insert(
        "deadlock.corrupt_notice".into(),
        "The vault file at {} is unreadable (not a valid deadlock vault).".into(),
    );

    m.insert(
        "deadlock.corrupt_restore_prompt".into(),
        "A backup copy of the vault is available. Restore it? [y/N]".into(),
    );

    m.insert(
        "deadlock.corrupt_restore_failed".into(),
        "Could not restore the backup: {}".into(),
    );

    m.insert(
        "deadlock.corrupt_restored".into(),
        "Vault restored from the backup copy.".into(),
    );

    m.insert(
        "deadlock.corrupt_recreate_prompt".into(),
        "Recreate an empty vault now? The unreadable file is kept next to the vault. [y/N]".into(),
    );

    m.insert(
        "deadlock.corrupt_cancelled".into(),
        "Recovery cancelled; the vault was left untouched.".into(),
    );

    m.insert(
        "deadlock.corrupt_aborted".into(),
        "The vault is unreadable and must be recovered before deadlock can open.".into(),
    );

    m.insert(
        "deadlock.corrupt_kept".into(),
        "Unreadable vault kept as {}.".into(),
    );

    m.insert(
        "deadlock.corrupt_recreate_failed".into(),
        "Could not move the unreadable vault aside: {}".into(),
    );

    m.insert(
        "deadlock.corrupt_recreated".into(),
        "Created a new empty vault.".into(),
    );
}
