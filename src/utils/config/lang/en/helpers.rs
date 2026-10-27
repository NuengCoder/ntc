use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "helpers.building_tree".into(),
        "Building directory tree...".into(),
    );

    m.insert(
        "helpers.calculating_sizes".into(),
        "Calculating directory sizes...".into(),
    );

    m.insert(
        "helpers.export_cancelled".into(),
        "Export cancelled.".into(),
    );

    m.insert(
        "helpers.exported_all".into(),
        "Exported all {} alias(es) to: {}".into(),
    );

    m.insert(
        "helpers.exported_categories".into(),
        "Exported {} category(ies) to: {}".into(),
    );

    m.insert(
        "helpers.exported_count".into(),
        "Exported {} alias(es) to: {}".into(),
    );

    m.insert(
        "helpers.exported_igcare_all".into(),
        "Exported all ignore/care settings to: {}".into(),
    );

    m.insert("helpers.file_not_found".into(), "File not found: {}".into());

    m.insert(
        "helpers.imported_igcare".into(),
        "Imported ignore/care settings from: {}".into(),
    );

    m.insert(
        "helpers.imported_ral".into(),
        "Imported run aliases from: {}".into(),
    );

    m.insert(
        "helpers.invalid_back_count".into(),
        "Invalid back count: {}".into(),
    );

    m.insert("helpers.invalid_input".into(), "Invalid input: {}".into());

    m.insert("helpers.invalid_num".into(), "Invalid number: {}".into());

    m.insert(
        "helpers.invalid_num_range".into(),
        "Invalid number: {}. Use 1-{} or 0 to finish.".into(),
    );

    m.insert(
        "helpers.no_editor".into(),
        "No external editor found. Opening built-in editor...".into(),
    );

    m.insert(
        "helpers.no_ral_to_export".into(),
        "No run aliases to export.".into(),
    );
}
