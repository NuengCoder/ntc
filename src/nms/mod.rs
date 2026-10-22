//! `nms` — NTC Music: a TUI sound player and media scanner.
//!
//! ```text
//! ntc [project]> nms play song.mp3        # fullscreen player (space = pause)
//! ntc [project]> nms play clip.mp4        # video files: audio track only
//! ntc [project]> nms scan -d 2 -s         # list audio + video files
//! ```
//!
//! Audio is decoded with `rodio`/symphonia and played through `cpal`. On a
//! machine without a usable sound driver (headless Linux servers, containers)
//! opening the output device fails gracefully with a clear message instead of
//! crashing.
//!
//! - [`player`] owns the playback engine and the level tap used by the UI.
//! - [`tui`] renders the waveform / level meter / countdown and handles keys.
//! - [`scan`] walks the current directory for playable media.

pub mod player;
pub mod scan;
pub mod tui;

use std::path::Path;
use std::time::Duration;

use player::PlayError;

/// Extensions played as sound.
pub const AUDIO_EXTS: &[&str] = &[
    "mp3", "wav", "wave", "flac", "m4a", "aac", "ogg", "oga", "opus", "wma", "aiff", "aif", "aifc",
    "mka", "weba", "mp4a",
];

/// Video extensions whose *audio track* is played (no video is rendered).
///
/// Note: `.ts` (MPEG transport stream) is intentionally *not* listed — it is
/// far more commonly TypeScript source, so treating it as media would confuse
/// both `nms scan` and `nms play`. `.m2ts` is unambiguous and stays.
pub const VIDEO_EXTS: &[&str] = &[
    "mp4", "m4v", "mkv", "webm", "mov", "avi", "mpg", "mpeg", "wmv", "flv", "3gp", "m2ts",
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

/// Best-effort check that the machine exposes an audio output device.
///
/// This never fails: it returns `false` when enumeration finds nothing. The
/// authoritative check happens when the player opens the device (see
/// [`play`]), which reports [`PlayError::NoDevice`].
pub fn audio_output_available() -> bool {
    player::probe_output_device()
}

/// Open `path`, play it through the [`tui`] player, and return when the user quits.
///
/// Video files are played as audio-only.
pub fn play(path: &Path) -> Result<(), PlayError> {
    let audio = player::AudioPlayer::new(path)?;
    tui::run(audio, path).map_err(|e| PlayError::Decode(e.to_string()))
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
}
