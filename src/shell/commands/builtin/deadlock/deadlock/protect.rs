//! OS-level protection for `deadlock.vault`.
//!
//! The vault file is already restricted to the current user (`0600` on Unix,
//! an owner-only ACL on Windows). This module adds a second, OS-enforced
//! barrier against *deletion*:
//!
//! * **Windows** — an explicit deny-`DELETE` ACE on the file (exactly the
//!   `DELETE` bit, via the Win32 API) plus a deny-`FILE_DELETE_CHILD` ACE on
//!   the dedicated vault directory. The directory ACE is required because
//!   `DeleteFileW` can bypass a file-only deny through the parent. This blocks
//!   `del`, `rm`, Explorer delete, and rename for that account while still
//!   allowing normal reads.
//! * **macOS** — the user-immutable flag (`chflags uchg`).
//! * **Linux** — the immutable attribute (`chattr +i`), which a normal user
//!   cannot set. It is therefore only applied when ntc runs as root.
//!
//! These are best-effort *deterrents*, not enforcement: the file owner can
//! always remove their own ACE/flag. Deletion is also not the only risk —
//! the file can still be overwritten, which is why `deadlock> destroy` is the
//! supported way to remove it.

use std::path::Path;

/// Protect `path` against deletion. Best-effort: callers may ignore the error.
pub fn protect(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err("vault file does not exist".to_string());
    }
    imp::protect(path)
}

/// Remove the OS-level deletion protection from `path`.
pub fn unprotect(path: &Path) -> Result<(), String> {
    imp::unprotect(path)
}

/// True when the OS-level deletion protection is currently active.
pub fn is_protected(path: &Path) -> bool {
    path.exists() && imp::is_protected(path)
}

// ---------------------------------------------------------------------------
// Windows: deny-DELETE ACE for the current user
// ---------------------------------------------------------------------------

#[cfg(windows)]
mod imp {
    use std::path::Path;
    use std::process::{Command, Stdio};

    use windows_sys::Win32::Foundation::LocalFree;
    use windows_sys::Win32::Security::Authorization::{
        ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1,
    };
    use windows_sys::Win32::Security::{
        SetFileSecurityW, DACL_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR,
    };

    /// `DELETE` access right (`FILE_DELETE_CHILD` is a directory-only right).
    const DELETE: u32 = 0x0001_0000;

    fn wide(value: &str) -> Vec<u16> {
        value.encode_utf16().chain(std::iter::once(0)).collect()
    }

    /// `DOMAIN\user` plus the numeric SID, taken from `whoami /user`.
    fn user_identity() -> Option<(String, String)> {
        // CSV output is locale-independent: `"domain\user","S-1-5-21-..."`.
        if let Ok(out) = Command::new("whoami")
            .args(["/user", "/fo", "csv", "/nh"])
            .output()
        {
            if out.status.success() {
                let text = String::from_utf8_lossy(&out.stdout);
                for line in text.lines() {
                    if !line.contains("S-1-") {
                        continue;
                    }
                    let mut parts = line.split("\",\"");
                    if let (Some(name), Some(sid)) = (parts.next(), parts.next()) {
                        let name = name.trim_matches('"').trim();
                        let sid = sid.trim_matches('"').trim();
                        if !name.is_empty() && sid.starts_with("S-1-") {
                            return Some((name.to_string(), sid.to_string()));
                        }
                    }
                }
            }
        }
        // Fallback: the plain table ends with `DOMAIN\user S-1-5-...`.
        let out = Command::new("whoami").arg("/user").output().ok()?;
        if !out.status.success() {
            return None;
        }
        let text = String::from_utf8_lossy(&out.stdout);
        let line = text.lines().rev().find(|line| line.contains("S-1-"))?;
        let mut fields = line.split_whitespace();
        let name = fields.next()?.to_string();
        let sid = fields.next()?.to_string();
        (sid.starts_with("S-1-")).then_some((name, sid))
    }

    fn icacls(args: &[&str]) -> Result<(), String> {
        let out = Command::new("icacls")
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("cannot run icacls: {}", e))?;
        if out.status.success() {
            Ok(())
        } else {
            let msg = String::from_utf8_lossy(&out.stderr).trim().to_string();
            Err(if msg.is_empty() {
                "icacls failed".to_string()
            } else {
                msg
            })
        }
    }

    /// Replace the file's DACL with "deny `mask` for me, allow full control
    /// for me". `None` removes the deny ACE.
    ///
    /// `icacls` cannot express a deny mask without `SYNCHRONIZE`, and denying
    /// `SYNCHRONIZE` makes every normal open fail (even a plain read). The
    /// Win32 API lets us deny exactly the one bit we want.
    fn set_file_dacl(path: &Path, deny_mask: Option<u32>) -> Result<(), String> {
        let (_, sid) =
            user_identity().ok_or_else(|| "cannot determine the current user SID".to_string())?;
        let sddl = match deny_mask {
            Some(mask) => format!("D:PAI(D;;0x{:X};;;{})(A;;FA;;;{})", mask, sid, sid),
            None => format!("D:PAI(A;;FA;;;{})", sid),
        };

        let sddl_w = wide(&sddl);
        let mut descriptor: PSECURITY_DESCRIPTOR = std::ptr::null_mut();
        let ok = unsafe {
            ConvertStringSecurityDescriptorToSecurityDescriptorW(
                sddl_w.as_ptr(),
                SDDL_REVISION_1,
                &mut descriptor,
                std::ptr::null_mut(),
            )
        };
        if ok == 0 {
            return Err(format!(
                "cannot build the vault ACL: {}",
                std::io::Error::last_os_error()
            ));
        }

        let path_w = wide(&path.to_string_lossy());
        let ok =
            unsafe { SetFileSecurityW(path_w.as_ptr(), DACL_SECURITY_INFORMATION, descriptor) };
        let error = std::io::Error::last_os_error();
        unsafe { LocalFree(descriptor) };
        if ok != 0 {
            Ok(())
        } else {
            Err(format!("cannot set the vault ACL: {}", error))
        }
    }

    /// True when `dir` is the vault's own directory (named `deadlock`).
    fn is_dedicated_dir(dir: &Path) -> bool {
        dir.is_dir()
            && dir
                .file_name()
                .map(|name| name == "deadlock")
                .unwrap_or(false)
    }

    pub fn protect(path: &Path) -> Result<(), String> {
        // Deny exactly DELETE on the file. Do not use `icacls (D)`: it also
        // denies SYNCHRONIZE, which breaks every read.
        set_file_dacl(path, Some(DELETE))?;

        // `DeleteFileW` (used by `del` and `std::fs::remove_file`) can bypass
        // the file ACE through the parent's FILE_DELETE_CHILD right, so deny
        // that too. Only the dedicated `deadlock` directory: denying DC on a
        // shared directory (e.g. `~/.config/ntc`) would block deleting
        // unrelated files. `icacls` is fine here because denying SYNCHRONIZE
        // on a directory does not block opening its children.
        if let Some(parent) = path.parent() {
            if is_dedicated_dir(parent) {
                let (user, _) = user_identity()
                    .ok_or_else(|| "cannot determine the current user".to_string())?;
                let parent_s = parent.to_string_lossy().to_string();
                icacls(&[&parent_s, "/deny", &format!("{}:(DC)", user)])?;
            }
        }
        Ok(())
    }

    pub fn unprotect(path: &Path) -> Result<(), String> {
        let mut result = Ok(());
        if path.exists() {
            // Rebuild the DACL without the deny ACE; this also restores DELETE
            // to the allow ACE (which `icacls /deny` strips).
            if let Err(e) = set_file_dacl(path, None) {
                result = Err(e);
            }
        }
        if let Some(parent) = path.parent() {
            if is_dedicated_dir(parent) {
                if let Some((user, _)) = user_identity() {
                    let parent_s = parent.to_string_lossy().to_string();
                    if let Err(e) = icacls(&[&parent_s, "/remove:d", &user]) {
                        result = Err(e);
                    }
                }
            }
        }
        result
    }

    pub fn is_protected(path: &Path) -> bool {
        use std::os::windows::fs::OpenOptionsExt;
        // Opening the file with DELETE access does not delete it; it only
        // asks the OS whether we would be allowed to. ERROR_ACCESS_DENIED (5)
        // means the deny ACE is active.
        match std::fs::OpenOptions::new()
            .access_mode(DELETE)
            .share_mode(0x1 | 0x2 | 0x4) // READ | WRITE | DELETE
            .open(path)
        {
            Ok(_) => false,
            Err(e) => e.raw_os_error() == Some(5),
        }
    }
}

// ---------------------------------------------------------------------------
// macOS: user-immutable flag
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod imp {
    use std::path::Path;
    use std::process::{Command, Stdio};

    /// `UF_IMMUTABLE` from `<sys/stat.h>`.
    const UF_IMMUTABLE: u32 = 0x0000_0002;

    fn chflags(flag: &str, path: &Path) -> Result<(), String> {
        let out = Command::new("chflags")
            .arg(flag)
            .arg(path)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("cannot run chflags: {}", e))?;
        if out.status.success() {
            Ok(())
        } else {
            let msg = String::from_utf8_lossy(&out.stderr).trim().to_string();
            Err(if msg.is_empty() {
                "chflags failed".to_string()
            } else {
                msg
            })
        }
    }

    pub fn protect(path: &Path) -> Result<(), String> {
        chflags("uchg", path)
    }

    pub fn unprotect(path: &Path) -> Result<(), String> {
        chflags("nouchg", path)
    }

    pub fn is_protected(path: &Path) -> bool {
        use std::os::macos::fs::MetadataExt;
        std::fs::metadata(path)
            .map(|m| m.st_flags() & UF_IMMUTABLE != 0)
            .unwrap_or(false)
    }
}

// ---------------------------------------------------------------------------
// Linux: immutable attribute (root only)
// ---------------------------------------------------------------------------

#[cfg(target_os = "linux")]
mod imp {
    use std::path::Path;
    use std::process::{Command, Stdio};

    fn chattr(flag: &str, path: &Path) -> Result<(), String> {
        let out = Command::new("chattr")
            .arg(flag)
            .arg(path)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("cannot run chattr: {}", e))?;
        if out.status.success() {
            Ok(())
        } else {
            let msg = String::from_utf8_lossy(&out.stderr).trim().to_string();
            Err(if msg.is_empty() {
                "chattr failed".to_string()
            } else {
                msg
            })
        }
    }

    pub fn protect(path: &Path) -> Result<(), String> {
        // `chattr +i` needs CAP_LINUX_IMMUTABLE. Applying it as a normal user
        // is impossible; silently skip instead of failing every save.
        if unsafe { libc::geteuid() } != 0 {
            return Err("immutable flag requires root".to_string());
        }
        chattr("+i", path)
    }

    pub fn unprotect(path: &Path) -> Result<(), String> {
        if unsafe { libc::geteuid() } != 0 {
            return Ok(());
        }
        chattr("-i", path)
    }

    pub fn is_protected(path: &Path) -> bool {
        let out = match Command::new("lsattr").arg("-d").arg(path).output() {
            Ok(out) if out.status.success() => out,
            _ => return false,
        };
        String::from_utf8_lossy(&out.stdout)
            .split_whitespace()
            .next()
            .map(|flags| flags.contains('i'))
            .unwrap_or(false)
    }
}

// ---------------------------------------------------------------------------
// Other platforms
// ---------------------------------------------------------------------------

#[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
mod imp {
    use std::path::Path;

    pub fn protect(_path: &Path) -> Result<(), String> {
        Err("OS-level vault protection is not supported on this platform".to_string())
    }

    pub fn unprotect(_path: &Path) -> Result<(), String> {
        Ok(())
    }

    pub fn is_protected(_path: &Path) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unprotect_missing_file_is_ok() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("missing.vault");
        assert!(unprotect(&path).is_ok());
        assert!(!is_protected(&path));
    }

    #[cfg(any(windows, target_os = "macos"))]
    #[test]
    fn protect_blocks_deletion_until_unprotected() {
        let root = tempfile::tempdir().unwrap();
        // Use the real layout: the vault lives in its own `deadlock` directory,
        // which is what carries the Windows FILE_DELETE_CHILD deny.
        let dir = root.path().join("deadlock");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("deadlock.vault");
        std::fs::write(&path, b"vault").unwrap();

        // Guard against leaving a protected temp file behind on failure.
        struct Cleanup<'a>(&'a Path);
        impl<'a> Drop for Cleanup<'a> {
            fn drop(&mut self) {
                let _ = unprotect(self.0);
            }
        }
        let _cleanup = Cleanup(&path);

        protect(&path).expect("protect");
        assert!(is_protected(&path));
        // Protection must only block deletion, never reading: the app has to
        // be able to open its own vault.
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "vault");
        assert!(
            std::fs::remove_file(&path).is_err(),
            "protected vault was deletable"
        );

        unprotect(&path).expect("unprotect");
        assert!(!is_protected(&path));
        assert!(std::fs::remove_file(&path).is_ok());
    }

    #[cfg(any(windows, target_os = "macos"))]
    #[test]
    fn protect_survives_recursive_directory_removal() {
        let root = tempfile::tempdir().unwrap();
        let dir = root.path().join("deadlock");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("deadlock.vault");
        std::fs::write(&path, b"vault").unwrap();

        struct Cleanup<'a>(&'a Path);
        impl<'a> Drop for Cleanup<'a> {
            fn drop(&mut self) {
                let _ = unprotect(self.0);
            }
        }
        let _cleanup = Cleanup(&path);

        protect(&path).expect("protect");
        assert!(
            std::fs::remove_dir_all(&dir).is_err(),
            "recursive removal unexpectedly succeeded"
        );
        assert!(path.exists(), "vault disappeared during recursive removal");
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "vault");

        unprotect(&path).expect("unprotect");
        assert!(std::fs::remove_dir_all(&dir).is_ok());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn protect_requires_root_on_linux() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("deadlock.vault");
        std::fs::write(&path, b"vault").unwrap();
        if unsafe { libc::geteuid() } == 0 {
            protect(&path).expect("protect as root");
            assert!(is_protected(&path));
            unprotect(&path).expect("unprotect as root");
        } else {
            assert!(protect(&path).is_err());
            assert!(!is_protected(&path));
        }
    }
}
