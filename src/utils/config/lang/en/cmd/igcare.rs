use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.igcare.create_failed".into(),
        "Failed to create file: {}".into(),
    );

    m.insert(
        "cmd.igcare.example_export_all".into(),
        "  igcare export --all myproject        # Creates myproject.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.example_export_select".into(),
        "  igcare export --select myproject     # Pick which categories to export".into(),
    );

    m.insert(
        "cmd.igcare.example_import".into(),
        "  igcare import myproject.ntc.igcare   # Import into current config".into(),
    );

    m.insert("cmd.igcare.examples_title".into(), "Examples:".into());

    m.insert(
        "cmd.igcare.export_cmd".into(),
        "igcare export --all <name> Export all settings to <name>.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.export_name_empty".into(),
        "Export name cannot be empty.".into(),
    );

    m.insert(
        "cmd.igcare.export_select_cmd".into(),
        "igcare export --select <name> Select categories to export to <name>.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.export_usage".into(),
        "Usage: igcare export --all <name> | igcare export --select <name>".into(),
    );

    m.insert(
        "cmd.igcare.file_exists".into(),
        "File already exists: {}".into(),
    );

    m.insert(
        "cmd.igcare.import_cmd".into(),
        "igcare import <file> Import settings from a .ntc.igcare file".into(),
    );

    m.insert(
        "cmd.igcare.import_example".into(),
        "Example: igcare import myproject.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.import_usage".into(),
        "Usage: igcare import <file>".into(),
    );

    m.insert("cmd.igcare.main_title".into(), "Ignore/Care Manager".into());

    m.insert(
        "cmd.igcare.template_created".into(),
        "Created igcare template: {}".into(),
    );

    m.insert(
        "cmd.igcare.template_hint".into(),
        "This is a local template file. Use 'igcare import' to apply settings.".into(),
    );

    m.insert(
        "cmd.igcare.type_help".into(),
        "Type 'igcare' for help.".into(),
    );

    m.insert(
        "cmd.igcare.unknown_flag".into(),
        "Unknown export flag: {}. Use --all or --select".into(),
    );

    m.insert(
        "cmd.igcare.unknown_subcmd".into(),
        "Unknown igcare subcommand: {}".into(),
    );
}
