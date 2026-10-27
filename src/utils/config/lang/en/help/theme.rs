use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.theme.add".into(),
        "theme add <name> ## Create a new theme (opens in editor)".into(),
    );

    m.insert(
        "help.theme.current".into(),
        "theme current ## Show current theme name and details".into(),
    );

    m.insert(
        "help.theme.edit".into(),
        "theme edit [name] ## Edit a theme (or open theme directory)".into(),
    );

    m.insert(
        "help.theme.export".into(),
        "theme export <name> ## Export theme to .ntc_theme file".into(),
    );

    m.insert(
        "help.theme.import".into(),
        "theme import <file> ## Import theme from .ntc_theme file".into(),
    );

    m.insert(
        "help.theme.info".into(),
        "theme info <name> ## Show theme metadata and color counts".into(),
    );

    m.insert(
        "help.theme.interactive".into(),
        "theme ## Interactive theme manager (menu)".into(),
    );

    m.insert(
        "help.theme.list".into(),
        "theme list ## List all available themes".into(),
    );

    m.insert(
        "help.theme.make".into(),
        "theme make [name] ## Create theme template in CWD".into(),
    );

    m.insert(
        "help.theme.reload".into(),
        "theme reload ## Reload themes from disk".into(),
    );

    m.insert(
        "help.theme.rm".into(),
        "theme rm <name> ## Remove a theme".into(),
    );

    m.insert(
        "help.theme.rnm".into(),
        "theme rnm <old> to <new> ## Rename a theme".into(),
    );

    m.insert(
        "help.theme.switch".into(),
        "theme <name> ## Switch to a theme by name".into(),
    );

    m.insert("help.theme.title".into(), "THEME COMMANDS:".into());

    m.insert(
        "help.theme.where".into(),
        "theme where ## Show theme directory location".into(),
    );

    m.insert(
        "help.theme.example1".into(),
        "theme list ## List all available themes".into(),
    );

    m.insert(
        "help.theme.example2".into(),
        "theme dark ## Switch to 'dark' theme".into(),
    );

    m.insert(
        "help.theme.example3".into(),
        "theme current ## Show current theme info".into(),
    );

    m.insert(
        "help.theme.example4".into(),
        "theme add mytheme ## Create a new theme".into(),
    );
}
