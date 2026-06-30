use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, EnableMouseCapture};
use crossterm::execute;

use crate::editor::{Editor, Focus, Mode};
use crate::editor::types::RunMessage;

impl Editor {
    pub(crate) fn handle_run_event(&mut self, ev: Event, stdout: &mut std::io::Stdout) -> Result<()> {
        if let Event::Resize(w, h) = ev {
            self.term_w = w as usize;
            self.term_h = h as usize;
            self.mark_all_dirty();
            return Ok(());
        }

        if let Event::Mouse(m) = ev {
            let eo = self.editor_offset();
            let col = m.column as usize;
            let split = self.run_split_row();

            // Run/terminal panel border drag — delegate while dragging or on separator row
            if self.dragging_run_border || (m.row as usize) == split {
                self.handle_mouse(m);
                return Ok(());
            }

            if self.sidebar.open && col < eo {
                self.handle_mouse(m);
                return Ok(());
            }

            if m.row as usize >= split {
                match m.kind {
                    crossterm::event::MouseEventKind::ScrollUp
                        if self.sv.run > 0 => {
                            self.sv.run = self.sv.run.saturating_sub(3);
                            self.mark_all_dirty();
                        }
                    crossterm::event::MouseEventKind::ScrollDown => {
                        let panel_h = self.run_panel_height();
                        let max = self.run_lines.len().saturating_sub(panel_h);
                        self.sv.run = (self.sv.run + 3).min(max);
                        self.mark_all_dirty();
                    }
                    _ => {}
                }
                return Ok(());
            }

            self.handle_mouse(m);
            return Ok(());
        }

        if !self.run_cmd_buf.is_empty() {
            if let Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. }) = ev { match code {
                KeyCode::Esc => {
                    self.run_cmd_buf.clear();
                    self.mark_all_dirty();
                }
                KeyCode::Enter => {
                    let cmd = self.run_cmd_buf.clone();
                    self.run_cmd_buf.clear();
                    if let Some(rest) = cmd.strip_prefix(":run ") {
                        let rest = rest.trim();
                        if !rest.is_empty() {
                            self.start_run(rest);
                        }
                    } else if let Some(rest) = cmd.strip_prefix(":") {
                        let rest = rest.trim();
                        if !rest.is_empty() {
                            self.run_lines.clear();
                            self.sv.run = 0;
                            self.start_run(rest);
                        }
                    }
                    self.mark_all_dirty();
                }
                KeyCode::Backspace => {
                    self.run_cmd_buf.pop();
                    self.mark_all_dirty();
                }
                KeyCode::Char(c) => {
                    self.run_cmd_buf.push(c);
                    self.mark_all_dirty();
                }
                _ => {}
            } }
            return Ok(());
        }

        if let Event::Key(KeyEvent { code, modifiers, kind: KeyEventKind::Press, .. }) = &ev {
            let ctrl = modifiers.contains(KeyModifiers::CONTROL);
            if ctrl {
                match code {
                    KeyCode::Char('f') => {
                        self.focus = Focus::Editor;
                        self.mode = Mode::Search;
                        if self.has_selection() {
                            self.search.query = self.selected_text();
                        } else {
                            self.search.query.clear();
                        }
                        self.search.matches.clear();
                        self.search.match_idx = 0;
                        self.rebuild_search_matches();
                        if !self.search.matches.is_empty() {
                            self.jump_to_match(0);
                        }
                        self.mark_all_dirty();
                        return Ok(());
                    }
                    KeyCode::Char('n') => {
                        return Ok(());
                    }
                    KeyCode::Char('g') => {
                        self.focus = Focus::Editor;
                        self.prev_mode = Mode::Normal;
                        self.mode = Mode::Jump;
                        self.jump_buf.clear();
                        self.status_msg = None;
                        return Ok(());
                    }
                    KeyCode::Char('Q') => {
                        self.close_all_tabs();
                        self.focus = Focus::Editor;
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
        match ev {
            Event::Key(KeyEvent { code: KeyCode::Esc, kind: KeyEventKind::Press, .. }) => {
                if let Some(mut child) = self.run_child.take() {
                    let _ = child.kill();
                    let _ = child.wait();
                }
                self.last_run_cmd = self.run_executing.take();
                self.run_rx = None;
                self.run_lines.clear();
                self.sv.run = 0;
                self.run_panel_open = false;
                self.focus = Focus::Editor;
                let _ = execute!(stdout, EnableMouseCapture);
                self.mark_all_dirty();
            }
            Event::Key(KeyEvent { code: KeyCode::Char(':'), kind: KeyEventKind::Press, .. }) => {
                self.run_cmd_buf = ":".into();
                self.mark_all_dirty();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if let Some(mut child) = self.run_child.take() {
                    let _ = child.kill();
                    let _ = child.wait();
                    self.last_run_cmd = self.run_executing.take();
                    self.run_rx = None;
                    self.run_lines.push("^C".into());
                    let panel_h = self.run_panel_height();
                    self.sv.run = self.run_lines.len().saturating_sub(panel_h);
                    let _ = execute!(stdout, EnableMouseCapture);
                    self.mark_all_dirty();
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('o'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                ..
            }) => {
                self.focus = Focus::Editor;
                self.mode = Mode::Insert;
                self.mark_all_dirty();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                ..
            }) => {
                self.run_cmd_buf = ":".into();
                self.mark_all_dirty();
            }
            Event::Key(KeyEvent { code: KeyCode::Up, kind: KeyEventKind::Press, .. })
            | Event::Key(KeyEvent { code: KeyCode::Char('k'), kind: KeyEventKind::Press, .. })
                if self.sv.run > 0 => {
                    self.sv.run -= 1;
                    self.mark_all_dirty();
                }
            Event::Key(KeyEvent { code: KeyCode::Down, kind: KeyEventKind::Press, .. })
            | Event::Key(KeyEvent { code: KeyCode::Char('j'), kind: KeyEventKind::Press, .. }) => {
                let panel_h = self.run_panel_height();
                if self.sv.run + panel_h < self.run_lines.len() {
                    self.sv.run += 1;
                    self.mark_all_dirty();
                }
            }
            Event::Key(KeyEvent { code: KeyCode::PageUp, kind: KeyEventKind::Press, .. }) => {
                let panel_h = self.run_panel_height();
                self.sv.run = self.sv.run.saturating_sub(panel_h);
                self.mark_all_dirty();
            }
            Event::Key(KeyEvent { code: KeyCode::PageDown, kind: KeyEventKind::Press, .. }) => {
                let panel_h = self.run_panel_height();
                let max = self.run_lines.len().saturating_sub(panel_h);
                self.sv.run = (self.sv.run + panel_h).min(max);
                self.mark_all_dirty();
            }
            Event::Key(KeyEvent { code: KeyCode::Home, kind: KeyEventKind::Press, .. }) => {
                self.sv.run = 0;
                self.mark_all_dirty();
            }
            Event::Key(KeyEvent { code: KeyCode::End, kind: KeyEventKind::Press, .. }) => {
                let panel_h = self.run_panel_height();
                self.sv.run = self.run_lines.len().saturating_sub(panel_h);
                self.mark_all_dirty();
            }
            _ => {}
        }
        Ok(())
    }

    pub(crate) fn start_run(&mut self, cmd_line: &str) {
        if let Some(mut old_child) = self.run_child.take() {
            let _ = old_child.kill();
            let _ = old_child.wait();
        }
        self.run_rx = None;
        self.run_executing = None;

        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let cmd_line = cmd_line.to_string();

        let child_result = {
                #[cfg(windows)]
                let shell = "cmd";
                #[cfg(windows)]
                let flag = "/C";
                #[cfg(not(windows))]
                let shell = "sh";
                #[cfg(not(windows))]
                let flag = "-c";
                std::process::Command::new(shell)
                    .args([flag, &cmd_line])
                    .current_dir(&cwd)
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .spawn()
            };

        match child_result {
            Ok(mut child) => {
                let (tx, rx) = std::sync::mpsc::channel::<RunMessage>();

                let stdout_pipe = child.stdout.take();
                let stderr_pipe = child.stderr.take();

                std::thread::spawn(move || {
                    use std::io::{BufRead, BufReader};

                    let read_pipe = |reader: Option<Box<dyn std::io::Read + Send>>, tx: &std::sync::mpsc::Sender<RunMessage>| {
                        if let Some(pipe) = reader {
                            let buf = BufReader::new(pipe);
                            for l in buf.lines().map_while(Result::ok) {
                                if tx.send(RunMessage::Line(l)).is_err() {
                                    break;
                                }
                            }
                        }
                    };

                    read_pipe(stdout_pipe.map(|p| Box::new(p) as Box<dyn std::io::Read + Send>), &tx);
                    read_pipe(stderr_pipe.map(|p| Box::new(p) as Box<dyn std::io::Read + Send>), &tx);

                    let _ = tx.send(RunMessage::Done);
                });

                self.run_rx = Some(rx);
                self.run_child = Some(child);
                self.run_executing = Some(cmd_line);
                self.run_lines.clear();
                self.sv.run = 0;
                self.run_panel_open = true;
                self.terminal = None;
                self.focus = Focus::Editor;
                self.status_msg = None;
                self.mark_all_dirty();
            }
            Err(e) => {
                self.status_msg = Some(format!("run: {}", e));
            }
        }
    }

    pub(crate) fn run_panel_height(&self) -> usize {
        let split = self.run_split_row();
        self.term_h.saturating_sub(4).saturating_sub(split).max(3)
    }

    pub(crate) fn run_split_row(&self) -> usize {
        let max_split = self.term_h.saturating_sub(7).max(5);
        self.stored_run_split.clamp(5, max_split)
    }
}
