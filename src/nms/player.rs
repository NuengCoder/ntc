//! Playback engine for `nms`: a thin wrapper over `rodio` that exposes the
//! control surface the countdown needs (position, duration, finished) and
//! starts playback at an optional offset.
//!
//! symphonia (rodio's decoder) covers the common audio containers but has no
//! demuxer for video, so [`AudioPlayer::open`] retries through ffmpeg.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;

use rodio::cpal::traits::HostTrait;
use rodio::{Decoder, MixerDeviceSink, Player, Source};

use super::ffmpeg;

/// Why playback could not start.
#[derive(Debug)]
pub enum PlayError {
    /// No usable audio output device / sound driver on this machine.
    NoDevice,
    /// The file could not be opened or decoded.
    Decode(String),
    /// Needs ffmpeg (video container, or a codec symphonia cannot read) but no
    /// ffmpeg binary was found.
    NoFfmpeg,
    /// The media has no audio track to play.
    NoAudio,
}

impl std::fmt::Display for PlayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayError::NoDevice => write!(f, "no audio output device"),
            PlayError::Decode(m) => write!(f, "{}", m),
            PlayError::NoFfmpeg => write!(f, "ffmpeg is required to decode this file"),
            PlayError::NoAudio => write!(f, "no audio track"),
        }
    }
}

impl std::error::Error for PlayError {}

/// True when the machine has an enumerable default output device.
///
/// A device can still fail to *open* even when enumerated; the authoritative
/// check is [`AudioPlayer::new`].
pub fn probe_output_device() -> bool {
    rodio::cpal::default_host()
        .default_output_device()
        .is_some()
}

/// Best-effort media duration, read from the decoder headers without playing
/// or fully decoding the file.
///
/// Returns `None` for unreadable / unsupported files, or when the format does
/// not expose a total length (some headerless streams). No audio device is
/// required, so this works on headless machines. Containers symphonia refuses
/// (most video) fall back to ffprobe.
pub fn probe_duration(path: &Path) -> Option<Duration> {
    let file = File::open(path).ok()?;
    if let Ok(decoder) = Decoder::new(BufReader::new(file)) {
        return decoder.total_duration();
    }
    ffmpeg::probe(path).and_then(|p| p.duration)
}

/// Open the default output device, with rodio's drop notice silenced.
fn open_sink() -> Result<MixerDeviceSink, PlayError> {
    let mut sink = rodio::DeviceSinkBuilder::open_default_sink().map_err(|_| PlayError::NoDevice)?;
    sink.log_on_drop(false);
    Ok(sink)
}

/// An opened media track plus its device sink.
///
/// Keep this value alive for as long as sound should play: dropping it stops
/// playback and releases the device.
pub struct AudioPlayer {
    /// Kept alive so the mixer stays connected to the OS sink.
    _sink: MixerDeviceSink,
    player: Player,
    duration: Option<Duration>,
    /// Where the source started: an ffmpeg stream seeked with `-ss` begins at
    /// zero, so this is added back to report absolute track time.
    offset: Duration,
}

impl AudioPlayer {
    /// Open `path` on the default output device, starting at `start_from`.
    ///
    /// Tries symphonia first — no subprocess, the fast path for mp3/flac/wav —
    /// and falls back to ffmpeg when the container or codec is beyond it.
    pub fn open(path: &Path, start_from: Duration) -> Result<Self, PlayError> {
        match Self::new(path, start_from) {
            Err(PlayError::Decode(native)) => Self::with_ffmpeg(path, start_from, native),
            other => other,
        }
    }

    /// Decode `path` with symphonia and append it, starting at `start_from`.
    ///
    /// Returns [`PlayError::NoDevice`] when no sound driver is available and
    /// [`PlayError::Decode`] when the file cannot be read or decoded.
    pub fn new(path: &Path, start_from: Duration) -> Result<Self, PlayError> {
        let file = File::open(path).map_err(|e| PlayError::Decode(e.to_string()))?;
        let decoder =
            Decoder::new(BufReader::new(file)).map_err(|e| PlayError::Decode(e.to_string()))?;

        // Decode before touching the device: an unreadable file must not
        // occupy (and then release) the output for nothing.
        let duration = decoder.total_duration();
        let sink = open_sink()?;
        let player = Player::connect_new(sink.mixer());
        player.append(decoder);
        player.play();
        if !start_from.is_zero() {
            // Not every container supports seeking; fall back to the start.
            let _ = player.try_seek(start_from);
        }

        Ok(Self {
            _sink: sink,
            player,
            duration,
            offset: Duration::ZERO,
        })
    }

    /// Stream the audio track of `path` through ffmpeg, starting at `start_from`.
    ///
    /// `native` is the symphonia error that made the fallback necessary; it is
    /// only kept for the message when ffmpeg cannot decode either.
    fn with_ffmpeg(path: &Path, start_from: Duration, native: String) -> Result<Self, PlayError> {
        let probe = ffmpeg::probe(path);
        if probe.as_ref().is_some_and(|p| !p.has_audio) {
            return Err(PlayError::NoAudio);
        }
        let duration = probe.and_then(|p| p.duration);
        let source = ffmpeg::FfmpegPcm::spawn(path, start_from, duration).map_err(|e| match e {
            ffmpeg::FfmpegError::NotFound => PlayError::NoFfmpeg,
            ffmpeg::FfmpegError::Failed(m) => PlayError::Decode(format!("{} (ffmpeg: {})", native, m)),
        })?;

        let sink = open_sink()?;
        let player = Player::connect_new(sink.mixer());
        player.append(source);
        player.play();

        Ok(Self {
            _sink: sink,
            player,
            duration,
            offset: start_from,
        })
    }

    /// Total duration, when the decoder knows it.
    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }

    /// Current playback position, counted from the start of the track.
    pub fn position(&self) -> Duration {
        self.offset + self.player.get_pos()
    }

    /// True when the track has finished (no queued sources left).
    pub fn finished(&self) -> bool {
        self.player.empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_never_panics() {
        // Result is irrelevant on CI; it must simply not panic on any host.
        let _ = probe_output_device();
    }

    #[test]
    fn missing_file_is_a_decode_error() {
        // The decoder is opened before the device, so a missing file reports
        // the read failure rather than depending on the host's sound driver.
        match AudioPlayer::new(Path::new("ntc-definitely-missing.mp3"), Duration::ZERO) {
            Err(PlayError::Decode(_)) => {}
            Err(other) => panic!("missing file must report a decode error: {}", other),
            Ok(_) => panic!("missing file must not open"),
        }
    }
}
