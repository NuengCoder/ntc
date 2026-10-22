#[cfg(test)]
mod tests {
    use crate::editor::{CursorPos, Editor};

    fn test_editor(content: &[&str]) -> Editor {
        let dir = std::env::temp_dir().join("ntc_editor_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test_editor.txt");
        let lines: Vec<String> = content.iter().map(|s| s.to_string()).collect();
        std::fs::write(&path, lines.join("\n")).unwrap();
        let mut editor = Editor::new(&path).unwrap();
        editor.lines = lines.clone();
        if lines.is_empty() || lines[0].is_empty() {
            editor.lines = lines;
        }
        editor
    }

    #[test]
    fn terminal_box_geometry() {
        let mut editor = test_editor(&["one"]);
        editor.term_h = 40;
        editor.term_w = 100;
        editor.sidebar.open = false;
        let split = editor.run_split_row();
        assert_eq!(split, 20);
        let (top, left, right, bottom) = editor.terminal_box();
        assert_eq!((top, left), (20, 0));
        assert_eq!(right, 98);
        // Box bottom stays above the cwd/status/hint bars.
        assert!(bottom <= 40 - 4);
        let (cols, rows) = editor.terminal_content_size();
        assert_eq!(rows, editor.run_panel_height() - 1);
        assert_eq!(cols, 100 - 0 - 3);
        assert_eq!((cols, rows), (97, 15));
    }

    #[test]
    fn close_background_modified_tab_saves_file() {
        let dir = std::env::temp_dir().join("ntc_editor_test");
        let _ = std::fs::create_dir_all(&dir);
        let path_a = dir.join("close_a.txt");
        let path_b = dir.join("close_b.txt");
        std::fs::write(&path_a, "a").unwrap();
        std::fs::write(&path_b, "b").unwrap();

        let mut editor = Editor::new(&path_a).unwrap();
        editor.init_tabs();
        editor.open_file(&path_b);
        assert_eq!(editor.tabs.len(), 2);
        assert_eq!(editor.active_tab, 1);
        editor.switch_tab(0);
        // Edit tab 1's stored state directly (simulates unsaved edits on a
        // background tab), then close it.
        editor.tabs[1].lines = vec!["edited b".to_string()];
        editor.tabs[1].modified = true;
        let should_quit = editor.close_tab(1);
        assert_eq!(should_quit, false);
        assert_eq!(editor.tabs.len(), 1);
        let content = std::fs::read_to_string(&path_b).unwrap();
        assert_eq!(content, "edited b");
        let _ = std::fs::remove_file(&path_b);
        let _ = std::fs::remove_file(&path_a);
    }

    #[test]
    fn close_last_modified_tab_saves_before_scratch() {
        let dir = std::env::temp_dir().join("ntc_editor_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("close_last.txt");
        std::fs::write(&path, "original").unwrap();

        let mut editor = Editor::new(&path).unwrap();
        editor.init_tabs();
        editor.lines = vec!["edited".to_string()];
        editor.modified = true;
        let _ = editor.close_tab(0);
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, "edited");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_snapshot_undo_redo() {
        let mut editor = test_editor(&["line1", "line2", "line3"]);
        editor.snapshot();
        editor.lines[0] = "modified".to_string();
        assert_eq!(editor.lines[0], "modified");

        editor.undo();
        assert_eq!(editor.lines[0], "line1");

        editor.redo();
        assert_eq!(editor.lines[0], "modified");
    }

    #[test]
    fn test_undo_empty_stack() {
        let mut editor = test_editor(&["line1"]);
        let before = editor.lines[0].clone();
        editor.undo();
        assert_eq!(editor.lines[0], before);
    }

    #[test]
    fn test_redo_cleared_on_new_snapshot() {
        let mut editor = test_editor(&["line1"]);
        editor.snapshot();
        editor.lines[0] = "change1".to_string();
        editor.undo();
        assert_eq!(editor.lines[0], "line1");

        editor.snapshot();
        editor.lines[0] = "change2".to_string();
        editor.redo();
        assert_eq!(editor.lines[0], "change2");
    }

    #[test]
    fn test_max_undo_capped() {
        let mut editor = test_editor(&["line"]);
        let max_undo = editor.editor_cfg.undo_depth;
        for i in 0..max_undo + 50 {
            editor.snapshot();
            editor.lines[0] = format!("change{}", i);
        }
        assert!(editor.undo_stack.len() <= max_undo);
    }

    #[test]
    fn test_has_selection_no_anchor() {
        let editor = test_editor(&["hello world"]);
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_has_selection_with_anchor() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.byte = 5;
        assert!(editor.has_selection());
    }

    #[test]
    fn test_has_selection_identical_positions() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.byte = 0;
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_selected_text() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.byte = 5;
        assert_eq!(editor.selected_text(), "hello");
    }

    #[test]
    fn test_selected_text_multi_line() {
        let mut editor = test_editor(&["hello", "world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.y = 1;
        editor.cursor.byte = 3;
        assert_eq!(editor.selected_text(), "hello\nwor");
    }

    #[test]
    fn test_select_all() {
        let mut editor = test_editor(&["line1", "line2", "line3"]);
        editor.select_all();
        assert!(editor.has_selection());
        let (ay, _) = editor.cursor.selection_anchor.unwrap();
        assert_eq!(ay, 0);
        assert_eq!(editor.cursor.y, 2);
    }

    #[test]
    fn test_delete_selection_single_line() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.byte = 5;
        editor.modified = true;
        editor.delete_selection();
        assert_eq!(editor.lines[0], " world");
    }

    #[test]
    fn test_delete_selection_multi_line() {
        let mut editor = test_editor(&["hello", "cruel", "world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.y = 2;
        editor.cursor.byte = 3;
        editor.modified = true;
        editor.delete_selection();
        assert_eq!(editor.lines.len(), 1);
        assert_eq!(editor.lines[0], "ld");
    }

    #[test]
    fn test_clamp_cursor() {
        let mut editor = test_editor(&["short"]);
        editor.cursor.y = 100;
        editor.cursor.byte = 100;
        editor.clamp();
        assert_eq!(editor.cursor.y, 0);
        assert_eq!(editor.cursor.byte, 5);
    }

    #[test]
    fn test_yank_line() {
        let mut editor = test_editor(&["yank me"]);
        editor.yank_line();
        assert!(editor.clipboard.contains("yank me"));
    }

    #[test]
    fn test_copy_selection_no_selection() {
        let mut editor = test_editor(&["copy line"]);
        editor.copy_selection();
        assert!(editor.clipboard.contains("copy line"));
    }

    #[test]
    fn test_copy_selection_with_selection() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.byte = 5;
        editor.copy_selection();
        assert_eq!(editor.clipboard, "hello");
    }

    #[test]
    fn test_paste_fast_single_line() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.byte = 5;
        editor.paste_fast(" beautiful");
        assert_eq!(editor.lines[0], "hello beautiful world");
    }

    #[test]
    fn test_paste_fast_multi_line() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.byte = 5;
        editor.paste_fast("line1\nline2\nline3");
        assert_eq!(editor.lines[0], "helloline1");
        assert_eq!(editor.lines[1], "line2");
        assert_eq!(editor.lines[2], "line3 world");
    }

    #[test]
    fn test_paste_fast_empty() {
        let mut editor = test_editor(&["hello"]);
        editor.paste_fast("");
        assert_eq!(editor.lines[0], "hello");
    }

    #[test]
    fn test_select_word_under_cursor() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.byte = 6;
        let word = editor.select_word_under_cursor();
        assert_eq!(word, Some("world".to_string()));
    }

    #[test]
    fn test_clear_selection() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.byte = 5;
        assert!(editor.has_selection());
        editor.clear_selection();
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_clear_extra_cursors() {
        let mut editor = test_editor(&["line1", "line2", "line3"]);
        editor.cursor.extra_cursors.push(CursorPos {
            y: 1,
            byte: 0,
            anchor: None,
        });
        assert!(editor.has_multiple_cursors());
        editor.clear_extra_cursors();
        assert!(!editor.has_multiple_cursors());
    }

    #[test]
    fn test_cut_selection() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.byte = 5;
        editor.cut_selection();
        assert_eq!(editor.lines[0], " world");
    }

    #[test]
    fn save_utf16le_writes_bom_and_reloads() {
        use crate::editor::{save_lines_to, EditorEncoding};
        let dir = std::env::temp_dir().join("ntc_editor_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("utf16.txt");
        let lines = vec!["hello".to_string(), "world".to_string()];
        save_lines_to(&path, &lines, EditorEncoding::Utf16Le).unwrap();
        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(&bytes[0..2], b"\xff\xfe");
        let (decoded, enc) = crate::editor::core::read_file_lines(&path).unwrap();
        assert_eq!(enc, EditorEncoding::Utf16Le);
        assert_eq!(decoded, vec!["hello", "world"]);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn save_utf8_does_not_add_bom() {
        use crate::editor::{save_lines_to, EditorEncoding};
        let dir = std::env::temp_dir().join("ntc_editor_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("utf8.txt");
        let lines = vec!["plain".to_string()];
        save_lines_to(&path, &lines, EditorEncoding::Utf8).unwrap();
        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(bytes, b"plain");
        let (decoded, enc) = crate::editor::core::read_file_lines(&path).unwrap();
        assert_eq!(enc, EditorEncoding::Utf8);
        assert_eq!(decoded, vec!["plain"]);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn picker_encoding_list_has_all_four() {
        use crate::editor::ALL_ENCODINGS;
        assert_eq!(ALL_ENCODINGS.len(), 4);
        let mut seen = std::collections::HashSet::new();
        for e in ALL_ENCODINGS {
            assert!(seen.insert(e));
        }
    }

    #[test]
    fn reload_with_encoding_round_trips() {
        use crate::editor::{save_lines_to, EditorEncoding};
        let dir = std::env::temp_dir().join("ntc_editor_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("reload.txt");
        let lines = vec!["héllo".to_string(), "wörld".to_string()];
        save_lines_to(&path, &lines, EditorEncoding::Utf16Le).unwrap();

        let mut editor = Editor::new(&path).unwrap();
        editor.init_tabs();
        assert_eq!(editor.encoding, EditorEncoding::Utf16Le);
        assert_eq!(editor.lines, lines);

        // Wrong explicit encoding garbles the buffer...
        editor.encoding = EditorEncoding::Utf16Be;
        editor.reload_with_encoding().unwrap();
        assert_ne!(editor.lines, lines);
        assert!(!editor.modified);

        // ...and reloading with the correct one restores it.
        editor.encoding = EditorEncoding::Utf16Le;
        editor.reload_with_encoding().unwrap();
        assert_eq!(editor.lines, lines);
        assert!(!editor.modified);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn decode_windows1252_and_utf16be() {
        use crate::editor::core::tabs::decode_bytes_to_lines;
        use crate::editor::EditorEncoding;
        // 0xE9 is "é" in Windows-1252 / Latin-1.
        let latin: Vec<u8> = vec![0x68, 0xE9, 0x6C, 0x6C, 0x6F];
        let decoded = decode_bytes_to_lines(&latin, EditorEncoding::Windows1252);
        assert_eq!(decoded, vec!["héllo"]);

        let be: Vec<u8> = vec![0xFE, 0xFF, 0x00, 0x68, 0x00, 0x69, 0x00, 0x21];
        let decoded = decode_bytes_to_lines(&be, EditorEncoding::Utf16Be);
        assert_eq!(decoded, vec!["hi!"]);
    }

    #[test]
    fn find_tab_matches_across_unc_prefix() {
        let mut editor = test_editor(&["hello"]);
        editor.init_tabs();
        let p = editor.path.clone();
        assert_eq!(editor.find_tab_by_path(&p), Some(0));
        #[cfg(windows)]
        {
            let prefixed = std::path::PathBuf::from(format!(r"\\?\{}", p.display()));
            assert_eq!(editor.find_tab_by_path(&prefixed), Some(0));
        }
    }
    #[test]
    fn open_missing_requested_file_focuses_it() {
        // Reproduces the stale-snapshot state from `ne <file>`: the buffer
        // holds the requested file but tabs only contain an older file.
        // `open_file` must load and focus the request (edit.rs relies on it).
        let dir = std::env::temp_dir().join("ntc_editor_test");
        let _ = std::fs::create_dir_all(&dir);
        let old = dir.join("snapshot_old.txt");
        let wanted = dir.join("wanted_file.txt");
        std::fs::write(&old, "old cache").unwrap();
        std::fs::write(&wanted, "wanted content").unwrap();

        let mut editor = Editor::new(&old).unwrap();
        editor.init_tabs();
        // Simulate rebuild loop leaving buffer on requested content while
        // tabs only know the old file.
        editor.lines = vec!["wanted content".to_string()];

        editor.open_file(&wanted);
        assert_eq!(editor.path, wanted);
        assert_eq!(editor.lines, vec!["wanted content".to_string()]);
        assert_eq!(editor.find_tab_by_path(&wanted), Some(editor.active_tab));
        let _ = std::fs::remove_file(&old);
        let _ = std::fs::remove_file(&wanted);
    }

    // ── vim-style keymap regression (headless key-event simulation) ──

    use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

    use crate::editor::Mode;

    fn press(code: KeyCode) -> Event {
        Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::empty(),
        })
    }

    fn press_mod(code: KeyCode, mods: KeyModifiers) -> Event {
        Event::Key(KeyEvent {
            code,
            modifiers: mods,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::empty(),
        })
    }

    fn stdout_sink() -> std::io::Stdout {
        std::io::stdout()
    }

    #[test]
    fn normal_verbs_switch_modes() {
        let mut editor = test_editor(&["one", "two"]);
        let mut out = stdout_sink();
        assert_eq!(editor.mode, Mode::Normal);
        editor
            .handle_normal_event(press(KeyCode::Char('i')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Insert);
        editor.mode = Mode::Normal;
        editor
            .handle_normal_event(press(KeyCode::Char('v')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Visual);
        editor.mode = Mode::Normal;
        editor
            .handle_normal_event(press(KeyCode::Char(':')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Command);
        editor.mode = Mode::Normal;
        editor
            .handle_normal_event(press(KeyCode::Char('?')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Help);
        editor.mode = Mode::Normal;
        editor
            .handle_normal_event(press(KeyCode::Char('q')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Query);
        editor.mode = Mode::Normal;
        editor
            .handle_normal_event(press(KeyCode::Char('f')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Search);
        editor.mode = Mode::Normal;
        editor
            .handle_normal_event(press(KeyCode::Char('b')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::FileFinder);
        editor.mode = Mode::Normal;
        editor
            .handle_normal_event(press(KeyCode::Char('g')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Gmode);
        editor.mode = Mode::Normal;
        editor
            .handle_normal_event(press(KeyCode::Char('u')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Encoding);
    }

    #[test]
    fn normal_motion_letters_and_arrows() {
        let mut editor = test_editor(&["ab", "cde", "f"]);
        let mut out = stdout_sink();
        // down: j/J/Down (s is save in Normal); up: k/w/Up
        for code in [KeyCode::Char('j'), KeyCode::Char('J'), KeyCode::Down] {
            editor.cursor.y = 0;
            editor.handle_normal_event(press(code), &mut out).unwrap();
            assert_eq!(editor.cursor.y, 1);
        }
        for code in [KeyCode::Char('k'), KeyCode::Char('w'), KeyCode::Up] {
            editor.cursor.y = 1;
            editor.handle_normal_event(press(code), &mut out).unwrap();
            assert_eq!(editor.cursor.y, 0);
        }
        editor.cursor.byte = 0;
        for code in [KeyCode::Char('l'), KeyCode::Char('d'), KeyCode::Right] {
            editor.cursor.byte = 0;
            editor.handle_normal_event(press(code), &mut out).unwrap();
            assert_eq!(editor.cursor.byte, 1);
        }
        editor.cursor.byte = 1;
        for code in [KeyCode::Char('h'), KeyCode::Char('a'), KeyCode::Left] {
            editor.cursor.byte = 1;
            editor.handle_normal_event(press(code), &mut out).unwrap();
            assert_eq!(editor.cursor.byte, 0);
        }
        // typing-position keys never insert in Normal
        let before = editor.lines.clone();
        editor
            .handle_normal_event(press(KeyCode::Char('z')), &mut out)
            .unwrap();
        assert_eq!(editor.lines, before);
        assert_eq!(editor.mode, Mode::Normal);
    }

    #[test]
    fn normal_ctrl_j_enters_jump() {
        let mut editor = test_editor(&["one"]);
        let mut out = stdout_sink();
        editor
            .handle_normal_event(
                press_mod(KeyCode::Char('j'), KeyModifiers::CONTROL),
                &mut out,
            )
            .unwrap();
        assert_eq!(editor.mode, Mode::Jump);
    }

    #[test]
    fn insert_ctrl_remap() {
        let mut editor = test_editor(&["one"]);
        let mut out = stdout_sink();
        // ctrl+e enters nfe (sidebar opens).
        editor.mode = Mode::Insert;
        editor
            .handle_insert_event(
                press_mod(KeyCode::Char('e'), KeyModifiers::CONTROL),
                &mut out,
            )
            .unwrap();
        assert_eq!(editor.mode, Mode::Nfe);
        assert!(editor.sidebar.open);
        // ctrl+g enters gmode.
        editor.mode = Mode::Insert;
        editor.sidebar.open = false;
        editor
            .handle_insert_event(
                press_mod(KeyCode::Char('g'), KeyModifiers::CONTROL),
                &mut out,
            )
            .unwrap();
        assert_eq!(editor.mode, Mode::Gmode);
        // ctrl+w enters visual.
        editor.mode = Mode::Insert;
        editor
            .handle_insert_event(
                press_mod(KeyCode::Char('w'), KeyModifiers::CONTROL),
                &mut out,
            )
            .unwrap();
        assert_eq!(editor.mode, Mode::Visual);
        // ctrl+: and ctrl+; enter command mode.
        for code in [KeyCode::Char(':'), KeyCode::Char(';')] {
            editor.mode = Mode::Insert;
            editor
                .handle_insert_event(press_mod(code, KeyModifiers::CONTROL), &mut out)
                .unwrap();
            assert_eq!(editor.mode, Mode::Command);
        }
        // ctrl+j is a no-op in insert (Jump stays via Normal/Command).
        editor.mode = Mode::Insert;
        editor
            .handle_insert_event(
                press_mod(KeyCode::Char('j'), KeyModifiers::CONTROL),
                &mut out,
            )
            .unwrap();
        assert_eq!(editor.mode, Mode::Insert);
    }

    #[test]
    fn gmode_i_v_e_switch_modes() {
        let mut editor = test_editor(&["one"]);
        let mut out = stdout_sink();
        for (keys, want) in [(vec!['i'], Mode::Insert), (vec!['v'], Mode::Visual)] {
            editor.mode = Mode::Gmode;
            editor.prev_mode = Mode::Normal;
            editor.gmode_buf.clear();
            for c in keys {
                editor
                    .handle_gmode_event(press(KeyCode::Char(c)), &mut out)
                    .unwrap();
            }
            editor
                .handle_gmode_event(press(KeyCode::Enter), &mut out)
                .unwrap();
            assert_eq!(editor.mode, want);
        }
        // ge opens the sidebar explorer.
        editor.mode = Mode::Gmode;
        editor.prev_mode = Mode::Normal;
        editor.sidebar.open = false;
        editor.gmode_buf.clear();
        editor
            .handle_gmode_event(press(KeyCode::Char('e')), &mut out)
            .unwrap();
        editor
            .handle_gmode_event(press(KeyCode::Enter), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Nfe);
        assert!(editor.sidebar.open);
    }

    #[test]
    fn command_e_opens_nfe() {
        let mut editor = test_editor(&["one"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Command;
        editor.sidebar.open = false;
        editor.cmd_buf = "e".to_string();
        editor
            .handle_command_event(press(KeyCode::Enter), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Nfe);
        assert!(editor.sidebar.open);
    }

    #[test]
    fn normal_x_saves_and_s_moves() {
        let dir = std::env::temp_dir().join("ntc_editor_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("nsave_test.txt");
        std::fs::write(&path, "one\ntwo").unwrap();
        let mut editor = Editor::new(&path).unwrap();
        editor.lines = vec!["one".to_string(), "two".to_string()];
        editor.modified = true;
        let mut out = stdout_sink();
        editor
            .handle_normal_event(press(KeyCode::Char('x')), &mut out)
            .unwrap();
        assert!(!editor.modified);
        assert_eq!(editor.mode, Mode::Normal);
        // s is motion now, not save
        editor.cursor.y = 0;
        editor
            .handle_normal_event(press(KeyCode::Char('s')), &mut out)
            .unwrap();
        assert_eq!(editor.cursor.y, 1);
        assert_eq!(editor.mode, Mode::Normal);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn normal_n_opens_create_prompt_and_esc_cancels() {
        let mut editor = test_editor(&["one"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Normal;
        editor
            .handle_normal_event(press(KeyCode::Char('n')), &mut out)
            .unwrap();
        assert_eq!(editor.create_kind, Some(crate::editor::CreateKind::File));
        for c in "hi".chars() {
            editor
                .handle_normal_event(press(KeyCode::Char(c)), &mut out)
                .unwrap();
        }
        assert_eq!(editor.create_buf, "hi");
        editor
            .handle_normal_event(press(KeyCode::Esc), &mut out)
            .unwrap();
        assert!(editor.create_kind.is_none());
        assert!(editor.create_buf.is_empty());
        assert_eq!(editor.mode, Mode::Normal);
    }

    #[test]
    fn normal_m_opens_dir_prompt() {
        let mut editor = test_editor(&["one"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Normal;
        editor
            .handle_normal_event(press(KeyCode::Char('m')), &mut out)
            .unwrap();
        assert_eq!(editor.create_kind, Some(crate::editor::CreateKind::Dir));
    }

    #[test]
    fn normal_n_creates_template_without_opening_tab() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[("probe.txt", "x")], &[]);
        let mut editor = Editor::new(std::path::Path::new("probe.txt")).unwrap();
        editor.init_tabs();
        let tabs_before = editor.tabs.len();
        let mut out = stdout_sink();
        editor.mode = Mode::Normal;
        editor
            .handle_normal_event(press(KeyCode::Char('n')), &mut out)
            .unwrap();
        for c in "new.rs".chars() {
            editor
                .handle_normal_event(press(KeyCode::Char(c)), &mut out)
                .unwrap();
        }
        editor
            .handle_normal_event(press(KeyCode::Enter), &mut out)
            .unwrap();
        assert!(std::path::Path::new("new.rs").is_file());
        assert!(std::fs::read_to_string("new.rs")
            .unwrap()
            .contains("fn main"));
        assert_eq!(editor.tabs.len(), tabs_before);
        assert_eq!(editor.path, std::path::Path::new("probe.txt"));
        assert_eq!(editor.mode, Mode::Normal);
    }

    #[test]
    fn normal_ctrl_word_motion_jumps_words() {
        let mut editor = test_editor(&["foo bar baz"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Normal;
        editor.cursor.byte = 0;
        editor
            .handle_normal_event(
                press_mod(KeyCode::Char('l'), KeyModifiers::CONTROL),
                &mut out,
            )
            .unwrap();
        assert!(editor.cursor.byte > 0);
        let after_l = editor.cursor.byte;
        editor
            .handle_normal_event(press_mod(KeyCode::Right, KeyModifiers::CONTROL), &mut out)
            .unwrap();
        assert!(editor.cursor.byte >= after_l);
        editor
            .handle_normal_event(
                press_mod(KeyCode::Char('h'), KeyModifiers::CONTROL),
                &mut out,
            )
            .unwrap();
        assert!(editor.cursor.byte < after_l + 10);
    }

    #[test]
    fn visual_i_enters_insert_keeping_selection() {
        let mut editor = test_editor(&["ab", "cd"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Visual;
        editor.cursor.y = 1;
        editor.cursor.byte = 2;
        editor.cursor.selection_anchor = Some((0, 0));
        editor
            .handle_visual_event(press(KeyCode::Char('i')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Insert);
        assert!(editor.cursor.selection_anchor.is_some());
    }

    #[test]
    fn visual_u_r_undo_redo_stays_visual() {
        let mut editor = test_editor(&["ab", "cd"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Visual;
        editor.snapshot();
        editor.lines[0] = "AB".to_string();
        editor
            .handle_visual_event(press(KeyCode::Char('u')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Visual);
        assert_eq!(editor.lines[0], "ab");
        editor
            .handle_visual_event(press(KeyCode::Char('r')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Visual);
        assert_eq!(editor.lines[0], "AB");
    }

    #[test]
    fn visual_ctrl_word_motion_moves_and_clears() {
        let mut editor = test_editor(&["foo bar"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Visual;
        editor.cursor.byte = 0;
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.byte = 3;
        editor
            .handle_visual_event(
                press_mod(KeyCode::Char('l'), KeyModifiers::CONTROL),
                &mut out,
            )
            .unwrap();
        assert!(!editor.has_selection());
        assert!(editor.cursor.byte >= 3);
    }

    #[test]
    fn normal_shift_arrow_extends_and_c_copies() {
        let mut editor = test_editor(&["abcdef"]);
        let mut out = stdout_sink();
        editor
            .handle_normal_event(press_mod(KeyCode::Right, KeyModifiers::SHIFT), &mut out)
            .unwrap();
        assert!(editor.has_selection());
        assert_eq!(editor.mode, Mode::Normal);
        editor
            .handle_normal_event(press(KeyCode::Char('c')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Normal);
    }

    #[test]
    fn terminal_esc_closes_panel() {
        let mut editor = test_editor(&["one"]);
        let mut out = stdout_sink();
        editor.toggle_terminal();
        if editor.terminal.is_none() {
            return; // no PTY here; nothing to close
        }
        assert_eq!(editor.focus, crate::editor::Focus::Terminal);
        editor
            .handle_terminal_event(press(KeyCode::Esc), &mut out)
            .unwrap();
        assert!(editor.terminal.is_none());
        assert_eq!(editor.focus, crate::editor::Focus::Editor);
    }

    #[test]
    fn help_hjkl_scrolls() {
        let mut editor = test_editor(&["one"]);
        editor.mode = Mode::Help;
        let start = editor.sv.help;
        editor.handle_help_event(press(KeyCode::Char('j')));
        assert_eq!(editor.sv.help, start + 1);
        editor.handle_help_event(press(KeyCode::Char('s')));
        assert_eq!(editor.sv.help, start + 2);
        editor.handle_help_event(press(KeyCode::Char('k')));
        assert_eq!(editor.sv.help, start + 1);
        editor.handle_help_event(press(KeyCode::Down));
        assert_eq!(editor.sv.help, start + 2);
        assert_eq!(editor.mode, Mode::Help);
    }

    #[test]
    fn insert_ctrl_q_returns_to_normal() {
        let mut editor = test_editor(&["one"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Insert;
        editor
            .handle_insert_event(
                press_mod(KeyCode::Char('q'), KeyModifiers::CONTROL),
                &mut out,
            )
            .unwrap();
        assert_eq!(editor.mode, Mode::Normal);
    }

    #[test]
    fn visual_verbs_and_extend() {
        let mut editor = test_editor(&["ab", "cd"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Visual;
        // Shift+arrows extend the selection; plain motion does not
        editor
            .handle_visual_event(press_mod(KeyCode::Right, KeyModifiers::SHIFT), &mut out)
            .unwrap();
        assert!(editor.has_selection());
        editor
            .handle_visual_event(press(KeyCode::Char('l')), &mut out)
            .unwrap();
        assert!(!editor.has_selection());
        assert_eq!(editor.cursor.byte, 2);
        // a toggles select-all
        editor
            .handle_visual_event(press(KeyCode::Char('a')), &mut out)
            .unwrap();
        assert_eq!(editor.cursor.y, 1);
        assert_eq!(editor.cursor.byte, 2);
        // : enters command
        editor
            .handle_visual_event(press(KeyCode::Char(':')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Command);
        // g enters gmode
        editor.mode = Mode::Visual;
        editor
            .handle_visual_event(press(KeyCode::Char('g')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Gmode);
        // Esc exits to normal
        editor.mode = Mode::Visual;
        editor
            .handle_visual_event(press(KeyCode::Esc), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Normal);
    }

    fn select_rows(editor: &mut crate::editor::Editor, sy: usize, ey: usize) {
        editor.cursor.y = ey;
        editor.cursor.byte = editor.lines[ey].len();
        editor.cursor.selection_anchor = Some((sy, 0));
    }

    #[test]
    fn visual_alt_moves_selected_block_and_follows() {
        let mut editor = test_editor(&["a", "b", "c", "d"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Visual;
        select_rows(&mut editor, 0, 1);
        let undo_before = editor.undo_stack.len();
        editor
            .handle_visual_event(press_mod(KeyCode::Char('j'), KeyModifiers::ALT), &mut out)
            .unwrap();
        assert_eq!(editor.lines, vec!["c", "a", "b", "d"]);
        assert_eq!(editor.undo_stack.len(), undo_before + 1);
        // Selection followed the block (rows 1..=2).
        let (ay, _) = editor.cursor.selection_anchor.unwrap();
        assert_eq!((editor.cursor.y.min(ay), editor.cursor.y.max(ay)), (1, 2));
        assert_eq!(editor.mode, Mode::Visual);
        // Move back up with Alt+k.
        editor
            .handle_visual_event(press_mod(KeyCode::Char('k'), KeyModifiers::ALT), &mut out)
            .unwrap();
        assert_eq!(editor.lines, vec!["a", "b", "c", "d"]);
        // Undo restores the moved state step by step.
        editor.undo();
        assert_eq!(editor.lines, vec!["c", "a", "b", "d"]);
    }

    #[test]
    fn visual_alt_block_edges_are_noops() {
        let mut editor = test_editor(&["a", "b", "c"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Visual;
        // Top edge stays a no-op (no snapshot).
        select_rows(&mut editor, 0, 1);
        let undo_before = editor.undo_stack.len();
        editor
            .handle_visual_event(press_mod(KeyCode::Char('k'), KeyModifiers::ALT), &mut out)
            .unwrap();
        assert_eq!(editor.lines, vec!["a", "b", "c"]);
        assert_eq!(editor.undo_stack.len(), undo_before);
        // Bottom edge now creates a line and moves (see
        // visual_alt_j_block_at_bottom_creates_one_line).
    }

    #[test]
    fn visual_alt_arrows_match_hjkl() {
        let mut editor = test_editor(&["a", "b", "c"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Visual;
        select_rows(&mut editor, 0, 0);
        editor
            .handle_visual_event(press_mod(KeyCode::Down, KeyModifiers::ALT), &mut out)
            .unwrap();
        assert_eq!(editor.lines, vec!["b", "a", "c"]);
        editor
            .handle_visual_event(press_mod(KeyCode::Up, KeyModifiers::ALT), &mut out)
            .unwrap();
        assert_eq!(editor.lines, vec!["a", "b", "c"]);
    }

    #[test]
    fn visual_alt_indent_dedent_roundtrip() {
        let mut editor = test_editor(&["ab", "cd"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Visual;
        select_rows(&mut editor, 0, 1);
        let tab = " ".repeat(editor.editor_cfg.tab_size);
        editor
            .handle_visual_event(press_mod(KeyCode::Char('l'), KeyModifiers::ALT), &mut out)
            .unwrap();
        assert_eq!(editor.lines[0], format!("{tab}ab"));
        assert_eq!(editor.lines[1], format!("{tab}cd"));
        assert!(editor.has_selection());
        editor
            .handle_visual_event(press_mod(KeyCode::Left, KeyModifiers::ALT), &mut out)
            .unwrap();
        assert_eq!(editor.lines, vec!["ab", "cd"]);
    }

    #[test]
    fn visual_alt_h_aborts_when_line_has_no_indent() {
        let mut editor = test_editor(&["  ab", "cd"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Visual;
        select_rows(&mut editor, 0, 1);
        let undo_before = editor.undo_stack.len();
        editor
            .handle_visual_event(press_mod(KeyCode::Char('h'), KeyModifiers::ALT), &mut out)
            .unwrap();
        // Whole block untouched: no snapshot, lines identical, hint shown.
        assert_eq!(editor.lines, vec!["  ab", "cd"]);
        assert_eq!(editor.undo_stack.len(), undo_before);
        assert!(editor.status_msg.is_some());
        assert!(editor.has_selection());
        assert_eq!(editor.mode, Mode::Visual);
    }

    #[test]
    fn visual_alt_h_empty_lines_dont_block() {
        let mut editor = test_editor(&["  ab", "", "  cd"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Visual;
        select_rows(&mut editor, 0, 2);
        editor
            .handle_visual_event(press_mod(KeyCode::Left, KeyModifiers::ALT), &mut out)
            .unwrap();
        assert_eq!(editor.lines, vec!["ab", "", "cd"]);
    }

    #[test]
    fn visual_alt_j_at_bottom_creates_line_and_follows() {
        let mut editor = test_editor(&["a", "b"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Visual;
        select_rows(&mut editor, 1, 1);
        let undo_before = editor.undo_stack.len();
        editor
            .handle_visual_event(press_mod(KeyCode::Char('j'), KeyModifiers::ALT), &mut out)
            .unwrap();
        // Empty line created, block moved into it, selection follows.
        assert_eq!(editor.lines, vec!["a", "", "b"]);
        assert_eq!(editor.undo_stack.len(), undo_before + 1);
        let (ay, _) = editor.cursor.selection_anchor.unwrap();
        assert_eq!((editor.cursor.y.min(ay), editor.cursor.y.max(ay)), (2, 2));
        // Undo removes the created line too.
        editor.undo();
        assert_eq!(editor.lines, vec!["a", "b"]);
    }

    #[test]
    fn visual_alt_j_block_at_bottom_creates_one_line() {
        let mut editor = test_editor(&["a", "b", "c"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Visual;
        select_rows(&mut editor, 1, 2);
        editor
            .handle_visual_event(press_mod(KeyCode::Down, KeyModifiers::ALT), &mut out)
            .unwrap();
        assert_eq!(editor.lines, vec!["a", "", "b", "c"]);
        let (ay, _) = editor.cursor.selection_anchor.unwrap();
        assert_eq!((editor.cursor.y.min(ay), editor.cursor.y.max(ay)), (2, 3));
        editor.undo();
        assert_eq!(editor.lines, vec!["a", "b", "c"]);
    }

    #[test]
    fn visual_alt_without_selection_is_noop() {
        let mut editor = test_editor(&["ab", "cd"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Visual;
        assert!(!editor.has_selection());
        let undo_before = editor.undo_stack.len();
        for code in [
            KeyCode::Char('j'),
            KeyCode::Char('k'),
            KeyCode::Char('h'),
            KeyCode::Char('l'),
            KeyCode::Up,
            KeyCode::Down,
            KeyCode::Left,
            KeyCode::Right,
        ] {
            editor
                .handle_visual_event(press_mod(code, KeyModifiers::ALT), &mut out)
                .unwrap();
        }
        assert_eq!(editor.lines, vec!["ab", "cd"]);
        assert_eq!(editor.undo_stack.len(), undo_before);
        assert_eq!(editor.mode, Mode::Visual);
    }

    #[test]
    fn gmode_tab_and_goto() {
        let mut editor = test_editor(&["one", "two", "three"]);
        editor.init_tabs();
        let mut out = stdout_sink();
        editor.mode = Mode::Gmode;
        // goto line 3
        for c in ['3'] {
            editor
                .handle_gmode_event(press(KeyCode::Char(c)), &mut out)
                .unwrap();
        }
        editor
            .handle_gmode_event(press(KeyCode::Enter), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Normal);
        assert_eq!(editor.cursor.y, 2);
        // next tab
        editor.mode = Mode::Gmode;
        editor
            .handle_gmode_event(press(KeyCode::Char('t')), &mut out)
            .unwrap();
        editor
            .handle_gmode_event(press(KeyCode::Enter), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Normal);
        // unknown word stays in gmode with a message
        editor.mode = Mode::Gmode;
        editor
            .handle_gmode_event(press(KeyCode::Char('z')), &mut out)
            .unwrap();
        editor
            .handle_gmode_event(press(KeyCode::Char('z')), &mut out)
            .unwrap();
        editor
            .handle_gmode_event(press(KeyCode::Enter), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Gmode);
        assert!(editor.status_msg.is_some());
        // Esc restores the invoker
        editor
            .handle_gmode_event(press(KeyCode::Esc), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Normal);
    }

    #[test]
    fn gmode_u_r_undo_redo() {
        let mut editor = test_editor(&["one", "two"]);
        let mut out = stdout_sink();
        editor.snapshot();
        editor.lines[0] = "CHANGED".to_string();
        editor.mode = Mode::Gmode;
        editor.prev_mode = Mode::Normal;
        editor
            .handle_gmode_event(press(KeyCode::Char('u')), &mut out)
            .unwrap();
        editor
            .handle_gmode_event(press(KeyCode::Enter), &mut out)
            .unwrap();
        assert_eq!(editor.lines[0], "one");
        assert_eq!(editor.mode, Mode::Normal);
        editor.mode = Mode::Gmode;
        editor.prev_mode = Mode::Normal;
        editor
            .handle_gmode_event(press(KeyCode::Char('r')), &mut out)
            .unwrap();
        editor
            .handle_gmode_event(press(KeyCode::Enter), &mut out)
            .unwrap();
        assert_eq!(editor.lines[0], "CHANGED");
    }

    #[test]
    fn gmode_n_m_create_file_and_dir() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[("probe.txt", "x")], &[]);
        let dir = std::env::current_dir().unwrap();
        let probe = dir.join("probe.txt");
        let mut editor = Editor::new(&probe).unwrap();
        let mut out = stdout_sink();
        editor.mode = Mode::Gmode;
        editor.prev_mode = Mode::Normal;
        for c in "n fresh.rs".chars() {
            editor
                .handle_gmode_event(press(KeyCode::Char(c)), &mut out)
                .unwrap();
        }
        editor
            .handle_gmode_event(press(KeyCode::Enter), &mut out)
            .unwrap();
        assert!(dir.join("fresh.rs").is_file());
        assert!(std::fs::read_to_string(dir.join("fresh.rs"))
            .unwrap()
            .contains("fn main"));
        assert!(editor.tabs.is_empty());
        assert_eq!(editor.mode, Mode::Normal);
        editor.mode = Mode::Gmode;
        editor.prev_mode = Mode::Normal;
        for c in "m subdir".chars() {
            editor
                .handle_gmode_event(press(KeyCode::Char(c)), &mut out)
                .unwrap();
        }
        editor
            .handle_gmode_event(press(KeyCode::Enter), &mut out)
            .unwrap();
        assert!(dir.join("subdir").is_dir());
        // Bare n stays open with usage.
        editor.mode = Mode::Gmode;
        editor.prev_mode = Mode::Normal;
        editor
            .handle_gmode_event(press(KeyCode::Char('n')), &mut out)
            .unwrap();
        editor
            .handle_gmode_event(press(KeyCode::Enter), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Gmode);
    }

    #[test]
    fn command_ne_creates_templates_without_opening_tabs() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[("probe.txt", "x")], &[]);
        let mut editor = Editor::new(std::path::Path::new("probe.txt")).unwrap();
        editor.init_tabs();
        let tabs_before = editor.tabs.len();
        let mut out = stdout_sink();

        editor
            .execute_command("ne --init new.rs", &mut out)
            .unwrap();
        assert!(std::path::Path::new("new.rs").is_file());
        assert!(std::fs::read_to_string("new.rs")
            .unwrap()
            .contains("fn main"));
        assert_eq!(editor.tabs.len(), tabs_before);
        assert_eq!(editor.path, std::path::Path::new("probe.txt"));

        editor
            .execute_command("ne --super rich.rs", &mut out)
            .unwrap();
        assert!(std::path::Path::new("rich.rs").is_file());
        assert_eq!(editor.tabs.len(), tabs_before);

        editor.execute_command("ne --init a b", &mut out).unwrap();
        assert!(!std::path::Path::new("a b").exists());
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn command_ne_missing_file_gives_guidance() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[("probe.txt", "x")], &[]);
        let mut editor = Editor::new(std::path::Path::new("probe.txt")).unwrap();
        let mut out = stdout_sink();
        editor.execute_command("ne missing.rs", &mut out).unwrap();
        assert!(!std::path::Path::new("missing.rs").exists());
        assert!(editor
            .status_msg
            .as_deref()
            .unwrap_or("")
            .contains("--init"));
        assert!(editor
            .status_msg
            .as_deref()
            .unwrap_or("")
            .contains("--super"));
    }

    #[test]
    fn command_u_r_undo_redo() {
        let mut editor = test_editor(&["one"]);
        let mut out = stdout_sink();
        editor.snapshot();
        editor.lines[0] = "CHANGED".to_string();
        editor.execute_command("u", &mut out).unwrap();
        assert_eq!(editor.lines[0], "one");
        editor.execute_command("r", &mut out).unwrap();
        assert_eq!(editor.lines[0], "CHANGED");
    }

    #[test]
    fn command_bang_runs_in_terminal_without_panic() {
        let mut editor = test_editor(&["one"]);
        let mut out = stdout_sink();
        editor.execute_command("! echo hi", &mut out).unwrap();
        // Headless CI may lack a PTY: either the terminal opened with
        // focus, or a status error was reported. Never a panic.
        assert!(editor.terminal.is_some() || editor.status_msg.is_some());
        editor.execute_command("!", &mut out).unwrap();
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn nfe_enter_move_exit() {
        let mut editor = test_editor(&["one"]);
        let mut out = stdout_sink();
        editor.prev_mode = Mode::Normal;
        editor.enter_nfe();
        assert_eq!(editor.mode, Mode::Nfe);
        assert!(editor.sidebar.open);
        let first = editor.sidebar.selected;
        editor
            .handle_nfe_event(press(KeyCode::Down), &mut out)
            .unwrap();
        assert!(editor.sidebar.selected >= first);
        editor
            .handle_nfe_event(press(KeyCode::Esc), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Normal);
    }

    #[test]
    fn nfe_exit_hides_sidebar_and_toggles() {
        let mut editor = test_editor(&["one"]);
        let mut out = stdout_sink();
        // e toggles out when already inside
        editor.prev_mode = Mode::Normal;
        editor.enter_nfe();
        assert!(editor.sidebar.open);
        editor
            .handle_nfe_event(press(KeyCode::Char('e')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Normal);
        assert!(!editor.sidebar.open);
        // Ctrl+Q also exits and hides
        editor.prev_mode = Mode::Normal;
        editor.enter_nfe();
        assert!(editor.sidebar.open);
        editor
            .handle_nfe_event(
                press_mod(KeyCode::Char('q'), KeyModifiers::CONTROL),
                &mut out,
            )
            .unwrap();
        assert_eq!(editor.mode, Mode::Normal);
        assert!(!editor.sidebar.open);
    }

    #[test]
    fn normal_h_moves_and_question_helps() {
        // H is motion again: bare H moves left, flagged Shift+H extends,
        // and ? owns Help.
        let mut editor = test_editor(&["abcdef"]);
        let mut out = stdout_sink();
        editor.cursor.byte = 2;
        editor
            .handle_normal_event(press(KeyCode::Char('H')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Normal);
        assert_eq!(editor.cursor.byte, 1);
        assert!(!editor.has_selection());
        editor.cursor.byte = 2;
        editor
            .handle_normal_event(press_mod(KeyCode::Char('H'), KeyModifiers::SHIFT), &mut out)
            .unwrap();
        assert!(editor.has_selection());
        assert_eq!(editor.mode, Mode::Normal);
        editor.clear_selection();
        editor
            .handle_normal_event(press(KeyCode::Char('?')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Help);
        editor.mode = Mode::Normal;
        // unfolded Shift+; still reaches command mode
        editor
            .handle_normal_event(press_mod(KeyCode::Char(';'), KeyModifiers::SHIFT), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Command);
    }

    #[test]
    fn toggle_terminal_focuses_terminal() {
        let mut editor = test_editor(&["one"]);
        editor.toggle_terminal();
        if editor.terminal.is_some() {
            assert_eq!(editor.focus, crate::editor::Focus::Terminal);
            editor.toggle_terminal();
            assert!(editor.terminal.is_none());
            assert_eq!(editor.focus, crate::editor::Focus::Editor);
        } else {
            // No PTY on this machine: stays closed, focus untouched.
            assert_eq!(editor.focus, crate::editor::Focus::Editor);
        }
    }

    #[test]
    fn start_run_focuses_run_panel() {
        let mut editor = test_editor(&["one"]);
        editor.start_run("echo hi");
        assert!(editor.run_panel_open);
        assert_eq!(editor.focus, crate::editor::Focus::Run);
    }

    #[test]
    fn normal_flagged_uppercase_extends() {
        let mut editor = test_editor(&["abcdef"]);
        let mut out = stdout_sink();
        // kitty-style Shift+H (flagged uppercase) selects left
        editor.cursor.byte = 2;
        editor
            .handle_normal_event(press_mod(KeyCode::Char('H'), KeyModifiers::SHIFT), &mut out)
            .unwrap();
        assert!(editor.has_selection());
        assert_eq!(editor.mode, Mode::Normal);
        // bare H moves left again (? owns Help)
        editor.clear_selection();
        editor.cursor.byte = 2;
        editor
            .handle_normal_event(press(KeyCode::Char('H')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Normal);
        assert_eq!(editor.cursor.byte, 1);
        assert!(!editor.has_selection());
    }

    #[test]
    fn nfe_size_prompt_flow() {
        let mut editor = test_editor(&["one"]);
        let mut out = stdout_sink();
        // Hermetic: snapshot the real user config, restore at the end.
        let pct_before = editor.editor_cfg.sidebar_width_pct;
        editor.prev_mode = Mode::Normal;
        editor.enter_nfe();
        // : opens the prompt
        editor
            .handle_nfe_event(press(KeyCode::Char(':')), &mut out)
            .unwrap();
        assert_eq!(editor.nfe_cmd, Some(String::new()));
        // type an invalid command -> error status, prompt closes, value kept
        for c in "size huge".chars() {
            editor
                .handle_nfe_event(press(KeyCode::Char(c)), &mut out)
                .unwrap();
        }
        assert_eq!(editor.nfe_cmd, Some("size huge".to_string()));
        editor
            .handle_nfe_event(press(KeyCode::Enter), &mut out)
            .unwrap();
        assert_eq!(editor.nfe_cmd, Some("size huge".to_string()));
        assert!(editor.status_msg.is_some());
        assert_eq!(editor.editor_cfg.sidebar_width_pct, pct_before);
        // Esc cancels, then a valid command applies + persists
        editor
            .handle_nfe_event(press(KeyCode::Esc), &mut out)
            .unwrap();
        assert_eq!(editor.nfe_cmd, None);
        editor
            .handle_nfe_event(press(KeyCode::Char(':')), &mut out)
            .unwrap();
        for c in "size 50%".chars() {
            editor
                .handle_nfe_event(press(KeyCode::Char(c)), &mut out)
                .unwrap();
        }
        editor
            .handle_nfe_event(press(KeyCode::Enter), &mut out)
            .unwrap();
        assert_eq!(editor.editor_cfg.sidebar_width_pct, Some(50));
        // Esc cancels an open prompt
        editor
            .handle_nfe_event(press(KeyCode::Char(':')), &mut out)
            .unwrap();
        editor
            .handle_nfe_event(press(KeyCode::Esc), &mut out)
            .unwrap();
        assert_eq!(editor.nfe_cmd, None);
        assert_eq!(editor.mode, Mode::Nfe);
        // restore the user's real config
        editor.editor_cfg.sidebar_width_pct = pct_before;
        editor.editor_cfg.save();
    }

    #[test]
    fn help_colon_enters_command() {
        let mut editor = test_editor(&["one"]);
        editor.mode = Mode::Help;
        editor.handle_help_event(press(KeyCode::Char(':')));
        assert_eq!(editor.mode, Mode::Command);
    }

    // ── MQL completion popup (insert mode, headless) ──

    fn mql_editor(content: &[&str]) -> Editor {
        let mut editor = test_editor(content);
        editor.syntax.set_language(Some("mq5"));
        editor.syntax.resize_cache(editor.lines.len());
        editor.mode = Mode::Insert;
        editor
    }

    #[test]
    fn completion_triggers_cycles_accepts() {
        let mut editor = mql_editor(&["void fun(int a, int b){}", ""]);
        let mut out = stdout_sink();
        editor.cursor.y = 1;
        editor.cursor.byte = 0;
        editor
            .handle_insert_event(press(KeyCode::Char('f')), &mut out)
            .unwrap();
        let st = editor.completion.as_ref().expect("list should show");
        assert_eq!(st.prefix, "f");
        assert_eq!(st.items[0].name, "fun");
        assert_eq!(st.selected, 0);
        // Down cycles, wraps at the end
        let n = st.items.len();
        assert!(n > 1);
        editor
            .handle_insert_event(press(KeyCode::Down), &mut out)
            .unwrap();
        assert_eq!(editor.completion.as_ref().unwrap().selected, 1 % n);
        editor
            .handle_insert_event(press(KeyCode::Up), &mut out)
            .unwrap();
        assert_eq!(editor.completion.as_ref().unwrap().selected, 0);
        // Enter accepts: fun() with cursor inside + sig-help open
        editor
            .handle_insert_event(press(KeyCode::Enter), &mut out)
            .unwrap();
        assert_eq!(editor.lines[1], "fun()");
        assert_eq!(editor.cursor.byte, 4);
        assert!(editor.completion.is_none());
        let sig = editor.sighelp.as_ref().expect("sig-help should open");
        assert_eq!(sig.label, "fun");
        assert_eq!(sig.ret, "void");
        assert_eq!(sig.params, vec!["int a", "int b"]);
        assert_eq!(sig.active, 0);
    }

    #[test]
    fn completion_esc_dismisses_and_paren_commits() {
        let mut editor = mql_editor(&["void myfun(int q){}", "myfu"]);
        let mut out = stdout_sink();
        editor.cursor.y = 1;
        editor.cursor.byte = 4;
        editor.maybe_trigger_completion();
        assert!(editor.completion.is_some());
        editor
            .handle_insert_event(press(KeyCode::Esc), &mut out)
            .unwrap();
        assert!(editor.completion.is_none());
        assert_eq!(editor.mode, Mode::Insert);
        // ( commits the selected function + opens sig-help
        editor.maybe_trigger_completion();
        editor
            .handle_insert_event(press(KeyCode::Char('(')), &mut out)
            .unwrap();
        assert_eq!(editor.lines[1], "myfun()");
        assert_eq!(editor.cursor.byte, 6);
        let sig = editor.sighelp.as_ref().expect("sig-help should open");
        assert_eq!(sig.label, "myfun");
        // ) closes past the call: sig-help dismisses
        editor
            .handle_insert_event(press(KeyCode::Char(')')), &mut out)
            .unwrap();
        assert_eq!(editor.lines[1], "myfun()");
        assert!(editor.sighelp.is_none());
    }

    #[test]
    fn completion_mql_only_and_word_gated() {
        // Plain text file: no popup even on word chars.
        let mut editor = test_editor(&["hello"]);
        let mut out = stdout_sink();
        editor.mode = Mode::Insert;
        editor.cursor.y = 0;
        editor.cursor.byte = 5;
        editor
            .handle_insert_event(press(KeyCode::Char('x')), &mut out)
            .unwrap();
        assert!(editor.completion.is_none());
        // MQL file, no word under cursor: nothing shows.
        let mut editor = mql_editor(&["int foo;"]);
        editor.cursor.y = 0;
        editor.cursor.byte = 0;
        editor.maybe_trigger_completion();
        assert!(editor.completion.is_none());
    }

    #[test]
    fn sighelp_follows_cursor_into_parens() {
        let mut editor = mql_editor(&["OrderSend(symbol, cmd);"]);
        let mut out = stdout_sink();
        // Move into the parens: sig-help appears with active index.
        editor.cursor.y = 0;
        editor.cursor.byte = 0;
        editor
            .handle_insert_event(press(KeyCode::Right), &mut out)
            .unwrap();
        assert!(editor.sighelp.is_none());
        editor.cursor.byte = 12; // after "OrderSend(sy"
        editor.refresh_sighelp();
        let sig = editor.sighelp.as_ref().expect("sig-help inside parens");
        assert_eq!(sig.label, "OrderSend");
        assert_eq!(sig.active, 0);
        editor.cursor.byte = 18; // after "OrderSend(symbol,"
        editor.refresh_sighelp();
        assert_eq!(editor.sighelp.as_ref().unwrap().active, 1);
    }

    #[test]
    fn sighelp_clears_before_paren_and_past_close() {
        // `|fun()` — cursor left of the call: nothing shows.
        let mut editor = mql_editor(&["void fun(int a, int b){}", "fun()"]);
        editor.cursor.y = 1;
        editor.cursor.byte = 0;
        editor.trigger_function_signature();
        editor.refresh_sighelp();
        assert!(editor.hover_text.is_none());
        assert!(!editor.hover_sig_active);
        assert!(editor.sighelp.is_none());
        // `fun()|` — cursor past the closing paren: stale `int a`
        // must not linger.
        editor.cursor.byte = 5;
        editor.trigger_function_signature();
        editor.refresh_sighelp();
        assert!(editor.hover_text.is_none());
        assert!(!editor.hover_sig_active);
        assert!(editor.sighelp.is_none());
    }

    #[test]
    fn left_arrow_mid_word_dismisses_sighelp_and_repaints() {
        // Inside the parens sig-help + hover are visible; moving left
        // into the name (`fu|n(a, b)`) must clear both AND mark dirty,
        // or the dismissed box survives as ghost pixels.
        let mut editor = mql_editor(&["void fun(int a, int b){}", "fun(a, b)"]);
        let mut out = stdout_sink();
        editor.cursor.y = 1;
        editor.cursor.byte = 5; // after "fun(a"
        editor.refresh_sighelp();
        editor.trigger_function_signature();
        assert!(editor.sighelp.is_some());
        assert!(editor.hover_text.is_some());
        editor.dirty_start = 0;
        editor.dirty_end = 0;
        for _ in 0..3 {
            editor
                .handle_insert_event(press(KeyCode::Left), &mut out)
                .unwrap();
        }
        assert_eq!(editor.cursor.byte, 2); // `fu|n(a, b)`
        assert!(editor.sighelp.is_none());
        assert!(editor.hover_text.is_none());
        assert!(!editor.hover_sig_active);
        assert!(editor.dirty_end > 0, "dismissal must repaint");
    }

    #[test]
    fn sighelp_inside_parens_shows_full_user_signature() {
        // `fun(|` — full `void fun(int a, int b)` signature, first
        // param active in both the sig-help box and the hover line.
        let mut editor = mql_editor(&["void fun(int a, int b){}", "fun("]);
        editor.cursor.y = 1;
        editor.cursor.byte = 4;
        editor.refresh_sighelp();
        let sig = editor.sighelp.as_ref().expect("sig-help inside fun(");
        assert_eq!(sig.label, "fun");
        assert_eq!(sig.ret, "void");
        assert_eq!(sig.params, vec!["int a", "int b"]);
        assert_eq!(sig.active, 0);
        editor.trigger_function_signature();
        assert_eq!(editor.hover_text.as_deref(), Some("a: int"));
        // `fun(x,|` — active index follows the comma.
        let mut editor = mql_editor(&["void fun(int a, int b){}", "fun(x,"]);
        editor.cursor.y = 1;
        editor.cursor.byte = 6;
        editor.refresh_sighelp();
        assert_eq!(editor.sighelp.as_ref().unwrap().active, 1);
        editor.trigger_function_signature();
        assert_eq!(editor.hover_text.as_deref(), Some("b: int"));
    }

    #[test]
    fn sidebar_click_selects_and_opens() {
        use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
        let _cwd =
            crate::utils::test_support::TempCwd::with_fixture(&[("file.txt", "x")], &["sub"]);
        let mut editor = test_editor(&["one"]);
        editor.term_h = 24;
        editor.term_w = 100;
        editor.sidebar.open = true;
        editor.sidebar.scroll = 0;
        assert!(!editor.sidebar.nodes.is_empty());
        let click = |row: u16| MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 1,
            row,
            modifiers: KeyModifiers::empty(),
        };
        // click a directory row toggles expansion (skip the ".." parent)
        let dir_idx = editor
            .sidebar
            .nodes
            .iter()
            .position(|n| n.is_dir && n.name != "..")
            .unwrap();
        let was_expanded = editor.sidebar.nodes[dir_idx].expanded;
        editor.handle_mouse(click(dir_idx as u16));
        assert_eq!(editor.sidebar.selected, dir_idx);
        assert_eq!(editor.sidebar.nodes[dir_idx].expanded, !was_expanded);
        // click a file row opens it in place
        let file_idx = editor.sidebar.nodes.iter().position(|n| !n.is_dir).unwrap();
        let wanted = editor.sidebar.nodes[file_idx].path.clone();
        editor.handle_mouse(click(file_idx as u16));
        assert_eq!(editor.sidebar.selected, file_idx);
        assert_eq!(editor.path, wanted);
    }

    fn mouse_tab_editor() -> (Editor, std::path::PathBuf, std::path::PathBuf) {
        let dir = std::env::temp_dir().join("ntc_editor_mouse_tabs");
        let _ = std::fs::create_dir_all(&dir);
        let path_a = dir.join("alpha.txt");
        let path_b = dir.join("bravo.txt");
        std::fs::write(&path_a, "alpha\nsecond").unwrap();
        std::fs::write(&path_b, "bravo\nline2").unwrap();
        let mut editor = Editor::new(&path_a).unwrap();
        editor.init_tabs();
        editor.open_file(&path_b);
        editor.switch_tab(0);
        editor.term_w = 120;
        editor.term_h = 24;
        editor.sidebar.open = false;
        (editor, path_a, path_b)
    }

    fn mouse_click(column: u16, row: u16) -> crossterm::event::MouseEvent {
        crossterm::event::MouseEvent {
            kind: crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left),
            column,
            row,
            modifiers: KeyModifiers::empty(),
        }
    }

    #[test]
    fn mouse_tab_click_switches_and_preserves_mode() {
        for mode in [Mode::Normal, Mode::Insert, Mode::Visual, Mode::Nfe] {
            let (mut editor, path_a, path_b) = mouse_tab_editor();
            editor.mode = mode.clone();
            editor.sidebar.open = mode == Mode::Nfe;
            assert_eq!(editor.active_tab, 0);

            let zones = editor.tab_hit_zones();
            assert_eq!(zones.len(), 2);
            editor.handle_mouse(mouse_click((zones[1].0 + 1) as u16, 0));
            assert_eq!(editor.active_tab, 1);
            assert_eq!(editor.path, path_b);
            assert_eq!(editor.lines[0], "bravo");
            assert_eq!(editor.mode, mode);

            let zones = editor.tab_hit_zones();
            editor.handle_mouse(mouse_click((zones[0].0 + 1) as u16, 0));
            assert_eq!(editor.active_tab, 0);
            assert_eq!(editor.path, path_a);
            assert_eq!(editor.lines[0], "alpha");
            assert_eq!(editor.mode, mode);
        }
    }

    #[test]
    fn mouse_tab_close_button_closes_clicked_tab() {
        let (mut editor, _, path_b) = mouse_tab_editor();
        editor.mode = Mode::Insert;
        let zones = editor.tab_hit_zones();
        let (_, end) = zones[0];
        editor.handle_mouse(mouse_click((end - 1) as u16, 0));
        assert_eq!(editor.tabs.len(), 1);
        assert_eq!(editor.path, path_b);
        assert_eq!(editor.mode, Mode::Insert);
    }

    #[test]
    fn member_dot_flow_lists_accepts_signals() {
        let mut editor = mql_editor(&[
            "class O {",
            "public:",
            "  O(int a, double b);",
            "  void run(int x);",
            "  int count;",
            "};",
            "O o;",
            "o.",
        ]);
        let mut out = stdout_sink();
        // Cursor after `o.`: member list with methods first.
        editor.cursor.y = 7;
        editor.cursor.byte = 2;
        editor.maybe_trigger_completion();
        let st = editor.completion.as_ref().expect("member list should show");
        let names: Vec<&str> = st.items.iter().map(|i| i.name.as_str()).collect();
        assert!(names.contains(&"O"));
        assert!(names.contains(&"run"));
        assert!(names.contains(&"count"));
        assert_eq!(st.items[0].kind, crate::editor::complete::Kind::Function);
        // Filter by typing + accept the method: sig-help opens.
        editor
            .handle_insert_event(press(KeyCode::Char('r')), &mut out)
            .unwrap();
        {
            let st = editor.completion.as_ref().expect("filtered member list");
            assert!(st
                .items
                .iter()
                .all(|i| i.name.to_lowercase().starts_with('r')));
        }
        editor
            .handle_insert_event(press(KeyCode::Enter), &mut out)
            .unwrap();
        assert_eq!(editor.lines[7], "o.run()");
        let sig = editor
            .sighelp
            .as_ref()
            .expect("method sig-help should open");
        assert_eq!(sig.label, "run");
        assert_eq!(sig.params, vec!["int x"]);
    }

    #[test]
    fn pgup_pgdn_page_selection() {
        let mut editor = mql_editor(&["O"]);
        let mut out = stdout_sink();
        editor.cursor.y = 0;
        editor.cursor.byte = 1;
        editor.maybe_trigger_completion();
        let n = editor.completion.as_ref().unwrap().items.len();
        assert!(n > 0);
        editor
            .handle_insert_event(press(KeyCode::PageDown), &mut out)
            .unwrap();
        assert_eq!(
            editor.completion.as_ref().unwrap().selected,
            (0 + crate::editor::complete::COMPLETE_ROWS) % n
        );
        editor
            .handle_insert_event(press(KeyCode::PageUp), &mut out)
            .unwrap();
        assert_eq!(editor.completion.as_ref().unwrap().selected, 0);
        // Cursor never moved: still on the trigger line.
        assert_eq!((editor.cursor.y, editor.cursor.byte), (0, 1));
    }

    // ── External-change detection (fingerprint poll, headless) ──

    fn watch_dir(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("ntc_watch_{}_{}", tag, std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn external_change_reloads_clean_buffer() {
        let dir = watch_dir("clean");
        let path = dir.join("clean.txt");
        std::fs::write(&path, "one").unwrap();
        let mut editor = Editor::new(&path).unwrap();
        assert!(!editor.modified);
        // Simulate vscode/vim saving externally (different length so the
        // fingerprint differs even at coarse mtime granularity).
        std::fs::write(&path, "one\ntwo").unwrap();
        assert!(editor.check_file_changed());
        assert_eq!(editor.lines, vec!["one".to_string(), "two".to_string()]);
        assert!(!editor.modified);
        assert!(!editor.external_change_pending);
        // Steady state: no further reload.
        assert!(!editor.check_file_changed());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn external_change_keeps_dirty_buffer_and_warns() {
        let dir = watch_dir("dirty");
        let path = dir.join("dirty.txt");
        std::fs::write(&path, "one").unwrap();
        let mut editor = Editor::new(&path).unwrap();
        editor.lines[0] = "edited".to_string();
        editor.modified = true;
        std::fs::write(&path, "one\ndisk").unwrap();
        // Never clobber unsaved edits.
        assert!(!editor.check_file_changed());
        assert_eq!(editor.lines, vec!["edited".to_string()]);
        assert!(editor.external_change_pending);
        assert!(editor.status_msg.is_some());
        // `:e!` force path discards edits and loads disk.
        assert!(editor.reload_from_disk());
        assert_eq!(editor.lines, vec!["one".to_string(), "disk".to_string()]);
        assert!(!editor.modified);
        assert!(!editor.external_change_pending);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn own_save_does_not_trigger_reload() {
        let dir = watch_dir("own");
        let path = dir.join("own.txt");
        std::fs::write(&path, "one").unwrap();
        let mut editor = Editor::new(&path).unwrap();
        editor.lines[0] = "mine".to_string();
        editor.save().unwrap();
        assert!(!editor.check_file_changed());
        assert_eq!(editor.lines, vec!["mine".to_string()]);
        let _ = std::fs::remove_file(&path);
    }
}
