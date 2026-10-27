use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.fake.title".into(),
        "fake - generate fake/random data".into(),
    );

    m.insert(
        "help.fake.gen".into(),
        "  fake gen <types> [-n N] [-f fmt] [-s seed] [-o file]   Generate rows".into(),
    );

    m.insert(
        "help.fake.gen_options".into(),
        "  formats: json | txt | sql | csv | xlsx  (default: txt table)".into(),
    );

    m.insert(
        "help.fake.gen_group".into(),
        "  fake gen -g <group> ...                                Generate from a saved group"
            .into(),
    );

    m.insert(
        "help.fake.gen_unique".into(),
        "  -u, --unique [cols]                                    Unique rows or named columns"
            .into(),
    );

    m.insert("help.fake.gen_locale".into(), "  -l, --locale <code>                                    Locale pools (en_US en_GB zh_CN th_TH lo_LA)".into());

    m.insert(
        "help.fake.types".into(),
        "  fake types                                             List every type".into(),
    );

    m.insert(
        "help.fake.types_search".into(),
        "  fake types -s <pattern>                                Fuzzy-search types".into(),
    );

    m.insert(
        "help.fake.locales".into(),
        "  fake locales                                           List --locale codes".into(),
    );

    m.insert(
        "help.fake.group_add".into(),
        "  fake group add <name> [alias as type, ...]             Create a group".into(),
    );

    m.insert(
        "help.fake.group_edit".into(),
        "  fake group edit <name> [alias as type, ...]            Replace a group's columns".into(),
    );

    m.insert(
        "help.fake.group_rm".into(),
        "  fake group rm <name>                                   Delete a group".into(),
    );

    m.insert(
        "help.fake.group_rnm".into(),
        "  fake group rnm <old> <new>                             Rename a group".into(),
    );

    m.insert(
        "help.fake.group_ls".into(),
        "  fake group ls [pattern]                                List groups (glob, e.g. u*)"
            .into(),
    );

    m.insert(
        "help.fake.group_info".into(),
        "  fake group info <name>                                 Show a group's columns".into(),
    );

    m.insert(
        "help.fake.group_clear".into(),
        "  fake group clear [-f]                                  Delete every group".into(),
    );

    m.insert(
        "help.fake.group_export".into(),
        "  fake group export <name>|--all [-o file]               Share groups as .ntc_fake".into(),
    );

    m.insert(
        "help.fake.group_import".into(),
        "  fake group import <file> [--force] [--prefix p]        Load groups from a file".into(),
    );

    m.insert(
        "help.fake.group_ref".into(),
        "  columns: 'alias as type', 'type !' = unique, 'alias as ref:group.col'".into(),
    );

    m.insert(
        "help.fake.help".into(),
        "  fake help                                              Show this help".into(),
    );

    m.insert(
        "help.fake.example1".into(),
        "fake gen name,email,city -n 5 ## txt table".into(),
    );

    m.insert(
        "help.fake.example2".into(),
        "fake gen uuid,age -f json -o rows.json".into(),
    );

    m.insert(
        "help.fake.example3".into(),
        "fake group add users [username as name, year as age]".into(),
    );

    m.insert(
        "help.fake.example4".into(),
        "fake gen -g users -f sql -n 10 ## INSERT INTO users ...".into(),
    );
}
