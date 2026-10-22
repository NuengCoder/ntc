//! Playback engine for `nms`: a thin wrapper over `rodio` that exposes the
//! control surface the TUI needs (pause, seek, volume, position) plus a
//! lock-free "level tap" so the UI can draw a real waveform without sharing
//! the audio source across threads.

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use rodio::cpal::traits::HostTrait;
use rodio::source::SeekError;
use rodio::{ChannelCount, Decoder, MixerDeviceSink, Player, SampleRate, Source};

/// Why playback could not start.
#[derive(Debug)]
pub enum PlayError {
    /// No usable audio output device / sound driver on this machine.
    NoDevice,
    /// The file could not be opened or decoded.
    Decode(String),
}

impl std::fmt::Display for PlayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayError::NoDevice => write!(f, "no audio output device"),
            PlayError::Decode(m) => write!(f, "{}", m),
        }
    }
}

impl std::error::Error for PlayError {}

/// Cross-thread level information written by the audio thread and read by the UI.
///
/// Values are the bit patterns of `f32` amplitudes in `0.0..=1.0`.
#[derive(Default)]
pub struct Levels {
    /// Peak amplitude observed since the last [`Levels::take_peak`].
    peak: AtomicU32,
    /// Most recent instantaneous amplitude (for the VU-style meter).
    level: AtomicU32,
}

impl Levels {
    #[inline]
    fn push(&self, sample: f32) {
        let a = sample.abs().min(1.0);
        self.level.store(a.to_bits(), Ordering::Relaxed);
        let current = f32::from_bits(self.peak.load(Ordering::Relaxed));
        if a > current {
            self.peak.store(a.to_bits(), Ordering::Relaxed);
        }
    }

    /// Read and reset the accumulated peak (called once per UI frame).
    pub fn take_peak(&self) -> f32 {
        f32::from_bits(self.peak.swap(0, Ordering::Relaxed))
    }

    /// Most recent instantaneous amplitude.
    pub fn level(&self) -> f32 {
        f32::from_bits(self.level.load(Ordering::Relaxed))
    }
}

/// A `Source` wrapper that records amplitudes into [`Levels`] as they are
/// pulled by the audio thread, then forwards them untouched.
struct TapSource<S: Source> {
    inner: S,
    levels: Arc<Levels>,
}

impl<S: Source> Iterator for TapSource<S> {
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next();
        if let Some(sample) = item.as_ref() {
            self.levels.push(*sample);
        }
        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<S: Source> Source for TapSource<S> {
    fn current_span_len(&self) -> Option<usize> {
        self.inner.current_span_len()
    }

    fn channels(&self) -> ChannelCount {
        self.inner.channels()
    }

    fn sample_rate(&self) -> SampleRate {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
    }

    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        self.inner.try_seek(pos)
    }
}

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
/// required, so this works on headless machines.
pub fn probe_duration(path: &Path) -> Option<Duration> {
    let file = File::open(path).ok()?;
    let decoder = Decoder::new(BufReader::new(file)).ok()?;
    decoder.total_duration()
}

/// An opened media track plus its device sink.
///
/// Keep this value alive for as long as sound should play: dropping it stops
/// playback and releases the device.
pub struct AudioPlayer {
    /// Kept alive so the mixer stays connected to the OS sink.
    _sink: MixerDeviceSink,
    player: Player,
    levels: Arc<Levels>,
    duration: Option<Duration>,
    volume: f32,
    path: PathBuf,
}

impl AudioPlayer {
    /// Open the default output device and decode `path`, starting playback.
    ///
    /// Returns [`PlayError::NoDevice`] when no sound driver is available and
    /// [`PlayError::Decode`] when the file cannot be read or decoded.
    pub fn new(path: &Path) -> Result<Self, PlayError> {
        let mut sink =
            rodio::DeviceSinkBuilder::open_default_sink().map_err(|_| PlayError::NoDevice)?;
        // Silence rodio's "Dropping DeviceSink" message on exit.
        sink.log_on_drop(false);
        let player = Player::connect_new(sink.mixer());

        let file = File::open(path).map_err(|e| PlayError::Decode(e.to_string()))?;
        let decoder =
            Decoder::new(BufReader::new(file)).map_err(|e| PlayError::Decode(e.to_string()))?;

        let duration = decoder.total_duration();
        let levels = Arc::new(Levels::default());
        let tap = TapSource {
            inner: decoder,
            levels: levels.clone(),
        };

        player.set_volume(1.0);
        player.append(tap);
        player.play();

        Ok(Self {
            _sink: sink,
            player,
            levels,
            duration,
            volume: 1.0,
            path: path.to_path_buf(),
        })
    }

    /// Re-open the current file and restart playback from `0s`.
    ///
    /// The device sink is reused (no device re-enumeration) but a fresh
    /// [`Player`] is connected so the position resets to zero and the previous
    /// source is dropped. The current volume is preserved. Used by the TUI's
    /// `a` (replay) key and after a finished track freezes.
    pub fn restart(&mut self) -> Result<(), PlayError> {
        let file = File::open(&self.path).map_err(|e| PlayError::Decode(e.to_string()))?;
        let decoder =
            Decoder::new(BufReader::new(file)).map_err(|e| PlayError::Decode(e.to_string()))?;
        let duration = decoder.total_duration();
        let levels = Arc::new(Levels::default());
        let tap = TapSource {
            inner: decoder,
            levels: levels.clone(),
        };

        let player = Player::connect_new(self._sink.mixer());
        player.set_volume(self.volume);
        player.append(tap);
        player.play();

        // Assigning drops the previous player, stopping the old source.
        self.player = player;
        self.levels = levels;
        self.duration = duration;
        Ok(())
    }

    /// Path being played.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Total duration, when the decoder knows it.
    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }

    /// Current playback position.
    pub fn position(&self) -> Duration {
        self.player.get_pos()
    }

    /// True while playback is paused.
    pub fn is_paused(&self) -> bool {
        self.player.is_paused()
    }

    /// True when the track has finished (no queued sources left).
    pub fn finished(&self) -> bool {
        self.player.empty()
    }

    /// Pause if playing, resume if paused.
    pub fn toggle_pause(&self) {
        if self.player.is_paused() {
            self.player.play();
        } else {
            self.player.pause();
        }
    }

    /// Seek by `secs` (negative rewinds). Clamped to the track bounds.
    pub fn seek_relative(&self, secs: f32) {
        let pos = self.position();
        let target = if secs >= 0.0 {
            pos + Duration::from_secs_f32(secs)
        } else {
            pos.saturating_sub(Duration::from_secs_f32(-secs))
        };
        let _ = self.player.try_seek(target);
    }

    /// Adjust volume by `delta` (clamped to `0.0..=2.0`).
    pub fn adjust_volume(&mut self, delta: f32) {
        self.volume = (self.volume + delta).clamp(0.0, 2.0);
        self.player.set_volume(self.volume);
    }

    /// Current volume multiplier (`1.0` = normal).
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// Read and reset the peak amplitude since the previous call.
    pub fn take_peak(&self) -> f32 {
        self.levels.take_peak()
    }

    /// Most recent instantaneous amplitude.
    pub fn level(&self) -> f32 {
        self.levels.level()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_track_peak_and_reset() {
        let l = Levels::default();
        l.push(0.5);
        l.push(-0.75);
        l.push(0.2);
        assert!((l.take_peak() - 0.75).abs() < 1e-6);
        // Peak is drained; only fresh samples count.
        assert_eq!(l.take_peak(), 0.0);
        // Level keeps the most recent (absolute) sample.
        assert!((l.level() - 0.2).abs() < 1e-6);
    }

    #[test]
    fn levels_clamp_out_of_range() {
        let l = Levels::default();
        l.push(5.0);
        assert!((l.take_peak() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn probe_never_panics() {
        // Result is irrelevant on CI; it must simply not panic on any host.
        let _ = probe_output_device();
    }
}
