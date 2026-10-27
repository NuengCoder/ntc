use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "theme.create_failed".into(),
        "Failed to create theme: {}".into(),
    );

    m.insert(
        "theme.create_file_failed".into(),
        "Failed to create file: {}".into(),
    );

    m.insert(
        "theme.create_first".into(),
        "Run 'theme add <name>' to create your first theme".into(),
    );

    m.insert(
        "theme.created".into(),
        "Created theme '{}' (opened in editor)".into(),
    );

    m.insert(
        "theme.created_hint".into(),
        "Edit the file, save, then run 'theme reload' to apply changes".into(),
    );

    m.insert("theme.current_title".into(), "Current theme: {}".into());

    m.insert("theme.dir_title".into(), "Theme directory:".into());

    m.insert(
        "theme.edit_hint".into(),
        "Edit '{}', save, then run 'theme reload'".into(),
    );

    m.insert(
        "theme.export_failed".into(),
        "Failed to export theme: {}".into(),
    );

    m.insert(
        "theme.export_failed_simple".into(),
        "Failed to export: {}".into(),
    );

    m.insert("theme.exported_simple".into(), "Exported to: {}".into());

    m.insert(
        "theme.exported_to".into(),
        "Exported theme '{}' to: {}".into(),
    );

    m.insert("theme.file_exists".into(), "File already exists: {}".into());

    m.insert("theme.file_not_found".into(), "File not found: {}".into());

    m.insert("theme.files_title".into(), "Theme files:".into());

    m.insert(
        "theme.import_failed".into(),
        "Failed to import theme: {}".into(),
    );

    m.insert(
        "theme.import_failed_simple".into(),
        "Failed to import: {}".into(),
    );

    m.insert(
        "theme.import_hint".into(),
        "Run 'theme list' to see available themes".into(),
    );

    m.insert("theme.imported".into(), "Imported theme from: {}".into());

    m.insert("theme.info_author".into(), "Author".into());

    m.insert("theme.info_defined".into(), "\u{2713} defined".into());

    m.insert("theme.info_description".into(), "Description".into());

    m.insert("theme.info_editor".into(), "Editor colors".into());

    m.insert("theme.info_shell".into(), "Shell colors".into());

    m.insert("theme.info_syntax".into(), "Syntax colors".into());

    m.insert("theme.info_title".into(), "\u{1F3A8} Theme: {}".into());

    m.insert(
        "theme.invalid_number".into(),
        "Invalid number: {}. Use 1-{}".into(),
    );

    m.insert(
        "theme.name_empty".into(),
        "Theme name cannot be empty".into(),
    );

    m.insert(
        "theme.no_dir".into(),
        "Could not determine theme directory".into(),
    );

    m.insert(
        "theme.no_dir_yet".into(),
        "(directory does not exist yet)".into(),
    );

    m.insert("theme.no_files".into(), "(no theme files)".into());

    m.insert("theme.not_found".into(), "Theme '{}' not found".into());

    m.insert(
        "theme.not_found_hint".into(),
        "Theme not found: {}. Use 'theme list' to see available themes.".into(),
    );

    m.insert(
        "theme.opened_dir".into(),
        "Opened theme directory: {}".into(),
    );

    m.insert("theme.reloaded".into(), "Themes reloaded from disk".into());

    m.insert(
        "theme.remove_failed".into(),
        "Failed to remove theme: {}".into(),
    );

    m.insert("theme.removed".into(), "Removed theme: {}".into());

    m.insert(
        "theme.rename_failed".into(),
        "Failed to rename theme: {}".into(),
    );

    m.insert(
        "theme.rename_failed_simple".into(),
        "Failed to rename: {}".into(),
    );

    m.insert("theme.renamed".into(), "Renamed theme '{}' -> '{}'".into());

    m.insert("theme.renamed_simple".into(), "Renamed '{}' -> '{}'".into());

    m.insert(
        "theme.switch_failed".into(),
        "Failed to switch to theme: {}".into(),
    );

    m.insert(
        "theme.switch_failed_simple".into(),
        "Failed to switch to theme: {}".into(),
    );

    m.insert("theme.switched".into(), "Switched to theme: {}".into());

    m.insert(
        "theme.template_created".into(),
        "Created theme template: {}".into(),
    );

    m.insert(
        "theme.template_info".into(),
        "This is a local template file. Use 'theme import' to add it to your theme list.".into(),
    );

    m.insert(
        "theme.type_interactive".into(),
        "Type 'theme' for interactive menu".into(),
    );

    m.insert("theme.ui.current".into(), "Current theme".into());

    m.insert("theme.ui.exit".into(), "Exiting theme manager.".into());

    m.insert("theme.ui.name_prompt".into(), "Theme name: ".into());

    m.insert("theme.ui.theme_prompt".into(), "theme>".into());

    m.insert(
        "theme.ui.title".into(),
        "\u{1F3A8} ntc Theme Manager".into(),
    );

    m.insert("theme.unknown_cmd".into(), "Unknown command: {}".into());

    m.insert("theme.usage_add".into(), "Usage: theme add <name>".into());

    m.insert(
        "theme.usage_export".into(),
        "Usage: theme export <name> [output_file]".into(),
    );

    m.insert(
        "theme.usage_import".into(),
        "Usage: theme import <file.ntc_theme>".into(),
    );

    m.insert("theme.usage_info".into(), "Usage: theme info <name>".into());

    m.insert(
        "theme.usage_rename".into(),
        "Usage: theme rename <old_name> to <new_name>".into(),
    );

    m.insert("theme.usage_rm".into(), "Usage: theme rm <name>".into());

    m.insert(
        "theme.usage_rnm".into(),
        "Usage: theme rnm <old_name> to <new_name>".into(),
    );
}
