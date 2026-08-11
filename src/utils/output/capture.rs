use std::io::Write;

// Stdout capture (used by the modern shell to keep command output inside the
// UI's body region instead of clobbering the input line)

/// RAII guard that restores stdout to its original handle on drop.
/// This ensures stdout is always restored even if `f()` panics.
#[cfg(windows)]
struct StdoutGuard {
    original_fd: std::os::windows::io::RawHandle,
    tmp_file: std::path::PathBuf,
}

#[cfg(windows)]
impl StdoutGuard {
    fn new(_file: &std::fs::File, tmp_file: std::path::PathBuf) -> Self {
        extern "system" {
            fn GetStdHandle(nStdHandle: u32) -> std::os::windows::io::RawHandle;
        }
        const STD_OUTPUT_HANDLE: u32 = 0xFFFFFFF5u32;
        let original_fd = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
        StdoutGuard {
            original_fd,
            tmp_file,
        }
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
        let _ = std::fs::remove_file(&self.tmp_file);
    }
}

/// RAII guard that restores stdout to its original fd on drop.
#[cfg(not(windows))]
struct StdoutGuard {
    saved_fd: i32,
    tmp_file: std::path::PathBuf,
}

#[cfg(not(windows))]
impl StdoutGuard {
    fn new(_file: &std::fs::File, tmp_file: std::path::PathBuf) -> Self {
        let saved_fd = unsafe { libc::dup(1) };
        StdoutGuard { saved_fd, tmp_file }
    }
}

#[cfg(not(windows))]
impl Drop for StdoutGuard {
    fn drop(&mut self) {
        unsafe { libc::dup2(self.saved_fd, 1) };
        unsafe { libc::close(self.saved_fd) };
        let _ = std::fs::remove_file(&self.tmp_file);
    }
}

/// Redirect stdout to a temporary file, capture output, then restore.
/// This captures ALL output — `println!`, `print!`, colored crate, everything.
/// Uses a RAII guard to ensure stdout is restored even if `f()` panics.
#[cfg(windows)]
fn with_stdout_captured<F: FnOnce()>(f: F) -> String {
    use std::os::windows::io::AsRawHandle;
    use std::os::windows::io::RawHandle;

    // Create a temp file path
    let tmp_dir = std::env::temp_dir();
    let tmp_file = tmp_dir.join(format!("ntc_capture_{}.txt", std::process::id()));
    let tmp_file2 = tmp_file.clone();

    // Open a file for writing (will be the new stdout)
    let file = std::fs::File::create(&tmp_file).expect("failed to create capture temp file");

    extern "system" {
        fn SetStdHandle(nStdHandle: u32, hHandle: RawHandle) -> i32;
    }
    const STD_OUTPUT_HANDLE: u32 = 0xFFFFFFF5u32;

    // Create guard that will restore stdout on drop (even during panic)
    let guard = StdoutGuard::new(&file, tmp_file2);

    // Redirect stdout to our file
    unsafe { SetStdHandle(STD_OUTPUT_HANDLE, file.as_raw_handle()) };

    f();

    // Flush stdout
    let _ = std::io::stdout().flush();
    drop(file);

    // Read the captured output
    std::fs::read_to_string(&guard.tmp_file).unwrap_or_default()
}

/// Redirect stdout to a temporary file, capture output, then restore.
/// This captures ALL output — `println!`, `print!`, colored crate, everything.
/// Uses a RAII guard to ensure stdout is restored even if `f()` panics.
#[cfg(not(windows))]
fn with_stdout_captured<F: FnOnce()>(f: F) -> String {
    use libc;
    use std::os::unix::io::AsRawFd;

    // Create a temp file path
    let tmp_dir = std::env::temp_dir();
    let tmp_file = tmp_dir.join(format!("ntc_capture_{}.txt", std::process::id()));
    let tmp_file2 = tmp_file.clone();

    // Open a file for writing
    let file = std::fs::File::create(&tmp_file).expect("failed to create capture temp file");

    let fd = file.as_raw_fd();

    // Create guard that will restore stdout on drop (even during panic)
    let _guard = StdoutGuard::new(&file, tmp_file2);

    // Redirect stdout (fd 1) to our file
    unsafe { libc::dup2(fd, 1) };

    f();

    // Flush stdout
    let _ = std::io::stdout().flush();
    drop(file);

    // Read the captured output
    std::fs::read_to_string(&_guard.tmp_file).unwrap_or_default()
}

/// Run `f` with command output captured to an in-memory buffer.
/// This captures ALL stdout output including `println!()`, the colored crate,
/// and the custom `print_info`/`print_error` helpers.
pub fn with_captured_output<F: FnOnce()>(f: F) -> String {
    with_stdout_captured(f)
}
