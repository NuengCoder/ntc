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
}
