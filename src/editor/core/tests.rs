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
}
