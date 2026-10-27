use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "qsc.macos_listen_hint".into(),
        "QSC: Run 'qsc listen' once support is added.".into(),
    );

    m.insert(
        "qsc.macos_not_implemented".into(),
        "QSC: macOS global hotkey support not yet implemented.".into(),
    );

    m.insert(
        "qsc.unsupported_platform".into(),
        "QSC: unsupported platform".into(),
    );
}
