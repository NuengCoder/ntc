// src/utils/test_support.rs
// Test-only helpers that serialize process-global state (CWD, env vars).
//
// Background: `cargo test` runs tests in parallel threads sharing one
// process. `std::env::set_current_dir` and env vars are process-global, so
// one test calling `Navigator::go_to()` (which chdirs) silently moves the
// CWD out from under filesystem-completion tests (`hinter`, `helpers`,
// `nssh`) that assert on repo fixtures like `src/` / `Cargo.toml`.
// On CI the interleaving hits reliably (12 failures); locally it usually
// passes by luck. Every test that reads process CWD or HOME must hold the
// lock below, and CWD-changing tests must restore the original dir.

use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};

static CWD_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn lock() -> &'static Mutex<()> {
    CWD_LOCK.get_or_init(|| Mutex::new(()))
}

/// Hold while reading process CWD/HOME so no other test can move it.
pub(crate) fn lock_globals() -> MutexGuard<'static, ()> {
    lock().lock().unwrap_or_else(|e| e.into_inner())
}

/// Guard that chdirs into `dir` on creation and restores the previous CWD
/// on drop. Holds the global lock for its whole lifetime so parallel tests
/// cannot observe the temporary CWD.
pub(crate) struct TempCwd {
    _lock: MutexGuard<'static, ()>,
    old: PathBuf,
    // Kept alive so the directory is not deleted while it is our CWD.
    // (Field drop runs after `Drop::drop`, which chdirs out first.)
    _tmp: Option<tempfile::TempDir>,
}

impl TempCwd {
    /// chdir into an existing directory.
    pub(crate) fn chdir(dir: &Path) -> Self {
        let lock = lock_globals();
        let old = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        std::env::set_current_dir(dir).expect("test chdir failed");
        Self {
            _lock: lock,
            old,
            _tmp: None,
        }
    }

    /// Create a tempdir populated with `files` (`(relpath, contents)`) and
    /// `dirs`, chdir into it, and return the guard.
    pub(crate) fn with_fixture(files: &[(&str, &str)], dirs: &[&str]) -> Self {
        let tmp = tempfile::tempdir().expect("tempdir failed");
        for d in dirs {
            std::fs::create_dir_all(tmp.path().join(d)).expect("mkdir fixture failed");
        }
        for (rel, contents) in files {
            let p = tmp.path().join(rel);
            if let Some(parent) = p.parent() {
                std::fs::create_dir_all(parent).expect("mkdir parent failed");
            }
            std::fs::write(&p, contents).expect("write fixture failed");
        }
        let lock = lock_globals();
        let old = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        std::env::set_current_dir(tmp.path()).expect("test chdir failed");
        Self {
            _lock: lock,
            old,
            _tmp: Some(tmp),
        }
    }

    /// Standard repo-like fixture used by hinter completion tests:
    /// `src/` dir, `Cargo.toml`, a `.toml` file (for `rmf *.` patterns),
    /// an image file plus a non-image file.
    pub(crate) fn with_repo_fixture() -> Self {
        Self::with_fixture(
            &[
                ("Cargo.toml", "[package]\nname = \"fixture\"\n"),
                ("app.toml", "x = 1\n"),
                ("notes.txt", "hello\n"),
                ("photo.png", "fakepng"),
                ("src/main.rs", "fn main() {}\n"),
            ],
            &["src", "docs"],
        )
    }
}

impl Drop for TempCwd {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.old);
        // `_tmp` (if any) is deleted here, after we chdir'd out.
    }
}

/// Isolate the `nssh` profile store (`~/.config/ntc/nssh.toml`) and the
/// `deadlock` vault by pointing XDG/APPDATA/NTC_CONFIG_HOME at a tempdir.
/// Holds the global lock (env vars are global).
pub(crate) struct IsolatedConfigHome {
    _lock: MutexGuard<'static, ()>,
    _tmp: tempfile::TempDir,
    old_xdg: Option<String>,
    old_appdata: Option<String>,
    old_ntc_config_home: Option<String>,
}

impl IsolatedConfigHome {
    pub(crate) fn new() -> Self {
        let lock = lock_globals();
        let tmp = tempfile::tempdir().expect("tempdir failed");
        let old_xdg = std::env::var("XDG_CONFIG_HOME").ok();
        let old_appdata = std::env::var("APPDATA").ok();
        let old_ntc_config_home = std::env::var("NTC_CONFIG_HOME").ok();
        // `dirs::config_dir` honors XDG_CONFIG_HOME on Linux, but on Windows
        // it calls the Known Folder API and ignores both XDG and APPDATA.
        // `NTC_CONFIG_HOME` is read by `utils::paths::config_home`, which the
        // on-disk stores use, so isolation also works on Windows.
        std::env::set_var("XDG_CONFIG_HOME", tmp.path());
        std::env::set_var("APPDATA", tmp.path());
        std::env::set_var("NTC_CONFIG_HOME", tmp.path());
        Self {
            _lock: lock,
            _tmp: tmp,
            old_xdg,
            old_appdata,
            old_ntc_config_home,
        }
    }

    /// Temporary config root used while this guard is alive.
    pub(crate) fn path(&self) -> &Path {
        self._tmp.path()
    }
}

impl Drop for IsolatedConfigHome {
    fn drop(&mut self) {
        match &self.old_xdg {
            Some(v) => std::env::set_var("XDG_CONFIG_HOME", v),
            None => std::env::remove_var("XDG_CONFIG_HOME"),
        }
        match &self.old_appdata {
            Some(v) => std::env::set_var("APPDATA", v),
            None => std::env::remove_var("APPDATA"),
        }
        match &self.old_ntc_config_home {
            Some(v) => std::env::set_var("NTC_CONFIG_HOME", v),
            None => std::env::remove_var("NTC_CONFIG_HOME"),
        }
    }
}
