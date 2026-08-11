use std::path::Path;

use anyhow::Result;

use crate::editor::Editor;
use crate::session::EditorSession;
use crate::t;

pub fn edit_file(path: &Path) -> Result<bool> {
    edit_file_with_session(path, None).map(|(r, _)| r)
}

pub fn edit_file_with_session(
    path: &Path,
    restored: Option<EditorSession>,
) -> Result<(bool, Option<EditorSession>)> {
    if let Some(data) = crate::editor::recovery::RecoveryData::check_recovery(path) {
        eprintln!("\n{} {}", t!("editor.tmpl_recovery_found"), path.display());
        eprintln!("{} {}", t!("editor.tmpl_last_saved"), data.timestamp);
        eprint!("{}", t!("editor.tmpl_recover_prompt"));
        use std::io::Write;
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        let should_recover = std::io::stdin().read_line(&mut input).is_ok()
            && input.trim().eq_ignore_ascii_case("y");
        if should_recover {
            let content = data.lines.join("\n");
            let _ = std::fs::write(path, &content);
            crate::editor::recovery::RecoveryData::remove_recovery(path);
            let mut editor = Editor::new(path)?;
            editor.init_tabs();
            editor.cursor.y = data.cursor_y.min(editor.lines.len().saturating_sub(1));
            editor.cursor.byte = data.cursor_byte.min(editor.current().len());
            editor.sv.offset = data.scroll.min(editor.lines.len().saturating_sub(1));
            editor.sv.x = data.scroll_x;
            editor.modified = true;
            let result = match editor.run() {
                Ok(v) => v,
                Err(e) if e.to_string() == "__exit__" => false,
                Err(e) => return Err(e),
            };
            let captured = if result {
                None
            } else {
                Some(editor.capture_session())
            };
            return Ok((result, captured));
        } else {
            crate::editor::recovery::RecoveryData::remove_recovery(path);
        }
    }

    let mut editor = Editor::new(path)?;
    if let Some(ref session) = restored {
        if session.current_file == path {
            editor.restore_from_session(session);
        }
    }

    let has_session_snapshot = {
        let session = crate::session::SessionState::read_global();
        session.session_snapshot.is_some()
    };
    if has_session_snapshot {
        let snapshot = crate::session::SessionState::read_global()
            .session_snapshot
            .clone()
            .unwrap();
        let max_age_hours = editor.editor_cfg.session_max_age_hours;
        let is_recent =
            chrono::NaiveDateTime::parse_from_str(&snapshot.timestamp, "%Y-%m-%d %H:%M:%S")
                .ok()
                .map(|dt| {
                    let now = chrono::Local::now().naive_local();
                    let age = now.signed_duration_since(dt).num_hours();
                    age < max_age_hours as i64
                })
                .unwrap_or(false);
        if is_recent && !snapshot.tabs.is_empty() {
            let mut first_path = None;
            for tab_snap in &snapshot.tabs {
                if tab_snap.path.exists() {
                    if first_path.is_none() {
                        first_path = Some(tab_snap.path.clone());
                    } else {
                        editor.open_file(&tab_snap.path);
                    }
                }
            }
            if let Some(ref p) = first_path {
                if *p != path {
                    editor.open_file(path);
                }
            }
            for i in 0..snapshot.tabs.len().min(editor.tabs.len()) {
                editor.restore_tab_from_snapshot(i, &snapshot);
            }
            if snapshot.active_tab < editor.tabs.len() {
                editor.switch_tab(snapshot.active_tab);
            }
            editor.sidebar.open = snapshot.sidebar_open;
        }
    }

    let goto = crate::editor::GOTO_LINE.swap(0, std::sync::atomic::Ordering::SeqCst);
    if goto > 0 {
        editor.goto_line(goto);
    }

    let result = match editor.run() {
        Ok(v) => v,
        Err(e) if e.to_string() == "__exit__" => false,
        Err(e) => return Err(e),
    };
    let captured = if result {
        None
    } else {
        Some(editor.capture_session())
    };

    if !result {
        let snapshot = editor.capture_full_session();
        let mut session = crate::session::SessionState::write_global();
        session.session_snapshot = Some(snapshot);
        session.save();
    }

    Ok((result, captured))
}
