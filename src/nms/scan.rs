//! Media scanner for `nms scan`: walks the current directory for audio and
//! video files, honoring the same depth / care / ignore semantics as
//! `scan ext`.

use std::path::{Path, PathBuf};
use std::time::Duration;

use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

use crate::config::Config;
use crate::filetype::{make_ignored_dir_filter, matches_any_extension};
use crate::nms::all_extensions;

/// Options for [`scan_media`].
#[derive(Debug, Clone, Copy, Default)]
pub struct ScanOpts {
    /// Max walk depth (`None` = the configured default).
    pub max_depth: Option<usize>,
    /// Care mode: ignore the ignore-rules and include everything.
    pub care: bool,
    /// When set, only keep files whose decoded duration is longer than this.
    pub min_duration: Option<Duration>,
}

impl ScanOpts {
    fn depth(&self) -> usize {
        self.max_depth.unwrap_or_else(Config::global_get_max_depth)
    }
}

/// One media file found by [`scan_media`].
#[derive(Debug, Clone)]
pub struct MediaEntry {
    /// Path relative to the scan root, using `/` separators.
    pub rel: String,
    /// Size in bytes.
    pub size: u64,
    /// Decoded duration — only probed when [`ScanOpts::min_duration`] is set.
    pub duration: Option<Duration>,
}

/// Walk `root` and return every media file, sorted case-insensitively by path.
///
/// Paths use `/` separators. When [`ScanOpts::min_duration`] is set, each file's
/// duration is probed (in parallel) and only files *longer than* the threshold
/// are kept; files with an unreadable/unknown duration are excluded.
pub fn scan_media(root: &Path, opts: &ScanOpts) -> Vec<MediaEntry> {
    let exts = all_extensions();
    let max_depth = opts.depth();

    let walked: Vec<DirEntry> = if opts.care {
        WalkDir::new(root)
            .max_depth(max_depth)
            .into_iter()
            .filter_map(|e| e.ok())
            .collect()
    } else {
        let ignored = Config::get_ignored_dirs_combined();
        WalkDir::new(root)
            .max_depth(max_depth)
            .into_iter()
            .filter_entry(|e| (make_ignored_dir_filter(&ignored))(e))
            .filter_map(|e| e.ok())
            .collect()
    };

    let mut raw: Vec<(PathBuf, String, u64)> = Vec::new();
    for entry in walked {
        if !entry.file_type().is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if !matches_any_extension(&name, &exts) {
            continue;
        }
        let size = std::fs::metadata(entry.path())
            .map(|m| m.len())
            .unwrap_or(0);
        let rel = match entry.path().strip_prefix(root) {
            Ok(p) => p.to_string_lossy().replace('\\', "/"),
            Err(_) => entry.path().display().to_string(),
        };
        raw.push((entry.path().to_path_buf(), rel, size));
    }
    raw.sort_by_key(|(_, rel, _)| rel.to_lowercase());

    match opts.min_duration {
        Some(threshold) => {
            // Probe in parallel; `map` + `flatten` preserves the sorted order.
            let probed: Vec<Option<MediaEntry>> = raw
                .into_par_iter()
                .map(|(abs, rel, size)| {
                    let duration = crate::nms::player::probe_duration(&abs)?;
                    (duration > threshold).then_some(MediaEntry {
                        rel,
                        size,
                        duration: Some(duration),
                    })
                })
                .collect();
            probed.into_iter().flatten().collect()
        }
        None => raw
            .into_iter()
            .map(|(_, rel, size)| MediaEntry {
                rel,
                size,
                duration: None,
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Write a minimal 16-bit mono PCM WAV of `secs` seconds (silence).
    fn write_wav(path: &Path, secs: f64, sample_rate: u32) {
        let samples = (secs * sample_rate as f64) as u32;
        let data_len = samples * 2; // 16-bit mono
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&(36 + data_len).to_le_bytes());
        bytes.extend_from_slice(b"WAVE");
        bytes.extend_from_slice(b"fmt ");
        bytes.extend_from_slice(&16u32.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes()); // PCM
        bytes.extend_from_slice(&1u16.to_le_bytes()); // mono
        bytes.extend_from_slice(&sample_rate.to_le_bytes());
        bytes.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // byte rate
        bytes.extend_from_slice(&2u16.to_le_bytes()); // block align
        bytes.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
        bytes.extend_from_slice(b"data");
        bytes.extend_from_slice(&data_len.to_le_bytes());
        bytes.resize(bytes.len() + data_len as usize, 0);
        std::fs::write(path, bytes).unwrap();
    }

    #[test]
    fn scan_finds_audio_and_video_only() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("song.mp3"), "x").unwrap();
        std::fs::write(dir.path().join("clip.MP4"), "yy").unwrap();
        std::fs::write(dir.path().join("notes.txt"), "zzz").unwrap();
        let sub = dir.path().join("sub");
        std::fs::create_dir(&sub).unwrap();
        std::fs::write(sub.join("deep.flac"), "abcd").unwrap();

        let found = scan_media(dir.path(), &ScanOpts::default());
        let names: Vec<&str> = found.iter().map(|e| e.rel.as_str()).collect();
        assert_eq!(names, vec!["clip.MP4", "song.mp3", "sub/deep.flac"]);

        // Size is reported alongside the path.
        let mp3 = found.iter().find(|e| e.rel == "song.mp3").unwrap();
        assert_eq!(mp3.size, 1);
        assert!(mp3.duration.is_none());
    }

    #[test]
    fn scan_respects_depth_limit() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.wav"), "x").unwrap();
        let sub = dir.path().join("sub");
        std::fs::create_dir(&sub).unwrap();
        std::fs::write(sub.join("b.wav"), "x").unwrap();

        let shallow = scan_media(
            dir.path(),
            &ScanOpts {
                max_depth: Some(1),
                care: false,
                min_duration: None,
            },
        );
        assert_eq!(shallow.len(), 1);
        assert_eq!(shallow[0].rel, "a.wav");
    }

    #[test]
    fn scan_filters_by_min_duration() {
        let dir = tempfile::tempdir().unwrap();
        write_wav(&dir.path().join("long.wav"), 2.0, 8000);
        write_wav(&dir.path().join("short.wav"), 0.4, 8000);
        // A media extension with non-audio bytes: duration is unknown → excluded.
        std::fs::write(dir.path().join("fake.mp3"), "not really audio").unwrap();

        let found = scan_media(
            dir.path(),
            &ScanOpts {
                max_depth: None,
                care: false,
                min_duration: Some(Duration::from_secs(1)),
            },
        );
        let names: Vec<&str> = found.iter().map(|e| e.rel.as_str()).collect();
        assert_eq!(names, vec!["long.wav"]);
        assert!(found[0].duration.unwrap() >= Duration::from_secs(2));
    }
}
