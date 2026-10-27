//! Encrypted password vault used by the `deadlock` shell.
//!
//! The whole vault is one file (`~/.config/ntc/deadlock/deadlock.vault`) holding
//! AES-256-GCM ciphertext. The encryption key is derived from the master
//! password with Argon2id; the file only stores the KDF parameters, a random
//! salt and a random nonce, so no stored secret is readable on disk.
//!
//! Every save keeps the previous generation as `deadlock.vault.bak`, and a
//! structurally unreadable vault can be recovered (restore the backup, or
//! recreate an empty vault) instead of blocking `deadlock` forever.

use super::protect;
use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Nonce};
use argon2::{Algorithm, Argon2, Params, Version};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

/// Master password a brand-new vault starts with.
pub const DEFAULT_MASTER: &str = "12345678";

const VAULT_VERSION: u32 = 1;
const KDF_NAME: &str = "argon2id";
/// Argon2id cost: ~19 MiB, 2 passes, 1 lane (interactive-safe).
const KDF_M_COST: u32 = 19_456;
const KDF_T_COST: u32 = 2;
const KDF_P_COST: u32 = 1;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;
/// Bound the KDF parameters read back from disk so a tampered file cannot
/// make unlocking allocate absurd amounts of memory.
const MAX_M_COST: u32 = 1_048_576;
const MAX_T_COST: u32 = 32;
const MAX_P_COST: u32 = 16;
/// Extra authenticated data ties ciphertexts to this format/version.
const AAD: &[u8] = b"ntc-deadlock-v1";

/// Failure modes when unlocking a vault.
#[derive(Debug)]
pub enum UnlockError {
    /// The master password did not decrypt the vault.
    WrongPassword,
    /// The file exists but is not a structurally valid vault (bad TOML/hex,
    /// unsupported version or KDF, corrupt payload). Unlike [`Self::Other`]
    /// this is recoverable without the master password, so callers may offer
    /// to restore a backup or recreate the vault.
    Corrupt(String),
    /// The file is missing or unreadable for another reason.
    Other(String),
}

impl std::fmt::Display for UnlockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnlockError::WrongPassword => write!(f, "wrong master password"),
            UnlockError::Corrupt(msg) => write!(f, "vault file is corrupt: {}", msg),
            UnlockError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for UnlockError {}

/// On-disk header + ciphertext.
#[derive(Debug, Serialize, Deserialize)]
struct VaultFile {
    version: u32,
    kdf: String,
    m_cost: u32,
    t_cost: u32,
    p_cost: u32,
    /// Random salt (hex).
    salt: String,
    /// Random nonce (hex).
    nonce: String,
    /// Ciphertext of the entries payload (hex).
    data: String,
}

/// Plaintext that is encrypted into `VaultFile::data`.
#[derive(Debug, Default, Serialize, Deserialize)]
struct VaultPayload {
    #[serde(default)]
    entries: BTreeMap<String, String>,
}

/// An unlocked vault. Drop it and the master password leaves memory with it.
pub struct Vault {
    path: PathBuf,
    master: String,
    salt: [u8; SALT_LEN],
    entries: BTreeMap<String, String>,
}

/// `~/.config/ntc/deadlock/deadlock.vault` (APPDATA on Windows).
///
/// The vault lives in its own directory so the directory itself can carry a
/// deny-`FILE_DELETE_CHILD` ACE on Windows (a file-level deny alone can be
/// bypassed through the parent directory's delete-child right).
pub fn vault_path() -> PathBuf {
    crate::utils::paths::config_home()
        .join("ntc")
        .join("deadlock")
        .join("deadlock.vault")
}

/// Old location used before the vault got its own directory.
pub fn legacy_vault_path() -> PathBuf {
    crate::utils::paths::config_home()
        .join("ntc")
        .join("deadlock.vault")
}

/// One-generation rollback copy written before every in-place vault update.
pub fn backup_path(path: &Path) -> PathBuf {
    path.with_file_name("deadlock.vault.bak")
}

/// Return the active vault path, moving a vault from the legacy location into
/// the protected directory on first use.
pub fn ensure_vault_location() -> PathBuf {
    let new = vault_path();
    let old = legacy_vault_path();
    if !new.exists() && old.exists() {
        if let Some(parent) = new.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        // A previous build may have added an OS-level barrier to the old file.
        let _ = protect::unprotect(&old);
        let moved = if std::fs::rename(&old, &new).is_ok() {
            true
        } else if std::fs::copy(&old, &new).is_ok() {
            // Cross-device or locked: copy succeeded, drop the old file.
            let _ = std::fs::remove_file(&old);
            true
        } else {
            false
        };
        if moved {
            // Protect the moved vault immediately; it may never be written
            // again (a read-only `nssh` unlock must not leave it exposed).
            #[cfg(not(test))]
            {
                let _ = protect::protect(&new);
            }
            return new;
        }
        // Migration failed: keep using the legacy vault instead of silently
        // creating a fresh, empty one at the new path.
        return old;
    }
    new
}

/// True when a vault file already exists at `path`.
pub fn exists(path: &Path) -> bool {
    path.is_file()
}

/// Pass-env names must be simple tokens (also keeps future `pass-env`
/// references unambiguous).
pub fn valid_name(name: &str) -> bool {
    !name.trim().is_empty()
        && name.len() <= 64
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
}

impl Vault {
    /// Create an empty vault protected by [`DEFAULT_MASTER`] when `path` does
    /// not exist yet.
    pub fn create_default(path: &Path) -> Result<(), String> {
        if path.exists() {
            return Ok(());
        }
        let salt: [u8; SALT_LEN] = rand::random();
        let file = seal(DEFAULT_MASTER, salt, &BTreeMap::new())?;
        write_file(path, &file)
    }

    /// Decrypt the vault at `path` with `master`.
    pub fn unlock(path: &Path, master: &str) -> Result<Vault, UnlockError> {
        let (file, salt, nonce) = read_header(path)?;
        let ciphertext = hex::decode(&file.data)
            .map_err(|e| UnlockError::Corrupt(format!("vault data is not valid hex: {}", e)))?;
        let key = derive_key(
            master,
            &salt,
            file.m_cost.clamp(8, MAX_M_COST),
            file.t_cost.clamp(1, MAX_T_COST),
            file.p_cost.clamp(1, MAX_P_COST),
        )
        .map_err(UnlockError::Other)?;
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| UnlockError::Other(format!("cannot init cipher: {}", e)))?;
        let nonce = Nonce::try_from(nonce.as_slice())
            .map_err(|_| UnlockError::Other("vault nonce has the wrong length".to_string()))?;
        let plaintext = cipher
            .decrypt(
                &nonce,
                Payload {
                    msg: &ciphertext,
                    aad: AAD,
                },
            )
            .map_err(|_| UnlockError::WrongPassword)?;
        let payload: VaultPayload = serde_json::from_slice(&plaintext)
            .map_err(|e| UnlockError::Corrupt(format!("vault payload is corrupt: {}", e)))?;
        Ok(Vault {
            path: path.to_path_buf(),
            master: master.to_string(),
            salt,
            entries: payload.entries,
        })
    }

    /// Validate the on-disk structure without a master password.
    ///
    /// Used before prompting so a corrupt file can be reported (and recovered)
    /// instead of looking like a wrong password.
    pub fn probe(path: &Path) -> Result<(), UnlockError> {
        read_header(path).map(|_| ())
    }

    /// True while the vault still uses the well-known default master password.
    pub fn is_default_master(&self) -> bool {
        self.master == DEFAULT_MASTER
    }

    /// Check a candidate against the master password that unlocked the vault.
    pub fn verify_master(&self, candidate: &str) -> bool {
        self.master == candidate
    }

    /// The master password that unlocked this vault (for caching).
    pub fn master(&self) -> &str {
        &self.master
    }

    /// Path of the vault file on disk.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// All stored pass-envs (name -> real password), sorted by name.
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.entries
    }

    /// Look up one secret (used by `nssh` / `smtp --deadlock` later).
    pub fn get(&self, name: &str) -> Option<&str> {
        self.entries.get(name).map(|s| s.as_str())
    }

    /// Store or update a secret. Returns true when the name was new.
    pub fn add(&mut self, name: &str, password: &str) -> Result<bool, String> {
        if !valid_name(name) {
            return Err(format!("invalid pass-env name '{}'", name));
        }
        if password.is_empty() {
            return Err("password cannot be empty".to_string());
        }
        let created = self
            .entries
            .insert(name.to_string(), password.to_string())
            .is_none();
        self.save()?;
        Ok(created)
    }

    /// Delete a secret. Returns false when the name was not stored anyway.
    pub fn remove(&mut self, name: &str) -> Result<bool, String> {
        let removed = self.entries.remove(name).is_some();
        if removed {
            self.save()?;
        }
        Ok(removed)
    }

    /// Re-encrypt everything under a new master password (new salt).
    pub fn change_master(&mut self, new_master: &str) -> Result<(), String> {
        if new_master.is_empty() {
            return Err("master password cannot be empty".to_string());
        }
        // Encrypt before mutating so a failure keeps the vault usable.
        let salt: [u8; SALT_LEN] = rand::random();
        let file = seal(new_master, salt, &self.entries)?;
        write_file(&self.path, &file)?;
        self.master = new_master.to_string();
        self.salt = salt;
        Ok(())
    }

    fn save(&self) -> Result<(), String> {
        let file = seal(&self.master, self.salt, &self.entries)?;
        write_file(&self.path, &file)
    }
}

/// Build an encrypted vault file for `entries`.
fn seal(
    master: &str,
    salt: [u8; SALT_LEN],
    entries: &BTreeMap<String, String>,
) -> Result<VaultFile, String> {
    let key =
        derive_key(master, &salt, KDF_M_COST, KDF_T_COST, KDF_P_COST).map_err(|e| e.to_string())?;
    let cipher =
        Aes256Gcm::new_from_slice(&key).map_err(|e| format!("cannot init cipher: {}", e))?;
    let nonce_bytes: [u8; NONCE_LEN] = rand::random();
    let payload = VaultPayload {
        entries: entries.clone(),
    };
    let plaintext =
        serde_json::to_vec(&payload).map_err(|e| format!("cannot encode vault: {}", e))?;
    let nonce =
        Nonce::try_from(nonce_bytes.as_slice()).map_err(|_| "internal nonce error".to_string())?;
    let ciphertext = cipher
        .encrypt(
            &nonce,
            Payload {
                msg: &plaintext,
                aad: AAD,
            },
        )
        .map_err(|_| "encryption failed".to_string())?;
    Ok(VaultFile {
        version: VAULT_VERSION,
        kdf: KDF_NAME.to_string(),
        m_cost: KDF_M_COST,
        t_cost: KDF_T_COST,
        p_cost: KDF_P_COST,
        salt: hex::encode(salt),
        nonce: hex::encode(nonce_bytes),
        data: hex::encode(ciphertext),
    })
}

/// Argon2id -> 32-byte key.
fn derive_key(master: &str, salt: &[u8], m: u32, t: u32, p: u32) -> Result<[u8; KEY_LEN], String> {
    let params = Params::new(m, t, p, Some(KEY_LEN))
        .map_err(|e| format!("invalid KDF parameters: {}", e))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; KEY_LEN];
    argon2
        .hash_password_into(master.as_bytes(), salt, &mut key)
        .map_err(|e| format!("key derivation failed: {}", e))?;
    Ok(key)
}

/// Decode a hex field that must have exactly `N` bytes.
fn decode_exact<const N: usize>(value: &str, field: &str) -> Result<[u8; N], UnlockError> {
    let bytes = hex::decode(value)
        .map_err(|e| UnlockError::Corrupt(format!("vault {} is not valid hex: {}", field, e)))?;
    bytes
        .try_into()
        .map_err(|_| UnlockError::Corrupt(format!("vault {} has the wrong length", field)))
}

/// Read and validate the vault header, returning the parsed fields and the
/// decoded salt/nonce. Shared by [`Vault::unlock`] and [`Vault::probe`].
fn read_header(path: &Path) -> Result<(VaultFile, [u8; SALT_LEN], [u8; NONCE_LEN]), UnlockError> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| UnlockError::Other(format!("cannot read vault: {}", e)))?;
    let file: VaultFile = toml::from_str(&text)
        .map_err(|e| UnlockError::Corrupt(format!("vault file is not valid TOML: {}", e)))?;
    if file.version != VAULT_VERSION {
        return Err(UnlockError::Corrupt(format!(
            "unsupported vault version {}",
            file.version
        )));
    }
    if file.kdf != KDF_NAME {
        return Err(UnlockError::Corrupt(format!(
            "unsupported vault KDF '{}'",
            file.kdf
        )));
    }
    let salt = decode_exact::<SALT_LEN>(&file.salt, "salt")?;
    let nonce = decode_exact::<NONCE_LEN>(&file.nonce, "nonce")?;
    // Validate the ciphertext encoding here so `probe` catches it too.
    hex::decode(&file.data)
        .map_err(|e| UnlockError::Corrupt(format!("vault data is not valid hex: {}", e)))?;
    Ok((file, salt, nonce))
}

/// Write the vault, creating the parent directory and restricting access.
fn write_file(path: &Path, file: &VaultFile) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("cannot create vault directory: {}", e))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o700));
        }
    }
    let text = toml::to_string(file).map_err(|e| format!("cannot encode vault: {}", e))?;

    // Keep the previous generation so an interrupted/failed write (or an
    // accidental overwrite) can be recovered instead of losing every secret.
    backup_existing(path);

    // The vault may carry an OS-level deletion barrier (deny-DELETE ACE,
    // immutable flag). Remove it for the duration of the write. We write in
    // place instead of using a temp-file + rename because rename needs DELETE
    // permission on Windows and would be blocked by the barrier.
    let _ = protect::unprotect(path);

    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut handle = options
        .open(path)
        .map_err(|e| format!("cannot write vault: {}", e))?;
    handle
        .set_len(0)
        .map_err(|e| format!("cannot truncate vault: {}", e))?;
    handle
        .seek(SeekFrom::Start(0))
        .map_err(|e| format!("cannot seek vault: {}", e))?;
    handle
        .write_all(text.as_bytes())
        .map_err(|e| format!("cannot write vault: {}", e))?;
    handle
        .sync_all()
        .map_err(|e| format!("cannot flush vault: {}", e))?;
    drop(handle);

    restrict_permissions(path);
    // Unit tests create throw-away vaults in temp dirs; hardening every one
    // of them would leave protected files behind. The barrier itself is
    // covered by dedicated tests in `protect`.
    #[cfg(not(test))]
    {
        let _ = protect::protect(path);
    }
    Ok(())
}

/// Copy the current vault to [`backup_path`] before it is overwritten.
///
/// Best effort: a vault that cannot be copied (fresh install, locked file)
/// simply has no backup, and saving continues.
fn backup_existing(path: &Path) {
    if !path.is_file() {
        return;
    }
    let backup = backup_path(path);
    // An older backup may carry the deny-DELETE barrier; clear it first.
    let _ = protect::unprotect(&backup);
    if std::fs::copy(path, &backup).is_err() {
        return;
    }
    restrict_permissions(&backup);
    #[cfg(not(test))]
    {
        let _ = protect::protect(&backup);
    }
}

/// Best-effort OS-level hardening: owner-only file on Unix, an explicit
/// owner-only ACL on Windows (so other accounts cannot read the vault).
fn restrict_permissions(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
    }
    #[cfg(windows)]
    {
        if let Ok(user) = std::env::var("USERNAME") {
            if user.is_empty() {
                return;
            }
            let _ = std::process::Command::new("icacls")
                .arg(path)
                .arg("/inheritance:r")
                .arg("/grant:r")
                .arg(format!("{}:F", user))
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_vault() -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("deadlock.vault");
        (dir, path)
    }

    #[test]
    fn create_unlock_roundtrip() {
        let (_dir, path) = temp_vault();
        Vault::create_default(&path).unwrap();
        assert!(exists(&path));

        let vault = Vault::unlock(&path, DEFAULT_MASTER).unwrap();
        assert!(vault.is_default_master());
        assert!(vault.entries().is_empty());
    }

    #[test]
    fn wrong_master_is_rejected() {
        let (_dir, path) = temp_vault();
        Vault::create_default(&path).unwrap();
        match Vault::unlock(&path, "not-the-password") {
            Err(UnlockError::WrongPassword) => {}
            other => panic!("expected WrongPassword, got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn add_get_remove_persist() {
        let (_dir, path) = temp_vault();
        Vault::create_default(&path).unwrap();
        let mut vault = Vault::unlock(&path, DEFAULT_MASTER).unwrap();
        assert!(vault.add("prod_pass", "CatLoveFish360@house").unwrap());
        assert!(!vault.add("prod_pass", "rotated").unwrap());
        assert_eq!(vault.get("prod_pass"), Some("rotated"));

        // Re-open: the secret survived on disk, under the same master.
        let reloaded = Vault::unlock(&path, DEFAULT_MASTER).unwrap();
        assert_eq!(reloaded.get("prod_pass"), Some("rotated"));

        let mut reloaded = reloaded;
        assert!(reloaded.remove("prod_pass").unwrap());
        assert!(!reloaded.remove("prod_pass").unwrap());
        assert!(Vault::unlock(&path, DEFAULT_MASTER)
            .unwrap()
            .get("prod_pass")
            .is_none());
    }

    #[test]
    fn master_change_reencrypts() {
        let (_dir, path) = temp_vault();
        Vault::create_default(&path).unwrap();
        let mut vault = Vault::unlock(&path, DEFAULT_MASTER).unwrap();
        vault.add("token", "s3cr3t").unwrap();
        vault.change_master("a-better-master").unwrap();
        assert!(!vault.is_default_master());

        assert!(matches!(
            Vault::unlock(&path, DEFAULT_MASTER),
            Err(UnlockError::WrongPassword)
        ));
        let reopened = Vault::unlock(&path, "a-better-master").unwrap();
        assert_eq!(reopened.get("token"), Some("s3cr3t"));
    }

    #[test]
    fn secrets_are_not_stored_in_plaintext() {
        let (_dir, path) = temp_vault();
        Vault::create_default(&path).unwrap();
        let mut vault = Vault::unlock(&path, DEFAULT_MASTER).unwrap();
        vault.add("db_pass", "Pl41nt3xt-S3cret").unwrap();
        let raw = std::fs::read(&path).unwrap();
        let needle = b"Pl41nt3xt-S3cret";
        assert!(
            !raw.windows(needle.len()).any(|w| w == needle),
            "vault file leaked the secret in plaintext"
        );
        // ...and neither the name nor the master password appear in cleartext.
        let needle = b"db_pass";
        assert!(!raw.windows(needle.len()).any(|w| w == needle));
        let needle = DEFAULT_MASTER.as_bytes();
        assert!(!raw.windows(needle.len()).any(|w| w == needle));
    }

    #[test]
    fn probe_rejects_non_vault_content() {
        let (_dir, path) = temp_vault();
        std::fs::write(&path, "dummy vault content").unwrap();
        assert!(matches!(Vault::probe(&path), Err(UnlockError::Corrupt(_))));
        assert!(matches!(
            Vault::unlock(&path, DEFAULT_MASTER),
            Err(UnlockError::Corrupt(_))
        ));
    }

    #[test]
    fn save_keeps_the_previous_generation_as_backup() {
        let (_dir, path) = temp_vault();
        Vault::create_default(&path).unwrap();
        // Nothing existed before the first save, so there is no backup yet.
        assert!(!backup_path(&path).exists());

        let mut vault = Vault::unlock(&path, DEFAULT_MASTER).unwrap();
        vault.add("prod_pass", "first").unwrap();
        // The backup holds the state *before* this write (still empty).
        let backup = Vault::unlock(&backup_path(&path), DEFAULT_MASTER).unwrap();
        assert!(backup.entries().is_empty());

        vault.add("prod_pass", "second").unwrap();
        let backup = Vault::unlock(&backup_path(&path), DEFAULT_MASTER).unwrap();
        assert_eq!(backup.get("prod_pass"), Some("first"));
        assert!(Vault::probe(&backup_path(&path)).is_ok());
    }

    #[test]
    fn valid_names_match_pass_env_rules() {
        assert!(valid_name("prod_pass"));
        assert!(valid_name("web-01.prod"));
        assert!(!valid_name(""));
        assert!(!valid_name("has space"));
        assert!(!valid_name("a/b"));
    }
}
