//! OS credential-store integration for the deadlock master password.
//!
//! Registering the master with the platform's credential store lets `nssh`
//! (and `smtp --deadlock`) unlock the vault silently in a fresh process,
//! without opening the `deadlock` shell or asking for the master password.
//!
//! * **Windows** — Credential Manager (`CredWriteW` / `CredReadW`).
//! * **macOS** — the `security` command-line tool (Keychain).
//! * **Linux** — `secret-tool` (Secret Service), when installed.
//!
//! Everything is best-effort: when the platform store is unavailable the
//! callers fall back to prompting for the master password.

/// Credential Manager target name / Keychain service name.
pub const TARGET: &str = "ntc/deadlock/master";
/// Account name stored alongside [`TARGET`].
#[allow(dead_code)] // only referenced by the platform backends (not under `cfg(test)`)
const ACCOUNT: &str = "ntc";

/// Store (or overwrite) the master password in the OS credential store.
pub fn store_master(master: &str) -> Result<(), String> {
    #[cfg(test)]
    {
        *test_store::TEST_MASTER.lock().unwrap() = Some(master.to_string());
        Ok(())
    }
    #[cfg(not(test))]
    {
        imp::store(master)
    }
}

/// Read the master password from the OS credential store, if present.
pub fn load_master() -> Option<String> {
    #[cfg(test)]
    {
        test_store::TEST_MASTER.lock().unwrap().clone()
    }
    #[cfg(not(test))]
    {
        imp::load()
    }
}

/// Remove the master password from the OS credential store.
///
/// Removing a credential that is not there is not an error.
pub fn forget_master() -> Result<(), String> {
    #[cfg(test)]
    {
        *test_store::TEST_MASTER.lock().unwrap() = None;
        Ok(())
    }
    #[cfg(not(test))]
    {
        imp::forget()
    }
}

#[cfg(test)]
mod test_store {
    use std::sync::{LazyLock, Mutex};

    pub static TEST_MASTER: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
}

// ---------------------------------------------------------------------------
// Windows: Credential Manager
// ---------------------------------------------------------------------------

#[cfg(all(windows, not(test)))]
mod imp {
    use super::{ACCOUNT, TARGET};
    use std::ffi::c_void;
    use std::ptr;
    use windows_sys::Win32::Foundation::ERROR_NOT_FOUND;
    use windows_sys::Win32::Security::Credentials::{
        CredDeleteW, CredFree, CredReadW, CredWriteW, CREDENTIALW, CRED_PERSIST_LOCAL_MACHINE,
        CRED_TYPE_GENERIC,
    };

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    pub fn store(master: &str) -> Result<(), String> {
        let mut target = wide(TARGET);
        let mut account = wide(ACCOUNT);
        let mut blob: Vec<u8> = master
            .encode_utf16()
            .flat_map(|unit| unit.to_le_bytes())
            .collect();
        let cred = CREDENTIALW {
            Flags: 0,
            Type: CRED_TYPE_GENERIC,
            TargetName: target.as_mut_ptr(),
            Comment: ptr::null_mut(),
            LastWritten: unsafe { std::mem::zeroed() },
            CredentialBlobSize: blob.len() as u32,
            CredentialBlob: blob.as_mut_ptr(),
            Persist: CRED_PERSIST_LOCAL_MACHINE,
            AttributeCount: 0,
            Attributes: ptr::null_mut(),
            TargetAlias: ptr::null_mut(),
            UserName: account.as_mut_ptr(),
        };
        let ok = unsafe { CredWriteW(&cred, 0) };
        // Do not leave the plaintext master in the local buffer.
        blob.fill(0);
        if ok != 0 {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error().to_string())
        }
    }

    pub fn load() -> Option<String> {
        let target = wide(TARGET);
        let mut cred_ptr: *mut CREDENTIALW = ptr::null_mut();
        let ok = unsafe { CredReadW(target.as_ptr(), CRED_TYPE_GENERIC, 0, &mut cred_ptr) };
        if ok == 0 || cred_ptr.is_null() {
            return None;
        }
        let cred = unsafe { &*cred_ptr };
        let bytes = unsafe {
            std::slice::from_raw_parts(cred.CredentialBlob, cred.CredentialBlobSize as usize)
        };
        let units: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect();
        let master = String::from_utf16(&units).ok();
        unsafe { CredFree(cred_ptr as *const c_void) };
        master
    }

    pub fn forget() -> Result<(), String> {
        let target = wide(TARGET);
        let ok = unsafe { CredDeleteW(target.as_ptr(), CRED_TYPE_GENERIC, 0) };
        if ok != 0 {
            return Ok(());
        }
        let err = std::io::Error::last_os_error();
        if err.raw_os_error() == Some(ERROR_NOT_FOUND as i32) {
            Ok(())
        } else {
            Err(err.to_string())
        }
    }
}

// ---------------------------------------------------------------------------
// macOS: Keychain via the `security` CLI
// ---------------------------------------------------------------------------

#[cfg(all(target_os = "macos", not(test)))]
mod imp {
    use super::{ACCOUNT, TARGET};
    use std::process::{Command, Stdio};

    fn run(args: &[&str]) -> Result<std::process::Output, String> {
        Command::new("security")
            .args(args)
            .stdin(Stdio::null())
            .output()
            .map_err(|e| format!("cannot run security: {}", e))
    }

    pub fn store(master: &str) -> Result<(), String> {
        let out = run(&[
            "add-generic-password",
            "-U",
            "-a",
            ACCOUNT,
            "-s",
            TARGET,
            "-w",
            master,
        ])?;
        if out.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
        }
    }

    pub fn load() -> Option<String> {
        let out = run(&["find-generic-password", "-a", ACCOUNT, "-s", TARGET, "-w"]).ok()?;
        if !out.status.success() {
            return None;
        }
        let master = String::from_utf8(out.stdout).ok()?;
        let master = master.trim_end_matches(['\r', '\n']).to_string();
        (!master.is_empty()).then_some(master)
    }

    pub fn forget() -> Result<(), String> {
        let out = run(&["delete-generic-password", "-a", ACCOUNT, "-s", TARGET])?;
        // errSecItemNotFound (44) simply means there was nothing to remove.
        if out.status.success() || out.status.code() == Some(44) {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
        }
    }
}

// ---------------------------------------------------------------------------
// Linux: Secret Service via `secret-tool`
// ---------------------------------------------------------------------------

#[cfg(all(target_os = "linux", not(test)))]
mod imp {
    use std::io::Write;
    use std::process::{Command, Stdio};

    const ATTRS: [&str; 4] = ["service", "ntc", "account", "deadlock"];

    fn command() -> Command {
        Command::new("secret-tool")
    }

    pub fn store(master: &str) -> Result<(), String> {
        let mut child = command()
            .arg("store")
            .arg("--label=ntc deadlock master")
            .args(ATTRS)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("secret-tool is not available: {}", e))?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(master.as_bytes())
                .map_err(|e| format!("cannot write to secret-tool: {}", e))?;
        }
        let out = child
            .wait_with_output()
            .map_err(|e| format!("secret-tool failed: {}", e))?;
        if out.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
        }
    }

    pub fn load() -> Option<String> {
        let out = command().arg("lookup").args(ATTRS).output().ok()?;
        if !out.status.success() {
            return None;
        }
        let master = String::from_utf8(out.stdout).ok()?;
        let master = master.trim_end_matches(['\r', '\n']).to_string();
        (!master.is_empty()).then_some(master)
    }

    pub fn forget() -> Result<(), String> {
        let out = command()
            .arg("clear")
            .args(ATTRS)
            .output()
            .map_err(|e| format!("secret-tool is not available: {}", e))?;
        if out.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
        }
    }
}

// ---------------------------------------------------------------------------
// Other platforms
// ---------------------------------------------------------------------------

#[cfg(all(
    not(windows),
    not(target_os = "macos"),
    not(target_os = "linux"),
    not(test)
))]
mod imp {
    pub fn store(_master: &str) -> Result<(), String> {
        Err("OS credential store is not supported on this platform".to_string())
    }

    pub fn load() -> Option<String> {
        None
    }

    pub fn forget() -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_roundtrip() {
        // Tests must never touch the real OS credential store.
        forget_master().unwrap();
        assert!(load_master().is_none());
        store_master("s3cr3t-master").unwrap();
        assert_eq!(load_master().as_deref(), Some("s3cr3t-master"));
        forget_master().unwrap();
        assert!(load_master().is_none());
    }

    #[test]
    fn target_is_stable() {
        assert_eq!(TARGET, "ntc/deadlock/master");
    }
}
