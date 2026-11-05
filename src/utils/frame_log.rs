// src/utils/frame_log.rs
// Opt-in frame paint telemetry. Setting `NTC_FRAME_LOG=1` appends one line
// per painted frame to `ntc-frame.log` in the temp dir (bytes queued and
// time spent composing the frame). Env-gated with a cached lookup, so normal
// runs pay nothing per frame.

use std::io::Write;
use std::time::Duration;

static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();

pub(crate) fn log(tag: &str, bytes: usize, elapsed: Duration) {
    if !*ENABLED.get_or_init(|| std::env::var_os("NTC_FRAME_LOG").is_some()) {
        return;
    }
    let path = std::env::temp_dir().join("ntc-frame.log");
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = writeln!(f, "{} {}B {}us", tag, bytes, elapsed.as_micros());
    }
}
