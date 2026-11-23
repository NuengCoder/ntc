use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("doctor.title".into(), "ntc doctor".into());

    m.insert("doctor.platform".into(), "ntc {} ({}) on {}/{}".into());

    m.insert(
        "doctor.no_config_dir".into(),
        "no config directory is available on this system".into(),
    );

    m.insert(
        "doctor.config_new".into(),
        "config directory not created yet: {} (created on first save)".into(),
    );

    m.insert("doctor.config_ok".into(), "config: {}".into());

    m.insert(
        "doctor.config_unwritable".into(),
        "config directory is not writable: {} ({})".into(),
    );

    m.insert("doctor.settings".into(), "language: {}, theme: {}".into());

    m.insert("doctor.install".into(), "{} on PATH: {}".into());

    m.insert(
        "doctor.install_missing".into(),
        "{} is not on PATH \u{2014} only this build directory can run it".into(),
    );

    m.insert(
        "doctor.ran_none".into(),
        "ran: no ranfile in {} (nothing to run here)".into(),
    );

    m.insert("doctor.ran_ok".into(), "ran: {} with {} target(s)".into());

    m.insert(
        "doctor.ran_env".into(),
        "ran: {} environment variable(s), {} variable(s)".into(),
    );

    m.insert(
        "doctor.ran_problems".into(),
        "ran: {} has {} problem(s) \u{2014} run `{}`".into(),
    );

    m.insert("doctor.ran_error".into(), "ran: {}: {}".into());

    m.insert(
        "doctor.ntpkg_none".into(),
        "ntpkg: no runtimes installed".into(),
    );

    m.insert(
        "doctor.ntpkg_ok".into(),
        "ntpkg: {} version(s) installed, {} active".into(),
    );

    m.insert("doctor.ntpkg_error".into(), "ntpkg: {}: {}".into());

    m.insert(
        "doctor.ntpkg_path".into(),
        "ntpkg: {} active bin dir(s) missing from PATH".into(),
    );

    m.insert("doctor.ok".into(), "no issues found".into());

    m.insert("doctor.issues".into(), "{} issue(s) to look at".into());
}
