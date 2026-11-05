//! ffmpeg-backed audio decoding for `nms`.
//!
//! `rodio` decodes through symphonia, and symphonia only carries a few demuxers:
//! it refuses an ordinary `.mp4` (it reports `UnrecognizedFormat`) and has no
//! demuxer at all for `.mkv`, `.webm`, `.avi`, `.mov` or `.wmv`. Rather than
//! advertise those extensions and then fail, `nms` asks ffmpeg for the audio
//! track and streams the decoded PCM straight out of the child process — no
//! temp file, no waiting for a full extraction, and every container ffmpeg
//! reads becomes playable.
//!
//! The binaries are looked up on `PATH` first, then in the `ntpkg` package
//! directory, so `ntpkg install ffmpeg` is enough on a machine with no
//! system-wide ffmpeg.

use std::io::{BufReader, Read};
use std::num::NonZero;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use rodio::{ChannelCount, Sample, SampleRate, Source};

/// Sample rate handed to the mixer; ffmpeg resamples to it.
const RATE: u32 = 48_000;
/// Stereo output: channel layout is left to ffmpeg.
const CHANNELS: u16 = 2;
/// Bytes per sample (`pcm_f32le`).
const SAMPLE_BYTES: usize = 4;
/// Frames pulled out of the pipe per read (~21 ms at 48 kHz).
const CHUNK_FRAMES: usize = 1024;

/// Why the ffmpeg path could not start.
#[derive(Debug)]
pub enum FfmpegError {
    /// Neither `PATH` nor the ntpkg package directory holds an ffmpeg binary.
    NotFound,
    /// ffmpeg was found but could not be spawned, or the file has no audio.
    Failed(String),
}

impl std::fmt::Display for FfmpegError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfmpegError::NotFound => write!(f, "ffmpeg not found"),
            FfmpegError::Failed(m) => write!(f, "{}", m),
        }
    }
}

impl std::error::Error for FfmpegError {}

/// Name of the executable to look for on this platform.
fn exe_name(stem: &str) -> String {
    #[cfg(windows)]
    {
        format!("{}.exe", stem)
    }
    #[cfg(not(windows))]
    {
        stem.to_string()
    }
}

/// Locate an ffmpeg-family binary: `PATH` first, then an ntpkg install.
///
/// ntpkg unpacks ffmpeg into `<packages>/ffmpeg/<version>/` (with a `bin`
/// subdir off Windows), so both layouts are checked, newest version first.
pub fn binary(stem: &str) -> Option<PathBuf> {
    if let Ok(path) = which::which(stem) {
        return Some(path);
    }
    let root = crate::ntpkg::types::package_dir().join("ffmpeg");
    let mut versions: Vec<PathBuf> = std::fs::read_dir(&root)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    // `latest` (what ntpkg installs for ffmpeg) sorts after the numbered
    // dirs, and reversing puts numbered versions newest-first too.
    versions.sort();
    versions.reverse();
    for dir in versions {
        for base in [dir.clone(), dir.join("bin")] {
            let candidate = base.join(exe_name(stem));
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

/// True when an ffmpeg binary is reachable, so the caller can promise the
/// fallback before starting playback.
pub fn is_available() -> bool {
    binary("ffmpeg").is_some()
}

/// What ffprobe learned about a media file.
#[derive(Debug, Clone, Copy)]
pub struct Probe {
    /// The ffprobe run itself succeeded (a missing ffprobe is `None` overall).
    pub probed: bool,
    /// At least one audio stream exists.
    pub has_audio: bool,
    /// Total duration, when the container exposes one.
    pub duration: Option<Duration>,
}

/// Read the audio-stream presence and duration of `path` with ffprobe.
///
/// Best-effort: `None` when ffprobe is unavailable or failed, so callers never
/// block playback on metadata they can live without.
pub fn probe(path: &Path) -> Option<Probe> {
    let ffprobe = binary("ffprobe")?;
    let out = Command::new(&ffprobe)
        .arg("-v")
        .arg("error")
        .arg("-show_entries")
        .arg("stream=codec_type:format=duration")
        .arg("-of")
        .arg("default=noprint_wrappers=1")
        .arg("--")
        .arg(path)
        .stdin(Stdio::null())
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    Some(parse_probe(&text))
}

/// Interpret `ffprobe -show_entries stream=codec_type:format=duration` output.
///
/// `noprint_wrappers=1` drops the `[STREAM]`/`[FORMAT]` headers, and the only
/// duration asked for is the format one, so a flat line scan is enough.
fn parse_probe(text: &str) -> Probe {
    let mut has_audio = false;
    let mut duration = None;
    for line in text.lines() {
        let line = line.trim();
        if line == "codec_type=audio" {
            has_audio = true;
        } else if let Some(rest) = line.strip_prefix("duration=") {
            // `N/A` (headerless streams) simply leaves the duration unset.
            if let Ok(secs) = rest.parse::<f64>() {
                duration = Duration::try_from_secs_f64(secs).ok();
            }
        }
    }
    Probe {
        probed: true,
        has_audio,
        duration,
    }
}

/// A `rodio` source fed by an ffmpeg child process.
///
/// ffmpeg demuxes and decodes the first audio stream into `pcm_f32le` on
/// stdout; the mixer thread pulls samples from here and blocks only while the
/// decoder falls behind, which it never does for audio-only work.
pub struct FfmpegPcm {
    child: Child,
    stdout: BufReader<std::process::ChildStdout>,
    /// Reused read buffer for one chunk of raw bytes.
    raw: Vec<u8>,
    /// Samples decoded from the last chunk, not yet handed to the mixer.
    samples: Vec<Sample>,
    /// Cursor into `samples`.
    cursor: usize,
    /// Set once the pipe reached EOF or an error; the source then ends.
    ended: bool,
    duration: Option<Duration>,
}

impl FfmpegPcm {
    /// Start ffmpeg on `path`, beginning at `start_from`.
    pub fn spawn(path: &Path, start_from: Duration, duration: Option<Duration>) -> Result<Self, FfmpegError> {
        let ffmpeg = binary("ffmpeg").ok_or(FfmpegError::NotFound)?;
        let mut cmd = Command::new(&ffmpeg);
        cmd.arg("-hide_banner")
            .arg("-nostdin")
            .arg("-loglevel")
            .arg("error");
        if !start_from.is_zero() {
            // Input seek: ffmpeg jumps to the offset before decoding, so
            // playback starts immediately instead of discarding samples.
            cmd.arg("-ss").arg(format!("{:.3}", start_from.as_secs_f64()));
        }
        cmd.arg("-i")
            .arg(path)
            .arg("-vn")
            .arg("-map")
            .arg("0:a:0")
            .arg("-c:a")
            .arg("pcm_f32le")
            .arg("-ac")
            .arg(CHANNELS.to_string())
            .arg("-ar")
            .arg(RATE.to_string())
            .arg("-f")
            .arg("f32le")
            .arg("-")
            .stdin(Stdio::null())
            // Diagnostics stay out of the terminal; `-loglevel error` keeps
            // stderr tiny anyway, and failures surface as a decode error.
            .stderr(Stdio::null())
            .stdout(Stdio::piped());
        let mut child = cmd
            .spawn()
            .map_err(|e| FfmpegError::Failed(format!("ffmpeg: {}", e)))?;
        let stdout = child
            .stdout
            .take()
            .ok_or(FfmpegError::Failed("ffmpeg: no stdout".into()))?;
        Ok(Self {
            child,
            stdout: BufReader::with_capacity(CHUNK_FRAMES as usize * CHANNELS as usize * SAMPLE_BYTES, stdout),
            raw: Vec::with_capacity(CHUNK_FRAMES as usize * CHANNELS as usize * SAMPLE_BYTES),
            samples: Vec::new(),
            cursor: 0,
            ended: false,
            duration,
        })
    }

    /// Fill `samples` from the pipe, marking the end on EOF or a read error.
    fn refill(&mut self) {
        let frame_bytes = CHANNELS as usize * SAMPLE_BYTES;
        let want = CHUNK_FRAMES * frame_bytes;
        self.raw.clear();
        self.raw.resize(want, 0);
        let mut filled = 0usize;
        while filled < want {
            match self.stdout.read(&mut self.raw[filled..]) {
                Ok(0) => {
                    self.ended = true;
                    break;
                }
                Ok(n) => filled += n,
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_) => {
                    self.ended = true;
                    break;
                }
            }
        }
        // Whole frames only: a partial frame would desynchronise the channels.
        let usable = filled - filled % frame_bytes;
        self.samples.clear();
        self.samples.extend(
            self.raw[..usable]
                .chunks_exact(SAMPLE_BYTES)
                .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]])),
        );
        self.cursor = 0;
    }
}

impl Iterator for FfmpegPcm {
    type Item = Sample;

    fn next(&mut self) -> Option<Sample> {
        if self.cursor >= self.samples.len() {
            if self.ended {
                return None;
            }
            self.refill();
            if self.cursor >= self.samples.len() {
                return None;
            }
        }
        let sample = self.samples[self.cursor];
        self.cursor += 1;
        Some(sample)
    }

    /// Unknown length: the mixer only needs to know nothing is buffered.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl Source for FfmpegPcm {
    /// Remaining samples of the current chunk. Frame alignment holds because
    /// every chunk is a whole number of frames and the mixer consumes spans of
    /// whole frames.
    fn current_span_len(&self) -> Option<usize> {
        Some(self.samples.len() - self.cursor)
    }

    fn channels(&self) -> ChannelCount {
        NonZero::new(CHANNELS).expect("stereo channel count is non-zero")
    }

    fn sample_rate(&self) -> SampleRate {
        NonZero::new(RATE).expect("sample rate is non-zero")
    }

    fn total_duration(&self) -> Option<Duration> {
        self.duration
    }
}

impl Drop for FfmpegPcm {
    fn drop(&mut self) {
        // Playback can end early (Ctrl+C, a stopped player): stop the decoder
        // instead of letting it run to the end of the file.
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_lookup_never_panics() {
        // Whatever the host has installed, the lookup is a plain Option.
        let _ = binary("ffmpeg");
        let _ = is_available();
    }

    #[test]
    fn probe_of_missing_file_is_none_or_unprobed() {
        match probe(Path::new("ntc-definitely-missing.mp4")) {
            None => {}
            Some(p) => assert!(!p.has_audio),
        }
    }

    #[test]
    fn missing_input_ends_the_stream_instead_of_hanging() {
        // ffmpeg starts fine on a bad path and then exits; the source must
        // report the end rather than block the mixer forever.
        match FfmpegPcm::spawn(Path::new("ntc-definitely-missing.mp4"), Duration::ZERO, None) {
            Err(FfmpegError::NotFound) | Err(FfmpegError::Failed(_)) => {}
            Ok(mut source) => {
                assert_eq!(source.next(), None);
                assert!(source.ended);
            }
        }
    }

    #[test]
    fn format_names_are_correct_for_platform() {
        let name = exe_name("ffmpeg");
        #[cfg(windows)]
        assert_eq!(name, "ffmpeg.exe");
        #[cfg(not(windows))]
        assert_eq!(name, "ffmpeg");
    }

    #[test]
    fn probe_output_reads_audio_track_and_duration() {
        // Real ffprobe output for a screen recording (h264 video + aac audio).
        let p = parse_probe("codec_type=video\ncodec_type=audio\nduration=3.800000\n");
        assert!(p.probed);
        assert!(p.has_audio);
        assert_eq!(p.duration, Some(Duration::from_millis(3800)));
    }

    #[test]
    fn probe_output_handles_silent_and_unmeasured_files() {
        // Video only, and a container that reports no total length.
        let p = parse_probe("codec_type=video\nduration=N/A\n");
        assert!(!p.has_audio);
        assert_eq!(p.duration, None);
        // Empty output (ffprobe printed nothing it recognized).
        let p = parse_probe("");
        assert!(!p.has_audio);
        assert_eq!(p.duration, None);
    }
}
