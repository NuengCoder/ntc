use super::actions::*;
use super::model::*;

#[test]
fn parse_legacy_bare_line() {
    let t = parse_line("Fix bug").unwrap();
    assert!(!t.done);
    assert_eq!(t.priority, 0);
    assert_eq!(t.text, "Fix bug");
}

#[test]
fn parse_checkbox_and_priority() {
    assert_eq!(
        parse_line("- [ ] Write docs !!").unwrap(),
        Task {
            done: false,
            priority: 2,
            text: "Write docs".into()
        }
    );
    assert_eq!(
        parse_line("- [x] Old !").unwrap(),
        Task {
            done: true,
            priority: 1,
            text: "Old".into()
        }
    );
    assert!(parse_line("- [X] Caps").unwrap().done);
}

#[test]
fn format_roundtrip() {
    let task = Task {
        done: false,
        priority: 2,
        text: "Ship".into(),
    };
    assert_eq!(format_line(&task), "- [ ] Ship !!");
    assert_eq!(parse_line(&format_line(&task)).unwrap(), task);
}

#[test]
fn add_parses_priority_and_position() {
    let (task, pos) = parse_add_args("\"Fix bug !!\" -> 2").unwrap();
    assert_eq!(task.text, "Fix bug");
    assert_eq!(task.priority, 2);
    assert_eq!(pos, Some(2));
    let (task, pos) = parse_add_args("\"Plain\"").unwrap();
    assert_eq!(task.priority, 0);
    assert_eq!(pos, None);
}

#[test]
fn done_undo_toggle() {
    let dir = tempfile::tempdir().unwrap();
    save_tasks(
        dir.path(),
        &[Task {
            done: false,
            priority: 0,
            text: "A".into(),
        }],
    )
    .unwrap();
    cmd_done("1", dir.path()).unwrap();
    let tasks = load_tasks(dir.path()).unwrap();
    assert!(tasks[0].done);
    cmd_undo("1", dir.path()).unwrap();
    let tasks = load_tasks(dir.path()).unwrap();
    assert!(!tasks[0].done);
}

#[test]
fn edit_renames_and_reprioritizes() {
    let dir = tempfile::tempdir().unwrap();
    save_tasks(
        dir.path(),
        &[Task {
            done: false,
            priority: 0,
            text: "Old".into(),
        }],
    )
    .unwrap();
    cmd_edit("1 \"New !!\"", dir.path()).unwrap();
    let tasks = load_tasks(dir.path()).unwrap();
    assert_eq!(tasks[0].text, "New");
    assert_eq!(tasks[0].priority, 2);
}

fn sample_tasks() -> Vec<Task> {
    vec![
        Task {
            done: false,
            priority: 0,
            text: "plain".into(),
        },
        Task {
            done: true,
            priority: 2,
            text: "urgent & <done>".into(),
        },
    ]
}

#[test]
fn roundtrip_all_formats() {
    for format in ["txt", "md", "json", "toml", "yaml", "html"] {
        let dir = tempfile::tempdir().unwrap();
        save_tasks_with(dir.path(), format, &sample_tasks()).unwrap();
        let loaded = load_tasks_with(dir.path(), format).unwrap();
        assert_eq!(loaded, sample_tasks(), "format={format}");
    }
}

#[test]
fn empty_documents_reload_as_empty() {
    for format in ["txt", "md", "json", "toml", "yaml", "html"] {
        let dir = tempfile::tempdir().unwrap();
        save_tasks_with(dir.path(), format, &[]).unwrap();
        let loaded = load_tasks_with(dir.path(), format).unwrap();
        assert!(loaded.is_empty(), "format={format}");
    }
}

#[test]
fn migrates_legacy_txt_once() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("future_todo.txt"),
        "- [ ] legacy\n- [x] old !!\n",
    )
    .unwrap();
    let tasks = load_tasks_migrating(dir.path(), "md").unwrap();
    assert_eq!(tasks.len(), 2);
    assert!(tasks[0].text == "legacy");
    assert!(dir.path().join("future_todo.md").exists());
    // Second load reads the migrated file, not the legacy one.
    let again = load_tasks_migrating(dir.path(), "md").unwrap();
    assert_eq!(again, tasks);
}

#[test]
fn move_and_swap_reorder_like_the_examples() {
    let mut tasks: Vec<Task> = ["a", "b", "c", "d", "e"]
        .iter()
        .map(|s| Task {
            done: false,
            priority: 0,
            text: (*s).into(),
        })
        .collect();
    move_task(&mut tasks, 5, 1).unwrap();
    let order: Vec<&str> = tasks.iter().map(|t| t.text.as_str()).collect();
    assert_eq!(order, vec!["e", "a", "b", "c", "d"]);

    let mut tasks: Vec<Task> = ["a", "b", "c", "d", "e"]
        .iter()
        .map(|s| Task {
            done: false,
            priority: 0,
            text: (*s).into(),
        })
        .collect();
    swap_task(&mut tasks, 5, 1).unwrap();
    let order: Vec<&str> = tasks.iter().map(|t| t.text.as_str()).collect();
    assert_eq!(order, vec!["e", "b", "c", "d", "a"]);

    // Middle moves keep the relative order of everyone else.
    let mut tasks: Vec<Task> = ["a", "b", "c", "d"]
        .iter()
        .map(|s| Task {
            done: false,
            priority: 0,
            text: (*s).into(),
        })
        .collect();
    move_task(&mut tasks, 2, 3).unwrap();
    let order: Vec<&str> = tasks.iter().map(|t| t.text.as_str()).collect();
    assert_eq!(order, vec!["a", "c", "b", "d"]);

    // Same index is a no-op, out-of-range is rejected.
    move_task(&mut tasks, 2, 2).unwrap();
    assert!(move_task(&mut tasks, 0, 1).is_err());
    assert!(move_task(&mut tasks, 1, 9).is_err());
    assert!(swap_task(&mut tasks, 1, 9).is_err());
}

#[test]
fn pair_parser_rejects_bad_input() {
    assert_eq!(parse_pair("1 5", "cmd.todo.move_usage").unwrap(), (1, 5));
    assert_eq!(
        parse_pair("  5   1  ", "cmd.todo.move_usage").unwrap(),
        (5, 1)
    );
    assert!(parse_pair("", "cmd.todo.move_usage").is_err());
    assert!(parse_pair("5", "cmd.todo.move_usage").is_err());
    assert!(parse_pair("0 1", "cmd.todo.move_usage").is_err());
    assert!(parse_pair("1 x", "cmd.todo.move_usage").is_err());
    assert!(parse_pair("1 2 3", "cmd.todo.move_usage").is_err());
}

#[test]
fn move_and_replace_roundtrip_through_a_file() {
    let dir = tempfile::tempdir().unwrap();
    let tasks: Vec<Task> = ["one", "two", "three"]
        .iter()
        .map(|s| Task {
            done: false,
            priority: 0,
            text: (*s).into(),
        })
        .collect();
    save_tasks(dir.path(), &tasks).unwrap();

    cmd_move("3 1", dir.path()).unwrap();
    let order: Vec<String> = load_tasks(dir.path())
        .unwrap()
        .into_iter()
        .map(|t| t.text)
        .collect();
    assert_eq!(order, vec!["three", "one", "two"]);

    cmd_replace("1 3", dir.path()).unwrap();
    let order: Vec<String> = load_tasks(dir.path())
        .unwrap()
        .into_iter()
        .map(|t| t.text)
        .collect();
    assert_eq!(order, vec!["two", "one", "three"]);

    // Nothing changed when the indices are equal or out of range.
    cmd_move("2 2", dir.path()).unwrap();
    cmd_replace("9 1", dir.path()).unwrap();
    let order: Vec<String> = load_tasks(dir.path())
        .unwrap()
        .into_iter()
        .map(|t| t.text)
        .collect();
    assert_eq!(order, vec!["two", "one", "three"]);
}
