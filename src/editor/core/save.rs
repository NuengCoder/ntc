use anyhow::Result;
use crossterm::style::Color;

use crate::editor::{set_status_bar, prompt_yes_no, Editor, Mode};

impl Editor {
    pub(crate) fn save(&mut self) -> Result<()> {
        if let Some(p) = self.path.parent() {
            std::fs::create_dir_all(p)?;
        }
        let mut out = String::new();
        for (i, l) in self.lines.iter().enumerate() {
            if i > 0 {
                out.push('\n');
            }
            out.push_str(l);
        }
        std::fs::write(&self.path, out)?;
        self.modified = false;
        Ok(())
    }

    fn close_all_tabs_inner(&mut self) {
        if self.tabs.is_empty() {
            self.open_scratch_tab();
            return;
        }
        self.save_active_tab();
        let modified_tabs: Vec<_> = self.tabs.iter()
            .filter(|t| t.modified && !self.is_scratch_path(&t.path))
            .map(|t| (t.path.clone(), t.lines.clone()))
            .collect();
        for (path, lines) in modified_tabs {
            self.path = path;
            self.lines = lines;
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
                self.status_msg = Some(format!("Auto-save error: {}", e));
            }
        }
    }

    pub(crate) fn exit_flow(&mut self, stdout: &mut std::io::Stdout) -> Result<bool> {
        if self.editor_cfg.auto_save && self.modified {
            if let Err(e) = self.save() {
                let _ = set_status_bar(stdout, Color::Red, Color::White, &format!(" {}", e));
                return Ok(false);
            }
            self.clear_recovery_snapshot();
            return Ok(true);
        }
        if !self.modified {
            self.clear_recovery_snapshot();
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
                "Save file [Y|N]?",
            )? {
                Some(true) => {
                    if let Err(e) = self.save() {
                        let _ =
                            set_status_bar(stdout, Color::Red, Color::White, &format!(" {}", e));
                        continue;
                    }
                    let _ = set_status_bar(stdout, Color::Green, Color::White, " Saved \u{2713}");
                    self.clear_recovery_snapshot();
                    return Ok(true);
                }
                Some(false) => {
                    self.clear_recovery_snapshot();
                    return Ok(true);
                }
                None => return Ok(false),
            }
        }
    }
}
