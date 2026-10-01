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
            // Rebuild tabs to mirror the snapshot (path-matched, so ordering and
            // missing files can never shift state onto the wrong buffer).
            let mut focus_tab: Option<usize> = None;
            for tab_snap in &snapshot.tabs {
                if !tab_snap.path.exists() {
                    continue;
                }
                match editor.find_tab_by_path(&tab_snap.path) {
                    Some(i) => editor.restore_tab_from_snapshot_path(i, tab_snap),
                    None => {
                        editor.open_file(&tab_snap.path);
                        if let Some(i) = editor.find_tab_by_path(&tab_snap.path) {
                            editor.restore_tab_from_snapshot_path(i, tab_snap);
                        }
                    }
                }
                if tab_snap.path == path {
                    focus_tab = editor.find_tab_by_path(path);
                }
            }
            // The requested file must always end up open and focused, even
            // when it is not part of the snapshot: the rebuild loop above
            // leaves the buffer on the last snapshot file, so without this
            // `ne <file>` would show stale cache instead of the request.
            // `open_file` no-ops to a tab switch when already present.
            let focus = focus_tab.or_else(|| editor.find_tab_by_path(path));
            match focus {
                Some(i) => editor.switch_tab(i),
                None => editor.open_file(path),
            }
            editor.sidebar.open = snapshot.sidebar_open;
        }
    }

    let goto = crate::editor::GOTO_LINE.swap(0, std::sync::atomic::Ordering::SeqCst);
    let highlight = crate::editor::GOTO_HIGHLIGHT.swap(false, std::sync::atomic::Ordering::SeqCst);
    let _jump_pattern = crate::editor::take_goto_pattern();
    if goto > 0 {
        if highlight {
            editor.goto_line_centered_highlight(goto);
        } else {
            editor.goto_line(goto);
        }
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
