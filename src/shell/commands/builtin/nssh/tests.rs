use super::cmd_nssh;
use super::copy::{expand_remote_path, split_remote};
use super::tokenize;
use crate::navigator::Navigator;

#[test]
fn tokenize_quotes() {
    assert_eq!(
        tokenize(r#"ssh prod --cmd "uptime; df -h""#),
        vec!["ssh", "prod", "--cmd", "uptime; df -h"]
    );
}

#[test]
fn split_remote_needs_known_profile() {
    // No profiles in a fresh temp HOME... store reads real config dir;
    // only assert Windows-drive safety which needs no store entry.
    assert!(split_remote(r"C:\temp\x").is_none());
    assert!(split_remote("C:/temp/x").is_none());
    assert!(split_remote("./local/file").is_none());
    assert!(split_remote("no-colon-here").is_none());
}

#[test]
fn expand_remote_dir_appends_basename() {
    assert_eq!(
        expand_remote_path("/tmp/", Some("app.tar.gz")),
        "/tmp/app.tar.gz"
    );
    assert_eq!(expand_remote_path("/tmp/x.tgz", Some("a")), "/tmp/x.tgz");
}

#[test]
fn bare_name_routes_to_ssh() {
    // Unknown profile prints an error and returns Ok(false).
    // Isolated config home (empty store) so a real user profile on the
    // dev machine / CI runner cannot leak in; also serializes CWD/env
    // access with parallel tests.
    let _home = crate::utils::test_support::IsolatedConfigHome::new();
    let mut nav = Navigator::new().unwrap();
    let r = cmd_nssh("definitely-not-a-profile-xyz", &mut nav).unwrap();
    assert!(!r);
}
