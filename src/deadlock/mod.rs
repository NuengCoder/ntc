//! `deadlock` — a standalone shell for an encrypted password vault.
//!
//! ```text
//! deadlock            # unlock the vault, then use its own prompt
//! deadlock> ls        # pass-env -> real password
//! deadlock> add prod_pass CatLoveFish360@house
//! deadlock> rmpw prod_pass
//! deadlock> master-pass
//! deadlock> forget      # drop the OS credential-store entry
//! deadlock> destroy     # delete the vault entirely
//! deadlock> exit
//! ```
//!
//! Secrets live in `~/.config/ntc/deadlock/deadlock.vault`, encrypted with
//! AES-256-GCM under an Argon2id key derived from the master password (see
//! [`vault`]). The default master password is `12345678` and must be changed
//! on first login.
//!
//! The vault is protected against deletion by the OS ([`protect`]) and the
//! master password is registered with the OS credential store ([`credential`])
//! so `nssh` can resolve `--pass-env` names in a fresh process.
//!
//! If the file on disk is structurally unreadable, the shell offers to restore
//! `deadlock.vault.bak` or recreate an empty vault (keeping the broken file as
//! `deadlock.vault.corrupt-<timestamp>`) instead of failing on every launch.

pub mod credential;
pub mod protect;
pub mod vault;

use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use anyhow::Result;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::sync::{LazyLock, Mutex};
use vault::{UnlockError, Vault, DEFAULT_MASTER};

const MAX_UNLOCK_ATTEMPTS: u32 = 3;

/// Process-local cache of the vault master password, populated by
/// `run_deadlock_shell` after a successful unlock and cleared on shell exit.
/// `nssh` reads it silently so a profile that changed its master password
/// still unlocks the vault without an interactive prompt.
static MASTER_CACHE: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));

/// Cache the vault master password for the current process.
pub fn cache_master(master: &str) {
    if let Ok(mut guard) = MASTER_CACHE.lock() {
        *guard = Some(master.to_string());
    }
}

/// Clear the cached master password (called on deadlock shell exit).
pub fn clear_master_cache() {
    if let Ok(mut guard) = MASTER_CACHE.lock() {
        *guard = None;
    }
}

/// Read the cached master password, if any (process-local only).
pub fn get_cached_master() -> Option<String> {
    MASTER_CACHE.lock().ok().and_then(|g| g.clone())
}

/// Guard that clears the master cache when dropped, ensuring the cached
/// password never outlives the `deadlock` shell session — including on
/// normal exit, early `?` returns, and Ctrl-C / interrupt.
struct MasterCacheGuard;

impl Drop for MasterCacheGuard {
    fn drop(&mut self) {
        clear_master_cache();
    }
}

/// Outcome of the corrupt-vault recovery prompt.
enum Recovery {
    /// The previous generation (`.bak`) replaced the unreadable vault.
    Restored,
    /// A fresh empty vault was created; the corrupt file was kept aside.
    Recreated,
    /// The user declined; nothing was changed.
    Aborted,
}

/// Replace the corrupt vault with its one-generation `.bak` copy.
fn restore_backup(path: &std::path::Path) -> Result<(), String> {
    let backup = vault::backup_path(path);
    let _ = protect::unprotect(path);
    let _ = protect::unprotect(&backup);
    std::fs::copy(&backup, path).map_err(|e| format!("{}", e))?;
    #[cfg(not(test))]
    {
        let _ = protect::protect(&backup);
        let _ = protect::protect(path);
    }
    Ok(())
}

/// Move the corrupt vault aside (or delete it if it cannot be moved) and
/// create a fresh default vault at `path`. Returns the kept copy, if any.
fn recreate_vault(path: &std::path::Path) -> Result<Option<std::path::PathBuf>, String> {
    // The barrier blocks rename as well as delete, so drop it first.
    let _ = protect::unprotect(path);
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let kept = path.with_file_name(format!("deadlock.vault.corrupt-{}", stamp));
    let kept = match std::fs::rename(path, &kept) {
        Ok(()) => Some(kept),
        Err(_) => {
            // Fall back to deleting it, but only after the user already
            // confirmed the loss above.
            std::fs::remove_file(path).map_err(|e| format!("{}", e))?;
            None
        }
    };
    // The stored master belonged to the old vault; do not let `nssh` try it.
    let _ = credential::forget_master();
    Vault::create_default(path)?;
    Ok(kept)
}

/// Offer to recover an unreadable vault instead of failing forever.
///
/// A corrupt vault cannot be unlocked, so `destroy` (which requires a working
/// vault) is not an escape hatch. Recovery either restores the one-generation
/// backup or moves the corrupt file aside and creates a fresh vault.
fn recover_corrupt_vault(path: &std::path::Path, reason: &str) -> Result<Recovery> {
    print_error(&t!("deadlock.corrupt_notice", path.display().to_string()));
    println!("{}", reason.dimmed());

    if Vault::probe(&vault::backup_path(path)).is_ok()
        && crate::utils::interactive::prompt_yes_no(&t!("deadlock.corrupt_restore_prompt"), false)?
    {
        restore_backup(path)
            .map_err(|e| anyhow::anyhow!("{}", t!("deadlock.corrupt_restore_failed", e)))?;
        print_success(&t!("deadlock.corrupt_restored"));
        return Ok(Recovery::Restored);
    }

    if !crate::utils::interactive::prompt_yes_no(&t!("deadlock.corrupt_recreate_prompt"), false)? {
        print_info(&t!("deadlock.corrupt_cancelled"));
        return Ok(Recovery::Aborted);
    }

    if let Some(kept) = recreate_vault(path)
        .map_err(|e| anyhow::anyhow!("{}", t!("deadlock.corrupt_recreate_failed", e)))?
    {
        print_info(&t!("deadlock.corrupt_kept", kept.display().to_string()));
    }
    print_success(&t!("deadlock.corrupt_recreated"));
    Ok(Recovery::Recreated)
}

/// Open the deadlock shell: unlock (creating the vault on first run) and run
/// its REPL until the user exits.
pub fn run_deadlock_shell() -> Result<()> {
    let path = vault::ensure_vault_location();
    let mut recreated = false;
    if !vault::exists(&path) {
        Vault::create_default(&path).map_err(|e| anyhow::anyhow!("{}", e))?;
        print_info(&t!("deadlock.created", path.display().to_string()));
        print_info(&t!("deadlock.default_master_hint", DEFAULT_MASTER));
    } else if let Err(UnlockError::Corrupt(reason)) = Vault::probe(&path) {
        match recover_corrupt_vault(&path, &reason)? {
            Recovery::Recreated => recreated = true,
            Recovery::Restored => {}
            Recovery::Aborted => {
                return Err(anyhow::anyhow!("{}", t!("deadlock.corrupt_aborted")));
            }
        }
    }
    if recreated {
        print_info(&t!("deadlock.created", path.display().to_string()));
        print_info(&t!("deadlock.default_master_hint", DEFAULT_MASTER));
    }

    let mut vault = unlock(&path)?;
    cache_master(vault.master());
    // Guard clears the cache on every exit path (normal, `?`, Ctrl-C).
    let _guard = MasterCacheGuard;
    // A vault created or migrated by an older build may predate the OS-level
    // barrier; apply it before showing the status line.
    #[cfg(not(test))]
    {
        let _ = protect::protect(&path);
    }
    if vault.is_default_master() {
        print_warning(&t!("deadlock.force_change_notice"));
        change_master(&mut vault)?;
    }
    // Register the master with the OS credential store so `nssh` can resolve
    // pass-env names in a fresh process without opening this shell.
    if credential::store_master(vault.master()).is_ok() {
        println!("{}", t!("deadlock.keychain_registered").dimmed());
    } else {
        println!("{}", t!("deadlock.keychain_unavailable").dimmed());
    }
    print_success(&t!("deadlock.unlocked"));
    if protect::is_protected(&path) {
        println!("{}", t!("deadlock.os_protected").dimmed());
    } else {
        println!("{}", t!("deadlock.os_unprotected").dimmed());
    }
    shell_loop(&mut vault)
}

/// Prompt for the master password and unlock the vault when one exists.
///
/// `Ok(None)` means there is no vault yet (nothing to unlock). Used by other
/// commands (e.g. `nssh`, `smtp --deadlock`) that read a stored secret.
///
/// The OS credential store is tried first, so a registered master unlocks the
/// vault without prompting.
pub fn unlock_existing() -> Result<Option<Vault>> {
    let path = vault::ensure_vault_location();
    if !vault::exists(&path) {
        return Ok(None);
    }
    if let Some(unlocked) = unlock_silent() {
        return Ok(Some(unlocked));
    }
    unlock(&path).map(Some)
}

/// Unlock the vault without any interactive prompt.
///
/// Tries the process-local master cache, then the OS credential store, then
/// the well-known default master. Returns `None` when the vault is missing or
/// none of those unlock it.
pub fn unlock_silent() -> Option<Vault> {
    let path = vault::ensure_vault_location();
    if !vault::exists(&path) {
        return None;
    }
    let mut candidates: Vec<String> = Vec::new();
    if let Some(cached) = get_cached_master() {
        candidates.push(cached);
    }
    if let Some(stored) = credential::load_master() {
        if !candidates.iter().any(|c| c == &stored) {
            candidates.push(stored);
        }
    }
    if !candidates.iter().any(|c| c == DEFAULT_MASTER) {
        candidates.push(DEFAULT_MASTER.to_string());
    }
    candidates
        .into_iter()
        .find_map(|master| Vault::unlock(&path, &master).ok())
}

/// Look up a pass-env secret without prompting. Used by `nssh`.
pub fn find_secret(name: &str) -> Option<String> {
    unlock_silent().and_then(|unlocked| unlocked.get(name).map(str::to_string))
}

/// Unlock the vault, prompting for the master only when the silent paths
/// fail. A successful prompt is cached for the process and registered with
/// the OS credential store so later calls stay silent.
pub fn unlock_or_prompt() -> Result<Option<Vault>> {
    if let Some(unlocked) = unlock_silent() {
        return Ok(Some(unlocked));
    }
    let path = vault::ensure_vault_location();
    if !vault::exists(&path) {
        return Ok(None);
    }
    let unlocked = unlock(&path)?;
    cache_master(unlocked.master());
    let _ = credential::store_master(unlocked.master());
    Ok(Some(unlocked))
}

/// Like [`unlock_existing`], but a missing vault is an error.
pub fn unlock_required() -> Result<Vault> {
    unlock_existing()?.ok_or_else(|| anyhow::anyhow!("{}", t!("deadlock.vault_missing")))
}

/// Prompt for the master password, allowing a few attempts.
fn unlock(path: &std::path::Path) -> Result<Vault> {
    // Report a structurally broken file instead of pretending it is a wrong
    // password (and burning the three attempts).
    if let Err(err) = Vault::probe(path) {
        return Err(anyhow::anyhow!("{}", err));
    }
    for attempt in 1..=MAX_UNLOCK_ATTEMPTS {
        let master = prompt_hidden(&t!("deadlock.master_prompt"))?;
        match Vault::unlock(path, &master) {
            Ok(v) => return Ok(v),
            Err(UnlockError::WrongPassword) => {
                print_error(&t!("deadlock.wrong_master", attempt.to_string()));
            }
            Err(UnlockError::Other(msg)) => return Err(anyhow::anyhow!("{}", msg)),
            // `probe` above already rules this out, but do not loop on it.
            Err(err @ UnlockError::Corrupt(_)) => return Err(anyhow::anyhow!("{}", err)),
        }
    }
    Err(anyhow::anyhow!("{}", t!("deadlock.too_many_attempts")))
}

/// Ask (twice) for a new master password and re-encrypt the vault.
fn change_master(vault: &mut Vault) -> Result<()> {
    loop {
        let new = prompt_hidden(&t!("deadlock.new_master_prompt"))?;
        if new.trim().is_empty() {
            print_error(&t!("deadlock.empty_master"));
            continue;
        }
        if new == DEFAULT_MASTER {
            print_error(&t!("deadlock.default_master_rejected"));
            continue;
        }
        let confirm = prompt_hidden(&t!("deadlock.confirm_master_prompt"))?;
        if new != confirm {
            print_error(&t!("deadlock.master_mismatch"));
            continue;
        }
        vault
            .change_master(&new)
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        cache_master(&new);
        let _ = credential::store_master(&new);
        print_success(&t!("deadlock.master_changed"));
        return Ok(());
    }
}

/// `master-pass` inside the shell: verify the old password first.
fn change_master_command(vault: &mut Vault) -> Result<()> {
    let old = prompt_hidden(&t!("deadlock.old_master_prompt"))?;
    if !vault.verify_master(&old) {
        print_error(&t!("deadlock.wrong_master_once"));
        return Ok(());
    }
    change_master(vault)
}

/// Hidden input (never echoed, never written to history).
fn prompt_hidden(prompt: &str) -> Result<String> {
    rpassword::prompt_password(prompt).map_err(|e| anyhow::anyhow!("{}", e))
}

/// Print the in-shell command summary.
fn print_shell_help() {
    println!("{}", t!("help.deadlock.title").cyan().bold());
    println!("{}", t!("help.deadlock.ls"));
    println!("{}", t!("help.deadlock.add"));
    println!("{}", t!("help.deadlock.rmpw"));
    println!("{}", t!("help.deadlock.master"));
    println!("{}", t!("help.deadlock.forget"));
    println!("{}", t!("help.deadlock.destroy"));
    println!("{}", t!("help.deadlock.exit"));
    println!();
    println!("{}", t!("help.deadlock.note").dimmed());
}

fn list_entries(vault: &Vault) {
    if vault.entries().is_empty() {
        print_info(&t!("deadlock.ls_empty"));
        return;
    }
    println!("{}", t!("deadlock.ls_title").cyan().bold());
    for (name, password) in vault.entries() {
        println!("  {} {} {}", name.cyan(), "->".dimmed(), password);
    }
}

/// `add <pass-env> <real-password>` (password may be quoted; when omitted it
/// is prompted without echo).
fn add_entry(vault: &mut Vault, tokens: &[String]) -> Result<(), String> {
    let name = tokens
        .get(1)
        .ok_or_else(|| t!("deadlock.add_usage"))?
        .clone();
    if !vault::valid_name(&name) {
        return Err(t!("deadlock.invalid_name", name));
    }
    let password = match tokens.get(2) {
        Some(value) if !value.is_empty() => value.clone(),
        _ => rpassword::prompt_password(&t!("deadlock.value_prompt"))
            .map_err(|e| format!("{}", e))?,
    };
    if password.is_empty() {
        return Err(t!("deadlock.empty_password"));
    }
    let created = vault.add(&name, &password)?;
    // Register the master with the OS credential store. This is what makes
    // `nssh add ... --pass-env <name>` resolve the secret in a fresh process
    // without opening the deadlock shell first.
    let _ = credential::store_master(vault.master());
    if created {
        print_success(&t!("deadlock.added", name));
    } else {
        print_success(&t!("deadlock.updated", name));
    }
    Ok(())
}

fn remove_entry(vault: &mut Vault, tokens: &[String]) -> Result<(), String> {
    let name = tokens
        .get(1)
        .ok_or_else(|| t!("deadlock.rmpw_usage"))?
        .clone();
    if vault.remove(&name)? {
        print_success(&t!("deadlock.removed", name));
    } else {
        print_error(&t!("deadlock.not_found", name));
    }
    Ok(())
}

/// `forget` — remove the master password from the OS credential store.
fn forget_credential() {
    match credential::forget_master() {
        Ok(()) => print_success(&t!("deadlock.forget_done")),
        Err(e) => print_error(&t!("deadlock.forget_failed", e)),
    }
}

/// `destroy` — permanently delete the vault (the supported way to remove it,
/// since the file is normally protected against deletion). Returns true when
/// the vault was deleted.
fn destroy_vault(vault: &Vault) -> Result<bool> {
    let confirmed =
        crate::utils::interactive::prompt_yes_no(&t!("deadlock.destroy_confirm"), false)?;
    if !confirmed {
        print_info(&t!("deadlock.destroy_cancelled"));
        return Ok(false);
    }

    let path = vault.path().to_path_buf();
    let _ = credential::forget_master();
    // Drop the OS-level deletion barrier first; otherwise remove_file fails
    // even for the owner (Windows deny ACE / macOS uchg).
    let _ = protect::unprotect(&path);
    std::fs::remove_file(&path)
        .map_err(|e| anyhow::anyhow!("{}", t!("deadlock.destroy_failed", e)))?;
    print_success(&t!("deadlock.destroyed"));
    Ok(true)
}

/// Split a line into tokens, honouring single and double quotes.
fn tokenize(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut single = false;
    let mut double = false;
    for ch in input.chars() {
        match ch {
            '\'' if !double => single = !single,
            '"' if !single => double = !double,
            ' ' | '\t' if !single && !double => {
                if !current.is_empty() {
                    out.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

/// `ntc_history.txt` -> `ntc_history.txt.deadlock`.
fn deadlock_history_path(base: &std::path::Path) -> std::path::PathBuf {
    let mut path = base.to_path_buf();
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "ntc_history".to_string());
    path.set_file_name(format!("{}.deadlock", name));
    path
}

fn shell_loop(vault: &mut Vault) -> Result<()> {
    let mut rl =
        DefaultEditor::new().map_err(|e| anyhow::anyhow!("{}", t!("deadlock.editor_fail", e)))?;
    let history_path = crate::config::Config::read_global()
        .resolve_history_path()
        .map(|p| deadlock_history_path(&p));
    if let Some(ref hp) = history_path {
        let _ = rl.load_history(hp);
    }

    print_shell_help();
    loop {
        let line = match rl.readline("deadlock> ") {
            Ok(line) => {
                rl.add_history_entry(&line).ok();
                line
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                print_error(&format!("{}", e));
                break;
            }
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let tokens = tokenize(trimmed);
        let cmd = tokens[0].to_lowercase();
        match cmd.as_str() {
            "ls" | "list" => list_entries(vault),
            "add" => {
                if let Err(e) = add_entry(vault, &tokens) {
                    print_error(&e);
                }
            }
            "rmpw" | "rm" => {
                if let Err(e) = remove_entry(vault, &tokens) {
                    print_error(&e);
                }
            }
            "master-pass" | "masterpass" | "passwd" => {
                if let Err(e) = change_master_command(vault) {
                    print_error(&format!("{}", e));
                }
            }
            "forget" | "forget-master" | "unremember" => forget_credential(),
            "destroy" | "remove-vault" => match destroy_vault(vault) {
                Ok(true) => break,
                Ok(false) => {}
                Err(e) => print_error(&format!("{}", e)),
            },
            "help" | "?" | "-h" | "--help" => print_shell_help(),
            "exit" | "quit" | "esc" => break,
            other => {
                print_error(&t!("deadlock.unknown_cmd", other));
                println!("{}", t!("deadlock.hint_help").dimmed());
            }
        }
    }

    if let Some(hp) = history_path {
        let _ = rl.save_history(&hp);
    }
    println!("{}", t!("deadlock.bye").dimmed());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_keeps_quoted_passwords_together() {
        assert_eq!(
            tokenize("add prod_pass \"Cat Love Fish\""),
            vec![
                "add".to_string(),
                "prod_pass".to_string(),
                "Cat Love Fish".to_string()
            ]
        );
        assert_eq!(
            tokenize("add 'x y' z"),
            vec!["add".to_string(), "x y".to_string(), "z".to_string()]
        );
        assert!(tokenize("   ").is_empty());
    }

    #[test]
    fn history_path_gets_deadlock_suffix() {
        let p = deadlock_history_path(std::path::Path::new("/tmp/ntc_history.txt"));
        assert_eq!(
            p.file_name().unwrap().to_string_lossy(),
            "ntc_history.txt.deadlock"
        );
    }

    #[test]
    fn master_cache_roundtrip() {
        clear_master_cache();
        cache_master("s3cr3t");
        assert_eq!(get_cached_master().as_deref(), Some("s3cr3t"));
        clear_master_cache();
        assert!(get_cached_master().is_none());
    }

    #[test]
    fn master_cache_guard_clears_on_drop() {
        clear_master_cache();
        cache_master("temp-master");
        assert_eq!(get_cached_master().as_deref(), Some("temp-master"));
        {
            let _guard = MasterCacheGuard;
        }
        assert!(get_cached_master().is_none());
        clear_master_cache();
    }

    #[test]
    fn restore_backup_recovers_the_previous_generation() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("deadlock.vault");
        Vault::create_default(&path).unwrap();
        let mut vault = Vault::unlock(&path, DEFAULT_MASTER).unwrap();
        vault.add("prod", "s3cr3t").unwrap();
        // A second save moves "s3cr3t" into the backup slot.
        vault.add("prod", "rotated").unwrap();

        std::fs::write(&path, "dummy vault content").unwrap();
        assert!(Vault::probe(&path).is_err());

        restore_backup(&path).unwrap();
        let restored = Vault::unlock(&path, DEFAULT_MASTER).unwrap();
        assert_eq!(restored.get("prod"), Some("s3cr3t"));
    }

    #[test]
    fn recreate_vault_keeps_the_corrupt_file_aside() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("deadlock.vault");
        std::fs::write(&path, "dummy vault content").unwrap();

        let kept = recreate_vault(&path)
            .expect("recreate should succeed")
            .expect("corrupt file should be kept");
        assert_eq!(
            std::fs::read_to_string(&kept).unwrap(),
            "dummy vault content"
        );
        assert!(Vault::probe(&path).is_ok());
        assert!(Vault::unlock(&path, DEFAULT_MASTER).is_ok());
    }
}
