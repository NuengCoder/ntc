use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "git.no_git_binary".into(),
        "Git is not installed. Run `ntpkg install git` to install it.".into(),
    );

    m.insert(
        "git.not_repo".into(),
        "Not a git repository (or any of the parent directories).".into(),
    );

    m.insert("git.executing".into(), "Executing: {}".into());

    m.insert("git.completed".into(), "git command completed.".into());

    m.insert(
        "git.exit_code".into(),
        "Command exited with code: {}".into(),
    );

    m.insert("git.terminated".into(), "Command terminated.".into());

    m.insert("git.failed".into(), "Failed to execute command: {}".into());

    m.insert("git.checking_status".into(), "Checking status".into());

    m.insert(
        "git.status_failed".into(),
        "Could not read git status.".into(),
    );

    m.insert("git.branch_header".into(), "On branch".into());

    m.insert("git.upstream".into(), "tracking {}".into());

    m.insert("git.ahead".into(), "ahead {}".into());

    m.insert("git.behind".into(), "behind {}".into());

    m.insert("git.detached_header".into(), "Detached HEAD".into());

    m.insert("git.section_conflicts".into(), "Unmerged paths:".into());

    m.insert(
        "git.section_staged".into(),
        "Changes to be committed:".into(),
    );

    m.insert(
        "git.section_unstaged".into(),
        "Changes not staged for commit:".into(),
    );

    m.insert("git.section_untracked".into(), "Untracked files:".into());

    m.insert("git.clean".into(), "Working tree clean.".into());

    m.insert(
        "git.branch_failed".into(),
        "Could not list branches.".into(),
    );

    m.insert("git.log_failed".into(), "Could not read git log.".into());

    m.insert("git.log_empty".into(), "No commits found.".into());

    m.insert("git.help_title".into(), "GIT INTEGRATION:".into());

    m.insert(
        "git.help_status".into(),
        "git status ## Show branch + staged/unstaged/untracked".into(),
    );

    m.insert(
        "git.help_branch".into(),
        "git branch ## List branches".into(),
    );

    m.insert(
        "git.help_log".into(),
        "git log ## Recent colored commits".into(),
    );

    m.insert(
        "git.help_diff".into(),
        "git diff ## Working tree diff".into(),
    );

    m.insert(
        "git.help_passthrough".into(),
        "git <any> ## Everything else passes to real git".into(),
    );

    m.insert("git.examples_header".into(), "EXAMPLES:".into());

    m.insert("git.help_example1".into(), "git status".into());

    m.insert("git.help_example2".into(), "git checkout main".into());
}
