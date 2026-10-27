use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.mode.admin_relaunch".into(),
        "Re-launching ntc as Administrator...".into(),
    );

    m.insert(
        "cmd.mode.elevate_fail".into(),
        "Failed to launch elevated process: {}".into(),
    );

    m.insert(
        "cmd.mode.exe_fail".into(),
        "Failed to get executable path: {}".into(),
    );

    m.insert(
        "cmd.mode.launch_fail".into(),
        "Failed to launch process: {}".into(),
    );

    m.insert(
        "cmd.mode.normal_relaunch".into(),
        "Re-launching ntc in normal mode...".into(),
    );

    m.insert(
        "cmd.mode.windows_only".into(),
        "mode is a Windows-only feature.".into(),
    );
}
