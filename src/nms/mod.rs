//! `nms` — NTC Music: a terminal sound player and media scanner.
//!
//! ```text
//! ntc [project]> nms play song.mp3           # play audio, Ctrl+C stops
//! ntc [project]> nms play -sf 30 song.mp3    # start at 0:30
//! ntc [project]> nms play clip.mp4           # video files: audio track only
//! ntc [project]> nms scan -d 2 -s            # list audio + video files
//! ```
//!
//! Audio is decoded with `rodio`/symphonia and played through `cpal`. Video
//! containers (`.mp4`, `.mkv`, `.mov`, ...) are decoded by ffmpeg instead,
//! which `nms` streams in the background — see [`ffmpeg`]. On a machine
//! without a usable sound driver (headless Linux servers, containers) opening
//! the output device fails gracefully with a clear message instead of crashing.
//!
//! `play` is not a fullscreen UI: it prints the track and a live
//! position / remaining-time countdown, and Ctrl+C (or the end of the track)
//! returns to the shell.
//!
//! - [`player`] owns the playback engine.
//! - [`ffmpeg`] is the fallback decoder for video.
//! - [`scan`] walks the current directory for playable media.

pub mod ffmpeg;
pub mod player;
pub mod scan;

use std::io::{IsTerminal, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use player::PlayError;
use crate::t;

/// Countdown refresh interval while playing.
const TICK: Duration = Duration::from_millis(200);

/// Extensions played as sound.
///
/// Some of these (`.mka`, `.wma`, `.weba`) have no symphonia demuxer either and
/// reach the listener through the ffmpeg fallback.
pub const AUDIO_EXTS: &[&str] = &[
    "mp3", "wav", "wave", "flac", "m4a", "aac", "ogg", "oga", "opus", "wma", "aiff", "aif", "aifc",
    "mka", "weba", "mp4a",
];

/// Video extensions whose *audio track* is played (no video is rendered).
///
/// These are decoded by ffmpeg, so the playable set follows ffmpeg's demuxers
/// rather than symphonia's.
///
/// Note: `.ts` (MPEG transport stream) is intentionally *not* listed — it is
/// far more commonly TypeScript source, so treating it as media would confuse
/// both `nms scan` and `nms play`. `.m2ts` and `.mts` are unambiguous and stay.
pub const VIDEO_EXTS: &[&str] = &[
    "mp4", "m4v", "mkv", "webm", "mov", "avi", "mpg", "mpeg", "mpe", "wmv", "flv", "f4v", "3gp",
    "3g2", "m2ts", "mts", "ogv", "vob", "divx",
];

/// Lowercased extension of `path` (without the dot), if any.
pub fn extension_of(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
}

/// True if `path` has a known audio extension.
pub fn is_audio(path: &Path) -> bool {
    extension_of(path)
        .map(|e| AUDIO_EXTS.contains(&e.as_str()))
        .unwrap_or(false)
}

/// True if `path` has a known video extension (played as audio-only).
pub fn is_video(path: &Path) -> bool {
    extension_of(path)
        .map(|e| VIDEO_EXTS.contains(&e.as_str()))
        .unwrap_or(false)
}

/// True if `path` is either audio or video media handled by `nms`.
pub fn is_media(path: &Path) -> bool {
    is_audio(path) || is_video(path)
}

/// All playable extensions (audio first, then video), without dots.
pub fn all_extensions() -> Vec<String> {
    AUDIO_EXTS
        .iter()
        .chain(VIDEO_EXTS.iter())
        .map(|s| s.to_string())
        .collect()
}

/// Parse a human duration like `25s`, `1mn` (one minute) or `1h` (one hour).
///
/// Accepted units: `s`/`sec`/`secs` (seconds), `mn`/`m`/`min`/`mins` (minutes),
/// `h`/`hr`/`hrs` (hours). A bare number is read as seconds. Units are
/// case-insensitive and decimals are allowed (`1.5mn`). Returns `None` for
/// empty, malformed, negative or overflowing input.
pub fn parse_duration(input: &str) -> Option<Duration> {
    let s = input.trim().to_ascii_lowercase();
    if s.is_empty() {
        return None;
    }
    // Longer suffixes are matched first so `1min` is not read as `1mi` + `n`.
    let (num_str, mult) = if let Some(r) = s.strip_suffix("mins") {
        (r, 60.0)
    } else if let Some(r) = s.strip_suffix("min") {
        (r, 60.0)
    } else if let Some(r) = s.strip_suffix("mn") {
        (r, 60.0)
    } else if let Some(r) = s.strip_suffix("secs") {
        (r, 1.0)
    } else if let Some(r) = s.strip_suffix("sec") {
        (r, 1.0)
    } else if let Some(r) = s.strip_suffix("hrs") {
        (r, 3600.0)
    } else if let Some(r) = s.strip_suffix("hr") {
        (r, 3600.0)
    } else if let Some(r) = s.strip_suffix('h') {
        (r, 3600.0)
    } else if let Some(r) = s.strip_suffix('m') {
        (r, 60.0)
    } else if let Some(r) = s.strip_suffix('s') {
        (r, 1.0)
    } else {
        (s.as_str(), 1.0)
    };
    let value: f64 = num_str.trim().parse().ok()?;
    if !value.is_finite() || value < 0.0 {
        return None;
    }
    Duration::try_from_secs_f64(value * mult).ok()
}

/// Format a duration as `M:SS` (or `H:MM:SS` past an hour).
pub fn format_time(d: Duration) -> String {
    let total = d.as_secs();
    let (h, m, s) = (total / 3600, (total % 3600) / 60, total % 60);
    if h > 0 {
        format!("{}:{:02}:{:02}", h, m, s)
    } else {
        format!("{}:{:02}", m, s)
    }
}

/// Best-effort check that the machine exposes an audio output device.
///
/// This never fails: it returns `false` when enumeration finds nothing. The
/// authoritative check happens when the player opens the device (see
/// [`play`]), which reports [`PlayError::NoDevice`].
pub fn audio_output_available() -> bool {
    player::probe_output_device()
}

/// Registers a shutdown flag for as long as playback is running so the
/// shell's Ctrl+C handler can ask the countdown loop to stop.
struct StopGuard(Arc<AtomicBool>);

impl StopGuard {
    fn install(flag: Arc<AtomicBool>) -> Self {
        crate::shell::commands::init_ctrlc_handler();
        crate::shell::commands::register_shutdown_flag(flag.clone());
        Self(flag)
    }
}

impl Drop for StopGuard {
    fn drop(&mut self) {
        crate::shell::commands::unregister_shutdown_flag(&self.0);
    }
}

/// Open `path` and play it until the track ends or the user presses Ctrl+C.
///
/// Video files are played as audio-only. `start_from` seeks past the first
/// N seconds before the countdown starts.
pub fn play(path: &Path, start_from: Duration) -> Result<(), PlayError> {
    let player = player::AudioPlayer::open(path, start_from)?;

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string());
    let tag = if is_video(path) {
        "VIDEO · audio only"
    } else {
        "AUDIO"
    };
    println!("{}", t!("cmd.nms.playing", name, tag));

    let interactive = std::io::stdout().is_terminal();
    let stop = Arc::new(AtomicBool::new(false));
    let _guard = StopGuard::install(stop.clone());

    let mut next_tick = Instant::now();
    while !stop.load(Ordering::SeqCst) && !player.finished() {
        if interactive && Instant::now() >= next_tick {
            let pos = player.position();
            let dur = player.duration();
            let total = dur
                .map(format_time)
                .unwrap_or_else(|| "--:--".to_string());
            let remaining = dur
                .map(|d| format_time(d.saturating_sub(pos)))
                .unwrap_or_else(|| "--:--".to_string());
            print!(
                "\r  {} / {}  -{}   [{}]",
                format_time(pos),
                total,
                remaining,
                t!("cmd.nms.stop_hint")
            );
            let _ = std::io::stdout().flush();
            next_tick = Instant::now() + TICK;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    if interactive {
        println!();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_duration_units() {
        assert_eq!(parse_duration("25s"), Some(Duration::from_secs(25)));
        assert_eq!(parse_duration("90s"), Some(Duration::from_secs(90)));
        assert_eq!(parse_duration("1mn"), Some(Duration::from_secs(60)));
        assert_eq!(parse_duration("1min"), Some(Duration::from_secs(60)));
        assert_eq!(parse_duration("2m"), Some(Duration::from_secs(120)));
        assert_eq!(parse_duration("1h"), Some(Duration::from_secs(3600)));
        assert_eq!(parse_duration("1hr"), Some(Duration::from_secs(3600)));
        // Bare number means seconds.
        assert_eq!(parse_duration("30"), Some(Duration::from_secs(30)));
        // Decimals + case-insensitivity.
        assert_eq!(parse_duration("1.5mn"), Some(Duration::from_secs(90)));
        assert_eq!(parse_duration(" 25S "), Some(Duration::from_secs(25)));
    }

    #[test]
    fn parse_duration_rejects_junk() {
        assert_eq!(parse_duration(""), None);
        assert_eq!(parse_duration("abc"), None);
        assert_eq!(parse_duration("s"), None);
        assert_eq!(parse_duration("-5s"), None);
        assert_eq!(parse_duration("1.2.3h"), None);
    }

    #[test]
    fn ts_is_not_media_but_m2ts_is() {
        use std::path::PathBuf;
        assert!(!is_media(&PathBuf::from("script.ts")));
        assert!(is_media(&PathBuf::from("clip.m2ts")));
    }

    #[test]
    fn formats_times() {
        assert_eq!(format_time(Duration::from_secs(0)), "0:00");
        assert_eq!(format_time(Duration::from_secs(65)), "1:05");
        assert_eq!(format_time(Duration::from_secs(3661)), "1:01:01");
    }
}
