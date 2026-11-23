use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ral.title".into(), "RUN ALIAS COMMANDS:".into());

    m.insert(
        "help.ral.add".into(),
        "ral add <name> \"<cmd>\" ## Create alias".into(),
    );

    m.insert(
        "help.ral.add_param".into(),
        "ral add <name>(x) \"<cmd>\" ## Create parameterised alias with ${x}".into(),
    );

    m.insert(
        "help.ral.env_ref".into(),
        "ral add <name> \"<cmd>\" ## Bare $NAME is an env var at run time; use ${x} for parameters"
            .into(),
    );

    m.insert(
        "help.ral.add_defaults".into(),
        "ral add <name>({x=a,e=txt}) \"<cmd>\" ## Alias with default values".into(),
    );

    m.insert(
        "help.ral.add_array".into(),
        "ral add <name>(x[],y) \"<cmd>\" ## Alias with array params".into(),
    );

    m.insert(
        "help.ral.add_rest".into(),
        "ral add <name>(x) \"echo $*{x}\" ## One arg with spaces (quotes stripped)".into(),
    );

    m.insert(
        "help.ral.edit".into(),
        "ral edit <name> \"<cmd>\" ## Update alias".into(),
    );

    m.insert(
        "help.ral.rnm".into(),
        "ral rnm <old> to <new> ## Rename alias".into(),
    );

    m.insert(
        "help.ral.rm".into(),
        "ral rm <name>[, ...] ## Remove alias(es)".into(),
    );

    m.insert(
        "help.ral.list".into(),
        "ral list ## Show all aliases".into(),
    );

    m.insert(
        "help.ral.info".into(),
        "ral info <name> [--edit] ## Show alias details (--edit: copy edit command to clipboard)"
            .into(),
    );

    m.insert(
        "help.ral.cls".into(),
        "ral cls ## Clear ALL aliases (with confirmation)".into(),
    );

    m.insert(
        "help.ral.export_all".into(),
        "ral export --all <name> ## Export all aliases to <name>.ntc.ral".into(),
    );

    m.insert(
        "help.ral.export_select".into(),
        "ral export --select <name> ## Select aliases to export".into(),
    );

    m.insert(
        "help.ral.import".into(),
        "ral import <file> ## Import aliases from .ntc.ral file".into(),
    );

    m.insert(
        "help.ral.install".into(),
        "ral install ## Install run aliases from the ntc GitHub catalog".into(),
    );

    m.insert(
        "help.ral.make".into(),
        "ral make [name] ## Create alias template in CWD".into(),
    );

    m.insert(
        "help.ral.hintn".into(),
        "ral hintn add|rm|rnm|info|ls ## Manage named value lists for --hint-n/--select-n".into(),
    );

    m.insert(
        "help.ral.example1".into(),
        "ral add build \"cargo build\" ## Create simple alias".into(),
    );

    m.insert(
        "help.ral.example2".into(),
        "ral add greet(x) \"echo Hello ${x}\" ## Parameterised alias".into(),
    );

    m.insert(
        "help.ral.example3".into(),
        "ral add v \"view --care -t -d ${x:-1}\" ## Alias with defaults".into(),
    );

    m.insert(
        "help.ral.example4".into(),
        "ral list ## Show all aliases".into(),
    );

    m.insert(
        "help.ral.example5".into(),
        "ral export --all myaliases ## Export aliases".into(),
    );

    m.insert(
        "help.ral.example6".into(),
        "ral hintn add port [8080,5050] ## Named list for #x --hint-n port".into(),
    );
}
