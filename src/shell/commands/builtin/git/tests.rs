use super::*;

#[test]
fn branch_with_args_reaches_real_git() {
    // Regression: `git branch -M main` used to hit the pretty-list
    // branch and silently drop its args, so the rename never ran.
    if !gitmod::git_available() {
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let run = |args: &[&str]| {
        std::process::Command::new("git")
            .args(args)
            .current_dir(dir.path())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    };
    if !run(&["init", "-b", "master"]) {
        return;
    }
    // NOTE: intentionally uses `for_test` instead of `go_to` — `go_to`
    // calls `set_current_dir`, which is process-global and used to move
    // the CWD out from under parallel hinter/helpers tests on CI.
    let mut nav = Navigator::for_test(dir.path());
    cmd_git("branch -M main", &mut nav).unwrap();
    let head =
        gitmod::git_output(dir.path(), &["symbolic-ref", "--short", "HEAD"]).unwrap_or_default();
    assert_eq!(head.trim(), "main");
    // Bare `git branch` still renders the list without side effects.
    cmd_git("branch", &mut nav).unwrap();
    let head =
        gitmod::git_output(dir.path(), &["symbolic-ref", "--short", "HEAD"]).unwrap_or_default();
    assert_eq!(head.trim(), "main");
}
