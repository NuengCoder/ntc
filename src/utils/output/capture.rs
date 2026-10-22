use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

// Stdout capture (used by the shell's pipeline/redirection support to keep a
// command's output in memory before handing it to the next stage or a file).

static CAPTURE_SEQ: AtomicU64 = AtomicU64::new(0);

/// Unique temp path per capture so sequential/nested captures can't collide.
fn capture_tmp_path() -> PathBuf {
    let n = CAPTURE_SEQ.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!("ntc_capture_{}_{}.txt", std::process::id(), n))
}

// ── platform redirect helpers ──────────────────────────────────────────────

/// RAII guard that restores stdout to its original handle/fd on drop.
/// Ensures stdout is always restored even if `f()` panics.
#[cfg(windows)]
struct StdoutGuard {
    original_fd: std::os::windows::io::RawHandle,
}

#[cfg(windows)]
impl StdoutGuard {
    fn new() -> Self {
        extern "system" {
            fn GetStdHandle(nStdHandle: u32) -> std::os::windows::io::RawHandle;
        }
        const STD_OUTPUT_HANDLE: u32 = 0xFFFFFFF5u32;
        let original_fd = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
        StdoutGuard { original_fd }
    }
}

#[cfg(windows)]
impl Drop for StdoutGuard {
    fn drop(&mut self) {
        extern "system" {
            fn SetStdHandle(nStdHandle: u32, hHandle: std::os::windows::io::RawHandle) -> i32;
        }
        const STD_OUTPUT_HANDLE: u32 = 0xFFFFFFF5u32;
        unsafe { SetStdHandle(STD_OUTPUT_HANDLE, self.original_fd) };
    }
}

/// RAII guard that restores stdout to its original fd on drop.
#[cfg(not(windows))]
struct StdoutGuard {
    saved_fd: i32,
}

#[cfg(not(windows))]
impl StdoutGuard {
    fn new() -> Self {
        let saved_fd = unsafe { libc::dup(1) };
        StdoutGuard { saved_fd }
    }
}

#[cfg(not(windows))]
impl Drop for StdoutGuard {
    fn drop(&mut self) {
        unsafe { libc::dup2(self.saved_fd, 1) };
        unsafe { libc::close(self.saved_fd) };
    }
}

/// Point the process stdout at `file` (fd 1 / STD_OUTPUT_HANDLE).
#[cfg(windows)]
fn redirect_stdout_to(file: &std::fs::File) {
    use std::os::windows::io::AsRawHandle;
    use std::os::windows::io::RawHandle;
    extern "system" {
        fn SetStdHandle(nStdHandle: u32, hHandle: RawHandle) -> i32;
    }
    const STD_OUTPUT_HANDLE: u32 = 0xFFFFFFF5u32;
    unsafe { SetStdHandle(STD_OUTPUT_HANDLE, file.as_raw_handle()) };
}

#[cfg(not(windows))]
fn redirect_stdout_to(file: &std::fs::File) {
    use std::os::unix::io::AsRawFd;
    unsafe { libc::dup2(file.as_raw_fd(), 1) };
}

/// Redirect stdout to a temporary file, capture output, then restore.
/// This captures ALL output — `println!`, `print!`, colored crate, child
/// processes that inherit the handle — as raw bytes (lossless).
/// Uses a RAII guard to ensure stdout is restored even if `f()` panics.
pub fn with_captured_output_bytes<F: FnOnce()>(f: F) -> Vec<u8> {
    let tmp = capture_tmp_path();
    let file = match std::fs::File::create(&tmp) {
        Ok(f) => f,
        Err(_) => {
            // Could not open a capture file — run without capturing.
            f();
            return Vec::new();
        }
    };

    let guard = StdoutGuard::new();
    redirect_stdout_to(&file);

    f();
    let _ = std::io::stdout().flush();

    // Close the capture file before reading it back (needed on Windows).
    drop(file);
    drop(guard);

    let bytes = std::fs::read(&tmp).unwrap_or_default();
    let _ = std::fs::remove_file(&tmp);
    bytes
}

/// Run `f` with command output captured to a UTF-8 string.
/// This captures ALL stdout output including `println!()`, the colored crate,
/// and the custom `print_info`/`print_error` helpers.
pub fn with_captured_output<F: FnOnce()>(f: F) -> String {
    let bytes = with_captured_output_bytes(f);
    String::from_utf8_lossy(&bytes).into_owned()
}
