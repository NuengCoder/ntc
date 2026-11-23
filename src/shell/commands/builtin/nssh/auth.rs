use crate::output::{print_info, print_warning};
use crate::shell::commands::builtin::nssh::nssh::ssh as transport;
use crate::shell::commands::builtin::nssh::nssh::store;
use crate::t;
use anyhow::Result;

pub(crate) fn rt() -> Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| anyhow::anyhow!("cannot start async runtime: {}", e))
}

/// Build auth from profile: key from `profile.key`, password from the
/// `pass_env` name — the deadlock vault (master cached in this process, the
/// OS credential store, then the default master) or an environment variable.
/// Never from argv, never stored in the profile file.
///
/// When the vault cannot be unlocked silently and no env var is set, the user
/// is asked for the master password once; it is then cached for the process
/// and registered in the OS credential store.
pub(crate) fn build_auth(profile: &store::Profile) -> Result<transport::SshAuth> {
    let key_path = profile.key.as_deref().map(store::expand_home);
    if let Some(ref var) = profile.pass_env {
        let mut vault_checked = false;
        if let Some(vault) = crate::shell::commands::builtin::deadlock::deadlock::unlock_silent() {
            vault_checked = true;
            if let Some(secret) = vault.get(var) {
                print_info(&t!("cmd.nssh.pass_env_from_vault", var));
                return Ok(transport::SshAuth {
                    key_path,
                    password: Some(secret.to_string()),
                });
            }
        }
        // Environment variable fallback: no prompt needed.
        if let Ok(value) = std::env::var(var) {
            if !value.is_empty() {
                return Ok(transport::SshAuth {
                    key_path,
                    password: Some(value),
                });
            }
        }
        // The vault exists but was not silently unlockable — ask for the
        // master once (hidden) and remember it.
        if !vault_checked
            && crate::shell::commands::builtin::deadlock::deadlock::vault::exists(
                &crate::shell::commands::builtin::deadlock::deadlock::vault::ensure_vault_location(
                ),
            )
        {
            match crate::shell::commands::builtin::deadlock::deadlock::unlock_or_prompt() {
                Ok(Some(vault)) => {
                    if let Some(secret) = vault.get(var) {
                        print_info(&t!("cmd.nssh.pass_env_from_vault", var));
                        return Ok(transport::SshAuth {
                            key_path,
                            password: Some(secret.to_string()),
                        });
                    }
                    print_warning(&t!("cmd.nssh.pass_env_not_in_vault", var));
                }
                Ok(None) => {}
                Err(e) => print_warning(&t!("cmd.nssh.master_unlock_failed", e)),
            }
        }
        print_warning(&t!("cmd.nssh.pass_env_unset", var));
    }
    Ok(transport::SshAuth {
        key_path,
        password: None,
    })
}

/// True when a connect error is an SSH authentication failure worth retrying
/// with a freshly prompted password.
pub(crate) fn is_auth_failure(err: &anyhow::Error) -> bool {
    let msg = err.to_string();
    msg.contains("authentication failed") || msg.contains("key auth failed")
}
