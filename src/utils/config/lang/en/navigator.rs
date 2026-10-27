use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "navigator.already_root".into(),
        "Already at root, cannot go back further".into(),
    );

    m.insert(
        "navigator.cannot_navigate_to".into(),
        "Cannot navigate to: {}".into(),
    );

    m.insert(
        "navigator.cannot_read_dir".into(),
        "Cannot read directory: {}".into(),
    );

    m.insert(
        "navigator.drive_windows_only".into(),
        "Drive navigation is only supported on Windows".into(),
    );

    m.insert(
        "navigator.not_a_directory".into(),
        "{} is not a directory".into(),
    );

    m.insert(
        "navigator.venv_activate_unix".into(),
        "   Run: source ./venv/bin/activate".into(),
    );

    m.insert(
        "navigator.venv_activate_win".into(),
        "   Run: .\\\\\\\\venv\\\\\\\\Scripts\\\\\\\\activate".into(),
    );

    m.insert(
        "navigator.venv_tip".into(),
        "   Run: source ./venv/bin/activate".into(),
    );
}
