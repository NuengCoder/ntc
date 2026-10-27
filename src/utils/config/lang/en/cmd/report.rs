use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.report.content_saved".into(),
        "File content saved to: {}".into(),
    );

    m.insert(
        "cmd.report.copied_clipboard".into(),
        "File '{}' copied to clipboard!".into(),
    );

    m.insert("cmd.report.copy_failed".into(), "Failed to copy: {}".into());

    m.insert(
        "cmd.report.copying_clipboard".into(),
        "Copying '{}' to clipboard...".into(),
    );

    m.insert(
        "cmd.report.dir_only".into(),
        "{} report only works on directories".into(),
    );

    m.insert(
        "cmd.report.example_json".into(),
        "  report json ..         Report on parent directory".into(),
    );

    m.insert(
        "cmd.report.example_txt".into(),
        "  report txt             Report on current directory".into(),
    );

    m.insert(
        "cmd.report.example_project".into(),
        "  report project html . --name report.html".into(),
    );

    m.insert("cmd.report.example_clip".into(), "  report clip 1".into());

    m.insert(
        "cmd.report.clip_header".into(),
        "Clipboard entry #{}:".into(),
    );

    m.insert("cmd.report.clip_meta_date".into(), "Date".into());

    m.insert("cmd.report.clip_meta_lines".into(), "Lines".into());

    m.insert("cmd.report.clip_meta_chars".into(), "Characters".into());

    m.insert("cmd.report.examples".into(), "Examples:".into());

    m.insert(
        "cmd.report.file_not_found".into(),
        "File not found: {}".into(),
    );

    m.insert(
        "cmd.report.no_files_in_dir".into(),
        "No supported files to display in: {}".into(),
    );

    m.insert(
        "cmd.report.help_docx".into(),
        "  report docx [path]     Generate DOCX report".into(),
    );

    m.insert(
        "cmd.report.help_html".into(),
        "  report html [path]     Generate HTML report".into(),
    );

    m.insert(
        "cmd.report.help_json".into(),
        "  report json [path]     Generate JSON report".into(),
    );

    m.insert(
        "cmd.report.help_md".into(),
        "  report md [path]       Generate Markdown report".into(),
    );

    m.insert(
        "cmd.report.help_pdf".into(),
        "  report pdf [path]      Generate PDF report".into(),
    );

    m.insert(
        "cmd.report.help_clip".into(),
        "  report clip <n>       Show clipboard history entry in full".into(),
    );

    m.insert(
        "cmd.report.legacy_removed".into(),
        "Legacy report shortcuts are removed. Use 'report project <fmt> ...' instead.".into(),
    );

    m.insert("cmd.report.help_project".into(), "  report project <fmt> [path] --name <file>   Project report saved as <file> (--name optional; <name>.html when no extension)".into());

    m.insert(
        "cmd.report.project_usage".into(),
        "Usage: report project <txt|md|html|json|pdf|docx|xlsx> [path] [--name <output>]".into(),
    );

    m.insert(
        "cmd.report.help_txt".into(),
        "  report txt [path]      Generate TXT report".into(),
    );

    m.insert(
        "cmd.report.help_xlsx".into(),
        "  report xlsx [path]     Generate XLSX report".into(),
    );

    m.insert(
        "cmd.report.invalid_depth".into(),
        "Invalid depth: {}".into(),
    );

    m.insert(
        "cmd.report.neovim_hint".into(),
        "You can open this in Neovim with: :edit {}".into(),
    );

    m.insert(
        "cmd.report.skipped_unsupported".into(),
        "Skipped (not support format): {}".into(),
    );

    m.insert(
        "cmd.report.ext_usage".into(),
        "Usage: view -ext <ext...>  (e.g. view -ext rs c)".into(),
    );

    m.insert("cmd.report.time_label".into(), "Time:".into());

    m.insert(
        "cmd.report.title".into(),
        "report — Generate project reports".into(),
    );

    m.insert(
        "cmd.report.unknown_option".into(),
        "Unknown view option: {}".into(),
    );

    m.insert("cmd.report.view_usage".into(), "Usage: view [-s|--size [min-size]] [-fs|--filesize [min-size]] [-ds|--dirsize [min-size]] [--sort size|char|ext] [--head <n>|--tail <n>] [-c|--cp] [--care] [-t|--time] [-d|--depth <n>] [-p|--path <path|@tp>] [-ext|--ext <ext...>]".into());

    m.insert(
        "cmd.report.invalid_sort".into(),
        "Invalid sort key: '{}' (size|char|ext)".into(),
    );

    m.insert(
        "cmd.report.invalid_count".into(),
        "Invalid line count: '{}' (must be > 0)".into(),
    );

    m.insert(
        "cmd.report.invalid_range".into(),
        "Invalid range: '{}' (expected n:m, e.g. --betw 10:20)".into(),
    );
}
