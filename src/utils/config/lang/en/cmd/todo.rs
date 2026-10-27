use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.todo.add_usage".into(),
        "Usage: todo add \"task name\" [-> N]".into(),
    );

    m.insert("cmd.todo.all_cleared".into(), "All tasks cleared.".into());

    m.insert("cmd.todo.cancelled".into(), "Cancelled.".into());

    m.insert(
        "cmd.todo.clear_prompt".into(),
        "Clear all {} tasks? [y/N] ".into(),
    );

    m.insert(
        "cmd.todo.empty_name".into(),
        "Task name cannot be empty".into(),
    );

    m.insert(
        "cmd.todo.help_add".into(),
        "  todo add \\\"name\\\" [-> N]  Add task (optionally at position N)".into(),
    );

    m.insert(
        "cmd.todo.help_clear".into(),
        "  todo clear              Clear all tasks (with confirmation)".into(),
    );

    m.insert(
        "cmd.todo.help_info".into(),
        "  todo info [N]           Show all tasks or task N".into(),
    );

    m.insert(
        "cmd.todo.help_rm".into(),
        "  todo rm <N>             Remove task N".into(),
    );

    m.insert(
        "cmd.todo.help_show".into(),
        "  todo                    Show all tasks".into(),
    );

    m.insert(
        "cmd.todo.info_usage".into(),
        "Usage: todo info <N>  — N must be a number".into(),
    );

    m.insert(
        "cmd.todo.invalid_position".into(),
        "Invalid position: '{}' must be a number".into(),
    );

    m.insert("cmd.todo.list_title".into(), "TODO LIST:".into());

    m.insert(
        "cmd.todo.missing_position".into(),
        "Missing position number after '->'".into(),
    );

    m.insert(
        "cmd.todo.missing_quote".into(),
        "Missing closing quote".into(),
    );

    m.insert(
        "cmd.todo.must_be_quoted".into(),
        "Task name must be in quotes".into(),
    );

    m.insert("cmd.todo.no_tasks".into(), "  {}".into());

    m.insert("cmd.todo.none_to_clear".into(), "No tasks to clear.".into());

    m.insert(
        "cmd.todo.one_based".into(),
        "Task numbers are 1-based (minimum 1)".into(),
    );

    m.insert(
        "cmd.todo.position_one_based".into(),
        "Position must be 1-based (minimum 1)".into(),
    );

    m.insert("cmd.todo.rm_usage".into(), "Usage: todo rm <N>".into());

    m.insert(
        "cmd.todo.rm_usage_num".into(),
        "Usage: todo rm <N>  — N must be a number".into(),
    );

    m.insert("cmd.todo.task_added".into(), "Task added: {}".into());

    m.insert(
        "cmd.todo.task_not_found".into(),
        "Task #{} not found (have {} tasks)".into(),
    );

    m.insert("cmd.todo.task_removed".into(), "Task removed: {}".into());

    m.insert(
        "cmd.todo.unexpected_text".into(),
        "Unexpected text after task name: '{}'".into(),
    );

    m.insert(
        "cmd.todo.unknown_sub".into(),
        "Unknown todo subcommand: '{}'".into(),
    );

    m.insert("cmd.todo.usage_title".into(), "Usage:".into());

    m.insert(
        "cmd.todo.help_format".into(),
        "  config todo format <fmt>  Set todo file format (txt|md|json|toml|yaml|html)".into(),
    );

    m.insert(
        "cmd.todo.unknown_format".into(),
        "Unknown todo format: '{}' (use txt|md|json|toml|yaml|html)".into(),
    );

    m.insert(
        "cmd.todo.parse_failed".into(),
        "Failed to parse {} todo file.".into(),
    );

    m.insert(
        "cmd.todo.help_done".into(),
        "  todo done <N>           Mark task N as done".into(),
    );

    m.insert(
        "cmd.todo.help_undo".into(),
        "  todo undo <N>           Mark task N as not done".into(),
    );

    m.insert(
        "cmd.todo.help_edit".into(),
        "  todo edit <N> \"name\"    Rename task N".into(),
    );

    m.insert("cmd.todo.done_usage".into(), "Usage: todo done <N>".into());

    m.insert("cmd.todo.undo_usage".into(), "Usage: todo undo <N>".into());

    m.insert(
        "cmd.todo.edit_usage".into(),
        "Usage: todo edit <N> \"new name\"".into(),
    );

    m.insert("cmd.todo.task_done".into(), "Task done: {}".into());

    m.insert("cmd.todo.task_undone".into(), "Task reopened: {}".into());

    m.insert("cmd.todo.task_edited".into(), "Task updated: {}".into());

    m.insert(
        "cmd.todo.already_done".into(),
        "Task #{} is already done".into(),
    );

    m.insert(
        "cmd.todo.already_open".into(),
        "Task #{} is already open".into(),
    );
}
