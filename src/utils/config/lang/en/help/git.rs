use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.git.title".into(), "GIT INTEGRATION COMMANDS:".into());

    m.insert(
        "help.git.status".into(),
        "git status | st ## Pretty-printed repo status".into(),
    );

    m.insert(
        "help.git.branch".into(),
        "git branch | b ## List branches (current highlighted)".into(),
    );

    m.insert(
        "help.git.log".into(),
        "git log | l ## Colored recent commit log".into(),
    );

    m.insert(
        "help.git.diff".into(),
        "git diff ## Show working tree diff".into(),
    );

    m.insert(
        "help.git.passthrough".into(),
        "git <any> ## All other git subcommands pass through".into(),
    );

    m.insert(
        "help.git.example1".into(),
        "git status ## Show branch + staged/unstaged/untracked".into(),
    );

    m.insert(
        "help.git.example2".into(),
        "git branch -a ## List all branches (passthrough)".into(),
    );

    m.insert(
        "help.git.example3".into(),
        "git commit -am \"msg\" ## Normal git flow (passthrough)".into(),
    );
}
