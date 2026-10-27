use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.todo.add".into(),
        "todo add \"name\" ## Append a new task".into(),
    );

    m.insert(
        "help.todo.add_at".into(),
        "todo add \"name\" -> N ## Insert task at position N".into(),
    );

    m.insert(
        "help.todo.clear".into(),
        "todo clear ## Clear ALL tasks (with confirmation)".into(),
    );

    m.insert(
        "help.todo.info".into(),
        "todo info [N] ## Show all tasks or details of task N".into(),
    );

    m.insert("help.todo.rm".into(), "todo rm <N> ## Remove task N".into());

    m.insert(
        "help.todo.show".into(),
        "todo ## Show all tasks (from future_todo.<fmt> in CWD; set with config todo format)"
            .into(),
    );

    m.insert("help.todo.title".into(), "TODO COMMANDS:".into());

    m.insert(
        "help.todo.done".into(),
        "todo done <N> ## Mark task N as done".into(),
    );

    m.insert(
        "help.todo.undo".into(),
        "todo undo <N> ## Reopen task N".into(),
    );

    m.insert(
        "help.todo.edit".into(),
        "todo edit <N> \"name\" ## Rename task N".into(),
    );

    m.insert("help.todo.example1".into(), "todo ## Show all tasks".into());

    m.insert(
        "help.todo.example2".into(),
        "todo add \"Fix bug\" ## Add a new task".into(),
    );

    m.insert(
        "help.todo.example3".into(),
        "todo rm 1 ## Remove task #1".into(),
    );
}
