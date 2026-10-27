use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.ntpkg.available".into(),
        "ntpkg available [lang] ## List languages or downloadable versions".into(),
    );

    m.insert(
        "help.ntpkg.help".into(),
        "ntpkg help ## Show ntpkg usage".into(),
    );

    m.insert(
        "help.ntpkg.info".into(),
        "ntpkg info <lang> ## Show package info (version, location, binaries)".into(),
    );

    m.insert(
        "help.ntpkg.install".into(),
        "ntpkg install <lang> [ver] ## Download & install a runtime".into(),
    );

    m.insert(
        "help.ntpkg.list".into(),
        "ntpkg list ## Show installed runtimes".into(),
    );

    m.insert(
        "help.ntpkg.ntpx".into(),
        "ntpx <pkg> [bin] [args] ## Run an installed package's binary".into(),
    );

    m.insert(
        "help.ntpkg.search".into(),
        "ntpkg search [keyword] ## Search available packages".into(),
    );

    m.insert(
        "help.ntpkg.title".into(),
        "PACKAGE MANAGER (ntpkg) + ntpx:".into(),
    );

    m.insert(
        "help.ntpkg.uninstall".into(),
        "ntpkg uninstall <lang> [ver] ## Remove an installed runtime".into(),
    );

    m.insert(
        "help.ntpkg.use".into(),
        "ntpkg use <lang> <ver> ## Activate a specific version".into(),
    );

    m.insert(
        "help.ntpkg.which".into(),
        "ntpkg which <lang> ## Show path to active runtime".into(),
    );

    m.insert(
        "help.ntpkg.example1".into(),
        "ntpkg install node 18 ## Install Node.js 18".into(),
    );

    m.insert(
        "help.ntpkg.example2".into(),
        "ntpkg list ## Show installed runtimes".into(),
    );

    m.insert(
        "help.ntpkg.example3".into(),
        "ntpkg use node 18 ## Activate Node.js 18".into(),
    );

    m.insert(
        "help.ntpkg.example4".into(),
        "ntpkg which node ## Show path to Node.js".into(),
    );

    m.insert(
        "help.ntpkg.example5".into(),
        "ntpkg search python ## Search for Python packages".into(),
    );

    m.insert(
        "help.ntpkg.example6".into(),
        "ntpkg info node ## Show Node.js package info".into(),
    );

    m.insert(
        "help.ntpkg.example7".into(),
        "ntpkg available ## List downloadable languages".into(),
    );

    m.insert(
        "help.ntpkg.example8".into(),
        "ntpx node npm init ## Run npm via ntpx".into(),
    );

    m.insert(
        "help.ntpkg.example9".into(),
        "ntpkg help ## Show ntpkg usage".into(),
    );
}
