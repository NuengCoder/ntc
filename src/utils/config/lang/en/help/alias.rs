use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.alias.add".into(),
        "ral add <name> \"<cmd>\" ## Create alias".into(),
    );

    m.insert(
        "help.alias.add_array".into(),
        "ral add <name>(x[],y) \"<cmd>\" ## Alias with array params".into(),
    );

    m.insert(
        "help.alias.add_rest".into(),
        "ral add <name>(x) \"echo $*{x}\" ## One arg with spaces (quotes stripped)".into(),
    );

    m.insert(
        "help.alias.add_defaults".into(),
        "ral add <name>({x=a,e=txt}) \"<cmd>\" ## Alias with default values".into(),
    );

    m.insert(
        "help.alias.add_param".into(),
        "ral add <name>(x) \"<cmd>\" ## Create parameterised alias with $x".into(),
    );

    m.insert(
        "help.alias.cls".into(),
        "ral cls ## Clear ALL aliases (with confirmation)".into(),
    );

    m.insert(
        "help.alias.edit".into(),
        "ral edit <name> \"<cmd>\" ## Update alias".into(),
    );

    m.insert(
        "help.alias.export_all".into(),
        "ral export --all <name> ## Export all aliases to <name>.ntc.ral".into(),
    );

    m.insert(
        "help.alias.export_select".into(),
        "ral export --select <name> ## Select aliases to export".into(),
    );

    m.insert(
        "help.alias.import".into(),
        "ral import <file> ## Import aliases from .ntc.ral file".into(),
    );

    m.insert(
        "help.alias.info".into(),
        "ral info <name> [--edit] ## Show alias details (--edit: copy edit command to clipboard)"
            .into(),
    );

    m.insert(
        "help.alias.list".into(),
        "ral list ## Show all aliases".into(),
    );

    m.insert(
        "help.alias.make".into(),
        "ral make [name] ## Create alias template in CWD".into(),
    );

    m.insert(
        "help.alias.hintn".into(),
        "ral hintn add|rm|rnm|info|ls ## Manage named value lists for --hint-n/--select-n".into(),
    );

    m.insert(
        "help.alias.rm".into(),
        "ral rm <name>[, ...] ## Remove alias(es)".into(),
    );

    m.insert(
        "help.alias.rnm".into(),
        "ral rnm <old> to <new> ## Rename alias".into(),
    );

    m.insert(
        "help.alias.run".into(),
        "run <alias> ## Execute alias with 'run'".into(),
    );

    m.insert(
        "help.alias.run_direct".into(),
        "<alias> ## Execute alias directly".into(),
    );

    m.insert(
        "help.alias.run_param".into(),
        "<alias>(<arg>) ## Execute parameterised alias".into(),
    );

    m.insert("help.alias.title".into(), "RUN ALIAS COMMANDS:".into());

    m.insert(
        "help.alias.example1".into(),
        "ral add build \"cargo build\" ## Create simple alias".into(),
    );

    m.insert(
        "help.alias.example2".into(),
        "ral add greet(x) \"echo Hello $x\" ## Parameterised alias".into(),
    );

    m.insert(
        "help.alias.example3".into(),
        "ral add v \"view --care -t -d ${x:-1}\" ## Alias with defaults".into(),
    );

    m.insert(
        "help.alias.example4".into(),
        "ral list ## Show all aliases".into(),
    );

    m.insert(
        "help.alias.example5".into(),
        "ral export --all myaliases ## Export aliases".into(),
    );

    m.insert(
        "help.alias.example6".into(),
        "build ## Run alias directly".into(),
    );

    m.insert(
        "help.alias.example7".into(),
        "ral hintn add port [8080,5050] ## Named list for #x --hint-n port".into(),
    );
}
