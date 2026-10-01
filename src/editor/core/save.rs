use anyhow::Result;
use crossterm::style::Color;
use std::path::Path;

use crate::editor::{prompt_yes_no, set_status_bar, Editor, EditorEncoding, Mode};
use crate::t;

/// Encode `text` as UTF-16 bytes (without BOM). encoding_rs exposes UTF-16LE/
/// BE as decode-only, so the encoding is done manually here.
fn utf16_bytes(text: &str, big_endian: bool) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len() * 2);
    for u in text.encode_utf16() {
        if big_endian {
            out.extend_from_slice(&u.to_be_bytes());
        } else {
            out.extend_from_slice(&u.to_le_bytes());
        }
    }
    out
}

/// Write `lines` to `path` encoded as `enc` (LF line endings, BOM prepended
/// when the encoding defines one), creating parent directories as needed.
/// Shared by the editor save flow and tab-close saves.
pub(crate) fn save_lines_to(
    path: &Path,
    lines: &[String],
    enc: EditorEncoding,
) -> Result<()> {
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p)?;
    }
    let mut text = String::new();
    for (i, l) in lines.iter().enumerate() {
        if i > 0 {
            text.push('\n');
        }
        text.push_str(l);
    }
    let mut bytes: Vec<u8> = match enc {
        EditorEncoding::Utf8 => text.into_bytes(),
        EditorEncoding::Utf16Le => utf16_bytes(&text, false),
        EditorEncoding::Utf16Be => utf16_bytes(&text, true),
        EditorEncoding::Windows1252 => encoding_rs::WINDOWS_1252.encode(&text).0.into_owned(),
    };
    let bom = enc.bom();
    if !bom.is_empty() {
        bytes.splice(0..0, bom.iter().copied());
    }
    std::fs::write(path, bytes)?;
    Ok(())
}

impl Editor {
    pub(crate) fn save(&mut self) -> Result<()> {
        save_lines_to(&self.path, &self.lines, self.encoding)?;
        self.modified = false;
        // A clean save means the on-disk file is newer than any crash snapshot.
        self.clear_recovery_snapshot();
        // Buffer agrees with disk again: re-baseline the external-change
        // fingerprint (also swallows our own save racing the watcher).
        self.snapshot_disk_state();
        Ok(())
    }

    fn close_all_tabs_inner(&mut self) {
        if self.tabs.is_empty() {
            self.open_scratch_tab();
            return;
        }
        self.save_active_tab();
        let modified_tabs: Vec<_> = self
            .tabs
            .iter()
            .filter(|t| t.modified && !self.is_scratch_path(&t.path))
            .map(|t| (t.path.clone(), t.lines.clone(), t.encoding))
            .collect();
        for (path, lines, encoding) in modified_tabs {
            self.path = path;
            self.lines = lines;
            self.encoding = encoding;
            self.modified = true;
            let _ = self.save();
        }
        self.tabs.clear();
        self.mode = Mode::Normal;
        self.clear_extra_cursors();
        self.open_scratch_tab();
        self.mark_all_dirty();
    }

    pub(crate) fn close_all_tabs(&mut self) {
        self.close_all_tabs_inner();
    }

    pub(crate) fn save_if_auto(&mut self) {
        if self.editor_cfg.auto_save && self.modified && !self.is_scratch_path(&self.path) {
            if let Err(e) = self.save() {
                self.status_msg = Some(format!("{} {}", t!("editor.save_auto_error"), e));
            }
        }
    }

    pub(crate) fn exit_flow(&mut self, stdout: &mut std::io::Stdout) -> Result<bool> {
        // Welcome/scratch buffers must never be written to disk or prompted on quit.
        if self.is_scratch_path(&self.path) {
            self.clear_recovery_snapshot();
            return Ok(true);
        }
        if self.editor_cfg.auto_save && self.modified {
            if let Err(e) = self.save() {
                let _ = set_status_bar(stdout, Color::Red, Color::White, &format!(" {}", e));
                return Ok(false);
            }
            self.clear_all_recovery_snapshots();
            return Ok(true);
        }
        if !self.modified {
            self.clear_all_recovery_snapshots();
            return Ok(true);
        }
        loop {
            match prompt_yes_no(
                stdout,
                Color::Rgb {
                    r: 160,
                    g: 100,
                    b: 20,
                },
                &t!("editor.save_prompt"),
            )? {
                Some(true) => {
                    if let Err(e) = self.save() {
                        let _ =
                            set_status_bar(stdout, Color::Red, Color::White, &format!(" {}", e));
                        continue;
                    }
                    let _ = set_status_bar(
                        stdout,
                        Color::Green,
                        Color::White,
                        &t!("editor.saved_full"),
                    );
                    self.clear_all_recovery_snapshots();
                    return Ok(true);
                }
                Some(false) => {
                    self.clear_all_recovery_snapshots();
                    return Ok(true);
                }
                None => return Ok(false),
            }
        }
    }
}
